use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct HavokPhysicsData {
}

pub trait HavokPhysicsDataTrait: TypeObject {
}

impl HavokPhysicsDataTrait for HavokPhysicsData {
}

pub static HAVOKPHYSICSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HavokPhysicsData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<HavokPhysicsData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(HAVOKPHYSICSDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for HavokPhysicsData {
    fn type_info(&self) -> &'static TypeInfo {
        HAVOKPHYSICSDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static HAVOKPHYSICSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HavokPhysicsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("HavokPhysicsData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RagdollResource {
}

pub trait RagdollResourceTrait: TypeObject {
}

impl RagdollResourceTrait for RagdollResource {
}

pub static RAGDOLLRESOURCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RagdollResource",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RagdollResource as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(RAGDOLLRESOURCE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RagdollResource {
    fn type_info(&self) -> &'static TypeInfo {
        RAGDOLLRESOURCE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RAGDOLLRESOURCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RagdollResource-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("RagdollResource"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsRootControlConstraint {
    pub _glacier_base: PhysicsConstraint,
}

pub trait PhysicsRootControlConstraintTrait: PhysicsConstraintTrait {
}

impl PhysicsRootControlConstraintTrait for PhysicsRootControlConstraint {
}

impl PhysicsConstraintTrait for PhysicsRootControlConstraint {
}

impl super::entity::EntityTrait for PhysicsRootControlConstraint {
}

impl super::entity::EntityBusPeerTrait for PhysicsRootControlConstraint {
}

pub static PHYSICSROOTCONTROLCONSTRAINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRootControlConstraint",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsRootControlConstraint as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSROOTCONTROLCONSTRAINT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PhysicsRootControlConstraint {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSROOTCONTROLCONSTRAINT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSROOTCONTROLCONSTRAINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRootControlConstraint-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsRootControlConstraint"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsRagdollConstraint {
    pub _glacier_base: PhysicsConstraint,
}

pub trait PhysicsRagdollConstraintTrait: PhysicsConstraintTrait {
}

impl PhysicsRagdollConstraintTrait for PhysicsRagdollConstraint {
}

impl PhysicsConstraintTrait for PhysicsRagdollConstraint {
}

impl super::entity::EntityTrait for PhysicsRagdollConstraint {
}

impl super::entity::EntityBusPeerTrait for PhysicsRagdollConstraint {
}

pub static PHYSICSRAGDOLLCONSTRAINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRagdollConstraint",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsRagdollConstraint as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSRAGDOLLCONSTRAINT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PhysicsRagdollConstraint {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSRAGDOLLCONSTRAINT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSRAGDOLLCONSTRAINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRagdollConstraint-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsRagdollConstraint"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsPrismaticConstraint {
    pub _glacier_base: PhysicsConstraint,
}

pub trait PhysicsPrismaticConstraintTrait: PhysicsConstraintTrait {
}

impl PhysicsPrismaticConstraintTrait for PhysicsPrismaticConstraint {
}

impl PhysicsConstraintTrait for PhysicsPrismaticConstraint {
}

impl super::entity::EntityTrait for PhysicsPrismaticConstraint {
}

impl super::entity::EntityBusPeerTrait for PhysicsPrismaticConstraint {
}

pub static PHYSICSPRISMATICCONSTRAINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsPrismaticConstraint",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsPrismaticConstraint as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSPRISMATICCONSTRAINT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PhysicsPrismaticConstraint {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSPRISMATICCONSTRAINT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSPRISMATICCONSTRAINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsPrismaticConstraint-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsPrismaticConstraint"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsHingeConstraint {
    pub _glacier_base: PhysicsConstraint,
}

pub trait PhysicsHingeConstraintTrait: PhysicsConstraintTrait {
}

impl PhysicsHingeConstraintTrait for PhysicsHingeConstraint {
}

impl PhysicsConstraintTrait for PhysicsHingeConstraint {
}

impl super::entity::EntityTrait for PhysicsHingeConstraint {
}

impl super::entity::EntityBusPeerTrait for PhysicsHingeConstraint {
}

pub static PHYSICSHINGECONSTRAINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsHingeConstraint",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsHingeConstraint as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSHINGECONSTRAINT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PhysicsHingeConstraint {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSHINGECONSTRAINT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSHINGECONSTRAINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsHingeConstraint-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsHingeConstraint"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsBallAndSocketConstraint {
    pub _glacier_base: PhysicsConstraint,
}

pub trait PhysicsBallAndSocketConstraintTrait: PhysicsConstraintTrait {
}

impl PhysicsBallAndSocketConstraintTrait for PhysicsBallAndSocketConstraint {
}

impl PhysicsConstraintTrait for PhysicsBallAndSocketConstraint {
}

impl super::entity::EntityTrait for PhysicsBallAndSocketConstraint {
}

impl super::entity::EntityBusPeerTrait for PhysicsBallAndSocketConstraint {
}

pub static PHYSICSBALLANDSOCKETCONSTRAINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsBallAndSocketConstraint",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsBallAndSocketConstraint as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSBALLANDSOCKETCONSTRAINT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PhysicsBallAndSocketConstraint {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSBALLANDSOCKETCONSTRAINT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSBALLANDSOCKETCONSTRAINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsBallAndSocketConstraint-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsBallAndSocketConstraint"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsAngularLimitConstraint {
    pub _glacier_base: PhysicsConstraint,
}

pub trait PhysicsAngularLimitConstraintTrait: PhysicsConstraintTrait {
}

impl PhysicsAngularLimitConstraintTrait for PhysicsAngularLimitConstraint {
}

impl PhysicsConstraintTrait for PhysicsAngularLimitConstraint {
}

impl super::entity::EntityTrait for PhysicsAngularLimitConstraint {
}

impl super::entity::EntityBusPeerTrait for PhysicsAngularLimitConstraint {
}

pub static PHYSICSANGULARLIMITCONSTRAINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsAngularLimitConstraint",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsAngularLimitConstraint as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSANGULARLIMITCONSTRAINT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PhysicsAngularLimitConstraint {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSANGULARLIMITCONSTRAINT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSANGULARLIMITCONSTRAINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsAngularLimitConstraint-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsAngularLimitConstraint"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct IglooModuleInitializer {
}

pub trait IglooModuleInitializerTrait: TypeObject {
}

impl IglooModuleInitializerTrait for IglooModuleInitializer {
}

pub static IGLOOMODULEINITIALIZER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IglooModuleInitializer",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IglooModuleInitializer as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(IGLOOMODULEINITIALIZER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IglooModuleInitializer {
    fn type_info(&self) -> &'static TypeInfo {
        IGLOOMODULEINITIALIZER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static IGLOOMODULEINITIALIZER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IglooModuleInitializer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("IglooModuleInitializer"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct IglooMemoryContext {
}

pub trait IglooMemoryContextTrait: TypeObject {
}

impl IglooMemoryContextTrait for IglooMemoryContext {
}

pub static IGLOOMEMORYCONTEXT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IglooMemoryContext",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IglooMemoryContext as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(IGLOOMEMORYCONTEXT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IglooMemoryContext {
    fn type_info(&self) -> &'static TypeInfo {
        IGLOOMEMORYCONTEXT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static IGLOOMEMORYCONTEXT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IglooMemoryContext-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("IglooMemoryContext"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct IglooGrabTool {
}

pub trait IglooGrabToolTrait: TypeObject {
}

impl IglooGrabToolTrait for IglooGrabTool {
}

pub static IGLOOGRABTOOL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IglooGrabTool",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IglooGrabTool as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(IGLOOGRABTOOL_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for IglooGrabTool {
    fn type_info(&self) -> &'static TypeInfo {
        IGLOOGRABTOOL_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static IGLOOGRABTOOL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IglooGrabTool-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("IglooGrabTool"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct IglooEntityCreator {
}

pub trait IglooEntityCreatorTrait: TypeObject {
}

impl IglooEntityCreatorTrait for IglooEntityCreator {
}

pub static IGLOOENTITYCREATOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IglooEntityCreator",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IglooEntityCreator as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(IGLOOENTITYCREATOR_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IglooEntityCreator {
    fn type_info(&self) -> &'static TypeInfo {
        IGLOOENTITYCREATOR_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static IGLOOENTITYCREATOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IglooEntityCreator-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("IglooEntityCreator"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct IglooEntityFactory {
}

pub trait IglooEntityFactoryTrait: TypeObject {
}

impl IglooEntityFactoryTrait for IglooEntityFactory {
}

pub static IGLOOENTITYFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IglooEntityFactory",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IglooEntityFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(IGLOOENTITYFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IglooEntityFactory {
    fn type_info(&self) -> &'static TypeInfo {
        IGLOOENTITYFACTORY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static IGLOOENTITYFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IglooEntityFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("IglooEntityFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct IglooDebugger {
}

pub trait IglooDebuggerTrait: TypeObject {
}

impl IglooDebuggerTrait for IglooDebugger {
}

pub static IGLOODEBUGGER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IglooDebugger",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IglooDebugger as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(IGLOODEBUGGER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IglooDebugger {
    fn type_info(&self) -> &'static TypeInfo {
        IGLOODEBUGGER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static IGLOODEBUGGER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IglooDebugger-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("IglooDebugger"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FloatPhysicsData {
    pub _glacier_base: super::core::DataContainer,
    pub density: f32,
    pub filled_density: f32,
}

pub trait FloatPhysicsDataTrait: super::core::DataContainerTrait {
    fn density(&self) -> &f32;
    fn filled_density(&self) -> &f32;
}

impl FloatPhysicsDataTrait for FloatPhysicsData {
    fn density(&self) -> &f32 {
        &self.density
    }
    fn filled_density(&self) -> &f32 {
        &self.filled_density
    }
}

impl super::core::DataContainerTrait for FloatPhysicsData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static FLOATPHYSICSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatPhysicsData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FloatPhysicsData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Density",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FloatPhysicsData, density),
            },
            FieldInfoData {
                name: "FilledDensity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FloatPhysicsData, filled_density),
            },
        ],
    }),
    array_type: Some(FLOATPHYSICSDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FloatPhysicsData {
    fn type_info(&self) -> &'static TypeInfo {
        FLOATPHYSICSDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FLOATPHYSICSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatPhysicsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("FloatPhysicsData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PartInfoData {
    pub aabb: super::core::AxisAlignedBox,
    pub translation: super::core::Vec3,
}

pub trait PartInfoDataTrait: TypeObject {
    fn aabb(&self) -> &super::core::AxisAlignedBox;
    fn translation(&self) -> &super::core::Vec3;
}

impl PartInfoDataTrait for PartInfoData {
    fn aabb(&self) -> &super::core::AxisAlignedBox {
        &self.aabb
    }
    fn translation(&self) -> &super::core::Vec3 {
        &self.translation
    }
}

pub static PARTINFODATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartInfoData",
    flags: MemberInfoFlags::new(32841),
    module: "Physics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PartInfoData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Aabb",
                flags: MemberInfoFlags::new(0),
                field_type: "AxisAlignedBox",
                rust_offset: offset_of!(PartInfoData, aabb),
            },
            FieldInfoData {
                name: "Translation",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(PartInfoData, translation),
            },
        ],
    }),
    array_type: Some(PARTINFODATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PartInfoData {
    fn type_info(&self) -> &'static TypeInfo {
        PARTINFODATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PARTINFODATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartInfoData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PartInfoData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PreSolveCollisionPhysicsCallbackHandlerData {
    pub _glacier_base: PhysicsCallbackHandlerData,
}

pub trait PreSolveCollisionPhysicsCallbackHandlerDataTrait: PhysicsCallbackHandlerDataTrait {
}

impl PreSolveCollisionPhysicsCallbackHandlerDataTrait for PreSolveCollisionPhysicsCallbackHandlerData {
}

impl PhysicsCallbackHandlerDataTrait for PreSolveCollisionPhysicsCallbackHandlerData {
}

impl super::core::DataContainerTrait for PreSolveCollisionPhysicsCallbackHandlerData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PRESOLVECOLLISIONPHYSICSCALLBACKHANDLERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PreSolveCollisionPhysicsCallbackHandlerData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCALLBACKHANDLERDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PreSolveCollisionPhysicsCallbackHandlerData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESOLVECOLLISIONPHYSICSCALLBACKHANDLERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PreSolveCollisionPhysicsCallbackHandlerData {
    fn type_info(&self) -> &'static TypeInfo {
        PRESOLVECOLLISIONPHYSICSCALLBACKHANDLERDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PRESOLVECOLLISIONPHYSICSCALLBACKHANDLERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PreSolveCollisionPhysicsCallbackHandlerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PreSolveCollisionPhysicsCallbackHandlerData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VehiclePhysicsCallbackHandlerData {
    pub _glacier_base: PhysicsCallbackHandlerData,
}

pub trait VehiclePhysicsCallbackHandlerDataTrait: PhysicsCallbackHandlerDataTrait {
}

impl VehiclePhysicsCallbackHandlerDataTrait for VehiclePhysicsCallbackHandlerData {
}

impl PhysicsCallbackHandlerDataTrait for VehiclePhysicsCallbackHandlerData {
}

impl super::core::DataContainerTrait for VehiclePhysicsCallbackHandlerData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static VEHICLEPHYSICSCALLBACKHANDLERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehiclePhysicsCallbackHandlerData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCALLBACKHANDLERDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VehiclePhysicsCallbackHandlerData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(VEHICLEPHYSICSCALLBACKHANDLERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VehiclePhysicsCallbackHandlerData {
    fn type_info(&self) -> &'static TypeInfo {
        VEHICLEPHYSICSCALLBACKHANDLERDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VEHICLEPHYSICSCALLBACKHANDLERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehiclePhysicsCallbackHandlerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("VehiclePhysicsCallbackHandlerData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientDynamicPhysicsCallbackHandlerData {
    pub _glacier_base: PhysicsCallbackHandlerData,
}

pub trait ClientDynamicPhysicsCallbackHandlerDataTrait: PhysicsCallbackHandlerDataTrait {
}

impl ClientDynamicPhysicsCallbackHandlerDataTrait for ClientDynamicPhysicsCallbackHandlerData {
}

impl PhysicsCallbackHandlerDataTrait for ClientDynamicPhysicsCallbackHandlerData {
}

impl super::core::DataContainerTrait for ClientDynamicPhysicsCallbackHandlerData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static CLIENTDYNAMICPHYSICSCALLBACKHANDLERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDynamicPhysicsCallbackHandlerData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCALLBACKHANDLERDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientDynamicPhysicsCallbackHandlerData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTDYNAMICPHYSICSCALLBACKHANDLERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ClientDynamicPhysicsCallbackHandlerData {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTDYNAMICPHYSICSCALLBACKHANDLERDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTDYNAMICPHYSICSCALLBACKHANDLERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDynamicPhysicsCallbackHandlerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ClientDynamicPhysicsCallbackHandlerData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LightDynamicPhysicsCallbackHandlerData {
    pub _glacier_base: PhysicsCallbackHandlerData,
}

pub trait LightDynamicPhysicsCallbackHandlerDataTrait: PhysicsCallbackHandlerDataTrait {
}

impl LightDynamicPhysicsCallbackHandlerDataTrait for LightDynamicPhysicsCallbackHandlerData {
}

impl PhysicsCallbackHandlerDataTrait for LightDynamicPhysicsCallbackHandlerData {
}

impl super::core::DataContainerTrait for LightDynamicPhysicsCallbackHandlerData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static LIGHTDYNAMICPHYSICSCALLBACKHANDLERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightDynamicPhysicsCallbackHandlerData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCALLBACKHANDLERDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LightDynamicPhysicsCallbackHandlerData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LIGHTDYNAMICPHYSICSCALLBACKHANDLERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LightDynamicPhysicsCallbackHandlerData {
    fn type_info(&self) -> &'static TypeInfo {
        LIGHTDYNAMICPHYSICSCALLBACKHANDLERDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LIGHTDYNAMICPHYSICSCALLBACKHANDLERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightDynamicPhysicsCallbackHandlerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("LightDynamicPhysicsCallbackHandlerData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DefaultPhysicsCallbackHandlerData {
    pub _glacier_base: PhysicsCallbackHandlerData,
}

pub trait DefaultPhysicsCallbackHandlerDataTrait: PhysicsCallbackHandlerDataTrait {
}

impl DefaultPhysicsCallbackHandlerDataTrait for DefaultPhysicsCallbackHandlerData {
}

impl PhysicsCallbackHandlerDataTrait for DefaultPhysicsCallbackHandlerData {
}

impl super::core::DataContainerTrait for DefaultPhysicsCallbackHandlerData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static DEFAULTPHYSICSCALLBACKHANDLERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DefaultPhysicsCallbackHandlerData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCALLBACKHANDLERDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DefaultPhysicsCallbackHandlerData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DEFAULTPHYSICSCALLBACKHANDLERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DefaultPhysicsCallbackHandlerData {
    fn type_info(&self) -> &'static TypeInfo {
        DEFAULTPHYSICSCALLBACKHANDLERDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DEFAULTPHYSICSCALLBACKHANDLERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DefaultPhysicsCallbackHandlerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("DefaultPhysicsCallbackHandlerData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct NonePhysicsCallbackHandlerData {
    pub _glacier_base: PhysicsCallbackHandlerData,
}

pub trait NonePhysicsCallbackHandlerDataTrait: PhysicsCallbackHandlerDataTrait {
}

impl NonePhysicsCallbackHandlerDataTrait for NonePhysicsCallbackHandlerData {
}

impl PhysicsCallbackHandlerDataTrait for NonePhysicsCallbackHandlerData {
}

impl super::core::DataContainerTrait for NonePhysicsCallbackHandlerData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static NONEPHYSICSCALLBACKHANDLERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NonePhysicsCallbackHandlerData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCALLBACKHANDLERDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NonePhysicsCallbackHandlerData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(NONEPHYSICSCALLBACKHANDLERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NonePhysicsCallbackHandlerData {
    fn type_info(&self) -> &'static TypeInfo {
        NONEPHYSICSCALLBACKHANDLERDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static NONEPHYSICSCALLBACKHANDLERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NonePhysicsCallbackHandlerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("NonePhysicsCallbackHandlerData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsCallbackHandlerData {
    pub _glacier_base: super::core::DataContainer,
}

pub trait PhysicsCallbackHandlerDataTrait: super::core::DataContainerTrait {
}

impl PhysicsCallbackHandlerDataTrait for PhysicsCallbackHandlerData {
}

impl super::core::DataContainerTrait for PhysicsCallbackHandlerData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PHYSICSCALLBACKHANDLERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsCallbackHandlerData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsCallbackHandlerData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSCALLBACKHANDLERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PhysicsCallbackHandlerData {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSCALLBACKHANDLERDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSCALLBACKHANDLERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsCallbackHandlerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsCallbackHandlerData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static RIGIDBODYCOLLISIONLAYER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RigidBodyCollisionLayer",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(RIGIDBODYCOLLISIONLAYER_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RigidBodyCollisionLayer {
    fn type_info(&self) -> &'static TypeInfo {
        RIGIDBODYCOLLISIONLAYER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RIGIDBODYCOLLISIONLAYER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RigidBodyCollisionLayer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("RigidBodyCollisionLayer"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static RIGIDBODYQUALITYTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RigidBodyQualityType",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(RIGIDBODYQUALITYTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RigidBodyQualityType {
    fn type_info(&self) -> &'static TypeInfo {
        RIGIDBODYQUALITYTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RIGIDBODYQUALITYTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RigidBodyQualityType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("RigidBodyQualityType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum RigidBodyMotionType {
    #[default]
    RigidBodyMotionType_Invalid = 0,
    RigidBodyMotionType_Fixed = 1,
    RigidBodyMotionType_Keyframed = 2,
    RigidBodyMotionType_Dynamic = 3,
    RigidBodyMotionType_Size = 4,
}

pub static RIGIDBODYMOTIONTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RigidBodyMotionType",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(RIGIDBODYMOTIONTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RigidBodyMotionType {
    fn type_info(&self) -> &'static TypeInfo {
        RIGIDBODYMOTIONTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RIGIDBODYMOTIONTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RigidBodyMotionType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("RigidBodyMotionType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static RIGIDBODYTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RigidBodyType",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(RIGIDBODYTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RigidBodyType {
    fn type_info(&self) -> &'static TypeInfo {
        RIGIDBODYTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RIGIDBODYTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RigidBodyType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("RigidBodyType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsBodyLauncherEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub use_local_body_space: bool,
    pub launch_transform: super::core::LinearTransform,
    pub linear_velocity: super::core::Vec3,
    pub angular_velocity: super::core::Vec3,
}

pub trait PhysicsBodyLauncherEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn use_local_body_space(&self) -> &bool;
    fn launch_transform(&self) -> &super::core::LinearTransform;
    fn linear_velocity(&self) -> &super::core::Vec3;
    fn angular_velocity(&self) -> &super::core::Vec3;
}

impl PhysicsBodyLauncherEntityDataTrait for PhysicsBodyLauncherEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn use_local_body_space(&self) -> &bool {
        &self.use_local_body_space
    }
    fn launch_transform(&self) -> &super::core::LinearTransform {
        &self.launch_transform
    }
    fn linear_velocity(&self) -> &super::core::Vec3 {
        &self.linear_velocity
    }
    fn angular_velocity(&self) -> &super::core::Vec3 {
        &self.angular_velocity
    }
}

impl super::entity::EntityDataTrait for PhysicsBodyLauncherEntityData {
}

impl super::entity::GameObjectDataTrait for PhysicsBodyLauncherEntityData {
}

impl super::core::DataBusPeerTrait for PhysicsBodyLauncherEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for PhysicsBodyLauncherEntityData {
}

impl super::core::DataContainerTrait for PhysicsBodyLauncherEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PHYSICSBODYLAUNCHERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsBodyLauncherEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsBodyLauncherEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(PhysicsBodyLauncherEntityData, realm),
            },
            FieldInfoData {
                name: "UseLocalBodySpace",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsBodyLauncherEntityData, use_local_body_space),
            },
            FieldInfoData {
                name: "LaunchTransform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(PhysicsBodyLauncherEntityData, launch_transform),
            },
            FieldInfoData {
                name: "LinearVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(PhysicsBodyLauncherEntityData, linear_velocity),
            },
            FieldInfoData {
                name: "AngularVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(PhysicsBodyLauncherEntityData, angular_velocity),
            },
        ],
    }),
    array_type: Some(PHYSICSBODYLAUNCHERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsBodyLauncherEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSBODYLAUNCHERENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSBODYLAUNCHERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsBodyLauncherEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsBodyLauncherEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsDebugSettings {
    pub _glacier_base: super::core::DataContainer,
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

pub trait PhysicsDebugSettingsTrait: super::core::DataContainerTrait {
    fn debug_hinge_constraints(&self) -> &bool;
    fn use_physics_cpu_timers(&self) -> &bool;
    fn timing_recursion_depth(&self) -> &u32;
    fn use_brute_force_broadphase(&self) -> &bool;
    fn v_d_b_shows_effects_world(&self) -> &bool;
    fn v_d_b_enable_havok_monitors(&self) -> &bool;
    fn v_d_b_enable_capture_to_file(&self) -> &bool;
    fn v_d_b_capture_file_name(&self) -> &String;
    fn v_d_b_connection_time_out(&self) -> &u32;
    fn v_d_b_show_body_ids(&self) -> &bool;
    fn v_d_b_show_motion_ids(&self) -> &bool;
    fn v_d_b_show_broadphase(&self) -> &bool;
    fn v_d_b_show_shapes(&self) -> &bool;
    fn v_d_b_show_mass_properties(&self) -> &bool;
    fn v_d_b_show_constraints(&self) -> &bool;
    fn v_d_b_show_manifold(&self) -> &bool;
    fn v_d_b_show_motion_ttrail(&self) -> &bool;
    fn v_d_b_show_bounding_radius(&self) -> &bool;
    fn v_d_b_show_deactivation(&self) -> &bool;
    fn v_b_d_show_cell(&self) -> &bool;
    fn v_d_b_show_triangle_welding(&self) -> &bool;
    fn v_d_b_enable_world_snapshot(&self) -> &bool;
    fn v_d_b_show_destruction(&self) -> &bool;
}

impl PhysicsDebugSettingsTrait for PhysicsDebugSettings {
    fn debug_hinge_constraints(&self) -> &bool {
        &self.debug_hinge_constraints
    }
    fn use_physics_cpu_timers(&self) -> &bool {
        &self.use_physics_cpu_timers
    }
    fn timing_recursion_depth(&self) -> &u32 {
        &self.timing_recursion_depth
    }
    fn use_brute_force_broadphase(&self) -> &bool {
        &self.use_brute_force_broadphase
    }
    fn v_d_b_shows_effects_world(&self) -> &bool {
        &self.v_d_b_shows_effects_world
    }
    fn v_d_b_enable_havok_monitors(&self) -> &bool {
        &self.v_d_b_enable_havok_monitors
    }
    fn v_d_b_enable_capture_to_file(&self) -> &bool {
        &self.v_d_b_enable_capture_to_file
    }
    fn v_d_b_capture_file_name(&self) -> &String {
        &self.v_d_b_capture_file_name
    }
    fn v_d_b_connection_time_out(&self) -> &u32 {
        &self.v_d_b_connection_time_out
    }
    fn v_d_b_show_body_ids(&self) -> &bool {
        &self.v_d_b_show_body_ids
    }
    fn v_d_b_show_motion_ids(&self) -> &bool {
        &self.v_d_b_show_motion_ids
    }
    fn v_d_b_show_broadphase(&self) -> &bool {
        &self.v_d_b_show_broadphase
    }
    fn v_d_b_show_shapes(&self) -> &bool {
        &self.v_d_b_show_shapes
    }
    fn v_d_b_show_mass_properties(&self) -> &bool {
        &self.v_d_b_show_mass_properties
    }
    fn v_d_b_show_constraints(&self) -> &bool {
        &self.v_d_b_show_constraints
    }
    fn v_d_b_show_manifold(&self) -> &bool {
        &self.v_d_b_show_manifold
    }
    fn v_d_b_show_motion_ttrail(&self) -> &bool {
        &self.v_d_b_show_motion_ttrail
    }
    fn v_d_b_show_bounding_radius(&self) -> &bool {
        &self.v_d_b_show_bounding_radius
    }
    fn v_d_b_show_deactivation(&self) -> &bool {
        &self.v_d_b_show_deactivation
    }
    fn v_b_d_show_cell(&self) -> &bool {
        &self.v_b_d_show_cell
    }
    fn v_d_b_show_triangle_welding(&self) -> &bool {
        &self.v_d_b_show_triangle_welding
    }
    fn v_d_b_enable_world_snapshot(&self) -> &bool {
        &self.v_d_b_enable_world_snapshot
    }
    fn v_d_b_show_destruction(&self) -> &bool {
        &self.v_d_b_show_destruction
    }
}

impl super::core::DataContainerTrait for PhysicsDebugSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PHYSICSDEBUGSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsDebugSettings",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsDebugSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "DebugHingeConstraints",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsDebugSettings, debug_hinge_constraints),
            },
            FieldInfoData {
                name: "UsePhysicsCpuTimers",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsDebugSettings, use_physics_cpu_timers),
            },
            FieldInfoData {
                name: "TimingRecursionDepth",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(PhysicsDebugSettings, timing_recursion_depth),
            },
            FieldInfoData {
                name: "UseBruteForceBroadphase",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsDebugSettings, use_brute_force_broadphase),
            },
            FieldInfoData {
                name: "VDBShowsEffectsWorld",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsDebugSettings, v_d_b_shows_effects_world),
            },
            FieldInfoData {
                name: "VDBEnableHavokMonitors",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsDebugSettings, v_d_b_enable_havok_monitors),
            },
            FieldInfoData {
                name: "VDBEnableCaptureToFile",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsDebugSettings, v_d_b_enable_capture_to_file),
            },
            FieldInfoData {
                name: "VDBCaptureFileName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(PhysicsDebugSettings, v_d_b_capture_file_name),
            },
            FieldInfoData {
                name: "VDBConnectionTimeOut",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(PhysicsDebugSettings, v_d_b_connection_time_out),
            },
            FieldInfoData {
                name: "VDBShowBodyIds",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsDebugSettings, v_d_b_show_body_ids),
            },
            FieldInfoData {
                name: "VDBShowMotionIds",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsDebugSettings, v_d_b_show_motion_ids),
            },
            FieldInfoData {
                name: "VDBShowBroadphase",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsDebugSettings, v_d_b_show_broadphase),
            },
            FieldInfoData {
                name: "VDBShowShapes",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsDebugSettings, v_d_b_show_shapes),
            },
            FieldInfoData {
                name: "VDBShowMassProperties",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsDebugSettings, v_d_b_show_mass_properties),
            },
            FieldInfoData {
                name: "VDBShowConstraints",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsDebugSettings, v_d_b_show_constraints),
            },
            FieldInfoData {
                name: "VDBShowManifold",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsDebugSettings, v_d_b_show_manifold),
            },
            FieldInfoData {
                name: "VDBShowMotionTtrail",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsDebugSettings, v_d_b_show_motion_ttrail),
            },
            FieldInfoData {
                name: "VDBShowBoundingRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsDebugSettings, v_d_b_show_bounding_radius),
            },
            FieldInfoData {
                name: "VDBShowDeactivation",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsDebugSettings, v_d_b_show_deactivation),
            },
            FieldInfoData {
                name: "VBDShowCell",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsDebugSettings, v_b_d_show_cell),
            },
            FieldInfoData {
                name: "VDBShowTriangleWelding",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsDebugSettings, v_d_b_show_triangle_welding),
            },
            FieldInfoData {
                name: "VDBEnableWorldSnapshot",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsDebugSettings, v_d_b_enable_world_snapshot),
            },
            FieldInfoData {
                name: "VDBShowDestruction",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsDebugSettings, v_d_b_show_destruction),
            },
        ],
    }),
    array_type: Some(PHYSICSDEBUGSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PhysicsDebugSettings {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSDEBUGSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSDEBUGSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsDebugSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsDebugSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsSettings {
    pub _glacier_base: super::core::DataContainer,
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

pub trait PhysicsSettingsTrait: super::core::DataContainerTrait {
    fn enable(&self) -> &bool;
    fn run_client_simulation_single_threaded(&self) -> &bool;
    fn run_effect_simulation_single_threaded(&self) -> &bool;
    fn run_server_simulation_single_threaded(&self) -> &bool;
    fn wind_manager_awakening_radius(&self) -> &f32;
    fn enable_a_i_rigid_body(&self) -> &bool;
    fn forest_enable(&self) -> &bool;
    fn enable_jobs(&self) -> &bool;
    fn remove_ragdoll_when_woken(&self) -> &bool;
    fn remove_from_world_on_collision_overflow(&self) -> &bool;
    fn single_step_character(&self) -> &bool;
    fn force_single_step_character_in_s_p(&self) -> &bool;
    fn enable_follow_wheel_raycasts(&self) -> &bool;
    fn enable_client_wheel_raycasts(&self) -> &bool;
    fn enable_a_sync_wheel_raycasts(&self) -> &bool;
    fn use_delayed_wake_up_client(&self) -> &bool;
    fn use_delayed_wake_up_server(&self) -> &bool;
    fn suppress_debris_spawn_until_ready(&self) -> &bool;
    fn enable_player_v_s_a_i_collisions(&self) -> &bool;
    fn enable_client_keyframed_collisions(&self) -> &bool;
    fn enable_f_x_keyframed_collisions(&self) -> &bool;
    fn block_stream_allocator_memory(&self) -> &u32;
    fn step_local_stream_allocator(&self) -> &u32;
    fn body_buffer_capacity(&self) -> &u32;
    fn motion_buffer_capacity(&self) -> &u32;
    fn constraint_buffer_capacity(&self) -> &u32;
    fn havok_disable_free_list_allocator(&self) -> &bool;
    fn enable_remove_from_world_keep_active(&self) -> &bool;
    fn enable_physics_state_stream(&self) -> &bool;
}

impl PhysicsSettingsTrait for PhysicsSettings {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn run_client_simulation_single_threaded(&self) -> &bool {
        &self.run_client_simulation_single_threaded
    }
    fn run_effect_simulation_single_threaded(&self) -> &bool {
        &self.run_effect_simulation_single_threaded
    }
    fn run_server_simulation_single_threaded(&self) -> &bool {
        &self.run_server_simulation_single_threaded
    }
    fn wind_manager_awakening_radius(&self) -> &f32 {
        &self.wind_manager_awakening_radius
    }
    fn enable_a_i_rigid_body(&self) -> &bool {
        &self.enable_a_i_rigid_body
    }
    fn forest_enable(&self) -> &bool {
        &self.forest_enable
    }
    fn enable_jobs(&self) -> &bool {
        &self.enable_jobs
    }
    fn remove_ragdoll_when_woken(&self) -> &bool {
        &self.remove_ragdoll_when_woken
    }
    fn remove_from_world_on_collision_overflow(&self) -> &bool {
        &self.remove_from_world_on_collision_overflow
    }
    fn single_step_character(&self) -> &bool {
        &self.single_step_character
    }
    fn force_single_step_character_in_s_p(&self) -> &bool {
        &self.force_single_step_character_in_s_p
    }
    fn enable_follow_wheel_raycasts(&self) -> &bool {
        &self.enable_follow_wheel_raycasts
    }
    fn enable_client_wheel_raycasts(&self) -> &bool {
        &self.enable_client_wheel_raycasts
    }
    fn enable_a_sync_wheel_raycasts(&self) -> &bool {
        &self.enable_a_sync_wheel_raycasts
    }
    fn use_delayed_wake_up_client(&self) -> &bool {
        &self.use_delayed_wake_up_client
    }
    fn use_delayed_wake_up_server(&self) -> &bool {
        &self.use_delayed_wake_up_server
    }
    fn suppress_debris_spawn_until_ready(&self) -> &bool {
        &self.suppress_debris_spawn_until_ready
    }
    fn enable_player_v_s_a_i_collisions(&self) -> &bool {
        &self.enable_player_v_s_a_i_collisions
    }
    fn enable_client_keyframed_collisions(&self) -> &bool {
        &self.enable_client_keyframed_collisions
    }
    fn enable_f_x_keyframed_collisions(&self) -> &bool {
        &self.enable_f_x_keyframed_collisions
    }
    fn block_stream_allocator_memory(&self) -> &u32 {
        &self.block_stream_allocator_memory
    }
    fn step_local_stream_allocator(&self) -> &u32 {
        &self.step_local_stream_allocator
    }
    fn body_buffer_capacity(&self) -> &u32 {
        &self.body_buffer_capacity
    }
    fn motion_buffer_capacity(&self) -> &u32 {
        &self.motion_buffer_capacity
    }
    fn constraint_buffer_capacity(&self) -> &u32 {
        &self.constraint_buffer_capacity
    }
    fn havok_disable_free_list_allocator(&self) -> &bool {
        &self.havok_disable_free_list_allocator
    }
    fn enable_remove_from_world_keep_active(&self) -> &bool {
        &self.enable_remove_from_world_keep_active
    }
    fn enable_physics_state_stream(&self) -> &bool {
        &self.enable_physics_state_stream
    }
}

impl super::core::DataContainerTrait for PhysicsSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PHYSICSSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsSettings",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsSettings, enable),
            },
            FieldInfoData {
                name: "RunClientSimulationSingleThreaded",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsSettings, run_client_simulation_single_threaded),
            },
            FieldInfoData {
                name: "RunEffectSimulationSingleThreaded",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsSettings, run_effect_simulation_single_threaded),
            },
            FieldInfoData {
                name: "RunServerSimulationSingleThreaded",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsSettings, run_server_simulation_single_threaded),
            },
            FieldInfoData {
                name: "WindManagerAwakeningRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PhysicsSettings, wind_manager_awakening_radius),
            },
            FieldInfoData {
                name: "EnableAIRigidBody",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsSettings, enable_a_i_rigid_body),
            },
            FieldInfoData {
                name: "ForestEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsSettings, forest_enable),
            },
            FieldInfoData {
                name: "EnableJobs",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsSettings, enable_jobs),
            },
            FieldInfoData {
                name: "RemoveRagdollWhenWoken",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsSettings, remove_ragdoll_when_woken),
            },
            FieldInfoData {
                name: "RemoveFromWorldOnCollisionOverflow",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsSettings, remove_from_world_on_collision_overflow),
            },
            FieldInfoData {
                name: "SingleStepCharacter",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsSettings, single_step_character),
            },
            FieldInfoData {
                name: "ForceSingleStepCharacterInSP",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsSettings, force_single_step_character_in_s_p),
            },
            FieldInfoData {
                name: "EnableFollowWheelRaycasts",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsSettings, enable_follow_wheel_raycasts),
            },
            FieldInfoData {
                name: "EnableClientWheelRaycasts",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsSettings, enable_client_wheel_raycasts),
            },
            FieldInfoData {
                name: "EnableASyncWheelRaycasts",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsSettings, enable_a_sync_wheel_raycasts),
            },
            FieldInfoData {
                name: "UseDelayedWakeUpClient",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsSettings, use_delayed_wake_up_client),
            },
            FieldInfoData {
                name: "UseDelayedWakeUpServer",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsSettings, use_delayed_wake_up_server),
            },
            FieldInfoData {
                name: "SuppressDebrisSpawnUntilReady",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsSettings, suppress_debris_spawn_until_ready),
            },
            FieldInfoData {
                name: "EnablePlayerVSAICollisions",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsSettings, enable_player_v_s_a_i_collisions),
            },
            FieldInfoData {
                name: "EnableClientKeyframedCollisions",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsSettings, enable_client_keyframed_collisions),
            },
            FieldInfoData {
                name: "EnableFXKeyframedCollisions",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsSettings, enable_f_x_keyframed_collisions),
            },
            FieldInfoData {
                name: "BlockStreamAllocatorMemory",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(PhysicsSettings, block_stream_allocator_memory),
            },
            FieldInfoData {
                name: "StepLocalStreamAllocator",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(PhysicsSettings, step_local_stream_allocator),
            },
            FieldInfoData {
                name: "BodyBufferCapacity",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(PhysicsSettings, body_buffer_capacity),
            },
            FieldInfoData {
                name: "MotionBufferCapacity",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(PhysicsSettings, motion_buffer_capacity),
            },
            FieldInfoData {
                name: "ConstraintBufferCapacity",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(PhysicsSettings, constraint_buffer_capacity),
            },
            FieldInfoData {
                name: "HavokDisableFreeListAllocator",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsSettings, havok_disable_free_list_allocator),
            },
            FieldInfoData {
                name: "EnableRemoveFromWorldKeepActive",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsSettings, enable_remove_from_world_keep_active),
            },
            FieldInfoData {
                name: "EnablePhysicsStateStream",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsSettings, enable_physics_state_stream),
            },
        ],
    }),
    array_type: Some(PHYSICSSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PhysicsSettings {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum PhysicsWorldType {
    #[default]
    PhysicsWorldType_Client = 0,
    PhysicsWorldType_ClientEffect = 1,
    PhysicsWorldType_Server = 2,
    PhysicsWorldType_Max = 3,
}

pub static PHYSICSWORLDTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsWorldType",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(PHYSICSWORLDTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PhysicsWorldType {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSWORLDTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSWORLDTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsWorldType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsWorldType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsRenderSettings {
    pub _glacier_base: super::core::DataContainer,
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

pub trait PhysicsRenderSettingsTrait: super::core::DataContainerTrait {
    fn render_server(&self) -> &bool;
    fn render_client(&self) -> &bool;
    fn render_effect_world(&self) -> &bool;
    fn render_state_stream_world(&self) -> &bool;
    fn render_static(&self) -> &bool;
    fn render_detail(&self) -> &bool;
    fn render_group(&self) -> &bool;
    fn render_ungrouped(&self) -> &bool;
    fn render_ragdoll(&self) -> &bool;
    fn render_water(&self) -> &bool;
    fn render_terrain(&self) -> &bool;
    fn render_terrain_min_max_level(&self) -> &i32;
    fn render_characters(&self) -> &bool;
    fn render_aabb_trigger(&self) -> &bool;
    fn render_character_collision(&self) -> &bool;
    fn view_distance(&self) -> &f32;
    fn render_solid_geometry(&self) -> &bool;
    fn use_shape_cache(&self) -> &bool;
    fn depth_test(&self) -> &bool;
    fn render_constraints(&self) -> &bool;
    fn render_contacts(&self) -> &bool;
    fn render_only_contact_constraints(&self) -> &bool;
    fn render_simulation_islands(&self) -> &bool;
    fn render_broadphase_handles(&self) -> &bool;
    fn render_destruction_connections(&self) -> &bool;
    fn render_sleep_status(&self) -> &bool;
    fn render_quality_type(&self) -> &bool;
    fn render_part_bounding_boxes(&self) -> &bool;
    fn render_only_bounding_boxes(&self) -> &bool;
    fn render_rigid_body_transform(&self) -> &bool;
    fn render_inertia(&self) -> &bool;
    fn render_center_of_mass(&self) -> &bool;
    fn render_linear_velocity(&self) -> &bool;
    fn render_collision_spheres(&self) -> &bool;
    fn render_ray_queries(&self) -> &bool;
    fn render_shapes(&self) -> &bool;
    fn render_drive_targets(&self) -> &bool;
    fn render_entity_stats(&self) -> &bool;
    fn render_memory_used(&self) -> &bool;
    fn collision_spawn_debug(&self) -> &bool;
    fn render_specific_part(&self) -> &i32;
}

impl PhysicsRenderSettingsTrait for PhysicsRenderSettings {
    fn render_server(&self) -> &bool {
        &self.render_server
    }
    fn render_client(&self) -> &bool {
        &self.render_client
    }
    fn render_effect_world(&self) -> &bool {
        &self.render_effect_world
    }
    fn render_state_stream_world(&self) -> &bool {
        &self.render_state_stream_world
    }
    fn render_static(&self) -> &bool {
        &self.render_static
    }
    fn render_detail(&self) -> &bool {
        &self.render_detail
    }
    fn render_group(&self) -> &bool {
        &self.render_group
    }
    fn render_ungrouped(&self) -> &bool {
        &self.render_ungrouped
    }
    fn render_ragdoll(&self) -> &bool {
        &self.render_ragdoll
    }
    fn render_water(&self) -> &bool {
        &self.render_water
    }
    fn render_terrain(&self) -> &bool {
        &self.render_terrain
    }
    fn render_terrain_min_max_level(&self) -> &i32 {
        &self.render_terrain_min_max_level
    }
    fn render_characters(&self) -> &bool {
        &self.render_characters
    }
    fn render_aabb_trigger(&self) -> &bool {
        &self.render_aabb_trigger
    }
    fn render_character_collision(&self) -> &bool {
        &self.render_character_collision
    }
    fn view_distance(&self) -> &f32 {
        &self.view_distance
    }
    fn render_solid_geometry(&self) -> &bool {
        &self.render_solid_geometry
    }
    fn use_shape_cache(&self) -> &bool {
        &self.use_shape_cache
    }
    fn depth_test(&self) -> &bool {
        &self.depth_test
    }
    fn render_constraints(&self) -> &bool {
        &self.render_constraints
    }
    fn render_contacts(&self) -> &bool {
        &self.render_contacts
    }
    fn render_only_contact_constraints(&self) -> &bool {
        &self.render_only_contact_constraints
    }
    fn render_simulation_islands(&self) -> &bool {
        &self.render_simulation_islands
    }
    fn render_broadphase_handles(&self) -> &bool {
        &self.render_broadphase_handles
    }
    fn render_destruction_connections(&self) -> &bool {
        &self.render_destruction_connections
    }
    fn render_sleep_status(&self) -> &bool {
        &self.render_sleep_status
    }
    fn render_quality_type(&self) -> &bool {
        &self.render_quality_type
    }
    fn render_part_bounding_boxes(&self) -> &bool {
        &self.render_part_bounding_boxes
    }
    fn render_only_bounding_boxes(&self) -> &bool {
        &self.render_only_bounding_boxes
    }
    fn render_rigid_body_transform(&self) -> &bool {
        &self.render_rigid_body_transform
    }
    fn render_inertia(&self) -> &bool {
        &self.render_inertia
    }
    fn render_center_of_mass(&self) -> &bool {
        &self.render_center_of_mass
    }
    fn render_linear_velocity(&self) -> &bool {
        &self.render_linear_velocity
    }
    fn render_collision_spheres(&self) -> &bool {
        &self.render_collision_spheres
    }
    fn render_ray_queries(&self) -> &bool {
        &self.render_ray_queries
    }
    fn render_shapes(&self) -> &bool {
        &self.render_shapes
    }
    fn render_drive_targets(&self) -> &bool {
        &self.render_drive_targets
    }
    fn render_entity_stats(&self) -> &bool {
        &self.render_entity_stats
    }
    fn render_memory_used(&self) -> &bool {
        &self.render_memory_used
    }
    fn collision_spawn_debug(&self) -> &bool {
        &self.collision_spawn_debug
    }
    fn render_specific_part(&self) -> &i32 {
        &self.render_specific_part
    }
}

impl super::core::DataContainerTrait for PhysicsRenderSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PHYSICSRENDERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRenderSettings",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsRenderSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "RenderServer",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsRenderSettings, render_server),
            },
            FieldInfoData {
                name: "RenderClient",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsRenderSettings, render_client),
            },
            FieldInfoData {
                name: "RenderEffectWorld",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsRenderSettings, render_effect_world),
            },
            FieldInfoData {
                name: "RenderStateStreamWorld",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsRenderSettings, render_state_stream_world),
            },
            FieldInfoData {
                name: "RenderStatic",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsRenderSettings, render_static),
            },
            FieldInfoData {
                name: "RenderDetail",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsRenderSettings, render_detail),
            },
            FieldInfoData {
                name: "RenderGroup",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsRenderSettings, render_group),
            },
            FieldInfoData {
                name: "RenderUngrouped",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsRenderSettings, render_ungrouped),
            },
            FieldInfoData {
                name: "RenderRagdoll",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsRenderSettings, render_ragdoll),
            },
            FieldInfoData {
                name: "RenderWater",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsRenderSettings, render_water),
            },
            FieldInfoData {
                name: "RenderTerrain",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsRenderSettings, render_terrain),
            },
            FieldInfoData {
                name: "RenderTerrainMinMaxLevel",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PhysicsRenderSettings, render_terrain_min_max_level),
            },
            FieldInfoData {
                name: "RenderCharacters",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsRenderSettings, render_characters),
            },
            FieldInfoData {
                name: "RenderAabbTrigger",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsRenderSettings, render_aabb_trigger),
            },
            FieldInfoData {
                name: "RenderCharacterCollision",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsRenderSettings, render_character_collision),
            },
            FieldInfoData {
                name: "ViewDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PhysicsRenderSettings, view_distance),
            },
            FieldInfoData {
                name: "RenderSolidGeometry",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsRenderSettings, render_solid_geometry),
            },
            FieldInfoData {
                name: "UseShapeCache",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsRenderSettings, use_shape_cache),
            },
            FieldInfoData {
                name: "DepthTest",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsRenderSettings, depth_test),
            },
            FieldInfoData {
                name: "RenderConstraints",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsRenderSettings, render_constraints),
            },
            FieldInfoData {
                name: "RenderContacts",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsRenderSettings, render_contacts),
            },
            FieldInfoData {
                name: "RenderOnlyContactConstraints",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsRenderSettings, render_only_contact_constraints),
            },
            FieldInfoData {
                name: "RenderSimulationIslands",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsRenderSettings, render_simulation_islands),
            },
            FieldInfoData {
                name: "RenderBroadphaseHandles",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsRenderSettings, render_broadphase_handles),
            },
            FieldInfoData {
                name: "RenderDestructionConnections",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsRenderSettings, render_destruction_connections),
            },
            FieldInfoData {
                name: "RenderSleepStatus",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsRenderSettings, render_sleep_status),
            },
            FieldInfoData {
                name: "RenderQualityType",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsRenderSettings, render_quality_type),
            },
            FieldInfoData {
                name: "RenderPartBoundingBoxes",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsRenderSettings, render_part_bounding_boxes),
            },
            FieldInfoData {
                name: "RenderOnlyBoundingBoxes",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsRenderSettings, render_only_bounding_boxes),
            },
            FieldInfoData {
                name: "RenderRigidBodyTransform",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsRenderSettings, render_rigid_body_transform),
            },
            FieldInfoData {
                name: "RenderInertia",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsRenderSettings, render_inertia),
            },
            FieldInfoData {
                name: "RenderCenterOfMass",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsRenderSettings, render_center_of_mass),
            },
            FieldInfoData {
                name: "RenderLinearVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsRenderSettings, render_linear_velocity),
            },
            FieldInfoData {
                name: "RenderCollisionSpheres",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsRenderSettings, render_collision_spheres),
            },
            FieldInfoData {
                name: "RenderRayQueries",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsRenderSettings, render_ray_queries),
            },
            FieldInfoData {
                name: "RenderShapes",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsRenderSettings, render_shapes),
            },
            FieldInfoData {
                name: "RenderDriveTargets",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsRenderSettings, render_drive_targets),
            },
            FieldInfoData {
                name: "RenderEntityStats",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsRenderSettings, render_entity_stats),
            },
            FieldInfoData {
                name: "RenderMemoryUsed",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsRenderSettings, render_memory_used),
            },
            FieldInfoData {
                name: "CollisionSpawnDebug",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsRenderSettings, collision_spawn_debug),
            },
            FieldInfoData {
                name: "RenderSpecificPart",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PhysicsRenderSettings, render_specific_part),
            },
        ],
    }),
    array_type: Some(PHYSICSRENDERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PhysicsRenderSettings {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSRENDERSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSRENDERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRenderSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsRenderSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsComponentOnPreSolveCollisionMessage {
}

pub trait PhysicsComponentOnPreSolveCollisionMessageTrait: TypeObject {
}

impl PhysicsComponentOnPreSolveCollisionMessageTrait for PhysicsComponentOnPreSolveCollisionMessage {
}

pub static PHYSICSCOMPONENTONPRESOLVECOLLISIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsComponentOnPreSolveCollisionMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Physics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsComponentOnPreSolveCollisionMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for PhysicsComponentOnPreSolveCollisionMessage {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSCOMPONENTONPRESOLVECOLLISIONMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct PhysicsComponentOnImpulseMessage {
}

pub trait PhysicsComponentOnImpulseMessageTrait: TypeObject {
}

impl PhysicsComponentOnImpulseMessageTrait for PhysicsComponentOnImpulseMessage {
}

pub static PHYSICSCOMPONENTONIMPULSEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsComponentOnImpulseMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Physics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsComponentOnImpulseMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for PhysicsComponentOnImpulseMessage {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSCOMPONENTONIMPULSEMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct PhysicsPrismaticConstraintInitialStanceData {
    pub _glacier_base: PhysicsConstraintInitialStanceData,
}

pub trait PhysicsPrismaticConstraintInitialStanceDataTrait: PhysicsConstraintInitialStanceDataTrait {
}

impl PhysicsPrismaticConstraintInitialStanceDataTrait for PhysicsPrismaticConstraintInitialStanceData {
}

impl PhysicsConstraintInitialStanceDataTrait for PhysicsPrismaticConstraintInitialStanceData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
}

impl super::entity::EntityDataTrait for PhysicsPrismaticConstraintInitialStanceData {
}

impl super::entity::GameObjectDataTrait for PhysicsPrismaticConstraintInitialStanceData {
}

impl super::core::DataBusPeerTrait for PhysicsPrismaticConstraintInitialStanceData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for PhysicsPrismaticConstraintInitialStanceData {
}

impl super::core::DataContainerTrait for PhysicsPrismaticConstraintInitialStanceData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PHYSICSPRISMATICCONSTRAINTINITIALSTANCEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsPrismaticConstraintInitialStanceData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINTINITIALSTANCEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsPrismaticConstraintInitialStanceData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSPRISMATICCONSTRAINTINITIALSTANCEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsPrismaticConstraintInitialStanceData {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSPRISMATICCONSTRAINTINITIALSTANCEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSPRISMATICCONSTRAINTINITIALSTANCEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsPrismaticConstraintInitialStanceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsPrismaticConstraintInitialStanceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsRootControlConstraintInitialStanceData {
    pub _glacier_base: PhysicsConstraintInitialStanceData,
}

pub trait PhysicsRootControlConstraintInitialStanceDataTrait: PhysicsConstraintInitialStanceDataTrait {
}

impl PhysicsRootControlConstraintInitialStanceDataTrait for PhysicsRootControlConstraintInitialStanceData {
}

impl PhysicsConstraintInitialStanceDataTrait for PhysicsRootControlConstraintInitialStanceData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
}

impl super::entity::EntityDataTrait for PhysicsRootControlConstraintInitialStanceData {
}

impl super::entity::GameObjectDataTrait for PhysicsRootControlConstraintInitialStanceData {
}

impl super::core::DataBusPeerTrait for PhysicsRootControlConstraintInitialStanceData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for PhysicsRootControlConstraintInitialStanceData {
}

impl super::core::DataContainerTrait for PhysicsRootControlConstraintInitialStanceData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PHYSICSROOTCONTROLCONSTRAINTINITIALSTANCEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRootControlConstraintInitialStanceData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINTINITIALSTANCEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsRootControlConstraintInitialStanceData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSROOTCONTROLCONSTRAINTINITIALSTANCEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsRootControlConstraintInitialStanceData {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSROOTCONTROLCONSTRAINTINITIALSTANCEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSROOTCONTROLCONSTRAINTINITIALSTANCEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRootControlConstraintInitialStanceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsRootControlConstraintInitialStanceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsRagdollConstraintInitialStanceData {
    pub _glacier_base: PhysicsConstraintInitialStanceData,
}

pub trait PhysicsRagdollConstraintInitialStanceDataTrait: PhysicsConstraintInitialStanceDataTrait {
}

impl PhysicsRagdollConstraintInitialStanceDataTrait for PhysicsRagdollConstraintInitialStanceData {
}

impl PhysicsConstraintInitialStanceDataTrait for PhysicsRagdollConstraintInitialStanceData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
}

impl super::entity::EntityDataTrait for PhysicsRagdollConstraintInitialStanceData {
}

impl super::entity::GameObjectDataTrait for PhysicsRagdollConstraintInitialStanceData {
}

impl super::core::DataBusPeerTrait for PhysicsRagdollConstraintInitialStanceData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for PhysicsRagdollConstraintInitialStanceData {
}

impl super::core::DataContainerTrait for PhysicsRagdollConstraintInitialStanceData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PHYSICSRAGDOLLCONSTRAINTINITIALSTANCEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRagdollConstraintInitialStanceData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINTINITIALSTANCEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsRagdollConstraintInitialStanceData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSRAGDOLLCONSTRAINTINITIALSTANCEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsRagdollConstraintInitialStanceData {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSRAGDOLLCONSTRAINTINITIALSTANCEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSRAGDOLLCONSTRAINTINITIALSTANCEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRagdollConstraintInitialStanceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsRagdollConstraintInitialStanceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsHingeConstraintInitialStanceData {
    pub _glacier_base: PhysicsConstraintInitialStanceData,
}

pub trait PhysicsHingeConstraintInitialStanceDataTrait: PhysicsConstraintInitialStanceDataTrait {
}

impl PhysicsHingeConstraintInitialStanceDataTrait for PhysicsHingeConstraintInitialStanceData {
}

impl PhysicsConstraintInitialStanceDataTrait for PhysicsHingeConstraintInitialStanceData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
}

impl super::entity::EntityDataTrait for PhysicsHingeConstraintInitialStanceData {
}

impl super::entity::GameObjectDataTrait for PhysicsHingeConstraintInitialStanceData {
}

impl super::core::DataBusPeerTrait for PhysicsHingeConstraintInitialStanceData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for PhysicsHingeConstraintInitialStanceData {
}

impl super::core::DataContainerTrait for PhysicsHingeConstraintInitialStanceData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PHYSICSHINGECONSTRAINTINITIALSTANCEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsHingeConstraintInitialStanceData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINTINITIALSTANCEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsHingeConstraintInitialStanceData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSHINGECONSTRAINTINITIALSTANCEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsHingeConstraintInitialStanceData {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSHINGECONSTRAINTINITIALSTANCEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSHINGECONSTRAINTINITIALSTANCEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsHingeConstraintInitialStanceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsHingeConstraintInitialStanceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsBallAndSocketConstraintInitialStanceData {
    pub _glacier_base: PhysicsConstraintInitialStanceData,
}

pub trait PhysicsBallAndSocketConstraintInitialStanceDataTrait: PhysicsConstraintInitialStanceDataTrait {
}

impl PhysicsBallAndSocketConstraintInitialStanceDataTrait for PhysicsBallAndSocketConstraintInitialStanceData {
}

impl PhysicsConstraintInitialStanceDataTrait for PhysicsBallAndSocketConstraintInitialStanceData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
}

impl super::entity::EntityDataTrait for PhysicsBallAndSocketConstraintInitialStanceData {
}

impl super::entity::GameObjectDataTrait for PhysicsBallAndSocketConstraintInitialStanceData {
}

impl super::core::DataBusPeerTrait for PhysicsBallAndSocketConstraintInitialStanceData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for PhysicsBallAndSocketConstraintInitialStanceData {
}

impl super::core::DataContainerTrait for PhysicsBallAndSocketConstraintInitialStanceData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PHYSICSBALLANDSOCKETCONSTRAINTINITIALSTANCEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsBallAndSocketConstraintInitialStanceData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINTINITIALSTANCEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsBallAndSocketConstraintInitialStanceData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSBALLANDSOCKETCONSTRAINTINITIALSTANCEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsBallAndSocketConstraintInitialStanceData {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSBALLANDSOCKETCONSTRAINTINITIALSTANCEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSBALLANDSOCKETCONSTRAINTINITIALSTANCEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsBallAndSocketConstraintInitialStanceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsBallAndSocketConstraintInitialStanceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsConstraintInitialStanceData {
    pub _glacier_base: super::entity::EntityData,
    pub transform: super::core::LinearTransform,
}

pub trait PhysicsConstraintInitialStanceDataTrait: super::entity::EntityDataTrait {
    fn transform(&self) -> &super::core::LinearTransform;
}

impl PhysicsConstraintInitialStanceDataTrait for PhysicsConstraintInitialStanceData {
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
}

impl super::entity::EntityDataTrait for PhysicsConstraintInitialStanceData {
}

impl super::entity::GameObjectDataTrait for PhysicsConstraintInitialStanceData {
}

impl super::core::DataBusPeerTrait for PhysicsConstraintInitialStanceData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for PhysicsConstraintInitialStanceData {
}

impl super::core::DataContainerTrait for PhysicsConstraintInitialStanceData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PHYSICSCONSTRAINTINITIALSTANCEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsConstraintInitialStanceData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsConstraintInitialStanceData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(PhysicsConstraintInitialStanceData, transform),
            },
        ],
    }),
    array_type: Some(PHYSICSCONSTRAINTINITIALSTANCEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsConstraintInitialStanceData {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSCONSTRAINTINITIALSTANCEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSCONSTRAINTINITIALSTANCEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsConstraintInitialStanceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsConstraintInitialStanceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsPrismaticConstraintData {
    pub _glacier_base: PhysicsConstraintData,
    pub has_limits: bool,
    pub linear_limit: f32,
    pub motor_data: Option<Arc<Mutex<dyn PhysicsPrismaticConstraintMotorDataTrait>>>,
}

pub trait PhysicsPrismaticConstraintDataTrait: PhysicsConstraintDataTrait {
    fn has_limits(&self) -> &bool;
    fn linear_limit(&self) -> &f32;
    fn motor_data(&self) -> &Option<Arc<Mutex<dyn PhysicsPrismaticConstraintMotorDataTrait>>>;
}

impl PhysicsPrismaticConstraintDataTrait for PhysicsPrismaticConstraintData {
    fn has_limits(&self) -> &bool {
        &self.has_limits
    }
    fn linear_limit(&self) -> &f32 {
        &self.linear_limit
    }
    fn motor_data(&self) -> &Option<Arc<Mutex<dyn PhysicsPrismaticConstraintMotorDataTrait>>> {
        &self.motor_data
    }
}

impl PhysicsConstraintDataTrait for PhysicsPrismaticConstraintData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn is_breakable(&self) -> &bool {
        self._glacier_base.is_breakable()
    }
    fn break_threshold(&self) -> &f32 {
        self._glacier_base.break_threshold()
    }
    fn stabilized(&self) -> &bool {
        self._glacier_base.stabilized()
    }
    fn enable_continuous_simulation(&self) -> &bool {
        self._glacier_base.enable_continuous_simulation()
    }
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn world_index(&self) -> &u8 {
        self._glacier_base.world_index()
    }
    fn initial_stance_data(&self) -> &Option<Arc<Mutex<dyn PhysicsConstraintInitialStanceDataTrait>>> {
        self._glacier_base.initial_stance_data()
    }
}

impl super::entity::EntityDataTrait for PhysicsPrismaticConstraintData {
}

impl super::entity::GameObjectDataTrait for PhysicsPrismaticConstraintData {
}

impl super::core::DataBusPeerTrait for PhysicsPrismaticConstraintData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for PhysicsPrismaticConstraintData {
}

impl super::core::DataContainerTrait for PhysicsPrismaticConstraintData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PHYSICSPRISMATICCONSTRAINTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsPrismaticConstraintData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsPrismaticConstraintData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "HasLimits",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsPrismaticConstraintData, has_limits),
            },
            FieldInfoData {
                name: "LinearLimit",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PhysicsPrismaticConstraintData, linear_limit),
            },
            FieldInfoData {
                name: "MotorData",
                flags: MemberInfoFlags::new(0),
                field_type: "PhysicsPrismaticConstraintMotorData",
                rust_offset: offset_of!(PhysicsPrismaticConstraintData, motor_data),
            },
        ],
    }),
    array_type: Some(PHYSICSPRISMATICCONSTRAINTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsPrismaticConstraintData {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSPRISMATICCONSTRAINTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSPRISMATICCONSTRAINTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsPrismaticConstraintData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsPrismaticConstraintData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsRootControlConstraintData {
    pub _glacier_base: PhysicsConstraintData,
    pub motor_data: Option<Arc<Mutex<dyn PhysicsRootControlConstraintMotorDataTrait>>>,
}

pub trait PhysicsRootControlConstraintDataTrait: PhysicsConstraintDataTrait {
    fn motor_data(&self) -> &Option<Arc<Mutex<dyn PhysicsRootControlConstraintMotorDataTrait>>>;
}

impl PhysicsRootControlConstraintDataTrait for PhysicsRootControlConstraintData {
    fn motor_data(&self) -> &Option<Arc<Mutex<dyn PhysicsRootControlConstraintMotorDataTrait>>> {
        &self.motor_data
    }
}

impl PhysicsConstraintDataTrait for PhysicsRootControlConstraintData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn is_breakable(&self) -> &bool {
        self._glacier_base.is_breakable()
    }
    fn break_threshold(&self) -> &f32 {
        self._glacier_base.break_threshold()
    }
    fn stabilized(&self) -> &bool {
        self._glacier_base.stabilized()
    }
    fn enable_continuous_simulation(&self) -> &bool {
        self._glacier_base.enable_continuous_simulation()
    }
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn world_index(&self) -> &u8 {
        self._glacier_base.world_index()
    }
    fn initial_stance_data(&self) -> &Option<Arc<Mutex<dyn PhysicsConstraintInitialStanceDataTrait>>> {
        self._glacier_base.initial_stance_data()
    }
}

impl super::entity::EntityDataTrait for PhysicsRootControlConstraintData {
}

impl super::entity::GameObjectDataTrait for PhysicsRootControlConstraintData {
}

impl super::core::DataBusPeerTrait for PhysicsRootControlConstraintData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for PhysicsRootControlConstraintData {
}

impl super::core::DataContainerTrait for PhysicsRootControlConstraintData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PHYSICSROOTCONTROLCONSTRAINTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRootControlConstraintData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsRootControlConstraintData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MotorData",
                flags: MemberInfoFlags::new(0),
                field_type: "PhysicsRootControlConstraintMotorData",
                rust_offset: offset_of!(PhysicsRootControlConstraintData, motor_data),
            },
        ],
    }),
    array_type: Some(PHYSICSROOTCONTROLCONSTRAINTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsRootControlConstraintData {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSROOTCONTROLCONSTRAINTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSROOTCONTROLCONSTRAINTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRootControlConstraintData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsRootControlConstraintData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsRagdollConstraintData {
    pub _glacier_base: PhysicsConstraintData,
    pub has_limits: bool,
    pub twist_min_angular_limit: f32,
    pub twist_max_angular_limit: f32,
    pub plane_min_angular_limit: f32,
    pub plane_max_angular_limit: f32,
    pub cone_angular_limit: f32,
    pub angular_friction: f32,
    pub angular_stiffness: f32,
    pub motor_data: Option<Arc<Mutex<dyn PhysicsRagdollConstraintMotorDataTrait>>>,
}

pub trait PhysicsRagdollConstraintDataTrait: PhysicsConstraintDataTrait {
    fn has_limits(&self) -> &bool;
    fn twist_min_angular_limit(&self) -> &f32;
    fn twist_max_angular_limit(&self) -> &f32;
    fn plane_min_angular_limit(&self) -> &f32;
    fn plane_max_angular_limit(&self) -> &f32;
    fn cone_angular_limit(&self) -> &f32;
    fn angular_friction(&self) -> &f32;
    fn angular_stiffness(&self) -> &f32;
    fn motor_data(&self) -> &Option<Arc<Mutex<dyn PhysicsRagdollConstraintMotorDataTrait>>>;
}

impl PhysicsRagdollConstraintDataTrait for PhysicsRagdollConstraintData {
    fn has_limits(&self) -> &bool {
        &self.has_limits
    }
    fn twist_min_angular_limit(&self) -> &f32 {
        &self.twist_min_angular_limit
    }
    fn twist_max_angular_limit(&self) -> &f32 {
        &self.twist_max_angular_limit
    }
    fn plane_min_angular_limit(&self) -> &f32 {
        &self.plane_min_angular_limit
    }
    fn plane_max_angular_limit(&self) -> &f32 {
        &self.plane_max_angular_limit
    }
    fn cone_angular_limit(&self) -> &f32 {
        &self.cone_angular_limit
    }
    fn angular_friction(&self) -> &f32 {
        &self.angular_friction
    }
    fn angular_stiffness(&self) -> &f32 {
        &self.angular_stiffness
    }
    fn motor_data(&self) -> &Option<Arc<Mutex<dyn PhysicsRagdollConstraintMotorDataTrait>>> {
        &self.motor_data
    }
}

impl PhysicsConstraintDataTrait for PhysicsRagdollConstraintData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn is_breakable(&self) -> &bool {
        self._glacier_base.is_breakable()
    }
    fn break_threshold(&self) -> &f32 {
        self._glacier_base.break_threshold()
    }
    fn stabilized(&self) -> &bool {
        self._glacier_base.stabilized()
    }
    fn enable_continuous_simulation(&self) -> &bool {
        self._glacier_base.enable_continuous_simulation()
    }
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn world_index(&self) -> &u8 {
        self._glacier_base.world_index()
    }
    fn initial_stance_data(&self) -> &Option<Arc<Mutex<dyn PhysicsConstraintInitialStanceDataTrait>>> {
        self._glacier_base.initial_stance_data()
    }
}

impl super::entity::EntityDataTrait for PhysicsRagdollConstraintData {
}

impl super::entity::GameObjectDataTrait for PhysicsRagdollConstraintData {
}

impl super::core::DataBusPeerTrait for PhysicsRagdollConstraintData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for PhysicsRagdollConstraintData {
}

impl super::core::DataContainerTrait for PhysicsRagdollConstraintData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PHYSICSRAGDOLLCONSTRAINTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRagdollConstraintData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsRagdollConstraintData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "HasLimits",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsRagdollConstraintData, has_limits),
            },
            FieldInfoData {
                name: "TwistMinAngularLimit",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PhysicsRagdollConstraintData, twist_min_angular_limit),
            },
            FieldInfoData {
                name: "TwistMaxAngularLimit",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PhysicsRagdollConstraintData, twist_max_angular_limit),
            },
            FieldInfoData {
                name: "PlaneMinAngularLimit",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PhysicsRagdollConstraintData, plane_min_angular_limit),
            },
            FieldInfoData {
                name: "PlaneMaxAngularLimit",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PhysicsRagdollConstraintData, plane_max_angular_limit),
            },
            FieldInfoData {
                name: "ConeAngularLimit",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PhysicsRagdollConstraintData, cone_angular_limit),
            },
            FieldInfoData {
                name: "AngularFriction",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PhysicsRagdollConstraintData, angular_friction),
            },
            FieldInfoData {
                name: "AngularStiffness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PhysicsRagdollConstraintData, angular_stiffness),
            },
            FieldInfoData {
                name: "MotorData",
                flags: MemberInfoFlags::new(0),
                field_type: "PhysicsRagdollConstraintMotorData",
                rust_offset: offset_of!(PhysicsRagdollConstraintData, motor_data),
            },
        ],
    }),
    array_type: Some(PHYSICSRAGDOLLCONSTRAINTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsRagdollConstraintData {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSRAGDOLLCONSTRAINTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSRAGDOLLCONSTRAINTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRagdollConstraintData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsRagdollConstraintData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsHingeConstraintData {
    pub _glacier_base: PhysicsConstraintData,
    pub has_limits: bool,
    pub min_angle: f32,
    pub max_angle: f32,
    pub angular_friction: f32,
    pub angular_stiffness: f32,
    pub motor_data: Option<Arc<Mutex<dyn PhysicsHingeConstraintMotorDataTrait>>>,
}

pub trait PhysicsHingeConstraintDataTrait: PhysicsConstraintDataTrait {
    fn has_limits(&self) -> &bool;
    fn min_angle(&self) -> &f32;
    fn max_angle(&self) -> &f32;
    fn angular_friction(&self) -> &f32;
    fn angular_stiffness(&self) -> &f32;
    fn motor_data(&self) -> &Option<Arc<Mutex<dyn PhysicsHingeConstraintMotorDataTrait>>>;
}

impl PhysicsHingeConstraintDataTrait for PhysicsHingeConstraintData {
    fn has_limits(&self) -> &bool {
        &self.has_limits
    }
    fn min_angle(&self) -> &f32 {
        &self.min_angle
    }
    fn max_angle(&self) -> &f32 {
        &self.max_angle
    }
    fn angular_friction(&self) -> &f32 {
        &self.angular_friction
    }
    fn angular_stiffness(&self) -> &f32 {
        &self.angular_stiffness
    }
    fn motor_data(&self) -> &Option<Arc<Mutex<dyn PhysicsHingeConstraintMotorDataTrait>>> {
        &self.motor_data
    }
}

impl PhysicsConstraintDataTrait for PhysicsHingeConstraintData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn is_breakable(&self) -> &bool {
        self._glacier_base.is_breakable()
    }
    fn break_threshold(&self) -> &f32 {
        self._glacier_base.break_threshold()
    }
    fn stabilized(&self) -> &bool {
        self._glacier_base.stabilized()
    }
    fn enable_continuous_simulation(&self) -> &bool {
        self._glacier_base.enable_continuous_simulation()
    }
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn world_index(&self) -> &u8 {
        self._glacier_base.world_index()
    }
    fn initial_stance_data(&self) -> &Option<Arc<Mutex<dyn PhysicsConstraintInitialStanceDataTrait>>> {
        self._glacier_base.initial_stance_data()
    }
}

impl super::entity::EntityDataTrait for PhysicsHingeConstraintData {
}

impl super::entity::GameObjectDataTrait for PhysicsHingeConstraintData {
}

impl super::core::DataBusPeerTrait for PhysicsHingeConstraintData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for PhysicsHingeConstraintData {
}

impl super::core::DataContainerTrait for PhysicsHingeConstraintData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PHYSICSHINGECONSTRAINTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsHingeConstraintData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsHingeConstraintData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "HasLimits",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsHingeConstraintData, has_limits),
            },
            FieldInfoData {
                name: "MinAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PhysicsHingeConstraintData, min_angle),
            },
            FieldInfoData {
                name: "MaxAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PhysicsHingeConstraintData, max_angle),
            },
            FieldInfoData {
                name: "AngularFriction",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PhysicsHingeConstraintData, angular_friction),
            },
            FieldInfoData {
                name: "AngularStiffness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PhysicsHingeConstraintData, angular_stiffness),
            },
            FieldInfoData {
                name: "MotorData",
                flags: MemberInfoFlags::new(0),
                field_type: "PhysicsHingeConstraintMotorData",
                rust_offset: offset_of!(PhysicsHingeConstraintData, motor_data),
            },
        ],
    }),
    array_type: Some(PHYSICSHINGECONSTRAINTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsHingeConstraintData {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSHINGECONSTRAINTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSHINGECONSTRAINTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsHingeConstraintData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsHingeConstraintData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsBallAndSocketConstraintData {
    pub _glacier_base: PhysicsConstraintData,
}

pub trait PhysicsBallAndSocketConstraintDataTrait: PhysicsConstraintDataTrait {
}

impl PhysicsBallAndSocketConstraintDataTrait for PhysicsBallAndSocketConstraintData {
}

impl PhysicsConstraintDataTrait for PhysicsBallAndSocketConstraintData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn is_breakable(&self) -> &bool {
        self._glacier_base.is_breakable()
    }
    fn break_threshold(&self) -> &f32 {
        self._glacier_base.break_threshold()
    }
    fn stabilized(&self) -> &bool {
        self._glacier_base.stabilized()
    }
    fn enable_continuous_simulation(&self) -> &bool {
        self._glacier_base.enable_continuous_simulation()
    }
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn world_index(&self) -> &u8 {
        self._glacier_base.world_index()
    }
    fn initial_stance_data(&self) -> &Option<Arc<Mutex<dyn PhysicsConstraintInitialStanceDataTrait>>> {
        self._glacier_base.initial_stance_data()
    }
}

impl super::entity::EntityDataTrait for PhysicsBallAndSocketConstraintData {
}

impl super::entity::GameObjectDataTrait for PhysicsBallAndSocketConstraintData {
}

impl super::core::DataBusPeerTrait for PhysicsBallAndSocketConstraintData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for PhysicsBallAndSocketConstraintData {
}

impl super::core::DataContainerTrait for PhysicsBallAndSocketConstraintData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PHYSICSBALLANDSOCKETCONSTRAINTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsBallAndSocketConstraintData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsBallAndSocketConstraintData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSBALLANDSOCKETCONSTRAINTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsBallAndSocketConstraintData {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSBALLANDSOCKETCONSTRAINTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSBALLANDSOCKETCONSTRAINTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsBallAndSocketConstraintData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsBallAndSocketConstraintData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsPrismaticConstraintMotorData {
    pub _glacier_base: PhysicsConstraintMotorData,
    pub max_force: f32,
}

pub trait PhysicsPrismaticConstraintMotorDataTrait: PhysicsConstraintMotorDataTrait {
    fn max_force(&self) -> &f32;
}

impl PhysicsPrismaticConstraintMotorDataTrait for PhysicsPrismaticConstraintMotorData {
    fn max_force(&self) -> &f32 {
        &self.max_force
    }
}

impl PhysicsConstraintMotorDataTrait for PhysicsPrismaticConstraintMotorData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn spring_strength(&self) -> &f32 {
        self._glacier_base.spring_strength()
    }
    fn damping(&self) -> &f32 {
        self._glacier_base.damping()
    }
}

impl super::entity::EntityDataTrait for PhysicsPrismaticConstraintMotorData {
}

impl super::entity::GameObjectDataTrait for PhysicsPrismaticConstraintMotorData {
}

impl super::core::DataBusPeerTrait for PhysicsPrismaticConstraintMotorData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for PhysicsPrismaticConstraintMotorData {
}

impl super::core::DataContainerTrait for PhysicsPrismaticConstraintMotorData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PHYSICSPRISMATICCONSTRAINTMOTORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsPrismaticConstraintMotorData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINTMOTORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsPrismaticConstraintMotorData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MaxForce",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PhysicsPrismaticConstraintMotorData, max_force),
            },
        ],
    }),
    array_type: Some(PHYSICSPRISMATICCONSTRAINTMOTORDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsPrismaticConstraintMotorData {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSPRISMATICCONSTRAINTMOTORDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSPRISMATICCONSTRAINTMOTORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsPrismaticConstraintMotorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsPrismaticConstraintMotorData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsRootControlConstraintMotorData {
    pub _glacier_base: PhysicsConstraintMotorData,
    pub forward_max_force: f32,
    pub backward_max_force: f32,
    pub radial_max_force: f32,
    pub swing_max_torque: f32,
    pub twist_max_torque: f32,
}

pub trait PhysicsRootControlConstraintMotorDataTrait: PhysicsConstraintMotorDataTrait {
    fn forward_max_force(&self) -> &f32;
    fn backward_max_force(&self) -> &f32;
    fn radial_max_force(&self) -> &f32;
    fn swing_max_torque(&self) -> &f32;
    fn twist_max_torque(&self) -> &f32;
}

impl PhysicsRootControlConstraintMotorDataTrait for PhysicsRootControlConstraintMotorData {
    fn forward_max_force(&self) -> &f32 {
        &self.forward_max_force
    }
    fn backward_max_force(&self) -> &f32 {
        &self.backward_max_force
    }
    fn radial_max_force(&self) -> &f32 {
        &self.radial_max_force
    }
    fn swing_max_torque(&self) -> &f32 {
        &self.swing_max_torque
    }
    fn twist_max_torque(&self) -> &f32 {
        &self.twist_max_torque
    }
}

impl PhysicsConstraintMotorDataTrait for PhysicsRootControlConstraintMotorData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn spring_strength(&self) -> &f32 {
        self._glacier_base.spring_strength()
    }
    fn damping(&self) -> &f32 {
        self._glacier_base.damping()
    }
}

impl super::entity::EntityDataTrait for PhysicsRootControlConstraintMotorData {
}

impl super::entity::GameObjectDataTrait for PhysicsRootControlConstraintMotorData {
}

impl super::core::DataBusPeerTrait for PhysicsRootControlConstraintMotorData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for PhysicsRootControlConstraintMotorData {
}

impl super::core::DataContainerTrait for PhysicsRootControlConstraintMotorData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PHYSICSROOTCONTROLCONSTRAINTMOTORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRootControlConstraintMotorData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINTMOTORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsRootControlConstraintMotorData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ForwardMaxForce",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PhysicsRootControlConstraintMotorData, forward_max_force),
            },
            FieldInfoData {
                name: "BackwardMaxForce",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PhysicsRootControlConstraintMotorData, backward_max_force),
            },
            FieldInfoData {
                name: "RadialMaxForce",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PhysicsRootControlConstraintMotorData, radial_max_force),
            },
            FieldInfoData {
                name: "SwingMaxTorque",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PhysicsRootControlConstraintMotorData, swing_max_torque),
            },
            FieldInfoData {
                name: "TwistMaxTorque",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PhysicsRootControlConstraintMotorData, twist_max_torque),
            },
        ],
    }),
    array_type: Some(PHYSICSROOTCONTROLCONSTRAINTMOTORDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsRootControlConstraintMotorData {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSROOTCONTROLCONSTRAINTMOTORDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSROOTCONTROLCONSTRAINTMOTORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRootControlConstraintMotorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsRootControlConstraintMotorData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsRagdollConstraintMotorData {
    pub _glacier_base: PhysicsConstraintMotorData,
    pub swing_max_torque: f32,
    pub twist_max_torque: f32,
}

pub trait PhysicsRagdollConstraintMotorDataTrait: PhysicsConstraintMotorDataTrait {
    fn swing_max_torque(&self) -> &f32;
    fn twist_max_torque(&self) -> &f32;
}

impl PhysicsRagdollConstraintMotorDataTrait for PhysicsRagdollConstraintMotorData {
    fn swing_max_torque(&self) -> &f32 {
        &self.swing_max_torque
    }
    fn twist_max_torque(&self) -> &f32 {
        &self.twist_max_torque
    }
}

impl PhysicsConstraintMotorDataTrait for PhysicsRagdollConstraintMotorData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn spring_strength(&self) -> &f32 {
        self._glacier_base.spring_strength()
    }
    fn damping(&self) -> &f32 {
        self._glacier_base.damping()
    }
}

impl super::entity::EntityDataTrait for PhysicsRagdollConstraintMotorData {
}

impl super::entity::GameObjectDataTrait for PhysicsRagdollConstraintMotorData {
}

impl super::core::DataBusPeerTrait for PhysicsRagdollConstraintMotorData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for PhysicsRagdollConstraintMotorData {
}

impl super::core::DataContainerTrait for PhysicsRagdollConstraintMotorData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PHYSICSRAGDOLLCONSTRAINTMOTORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRagdollConstraintMotorData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINTMOTORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsRagdollConstraintMotorData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "SwingMaxTorque",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PhysicsRagdollConstraintMotorData, swing_max_torque),
            },
            FieldInfoData {
                name: "TwistMaxTorque",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PhysicsRagdollConstraintMotorData, twist_max_torque),
            },
        ],
    }),
    array_type: Some(PHYSICSRAGDOLLCONSTRAINTMOTORDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsRagdollConstraintMotorData {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSRAGDOLLCONSTRAINTMOTORDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSRAGDOLLCONSTRAINTMOTORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRagdollConstraintMotorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsRagdollConstraintMotorData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsHingeConstraintMotorData {
    pub _glacier_base: PhysicsConstraintMotorData,
    pub max_torque: f32,
}

pub trait PhysicsHingeConstraintMotorDataTrait: PhysicsConstraintMotorDataTrait {
    fn max_torque(&self) -> &f32;
}

impl PhysicsHingeConstraintMotorDataTrait for PhysicsHingeConstraintMotorData {
    fn max_torque(&self) -> &f32 {
        &self.max_torque
    }
}

impl PhysicsConstraintMotorDataTrait for PhysicsHingeConstraintMotorData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn spring_strength(&self) -> &f32 {
        self._glacier_base.spring_strength()
    }
    fn damping(&self) -> &f32 {
        self._glacier_base.damping()
    }
}

impl super::entity::EntityDataTrait for PhysicsHingeConstraintMotorData {
}

impl super::entity::GameObjectDataTrait for PhysicsHingeConstraintMotorData {
}

impl super::core::DataBusPeerTrait for PhysicsHingeConstraintMotorData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for PhysicsHingeConstraintMotorData {
}

impl super::core::DataContainerTrait for PhysicsHingeConstraintMotorData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PHYSICSHINGECONSTRAINTMOTORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsHingeConstraintMotorData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINTMOTORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsHingeConstraintMotorData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MaxTorque",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PhysicsHingeConstraintMotorData, max_torque),
            },
        ],
    }),
    array_type: Some(PHYSICSHINGECONSTRAINTMOTORDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsHingeConstraintMotorData {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSHINGECONSTRAINTMOTORDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSHINGECONSTRAINTMOTORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsHingeConstraintMotorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsHingeConstraintMotorData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsConstraintMotorData {
    pub _glacier_base: super::entity::EntityData,
    pub transform: super::core::LinearTransform,
    pub spring_strength: f32,
    pub damping: f32,
}

pub trait PhysicsConstraintMotorDataTrait: super::entity::EntityDataTrait {
    fn transform(&self) -> &super::core::LinearTransform;
    fn spring_strength(&self) -> &f32;
    fn damping(&self) -> &f32;
}

impl PhysicsConstraintMotorDataTrait for PhysicsConstraintMotorData {
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
    fn spring_strength(&self) -> &f32 {
        &self.spring_strength
    }
    fn damping(&self) -> &f32 {
        &self.damping
    }
}

impl super::entity::EntityDataTrait for PhysicsConstraintMotorData {
}

impl super::entity::GameObjectDataTrait for PhysicsConstraintMotorData {
}

impl super::core::DataBusPeerTrait for PhysicsConstraintMotorData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for PhysicsConstraintMotorData {
}

impl super::core::DataContainerTrait for PhysicsConstraintMotorData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PHYSICSCONSTRAINTMOTORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsConstraintMotorData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsConstraintMotorData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(PhysicsConstraintMotorData, transform),
            },
            FieldInfoData {
                name: "SpringStrength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PhysicsConstraintMotorData, spring_strength),
            },
            FieldInfoData {
                name: "Damping",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PhysicsConstraintMotorData, damping),
            },
        ],
    }),
    array_type: Some(PHYSICSCONSTRAINTMOTORDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsConstraintMotorData {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSCONSTRAINTMOTORDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSCONSTRAINTMOTORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsConstraintMotorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsConstraintMotorData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsConstraintOwnerData {
    pub _glacier_base: super::entity::EntityData,
    pub constraint_data: Option<Arc<Mutex<dyn PhysicsConstraintDataTrait>>>,
}

pub trait PhysicsConstraintOwnerDataTrait: super::entity::EntityDataTrait {
    fn constraint_data(&self) -> &Option<Arc<Mutex<dyn PhysicsConstraintDataTrait>>>;
}

impl PhysicsConstraintOwnerDataTrait for PhysicsConstraintOwnerData {
    fn constraint_data(&self) -> &Option<Arc<Mutex<dyn PhysicsConstraintDataTrait>>> {
        &self.constraint_data
    }
}

impl super::entity::EntityDataTrait for PhysicsConstraintOwnerData {
}

impl super::entity::GameObjectDataTrait for PhysicsConstraintOwnerData {
}

impl super::core::DataBusPeerTrait for PhysicsConstraintOwnerData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for PhysicsConstraintOwnerData {
}

impl super::core::DataContainerTrait for PhysicsConstraintOwnerData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PHYSICSCONSTRAINTOWNERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsConstraintOwnerData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsConstraintOwnerData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ConstraintData",
                flags: MemberInfoFlags::new(0),
                field_type: "PhysicsConstraintData",
                rust_offset: offset_of!(PhysicsConstraintOwnerData, constraint_data),
            },
        ],
    }),
    array_type: Some(PHYSICSCONSTRAINTOWNERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PhysicsConstraintOwnerData {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSCONSTRAINTOWNERDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSCONSTRAINTOWNERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsConstraintOwnerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsConstraintOwnerData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsConstraintData {
    pub _glacier_base: super::entity::EntityData,
    pub transform: super::core::LinearTransform,
    pub is_breakable: bool,
    pub break_threshold: f32,
    pub stabilized: bool,
    pub enable_continuous_simulation: bool,
    pub realm: super::core::Realm,
    pub world_index: u8,
    pub initial_stance_data: Option<Arc<Mutex<dyn PhysicsConstraintInitialStanceDataTrait>>>,
}

pub trait PhysicsConstraintDataTrait: super::entity::EntityDataTrait {
    fn transform(&self) -> &super::core::LinearTransform;
    fn is_breakable(&self) -> &bool;
    fn break_threshold(&self) -> &f32;
    fn stabilized(&self) -> &bool;
    fn enable_continuous_simulation(&self) -> &bool;
    fn realm(&self) -> &super::core::Realm;
    fn world_index(&self) -> &u8;
    fn initial_stance_data(&self) -> &Option<Arc<Mutex<dyn PhysicsConstraintInitialStanceDataTrait>>>;
}

impl PhysicsConstraintDataTrait for PhysicsConstraintData {
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
    fn is_breakable(&self) -> &bool {
        &self.is_breakable
    }
    fn break_threshold(&self) -> &f32 {
        &self.break_threshold
    }
    fn stabilized(&self) -> &bool {
        &self.stabilized
    }
    fn enable_continuous_simulation(&self) -> &bool {
        &self.enable_continuous_simulation
    }
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn world_index(&self) -> &u8 {
        &self.world_index
    }
    fn initial_stance_data(&self) -> &Option<Arc<Mutex<dyn PhysicsConstraintInitialStanceDataTrait>>> {
        &self.initial_stance_data
    }
}

impl super::entity::EntityDataTrait for PhysicsConstraintData {
}

impl super::entity::GameObjectDataTrait for PhysicsConstraintData {
}

impl super::core::DataBusPeerTrait for PhysicsConstraintData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for PhysicsConstraintData {
}

impl super::core::DataContainerTrait for PhysicsConstraintData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PHYSICSCONSTRAINTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsConstraintData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsConstraintData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(PhysicsConstraintData, transform),
            },
            FieldInfoData {
                name: "IsBreakable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsConstraintData, is_breakable),
            },
            FieldInfoData {
                name: "BreakThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PhysicsConstraintData, break_threshold),
            },
            FieldInfoData {
                name: "Stabilized",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsConstraintData, stabilized),
            },
            FieldInfoData {
                name: "EnableContinuousSimulation",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsConstraintData, enable_continuous_simulation),
            },
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(PhysicsConstraintData, realm),
            },
            FieldInfoData {
                name: "WorldIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(PhysicsConstraintData, world_index),
            },
            FieldInfoData {
                name: "InitialStanceData",
                flags: MemberInfoFlags::new(0),
                field_type: "PhysicsConstraintInitialStanceData",
                rust_offset: offset_of!(PhysicsConstraintData, initial_stance_data),
            },
        ],
    }),
    array_type: Some(PHYSICSCONSTRAINTDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PhysicsConstraintData {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSCONSTRAINTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSCONSTRAINTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsConstraintData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsConstraintData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ProximityTrackerData {
    pub _glacier_base: super::core::DataContainer,
    pub half_extents: super::core::Vec3,
    pub collision_layer: u32,
}

pub trait ProximityTrackerDataTrait: super::core::DataContainerTrait {
    fn half_extents(&self) -> &super::core::Vec3;
    fn collision_layer(&self) -> &u32;
}

impl ProximityTrackerDataTrait for ProximityTrackerData {
    fn half_extents(&self) -> &super::core::Vec3 {
        &self.half_extents
    }
    fn collision_layer(&self) -> &u32 {
        &self.collision_layer
    }
}

impl super::core::DataContainerTrait for ProximityTrackerData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PROXIMITYTRACKERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProximityTrackerData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ProximityTrackerData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "HalfExtents",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(ProximityTrackerData, half_extents),
            },
            FieldInfoData {
                name: "CollisionLayer",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ProximityTrackerData, collision_layer),
            },
        ],
    }),
    array_type: Some(PROXIMITYTRACKERDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ProximityTrackerData {
    fn type_info(&self) -> &'static TypeInfo {
        PROXIMITYTRACKERDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PROXIMITYTRACKERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProximityTrackerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ProximityTrackerData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AabbTriggerPhysicsBodyData {
    pub _glacier_base: PhysicsBodyData,
    pub half_extents: super::core::Vec3,
}

pub trait AabbTriggerPhysicsBodyDataTrait: PhysicsBodyDataTrait {
    fn half_extents(&self) -> &super::core::Vec3;
}

impl AabbTriggerPhysicsBodyDataTrait for AabbTriggerPhysicsBodyData {
    fn half_extents(&self) -> &super::core::Vec3 {
        &self.half_extents
    }
}

impl PhysicsBodyDataTrait for AabbTriggerPhysicsBodyData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn rigid_body_type(&self) -> &RigidBodyType {
        self._glacier_base.rigid_body_type()
    }
    fn collision_layer(&self) -> &RigidBodyCollisionLayer {
        self._glacier_base.collision_layer()
    }
    fn motion_type(&self) -> &RigidBodyMotionType {
        self._glacier_base.motion_type()
    }
    fn quality_type(&self) -> &RigidBodyQualityType {
        self._glacier_base.quality_type()
    }
    fn asset(&self) -> &Option<Arc<Mutex<dyn super::core::AssetTrait>>> {
        self._glacier_base.asset()
    }
    fn transform_index(&self) -> &u8 {
        self._glacier_base.transform_index()
    }
    fn world_index(&self) -> &u8 {
        self._glacier_base.world_index()
    }
    fn collision_root_shape_index(&self) -> &u8 {
        self._glacier_base.collision_root_shape_index()
    }
    fn raycast_root_shape_index(&self) -> &u8 {
        self._glacier_base.raycast_root_shape_index()
    }
    fn add_to_spatial_query_manager(&self) -> &bool {
        self._glacier_base.add_to_spatial_query_manager()
    }
    fn physics_callback_handler(&self) -> &Option<Arc<Mutex<dyn PhysicsCallbackHandlerDataTrait>>> {
        self._glacier_base.physics_callback_handler()
    }
}

impl super::entity::EntityDataTrait for AabbTriggerPhysicsBodyData {
}

impl super::entity::GameObjectDataTrait for AabbTriggerPhysicsBodyData {
}

impl super::core::DataBusPeerTrait for AabbTriggerPhysicsBodyData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for AabbTriggerPhysicsBodyData {
}

impl super::core::DataContainerTrait for AabbTriggerPhysicsBodyData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static AABBTRIGGERPHYSICSBODYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AabbTriggerPhysicsBodyData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSBODYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AabbTriggerPhysicsBodyData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "HalfExtents",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AabbTriggerPhysicsBodyData, half_extents),
            },
        ],
    }),
    array_type: Some(AABBTRIGGERPHYSICSBODYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AabbTriggerPhysicsBodyData {
    fn type_info(&self) -> &'static TypeInfo {
        AABBTRIGGERPHYSICSBODYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static AABBTRIGGERPHYSICSBODYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AabbTriggerPhysicsBodyData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("AabbTriggerPhysicsBodyData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TerrainPhysicsBodyData {
    pub _glacier_base: PhysicsBodyData,
    pub use_s_d_f_collision: bool,
    pub rigid_body_index: u32,
    pub max_resolution_size_use_for_split: f32,
    pub s_d_f_thickness: f32,
}

pub trait TerrainPhysicsBodyDataTrait: PhysicsBodyDataTrait {
    fn use_s_d_f_collision(&self) -> &bool;
    fn rigid_body_index(&self) -> &u32;
    fn max_resolution_size_use_for_split(&self) -> &f32;
    fn s_d_f_thickness(&self) -> &f32;
}

impl TerrainPhysicsBodyDataTrait for TerrainPhysicsBodyData {
    fn use_s_d_f_collision(&self) -> &bool {
        &self.use_s_d_f_collision
    }
    fn rigid_body_index(&self) -> &u32 {
        &self.rigid_body_index
    }
    fn max_resolution_size_use_for_split(&self) -> &f32 {
        &self.max_resolution_size_use_for_split
    }
    fn s_d_f_thickness(&self) -> &f32 {
        &self.s_d_f_thickness
    }
}

impl PhysicsBodyDataTrait for TerrainPhysicsBodyData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn rigid_body_type(&self) -> &RigidBodyType {
        self._glacier_base.rigid_body_type()
    }
    fn collision_layer(&self) -> &RigidBodyCollisionLayer {
        self._glacier_base.collision_layer()
    }
    fn motion_type(&self) -> &RigidBodyMotionType {
        self._glacier_base.motion_type()
    }
    fn quality_type(&self) -> &RigidBodyQualityType {
        self._glacier_base.quality_type()
    }
    fn asset(&self) -> &Option<Arc<Mutex<dyn super::core::AssetTrait>>> {
        self._glacier_base.asset()
    }
    fn transform_index(&self) -> &u8 {
        self._glacier_base.transform_index()
    }
    fn world_index(&self) -> &u8 {
        self._glacier_base.world_index()
    }
    fn collision_root_shape_index(&self) -> &u8 {
        self._glacier_base.collision_root_shape_index()
    }
    fn raycast_root_shape_index(&self) -> &u8 {
        self._glacier_base.raycast_root_shape_index()
    }
    fn add_to_spatial_query_manager(&self) -> &bool {
        self._glacier_base.add_to_spatial_query_manager()
    }
    fn physics_callback_handler(&self) -> &Option<Arc<Mutex<dyn PhysicsCallbackHandlerDataTrait>>> {
        self._glacier_base.physics_callback_handler()
    }
}

impl super::entity::EntityDataTrait for TerrainPhysicsBodyData {
}

impl super::entity::GameObjectDataTrait for TerrainPhysicsBodyData {
}

impl super::core::DataBusPeerTrait for TerrainPhysicsBodyData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for TerrainPhysicsBodyData {
}

impl super::core::DataContainerTrait for TerrainPhysicsBodyData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TERRAINPHYSICSBODYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainPhysicsBodyData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSBODYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainPhysicsBodyData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "UseSDFCollision",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TerrainPhysicsBodyData, use_s_d_f_collision),
            },
            FieldInfoData {
                name: "RigidBodyIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainPhysicsBodyData, rigid_body_index),
            },
            FieldInfoData {
                name: "MaxResolutionSizeUseForSplit",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainPhysicsBodyData, max_resolution_size_use_for_split),
            },
            FieldInfoData {
                name: "SDFThickness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainPhysicsBodyData, s_d_f_thickness),
            },
        ],
    }),
    array_type: Some(TERRAINPHYSICSBODYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TerrainPhysicsBodyData {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINPHYSICSBODYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAINPHYSICSBODYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainPhysicsBodyData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("TerrainPhysicsBodyData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct StaticWaterPhysicsBodyData {
    pub _glacier_base: WaterPhysicsBodyData,
}

pub trait StaticWaterPhysicsBodyDataTrait: WaterPhysicsBodyDataTrait {
}

impl StaticWaterPhysicsBodyDataTrait for StaticWaterPhysicsBodyData {
}

impl WaterPhysicsBodyDataTrait for StaticWaterPhysicsBodyData {
}

impl PhysicsBodyDataTrait for StaticWaterPhysicsBodyData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn rigid_body_type(&self) -> &RigidBodyType {
        self._glacier_base.rigid_body_type()
    }
    fn collision_layer(&self) -> &RigidBodyCollisionLayer {
        self._glacier_base.collision_layer()
    }
    fn motion_type(&self) -> &RigidBodyMotionType {
        self._glacier_base.motion_type()
    }
    fn quality_type(&self) -> &RigidBodyQualityType {
        self._glacier_base.quality_type()
    }
    fn asset(&self) -> &Option<Arc<Mutex<dyn super::core::AssetTrait>>> {
        self._glacier_base.asset()
    }
    fn transform_index(&self) -> &u8 {
        self._glacier_base.transform_index()
    }
    fn world_index(&self) -> &u8 {
        self._glacier_base.world_index()
    }
    fn collision_root_shape_index(&self) -> &u8 {
        self._glacier_base.collision_root_shape_index()
    }
    fn raycast_root_shape_index(&self) -> &u8 {
        self._glacier_base.raycast_root_shape_index()
    }
    fn add_to_spatial_query_manager(&self) -> &bool {
        self._glacier_base.add_to_spatial_query_manager()
    }
    fn physics_callback_handler(&self) -> &Option<Arc<Mutex<dyn PhysicsCallbackHandlerDataTrait>>> {
        self._glacier_base.physics_callback_handler()
    }
}

impl super::entity::EntityDataTrait for StaticWaterPhysicsBodyData {
}

impl super::entity::GameObjectDataTrait for StaticWaterPhysicsBodyData {
}

impl super::core::DataBusPeerTrait for StaticWaterPhysicsBodyData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for StaticWaterPhysicsBodyData {
}

impl super::core::DataContainerTrait for StaticWaterPhysicsBodyData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static STATICWATERPHYSICSBODYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticWaterPhysicsBodyData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WATERPHYSICSBODYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StaticWaterPhysicsBodyData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(STATICWATERPHYSICSBODYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StaticWaterPhysicsBodyData {
    fn type_info(&self) -> &'static TypeInfo {
        STATICWATERPHYSICSBODYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static STATICWATERPHYSICSBODYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticWaterPhysicsBodyData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("StaticWaterPhysicsBodyData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WaterPhysicsBodyData {
    pub _glacier_base: PhysicsBodyData,
}

pub trait WaterPhysicsBodyDataTrait: PhysicsBodyDataTrait {
}

impl WaterPhysicsBodyDataTrait for WaterPhysicsBodyData {
}

impl PhysicsBodyDataTrait for WaterPhysicsBodyData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn rigid_body_type(&self) -> &RigidBodyType {
        self._glacier_base.rigid_body_type()
    }
    fn collision_layer(&self) -> &RigidBodyCollisionLayer {
        self._glacier_base.collision_layer()
    }
    fn motion_type(&self) -> &RigidBodyMotionType {
        self._glacier_base.motion_type()
    }
    fn quality_type(&self) -> &RigidBodyQualityType {
        self._glacier_base.quality_type()
    }
    fn asset(&self) -> &Option<Arc<Mutex<dyn super::core::AssetTrait>>> {
        self._glacier_base.asset()
    }
    fn transform_index(&self) -> &u8 {
        self._glacier_base.transform_index()
    }
    fn world_index(&self) -> &u8 {
        self._glacier_base.world_index()
    }
    fn collision_root_shape_index(&self) -> &u8 {
        self._glacier_base.collision_root_shape_index()
    }
    fn raycast_root_shape_index(&self) -> &u8 {
        self._glacier_base.raycast_root_shape_index()
    }
    fn add_to_spatial_query_manager(&self) -> &bool {
        self._glacier_base.add_to_spatial_query_manager()
    }
    fn physics_callback_handler(&self) -> &Option<Arc<Mutex<dyn PhysicsCallbackHandlerDataTrait>>> {
        self._glacier_base.physics_callback_handler()
    }
}

impl super::entity::EntityDataTrait for WaterPhysicsBodyData {
}

impl super::entity::GameObjectDataTrait for WaterPhysicsBodyData {
}

impl super::core::DataBusPeerTrait for WaterPhysicsBodyData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for WaterPhysicsBodyData {
}

impl super::core::DataContainerTrait for WaterPhysicsBodyData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static WATERPHYSICSBODYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterPhysicsBodyData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSBODYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaterPhysicsBodyData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(WATERPHYSICSBODYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WaterPhysicsBodyData {
    fn type_info(&self) -> &'static TypeInfo {
        WATERPHYSICSBODYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WATERPHYSICSBODYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterPhysicsBodyData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("WaterPhysicsBodyData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct GroupRigidBodyData {
    pub _glacier_base: RigidBodyData,
    pub raycast_material_indices_lookups: Vec<MaterialIndicesLookup>,
}

pub trait GroupRigidBodyDataTrait: RigidBodyDataTrait {
    fn raycast_material_indices_lookups(&self) -> &Vec<MaterialIndicesLookup>;
}

impl GroupRigidBodyDataTrait for GroupRigidBodyData {
    fn raycast_material_indices_lookups(&self) -> &Vec<MaterialIndicesLookup> {
        &self.raycast_material_indices_lookups
    }
}

impl RigidBodyDataTrait for GroupRigidBodyData {
    fn mass(&self) -> &f32 {
        self._glacier_base.mass()
    }
    fn material(&self) -> &super::entity::MaterialDecl {
        self._glacier_base.material()
    }
    fn dynamic_friction(&self) -> &f32 {
        self._glacier_base.dynamic_friction()
    }
    fn static_friction(&self) -> &f32 {
        self._glacier_base.static_friction()
    }
    fn restitution(&self) -> &f32 {
        self._glacier_base.restitution()
    }
    fn compute_center_of_mass(&self) -> &bool {
        self._glacier_base.compute_center_of_mass()
    }
    fn center_of_mass(&self) -> &super::core::Vec3 {
        self._glacier_base.center_of_mass()
    }
    fn compute_inertia_tensor(&self) -> &bool {
        self._glacier_base.compute_inertia_tensor()
    }
    fn inertia_modifier(&self) -> &super::core::Vec3 {
        self._glacier_base.inertia_modifier()
    }
    fn angular_velocity_damping(&self) -> &f32 {
        self._glacier_base.angular_velocity_damping()
    }
    fn linear_velocity_damping(&self) -> &f32 {
        self._glacier_base.linear_velocity_damping()
    }
    fn system_group(&self) -> &u32 {
        self._glacier_base.system_group()
    }
    fn sub_system_id(&self) -> &u32 {
        self._glacier_base.sub_system_id()
    }
    fn sub_system_id_no_collide(&self) -> &u32 {
        self._glacier_base.sub_system_id_no_collide()
    }
    fn part_indices(&self) -> &Vec<u32> {
        self._glacier_base.part_indices()
    }
    fn float_physics(&self) -> &Option<Arc<Mutex<dyn FloatPhysicsDataTrait>>> {
        self._glacier_base.float_physics()
    }
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn is_root_controller(&self) -> &bool {
        self._glacier_base.is_root_controller()
    }
    fn part_materials(&self) -> &Vec<super::entity::MaterialDecl> {
        self._glacier_base.part_materials()
    }
    fn inverse_inertia_tensor(&self) -> &super::core::Vec3 {
        self._glacier_base.inverse_inertia_tensor()
    }
    fn principal_axes_of_inertia(&self) -> &super::core::Quat {
        self._glacier_base.principal_axes_of_inertia()
    }
}

impl PhysicsBodyDataTrait for GroupRigidBodyData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn rigid_body_type(&self) -> &RigidBodyType {
        self._glacier_base.rigid_body_type()
    }
    fn collision_layer(&self) -> &RigidBodyCollisionLayer {
        self._glacier_base.collision_layer()
    }
    fn motion_type(&self) -> &RigidBodyMotionType {
        self._glacier_base.motion_type()
    }
    fn quality_type(&self) -> &RigidBodyQualityType {
        self._glacier_base.quality_type()
    }
    fn asset(&self) -> &Option<Arc<Mutex<dyn super::core::AssetTrait>>> {
        self._glacier_base.asset()
    }
    fn transform_index(&self) -> &u8 {
        self._glacier_base.transform_index()
    }
    fn world_index(&self) -> &u8 {
        self._glacier_base.world_index()
    }
    fn collision_root_shape_index(&self) -> &u8 {
        self._glacier_base.collision_root_shape_index()
    }
    fn raycast_root_shape_index(&self) -> &u8 {
        self._glacier_base.raycast_root_shape_index()
    }
    fn add_to_spatial_query_manager(&self) -> &bool {
        self._glacier_base.add_to_spatial_query_manager()
    }
    fn physics_callback_handler(&self) -> &Option<Arc<Mutex<dyn PhysicsCallbackHandlerDataTrait>>> {
        self._glacier_base.physics_callback_handler()
    }
}

impl super::entity::EntityDataTrait for GroupRigidBodyData {
}

impl super::entity::GameObjectDataTrait for GroupRigidBodyData {
}

impl super::core::DataBusPeerTrait for GroupRigidBodyData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for GroupRigidBodyData {
}

impl super::core::DataContainerTrait for GroupRigidBodyData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static GROUPRIGIDBODYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroupRigidBodyData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RIGIDBODYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GroupRigidBodyData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "RaycastMaterialIndicesLookups",
                flags: MemberInfoFlags::new(144),
                field_type: "MaterialIndicesLookup-Array",
                rust_offset: offset_of!(GroupRigidBodyData, raycast_material_indices_lookups),
            },
        ],
    }),
    array_type: Some(GROUPRIGIDBODYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for GroupRigidBodyData {
    fn type_info(&self) -> &'static TypeInfo {
        GROUPRIGIDBODYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static GROUPRIGIDBODYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroupRigidBodyData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("GroupRigidBodyData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MaterialIndicesLookup {
    pub material_indices: Vec<u32>,
}

pub trait MaterialIndicesLookupTrait: TypeObject {
    fn material_indices(&self) -> &Vec<u32>;
}

impl MaterialIndicesLookupTrait for MaterialIndicesLookup {
    fn material_indices(&self) -> &Vec<u32> {
        &self.material_indices
    }
}

pub static MATERIALINDICESLOOKUP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialIndicesLookup",
    flags: MemberInfoFlags::new(73),
    module: "Physics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MaterialIndicesLookup as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MaterialIndices",
                flags: MemberInfoFlags::new(144),
                field_type: "Uint32-Array",
                rust_offset: offset_of!(MaterialIndicesLookup, material_indices),
            },
        ],
    }),
    array_type: Some(MATERIALINDICESLOOKUP_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MaterialIndicesLookup {
    fn type_info(&self) -> &'static TypeInfo {
        MATERIALINDICESLOOKUP_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MATERIALINDICESLOOKUP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialIndicesLookup-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("MaterialIndicesLookup"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RigidBodyData {
    pub _glacier_base: PhysicsBodyData,
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
    pub float_physics: Option<Arc<Mutex<dyn FloatPhysicsDataTrait>>>,
    pub transform: super::core::LinearTransform,
    pub is_root_controller: bool,
    pub part_materials: Vec<super::entity::MaterialDecl>,
    pub inverse_inertia_tensor: super::core::Vec3,
    pub principal_axes_of_inertia: super::core::Quat,
}

pub trait RigidBodyDataTrait: PhysicsBodyDataTrait {
    fn mass(&self) -> &f32;
    fn material(&self) -> &super::entity::MaterialDecl;
    fn dynamic_friction(&self) -> &f32;
    fn static_friction(&self) -> &f32;
    fn restitution(&self) -> &f32;
    fn compute_center_of_mass(&self) -> &bool;
    fn center_of_mass(&self) -> &super::core::Vec3;
    fn compute_inertia_tensor(&self) -> &bool;
    fn inertia_modifier(&self) -> &super::core::Vec3;
    fn angular_velocity_damping(&self) -> &f32;
    fn linear_velocity_damping(&self) -> &f32;
    fn system_group(&self) -> &u32;
    fn sub_system_id(&self) -> &u32;
    fn sub_system_id_no_collide(&self) -> &u32;
    fn part_indices(&self) -> &Vec<u32>;
    fn float_physics(&self) -> &Option<Arc<Mutex<dyn FloatPhysicsDataTrait>>>;
    fn transform(&self) -> &super::core::LinearTransform;
    fn is_root_controller(&self) -> &bool;
    fn part_materials(&self) -> &Vec<super::entity::MaterialDecl>;
    fn inverse_inertia_tensor(&self) -> &super::core::Vec3;
    fn principal_axes_of_inertia(&self) -> &super::core::Quat;
}

impl RigidBodyDataTrait for RigidBodyData {
    fn mass(&self) -> &f32 {
        &self.mass
    }
    fn material(&self) -> &super::entity::MaterialDecl {
        &self.material
    }
    fn dynamic_friction(&self) -> &f32 {
        &self.dynamic_friction
    }
    fn static_friction(&self) -> &f32 {
        &self.static_friction
    }
    fn restitution(&self) -> &f32 {
        &self.restitution
    }
    fn compute_center_of_mass(&self) -> &bool {
        &self.compute_center_of_mass
    }
    fn center_of_mass(&self) -> &super::core::Vec3 {
        &self.center_of_mass
    }
    fn compute_inertia_tensor(&self) -> &bool {
        &self.compute_inertia_tensor
    }
    fn inertia_modifier(&self) -> &super::core::Vec3 {
        &self.inertia_modifier
    }
    fn angular_velocity_damping(&self) -> &f32 {
        &self.angular_velocity_damping
    }
    fn linear_velocity_damping(&self) -> &f32 {
        &self.linear_velocity_damping
    }
    fn system_group(&self) -> &u32 {
        &self.system_group
    }
    fn sub_system_id(&self) -> &u32 {
        &self.sub_system_id
    }
    fn sub_system_id_no_collide(&self) -> &u32 {
        &self.sub_system_id_no_collide
    }
    fn part_indices(&self) -> &Vec<u32> {
        &self.part_indices
    }
    fn float_physics(&self) -> &Option<Arc<Mutex<dyn FloatPhysicsDataTrait>>> {
        &self.float_physics
    }
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
    fn is_root_controller(&self) -> &bool {
        &self.is_root_controller
    }
    fn part_materials(&self) -> &Vec<super::entity::MaterialDecl> {
        &self.part_materials
    }
    fn inverse_inertia_tensor(&self) -> &super::core::Vec3 {
        &self.inverse_inertia_tensor
    }
    fn principal_axes_of_inertia(&self) -> &super::core::Quat {
        &self.principal_axes_of_inertia
    }
}

impl PhysicsBodyDataTrait for RigidBodyData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn rigid_body_type(&self) -> &RigidBodyType {
        self._glacier_base.rigid_body_type()
    }
    fn collision_layer(&self) -> &RigidBodyCollisionLayer {
        self._glacier_base.collision_layer()
    }
    fn motion_type(&self) -> &RigidBodyMotionType {
        self._glacier_base.motion_type()
    }
    fn quality_type(&self) -> &RigidBodyQualityType {
        self._glacier_base.quality_type()
    }
    fn asset(&self) -> &Option<Arc<Mutex<dyn super::core::AssetTrait>>> {
        self._glacier_base.asset()
    }
    fn transform_index(&self) -> &u8 {
        self._glacier_base.transform_index()
    }
    fn world_index(&self) -> &u8 {
        self._glacier_base.world_index()
    }
    fn collision_root_shape_index(&self) -> &u8 {
        self._glacier_base.collision_root_shape_index()
    }
    fn raycast_root_shape_index(&self) -> &u8 {
        self._glacier_base.raycast_root_shape_index()
    }
    fn add_to_spatial_query_manager(&self) -> &bool {
        self._glacier_base.add_to_spatial_query_manager()
    }
    fn physics_callback_handler(&self) -> &Option<Arc<Mutex<dyn PhysicsCallbackHandlerDataTrait>>> {
        self._glacier_base.physics_callback_handler()
    }
}

impl super::entity::EntityDataTrait for RigidBodyData {
}

impl super::entity::GameObjectDataTrait for RigidBodyData {
}

impl super::core::DataBusPeerTrait for RigidBodyData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for RigidBodyData {
}

impl super::core::DataContainerTrait for RigidBodyData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static RIGIDBODYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RigidBodyData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSBODYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RigidBodyData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Mass",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RigidBodyData, mass),
            },
            FieldInfoData {
                name: "Material",
                flags: MemberInfoFlags::new(0),
                field_type: "MaterialDecl",
                rust_offset: offset_of!(RigidBodyData, material),
            },
            FieldInfoData {
                name: "DynamicFriction",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RigidBodyData, dynamic_friction),
            },
            FieldInfoData {
                name: "StaticFriction",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RigidBodyData, static_friction),
            },
            FieldInfoData {
                name: "Restitution",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RigidBodyData, restitution),
            },
            FieldInfoData {
                name: "ComputeCenterOfMass",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RigidBodyData, compute_center_of_mass),
            },
            FieldInfoData {
                name: "CenterOfMass",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(RigidBodyData, center_of_mass),
            },
            FieldInfoData {
                name: "ComputeInertiaTensor",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RigidBodyData, compute_inertia_tensor),
            },
            FieldInfoData {
                name: "InertiaModifier",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(RigidBodyData, inertia_modifier),
            },
            FieldInfoData {
                name: "AngularVelocityDamping",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RigidBodyData, angular_velocity_damping),
            },
            FieldInfoData {
                name: "LinearVelocityDamping",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RigidBodyData, linear_velocity_damping),
            },
            FieldInfoData {
                name: "SystemGroup",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(RigidBodyData, system_group),
            },
            FieldInfoData {
                name: "SubSystemId",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(RigidBodyData, sub_system_id),
            },
            FieldInfoData {
                name: "SubSystemIdNoCollide",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(RigidBodyData, sub_system_id_no_collide),
            },
            FieldInfoData {
                name: "PartIndices",
                flags: MemberInfoFlags::new(144),
                field_type: "Uint32-Array",
                rust_offset: offset_of!(RigidBodyData, part_indices),
            },
            FieldInfoData {
                name: "FloatPhysics",
                flags: MemberInfoFlags::new(0),
                field_type: "FloatPhysicsData",
                rust_offset: offset_of!(RigidBodyData, float_physics),
            },
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(RigidBodyData, transform),
            },
            FieldInfoData {
                name: "IsRootController",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RigidBodyData, is_root_controller),
            },
            FieldInfoData {
                name: "PartMaterials",
                flags: MemberInfoFlags::new(144),
                field_type: "MaterialDecl-Array",
                rust_offset: offset_of!(RigidBodyData, part_materials),
            },
            FieldInfoData {
                name: "InverseInertiaTensor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(RigidBodyData, inverse_inertia_tensor),
            },
            FieldInfoData {
                name: "PrincipalAxesOfInertia",
                flags: MemberInfoFlags::new(0),
                field_type: "Quat",
                rust_offset: offset_of!(RigidBodyData, principal_axes_of_inertia),
            },
        ],
    }),
    array_type: Some(RIGIDBODYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RigidBodyData {
    fn type_info(&self) -> &'static TypeInfo {
        RIGIDBODYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RIGIDBODYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RigidBodyData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("RigidBodyData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsBodyData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub rigid_body_type: RigidBodyType,
    pub collision_layer: RigidBodyCollisionLayer,
    pub motion_type: RigidBodyMotionType,
    pub quality_type: RigidBodyQualityType,
    pub asset: Option<Arc<Mutex<dyn super::core::AssetTrait>>>,
    pub transform_index: u8,
    pub world_index: u8,
    pub collision_root_shape_index: u8,
    pub raycast_root_shape_index: u8,
    pub add_to_spatial_query_manager: bool,
    pub physics_callback_handler: Option<Arc<Mutex<dyn PhysicsCallbackHandlerDataTrait>>>,
}

pub trait PhysicsBodyDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn rigid_body_type(&self) -> &RigidBodyType;
    fn collision_layer(&self) -> &RigidBodyCollisionLayer;
    fn motion_type(&self) -> &RigidBodyMotionType;
    fn quality_type(&self) -> &RigidBodyQualityType;
    fn asset(&self) -> &Option<Arc<Mutex<dyn super::core::AssetTrait>>>;
    fn transform_index(&self) -> &u8;
    fn world_index(&self) -> &u8;
    fn collision_root_shape_index(&self) -> &u8;
    fn raycast_root_shape_index(&self) -> &u8;
    fn add_to_spatial_query_manager(&self) -> &bool;
    fn physics_callback_handler(&self) -> &Option<Arc<Mutex<dyn PhysicsCallbackHandlerDataTrait>>>;
}

impl PhysicsBodyDataTrait for PhysicsBodyData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn rigid_body_type(&self) -> &RigidBodyType {
        &self.rigid_body_type
    }
    fn collision_layer(&self) -> &RigidBodyCollisionLayer {
        &self.collision_layer
    }
    fn motion_type(&self) -> &RigidBodyMotionType {
        &self.motion_type
    }
    fn quality_type(&self) -> &RigidBodyQualityType {
        &self.quality_type
    }
    fn asset(&self) -> &Option<Arc<Mutex<dyn super::core::AssetTrait>>> {
        &self.asset
    }
    fn transform_index(&self) -> &u8 {
        &self.transform_index
    }
    fn world_index(&self) -> &u8 {
        &self.world_index
    }
    fn collision_root_shape_index(&self) -> &u8 {
        &self.collision_root_shape_index
    }
    fn raycast_root_shape_index(&self) -> &u8 {
        &self.raycast_root_shape_index
    }
    fn add_to_spatial_query_manager(&self) -> &bool {
        &self.add_to_spatial_query_manager
    }
    fn physics_callback_handler(&self) -> &Option<Arc<Mutex<dyn PhysicsCallbackHandlerDataTrait>>> {
        &self.physics_callback_handler
    }
}

impl super::entity::EntityDataTrait for PhysicsBodyData {
}

impl super::entity::GameObjectDataTrait for PhysicsBodyData {
}

impl super::core::DataBusPeerTrait for PhysicsBodyData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for PhysicsBodyData {
}

impl super::core::DataContainerTrait for PhysicsBodyData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PHYSICSBODYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsBodyData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsBodyData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(PhysicsBodyData, realm),
            },
            FieldInfoData {
                name: "RigidBodyType",
                flags: MemberInfoFlags::new(0),
                field_type: "RigidBodyType",
                rust_offset: offset_of!(PhysicsBodyData, rigid_body_type),
            },
            FieldInfoData {
                name: "CollisionLayer",
                flags: MemberInfoFlags::new(0),
                field_type: "RigidBodyCollisionLayer",
                rust_offset: offset_of!(PhysicsBodyData, collision_layer),
            },
            FieldInfoData {
                name: "MotionType",
                flags: MemberInfoFlags::new(0),
                field_type: "RigidBodyMotionType",
                rust_offset: offset_of!(PhysicsBodyData, motion_type),
            },
            FieldInfoData {
                name: "QualityType",
                flags: MemberInfoFlags::new(0),
                field_type: "RigidBodyQualityType",
                rust_offset: offset_of!(PhysicsBodyData, quality_type),
            },
            FieldInfoData {
                name: "Asset",
                flags: MemberInfoFlags::new(0),
                field_type: "Asset",
                rust_offset: offset_of!(PhysicsBodyData, asset),
            },
            FieldInfoData {
                name: "TransformIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(PhysicsBodyData, transform_index),
            },
            FieldInfoData {
                name: "WorldIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(PhysicsBodyData, world_index),
            },
            FieldInfoData {
                name: "CollisionRootShapeIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(PhysicsBodyData, collision_root_shape_index),
            },
            FieldInfoData {
                name: "RaycastRootShapeIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(PhysicsBodyData, raycast_root_shape_index),
            },
            FieldInfoData {
                name: "AddToSpatialQueryManager",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsBodyData, add_to_spatial_query_manager),
            },
            FieldInfoData {
                name: "PhysicsCallbackHandler",
                flags: MemberInfoFlags::new(0),
                field_type: "PhysicsCallbackHandlerData",
                rust_offset: offset_of!(PhysicsBodyData, physics_callback_handler),
            },
        ],
    }),
    array_type: Some(PHYSICSBODYDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PhysicsBodyData {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSBODYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSBODYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsBodyData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsBodyData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BoxFloatPhysicsData {
    pub _glacier_base: FloatPhysicsData,
    pub box_action_data: Option<Arc<Mutex<dyn FloatPhysicsActionDataTrait>>>,
}

pub trait BoxFloatPhysicsDataTrait: FloatPhysicsDataTrait {
    fn box_action_data(&self) -> &Option<Arc<Mutex<dyn FloatPhysicsActionDataTrait>>>;
}

impl BoxFloatPhysicsDataTrait for BoxFloatPhysicsData {
    fn box_action_data(&self) -> &Option<Arc<Mutex<dyn FloatPhysicsActionDataTrait>>> {
        &self.box_action_data
    }
}

impl FloatPhysicsDataTrait for BoxFloatPhysicsData {
    fn density(&self) -> &f32 {
        self._glacier_base.density()
    }
    fn filled_density(&self) -> &f32 {
        self._glacier_base.filled_density()
    }
}

impl super::core::DataContainerTrait for BoxFloatPhysicsData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static BOXFLOATPHYSICSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoxFloatPhysicsData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(FLOATPHYSICSDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BoxFloatPhysicsData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "BoxActionData",
                flags: MemberInfoFlags::new(0),
                field_type: "FloatPhysicsActionData",
                rust_offset: offset_of!(BoxFloatPhysicsData, box_action_data),
            },
        ],
    }),
    array_type: Some(BOXFLOATPHYSICSDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BoxFloatPhysicsData {
    fn type_info(&self) -> &'static TypeInfo {
        BOXFLOATPHYSICSDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static BOXFLOATPHYSICSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoxFloatPhysicsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("BoxFloatPhysicsData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FloatPhysicsActionData {
    pub _glacier_base: PhysicsActionData,
}

pub trait FloatPhysicsActionDataTrait: PhysicsActionDataTrait {
}

impl FloatPhysicsActionDataTrait for FloatPhysicsActionData {
}

impl PhysicsActionDataTrait for FloatPhysicsActionData {
}

impl super::entity::EntityDataTrait for FloatPhysicsActionData {
}

impl super::entity::GameObjectDataTrait for FloatPhysicsActionData {
}

impl super::core::DataBusPeerTrait for FloatPhysicsActionData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for FloatPhysicsActionData {
}

impl super::core::DataContainerTrait for FloatPhysicsActionData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static FLOATPHYSICSACTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatPhysicsActionData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSACTIONDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FloatPhysicsActionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(FLOATPHYSICSACTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FloatPhysicsActionData {
    fn type_info(&self) -> &'static TypeInfo {
        FLOATPHYSICSACTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FLOATPHYSICSACTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatPhysicsActionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("FloatPhysicsActionData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WindPhysicsActionData {
    pub _glacier_base: PhysicsActionData,
    pub resistance_factor: f32,
}

pub trait WindPhysicsActionDataTrait: PhysicsActionDataTrait {
    fn resistance_factor(&self) -> &f32;
}

impl WindPhysicsActionDataTrait for WindPhysicsActionData {
    fn resistance_factor(&self) -> &f32 {
        &self.resistance_factor
    }
}

impl PhysicsActionDataTrait for WindPhysicsActionData {
}

impl super::entity::EntityDataTrait for WindPhysicsActionData {
}

impl super::entity::GameObjectDataTrait for WindPhysicsActionData {
}

impl super::core::DataBusPeerTrait for WindPhysicsActionData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for WindPhysicsActionData {
}

impl super::core::DataContainerTrait for WindPhysicsActionData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static WINDPHYSICSACTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindPhysicsActionData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSACTIONDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WindPhysicsActionData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ResistanceFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WindPhysicsActionData, resistance_factor),
            },
        ],
    }),
    array_type: Some(WINDPHYSICSACTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WindPhysicsActionData {
    fn type_info(&self) -> &'static TypeInfo {
        WINDPHYSICSACTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WINDPHYSICSACTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindPhysicsActionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("WindPhysicsActionData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsActionData {
    pub _glacier_base: super::entity::EntityData,
}

pub trait PhysicsActionDataTrait: super::entity::EntityDataTrait {
}

impl PhysicsActionDataTrait for PhysicsActionData {
}

impl super::entity::EntityDataTrait for PhysicsActionData {
}

impl super::entity::GameObjectDataTrait for PhysicsActionData {
}

impl super::core::DataBusPeerTrait for PhysicsActionData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for PhysicsActionData {
}

impl super::core::DataContainerTrait for PhysicsActionData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PHYSICSACTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsActionData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsActionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSACTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PhysicsActionData {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSACTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSACTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsActionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsActionData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsProxyEntityData {
    pub _glacier_base: super::entity::ComponentEntityData,
    pub realm: super::core::Realm,
}

pub trait PhysicsProxyEntityDataTrait: super::entity::ComponentEntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
}

impl PhysicsProxyEntityDataTrait for PhysicsProxyEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
}

impl super::entity::ComponentEntityDataTrait for PhysicsProxyEntityData {
    fn components(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components()
    }
    fn part_bounding_boxes(&self) -> &Vec<super::core::AxisAlignedBox> {
        self._glacier_base.part_bounding_boxes()
    }
    fn client_runtime_component_count(&self) -> &u8 {
        self._glacier_base.client_runtime_component_count()
    }
    fn server_runtime_component_count(&self) -> &u8 {
        self._glacier_base.server_runtime_component_count()
    }
    fn client_runtime_transformation_count(&self) -> &u8 {
        self._glacier_base.client_runtime_transformation_count()
    }
    fn server_runtime_transformation_count(&self) -> &u8 {
        self._glacier_base.server_runtime_transformation_count()
    }
}

impl super::entity::SpatialEntityDataTrait for PhysicsProxyEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
}

impl super::entity::EntityDataTrait for PhysicsProxyEntityData {
}

impl super::entity::GameObjectDataTrait for PhysicsProxyEntityData {
}

impl super::core::DataBusPeerTrait for PhysicsProxyEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for PhysicsProxyEntityData {
}

impl super::core::DataContainerTrait for PhysicsProxyEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PHYSICSPROXYENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsProxyEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::COMPONENTENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsProxyEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(PhysicsProxyEntityData, realm),
            },
        ],
    }),
    array_type: Some(PHYSICSPROXYENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsProxyEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSPROXYENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSPROXYENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsProxyEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsProxyEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct GamePhysicsEntityData {
    pub _glacier_base: super::entity::GameComponentEntityData,
}

pub trait GamePhysicsEntityDataTrait: super::entity::GameComponentEntityDataTrait {
}

impl GamePhysicsEntityDataTrait for GamePhysicsEntityData {
}

impl super::entity::GameComponentEntityDataTrait for GamePhysicsEntityData {
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
}

impl super::entity::ComponentEntityDataTrait for GamePhysicsEntityData {
    fn components(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components()
    }
    fn part_bounding_boxes(&self) -> &Vec<super::core::AxisAlignedBox> {
        self._glacier_base.part_bounding_boxes()
    }
    fn client_runtime_component_count(&self) -> &u8 {
        self._glacier_base.client_runtime_component_count()
    }
    fn server_runtime_component_count(&self) -> &u8 {
        self._glacier_base.server_runtime_component_count()
    }
    fn client_runtime_transformation_count(&self) -> &u8 {
        self._glacier_base.client_runtime_transformation_count()
    }
    fn server_runtime_transformation_count(&self) -> &u8 {
        self._glacier_base.server_runtime_transformation_count()
    }
}

impl super::entity::SpatialEntityDataTrait for GamePhysicsEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
}

impl super::entity::EntityDataTrait for GamePhysicsEntityData {
}

impl super::entity::GameObjectDataTrait for GamePhysicsEntityData {
}

impl super::core::DataBusPeerTrait for GamePhysicsEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for GamePhysicsEntityData {
}

impl super::core::DataContainerTrait for GamePhysicsEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static GAMEPHYSICSENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GamePhysicsEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::GAMECOMPONENTENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GamePhysicsEntityData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(GAMEPHYSICSENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for GamePhysicsEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        GAMEPHYSICSENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static GAMEPHYSICSENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GamePhysicsEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("GamePhysicsEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DefaultPartPhysicsComponentData {
    pub _glacier_base: PartPhysicsComponentData,
}

pub trait DefaultPartPhysicsComponentDataTrait: PartPhysicsComponentDataTrait {
}

impl DefaultPartPhysicsComponentDataTrait for DefaultPartPhysicsComponentData {
}

impl PartPhysicsComponentDataTrait for DefaultPartPhysicsComponentData {
}

impl PhysicsComponentDataTrait for DefaultPartPhysicsComponentData {
    fn physics_bodies(&self) -> &Vec<Option<Arc<Mutex<dyn PhysicsBodyDataTrait>>>> {
        self._glacier_base.physics_bodies()
    }
    fn physics_constraints(&self) -> &Vec<Option<Arc<Mutex<dyn PhysicsConstraintDataTrait>>>> {
        self._glacier_base.physics_constraints()
    }
    fn parts(&self) -> &Vec<PhysicsPartData> {
        self._glacier_base.parts()
    }
    fn movable_parts(&self) -> &bool {
        self._glacier_base.movable_parts()
    }
    fn internal_collision_disabling(&self) -> &InternalCollisionDisablingBehavior {
        self._glacier_base.internal_collision_disabling()
    }
    fn enable_collision_events(&self) -> &bool {
        self._glacier_base.enable_collision_events()
    }
}

impl super::entity::ComponentDataTrait for DefaultPartPhysicsComponentData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn components(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components()
    }
    fn client_index(&self) -> &u8 {
        self._glacier_base.client_index()
    }
    fn server_index(&self) -> &u8 {
        self._glacier_base.server_index()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
}

impl super::entity::GameObjectDataTrait for DefaultPartPhysicsComponentData {
}

impl super::core::DataBusPeerTrait for DefaultPartPhysicsComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for DefaultPartPhysicsComponentData {
}

impl super::core::DataContainerTrait for DefaultPartPhysicsComponentData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static DEFAULTPARTPHYSICSCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DefaultPartPhysicsComponentData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PARTPHYSICSCOMPONENTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DefaultPartPhysicsComponentData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DEFAULTPARTPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DefaultPartPhysicsComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        DEFAULTPARTPHYSICSCOMPONENTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DEFAULTPARTPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DefaultPartPhysicsComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("DefaultPartPhysicsComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PartPhysicsComponentData {
    pub _glacier_base: PhysicsComponentData,
}

pub trait PartPhysicsComponentDataTrait: PhysicsComponentDataTrait {
}

impl PartPhysicsComponentDataTrait for PartPhysicsComponentData {
}

impl PhysicsComponentDataTrait for PartPhysicsComponentData {
    fn physics_bodies(&self) -> &Vec<Option<Arc<Mutex<dyn PhysicsBodyDataTrait>>>> {
        self._glacier_base.physics_bodies()
    }
    fn physics_constraints(&self) -> &Vec<Option<Arc<Mutex<dyn PhysicsConstraintDataTrait>>>> {
        self._glacier_base.physics_constraints()
    }
    fn parts(&self) -> &Vec<PhysicsPartData> {
        self._glacier_base.parts()
    }
    fn movable_parts(&self) -> &bool {
        self._glacier_base.movable_parts()
    }
    fn internal_collision_disabling(&self) -> &InternalCollisionDisablingBehavior {
        self._glacier_base.internal_collision_disabling()
    }
    fn enable_collision_events(&self) -> &bool {
        self._glacier_base.enable_collision_events()
    }
}

impl super::entity::ComponentDataTrait for PartPhysicsComponentData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn components(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components()
    }
    fn client_index(&self) -> &u8 {
        self._glacier_base.client_index()
    }
    fn server_index(&self) -> &u8 {
        self._glacier_base.server_index()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
}

impl super::entity::GameObjectDataTrait for PartPhysicsComponentData {
}

impl super::core::DataBusPeerTrait for PartPhysicsComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for PartPhysicsComponentData {
}

impl super::core::DataContainerTrait for PartPhysicsComponentData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PARTPHYSICSCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartPhysicsComponentData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCOMPONENTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PartPhysicsComponentData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PARTPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PartPhysicsComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        PARTPHYSICSCOMPONENTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PARTPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartPhysicsComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PartPhysicsComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RagdollPhysicsComponentData {
    pub _glacier_base: PhysicsComponentData,
    pub bodies_names_hashes: Vec<u32>,
}

pub trait RagdollPhysicsComponentDataTrait: PhysicsComponentDataTrait {
    fn bodies_names_hashes(&self) -> &Vec<u32>;
}

impl RagdollPhysicsComponentDataTrait for RagdollPhysicsComponentData {
    fn bodies_names_hashes(&self) -> &Vec<u32> {
        &self.bodies_names_hashes
    }
}

impl PhysicsComponentDataTrait for RagdollPhysicsComponentData {
    fn physics_bodies(&self) -> &Vec<Option<Arc<Mutex<dyn PhysicsBodyDataTrait>>>> {
        self._glacier_base.physics_bodies()
    }
    fn physics_constraints(&self) -> &Vec<Option<Arc<Mutex<dyn PhysicsConstraintDataTrait>>>> {
        self._glacier_base.physics_constraints()
    }
    fn parts(&self) -> &Vec<PhysicsPartData> {
        self._glacier_base.parts()
    }
    fn movable_parts(&self) -> &bool {
        self._glacier_base.movable_parts()
    }
    fn internal_collision_disabling(&self) -> &InternalCollisionDisablingBehavior {
        self._glacier_base.internal_collision_disabling()
    }
    fn enable_collision_events(&self) -> &bool {
        self._glacier_base.enable_collision_events()
    }
}

impl super::entity::ComponentDataTrait for RagdollPhysicsComponentData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn components(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components()
    }
    fn client_index(&self) -> &u8 {
        self._glacier_base.client_index()
    }
    fn server_index(&self) -> &u8 {
        self._glacier_base.server_index()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
}

impl super::entity::GameObjectDataTrait for RagdollPhysicsComponentData {
}

impl super::core::DataBusPeerTrait for RagdollPhysicsComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for RagdollPhysicsComponentData {
}

impl super::core::DataContainerTrait for RagdollPhysicsComponentData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static RAGDOLLPHYSICSCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RagdollPhysicsComponentData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCOMPONENTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RagdollPhysicsComponentData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "BodiesNamesHashes",
                flags: MemberInfoFlags::new(144),
                field_type: "Uint32-Array",
                rust_offset: offset_of!(RagdollPhysicsComponentData, bodies_names_hashes),
            },
        ],
    }),
    array_type: Some(RAGDOLLPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RagdollPhysicsComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        RAGDOLLPHYSICSCOMPONENTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RAGDOLLPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RagdollPhysicsComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("RagdollPhysicsComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ForceComponentData {
    pub _glacier_base: super::entity::ComponentData,
    pub wind_action_data: Option<Arc<Mutex<dyn WindPhysicsActionDataTrait>>>,
}

pub trait ForceComponentDataTrait: super::entity::ComponentDataTrait {
    fn wind_action_data(&self) -> &Option<Arc<Mutex<dyn WindPhysicsActionDataTrait>>>;
}

impl ForceComponentDataTrait for ForceComponentData {
    fn wind_action_data(&self) -> &Option<Arc<Mutex<dyn WindPhysicsActionDataTrait>>> {
        &self.wind_action_data
    }
}

impl super::entity::ComponentDataTrait for ForceComponentData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn components(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components()
    }
    fn client_index(&self) -> &u8 {
        self._glacier_base.client_index()
    }
    fn server_index(&self) -> &u8 {
        self._glacier_base.server_index()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
}

impl super::entity::GameObjectDataTrait for ForceComponentData {
}

impl super::core::DataBusPeerTrait for ForceComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for ForceComponentData {
}

impl super::core::DataContainerTrait for ForceComponentData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static FORCECOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForceComponentData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::COMPONENTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ForceComponentData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "WindActionData",
                flags: MemberInfoFlags::new(0),
                field_type: "WindPhysicsActionData",
                rust_offset: offset_of!(ForceComponentData, wind_action_data),
            },
        ],
    }),
    array_type: Some(FORCECOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ForceComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        FORCECOMPONENTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FORCECOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForceComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ForceComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsComponentData {
    pub _glacier_base: super::entity::ComponentData,
    pub physics_bodies: Vec<Option<Arc<Mutex<dyn PhysicsBodyDataTrait>>>>,
    pub physics_constraints: Vec<Option<Arc<Mutex<dyn PhysicsConstraintDataTrait>>>>,
    pub parts: Vec<PhysicsPartData>,
    pub movable_parts: bool,
    pub internal_collision_disabling: InternalCollisionDisablingBehavior,
    pub enable_collision_events: bool,
}

pub trait PhysicsComponentDataTrait: super::entity::ComponentDataTrait {
    fn physics_bodies(&self) -> &Vec<Option<Arc<Mutex<dyn PhysicsBodyDataTrait>>>>;
    fn physics_constraints(&self) -> &Vec<Option<Arc<Mutex<dyn PhysicsConstraintDataTrait>>>>;
    fn parts(&self) -> &Vec<PhysicsPartData>;
    fn movable_parts(&self) -> &bool;
    fn internal_collision_disabling(&self) -> &InternalCollisionDisablingBehavior;
    fn enable_collision_events(&self) -> &bool;
}

impl PhysicsComponentDataTrait for PhysicsComponentData {
    fn physics_bodies(&self) -> &Vec<Option<Arc<Mutex<dyn PhysicsBodyDataTrait>>>> {
        &self.physics_bodies
    }
    fn physics_constraints(&self) -> &Vec<Option<Arc<Mutex<dyn PhysicsConstraintDataTrait>>>> {
        &self.physics_constraints
    }
    fn parts(&self) -> &Vec<PhysicsPartData> {
        &self.parts
    }
    fn movable_parts(&self) -> &bool {
        &self.movable_parts
    }
    fn internal_collision_disabling(&self) -> &InternalCollisionDisablingBehavior {
        &self.internal_collision_disabling
    }
    fn enable_collision_events(&self) -> &bool {
        &self.enable_collision_events
    }
}

impl super::entity::ComponentDataTrait for PhysicsComponentData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn components(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components()
    }
    fn client_index(&self) -> &u8 {
        self._glacier_base.client_index()
    }
    fn server_index(&self) -> &u8 {
        self._glacier_base.server_index()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
}

impl super::entity::GameObjectDataTrait for PhysicsComponentData {
}

impl super::core::DataBusPeerTrait for PhysicsComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for PhysicsComponentData {
}

impl super::core::DataContainerTrait for PhysicsComponentData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PHYSICSCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsComponentData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::COMPONENTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsComponentData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "PhysicsBodies",
                flags: MemberInfoFlags::new(144),
                field_type: "PhysicsBodyData-Array",
                rust_offset: offset_of!(PhysicsComponentData, physics_bodies),
            },
            FieldInfoData {
                name: "PhysicsConstraints",
                flags: MemberInfoFlags::new(144),
                field_type: "PhysicsConstraintData-Array",
                rust_offset: offset_of!(PhysicsComponentData, physics_constraints),
            },
            FieldInfoData {
                name: "Parts",
                flags: MemberInfoFlags::new(144),
                field_type: "PhysicsPartData-Array",
                rust_offset: offset_of!(PhysicsComponentData, parts),
            },
            FieldInfoData {
                name: "MovableParts",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsComponentData, movable_parts),
            },
            FieldInfoData {
                name: "InternalCollisionDisabling",
                flags: MemberInfoFlags::new(0),
                field_type: "InternalCollisionDisablingBehavior",
                rust_offset: offset_of!(PhysicsComponentData, internal_collision_disabling),
            },
            FieldInfoData {
                name: "EnableCollisionEvents",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsComponentData, enable_collision_events),
            },
        ],
    }),
    array_type: Some(PHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PhysicsComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSCOMPONENTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum InternalCollisionDisablingBehavior {
    #[default]
    InternalCollisionDisablingBehavior_Auto = 0,
    InternalCollisionDisablingBehavior_DisableNone = 1,
    InternalCollisionDisablingBehavior_DisableConstrained = 2,
}

pub static INTERNALCOLLISIONDISABLINGBEHAVIOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InternalCollisionDisablingBehavior",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(INTERNALCOLLISIONDISABLINGBEHAVIOR_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for InternalCollisionDisablingBehavior {
    fn type_info(&self) -> &'static TypeInfo {
        INTERNALCOLLISIONDISABLINGBEHAVIOR_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static INTERNALCOLLISIONDISABLINGBEHAVIOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InternalCollisionDisablingBehavior-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("InternalCollisionDisablingBehavior"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsPartData {
    pub aabb: super::core::AxisAlignedBox,
    pub transform_node: Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>,
}

pub trait PhysicsPartDataTrait: TypeObject {
    fn aabb(&self) -> &super::core::AxisAlignedBox;
    fn transform_node(&self) -> &Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>;
}

impl PhysicsPartDataTrait for PhysicsPartData {
    fn aabb(&self) -> &super::core::AxisAlignedBox {
        &self.aabb
    }
    fn transform_node(&self) -> &Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>> {
        &self.transform_node
    }
}

pub static PHYSICSPARTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsPartData",
    flags: MemberInfoFlags::new(73),
    module: "Physics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsPartData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Aabb",
                flags: MemberInfoFlags::new(0),
                field_type: "AxisAlignedBox",
                rust_offset: offset_of!(PhysicsPartData, aabb),
            },
            FieldInfoData {
                name: "TransformNode",
                flags: MemberInfoFlags::new(0),
                field_type: "GameObjectData",
                rust_offset: offset_of!(PhysicsPartData, transform_node),
            },
        ],
    }),
    array_type: Some(PHYSICSPARTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsPartData {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSPARTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSPARTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsPartData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsPartData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MaterialPropertyPhysicsDestructionData {
    pub _glacier_base: super::entity::PhysicsMaterialRelationPropertyData,
    pub density: f32,
    pub destruction_radius: f32,
    pub elasticity: f32,
    pub yield_strength: f32,
    pub breaking_strength: f32,
}

pub trait MaterialPropertyPhysicsDestructionDataTrait: super::entity::PhysicsMaterialRelationPropertyDataTrait {
    fn density(&self) -> &f32;
    fn destruction_radius(&self) -> &f32;
    fn elasticity(&self) -> &f32;
    fn yield_strength(&self) -> &f32;
    fn breaking_strength(&self) -> &f32;
}

impl MaterialPropertyPhysicsDestructionDataTrait for MaterialPropertyPhysicsDestructionData {
    fn density(&self) -> &f32 {
        &self.density
    }
    fn destruction_radius(&self) -> &f32 {
        &self.destruction_radius
    }
    fn elasticity(&self) -> &f32 {
        &self.elasticity
    }
    fn yield_strength(&self) -> &f32 {
        &self.yield_strength
    }
    fn breaking_strength(&self) -> &f32 {
        &self.breaking_strength
    }
}

impl super::entity::PhysicsMaterialRelationPropertyDataTrait for MaterialPropertyPhysicsDestructionData {
}

impl super::entity::MaterialRelationPropertyDataTrait for MaterialPropertyPhysicsDestructionData {
}

impl super::core::DataContainerTrait for MaterialPropertyPhysicsDestructionData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static MATERIALPROPERTYPHYSICSDESTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialPropertyPhysicsDestructionData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::PHYSICSMATERIALRELATIONPROPERTYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MaterialPropertyPhysicsDestructionData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Density",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MaterialPropertyPhysicsDestructionData, density),
            },
            FieldInfoData {
                name: "DestructionRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MaterialPropertyPhysicsDestructionData, destruction_radius),
            },
            FieldInfoData {
                name: "Elasticity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MaterialPropertyPhysicsDestructionData, elasticity),
            },
            FieldInfoData {
                name: "YieldStrength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MaterialPropertyPhysicsDestructionData, yield_strength),
            },
            FieldInfoData {
                name: "BreakingStrength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MaterialPropertyPhysicsDestructionData, breaking_strength),
            },
        ],
    }),
    array_type: Some(MATERIALPROPERTYPHYSICSDESTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MaterialPropertyPhysicsDestructionData {
    fn type_info(&self) -> &'static TypeInfo {
        MATERIALPROPERTYPHYSICSDESTRUCTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MATERIALPROPERTYPHYSICSDESTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialPropertyPhysicsDestructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("MaterialPropertyPhysicsDestructionData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MaterialPropertyPhysicsData {
    pub _glacier_base: super::entity::PhysicsMaterialRelationPropertyData,
    pub dynamic_friction: f32,
    pub static_friction: f32,
    pub restitution: f32,
    pub dynamic_friction_modifier: f32,
    pub static_friction_modifier: f32,
    pub resistance: f32,
}

pub trait MaterialPropertyPhysicsDataTrait: super::entity::PhysicsMaterialRelationPropertyDataTrait {
    fn dynamic_friction(&self) -> &f32;
    fn static_friction(&self) -> &f32;
    fn restitution(&self) -> &f32;
    fn dynamic_friction_modifier(&self) -> &f32;
    fn static_friction_modifier(&self) -> &f32;
    fn resistance(&self) -> &f32;
}

impl MaterialPropertyPhysicsDataTrait for MaterialPropertyPhysicsData {
    fn dynamic_friction(&self) -> &f32 {
        &self.dynamic_friction
    }
    fn static_friction(&self) -> &f32 {
        &self.static_friction
    }
    fn restitution(&self) -> &f32 {
        &self.restitution
    }
    fn dynamic_friction_modifier(&self) -> &f32 {
        &self.dynamic_friction_modifier
    }
    fn static_friction_modifier(&self) -> &f32 {
        &self.static_friction_modifier
    }
    fn resistance(&self) -> &f32 {
        &self.resistance
    }
}

impl super::entity::PhysicsMaterialRelationPropertyDataTrait for MaterialPropertyPhysicsData {
}

impl super::entity::MaterialRelationPropertyDataTrait for MaterialPropertyPhysicsData {
}

impl super::core::DataContainerTrait for MaterialPropertyPhysicsData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static MATERIALPROPERTYPHYSICSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialPropertyPhysicsData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::PHYSICSMATERIALRELATIONPROPERTYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MaterialPropertyPhysicsData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "DynamicFriction",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MaterialPropertyPhysicsData, dynamic_friction),
            },
            FieldInfoData {
                name: "StaticFriction",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MaterialPropertyPhysicsData, static_friction),
            },
            FieldInfoData {
                name: "Restitution",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MaterialPropertyPhysicsData, restitution),
            },
            FieldInfoData {
                name: "DynamicFrictionModifier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MaterialPropertyPhysicsData, dynamic_friction_modifier),
            },
            FieldInfoData {
                name: "StaticFrictionModifier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MaterialPropertyPhysicsData, static_friction_modifier),
            },
            FieldInfoData {
                name: "Resistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MaterialPropertyPhysicsData, resistance),
            },
        ],
    }),
    array_type: Some(MATERIALPROPERTYPHYSICSDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MaterialPropertyPhysicsData {
    fn type_info(&self) -> &'static TypeInfo {
        MATERIALPROPERTYPHYSICSDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MATERIALPROPERTYPHYSICSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialPropertyPhysicsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("MaterialPropertyPhysicsData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ProximityData {
    pub _glacier_base: super::core::DataContainer,
    pub proximity_type: ProximityObjectType,
}

pub trait ProximityDataTrait: super::core::DataContainerTrait {
    fn proximity_type(&self) -> &ProximityObjectType;
}

impl ProximityDataTrait for ProximityData {
    fn proximity_type(&self) -> &ProximityObjectType {
        &self.proximity_type
    }
}

impl super::core::DataContainerTrait for ProximityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PROXIMITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProximityData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ProximityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ProximityType",
                flags: MemberInfoFlags::new(0),
                field_type: "ProximityObjectType",
                rust_offset: offset_of!(ProximityData, proximity_type),
            },
        ],
    }),
    array_type: Some(PROXIMITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProximityData {
    fn type_info(&self) -> &'static TypeInfo {
        PROXIMITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PROXIMITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProximityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ProximityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ProximityObjectType {
    #[default]
    PotProximityDisabled = 0,
    PotVaultableLow = 1,
    PotVaultableHigh = 2,
    PotSupportedShooting = 3,
    PotInteractWith = 4,
    PotBashable = 5,
}

pub static PROXIMITYOBJECTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProximityObjectType",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(PROXIMITYOBJECTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ProximityObjectType {
    fn type_info(&self) -> &'static TypeInfo {
        PROXIMITYOBJECTTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PROXIMITYOBJECTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProximityObjectType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ProximityObjectType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsResourceContainerAsset {
    pub _glacier_base: super::core::Asset,
    pub physics_resource: glacier_reflect::builtin::ResourceRef,
}

pub trait PhysicsResourceContainerAssetTrait: super::core::AssetTrait {
    fn physics_resource(&self) -> &glacier_reflect::builtin::ResourceRef;
}

impl PhysicsResourceContainerAssetTrait for PhysicsResourceContainerAsset {
    fn physics_resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.physics_resource
    }
}

impl super::core::AssetTrait for PhysicsResourceContainerAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for PhysicsResourceContainerAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PHYSICSRESOURCECONTAINERASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsResourceContainerAsset",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsResourceContainerAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "PhysicsResource",
                flags: MemberInfoFlags::new(0),
                field_type: "ResourceRef",
                rust_offset: offset_of!(PhysicsResourceContainerAsset, physics_resource),
            },
        ],
    }),
    array_type: Some(PHYSICSRESOURCECONTAINERASSET_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PhysicsResourceContainerAsset {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSRESOURCECONTAINERASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSRESOURCECONTAINERASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsResourceContainerAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsResourceContainerAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RagdollAsset {
    pub _glacier_base: super::core::Asset,
    pub material_pair: super::entity::MaterialDecl,
    pub resource: glacier_reflect::builtin::ResourceRef,
}

pub trait RagdollAssetTrait: super::core::AssetTrait {
    fn material_pair(&self) -> &super::entity::MaterialDecl;
    fn resource(&self) -> &glacier_reflect::builtin::ResourceRef;
}

impl RagdollAssetTrait for RagdollAsset {
    fn material_pair(&self) -> &super::entity::MaterialDecl {
        &self.material_pair
    }
    fn resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.resource
    }
}

impl super::core::AssetTrait for RagdollAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for RagdollAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static RAGDOLLASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RagdollAsset",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RagdollAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MaterialPair",
                flags: MemberInfoFlags::new(0),
                field_type: "MaterialDecl",
                rust_offset: offset_of!(RagdollAsset, material_pair),
            },
            FieldInfoData {
                name: "Resource",
                flags: MemberInfoFlags::new(0),
                field_type: "ResourceRef",
                rust_offset: offset_of!(RagdollAsset, resource),
            },
        ],
    }),
    array_type: Some(RAGDOLLASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RagdollAsset {
    fn type_info(&self) -> &'static TypeInfo {
        RAGDOLLASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RAGDOLLASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RagdollAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("RagdollAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct GroupHavokAsset {
    pub _glacier_base: HavokAsset,
    pub aabb: Vec<AssetAabbs>,
}

pub trait GroupHavokAssetTrait: HavokAssetTrait {
    fn aabb(&self) -> &Vec<AssetAabbs>;
}

impl GroupHavokAssetTrait for GroupHavokAsset {
    fn aabb(&self) -> &Vec<AssetAabbs> {
        &self.aabb
    }
}

impl HavokAssetTrait for GroupHavokAsset {
    fn external_assets(&self) -> &Vec<Option<Arc<Mutex<dyn super::core::DataContainerTrait>>>> {
        self._glacier_base.external_assets()
    }
    fn resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        self._glacier_base.resource()
    }
}

impl super::core::AssetTrait for GroupHavokAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for GroupHavokAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static GROUPHAVOKASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroupHavokAsset",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(HAVOKASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GroupHavokAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Aabb",
                flags: MemberInfoFlags::new(144),
                field_type: "AssetAabbs-Array",
                rust_offset: offset_of!(GroupHavokAsset, aabb),
            },
        ],
    }),
    array_type: Some(GROUPHAVOKASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GroupHavokAsset {
    fn type_info(&self) -> &'static TypeInfo {
        GROUPHAVOKASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static GROUPHAVOKASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroupHavokAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("GroupHavokAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AssetAabbs {
    pub part_aabb: Vec<super::core::AxisAlignedBox>,
}

pub trait AssetAabbsTrait: TypeObject {
    fn part_aabb(&self) -> &Vec<super::core::AxisAlignedBox>;
}

impl AssetAabbsTrait for AssetAabbs {
    fn part_aabb(&self) -> &Vec<super::core::AxisAlignedBox> {
        &self.part_aabb
    }
}

pub static ASSETAABBS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AssetAabbs",
    flags: MemberInfoFlags::new(73),
    module: "Physics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AssetAabbs as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "PartAabb",
                flags: MemberInfoFlags::new(144),
                field_type: "AxisAlignedBox-Array",
                rust_offset: offset_of!(AssetAabbs, part_aabb),
            },
        ],
    }),
    array_type: Some(ASSETAABBS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AssetAabbs {
    fn type_info(&self) -> &'static TypeInfo {
        ASSETAABBS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ASSETAABBS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AssetAabbs-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("AssetAabbs"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct HavokAsset {
    pub _glacier_base: super::core::Asset,
    pub external_assets: Vec<Option<Arc<Mutex<dyn super::core::DataContainerTrait>>>>,
    pub resource: glacier_reflect::builtin::ResourceRef,
}

pub trait HavokAssetTrait: super::core::AssetTrait {
    fn external_assets(&self) -> &Vec<Option<Arc<Mutex<dyn super::core::DataContainerTrait>>>>;
    fn resource(&self) -> &glacier_reflect::builtin::ResourceRef;
}

impl HavokAssetTrait for HavokAsset {
    fn external_assets(&self) -> &Vec<Option<Arc<Mutex<dyn super::core::DataContainerTrait>>>> {
        &self.external_assets
    }
    fn resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.resource
    }
}

impl super::core::AssetTrait for HavokAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for HavokAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static HAVOKASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HavokAsset",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<HavokAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ExternalAssets",
                flags: MemberInfoFlags::new(144),
                field_type: "DataContainer-Array",
                rust_offset: offset_of!(HavokAsset, external_assets),
            },
            FieldInfoData {
                name: "Resource",
                flags: MemberInfoFlags::new(0),
                field_type: "ResourceRef",
                rust_offset: offset_of!(HavokAsset, resource),
            },
        ],
    }),
    array_type: Some(HAVOKASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for HavokAsset {
    fn type_info(&self) -> &'static TypeInfo {
        HAVOKASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static HAVOKASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HavokAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("HavokAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct HeightfieldTestEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub skip_samples: i32,
}

pub trait HeightfieldTestEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn skip_samples(&self) -> &i32;
}

impl HeightfieldTestEntityDataTrait for HeightfieldTestEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn skip_samples(&self) -> &i32 {
        &self.skip_samples
    }
}

impl super::entity::EntityDataTrait for HeightfieldTestEntityData {
}

impl super::entity::GameObjectDataTrait for HeightfieldTestEntityData {
}

impl super::core::DataBusPeerTrait for HeightfieldTestEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for HeightfieldTestEntityData {
}

impl super::core::DataContainerTrait for HeightfieldTestEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static HEIGHTFIELDTESTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HeightfieldTestEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<HeightfieldTestEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(HeightfieldTestEntityData, realm),
            },
            FieldInfoData {
                name: "SkipSamples",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(HeightfieldTestEntityData, skip_samples),
            },
        ],
    }),
    array_type: Some(HEIGHTFIELDTESTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for HeightfieldTestEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        HEIGHTFIELDTESTENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static HEIGHTFIELDTESTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HeightfieldTestEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("HeightfieldTestEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CharacterPhysicsData {
    pub _glacier_base: super::core::Asset,
    pub poses: Vec<Option<Arc<Mutex<dyn CharacterPoseDataTrait>>>>,
    pub states: Vec<Option<Arc<Mutex<dyn CharacterStateDataTrait>>>>,
    pub default_state: CharacterStateType,
    pub sprint: Option<Arc<Mutex<dyn CharacterSprintDataTrait>>>,
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
    pub physics_body: Option<Arc<Mutex<dyn CharacterPhysicsBodyDataTrait>>>,
    pub check_support_raycast_count_moving: i32,
    pub check_support_lookahead_distance: f32,
    pub ray_start_height_on_ground: f32,
    pub ray_end_height_on_ground: f32,
    pub ray_start_height_in_air: f32,
    pub ray_end_height_in_air: f32,
    pub speed_for_moving_ray_casts: f32,
}

pub trait CharacterPhysicsDataTrait: super::core::AssetTrait {
    fn poses(&self) -> &Vec<Option<Arc<Mutex<dyn CharacterPoseDataTrait>>>>;
    fn states(&self) -> &Vec<Option<Arc<Mutex<dyn CharacterStateDataTrait>>>>;
    fn default_state(&self) -> &CharacterStateType;
    fn sprint(&self) -> &Option<Arc<Mutex<dyn CharacterSprintDataTrait>>>;
    fn material_pair(&self) -> &super::entity::MaterialDecl;
    fn pushable_object_weight(&self) -> &i32;
    fn mass(&self) -> &f32;
    fn max_ascend_angle(&self) -> &f32;
    fn allow_supported_slide_state(&self) -> &bool;
    fn slide_angle(&self) -> &f32;
    fn slide_speed_condition(&self) -> &f32;
    fn physical_radius(&self) -> &f32;
    fn enter_swim_state_depth(&self) -> &f32;
    fn exit_swim_state_depth(&self) -> &f32;
    fn input_acceleration(&self) -> &f32;
    fn ladder_accept_angle(&self) -> &f32;
    fn ladder_accept_angle_pitch(&self) -> &f32;
    fn jump_penalty_time(&self) -> &f32;
    fn jump_penalty_factor(&self) -> &f32;
    fn radius_to_predict_collision_on_characters(&self) -> &f32;
    fn allow_pose_change_during_transition(&self) -> &bool;
    fn auto_push_away_from_walls_in_prone(&self) -> &bool;
    fn physics_body(&self) -> &Option<Arc<Mutex<dyn CharacterPhysicsBodyDataTrait>>>;
    fn check_support_raycast_count_moving(&self) -> &i32;
    fn check_support_lookahead_distance(&self) -> &f32;
    fn ray_start_height_on_ground(&self) -> &f32;
    fn ray_end_height_on_ground(&self) -> &f32;
    fn ray_start_height_in_air(&self) -> &f32;
    fn ray_end_height_in_air(&self) -> &f32;
    fn speed_for_moving_ray_casts(&self) -> &f32;
}

impl CharacterPhysicsDataTrait for CharacterPhysicsData {
    fn poses(&self) -> &Vec<Option<Arc<Mutex<dyn CharacterPoseDataTrait>>>> {
        &self.poses
    }
    fn states(&self) -> &Vec<Option<Arc<Mutex<dyn CharacterStateDataTrait>>>> {
        &self.states
    }
    fn default_state(&self) -> &CharacterStateType {
        &self.default_state
    }
    fn sprint(&self) -> &Option<Arc<Mutex<dyn CharacterSprintDataTrait>>> {
        &self.sprint
    }
    fn material_pair(&self) -> &super::entity::MaterialDecl {
        &self.material_pair
    }
    fn pushable_object_weight(&self) -> &i32 {
        &self.pushable_object_weight
    }
    fn mass(&self) -> &f32 {
        &self.mass
    }
    fn max_ascend_angle(&self) -> &f32 {
        &self.max_ascend_angle
    }
    fn allow_supported_slide_state(&self) -> &bool {
        &self.allow_supported_slide_state
    }
    fn slide_angle(&self) -> &f32 {
        &self.slide_angle
    }
    fn slide_speed_condition(&self) -> &f32 {
        &self.slide_speed_condition
    }
    fn physical_radius(&self) -> &f32 {
        &self.physical_radius
    }
    fn enter_swim_state_depth(&self) -> &f32 {
        &self.enter_swim_state_depth
    }
    fn exit_swim_state_depth(&self) -> &f32 {
        &self.exit_swim_state_depth
    }
    fn input_acceleration(&self) -> &f32 {
        &self.input_acceleration
    }
    fn ladder_accept_angle(&self) -> &f32 {
        &self.ladder_accept_angle
    }
    fn ladder_accept_angle_pitch(&self) -> &f32 {
        &self.ladder_accept_angle_pitch
    }
    fn jump_penalty_time(&self) -> &f32 {
        &self.jump_penalty_time
    }
    fn jump_penalty_factor(&self) -> &f32 {
        &self.jump_penalty_factor
    }
    fn radius_to_predict_collision_on_characters(&self) -> &f32 {
        &self.radius_to_predict_collision_on_characters
    }
    fn allow_pose_change_during_transition(&self) -> &bool {
        &self.allow_pose_change_during_transition
    }
    fn auto_push_away_from_walls_in_prone(&self) -> &bool {
        &self.auto_push_away_from_walls_in_prone
    }
    fn physics_body(&self) -> &Option<Arc<Mutex<dyn CharacterPhysicsBodyDataTrait>>> {
        &self.physics_body
    }
    fn check_support_raycast_count_moving(&self) -> &i32 {
        &self.check_support_raycast_count_moving
    }
    fn check_support_lookahead_distance(&self) -> &f32 {
        &self.check_support_lookahead_distance
    }
    fn ray_start_height_on_ground(&self) -> &f32 {
        &self.ray_start_height_on_ground
    }
    fn ray_end_height_on_ground(&self) -> &f32 {
        &self.ray_end_height_on_ground
    }
    fn ray_start_height_in_air(&self) -> &f32 {
        &self.ray_start_height_in_air
    }
    fn ray_end_height_in_air(&self) -> &f32 {
        &self.ray_end_height_in_air
    }
    fn speed_for_moving_ray_casts(&self) -> &f32 {
        &self.speed_for_moving_ray_casts
    }
}

impl super::core::AssetTrait for CharacterPhysicsData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for CharacterPhysicsData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static CHARACTERPHYSICSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterPhysicsData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CharacterPhysicsData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Poses",
                flags: MemberInfoFlags::new(144),
                field_type: "CharacterPoseData-Array",
                rust_offset: offset_of!(CharacterPhysicsData, poses),
            },
            FieldInfoData {
                name: "States",
                flags: MemberInfoFlags::new(144),
                field_type: "CharacterStateData-Array",
                rust_offset: offset_of!(CharacterPhysicsData, states),
            },
            FieldInfoData {
                name: "DefaultState",
                flags: MemberInfoFlags::new(0),
                field_type: "CharacterStateType",
                rust_offset: offset_of!(CharacterPhysicsData, default_state),
            },
            FieldInfoData {
                name: "Sprint",
                flags: MemberInfoFlags::new(0),
                field_type: "CharacterSprintData",
                rust_offset: offset_of!(CharacterPhysicsData, sprint),
            },
            FieldInfoData {
                name: "MaterialPair",
                flags: MemberInfoFlags::new(0),
                field_type: "MaterialDecl",
                rust_offset: offset_of!(CharacterPhysicsData, material_pair),
            },
            FieldInfoData {
                name: "PushableObjectWeight",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(CharacterPhysicsData, pushable_object_weight),
            },
            FieldInfoData {
                name: "Mass",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CharacterPhysicsData, mass),
            },
            FieldInfoData {
                name: "MaxAscendAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CharacterPhysicsData, max_ascend_angle),
            },
            FieldInfoData {
                name: "AllowSupportedSlideState",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CharacterPhysicsData, allow_supported_slide_state),
            },
            FieldInfoData {
                name: "SlideAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CharacterPhysicsData, slide_angle),
            },
            FieldInfoData {
                name: "SlideSpeedCondition",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CharacterPhysicsData, slide_speed_condition),
            },
            FieldInfoData {
                name: "PhysicalRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CharacterPhysicsData, physical_radius),
            },
            FieldInfoData {
                name: "EnterSwimStateDepth",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CharacterPhysicsData, enter_swim_state_depth),
            },
            FieldInfoData {
                name: "ExitSwimStateDepth",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CharacterPhysicsData, exit_swim_state_depth),
            },
            FieldInfoData {
                name: "InputAcceleration",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CharacterPhysicsData, input_acceleration),
            },
            FieldInfoData {
                name: "LadderAcceptAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CharacterPhysicsData, ladder_accept_angle),
            },
            FieldInfoData {
                name: "LadderAcceptAnglePitch",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CharacterPhysicsData, ladder_accept_angle_pitch),
            },
            FieldInfoData {
                name: "JumpPenaltyTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CharacterPhysicsData, jump_penalty_time),
            },
            FieldInfoData {
                name: "JumpPenaltyFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CharacterPhysicsData, jump_penalty_factor),
            },
            FieldInfoData {
                name: "RadiusToPredictCollisionOnCharacters",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CharacterPhysicsData, radius_to_predict_collision_on_characters),
            },
            FieldInfoData {
                name: "AllowPoseChangeDuringTransition",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CharacterPhysicsData, allow_pose_change_during_transition),
            },
            FieldInfoData {
                name: "AutoPushAwayFromWallsInProne",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CharacterPhysicsData, auto_push_away_from_walls_in_prone),
            },
            FieldInfoData {
                name: "PhysicsBody",
                flags: MemberInfoFlags::new(0),
                field_type: "CharacterPhysicsBodyData",
                rust_offset: offset_of!(CharacterPhysicsData, physics_body),
            },
            FieldInfoData {
                name: "CheckSupportRaycastCountMoving",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(CharacterPhysicsData, check_support_raycast_count_moving),
            },
            FieldInfoData {
                name: "CheckSupportLookaheadDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CharacterPhysicsData, check_support_lookahead_distance),
            },
            FieldInfoData {
                name: "RayStartHeightOnGround",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CharacterPhysicsData, ray_start_height_on_ground),
            },
            FieldInfoData {
                name: "RayEndHeightOnGround",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CharacterPhysicsData, ray_end_height_on_ground),
            },
            FieldInfoData {
                name: "RayStartHeightInAir",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CharacterPhysicsData, ray_start_height_in_air),
            },
            FieldInfoData {
                name: "RayEndHeightInAir",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CharacterPhysicsData, ray_end_height_in_air),
            },
            FieldInfoData {
                name: "SpeedForMovingRayCasts",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CharacterPhysicsData, speed_for_moving_ray_casts),
            },
        ],
    }),
    array_type: Some(CHARACTERPHYSICSDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CharacterPhysicsData {
    fn type_info(&self) -> &'static TypeInfo {
        CHARACTERPHYSICSDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CHARACTERPHYSICSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterPhysicsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("CharacterPhysicsData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CharacterPhysicsBodyData {
    pub _glacier_base: PhysicsBodyData,
    pub character_physics: Option<Arc<Mutex<dyn CharacterPhysicsDataTrait>>>,
}

pub trait CharacterPhysicsBodyDataTrait: PhysicsBodyDataTrait {
    fn character_physics(&self) -> &Option<Arc<Mutex<dyn CharacterPhysicsDataTrait>>>;
}

impl CharacterPhysicsBodyDataTrait for CharacterPhysicsBodyData {
    fn character_physics(&self) -> &Option<Arc<Mutex<dyn CharacterPhysicsDataTrait>>> {
        &self.character_physics
    }
}

impl PhysicsBodyDataTrait for CharacterPhysicsBodyData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn rigid_body_type(&self) -> &RigidBodyType {
        self._glacier_base.rigid_body_type()
    }
    fn collision_layer(&self) -> &RigidBodyCollisionLayer {
        self._glacier_base.collision_layer()
    }
    fn motion_type(&self) -> &RigidBodyMotionType {
        self._glacier_base.motion_type()
    }
    fn quality_type(&self) -> &RigidBodyQualityType {
        self._glacier_base.quality_type()
    }
    fn asset(&self) -> &Option<Arc<Mutex<dyn super::core::AssetTrait>>> {
        self._glacier_base.asset()
    }
    fn transform_index(&self) -> &u8 {
        self._glacier_base.transform_index()
    }
    fn world_index(&self) -> &u8 {
        self._glacier_base.world_index()
    }
    fn collision_root_shape_index(&self) -> &u8 {
        self._glacier_base.collision_root_shape_index()
    }
    fn raycast_root_shape_index(&self) -> &u8 {
        self._glacier_base.raycast_root_shape_index()
    }
    fn add_to_spatial_query_manager(&self) -> &bool {
        self._glacier_base.add_to_spatial_query_manager()
    }
    fn physics_callback_handler(&self) -> &Option<Arc<Mutex<dyn PhysicsCallbackHandlerDataTrait>>> {
        self._glacier_base.physics_callback_handler()
    }
}

impl super::entity::EntityDataTrait for CharacterPhysicsBodyData {
}

impl super::entity::GameObjectDataTrait for CharacterPhysicsBodyData {
}

impl super::core::DataBusPeerTrait for CharacterPhysicsBodyData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for CharacterPhysicsBodyData {
}

impl super::core::DataContainerTrait for CharacterPhysicsBodyData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static CHARACTERPHYSICSBODYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterPhysicsBodyData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSBODYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CharacterPhysicsBodyData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "CharacterPhysics",
                flags: MemberInfoFlags::new(0),
                field_type: "CharacterPhysicsData",
                rust_offset: offset_of!(CharacterPhysicsBodyData, character_physics),
            },
        ],
    }),
    array_type: Some(CHARACTERPHYSICSBODYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CharacterPhysicsBodyData {
    fn type_info(&self) -> &'static TypeInfo {
        CHARACTERPHYSICSBODYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CHARACTERPHYSICSBODYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterPhysicsBodyData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("CharacterPhysicsBodyData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ParachuteStateData {
    pub _glacier_base: CharacterStateData,
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

pub trait ParachuteStateDataTrait: CharacterStateDataTrait {
    fn deploy_time(&self) -> &f32;
    fn terminal_velocity(&self) -> &f32;
    fn forward_drag_coefficient(&self) -> &f32;
    fn angle_of_attack(&self) -> &f32;
    fn bank_offset(&self) -> &f32;
    fn throttle_offset(&self) -> &f32;
    fn brake_offset(&self) -> &f32;
    fn max_roll_velocity(&self) -> &f32;
    fn max_pitch_velocity(&self) -> &f32;
    fn max_yaw_velocity(&self) -> &f32;
}

impl ParachuteStateDataTrait for ParachuteStateData {
    fn deploy_time(&self) -> &f32 {
        &self.deploy_time
    }
    fn terminal_velocity(&self) -> &f32 {
        &self.terminal_velocity
    }
    fn forward_drag_coefficient(&self) -> &f32 {
        &self.forward_drag_coefficient
    }
    fn angle_of_attack(&self) -> &f32 {
        &self.angle_of_attack
    }
    fn bank_offset(&self) -> &f32 {
        &self.bank_offset
    }
    fn throttle_offset(&self) -> &f32 {
        &self.throttle_offset
    }
    fn brake_offset(&self) -> &f32 {
        &self.brake_offset
    }
    fn max_roll_velocity(&self) -> &f32 {
        &self.max_roll_velocity
    }
    fn max_pitch_velocity(&self) -> &f32 {
        &self.max_pitch_velocity
    }
    fn max_yaw_velocity(&self) -> &f32 {
        &self.max_yaw_velocity
    }
}

impl CharacterStateDataTrait for ParachuteStateData {
    fn pose_info(&self) -> &Vec<Option<Arc<Mutex<dyn CharacterStatePoseInfoTrait>>>> {
        self._glacier_base.pose_info()
    }
}

impl super::core::DataContainerTrait for ParachuteStateData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PARACHUTESTATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ParachuteStateData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CHARACTERSTATEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ParachuteStateData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "DeployTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ParachuteStateData, deploy_time),
            },
            FieldInfoData {
                name: "TerminalVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ParachuteStateData, terminal_velocity),
            },
            FieldInfoData {
                name: "ForwardDragCoefficient",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ParachuteStateData, forward_drag_coefficient),
            },
            FieldInfoData {
                name: "AngleOfAttack",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ParachuteStateData, angle_of_attack),
            },
            FieldInfoData {
                name: "BankOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ParachuteStateData, bank_offset),
            },
            FieldInfoData {
                name: "ThrottleOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ParachuteStateData, throttle_offset),
            },
            FieldInfoData {
                name: "BrakeOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ParachuteStateData, brake_offset),
            },
            FieldInfoData {
                name: "MaxRollVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ParachuteStateData, max_roll_velocity),
            },
            FieldInfoData {
                name: "MaxPitchVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ParachuteStateData, max_pitch_velocity),
            },
            FieldInfoData {
                name: "MaxYawVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ParachuteStateData, max_yaw_velocity),
            },
        ],
    }),
    array_type: Some(PARACHUTESTATEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ParachuteStateData {
    fn type_info(&self) -> &'static TypeInfo {
        PARACHUTESTATEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PARACHUTESTATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ParachuteStateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ParachuteStateData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SwimmingStateData {
    pub _glacier_base: CharacterStateData,
    pub body_under_water: f32,
}

pub trait SwimmingStateDataTrait: CharacterStateDataTrait {
    fn body_under_water(&self) -> &f32;
}

impl SwimmingStateDataTrait for SwimmingStateData {
    fn body_under_water(&self) -> &f32 {
        &self.body_under_water
    }
}

impl CharacterStateDataTrait for SwimmingStateData {
    fn pose_info(&self) -> &Vec<Option<Arc<Mutex<dyn CharacterStatePoseInfoTrait>>>> {
        self._glacier_base.pose_info()
    }
}

impl super::core::DataContainerTrait for SwimmingStateData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static SWIMMINGSTATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SwimmingStateData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CHARACTERSTATEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SwimmingStateData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "BodyUnderWater",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SwimmingStateData, body_under_water),
            },
        ],
    }),
    array_type: Some(SWIMMINGSTATEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SwimmingStateData {
    fn type_info(&self) -> &'static TypeInfo {
        SWIMMINGSTATEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SWIMMINGSTATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SwimmingStateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("SwimmingStateData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct JumpStateData {
    pub _glacier_base: CharacterStateData,
    pub jump_height: f32,
    pub jump_effect_size: f32,
}

pub trait JumpStateDataTrait: CharacterStateDataTrait {
    fn jump_height(&self) -> &f32;
    fn jump_effect_size(&self) -> &f32;
}

impl JumpStateDataTrait for JumpStateData {
    fn jump_height(&self) -> &f32 {
        &self.jump_height
    }
    fn jump_effect_size(&self) -> &f32 {
        &self.jump_effect_size
    }
}

impl CharacterStateDataTrait for JumpStateData {
    fn pose_info(&self) -> &Vec<Option<Arc<Mutex<dyn CharacterStatePoseInfoTrait>>>> {
        self._glacier_base.pose_info()
    }
}

impl super::core::DataContainerTrait for JumpStateData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static JUMPSTATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "JumpStateData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CHARACTERSTATEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<JumpStateData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "JumpHeight",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(JumpStateData, jump_height),
            },
            FieldInfoData {
                name: "JumpEffectSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(JumpStateData, jump_effect_size),
            },
        ],
    }),
    array_type: Some(JUMPSTATEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for JumpStateData {
    fn type_info(&self) -> &'static TypeInfo {
        JUMPSTATEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static JUMPSTATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "JumpStateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("JumpStateData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClimbingStateData {
    pub _glacier_base: CharacterStateData,
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

pub trait ClimbingStateDataTrait: CharacterStateDataTrait {
    fn lateral_input_scale(&self) -> &f32;
    fn down_angle_limit(&self) -> &f32;
    fn drop_off_angle(&self) -> &f32;
    fn attraction_velocity(&self) -> &f32;
    fn push_away_velocity(&self) -> &f32;
    fn climb_height_offset(&self) -> &f32;
    fn climb_off_vertical_distance(&self) -> &f32;
    fn climb_off_vertical_time(&self) -> &f32;
    fn climb_off_horizontal_distance(&self) -> &f32;
    fn climb_off_horizontal_time(&self) -> &f32;
}

impl ClimbingStateDataTrait for ClimbingStateData {
    fn lateral_input_scale(&self) -> &f32 {
        &self.lateral_input_scale
    }
    fn down_angle_limit(&self) -> &f32 {
        &self.down_angle_limit
    }
    fn drop_off_angle(&self) -> &f32 {
        &self.drop_off_angle
    }
    fn attraction_velocity(&self) -> &f32 {
        &self.attraction_velocity
    }
    fn push_away_velocity(&self) -> &f32 {
        &self.push_away_velocity
    }
    fn climb_height_offset(&self) -> &f32 {
        &self.climb_height_offset
    }
    fn climb_off_vertical_distance(&self) -> &f32 {
        &self.climb_off_vertical_distance
    }
    fn climb_off_vertical_time(&self) -> &f32 {
        &self.climb_off_vertical_time
    }
    fn climb_off_horizontal_distance(&self) -> &f32 {
        &self.climb_off_horizontal_distance
    }
    fn climb_off_horizontal_time(&self) -> &f32 {
        &self.climb_off_horizontal_time
    }
}

impl CharacterStateDataTrait for ClimbingStateData {
    fn pose_info(&self) -> &Vec<Option<Arc<Mutex<dyn CharacterStatePoseInfoTrait>>>> {
        self._glacier_base.pose_info()
    }
}

impl super::core::DataContainerTrait for ClimbingStateData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static CLIMBINGSTATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClimbingStateData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CHARACTERSTATEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClimbingStateData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "LateralInputScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ClimbingStateData, lateral_input_scale),
            },
            FieldInfoData {
                name: "DownAngleLimit",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ClimbingStateData, down_angle_limit),
            },
            FieldInfoData {
                name: "DropOffAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ClimbingStateData, drop_off_angle),
            },
            FieldInfoData {
                name: "AttractionVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ClimbingStateData, attraction_velocity),
            },
            FieldInfoData {
                name: "PushAwayVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ClimbingStateData, push_away_velocity),
            },
            FieldInfoData {
                name: "ClimbHeightOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ClimbingStateData, climb_height_offset),
            },
            FieldInfoData {
                name: "ClimbOffVerticalDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ClimbingStateData, climb_off_vertical_distance),
            },
            FieldInfoData {
                name: "ClimbOffVerticalTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ClimbingStateData, climb_off_vertical_time),
            },
            FieldInfoData {
                name: "ClimbOffHorizontalDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ClimbingStateData, climb_off_horizontal_distance),
            },
            FieldInfoData {
                name: "ClimbOffHorizontalTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ClimbingStateData, climb_off_horizontal_time),
            },
        ],
    }),
    array_type: Some(CLIMBINGSTATEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ClimbingStateData {
    fn type_info(&self) -> &'static TypeInfo {
        CLIMBINGSTATEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIMBINGSTATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClimbingStateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ClimbingStateData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FallingStateData {
    pub _glacier_base: CharacterStateData,
}

pub trait FallingStateDataTrait: CharacterStateDataTrait {
}

impl FallingStateDataTrait for FallingStateData {
}

impl CharacterStateDataTrait for FallingStateData {
    fn pose_info(&self) -> &Vec<Option<Arc<Mutex<dyn CharacterStatePoseInfoTrait>>>> {
        self._glacier_base.pose_info()
    }
}

impl super::core::DataContainerTrait for FallingStateData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static FALLINGSTATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FallingStateData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CHARACTERSTATEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FallingStateData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(FALLINGSTATEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FallingStateData {
    fn type_info(&self) -> &'static TypeInfo {
        FALLINGSTATEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FALLINGSTATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FallingStateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("FallingStateData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct InAirStateData {
    pub _glacier_base: CharacterStateData,
    pub free_fall_velocity: f32,
}

pub trait InAirStateDataTrait: CharacterStateDataTrait {
    fn free_fall_velocity(&self) -> &f32;
}

impl InAirStateDataTrait for InAirStateData {
    fn free_fall_velocity(&self) -> &f32 {
        &self.free_fall_velocity
    }
}

impl CharacterStateDataTrait for InAirStateData {
    fn pose_info(&self) -> &Vec<Option<Arc<Mutex<dyn CharacterStatePoseInfoTrait>>>> {
        self._glacier_base.pose_info()
    }
}

impl super::core::DataContainerTrait for InAirStateData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static INAIRSTATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InAirStateData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CHARACTERSTATEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InAirStateData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "FreeFallVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InAirStateData, free_fall_velocity),
            },
        ],
    }),
    array_type: Some(INAIRSTATEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for InAirStateData {
    fn type_info(&self) -> &'static TypeInfo {
        INAIRSTATEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static INAIRSTATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InAirStateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("InAirStateData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct OnGroundStateData {
    pub _glacier_base: CharacterStateData,
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

pub trait OnGroundStateDataTrait: CharacterStateDataTrait {
    fn jump_delay(&self) -> &f32;
    fn jump_stamina_penalty(&self) -> &f32;
    fn allowed_distance_from_ground(&self) -> &f32;
    fn fall_with_gravity_distance_from_ground(&self) -> &f32;
    fn clamp_surface_normal_angle(&self) -> &f32;
    fn ground_hugging(&self) -> &bool;
    fn limit_downward_velocity(&self) -> &bool;
    fn uphill_speed_modifier(&self) -> &f32;
    fn uphill_speed_modifier_max_angle(&self) -> &f32;
    fn downhill_speed_modifier(&self) -> &f32;
    fn downhill_speed_modifier_max_angle(&self) -> &f32;
    fn hill_speed_modifier_dead_zone(&self) -> &f32;
    fn character_spring_scale_moving_under_terrain(&self) -> &f32;
    fn character_spring_scale_moving_over_terrain(&self) -> &f32;
    fn character_spring_scale_still(&self) -> &f32;
}

impl OnGroundStateDataTrait for OnGroundStateData {
    fn jump_delay(&self) -> &f32 {
        &self.jump_delay
    }
    fn jump_stamina_penalty(&self) -> &f32 {
        &self.jump_stamina_penalty
    }
    fn allowed_distance_from_ground(&self) -> &f32 {
        &self.allowed_distance_from_ground
    }
    fn fall_with_gravity_distance_from_ground(&self) -> &f32 {
        &self.fall_with_gravity_distance_from_ground
    }
    fn clamp_surface_normal_angle(&self) -> &f32 {
        &self.clamp_surface_normal_angle
    }
    fn ground_hugging(&self) -> &bool {
        &self.ground_hugging
    }
    fn limit_downward_velocity(&self) -> &bool {
        &self.limit_downward_velocity
    }
    fn uphill_speed_modifier(&self) -> &f32 {
        &self.uphill_speed_modifier
    }
    fn uphill_speed_modifier_max_angle(&self) -> &f32 {
        &self.uphill_speed_modifier_max_angle
    }
    fn downhill_speed_modifier(&self) -> &f32 {
        &self.downhill_speed_modifier
    }
    fn downhill_speed_modifier_max_angle(&self) -> &f32 {
        &self.downhill_speed_modifier_max_angle
    }
    fn hill_speed_modifier_dead_zone(&self) -> &f32 {
        &self.hill_speed_modifier_dead_zone
    }
    fn character_spring_scale_moving_under_terrain(&self) -> &f32 {
        &self.character_spring_scale_moving_under_terrain
    }
    fn character_spring_scale_moving_over_terrain(&self) -> &f32 {
        &self.character_spring_scale_moving_over_terrain
    }
    fn character_spring_scale_still(&self) -> &f32 {
        &self.character_spring_scale_still
    }
}

impl CharacterStateDataTrait for OnGroundStateData {
    fn pose_info(&self) -> &Vec<Option<Arc<Mutex<dyn CharacterStatePoseInfoTrait>>>> {
        self._glacier_base.pose_info()
    }
}

impl super::core::DataContainerTrait for OnGroundStateData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static ONGROUNDSTATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnGroundStateData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CHARACTERSTATEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OnGroundStateData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "JumpDelay",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OnGroundStateData, jump_delay),
            },
            FieldInfoData {
                name: "JumpStaminaPenalty",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OnGroundStateData, jump_stamina_penalty),
            },
            FieldInfoData {
                name: "AllowedDistanceFromGround",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OnGroundStateData, allowed_distance_from_ground),
            },
            FieldInfoData {
                name: "FallWithGravityDistanceFromGround",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OnGroundStateData, fall_with_gravity_distance_from_ground),
            },
            FieldInfoData {
                name: "ClampSurfaceNormalAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OnGroundStateData, clamp_surface_normal_angle),
            },
            FieldInfoData {
                name: "GroundHugging",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OnGroundStateData, ground_hugging),
            },
            FieldInfoData {
                name: "LimitDownwardVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OnGroundStateData, limit_downward_velocity),
            },
            FieldInfoData {
                name: "UphillSpeedModifier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OnGroundStateData, uphill_speed_modifier),
            },
            FieldInfoData {
                name: "UphillSpeedModifierMaxAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OnGroundStateData, uphill_speed_modifier_max_angle),
            },
            FieldInfoData {
                name: "DownhillSpeedModifier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OnGroundStateData, downhill_speed_modifier),
            },
            FieldInfoData {
                name: "DownhillSpeedModifierMaxAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OnGroundStateData, downhill_speed_modifier_max_angle),
            },
            FieldInfoData {
                name: "HillSpeedModifierDeadZone",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OnGroundStateData, hill_speed_modifier_dead_zone),
            },
            FieldInfoData {
                name: "CharacterSpringScaleMovingUnderTerrain",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OnGroundStateData, character_spring_scale_moving_under_terrain),
            },
            FieldInfoData {
                name: "CharacterSpringScaleMovingOverTerrain",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OnGroundStateData, character_spring_scale_moving_over_terrain),
            },
            FieldInfoData {
                name: "CharacterSpringScaleStill",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OnGroundStateData, character_spring_scale_still),
            },
        ],
    }),
    array_type: Some(ONGROUNDSTATEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OnGroundStateData {
    fn type_info(&self) -> &'static TypeInfo {
        ONGROUNDSTATEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ONGROUNDSTATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnGroundStateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("OnGroundStateData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SlidingStateData {
    pub _glacier_base: CharacterStateData,
    pub horizontal_input_scale: f32,
    pub gravity_scale: f32,
    pub character_spring_scale: f32,
    pub allowed_distance_from_ground: f32,
}

pub trait SlidingStateDataTrait: CharacterStateDataTrait {
    fn horizontal_input_scale(&self) -> &f32;
    fn gravity_scale(&self) -> &f32;
    fn character_spring_scale(&self) -> &f32;
    fn allowed_distance_from_ground(&self) -> &f32;
}

impl SlidingStateDataTrait for SlidingStateData {
    fn horizontal_input_scale(&self) -> &f32 {
        &self.horizontal_input_scale
    }
    fn gravity_scale(&self) -> &f32 {
        &self.gravity_scale
    }
    fn character_spring_scale(&self) -> &f32 {
        &self.character_spring_scale
    }
    fn allowed_distance_from_ground(&self) -> &f32 {
        &self.allowed_distance_from_ground
    }
}

impl CharacterStateDataTrait for SlidingStateData {
    fn pose_info(&self) -> &Vec<Option<Arc<Mutex<dyn CharacterStatePoseInfoTrait>>>> {
        self._glacier_base.pose_info()
    }
}

impl super::core::DataContainerTrait for SlidingStateData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static SLIDINGSTATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SlidingStateData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CHARACTERSTATEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SlidingStateData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "HorizontalInputScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SlidingStateData, horizontal_input_scale),
            },
            FieldInfoData {
                name: "GravityScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SlidingStateData, gravity_scale),
            },
            FieldInfoData {
                name: "CharacterSpringScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SlidingStateData, character_spring_scale),
            },
            FieldInfoData {
                name: "AllowedDistanceFromGround",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SlidingStateData, allowed_distance_from_ground),
            },
        ],
    }),
    array_type: Some(SLIDINGSTATEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SlidingStateData {
    fn type_info(&self) -> &'static TypeInfo {
        SLIDINGSTATEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SLIDINGSTATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SlidingStateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("SlidingStateData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AnimationControlledStateData {
    pub _glacier_base: CharacterStateData,
    pub up_normal_tolerance: f32,
    pub character_spring_strength: f32,
}

pub trait AnimationControlledStateDataTrait: CharacterStateDataTrait {
    fn up_normal_tolerance(&self) -> &f32;
    fn character_spring_strength(&self) -> &f32;
}

impl AnimationControlledStateDataTrait for AnimationControlledStateData {
    fn up_normal_tolerance(&self) -> &f32 {
        &self.up_normal_tolerance
    }
    fn character_spring_strength(&self) -> &f32 {
        &self.character_spring_strength
    }
}

impl CharacterStateDataTrait for AnimationControlledStateData {
    fn pose_info(&self) -> &Vec<Option<Arc<Mutex<dyn CharacterStatePoseInfoTrait>>>> {
        self._glacier_base.pose_info()
    }
}

impl super::core::DataContainerTrait for AnimationControlledStateData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static ANIMATIONCONTROLLEDSTATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimationControlledStateData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CHARACTERSTATEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AnimationControlledStateData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "UpNormalTolerance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AnimationControlledStateData, up_normal_tolerance),
            },
            FieldInfoData {
                name: "CharacterSpringStrength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AnimationControlledStateData, character_spring_strength),
            },
        ],
    }),
    array_type: Some(ANIMATIONCONTROLLEDSTATEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AnimationControlledStateData {
    fn type_info(&self) -> &'static TypeInfo {
        ANIMATIONCONTROLLEDSTATEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ANIMATIONCONTROLLEDSTATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimationControlledStateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("AnimationControlledStateData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CharacterStateData {
    pub _glacier_base: super::core::DataContainer,
    pub pose_info: Vec<Option<Arc<Mutex<dyn CharacterStatePoseInfoTrait>>>>,
}

pub trait CharacterStateDataTrait: super::core::DataContainerTrait {
    fn pose_info(&self) -> &Vec<Option<Arc<Mutex<dyn CharacterStatePoseInfoTrait>>>>;
}

impl CharacterStateDataTrait for CharacterStateData {
    fn pose_info(&self) -> &Vec<Option<Arc<Mutex<dyn CharacterStatePoseInfoTrait>>>> {
        &self.pose_info
    }
}

impl super::core::DataContainerTrait for CharacterStateData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static CHARACTERSTATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterStateData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CharacterStateData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "PoseInfo",
                flags: MemberInfoFlags::new(144),
                field_type: "CharacterStatePoseInfo-Array",
                rust_offset: offset_of!(CharacterStateData, pose_info),
            },
        ],
    }),
    array_type: Some(CHARACTERSTATEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CharacterStateData {
    fn type_info(&self) -> &'static TypeInfo {
        CHARACTERSTATEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CHARACTERSTATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterStateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("CharacterStateData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static CHARACTERSTATETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterStateType",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(CHARACTERSTATETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CharacterStateType {
    fn type_info(&self) -> &'static TypeInfo {
        CHARACTERSTATETYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CHARACTERSTATETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterStateType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("CharacterStateType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CharacterSprintData {
    pub _glacier_base: super::core::DataContainer,
    pub sprint_power_decrease_per_second: f32,
    pub sprint_power_increase_per_second: f32,
    pub sprint_minimum_power: f32,
    pub allow_continous_sprinting: bool,
}

pub trait CharacterSprintDataTrait: super::core::DataContainerTrait {
    fn sprint_power_decrease_per_second(&self) -> &f32;
    fn sprint_power_increase_per_second(&self) -> &f32;
    fn sprint_minimum_power(&self) -> &f32;
    fn allow_continous_sprinting(&self) -> &bool;
}

impl CharacterSprintDataTrait for CharacterSprintData {
    fn sprint_power_decrease_per_second(&self) -> &f32 {
        &self.sprint_power_decrease_per_second
    }
    fn sprint_power_increase_per_second(&self) -> &f32 {
        &self.sprint_power_increase_per_second
    }
    fn sprint_minimum_power(&self) -> &f32 {
        &self.sprint_minimum_power
    }
    fn allow_continous_sprinting(&self) -> &bool {
        &self.allow_continous_sprinting
    }
}

impl super::core::DataContainerTrait for CharacterSprintData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static CHARACTERSPRINTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterSprintData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CharacterSprintData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "SprintPowerDecreasePerSecond",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CharacterSprintData, sprint_power_decrease_per_second),
            },
            FieldInfoData {
                name: "SprintPowerIncreasePerSecond",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CharacterSprintData, sprint_power_increase_per_second),
            },
            FieldInfoData {
                name: "SprintMinimumPower",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CharacterSprintData, sprint_minimum_power),
            },
            FieldInfoData {
                name: "AllowContinousSprinting",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CharacterSprintData, allow_continous_sprinting),
            },
        ],
    }),
    array_type: Some(CHARACTERSPRINTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CharacterSprintData {
    fn type_info(&self) -> &'static TypeInfo {
        CHARACTERSPRINTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CHARACTERSPRINTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterSprintData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("CharacterSprintData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CharacterStatePoseInfo {
    pub _glacier_base: super::core::DataContainer,
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

pub trait CharacterStatePoseInfoTrait: super::core::DataContainerTrait {
    fn pose_type(&self) -> &CharacterPoseType;
    fn velocity(&self) -> &f32;
    fn acceleration_gain(&self) -> &f32;
    fn deceleration_gain(&self) -> &f32;
    fn direction_change_acceleration_gain(&self) -> &f32;
    fn direction_change_threshold(&self) -> &f32;
    fn sprint_gain(&self) -> &f32;
    fn sprint_multiplier(&self) -> &f32;
    fn speed_modifier(&self) -> &SpeedModifierData;
    fn shallow_water_multiplier(&self) -> &f32;
}

impl CharacterStatePoseInfoTrait for CharacterStatePoseInfo {
    fn pose_type(&self) -> &CharacterPoseType {
        &self.pose_type
    }
    fn velocity(&self) -> &f32 {
        &self.velocity
    }
    fn acceleration_gain(&self) -> &f32 {
        &self.acceleration_gain
    }
    fn deceleration_gain(&self) -> &f32 {
        &self.deceleration_gain
    }
    fn direction_change_acceleration_gain(&self) -> &f32 {
        &self.direction_change_acceleration_gain
    }
    fn direction_change_threshold(&self) -> &f32 {
        &self.direction_change_threshold
    }
    fn sprint_gain(&self) -> &f32 {
        &self.sprint_gain
    }
    fn sprint_multiplier(&self) -> &f32 {
        &self.sprint_multiplier
    }
    fn speed_modifier(&self) -> &SpeedModifierData {
        &self.speed_modifier
    }
    fn shallow_water_multiplier(&self) -> &f32 {
        &self.shallow_water_multiplier
    }
}

impl super::core::DataContainerTrait for CharacterStatePoseInfo {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static CHARACTERSTATEPOSEINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterStatePoseInfo",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CharacterStatePoseInfo as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "PoseType",
                flags: MemberInfoFlags::new(0),
                field_type: "CharacterPoseType",
                rust_offset: offset_of!(CharacterStatePoseInfo, pose_type),
            },
            FieldInfoData {
                name: "Velocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CharacterStatePoseInfo, velocity),
            },
            FieldInfoData {
                name: "AccelerationGain",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CharacterStatePoseInfo, acceleration_gain),
            },
            FieldInfoData {
                name: "DecelerationGain",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CharacterStatePoseInfo, deceleration_gain),
            },
            FieldInfoData {
                name: "DirectionChangeAccelerationGain",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CharacterStatePoseInfo, direction_change_acceleration_gain),
            },
            FieldInfoData {
                name: "DirectionChangeThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CharacterStatePoseInfo, direction_change_threshold),
            },
            FieldInfoData {
                name: "SprintGain",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CharacterStatePoseInfo, sprint_gain),
            },
            FieldInfoData {
                name: "SprintMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CharacterStatePoseInfo, sprint_multiplier),
            },
            FieldInfoData {
                name: "SpeedModifier",
                flags: MemberInfoFlags::new(0),
                field_type: "SpeedModifierData",
                rust_offset: offset_of!(CharacterStatePoseInfo, speed_modifier),
            },
            FieldInfoData {
                name: "ShallowWaterMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CharacterStatePoseInfo, shallow_water_multiplier),
            },
        ],
    }),
    array_type: Some(CHARACTERSTATEPOSEINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CharacterStatePoseInfo {
    fn type_info(&self) -> &'static TypeInfo {
        CHARACTERSTATEPOSEINFO_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CHARACTERSTATEPOSEINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterStatePoseInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("CharacterStatePoseInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SpeedModifierData {
    pub forward_constant: f32,
    pub backward_constant: f32,
    pub left_constant: f32,
    pub right_constant: f32,
}

pub trait SpeedModifierDataTrait: TypeObject {
    fn forward_constant(&self) -> &f32;
    fn backward_constant(&self) -> &f32;
    fn left_constant(&self) -> &f32;
    fn right_constant(&self) -> &f32;
}

impl SpeedModifierDataTrait for SpeedModifierData {
    fn forward_constant(&self) -> &f32 {
        &self.forward_constant
    }
    fn backward_constant(&self) -> &f32 {
        &self.backward_constant
    }
    fn left_constant(&self) -> &f32 {
        &self.left_constant
    }
    fn right_constant(&self) -> &f32 {
        &self.right_constant
    }
}

pub static SPEEDMODIFIERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpeedModifierData",
    flags: MemberInfoFlags::new(36937),
    module: "Physics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SpeedModifierData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ForwardConstant",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SpeedModifierData, forward_constant),
            },
            FieldInfoData {
                name: "BackwardConstant",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SpeedModifierData, backward_constant),
            },
            FieldInfoData {
                name: "LeftConstant",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SpeedModifierData, left_constant),
            },
            FieldInfoData {
                name: "RightConstant",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SpeedModifierData, right_constant),
            },
        ],
    }),
    array_type: Some(SPEEDMODIFIERDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for SpeedModifierData {
    fn type_info(&self) -> &'static TypeInfo {
        SPEEDMODIFIERDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SPEEDMODIFIERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpeedModifierData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("SpeedModifierData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CharacterPoseData {
    pub _glacier_base: super::core::DataContainer,
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

pub trait CharacterPoseDataTrait: super::core::DataContainerTrait {
    fn eye_position(&self) -> &super::core::Vec3;
    fn throttle_modifier_curve(&self) -> &Vec<super::core::Vec2>;
    fn step_height(&self) -> &f32;
    fn height(&self) -> &f32;
    fn transition_times(&self) -> &Vec<PoseTransitionTime>;
    fn look_constraints(&self) -> &LookConstraintsData;
    fn pose_type(&self) -> &CharacterPoseType;
    fn collision_type(&self) -> &CharacterPoseCollisionType;
    fn collision_box_min_expand(&self) -> &super::core::Vec3;
    fn collision_box_max_expand(&self) -> &super::core::Vec3;
    fn raycast_radius_padding(&self) -> &f32;
    fn raycast_height_padding(&self) -> &f32;
    fn raycast_offset(&self) -> &super::core::Vec3;
}

impl CharacterPoseDataTrait for CharacterPoseData {
    fn eye_position(&self) -> &super::core::Vec3 {
        &self.eye_position
    }
    fn throttle_modifier_curve(&self) -> &Vec<super::core::Vec2> {
        &self.throttle_modifier_curve
    }
    fn step_height(&self) -> &f32 {
        &self.step_height
    }
    fn height(&self) -> &f32 {
        &self.height
    }
    fn transition_times(&self) -> &Vec<PoseTransitionTime> {
        &self.transition_times
    }
    fn look_constraints(&self) -> &LookConstraintsData {
        &self.look_constraints
    }
    fn pose_type(&self) -> &CharacterPoseType {
        &self.pose_type
    }
    fn collision_type(&self) -> &CharacterPoseCollisionType {
        &self.collision_type
    }
    fn collision_box_min_expand(&self) -> &super::core::Vec3 {
        &self.collision_box_min_expand
    }
    fn collision_box_max_expand(&self) -> &super::core::Vec3 {
        &self.collision_box_max_expand
    }
    fn raycast_radius_padding(&self) -> &f32 {
        &self.raycast_radius_padding
    }
    fn raycast_height_padding(&self) -> &f32 {
        &self.raycast_height_padding
    }
    fn raycast_offset(&self) -> &super::core::Vec3 {
        &self.raycast_offset
    }
}

impl super::core::DataContainerTrait for CharacterPoseData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static CHARACTERPOSEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterPoseData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CharacterPoseData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "EyePosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(CharacterPoseData, eye_position),
            },
            FieldInfoData {
                name: "ThrottleModifierCurve",
                flags: MemberInfoFlags::new(144),
                field_type: "Vec2-Array",
                rust_offset: offset_of!(CharacterPoseData, throttle_modifier_curve),
            },
            FieldInfoData {
                name: "StepHeight",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CharacterPoseData, step_height),
            },
            FieldInfoData {
                name: "Height",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CharacterPoseData, height),
            },
            FieldInfoData {
                name: "TransitionTimes",
                flags: MemberInfoFlags::new(144),
                field_type: "PoseTransitionTime-Array",
                rust_offset: offset_of!(CharacterPoseData, transition_times),
            },
            FieldInfoData {
                name: "LookConstraints",
                flags: MemberInfoFlags::new(0),
                field_type: "LookConstraintsData",
                rust_offset: offset_of!(CharacterPoseData, look_constraints),
            },
            FieldInfoData {
                name: "PoseType",
                flags: MemberInfoFlags::new(0),
                field_type: "CharacterPoseType",
                rust_offset: offset_of!(CharacterPoseData, pose_type),
            },
            FieldInfoData {
                name: "CollisionType",
                flags: MemberInfoFlags::new(0),
                field_type: "CharacterPoseCollisionType",
                rust_offset: offset_of!(CharacterPoseData, collision_type),
            },
            FieldInfoData {
                name: "CollisionBoxMinExpand",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(CharacterPoseData, collision_box_min_expand),
            },
            FieldInfoData {
                name: "CollisionBoxMaxExpand",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(CharacterPoseData, collision_box_max_expand),
            },
            FieldInfoData {
                name: "RaycastRadiusPadding",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CharacterPoseData, raycast_radius_padding),
            },
            FieldInfoData {
                name: "RaycastHeightPadding",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CharacterPoseData, raycast_height_padding),
            },
            FieldInfoData {
                name: "RaycastOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(CharacterPoseData, raycast_offset),
            },
        ],
    }),
    array_type: Some(CHARACTERPOSEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CharacterPoseData {
    fn type_info(&self) -> &'static TypeInfo {
        CHARACTERPOSEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CHARACTERPOSEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterPoseData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("CharacterPoseData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PoseTransitionTime {
    pub to_pose: CharacterPoseType,
    pub transition_time: f32,
}

pub trait PoseTransitionTimeTrait: TypeObject {
    fn to_pose(&self) -> &CharacterPoseType;
    fn transition_time(&self) -> &f32;
}

impl PoseTransitionTimeTrait for PoseTransitionTime {
    fn to_pose(&self) -> &CharacterPoseType {
        &self.to_pose
    }
    fn transition_time(&self) -> &f32 {
        &self.transition_time
    }
}

pub static POSETRANSITIONTIME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PoseTransitionTime",
    flags: MemberInfoFlags::new(36937),
    module: "Physics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PoseTransitionTime as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ToPose",
                flags: MemberInfoFlags::new(0),
                field_type: "CharacterPoseType",
                rust_offset: offset_of!(PoseTransitionTime, to_pose),
            },
            FieldInfoData {
                name: "TransitionTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PoseTransitionTime, transition_time),
            },
        ],
    }),
    array_type: Some(POSETRANSITIONTIME_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for PoseTransitionTime {
    fn type_info(&self) -> &'static TypeInfo {
        POSETRANSITIONTIME_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static POSETRANSITIONTIME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PoseTransitionTime-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PoseTransitionTime"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum CharacterPoseCollisionType {
    #[default]
    CharacterPoseCollisionType_Capsule = 0,
    CharacterPoseCollisionType_Pencil = 1,
    CharacterPoseCollisionType_HorizontalCapsule = 2,
    CharacterPoseCollisionTypeCount = 3,
}

pub static CHARACTERPOSECOLLISIONTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterPoseCollisionType",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(CHARACTERPOSECOLLISIONTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CharacterPoseCollisionType {
    fn type_info(&self) -> &'static TypeInfo {
        CHARACTERPOSECOLLISIONTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CHARACTERPOSECOLLISIONTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterPoseCollisionType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("CharacterPoseCollisionType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum CharacterPoseType {
    #[default]
    CharacterPoseType_Stand = 0,
    CharacterPoseType_Crouch = 1,
    CharacterPoseType_Prone = 2,
    CharacterPoseTypeCount = 3,
}

pub static CHARACTERPOSETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterPoseType",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(CHARACTERPOSETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CharacterPoseType {
    fn type_info(&self) -> &'static TypeInfo {
        CHARACTERPOSETYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CHARACTERPOSETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterPoseType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("CharacterPoseType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CharacterPoseConstraintsData {
    pub stand_pose: bool,
    pub crouch_pose: bool,
    pub prone_pose: bool,
}

pub trait CharacterPoseConstraintsDataTrait: TypeObject {
    fn stand_pose(&self) -> &bool;
    fn crouch_pose(&self) -> &bool;
    fn prone_pose(&self) -> &bool;
}

impl CharacterPoseConstraintsDataTrait for CharacterPoseConstraintsData {
    fn stand_pose(&self) -> &bool {
        &self.stand_pose
    }
    fn crouch_pose(&self) -> &bool {
        &self.crouch_pose
    }
    fn prone_pose(&self) -> &bool {
        &self.prone_pose
    }
}

pub static CHARACTERPOSECONSTRAINTSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterPoseConstraintsData",
    flags: MemberInfoFlags::new(36937),
    module: "Physics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CharacterPoseConstraintsData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "StandPose",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CharacterPoseConstraintsData, stand_pose),
            },
            FieldInfoData {
                name: "CrouchPose",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CharacterPoseConstraintsData, crouch_pose),
            },
            FieldInfoData {
                name: "PronePose",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CharacterPoseConstraintsData, prone_pose),
            },
        ],
    }),
    array_type: Some(CHARACTERPOSECONSTRAINTSDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CharacterPoseConstraintsData {
    fn type_info(&self) -> &'static TypeInfo {
        CHARACTERPOSECONSTRAINTSDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CHARACTERPOSECONSTRAINTSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterPoseConstraintsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("CharacterPoseConstraintsData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LookConstraintsData {
    pub min_look_yaw: f32,
    pub max_look_yaw: f32,
    pub min_look_pitch: f32,
    pub max_look_pitch: f32,
}

pub trait LookConstraintsDataTrait: TypeObject {
    fn min_look_yaw(&self) -> &f32;
    fn max_look_yaw(&self) -> &f32;
    fn min_look_pitch(&self) -> &f32;
    fn max_look_pitch(&self) -> &f32;
}

impl LookConstraintsDataTrait for LookConstraintsData {
    fn min_look_yaw(&self) -> &f32 {
        &self.min_look_yaw
    }
    fn max_look_yaw(&self) -> &f32 {
        &self.max_look_yaw
    }
    fn min_look_pitch(&self) -> &f32 {
        &self.min_look_pitch
    }
    fn max_look_pitch(&self) -> &f32 {
        &self.max_look_pitch
    }
}

pub static LOOKCONSTRAINTSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LookConstraintsData",
    flags: MemberInfoFlags::new(36937),
    module: "Physics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LookConstraintsData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MinLookYaw",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LookConstraintsData, min_look_yaw),
            },
            FieldInfoData {
                name: "MaxLookYaw",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LookConstraintsData, max_look_yaw),
            },
            FieldInfoData {
                name: "MinLookPitch",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LookConstraintsData, min_look_pitch),
            },
            FieldInfoData {
                name: "MaxLookPitch",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LookConstraintsData, max_look_pitch),
            },
        ],
    }),
    array_type: Some(LOOKCONSTRAINTSDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for LookConstraintsData {
    fn type_info(&self) -> &'static TypeInfo {
        LOOKCONSTRAINTSDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LOOKCONSTRAINTSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LookConstraintsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("LookConstraintsData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VehicleSpawnStateEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub initial_velocity: super::core::Vec3,
    pub startup_delay_modifier: f32,
    pub initial_throttle: f32,
    pub engine_index: u32,
    pub initial_rpm_modifier: f32,
    pub height_offset: f32,
    pub radius: f32,
}

pub trait VehicleSpawnStateEntityDataTrait: super::entity::EntityDataTrait {
    fn initial_velocity(&self) -> &super::core::Vec3;
    fn startup_delay_modifier(&self) -> &f32;
    fn initial_throttle(&self) -> &f32;
    fn engine_index(&self) -> &u32;
    fn initial_rpm_modifier(&self) -> &f32;
    fn height_offset(&self) -> &f32;
    fn radius(&self) -> &f32;
}

impl VehicleSpawnStateEntityDataTrait for VehicleSpawnStateEntityData {
    fn initial_velocity(&self) -> &super::core::Vec3 {
        &self.initial_velocity
    }
    fn startup_delay_modifier(&self) -> &f32 {
        &self.startup_delay_modifier
    }
    fn initial_throttle(&self) -> &f32 {
        &self.initial_throttle
    }
    fn engine_index(&self) -> &u32 {
        &self.engine_index
    }
    fn initial_rpm_modifier(&self) -> &f32 {
        &self.initial_rpm_modifier
    }
    fn height_offset(&self) -> &f32 {
        &self.height_offset
    }
    fn radius(&self) -> &f32 {
        &self.radius
    }
}

impl super::entity::EntityDataTrait for VehicleSpawnStateEntityData {
}

impl super::entity::GameObjectDataTrait for VehicleSpawnStateEntityData {
}

impl super::core::DataBusPeerTrait for VehicleSpawnStateEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for VehicleSpawnStateEntityData {
}

impl super::core::DataContainerTrait for VehicleSpawnStateEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static VEHICLESPAWNSTATEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleSpawnStateEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VehicleSpawnStateEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "InitialVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(VehicleSpawnStateEntityData, initial_velocity),
            },
            FieldInfoData {
                name: "StartupDelayModifier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleSpawnStateEntityData, startup_delay_modifier),
            },
            FieldInfoData {
                name: "InitialThrottle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleSpawnStateEntityData, initial_throttle),
            },
            FieldInfoData {
                name: "EngineIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VehicleSpawnStateEntityData, engine_index),
            },
            FieldInfoData {
                name: "InitialRpmModifier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleSpawnStateEntityData, initial_rpm_modifier),
            },
            FieldInfoData {
                name: "HeightOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleSpawnStateEntityData, height_offset),
            },
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleSpawnStateEntityData, radius),
            },
        ],
    }),
    array_type: Some(VEHICLESPAWNSTATEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for VehicleSpawnStateEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        VEHICLESPAWNSTATEENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VEHICLESPAWNSTATEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleSpawnStateEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("VehicleSpawnStateEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LinearMovingBodyData {
    pub _glacier_base: MovingBodyData,
    pub axis: super::core::Vec3,
    pub start: EndPointData,
    pub end: EndPointData,
}

pub trait LinearMovingBodyDataTrait: MovingBodyDataTrait {
    fn axis(&self) -> &super::core::Vec3;
    fn start(&self) -> &EndPointData;
    fn end(&self) -> &EndPointData;
}

impl LinearMovingBodyDataTrait for LinearMovingBodyData {
    fn axis(&self) -> &super::core::Vec3 {
        &self.axis
    }
    fn start(&self) -> &EndPointData {
        &self.start
    }
    fn end(&self) -> &EndPointData {
        &self.end
    }
}

impl MovingBodyDataTrait for LinearMovingBodyData {
    fn mass(&self) -> &f32 {
        self._glacier_base.mass()
    }
    fn inertia(&self) -> &f32 {
        self._glacier_base.inertia()
    }
}

impl super::core::DataContainerTrait for LinearMovingBodyData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static LINEARMOVINGBODYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMovingBodyData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MOVINGBODYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LinearMovingBodyData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Axis",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(LinearMovingBodyData, axis),
            },
            FieldInfoData {
                name: "Start",
                flags: MemberInfoFlags::new(0),
                field_type: "EndPointData",
                rust_offset: offset_of!(LinearMovingBodyData, start),
            },
            FieldInfoData {
                name: "End",
                flags: MemberInfoFlags::new(0),
                field_type: "EndPointData",
                rust_offset: offset_of!(LinearMovingBodyData, end),
            },
        ],
    }),
    array_type: Some(LINEARMOVINGBODYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LinearMovingBodyData {
    fn type_info(&self) -> &'static TypeInfo {
        LINEARMOVINGBODYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LINEARMOVINGBODYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMovingBodyData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("LinearMovingBodyData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EndPointData {
    pub pos: f32,
    pub end_damping: f32,
    pub spring_length: f32,
    pub spring_acceleration: f32,
    pub spring_damping: f32,
}

pub trait EndPointDataTrait: TypeObject {
    fn pos(&self) -> &f32;
    fn end_damping(&self) -> &f32;
    fn spring_length(&self) -> &f32;
    fn spring_acceleration(&self) -> &f32;
    fn spring_damping(&self) -> &f32;
}

impl EndPointDataTrait for EndPointData {
    fn pos(&self) -> &f32 {
        &self.pos
    }
    fn end_damping(&self) -> &f32 {
        &self.end_damping
    }
    fn spring_length(&self) -> &f32 {
        &self.spring_length
    }
    fn spring_acceleration(&self) -> &f32 {
        &self.spring_acceleration
    }
    fn spring_damping(&self) -> &f32 {
        &self.spring_damping
    }
}

pub static ENDPOINTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EndPointData",
    flags: MemberInfoFlags::new(36937),
    module: "Physics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EndPointData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Pos",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EndPointData, pos),
            },
            FieldInfoData {
                name: "EndDamping",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EndPointData, end_damping),
            },
            FieldInfoData {
                name: "SpringLength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EndPointData, spring_length),
            },
            FieldInfoData {
                name: "SpringAcceleration",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EndPointData, spring_acceleration),
            },
            FieldInfoData {
                name: "SpringDamping",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EndPointData, spring_damping),
            },
        ],
    }),
    array_type: Some(ENDPOINTDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for EndPointData {
    fn type_info(&self) -> &'static TypeInfo {
        ENDPOINTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ENDPOINTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EndPointData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("EndPointData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RotationBodyData {
    pub _glacier_base: MovingBodyData,
    pub angular_momentum_multiplier: f32,
    pub angular_momentum_threshold: f32,
    pub angular_momentum_damping: f32,
    pub angular_momentum_damping_acc: f32,
    pub angular_momentum_damping_deacc: f32,
    pub use_post_satisfy_constraints: bool,
    pub angular_constraint_min: f32,
    pub angular_constraint_max: f32,
    pub rotation_axis: i32,
    pub extended_constraints: Option<Arc<Mutex<dyn ExtendedConstraintsDataTrait>>>,
    pub use_angular_momentum_threshold: bool,
    pub use_angular_momentum_damping: bool,
    pub use_angular_constraint: bool,
}

pub trait RotationBodyDataTrait: MovingBodyDataTrait {
    fn angular_momentum_multiplier(&self) -> &f32;
    fn angular_momentum_threshold(&self) -> &f32;
    fn angular_momentum_damping(&self) -> &f32;
    fn angular_momentum_damping_acc(&self) -> &f32;
    fn angular_momentum_damping_deacc(&self) -> &f32;
    fn use_post_satisfy_constraints(&self) -> &bool;
    fn angular_constraint_min(&self) -> &f32;
    fn angular_constraint_max(&self) -> &f32;
    fn rotation_axis(&self) -> &i32;
    fn extended_constraints(&self) -> &Option<Arc<Mutex<dyn ExtendedConstraintsDataTrait>>>;
    fn use_angular_momentum_threshold(&self) -> &bool;
    fn use_angular_momentum_damping(&self) -> &bool;
    fn use_angular_constraint(&self) -> &bool;
}

impl RotationBodyDataTrait for RotationBodyData {
    fn angular_momentum_multiplier(&self) -> &f32 {
        &self.angular_momentum_multiplier
    }
    fn angular_momentum_threshold(&self) -> &f32 {
        &self.angular_momentum_threshold
    }
    fn angular_momentum_damping(&self) -> &f32 {
        &self.angular_momentum_damping
    }
    fn angular_momentum_damping_acc(&self) -> &f32 {
        &self.angular_momentum_damping_acc
    }
    fn angular_momentum_damping_deacc(&self) -> &f32 {
        &self.angular_momentum_damping_deacc
    }
    fn use_post_satisfy_constraints(&self) -> &bool {
        &self.use_post_satisfy_constraints
    }
    fn angular_constraint_min(&self) -> &f32 {
        &self.angular_constraint_min
    }
    fn angular_constraint_max(&self) -> &f32 {
        &self.angular_constraint_max
    }
    fn rotation_axis(&self) -> &i32 {
        &self.rotation_axis
    }
    fn extended_constraints(&self) -> &Option<Arc<Mutex<dyn ExtendedConstraintsDataTrait>>> {
        &self.extended_constraints
    }
    fn use_angular_momentum_threshold(&self) -> &bool {
        &self.use_angular_momentum_threshold
    }
    fn use_angular_momentum_damping(&self) -> &bool {
        &self.use_angular_momentum_damping
    }
    fn use_angular_constraint(&self) -> &bool {
        &self.use_angular_constraint
    }
}

impl MovingBodyDataTrait for RotationBodyData {
    fn mass(&self) -> &f32 {
        self._glacier_base.mass()
    }
    fn inertia(&self) -> &f32 {
        self._glacier_base.inertia()
    }
}

impl super::core::DataContainerTrait for RotationBodyData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static ROTATIONBODYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RotationBodyData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MOVINGBODYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RotationBodyData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "AngularMomentumMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RotationBodyData, angular_momentum_multiplier),
            },
            FieldInfoData {
                name: "AngularMomentumThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RotationBodyData, angular_momentum_threshold),
            },
            FieldInfoData {
                name: "AngularMomentumDamping",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RotationBodyData, angular_momentum_damping),
            },
            FieldInfoData {
                name: "AngularMomentumDampingAcc",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RotationBodyData, angular_momentum_damping_acc),
            },
            FieldInfoData {
                name: "AngularMomentumDampingDeacc",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RotationBodyData, angular_momentum_damping_deacc),
            },
            FieldInfoData {
                name: "UsePostSatisfyConstraints",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RotationBodyData, use_post_satisfy_constraints),
            },
            FieldInfoData {
                name: "AngularConstraintMin",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RotationBodyData, angular_constraint_min),
            },
            FieldInfoData {
                name: "AngularConstraintMax",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RotationBodyData, angular_constraint_max),
            },
            FieldInfoData {
                name: "RotationAxis",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(RotationBodyData, rotation_axis),
            },
            FieldInfoData {
                name: "ExtendedConstraints",
                flags: MemberInfoFlags::new(0),
                field_type: "ExtendedConstraintsData",
                rust_offset: offset_of!(RotationBodyData, extended_constraints),
            },
            FieldInfoData {
                name: "UseAngularMomentumThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RotationBodyData, use_angular_momentum_threshold),
            },
            FieldInfoData {
                name: "UseAngularMomentumDamping",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RotationBodyData, use_angular_momentum_damping),
            },
            FieldInfoData {
                name: "UseAngularConstraint",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RotationBodyData, use_angular_constraint),
            },
        ],
    }),
    array_type: Some(ROTATIONBODYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RotationBodyData {
    fn type_info(&self) -> &'static TypeInfo {
        ROTATIONBODYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ROTATIONBODYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RotationBodyData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("RotationBodyData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ExtendedConstraintsData {
    pub _glacier_base: super::core::DataContainer,
    pub heading: f32,
    pub width: f32,
    pub falloff: f32,
    pub angular_constraint_min: f32,
    pub angular_constraint_max: f32,
}

pub trait ExtendedConstraintsDataTrait: super::core::DataContainerTrait {
    fn heading(&self) -> &f32;
    fn width(&self) -> &f32;
    fn falloff(&self) -> &f32;
    fn angular_constraint_min(&self) -> &f32;
    fn angular_constraint_max(&self) -> &f32;
}

impl ExtendedConstraintsDataTrait for ExtendedConstraintsData {
    fn heading(&self) -> &f32 {
        &self.heading
    }
    fn width(&self) -> &f32 {
        &self.width
    }
    fn falloff(&self) -> &f32 {
        &self.falloff
    }
    fn angular_constraint_min(&self) -> &f32 {
        &self.angular_constraint_min
    }
    fn angular_constraint_max(&self) -> &f32 {
        &self.angular_constraint_max
    }
}

impl super::core::DataContainerTrait for ExtendedConstraintsData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static EXTENDEDCONSTRAINTSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExtendedConstraintsData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ExtendedConstraintsData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Heading",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ExtendedConstraintsData, heading),
            },
            FieldInfoData {
                name: "Width",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ExtendedConstraintsData, width),
            },
            FieldInfoData {
                name: "Falloff",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ExtendedConstraintsData, falloff),
            },
            FieldInfoData {
                name: "AngularConstraintMin",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ExtendedConstraintsData, angular_constraint_min),
            },
            FieldInfoData {
                name: "AngularConstraintMax",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ExtendedConstraintsData, angular_constraint_max),
            },
        ],
    }),
    array_type: Some(EXTENDEDCONSTRAINTSDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ExtendedConstraintsData {
    fn type_info(&self) -> &'static TypeInfo {
        EXTENDEDCONSTRAINTSDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static EXTENDEDCONSTRAINTSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExtendedConstraintsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ExtendedConstraintsData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MovingBodyData {
    pub _glacier_base: super::core::DataContainer,
    pub mass: f32,
    pub inertia: f32,
}

pub trait MovingBodyDataTrait: super::core::DataContainerTrait {
    fn mass(&self) -> &f32;
    fn inertia(&self) -> &f32;
}

impl MovingBodyDataTrait for MovingBodyData {
    fn mass(&self) -> &f32 {
        &self.mass
    }
    fn inertia(&self) -> &f32 {
        &self.inertia
    }
}

impl super::core::DataContainerTrait for MovingBodyData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static MOVINGBODYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovingBodyData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MovingBodyData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Mass",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MovingBodyData, mass),
            },
            FieldInfoData {
                name: "Inertia",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MovingBodyData, inertia),
            },
        ],
    }),
    array_type: Some(MOVINGBODYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MovingBodyData {
    fn type_info(&self) -> &'static TypeInfo {
        MOVINGBODYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MOVINGBODYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovingBodyData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("MovingBodyData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CameraLoosePartPhysicsData {
    pub _glacier_base: LoosePartPhysicsData,
    pub scale: f32,
}

pub trait CameraLoosePartPhysicsDataTrait: LoosePartPhysicsDataTrait {
    fn scale(&self) -> &f32;
}

impl CameraLoosePartPhysicsDataTrait for CameraLoosePartPhysicsData {
    fn scale(&self) -> &f32 {
        &self.scale
    }
}

impl LoosePartPhysicsDataTrait for CameraLoosePartPhysicsData {
    fn fake_physics(&self) -> &Option<Arc<Mutex<dyn FakePhysicsDataTrait>>> {
        self._glacier_base.fake_physics()
    }
}

impl super::core::DataContainerTrait for CameraLoosePartPhysicsData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static CAMERALOOSEPARTPHYSICSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraLoosePartPhysicsData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOOSEPARTPHYSICSDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CameraLoosePartPhysicsData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Scale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CameraLoosePartPhysicsData, scale),
            },
        ],
    }),
    array_type: Some(CAMERALOOSEPARTPHYSICSDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CameraLoosePartPhysicsData {
    fn type_info(&self) -> &'static TypeInfo {
        CAMERALOOSEPARTPHYSICSDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CAMERALOOSEPARTPHYSICSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraLoosePartPhysicsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("CameraLoosePartPhysicsData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct NetworkableLoosePartPhysicsData {
    pub _glacier_base: LoosePartPhysicsData,
    pub networked: bool,
}

pub trait NetworkableLoosePartPhysicsDataTrait: LoosePartPhysicsDataTrait {
    fn networked(&self) -> &bool;
}

impl NetworkableLoosePartPhysicsDataTrait for NetworkableLoosePartPhysicsData {
    fn networked(&self) -> &bool {
        &self.networked
    }
}

impl LoosePartPhysicsDataTrait for NetworkableLoosePartPhysicsData {
    fn fake_physics(&self) -> &Option<Arc<Mutex<dyn FakePhysicsDataTrait>>> {
        self._glacier_base.fake_physics()
    }
}

impl super::core::DataContainerTrait for NetworkableLoosePartPhysicsData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static NETWORKABLELOOSEPARTPHYSICSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkableLoosePartPhysicsData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOOSEPARTPHYSICSDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkableLoosePartPhysicsData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Networked",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(NetworkableLoosePartPhysicsData, networked),
            },
        ],
    }),
    array_type: Some(NETWORKABLELOOSEPARTPHYSICSDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NetworkableLoosePartPhysicsData {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKABLELOOSEPARTPHYSICSDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static NETWORKABLELOOSEPARTPHYSICSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkableLoosePartPhysicsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("NetworkableLoosePartPhysicsData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LoosePartPhysicsData {
    pub _glacier_base: super::core::DataContainer,
    pub fake_physics: Option<Arc<Mutex<dyn FakePhysicsDataTrait>>>,
}

pub trait LoosePartPhysicsDataTrait: super::core::DataContainerTrait {
    fn fake_physics(&self) -> &Option<Arc<Mutex<dyn FakePhysicsDataTrait>>>;
}

impl LoosePartPhysicsDataTrait for LoosePartPhysicsData {
    fn fake_physics(&self) -> &Option<Arc<Mutex<dyn FakePhysicsDataTrait>>> {
        &self.fake_physics
    }
}

impl super::core::DataContainerTrait for LoosePartPhysicsData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static LOOSEPARTPHYSICSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LoosePartPhysicsData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LoosePartPhysicsData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "FakePhysics",
                flags: MemberInfoFlags::new(0),
                field_type: "FakePhysicsData",
                rust_offset: offset_of!(LoosePartPhysicsData, fake_physics),
            },
        ],
    }),
    array_type: Some(LOOSEPARTPHYSICSDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LoosePartPhysicsData {
    fn type_info(&self) -> &'static TypeInfo {
        LOOSEPARTPHYSICSDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LOOSEPARTPHYSICSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LoosePartPhysicsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("LoosePartPhysicsData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WheelConfigMotorbikeData {
    pub _glacier_base: WheelConfigData,
    pub max_spring_force: f32,
    pub collision_yaw_dampening_duration: f32,
    pub collision_yaw_dampening: f32,
}

pub trait WheelConfigMotorbikeDataTrait: WheelConfigDataTrait {
    fn max_spring_force(&self) -> &f32;
    fn collision_yaw_dampening_duration(&self) -> &f32;
    fn collision_yaw_dampening(&self) -> &f32;
}

impl WheelConfigMotorbikeDataTrait for WheelConfigMotorbikeData {
    fn max_spring_force(&self) -> &f32 {
        &self.max_spring_force
    }
    fn collision_yaw_dampening_duration(&self) -> &f32 {
        &self.collision_yaw_dampening_duration
    }
    fn collision_yaw_dampening(&self) -> &f32 {
        &self.collision_yaw_dampening
    }
}

impl WheelConfigDataTrait for WheelConfigMotorbikeData {
    fn offset(&self) -> &super::core::Vec3 {
        self._glacier_base.offset()
    }
    fn inertia(&self) -> &super::core::Vec3 {
        self._glacier_base.inertia()
    }
    fn rotation_body(&self) -> &Option<Arc<Mutex<dyn RotationBodyDataTrait>>> {
        self._glacier_base.rotation_body()
    }
    fn sphere_collision(&self) -> &SphereCollisionData {
        self._glacier_base.sphere_collision()
    }
    fn spring(&self) -> &SpringData {
        self._glacier_base.spring()
    }
    fn mass(&self) -> &f32 {
        self._glacier_base.mass()
    }
    fn radius(&self) -> &f32 {
        self._glacier_base.radius()
    }
    fn width(&self) -> &f32 {
        self._glacier_base.width()
    }
    fn rolling_resistance_internal_base_factor(&self) -> &f32 {
        self._glacier_base.rolling_resistance_internal_base_factor()
    }
    fn rolling_resistance_base_factor(&self) -> &f32 {
        self._glacier_base.rolling_resistance_base_factor()
    }
    fn rolling_resistance_velocity_factor(&self) -> &f32 {
        self._glacier_base.rolling_resistance_velocity_factor()
    }
    fn engine_brake_velocity_factor(&self) -> &f32 {
        self._glacier_base.engine_brake_velocity_factor()
    }
    fn engine_brake_min_factor(&self) -> &f32 {
        self._glacier_base.engine_brake_min_factor()
    }
    fn engine_brake_max_factor(&self) -> &f32 {
        self._glacier_base.engine_brake_max_factor()
    }
    fn transmission_loss(&self) -> &Vec<super::core::Vec2> {
        self._glacier_base.transmission_loss()
    }
    fn diff_gear_ratio(&self) -> &f32 {
        self._glacier_base.diff_gear_ratio()
    }
    fn low_speed_steering_sensitivity(&self) -> &f32 {
        self._glacier_base.low_speed_steering_sensitivity()
    }
    fn mid_speed_steering_sensitivity(&self) -> &f32 {
        self._glacier_base.mid_speed_steering_sensitivity()
    }
    fn high_speed_steering_sensitivity(&self) -> &f32 {
        self._glacier_base.high_speed_steering_sensitivity()
    }
    fn sensitivity_range_scale(&self) -> &f32 {
        self._glacier_base.sensitivity_range_scale()
    }
    fn low_speed_steering_sensitivity_limit(&self) -> &f32 {
        self._glacier_base.low_speed_steering_sensitivity_limit()
    }
    fn mid_speed_steering_sensitivity_limit(&self) -> &f32 {
        self._glacier_base.mid_speed_steering_sensitivity_limit()
    }
    fn high_speed_steering_sensitivity_limit(&self) -> &f32 {
        self._glacier_base.high_speed_steering_sensitivity_limit()
    }
    fn tracked_turn_speed_limit(&self) -> &f32 {
        self._glacier_base.tracked_turn_speed_limit()
    }
    fn tracked_forward_speed_limit(&self) -> &f32 {
        self._glacier_base.tracked_forward_speed_limit()
    }
    fn tracked_slip_steer_reduction_scale(&self) -> &f32 {
        self._glacier_base.tracked_slip_steer_reduction_scale()
    }
    fn steer_inertia(&self) -> &Option<Arc<Mutex<dyn Curve2DTrait>>> {
        self._glacier_base.steer_inertia()
    }
    fn steering_sensitivity(&self) -> &Vec<SensitivityAtVelocity> {
        self._glacier_base.steering_sensitivity()
    }
    fn tracked_steering_boost_on_opposite_torque(&self) -> &f32 {
        self._glacier_base.tracked_steering_boost_on_opposite_torque()
    }
    fn slope_grip_min_angle(&self) -> &f32 {
        self._glacier_base.slope_grip_min_angle()
    }
    fn slope_grip_max_angle(&self) -> &f32 {
        self._glacier_base.slope_grip_max_angle()
    }
    fn slope_grip_exponent(&self) -> &i32 {
        self._glacier_base.slope_grip_exponent()
    }
    fn off_ground_gravity_modifier(&self) -> &f32 {
        self._glacier_base.off_ground_gravity_modifier()
    }
    fn side_slip_angle_max_slip_condition(&self) -> &f32 {
        self._glacier_base.side_slip_angle_max_slip_condition()
    }
    fn angular_velocity_min_slip_condition(&self) -> &f32 {
        self._glacier_base.angular_velocity_min_slip_condition()
    }
    fn wheel_velocity_x_min_slip_condition(&self) -> &f32 {
        self._glacier_base.wheel_velocity_x_min_slip_condition()
    }
    fn wheel_slip_ratio_max_slip_condition(&self) -> &f32 {
        self._glacier_base.wheel_slip_ratio_max_slip_condition()
    }
    fn longitudinal_friction_force_max_factor(&self) -> &f32 {
        self._glacier_base.longitudinal_friction_force_max_factor()
    }
    fn lateral_friction_force_max_factor(&self) -> &f32 {
        self._glacier_base.lateral_friction_force_max_factor()
    }
    fn allow_grip_slip_transition(&self) -> &bool {
        self._glacier_base.allow_grip_slip_transition()
    }
    fn longitude_friction_scale(&self) -> &Vec<FrictionScaleAtVelocity> {
        self._glacier_base.longitude_friction_scale()
    }
    fn lattitude_friction_scale(&self) -> &Vec<FrictionScaleAtVelocity> {
        self._glacier_base.lattitude_friction_scale()
    }
    fn wheel_friction_lattitude_brake_scale(&self) -> &f32 {
        self._glacier_base.wheel_friction_lattitude_brake_scale()
    }
    fn resistance(&self) -> &f32 {
        self._glacier_base.resistance()
    }
    fn fx_torque_radius_multiplier(&self) -> &f32 {
        self._glacier_base.fx_torque_radius_multiplier()
    }
    fn friction_moment_velocity_min(&self) -> &f32 {
        self._glacier_base.friction_moment_velocity_min()
    }
    fn friction_moment_velocity_max(&self) -> &f32 {
        self._glacier_base.friction_moment_velocity_max()
    }
    fn friction_moment_multiplier(&self) -> &f32 {
        self._glacier_base.friction_moment_multiplier()
    }
    fn friction_moment_max_factor(&self) -> &f32 {
        self._glacier_base.friction_moment_max_factor()
    }
    fn brake_factor(&self) -> &f32 {
        self._glacier_base.brake_factor()
    }
    fn brake_force(&self) -> &f32 {
        self._glacier_base.brake_force()
    }
    fn auto_hand_brake_if_no_throttle_and_steer(&self) -> &bool {
        self._glacier_base.auto_hand_brake_if_no_throttle_and_steer()
    }
    fn hand_brake_factor(&self) -> &f32 {
        self._glacier_base.hand_brake_factor()
    }
    fn hand_brake_force(&self) -> &f32 {
        self._glacier_base.hand_brake_force()
    }
    fn total_friction_scale(&self) -> &f32 {
        self._glacier_base.total_friction_scale()
    }
    fn total_lateral_friction_scale(&self) -> &f32 {
        self._glacier_base.total_lateral_friction_scale()
    }
    fn lateral_pos_k(&self) -> &f32 {
        self._glacier_base.lateral_pos_k()
    }
    fn lateral_neg_k(&self) -> &f32 {
        self._glacier_base.lateral_neg_k()
    }
    fn longitudinal_pos_k(&self) -> &f32 {
        self._glacier_base.longitudinal_pos_k()
    }
    fn longitudinal_neg_k(&self) -> &f32 {
        self._glacier_base.longitudinal_neg_k()
    }
    fn align_mom_scale(&self) -> &f32 {
        self._glacier_base.align_mom_scale()
    }
    fn wheel_base_lateral(&self) -> &f32 {
        self._glacier_base.wheel_base_lateral()
    }
    fn wheel_base_longitudinal(&self) -> &f32 {
        self._glacier_base.wheel_base_longitudinal()
    }
    fn driving_type(&self) -> &i32 {
        self._glacier_base.driving_type()
    }
    fn steering_type(&self) -> &i32 {
        self._glacier_base.steering_type()
    }
    fn friction_method(&self) -> &i32 {
        self._glacier_base.friction_method()
    }
    fn rotation_direction_index(&self) -> &i32 {
        self._glacier_base.rotation_direction_index()
    }
    fn steering_angle_index(&self) -> &i32 {
        self._glacier_base.steering_angle_index()
    }
    fn pacejka_config_index(&self) -> &i32 {
        self._glacier_base.pacejka_config_index()
    }
    fn engine_index(&self) -> &i32 {
        self._glacier_base.engine_index()
    }
    fn ackerman_device_type(&self) -> &i32 {
        self._glacier_base.ackerman_device_type()
    }
    fn use_rolling_resistance_velocity_factor(&self) -> &bool {
        self._glacier_base.use_rolling_resistance_velocity_factor()
    }
    fn use_rolling_resistance_base_factor(&self) -> &bool {
        self._glacier_base.use_rolling_resistance_base_factor()
    }
    fn use_engine_brake(&self) -> &bool {
        self._glacier_base.use_engine_brake()
    }
    fn is_allowed_to_spin(&self) -> &bool {
        self._glacier_base.is_allowed_to_spin()
    }
    fn has_steering_inverted(&self) -> &bool {
        self._glacier_base.has_steering_inverted()
    }
    fn use_friction_moment(&self) -> &bool {
        self._glacier_base.use_friction_moment()
    }
    fn use_low_speed_auto_brake(&self) -> &bool {
        self._glacier_base.use_low_speed_auto_brake()
    }
    fn adjust_wheel_rotation(&self) -> &bool {
        self._glacier_base.adjust_wheel_rotation()
    }
    fn collision_material_pair(&self) -> &super::entity::MaterialDecl {
        self._glacier_base.collision_material_pair()
    }
}

impl super::core::DataContainerTrait for WheelConfigMotorbikeData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static WHEELCONFIGMOTORBIKEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WheelConfigMotorbikeData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WHEELCONFIGDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WheelConfigMotorbikeData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MaxSpringForce",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigMotorbikeData, max_spring_force),
            },
            FieldInfoData {
                name: "CollisionYawDampeningDuration",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigMotorbikeData, collision_yaw_dampening_duration),
            },
            FieldInfoData {
                name: "CollisionYawDampening",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigMotorbikeData, collision_yaw_dampening),
            },
        ],
    }),
    array_type: Some(WHEELCONFIGMOTORBIKEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WheelConfigMotorbikeData {
    fn type_info(&self) -> &'static TypeInfo {
        WHEELCONFIGMOTORBIKEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WHEELCONFIGMOTORBIKEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WheelConfigMotorbikeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("WheelConfigMotorbikeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WheelConfigData {
    pub _glacier_base: super::core::DataContainer,
    pub offset: super::core::Vec3,
    pub inertia: super::core::Vec3,
    pub rotation_body: Option<Arc<Mutex<dyn RotationBodyDataTrait>>>,
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
    pub steer_inertia: Option<Arc<Mutex<dyn Curve2DTrait>>>,
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

pub trait WheelConfigDataTrait: super::core::DataContainerTrait {
    fn offset(&self) -> &super::core::Vec3;
    fn inertia(&self) -> &super::core::Vec3;
    fn rotation_body(&self) -> &Option<Arc<Mutex<dyn RotationBodyDataTrait>>>;
    fn sphere_collision(&self) -> &SphereCollisionData;
    fn spring(&self) -> &SpringData;
    fn mass(&self) -> &f32;
    fn radius(&self) -> &f32;
    fn width(&self) -> &f32;
    fn rolling_resistance_internal_base_factor(&self) -> &f32;
    fn rolling_resistance_base_factor(&self) -> &f32;
    fn rolling_resistance_velocity_factor(&self) -> &f32;
    fn engine_brake_velocity_factor(&self) -> &f32;
    fn engine_brake_min_factor(&self) -> &f32;
    fn engine_brake_max_factor(&self) -> &f32;
    fn transmission_loss(&self) -> &Vec<super::core::Vec2>;
    fn diff_gear_ratio(&self) -> &f32;
    fn low_speed_steering_sensitivity(&self) -> &f32;
    fn mid_speed_steering_sensitivity(&self) -> &f32;
    fn high_speed_steering_sensitivity(&self) -> &f32;
    fn sensitivity_range_scale(&self) -> &f32;
    fn low_speed_steering_sensitivity_limit(&self) -> &f32;
    fn mid_speed_steering_sensitivity_limit(&self) -> &f32;
    fn high_speed_steering_sensitivity_limit(&self) -> &f32;
    fn tracked_turn_speed_limit(&self) -> &f32;
    fn tracked_forward_speed_limit(&self) -> &f32;
    fn tracked_slip_steer_reduction_scale(&self) -> &f32;
    fn steer_inertia(&self) -> &Option<Arc<Mutex<dyn Curve2DTrait>>>;
    fn steering_sensitivity(&self) -> &Vec<SensitivityAtVelocity>;
    fn tracked_steering_boost_on_opposite_torque(&self) -> &f32;
    fn slope_grip_min_angle(&self) -> &f32;
    fn slope_grip_max_angle(&self) -> &f32;
    fn slope_grip_exponent(&self) -> &i32;
    fn off_ground_gravity_modifier(&self) -> &f32;
    fn side_slip_angle_max_slip_condition(&self) -> &f32;
    fn angular_velocity_min_slip_condition(&self) -> &f32;
    fn wheel_velocity_x_min_slip_condition(&self) -> &f32;
    fn wheel_slip_ratio_max_slip_condition(&self) -> &f32;
    fn longitudinal_friction_force_max_factor(&self) -> &f32;
    fn lateral_friction_force_max_factor(&self) -> &f32;
    fn allow_grip_slip_transition(&self) -> &bool;
    fn longitude_friction_scale(&self) -> &Vec<FrictionScaleAtVelocity>;
    fn lattitude_friction_scale(&self) -> &Vec<FrictionScaleAtVelocity>;
    fn wheel_friction_lattitude_brake_scale(&self) -> &f32;
    fn resistance(&self) -> &f32;
    fn fx_torque_radius_multiplier(&self) -> &f32;
    fn friction_moment_velocity_min(&self) -> &f32;
    fn friction_moment_velocity_max(&self) -> &f32;
    fn friction_moment_multiplier(&self) -> &f32;
    fn friction_moment_max_factor(&self) -> &f32;
    fn brake_factor(&self) -> &f32;
    fn brake_force(&self) -> &f32;
    fn auto_hand_brake_if_no_throttle_and_steer(&self) -> &bool;
    fn hand_brake_factor(&self) -> &f32;
    fn hand_brake_force(&self) -> &f32;
    fn total_friction_scale(&self) -> &f32;
    fn total_lateral_friction_scale(&self) -> &f32;
    fn lateral_pos_k(&self) -> &f32;
    fn lateral_neg_k(&self) -> &f32;
    fn longitudinal_pos_k(&self) -> &f32;
    fn longitudinal_neg_k(&self) -> &f32;
    fn align_mom_scale(&self) -> &f32;
    fn wheel_base_lateral(&self) -> &f32;
    fn wheel_base_longitudinal(&self) -> &f32;
    fn driving_type(&self) -> &i32;
    fn steering_type(&self) -> &i32;
    fn friction_method(&self) -> &i32;
    fn rotation_direction_index(&self) -> &i32;
    fn steering_angle_index(&self) -> &i32;
    fn pacejka_config_index(&self) -> &i32;
    fn engine_index(&self) -> &i32;
    fn ackerman_device_type(&self) -> &i32;
    fn use_rolling_resistance_velocity_factor(&self) -> &bool;
    fn use_rolling_resistance_base_factor(&self) -> &bool;
    fn use_engine_brake(&self) -> &bool;
    fn is_allowed_to_spin(&self) -> &bool;
    fn has_steering_inverted(&self) -> &bool;
    fn use_friction_moment(&self) -> &bool;
    fn use_low_speed_auto_brake(&self) -> &bool;
    fn adjust_wheel_rotation(&self) -> &bool;
    fn collision_material_pair(&self) -> &super::entity::MaterialDecl;
}

impl WheelConfigDataTrait for WheelConfigData {
    fn offset(&self) -> &super::core::Vec3 {
        &self.offset
    }
    fn inertia(&self) -> &super::core::Vec3 {
        &self.inertia
    }
    fn rotation_body(&self) -> &Option<Arc<Mutex<dyn RotationBodyDataTrait>>> {
        &self.rotation_body
    }
    fn sphere_collision(&self) -> &SphereCollisionData {
        &self.sphere_collision
    }
    fn spring(&self) -> &SpringData {
        &self.spring
    }
    fn mass(&self) -> &f32 {
        &self.mass
    }
    fn radius(&self) -> &f32 {
        &self.radius
    }
    fn width(&self) -> &f32 {
        &self.width
    }
    fn rolling_resistance_internal_base_factor(&self) -> &f32 {
        &self.rolling_resistance_internal_base_factor
    }
    fn rolling_resistance_base_factor(&self) -> &f32 {
        &self.rolling_resistance_base_factor
    }
    fn rolling_resistance_velocity_factor(&self) -> &f32 {
        &self.rolling_resistance_velocity_factor
    }
    fn engine_brake_velocity_factor(&self) -> &f32 {
        &self.engine_brake_velocity_factor
    }
    fn engine_brake_min_factor(&self) -> &f32 {
        &self.engine_brake_min_factor
    }
    fn engine_brake_max_factor(&self) -> &f32 {
        &self.engine_brake_max_factor
    }
    fn transmission_loss(&self) -> &Vec<super::core::Vec2> {
        &self.transmission_loss
    }
    fn diff_gear_ratio(&self) -> &f32 {
        &self.diff_gear_ratio
    }
    fn low_speed_steering_sensitivity(&self) -> &f32 {
        &self.low_speed_steering_sensitivity
    }
    fn mid_speed_steering_sensitivity(&self) -> &f32 {
        &self.mid_speed_steering_sensitivity
    }
    fn high_speed_steering_sensitivity(&self) -> &f32 {
        &self.high_speed_steering_sensitivity
    }
    fn sensitivity_range_scale(&self) -> &f32 {
        &self.sensitivity_range_scale
    }
    fn low_speed_steering_sensitivity_limit(&self) -> &f32 {
        &self.low_speed_steering_sensitivity_limit
    }
    fn mid_speed_steering_sensitivity_limit(&self) -> &f32 {
        &self.mid_speed_steering_sensitivity_limit
    }
    fn high_speed_steering_sensitivity_limit(&self) -> &f32 {
        &self.high_speed_steering_sensitivity_limit
    }
    fn tracked_turn_speed_limit(&self) -> &f32 {
        &self.tracked_turn_speed_limit
    }
    fn tracked_forward_speed_limit(&self) -> &f32 {
        &self.tracked_forward_speed_limit
    }
    fn tracked_slip_steer_reduction_scale(&self) -> &f32 {
        &self.tracked_slip_steer_reduction_scale
    }
    fn steer_inertia(&self) -> &Option<Arc<Mutex<dyn Curve2DTrait>>> {
        &self.steer_inertia
    }
    fn steering_sensitivity(&self) -> &Vec<SensitivityAtVelocity> {
        &self.steering_sensitivity
    }
    fn tracked_steering_boost_on_opposite_torque(&self) -> &f32 {
        &self.tracked_steering_boost_on_opposite_torque
    }
    fn slope_grip_min_angle(&self) -> &f32 {
        &self.slope_grip_min_angle
    }
    fn slope_grip_max_angle(&self) -> &f32 {
        &self.slope_grip_max_angle
    }
    fn slope_grip_exponent(&self) -> &i32 {
        &self.slope_grip_exponent
    }
    fn off_ground_gravity_modifier(&self) -> &f32 {
        &self.off_ground_gravity_modifier
    }
    fn side_slip_angle_max_slip_condition(&self) -> &f32 {
        &self.side_slip_angle_max_slip_condition
    }
    fn angular_velocity_min_slip_condition(&self) -> &f32 {
        &self.angular_velocity_min_slip_condition
    }
    fn wheel_velocity_x_min_slip_condition(&self) -> &f32 {
        &self.wheel_velocity_x_min_slip_condition
    }
    fn wheel_slip_ratio_max_slip_condition(&self) -> &f32 {
        &self.wheel_slip_ratio_max_slip_condition
    }
    fn longitudinal_friction_force_max_factor(&self) -> &f32 {
        &self.longitudinal_friction_force_max_factor
    }
    fn lateral_friction_force_max_factor(&self) -> &f32 {
        &self.lateral_friction_force_max_factor
    }
    fn allow_grip_slip_transition(&self) -> &bool {
        &self.allow_grip_slip_transition
    }
    fn longitude_friction_scale(&self) -> &Vec<FrictionScaleAtVelocity> {
        &self.longitude_friction_scale
    }
    fn lattitude_friction_scale(&self) -> &Vec<FrictionScaleAtVelocity> {
        &self.lattitude_friction_scale
    }
    fn wheel_friction_lattitude_brake_scale(&self) -> &f32 {
        &self.wheel_friction_lattitude_brake_scale
    }
    fn resistance(&self) -> &f32 {
        &self.resistance
    }
    fn fx_torque_radius_multiplier(&self) -> &f32 {
        &self.fx_torque_radius_multiplier
    }
    fn friction_moment_velocity_min(&self) -> &f32 {
        &self.friction_moment_velocity_min
    }
    fn friction_moment_velocity_max(&self) -> &f32 {
        &self.friction_moment_velocity_max
    }
    fn friction_moment_multiplier(&self) -> &f32 {
        &self.friction_moment_multiplier
    }
    fn friction_moment_max_factor(&self) -> &f32 {
        &self.friction_moment_max_factor
    }
    fn brake_factor(&self) -> &f32 {
        &self.brake_factor
    }
    fn brake_force(&self) -> &f32 {
        &self.brake_force
    }
    fn auto_hand_brake_if_no_throttle_and_steer(&self) -> &bool {
        &self.auto_hand_brake_if_no_throttle_and_steer
    }
    fn hand_brake_factor(&self) -> &f32 {
        &self.hand_brake_factor
    }
    fn hand_brake_force(&self) -> &f32 {
        &self.hand_brake_force
    }
    fn total_friction_scale(&self) -> &f32 {
        &self.total_friction_scale
    }
    fn total_lateral_friction_scale(&self) -> &f32 {
        &self.total_lateral_friction_scale
    }
    fn lateral_pos_k(&self) -> &f32 {
        &self.lateral_pos_k
    }
    fn lateral_neg_k(&self) -> &f32 {
        &self.lateral_neg_k
    }
    fn longitudinal_pos_k(&self) -> &f32 {
        &self.longitudinal_pos_k
    }
    fn longitudinal_neg_k(&self) -> &f32 {
        &self.longitudinal_neg_k
    }
    fn align_mom_scale(&self) -> &f32 {
        &self.align_mom_scale
    }
    fn wheel_base_lateral(&self) -> &f32 {
        &self.wheel_base_lateral
    }
    fn wheel_base_longitudinal(&self) -> &f32 {
        &self.wheel_base_longitudinal
    }
    fn driving_type(&self) -> &i32 {
        &self.driving_type
    }
    fn steering_type(&self) -> &i32 {
        &self.steering_type
    }
    fn friction_method(&self) -> &i32 {
        &self.friction_method
    }
    fn rotation_direction_index(&self) -> &i32 {
        &self.rotation_direction_index
    }
    fn steering_angle_index(&self) -> &i32 {
        &self.steering_angle_index
    }
    fn pacejka_config_index(&self) -> &i32 {
        &self.pacejka_config_index
    }
    fn engine_index(&self) -> &i32 {
        &self.engine_index
    }
    fn ackerman_device_type(&self) -> &i32 {
        &self.ackerman_device_type
    }
    fn use_rolling_resistance_velocity_factor(&self) -> &bool {
        &self.use_rolling_resistance_velocity_factor
    }
    fn use_rolling_resistance_base_factor(&self) -> &bool {
        &self.use_rolling_resistance_base_factor
    }
    fn use_engine_brake(&self) -> &bool {
        &self.use_engine_brake
    }
    fn is_allowed_to_spin(&self) -> &bool {
        &self.is_allowed_to_spin
    }
    fn has_steering_inverted(&self) -> &bool {
        &self.has_steering_inverted
    }
    fn use_friction_moment(&self) -> &bool {
        &self.use_friction_moment
    }
    fn use_low_speed_auto_brake(&self) -> &bool {
        &self.use_low_speed_auto_brake
    }
    fn adjust_wheel_rotation(&self) -> &bool {
        &self.adjust_wheel_rotation
    }
    fn collision_material_pair(&self) -> &super::entity::MaterialDecl {
        &self.collision_material_pair
    }
}

impl super::core::DataContainerTrait for WheelConfigData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static WHEELCONFIGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WheelConfigData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WheelConfigData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Offset",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(WheelConfigData, offset),
            },
            FieldInfoData {
                name: "Inertia",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(WheelConfigData, inertia),
            },
            FieldInfoData {
                name: "RotationBody",
                flags: MemberInfoFlags::new(0),
                field_type: "RotationBodyData",
                rust_offset: offset_of!(WheelConfigData, rotation_body),
            },
            FieldInfoData {
                name: "SphereCollision",
                flags: MemberInfoFlags::new(0),
                field_type: "SphereCollisionData",
                rust_offset: offset_of!(WheelConfigData, sphere_collision),
            },
            FieldInfoData {
                name: "Spring",
                flags: MemberInfoFlags::new(0),
                field_type: "SpringData",
                rust_offset: offset_of!(WheelConfigData, spring),
            },
            FieldInfoData {
                name: "Mass",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, mass),
            },
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, radius),
            },
            FieldInfoData {
                name: "Width",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, width),
            },
            FieldInfoData {
                name: "RollingResistanceInternalBaseFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, rolling_resistance_internal_base_factor),
            },
            FieldInfoData {
                name: "RollingResistanceBaseFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, rolling_resistance_base_factor),
            },
            FieldInfoData {
                name: "RollingResistanceVelocityFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, rolling_resistance_velocity_factor),
            },
            FieldInfoData {
                name: "EngineBrakeVelocityFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, engine_brake_velocity_factor),
            },
            FieldInfoData {
                name: "EngineBrakeMinFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, engine_brake_min_factor),
            },
            FieldInfoData {
                name: "EngineBrakeMaxFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, engine_brake_max_factor),
            },
            FieldInfoData {
                name: "TransmissionLoss",
                flags: MemberInfoFlags::new(144),
                field_type: "Vec2-Array",
                rust_offset: offset_of!(WheelConfigData, transmission_loss),
            },
            FieldInfoData {
                name: "DiffGearRatio",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, diff_gear_ratio),
            },
            FieldInfoData {
                name: "LowSpeedSteeringSensitivity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, low_speed_steering_sensitivity),
            },
            FieldInfoData {
                name: "MidSpeedSteeringSensitivity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, mid_speed_steering_sensitivity),
            },
            FieldInfoData {
                name: "HighSpeedSteeringSensitivity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, high_speed_steering_sensitivity),
            },
            FieldInfoData {
                name: "SensitivityRangeScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, sensitivity_range_scale),
            },
            FieldInfoData {
                name: "LowSpeedSteeringSensitivityLimit",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, low_speed_steering_sensitivity_limit),
            },
            FieldInfoData {
                name: "MidSpeedSteeringSensitivityLimit",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, mid_speed_steering_sensitivity_limit),
            },
            FieldInfoData {
                name: "HighSpeedSteeringSensitivityLimit",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, high_speed_steering_sensitivity_limit),
            },
            FieldInfoData {
                name: "TrackedTurnSpeedLimit",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, tracked_turn_speed_limit),
            },
            FieldInfoData {
                name: "TrackedForwardSpeedLimit",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, tracked_forward_speed_limit),
            },
            FieldInfoData {
                name: "TrackedSlipSteerReductionScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, tracked_slip_steer_reduction_scale),
            },
            FieldInfoData {
                name: "SteerInertia",
                flags: MemberInfoFlags::new(0),
                field_type: "Curve2D",
                rust_offset: offset_of!(WheelConfigData, steer_inertia),
            },
            FieldInfoData {
                name: "SteeringSensitivity",
                flags: MemberInfoFlags::new(144),
                field_type: "SensitivityAtVelocity-Array",
                rust_offset: offset_of!(WheelConfigData, steering_sensitivity),
            },
            FieldInfoData {
                name: "TrackedSteeringBoostOnOppositeTorque",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, tracked_steering_boost_on_opposite_torque),
            },
            FieldInfoData {
                name: "SlopeGripMinAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, slope_grip_min_angle),
            },
            FieldInfoData {
                name: "SlopeGripMaxAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, slope_grip_max_angle),
            },
            FieldInfoData {
                name: "SlopeGripExponent",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(WheelConfigData, slope_grip_exponent),
            },
            FieldInfoData {
                name: "OffGroundGravityModifier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, off_ground_gravity_modifier),
            },
            FieldInfoData {
                name: "SideSlipAngleMaxSlipCondition",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, side_slip_angle_max_slip_condition),
            },
            FieldInfoData {
                name: "AngularVelocityMinSlipCondition",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, angular_velocity_min_slip_condition),
            },
            FieldInfoData {
                name: "WheelVelocityXMinSlipCondition",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, wheel_velocity_x_min_slip_condition),
            },
            FieldInfoData {
                name: "WheelSlipRatioMaxSlipCondition",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, wheel_slip_ratio_max_slip_condition),
            },
            FieldInfoData {
                name: "LongitudinalFrictionForceMaxFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, longitudinal_friction_force_max_factor),
            },
            FieldInfoData {
                name: "LateralFrictionForceMaxFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, lateral_friction_force_max_factor),
            },
            FieldInfoData {
                name: "AllowGripSlipTransition",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WheelConfigData, allow_grip_slip_transition),
            },
            FieldInfoData {
                name: "LongitudeFrictionScale",
                flags: MemberInfoFlags::new(144),
                field_type: "FrictionScaleAtVelocity-Array",
                rust_offset: offset_of!(WheelConfigData, longitude_friction_scale),
            },
            FieldInfoData {
                name: "LattitudeFrictionScale",
                flags: MemberInfoFlags::new(144),
                field_type: "FrictionScaleAtVelocity-Array",
                rust_offset: offset_of!(WheelConfigData, lattitude_friction_scale),
            },
            FieldInfoData {
                name: "WheelFrictionLattitudeBrakeScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, wheel_friction_lattitude_brake_scale),
            },
            FieldInfoData {
                name: "Resistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, resistance),
            },
            FieldInfoData {
                name: "FxTorqueRadiusMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, fx_torque_radius_multiplier),
            },
            FieldInfoData {
                name: "FrictionMomentVelocityMin",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, friction_moment_velocity_min),
            },
            FieldInfoData {
                name: "FrictionMomentVelocityMax",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, friction_moment_velocity_max),
            },
            FieldInfoData {
                name: "FrictionMomentMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, friction_moment_multiplier),
            },
            FieldInfoData {
                name: "FrictionMomentMaxFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, friction_moment_max_factor),
            },
            FieldInfoData {
                name: "BrakeFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, brake_factor),
            },
            FieldInfoData {
                name: "BrakeForce",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, brake_force),
            },
            FieldInfoData {
                name: "AutoHandBrakeIfNoThrottleAndSteer",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WheelConfigData, auto_hand_brake_if_no_throttle_and_steer),
            },
            FieldInfoData {
                name: "HandBrakeFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, hand_brake_factor),
            },
            FieldInfoData {
                name: "HandBrakeForce",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, hand_brake_force),
            },
            FieldInfoData {
                name: "TotalFrictionScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, total_friction_scale),
            },
            FieldInfoData {
                name: "TotalLateralFrictionScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, total_lateral_friction_scale),
            },
            FieldInfoData {
                name: "LateralPosK",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, lateral_pos_k),
            },
            FieldInfoData {
                name: "LateralNegK",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, lateral_neg_k),
            },
            FieldInfoData {
                name: "LongitudinalPosK",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, longitudinal_pos_k),
            },
            FieldInfoData {
                name: "LongitudinalNegK",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, longitudinal_neg_k),
            },
            FieldInfoData {
                name: "AlignMomScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, align_mom_scale),
            },
            FieldInfoData {
                name: "WheelBaseLateral",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, wheel_base_lateral),
            },
            FieldInfoData {
                name: "WheelBaseLongitudinal",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WheelConfigData, wheel_base_longitudinal),
            },
            FieldInfoData {
                name: "DrivingType",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(WheelConfigData, driving_type),
            },
            FieldInfoData {
                name: "SteeringType",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(WheelConfigData, steering_type),
            },
            FieldInfoData {
                name: "FrictionMethod",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(WheelConfigData, friction_method),
            },
            FieldInfoData {
                name: "RotationDirectionIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(WheelConfigData, rotation_direction_index),
            },
            FieldInfoData {
                name: "SteeringAngleIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(WheelConfigData, steering_angle_index),
            },
            FieldInfoData {
                name: "PacejkaConfigIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(WheelConfigData, pacejka_config_index),
            },
            FieldInfoData {
                name: "EngineIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(WheelConfigData, engine_index),
            },
            FieldInfoData {
                name: "AckermanDeviceType",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(WheelConfigData, ackerman_device_type),
            },
            FieldInfoData {
                name: "UseRollingResistanceVelocityFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WheelConfigData, use_rolling_resistance_velocity_factor),
            },
            FieldInfoData {
                name: "UseRollingResistanceBaseFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WheelConfigData, use_rolling_resistance_base_factor),
            },
            FieldInfoData {
                name: "UseEngineBrake",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WheelConfigData, use_engine_brake),
            },
            FieldInfoData {
                name: "IsAllowedToSpin",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WheelConfigData, is_allowed_to_spin),
            },
            FieldInfoData {
                name: "HasSteeringInverted",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WheelConfigData, has_steering_inverted),
            },
            FieldInfoData {
                name: "UseFrictionMoment",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WheelConfigData, use_friction_moment),
            },
            FieldInfoData {
                name: "UseLowSpeedAutoBrake",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WheelConfigData, use_low_speed_auto_brake),
            },
            FieldInfoData {
                name: "AdjustWheelRotation",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WheelConfigData, adjust_wheel_rotation),
            },
            FieldInfoData {
                name: "CollisionMaterialPair",
                flags: MemberInfoFlags::new(0),
                field_type: "MaterialDecl",
                rust_offset: offset_of!(WheelConfigData, collision_material_pair),
            },
        ],
    }),
    array_type: Some(WHEELCONFIGDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WheelConfigData {
    fn type_info(&self) -> &'static TypeInfo {
        WHEELCONFIGDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WHEELCONFIGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WheelConfigData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("WheelConfigData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FrictionScaleAtVelocity {
    pub friction_scale: f32,
    pub velocity: f32,
}

pub trait FrictionScaleAtVelocityTrait: TypeObject {
    fn friction_scale(&self) -> &f32;
    fn velocity(&self) -> &f32;
}

impl FrictionScaleAtVelocityTrait for FrictionScaleAtVelocity {
    fn friction_scale(&self) -> &f32 {
        &self.friction_scale
    }
    fn velocity(&self) -> &f32 {
        &self.velocity
    }
}

pub static FRICTIONSCALEATVELOCITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FrictionScaleAtVelocity",
    flags: MemberInfoFlags::new(36937),
    module: "Physics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FrictionScaleAtVelocity as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "FrictionScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FrictionScaleAtVelocity, friction_scale),
            },
            FieldInfoData {
                name: "Velocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FrictionScaleAtVelocity, velocity),
            },
        ],
    }),
    array_type: Some(FRICTIONSCALEATVELOCITY_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for FrictionScaleAtVelocity {
    fn type_info(&self) -> &'static TypeInfo {
        FRICTIONSCALEATVELOCITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FRICTIONSCALEATVELOCITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FrictionScaleAtVelocity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("FrictionScaleAtVelocity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SensitivityAtVelocity {
    pub steering_sensitivity: f32,
    pub velocity: f32,
}

pub trait SensitivityAtVelocityTrait: TypeObject {
    fn steering_sensitivity(&self) -> &f32;
    fn velocity(&self) -> &f32;
}

impl SensitivityAtVelocityTrait for SensitivityAtVelocity {
    fn steering_sensitivity(&self) -> &f32 {
        &self.steering_sensitivity
    }
    fn velocity(&self) -> &f32 {
        &self.velocity
    }
}

pub static SENSITIVITYATVELOCITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SensitivityAtVelocity",
    flags: MemberInfoFlags::new(36937),
    module: "Physics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SensitivityAtVelocity as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "SteeringSensitivity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SensitivityAtVelocity, steering_sensitivity),
            },
            FieldInfoData {
                name: "Velocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SensitivityAtVelocity, velocity),
            },
        ],
    }),
    array_type: Some(SENSITIVITYATVELOCITY_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for SensitivityAtVelocity {
    fn type_info(&self) -> &'static TypeInfo {
        SENSITIVITYATVELOCITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SENSITIVITYATVELOCITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SensitivityAtVelocity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("SensitivityAtVelocity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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

pub trait SpringDataTrait: TypeObject {
    fn length(&self) -> &f32;
    fn stiffness(&self) -> &f32;
    fn damping(&self) -> &f32;
    fn progressive_start_ratio(&self) -> &f32;
    fn progressive_exponent(&self) -> &f32;
    fn visual_clip_offset(&self) -> &f32;
    fn attach_offset_y(&self) -> &f32;
    fn disabled_strengh_modifier(&self) -> &f32;
}

impl SpringDataTrait for SpringData {
    fn length(&self) -> &f32 {
        &self.length
    }
    fn stiffness(&self) -> &f32 {
        &self.stiffness
    }
    fn damping(&self) -> &f32 {
        &self.damping
    }
    fn progressive_start_ratio(&self) -> &f32 {
        &self.progressive_start_ratio
    }
    fn progressive_exponent(&self) -> &f32 {
        &self.progressive_exponent
    }
    fn visual_clip_offset(&self) -> &f32 {
        &self.visual_clip_offset
    }
    fn attach_offset_y(&self) -> &f32 {
        &self.attach_offset_y
    }
    fn disabled_strengh_modifier(&self) -> &f32 {
        &self.disabled_strengh_modifier
    }
}

pub static SPRINGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpringData",
    flags: MemberInfoFlags::new(36937),
    module: "Physics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SpringData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Length",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SpringData, length),
            },
            FieldInfoData {
                name: "Stiffness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SpringData, stiffness),
            },
            FieldInfoData {
                name: "Damping",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SpringData, damping),
            },
            FieldInfoData {
                name: "ProgressiveStartRatio",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SpringData, progressive_start_ratio),
            },
            FieldInfoData {
                name: "ProgressiveExponent",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SpringData, progressive_exponent),
            },
            FieldInfoData {
                name: "VisualClipOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SpringData, visual_clip_offset),
            },
            FieldInfoData {
                name: "AttachOffsetY",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SpringData, attach_offset_y),
            },
            FieldInfoData {
                name: "DisabledStrenghModifier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SpringData, disabled_strengh_modifier),
            },
        ],
    }),
    array_type: Some(SPRINGDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for SpringData {
    fn type_info(&self) -> &'static TypeInfo {
        SPRINGDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SPRINGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpringData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("SpringData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SphereCollisionData {
    pub counter_normal_brake_force_mod: f32,
}

pub trait SphereCollisionDataTrait: TypeObject {
    fn counter_normal_brake_force_mod(&self) -> &f32;
}

impl SphereCollisionDataTrait for SphereCollisionData {
    fn counter_normal_brake_force_mod(&self) -> &f32 {
        &self.counter_normal_brake_force_mod
    }
}

pub static SPHERECOLLISIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SphereCollisionData",
    flags: MemberInfoFlags::new(36937),
    module: "Physics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SphereCollisionData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "CounterNormalBrakeForceMod",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SphereCollisionData, counter_normal_brake_force_mod),
            },
        ],
    }),
    array_type: Some(SPHERECOLLISIONDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for SphereCollisionData {
    fn type_info(&self) -> &'static TypeInfo {
        SPHERECOLLISIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SPHERECOLLISIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SphereCollisionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("SphereCollisionData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct JetEngineConfigData {
    pub _glacier_base: EngineConfigData,
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

pub trait JetEngineConfigDataTrait: EngineConfigDataTrait {
    fn direction_vector_index(&self) -> &u32;
    fn force_magnitude_multiplier(&self) -> &f32;
    fn angle_input_y_multiplier(&self) -> &f32;
    fn angle_input_pitch_multiplier(&self) -> &f32;
    fn pitch_force_modifier(&self) -> &f32;
    fn max_velocity(&self) -> &f32;
    fn is_turnable(&self) -> &bool;
    fn is_water_jet_engine(&self) -> &bool;
    fn use_force_position_water_test(&self) -> &bool;
    fn use_hull_in_water_test(&self) -> &bool;
    fn steering_sensitivity(&self) -> &Vec<SensitivityAtVelocity>;
    fn power_fade_out_range(&self) -> &super::core::Vec2;
}

impl JetEngineConfigDataTrait for JetEngineConfigData {
    fn direction_vector_index(&self) -> &u32 {
        &self.direction_vector_index
    }
    fn force_magnitude_multiplier(&self) -> &f32 {
        &self.force_magnitude_multiplier
    }
    fn angle_input_y_multiplier(&self) -> &f32 {
        &self.angle_input_y_multiplier
    }
    fn angle_input_pitch_multiplier(&self) -> &f32 {
        &self.angle_input_pitch_multiplier
    }
    fn pitch_force_modifier(&self) -> &f32 {
        &self.pitch_force_modifier
    }
    fn max_velocity(&self) -> &f32 {
        &self.max_velocity
    }
    fn is_turnable(&self) -> &bool {
        &self.is_turnable
    }
    fn is_water_jet_engine(&self) -> &bool {
        &self.is_water_jet_engine
    }
    fn use_force_position_water_test(&self) -> &bool {
        &self.use_force_position_water_test
    }
    fn use_hull_in_water_test(&self) -> &bool {
        &self.use_hull_in_water_test
    }
    fn steering_sensitivity(&self) -> &Vec<SensitivityAtVelocity> {
        &self.steering_sensitivity
    }
    fn power_fade_out_range(&self) -> &super::core::Vec2 {
        &self.power_fade_out_range
    }
}

impl EngineConfigDataTrait for JetEngineConfigData {
    fn position(&self) -> &super::core::Vec3 {
        self._glacier_base.position()
    }
    fn rpm_curve_points(&self) -> &Vec<f32> {
        self._glacier_base.rpm_curve_points()
    }
    fn torque_curve_points(&self) -> &Vec<f32> {
        self._glacier_base.torque_curve_points()
    }
    fn rpm_min(&self) -> &f32 {
        self._glacier_base.rpm_min()
    }
    fn rpm_max(&self) -> &f32 {
        self._glacier_base.rpm_max()
    }
    fn rpm_cut(&self) -> &f32 {
        self._glacier_base.rpm_cut()
    }
    fn engine_power_multiplier(&self) -> &f32 {
        self._glacier_base.engine_power_multiplier()
    }
    fn internal_acceleration_factor(&self) -> &f32 {
        self._glacier_base.internal_acceleration_factor()
    }
    fn internal_deacceleration_factor(&self) -> &f32 {
        self._glacier_base.internal_deacceleration_factor()
    }
    fn max_reverse_speed(&self) -> &f32 {
        self._glacier_base.max_reverse_speed()
    }
    fn boost(&self) -> &Boost {
        self._glacier_base.boost()
    }
    fn max_vehicle_height_offset(&self) -> &f32 {
        self._glacier_base.max_vehicle_height_offset()
    }
}

impl super::core::DataContainerTrait for JetEngineConfigData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static JETENGINECONFIGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "JetEngineConfigData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENGINECONFIGDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<JetEngineConfigData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "DirectionVectorIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(JetEngineConfigData, direction_vector_index),
            },
            FieldInfoData {
                name: "ForceMagnitudeMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(JetEngineConfigData, force_magnitude_multiplier),
            },
            FieldInfoData {
                name: "AngleInputYMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(JetEngineConfigData, angle_input_y_multiplier),
            },
            FieldInfoData {
                name: "AngleInputPitchMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(JetEngineConfigData, angle_input_pitch_multiplier),
            },
            FieldInfoData {
                name: "PitchForceModifier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(JetEngineConfigData, pitch_force_modifier),
            },
            FieldInfoData {
                name: "MaxVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(JetEngineConfigData, max_velocity),
            },
            FieldInfoData {
                name: "IsTurnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(JetEngineConfigData, is_turnable),
            },
            FieldInfoData {
                name: "IsWaterJetEngine",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(JetEngineConfigData, is_water_jet_engine),
            },
            FieldInfoData {
                name: "UseForcePositionWaterTest",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(JetEngineConfigData, use_force_position_water_test),
            },
            FieldInfoData {
                name: "UseHullInWaterTest",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(JetEngineConfigData, use_hull_in_water_test),
            },
            FieldInfoData {
                name: "SteeringSensitivity",
                flags: MemberInfoFlags::new(144),
                field_type: "SensitivityAtVelocity-Array",
                rust_offset: offset_of!(JetEngineConfigData, steering_sensitivity),
            },
            FieldInfoData {
                name: "PowerFadeOutRange",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(JetEngineConfigData, power_fade_out_range),
            },
        ],
    }),
    array_type: Some(JETENGINECONFIGDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for JetEngineConfigData {
    fn type_info(&self) -> &'static TypeInfo {
        JETENGINECONFIGDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static JETENGINECONFIGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "JetEngineConfigData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("JetEngineConfigData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PropellerEngineConfigData {
    pub _glacier_base: EngineConfigData,
    pub horisontal_force_offset: super::core::Vec3,
    pub propeller_type: PropellerType,
    pub rotor_config: Option<Arc<Mutex<dyn RotorParametersTrait>>>,
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

pub trait PropellerEngineConfigDataTrait: EngineConfigDataTrait {
    fn horisontal_force_offset(&self) -> &super::core::Vec3;
    fn propeller_type(&self) -> &PropellerType;
    fn rotor_config(&self) -> &Option<Arc<Mutex<dyn RotorParametersTrait>>>;
    fn force_magnitude_input_type(&self) -> &ForceMagnitudeInputType;
    fn direction_vector_index(&self) -> &u32;
    fn apply_force_as_torque(&self) -> &bool;
    fn force_magnitude_multiplier(&self) -> &f32;
    fn lift_force_spring_constant(&self) -> &f32;
    fn lift_force_damping_constant(&self) -> &f32;
    fn cyclic_input_scale_roll(&self) -> &f32;
    fn cyclic_roll_lift_mod(&self) -> &f32;
    fn cyclic_roll_strafe_mod(&self) -> &f32;
    fn cyclic_input_scale_pitch(&self) -> &f32;
    fn cyclic_pitch_lift_mod(&self) -> &f32;
    fn cyclic_pitch_strafe_mod(&self) -> &f32;
    fn cyclic_pitch_strafe_brake_mod(&self) -> &f32;
    fn collective_input_idle(&self) -> &f32;
    fn collective_throttle_input_scale(&self) -> &f32;
    fn collective_brake_input_scale(&self) -> &f32;
    fn default_angle_of_attack(&self) -> &f32;
    fn attack_angle_mod(&self) -> &f32;
    fn stabilizer_mod(&self) -> &f32;
    fn horisontal_min_effect_velocity(&self) -> &f32;
    fn horisontal_min_effect_mod(&self) -> &f32;
    fn enable_new_helicopter(&self) -> &bool;
    fn s_p_allowed(&self) -> &bool;
    fn s_p_default(&self) -> &bool;
    fn s_p_forward_input(&self) -> &ForceMagnitudeInputType;
    fn s_p_sideways_input(&self) -> &ForceMagnitudeInputType;
    fn s_p_vertical_input(&self) -> &ForceMagnitudeInputType;
    fn s_p_pitch_input(&self) -> &ForceMagnitudeInputType;
    fn s_p_yaw_input(&self) -> &ForceMagnitudeInputType;
    fn s_p_forward_strength(&self) -> &f32;
    fn s_p_sideways_strength(&self) -> &f32;
    fn s_p_vertical_strength(&self) -> &f32;
    fn s_p_reverse_force_mod(&self) -> &f32;
    fn pitch_strength(&self) -> &f32;
    fn pitch_limit(&self) -> &f32;
    fn pitch_from_vel(&self) -> &f32;
    fn velocity_from_pitch(&self) -> &f32;
    fn roll_strength(&self) -> &f32;
    fn banking_strength(&self) -> &f32;
    fn banking_limit(&self) -> &f32;
    fn pitch_up_when_bank_strength(&self) -> &f32;
    fn pitch_up_when_bank_limit(&self) -> &f32;
    fn gravity_mod(&self) -> &f32;
}

impl PropellerEngineConfigDataTrait for PropellerEngineConfigData {
    fn horisontal_force_offset(&self) -> &super::core::Vec3 {
        &self.horisontal_force_offset
    }
    fn propeller_type(&self) -> &PropellerType {
        &self.propeller_type
    }
    fn rotor_config(&self) -> &Option<Arc<Mutex<dyn RotorParametersTrait>>> {
        &self.rotor_config
    }
    fn force_magnitude_input_type(&self) -> &ForceMagnitudeInputType {
        &self.force_magnitude_input_type
    }
    fn direction_vector_index(&self) -> &u32 {
        &self.direction_vector_index
    }
    fn apply_force_as_torque(&self) -> &bool {
        &self.apply_force_as_torque
    }
    fn force_magnitude_multiplier(&self) -> &f32 {
        &self.force_magnitude_multiplier
    }
    fn lift_force_spring_constant(&self) -> &f32 {
        &self.lift_force_spring_constant
    }
    fn lift_force_damping_constant(&self) -> &f32 {
        &self.lift_force_damping_constant
    }
    fn cyclic_input_scale_roll(&self) -> &f32 {
        &self.cyclic_input_scale_roll
    }
    fn cyclic_roll_lift_mod(&self) -> &f32 {
        &self.cyclic_roll_lift_mod
    }
    fn cyclic_roll_strafe_mod(&self) -> &f32 {
        &self.cyclic_roll_strafe_mod
    }
    fn cyclic_input_scale_pitch(&self) -> &f32 {
        &self.cyclic_input_scale_pitch
    }
    fn cyclic_pitch_lift_mod(&self) -> &f32 {
        &self.cyclic_pitch_lift_mod
    }
    fn cyclic_pitch_strafe_mod(&self) -> &f32 {
        &self.cyclic_pitch_strafe_mod
    }
    fn cyclic_pitch_strafe_brake_mod(&self) -> &f32 {
        &self.cyclic_pitch_strafe_brake_mod
    }
    fn collective_input_idle(&self) -> &f32 {
        &self.collective_input_idle
    }
    fn collective_throttle_input_scale(&self) -> &f32 {
        &self.collective_throttle_input_scale
    }
    fn collective_brake_input_scale(&self) -> &f32 {
        &self.collective_brake_input_scale
    }
    fn default_angle_of_attack(&self) -> &f32 {
        &self.default_angle_of_attack
    }
    fn attack_angle_mod(&self) -> &f32 {
        &self.attack_angle_mod
    }
    fn stabilizer_mod(&self) -> &f32 {
        &self.stabilizer_mod
    }
    fn horisontal_min_effect_velocity(&self) -> &f32 {
        &self.horisontal_min_effect_velocity
    }
    fn horisontal_min_effect_mod(&self) -> &f32 {
        &self.horisontal_min_effect_mod
    }
    fn enable_new_helicopter(&self) -> &bool {
        &self.enable_new_helicopter
    }
    fn s_p_allowed(&self) -> &bool {
        &self.s_p_allowed
    }
    fn s_p_default(&self) -> &bool {
        &self.s_p_default
    }
    fn s_p_forward_input(&self) -> &ForceMagnitudeInputType {
        &self.s_p_forward_input
    }
    fn s_p_sideways_input(&self) -> &ForceMagnitudeInputType {
        &self.s_p_sideways_input
    }
    fn s_p_vertical_input(&self) -> &ForceMagnitudeInputType {
        &self.s_p_vertical_input
    }
    fn s_p_pitch_input(&self) -> &ForceMagnitudeInputType {
        &self.s_p_pitch_input
    }
    fn s_p_yaw_input(&self) -> &ForceMagnitudeInputType {
        &self.s_p_yaw_input
    }
    fn s_p_forward_strength(&self) -> &f32 {
        &self.s_p_forward_strength
    }
    fn s_p_sideways_strength(&self) -> &f32 {
        &self.s_p_sideways_strength
    }
    fn s_p_vertical_strength(&self) -> &f32 {
        &self.s_p_vertical_strength
    }
    fn s_p_reverse_force_mod(&self) -> &f32 {
        &self.s_p_reverse_force_mod
    }
    fn pitch_strength(&self) -> &f32 {
        &self.pitch_strength
    }
    fn pitch_limit(&self) -> &f32 {
        &self.pitch_limit
    }
    fn pitch_from_vel(&self) -> &f32 {
        &self.pitch_from_vel
    }
    fn velocity_from_pitch(&self) -> &f32 {
        &self.velocity_from_pitch
    }
    fn roll_strength(&self) -> &f32 {
        &self.roll_strength
    }
    fn banking_strength(&self) -> &f32 {
        &self.banking_strength
    }
    fn banking_limit(&self) -> &f32 {
        &self.banking_limit
    }
    fn pitch_up_when_bank_strength(&self) -> &f32 {
        &self.pitch_up_when_bank_strength
    }
    fn pitch_up_when_bank_limit(&self) -> &f32 {
        &self.pitch_up_when_bank_limit
    }
    fn gravity_mod(&self) -> &f32 {
        &self.gravity_mod
    }
}

impl EngineConfigDataTrait for PropellerEngineConfigData {
    fn position(&self) -> &super::core::Vec3 {
        self._glacier_base.position()
    }
    fn rpm_curve_points(&self) -> &Vec<f32> {
        self._glacier_base.rpm_curve_points()
    }
    fn torque_curve_points(&self) -> &Vec<f32> {
        self._glacier_base.torque_curve_points()
    }
    fn rpm_min(&self) -> &f32 {
        self._glacier_base.rpm_min()
    }
    fn rpm_max(&self) -> &f32 {
        self._glacier_base.rpm_max()
    }
    fn rpm_cut(&self) -> &f32 {
        self._glacier_base.rpm_cut()
    }
    fn engine_power_multiplier(&self) -> &f32 {
        self._glacier_base.engine_power_multiplier()
    }
    fn internal_acceleration_factor(&self) -> &f32 {
        self._glacier_base.internal_acceleration_factor()
    }
    fn internal_deacceleration_factor(&self) -> &f32 {
        self._glacier_base.internal_deacceleration_factor()
    }
    fn max_reverse_speed(&self) -> &f32 {
        self._glacier_base.max_reverse_speed()
    }
    fn boost(&self) -> &Boost {
        self._glacier_base.boost()
    }
    fn max_vehicle_height_offset(&self) -> &f32 {
        self._glacier_base.max_vehicle_height_offset()
    }
}

impl super::core::DataContainerTrait for PropellerEngineConfigData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PROPELLERENGINECONFIGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropellerEngineConfigData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENGINECONFIGDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PropellerEngineConfigData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "HorisontalForceOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(PropellerEngineConfigData, horisontal_force_offset),
            },
            FieldInfoData {
                name: "PropellerType",
                flags: MemberInfoFlags::new(0),
                field_type: "PropellerType",
                rust_offset: offset_of!(PropellerEngineConfigData, propeller_type),
            },
            FieldInfoData {
                name: "RotorConfig",
                flags: MemberInfoFlags::new(0),
                field_type: "RotorParameters",
                rust_offset: offset_of!(PropellerEngineConfigData, rotor_config),
            },
            FieldInfoData {
                name: "ForceMagnitudeInputType",
                flags: MemberInfoFlags::new(0),
                field_type: "ForceMagnitudeInputType",
                rust_offset: offset_of!(PropellerEngineConfigData, force_magnitude_input_type),
            },
            FieldInfoData {
                name: "DirectionVectorIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(PropellerEngineConfigData, direction_vector_index),
            },
            FieldInfoData {
                name: "ApplyForceAsTorque",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PropellerEngineConfigData, apply_force_as_torque),
            },
            FieldInfoData {
                name: "ForceMagnitudeMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PropellerEngineConfigData, force_magnitude_multiplier),
            },
            FieldInfoData {
                name: "LiftForceSpringConstant",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PropellerEngineConfigData, lift_force_spring_constant),
            },
            FieldInfoData {
                name: "LiftForceDampingConstant",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PropellerEngineConfigData, lift_force_damping_constant),
            },
            FieldInfoData {
                name: "CyclicInputScaleRoll",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PropellerEngineConfigData, cyclic_input_scale_roll),
            },
            FieldInfoData {
                name: "CyclicRollLiftMod",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PropellerEngineConfigData, cyclic_roll_lift_mod),
            },
            FieldInfoData {
                name: "CyclicRollStrafeMod",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PropellerEngineConfigData, cyclic_roll_strafe_mod),
            },
            FieldInfoData {
                name: "CyclicInputScalePitch",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PropellerEngineConfigData, cyclic_input_scale_pitch),
            },
            FieldInfoData {
                name: "CyclicPitchLiftMod",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PropellerEngineConfigData, cyclic_pitch_lift_mod),
            },
            FieldInfoData {
                name: "CyclicPitchStrafeMod",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PropellerEngineConfigData, cyclic_pitch_strafe_mod),
            },
            FieldInfoData {
                name: "CyclicPitchStrafeBrakeMod",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PropellerEngineConfigData, cyclic_pitch_strafe_brake_mod),
            },
            FieldInfoData {
                name: "CollectiveInputIdle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PropellerEngineConfigData, collective_input_idle),
            },
            FieldInfoData {
                name: "CollectiveThrottleInputScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PropellerEngineConfigData, collective_throttle_input_scale),
            },
            FieldInfoData {
                name: "CollectiveBrakeInputScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PropellerEngineConfigData, collective_brake_input_scale),
            },
            FieldInfoData {
                name: "DefaultAngleOfAttack",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PropellerEngineConfigData, default_angle_of_attack),
            },
            FieldInfoData {
                name: "AttackAngleMod",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PropellerEngineConfigData, attack_angle_mod),
            },
            FieldInfoData {
                name: "StabilizerMod",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PropellerEngineConfigData, stabilizer_mod),
            },
            FieldInfoData {
                name: "HorisontalMinEffectVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PropellerEngineConfigData, horisontal_min_effect_velocity),
            },
            FieldInfoData {
                name: "HorisontalMinEffectMod",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PropellerEngineConfigData, horisontal_min_effect_mod),
            },
            FieldInfoData {
                name: "EnableNewHelicopter",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PropellerEngineConfigData, enable_new_helicopter),
            },
            FieldInfoData {
                name: "SPAllowed",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PropellerEngineConfigData, s_p_allowed),
            },
            FieldInfoData {
                name: "SPDefault",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PropellerEngineConfigData, s_p_default),
            },
            FieldInfoData {
                name: "SPForwardInput",
                flags: MemberInfoFlags::new(0),
                field_type: "ForceMagnitudeInputType",
                rust_offset: offset_of!(PropellerEngineConfigData, s_p_forward_input),
            },
            FieldInfoData {
                name: "SPSidewaysInput",
                flags: MemberInfoFlags::new(0),
                field_type: "ForceMagnitudeInputType",
                rust_offset: offset_of!(PropellerEngineConfigData, s_p_sideways_input),
            },
            FieldInfoData {
                name: "SPVerticalInput",
                flags: MemberInfoFlags::new(0),
                field_type: "ForceMagnitudeInputType",
                rust_offset: offset_of!(PropellerEngineConfigData, s_p_vertical_input),
            },
            FieldInfoData {
                name: "SPPitchInput",
                flags: MemberInfoFlags::new(0),
                field_type: "ForceMagnitudeInputType",
                rust_offset: offset_of!(PropellerEngineConfigData, s_p_pitch_input),
            },
            FieldInfoData {
                name: "SPYawInput",
                flags: MemberInfoFlags::new(0),
                field_type: "ForceMagnitudeInputType",
                rust_offset: offset_of!(PropellerEngineConfigData, s_p_yaw_input),
            },
            FieldInfoData {
                name: "SPForwardStrength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PropellerEngineConfigData, s_p_forward_strength),
            },
            FieldInfoData {
                name: "SPSidewaysStrength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PropellerEngineConfigData, s_p_sideways_strength),
            },
            FieldInfoData {
                name: "SPVerticalStrength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PropellerEngineConfigData, s_p_vertical_strength),
            },
            FieldInfoData {
                name: "SPReverseForceMod",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PropellerEngineConfigData, s_p_reverse_force_mod),
            },
            FieldInfoData {
                name: "PitchStrength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PropellerEngineConfigData, pitch_strength),
            },
            FieldInfoData {
                name: "PitchLimit",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PropellerEngineConfigData, pitch_limit),
            },
            FieldInfoData {
                name: "PitchFromVel",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PropellerEngineConfigData, pitch_from_vel),
            },
            FieldInfoData {
                name: "VelocityFromPitch",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PropellerEngineConfigData, velocity_from_pitch),
            },
            FieldInfoData {
                name: "RollStrength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PropellerEngineConfigData, roll_strength),
            },
            FieldInfoData {
                name: "BankingStrength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PropellerEngineConfigData, banking_strength),
            },
            FieldInfoData {
                name: "BankingLimit",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PropellerEngineConfigData, banking_limit),
            },
            FieldInfoData {
                name: "PitchUpWhenBankStrength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PropellerEngineConfigData, pitch_up_when_bank_strength),
            },
            FieldInfoData {
                name: "PitchUpWhenBankLimit",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PropellerEngineConfigData, pitch_up_when_bank_limit),
            },
            FieldInfoData {
                name: "GravityMod",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PropellerEngineConfigData, gravity_mod),
            },
        ],
    }),
    array_type: Some(PROPELLERENGINECONFIGDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PropellerEngineConfigData {
    fn type_info(&self) -> &'static TypeInfo {
        PROPELLERENGINECONFIGDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PROPELLERENGINECONFIGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropellerEngineConfigData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PropellerEngineConfigData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RotorParameters {
    pub _glacier_base: super::core::DataContainer,
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

pub trait RotorParametersTrait: super::core::DataContainerTrait {
    fn cyclic_input_scale_roll(&self) -> &f32;
    fn cyclic_input_scale_pitch(&self) -> &f32;
    fn collective_throttle_input_scale(&self) -> &f32;
    fn collective_brake_input_scale(&self) -> &f32;
    fn collective_input_idle(&self) -> &f32;
    fn horizontal_force_modifier(&self) -> &f32;
    fn angle_of_attack(&self) -> &Vec<super::core::Vec2>;
    fn cyclic_fade_out_offset(&self) -> &f32;
    fn additional_gravity_modifier(&self) -> &f32;
    fn reverse_throttle(&self) -> &bool;
    fn enable_horisontal_min_effect(&self) -> &bool;
    fn horisontal_min_effect_velocity(&self) -> &f32;
    fn horisontal_min_effect_mod(&self) -> &f32;
}

impl RotorParametersTrait for RotorParameters {
    fn cyclic_input_scale_roll(&self) -> &f32 {
        &self.cyclic_input_scale_roll
    }
    fn cyclic_input_scale_pitch(&self) -> &f32 {
        &self.cyclic_input_scale_pitch
    }
    fn collective_throttle_input_scale(&self) -> &f32 {
        &self.collective_throttle_input_scale
    }
    fn collective_brake_input_scale(&self) -> &f32 {
        &self.collective_brake_input_scale
    }
    fn collective_input_idle(&self) -> &f32 {
        &self.collective_input_idle
    }
    fn horizontal_force_modifier(&self) -> &f32 {
        &self.horizontal_force_modifier
    }
    fn angle_of_attack(&self) -> &Vec<super::core::Vec2> {
        &self.angle_of_attack
    }
    fn cyclic_fade_out_offset(&self) -> &f32 {
        &self.cyclic_fade_out_offset
    }
    fn additional_gravity_modifier(&self) -> &f32 {
        &self.additional_gravity_modifier
    }
    fn reverse_throttle(&self) -> &bool {
        &self.reverse_throttle
    }
    fn enable_horisontal_min_effect(&self) -> &bool {
        &self.enable_horisontal_min_effect
    }
    fn horisontal_min_effect_velocity(&self) -> &f32 {
        &self.horisontal_min_effect_velocity
    }
    fn horisontal_min_effect_mod(&self) -> &f32 {
        &self.horisontal_min_effect_mod
    }
}

impl super::core::DataContainerTrait for RotorParameters {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static ROTORPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RotorParameters",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RotorParameters as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "CyclicInputScaleRoll",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RotorParameters, cyclic_input_scale_roll),
            },
            FieldInfoData {
                name: "CyclicInputScalePitch",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RotorParameters, cyclic_input_scale_pitch),
            },
            FieldInfoData {
                name: "CollectiveThrottleInputScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RotorParameters, collective_throttle_input_scale),
            },
            FieldInfoData {
                name: "CollectiveBrakeInputScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RotorParameters, collective_brake_input_scale),
            },
            FieldInfoData {
                name: "CollectiveInputIdle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RotorParameters, collective_input_idle),
            },
            FieldInfoData {
                name: "HorizontalForceModifier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RotorParameters, horizontal_force_modifier),
            },
            FieldInfoData {
                name: "AngleOfAttack",
                flags: MemberInfoFlags::new(144),
                field_type: "Vec2-Array",
                rust_offset: offset_of!(RotorParameters, angle_of_attack),
            },
            FieldInfoData {
                name: "CyclicFadeOutOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RotorParameters, cyclic_fade_out_offset),
            },
            FieldInfoData {
                name: "AdditionalGravityModifier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RotorParameters, additional_gravity_modifier),
            },
            FieldInfoData {
                name: "ReverseThrottle",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RotorParameters, reverse_throttle),
            },
            FieldInfoData {
                name: "EnableHorisontalMinEffect",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RotorParameters, enable_horisontal_min_effect),
            },
            FieldInfoData {
                name: "HorisontalMinEffectVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RotorParameters, horisontal_min_effect_velocity),
            },
            FieldInfoData {
                name: "HorisontalMinEffectMod",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RotorParameters, horisontal_min_effect_mod),
            },
        ],
    }),
    array_type: Some(ROTORPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RotorParameters {
    fn type_info(&self) -> &'static TypeInfo {
        ROTORPARAMETERS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ROTORPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RotorParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("RotorParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ForceMagnitudeInputType {
    #[default]
    FMITYaw = 0,
    FMITPitch = 1,
    FMITRoll = 2,
    FMITThrottle = 3,
    FMITStrafe = 4,
}

pub static FORCEMAGNITUDEINPUTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForceMagnitudeInputType",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(FORCEMAGNITUDEINPUTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ForceMagnitudeInputType {
    fn type_info(&self) -> &'static TypeInfo {
        FORCEMAGNITUDEINPUTTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FORCEMAGNITUDEINPUTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForceMagnitudeInputType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ForceMagnitudeInputType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum PropellerType {
    #[default]
    PropellerType_Regular = 0,
    PropellerType_Rotor = 1,
}

pub static PROPELLERTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropellerType",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(PROPELLERTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PropellerType {
    fn type_info(&self) -> &'static TypeInfo {
        PROPELLERTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PROPELLERTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropellerType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PropellerType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CombustionEngineConfigData {
    pub _glacier_base: EngineConfigData,
}

pub trait CombustionEngineConfigDataTrait: EngineConfigDataTrait {
}

impl CombustionEngineConfigDataTrait for CombustionEngineConfigData {
}

impl EngineConfigDataTrait for CombustionEngineConfigData {
    fn position(&self) -> &super::core::Vec3 {
        self._glacier_base.position()
    }
    fn rpm_curve_points(&self) -> &Vec<f32> {
        self._glacier_base.rpm_curve_points()
    }
    fn torque_curve_points(&self) -> &Vec<f32> {
        self._glacier_base.torque_curve_points()
    }
    fn rpm_min(&self) -> &f32 {
        self._glacier_base.rpm_min()
    }
    fn rpm_max(&self) -> &f32 {
        self._glacier_base.rpm_max()
    }
    fn rpm_cut(&self) -> &f32 {
        self._glacier_base.rpm_cut()
    }
    fn engine_power_multiplier(&self) -> &f32 {
        self._glacier_base.engine_power_multiplier()
    }
    fn internal_acceleration_factor(&self) -> &f32 {
        self._glacier_base.internal_acceleration_factor()
    }
    fn internal_deacceleration_factor(&self) -> &f32 {
        self._glacier_base.internal_deacceleration_factor()
    }
    fn max_reverse_speed(&self) -> &f32 {
        self._glacier_base.max_reverse_speed()
    }
    fn boost(&self) -> &Boost {
        self._glacier_base.boost()
    }
    fn max_vehicle_height_offset(&self) -> &f32 {
        self._glacier_base.max_vehicle_height_offset()
    }
}

impl super::core::DataContainerTrait for CombustionEngineConfigData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static COMBUSTIONENGINECONFIGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CombustionEngineConfigData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENGINECONFIGDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CombustionEngineConfigData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(COMBUSTIONENGINECONFIGDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CombustionEngineConfigData {
    fn type_info(&self) -> &'static TypeInfo {
        COMBUSTIONENGINECONFIGDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static COMBUSTIONENGINECONFIGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CombustionEngineConfigData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("CombustionEngineConfigData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EngineConfigData {
    pub _glacier_base: super::core::DataContainer,
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

pub trait EngineConfigDataTrait: super::core::DataContainerTrait {
    fn position(&self) -> &super::core::Vec3;
    fn rpm_curve_points(&self) -> &Vec<f32>;
    fn torque_curve_points(&self) -> &Vec<f32>;
    fn rpm_min(&self) -> &f32;
    fn rpm_max(&self) -> &f32;
    fn rpm_cut(&self) -> &f32;
    fn engine_power_multiplier(&self) -> &f32;
    fn internal_acceleration_factor(&self) -> &f32;
    fn internal_deacceleration_factor(&self) -> &f32;
    fn max_reverse_speed(&self) -> &f32;
    fn boost(&self) -> &Boost;
    fn max_vehicle_height_offset(&self) -> &f32;
}

impl EngineConfigDataTrait for EngineConfigData {
    fn position(&self) -> &super::core::Vec3 {
        &self.position
    }
    fn rpm_curve_points(&self) -> &Vec<f32> {
        &self.rpm_curve_points
    }
    fn torque_curve_points(&self) -> &Vec<f32> {
        &self.torque_curve_points
    }
    fn rpm_min(&self) -> &f32 {
        &self.rpm_min
    }
    fn rpm_max(&self) -> &f32 {
        &self.rpm_max
    }
    fn rpm_cut(&self) -> &f32 {
        &self.rpm_cut
    }
    fn engine_power_multiplier(&self) -> &f32 {
        &self.engine_power_multiplier
    }
    fn internal_acceleration_factor(&self) -> &f32 {
        &self.internal_acceleration_factor
    }
    fn internal_deacceleration_factor(&self) -> &f32 {
        &self.internal_deacceleration_factor
    }
    fn max_reverse_speed(&self) -> &f32 {
        &self.max_reverse_speed
    }
    fn boost(&self) -> &Boost {
        &self.boost
    }
    fn max_vehicle_height_offset(&self) -> &f32 {
        &self.max_vehicle_height_offset
    }
}

impl super::core::DataContainerTrait for EngineConfigData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static ENGINECONFIGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EngineConfigData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EngineConfigData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Position",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(EngineConfigData, position),
            },
            FieldInfoData {
                name: "RpmCurvePoints",
                flags: MemberInfoFlags::new(144),
                field_type: "Float32-Array",
                rust_offset: offset_of!(EngineConfigData, rpm_curve_points),
            },
            FieldInfoData {
                name: "TorqueCurvePoints",
                flags: MemberInfoFlags::new(144),
                field_type: "Float32-Array",
                rust_offset: offset_of!(EngineConfigData, torque_curve_points),
            },
            FieldInfoData {
                name: "RpmMin",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EngineConfigData, rpm_min),
            },
            FieldInfoData {
                name: "RpmMax",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EngineConfigData, rpm_max),
            },
            FieldInfoData {
                name: "RpmCut",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EngineConfigData, rpm_cut),
            },
            FieldInfoData {
                name: "EnginePowerMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EngineConfigData, engine_power_multiplier),
            },
            FieldInfoData {
                name: "InternalAccelerationFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EngineConfigData, internal_acceleration_factor),
            },
            FieldInfoData {
                name: "InternalDeaccelerationFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EngineConfigData, internal_deacceleration_factor),
            },
            FieldInfoData {
                name: "MaxReverseSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EngineConfigData, max_reverse_speed),
            },
            FieldInfoData {
                name: "Boost",
                flags: MemberInfoFlags::new(0),
                field_type: "Boost",
                rust_offset: offset_of!(EngineConfigData, boost),
            },
            FieldInfoData {
                name: "MaxVehicleHeightOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EngineConfigData, max_vehicle_height_offset),
            },
        ],
    }),
    array_type: Some(ENGINECONFIGDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EngineConfigData {
    fn type_info(&self) -> &'static TypeInfo {
        ENGINECONFIGDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ENGINECONFIGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EngineConfigData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("EngineConfigData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Boost {
    pub forward_strength: f32,
    pub reverse_strength: f32,
    pub dissipation_time: f32,
    pub recovery_time: f32,
    pub crawl_strength: f32,
    pub acceleration_scale: f32,
}

pub trait BoostTrait: TypeObject {
    fn forward_strength(&self) -> &f32;
    fn reverse_strength(&self) -> &f32;
    fn dissipation_time(&self) -> &f32;
    fn recovery_time(&self) -> &f32;
    fn crawl_strength(&self) -> &f32;
    fn acceleration_scale(&self) -> &f32;
}

impl BoostTrait for Boost {
    fn forward_strength(&self) -> &f32 {
        &self.forward_strength
    }
    fn reverse_strength(&self) -> &f32 {
        &self.reverse_strength
    }
    fn dissipation_time(&self) -> &f32 {
        &self.dissipation_time
    }
    fn recovery_time(&self) -> &f32 {
        &self.recovery_time
    }
    fn crawl_strength(&self) -> &f32 {
        &self.crawl_strength
    }
    fn acceleration_scale(&self) -> &f32 {
        &self.acceleration_scale
    }
}

pub static BOOST_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Boost",
    flags: MemberInfoFlags::new(36937),
    module: "Physics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Boost as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ForwardStrength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Boost, forward_strength),
            },
            FieldInfoData {
                name: "ReverseStrength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Boost, reverse_strength),
            },
            FieldInfoData {
                name: "DissipationTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Boost, dissipation_time),
            },
            FieldInfoData {
                name: "RecoveryTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Boost, recovery_time),
            },
            FieldInfoData {
                name: "CrawlStrength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Boost, crawl_strength),
            },
            FieldInfoData {
                name: "AccelerationScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Boost, acceleration_scale),
            },
        ],
    }),
    array_type: Some(BOOST_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for Boost {
    fn type_info(&self) -> &'static TypeInfo {
        BOOST_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static BOOST_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Boost-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("Boost"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct GearboxConfigData {
    pub _glacier_base: super::core::DataContainer,
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

pub trait GearboxConfigDataTrait: super::core::DataContainerTrait {
    fn forward_gear_ratios(&self) -> &Vec<f32>;
    fn forward_gear_speeds(&self) -> &Vec<f32>;
    fn reverse_gear_ratios(&self) -> &Vec<f32>;
    fn reverse_gear_speeds(&self) -> &Vec<f32>;
    fn gearbox_type(&self) -> &i32;
    fn gearbox_mode(&self) -> &i32;
    fn gear_change_time(&self) -> &f32;
    fn gear_down_speed_factor(&self) -> &f32;
    fn opposite_dir_gear_change_max_speed(&self) -> &f32;
    fn opposite_dir_gear_change_time(&self) -> &f32;
    fn clutch_speed_factor(&self) -> &f32;
    fn transmission_efficiency(&self) -> &f32;
    fn use_auto_clutch(&self) -> &bool;
    fn use_classic_gear_box_auto_clutch(&self) -> &bool;
    fn use_neutral_gear(&self) -> &bool;
}

impl GearboxConfigDataTrait for GearboxConfigData {
    fn forward_gear_ratios(&self) -> &Vec<f32> {
        &self.forward_gear_ratios
    }
    fn forward_gear_speeds(&self) -> &Vec<f32> {
        &self.forward_gear_speeds
    }
    fn reverse_gear_ratios(&self) -> &Vec<f32> {
        &self.reverse_gear_ratios
    }
    fn reverse_gear_speeds(&self) -> &Vec<f32> {
        &self.reverse_gear_speeds
    }
    fn gearbox_type(&self) -> &i32 {
        &self.gearbox_type
    }
    fn gearbox_mode(&self) -> &i32 {
        &self.gearbox_mode
    }
    fn gear_change_time(&self) -> &f32 {
        &self.gear_change_time
    }
    fn gear_down_speed_factor(&self) -> &f32 {
        &self.gear_down_speed_factor
    }
    fn opposite_dir_gear_change_max_speed(&self) -> &f32 {
        &self.opposite_dir_gear_change_max_speed
    }
    fn opposite_dir_gear_change_time(&self) -> &f32 {
        &self.opposite_dir_gear_change_time
    }
    fn clutch_speed_factor(&self) -> &f32 {
        &self.clutch_speed_factor
    }
    fn transmission_efficiency(&self) -> &f32 {
        &self.transmission_efficiency
    }
    fn use_auto_clutch(&self) -> &bool {
        &self.use_auto_clutch
    }
    fn use_classic_gear_box_auto_clutch(&self) -> &bool {
        &self.use_classic_gear_box_auto_clutch
    }
    fn use_neutral_gear(&self) -> &bool {
        &self.use_neutral_gear
    }
}

impl super::core::DataContainerTrait for GearboxConfigData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static GEARBOXCONFIGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GearboxConfigData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GearboxConfigData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ForwardGearRatios",
                flags: MemberInfoFlags::new(144),
                field_type: "Float32-Array",
                rust_offset: offset_of!(GearboxConfigData, forward_gear_ratios),
            },
            FieldInfoData {
                name: "ForwardGearSpeeds",
                flags: MemberInfoFlags::new(144),
                field_type: "Float32-Array",
                rust_offset: offset_of!(GearboxConfigData, forward_gear_speeds),
            },
            FieldInfoData {
                name: "ReverseGearRatios",
                flags: MemberInfoFlags::new(144),
                field_type: "Float32-Array",
                rust_offset: offset_of!(GearboxConfigData, reverse_gear_ratios),
            },
            FieldInfoData {
                name: "ReverseGearSpeeds",
                flags: MemberInfoFlags::new(144),
                field_type: "Float32-Array",
                rust_offset: offset_of!(GearboxConfigData, reverse_gear_speeds),
            },
            FieldInfoData {
                name: "GearboxType",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(GearboxConfigData, gearbox_type),
            },
            FieldInfoData {
                name: "GearboxMode",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(GearboxConfigData, gearbox_mode),
            },
            FieldInfoData {
                name: "GearChangeTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(GearboxConfigData, gear_change_time),
            },
            FieldInfoData {
                name: "GearDownSpeedFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(GearboxConfigData, gear_down_speed_factor),
            },
            FieldInfoData {
                name: "OppositeDirGearChangeMaxSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(GearboxConfigData, opposite_dir_gear_change_max_speed),
            },
            FieldInfoData {
                name: "OppositeDirGearChangeTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(GearboxConfigData, opposite_dir_gear_change_time),
            },
            FieldInfoData {
                name: "ClutchSpeedFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(GearboxConfigData, clutch_speed_factor),
            },
            FieldInfoData {
                name: "TransmissionEfficiency",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(GearboxConfigData, transmission_efficiency),
            },
            FieldInfoData {
                name: "UseAutoClutch",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GearboxConfigData, use_auto_clutch),
            },
            FieldInfoData {
                name: "UseClassicGearBoxAutoClutch",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GearboxConfigData, use_classic_gear_box_auto_clutch),
            },
            FieldInfoData {
                name: "UseNeutralGear",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GearboxConfigData, use_neutral_gear),
            },
        ],
    }),
    array_type: Some(GEARBOXCONFIGDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GearboxConfigData {
    fn type_info(&self) -> &'static TypeInfo {
        GEARBOXCONFIGDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static GEARBOXCONFIGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GearboxConfigData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("GearboxConfigData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VehicleInputTweakData {
    pub _glacier_base: super::core::DataContainer,
    pub input_tweak_type: VehicleInputTweakType,
    pub min_speed: f32,
    pub max_speed: f32,
    pub min_speed_scale: f32,
    pub max_speed_scale: f32,
}

pub trait VehicleInputTweakDataTrait: super::core::DataContainerTrait {
    fn input_tweak_type(&self) -> &VehicleInputTweakType;
    fn min_speed(&self) -> &f32;
    fn max_speed(&self) -> &f32;
    fn min_speed_scale(&self) -> &f32;
    fn max_speed_scale(&self) -> &f32;
}

impl VehicleInputTweakDataTrait for VehicleInputTweakData {
    fn input_tweak_type(&self) -> &VehicleInputTweakType {
        &self.input_tweak_type
    }
    fn min_speed(&self) -> &f32 {
        &self.min_speed
    }
    fn max_speed(&self) -> &f32 {
        &self.max_speed
    }
    fn min_speed_scale(&self) -> &f32 {
        &self.min_speed_scale
    }
    fn max_speed_scale(&self) -> &f32 {
        &self.max_speed_scale
    }
}

impl super::core::DataContainerTrait for VehicleInputTweakData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static VEHICLEINPUTTWEAKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleInputTweakData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VehicleInputTweakData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "InputTweakType",
                flags: MemberInfoFlags::new(0),
                field_type: "VehicleInputTweakType",
                rust_offset: offset_of!(VehicleInputTweakData, input_tweak_type),
            },
            FieldInfoData {
                name: "MinSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleInputTweakData, min_speed),
            },
            FieldInfoData {
                name: "MaxSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleInputTweakData, max_speed),
            },
            FieldInfoData {
                name: "MinSpeedScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleInputTweakData, min_speed_scale),
            },
            FieldInfoData {
                name: "MaxSpeedScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleInputTweakData, max_speed_scale),
            },
        ],
    }),
    array_type: Some(VEHICLEINPUTTWEAKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VehicleInputTweakData {
    fn type_info(&self) -> &'static TypeInfo {
        VEHICLEINPUTTWEAKDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VEHICLEINPUTTWEAKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleInputTweakData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("VehicleInputTweakData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum VehicleInputTweakType {
    #[default]
    CombinedTimedSpeedTweakType = 0,
    CombinedSpeedTweakType = 1,
    SpeedTweakType = 2,
    ScaledSpeedTweakType = 3,
}

pub static VEHICLEINPUTTWEAKTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleInputTweakType",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(VEHICLEINPUTTWEAKTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for VehicleInputTweakType {
    fn type_info(&self) -> &'static TypeInfo {
        VEHICLEINPUTTWEAKTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VEHICLEINPUTTWEAKTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleInputTweakType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("VehicleInputTweakType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MotionDampingData {
    pub _glacier_base: super::core::DataContainer,
    pub linear_modifier: super::core::Vec3,
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
    pub linear: f32,
}

pub trait MotionDampingDataTrait: super::core::DataContainerTrait {
    fn linear_modifier(&self) -> &super::core::Vec3;
    fn pitch(&self) -> &f32;
    fn yaw(&self) -> &f32;
    fn roll(&self) -> &f32;
    fn linear(&self) -> &f32;
}

impl MotionDampingDataTrait for MotionDampingData {
    fn linear_modifier(&self) -> &super::core::Vec3 {
        &self.linear_modifier
    }
    fn pitch(&self) -> &f32 {
        &self.pitch
    }
    fn yaw(&self) -> &f32 {
        &self.yaw
    }
    fn roll(&self) -> &f32 {
        &self.roll
    }
    fn linear(&self) -> &f32 {
        &self.linear
    }
}

impl super::core::DataContainerTrait for MotionDampingData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static MOTIONDAMPINGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MotionDampingData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MotionDampingData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "LinearModifier",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(MotionDampingData, linear_modifier),
            },
            FieldInfoData {
                name: "Pitch",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotionDampingData, pitch),
            },
            FieldInfoData {
                name: "Yaw",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotionDampingData, yaw),
            },
            FieldInfoData {
                name: "Roll",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotionDampingData, roll),
            },
            FieldInfoData {
                name: "Linear",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotionDampingData, linear),
            },
        ],
    }),
    array_type: Some(MOTIONDAMPINGDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for MotionDampingData {
    fn type_info(&self) -> &'static TypeInfo {
        MOTIONDAMPINGDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MOTIONDAMPINGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MotionDampingData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("MotionDampingData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct StabilizerData {
    pub _glacier_base: super::core::DataContainer,
    pub pitch_strength: f32,
    pub roll_strength: f32,
    pub advanced: bool,
    pub yaw_strength: f32,
    pub advanced_yaw: bool,
    pub vertical_velocity_strength: f32,
}

pub trait StabilizerDataTrait: super::core::DataContainerTrait {
    fn pitch_strength(&self) -> &f32;
    fn roll_strength(&self) -> &f32;
    fn advanced(&self) -> &bool;
    fn yaw_strength(&self) -> &f32;
    fn advanced_yaw(&self) -> &bool;
    fn vertical_velocity_strength(&self) -> &f32;
}

impl StabilizerDataTrait for StabilizerData {
    fn pitch_strength(&self) -> &f32 {
        &self.pitch_strength
    }
    fn roll_strength(&self) -> &f32 {
        &self.roll_strength
    }
    fn advanced(&self) -> &bool {
        &self.advanced
    }
    fn yaw_strength(&self) -> &f32 {
        &self.yaw_strength
    }
    fn advanced_yaw(&self) -> &bool {
        &self.advanced_yaw
    }
    fn vertical_velocity_strength(&self) -> &f32 {
        &self.vertical_velocity_strength
    }
}

impl super::core::DataContainerTrait for StabilizerData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static STABILIZERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StabilizerData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StabilizerData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "PitchStrength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(StabilizerData, pitch_strength),
            },
            FieldInfoData {
                name: "RollStrength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(StabilizerData, roll_strength),
            },
            FieldInfoData {
                name: "Advanced",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(StabilizerData, advanced),
            },
            FieldInfoData {
                name: "YawStrength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(StabilizerData, yaw_strength),
            },
            FieldInfoData {
                name: "AdvancedYaw",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(StabilizerData, advanced_yaw),
            },
            FieldInfoData {
                name: "VerticalVelocityStrength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(StabilizerData, vertical_velocity_strength),
            },
        ],
    }),
    array_type: Some(STABILIZERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StabilizerData {
    fn type_info(&self) -> &'static TypeInfo {
        STABILIZERDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static STABILIZERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StabilizerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("StabilizerData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WingPhysicsData {
    pub _glacier_base: super::core::DataContainer,
    pub lift: f32,
    pub flap_lift: f32,
    pub lift_coefficient: Option<Arc<Mutex<dyn Curve2DTrait>>>,
    pub drag: f32,
    pub flap_drag: f32,
    pub drag_coefficient: Option<Arc<Mutex<dyn Curve2DTrait>>>,
    pub angular_velocity_lift_modifier: f32,
    pub drag_rotation_modifier: f32,
    pub base_angle_of_attack: f32,
    pub input_for_flap: i32,
    pub flap_turn_speed: f32,
    pub visual_flap_turn_speed: f32,
    pub visual_flap_angle_limit: f32,
    pub max_flap_angle_scale_factor: Option<Arc<Mutex<dyn Curve2DTrait>>>,
    pub landing_flap_lift: f32,
    pub landing_flap_logic: Option<Arc<Mutex<dyn LandingFlapDataTrait>>>,
}

pub trait WingPhysicsDataTrait: super::core::DataContainerTrait {
    fn lift(&self) -> &f32;
    fn flap_lift(&self) -> &f32;
    fn lift_coefficient(&self) -> &Option<Arc<Mutex<dyn Curve2DTrait>>>;
    fn drag(&self) -> &f32;
    fn flap_drag(&self) -> &f32;
    fn drag_coefficient(&self) -> &Option<Arc<Mutex<dyn Curve2DTrait>>>;
    fn angular_velocity_lift_modifier(&self) -> &f32;
    fn drag_rotation_modifier(&self) -> &f32;
    fn base_angle_of_attack(&self) -> &f32;
    fn input_for_flap(&self) -> &i32;
    fn flap_turn_speed(&self) -> &f32;
    fn visual_flap_turn_speed(&self) -> &f32;
    fn visual_flap_angle_limit(&self) -> &f32;
    fn max_flap_angle_scale_factor(&self) -> &Option<Arc<Mutex<dyn Curve2DTrait>>>;
    fn landing_flap_lift(&self) -> &f32;
    fn landing_flap_logic(&self) -> &Option<Arc<Mutex<dyn LandingFlapDataTrait>>>;
}

impl WingPhysicsDataTrait for WingPhysicsData {
    fn lift(&self) -> &f32 {
        &self.lift
    }
    fn flap_lift(&self) -> &f32 {
        &self.flap_lift
    }
    fn lift_coefficient(&self) -> &Option<Arc<Mutex<dyn Curve2DTrait>>> {
        &self.lift_coefficient
    }
    fn drag(&self) -> &f32 {
        &self.drag
    }
    fn flap_drag(&self) -> &f32 {
        &self.flap_drag
    }
    fn drag_coefficient(&self) -> &Option<Arc<Mutex<dyn Curve2DTrait>>> {
        &self.drag_coefficient
    }
    fn angular_velocity_lift_modifier(&self) -> &f32 {
        &self.angular_velocity_lift_modifier
    }
    fn drag_rotation_modifier(&self) -> &f32 {
        &self.drag_rotation_modifier
    }
    fn base_angle_of_attack(&self) -> &f32 {
        &self.base_angle_of_attack
    }
    fn input_for_flap(&self) -> &i32 {
        &self.input_for_flap
    }
    fn flap_turn_speed(&self) -> &f32 {
        &self.flap_turn_speed
    }
    fn visual_flap_turn_speed(&self) -> &f32 {
        &self.visual_flap_turn_speed
    }
    fn visual_flap_angle_limit(&self) -> &f32 {
        &self.visual_flap_angle_limit
    }
    fn max_flap_angle_scale_factor(&self) -> &Option<Arc<Mutex<dyn Curve2DTrait>>> {
        &self.max_flap_angle_scale_factor
    }
    fn landing_flap_lift(&self) -> &f32 {
        &self.landing_flap_lift
    }
    fn landing_flap_logic(&self) -> &Option<Arc<Mutex<dyn LandingFlapDataTrait>>> {
        &self.landing_flap_logic
    }
}

impl super::core::DataContainerTrait for WingPhysicsData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static WINGPHYSICSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WingPhysicsData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WingPhysicsData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Lift",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WingPhysicsData, lift),
            },
            FieldInfoData {
                name: "FlapLift",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WingPhysicsData, flap_lift),
            },
            FieldInfoData {
                name: "LiftCoefficient",
                flags: MemberInfoFlags::new(0),
                field_type: "Curve2D",
                rust_offset: offset_of!(WingPhysicsData, lift_coefficient),
            },
            FieldInfoData {
                name: "Drag",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WingPhysicsData, drag),
            },
            FieldInfoData {
                name: "FlapDrag",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WingPhysicsData, flap_drag),
            },
            FieldInfoData {
                name: "DragCoefficient",
                flags: MemberInfoFlags::new(0),
                field_type: "Curve2D",
                rust_offset: offset_of!(WingPhysicsData, drag_coefficient),
            },
            FieldInfoData {
                name: "AngularVelocityLiftModifier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WingPhysicsData, angular_velocity_lift_modifier),
            },
            FieldInfoData {
                name: "DragRotationModifier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WingPhysicsData, drag_rotation_modifier),
            },
            FieldInfoData {
                name: "BaseAngleOfAttack",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WingPhysicsData, base_angle_of_attack),
            },
            FieldInfoData {
                name: "InputForFlap",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(WingPhysicsData, input_for_flap),
            },
            FieldInfoData {
                name: "FlapTurnSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WingPhysicsData, flap_turn_speed),
            },
            FieldInfoData {
                name: "VisualFlapTurnSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WingPhysicsData, visual_flap_turn_speed),
            },
            FieldInfoData {
                name: "VisualFlapAngleLimit",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WingPhysicsData, visual_flap_angle_limit),
            },
            FieldInfoData {
                name: "MaxFlapAngleScaleFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Curve2D",
                rust_offset: offset_of!(WingPhysicsData, max_flap_angle_scale_factor),
            },
            FieldInfoData {
                name: "LandingFlapLift",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WingPhysicsData, landing_flap_lift),
            },
            FieldInfoData {
                name: "LandingFlapLogic",
                flags: MemberInfoFlags::new(0),
                field_type: "LandingFlapData",
                rust_offset: offset_of!(WingPhysicsData, landing_flap_logic),
            },
        ],
    }),
    array_type: Some(WINGPHYSICSDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WingPhysicsData {
    fn type_info(&self) -> &'static TypeInfo {
        WINGPHYSICSDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WINGPHYSICSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WingPhysicsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("WingPhysicsData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Curve2D {
    pub _glacier_base: super::core::DataContainer,
    pub curve: Vec<super::core::Vec2>,
}

pub trait Curve2DTrait: super::core::DataContainerTrait {
    fn curve(&self) -> &Vec<super::core::Vec2>;
}

impl Curve2DTrait for Curve2D {
    fn curve(&self) -> &Vec<super::core::Vec2> {
        &self.curve
    }
}

impl super::core::DataContainerTrait for Curve2D {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static CURVE2D_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Curve2D",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Curve2D as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Curve",
                flags: MemberInfoFlags::new(144),
                field_type: "Vec2-Array",
                rust_offset: offset_of!(Curve2D, curve),
            },
        ],
    }),
    array_type: Some(CURVE2D_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Curve2D {
    fn type_info(&self) -> &'static TypeInfo {
        CURVE2D_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CURVE2D_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Curve2D-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("Curve2D"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LandingFlapData {
    pub _glacier_base: super::core::DataContainer,
    pub activation_height: f32,
    pub height_tolerance: f32,
    pub activation_velocity: f32,
    pub velocity_tolerance: f32,
}

pub trait LandingFlapDataTrait: super::core::DataContainerTrait {
    fn activation_height(&self) -> &f32;
    fn height_tolerance(&self) -> &f32;
    fn activation_velocity(&self) -> &f32;
    fn velocity_tolerance(&self) -> &f32;
}

impl LandingFlapDataTrait for LandingFlapData {
    fn activation_height(&self) -> &f32 {
        &self.activation_height
    }
    fn height_tolerance(&self) -> &f32 {
        &self.height_tolerance
    }
    fn activation_velocity(&self) -> &f32 {
        &self.activation_velocity
    }
    fn velocity_tolerance(&self) -> &f32 {
        &self.velocity_tolerance
    }
}

impl super::core::DataContainerTrait for LandingFlapData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static LANDINGFLAPDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LandingFlapData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LandingFlapData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ActivationHeight",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LandingFlapData, activation_height),
            },
            FieldInfoData {
                name: "HeightTolerance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LandingFlapData, height_tolerance),
            },
            FieldInfoData {
                name: "ActivationVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LandingFlapData, activation_velocity),
            },
            FieldInfoData {
                name: "VelocityTolerance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LandingFlapData, velocity_tolerance),
            },
        ],
    }),
    array_type: Some(LANDINGFLAPDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LandingFlapData {
    fn type_info(&self) -> &'static TypeInfo {
        LANDINGFLAPDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LANDINGFLAPDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LandingFlapData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("LandingFlapData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AeroDynamicPhysicsData {
    pub _glacier_base: super::core::DataContainer,
    pub body_drag: super::core::Vec3,
    pub body_drag_offset_y_z: super::core::Vec3,
    pub body_drag_offset_x_z: super::core::Vec3,
    pub body_drag_offset_x_y: super::core::Vec3,
}

pub trait AeroDynamicPhysicsDataTrait: super::core::DataContainerTrait {
    fn body_drag(&self) -> &super::core::Vec3;
    fn body_drag_offset_y_z(&self) -> &super::core::Vec3;
    fn body_drag_offset_x_z(&self) -> &super::core::Vec3;
    fn body_drag_offset_x_y(&self) -> &super::core::Vec3;
}

impl AeroDynamicPhysicsDataTrait for AeroDynamicPhysicsData {
    fn body_drag(&self) -> &super::core::Vec3 {
        &self.body_drag
    }
    fn body_drag_offset_y_z(&self) -> &super::core::Vec3 {
        &self.body_drag_offset_y_z
    }
    fn body_drag_offset_x_z(&self) -> &super::core::Vec3 {
        &self.body_drag_offset_x_z
    }
    fn body_drag_offset_x_y(&self) -> &super::core::Vec3 {
        &self.body_drag_offset_x_y
    }
}

impl super::core::DataContainerTrait for AeroDynamicPhysicsData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static AERODYNAMICPHYSICSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AeroDynamicPhysicsData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AeroDynamicPhysicsData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "BodyDrag",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AeroDynamicPhysicsData, body_drag),
            },
            FieldInfoData {
                name: "BodyDragOffsetYZ",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AeroDynamicPhysicsData, body_drag_offset_y_z),
            },
            FieldInfoData {
                name: "BodyDragOffsetXZ",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AeroDynamicPhysicsData, body_drag_offset_x_z),
            },
            FieldInfoData {
                name: "BodyDragOffsetXY",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AeroDynamicPhysicsData, body_drag_offset_x_y),
            },
        ],
    }),
    array_type: Some(AERODYNAMICPHYSICSDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AeroDynamicPhysicsData {
    fn type_info(&self) -> &'static TypeInfo {
        AERODYNAMICPHYSICSDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static AERODYNAMICPHYSICSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AeroDynamicPhysicsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("AeroDynamicPhysicsData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct HovercraftFloatPhysicsData {
    pub _glacier_base: HullFloatPhysicsData,
    pub land_resistance_axis_mod: super::core::Vec3,
    pub land_friction_axis_mod: super::core::Vec3,
    pub front_length: f32,
    pub side_length: f32,
}

pub trait HovercraftFloatPhysicsDataTrait: HullFloatPhysicsDataTrait {
    fn land_resistance_axis_mod(&self) -> &super::core::Vec3;
    fn land_friction_axis_mod(&self) -> &super::core::Vec3;
    fn front_length(&self) -> &f32;
    fn side_length(&self) -> &f32;
}

impl HovercraftFloatPhysicsDataTrait for HovercraftFloatPhysicsData {
    fn land_resistance_axis_mod(&self) -> &super::core::Vec3 {
        &self.land_resistance_axis_mod
    }
    fn land_friction_axis_mod(&self) -> &super::core::Vec3 {
        &self.land_friction_axis_mod
    }
    fn front_length(&self) -> &f32 {
        &self.front_length
    }
    fn side_length(&self) -> &f32 {
        &self.side_length
    }
}

impl HullFloatPhysicsDataTrait for HovercraftFloatPhysicsData {
    fn water_resistance_axis_mod(&self) -> &super::core::Vec3 {
        self._glacier_base.water_resistance_axis_mod()
    }
    fn water_friction_axis_mod(&self) -> &super::core::Vec3 {
        self._glacier_base.water_friction_axis_mod()
    }
    fn offset(&self) -> &super::core::Vec3 {
        self._glacier_base.offset()
    }
    fn sub_surface_splits(&self) -> &i32 {
        self._glacier_base.sub_surface_splits()
    }
    fn depth(&self) -> &f32 {
        self._glacier_base.depth()
    }
    fn width(&self) -> &f32 {
        self._glacier_base.width()
    }
    fn length(&self) -> &f32 {
        self._glacier_base.length()
    }
    fn front_curve_degree(&self) -> &f32 {
        self._glacier_base.front_curve_degree()
    }
    fn side_curve_degree(&self) -> &f32 {
        self._glacier_base.side_curve_degree()
    }
    fn non_engine_steer(&self) -> &f32 {
        self._glacier_base.non_engine_steer()
    }
    fn non_engine_steer_min_speed(&self) -> &f32 {
        self._glacier_base.non_engine_steer_min_speed()
    }
    fn non_engine_steer_max_speed(&self) -> &f32 {
        self._glacier_base.non_engine_steer_max_speed()
    }
    fn water_dampening_mod(&self) -> &f32 {
        self._glacier_base.water_dampening_mod()
    }
    fn lift_modifier(&self) -> &f32 {
        self._glacier_base.lift_modifier()
    }
    fn support_size_mod(&self) -> &f32 {
        self._glacier_base.support_size_mod()
    }
    fn angular_dampening(&self) -> &f32 {
        self._glacier_base.angular_dampening()
    }
    fn friction_throttle_modifier(&self) -> &f32 {
        self._glacier_base.friction_throttle_modifier()
    }
}

impl FloatPhysicsDataTrait for HovercraftFloatPhysicsData {
    fn density(&self) -> &f32 {
        self._glacier_base.density()
    }
    fn filled_density(&self) -> &f32 {
        self._glacier_base.filled_density()
    }
}

impl super::core::DataContainerTrait for HovercraftFloatPhysicsData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static HOVERCRAFTFLOATPHYSICSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HovercraftFloatPhysicsData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(HULLFLOATPHYSICSDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<HovercraftFloatPhysicsData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "LandResistanceAxisMod",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(HovercraftFloatPhysicsData, land_resistance_axis_mod),
            },
            FieldInfoData {
                name: "LandFrictionAxisMod",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(HovercraftFloatPhysicsData, land_friction_axis_mod),
            },
            FieldInfoData {
                name: "FrontLength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HovercraftFloatPhysicsData, front_length),
            },
            FieldInfoData {
                name: "SideLength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HovercraftFloatPhysicsData, side_length),
            },
        ],
    }),
    array_type: Some(HOVERCRAFTFLOATPHYSICSDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for HovercraftFloatPhysicsData {
    fn type_info(&self) -> &'static TypeInfo {
        HOVERCRAFTFLOATPHYSICSDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static HOVERCRAFTFLOATPHYSICSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HovercraftFloatPhysicsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("HovercraftFloatPhysicsData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BoatFloatPhysicsData {
    pub _glacier_base: HullFloatPhysicsData,
    pub front_ratio: f32,
}

pub trait BoatFloatPhysicsDataTrait: HullFloatPhysicsDataTrait {
    fn front_ratio(&self) -> &f32;
}

impl BoatFloatPhysicsDataTrait for BoatFloatPhysicsData {
    fn front_ratio(&self) -> &f32 {
        &self.front_ratio
    }
}

impl HullFloatPhysicsDataTrait for BoatFloatPhysicsData {
    fn water_resistance_axis_mod(&self) -> &super::core::Vec3 {
        self._glacier_base.water_resistance_axis_mod()
    }
    fn water_friction_axis_mod(&self) -> &super::core::Vec3 {
        self._glacier_base.water_friction_axis_mod()
    }
    fn offset(&self) -> &super::core::Vec3 {
        self._glacier_base.offset()
    }
    fn sub_surface_splits(&self) -> &i32 {
        self._glacier_base.sub_surface_splits()
    }
    fn depth(&self) -> &f32 {
        self._glacier_base.depth()
    }
    fn width(&self) -> &f32 {
        self._glacier_base.width()
    }
    fn length(&self) -> &f32 {
        self._glacier_base.length()
    }
    fn front_curve_degree(&self) -> &f32 {
        self._glacier_base.front_curve_degree()
    }
    fn side_curve_degree(&self) -> &f32 {
        self._glacier_base.side_curve_degree()
    }
    fn non_engine_steer(&self) -> &f32 {
        self._glacier_base.non_engine_steer()
    }
    fn non_engine_steer_min_speed(&self) -> &f32 {
        self._glacier_base.non_engine_steer_min_speed()
    }
    fn non_engine_steer_max_speed(&self) -> &f32 {
        self._glacier_base.non_engine_steer_max_speed()
    }
    fn water_dampening_mod(&self) -> &f32 {
        self._glacier_base.water_dampening_mod()
    }
    fn lift_modifier(&self) -> &f32 {
        self._glacier_base.lift_modifier()
    }
    fn support_size_mod(&self) -> &f32 {
        self._glacier_base.support_size_mod()
    }
    fn angular_dampening(&self) -> &f32 {
        self._glacier_base.angular_dampening()
    }
    fn friction_throttle_modifier(&self) -> &f32 {
        self._glacier_base.friction_throttle_modifier()
    }
}

impl FloatPhysicsDataTrait for BoatFloatPhysicsData {
    fn density(&self) -> &f32 {
        self._glacier_base.density()
    }
    fn filled_density(&self) -> &f32 {
        self._glacier_base.filled_density()
    }
}

impl super::core::DataContainerTrait for BoatFloatPhysicsData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static BOATFLOATPHYSICSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoatFloatPhysicsData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(HULLFLOATPHYSICSDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BoatFloatPhysicsData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "FrontRatio",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BoatFloatPhysicsData, front_ratio),
            },
        ],
    }),
    array_type: Some(BOATFLOATPHYSICSDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for BoatFloatPhysicsData {
    fn type_info(&self) -> &'static TypeInfo {
        BOATFLOATPHYSICSDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static BOATFLOATPHYSICSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoatFloatPhysicsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("BoatFloatPhysicsData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct HullFloatPhysicsData {
    pub _glacier_base: FloatPhysicsData,
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

pub trait HullFloatPhysicsDataTrait: FloatPhysicsDataTrait {
    fn water_resistance_axis_mod(&self) -> &super::core::Vec3;
    fn water_friction_axis_mod(&self) -> &super::core::Vec3;
    fn offset(&self) -> &super::core::Vec3;
    fn sub_surface_splits(&self) -> &i32;
    fn depth(&self) -> &f32;
    fn width(&self) -> &f32;
    fn length(&self) -> &f32;
    fn front_curve_degree(&self) -> &f32;
    fn side_curve_degree(&self) -> &f32;
    fn non_engine_steer(&self) -> &f32;
    fn non_engine_steer_min_speed(&self) -> &f32;
    fn non_engine_steer_max_speed(&self) -> &f32;
    fn water_dampening_mod(&self) -> &f32;
    fn lift_modifier(&self) -> &f32;
    fn support_size_mod(&self) -> &f32;
    fn angular_dampening(&self) -> &f32;
    fn friction_throttle_modifier(&self) -> &f32;
}

impl HullFloatPhysicsDataTrait for HullFloatPhysicsData {
    fn water_resistance_axis_mod(&self) -> &super::core::Vec3 {
        &self.water_resistance_axis_mod
    }
    fn water_friction_axis_mod(&self) -> &super::core::Vec3 {
        &self.water_friction_axis_mod
    }
    fn offset(&self) -> &super::core::Vec3 {
        &self.offset
    }
    fn sub_surface_splits(&self) -> &i32 {
        &self.sub_surface_splits
    }
    fn depth(&self) -> &f32 {
        &self.depth
    }
    fn width(&self) -> &f32 {
        &self.width
    }
    fn length(&self) -> &f32 {
        &self.length
    }
    fn front_curve_degree(&self) -> &f32 {
        &self.front_curve_degree
    }
    fn side_curve_degree(&self) -> &f32 {
        &self.side_curve_degree
    }
    fn non_engine_steer(&self) -> &f32 {
        &self.non_engine_steer
    }
    fn non_engine_steer_min_speed(&self) -> &f32 {
        &self.non_engine_steer_min_speed
    }
    fn non_engine_steer_max_speed(&self) -> &f32 {
        &self.non_engine_steer_max_speed
    }
    fn water_dampening_mod(&self) -> &f32 {
        &self.water_dampening_mod
    }
    fn lift_modifier(&self) -> &f32 {
        &self.lift_modifier
    }
    fn support_size_mod(&self) -> &f32 {
        &self.support_size_mod
    }
    fn angular_dampening(&self) -> &f32 {
        &self.angular_dampening
    }
    fn friction_throttle_modifier(&self) -> &f32 {
        &self.friction_throttle_modifier
    }
}

impl FloatPhysicsDataTrait for HullFloatPhysicsData {
    fn density(&self) -> &f32 {
        self._glacier_base.density()
    }
    fn filled_density(&self) -> &f32 {
        self._glacier_base.filled_density()
    }
}

impl super::core::DataContainerTrait for HullFloatPhysicsData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static HULLFLOATPHYSICSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HullFloatPhysicsData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(FLOATPHYSICSDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<HullFloatPhysicsData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "WaterResistanceAxisMod",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(HullFloatPhysicsData, water_resistance_axis_mod),
            },
            FieldInfoData {
                name: "WaterFrictionAxisMod",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(HullFloatPhysicsData, water_friction_axis_mod),
            },
            FieldInfoData {
                name: "Offset",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(HullFloatPhysicsData, offset),
            },
            FieldInfoData {
                name: "SubSurfaceSplits",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(HullFloatPhysicsData, sub_surface_splits),
            },
            FieldInfoData {
                name: "Depth",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HullFloatPhysicsData, depth),
            },
            FieldInfoData {
                name: "Width",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HullFloatPhysicsData, width),
            },
            FieldInfoData {
                name: "Length",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HullFloatPhysicsData, length),
            },
            FieldInfoData {
                name: "FrontCurveDegree",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HullFloatPhysicsData, front_curve_degree),
            },
            FieldInfoData {
                name: "SideCurveDegree",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HullFloatPhysicsData, side_curve_degree),
            },
            FieldInfoData {
                name: "NonEngineSteer",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HullFloatPhysicsData, non_engine_steer),
            },
            FieldInfoData {
                name: "NonEngineSteerMinSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HullFloatPhysicsData, non_engine_steer_min_speed),
            },
            FieldInfoData {
                name: "NonEngineSteerMaxSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HullFloatPhysicsData, non_engine_steer_max_speed),
            },
            FieldInfoData {
                name: "WaterDampeningMod",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HullFloatPhysicsData, water_dampening_mod),
            },
            FieldInfoData {
                name: "LiftModifier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HullFloatPhysicsData, lift_modifier),
            },
            FieldInfoData {
                name: "SupportSizeMod",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HullFloatPhysicsData, support_size_mod),
            },
            FieldInfoData {
                name: "AngularDampening",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HullFloatPhysicsData, angular_dampening),
            },
            FieldInfoData {
                name: "FrictionThrottleModifier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HullFloatPhysicsData, friction_throttle_modifier),
            },
        ],
    }),
    array_type: Some(HULLFLOATPHYSICSDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for HullFloatPhysicsData {
    fn type_info(&self) -> &'static TypeInfo {
        HULLFLOATPHYSICSDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static HULLFLOATPHYSICSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HullFloatPhysicsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("HullFloatPhysicsData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VehiclePhysicsActionData {
    pub _glacier_base: PhysicsActionData,
}

pub trait VehiclePhysicsActionDataTrait: PhysicsActionDataTrait {
}

impl VehiclePhysicsActionDataTrait for VehiclePhysicsActionData {
}

impl PhysicsActionDataTrait for VehiclePhysicsActionData {
}

impl super::entity::EntityDataTrait for VehiclePhysicsActionData {
}

impl super::entity::GameObjectDataTrait for VehiclePhysicsActionData {
}

impl super::core::DataBusPeerTrait for VehiclePhysicsActionData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for VehiclePhysicsActionData {
}

impl super::core::DataContainerTrait for VehiclePhysicsActionData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static VEHICLEPHYSICSACTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehiclePhysicsActionData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSACTIONDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VehiclePhysicsActionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(VEHICLEPHYSICSACTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VehiclePhysicsActionData {
    fn type_info(&self) -> &'static TypeInfo {
        VEHICLEPHYSICSACTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VEHICLEPHYSICSACTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehiclePhysicsActionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("VehiclePhysicsActionData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VehicleConfigData {
    pub _glacier_base: super::core::DataContainer,
    pub center_of_mass: super::core::Vec3,
    pub center_of_mass_handling_offset: super::core::Vec3,
    pub inertia_override: super::core::Vec3,
    pub inertia_modifier: super::core::Vec3,
    pub aero_dynamic_physics: Option<Arc<Mutex<dyn AeroDynamicPhysicsDataTrait>>>,
    pub motorbike_physics: Option<Arc<Mutex<dyn MotorbikeDataTrait>>>,
    pub motion_damping: Option<Arc<Mutex<dyn MotionDampingDataTrait>>>,
    pub input: VehicleInputData,
    pub float_physics: Option<Arc<Mutex<dyn FloatPhysicsDataTrait>>>,
    pub stabilizer: Option<Arc<Mutex<dyn StabilizerDataTrait>>>,
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

pub trait VehicleConfigDataTrait: super::core::DataContainerTrait {
    fn center_of_mass(&self) -> &super::core::Vec3;
    fn center_of_mass_handling_offset(&self) -> &super::core::Vec3;
    fn inertia_override(&self) -> &super::core::Vec3;
    fn inertia_modifier(&self) -> &super::core::Vec3;
    fn aero_dynamic_physics(&self) -> &Option<Arc<Mutex<dyn AeroDynamicPhysicsDataTrait>>>;
    fn motorbike_physics(&self) -> &Option<Arc<Mutex<dyn MotorbikeDataTrait>>>;
    fn motion_damping(&self) -> &Option<Arc<Mutex<dyn MotionDampingDataTrait>>>;
    fn input(&self) -> &VehicleInputData;
    fn float_physics(&self) -> &Option<Arc<Mutex<dyn FloatPhysicsDataTrait>>>;
    fn stabilizer(&self) -> &Option<Arc<Mutex<dyn StabilizerDataTrait>>>;
    fn stabilizers(&self) -> &Vec<StabilizerSettings>;
    fn constant_force(&self) -> &Vec<ConstantForceData>;
    fn vehicle_mode_at_reset(&self) -> &VehicleMode;
    fn body_mass(&self) -> &f32;
    fn gravity_modifier(&self) -> &f32;
    fn yaw_min(&self) -> &f32;
    fn yaw_max(&self) -> &f32;
    fn down_force_base_factor(&self) -> &f32;
    fn down_force_wheel_factor(&self) -> &f32;
    fn vehicle_mode_change_entering_time(&self) -> &f32;
    fn vehicle_mode_change_starting_time(&self) -> &f32;
    fn vehicle_mode_change_stopping_time(&self) -> &f32;
    fn vehicle_mode_change_leaving_time(&self) -> &f32;
    fn stand_still_low_speed_time_limit(&self) -> &f32;
    fn static_friction_break_collision_mod(&self) -> &f32;
    fn static_friction_break_velocity_mod(&self) -> &f32;
    fn coefficient_of_air_friction(&self) -> &f32;
    fn air_density(&self) -> &f32;
    fn air_drag_area(&self) -> &f32;
    fn wind_resistance_base_factor(&self) -> &f32;
    fn wind_resistance_velocity_factor(&self) -> &f32;
    fn wind_resistance_velocity_factor_min(&self) -> &f32;
    fn wind_resistance_velocity_factor_max(&self) -> &f32;
    fn use_down_force(&self) -> &bool;
    fn use_down_force_wheel_factor(&self) -> &bool;
    fn use_gearbox(&self) -> &bool;
    fn use_stand_still_brake(&self) -> &bool;
    fn use_stand_still_sleep(&self) -> &bool;
    fn use_turn_around_force(&self) -> &bool;
    fn use_motorcycle_control(&self) -> &bool;
    fn invert_pitch_allowed(&self) -> &bool;
    fn use_wind_resistance(&self) -> &bool;
    fn use_input_yaw_as_throttle(&self) -> &InputThrottle;
    fn anti_roll_bars(&self) -> &AntiRollBars;
    fn max_ground_speed(&self) -> &f32;
    fn proximity_ext_scale(&self) -> &super::core::Vec3;
    fn proximity_height_translation(&self) -> &f32;
    fn friction_at_low_velocity(&self) -> &f32;
    fn contact_material_velocity_threshold(&self) -> &f32;
}

impl VehicleConfigDataTrait for VehicleConfigData {
    fn center_of_mass(&self) -> &super::core::Vec3 {
        &self.center_of_mass
    }
    fn center_of_mass_handling_offset(&self) -> &super::core::Vec3 {
        &self.center_of_mass_handling_offset
    }
    fn inertia_override(&self) -> &super::core::Vec3 {
        &self.inertia_override
    }
    fn inertia_modifier(&self) -> &super::core::Vec3 {
        &self.inertia_modifier
    }
    fn aero_dynamic_physics(&self) -> &Option<Arc<Mutex<dyn AeroDynamicPhysicsDataTrait>>> {
        &self.aero_dynamic_physics
    }
    fn motorbike_physics(&self) -> &Option<Arc<Mutex<dyn MotorbikeDataTrait>>> {
        &self.motorbike_physics
    }
    fn motion_damping(&self) -> &Option<Arc<Mutex<dyn MotionDampingDataTrait>>> {
        &self.motion_damping
    }
    fn input(&self) -> &VehicleInputData {
        &self.input
    }
    fn float_physics(&self) -> &Option<Arc<Mutex<dyn FloatPhysicsDataTrait>>> {
        &self.float_physics
    }
    fn stabilizer(&self) -> &Option<Arc<Mutex<dyn StabilizerDataTrait>>> {
        &self.stabilizer
    }
    fn stabilizers(&self) -> &Vec<StabilizerSettings> {
        &self.stabilizers
    }
    fn constant_force(&self) -> &Vec<ConstantForceData> {
        &self.constant_force
    }
    fn vehicle_mode_at_reset(&self) -> &VehicleMode {
        &self.vehicle_mode_at_reset
    }
    fn body_mass(&self) -> &f32 {
        &self.body_mass
    }
    fn gravity_modifier(&self) -> &f32 {
        &self.gravity_modifier
    }
    fn yaw_min(&self) -> &f32 {
        &self.yaw_min
    }
    fn yaw_max(&self) -> &f32 {
        &self.yaw_max
    }
    fn down_force_base_factor(&self) -> &f32 {
        &self.down_force_base_factor
    }
    fn down_force_wheel_factor(&self) -> &f32 {
        &self.down_force_wheel_factor
    }
    fn vehicle_mode_change_entering_time(&self) -> &f32 {
        &self.vehicle_mode_change_entering_time
    }
    fn vehicle_mode_change_starting_time(&self) -> &f32 {
        &self.vehicle_mode_change_starting_time
    }
    fn vehicle_mode_change_stopping_time(&self) -> &f32 {
        &self.vehicle_mode_change_stopping_time
    }
    fn vehicle_mode_change_leaving_time(&self) -> &f32 {
        &self.vehicle_mode_change_leaving_time
    }
    fn stand_still_low_speed_time_limit(&self) -> &f32 {
        &self.stand_still_low_speed_time_limit
    }
    fn static_friction_break_collision_mod(&self) -> &f32 {
        &self.static_friction_break_collision_mod
    }
    fn static_friction_break_velocity_mod(&self) -> &f32 {
        &self.static_friction_break_velocity_mod
    }
    fn coefficient_of_air_friction(&self) -> &f32 {
        &self.coefficient_of_air_friction
    }
    fn air_density(&self) -> &f32 {
        &self.air_density
    }
    fn air_drag_area(&self) -> &f32 {
        &self.air_drag_area
    }
    fn wind_resistance_base_factor(&self) -> &f32 {
        &self.wind_resistance_base_factor
    }
    fn wind_resistance_velocity_factor(&self) -> &f32 {
        &self.wind_resistance_velocity_factor
    }
    fn wind_resistance_velocity_factor_min(&self) -> &f32 {
        &self.wind_resistance_velocity_factor_min
    }
    fn wind_resistance_velocity_factor_max(&self) -> &f32 {
        &self.wind_resistance_velocity_factor_max
    }
    fn use_down_force(&self) -> &bool {
        &self.use_down_force
    }
    fn use_down_force_wheel_factor(&self) -> &bool {
        &self.use_down_force_wheel_factor
    }
    fn use_gearbox(&self) -> &bool {
        &self.use_gearbox
    }
    fn use_stand_still_brake(&self) -> &bool {
        &self.use_stand_still_brake
    }
    fn use_stand_still_sleep(&self) -> &bool {
        &self.use_stand_still_sleep
    }
    fn use_turn_around_force(&self) -> &bool {
        &self.use_turn_around_force
    }
    fn use_motorcycle_control(&self) -> &bool {
        &self.use_motorcycle_control
    }
    fn invert_pitch_allowed(&self) -> &bool {
        &self.invert_pitch_allowed
    }
    fn use_wind_resistance(&self) -> &bool {
        &self.use_wind_resistance
    }
    fn use_input_yaw_as_throttle(&self) -> &InputThrottle {
        &self.use_input_yaw_as_throttle
    }
    fn anti_roll_bars(&self) -> &AntiRollBars {
        &self.anti_roll_bars
    }
    fn max_ground_speed(&self) -> &f32 {
        &self.max_ground_speed
    }
    fn proximity_ext_scale(&self) -> &super::core::Vec3 {
        &self.proximity_ext_scale
    }
    fn proximity_height_translation(&self) -> &f32 {
        &self.proximity_height_translation
    }
    fn friction_at_low_velocity(&self) -> &f32 {
        &self.friction_at_low_velocity
    }
    fn contact_material_velocity_threshold(&self) -> &f32 {
        &self.contact_material_velocity_threshold
    }
}

impl super::core::DataContainerTrait for VehicleConfigData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static VEHICLECONFIGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleConfigData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VehicleConfigData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "CenterOfMass",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(VehicleConfigData, center_of_mass),
            },
            FieldInfoData {
                name: "CenterOfMassHandlingOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(VehicleConfigData, center_of_mass_handling_offset),
            },
            FieldInfoData {
                name: "InertiaOverride",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(VehicleConfigData, inertia_override),
            },
            FieldInfoData {
                name: "InertiaModifier",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(VehicleConfigData, inertia_modifier),
            },
            FieldInfoData {
                name: "AeroDynamicPhysics",
                flags: MemberInfoFlags::new(0),
                field_type: "AeroDynamicPhysicsData",
                rust_offset: offset_of!(VehicleConfigData, aero_dynamic_physics),
            },
            FieldInfoData {
                name: "MotorbikePhysics",
                flags: MemberInfoFlags::new(0),
                field_type: "MotorbikeData",
                rust_offset: offset_of!(VehicleConfigData, motorbike_physics),
            },
            FieldInfoData {
                name: "MotionDamping",
                flags: MemberInfoFlags::new(0),
                field_type: "MotionDampingData",
                rust_offset: offset_of!(VehicleConfigData, motion_damping),
            },
            FieldInfoData {
                name: "Input",
                flags: MemberInfoFlags::new(0),
                field_type: "VehicleInputData",
                rust_offset: offset_of!(VehicleConfigData, input),
            },
            FieldInfoData {
                name: "FloatPhysics",
                flags: MemberInfoFlags::new(0),
                field_type: "FloatPhysicsData",
                rust_offset: offset_of!(VehicleConfigData, float_physics),
            },
            FieldInfoData {
                name: "Stabilizer",
                flags: MemberInfoFlags::new(0),
                field_type: "StabilizerData",
                rust_offset: offset_of!(VehicleConfigData, stabilizer),
            },
            FieldInfoData {
                name: "Stabilizers",
                flags: MemberInfoFlags::new(144),
                field_type: "StabilizerSettings-Array",
                rust_offset: offset_of!(VehicleConfigData, stabilizers),
            },
            FieldInfoData {
                name: "ConstantForce",
                flags: MemberInfoFlags::new(144),
                field_type: "ConstantForceData-Array",
                rust_offset: offset_of!(VehicleConfigData, constant_force),
            },
            FieldInfoData {
                name: "VehicleModeAtReset",
                flags: MemberInfoFlags::new(0),
                field_type: "VehicleMode",
                rust_offset: offset_of!(VehicleConfigData, vehicle_mode_at_reset),
            },
            FieldInfoData {
                name: "BodyMass",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleConfigData, body_mass),
            },
            FieldInfoData {
                name: "GravityModifier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleConfigData, gravity_modifier),
            },
            FieldInfoData {
                name: "YawMin",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleConfigData, yaw_min),
            },
            FieldInfoData {
                name: "YawMax",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleConfigData, yaw_max),
            },
            FieldInfoData {
                name: "DownForceBaseFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleConfigData, down_force_base_factor),
            },
            FieldInfoData {
                name: "DownForceWheelFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleConfigData, down_force_wheel_factor),
            },
            FieldInfoData {
                name: "VehicleModeChangeEnteringTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleConfigData, vehicle_mode_change_entering_time),
            },
            FieldInfoData {
                name: "VehicleModeChangeStartingTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleConfigData, vehicle_mode_change_starting_time),
            },
            FieldInfoData {
                name: "VehicleModeChangeStoppingTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleConfigData, vehicle_mode_change_stopping_time),
            },
            FieldInfoData {
                name: "VehicleModeChangeLeavingTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleConfigData, vehicle_mode_change_leaving_time),
            },
            FieldInfoData {
                name: "StandStillLowSpeedTimeLimit",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleConfigData, stand_still_low_speed_time_limit),
            },
            FieldInfoData {
                name: "StaticFrictionBreakCollisionMod",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleConfigData, static_friction_break_collision_mod),
            },
            FieldInfoData {
                name: "StaticFrictionBreakVelocityMod",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleConfigData, static_friction_break_velocity_mod),
            },
            FieldInfoData {
                name: "CoefficientOfAirFriction",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleConfigData, coefficient_of_air_friction),
            },
            FieldInfoData {
                name: "AirDensity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleConfigData, air_density),
            },
            FieldInfoData {
                name: "AirDragArea",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleConfigData, air_drag_area),
            },
            FieldInfoData {
                name: "WindResistanceBaseFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleConfigData, wind_resistance_base_factor),
            },
            FieldInfoData {
                name: "WindResistanceVelocityFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleConfigData, wind_resistance_velocity_factor),
            },
            FieldInfoData {
                name: "WindResistanceVelocityFactorMin",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleConfigData, wind_resistance_velocity_factor_min),
            },
            FieldInfoData {
                name: "WindResistanceVelocityFactorMax",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleConfigData, wind_resistance_velocity_factor_max),
            },
            FieldInfoData {
                name: "UseDownForce",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VehicleConfigData, use_down_force),
            },
            FieldInfoData {
                name: "UseDownForceWheelFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VehicleConfigData, use_down_force_wheel_factor),
            },
            FieldInfoData {
                name: "UseGearbox",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VehicleConfigData, use_gearbox),
            },
            FieldInfoData {
                name: "UseStandStillBrake",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VehicleConfigData, use_stand_still_brake),
            },
            FieldInfoData {
                name: "UseStandStillSleep",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VehicleConfigData, use_stand_still_sleep),
            },
            FieldInfoData {
                name: "UseTurnAroundForce",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VehicleConfigData, use_turn_around_force),
            },
            FieldInfoData {
                name: "UseMotorcycleControl",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VehicleConfigData, use_motorcycle_control),
            },
            FieldInfoData {
                name: "InvertPitchAllowed",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VehicleConfigData, invert_pitch_allowed),
            },
            FieldInfoData {
                name: "UseWindResistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VehicleConfigData, use_wind_resistance),
            },
            FieldInfoData {
                name: "UseInputYawAsThrottle",
                flags: MemberInfoFlags::new(0),
                field_type: "InputThrottle",
                rust_offset: offset_of!(VehicleConfigData, use_input_yaw_as_throttle),
            },
            FieldInfoData {
                name: "AntiRollBars",
                flags: MemberInfoFlags::new(0),
                field_type: "AntiRollBars",
                rust_offset: offset_of!(VehicleConfigData, anti_roll_bars),
            },
            FieldInfoData {
                name: "MaxGroundSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleConfigData, max_ground_speed),
            },
            FieldInfoData {
                name: "ProximityExtScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(VehicleConfigData, proximity_ext_scale),
            },
            FieldInfoData {
                name: "ProximityHeightTranslation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleConfigData, proximity_height_translation),
            },
            FieldInfoData {
                name: "FrictionAtLowVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleConfigData, friction_at_low_velocity),
            },
            FieldInfoData {
                name: "ContactMaterialVelocityThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleConfigData, contact_material_velocity_threshold),
            },
        ],
    }),
    array_type: Some(VEHICLECONFIGDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for VehicleConfigData {
    fn type_info(&self) -> &'static TypeInfo {
        VEHICLECONFIGDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VEHICLECONFIGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleConfigData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("VehicleConfigData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MotorbikeData {
    pub _glacier_base: super::core::DataContainer,
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

pub trait MotorbikeDataTrait: super::core::DataContainerTrait {
    fn max_leaning_roll(&self) -> &f32;
    fn counter_lean_force(&self) -> &f32;
    fn stand_still_lean_force(&self) -> &f32;
    fn lean_force(&self) -> &f32;
    fn jump_forward_lean_min_angle(&self) -> &f32;
    fn jump_forward_lean_momentum(&self) -> &f32;
    fn jump_forward_lean_min_no_contact_time(&self) -> &f32;
    fn lean_force_max_vel(&self) -> &f32;
    fn kickstand_roll(&self) -> &f32;
    fn kickstand_linear_damping(&self) -> &f32;
    fn stand_still_roll(&self) -> &f32;
    fn damp_big_jump_impact(&self) -> &bool;
    fn damp_big_jump_impact_counter_force(&self) -> &super::core::Vec3;
    fn damp_big_jump_impact_velocity(&self) -> &f32;
    fn damp_big_jump_max_spring_force_fraction(&self) -> &f32;
    fn damp_big_jump_impact_vertical_velocity(&self) -> &f32;
    fn yaw_brake_damping_lerp_start_scale(&self) -> &f32;
    fn yaw_brake_damping_lerp_end_scale(&self) -> &f32;
    fn stoppie_activation_velocity(&self) -> &f32;
    fn stoppie_start_velocity(&self) -> &f32;
    fn stoppie_stop_velocity(&self) -> &f32;
    fn stoppie_momentum(&self) -> &f32;
    fn short_offground_gravity_multiplier(&self) -> &f32;
    fn short_offground_period(&self) -> &f32;
    fn wheelie_switch_off_no_contact_time(&self) -> &f32;
    fn wheelie_max_no_down_force_contact_time(&self) -> &f32;
    fn wheelie_max_no_contact_time(&self) -> &f32;
    fn wheelie_steering_factor(&self) -> &f32;
    fn wheelie_inertia(&self) -> &f32;
    fn wheelie_start_velocity(&self) -> &f32;
    fn wheelie_max_velocity_undamped(&self) -> &f32;
    fn wheelie_max_velocity_damp_range(&self) -> &f32;
    fn wheelie_out_angular_momentum(&self) -> &f32;
    fn wheelie_spring_damping(&self) -> &f32;
    fn wheelie_spring_k_scale(&self) -> &f32;
    fn wheelie_angular_damping(&self) -> &f32;
    fn wheelie_angular_damping_speed(&self) -> &f32;
    fn wheelie_force(&self) -> &super::core::Vec3;
    fn wheelie_force_body_offset(&self) -> &super::core::Vec3;
    fn wheelie_max_angle(&self) -> &f32;
    fn wheelie_max_velocity(&self) -> &f32;
    fn wheelie_velocity_force_scale(&self) -> &f32;
    fn wheelie_angular_start_momentum(&self) -> &f32;
}

impl MotorbikeDataTrait for MotorbikeData {
    fn max_leaning_roll(&self) -> &f32 {
        &self.max_leaning_roll
    }
    fn counter_lean_force(&self) -> &f32 {
        &self.counter_lean_force
    }
    fn stand_still_lean_force(&self) -> &f32 {
        &self.stand_still_lean_force
    }
    fn lean_force(&self) -> &f32 {
        &self.lean_force
    }
    fn jump_forward_lean_min_angle(&self) -> &f32 {
        &self.jump_forward_lean_min_angle
    }
    fn jump_forward_lean_momentum(&self) -> &f32 {
        &self.jump_forward_lean_momentum
    }
    fn jump_forward_lean_min_no_contact_time(&self) -> &f32 {
        &self.jump_forward_lean_min_no_contact_time
    }
    fn lean_force_max_vel(&self) -> &f32 {
        &self.lean_force_max_vel
    }
    fn kickstand_roll(&self) -> &f32 {
        &self.kickstand_roll
    }
    fn kickstand_linear_damping(&self) -> &f32 {
        &self.kickstand_linear_damping
    }
    fn stand_still_roll(&self) -> &f32 {
        &self.stand_still_roll
    }
    fn damp_big_jump_impact(&self) -> &bool {
        &self.damp_big_jump_impact
    }
    fn damp_big_jump_impact_counter_force(&self) -> &super::core::Vec3 {
        &self.damp_big_jump_impact_counter_force
    }
    fn damp_big_jump_impact_velocity(&self) -> &f32 {
        &self.damp_big_jump_impact_velocity
    }
    fn damp_big_jump_max_spring_force_fraction(&self) -> &f32 {
        &self.damp_big_jump_max_spring_force_fraction
    }
    fn damp_big_jump_impact_vertical_velocity(&self) -> &f32 {
        &self.damp_big_jump_impact_vertical_velocity
    }
    fn yaw_brake_damping_lerp_start_scale(&self) -> &f32 {
        &self.yaw_brake_damping_lerp_start_scale
    }
    fn yaw_brake_damping_lerp_end_scale(&self) -> &f32 {
        &self.yaw_brake_damping_lerp_end_scale
    }
    fn stoppie_activation_velocity(&self) -> &f32 {
        &self.stoppie_activation_velocity
    }
    fn stoppie_start_velocity(&self) -> &f32 {
        &self.stoppie_start_velocity
    }
    fn stoppie_stop_velocity(&self) -> &f32 {
        &self.stoppie_stop_velocity
    }
    fn stoppie_momentum(&self) -> &f32 {
        &self.stoppie_momentum
    }
    fn short_offground_gravity_multiplier(&self) -> &f32 {
        &self.short_offground_gravity_multiplier
    }
    fn short_offground_period(&self) -> &f32 {
        &self.short_offground_period
    }
    fn wheelie_switch_off_no_contact_time(&self) -> &f32 {
        &self.wheelie_switch_off_no_contact_time
    }
    fn wheelie_max_no_down_force_contact_time(&self) -> &f32 {
        &self.wheelie_max_no_down_force_contact_time
    }
    fn wheelie_max_no_contact_time(&self) -> &f32 {
        &self.wheelie_max_no_contact_time
    }
    fn wheelie_steering_factor(&self) -> &f32 {
        &self.wheelie_steering_factor
    }
    fn wheelie_inertia(&self) -> &f32 {
        &self.wheelie_inertia
    }
    fn wheelie_start_velocity(&self) -> &f32 {
        &self.wheelie_start_velocity
    }
    fn wheelie_max_velocity_undamped(&self) -> &f32 {
        &self.wheelie_max_velocity_undamped
    }
    fn wheelie_max_velocity_damp_range(&self) -> &f32 {
        &self.wheelie_max_velocity_damp_range
    }
    fn wheelie_out_angular_momentum(&self) -> &f32 {
        &self.wheelie_out_angular_momentum
    }
    fn wheelie_spring_damping(&self) -> &f32 {
        &self.wheelie_spring_damping
    }
    fn wheelie_spring_k_scale(&self) -> &f32 {
        &self.wheelie_spring_k_scale
    }
    fn wheelie_angular_damping(&self) -> &f32 {
        &self.wheelie_angular_damping
    }
    fn wheelie_angular_damping_speed(&self) -> &f32 {
        &self.wheelie_angular_damping_speed
    }
    fn wheelie_force(&self) -> &super::core::Vec3 {
        &self.wheelie_force
    }
    fn wheelie_force_body_offset(&self) -> &super::core::Vec3 {
        &self.wheelie_force_body_offset
    }
    fn wheelie_max_angle(&self) -> &f32 {
        &self.wheelie_max_angle
    }
    fn wheelie_max_velocity(&self) -> &f32 {
        &self.wheelie_max_velocity
    }
    fn wheelie_velocity_force_scale(&self) -> &f32 {
        &self.wheelie_velocity_force_scale
    }
    fn wheelie_angular_start_momentum(&self) -> &f32 {
        &self.wheelie_angular_start_momentum
    }
}

impl super::core::DataContainerTrait for MotorbikeData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static MOTORBIKEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MotorbikeData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MotorbikeData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MaxLeaningRoll",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotorbikeData, max_leaning_roll),
            },
            FieldInfoData {
                name: "CounterLeanForce",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotorbikeData, counter_lean_force),
            },
            FieldInfoData {
                name: "StandStillLeanForce",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotorbikeData, stand_still_lean_force),
            },
            FieldInfoData {
                name: "LeanForce",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotorbikeData, lean_force),
            },
            FieldInfoData {
                name: "JumpForwardLeanMinAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotorbikeData, jump_forward_lean_min_angle),
            },
            FieldInfoData {
                name: "JumpForwardLeanMomentum",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotorbikeData, jump_forward_lean_momentum),
            },
            FieldInfoData {
                name: "JumpForwardLeanMinNoContactTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotorbikeData, jump_forward_lean_min_no_contact_time),
            },
            FieldInfoData {
                name: "LeanForceMaxVel",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotorbikeData, lean_force_max_vel),
            },
            FieldInfoData {
                name: "KickstandRoll",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotorbikeData, kickstand_roll),
            },
            FieldInfoData {
                name: "KickstandLinearDamping",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotorbikeData, kickstand_linear_damping),
            },
            FieldInfoData {
                name: "StandStillRoll",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotorbikeData, stand_still_roll),
            },
            FieldInfoData {
                name: "DampBigJumpImpact",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MotorbikeData, damp_big_jump_impact),
            },
            FieldInfoData {
                name: "DampBigJumpImpactCounterForce",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(MotorbikeData, damp_big_jump_impact_counter_force),
            },
            FieldInfoData {
                name: "DampBigJumpImpactVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotorbikeData, damp_big_jump_impact_velocity),
            },
            FieldInfoData {
                name: "DampBigJumpMaxSpringForceFraction",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotorbikeData, damp_big_jump_max_spring_force_fraction),
            },
            FieldInfoData {
                name: "DampBigJumpImpactVerticalVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotorbikeData, damp_big_jump_impact_vertical_velocity),
            },
            FieldInfoData {
                name: "YawBrakeDampingLerpStartScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotorbikeData, yaw_brake_damping_lerp_start_scale),
            },
            FieldInfoData {
                name: "YawBrakeDampingLerpEndScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotorbikeData, yaw_brake_damping_lerp_end_scale),
            },
            FieldInfoData {
                name: "StoppieActivationVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotorbikeData, stoppie_activation_velocity),
            },
            FieldInfoData {
                name: "StoppieStartVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotorbikeData, stoppie_start_velocity),
            },
            FieldInfoData {
                name: "StoppieStopVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotorbikeData, stoppie_stop_velocity),
            },
            FieldInfoData {
                name: "StoppieMomentum",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotorbikeData, stoppie_momentum),
            },
            FieldInfoData {
                name: "ShortOffgroundGravityMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotorbikeData, short_offground_gravity_multiplier),
            },
            FieldInfoData {
                name: "ShortOffgroundPeriod",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotorbikeData, short_offground_period),
            },
            FieldInfoData {
                name: "WheelieSwitchOffNoContactTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotorbikeData, wheelie_switch_off_no_contact_time),
            },
            FieldInfoData {
                name: "WheelieMaxNoDownForceContactTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotorbikeData, wheelie_max_no_down_force_contact_time),
            },
            FieldInfoData {
                name: "WheelieMaxNoContactTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotorbikeData, wheelie_max_no_contact_time),
            },
            FieldInfoData {
                name: "WheelieSteeringFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotorbikeData, wheelie_steering_factor),
            },
            FieldInfoData {
                name: "WheelieInertia",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotorbikeData, wheelie_inertia),
            },
            FieldInfoData {
                name: "WheelieStartVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotorbikeData, wheelie_start_velocity),
            },
            FieldInfoData {
                name: "WheelieMaxVelocityUndamped",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotorbikeData, wheelie_max_velocity_undamped),
            },
            FieldInfoData {
                name: "WheelieMaxVelocityDampRange",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotorbikeData, wheelie_max_velocity_damp_range),
            },
            FieldInfoData {
                name: "WheelieOutAngularMomentum",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotorbikeData, wheelie_out_angular_momentum),
            },
            FieldInfoData {
                name: "WheelieSpringDamping",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotorbikeData, wheelie_spring_damping),
            },
            FieldInfoData {
                name: "WheelieSpringKScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotorbikeData, wheelie_spring_k_scale),
            },
            FieldInfoData {
                name: "WheelieAngularDamping",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotorbikeData, wheelie_angular_damping),
            },
            FieldInfoData {
                name: "WheelieAngularDampingSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotorbikeData, wheelie_angular_damping_speed),
            },
            FieldInfoData {
                name: "WheelieForce",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(MotorbikeData, wheelie_force),
            },
            FieldInfoData {
                name: "WheelieForceBodyOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(MotorbikeData, wheelie_force_body_offset),
            },
            FieldInfoData {
                name: "WheelieMaxAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotorbikeData, wheelie_max_angle),
            },
            FieldInfoData {
                name: "WheelieMaxVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotorbikeData, wheelie_max_velocity),
            },
            FieldInfoData {
                name: "WheelieVelocityForceScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotorbikeData, wheelie_velocity_force_scale),
            },
            FieldInfoData {
                name: "WheelieAngularStartMomentum",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotorbikeData, wheelie_angular_start_momentum),
            },
        ],
    }),
    array_type: Some(MOTORBIKEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for MotorbikeData {
    fn type_info(&self) -> &'static TypeInfo {
        MOTORBIKEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MOTORBIKEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MotorbikeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("MotorbikeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct InputThrottle {
    pub enabled: bool,
    pub forward_speed_supression_amount: f32,
    pub backward_speed_supression_amount: f32,
    pub side_speed_supression_amount: f32,
    pub ignore_brake_speed_threshold: f32,
}

pub trait InputThrottleTrait: TypeObject {
    fn enabled(&self) -> &bool;
    fn forward_speed_supression_amount(&self) -> &f32;
    fn backward_speed_supression_amount(&self) -> &f32;
    fn side_speed_supression_amount(&self) -> &f32;
    fn ignore_brake_speed_threshold(&self) -> &f32;
}

impl InputThrottleTrait for InputThrottle {
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn forward_speed_supression_amount(&self) -> &f32 {
        &self.forward_speed_supression_amount
    }
    fn backward_speed_supression_amount(&self) -> &f32 {
        &self.backward_speed_supression_amount
    }
    fn side_speed_supression_amount(&self) -> &f32 {
        &self.side_speed_supression_amount
    }
    fn ignore_brake_speed_threshold(&self) -> &f32 {
        &self.ignore_brake_speed_threshold
    }
}

pub static INPUTTHROTTLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputThrottle",
    flags: MemberInfoFlags::new(36937),
    module: "Physics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InputThrottle as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(InputThrottle, enabled),
            },
            FieldInfoData {
                name: "ForwardSpeedSupressionAmount",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InputThrottle, forward_speed_supression_amount),
            },
            FieldInfoData {
                name: "BackwardSpeedSupressionAmount",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InputThrottle, backward_speed_supression_amount),
            },
            FieldInfoData {
                name: "SideSpeedSupressionAmount",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InputThrottle, side_speed_supression_amount),
            },
            FieldInfoData {
                name: "IgnoreBrakeSpeedThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InputThrottle, ignore_brake_speed_threshold),
            },
        ],
    }),
    array_type: Some(INPUTTHROTTLE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for InputThrottle {
    fn type_info(&self) -> &'static TypeInfo {
        INPUTTHROTTLE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static INPUTTHROTTLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputThrottle-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("InputThrottle"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ConstantForceData {
    pub condition: ForceCondition,
    pub type_of_force: ForceType,
    pub value: super::core::Vec3,
    pub space: SpaceType,
}

pub trait ConstantForceDataTrait: TypeObject {
    fn condition(&self) -> &ForceCondition;
    fn type_of_force(&self) -> &ForceType;
    fn value(&self) -> &super::core::Vec3;
    fn space(&self) -> &SpaceType;
}

impl ConstantForceDataTrait for ConstantForceData {
    fn condition(&self) -> &ForceCondition {
        &self.condition
    }
    fn type_of_force(&self) -> &ForceType {
        &self.type_of_force
    }
    fn value(&self) -> &super::core::Vec3 {
        &self.value
    }
    fn space(&self) -> &SpaceType {
        &self.space
    }
}

pub static CONSTANTFORCEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConstantForceData",
    flags: MemberInfoFlags::new(36937),
    module: "Physics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ConstantForceData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Condition",
                flags: MemberInfoFlags::new(0),
                field_type: "ForceCondition",
                rust_offset: offset_of!(ConstantForceData, condition),
            },
            FieldInfoData {
                name: "TypeOfForce",
                flags: MemberInfoFlags::new(0),
                field_type: "ForceType",
                rust_offset: offset_of!(ConstantForceData, type_of_force),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(ConstantForceData, value),
            },
            FieldInfoData {
                name: "Space",
                flags: MemberInfoFlags::new(0),
                field_type: "SpaceType",
                rust_offset: offset_of!(ConstantForceData, space),
            },
        ],
    }),
    array_type: Some(CONSTANTFORCEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ConstantForceData {
    fn type_info(&self) -> &'static TypeInfo {
        CONSTANTFORCEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CONSTANTFORCEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConstantForceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ConstantForceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum SpaceType {
    #[default]
    STBody = 0,
    STWorld = 1,
}

pub static SPACETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpaceType",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(SPACETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SpaceType {
    fn type_info(&self) -> &'static TypeInfo {
        SPACETYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SPACETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpaceType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("SpaceType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ForceType {
    #[default]
    FTForce = 0,
    FTTorque = 1,
}

pub static FORCETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForceType",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(FORCETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ForceType {
    fn type_info(&self) -> &'static TypeInfo {
        FORCETYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FORCETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForceType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ForceType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ForceCondition {
    #[default]
    FCNever = 0,
    FCNotCriticalDamaged = 1,
    FCCriticalDamaged = 2,
    FCNotOccupied = 3,
    FCOccupied = 4,
    FCAlways = 5,
}

pub static FORCECONDITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForceCondition",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(FORCECONDITION_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ForceCondition {
    fn type_info(&self) -> &'static TypeInfo {
        FORCECONDITION_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FORCECONDITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForceCondition-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ForceCondition"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct StabilizerSettings {
    pub property: StabilizerProperty,
    pub strength: f32,
    pub advanced: bool,
    pub radius: f32,
    pub use2_d_radius_test: bool,
    pub use_input_override: bool,
}

pub trait StabilizerSettingsTrait: TypeObject {
    fn property(&self) -> &StabilizerProperty;
    fn strength(&self) -> &f32;
    fn advanced(&self) -> &bool;
    fn radius(&self) -> &f32;
    fn use2_d_radius_test(&self) -> &bool;
    fn use_input_override(&self) -> &bool;
}

impl StabilizerSettingsTrait for StabilizerSettings {
    fn property(&self) -> &StabilizerProperty {
        &self.property
    }
    fn strength(&self) -> &f32 {
        &self.strength
    }
    fn advanced(&self) -> &bool {
        &self.advanced
    }
    fn radius(&self) -> &f32 {
        &self.radius
    }
    fn use2_d_radius_test(&self) -> &bool {
        &self.use2_d_radius_test
    }
    fn use_input_override(&self) -> &bool {
        &self.use_input_override
    }
}

pub static STABILIZERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StabilizerSettings",
    flags: MemberInfoFlags::new(36937),
    module: "Physics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StabilizerSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Property",
                flags: MemberInfoFlags::new(0),
                field_type: "StabilizerProperty",
                rust_offset: offset_of!(StabilizerSettings, property),
            },
            FieldInfoData {
                name: "Strength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(StabilizerSettings, strength),
            },
            FieldInfoData {
                name: "Advanced",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(StabilizerSettings, advanced),
            },
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(StabilizerSettings, radius),
            },
            FieldInfoData {
                name: "Use2DRadiusTest",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(StabilizerSettings, use2_d_radius_test),
            },
            FieldInfoData {
                name: "UseInputOverride",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(StabilizerSettings, use_input_override),
            },
        ],
    }),
    array_type: Some(STABILIZERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for StabilizerSettings {
    fn type_info(&self) -> &'static TypeInfo {
        STABILIZERSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static STABILIZERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StabilizerSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("StabilizerSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static STABILIZERPROPERTY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StabilizerProperty",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(STABILIZERPROPERTY_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for StabilizerProperty {
    fn type_info(&self) -> &'static TypeInfo {
        STABILIZERPROPERTY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static STABILIZERPROPERTY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StabilizerProperty-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("StabilizerProperty"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AntiRollBars {
    pub front: Option<Arc<Mutex<dyn AntiRollBarTrait>>>,
    pub rear: Option<Arc<Mutex<dyn AntiRollBarTrait>>>,
}

pub trait AntiRollBarsTrait: TypeObject {
    fn front(&self) -> &Option<Arc<Mutex<dyn AntiRollBarTrait>>>;
    fn rear(&self) -> &Option<Arc<Mutex<dyn AntiRollBarTrait>>>;
}

impl AntiRollBarsTrait for AntiRollBars {
    fn front(&self) -> &Option<Arc<Mutex<dyn AntiRollBarTrait>>> {
        &self.front
    }
    fn rear(&self) -> &Option<Arc<Mutex<dyn AntiRollBarTrait>>> {
        &self.rear
    }
}

pub static ANTIROLLBARS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntiRollBars",
    flags: MemberInfoFlags::new(73),
    module: "Physics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AntiRollBars as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Front",
                flags: MemberInfoFlags::new(0),
                field_type: "AntiRollBar",
                rust_offset: offset_of!(AntiRollBars, front),
            },
            FieldInfoData {
                name: "Rear",
                flags: MemberInfoFlags::new(0),
                field_type: "AntiRollBar",
                rust_offset: offset_of!(AntiRollBars, rear),
            },
        ],
    }),
    array_type: Some(ANTIROLLBARS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AntiRollBars {
    fn type_info(&self) -> &'static TypeInfo {
        ANTIROLLBARS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ANTIROLLBARS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntiRollBars-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("AntiRollBars"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AntiRollBar {
    pub _glacier_base: super::core::DataContainer,
    pub stiffness: f32,
    pub torque_activation_limit: f32,
}

pub trait AntiRollBarTrait: super::core::DataContainerTrait {
    fn stiffness(&self) -> &f32;
    fn torque_activation_limit(&self) -> &f32;
}

impl AntiRollBarTrait for AntiRollBar {
    fn stiffness(&self) -> &f32 {
        &self.stiffness
    }
    fn torque_activation_limit(&self) -> &f32 {
        &self.torque_activation_limit
    }
}

impl super::core::DataContainerTrait for AntiRollBar {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static ANTIROLLBAR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntiRollBar",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AntiRollBar as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Stiffness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AntiRollBar, stiffness),
            },
            FieldInfoData {
                name: "TorqueActivationLimit",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AntiRollBar, torque_activation_limit),
            },
        ],
    }),
    array_type: Some(ANTIROLLBAR_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AntiRollBar {
    fn type_info(&self) -> &'static TypeInfo {
        ANTIROLLBAR_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ANTIROLLBAR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntiRollBar-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("AntiRollBar"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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

pub trait VehicleInputDataTrait: TypeObject {
    fn throttle_deadzone(&self) -> &f32;
    fn brake_deadzone(&self) -> &f32;
    fn yaw_deadzone(&self) -> &f32;
    fn pitch_deadzone(&self) -> &f32;
    fn roll_deadzone(&self) -> &f32;
    fn throttle_inertia_out_duration(&self) -> &f32;
    fn throttle_inertia_in_duration(&self) -> &f32;
    fn throttle_inertia_min_ratio(&self) -> &f32;
    fn brake_inertia_out_duration(&self) -> &f32;
    fn brake_inertia_in_duration(&self) -> &f32;
    fn brake_inertia_min_ratio(&self) -> &f32;
    fn yaw_inertia_out_duration(&self) -> &f32;
    fn yaw_inertia_in_duration(&self) -> &f32;
    fn yaw_inertia_min_ratio(&self) -> &f32;
    fn pitch_inertia_out_duration(&self) -> &f32;
    fn pitch_inertia_in_duration(&self) -> &f32;
    fn pitch_inertia_min_ratio(&self) -> &f32;
    fn roll_inertia_out_duration(&self) -> &f32;
    fn roll_inertia_in_duration(&self) -> &f32;
    fn roll_inertia_min_ratio(&self) -> &f32;
}

impl VehicleInputDataTrait for VehicleInputData {
    fn throttle_deadzone(&self) -> &f32 {
        &self.throttle_deadzone
    }
    fn brake_deadzone(&self) -> &f32 {
        &self.brake_deadzone
    }
    fn yaw_deadzone(&self) -> &f32 {
        &self.yaw_deadzone
    }
    fn pitch_deadzone(&self) -> &f32 {
        &self.pitch_deadzone
    }
    fn roll_deadzone(&self) -> &f32 {
        &self.roll_deadzone
    }
    fn throttle_inertia_out_duration(&self) -> &f32 {
        &self.throttle_inertia_out_duration
    }
    fn throttle_inertia_in_duration(&self) -> &f32 {
        &self.throttle_inertia_in_duration
    }
    fn throttle_inertia_min_ratio(&self) -> &f32 {
        &self.throttle_inertia_min_ratio
    }
    fn brake_inertia_out_duration(&self) -> &f32 {
        &self.brake_inertia_out_duration
    }
    fn brake_inertia_in_duration(&self) -> &f32 {
        &self.brake_inertia_in_duration
    }
    fn brake_inertia_min_ratio(&self) -> &f32 {
        &self.brake_inertia_min_ratio
    }
    fn yaw_inertia_out_duration(&self) -> &f32 {
        &self.yaw_inertia_out_duration
    }
    fn yaw_inertia_in_duration(&self) -> &f32 {
        &self.yaw_inertia_in_duration
    }
    fn yaw_inertia_min_ratio(&self) -> &f32 {
        &self.yaw_inertia_min_ratio
    }
    fn pitch_inertia_out_duration(&self) -> &f32 {
        &self.pitch_inertia_out_duration
    }
    fn pitch_inertia_in_duration(&self) -> &f32 {
        &self.pitch_inertia_in_duration
    }
    fn pitch_inertia_min_ratio(&self) -> &f32 {
        &self.pitch_inertia_min_ratio
    }
    fn roll_inertia_out_duration(&self) -> &f32 {
        &self.roll_inertia_out_duration
    }
    fn roll_inertia_in_duration(&self) -> &f32 {
        &self.roll_inertia_in_duration
    }
    fn roll_inertia_min_ratio(&self) -> &f32 {
        &self.roll_inertia_min_ratio
    }
}

pub static VEHICLEINPUTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleInputData",
    flags: MemberInfoFlags::new(36937),
    module: "Physics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VehicleInputData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ThrottleDeadzone",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleInputData, throttle_deadzone),
            },
            FieldInfoData {
                name: "BrakeDeadzone",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleInputData, brake_deadzone),
            },
            FieldInfoData {
                name: "YawDeadzone",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleInputData, yaw_deadzone),
            },
            FieldInfoData {
                name: "PitchDeadzone",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleInputData, pitch_deadzone),
            },
            FieldInfoData {
                name: "RollDeadzone",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleInputData, roll_deadzone),
            },
            FieldInfoData {
                name: "ThrottleInertiaOutDuration",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleInputData, throttle_inertia_out_duration),
            },
            FieldInfoData {
                name: "ThrottleInertiaInDuration",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleInputData, throttle_inertia_in_duration),
            },
            FieldInfoData {
                name: "ThrottleInertiaMinRatio",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleInputData, throttle_inertia_min_ratio),
            },
            FieldInfoData {
                name: "BrakeInertiaOutDuration",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleInputData, brake_inertia_out_duration),
            },
            FieldInfoData {
                name: "BrakeInertiaInDuration",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleInputData, brake_inertia_in_duration),
            },
            FieldInfoData {
                name: "BrakeInertiaMinRatio",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleInputData, brake_inertia_min_ratio),
            },
            FieldInfoData {
                name: "YawInertiaOutDuration",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleInputData, yaw_inertia_out_duration),
            },
            FieldInfoData {
                name: "YawInertiaInDuration",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleInputData, yaw_inertia_in_duration),
            },
            FieldInfoData {
                name: "YawInertiaMinRatio",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleInputData, yaw_inertia_min_ratio),
            },
            FieldInfoData {
                name: "PitchInertiaOutDuration",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleInputData, pitch_inertia_out_duration),
            },
            FieldInfoData {
                name: "PitchInertiaInDuration",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleInputData, pitch_inertia_in_duration),
            },
            FieldInfoData {
                name: "PitchInertiaMinRatio",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleInputData, pitch_inertia_min_ratio),
            },
            FieldInfoData {
                name: "RollInertiaOutDuration",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleInputData, roll_inertia_out_duration),
            },
            FieldInfoData {
                name: "RollInertiaInDuration",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleInputData, roll_inertia_in_duration),
            },
            FieldInfoData {
                name: "RollInertiaMinRatio",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleInputData, roll_inertia_min_ratio),
            },
        ],
    }),
    array_type: Some(VEHICLEINPUTDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for VehicleInputData {
    fn type_info(&self) -> &'static TypeInfo {
        VEHICLEINPUTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VEHICLEINPUTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleInputData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("VehicleInputData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static VEHICLEMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleMode",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(VEHICLEMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for VehicleMode {
    fn type_info(&self) -> &'static TypeInfo {
        VEHICLEMODE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VEHICLEMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("VehicleMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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

pub trait GearboxNetStateTrait: TypeObject {
    fn gearbox_lock_mode(&self) -> &GearboxLockMode;
    fn gearbox_status(&self) -> &GearboxStatus;
    fn gear_status(&self) -> &GearStatus;
    fn clutch_status(&self) -> &ClutchStatus;
    fn gear(&self) -> &i8;
    fn gear_timer(&self) -> &f32;
    fn inverted_timer(&self) -> &f32;
    fn clutch_toggle(&self) -> &bool;
    fn is_gearbox_inverted(&self) -> &bool;
}

impl GearboxNetStateTrait for GearboxNetState {
    fn gearbox_lock_mode(&self) -> &GearboxLockMode {
        &self.gearbox_lock_mode
    }
    fn gearbox_status(&self) -> &GearboxStatus {
        &self.gearbox_status
    }
    fn gear_status(&self) -> &GearStatus {
        &self.gear_status
    }
    fn clutch_status(&self) -> &ClutchStatus {
        &self.clutch_status
    }
    fn gear(&self) -> &i8 {
        &self.gear
    }
    fn gear_timer(&self) -> &f32 {
        &self.gear_timer
    }
    fn inverted_timer(&self) -> &f32 {
        &self.inverted_timer
    }
    fn clutch_toggle(&self) -> &bool {
        &self.clutch_toggle
    }
    fn is_gearbox_inverted(&self) -> &bool {
        &self.is_gearbox_inverted
    }
}

pub static GEARBOXNETSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GearboxNetState",
    flags: MemberInfoFlags::new(36937),
    module: "Physics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GearboxNetState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "GearboxLockMode",
                flags: MemberInfoFlags::new(0),
                field_type: "GearboxLockMode",
                rust_offset: offset_of!(GearboxNetState, gearbox_lock_mode),
            },
            FieldInfoData {
                name: "GearboxStatus",
                flags: MemberInfoFlags::new(0),
                field_type: "GearboxStatus",
                rust_offset: offset_of!(GearboxNetState, gearbox_status),
            },
            FieldInfoData {
                name: "GearStatus",
                flags: MemberInfoFlags::new(0),
                field_type: "GearStatus",
                rust_offset: offset_of!(GearboxNetState, gear_status),
            },
            FieldInfoData {
                name: "ClutchStatus",
                flags: MemberInfoFlags::new(0),
                field_type: "ClutchStatus",
                rust_offset: offset_of!(GearboxNetState, clutch_status),
            },
            FieldInfoData {
                name: "Gear",
                flags: MemberInfoFlags::new(0),
                field_type: "Int8",
                rust_offset: offset_of!(GearboxNetState, gear),
            },
            FieldInfoData {
                name: "GearTimer",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(GearboxNetState, gear_timer),
            },
            FieldInfoData {
                name: "InvertedTimer",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(GearboxNetState, inverted_timer),
            },
            FieldInfoData {
                name: "ClutchToggle",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GearboxNetState, clutch_toggle),
            },
            FieldInfoData {
                name: "IsGearboxInverted",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GearboxNetState, is_gearbox_inverted),
            },
        ],
    }),
    array_type: Some(GEARBOXNETSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for GearboxNetState {
    fn type_info(&self) -> &'static TypeInfo {
        GEARBOXNETSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static GEARBOXNETSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GearboxNetState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("GearboxNetState"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum GearboxGearLimits {
    #[default]
    GearboxGearLimits_Reverse = 4294967288,
    GearboxGearLimits_Forward = 8,
}

pub static GEARBOXGEARLIMITS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GearboxGearLimits",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(GEARBOXGEARLIMITS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for GearboxGearLimits {
    fn type_info(&self) -> &'static TypeInfo {
        GEARBOXGEARLIMITS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static GEARBOXGEARLIMITS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GearboxGearLimits-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("GearboxGearLimits"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ClutchStatus {
    #[default]
    ClutchStatus_ClutchNoChange = 0,
    ClutchStatus_ClutchDownGearUp = 1,
    ClutchStatus_ClutchDownGearDown = 2,
    ClutchStatus_ClutchUp = 3,
}

pub static CLUTCHSTATUS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClutchStatus",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(CLUTCHSTATUS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ClutchStatus {
    fn type_info(&self) -> &'static TypeInfo {
        CLUTCHSTATUS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLUTCHSTATUS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClutchStatus-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ClutchStatus"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum GearStatus {
    #[default]
    GearStatus_GearNoChange = 0,
    GearStatus_GearDoGearUp = 1,
    GearStatus_GearGearUp = 2,
    GearStatus_GearDoGearDown = 3,
    GearStatus_GearGearDown = 4,
    GearStatus_GearInvertGearbox = 5,
}

pub static GEARSTATUS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GearStatus",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(GEARSTATUS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for GearStatus {
    fn type_info(&self) -> &'static TypeInfo {
        GEARSTATUS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static GEARSTATUS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GearStatus-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("GearStatus"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum GearboxStatus {
    #[default]
    GearboxStatus_GearboxNoChange = 0,
    GearboxStatus_GearboxClutch = 1,
}

pub static GEARBOXSTATUS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GearboxStatus",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(GEARBOXSTATUS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for GearboxStatus {
    fn type_info(&self) -> &'static TypeInfo {
        GEARBOXSTATUS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static GEARBOXSTATUS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GearboxStatus-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("GearboxStatus"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum GearboxLockMode {
    #[default]
    GearboxLockMode_NoLock = 0,
    GearboxLockMode_LockForward = 1,
    GearboxLockMode_LockReverse = 2,
}

pub static GEARBOXLOCKMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GearboxLockMode",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(GEARBOXLOCKMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for GearboxLockMode {
    fn type_info(&self) -> &'static TypeInfo {
        GEARBOXLOCKMODE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static GEARBOXLOCKMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GearboxLockMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("GearboxLockMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum GearboxMode {
    #[default]
    GearboxMode_Direct = 0,
    GearboxMode_Classic = 1,
    GearboxMode_MhStyle = 2,
}

pub static GEARBOXMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GearboxMode",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(GEARBOXMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for GearboxMode {
    fn type_info(&self) -> &'static TypeInfo {
        GEARBOXMODE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static GEARBOXMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GearboxMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("GearboxMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum GearboxType {
    #[default]
    GearboxType_Manual = 0,
    GearboxType_Automatic = 1,
}

pub static GEARBOXTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GearboxType",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(GEARBOXTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for GearboxType {
    fn type_info(&self) -> &'static TypeInfo {
        GEARBOXTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static GEARBOXTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GearboxType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("GearboxType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct IglooGrabToolSettings {
    pub _glacier_base: super::core::DataContainer,
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

pub trait IglooGrabToolSettingsTrait: super::core::DataContainerTrait {
    fn max_grab_distance(&self) -> &f32;
    fn debug_pick_point_sphere_radius(&self) -> &f32;
    fn debug_pick_point_normal_length(&self) -> &f32;
    fn debug_grab_hand_square_size(&self) -> &f32;
    fn debug_color_grab_hand(&self) -> &super::core::Vec4;
    fn debug_color_grab_target_position(&self) -> &super::core::Vec4;
    fn debug_color_grab_on_server(&self) -> &super::core::Vec4;
    fn debug_color_grab_on_client(&self) -> &super::core::Vec4;
    fn debug_color_grab_on_effect(&self) -> &super::core::Vec4;
}

impl IglooGrabToolSettingsTrait for IglooGrabToolSettings {
    fn max_grab_distance(&self) -> &f32 {
        &self.max_grab_distance
    }
    fn debug_pick_point_sphere_radius(&self) -> &f32 {
        &self.debug_pick_point_sphere_radius
    }
    fn debug_pick_point_normal_length(&self) -> &f32 {
        &self.debug_pick_point_normal_length
    }
    fn debug_grab_hand_square_size(&self) -> &f32 {
        &self.debug_grab_hand_square_size
    }
    fn debug_color_grab_hand(&self) -> &super::core::Vec4 {
        &self.debug_color_grab_hand
    }
    fn debug_color_grab_target_position(&self) -> &super::core::Vec4 {
        &self.debug_color_grab_target_position
    }
    fn debug_color_grab_on_server(&self) -> &super::core::Vec4 {
        &self.debug_color_grab_on_server
    }
    fn debug_color_grab_on_client(&self) -> &super::core::Vec4 {
        &self.debug_color_grab_on_client
    }
    fn debug_color_grab_on_effect(&self) -> &super::core::Vec4 {
        &self.debug_color_grab_on_effect
    }
}

impl super::core::DataContainerTrait for IglooGrabToolSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static IGLOOGRABTOOLSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IglooGrabToolSettings",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IglooGrabToolSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MaxGrabDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(IglooGrabToolSettings, max_grab_distance),
            },
            FieldInfoData {
                name: "DebugPickPointSphereRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(IglooGrabToolSettings, debug_pick_point_sphere_radius),
            },
            FieldInfoData {
                name: "DebugPickPointNormalLength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(IglooGrabToolSettings, debug_pick_point_normal_length),
            },
            FieldInfoData {
                name: "DebugGrabHandSquareSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(IglooGrabToolSettings, debug_grab_hand_square_size),
            },
            FieldInfoData {
                name: "DebugColorGrabHand",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(IglooGrabToolSettings, debug_color_grab_hand),
            },
            FieldInfoData {
                name: "DebugColorGrabTargetPosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(IglooGrabToolSettings, debug_color_grab_target_position),
            },
            FieldInfoData {
                name: "DebugColorGrabOnServer",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(IglooGrabToolSettings, debug_color_grab_on_server),
            },
            FieldInfoData {
                name: "DebugColorGrabOnClient",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(IglooGrabToolSettings, debug_color_grab_on_client),
            },
            FieldInfoData {
                name: "DebugColorGrabOnEffect",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(IglooGrabToolSettings, debug_color_grab_on_effect),
            },
        ],
    }),
    array_type: Some(IGLOOGRABTOOLSETTINGS_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for IglooGrabToolSettings {
    fn type_info(&self) -> &'static TypeInfo {
        IGLOOGRABTOOLSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static IGLOOGRABTOOLSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IglooGrabToolSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("IglooGrabToolSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsRenderWorldDynamicState {
    pub part_enabled: Vec<bool>,
    pub add_to_world: bool,
    pub character_pose: CharacterPoseType,
    pub transform: super::core::LinearTransform,
    pub field_flag_changed0: u8,
}

pub trait PhysicsRenderWorldDynamicStateTrait: TypeObject {
    fn part_enabled(&self) -> &Vec<bool>;
    fn add_to_world(&self) -> &bool;
    fn character_pose(&self) -> &CharacterPoseType;
    fn transform(&self) -> &super::core::LinearTransform;
    fn field_flag_changed0(&self) -> &u8;
}

impl PhysicsRenderWorldDynamicStateTrait for PhysicsRenderWorldDynamicState {
    fn part_enabled(&self) -> &Vec<bool> {
        &self.part_enabled
    }
    fn add_to_world(&self) -> &bool {
        &self.add_to_world
    }
    fn character_pose(&self) -> &CharacterPoseType {
        &self.character_pose
    }
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static PHYSICSRENDERWORLDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRenderWorldDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "Physics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsRenderWorldDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "PartEnabled",
                flags: MemberInfoFlags::new(144),
                field_type: "Boolean-Array",
                rust_offset: offset_of!(PhysicsRenderWorldDynamicState, part_enabled),
            },
            FieldInfoData {
                name: "AddToWorld",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicsRenderWorldDynamicState, add_to_world),
            },
            FieldInfoData {
                name: "CharacterPose",
                flags: MemberInfoFlags::new(0),
                field_type: "CharacterPoseType",
                rust_offset: offset_of!(PhysicsRenderWorldDynamicState, character_pose),
            },
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(PhysicsRenderWorldDynamicState, transform),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(PhysicsRenderWorldDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(PHYSICSRENDERWORLDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsRenderWorldDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSRENDERWORLDDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSRENDERWORLDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRenderWorldDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsRenderWorldDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsRenderWorldStaticState {
    pub physics_component_data: Option<Arc<Mutex<dyn PhysicsComponentDataTrait>>>,
    pub physics_body_data: Option<Arc<Mutex<dyn PhysicsBodyDataTrait>>>,
    pub body_transform_fixups: Vec<super::core::LinearTransform>,
    pub transform_space: super::state_stream::TransformSpaceHandle,
    pub field_flag_changed0: u8,
}

pub trait PhysicsRenderWorldStaticStateTrait: TypeObject {
    fn physics_component_data(&self) -> &Option<Arc<Mutex<dyn PhysicsComponentDataTrait>>>;
    fn physics_body_data(&self) -> &Option<Arc<Mutex<dyn PhysicsBodyDataTrait>>>;
    fn body_transform_fixups(&self) -> &Vec<super::core::LinearTransform>;
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle;
    fn field_flag_changed0(&self) -> &u8;
}

impl PhysicsRenderWorldStaticStateTrait for PhysicsRenderWorldStaticState {
    fn physics_component_data(&self) -> &Option<Arc<Mutex<dyn PhysicsComponentDataTrait>>> {
        &self.physics_component_data
    }
    fn physics_body_data(&self) -> &Option<Arc<Mutex<dyn PhysicsBodyDataTrait>>> {
        &self.physics_body_data
    }
    fn body_transform_fixups(&self) -> &Vec<super::core::LinearTransform> {
        &self.body_transform_fixups
    }
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle {
        &self.transform_space
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static PHYSICSRENDERWORLDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRenderWorldStaticState",
    flags: MemberInfoFlags::new(73),
    module: "Physics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsRenderWorldStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "physicsComponentData",
                flags: MemberInfoFlags::new(0),
                field_type: "PhysicsComponentData",
                rust_offset: offset_of!(PhysicsRenderWorldStaticState, physics_component_data),
            },
            FieldInfoData {
                name: "physicsBodyData",
                flags: MemberInfoFlags::new(0),
                field_type: "PhysicsBodyData",
                rust_offset: offset_of!(PhysicsRenderWorldStaticState, physics_body_data),
            },
            FieldInfoData {
                name: "bodyTransformFixups",
                flags: MemberInfoFlags::new(144),
                field_type: "LinearTransform-Array",
                rust_offset: offset_of!(PhysicsRenderWorldStaticState, body_transform_fixups),
            },
            FieldInfoData {
                name: "TransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: "TransformSpaceHandle",
                rust_offset: offset_of!(PhysicsRenderWorldStaticState, transform_space),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(PhysicsRenderWorldStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(PHYSICSRENDERWORLDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PhysicsRenderWorldStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSRENDERWORLDSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSRENDERWORLDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRenderWorldStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsRenderWorldStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsRenderWorldHandle {
}

pub trait PhysicsRenderWorldHandleTrait: TypeObject {
}

impl PhysicsRenderWorldHandleTrait for PhysicsRenderWorldHandle {
}

pub static PHYSICSRENDERWORLDHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRenderWorldHandle",
    flags: MemberInfoFlags::new(73),
    module: "Physics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsRenderWorldHandle as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSRENDERWORLDHANDLE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for PhysicsRenderWorldHandle {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSRENDERWORLDHANDLE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSRENDERWORLDHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsRenderWorldHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsRenderWorldHandle"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsCategorySet {
    pub categories: u32,
}

pub trait PhysicsCategorySetTrait: TypeObject {
    fn categories(&self) -> &u32;
}

impl PhysicsCategorySetTrait for PhysicsCategorySet {
    fn categories(&self) -> &u32 {
        &self.categories
    }
}

pub static PHYSICSCATEGORYSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsCategorySet",
    flags: MemberInfoFlags::new(36937),
    module: "Physics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsCategorySet as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Categories",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(PhysicsCategorySet, categories),
            },
        ],
    }),
    array_type: Some(PHYSICSCATEGORYSET_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for PhysicsCategorySet {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSCATEGORYSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSCATEGORYSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsCategorySet-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsCategorySet"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FBPhysicsWorldData {
    pub _glacier_base: super::entity::SubWorldDataComponent,
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

pub trait FBPhysicsWorldDataTrait: super::entity::SubWorldDataComponentTrait {
    fn max_body_count(&self) -> &u32;
    fn max_heightfield_count(&self) -> &u32;
    fn max_non_fixed_shape_count(&self) -> &u32;
    fn max_fixed_shape_count(&self) -> &u32;
    fn max_shape_pairs(&self) -> &u32;
    fn max_shape_pair_batches(&self) -> &u32;
    fn shape_pair_index_count(&self) -> &u32;
    fn max_contact_count(&self) -> &u32;
    fn max_ray_query_count(&self) -> &u32;
    fn max_ray_query_result_count(&self) -> &u32;
    fn max_directional_drive_count(&self) -> &u32;
    fn max_temporary_directional_drive_count(&self) -> &u32;
    fn max_locked_linear_joint_count(&self) -> &u32;
    fn max_temporary_locked_linear_joint_count(&self) -> &u32;
    fn max_angular_joint_count(&self) -> &u32;
    fn max_temporary_angular_joint_count(&self) -> &u32;
    fn max_vertex_count(&self) -> &u32;
    fn max_edge_count(&self) -> &u32;
    fn max_fixed_partition_count(&self) -> &u32;
    fn max_collision_report_count(&self) -> &u32;
    fn max_immediate_query_count(&self) -> &u32;
    fn max_results_per_immediate_query(&self) -> &u32;
    fn max_culling_tables(&self) -> &u32;
    fn max_culling_group_pairs(&self) -> &u32;
    fn max_culling_table_size(&self) -> &u32;
    fn gravity(&self) -> &super::core::Vec3;
    fn max_g_j_k_iterations(&self) -> &u32;
    fn max_solve_iterations(&self) -> &u32;
}

impl FBPhysicsWorldDataTrait for FBPhysicsWorldData {
    fn max_body_count(&self) -> &u32 {
        &self.max_body_count
    }
    fn max_heightfield_count(&self) -> &u32 {
        &self.max_heightfield_count
    }
    fn max_non_fixed_shape_count(&self) -> &u32 {
        &self.max_non_fixed_shape_count
    }
    fn max_fixed_shape_count(&self) -> &u32 {
        &self.max_fixed_shape_count
    }
    fn max_shape_pairs(&self) -> &u32 {
        &self.max_shape_pairs
    }
    fn max_shape_pair_batches(&self) -> &u32 {
        &self.max_shape_pair_batches
    }
    fn shape_pair_index_count(&self) -> &u32 {
        &self.shape_pair_index_count
    }
    fn max_contact_count(&self) -> &u32 {
        &self.max_contact_count
    }
    fn max_ray_query_count(&self) -> &u32 {
        &self.max_ray_query_count
    }
    fn max_ray_query_result_count(&self) -> &u32 {
        &self.max_ray_query_result_count
    }
    fn max_directional_drive_count(&self) -> &u32 {
        &self.max_directional_drive_count
    }
    fn max_temporary_directional_drive_count(&self) -> &u32 {
        &self.max_temporary_directional_drive_count
    }
    fn max_locked_linear_joint_count(&self) -> &u32 {
        &self.max_locked_linear_joint_count
    }
    fn max_temporary_locked_linear_joint_count(&self) -> &u32 {
        &self.max_temporary_locked_linear_joint_count
    }
    fn max_angular_joint_count(&self) -> &u32 {
        &self.max_angular_joint_count
    }
    fn max_temporary_angular_joint_count(&self) -> &u32 {
        &self.max_temporary_angular_joint_count
    }
    fn max_vertex_count(&self) -> &u32 {
        &self.max_vertex_count
    }
    fn max_edge_count(&self) -> &u32 {
        &self.max_edge_count
    }
    fn max_fixed_partition_count(&self) -> &u32 {
        &self.max_fixed_partition_count
    }
    fn max_collision_report_count(&self) -> &u32 {
        &self.max_collision_report_count
    }
    fn max_immediate_query_count(&self) -> &u32 {
        &self.max_immediate_query_count
    }
    fn max_results_per_immediate_query(&self) -> &u32 {
        &self.max_results_per_immediate_query
    }
    fn max_culling_tables(&self) -> &u32 {
        &self.max_culling_tables
    }
    fn max_culling_group_pairs(&self) -> &u32 {
        &self.max_culling_group_pairs
    }
    fn max_culling_table_size(&self) -> &u32 {
        &self.max_culling_table_size
    }
    fn gravity(&self) -> &super::core::Vec3 {
        &self.gravity
    }
    fn max_g_j_k_iterations(&self) -> &u32 {
        &self.max_g_j_k_iterations
    }
    fn max_solve_iterations(&self) -> &u32 {
        &self.max_solve_iterations
    }
}

impl super::entity::SubWorldDataComponentTrait for FBPhysicsWorldData {
}

impl super::core::DataContainerTrait for FBPhysicsWorldData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static FBPHYSICSWORLDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBPhysicsWorldData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SUBWORLDDATACOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FBPhysicsWorldData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MaxBodyCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FBPhysicsWorldData, max_body_count),
            },
            FieldInfoData {
                name: "MaxHeightfieldCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FBPhysicsWorldData, max_heightfield_count),
            },
            FieldInfoData {
                name: "MaxNonFixedShapeCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FBPhysicsWorldData, max_non_fixed_shape_count),
            },
            FieldInfoData {
                name: "MaxFixedShapeCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FBPhysicsWorldData, max_fixed_shape_count),
            },
            FieldInfoData {
                name: "MaxShapePairs",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FBPhysicsWorldData, max_shape_pairs),
            },
            FieldInfoData {
                name: "MaxShapePairBatches",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FBPhysicsWorldData, max_shape_pair_batches),
            },
            FieldInfoData {
                name: "ShapePairIndexCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FBPhysicsWorldData, shape_pair_index_count),
            },
            FieldInfoData {
                name: "MaxContactCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FBPhysicsWorldData, max_contact_count),
            },
            FieldInfoData {
                name: "MaxRayQueryCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FBPhysicsWorldData, max_ray_query_count),
            },
            FieldInfoData {
                name: "MaxRayQueryResultCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FBPhysicsWorldData, max_ray_query_result_count),
            },
            FieldInfoData {
                name: "MaxDirectionalDriveCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FBPhysicsWorldData, max_directional_drive_count),
            },
            FieldInfoData {
                name: "MaxTemporaryDirectionalDriveCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FBPhysicsWorldData, max_temporary_directional_drive_count),
            },
            FieldInfoData {
                name: "MaxLockedLinearJointCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FBPhysicsWorldData, max_locked_linear_joint_count),
            },
            FieldInfoData {
                name: "MaxTemporaryLockedLinearJointCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FBPhysicsWorldData, max_temporary_locked_linear_joint_count),
            },
            FieldInfoData {
                name: "MaxAngularJointCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FBPhysicsWorldData, max_angular_joint_count),
            },
            FieldInfoData {
                name: "MaxTemporaryAngularJointCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FBPhysicsWorldData, max_temporary_angular_joint_count),
            },
            FieldInfoData {
                name: "MaxVertexCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FBPhysicsWorldData, max_vertex_count),
            },
            FieldInfoData {
                name: "MaxEdgeCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FBPhysicsWorldData, max_edge_count),
            },
            FieldInfoData {
                name: "MaxFixedPartitionCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FBPhysicsWorldData, max_fixed_partition_count),
            },
            FieldInfoData {
                name: "MaxCollisionReportCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FBPhysicsWorldData, max_collision_report_count),
            },
            FieldInfoData {
                name: "MaxImmediateQueryCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FBPhysicsWorldData, max_immediate_query_count),
            },
            FieldInfoData {
                name: "MaxResultsPerImmediateQuery",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FBPhysicsWorldData, max_results_per_immediate_query),
            },
            FieldInfoData {
                name: "MaxCullingTables",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FBPhysicsWorldData, max_culling_tables),
            },
            FieldInfoData {
                name: "MaxCullingGroupPairs",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FBPhysicsWorldData, max_culling_group_pairs),
            },
            FieldInfoData {
                name: "MaxCullingTableSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FBPhysicsWorldData, max_culling_table_size),
            },
            FieldInfoData {
                name: "Gravity",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(FBPhysicsWorldData, gravity),
            },
            FieldInfoData {
                name: "MaxGJKIterations",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FBPhysicsWorldData, max_g_j_k_iterations),
            },
            FieldInfoData {
                name: "MaxSolveIterations",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FBPhysicsWorldData, max_solve_iterations),
            },
        ],
    }),
    array_type: Some(FBPHYSICSWORLDDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FBPhysicsWorldData {
    fn type_info(&self) -> &'static TypeInfo {
        FBPHYSICSWORLDDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FBPHYSICSWORLDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBPhysicsWorldData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("FBPhysicsWorldData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FBPhysicsSubLevelData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub server_fixed_shape_count: u32,
    pub client_fixed_shape_count: u32,
}

pub trait FBPhysicsSubLevelDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn server_fixed_shape_count(&self) -> &u32;
    fn client_fixed_shape_count(&self) -> &u32;
}

impl FBPhysicsSubLevelDataTrait for FBPhysicsSubLevelData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn server_fixed_shape_count(&self) -> &u32 {
        &self.server_fixed_shape_count
    }
    fn client_fixed_shape_count(&self) -> &u32 {
        &self.client_fixed_shape_count
    }
}

impl super::entity::EntityDataTrait for FBPhysicsSubLevelData {
}

impl super::entity::GameObjectDataTrait for FBPhysicsSubLevelData {
}

impl super::core::DataBusPeerTrait for FBPhysicsSubLevelData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for FBPhysicsSubLevelData {
}

impl super::core::DataContainerTrait for FBPhysicsSubLevelData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static FBPHYSICSSUBLEVELDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBPhysicsSubLevelData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FBPhysicsSubLevelData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(FBPhysicsSubLevelData, realm),
            },
            FieldInfoData {
                name: "ServerFixedShapeCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FBPhysicsSubLevelData, server_fixed_shape_count),
            },
            FieldInfoData {
                name: "ClientFixedShapeCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FBPhysicsSubLevelData, client_fixed_shape_count),
            },
        ],
    }),
    array_type: Some(FBPHYSICSSUBLEVELDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FBPhysicsSubLevelData {
    fn type_info(&self) -> &'static TypeInfo {
        FBPHYSICSSUBLEVELDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FBPHYSICSSUBLEVELDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBPhysicsSubLevelData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("FBPhysicsSubLevelData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static PHYSICSCOLLISIONCATEGORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsCollisionCategory",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(PHYSICSCOLLISIONCATEGORY_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PhysicsCollisionCategory {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSCOLLISIONCATEGORY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSCOLLISIONCATEGORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsCollisionCategory-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsCollisionCategory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FBPhysicsComponentData {
    pub _glacier_base: super::entity::ComponentData,
    pub realm: super::core::Realm,
    pub bones: Vec<PhysicsBoneData>,
    pub asset: Option<Arc<Mutex<dyn PhysicsAssetTrait>>>,
}

pub trait FBPhysicsComponentDataTrait: super::entity::ComponentDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn bones(&self) -> &Vec<PhysicsBoneData>;
    fn asset(&self) -> &Option<Arc<Mutex<dyn PhysicsAssetTrait>>>;
}

impl FBPhysicsComponentDataTrait for FBPhysicsComponentData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn bones(&self) -> &Vec<PhysicsBoneData> {
        &self.bones
    }
    fn asset(&self) -> &Option<Arc<Mutex<dyn PhysicsAssetTrait>>> {
        &self.asset
    }
}

impl super::entity::ComponentDataTrait for FBPhysicsComponentData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn components(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components()
    }
    fn client_index(&self) -> &u8 {
        self._glacier_base.client_index()
    }
    fn server_index(&self) -> &u8 {
        self._glacier_base.server_index()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
}

impl super::entity::GameObjectDataTrait for FBPhysicsComponentData {
}

impl super::core::DataBusPeerTrait for FBPhysicsComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for FBPhysicsComponentData {
}

impl super::core::DataContainerTrait for FBPhysicsComponentData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static FBPHYSICSCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBPhysicsComponentData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::COMPONENTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FBPhysicsComponentData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(FBPhysicsComponentData, realm),
            },
            FieldInfoData {
                name: "Bones",
                flags: MemberInfoFlags::new(144),
                field_type: "PhysicsBoneData-Array",
                rust_offset: offset_of!(FBPhysicsComponentData, bones),
            },
            FieldInfoData {
                name: "Asset",
                flags: MemberInfoFlags::new(0),
                field_type: "PhysicsAsset",
                rust_offset: offset_of!(FBPhysicsComponentData, asset),
            },
        ],
    }),
    array_type: Some(FBPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FBPhysicsComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        FBPHYSICSCOMPONENTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FBPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBPhysicsComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("FBPhysicsComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsAsset {
    pub _glacier_base: super::core::Asset,
    pub realm: super::core::Realm,
    pub resource: glacier_reflect::builtin::ResourceRef,
    pub locked_linear_joints: Vec<Option<Arc<Mutex<dyn FBPhysicsLockedLinearJointDataTrait>>>>,
    pub angular_joints: Vec<Option<Arc<Mutex<dyn FBPhysicsAngularJointDataTrait>>>>,
    pub directional_drives: Vec<Option<Arc<Mutex<dyn FBPhysicsDirectionalDriveDataTrait>>>>,
    pub culling_tables: Vec<Option<Arc<Mutex<dyn PhysicsCullingTableDataTrait>>>>,
}

pub trait PhysicsAssetTrait: super::core::AssetTrait {
    fn realm(&self) -> &super::core::Realm;
    fn resource(&self) -> &glacier_reflect::builtin::ResourceRef;
    fn locked_linear_joints(&self) -> &Vec<Option<Arc<Mutex<dyn FBPhysicsLockedLinearJointDataTrait>>>>;
    fn angular_joints(&self) -> &Vec<Option<Arc<Mutex<dyn FBPhysicsAngularJointDataTrait>>>>;
    fn directional_drives(&self) -> &Vec<Option<Arc<Mutex<dyn FBPhysicsDirectionalDriveDataTrait>>>>;
    fn culling_tables(&self) -> &Vec<Option<Arc<Mutex<dyn PhysicsCullingTableDataTrait>>>>;
}

impl PhysicsAssetTrait for PhysicsAsset {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.resource
    }
    fn locked_linear_joints(&self) -> &Vec<Option<Arc<Mutex<dyn FBPhysicsLockedLinearJointDataTrait>>>> {
        &self.locked_linear_joints
    }
    fn angular_joints(&self) -> &Vec<Option<Arc<Mutex<dyn FBPhysicsAngularJointDataTrait>>>> {
        &self.angular_joints
    }
    fn directional_drives(&self) -> &Vec<Option<Arc<Mutex<dyn FBPhysicsDirectionalDriveDataTrait>>>> {
        &self.directional_drives
    }
    fn culling_tables(&self) -> &Vec<Option<Arc<Mutex<dyn PhysicsCullingTableDataTrait>>>> {
        &self.culling_tables
    }
}

impl super::core::AssetTrait for PhysicsAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for PhysicsAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PHYSICSASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsAsset",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(PhysicsAsset, realm),
            },
            FieldInfoData {
                name: "Resource",
                flags: MemberInfoFlags::new(0),
                field_type: "ResourceRef",
                rust_offset: offset_of!(PhysicsAsset, resource),
            },
            FieldInfoData {
                name: "LockedLinearJoints",
                flags: MemberInfoFlags::new(144),
                field_type: "FBPhysicsLockedLinearJointData-Array",
                rust_offset: offset_of!(PhysicsAsset, locked_linear_joints),
            },
            FieldInfoData {
                name: "AngularJoints",
                flags: MemberInfoFlags::new(144),
                field_type: "FBPhysicsAngularJointData-Array",
                rust_offset: offset_of!(PhysicsAsset, angular_joints),
            },
            FieldInfoData {
                name: "DirectionalDrives",
                flags: MemberInfoFlags::new(144),
                field_type: "FBPhysicsDirectionalDriveData-Array",
                rust_offset: offset_of!(PhysicsAsset, directional_drives),
            },
            FieldInfoData {
                name: "CullingTables",
                flags: MemberInfoFlags::new(144),
                field_type: "PhysicsCullingTableData-Array",
                rust_offset: offset_of!(PhysicsAsset, culling_tables),
            },
        ],
    }),
    array_type: Some(PHYSICSASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PhysicsAsset {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsCullingTableData {
    pub _glacier_base: super::core::DataContainer,
    pub entries: Vec<bool>,
    pub group_a_size: u32,
    pub group_b_size: u32,
}

pub trait PhysicsCullingTableDataTrait: super::core::DataContainerTrait {
    fn entries(&self) -> &Vec<bool>;
    fn group_a_size(&self) -> &u32;
    fn group_b_size(&self) -> &u32;
}

impl PhysicsCullingTableDataTrait for PhysicsCullingTableData {
    fn entries(&self) -> &Vec<bool> {
        &self.entries
    }
    fn group_a_size(&self) -> &u32 {
        &self.group_a_size
    }
    fn group_b_size(&self) -> &u32 {
        &self.group_b_size
    }
}

impl super::core::DataContainerTrait for PhysicsCullingTableData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PHYSICSCULLINGTABLEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsCullingTableData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsCullingTableData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Entries",
                flags: MemberInfoFlags::new(144),
                field_type: "Boolean-Array",
                rust_offset: offset_of!(PhysicsCullingTableData, entries),
            },
            FieldInfoData {
                name: "GroupASize",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(PhysicsCullingTableData, group_a_size),
            },
            FieldInfoData {
                name: "GroupBSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(PhysicsCullingTableData, group_b_size),
            },
        ],
    }),
    array_type: Some(PHYSICSCULLINGTABLEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PhysicsCullingTableData {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSCULLINGTABLEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSCULLINGTABLEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsCullingTableData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsCullingTableData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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

pub trait FBPhysicsShapeDataTrait: TypeObject {
    fn convex_hull_index(&self) -> &u32;
    fn position(&self) -> &super::core::Vec3;
    fn orientation(&self) -> &super::core::Quat;
    fn shape_type(&self) -> &PhysicsShapeType;
    fn radius(&self) -> &f32;
    fn body_index_in_physics_component(&self) -> &u32;
    fn part_index(&self) -> &u32;
    fn in_categories(&self) -> &PhysicsCategorySet;
    fn collides_with_categories(&self) -> &PhysicsCategorySet;
    fn material(&self) -> &super::entity::MaterialDecl;
}

impl FBPhysicsShapeDataTrait for FBPhysicsShapeData {
    fn convex_hull_index(&self) -> &u32 {
        &self.convex_hull_index
    }
    fn position(&self) -> &super::core::Vec3 {
        &self.position
    }
    fn orientation(&self) -> &super::core::Quat {
        &self.orientation
    }
    fn shape_type(&self) -> &PhysicsShapeType {
        &self.shape_type
    }
    fn radius(&self) -> &f32 {
        &self.radius
    }
    fn body_index_in_physics_component(&self) -> &u32 {
        &self.body_index_in_physics_component
    }
    fn part_index(&self) -> &u32 {
        &self.part_index
    }
    fn in_categories(&self) -> &PhysicsCategorySet {
        &self.in_categories
    }
    fn collides_with_categories(&self) -> &PhysicsCategorySet {
        &self.collides_with_categories
    }
    fn material(&self) -> &super::entity::MaterialDecl {
        &self.material
    }
}

pub static FBPHYSICSSHAPEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBPhysicsShapeData",
    flags: MemberInfoFlags::new(32841),
    module: "Physics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FBPhysicsShapeData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ConvexHullIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FBPhysicsShapeData, convex_hull_index),
            },
            FieldInfoData {
                name: "Position",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(FBPhysicsShapeData, position),
            },
            FieldInfoData {
                name: "Orientation",
                flags: MemberInfoFlags::new(0),
                field_type: "Quat",
                rust_offset: offset_of!(FBPhysicsShapeData, orientation),
            },
            FieldInfoData {
                name: "ShapeType",
                flags: MemberInfoFlags::new(0),
                field_type: "PhysicsShapeType",
                rust_offset: offset_of!(FBPhysicsShapeData, shape_type),
            },
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FBPhysicsShapeData, radius),
            },
            FieldInfoData {
                name: "BodyIndexInPhysicsComponent",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FBPhysicsShapeData, body_index_in_physics_component),
            },
            FieldInfoData {
                name: "PartIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FBPhysicsShapeData, part_index),
            },
            FieldInfoData {
                name: "InCategories",
                flags: MemberInfoFlags::new(0),
                field_type: "PhysicsCategorySet",
                rust_offset: offset_of!(FBPhysicsShapeData, in_categories),
            },
            FieldInfoData {
                name: "CollidesWithCategories",
                flags: MemberInfoFlags::new(0),
                field_type: "PhysicsCategorySet",
                rust_offset: offset_of!(FBPhysicsShapeData, collides_with_categories),
            },
            FieldInfoData {
                name: "Material",
                flags: MemberInfoFlags::new(0),
                field_type: "MaterialDecl",
                rust_offset: offset_of!(FBPhysicsShapeData, material),
            },
        ],
    }),
    array_type: Some(FBPHYSICSSHAPEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FBPhysicsShapeData {
    fn type_info(&self) -> &'static TypeInfo {
        FBPHYSICSSHAPEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FBPHYSICSSHAPEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBPhysicsShapeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("FBPhysicsShapeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FBPhysicsConvexHullData {
    pub positions: Vec<super::core::Vec3>,
    pub end_vertices: Vec<u32>,
    pub edge_cosines: Vec<f32>,
    pub next_edge_in_faces: Vec<u32>,
}

pub trait FBPhysicsConvexHullDataTrait: TypeObject {
    fn positions(&self) -> &Vec<super::core::Vec3>;
    fn end_vertices(&self) -> &Vec<u32>;
    fn edge_cosines(&self) -> &Vec<f32>;
    fn next_edge_in_faces(&self) -> &Vec<u32>;
}

impl FBPhysicsConvexHullDataTrait for FBPhysicsConvexHullData {
    fn positions(&self) -> &Vec<super::core::Vec3> {
        &self.positions
    }
    fn end_vertices(&self) -> &Vec<u32> {
        &self.end_vertices
    }
    fn edge_cosines(&self) -> &Vec<f32> {
        &self.edge_cosines
    }
    fn next_edge_in_faces(&self) -> &Vec<u32> {
        &self.next_edge_in_faces
    }
}

pub static FBPHYSICSCONVEXHULLDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBPhysicsConvexHullData",
    flags: MemberInfoFlags::new(73),
    module: "Physics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FBPhysicsConvexHullData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Positions",
                flags: MemberInfoFlags::new(144),
                field_type: "Vec3-Array",
                rust_offset: offset_of!(FBPhysicsConvexHullData, positions),
            },
            FieldInfoData {
                name: "EndVertices",
                flags: MemberInfoFlags::new(144),
                field_type: "Uint32-Array",
                rust_offset: offset_of!(FBPhysicsConvexHullData, end_vertices),
            },
            FieldInfoData {
                name: "EdgeCosines",
                flags: MemberInfoFlags::new(144),
                field_type: "Float32-Array",
                rust_offset: offset_of!(FBPhysicsConvexHullData, edge_cosines),
            },
            FieldInfoData {
                name: "NextEdgeInFaces",
                flags: MemberInfoFlags::new(144),
                field_type: "Uint32-Array",
                rust_offset: offset_of!(FBPhysicsConvexHullData, next_edge_in_faces),
            },
        ],
    }),
    array_type: Some(FBPHYSICSCONVEXHULLDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FBPhysicsConvexHullData {
    fn type_info(&self) -> &'static TypeInfo {
        FBPHYSICSCONVEXHULLDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FBPHYSICSCONVEXHULLDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBPhysicsConvexHullData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("FBPhysicsConvexHullData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FBPhysicsDirectionalDriveData {
    pub _glacier_base: super::core::DataContainer,
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

pub trait FBPhysicsDirectionalDriveDataTrait: super::core::DataContainerTrait {
    fn body_a_index_in_component(&self) -> &u32;
    fn body_b_index_in_component(&self) -> &u32;
    fn position_on_body_a(&self) -> &super::core::Vec3;
    fn position_on_body_b(&self) -> &super::core::Vec3;
    fn orientation_on_body_a(&self) -> &super::core::Quat;
    fn orientation_on_body_b(&self) -> &super::core::Quat;
    fn angular_spring(&self) -> &f32;
    fn angular_damping(&self) -> &f32;
    fn linear_spring(&self) -> &f32;
    fn linear_damping(&self) -> &f32;
    fn angular_limit_axis_space(&self) -> &PhysicsConstraintLimitSpace;
    fn angular_limit_axis(&self) -> &super::core::Vec3;
    fn swing_torque_limit(&self) -> &f32;
    fn twist_torque_limit(&self) -> &f32;
    fn linear_limit_axis_space(&self) -> &PhysicsConstraintLimitSpace;
    fn linear_limit_axis(&self) -> &super::core::Vec3;
    fn forward_force_limit(&self) -> &f32;
    fn backward_force_limit(&self) -> &f32;
    fn radial_force_limit(&self) -> &f32;
}

impl FBPhysicsDirectionalDriveDataTrait for FBPhysicsDirectionalDriveData {
    fn body_a_index_in_component(&self) -> &u32 {
        &self.body_a_index_in_component
    }
    fn body_b_index_in_component(&self) -> &u32 {
        &self.body_b_index_in_component
    }
    fn position_on_body_a(&self) -> &super::core::Vec3 {
        &self.position_on_body_a
    }
    fn position_on_body_b(&self) -> &super::core::Vec3 {
        &self.position_on_body_b
    }
    fn orientation_on_body_a(&self) -> &super::core::Quat {
        &self.orientation_on_body_a
    }
    fn orientation_on_body_b(&self) -> &super::core::Quat {
        &self.orientation_on_body_b
    }
    fn angular_spring(&self) -> &f32 {
        &self.angular_spring
    }
    fn angular_damping(&self) -> &f32 {
        &self.angular_damping
    }
    fn linear_spring(&self) -> &f32 {
        &self.linear_spring
    }
    fn linear_damping(&self) -> &f32 {
        &self.linear_damping
    }
    fn angular_limit_axis_space(&self) -> &PhysicsConstraintLimitSpace {
        &self.angular_limit_axis_space
    }
    fn angular_limit_axis(&self) -> &super::core::Vec3 {
        &self.angular_limit_axis
    }
    fn swing_torque_limit(&self) -> &f32 {
        &self.swing_torque_limit
    }
    fn twist_torque_limit(&self) -> &f32 {
        &self.twist_torque_limit
    }
    fn linear_limit_axis_space(&self) -> &PhysicsConstraintLimitSpace {
        &self.linear_limit_axis_space
    }
    fn linear_limit_axis(&self) -> &super::core::Vec3 {
        &self.linear_limit_axis
    }
    fn forward_force_limit(&self) -> &f32 {
        &self.forward_force_limit
    }
    fn backward_force_limit(&self) -> &f32 {
        &self.backward_force_limit
    }
    fn radial_force_limit(&self) -> &f32 {
        &self.radial_force_limit
    }
}

impl super::core::DataContainerTrait for FBPhysicsDirectionalDriveData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static FBPHYSICSDIRECTIONALDRIVEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBPhysicsDirectionalDriveData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FBPhysicsDirectionalDriveData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "BodyAIndexInComponent",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FBPhysicsDirectionalDriveData, body_a_index_in_component),
            },
            FieldInfoData {
                name: "BodyBIndexInComponent",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FBPhysicsDirectionalDriveData, body_b_index_in_component),
            },
            FieldInfoData {
                name: "PositionOnBodyA",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(FBPhysicsDirectionalDriveData, position_on_body_a),
            },
            FieldInfoData {
                name: "PositionOnBodyB",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(FBPhysicsDirectionalDriveData, position_on_body_b),
            },
            FieldInfoData {
                name: "OrientationOnBodyA",
                flags: MemberInfoFlags::new(0),
                field_type: "Quat",
                rust_offset: offset_of!(FBPhysicsDirectionalDriveData, orientation_on_body_a),
            },
            FieldInfoData {
                name: "OrientationOnBodyB",
                flags: MemberInfoFlags::new(0),
                field_type: "Quat",
                rust_offset: offset_of!(FBPhysicsDirectionalDriveData, orientation_on_body_b),
            },
            FieldInfoData {
                name: "AngularSpring",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FBPhysicsDirectionalDriveData, angular_spring),
            },
            FieldInfoData {
                name: "AngularDamping",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FBPhysicsDirectionalDriveData, angular_damping),
            },
            FieldInfoData {
                name: "LinearSpring",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FBPhysicsDirectionalDriveData, linear_spring),
            },
            FieldInfoData {
                name: "LinearDamping",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FBPhysicsDirectionalDriveData, linear_damping),
            },
            FieldInfoData {
                name: "AngularLimitAxisSpace",
                flags: MemberInfoFlags::new(0),
                field_type: "PhysicsConstraintLimitSpace",
                rust_offset: offset_of!(FBPhysicsDirectionalDriveData, angular_limit_axis_space),
            },
            FieldInfoData {
                name: "AngularLimitAxis",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(FBPhysicsDirectionalDriveData, angular_limit_axis),
            },
            FieldInfoData {
                name: "SwingTorqueLimit",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FBPhysicsDirectionalDriveData, swing_torque_limit),
            },
            FieldInfoData {
                name: "TwistTorqueLimit",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FBPhysicsDirectionalDriveData, twist_torque_limit),
            },
            FieldInfoData {
                name: "LinearLimitAxisSpace",
                flags: MemberInfoFlags::new(0),
                field_type: "PhysicsConstraintLimitSpace",
                rust_offset: offset_of!(FBPhysicsDirectionalDriveData, linear_limit_axis_space),
            },
            FieldInfoData {
                name: "LinearLimitAxis",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(FBPhysicsDirectionalDriveData, linear_limit_axis),
            },
            FieldInfoData {
                name: "ForwardForceLimit",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FBPhysicsDirectionalDriveData, forward_force_limit),
            },
            FieldInfoData {
                name: "BackwardForceLimit",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FBPhysicsDirectionalDriveData, backward_force_limit),
            },
            FieldInfoData {
                name: "RadialForceLimit",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FBPhysicsDirectionalDriveData, radial_force_limit),
            },
        ],
    }),
    array_type: Some(FBPHYSICSDIRECTIONALDRIVEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FBPhysicsDirectionalDriveData {
    fn type_info(&self) -> &'static TypeInfo {
        FBPHYSICSDIRECTIONALDRIVEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FBPHYSICSDIRECTIONALDRIVEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBPhysicsDirectionalDriveData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("FBPhysicsDirectionalDriveData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FBPhysicsAngularJointData {
    pub _glacier_base: super::core::DataContainer,
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

pub trait FBPhysicsAngularJointDataTrait: super::core::DataContainerTrait {
    fn body_a_index_in_component(&self) -> &u32;
    fn body_b_index_in_component(&self) -> &u32;
    fn swing_spring(&self) -> &f32;
    fn swing_damping(&self) -> &f32;
    fn twist_spring(&self) -> &f32;
    fn twist_damping(&self) -> &f32;
    fn swing_limit_y(&self) -> &f32;
    fn swing_limit_z(&self) -> &f32;
    fn twist_limit(&self) -> &f32;
    fn soft_swing_limit_y(&self) -> &f32;
    fn soft_swing_limit_z(&self) -> &f32;
    fn soft_twist_limit(&self) -> &f32;
    fn swing_velocity_limit(&self) -> &f32;
    fn twist_velocity_limit(&self) -> &f32;
    fn orientation_on_body_a(&self) -> &super::core::Quat;
    fn orientation_on_body_b(&self) -> &super::core::Quat;
    fn swing_type(&self) -> &PhysicsAngularJointSwingType;
    fn twist_type(&self) -> &PhysicsAngularJointTwistType;
}

impl FBPhysicsAngularJointDataTrait for FBPhysicsAngularJointData {
    fn body_a_index_in_component(&self) -> &u32 {
        &self.body_a_index_in_component
    }
    fn body_b_index_in_component(&self) -> &u32 {
        &self.body_b_index_in_component
    }
    fn swing_spring(&self) -> &f32 {
        &self.swing_spring
    }
    fn swing_damping(&self) -> &f32 {
        &self.swing_damping
    }
    fn twist_spring(&self) -> &f32 {
        &self.twist_spring
    }
    fn twist_damping(&self) -> &f32 {
        &self.twist_damping
    }
    fn swing_limit_y(&self) -> &f32 {
        &self.swing_limit_y
    }
    fn swing_limit_z(&self) -> &f32 {
        &self.swing_limit_z
    }
    fn twist_limit(&self) -> &f32 {
        &self.twist_limit
    }
    fn soft_swing_limit_y(&self) -> &f32 {
        &self.soft_swing_limit_y
    }
    fn soft_swing_limit_z(&self) -> &f32 {
        &self.soft_swing_limit_z
    }
    fn soft_twist_limit(&self) -> &f32 {
        &self.soft_twist_limit
    }
    fn swing_velocity_limit(&self) -> &f32 {
        &self.swing_velocity_limit
    }
    fn twist_velocity_limit(&self) -> &f32 {
        &self.twist_velocity_limit
    }
    fn orientation_on_body_a(&self) -> &super::core::Quat {
        &self.orientation_on_body_a
    }
    fn orientation_on_body_b(&self) -> &super::core::Quat {
        &self.orientation_on_body_b
    }
    fn swing_type(&self) -> &PhysicsAngularJointSwingType {
        &self.swing_type
    }
    fn twist_type(&self) -> &PhysicsAngularJointTwistType {
        &self.twist_type
    }
}

impl super::core::DataContainerTrait for FBPhysicsAngularJointData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static FBPHYSICSANGULARJOINTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBPhysicsAngularJointData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FBPhysicsAngularJointData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "BodyAIndexInComponent",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FBPhysicsAngularJointData, body_a_index_in_component),
            },
            FieldInfoData {
                name: "BodyBIndexInComponent",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FBPhysicsAngularJointData, body_b_index_in_component),
            },
            FieldInfoData {
                name: "SwingSpring",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FBPhysicsAngularJointData, swing_spring),
            },
            FieldInfoData {
                name: "SwingDamping",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FBPhysicsAngularJointData, swing_damping),
            },
            FieldInfoData {
                name: "TwistSpring",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FBPhysicsAngularJointData, twist_spring),
            },
            FieldInfoData {
                name: "TwistDamping",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FBPhysicsAngularJointData, twist_damping),
            },
            FieldInfoData {
                name: "SwingLimitY",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FBPhysicsAngularJointData, swing_limit_y),
            },
            FieldInfoData {
                name: "SwingLimitZ",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FBPhysicsAngularJointData, swing_limit_z),
            },
            FieldInfoData {
                name: "TwistLimit",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FBPhysicsAngularJointData, twist_limit),
            },
            FieldInfoData {
                name: "SoftSwingLimitY",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FBPhysicsAngularJointData, soft_swing_limit_y),
            },
            FieldInfoData {
                name: "SoftSwingLimitZ",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FBPhysicsAngularJointData, soft_swing_limit_z),
            },
            FieldInfoData {
                name: "SoftTwistLimit",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FBPhysicsAngularJointData, soft_twist_limit),
            },
            FieldInfoData {
                name: "SwingVelocityLimit",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FBPhysicsAngularJointData, swing_velocity_limit),
            },
            FieldInfoData {
                name: "TwistVelocityLimit",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FBPhysicsAngularJointData, twist_velocity_limit),
            },
            FieldInfoData {
                name: "OrientationOnBodyA",
                flags: MemberInfoFlags::new(0),
                field_type: "Quat",
                rust_offset: offset_of!(FBPhysicsAngularJointData, orientation_on_body_a),
            },
            FieldInfoData {
                name: "OrientationOnBodyB",
                flags: MemberInfoFlags::new(0),
                field_type: "Quat",
                rust_offset: offset_of!(FBPhysicsAngularJointData, orientation_on_body_b),
            },
            FieldInfoData {
                name: "SwingType",
                flags: MemberInfoFlags::new(0),
                field_type: "PhysicsAngularJointSwingType",
                rust_offset: offset_of!(FBPhysicsAngularJointData, swing_type),
            },
            FieldInfoData {
                name: "TwistType",
                flags: MemberInfoFlags::new(0),
                field_type: "PhysicsAngularJointTwistType",
                rust_offset: offset_of!(FBPhysicsAngularJointData, twist_type),
            },
        ],
    }),
    array_type: Some(FBPHYSICSANGULARJOINTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FBPhysicsAngularJointData {
    fn type_info(&self) -> &'static TypeInfo {
        FBPHYSICSANGULARJOINTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FBPHYSICSANGULARJOINTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBPhysicsAngularJointData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("FBPhysicsAngularJointData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FBPhysicsLockedLinearJointData {
    pub _glacier_base: super::core::DataContainer,
    pub body_a_index_in_component: u32,
    pub body_b_index_in_component: u32,
    pub position_on_body_a: super::core::Vec3,
    pub position_on_body_b: super::core::Vec3,
}

pub trait FBPhysicsLockedLinearJointDataTrait: super::core::DataContainerTrait {
    fn body_a_index_in_component(&self) -> &u32;
    fn body_b_index_in_component(&self) -> &u32;
    fn position_on_body_a(&self) -> &super::core::Vec3;
    fn position_on_body_b(&self) -> &super::core::Vec3;
}

impl FBPhysicsLockedLinearJointDataTrait for FBPhysicsLockedLinearJointData {
    fn body_a_index_in_component(&self) -> &u32 {
        &self.body_a_index_in_component
    }
    fn body_b_index_in_component(&self) -> &u32 {
        &self.body_b_index_in_component
    }
    fn position_on_body_a(&self) -> &super::core::Vec3 {
        &self.position_on_body_a
    }
    fn position_on_body_b(&self) -> &super::core::Vec3 {
        &self.position_on_body_b
    }
}

impl super::core::DataContainerTrait for FBPhysicsLockedLinearJointData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static FBPHYSICSLOCKEDLINEARJOINTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBPhysicsLockedLinearJointData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FBPhysicsLockedLinearJointData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "BodyAIndexInComponent",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FBPhysicsLockedLinearJointData, body_a_index_in_component),
            },
            FieldInfoData {
                name: "BodyBIndexInComponent",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FBPhysicsLockedLinearJointData, body_b_index_in_component),
            },
            FieldInfoData {
                name: "PositionOnBodyA",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(FBPhysicsLockedLinearJointData, position_on_body_a),
            },
            FieldInfoData {
                name: "PositionOnBodyB",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(FBPhysicsLockedLinearJointData, position_on_body_b),
            },
        ],
    }),
    array_type: Some(FBPHYSICSLOCKEDLINEARJOINTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FBPhysicsLockedLinearJointData {
    fn type_info(&self) -> &'static TypeInfo {
        FBPHYSICSLOCKEDLINEARJOINTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FBPHYSICSLOCKEDLINEARJOINTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBPhysicsLockedLinearJointData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("FBPhysicsLockedLinearJointData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FBPhysicsBodyData {
    pub motion_type: RigidBodyMotionType,
    pub reciprocal_mass: f32,
    pub principal_axes_of_inertia: super::core::Quat,
    pub unit_mass_inverse_inertia: super::core::Vec3,
    pub center_of_mass_offset: super::core::Vec3,
}

pub trait FBPhysicsBodyDataTrait: TypeObject {
    fn motion_type(&self) -> &RigidBodyMotionType;
    fn reciprocal_mass(&self) -> &f32;
    fn principal_axes_of_inertia(&self) -> &super::core::Quat;
    fn unit_mass_inverse_inertia(&self) -> &super::core::Vec3;
    fn center_of_mass_offset(&self) -> &super::core::Vec3;
}

impl FBPhysicsBodyDataTrait for FBPhysicsBodyData {
    fn motion_type(&self) -> &RigidBodyMotionType {
        &self.motion_type
    }
    fn reciprocal_mass(&self) -> &f32 {
        &self.reciprocal_mass
    }
    fn principal_axes_of_inertia(&self) -> &super::core::Quat {
        &self.principal_axes_of_inertia
    }
    fn unit_mass_inverse_inertia(&self) -> &super::core::Vec3 {
        &self.unit_mass_inverse_inertia
    }
    fn center_of_mass_offset(&self) -> &super::core::Vec3 {
        &self.center_of_mass_offset
    }
}

pub static FBPHYSICSBODYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBPhysicsBodyData",
    flags: MemberInfoFlags::new(32841),
    module: "Physics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FBPhysicsBodyData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MotionType",
                flags: MemberInfoFlags::new(0),
                field_type: "RigidBodyMotionType",
                rust_offset: offset_of!(FBPhysicsBodyData, motion_type),
            },
            FieldInfoData {
                name: "ReciprocalMass",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FBPhysicsBodyData, reciprocal_mass),
            },
            FieldInfoData {
                name: "PrincipalAxesOfInertia",
                flags: MemberInfoFlags::new(0),
                field_type: "Quat",
                rust_offset: offset_of!(FBPhysicsBodyData, principal_axes_of_inertia),
            },
            FieldInfoData {
                name: "UnitMassInverseInertia",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(FBPhysicsBodyData, unit_mass_inverse_inertia),
            },
            FieldInfoData {
                name: "CenterOfMassOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(FBPhysicsBodyData, center_of_mass_offset),
            },
        ],
    }),
    array_type: Some(FBPHYSICSBODYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FBPhysicsBodyData {
    fn type_info(&self) -> &'static TypeInfo {
        FBPHYSICSBODYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FBPHYSICSBODYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBPhysicsBodyData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("FBPhysicsBodyData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsBoneData {
    pub bone: Option<Arc<Mutex<dyn super::entity::ComponentDataTrait>>>,
    pub body_index: u32,
}

pub trait PhysicsBoneDataTrait: TypeObject {
    fn bone(&self) -> &Option<Arc<Mutex<dyn super::entity::ComponentDataTrait>>>;
    fn body_index(&self) -> &u32;
}

impl PhysicsBoneDataTrait for PhysicsBoneData {
    fn bone(&self) -> &Option<Arc<Mutex<dyn super::entity::ComponentDataTrait>>> {
        &self.bone
    }
    fn body_index(&self) -> &u32 {
        &self.body_index
    }
}

pub static PHYSICSBONEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsBoneData",
    flags: MemberInfoFlags::new(73),
    module: "Physics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsBoneData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Bone",
                flags: MemberInfoFlags::new(0),
                field_type: "ComponentData",
                rust_offset: offset_of!(PhysicsBoneData, bone),
            },
            FieldInfoData {
                name: "BodyIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(PhysicsBoneData, body_index),
            },
        ],
    }),
    array_type: Some(PHYSICSBONEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PhysicsBoneData {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSBONEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSBONEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsBoneData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsBoneData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsTriangleArrayData {
    pub _glacier_base: super::core::DataContainer,
    pub triangles: Vec<PhysicsTriangle>,
}

pub trait PhysicsTriangleArrayDataTrait: super::core::DataContainerTrait {
    fn triangles(&self) -> &Vec<PhysicsTriangle>;
}

impl PhysicsTriangleArrayDataTrait for PhysicsTriangleArrayData {
    fn triangles(&self) -> &Vec<PhysicsTriangle> {
        &self.triangles
    }
}

impl super::core::DataContainerTrait for PhysicsTriangleArrayData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PHYSICSTRIANGLEARRAYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsTriangleArrayData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsTriangleArrayData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Triangles",
                flags: MemberInfoFlags::new(144),
                field_type: "PhysicsTriangle-Array",
                rust_offset: offset_of!(PhysicsTriangleArrayData, triangles),
            },
        ],
    }),
    array_type: Some(PHYSICSTRIANGLEARRAYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PhysicsTriangleArrayData {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSTRIANGLEARRAYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSTRIANGLEARRAYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsTriangleArrayData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsTriangleArrayData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsTriangle {
    pub p0: super::core::Vec3,
    pub p1: super::core::Vec3,
    pub p2: super::core::Vec3,
}

pub trait PhysicsTriangleTrait: TypeObject {
    fn p0(&self) -> &super::core::Vec3;
    fn p1(&self) -> &super::core::Vec3;
    fn p2(&self) -> &super::core::Vec3;
}

impl PhysicsTriangleTrait for PhysicsTriangle {
    fn p0(&self) -> &super::core::Vec3 {
        &self.p0
    }
    fn p1(&self) -> &super::core::Vec3 {
        &self.p1
    }
    fn p2(&self) -> &super::core::Vec3 {
        &self.p2
    }
}

pub static PHYSICSTRIANGLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsTriangle",
    flags: MemberInfoFlags::new(36937),
    module: "Physics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsTriangle as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "P0",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(PhysicsTriangle, p0),
            },
            FieldInfoData {
                name: "P1",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(PhysicsTriangle, p1),
            },
            FieldInfoData {
                name: "P2",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(PhysicsTriangle, p2),
            },
        ],
    }),
    array_type: Some(PHYSICSTRIANGLE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsTriangle {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSTRIANGLE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSTRIANGLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsTriangle-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsTriangle"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum PhysicsConstraintLimitSpace {
    #[default]
    PhysicsConstraintLimitSpace_World = 0,
    PhysicsConstraintLimitSpace_BodyA = 1,
    PhysicsConstraintLimitSpace_BodyB = 2,
}

pub static PHYSICSCONSTRAINTLIMITSPACE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsConstraintLimitSpace",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(PHYSICSCONSTRAINTLIMITSPACE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PhysicsConstraintLimitSpace {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSCONSTRAINTLIMITSPACE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSCONSTRAINTLIMITSPACE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsConstraintLimitSpace-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsConstraintLimitSpace"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum PhysicsAngularJointSwingType {
    #[default]
    PhysicsAngularJointSwing_Locked = 0,
    PhysicsAngularJointSwing_Cone = 1,
    PhysicsAngularJointSwing_Hinge = 2,
    PhysicsAngularJointSwing_Axle = 3,
    PhysicsAngularJointSwing_Free = 4,
    PhysicsAngularJointSwing_Ellipse = 5,
}

pub static PHYSICSANGULARJOINTSWINGTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsAngularJointSwingType",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(PHYSICSANGULARJOINTSWINGTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PhysicsAngularJointSwingType {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSANGULARJOINTSWINGTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSANGULARJOINTSWINGTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsAngularJointSwingType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsAngularJointSwingType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum PhysicsAngularJointTwistType {
    #[default]
    PhysicsAngularJointTwist_Locked = 0,
    PhysicsAngularJointTwist_Arc = 1,
    PhysicsAngularJointTwist_Free = 2,
}

pub static PHYSICSANGULARJOINTTWISTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsAngularJointTwistType",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(PHYSICSANGULARJOINTTWISTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PhysicsAngularJointTwistType {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSANGULARJOINTTWISTTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSANGULARJOINTTWISTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsAngularJointTwistType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsAngularJointTwistType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum PhysicsShapeType {
    #[default]
    PhysicsShapeType_Sphere = 0,
    PhysicsShapeType_Capsule = 1,
    PhysicsShapeType_Box = 2,
    PhysicsShapeType_ConvexHull = 3,
    PhysicsShapeType_Cylinder = 4,
    PhysicsShapeType_Heightfield = 5,
}

pub static PHYSICSSHAPETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsShapeType",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(PHYSICSSHAPETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PhysicsShapeType {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSSHAPETYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSSHAPETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsShapeType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsShapeType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum PhysicsComponentConstants {
    #[default]
    PhysicsComponentConstants_UseWorldBody = 2147483647,
}

pub static PHYSICSCOMPONENTCONSTANTS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsComponentConstants",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(PHYSICSCOMPONENTCONSTANTS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PhysicsComponentConstants {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSCOMPONENTCONSTANTS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSCOMPONENTCONSTANTS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsComponentConstants-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsComponentConstants"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct HavokMemoryContext {
    pub _glacier_base: IglooMemoryContext,
}

pub trait HavokMemoryContextTrait: IglooMemoryContextTrait {
}

impl HavokMemoryContextTrait for HavokMemoryContext {
}

impl IglooMemoryContextTrait for HavokMemoryContext {
}

pub static HAVOKMEMORYCONTEXT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HavokMemoryContext",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IGLOOMEMORYCONTEXT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<HavokMemoryContext as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(HAVOKMEMORYCONTEXT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for HavokMemoryContext {
    fn type_info(&self) -> &'static TypeInfo {
        HAVOKMEMORYCONTEXT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static HAVOKMEMORYCONTEXT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HavokMemoryContext-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("HavokMemoryContext"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WindPhysicsAction {
    pub _glacier_base: PhysicsAction,
}

pub trait WindPhysicsActionTrait: PhysicsActionTrait {
}

impl WindPhysicsActionTrait for WindPhysicsAction {
}

impl PhysicsActionTrait for WindPhysicsAction {
}

impl super::entity::EntityTrait for WindPhysicsAction {
}

impl super::entity::EntityBusPeerTrait for WindPhysicsAction {
}

pub static WINDPHYSICSACTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindPhysicsAction",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSACTION_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WindPhysicsAction as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(WINDPHYSICSACTION_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for WindPhysicsAction {
    fn type_info(&self) -> &'static TypeInfo {
        WINDPHYSICSACTION_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WINDPHYSICSACTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindPhysicsAction-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("WindPhysicsAction"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientDestructionInfo {
    pub _glacier_base: DestructionInfo,
}

pub trait ClientDestructionInfoTrait: DestructionInfoTrait {
}

impl ClientDestructionInfoTrait for ClientDestructionInfo {
}

impl DestructionInfoTrait for ClientDestructionInfo {
}

pub static CLIENTDESTRUCTIONINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDestructionInfo",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DESTRUCTIONINFO_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientDestructionInfo as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTDESTRUCTIONINFO_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientDestructionInfo {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTDESTRUCTIONINFO_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTDESTRUCTIONINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDestructionInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ClientDestructionInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerDestructionInfo {
    pub _glacier_base: DestructionInfo,
}

pub trait ServerDestructionInfoTrait: DestructionInfoTrait {
}

impl ServerDestructionInfoTrait for ServerDestructionInfo {
}

impl DestructionInfoTrait for ServerDestructionInfo {
}

pub static SERVERDESTRUCTIONINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDestructionInfo",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DESTRUCTIONINFO_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerDestructionInfo as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERDESTRUCTIONINFO_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerDestructionInfo {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERDESTRUCTIONINFO_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERDESTRUCTIONINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDestructionInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ServerDestructionInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DestructionInfo {
}

pub trait DestructionInfoTrait: TypeObject {
}

impl DestructionInfoTrait for DestructionInfo {
}

pub static DESTRUCTIONINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionInfo",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DestructionInfo as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DESTRUCTIONINFO_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DestructionInfo {
    fn type_info(&self) -> &'static TypeInfo {
        DESTRUCTIONINFO_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DESTRUCTIONINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("DestructionInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DestructionPhysicsComponent {
    pub _glacier_base: PhysicsComponent,
}

pub trait DestructionPhysicsComponentTrait: PhysicsComponentTrait {
}

impl DestructionPhysicsComponentTrait for DestructionPhysicsComponent {
}

impl PhysicsComponentTrait for DestructionPhysicsComponent {
}

impl super::entity::ComponentTrait for DestructionPhysicsComponent {
}

impl super::entity::EntityBusPeerTrait for DestructionPhysicsComponent {
}

pub static DESTRUCTIONPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DestructionPhysicsComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DESTRUCTIONPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DestructionPhysicsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        DESTRUCTIONPHYSICSCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DESTRUCTIONPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("DestructionPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DestructionEntityFactory {
    pub _glacier_base: IglooEntityFactory,
}

pub trait DestructionEntityFactoryTrait: IglooEntityFactoryTrait {
}

impl DestructionEntityFactoryTrait for DestructionEntityFactory {
}

impl IglooEntityFactoryTrait for DestructionEntityFactory {
}

pub static DESTRUCTIONENTITYFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionEntityFactory",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IGLOOENTITYFACTORY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DestructionEntityFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DESTRUCTIONENTITYFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DestructionEntityFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DESTRUCTIONENTITYFACTORY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DESTRUCTIONENTITYFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionEntityFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("DestructionEntityFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct IglooSubsystem {
}

pub trait IglooSubsystemTrait: TypeObject {
}

impl IglooSubsystemTrait for IglooSubsystem {
}

pub static IGLOOSUBSYSTEM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IglooSubsystem",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IglooSubsystem as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(IGLOOSUBSYSTEM_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IglooSubsystem {
    fn type_info(&self) -> &'static TypeInfo {
        IGLOOSUBSYSTEM_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static IGLOOSUBSYSTEM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IglooSubsystem-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("IglooSubsystem"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsProxyEntity {
    pub _glacier_base: super::entity::ComponentEntity,
}

pub trait PhysicsProxyEntityTrait: super::entity::ComponentEntityTrait {
}

impl PhysicsProxyEntityTrait for PhysicsProxyEntity {
}

impl super::entity::ComponentEntityTrait for PhysicsProxyEntity {
}

impl super::entity::SpatialEntityTrait for PhysicsProxyEntity {
}

impl super::entity::EntityTrait for PhysicsProxyEntity {
}

impl super::entity::EntityBusPeerTrait for PhysicsProxyEntity {
}

pub static PHYSICSPROXYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsProxyEntity",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::COMPONENTENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsProxyEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSPROXYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PhysicsProxyEntity {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSPROXYENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSPROXYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsProxyEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsProxyEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsComponent {
    pub _glacier_base: super::entity::Component,
}

pub trait PhysicsComponentTrait: super::entity::ComponentTrait {
}

impl PhysicsComponentTrait for PhysicsComponent {
}

impl super::entity::ComponentTrait for PhysicsComponent {
}

impl super::entity::EntityBusPeerTrait for PhysicsComponent {
}

pub static PHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::COMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PhysicsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MultiCollisionEvent {
    pub _glacier_base: super::entity::EntityEvent,
}

pub trait MultiCollisionEventTrait: super::entity::EntityEventTrait {
}

impl MultiCollisionEventTrait for MultiCollisionEvent {
}

impl super::entity::EntityEventTrait for MultiCollisionEvent {
}

pub static MULTICOLLISIONEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiCollisionEvent",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MultiCollisionEvent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(MULTICOLLISIONEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MultiCollisionEvent {
    fn type_info(&self) -> &'static TypeInfo {
        MULTICOLLISIONEVENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MULTICOLLISIONEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiCollisionEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("MultiCollisionEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PreSolveCollisionInfo {
    pub _glacier_base: CollisionInfo,
}

pub trait PreSolveCollisionInfoTrait: CollisionInfoTrait {
}

impl PreSolveCollisionInfoTrait for PreSolveCollisionInfo {
}

impl CollisionInfoTrait for PreSolveCollisionInfo {
}

pub static PRESOLVECOLLISIONINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PreSolveCollisionInfo",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COLLISIONINFO_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PreSolveCollisionInfo as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESOLVECOLLISIONINFO_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PreSolveCollisionInfo {
    fn type_info(&self) -> &'static TypeInfo {
        PRESOLVECOLLISIONINFO_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PRESOLVECOLLISIONINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PreSolveCollisionInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PreSolveCollisionInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerShockwaveCollisionInfo {
    pub _glacier_base: ShockwaveCollisionInfo,
}

pub trait ServerShockwaveCollisionInfoTrait: ShockwaveCollisionInfoTrait {
}

impl ServerShockwaveCollisionInfoTrait for ServerShockwaveCollisionInfo {
}

impl ShockwaveCollisionInfoTrait for ServerShockwaveCollisionInfo {
}

pub static SERVERSHOCKWAVECOLLISIONINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerShockwaveCollisionInfo",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SHOCKWAVECOLLISIONINFO_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerShockwaveCollisionInfo as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSHOCKWAVECOLLISIONINFO_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerShockwaveCollisionInfo {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSHOCKWAVECOLLISIONINFO_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERSHOCKWAVECOLLISIONINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerShockwaveCollisionInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ServerShockwaveCollisionInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientShockwaveCollisionInfo {
    pub _glacier_base: ShockwaveCollisionInfo,
}

pub trait ClientShockwaveCollisionInfoTrait: ShockwaveCollisionInfoTrait {
}

impl ClientShockwaveCollisionInfoTrait for ClientShockwaveCollisionInfo {
}

impl ShockwaveCollisionInfoTrait for ClientShockwaveCollisionInfo {
}

pub static CLIENTSHOCKWAVECOLLISIONINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientShockwaveCollisionInfo",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SHOCKWAVECOLLISIONINFO_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientShockwaveCollisionInfo as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSHOCKWAVECOLLISIONINFO_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientShockwaveCollisionInfo {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSHOCKWAVECOLLISIONINFO_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTSHOCKWAVECOLLISIONINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientShockwaveCollisionInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ClientShockwaveCollisionInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ShockwaveCollisionInfo {
}

pub trait ShockwaveCollisionInfoTrait: TypeObject {
}

impl ShockwaveCollisionInfoTrait for ShockwaveCollisionInfo {
}

pub static SHOCKWAVECOLLISIONINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShockwaveCollisionInfo",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ShockwaveCollisionInfo as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SHOCKWAVECOLLISIONINFO_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ShockwaveCollisionInfo {
    fn type_info(&self) -> &'static TypeInfo {
        SHOCKWAVECOLLISIONINFO_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SHOCKWAVECOLLISIONINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShockwaveCollisionInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ShockwaveCollisionInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerFakeCollisionInfo {
    pub _glacier_base: FakeCollisionInfo,
}

pub trait ServerFakeCollisionInfoTrait: FakeCollisionInfoTrait {
}

impl ServerFakeCollisionInfoTrait for ServerFakeCollisionInfo {
}

impl FakeCollisionInfoTrait for ServerFakeCollisionInfo {
}

pub static SERVERFAKECOLLISIONINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerFakeCollisionInfo",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(FAKECOLLISIONINFO_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerFakeCollisionInfo as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERFAKECOLLISIONINFO_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerFakeCollisionInfo {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERFAKECOLLISIONINFO_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERFAKECOLLISIONINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerFakeCollisionInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ServerFakeCollisionInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientFakeCollisionInfo {
    pub _glacier_base: FakeCollisionInfo,
}

pub trait ClientFakeCollisionInfoTrait: FakeCollisionInfoTrait {
}

impl ClientFakeCollisionInfoTrait for ClientFakeCollisionInfo {
}

impl FakeCollisionInfoTrait for ClientFakeCollisionInfo {
}

pub static CLIENTFAKECOLLISIONINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientFakeCollisionInfo",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(FAKECOLLISIONINFO_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientFakeCollisionInfo as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTFAKECOLLISIONINFO_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientFakeCollisionInfo {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTFAKECOLLISIONINFO_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTFAKECOLLISIONINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientFakeCollisionInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ClientFakeCollisionInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FakeCollisionInfo {
}

pub trait FakeCollisionInfoTrait: TypeObject {
}

impl FakeCollisionInfoTrait for FakeCollisionInfo {
}

pub static FAKECOLLISIONINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FakeCollisionInfo",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FakeCollisionInfo as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(FAKECOLLISIONINFO_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FakeCollisionInfo {
    fn type_info(&self) -> &'static TypeInfo {
        FAKECOLLISIONINFO_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FAKECOLLISIONINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FakeCollisionInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("FakeCollisionInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CollisionInfo {
}

pub trait CollisionInfoTrait: TypeObject {
}

impl CollisionInfoTrait for CollisionInfo {
}

pub static COLLISIONINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CollisionInfo",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CollisionInfo as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(COLLISIONINFO_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CollisionInfo {
    fn type_info(&self) -> &'static TypeInfo {
        COLLISIONINFO_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static COLLISIONINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CollisionInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("CollisionInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PartPhysicsComponent {
    pub _glacier_base: PhysicsComponent,
}

pub trait PartPhysicsComponentTrait: PhysicsComponentTrait {
}

impl PartPhysicsComponentTrait for PartPhysicsComponent {
}

impl PhysicsComponentTrait for PartPhysicsComponent {
}

impl super::entity::ComponentTrait for PartPhysicsComponent {
}

impl super::entity::EntityBusPeerTrait for PartPhysicsComponent {
}

pub static PARTPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PartPhysicsComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PARTPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PartPhysicsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        PARTPHYSICSCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PARTPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PartPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct GroupPhysicsComponent {
    pub _glacier_base: PhysicsComponent,
}

pub trait GroupPhysicsComponentTrait: PhysicsComponentTrait {
}

impl GroupPhysicsComponentTrait for GroupPhysicsComponent {
}

impl PhysicsComponentTrait for GroupPhysicsComponent {
}

impl super::entity::ComponentTrait for GroupPhysicsComponent {
}

impl super::entity::EntityBusPeerTrait for GroupPhysicsComponent {
}

pub static GROUPPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroupPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GroupPhysicsComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(GROUPPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for GroupPhysicsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        GROUPPHYSICSCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static GROUPPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroupPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("GroupPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ForceComponent {
    pub _glacier_base: super::entity::Component,
}

pub trait ForceComponentTrait: super::entity::ComponentTrait {
}

impl ForceComponentTrait for ForceComponent {
}

impl super::entity::ComponentTrait for ForceComponent {
}

impl super::entity::EntityBusPeerTrait for ForceComponent {
}

pub static FORCECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForceComponent",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::COMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ForceComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(FORCECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ForceComponent {
    fn type_info(&self) -> &'static TypeInfo {
        FORCECOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FORCECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForceComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("ForceComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DefaultPartPhysicsComponent {
    pub _glacier_base: PartPhysicsComponent,
}

pub trait DefaultPartPhysicsComponentTrait: PartPhysicsComponentTrait {
}

impl DefaultPartPhysicsComponentTrait for DefaultPartPhysicsComponent {
}

impl PartPhysicsComponentTrait for DefaultPartPhysicsComponent {
}

impl PhysicsComponentTrait for DefaultPartPhysicsComponent {
}

impl super::entity::ComponentTrait for DefaultPartPhysicsComponent {
}

impl super::entity::EntityBusPeerTrait for DefaultPartPhysicsComponent {
}

pub static DEFAULTPARTPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DefaultPartPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PARTPHYSICSCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DefaultPartPhysicsComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DEFAULTPARTPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DefaultPartPhysicsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        DEFAULTPARTPHYSICSCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DEFAULTPARTPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DefaultPartPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("DefaultPartPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CharacterPhysicsComponent {
    pub _glacier_base: PhysicsComponent,
}

pub trait CharacterPhysicsComponentTrait: PhysicsComponentTrait {
}

impl CharacterPhysicsComponentTrait for CharacterPhysicsComponent {
}

impl PhysicsComponentTrait for CharacterPhysicsComponent {
}

impl super::entity::ComponentTrait for CharacterPhysicsComponent {
}

impl super::entity::EntityBusPeerTrait for CharacterPhysicsComponent {
}

pub static CHARACTERPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CharacterPhysicsComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CHARACTERPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CharacterPhysicsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CHARACTERPHYSICSCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CHARACTERPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("CharacterPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsConstraintOwner {
    pub _glacier_base: super::entity::Entity,
}

pub trait PhysicsConstraintOwnerTrait: super::entity::EntityTrait {
}

impl PhysicsConstraintOwnerTrait for PhysicsConstraintOwner {
}

impl super::entity::EntityTrait for PhysicsConstraintOwner {
}

impl super::entity::EntityBusPeerTrait for PhysicsConstraintOwner {
}

pub static PHYSICSCONSTRAINTOWNER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsConstraintOwner",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsConstraintOwner as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSCONSTRAINTOWNER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PhysicsConstraintOwner {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSCONSTRAINTOWNER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSCONSTRAINTOWNER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsConstraintOwner-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsConstraintOwner"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsConstraint {
    pub _glacier_base: super::entity::Entity,
}

pub trait PhysicsConstraintTrait: super::entity::EntityTrait {
}

impl PhysicsConstraintTrait for PhysicsConstraint {
}

impl super::entity::EntityTrait for PhysicsConstraint {
}

impl super::entity::EntityBusPeerTrait for PhysicsConstraint {
}

pub static PHYSICSCONSTRAINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsConstraint",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsConstraint as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSCONSTRAINT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PhysicsConstraint {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSCONSTRAINT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSCONSTRAINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsConstraint-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsConstraint"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct StaticWaterPhysicsBody {
    pub _glacier_base: WaterPhysicsBody,
}

pub trait StaticWaterPhysicsBodyTrait: WaterPhysicsBodyTrait {
}

impl StaticWaterPhysicsBodyTrait for StaticWaterPhysicsBody {
}

impl WaterPhysicsBodyTrait for StaticWaterPhysicsBody {
}

impl PhysicsBodyTrait for StaticWaterPhysicsBody {
}

impl super::entity::EntityTrait for StaticWaterPhysicsBody {
}

impl super::entity::EntityBusPeerTrait for StaticWaterPhysicsBody {
}

pub static STATICWATERPHYSICSBODY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticWaterPhysicsBody",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WATERPHYSICSBODY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StaticWaterPhysicsBody as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(STATICWATERPHYSICSBODY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for StaticWaterPhysicsBody {
    fn type_info(&self) -> &'static TypeInfo {
        STATICWATERPHYSICSBODY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static STATICWATERPHYSICSBODY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticWaterPhysicsBody-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("StaticWaterPhysicsBody"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsBody {
    pub _glacier_base: super::entity::Entity,
}

pub trait PhysicsBodyTrait: super::entity::EntityTrait {
}

impl PhysicsBodyTrait for PhysicsBody {
}

impl super::entity::EntityTrait for PhysicsBody {
}

impl super::entity::EntityBusPeerTrait for PhysicsBody {
}

pub static PHYSICSBODY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsBody",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsBody as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSBODY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PhysicsBody {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSBODY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSBODY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsBody-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsBody"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsAction {
    pub _glacier_base: super::entity::Entity,
}

pub trait PhysicsActionTrait: super::entity::EntityTrait {
}

impl PhysicsActionTrait for PhysicsAction {
}

impl super::entity::EntityTrait for PhysicsAction {
}

impl super::entity::EntityBusPeerTrait for PhysicsAction {
}

pub static PHYSICSACTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsAction",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsAction as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSACTION_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PhysicsAction {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSACTION_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSACTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsAction-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsAction"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FloatPhysicsAction {
    pub _glacier_base: PhysicsAction,
}

pub trait FloatPhysicsActionTrait: PhysicsActionTrait {
}

impl FloatPhysicsActionTrait for FloatPhysicsAction {
}

impl PhysicsActionTrait for FloatPhysicsAction {
}

impl super::entity::EntityTrait for FloatPhysicsAction {
}

impl super::entity::EntityBusPeerTrait for FloatPhysicsAction {
}

pub static FLOATPHYSICSACTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatPhysicsAction",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSACTION_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FloatPhysicsAction as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(FLOATPHYSICSACTION_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FloatPhysicsAction {
    fn type_info(&self) -> &'static TypeInfo {
        FLOATPHYSICSACTION_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FLOATPHYSICSACTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatPhysicsAction-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("FloatPhysicsAction"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FloatPhysicsActionFactory {
    pub _glacier_base: IglooEntityFactory,
}

pub trait FloatPhysicsActionFactoryTrait: IglooEntityFactoryTrait {
}

impl FloatPhysicsActionFactoryTrait for FloatPhysicsActionFactory {
}

impl IglooEntityFactoryTrait for FloatPhysicsActionFactory {
}

pub static FLOATPHYSICSACTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatPhysicsActionFactory",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IGLOOENTITYFACTORY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FloatPhysicsActionFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(FLOATPHYSICSACTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FloatPhysicsActionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        FLOATPHYSICSACTIONFACTORY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FLOATPHYSICSACTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatPhysicsActionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("FloatPhysicsActionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RagdollPhysicsComponent {
    pub _glacier_base: PhysicsComponent,
}

pub trait RagdollPhysicsComponentTrait: PhysicsComponentTrait {
}

impl RagdollPhysicsComponentTrait for RagdollPhysicsComponent {
}

impl PhysicsComponentTrait for RagdollPhysicsComponent {
}

impl super::entity::ComponentTrait for RagdollPhysicsComponent {
}

impl super::entity::EntityBusPeerTrait for RagdollPhysicsComponent {
}

pub static RAGDOLLPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RagdollPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RagdollPhysicsComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(RAGDOLLPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RagdollPhysicsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        RAGDOLLPHYSICSCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RAGDOLLPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RagdollPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("RagdollPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WaterPhysicsBody {
    pub _glacier_base: PhysicsBody,
}

pub trait WaterPhysicsBodyTrait: PhysicsBodyTrait {
}

impl WaterPhysicsBodyTrait for WaterPhysicsBody {
}

impl PhysicsBodyTrait for WaterPhysicsBody {
}

impl super::entity::EntityTrait for WaterPhysicsBody {
}

impl super::entity::EntityBusPeerTrait for WaterPhysicsBody {
}

pub static WATERPHYSICSBODY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterPhysicsBody",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSBODY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaterPhysicsBody as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(WATERPHYSICSBODY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for WaterPhysicsBody {
    fn type_info(&self) -> &'static TypeInfo {
        WATERPHYSICSBODY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WATERPHYSICSBODY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterPhysicsBody-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("WaterPhysicsBody"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TerrainPhysicsBody {
    pub _glacier_base: PhysicsBody,
}

pub trait TerrainPhysicsBodyTrait: PhysicsBodyTrait {
}

impl TerrainPhysicsBodyTrait for TerrainPhysicsBody {
}

impl PhysicsBodyTrait for TerrainPhysicsBody {
}

impl super::entity::EntityTrait for TerrainPhysicsBody {
}

impl super::entity::EntityBusPeerTrait for TerrainPhysicsBody {
}

pub static TERRAINPHYSICSBODY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainPhysicsBody",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSBODY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainPhysicsBody as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TERRAINPHYSICSBODY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TerrainPhysicsBody {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINPHYSICSBODY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAINPHYSICSBODY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainPhysicsBody-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("TerrainPhysicsBody"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RigidBody {
    pub _glacier_base: PhysicsBody,
}

pub trait RigidBodyTrait: PhysicsBodyTrait {
}

impl RigidBodyTrait for RigidBody {
}

impl PhysicsBodyTrait for RigidBody {
}

impl super::entity::EntityTrait for RigidBody {
}

impl super::entity::EntityBusPeerTrait for RigidBody {
}

pub static RIGIDBODY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RigidBody",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSBODY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RigidBody as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(RIGIDBODY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RigidBody {
    fn type_info(&self) -> &'static TypeInfo {
        RIGIDBODY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RIGIDBODY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RigidBody-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("RigidBody"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct GroupRigidBody {
    pub _glacier_base: PhysicsBody,
}

pub trait GroupRigidBodyTrait: PhysicsBodyTrait {
}

impl GroupRigidBodyTrait for GroupRigidBody {
}

impl PhysicsBodyTrait for GroupRigidBody {
}

impl super::entity::EntityTrait for GroupRigidBody {
}

impl super::entity::EntityBusPeerTrait for GroupRigidBody {
}

pub static GROUPRIGIDBODY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroupRigidBody",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSBODY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GroupRigidBody as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(GROUPRIGIDBODY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for GroupRigidBody {
    fn type_info(&self) -> &'static TypeInfo {
        GROUPRIGIDBODY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static GROUPRIGIDBODY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroupRigidBody-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("GroupRigidBody"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CharacterPhysicsBody {
    pub _glacier_base: PhysicsBody,
}

pub trait CharacterPhysicsBodyTrait: PhysicsBodyTrait {
}

impl CharacterPhysicsBodyTrait for CharacterPhysicsBody {
}

impl PhysicsBodyTrait for CharacterPhysicsBody {
}

impl super::entity::EntityTrait for CharacterPhysicsBody {
}

impl super::entity::EntityBusPeerTrait for CharacterPhysicsBody {
}

pub static CHARACTERPHYSICSBODY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterPhysicsBody",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSBODY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CharacterPhysicsBody as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CHARACTERPHYSICSBODY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CharacterPhysicsBody {
    fn type_info(&self) -> &'static TypeInfo {
        CHARACTERPHYSICSBODY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CHARACTERPHYSICSBODY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterPhysicsBody-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("CharacterPhysicsBody"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AabbTriggerPhysicsBody {
    pub _glacier_base: PhysicsBody,
}

pub trait AabbTriggerPhysicsBodyTrait: PhysicsBodyTrait {
}

impl AabbTriggerPhysicsBodyTrait for AabbTriggerPhysicsBody {
}

impl PhysicsBodyTrait for AabbTriggerPhysicsBody {
}

impl super::entity::EntityTrait for AabbTriggerPhysicsBody {
}

impl super::entity::EntityBusPeerTrait for AabbTriggerPhysicsBody {
}

pub static AABBTRIGGERPHYSICSBODY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AabbTriggerPhysicsBody",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSBODY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AabbTriggerPhysicsBody as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(AABBTRIGGERPHYSICSBODY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AabbTriggerPhysicsBody {
    fn type_info(&self) -> &'static TypeInfo {
        AABBTRIGGERPHYSICSBODY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static AABBTRIGGERPHYSICSBODY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AabbTriggerPhysicsBody-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("AabbTriggerPhysicsBody"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsManager {
    pub _glacier_base: IglooSubsystem,
}

pub trait PhysicsManagerTrait: IglooSubsystemTrait {
}

impl PhysicsManagerTrait for PhysicsManager {
}

impl IglooSubsystemTrait for PhysicsManager {
}

pub static PHYSICSMANAGER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsManager",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IGLOOSUBSYSTEM_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsManager as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSMANAGER_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsManager {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSMANAGER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSMANAGER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsManager-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsManager"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsEntityCreator {
    pub _glacier_base: IglooEntityCreator,
}

pub trait PhysicsEntityCreatorTrait: IglooEntityCreatorTrait {
}

impl PhysicsEntityCreatorTrait for PhysicsEntityCreator {
}

impl IglooEntityCreatorTrait for PhysicsEntityCreator {
}

pub static PHYSICSENTITYCREATOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsEntityCreator",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IGLOOENTITYCREATOR_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsEntityCreator as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSENTITYCREATOR_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PhysicsEntityCreator {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSENTITYCREATOR_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSENTITYCREATOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsEntityCreator-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsEntityCreator"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsEntityFactory {
    pub _glacier_base: IglooEntityFactory,
}

pub trait PhysicsEntityFactoryTrait: IglooEntityFactoryTrait {
}

impl PhysicsEntityFactoryTrait for PhysicsEntityFactory {
}

impl IglooEntityFactoryTrait for PhysicsEntityFactory {
}

pub static PHYSICSENTITYFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsEntityFactory",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IGLOOENTITYFACTORY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsEntityFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSENTITYFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PhysicsEntityFactory {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSENTITYFACTORY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSENTITYFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsEntityFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsEntityFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DestructionDebrisReferenceObjectData {
    pub _glacier_base: super::entity::SpatialReferenceObjectData,
    pub insert_type: DebrisInsertType,
    pub gap: f32,
    pub max_piece_count: super::core::QualityScalableInt,
    pub random_seed: u32,
}

pub trait DestructionDebrisReferenceObjectDataTrait: super::entity::SpatialReferenceObjectDataTrait {
    fn insert_type(&self) -> &DebrisInsertType;
    fn gap(&self) -> &f32;
    fn max_piece_count(&self) -> &super::core::QualityScalableInt;
    fn random_seed(&self) -> &u32;
}

impl DestructionDebrisReferenceObjectDataTrait for DestructionDebrisReferenceObjectData {
    fn insert_type(&self) -> &DebrisInsertType {
        &self.insert_type
    }
    fn gap(&self) -> &f32 {
        &self.gap
    }
    fn max_piece_count(&self) -> &super::core::QualityScalableInt {
        &self.max_piece_count
    }
    fn random_seed(&self) -> &u32 {
        &self.random_seed
    }
}

impl super::entity::SpatialReferenceObjectDataTrait for DestructionDebrisReferenceObjectData {
    fn local_player_id(&self) -> &super::core::LocalPlayerId {
        self._glacier_base.local_player_id()
    }
}

impl super::entity::ReferenceObjectDataTrait for DestructionDebrisReferenceObjectData {
    fn blueprint_transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.blueprint_transform()
    }
    fn blueprint(&self) -> &Option<Arc<Mutex<dyn super::entity::BlueprintTrait>>> {
        self._glacier_base.blueprint()
    }
    fn object_variation(&self) -> &Option<Arc<Mutex<dyn super::entity::ObjectVariationTrait>>> {
        self._glacier_base.object_variation()
    }
    fn stream_realm(&self) -> &super::entity::StreamRealm {
        self._glacier_base.stream_realm()
    }
    fn radiosity_type_override(&self) -> &super::core::RadiosityTypeOverride {
        self._glacier_base.radiosity_type_override()
    }
    fn lightmap_resolution_scale(&self) -> &u32 {
        self._glacier_base.lightmap_resolution_scale()
    }
    fn lightmap_scale_with_size(&self) -> &bool {
        self._glacier_base.lightmap_scale_with_size()
    }
    fn rendering_overrides(&self) -> &super::core::RenderingOverrides {
        self._glacier_base.rendering_overrides()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
    fn create_indestructible_entity(&self) -> &bool {
        self._glacier_base.create_indestructible_entity()
    }
}

impl super::entity::GameObjectDataTrait for DestructionDebrisReferenceObjectData {
}

impl super::core::DataBusPeerTrait for DestructionDebrisReferenceObjectData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for DestructionDebrisReferenceObjectData {
}

impl super::core::DataContainerTrait for DestructionDebrisReferenceObjectData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static DESTRUCTIONDEBRISREFERENCEOBJECTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionDebrisReferenceObjectData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALREFERENCEOBJECTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DestructionDebrisReferenceObjectData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "InsertType",
                flags: MemberInfoFlags::new(0),
                field_type: "DebrisInsertType",
                rust_offset: offset_of!(DestructionDebrisReferenceObjectData, insert_type),
            },
            FieldInfoData {
                name: "Gap",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DestructionDebrisReferenceObjectData, gap),
            },
            FieldInfoData {
                name: "MaxPieceCount",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableInt",
                rust_offset: offset_of!(DestructionDebrisReferenceObjectData, max_piece_count),
            },
            FieldInfoData {
                name: "RandomSeed",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DestructionDebrisReferenceObjectData, random_seed),
            },
        ],
    }),
    array_type: Some(DESTRUCTIONDEBRISREFERENCEOBJECTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DestructionDebrisReferenceObjectData {
    fn type_info(&self) -> &'static TypeInfo {
        DESTRUCTIONDEBRISREFERENCEOBJECTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DESTRUCTIONDEBRISREFERENCEOBJECTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionDebrisReferenceObjectData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("DestructionDebrisReferenceObjectData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum DebrisInsertType {
    #[default]
    DebrisInsertType_UseDebrisPivot = 0,
    DebrisInsertType_RandomPosition = 1,
    DebrisInsertType_RandomTransform = 2,
}

pub static DEBRISINSERTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisInsertType",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(DEBRISINSERTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DebrisInsertType {
    fn type_info(&self) -> &'static TypeInfo {
        DEBRISINSERTTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DEBRISINSERTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisInsertType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("DebrisInsertType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DestructionControllerComponentData {
    pub _glacier_base: super::entity::GameComponentData,
    pub structural_integrity_analyzer_enable: bool,
    pub breaking_length: f32,
    pub fracture_position: f32,
    pub destruction_radius: f32,
    pub delay: f32,
    pub refine_radius: f32,
}

pub trait DestructionControllerComponentDataTrait: super::entity::GameComponentDataTrait {
    fn structural_integrity_analyzer_enable(&self) -> &bool;
    fn breaking_length(&self) -> &f32;
    fn fracture_position(&self) -> &f32;
    fn destruction_radius(&self) -> &f32;
    fn delay(&self) -> &f32;
    fn refine_radius(&self) -> &f32;
}

impl DestructionControllerComponentDataTrait for DestructionControllerComponentData {
    fn structural_integrity_analyzer_enable(&self) -> &bool {
        &self.structural_integrity_analyzer_enable
    }
    fn breaking_length(&self) -> &f32 {
        &self.breaking_length
    }
    fn fracture_position(&self) -> &f32 {
        &self.fracture_position
    }
    fn destruction_radius(&self) -> &f32 {
        &self.destruction_radius
    }
    fn delay(&self) -> &f32 {
        &self.delay
    }
    fn refine_radius(&self) -> &f32 {
        &self.refine_radius
    }
}

impl super::entity::GameComponentDataTrait for DestructionControllerComponentData {
}

impl super::entity::ComponentDataTrait for DestructionControllerComponentData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn components(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components()
    }
    fn client_index(&self) -> &u8 {
        self._glacier_base.client_index()
    }
    fn server_index(&self) -> &u8 {
        self._glacier_base.server_index()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
}

impl super::entity::GameObjectDataTrait for DestructionControllerComponentData {
}

impl super::core::DataBusPeerTrait for DestructionControllerComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for DestructionControllerComponentData {
}

impl super::core::DataContainerTrait for DestructionControllerComponentData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static DESTRUCTIONCONTROLLERCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionControllerComponentData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::GAMECOMPONENTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DestructionControllerComponentData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "StructuralIntegrityAnalyzerEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DestructionControllerComponentData, structural_integrity_analyzer_enable),
            },
            FieldInfoData {
                name: "BreakingLength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DestructionControllerComponentData, breaking_length),
            },
            FieldInfoData {
                name: "FracturePosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DestructionControllerComponentData, fracture_position),
            },
            FieldInfoData {
                name: "DestructionRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DestructionControllerComponentData, destruction_radius),
            },
            FieldInfoData {
                name: "Delay",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DestructionControllerComponentData, delay),
            },
            FieldInfoData {
                name: "RefineRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DestructionControllerComponentData, refine_radius),
            },
        ],
    }),
    array_type: Some(DESTRUCTIONCONTROLLERCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DestructionControllerComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        DESTRUCTIONCONTROLLERCOMPONENTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DESTRUCTIONCONTROLLERCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionControllerComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("DestructionControllerComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EdgeModelInstance {
    pub transform: super::core::LinearTransform,
}

pub trait EdgeModelInstanceTrait: TypeObject {
    fn transform(&self) -> &super::core::LinearTransform;
}

impl EdgeModelInstanceTrait for EdgeModelInstance {
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
}

pub static EDGEMODELINSTANCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EdgeModelInstance",
    flags: MemberInfoFlags::new(32841),
    module: "Physics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EdgeModelInstance as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(EdgeModelInstance, transform),
            },
        ],
    }),
    array_type: Some(EDGEMODELINSTANCE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EdgeModelInstance {
    fn type_info(&self) -> &'static TypeInfo {
        EDGEMODELINSTANCE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static EDGEMODELINSTANCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EdgeModelInstance-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("EdgeModelInstance"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DestructionPartComponentData {
    pub _glacier_base: super::entity::GameComponentData,
    pub objects: Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>>,
    pub part_index: u32,
    pub fixed: bool,
    pub fragile: bool,
}

pub trait DestructionPartComponentDataTrait: super::entity::GameComponentDataTrait {
    fn objects(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>>;
    fn part_index(&self) -> &u32;
    fn fixed(&self) -> &bool;
    fn fragile(&self) -> &bool;
}

impl DestructionPartComponentDataTrait for DestructionPartComponentData {
    fn objects(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        &self.objects
    }
    fn part_index(&self) -> &u32 {
        &self.part_index
    }
    fn fixed(&self) -> &bool {
        &self.fixed
    }
    fn fragile(&self) -> &bool {
        &self.fragile
    }
}

impl super::entity::GameComponentDataTrait for DestructionPartComponentData {
}

impl super::entity::ComponentDataTrait for DestructionPartComponentData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn components(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components()
    }
    fn client_index(&self) -> &u8 {
        self._glacier_base.client_index()
    }
    fn server_index(&self) -> &u8 {
        self._glacier_base.server_index()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
}

impl super::entity::GameObjectDataTrait for DestructionPartComponentData {
}

impl super::core::DataBusPeerTrait for DestructionPartComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for DestructionPartComponentData {
}

impl super::core::DataContainerTrait for DestructionPartComponentData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static DESTRUCTIONPARTCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionPartComponentData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::GAMECOMPONENTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DestructionPartComponentData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Objects",
                flags: MemberInfoFlags::new(144),
                field_type: "GameObjectData-Array",
                rust_offset: offset_of!(DestructionPartComponentData, objects),
            },
            FieldInfoData {
                name: "PartIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DestructionPartComponentData, part_index),
            },
            FieldInfoData {
                name: "Fixed",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DestructionPartComponentData, fixed),
            },
            FieldInfoData {
                name: "Fragile",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DestructionPartComponentData, fragile),
            },
        ],
    }),
    array_type: Some(DESTRUCTIONPARTCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DestructionPartComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        DESTRUCTIONPARTCOMPONENTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DESTRUCTIONPARTCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionPartComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("DestructionPartComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FakeHingeData {
    pub _glacier_base: FakePhysicsData,
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

pub trait FakeHingeDataTrait: FakePhysicsDataTrait {
    fn pivot(&self) -> &super::core::Vec3;
    fn rotation_axis(&self) -> &super::core::Vec3;
    fn extension_axis(&self) -> &super::core::Vec3;
    fn min_angle(&self) -> &f32;
    fn max_angle(&self) -> &f32;
    fn angular_dampening(&self) -> &f32;
    fn pullback_acceleration(&self) -> &f32;
    fn progressive_exponent(&self) -> &f32;
    fn inertia_modifier(&self) -> &f32;
}

impl FakeHingeDataTrait for FakeHingeData {
    fn pivot(&self) -> &super::core::Vec3 {
        &self.pivot
    }
    fn rotation_axis(&self) -> &super::core::Vec3 {
        &self.rotation_axis
    }
    fn extension_axis(&self) -> &super::core::Vec3 {
        &self.extension_axis
    }
    fn min_angle(&self) -> &f32 {
        &self.min_angle
    }
    fn max_angle(&self) -> &f32 {
        &self.max_angle
    }
    fn angular_dampening(&self) -> &f32 {
        &self.angular_dampening
    }
    fn pullback_acceleration(&self) -> &f32 {
        &self.pullback_acceleration
    }
    fn progressive_exponent(&self) -> &f32 {
        &self.progressive_exponent
    }
    fn inertia_modifier(&self) -> &f32 {
        &self.inertia_modifier
    }
}

impl FakePhysicsDataTrait for FakeHingeData {
    fn child_fake_physics(&self) -> &Option<Arc<Mutex<dyn FakePhysicsDataTrait>>> {
        self._glacier_base.child_fake_physics()
    }
    fn start_speed(&self) -> &f32 {
        self._glacier_base.start_speed()
    }
    fn gravity_modifier(&self) -> &f32 {
        self._glacier_base.gravity_modifier()
    }
    fn start_dampening(&self) -> &f32 {
        self._glacier_base.start_dampening()
    }
    fn end_dampening(&self) -> &f32 {
        self._glacier_base.end_dampening()
    }
    fn min_speed(&self) -> &f32 {
        self._glacier_base.min_speed()
    }
    fn max_speed(&self) -> &f32 {
        self._glacier_base.max_speed()
    }
    fn mass(&self) -> &f32 {
        self._glacier_base.mass()
    }
}

impl super::core::DataContainerTrait for FakeHingeData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static FAKEHINGEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FakeHingeData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(FAKEPHYSICSDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FakeHingeData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Pivot",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(FakeHingeData, pivot),
            },
            FieldInfoData {
                name: "RotationAxis",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(FakeHingeData, rotation_axis),
            },
            FieldInfoData {
                name: "ExtensionAxis",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(FakeHingeData, extension_axis),
            },
            FieldInfoData {
                name: "MinAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FakeHingeData, min_angle),
            },
            FieldInfoData {
                name: "MaxAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FakeHingeData, max_angle),
            },
            FieldInfoData {
                name: "AngularDampening",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FakeHingeData, angular_dampening),
            },
            FieldInfoData {
                name: "PullbackAcceleration",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FakeHingeData, pullback_acceleration),
            },
            FieldInfoData {
                name: "ProgressiveExponent",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FakeHingeData, progressive_exponent),
            },
            FieldInfoData {
                name: "InertiaModifier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FakeHingeData, inertia_modifier),
            },
        ],
    }),
    array_type: Some(FAKEHINGEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FakeHingeData {
    fn type_info(&self) -> &'static TypeInfo {
        FAKEHINGEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FAKEHINGEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FakeHingeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("FakeHingeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FakeSpringData {
    pub _glacier_base: FakePhysicsData,
    pub direction: super::core::Vec3,
    pub length: f32,
    pub acceleration: f32,
    pub progressive_exponent: f32,
    pub damping: f32,
}

pub trait FakeSpringDataTrait: FakePhysicsDataTrait {
    fn direction(&self) -> &super::core::Vec3;
    fn length(&self) -> &f32;
    fn acceleration(&self) -> &f32;
    fn progressive_exponent(&self) -> &f32;
    fn damping(&self) -> &f32;
}

impl FakeSpringDataTrait for FakeSpringData {
    fn direction(&self) -> &super::core::Vec3 {
        &self.direction
    }
    fn length(&self) -> &f32 {
        &self.length
    }
    fn acceleration(&self) -> &f32 {
        &self.acceleration
    }
    fn progressive_exponent(&self) -> &f32 {
        &self.progressive_exponent
    }
    fn damping(&self) -> &f32 {
        &self.damping
    }
}

impl FakePhysicsDataTrait for FakeSpringData {
    fn child_fake_physics(&self) -> &Option<Arc<Mutex<dyn FakePhysicsDataTrait>>> {
        self._glacier_base.child_fake_physics()
    }
    fn start_speed(&self) -> &f32 {
        self._glacier_base.start_speed()
    }
    fn gravity_modifier(&self) -> &f32 {
        self._glacier_base.gravity_modifier()
    }
    fn start_dampening(&self) -> &f32 {
        self._glacier_base.start_dampening()
    }
    fn end_dampening(&self) -> &f32 {
        self._glacier_base.end_dampening()
    }
    fn min_speed(&self) -> &f32 {
        self._glacier_base.min_speed()
    }
    fn max_speed(&self) -> &f32 {
        self._glacier_base.max_speed()
    }
    fn mass(&self) -> &f32 {
        self._glacier_base.mass()
    }
}

impl super::core::DataContainerTrait for FakeSpringData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static FAKESPRINGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FakeSpringData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(FAKEPHYSICSDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FakeSpringData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Direction",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(FakeSpringData, direction),
            },
            FieldInfoData {
                name: "Length",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FakeSpringData, length),
            },
            FieldInfoData {
                name: "Acceleration",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FakeSpringData, acceleration),
            },
            FieldInfoData {
                name: "ProgressiveExponent",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FakeSpringData, progressive_exponent),
            },
            FieldInfoData {
                name: "Damping",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FakeSpringData, damping),
            },
        ],
    }),
    array_type: Some(FAKESPRINGDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FakeSpringData {
    fn type_info(&self) -> &'static TypeInfo {
        FAKESPRINGDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FAKESPRINGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FakeSpringData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("FakeSpringData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FakePhysicsData {
    pub _glacier_base: super::core::DataContainer,
    pub child_fake_physics: Option<Arc<Mutex<dyn FakePhysicsDataTrait>>>,
    pub start_speed: f32,
    pub gravity_modifier: f32,
    pub start_dampening: f32,
    pub end_dampening: f32,
    pub min_speed: f32,
    pub max_speed: f32,
    pub mass: f32,
}

pub trait FakePhysicsDataTrait: super::core::DataContainerTrait {
    fn child_fake_physics(&self) -> &Option<Arc<Mutex<dyn FakePhysicsDataTrait>>>;
    fn start_speed(&self) -> &f32;
    fn gravity_modifier(&self) -> &f32;
    fn start_dampening(&self) -> &f32;
    fn end_dampening(&self) -> &f32;
    fn min_speed(&self) -> &f32;
    fn max_speed(&self) -> &f32;
    fn mass(&self) -> &f32;
}

impl FakePhysicsDataTrait for FakePhysicsData {
    fn child_fake_physics(&self) -> &Option<Arc<Mutex<dyn FakePhysicsDataTrait>>> {
        &self.child_fake_physics
    }
    fn start_speed(&self) -> &f32 {
        &self.start_speed
    }
    fn gravity_modifier(&self) -> &f32 {
        &self.gravity_modifier
    }
    fn start_dampening(&self) -> &f32 {
        &self.start_dampening
    }
    fn end_dampening(&self) -> &f32 {
        &self.end_dampening
    }
    fn min_speed(&self) -> &f32 {
        &self.min_speed
    }
    fn max_speed(&self) -> &f32 {
        &self.max_speed
    }
    fn mass(&self) -> &f32 {
        &self.mass
    }
}

impl super::core::DataContainerTrait for FakePhysicsData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static FAKEPHYSICSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FakePhysicsData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FakePhysicsData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ChildFakePhysics",
                flags: MemberInfoFlags::new(0),
                field_type: "FakePhysicsData",
                rust_offset: offset_of!(FakePhysicsData, child_fake_physics),
            },
            FieldInfoData {
                name: "StartSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FakePhysicsData, start_speed),
            },
            FieldInfoData {
                name: "GravityModifier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FakePhysicsData, gravity_modifier),
            },
            FieldInfoData {
                name: "StartDampening",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FakePhysicsData, start_dampening),
            },
            FieldInfoData {
                name: "EndDampening",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FakePhysicsData, end_dampening),
            },
            FieldInfoData {
                name: "MinSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FakePhysicsData, min_speed),
            },
            FieldInfoData {
                name: "MaxSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FakePhysicsData, max_speed),
            },
            FieldInfoData {
                name: "Mass",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FakePhysicsData, mass),
            },
        ],
    }),
    array_type: Some(FAKEPHYSICSDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FakePhysicsData {
    fn type_info(&self) -> &'static TypeInfo {
        FAKEPHYSICSDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FAKEPHYSICSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FakePhysicsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("FakePhysicsData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsAngularLimitConstraintData {
    pub _glacier_base: PhysicsConstraintData,
    pub min_angle: f32,
    pub max_angle: f32,
    pub constrained_axis: AngularLimitConstraintAxis,
    pub angular_stiffness: f32,
    pub target_transform: super::core::LinearTransform,
}

pub trait PhysicsAngularLimitConstraintDataTrait: PhysicsConstraintDataTrait {
    fn min_angle(&self) -> &f32;
    fn max_angle(&self) -> &f32;
    fn constrained_axis(&self) -> &AngularLimitConstraintAxis;
    fn angular_stiffness(&self) -> &f32;
    fn target_transform(&self) -> &super::core::LinearTransform;
}

impl PhysicsAngularLimitConstraintDataTrait for PhysicsAngularLimitConstraintData {
    fn min_angle(&self) -> &f32 {
        &self.min_angle
    }
    fn max_angle(&self) -> &f32 {
        &self.max_angle
    }
    fn constrained_axis(&self) -> &AngularLimitConstraintAxis {
        &self.constrained_axis
    }
    fn angular_stiffness(&self) -> &f32 {
        &self.angular_stiffness
    }
    fn target_transform(&self) -> &super::core::LinearTransform {
        &self.target_transform
    }
}

impl PhysicsConstraintDataTrait for PhysicsAngularLimitConstraintData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn is_breakable(&self) -> &bool {
        self._glacier_base.is_breakable()
    }
    fn break_threshold(&self) -> &f32 {
        self._glacier_base.break_threshold()
    }
    fn stabilized(&self) -> &bool {
        self._glacier_base.stabilized()
    }
    fn enable_continuous_simulation(&self) -> &bool {
        self._glacier_base.enable_continuous_simulation()
    }
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn world_index(&self) -> &u8 {
        self._glacier_base.world_index()
    }
    fn initial_stance_data(&self) -> &Option<Arc<Mutex<dyn PhysicsConstraintInitialStanceDataTrait>>> {
        self._glacier_base.initial_stance_data()
    }
}

impl super::entity::EntityDataTrait for PhysicsAngularLimitConstraintData {
}

impl super::entity::GameObjectDataTrait for PhysicsAngularLimitConstraintData {
}

impl super::core::DataBusPeerTrait for PhysicsAngularLimitConstraintData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for PhysicsAngularLimitConstraintData {
}

impl super::core::DataContainerTrait for PhysicsAngularLimitConstraintData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PHYSICSANGULARLIMITCONSTRAINTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsAngularLimitConstraintData",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCONSTRAINTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsAngularLimitConstraintData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MinAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PhysicsAngularLimitConstraintData, min_angle),
            },
            FieldInfoData {
                name: "MaxAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PhysicsAngularLimitConstraintData, max_angle),
            },
            FieldInfoData {
                name: "ConstrainedAxis",
                flags: MemberInfoFlags::new(0),
                field_type: "AngularLimitConstraintAxis",
                rust_offset: offset_of!(PhysicsAngularLimitConstraintData, constrained_axis),
            },
            FieldInfoData {
                name: "AngularStiffness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PhysicsAngularLimitConstraintData, angular_stiffness),
            },
            FieldInfoData {
                name: "TargetTransform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(PhysicsAngularLimitConstraintData, target_transform),
            },
        ],
    }),
    array_type: Some(PHYSICSANGULARLIMITCONSTRAINTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsAngularLimitConstraintData {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSANGULARLIMITCONSTRAINTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSANGULARLIMITCONSTRAINTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsAngularLimitConstraintData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("PhysicsAngularLimitConstraintData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum AngularLimitConstraintAxis {
    #[default]
    AngularLimitConstraintAxis_X = 0,
    AngularLimitConstraintAxis_Y = 1,
    AngularLimitConstraintAxis_Z = 2,
}

pub static ANGULARLIMITCONSTRAINTAXIS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngularLimitConstraintAxis",
    flags: MemberInfoFlags::new(49429),
    module: "Physics",
    data: TypeInfoData::Enum,
    array_type: Some(ANGULARLIMITCONSTRAINTAXIS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AngularLimitConstraintAxis {
    fn type_info(&self) -> &'static TypeInfo {
        ANGULARLIMITCONSTRAINTAXIS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ANGULARLIMITCONSTRAINTAXIS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngularLimitConstraintAxis-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("AngularLimitConstraintAxis"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VehiclePhysicsActionFactory {
    pub _glacier_base: IglooEntityFactory,
}

pub trait VehiclePhysicsActionFactoryTrait: IglooEntityFactoryTrait {
}

impl VehiclePhysicsActionFactoryTrait for VehiclePhysicsActionFactory {
}

impl IglooEntityFactoryTrait for VehiclePhysicsActionFactory {
}

pub static VEHICLEPHYSICSACTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehiclePhysicsActionFactory",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IGLOOENTITYFACTORY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VehiclePhysicsActionFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(VEHICLEPHYSICSACTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VehiclePhysicsActionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        VEHICLEPHYSICSACTIONFACTORY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VEHICLEPHYSICSACTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehiclePhysicsActionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("VehiclePhysicsActionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VehiclePhysicsAction {
    pub _glacier_base: PhysicsAction,
}

pub trait VehiclePhysicsActionTrait: PhysicsActionTrait {
}

impl VehiclePhysicsActionTrait for VehiclePhysicsAction {
}

impl PhysicsActionTrait for VehiclePhysicsAction {
}

impl super::entity::EntityTrait for VehiclePhysicsAction {
}

impl super::entity::EntityBusPeerTrait for VehiclePhysicsAction {
}

pub static VEHICLEPHYSICSACTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehiclePhysicsAction",
    flags: MemberInfoFlags::new(101),
    module: "Physics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSACTION_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VehiclePhysicsAction as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(VEHICLEPHYSICSACTION_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for VehiclePhysicsAction {
    fn type_info(&self) -> &'static TypeInfo {
        VEHICLEPHYSICSACTION_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VEHICLEPHYSICSACTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehiclePhysicsAction-Array",
    flags: MemberInfoFlags::new(145),
    module: "Physics",
    data: TypeInfoData::Array("VehiclePhysicsAction"),
    array_type: None,
    alignment: 8,
};


