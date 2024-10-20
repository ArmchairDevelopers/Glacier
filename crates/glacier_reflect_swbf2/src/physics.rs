use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_physics_types(registry: &mut TypeRegistry) {
    registry.register_type(HAVOKPHYSICSDATA_TYPE_INFO);
    registry.register_type(HAVOKPHYSICSDATA_ARRAY_TYPE_INFO);
    registry.register_type(RAGDOLLRESOURCE_TYPE_INFO);
    registry.register_type(RAGDOLLRESOURCE_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSROOTCONTROLCONSTRAINT_TYPE_INFO);
    registry.register_type(PHYSICSROOTCONTROLCONSTRAINT_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSRAGDOLLCONSTRAINT_TYPE_INFO);
    registry.register_type(PHYSICSRAGDOLLCONSTRAINT_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSPRISMATICCONSTRAINT_TYPE_INFO);
    registry.register_type(PHYSICSPRISMATICCONSTRAINT_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSHINGECONSTRAINT_TYPE_INFO);
    registry.register_type(PHYSICSHINGECONSTRAINT_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSBALLANDSOCKETCONSTRAINT_TYPE_INFO);
    registry.register_type(PHYSICSBALLANDSOCKETCONSTRAINT_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSANGULARLIMITCONSTRAINT_TYPE_INFO);
    registry.register_type(PHYSICSANGULARLIMITCONSTRAINT_ARRAY_TYPE_INFO);
    registry.register_type(IGLOOMODULEINITIALIZER_TYPE_INFO);
    registry.register_type(IGLOOMODULEINITIALIZER_ARRAY_TYPE_INFO);
    registry.register_type(IGLOOMEMORYCONTEXT_TYPE_INFO);
    registry.register_type(IGLOOMEMORYCONTEXT_ARRAY_TYPE_INFO);
    registry.register_type(IGLOOGRABTOOL_TYPE_INFO);
    registry.register_type(IGLOOGRABTOOL_ARRAY_TYPE_INFO);
    registry.register_type(IGLOOENTITYCREATOR_TYPE_INFO);
    registry.register_type(IGLOOENTITYCREATOR_ARRAY_TYPE_INFO);
    registry.register_type(IGLOOENTITYFACTORY_TYPE_INFO);
    registry.register_type(IGLOOENTITYFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(IGLOODEBUGGER_TYPE_INFO);
    registry.register_type(IGLOODEBUGGER_ARRAY_TYPE_INFO);
    registry.register_type(FLOATPHYSICSDATA_TYPE_INFO);
    registry.register_type(FLOATPHYSICSDATA_ARRAY_TYPE_INFO);
    registry.register_type(PARTINFODATA_TYPE_INFO);
    registry.register_type(PARTINFODATA_ARRAY_TYPE_INFO);
    registry.register_type(PRESOLVECOLLISIONPHYSICSCALLBACKHANDLERDATA_TYPE_INFO);
    registry.register_type(PRESOLVECOLLISIONPHYSICSCALLBACKHANDLERDATA_ARRAY_TYPE_INFO);
    registry.register_type(VEHICLEPHYSICSCALLBACKHANDLERDATA_TYPE_INFO);
    registry.register_type(VEHICLEPHYSICSCALLBACKHANDLERDATA_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTDYNAMICPHYSICSCALLBACKHANDLERDATA_TYPE_INFO);
    registry.register_type(CLIENTDYNAMICPHYSICSCALLBACKHANDLERDATA_ARRAY_TYPE_INFO);
    registry.register_type(LIGHTDYNAMICPHYSICSCALLBACKHANDLERDATA_TYPE_INFO);
    registry.register_type(LIGHTDYNAMICPHYSICSCALLBACKHANDLERDATA_ARRAY_TYPE_INFO);
    registry.register_type(DEFAULTPHYSICSCALLBACKHANDLERDATA_TYPE_INFO);
    registry.register_type(DEFAULTPHYSICSCALLBACKHANDLERDATA_ARRAY_TYPE_INFO);
    registry.register_type(NONEPHYSICSCALLBACKHANDLERDATA_TYPE_INFO);
    registry.register_type(NONEPHYSICSCALLBACKHANDLERDATA_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSCALLBACKHANDLERDATA_TYPE_INFO);
    registry.register_type(PHYSICSCALLBACKHANDLERDATA_ARRAY_TYPE_INFO);
    registry.register_type(RIGIDBODYCOLLISIONLAYER_TYPE_INFO);
    registry.register_type(RIGIDBODYCOLLISIONLAYER_ARRAY_TYPE_INFO);
    registry.register_type(RIGIDBODYQUALITYTYPE_TYPE_INFO);
    registry.register_type(RIGIDBODYQUALITYTYPE_ARRAY_TYPE_INFO);
    registry.register_type(RIGIDBODYMOTIONTYPE_TYPE_INFO);
    registry.register_type(RIGIDBODYMOTIONTYPE_ARRAY_TYPE_INFO);
    registry.register_type(RIGIDBODYTYPE_TYPE_INFO);
    registry.register_type(RIGIDBODYTYPE_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSBODYLAUNCHERENTITYDATA_TYPE_INFO);
    registry.register_type(PHYSICSBODYLAUNCHERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSDEBUGSETTINGS_TYPE_INFO);
    registry.register_type(PHYSICSDEBUGSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSSETTINGS_TYPE_INFO);
    registry.register_type(PHYSICSSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSWORLDTYPE_TYPE_INFO);
    registry.register_type(PHYSICSWORLDTYPE_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSRENDERSETTINGS_TYPE_INFO);
    registry.register_type(PHYSICSRENDERSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSCOMPONENTONPRESOLVECOLLISIONMESSAGE_TYPE_INFO);
    registry.register_type(PHYSICSCOMPONENTONIMPULSEMESSAGE_TYPE_INFO);
    registry.register_type(PHYSICSPRISMATICCONSTRAINTINITIALSTANCEDATA_TYPE_INFO);
    registry.register_type(PHYSICSPRISMATICCONSTRAINTINITIALSTANCEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSROOTCONTROLCONSTRAINTINITIALSTANCEDATA_TYPE_INFO);
    registry.register_type(PHYSICSROOTCONTROLCONSTRAINTINITIALSTANCEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSRAGDOLLCONSTRAINTINITIALSTANCEDATA_TYPE_INFO);
    registry.register_type(PHYSICSRAGDOLLCONSTRAINTINITIALSTANCEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSHINGECONSTRAINTINITIALSTANCEDATA_TYPE_INFO);
    registry.register_type(PHYSICSHINGECONSTRAINTINITIALSTANCEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSBALLANDSOCKETCONSTRAINTINITIALSTANCEDATA_TYPE_INFO);
    registry.register_type(PHYSICSBALLANDSOCKETCONSTRAINTINITIALSTANCEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSCONSTRAINTINITIALSTANCEDATA_TYPE_INFO);
    registry.register_type(PHYSICSCONSTRAINTINITIALSTANCEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSPRISMATICCONSTRAINTDATA_TYPE_INFO);
    registry.register_type(PHYSICSPRISMATICCONSTRAINTDATA_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSROOTCONTROLCONSTRAINTDATA_TYPE_INFO);
    registry.register_type(PHYSICSROOTCONTROLCONSTRAINTDATA_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSRAGDOLLCONSTRAINTDATA_TYPE_INFO);
    registry.register_type(PHYSICSRAGDOLLCONSTRAINTDATA_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSHINGECONSTRAINTDATA_TYPE_INFO);
    registry.register_type(PHYSICSHINGECONSTRAINTDATA_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSBALLANDSOCKETCONSTRAINTDATA_TYPE_INFO);
    registry.register_type(PHYSICSBALLANDSOCKETCONSTRAINTDATA_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSPRISMATICCONSTRAINTMOTORDATA_TYPE_INFO);
    registry.register_type(PHYSICSPRISMATICCONSTRAINTMOTORDATA_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSROOTCONTROLCONSTRAINTMOTORDATA_TYPE_INFO);
    registry.register_type(PHYSICSROOTCONTROLCONSTRAINTMOTORDATA_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSRAGDOLLCONSTRAINTMOTORDATA_TYPE_INFO);
    registry.register_type(PHYSICSRAGDOLLCONSTRAINTMOTORDATA_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSHINGECONSTRAINTMOTORDATA_TYPE_INFO);
    registry.register_type(PHYSICSHINGECONSTRAINTMOTORDATA_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSCONSTRAINTMOTORDATA_TYPE_INFO);
    registry.register_type(PHYSICSCONSTRAINTMOTORDATA_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSCONSTRAINTOWNERDATA_TYPE_INFO);
    registry.register_type(PHYSICSCONSTRAINTOWNERDATA_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSCONSTRAINTDATA_TYPE_INFO);
    registry.register_type(PHYSICSCONSTRAINTDATA_ARRAY_TYPE_INFO);
    registry.register_type(PROXIMITYTRACKERDATA_TYPE_INFO);
    registry.register_type(PROXIMITYTRACKERDATA_ARRAY_TYPE_INFO);
    registry.register_type(AABBTRIGGERPHYSICSBODYDATA_TYPE_INFO);
    registry.register_type(AABBTRIGGERPHYSICSBODYDATA_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINPHYSICSBODYDATA_TYPE_INFO);
    registry.register_type(TERRAINPHYSICSBODYDATA_ARRAY_TYPE_INFO);
    registry.register_type(STATICWATERPHYSICSBODYDATA_TYPE_INFO);
    registry.register_type(STATICWATERPHYSICSBODYDATA_ARRAY_TYPE_INFO);
    registry.register_type(WATERPHYSICSBODYDATA_TYPE_INFO);
    registry.register_type(WATERPHYSICSBODYDATA_ARRAY_TYPE_INFO);
    registry.register_type(GROUPRIGIDBODYDATA_TYPE_INFO);
    registry.register_type(GROUPRIGIDBODYDATA_ARRAY_TYPE_INFO);
    registry.register_type(MATERIALINDICESLOOKUP_TYPE_INFO);
    registry.register_type(MATERIALINDICESLOOKUP_ARRAY_TYPE_INFO);
    registry.register_type(RIGIDBODYDATA_TYPE_INFO);
    registry.register_type(RIGIDBODYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSBODYDATA_TYPE_INFO);
    registry.register_type(PHYSICSBODYDATA_ARRAY_TYPE_INFO);
    registry.register_type(BOXFLOATPHYSICSDATA_TYPE_INFO);
    registry.register_type(BOXFLOATPHYSICSDATA_ARRAY_TYPE_INFO);
    registry.register_type(FLOATPHYSICSACTIONDATA_TYPE_INFO);
    registry.register_type(FLOATPHYSICSACTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(WINDPHYSICSACTIONDATA_TYPE_INFO);
    registry.register_type(WINDPHYSICSACTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSACTIONDATA_TYPE_INFO);
    registry.register_type(PHYSICSACTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSPROXYENTITYDATA_TYPE_INFO);
    registry.register_type(PHYSICSPROXYENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(GAMEPHYSICSENTITYDATA_TYPE_INFO);
    registry.register_type(GAMEPHYSICSENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(DEFAULTPARTPHYSICSCOMPONENTDATA_TYPE_INFO);
    registry.register_type(DEFAULTPARTPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(PARTPHYSICSCOMPONENTDATA_TYPE_INFO);
    registry.register_type(PARTPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(RAGDOLLPHYSICSCOMPONENTDATA_TYPE_INFO);
    registry.register_type(RAGDOLLPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(FORCECOMPONENTDATA_TYPE_INFO);
    registry.register_type(FORCECOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSCOMPONENTDATA_TYPE_INFO);
    registry.register_type(PHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(INTERNALCOLLISIONDISABLINGBEHAVIOR_TYPE_INFO);
    registry.register_type(INTERNALCOLLISIONDISABLINGBEHAVIOR_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSPARTDATA_TYPE_INFO);
    registry.register_type(PHYSICSPARTDATA_ARRAY_TYPE_INFO);
    registry.register_type(MATERIALPROPERTYPHYSICSDESTRUCTIONDATA_TYPE_INFO);
    registry.register_type(MATERIALPROPERTYPHYSICSDESTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(MATERIALPROPERTYPHYSICSDATA_TYPE_INFO);
    registry.register_type(MATERIALPROPERTYPHYSICSDATA_ARRAY_TYPE_INFO);
    registry.register_type(PROXIMITYDATA_TYPE_INFO);
    registry.register_type(PROXIMITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PROXIMITYOBJECTTYPE_TYPE_INFO);
    registry.register_type(PROXIMITYOBJECTTYPE_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSRESOURCECONTAINERASSET_TYPE_INFO);
    registry.register_type(PHYSICSRESOURCECONTAINERASSET_ARRAY_TYPE_INFO);
    registry.register_type(RAGDOLLASSET_TYPE_INFO);
    registry.register_type(RAGDOLLASSET_ARRAY_TYPE_INFO);
    registry.register_type(GROUPHAVOKASSET_TYPE_INFO);
    registry.register_type(GROUPHAVOKASSET_ARRAY_TYPE_INFO);
    registry.register_type(ASSETAABBS_TYPE_INFO);
    registry.register_type(ASSETAABBS_ARRAY_TYPE_INFO);
    registry.register_type(HAVOKASSET_TYPE_INFO);
    registry.register_type(HAVOKASSET_ARRAY_TYPE_INFO);
    registry.register_type(HEIGHTFIELDTESTENTITYDATA_TYPE_INFO);
    registry.register_type(HEIGHTFIELDTESTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CHARACTERPHYSICSDATA_TYPE_INFO);
    registry.register_type(CHARACTERPHYSICSDATA_ARRAY_TYPE_INFO);
    registry.register_type(CHARACTERPHYSICSBODYDATA_TYPE_INFO);
    registry.register_type(CHARACTERPHYSICSBODYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PARACHUTESTATEDATA_TYPE_INFO);
    registry.register_type(PARACHUTESTATEDATA_ARRAY_TYPE_INFO);
    registry.register_type(SWIMMINGSTATEDATA_TYPE_INFO);
    registry.register_type(SWIMMINGSTATEDATA_ARRAY_TYPE_INFO);
    registry.register_type(JUMPSTATEDATA_TYPE_INFO);
    registry.register_type(JUMPSTATEDATA_ARRAY_TYPE_INFO);
    registry.register_type(CLIMBINGSTATEDATA_TYPE_INFO);
    registry.register_type(CLIMBINGSTATEDATA_ARRAY_TYPE_INFO);
    registry.register_type(FALLINGSTATEDATA_TYPE_INFO);
    registry.register_type(FALLINGSTATEDATA_ARRAY_TYPE_INFO);
    registry.register_type(INAIRSTATEDATA_TYPE_INFO);
    registry.register_type(INAIRSTATEDATA_ARRAY_TYPE_INFO);
    registry.register_type(ONGROUNDSTATEDATA_TYPE_INFO);
    registry.register_type(ONGROUNDSTATEDATA_ARRAY_TYPE_INFO);
    registry.register_type(SLIDINGSTATEDATA_TYPE_INFO);
    registry.register_type(SLIDINGSTATEDATA_ARRAY_TYPE_INFO);
    registry.register_type(ANIMATIONCONTROLLEDSTATEDATA_TYPE_INFO);
    registry.register_type(ANIMATIONCONTROLLEDSTATEDATA_ARRAY_TYPE_INFO);
    registry.register_type(CHARACTERSTATEDATA_TYPE_INFO);
    registry.register_type(CHARACTERSTATEDATA_ARRAY_TYPE_INFO);
    registry.register_type(CHARACTERSTATETYPE_TYPE_INFO);
    registry.register_type(CHARACTERSTATETYPE_ARRAY_TYPE_INFO);
    registry.register_type(CHARACTERSPRINTDATA_TYPE_INFO);
    registry.register_type(CHARACTERSPRINTDATA_ARRAY_TYPE_INFO);
    registry.register_type(CHARACTERSTATEPOSEINFO_TYPE_INFO);
    registry.register_type(CHARACTERSTATEPOSEINFO_ARRAY_TYPE_INFO);
    registry.register_type(SPEEDMODIFIERDATA_TYPE_INFO);
    registry.register_type(SPEEDMODIFIERDATA_ARRAY_TYPE_INFO);
    registry.register_type(CHARACTERPOSEDATA_TYPE_INFO);
    registry.register_type(CHARACTERPOSEDATA_ARRAY_TYPE_INFO);
    registry.register_type(POSETRANSITIONTIME_TYPE_INFO);
    registry.register_type(POSETRANSITIONTIME_ARRAY_TYPE_INFO);
    registry.register_type(CHARACTERPOSECOLLISIONTYPE_TYPE_INFO);
    registry.register_type(CHARACTERPOSECOLLISIONTYPE_ARRAY_TYPE_INFO);
    registry.register_type(CHARACTERPOSETYPE_TYPE_INFO);
    registry.register_type(CHARACTERPOSETYPE_ARRAY_TYPE_INFO);
    registry.register_type(CHARACTERPOSECONSTRAINTSDATA_TYPE_INFO);
    registry.register_type(CHARACTERPOSECONSTRAINTSDATA_ARRAY_TYPE_INFO);
    registry.register_type(LOOKCONSTRAINTSDATA_TYPE_INFO);
    registry.register_type(LOOKCONSTRAINTSDATA_ARRAY_TYPE_INFO);
    registry.register_type(VEHICLESPAWNSTATEENTITYDATA_TYPE_INFO);
    registry.register_type(VEHICLESPAWNSTATEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(LINEARMOVINGBODYDATA_TYPE_INFO);
    registry.register_type(LINEARMOVINGBODYDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENDPOINTDATA_TYPE_INFO);
    registry.register_type(ENDPOINTDATA_ARRAY_TYPE_INFO);
    registry.register_type(ROTATIONBODYDATA_TYPE_INFO);
    registry.register_type(ROTATIONBODYDATA_ARRAY_TYPE_INFO);
    registry.register_type(EXTENDEDCONSTRAINTSDATA_TYPE_INFO);
    registry.register_type(EXTENDEDCONSTRAINTSDATA_ARRAY_TYPE_INFO);
    registry.register_type(MOVINGBODYDATA_TYPE_INFO);
    registry.register_type(MOVINGBODYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CAMERALOOSEPARTPHYSICSDATA_TYPE_INFO);
    registry.register_type(CAMERALOOSEPARTPHYSICSDATA_ARRAY_TYPE_INFO);
    registry.register_type(NETWORKABLELOOSEPARTPHYSICSDATA_TYPE_INFO);
    registry.register_type(NETWORKABLELOOSEPARTPHYSICSDATA_ARRAY_TYPE_INFO);
    registry.register_type(LOOSEPARTPHYSICSDATA_TYPE_INFO);
    registry.register_type(LOOSEPARTPHYSICSDATA_ARRAY_TYPE_INFO);
    registry.register_type(WHEELCONFIGMOTORBIKEDATA_TYPE_INFO);
    registry.register_type(WHEELCONFIGMOTORBIKEDATA_ARRAY_TYPE_INFO);
    registry.register_type(WHEELCONFIGDATA_TYPE_INFO);
    registry.register_type(WHEELCONFIGDATA_ARRAY_TYPE_INFO);
    registry.register_type(FRICTIONSCALEATVELOCITY_TYPE_INFO);
    registry.register_type(FRICTIONSCALEATVELOCITY_ARRAY_TYPE_INFO);
    registry.register_type(SENSITIVITYATVELOCITY_TYPE_INFO);
    registry.register_type(SENSITIVITYATVELOCITY_ARRAY_TYPE_INFO);
    registry.register_type(SPRINGDATA_TYPE_INFO);
    registry.register_type(SPRINGDATA_ARRAY_TYPE_INFO);
    registry.register_type(SPHERECOLLISIONDATA_TYPE_INFO);
    registry.register_type(SPHERECOLLISIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(JETENGINECONFIGDATA_TYPE_INFO);
    registry.register_type(JETENGINECONFIGDATA_ARRAY_TYPE_INFO);
    registry.register_type(PROPELLERENGINECONFIGDATA_TYPE_INFO);
    registry.register_type(PROPELLERENGINECONFIGDATA_ARRAY_TYPE_INFO);
    registry.register_type(ROTORPARAMETERS_TYPE_INFO);
    registry.register_type(ROTORPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(FORCEMAGNITUDEINPUTTYPE_TYPE_INFO);
    registry.register_type(FORCEMAGNITUDEINPUTTYPE_ARRAY_TYPE_INFO);
    registry.register_type(PROPELLERTYPE_TYPE_INFO);
    registry.register_type(PROPELLERTYPE_ARRAY_TYPE_INFO);
    registry.register_type(COMBUSTIONENGINECONFIGDATA_TYPE_INFO);
    registry.register_type(COMBUSTIONENGINECONFIGDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENGINECONFIGDATA_TYPE_INFO);
    registry.register_type(ENGINECONFIGDATA_ARRAY_TYPE_INFO);
    registry.register_type(BOOST_TYPE_INFO);
    registry.register_type(BOOST_ARRAY_TYPE_INFO);
    registry.register_type(GEARBOXCONFIGDATA_TYPE_INFO);
    registry.register_type(GEARBOXCONFIGDATA_ARRAY_TYPE_INFO);
    registry.register_type(VEHICLEINPUTTWEAKDATA_TYPE_INFO);
    registry.register_type(VEHICLEINPUTTWEAKDATA_ARRAY_TYPE_INFO);
    registry.register_type(VEHICLEINPUTTWEAKTYPE_TYPE_INFO);
    registry.register_type(VEHICLEINPUTTWEAKTYPE_ARRAY_TYPE_INFO);
    registry.register_type(MOTIONDAMPINGDATA_TYPE_INFO);
    registry.register_type(MOTIONDAMPINGDATA_ARRAY_TYPE_INFO);
    registry.register_type(STABILIZERDATA_TYPE_INFO);
    registry.register_type(STABILIZERDATA_ARRAY_TYPE_INFO);
    registry.register_type(WINGPHYSICSDATA_TYPE_INFO);
    registry.register_type(WINGPHYSICSDATA_ARRAY_TYPE_INFO);
    registry.register_type(CURVE2D_TYPE_INFO);
    registry.register_type(CURVE2D_ARRAY_TYPE_INFO);
    registry.register_type(LANDINGFLAPDATA_TYPE_INFO);
    registry.register_type(LANDINGFLAPDATA_ARRAY_TYPE_INFO);
    registry.register_type(AERODYNAMICPHYSICSDATA_TYPE_INFO);
    registry.register_type(AERODYNAMICPHYSICSDATA_ARRAY_TYPE_INFO);
    registry.register_type(HOVERCRAFTFLOATPHYSICSDATA_TYPE_INFO);
    registry.register_type(HOVERCRAFTFLOATPHYSICSDATA_ARRAY_TYPE_INFO);
    registry.register_type(BOATFLOATPHYSICSDATA_TYPE_INFO);
    registry.register_type(BOATFLOATPHYSICSDATA_ARRAY_TYPE_INFO);
    registry.register_type(HULLFLOATPHYSICSDATA_TYPE_INFO);
    registry.register_type(HULLFLOATPHYSICSDATA_ARRAY_TYPE_INFO);
    registry.register_type(VEHICLEPHYSICSACTIONDATA_TYPE_INFO);
    registry.register_type(VEHICLEPHYSICSACTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(VEHICLECONFIGDATA_TYPE_INFO);
    registry.register_type(VEHICLECONFIGDATA_ARRAY_TYPE_INFO);
    registry.register_type(MOTORBIKEDATA_TYPE_INFO);
    registry.register_type(MOTORBIKEDATA_ARRAY_TYPE_INFO);
    registry.register_type(INPUTTHROTTLE_TYPE_INFO);
    registry.register_type(INPUTTHROTTLE_ARRAY_TYPE_INFO);
    registry.register_type(CONSTANTFORCEDATA_TYPE_INFO);
    registry.register_type(CONSTANTFORCEDATA_ARRAY_TYPE_INFO);
    registry.register_type(SPACETYPE_TYPE_INFO);
    registry.register_type(SPACETYPE_ARRAY_TYPE_INFO);
    registry.register_type(FORCETYPE_TYPE_INFO);
    registry.register_type(FORCETYPE_ARRAY_TYPE_INFO);
    registry.register_type(FORCECONDITION_TYPE_INFO);
    registry.register_type(FORCECONDITION_ARRAY_TYPE_INFO);
    registry.register_type(STABILIZERSETTINGS_TYPE_INFO);
    registry.register_type(STABILIZERSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(STABILIZERPROPERTY_TYPE_INFO);
    registry.register_type(STABILIZERPROPERTY_ARRAY_TYPE_INFO);
    registry.register_type(ANTIROLLBARS_TYPE_INFO);
    registry.register_type(ANTIROLLBARS_ARRAY_TYPE_INFO);
    registry.register_type(ANTIROLLBAR_TYPE_INFO);
    registry.register_type(ANTIROLLBAR_ARRAY_TYPE_INFO);
    registry.register_type(VEHICLEINPUTDATA_TYPE_INFO);
    registry.register_type(VEHICLEINPUTDATA_ARRAY_TYPE_INFO);
    registry.register_type(VEHICLEMODE_TYPE_INFO);
    registry.register_type(VEHICLEMODE_ARRAY_TYPE_INFO);
    registry.register_type(GEARBOXNETSTATE_TYPE_INFO);
    registry.register_type(GEARBOXNETSTATE_ARRAY_TYPE_INFO);
    registry.register_type(GEARBOXGEARLIMITS_TYPE_INFO);
    registry.register_type(GEARBOXGEARLIMITS_ARRAY_TYPE_INFO);
    registry.register_type(CLUTCHSTATUS_TYPE_INFO);
    registry.register_type(CLUTCHSTATUS_ARRAY_TYPE_INFO);
    registry.register_type(GEARSTATUS_TYPE_INFO);
    registry.register_type(GEARSTATUS_ARRAY_TYPE_INFO);
    registry.register_type(GEARBOXSTATUS_TYPE_INFO);
    registry.register_type(GEARBOXSTATUS_ARRAY_TYPE_INFO);
    registry.register_type(GEARBOXLOCKMODE_TYPE_INFO);
    registry.register_type(GEARBOXLOCKMODE_ARRAY_TYPE_INFO);
    registry.register_type(GEARBOXMODE_TYPE_INFO);
    registry.register_type(GEARBOXMODE_ARRAY_TYPE_INFO);
    registry.register_type(GEARBOXTYPE_TYPE_INFO);
    registry.register_type(GEARBOXTYPE_ARRAY_TYPE_INFO);
    registry.register_type(IGLOOGRABTOOLSETTINGS_TYPE_INFO);
    registry.register_type(IGLOOGRABTOOLSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSRENDERWORLDDYNAMICSTATE_TYPE_INFO);
    registry.register_type(PHYSICSRENDERWORLDDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSRENDERWORLDSTATICSTATE_TYPE_INFO);
    registry.register_type(PHYSICSRENDERWORLDSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSRENDERWORLDHANDLE_TYPE_INFO);
    registry.register_type(PHYSICSRENDERWORLDHANDLE_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSCATEGORYSET_TYPE_INFO);
    registry.register_type(PHYSICSCATEGORYSET_ARRAY_TYPE_INFO);
    registry.register_type(FBPHYSICSWORLDDATA_TYPE_INFO);
    registry.register_type(FBPHYSICSWORLDDATA_ARRAY_TYPE_INFO);
    registry.register_type(FBPHYSICSSUBLEVELDATA_TYPE_INFO);
    registry.register_type(FBPHYSICSSUBLEVELDATA_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSCOLLISIONCATEGORY_TYPE_INFO);
    registry.register_type(PHYSICSCOLLISIONCATEGORY_ARRAY_TYPE_INFO);
    registry.register_type(FBPHYSICSCOMPONENTDATA_TYPE_INFO);
    registry.register_type(FBPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSASSET_TYPE_INFO);
    registry.register_type(PHYSICSASSET_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSCULLINGTABLEDATA_TYPE_INFO);
    registry.register_type(PHYSICSCULLINGTABLEDATA_ARRAY_TYPE_INFO);
    registry.register_type(FBPHYSICSSHAPEDATA_TYPE_INFO);
    registry.register_type(FBPHYSICSSHAPEDATA_ARRAY_TYPE_INFO);
    registry.register_type(FBPHYSICSCONVEXHULLDATA_TYPE_INFO);
    registry.register_type(FBPHYSICSCONVEXHULLDATA_ARRAY_TYPE_INFO);
    registry.register_type(FBPHYSICSDIRECTIONALDRIVEDATA_TYPE_INFO);
    registry.register_type(FBPHYSICSDIRECTIONALDRIVEDATA_ARRAY_TYPE_INFO);
    registry.register_type(FBPHYSICSANGULARJOINTDATA_TYPE_INFO);
    registry.register_type(FBPHYSICSANGULARJOINTDATA_ARRAY_TYPE_INFO);
    registry.register_type(FBPHYSICSLOCKEDLINEARJOINTDATA_TYPE_INFO);
    registry.register_type(FBPHYSICSLOCKEDLINEARJOINTDATA_ARRAY_TYPE_INFO);
    registry.register_type(FBPHYSICSBODYDATA_TYPE_INFO);
    registry.register_type(FBPHYSICSBODYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSBONEDATA_TYPE_INFO);
    registry.register_type(PHYSICSBONEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSTRIANGLEARRAYDATA_TYPE_INFO);
    registry.register_type(PHYSICSTRIANGLEARRAYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSTRIANGLE_TYPE_INFO);
    registry.register_type(PHYSICSTRIANGLE_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSCONSTRAINTLIMITSPACE_TYPE_INFO);
    registry.register_type(PHYSICSCONSTRAINTLIMITSPACE_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSANGULARJOINTSWINGTYPE_TYPE_INFO);
    registry.register_type(PHYSICSANGULARJOINTSWINGTYPE_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSANGULARJOINTTWISTTYPE_TYPE_INFO);
    registry.register_type(PHYSICSANGULARJOINTTWISTTYPE_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSSHAPETYPE_TYPE_INFO);
    registry.register_type(PHYSICSSHAPETYPE_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSCOMPONENTCONSTANTS_TYPE_INFO);
    registry.register_type(PHYSICSCOMPONENTCONSTANTS_ARRAY_TYPE_INFO);
    registry.register_type(HAVOKMEMORYCONTEXT_TYPE_INFO);
    registry.register_type(HAVOKMEMORYCONTEXT_ARRAY_TYPE_INFO);
    registry.register_type(WINDPHYSICSACTION_TYPE_INFO);
    registry.register_type(WINDPHYSICSACTION_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTDESTRUCTIONINFO_TYPE_INFO);
    registry.register_type(CLIENTDESTRUCTIONINFO_ARRAY_TYPE_INFO);
    registry.register_type(SERVERDESTRUCTIONINFO_TYPE_INFO);
    registry.register_type(SERVERDESTRUCTIONINFO_ARRAY_TYPE_INFO);
    registry.register_type(DESTRUCTIONINFO_TYPE_INFO);
    registry.register_type(DESTRUCTIONINFO_ARRAY_TYPE_INFO);
    registry.register_type(DESTRUCTIONPHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(DESTRUCTIONPHYSICSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(DESTRUCTIONENTITYFACTORY_TYPE_INFO);
    registry.register_type(DESTRUCTIONENTITYFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(IGLOOSUBSYSTEM_TYPE_INFO);
    registry.register_type(IGLOOSUBSYSTEM_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSPROXYENTITY_TYPE_INFO);
    registry.register_type(PHYSICSPROXYENTITY_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(PHYSICSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(MULTICOLLISIONEVENT_TYPE_INFO);
    registry.register_type(MULTICOLLISIONEVENT_ARRAY_TYPE_INFO);
    registry.register_type(PRESOLVECOLLISIONINFO_TYPE_INFO);
    registry.register_type(PRESOLVECOLLISIONINFO_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSHOCKWAVECOLLISIONINFO_TYPE_INFO);
    registry.register_type(SERVERSHOCKWAVECOLLISIONINFO_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSHOCKWAVECOLLISIONINFO_TYPE_INFO);
    registry.register_type(CLIENTSHOCKWAVECOLLISIONINFO_ARRAY_TYPE_INFO);
    registry.register_type(SHOCKWAVECOLLISIONINFO_TYPE_INFO);
    registry.register_type(SHOCKWAVECOLLISIONINFO_ARRAY_TYPE_INFO);
    registry.register_type(SERVERFAKECOLLISIONINFO_TYPE_INFO);
    registry.register_type(SERVERFAKECOLLISIONINFO_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTFAKECOLLISIONINFO_TYPE_INFO);
    registry.register_type(CLIENTFAKECOLLISIONINFO_ARRAY_TYPE_INFO);
    registry.register_type(FAKECOLLISIONINFO_TYPE_INFO);
    registry.register_type(FAKECOLLISIONINFO_ARRAY_TYPE_INFO);
    registry.register_type(COLLISIONINFO_TYPE_INFO);
    registry.register_type(COLLISIONINFO_ARRAY_TYPE_INFO);
    registry.register_type(PARTPHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(PARTPHYSICSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(GROUPPHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(GROUPPHYSICSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(FORCECOMPONENT_TYPE_INFO);
    registry.register_type(FORCECOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(DEFAULTPARTPHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(DEFAULTPARTPHYSICSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CHARACTERPHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(CHARACTERPHYSICSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSCONSTRAINTOWNER_TYPE_INFO);
    registry.register_type(PHYSICSCONSTRAINTOWNER_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSCONSTRAINT_TYPE_INFO);
    registry.register_type(PHYSICSCONSTRAINT_ARRAY_TYPE_INFO);
    registry.register_type(STATICWATERPHYSICSBODY_TYPE_INFO);
    registry.register_type(STATICWATERPHYSICSBODY_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSBODY_TYPE_INFO);
    registry.register_type(PHYSICSBODY_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSACTION_TYPE_INFO);
    registry.register_type(PHYSICSACTION_ARRAY_TYPE_INFO);
    registry.register_type(FLOATPHYSICSACTION_TYPE_INFO);
    registry.register_type(FLOATPHYSICSACTION_ARRAY_TYPE_INFO);
    registry.register_type(FLOATPHYSICSACTIONFACTORY_TYPE_INFO);
    registry.register_type(FLOATPHYSICSACTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(RAGDOLLPHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(RAGDOLLPHYSICSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(WATERPHYSICSBODY_TYPE_INFO);
    registry.register_type(WATERPHYSICSBODY_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINPHYSICSBODY_TYPE_INFO);
    registry.register_type(TERRAINPHYSICSBODY_ARRAY_TYPE_INFO);
    registry.register_type(RIGIDBODY_TYPE_INFO);
    registry.register_type(RIGIDBODY_ARRAY_TYPE_INFO);
    registry.register_type(GROUPRIGIDBODY_TYPE_INFO);
    registry.register_type(GROUPRIGIDBODY_ARRAY_TYPE_INFO);
    registry.register_type(CHARACTERPHYSICSBODY_TYPE_INFO);
    registry.register_type(CHARACTERPHYSICSBODY_ARRAY_TYPE_INFO);
    registry.register_type(AABBTRIGGERPHYSICSBODY_TYPE_INFO);
    registry.register_type(AABBTRIGGERPHYSICSBODY_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSMANAGER_TYPE_INFO);
    registry.register_type(PHYSICSMANAGER_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSENTITYCREATOR_TYPE_INFO);
    registry.register_type(PHYSICSENTITYCREATOR_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSENTITYFACTORY_TYPE_INFO);
    registry.register_type(PHYSICSENTITYFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DESTRUCTIONDEBRISREFERENCEOBJECTDATA_TYPE_INFO);
    registry.register_type(DESTRUCTIONDEBRISREFERENCEOBJECTDATA_ARRAY_TYPE_INFO);
    registry.register_type(DEBRISINSERTTYPE_TYPE_INFO);
    registry.register_type(DEBRISINSERTTYPE_ARRAY_TYPE_INFO);
    registry.register_type(DESTRUCTIONCONTROLLERCOMPONENTDATA_TYPE_INFO);
    registry.register_type(DESTRUCTIONCONTROLLERCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(EDGEMODELINSTANCE_TYPE_INFO);
    registry.register_type(EDGEMODELINSTANCE_ARRAY_TYPE_INFO);
    registry.register_type(DESTRUCTIONPARTCOMPONENTDATA_TYPE_INFO);
    registry.register_type(DESTRUCTIONPARTCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(FAKEHINGEDATA_TYPE_INFO);
    registry.register_type(FAKEHINGEDATA_ARRAY_TYPE_INFO);
    registry.register_type(FAKESPRINGDATA_TYPE_INFO);
    registry.register_type(FAKESPRINGDATA_ARRAY_TYPE_INFO);
    registry.register_type(FAKEPHYSICSDATA_TYPE_INFO);
    registry.register_type(FAKEPHYSICSDATA_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSANGULARLIMITCONSTRAINTDATA_TYPE_INFO);
    registry.register_type(PHYSICSANGULARLIMITCONSTRAINTDATA_ARRAY_TYPE_INFO);
    registry.register_type(ANGULARLIMITCONSTRAINTAXIS_TYPE_INFO);
    registry.register_type(ANGULARLIMITCONSTRAINTAXIS_ARRAY_TYPE_INFO);
    registry.register_type(VEHICLEPHYSICSACTIONFACTORY_TYPE_INFO);
    registry.register_type(VEHICLEPHYSICSACTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(VEHICLEPHYSICSACTION_TYPE_INFO);
    registry.register_type(VEHICLEPHYSICSACTION_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HavokPhysicsData {
}

pub const HAVOKPHYSICSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HavokPhysicsData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(HAVOKPHYSICSDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for HavokPhysicsData {
    fn type_info() -> &'static TypeInfo {
        HAVOKPHYSICSDATA_TYPE_INFO
    }
}


pub const HAVOKPHYSICSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HavokPhysicsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("HavokPhysicsData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RagdollResource {
}

pub const RAGDOLLRESOURCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RagdollResource",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(RAGDOLLRESOURCE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RagdollResource {
    fn type_info() -> &'static TypeInfo {
        RAGDOLLRESOURCE_TYPE_INFO
    }
}


pub const RAGDOLLRESOURCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RagdollResource-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("RagdollResource-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PhysicsRootControlConstraint {
}

pub const PHYSICSROOTCONTROLCONSTRAINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRootControlConstraint",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSROOTCONTROLCONSTRAINT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PhysicsRootControlConstraint {
    fn type_info() -> &'static TypeInfo {
        PHYSICSROOTCONTROLCONSTRAINT_TYPE_INFO
    }
}


pub const PHYSICSROOTCONTROLCONSTRAINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRootControlConstraint-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsRootControlConstraint-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PhysicsRagdollConstraint {
}

pub const PHYSICSRAGDOLLCONSTRAINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRagdollConstraint",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSRAGDOLLCONSTRAINT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PhysicsRagdollConstraint {
    fn type_info() -> &'static TypeInfo {
        PHYSICSRAGDOLLCONSTRAINT_TYPE_INFO
    }
}


pub const PHYSICSRAGDOLLCONSTRAINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRagdollConstraint-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsRagdollConstraint-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PhysicsPrismaticConstraint {
}

pub const PHYSICSPRISMATICCONSTRAINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsPrismaticConstraint",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSPRISMATICCONSTRAINT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PhysicsPrismaticConstraint {
    fn type_info() -> &'static TypeInfo {
        PHYSICSPRISMATICCONSTRAINT_TYPE_INFO
    }
}


pub const PHYSICSPRISMATICCONSTRAINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsPrismaticConstraint-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsPrismaticConstraint-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PhysicsHingeConstraint {
}

pub const PHYSICSHINGECONSTRAINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsHingeConstraint",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSHINGECONSTRAINT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PhysicsHingeConstraint {
    fn type_info() -> &'static TypeInfo {
        PHYSICSHINGECONSTRAINT_TYPE_INFO
    }
}


pub const PHYSICSHINGECONSTRAINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsHingeConstraint-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsHingeConstraint-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PhysicsBallAndSocketConstraint {
}

pub const PHYSICSBALLANDSOCKETCONSTRAINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsBallAndSocketConstraint",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSBALLANDSOCKETCONSTRAINT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PhysicsBallAndSocketConstraint {
    fn type_info() -> &'static TypeInfo {
        PHYSICSBALLANDSOCKETCONSTRAINT_TYPE_INFO
    }
}


pub const PHYSICSBALLANDSOCKETCONSTRAINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsBallAndSocketConstraint-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsBallAndSocketConstraint-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PhysicsAngularLimitConstraint {
}

pub const PHYSICSANGULARLIMITCONSTRAINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsAngularLimitConstraint",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSANGULARLIMITCONSTRAINT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PhysicsAngularLimitConstraint {
    fn type_info() -> &'static TypeInfo {
        PHYSICSANGULARLIMITCONSTRAINT_TYPE_INFO
    }
}


pub const PHYSICSANGULARLIMITCONSTRAINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsAngularLimitConstraint-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsAngularLimitConstraint-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IglooModuleInitializer {
}

pub const IGLOOMODULEINITIALIZER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IglooModuleInitializer",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(IGLOOMODULEINITIALIZER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IglooModuleInitializer {
    fn type_info() -> &'static TypeInfo {
        IGLOOMODULEINITIALIZER_TYPE_INFO
    }
}


pub const IGLOOMODULEINITIALIZER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IglooModuleInitializer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("IglooModuleInitializer-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IglooMemoryContext {
}

pub const IGLOOMEMORYCONTEXT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IglooMemoryContext",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(IGLOOMEMORYCONTEXT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IglooMemoryContext {
    fn type_info() -> &'static TypeInfo {
        IGLOOMEMORYCONTEXT_TYPE_INFO
    }
}


pub const IGLOOMEMORYCONTEXT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IglooMemoryContext-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("IglooMemoryContext-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IglooGrabTool {
}

pub const IGLOOGRABTOOL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IglooGrabTool",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(IGLOOGRABTOOL_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for IglooGrabTool {
    fn type_info() -> &'static TypeInfo {
        IGLOOGRABTOOL_TYPE_INFO
    }
}


pub const IGLOOGRABTOOL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IglooGrabTool-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("IglooGrabTool-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IglooEntityCreator {
}

pub const IGLOOENTITYCREATOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IglooEntityCreator",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(IGLOOENTITYCREATOR_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IglooEntityCreator {
    fn type_info() -> &'static TypeInfo {
        IGLOOENTITYCREATOR_TYPE_INFO
    }
}


pub const IGLOOENTITYCREATOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IglooEntityCreator-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("IglooEntityCreator-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IglooEntityFactory {
}

pub const IGLOOENTITYFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IglooEntityFactory",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(IGLOOENTITYFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IglooEntityFactory {
    fn type_info() -> &'static TypeInfo {
        IGLOOENTITYFACTORY_TYPE_INFO
    }
}


pub const IGLOOENTITYFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IglooEntityFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("IglooEntityFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IglooDebugger {
}

pub const IGLOODEBUGGER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IglooDebugger",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(IGLOODEBUGGER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IglooDebugger {
    fn type_info() -> &'static TypeInfo {
        IGLOODEBUGGER_TYPE_INFO
    }
}


pub const IGLOODEBUGGER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IglooDebugger-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("IglooDebugger-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FloatPhysicsData {
    pub density: f32,
    pub filled_density: f32,
}

pub const FLOATPHYSICSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatPhysicsData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Density",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FloatPhysicsData, density),
            },
            FieldInfoData {
                name: "FilledDensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FloatPhysicsData, filled_density),
            },
        ],
    }),
    array_type: Some(FLOATPHYSICSDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FloatPhysicsData {
    fn type_info() -> &'static TypeInfo {
        FLOATPHYSICSDATA_TYPE_INFO
    }
}


pub const FLOATPHYSICSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatPhysicsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("FloatPhysicsData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PartInfoData {
    pub aabb: super::core::AxisAlignedBox,
    pub translation: super::core::Vec3,
}

pub const PARTINFODATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartInfoData",
    flags: MemberInfoFlags::new(32841),
    module: "Physics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Aabb",
                flags: MemberInfoFlags::new(0),
                field_type: AXISALIGNEDBOX_TYPE_INFO,
                rust_offset: offset_of!(PartInfoData, aabb),
            },
            FieldInfoData {
                name: "Translation",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PartInfoData, translation),
            },
        ],
    }),
    array_type: Some(PARTINFODATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PartInfoData {
    fn type_info() -> &'static TypeInfo {
        PARTINFODATA_TYPE_INFO
    }
}


pub const PARTINFODATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartInfoData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PartInfoData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PreSolveCollisionPhysicsCallbackHandlerData {
}

pub const PRESOLVECOLLISIONPHYSICSCALLBACKHANDLERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PreSolveCollisionPhysicsCallbackHandlerData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCALLBACKHANDLERDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESOLVECOLLISIONPHYSICSCALLBACKHANDLERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PreSolveCollisionPhysicsCallbackHandlerData {
    fn type_info() -> &'static TypeInfo {
        PRESOLVECOLLISIONPHYSICSCALLBACKHANDLERDATA_TYPE_INFO
    }
}


pub const PRESOLVECOLLISIONPHYSICSCALLBACKHANDLERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PreSolveCollisionPhysicsCallbackHandlerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PreSolveCollisionPhysicsCallbackHandlerData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VehiclePhysicsCallbackHandlerData {
}

pub const VEHICLEPHYSICSCALLBACKHANDLERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehiclePhysicsCallbackHandlerData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCALLBACKHANDLERDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VEHICLEPHYSICSCALLBACKHANDLERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VehiclePhysicsCallbackHandlerData {
    fn type_info() -> &'static TypeInfo {
        VEHICLEPHYSICSCALLBACKHANDLERDATA_TYPE_INFO
    }
}


pub const VEHICLEPHYSICSCALLBACKHANDLERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehiclePhysicsCallbackHandlerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("VehiclePhysicsCallbackHandlerData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientDynamicPhysicsCallbackHandlerData {
}

pub const CLIENTDYNAMICPHYSICSCALLBACKHANDLERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDynamicPhysicsCallbackHandlerData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCALLBACKHANDLERDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTDYNAMICPHYSICSCALLBACKHANDLERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ClientDynamicPhysicsCallbackHandlerData {
    fn type_info() -> &'static TypeInfo {
        CLIENTDYNAMICPHYSICSCALLBACKHANDLERDATA_TYPE_INFO
    }
}


pub const CLIENTDYNAMICPHYSICSCALLBACKHANDLERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDynamicPhysicsCallbackHandlerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ClientDynamicPhysicsCallbackHandlerData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LightDynamicPhysicsCallbackHandlerData {
}

pub const LIGHTDYNAMICPHYSICSCALLBACKHANDLERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightDynamicPhysicsCallbackHandlerData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCALLBACKHANDLERDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LIGHTDYNAMICPHYSICSCALLBACKHANDLERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LightDynamicPhysicsCallbackHandlerData {
    fn type_info() -> &'static TypeInfo {
        LIGHTDYNAMICPHYSICSCALLBACKHANDLERDATA_TYPE_INFO
    }
}


pub const LIGHTDYNAMICPHYSICSCALLBACKHANDLERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightDynamicPhysicsCallbackHandlerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("LightDynamicPhysicsCallbackHandlerData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DefaultPhysicsCallbackHandlerData {
}

pub const DEFAULTPHYSICSCALLBACKHANDLERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DefaultPhysicsCallbackHandlerData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCALLBACKHANDLERDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DEFAULTPHYSICSCALLBACKHANDLERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DefaultPhysicsCallbackHandlerData {
    fn type_info() -> &'static TypeInfo {
        DEFAULTPHYSICSCALLBACKHANDLERDATA_TYPE_INFO
    }
}


pub const DEFAULTPHYSICSCALLBACKHANDLERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DefaultPhysicsCallbackHandlerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("DefaultPhysicsCallbackHandlerData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NonePhysicsCallbackHandlerData {
}

pub const NONEPHYSICSCALLBACKHANDLERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NonePhysicsCallbackHandlerData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCALLBACKHANDLERDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(NONEPHYSICSCALLBACKHANDLERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NonePhysicsCallbackHandlerData {
    fn type_info() -> &'static TypeInfo {
        NONEPHYSICSCALLBACKHANDLERDATA_TYPE_INFO
    }
}


pub const NONEPHYSICSCALLBACKHANDLERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NonePhysicsCallbackHandlerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("NonePhysicsCallbackHandlerData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PhysicsCallbackHandlerData {
}

pub const PHYSICSCALLBACKHANDLERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsCallbackHandlerData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSCALLBACKHANDLERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PhysicsCallbackHandlerData {
    fn type_info() -> &'static TypeInfo {
        PHYSICSCALLBACKHANDLERDATA_TYPE_INFO
    }
}


pub const PHYSICSCALLBACKHANDLERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsCallbackHandlerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsCallbackHandlerData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum RigidBodyCollisionLayer {
    #[default]
    RigidBodyCollisionLayer_Invalid = 0,
    RigidBodyCollisionLayer_StaticLayer = 1,
    RigidBodyCollisionLayer_DynamicLayer = 2,
    RigidBodyCollisionLayer_PlayerCollisionLayer = 3,
    RigidBodyCollisionLayer_AICollisionLayer = 4,
    RigidBodyCollisionLayer_KeyframeLayer = 5,
    RigidBodyCollisionLayer_DebrisLayer = 6,
    RigidBodyCollisionLayer_FastDebrisLayer = 7,
    RigidBodyCollisionLayer_OnlyStaticCollisionLayer = 8,
    RigidBodyCollisionLayer_RagdollLayer = 9,
    RigidBodyCollisionLayer_NoCollisionLayer = 10,
    RigidBodyCollisionLayer_WaterLayer = 11,
    RigidBodyCollisionLayer_BangerLayer = 12,
    RigidBodyCollisionLayer_NoVehicleCollisionLayer = 13,
    RigidBodyCollisionLayer_CharacterLayer = 14,
    RigidBodyCollisionLayer_DynamicNoCharacterCollisionLayer = 15,
    RigidBodyCollisionLayer_PredictedVehicleLayer = 16,
    RigidBodyCollisionLayer_TerrainLayer = 17,
    RigidBodyCollisionLayer_OnlyTerrainCollionLayer = 18,
    RigidBodyCollisionLayer_CharacterCollisionGeometryLayer = 19,
    RigidBodyCollisionLayer_AiCollisionBodyLayer = 20,
    RigidBodyCollisionLayer_CameraCollisionLayer = 21,
    RigidBodyCollisionLayer_OnlyStaticCameraCollisionLayer = 22,
    RigidBodyCollisionLayer_KeyframedCollisionBodyLayer = 23,
    RigidBodyCollisionLayer_VehicleLayer = 24,
    RigidBodyCollisionLayer_VehicleAndCharacterCollisionLayer = 25,
    RigidBodyCollisionLayer_BlockVehicleOnlyCollisionLayer = 26,
    RigidBodyCollisionLayer_TerrainAndStaticCollisionLayer = 27,
    RigidBodyCollisionLayer_StarfighterLayer = 28,
    RigidBodyCollisionLayer_DefaultQueryLayer = 29,
    RigidBodyCollisionLayer_Size = 30,
}

pub const RIGIDBODYCOLLISIONLAYER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RigidBodyCollisionLayer",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(RIGIDBODYCOLLISIONLAYER_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RigidBodyCollisionLayer {
    fn type_info() -> &'static TypeInfo {
        RIGIDBODYCOLLISIONLAYER_TYPE_INFO
    }
}


pub const RIGIDBODYCOLLISIONLAYER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RigidBodyCollisionLayer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("RigidBodyCollisionLayer-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum RigidBodyQualityType {
    #[default]
    RigidBodyQualityType_Fixed = 0,
    RigidBodyQualityType_Debris = 1,
    RigidBodyQualityType_Dynamic = 2,
    RigidBodyQualityType_NeighborWelding = 3,
    RigidBodyQualityType_MotionWelding = 4,
    RigidBodyQualityType_TriangleWelding = 5,
    RigidBodyQualityType_Critical = 6,
    RigidBodyQualityType_Vehicle = 7,
    RigidBodyQualityType_Character = 8,
    RigidBodyQualityType_Grenade = 9,
    RigidBodyQualityType_Projectile = 10,
    RigidBodyQualityType_Invalid = 11,
}

pub const RIGIDBODYQUALITYTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RigidBodyQualityType",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(RIGIDBODYQUALITYTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RigidBodyQualityType {
    fn type_info() -> &'static TypeInfo {
        RIGIDBODYQUALITYTYPE_TYPE_INFO
    }
}


pub const RIGIDBODYQUALITYTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RigidBodyQualityType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("RigidBodyQualityType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum RigidBodyMotionType {
    #[default]
    RigidBodyMotionType_Invalid = 0,
    RigidBodyMotionType_Fixed = 1,
    RigidBodyMotionType_Keyframed = 2,
    RigidBodyMotionType_Dynamic = 3,
    RigidBodyMotionType_Size = 4,
}

pub const RIGIDBODYMOTIONTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RigidBodyMotionType",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(RIGIDBODYMOTIONTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RigidBodyMotionType {
    fn type_info() -> &'static TypeInfo {
        RIGIDBODYMOTIONTYPE_TYPE_INFO
    }
}


pub const RIGIDBODYMOTIONTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RigidBodyMotionType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("RigidBodyMotionType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum RigidBodyType {
    #[default]
    RBTypeCollision = 0,
    RBTypeDetail = 1,
    RBTypeCharacter = 2,
    RBTypeRaycast = 3,
    RBTypeGroup = 4,
    RBTypeProxy = 5,
    RBTypeCloth = 6,
    RBTypeSize = 7,
}

pub const RIGIDBODYTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RigidBodyType",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(RIGIDBODYTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RigidBodyType {
    fn type_info() -> &'static TypeInfo {
        RIGIDBODYTYPE_TYPE_INFO
    }
}


pub const RIGIDBODYTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RigidBodyType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("RigidBodyType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PhysicsBodyLauncherEntityData {
    pub realm: super::core::Realm,
    pub use_local_body_space: bool,
    pub launch_transform: super::core::LinearTransform,
    pub linear_velocity: super::core::Vec3,
    pub angular_velocity: super::core::Vec3,
}

pub const PHYSICSBODYLAUNCHERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsBodyLauncherEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(PhysicsBodyLauncherEntityData, realm),
            },
            FieldInfoData {
                name: "UseLocalBodySpace",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsBodyLauncherEntityData, use_local_body_space),
            },
            FieldInfoData {
                name: "LaunchTransform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(PhysicsBodyLauncherEntityData, launch_transform),
            },
            FieldInfoData {
                name: "LinearVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PhysicsBodyLauncherEntityData, linear_velocity),
            },
            FieldInfoData {
                name: "AngularVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PhysicsBodyLauncherEntityData, angular_velocity),
            },
        ],
    }),
    array_type: Some(PHYSICSBODYLAUNCHERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsBodyLauncherEntityData {
    fn type_info() -> &'static TypeInfo {
        PHYSICSBODYLAUNCHERENTITYDATA_TYPE_INFO
    }
}


pub const PHYSICSBODYLAUNCHERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsBodyLauncherEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsBodyLauncherEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PhysicsDebugSettings {
    pub debug_hinge_constraints: bool,
    pub use_physics_cpu_timers: bool,
    pub timing_recursion_depth: u32,
    pub use_brute_force_broadphase: bool,
    pub v_d_b_shows_effects_world: bool,
    pub v_d_b_enable_havok_monitors: bool,
    pub v_d_b_enable_capture_to_file: bool,
    pub v_d_b_capture_file_name: String,
    pub v_d_b_connection_time_out: u32,
    pub v_d_b_show_body_ids: bool,
    pub v_d_b_show_motion_ids: bool,
    pub v_d_b_show_broadphase: bool,
    pub v_d_b_show_shapes: bool,
    pub v_d_b_show_mass_properties: bool,
    pub v_d_b_show_constraints: bool,
    pub v_d_b_show_manifold: bool,
    pub v_d_b_show_motion_ttrail: bool,
    pub v_d_b_show_bounding_radius: bool,
    pub v_d_b_show_deactivation: bool,
    pub v_b_d_show_cell: bool,
    pub v_d_b_show_triangle_welding: bool,
    pub v_d_b_enable_world_snapshot: bool,
    pub v_d_b_show_destruction: bool,
}

pub const PHYSICSDEBUGSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsDebugSettings",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DebugHingeConstraints",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsDebugSettings, debug_hinge_constraints),
            },
            FieldInfoData {
                name: "UsePhysicsCpuTimers",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsDebugSettings, use_physics_cpu_timers),
            },
            FieldInfoData {
                name: "TimingRecursionDepth",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsDebugSettings, timing_recursion_depth),
            },
            FieldInfoData {
                name: "UseBruteForceBroadphase",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsDebugSettings, use_brute_force_broadphase),
            },
            FieldInfoData {
                name: "VDBShowsEffectsWorld",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsDebugSettings, v_d_b_shows_effects_world),
            },
            FieldInfoData {
                name: "VDBEnableHavokMonitors",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsDebugSettings, v_d_b_enable_havok_monitors),
            },
            FieldInfoData {
                name: "VDBEnableCaptureToFile",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsDebugSettings, v_d_b_enable_capture_to_file),
            },
            FieldInfoData {
                name: "VDBCaptureFileName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(PhysicsDebugSettings, v_d_b_capture_file_name),
            },
            FieldInfoData {
                name: "VDBConnectionTimeOut",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsDebugSettings, v_d_b_connection_time_out),
            },
            FieldInfoData {
                name: "VDBShowBodyIds",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsDebugSettings, v_d_b_show_body_ids),
            },
            FieldInfoData {
                name: "VDBShowMotionIds",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsDebugSettings, v_d_b_show_motion_ids),
            },
            FieldInfoData {
                name: "VDBShowBroadphase",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsDebugSettings, v_d_b_show_broadphase),
            },
            FieldInfoData {
                name: "VDBShowShapes",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsDebugSettings, v_d_b_show_shapes),
            },
            FieldInfoData {
                name: "VDBShowMassProperties",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsDebugSettings, v_d_b_show_mass_properties),
            },
            FieldInfoData {
                name: "VDBShowConstraints",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsDebugSettings, v_d_b_show_constraints),
            },
            FieldInfoData {
                name: "VDBShowManifold",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsDebugSettings, v_d_b_show_manifold),
            },
            FieldInfoData {
                name: "VDBShowMotionTtrail",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsDebugSettings, v_d_b_show_motion_ttrail),
            },
            FieldInfoData {
                name: "VDBShowBoundingRadius",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsDebugSettings, v_d_b_show_bounding_radius),
            },
            FieldInfoData {
                name: "VDBShowDeactivation",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsDebugSettings, v_d_b_show_deactivation),
            },
            FieldInfoData {
                name: "VBDShowCell",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsDebugSettings, v_b_d_show_cell),
            },
            FieldInfoData {
                name: "VDBShowTriangleWelding",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsDebugSettings, v_d_b_show_triangle_welding),
            },
            FieldInfoData {
                name: "VDBEnableWorldSnapshot",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsDebugSettings, v_d_b_enable_world_snapshot),
            },
            FieldInfoData {
                name: "VDBShowDestruction",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsDebugSettings, v_d_b_show_destruction),
            },
        ],
    }),
    array_type: Some(PHYSICSDEBUGSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PhysicsDebugSettings {
    fn type_info() -> &'static TypeInfo {
        PHYSICSDEBUGSETTINGS_TYPE_INFO
    }
}


pub const PHYSICSDEBUGSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsDebugSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsDebugSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PhysicsSettings {
    pub enable: bool,
    pub run_client_simulation_single_threaded: bool,
    pub run_effect_simulation_single_threaded: bool,
    pub run_server_simulation_single_threaded: bool,
    pub wind_manager_awakening_radius: f32,
    pub enable_a_i_rigid_body: bool,
    pub forest_enable: bool,
    pub enable_jobs: bool,
    pub remove_ragdoll_when_woken: bool,
    pub remove_from_world_on_collision_overflow: bool,
    pub single_step_character: bool,
    pub force_single_step_character_in_s_p: bool,
    pub enable_follow_wheel_raycasts: bool,
    pub enable_client_wheel_raycasts: bool,
    pub enable_a_sync_wheel_raycasts: bool,
    pub use_delayed_wake_up_client: bool,
    pub use_delayed_wake_up_server: bool,
    pub suppress_debris_spawn_until_ready: bool,
    pub enable_player_v_s_a_i_collisions: bool,
    pub enable_client_keyframed_collisions: bool,
    pub enable_f_x_keyframed_collisions: bool,
    pub block_stream_allocator_memory: u32,
    pub step_local_stream_allocator: u32,
    pub body_buffer_capacity: u32,
    pub motion_buffer_capacity: u32,
    pub constraint_buffer_capacity: u32,
    pub havok_disable_free_list_allocator: bool,
    pub enable_remove_from_world_keep_active: bool,
    pub enable_physics_state_stream: bool,
}

pub const PHYSICSSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsSettings",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsSettings, enable),
            },
            FieldInfoData {
                name: "RunClientSimulationSingleThreaded",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsSettings, run_client_simulation_single_threaded),
            },
            FieldInfoData {
                name: "RunEffectSimulationSingleThreaded",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsSettings, run_effect_simulation_single_threaded),
            },
            FieldInfoData {
                name: "RunServerSimulationSingleThreaded",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsSettings, run_server_simulation_single_threaded),
            },
            FieldInfoData {
                name: "WindManagerAwakeningRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsSettings, wind_manager_awakening_radius),
            },
            FieldInfoData {
                name: "EnableAIRigidBody",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsSettings, enable_a_i_rigid_body),
            },
            FieldInfoData {
                name: "ForestEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsSettings, forest_enable),
            },
            FieldInfoData {
                name: "EnableJobs",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsSettings, enable_jobs),
            },
            FieldInfoData {
                name: "RemoveRagdollWhenWoken",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsSettings, remove_ragdoll_when_woken),
            },
            FieldInfoData {
                name: "RemoveFromWorldOnCollisionOverflow",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsSettings, remove_from_world_on_collision_overflow),
            },
            FieldInfoData {
                name: "SingleStepCharacter",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsSettings, single_step_character),
            },
            FieldInfoData {
                name: "ForceSingleStepCharacterInSP",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsSettings, force_single_step_character_in_s_p),
            },
            FieldInfoData {
                name: "EnableFollowWheelRaycasts",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsSettings, enable_follow_wheel_raycasts),
            },
            FieldInfoData {
                name: "EnableClientWheelRaycasts",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsSettings, enable_client_wheel_raycasts),
            },
            FieldInfoData {
                name: "EnableASyncWheelRaycasts",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsSettings, enable_a_sync_wheel_raycasts),
            },
            FieldInfoData {
                name: "UseDelayedWakeUpClient",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsSettings, use_delayed_wake_up_client),
            },
            FieldInfoData {
                name: "UseDelayedWakeUpServer",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsSettings, use_delayed_wake_up_server),
            },
            FieldInfoData {
                name: "SuppressDebrisSpawnUntilReady",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsSettings, suppress_debris_spawn_until_ready),
            },
            FieldInfoData {
                name: "EnablePlayerVSAICollisions",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsSettings, enable_player_v_s_a_i_collisions),
            },
            FieldInfoData {
                name: "EnableClientKeyframedCollisions",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsSettings, enable_client_keyframed_collisions),
            },
            FieldInfoData {
                name: "EnableFXKeyframedCollisions",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsSettings, enable_f_x_keyframed_collisions),
            },
            FieldInfoData {
                name: "BlockStreamAllocatorMemory",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsSettings, block_stream_allocator_memory),
            },
            FieldInfoData {
                name: "StepLocalStreamAllocator",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsSettings, step_local_stream_allocator),
            },
            FieldInfoData {
                name: "BodyBufferCapacity",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsSettings, body_buffer_capacity),
            },
            FieldInfoData {
                name: "MotionBufferCapacity",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsSettings, motion_buffer_capacity),
            },
            FieldInfoData {
                name: "ConstraintBufferCapacity",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsSettings, constraint_buffer_capacity),
            },
            FieldInfoData {
                name: "HavokDisableFreeListAllocator",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsSettings, havok_disable_free_list_allocator),
            },
            FieldInfoData {
                name: "EnableRemoveFromWorldKeepActive",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsSettings, enable_remove_from_world_keep_active),
            },
            FieldInfoData {
                name: "EnablePhysicsStateStream",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsSettings, enable_physics_state_stream),
            },
        ],
    }),
    array_type: Some(PHYSICSSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PhysicsSettings {
    fn type_info() -> &'static TypeInfo {
        PHYSICSSETTINGS_TYPE_INFO
    }
}


pub const PHYSICSSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum PhysicsWorldType {
    #[default]
    PhysicsWorldType_Client = 0,
    PhysicsWorldType_ClientEffect = 1,
    PhysicsWorldType_Server = 2,
    PhysicsWorldType_Max = 3,
}

pub const PHYSICSWORLDTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsWorldType",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(PHYSICSWORLDTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PhysicsWorldType {
    fn type_info() -> &'static TypeInfo {
        PHYSICSWORLDTYPE_TYPE_INFO
    }
}


pub const PHYSICSWORLDTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsWorldType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsWorldType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PhysicsRenderSettings {
    pub render_server: bool,
    pub render_client: bool,
    pub render_effect_world: bool,
    pub render_state_stream_world: bool,
    pub render_static: bool,
    pub render_detail: bool,
    pub render_group: bool,
    pub render_ungrouped: bool,
    pub render_ragdoll: bool,
    pub render_water: bool,
    pub render_terrain: bool,
    pub render_terrain_min_max_level: i32,
    pub render_characters: bool,
    pub render_aabb_trigger: bool,
    pub render_character_collision: bool,
    pub view_distance: f32,
    pub render_solid_geometry: bool,
    pub use_shape_cache: bool,
    pub depth_test: bool,
    pub render_constraints: bool,
    pub render_contacts: bool,
    pub render_only_contact_constraints: bool,
    pub render_simulation_islands: bool,
    pub render_broadphase_handles: bool,
    pub render_destruction_connections: bool,
    pub render_sleep_status: bool,
    pub render_quality_type: bool,
    pub render_part_bounding_boxes: bool,
    pub render_only_bounding_boxes: bool,
    pub render_rigid_body_transform: bool,
    pub render_inertia: bool,
    pub render_center_of_mass: bool,
    pub render_linear_velocity: bool,
    pub render_collision_spheres: bool,
    pub render_ray_queries: bool,
    pub render_shapes: bool,
    pub render_drive_targets: bool,
    pub render_entity_stats: bool,
    pub render_memory_used: bool,
    pub collision_spawn_debug: bool,
    pub render_specific_part: i32,
}

pub const PHYSICSRENDERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRenderSettings",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "RenderServer",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, render_server),
            },
            FieldInfoData {
                name: "RenderClient",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, render_client),
            },
            FieldInfoData {
                name: "RenderEffectWorld",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, render_effect_world),
            },
            FieldInfoData {
                name: "RenderStateStreamWorld",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, render_state_stream_world),
            },
            FieldInfoData {
                name: "RenderStatic",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, render_static),
            },
            FieldInfoData {
                name: "RenderDetail",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, render_detail),
            },
            FieldInfoData {
                name: "RenderGroup",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, render_group),
            },
            FieldInfoData {
                name: "RenderUngrouped",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, render_ungrouped),
            },
            FieldInfoData {
                name: "RenderRagdoll",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, render_ragdoll),
            },
            FieldInfoData {
                name: "RenderWater",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, render_water),
            },
            FieldInfoData {
                name: "RenderTerrain",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, render_terrain),
            },
            FieldInfoData {
                name: "RenderTerrainMinMaxLevel",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, render_terrain_min_max_level),
            },
            FieldInfoData {
                name: "RenderCharacters",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, render_characters),
            },
            FieldInfoData {
                name: "RenderAabbTrigger",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, render_aabb_trigger),
            },
            FieldInfoData {
                name: "RenderCharacterCollision",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, render_character_collision),
            },
            FieldInfoData {
                name: "ViewDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, view_distance),
            },
            FieldInfoData {
                name: "RenderSolidGeometry",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, render_solid_geometry),
            },
            FieldInfoData {
                name: "UseShapeCache",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, use_shape_cache),
            },
            FieldInfoData {
                name: "DepthTest",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, depth_test),
            },
            FieldInfoData {
                name: "RenderConstraints",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, render_constraints),
            },
            FieldInfoData {
                name: "RenderContacts",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, render_contacts),
            },
            FieldInfoData {
                name: "RenderOnlyContactConstraints",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, render_only_contact_constraints),
            },
            FieldInfoData {
                name: "RenderSimulationIslands",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, render_simulation_islands),
            },
            FieldInfoData {
                name: "RenderBroadphaseHandles",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, render_broadphase_handles),
            },
            FieldInfoData {
                name: "RenderDestructionConnections",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, render_destruction_connections),
            },
            FieldInfoData {
                name: "RenderSleepStatus",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, render_sleep_status),
            },
            FieldInfoData {
                name: "RenderQualityType",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, render_quality_type),
            },
            FieldInfoData {
                name: "RenderPartBoundingBoxes",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, render_part_bounding_boxes),
            },
            FieldInfoData {
                name: "RenderOnlyBoundingBoxes",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, render_only_bounding_boxes),
            },
            FieldInfoData {
                name: "RenderRigidBodyTransform",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, render_rigid_body_transform),
            },
            FieldInfoData {
                name: "RenderInertia",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, render_inertia),
            },
            FieldInfoData {
                name: "RenderCenterOfMass",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, render_center_of_mass),
            },
            FieldInfoData {
                name: "RenderLinearVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, render_linear_velocity),
            },
            FieldInfoData {
                name: "RenderCollisionSpheres",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, render_collision_spheres),
            },
            FieldInfoData {
                name: "RenderRayQueries",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, render_ray_queries),
            },
            FieldInfoData {
                name: "RenderShapes",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, render_shapes),
            },
            FieldInfoData {
                name: "RenderDriveTargets",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, render_drive_targets),
            },
            FieldInfoData {
                name: "RenderEntityStats",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, render_entity_stats),
            },
            FieldInfoData {
                name: "RenderMemoryUsed",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, render_memory_used),
            },
            FieldInfoData {
                name: "CollisionSpawnDebug",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, collision_spawn_debug),
            },
            FieldInfoData {
                name: "RenderSpecificPart",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderSettings, render_specific_part),
            },
        ],
    }),
    array_type: Some(PHYSICSRENDERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PhysicsRenderSettings {
    fn type_info() -> &'static TypeInfo {
        PHYSICSRENDERSETTINGS_TYPE_INFO
    }
}


pub const PHYSICSRENDERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRenderSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsRenderSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PhysicsComponentOnPreSolveCollisionMessage {
}

pub const PHYSICSCOMPONENTONPRESOLVECOLLISIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsComponentOnPreSolveCollisionMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Physics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for PhysicsComponentOnPreSolveCollisionMessage {
    fn type_info() -> &'static TypeInfo {
        PHYSICSCOMPONENTONPRESOLVECOLLISIONMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PhysicsComponentOnImpulseMessage {
}

pub const PHYSICSCOMPONENTONIMPULSEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsComponentOnImpulseMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Physics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for PhysicsComponentOnImpulseMessage {
    fn type_info() -> &'static TypeInfo {
        PHYSICSCOMPONENTONIMPULSEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct PhysicsPrismaticConstraintInitialStanceData {
}

pub const PHYSICSPRISMATICCONSTRAINTINITIALSTANCEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsPrismaticConstraintInitialStanceData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINTINITIALSTANCEDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSPRISMATICCONSTRAINTINITIALSTANCEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsPrismaticConstraintInitialStanceData {
    fn type_info() -> &'static TypeInfo {
        PHYSICSPRISMATICCONSTRAINTINITIALSTANCEDATA_TYPE_INFO
    }
}


pub const PHYSICSPRISMATICCONSTRAINTINITIALSTANCEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsPrismaticConstraintInitialStanceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsPrismaticConstraintInitialStanceData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PhysicsRootControlConstraintInitialStanceData {
}

pub const PHYSICSROOTCONTROLCONSTRAINTINITIALSTANCEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRootControlConstraintInitialStanceData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINTINITIALSTANCEDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSROOTCONTROLCONSTRAINTINITIALSTANCEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsRootControlConstraintInitialStanceData {
    fn type_info() -> &'static TypeInfo {
        PHYSICSROOTCONTROLCONSTRAINTINITIALSTANCEDATA_TYPE_INFO
    }
}


pub const PHYSICSROOTCONTROLCONSTRAINTINITIALSTANCEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRootControlConstraintInitialStanceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsRootControlConstraintInitialStanceData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PhysicsRagdollConstraintInitialStanceData {
}

pub const PHYSICSRAGDOLLCONSTRAINTINITIALSTANCEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRagdollConstraintInitialStanceData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINTINITIALSTANCEDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSRAGDOLLCONSTRAINTINITIALSTANCEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsRagdollConstraintInitialStanceData {
    fn type_info() -> &'static TypeInfo {
        PHYSICSRAGDOLLCONSTRAINTINITIALSTANCEDATA_TYPE_INFO
    }
}


pub const PHYSICSRAGDOLLCONSTRAINTINITIALSTANCEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRagdollConstraintInitialStanceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsRagdollConstraintInitialStanceData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PhysicsHingeConstraintInitialStanceData {
}

pub const PHYSICSHINGECONSTRAINTINITIALSTANCEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsHingeConstraintInitialStanceData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINTINITIALSTANCEDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSHINGECONSTRAINTINITIALSTANCEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsHingeConstraintInitialStanceData {
    fn type_info() -> &'static TypeInfo {
        PHYSICSHINGECONSTRAINTINITIALSTANCEDATA_TYPE_INFO
    }
}


pub const PHYSICSHINGECONSTRAINTINITIALSTANCEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsHingeConstraintInitialStanceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsHingeConstraintInitialStanceData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PhysicsBallAndSocketConstraintInitialStanceData {
}

pub const PHYSICSBALLANDSOCKETCONSTRAINTINITIALSTANCEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsBallAndSocketConstraintInitialStanceData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINTINITIALSTANCEDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSBALLANDSOCKETCONSTRAINTINITIALSTANCEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsBallAndSocketConstraintInitialStanceData {
    fn type_info() -> &'static TypeInfo {
        PHYSICSBALLANDSOCKETCONSTRAINTINITIALSTANCEDATA_TYPE_INFO
    }
}


pub const PHYSICSBALLANDSOCKETCONSTRAINTINITIALSTANCEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsBallAndSocketConstraintInitialStanceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsBallAndSocketConstraintInitialStanceData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PhysicsConstraintInitialStanceData {
    pub transform: super::core::LinearTransform,
}

pub const PHYSICSCONSTRAINTINITIALSTANCEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsConstraintInitialStanceData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(PhysicsConstraintInitialStanceData, transform),
            },
        ],
    }),
    array_type: Some(PHYSICSCONSTRAINTINITIALSTANCEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsConstraintInitialStanceData {
    fn type_info() -> &'static TypeInfo {
        PHYSICSCONSTRAINTINITIALSTANCEDATA_TYPE_INFO
    }
}


pub const PHYSICSCONSTRAINTINITIALSTANCEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsConstraintInitialStanceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsConstraintInitialStanceData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PhysicsPrismaticConstraintData {
    pub has_limits: bool,
    pub linear_limit: f32,
    pub motor_data: PhysicsPrismaticConstraintMotorData,
}

pub const PHYSICSPRISMATICCONSTRAINTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsPrismaticConstraintData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "HasLimits",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsPrismaticConstraintData, has_limits),
            },
            FieldInfoData {
                name: "LinearLimit",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsPrismaticConstraintData, linear_limit),
            },
            FieldInfoData {
                name: "MotorData",
                flags: MemberInfoFlags::new(0),
                field_type: PHYSICSPRISMATICCONSTRAINTMOTORDATA_TYPE_INFO,
                rust_offset: offset_of!(PhysicsPrismaticConstraintData, motor_data),
            },
        ],
    }),
    array_type: Some(PHYSICSPRISMATICCONSTRAINTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsPrismaticConstraintData {
    fn type_info() -> &'static TypeInfo {
        PHYSICSPRISMATICCONSTRAINTDATA_TYPE_INFO
    }
}


pub const PHYSICSPRISMATICCONSTRAINTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsPrismaticConstraintData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsPrismaticConstraintData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PhysicsRootControlConstraintData {
    pub motor_data: PhysicsRootControlConstraintMotorData,
}

pub const PHYSICSROOTCONTROLCONSTRAINTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRootControlConstraintData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MotorData",
                flags: MemberInfoFlags::new(0),
                field_type: PHYSICSROOTCONTROLCONSTRAINTMOTORDATA_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRootControlConstraintData, motor_data),
            },
        ],
    }),
    array_type: Some(PHYSICSROOTCONTROLCONSTRAINTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsRootControlConstraintData {
    fn type_info() -> &'static TypeInfo {
        PHYSICSROOTCONTROLCONSTRAINTDATA_TYPE_INFO
    }
}


pub const PHYSICSROOTCONTROLCONSTRAINTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRootControlConstraintData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsRootControlConstraintData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PhysicsRagdollConstraintData {
    pub has_limits: bool,
    pub twist_min_angular_limit: f32,
    pub twist_max_angular_limit: f32,
    pub plane_min_angular_limit: f32,
    pub plane_max_angular_limit: f32,
    pub cone_angular_limit: f32,
    pub angular_friction: f32,
    pub angular_stiffness: f32,
    pub motor_data: PhysicsRagdollConstraintMotorData,
}

pub const PHYSICSRAGDOLLCONSTRAINTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRagdollConstraintData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "HasLimits",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRagdollConstraintData, has_limits),
            },
            FieldInfoData {
                name: "TwistMinAngularLimit",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRagdollConstraintData, twist_min_angular_limit),
            },
            FieldInfoData {
                name: "TwistMaxAngularLimit",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRagdollConstraintData, twist_max_angular_limit),
            },
            FieldInfoData {
                name: "PlaneMinAngularLimit",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRagdollConstraintData, plane_min_angular_limit),
            },
            FieldInfoData {
                name: "PlaneMaxAngularLimit",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRagdollConstraintData, plane_max_angular_limit),
            },
            FieldInfoData {
                name: "ConeAngularLimit",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRagdollConstraintData, cone_angular_limit),
            },
            FieldInfoData {
                name: "AngularFriction",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRagdollConstraintData, angular_friction),
            },
            FieldInfoData {
                name: "AngularStiffness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRagdollConstraintData, angular_stiffness),
            },
            FieldInfoData {
                name: "MotorData",
                flags: MemberInfoFlags::new(0),
                field_type: PHYSICSRAGDOLLCONSTRAINTMOTORDATA_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRagdollConstraintData, motor_data),
            },
        ],
    }),
    array_type: Some(PHYSICSRAGDOLLCONSTRAINTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsRagdollConstraintData {
    fn type_info() -> &'static TypeInfo {
        PHYSICSRAGDOLLCONSTRAINTDATA_TYPE_INFO
    }
}


pub const PHYSICSRAGDOLLCONSTRAINTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRagdollConstraintData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsRagdollConstraintData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PhysicsHingeConstraintData {
    pub has_limits: bool,
    pub min_angle: f32,
    pub max_angle: f32,
    pub angular_friction: f32,
    pub angular_stiffness: f32,
    pub motor_data: PhysicsHingeConstraintMotorData,
}

pub const PHYSICSHINGECONSTRAINTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsHingeConstraintData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "HasLimits",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsHingeConstraintData, has_limits),
            },
            FieldInfoData {
                name: "MinAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsHingeConstraintData, min_angle),
            },
            FieldInfoData {
                name: "MaxAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsHingeConstraintData, max_angle),
            },
            FieldInfoData {
                name: "AngularFriction",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsHingeConstraintData, angular_friction),
            },
            FieldInfoData {
                name: "AngularStiffness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsHingeConstraintData, angular_stiffness),
            },
            FieldInfoData {
                name: "MotorData",
                flags: MemberInfoFlags::new(0),
                field_type: PHYSICSHINGECONSTRAINTMOTORDATA_TYPE_INFO,
                rust_offset: offset_of!(PhysicsHingeConstraintData, motor_data),
            },
        ],
    }),
    array_type: Some(PHYSICSHINGECONSTRAINTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsHingeConstraintData {
    fn type_info() -> &'static TypeInfo {
        PHYSICSHINGECONSTRAINTDATA_TYPE_INFO
    }
}


pub const PHYSICSHINGECONSTRAINTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsHingeConstraintData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsHingeConstraintData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PhysicsBallAndSocketConstraintData {
}

pub const PHYSICSBALLANDSOCKETCONSTRAINTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsBallAndSocketConstraintData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINTDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSBALLANDSOCKETCONSTRAINTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsBallAndSocketConstraintData {
    fn type_info() -> &'static TypeInfo {
        PHYSICSBALLANDSOCKETCONSTRAINTDATA_TYPE_INFO
    }
}


pub const PHYSICSBALLANDSOCKETCONSTRAINTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsBallAndSocketConstraintData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsBallAndSocketConstraintData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PhysicsPrismaticConstraintMotorData {
    pub max_force: f32,
}

pub const PHYSICSPRISMATICCONSTRAINTMOTORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsPrismaticConstraintMotorData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINTMOTORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MaxForce",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsPrismaticConstraintMotorData, max_force),
            },
        ],
    }),
    array_type: Some(PHYSICSPRISMATICCONSTRAINTMOTORDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsPrismaticConstraintMotorData {
    fn type_info() -> &'static TypeInfo {
        PHYSICSPRISMATICCONSTRAINTMOTORDATA_TYPE_INFO
    }
}


pub const PHYSICSPRISMATICCONSTRAINTMOTORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsPrismaticConstraintMotorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsPrismaticConstraintMotorData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PhysicsRootControlConstraintMotorData {
    pub forward_max_force: f32,
    pub backward_max_force: f32,
    pub radial_max_force: f32,
    pub swing_max_torque: f32,
    pub twist_max_torque: f32,
}

pub const PHYSICSROOTCONTROLCONSTRAINTMOTORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRootControlConstraintMotorData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINTMOTORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ForwardMaxForce",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRootControlConstraintMotorData, forward_max_force),
            },
            FieldInfoData {
                name: "BackwardMaxForce",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRootControlConstraintMotorData, backward_max_force),
            },
            FieldInfoData {
                name: "RadialMaxForce",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRootControlConstraintMotorData, radial_max_force),
            },
            FieldInfoData {
                name: "SwingMaxTorque",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRootControlConstraintMotorData, swing_max_torque),
            },
            FieldInfoData {
                name: "TwistMaxTorque",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRootControlConstraintMotorData, twist_max_torque),
            },
        ],
    }),
    array_type: Some(PHYSICSROOTCONTROLCONSTRAINTMOTORDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsRootControlConstraintMotorData {
    fn type_info() -> &'static TypeInfo {
        PHYSICSROOTCONTROLCONSTRAINTMOTORDATA_TYPE_INFO
    }
}


pub const PHYSICSROOTCONTROLCONSTRAINTMOTORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRootControlConstraintMotorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsRootControlConstraintMotorData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PhysicsRagdollConstraintMotorData {
    pub swing_max_torque: f32,
    pub twist_max_torque: f32,
}

pub const PHYSICSRAGDOLLCONSTRAINTMOTORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRagdollConstraintMotorData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINTMOTORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "SwingMaxTorque",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRagdollConstraintMotorData, swing_max_torque),
            },
            FieldInfoData {
                name: "TwistMaxTorque",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRagdollConstraintMotorData, twist_max_torque),
            },
        ],
    }),
    array_type: Some(PHYSICSRAGDOLLCONSTRAINTMOTORDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsRagdollConstraintMotorData {
    fn type_info() -> &'static TypeInfo {
        PHYSICSRAGDOLLCONSTRAINTMOTORDATA_TYPE_INFO
    }
}


pub const PHYSICSRAGDOLLCONSTRAINTMOTORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRagdollConstraintMotorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsRagdollConstraintMotorData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PhysicsHingeConstraintMotorData {
    pub max_torque: f32,
}

pub const PHYSICSHINGECONSTRAINTMOTORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsHingeConstraintMotorData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINTMOTORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MaxTorque",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsHingeConstraintMotorData, max_torque),
            },
        ],
    }),
    array_type: Some(PHYSICSHINGECONSTRAINTMOTORDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsHingeConstraintMotorData {
    fn type_info() -> &'static TypeInfo {
        PHYSICSHINGECONSTRAINTMOTORDATA_TYPE_INFO
    }
}


pub const PHYSICSHINGECONSTRAINTMOTORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsHingeConstraintMotorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsHingeConstraintMotorData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PhysicsConstraintMotorData {
    pub transform: super::core::LinearTransform,
    pub spring_strength: f32,
    pub damping: f32,
}

pub const PHYSICSCONSTRAINTMOTORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsConstraintMotorData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(PhysicsConstraintMotorData, transform),
            },
            FieldInfoData {
                name: "SpringStrength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsConstraintMotorData, spring_strength),
            },
            FieldInfoData {
                name: "Damping",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsConstraintMotorData, damping),
            },
        ],
    }),
    array_type: Some(PHYSICSCONSTRAINTMOTORDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsConstraintMotorData {
    fn type_info() -> &'static TypeInfo {
        PHYSICSCONSTRAINTMOTORDATA_TYPE_INFO
    }
}


pub const PHYSICSCONSTRAINTMOTORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsConstraintMotorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsConstraintMotorData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PhysicsConstraintOwnerData {
    pub constraint_data: PhysicsConstraintData,
}

pub const PHYSICSCONSTRAINTOWNERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsConstraintOwnerData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ConstraintData",
                flags: MemberInfoFlags::new(0),
                field_type: PHYSICSCONSTRAINTDATA_TYPE_INFO,
                rust_offset: offset_of!(PhysicsConstraintOwnerData, constraint_data),
            },
        ],
    }),
    array_type: Some(PHYSICSCONSTRAINTOWNERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PhysicsConstraintOwnerData {
    fn type_info() -> &'static TypeInfo {
        PHYSICSCONSTRAINTOWNERDATA_TYPE_INFO
    }
}


pub const PHYSICSCONSTRAINTOWNERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsConstraintOwnerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsConstraintOwnerData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PhysicsConstraintData {
    pub transform: super::core::LinearTransform,
    pub is_breakable: bool,
    pub break_threshold: f32,
    pub stabilized: bool,
    pub enable_continuous_simulation: bool,
    pub realm: super::core::Realm,
    pub world_index: u8,
    pub initial_stance_data: PhysicsConstraintInitialStanceData,
}

pub const PHYSICSCONSTRAINTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsConstraintData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(PhysicsConstraintData, transform),
            },
            FieldInfoData {
                name: "IsBreakable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsConstraintData, is_breakable),
            },
            FieldInfoData {
                name: "BreakThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsConstraintData, break_threshold),
            },
            FieldInfoData {
                name: "Stabilized",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsConstraintData, stabilized),
            },
            FieldInfoData {
                name: "EnableContinuousSimulation",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsConstraintData, enable_continuous_simulation),
            },
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(PhysicsConstraintData, realm),
            },
            FieldInfoData {
                name: "WorldIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(PhysicsConstraintData, world_index),
            },
            FieldInfoData {
                name: "InitialStanceData",
                flags: MemberInfoFlags::new(0),
                field_type: PHYSICSCONSTRAINTINITIALSTANCEDATA_TYPE_INFO,
                rust_offset: offset_of!(PhysicsConstraintData, initial_stance_data),
            },
        ],
    }),
    array_type: Some(PHYSICSCONSTRAINTDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PhysicsConstraintData {
    fn type_info() -> &'static TypeInfo {
        PHYSICSCONSTRAINTDATA_TYPE_INFO
    }
}


pub const PHYSICSCONSTRAINTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsConstraintData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsConstraintData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ProximityTrackerData {
    pub half_extents: super::core::Vec3,
    pub collision_layer: u32,
}

pub const PROXIMITYTRACKERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProximityTrackerData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "HalfExtents",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ProximityTrackerData, half_extents),
            },
            FieldInfoData {
                name: "CollisionLayer",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ProximityTrackerData, collision_layer),
            },
        ],
    }),
    array_type: Some(PROXIMITYTRACKERDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ProximityTrackerData {
    fn type_info() -> &'static TypeInfo {
        PROXIMITYTRACKERDATA_TYPE_INFO
    }
}


pub const PROXIMITYTRACKERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProximityTrackerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ProximityTrackerData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AabbTriggerPhysicsBodyData {
    pub half_extents: super::core::Vec3,
}

pub const AABBTRIGGERPHYSICSBODYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AabbTriggerPhysicsBodyData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSBODYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "HalfExtents",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AabbTriggerPhysicsBodyData, half_extents),
            },
        ],
    }),
    array_type: Some(AABBTRIGGERPHYSICSBODYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AabbTriggerPhysicsBodyData {
    fn type_info() -> &'static TypeInfo {
        AABBTRIGGERPHYSICSBODYDATA_TYPE_INFO
    }
}


pub const AABBTRIGGERPHYSICSBODYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AabbTriggerPhysicsBodyData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("AabbTriggerPhysicsBodyData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TerrainPhysicsBodyData {
    pub use_s_d_f_collision: bool,
    pub rigid_body_index: u32,
    pub max_resolution_size_use_for_split: f32,
    pub s_d_f_thickness: f32,
}

pub const TERRAINPHYSICSBODYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainPhysicsBodyData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSBODYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "UseSDFCollision",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TerrainPhysicsBodyData, use_s_d_f_collision),
            },
            FieldInfoData {
                name: "RigidBodyIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainPhysicsBodyData, rigid_body_index),
            },
            FieldInfoData {
                name: "MaxResolutionSizeUseForSplit",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainPhysicsBodyData, max_resolution_size_use_for_split),
            },
            FieldInfoData {
                name: "SDFThickness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainPhysicsBodyData, s_d_f_thickness),
            },
        ],
    }),
    array_type: Some(TERRAINPHYSICSBODYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TerrainPhysicsBodyData {
    fn type_info() -> &'static TypeInfo {
        TERRAINPHYSICSBODYDATA_TYPE_INFO
    }
}


pub const TERRAINPHYSICSBODYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainPhysicsBodyData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("TerrainPhysicsBodyData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StaticWaterPhysicsBodyData {
}

pub const STATICWATERPHYSICSBODYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticWaterPhysicsBodyData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WATERPHYSICSBODYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(STATICWATERPHYSICSBODYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StaticWaterPhysicsBodyData {
    fn type_info() -> &'static TypeInfo {
        STATICWATERPHYSICSBODYDATA_TYPE_INFO
    }
}


pub const STATICWATERPHYSICSBODYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticWaterPhysicsBodyData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("StaticWaterPhysicsBodyData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WaterPhysicsBodyData {
}

pub const WATERPHYSICSBODYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterPhysicsBodyData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSBODYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WATERPHYSICSBODYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WaterPhysicsBodyData {
    fn type_info() -> &'static TypeInfo {
        WATERPHYSICSBODYDATA_TYPE_INFO
    }
}


pub const WATERPHYSICSBODYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterPhysicsBodyData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("WaterPhysicsBodyData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct GroupRigidBodyData {
    pub raycast_material_indices_lookups: Vec<MaterialIndicesLookup>,
}

pub const GROUPRIGIDBODYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroupRigidBodyData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RIGIDBODYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "RaycastMaterialIndicesLookups",
                flags: MemberInfoFlags::new(144),
                field_type: MATERIALINDICESLOOKUP_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(GroupRigidBodyData, raycast_material_indices_lookups),
            },
        ],
    }),
    array_type: Some(GROUPRIGIDBODYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for GroupRigidBodyData {
    fn type_info() -> &'static TypeInfo {
        GROUPRIGIDBODYDATA_TYPE_INFO
    }
}


pub const GROUPRIGIDBODYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroupRigidBodyData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("GroupRigidBodyData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MaterialIndicesLookup {
    pub material_indices: Vec<u32>,
}

pub const MATERIALINDICESLOOKUP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialIndicesLookup",
    flags: MemberInfoFlags::new(73),
    module: "Physics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "MaterialIndices",
                flags: MemberInfoFlags::new(144),
                field_type: UINT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MaterialIndicesLookup, material_indices),
            },
        ],
    }),
    array_type: Some(MATERIALINDICESLOOKUP_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MaterialIndicesLookup {
    fn type_info() -> &'static TypeInfo {
        MATERIALINDICESLOOKUP_TYPE_INFO
    }
}


pub const MATERIALINDICESLOOKUP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialIndicesLookup-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("MaterialIndicesLookup-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RigidBodyData {
    pub mass: f32,
    pub material: super::entity::MaterialDecl,
    pub dynamic_friction: f32,
    pub static_friction: f32,
    pub restitution: f32,
    pub compute_center_of_mass: bool,
    pub center_of_mass: super::core::Vec3,
    pub compute_inertia_tensor: bool,
    pub inertia_modifier: super::core::Vec3,
    pub angular_velocity_damping: f32,
    pub linear_velocity_damping: f32,
    pub system_group: u32,
    pub sub_system_id: u32,
    pub sub_system_id_no_collide: u32,
    pub part_indices: Vec<u32>,
    pub float_physics: FloatPhysicsData,
    pub transform: super::core::LinearTransform,
    pub is_root_controller: bool,
    pub part_materials: Vec<super::entity::MaterialDecl>,
    pub inverse_inertia_tensor: super::core::Vec3,
    pub principal_axes_of_inertia: super::core::Quat,
}

pub const RIGIDBODYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RigidBodyData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSBODYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Mass",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RigidBodyData, mass),
            },
            FieldInfoData {
                name: "Material",
                flags: MemberInfoFlags::new(0),
                field_type: MATERIALDECL_TYPE_INFO,
                rust_offset: offset_of!(RigidBodyData, material),
            },
            FieldInfoData {
                name: "DynamicFriction",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RigidBodyData, dynamic_friction),
            },
            FieldInfoData {
                name: "StaticFriction",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RigidBodyData, static_friction),
            },
            FieldInfoData {
                name: "Restitution",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RigidBodyData, restitution),
            },
            FieldInfoData {
                name: "ComputeCenterOfMass",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RigidBodyData, compute_center_of_mass),
            },
            FieldInfoData {
                name: "CenterOfMass",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(RigidBodyData, center_of_mass),
            },
            FieldInfoData {
                name: "ComputeInertiaTensor",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RigidBodyData, compute_inertia_tensor),
            },
            FieldInfoData {
                name: "InertiaModifier",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(RigidBodyData, inertia_modifier),
            },
            FieldInfoData {
                name: "AngularVelocityDamping",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RigidBodyData, angular_velocity_damping),
            },
            FieldInfoData {
                name: "LinearVelocityDamping",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RigidBodyData, linear_velocity_damping),
            },
            FieldInfoData {
                name: "SystemGroup",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(RigidBodyData, system_group),
            },
            FieldInfoData {
                name: "SubSystemId",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(RigidBodyData, sub_system_id),
            },
            FieldInfoData {
                name: "SubSystemIdNoCollide",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(RigidBodyData, sub_system_id_no_collide),
            },
            FieldInfoData {
                name: "PartIndices",
                flags: MemberInfoFlags::new(144),
                field_type: UINT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(RigidBodyData, part_indices),
            },
            FieldInfoData {
                name: "FloatPhysics",
                flags: MemberInfoFlags::new(0),
                field_type: FLOATPHYSICSDATA_TYPE_INFO,
                rust_offset: offset_of!(RigidBodyData, float_physics),
            },
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(RigidBodyData, transform),
            },
            FieldInfoData {
                name: "IsRootController",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RigidBodyData, is_root_controller),
            },
            FieldInfoData {
                name: "PartMaterials",
                flags: MemberInfoFlags::new(144),
                field_type: MATERIALDECL_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(RigidBodyData, part_materials),
            },
            FieldInfoData {
                name: "InverseInertiaTensor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(RigidBodyData, inverse_inertia_tensor),
            },
            FieldInfoData {
                name: "PrincipalAxesOfInertia",
                flags: MemberInfoFlags::new(0),
                field_type: QUAT_TYPE_INFO,
                rust_offset: offset_of!(RigidBodyData, principal_axes_of_inertia),
            },
        ],
    }),
    array_type: Some(RIGIDBODYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RigidBodyData {
    fn type_info() -> &'static TypeInfo {
        RIGIDBODYDATA_TYPE_INFO
    }
}


pub const RIGIDBODYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RigidBodyData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("RigidBodyData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PhysicsBodyData {
    pub realm: super::core::Realm,
    pub rigid_body_type: RigidBodyType,
    pub collision_layer: RigidBodyCollisionLayer,
    pub motion_type: RigidBodyMotionType,
    pub quality_type: RigidBodyQualityType,
    pub asset: super::core::Asset,
    pub transform_index: u8,
    pub world_index: u8,
    pub collision_root_shape_index: u8,
    pub raycast_root_shape_index: u8,
    pub add_to_spatial_query_manager: bool,
    pub physics_callback_handler: PhysicsCallbackHandlerData,
}

pub const PHYSICSBODYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsBodyData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(PhysicsBodyData, realm),
            },
            FieldInfoData {
                name: "RigidBodyType",
                flags: MemberInfoFlags::new(0),
                field_type: RIGIDBODYTYPE_TYPE_INFO,
                rust_offset: offset_of!(PhysicsBodyData, rigid_body_type),
            },
            FieldInfoData {
                name: "CollisionLayer",
                flags: MemberInfoFlags::new(0),
                field_type: RIGIDBODYCOLLISIONLAYER_TYPE_INFO,
                rust_offset: offset_of!(PhysicsBodyData, collision_layer),
            },
            FieldInfoData {
                name: "MotionType",
                flags: MemberInfoFlags::new(0),
                field_type: RIGIDBODYMOTIONTYPE_TYPE_INFO,
                rust_offset: offset_of!(PhysicsBodyData, motion_type),
            },
            FieldInfoData {
                name: "QualityType",
                flags: MemberInfoFlags::new(0),
                field_type: RIGIDBODYQUALITYTYPE_TYPE_INFO,
                rust_offset: offset_of!(PhysicsBodyData, quality_type),
            },
            FieldInfoData {
                name: "Asset",
                flags: MemberInfoFlags::new(0),
                field_type: ASSET_TYPE_INFO,
                rust_offset: offset_of!(PhysicsBodyData, asset),
            },
            FieldInfoData {
                name: "TransformIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(PhysicsBodyData, transform_index),
            },
            FieldInfoData {
                name: "WorldIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(PhysicsBodyData, world_index),
            },
            FieldInfoData {
                name: "CollisionRootShapeIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(PhysicsBodyData, collision_root_shape_index),
            },
            FieldInfoData {
                name: "RaycastRootShapeIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(PhysicsBodyData, raycast_root_shape_index),
            },
            FieldInfoData {
                name: "AddToSpatialQueryManager",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsBodyData, add_to_spatial_query_manager),
            },
            FieldInfoData {
                name: "PhysicsCallbackHandler",
                flags: MemberInfoFlags::new(0),
                field_type: PHYSICSCALLBACKHANDLERDATA_TYPE_INFO,
                rust_offset: offset_of!(PhysicsBodyData, physics_callback_handler),
            },
        ],
    }),
    array_type: Some(PHYSICSBODYDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PhysicsBodyData {
    fn type_info() -> &'static TypeInfo {
        PHYSICSBODYDATA_TYPE_INFO
    }
}


pub const PHYSICSBODYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsBodyData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsBodyData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct BoxFloatPhysicsData {
    pub box_action_data: FloatPhysicsActionData,
}

pub const BOXFLOATPHYSICSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoxFloatPhysicsData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(FLOATPHYSICSDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "BoxActionData",
                flags: MemberInfoFlags::new(0),
                field_type: FLOATPHYSICSACTIONDATA_TYPE_INFO,
                rust_offset: offset_of!(BoxFloatPhysicsData, box_action_data),
            },
        ],
    }),
    array_type: Some(BOXFLOATPHYSICSDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BoxFloatPhysicsData {
    fn type_info() -> &'static TypeInfo {
        BOXFLOATPHYSICSDATA_TYPE_INFO
    }
}


pub const BOXFLOATPHYSICSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoxFloatPhysicsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("BoxFloatPhysicsData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FloatPhysicsActionData {
}

pub const FLOATPHYSICSACTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatPhysicsActionData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSACTIONDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FLOATPHYSICSACTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FloatPhysicsActionData {
    fn type_info() -> &'static TypeInfo {
        FLOATPHYSICSACTIONDATA_TYPE_INFO
    }
}


pub const FLOATPHYSICSACTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatPhysicsActionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("FloatPhysicsActionData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WindPhysicsActionData {
    pub resistance_factor: f32,
}

pub const WINDPHYSICSACTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindPhysicsActionData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSACTIONDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ResistanceFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WindPhysicsActionData, resistance_factor),
            },
        ],
    }),
    array_type: Some(WINDPHYSICSACTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WindPhysicsActionData {
    fn type_info() -> &'static TypeInfo {
        WINDPHYSICSACTIONDATA_TYPE_INFO
    }
}


pub const WINDPHYSICSACTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindPhysicsActionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("WindPhysicsActionData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PhysicsActionData {
}

pub const PHYSICSACTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsActionData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSACTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PhysicsActionData {
    fn type_info() -> &'static TypeInfo {
        PHYSICSACTIONDATA_TYPE_INFO
    }
}


pub const PHYSICSACTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsActionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsActionData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PhysicsProxyEntityData {
    pub realm: super::core::Realm,
}

pub const PHYSICSPROXYENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsProxyEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENTENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(PhysicsProxyEntityData, realm),
            },
        ],
    }),
    array_type: Some(PHYSICSPROXYENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsProxyEntityData {
    fn type_info() -> &'static TypeInfo {
        PHYSICSPROXYENTITYDATA_TYPE_INFO
    }
}


pub const PHYSICSPROXYENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsProxyEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsProxyEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct GamePhysicsEntityData {
}

pub const GAMEPHYSICSENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GamePhysicsEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMECOMPONENTENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(GAMEPHYSICSENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for GamePhysicsEntityData {
    fn type_info() -> &'static TypeInfo {
        GAMEPHYSICSENTITYDATA_TYPE_INFO
    }
}


pub const GAMEPHYSICSENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GamePhysicsEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("GamePhysicsEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DefaultPartPhysicsComponentData {
}

pub const DEFAULTPARTPHYSICSCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DefaultPartPhysicsComponentData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PARTPHYSICSCOMPONENTDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DEFAULTPARTPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DefaultPartPhysicsComponentData {
    fn type_info() -> &'static TypeInfo {
        DEFAULTPARTPHYSICSCOMPONENTDATA_TYPE_INFO
    }
}


pub const DEFAULTPARTPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DefaultPartPhysicsComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("DefaultPartPhysicsComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PartPhysicsComponentData {
}

pub const PARTPHYSICSCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartPhysicsComponentData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCOMPONENTDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PARTPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PartPhysicsComponentData {
    fn type_info() -> &'static TypeInfo {
        PARTPHYSICSCOMPONENTDATA_TYPE_INFO
    }
}


pub const PARTPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartPhysicsComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PartPhysicsComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RagdollPhysicsComponentData {
    pub bodies_names_hashes: Vec<u32>,
}

pub const RAGDOLLPHYSICSCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RagdollPhysicsComponentData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "BodiesNamesHashes",
                flags: MemberInfoFlags::new(144),
                field_type: UINT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(RagdollPhysicsComponentData, bodies_names_hashes),
            },
        ],
    }),
    array_type: Some(RAGDOLLPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RagdollPhysicsComponentData {
    fn type_info() -> &'static TypeInfo {
        RAGDOLLPHYSICSCOMPONENTDATA_TYPE_INFO
    }
}


pub const RAGDOLLPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RagdollPhysicsComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("RagdollPhysicsComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ForceComponentData {
    pub wind_action_data: WindPhysicsActionData,
}

pub const FORCECOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForceComponentData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "WindActionData",
                flags: MemberInfoFlags::new(0),
                field_type: WINDPHYSICSACTIONDATA_TYPE_INFO,
                rust_offset: offset_of!(ForceComponentData, wind_action_data),
            },
        ],
    }),
    array_type: Some(FORCECOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ForceComponentData {
    fn type_info() -> &'static TypeInfo {
        FORCECOMPONENTDATA_TYPE_INFO
    }
}


pub const FORCECOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForceComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ForceComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PhysicsComponentData {
    pub physics_bodies: Vec<PhysicsBodyData>,
    pub physics_constraints: Vec<PhysicsConstraintData>,
    pub parts: Vec<PhysicsPartData>,
    pub movable_parts: bool,
    pub internal_collision_disabling: InternalCollisionDisablingBehavior,
    pub enable_collision_events: bool,
}

pub const PHYSICSCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsComponentData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "PhysicsBodies",
                flags: MemberInfoFlags::new(144),
                field_type: PHYSICSBODYDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(PhysicsComponentData, physics_bodies),
            },
            FieldInfoData {
                name: "PhysicsConstraints",
                flags: MemberInfoFlags::new(144),
                field_type: PHYSICSCONSTRAINTDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(PhysicsComponentData, physics_constraints),
            },
            FieldInfoData {
                name: "Parts",
                flags: MemberInfoFlags::new(144),
                field_type: PHYSICSPARTDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(PhysicsComponentData, parts),
            },
            FieldInfoData {
                name: "MovableParts",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsComponentData, movable_parts),
            },
            FieldInfoData {
                name: "InternalCollisionDisabling",
                flags: MemberInfoFlags::new(0),
                field_type: INTERNALCOLLISIONDISABLINGBEHAVIOR_TYPE_INFO,
                rust_offset: offset_of!(PhysicsComponentData, internal_collision_disabling),
            },
            FieldInfoData {
                name: "EnableCollisionEvents",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsComponentData, enable_collision_events),
            },
        ],
    }),
    array_type: Some(PHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PhysicsComponentData {
    fn type_info() -> &'static TypeInfo {
        PHYSICSCOMPONENTDATA_TYPE_INFO
    }
}


pub const PHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum InternalCollisionDisablingBehavior {
    #[default]
    InternalCollisionDisablingBehavior_Auto = 0,
    InternalCollisionDisablingBehavior_DisableNone = 1,
    InternalCollisionDisablingBehavior_DisableConstrained = 2,
}

pub const INTERNALCOLLISIONDISABLINGBEHAVIOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InternalCollisionDisablingBehavior",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(INTERNALCOLLISIONDISABLINGBEHAVIOR_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for InternalCollisionDisablingBehavior {
    fn type_info() -> &'static TypeInfo {
        INTERNALCOLLISIONDISABLINGBEHAVIOR_TYPE_INFO
    }
}


pub const INTERNALCOLLISIONDISABLINGBEHAVIOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InternalCollisionDisablingBehavior-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("InternalCollisionDisablingBehavior-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PhysicsPartData {
    pub aabb: super::core::AxisAlignedBox,
    pub transform_node: super::entity::GameObjectData,
}

pub const PHYSICSPARTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsPartData",
    flags: MemberInfoFlags::new(73),
    module: "Physics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Aabb",
                flags: MemberInfoFlags::new(0),
                field_type: AXISALIGNEDBOX_TYPE_INFO,
                rust_offset: offset_of!(PhysicsPartData, aabb),
            },
            FieldInfoData {
                name: "TransformNode",
                flags: MemberInfoFlags::new(0),
                field_type: GAMEOBJECTDATA_TYPE_INFO,
                rust_offset: offset_of!(PhysicsPartData, transform_node),
            },
        ],
    }),
    array_type: Some(PHYSICSPARTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsPartData {
    fn type_info() -> &'static TypeInfo {
        PHYSICSPARTDATA_TYPE_INFO
    }
}


pub const PHYSICSPARTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsPartData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsPartData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MaterialPropertyPhysicsDestructionData {
    pub density: f32,
    pub destruction_radius: f32,
    pub elasticity: f32,
    pub yield_strength: f32,
    pub breaking_strength: f32,
}

pub const MATERIALPROPERTYPHYSICSDESTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialPropertyPhysicsDestructionData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSMATERIALRELATIONPROPERTYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Density",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MaterialPropertyPhysicsDestructionData, density),
            },
            FieldInfoData {
                name: "DestructionRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MaterialPropertyPhysicsDestructionData, destruction_radius),
            },
            FieldInfoData {
                name: "Elasticity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MaterialPropertyPhysicsDestructionData, elasticity),
            },
            FieldInfoData {
                name: "YieldStrength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MaterialPropertyPhysicsDestructionData, yield_strength),
            },
            FieldInfoData {
                name: "BreakingStrength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MaterialPropertyPhysicsDestructionData, breaking_strength),
            },
        ],
    }),
    array_type: Some(MATERIALPROPERTYPHYSICSDESTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MaterialPropertyPhysicsDestructionData {
    fn type_info() -> &'static TypeInfo {
        MATERIALPROPERTYPHYSICSDESTRUCTIONDATA_TYPE_INFO
    }
}


pub const MATERIALPROPERTYPHYSICSDESTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialPropertyPhysicsDestructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("MaterialPropertyPhysicsDestructionData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MaterialPropertyPhysicsData {
    pub dynamic_friction: f32,
    pub static_friction: f32,
    pub restitution: f32,
    pub dynamic_friction_modifier: f32,
    pub static_friction_modifier: f32,
    pub resistance: f32,
}

pub const MATERIALPROPERTYPHYSICSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialPropertyPhysicsData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSMATERIALRELATIONPROPERTYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DynamicFriction",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MaterialPropertyPhysicsData, dynamic_friction),
            },
            FieldInfoData {
                name: "StaticFriction",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MaterialPropertyPhysicsData, static_friction),
            },
            FieldInfoData {
                name: "Restitution",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MaterialPropertyPhysicsData, restitution),
            },
            FieldInfoData {
                name: "DynamicFrictionModifier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MaterialPropertyPhysicsData, dynamic_friction_modifier),
            },
            FieldInfoData {
                name: "StaticFrictionModifier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MaterialPropertyPhysicsData, static_friction_modifier),
            },
            FieldInfoData {
                name: "Resistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MaterialPropertyPhysicsData, resistance),
            },
        ],
    }),
    array_type: Some(MATERIALPROPERTYPHYSICSDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MaterialPropertyPhysicsData {
    fn type_info() -> &'static TypeInfo {
        MATERIALPROPERTYPHYSICSDATA_TYPE_INFO
    }
}


pub const MATERIALPROPERTYPHYSICSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialPropertyPhysicsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("MaterialPropertyPhysicsData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProximityData {
    pub proximity_type: ProximityObjectType,
}

pub const PROXIMITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProximityData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ProximityType",
                flags: MemberInfoFlags::new(0),
                field_type: PROXIMITYOBJECTTYPE_TYPE_INFO,
                rust_offset: offset_of!(ProximityData, proximity_type),
            },
        ],
    }),
    array_type: Some(PROXIMITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProximityData {
    fn type_info() -> &'static TypeInfo {
        PROXIMITYDATA_TYPE_INFO
    }
}


pub const PROXIMITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProximityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ProximityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ProximityObjectType {
    #[default]
    PotProximityDisabled = 0,
    PotVaultableLow = 1,
    PotVaultableHigh = 2,
    PotSupportedShooting = 3,
    PotInteractWith = 4,
    PotBashable = 5,
}

pub const PROXIMITYOBJECTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProximityObjectType",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(PROXIMITYOBJECTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ProximityObjectType {
    fn type_info() -> &'static TypeInfo {
        PROXIMITYOBJECTTYPE_TYPE_INFO
    }
}


pub const PROXIMITYOBJECTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProximityObjectType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ProximityObjectType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PhysicsResourceContainerAsset {
    pub physics_resource: super::core::ResourceRef,
}

pub const PHYSICSRESOURCECONTAINERASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsResourceContainerAsset",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "PhysicsResource",
                flags: MemberInfoFlags::new(0),
                field_type: RESOURCEREF_TYPE_INFO,
                rust_offset: offset_of!(PhysicsResourceContainerAsset, physics_resource),
            },
        ],
    }),
    array_type: Some(PHYSICSRESOURCECONTAINERASSET_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PhysicsResourceContainerAsset {
    fn type_info() -> &'static TypeInfo {
        PHYSICSRESOURCECONTAINERASSET_TYPE_INFO
    }
}


pub const PHYSICSRESOURCECONTAINERASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsResourceContainerAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsResourceContainerAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RagdollAsset {
    pub material_pair: super::entity::MaterialDecl,
    pub resource: super::core::ResourceRef,
}

pub const RAGDOLLASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RagdollAsset",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MaterialPair",
                flags: MemberInfoFlags::new(0),
                field_type: MATERIALDECL_TYPE_INFO,
                rust_offset: offset_of!(RagdollAsset, material_pair),
            },
            FieldInfoData {
                name: "Resource",
                flags: MemberInfoFlags::new(0),
                field_type: RESOURCEREF_TYPE_INFO,
                rust_offset: offset_of!(RagdollAsset, resource),
            },
        ],
    }),
    array_type: Some(RAGDOLLASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RagdollAsset {
    fn type_info() -> &'static TypeInfo {
        RAGDOLLASSET_TYPE_INFO
    }
}


pub const RAGDOLLASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RagdollAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("RagdollAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct GroupHavokAsset {
    pub aabb: Vec<AssetAabbs>,
}

pub const GROUPHAVOKASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroupHavokAsset",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(HAVOKASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Aabb",
                flags: MemberInfoFlags::new(144),
                field_type: ASSETAABBS_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(GroupHavokAsset, aabb),
            },
        ],
    }),
    array_type: Some(GROUPHAVOKASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GroupHavokAsset {
    fn type_info() -> &'static TypeInfo {
        GROUPHAVOKASSET_TYPE_INFO
    }
}


pub const GROUPHAVOKASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroupHavokAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("GroupHavokAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AssetAabbs {
    pub part_aabb: Vec<super::core::AxisAlignedBox>,
}

pub const ASSETAABBS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AssetAabbs",
    flags: MemberInfoFlags::new(73),
    module: "Physics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "PartAabb",
                flags: MemberInfoFlags::new(144),
                field_type: AXISALIGNEDBOX_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(AssetAabbs, part_aabb),
            },
        ],
    }),
    array_type: Some(ASSETAABBS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AssetAabbs {
    fn type_info() -> &'static TypeInfo {
        ASSETAABBS_TYPE_INFO
    }
}


pub const ASSETAABBS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AssetAabbs-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("AssetAabbs-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HavokAsset {
    pub external_assets: Vec<super::core::DataContainer>,
    pub resource: super::core::ResourceRef,
}

pub const HAVOKASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HavokAsset",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ExternalAssets",
                flags: MemberInfoFlags::new(144),
                field_type: DATACONTAINER_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(HavokAsset, external_assets),
            },
            FieldInfoData {
                name: "Resource",
                flags: MemberInfoFlags::new(0),
                field_type: RESOURCEREF_TYPE_INFO,
                rust_offset: offset_of!(HavokAsset, resource),
            },
        ],
    }),
    array_type: Some(HAVOKASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for HavokAsset {
    fn type_info() -> &'static TypeInfo {
        HAVOKASSET_TYPE_INFO
    }
}


pub const HAVOKASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HavokAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("HavokAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HeightfieldTestEntityData {
    pub realm: super::core::Realm,
    pub skip_samples: i32,
}

pub const HEIGHTFIELDTESTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HeightfieldTestEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(HeightfieldTestEntityData, realm),
            },
            FieldInfoData {
                name: "SkipSamples",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(HeightfieldTestEntityData, skip_samples),
            },
        ],
    }),
    array_type: Some(HEIGHTFIELDTESTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for HeightfieldTestEntityData {
    fn type_info() -> &'static TypeInfo {
        HEIGHTFIELDTESTENTITYDATA_TYPE_INFO
    }
}


pub const HEIGHTFIELDTESTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HeightfieldTestEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("HeightfieldTestEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CharacterPhysicsData {
    pub poses: Vec<CharacterPoseData>,
    pub states: Vec<CharacterStateData>,
    pub default_state: CharacterStateType,
    pub sprint: CharacterSprintData,
    pub material_pair: super::entity::MaterialDecl,
    pub pushable_object_weight: i32,
    pub mass: f32,
    pub max_ascend_angle: f32,
    pub allow_supported_slide_state: bool,
    pub slide_angle: f32,
    pub slide_speed_condition: f32,
    pub physical_radius: f32,
    pub enter_swim_state_depth: f32,
    pub exit_swim_state_depth: f32,
    pub input_acceleration: f32,
    pub ladder_accept_angle: f32,
    pub ladder_accept_angle_pitch: f32,
    pub jump_penalty_time: f32,
    pub jump_penalty_factor: f32,
    pub radius_to_predict_collision_on_characters: f32,
    pub allow_pose_change_during_transition: bool,
    pub auto_push_away_from_walls_in_prone: bool,
    pub physics_body: CharacterPhysicsBodyData,
    pub check_support_raycast_count_moving: i32,
    pub check_support_lookahead_distance: f32,
    pub ray_start_height_on_ground: f32,
    pub ray_end_height_on_ground: f32,
    pub ray_start_height_in_air: f32,
    pub ray_end_height_in_air: f32,
    pub speed_for_moving_ray_casts: f32,
}

pub const CHARACTERPHYSICSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterPhysicsData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Poses",
                flags: MemberInfoFlags::new(144),
                field_type: CHARACTERPOSEDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(CharacterPhysicsData, poses),
            },
            FieldInfoData {
                name: "States",
                flags: MemberInfoFlags::new(144),
                field_type: CHARACTERSTATEDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(CharacterPhysicsData, states),
            },
            FieldInfoData {
                name: "DefaultState",
                flags: MemberInfoFlags::new(0),
                field_type: CHARACTERSTATETYPE_TYPE_INFO,
                rust_offset: offset_of!(CharacterPhysicsData, default_state),
            },
            FieldInfoData {
                name: "Sprint",
                flags: MemberInfoFlags::new(0),
                field_type: CHARACTERSPRINTDATA_TYPE_INFO,
                rust_offset: offset_of!(CharacterPhysicsData, sprint),
            },
            FieldInfoData {
                name: "MaterialPair",
                flags: MemberInfoFlags::new(0),
                field_type: MATERIALDECL_TYPE_INFO,
                rust_offset: offset_of!(CharacterPhysicsData, material_pair),
            },
            FieldInfoData {
                name: "PushableObjectWeight",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterPhysicsData, pushable_object_weight),
            },
            FieldInfoData {
                name: "Mass",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterPhysicsData, mass),
            },
            FieldInfoData {
                name: "MaxAscendAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterPhysicsData, max_ascend_angle),
            },
            FieldInfoData {
                name: "AllowSupportedSlideState",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CharacterPhysicsData, allow_supported_slide_state),
            },
            FieldInfoData {
                name: "SlideAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterPhysicsData, slide_angle),
            },
            FieldInfoData {
                name: "SlideSpeedCondition",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterPhysicsData, slide_speed_condition),
            },
            FieldInfoData {
                name: "PhysicalRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterPhysicsData, physical_radius),
            },
            FieldInfoData {
                name: "EnterSwimStateDepth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterPhysicsData, enter_swim_state_depth),
            },
            FieldInfoData {
                name: "ExitSwimStateDepth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterPhysicsData, exit_swim_state_depth),
            },
            FieldInfoData {
                name: "InputAcceleration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterPhysicsData, input_acceleration),
            },
            FieldInfoData {
                name: "LadderAcceptAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterPhysicsData, ladder_accept_angle),
            },
            FieldInfoData {
                name: "LadderAcceptAnglePitch",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterPhysicsData, ladder_accept_angle_pitch),
            },
            FieldInfoData {
                name: "JumpPenaltyTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterPhysicsData, jump_penalty_time),
            },
            FieldInfoData {
                name: "JumpPenaltyFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterPhysicsData, jump_penalty_factor),
            },
            FieldInfoData {
                name: "RadiusToPredictCollisionOnCharacters",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterPhysicsData, radius_to_predict_collision_on_characters),
            },
            FieldInfoData {
                name: "AllowPoseChangeDuringTransition",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CharacterPhysicsData, allow_pose_change_during_transition),
            },
            FieldInfoData {
                name: "AutoPushAwayFromWallsInProne",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CharacterPhysicsData, auto_push_away_from_walls_in_prone),
            },
            FieldInfoData {
                name: "PhysicsBody",
                flags: MemberInfoFlags::new(0),
                field_type: CHARACTERPHYSICSBODYDATA_TYPE_INFO,
                rust_offset: offset_of!(CharacterPhysicsData, physics_body),
            },
            FieldInfoData {
                name: "CheckSupportRaycastCountMoving",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterPhysicsData, check_support_raycast_count_moving),
            },
            FieldInfoData {
                name: "CheckSupportLookaheadDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterPhysicsData, check_support_lookahead_distance),
            },
            FieldInfoData {
                name: "RayStartHeightOnGround",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterPhysicsData, ray_start_height_on_ground),
            },
            FieldInfoData {
                name: "RayEndHeightOnGround",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterPhysicsData, ray_end_height_on_ground),
            },
            FieldInfoData {
                name: "RayStartHeightInAir",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterPhysicsData, ray_start_height_in_air),
            },
            FieldInfoData {
                name: "RayEndHeightInAir",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterPhysicsData, ray_end_height_in_air),
            },
            FieldInfoData {
                name: "SpeedForMovingRayCasts",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterPhysicsData, speed_for_moving_ray_casts),
            },
        ],
    }),
    array_type: Some(CHARACTERPHYSICSDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CharacterPhysicsData {
    fn type_info() -> &'static TypeInfo {
        CHARACTERPHYSICSDATA_TYPE_INFO
    }
}


pub const CHARACTERPHYSICSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterPhysicsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("CharacterPhysicsData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CharacterPhysicsBodyData {
    pub character_physics: CharacterPhysicsData,
}

pub const CHARACTERPHYSICSBODYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterPhysicsBodyData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSBODYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "CharacterPhysics",
                flags: MemberInfoFlags::new(0),
                field_type: CHARACTERPHYSICSDATA_TYPE_INFO,
                rust_offset: offset_of!(CharacterPhysicsBodyData, character_physics),
            },
        ],
    }),
    array_type: Some(CHARACTERPHYSICSBODYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CharacterPhysicsBodyData {
    fn type_info() -> &'static TypeInfo {
        CHARACTERPHYSICSBODYDATA_TYPE_INFO
    }
}


pub const CHARACTERPHYSICSBODYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterPhysicsBodyData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("CharacterPhysicsBodyData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ParachuteStateData {
    pub deploy_time: f32,
    pub terminal_velocity: f32,
    pub forward_drag_coefficient: f32,
    pub angle_of_attack: f32,
    pub bank_offset: f32,
    pub throttle_offset: f32,
    pub brake_offset: f32,
    pub max_roll_velocity: f32,
    pub max_pitch_velocity: f32,
    pub max_yaw_velocity: f32,
}

pub const PARACHUTESTATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ParachuteStateData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CHARACTERSTATEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DeployTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ParachuteStateData, deploy_time),
            },
            FieldInfoData {
                name: "TerminalVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ParachuteStateData, terminal_velocity),
            },
            FieldInfoData {
                name: "ForwardDragCoefficient",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ParachuteStateData, forward_drag_coefficient),
            },
            FieldInfoData {
                name: "AngleOfAttack",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ParachuteStateData, angle_of_attack),
            },
            FieldInfoData {
                name: "BankOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ParachuteStateData, bank_offset),
            },
            FieldInfoData {
                name: "ThrottleOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ParachuteStateData, throttle_offset),
            },
            FieldInfoData {
                name: "BrakeOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ParachuteStateData, brake_offset),
            },
            FieldInfoData {
                name: "MaxRollVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ParachuteStateData, max_roll_velocity),
            },
            FieldInfoData {
                name: "MaxPitchVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ParachuteStateData, max_pitch_velocity),
            },
            FieldInfoData {
                name: "MaxYawVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ParachuteStateData, max_yaw_velocity),
            },
        ],
    }),
    array_type: Some(PARACHUTESTATEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ParachuteStateData {
    fn type_info() -> &'static TypeInfo {
        PARACHUTESTATEDATA_TYPE_INFO
    }
}


pub const PARACHUTESTATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ParachuteStateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ParachuteStateData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SwimmingStateData {
    pub body_under_water: f32,
}

pub const SWIMMINGSTATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SwimmingStateData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CHARACTERSTATEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "BodyUnderWater",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SwimmingStateData, body_under_water),
            },
        ],
    }),
    array_type: Some(SWIMMINGSTATEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SwimmingStateData {
    fn type_info() -> &'static TypeInfo {
        SWIMMINGSTATEDATA_TYPE_INFO
    }
}


pub const SWIMMINGSTATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SwimmingStateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("SwimmingStateData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct JumpStateData {
    pub jump_height: f32,
    pub jump_effect_size: f32,
}

pub const JUMPSTATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "JumpStateData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CHARACTERSTATEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "JumpHeight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(JumpStateData, jump_height),
            },
            FieldInfoData {
                name: "JumpEffectSize",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(JumpStateData, jump_effect_size),
            },
        ],
    }),
    array_type: Some(JUMPSTATEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for JumpStateData {
    fn type_info() -> &'static TypeInfo {
        JUMPSTATEDATA_TYPE_INFO
    }
}


pub const JUMPSTATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "JumpStateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("JumpStateData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ClimbingStateData {
    pub lateral_input_scale: f32,
    pub down_angle_limit: f32,
    pub drop_off_angle: f32,
    pub attraction_velocity: f32,
    pub push_away_velocity: f32,
    pub climb_height_offset: f32,
    pub climb_off_vertical_distance: f32,
    pub climb_off_vertical_time: f32,
    pub climb_off_horizontal_distance: f32,
    pub climb_off_horizontal_time: f32,
}

pub const CLIMBINGSTATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClimbingStateData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CHARACTERSTATEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "LateralInputScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ClimbingStateData, lateral_input_scale),
            },
            FieldInfoData {
                name: "DownAngleLimit",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ClimbingStateData, down_angle_limit),
            },
            FieldInfoData {
                name: "DropOffAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ClimbingStateData, drop_off_angle),
            },
            FieldInfoData {
                name: "AttractionVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ClimbingStateData, attraction_velocity),
            },
            FieldInfoData {
                name: "PushAwayVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ClimbingStateData, push_away_velocity),
            },
            FieldInfoData {
                name: "ClimbHeightOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ClimbingStateData, climb_height_offset),
            },
            FieldInfoData {
                name: "ClimbOffVerticalDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ClimbingStateData, climb_off_vertical_distance),
            },
            FieldInfoData {
                name: "ClimbOffVerticalTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ClimbingStateData, climb_off_vertical_time),
            },
            FieldInfoData {
                name: "ClimbOffHorizontalDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ClimbingStateData, climb_off_horizontal_distance),
            },
            FieldInfoData {
                name: "ClimbOffHorizontalTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ClimbingStateData, climb_off_horizontal_time),
            },
        ],
    }),
    array_type: Some(CLIMBINGSTATEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ClimbingStateData {
    fn type_info() -> &'static TypeInfo {
        CLIMBINGSTATEDATA_TYPE_INFO
    }
}


pub const CLIMBINGSTATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClimbingStateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ClimbingStateData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FallingStateData {
}

pub const FALLINGSTATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FallingStateData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CHARACTERSTATEDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FALLINGSTATEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FallingStateData {
    fn type_info() -> &'static TypeInfo {
        FALLINGSTATEDATA_TYPE_INFO
    }
}


pub const FALLINGSTATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FallingStateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("FallingStateData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct InAirStateData {
    pub free_fall_velocity: f32,
}

pub const INAIRSTATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InAirStateData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CHARACTERSTATEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "FreeFallVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(InAirStateData, free_fall_velocity),
            },
        ],
    }),
    array_type: Some(INAIRSTATEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for InAirStateData {
    fn type_info() -> &'static TypeInfo {
        INAIRSTATEDATA_TYPE_INFO
    }
}


pub const INAIRSTATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InAirStateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("InAirStateData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct OnGroundStateData {
    pub jump_delay: f32,
    pub jump_stamina_penalty: f32,
    pub allowed_distance_from_ground: f32,
    pub fall_with_gravity_distance_from_ground: f32,
    pub clamp_surface_normal_angle: f32,
    pub ground_hugging: bool,
    pub limit_downward_velocity: bool,
    pub uphill_speed_modifier: f32,
    pub uphill_speed_modifier_max_angle: f32,
    pub downhill_speed_modifier: f32,
    pub downhill_speed_modifier_max_angle: f32,
    pub hill_speed_modifier_dead_zone: f32,
    pub character_spring_scale_moving_under_terrain: f32,
    pub character_spring_scale_moving_over_terrain: f32,
    pub character_spring_scale_still: f32,
}

pub const ONGROUNDSTATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnGroundStateData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CHARACTERSTATEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "JumpDelay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OnGroundStateData, jump_delay),
            },
            FieldInfoData {
                name: "JumpStaminaPenalty",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OnGroundStateData, jump_stamina_penalty),
            },
            FieldInfoData {
                name: "AllowedDistanceFromGround",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OnGroundStateData, allowed_distance_from_ground),
            },
            FieldInfoData {
                name: "FallWithGravityDistanceFromGround",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OnGroundStateData, fall_with_gravity_distance_from_ground),
            },
            FieldInfoData {
                name: "ClampSurfaceNormalAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OnGroundStateData, clamp_surface_normal_angle),
            },
            FieldInfoData {
                name: "GroundHugging",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OnGroundStateData, ground_hugging),
            },
            FieldInfoData {
                name: "LimitDownwardVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OnGroundStateData, limit_downward_velocity),
            },
            FieldInfoData {
                name: "UphillSpeedModifier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OnGroundStateData, uphill_speed_modifier),
            },
            FieldInfoData {
                name: "UphillSpeedModifierMaxAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OnGroundStateData, uphill_speed_modifier_max_angle),
            },
            FieldInfoData {
                name: "DownhillSpeedModifier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OnGroundStateData, downhill_speed_modifier),
            },
            FieldInfoData {
                name: "DownhillSpeedModifierMaxAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OnGroundStateData, downhill_speed_modifier_max_angle),
            },
            FieldInfoData {
                name: "HillSpeedModifierDeadZone",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OnGroundStateData, hill_speed_modifier_dead_zone),
            },
            FieldInfoData {
                name: "CharacterSpringScaleMovingUnderTerrain",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OnGroundStateData, character_spring_scale_moving_under_terrain),
            },
            FieldInfoData {
                name: "CharacterSpringScaleMovingOverTerrain",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OnGroundStateData, character_spring_scale_moving_over_terrain),
            },
            FieldInfoData {
                name: "CharacterSpringScaleStill",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OnGroundStateData, character_spring_scale_still),
            },
        ],
    }),
    array_type: Some(ONGROUNDSTATEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OnGroundStateData {
    fn type_info() -> &'static TypeInfo {
        ONGROUNDSTATEDATA_TYPE_INFO
    }
}


pub const ONGROUNDSTATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnGroundStateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("OnGroundStateData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SlidingStateData {
    pub horizontal_input_scale: f32,
    pub gravity_scale: f32,
    pub character_spring_scale: f32,
    pub allowed_distance_from_ground: f32,
}

pub const SLIDINGSTATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SlidingStateData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CHARACTERSTATEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "HorizontalInputScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SlidingStateData, horizontal_input_scale),
            },
            FieldInfoData {
                name: "GravityScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SlidingStateData, gravity_scale),
            },
            FieldInfoData {
                name: "CharacterSpringScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SlidingStateData, character_spring_scale),
            },
            FieldInfoData {
                name: "AllowedDistanceFromGround",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SlidingStateData, allowed_distance_from_ground),
            },
        ],
    }),
    array_type: Some(SLIDINGSTATEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SlidingStateData {
    fn type_info() -> &'static TypeInfo {
        SLIDINGSTATEDATA_TYPE_INFO
    }
}


pub const SLIDINGSTATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SlidingStateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("SlidingStateData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AnimationControlledStateData {
    pub up_normal_tolerance: f32,
    pub character_spring_strength: f32,
}

pub const ANIMATIONCONTROLLEDSTATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimationControlledStateData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CHARACTERSTATEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "UpNormalTolerance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AnimationControlledStateData, up_normal_tolerance),
            },
            FieldInfoData {
                name: "CharacterSpringStrength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AnimationControlledStateData, character_spring_strength),
            },
        ],
    }),
    array_type: Some(ANIMATIONCONTROLLEDSTATEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AnimationControlledStateData {
    fn type_info() -> &'static TypeInfo {
        ANIMATIONCONTROLLEDSTATEDATA_TYPE_INFO
    }
}


pub const ANIMATIONCONTROLLEDSTATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimationControlledStateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("AnimationControlledStateData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CharacterStateData {
    pub pose_info: Vec<CharacterStatePoseInfo>,
}

pub const CHARACTERSTATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterStateData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "PoseInfo",
                flags: MemberInfoFlags::new(144),
                field_type: CHARACTERSTATEPOSEINFO_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(CharacterStateData, pose_info),
            },
        ],
    }),
    array_type: Some(CHARACTERSTATEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CharacterStateData {
    fn type_info() -> &'static TypeInfo {
        CHARACTERSTATEDATA_TYPE_INFO
    }
}


pub const CHARACTERSTATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterStateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("CharacterStateData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum CharacterStateType {
    #[default]
    CharacterStateType_OnGround = 0,
    CharacterStateType_Jumping = 1,
    CharacterStateType_InAir = 2,
    CharacterStateType_Climbing = 3,
    CharacterStateType_Falling = 4,
    CharacterStateType_User_0 = 5,
    CharacterStateType_User_1 = 6,
    CharacterStateType_User_2 = 7,
    CharacterStateType_User_3 = 8,
    CharacterStateType_User_4 = 9,
    CharacterStateType_User_5 = 10,
    CharacterStateType_StateCount = 11,
}

pub const CHARACTERSTATETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterStateType",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(CHARACTERSTATETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CharacterStateType {
    fn type_info() -> &'static TypeInfo {
        CHARACTERSTATETYPE_TYPE_INFO
    }
}


pub const CHARACTERSTATETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterStateType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("CharacterStateType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CharacterSprintData {
    pub sprint_power_decrease_per_second: f32,
    pub sprint_power_increase_per_second: f32,
    pub sprint_minimum_power: f32,
    pub allow_continous_sprinting: bool,
}

pub const CHARACTERSPRINTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterSprintData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "SprintPowerDecreasePerSecond",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterSprintData, sprint_power_decrease_per_second),
            },
            FieldInfoData {
                name: "SprintPowerIncreasePerSecond",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterSprintData, sprint_power_increase_per_second),
            },
            FieldInfoData {
                name: "SprintMinimumPower",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterSprintData, sprint_minimum_power),
            },
            FieldInfoData {
                name: "AllowContinousSprinting",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CharacterSprintData, allow_continous_sprinting),
            },
        ],
    }),
    array_type: Some(CHARACTERSPRINTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CharacterSprintData {
    fn type_info() -> &'static TypeInfo {
        CHARACTERSPRINTDATA_TYPE_INFO
    }
}


pub const CHARACTERSPRINTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterSprintData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("CharacterSprintData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CharacterStatePoseInfo {
    pub pose_type: CharacterPoseType,
    pub velocity: f32,
    pub acceleration_gain: f32,
    pub deceleration_gain: f32,
    pub direction_change_acceleration_gain: f32,
    pub direction_change_threshold: f32,
    pub sprint_gain: f32,
    pub sprint_multiplier: f32,
    pub speed_modifier: SpeedModifierData,
    pub shallow_water_multiplier: f32,
}

pub const CHARACTERSTATEPOSEINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterStatePoseInfo",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "PoseType",
                flags: MemberInfoFlags::new(0),
                field_type: CHARACTERPOSETYPE_TYPE_INFO,
                rust_offset: offset_of!(CharacterStatePoseInfo, pose_type),
            },
            FieldInfoData {
                name: "Velocity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterStatePoseInfo, velocity),
            },
            FieldInfoData {
                name: "AccelerationGain",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterStatePoseInfo, acceleration_gain),
            },
            FieldInfoData {
                name: "DecelerationGain",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterStatePoseInfo, deceleration_gain),
            },
            FieldInfoData {
                name: "DirectionChangeAccelerationGain",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterStatePoseInfo, direction_change_acceleration_gain),
            },
            FieldInfoData {
                name: "DirectionChangeThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterStatePoseInfo, direction_change_threshold),
            },
            FieldInfoData {
                name: "SprintGain",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterStatePoseInfo, sprint_gain),
            },
            FieldInfoData {
                name: "SprintMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterStatePoseInfo, sprint_multiplier),
            },
            FieldInfoData {
                name: "SpeedModifier",
                flags: MemberInfoFlags::new(0),
                field_type: SPEEDMODIFIERDATA_TYPE_INFO,
                rust_offset: offset_of!(CharacterStatePoseInfo, speed_modifier),
            },
            FieldInfoData {
                name: "ShallowWaterMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterStatePoseInfo, shallow_water_multiplier),
            },
        ],
    }),
    array_type: Some(CHARACTERSTATEPOSEINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CharacterStatePoseInfo {
    fn type_info() -> &'static TypeInfo {
        CHARACTERSTATEPOSEINFO_TYPE_INFO
    }
}


pub const CHARACTERSTATEPOSEINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterStatePoseInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("CharacterStatePoseInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SpeedModifierData {
    pub forward_constant: f32,
    pub backward_constant: f32,
    pub left_constant: f32,
    pub right_constant: f32,
}

pub const SPEEDMODIFIERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpeedModifierData",
    flags: MemberInfoFlags::new(36937),
    module: "Physics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ForwardConstant",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpeedModifierData, forward_constant),
            },
            FieldInfoData {
                name: "BackwardConstant",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpeedModifierData, backward_constant),
            },
            FieldInfoData {
                name: "LeftConstant",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpeedModifierData, left_constant),
            },
            FieldInfoData {
                name: "RightConstant",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpeedModifierData, right_constant),
            },
        ],
    }),
    array_type: Some(SPEEDMODIFIERDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for SpeedModifierData {
    fn type_info() -> &'static TypeInfo {
        SPEEDMODIFIERDATA_TYPE_INFO
    }
}


pub const SPEEDMODIFIERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpeedModifierData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("SpeedModifierData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CharacterPoseData {
    pub eye_position: super::core::Vec3,
    pub throttle_modifier_curve: Vec<super::core::Vec2>,
    pub step_height: f32,
    pub height: f32,
    pub transition_times: Vec<PoseTransitionTime>,
    pub look_constraints: LookConstraintsData,
    pub pose_type: CharacterPoseType,
    pub collision_type: CharacterPoseCollisionType,
    pub collision_box_min_expand: super::core::Vec3,
    pub collision_box_max_expand: super::core::Vec3,
    pub raycast_radius_padding: f32,
    pub raycast_height_padding: f32,
    pub raycast_offset: super::core::Vec3,
}

pub const CHARACTERPOSEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterPoseData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "EyePosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(CharacterPoseData, eye_position),
            },
            FieldInfoData {
                name: "ThrottleModifierCurve",
                flags: MemberInfoFlags::new(144),
                field_type: VEC2_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(CharacterPoseData, throttle_modifier_curve),
            },
            FieldInfoData {
                name: "StepHeight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterPoseData, step_height),
            },
            FieldInfoData {
                name: "Height",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterPoseData, height),
            },
            FieldInfoData {
                name: "TransitionTimes",
                flags: MemberInfoFlags::new(144),
                field_type: POSETRANSITIONTIME_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(CharacterPoseData, transition_times),
            },
            FieldInfoData {
                name: "LookConstraints",
                flags: MemberInfoFlags::new(0),
                field_type: LOOKCONSTRAINTSDATA_TYPE_INFO,
                rust_offset: offset_of!(CharacterPoseData, look_constraints),
            },
            FieldInfoData {
                name: "PoseType",
                flags: MemberInfoFlags::new(0),
                field_type: CHARACTERPOSETYPE_TYPE_INFO,
                rust_offset: offset_of!(CharacterPoseData, pose_type),
            },
            FieldInfoData {
                name: "CollisionType",
                flags: MemberInfoFlags::new(0),
                field_type: CHARACTERPOSECOLLISIONTYPE_TYPE_INFO,
                rust_offset: offset_of!(CharacterPoseData, collision_type),
            },
            FieldInfoData {
                name: "CollisionBoxMinExpand",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(CharacterPoseData, collision_box_min_expand),
            },
            FieldInfoData {
                name: "CollisionBoxMaxExpand",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(CharacterPoseData, collision_box_max_expand),
            },
            FieldInfoData {
                name: "RaycastRadiusPadding",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterPoseData, raycast_radius_padding),
            },
            FieldInfoData {
                name: "RaycastHeightPadding",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterPoseData, raycast_height_padding),
            },
            FieldInfoData {
                name: "RaycastOffset",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(CharacterPoseData, raycast_offset),
            },
        ],
    }),
    array_type: Some(CHARACTERPOSEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CharacterPoseData {
    fn type_info() -> &'static TypeInfo {
        CHARACTERPOSEDATA_TYPE_INFO
    }
}


pub const CHARACTERPOSEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterPoseData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("CharacterPoseData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PoseTransitionTime {
    pub to_pose: CharacterPoseType,
    pub transition_time: f32,
}

pub const POSETRANSITIONTIME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PoseTransitionTime",
    flags: MemberInfoFlags::new(36937),
    module: "Physics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ToPose",
                flags: MemberInfoFlags::new(0),
                field_type: CHARACTERPOSETYPE_TYPE_INFO,
                rust_offset: offset_of!(PoseTransitionTime, to_pose),
            },
            FieldInfoData {
                name: "TransitionTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PoseTransitionTime, transition_time),
            },
        ],
    }),
    array_type: Some(POSETRANSITIONTIME_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for PoseTransitionTime {
    fn type_info() -> &'static TypeInfo {
        POSETRANSITIONTIME_TYPE_INFO
    }
}


pub const POSETRANSITIONTIME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PoseTransitionTime-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PoseTransitionTime-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum CharacterPoseCollisionType {
    #[default]
    CharacterPoseCollisionType_Capsule = 0,
    CharacterPoseCollisionType_Pencil = 1,
    CharacterPoseCollisionType_HorizontalCapsule = 2,
    CharacterPoseCollisionTypeCount = 3,
}

pub const CHARACTERPOSECOLLISIONTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterPoseCollisionType",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(CHARACTERPOSECOLLISIONTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CharacterPoseCollisionType {
    fn type_info() -> &'static TypeInfo {
        CHARACTERPOSECOLLISIONTYPE_TYPE_INFO
    }
}


pub const CHARACTERPOSECOLLISIONTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterPoseCollisionType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("CharacterPoseCollisionType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum CharacterPoseType {
    #[default]
    CharacterPoseType_Stand = 0,
    CharacterPoseType_Crouch = 1,
    CharacterPoseType_Prone = 2,
    CharacterPoseTypeCount = 3,
}

pub const CHARACTERPOSETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterPoseType",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(CHARACTERPOSETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CharacterPoseType {
    fn type_info() -> &'static TypeInfo {
        CHARACTERPOSETYPE_TYPE_INFO
    }
}


pub const CHARACTERPOSETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterPoseType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("CharacterPoseType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CharacterPoseConstraintsData {
    pub stand_pose: bool,
    pub crouch_pose: bool,
    pub prone_pose: bool,
}

pub const CHARACTERPOSECONSTRAINTSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterPoseConstraintsData",
    flags: MemberInfoFlags::new(36937),
    module: "Physics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "StandPose",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CharacterPoseConstraintsData, stand_pose),
            },
            FieldInfoData {
                name: "CrouchPose",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CharacterPoseConstraintsData, crouch_pose),
            },
            FieldInfoData {
                name: "PronePose",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CharacterPoseConstraintsData, prone_pose),
            },
        ],
    }),
    array_type: Some(CHARACTERPOSECONSTRAINTSDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CharacterPoseConstraintsData {
    fn type_info() -> &'static TypeInfo {
        CHARACTERPOSECONSTRAINTSDATA_TYPE_INFO
    }
}


pub const CHARACTERPOSECONSTRAINTSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterPoseConstraintsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("CharacterPoseConstraintsData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LookConstraintsData {
    pub min_look_yaw: f32,
    pub max_look_yaw: f32,
    pub min_look_pitch: f32,
    pub max_look_pitch: f32,
}

pub const LOOKCONSTRAINTSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LookConstraintsData",
    flags: MemberInfoFlags::new(36937),
    module: "Physics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "MinLookYaw",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LookConstraintsData, min_look_yaw),
            },
            FieldInfoData {
                name: "MaxLookYaw",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LookConstraintsData, max_look_yaw),
            },
            FieldInfoData {
                name: "MinLookPitch",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LookConstraintsData, min_look_pitch),
            },
            FieldInfoData {
                name: "MaxLookPitch",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LookConstraintsData, max_look_pitch),
            },
        ],
    }),
    array_type: Some(LOOKCONSTRAINTSDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for LookConstraintsData {
    fn type_info() -> &'static TypeInfo {
        LOOKCONSTRAINTSDATA_TYPE_INFO
    }
}


pub const LOOKCONSTRAINTSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LookConstraintsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("LookConstraintsData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct VehicleSpawnStateEntityData {
    pub initial_velocity: super::core::Vec3,
    pub startup_delay_modifier: f32,
    pub initial_throttle: f32,
    pub engine_index: u32,
    pub initial_rpm_modifier: f32,
    pub height_offset: f32,
    pub radius: f32,
}

pub const VEHICLESPAWNSTATEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleSpawnStateEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "InitialVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(VehicleSpawnStateEntityData, initial_velocity),
            },
            FieldInfoData {
                name: "StartupDelayModifier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleSpawnStateEntityData, startup_delay_modifier),
            },
            FieldInfoData {
                name: "InitialThrottle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleSpawnStateEntityData, initial_throttle),
            },
            FieldInfoData {
                name: "EngineIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleSpawnStateEntityData, engine_index),
            },
            FieldInfoData {
                name: "InitialRpmModifier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleSpawnStateEntityData, initial_rpm_modifier),
            },
            FieldInfoData {
                name: "HeightOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleSpawnStateEntityData, height_offset),
            },
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleSpawnStateEntityData, radius),
            },
        ],
    }),
    array_type: Some(VEHICLESPAWNSTATEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for VehicleSpawnStateEntityData {
    fn type_info() -> &'static TypeInfo {
        VEHICLESPAWNSTATEENTITYDATA_TYPE_INFO
    }
}


pub const VEHICLESPAWNSTATEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleSpawnStateEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("VehicleSpawnStateEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LinearMovingBodyData {
    pub axis: super::core::Vec3,
    pub start: EndPointData,
    pub end: EndPointData,
}

pub const LINEARMOVINGBODYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMovingBodyData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MOVINGBODYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Axis",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(LinearMovingBodyData, axis),
            },
            FieldInfoData {
                name: "Start",
                flags: MemberInfoFlags::new(0),
                field_type: ENDPOINTDATA_TYPE_INFO,
                rust_offset: offset_of!(LinearMovingBodyData, start),
            },
            FieldInfoData {
                name: "End",
                flags: MemberInfoFlags::new(0),
                field_type: ENDPOINTDATA_TYPE_INFO,
                rust_offset: offset_of!(LinearMovingBodyData, end),
            },
        ],
    }),
    array_type: Some(LINEARMOVINGBODYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LinearMovingBodyData {
    fn type_info() -> &'static TypeInfo {
        LINEARMOVINGBODYDATA_TYPE_INFO
    }
}


pub const LINEARMOVINGBODYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMovingBodyData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("LinearMovingBodyData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EndPointData {
    pub pos: f32,
    pub end_damping: f32,
    pub spring_length: f32,
    pub spring_acceleration: f32,
    pub spring_damping: f32,
}

pub const ENDPOINTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EndPointData",
    flags: MemberInfoFlags::new(36937),
    module: "Physics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Pos",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EndPointData, pos),
            },
            FieldInfoData {
                name: "EndDamping",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EndPointData, end_damping),
            },
            FieldInfoData {
                name: "SpringLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EndPointData, spring_length),
            },
            FieldInfoData {
                name: "SpringAcceleration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EndPointData, spring_acceleration),
            },
            FieldInfoData {
                name: "SpringDamping",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EndPointData, spring_damping),
            },
        ],
    }),
    array_type: Some(ENDPOINTDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for EndPointData {
    fn type_info() -> &'static TypeInfo {
        ENDPOINTDATA_TYPE_INFO
    }
}


pub const ENDPOINTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EndPointData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("EndPointData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RotationBodyData {
    pub angular_momentum_multiplier: f32,
    pub angular_momentum_threshold: f32,
    pub angular_momentum_damping: f32,
    pub angular_momentum_damping_acc: f32,
    pub angular_momentum_damping_deacc: f32,
    pub use_post_satisfy_constraints: bool,
    pub angular_constraint_min: f32,
    pub angular_constraint_max: f32,
    pub rotation_axis: i32,
    pub extended_constraints: ExtendedConstraintsData,
    pub use_angular_momentum_threshold: bool,
    pub use_angular_momentum_damping: bool,
    pub use_angular_constraint: bool,
}

pub const ROTATIONBODYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RotationBodyData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MOVINGBODYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "AngularMomentumMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RotationBodyData, angular_momentum_multiplier),
            },
            FieldInfoData {
                name: "AngularMomentumThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RotationBodyData, angular_momentum_threshold),
            },
            FieldInfoData {
                name: "AngularMomentumDamping",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RotationBodyData, angular_momentum_damping),
            },
            FieldInfoData {
                name: "AngularMomentumDampingAcc",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RotationBodyData, angular_momentum_damping_acc),
            },
            FieldInfoData {
                name: "AngularMomentumDampingDeacc",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RotationBodyData, angular_momentum_damping_deacc),
            },
            FieldInfoData {
                name: "UsePostSatisfyConstraints",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RotationBodyData, use_post_satisfy_constraints),
            },
            FieldInfoData {
                name: "AngularConstraintMin",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RotationBodyData, angular_constraint_min),
            },
            FieldInfoData {
                name: "AngularConstraintMax",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RotationBodyData, angular_constraint_max),
            },
            FieldInfoData {
                name: "RotationAxis",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(RotationBodyData, rotation_axis),
            },
            FieldInfoData {
                name: "ExtendedConstraints",
                flags: MemberInfoFlags::new(0),
                field_type: EXTENDEDCONSTRAINTSDATA_TYPE_INFO,
                rust_offset: offset_of!(RotationBodyData, extended_constraints),
            },
            FieldInfoData {
                name: "UseAngularMomentumThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RotationBodyData, use_angular_momentum_threshold),
            },
            FieldInfoData {
                name: "UseAngularMomentumDamping",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RotationBodyData, use_angular_momentum_damping),
            },
            FieldInfoData {
                name: "UseAngularConstraint",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RotationBodyData, use_angular_constraint),
            },
        ],
    }),
    array_type: Some(ROTATIONBODYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RotationBodyData {
    fn type_info() -> &'static TypeInfo {
        ROTATIONBODYDATA_TYPE_INFO
    }
}


pub const ROTATIONBODYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RotationBodyData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("RotationBodyData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ExtendedConstraintsData {
    pub heading: f32,
    pub width: f32,
    pub falloff: f32,
    pub angular_constraint_min: f32,
    pub angular_constraint_max: f32,
}

pub const EXTENDEDCONSTRAINTSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExtendedConstraintsData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Heading",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ExtendedConstraintsData, heading),
            },
            FieldInfoData {
                name: "Width",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ExtendedConstraintsData, width),
            },
            FieldInfoData {
                name: "Falloff",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ExtendedConstraintsData, falloff),
            },
            FieldInfoData {
                name: "AngularConstraintMin",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ExtendedConstraintsData, angular_constraint_min),
            },
            FieldInfoData {
                name: "AngularConstraintMax",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ExtendedConstraintsData, angular_constraint_max),
            },
        ],
    }),
    array_type: Some(EXTENDEDCONSTRAINTSDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ExtendedConstraintsData {
    fn type_info() -> &'static TypeInfo {
        EXTENDEDCONSTRAINTSDATA_TYPE_INFO
    }
}


pub const EXTENDEDCONSTRAINTSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExtendedConstraintsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ExtendedConstraintsData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MovingBodyData {
    pub mass: f32,
    pub inertia: f32,
}

pub const MOVINGBODYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovingBodyData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Mass",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MovingBodyData, mass),
            },
            FieldInfoData {
                name: "Inertia",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MovingBodyData, inertia),
            },
        ],
    }),
    array_type: Some(MOVINGBODYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MovingBodyData {
    fn type_info() -> &'static TypeInfo {
        MOVINGBODYDATA_TYPE_INFO
    }
}


pub const MOVINGBODYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovingBodyData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("MovingBodyData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CameraLoosePartPhysicsData {
    pub scale: f32,
}

pub const CAMERALOOSEPARTPHYSICSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraLoosePartPhysicsData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOOSEPARTPHYSICSDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Scale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CameraLoosePartPhysicsData, scale),
            },
        ],
    }),
    array_type: Some(CAMERALOOSEPARTPHYSICSDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CameraLoosePartPhysicsData {
    fn type_info() -> &'static TypeInfo {
        CAMERALOOSEPARTPHYSICSDATA_TYPE_INFO
    }
}


pub const CAMERALOOSEPARTPHYSICSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraLoosePartPhysicsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("CameraLoosePartPhysicsData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct NetworkableLoosePartPhysicsData {
    pub networked: bool,
}

pub const NETWORKABLELOOSEPARTPHYSICSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkableLoosePartPhysicsData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOOSEPARTPHYSICSDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Networked",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(NetworkableLoosePartPhysicsData, networked),
            },
        ],
    }),
    array_type: Some(NETWORKABLELOOSEPARTPHYSICSDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NetworkableLoosePartPhysicsData {
    fn type_info() -> &'static TypeInfo {
        NETWORKABLELOOSEPARTPHYSICSDATA_TYPE_INFO
    }
}


pub const NETWORKABLELOOSEPARTPHYSICSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkableLoosePartPhysicsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("NetworkableLoosePartPhysicsData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LoosePartPhysicsData {
    pub fake_physics: FakePhysicsData,
}

pub const LOOSEPARTPHYSICSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LoosePartPhysicsData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "FakePhysics",
                flags: MemberInfoFlags::new(0),
                field_type: FAKEPHYSICSDATA_TYPE_INFO,
                rust_offset: offset_of!(LoosePartPhysicsData, fake_physics),
            },
        ],
    }),
    array_type: Some(LOOSEPARTPHYSICSDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LoosePartPhysicsData {
    fn type_info() -> &'static TypeInfo {
        LOOSEPARTPHYSICSDATA_TYPE_INFO
    }
}


pub const LOOSEPARTPHYSICSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LoosePartPhysicsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("LoosePartPhysicsData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WheelConfigMotorbikeData {
    pub max_spring_force: f32,
    pub collision_yaw_dampening_duration: f32,
    pub collision_yaw_dampening: f32,
}

pub const WHEELCONFIGMOTORBIKEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WheelConfigMotorbikeData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WHEELCONFIGDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MaxSpringForce",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigMotorbikeData, max_spring_force),
            },
            FieldInfoData {
                name: "CollisionYawDampeningDuration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigMotorbikeData, collision_yaw_dampening_duration),
            },
            FieldInfoData {
                name: "CollisionYawDampening",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigMotorbikeData, collision_yaw_dampening),
            },
        ],
    }),
    array_type: Some(WHEELCONFIGMOTORBIKEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WheelConfigMotorbikeData {
    fn type_info() -> &'static TypeInfo {
        WHEELCONFIGMOTORBIKEDATA_TYPE_INFO
    }
}


pub const WHEELCONFIGMOTORBIKEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WheelConfigMotorbikeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("WheelConfigMotorbikeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WheelConfigData {
    pub offset: super::core::Vec3,
    pub inertia: super::core::Vec3,
    pub rotation_body: RotationBodyData,
    pub sphere_collision: SphereCollisionData,
    pub spring: SpringData,
    pub mass: f32,
    pub radius: f32,
    pub width: f32,
    pub rolling_resistance_internal_base_factor: f32,
    pub rolling_resistance_base_factor: f32,
    pub rolling_resistance_velocity_factor: f32,
    pub engine_brake_velocity_factor: f32,
    pub engine_brake_min_factor: f32,
    pub engine_brake_max_factor: f32,
    pub transmission_loss: Vec<super::core::Vec2>,
    pub diff_gear_ratio: f32,
    pub low_speed_steering_sensitivity: f32,
    pub mid_speed_steering_sensitivity: f32,
    pub high_speed_steering_sensitivity: f32,
    pub sensitivity_range_scale: f32,
    pub low_speed_steering_sensitivity_limit: f32,
    pub mid_speed_steering_sensitivity_limit: f32,
    pub high_speed_steering_sensitivity_limit: f32,
    pub tracked_turn_speed_limit: f32,
    pub tracked_forward_speed_limit: f32,
    pub tracked_slip_steer_reduction_scale: f32,
    pub steer_inertia: Curve2D,
    pub steering_sensitivity: Vec<SensitivityAtVelocity>,
    pub tracked_steering_boost_on_opposite_torque: f32,
    pub slope_grip_min_angle: f32,
    pub slope_grip_max_angle: f32,
    pub slope_grip_exponent: i32,
    pub off_ground_gravity_modifier: f32,
    pub side_slip_angle_max_slip_condition: f32,
    pub angular_velocity_min_slip_condition: f32,
    pub wheel_velocity_x_min_slip_condition: f32,
    pub wheel_slip_ratio_max_slip_condition: f32,
    pub longitudinal_friction_force_max_factor: f32,
    pub lateral_friction_force_max_factor: f32,
    pub allow_grip_slip_transition: bool,
    pub longitude_friction_scale: Vec<FrictionScaleAtVelocity>,
    pub lattitude_friction_scale: Vec<FrictionScaleAtVelocity>,
    pub wheel_friction_lattitude_brake_scale: f32,
    pub resistance: f32,
    pub fx_torque_radius_multiplier: f32,
    pub friction_moment_velocity_min: f32,
    pub friction_moment_velocity_max: f32,
    pub friction_moment_multiplier: f32,
    pub friction_moment_max_factor: f32,
    pub brake_factor: f32,
    pub brake_force: f32,
    pub auto_hand_brake_if_no_throttle_and_steer: bool,
    pub hand_brake_factor: f32,
    pub hand_brake_force: f32,
    pub total_friction_scale: f32,
    pub total_lateral_friction_scale: f32,
    pub lateral_pos_k: f32,
    pub lateral_neg_k: f32,
    pub longitudinal_pos_k: f32,
    pub longitudinal_neg_k: f32,
    pub align_mom_scale: f32,
    pub wheel_base_lateral: f32,
    pub wheel_base_longitudinal: f32,
    pub driving_type: i32,
    pub steering_type: i32,
    pub friction_method: i32,
    pub rotation_direction_index: i32,
    pub steering_angle_index: i32,
    pub pacejka_config_index: i32,
    pub engine_index: i32,
    pub ackerman_device_type: i32,
    pub use_rolling_resistance_velocity_factor: bool,
    pub use_rolling_resistance_base_factor: bool,
    pub use_engine_brake: bool,
    pub is_allowed_to_spin: bool,
    pub has_steering_inverted: bool,
    pub use_friction_moment: bool,
    pub use_low_speed_auto_brake: bool,
    pub adjust_wheel_rotation: bool,
    pub collision_material_pair: super::entity::MaterialDecl,
}

pub const WHEELCONFIGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WheelConfigData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Offset",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, offset),
            },
            FieldInfoData {
                name: "Inertia",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, inertia),
            },
            FieldInfoData {
                name: "RotationBody",
                flags: MemberInfoFlags::new(0),
                field_type: ROTATIONBODYDATA_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, rotation_body),
            },
            FieldInfoData {
                name: "SphereCollision",
                flags: MemberInfoFlags::new(0),
                field_type: SPHERECOLLISIONDATA_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, sphere_collision),
            },
            FieldInfoData {
                name: "Spring",
                flags: MemberInfoFlags::new(0),
                field_type: SPRINGDATA_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, spring),
            },
            FieldInfoData {
                name: "Mass",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, mass),
            },
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, radius),
            },
            FieldInfoData {
                name: "Width",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, width),
            },
            FieldInfoData {
                name: "RollingResistanceInternalBaseFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, rolling_resistance_internal_base_factor),
            },
            FieldInfoData {
                name: "RollingResistanceBaseFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, rolling_resistance_base_factor),
            },
            FieldInfoData {
                name: "RollingResistanceVelocityFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, rolling_resistance_velocity_factor),
            },
            FieldInfoData {
                name: "EngineBrakeVelocityFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, engine_brake_velocity_factor),
            },
            FieldInfoData {
                name: "EngineBrakeMinFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, engine_brake_min_factor),
            },
            FieldInfoData {
                name: "EngineBrakeMaxFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, engine_brake_max_factor),
            },
            FieldInfoData {
                name: "TransmissionLoss",
                flags: MemberInfoFlags::new(144),
                field_type: VEC2_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, transmission_loss),
            },
            FieldInfoData {
                name: "DiffGearRatio",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, diff_gear_ratio),
            },
            FieldInfoData {
                name: "LowSpeedSteeringSensitivity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, low_speed_steering_sensitivity),
            },
            FieldInfoData {
                name: "MidSpeedSteeringSensitivity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, mid_speed_steering_sensitivity),
            },
            FieldInfoData {
                name: "HighSpeedSteeringSensitivity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, high_speed_steering_sensitivity),
            },
            FieldInfoData {
                name: "SensitivityRangeScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, sensitivity_range_scale),
            },
            FieldInfoData {
                name: "LowSpeedSteeringSensitivityLimit",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, low_speed_steering_sensitivity_limit),
            },
            FieldInfoData {
                name: "MidSpeedSteeringSensitivityLimit",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, mid_speed_steering_sensitivity_limit),
            },
            FieldInfoData {
                name: "HighSpeedSteeringSensitivityLimit",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, high_speed_steering_sensitivity_limit),
            },
            FieldInfoData {
                name: "TrackedTurnSpeedLimit",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, tracked_turn_speed_limit),
            },
            FieldInfoData {
                name: "TrackedForwardSpeedLimit",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, tracked_forward_speed_limit),
            },
            FieldInfoData {
                name: "TrackedSlipSteerReductionScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, tracked_slip_steer_reduction_scale),
            },
            FieldInfoData {
                name: "SteerInertia",
                flags: MemberInfoFlags::new(0),
                field_type: CURVE2D_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, steer_inertia),
            },
            FieldInfoData {
                name: "SteeringSensitivity",
                flags: MemberInfoFlags::new(144),
                field_type: SENSITIVITYATVELOCITY_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, steering_sensitivity),
            },
            FieldInfoData {
                name: "TrackedSteeringBoostOnOppositeTorque",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, tracked_steering_boost_on_opposite_torque),
            },
            FieldInfoData {
                name: "SlopeGripMinAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, slope_grip_min_angle),
            },
            FieldInfoData {
                name: "SlopeGripMaxAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, slope_grip_max_angle),
            },
            FieldInfoData {
                name: "SlopeGripExponent",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, slope_grip_exponent),
            },
            FieldInfoData {
                name: "OffGroundGravityModifier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, off_ground_gravity_modifier),
            },
            FieldInfoData {
                name: "SideSlipAngleMaxSlipCondition",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, side_slip_angle_max_slip_condition),
            },
            FieldInfoData {
                name: "AngularVelocityMinSlipCondition",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, angular_velocity_min_slip_condition),
            },
            FieldInfoData {
                name: "WheelVelocityXMinSlipCondition",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, wheel_velocity_x_min_slip_condition),
            },
            FieldInfoData {
                name: "WheelSlipRatioMaxSlipCondition",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, wheel_slip_ratio_max_slip_condition),
            },
            FieldInfoData {
                name: "LongitudinalFrictionForceMaxFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, longitudinal_friction_force_max_factor),
            },
            FieldInfoData {
                name: "LateralFrictionForceMaxFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, lateral_friction_force_max_factor),
            },
            FieldInfoData {
                name: "AllowGripSlipTransition",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, allow_grip_slip_transition),
            },
            FieldInfoData {
                name: "LongitudeFrictionScale",
                flags: MemberInfoFlags::new(144),
                field_type: FRICTIONSCALEATVELOCITY_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, longitude_friction_scale),
            },
            FieldInfoData {
                name: "LattitudeFrictionScale",
                flags: MemberInfoFlags::new(144),
                field_type: FRICTIONSCALEATVELOCITY_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, lattitude_friction_scale),
            },
            FieldInfoData {
                name: "WheelFrictionLattitudeBrakeScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, wheel_friction_lattitude_brake_scale),
            },
            FieldInfoData {
                name: "Resistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, resistance),
            },
            FieldInfoData {
                name: "FxTorqueRadiusMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, fx_torque_radius_multiplier),
            },
            FieldInfoData {
                name: "FrictionMomentVelocityMin",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, friction_moment_velocity_min),
            },
            FieldInfoData {
                name: "FrictionMomentVelocityMax",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, friction_moment_velocity_max),
            },
            FieldInfoData {
                name: "FrictionMomentMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, friction_moment_multiplier),
            },
            FieldInfoData {
                name: "FrictionMomentMaxFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, friction_moment_max_factor),
            },
            FieldInfoData {
                name: "BrakeFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, brake_factor),
            },
            FieldInfoData {
                name: "BrakeForce",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, brake_force),
            },
            FieldInfoData {
                name: "AutoHandBrakeIfNoThrottleAndSteer",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, auto_hand_brake_if_no_throttle_and_steer),
            },
            FieldInfoData {
                name: "HandBrakeFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, hand_brake_factor),
            },
            FieldInfoData {
                name: "HandBrakeForce",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, hand_brake_force),
            },
            FieldInfoData {
                name: "TotalFrictionScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, total_friction_scale),
            },
            FieldInfoData {
                name: "TotalLateralFrictionScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, total_lateral_friction_scale),
            },
            FieldInfoData {
                name: "LateralPosK",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, lateral_pos_k),
            },
            FieldInfoData {
                name: "LateralNegK",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, lateral_neg_k),
            },
            FieldInfoData {
                name: "LongitudinalPosK",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, longitudinal_pos_k),
            },
            FieldInfoData {
                name: "LongitudinalNegK",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, longitudinal_neg_k),
            },
            FieldInfoData {
                name: "AlignMomScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, align_mom_scale),
            },
            FieldInfoData {
                name: "WheelBaseLateral",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, wheel_base_lateral),
            },
            FieldInfoData {
                name: "WheelBaseLongitudinal",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, wheel_base_longitudinal),
            },
            FieldInfoData {
                name: "DrivingType",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, driving_type),
            },
            FieldInfoData {
                name: "SteeringType",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, steering_type),
            },
            FieldInfoData {
                name: "FrictionMethod",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, friction_method),
            },
            FieldInfoData {
                name: "RotationDirectionIndex",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, rotation_direction_index),
            },
            FieldInfoData {
                name: "SteeringAngleIndex",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, steering_angle_index),
            },
            FieldInfoData {
                name: "PacejkaConfigIndex",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, pacejka_config_index),
            },
            FieldInfoData {
                name: "EngineIndex",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, engine_index),
            },
            FieldInfoData {
                name: "AckermanDeviceType",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, ackerman_device_type),
            },
            FieldInfoData {
                name: "UseRollingResistanceVelocityFactor",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, use_rolling_resistance_velocity_factor),
            },
            FieldInfoData {
                name: "UseRollingResistanceBaseFactor",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, use_rolling_resistance_base_factor),
            },
            FieldInfoData {
                name: "UseEngineBrake",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, use_engine_brake),
            },
            FieldInfoData {
                name: "IsAllowedToSpin",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, is_allowed_to_spin),
            },
            FieldInfoData {
                name: "HasSteeringInverted",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, has_steering_inverted),
            },
            FieldInfoData {
                name: "UseFrictionMoment",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, use_friction_moment),
            },
            FieldInfoData {
                name: "UseLowSpeedAutoBrake",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, use_low_speed_auto_brake),
            },
            FieldInfoData {
                name: "AdjustWheelRotation",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, adjust_wheel_rotation),
            },
            FieldInfoData {
                name: "CollisionMaterialPair",
                flags: MemberInfoFlags::new(0),
                field_type: MATERIALDECL_TYPE_INFO,
                rust_offset: offset_of!(WheelConfigData, collision_material_pair),
            },
        ],
    }),
    array_type: Some(WHEELCONFIGDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WheelConfigData {
    fn type_info() -> &'static TypeInfo {
        WHEELCONFIGDATA_TYPE_INFO
    }
}


pub const WHEELCONFIGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WheelConfigData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("WheelConfigData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FrictionScaleAtVelocity {
    pub friction_scale: f32,
    pub velocity: f32,
}

pub const FRICTIONSCALEATVELOCITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FrictionScaleAtVelocity",
    flags: MemberInfoFlags::new(36937),
    module: "Physics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "FrictionScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FrictionScaleAtVelocity, friction_scale),
            },
            FieldInfoData {
                name: "Velocity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FrictionScaleAtVelocity, velocity),
            },
        ],
    }),
    array_type: Some(FRICTIONSCALEATVELOCITY_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for FrictionScaleAtVelocity {
    fn type_info() -> &'static TypeInfo {
        FRICTIONSCALEATVELOCITY_TYPE_INFO
    }
}


pub const FRICTIONSCALEATVELOCITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FrictionScaleAtVelocity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("FrictionScaleAtVelocity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SensitivityAtVelocity {
    pub steering_sensitivity: f32,
    pub velocity: f32,
}

pub const SENSITIVITYATVELOCITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SensitivityAtVelocity",
    flags: MemberInfoFlags::new(36937),
    module: "Physics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "SteeringSensitivity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SensitivityAtVelocity, steering_sensitivity),
            },
            FieldInfoData {
                name: "Velocity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SensitivityAtVelocity, velocity),
            },
        ],
    }),
    array_type: Some(SENSITIVITYATVELOCITY_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for SensitivityAtVelocity {
    fn type_info() -> &'static TypeInfo {
        SENSITIVITYATVELOCITY_TYPE_INFO
    }
}


pub const SENSITIVITYATVELOCITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SensitivityAtVelocity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("SensitivityAtVelocity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SpringData {
    pub length: f32,
    pub stiffness: f32,
    pub damping: f32,
    pub progressive_start_ratio: f32,
    pub progressive_exponent: f32,
    pub visual_clip_offset: f32,
    pub attach_offset_y: f32,
    pub disabled_strengh_modifier: f32,
}

pub const SPRINGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpringData",
    flags: MemberInfoFlags::new(36937),
    module: "Physics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Length",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpringData, length),
            },
            FieldInfoData {
                name: "Stiffness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpringData, stiffness),
            },
            FieldInfoData {
                name: "Damping",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpringData, damping),
            },
            FieldInfoData {
                name: "ProgressiveStartRatio",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpringData, progressive_start_ratio),
            },
            FieldInfoData {
                name: "ProgressiveExponent",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpringData, progressive_exponent),
            },
            FieldInfoData {
                name: "VisualClipOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpringData, visual_clip_offset),
            },
            FieldInfoData {
                name: "AttachOffsetY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpringData, attach_offset_y),
            },
            FieldInfoData {
                name: "DisabledStrenghModifier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpringData, disabled_strengh_modifier),
            },
        ],
    }),
    array_type: Some(SPRINGDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for SpringData {
    fn type_info() -> &'static TypeInfo {
        SPRINGDATA_TYPE_INFO
    }
}


pub const SPRINGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpringData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("SpringData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SphereCollisionData {
    pub counter_normal_brake_force_mod: f32,
}

pub const SPHERECOLLISIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SphereCollisionData",
    flags: MemberInfoFlags::new(36937),
    module: "Physics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "CounterNormalBrakeForceMod",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SphereCollisionData, counter_normal_brake_force_mod),
            },
        ],
    }),
    array_type: Some(SPHERECOLLISIONDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for SphereCollisionData {
    fn type_info() -> &'static TypeInfo {
        SPHERECOLLISIONDATA_TYPE_INFO
    }
}


pub const SPHERECOLLISIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SphereCollisionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("SphereCollisionData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct JetEngineConfigData {
    pub direction_vector_index: u32,
    pub force_magnitude_multiplier: f32,
    pub angle_input_y_multiplier: f32,
    pub angle_input_pitch_multiplier: f32,
    pub pitch_force_modifier: f32,
    pub max_velocity: f32,
    pub is_turnable: bool,
    pub is_water_jet_engine: bool,
    pub use_force_position_water_test: bool,
    pub use_hull_in_water_test: bool,
    pub steering_sensitivity: Vec<SensitivityAtVelocity>,
    pub power_fade_out_range: super::core::Vec2,
}

pub const JETENGINECONFIGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "JetEngineConfigData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENGINECONFIGDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DirectionVectorIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(JetEngineConfigData, direction_vector_index),
            },
            FieldInfoData {
                name: "ForceMagnitudeMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(JetEngineConfigData, force_magnitude_multiplier),
            },
            FieldInfoData {
                name: "AngleInputYMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(JetEngineConfigData, angle_input_y_multiplier),
            },
            FieldInfoData {
                name: "AngleInputPitchMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(JetEngineConfigData, angle_input_pitch_multiplier),
            },
            FieldInfoData {
                name: "PitchForceModifier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(JetEngineConfigData, pitch_force_modifier),
            },
            FieldInfoData {
                name: "MaxVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(JetEngineConfigData, max_velocity),
            },
            FieldInfoData {
                name: "IsTurnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(JetEngineConfigData, is_turnable),
            },
            FieldInfoData {
                name: "IsWaterJetEngine",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(JetEngineConfigData, is_water_jet_engine),
            },
            FieldInfoData {
                name: "UseForcePositionWaterTest",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(JetEngineConfigData, use_force_position_water_test),
            },
            FieldInfoData {
                name: "UseHullInWaterTest",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(JetEngineConfigData, use_hull_in_water_test),
            },
            FieldInfoData {
                name: "SteeringSensitivity",
                flags: MemberInfoFlags::new(144),
                field_type: SENSITIVITYATVELOCITY_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(JetEngineConfigData, steering_sensitivity),
            },
            FieldInfoData {
                name: "PowerFadeOutRange",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(JetEngineConfigData, power_fade_out_range),
            },
        ],
    }),
    array_type: Some(JETENGINECONFIGDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for JetEngineConfigData {
    fn type_info() -> &'static TypeInfo {
        JETENGINECONFIGDATA_TYPE_INFO
    }
}


pub const JETENGINECONFIGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "JetEngineConfigData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("JetEngineConfigData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PropellerEngineConfigData {
    pub horisontal_force_offset: super::core::Vec3,
    pub propeller_type: PropellerType,
    pub rotor_config: RotorParameters,
    pub force_magnitude_input_type: ForceMagnitudeInputType,
    pub direction_vector_index: u32,
    pub apply_force_as_torque: bool,
    pub force_magnitude_multiplier: f32,
    pub lift_force_spring_constant: f32,
    pub lift_force_damping_constant: f32,
    pub cyclic_input_scale_roll: f32,
    pub cyclic_roll_lift_mod: f32,
    pub cyclic_roll_strafe_mod: f32,
    pub cyclic_input_scale_pitch: f32,
    pub cyclic_pitch_lift_mod: f32,
    pub cyclic_pitch_strafe_mod: f32,
    pub cyclic_pitch_strafe_brake_mod: f32,
    pub collective_input_idle: f32,
    pub collective_throttle_input_scale: f32,
    pub collective_brake_input_scale: f32,
    pub default_angle_of_attack: f32,
    pub attack_angle_mod: f32,
    pub stabilizer_mod: f32,
    pub horisontal_min_effect_velocity: f32,
    pub horisontal_min_effect_mod: f32,
    pub enable_new_helicopter: bool,
    pub s_p_allowed: bool,
    pub s_p_default: bool,
    pub s_p_forward_input: ForceMagnitudeInputType,
    pub s_p_sideways_input: ForceMagnitudeInputType,
    pub s_p_vertical_input: ForceMagnitudeInputType,
    pub s_p_pitch_input: ForceMagnitudeInputType,
    pub s_p_yaw_input: ForceMagnitudeInputType,
    pub s_p_forward_strength: f32,
    pub s_p_sideways_strength: f32,
    pub s_p_vertical_strength: f32,
    pub s_p_reverse_force_mod: f32,
    pub pitch_strength: f32,
    pub pitch_limit: f32,
    pub pitch_from_vel: f32,
    pub velocity_from_pitch: f32,
    pub roll_strength: f32,
    pub banking_strength: f32,
    pub banking_limit: f32,
    pub pitch_up_when_bank_strength: f32,
    pub pitch_up_when_bank_limit: f32,
    pub gravity_mod: f32,
}

pub const PROPELLERENGINECONFIGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropellerEngineConfigData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENGINECONFIGDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "HorisontalForceOffset",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, horisontal_force_offset),
            },
            FieldInfoData {
                name: "PropellerType",
                flags: MemberInfoFlags::new(0),
                field_type: PROPELLERTYPE_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, propeller_type),
            },
            FieldInfoData {
                name: "RotorConfig",
                flags: MemberInfoFlags::new(0),
                field_type: ROTORPARAMETERS_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, rotor_config),
            },
            FieldInfoData {
                name: "ForceMagnitudeInputType",
                flags: MemberInfoFlags::new(0),
                field_type: FORCEMAGNITUDEINPUTTYPE_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, force_magnitude_input_type),
            },
            FieldInfoData {
                name: "DirectionVectorIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, direction_vector_index),
            },
            FieldInfoData {
                name: "ApplyForceAsTorque",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, apply_force_as_torque),
            },
            FieldInfoData {
                name: "ForceMagnitudeMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, force_magnitude_multiplier),
            },
            FieldInfoData {
                name: "LiftForceSpringConstant",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, lift_force_spring_constant),
            },
            FieldInfoData {
                name: "LiftForceDampingConstant",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, lift_force_damping_constant),
            },
            FieldInfoData {
                name: "CyclicInputScaleRoll",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, cyclic_input_scale_roll),
            },
            FieldInfoData {
                name: "CyclicRollLiftMod",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, cyclic_roll_lift_mod),
            },
            FieldInfoData {
                name: "CyclicRollStrafeMod",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, cyclic_roll_strafe_mod),
            },
            FieldInfoData {
                name: "CyclicInputScalePitch",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, cyclic_input_scale_pitch),
            },
            FieldInfoData {
                name: "CyclicPitchLiftMod",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, cyclic_pitch_lift_mod),
            },
            FieldInfoData {
                name: "CyclicPitchStrafeMod",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, cyclic_pitch_strafe_mod),
            },
            FieldInfoData {
                name: "CyclicPitchStrafeBrakeMod",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, cyclic_pitch_strafe_brake_mod),
            },
            FieldInfoData {
                name: "CollectiveInputIdle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, collective_input_idle),
            },
            FieldInfoData {
                name: "CollectiveThrottleInputScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, collective_throttle_input_scale),
            },
            FieldInfoData {
                name: "CollectiveBrakeInputScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, collective_brake_input_scale),
            },
            FieldInfoData {
                name: "DefaultAngleOfAttack",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, default_angle_of_attack),
            },
            FieldInfoData {
                name: "AttackAngleMod",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, attack_angle_mod),
            },
            FieldInfoData {
                name: "StabilizerMod",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, stabilizer_mod),
            },
            FieldInfoData {
                name: "HorisontalMinEffectVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, horisontal_min_effect_velocity),
            },
            FieldInfoData {
                name: "HorisontalMinEffectMod",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, horisontal_min_effect_mod),
            },
            FieldInfoData {
                name: "EnableNewHelicopter",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, enable_new_helicopter),
            },
            FieldInfoData {
                name: "SPAllowed",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, s_p_allowed),
            },
            FieldInfoData {
                name: "SPDefault",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, s_p_default),
            },
            FieldInfoData {
                name: "SPForwardInput",
                flags: MemberInfoFlags::new(0),
                field_type: FORCEMAGNITUDEINPUTTYPE_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, s_p_forward_input),
            },
            FieldInfoData {
                name: "SPSidewaysInput",
                flags: MemberInfoFlags::new(0),
                field_type: FORCEMAGNITUDEINPUTTYPE_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, s_p_sideways_input),
            },
            FieldInfoData {
                name: "SPVerticalInput",
                flags: MemberInfoFlags::new(0),
                field_type: FORCEMAGNITUDEINPUTTYPE_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, s_p_vertical_input),
            },
            FieldInfoData {
                name: "SPPitchInput",
                flags: MemberInfoFlags::new(0),
                field_type: FORCEMAGNITUDEINPUTTYPE_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, s_p_pitch_input),
            },
            FieldInfoData {
                name: "SPYawInput",
                flags: MemberInfoFlags::new(0),
                field_type: FORCEMAGNITUDEINPUTTYPE_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, s_p_yaw_input),
            },
            FieldInfoData {
                name: "SPForwardStrength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, s_p_forward_strength),
            },
            FieldInfoData {
                name: "SPSidewaysStrength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, s_p_sideways_strength),
            },
            FieldInfoData {
                name: "SPVerticalStrength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, s_p_vertical_strength),
            },
            FieldInfoData {
                name: "SPReverseForceMod",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, s_p_reverse_force_mod),
            },
            FieldInfoData {
                name: "PitchStrength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, pitch_strength),
            },
            FieldInfoData {
                name: "PitchLimit",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, pitch_limit),
            },
            FieldInfoData {
                name: "PitchFromVel",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, pitch_from_vel),
            },
            FieldInfoData {
                name: "VelocityFromPitch",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, velocity_from_pitch),
            },
            FieldInfoData {
                name: "RollStrength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, roll_strength),
            },
            FieldInfoData {
                name: "BankingStrength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, banking_strength),
            },
            FieldInfoData {
                name: "BankingLimit",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, banking_limit),
            },
            FieldInfoData {
                name: "PitchUpWhenBankStrength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, pitch_up_when_bank_strength),
            },
            FieldInfoData {
                name: "PitchUpWhenBankLimit",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, pitch_up_when_bank_limit),
            },
            FieldInfoData {
                name: "GravityMod",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PropellerEngineConfigData, gravity_mod),
            },
        ],
    }),
    array_type: Some(PROPELLERENGINECONFIGDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PropellerEngineConfigData {
    fn type_info() -> &'static TypeInfo {
        PROPELLERENGINECONFIGDATA_TYPE_INFO
    }
}


pub const PROPELLERENGINECONFIGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropellerEngineConfigData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PropellerEngineConfigData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RotorParameters {
    pub cyclic_input_scale_roll: f32,
    pub cyclic_input_scale_pitch: f32,
    pub collective_throttle_input_scale: f32,
    pub collective_brake_input_scale: f32,
    pub collective_input_idle: f32,
    pub horizontal_force_modifier: f32,
    pub angle_of_attack: Vec<super::core::Vec2>,
    pub cyclic_fade_out_offset: f32,
    pub additional_gravity_modifier: f32,
    pub reverse_throttle: bool,
    pub enable_horisontal_min_effect: bool,
    pub horisontal_min_effect_velocity: f32,
    pub horisontal_min_effect_mod: f32,
}

pub const ROTORPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RotorParameters",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "CyclicInputScaleRoll",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RotorParameters, cyclic_input_scale_roll),
            },
            FieldInfoData {
                name: "CyclicInputScalePitch",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RotorParameters, cyclic_input_scale_pitch),
            },
            FieldInfoData {
                name: "CollectiveThrottleInputScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RotorParameters, collective_throttle_input_scale),
            },
            FieldInfoData {
                name: "CollectiveBrakeInputScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RotorParameters, collective_brake_input_scale),
            },
            FieldInfoData {
                name: "CollectiveInputIdle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RotorParameters, collective_input_idle),
            },
            FieldInfoData {
                name: "HorizontalForceModifier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RotorParameters, horizontal_force_modifier),
            },
            FieldInfoData {
                name: "AngleOfAttack",
                flags: MemberInfoFlags::new(144),
                field_type: VEC2_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(RotorParameters, angle_of_attack),
            },
            FieldInfoData {
                name: "CyclicFadeOutOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RotorParameters, cyclic_fade_out_offset),
            },
            FieldInfoData {
                name: "AdditionalGravityModifier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RotorParameters, additional_gravity_modifier),
            },
            FieldInfoData {
                name: "ReverseThrottle",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RotorParameters, reverse_throttle),
            },
            FieldInfoData {
                name: "EnableHorisontalMinEffect",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RotorParameters, enable_horisontal_min_effect),
            },
            FieldInfoData {
                name: "HorisontalMinEffectVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RotorParameters, horisontal_min_effect_velocity),
            },
            FieldInfoData {
                name: "HorisontalMinEffectMod",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RotorParameters, horisontal_min_effect_mod),
            },
        ],
    }),
    array_type: Some(ROTORPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RotorParameters {
    fn type_info() -> &'static TypeInfo {
        ROTORPARAMETERS_TYPE_INFO
    }
}


pub const ROTORPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RotorParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("RotorParameters-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ForceMagnitudeInputType {
    #[default]
    FMITYaw = 0,
    FMITPitch = 1,
    FMITRoll = 2,
    FMITThrottle = 3,
    FMITStrafe = 4,
}

pub const FORCEMAGNITUDEINPUTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForceMagnitudeInputType",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(FORCEMAGNITUDEINPUTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ForceMagnitudeInputType {
    fn type_info() -> &'static TypeInfo {
        FORCEMAGNITUDEINPUTTYPE_TYPE_INFO
    }
}


pub const FORCEMAGNITUDEINPUTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForceMagnitudeInputType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ForceMagnitudeInputType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum PropellerType {
    #[default]
    PropellerType_Regular = 0,
    PropellerType_Rotor = 1,
}

pub const PROPELLERTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropellerType",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(PROPELLERTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PropellerType {
    fn type_info() -> &'static TypeInfo {
        PROPELLERTYPE_TYPE_INFO
    }
}


pub const PROPELLERTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropellerType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PropellerType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CombustionEngineConfigData {
}

pub const COMBUSTIONENGINECONFIGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CombustionEngineConfigData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENGINECONFIGDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(COMBUSTIONENGINECONFIGDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CombustionEngineConfigData {
    fn type_info() -> &'static TypeInfo {
        COMBUSTIONENGINECONFIGDATA_TYPE_INFO
    }
}


pub const COMBUSTIONENGINECONFIGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CombustionEngineConfigData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("CombustionEngineConfigData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EngineConfigData {
    pub position: super::core::Vec3,
    pub rpm_curve_points: Vec<f32>,
    pub torque_curve_points: Vec<f32>,
    pub rpm_min: f32,
    pub rpm_max: f32,
    pub rpm_cut: f32,
    pub engine_power_multiplier: f32,
    pub internal_acceleration_factor: f32,
    pub internal_deacceleration_factor: f32,
    pub max_reverse_speed: f32,
    pub boost: Boost,
    pub max_vehicle_height_offset: f32,
}

pub const ENGINECONFIGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EngineConfigData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Position",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(EngineConfigData, position),
            },
            FieldInfoData {
                name: "RpmCurvePoints",
                flags: MemberInfoFlags::new(144),
                field_type: FLOAT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EngineConfigData, rpm_curve_points),
            },
            FieldInfoData {
                name: "TorqueCurvePoints",
                flags: MemberInfoFlags::new(144),
                field_type: FLOAT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EngineConfigData, torque_curve_points),
            },
            FieldInfoData {
                name: "RpmMin",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EngineConfigData, rpm_min),
            },
            FieldInfoData {
                name: "RpmMax",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EngineConfigData, rpm_max),
            },
            FieldInfoData {
                name: "RpmCut",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EngineConfigData, rpm_cut),
            },
            FieldInfoData {
                name: "EnginePowerMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EngineConfigData, engine_power_multiplier),
            },
            FieldInfoData {
                name: "InternalAccelerationFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EngineConfigData, internal_acceleration_factor),
            },
            FieldInfoData {
                name: "InternalDeaccelerationFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EngineConfigData, internal_deacceleration_factor),
            },
            FieldInfoData {
                name: "MaxReverseSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EngineConfigData, max_reverse_speed),
            },
            FieldInfoData {
                name: "Boost",
                flags: MemberInfoFlags::new(0),
                field_type: BOOST_TYPE_INFO,
                rust_offset: offset_of!(EngineConfigData, boost),
            },
            FieldInfoData {
                name: "MaxVehicleHeightOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EngineConfigData, max_vehicle_height_offset),
            },
        ],
    }),
    array_type: Some(ENGINECONFIGDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EngineConfigData {
    fn type_info() -> &'static TypeInfo {
        ENGINECONFIGDATA_TYPE_INFO
    }
}


pub const ENGINECONFIGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EngineConfigData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("EngineConfigData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct Boost {
    pub forward_strength: f32,
    pub reverse_strength: f32,
    pub dissipation_time: f32,
    pub recovery_time: f32,
    pub crawl_strength: f32,
    pub acceleration_scale: f32,
}

pub const BOOST_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Boost",
    flags: MemberInfoFlags::new(36937),
    module: "Physics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ForwardStrength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Boost, forward_strength),
            },
            FieldInfoData {
                name: "ReverseStrength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Boost, reverse_strength),
            },
            FieldInfoData {
                name: "DissipationTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Boost, dissipation_time),
            },
            FieldInfoData {
                name: "RecoveryTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Boost, recovery_time),
            },
            FieldInfoData {
                name: "CrawlStrength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Boost, crawl_strength),
            },
            FieldInfoData {
                name: "AccelerationScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Boost, acceleration_scale),
            },
        ],
    }),
    array_type: Some(BOOST_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for Boost {
    fn type_info() -> &'static TypeInfo {
        BOOST_TYPE_INFO
    }
}


pub const BOOST_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Boost-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("Boost-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct GearboxConfigData {
    pub forward_gear_ratios: Vec<f32>,
    pub forward_gear_speeds: Vec<f32>,
    pub reverse_gear_ratios: Vec<f32>,
    pub reverse_gear_speeds: Vec<f32>,
    pub gearbox_type: i32,
    pub gearbox_mode: i32,
    pub gear_change_time: f32,
    pub gear_down_speed_factor: f32,
    pub opposite_dir_gear_change_max_speed: f32,
    pub opposite_dir_gear_change_time: f32,
    pub clutch_speed_factor: f32,
    pub transmission_efficiency: f32,
    pub use_auto_clutch: bool,
    pub use_classic_gear_box_auto_clutch: bool,
    pub use_neutral_gear: bool,
}

pub const GEARBOXCONFIGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GearboxConfigData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ForwardGearRatios",
                flags: MemberInfoFlags::new(144),
                field_type: FLOAT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(GearboxConfigData, forward_gear_ratios),
            },
            FieldInfoData {
                name: "ForwardGearSpeeds",
                flags: MemberInfoFlags::new(144),
                field_type: FLOAT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(GearboxConfigData, forward_gear_speeds),
            },
            FieldInfoData {
                name: "ReverseGearRatios",
                flags: MemberInfoFlags::new(144),
                field_type: FLOAT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(GearboxConfigData, reverse_gear_ratios),
            },
            FieldInfoData {
                name: "ReverseGearSpeeds",
                flags: MemberInfoFlags::new(144),
                field_type: FLOAT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(GearboxConfigData, reverse_gear_speeds),
            },
            FieldInfoData {
                name: "GearboxType",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(GearboxConfigData, gearbox_type),
            },
            FieldInfoData {
                name: "GearboxMode",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(GearboxConfigData, gearbox_mode),
            },
            FieldInfoData {
                name: "GearChangeTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GearboxConfigData, gear_change_time),
            },
            FieldInfoData {
                name: "GearDownSpeedFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GearboxConfigData, gear_down_speed_factor),
            },
            FieldInfoData {
                name: "OppositeDirGearChangeMaxSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GearboxConfigData, opposite_dir_gear_change_max_speed),
            },
            FieldInfoData {
                name: "OppositeDirGearChangeTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GearboxConfigData, opposite_dir_gear_change_time),
            },
            FieldInfoData {
                name: "ClutchSpeedFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GearboxConfigData, clutch_speed_factor),
            },
            FieldInfoData {
                name: "TransmissionEfficiency",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GearboxConfigData, transmission_efficiency),
            },
            FieldInfoData {
                name: "UseAutoClutch",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GearboxConfigData, use_auto_clutch),
            },
            FieldInfoData {
                name: "UseClassicGearBoxAutoClutch",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GearboxConfigData, use_classic_gear_box_auto_clutch),
            },
            FieldInfoData {
                name: "UseNeutralGear",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GearboxConfigData, use_neutral_gear),
            },
        ],
    }),
    array_type: Some(GEARBOXCONFIGDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GearboxConfigData {
    fn type_info() -> &'static TypeInfo {
        GEARBOXCONFIGDATA_TYPE_INFO
    }
}


pub const GEARBOXCONFIGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GearboxConfigData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("GearboxConfigData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct VehicleInputTweakData {
    pub input_tweak_type: VehicleInputTweakType,
    pub min_speed: f32,
    pub max_speed: f32,
    pub min_speed_scale: f32,
    pub max_speed_scale: f32,
}

pub const VEHICLEINPUTTWEAKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleInputTweakData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "InputTweakType",
                flags: MemberInfoFlags::new(0),
                field_type: VEHICLEINPUTTWEAKTYPE_TYPE_INFO,
                rust_offset: offset_of!(VehicleInputTweakData, input_tweak_type),
            },
            FieldInfoData {
                name: "MinSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleInputTweakData, min_speed),
            },
            FieldInfoData {
                name: "MaxSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleInputTweakData, max_speed),
            },
            FieldInfoData {
                name: "MinSpeedScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleInputTweakData, min_speed_scale),
            },
            FieldInfoData {
                name: "MaxSpeedScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleInputTweakData, max_speed_scale),
            },
        ],
    }),
    array_type: Some(VEHICLEINPUTTWEAKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VehicleInputTweakData {
    fn type_info() -> &'static TypeInfo {
        VEHICLEINPUTTWEAKDATA_TYPE_INFO
    }
}


pub const VEHICLEINPUTTWEAKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleInputTweakData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("VehicleInputTweakData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum VehicleInputTweakType {
    #[default]
    CombinedTimedSpeedTweakType = 0,
    CombinedSpeedTweakType = 1,
    SpeedTweakType = 2,
    ScaledSpeedTweakType = 3,
}

pub const VEHICLEINPUTTWEAKTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleInputTweakType",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(VEHICLEINPUTTWEAKTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for VehicleInputTweakType {
    fn type_info() -> &'static TypeInfo {
        VEHICLEINPUTTWEAKTYPE_TYPE_INFO
    }
}


pub const VEHICLEINPUTTWEAKTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleInputTweakType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("VehicleInputTweakType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MotionDampingData {
    pub linear_modifier: super::core::Vec3,
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
    pub linear: f32,
}

pub const MOTIONDAMPINGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MotionDampingData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "LinearModifier",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(MotionDampingData, linear_modifier),
            },
            FieldInfoData {
                name: "Pitch",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotionDampingData, pitch),
            },
            FieldInfoData {
                name: "Yaw",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotionDampingData, yaw),
            },
            FieldInfoData {
                name: "Roll",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotionDampingData, roll),
            },
            FieldInfoData {
                name: "Linear",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotionDampingData, linear),
            },
        ],
    }),
    array_type: Some(MOTIONDAMPINGDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for MotionDampingData {
    fn type_info() -> &'static TypeInfo {
        MOTIONDAMPINGDATA_TYPE_INFO
    }
}


pub const MOTIONDAMPINGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MotionDampingData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("MotionDampingData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct StabilizerData {
    pub pitch_strength: f32,
    pub roll_strength: f32,
    pub advanced: bool,
    pub yaw_strength: f32,
    pub advanced_yaw: bool,
    pub vertical_velocity_strength: f32,
}

pub const STABILIZERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StabilizerData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "PitchStrength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(StabilizerData, pitch_strength),
            },
            FieldInfoData {
                name: "RollStrength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(StabilizerData, roll_strength),
            },
            FieldInfoData {
                name: "Advanced",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(StabilizerData, advanced),
            },
            FieldInfoData {
                name: "YawStrength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(StabilizerData, yaw_strength),
            },
            FieldInfoData {
                name: "AdvancedYaw",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(StabilizerData, advanced_yaw),
            },
            FieldInfoData {
                name: "VerticalVelocityStrength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(StabilizerData, vertical_velocity_strength),
            },
        ],
    }),
    array_type: Some(STABILIZERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StabilizerData {
    fn type_info() -> &'static TypeInfo {
        STABILIZERDATA_TYPE_INFO
    }
}


pub const STABILIZERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StabilizerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("StabilizerData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WingPhysicsData {
    pub lift: f32,
    pub flap_lift: f32,
    pub lift_coefficient: Curve2D,
    pub drag: f32,
    pub flap_drag: f32,
    pub drag_coefficient: Curve2D,
    pub angular_velocity_lift_modifier: f32,
    pub drag_rotation_modifier: f32,
    pub base_angle_of_attack: f32,
    pub input_for_flap: i32,
    pub flap_turn_speed: f32,
    pub visual_flap_turn_speed: f32,
    pub visual_flap_angle_limit: f32,
    pub max_flap_angle_scale_factor: Curve2D,
    pub landing_flap_lift: f32,
    pub landing_flap_logic: LandingFlapData,
}

pub const WINGPHYSICSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WingPhysicsData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Lift",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WingPhysicsData, lift),
            },
            FieldInfoData {
                name: "FlapLift",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WingPhysicsData, flap_lift),
            },
            FieldInfoData {
                name: "LiftCoefficient",
                flags: MemberInfoFlags::new(0),
                field_type: CURVE2D_TYPE_INFO,
                rust_offset: offset_of!(WingPhysicsData, lift_coefficient),
            },
            FieldInfoData {
                name: "Drag",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WingPhysicsData, drag),
            },
            FieldInfoData {
                name: "FlapDrag",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WingPhysicsData, flap_drag),
            },
            FieldInfoData {
                name: "DragCoefficient",
                flags: MemberInfoFlags::new(0),
                field_type: CURVE2D_TYPE_INFO,
                rust_offset: offset_of!(WingPhysicsData, drag_coefficient),
            },
            FieldInfoData {
                name: "AngularVelocityLiftModifier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WingPhysicsData, angular_velocity_lift_modifier),
            },
            FieldInfoData {
                name: "DragRotationModifier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WingPhysicsData, drag_rotation_modifier),
            },
            FieldInfoData {
                name: "BaseAngleOfAttack",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WingPhysicsData, base_angle_of_attack),
            },
            FieldInfoData {
                name: "InputForFlap",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(WingPhysicsData, input_for_flap),
            },
            FieldInfoData {
                name: "FlapTurnSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WingPhysicsData, flap_turn_speed),
            },
            FieldInfoData {
                name: "VisualFlapTurnSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WingPhysicsData, visual_flap_turn_speed),
            },
            FieldInfoData {
                name: "VisualFlapAngleLimit",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WingPhysicsData, visual_flap_angle_limit),
            },
            FieldInfoData {
                name: "MaxFlapAngleScaleFactor",
                flags: MemberInfoFlags::new(0),
                field_type: CURVE2D_TYPE_INFO,
                rust_offset: offset_of!(WingPhysicsData, max_flap_angle_scale_factor),
            },
            FieldInfoData {
                name: "LandingFlapLift",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WingPhysicsData, landing_flap_lift),
            },
            FieldInfoData {
                name: "LandingFlapLogic",
                flags: MemberInfoFlags::new(0),
                field_type: LANDINGFLAPDATA_TYPE_INFO,
                rust_offset: offset_of!(WingPhysicsData, landing_flap_logic),
            },
        ],
    }),
    array_type: Some(WINGPHYSICSDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WingPhysicsData {
    fn type_info() -> &'static TypeInfo {
        WINGPHYSICSDATA_TYPE_INFO
    }
}


pub const WINGPHYSICSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WingPhysicsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("WingPhysicsData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct Curve2D {
    pub curve: Vec<super::core::Vec2>,
}

pub const CURVE2D_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Curve2D",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Curve",
                flags: MemberInfoFlags::new(144),
                field_type: VEC2_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(Curve2D, curve),
            },
        ],
    }),
    array_type: Some(CURVE2D_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Curve2D {
    fn type_info() -> &'static TypeInfo {
        CURVE2D_TYPE_INFO
    }
}


pub const CURVE2D_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Curve2D-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("Curve2D-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LandingFlapData {
    pub activation_height: f32,
    pub height_tolerance: f32,
    pub activation_velocity: f32,
    pub velocity_tolerance: f32,
}

pub const LANDINGFLAPDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LandingFlapData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ActivationHeight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LandingFlapData, activation_height),
            },
            FieldInfoData {
                name: "HeightTolerance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LandingFlapData, height_tolerance),
            },
            FieldInfoData {
                name: "ActivationVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LandingFlapData, activation_velocity),
            },
            FieldInfoData {
                name: "VelocityTolerance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LandingFlapData, velocity_tolerance),
            },
        ],
    }),
    array_type: Some(LANDINGFLAPDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LandingFlapData {
    fn type_info() -> &'static TypeInfo {
        LANDINGFLAPDATA_TYPE_INFO
    }
}


pub const LANDINGFLAPDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LandingFlapData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("LandingFlapData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AeroDynamicPhysicsData {
    pub body_drag: super::core::Vec3,
    pub body_drag_offset_y_z: super::core::Vec3,
    pub body_drag_offset_x_z: super::core::Vec3,
    pub body_drag_offset_x_y: super::core::Vec3,
}

pub const AERODYNAMICPHYSICSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AeroDynamicPhysicsData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "BodyDrag",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AeroDynamicPhysicsData, body_drag),
            },
            FieldInfoData {
                name: "BodyDragOffsetYZ",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AeroDynamicPhysicsData, body_drag_offset_y_z),
            },
            FieldInfoData {
                name: "BodyDragOffsetXZ",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AeroDynamicPhysicsData, body_drag_offset_x_z),
            },
            FieldInfoData {
                name: "BodyDragOffsetXY",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AeroDynamicPhysicsData, body_drag_offset_x_y),
            },
        ],
    }),
    array_type: Some(AERODYNAMICPHYSICSDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AeroDynamicPhysicsData {
    fn type_info() -> &'static TypeInfo {
        AERODYNAMICPHYSICSDATA_TYPE_INFO
    }
}


pub const AERODYNAMICPHYSICSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AeroDynamicPhysicsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("AeroDynamicPhysicsData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct HovercraftFloatPhysicsData {
    pub land_resistance_axis_mod: super::core::Vec3,
    pub land_friction_axis_mod: super::core::Vec3,
    pub front_length: f32,
    pub side_length: f32,
}

pub const HOVERCRAFTFLOATPHYSICSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HovercraftFloatPhysicsData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(HULLFLOATPHYSICSDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "LandResistanceAxisMod",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HovercraftFloatPhysicsData, land_resistance_axis_mod),
            },
            FieldInfoData {
                name: "LandFrictionAxisMod",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HovercraftFloatPhysicsData, land_friction_axis_mod),
            },
            FieldInfoData {
                name: "FrontLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HovercraftFloatPhysicsData, front_length),
            },
            FieldInfoData {
                name: "SideLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HovercraftFloatPhysicsData, side_length),
            },
        ],
    }),
    array_type: Some(HOVERCRAFTFLOATPHYSICSDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for HovercraftFloatPhysicsData {
    fn type_info() -> &'static TypeInfo {
        HOVERCRAFTFLOATPHYSICSDATA_TYPE_INFO
    }
}


pub const HOVERCRAFTFLOATPHYSICSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HovercraftFloatPhysicsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("HovercraftFloatPhysicsData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct BoatFloatPhysicsData {
    pub front_ratio: f32,
}

pub const BOATFLOATPHYSICSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoatFloatPhysicsData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(HULLFLOATPHYSICSDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "FrontRatio",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BoatFloatPhysicsData, front_ratio),
            },
        ],
    }),
    array_type: Some(BOATFLOATPHYSICSDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for BoatFloatPhysicsData {
    fn type_info() -> &'static TypeInfo {
        BOATFLOATPHYSICSDATA_TYPE_INFO
    }
}


pub const BOATFLOATPHYSICSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoatFloatPhysicsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("BoatFloatPhysicsData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct HullFloatPhysicsData {
    pub water_resistance_axis_mod: super::core::Vec3,
    pub water_friction_axis_mod: super::core::Vec3,
    pub offset: super::core::Vec3,
    pub sub_surface_splits: i32,
    pub depth: f32,
    pub width: f32,
    pub length: f32,
    pub front_curve_degree: f32,
    pub side_curve_degree: f32,
    pub non_engine_steer: f32,
    pub non_engine_steer_min_speed: f32,
    pub non_engine_steer_max_speed: f32,
    pub water_dampening_mod: f32,
    pub lift_modifier: f32,
    pub support_size_mod: f32,
    pub angular_dampening: f32,
    pub friction_throttle_modifier: f32,
}

pub const HULLFLOATPHYSICSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HullFloatPhysicsData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(FLOATPHYSICSDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "WaterResistanceAxisMod",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HullFloatPhysicsData, water_resistance_axis_mod),
            },
            FieldInfoData {
                name: "WaterFrictionAxisMod",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HullFloatPhysicsData, water_friction_axis_mod),
            },
            FieldInfoData {
                name: "Offset",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HullFloatPhysicsData, offset),
            },
            FieldInfoData {
                name: "SubSurfaceSplits",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(HullFloatPhysicsData, sub_surface_splits),
            },
            FieldInfoData {
                name: "Depth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HullFloatPhysicsData, depth),
            },
            FieldInfoData {
                name: "Width",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HullFloatPhysicsData, width),
            },
            FieldInfoData {
                name: "Length",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HullFloatPhysicsData, length),
            },
            FieldInfoData {
                name: "FrontCurveDegree",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HullFloatPhysicsData, front_curve_degree),
            },
            FieldInfoData {
                name: "SideCurveDegree",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HullFloatPhysicsData, side_curve_degree),
            },
            FieldInfoData {
                name: "NonEngineSteer",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HullFloatPhysicsData, non_engine_steer),
            },
            FieldInfoData {
                name: "NonEngineSteerMinSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HullFloatPhysicsData, non_engine_steer_min_speed),
            },
            FieldInfoData {
                name: "NonEngineSteerMaxSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HullFloatPhysicsData, non_engine_steer_max_speed),
            },
            FieldInfoData {
                name: "WaterDampeningMod",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HullFloatPhysicsData, water_dampening_mod),
            },
            FieldInfoData {
                name: "LiftModifier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HullFloatPhysicsData, lift_modifier),
            },
            FieldInfoData {
                name: "SupportSizeMod",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HullFloatPhysicsData, support_size_mod),
            },
            FieldInfoData {
                name: "AngularDampening",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HullFloatPhysicsData, angular_dampening),
            },
            FieldInfoData {
                name: "FrictionThrottleModifier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HullFloatPhysicsData, friction_throttle_modifier),
            },
        ],
    }),
    array_type: Some(HULLFLOATPHYSICSDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for HullFloatPhysicsData {
    fn type_info() -> &'static TypeInfo {
        HULLFLOATPHYSICSDATA_TYPE_INFO
    }
}


pub const HULLFLOATPHYSICSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HullFloatPhysicsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("HullFloatPhysicsData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VehiclePhysicsActionData {
}

pub const VEHICLEPHYSICSACTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehiclePhysicsActionData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSACTIONDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VEHICLEPHYSICSACTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VehiclePhysicsActionData {
    fn type_info() -> &'static TypeInfo {
        VEHICLEPHYSICSACTIONDATA_TYPE_INFO
    }
}


pub const VEHICLEPHYSICSACTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehiclePhysicsActionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("VehiclePhysicsActionData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct VehicleConfigData {
    pub center_of_mass: super::core::Vec3,
    pub center_of_mass_handling_offset: super::core::Vec3,
    pub inertia_override: super::core::Vec3,
    pub inertia_modifier: super::core::Vec3,
    pub aero_dynamic_physics: AeroDynamicPhysicsData,
    pub motorbike_physics: MotorbikeData,
    pub motion_damping: MotionDampingData,
    pub input: VehicleInputData,
    pub float_physics: FloatPhysicsData,
    pub stabilizer: StabilizerData,
    pub stabilizers: Vec<StabilizerSettings>,
    pub constant_force: Vec<ConstantForceData>,
    pub vehicle_mode_at_reset: VehicleMode,
    pub body_mass: f32,
    pub gravity_modifier: f32,
    pub yaw_min: f32,
    pub yaw_max: f32,
    pub down_force_base_factor: f32,
    pub down_force_wheel_factor: f32,
    pub vehicle_mode_change_entering_time: f32,
    pub vehicle_mode_change_starting_time: f32,
    pub vehicle_mode_change_stopping_time: f32,
    pub vehicle_mode_change_leaving_time: f32,
    pub stand_still_low_speed_time_limit: f32,
    pub static_friction_break_collision_mod: f32,
    pub static_friction_break_velocity_mod: f32,
    pub coefficient_of_air_friction: f32,
    pub air_density: f32,
    pub air_drag_area: f32,
    pub wind_resistance_base_factor: f32,
    pub wind_resistance_velocity_factor: f32,
    pub wind_resistance_velocity_factor_min: f32,
    pub wind_resistance_velocity_factor_max: f32,
    pub use_down_force: bool,
    pub use_down_force_wheel_factor: bool,
    pub use_gearbox: bool,
    pub use_stand_still_brake: bool,
    pub use_stand_still_sleep: bool,
    pub use_turn_around_force: bool,
    pub use_motorcycle_control: bool,
    pub invert_pitch_allowed: bool,
    pub use_wind_resistance: bool,
    pub use_input_yaw_as_throttle: InputThrottle,
    pub anti_roll_bars: AntiRollBars,
    pub max_ground_speed: f32,
    pub proximity_ext_scale: super::core::Vec3,
    pub proximity_height_translation: f32,
    pub friction_at_low_velocity: f32,
    pub contact_material_velocity_threshold: f32,
}

pub const VEHICLECONFIGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleConfigData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "CenterOfMass",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, center_of_mass),
            },
            FieldInfoData {
                name: "CenterOfMassHandlingOffset",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, center_of_mass_handling_offset),
            },
            FieldInfoData {
                name: "InertiaOverride",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, inertia_override),
            },
            FieldInfoData {
                name: "InertiaModifier",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, inertia_modifier),
            },
            FieldInfoData {
                name: "AeroDynamicPhysics",
                flags: MemberInfoFlags::new(0),
                field_type: AERODYNAMICPHYSICSDATA_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, aero_dynamic_physics),
            },
            FieldInfoData {
                name: "MotorbikePhysics",
                flags: MemberInfoFlags::new(0),
                field_type: MOTORBIKEDATA_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, motorbike_physics),
            },
            FieldInfoData {
                name: "MotionDamping",
                flags: MemberInfoFlags::new(0),
                field_type: MOTIONDAMPINGDATA_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, motion_damping),
            },
            FieldInfoData {
                name: "Input",
                flags: MemberInfoFlags::new(0),
                field_type: VEHICLEINPUTDATA_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, input),
            },
            FieldInfoData {
                name: "FloatPhysics",
                flags: MemberInfoFlags::new(0),
                field_type: FLOATPHYSICSDATA_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, float_physics),
            },
            FieldInfoData {
                name: "Stabilizer",
                flags: MemberInfoFlags::new(0),
                field_type: STABILIZERDATA_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, stabilizer),
            },
            FieldInfoData {
                name: "Stabilizers",
                flags: MemberInfoFlags::new(144),
                field_type: STABILIZERSETTINGS_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, stabilizers),
            },
            FieldInfoData {
                name: "ConstantForce",
                flags: MemberInfoFlags::new(144),
                field_type: CONSTANTFORCEDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, constant_force),
            },
            FieldInfoData {
                name: "VehicleModeAtReset",
                flags: MemberInfoFlags::new(0),
                field_type: VEHICLEMODE_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, vehicle_mode_at_reset),
            },
            FieldInfoData {
                name: "BodyMass",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, body_mass),
            },
            FieldInfoData {
                name: "GravityModifier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, gravity_modifier),
            },
            FieldInfoData {
                name: "YawMin",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, yaw_min),
            },
            FieldInfoData {
                name: "YawMax",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, yaw_max),
            },
            FieldInfoData {
                name: "DownForceBaseFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, down_force_base_factor),
            },
            FieldInfoData {
                name: "DownForceWheelFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, down_force_wheel_factor),
            },
            FieldInfoData {
                name: "VehicleModeChangeEnteringTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, vehicle_mode_change_entering_time),
            },
            FieldInfoData {
                name: "VehicleModeChangeStartingTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, vehicle_mode_change_starting_time),
            },
            FieldInfoData {
                name: "VehicleModeChangeStoppingTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, vehicle_mode_change_stopping_time),
            },
            FieldInfoData {
                name: "VehicleModeChangeLeavingTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, vehicle_mode_change_leaving_time),
            },
            FieldInfoData {
                name: "StandStillLowSpeedTimeLimit",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, stand_still_low_speed_time_limit),
            },
            FieldInfoData {
                name: "StaticFrictionBreakCollisionMod",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, static_friction_break_collision_mod),
            },
            FieldInfoData {
                name: "StaticFrictionBreakVelocityMod",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, static_friction_break_velocity_mod),
            },
            FieldInfoData {
                name: "CoefficientOfAirFriction",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, coefficient_of_air_friction),
            },
            FieldInfoData {
                name: "AirDensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, air_density),
            },
            FieldInfoData {
                name: "AirDragArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, air_drag_area),
            },
            FieldInfoData {
                name: "WindResistanceBaseFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, wind_resistance_base_factor),
            },
            FieldInfoData {
                name: "WindResistanceVelocityFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, wind_resistance_velocity_factor),
            },
            FieldInfoData {
                name: "WindResistanceVelocityFactorMin",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, wind_resistance_velocity_factor_min),
            },
            FieldInfoData {
                name: "WindResistanceVelocityFactorMax",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, wind_resistance_velocity_factor_max),
            },
            FieldInfoData {
                name: "UseDownForce",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, use_down_force),
            },
            FieldInfoData {
                name: "UseDownForceWheelFactor",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, use_down_force_wheel_factor),
            },
            FieldInfoData {
                name: "UseGearbox",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, use_gearbox),
            },
            FieldInfoData {
                name: "UseStandStillBrake",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, use_stand_still_brake),
            },
            FieldInfoData {
                name: "UseStandStillSleep",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, use_stand_still_sleep),
            },
            FieldInfoData {
                name: "UseTurnAroundForce",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, use_turn_around_force),
            },
            FieldInfoData {
                name: "UseMotorcycleControl",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, use_motorcycle_control),
            },
            FieldInfoData {
                name: "InvertPitchAllowed",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, invert_pitch_allowed),
            },
            FieldInfoData {
                name: "UseWindResistance",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, use_wind_resistance),
            },
            FieldInfoData {
                name: "UseInputYawAsThrottle",
                flags: MemberInfoFlags::new(0),
                field_type: INPUTTHROTTLE_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, use_input_yaw_as_throttle),
            },
            FieldInfoData {
                name: "AntiRollBars",
                flags: MemberInfoFlags::new(0),
                field_type: ANTIROLLBARS_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, anti_roll_bars),
            },
            FieldInfoData {
                name: "MaxGroundSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, max_ground_speed),
            },
            FieldInfoData {
                name: "ProximityExtScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, proximity_ext_scale),
            },
            FieldInfoData {
                name: "ProximityHeightTranslation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, proximity_height_translation),
            },
            FieldInfoData {
                name: "FrictionAtLowVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, friction_at_low_velocity),
            },
            FieldInfoData {
                name: "ContactMaterialVelocityThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleConfigData, contact_material_velocity_threshold),
            },
        ],
    }),
    array_type: Some(VEHICLECONFIGDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for VehicleConfigData {
    fn type_info() -> &'static TypeInfo {
        VEHICLECONFIGDATA_TYPE_INFO
    }
}


pub const VEHICLECONFIGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleConfigData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("VehicleConfigData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MotorbikeData {
    pub max_leaning_roll: f32,
    pub counter_lean_force: f32,
    pub stand_still_lean_force: f32,
    pub lean_force: f32,
    pub jump_forward_lean_min_angle: f32,
    pub jump_forward_lean_momentum: f32,
    pub jump_forward_lean_min_no_contact_time: f32,
    pub lean_force_max_vel: f32,
    pub kickstand_roll: f32,
    pub kickstand_linear_damping: f32,
    pub stand_still_roll: f32,
    pub damp_big_jump_impact: bool,
    pub damp_big_jump_impact_counter_force: super::core::Vec3,
    pub damp_big_jump_impact_velocity: f32,
    pub damp_big_jump_max_spring_force_fraction: f32,
    pub damp_big_jump_impact_vertical_velocity: f32,
    pub yaw_brake_damping_lerp_start_scale: f32,
    pub yaw_brake_damping_lerp_end_scale: f32,
    pub stoppie_activation_velocity: f32,
    pub stoppie_start_velocity: f32,
    pub stoppie_stop_velocity: f32,
    pub stoppie_momentum: f32,
    pub short_offground_gravity_multiplier: f32,
    pub short_offground_period: f32,
    pub wheelie_switch_off_no_contact_time: f32,
    pub wheelie_max_no_down_force_contact_time: f32,
    pub wheelie_max_no_contact_time: f32,
    pub wheelie_steering_factor: f32,
    pub wheelie_inertia: f32,
    pub wheelie_start_velocity: f32,
    pub wheelie_max_velocity_undamped: f32,
    pub wheelie_max_velocity_damp_range: f32,
    pub wheelie_out_angular_momentum: f32,
    pub wheelie_spring_damping: f32,
    pub wheelie_spring_k_scale: f32,
    pub wheelie_angular_damping: f32,
    pub wheelie_angular_damping_speed: f32,
    pub wheelie_force: super::core::Vec3,
    pub wheelie_force_body_offset: super::core::Vec3,
    pub wheelie_max_angle: f32,
    pub wheelie_max_velocity: f32,
    pub wheelie_velocity_force_scale: f32,
    pub wheelie_angular_start_momentum: f32,
}

pub const MOTORBIKEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MotorbikeData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MaxLeaningRoll",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, max_leaning_roll),
            },
            FieldInfoData {
                name: "CounterLeanForce",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, counter_lean_force),
            },
            FieldInfoData {
                name: "StandStillLeanForce",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, stand_still_lean_force),
            },
            FieldInfoData {
                name: "LeanForce",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, lean_force),
            },
            FieldInfoData {
                name: "JumpForwardLeanMinAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, jump_forward_lean_min_angle),
            },
            FieldInfoData {
                name: "JumpForwardLeanMomentum",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, jump_forward_lean_momentum),
            },
            FieldInfoData {
                name: "JumpForwardLeanMinNoContactTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, jump_forward_lean_min_no_contact_time),
            },
            FieldInfoData {
                name: "LeanForceMaxVel",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, lean_force_max_vel),
            },
            FieldInfoData {
                name: "KickstandRoll",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, kickstand_roll),
            },
            FieldInfoData {
                name: "KickstandLinearDamping",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, kickstand_linear_damping),
            },
            FieldInfoData {
                name: "StandStillRoll",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, stand_still_roll),
            },
            FieldInfoData {
                name: "DampBigJumpImpact",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, damp_big_jump_impact),
            },
            FieldInfoData {
                name: "DampBigJumpImpactCounterForce",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, damp_big_jump_impact_counter_force),
            },
            FieldInfoData {
                name: "DampBigJumpImpactVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, damp_big_jump_impact_velocity),
            },
            FieldInfoData {
                name: "DampBigJumpMaxSpringForceFraction",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, damp_big_jump_max_spring_force_fraction),
            },
            FieldInfoData {
                name: "DampBigJumpImpactVerticalVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, damp_big_jump_impact_vertical_velocity),
            },
            FieldInfoData {
                name: "YawBrakeDampingLerpStartScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, yaw_brake_damping_lerp_start_scale),
            },
            FieldInfoData {
                name: "YawBrakeDampingLerpEndScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, yaw_brake_damping_lerp_end_scale),
            },
            FieldInfoData {
                name: "StoppieActivationVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, stoppie_activation_velocity),
            },
            FieldInfoData {
                name: "StoppieStartVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, stoppie_start_velocity),
            },
            FieldInfoData {
                name: "StoppieStopVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, stoppie_stop_velocity),
            },
            FieldInfoData {
                name: "StoppieMomentum",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, stoppie_momentum),
            },
            FieldInfoData {
                name: "ShortOffgroundGravityMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, short_offground_gravity_multiplier),
            },
            FieldInfoData {
                name: "ShortOffgroundPeriod",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, short_offground_period),
            },
            FieldInfoData {
                name: "WheelieSwitchOffNoContactTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, wheelie_switch_off_no_contact_time),
            },
            FieldInfoData {
                name: "WheelieMaxNoDownForceContactTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, wheelie_max_no_down_force_contact_time),
            },
            FieldInfoData {
                name: "WheelieMaxNoContactTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, wheelie_max_no_contact_time),
            },
            FieldInfoData {
                name: "WheelieSteeringFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, wheelie_steering_factor),
            },
            FieldInfoData {
                name: "WheelieInertia",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, wheelie_inertia),
            },
            FieldInfoData {
                name: "WheelieStartVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, wheelie_start_velocity),
            },
            FieldInfoData {
                name: "WheelieMaxVelocityUndamped",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, wheelie_max_velocity_undamped),
            },
            FieldInfoData {
                name: "WheelieMaxVelocityDampRange",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, wheelie_max_velocity_damp_range),
            },
            FieldInfoData {
                name: "WheelieOutAngularMomentum",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, wheelie_out_angular_momentum),
            },
            FieldInfoData {
                name: "WheelieSpringDamping",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, wheelie_spring_damping),
            },
            FieldInfoData {
                name: "WheelieSpringKScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, wheelie_spring_k_scale),
            },
            FieldInfoData {
                name: "WheelieAngularDamping",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, wheelie_angular_damping),
            },
            FieldInfoData {
                name: "WheelieAngularDampingSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, wheelie_angular_damping_speed),
            },
            FieldInfoData {
                name: "WheelieForce",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, wheelie_force),
            },
            FieldInfoData {
                name: "WheelieForceBodyOffset",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, wheelie_force_body_offset),
            },
            FieldInfoData {
                name: "WheelieMaxAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, wheelie_max_angle),
            },
            FieldInfoData {
                name: "WheelieMaxVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, wheelie_max_velocity),
            },
            FieldInfoData {
                name: "WheelieVelocityForceScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, wheelie_velocity_force_scale),
            },
            FieldInfoData {
                name: "WheelieAngularStartMomentum",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotorbikeData, wheelie_angular_start_momentum),
            },
        ],
    }),
    array_type: Some(MOTORBIKEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for MotorbikeData {
    fn type_info() -> &'static TypeInfo {
        MOTORBIKEDATA_TYPE_INFO
    }
}


pub const MOTORBIKEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MotorbikeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("MotorbikeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct InputThrottle {
    pub enabled: bool,
    pub forward_speed_supression_amount: f32,
    pub backward_speed_supression_amount: f32,
    pub side_speed_supression_amount: f32,
    pub ignore_brake_speed_threshold: f32,
}

pub const INPUTTHROTTLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputThrottle",
    flags: MemberInfoFlags::new(36937),
    module: "Physics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(InputThrottle, enabled),
            },
            FieldInfoData {
                name: "ForwardSpeedSupressionAmount",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(InputThrottle, forward_speed_supression_amount),
            },
            FieldInfoData {
                name: "BackwardSpeedSupressionAmount",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(InputThrottle, backward_speed_supression_amount),
            },
            FieldInfoData {
                name: "SideSpeedSupressionAmount",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(InputThrottle, side_speed_supression_amount),
            },
            FieldInfoData {
                name: "IgnoreBrakeSpeedThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(InputThrottle, ignore_brake_speed_threshold),
            },
        ],
    }),
    array_type: Some(INPUTTHROTTLE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for InputThrottle {
    fn type_info() -> &'static TypeInfo {
        INPUTTHROTTLE_TYPE_INFO
    }
}


pub const INPUTTHROTTLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputThrottle-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("InputThrottle-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ConstantForceData {
    pub condition: ForceCondition,
    pub type_of_force: ForceType,
    pub value: super::core::Vec3,
    pub space: SpaceType,
}

pub const CONSTANTFORCEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConstantForceData",
    flags: MemberInfoFlags::new(36937),
    module: "Physics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Condition",
                flags: MemberInfoFlags::new(0),
                field_type: FORCECONDITION_TYPE_INFO,
                rust_offset: offset_of!(ConstantForceData, condition),
            },
            FieldInfoData {
                name: "TypeOfForce",
                flags: MemberInfoFlags::new(0),
                field_type: FORCETYPE_TYPE_INFO,
                rust_offset: offset_of!(ConstantForceData, type_of_force),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ConstantForceData, value),
            },
            FieldInfoData {
                name: "Space",
                flags: MemberInfoFlags::new(0),
                field_type: SPACETYPE_TYPE_INFO,
                rust_offset: offset_of!(ConstantForceData, space),
            },
        ],
    }),
    array_type: Some(CONSTANTFORCEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ConstantForceData {
    fn type_info() -> &'static TypeInfo {
        CONSTANTFORCEDATA_TYPE_INFO
    }
}


pub const CONSTANTFORCEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConstantForceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ConstantForceData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum SpaceType {
    #[default]
    STBody = 0,
    STWorld = 1,
}

pub const SPACETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpaceType",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(SPACETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SpaceType {
    fn type_info() -> &'static TypeInfo {
        SPACETYPE_TYPE_INFO
    }
}


pub const SPACETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpaceType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("SpaceType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ForceType {
    #[default]
    FTForce = 0,
    FTTorque = 1,
}

pub const FORCETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForceType",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(FORCETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ForceType {
    fn type_info() -> &'static TypeInfo {
        FORCETYPE_TYPE_INFO
    }
}


pub const FORCETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForceType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ForceType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ForceCondition {
    #[default]
    FCNever = 0,
    FCNotCriticalDamaged = 1,
    FCCriticalDamaged = 2,
    FCNotOccupied = 3,
    FCOccupied = 4,
    FCAlways = 5,
}

pub const FORCECONDITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForceCondition",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(FORCECONDITION_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ForceCondition {
    fn type_info() -> &'static TypeInfo {
        FORCECONDITION_TYPE_INFO
    }
}


pub const FORCECONDITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForceCondition-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ForceCondition-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct StabilizerSettings {
    pub property: StabilizerProperty,
    pub strength: f32,
    pub advanced: bool,
    pub radius: f32,
    pub use2_d_radius_test: bool,
    pub use_input_override: bool,
}

pub const STABILIZERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StabilizerSettings",
    flags: MemberInfoFlags::new(36937),
    module: "Physics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Property",
                flags: MemberInfoFlags::new(0),
                field_type: STABILIZERPROPERTY_TYPE_INFO,
                rust_offset: offset_of!(StabilizerSettings, property),
            },
            FieldInfoData {
                name: "Strength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(StabilizerSettings, strength),
            },
            FieldInfoData {
                name: "Advanced",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(StabilizerSettings, advanced),
            },
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(StabilizerSettings, radius),
            },
            FieldInfoData {
                name: "Use2DRadiusTest",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(StabilizerSettings, use2_d_radius_test),
            },
            FieldInfoData {
                name: "UseInputOverride",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(StabilizerSettings, use_input_override),
            },
        ],
    }),
    array_type: Some(STABILIZERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for StabilizerSettings {
    fn type_info() -> &'static TypeInfo {
        STABILIZERSETTINGS_TYPE_INFO
    }
}


pub const STABILIZERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StabilizerSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("StabilizerSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum StabilizerProperty {
    #[default]
    SPPitchAngle = 0,
    SPYawAngle = 1,
    SPRollAngle = 2,
    SPVerticalPosition = 3,
    SPVerticalVelocity = 4,
    SPXPosition = 5,
    SPYPosition = 6,
    SPZPosition = 7,
}

pub const STABILIZERPROPERTY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StabilizerProperty",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(STABILIZERPROPERTY_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for StabilizerProperty {
    fn type_info() -> &'static TypeInfo {
        STABILIZERPROPERTY_TYPE_INFO
    }
}


pub const STABILIZERPROPERTY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StabilizerProperty-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("StabilizerProperty-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AntiRollBars {
    pub front: AntiRollBar,
    pub rear: AntiRollBar,
}

pub const ANTIROLLBARS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntiRollBars",
    flags: MemberInfoFlags::new(73),
    module: "Physics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Front",
                flags: MemberInfoFlags::new(0),
                field_type: ANTIROLLBAR_TYPE_INFO,
                rust_offset: offset_of!(AntiRollBars, front),
            },
            FieldInfoData {
                name: "Rear",
                flags: MemberInfoFlags::new(0),
                field_type: ANTIROLLBAR_TYPE_INFO,
                rust_offset: offset_of!(AntiRollBars, rear),
            },
        ],
    }),
    array_type: Some(ANTIROLLBARS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AntiRollBars {
    fn type_info() -> &'static TypeInfo {
        ANTIROLLBARS_TYPE_INFO
    }
}


pub const ANTIROLLBARS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntiRollBars-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("AntiRollBars-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AntiRollBar {
    pub stiffness: f32,
    pub torque_activation_limit: f32,
}

pub const ANTIROLLBAR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntiRollBar",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Stiffness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AntiRollBar, stiffness),
            },
            FieldInfoData {
                name: "TorqueActivationLimit",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AntiRollBar, torque_activation_limit),
            },
        ],
    }),
    array_type: Some(ANTIROLLBAR_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AntiRollBar {
    fn type_info() -> &'static TypeInfo {
        ANTIROLLBAR_TYPE_INFO
    }
}


pub const ANTIROLLBAR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntiRollBar-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("AntiRollBar-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct VehicleInputData {
    pub throttle_deadzone: f32,
    pub brake_deadzone: f32,
    pub yaw_deadzone: f32,
    pub pitch_deadzone: f32,
    pub roll_deadzone: f32,
    pub throttle_inertia_out_duration: f32,
    pub throttle_inertia_in_duration: f32,
    pub throttle_inertia_min_ratio: f32,
    pub brake_inertia_out_duration: f32,
    pub brake_inertia_in_duration: f32,
    pub brake_inertia_min_ratio: f32,
    pub yaw_inertia_out_duration: f32,
    pub yaw_inertia_in_duration: f32,
    pub yaw_inertia_min_ratio: f32,
    pub pitch_inertia_out_duration: f32,
    pub pitch_inertia_in_duration: f32,
    pub pitch_inertia_min_ratio: f32,
    pub roll_inertia_out_duration: f32,
    pub roll_inertia_in_duration: f32,
    pub roll_inertia_min_ratio: f32,
}

pub const VEHICLEINPUTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleInputData",
    flags: MemberInfoFlags::new(36937),
    module: "Physics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ThrottleDeadzone",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleInputData, throttle_deadzone),
            },
            FieldInfoData {
                name: "BrakeDeadzone",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleInputData, brake_deadzone),
            },
            FieldInfoData {
                name: "YawDeadzone",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleInputData, yaw_deadzone),
            },
            FieldInfoData {
                name: "PitchDeadzone",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleInputData, pitch_deadzone),
            },
            FieldInfoData {
                name: "RollDeadzone",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleInputData, roll_deadzone),
            },
            FieldInfoData {
                name: "ThrottleInertiaOutDuration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleInputData, throttle_inertia_out_duration),
            },
            FieldInfoData {
                name: "ThrottleInertiaInDuration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleInputData, throttle_inertia_in_duration),
            },
            FieldInfoData {
                name: "ThrottleInertiaMinRatio",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleInputData, throttle_inertia_min_ratio),
            },
            FieldInfoData {
                name: "BrakeInertiaOutDuration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleInputData, brake_inertia_out_duration),
            },
            FieldInfoData {
                name: "BrakeInertiaInDuration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleInputData, brake_inertia_in_duration),
            },
            FieldInfoData {
                name: "BrakeInertiaMinRatio",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleInputData, brake_inertia_min_ratio),
            },
            FieldInfoData {
                name: "YawInertiaOutDuration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleInputData, yaw_inertia_out_duration),
            },
            FieldInfoData {
                name: "YawInertiaInDuration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleInputData, yaw_inertia_in_duration),
            },
            FieldInfoData {
                name: "YawInertiaMinRatio",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleInputData, yaw_inertia_min_ratio),
            },
            FieldInfoData {
                name: "PitchInertiaOutDuration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleInputData, pitch_inertia_out_duration),
            },
            FieldInfoData {
                name: "PitchInertiaInDuration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleInputData, pitch_inertia_in_duration),
            },
            FieldInfoData {
                name: "PitchInertiaMinRatio",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleInputData, pitch_inertia_min_ratio),
            },
            FieldInfoData {
                name: "RollInertiaOutDuration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleInputData, roll_inertia_out_duration),
            },
            FieldInfoData {
                name: "RollInertiaInDuration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleInputData, roll_inertia_in_duration),
            },
            FieldInfoData {
                name: "RollInertiaMinRatio",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleInputData, roll_inertia_min_ratio),
            },
        ],
    }),
    array_type: Some(VEHICLEINPUTDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for VehicleInputData {
    fn type_info() -> &'static TypeInfo {
        VEHICLEINPUTDATA_TYPE_INFO
    }
}


pub const VEHICLEINPUTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleInputData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("VehicleInputData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum VehicleMode {
    #[default]
    VmIdle = 0,
    VmEntering = 1,
    VmEntered = 2,
    VmStarting = 3,
    VmStarted = 4,
    VmStopping = 5,
    VmLeaving = 6,
}

pub const VEHICLEMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleMode",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(VEHICLEMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for VehicleMode {
    fn type_info() -> &'static TypeInfo {
        VEHICLEMODE_TYPE_INFO
    }
}


pub const VEHICLEMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("VehicleMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct GearboxNetState {
    pub gearbox_lock_mode: GearboxLockMode,
    pub gearbox_status: GearboxStatus,
    pub gear_status: GearStatus,
    pub clutch_status: ClutchStatus,
    pub gear: i8,
    pub gear_timer: f32,
    pub inverted_timer: f32,
    pub clutch_toggle: bool,
    pub is_gearbox_inverted: bool,
}

pub const GEARBOXNETSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GearboxNetState",
    flags: MemberInfoFlags::new(36937),
    module: "Physics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "GearboxLockMode",
                flags: MemberInfoFlags::new(0),
                field_type: GEARBOXLOCKMODE_TYPE_INFO,
                rust_offset: offset_of!(GearboxNetState, gearbox_lock_mode),
            },
            FieldInfoData {
                name: "GearboxStatus",
                flags: MemberInfoFlags::new(0),
                field_type: GEARBOXSTATUS_TYPE_INFO,
                rust_offset: offset_of!(GearboxNetState, gearbox_status),
            },
            FieldInfoData {
                name: "GearStatus",
                flags: MemberInfoFlags::new(0),
                field_type: GEARSTATUS_TYPE_INFO,
                rust_offset: offset_of!(GearboxNetState, gear_status),
            },
            FieldInfoData {
                name: "ClutchStatus",
                flags: MemberInfoFlags::new(0),
                field_type: CLUTCHSTATUS_TYPE_INFO,
                rust_offset: offset_of!(GearboxNetState, clutch_status),
            },
            FieldInfoData {
                name: "Gear",
                flags: MemberInfoFlags::new(0),
                field_type: INT8_TYPE_INFO,
                rust_offset: offset_of!(GearboxNetState, gear),
            },
            FieldInfoData {
                name: "GearTimer",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GearboxNetState, gear_timer),
            },
            FieldInfoData {
                name: "InvertedTimer",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GearboxNetState, inverted_timer),
            },
            FieldInfoData {
                name: "ClutchToggle",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GearboxNetState, clutch_toggle),
            },
            FieldInfoData {
                name: "IsGearboxInverted",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GearboxNetState, is_gearbox_inverted),
            },
        ],
    }),
    array_type: Some(GEARBOXNETSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for GearboxNetState {
    fn type_info() -> &'static TypeInfo {
        GEARBOXNETSTATE_TYPE_INFO
    }
}


pub const GEARBOXNETSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GearboxNetState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("GearboxNetState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum GearboxGearLimits {
    #[default]
    GearboxGearLimits_Reverse = 4294967288,
    GearboxGearLimits_Forward = 8,
}

pub const GEARBOXGEARLIMITS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GearboxGearLimits",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(GEARBOXGEARLIMITS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for GearboxGearLimits {
    fn type_info() -> &'static TypeInfo {
        GEARBOXGEARLIMITS_TYPE_INFO
    }
}


pub const GEARBOXGEARLIMITS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GearboxGearLimits-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("GearboxGearLimits-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ClutchStatus {
    #[default]
    ClutchStatus_ClutchNoChange = 0,
    ClutchStatus_ClutchDownGearUp = 1,
    ClutchStatus_ClutchDownGearDown = 2,
    ClutchStatus_ClutchUp = 3,
}

pub const CLUTCHSTATUS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClutchStatus",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(CLUTCHSTATUS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ClutchStatus {
    fn type_info() -> &'static TypeInfo {
        CLUTCHSTATUS_TYPE_INFO
    }
}


pub const CLUTCHSTATUS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClutchStatus-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ClutchStatus-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum GearStatus {
    #[default]
    GearStatus_GearNoChange = 0,
    GearStatus_GearDoGearUp = 1,
    GearStatus_GearGearUp = 2,
    GearStatus_GearDoGearDown = 3,
    GearStatus_GearGearDown = 4,
    GearStatus_GearInvertGearbox = 5,
}

pub const GEARSTATUS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GearStatus",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(GEARSTATUS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for GearStatus {
    fn type_info() -> &'static TypeInfo {
        GEARSTATUS_TYPE_INFO
    }
}


pub const GEARSTATUS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GearStatus-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("GearStatus-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum GearboxStatus {
    #[default]
    GearboxStatus_GearboxNoChange = 0,
    GearboxStatus_GearboxClutch = 1,
}

pub const GEARBOXSTATUS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GearboxStatus",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(GEARBOXSTATUS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for GearboxStatus {
    fn type_info() -> &'static TypeInfo {
        GEARBOXSTATUS_TYPE_INFO
    }
}


pub const GEARBOXSTATUS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GearboxStatus-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("GearboxStatus-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum GearboxLockMode {
    #[default]
    GearboxLockMode_NoLock = 0,
    GearboxLockMode_LockForward = 1,
    GearboxLockMode_LockReverse = 2,
}

pub const GEARBOXLOCKMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GearboxLockMode",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(GEARBOXLOCKMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for GearboxLockMode {
    fn type_info() -> &'static TypeInfo {
        GEARBOXLOCKMODE_TYPE_INFO
    }
}


pub const GEARBOXLOCKMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GearboxLockMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("GearboxLockMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum GearboxMode {
    #[default]
    GearboxMode_Direct = 0,
    GearboxMode_Classic = 1,
    GearboxMode_MhStyle = 2,
}

pub const GEARBOXMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GearboxMode",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(GEARBOXMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for GearboxMode {
    fn type_info() -> &'static TypeInfo {
        GEARBOXMODE_TYPE_INFO
    }
}


pub const GEARBOXMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GearboxMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("GearboxMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum GearboxType {
    #[default]
    GearboxType_Manual = 0,
    GearboxType_Automatic = 1,
}

pub const GEARBOXTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GearboxType",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(GEARBOXTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for GearboxType {
    fn type_info() -> &'static TypeInfo {
        GEARBOXTYPE_TYPE_INFO
    }
}


pub const GEARBOXTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GearboxType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("GearboxType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct IglooGrabToolSettings {
    pub max_grab_distance: f32,
    pub debug_pick_point_sphere_radius: f32,
    pub debug_pick_point_normal_length: f32,
    pub debug_grab_hand_square_size: f32,
    pub debug_color_grab_hand: super::core::Vec4,
    pub debug_color_grab_target_position: super::core::Vec4,
    pub debug_color_grab_on_server: super::core::Vec4,
    pub debug_color_grab_on_client: super::core::Vec4,
    pub debug_color_grab_on_effect: super::core::Vec4,
}

pub const IGLOOGRABTOOLSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IglooGrabToolSettings",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MaxGrabDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(IglooGrabToolSettings, max_grab_distance),
            },
            FieldInfoData {
                name: "DebugPickPointSphereRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(IglooGrabToolSettings, debug_pick_point_sphere_radius),
            },
            FieldInfoData {
                name: "DebugPickPointNormalLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(IglooGrabToolSettings, debug_pick_point_normal_length),
            },
            FieldInfoData {
                name: "DebugGrabHandSquareSize",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(IglooGrabToolSettings, debug_grab_hand_square_size),
            },
            FieldInfoData {
                name: "DebugColorGrabHand",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(IglooGrabToolSettings, debug_color_grab_hand),
            },
            FieldInfoData {
                name: "DebugColorGrabTargetPosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(IglooGrabToolSettings, debug_color_grab_target_position),
            },
            FieldInfoData {
                name: "DebugColorGrabOnServer",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(IglooGrabToolSettings, debug_color_grab_on_server),
            },
            FieldInfoData {
                name: "DebugColorGrabOnClient",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(IglooGrabToolSettings, debug_color_grab_on_client),
            },
            FieldInfoData {
                name: "DebugColorGrabOnEffect",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(IglooGrabToolSettings, debug_color_grab_on_effect),
            },
        ],
    }),
    array_type: Some(IGLOOGRABTOOLSETTINGS_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for IglooGrabToolSettings {
    fn type_info() -> &'static TypeInfo {
        IGLOOGRABTOOLSETTINGS_TYPE_INFO
    }
}


pub const IGLOOGRABTOOLSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IglooGrabToolSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("IglooGrabToolSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PhysicsRenderWorldDynamicState {
    pub part_enabled: Vec<bool>,
    pub add_to_world: bool,
    pub character_pose: CharacterPoseType,
    pub transform: super::core::LinearTransform,
    pub field_flag_changed0: u8,
}

pub const PHYSICSRENDERWORLDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRenderWorldDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "Physics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "PartEnabled",
                flags: MemberInfoFlags::new(144),
                field_type: BOOLEAN_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderWorldDynamicState, part_enabled),
            },
            FieldInfoData {
                name: "AddToWorld",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderWorldDynamicState, add_to_world),
            },
            FieldInfoData {
                name: "CharacterPose",
                flags: MemberInfoFlags::new(0),
                field_type: CHARACTERPOSETYPE_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderWorldDynamicState, character_pose),
            },
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderWorldDynamicState, transform),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderWorldDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(PHYSICSRENDERWORLDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsRenderWorldDynamicState {
    fn type_info() -> &'static TypeInfo {
        PHYSICSRENDERWORLDDYNAMICSTATE_TYPE_INFO
    }
}


pub const PHYSICSRENDERWORLDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRenderWorldDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsRenderWorldDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PhysicsRenderWorldStaticState {
    pub physics_component_data: PhysicsComponentData,
    pub physics_body_data: PhysicsBodyData,
    pub body_transform_fixups: Vec<super::core::LinearTransform>,
    pub transform_space: super::state_stream::TransformSpaceHandle,
    pub field_flag_changed0: u8,
}

pub const PHYSICSRENDERWORLDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRenderWorldStaticState",
    flags: MemberInfoFlags::new(73),
    module: "Physics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "physicsComponentData",
                flags: MemberInfoFlags::new(0),
                field_type: PHYSICSCOMPONENTDATA_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderWorldStaticState, physics_component_data),
            },
            FieldInfoData {
                name: "physicsBodyData",
                flags: MemberInfoFlags::new(0),
                field_type: PHYSICSBODYDATA_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderWorldStaticState, physics_body_data),
            },
            FieldInfoData {
                name: "bodyTransformFixups",
                flags: MemberInfoFlags::new(144),
                field_type: LINEARTRANSFORM_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderWorldStaticState, body_transform_fixups),
            },
            FieldInfoData {
                name: "TransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: TRANSFORMSPACEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderWorldStaticState, transform_space),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(PhysicsRenderWorldStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(PHYSICSRENDERWORLDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PhysicsRenderWorldStaticState {
    fn type_info() -> &'static TypeInfo {
        PHYSICSRENDERWORLDSTATICSTATE_TYPE_INFO
    }
}


pub const PHYSICSRENDERWORLDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRenderWorldStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsRenderWorldStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PhysicsRenderWorldHandle {
}

pub const PHYSICSRENDERWORLDHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRenderWorldHandle",
    flags: MemberInfoFlags::new(73),
    module: "Physics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSRENDERWORLDHANDLE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for PhysicsRenderWorldHandle {
    fn type_info() -> &'static TypeInfo {
        PHYSICSRENDERWORLDHANDLE_TYPE_INFO
    }
}


pub const PHYSICSRENDERWORLDHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRenderWorldHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsRenderWorldHandle-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PhysicsCategorySet {
    pub categories: u32,
}

pub const PHYSICSCATEGORYSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsCategorySet",
    flags: MemberInfoFlags::new(36937),
    module: "Physics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Categories",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsCategorySet, categories),
            },
        ],
    }),
    array_type: Some(PHYSICSCATEGORYSET_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for PhysicsCategorySet {
    fn type_info() -> &'static TypeInfo {
        PHYSICSCATEGORYSET_TYPE_INFO
    }
}


pub const PHYSICSCATEGORYSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsCategorySet-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsCategorySet-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FBPhysicsWorldData {
    pub max_body_count: u32,
    pub max_heightfield_count: u32,
    pub max_non_fixed_shape_count: u32,
    pub max_fixed_shape_count: u32,
    pub max_shape_pairs: u32,
    pub max_shape_pair_batches: u32,
    pub shape_pair_index_count: u32,
    pub max_contact_count: u32,
    pub max_ray_query_count: u32,
    pub max_ray_query_result_count: u32,
    pub max_directional_drive_count: u32,
    pub max_temporary_directional_drive_count: u32,
    pub max_locked_linear_joint_count: u32,
    pub max_temporary_locked_linear_joint_count: u32,
    pub max_angular_joint_count: u32,
    pub max_temporary_angular_joint_count: u32,
    pub max_vertex_count: u32,
    pub max_edge_count: u32,
    pub max_fixed_partition_count: u32,
    pub max_collision_report_count: u32,
    pub max_immediate_query_count: u32,
    pub max_results_per_immediate_query: u32,
    pub max_culling_tables: u32,
    pub max_culling_group_pairs: u32,
    pub max_culling_table_size: u32,
    pub gravity: super::core::Vec3,
    pub max_g_j_k_iterations: u32,
    pub max_solve_iterations: u32,
}

pub const FBPHYSICSWORLDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBPhysicsWorldData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SUBWORLDDATACOMPONENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MaxBodyCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsWorldData, max_body_count),
            },
            FieldInfoData {
                name: "MaxHeightfieldCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsWorldData, max_heightfield_count),
            },
            FieldInfoData {
                name: "MaxNonFixedShapeCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsWorldData, max_non_fixed_shape_count),
            },
            FieldInfoData {
                name: "MaxFixedShapeCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsWorldData, max_fixed_shape_count),
            },
            FieldInfoData {
                name: "MaxShapePairs",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsWorldData, max_shape_pairs),
            },
            FieldInfoData {
                name: "MaxShapePairBatches",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsWorldData, max_shape_pair_batches),
            },
            FieldInfoData {
                name: "ShapePairIndexCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsWorldData, shape_pair_index_count),
            },
            FieldInfoData {
                name: "MaxContactCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsWorldData, max_contact_count),
            },
            FieldInfoData {
                name: "MaxRayQueryCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsWorldData, max_ray_query_count),
            },
            FieldInfoData {
                name: "MaxRayQueryResultCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsWorldData, max_ray_query_result_count),
            },
            FieldInfoData {
                name: "MaxDirectionalDriveCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsWorldData, max_directional_drive_count),
            },
            FieldInfoData {
                name: "MaxTemporaryDirectionalDriveCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsWorldData, max_temporary_directional_drive_count),
            },
            FieldInfoData {
                name: "MaxLockedLinearJointCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsWorldData, max_locked_linear_joint_count),
            },
            FieldInfoData {
                name: "MaxTemporaryLockedLinearJointCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsWorldData, max_temporary_locked_linear_joint_count),
            },
            FieldInfoData {
                name: "MaxAngularJointCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsWorldData, max_angular_joint_count),
            },
            FieldInfoData {
                name: "MaxTemporaryAngularJointCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsWorldData, max_temporary_angular_joint_count),
            },
            FieldInfoData {
                name: "MaxVertexCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsWorldData, max_vertex_count),
            },
            FieldInfoData {
                name: "MaxEdgeCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsWorldData, max_edge_count),
            },
            FieldInfoData {
                name: "MaxFixedPartitionCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsWorldData, max_fixed_partition_count),
            },
            FieldInfoData {
                name: "MaxCollisionReportCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsWorldData, max_collision_report_count),
            },
            FieldInfoData {
                name: "MaxImmediateQueryCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsWorldData, max_immediate_query_count),
            },
            FieldInfoData {
                name: "MaxResultsPerImmediateQuery",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsWorldData, max_results_per_immediate_query),
            },
            FieldInfoData {
                name: "MaxCullingTables",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsWorldData, max_culling_tables),
            },
            FieldInfoData {
                name: "MaxCullingGroupPairs",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsWorldData, max_culling_group_pairs),
            },
            FieldInfoData {
                name: "MaxCullingTableSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsWorldData, max_culling_table_size),
            },
            FieldInfoData {
                name: "Gravity",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsWorldData, gravity),
            },
            FieldInfoData {
                name: "MaxGJKIterations",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsWorldData, max_g_j_k_iterations),
            },
            FieldInfoData {
                name: "MaxSolveIterations",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsWorldData, max_solve_iterations),
            },
        ],
    }),
    array_type: Some(FBPHYSICSWORLDDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FBPhysicsWorldData {
    fn type_info() -> &'static TypeInfo {
        FBPHYSICSWORLDDATA_TYPE_INFO
    }
}


pub const FBPHYSICSWORLDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBPhysicsWorldData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("FBPhysicsWorldData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FBPhysicsSubLevelData {
    pub realm: super::core::Realm,
    pub server_fixed_shape_count: u32,
    pub client_fixed_shape_count: u32,
}

pub const FBPHYSICSSUBLEVELDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBPhysicsSubLevelData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsSubLevelData, realm),
            },
            FieldInfoData {
                name: "ServerFixedShapeCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsSubLevelData, server_fixed_shape_count),
            },
            FieldInfoData {
                name: "ClientFixedShapeCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsSubLevelData, client_fixed_shape_count),
            },
        ],
    }),
    array_type: Some(FBPHYSICSSUBLEVELDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FBPhysicsSubLevelData {
    fn type_info() -> &'static TypeInfo {
        FBPHYSICSSUBLEVELDATA_TYPE_INFO
    }
}


pub const FBPHYSICSSUBLEVELDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBPhysicsSubLevelData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("FBPhysicsSubLevelData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum PhysicsCollisionCategory {
    #[default]
    PhysicsCollisionCategory_Terrain = 0,
    PhysicsCollisionCategory_Water = 1,
    PhysicsCollisionCategory_Character = 2,
    PhysicsCollisionCategory_Ragdoll = 3,
    PhysicsCollisionCategory_FastDebris = 4,
    PhysicsCollisionCategory_Detail = 5,
    PhysicsCollisionCategory_CharacterCollision = 6,
    PhysicsCollisionCategory_FastDebrisCollision = 7,
    PhysicsCollisionCategory_GameplayCollision = 8,
    PhysicsCollisionCategory_Collision = 9,
    PhysicsCollisionCategory_Audio = 10,
    PhysicsCollisionCategory_Camera = 11,
    PhysicsCollisionCategory_WaterDisturbances = 12,
    PhysicsCollisionCategory_Count = 13,
}

pub const PHYSICSCOLLISIONCATEGORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsCollisionCategory",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(PHYSICSCOLLISIONCATEGORY_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PhysicsCollisionCategory {
    fn type_info() -> &'static TypeInfo {
        PHYSICSCOLLISIONCATEGORY_TYPE_INFO
    }
}


pub const PHYSICSCOLLISIONCATEGORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsCollisionCategory-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsCollisionCategory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FBPhysicsComponentData {
    pub realm: super::core::Realm,
    pub bones: Vec<PhysicsBoneData>,
    pub asset: PhysicsAsset,
}

pub const FBPHYSICSCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBPhysicsComponentData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsComponentData, realm),
            },
            FieldInfoData {
                name: "Bones",
                flags: MemberInfoFlags::new(144),
                field_type: PHYSICSBONEDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsComponentData, bones),
            },
            FieldInfoData {
                name: "Asset",
                flags: MemberInfoFlags::new(0),
                field_type: PHYSICSASSET_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsComponentData, asset),
            },
        ],
    }),
    array_type: Some(FBPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FBPhysicsComponentData {
    fn type_info() -> &'static TypeInfo {
        FBPHYSICSCOMPONENTDATA_TYPE_INFO
    }
}


pub const FBPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBPhysicsComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("FBPhysicsComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PhysicsAsset {
    pub realm: super::core::Realm,
    pub resource: super::core::ResourceRef,
    pub locked_linear_joints: Vec<FBPhysicsLockedLinearJointData>,
    pub angular_joints: Vec<FBPhysicsAngularJointData>,
    pub directional_drives: Vec<FBPhysicsDirectionalDriveData>,
    pub culling_tables: Vec<PhysicsCullingTableData>,
}

pub const PHYSICSASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsAsset",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(PhysicsAsset, realm),
            },
            FieldInfoData {
                name: "Resource",
                flags: MemberInfoFlags::new(0),
                field_type: RESOURCEREF_TYPE_INFO,
                rust_offset: offset_of!(PhysicsAsset, resource),
            },
            FieldInfoData {
                name: "LockedLinearJoints",
                flags: MemberInfoFlags::new(144),
                field_type: FBPHYSICSLOCKEDLINEARJOINTDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(PhysicsAsset, locked_linear_joints),
            },
            FieldInfoData {
                name: "AngularJoints",
                flags: MemberInfoFlags::new(144),
                field_type: FBPHYSICSANGULARJOINTDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(PhysicsAsset, angular_joints),
            },
            FieldInfoData {
                name: "DirectionalDrives",
                flags: MemberInfoFlags::new(144),
                field_type: FBPHYSICSDIRECTIONALDRIVEDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(PhysicsAsset, directional_drives),
            },
            FieldInfoData {
                name: "CullingTables",
                flags: MemberInfoFlags::new(144),
                field_type: PHYSICSCULLINGTABLEDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(PhysicsAsset, culling_tables),
            },
        ],
    }),
    array_type: Some(PHYSICSASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PhysicsAsset {
    fn type_info() -> &'static TypeInfo {
        PHYSICSASSET_TYPE_INFO
    }
}


pub const PHYSICSASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PhysicsCullingTableData {
    pub entries: Vec<bool>,
    pub group_a_size: u32,
    pub group_b_size: u32,
}

pub const PHYSICSCULLINGTABLEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsCullingTableData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Entries",
                flags: MemberInfoFlags::new(144),
                field_type: BOOLEAN_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(PhysicsCullingTableData, entries),
            },
            FieldInfoData {
                name: "GroupASize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsCullingTableData, group_a_size),
            },
            FieldInfoData {
                name: "GroupBSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsCullingTableData, group_b_size),
            },
        ],
    }),
    array_type: Some(PHYSICSCULLINGTABLEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PhysicsCullingTableData {
    fn type_info() -> &'static TypeInfo {
        PHYSICSCULLINGTABLEDATA_TYPE_INFO
    }
}


pub const PHYSICSCULLINGTABLEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsCullingTableData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsCullingTableData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FBPhysicsShapeData {
    pub convex_hull_index: u32,
    pub position: super::core::Vec3,
    pub orientation: super::core::Quat,
    pub shape_type: PhysicsShapeType,
    pub radius: f32,
    pub body_index_in_physics_component: u32,
    pub part_index: u32,
    pub in_categories: PhysicsCategorySet,
    pub collides_with_categories: PhysicsCategorySet,
    pub material: super::entity::MaterialDecl,
}

pub const FBPHYSICSSHAPEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBPhysicsShapeData",
    flags: MemberInfoFlags::new(32841),
    module: "Physics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ConvexHullIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsShapeData, convex_hull_index),
            },
            FieldInfoData {
                name: "Position",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsShapeData, position),
            },
            FieldInfoData {
                name: "Orientation",
                flags: MemberInfoFlags::new(0),
                field_type: QUAT_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsShapeData, orientation),
            },
            FieldInfoData {
                name: "ShapeType",
                flags: MemberInfoFlags::new(0),
                field_type: PHYSICSSHAPETYPE_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsShapeData, shape_type),
            },
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsShapeData, radius),
            },
            FieldInfoData {
                name: "BodyIndexInPhysicsComponent",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsShapeData, body_index_in_physics_component),
            },
            FieldInfoData {
                name: "PartIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsShapeData, part_index),
            },
            FieldInfoData {
                name: "InCategories",
                flags: MemberInfoFlags::new(0),
                field_type: PHYSICSCATEGORYSET_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsShapeData, in_categories),
            },
            FieldInfoData {
                name: "CollidesWithCategories",
                flags: MemberInfoFlags::new(0),
                field_type: PHYSICSCATEGORYSET_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsShapeData, collides_with_categories),
            },
            FieldInfoData {
                name: "Material",
                flags: MemberInfoFlags::new(0),
                field_type: MATERIALDECL_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsShapeData, material),
            },
        ],
    }),
    array_type: Some(FBPHYSICSSHAPEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FBPhysicsShapeData {
    fn type_info() -> &'static TypeInfo {
        FBPHYSICSSHAPEDATA_TYPE_INFO
    }
}


pub const FBPHYSICSSHAPEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBPhysicsShapeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("FBPhysicsShapeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FBPhysicsConvexHullData {
    pub positions: Vec<super::core::Vec3>,
    pub end_vertices: Vec<u32>,
    pub edge_cosines: Vec<f32>,
    pub next_edge_in_faces: Vec<u32>,
}

pub const FBPHYSICSCONVEXHULLDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBPhysicsConvexHullData",
    flags: MemberInfoFlags::new(73),
    module: "Physics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Positions",
                flags: MemberInfoFlags::new(144),
                field_type: VEC3_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsConvexHullData, positions),
            },
            FieldInfoData {
                name: "EndVertices",
                flags: MemberInfoFlags::new(144),
                field_type: UINT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsConvexHullData, end_vertices),
            },
            FieldInfoData {
                name: "EdgeCosines",
                flags: MemberInfoFlags::new(144),
                field_type: FLOAT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsConvexHullData, edge_cosines),
            },
            FieldInfoData {
                name: "NextEdgeInFaces",
                flags: MemberInfoFlags::new(144),
                field_type: UINT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsConvexHullData, next_edge_in_faces),
            },
        ],
    }),
    array_type: Some(FBPHYSICSCONVEXHULLDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FBPhysicsConvexHullData {
    fn type_info() -> &'static TypeInfo {
        FBPHYSICSCONVEXHULLDATA_TYPE_INFO
    }
}


pub const FBPHYSICSCONVEXHULLDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBPhysicsConvexHullData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("FBPhysicsConvexHullData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FBPhysicsDirectionalDriveData {
    pub body_a_index_in_component: u32,
    pub body_b_index_in_component: u32,
    pub position_on_body_a: super::core::Vec3,
    pub position_on_body_b: super::core::Vec3,
    pub orientation_on_body_a: super::core::Quat,
    pub orientation_on_body_b: super::core::Quat,
    pub angular_spring: f32,
    pub angular_damping: f32,
    pub linear_spring: f32,
    pub linear_damping: f32,
    pub angular_limit_axis_space: PhysicsConstraintLimitSpace,
    pub angular_limit_axis: super::core::Vec3,
    pub swing_torque_limit: f32,
    pub twist_torque_limit: f32,
    pub linear_limit_axis_space: PhysicsConstraintLimitSpace,
    pub linear_limit_axis: super::core::Vec3,
    pub forward_force_limit: f32,
    pub backward_force_limit: f32,
    pub radial_force_limit: f32,
}

pub const FBPHYSICSDIRECTIONALDRIVEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBPhysicsDirectionalDriveData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "BodyAIndexInComponent",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsDirectionalDriveData, body_a_index_in_component),
            },
            FieldInfoData {
                name: "BodyBIndexInComponent",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsDirectionalDriveData, body_b_index_in_component),
            },
            FieldInfoData {
                name: "PositionOnBodyA",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsDirectionalDriveData, position_on_body_a),
            },
            FieldInfoData {
                name: "PositionOnBodyB",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsDirectionalDriveData, position_on_body_b),
            },
            FieldInfoData {
                name: "OrientationOnBodyA",
                flags: MemberInfoFlags::new(0),
                field_type: QUAT_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsDirectionalDriveData, orientation_on_body_a),
            },
            FieldInfoData {
                name: "OrientationOnBodyB",
                flags: MemberInfoFlags::new(0),
                field_type: QUAT_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsDirectionalDriveData, orientation_on_body_b),
            },
            FieldInfoData {
                name: "AngularSpring",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsDirectionalDriveData, angular_spring),
            },
            FieldInfoData {
                name: "AngularDamping",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsDirectionalDriveData, angular_damping),
            },
            FieldInfoData {
                name: "LinearSpring",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsDirectionalDriveData, linear_spring),
            },
            FieldInfoData {
                name: "LinearDamping",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsDirectionalDriveData, linear_damping),
            },
            FieldInfoData {
                name: "AngularLimitAxisSpace",
                flags: MemberInfoFlags::new(0),
                field_type: PHYSICSCONSTRAINTLIMITSPACE_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsDirectionalDriveData, angular_limit_axis_space),
            },
            FieldInfoData {
                name: "AngularLimitAxis",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsDirectionalDriveData, angular_limit_axis),
            },
            FieldInfoData {
                name: "SwingTorqueLimit",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsDirectionalDriveData, swing_torque_limit),
            },
            FieldInfoData {
                name: "TwistTorqueLimit",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsDirectionalDriveData, twist_torque_limit),
            },
            FieldInfoData {
                name: "LinearLimitAxisSpace",
                flags: MemberInfoFlags::new(0),
                field_type: PHYSICSCONSTRAINTLIMITSPACE_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsDirectionalDriveData, linear_limit_axis_space),
            },
            FieldInfoData {
                name: "LinearLimitAxis",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsDirectionalDriveData, linear_limit_axis),
            },
            FieldInfoData {
                name: "ForwardForceLimit",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsDirectionalDriveData, forward_force_limit),
            },
            FieldInfoData {
                name: "BackwardForceLimit",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsDirectionalDriveData, backward_force_limit),
            },
            FieldInfoData {
                name: "RadialForceLimit",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsDirectionalDriveData, radial_force_limit),
            },
        ],
    }),
    array_type: Some(FBPHYSICSDIRECTIONALDRIVEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FBPhysicsDirectionalDriveData {
    fn type_info() -> &'static TypeInfo {
        FBPHYSICSDIRECTIONALDRIVEDATA_TYPE_INFO
    }
}


pub const FBPHYSICSDIRECTIONALDRIVEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBPhysicsDirectionalDriveData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("FBPhysicsDirectionalDriveData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FBPhysicsAngularJointData {
    pub body_a_index_in_component: u32,
    pub body_b_index_in_component: u32,
    pub swing_spring: f32,
    pub swing_damping: f32,
    pub twist_spring: f32,
    pub twist_damping: f32,
    pub swing_limit_y: f32,
    pub swing_limit_z: f32,
    pub twist_limit: f32,
    pub soft_swing_limit_y: f32,
    pub soft_swing_limit_z: f32,
    pub soft_twist_limit: f32,
    pub swing_velocity_limit: f32,
    pub twist_velocity_limit: f32,
    pub orientation_on_body_a: super::core::Quat,
    pub orientation_on_body_b: super::core::Quat,
    pub swing_type: PhysicsAngularJointSwingType,
    pub twist_type: PhysicsAngularJointTwistType,
}

pub const FBPHYSICSANGULARJOINTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBPhysicsAngularJointData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "BodyAIndexInComponent",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsAngularJointData, body_a_index_in_component),
            },
            FieldInfoData {
                name: "BodyBIndexInComponent",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsAngularJointData, body_b_index_in_component),
            },
            FieldInfoData {
                name: "SwingSpring",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsAngularJointData, swing_spring),
            },
            FieldInfoData {
                name: "SwingDamping",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsAngularJointData, swing_damping),
            },
            FieldInfoData {
                name: "TwistSpring",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsAngularJointData, twist_spring),
            },
            FieldInfoData {
                name: "TwistDamping",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsAngularJointData, twist_damping),
            },
            FieldInfoData {
                name: "SwingLimitY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsAngularJointData, swing_limit_y),
            },
            FieldInfoData {
                name: "SwingLimitZ",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsAngularJointData, swing_limit_z),
            },
            FieldInfoData {
                name: "TwistLimit",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsAngularJointData, twist_limit),
            },
            FieldInfoData {
                name: "SoftSwingLimitY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsAngularJointData, soft_swing_limit_y),
            },
            FieldInfoData {
                name: "SoftSwingLimitZ",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsAngularJointData, soft_swing_limit_z),
            },
            FieldInfoData {
                name: "SoftTwistLimit",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsAngularJointData, soft_twist_limit),
            },
            FieldInfoData {
                name: "SwingVelocityLimit",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsAngularJointData, swing_velocity_limit),
            },
            FieldInfoData {
                name: "TwistVelocityLimit",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsAngularJointData, twist_velocity_limit),
            },
            FieldInfoData {
                name: "OrientationOnBodyA",
                flags: MemberInfoFlags::new(0),
                field_type: QUAT_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsAngularJointData, orientation_on_body_a),
            },
            FieldInfoData {
                name: "OrientationOnBodyB",
                flags: MemberInfoFlags::new(0),
                field_type: QUAT_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsAngularJointData, orientation_on_body_b),
            },
            FieldInfoData {
                name: "SwingType",
                flags: MemberInfoFlags::new(0),
                field_type: PHYSICSANGULARJOINTSWINGTYPE_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsAngularJointData, swing_type),
            },
            FieldInfoData {
                name: "TwistType",
                flags: MemberInfoFlags::new(0),
                field_type: PHYSICSANGULARJOINTTWISTTYPE_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsAngularJointData, twist_type),
            },
        ],
    }),
    array_type: Some(FBPHYSICSANGULARJOINTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FBPhysicsAngularJointData {
    fn type_info() -> &'static TypeInfo {
        FBPHYSICSANGULARJOINTDATA_TYPE_INFO
    }
}


pub const FBPHYSICSANGULARJOINTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBPhysicsAngularJointData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("FBPhysicsAngularJointData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FBPhysicsLockedLinearJointData {
    pub body_a_index_in_component: u32,
    pub body_b_index_in_component: u32,
    pub position_on_body_a: super::core::Vec3,
    pub position_on_body_b: super::core::Vec3,
}

pub const FBPHYSICSLOCKEDLINEARJOINTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBPhysicsLockedLinearJointData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "BodyAIndexInComponent",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsLockedLinearJointData, body_a_index_in_component),
            },
            FieldInfoData {
                name: "BodyBIndexInComponent",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsLockedLinearJointData, body_b_index_in_component),
            },
            FieldInfoData {
                name: "PositionOnBodyA",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsLockedLinearJointData, position_on_body_a),
            },
            FieldInfoData {
                name: "PositionOnBodyB",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsLockedLinearJointData, position_on_body_b),
            },
        ],
    }),
    array_type: Some(FBPHYSICSLOCKEDLINEARJOINTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FBPhysicsLockedLinearJointData {
    fn type_info() -> &'static TypeInfo {
        FBPHYSICSLOCKEDLINEARJOINTDATA_TYPE_INFO
    }
}


pub const FBPHYSICSLOCKEDLINEARJOINTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBPhysicsLockedLinearJointData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("FBPhysicsLockedLinearJointData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FBPhysicsBodyData {
    pub motion_type: RigidBodyMotionType,
    pub reciprocal_mass: f32,
    pub principal_axes_of_inertia: super::core::Quat,
    pub unit_mass_inverse_inertia: super::core::Vec3,
    pub center_of_mass_offset: super::core::Vec3,
}

pub const FBPHYSICSBODYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBPhysicsBodyData",
    flags: MemberInfoFlags::new(32841),
    module: "Physics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "MotionType",
                flags: MemberInfoFlags::new(0),
                field_type: RIGIDBODYMOTIONTYPE_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsBodyData, motion_type),
            },
            FieldInfoData {
                name: "ReciprocalMass",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsBodyData, reciprocal_mass),
            },
            FieldInfoData {
                name: "PrincipalAxesOfInertia",
                flags: MemberInfoFlags::new(0),
                field_type: QUAT_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsBodyData, principal_axes_of_inertia),
            },
            FieldInfoData {
                name: "UnitMassInverseInertia",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsBodyData, unit_mass_inverse_inertia),
            },
            FieldInfoData {
                name: "CenterOfMassOffset",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(FBPhysicsBodyData, center_of_mass_offset),
            },
        ],
    }),
    array_type: Some(FBPHYSICSBODYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FBPhysicsBodyData {
    fn type_info() -> &'static TypeInfo {
        FBPHYSICSBODYDATA_TYPE_INFO
    }
}


pub const FBPHYSICSBODYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBPhysicsBodyData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("FBPhysicsBodyData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PhysicsBoneData {
    pub bone: super::entity::ComponentData,
    pub body_index: u32,
}

pub const PHYSICSBONEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsBoneData",
    flags: MemberInfoFlags::new(73),
    module: "Physics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Bone",
                flags: MemberInfoFlags::new(0),
                field_type: COMPONENTDATA_TYPE_INFO,
                rust_offset: offset_of!(PhysicsBoneData, bone),
            },
            FieldInfoData {
                name: "BodyIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsBoneData, body_index),
            },
        ],
    }),
    array_type: Some(PHYSICSBONEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PhysicsBoneData {
    fn type_info() -> &'static TypeInfo {
        PHYSICSBONEDATA_TYPE_INFO
    }
}


pub const PHYSICSBONEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsBoneData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsBoneData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PhysicsTriangleArrayData {
    pub triangles: Vec<PhysicsTriangle>,
}

pub const PHYSICSTRIANGLEARRAYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsTriangleArrayData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Triangles",
                flags: MemberInfoFlags::new(144),
                field_type: PHYSICSTRIANGLE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(PhysicsTriangleArrayData, triangles),
            },
        ],
    }),
    array_type: Some(PHYSICSTRIANGLEARRAYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PhysicsTriangleArrayData {
    fn type_info() -> &'static TypeInfo {
        PHYSICSTRIANGLEARRAYDATA_TYPE_INFO
    }
}


pub const PHYSICSTRIANGLEARRAYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsTriangleArrayData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsTriangleArrayData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PhysicsTriangle {
    pub p0: super::core::Vec3,
    pub p1: super::core::Vec3,
    pub p2: super::core::Vec3,
}

pub const PHYSICSTRIANGLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsTriangle",
    flags: MemberInfoFlags::new(36937),
    module: "Physics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "P0",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PhysicsTriangle, p0),
            },
            FieldInfoData {
                name: "P1",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PhysicsTriangle, p1),
            },
            FieldInfoData {
                name: "P2",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PhysicsTriangle, p2),
            },
        ],
    }),
    array_type: Some(PHYSICSTRIANGLE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsTriangle {
    fn type_info() -> &'static TypeInfo {
        PHYSICSTRIANGLE_TYPE_INFO
    }
}


pub const PHYSICSTRIANGLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsTriangle-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsTriangle-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum PhysicsConstraintLimitSpace {
    #[default]
    PhysicsConstraintLimitSpace_World = 0,
    PhysicsConstraintLimitSpace_BodyA = 1,
    PhysicsConstraintLimitSpace_BodyB = 2,
}

pub const PHYSICSCONSTRAINTLIMITSPACE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsConstraintLimitSpace",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(PHYSICSCONSTRAINTLIMITSPACE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PhysicsConstraintLimitSpace {
    fn type_info() -> &'static TypeInfo {
        PHYSICSCONSTRAINTLIMITSPACE_TYPE_INFO
    }
}


pub const PHYSICSCONSTRAINTLIMITSPACE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsConstraintLimitSpace-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsConstraintLimitSpace-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum PhysicsAngularJointSwingType {
    #[default]
    PhysicsAngularJointSwing_Locked = 0,
    PhysicsAngularJointSwing_Cone = 1,
    PhysicsAngularJointSwing_Hinge = 2,
    PhysicsAngularJointSwing_Axle = 3,
    PhysicsAngularJointSwing_Free = 4,
    PhysicsAngularJointSwing_Ellipse = 5,
}

pub const PHYSICSANGULARJOINTSWINGTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsAngularJointSwingType",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(PHYSICSANGULARJOINTSWINGTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PhysicsAngularJointSwingType {
    fn type_info() -> &'static TypeInfo {
        PHYSICSANGULARJOINTSWINGTYPE_TYPE_INFO
    }
}


pub const PHYSICSANGULARJOINTSWINGTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsAngularJointSwingType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsAngularJointSwingType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum PhysicsAngularJointTwistType {
    #[default]
    PhysicsAngularJointTwist_Locked = 0,
    PhysicsAngularJointTwist_Arc = 1,
    PhysicsAngularJointTwist_Free = 2,
}

pub const PHYSICSANGULARJOINTTWISTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsAngularJointTwistType",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(PHYSICSANGULARJOINTTWISTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PhysicsAngularJointTwistType {
    fn type_info() -> &'static TypeInfo {
        PHYSICSANGULARJOINTTWISTTYPE_TYPE_INFO
    }
}


pub const PHYSICSANGULARJOINTTWISTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsAngularJointTwistType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsAngularJointTwistType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum PhysicsShapeType {
    #[default]
    PhysicsShapeType_Sphere = 0,
    PhysicsShapeType_Capsule = 1,
    PhysicsShapeType_Box = 2,
    PhysicsShapeType_ConvexHull = 3,
    PhysicsShapeType_Cylinder = 4,
    PhysicsShapeType_Heightfield = 5,
}

pub const PHYSICSSHAPETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsShapeType",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(PHYSICSSHAPETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PhysicsShapeType {
    fn type_info() -> &'static TypeInfo {
        PHYSICSSHAPETYPE_TYPE_INFO
    }
}


pub const PHYSICSSHAPETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsShapeType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsShapeType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum PhysicsComponentConstants {
    #[default]
    PhysicsComponentConstants_UseWorldBody = 2147483647,
}

pub const PHYSICSCOMPONENTCONSTANTS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsComponentConstants",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(PHYSICSCOMPONENTCONSTANTS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PhysicsComponentConstants {
    fn type_info() -> &'static TypeInfo {
        PHYSICSCOMPONENTCONSTANTS_TYPE_INFO
    }
}


pub const PHYSICSCOMPONENTCONSTANTS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsComponentConstants-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsComponentConstants-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HavokMemoryContext {
}

pub const HAVOKMEMORYCONTEXT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HavokMemoryContext",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IGLOOMEMORYCONTEXT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(HAVOKMEMORYCONTEXT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for HavokMemoryContext {
    fn type_info() -> &'static TypeInfo {
        HAVOKMEMORYCONTEXT_TYPE_INFO
    }
}


pub const HAVOKMEMORYCONTEXT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HavokMemoryContext-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("HavokMemoryContext-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WindPhysicsAction {
}

pub const WINDPHYSICSACTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindPhysicsAction",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSACTION_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WINDPHYSICSACTION_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for WindPhysicsAction {
    fn type_info() -> &'static TypeInfo {
        WINDPHYSICSACTION_TYPE_INFO
    }
}


pub const WINDPHYSICSACTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindPhysicsAction-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("WindPhysicsAction-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientDestructionInfo {
}

pub const CLIENTDESTRUCTIONINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDestructionInfo",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DESTRUCTIONINFO_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTDESTRUCTIONINFO_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientDestructionInfo {
    fn type_info() -> &'static TypeInfo {
        CLIENTDESTRUCTIONINFO_TYPE_INFO
    }
}


pub const CLIENTDESTRUCTIONINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDestructionInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ClientDestructionInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerDestructionInfo {
}

pub const SERVERDESTRUCTIONINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDestructionInfo",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DESTRUCTIONINFO_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERDESTRUCTIONINFO_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerDestructionInfo {
    fn type_info() -> &'static TypeInfo {
        SERVERDESTRUCTIONINFO_TYPE_INFO
    }
}


pub const SERVERDESTRUCTIONINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDestructionInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ServerDestructionInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DestructionInfo {
}

pub const DESTRUCTIONINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionInfo",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(DESTRUCTIONINFO_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DestructionInfo {
    fn type_info() -> &'static TypeInfo {
        DESTRUCTIONINFO_TYPE_INFO
    }
}


pub const DESTRUCTIONINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("DestructionInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DestructionPhysicsComponent {
}

pub const DESTRUCTIONPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DESTRUCTIONPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DestructionPhysicsComponent {
    fn type_info() -> &'static TypeInfo {
        DESTRUCTIONPHYSICSCOMPONENT_TYPE_INFO
    }
}


pub const DESTRUCTIONPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("DestructionPhysicsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DestructionEntityFactory {
}

pub const DESTRUCTIONENTITYFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionEntityFactory",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IGLOOENTITYFACTORY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DESTRUCTIONENTITYFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DestructionEntityFactory {
    fn type_info() -> &'static TypeInfo {
        DESTRUCTIONENTITYFACTORY_TYPE_INFO
    }
}


pub const DESTRUCTIONENTITYFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionEntityFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("DestructionEntityFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IglooSubsystem {
}

pub const IGLOOSUBSYSTEM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IglooSubsystem",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(IGLOOSUBSYSTEM_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IglooSubsystem {
    fn type_info() -> &'static TypeInfo {
        IGLOOSUBSYSTEM_TYPE_INFO
    }
}


pub const IGLOOSUBSYSTEM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IglooSubsystem-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("IglooSubsystem-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PhysicsProxyEntity {
}

pub const PHYSICSPROXYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsProxyEntity",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENTENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSPROXYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PhysicsProxyEntity {
    fn type_info() -> &'static TypeInfo {
        PHYSICSPROXYENTITY_TYPE_INFO
    }
}


pub const PHYSICSPROXYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsProxyEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsProxyEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PhysicsComponent {
}

pub const PHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PhysicsComponent {
    fn type_info() -> &'static TypeInfo {
        PHYSICSCOMPONENT_TYPE_INFO
    }
}


pub const PHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MultiCollisionEvent {
}

pub const MULTICOLLISIONEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiCollisionEvent",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYEVENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(MULTICOLLISIONEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MultiCollisionEvent {
    fn type_info() -> &'static TypeInfo {
        MULTICOLLISIONEVENT_TYPE_INFO
    }
}


pub const MULTICOLLISIONEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiCollisionEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("MultiCollisionEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PreSolveCollisionInfo {
}

pub const PRESOLVECOLLISIONINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PreSolveCollisionInfo",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COLLISIONINFO_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESOLVECOLLISIONINFO_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PreSolveCollisionInfo {
    fn type_info() -> &'static TypeInfo {
        PRESOLVECOLLISIONINFO_TYPE_INFO
    }
}


pub const PRESOLVECOLLISIONINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PreSolveCollisionInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PreSolveCollisionInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerShockwaveCollisionInfo {
}

pub const SERVERSHOCKWAVECOLLISIONINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerShockwaveCollisionInfo",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SHOCKWAVECOLLISIONINFO_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSHOCKWAVECOLLISIONINFO_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerShockwaveCollisionInfo {
    fn type_info() -> &'static TypeInfo {
        SERVERSHOCKWAVECOLLISIONINFO_TYPE_INFO
    }
}


pub const SERVERSHOCKWAVECOLLISIONINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerShockwaveCollisionInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ServerShockwaveCollisionInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientShockwaveCollisionInfo {
}

pub const CLIENTSHOCKWAVECOLLISIONINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientShockwaveCollisionInfo",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SHOCKWAVECOLLISIONINFO_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSHOCKWAVECOLLISIONINFO_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientShockwaveCollisionInfo {
    fn type_info() -> &'static TypeInfo {
        CLIENTSHOCKWAVECOLLISIONINFO_TYPE_INFO
    }
}


pub const CLIENTSHOCKWAVECOLLISIONINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientShockwaveCollisionInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ClientShockwaveCollisionInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShockwaveCollisionInfo {
}

pub const SHOCKWAVECOLLISIONINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShockwaveCollisionInfo",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(SHOCKWAVECOLLISIONINFO_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ShockwaveCollisionInfo {
    fn type_info() -> &'static TypeInfo {
        SHOCKWAVECOLLISIONINFO_TYPE_INFO
    }
}


pub const SHOCKWAVECOLLISIONINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShockwaveCollisionInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ShockwaveCollisionInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerFakeCollisionInfo {
}

pub const SERVERFAKECOLLISIONINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerFakeCollisionInfo",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(FAKECOLLISIONINFO_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERFAKECOLLISIONINFO_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerFakeCollisionInfo {
    fn type_info() -> &'static TypeInfo {
        SERVERFAKECOLLISIONINFO_TYPE_INFO
    }
}


pub const SERVERFAKECOLLISIONINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerFakeCollisionInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ServerFakeCollisionInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientFakeCollisionInfo {
}

pub const CLIENTFAKECOLLISIONINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientFakeCollisionInfo",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(FAKECOLLISIONINFO_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTFAKECOLLISIONINFO_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientFakeCollisionInfo {
    fn type_info() -> &'static TypeInfo {
        CLIENTFAKECOLLISIONINFO_TYPE_INFO
    }
}


pub const CLIENTFAKECOLLISIONINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientFakeCollisionInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ClientFakeCollisionInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FakeCollisionInfo {
}

pub const FAKECOLLISIONINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FakeCollisionInfo",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(FAKECOLLISIONINFO_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FakeCollisionInfo {
    fn type_info() -> &'static TypeInfo {
        FAKECOLLISIONINFO_TYPE_INFO
    }
}


pub const FAKECOLLISIONINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FakeCollisionInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("FakeCollisionInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CollisionInfo {
}

pub const COLLISIONINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CollisionInfo",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(COLLISIONINFO_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CollisionInfo {
    fn type_info() -> &'static TypeInfo {
        COLLISIONINFO_TYPE_INFO
    }
}


pub const COLLISIONINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CollisionInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("CollisionInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PartPhysicsComponent {
}

pub const PARTPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PARTPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PartPhysicsComponent {
    fn type_info() -> &'static TypeInfo {
        PARTPHYSICSCOMPONENT_TYPE_INFO
    }
}


pub const PARTPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PartPhysicsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GroupPhysicsComponent {
}

pub const GROUPPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroupPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(GROUPPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for GroupPhysicsComponent {
    fn type_info() -> &'static TypeInfo {
        GROUPPHYSICSCOMPONENT_TYPE_INFO
    }
}


pub const GROUPPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroupPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("GroupPhysicsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ForceComponent {
}

pub const FORCECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForceComponent",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FORCECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ForceComponent {
    fn type_info() -> &'static TypeInfo {
        FORCECOMPONENT_TYPE_INFO
    }
}


pub const FORCECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForceComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ForceComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DefaultPartPhysicsComponent {
}

pub const DEFAULTPARTPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DefaultPartPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PARTPHYSICSCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DEFAULTPARTPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DefaultPartPhysicsComponent {
    fn type_info() -> &'static TypeInfo {
        DEFAULTPARTPHYSICSCOMPONENT_TYPE_INFO
    }
}


pub const DEFAULTPARTPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DefaultPartPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("DefaultPartPhysicsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CharacterPhysicsComponent {
}

pub const CHARACTERPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CHARACTERPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CharacterPhysicsComponent {
    fn type_info() -> &'static TypeInfo {
        CHARACTERPHYSICSCOMPONENT_TYPE_INFO
    }
}


pub const CHARACTERPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("CharacterPhysicsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PhysicsConstraintOwner {
}

pub const PHYSICSCONSTRAINTOWNER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsConstraintOwner",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSCONSTRAINTOWNER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PhysicsConstraintOwner {
    fn type_info() -> &'static TypeInfo {
        PHYSICSCONSTRAINTOWNER_TYPE_INFO
    }
}


pub const PHYSICSCONSTRAINTOWNER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsConstraintOwner-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsConstraintOwner-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PhysicsConstraint {
}

pub const PHYSICSCONSTRAINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsConstraint",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSCONSTRAINT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PhysicsConstraint {
    fn type_info() -> &'static TypeInfo {
        PHYSICSCONSTRAINT_TYPE_INFO
    }
}


pub const PHYSICSCONSTRAINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsConstraint-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsConstraint-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StaticWaterPhysicsBody {
}

pub const STATICWATERPHYSICSBODY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticWaterPhysicsBody",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WATERPHYSICSBODY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(STATICWATERPHYSICSBODY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for StaticWaterPhysicsBody {
    fn type_info() -> &'static TypeInfo {
        STATICWATERPHYSICSBODY_TYPE_INFO
    }
}


pub const STATICWATERPHYSICSBODY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticWaterPhysicsBody-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("StaticWaterPhysicsBody-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PhysicsBody {
}

pub const PHYSICSBODY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsBody",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSBODY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PhysicsBody {
    fn type_info() -> &'static TypeInfo {
        PHYSICSBODY_TYPE_INFO
    }
}


pub const PHYSICSBODY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsBody-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsBody-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PhysicsAction {
}

pub const PHYSICSACTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsAction",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSACTION_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PhysicsAction {
    fn type_info() -> &'static TypeInfo {
        PHYSICSACTION_TYPE_INFO
    }
}


pub const PHYSICSACTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsAction-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsAction-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FloatPhysicsAction {
}

pub const FLOATPHYSICSACTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatPhysicsAction",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSACTION_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FLOATPHYSICSACTION_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FloatPhysicsAction {
    fn type_info() -> &'static TypeInfo {
        FLOATPHYSICSACTION_TYPE_INFO
    }
}


pub const FLOATPHYSICSACTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatPhysicsAction-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("FloatPhysicsAction-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FloatPhysicsActionFactory {
}

pub const FLOATPHYSICSACTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatPhysicsActionFactory",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IGLOOENTITYFACTORY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FLOATPHYSICSACTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FloatPhysicsActionFactory {
    fn type_info() -> &'static TypeInfo {
        FLOATPHYSICSACTIONFACTORY_TYPE_INFO
    }
}


pub const FLOATPHYSICSACTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatPhysicsActionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("FloatPhysicsActionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RagdollPhysicsComponent {
}

pub const RAGDOLLPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RagdollPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(RAGDOLLPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RagdollPhysicsComponent {
    fn type_info() -> &'static TypeInfo {
        RAGDOLLPHYSICSCOMPONENT_TYPE_INFO
    }
}


pub const RAGDOLLPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RagdollPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("RagdollPhysicsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WaterPhysicsBody {
}

pub const WATERPHYSICSBODY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterPhysicsBody",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSBODY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WATERPHYSICSBODY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for WaterPhysicsBody {
    fn type_info() -> &'static TypeInfo {
        WATERPHYSICSBODY_TYPE_INFO
    }
}


pub const WATERPHYSICSBODY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterPhysicsBody-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("WaterPhysicsBody-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TerrainPhysicsBody {
}

pub const TERRAINPHYSICSBODY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainPhysicsBody",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSBODY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TERRAINPHYSICSBODY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TerrainPhysicsBody {
    fn type_info() -> &'static TypeInfo {
        TERRAINPHYSICSBODY_TYPE_INFO
    }
}


pub const TERRAINPHYSICSBODY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainPhysicsBody-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("TerrainPhysicsBody-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RigidBody {
}

pub const RIGIDBODY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RigidBody",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSBODY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(RIGIDBODY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RigidBody {
    fn type_info() -> &'static TypeInfo {
        RIGIDBODY_TYPE_INFO
    }
}


pub const RIGIDBODY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RigidBody-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("RigidBody-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GroupRigidBody {
}

pub const GROUPRIGIDBODY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroupRigidBody",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSBODY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(GROUPRIGIDBODY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for GroupRigidBody {
    fn type_info() -> &'static TypeInfo {
        GROUPRIGIDBODY_TYPE_INFO
    }
}


pub const GROUPRIGIDBODY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroupRigidBody-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("GroupRigidBody-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CharacterPhysicsBody {
}

pub const CHARACTERPHYSICSBODY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterPhysicsBody",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSBODY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CHARACTERPHYSICSBODY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CharacterPhysicsBody {
    fn type_info() -> &'static TypeInfo {
        CHARACTERPHYSICSBODY_TYPE_INFO
    }
}


pub const CHARACTERPHYSICSBODY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterPhysicsBody-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("CharacterPhysicsBody-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AabbTriggerPhysicsBody {
}

pub const AABBTRIGGERPHYSICSBODY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AabbTriggerPhysicsBody",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSBODY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(AABBTRIGGERPHYSICSBODY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AabbTriggerPhysicsBody {
    fn type_info() -> &'static TypeInfo {
        AABBTRIGGERPHYSICSBODY_TYPE_INFO
    }
}


pub const AABBTRIGGERPHYSICSBODY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AabbTriggerPhysicsBody-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("AabbTriggerPhysicsBody-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PhysicsManager {
}

pub const PHYSICSMANAGER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsManager",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IGLOOSUBSYSTEM_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSMANAGER_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsManager {
    fn type_info() -> &'static TypeInfo {
        PHYSICSMANAGER_TYPE_INFO
    }
}


pub const PHYSICSMANAGER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsManager-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsManager-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PhysicsEntityCreator {
}

pub const PHYSICSENTITYCREATOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsEntityCreator",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IGLOOENTITYCREATOR_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSENTITYCREATOR_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PhysicsEntityCreator {
    fn type_info() -> &'static TypeInfo {
        PHYSICSENTITYCREATOR_TYPE_INFO
    }
}


pub const PHYSICSENTITYCREATOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsEntityCreator-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsEntityCreator-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PhysicsEntityFactory {
}

pub const PHYSICSENTITYFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsEntityFactory",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IGLOOENTITYFACTORY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSENTITYFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PhysicsEntityFactory {
    fn type_info() -> &'static TypeInfo {
        PHYSICSENTITYFACTORY_TYPE_INFO
    }
}


pub const PHYSICSENTITYFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsEntityFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsEntityFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DestructionDebrisReferenceObjectData {
    pub insert_type: DebrisInsertType,
    pub gap: f32,
    pub max_piece_count: super::core::QualityScalableInt,
    pub random_seed: u32,
}

pub const DESTRUCTIONDEBRISREFERENCEOBJECTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionDebrisReferenceObjectData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALREFERENCEOBJECTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "InsertType",
                flags: MemberInfoFlags::new(0),
                field_type: DEBRISINSERTTYPE_TYPE_INFO,
                rust_offset: offset_of!(DestructionDebrisReferenceObjectData, insert_type),
            },
            FieldInfoData {
                name: "Gap",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DestructionDebrisReferenceObjectData, gap),
            },
            FieldInfoData {
                name: "MaxPieceCount",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(DestructionDebrisReferenceObjectData, max_piece_count),
            },
            FieldInfoData {
                name: "RandomSeed",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DestructionDebrisReferenceObjectData, random_seed),
            },
        ],
    }),
    array_type: Some(DESTRUCTIONDEBRISREFERENCEOBJECTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DestructionDebrisReferenceObjectData {
    fn type_info() -> &'static TypeInfo {
        DESTRUCTIONDEBRISREFERENCEOBJECTDATA_TYPE_INFO
    }
}


pub const DESTRUCTIONDEBRISREFERENCEOBJECTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionDebrisReferenceObjectData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("DestructionDebrisReferenceObjectData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum DebrisInsertType {
    #[default]
    DebrisInsertType_UseDebrisPivot = 0,
    DebrisInsertType_RandomPosition = 1,
    DebrisInsertType_RandomTransform = 2,
}

pub const DEBRISINSERTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisInsertType",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(DEBRISINSERTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DebrisInsertType {
    fn type_info() -> &'static TypeInfo {
        DEBRISINSERTTYPE_TYPE_INFO
    }
}


pub const DEBRISINSERTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisInsertType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("DebrisInsertType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DestructionControllerComponentData {
    pub structural_integrity_analyzer_enable: bool,
    pub breaking_length: f32,
    pub fracture_position: f32,
    pub destruction_radius: f32,
    pub delay: f32,
    pub refine_radius: f32,
}

pub const DESTRUCTIONCONTROLLERCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionControllerComponentData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMECOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "StructuralIntegrityAnalyzerEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DestructionControllerComponentData, structural_integrity_analyzer_enable),
            },
            FieldInfoData {
                name: "BreakingLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DestructionControllerComponentData, breaking_length),
            },
            FieldInfoData {
                name: "FracturePosition",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DestructionControllerComponentData, fracture_position),
            },
            FieldInfoData {
                name: "DestructionRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DestructionControllerComponentData, destruction_radius),
            },
            FieldInfoData {
                name: "Delay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DestructionControllerComponentData, delay),
            },
            FieldInfoData {
                name: "RefineRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DestructionControllerComponentData, refine_radius),
            },
        ],
    }),
    array_type: Some(DESTRUCTIONCONTROLLERCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DestructionControllerComponentData {
    fn type_info() -> &'static TypeInfo {
        DESTRUCTIONCONTROLLERCOMPONENTDATA_TYPE_INFO
    }
}


pub const DESTRUCTIONCONTROLLERCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionControllerComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("DestructionControllerComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EdgeModelInstance {
    pub transform: super::core::LinearTransform,
}

pub const EDGEMODELINSTANCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EdgeModelInstance",
    flags: MemberInfoFlags::new(32841),
    module: "Physics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(EdgeModelInstance, transform),
            },
        ],
    }),
    array_type: Some(EDGEMODELINSTANCE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EdgeModelInstance {
    fn type_info() -> &'static TypeInfo {
        EDGEMODELINSTANCE_TYPE_INFO
    }
}


pub const EDGEMODELINSTANCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EdgeModelInstance-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("EdgeModelInstance-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DestructionPartComponentData {
    pub objects: Vec<super::entity::GameObjectData>,
    pub part_index: u32,
    pub fixed: bool,
    pub fragile: bool,
}

pub const DESTRUCTIONPARTCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionPartComponentData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMECOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Objects",
                flags: MemberInfoFlags::new(144),
                field_type: GAMEOBJECTDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(DestructionPartComponentData, objects),
            },
            FieldInfoData {
                name: "PartIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DestructionPartComponentData, part_index),
            },
            FieldInfoData {
                name: "Fixed",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DestructionPartComponentData, fixed),
            },
            FieldInfoData {
                name: "Fragile",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DestructionPartComponentData, fragile),
            },
        ],
    }),
    array_type: Some(DESTRUCTIONPARTCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DestructionPartComponentData {
    fn type_info() -> &'static TypeInfo {
        DESTRUCTIONPARTCOMPONENTDATA_TYPE_INFO
    }
}


pub const DESTRUCTIONPARTCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionPartComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("DestructionPartComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FakeHingeData {
    pub pivot: super::core::Vec3,
    pub rotation_axis: super::core::Vec3,
    pub extension_axis: super::core::Vec3,
    pub min_angle: f32,
    pub max_angle: f32,
    pub angular_dampening: f32,
    pub pullback_acceleration: f32,
    pub progressive_exponent: f32,
    pub inertia_modifier: f32,
}

pub const FAKEHINGEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FakeHingeData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(FAKEPHYSICSDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Pivot",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(FakeHingeData, pivot),
            },
            FieldInfoData {
                name: "RotationAxis",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(FakeHingeData, rotation_axis),
            },
            FieldInfoData {
                name: "ExtensionAxis",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(FakeHingeData, extension_axis),
            },
            FieldInfoData {
                name: "MinAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FakeHingeData, min_angle),
            },
            FieldInfoData {
                name: "MaxAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FakeHingeData, max_angle),
            },
            FieldInfoData {
                name: "AngularDampening",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FakeHingeData, angular_dampening),
            },
            FieldInfoData {
                name: "PullbackAcceleration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FakeHingeData, pullback_acceleration),
            },
            FieldInfoData {
                name: "ProgressiveExponent",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FakeHingeData, progressive_exponent),
            },
            FieldInfoData {
                name: "InertiaModifier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FakeHingeData, inertia_modifier),
            },
        ],
    }),
    array_type: Some(FAKEHINGEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FakeHingeData {
    fn type_info() -> &'static TypeInfo {
        FAKEHINGEDATA_TYPE_INFO
    }
}


pub const FAKEHINGEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FakeHingeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("FakeHingeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FakeSpringData {
    pub direction: super::core::Vec3,
    pub length: f32,
    pub acceleration: f32,
    pub progressive_exponent: f32,
    pub damping: f32,
}

pub const FAKESPRINGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FakeSpringData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(FAKEPHYSICSDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Direction",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(FakeSpringData, direction),
            },
            FieldInfoData {
                name: "Length",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FakeSpringData, length),
            },
            FieldInfoData {
                name: "Acceleration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FakeSpringData, acceleration),
            },
            FieldInfoData {
                name: "ProgressiveExponent",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FakeSpringData, progressive_exponent),
            },
            FieldInfoData {
                name: "Damping",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FakeSpringData, damping),
            },
        ],
    }),
    array_type: Some(FAKESPRINGDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FakeSpringData {
    fn type_info() -> &'static TypeInfo {
        FAKESPRINGDATA_TYPE_INFO
    }
}


pub const FAKESPRINGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FakeSpringData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("FakeSpringData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FakePhysicsData {
    pub child_fake_physics: FakePhysicsData,
    pub start_speed: f32,
    pub gravity_modifier: f32,
    pub start_dampening: f32,
    pub end_dampening: f32,
    pub min_speed: f32,
    pub max_speed: f32,
    pub mass: f32,
}

pub const FAKEPHYSICSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FakePhysicsData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ChildFakePhysics",
                flags: MemberInfoFlags::new(0),
                field_type: FAKEPHYSICSDATA_TYPE_INFO,
                rust_offset: offset_of!(FakePhysicsData, child_fake_physics),
            },
            FieldInfoData {
                name: "StartSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FakePhysicsData, start_speed),
            },
            FieldInfoData {
                name: "GravityModifier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FakePhysicsData, gravity_modifier),
            },
            FieldInfoData {
                name: "StartDampening",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FakePhysicsData, start_dampening),
            },
            FieldInfoData {
                name: "EndDampening",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FakePhysicsData, end_dampening),
            },
            FieldInfoData {
                name: "MinSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FakePhysicsData, min_speed),
            },
            FieldInfoData {
                name: "MaxSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FakePhysicsData, max_speed),
            },
            FieldInfoData {
                name: "Mass",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FakePhysicsData, mass),
            },
        ],
    }),
    array_type: Some(FAKEPHYSICSDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FakePhysicsData {
    fn type_info() -> &'static TypeInfo {
        FAKEPHYSICSDATA_TYPE_INFO
    }
}


pub const FAKEPHYSICSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FakePhysicsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("FakePhysicsData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PhysicsAngularLimitConstraintData {
    pub min_angle: f32,
    pub max_angle: f32,
    pub constrained_axis: AngularLimitConstraintAxis,
    pub angular_stiffness: f32,
    pub target_transform: super::core::LinearTransform,
}

pub const PHYSICSANGULARLIMITCONSTRAINTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsAngularLimitConstraintData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MinAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsAngularLimitConstraintData, min_angle),
            },
            FieldInfoData {
                name: "MaxAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsAngularLimitConstraintData, max_angle),
            },
            FieldInfoData {
                name: "ConstrainedAxis",
                flags: MemberInfoFlags::new(0),
                field_type: ANGULARLIMITCONSTRAINTAXIS_TYPE_INFO,
                rust_offset: offset_of!(PhysicsAngularLimitConstraintData, constrained_axis),
            },
            FieldInfoData {
                name: "AngularStiffness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicsAngularLimitConstraintData, angular_stiffness),
            },
            FieldInfoData {
                name: "TargetTransform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(PhysicsAngularLimitConstraintData, target_transform),
            },
        ],
    }),
    array_type: Some(PHYSICSANGULARLIMITCONSTRAINTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsAngularLimitConstraintData {
    fn type_info() -> &'static TypeInfo {
        PHYSICSANGULARLIMITCONSTRAINTDATA_TYPE_INFO
    }
}


pub const PHYSICSANGULARLIMITCONSTRAINTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsAngularLimitConstraintData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsAngularLimitConstraintData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum AngularLimitConstraintAxis {
    #[default]
    AngularLimitConstraintAxis_X = 0,
    AngularLimitConstraintAxis_Y = 1,
    AngularLimitConstraintAxis_Z = 2,
}

pub const ANGULARLIMITCONSTRAINTAXIS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngularLimitConstraintAxis",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(ANGULARLIMITCONSTRAINTAXIS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AngularLimitConstraintAxis {
    fn type_info() -> &'static TypeInfo {
        ANGULARLIMITCONSTRAINTAXIS_TYPE_INFO
    }
}


pub const ANGULARLIMITCONSTRAINTAXIS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngularLimitConstraintAxis-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("AngularLimitConstraintAxis-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VehiclePhysicsActionFactory {
}

pub const VEHICLEPHYSICSACTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehiclePhysicsActionFactory",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IGLOOENTITYFACTORY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VEHICLEPHYSICSACTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VehiclePhysicsActionFactory {
    fn type_info() -> &'static TypeInfo {
        VEHICLEPHYSICSACTIONFACTORY_TYPE_INFO
    }
}


pub const VEHICLEPHYSICSACTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehiclePhysicsActionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("VehiclePhysicsActionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VehiclePhysicsAction {
}

pub const VEHICLEPHYSICSACTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehiclePhysicsAction",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSACTION_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VEHICLEPHYSICSACTION_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for VehiclePhysicsAction {
    fn type_info() -> &'static TypeInfo {
        VEHICLEPHYSICSACTION_TYPE_INFO
    }
}


pub const VEHICLEPHYSICSACTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehiclePhysicsAction-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("VehiclePhysicsAction-Array"),
    array_type: None,
    alignment: 8,
};


