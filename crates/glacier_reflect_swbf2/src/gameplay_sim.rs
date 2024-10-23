use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_gameplay_sim_types(registry: &mut TypeRegistry) {
    registry.register_type(TARGETCAMERA_TYPE_INFO);
    registry.register_type(TARGETCAMERA_ARRAY_TYPE_INFO);
    registry.register_type(TARGETCAMERACALLBACK_TYPE_INFO);
    registry.register_type(TARGETCAMERACALLBACK_ARRAY_TYPE_INFO);
    registry.register_type(FREECAMERA_TYPE_INFO);
    registry.register_type(FREECAMERA_ARRAY_TYPE_INFO);
    registry.register_type(CAMERA_TYPE_INFO);
    registry.register_type(CAMERA_ARRAY_TYPE_INFO);
    registry.register_type(ANTANIMATABLEENTITY_TYPE_INFO);
    registry.register_type(ANTANIMATABLEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(DIFFICULTYDATASBASE_TYPE_INFO);
    registry.register_type(DIFFICULTYDATASBASE_ARRAY_TYPE_INFO);
    registry.register_type(LOCKTYPE_TYPE_INFO);
    registry.register_type(LOCKTYPE_ARRAY_TYPE_INFO);
    registry.register_type(TOOLDATA_TYPE_INFO);
    registry.register_type(TOOLDATA_ARRAY_TYPE_INFO);
    registry.register_type(VOICECHANNEL_TYPE_INFO);
    registry.register_type(VOICECHANNEL_ARRAY_TYPE_INFO);
    registry.register_type(SQUADID_TYPE_INFO);
    registry.register_type(SQUADID_ARRAY_TYPE_INFO);
    registry.register_type(FACTIONID_TYPE_INFO);
    registry.register_type(FACTIONID_ARRAY_TYPE_INFO);
    registry.register_type(TEAMID_TYPE_INFO);
    registry.register_type(TEAMID_ARRAY_TYPE_INFO);
    registry.register_type(IKEFFECTORENUM_TYPE_INFO);
    registry.register_type(IKEFFECTORENUM_ARRAY_TYPE_INFO);
    registry.register_type(ROTATIONAXIS_TYPE_INFO);
    registry.register_type(ROTATIONAXIS_ARRAY_TYPE_INFO);
    registry.register_type(WHEELPHYSICSTYPE_TYPE_INFO);
    registry.register_type(WHEELPHYSICSTYPE_ARRAY_TYPE_INFO);
    registry.register_type(ENTRYCLASS_TYPE_INFO);
    registry.register_type(ENTRYCLASS_ARRAY_TYPE_INFO);
    registry.register_type(SHADERPARAMETERCOMPONENTDATA_TYPE_INFO);
    registry.register_type(SHADERPARAMETERCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(SHADERPARAMETERTEXTURE_TYPE_INFO);
    registry.register_type(SHADERPARAMETERTEXTURE_ARRAY_TYPE_INFO);
    registry.register_type(SHADERPARAMETERVECTOR_TYPE_INFO);
    registry.register_type(SHADERPARAMETERVECTOR_ARRAY_TYPE_INFO);
    registry.register_type(COMPONENTENABLEDMESSAGE_TYPE_INFO);
    registry.register_type(CONTROLLABLEHEALTHCOMPONENTDATA_TYPE_INFO);
    registry.register_type(CONTROLLABLEHEALTHCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(GAMEHEALTHCOMPONENTDATA_TYPE_INFO);
    registry.register_type(GAMEHEALTHCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(DEFAULTHEALTHCOMPONENTDATA_TYPE_INFO);
    registry.register_type(DEFAULTHEALTHCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(HEALTHCOMPONENTDATA_TYPE_INFO);
    registry.register_type(HEALTHCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(HEALTHCOMPONENTONDAMAGEMESSAGE_TYPE_INFO);
    registry.register_type(CONTROLLABLEPHYSICSCOMPONENTDATA_TYPE_INFO);
    registry.register_type(CONTROLLABLEPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(GAMEPHYSICSCOMPONENTDATA_TYPE_INFO);
    registry.register_type(GAMEPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(TWOPARTCOMPONENTDATA_TYPE_INFO);
    registry.register_type(TWOPARTCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(PARTCOMPONENTDATA_TYPE_INFO);
    registry.register_type(PARTCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(PARTLINKDATA_TYPE_INFO);
    registry.register_type(PARTLINKDATA_ARRAY_TYPE_INFO);
    registry.register_type(PARTCOMPONENTCONSTANTS_TYPE_INFO);
    registry.register_type(PARTCOMPONENTCONSTANTS_ARRAY_TYPE_INFO);
    registry.register_type(HEALTHSTATEDATA_TYPE_INFO);
    registry.register_type(HEALTHSTATEDATA_ARRAY_TYPE_INFO);
    registry.register_type(HEALTHSTATESPAWNOBJECTSCULLINGPARAMS_TYPE_INFO);
    registry.register_type(HEALTHSTATESPAWNOBJECTSCULLINGPARAMS_ARRAY_TYPE_INFO);
    registry.register_type(BANGERSPAWNIMPULSEPARAMS_TYPE_INFO);
    registry.register_type(BANGERSPAWNIMPULSEPARAMS_ARRAY_TYPE_INFO);
    registry.register_type(PARTCOMPONENTTRACKHEALTHSTATESPAWNSMESSAGE_TYPE_INFO);
    registry.register_type(TRACKHEALTHSTATESPAWNS_TYPE_INFO);
    registry.register_type(TRACKHEALTHSTATESPAWNS_ARRAY_TYPE_INFO);
    registry.register_type(BLUEPRINTBUNDLESTATUSENTITYDATA_TYPE_INFO);
    registry.register_type(BLUEPRINTBUNDLESTATUSENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(BLUEPRINTBUNDLECOLLECTIONENTITYDATA_TYPE_INFO);
    registry.register_type(BLUEPRINTBUNDLECOLLECTIONENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(BLUEPRINTBUNDLEENTITYDATA_TYPE_INFO);
    registry.register_type(BLUEPRINTBUNDLEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(RUNTIMEASSETBLUEPRINT_TYPE_INFO);
    registry.register_type(RUNTIMEASSETBLUEPRINT_ARRAY_TYPE_INFO);
    registry.register_type(AUTOGENERATEDBUNDLE_TYPE_INFO);
    registry.register_type(AUTOGENERATEDBUNDLE_ARRAY_TYPE_INFO);
    registry.register_type(BLUEPRINTBUNDLECOLLECTION_TYPE_INFO);
    registry.register_type(BLUEPRINTBUNDLECOLLECTION_ARRAY_TYPE_INFO);
    registry.register_type(BLUEPRINTBUNDLEREFERENCE_TYPE_INFO);
    registry.register_type(BLUEPRINTBUNDLEREFERENCE_ARRAY_TYPE_INFO);
    registry.register_type(DYNAMICBLUEPRINTBUNDLEASSET_TYPE_INFO);
    registry.register_type(DYNAMICBLUEPRINTBUNDLEASSET_ARRAY_TYPE_INFO);
    registry.register_type(DYNAMICBUNDLEASSET_TYPE_INFO);
    registry.register_type(DYNAMICBUNDLEASSET_ARRAY_TYPE_INFO);
    registry.register_type(BLUEPRINTBUNDLE_TYPE_INFO);
    registry.register_type(BLUEPRINTBUNDLE_ARRAY_TYPE_INFO);
    registry.register_type(BLUEPRINTBUNDLESETTINGS_TYPE_INFO);
    registry.register_type(BLUEPRINTBUNDLESETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(ANTANIMATABLEENTITYDATA_TYPE_INFO);
    registry.register_type(ANTANIMATABLEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(ANTANIMATABLECOMPONENTMESHDATA_TYPE_INFO);
    registry.register_type(ANTANIMATABLECOMPONENTMESHDATA_ARRAY_TYPE_INFO);
    registry.register_type(ANIMATIONCONTROLMODEENUM_TYPE_INFO);
    registry.register_type(ANIMATIONCONTROLMODEENUM_ARRAY_TYPE_INFO);
    registry.register_type(JOINTOUTPUTMODEENUM_TYPE_INFO);
    registry.register_type(JOINTOUTPUTMODEENUM_ARRAY_TYPE_INFO);
    registry.register_type(ANTANIMATABLECOMPONENTMESHRENDERCONTEXT_TYPE_INFO);
    registry.register_type(ANTANIMATABLECOMPONENTMESHRENDERCONTEXT_ARRAY_TYPE_INFO);
    registry.register_type(ANTANIMATABLECOMPONENTMESHRENDERTYPE_TYPE_INFO);
    registry.register_type(ANTANIMATABLECOMPONENTMESHRENDERTYPE_ARRAY_TYPE_INFO);
    registry.register_type(ANTANIMATIONHANDLERDATA_TYPE_INFO);
    registry.register_type(ANTANIMATIONHANDLERDATA_ARRAY_TYPE_INFO);
    registry.register_type(ANIMATIONCONTROLBINDING_TYPE_INFO);
    registry.register_type(ANIMATIONCONTROLBINDING_ARRAY_TYPE_INFO);
    registry.register_type(LODBINDING_TYPE_INFO);
    registry.register_type(LODBINDING_ARRAY_TYPE_INFO);
    registry.register_type(ANTENUMERATION_TYPE_INFO);
    registry.register_type(ANTENUMERATION_ARRAY_TYPE_INFO);
    registry.register_type(GAMEAIWEAPONDATA_TYPE_INFO);
    registry.register_type(GAMEAIWEAPONDATA_ARRAY_TYPE_INFO);
    registry.register_type(GAMEAIENTRYDATA_TYPE_INFO);
    registry.register_type(GAMEAIENTRYDATA_ARRAY_TYPE_INFO);
    registry.register_type(POSECONSTRAINTSDATA_TYPE_INFO);
    registry.register_type(POSECONSTRAINTSDATA_ARRAY_TYPE_INFO);
    registry.register_type(AIMINGCONSTRAINTSDATA_TYPE_INFO);
    registry.register_type(AIMINGCONSTRAINTSDATA_ARRAY_TYPE_INFO);
    registry.register_type(TEXTUREASSETENTITYDATA_TYPE_INFO);
    registry.register_type(TEXTUREASSETENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINSHADERPARAMETERENTITYDATA_TYPE_INFO);
    registry.register_type(TERRAINSHADERPARAMETERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINSHADERPARAMETER_TYPE_INFO);
    registry.register_type(TERRAINSHADERPARAMETER_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINSHADERPARAMETERDATATYPE_TYPE_INFO);
    registry.register_type(TERRAINSHADERPARAMETERDATATYPE_ARRAY_TYPE_INFO);
    registry.register_type(SHADERPARAMETERENTITYDATA_TYPE_INFO);
    registry.register_type(SHADERPARAMETERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SHADERPARAMETERDATATYPE_TYPE_INFO);
    registry.register_type(SHADERPARAMETERDATATYPE_ARRAY_TYPE_INFO);
    registry.register_type(ANTLAYERBLENDTYPE_TYPE_INFO);
    registry.register_type(ANTLAYERBLENDTYPE_ARRAY_TYPE_INFO);
    registry.register_type(ANTCLIPENDRULE_TYPE_INFO);
    registry.register_type(ANTCLIPENDRULE_ARRAY_TYPE_INFO);
    registry.register_type(ANTCLIPSTARTRULE_TYPE_INFO);
    registry.register_type(ANTCLIPSTARTRULE_ARRAY_TYPE_INFO);
    registry.register_type(SUBVIEWDATA_TYPE_INFO);
    registry.register_type(SUBVIEWDATA_ARRAY_TYPE_INFO);
    registry.register_type(PLAYERVIEWDATA_TYPE_INFO);
    registry.register_type(PLAYERVIEWDATA_ARRAY_TYPE_INFO);
    registry.register_type(PLAYERDATA_TYPE_INFO);
    registry.register_type(PLAYERDATA_ARRAY_TYPE_INFO);
    registry.register_type(NETWORKEDONLINEID_TYPE_INFO);
    registry.register_type(NETWORKEDONLINEID_ARRAY_TYPE_INFO);
    registry.register_type(ONLINEIDCONSTANTS_TYPE_INFO);
    registry.register_type(ONLINEIDCONSTANTS_ARRAY_TYPE_INFO);
    registry.register_type(UINETWORKPLAYERDISCONNECTMESSAGE_TYPE_INFO);
    registry.register_type(UINETWORKPLAYERCONNECTMESSAGE_TYPE_INFO);
    registry.register_type(NETWORKSETTINGS_TYPE_INFO);
    registry.register_type(NETWORKSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(VERSIONDATA_TYPE_INFO);
    registry.register_type(VERSIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(UISETTINGS_TYPE_INFO);
    registry.register_type(UISETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(UIFONTCONFIGURATIONASSETBASE_TYPE_INFO);
    registry.register_type(UIFONTCONFIGURATIONASSETBASE_ARRAY_TYPE_INFO);
    registry.register_type(WATERHEALTHCOMPONENTDATA_TYPE_INFO);
    registry.register_type(WATERHEALTHCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINHEALTHCOMPONENTDATA_TYPE_INFO);
    registry.register_type(TERRAINHEALTHCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(WATERPHYSICSCOMPONENTDATA_TYPE_INFO);
    registry.register_type(WATERPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINPHYSICSCOMPONENTDATA_TYPE_INFO);
    registry.register_type(TERRAINPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(WATERENTITYDATA_TYPE_INFO);
    registry.register_type(WATERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINENTITYDATA_TYPE_INFO);
    registry.register_type(TERRAINENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PROFILEOPTIONDATABINARY_TYPE_INFO);
    registry.register_type(PROFILEOPTIONDATABINARY_ARRAY_TYPE_INFO);
    registry.register_type(PROFILEOPTIONDATASTRING_TYPE_INFO);
    registry.register_type(PROFILEOPTIONDATASTRING_ARRAY_TYPE_INFO);
    registry.register_type(PROFILEOPTIONDATABOOL_TYPE_INFO);
    registry.register_type(PROFILEOPTIONDATABOOL_ARRAY_TYPE_INFO);
    registry.register_type(PROFILEOPTIONDATAENUM_TYPE_INFO);
    registry.register_type(PROFILEOPTIONDATAENUM_ARRAY_TYPE_INFO);
    registry.register_type(PROFILEOPTIONDATAENUMITEM_TYPE_INFO);
    registry.register_type(PROFILEOPTIONDATAENUMITEM_ARRAY_TYPE_INFO);
    registry.register_type(PROFILEOPTIONDATAFLOAT_TYPE_INFO);
    registry.register_type(PROFILEOPTIONDATAFLOAT_ARRAY_TYPE_INFO);
    registry.register_type(PROFILEOPTIONDATAINT_TYPE_INFO);
    registry.register_type(PROFILEOPTIONDATAINT_ARRAY_TYPE_INFO);
    registry.register_type(PROFILEOPTIONDATA_TYPE_INFO);
    registry.register_type(PROFILEOPTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(PROFILEOPTIONSASSET_TYPE_INFO);
    registry.register_type(PROFILEOPTIONSASSET_ARRAY_TYPE_INFO);
    registry.register_type(BINARYOPTION_TYPE_INFO);
    registry.register_type(BINARYOPTION_ARRAY_TYPE_INFO);
    registry.register_type(STRINGOPTION_TYPE_INFO);
    registry.register_type(STRINGOPTION_ARRAY_TYPE_INFO);
    registry.register_type(INTOPTION_TYPE_INFO);
    registry.register_type(INTOPTION_ARRAY_TYPE_INFO);
    registry.register_type(FLOATOPTION_TYPE_INFO);
    registry.register_type(FLOATOPTION_ARRAY_TYPE_INFO);
    registry.register_type(PROFILEOPTIONSTYPE_TYPE_INFO);
    registry.register_type(PROFILEOPTIONSTYPE_ARRAY_TYPE_INFO);
    registry.register_type(MATERIALRELATIONTERRAINDESTRUCTIONDATA_TYPE_INFO);
    registry.register_type(MATERIALRELATIONTERRAINDESTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(MATERIALPROPERTYTERRAINDATA_TYPE_INFO);
    registry.register_type(MATERIALPROPERTYTERRAINDATA_ARRAY_TYPE_INFO);
    registry.register_type(MATERIALRELATIONSOUNDDATA_TYPE_INFO);
    registry.register_type(MATERIALRELATIONSOUNDDATA_ARRAY_TYPE_INFO);
    registry.register_type(MATERIALPROPERTYSOUNDDATA_TYPE_INFO);
    registry.register_type(MATERIALPROPERTYSOUNDDATA_ARRAY_TYPE_INFO);
    registry.register_type(MATERIALPROPERTYLICENSEESOUNDDATA_TYPE_INFO);
    registry.register_type(MATERIALPROPERTYLICENSEESOUNDDATA_ARRAY_TYPE_INFO);
    registry.register_type(MATERIALPROPERTYFIREDATA_TYPE_INFO);
    registry.register_type(MATERIALPROPERTYFIREDATA_ARRAY_TYPE_INFO);
    registry.register_type(MATERIALRELATIONVEHICLEDATA_TYPE_INFO);
    registry.register_type(MATERIALRELATIONVEHICLEDATA_ARRAY_TYPE_INFO);
    registry.register_type(MATERIALRELATIONPENETRATIONDATA_TYPE_INFO);
    registry.register_type(MATERIALRELATIONPENETRATIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(MATERIALRELATIONDAMAGEDATA_TYPE_INFO);
    registry.register_type(MATERIALRELATIONDAMAGEDATA_ARRAY_TYPE_INFO);
    registry.register_type(MATERIALRELATIONDYNAMICFIREDATA_TYPE_INFO);
    registry.register_type(MATERIALRELATIONDYNAMICFIREDATA_ARRAY_TYPE_INFO);
    registry.register_type(MATERIALRELATIONEFFECTDATA_TYPE_INFO);
    registry.register_type(MATERIALRELATIONEFFECTDATA_ARRAY_TYPE_INFO);
    registry.register_type(MATERIALPROPERTYIMPULSEDATA_TYPE_INFO);
    registry.register_type(MATERIALPROPERTYIMPULSEDATA_ARRAY_TYPE_INFO);
    registry.register_type(MATERIALPROPERTYHEALTHDATA_TYPE_INFO);
    registry.register_type(MATERIALPROPERTYHEALTHDATA_ARRAY_TYPE_INFO);
    registry.register_type(MATERIALRELATIONDECALDATA_TYPE_INFO);
    registry.register_type(MATERIALRELATIONDECALDATA_ARRAY_TYPE_INFO);
    registry.register_type(MATERIALPROPERTYEFFECTDATA_TYPE_INFO);
    registry.register_type(MATERIALPROPERTYEFFECTDATA_ARRAY_TYPE_INFO);
    registry.register_type(EFFECTWITHSPEEDRANGE_TYPE_INFO);
    registry.register_type(EFFECTWITHSPEEDRANGE_ARRAY_TYPE_INFO);
    registry.register_type(LEVELDESCRIPTIONASSET_TYPE_INFO);
    registry.register_type(LEVELDESCRIPTIONASSET_ARRAY_TYPE_INFO);
    registry.register_type(LEVELSTARTPOINT_TYPE_INFO);
    registry.register_type(LEVELSTARTPOINT_ARRAY_TYPE_INFO);
    registry.register_type(LEVELBUNDLELOAD_TYPE_INFO);
    registry.register_type(LEVELBUNDLELOAD_ARRAY_TYPE_INFO);
    registry.register_type(LEVELDESCRIPTIONINCLUSIONCATEGORY_TYPE_INFO);
    registry.register_type(LEVELDESCRIPTIONINCLUSIONCATEGORY_ARRAY_TYPE_INFO);
    registry.register_type(WORLDRENDERLEVELDESCRIPTIONCOMPONENT_TYPE_INFO);
    registry.register_type(WORLDRENDERLEVELDESCRIPTIONCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(MEMORYLEVELDESCRIPTIONCOMPONENT_TYPE_INFO);
    registry.register_type(MEMORYLEVELDESCRIPTIONCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(LEVELDESCRIPTION_TYPE_INFO);
    registry.register_type(LEVELDESCRIPTION_ARRAY_TYPE_INFO);
    registry.register_type(LEVELDESCRIPTIONCOMPONENT_TYPE_INFO);
    registry.register_type(LEVELDESCRIPTIONCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(LEVELDATA_TYPE_INFO);
    registry.register_type(LEVELDATA_ARRAY_TYPE_INFO);
    registry.register_type(UNLOCKIDTABLE_TYPE_INFO);
    registry.register_type(UNLOCKIDTABLE_ARRAY_TYPE_INFO);
    registry.register_type(LEVELDATACOMPONENT_TYPE_INFO);
    registry.register_type(LEVELDATACOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(LEVELPRELOADINFO_TYPE_INFO);
    registry.register_type(LEVELPRELOADINFO_ARRAY_TYPE_INFO);
    registry.register_type(SUBLEVELPRELOADINFO_TYPE_INFO);
    registry.register_type(SUBLEVELPRELOADINFO_ARRAY_TYPE_INFO);
    registry.register_type(CAMERATRANSITION_TYPE_INFO);
    registry.register_type(CAMERATRANSITION_ARRAY_TYPE_INFO);
    registry.register_type(CAMERAMODEASSET_TYPE_INFO);
    registry.register_type(CAMERAMODEASSET_ARRAY_TYPE_INFO);
    registry.register_type(GAMEAISYSTEM_TYPE_INFO);
    registry.register_type(GAMEAISYSTEM_ARRAY_TYPE_INFO);
    registry.register_type(FACEANIMATIONWAVEMAPPINGSASSET_TYPE_INFO);
    registry.register_type(FACEANIMATIONWAVEMAPPINGSASSET_ARRAY_TYPE_INFO);
    registry.register_type(FACEANIMATIONWAVEMAPPINGS_TYPE_INFO);
    registry.register_type(FACEANIMATIONWAVEMAPPINGS_ARRAY_TYPE_INFO);
    registry.register_type(FACEANIMATIONWAVEMAPPING_TYPE_INFO);
    registry.register_type(FACEANIMATIONWAVEMAPPING_ARRAY_TYPE_INFO);
    registry.register_type(BIGWORLDSETTINGSASSET_TYPE_INFO);
    registry.register_type(BIGWORLDSETTINGSASSET_ARRAY_TYPE_INFO);
    registry.register_type(BIGWORLDSETTING_TYPE_INFO);
    registry.register_type(BIGWORLDSETTING_ARRAY_TYPE_INFO);
    registry.register_type(LEVELREPORTINGASSET_TYPE_INFO);
    registry.register_type(LEVELREPORTINGASSET_ARRAY_TYPE_INFO);
    registry.register_type(HUDDATA_TYPE_INFO);
    registry.register_type(HUDDATA_ARRAY_TYPE_INFO);
    registry.register_type(HIKDATA_TYPE_INFO);
    registry.register_type(HIKDATA_ARRAY_TYPE_INFO);
    registry.register_type(GAMEANIMATIONSETTINGS_TYPE_INFO);
    registry.register_type(GAMEANIMATIONSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(DEMOSETTINGS_TYPE_INFO);
    registry.register_type(DEMOSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(PERFORMANCETRACKERSETTINGS_TYPE_INFO);
    registry.register_type(PERFORMANCETRACKERSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(GAMETIMESETTINGS_TYPE_INFO);
    registry.register_type(GAMETIMESETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(SYNCEDGAMESETTINGS_TYPE_INFO);
    registry.register_type(SYNCEDGAMESETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(GAMESETTINGS_TYPE_INFO);
    registry.register_type(GAMESETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(GAMESETTINGSCOMPONENT_TYPE_INFO);
    registry.register_type(GAMESETTINGSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(LOGFILECOLLISIONMODE_TYPE_INFO);
    registry.register_type(LOGFILECOLLISIONMODE_ARRAY_TYPE_INFO);
    registry.register_type(COREDEBUGREADPROFILEGAMEDATAMESSAGE_TYPE_INFO);
    registry.register_type(COREREADSAVEGAMEDATADONEMESSAGE_TYPE_INFO);
    registry.register_type(COREREADSAVEGAMEDONEMESSAGE_TYPE_INFO);
    registry.register_type(COREREADSAVEGAMEDATAMESSAGE_TYPE_INFO);
    registry.register_type(COREWRITESAVEGAMEDONEMESSAGE_TYPE_INFO);
    registry.register_type(COREWRITESAVEGAMEMESSAGE_TYPE_INFO);
    registry.register_type(COREWRITEPROFILEGAMEMESSAGE_TYPE_INFO);
    registry.register_type(SAVEGAMEHEADERENTRY_TYPE_INFO);
    registry.register_type(SAVEGAMEHEADERENTRY_ARRAY_TYPE_INFO);
    registry.register_type(CORECLIENTPRELOADCOMPLETEMESSAGE_TYPE_INFO);
    registry.register_type(CORECLIENTPRELOADMESSAGE_TYPE_INFO);
    registry.register_type(CORECLIENTPRESAVECOMPLETEMESSAGE_TYPE_INFO);
    registry.register_type(CORECLIENTPRESAVEMESSAGE_TYPE_INFO);
    registry.register_type(LOADGAMELOADREQUESTEDMESSAGE_TYPE_INFO);
    registry.register_type(LOADREQUEST_TYPE_INFO);
    registry.register_type(LOADREQUEST_ARRAY_TYPE_INFO);
    registry.register_type(LOADGAMEBEGINLOADMESSAGE_TYPE_INFO);
    registry.register_type(SAVEGAMEBEGINSAVEMESSAGE_TYPE_INFO);
    registry.register_type(BLUEPRINTBUNDLESTREAMEDINMESSAGE_TYPE_INFO);
    registry.register_type(SESSIONPLAYERJOINEDMESSAGE_TYPE_INFO);
    registry.register_type(SESSIONPLAYERAUTHENTICATEDMESSAGE_TYPE_INFO);
    registry.register_type(SESSIONPLAYERLEFTMESSAGE_TYPE_INFO);
    registry.register_type(COREGAMETIMERMESSAGE_TYPE_INFO);
    registry.register_type(COREEXITINGAMEMESSAGE_TYPE_INFO);
    registry.register_type(COREENTEREDINGAMEMESSAGE_TYPE_INFO);
    registry.register_type(PERFORMANCECLIENTNETWORKMESSAGE_TYPE_INFO);
    registry.register_type(PERFORMANCESERVERNETWORKMESSAGE_TYPE_INFO);
    registry.register_type(PERFORMANCESERVERMESSAGE_TYPE_INFO);
    registry.register_type(PROFILEOPTIONSSETTINGSSAVEDMESSAGE_TYPE_INFO);
    registry.register_type(PROFILEOPTIONSSETTINGSPRESAVEMESSAGE_TYPE_INFO);
    registry.register_type(PROFILEOPTIONSSETTINGSLOADEDMESSAGE_TYPE_INFO);
    registry.register_type(PROFILEOPTIONSAPPLYMESSAGE_TYPE_INFO);
    registry.register_type(PROFILEOPTIONSLOADSTATUS_TYPE_INFO);
    registry.register_type(PROFILEOPTIONSLOADSTATUS_ARRAY_TYPE_INFO);
    registry.register_type(STATISTICSEVENTMESSAGEBASE_TYPE_INFO);
    registry.register_type(NETWORKDISCONNECTEDMESSAGE_TYPE_INFO);
    registry.register_type(NETWORKCONNECTEDMESSAGE_TYPE_INFO);
    registry.register_type(TRANSFORMPARAMETEREVENT_TYPE_INFO);
    registry.register_type(TRANSFORMPARAMETEREVENT_ARRAY_TYPE_INFO);
    registry.register_type(PLAYEREVENTBASE_TYPE_INFO);
    registry.register_type(PLAYEREVENTBASE_ARRAY_TYPE_INFO);
    registry.register_type(INTPARAMETEREVENT_TYPE_INFO);
    registry.register_type(INTPARAMETEREVENT_ARRAY_TYPE_INFO);
    registry.register_type(FLOATPARAMETEREVENT_TYPE_INFO);
    registry.register_type(FLOATPARAMETEREVENT_ARRAY_TYPE_INFO);
    registry.register_type(ENTITYPARAMETEREVENT_TYPE_INFO);
    registry.register_type(ENTITYPARAMETEREVENT_ARRAY_TYPE_INFO);
    registry.register_type(COMPONENTPARAMETEREVENT_TYPE_INFO);
    registry.register_type(COMPONENTPARAMETEREVENT_ARRAY_TYPE_INFO);
    registry.register_type(COLLISIONEVENT_TYPE_INFO);
    registry.register_type(COLLISIONEVENT_ARRAY_TYPE_INFO);
    registry.register_type(HEALTHENTITYFACTORY_TYPE_INFO);
    registry.register_type(HEALTHENTITYFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(HEALTHCOMPONENT_TYPE_INFO);
    registry.register_type(HEALTHCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTDAMAGEINFO_TYPE_INFO);
    registry.register_type(CLIENTDAMAGEINFO_ARRAY_TYPE_INFO);
    registry.register_type(SERVERDAMAGEINFO_TYPE_INFO);
    registry.register_type(SERVERDAMAGEINFO_ARRAY_TYPE_INFO);
    registry.register_type(DAMAGEINFO_TYPE_INFO);
    registry.register_type(DAMAGEINFO_ARRAY_TYPE_INFO);
    registry.register_type(DEFAULTHEALTHCOMPONENT_TYPE_INFO);
    registry.register_type(DEFAULTHEALTHCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(GAMEWORLDRAYCASTER_TYPE_INFO);
    registry.register_type(GAMEWORLDRAYCASTER_ARRAY_TYPE_INFO);
    registry.register_type(GAMECOMPONENTENTITY_TYPE_INFO);
    registry.register_type(GAMECOMPONENTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(GAMECOMPONENT_TYPE_INFO);
    registry.register_type(GAMECOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(PARTCOMPONENTCREATEDENTITYINFO_TYPE_INFO);
    registry.register_type(PARTCOMPONENTCREATEDENTITYINFO_ARRAY_TYPE_INFO);
    registry.register_type(SUBVIEW_TYPE_INFO);
    registry.register_type(SUBVIEW_ARRAY_TYPE_INFO);
    registry.register_type(SHADERPARAMETERENTITY_TYPE_INFO);
    registry.register_type(SHADERPARAMETERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(NETWORKSERVERDIAGNOSTICSMESSAGE_TYPE_INFO);
    registry.register_type(NETWORKCAMERAREPLAYMESSAGE_TYPE_INFO);
    registry.register_type(NETWORKCAMERAFREECAMERAMESSAGE_TYPE_INFO);
    registry.register_type(NETWORKSYNCHRONIZEINTERNETSIMULATIONSTATEMESSAGE_TYPE_INFO);
    registry.register_type(NETWORKCHANGEGAMESETTINGMESSAGE_TYPE_INFO);
    registry.register_type(CHANGEGAMESETTINGTYPE_TYPE_INFO);
    registry.register_type(CHANGEGAMESETTINGTYPE_ARRAY_TYPE_INFO);
    registry.register_type(NETWORKDESTROYLOCALPLAYERMESSAGE_TYPE_INFO);
    registry.register_type(NETWORKCREATEPLAYERMESSAGE_TYPE_INFO);
    registry.register_type(NETWORKTINYEVENTMESSAGE_TYPE_INFO);
    registry.register_type(NETWORKLOADLEVELMESSAGE_TYPE_INFO);
    registry.register_type(NETWORKLEVELLOADEDACKMESSAGE_TYPE_INFO);
    registry.register_type(NETWORKTIMESYNCMESSAGE_TYPE_INFO);
    registry.register_type(LOADLEVELINFO_TYPE_INFO);
    registry.register_type(LOADLEVELINFO_ARRAY_TYPE_INFO);
    registry.register_type(BLUEPRINTBUNDLEPRELOADINFO_TYPE_INFO);
    registry.register_type(BLUEPRINTBUNDLEPRELOADINFO_ARRAY_TYPE_INFO);
    registry.register_type(TINYEVENT_TYPE_INFO);
    registry.register_type(TINYEVENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTTOSERVERLOCALPLAYER_TYPE_INFO);
    registry.register_type(CLIENTTOSERVERLOCALPLAYER_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTTOSERVERLOCALPLAYERVIEW_TYPE_INFO);
    registry.register_type(CLIENTTOSERVERLOCALPLAYERVIEW_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTTOSERVERCONSTANTS_TYPE_INFO);
    registry.register_type(CLIENTTOSERVERCONSTANTS_ARRAY_TYPE_INFO);
    registry.register_type(INPUTMODIFIERENTITYDATA_TYPE_INFO);
    registry.register_type(INPUTMODIFIERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(GAMEPLAYSPAWNREFERENCEOBJECTDATA_TYPE_INFO);
    registry.register_type(GAMEPLAYSPAWNREFERENCEOBJECTDATA_ARRAY_TYPE_INFO);
    registry.register_type(EXTRASPAWNDATA_TYPE_INFO);
    registry.register_type(EXTRASPAWNDATA_ARRAY_TYPE_INFO);
    registry.register_type(GAMEPLAYTEAMENTITYDATA_TYPE_INFO);
    registry.register_type(GAMEPLAYTEAMENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(GAMEPLAYTEAMDATA_TYPE_INFO);
    registry.register_type(GAMEPLAYTEAMDATA_ARRAY_TYPE_INFO);
    registry.register_type(SUBLEVELCOLLECTIONENTITYDATA_TYPE_INFO);
    registry.register_type(SUBLEVELCOLLECTIONENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(DETACHABLESUBWORLDCOLLECTIONBASEDATA_TYPE_INFO);
    registry.register_type(DETACHABLESUBWORLDCOLLECTIONBASEDATA_ARRAY_TYPE_INFO);
    registry.register_type(DEMOCLIENTREADYFORPLAYBACKMESSAGE_TYPE_INFO);
    registry.register_type(NETWORKLOGICFIREDOUBLEPLAYEREVENTMESSAGEBASE_TYPE_INFO);
    registry.register_type(NETWORKLOGICFIREPLAYEREVENTMESSAGEBASE_TYPE_INFO);
    registry.register_type(NETWORKLOGICFIREEVENTMESSAGEBASE_TYPE_INFO);
    registry.register_type(SUBLEVELFROMCLIENTSUBLEVELUNLOADEDMESSAGE_TYPE_INFO);
    registry.register_type(SUBLEVELFROMCLIENTSUBLEVELLOADEDMESSAGE_TYPE_INFO);
    registry.register_type(SUBLEVELFROMCLIENTREQUESTBUNDLEBASELINEMESSAGE_TYPE_INFO);
    registry.register_type(SUBLEVELTOCLIENTDROPBUNDLEBASELINEMESSAGE_TYPE_INFO);
    registry.register_type(SUBLEVELTOCLIENTUNLOADREQUESTSMESSAGE_TYPE_INFO);
    registry.register_type(SUBLEVELTOCLIENTLOADREQUESTSMESSAGE_TYPE_INFO);
    registry.register_type(SUBLEVELBUNDLEINFO_TYPE_INFO);
    registry.register_type(SUBLEVELBUNDLEINFO_ARRAY_TYPE_INFO);
    registry.register_type(SUBLEVELTOCLIENTSUBLEVELNAMEMESSAGE_TYPE_INFO);
    registry.register_type(BUNDLENAMEANDINDEX_TYPE_INFO);
    registry.register_type(BUNDLENAMEANDINDEX_ARRAY_TYPE_INFO);
    registry.register_type(CONTROLLABLETOCOMPONENTSONTELEPORTEDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERCONTROLLABLETOCOMPONENTSPLAYEREXITMESSAGE_TYPE_INFO);
    registry.register_type(SERVERCONTROLLABLETOCOMPONENTSPLAYERENTEREDMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTCONTROLLABLETOCOMPONENTSPLAYEREXITMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTCONTROLLABLETOCOMPONENTSPLAYERENTEREDMESSAGE_TYPE_INFO);
    registry.register_type(CONTROLLABLEENTITYDATA_TYPE_INFO);
    registry.register_type(CONTROLLABLEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENTRYCOMPONENTDATA_TYPE_INFO);
    registry.register_type(ENTRYCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(INPUTCURVEDATA_TYPE_INFO);
    registry.register_type(INPUTCURVEDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENTRYSPOTTINGSETTINGS_TYPE_INFO);
    registry.register_type(ENTRYSPOTTINGSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(ENTRYCOMPONENTHUDDATA_TYPE_INFO);
    registry.register_type(ENTRYCOMPONENTHUDDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENTRYSEATTYPE_TYPE_INFO);
    registry.register_type(ENTRYSEATTYPE_ARRAY_TYPE_INFO);
    registry.register_type(FOVTRANSITIONDATA_TYPE_INFO);
    registry.register_type(FOVTRANSITIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(FOVTRANSITIONTYPE_TYPE_INFO);
    registry.register_type(FOVTRANSITIONTYPE_ARRAY_TYPE_INFO);
    registry.register_type(CAMERATRANSITIONEFFECTDATAPACK_TYPE_INFO);
    registry.register_type(CAMERATRANSITIONEFFECTDATAPACK_ARRAY_TYPE_INFO);
    registry.register_type(CAMERATRANSITIONEFFECTDATA_TYPE_INFO);
    registry.register_type(CAMERATRANSITIONEFFECTDATA_ARRAY_TYPE_INFO);
    registry.register_type(CAMERATRANSITIONPRIORITY_TYPE_INFO);
    registry.register_type(CAMERATRANSITIONPRIORITY_ARRAY_TYPE_INFO);
    registry.register_type(CAMERATRANSITIONEASE_TYPE_INFO);
    registry.register_type(CAMERATRANSITIONEASE_ARRAY_TYPE_INFO);
    registry.register_type(CAMERATRANSITIONTYPE_TYPE_INFO);
    registry.register_type(CAMERATRANSITIONTYPE_ARRAY_TYPE_INFO);
    registry.register_type(TARGETCAMERADATA_TYPE_INFO);
    registry.register_type(TARGETCAMERADATA_ARRAY_TYPE_INFO);
    registry.register_type(GAMEMODEVIEWDEFINITION_TYPE_INFO);
    registry.register_type(GAMEMODEVIEWDEFINITION_ARRAY_TYPE_INFO);
    registry.register_type(PLAYERVIEWDEFINITION_TYPE_INFO);
    registry.register_type(PLAYERVIEWDEFINITION_ARRAY_TYPE_INFO);
    registry.register_type(CAMERAIDS_TYPE_INFO);
    registry.register_type(CAMERAIDS_ARRAY_TYPE_INFO);
    registry.register_type(CAMERADATA_TYPE_INFO);
    registry.register_type(CAMERADATA_ARRAY_TYPE_INFO);
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct TargetCamera {
    pub _glacier_base: Camera,
}

pub trait TargetCameraTrait: CameraTrait {
}

impl TargetCameraTrait for TargetCamera {
}

impl CameraTrait for TargetCamera {
}

pub static TARGETCAMERA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TargetCamera",
    name_hash: 1472465709,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CAMERA_TYPE_INFO),
        super_class_offset: offset_of!(TargetCamera, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TargetCamera as Default>::default())),
            create_boxed: || Box::new(<TargetCamera as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(TARGETCAMERA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TargetCamera {
    fn type_info(&self) -> &'static TypeInfo {
        TARGETCAMERA_TYPE_INFO
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


pub static TARGETCAMERA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TargetCamera-Array",
    name_hash: 2335481753,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("TargetCamera"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TargetCameraCallback {
}

pub trait TargetCameraCallbackTrait: TypeObject {
}

impl TargetCameraCallbackTrait for TargetCameraCallback {
}

pub static TARGETCAMERACALLBACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TargetCameraCallback",
    name_hash: 4206536580,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TargetCameraCallback as Default>::default())),
            create_boxed: || Box::new(<TargetCameraCallback as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(TARGETCAMERACALLBACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TargetCameraCallback {
    fn type_info(&self) -> &'static TypeInfo {
        TARGETCAMERACALLBACK_TYPE_INFO
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


pub static TARGETCAMERACALLBACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TargetCameraCallback-Array",
    name_hash: 3738737584,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("TargetCameraCallback"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct FreeCamera {
    pub _glacier_base: Camera,
}

pub trait FreeCameraTrait: CameraTrait {
}

impl FreeCameraTrait for FreeCamera {
}

impl CameraTrait for FreeCamera {
}

pub static FREECAMERA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FreeCamera",
    name_hash: 1518705000,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CAMERA_TYPE_INFO),
        super_class_offset: offset_of!(FreeCamera, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FreeCamera as Default>::default())),
            create_boxed: || Box::new(<FreeCamera as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(FREECAMERA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FreeCamera {
    fn type_info(&self) -> &'static TypeInfo {
        FREECAMERA_TYPE_INFO
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


pub static FREECAMERA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FreeCamera-Array",
    name_hash: 3894209372,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("FreeCamera"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Camera {
}

pub trait CameraTrait: TypeObject {
}

impl CameraTrait for Camera {
}

pub static CAMERA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Camera",
    name_hash: 2716359356,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Camera as Default>::default())),
            create_boxed: || Box::new(<Camera as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CAMERA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Camera {
    fn type_info(&self) -> &'static TypeInfo {
        CAMERA_TYPE_INFO
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


pub static CAMERA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Camera-Array",
    name_hash: 2431783432,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("Camera"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AntAnimatableEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait AntAnimatableEntityTrait: super::entity::EntityTrait {
}

impl AntAnimatableEntityTrait for AntAnimatableEntity {
}

impl super::entity::EntityTrait for AntAnimatableEntity {
}

impl super::entity::EntityBusPeerTrait for AntAnimatableEntity {
}

pub static ANTANIMATABLEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntAnimatableEntity",
    name_hash: 3034896401,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(AntAnimatableEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AntAnimatableEntity as Default>::default())),
            create_boxed: || Box::new(<AntAnimatableEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(ANTANIMATABLEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AntAnimatableEntity {
    fn type_info(&self) -> &'static TypeInfo {
        ANTANIMATABLEENTITY_TYPE_INFO
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


pub static ANTANIMATABLEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntAnimatableEntity-Array",
    name_hash: 3810728357,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("AntAnimatableEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DifficultyDatasBase {
    pub _glacier_base: super::core::Asset,
}

pub trait DifficultyDatasBaseTrait: super::core::AssetTrait {
}

impl DifficultyDatasBaseTrait for DifficultyDatasBase {
}

impl super::core::AssetTrait for DifficultyDatasBase {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for DifficultyDatasBase {
}

pub static DIFFICULTYDATASBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DifficultyDatasBase",
    name_hash: 1981683296,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(DifficultyDatasBase, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DifficultyDatasBase as Default>::default())),
            create_boxed: || Box::new(<DifficultyDatasBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DIFFICULTYDATASBASE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DifficultyDatasBase {
    fn type_info(&self) -> &'static TypeInfo {
        DIFFICULTYDATASBASE_TYPE_INFO
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


pub static DIFFICULTYDATASBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DifficultyDatasBase-Array",
    name_hash: 1121963092,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("DifficultyDatasBase"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum LockType {
    #[default]
    LockAlways = 0,
    LockOnRadar = 1,
    LockOnHeat = 2,
    LockOnLaserPainted = 3,
    LockNever = 4,
    LockTypeCount = 5,
}

pub static LOCKTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LockType",
    name_hash: 1982079030,
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(LOCKTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LockType {
    fn type_info(&self) -> &'static TypeInfo {
        LOCKTYPE_TYPE_INFO
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


pub static LOCKTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LockType-Array",
    name_hash: 3406107778,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("LockType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ToolData {
    pub _glacier_base: super::core::DataContainer,
    pub is_always_active: bool,
}

pub trait ToolDataTrait: super::core::DataContainerTrait {
    fn is_always_active(&self) -> &bool;
    fn is_always_active_mut(&mut self) -> &mut bool;
}

impl ToolDataTrait for ToolData {
    fn is_always_active(&self) -> &bool {
        &self.is_always_active
    }
    fn is_always_active_mut(&mut self) -> &mut bool {
        &mut self.is_always_active
    }
}

impl super::core::DataContainerTrait for ToolData {
}

pub static TOOLDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ToolData",
    name_hash: 2353971693,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(ToolData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ToolData as Default>::default())),
            create_boxed: || Box::new(<ToolData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "IsAlwaysActive",
                name_hash: 3510629186,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ToolData, is_always_active),
            },
        ],
    }),
    array_type: Some(TOOLDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ToolData {
    fn type_info(&self) -> &'static TypeInfo {
        TOOLDATA_TYPE_INFO
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


pub static TOOLDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ToolData-Array",
    name_hash: 728606425,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ToolData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum VoiceChannel {
    #[default]
    VoiceChannel_Off = 0,
    VoiceChannel_Team = 1,
    VoiceChannel_Squad = 2,
    VoiceChannel_Loopback = 3,
    VoiceChannel_Count = 4,
}

pub static VOICECHANNEL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceChannel",
    name_hash: 1192336432,
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(VOICECHANNEL_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for VoiceChannel {
    fn type_info(&self) -> &'static TypeInfo {
        VOICECHANNEL_TYPE_INFO
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


pub static VOICECHANNEL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceChannel-Array",
    name_hash: 1403480452,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("VoiceChannel"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum SquadId {
    #[default]
    SquadNone = 0,
    Squad1 = 1,
    Squad2 = 2,
    Squad3 = 3,
    Squad4 = 4,
    Squad5 = 5,
    Squad6 = 6,
    Squad7 = 7,
    Squad8 = 8,
    Squad9 = 9,
    Squad10 = 10,
    Squad11 = 11,
    Squad12 = 12,
    Squad13 = 13,
    Squad14 = 14,
    Squad15 = 15,
    Squad16 = 16,
    Squad17 = 17,
    Squad18 = 18,
    Squad19 = 19,
    Squad20 = 20,
    Squad21 = 21,
    Squad22 = 22,
    Squad23 = 23,
    Squad24 = 24,
    Squad25 = 25,
    Squad26 = 26,
    Squad27 = 27,
    Squad28 = 28,
    Squad29 = 29,
    Squad30 = 30,
    Squad31 = 31,
    Squad32 = 32,
    SquadIdCount = 33,
}

pub static SQUADID_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SquadId",
    name_hash: 2312333818,
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(SQUADID_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SquadId {
    fn type_info(&self) -> &'static TypeInfo {
        SQUADID_TYPE_INFO
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


pub static SQUADID_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SquadId-Array",
    name_hash: 3571719374,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("SquadId"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum FactionId {
    #[default]
    FactionNeutral = 0,
    FactionUS = 1,
    FactionRUS = 2,
    FactionMEC = 3,
    FactionIdCount = 4,
    FactionInvalid = 5,
}

pub static FACTIONID_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FactionId",
    name_hash: 4230184176,
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(FACTIONID_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for FactionId {
    fn type_info(&self) -> &'static TypeInfo {
        FACTIONID_TYPE_INFO
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


pub static FACTIONID_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FactionId-Array",
    name_hash: 2039143108,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("FactionId"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum TeamId {
    #[default]
    TeamNeutral = 0,
    Team1 = 1,
    Team2 = 2,
    Team3 = 3,
    Team4 = 4,
    Team5 = 5,
    Team6 = 6,
    Team7 = 7,
    Team8 = 8,
    Team9 = 9,
    Team10 = 10,
    Team11 = 11,
    Team12 = 12,
    Team13 = 13,
    Team14 = 14,
    Team15 = 15,
    Team16 = 16,
    TeamIdCount = 17,
}

pub static TEAMID_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TeamId",
    name_hash: 3220374101,
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(TEAMID_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TeamId {
    fn type_info(&self) -> &'static TypeInfo {
        TEAMID_TYPE_INFO
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


pub static TEAMID_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TeamId-Array",
    name_hash: 3853754977,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("TeamId"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum IKEffectorEnum {
    #[default]
    IKLeftHand = 0,
    IKRightHand = 1,
}

pub static IKEFFECTORENUM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IKEffectorEnum",
    name_hash: 672059102,
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(IKEFFECTORENUM_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for IKEffectorEnum {
    fn type_info(&self) -> &'static TypeInfo {
        IKEFFECTORENUM_TYPE_INFO
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


pub static IKEFFECTORENUM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IKEffectorEnum-Array",
    name_hash: 3456880746,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("IKEffectorEnum"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum RotationAxis {
    #[default]
    raX = 0,
    raY = 1,
    raZ = 2,
}

pub static ROTATIONAXIS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RotationAxis",
    name_hash: 3148542130,
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(ROTATIONAXIS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RotationAxis {
    fn type_info(&self) -> &'static TypeInfo {
        ROTATIONAXIS_TYPE_INFO
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


pub static ROTATIONAXIS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RotationAxis-Array",
    name_hash: 1246140166,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("RotationAxis"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum WheelPhysicsType {
    #[default]
    wptNormal = 0,
    wptFollow = 1,
    wptStatic = 2,
}

pub static WHEELPHYSICSTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WheelPhysicsType",
    name_hash: 1739783333,
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(WHEELPHYSICSTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for WheelPhysicsType {
    fn type_info(&self) -> &'static TypeInfo {
        WHEELPHYSICSTYPE_TYPE_INFO
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


pub static WHEELPHYSICSTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WheelPhysicsType-Array",
    name_hash: 594760977,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("WheelPhysicsType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum EntryClass {
    #[default]
    ecPrimary = 0,
    ecSecondary = 1,
}

pub static ENTRYCLASS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntryClass",
    name_hash: 292493759,
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(ENTRYCLASS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EntryClass {
    fn type_info(&self) -> &'static TypeInfo {
        ENTRYCLASS_TYPE_INFO
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


pub static ENTRYCLASS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntryClass-Array",
    name_hash: 794545931,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("EntryClass"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ShaderParameterComponentData {
    pub _glacier_base: super::entity::GameComponentData,
    pub shader_parameter_vectors: Vec<BoxedTypeObject /* ShaderParameterVector */>,
    pub shader_parameter_texture: Vec<BoxedTypeObject /* ShaderParameterTexture */>,
}

pub trait ShaderParameterComponentDataTrait: super::entity::GameComponentDataTrait {
    fn shader_parameter_vectors(&self) -> &Vec<BoxedTypeObject /* ShaderParameterVector */>;
    fn shader_parameter_vectors_mut(&mut self) -> &mut Vec<BoxedTypeObject /* ShaderParameterVector */>;
    fn shader_parameter_texture(&self) -> &Vec<BoxedTypeObject /* ShaderParameterTexture */>;
    fn shader_parameter_texture_mut(&mut self) -> &mut Vec<BoxedTypeObject /* ShaderParameterTexture */>;
}

impl ShaderParameterComponentDataTrait for ShaderParameterComponentData {
    fn shader_parameter_vectors(&self) -> &Vec<BoxedTypeObject /* ShaderParameterVector */> {
        &self.shader_parameter_vectors
    }
    fn shader_parameter_vectors_mut(&mut self) -> &mut Vec<BoxedTypeObject /* ShaderParameterVector */> {
        &mut self.shader_parameter_vectors
    }
    fn shader_parameter_texture(&self) -> &Vec<BoxedTypeObject /* ShaderParameterTexture */> {
        &self.shader_parameter_texture
    }
    fn shader_parameter_texture_mut(&mut self) -> &mut Vec<BoxedTypeObject /* ShaderParameterTexture */> {
        &mut self.shader_parameter_texture
    }
}

impl super::entity::GameComponentDataTrait for ShaderParameterComponentData {
}

impl super::entity::ComponentDataTrait for ShaderParameterComponentData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
    fn components(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components_mut()
    }
    fn client_index(&self) -> &u8 {
        self._glacier_base.client_index()
    }
    fn client_index_mut(&mut self) -> &mut u8 {
        self._glacier_base.client_index_mut()
    }
    fn server_index(&self) -> &u8 {
        self._glacier_base.server_index()
    }
    fn server_index_mut(&mut self) -> &mut u8 {
        self._glacier_base.server_index_mut()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
    fn excluded_mut(&mut self) -> &mut bool {
        self._glacier_base.excluded_mut()
    }
}

impl super::entity::GameObjectDataTrait for ShaderParameterComponentData {
}

impl super::core::DataBusPeerTrait for ShaderParameterComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ShaderParameterComponentData {
}

impl super::core::DataContainerTrait for ShaderParameterComponentData {
}

pub static SHADERPARAMETERCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterComponentData",
    name_hash: 1368733242,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::GAMECOMPONENTDATA_TYPE_INFO),
        super_class_offset: offset_of!(ShaderParameterComponentData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ShaderParameterComponentData as Default>::default())),
            create_boxed: || Box::new(<ShaderParameterComponentData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ShaderParameterVectors",
                name_hash: 2124466895,
                flags: MemberInfoFlags::new(144),
                field_type: "ShaderParameterVector-Array",
                rust_offset: offset_of!(ShaderParameterComponentData, shader_parameter_vectors),
            },
            FieldInfoData {
                name: "ShaderParameterTexture",
                name_hash: 251018586,
                flags: MemberInfoFlags::new(144),
                field_type: "ShaderParameterTexture-Array",
                rust_offset: offset_of!(ShaderParameterComponentData, shader_parameter_texture),
            },
        ],
    }),
    array_type: Some(SHADERPARAMETERCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ShaderParameterComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        SHADERPARAMETERCOMPONENTDATA_TYPE_INFO
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


pub static SHADERPARAMETERCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterComponentData-Array",
    name_hash: 1504598158,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ShaderParameterComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ShaderParameterTexture {
    pub parameter_name: String,
    pub texture: Option<LockedTypeObject /* super::render_base::TextureBaseAsset */>,
}

pub trait ShaderParameterTextureTrait: TypeObject {
    fn parameter_name(&self) -> &String;
    fn parameter_name_mut(&mut self) -> &mut String;
    fn texture(&self) -> &Option<LockedTypeObject /* super::render_base::TextureBaseAsset */>;
    fn texture_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::TextureBaseAsset */>;
}

impl ShaderParameterTextureTrait for ShaderParameterTexture {
    fn parameter_name(&self) -> &String {
        &self.parameter_name
    }
    fn parameter_name_mut(&mut self) -> &mut String {
        &mut self.parameter_name
    }
    fn texture(&self) -> &Option<LockedTypeObject /* super::render_base::TextureBaseAsset */> {
        &self.texture
    }
    fn texture_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::TextureBaseAsset */> {
        &mut self.texture
    }
}

pub static SHADERPARAMETERTEXTURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterTexture",
    name_hash: 251018586,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ShaderParameterTexture as Default>::default())),
            create_boxed: || Box::new(<ShaderParameterTexture as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ParameterName",
                name_hash: 1568946859,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ShaderParameterTexture, parameter_name),
            },
            FieldInfoData {
                name: "Texture",
                name_hash: 3185041626,
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(ShaderParameterTexture, texture),
            },
        ],
    }),
    array_type: Some(SHADERPARAMETERTEXTURE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ShaderParameterTexture {
    fn type_info(&self) -> &'static TypeInfo {
        SHADERPARAMETERTEXTURE_TYPE_INFO
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


pub static SHADERPARAMETERTEXTURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterTexture-Array",
    name_hash: 3895226606,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ShaderParameterTexture"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ShaderParameterVector {
    pub parameter_name: String,
    pub value: super::core::Vec4,
}

pub trait ShaderParameterVectorTrait: TypeObject {
    fn parameter_name(&self) -> &String;
    fn parameter_name_mut(&mut self) -> &mut String;
    fn value(&self) -> &super::core::Vec4;
    fn value_mut(&mut self) -> &mut super::core::Vec4;
}

impl ShaderParameterVectorTrait for ShaderParameterVector {
    fn parameter_name(&self) -> &String {
        &self.parameter_name
    }
    fn parameter_name_mut(&mut self) -> &mut String {
        &mut self.parameter_name
    }
    fn value(&self) -> &super::core::Vec4 {
        &self.value
    }
    fn value_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.value
    }
}

pub static SHADERPARAMETERVECTOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterVector",
    name_hash: 4229194556,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ShaderParameterVector as Default>::default())),
            create_boxed: || Box::new(<ShaderParameterVector as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ParameterName",
                name_hash: 1568946859,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ShaderParameterVector, parameter_name),
            },
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ShaderParameterVector, value),
            },
        ],
    }),
    array_type: Some(SHADERPARAMETERVECTOR_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ShaderParameterVector {
    fn type_info(&self) -> &'static TypeInfo {
        SHADERPARAMETERVECTOR_TYPE_INFO
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


pub static SHADERPARAMETERVECTOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterVector-Array",
    name_hash: 723679880,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ShaderParameterVector"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ComponentEnabledMessage {
}

pub trait ComponentEnabledMessageTrait: TypeObject {
}

impl ComponentEnabledMessageTrait for ComponentEnabledMessage {
}

pub static COMPONENTENABLEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ComponentEnabledMessage",
    name_hash: 2349682084,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ComponentEnabledMessage as Default>::default())),
            create_boxed: || Box::new(<ComponentEnabledMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ComponentEnabledMessage {
    fn type_info(&self) -> &'static TypeInfo {
        COMPONENTENABLEDMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct ControllableHealthComponentData {
    pub _glacier_base: GameHealthComponentData,
}

pub trait ControllableHealthComponentDataTrait: GameHealthComponentDataTrait {
}

impl ControllableHealthComponentDataTrait for ControllableHealthComponentData {
}

impl GameHealthComponentDataTrait for ControllableHealthComponentData {
}

impl HealthComponentDataTrait for ControllableHealthComponentData {
}

impl super::entity::ComponentDataTrait for ControllableHealthComponentData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
    fn components(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components_mut()
    }
    fn client_index(&self) -> &u8 {
        self._glacier_base.client_index()
    }
    fn client_index_mut(&mut self) -> &mut u8 {
        self._glacier_base.client_index_mut()
    }
    fn server_index(&self) -> &u8 {
        self._glacier_base.server_index()
    }
    fn server_index_mut(&mut self) -> &mut u8 {
        self._glacier_base.server_index_mut()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
    fn excluded_mut(&mut self) -> &mut bool {
        self._glacier_base.excluded_mut()
    }
}

impl super::entity::GameObjectDataTrait for ControllableHealthComponentData {
}

impl super::core::DataBusPeerTrait for ControllableHealthComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ControllableHealthComponentData {
}

impl super::core::DataContainerTrait for ControllableHealthComponentData {
}

pub static CONTROLLABLEHEALTHCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ControllableHealthComponentData",
    name_hash: 3010390439,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMEHEALTHCOMPONENTDATA_TYPE_INFO),
        super_class_offset: offset_of!(ControllableHealthComponentData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ControllableHealthComponentData as Default>::default())),
            create_boxed: || Box::new(<ControllableHealthComponentData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CONTROLLABLEHEALTHCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ControllableHealthComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        CONTROLLABLEHEALTHCOMPONENTDATA_TYPE_INFO
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


pub static CONTROLLABLEHEALTHCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ControllableHealthComponentData-Array",
    name_hash: 1407548179,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ControllableHealthComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct GameHealthComponentData {
    pub _glacier_base: HealthComponentData,
}

pub trait GameHealthComponentDataTrait: HealthComponentDataTrait {
}

impl GameHealthComponentDataTrait for GameHealthComponentData {
}

impl HealthComponentDataTrait for GameHealthComponentData {
}

impl super::entity::ComponentDataTrait for GameHealthComponentData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
    fn components(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components_mut()
    }
    fn client_index(&self) -> &u8 {
        self._glacier_base.client_index()
    }
    fn client_index_mut(&mut self) -> &mut u8 {
        self._glacier_base.client_index_mut()
    }
    fn server_index(&self) -> &u8 {
        self._glacier_base.server_index()
    }
    fn server_index_mut(&mut self) -> &mut u8 {
        self._glacier_base.server_index_mut()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
    fn excluded_mut(&mut self) -> &mut bool {
        self._glacier_base.excluded_mut()
    }
}

impl super::entity::GameObjectDataTrait for GameHealthComponentData {
}

impl super::core::DataBusPeerTrait for GameHealthComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for GameHealthComponentData {
}

impl super::core::DataContainerTrait for GameHealthComponentData {
}

pub static GAMEHEALTHCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameHealthComponentData",
    name_hash: 4180572168,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(HEALTHCOMPONENTDATA_TYPE_INFO),
        super_class_offset: offset_of!(GameHealthComponentData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GameHealthComponentData as Default>::default())),
            create_boxed: || Box::new(<GameHealthComponentData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(GAMEHEALTHCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for GameHealthComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        GAMEHEALTHCOMPONENTDATA_TYPE_INFO
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


pub static GAMEHEALTHCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameHealthComponentData-Array",
    name_hash: 2537978428,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("GameHealthComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DefaultHealthComponentData {
    pub _glacier_base: HealthComponentData,
}

pub trait DefaultHealthComponentDataTrait: HealthComponentDataTrait {
}

impl DefaultHealthComponentDataTrait for DefaultHealthComponentData {
}

impl HealthComponentDataTrait for DefaultHealthComponentData {
}

impl super::entity::ComponentDataTrait for DefaultHealthComponentData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
    fn components(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components_mut()
    }
    fn client_index(&self) -> &u8 {
        self._glacier_base.client_index()
    }
    fn client_index_mut(&mut self) -> &mut u8 {
        self._glacier_base.client_index_mut()
    }
    fn server_index(&self) -> &u8 {
        self._glacier_base.server_index()
    }
    fn server_index_mut(&mut self) -> &mut u8 {
        self._glacier_base.server_index_mut()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
    fn excluded_mut(&mut self) -> &mut bool {
        self._glacier_base.excluded_mut()
    }
}

impl super::entity::GameObjectDataTrait for DefaultHealthComponentData {
}

impl super::core::DataBusPeerTrait for DefaultHealthComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for DefaultHealthComponentData {
}

impl super::core::DataContainerTrait for DefaultHealthComponentData {
}

pub static DEFAULTHEALTHCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DefaultHealthComponentData",
    name_hash: 2737648941,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(HEALTHCOMPONENTDATA_TYPE_INFO),
        super_class_offset: offset_of!(DefaultHealthComponentData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DefaultHealthComponentData as Default>::default())),
            create_boxed: || Box::new(<DefaultHealthComponentData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DEFAULTHEALTHCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DefaultHealthComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        DEFAULTHEALTHCOMPONENTDATA_TYPE_INFO
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


pub static DEFAULTHEALTHCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DefaultHealthComponentData-Array",
    name_hash: 257247641,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("DefaultHealthComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct HealthComponentData {
    pub _glacier_base: super::entity::ComponentData,
}

pub trait HealthComponentDataTrait: super::entity::ComponentDataTrait {
}

impl HealthComponentDataTrait for HealthComponentData {
}

impl super::entity::ComponentDataTrait for HealthComponentData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
    fn components(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components_mut()
    }
    fn client_index(&self) -> &u8 {
        self._glacier_base.client_index()
    }
    fn client_index_mut(&mut self) -> &mut u8 {
        self._glacier_base.client_index_mut()
    }
    fn server_index(&self) -> &u8 {
        self._glacier_base.server_index()
    }
    fn server_index_mut(&mut self) -> &mut u8 {
        self._glacier_base.server_index_mut()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
    fn excluded_mut(&mut self) -> &mut bool {
        self._glacier_base.excluded_mut()
    }
}

impl super::entity::GameObjectDataTrait for HealthComponentData {
}

impl super::core::DataBusPeerTrait for HealthComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for HealthComponentData {
}

impl super::core::DataContainerTrait for HealthComponentData {
}

pub static HEALTHCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthComponentData",
    name_hash: 185100774,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::COMPONENTDATA_TYPE_INFO),
        super_class_offset: offset_of!(HealthComponentData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<HealthComponentData as Default>::default())),
            create_boxed: || Box::new(<HealthComponentData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(HEALTHCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for HealthComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        HEALTHCOMPONENTDATA_TYPE_INFO
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


pub static HEALTHCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthComponentData-Array",
    name_hash: 3104220882,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("HealthComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct HealthComponentOnDamageMessage {
}

pub trait HealthComponentOnDamageMessageTrait: TypeObject {
}

impl HealthComponentOnDamageMessageTrait for HealthComponentOnDamageMessage {
}

pub static HEALTHCOMPONENTONDAMAGEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthComponentOnDamageMessage",
    name_hash: 2866373015,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<HealthComponentOnDamageMessage as Default>::default())),
            create_boxed: || Box::new(<HealthComponentOnDamageMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for HealthComponentOnDamageMessage {
    fn type_info(&self) -> &'static TypeInfo {
        HEALTHCOMPONENTONDAMAGEMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct ControllablePhysicsComponentData {
    pub _glacier_base: GamePhysicsComponentData,
}

pub trait ControllablePhysicsComponentDataTrait: GamePhysicsComponentDataTrait {
}

impl ControllablePhysicsComponentDataTrait for ControllablePhysicsComponentData {
}

impl GamePhysicsComponentDataTrait for ControllablePhysicsComponentData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        self._glacier_base.realm_mut()
    }
    fn effect_parameters(&self) -> &Vec<Option<LockedTypeObject /* super::effect_base::EffectParameter */>> {
        self._glacier_base.effect_parameters()
    }
    fn effect_parameters_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::effect_base::EffectParameter */>> {
        self._glacier_base.effect_parameters_mut()
    }
}

impl super::physics::PhysicsComponentDataTrait for ControllablePhysicsComponentData {
    fn physics_bodies(&self) -> &Vec<Option<LockedTypeObject /* super::physics::PhysicsBodyData */>> {
        self._glacier_base.physics_bodies()
    }
    fn physics_bodies_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::physics::PhysicsBodyData */>> {
        self._glacier_base.physics_bodies_mut()
    }
    fn physics_constraints(&self) -> &Vec<Option<LockedTypeObject /* super::physics::PhysicsConstraintData */>> {
        self._glacier_base.physics_constraints()
    }
    fn physics_constraints_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::physics::PhysicsConstraintData */>> {
        self._glacier_base.physics_constraints_mut()
    }
    fn parts(&self) -> &Vec<BoxedTypeObject /* super::physics::PhysicsPartData */> {
        self._glacier_base.parts()
    }
    fn parts_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::physics::PhysicsPartData */> {
        self._glacier_base.parts_mut()
    }
    fn movable_parts(&self) -> &bool {
        self._glacier_base.movable_parts()
    }
    fn movable_parts_mut(&mut self) -> &mut bool {
        self._glacier_base.movable_parts_mut()
    }
    fn internal_collision_disabling(&self) -> &super::physics::InternalCollisionDisablingBehavior {
        self._glacier_base.internal_collision_disabling()
    }
    fn internal_collision_disabling_mut(&mut self) -> &mut super::physics::InternalCollisionDisablingBehavior {
        self._glacier_base.internal_collision_disabling_mut()
    }
    fn enable_collision_events(&self) -> &bool {
        self._glacier_base.enable_collision_events()
    }
    fn enable_collision_events_mut(&mut self) -> &mut bool {
        self._glacier_base.enable_collision_events_mut()
    }
}

impl super::entity::ComponentDataTrait for ControllablePhysicsComponentData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
    fn components(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components_mut()
    }
    fn client_index(&self) -> &u8 {
        self._glacier_base.client_index()
    }
    fn client_index_mut(&mut self) -> &mut u8 {
        self._glacier_base.client_index_mut()
    }
    fn server_index(&self) -> &u8 {
        self._glacier_base.server_index()
    }
    fn server_index_mut(&mut self) -> &mut u8 {
        self._glacier_base.server_index_mut()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
    fn excluded_mut(&mut self) -> &mut bool {
        self._glacier_base.excluded_mut()
    }
}

impl super::entity::GameObjectDataTrait for ControllablePhysicsComponentData {
}

impl super::core::DataBusPeerTrait for ControllablePhysicsComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ControllablePhysicsComponentData {
}

impl super::core::DataContainerTrait for ControllablePhysicsComponentData {
}

pub static CONTROLLABLEPHYSICSCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ControllablePhysicsComponentData",
    name_hash: 1678795088,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMEPHYSICSCOMPONENTDATA_TYPE_INFO),
        super_class_offset: offset_of!(ControllablePhysicsComponentData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ControllablePhysicsComponentData as Default>::default())),
            create_boxed: || Box::new(<ControllablePhysicsComponentData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CONTROLLABLEPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ControllablePhysicsComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        CONTROLLABLEPHYSICSCOMPONENTDATA_TYPE_INFO
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


pub static CONTROLLABLEPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ControllablePhysicsComponentData-Array",
    name_hash: 2090450916,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ControllablePhysicsComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct GamePhysicsComponentData {
    pub _glacier_base: super::physics::PhysicsComponentData,
    pub realm: super::core::Realm,
    pub effect_parameters: Vec<Option<LockedTypeObject /* super::effect_base::EffectParameter */>>,
}

pub trait GamePhysicsComponentDataTrait: super::physics::PhysicsComponentDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn effect_parameters(&self) -> &Vec<Option<LockedTypeObject /* super::effect_base::EffectParameter */>>;
    fn effect_parameters_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::effect_base::EffectParameter */>>;
}

impl GamePhysicsComponentDataTrait for GamePhysicsComponentData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn effect_parameters(&self) -> &Vec<Option<LockedTypeObject /* super::effect_base::EffectParameter */>> {
        &self.effect_parameters
    }
    fn effect_parameters_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::effect_base::EffectParameter */>> {
        &mut self.effect_parameters
    }
}

impl super::physics::PhysicsComponentDataTrait for GamePhysicsComponentData {
    fn physics_bodies(&self) -> &Vec<Option<LockedTypeObject /* super::physics::PhysicsBodyData */>> {
        self._glacier_base.physics_bodies()
    }
    fn physics_bodies_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::physics::PhysicsBodyData */>> {
        self._glacier_base.physics_bodies_mut()
    }
    fn physics_constraints(&self) -> &Vec<Option<LockedTypeObject /* super::physics::PhysicsConstraintData */>> {
        self._glacier_base.physics_constraints()
    }
    fn physics_constraints_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::physics::PhysicsConstraintData */>> {
        self._glacier_base.physics_constraints_mut()
    }
    fn parts(&self) -> &Vec<BoxedTypeObject /* super::physics::PhysicsPartData */> {
        self._glacier_base.parts()
    }
    fn parts_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::physics::PhysicsPartData */> {
        self._glacier_base.parts_mut()
    }
    fn movable_parts(&self) -> &bool {
        self._glacier_base.movable_parts()
    }
    fn movable_parts_mut(&mut self) -> &mut bool {
        self._glacier_base.movable_parts_mut()
    }
    fn internal_collision_disabling(&self) -> &super::physics::InternalCollisionDisablingBehavior {
        self._glacier_base.internal_collision_disabling()
    }
    fn internal_collision_disabling_mut(&mut self) -> &mut super::physics::InternalCollisionDisablingBehavior {
        self._glacier_base.internal_collision_disabling_mut()
    }
    fn enable_collision_events(&self) -> &bool {
        self._glacier_base.enable_collision_events()
    }
    fn enable_collision_events_mut(&mut self) -> &mut bool {
        self._glacier_base.enable_collision_events_mut()
    }
}

impl super::entity::ComponentDataTrait for GamePhysicsComponentData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
    fn components(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components_mut()
    }
    fn client_index(&self) -> &u8 {
        self._glacier_base.client_index()
    }
    fn client_index_mut(&mut self) -> &mut u8 {
        self._glacier_base.client_index_mut()
    }
    fn server_index(&self) -> &u8 {
        self._glacier_base.server_index()
    }
    fn server_index_mut(&mut self) -> &mut u8 {
        self._glacier_base.server_index_mut()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
    fn excluded_mut(&mut self) -> &mut bool {
        self._glacier_base.excluded_mut()
    }
}

impl super::entity::GameObjectDataTrait for GamePhysicsComponentData {
}

impl super::core::DataBusPeerTrait for GamePhysicsComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for GamePhysicsComponentData {
}

impl super::core::DataContainerTrait for GamePhysicsComponentData {
}

pub static GAMEPHYSICSCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GamePhysicsComponentData",
    name_hash: 1707738015,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::PHYSICSCOMPONENTDATA_TYPE_INFO),
        super_class_offset: offset_of!(GamePhysicsComponentData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GamePhysicsComponentData as Default>::default())),
            create_boxed: || Box::new(<GamePhysicsComponentData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                name_hash: 229961746,
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(GamePhysicsComponentData, realm),
            },
            FieldInfoData {
                name: "EffectParameters",
                name_hash: 929782248,
                flags: MemberInfoFlags::new(144),
                field_type: "EffectParameter-Array",
                rust_offset: offset_of!(GamePhysicsComponentData, effect_parameters),
            },
        ],
    }),
    array_type: Some(GAMEPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for GamePhysicsComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        GAMEPHYSICSCOMPONENTDATA_TYPE_INFO
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


pub static GAMEPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GamePhysicsComponentData-Array",
    name_hash: 1863477291,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("GamePhysicsComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TwoPartComponentData {
    pub _glacier_base: super::entity::GameComponentData,
}

pub trait TwoPartComponentDataTrait: super::entity::GameComponentDataTrait {
}

impl TwoPartComponentDataTrait for TwoPartComponentData {
}

impl super::entity::GameComponentDataTrait for TwoPartComponentData {
}

impl super::entity::ComponentDataTrait for TwoPartComponentData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
    fn components(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components_mut()
    }
    fn client_index(&self) -> &u8 {
        self._glacier_base.client_index()
    }
    fn client_index_mut(&mut self) -> &mut u8 {
        self._glacier_base.client_index_mut()
    }
    fn server_index(&self) -> &u8 {
        self._glacier_base.server_index()
    }
    fn server_index_mut(&mut self) -> &mut u8 {
        self._glacier_base.server_index_mut()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
    fn excluded_mut(&mut self) -> &mut bool {
        self._glacier_base.excluded_mut()
    }
}

impl super::entity::GameObjectDataTrait for TwoPartComponentData {
}

impl super::core::DataBusPeerTrait for TwoPartComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for TwoPartComponentData {
}

impl super::core::DataContainerTrait for TwoPartComponentData {
}

pub static TWOPARTCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TwoPartComponentData",
    name_hash: 2859815969,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::GAMECOMPONENTDATA_TYPE_INFO),
        super_class_offset: offset_of!(TwoPartComponentData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TwoPartComponentData as Default>::default())),
            create_boxed: || Box::new(<TwoPartComponentData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(TWOPARTCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TwoPartComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        TWOPARTCOMPONENTDATA_TYPE_INFO
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


pub static TWOPARTCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TwoPartComponentData-Array",
    name_hash: 1446616725,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("TwoPartComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PartComponentData {
    pub _glacier_base: super::entity::GameComponentData,
    pub health_states: Vec<Option<LockedTypeObject /* HealthStateData */>>,
    pub part_links: Vec<Option<LockedTypeObject /* PartLinkData */>>,
    pub is_supported: bool,
    pub is_fragile: bool,
    pub is_networkable: bool,
    pub animate_physics: bool,
}

pub trait PartComponentDataTrait: super::entity::GameComponentDataTrait {
    fn health_states(&self) -> &Vec<Option<LockedTypeObject /* HealthStateData */>>;
    fn health_states_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* HealthStateData */>>;
    fn part_links(&self) -> &Vec<Option<LockedTypeObject /* PartLinkData */>>;
    fn part_links_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* PartLinkData */>>;
    fn is_supported(&self) -> &bool;
    fn is_supported_mut(&mut self) -> &mut bool;
    fn is_fragile(&self) -> &bool;
    fn is_fragile_mut(&mut self) -> &mut bool;
    fn is_networkable(&self) -> &bool;
    fn is_networkable_mut(&mut self) -> &mut bool;
    fn animate_physics(&self) -> &bool;
    fn animate_physics_mut(&mut self) -> &mut bool;
}

impl PartComponentDataTrait for PartComponentData {
    fn health_states(&self) -> &Vec<Option<LockedTypeObject /* HealthStateData */>> {
        &self.health_states
    }
    fn health_states_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* HealthStateData */>> {
        &mut self.health_states
    }
    fn part_links(&self) -> &Vec<Option<LockedTypeObject /* PartLinkData */>> {
        &self.part_links
    }
    fn part_links_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* PartLinkData */>> {
        &mut self.part_links
    }
    fn is_supported(&self) -> &bool {
        &self.is_supported
    }
    fn is_supported_mut(&mut self) -> &mut bool {
        &mut self.is_supported
    }
    fn is_fragile(&self) -> &bool {
        &self.is_fragile
    }
    fn is_fragile_mut(&mut self) -> &mut bool {
        &mut self.is_fragile
    }
    fn is_networkable(&self) -> &bool {
        &self.is_networkable
    }
    fn is_networkable_mut(&mut self) -> &mut bool {
        &mut self.is_networkable
    }
    fn animate_physics(&self) -> &bool {
        &self.animate_physics
    }
    fn animate_physics_mut(&mut self) -> &mut bool {
        &mut self.animate_physics
    }
}

impl super::entity::GameComponentDataTrait for PartComponentData {
}

impl super::entity::ComponentDataTrait for PartComponentData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
    fn components(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components_mut()
    }
    fn client_index(&self) -> &u8 {
        self._glacier_base.client_index()
    }
    fn client_index_mut(&mut self) -> &mut u8 {
        self._glacier_base.client_index_mut()
    }
    fn server_index(&self) -> &u8 {
        self._glacier_base.server_index()
    }
    fn server_index_mut(&mut self) -> &mut u8 {
        self._glacier_base.server_index_mut()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
    fn excluded_mut(&mut self) -> &mut bool {
        self._glacier_base.excluded_mut()
    }
}

impl super::entity::GameObjectDataTrait for PartComponentData {
}

impl super::core::DataBusPeerTrait for PartComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for PartComponentData {
}

impl super::core::DataContainerTrait for PartComponentData {
}

pub static PARTCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartComponentData",
    name_hash: 4292164845,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::GAMECOMPONENTDATA_TYPE_INFO),
        super_class_offset: offset_of!(PartComponentData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PartComponentData as Default>::default())),
            create_boxed: || Box::new(<PartComponentData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "HealthStates",
                name_hash: 2256734877,
                flags: MemberInfoFlags::new(144),
                field_type: "HealthStateData-Array",
                rust_offset: offset_of!(PartComponentData, health_states),
            },
            FieldInfoData {
                name: "PartLinks",
                name_hash: 3210318177,
                flags: MemberInfoFlags::new(144),
                field_type: "PartLinkData-Array",
                rust_offset: offset_of!(PartComponentData, part_links),
            },
            FieldInfoData {
                name: "IsSupported",
                name_hash: 797903537,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PartComponentData, is_supported),
            },
            FieldInfoData {
                name: "IsFragile",
                name_hash: 3219228557,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PartComponentData, is_fragile),
            },
            FieldInfoData {
                name: "IsNetworkable",
                name_hash: 4000919755,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PartComponentData, is_networkable),
            },
            FieldInfoData {
                name: "AnimatePhysics",
                name_hash: 443997333,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PartComponentData, animate_physics),
            },
        ],
    }),
    array_type: Some(PARTCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PartComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        PARTCOMPONENTDATA_TYPE_INFO
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


pub static PARTCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartComponentData-Array",
    name_hash: 3869532633,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("PartComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PartLinkData {
    pub _glacier_base: super::core::DataContainer,
    pub part_component_index1: u32,
    pub part_component_index2: u32,
}

pub trait PartLinkDataTrait: super::core::DataContainerTrait {
    fn part_component_index1(&self) -> &u32;
    fn part_component_index1_mut(&mut self) -> &mut u32;
    fn part_component_index2(&self) -> &u32;
    fn part_component_index2_mut(&mut self) -> &mut u32;
}

impl PartLinkDataTrait for PartLinkData {
    fn part_component_index1(&self) -> &u32 {
        &self.part_component_index1
    }
    fn part_component_index1_mut(&mut self) -> &mut u32 {
        &mut self.part_component_index1
    }
    fn part_component_index2(&self) -> &u32 {
        &self.part_component_index2
    }
    fn part_component_index2_mut(&mut self) -> &mut u32 {
        &mut self.part_component_index2
    }
}

impl super::core::DataContainerTrait for PartLinkData {
}

pub static PARTLINKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartLinkData",
    name_hash: 2087497698,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(PartLinkData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PartLinkData as Default>::default())),
            create_boxed: || Box::new(<PartLinkData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "PartComponentIndex1",
                name_hash: 830683346,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(PartLinkData, part_component_index1),
            },
            FieldInfoData {
                name: "PartComponentIndex2",
                name_hash: 830683345,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(PartLinkData, part_component_index2),
            },
        ],
    }),
    array_type: Some(PARTLINKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PartLinkData {
    fn type_info(&self) -> &'static TypeInfo {
        PARTLINKDATA_TYPE_INFO
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


pub static PARTLINKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartLinkData-Array",
    name_hash: 4039622358,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("PartLinkData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum PartComponentConstants {
    #[default]
    MaxHealthStateCount = 256,
}

pub static PARTCOMPONENTCONSTANTS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartComponentConstants",
    name_hash: 1485283664,
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(PARTCOMPONENTCONSTANTS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PartComponentConstants {
    fn type_info(&self) -> &'static TypeInfo {
        PARTCOMPONENTCONSTANTS_TYPE_INFO
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


pub static PARTCOMPONENTCONSTANTS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartComponentConstants-Array",
    name_hash: 1031864292,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("PartComponentConstants"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct HealthStateData {
    pub _glacier_base: super::entity::GameObjectData,
    pub objects: Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>>,
    pub loose_part_physics: Vec<Option<LockedTypeObject /* super::physics::NetworkableLoosePartPhysicsData */>>,
    pub spawned_banger_blueprint: Option<LockedTypeObject /* super::entity::ObjectBlueprint */>,
    pub copy_damage_to_banger: bool,
    pub spawned_banger_impulse_params: Option<LockedTypeObject /* BangerSpawnImpulseParams */>,
    pub physics_enabled: bool,
    pub health: f32,
    pub spawn_objects_culling_params: HealthStateSpawnObjectsCullingParams,
    pub part_index: u32,
    pub can_support_other_parts: bool,
}

pub trait HealthStateDataTrait: super::entity::GameObjectDataTrait {
    fn objects(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>>;
    fn objects_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>>;
    fn loose_part_physics(&self) -> &Vec<Option<LockedTypeObject /* super::physics::NetworkableLoosePartPhysicsData */>>;
    fn loose_part_physics_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::physics::NetworkableLoosePartPhysicsData */>>;
    fn spawned_banger_blueprint(&self) -> &Option<LockedTypeObject /* super::entity::ObjectBlueprint */>;
    fn spawned_banger_blueprint_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::ObjectBlueprint */>;
    fn copy_damage_to_banger(&self) -> &bool;
    fn copy_damage_to_banger_mut(&mut self) -> &mut bool;
    fn spawned_banger_impulse_params(&self) -> &Option<LockedTypeObject /* BangerSpawnImpulseParams */>;
    fn spawned_banger_impulse_params_mut(&mut self) -> &mut Option<LockedTypeObject /* BangerSpawnImpulseParams */>;
    fn physics_enabled(&self) -> &bool;
    fn physics_enabled_mut(&mut self) -> &mut bool;
    fn health(&self) -> &f32;
    fn health_mut(&mut self) -> &mut f32;
    fn spawn_objects_culling_params(&self) -> &HealthStateSpawnObjectsCullingParams;
    fn spawn_objects_culling_params_mut(&mut self) -> &mut HealthStateSpawnObjectsCullingParams;
    fn part_index(&self) -> &u32;
    fn part_index_mut(&mut self) -> &mut u32;
    fn can_support_other_parts(&self) -> &bool;
    fn can_support_other_parts_mut(&mut self) -> &mut bool;
}

impl HealthStateDataTrait for HealthStateData {
    fn objects(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        &self.objects
    }
    fn objects_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        &mut self.objects
    }
    fn loose_part_physics(&self) -> &Vec<Option<LockedTypeObject /* super::physics::NetworkableLoosePartPhysicsData */>> {
        &self.loose_part_physics
    }
    fn loose_part_physics_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::physics::NetworkableLoosePartPhysicsData */>> {
        &mut self.loose_part_physics
    }
    fn spawned_banger_blueprint(&self) -> &Option<LockedTypeObject /* super::entity::ObjectBlueprint */> {
        &self.spawned_banger_blueprint
    }
    fn spawned_banger_blueprint_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::ObjectBlueprint */> {
        &mut self.spawned_banger_blueprint
    }
    fn copy_damage_to_banger(&self) -> &bool {
        &self.copy_damage_to_banger
    }
    fn copy_damage_to_banger_mut(&mut self) -> &mut bool {
        &mut self.copy_damage_to_banger
    }
    fn spawned_banger_impulse_params(&self) -> &Option<LockedTypeObject /* BangerSpawnImpulseParams */> {
        &self.spawned_banger_impulse_params
    }
    fn spawned_banger_impulse_params_mut(&mut self) -> &mut Option<LockedTypeObject /* BangerSpawnImpulseParams */> {
        &mut self.spawned_banger_impulse_params
    }
    fn physics_enabled(&self) -> &bool {
        &self.physics_enabled
    }
    fn physics_enabled_mut(&mut self) -> &mut bool {
        &mut self.physics_enabled
    }
    fn health(&self) -> &f32 {
        &self.health
    }
    fn health_mut(&mut self) -> &mut f32 {
        &mut self.health
    }
    fn spawn_objects_culling_params(&self) -> &HealthStateSpawnObjectsCullingParams {
        &self.spawn_objects_culling_params
    }
    fn spawn_objects_culling_params_mut(&mut self) -> &mut HealthStateSpawnObjectsCullingParams {
        &mut self.spawn_objects_culling_params
    }
    fn part_index(&self) -> &u32 {
        &self.part_index
    }
    fn part_index_mut(&mut self) -> &mut u32 {
        &mut self.part_index
    }
    fn can_support_other_parts(&self) -> &bool {
        &self.can_support_other_parts
    }
    fn can_support_other_parts_mut(&mut self) -> &mut bool {
        &mut self.can_support_other_parts
    }
}

impl super::entity::GameObjectDataTrait for HealthStateData {
}

impl super::core::DataBusPeerTrait for HealthStateData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for HealthStateData {
}

impl super::core::DataContainerTrait for HealthStateData {
}

pub static HEALTHSTATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthStateData",
    name_hash: 2709152798,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::GAMEOBJECTDATA_TYPE_INFO),
        super_class_offset: offset_of!(HealthStateData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<HealthStateData as Default>::default())),
            create_boxed: || Box::new(<HealthStateData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Objects",
                name_hash: 105488131,
                flags: MemberInfoFlags::new(144),
                field_type: "GameObjectData-Array",
                rust_offset: offset_of!(HealthStateData, objects),
            },
            FieldInfoData {
                name: "LoosePartPhysics",
                name_hash: 4017113603,
                flags: MemberInfoFlags::new(144),
                field_type: "NetworkableLoosePartPhysicsData-Array",
                rust_offset: offset_of!(HealthStateData, loose_part_physics),
            },
            FieldInfoData {
                name: "SpawnedBangerBlueprint",
                name_hash: 1648406157,
                flags: MemberInfoFlags::new(0),
                field_type: "ObjectBlueprint",
                rust_offset: offset_of!(HealthStateData, spawned_banger_blueprint),
            },
            FieldInfoData {
                name: "CopyDamageToBanger",
                name_hash: 3099666957,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(HealthStateData, copy_damage_to_banger),
            },
            FieldInfoData {
                name: "SpawnedBangerImpulseParams",
                name_hash: 4146360197,
                flags: MemberInfoFlags::new(0),
                field_type: "BangerSpawnImpulseParams",
                rust_offset: offset_of!(HealthStateData, spawned_banger_impulse_params),
            },
            FieldInfoData {
                name: "PhysicsEnabled",
                name_hash: 752388619,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(HealthStateData, physics_enabled),
            },
            FieldInfoData {
                name: "Health",
                name_hash: 3054337113,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HealthStateData, health),
            },
            FieldInfoData {
                name: "SpawnObjectsCullingParams",
                name_hash: 3981404306,
                flags: MemberInfoFlags::new(0),
                field_type: "HealthStateSpawnObjectsCullingParams",
                rust_offset: offset_of!(HealthStateData, spawn_objects_culling_params),
            },
            FieldInfoData {
                name: "PartIndex",
                name_hash: 3213901068,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(HealthStateData, part_index),
            },
            FieldInfoData {
                name: "CanSupportOtherParts",
                name_hash: 3085116102,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(HealthStateData, can_support_other_parts),
            },
        ],
    }),
    array_type: Some(HEALTHSTATEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for HealthStateData {
    fn type_info(&self) -> &'static TypeInfo {
        HEALTHSTATEDATA_TYPE_INFO
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


pub static HEALTHSTATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthStateData-Array",
    name_hash: 1179784490,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("HealthStateData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct HealthStateSpawnObjectsCullingParams {
    pub debris_cull_distance: f32,
    pub debris_cull_distance_outside_view: f32,
    pub sound_cull_distance: f32,
    pub sound_cull_distance_outside_view: f32,
    pub effect_cull_distance: f32,
    pub effect_cull_distance_outside_view: f32,
}

pub trait HealthStateSpawnObjectsCullingParamsTrait: TypeObject {
    fn debris_cull_distance(&self) -> &f32;
    fn debris_cull_distance_mut(&mut self) -> &mut f32;
    fn debris_cull_distance_outside_view(&self) -> &f32;
    fn debris_cull_distance_outside_view_mut(&mut self) -> &mut f32;
    fn sound_cull_distance(&self) -> &f32;
    fn sound_cull_distance_mut(&mut self) -> &mut f32;
    fn sound_cull_distance_outside_view(&self) -> &f32;
    fn sound_cull_distance_outside_view_mut(&mut self) -> &mut f32;
    fn effect_cull_distance(&self) -> &f32;
    fn effect_cull_distance_mut(&mut self) -> &mut f32;
    fn effect_cull_distance_outside_view(&self) -> &f32;
    fn effect_cull_distance_outside_view_mut(&mut self) -> &mut f32;
}

impl HealthStateSpawnObjectsCullingParamsTrait for HealthStateSpawnObjectsCullingParams {
    fn debris_cull_distance(&self) -> &f32 {
        &self.debris_cull_distance
    }
    fn debris_cull_distance_mut(&mut self) -> &mut f32 {
        &mut self.debris_cull_distance
    }
    fn debris_cull_distance_outside_view(&self) -> &f32 {
        &self.debris_cull_distance_outside_view
    }
    fn debris_cull_distance_outside_view_mut(&mut self) -> &mut f32 {
        &mut self.debris_cull_distance_outside_view
    }
    fn sound_cull_distance(&self) -> &f32 {
        &self.sound_cull_distance
    }
    fn sound_cull_distance_mut(&mut self) -> &mut f32 {
        &mut self.sound_cull_distance
    }
    fn sound_cull_distance_outside_view(&self) -> &f32 {
        &self.sound_cull_distance_outside_view
    }
    fn sound_cull_distance_outside_view_mut(&mut self) -> &mut f32 {
        &mut self.sound_cull_distance_outside_view
    }
    fn effect_cull_distance(&self) -> &f32 {
        &self.effect_cull_distance
    }
    fn effect_cull_distance_mut(&mut self) -> &mut f32 {
        &mut self.effect_cull_distance
    }
    fn effect_cull_distance_outside_view(&self) -> &f32 {
        &self.effect_cull_distance_outside_view
    }
    fn effect_cull_distance_outside_view_mut(&mut self) -> &mut f32 {
        &mut self.effect_cull_distance_outside_view
    }
}

pub static HEALTHSTATESPAWNOBJECTSCULLINGPARAMS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthStateSpawnObjectsCullingParams",
    name_hash: 1209550425,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<HealthStateSpawnObjectsCullingParams as Default>::default())),
            create_boxed: || Box::new(<HealthStateSpawnObjectsCullingParams as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "DebrisCullDistance",
                name_hash: 4282615835,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HealthStateSpawnObjectsCullingParams, debris_cull_distance),
            },
            FieldInfoData {
                name: "DebrisCullDistanceOutsideView",
                name_hash: 465150499,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HealthStateSpawnObjectsCullingParams, debris_cull_distance_outside_view),
            },
            FieldInfoData {
                name: "SoundCullDistance",
                name_hash: 3697771219,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HealthStateSpawnObjectsCullingParams, sound_cull_distance),
            },
            FieldInfoData {
                name: "SoundCullDistanceOutsideView",
                name_hash: 1707472235,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HealthStateSpawnObjectsCullingParams, sound_cull_distance_outside_view),
            },
            FieldInfoData {
                name: "EffectCullDistance",
                name_hash: 904738215,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HealthStateSpawnObjectsCullingParams, effect_cull_distance),
            },
            FieldInfoData {
                name: "EffectCullDistanceOutsideView",
                name_hash: 826386719,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HealthStateSpawnObjectsCullingParams, effect_cull_distance_outside_view),
            },
        ],
    }),
    array_type: Some(HEALTHSTATESPAWNOBJECTSCULLINGPARAMS_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for HealthStateSpawnObjectsCullingParams {
    fn type_info(&self) -> &'static TypeInfo {
        HEALTHSTATESPAWNOBJECTSCULLINGPARAMS_TYPE_INFO
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


pub static HEALTHSTATESPAWNOBJECTSCULLINGPARAMS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthStateSpawnObjectsCullingParams-Array",
    name_hash: 3472594029,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("HealthStateSpawnObjectsCullingParams"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct BangerSpawnImpulseParams {
    pub _glacier_base: super::core::DataContainer,
    pub min_horizontal_angle: f32,
    pub max_horizontal_angle: f32,
    pub min_vertical_angle: f32,
    pub max_vertical_angle: f32,
    pub min_strength: f32,
    pub max_strength: f32,
    pub water_strength_modifier: f32,
    pub linear_damping: f32,
    pub angular_damping: f32,
}

pub trait BangerSpawnImpulseParamsTrait: super::core::DataContainerTrait {
    fn min_horizontal_angle(&self) -> &f32;
    fn min_horizontal_angle_mut(&mut self) -> &mut f32;
    fn max_horizontal_angle(&self) -> &f32;
    fn max_horizontal_angle_mut(&mut self) -> &mut f32;
    fn min_vertical_angle(&self) -> &f32;
    fn min_vertical_angle_mut(&mut self) -> &mut f32;
    fn max_vertical_angle(&self) -> &f32;
    fn max_vertical_angle_mut(&mut self) -> &mut f32;
    fn min_strength(&self) -> &f32;
    fn min_strength_mut(&mut self) -> &mut f32;
    fn max_strength(&self) -> &f32;
    fn max_strength_mut(&mut self) -> &mut f32;
    fn water_strength_modifier(&self) -> &f32;
    fn water_strength_modifier_mut(&mut self) -> &mut f32;
    fn linear_damping(&self) -> &f32;
    fn linear_damping_mut(&mut self) -> &mut f32;
    fn angular_damping(&self) -> &f32;
    fn angular_damping_mut(&mut self) -> &mut f32;
}

impl BangerSpawnImpulseParamsTrait for BangerSpawnImpulseParams {
    fn min_horizontal_angle(&self) -> &f32 {
        &self.min_horizontal_angle
    }
    fn min_horizontal_angle_mut(&mut self) -> &mut f32 {
        &mut self.min_horizontal_angle
    }
    fn max_horizontal_angle(&self) -> &f32 {
        &self.max_horizontal_angle
    }
    fn max_horizontal_angle_mut(&mut self) -> &mut f32 {
        &mut self.max_horizontal_angle
    }
    fn min_vertical_angle(&self) -> &f32 {
        &self.min_vertical_angle
    }
    fn min_vertical_angle_mut(&mut self) -> &mut f32 {
        &mut self.min_vertical_angle
    }
    fn max_vertical_angle(&self) -> &f32 {
        &self.max_vertical_angle
    }
    fn max_vertical_angle_mut(&mut self) -> &mut f32 {
        &mut self.max_vertical_angle
    }
    fn min_strength(&self) -> &f32 {
        &self.min_strength
    }
    fn min_strength_mut(&mut self) -> &mut f32 {
        &mut self.min_strength
    }
    fn max_strength(&self) -> &f32 {
        &self.max_strength
    }
    fn max_strength_mut(&mut self) -> &mut f32 {
        &mut self.max_strength
    }
    fn water_strength_modifier(&self) -> &f32 {
        &self.water_strength_modifier
    }
    fn water_strength_modifier_mut(&mut self) -> &mut f32 {
        &mut self.water_strength_modifier
    }
    fn linear_damping(&self) -> &f32 {
        &self.linear_damping
    }
    fn linear_damping_mut(&mut self) -> &mut f32 {
        &mut self.linear_damping
    }
    fn angular_damping(&self) -> &f32 {
        &self.angular_damping
    }
    fn angular_damping_mut(&mut self) -> &mut f32 {
        &mut self.angular_damping
    }
}

impl super::core::DataContainerTrait for BangerSpawnImpulseParams {
}

pub static BANGERSPAWNIMPULSEPARAMS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BangerSpawnImpulseParams",
    name_hash: 205743652,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(BangerSpawnImpulseParams, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BangerSpawnImpulseParams as Default>::default())),
            create_boxed: || Box::new(<BangerSpawnImpulseParams as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "MinHorizontalAngle",
                name_hash: 1294078192,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BangerSpawnImpulseParams, min_horizontal_angle),
            },
            FieldInfoData {
                name: "MaxHorizontalAngle",
                name_hash: 3734684206,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BangerSpawnImpulseParams, max_horizontal_angle),
            },
            FieldInfoData {
                name: "MinVerticalAngle",
                name_hash: 335615388,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BangerSpawnImpulseParams, min_vertical_angle),
            },
            FieldInfoData {
                name: "MaxVerticalAngle",
                name_hash: 3164410690,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BangerSpawnImpulseParams, max_vertical_angle),
            },
            FieldInfoData {
                name: "MinStrength",
                name_hash: 1539610890,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BangerSpawnImpulseParams, min_strength),
            },
            FieldInfoData {
                name: "MaxStrength",
                name_hash: 1356104340,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BangerSpawnImpulseParams, max_strength),
            },
            FieldInfoData {
                name: "WaterStrengthModifier",
                name_hash: 778848386,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BangerSpawnImpulseParams, water_strength_modifier),
            },
            FieldInfoData {
                name: "LinearDamping",
                name_hash: 1796037280,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BangerSpawnImpulseParams, linear_damping),
            },
            FieldInfoData {
                name: "AngularDamping",
                name_hash: 2055444159,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BangerSpawnImpulseParams, angular_damping),
            },
        ],
    }),
    array_type: Some(BANGERSPAWNIMPULSEPARAMS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BangerSpawnImpulseParams {
    fn type_info(&self) -> &'static TypeInfo {
        BANGERSPAWNIMPULSEPARAMS_TYPE_INFO
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


pub static BANGERSPAWNIMPULSEPARAMS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BangerSpawnImpulseParams-Array",
    name_hash: 1060477328,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("BangerSpawnImpulseParams"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PartComponentTrackHealthStateSpawnsMessage {
}

pub trait PartComponentTrackHealthStateSpawnsMessageTrait: TypeObject {
}

impl PartComponentTrackHealthStateSpawnsMessageTrait for PartComponentTrackHealthStateSpawnsMessage {
}

pub static PARTCOMPONENTTRACKHEALTHSTATESPAWNSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartComponentTrackHealthStateSpawnsMessage",
    name_hash: 3004234746,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PartComponentTrackHealthStateSpawnsMessage as Default>::default())),
            create_boxed: || Box::new(<PartComponentTrackHealthStateSpawnsMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for PartComponentTrackHealthStateSpawnsMessage {
    fn type_info(&self) -> &'static TypeInfo {
        PARTCOMPONENTTRACKHEALTHSTATESPAWNSMESSAGE_TYPE_INFO
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

#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum TrackHealthStateSpawns {
    #[default]
    TrackHealthStateSpawns_All = 0,
    TrackHealthStateSpawns_AllButLastHealthState = 1,
    TrackHealthStateSpawns_None = 2,
}

pub static TRACKHEALTHSTATESPAWNS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TrackHealthStateSpawns",
    name_hash: 4153717225,
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(TRACKHEALTHSTATESPAWNS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TrackHealthStateSpawns {
    fn type_info(&self) -> &'static TypeInfo {
        TRACKHEALTHSTATESPAWNS_TYPE_INFO
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


pub static TRACKHEALTHSTATESPAWNS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TrackHealthStateSpawns-Array",
    name_hash: 2921851101,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("TrackHealthStateSpawns"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct BlueprintBundleStatusEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub stream_realm: super::entity::StreamRealm,
    pub bundle_name: String,
}

pub trait BlueprintBundleStatusEntityDataTrait: super::entity::EntityDataTrait {
    fn stream_realm(&self) -> &super::entity::StreamRealm;
    fn stream_realm_mut(&mut self) -> &mut super::entity::StreamRealm;
    fn bundle_name(&self) -> &String;
    fn bundle_name_mut(&mut self) -> &mut String;
}

impl BlueprintBundleStatusEntityDataTrait for BlueprintBundleStatusEntityData {
    fn stream_realm(&self) -> &super::entity::StreamRealm {
        &self.stream_realm
    }
    fn stream_realm_mut(&mut self) -> &mut super::entity::StreamRealm {
        &mut self.stream_realm
    }
    fn bundle_name(&self) -> &String {
        &self.bundle_name
    }
    fn bundle_name_mut(&mut self) -> &mut String {
        &mut self.bundle_name
    }
}

impl super::entity::EntityDataTrait for BlueprintBundleStatusEntityData {
}

impl super::entity::GameObjectDataTrait for BlueprintBundleStatusEntityData {
}

impl super::core::DataBusPeerTrait for BlueprintBundleStatusEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for BlueprintBundleStatusEntityData {
}

impl super::core::DataContainerTrait for BlueprintBundleStatusEntityData {
}

pub static BLUEPRINTBUNDLESTATUSENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundleStatusEntityData",
    name_hash: 348388417,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(BlueprintBundleStatusEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BlueprintBundleStatusEntityData as Default>::default())),
            create_boxed: || Box::new(<BlueprintBundleStatusEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "StreamRealm",
                name_hash: 1844114478,
                flags: MemberInfoFlags::new(0),
                field_type: "StreamRealm",
                rust_offset: offset_of!(BlueprintBundleStatusEntityData, stream_realm),
            },
            FieldInfoData {
                name: "BundleName",
                name_hash: 461157046,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(BlueprintBundleStatusEntityData, bundle_name),
            },
        ],
    }),
    array_type: Some(BLUEPRINTBUNDLESTATUSENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BlueprintBundleStatusEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        BLUEPRINTBUNDLESTATUSENTITYDATA_TYPE_INFO
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


pub static BLUEPRINTBUNDLESTATUSENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundleStatusEntityData-Array",
    name_hash: 84256373,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("BlueprintBundleStatusEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct BlueprintBundleCollectionEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub stream_realm: super::entity::StreamRealm,
    pub collection: Option<LockedTypeObject /* BlueprintBundleCollection */>,
    pub active_index: i32,
}

pub trait BlueprintBundleCollectionEntityDataTrait: super::entity::EntityDataTrait {
    fn stream_realm(&self) -> &super::entity::StreamRealm;
    fn stream_realm_mut(&mut self) -> &mut super::entity::StreamRealm;
    fn collection(&self) -> &Option<LockedTypeObject /* BlueprintBundleCollection */>;
    fn collection_mut(&mut self) -> &mut Option<LockedTypeObject /* BlueprintBundleCollection */>;
    fn active_index(&self) -> &i32;
    fn active_index_mut(&mut self) -> &mut i32;
}

impl BlueprintBundleCollectionEntityDataTrait for BlueprintBundleCollectionEntityData {
    fn stream_realm(&self) -> &super::entity::StreamRealm {
        &self.stream_realm
    }
    fn stream_realm_mut(&mut self) -> &mut super::entity::StreamRealm {
        &mut self.stream_realm
    }
    fn collection(&self) -> &Option<LockedTypeObject /* BlueprintBundleCollection */> {
        &self.collection
    }
    fn collection_mut(&mut self) -> &mut Option<LockedTypeObject /* BlueprintBundleCollection */> {
        &mut self.collection
    }
    fn active_index(&self) -> &i32 {
        &self.active_index
    }
    fn active_index_mut(&mut self) -> &mut i32 {
        &mut self.active_index
    }
}

impl super::entity::EntityDataTrait for BlueprintBundleCollectionEntityData {
}

impl super::entity::GameObjectDataTrait for BlueprintBundleCollectionEntityData {
}

impl super::core::DataBusPeerTrait for BlueprintBundleCollectionEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for BlueprintBundleCollectionEntityData {
}

impl super::core::DataContainerTrait for BlueprintBundleCollectionEntityData {
}

pub static BLUEPRINTBUNDLECOLLECTIONENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundleCollectionEntityData",
    name_hash: 1410618499,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(BlueprintBundleCollectionEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BlueprintBundleCollectionEntityData as Default>::default())),
            create_boxed: || Box::new(<BlueprintBundleCollectionEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "StreamRealm",
                name_hash: 1844114478,
                flags: MemberInfoFlags::new(0),
                field_type: "StreamRealm",
                rust_offset: offset_of!(BlueprintBundleCollectionEntityData, stream_realm),
            },
            FieldInfoData {
                name: "Collection",
                name_hash: 3772761619,
                flags: MemberInfoFlags::new(0),
                field_type: "BlueprintBundleCollection",
                rust_offset: offset_of!(BlueprintBundleCollectionEntityData, collection),
            },
            FieldInfoData {
                name: "ActiveIndex",
                name_hash: 1397492695,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(BlueprintBundleCollectionEntityData, active_index),
            },
        ],
    }),
    array_type: Some(BLUEPRINTBUNDLECOLLECTIONENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BlueprintBundleCollectionEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        BLUEPRINTBUNDLECOLLECTIONENTITYDATA_TYPE_INFO
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


pub static BLUEPRINTBUNDLECOLLECTIONENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundleCollectionEntityData-Array",
    name_hash: 1724098359,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("BlueprintBundleCollectionEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct BlueprintBundleEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub stream_realm: super::entity::StreamRealm,
    pub bundle: BlueprintBundleReference,
}

pub trait BlueprintBundleEntityDataTrait: super::entity::EntityDataTrait {
    fn stream_realm(&self) -> &super::entity::StreamRealm;
    fn stream_realm_mut(&mut self) -> &mut super::entity::StreamRealm;
    fn bundle(&self) -> &BlueprintBundleReference;
    fn bundle_mut(&mut self) -> &mut BlueprintBundleReference;
}

impl BlueprintBundleEntityDataTrait for BlueprintBundleEntityData {
    fn stream_realm(&self) -> &super::entity::StreamRealm {
        &self.stream_realm
    }
    fn stream_realm_mut(&mut self) -> &mut super::entity::StreamRealm {
        &mut self.stream_realm
    }
    fn bundle(&self) -> &BlueprintBundleReference {
        &self.bundle
    }
    fn bundle_mut(&mut self) -> &mut BlueprintBundleReference {
        &mut self.bundle
    }
}

impl super::entity::EntityDataTrait for BlueprintBundleEntityData {
}

impl super::entity::GameObjectDataTrait for BlueprintBundleEntityData {
}

impl super::core::DataBusPeerTrait for BlueprintBundleEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for BlueprintBundleEntityData {
}

impl super::core::DataContainerTrait for BlueprintBundleEntityData {
}

pub static BLUEPRINTBUNDLEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundleEntityData",
    name_hash: 3856015061,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(BlueprintBundleEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BlueprintBundleEntityData as Default>::default())),
            create_boxed: || Box::new(<BlueprintBundleEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "StreamRealm",
                name_hash: 1844114478,
                flags: MemberInfoFlags::new(0),
                field_type: "StreamRealm",
                rust_offset: offset_of!(BlueprintBundleEntityData, stream_realm),
            },
            FieldInfoData {
                name: "Bundle",
                name_hash: 2687094257,
                flags: MemberInfoFlags::new(0),
                field_type: "BlueprintBundleReference",
                rust_offset: offset_of!(BlueprintBundleEntityData, bundle),
            },
        ],
    }),
    array_type: Some(BLUEPRINTBUNDLEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BlueprintBundleEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        BLUEPRINTBUNDLEENTITYDATA_TYPE_INFO
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


pub static BLUEPRINTBUNDLEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundleEntityData-Array",
    name_hash: 2241470177,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("BlueprintBundleEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct RuntimeAssetBlueprint {
    pub _glacier_base: super::entity::Blueprint,
    pub asset: Option<LockedTypeObject /* super::core::Asset */>,
}

pub trait RuntimeAssetBlueprintTrait: super::entity::BlueprintTrait {
    fn asset(&self) -> &Option<LockedTypeObject /* super::core::Asset */>;
    fn asset_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::Asset */>;
}

impl RuntimeAssetBlueprintTrait for RuntimeAssetBlueprint {
    fn asset(&self) -> &Option<LockedTypeObject /* super::core::Asset */> {
        &self.asset
    }
    fn asset_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::Asset */> {
        &mut self.asset
    }
}

impl super::entity::BlueprintTrait for RuntimeAssetBlueprint {
    fn objects(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.objects()
    }
    fn objects_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.objects_mut()
    }
    fn schematics(&self) -> &Option<LockedTypeObject /* super::schematics::SchematicsBaseAsset */> {
        self._glacier_base.schematics()
    }
    fn schematics_mut(&mut self) -> &mut Option<LockedTypeObject /* super::schematics::SchematicsBaseAsset */> {
        self._glacier_base.schematics_mut()
    }
}

impl super::entity::EntityBusDataTrait for RuntimeAssetBlueprint {
    fn event_connections(&self) -> &Vec<BoxedTypeObject /* super::entity::EventConnection */> {
        self._glacier_base.event_connections()
    }
    fn event_connections_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::entity::EventConnection */> {
        self._glacier_base.event_connections_mut()
    }
}

impl super::core::DataBusDataTrait for RuntimeAssetBlueprint {
    fn flags(&self) -> &u16 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.flags_mut()
    }
    fn property_connections(&self) -> &Vec<BoxedTypeObject /* super::core::PropertyConnection */> {
        self._glacier_base.property_connections()
    }
    fn property_connections_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::core::PropertyConnection */> {
        self._glacier_base.property_connections_mut()
    }
    fn link_connections(&self) -> &Vec<BoxedTypeObject /* super::core::LinkConnection */> {
        self._glacier_base.link_connections()
    }
    fn link_connections_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::core::LinkConnection */> {
        self._glacier_base.link_connections_mut()
    }
    fn interface(&self) -> &Option<LockedTypeObject /* super::core::DynamicDataContainer */> {
        self._glacier_base.interface()
    }
    fn interface_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::DynamicDataContainer */> {
        self._glacier_base.interface_mut()
    }
}

impl super::core::AssetTrait for RuntimeAssetBlueprint {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for RuntimeAssetBlueprint {
}

pub static RUNTIMEASSETBLUEPRINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RuntimeAssetBlueprint",
    name_hash: 2432729926,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::BLUEPRINT_TYPE_INFO),
        super_class_offset: offset_of!(RuntimeAssetBlueprint, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RuntimeAssetBlueprint as Default>::default())),
            create_boxed: || Box::new(<RuntimeAssetBlueprint as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Asset",
                name_hash: 205976053,
                flags: MemberInfoFlags::new(0),
                field_type: "Asset",
                rust_offset: offset_of!(RuntimeAssetBlueprint, asset),
            },
        ],
    }),
    array_type: Some(RUNTIMEASSETBLUEPRINT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RuntimeAssetBlueprint {
    fn type_info(&self) -> &'static TypeInfo {
        RUNTIMEASSETBLUEPRINT_TYPE_INFO
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


pub static RUNTIMEASSETBLUEPRINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RuntimeAssetBlueprint-Array",
    name_hash: 3313549554,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("RuntimeAssetBlueprint"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AutoGeneratedBundle {
    pub blueprint_bundle_reference: BlueprintBundleReference,
}

pub trait AutoGeneratedBundleTrait: TypeObject {
    fn blueprint_bundle_reference(&self) -> &BlueprintBundleReference;
    fn blueprint_bundle_reference_mut(&mut self) -> &mut BlueprintBundleReference;
}

impl AutoGeneratedBundleTrait for AutoGeneratedBundle {
    fn blueprint_bundle_reference(&self) -> &BlueprintBundleReference {
        &self.blueprint_bundle_reference
    }
    fn blueprint_bundle_reference_mut(&mut self) -> &mut BlueprintBundleReference {
        &mut self.blueprint_bundle_reference
    }
}

pub static AUTOGENERATEDBUNDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoGeneratedBundle",
    name_hash: 4216184305,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutoGeneratedBundle as Default>::default())),
            create_boxed: || Box::new(<AutoGeneratedBundle as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "BlueprintBundleReference",
                name_hash: 1707452981,
                flags: MemberInfoFlags::new(0),
                field_type: "BlueprintBundleReference",
                rust_offset: offset_of!(AutoGeneratedBundle, blueprint_bundle_reference),
            },
        ],
    }),
    array_type: Some(AUTOGENERATEDBUNDLE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AutoGeneratedBundle {
    fn type_info(&self) -> &'static TypeInfo {
        AUTOGENERATEDBUNDLE_TYPE_INFO
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


pub static AUTOGENERATEDBUNDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoGeneratedBundle-Array",
    name_hash: 1747288261,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("AutoGeneratedBundle"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct BlueprintBundleCollection {
    pub _glacier_base: super::core::Asset,
    pub bundles: Vec<BoxedTypeObject /* BlueprintBundleReference */>,
}

pub trait BlueprintBundleCollectionTrait: super::core::AssetTrait {
    fn bundles(&self) -> &Vec<BoxedTypeObject /* BlueprintBundleReference */>;
    fn bundles_mut(&mut self) -> &mut Vec<BoxedTypeObject /* BlueprintBundleReference */>;
}

impl BlueprintBundleCollectionTrait for BlueprintBundleCollection {
    fn bundles(&self) -> &Vec<BoxedTypeObject /* BlueprintBundleReference */> {
        &self.bundles
    }
    fn bundles_mut(&mut self) -> &mut Vec<BoxedTypeObject /* BlueprintBundleReference */> {
        &mut self.bundles
    }
}

impl super::core::AssetTrait for BlueprintBundleCollection {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for BlueprintBundleCollection {
}

pub static BLUEPRINTBUNDLECOLLECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundleCollection",
    name_hash: 669830440,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(BlueprintBundleCollection, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BlueprintBundleCollection as Default>::default())),
            create_boxed: || Box::new(<BlueprintBundleCollection as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Bundles",
                name_hash: 2774764642,
                flags: MemberInfoFlags::new(144),
                field_type: "BlueprintBundleReference-Array",
                rust_offset: offset_of!(BlueprintBundleCollection, bundles),
            },
        ],
    }),
    array_type: Some(BLUEPRINTBUNDLECOLLECTION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BlueprintBundleCollection {
    fn type_info(&self) -> &'static TypeInfo {
        BLUEPRINTBUNDLECOLLECTION_TYPE_INFO
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


pub static BLUEPRINTBUNDLECOLLECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundleCollection-Array",
    name_hash: 2236608156,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("BlueprintBundleCollection"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct BlueprintBundleReference {
    pub name: String,
    pub settings: BlueprintBundleSettings,
    pub parents: Vec<BoxedTypeObject /* super::entity::SharedBundleReference */>,
    pub contains_controllable: bool,
}

pub trait BlueprintBundleReferenceTrait: TypeObject {
    fn name(&self) -> &String;
    fn name_mut(&mut self) -> &mut String;
    fn settings(&self) -> &BlueprintBundleSettings;
    fn settings_mut(&mut self) -> &mut BlueprintBundleSettings;
    fn parents(&self) -> &Vec<BoxedTypeObject /* super::entity::SharedBundleReference */>;
    fn parents_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::entity::SharedBundleReference */>;
    fn contains_controllable(&self) -> &bool;
    fn contains_controllable_mut(&mut self) -> &mut bool;
}

impl BlueprintBundleReferenceTrait for BlueprintBundleReference {
    fn name(&self) -> &String {
        &self.name
    }
    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    fn settings(&self) -> &BlueprintBundleSettings {
        &self.settings
    }
    fn settings_mut(&mut self) -> &mut BlueprintBundleSettings {
        &mut self.settings
    }
    fn parents(&self) -> &Vec<BoxedTypeObject /* super::entity::SharedBundleReference */> {
        &self.parents
    }
    fn parents_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::entity::SharedBundleReference */> {
        &mut self.parents
    }
    fn contains_controllable(&self) -> &bool {
        &self.contains_controllable
    }
    fn contains_controllable_mut(&mut self) -> &mut bool {
        &mut self.contains_controllable
    }
}

pub static BLUEPRINTBUNDLEREFERENCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundleReference",
    name_hash: 1707452981,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BlueprintBundleReference as Default>::default())),
            create_boxed: || Box::new(<BlueprintBundleReference as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                name_hash: 2088949890,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(BlueprintBundleReference, name),
            },
            FieldInfoData {
                name: "Settings",
                name_hash: 649772672,
                flags: MemberInfoFlags::new(0),
                field_type: "BlueprintBundleSettings",
                rust_offset: offset_of!(BlueprintBundleReference, settings),
            },
            FieldInfoData {
                name: "Parents",
                name_hash: 3887384042,
                flags: MemberInfoFlags::new(144),
                field_type: "SharedBundleReference-Array",
                rust_offset: offset_of!(BlueprintBundleReference, parents),
            },
            FieldInfoData {
                name: "ContainsControllable",
                name_hash: 2307719303,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(BlueprintBundleReference, contains_controllable),
            },
        ],
    }),
    array_type: Some(BLUEPRINTBUNDLEREFERENCE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BlueprintBundleReference {
    fn type_info(&self) -> &'static TypeInfo {
        BLUEPRINTBUNDLEREFERENCE_TYPE_INFO
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


pub static BLUEPRINTBUNDLEREFERENCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundleReference-Array",
    name_hash: 1321527937,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("BlueprintBundleReference"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DynamicBlueprintBundleAsset {
    pub _glacier_base: DynamicBundleAsset,
    pub blueprint_bundle: Option<LockedTypeObject /* BlueprintBundle */>,
}

pub trait DynamicBlueprintBundleAssetTrait: DynamicBundleAssetTrait {
    fn blueprint_bundle(&self) -> &Option<LockedTypeObject /* BlueprintBundle */>;
    fn blueprint_bundle_mut(&mut self) -> &mut Option<LockedTypeObject /* BlueprintBundle */>;
}

impl DynamicBlueprintBundleAssetTrait for DynamicBlueprintBundleAsset {
    fn blueprint_bundle(&self) -> &Option<LockedTypeObject /* BlueprintBundle */> {
        &self.blueprint_bundle
    }
    fn blueprint_bundle_mut(&mut self) -> &mut Option<LockedTypeObject /* BlueprintBundle */> {
        &mut self.blueprint_bundle
    }
}

impl DynamicBundleAssetTrait for DynamicBlueprintBundleAsset {
}

impl super::core::DataContainerPolicyAssetTrait for DynamicBlueprintBundleAsset {
}

impl super::core::AssetTrait for DynamicBlueprintBundleAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for DynamicBlueprintBundleAsset {
}

pub static DYNAMICBLUEPRINTBUNDLEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicBlueprintBundleAsset",
    name_hash: 1748934523,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DYNAMICBUNDLEASSET_TYPE_INFO),
        super_class_offset: offset_of!(DynamicBlueprintBundleAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DynamicBlueprintBundleAsset as Default>::default())),
            create_boxed: || Box::new(<DynamicBlueprintBundleAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "BlueprintBundle",
                name_hash: 1648021886,
                flags: MemberInfoFlags::new(0),
                field_type: "BlueprintBundle",
                rust_offset: offset_of!(DynamicBlueprintBundleAsset, blueprint_bundle),
            },
        ],
    }),
    array_type: Some(DYNAMICBLUEPRINTBUNDLEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DynamicBlueprintBundleAsset {
    fn type_info(&self) -> &'static TypeInfo {
        DYNAMICBLUEPRINTBUNDLEASSET_TYPE_INFO
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


pub static DYNAMICBLUEPRINTBUNDLEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicBlueprintBundleAsset-Array",
    name_hash: 3391944527,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("DynamicBlueprintBundleAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DynamicBundleAsset {
    pub _glacier_base: super::core::DataContainerPolicyAsset,
}

pub trait DynamicBundleAssetTrait: super::core::DataContainerPolicyAssetTrait {
}

impl DynamicBundleAssetTrait for DynamicBundleAsset {
}

impl super::core::DataContainerPolicyAssetTrait for DynamicBundleAsset {
}

impl super::core::AssetTrait for DynamicBundleAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for DynamicBundleAsset {
}

pub static DYNAMICBUNDLEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicBundleAsset",
    name_hash: 1048382900,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINERPOLICYASSET_TYPE_INFO),
        super_class_offset: offset_of!(DynamicBundleAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DynamicBundleAsset as Default>::default())),
            create_boxed: || Box::new(<DynamicBundleAsset as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DYNAMICBUNDLEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DynamicBundleAsset {
    fn type_info(&self) -> &'static TypeInfo {
        DYNAMICBUNDLEASSET_TYPE_INFO
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


pub static DYNAMICBUNDLEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicBundleAsset-Array",
    name_hash: 42051840,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("DynamicBundleAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct BlueprintBundle {
    pub _glacier_base: super::entity::SharedBundleBaseAsset,
    pub blueprint: Option<LockedTypeObject /* super::entity::Blueprint */>,
}

pub trait BlueprintBundleTrait: super::entity::SharedBundleBaseAssetTrait {
    fn blueprint(&self) -> &Option<LockedTypeObject /* super::entity::Blueprint */>;
    fn blueprint_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::Blueprint */>;
}

impl BlueprintBundleTrait for BlueprintBundle {
    fn blueprint(&self) -> &Option<LockedTypeObject /* super::entity::Blueprint */> {
        &self.blueprint
    }
    fn blueprint_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::Blueprint */> {
        &mut self.blueprint
    }
}

impl super::entity::SharedBundleBaseAssetTrait for BlueprintBundle {
}

impl super::core::AssetTrait for BlueprintBundle {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for BlueprintBundle {
}

pub static BLUEPRINTBUNDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundle",
    name_hash: 1648021886,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SHAREDBUNDLEBASEASSET_TYPE_INFO),
        super_class_offset: offset_of!(BlueprintBundle, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BlueprintBundle as Default>::default())),
            create_boxed: || Box::new(<BlueprintBundle as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Blueprint",
                name_hash: 4232469066,
                flags: MemberInfoFlags::new(0),
                field_type: "Blueprint",
                rust_offset: offset_of!(BlueprintBundle, blueprint),
            },
        ],
    }),
    array_type: Some(BLUEPRINTBUNDLE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BlueprintBundle {
    fn type_info(&self) -> &'static TypeInfo {
        BLUEPRINTBUNDLE_TYPE_INFO
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


pub static BLUEPRINTBUNDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundle-Array",
    name_hash: 1801114186,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("BlueprintBundle"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct BlueprintBundleSettings {
    pub heap: super::entity::BundleHeapInfo,
    pub bundle_settings_info: super::entity::BundleSettingsInfo,
    pub dynamic_bundle: bool,
}

pub trait BlueprintBundleSettingsTrait: TypeObject {
    fn heap(&self) -> &super::entity::BundleHeapInfo;
    fn heap_mut(&mut self) -> &mut super::entity::BundleHeapInfo;
    fn bundle_settings_info(&self) -> &super::entity::BundleSettingsInfo;
    fn bundle_settings_info_mut(&mut self) -> &mut super::entity::BundleSettingsInfo;
    fn dynamic_bundle(&self) -> &bool;
    fn dynamic_bundle_mut(&mut self) -> &mut bool;
}

impl BlueprintBundleSettingsTrait for BlueprintBundleSettings {
    fn heap(&self) -> &super::entity::BundleHeapInfo {
        &self.heap
    }
    fn heap_mut(&mut self) -> &mut super::entity::BundleHeapInfo {
        &mut self.heap
    }
    fn bundle_settings_info(&self) -> &super::entity::BundleSettingsInfo {
        &self.bundle_settings_info
    }
    fn bundle_settings_info_mut(&mut self) -> &mut super::entity::BundleSettingsInfo {
        &mut self.bundle_settings_info
    }
    fn dynamic_bundle(&self) -> &bool {
        &self.dynamic_bundle
    }
    fn dynamic_bundle_mut(&mut self) -> &mut bool {
        &mut self.dynamic_bundle
    }
}

pub static BLUEPRINTBUNDLESETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundleSettings",
    name_hash: 4205092795,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BlueprintBundleSettings as Default>::default())),
            create_boxed: || Box::new(<BlueprintBundleSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Heap",
                name_hash: 2089157081,
                flags: MemberInfoFlags::new(0),
                field_type: "BundleHeapInfo",
                rust_offset: offset_of!(BlueprintBundleSettings, heap),
            },
            FieldInfoData {
                name: "BundleSettingsInfo",
                name_hash: 2785571642,
                flags: MemberInfoFlags::new(0),
                field_type: "BundleSettingsInfo",
                rust_offset: offset_of!(BlueprintBundleSettings, bundle_settings_info),
            },
            FieldInfoData {
                name: "DynamicBundle",
                name_hash: 2165366244,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(BlueprintBundleSettings, dynamic_bundle),
            },
        ],
    }),
    array_type: Some(BLUEPRINTBUNDLESETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BlueprintBundleSettings {
    fn type_info(&self) -> &'static TypeInfo {
        BLUEPRINTBUNDLESETTINGS_TYPE_INFO
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


pub static BLUEPRINTBUNDLESETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundleSettings-Array",
    name_hash: 1188964111,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("BlueprintBundleSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AntAnimatableEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub sub_realm: super::entity::SubRealm,
    pub animation_data: AntAnimationHandlerData,
    pub auto_activate: bool,
    pub initial_animation_control_mode: AnimationControlModeEnum,
    pub initial_force_disable_culling: bool,
    pub disable_auto_distance_culling: bool,
    pub interpolation: bool,
    pub joint_output_mode: JointOutputModeEnum,
    pub joint_output_property_ids: Vec<i32>,
    pub use_simple_skeleton_compression: bool,
}

pub trait AntAnimatableEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn sub_realm(&self) -> &super::entity::SubRealm;
    fn sub_realm_mut(&mut self) -> &mut super::entity::SubRealm;
    fn animation_data(&self) -> &AntAnimationHandlerData;
    fn animation_data_mut(&mut self) -> &mut AntAnimationHandlerData;
    fn auto_activate(&self) -> &bool;
    fn auto_activate_mut(&mut self) -> &mut bool;
    fn initial_animation_control_mode(&self) -> &AnimationControlModeEnum;
    fn initial_animation_control_mode_mut(&mut self) -> &mut AnimationControlModeEnum;
    fn initial_force_disable_culling(&self) -> &bool;
    fn initial_force_disable_culling_mut(&mut self) -> &mut bool;
    fn disable_auto_distance_culling(&self) -> &bool;
    fn disable_auto_distance_culling_mut(&mut self) -> &mut bool;
    fn interpolation(&self) -> &bool;
    fn interpolation_mut(&mut self) -> &mut bool;
    fn joint_output_mode(&self) -> &JointOutputModeEnum;
    fn joint_output_mode_mut(&mut self) -> &mut JointOutputModeEnum;
    fn joint_output_property_ids(&self) -> &Vec<i32>;
    fn joint_output_property_ids_mut(&mut self) -> &mut Vec<i32>;
    fn use_simple_skeleton_compression(&self) -> &bool;
    fn use_simple_skeleton_compression_mut(&mut self) -> &mut bool;
}

impl AntAnimatableEntityDataTrait for AntAnimatableEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn sub_realm(&self) -> &super::entity::SubRealm {
        &self.sub_realm
    }
    fn sub_realm_mut(&mut self) -> &mut super::entity::SubRealm {
        &mut self.sub_realm
    }
    fn animation_data(&self) -> &AntAnimationHandlerData {
        &self.animation_data
    }
    fn animation_data_mut(&mut self) -> &mut AntAnimationHandlerData {
        &mut self.animation_data
    }
    fn auto_activate(&self) -> &bool {
        &self.auto_activate
    }
    fn auto_activate_mut(&mut self) -> &mut bool {
        &mut self.auto_activate
    }
    fn initial_animation_control_mode(&self) -> &AnimationControlModeEnum {
        &self.initial_animation_control_mode
    }
    fn initial_animation_control_mode_mut(&mut self) -> &mut AnimationControlModeEnum {
        &mut self.initial_animation_control_mode
    }
    fn initial_force_disable_culling(&self) -> &bool {
        &self.initial_force_disable_culling
    }
    fn initial_force_disable_culling_mut(&mut self) -> &mut bool {
        &mut self.initial_force_disable_culling
    }
    fn disable_auto_distance_culling(&self) -> &bool {
        &self.disable_auto_distance_culling
    }
    fn disable_auto_distance_culling_mut(&mut self) -> &mut bool {
        &mut self.disable_auto_distance_culling
    }
    fn interpolation(&self) -> &bool {
        &self.interpolation
    }
    fn interpolation_mut(&mut self) -> &mut bool {
        &mut self.interpolation
    }
    fn joint_output_mode(&self) -> &JointOutputModeEnum {
        &self.joint_output_mode
    }
    fn joint_output_mode_mut(&mut self) -> &mut JointOutputModeEnum {
        &mut self.joint_output_mode
    }
    fn joint_output_property_ids(&self) -> &Vec<i32> {
        &self.joint_output_property_ids
    }
    fn joint_output_property_ids_mut(&mut self) -> &mut Vec<i32> {
        &mut self.joint_output_property_ids
    }
    fn use_simple_skeleton_compression(&self) -> &bool {
        &self.use_simple_skeleton_compression
    }
    fn use_simple_skeleton_compression_mut(&mut self) -> &mut bool {
        &mut self.use_simple_skeleton_compression
    }
}

impl super::entity::EntityDataTrait for AntAnimatableEntityData {
}

impl super::entity::GameObjectDataTrait for AntAnimatableEntityData {
}

impl super::core::DataBusPeerTrait for AntAnimatableEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for AntAnimatableEntityData {
}

impl super::core::DataContainerTrait for AntAnimatableEntityData {
}

pub static ANTANIMATABLEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntAnimatableEntityData",
    name_hash: 3437971425,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(AntAnimatableEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AntAnimatableEntityData as Default>::default())),
            create_boxed: || Box::new(<AntAnimatableEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                name_hash: 229961746,
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(AntAnimatableEntityData, realm),
            },
            FieldInfoData {
                name: "SubRealm",
                name_hash: 1747841366,
                flags: MemberInfoFlags::new(0),
                field_type: "SubRealm",
                rust_offset: offset_of!(AntAnimatableEntityData, sub_realm),
            },
            FieldInfoData {
                name: "AnimationData",
                name_hash: 2826644739,
                flags: MemberInfoFlags::new(0),
                field_type: "AntAnimationHandlerData",
                rust_offset: offset_of!(AntAnimatableEntityData, animation_data),
            },
            FieldInfoData {
                name: "AutoActivate",
                name_hash: 778899923,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AntAnimatableEntityData, auto_activate),
            },
            FieldInfoData {
                name: "InitialAnimationControlMode",
                name_hash: 3044617865,
                flags: MemberInfoFlags::new(0),
                field_type: "AnimationControlModeEnum",
                rust_offset: offset_of!(AntAnimatableEntityData, initial_animation_control_mode),
            },
            FieldInfoData {
                name: "InitialForceDisableCulling",
                name_hash: 975987268,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AntAnimatableEntityData, initial_force_disable_culling),
            },
            FieldInfoData {
                name: "DisableAutoDistanceCulling",
                name_hash: 2107417835,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AntAnimatableEntityData, disable_auto_distance_culling),
            },
            FieldInfoData {
                name: "Interpolation",
                name_hash: 1700316975,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AntAnimatableEntityData, interpolation),
            },
            FieldInfoData {
                name: "JointOutputMode",
                name_hash: 424205231,
                flags: MemberInfoFlags::new(0),
                field_type: "JointOutputModeEnum",
                rust_offset: offset_of!(AntAnimatableEntityData, joint_output_mode),
            },
            FieldInfoData {
                name: "JointOutputPropertyIds",
                name_hash: 2683361333,
                flags: MemberInfoFlags::new(144),
                field_type: "Int32-Array",
                rust_offset: offset_of!(AntAnimatableEntityData, joint_output_property_ids),
            },
            FieldInfoData {
                name: "UseSimpleSkeletonCompression",
                name_hash: 2552906087,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AntAnimatableEntityData, use_simple_skeleton_compression),
            },
        ],
    }),
    array_type: Some(ANTANIMATABLEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AntAnimatableEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        ANTANIMATABLEENTITYDATA_TYPE_INFO
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


pub static ANTANIMATABLEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntAnimatableEntityData-Array",
    name_hash: 152543957,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("AntAnimatableEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AntAnimatableComponentMeshData {
    pub mesh_render_type: AntAnimatableComponentMeshRenderType,
    pub mesh_render_context: AntAnimatableComponentMeshRenderContext,
    pub mesh: Option<LockedTypeObject /* super::render_base::MeshBaseAsset */>,
    pub camera_relative_scale_x: f32,
    pub camera_relative_scale_y: f32,
    pub camera_relative_scale_z: f32,
    pub camera_relative_offset_x: f32,
    pub camera_relative_offset_y: f32,
    pub camera_relative_offset_z: f32,
}

pub trait AntAnimatableComponentMeshDataTrait: TypeObject {
    fn mesh_render_type(&self) -> &AntAnimatableComponentMeshRenderType;
    fn mesh_render_type_mut(&mut self) -> &mut AntAnimatableComponentMeshRenderType;
    fn mesh_render_context(&self) -> &AntAnimatableComponentMeshRenderContext;
    fn mesh_render_context_mut(&mut self) -> &mut AntAnimatableComponentMeshRenderContext;
    fn mesh(&self) -> &Option<LockedTypeObject /* super::render_base::MeshBaseAsset */>;
    fn mesh_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::MeshBaseAsset */>;
    fn camera_relative_scale_x(&self) -> &f32;
    fn camera_relative_scale_x_mut(&mut self) -> &mut f32;
    fn camera_relative_scale_y(&self) -> &f32;
    fn camera_relative_scale_y_mut(&mut self) -> &mut f32;
    fn camera_relative_scale_z(&self) -> &f32;
    fn camera_relative_scale_z_mut(&mut self) -> &mut f32;
    fn camera_relative_offset_x(&self) -> &f32;
    fn camera_relative_offset_x_mut(&mut self) -> &mut f32;
    fn camera_relative_offset_y(&self) -> &f32;
    fn camera_relative_offset_y_mut(&mut self) -> &mut f32;
    fn camera_relative_offset_z(&self) -> &f32;
    fn camera_relative_offset_z_mut(&mut self) -> &mut f32;
}

impl AntAnimatableComponentMeshDataTrait for AntAnimatableComponentMeshData {
    fn mesh_render_type(&self) -> &AntAnimatableComponentMeshRenderType {
        &self.mesh_render_type
    }
    fn mesh_render_type_mut(&mut self) -> &mut AntAnimatableComponentMeshRenderType {
        &mut self.mesh_render_type
    }
    fn mesh_render_context(&self) -> &AntAnimatableComponentMeshRenderContext {
        &self.mesh_render_context
    }
    fn mesh_render_context_mut(&mut self) -> &mut AntAnimatableComponentMeshRenderContext {
        &mut self.mesh_render_context
    }
    fn mesh(&self) -> &Option<LockedTypeObject /* super::render_base::MeshBaseAsset */> {
        &self.mesh
    }
    fn mesh_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::MeshBaseAsset */> {
        &mut self.mesh
    }
    fn camera_relative_scale_x(&self) -> &f32 {
        &self.camera_relative_scale_x
    }
    fn camera_relative_scale_x_mut(&mut self) -> &mut f32 {
        &mut self.camera_relative_scale_x
    }
    fn camera_relative_scale_y(&self) -> &f32 {
        &self.camera_relative_scale_y
    }
    fn camera_relative_scale_y_mut(&mut self) -> &mut f32 {
        &mut self.camera_relative_scale_y
    }
    fn camera_relative_scale_z(&self) -> &f32 {
        &self.camera_relative_scale_z
    }
    fn camera_relative_scale_z_mut(&mut self) -> &mut f32 {
        &mut self.camera_relative_scale_z
    }
    fn camera_relative_offset_x(&self) -> &f32 {
        &self.camera_relative_offset_x
    }
    fn camera_relative_offset_x_mut(&mut self) -> &mut f32 {
        &mut self.camera_relative_offset_x
    }
    fn camera_relative_offset_y(&self) -> &f32 {
        &self.camera_relative_offset_y
    }
    fn camera_relative_offset_y_mut(&mut self) -> &mut f32 {
        &mut self.camera_relative_offset_y
    }
    fn camera_relative_offset_z(&self) -> &f32 {
        &self.camera_relative_offset_z
    }
    fn camera_relative_offset_z_mut(&mut self) -> &mut f32 {
        &mut self.camera_relative_offset_z
    }
}

pub static ANTANIMATABLECOMPONENTMESHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntAnimatableComponentMeshData",
    name_hash: 3218525606,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AntAnimatableComponentMeshData as Default>::default())),
            create_boxed: || Box::new(<AntAnimatableComponentMeshData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "MeshRenderType",
                name_hash: 1947649988,
                flags: MemberInfoFlags::new(0),
                field_type: "AntAnimatableComponentMeshRenderType",
                rust_offset: offset_of!(AntAnimatableComponentMeshData, mesh_render_type),
            },
            FieldInfoData {
                name: "MeshRenderContext",
                name_hash: 3147320067,
                flags: MemberInfoFlags::new(0),
                field_type: "AntAnimatableComponentMeshRenderContext",
                rust_offset: offset_of!(AntAnimatableComponentMeshData, mesh_render_context),
            },
            FieldInfoData {
                name: "Mesh",
                name_hash: 2088783990,
                flags: MemberInfoFlags::new(0),
                field_type: "MeshBaseAsset",
                rust_offset: offset_of!(AntAnimatableComponentMeshData, mesh),
            },
            FieldInfoData {
                name: "CameraRelativeScaleX",
                name_hash: 1274780264,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AntAnimatableComponentMeshData, camera_relative_scale_x),
            },
            FieldInfoData {
                name: "CameraRelativeScaleY",
                name_hash: 1274780265,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AntAnimatableComponentMeshData, camera_relative_scale_y),
            },
            FieldInfoData {
                name: "CameraRelativeScaleZ",
                name_hash: 1274780266,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AntAnimatableComponentMeshData, camera_relative_scale_z),
            },
            FieldInfoData {
                name: "CameraRelativeOffsetX",
                name_hash: 3164565469,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AntAnimatableComponentMeshData, camera_relative_offset_x),
            },
            FieldInfoData {
                name: "CameraRelativeOffsetY",
                name_hash: 3164565468,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AntAnimatableComponentMeshData, camera_relative_offset_y),
            },
            FieldInfoData {
                name: "CameraRelativeOffsetZ",
                name_hash: 3164565471,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AntAnimatableComponentMeshData, camera_relative_offset_z),
            },
        ],
    }),
    array_type: Some(ANTANIMATABLECOMPONENTMESHDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AntAnimatableComponentMeshData {
    fn type_info(&self) -> &'static TypeInfo {
        ANTANIMATABLECOMPONENTMESHDATA_TYPE_INFO
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


pub static ANTANIMATABLECOMPONENTMESHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntAnimatableComponentMeshData-Array",
    name_hash: 3671136786,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("AntAnimatableComponentMeshData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum AnimationControlModeEnum {
    #[default]
    AnimationControlModeEnum_PlaceTrajectoryAtObject = 0,
    AnimationControlModeEnum_PlaceObjectAtTrajectory = 1,
    AnimationControlModeEnum_DisconnectTrajectoryAndObject = 2,
}

pub static ANIMATIONCONTROLMODEENUM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimationControlModeEnum",
    name_hash: 1955329636,
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(ANIMATIONCONTROLMODEENUM_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AnimationControlModeEnum {
    fn type_info(&self) -> &'static TypeInfo {
        ANIMATIONCONTROLMODEENUM_TYPE_INFO
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


pub static ANIMATIONCONTROLMODEENUM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimationControlModeEnum-Array",
    name_hash: 3044366928,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("AnimationControlModeEnum"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum JointOutputModeEnum {
    #[default]
    JointOutputModeEnum_NoOutput = 0,
    JointOutputModeEnum_AllJoints = 1,
    JointOutputModeEnum_GameplayBones = 2,
    JointOutputModeEnum_NoOutputUpdateCulled = 3,
    JointOutputModeEnum_GameplayBonesUpdateCulled = 4,
}

pub static JOINTOUTPUTMODEENUM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "JointOutputModeEnum",
    name_hash: 79620572,
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(JOINTOUTPUTMODEENUM_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for JointOutputModeEnum {
    fn type_info(&self) -> &'static TypeInfo {
        JOINTOUTPUTMODEENUM_TYPE_INFO
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


pub static JOINTOUTPUTMODEENUM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "JointOutputModeEnum-Array",
    name_hash: 954970728,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("JointOutputModeEnum"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum AntAnimatableComponentMeshRenderContext {
    #[default]
    AntAnimatableComponentMeshRenderContext_World = 0,
    AntAnimatableComponentMeshRenderContext_Foreground = 1,
    AntAnimatableComponentMeshRenderContext_CameraRelative = 2,
}

pub static ANTANIMATABLECOMPONENTMESHRENDERCONTEXT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntAnimatableComponentMeshRenderContext",
    name_hash: 282498659,
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(ANTANIMATABLECOMPONENTMESHRENDERCONTEXT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AntAnimatableComponentMeshRenderContext {
    fn type_info(&self) -> &'static TypeInfo {
        ANTANIMATABLECOMPONENTMESHRENDERCONTEXT_TYPE_INFO
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


pub static ANTANIMATABLECOMPONENTMESHRENDERCONTEXT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntAnimatableComponentMeshRenderContext-Array",
    name_hash: 2794457687,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("AntAnimatableComponentMeshRenderContext"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum AntAnimatableComponentMeshRenderType {
    #[default]
    AntAnimatableComponentMeshRenderType_Default = 0,
    AntAnimatableComponentMeshRenderType_ShadowOnly = 1,
    AntAnimatableComponentMeshRenderType_NoShadow = 2,
}

pub static ANTANIMATABLECOMPONENTMESHRENDERTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntAnimatableComponentMeshRenderType",
    name_hash: 1253473892,
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(ANTANIMATABLECOMPONENTMESHRENDERTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AntAnimatableComponentMeshRenderType {
    fn type_info(&self) -> &'static TypeInfo {
        ANTANIMATABLECOMPONENTMESHRENDERTYPE_TYPE_INFO
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


pub static ANTANIMATABLECOMPONENTMESHRENDERTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntAnimatableComponentMeshRenderType-Array",
    name_hash: 4083540560,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("AntAnimatableComponentMeshRenderType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AntAnimationHandlerData {
    pub animatable: super::ant::AntAnimatableData,
    pub root_controller: super::ant::AntRef,
    pub lod_binding: LodBinding,
    pub animation_control_binding: AnimationControlBinding,
    pub lod_distance_scale: f32,
    pub report_back_from_ant: bool,
    pub enable_master_slave_copy: bool,
    pub force_server_as_slave: bool,
    pub is_prop: bool,
    pub joint_output_field_hashes: Vec<i32>,
    pub joint_enabled_field_hashes: Vec<i32>,
}

pub trait AntAnimationHandlerDataTrait: TypeObject {
    fn animatable(&self) -> &super::ant::AntAnimatableData;
    fn animatable_mut(&mut self) -> &mut super::ant::AntAnimatableData;
    fn root_controller(&self) -> &super::ant::AntRef;
    fn root_controller_mut(&mut self) -> &mut super::ant::AntRef;
    fn lod_binding(&self) -> &LodBinding;
    fn lod_binding_mut(&mut self) -> &mut LodBinding;
    fn animation_control_binding(&self) -> &AnimationControlBinding;
    fn animation_control_binding_mut(&mut self) -> &mut AnimationControlBinding;
    fn lod_distance_scale(&self) -> &f32;
    fn lod_distance_scale_mut(&mut self) -> &mut f32;
    fn report_back_from_ant(&self) -> &bool;
    fn report_back_from_ant_mut(&mut self) -> &mut bool;
    fn enable_master_slave_copy(&self) -> &bool;
    fn enable_master_slave_copy_mut(&mut self) -> &mut bool;
    fn force_server_as_slave(&self) -> &bool;
    fn force_server_as_slave_mut(&mut self) -> &mut bool;
    fn is_prop(&self) -> &bool;
    fn is_prop_mut(&mut self) -> &mut bool;
    fn joint_output_field_hashes(&self) -> &Vec<i32>;
    fn joint_output_field_hashes_mut(&mut self) -> &mut Vec<i32>;
    fn joint_enabled_field_hashes(&self) -> &Vec<i32>;
    fn joint_enabled_field_hashes_mut(&mut self) -> &mut Vec<i32>;
}

impl AntAnimationHandlerDataTrait for AntAnimationHandlerData {
    fn animatable(&self) -> &super::ant::AntAnimatableData {
        &self.animatable
    }
    fn animatable_mut(&mut self) -> &mut super::ant::AntAnimatableData {
        &mut self.animatable
    }
    fn root_controller(&self) -> &super::ant::AntRef {
        &self.root_controller
    }
    fn root_controller_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.root_controller
    }
    fn lod_binding(&self) -> &LodBinding {
        &self.lod_binding
    }
    fn lod_binding_mut(&mut self) -> &mut LodBinding {
        &mut self.lod_binding
    }
    fn animation_control_binding(&self) -> &AnimationControlBinding {
        &self.animation_control_binding
    }
    fn animation_control_binding_mut(&mut self) -> &mut AnimationControlBinding {
        &mut self.animation_control_binding
    }
    fn lod_distance_scale(&self) -> &f32 {
        &self.lod_distance_scale
    }
    fn lod_distance_scale_mut(&mut self) -> &mut f32 {
        &mut self.lod_distance_scale
    }
    fn report_back_from_ant(&self) -> &bool {
        &self.report_back_from_ant
    }
    fn report_back_from_ant_mut(&mut self) -> &mut bool {
        &mut self.report_back_from_ant
    }
    fn enable_master_slave_copy(&self) -> &bool {
        &self.enable_master_slave_copy
    }
    fn enable_master_slave_copy_mut(&mut self) -> &mut bool {
        &mut self.enable_master_slave_copy
    }
    fn force_server_as_slave(&self) -> &bool {
        &self.force_server_as_slave
    }
    fn force_server_as_slave_mut(&mut self) -> &mut bool {
        &mut self.force_server_as_slave
    }
    fn is_prop(&self) -> &bool {
        &self.is_prop
    }
    fn is_prop_mut(&mut self) -> &mut bool {
        &mut self.is_prop
    }
    fn joint_output_field_hashes(&self) -> &Vec<i32> {
        &self.joint_output_field_hashes
    }
    fn joint_output_field_hashes_mut(&mut self) -> &mut Vec<i32> {
        &mut self.joint_output_field_hashes
    }
    fn joint_enabled_field_hashes(&self) -> &Vec<i32> {
        &self.joint_enabled_field_hashes
    }
    fn joint_enabled_field_hashes_mut(&mut self) -> &mut Vec<i32> {
        &mut self.joint_enabled_field_hashes
    }
}

pub static ANTANIMATIONHANDLERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntAnimationHandlerData",
    name_hash: 1160110016,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AntAnimationHandlerData as Default>::default())),
            create_boxed: || Box::new(<AntAnimationHandlerData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Animatable",
                name_hash: 3208693873,
                flags: MemberInfoFlags::new(0),
                field_type: "AntAnimatableData",
                rust_offset: offset_of!(AntAnimationHandlerData, animatable),
            },
            FieldInfoData {
                name: "RootController",
                name_hash: 3403232383,
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(AntAnimationHandlerData, root_controller),
            },
            FieldInfoData {
                name: "LodBinding",
                name_hash: 3718483171,
                flags: MemberInfoFlags::new(0),
                field_type: "LodBinding",
                rust_offset: offset_of!(AntAnimationHandlerData, lod_binding),
            },
            FieldInfoData {
                name: "AnimationControlBinding",
                name_hash: 2597118709,
                flags: MemberInfoFlags::new(0),
                field_type: "AnimationControlBinding",
                rust_offset: offset_of!(AntAnimationHandlerData, animation_control_binding),
            },
            FieldInfoData {
                name: "LodDistanceScale",
                name_hash: 3582099609,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AntAnimationHandlerData, lod_distance_scale),
            },
            FieldInfoData {
                name: "ReportBackFromAnt",
                name_hash: 3577562349,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AntAnimationHandlerData, report_back_from_ant),
            },
            FieldInfoData {
                name: "EnableMasterSlaveCopy",
                name_hash: 4006836336,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AntAnimationHandlerData, enable_master_slave_copy),
            },
            FieldInfoData {
                name: "ForceServerAsSlave",
                name_hash: 4122425698,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AntAnimationHandlerData, force_server_as_slave),
            },
            FieldInfoData {
                name: "IsProp",
                name_hash: 2816600898,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AntAnimationHandlerData, is_prop),
            },
            FieldInfoData {
                name: "JointOutputFieldHashes",
                name_hash: 4135396458,
                flags: MemberInfoFlags::new(144),
                field_type: "Int32-Array",
                rust_offset: offset_of!(AntAnimationHandlerData, joint_output_field_hashes),
            },
            FieldInfoData {
                name: "JointEnabledFieldHashes",
                name_hash: 1623771088,
                flags: MemberInfoFlags::new(144),
                field_type: "Int32-Array",
                rust_offset: offset_of!(AntAnimationHandlerData, joint_enabled_field_hashes),
            },
        ],
    }),
    array_type: Some(ANTANIMATIONHANDLERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AntAnimationHandlerData {
    fn type_info(&self) -> &'static TypeInfo {
        ANTANIMATIONHANDLERDATA_TYPE_INFO
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


pub static ANTANIMATIONHANDLERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntAnimationHandlerData-Array",
    name_hash: 4041299572,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("AntAnimationHandlerData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AnimationControlBinding {
    pub animation_velocity_to_physics: super::ant::AntRef,
}

pub trait AnimationControlBindingTrait: TypeObject {
    fn animation_velocity_to_physics(&self) -> &super::ant::AntRef;
    fn animation_velocity_to_physics_mut(&mut self) -> &mut super::ant::AntRef;
}

impl AnimationControlBindingTrait for AnimationControlBinding {
    fn animation_velocity_to_physics(&self) -> &super::ant::AntRef {
        &self.animation_velocity_to_physics
    }
    fn animation_velocity_to_physics_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.animation_velocity_to_physics
    }
}

pub static ANIMATIONCONTROLBINDING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimationControlBinding",
    name_hash: 2597118709,
    flags: MemberInfoFlags::new(32841),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AnimationControlBinding as Default>::default())),
            create_boxed: || Box::new(<AnimationControlBinding as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "AnimationVelocityToPhysics",
                name_hash: 2796900084,
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(AnimationControlBinding, animation_velocity_to_physics),
            },
        ],
    }),
    array_type: Some(ANIMATIONCONTROLBINDING_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for AnimationControlBinding {
    fn type_info(&self) -> &'static TypeInfo {
        ANIMATIONCONTROLBINDING_TYPE_INFO
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


pub static ANIMATIONCONTROLBINDING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimationControlBinding-Array",
    name_hash: 1867772865,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("AnimationControlBinding"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LodBinding {
    pub distance_from_camera: super::ant::AntRef,
    pub pixel_size: super::ant::AntRef,
    pub had_visual_update: super::ant::AntRef,
    pub reset_controller: super::ant::AntRef,
}

pub trait LodBindingTrait: TypeObject {
    fn distance_from_camera(&self) -> &super::ant::AntRef;
    fn distance_from_camera_mut(&mut self) -> &mut super::ant::AntRef;
    fn pixel_size(&self) -> &super::ant::AntRef;
    fn pixel_size_mut(&mut self) -> &mut super::ant::AntRef;
    fn had_visual_update(&self) -> &super::ant::AntRef;
    fn had_visual_update_mut(&mut self) -> &mut super::ant::AntRef;
    fn reset_controller(&self) -> &super::ant::AntRef;
    fn reset_controller_mut(&mut self) -> &mut super::ant::AntRef;
}

impl LodBindingTrait for LodBinding {
    fn distance_from_camera(&self) -> &super::ant::AntRef {
        &self.distance_from_camera
    }
    fn distance_from_camera_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.distance_from_camera
    }
    fn pixel_size(&self) -> &super::ant::AntRef {
        &self.pixel_size
    }
    fn pixel_size_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.pixel_size
    }
    fn had_visual_update(&self) -> &super::ant::AntRef {
        &self.had_visual_update
    }
    fn had_visual_update_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.had_visual_update
    }
    fn reset_controller(&self) -> &super::ant::AntRef {
        &self.reset_controller
    }
    fn reset_controller_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.reset_controller
    }
}

pub static LODBINDING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LodBinding",
    name_hash: 3718483171,
    flags: MemberInfoFlags::new(32841),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LodBinding as Default>::default())),
            create_boxed: || Box::new(<LodBinding as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "DistanceFromCamera",
                name_hash: 621427369,
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(LodBinding, distance_from_camera),
            },
            FieldInfoData {
                name: "PixelSize",
                name_hash: 854589352,
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(LodBinding, pixel_size),
            },
            FieldInfoData {
                name: "HadVisualUpdate",
                name_hash: 763719981,
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(LodBinding, had_visual_update),
            },
            FieldInfoData {
                name: "ResetController",
                name_hash: 602369868,
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(LodBinding, reset_controller),
            },
        ],
    }),
    array_type: Some(LODBINDING_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for LodBinding {
    fn type_info(&self) -> &'static TypeInfo {
        LODBINDING_TYPE_INFO
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


pub static LODBINDING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LodBinding-Array",
    name_hash: 3487293655,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("LodBinding"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AntEnumeration {
    pub _glacier_base: super::core::DataContainer,
    pub ant_asset: super::ant::AntRef,
    pub value: i32,
}

pub trait AntEnumerationTrait: super::core::DataContainerTrait {
    fn ant_asset(&self) -> &super::ant::AntRef;
    fn ant_asset_mut(&mut self) -> &mut super::ant::AntRef;
    fn value(&self) -> &i32;
    fn value_mut(&mut self) -> &mut i32;
}

impl AntEnumerationTrait for AntEnumeration {
    fn ant_asset(&self) -> &super::ant::AntRef {
        &self.ant_asset
    }
    fn ant_asset_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.ant_asset
    }
    fn value(&self) -> &i32 {
        &self.value
    }
    fn value_mut(&mut self) -> &mut i32 {
        &mut self.value
    }
}

impl super::core::DataContainerTrait for AntEnumeration {
}

pub static ANTENUMERATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntEnumeration",
    name_hash: 1891317927,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(AntEnumeration, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AntEnumeration as Default>::default())),
            create_boxed: || Box::new(<AntEnumeration as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "AntAsset",
                name_hash: 1873036974,
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(AntEnumeration, ant_asset),
            },
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AntEnumeration, value),
            },
        ],
    }),
    array_type: Some(ANTENUMERATION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AntEnumeration {
    fn type_info(&self) -> &'static TypeInfo {
        ANTENUMERATION_TYPE_INFO
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


pub static ANTENUMERATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntEnumeration-Array",
    name_hash: 317786643,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("AntEnumeration"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct GameAIWeaponData {
    pub _glacier_base: super::core::Asset,
}

pub trait GameAIWeaponDataTrait: super::core::AssetTrait {
}

impl GameAIWeaponDataTrait for GameAIWeaponData {
}

impl super::core::AssetTrait for GameAIWeaponData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for GameAIWeaponData {
}

pub static GAMEAIWEAPONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameAIWeaponData",
    name_hash: 1246871281,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(GameAIWeaponData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GameAIWeaponData as Default>::default())),
            create_boxed: || Box::new(<GameAIWeaponData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(GAMEAIWEAPONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GameAIWeaponData {
    fn type_info(&self) -> &'static TypeInfo {
        GAMEAIWEAPONDATA_TYPE_INFO
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


pub static GAMEAIWEAPONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameAIWeaponData-Array",
    name_hash: 2630754757,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("GameAIWeaponData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct GameAIEntryData {
    pub _glacier_base: super::core::Asset,
    pub ai2_data: Option<LockedTypeObject /* GameAIEntryData */>,
}

pub trait GameAIEntryDataTrait: super::core::AssetTrait {
    fn ai2_data(&self) -> &Option<LockedTypeObject /* GameAIEntryData */>;
    fn ai2_data_mut(&mut self) -> &mut Option<LockedTypeObject /* GameAIEntryData */>;
}

impl GameAIEntryDataTrait for GameAIEntryData {
    fn ai2_data(&self) -> &Option<LockedTypeObject /* GameAIEntryData */> {
        &self.ai2_data
    }
    fn ai2_data_mut(&mut self) -> &mut Option<LockedTypeObject /* GameAIEntryData */> {
        &mut self.ai2_data
    }
}

impl super::core::AssetTrait for GameAIEntryData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for GameAIEntryData {
}

pub static GAMEAIENTRYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameAIEntryData",
    name_hash: 2748531399,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(GameAIEntryData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GameAIEntryData as Default>::default())),
            create_boxed: || Box::new(<GameAIEntryData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Ai2Data",
                name_hash: 692238447,
                flags: MemberInfoFlags::new(0),
                field_type: "GameAIEntryData",
                rust_offset: offset_of!(GameAIEntryData, ai2_data),
            },
        ],
    }),
    array_type: Some(GAMEAIENTRYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GameAIEntryData {
    fn type_info(&self) -> &'static TypeInfo {
        GAMEAIENTRYDATA_TYPE_INFO
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


pub static GAMEAIENTRYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameAIEntryData-Array",
    name_hash: 133936115,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("GameAIEntryData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PoseConstraintsData {
    pub stand_pose: bool,
    pub crouch_pose: bool,
    pub prone_pose: bool,
}

pub trait PoseConstraintsDataTrait: TypeObject {
    fn stand_pose(&self) -> &bool;
    fn stand_pose_mut(&mut self) -> &mut bool;
    fn crouch_pose(&self) -> &bool;
    fn crouch_pose_mut(&mut self) -> &mut bool;
    fn prone_pose(&self) -> &bool;
    fn prone_pose_mut(&mut self) -> &mut bool;
}

impl PoseConstraintsDataTrait for PoseConstraintsData {
    fn stand_pose(&self) -> &bool {
        &self.stand_pose
    }
    fn stand_pose_mut(&mut self) -> &mut bool {
        &mut self.stand_pose
    }
    fn crouch_pose(&self) -> &bool {
        &self.crouch_pose
    }
    fn crouch_pose_mut(&mut self) -> &mut bool {
        &mut self.crouch_pose
    }
    fn prone_pose(&self) -> &bool {
        &self.prone_pose
    }
    fn prone_pose_mut(&mut self) -> &mut bool {
        &mut self.prone_pose
    }
}

pub static POSECONSTRAINTSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PoseConstraintsData",
    name_hash: 2822963658,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PoseConstraintsData as Default>::default())),
            create_boxed: || Box::new(<PoseConstraintsData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "StandPose",
                name_hash: 3580661600,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PoseConstraintsData, stand_pose),
            },
            FieldInfoData {
                name: "CrouchPose",
                name_hash: 1318089740,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PoseConstraintsData, crouch_pose),
            },
            FieldInfoData {
                name: "PronePose",
                name_hash: 3841962,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PoseConstraintsData, prone_pose),
            },
        ],
    }),
    array_type: Some(POSECONSTRAINTSDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PoseConstraintsData {
    fn type_info(&self) -> &'static TypeInfo {
        POSECONSTRAINTSDATA_TYPE_INFO
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


pub static POSECONSTRAINTSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PoseConstraintsData-Array",
    name_hash: 2391264638,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("PoseConstraintsData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AimingConstraintsData {
    pub min_yaw: f32,
    pub max_yaw: f32,
    pub min_pitch: f32,
    pub max_pitch: f32,
}

pub trait AimingConstraintsDataTrait: TypeObject {
    fn min_yaw(&self) -> &f32;
    fn min_yaw_mut(&mut self) -> &mut f32;
    fn max_yaw(&self) -> &f32;
    fn max_yaw_mut(&mut self) -> &mut f32;
    fn min_pitch(&self) -> &f32;
    fn min_pitch_mut(&mut self) -> &mut f32;
    fn max_pitch(&self) -> &f32;
    fn max_pitch_mut(&mut self) -> &mut f32;
}

impl AimingConstraintsDataTrait for AimingConstraintsData {
    fn min_yaw(&self) -> &f32 {
        &self.min_yaw
    }
    fn min_yaw_mut(&mut self) -> &mut f32 {
        &mut self.min_yaw
    }
    fn max_yaw(&self) -> &f32 {
        &self.max_yaw
    }
    fn max_yaw_mut(&mut self) -> &mut f32 {
        &mut self.max_yaw
    }
    fn min_pitch(&self) -> &f32 {
        &self.min_pitch
    }
    fn min_pitch_mut(&mut self) -> &mut f32 {
        &mut self.min_pitch
    }
    fn max_pitch(&self) -> &f32 {
        &self.max_pitch
    }
    fn max_pitch_mut(&mut self) -> &mut f32 {
        &mut self.max_pitch
    }
}

pub static AIMINGCONSTRAINTSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingConstraintsData",
    name_hash: 258593766,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AimingConstraintsData as Default>::default())),
            create_boxed: || Box::new(<AimingConstraintsData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "MinYaw",
                name_hash: 2633709248,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingConstraintsData, min_yaw),
            },
            FieldInfoData {
                name: "MaxYaw",
                name_hash: 2642824094,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingConstraintsData, max_yaw),
            },
            FieldInfoData {
                name: "MinPitch",
                name_hash: 3374061353,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingConstraintsData, min_pitch),
            },
            FieldInfoData {
                name: "MaxPitch",
                name_hash: 397101687,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingConstraintsData, max_pitch),
            },
        ],
    }),
    array_type: Some(AIMINGCONSTRAINTSDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for AimingConstraintsData {
    fn type_info(&self) -> &'static TypeInfo {
        AIMINGCONSTRAINTSDATA_TYPE_INFO
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


pub static AIMINGCONSTRAINTSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingConstraintsData-Array",
    name_hash: 1599049938,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("AimingConstraintsData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TextureAssetEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub default_texture_asset: Option<LockedTypeObject /* super::render_base::TextureBaseAsset */>,
}

pub trait TextureAssetEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn default_texture_asset(&self) -> &Option<LockedTypeObject /* super::render_base::TextureBaseAsset */>;
    fn default_texture_asset_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::TextureBaseAsset */>;
}

impl TextureAssetEntityDataTrait for TextureAssetEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn default_texture_asset(&self) -> &Option<LockedTypeObject /* super::render_base::TextureBaseAsset */> {
        &self.default_texture_asset
    }
    fn default_texture_asset_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::TextureBaseAsset */> {
        &mut self.default_texture_asset
    }
}

impl super::entity::EntityDataTrait for TextureAssetEntityData {
}

impl super::entity::GameObjectDataTrait for TextureAssetEntityData {
}

impl super::core::DataBusPeerTrait for TextureAssetEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for TextureAssetEntityData {
}

impl super::core::DataContainerTrait for TextureAssetEntityData {
}

pub static TEXTUREASSETENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureAssetEntityData",
    name_hash: 3641402785,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(TextureAssetEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TextureAssetEntityData as Default>::default())),
            create_boxed: || Box::new(<TextureAssetEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                name_hash: 229961746,
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(TextureAssetEntityData, realm),
            },
            FieldInfoData {
                name: "DefaultTextureAsset",
                name_hash: 1859482465,
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(TextureAssetEntityData, default_texture_asset),
            },
        ],
    }),
    array_type: Some(TEXTUREASSETENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TextureAssetEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        TEXTUREASSETENTITYDATA_TYPE_INFO
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


pub static TEXTUREASSETENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureAssetEntityData-Array",
    name_hash: 3094794261,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("TextureAssetEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TerrainShaderParameterEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub shader_parameters: Vec<BoxedTypeObject /* TerrainShaderParameter */>,
}

pub trait TerrainShaderParameterEntityDataTrait: super::entity::EntityDataTrait {
    fn shader_parameters(&self) -> &Vec<BoxedTypeObject /* TerrainShaderParameter */>;
    fn shader_parameters_mut(&mut self) -> &mut Vec<BoxedTypeObject /* TerrainShaderParameter */>;
}

impl TerrainShaderParameterEntityDataTrait for TerrainShaderParameterEntityData {
    fn shader_parameters(&self) -> &Vec<BoxedTypeObject /* TerrainShaderParameter */> {
        &self.shader_parameters
    }
    fn shader_parameters_mut(&mut self) -> &mut Vec<BoxedTypeObject /* TerrainShaderParameter */> {
        &mut self.shader_parameters
    }
}

impl super::entity::EntityDataTrait for TerrainShaderParameterEntityData {
}

impl super::entity::GameObjectDataTrait for TerrainShaderParameterEntityData {
}

impl super::core::DataBusPeerTrait for TerrainShaderParameterEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for TerrainShaderParameterEntityData {
}

impl super::core::DataContainerTrait for TerrainShaderParameterEntityData {
}

pub static TERRAINSHADERPARAMETERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainShaderParameterEntityData",
    name_hash: 4116228345,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(TerrainShaderParameterEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainShaderParameterEntityData as Default>::default())),
            create_boxed: || Box::new(<TerrainShaderParameterEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ShaderParameters",
                name_hash: 3266687574,
                flags: MemberInfoFlags::new(144),
                field_type: "TerrainShaderParameter-Array",
                rust_offset: offset_of!(TerrainShaderParameterEntityData, shader_parameters),
            },
        ],
    }),
    array_type: Some(TERRAINSHADERPARAMETERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TerrainShaderParameterEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINSHADERPARAMETERENTITYDATA_TYPE_INFO
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


pub static TERRAINSHADERPARAMETERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainShaderParameterEntityData-Array",
    name_hash: 4066109389,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("TerrainShaderParameterEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TerrainShaderParameter {
    pub parameter_desc: super::render_base::ShaderParameterDesc,
    pub parameter_type: TerrainShaderParameterDataType,
    pub parameter_name: String,
}

pub trait TerrainShaderParameterTrait: TypeObject {
    fn parameter_desc(&self) -> &super::render_base::ShaderParameterDesc;
    fn parameter_desc_mut(&mut self) -> &mut super::render_base::ShaderParameterDesc;
    fn parameter_type(&self) -> &TerrainShaderParameterDataType;
    fn parameter_type_mut(&mut self) -> &mut TerrainShaderParameterDataType;
    fn parameter_name(&self) -> &String;
    fn parameter_name_mut(&mut self) -> &mut String;
}

impl TerrainShaderParameterTrait for TerrainShaderParameter {
    fn parameter_desc(&self) -> &super::render_base::ShaderParameterDesc {
        &self.parameter_desc
    }
    fn parameter_desc_mut(&mut self) -> &mut super::render_base::ShaderParameterDesc {
        &mut self.parameter_desc
    }
    fn parameter_type(&self) -> &TerrainShaderParameterDataType {
        &self.parameter_type
    }
    fn parameter_type_mut(&mut self) -> &mut TerrainShaderParameterDataType {
        &mut self.parameter_type
    }
    fn parameter_name(&self) -> &String {
        &self.parameter_name
    }
    fn parameter_name_mut(&mut self) -> &mut String {
        &mut self.parameter_name
    }
}

pub static TERRAINSHADERPARAMETER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainShaderParameter",
    name_hash: 1610951058,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainShaderParameter as Default>::default())),
            create_boxed: || Box::new(<TerrainShaderParameter as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ParameterDesc",
                name_hash: 1569306429,
                flags: MemberInfoFlags::new(0),
                field_type: "ShaderParameterDesc",
                rust_offset: offset_of!(TerrainShaderParameter, parameter_desc),
            },
            FieldInfoData {
                name: "ParameterType",
                name_hash: 1569850964,
                flags: MemberInfoFlags::new(0),
                field_type: "TerrainShaderParameterDataType",
                rust_offset: offset_of!(TerrainShaderParameter, parameter_type),
            },
            FieldInfoData {
                name: "ParameterName",
                name_hash: 1568946859,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TerrainShaderParameter, parameter_name),
            },
        ],
    }),
    array_type: Some(TERRAINSHADERPARAMETER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TerrainShaderParameter {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINSHADERPARAMETER_TYPE_INFO
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


pub static TERRAINSHADERPARAMETER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainShaderParameter-Array",
    name_hash: 4001099942,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("TerrainShaderParameter"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum TerrainShaderParameterDataType {
    #[default]
    TerrainShaderParameterDataType_Bool = 0,
    TerrainShaderParameterDataType_Scalar = 1,
    TerrainShaderParameterDataType_Vector2 = 2,
    TerrainShaderParameterDataType_Vector3 = 3,
    TerrainShaderParameterDataType_Vector4 = 4,
}

pub static TERRAINSHADERPARAMETERDATATYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainShaderParameterDataType",
    name_hash: 4013187354,
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(TERRAINSHADERPARAMETERDATATYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TerrainShaderParameterDataType {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINSHADERPARAMETERDATATYPE_TYPE_INFO
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


pub static TERRAINSHADERPARAMETERDATATYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainShaderParameterDataType-Array",
    name_hash: 1737892910,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("TerrainShaderParameterDataType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ShaderParameterEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub parameter_desc: super::render_base::ShaderParameterDesc,
    pub parameter_type: ShaderParameterDataType,
    pub vec_param: super::core::Vec4,
    pub bool_param: bool,
    pub texture_param: Option<LockedTypeObject /* super::render_base::TextureBaseAsset */>,
    pub conditional_param: u32,
}

pub trait ShaderParameterEntityDataTrait: super::entity::EntityDataTrait {
    fn parameter_desc(&self) -> &super::render_base::ShaderParameterDesc;
    fn parameter_desc_mut(&mut self) -> &mut super::render_base::ShaderParameterDesc;
    fn parameter_type(&self) -> &ShaderParameterDataType;
    fn parameter_type_mut(&mut self) -> &mut ShaderParameterDataType;
    fn vec_param(&self) -> &super::core::Vec4;
    fn vec_param_mut(&mut self) -> &mut super::core::Vec4;
    fn bool_param(&self) -> &bool;
    fn bool_param_mut(&mut self) -> &mut bool;
    fn texture_param(&self) -> &Option<LockedTypeObject /* super::render_base::TextureBaseAsset */>;
    fn texture_param_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::TextureBaseAsset */>;
    fn conditional_param(&self) -> &u32;
    fn conditional_param_mut(&mut self) -> &mut u32;
}

impl ShaderParameterEntityDataTrait for ShaderParameterEntityData {
    fn parameter_desc(&self) -> &super::render_base::ShaderParameterDesc {
        &self.parameter_desc
    }
    fn parameter_desc_mut(&mut self) -> &mut super::render_base::ShaderParameterDesc {
        &mut self.parameter_desc
    }
    fn parameter_type(&self) -> &ShaderParameterDataType {
        &self.parameter_type
    }
    fn parameter_type_mut(&mut self) -> &mut ShaderParameterDataType {
        &mut self.parameter_type
    }
    fn vec_param(&self) -> &super::core::Vec4 {
        &self.vec_param
    }
    fn vec_param_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.vec_param
    }
    fn bool_param(&self) -> &bool {
        &self.bool_param
    }
    fn bool_param_mut(&mut self) -> &mut bool {
        &mut self.bool_param
    }
    fn texture_param(&self) -> &Option<LockedTypeObject /* super::render_base::TextureBaseAsset */> {
        &self.texture_param
    }
    fn texture_param_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::TextureBaseAsset */> {
        &mut self.texture_param
    }
    fn conditional_param(&self) -> &u32 {
        &self.conditional_param
    }
    fn conditional_param_mut(&mut self) -> &mut u32 {
        &mut self.conditional_param
    }
}

impl super::entity::EntityDataTrait for ShaderParameterEntityData {
}

impl super::entity::GameObjectDataTrait for ShaderParameterEntityData {
}

impl super::core::DataBusPeerTrait for ShaderParameterEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ShaderParameterEntityData {
}

impl super::core::DataContainerTrait for ShaderParameterEntityData {
}

pub static SHADERPARAMETERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterEntityData",
    name_hash: 3158914350,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(ShaderParameterEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ShaderParameterEntityData as Default>::default())),
            create_boxed: || Box::new(<ShaderParameterEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ParameterDesc",
                name_hash: 1569306429,
                flags: MemberInfoFlags::new(0),
                field_type: "ShaderParameterDesc",
                rust_offset: offset_of!(ShaderParameterEntityData, parameter_desc),
            },
            FieldInfoData {
                name: "ParameterType",
                name_hash: 1569850964,
                flags: MemberInfoFlags::new(0),
                field_type: "ShaderParameterDataType",
                rust_offset: offset_of!(ShaderParameterEntityData, parameter_type),
            },
            FieldInfoData {
                name: "VecParam",
                name_hash: 3770433658,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ShaderParameterEntityData, vec_param),
            },
            FieldInfoData {
                name: "BoolParam",
                name_hash: 169062244,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ShaderParameterEntityData, bool_param),
            },
            FieldInfoData {
                name: "TextureParam",
                name_hash: 2145513813,
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(ShaderParameterEntityData, texture_param),
            },
            FieldInfoData {
                name: "ConditionalParam",
                name_hash: 1588524276,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ShaderParameterEntityData, conditional_param),
            },
        ],
    }),
    array_type: Some(SHADERPARAMETERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ShaderParameterEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        SHADERPARAMETERENTITYDATA_TYPE_INFO
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


pub static SHADERPARAMETERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterEntityData-Array",
    name_hash: 1389526426,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ShaderParameterEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ShaderParameterDataType {
    #[default]
    ShaderParameterDataType_Vector = 0,
    ShaderParameterDataType_Bool = 1,
    ShaderParameterDataType_Texture = 2,
    ShaderParameterDataType_Conditional = 3,
}

pub static SHADERPARAMETERDATATYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterDataType",
    name_hash: 915526669,
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(SHADERPARAMETERDATATYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShaderParameterDataType {
    fn type_info(&self) -> &'static TypeInfo {
        SHADERPARAMETERDATATYPE_TYPE_INFO
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


pub static SHADERPARAMETERDATATYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterDataType-Array",
    name_hash: 3720959929,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ShaderParameterDataType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ANTLayerBlendType {
    #[default]
    ANTLayerBlendType_OverrideBlend = 0,
    ANTLayerBlendType_AdditiveBlend = 1,
    ANTLayerBlendType_SubtractiveBlend = 2,
}

pub static ANTLAYERBLENDTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTLayerBlendType",
    name_hash: 885351588,
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(ANTLAYERBLENDTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ANTLayerBlendType {
    fn type_info(&self) -> &'static TypeInfo {
        ANTLAYERBLENDTYPE_TYPE_INFO
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


pub static ANTLAYERBLENDTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTLayerBlendType-Array",
    name_hash: 3386669072,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ANTLayerBlendType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ANTClipEndRule {
    #[default]
    ANTClipEndRule_None = 0,
    ANTClipEndRule_CarryIntoNextTimeline = 1,
    ANTClipEndRule_LoopIndefinitely = 2,
    ANTClipEndRule_PlayIndefinitely = 3,
    ANTClipEndRule_MatchBarLengthToRuntimeLength = 4,
    ANTClipEndRule_LoopToEndOfTimeline = 5,
    ANTClipEndRule_DoNotLoop = 6,
    ANTClipEndRule_Invalid = 7,
}

pub static ANTCLIPENDRULE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTClipEndRule",
    name_hash: 2312121321,
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(ANTCLIPENDRULE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ANTClipEndRule {
    fn type_info(&self) -> &'static TypeInfo {
        ANTCLIPENDRULE_TYPE_INFO
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


pub static ANTCLIPENDRULE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTClipEndRule-Array",
    name_hash: 26665693,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ANTClipEndRule"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ANTClipStartRule {
    #[default]
    ANTClipStartRule_None = 0,
    ANTClipStartRule_HoldStartFrameFromStartOfTimeline = 1,
}

pub static ANTCLIPSTARTRULE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTClipStartRule",
    name_hash: 3248007206,
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(ANTCLIPSTARTRULE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ANTClipStartRule {
    fn type_info(&self) -> &'static TypeInfo {
        ANTCLIPSTARTRULE_TYPE_INFO
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


pub static ANTCLIPSTARTRULE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTClipStartRule-Array",
    name_hash: 4264350866,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ANTClipStartRule"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SubViewData {
    pub _glacier_base: super::core::DataContainer,
}

pub trait SubViewDataTrait: super::core::DataContainerTrait {
}

impl SubViewDataTrait for SubViewData {
}

impl super::core::DataContainerTrait for SubViewData {
}

pub static SUBVIEWDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubViewData",
    name_hash: 1937039356,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(SubViewData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SubViewData as Default>::default())),
            create_boxed: || Box::new(<SubViewData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SUBVIEWDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SubViewData {
    fn type_info(&self) -> &'static TypeInfo {
        SUBVIEWDATA_TYPE_INFO
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


pub static SUBVIEWDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubViewData-Array",
    name_hash: 2463729608,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("SubViewData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PlayerViewData {
    pub _glacier_base: super::core::DataContainer,
    pub sub_views: Vec<Option<LockedTypeObject /* SubViewData */>>,
}

pub trait PlayerViewDataTrait: super::core::DataContainerTrait {
    fn sub_views(&self) -> &Vec<Option<LockedTypeObject /* SubViewData */>>;
    fn sub_views_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* SubViewData */>>;
}

impl PlayerViewDataTrait for PlayerViewData {
    fn sub_views(&self) -> &Vec<Option<LockedTypeObject /* SubViewData */>> {
        &self.sub_views
    }
    fn sub_views_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* SubViewData */>> {
        &mut self.sub_views
    }
}

impl super::core::DataContainerTrait for PlayerViewData {
}

pub static PLAYERVIEWDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayerViewData",
    name_hash: 288312331,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(PlayerViewData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PlayerViewData as Default>::default())),
            create_boxed: || Box::new(<PlayerViewData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "SubViews",
                name_hash: 1752723775,
                flags: MemberInfoFlags::new(144),
                field_type: "SubViewData-Array",
                rust_offset: offset_of!(PlayerViewData, sub_views),
            },
        ],
    }),
    array_type: Some(PLAYERVIEWDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PlayerViewData {
    fn type_info(&self) -> &'static TypeInfo {
        PLAYERVIEWDATA_TYPE_INFO
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


pub static PLAYERVIEWDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayerViewData-Array",
    name_hash: 1067031231,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("PlayerViewData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PlayerData {
    pub _glacier_base: super::core::Asset,
    pub player_view: Option<LockedTypeObject /* PlayerViewData */>,
}

pub trait PlayerDataTrait: super::core::AssetTrait {
    fn player_view(&self) -> &Option<LockedTypeObject /* PlayerViewData */>;
    fn player_view_mut(&mut self) -> &mut Option<LockedTypeObject /* PlayerViewData */>;
}

impl PlayerDataTrait for PlayerData {
    fn player_view(&self) -> &Option<LockedTypeObject /* PlayerViewData */> {
        &self.player_view
    }
    fn player_view_mut(&mut self) -> &mut Option<LockedTypeObject /* PlayerViewData */> {
        &mut self.player_view
    }
}

impl super::core::AssetTrait for PlayerData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for PlayerData {
}

pub static PLAYERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayerData",
    name_hash: 775121542,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(PlayerData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PlayerData as Default>::default())),
            create_boxed: || Box::new(<PlayerData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "PlayerView",
                name_hash: 774623483,
                flags: MemberInfoFlags::new(0),
                field_type: "PlayerViewData",
                rust_offset: offset_of!(PlayerData, player_view),
            },
        ],
    }),
    array_type: Some(PLAYERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PlayerData {
    fn type_info(&self) -> &'static TypeInfo {
        PLAYERDATA_TYPE_INFO
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


pub static PLAYERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayerData-Array",
    name_hash: 38000562,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("PlayerData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct NetworkedOnlineId {
    pub native_data: u64,
}

pub trait NetworkedOnlineIdTrait: TypeObject {
    fn native_data(&self) -> &u64;
    fn native_data_mut(&mut self) -> &mut u64;
}

impl NetworkedOnlineIdTrait for NetworkedOnlineId {
    fn native_data(&self) -> &u64 {
        &self.native_data
    }
    fn native_data_mut(&mut self) -> &mut u64 {
        &mut self.native_data
    }
}

pub static NETWORKEDONLINEID_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkedOnlineId",
    name_hash: 129676824,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkedOnlineId as Default>::default())),
            create_boxed: || Box::new(<NetworkedOnlineId as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "NativeData",
                name_hash: 575994452,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint64",
                rust_offset: offset_of!(NetworkedOnlineId, native_data),
            },
        ],
    }),
    array_type: Some(NETWORKEDONLINEID_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NetworkedOnlineId {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKEDONLINEID_TYPE_INFO
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


pub static NETWORKEDONLINEID_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkedOnlineId-Array",
    name_hash: 3374055468,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("NetworkedOnlineId"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum OnlineIdConstants {
    #[default]
    OnlineIdConstants_IdLength = 17,
}

pub static ONLINEIDCONSTANTS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineIdConstants",
    name_hash: 2693756170,
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(ONLINEIDCONSTANTS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for OnlineIdConstants {
    fn type_info(&self) -> &'static TypeInfo {
        ONLINEIDCONSTANTS_TYPE_INFO
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


pub static ONLINEIDCONSTANTS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineIdConstants-Array",
    name_hash: 777875518,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("OnlineIdConstants"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct UINetworkPlayerDisconnectMessage {
}

pub trait UINetworkPlayerDisconnectMessageTrait: TypeObject {
}

impl UINetworkPlayerDisconnectMessageTrait for UINetworkPlayerDisconnectMessage {
}

pub static UINETWORKPLAYERDISCONNECTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UINetworkPlayerDisconnectMessage",
    name_hash: 3452770207,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UINetworkPlayerDisconnectMessage as Default>::default())),
            create_boxed: || Box::new(<UINetworkPlayerDisconnectMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UINetworkPlayerDisconnectMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UINETWORKPLAYERDISCONNECTMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct UINetworkPlayerConnectMessage {
}

pub trait UINetworkPlayerConnectMessageTrait: TypeObject {
}

impl UINetworkPlayerConnectMessageTrait for UINetworkPlayerConnectMessage {
}

pub static UINETWORKPLAYERCONNECTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UINetworkPlayerConnectMessage",
    name_hash: 3550809537,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UINetworkPlayerConnectMessage as Default>::default())),
            create_boxed: || Box::new(<UINetworkPlayerConnectMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UINetworkPlayerConnectMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UINETWORKPLAYERCONNECTMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct NetworkSettings {
    pub _glacier_base: super::core::SystemSettings,
    pub protocol_version: u32,
    pub title_id: String,
    pub client_port: u32,
    pub server_port: u32,
    pub max_ghost_count: u32,
    pub max_client_to_server_ghost_count: u32,
    pub max_client_count: u32,
    pub max_client_frame_size: u32,
    pub max_server_frame_size: u32,
    pub max_num_voip_peers: u32,
    pub server_address: String,
    pub client_connection_debug_file_prefix: String,
    pub server_connection_debug_file_prefix: String,
    pub single_player_time_nudge_bias: f32,
    pub single_player_time_nudge: f32,
    pub single_player_automatic_time_nudge: bool,
    pub memory_socket_time_nudge_bias: f32,
    pub memory_socket_time_nudge: f32,
    pub memory_socket_automatic_time_nudge: bool,
    pub local_host_time_nudge_bias: f32,
    pub local_host_time_nudge: f32,
    pub local_host_automatic_time_nudge: bool,
    pub default_time_nudge_bias: f32,
    pub default_time_nudge: f32,
    pub default_automatic_time_nudge: bool,
    pub increment_server_port_on_fail: bool,
    pub use_frame_manager: bool,
    pub time_sync_enabled: bool,
    pub connect_timeout: f32,
    pub packet_loss_log_interval: f32,
    pub valid_local_players_mask: u32,
    pub desired_local_players_mask: u32,
    pub persistent_local_players_mask: u32,
    pub m_l_u_r_enabled: bool,
    pub single_player_max_messages_per_network_frame: u32,
    pub max_messages_per_network_frame: u32,
}

pub trait NetworkSettingsTrait: super::core::SystemSettingsTrait {
    fn protocol_version(&self) -> &u32;
    fn protocol_version_mut(&mut self) -> &mut u32;
    fn title_id(&self) -> &String;
    fn title_id_mut(&mut self) -> &mut String;
    fn client_port(&self) -> &u32;
    fn client_port_mut(&mut self) -> &mut u32;
    fn server_port(&self) -> &u32;
    fn server_port_mut(&mut self) -> &mut u32;
    fn max_ghost_count(&self) -> &u32;
    fn max_ghost_count_mut(&mut self) -> &mut u32;
    fn max_client_to_server_ghost_count(&self) -> &u32;
    fn max_client_to_server_ghost_count_mut(&mut self) -> &mut u32;
    fn max_client_count(&self) -> &u32;
    fn max_client_count_mut(&mut self) -> &mut u32;
    fn max_client_frame_size(&self) -> &u32;
    fn max_client_frame_size_mut(&mut self) -> &mut u32;
    fn max_server_frame_size(&self) -> &u32;
    fn max_server_frame_size_mut(&mut self) -> &mut u32;
    fn max_num_voip_peers(&self) -> &u32;
    fn max_num_voip_peers_mut(&mut self) -> &mut u32;
    fn server_address(&self) -> &String;
    fn server_address_mut(&mut self) -> &mut String;
    fn client_connection_debug_file_prefix(&self) -> &String;
    fn client_connection_debug_file_prefix_mut(&mut self) -> &mut String;
    fn server_connection_debug_file_prefix(&self) -> &String;
    fn server_connection_debug_file_prefix_mut(&mut self) -> &mut String;
    fn single_player_time_nudge_bias(&self) -> &f32;
    fn single_player_time_nudge_bias_mut(&mut self) -> &mut f32;
    fn single_player_time_nudge(&self) -> &f32;
    fn single_player_time_nudge_mut(&mut self) -> &mut f32;
    fn single_player_automatic_time_nudge(&self) -> &bool;
    fn single_player_automatic_time_nudge_mut(&mut self) -> &mut bool;
    fn memory_socket_time_nudge_bias(&self) -> &f32;
    fn memory_socket_time_nudge_bias_mut(&mut self) -> &mut f32;
    fn memory_socket_time_nudge(&self) -> &f32;
    fn memory_socket_time_nudge_mut(&mut self) -> &mut f32;
    fn memory_socket_automatic_time_nudge(&self) -> &bool;
    fn memory_socket_automatic_time_nudge_mut(&mut self) -> &mut bool;
    fn local_host_time_nudge_bias(&self) -> &f32;
    fn local_host_time_nudge_bias_mut(&mut self) -> &mut f32;
    fn local_host_time_nudge(&self) -> &f32;
    fn local_host_time_nudge_mut(&mut self) -> &mut f32;
    fn local_host_automatic_time_nudge(&self) -> &bool;
    fn local_host_automatic_time_nudge_mut(&mut self) -> &mut bool;
    fn default_time_nudge_bias(&self) -> &f32;
    fn default_time_nudge_bias_mut(&mut self) -> &mut f32;
    fn default_time_nudge(&self) -> &f32;
    fn default_time_nudge_mut(&mut self) -> &mut f32;
    fn default_automatic_time_nudge(&self) -> &bool;
    fn default_automatic_time_nudge_mut(&mut self) -> &mut bool;
    fn increment_server_port_on_fail(&self) -> &bool;
    fn increment_server_port_on_fail_mut(&mut self) -> &mut bool;
    fn use_frame_manager(&self) -> &bool;
    fn use_frame_manager_mut(&mut self) -> &mut bool;
    fn time_sync_enabled(&self) -> &bool;
    fn time_sync_enabled_mut(&mut self) -> &mut bool;
    fn connect_timeout(&self) -> &f32;
    fn connect_timeout_mut(&mut self) -> &mut f32;
    fn packet_loss_log_interval(&self) -> &f32;
    fn packet_loss_log_interval_mut(&mut self) -> &mut f32;
    fn valid_local_players_mask(&self) -> &u32;
    fn valid_local_players_mask_mut(&mut self) -> &mut u32;
    fn desired_local_players_mask(&self) -> &u32;
    fn desired_local_players_mask_mut(&mut self) -> &mut u32;
    fn persistent_local_players_mask(&self) -> &u32;
    fn persistent_local_players_mask_mut(&mut self) -> &mut u32;
    fn m_l_u_r_enabled(&self) -> &bool;
    fn m_l_u_r_enabled_mut(&mut self) -> &mut bool;
    fn single_player_max_messages_per_network_frame(&self) -> &u32;
    fn single_player_max_messages_per_network_frame_mut(&mut self) -> &mut u32;
    fn max_messages_per_network_frame(&self) -> &u32;
    fn max_messages_per_network_frame_mut(&mut self) -> &mut u32;
}

impl NetworkSettingsTrait for NetworkSettings {
    fn protocol_version(&self) -> &u32 {
        &self.protocol_version
    }
    fn protocol_version_mut(&mut self) -> &mut u32 {
        &mut self.protocol_version
    }
    fn title_id(&self) -> &String {
        &self.title_id
    }
    fn title_id_mut(&mut self) -> &mut String {
        &mut self.title_id
    }
    fn client_port(&self) -> &u32 {
        &self.client_port
    }
    fn client_port_mut(&mut self) -> &mut u32 {
        &mut self.client_port
    }
    fn server_port(&self) -> &u32 {
        &self.server_port
    }
    fn server_port_mut(&mut self) -> &mut u32 {
        &mut self.server_port
    }
    fn max_ghost_count(&self) -> &u32 {
        &self.max_ghost_count
    }
    fn max_ghost_count_mut(&mut self) -> &mut u32 {
        &mut self.max_ghost_count
    }
    fn max_client_to_server_ghost_count(&self) -> &u32 {
        &self.max_client_to_server_ghost_count
    }
    fn max_client_to_server_ghost_count_mut(&mut self) -> &mut u32 {
        &mut self.max_client_to_server_ghost_count
    }
    fn max_client_count(&self) -> &u32 {
        &self.max_client_count
    }
    fn max_client_count_mut(&mut self) -> &mut u32 {
        &mut self.max_client_count
    }
    fn max_client_frame_size(&self) -> &u32 {
        &self.max_client_frame_size
    }
    fn max_client_frame_size_mut(&mut self) -> &mut u32 {
        &mut self.max_client_frame_size
    }
    fn max_server_frame_size(&self) -> &u32 {
        &self.max_server_frame_size
    }
    fn max_server_frame_size_mut(&mut self) -> &mut u32 {
        &mut self.max_server_frame_size
    }
    fn max_num_voip_peers(&self) -> &u32 {
        &self.max_num_voip_peers
    }
    fn max_num_voip_peers_mut(&mut self) -> &mut u32 {
        &mut self.max_num_voip_peers
    }
    fn server_address(&self) -> &String {
        &self.server_address
    }
    fn server_address_mut(&mut self) -> &mut String {
        &mut self.server_address
    }
    fn client_connection_debug_file_prefix(&self) -> &String {
        &self.client_connection_debug_file_prefix
    }
    fn client_connection_debug_file_prefix_mut(&mut self) -> &mut String {
        &mut self.client_connection_debug_file_prefix
    }
    fn server_connection_debug_file_prefix(&self) -> &String {
        &self.server_connection_debug_file_prefix
    }
    fn server_connection_debug_file_prefix_mut(&mut self) -> &mut String {
        &mut self.server_connection_debug_file_prefix
    }
    fn single_player_time_nudge_bias(&self) -> &f32 {
        &self.single_player_time_nudge_bias
    }
    fn single_player_time_nudge_bias_mut(&mut self) -> &mut f32 {
        &mut self.single_player_time_nudge_bias
    }
    fn single_player_time_nudge(&self) -> &f32 {
        &self.single_player_time_nudge
    }
    fn single_player_time_nudge_mut(&mut self) -> &mut f32 {
        &mut self.single_player_time_nudge
    }
    fn single_player_automatic_time_nudge(&self) -> &bool {
        &self.single_player_automatic_time_nudge
    }
    fn single_player_automatic_time_nudge_mut(&mut self) -> &mut bool {
        &mut self.single_player_automatic_time_nudge
    }
    fn memory_socket_time_nudge_bias(&self) -> &f32 {
        &self.memory_socket_time_nudge_bias
    }
    fn memory_socket_time_nudge_bias_mut(&mut self) -> &mut f32 {
        &mut self.memory_socket_time_nudge_bias
    }
    fn memory_socket_time_nudge(&self) -> &f32 {
        &self.memory_socket_time_nudge
    }
    fn memory_socket_time_nudge_mut(&mut self) -> &mut f32 {
        &mut self.memory_socket_time_nudge
    }
    fn memory_socket_automatic_time_nudge(&self) -> &bool {
        &self.memory_socket_automatic_time_nudge
    }
    fn memory_socket_automatic_time_nudge_mut(&mut self) -> &mut bool {
        &mut self.memory_socket_automatic_time_nudge
    }
    fn local_host_time_nudge_bias(&self) -> &f32 {
        &self.local_host_time_nudge_bias
    }
    fn local_host_time_nudge_bias_mut(&mut self) -> &mut f32 {
        &mut self.local_host_time_nudge_bias
    }
    fn local_host_time_nudge(&self) -> &f32 {
        &self.local_host_time_nudge
    }
    fn local_host_time_nudge_mut(&mut self) -> &mut f32 {
        &mut self.local_host_time_nudge
    }
    fn local_host_automatic_time_nudge(&self) -> &bool {
        &self.local_host_automatic_time_nudge
    }
    fn local_host_automatic_time_nudge_mut(&mut self) -> &mut bool {
        &mut self.local_host_automatic_time_nudge
    }
    fn default_time_nudge_bias(&self) -> &f32 {
        &self.default_time_nudge_bias
    }
    fn default_time_nudge_bias_mut(&mut self) -> &mut f32 {
        &mut self.default_time_nudge_bias
    }
    fn default_time_nudge(&self) -> &f32 {
        &self.default_time_nudge
    }
    fn default_time_nudge_mut(&mut self) -> &mut f32 {
        &mut self.default_time_nudge
    }
    fn default_automatic_time_nudge(&self) -> &bool {
        &self.default_automatic_time_nudge
    }
    fn default_automatic_time_nudge_mut(&mut self) -> &mut bool {
        &mut self.default_automatic_time_nudge
    }
    fn increment_server_port_on_fail(&self) -> &bool {
        &self.increment_server_port_on_fail
    }
    fn increment_server_port_on_fail_mut(&mut self) -> &mut bool {
        &mut self.increment_server_port_on_fail
    }
    fn use_frame_manager(&self) -> &bool {
        &self.use_frame_manager
    }
    fn use_frame_manager_mut(&mut self) -> &mut bool {
        &mut self.use_frame_manager
    }
    fn time_sync_enabled(&self) -> &bool {
        &self.time_sync_enabled
    }
    fn time_sync_enabled_mut(&mut self) -> &mut bool {
        &mut self.time_sync_enabled
    }
    fn connect_timeout(&self) -> &f32 {
        &self.connect_timeout
    }
    fn connect_timeout_mut(&mut self) -> &mut f32 {
        &mut self.connect_timeout
    }
    fn packet_loss_log_interval(&self) -> &f32 {
        &self.packet_loss_log_interval
    }
    fn packet_loss_log_interval_mut(&mut self) -> &mut f32 {
        &mut self.packet_loss_log_interval
    }
    fn valid_local_players_mask(&self) -> &u32 {
        &self.valid_local_players_mask
    }
    fn valid_local_players_mask_mut(&mut self) -> &mut u32 {
        &mut self.valid_local_players_mask
    }
    fn desired_local_players_mask(&self) -> &u32 {
        &self.desired_local_players_mask
    }
    fn desired_local_players_mask_mut(&mut self) -> &mut u32 {
        &mut self.desired_local_players_mask
    }
    fn persistent_local_players_mask(&self) -> &u32 {
        &self.persistent_local_players_mask
    }
    fn persistent_local_players_mask_mut(&mut self) -> &mut u32 {
        &mut self.persistent_local_players_mask
    }
    fn m_l_u_r_enabled(&self) -> &bool {
        &self.m_l_u_r_enabled
    }
    fn m_l_u_r_enabled_mut(&mut self) -> &mut bool {
        &mut self.m_l_u_r_enabled
    }
    fn single_player_max_messages_per_network_frame(&self) -> &u32 {
        &self.single_player_max_messages_per_network_frame
    }
    fn single_player_max_messages_per_network_frame_mut(&mut self) -> &mut u32 {
        &mut self.single_player_max_messages_per_network_frame
    }
    fn max_messages_per_network_frame(&self) -> &u32 {
        &self.max_messages_per_network_frame
    }
    fn max_messages_per_network_frame_mut(&mut self) -> &mut u32 {
        &mut self.max_messages_per_network_frame
    }
}

impl super::core::SystemSettingsTrait for NetworkSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl super::core::DataContainerTrait for NetworkSettings {
}

pub static NETWORKSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkSettings",
    name_hash: 2817246782,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        super_class_offset: offset_of!(NetworkSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkSettings as Default>::default())),
            create_boxed: || Box::new(<NetworkSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ProtocolVersion",
                name_hash: 3170322697,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(NetworkSettings, protocol_version),
            },
            FieldInfoData {
                name: "TitleId",
                name_hash: 3335995016,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(NetworkSettings, title_id),
            },
            FieldInfoData {
                name: "ClientPort",
                name_hash: 891909509,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(NetworkSettings, client_port),
            },
            FieldInfoData {
                name: "ServerPort",
                name_hash: 1802195673,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(NetworkSettings, server_port),
            },
            FieldInfoData {
                name: "MaxGhostCount",
                name_hash: 2256733237,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(NetworkSettings, max_ghost_count),
            },
            FieldInfoData {
                name: "MaxClientToServerGhostCount",
                name_hash: 2262274450,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(NetworkSettings, max_client_to_server_ghost_count),
            },
            FieldInfoData {
                name: "MaxClientCount",
                name_hash: 59362443,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(NetworkSettings, max_client_count),
            },
            FieldInfoData {
                name: "MaxClientFrameSize",
                name_hash: 1801242448,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(NetworkSettings, max_client_frame_size),
            },
            FieldInfoData {
                name: "MaxServerFrameSize",
                name_hash: 3192625292,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(NetworkSettings, max_server_frame_size),
            },
            FieldInfoData {
                name: "MaxNumVoipPeers",
                name_hash: 2852316886,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(NetworkSettings, max_num_voip_peers),
            },
            FieldInfoData {
                name: "ServerAddress",
                name_hash: 2537212758,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(NetworkSettings, server_address),
            },
            FieldInfoData {
                name: "ClientConnectionDebugFilePrefix",
                name_hash: 46256173,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(NetworkSettings, client_connection_debug_file_prefix),
            },
            FieldInfoData {
                name: "ServerConnectionDebugFilePrefix",
                name_hash: 1658812785,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(NetworkSettings, server_connection_debug_file_prefix),
            },
            FieldInfoData {
                name: "SinglePlayerTimeNudgeBias",
                name_hash: 1140163037,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetworkSettings, single_player_time_nudge_bias),
            },
            FieldInfoData {
                name: "SinglePlayerTimeNudge",
                name_hash: 2597524676,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetworkSettings, single_player_time_nudge),
            },
            FieldInfoData {
                name: "SinglePlayerAutomaticTimeNudge",
                name_hash: 3616082489,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(NetworkSettings, single_player_automatic_time_nudge),
            },
            FieldInfoData {
                name: "MemorySocketTimeNudgeBias",
                name_hash: 1941276464,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetworkSettings, memory_socket_time_nudge_bias),
            },
            FieldInfoData {
                name: "MemorySocketTimeNudge",
                name_hash: 2649709417,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetworkSettings, memory_socket_time_nudge),
            },
            FieldInfoData {
                name: "MemorySocketAutomaticTimeNudge",
                name_hash: 1057444724,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(NetworkSettings, memory_socket_automatic_time_nudge),
            },
            FieldInfoData {
                name: "LocalHostTimeNudgeBias",
                name_hash: 3248921433,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetworkSettings, local_host_time_nudge_bias),
            },
            FieldInfoData {
                name: "LocalHostTimeNudge",
                name_hash: 1817890880,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetworkSettings, local_host_time_nudge),
            },
            FieldInfoData {
                name: "LocalHostAutomaticTimeNudge",
                name_hash: 1605203005,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(NetworkSettings, local_host_automatic_time_nudge),
            },
            FieldInfoData {
                name: "DefaultTimeNudgeBias",
                name_hash: 1012032863,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetworkSettings, default_time_nudge_bias),
            },
            FieldInfoData {
                name: "DefaultTimeNudge",
                name_hash: 4010137798,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetworkSettings, default_time_nudge),
            },
            FieldInfoData {
                name: "DefaultAutomaticTimeNudge",
                name_hash: 242773115,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(NetworkSettings, default_automatic_time_nudge),
            },
            FieldInfoData {
                name: "IncrementServerPortOnFail",
                name_hash: 1185335067,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(NetworkSettings, increment_server_port_on_fail),
            },
            FieldInfoData {
                name: "UseFrameManager",
                name_hash: 1232877352,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(NetworkSettings, use_frame_manager),
            },
            FieldInfoData {
                name: "TimeSyncEnabled",
                name_hash: 1025585810,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(NetworkSettings, time_sync_enabled),
            },
            FieldInfoData {
                name: "ConnectTimeout",
                name_hash: 1924381792,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetworkSettings, connect_timeout),
            },
            FieldInfoData {
                name: "PacketLossLogInterval",
                name_hash: 1599984885,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetworkSettings, packet_loss_log_interval),
            },
            FieldInfoData {
                name: "ValidLocalPlayersMask",
                name_hash: 965321674,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(NetworkSettings, valid_local_players_mask),
            },
            FieldInfoData {
                name: "DesiredLocalPlayersMask",
                name_hash: 3800008884,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(NetworkSettings, desired_local_players_mask),
            },
            FieldInfoData {
                name: "PersistentLocalPlayersMask",
                name_hash: 2171004985,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(NetworkSettings, persistent_local_players_mask),
            },
            FieldInfoData {
                name: "MLUREnabled",
                name_hash: 1622515590,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(NetworkSettings, m_l_u_r_enabled),
            },
            FieldInfoData {
                name: "SinglePlayerMaxMessagesPerNetworkFrame",
                name_hash: 3317626788,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(NetworkSettings, single_player_max_messages_per_network_frame),
            },
            FieldInfoData {
                name: "MaxMessagesPerNetworkFrame",
                name_hash: 2385736461,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(NetworkSettings, max_messages_per_network_frame),
            },
        ],
    }),
    array_type: Some(NETWORKSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NetworkSettings {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKSETTINGS_TYPE_INFO
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


pub static NETWORKSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkSettings-Array",
    name_hash: 2079364746,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("NetworkSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct VersionData {
    pub _glacier_base: super::core::Asset,
    pub disclaimer: String,
    pub version: i32,
    pub date_time: String,
    pub branch_id: String,
    pub data_branch_id: String,
    pub game_name: String,
}

pub trait VersionDataTrait: super::core::AssetTrait {
    fn disclaimer(&self) -> &String;
    fn disclaimer_mut(&mut self) -> &mut String;
    fn version(&self) -> &i32;
    fn version_mut(&mut self) -> &mut i32;
    fn date_time(&self) -> &String;
    fn date_time_mut(&mut self) -> &mut String;
    fn branch_id(&self) -> &String;
    fn branch_id_mut(&mut self) -> &mut String;
    fn data_branch_id(&self) -> &String;
    fn data_branch_id_mut(&mut self) -> &mut String;
    fn game_name(&self) -> &String;
    fn game_name_mut(&mut self) -> &mut String;
}

impl VersionDataTrait for VersionData {
    fn disclaimer(&self) -> &String {
        &self.disclaimer
    }
    fn disclaimer_mut(&mut self) -> &mut String {
        &mut self.disclaimer
    }
    fn version(&self) -> &i32 {
        &self.version
    }
    fn version_mut(&mut self) -> &mut i32 {
        &mut self.version
    }
    fn date_time(&self) -> &String {
        &self.date_time
    }
    fn date_time_mut(&mut self) -> &mut String {
        &mut self.date_time
    }
    fn branch_id(&self) -> &String {
        &self.branch_id
    }
    fn branch_id_mut(&mut self) -> &mut String {
        &mut self.branch_id
    }
    fn data_branch_id(&self) -> &String {
        &self.data_branch_id
    }
    fn data_branch_id_mut(&mut self) -> &mut String {
        &mut self.data_branch_id
    }
    fn game_name(&self) -> &String {
        &self.game_name
    }
    fn game_name_mut(&mut self) -> &mut String {
        &mut self.game_name
    }
}

impl super::core::AssetTrait for VersionData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for VersionData {
}

pub static VERSIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VersionData",
    name_hash: 3679279023,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(VersionData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VersionData as Default>::default())),
            create_boxed: || Box::new(<VersionData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "disclaimer",
                name_hash: 33610342,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(VersionData, disclaimer),
            },
            FieldInfoData {
                name: "Version",
                name_hash: 747123679,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(VersionData, version),
            },
            FieldInfoData {
                name: "DateTime",
                name_hash: 3244533220,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(VersionData, date_time),
            },
            FieldInfoData {
                name: "BranchId",
                name_hash: 2969494588,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(VersionData, branch_id),
            },
            FieldInfoData {
                name: "DataBranchId",
                name_hash: 2193655564,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(VersionData, data_branch_id),
            },
            FieldInfoData {
                name: "GameName",
                name_hash: 509587500,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(VersionData, game_name),
            },
        ],
    }),
    array_type: Some(VERSIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VersionData {
    fn type_info(&self) -> &'static TypeInfo {
        VERSIONDATA_TYPE_INFO
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


pub static VERSIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VersionData-Array",
    name_hash: 853394715,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("VersionData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct UISettings {
    pub _glacier_base: super::core::SystemSettings,
    pub system: super::u_i::UISystemType,
    pub profile_options: Option<LockedTypeObject /* ProfileOptionsAsset */>,
    pub language: super::core::LanguageFormat,
    pub draw_enable: bool,
    pub enable_jobs: bool,
    pub scaleform_safe_mode: bool,
    pub localization: Option<LockedTypeObject /* super::u_i::LocalizationAsset */>,
    pub font_configuration: Option<LockedTypeObject /* UIFontConfigurationAssetBase */>,
    pub allow_slow_texture_loading_path: bool,
    pub allow_layer_name_access: bool,
}

pub trait UISettingsTrait: super::core::SystemSettingsTrait {
    fn system(&self) -> &super::u_i::UISystemType;
    fn system_mut(&mut self) -> &mut super::u_i::UISystemType;
    fn profile_options(&self) -> &Option<LockedTypeObject /* ProfileOptionsAsset */>;
    fn profile_options_mut(&mut self) -> &mut Option<LockedTypeObject /* ProfileOptionsAsset */>;
    fn language(&self) -> &super::core::LanguageFormat;
    fn language_mut(&mut self) -> &mut super::core::LanguageFormat;
    fn draw_enable(&self) -> &bool;
    fn draw_enable_mut(&mut self) -> &mut bool;
    fn enable_jobs(&self) -> &bool;
    fn enable_jobs_mut(&mut self) -> &mut bool;
    fn scaleform_safe_mode(&self) -> &bool;
    fn scaleform_safe_mode_mut(&mut self) -> &mut bool;
    fn localization(&self) -> &Option<LockedTypeObject /* super::u_i::LocalizationAsset */>;
    fn localization_mut(&mut self) -> &mut Option<LockedTypeObject /* super::u_i::LocalizationAsset */>;
    fn font_configuration(&self) -> &Option<LockedTypeObject /* UIFontConfigurationAssetBase */>;
    fn font_configuration_mut(&mut self) -> &mut Option<LockedTypeObject /* UIFontConfigurationAssetBase */>;
    fn allow_slow_texture_loading_path(&self) -> &bool;
    fn allow_slow_texture_loading_path_mut(&mut self) -> &mut bool;
    fn allow_layer_name_access(&self) -> &bool;
    fn allow_layer_name_access_mut(&mut self) -> &mut bool;
}

impl UISettingsTrait for UISettings {
    fn system(&self) -> &super::u_i::UISystemType {
        &self.system
    }
    fn system_mut(&mut self) -> &mut super::u_i::UISystemType {
        &mut self.system
    }
    fn profile_options(&self) -> &Option<LockedTypeObject /* ProfileOptionsAsset */> {
        &self.profile_options
    }
    fn profile_options_mut(&mut self) -> &mut Option<LockedTypeObject /* ProfileOptionsAsset */> {
        &mut self.profile_options
    }
    fn language(&self) -> &super::core::LanguageFormat {
        &self.language
    }
    fn language_mut(&mut self) -> &mut super::core::LanguageFormat {
        &mut self.language
    }
    fn draw_enable(&self) -> &bool {
        &self.draw_enable
    }
    fn draw_enable_mut(&mut self) -> &mut bool {
        &mut self.draw_enable
    }
    fn enable_jobs(&self) -> &bool {
        &self.enable_jobs
    }
    fn enable_jobs_mut(&mut self) -> &mut bool {
        &mut self.enable_jobs
    }
    fn scaleform_safe_mode(&self) -> &bool {
        &self.scaleform_safe_mode
    }
    fn scaleform_safe_mode_mut(&mut self) -> &mut bool {
        &mut self.scaleform_safe_mode
    }
    fn localization(&self) -> &Option<LockedTypeObject /* super::u_i::LocalizationAsset */> {
        &self.localization
    }
    fn localization_mut(&mut self) -> &mut Option<LockedTypeObject /* super::u_i::LocalizationAsset */> {
        &mut self.localization
    }
    fn font_configuration(&self) -> &Option<LockedTypeObject /* UIFontConfigurationAssetBase */> {
        &self.font_configuration
    }
    fn font_configuration_mut(&mut self) -> &mut Option<LockedTypeObject /* UIFontConfigurationAssetBase */> {
        &mut self.font_configuration
    }
    fn allow_slow_texture_loading_path(&self) -> &bool {
        &self.allow_slow_texture_loading_path
    }
    fn allow_slow_texture_loading_path_mut(&mut self) -> &mut bool {
        &mut self.allow_slow_texture_loading_path
    }
    fn allow_layer_name_access(&self) -> &bool {
        &self.allow_layer_name_access
    }
    fn allow_layer_name_access_mut(&mut self) -> &mut bool {
        &mut self.allow_layer_name_access
    }
}

impl super::core::SystemSettingsTrait for UISettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl super::core::DataContainerTrait for UISettings {
}

pub static UISETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UISettings",
    name_hash: 1347826556,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        super_class_offset: offset_of!(UISettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UISettings as Default>::default())),
            create_boxed: || Box::new(<UISettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "System",
                name_hash: 3333232768,
                flags: MemberInfoFlags::new(0),
                field_type: "UISystemType",
                rust_offset: offset_of!(UISettings, system),
            },
            FieldInfoData {
                name: "ProfileOptions",
                name_hash: 1140566110,
                flags: MemberInfoFlags::new(0),
                field_type: "ProfileOptionsAsset",
                rust_offset: offset_of!(UISettings, profile_options),
            },
            FieldInfoData {
                name: "Language",
                name_hash: 3872303031,
                flags: MemberInfoFlags::new(0),
                field_type: "LanguageFormat",
                rust_offset: offset_of!(UISettings, language),
            },
            FieldInfoData {
                name: "DrawEnable",
                name_hash: 1347356004,
                flags: MemberInfoFlags::new(8192),
                field_type: "Boolean",
                rust_offset: offset_of!(UISettings, draw_enable),
            },
            FieldInfoData {
                name: "EnableJobs",
                name_hash: 1190923856,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UISettings, enable_jobs),
            },
            FieldInfoData {
                name: "ScaleformSafeMode",
                name_hash: 456367385,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UISettings, scaleform_safe_mode),
            },
            FieldInfoData {
                name: "Localization",
                name_hash: 13208870,
                flags: MemberInfoFlags::new(0),
                field_type: "LocalizationAsset",
                rust_offset: offset_of!(UISettings, localization),
            },
            FieldInfoData {
                name: "FontConfiguration",
                name_hash: 2529725446,
                flags: MemberInfoFlags::new(0),
                field_type: "UIFontConfigurationAssetBase",
                rust_offset: offset_of!(UISettings, font_configuration),
            },
            FieldInfoData {
                name: "AllowSlowTextureLoadingPath",
                name_hash: 2815132655,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UISettings, allow_slow_texture_loading_path),
            },
            FieldInfoData {
                name: "AllowLayerNameAccess",
                name_hash: 1168017244,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UISettings, allow_layer_name_access),
            },
        ],
    }),
    array_type: Some(UISETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UISettings {
    fn type_info(&self) -> &'static TypeInfo {
        UISETTINGS_TYPE_INFO
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


pub static UISETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UISettings-Array",
    name_hash: 1934326600,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("UISettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct UIFontConfigurationAssetBase {
    pub _glacier_base: super::core::Asset,
}

pub trait UIFontConfigurationAssetBaseTrait: super::core::AssetTrait {
}

impl UIFontConfigurationAssetBaseTrait for UIFontConfigurationAssetBase {
}

impl super::core::AssetTrait for UIFontConfigurationAssetBase {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for UIFontConfigurationAssetBase {
}

pub static UIFONTCONFIGURATIONASSETBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIFontConfigurationAssetBase",
    name_hash: 1592364287,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(UIFontConfigurationAssetBase, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIFontConfigurationAssetBase as Default>::default())),
            create_boxed: || Box::new(<UIFontConfigurationAssetBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(UIFONTCONFIGURATIONASSETBASE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIFontConfigurationAssetBase {
    fn type_info(&self) -> &'static TypeInfo {
        UIFONTCONFIGURATIONASSETBASE_TYPE_INFO
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


pub static UIFONTCONFIGURATIONASSETBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIFontConfigurationAssetBase-Array",
    name_hash: 3774067403,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("UIFontConfigurationAssetBase"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WaterHealthComponentData {
    pub _glacier_base: GameHealthComponentData,
}

pub trait WaterHealthComponentDataTrait: GameHealthComponentDataTrait {
}

impl WaterHealthComponentDataTrait for WaterHealthComponentData {
}

impl GameHealthComponentDataTrait for WaterHealthComponentData {
}

impl HealthComponentDataTrait for WaterHealthComponentData {
}

impl super::entity::ComponentDataTrait for WaterHealthComponentData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
    fn components(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components_mut()
    }
    fn client_index(&self) -> &u8 {
        self._glacier_base.client_index()
    }
    fn client_index_mut(&mut self) -> &mut u8 {
        self._glacier_base.client_index_mut()
    }
    fn server_index(&self) -> &u8 {
        self._glacier_base.server_index()
    }
    fn server_index_mut(&mut self) -> &mut u8 {
        self._glacier_base.server_index_mut()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
    fn excluded_mut(&mut self) -> &mut bool {
        self._glacier_base.excluded_mut()
    }
}

impl super::entity::GameObjectDataTrait for WaterHealthComponentData {
}

impl super::core::DataBusPeerTrait for WaterHealthComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for WaterHealthComponentData {
}

impl super::core::DataContainerTrait for WaterHealthComponentData {
}

pub static WATERHEALTHCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterHealthComponentData",
    name_hash: 3445732083,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMEHEALTHCOMPONENTDATA_TYPE_INFO),
        super_class_offset: offset_of!(WaterHealthComponentData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaterHealthComponentData as Default>::default())),
            create_boxed: || Box::new(<WaterHealthComponentData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(WATERHEALTHCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WaterHealthComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        WATERHEALTHCOMPONENTDATA_TYPE_INFO
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


pub static WATERHEALTHCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterHealthComponentData-Array",
    name_hash: 179392199,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("WaterHealthComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TerrainHealthComponentData {
    pub _glacier_base: GameHealthComponentData,
}

pub trait TerrainHealthComponentDataTrait: GameHealthComponentDataTrait {
}

impl TerrainHealthComponentDataTrait for TerrainHealthComponentData {
}

impl GameHealthComponentDataTrait for TerrainHealthComponentData {
}

impl HealthComponentDataTrait for TerrainHealthComponentData {
}

impl super::entity::ComponentDataTrait for TerrainHealthComponentData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
    fn components(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components_mut()
    }
    fn client_index(&self) -> &u8 {
        self._glacier_base.client_index()
    }
    fn client_index_mut(&mut self) -> &mut u8 {
        self._glacier_base.client_index_mut()
    }
    fn server_index(&self) -> &u8 {
        self._glacier_base.server_index()
    }
    fn server_index_mut(&mut self) -> &mut u8 {
        self._glacier_base.server_index_mut()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
    fn excluded_mut(&mut self) -> &mut bool {
        self._glacier_base.excluded_mut()
    }
}

impl super::entity::GameObjectDataTrait for TerrainHealthComponentData {
}

impl super::core::DataBusPeerTrait for TerrainHealthComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for TerrainHealthComponentData {
}

impl super::core::DataContainerTrait for TerrainHealthComponentData {
}

pub static TERRAINHEALTHCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainHealthComponentData",
    name_hash: 1040082353,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMEHEALTHCOMPONENTDATA_TYPE_INFO),
        super_class_offset: offset_of!(TerrainHealthComponentData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainHealthComponentData as Default>::default())),
            create_boxed: || Box::new(<TerrainHealthComponentData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(TERRAINHEALTHCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TerrainHealthComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINHEALTHCOMPONENTDATA_TYPE_INFO
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


pub static TERRAINHEALTHCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainHealthComponentData-Array",
    name_hash: 2537330693,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("TerrainHealthComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WaterPhysicsComponentData {
    pub _glacier_base: GamePhysicsComponentData,
}

pub trait WaterPhysicsComponentDataTrait: GamePhysicsComponentDataTrait {
}

impl WaterPhysicsComponentDataTrait for WaterPhysicsComponentData {
}

impl GamePhysicsComponentDataTrait for WaterPhysicsComponentData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        self._glacier_base.realm_mut()
    }
    fn effect_parameters(&self) -> &Vec<Option<LockedTypeObject /* super::effect_base::EffectParameter */>> {
        self._glacier_base.effect_parameters()
    }
    fn effect_parameters_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::effect_base::EffectParameter */>> {
        self._glacier_base.effect_parameters_mut()
    }
}

impl super::physics::PhysicsComponentDataTrait for WaterPhysicsComponentData {
    fn physics_bodies(&self) -> &Vec<Option<LockedTypeObject /* super::physics::PhysicsBodyData */>> {
        self._glacier_base.physics_bodies()
    }
    fn physics_bodies_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::physics::PhysicsBodyData */>> {
        self._glacier_base.physics_bodies_mut()
    }
    fn physics_constraints(&self) -> &Vec<Option<LockedTypeObject /* super::physics::PhysicsConstraintData */>> {
        self._glacier_base.physics_constraints()
    }
    fn physics_constraints_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::physics::PhysicsConstraintData */>> {
        self._glacier_base.physics_constraints_mut()
    }
    fn parts(&self) -> &Vec<BoxedTypeObject /* super::physics::PhysicsPartData */> {
        self._glacier_base.parts()
    }
    fn parts_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::physics::PhysicsPartData */> {
        self._glacier_base.parts_mut()
    }
    fn movable_parts(&self) -> &bool {
        self._glacier_base.movable_parts()
    }
    fn movable_parts_mut(&mut self) -> &mut bool {
        self._glacier_base.movable_parts_mut()
    }
    fn internal_collision_disabling(&self) -> &super::physics::InternalCollisionDisablingBehavior {
        self._glacier_base.internal_collision_disabling()
    }
    fn internal_collision_disabling_mut(&mut self) -> &mut super::physics::InternalCollisionDisablingBehavior {
        self._glacier_base.internal_collision_disabling_mut()
    }
    fn enable_collision_events(&self) -> &bool {
        self._glacier_base.enable_collision_events()
    }
    fn enable_collision_events_mut(&mut self) -> &mut bool {
        self._glacier_base.enable_collision_events_mut()
    }
}

impl super::entity::ComponentDataTrait for WaterPhysicsComponentData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
    fn components(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components_mut()
    }
    fn client_index(&self) -> &u8 {
        self._glacier_base.client_index()
    }
    fn client_index_mut(&mut self) -> &mut u8 {
        self._glacier_base.client_index_mut()
    }
    fn server_index(&self) -> &u8 {
        self._glacier_base.server_index()
    }
    fn server_index_mut(&mut self) -> &mut u8 {
        self._glacier_base.server_index_mut()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
    fn excluded_mut(&mut self) -> &mut bool {
        self._glacier_base.excluded_mut()
    }
}

impl super::entity::GameObjectDataTrait for WaterPhysicsComponentData {
}

impl super::core::DataBusPeerTrait for WaterPhysicsComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for WaterPhysicsComponentData {
}

impl super::core::DataContainerTrait for WaterPhysicsComponentData {
}

pub static WATERPHYSICSCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterPhysicsComponentData",
    name_hash: 812883844,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMEPHYSICSCOMPONENTDATA_TYPE_INFO),
        super_class_offset: offset_of!(WaterPhysicsComponentData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaterPhysicsComponentData as Default>::default())),
            create_boxed: || Box::new(<WaterPhysicsComponentData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(WATERPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WaterPhysicsComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        WATERPHYSICSCOMPONENTDATA_TYPE_INFO
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


pub static WATERPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterPhysicsComponentData-Array",
    name_hash: 3916141488,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("WaterPhysicsComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TerrainPhysicsComponentData {
    pub _glacier_base: GamePhysicsComponentData,
}

pub trait TerrainPhysicsComponentDataTrait: GamePhysicsComponentDataTrait {
}

impl TerrainPhysicsComponentDataTrait for TerrainPhysicsComponentData {
}

impl GamePhysicsComponentDataTrait for TerrainPhysicsComponentData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        self._glacier_base.realm_mut()
    }
    fn effect_parameters(&self) -> &Vec<Option<LockedTypeObject /* super::effect_base::EffectParameter */>> {
        self._glacier_base.effect_parameters()
    }
    fn effect_parameters_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::effect_base::EffectParameter */>> {
        self._glacier_base.effect_parameters_mut()
    }
}

impl super::physics::PhysicsComponentDataTrait for TerrainPhysicsComponentData {
    fn physics_bodies(&self) -> &Vec<Option<LockedTypeObject /* super::physics::PhysicsBodyData */>> {
        self._glacier_base.physics_bodies()
    }
    fn physics_bodies_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::physics::PhysicsBodyData */>> {
        self._glacier_base.physics_bodies_mut()
    }
    fn physics_constraints(&self) -> &Vec<Option<LockedTypeObject /* super::physics::PhysicsConstraintData */>> {
        self._glacier_base.physics_constraints()
    }
    fn physics_constraints_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::physics::PhysicsConstraintData */>> {
        self._glacier_base.physics_constraints_mut()
    }
    fn parts(&self) -> &Vec<BoxedTypeObject /* super::physics::PhysicsPartData */> {
        self._glacier_base.parts()
    }
    fn parts_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::physics::PhysicsPartData */> {
        self._glacier_base.parts_mut()
    }
    fn movable_parts(&self) -> &bool {
        self._glacier_base.movable_parts()
    }
    fn movable_parts_mut(&mut self) -> &mut bool {
        self._glacier_base.movable_parts_mut()
    }
    fn internal_collision_disabling(&self) -> &super::physics::InternalCollisionDisablingBehavior {
        self._glacier_base.internal_collision_disabling()
    }
    fn internal_collision_disabling_mut(&mut self) -> &mut super::physics::InternalCollisionDisablingBehavior {
        self._glacier_base.internal_collision_disabling_mut()
    }
    fn enable_collision_events(&self) -> &bool {
        self._glacier_base.enable_collision_events()
    }
    fn enable_collision_events_mut(&mut self) -> &mut bool {
        self._glacier_base.enable_collision_events_mut()
    }
}

impl super::entity::ComponentDataTrait for TerrainPhysicsComponentData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
    fn components(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components_mut()
    }
    fn client_index(&self) -> &u8 {
        self._glacier_base.client_index()
    }
    fn client_index_mut(&mut self) -> &mut u8 {
        self._glacier_base.client_index_mut()
    }
    fn server_index(&self) -> &u8 {
        self._glacier_base.server_index()
    }
    fn server_index_mut(&mut self) -> &mut u8 {
        self._glacier_base.server_index_mut()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
    fn excluded_mut(&mut self) -> &mut bool {
        self._glacier_base.excluded_mut()
    }
}

impl super::entity::GameObjectDataTrait for TerrainPhysicsComponentData {
}

impl super::core::DataBusPeerTrait for TerrainPhysicsComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for TerrainPhysicsComponentData {
}

impl super::core::DataContainerTrait for TerrainPhysicsComponentData {
}

pub static TERRAINPHYSICSCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainPhysicsComponentData",
    name_hash: 2282039302,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMEPHYSICSCOMPONENTDATA_TYPE_INFO),
        super_class_offset: offset_of!(TerrainPhysicsComponentData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainPhysicsComponentData as Default>::default())),
            create_boxed: || Box::new(<TerrainPhysicsComponentData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(TERRAINPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TerrainPhysicsComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINPHYSICSCOMPONENTDATA_TYPE_INFO
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


pub static TERRAINPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainPhysicsComponentData-Array",
    name_hash: 4040013106,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("TerrainPhysicsComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WaterEntityData {
    pub _glacier_base: super::physics::GamePhysicsEntityData,
}

pub trait WaterEntityDataTrait: super::physics::GamePhysicsEntityDataTrait {
}

impl WaterEntityDataTrait for WaterEntityData {
}

impl super::physics::GamePhysicsEntityDataTrait for WaterEntityData {
}

impl super::entity::GameComponentEntityDataTrait for WaterEntityData {
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::entity::ComponentEntityDataTrait for WaterEntityData {
    fn components(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components_mut()
    }
    fn part_bounding_boxes(&self) -> &Vec<BoxedTypeObject /* super::core::AxisAlignedBox */> {
        self._glacier_base.part_bounding_boxes()
    }
    fn part_bounding_boxes_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::core::AxisAlignedBox */> {
        self._glacier_base.part_bounding_boxes_mut()
    }
    fn client_runtime_component_count(&self) -> &u8 {
        self._glacier_base.client_runtime_component_count()
    }
    fn client_runtime_component_count_mut(&mut self) -> &mut u8 {
        self._glacier_base.client_runtime_component_count_mut()
    }
    fn server_runtime_component_count(&self) -> &u8 {
        self._glacier_base.server_runtime_component_count()
    }
    fn server_runtime_component_count_mut(&mut self) -> &mut u8 {
        self._glacier_base.server_runtime_component_count_mut()
    }
    fn client_runtime_transformation_count(&self) -> &u8 {
        self._glacier_base.client_runtime_transformation_count()
    }
    fn client_runtime_transformation_count_mut(&mut self) -> &mut u8 {
        self._glacier_base.client_runtime_transformation_count_mut()
    }
    fn server_runtime_transformation_count(&self) -> &u8 {
        self._glacier_base.server_runtime_transformation_count()
    }
    fn server_runtime_transformation_count_mut(&mut self) -> &mut u8 {
        self._glacier_base.server_runtime_transformation_count_mut()
    }
}

impl super::entity::SpatialEntityDataTrait for WaterEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::entity::EntityDataTrait for WaterEntityData {
}

impl super::entity::GameObjectDataTrait for WaterEntityData {
}

impl super::core::DataBusPeerTrait for WaterEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for WaterEntityData {
}

impl super::core::DataContainerTrait for WaterEntityData {
}

pub static WATERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterEntityData",
    name_hash: 2881231323,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::GAMEPHYSICSENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(WaterEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaterEntityData as Default>::default())),
            create_boxed: || Box::new(<WaterEntityData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(WATERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WaterEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        WATERENTITYDATA_TYPE_INFO
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


pub static WATERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterEntityData-Array",
    name_hash: 2008826095,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("WaterEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TerrainEntityData {
    pub _glacier_base: super::physics::GamePhysicsEntityData,
    pub terrain_asset: Option<LockedTypeObject /* super::terrain_base::TerrainBaseAsset */>,
    pub water_material: super::entity::MaterialDecl,
    pub visible: bool,
    pub decals_resource: glacier_reflect::builtin::ResourceRef,
}

pub trait TerrainEntityDataTrait: super::physics::GamePhysicsEntityDataTrait {
    fn terrain_asset(&self) -> &Option<LockedTypeObject /* super::terrain_base::TerrainBaseAsset */>;
    fn terrain_asset_mut(&mut self) -> &mut Option<LockedTypeObject /* super::terrain_base::TerrainBaseAsset */>;
    fn water_material(&self) -> &super::entity::MaterialDecl;
    fn water_material_mut(&mut self) -> &mut super::entity::MaterialDecl;
    fn visible(&self) -> &bool;
    fn visible_mut(&mut self) -> &mut bool;
    fn decals_resource(&self) -> &glacier_reflect::builtin::ResourceRef;
    fn decals_resource_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef;
}

impl TerrainEntityDataTrait for TerrainEntityData {
    fn terrain_asset(&self) -> &Option<LockedTypeObject /* super::terrain_base::TerrainBaseAsset */> {
        &self.terrain_asset
    }
    fn terrain_asset_mut(&mut self) -> &mut Option<LockedTypeObject /* super::terrain_base::TerrainBaseAsset */> {
        &mut self.terrain_asset
    }
    fn water_material(&self) -> &super::entity::MaterialDecl {
        &self.water_material
    }
    fn water_material_mut(&mut self) -> &mut super::entity::MaterialDecl {
        &mut self.water_material
    }
    fn visible(&self) -> &bool {
        &self.visible
    }
    fn visible_mut(&mut self) -> &mut bool {
        &mut self.visible
    }
    fn decals_resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.decals_resource
    }
    fn decals_resource_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef {
        &mut self.decals_resource
    }
}

impl super::physics::GamePhysicsEntityDataTrait for TerrainEntityData {
}

impl super::entity::GameComponentEntityDataTrait for TerrainEntityData {
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::entity::ComponentEntityDataTrait for TerrainEntityData {
    fn components(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components_mut()
    }
    fn part_bounding_boxes(&self) -> &Vec<BoxedTypeObject /* super::core::AxisAlignedBox */> {
        self._glacier_base.part_bounding_boxes()
    }
    fn part_bounding_boxes_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::core::AxisAlignedBox */> {
        self._glacier_base.part_bounding_boxes_mut()
    }
    fn client_runtime_component_count(&self) -> &u8 {
        self._glacier_base.client_runtime_component_count()
    }
    fn client_runtime_component_count_mut(&mut self) -> &mut u8 {
        self._glacier_base.client_runtime_component_count_mut()
    }
    fn server_runtime_component_count(&self) -> &u8 {
        self._glacier_base.server_runtime_component_count()
    }
    fn server_runtime_component_count_mut(&mut self) -> &mut u8 {
        self._glacier_base.server_runtime_component_count_mut()
    }
    fn client_runtime_transformation_count(&self) -> &u8 {
        self._glacier_base.client_runtime_transformation_count()
    }
    fn client_runtime_transformation_count_mut(&mut self) -> &mut u8 {
        self._glacier_base.client_runtime_transformation_count_mut()
    }
    fn server_runtime_transformation_count(&self) -> &u8 {
        self._glacier_base.server_runtime_transformation_count()
    }
    fn server_runtime_transformation_count_mut(&mut self) -> &mut u8 {
        self._glacier_base.server_runtime_transformation_count_mut()
    }
}

impl super::entity::SpatialEntityDataTrait for TerrainEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::entity::EntityDataTrait for TerrainEntityData {
}

impl super::entity::GameObjectDataTrait for TerrainEntityData {
}

impl super::core::DataBusPeerTrait for TerrainEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for TerrainEntityData {
}

impl super::core::DataContainerTrait for TerrainEntityData {
}

pub static TERRAINENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainEntityData",
    name_hash: 1571616601,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::GAMEPHYSICSENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(TerrainEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainEntityData as Default>::default())),
            create_boxed: || Box::new(<TerrainEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "TerrainAsset",
                name_hash: 84434530,
                flags: MemberInfoFlags::new(0),
                field_type: "TerrainBaseAsset",
                rust_offset: offset_of!(TerrainEntityData, terrain_asset),
            },
            FieldInfoData {
                name: "WaterMaterial",
                name_hash: 2658409947,
                flags: MemberInfoFlags::new(0),
                field_type: "MaterialDecl",
                rust_offset: offset_of!(TerrainEntityData, water_material),
            },
            FieldInfoData {
                name: "Visible",
                name_hash: 901540267,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TerrainEntityData, visible),
            },
            FieldInfoData {
                name: "DecalsResource",
                name_hash: 3042841235,
                flags: MemberInfoFlags::new(0),
                field_type: "ResourceRef",
                rust_offset: offset_of!(TerrainEntityData, decals_resource),
            },
        ],
    }),
    array_type: Some(TERRAINENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TerrainEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINENTITYDATA_TYPE_INFO
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


pub static TERRAINENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainEntityData-Array",
    name_hash: 3348170093,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("TerrainEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ProfileOptionDataBinary {
    pub _glacier_base: ProfileOptionData,
    pub max_length: i32,
}

pub trait ProfileOptionDataBinaryTrait: ProfileOptionDataTrait {
    fn max_length(&self) -> &i32;
    fn max_length_mut(&mut self) -> &mut i32;
}

impl ProfileOptionDataBinaryTrait for ProfileOptionDataBinary {
    fn max_length(&self) -> &i32 {
        &self.max_length
    }
    fn max_length_mut(&mut self) -> &mut i32 {
        &mut self.max_length
    }
}

impl ProfileOptionDataTrait for ProfileOptionDataBinary {
    fn unique_id(&self) -> &String {
        self._glacier_base.unique_id()
    }
    fn unique_id_mut(&mut self) -> &mut String {
        self._glacier_base.unique_id_mut()
    }
    fn category(&self) -> &ProfileOptionsType {
        self._glacier_base.category()
    }
    fn category_mut(&mut self) -> &mut ProfileOptionsType {
        self._glacier_base.category_mut()
    }
    fn local_player_specific(&self) -> &bool {
        self._glacier_base.local_player_specific()
    }
    fn local_player_specific_mut(&mut self) -> &mut bool {
        self._glacier_base.local_player_specific_mut()
    }
    fn is_boot_option(&self) -> &bool {
        self._glacier_base.is_boot_option()
    }
    fn is_boot_option_mut(&mut self) -> &mut bool {
        self._glacier_base.is_boot_option_mut()
    }
}

impl super::core::AssetTrait for ProfileOptionDataBinary {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for ProfileOptionDataBinary {
}

pub static PROFILEOPTIONDATABINARY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionDataBinary",
    name_hash: 1663748306,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROFILEOPTIONDATA_TYPE_INFO),
        super_class_offset: offset_of!(ProfileOptionDataBinary, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ProfileOptionDataBinary as Default>::default())),
            create_boxed: || Box::new(<ProfileOptionDataBinary as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "MaxLength",
                name_hash: 1300353773,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ProfileOptionDataBinary, max_length),
            },
        ],
    }),
    array_type: Some(PROFILEOPTIONDATABINARY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProfileOptionDataBinary {
    fn type_info(&self) -> &'static TypeInfo {
        PROFILEOPTIONDATABINARY_TYPE_INFO
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


pub static PROFILEOPTIONDATABINARY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionDataBinary-Array",
    name_hash: 2902212710,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ProfileOptionDataBinary"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ProfileOptionDataString {
    pub _glacier_base: ProfileOptionData,
    pub max_length: i32,
    pub value: String,
}

pub trait ProfileOptionDataStringTrait: ProfileOptionDataTrait {
    fn max_length(&self) -> &i32;
    fn max_length_mut(&mut self) -> &mut i32;
    fn value(&self) -> &String;
    fn value_mut(&mut self) -> &mut String;
}

impl ProfileOptionDataStringTrait for ProfileOptionDataString {
    fn max_length(&self) -> &i32 {
        &self.max_length
    }
    fn max_length_mut(&mut self) -> &mut i32 {
        &mut self.max_length
    }
    fn value(&self) -> &String {
        &self.value
    }
    fn value_mut(&mut self) -> &mut String {
        &mut self.value
    }
}

impl ProfileOptionDataTrait for ProfileOptionDataString {
    fn unique_id(&self) -> &String {
        self._glacier_base.unique_id()
    }
    fn unique_id_mut(&mut self) -> &mut String {
        self._glacier_base.unique_id_mut()
    }
    fn category(&self) -> &ProfileOptionsType {
        self._glacier_base.category()
    }
    fn category_mut(&mut self) -> &mut ProfileOptionsType {
        self._glacier_base.category_mut()
    }
    fn local_player_specific(&self) -> &bool {
        self._glacier_base.local_player_specific()
    }
    fn local_player_specific_mut(&mut self) -> &mut bool {
        self._glacier_base.local_player_specific_mut()
    }
    fn is_boot_option(&self) -> &bool {
        self._glacier_base.is_boot_option()
    }
    fn is_boot_option_mut(&mut self) -> &mut bool {
        self._glacier_base.is_boot_option_mut()
    }
}

impl super::core::AssetTrait for ProfileOptionDataString {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for ProfileOptionDataString {
}

pub static PROFILEOPTIONDATASTRING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionDataString",
    name_hash: 1096291048,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROFILEOPTIONDATA_TYPE_INFO),
        super_class_offset: offset_of!(ProfileOptionDataString, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ProfileOptionDataString as Default>::default())),
            create_boxed: || Box::new(<ProfileOptionDataString as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "MaxLength",
                name_hash: 1300353773,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ProfileOptionDataString, max_length),
            },
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ProfileOptionDataString, value),
            },
        ],
    }),
    array_type: Some(PROFILEOPTIONDATASTRING_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProfileOptionDataString {
    fn type_info(&self) -> &'static TypeInfo {
        PROFILEOPTIONDATASTRING_TYPE_INFO
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


pub static PROFILEOPTIONDATASTRING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionDataString-Array",
    name_hash: 3896804572,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ProfileOptionDataString"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ProfileOptionDataBool {
    pub _glacier_base: ProfileOptionData,
    pub value: bool,
}

pub trait ProfileOptionDataBoolTrait: ProfileOptionDataTrait {
    fn value(&self) -> &bool;
    fn value_mut(&mut self) -> &mut bool;
}

impl ProfileOptionDataBoolTrait for ProfileOptionDataBool {
    fn value(&self) -> &bool {
        &self.value
    }
    fn value_mut(&mut self) -> &mut bool {
        &mut self.value
    }
}

impl ProfileOptionDataTrait for ProfileOptionDataBool {
    fn unique_id(&self) -> &String {
        self._glacier_base.unique_id()
    }
    fn unique_id_mut(&mut self) -> &mut String {
        self._glacier_base.unique_id_mut()
    }
    fn category(&self) -> &ProfileOptionsType {
        self._glacier_base.category()
    }
    fn category_mut(&mut self) -> &mut ProfileOptionsType {
        self._glacier_base.category_mut()
    }
    fn local_player_specific(&self) -> &bool {
        self._glacier_base.local_player_specific()
    }
    fn local_player_specific_mut(&mut self) -> &mut bool {
        self._glacier_base.local_player_specific_mut()
    }
    fn is_boot_option(&self) -> &bool {
        self._glacier_base.is_boot_option()
    }
    fn is_boot_option_mut(&mut self) -> &mut bool {
        self._glacier_base.is_boot_option_mut()
    }
}

impl super::core::AssetTrait for ProfileOptionDataBool {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for ProfileOptionDataBool {
}

pub static PROFILEOPTIONDATABOOL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionDataBool",
    name_hash: 2632135795,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROFILEOPTIONDATA_TYPE_INFO),
        super_class_offset: offset_of!(ProfileOptionDataBool, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ProfileOptionDataBool as Default>::default())),
            create_boxed: || Box::new(<ProfileOptionDataBool as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ProfileOptionDataBool, value),
            },
        ],
    }),
    array_type: Some(PROFILEOPTIONDATABOOL_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProfileOptionDataBool {
    fn type_info(&self) -> &'static TypeInfo {
        PROFILEOPTIONDATABOOL_TYPE_INFO
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


pub static PROFILEOPTIONDATABOOL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionDataBool-Array",
    name_hash: 1286467655,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ProfileOptionDataBool"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ProfileOptionDataEnum {
    pub _glacier_base: ProfileOptionData,
    pub items: Vec<BoxedTypeObject /* ProfileOptionDataEnumItem */>,
}

pub trait ProfileOptionDataEnumTrait: ProfileOptionDataTrait {
    fn items(&self) -> &Vec<BoxedTypeObject /* ProfileOptionDataEnumItem */>;
    fn items_mut(&mut self) -> &mut Vec<BoxedTypeObject /* ProfileOptionDataEnumItem */>;
}

impl ProfileOptionDataEnumTrait for ProfileOptionDataEnum {
    fn items(&self) -> &Vec<BoxedTypeObject /* ProfileOptionDataEnumItem */> {
        &self.items
    }
    fn items_mut(&mut self) -> &mut Vec<BoxedTypeObject /* ProfileOptionDataEnumItem */> {
        &mut self.items
    }
}

impl ProfileOptionDataTrait for ProfileOptionDataEnum {
    fn unique_id(&self) -> &String {
        self._glacier_base.unique_id()
    }
    fn unique_id_mut(&mut self) -> &mut String {
        self._glacier_base.unique_id_mut()
    }
    fn category(&self) -> &ProfileOptionsType {
        self._glacier_base.category()
    }
    fn category_mut(&mut self) -> &mut ProfileOptionsType {
        self._glacier_base.category_mut()
    }
    fn local_player_specific(&self) -> &bool {
        self._glacier_base.local_player_specific()
    }
    fn local_player_specific_mut(&mut self) -> &mut bool {
        self._glacier_base.local_player_specific_mut()
    }
    fn is_boot_option(&self) -> &bool {
        self._glacier_base.is_boot_option()
    }
    fn is_boot_option_mut(&mut self) -> &mut bool {
        self._glacier_base.is_boot_option_mut()
    }
}

impl super::core::AssetTrait for ProfileOptionDataEnum {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for ProfileOptionDataEnum {
}

pub static PROFILEOPTIONDATAENUM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionDataEnum",
    name_hash: 2631825294,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROFILEOPTIONDATA_TYPE_INFO),
        super_class_offset: offset_of!(ProfileOptionDataEnum, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ProfileOptionDataEnum as Default>::default())),
            create_boxed: || Box::new(<ProfileOptionDataEnum as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Items",
                name_hash: 215446531,
                flags: MemberInfoFlags::new(144),
                field_type: "ProfileOptionDataEnumItem-Array",
                rust_offset: offset_of!(ProfileOptionDataEnum, items),
            },
        ],
    }),
    array_type: Some(PROFILEOPTIONDATAENUM_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProfileOptionDataEnum {
    fn type_info(&self) -> &'static TypeInfo {
        PROFILEOPTIONDATAENUM_TYPE_INFO
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


pub static PROFILEOPTIONDATAENUM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionDataEnum-Array",
    name_hash: 2093220026,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ProfileOptionDataEnum"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ProfileOptionDataEnumItem {
    pub display_name: String,
    pub default: bool,
}

pub trait ProfileOptionDataEnumItemTrait: TypeObject {
    fn display_name(&self) -> &String;
    fn display_name_mut(&mut self) -> &mut String;
    fn default(&self) -> &bool;
    fn default_mut(&mut self) -> &mut bool;
}

impl ProfileOptionDataEnumItemTrait for ProfileOptionDataEnumItem {
    fn display_name(&self) -> &String {
        &self.display_name
    }
    fn display_name_mut(&mut self) -> &mut String {
        &mut self.display_name
    }
    fn default(&self) -> &bool {
        &self.default
    }
    fn default_mut(&mut self) -> &mut bool {
        &mut self.default
    }
}

pub static PROFILEOPTIONDATAENUMITEM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionDataEnumItem",
    name_hash: 1327705659,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ProfileOptionDataEnumItem as Default>::default())),
            create_boxed: || Box::new(<ProfileOptionDataEnumItem as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "DisplayName",
                name_hash: 1925224568,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ProfileOptionDataEnumItem, display_name),
            },
            FieldInfoData {
                name: "Default",
                name_hash: 3998752238,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ProfileOptionDataEnumItem, default),
            },
        ],
    }),
    array_type: Some(PROFILEOPTIONDATAENUMITEM_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProfileOptionDataEnumItem {
    fn type_info(&self) -> &'static TypeInfo {
        PROFILEOPTIONDATAENUMITEM_TYPE_INFO
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


pub static PROFILEOPTIONDATAENUMITEM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionDataEnumItem-Array",
    name_hash: 3433928079,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ProfileOptionDataEnumItem"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ProfileOptionDataFloat {
    pub _glacier_base: ProfileOptionData,
    pub min: f32,
    pub max: f32,
    pub value: f32,
    pub step: f32,
}

pub trait ProfileOptionDataFloatTrait: ProfileOptionDataTrait {
    fn min(&self) -> &f32;
    fn min_mut(&mut self) -> &mut f32;
    fn max(&self) -> &f32;
    fn max_mut(&mut self) -> &mut f32;
    fn value(&self) -> &f32;
    fn value_mut(&mut self) -> &mut f32;
    fn step(&self) -> &f32;
    fn step_mut(&mut self) -> &mut f32;
}

impl ProfileOptionDataFloatTrait for ProfileOptionDataFloat {
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
    fn value(&self) -> &f32 {
        &self.value
    }
    fn value_mut(&mut self) -> &mut f32 {
        &mut self.value
    }
    fn step(&self) -> &f32 {
        &self.step
    }
    fn step_mut(&mut self) -> &mut f32 {
        &mut self.step
    }
}

impl ProfileOptionDataTrait for ProfileOptionDataFloat {
    fn unique_id(&self) -> &String {
        self._glacier_base.unique_id()
    }
    fn unique_id_mut(&mut self) -> &mut String {
        self._glacier_base.unique_id_mut()
    }
    fn category(&self) -> &ProfileOptionsType {
        self._glacier_base.category()
    }
    fn category_mut(&mut self) -> &mut ProfileOptionsType {
        self._glacier_base.category_mut()
    }
    fn local_player_specific(&self) -> &bool {
        self._glacier_base.local_player_specific()
    }
    fn local_player_specific_mut(&mut self) -> &mut bool {
        self._glacier_base.local_player_specific_mut()
    }
    fn is_boot_option(&self) -> &bool {
        self._glacier_base.is_boot_option()
    }
    fn is_boot_option_mut(&mut self) -> &mut bool {
        self._glacier_base.is_boot_option_mut()
    }
}

impl super::core::AssetTrait for ProfileOptionDataFloat {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for ProfileOptionDataFloat {
}

pub static PROFILEOPTIONDATAFLOAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionDataFloat",
    name_hash: 956837549,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROFILEOPTIONDATA_TYPE_INFO),
        super_class_offset: offset_of!(ProfileOptionDataFloat, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ProfileOptionDataFloat as Default>::default())),
            create_boxed: || Box::new(<ProfileOptionDataFloat as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Min",
                name_hash: 193446607,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ProfileOptionDataFloat, min),
            },
            FieldInfoData {
                name: "Max",
                name_hash: 193446865,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ProfileOptionDataFloat, max),
            },
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ProfileOptionDataFloat, value),
            },
            FieldInfoData {
                name: "Step",
                name_hash: 2089400887,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ProfileOptionDataFloat, step),
            },
        ],
    }),
    array_type: Some(PROFILEOPTIONDATAFLOAT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProfileOptionDataFloat {
    fn type_info(&self) -> &'static TypeInfo {
        PROFILEOPTIONDATAFLOAT_TYPE_INFO
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


pub static PROFILEOPTIONDATAFLOAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionDataFloat-Array",
    name_hash: 48587545,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ProfileOptionDataFloat"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ProfileOptionDataInt {
    pub _glacier_base: ProfileOptionData,
    pub min: i32,
    pub max: i32,
    pub value: i32,
    pub step: i32,
}

pub trait ProfileOptionDataIntTrait: ProfileOptionDataTrait {
    fn min(&self) -> &i32;
    fn min_mut(&mut self) -> &mut i32;
    fn max(&self) -> &i32;
    fn max_mut(&mut self) -> &mut i32;
    fn value(&self) -> &i32;
    fn value_mut(&mut self) -> &mut i32;
    fn step(&self) -> &i32;
    fn step_mut(&mut self) -> &mut i32;
}

impl ProfileOptionDataIntTrait for ProfileOptionDataInt {
    fn min(&self) -> &i32 {
        &self.min
    }
    fn min_mut(&mut self) -> &mut i32 {
        &mut self.min
    }
    fn max(&self) -> &i32 {
        &self.max
    }
    fn max_mut(&mut self) -> &mut i32 {
        &mut self.max
    }
    fn value(&self) -> &i32 {
        &self.value
    }
    fn value_mut(&mut self) -> &mut i32 {
        &mut self.value
    }
    fn step(&self) -> &i32 {
        &self.step
    }
    fn step_mut(&mut self) -> &mut i32 {
        &mut self.step
    }
}

impl ProfileOptionDataTrait for ProfileOptionDataInt {
    fn unique_id(&self) -> &String {
        self._glacier_base.unique_id()
    }
    fn unique_id_mut(&mut self) -> &mut String {
        self._glacier_base.unique_id_mut()
    }
    fn category(&self) -> &ProfileOptionsType {
        self._glacier_base.category()
    }
    fn category_mut(&mut self) -> &mut ProfileOptionsType {
        self._glacier_base.category_mut()
    }
    fn local_player_specific(&self) -> &bool {
        self._glacier_base.local_player_specific()
    }
    fn local_player_specific_mut(&mut self) -> &mut bool {
        self._glacier_base.local_player_specific_mut()
    }
    fn is_boot_option(&self) -> &bool {
        self._glacier_base.is_boot_option()
    }
    fn is_boot_option_mut(&mut self) -> &mut bool {
        self._glacier_base.is_boot_option_mut()
    }
}

impl super::core::AssetTrait for ProfileOptionDataInt {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for ProfileOptionDataInt {
}

pub static PROFILEOPTIONDATAINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionDataInt",
    name_hash: 3073210254,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROFILEOPTIONDATA_TYPE_INFO),
        super_class_offset: offset_of!(ProfileOptionDataInt, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ProfileOptionDataInt as Default>::default())),
            create_boxed: || Box::new(<ProfileOptionDataInt as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Min",
                name_hash: 193446607,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ProfileOptionDataInt, min),
            },
            FieldInfoData {
                name: "Max",
                name_hash: 193446865,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ProfileOptionDataInt, max),
            },
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ProfileOptionDataInt, value),
            },
            FieldInfoData {
                name: "Step",
                name_hash: 2089400887,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ProfileOptionDataInt, step),
            },
        ],
    }),
    array_type: Some(PROFILEOPTIONDATAINT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProfileOptionDataInt {
    fn type_info(&self) -> &'static TypeInfo {
        PROFILEOPTIONDATAINT_TYPE_INFO
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


pub static PROFILEOPTIONDATAINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionDataInt-Array",
    name_hash: 3595763898,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ProfileOptionDataInt"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ProfileOptionData {
    pub _glacier_base: super::core::Asset,
    pub unique_id: String,
    pub category: ProfileOptionsType,
    pub local_player_specific: bool,
    pub is_boot_option: bool,
}

pub trait ProfileOptionDataTrait: super::core::AssetTrait {
    fn unique_id(&self) -> &String;
    fn unique_id_mut(&mut self) -> &mut String;
    fn category(&self) -> &ProfileOptionsType;
    fn category_mut(&mut self) -> &mut ProfileOptionsType;
    fn local_player_specific(&self) -> &bool;
    fn local_player_specific_mut(&mut self) -> &mut bool;
    fn is_boot_option(&self) -> &bool;
    fn is_boot_option_mut(&mut self) -> &mut bool;
}

impl ProfileOptionDataTrait for ProfileOptionData {
    fn unique_id(&self) -> &String {
        &self.unique_id
    }
    fn unique_id_mut(&mut self) -> &mut String {
        &mut self.unique_id
    }
    fn category(&self) -> &ProfileOptionsType {
        &self.category
    }
    fn category_mut(&mut self) -> &mut ProfileOptionsType {
        &mut self.category
    }
    fn local_player_specific(&self) -> &bool {
        &self.local_player_specific
    }
    fn local_player_specific_mut(&mut self) -> &mut bool {
        &mut self.local_player_specific
    }
    fn is_boot_option(&self) -> &bool {
        &self.is_boot_option
    }
    fn is_boot_option_mut(&mut self) -> &mut bool {
        &mut self.is_boot_option
    }
}

impl super::core::AssetTrait for ProfileOptionData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for ProfileOptionData {
}

pub static PROFILEOPTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionData",
    name_hash: 1651885693,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(ProfileOptionData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ProfileOptionData as Default>::default())),
            create_boxed: || Box::new(<ProfileOptionData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "UniqueId",
                name_hash: 3045776603,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ProfileOptionData, unique_id),
            },
            FieldInfoData {
                name: "Category",
                name_hash: 3455858997,
                flags: MemberInfoFlags::new(0),
                field_type: "ProfileOptionsType",
                rust_offset: offset_of!(ProfileOptionData, category),
            },
            FieldInfoData {
                name: "LocalPlayerSpecific",
                name_hash: 1883450939,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ProfileOptionData, local_player_specific),
            },
            FieldInfoData {
                name: "IsBootOption",
                name_hash: 3144588650,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ProfileOptionData, is_boot_option),
            },
        ],
    }),
    array_type: Some(PROFILEOPTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProfileOptionData {
    fn type_info(&self) -> &'static TypeInfo {
        PROFILEOPTIONDATA_TYPE_INFO
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


pub static PROFILEOPTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionData-Array",
    name_hash: 4168667977,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ProfileOptionData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ProfileOptionsAsset {
    pub _glacier_base: super::core::Asset,
    pub file_name: String,
    pub content_name: String,
    pub file_size: u32,
    pub auto_save_on_quit: bool,
    pub options: Vec<Option<LockedTypeObject /* ProfileOptionData */>>,
    pub options_ps3: Vec<Option<LockedTypeObject /* ProfileOptionData */>>,
    pub options_xenon: Vec<Option<LockedTypeObject /* ProfileOptionData */>>,
    pub options_gen4a: Vec<Option<LockedTypeObject /* ProfileOptionData */>>,
    pub options_gen4b: Vec<Option<LockedTypeObject /* ProfileOptionData */>>,
    pub options_win: Vec<Option<LockedTypeObject /* ProfileOptionData */>>,
    pub options_android: Vec<Option<LockedTypeObject /* ProfileOptionData */>>,
    pub optionsi_o_s: Vec<Option<LockedTypeObject /* ProfileOptionData */>>,
}

pub trait ProfileOptionsAssetTrait: super::core::AssetTrait {
    fn file_name(&self) -> &String;
    fn file_name_mut(&mut self) -> &mut String;
    fn content_name(&self) -> &String;
    fn content_name_mut(&mut self) -> &mut String;
    fn file_size(&self) -> &u32;
    fn file_size_mut(&mut self) -> &mut u32;
    fn auto_save_on_quit(&self) -> &bool;
    fn auto_save_on_quit_mut(&mut self) -> &mut bool;
    fn options(&self) -> &Vec<Option<LockedTypeObject /* ProfileOptionData */>>;
    fn options_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* ProfileOptionData */>>;
    fn options_ps3(&self) -> &Vec<Option<LockedTypeObject /* ProfileOptionData */>>;
    fn options_ps3_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* ProfileOptionData */>>;
    fn options_xenon(&self) -> &Vec<Option<LockedTypeObject /* ProfileOptionData */>>;
    fn options_xenon_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* ProfileOptionData */>>;
    fn options_gen4a(&self) -> &Vec<Option<LockedTypeObject /* ProfileOptionData */>>;
    fn options_gen4a_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* ProfileOptionData */>>;
    fn options_gen4b(&self) -> &Vec<Option<LockedTypeObject /* ProfileOptionData */>>;
    fn options_gen4b_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* ProfileOptionData */>>;
    fn options_win(&self) -> &Vec<Option<LockedTypeObject /* ProfileOptionData */>>;
    fn options_win_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* ProfileOptionData */>>;
    fn options_android(&self) -> &Vec<Option<LockedTypeObject /* ProfileOptionData */>>;
    fn options_android_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* ProfileOptionData */>>;
    fn optionsi_o_s(&self) -> &Vec<Option<LockedTypeObject /* ProfileOptionData */>>;
    fn optionsi_o_s_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* ProfileOptionData */>>;
}

impl ProfileOptionsAssetTrait for ProfileOptionsAsset {
    fn file_name(&self) -> &String {
        &self.file_name
    }
    fn file_name_mut(&mut self) -> &mut String {
        &mut self.file_name
    }
    fn content_name(&self) -> &String {
        &self.content_name
    }
    fn content_name_mut(&mut self) -> &mut String {
        &mut self.content_name
    }
    fn file_size(&self) -> &u32 {
        &self.file_size
    }
    fn file_size_mut(&mut self) -> &mut u32 {
        &mut self.file_size
    }
    fn auto_save_on_quit(&self) -> &bool {
        &self.auto_save_on_quit
    }
    fn auto_save_on_quit_mut(&mut self) -> &mut bool {
        &mut self.auto_save_on_quit
    }
    fn options(&self) -> &Vec<Option<LockedTypeObject /* ProfileOptionData */>> {
        &self.options
    }
    fn options_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* ProfileOptionData */>> {
        &mut self.options
    }
    fn options_ps3(&self) -> &Vec<Option<LockedTypeObject /* ProfileOptionData */>> {
        &self.options_ps3
    }
    fn options_ps3_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* ProfileOptionData */>> {
        &mut self.options_ps3
    }
    fn options_xenon(&self) -> &Vec<Option<LockedTypeObject /* ProfileOptionData */>> {
        &self.options_xenon
    }
    fn options_xenon_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* ProfileOptionData */>> {
        &mut self.options_xenon
    }
    fn options_gen4a(&self) -> &Vec<Option<LockedTypeObject /* ProfileOptionData */>> {
        &self.options_gen4a
    }
    fn options_gen4a_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* ProfileOptionData */>> {
        &mut self.options_gen4a
    }
    fn options_gen4b(&self) -> &Vec<Option<LockedTypeObject /* ProfileOptionData */>> {
        &self.options_gen4b
    }
    fn options_gen4b_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* ProfileOptionData */>> {
        &mut self.options_gen4b
    }
    fn options_win(&self) -> &Vec<Option<LockedTypeObject /* ProfileOptionData */>> {
        &self.options_win
    }
    fn options_win_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* ProfileOptionData */>> {
        &mut self.options_win
    }
    fn options_android(&self) -> &Vec<Option<LockedTypeObject /* ProfileOptionData */>> {
        &self.options_android
    }
    fn options_android_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* ProfileOptionData */>> {
        &mut self.options_android
    }
    fn optionsi_o_s(&self) -> &Vec<Option<LockedTypeObject /* ProfileOptionData */>> {
        &self.optionsi_o_s
    }
    fn optionsi_o_s_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* ProfileOptionData */>> {
        &mut self.optionsi_o_s
    }
}

impl super::core::AssetTrait for ProfileOptionsAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for ProfileOptionsAsset {
}

pub static PROFILEOPTIONSASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionsAsset",
    name_hash: 3144204622,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(ProfileOptionsAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ProfileOptionsAsset as Default>::default())),
            create_boxed: || Box::new(<ProfileOptionsAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "FileName",
                name_hash: 1134474212,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ProfileOptionsAsset, file_name),
            },
            FieldInfoData {
                name: "ContentName",
                name_hash: 4236678091,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ProfileOptionsAsset, content_name),
            },
            FieldInfoData {
                name: "FileSize",
                name_hash: 1134658022,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ProfileOptionsAsset, file_size),
            },
            FieldInfoData {
                name: "AutoSaveOnQuit",
                name_hash: 2066387635,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ProfileOptionsAsset, auto_save_on_quit),
            },
            FieldInfoData {
                name: "Options",
                name_hash: 958915349,
                flags: MemberInfoFlags::new(144),
                field_type: "ProfileOptionData-Array",
                rust_offset: offset_of!(ProfileOptionsAsset, options),
            },
            FieldInfoData {
                name: "OptionsPs3",
                name_hash: 2018337157,
                flags: MemberInfoFlags::new(144),
                field_type: "ProfileOptionData-Array",
                rust_offset: offset_of!(ProfileOptionsAsset, options_ps3),
            },
            FieldInfoData {
                name: "OptionsXenon",
                name_hash: 3249518119,
                flags: MemberInfoFlags::new(144),
                field_type: "ProfileOptionData-Array",
                rust_offset: offset_of!(ProfileOptionsAsset, options_xenon),
            },
            FieldInfoData {
                name: "OptionsGen4a",
                name_hash: 3253650316,
                flags: MemberInfoFlags::new(144),
                field_type: "ProfileOptionData-Array",
                rust_offset: offset_of!(ProfileOptionsAsset, options_gen4a),
            },
            FieldInfoData {
                name: "OptionsGen4b",
                name_hash: 3253650319,
                flags: MemberInfoFlags::new(144),
                field_type: "ProfileOptionData-Array",
                rust_offset: offset_of!(ProfileOptionsAsset, options_gen4b),
            },
            FieldInfoData {
                name: "OptionsWin",
                name_hash: 2018331589,
                flags: MemberInfoFlags::new(144),
                field_type: "ProfileOptionData-Array",
                rust_offset: offset_of!(ProfileOptionsAsset, options_win),
            },
            FieldInfoData {
                name: "OptionsAndroid",
                name_hash: 1709040014,
                flags: MemberInfoFlags::new(144),
                field_type: "ProfileOptionData-Array",
                rust_offset: offset_of!(ProfileOptionsAsset, options_android),
            },
            FieldInfoData {
                name: "OptionsiOS",
                name_hash: 2018321184,
                flags: MemberInfoFlags::new(144),
                field_type: "ProfileOptionData-Array",
                rust_offset: offset_of!(ProfileOptionsAsset, optionsi_o_s),
            },
        ],
    }),
    array_type: Some(PROFILEOPTIONSASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProfileOptionsAsset {
    fn type_info(&self) -> &'static TypeInfo {
        PROFILEOPTIONSASSET_TYPE_INFO
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


pub static PROFILEOPTIONSASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionsAsset-Array",
    name_hash: 4213103354,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ProfileOptionsAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct BinaryOption {
    pub name: String,
}

pub trait BinaryOptionTrait: TypeObject {
    fn name(&self) -> &String;
    fn name_mut(&mut self) -> &mut String;
}

impl BinaryOptionTrait for BinaryOption {
    fn name(&self) -> &String {
        &self.name
    }
    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
}

pub static BINARYOPTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BinaryOption",
    name_hash: 2532984137,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BinaryOption as Default>::default())),
            create_boxed: || Box::new(<BinaryOption as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                name_hash: 2088949890,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(BinaryOption, name),
            },
        ],
    }),
    array_type: Some(BINARYOPTION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BinaryOption {
    fn type_info(&self) -> &'static TypeInfo {
        BINARYOPTION_TYPE_INFO
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


pub static BINARYOPTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BinaryOption-Array",
    name_hash: 1972243325,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("BinaryOption"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct StringOption {
    pub name: String,
    pub value: String,
}

pub trait StringOptionTrait: TypeObject {
    fn name(&self) -> &String;
    fn name_mut(&mut self) -> &mut String;
    fn value(&self) -> &String;
    fn value_mut(&mut self) -> &mut String;
}

impl StringOptionTrait for StringOption {
    fn name(&self) -> &String {
        &self.name
    }
    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    fn value(&self) -> &String {
        &self.value
    }
    fn value_mut(&mut self) -> &mut String {
        &mut self.value
    }
}

pub static STRINGOPTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StringOption",
    name_hash: 4126889139,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StringOption as Default>::default())),
            create_boxed: || Box::new(<StringOption as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                name_hash: 2088949890,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(StringOption, name),
            },
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(StringOption, value),
            },
        ],
    }),
    array_type: Some(STRINGOPTION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StringOption {
    fn type_info(&self) -> &'static TypeInfo {
        STRINGOPTION_TYPE_INFO
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


pub static STRINGOPTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StringOption-Array",
    name_hash: 2502508551,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("StringOption"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct IntOption {
    pub name: String,
    pub value: i32,
}

pub trait IntOptionTrait: TypeObject {
    fn name(&self) -> &String;
    fn name_mut(&mut self) -> &mut String;
    fn value(&self) -> &i32;
    fn value_mut(&mut self) -> &mut i32;
}

impl IntOptionTrait for IntOption {
    fn name(&self) -> &String {
        &self.name
    }
    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    fn value(&self) -> &i32 {
        &self.value
    }
    fn value_mut(&mut self) -> &mut i32 {
        &mut self.value
    }
}

pub static INTOPTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntOption",
    name_hash: 1072455509,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IntOption as Default>::default())),
            create_boxed: || Box::new(<IntOption as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                name_hash: 2088949890,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(IntOption, name),
            },
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(IntOption, value),
            },
        ],
    }),
    array_type: Some(INTOPTION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IntOption {
    fn type_info(&self) -> &'static TypeInfo {
        INTOPTION_TYPE_INFO
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


pub static INTOPTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntOption-Array",
    name_hash: 2759769953,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("IntOption"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct FloatOption {
    pub name: String,
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub step: f32,
}

pub trait FloatOptionTrait: TypeObject {
    fn name(&self) -> &String;
    fn name_mut(&mut self) -> &mut String;
    fn value(&self) -> &f32;
    fn value_mut(&mut self) -> &mut f32;
    fn min(&self) -> &f32;
    fn min_mut(&mut self) -> &mut f32;
    fn max(&self) -> &f32;
    fn max_mut(&mut self) -> &mut f32;
    fn step(&self) -> &f32;
    fn step_mut(&mut self) -> &mut f32;
}

impl FloatOptionTrait for FloatOption {
    fn name(&self) -> &String {
        &self.name
    }
    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    fn value(&self) -> &f32 {
        &self.value
    }
    fn value_mut(&mut self) -> &mut f32 {
        &mut self.value
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
    fn step(&self) -> &f32 {
        &self.step
    }
    fn step_mut(&mut self) -> &mut f32 {
        &mut self.step
    }
}

pub static FLOATOPTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatOption",
    name_hash: 658310134,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FloatOption as Default>::default())),
            create_boxed: || Box::new(<FloatOption as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                name_hash: 2088949890,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(FloatOption, name),
            },
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FloatOption, value),
            },
            FieldInfoData {
                name: "Min",
                name_hash: 193446607,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FloatOption, min),
            },
            FieldInfoData {
                name: "Max",
                name_hash: 193446865,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FloatOption, max),
            },
            FieldInfoData {
                name: "Step",
                name_hash: 2089400887,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FloatOption, step),
            },
        ],
    }),
    array_type: Some(FLOATOPTION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FloatOption {
    fn type_info(&self) -> &'static TypeInfo {
        FLOATOPTION_TYPE_INFO
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


pub static FLOATOPTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatOption-Array",
    name_hash: 2220460226,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("FloatOption"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ProfileOptionsType {
    #[default]
    GstAudio = 0,
    GstRender = 1,
    GstInput = 2,
    GstAI = 3,
    GstGameplay = 4,
    GstPlayerProfile = 5,
    GstPersistence = 6,
    GstBinary = 7,
    GstKeyBinding = 8,
    GstCount = 9,
}

pub static PROFILEOPTIONSTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionsType",
    name_hash: 2957934822,
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(PROFILEOPTIONSTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ProfileOptionsType {
    fn type_info(&self) -> &'static TypeInfo {
        PROFILEOPTIONSTYPE_TYPE_INFO
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


pub static PROFILEOPTIONSTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionsType-Array",
    name_hash: 910294482,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ProfileOptionsType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MaterialRelationTerrainDestructionData {
    pub _glacier_base: super::entity::PhysicsPropertyRelationPropertyData,
    pub dynamic_decal_template: Option<LockedTypeObject /* super::terrain_base::TerrainDynamicDecalTemplateData */>,
}

pub trait MaterialRelationTerrainDestructionDataTrait: super::entity::PhysicsPropertyRelationPropertyDataTrait {
    fn dynamic_decal_template(&self) -> &Option<LockedTypeObject /* super::terrain_base::TerrainDynamicDecalTemplateData */>;
    fn dynamic_decal_template_mut(&mut self) -> &mut Option<LockedTypeObject /* super::terrain_base::TerrainDynamicDecalTemplateData */>;
}

impl MaterialRelationTerrainDestructionDataTrait for MaterialRelationTerrainDestructionData {
    fn dynamic_decal_template(&self) -> &Option<LockedTypeObject /* super::terrain_base::TerrainDynamicDecalTemplateData */> {
        &self.dynamic_decal_template
    }
    fn dynamic_decal_template_mut(&mut self) -> &mut Option<LockedTypeObject /* super::terrain_base::TerrainDynamicDecalTemplateData */> {
        &mut self.dynamic_decal_template
    }
}

impl super::entity::PhysicsPropertyRelationPropertyDataTrait for MaterialRelationTerrainDestructionData {
}

impl super::entity::MaterialRelationPropertyDataTrait for MaterialRelationTerrainDestructionData {
}

impl super::core::DataContainerTrait for MaterialRelationTerrainDestructionData {
}

pub static MATERIALRELATIONTERRAINDESTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialRelationTerrainDestructionData",
    name_hash: 1492736081,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::PHYSICSPROPERTYRELATIONPROPERTYDATA_TYPE_INFO),
        super_class_offset: offset_of!(MaterialRelationTerrainDestructionData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MaterialRelationTerrainDestructionData as Default>::default())),
            create_boxed: || Box::new(<MaterialRelationTerrainDestructionData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "DynamicDecalTemplate",
                name_hash: 1291859343,
                flags: MemberInfoFlags::new(0),
                field_type: "TerrainDynamicDecalTemplateData",
                rust_offset: offset_of!(MaterialRelationTerrainDestructionData, dynamic_decal_template),
            },
        ],
    }),
    array_type: Some(MATERIALRELATIONTERRAINDESTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MaterialRelationTerrainDestructionData {
    fn type_info(&self) -> &'static TypeInfo {
        MATERIALRELATIONTERRAINDESTRUCTIONDATA_TYPE_INFO
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


pub static MATERIALRELATIONTERRAINDESTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialRelationTerrainDestructionData-Array",
    name_hash: 2590165605,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("MaterialRelationTerrainDestructionData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MaterialPropertyTerrainData {
    pub _glacier_base: super::entity::PhysicsMaterialRelationPropertyData,
    pub dirt_trigger_color: super::core::Vec3,
    pub destruction_effect: Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>,
    pub dirt_trigger_factor: f32,
    pub mask_material: super::entity::MaterialDecl,
}

pub trait MaterialPropertyTerrainDataTrait: super::entity::PhysicsMaterialRelationPropertyDataTrait {
    fn dirt_trigger_color(&self) -> &super::core::Vec3;
    fn dirt_trigger_color_mut(&mut self) -> &mut super::core::Vec3;
    fn destruction_effect(&self) -> &Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>;
    fn destruction_effect_mut(&mut self) -> &mut Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>;
    fn dirt_trigger_factor(&self) -> &f32;
    fn dirt_trigger_factor_mut(&mut self) -> &mut f32;
    fn mask_material(&self) -> &super::entity::MaterialDecl;
    fn mask_material_mut(&mut self) -> &mut super::entity::MaterialDecl;
}

impl MaterialPropertyTerrainDataTrait for MaterialPropertyTerrainData {
    fn dirt_trigger_color(&self) -> &super::core::Vec3 {
        &self.dirt_trigger_color
    }
    fn dirt_trigger_color_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.dirt_trigger_color
    }
    fn destruction_effect(&self) -> &Option<LockedTypeObject /* super::effect_base::EffectBlueprint */> {
        &self.destruction_effect
    }
    fn destruction_effect_mut(&mut self) -> &mut Option<LockedTypeObject /* super::effect_base::EffectBlueprint */> {
        &mut self.destruction_effect
    }
    fn dirt_trigger_factor(&self) -> &f32 {
        &self.dirt_trigger_factor
    }
    fn dirt_trigger_factor_mut(&mut self) -> &mut f32 {
        &mut self.dirt_trigger_factor
    }
    fn mask_material(&self) -> &super::entity::MaterialDecl {
        &self.mask_material
    }
    fn mask_material_mut(&mut self) -> &mut super::entity::MaterialDecl {
        &mut self.mask_material
    }
}

impl super::entity::PhysicsMaterialRelationPropertyDataTrait for MaterialPropertyTerrainData {
}

impl super::entity::MaterialRelationPropertyDataTrait for MaterialPropertyTerrainData {
}

impl super::core::DataContainerTrait for MaterialPropertyTerrainData {
}

pub static MATERIALPROPERTYTERRAINDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialPropertyTerrainData",
    name_hash: 391891534,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::PHYSICSMATERIALRELATIONPROPERTYDATA_TYPE_INFO),
        super_class_offset: offset_of!(MaterialPropertyTerrainData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MaterialPropertyTerrainData as Default>::default())),
            create_boxed: || Box::new(<MaterialPropertyTerrainData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "DirtTriggerColor",
                name_hash: 2764380651,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(MaterialPropertyTerrainData, dirt_trigger_color),
            },
            FieldInfoData {
                name: "DestructionEffect",
                name_hash: 3907820780,
                flags: MemberInfoFlags::new(0),
                field_type: "EffectBlueprint",
                rust_offset: offset_of!(MaterialPropertyTerrainData, destruction_effect),
            },
            FieldInfoData {
                name: "DirtTriggerFactor",
                name_hash: 602292507,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MaterialPropertyTerrainData, dirt_trigger_factor),
            },
            FieldInfoData {
                name: "MaskMaterial",
                name_hash: 1895627930,
                flags: MemberInfoFlags::new(0),
                field_type: "MaterialDecl",
                rust_offset: offset_of!(MaterialPropertyTerrainData, mask_material),
            },
        ],
    }),
    array_type: Some(MATERIALPROPERTYTERRAINDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for MaterialPropertyTerrainData {
    fn type_info(&self) -> &'static TypeInfo {
        MATERIALPROPERTYTERRAINDATA_TYPE_INFO
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


pub static MATERIALPROPERTYTERRAINDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialPropertyTerrainData-Array",
    name_hash: 225747962,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("MaterialPropertyTerrainData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MaterialRelationSoundData {
    pub _glacier_base: super::entity::PhysicsMaterialRelationPropertyData,
    pub impact_sound: Option<LockedTypeObject /* super::audio::SoundAsset */>,
    pub impact_sound_event: Option<LockedTypeObject /* super::audio::AudioGraphEvent */>,
    pub scrape_sound: Option<LockedTypeObject /* super::audio::SoundAsset */>,
    pub scrape_length: f32,
    pub scrape_impact_sound_event: Option<LockedTypeObject /* super::audio::AudioGraphEvent */>,
}

pub trait MaterialRelationSoundDataTrait: super::entity::PhysicsMaterialRelationPropertyDataTrait {
    fn impact_sound(&self) -> &Option<LockedTypeObject /* super::audio::SoundAsset */>;
    fn impact_sound_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundAsset */>;
    fn impact_sound_event(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphEvent */>;
    fn impact_sound_event_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphEvent */>;
    fn scrape_sound(&self) -> &Option<LockedTypeObject /* super::audio::SoundAsset */>;
    fn scrape_sound_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundAsset */>;
    fn scrape_length(&self) -> &f32;
    fn scrape_length_mut(&mut self) -> &mut f32;
    fn scrape_impact_sound_event(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphEvent */>;
    fn scrape_impact_sound_event_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphEvent */>;
}

impl MaterialRelationSoundDataTrait for MaterialRelationSoundData {
    fn impact_sound(&self) -> &Option<LockedTypeObject /* super::audio::SoundAsset */> {
        &self.impact_sound
    }
    fn impact_sound_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundAsset */> {
        &mut self.impact_sound
    }
    fn impact_sound_event(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphEvent */> {
        &self.impact_sound_event
    }
    fn impact_sound_event_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphEvent */> {
        &mut self.impact_sound_event
    }
    fn scrape_sound(&self) -> &Option<LockedTypeObject /* super::audio::SoundAsset */> {
        &self.scrape_sound
    }
    fn scrape_sound_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundAsset */> {
        &mut self.scrape_sound
    }
    fn scrape_length(&self) -> &f32 {
        &self.scrape_length
    }
    fn scrape_length_mut(&mut self) -> &mut f32 {
        &mut self.scrape_length
    }
    fn scrape_impact_sound_event(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphEvent */> {
        &self.scrape_impact_sound_event
    }
    fn scrape_impact_sound_event_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphEvent */> {
        &mut self.scrape_impact_sound_event
    }
}

impl super::entity::PhysicsMaterialRelationPropertyDataTrait for MaterialRelationSoundData {
}

impl super::entity::MaterialRelationPropertyDataTrait for MaterialRelationSoundData {
}

impl super::core::DataContainerTrait for MaterialRelationSoundData {
}

pub static MATERIALRELATIONSOUNDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialRelationSoundData",
    name_hash: 3123527995,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::PHYSICSMATERIALRELATIONPROPERTYDATA_TYPE_INFO),
        super_class_offset: offset_of!(MaterialRelationSoundData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MaterialRelationSoundData as Default>::default())),
            create_boxed: || Box::new(<MaterialRelationSoundData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ImpactSound",
                name_hash: 3986717348,
                flags: MemberInfoFlags::new(0),
                field_type: "SoundAsset",
                rust_offset: offset_of!(MaterialRelationSoundData, impact_sound),
            },
            FieldInfoData {
                name: "ImpactSoundEvent",
                name_hash: 2317001832,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphEvent",
                rust_offset: offset_of!(MaterialRelationSoundData, impact_sound_event),
            },
            FieldInfoData {
                name: "ScrapeSound",
                name_hash: 1279692944,
                flags: MemberInfoFlags::new(0),
                field_type: "SoundAsset",
                rust_offset: offset_of!(MaterialRelationSoundData, scrape_sound),
            },
            FieldInfoData {
                name: "ScrapeLength",
                name_hash: 544833071,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MaterialRelationSoundData, scrape_length),
            },
            FieldInfoData {
                name: "ScrapeImpactSoundEvent",
                name_hash: 479233278,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphEvent",
                rust_offset: offset_of!(MaterialRelationSoundData, scrape_impact_sound_event),
            },
        ],
    }),
    array_type: Some(MATERIALRELATIONSOUNDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MaterialRelationSoundData {
    fn type_info(&self) -> &'static TypeInfo {
        MATERIALRELATIONSOUNDDATA_TYPE_INFO
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


pub static MATERIALRELATIONSOUNDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialRelationSoundData-Array",
    name_hash: 1773791375,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("MaterialRelationSoundData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MaterialPropertySoundData {
    pub _glacier_base: super::entity::PhysicsMaterialRelationPropertyData,
    pub impact_sound: Option<LockedTypeObject /* super::audio::SoundAsset */>,
    pub scrape_sound: Option<LockedTypeObject /* super::audio::SoundAsset */>,
    pub scrape_length: f32,
    pub licensee_sound_data: Option<LockedTypeObject /* MaterialPropertyLicenseeSoundData */>,
    pub softness: f32,
    pub material_sound_id: f32,
}

pub trait MaterialPropertySoundDataTrait: super::entity::PhysicsMaterialRelationPropertyDataTrait {
    fn impact_sound(&self) -> &Option<LockedTypeObject /* super::audio::SoundAsset */>;
    fn impact_sound_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundAsset */>;
    fn scrape_sound(&self) -> &Option<LockedTypeObject /* super::audio::SoundAsset */>;
    fn scrape_sound_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundAsset */>;
    fn scrape_length(&self) -> &f32;
    fn scrape_length_mut(&mut self) -> &mut f32;
    fn licensee_sound_data(&self) -> &Option<LockedTypeObject /* MaterialPropertyLicenseeSoundData */>;
    fn licensee_sound_data_mut(&mut self) -> &mut Option<LockedTypeObject /* MaterialPropertyLicenseeSoundData */>;
    fn softness(&self) -> &f32;
    fn softness_mut(&mut self) -> &mut f32;
    fn material_sound_id(&self) -> &f32;
    fn material_sound_id_mut(&mut self) -> &mut f32;
}

impl MaterialPropertySoundDataTrait for MaterialPropertySoundData {
    fn impact_sound(&self) -> &Option<LockedTypeObject /* super::audio::SoundAsset */> {
        &self.impact_sound
    }
    fn impact_sound_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundAsset */> {
        &mut self.impact_sound
    }
    fn scrape_sound(&self) -> &Option<LockedTypeObject /* super::audio::SoundAsset */> {
        &self.scrape_sound
    }
    fn scrape_sound_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundAsset */> {
        &mut self.scrape_sound
    }
    fn scrape_length(&self) -> &f32 {
        &self.scrape_length
    }
    fn scrape_length_mut(&mut self) -> &mut f32 {
        &mut self.scrape_length
    }
    fn licensee_sound_data(&self) -> &Option<LockedTypeObject /* MaterialPropertyLicenseeSoundData */> {
        &self.licensee_sound_data
    }
    fn licensee_sound_data_mut(&mut self) -> &mut Option<LockedTypeObject /* MaterialPropertyLicenseeSoundData */> {
        &mut self.licensee_sound_data
    }
    fn softness(&self) -> &f32 {
        &self.softness
    }
    fn softness_mut(&mut self) -> &mut f32 {
        &mut self.softness
    }
    fn material_sound_id(&self) -> &f32 {
        &self.material_sound_id
    }
    fn material_sound_id_mut(&mut self) -> &mut f32 {
        &mut self.material_sound_id
    }
}

impl super::entity::PhysicsMaterialRelationPropertyDataTrait for MaterialPropertySoundData {
}

impl super::entity::MaterialRelationPropertyDataTrait for MaterialPropertySoundData {
}

impl super::core::DataContainerTrait for MaterialPropertySoundData {
}

pub static MATERIALPROPERTYSOUNDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialPropertySoundData",
    name_hash: 4241842810,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::PHYSICSMATERIALRELATIONPROPERTYDATA_TYPE_INFO),
        super_class_offset: offset_of!(MaterialPropertySoundData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MaterialPropertySoundData as Default>::default())),
            create_boxed: || Box::new(<MaterialPropertySoundData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ImpactSound",
                name_hash: 3986717348,
                flags: MemberInfoFlags::new(0),
                field_type: "SoundAsset",
                rust_offset: offset_of!(MaterialPropertySoundData, impact_sound),
            },
            FieldInfoData {
                name: "ScrapeSound",
                name_hash: 1279692944,
                flags: MemberInfoFlags::new(0),
                field_type: "SoundAsset",
                rust_offset: offset_of!(MaterialPropertySoundData, scrape_sound),
            },
            FieldInfoData {
                name: "ScrapeLength",
                name_hash: 544833071,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MaterialPropertySoundData, scrape_length),
            },
            FieldInfoData {
                name: "LicenseeSoundData",
                name_hash: 3289552456,
                flags: MemberInfoFlags::new(0),
                field_type: "MaterialPropertyLicenseeSoundData",
                rust_offset: offset_of!(MaterialPropertySoundData, licensee_sound_data),
            },
            FieldInfoData {
                name: "Softness",
                name_hash: 3898807136,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MaterialPropertySoundData, softness),
            },
            FieldInfoData {
                name: "MaterialSoundId",
                name_hash: 2481569024,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MaterialPropertySoundData, material_sound_id),
            },
        ],
    }),
    array_type: Some(MATERIALPROPERTYSOUNDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MaterialPropertySoundData {
    fn type_info(&self) -> &'static TypeInfo {
        MATERIALPROPERTYSOUNDDATA_TYPE_INFO
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


pub static MATERIALPROPERTYSOUNDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialPropertySoundData-Array",
    name_hash: 2896065870,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("MaterialPropertySoundData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MaterialPropertyLicenseeSoundData {
    pub _glacier_base: super::core::DataContainer,
}

pub trait MaterialPropertyLicenseeSoundDataTrait: super::core::DataContainerTrait {
}

impl MaterialPropertyLicenseeSoundDataTrait for MaterialPropertyLicenseeSoundData {
}

impl super::core::DataContainerTrait for MaterialPropertyLicenseeSoundData {
}

pub static MATERIALPROPERTYLICENSEESOUNDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialPropertyLicenseeSoundData",
    name_hash: 497118660,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(MaterialPropertyLicenseeSoundData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MaterialPropertyLicenseeSoundData as Default>::default())),
            create_boxed: || Box::new(<MaterialPropertyLicenseeSoundData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(MATERIALPROPERTYLICENSEESOUNDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MaterialPropertyLicenseeSoundData {
    fn type_info(&self) -> &'static TypeInfo {
        MATERIALPROPERTYLICENSEESOUNDDATA_TYPE_INFO
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


pub static MATERIALPROPERTYLICENSEESOUNDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialPropertyLicenseeSoundData-Array",
    name_hash: 3901690480,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("MaterialPropertyLicenseeSoundData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MaterialPropertyFireData {
    pub _glacier_base: super::entity::PhysicsMaterialRelationPropertyData,
    pub is_burnable: bool,
}

pub trait MaterialPropertyFireDataTrait: super::entity::PhysicsMaterialRelationPropertyDataTrait {
    fn is_burnable(&self) -> &bool;
    fn is_burnable_mut(&mut self) -> &mut bool;
}

impl MaterialPropertyFireDataTrait for MaterialPropertyFireData {
    fn is_burnable(&self) -> &bool {
        &self.is_burnable
    }
    fn is_burnable_mut(&mut self) -> &mut bool {
        &mut self.is_burnable
    }
}

impl super::entity::PhysicsMaterialRelationPropertyDataTrait for MaterialPropertyFireData {
}

impl super::entity::MaterialRelationPropertyDataTrait for MaterialPropertyFireData {
}

impl super::core::DataContainerTrait for MaterialPropertyFireData {
}

pub static MATERIALPROPERTYFIREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialPropertyFireData",
    name_hash: 1770773889,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::PHYSICSMATERIALRELATIONPROPERTYDATA_TYPE_INFO),
        super_class_offset: offset_of!(MaterialPropertyFireData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MaterialPropertyFireData as Default>::default())),
            create_boxed: || Box::new(<MaterialPropertyFireData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "IsBurnable",
                name_hash: 3979458142,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MaterialPropertyFireData, is_burnable),
            },
        ],
    }),
    array_type: Some(MATERIALPROPERTYFIREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MaterialPropertyFireData {
    fn type_info(&self) -> &'static TypeInfo {
        MATERIALPROPERTYFIREDATA_TYPE_INFO
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


pub static MATERIALPROPERTYFIREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialPropertyFireData-Array",
    name_hash: 3385161525,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("MaterialPropertyFireData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MaterialRelationVehicleData {
    pub _glacier_base: super::entity::PhysicsMaterialRelationPropertyData,
    pub chassi_effect: Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>,
    pub track_effect: Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>,
    pub wheel_effect: Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>,
    pub ground_effect: Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>,
}

pub trait MaterialRelationVehicleDataTrait: super::entity::PhysicsMaterialRelationPropertyDataTrait {
    fn chassi_effect(&self) -> &Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>;
    fn chassi_effect_mut(&mut self) -> &mut Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>;
    fn track_effect(&self) -> &Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>;
    fn track_effect_mut(&mut self) -> &mut Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>;
    fn wheel_effect(&self) -> &Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>;
    fn wheel_effect_mut(&mut self) -> &mut Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>;
    fn ground_effect(&self) -> &Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>;
    fn ground_effect_mut(&mut self) -> &mut Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>;
}

impl MaterialRelationVehicleDataTrait for MaterialRelationVehicleData {
    fn chassi_effect(&self) -> &Option<LockedTypeObject /* super::effect_base::EffectBlueprint */> {
        &self.chassi_effect
    }
    fn chassi_effect_mut(&mut self) -> &mut Option<LockedTypeObject /* super::effect_base::EffectBlueprint */> {
        &mut self.chassi_effect
    }
    fn track_effect(&self) -> &Option<LockedTypeObject /* super::effect_base::EffectBlueprint */> {
        &self.track_effect
    }
    fn track_effect_mut(&mut self) -> &mut Option<LockedTypeObject /* super::effect_base::EffectBlueprint */> {
        &mut self.track_effect
    }
    fn wheel_effect(&self) -> &Option<LockedTypeObject /* super::effect_base::EffectBlueprint */> {
        &self.wheel_effect
    }
    fn wheel_effect_mut(&mut self) -> &mut Option<LockedTypeObject /* super::effect_base::EffectBlueprint */> {
        &mut self.wheel_effect
    }
    fn ground_effect(&self) -> &Option<LockedTypeObject /* super::effect_base::EffectBlueprint */> {
        &self.ground_effect
    }
    fn ground_effect_mut(&mut self) -> &mut Option<LockedTypeObject /* super::effect_base::EffectBlueprint */> {
        &mut self.ground_effect
    }
}

impl super::entity::PhysicsMaterialRelationPropertyDataTrait for MaterialRelationVehicleData {
}

impl super::entity::MaterialRelationPropertyDataTrait for MaterialRelationVehicleData {
}

impl super::core::DataContainerTrait for MaterialRelationVehicleData {
}

pub static MATERIALRELATIONVEHICLEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialRelationVehicleData",
    name_hash: 3007876352,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::PHYSICSMATERIALRELATIONPROPERTYDATA_TYPE_INFO),
        super_class_offset: offset_of!(MaterialRelationVehicleData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MaterialRelationVehicleData as Default>::default())),
            create_boxed: || Box::new(<MaterialRelationVehicleData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ChassiEffect",
                name_hash: 2206681905,
                flags: MemberInfoFlags::new(0),
                field_type: "EffectBlueprint",
                rust_offset: offset_of!(MaterialRelationVehicleData, chassi_effect),
            },
            FieldInfoData {
                name: "TrackEffect",
                name_hash: 1006696637,
                flags: MemberInfoFlags::new(0),
                field_type: "EffectBlueprint",
                rust_offset: offset_of!(MaterialRelationVehicleData, track_effect),
            },
            FieldInfoData {
                name: "WheelEffect",
                name_hash: 3303703137,
                flags: MemberInfoFlags::new(0),
                field_type: "EffectBlueprint",
                rust_offset: offset_of!(MaterialRelationVehicleData, wheel_effect),
            },
            FieldInfoData {
                name: "GroundEffect",
                name_hash: 2819688663,
                flags: MemberInfoFlags::new(0),
                field_type: "EffectBlueprint",
                rust_offset: offset_of!(MaterialRelationVehicleData, ground_effect),
            },
        ],
    }),
    array_type: Some(MATERIALRELATIONVEHICLEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MaterialRelationVehicleData {
    fn type_info(&self) -> &'static TypeInfo {
        MATERIALRELATIONVEHICLEDATA_TYPE_INFO
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


pub static MATERIALRELATIONVEHICLEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialRelationVehicleData-Array",
    name_hash: 1096610612,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("MaterialRelationVehicleData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MaterialRelationPenetrationData {
    pub _glacier_base: super::entity::PhysicsPropertyRelationPropertyData,
    pub never_penetrate: bool,
}

pub trait MaterialRelationPenetrationDataTrait: super::entity::PhysicsPropertyRelationPropertyDataTrait {
    fn never_penetrate(&self) -> &bool;
    fn never_penetrate_mut(&mut self) -> &mut bool;
}

impl MaterialRelationPenetrationDataTrait for MaterialRelationPenetrationData {
    fn never_penetrate(&self) -> &bool {
        &self.never_penetrate
    }
    fn never_penetrate_mut(&mut self) -> &mut bool {
        &mut self.never_penetrate
    }
}

impl super::entity::PhysicsPropertyRelationPropertyDataTrait for MaterialRelationPenetrationData {
}

impl super::entity::MaterialRelationPropertyDataTrait for MaterialRelationPenetrationData {
}

impl super::core::DataContainerTrait for MaterialRelationPenetrationData {
}

pub static MATERIALRELATIONPENETRATIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialRelationPenetrationData",
    name_hash: 3920586493,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::PHYSICSPROPERTYRELATIONPROPERTYDATA_TYPE_INFO),
        super_class_offset: offset_of!(MaterialRelationPenetrationData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MaterialRelationPenetrationData as Default>::default())),
            create_boxed: || Box::new(<MaterialRelationPenetrationData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "NeverPenetrate",
                name_hash: 2064798663,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MaterialRelationPenetrationData, never_penetrate),
            },
        ],
    }),
    array_type: Some(MATERIALRELATIONPENETRATIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MaterialRelationPenetrationData {
    fn type_info(&self) -> &'static TypeInfo {
        MATERIALRELATIONPENETRATIONDATA_TYPE_INFO
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


pub static MATERIALRELATIONPENETRATIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialRelationPenetrationData-Array",
    name_hash: 696832969,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("MaterialRelationPenetrationData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MaterialRelationDamageData {
    pub _glacier_base: super::entity::PhysicsPropertyRelationPropertyData,
    pub collision_damage_multiplier: f32,
    pub collision_damage_threshold: f32,
    pub damage_protection_multiplier: f32,
    pub damage_penetration_multiplier: f32,
    pub damage_protection_threshold: f32,
    pub explosion_cover_damage_modifier: f32,
    pub inflicts_demolition_damage: bool,
    pub allow_client_destruction: bool,
}

pub trait MaterialRelationDamageDataTrait: super::entity::PhysicsPropertyRelationPropertyDataTrait {
    fn collision_damage_multiplier(&self) -> &f32;
    fn collision_damage_multiplier_mut(&mut self) -> &mut f32;
    fn collision_damage_threshold(&self) -> &f32;
    fn collision_damage_threshold_mut(&mut self) -> &mut f32;
    fn damage_protection_multiplier(&self) -> &f32;
    fn damage_protection_multiplier_mut(&mut self) -> &mut f32;
    fn damage_penetration_multiplier(&self) -> &f32;
    fn damage_penetration_multiplier_mut(&mut self) -> &mut f32;
    fn damage_protection_threshold(&self) -> &f32;
    fn damage_protection_threshold_mut(&mut self) -> &mut f32;
    fn explosion_cover_damage_modifier(&self) -> &f32;
    fn explosion_cover_damage_modifier_mut(&mut self) -> &mut f32;
    fn inflicts_demolition_damage(&self) -> &bool;
    fn inflicts_demolition_damage_mut(&mut self) -> &mut bool;
    fn allow_client_destruction(&self) -> &bool;
    fn allow_client_destruction_mut(&mut self) -> &mut bool;
}

impl MaterialRelationDamageDataTrait for MaterialRelationDamageData {
    fn collision_damage_multiplier(&self) -> &f32 {
        &self.collision_damage_multiplier
    }
    fn collision_damage_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.collision_damage_multiplier
    }
    fn collision_damage_threshold(&self) -> &f32 {
        &self.collision_damage_threshold
    }
    fn collision_damage_threshold_mut(&mut self) -> &mut f32 {
        &mut self.collision_damage_threshold
    }
    fn damage_protection_multiplier(&self) -> &f32 {
        &self.damage_protection_multiplier
    }
    fn damage_protection_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.damage_protection_multiplier
    }
    fn damage_penetration_multiplier(&self) -> &f32 {
        &self.damage_penetration_multiplier
    }
    fn damage_penetration_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.damage_penetration_multiplier
    }
    fn damage_protection_threshold(&self) -> &f32 {
        &self.damage_protection_threshold
    }
    fn damage_protection_threshold_mut(&mut self) -> &mut f32 {
        &mut self.damage_protection_threshold
    }
    fn explosion_cover_damage_modifier(&self) -> &f32 {
        &self.explosion_cover_damage_modifier
    }
    fn explosion_cover_damage_modifier_mut(&mut self) -> &mut f32 {
        &mut self.explosion_cover_damage_modifier
    }
    fn inflicts_demolition_damage(&self) -> &bool {
        &self.inflicts_demolition_damage
    }
    fn inflicts_demolition_damage_mut(&mut self) -> &mut bool {
        &mut self.inflicts_demolition_damage
    }
    fn allow_client_destruction(&self) -> &bool {
        &self.allow_client_destruction
    }
    fn allow_client_destruction_mut(&mut self) -> &mut bool {
        &mut self.allow_client_destruction
    }
}

impl super::entity::PhysicsPropertyRelationPropertyDataTrait for MaterialRelationDamageData {
}

impl super::entity::MaterialRelationPropertyDataTrait for MaterialRelationDamageData {
}

impl super::core::DataContainerTrait for MaterialRelationDamageData {
}

pub static MATERIALRELATIONDAMAGEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialRelationDamageData",
    name_hash: 3475133875,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::PHYSICSPROPERTYRELATIONPROPERTYDATA_TYPE_INFO),
        super_class_offset: offset_of!(MaterialRelationDamageData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MaterialRelationDamageData as Default>::default())),
            create_boxed: || Box::new(<MaterialRelationDamageData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "CollisionDamageMultiplier",
                name_hash: 31287099,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MaterialRelationDamageData, collision_damage_multiplier),
            },
            FieldInfoData {
                name: "CollisionDamageThreshold",
                name_hash: 2223249863,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MaterialRelationDamageData, collision_damage_threshold),
            },
            FieldInfoData {
                name: "DamageProtectionMultiplier",
                name_hash: 3194303654,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MaterialRelationDamageData, damage_protection_multiplier),
            },
            FieldInfoData {
                name: "DamagePenetrationMultiplier",
                name_hash: 2424339200,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MaterialRelationDamageData, damage_penetration_multiplier),
            },
            FieldInfoData {
                name: "DamageProtectionThreshold",
                name_hash: 1413485242,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MaterialRelationDamageData, damage_protection_threshold),
            },
            FieldInfoData {
                name: "ExplosionCoverDamageModifier",
                name_hash: 3323347809,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MaterialRelationDamageData, explosion_cover_damage_modifier),
            },
            FieldInfoData {
                name: "InflictsDemolitionDamage",
                name_hash: 3423431988,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MaterialRelationDamageData, inflicts_demolition_damage),
            },
            FieldInfoData {
                name: "AllowClientDestruction",
                name_hash: 1353422203,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MaterialRelationDamageData, allow_client_destruction),
            },
        ],
    }),
    array_type: Some(MATERIALRELATIONDAMAGEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MaterialRelationDamageData {
    fn type_info(&self) -> &'static TypeInfo {
        MATERIALRELATIONDAMAGEDATA_TYPE_INFO
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


pub static MATERIALRELATIONDAMAGEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialRelationDamageData-Array",
    name_hash: 1775042823,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("MaterialRelationDamageData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MaterialRelationDynamicFireData {
    pub _glacier_base: super::entity::PhysicsMaterialRelationPropertyData,
    pub can_set_fire: bool,
    pub fire_damage: f32,
    pub cell_damage_radius: u32,
}

pub trait MaterialRelationDynamicFireDataTrait: super::entity::PhysicsMaterialRelationPropertyDataTrait {
    fn can_set_fire(&self) -> &bool;
    fn can_set_fire_mut(&mut self) -> &mut bool;
    fn fire_damage(&self) -> &f32;
    fn fire_damage_mut(&mut self) -> &mut f32;
    fn cell_damage_radius(&self) -> &u32;
    fn cell_damage_radius_mut(&mut self) -> &mut u32;
}

impl MaterialRelationDynamicFireDataTrait for MaterialRelationDynamicFireData {
    fn can_set_fire(&self) -> &bool {
        &self.can_set_fire
    }
    fn can_set_fire_mut(&mut self) -> &mut bool {
        &mut self.can_set_fire
    }
    fn fire_damage(&self) -> &f32 {
        &self.fire_damage
    }
    fn fire_damage_mut(&mut self) -> &mut f32 {
        &mut self.fire_damage
    }
    fn cell_damage_radius(&self) -> &u32 {
        &self.cell_damage_radius
    }
    fn cell_damage_radius_mut(&mut self) -> &mut u32 {
        &mut self.cell_damage_radius
    }
}

impl super::entity::PhysicsMaterialRelationPropertyDataTrait for MaterialRelationDynamicFireData {
}

impl super::entity::MaterialRelationPropertyDataTrait for MaterialRelationDynamicFireData {
}

impl super::core::DataContainerTrait for MaterialRelationDynamicFireData {
}

pub static MATERIALRELATIONDYNAMICFIREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialRelationDynamicFireData",
    name_hash: 4218673877,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::PHYSICSMATERIALRELATIONPROPERTYDATA_TYPE_INFO),
        super_class_offset: offset_of!(MaterialRelationDynamicFireData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MaterialRelationDynamicFireData as Default>::default())),
            create_boxed: || Box::new(<MaterialRelationDynamicFireData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "CanSetFire",
                name_hash: 2727949683,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MaterialRelationDynamicFireData, can_set_fire),
            },
            FieldInfoData {
                name: "FireDamage",
                name_hash: 117999510,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MaterialRelationDynamicFireData, fire_damage),
            },
            FieldInfoData {
                name: "CellDamageRadius",
                name_hash: 3551636016,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(MaterialRelationDynamicFireData, cell_damage_radius),
            },
        ],
    }),
    array_type: Some(MATERIALRELATIONDYNAMICFIREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MaterialRelationDynamicFireData {
    fn type_info(&self) -> &'static TypeInfo {
        MATERIALRELATIONDYNAMICFIREDATA_TYPE_INFO
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


pub static MATERIALRELATIONDYNAMICFIREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialRelationDynamicFireData-Array",
    name_hash: 1637145313,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("MaterialRelationDynamicFireData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MaterialRelationEffectData {
    pub _glacier_base: super::entity::PhysicsMaterialRelationPropertyData,
    pub impact_effects: Vec<BoxedTypeObject /* EffectWithSpeedRange */>,
    pub impact_debris: Vec<Option<LockedTypeObject /* super::entity::ObjectBlueprint */>>,
    pub impact_effect_max_spread_angle: f32,
    pub mirror_impact_direction: bool,
    pub enable_inherited_velocity: bool,
}

pub trait MaterialRelationEffectDataTrait: super::entity::PhysicsMaterialRelationPropertyDataTrait {
    fn impact_effects(&self) -> &Vec<BoxedTypeObject /* EffectWithSpeedRange */>;
    fn impact_effects_mut(&mut self) -> &mut Vec<BoxedTypeObject /* EffectWithSpeedRange */>;
    fn impact_debris(&self) -> &Vec<Option<LockedTypeObject /* super::entity::ObjectBlueprint */>>;
    fn impact_debris_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::ObjectBlueprint */>>;
    fn impact_effect_max_spread_angle(&self) -> &f32;
    fn impact_effect_max_spread_angle_mut(&mut self) -> &mut f32;
    fn mirror_impact_direction(&self) -> &bool;
    fn mirror_impact_direction_mut(&mut self) -> &mut bool;
    fn enable_inherited_velocity(&self) -> &bool;
    fn enable_inherited_velocity_mut(&mut self) -> &mut bool;
}

impl MaterialRelationEffectDataTrait for MaterialRelationEffectData {
    fn impact_effects(&self) -> &Vec<BoxedTypeObject /* EffectWithSpeedRange */> {
        &self.impact_effects
    }
    fn impact_effects_mut(&mut self) -> &mut Vec<BoxedTypeObject /* EffectWithSpeedRange */> {
        &mut self.impact_effects
    }
    fn impact_debris(&self) -> &Vec<Option<LockedTypeObject /* super::entity::ObjectBlueprint */>> {
        &self.impact_debris
    }
    fn impact_debris_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::ObjectBlueprint */>> {
        &mut self.impact_debris
    }
    fn impact_effect_max_spread_angle(&self) -> &f32 {
        &self.impact_effect_max_spread_angle
    }
    fn impact_effect_max_spread_angle_mut(&mut self) -> &mut f32 {
        &mut self.impact_effect_max_spread_angle
    }
    fn mirror_impact_direction(&self) -> &bool {
        &self.mirror_impact_direction
    }
    fn mirror_impact_direction_mut(&mut self) -> &mut bool {
        &mut self.mirror_impact_direction
    }
    fn enable_inherited_velocity(&self) -> &bool {
        &self.enable_inherited_velocity
    }
    fn enable_inherited_velocity_mut(&mut self) -> &mut bool {
        &mut self.enable_inherited_velocity
    }
}

impl super::entity::PhysicsMaterialRelationPropertyDataTrait for MaterialRelationEffectData {
}

impl super::entity::MaterialRelationPropertyDataTrait for MaterialRelationEffectData {
}

impl super::core::DataContainerTrait for MaterialRelationEffectData {
}

pub static MATERIALRELATIONEFFECTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialRelationEffectData",
    name_hash: 3317419247,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::PHYSICSMATERIALRELATIONPROPERTYDATA_TYPE_INFO),
        super_class_offset: offset_of!(MaterialRelationEffectData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MaterialRelationEffectData as Default>::default())),
            create_boxed: || Box::new(<MaterialRelationEffectData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ImpactEffects",
                name_hash: 2149182307,
                flags: MemberInfoFlags::new(144),
                field_type: "EffectWithSpeedRange-Array",
                rust_offset: offset_of!(MaterialRelationEffectData, impact_effects),
            },
            FieldInfoData {
                name: "ImpactDebris",
                name_hash: 2118798188,
                flags: MemberInfoFlags::new(144),
                field_type: "ObjectBlueprint-Array",
                rust_offset: offset_of!(MaterialRelationEffectData, impact_debris),
            },
            FieldInfoData {
                name: "ImpactEffectMaxSpreadAngle",
                name_hash: 3809186484,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MaterialRelationEffectData, impact_effect_max_spread_angle),
            },
            FieldInfoData {
                name: "MirrorImpactDirection",
                name_hash: 559662331,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MaterialRelationEffectData, mirror_impact_direction),
            },
            FieldInfoData {
                name: "EnableInheritedVelocity",
                name_hash: 1456018071,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MaterialRelationEffectData, enable_inherited_velocity),
            },
        ],
    }),
    array_type: Some(MATERIALRELATIONEFFECTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MaterialRelationEffectData {
    fn type_info(&self) -> &'static TypeInfo {
        MATERIALRELATIONEFFECTDATA_TYPE_INFO
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


pub static MATERIALRELATIONEFFECTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialRelationEffectData-Array",
    name_hash: 1558391515,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("MaterialRelationEffectData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MaterialPropertyImpulseData {
    pub _glacier_base: super::entity::PhysicsPropertyRelationPropertyData,
    pub impulse_absorption_multiplier: f32,
}

pub trait MaterialPropertyImpulseDataTrait: super::entity::PhysicsPropertyRelationPropertyDataTrait {
    fn impulse_absorption_multiplier(&self) -> &f32;
    fn impulse_absorption_multiplier_mut(&mut self) -> &mut f32;
}

impl MaterialPropertyImpulseDataTrait for MaterialPropertyImpulseData {
    fn impulse_absorption_multiplier(&self) -> &f32 {
        &self.impulse_absorption_multiplier
    }
    fn impulse_absorption_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.impulse_absorption_multiplier
    }
}

impl super::entity::PhysicsPropertyRelationPropertyDataTrait for MaterialPropertyImpulseData {
}

impl super::entity::MaterialRelationPropertyDataTrait for MaterialPropertyImpulseData {
}

impl super::core::DataContainerTrait for MaterialPropertyImpulseData {
}

pub static MATERIALPROPERTYIMPULSEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialPropertyImpulseData",
    name_hash: 3017044514,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::PHYSICSPROPERTYRELATIONPROPERTYDATA_TYPE_INFO),
        super_class_offset: offset_of!(MaterialPropertyImpulseData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MaterialPropertyImpulseData as Default>::default())),
            create_boxed: || Box::new(<MaterialPropertyImpulseData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ImpulseAbsorptionMultiplier",
                name_hash: 1970095508,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MaterialPropertyImpulseData, impulse_absorption_multiplier),
            },
        ],
    }),
    array_type: Some(MATERIALPROPERTYIMPULSEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MaterialPropertyImpulseData {
    fn type_info(&self) -> &'static TypeInfo {
        MATERIALPROPERTYIMPULSEDATA_TYPE_INFO
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


pub static MATERIALPROPERTYIMPULSEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialPropertyImpulseData-Array",
    name_hash: 3277353110,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("MaterialPropertyImpulseData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MaterialPropertyHealthData {
    pub _glacier_base: super::entity::PhysicsPropertyRelationPropertyData,
    pub health: f32,
}

pub trait MaterialPropertyHealthDataTrait: super::entity::PhysicsPropertyRelationPropertyDataTrait {
    fn health(&self) -> &f32;
    fn health_mut(&mut self) -> &mut f32;
}

impl MaterialPropertyHealthDataTrait for MaterialPropertyHealthData {
    fn health(&self) -> &f32 {
        &self.health
    }
    fn health_mut(&mut self) -> &mut f32 {
        &mut self.health
    }
}

impl super::entity::PhysicsPropertyRelationPropertyDataTrait for MaterialPropertyHealthData {
}

impl super::entity::MaterialRelationPropertyDataTrait for MaterialPropertyHealthData {
}

impl super::core::DataContainerTrait for MaterialPropertyHealthData {
}

pub static MATERIALPROPERTYHEALTHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialPropertyHealthData",
    name_hash: 213685797,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::PHYSICSPROPERTYRELATIONPROPERTYDATA_TYPE_INFO),
        super_class_offset: offset_of!(MaterialPropertyHealthData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MaterialPropertyHealthData as Default>::default())),
            create_boxed: || Box::new(<MaterialPropertyHealthData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Health",
                name_hash: 3054337113,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MaterialPropertyHealthData, health),
            },
        ],
    }),
    array_type: Some(MATERIALPROPERTYHEALTHDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MaterialPropertyHealthData {
    fn type_info(&self) -> &'static TypeInfo {
        MATERIALPROPERTYHEALTHDATA_TYPE_INFO
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


pub static MATERIALPROPERTYHEALTHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialPropertyHealthData-Array",
    name_hash: 3064598161,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("MaterialPropertyHealthData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MaterialRelationDecalData {
    pub _glacier_base: super::entity::PhysicsMaterialRelationPropertyData,
    pub decal: Option<LockedTypeObject /* super::render_base::DecalTemplateBaseAsset */>,
    pub exit_decal: Option<LockedTypeObject /* super::render_base::DecalTemplateBaseAsset */>,
}

pub trait MaterialRelationDecalDataTrait: super::entity::PhysicsMaterialRelationPropertyDataTrait {
    fn decal(&self) -> &Option<LockedTypeObject /* super::render_base::DecalTemplateBaseAsset */>;
    fn decal_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::DecalTemplateBaseAsset */>;
    fn exit_decal(&self) -> &Option<LockedTypeObject /* super::render_base::DecalTemplateBaseAsset */>;
    fn exit_decal_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::DecalTemplateBaseAsset */>;
}

impl MaterialRelationDecalDataTrait for MaterialRelationDecalData {
    fn decal(&self) -> &Option<LockedTypeObject /* super::render_base::DecalTemplateBaseAsset */> {
        &self.decal
    }
    fn decal_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::DecalTemplateBaseAsset */> {
        &mut self.decal
    }
    fn exit_decal(&self) -> &Option<LockedTypeObject /* super::render_base::DecalTemplateBaseAsset */> {
        &self.exit_decal
    }
    fn exit_decal_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::DecalTemplateBaseAsset */> {
        &mut self.exit_decal
    }
}

impl super::entity::PhysicsMaterialRelationPropertyDataTrait for MaterialRelationDecalData {
}

impl super::entity::MaterialRelationPropertyDataTrait for MaterialRelationDecalData {
}

impl super::core::DataContainerTrait for MaterialRelationDecalData {
}

pub static MATERIALRELATIONDECALDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialRelationDecalData",
    name_hash: 3684616599,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::PHYSICSMATERIALRELATIONPROPERTYDATA_TYPE_INFO),
        super_class_offset: offset_of!(MaterialRelationDecalData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MaterialRelationDecalData as Default>::default())),
            create_boxed: || Box::new(<MaterialRelationDecalData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Decal",
                name_hash: 208760746,
                flags: MemberInfoFlags::new(0),
                field_type: "DecalTemplateBaseAsset",
                rust_offset: offset_of!(MaterialRelationDecalData, decal),
            },
            FieldInfoData {
                name: "ExitDecal",
                name_hash: 3812854282,
                flags: MemberInfoFlags::new(0),
                field_type: "DecalTemplateBaseAsset",
                rust_offset: offset_of!(MaterialRelationDecalData, exit_decal),
            },
        ],
    }),
    array_type: Some(MATERIALRELATIONDECALDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MaterialRelationDecalData {
    fn type_info(&self) -> &'static TypeInfo {
        MATERIALRELATIONDECALDATA_TYPE_INFO
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


pub static MATERIALRELATIONDECALDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialRelationDecalData-Array",
    name_hash: 1013207587,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("MaterialRelationDecalData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MaterialPropertyEffectData {
    pub _glacier_base: super::entity::PhysicsMaterialRelationPropertyData,
    pub impact_effects: Vec<BoxedTypeObject /* EffectWithSpeedRange */>,
}

pub trait MaterialPropertyEffectDataTrait: super::entity::PhysicsMaterialRelationPropertyDataTrait {
    fn impact_effects(&self) -> &Vec<BoxedTypeObject /* EffectWithSpeedRange */>;
    fn impact_effects_mut(&mut self) -> &mut Vec<BoxedTypeObject /* EffectWithSpeedRange */>;
}

impl MaterialPropertyEffectDataTrait for MaterialPropertyEffectData {
    fn impact_effects(&self) -> &Vec<BoxedTypeObject /* EffectWithSpeedRange */> {
        &self.impact_effects
    }
    fn impact_effects_mut(&mut self) -> &mut Vec<BoxedTypeObject /* EffectWithSpeedRange */> {
        &mut self.impact_effects
    }
}

impl super::entity::PhysicsMaterialRelationPropertyDataTrait for MaterialPropertyEffectData {
}

impl super::entity::MaterialRelationPropertyDataTrait for MaterialPropertyEffectData {
}

impl super::core::DataContainerTrait for MaterialPropertyEffectData {
}

pub static MATERIALPROPERTYEFFECTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialPropertyEffectData",
    name_hash: 1215214094,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::PHYSICSMATERIALRELATIONPROPERTYDATA_TYPE_INFO),
        super_class_offset: offset_of!(MaterialPropertyEffectData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MaterialPropertyEffectData as Default>::default())),
            create_boxed: || Box::new(<MaterialPropertyEffectData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ImpactEffects",
                name_hash: 2149182307,
                flags: MemberInfoFlags::new(144),
                field_type: "EffectWithSpeedRange-Array",
                rust_offset: offset_of!(MaterialPropertyEffectData, impact_effects),
            },
        ],
    }),
    array_type: Some(MATERIALPROPERTYEFFECTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MaterialPropertyEffectData {
    fn type_info(&self) -> &'static TypeInfo {
        MATERIALPROPERTYEFFECTDATA_TYPE_INFO
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


pub static MATERIALPROPERTYEFFECTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialPropertyEffectData-Array",
    name_hash: 3275103034,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("MaterialPropertyEffectData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EffectWithSpeedRange {
    pub effect: Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>,
    pub min_speed: f32,
    pub max_speed: f32,
}

pub trait EffectWithSpeedRangeTrait: TypeObject {
    fn effect(&self) -> &Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>;
    fn effect_mut(&mut self) -> &mut Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>;
    fn min_speed(&self) -> &f32;
    fn min_speed_mut(&mut self) -> &mut f32;
    fn max_speed(&self) -> &f32;
    fn max_speed_mut(&mut self) -> &mut f32;
}

impl EffectWithSpeedRangeTrait for EffectWithSpeedRange {
    fn effect(&self) -> &Option<LockedTypeObject /* super::effect_base::EffectBlueprint */> {
        &self.effect
    }
    fn effect_mut(&mut self) -> &mut Option<LockedTypeObject /* super::effect_base::EffectBlueprint */> {
        &mut self.effect
    }
    fn min_speed(&self) -> &f32 {
        &self.min_speed
    }
    fn min_speed_mut(&mut self) -> &mut f32 {
        &mut self.min_speed
    }
    fn max_speed(&self) -> &f32 {
        &self.max_speed
    }
    fn max_speed_mut(&mut self) -> &mut f32 {
        &mut self.max_speed
    }
}

pub static EFFECTWITHSPEEDRANGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectWithSpeedRange",
    name_hash: 323053480,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EffectWithSpeedRange as Default>::default())),
            create_boxed: || Box::new(<EffectWithSpeedRange as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Effect",
                name_hash: 2332983090,
                flags: MemberInfoFlags::new(0),
                field_type: "EffectBlueprint",
                rust_offset: offset_of!(EffectWithSpeedRange, effect),
            },
            FieldInfoData {
                name: "MinSpeed",
                name_hash: 3368183944,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EffectWithSpeedRange, min_speed),
            },
            FieldInfoData {
                name: "MaxSpeed",
                name_hash: 396228950,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EffectWithSpeedRange, max_speed),
            },
        ],
    }),
    array_type: Some(EFFECTWITHSPEEDRANGE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EffectWithSpeedRange {
    fn type_info(&self) -> &'static TypeInfo {
        EFFECTWITHSPEEDRANGE_TYPE_INFO
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


pub static EFFECTWITHSPEEDRANGE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectWithSpeedRange-Array",
    name_hash: 2773379356,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("EffectWithSpeedRange"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LevelDescriptionAsset {
    pub _glacier_base: super::core::Asset,
    pub level_name: String,
    pub categories: Vec<BoxedTypeObject /* LevelDescriptionInclusionCategory */>,
    pub level_descriptions: Vec<BoxedTypeObject /* LevelDescription */>,
    pub bundles: Vec<BoxedTypeObject /* LevelBundleLoad */>,
    pub start_points: Vec<BoxedTypeObject /* LevelStartPoint */>,
    pub super_bundles: Vec<String>,
    pub level_guid: glacier_util::guid::Guid,
}

pub trait LevelDescriptionAssetTrait: super::core::AssetTrait {
    fn level_name(&self) -> &String;
    fn level_name_mut(&mut self) -> &mut String;
    fn categories(&self) -> &Vec<BoxedTypeObject /* LevelDescriptionInclusionCategory */>;
    fn categories_mut(&mut self) -> &mut Vec<BoxedTypeObject /* LevelDescriptionInclusionCategory */>;
    fn level_descriptions(&self) -> &Vec<BoxedTypeObject /* LevelDescription */>;
    fn level_descriptions_mut(&mut self) -> &mut Vec<BoxedTypeObject /* LevelDescription */>;
    fn bundles(&self) -> &Vec<BoxedTypeObject /* LevelBundleLoad */>;
    fn bundles_mut(&mut self) -> &mut Vec<BoxedTypeObject /* LevelBundleLoad */>;
    fn start_points(&self) -> &Vec<BoxedTypeObject /* LevelStartPoint */>;
    fn start_points_mut(&mut self) -> &mut Vec<BoxedTypeObject /* LevelStartPoint */>;
    fn super_bundles(&self) -> &Vec<String>;
    fn super_bundles_mut(&mut self) -> &mut Vec<String>;
    fn level_guid(&self) -> &glacier_util::guid::Guid;
    fn level_guid_mut(&mut self) -> &mut glacier_util::guid::Guid;
}

impl LevelDescriptionAssetTrait for LevelDescriptionAsset {
    fn level_name(&self) -> &String {
        &self.level_name
    }
    fn level_name_mut(&mut self) -> &mut String {
        &mut self.level_name
    }
    fn categories(&self) -> &Vec<BoxedTypeObject /* LevelDescriptionInclusionCategory */> {
        &self.categories
    }
    fn categories_mut(&mut self) -> &mut Vec<BoxedTypeObject /* LevelDescriptionInclusionCategory */> {
        &mut self.categories
    }
    fn level_descriptions(&self) -> &Vec<BoxedTypeObject /* LevelDescription */> {
        &self.level_descriptions
    }
    fn level_descriptions_mut(&mut self) -> &mut Vec<BoxedTypeObject /* LevelDescription */> {
        &mut self.level_descriptions
    }
    fn bundles(&self) -> &Vec<BoxedTypeObject /* LevelBundleLoad */> {
        &self.bundles
    }
    fn bundles_mut(&mut self) -> &mut Vec<BoxedTypeObject /* LevelBundleLoad */> {
        &mut self.bundles
    }
    fn start_points(&self) -> &Vec<BoxedTypeObject /* LevelStartPoint */> {
        &self.start_points
    }
    fn start_points_mut(&mut self) -> &mut Vec<BoxedTypeObject /* LevelStartPoint */> {
        &mut self.start_points
    }
    fn super_bundles(&self) -> &Vec<String> {
        &self.super_bundles
    }
    fn super_bundles_mut(&mut self) -> &mut Vec<String> {
        &mut self.super_bundles
    }
    fn level_guid(&self) -> &glacier_util::guid::Guid {
        &self.level_guid
    }
    fn level_guid_mut(&mut self) -> &mut glacier_util::guid::Guid {
        &mut self.level_guid
    }
}

impl super::core::AssetTrait for LevelDescriptionAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for LevelDescriptionAsset {
}

pub static LEVELDESCRIPTIONASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelDescriptionAsset",
    name_hash: 3125119461,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(LevelDescriptionAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LevelDescriptionAsset as Default>::default())),
            create_boxed: || Box::new(<LevelDescriptionAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "LevelName",
                name_hash: 1599082292,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(LevelDescriptionAsset, level_name),
            },
            FieldInfoData {
                name: "Categories",
                name_hash: 1039077843,
                flags: MemberInfoFlags::new(144),
                field_type: "LevelDescriptionInclusionCategory-Array",
                rust_offset: offset_of!(LevelDescriptionAsset, categories),
            },
            FieldInfoData {
                name: "LevelDescriptions",
                name_hash: 1627834822,
                flags: MemberInfoFlags::new(144),
                field_type: "LevelDescription-Array",
                rust_offset: offset_of!(LevelDescriptionAsset, level_descriptions),
            },
            FieldInfoData {
                name: "Bundles",
                name_hash: 2774764642,
                flags: MemberInfoFlags::new(144),
                field_type: "LevelBundleLoad-Array",
                rust_offset: offset_of!(LevelDescriptionAsset, bundles),
            },
            FieldInfoData {
                name: "StartPoints",
                name_hash: 748137786,
                flags: MemberInfoFlags::new(144),
                field_type: "LevelStartPoint-Array",
                rust_offset: offset_of!(LevelDescriptionAsset, start_points),
            },
            FieldInfoData {
                name: "SuperBundles",
                name_hash: 3660095651,
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(LevelDescriptionAsset, super_bundles),
            },
            FieldInfoData {
                name: "LevelGuid",
                name_hash: 1598813708,
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(LevelDescriptionAsset, level_guid),
            },
        ],
    }),
    array_type: Some(LEVELDESCRIPTIONASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LevelDescriptionAsset {
    fn type_info(&self) -> &'static TypeInfo {
        LEVELDESCRIPTIONASSET_TYPE_INFO
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


pub static LEVELDESCRIPTIONASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelDescriptionAsset-Array",
    name_hash: 3067810513,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("LevelDescriptionAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LevelStartPoint {
    pub name: String,
    pub autoload_sublevels: Vec<String>,
    pub is_default: bool,
}

pub trait LevelStartPointTrait: TypeObject {
    fn name(&self) -> &String;
    fn name_mut(&mut self) -> &mut String;
    fn autoload_sublevels(&self) -> &Vec<String>;
    fn autoload_sublevels_mut(&mut self) -> &mut Vec<String>;
    fn is_default(&self) -> &bool;
    fn is_default_mut(&mut self) -> &mut bool;
}

impl LevelStartPointTrait for LevelStartPoint {
    fn name(&self) -> &String {
        &self.name
    }
    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    fn autoload_sublevels(&self) -> &Vec<String> {
        &self.autoload_sublevels
    }
    fn autoload_sublevels_mut(&mut self) -> &mut Vec<String> {
        &mut self.autoload_sublevels
    }
    fn is_default(&self) -> &bool {
        &self.is_default
    }
    fn is_default_mut(&mut self) -> &mut bool {
        &mut self.is_default
    }
}

pub static LEVELSTARTPOINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelStartPoint",
    name_hash: 2925525535,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LevelStartPoint as Default>::default())),
            create_boxed: || Box::new(<LevelStartPoint as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                name_hash: 2088949890,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(LevelStartPoint, name),
            },
            FieldInfoData {
                name: "AutoloadSublevels",
                name_hash: 1297099213,
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(LevelStartPoint, autoload_sublevels),
            },
            FieldInfoData {
                name: "IsDefault",
                name_hash: 2802724276,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LevelStartPoint, is_default),
            },
        ],
    }),
    array_type: Some(LEVELSTARTPOINT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LevelStartPoint {
    fn type_info(&self) -> &'static TypeInfo {
        LEVELSTARTPOINT_TYPE_INFO
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


pub static LEVELSTARTPOINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelStartPoint-Array",
    name_hash: 115818155,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("LevelStartPoint"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LevelBundleLoad {
    pub name: String,
    pub try_keep_between_levels: bool,
}

pub trait LevelBundleLoadTrait: TypeObject {
    fn name(&self) -> &String;
    fn name_mut(&mut self) -> &mut String;
    fn try_keep_between_levels(&self) -> &bool;
    fn try_keep_between_levels_mut(&mut self) -> &mut bool;
}

impl LevelBundleLoadTrait for LevelBundleLoad {
    fn name(&self) -> &String {
        &self.name
    }
    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    fn try_keep_between_levels(&self) -> &bool {
        &self.try_keep_between_levels
    }
    fn try_keep_between_levels_mut(&mut self) -> &mut bool {
        &mut self.try_keep_between_levels
    }
}

pub static LEVELBUNDLELOAD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelBundleLoad",
    name_hash: 2465763905,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LevelBundleLoad as Default>::default())),
            create_boxed: || Box::new(<LevelBundleLoad as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                name_hash: 2088949890,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(LevelBundleLoad, name),
            },
            FieldInfoData {
                name: "TryKeepBetweenLevels",
                name_hash: 4086627502,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LevelBundleLoad, try_keep_between_levels),
            },
        ],
    }),
    array_type: Some(LEVELBUNDLELOAD_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LevelBundleLoad {
    fn type_info(&self) -> &'static TypeInfo {
        LEVELBUNDLELOAD_TYPE_INFO
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


pub static LEVELBUNDLELOAD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelBundleLoad-Array",
    name_hash: 4191337589,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("LevelBundleLoad"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LevelDescriptionInclusionCategory {
    pub category: String,
    pub mode: Vec<String>,
}

pub trait LevelDescriptionInclusionCategoryTrait: TypeObject {
    fn category(&self) -> &String;
    fn category_mut(&mut self) -> &mut String;
    fn mode(&self) -> &Vec<String>;
    fn mode_mut(&mut self) -> &mut Vec<String>;
}

impl LevelDescriptionInclusionCategoryTrait for LevelDescriptionInclusionCategory {
    fn category(&self) -> &String {
        &self.category
    }
    fn category_mut(&mut self) -> &mut String {
        &mut self.category
    }
    fn mode(&self) -> &Vec<String> {
        &self.mode
    }
    fn mode_mut(&mut self) -> &mut Vec<String> {
        &mut self.mode
    }
}

pub static LEVELDESCRIPTIONINCLUSIONCATEGORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelDescriptionInclusionCategory",
    name_hash: 3520560323,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LevelDescriptionInclusionCategory as Default>::default())),
            create_boxed: || Box::new(<LevelDescriptionInclusionCategory as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Category",
                name_hash: 3455858997,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(LevelDescriptionInclusionCategory, category),
            },
            FieldInfoData {
                name: "Mode",
                name_hash: 2088772358,
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(LevelDescriptionInclusionCategory, mode),
            },
        ],
    }),
    array_type: Some(LEVELDESCRIPTIONINCLUSIONCATEGORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LevelDescriptionInclusionCategory {
    fn type_info(&self) -> &'static TypeInfo {
        LEVELDESCRIPTIONINCLUSIONCATEGORY_TYPE_INFO
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


pub static LEVELDESCRIPTIONINCLUSIONCATEGORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelDescriptionInclusionCategory-Array",
    name_hash: 964935671,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("LevelDescriptionInclusionCategory"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WorldRenderLevelDescriptionComponent {
    pub _glacier_base: LevelDescriptionComponent,
    pub shadowmap_slice_count_offset: super::core::PlatformScalableInt,
    pub shadowmap_slice_count_min: super::core::PlatformScalableInt,
    pub shadowmap_slice_count_max: super::core::PlatformScalableInt,
    pub shadowmap_slice_resolution_scale: super::core::PlatformScalableFloat,
    pub spotlight_shadow_count: super::core::PlatformScalableInt,
    pub transparent_dof_enable: super::core::PlatformScalableInt,
    pub sprite_dof_enable: super::core::PlatformScalableInt,
    pub sprite_dof_max_radius_gather_pass: super::core::PlatformScalableFloat,
    pub local_planar_reflections_enable: super::core::PlatformScalableInt,
    pub local_planar_reflection_convolution_enable: super::core::PlatformScalableInt,
    pub local_planar_reflection_force_lowest_lod_enable: super::core::PlatformScalableInt,
    pub local_light_shadow_atlas_slot_count: super::core::PlatformScalableInt,
    pub local_light_shadow_atlas_slot_resolution: super::core::PlatformScalableInt,
    pub local_light_shadow_resolution_high: super::core::PlatformScalableInt,
    pub local_light_shadow_resolution_ultra: super::core::PlatformScalableInt,
    pub procedural_sky_receive_height_fog: super::core::PlatformScalableInt,
}

pub trait WorldRenderLevelDescriptionComponentTrait: LevelDescriptionComponentTrait {
    fn shadowmap_slice_count_offset(&self) -> &super::core::PlatformScalableInt;
    fn shadowmap_slice_count_offset_mut(&mut self) -> &mut super::core::PlatformScalableInt;
    fn shadowmap_slice_count_min(&self) -> &super::core::PlatformScalableInt;
    fn shadowmap_slice_count_min_mut(&mut self) -> &mut super::core::PlatformScalableInt;
    fn shadowmap_slice_count_max(&self) -> &super::core::PlatformScalableInt;
    fn shadowmap_slice_count_max_mut(&mut self) -> &mut super::core::PlatformScalableInt;
    fn shadowmap_slice_resolution_scale(&self) -> &super::core::PlatformScalableFloat;
    fn shadowmap_slice_resolution_scale_mut(&mut self) -> &mut super::core::PlatformScalableFloat;
    fn spotlight_shadow_count(&self) -> &super::core::PlatformScalableInt;
    fn spotlight_shadow_count_mut(&mut self) -> &mut super::core::PlatformScalableInt;
    fn transparent_dof_enable(&self) -> &super::core::PlatformScalableInt;
    fn transparent_dof_enable_mut(&mut self) -> &mut super::core::PlatformScalableInt;
    fn sprite_dof_enable(&self) -> &super::core::PlatformScalableInt;
    fn sprite_dof_enable_mut(&mut self) -> &mut super::core::PlatformScalableInt;
    fn sprite_dof_max_radius_gather_pass(&self) -> &super::core::PlatformScalableFloat;
    fn sprite_dof_max_radius_gather_pass_mut(&mut self) -> &mut super::core::PlatformScalableFloat;
    fn local_planar_reflections_enable(&self) -> &super::core::PlatformScalableInt;
    fn local_planar_reflections_enable_mut(&mut self) -> &mut super::core::PlatformScalableInt;
    fn local_planar_reflection_convolution_enable(&self) -> &super::core::PlatformScalableInt;
    fn local_planar_reflection_convolution_enable_mut(&mut self) -> &mut super::core::PlatformScalableInt;
    fn local_planar_reflection_force_lowest_lod_enable(&self) -> &super::core::PlatformScalableInt;
    fn local_planar_reflection_force_lowest_lod_enable_mut(&mut self) -> &mut super::core::PlatformScalableInt;
    fn local_light_shadow_atlas_slot_count(&self) -> &super::core::PlatformScalableInt;
    fn local_light_shadow_atlas_slot_count_mut(&mut self) -> &mut super::core::PlatformScalableInt;
    fn local_light_shadow_atlas_slot_resolution(&self) -> &super::core::PlatformScalableInt;
    fn local_light_shadow_atlas_slot_resolution_mut(&mut self) -> &mut super::core::PlatformScalableInt;
    fn local_light_shadow_resolution_high(&self) -> &super::core::PlatformScalableInt;
    fn local_light_shadow_resolution_high_mut(&mut self) -> &mut super::core::PlatformScalableInt;
    fn local_light_shadow_resolution_ultra(&self) -> &super::core::PlatformScalableInt;
    fn local_light_shadow_resolution_ultra_mut(&mut self) -> &mut super::core::PlatformScalableInt;
    fn procedural_sky_receive_height_fog(&self) -> &super::core::PlatformScalableInt;
    fn procedural_sky_receive_height_fog_mut(&mut self) -> &mut super::core::PlatformScalableInt;
}

impl WorldRenderLevelDescriptionComponentTrait for WorldRenderLevelDescriptionComponent {
    fn shadowmap_slice_count_offset(&self) -> &super::core::PlatformScalableInt {
        &self.shadowmap_slice_count_offset
    }
    fn shadowmap_slice_count_offset_mut(&mut self) -> &mut super::core::PlatformScalableInt {
        &mut self.shadowmap_slice_count_offset
    }
    fn shadowmap_slice_count_min(&self) -> &super::core::PlatformScalableInt {
        &self.shadowmap_slice_count_min
    }
    fn shadowmap_slice_count_min_mut(&mut self) -> &mut super::core::PlatformScalableInt {
        &mut self.shadowmap_slice_count_min
    }
    fn shadowmap_slice_count_max(&self) -> &super::core::PlatformScalableInt {
        &self.shadowmap_slice_count_max
    }
    fn shadowmap_slice_count_max_mut(&mut self) -> &mut super::core::PlatformScalableInt {
        &mut self.shadowmap_slice_count_max
    }
    fn shadowmap_slice_resolution_scale(&self) -> &super::core::PlatformScalableFloat {
        &self.shadowmap_slice_resolution_scale
    }
    fn shadowmap_slice_resolution_scale_mut(&mut self) -> &mut super::core::PlatformScalableFloat {
        &mut self.shadowmap_slice_resolution_scale
    }
    fn spotlight_shadow_count(&self) -> &super::core::PlatformScalableInt {
        &self.spotlight_shadow_count
    }
    fn spotlight_shadow_count_mut(&mut self) -> &mut super::core::PlatformScalableInt {
        &mut self.spotlight_shadow_count
    }
    fn transparent_dof_enable(&self) -> &super::core::PlatformScalableInt {
        &self.transparent_dof_enable
    }
    fn transparent_dof_enable_mut(&mut self) -> &mut super::core::PlatformScalableInt {
        &mut self.transparent_dof_enable
    }
    fn sprite_dof_enable(&self) -> &super::core::PlatformScalableInt {
        &self.sprite_dof_enable
    }
    fn sprite_dof_enable_mut(&mut self) -> &mut super::core::PlatformScalableInt {
        &mut self.sprite_dof_enable
    }
    fn sprite_dof_max_radius_gather_pass(&self) -> &super::core::PlatformScalableFloat {
        &self.sprite_dof_max_radius_gather_pass
    }
    fn sprite_dof_max_radius_gather_pass_mut(&mut self) -> &mut super::core::PlatformScalableFloat {
        &mut self.sprite_dof_max_radius_gather_pass
    }
    fn local_planar_reflections_enable(&self) -> &super::core::PlatformScalableInt {
        &self.local_planar_reflections_enable
    }
    fn local_planar_reflections_enable_mut(&mut self) -> &mut super::core::PlatformScalableInt {
        &mut self.local_planar_reflections_enable
    }
    fn local_planar_reflection_convolution_enable(&self) -> &super::core::PlatformScalableInt {
        &self.local_planar_reflection_convolution_enable
    }
    fn local_planar_reflection_convolution_enable_mut(&mut self) -> &mut super::core::PlatformScalableInt {
        &mut self.local_planar_reflection_convolution_enable
    }
    fn local_planar_reflection_force_lowest_lod_enable(&self) -> &super::core::PlatformScalableInt {
        &self.local_planar_reflection_force_lowest_lod_enable
    }
    fn local_planar_reflection_force_lowest_lod_enable_mut(&mut self) -> &mut super::core::PlatformScalableInt {
        &mut self.local_planar_reflection_force_lowest_lod_enable
    }
    fn local_light_shadow_atlas_slot_count(&self) -> &super::core::PlatformScalableInt {
        &self.local_light_shadow_atlas_slot_count
    }
    fn local_light_shadow_atlas_slot_count_mut(&mut self) -> &mut super::core::PlatformScalableInt {
        &mut self.local_light_shadow_atlas_slot_count
    }
    fn local_light_shadow_atlas_slot_resolution(&self) -> &super::core::PlatformScalableInt {
        &self.local_light_shadow_atlas_slot_resolution
    }
    fn local_light_shadow_atlas_slot_resolution_mut(&mut self) -> &mut super::core::PlatformScalableInt {
        &mut self.local_light_shadow_atlas_slot_resolution
    }
    fn local_light_shadow_resolution_high(&self) -> &super::core::PlatformScalableInt {
        &self.local_light_shadow_resolution_high
    }
    fn local_light_shadow_resolution_high_mut(&mut self) -> &mut super::core::PlatformScalableInt {
        &mut self.local_light_shadow_resolution_high
    }
    fn local_light_shadow_resolution_ultra(&self) -> &super::core::PlatformScalableInt {
        &self.local_light_shadow_resolution_ultra
    }
    fn local_light_shadow_resolution_ultra_mut(&mut self) -> &mut super::core::PlatformScalableInt {
        &mut self.local_light_shadow_resolution_ultra
    }
    fn procedural_sky_receive_height_fog(&self) -> &super::core::PlatformScalableInt {
        &self.procedural_sky_receive_height_fog
    }
    fn procedural_sky_receive_height_fog_mut(&mut self) -> &mut super::core::PlatformScalableInt {
        &mut self.procedural_sky_receive_height_fog
    }
}

impl LevelDescriptionComponentTrait for WorldRenderLevelDescriptionComponent {
}

impl super::core::DataContainerTrait for WorldRenderLevelDescriptionComponent {
}

pub static WORLDRENDERLEVELDESCRIPTIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldRenderLevelDescriptionComponent",
    name_hash: 577969874,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LEVELDESCRIPTIONCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(WorldRenderLevelDescriptionComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WorldRenderLevelDescriptionComponent as Default>::default())),
            create_boxed: || Box::new(<WorldRenderLevelDescriptionComponent as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ShadowmapSliceCountOffset",
                name_hash: 2120894337,
                flags: MemberInfoFlags::new(0),
                field_type: "PlatformScalableInt",
                rust_offset: offset_of!(WorldRenderLevelDescriptionComponent, shadowmap_slice_count_offset),
            },
            FieldInfoData {
                name: "ShadowmapSliceCountMin",
                name_hash: 1964261958,
                flags: MemberInfoFlags::new(0),
                field_type: "PlatformScalableInt",
                rust_offset: offset_of!(WorldRenderLevelDescriptionComponent, shadowmap_slice_count_min),
            },
            FieldInfoData {
                name: "ShadowmapSliceCountMax",
                name_hash: 1964261720,
                flags: MemberInfoFlags::new(0),
                field_type: "PlatformScalableInt",
                rust_offset: offset_of!(WorldRenderLevelDescriptionComponent, shadowmap_slice_count_max),
            },
            FieldInfoData {
                name: "ShadowmapSliceResolutionScale",
                name_hash: 2721270425,
                flags: MemberInfoFlags::new(0),
                field_type: "PlatformScalableFloat",
                rust_offset: offset_of!(WorldRenderLevelDescriptionComponent, shadowmap_slice_resolution_scale),
            },
            FieldInfoData {
                name: "SpotlightShadowCount",
                name_hash: 3429092550,
                flags: MemberInfoFlags::new(0),
                field_type: "PlatformScalableInt",
                rust_offset: offset_of!(WorldRenderLevelDescriptionComponent, spotlight_shadow_count),
            },
            FieldInfoData {
                name: "TransparentDofEnable",
                name_hash: 149863631,
                flags: MemberInfoFlags::new(0),
                field_type: "PlatformScalableInt",
                rust_offset: offset_of!(WorldRenderLevelDescriptionComponent, transparent_dof_enable),
            },
            FieldInfoData {
                name: "SpriteDofEnable",
                name_hash: 3125443712,
                flags: MemberInfoFlags::new(0),
                field_type: "PlatformScalableInt",
                rust_offset: offset_of!(WorldRenderLevelDescriptionComponent, sprite_dof_enable),
            },
            FieldInfoData {
                name: "SpriteDofMaxRadiusGatherPass",
                name_hash: 3083430481,
                flags: MemberInfoFlags::new(0),
                field_type: "PlatformScalableFloat",
                rust_offset: offset_of!(WorldRenderLevelDescriptionComponent, sprite_dof_max_radius_gather_pass),
            },
            FieldInfoData {
                name: "LocalPlanarReflectionsEnable",
                name_hash: 2240627165,
                flags: MemberInfoFlags::new(0),
                field_type: "PlatformScalableInt",
                rust_offset: offset_of!(WorldRenderLevelDescriptionComponent, local_planar_reflections_enable),
            },
            FieldInfoData {
                name: "LocalPlanarReflectionConvolutionEnable",
                name_hash: 771848240,
                flags: MemberInfoFlags::new(0),
                field_type: "PlatformScalableInt",
                rust_offset: offset_of!(WorldRenderLevelDescriptionComponent, local_planar_reflection_convolution_enable),
            },
            FieldInfoData {
                name: "LocalPlanarReflectionForceLowestLodEnable",
                name_hash: 3357846242,
                flags: MemberInfoFlags::new(0),
                field_type: "PlatformScalableInt",
                rust_offset: offset_of!(WorldRenderLevelDescriptionComponent, local_planar_reflection_force_lowest_lod_enable),
            },
            FieldInfoData {
                name: "LocalLightShadowAtlasSlotCount",
                name_hash: 2036458652,
                flags: MemberInfoFlags::new(0),
                field_type: "PlatformScalableInt",
                rust_offset: offset_of!(WorldRenderLevelDescriptionComponent, local_light_shadow_atlas_slot_count),
            },
            FieldInfoData {
                name: "LocalLightShadowAtlasSlotResolution",
                name_hash: 1047618289,
                flags: MemberInfoFlags::new(0),
                field_type: "PlatformScalableInt",
                rust_offset: offset_of!(WorldRenderLevelDescriptionComponent, local_light_shadow_atlas_slot_resolution),
            },
            FieldInfoData {
                name: "LocalLightShadowResolutionHigh",
                name_hash: 2713058160,
                flags: MemberInfoFlags::new(0),
                field_type: "PlatformScalableInt",
                rust_offset: offset_of!(WorldRenderLevelDescriptionComponent, local_light_shadow_resolution_high),
            },
            FieldInfoData {
                name: "LocalLightShadowResolutionUltra",
                name_hash: 3625051104,
                flags: MemberInfoFlags::new(0),
                field_type: "PlatformScalableInt",
                rust_offset: offset_of!(WorldRenderLevelDescriptionComponent, local_light_shadow_resolution_ultra),
            },
            FieldInfoData {
                name: "ProceduralSkyReceiveHeightFog",
                name_hash: 2498783931,
                flags: MemberInfoFlags::new(0),
                field_type: "PlatformScalableInt",
                rust_offset: offset_of!(WorldRenderLevelDescriptionComponent, procedural_sky_receive_height_fog),
            },
        ],
    }),
    array_type: Some(WORLDRENDERLEVELDESCRIPTIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WorldRenderLevelDescriptionComponent {
    fn type_info(&self) -> &'static TypeInfo {
        WORLDRENDERLEVELDESCRIPTIONCOMPONENT_TYPE_INFO
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


pub static WORLDRENDERLEVELDESCRIPTIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldRenderLevelDescriptionComponent-Array",
    name_hash: 3534558822,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("WorldRenderLevelDescriptionComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MemoryLevelDescriptionComponent {
    pub _glacier_base: LevelDescriptionComponent,
    pub texture_streaming_pool_size: super::core::PlatformScalableInt,
    pub mesh_streaming_pool_size: super::core::PlatformScalableInt,
    pub emitter_base_atlas_width: super::core::PlatformScalableInt,
    pub emitter_base_atlas_height: super::core::PlatformScalableInt,
    pub emitter_base_atlas_mipmap_count: super::core::PlatformScalableInt,
    pub emitter_normal_atlas_width: super::core::PlatformScalableInt,
    pub emitter_normal_atlas_height: super::core::PlatformScalableInt,
    pub emitter_normal_atlas_mipmap_count: super::core::PlatformScalableInt,
}

pub trait MemoryLevelDescriptionComponentTrait: LevelDescriptionComponentTrait {
    fn texture_streaming_pool_size(&self) -> &super::core::PlatformScalableInt;
    fn texture_streaming_pool_size_mut(&mut self) -> &mut super::core::PlatformScalableInt;
    fn mesh_streaming_pool_size(&self) -> &super::core::PlatformScalableInt;
    fn mesh_streaming_pool_size_mut(&mut self) -> &mut super::core::PlatformScalableInt;
    fn emitter_base_atlas_width(&self) -> &super::core::PlatformScalableInt;
    fn emitter_base_atlas_width_mut(&mut self) -> &mut super::core::PlatformScalableInt;
    fn emitter_base_atlas_height(&self) -> &super::core::PlatformScalableInt;
    fn emitter_base_atlas_height_mut(&mut self) -> &mut super::core::PlatformScalableInt;
    fn emitter_base_atlas_mipmap_count(&self) -> &super::core::PlatformScalableInt;
    fn emitter_base_atlas_mipmap_count_mut(&mut self) -> &mut super::core::PlatformScalableInt;
    fn emitter_normal_atlas_width(&self) -> &super::core::PlatformScalableInt;
    fn emitter_normal_atlas_width_mut(&mut self) -> &mut super::core::PlatformScalableInt;
    fn emitter_normal_atlas_height(&self) -> &super::core::PlatformScalableInt;
    fn emitter_normal_atlas_height_mut(&mut self) -> &mut super::core::PlatformScalableInt;
    fn emitter_normal_atlas_mipmap_count(&self) -> &super::core::PlatformScalableInt;
    fn emitter_normal_atlas_mipmap_count_mut(&mut self) -> &mut super::core::PlatformScalableInt;
}

impl MemoryLevelDescriptionComponentTrait for MemoryLevelDescriptionComponent {
    fn texture_streaming_pool_size(&self) -> &super::core::PlatformScalableInt {
        &self.texture_streaming_pool_size
    }
    fn texture_streaming_pool_size_mut(&mut self) -> &mut super::core::PlatformScalableInt {
        &mut self.texture_streaming_pool_size
    }
    fn mesh_streaming_pool_size(&self) -> &super::core::PlatformScalableInt {
        &self.mesh_streaming_pool_size
    }
    fn mesh_streaming_pool_size_mut(&mut self) -> &mut super::core::PlatformScalableInt {
        &mut self.mesh_streaming_pool_size
    }
    fn emitter_base_atlas_width(&self) -> &super::core::PlatformScalableInt {
        &self.emitter_base_atlas_width
    }
    fn emitter_base_atlas_width_mut(&mut self) -> &mut super::core::PlatformScalableInt {
        &mut self.emitter_base_atlas_width
    }
    fn emitter_base_atlas_height(&self) -> &super::core::PlatformScalableInt {
        &self.emitter_base_atlas_height
    }
    fn emitter_base_atlas_height_mut(&mut self) -> &mut super::core::PlatformScalableInt {
        &mut self.emitter_base_atlas_height
    }
    fn emitter_base_atlas_mipmap_count(&self) -> &super::core::PlatformScalableInt {
        &self.emitter_base_atlas_mipmap_count
    }
    fn emitter_base_atlas_mipmap_count_mut(&mut self) -> &mut super::core::PlatformScalableInt {
        &mut self.emitter_base_atlas_mipmap_count
    }
    fn emitter_normal_atlas_width(&self) -> &super::core::PlatformScalableInt {
        &self.emitter_normal_atlas_width
    }
    fn emitter_normal_atlas_width_mut(&mut self) -> &mut super::core::PlatformScalableInt {
        &mut self.emitter_normal_atlas_width
    }
    fn emitter_normal_atlas_height(&self) -> &super::core::PlatformScalableInt {
        &self.emitter_normal_atlas_height
    }
    fn emitter_normal_atlas_height_mut(&mut self) -> &mut super::core::PlatformScalableInt {
        &mut self.emitter_normal_atlas_height
    }
    fn emitter_normal_atlas_mipmap_count(&self) -> &super::core::PlatformScalableInt {
        &self.emitter_normal_atlas_mipmap_count
    }
    fn emitter_normal_atlas_mipmap_count_mut(&mut self) -> &mut super::core::PlatformScalableInt {
        &mut self.emitter_normal_atlas_mipmap_count
    }
}

impl LevelDescriptionComponentTrait for MemoryLevelDescriptionComponent {
}

impl super::core::DataContainerTrait for MemoryLevelDescriptionComponent {
}

pub static MEMORYLEVELDESCRIPTIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryLevelDescriptionComponent",
    name_hash: 1692979547,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LEVELDESCRIPTIONCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(MemoryLevelDescriptionComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MemoryLevelDescriptionComponent as Default>::default())),
            create_boxed: || Box::new(<MemoryLevelDescriptionComponent as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "TextureStreamingPoolSize",
                name_hash: 1881392735,
                flags: MemberInfoFlags::new(0),
                field_type: "PlatformScalableInt",
                rust_offset: offset_of!(MemoryLevelDescriptionComponent, texture_streaming_pool_size),
            },
            FieldInfoData {
                name: "MeshStreamingPoolSize",
                name_hash: 4096161267,
                flags: MemberInfoFlags::new(0),
                field_type: "PlatformScalableInt",
                rust_offset: offset_of!(MemoryLevelDescriptionComponent, mesh_streaming_pool_size),
            },
            FieldInfoData {
                name: "EmitterBaseAtlasWidth",
                name_hash: 1899807243,
                flags: MemberInfoFlags::new(0),
                field_type: "PlatformScalableInt",
                rust_offset: offset_of!(MemoryLevelDescriptionComponent, emitter_base_atlas_width),
            },
            FieldInfoData {
                name: "EmitterBaseAtlasHeight",
                name_hash: 1825332754,
                flags: MemberInfoFlags::new(0),
                field_type: "PlatformScalableInt",
                rust_offset: offset_of!(MemoryLevelDescriptionComponent, emitter_base_atlas_height),
            },
            FieldInfoData {
                name: "EmitterBaseAtlasMipmapCount",
                name_hash: 929371110,
                flags: MemberInfoFlags::new(0),
                field_type: "PlatformScalableInt",
                rust_offset: offset_of!(MemoryLevelDescriptionComponent, emitter_base_atlas_mipmap_count),
            },
            FieldInfoData {
                name: "EmitterNormalAtlasWidth",
                name_hash: 2463706061,
                flags: MemberInfoFlags::new(0),
                field_type: "PlatformScalableInt",
                rust_offset: offset_of!(MemoryLevelDescriptionComponent, emitter_normal_atlas_width),
            },
            FieldInfoData {
                name: "EmitterNormalAtlasHeight",
                name_hash: 3098136212,
                flags: MemberInfoFlags::new(0),
                field_type: "PlatformScalableInt",
                rust_offset: offset_of!(MemoryLevelDescriptionComponent, emitter_normal_atlas_height),
            },
            FieldInfoData {
                name: "EmitterNormalAtlasMipmapCount",
                name_hash: 345229216,
                flags: MemberInfoFlags::new(0),
                field_type: "PlatformScalableInt",
                rust_offset: offset_of!(MemoryLevelDescriptionComponent, emitter_normal_atlas_mipmap_count),
            },
        ],
    }),
    array_type: Some(MEMORYLEVELDESCRIPTIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MemoryLevelDescriptionComponent {
    fn type_info(&self) -> &'static TypeInfo {
        MEMORYLEVELDESCRIPTIONCOMPONENT_TYPE_INFO
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


pub static MEMORYLEVELDESCRIPTIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryLevelDescriptionComponent-Array",
    name_hash: 3228866671,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("MemoryLevelDescriptionComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LevelDescription {
    pub name: String,
    pub sub_name: String,
    pub description: String,
    pub is_singleplayer: bool,
    pub is_coop: bool,
    pub is_menu: bool,
    pub is_epilogue: bool,
    pub components: Vec<Option<LockedTypeObject /* LevelDescriptionComponent */>>,
}

pub trait LevelDescriptionTrait: TypeObject {
    fn name(&self) -> &String;
    fn name_mut(&mut self) -> &mut String;
    fn sub_name(&self) -> &String;
    fn sub_name_mut(&mut self) -> &mut String;
    fn description(&self) -> &String;
    fn description_mut(&mut self) -> &mut String;
    fn is_singleplayer(&self) -> &bool;
    fn is_singleplayer_mut(&mut self) -> &mut bool;
    fn is_coop(&self) -> &bool;
    fn is_coop_mut(&mut self) -> &mut bool;
    fn is_menu(&self) -> &bool;
    fn is_menu_mut(&mut self) -> &mut bool;
    fn is_epilogue(&self) -> &bool;
    fn is_epilogue_mut(&mut self) -> &mut bool;
    fn components(&self) -> &Vec<Option<LockedTypeObject /* LevelDescriptionComponent */>>;
    fn components_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* LevelDescriptionComponent */>>;
}

impl LevelDescriptionTrait for LevelDescription {
    fn name(&self) -> &String {
        &self.name
    }
    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    fn sub_name(&self) -> &String {
        &self.sub_name
    }
    fn sub_name_mut(&mut self) -> &mut String {
        &mut self.sub_name
    }
    fn description(&self) -> &String {
        &self.description
    }
    fn description_mut(&mut self) -> &mut String {
        &mut self.description
    }
    fn is_singleplayer(&self) -> &bool {
        &self.is_singleplayer
    }
    fn is_singleplayer_mut(&mut self) -> &mut bool {
        &mut self.is_singleplayer
    }
    fn is_coop(&self) -> &bool {
        &self.is_coop
    }
    fn is_coop_mut(&mut self) -> &mut bool {
        &mut self.is_coop
    }
    fn is_menu(&self) -> &bool {
        &self.is_menu
    }
    fn is_menu_mut(&mut self) -> &mut bool {
        &mut self.is_menu
    }
    fn is_epilogue(&self) -> &bool {
        &self.is_epilogue
    }
    fn is_epilogue_mut(&mut self) -> &mut bool {
        &mut self.is_epilogue
    }
    fn components(&self) -> &Vec<Option<LockedTypeObject /* LevelDescriptionComponent */>> {
        &self.components
    }
    fn components_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* LevelDescriptionComponent */>> {
        &mut self.components
    }
}

pub static LEVELDESCRIPTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelDescription",
    name_hash: 1741285141,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LevelDescription as Default>::default())),
            create_boxed: || Box::new(<LevelDescription as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                name_hash: 2088949890,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(LevelDescription, name),
            },
            FieldInfoData {
                name: "SubName",
                name_hash: 2135224774,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(LevelDescription, sub_name),
            },
            FieldInfoData {
                name: "Description",
                name_hash: 1636673251,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(LevelDescription, description),
            },
            FieldInfoData {
                name: "IsSingleplayer",
                name_hash: 2936976598,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LevelDescription, is_singleplayer),
            },
            FieldInfoData {
                name: "IsCoop",
                name_hash: 2816971820,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LevelDescription, is_coop),
            },
            FieldInfoData {
                name: "IsMenu",
                name_hash: 2816483916,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LevelDescription, is_menu),
            },
            FieldInfoData {
                name: "IsEpilogue",
                name_hash: 2625976567,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LevelDescription, is_epilogue),
            },
            FieldInfoData {
                name: "Components",
                name_hash: 3391050425,
                flags: MemberInfoFlags::new(144),
                field_type: "LevelDescriptionComponent-Array",
                rust_offset: offset_of!(LevelDescription, components),
            },
        ],
    }),
    array_type: Some(LEVELDESCRIPTION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LevelDescription {
    fn type_info(&self) -> &'static TypeInfo {
        LEVELDESCRIPTION_TYPE_INFO
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


pub static LEVELDESCRIPTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelDescription-Array",
    name_hash: 949425825,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("LevelDescription"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LevelDescriptionComponent {
    pub _glacier_base: super::core::DataContainer,
}

pub trait LevelDescriptionComponentTrait: super::core::DataContainerTrait {
}

impl LevelDescriptionComponentTrait for LevelDescriptionComponent {
}

impl super::core::DataContainerTrait for LevelDescriptionComponent {
}

pub static LEVELDESCRIPTIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelDescriptionComponent",
    name_hash: 1303720346,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(LevelDescriptionComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LevelDescriptionComponent as Default>::default())),
            create_boxed: || Box::new(<LevelDescriptionComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(LEVELDESCRIPTIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LevelDescriptionComponent {
    fn type_info(&self) -> &'static TypeInfo {
        LEVELDESCRIPTIONCOMPONENT_TYPE_INFO
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


pub static LEVELDESCRIPTIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelDescriptionComponent-Array",
    name_hash: 1955681454,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("LevelDescriptionComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LevelData {
    pub _glacier_base: super::entity::WorldData,
    pub a_i_system: Option<LockedTypeObject /* GameAISystem */>,
    pub a_i2_system: Option<LockedTypeObject /* GameAISystem */>,
    pub game_configuration_name: String,
    pub default_f_o_v: f32,
    pub infantry_f_o_v_multiplier: f32,
    pub stream_pool_preset: Option<LockedTypeObject /* super::audio::StreamPoolPreset */>,
    pub voice_over_system: Option<LockedTypeObject /* super::audio::VoiceOverSystemAsset */>,
    pub voice_over_logic: Vec<Option<LockedTypeObject /* super::audio::VoiceOverLogicAsset */>>,
    pub max_vehicle_height: f32,
    pub huge_broad_phase: bool,
    pub enlighten_shader_database: Option<LockedTypeObject /* super::render_base::EnlightenShaderDatabaseBaseAsset */>,
    pub ant_project_asset: Option<LockedTypeObject /* super::ant::AntProjectAsset */>,
    pub aerial_heightmap_data: String,
    pub free_streaming_enable: bool,
    pub camera_modes: Vec<Option<LockedTypeObject /* CameraModeAsset */>>,
    pub camera_transitions: Vec<Option<LockedTypeObject /* CameraTransition */>>,
    pub preload_info: LevelPreloadInfo,
    pub face_animation_wave_mappings: Option<LockedTypeObject /* FaceAnimationWaveMappings */>,
    pub auto_load_bundles: Vec<String>,
    pub unlock_id_table: UnlockIdTable,
    pub autoloaded_detached_sub_world_datas: Vec<Option<LockedTypeObject /* super::entity::SubWorldReferenceObjectData */>>,
}

pub trait LevelDataTrait: super::entity::WorldDataTrait {
    fn a_i_system(&self) -> &Option<LockedTypeObject /* GameAISystem */>;
    fn a_i_system_mut(&mut self) -> &mut Option<LockedTypeObject /* GameAISystem */>;
    fn a_i2_system(&self) -> &Option<LockedTypeObject /* GameAISystem */>;
    fn a_i2_system_mut(&mut self) -> &mut Option<LockedTypeObject /* GameAISystem */>;
    fn game_configuration_name(&self) -> &String;
    fn game_configuration_name_mut(&mut self) -> &mut String;
    fn default_f_o_v(&self) -> &f32;
    fn default_f_o_v_mut(&mut self) -> &mut f32;
    fn infantry_f_o_v_multiplier(&self) -> &f32;
    fn infantry_f_o_v_multiplier_mut(&mut self) -> &mut f32;
    fn stream_pool_preset(&self) -> &Option<LockedTypeObject /* super::audio::StreamPoolPreset */>;
    fn stream_pool_preset_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::StreamPoolPreset */>;
    fn voice_over_system(&self) -> &Option<LockedTypeObject /* super::audio::VoiceOverSystemAsset */>;
    fn voice_over_system_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::VoiceOverSystemAsset */>;
    fn voice_over_logic(&self) -> &Vec<Option<LockedTypeObject /* super::audio::VoiceOverLogicAsset */>>;
    fn voice_over_logic_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::audio::VoiceOverLogicAsset */>>;
    fn max_vehicle_height(&self) -> &f32;
    fn max_vehicle_height_mut(&mut self) -> &mut f32;
    fn huge_broad_phase(&self) -> &bool;
    fn huge_broad_phase_mut(&mut self) -> &mut bool;
    fn enlighten_shader_database(&self) -> &Option<LockedTypeObject /* super::render_base::EnlightenShaderDatabaseBaseAsset */>;
    fn enlighten_shader_database_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::EnlightenShaderDatabaseBaseAsset */>;
    fn ant_project_asset(&self) -> &Option<LockedTypeObject /* super::ant::AntProjectAsset */>;
    fn ant_project_asset_mut(&mut self) -> &mut Option<LockedTypeObject /* super::ant::AntProjectAsset */>;
    fn aerial_heightmap_data(&self) -> &String;
    fn aerial_heightmap_data_mut(&mut self) -> &mut String;
    fn free_streaming_enable(&self) -> &bool;
    fn free_streaming_enable_mut(&mut self) -> &mut bool;
    fn camera_modes(&self) -> &Vec<Option<LockedTypeObject /* CameraModeAsset */>>;
    fn camera_modes_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* CameraModeAsset */>>;
    fn camera_transitions(&self) -> &Vec<Option<LockedTypeObject /* CameraTransition */>>;
    fn camera_transitions_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* CameraTransition */>>;
    fn preload_info(&self) -> &LevelPreloadInfo;
    fn preload_info_mut(&mut self) -> &mut LevelPreloadInfo;
    fn face_animation_wave_mappings(&self) -> &Option<LockedTypeObject /* FaceAnimationWaveMappings */>;
    fn face_animation_wave_mappings_mut(&mut self) -> &mut Option<LockedTypeObject /* FaceAnimationWaveMappings */>;
    fn auto_load_bundles(&self) -> &Vec<String>;
    fn auto_load_bundles_mut(&mut self) -> &mut Vec<String>;
    fn unlock_id_table(&self) -> &UnlockIdTable;
    fn unlock_id_table_mut(&mut self) -> &mut UnlockIdTable;
    fn autoloaded_detached_sub_world_datas(&self) -> &Vec<Option<LockedTypeObject /* super::entity::SubWorldReferenceObjectData */>>;
    fn autoloaded_detached_sub_world_datas_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::SubWorldReferenceObjectData */>>;
}

impl LevelDataTrait for LevelData {
    fn a_i_system(&self) -> &Option<LockedTypeObject /* GameAISystem */> {
        &self.a_i_system
    }
    fn a_i_system_mut(&mut self) -> &mut Option<LockedTypeObject /* GameAISystem */> {
        &mut self.a_i_system
    }
    fn a_i2_system(&self) -> &Option<LockedTypeObject /* GameAISystem */> {
        &self.a_i2_system
    }
    fn a_i2_system_mut(&mut self) -> &mut Option<LockedTypeObject /* GameAISystem */> {
        &mut self.a_i2_system
    }
    fn game_configuration_name(&self) -> &String {
        &self.game_configuration_name
    }
    fn game_configuration_name_mut(&mut self) -> &mut String {
        &mut self.game_configuration_name
    }
    fn default_f_o_v(&self) -> &f32 {
        &self.default_f_o_v
    }
    fn default_f_o_v_mut(&mut self) -> &mut f32 {
        &mut self.default_f_o_v
    }
    fn infantry_f_o_v_multiplier(&self) -> &f32 {
        &self.infantry_f_o_v_multiplier
    }
    fn infantry_f_o_v_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.infantry_f_o_v_multiplier
    }
    fn stream_pool_preset(&self) -> &Option<LockedTypeObject /* super::audio::StreamPoolPreset */> {
        &self.stream_pool_preset
    }
    fn stream_pool_preset_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::StreamPoolPreset */> {
        &mut self.stream_pool_preset
    }
    fn voice_over_system(&self) -> &Option<LockedTypeObject /* super::audio::VoiceOverSystemAsset */> {
        &self.voice_over_system
    }
    fn voice_over_system_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::VoiceOverSystemAsset */> {
        &mut self.voice_over_system
    }
    fn voice_over_logic(&self) -> &Vec<Option<LockedTypeObject /* super::audio::VoiceOverLogicAsset */>> {
        &self.voice_over_logic
    }
    fn voice_over_logic_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::audio::VoiceOverLogicAsset */>> {
        &mut self.voice_over_logic
    }
    fn max_vehicle_height(&self) -> &f32 {
        &self.max_vehicle_height
    }
    fn max_vehicle_height_mut(&mut self) -> &mut f32 {
        &mut self.max_vehicle_height
    }
    fn huge_broad_phase(&self) -> &bool {
        &self.huge_broad_phase
    }
    fn huge_broad_phase_mut(&mut self) -> &mut bool {
        &mut self.huge_broad_phase
    }
    fn enlighten_shader_database(&self) -> &Option<LockedTypeObject /* super::render_base::EnlightenShaderDatabaseBaseAsset */> {
        &self.enlighten_shader_database
    }
    fn enlighten_shader_database_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::EnlightenShaderDatabaseBaseAsset */> {
        &mut self.enlighten_shader_database
    }
    fn ant_project_asset(&self) -> &Option<LockedTypeObject /* super::ant::AntProjectAsset */> {
        &self.ant_project_asset
    }
    fn ant_project_asset_mut(&mut self) -> &mut Option<LockedTypeObject /* super::ant::AntProjectAsset */> {
        &mut self.ant_project_asset
    }
    fn aerial_heightmap_data(&self) -> &String {
        &self.aerial_heightmap_data
    }
    fn aerial_heightmap_data_mut(&mut self) -> &mut String {
        &mut self.aerial_heightmap_data
    }
    fn free_streaming_enable(&self) -> &bool {
        &self.free_streaming_enable
    }
    fn free_streaming_enable_mut(&mut self) -> &mut bool {
        &mut self.free_streaming_enable
    }
    fn camera_modes(&self) -> &Vec<Option<LockedTypeObject /* CameraModeAsset */>> {
        &self.camera_modes
    }
    fn camera_modes_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* CameraModeAsset */>> {
        &mut self.camera_modes
    }
    fn camera_transitions(&self) -> &Vec<Option<LockedTypeObject /* CameraTransition */>> {
        &self.camera_transitions
    }
    fn camera_transitions_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* CameraTransition */>> {
        &mut self.camera_transitions
    }
    fn preload_info(&self) -> &LevelPreloadInfo {
        &self.preload_info
    }
    fn preload_info_mut(&mut self) -> &mut LevelPreloadInfo {
        &mut self.preload_info
    }
    fn face_animation_wave_mappings(&self) -> &Option<LockedTypeObject /* FaceAnimationWaveMappings */> {
        &self.face_animation_wave_mappings
    }
    fn face_animation_wave_mappings_mut(&mut self) -> &mut Option<LockedTypeObject /* FaceAnimationWaveMappings */> {
        &mut self.face_animation_wave_mappings
    }
    fn auto_load_bundles(&self) -> &Vec<String> {
        &self.auto_load_bundles
    }
    fn auto_load_bundles_mut(&mut self) -> &mut Vec<String> {
        &mut self.auto_load_bundles
    }
    fn unlock_id_table(&self) -> &UnlockIdTable {
        &self.unlock_id_table
    }
    fn unlock_id_table_mut(&mut self) -> &mut UnlockIdTable {
        &mut self.unlock_id_table
    }
    fn autoloaded_detached_sub_world_datas(&self) -> &Vec<Option<LockedTypeObject /* super::entity::SubWorldReferenceObjectData */>> {
        &self.autoloaded_detached_sub_world_datas
    }
    fn autoloaded_detached_sub_world_datas_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::SubWorldReferenceObjectData */>> {
        &mut self.autoloaded_detached_sub_world_datas
    }
}

impl super::entity::WorldDataTrait for LevelData {
    fn world_size_x_z(&self) -> &f32 {
        self._glacier_base.world_size_x_z()
    }
    fn world_size_x_z_mut(&mut self) -> &mut f32 {
        self._glacier_base.world_size_x_z_mut()
    }
    fn world_size_y(&self) -> &f32 {
        self._glacier_base.world_size_y()
    }
    fn world_size_y_mut(&mut self) -> &mut f32 {
        self._glacier_base.world_size_y_mut()
    }
    fn is_procedural_level(&self) -> &bool {
        self._glacier_base.is_procedural_level()
    }
    fn is_procedural_level_mut(&mut self) -> &mut bool {
        self._glacier_base.is_procedural_level_mut()
    }
    fn is_dedicated_server_level(&self) -> &bool {
        self._glacier_base.is_dedicated_server_level()
    }
    fn is_dedicated_server_level_mut(&mut self) -> &mut bool {
        self._glacier_base.is_dedicated_server_level_mut()
    }
}

impl super::entity::SubWorldDataTrait for LevelData {
    fn runtime_material_grid(&self) -> &Option<LockedTypeObject /* super::entity::MaterialGridData */> {
        self._glacier_base.runtime_material_grid()
    }
    fn runtime_material_grid_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::MaterialGridData */> {
        self._glacier_base.runtime_material_grid_mut()
    }
    fn is_win32_sub_level(&self) -> &bool {
        self._glacier_base.is_win32_sub_level()
    }
    fn is_win32_sub_level_mut(&mut self) -> &mut bool {
        self._glacier_base.is_win32_sub_level_mut()
    }
    fn is_gen4a_sub_level(&self) -> &bool {
        self._glacier_base.is_gen4a_sub_level()
    }
    fn is_gen4a_sub_level_mut(&mut self) -> &mut bool {
        self._glacier_base.is_gen4a_sub_level_mut()
    }
    fn is_gen4b_sub_level(&self) -> &bool {
        self._glacier_base.is_gen4b_sub_level()
    }
    fn is_gen4b_sub_level_mut(&mut self) -> &mut bool {
        self._glacier_base.is_gen4b_sub_level_mut()
    }
    fn is_android_sub_level(&self) -> &bool {
        self._glacier_base.is_android_sub_level()
    }
    fn is_android_sub_level_mut(&mut self) -> &mut bool {
        self._glacier_base.is_android_sub_level_mut()
    }
    fn is_i_o_s_sub_level(&self) -> &bool {
        self._glacier_base.is_i_o_s_sub_level()
    }
    fn is_i_o_s_sub_level_mut(&mut self) -> &mut bool {
        self._glacier_base.is_i_o_s_sub_level_mut()
    }
    fn is_o_s_x_sub_level(&self) -> &bool {
        self._glacier_base.is_o_s_x_sub_level()
    }
    fn is_o_s_x_sub_level_mut(&mut self) -> &mut bool {
        self._glacier_base.is_o_s_x_sub_level_mut()
    }
    fn is_linux_sub_level(&self) -> &bool {
        self._glacier_base.is_linux_sub_level()
    }
    fn is_linux_sub_level_mut(&mut self) -> &mut bool {
        self._glacier_base.is_linux_sub_level_mut()
    }
    fn persistence_setting(&self) -> &super::entity::BlueprintPersistenceSetting {
        self._glacier_base.persistence_setting()
    }
    fn persistence_setting_mut(&mut self) -> &mut super::entity::BlueprintPersistenceSetting {
        self._glacier_base.persistence_setting_mut()
    }
    fn auto_asset_collector(&self) -> &Option<LockedTypeObject /* super::entity::AutoAssetCollector */> {
        self._glacier_base.auto_asset_collector()
    }
    fn auto_asset_collector_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::AutoAssetCollector */> {
        self._glacier_base.auto_asset_collector_mut()
    }
    fn components(&self) -> &Vec<Option<LockedTypeObject /* super::entity::SubWorldDataComponent */>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::SubWorldDataComponent */>> {
        self._glacier_base.components_mut()
    }
}

impl super::entity::SpatialPrefabBlueprintTrait for LevelData {
}

impl super::entity::PrefabBlueprintTrait for LevelData {
    fn time_delta_type(&self) -> &super::entity::TimeDeltaType {
        self._glacier_base.time_delta_type()
    }
    fn time_delta_type_mut(&mut self) -> &mut super::entity::TimeDeltaType {
        self._glacier_base.time_delta_type_mut()
    }
}

impl super::entity::BlueprintTrait for LevelData {
    fn objects(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.objects()
    }
    fn objects_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.objects_mut()
    }
    fn schematics(&self) -> &Option<LockedTypeObject /* super::schematics::SchematicsBaseAsset */> {
        self._glacier_base.schematics()
    }
    fn schematics_mut(&mut self) -> &mut Option<LockedTypeObject /* super::schematics::SchematicsBaseAsset */> {
        self._glacier_base.schematics_mut()
    }
}

impl super::entity::EntityBusDataTrait for LevelData {
    fn event_connections(&self) -> &Vec<BoxedTypeObject /* super::entity::EventConnection */> {
        self._glacier_base.event_connections()
    }
    fn event_connections_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::entity::EventConnection */> {
        self._glacier_base.event_connections_mut()
    }
}

impl super::core::DataBusDataTrait for LevelData {
    fn flags(&self) -> &u16 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.flags_mut()
    }
    fn property_connections(&self) -> &Vec<BoxedTypeObject /* super::core::PropertyConnection */> {
        self._glacier_base.property_connections()
    }
    fn property_connections_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::core::PropertyConnection */> {
        self._glacier_base.property_connections_mut()
    }
    fn link_connections(&self) -> &Vec<BoxedTypeObject /* super::core::LinkConnection */> {
        self._glacier_base.link_connections()
    }
    fn link_connections_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::core::LinkConnection */> {
        self._glacier_base.link_connections_mut()
    }
    fn interface(&self) -> &Option<LockedTypeObject /* super::core::DynamicDataContainer */> {
        self._glacier_base.interface()
    }
    fn interface_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::DynamicDataContainer */> {
        self._glacier_base.interface_mut()
    }
}

impl super::core::AssetTrait for LevelData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for LevelData {
}

pub static LEVELDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelData",
    name_hash: 1598726915,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::WORLDDATA_TYPE_INFO),
        super_class_offset: offset_of!(LevelData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LevelData as Default>::default())),
            create_boxed: || Box::new(<LevelData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "AISystem",
                name_hash: 2765605864,
                flags: MemberInfoFlags::new(0),
                field_type: "GameAISystem",
                rust_offset: offset_of!(LevelData, a_i_system),
            },
            FieldInfoData {
                name: "AI2System",
                name_hash: 695504570,
                flags: MemberInfoFlags::new(0),
                field_type: "GameAISystem",
                rust_offset: offset_of!(LevelData, a_i2_system),
            },
            FieldInfoData {
                name: "GameConfigurationName",
                name_hash: 2876236092,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(LevelData, game_configuration_name),
            },
            FieldInfoData {
                name: "DefaultFOV",
                name_hash: 2143448529,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LevelData, default_f_o_v),
            },
            FieldInfoData {
                name: "InfantryFOVMultiplier",
                name_hash: 3538640096,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LevelData, infantry_f_o_v_multiplier),
            },
            FieldInfoData {
                name: "StreamPoolPreset",
                name_hash: 105139008,
                flags: MemberInfoFlags::new(0),
                field_type: "StreamPoolPreset",
                rust_offset: offset_of!(LevelData, stream_pool_preset),
            },
            FieldInfoData {
                name: "VoiceOverSystem",
                name_hash: 2563745432,
                flags: MemberInfoFlags::new(0),
                field_type: "VoiceOverSystemAsset",
                rust_offset: offset_of!(LevelData, voice_over_system),
            },
            FieldInfoData {
                name: "VoiceOverLogic",
                name_hash: 2947014771,
                flags: MemberInfoFlags::new(144),
                field_type: "VoiceOverLogicAsset-Array",
                rust_offset: offset_of!(LevelData, voice_over_logic),
            },
            FieldInfoData {
                name: "MaxVehicleHeight",
                name_hash: 3489491638,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LevelData, max_vehicle_height),
            },
            FieldInfoData {
                name: "HugeBroadPhase",
                name_hash: 428788623,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LevelData, huge_broad_phase),
            },
            FieldInfoData {
                name: "EnlightenShaderDatabase",
                name_hash: 3999247479,
                flags: MemberInfoFlags::new(0),
                field_type: "EnlightenShaderDatabaseBaseAsset",
                rust_offset: offset_of!(LevelData, enlighten_shader_database),
            },
            FieldInfoData {
                name: "AntProjectAsset",
                name_hash: 2771396091,
                flags: MemberInfoFlags::new(0),
                field_type: "AntProjectAsset",
                rust_offset: offset_of!(LevelData, ant_project_asset),
            },
            FieldInfoData {
                name: "AerialHeightmapData",
                name_hash: 716637124,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(LevelData, aerial_heightmap_data),
            },
            FieldInfoData {
                name: "FreeStreamingEnable",
                name_hash: 2148797772,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LevelData, free_streaming_enable),
            },
            FieldInfoData {
                name: "CameraModes",
                name_hash: 3182808108,
                flags: MemberInfoFlags::new(144),
                field_type: "CameraModeAsset-Array",
                rust_offset: offset_of!(LevelData, camera_modes),
            },
            FieldInfoData {
                name: "CameraTransitions",
                name_hash: 2235180064,
                flags: MemberInfoFlags::new(144),
                field_type: "CameraTransition-Array",
                rust_offset: offset_of!(LevelData, camera_transitions),
            },
            FieldInfoData {
                name: "PreloadInfo",
                name_hash: 3047520778,
                flags: MemberInfoFlags::new(0),
                field_type: "LevelPreloadInfo",
                rust_offset: offset_of!(LevelData, preload_info),
            },
            FieldInfoData {
                name: "FaceAnimationWaveMappings",
                name_hash: 2869249096,
                flags: MemberInfoFlags::new(0),
                field_type: "FaceAnimationWaveMappings",
                rust_offset: offset_of!(LevelData, face_animation_wave_mappings),
            },
            FieldInfoData {
                name: "AutoLoadBundles",
                name_hash: 3889825547,
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(LevelData, auto_load_bundles),
            },
            FieldInfoData {
                name: "UnlockIdTable",
                name_hash: 331243462,
                flags: MemberInfoFlags::new(0),
                field_type: "UnlockIdTable",
                rust_offset: offset_of!(LevelData, unlock_id_table),
            },
            FieldInfoData {
                name: "AutoloadedDetachedSubWorldDatas",
                name_hash: 1287521110,
                flags: MemberInfoFlags::new(144),
                field_type: "SubWorldReferenceObjectData-Array",
                rust_offset: offset_of!(LevelData, autoloaded_detached_sub_world_datas),
            },
        ],
    }),
    array_type: Some(LEVELDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LevelData {
    fn type_info(&self) -> &'static TypeInfo {
        LEVELDATA_TYPE_INFO
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


pub static LEVELDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelData-Array",
    name_hash: 1041946039,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("LevelData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct UnlockIdTable {
    pub identifiers: Vec<u32>,
}

pub trait UnlockIdTableTrait: TypeObject {
    fn identifiers(&self) -> &Vec<u32>;
    fn identifiers_mut(&mut self) -> &mut Vec<u32>;
}

impl UnlockIdTableTrait for UnlockIdTable {
    fn identifiers(&self) -> &Vec<u32> {
        &self.identifiers
    }
    fn identifiers_mut(&mut self) -> &mut Vec<u32> {
        &mut self.identifiers
    }
}

pub static UNLOCKIDTABLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UnlockIdTable",
    name_hash: 331243462,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UnlockIdTable as Default>::default())),
            create_boxed: || Box::new(<UnlockIdTable as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Identifiers",
                name_hash: 4252931701,
                flags: MemberInfoFlags::new(144),
                field_type: "Uint32-Array",
                rust_offset: offset_of!(UnlockIdTable, identifiers),
            },
        ],
    }),
    array_type: Some(UNLOCKIDTABLE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UnlockIdTable {
    fn type_info(&self) -> &'static TypeInfo {
        UNLOCKIDTABLE_TYPE_INFO
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


pub static UNLOCKIDTABLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UnlockIdTable-Array",
    name_hash: 726793586,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("UnlockIdTable"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LevelDataComponent {
    pub _glacier_base: super::core::DataContainer,
}

pub trait LevelDataComponentTrait: super::core::DataContainerTrait {
}

impl LevelDataComponentTrait for LevelDataComponent {
}

impl super::core::DataContainerTrait for LevelDataComponent {
}

pub static LEVELDATACOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelDataComponent",
    name_hash: 2604199756,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(LevelDataComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LevelDataComponent as Default>::default())),
            create_boxed: || Box::new(<LevelDataComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(LEVELDATACOMPONENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LevelDataComponent {
    fn type_info(&self) -> &'static TypeInfo {
        LEVELDATACOMPONENT_TYPE_INFO
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


pub static LEVELDATACOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelDataComponent-Array",
    name_hash: 3550547960,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("LevelDataComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LevelPreloadInfo {
    pub preloaded_blueprint_bundles: Vec<String>,
    pub sub_level_preload_info_map: Vec<BoxedTypeObject /* SubLevelPreloadInfo */>,
}

pub trait LevelPreloadInfoTrait: TypeObject {
    fn preloaded_blueprint_bundles(&self) -> &Vec<String>;
    fn preloaded_blueprint_bundles_mut(&mut self) -> &mut Vec<String>;
    fn sub_level_preload_info_map(&self) -> &Vec<BoxedTypeObject /* SubLevelPreloadInfo */>;
    fn sub_level_preload_info_map_mut(&mut self) -> &mut Vec<BoxedTypeObject /* SubLevelPreloadInfo */>;
}

impl LevelPreloadInfoTrait for LevelPreloadInfo {
    fn preloaded_blueprint_bundles(&self) -> &Vec<String> {
        &self.preloaded_blueprint_bundles
    }
    fn preloaded_blueprint_bundles_mut(&mut self) -> &mut Vec<String> {
        &mut self.preloaded_blueprint_bundles
    }
    fn sub_level_preload_info_map(&self) -> &Vec<BoxedTypeObject /* SubLevelPreloadInfo */> {
        &self.sub_level_preload_info_map
    }
    fn sub_level_preload_info_map_mut(&mut self) -> &mut Vec<BoxedTypeObject /* SubLevelPreloadInfo */> {
        &mut self.sub_level_preload_info_map
    }
}

pub static LEVELPRELOADINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelPreloadInfo",
    name_hash: 2508566268,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LevelPreloadInfo as Default>::default())),
            create_boxed: || Box::new(<LevelPreloadInfo as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "PreloadedBlueprintBundles",
                name_hash: 4096622189,
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(LevelPreloadInfo, preloaded_blueprint_bundles),
            },
            FieldInfoData {
                name: "SubLevelPreloadInfoMap",
                name_hash: 2880457604,
                flags: MemberInfoFlags::new(144),
                field_type: "SubLevelPreloadInfo-Array",
                rust_offset: offset_of!(LevelPreloadInfo, sub_level_preload_info_map),
            },
        ],
    }),
    array_type: Some(LEVELPRELOADINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LevelPreloadInfo {
    fn type_info(&self) -> &'static TypeInfo {
        LEVELPRELOADINFO_TYPE_INFO
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


pub static LEVELPRELOADINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelPreloadInfo-Array",
    name_hash: 1477023432,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("LevelPreloadInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SubLevelPreloadInfo {
    pub sub_level_bundle_path: String,
    pub preloaded_blueprint_bundles: Vec<String>,
}

pub trait SubLevelPreloadInfoTrait: TypeObject {
    fn sub_level_bundle_path(&self) -> &String;
    fn sub_level_bundle_path_mut(&mut self) -> &mut String;
    fn preloaded_blueprint_bundles(&self) -> &Vec<String>;
    fn preloaded_blueprint_bundles_mut(&mut self) -> &mut Vec<String>;
}

impl SubLevelPreloadInfoTrait for SubLevelPreloadInfo {
    fn sub_level_bundle_path(&self) -> &String {
        &self.sub_level_bundle_path
    }
    fn sub_level_bundle_path_mut(&mut self) -> &mut String {
        &mut self.sub_level_bundle_path
    }
    fn preloaded_blueprint_bundles(&self) -> &Vec<String> {
        &self.preloaded_blueprint_bundles
    }
    fn preloaded_blueprint_bundles_mut(&mut self) -> &mut Vec<String> {
        &mut self.preloaded_blueprint_bundles
    }
}

pub static SUBLEVELPRELOADINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelPreloadInfo",
    name_hash: 3473270584,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SubLevelPreloadInfo as Default>::default())),
            create_boxed: || Box::new(<SubLevelPreloadInfo as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "SubLevelBundlePath",
                name_hash: 666771118,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(SubLevelPreloadInfo, sub_level_bundle_path),
            },
            FieldInfoData {
                name: "PreloadedBlueprintBundles",
                name_hash: 4096622189,
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(SubLevelPreloadInfo, preloaded_blueprint_bundles),
            },
        ],
    }),
    array_type: Some(SUBLEVELPRELOADINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SubLevelPreloadInfo {
    fn type_info(&self) -> &'static TypeInfo {
        SUBLEVELPRELOADINFO_TYPE_INFO
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


pub static SUBLEVELPRELOADINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelPreloadInfo-Array",
    name_hash: 2498187404,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("SubLevelPreloadInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CameraTransition {
    pub _glacier_base: super::core::DataContainer,
}

pub trait CameraTransitionTrait: super::core::DataContainerTrait {
}

impl CameraTransitionTrait for CameraTransition {
}

impl super::core::DataContainerTrait for CameraTransition {
}

pub static CAMERATRANSITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraTransition",
    name_hash: 328033779,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(CameraTransition, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CameraTransition as Default>::default())),
            create_boxed: || Box::new(<CameraTransition as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CAMERATRANSITION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CameraTransition {
    fn type_info(&self) -> &'static TypeInfo {
        CAMERATRANSITION_TYPE_INFO
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


pub static CAMERATRANSITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraTransition-Array",
    name_hash: 4254548423,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("CameraTransition"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CameraModeAsset {
    pub _glacier_base: super::core::Asset,
}

pub trait CameraModeAssetTrait: super::core::AssetTrait {
}

impl CameraModeAssetTrait for CameraModeAsset {
}

impl super::core::AssetTrait for CameraModeAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for CameraModeAsset {
}

pub static CAMERAMODEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraModeAsset",
    name_hash: 4257185679,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(CameraModeAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CameraModeAsset as Default>::default())),
            create_boxed: || Box::new(<CameraModeAsset as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CAMERAMODEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CameraModeAsset {
    fn type_info(&self) -> &'static TypeInfo {
        CAMERAMODEASSET_TYPE_INFO
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


pub static CAMERAMODEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraModeAsset-Array",
    name_hash: 3500284987,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("CameraModeAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct GameAISystem {
    pub _glacier_base: super::core::Asset,
    pub excluded_game_mode_list: Vec<String>,
}

pub trait GameAISystemTrait: super::core::AssetTrait {
    fn excluded_game_mode_list(&self) -> &Vec<String>;
    fn excluded_game_mode_list_mut(&mut self) -> &mut Vec<String>;
}

impl GameAISystemTrait for GameAISystem {
    fn excluded_game_mode_list(&self) -> &Vec<String> {
        &self.excluded_game_mode_list
    }
    fn excluded_game_mode_list_mut(&mut self) -> &mut Vec<String> {
        &mut self.excluded_game_mode_list
    }
}

impl super::core::AssetTrait for GameAISystem {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for GameAISystem {
}

pub static GAMEAISYSTEM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameAISystem",
    name_hash: 1729239942,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(GameAISystem, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GameAISystem as Default>::default())),
            create_boxed: || Box::new(<GameAISystem as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ExcludedGameModeList",
                name_hash: 3262631112,
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(GameAISystem, excluded_game_mode_list),
            },
        ],
    }),
    array_type: Some(GAMEAISYSTEM_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GameAISystem {
    fn type_info(&self) -> &'static TypeInfo {
        GAMEAISYSTEM_TYPE_INFO
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


pub static GAMEAISYSTEM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameAISystem-Array",
    name_hash: 3986901170,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("GameAISystem"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct FaceAnimationWaveMappingsAsset {
    pub _glacier_base: super::core::Asset,
    pub mappings: Vec<Option<LockedTypeObject /* FaceAnimationWaveMappings */>>,
}

pub trait FaceAnimationWaveMappingsAssetTrait: super::core::AssetTrait {
    fn mappings(&self) -> &Vec<Option<LockedTypeObject /* FaceAnimationWaveMappings */>>;
    fn mappings_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* FaceAnimationWaveMappings */>>;
}

impl FaceAnimationWaveMappingsAssetTrait for FaceAnimationWaveMappingsAsset {
    fn mappings(&self) -> &Vec<Option<LockedTypeObject /* FaceAnimationWaveMappings */>> {
        &self.mappings
    }
    fn mappings_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* FaceAnimationWaveMappings */>> {
        &mut self.mappings
    }
}

impl super::core::AssetTrait for FaceAnimationWaveMappingsAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for FaceAnimationWaveMappingsAsset {
}

pub static FACEANIMATIONWAVEMAPPINGSASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FaceAnimationWaveMappingsAsset",
    name_hash: 2559272088,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(FaceAnimationWaveMappingsAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FaceAnimationWaveMappingsAsset as Default>::default())),
            create_boxed: || Box::new(<FaceAnimationWaveMappingsAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Mappings",
                name_hash: 673881690,
                flags: MemberInfoFlags::new(144),
                field_type: "FaceAnimationWaveMappings-Array",
                rust_offset: offset_of!(FaceAnimationWaveMappingsAsset, mappings),
            },
        ],
    }),
    array_type: Some(FACEANIMATIONWAVEMAPPINGSASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FaceAnimationWaveMappingsAsset {
    fn type_info(&self) -> &'static TypeInfo {
        FACEANIMATIONWAVEMAPPINGSASSET_TYPE_INFO
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


pub static FACEANIMATIONWAVEMAPPINGSASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FaceAnimationWaveMappingsAsset-Array",
    name_hash: 1213566636,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("FaceAnimationWaveMappingsAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct FaceAnimationWaveMappings {
    pub _glacier_base: super::core::DataContainer,
    pub mappings: Vec<BoxedTypeObject /* FaceAnimationWaveMapping */>,
}

pub trait FaceAnimationWaveMappingsTrait: super::core::DataContainerTrait {
    fn mappings(&self) -> &Vec<BoxedTypeObject /* FaceAnimationWaveMapping */>;
    fn mappings_mut(&mut self) -> &mut Vec<BoxedTypeObject /* FaceAnimationWaveMapping */>;
}

impl FaceAnimationWaveMappingsTrait for FaceAnimationWaveMappings {
    fn mappings(&self) -> &Vec<BoxedTypeObject /* FaceAnimationWaveMapping */> {
        &self.mappings
    }
    fn mappings_mut(&mut self) -> &mut Vec<BoxedTypeObject /* FaceAnimationWaveMapping */> {
        &mut self.mappings
    }
}

impl super::core::DataContainerTrait for FaceAnimationWaveMappings {
}

pub static FACEANIMATIONWAVEMAPPINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FaceAnimationWaveMappings",
    name_hash: 2869249096,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(FaceAnimationWaveMappings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FaceAnimationWaveMappings as Default>::default())),
            create_boxed: || Box::new(<FaceAnimationWaveMappings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Mappings",
                name_hash: 673881690,
                flags: MemberInfoFlags::new(144),
                field_type: "FaceAnimationWaveMapping-Array",
                rust_offset: offset_of!(FaceAnimationWaveMappings, mappings),
            },
        ],
    }),
    array_type: Some(FACEANIMATIONWAVEMAPPINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FaceAnimationWaveMappings {
    fn type_info(&self) -> &'static TypeInfo {
        FACEANIMATIONWAVEMAPPINGS_TYPE_INFO
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


pub static FACEANIMATIONWAVEMAPPINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FaceAnimationWaveMappings-Array",
    name_hash: 2170124028,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("FaceAnimationWaveMappings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct FaceAnimationWaveMapping {
    pub wave_name_hash: i32,
    pub wave_variation_index: u16,
    pub facial_animation: super::ant::AntRef,
}

pub trait FaceAnimationWaveMappingTrait: TypeObject {
    fn wave_name_hash(&self) -> &i32;
    fn wave_name_hash_mut(&mut self) -> &mut i32;
    fn wave_variation_index(&self) -> &u16;
    fn wave_variation_index_mut(&mut self) -> &mut u16;
    fn facial_animation(&self) -> &super::ant::AntRef;
    fn facial_animation_mut(&mut self) -> &mut super::ant::AntRef;
}

impl FaceAnimationWaveMappingTrait for FaceAnimationWaveMapping {
    fn wave_name_hash(&self) -> &i32 {
        &self.wave_name_hash
    }
    fn wave_name_hash_mut(&mut self) -> &mut i32 {
        &mut self.wave_name_hash
    }
    fn wave_variation_index(&self) -> &u16 {
        &self.wave_variation_index
    }
    fn wave_variation_index_mut(&mut self) -> &mut u16 {
        &mut self.wave_variation_index
    }
    fn facial_animation(&self) -> &super::ant::AntRef {
        &self.facial_animation
    }
    fn facial_animation_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.facial_animation
    }
}

pub static FACEANIMATIONWAVEMAPPING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FaceAnimationWaveMapping",
    name_hash: 1258301659,
    flags: MemberInfoFlags::new(32841),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FaceAnimationWaveMapping as Default>::default())),
            create_boxed: || Box::new(<FaceAnimationWaveMapping as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "WaveNameHash",
                name_hash: 1305133845,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(FaceAnimationWaveMapping, wave_name_hash),
            },
            FieldInfoData {
                name: "WaveVariationIndex",
                name_hash: 3976155887,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(FaceAnimationWaveMapping, wave_variation_index),
            },
            FieldInfoData {
                name: "FacialAnimation",
                name_hash: 2847295635,
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(FaceAnimationWaveMapping, facial_animation),
            },
        ],
    }),
    array_type: Some(FACEANIMATIONWAVEMAPPING_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for FaceAnimationWaveMapping {
    fn type_info(&self) -> &'static TypeInfo {
        FACEANIMATIONWAVEMAPPING_TYPE_INFO
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


pub static FACEANIMATIONWAVEMAPPING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FaceAnimationWaveMapping-Array",
    name_hash: 596611055,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("FaceAnimationWaveMapping"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct BigWorldSettingsAsset {
    pub _glacier_base: super::core::Asset,
    pub settings: Vec<Option<LockedTypeObject /* BigWorldSetting */>>,
}

pub trait BigWorldSettingsAssetTrait: super::core::AssetTrait {
    fn settings(&self) -> &Vec<Option<LockedTypeObject /* BigWorldSetting */>>;
    fn settings_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* BigWorldSetting */>>;
}

impl BigWorldSettingsAssetTrait for BigWorldSettingsAsset {
    fn settings(&self) -> &Vec<Option<LockedTypeObject /* BigWorldSetting */>> {
        &self.settings
    }
    fn settings_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* BigWorldSetting */>> {
        &mut self.settings
    }
}

impl super::core::AssetTrait for BigWorldSettingsAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for BigWorldSettingsAsset {
}

pub static BIGWORLDSETTINGSASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BigWorldSettingsAsset",
    name_hash: 205167518,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(BigWorldSettingsAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BigWorldSettingsAsset as Default>::default())),
            create_boxed: || Box::new(<BigWorldSettingsAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Settings",
                name_hash: 649772672,
                flags: MemberInfoFlags::new(144),
                field_type: "BigWorldSetting-Array",
                rust_offset: offset_of!(BigWorldSettingsAsset, settings),
            },
        ],
    }),
    array_type: Some(BIGWORLDSETTINGSASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BigWorldSettingsAsset {
    fn type_info(&self) -> &'static TypeInfo {
        BIGWORLDSETTINGSASSET_TYPE_INFO
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


pub static BIGWORLDSETTINGSASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BigWorldSettingsAsset-Array",
    name_hash: 20835498,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("BigWorldSettingsAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct BigWorldSetting {
    pub _glacier_base: super::core::DataContainer,
    pub sound: Option<LockedTypeObject /* super::audio::SoundAsset */>,
    pub min_distance: i32,
    pub max_distance: i32,
    pub min_delay_time_in_minutes: f32,
    pub max_delay_time_in_minutes: f32,
}

pub trait BigWorldSettingTrait: super::core::DataContainerTrait {
    fn sound(&self) -> &Option<LockedTypeObject /* super::audio::SoundAsset */>;
    fn sound_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundAsset */>;
    fn min_distance(&self) -> &i32;
    fn min_distance_mut(&mut self) -> &mut i32;
    fn max_distance(&self) -> &i32;
    fn max_distance_mut(&mut self) -> &mut i32;
    fn min_delay_time_in_minutes(&self) -> &f32;
    fn min_delay_time_in_minutes_mut(&mut self) -> &mut f32;
    fn max_delay_time_in_minutes(&self) -> &f32;
    fn max_delay_time_in_minutes_mut(&mut self) -> &mut f32;
}

impl BigWorldSettingTrait for BigWorldSetting {
    fn sound(&self) -> &Option<LockedTypeObject /* super::audio::SoundAsset */> {
        &self.sound
    }
    fn sound_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundAsset */> {
        &mut self.sound
    }
    fn min_distance(&self) -> &i32 {
        &self.min_distance
    }
    fn min_distance_mut(&mut self) -> &mut i32 {
        &mut self.min_distance
    }
    fn max_distance(&self) -> &i32 {
        &self.max_distance
    }
    fn max_distance_mut(&mut self) -> &mut i32 {
        &mut self.max_distance
    }
    fn min_delay_time_in_minutes(&self) -> &f32 {
        &self.min_delay_time_in_minutes
    }
    fn min_delay_time_in_minutes_mut(&mut self) -> &mut f32 {
        &mut self.min_delay_time_in_minutes
    }
    fn max_delay_time_in_minutes(&self) -> &f32 {
        &self.max_delay_time_in_minutes
    }
    fn max_delay_time_in_minutes_mut(&mut self) -> &mut f32 {
        &mut self.max_delay_time_in_minutes
    }
}

impl super::core::DataContainerTrait for BigWorldSetting {
}

pub static BIGWORLDSETTING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BigWorldSetting",
    name_hash: 2009102045,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(BigWorldSetting, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BigWorldSetting as Default>::default())),
            create_boxed: || Box::new(<BigWorldSetting as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Sound",
                name_hash: 231353798,
                flags: MemberInfoFlags::new(0),
                field_type: "SoundAsset",
                rust_offset: offset_of!(BigWorldSetting, sound),
            },
            FieldInfoData {
                name: "MinDistance",
                name_hash: 1885855628,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(BigWorldSetting, min_distance),
            },
            FieldInfoData {
                name: "MaxDistance",
                name_hash: 3520454034,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(BigWorldSetting, max_distance),
            },
            FieldInfoData {
                name: "MinDelayTimeInMinutes",
                name_hash: 381594133,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BigWorldSetting, min_delay_time_in_minutes),
            },
            FieldInfoData {
                name: "MaxDelayTimeInMinutes",
                name_hash: 741272203,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BigWorldSetting, max_delay_time_in_minutes),
            },
        ],
    }),
    array_type: Some(BIGWORLDSETTING_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BigWorldSetting {
    fn type_info(&self) -> &'static TypeInfo {
        BIGWORLDSETTING_TYPE_INFO
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


pub static BIGWORLDSETTING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BigWorldSetting-Array",
    name_hash: 2791690985,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("BigWorldSetting"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LevelReportingAsset {
    pub _glacier_base: super::core::Asset,
    pub built_levels: Vec<glacier_util::guid::Guid>,
}

pub trait LevelReportingAssetTrait: super::core::AssetTrait {
    fn built_levels(&self) -> &Vec<glacier_util::guid::Guid>;
    fn built_levels_mut(&mut self) -> &mut Vec<glacier_util::guid::Guid>;
}

impl LevelReportingAssetTrait for LevelReportingAsset {
    fn built_levels(&self) -> &Vec<glacier_util::guid::Guid> {
        &self.built_levels
    }
    fn built_levels_mut(&mut self) -> &mut Vec<glacier_util::guid::Guid> {
        &mut self.built_levels
    }
}

impl super::core::AssetTrait for LevelReportingAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for LevelReportingAsset {
}

pub static LEVELREPORTINGASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelReportingAsset",
    name_hash: 3639474797,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(LevelReportingAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LevelReportingAsset as Default>::default())),
            create_boxed: || Box::new(<LevelReportingAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "BuiltLevels",
                name_hash: 3293235878,
                flags: MemberInfoFlags::new(144),
                field_type: "Guid-Array",
                rust_offset: offset_of!(LevelReportingAsset, built_levels),
            },
        ],
    }),
    array_type: Some(LEVELREPORTINGASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LevelReportingAsset {
    fn type_info(&self) -> &'static TypeInfo {
        LEVELREPORTINGASSET_TYPE_INFO
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


pub static LEVELREPORTINGASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelReportingAsset-Array",
    name_hash: 1840594777,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("LevelReportingAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct HudData {
    pub _glacier_base: super::core::DataContainer,
}

pub trait HudDataTrait: super::core::DataContainerTrait {
}

impl HudDataTrait for HudData {
}

impl super::core::DataContainerTrait for HudData {
}

pub static HUDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HudData",
    name_hash: 2639805868,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(HudData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<HudData as Default>::default())),
            create_boxed: || Box::new(<HudData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(HUDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for HudData {
    fn type_info(&self) -> &'static TypeInfo {
        HUDDATA_TYPE_INFO
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


pub static HUDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HudData-Array",
    name_hash: 938306840,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("HudData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct HIKData {
    pub reach_t: f32,
    pub reach_r: f32,
    pub pull: f32,
    pub resist: f32,
}

pub trait HIKDataTrait: TypeObject {
    fn reach_t(&self) -> &f32;
    fn reach_t_mut(&mut self) -> &mut f32;
    fn reach_r(&self) -> &f32;
    fn reach_r_mut(&mut self) -> &mut f32;
    fn pull(&self) -> &f32;
    fn pull_mut(&mut self) -> &mut f32;
    fn resist(&self) -> &f32;
    fn resist_mut(&mut self) -> &mut f32;
}

impl HIKDataTrait for HIKData {
    fn reach_t(&self) -> &f32 {
        &self.reach_t
    }
    fn reach_t_mut(&mut self) -> &mut f32 {
        &mut self.reach_t
    }
    fn reach_r(&self) -> &f32 {
        &self.reach_r
    }
    fn reach_r_mut(&mut self) -> &mut f32 {
        &mut self.reach_r
    }
    fn pull(&self) -> &f32 {
        &self.pull
    }
    fn pull_mut(&mut self) -> &mut f32 {
        &mut self.pull
    }
    fn resist(&self) -> &f32 {
        &self.resist
    }
    fn resist_mut(&mut self) -> &mut f32 {
        &mut self.resist
    }
}

pub static HIKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HIKData",
    name_hash: 646556927,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<HIKData as Default>::default())),
            create_boxed: || Box::new(<HIKData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ReachT",
                name_hash: 3293760940,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HIKData, reach_t),
            },
            FieldInfoData {
                name: "ReachR",
                name_hash: 3293760938,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HIKData, reach_r),
            },
            FieldInfoData {
                name: "Pull",
                name_hash: 2089427552,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HIKData, pull),
            },
            FieldInfoData {
                name: "Resist",
                name_hash: 3293260655,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HIKData, resist),
            },
        ],
    }),
    array_type: Some(HIKDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for HIKData {
    fn type_info(&self) -> &'static TypeInfo {
        HIKDATA_TYPE_INFO
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


pub static HIKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HIKData-Array",
    name_hash: 673960651,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("HIKData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct GameAnimationSettings {
    pub _glacier_base: super::core::SystemSettings,
    pub ant_on_client_only_gamemodes: Vec<String>,
    pub server_enable: bool,
    pub client_enable: bool,
}

pub trait GameAnimationSettingsTrait: super::core::SystemSettingsTrait {
    fn ant_on_client_only_gamemodes(&self) -> &Vec<String>;
    fn ant_on_client_only_gamemodes_mut(&mut self) -> &mut Vec<String>;
    fn server_enable(&self) -> &bool;
    fn server_enable_mut(&mut self) -> &mut bool;
    fn client_enable(&self) -> &bool;
    fn client_enable_mut(&mut self) -> &mut bool;
}

impl GameAnimationSettingsTrait for GameAnimationSettings {
    fn ant_on_client_only_gamemodes(&self) -> &Vec<String> {
        &self.ant_on_client_only_gamemodes
    }
    fn ant_on_client_only_gamemodes_mut(&mut self) -> &mut Vec<String> {
        &mut self.ant_on_client_only_gamemodes
    }
    fn server_enable(&self) -> &bool {
        &self.server_enable
    }
    fn server_enable_mut(&mut self) -> &mut bool {
        &mut self.server_enable
    }
    fn client_enable(&self) -> &bool {
        &self.client_enable
    }
    fn client_enable_mut(&mut self) -> &mut bool {
        &mut self.client_enable
    }
}

impl super::core::SystemSettingsTrait for GameAnimationSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl super::core::DataContainerTrait for GameAnimationSettings {
}

pub static GAMEANIMATIONSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameAnimationSettings",
    name_hash: 2429920664,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        super_class_offset: offset_of!(GameAnimationSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GameAnimationSettings as Default>::default())),
            create_boxed: || Box::new(<GameAnimationSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "AntOnClientOnlyGamemodes",
                name_hash: 3192598540,
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(GameAnimationSettings, ant_on_client_only_gamemodes),
            },
            FieldInfoData {
                name: "ServerEnable",
                name_hash: 3875294337,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GameAnimationSettings, server_enable),
            },
            FieldInfoData {
                name: "ClientEnable",
                name_hash: 1069919325,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GameAnimationSettings, client_enable),
            },
        ],
    }),
    array_type: Some(GAMEANIMATIONSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GameAnimationSettings {
    fn type_info(&self) -> &'static TypeInfo {
        GAMEANIMATIONSETTINGS_TYPE_INFO
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


pub static GAMEANIMATIONSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameAnimationSettings-Array",
    name_hash: 1502384044,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("GameAnimationSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DemoSettings {
    pub _glacier_base: super::core::SystemSettings,
    pub record_demo_file_name: String,
    pub playback_demo_file_name: String,
    pub time_demo: String,
    pub lock_to_player_name: String,
    pub change_player_interval: u32,
    pub forced_delta_tick_count: u32,
    pub start_profiling_on_frame: u32,
    pub stop_profiling_on_frame: u32,
    pub pause_on_startup: bool,
    pub allow_overwrite: bool,
    pub log_performance: bool,
    pub suppress_debug_log: bool,
    pub shutdown_on_demo_complete: bool,
    pub looping_demo: bool,
    pub lock_to_random_player: bool,
    pub take_screenshot_on_frame: u32,
}

pub trait DemoSettingsTrait: super::core::SystemSettingsTrait {
    fn record_demo_file_name(&self) -> &String;
    fn record_demo_file_name_mut(&mut self) -> &mut String;
    fn playback_demo_file_name(&self) -> &String;
    fn playback_demo_file_name_mut(&mut self) -> &mut String;
    fn time_demo(&self) -> &String;
    fn time_demo_mut(&mut self) -> &mut String;
    fn lock_to_player_name(&self) -> &String;
    fn lock_to_player_name_mut(&mut self) -> &mut String;
    fn change_player_interval(&self) -> &u32;
    fn change_player_interval_mut(&mut self) -> &mut u32;
    fn forced_delta_tick_count(&self) -> &u32;
    fn forced_delta_tick_count_mut(&mut self) -> &mut u32;
    fn start_profiling_on_frame(&self) -> &u32;
    fn start_profiling_on_frame_mut(&mut self) -> &mut u32;
    fn stop_profiling_on_frame(&self) -> &u32;
    fn stop_profiling_on_frame_mut(&mut self) -> &mut u32;
    fn pause_on_startup(&self) -> &bool;
    fn pause_on_startup_mut(&mut self) -> &mut bool;
    fn allow_overwrite(&self) -> &bool;
    fn allow_overwrite_mut(&mut self) -> &mut bool;
    fn log_performance(&self) -> &bool;
    fn log_performance_mut(&mut self) -> &mut bool;
    fn suppress_debug_log(&self) -> &bool;
    fn suppress_debug_log_mut(&mut self) -> &mut bool;
    fn shutdown_on_demo_complete(&self) -> &bool;
    fn shutdown_on_demo_complete_mut(&mut self) -> &mut bool;
    fn looping_demo(&self) -> &bool;
    fn looping_demo_mut(&mut self) -> &mut bool;
    fn lock_to_random_player(&self) -> &bool;
    fn lock_to_random_player_mut(&mut self) -> &mut bool;
    fn take_screenshot_on_frame(&self) -> &u32;
    fn take_screenshot_on_frame_mut(&mut self) -> &mut u32;
}

impl DemoSettingsTrait for DemoSettings {
    fn record_demo_file_name(&self) -> &String {
        &self.record_demo_file_name
    }
    fn record_demo_file_name_mut(&mut self) -> &mut String {
        &mut self.record_demo_file_name
    }
    fn playback_demo_file_name(&self) -> &String {
        &self.playback_demo_file_name
    }
    fn playback_demo_file_name_mut(&mut self) -> &mut String {
        &mut self.playback_demo_file_name
    }
    fn time_demo(&self) -> &String {
        &self.time_demo
    }
    fn time_demo_mut(&mut self) -> &mut String {
        &mut self.time_demo
    }
    fn lock_to_player_name(&self) -> &String {
        &self.lock_to_player_name
    }
    fn lock_to_player_name_mut(&mut self) -> &mut String {
        &mut self.lock_to_player_name
    }
    fn change_player_interval(&self) -> &u32 {
        &self.change_player_interval
    }
    fn change_player_interval_mut(&mut self) -> &mut u32 {
        &mut self.change_player_interval
    }
    fn forced_delta_tick_count(&self) -> &u32 {
        &self.forced_delta_tick_count
    }
    fn forced_delta_tick_count_mut(&mut self) -> &mut u32 {
        &mut self.forced_delta_tick_count
    }
    fn start_profiling_on_frame(&self) -> &u32 {
        &self.start_profiling_on_frame
    }
    fn start_profiling_on_frame_mut(&mut self) -> &mut u32 {
        &mut self.start_profiling_on_frame
    }
    fn stop_profiling_on_frame(&self) -> &u32 {
        &self.stop_profiling_on_frame
    }
    fn stop_profiling_on_frame_mut(&mut self) -> &mut u32 {
        &mut self.stop_profiling_on_frame
    }
    fn pause_on_startup(&self) -> &bool {
        &self.pause_on_startup
    }
    fn pause_on_startup_mut(&mut self) -> &mut bool {
        &mut self.pause_on_startup
    }
    fn allow_overwrite(&self) -> &bool {
        &self.allow_overwrite
    }
    fn allow_overwrite_mut(&mut self) -> &mut bool {
        &mut self.allow_overwrite
    }
    fn log_performance(&self) -> &bool {
        &self.log_performance
    }
    fn log_performance_mut(&mut self) -> &mut bool {
        &mut self.log_performance
    }
    fn suppress_debug_log(&self) -> &bool {
        &self.suppress_debug_log
    }
    fn suppress_debug_log_mut(&mut self) -> &mut bool {
        &mut self.suppress_debug_log
    }
    fn shutdown_on_demo_complete(&self) -> &bool {
        &self.shutdown_on_demo_complete
    }
    fn shutdown_on_demo_complete_mut(&mut self) -> &mut bool {
        &mut self.shutdown_on_demo_complete
    }
    fn looping_demo(&self) -> &bool {
        &self.looping_demo
    }
    fn looping_demo_mut(&mut self) -> &mut bool {
        &mut self.looping_demo
    }
    fn lock_to_random_player(&self) -> &bool {
        &self.lock_to_random_player
    }
    fn lock_to_random_player_mut(&mut self) -> &mut bool {
        &mut self.lock_to_random_player
    }
    fn take_screenshot_on_frame(&self) -> &u32 {
        &self.take_screenshot_on_frame
    }
    fn take_screenshot_on_frame_mut(&mut self) -> &mut u32 {
        &mut self.take_screenshot_on_frame
    }
}

impl super::core::SystemSettingsTrait for DemoSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl super::core::DataContainerTrait for DemoSettings {
}

pub static DEMOSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DemoSettings",
    name_hash: 1857099843,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        super_class_offset: offset_of!(DemoSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DemoSettings as Default>::default())),
            create_boxed: || Box::new(<DemoSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "RecordDemoFileName",
                name_hash: 3106827306,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(DemoSettings, record_demo_file_name),
            },
            FieldInfoData {
                name: "PlaybackDemoFileName",
                name_hash: 2477555912,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(DemoSettings, playback_demo_file_name),
            },
            FieldInfoData {
                name: "TimeDemo",
                name_hash: 2999418899,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(DemoSettings, time_demo),
            },
            FieldInfoData {
                name: "LockToPlayerName",
                name_hash: 1399694977,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(DemoSettings, lock_to_player_name),
            },
            FieldInfoData {
                name: "ChangePlayerInterval",
                name_hash: 4244661775,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DemoSettings, change_player_interval),
            },
            FieldInfoData {
                name: "ForcedDeltaTickCount",
                name_hash: 2341640882,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DemoSettings, forced_delta_tick_count),
            },
            FieldInfoData {
                name: "StartProfilingOnFrame",
                name_hash: 37151863,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DemoSettings, start_profiling_on_frame),
            },
            FieldInfoData {
                name: "StopProfilingOnFrame",
                name_hash: 3237723695,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DemoSettings, stop_profiling_on_frame),
            },
            FieldInfoData {
                name: "PauseOnStartup",
                name_hash: 1614327443,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DemoSettings, pause_on_startup),
            },
            FieldInfoData {
                name: "AllowOverwrite",
                name_hash: 1707043087,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DemoSettings, allow_overwrite),
            },
            FieldInfoData {
                name: "LogPerformance",
                name_hash: 2026456601,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DemoSettings, log_performance),
            },
            FieldInfoData {
                name: "SuppressDebugLog",
                name_hash: 1295919777,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DemoSettings, suppress_debug_log),
            },
            FieldInfoData {
                name: "ShutdownOnDemoComplete",
                name_hash: 296483878,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DemoSettings, shutdown_on_demo_complete),
            },
            FieldInfoData {
                name: "LoopingDemo",
                name_hash: 2714976858,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DemoSettings, looping_demo),
            },
            FieldInfoData {
                name: "LockToRandomPlayer",
                name_hash: 4183020829,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DemoSettings, lock_to_random_player),
            },
            FieldInfoData {
                name: "TakeScreenshotOnFrame",
                name_hash: 490968814,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DemoSettings, take_screenshot_on_frame),
            },
        ],
    }),
    array_type: Some(DEMOSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DemoSettings {
    fn type_info(&self) -> &'static TypeInfo {
        DEMOSETTINGS_TYPE_INFO
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


pub static DEMOSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DemoSettings-Array",
    name_hash: 1426324343,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("DemoSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PerformanceTrackerSettings {
    pub _glacier_base: super::core::SystemSettings,
    pub enabled: bool,
    pub interval: f32,
    pub supress_performance_stats_on_idle: bool,
    pub supress_performance_stats_until_spawned: bool,
    pub juice_log_performance: bool,
}

pub trait PerformanceTrackerSettingsTrait: super::core::SystemSettingsTrait {
    fn enabled(&self) -> &bool;
    fn enabled_mut(&mut self) -> &mut bool;
    fn interval(&self) -> &f32;
    fn interval_mut(&mut self) -> &mut f32;
    fn supress_performance_stats_on_idle(&self) -> &bool;
    fn supress_performance_stats_on_idle_mut(&mut self) -> &mut bool;
    fn supress_performance_stats_until_spawned(&self) -> &bool;
    fn supress_performance_stats_until_spawned_mut(&mut self) -> &mut bool;
    fn juice_log_performance(&self) -> &bool;
    fn juice_log_performance_mut(&mut self) -> &mut bool;
}

impl PerformanceTrackerSettingsTrait for PerformanceTrackerSettings {
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
    fn interval(&self) -> &f32 {
        &self.interval
    }
    fn interval_mut(&mut self) -> &mut f32 {
        &mut self.interval
    }
    fn supress_performance_stats_on_idle(&self) -> &bool {
        &self.supress_performance_stats_on_idle
    }
    fn supress_performance_stats_on_idle_mut(&mut self) -> &mut bool {
        &mut self.supress_performance_stats_on_idle
    }
    fn supress_performance_stats_until_spawned(&self) -> &bool {
        &self.supress_performance_stats_until_spawned
    }
    fn supress_performance_stats_until_spawned_mut(&mut self) -> &mut bool {
        &mut self.supress_performance_stats_until_spawned
    }
    fn juice_log_performance(&self) -> &bool {
        &self.juice_log_performance
    }
    fn juice_log_performance_mut(&mut self) -> &mut bool {
        &mut self.juice_log_performance
    }
}

impl super::core::SystemSettingsTrait for PerformanceTrackerSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl super::core::DataContainerTrait for PerformanceTrackerSettings {
}

pub static PERFORMANCETRACKERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerformanceTrackerSettings",
    name_hash: 934666528,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        super_class_offset: offset_of!(PerformanceTrackerSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PerformanceTrackerSettings as Default>::default())),
            create_boxed: || Box::new(<PerformanceTrackerSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Enabled",
                name_hash: 2662400,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PerformanceTrackerSettings, enabled),
            },
            FieldInfoData {
                name: "Interval",
                name_hash: 4280103418,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PerformanceTrackerSettings, interval),
            },
            FieldInfoData {
                name: "SupressPerformanceStatsOnIdle",
                name_hash: 1614849048,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PerformanceTrackerSettings, supress_performance_stats_on_idle),
            },
            FieldInfoData {
                name: "SupressPerformanceStatsUntilSpawned",
                name_hash: 13225645,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PerformanceTrackerSettings, supress_performance_stats_until_spawned),
            },
            FieldInfoData {
                name: "JuiceLogPerformance",
                name_hash: 2821486313,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PerformanceTrackerSettings, juice_log_performance),
            },
        ],
    }),
    array_type: Some(PERFORMANCETRACKERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PerformanceTrackerSettings {
    fn type_info(&self) -> &'static TypeInfo {
        PERFORMANCETRACKERSETTINGS_TYPE_INFO
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


pub static PERFORMANCETRACKERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerformanceTrackerSettings-Array",
    name_hash: 3524830868,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("PerformanceTrackerSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct GameTimeSettings {
    pub _glacier_base: super::core::SystemSettings,
    pub use_waitable_timers: bool,
    pub max_sim_fps: f32,
    pub force_sim_rate: u32,
    pub max_virtual_ticks: u32,
    pub max_variable_fps: f32,
    pub max_inactive_variable_fps: f32,
    pub force_delta_time: f32,
    pub force_use_sleep_timer: bool,
    pub time_scale: f32,
    pub debug_frame_delay_ms: u32,
    pub dedicated_server_sleep_in_ms_during_loading: u32,
}

pub trait GameTimeSettingsTrait: super::core::SystemSettingsTrait {
    fn use_waitable_timers(&self) -> &bool;
    fn use_waitable_timers_mut(&mut self) -> &mut bool;
    fn max_sim_fps(&self) -> &f32;
    fn max_sim_fps_mut(&mut self) -> &mut f32;
    fn force_sim_rate(&self) -> &u32;
    fn force_sim_rate_mut(&mut self) -> &mut u32;
    fn max_virtual_ticks(&self) -> &u32;
    fn max_virtual_ticks_mut(&mut self) -> &mut u32;
    fn max_variable_fps(&self) -> &f32;
    fn max_variable_fps_mut(&mut self) -> &mut f32;
    fn max_inactive_variable_fps(&self) -> &f32;
    fn max_inactive_variable_fps_mut(&mut self) -> &mut f32;
    fn force_delta_time(&self) -> &f32;
    fn force_delta_time_mut(&mut self) -> &mut f32;
    fn force_use_sleep_timer(&self) -> &bool;
    fn force_use_sleep_timer_mut(&mut self) -> &mut bool;
    fn time_scale(&self) -> &f32;
    fn time_scale_mut(&mut self) -> &mut f32;
    fn debug_frame_delay_ms(&self) -> &u32;
    fn debug_frame_delay_ms_mut(&mut self) -> &mut u32;
    fn dedicated_server_sleep_in_ms_during_loading(&self) -> &u32;
    fn dedicated_server_sleep_in_ms_during_loading_mut(&mut self) -> &mut u32;
}

impl GameTimeSettingsTrait for GameTimeSettings {
    fn use_waitable_timers(&self) -> &bool {
        &self.use_waitable_timers
    }
    fn use_waitable_timers_mut(&mut self) -> &mut bool {
        &mut self.use_waitable_timers
    }
    fn max_sim_fps(&self) -> &f32 {
        &self.max_sim_fps
    }
    fn max_sim_fps_mut(&mut self) -> &mut f32 {
        &mut self.max_sim_fps
    }
    fn force_sim_rate(&self) -> &u32 {
        &self.force_sim_rate
    }
    fn force_sim_rate_mut(&mut self) -> &mut u32 {
        &mut self.force_sim_rate
    }
    fn max_virtual_ticks(&self) -> &u32 {
        &self.max_virtual_ticks
    }
    fn max_virtual_ticks_mut(&mut self) -> &mut u32 {
        &mut self.max_virtual_ticks
    }
    fn max_variable_fps(&self) -> &f32 {
        &self.max_variable_fps
    }
    fn max_variable_fps_mut(&mut self) -> &mut f32 {
        &mut self.max_variable_fps
    }
    fn max_inactive_variable_fps(&self) -> &f32 {
        &self.max_inactive_variable_fps
    }
    fn max_inactive_variable_fps_mut(&mut self) -> &mut f32 {
        &mut self.max_inactive_variable_fps
    }
    fn force_delta_time(&self) -> &f32 {
        &self.force_delta_time
    }
    fn force_delta_time_mut(&mut self) -> &mut f32 {
        &mut self.force_delta_time
    }
    fn force_use_sleep_timer(&self) -> &bool {
        &self.force_use_sleep_timer
    }
    fn force_use_sleep_timer_mut(&mut self) -> &mut bool {
        &mut self.force_use_sleep_timer
    }
    fn time_scale(&self) -> &f32 {
        &self.time_scale
    }
    fn time_scale_mut(&mut self) -> &mut f32 {
        &mut self.time_scale
    }
    fn debug_frame_delay_ms(&self) -> &u32 {
        &self.debug_frame_delay_ms
    }
    fn debug_frame_delay_ms_mut(&mut self) -> &mut u32 {
        &mut self.debug_frame_delay_ms
    }
    fn dedicated_server_sleep_in_ms_during_loading(&self) -> &u32 {
        &self.dedicated_server_sleep_in_ms_during_loading
    }
    fn dedicated_server_sleep_in_ms_during_loading_mut(&mut self) -> &mut u32 {
        &mut self.dedicated_server_sleep_in_ms_during_loading
    }
}

impl super::core::SystemSettingsTrait for GameTimeSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl super::core::DataContainerTrait for GameTimeSettings {
}

pub static GAMETIMESETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameTimeSettings",
    name_hash: 252444219,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        super_class_offset: offset_of!(GameTimeSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GameTimeSettings as Default>::default())),
            create_boxed: || Box::new(<GameTimeSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "UseWaitableTimers",
                name_hash: 1210114003,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GameTimeSettings, use_waitable_timers),
            },
            FieldInfoData {
                name: "MaxSimFps",
                name_hash: 179519747,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(GameTimeSettings, max_sim_fps),
            },
            FieldInfoData {
                name: "ForceSimRate",
                name_hash: 3739955053,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(GameTimeSettings, force_sim_rate),
            },
            FieldInfoData {
                name: "MaxVirtualTicks",
                name_hash: 3471118902,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(GameTimeSettings, max_virtual_ticks),
            },
            FieldInfoData {
                name: "MaxVariableFps",
                name_hash: 3499862514,
                flags: MemberInfoFlags::new(8192),
                field_type: "Float32",
                rust_offset: offset_of!(GameTimeSettings, max_variable_fps),
            },
            FieldInfoData {
                name: "MaxInactiveVariableFps",
                name_hash: 2767785209,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(GameTimeSettings, max_inactive_variable_fps),
            },
            FieldInfoData {
                name: "ForceDeltaTime",
                name_hash: 989222869,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(GameTimeSettings, force_delta_time),
            },
            FieldInfoData {
                name: "ForceUseSleepTimer",
                name_hash: 21790035,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GameTimeSettings, force_use_sleep_timer),
            },
            FieldInfoData {
                name: "TimeScale",
                name_hash: 169511528,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(GameTimeSettings, time_scale),
            },
            FieldInfoData {
                name: "DebugFrameDelayMs",
                name_hash: 1816740642,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(GameTimeSettings, debug_frame_delay_ms),
            },
            FieldInfoData {
                name: "DedicatedServerSleepInMsDuringLoading",
                name_hash: 4228197768,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(GameTimeSettings, dedicated_server_sleep_in_ms_during_loading),
            },
        ],
    }),
    array_type: Some(GAMETIMESETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GameTimeSettings {
    fn type_info(&self) -> &'static TypeInfo {
        GAMETIMESETTINGS_TYPE_INFO
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


pub static GAMETIMESETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameTimeSettings-Array",
    name_hash: 737305999,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("GameTimeSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SyncedGameSettings {
    pub _glacier_base: super::core::SystemSettings,
    pub disable_toggle_entry_camera: bool,
    pub disable_regenerate_health: bool,
    pub enable_friendly_fire: bool,
    pub allow_client_side_damage_arbitration: bool,
    pub difficulty_index: u32,
    pub bullet_damage_modifier: f32,
    pub max_allowed_latency: f32,
    pub frame_history_time_max: f32,
    pub frame_history_time: f32,
    pub move_manager_outgoing_frequency_divider: u32,
    pub move_manager_single_player_outgoing_frequency_divider: u32,
    pub max_correction_update_count: u32,
    pub enable_automatic_correction_update_count: bool,
}

pub trait SyncedGameSettingsTrait: super::core::SystemSettingsTrait {
    fn disable_toggle_entry_camera(&self) -> &bool;
    fn disable_toggle_entry_camera_mut(&mut self) -> &mut bool;
    fn disable_regenerate_health(&self) -> &bool;
    fn disable_regenerate_health_mut(&mut self) -> &mut bool;
    fn enable_friendly_fire(&self) -> &bool;
    fn enable_friendly_fire_mut(&mut self) -> &mut bool;
    fn allow_client_side_damage_arbitration(&self) -> &bool;
    fn allow_client_side_damage_arbitration_mut(&mut self) -> &mut bool;
    fn difficulty_index(&self) -> &u32;
    fn difficulty_index_mut(&mut self) -> &mut u32;
    fn bullet_damage_modifier(&self) -> &f32;
    fn bullet_damage_modifier_mut(&mut self) -> &mut f32;
    fn max_allowed_latency(&self) -> &f32;
    fn max_allowed_latency_mut(&mut self) -> &mut f32;
    fn frame_history_time_max(&self) -> &f32;
    fn frame_history_time_max_mut(&mut self) -> &mut f32;
    fn frame_history_time(&self) -> &f32;
    fn frame_history_time_mut(&mut self) -> &mut f32;
    fn move_manager_outgoing_frequency_divider(&self) -> &u32;
    fn move_manager_outgoing_frequency_divider_mut(&mut self) -> &mut u32;
    fn move_manager_single_player_outgoing_frequency_divider(&self) -> &u32;
    fn move_manager_single_player_outgoing_frequency_divider_mut(&mut self) -> &mut u32;
    fn max_correction_update_count(&self) -> &u32;
    fn max_correction_update_count_mut(&mut self) -> &mut u32;
    fn enable_automatic_correction_update_count(&self) -> &bool;
    fn enable_automatic_correction_update_count_mut(&mut self) -> &mut bool;
}

impl SyncedGameSettingsTrait for SyncedGameSettings {
    fn disable_toggle_entry_camera(&self) -> &bool {
        &self.disable_toggle_entry_camera
    }
    fn disable_toggle_entry_camera_mut(&mut self) -> &mut bool {
        &mut self.disable_toggle_entry_camera
    }
    fn disable_regenerate_health(&self) -> &bool {
        &self.disable_regenerate_health
    }
    fn disable_regenerate_health_mut(&mut self) -> &mut bool {
        &mut self.disable_regenerate_health
    }
    fn enable_friendly_fire(&self) -> &bool {
        &self.enable_friendly_fire
    }
    fn enable_friendly_fire_mut(&mut self) -> &mut bool {
        &mut self.enable_friendly_fire
    }
    fn allow_client_side_damage_arbitration(&self) -> &bool {
        &self.allow_client_side_damage_arbitration
    }
    fn allow_client_side_damage_arbitration_mut(&mut self) -> &mut bool {
        &mut self.allow_client_side_damage_arbitration
    }
    fn difficulty_index(&self) -> &u32 {
        &self.difficulty_index
    }
    fn difficulty_index_mut(&mut self) -> &mut u32 {
        &mut self.difficulty_index
    }
    fn bullet_damage_modifier(&self) -> &f32 {
        &self.bullet_damage_modifier
    }
    fn bullet_damage_modifier_mut(&mut self) -> &mut f32 {
        &mut self.bullet_damage_modifier
    }
    fn max_allowed_latency(&self) -> &f32 {
        &self.max_allowed_latency
    }
    fn max_allowed_latency_mut(&mut self) -> &mut f32 {
        &mut self.max_allowed_latency
    }
    fn frame_history_time_max(&self) -> &f32 {
        &self.frame_history_time_max
    }
    fn frame_history_time_max_mut(&mut self) -> &mut f32 {
        &mut self.frame_history_time_max
    }
    fn frame_history_time(&self) -> &f32 {
        &self.frame_history_time
    }
    fn frame_history_time_mut(&mut self) -> &mut f32 {
        &mut self.frame_history_time
    }
    fn move_manager_outgoing_frequency_divider(&self) -> &u32 {
        &self.move_manager_outgoing_frequency_divider
    }
    fn move_manager_outgoing_frequency_divider_mut(&mut self) -> &mut u32 {
        &mut self.move_manager_outgoing_frequency_divider
    }
    fn move_manager_single_player_outgoing_frequency_divider(&self) -> &u32 {
        &self.move_manager_single_player_outgoing_frequency_divider
    }
    fn move_manager_single_player_outgoing_frequency_divider_mut(&mut self) -> &mut u32 {
        &mut self.move_manager_single_player_outgoing_frequency_divider
    }
    fn max_correction_update_count(&self) -> &u32 {
        &self.max_correction_update_count
    }
    fn max_correction_update_count_mut(&mut self) -> &mut u32 {
        &mut self.max_correction_update_count
    }
    fn enable_automatic_correction_update_count(&self) -> &bool {
        &self.enable_automatic_correction_update_count
    }
    fn enable_automatic_correction_update_count_mut(&mut self) -> &mut bool {
        &mut self.enable_automatic_correction_update_count
    }
}

impl super::core::SystemSettingsTrait for SyncedGameSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl super::core::DataContainerTrait for SyncedGameSettings {
}

pub static SYNCEDGAMESETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SyncedGameSettings",
    name_hash: 3835761160,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        super_class_offset: offset_of!(SyncedGameSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SyncedGameSettings as Default>::default())),
            create_boxed: || Box::new(<SyncedGameSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "DisableToggleEntryCamera",
                name_hash: 1613644462,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SyncedGameSettings, disable_toggle_entry_camera),
            },
            FieldInfoData {
                name: "DisableRegenerateHealth",
                name_hash: 1264747441,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SyncedGameSettings, disable_regenerate_health),
            },
            FieldInfoData {
                name: "EnableFriendlyFire",
                name_hash: 362696539,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SyncedGameSettings, enable_friendly_fire),
            },
            FieldInfoData {
                name: "AllowClientSideDamageArbitration",
                name_hash: 1990831926,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SyncedGameSettings, allow_client_side_damage_arbitration),
            },
            FieldInfoData {
                name: "DifficultyIndex",
                name_hash: 302070728,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(SyncedGameSettings, difficulty_index),
            },
            FieldInfoData {
                name: "BulletDamageModifier",
                name_hash: 204218335,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SyncedGameSettings, bullet_damage_modifier),
            },
            FieldInfoData {
                name: "MaxAllowedLatency",
                name_hash: 57128033,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SyncedGameSettings, max_allowed_latency),
            },
            FieldInfoData {
                name: "FrameHistoryTimeMax",
                name_hash: 1702962043,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SyncedGameSettings, frame_history_time_max),
            },
            FieldInfoData {
                name: "FrameHistoryTime",
                name_hash: 2520473839,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SyncedGameSettings, frame_history_time),
            },
            FieldInfoData {
                name: "MoveManagerOutgoingFrequencyDivider",
                name_hash: 785515140,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(SyncedGameSettings, move_manager_outgoing_frequency_divider),
            },
            FieldInfoData {
                name: "MoveManagerSinglePlayerOutgoingFrequencyDivider",
                name_hash: 2491979245,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(SyncedGameSettings, move_manager_single_player_outgoing_frequency_divider),
            },
            FieldInfoData {
                name: "MaxCorrectionUpdateCount",
                name_hash: 2870541109,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(SyncedGameSettings, max_correction_update_count),
            },
            FieldInfoData {
                name: "EnableAutomaticCorrectionUpdateCount",
                name_hash: 283031005,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SyncedGameSettings, enable_automatic_correction_update_count),
            },
        ],
    }),
    array_type: Some(SYNCEDGAMESETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SyncedGameSettings {
    fn type_info(&self) -> &'static TypeInfo {
        SYNCEDGAMESETTINGS_TYPE_INFO
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


pub static SYNCEDGAMESETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SyncedGameSettings-Array",
    name_hash: 1489278012,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("SyncedGameSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct GameSettings {
    pub _glacier_base: super::core::SystemSettings,
    pub max_player_count: u32,
    pub max_spectator_count: u32,
    pub log_file_enable: bool,
    pub log_file_collision_mode: LogFileCollisionMode,
    pub log_file_rotation_history_length: u32,
    pub level: String,
    pub start_point: String,
    pub installation_level: String,
    pub installation_start_point: String,
    pub installation_default_layer_inclusion: String,
    pub resource_refresh_always_allowed: bool,
    pub active_game_mode_view_definition: String,
    pub game_mode_view_definitions: Vec<Option<LockedTypeObject /* GameModeViewDefinition */>>,
    pub spawn_max_local_players_on_startup: bool,
    pub default_team_id: TeamId,
    pub version: Option<LockedTypeObject /* VersionData */>,
    pub layer_inclusion_table: Option<LockedTypeObject /* super::entity::SubWorldInclusion */>,
    pub default_layer_inclusion: String,
    pub time_to_wait_for_quit_task_completion: f32,
    pub player: Option<LockedTypeObject /* PlayerData */>,
    pub difficulty_index: i32,
    pub game_settings_components: Vec<Option<LockedTypeObject /* GameSettingsComponent */>>,
}

pub trait GameSettingsTrait: super::core::SystemSettingsTrait {
    fn max_player_count(&self) -> &u32;
    fn max_player_count_mut(&mut self) -> &mut u32;
    fn max_spectator_count(&self) -> &u32;
    fn max_spectator_count_mut(&mut self) -> &mut u32;
    fn log_file_enable(&self) -> &bool;
    fn log_file_enable_mut(&mut self) -> &mut bool;
    fn log_file_collision_mode(&self) -> &LogFileCollisionMode;
    fn log_file_collision_mode_mut(&mut self) -> &mut LogFileCollisionMode;
    fn log_file_rotation_history_length(&self) -> &u32;
    fn log_file_rotation_history_length_mut(&mut self) -> &mut u32;
    fn level(&self) -> &String;
    fn level_mut(&mut self) -> &mut String;
    fn start_point(&self) -> &String;
    fn start_point_mut(&mut self) -> &mut String;
    fn installation_level(&self) -> &String;
    fn installation_level_mut(&mut self) -> &mut String;
    fn installation_start_point(&self) -> &String;
    fn installation_start_point_mut(&mut self) -> &mut String;
    fn installation_default_layer_inclusion(&self) -> &String;
    fn installation_default_layer_inclusion_mut(&mut self) -> &mut String;
    fn resource_refresh_always_allowed(&self) -> &bool;
    fn resource_refresh_always_allowed_mut(&mut self) -> &mut bool;
    fn active_game_mode_view_definition(&self) -> &String;
    fn active_game_mode_view_definition_mut(&mut self) -> &mut String;
    fn game_mode_view_definitions(&self) -> &Vec<Option<LockedTypeObject /* GameModeViewDefinition */>>;
    fn game_mode_view_definitions_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* GameModeViewDefinition */>>;
    fn spawn_max_local_players_on_startup(&self) -> &bool;
    fn spawn_max_local_players_on_startup_mut(&mut self) -> &mut bool;
    fn default_team_id(&self) -> &TeamId;
    fn default_team_id_mut(&mut self) -> &mut TeamId;
    fn version(&self) -> &Option<LockedTypeObject /* VersionData */>;
    fn version_mut(&mut self) -> &mut Option<LockedTypeObject /* VersionData */>;
    fn layer_inclusion_table(&self) -> &Option<LockedTypeObject /* super::entity::SubWorldInclusion */>;
    fn layer_inclusion_table_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::SubWorldInclusion */>;
    fn default_layer_inclusion(&self) -> &String;
    fn default_layer_inclusion_mut(&mut self) -> &mut String;
    fn time_to_wait_for_quit_task_completion(&self) -> &f32;
    fn time_to_wait_for_quit_task_completion_mut(&mut self) -> &mut f32;
    fn player(&self) -> &Option<LockedTypeObject /* PlayerData */>;
    fn player_mut(&mut self) -> &mut Option<LockedTypeObject /* PlayerData */>;
    fn difficulty_index(&self) -> &i32;
    fn difficulty_index_mut(&mut self) -> &mut i32;
    fn game_settings_components(&self) -> &Vec<Option<LockedTypeObject /* GameSettingsComponent */>>;
    fn game_settings_components_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* GameSettingsComponent */>>;
}

impl GameSettingsTrait for GameSettings {
    fn max_player_count(&self) -> &u32 {
        &self.max_player_count
    }
    fn max_player_count_mut(&mut self) -> &mut u32 {
        &mut self.max_player_count
    }
    fn max_spectator_count(&self) -> &u32 {
        &self.max_spectator_count
    }
    fn max_spectator_count_mut(&mut self) -> &mut u32 {
        &mut self.max_spectator_count
    }
    fn log_file_enable(&self) -> &bool {
        &self.log_file_enable
    }
    fn log_file_enable_mut(&mut self) -> &mut bool {
        &mut self.log_file_enable
    }
    fn log_file_collision_mode(&self) -> &LogFileCollisionMode {
        &self.log_file_collision_mode
    }
    fn log_file_collision_mode_mut(&mut self) -> &mut LogFileCollisionMode {
        &mut self.log_file_collision_mode
    }
    fn log_file_rotation_history_length(&self) -> &u32 {
        &self.log_file_rotation_history_length
    }
    fn log_file_rotation_history_length_mut(&mut self) -> &mut u32 {
        &mut self.log_file_rotation_history_length
    }
    fn level(&self) -> &String {
        &self.level
    }
    fn level_mut(&mut self) -> &mut String {
        &mut self.level
    }
    fn start_point(&self) -> &String {
        &self.start_point
    }
    fn start_point_mut(&mut self) -> &mut String {
        &mut self.start_point
    }
    fn installation_level(&self) -> &String {
        &self.installation_level
    }
    fn installation_level_mut(&mut self) -> &mut String {
        &mut self.installation_level
    }
    fn installation_start_point(&self) -> &String {
        &self.installation_start_point
    }
    fn installation_start_point_mut(&mut self) -> &mut String {
        &mut self.installation_start_point
    }
    fn installation_default_layer_inclusion(&self) -> &String {
        &self.installation_default_layer_inclusion
    }
    fn installation_default_layer_inclusion_mut(&mut self) -> &mut String {
        &mut self.installation_default_layer_inclusion
    }
    fn resource_refresh_always_allowed(&self) -> &bool {
        &self.resource_refresh_always_allowed
    }
    fn resource_refresh_always_allowed_mut(&mut self) -> &mut bool {
        &mut self.resource_refresh_always_allowed
    }
    fn active_game_mode_view_definition(&self) -> &String {
        &self.active_game_mode_view_definition
    }
    fn active_game_mode_view_definition_mut(&mut self) -> &mut String {
        &mut self.active_game_mode_view_definition
    }
    fn game_mode_view_definitions(&self) -> &Vec<Option<LockedTypeObject /* GameModeViewDefinition */>> {
        &self.game_mode_view_definitions
    }
    fn game_mode_view_definitions_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* GameModeViewDefinition */>> {
        &mut self.game_mode_view_definitions
    }
    fn spawn_max_local_players_on_startup(&self) -> &bool {
        &self.spawn_max_local_players_on_startup
    }
    fn spawn_max_local_players_on_startup_mut(&mut self) -> &mut bool {
        &mut self.spawn_max_local_players_on_startup
    }
    fn default_team_id(&self) -> &TeamId {
        &self.default_team_id
    }
    fn default_team_id_mut(&mut self) -> &mut TeamId {
        &mut self.default_team_id
    }
    fn version(&self) -> &Option<LockedTypeObject /* VersionData */> {
        &self.version
    }
    fn version_mut(&mut self) -> &mut Option<LockedTypeObject /* VersionData */> {
        &mut self.version
    }
    fn layer_inclusion_table(&self) -> &Option<LockedTypeObject /* super::entity::SubWorldInclusion */> {
        &self.layer_inclusion_table
    }
    fn layer_inclusion_table_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::SubWorldInclusion */> {
        &mut self.layer_inclusion_table
    }
    fn default_layer_inclusion(&self) -> &String {
        &self.default_layer_inclusion
    }
    fn default_layer_inclusion_mut(&mut self) -> &mut String {
        &mut self.default_layer_inclusion
    }
    fn time_to_wait_for_quit_task_completion(&self) -> &f32 {
        &self.time_to_wait_for_quit_task_completion
    }
    fn time_to_wait_for_quit_task_completion_mut(&mut self) -> &mut f32 {
        &mut self.time_to_wait_for_quit_task_completion
    }
    fn player(&self) -> &Option<LockedTypeObject /* PlayerData */> {
        &self.player
    }
    fn player_mut(&mut self) -> &mut Option<LockedTypeObject /* PlayerData */> {
        &mut self.player
    }
    fn difficulty_index(&self) -> &i32 {
        &self.difficulty_index
    }
    fn difficulty_index_mut(&mut self) -> &mut i32 {
        &mut self.difficulty_index
    }
    fn game_settings_components(&self) -> &Vec<Option<LockedTypeObject /* GameSettingsComponent */>> {
        &self.game_settings_components
    }
    fn game_settings_components_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* GameSettingsComponent */>> {
        &mut self.game_settings_components
    }
}

impl super::core::SystemSettingsTrait for GameSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl super::core::DataContainerTrait for GameSettings {
}

pub static GAMESETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameSettings",
    name_hash: 970242990,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        super_class_offset: offset_of!(GameSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GameSettings as Default>::default())),
            create_boxed: || Box::new(<GameSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "MaxPlayerCount",
                name_hash: 4192814593,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(GameSettings, max_player_count),
            },
            FieldInfoData {
                name: "MaxSpectatorCount",
                name_hash: 3946650923,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(GameSettings, max_spectator_count),
            },
            FieldInfoData {
                name: "LogFileEnable",
                name_hash: 2156101894,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GameSettings, log_file_enable),
            },
            FieldInfoData {
                name: "LogFileCollisionMode",
                name_hash: 3429178042,
                flags: MemberInfoFlags::new(0),
                field_type: "LogFileCollisionMode",
                rust_offset: offset_of!(GameSettings, log_file_collision_mode),
            },
            FieldInfoData {
                name: "LogFileRotationHistoryLength",
                name_hash: 3548846413,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(GameSettings, log_file_rotation_history_length),
            },
            FieldInfoData {
                name: "Level",
                name_hash: 218262515,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(GameSettings, level),
            },
            FieldInfoData {
                name: "StartPoint",
                name_hash: 2755831849,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(GameSettings, start_point),
            },
            FieldInfoData {
                name: "InstallationLevel",
                name_hash: 1237134799,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(GameSettings, installation_level),
            },
            FieldInfoData {
                name: "InstallationStartPoint",
                name_hash: 3231582741,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(GameSettings, installation_start_point),
            },
            FieldInfoData {
                name: "InstallationDefaultLayerInclusion",
                name_hash: 2494104823,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(GameSettings, installation_default_layer_inclusion),
            },
            FieldInfoData {
                name: "ResourceRefreshAlwaysAllowed",
                name_hash: 3415354971,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GameSettings, resource_refresh_always_allowed),
            },
            FieldInfoData {
                name: "ActiveGameModeViewDefinition",
                name_hash: 1232105660,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(GameSettings, active_game_mode_view_definition),
            },
            FieldInfoData {
                name: "GameModeViewDefinitions",
                name_hash: 1331648515,
                flags: MemberInfoFlags::new(144),
                field_type: "GameModeViewDefinition-Array",
                rust_offset: offset_of!(GameSettings, game_mode_view_definitions),
            },
            FieldInfoData {
                name: "SpawnMaxLocalPlayersOnStartup",
                name_hash: 4181246019,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GameSettings, spawn_max_local_players_on_startup),
            },
            FieldInfoData {
                name: "DefaultTeamId",
                name_hash: 3990246590,
                flags: MemberInfoFlags::new(0),
                field_type: "TeamId",
                rust_offset: offset_of!(GameSettings, default_team_id),
            },
            FieldInfoData {
                name: "Version",
                name_hash: 747123679,
                flags: MemberInfoFlags::new(0),
                field_type: "VersionData",
                rust_offset: offset_of!(GameSettings, version),
            },
            FieldInfoData {
                name: "LayerInclusionTable",
                name_hash: 1500062686,
                flags: MemberInfoFlags::new(0),
                field_type: "SubWorldInclusion",
                rust_offset: offset_of!(GameSettings, layer_inclusion_table),
            },
            FieldInfoData {
                name: "DefaultLayerInclusion",
                name_hash: 4278341003,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(GameSettings, default_layer_inclusion),
            },
            FieldInfoData {
                name: "TimeToWaitForQuitTaskCompletion",
                name_hash: 2225926219,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(GameSettings, time_to_wait_for_quit_task_completion),
            },
            FieldInfoData {
                name: "Player",
                name_hash: 3384765366,
                flags: MemberInfoFlags::new(0),
                field_type: "PlayerData",
                rust_offset: offset_of!(GameSettings, player),
            },
            FieldInfoData {
                name: "DifficultyIndex",
                name_hash: 302070728,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(GameSettings, difficulty_index),
            },
            FieldInfoData {
                name: "GameSettingsComponents",
                name_hash: 3053914066,
                flags: MemberInfoFlags::new(144),
                field_type: "GameSettingsComponent-Array",
                rust_offset: offset_of!(GameSettings, game_settings_components),
            },
        ],
    }),
    array_type: Some(GAMESETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GameSettings {
    fn type_info(&self) -> &'static TypeInfo {
        GAMESETTINGS_TYPE_INFO
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


pub static GAMESETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameSettings-Array",
    name_hash: 92857370,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("GameSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct GameSettingsComponent {
    pub _glacier_base: super::core::Asset,
}

pub trait GameSettingsComponentTrait: super::core::AssetTrait {
}

impl GameSettingsComponentTrait for GameSettingsComponent {
}

impl super::core::AssetTrait for GameSettingsComponent {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for GameSettingsComponent {
}

pub static GAMESETTINGSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameSettingsComponent",
    name_hash: 92542849,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(GameSettingsComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GameSettingsComponent as Default>::default())),
            create_boxed: || Box::new(<GameSettingsComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(GAMESETTINGSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GameSettingsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        GAMESETTINGSCOMPONENT_TYPE_INFO
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


pub static GAMESETTINGSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameSettingsComponent-Array",
    name_hash: 830550325,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("GameSettingsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum LogFileCollisionMode {
    #[default]
    LFCM_Overwrite = 0,
    LFCM_Rotate = 1,
    LFCM_TimeStamp = 2,
}

pub static LOGFILECOLLISIONMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogFileCollisionMode",
    name_hash: 3429178042,
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(LOGFILECOLLISIONMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LogFileCollisionMode {
    fn type_info(&self) -> &'static TypeInfo {
        LOGFILECOLLISIONMODE_TYPE_INFO
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


pub static LOGFILECOLLISIONMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogFileCollisionMode-Array",
    name_hash: 3472334094,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("LogFileCollisionMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CoreDebugReadProfileGameDataMessage {
}

pub trait CoreDebugReadProfileGameDataMessageTrait: TypeObject {
}

impl CoreDebugReadProfileGameDataMessageTrait for CoreDebugReadProfileGameDataMessage {
}

pub static COREDEBUGREADPROFILEGAMEDATAMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreDebugReadProfileGameDataMessage",
    name_hash: 4267161987,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoreDebugReadProfileGameDataMessage as Default>::default())),
            create_boxed: || Box::new(<CoreDebugReadProfileGameDataMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreDebugReadProfileGameDataMessage {
    fn type_info(&self) -> &'static TypeInfo {
        COREDEBUGREADPROFILEGAMEDATAMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct CoreReadSaveGameDataDoneMessage {
}

pub trait CoreReadSaveGameDataDoneMessageTrait: TypeObject {
}

impl CoreReadSaveGameDataDoneMessageTrait for CoreReadSaveGameDataDoneMessage {
}

pub static COREREADSAVEGAMEDATADONEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreReadSaveGameDataDoneMessage",
    name_hash: 2345206392,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoreReadSaveGameDataDoneMessage as Default>::default())),
            create_boxed: || Box::new(<CoreReadSaveGameDataDoneMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreReadSaveGameDataDoneMessage {
    fn type_info(&self) -> &'static TypeInfo {
        COREREADSAVEGAMEDATADONEMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct CoreReadSaveGameDoneMessage {
}

pub trait CoreReadSaveGameDoneMessageTrait: TypeObject {
}

impl CoreReadSaveGameDoneMessageTrait for CoreReadSaveGameDoneMessage {
}

pub static COREREADSAVEGAMEDONEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreReadSaveGameDoneMessage",
    name_hash: 4115003528,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoreReadSaveGameDoneMessage as Default>::default())),
            create_boxed: || Box::new(<CoreReadSaveGameDoneMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreReadSaveGameDoneMessage {
    fn type_info(&self) -> &'static TypeInfo {
        COREREADSAVEGAMEDONEMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct CoreReadSaveGameDataMessage {
}

pub trait CoreReadSaveGameDataMessageTrait: TypeObject {
}

impl CoreReadSaveGameDataMessageTrait for CoreReadSaveGameDataMessage {
}

pub static COREREADSAVEGAMEDATAMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreReadSaveGameDataMessage",
    name_hash: 3946246488,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoreReadSaveGameDataMessage as Default>::default())),
            create_boxed: || Box::new(<CoreReadSaveGameDataMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreReadSaveGameDataMessage {
    fn type_info(&self) -> &'static TypeInfo {
        COREREADSAVEGAMEDATAMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct CoreWriteSaveGameDoneMessage {
}

pub trait CoreWriteSaveGameDoneMessageTrait: TypeObject {
}

impl CoreWriteSaveGameDoneMessageTrait for CoreWriteSaveGameDoneMessage {
}

pub static COREWRITESAVEGAMEDONEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreWriteSaveGameDoneMessage",
    name_hash: 4071547175,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoreWriteSaveGameDoneMessage as Default>::default())),
            create_boxed: || Box::new(<CoreWriteSaveGameDoneMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreWriteSaveGameDoneMessage {
    fn type_info(&self) -> &'static TypeInfo {
        COREWRITESAVEGAMEDONEMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct CoreWriteSaveGameMessage {
}

pub trait CoreWriteSaveGameMessageTrait: TypeObject {
}

impl CoreWriteSaveGameMessageTrait for CoreWriteSaveGameMessage {
}

pub static COREWRITESAVEGAMEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreWriteSaveGameMessage",
    name_hash: 2563614855,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoreWriteSaveGameMessage as Default>::default())),
            create_boxed: || Box::new(<CoreWriteSaveGameMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreWriteSaveGameMessage {
    fn type_info(&self) -> &'static TypeInfo {
        COREWRITESAVEGAMEMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct CoreWriteProfileGameMessage {
}

pub trait CoreWriteProfileGameMessageTrait: TypeObject {
}

impl CoreWriteProfileGameMessageTrait for CoreWriteProfileGameMessage {
}

pub static COREWRITEPROFILEGAMEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreWriteProfileGameMessage",
    name_hash: 1549510957,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoreWriteProfileGameMessage as Default>::default())),
            create_boxed: || Box::new(<CoreWriteProfileGameMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreWriteProfileGameMessage {
    fn type_info(&self) -> &'static TypeInfo {
        COREWRITEPROFILEGAMEMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct SaveGameHeaderEntry {
    pub key: i32,
    pub value: String,
}

pub trait SaveGameHeaderEntryTrait: TypeObject {
    fn key(&self) -> &i32;
    fn key_mut(&mut self) -> &mut i32;
    fn value(&self) -> &String;
    fn value_mut(&mut self) -> &mut String;
}

impl SaveGameHeaderEntryTrait for SaveGameHeaderEntry {
    fn key(&self) -> &i32 {
        &self.key
    }
    fn key_mut(&mut self) -> &mut i32 {
        &mut self.key
    }
    fn value(&self) -> &String {
        &self.value
    }
    fn value_mut(&mut self) -> &mut String {
        &mut self.value
    }
}

pub static SAVEGAMEHEADERENTRY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SaveGameHeaderEntry",
    name_hash: 3555070433,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SaveGameHeaderEntry as Default>::default())),
            create_boxed: || Box::new(<SaveGameHeaderEntry as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Key",
                name_hash: 193457490,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(SaveGameHeaderEntry, key),
            },
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(SaveGameHeaderEntry, value),
            },
        ],
    }),
    array_type: Some(SAVEGAMEHEADERENTRY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SaveGameHeaderEntry {
    fn type_info(&self) -> &'static TypeInfo {
        SAVEGAMEHEADERENTRY_TYPE_INFO
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


pub static SAVEGAMEHEADERENTRY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SaveGameHeaderEntry-Array",
    name_hash: 1468132565,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("SaveGameHeaderEntry"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CoreClientPreLoadCompleteMessage {
}

pub trait CoreClientPreLoadCompleteMessageTrait: TypeObject {
}

impl CoreClientPreLoadCompleteMessageTrait for CoreClientPreLoadCompleteMessage {
}

pub static CORECLIENTPRELOADCOMPLETEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreClientPreLoadCompleteMessage",
    name_hash: 1288151588,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoreClientPreLoadCompleteMessage as Default>::default())),
            create_boxed: || Box::new(<CoreClientPreLoadCompleteMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreClientPreLoadCompleteMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CORECLIENTPRELOADCOMPLETEMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct CoreClientPreLoadMessage {
}

pub trait CoreClientPreLoadMessageTrait: TypeObject {
}

impl CoreClientPreLoadMessageTrait for CoreClientPreLoadMessage {
}

pub static CORECLIENTPRELOADMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreClientPreLoadMessage",
    name_hash: 440787373,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoreClientPreLoadMessage as Default>::default())),
            create_boxed: || Box::new(<CoreClientPreLoadMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreClientPreLoadMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CORECLIENTPRELOADMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct CoreClientPreSaveCompleteMessage {
}

pub trait CoreClientPreSaveCompleteMessageTrait: TypeObject {
}

impl CoreClientPreSaveCompleteMessageTrait for CoreClientPreSaveCompleteMessage {
}

pub static CORECLIENTPRESAVECOMPLETEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreClientPreSaveCompleteMessage",
    name_hash: 3071967363,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoreClientPreSaveCompleteMessage as Default>::default())),
            create_boxed: || Box::new(<CoreClientPreSaveCompleteMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreClientPreSaveCompleteMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CORECLIENTPRESAVECOMPLETEMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct CoreClientPreSaveMessage {
}

pub trait CoreClientPreSaveMessageTrait: TypeObject {
}

impl CoreClientPreSaveMessageTrait for CoreClientPreSaveMessage {
}

pub static CORECLIENTPRESAVEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreClientPreSaveMessage",
    name_hash: 4071014666,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoreClientPreSaveMessage as Default>::default())),
            create_boxed: || Box::new(<CoreClientPreSaveMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreClientPreSaveMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CORECLIENTPRESAVEMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct LoadGameLoadRequestedMessage {
}

pub trait LoadGameLoadRequestedMessageTrait: TypeObject {
}

impl LoadGameLoadRequestedMessageTrait for LoadGameLoadRequestedMessage {
}

pub static LOADGAMELOADREQUESTEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LoadGameLoadRequestedMessage",
    name_hash: 3091917712,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LoadGameLoadRequestedMessage as Default>::default())),
            create_boxed: || Box::new(<LoadGameLoadRequestedMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for LoadGameLoadRequestedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        LOADGAMELOADREQUESTEDMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct LoadRequest {
    pub filename: String,
    pub setup_params: super::entity::LevelSetup,
    pub load_from_juice: bool,
}

pub trait LoadRequestTrait: TypeObject {
    fn filename(&self) -> &String;
    fn filename_mut(&mut self) -> &mut String;
    fn setup_params(&self) -> &super::entity::LevelSetup;
    fn setup_params_mut(&mut self) -> &mut super::entity::LevelSetup;
    fn load_from_juice(&self) -> &bool;
    fn load_from_juice_mut(&mut self) -> &mut bool;
}

impl LoadRequestTrait for LoadRequest {
    fn filename(&self) -> &String {
        &self.filename
    }
    fn filename_mut(&mut self) -> &mut String {
        &mut self.filename
    }
    fn setup_params(&self) -> &super::entity::LevelSetup {
        &self.setup_params
    }
    fn setup_params_mut(&mut self) -> &mut super::entity::LevelSetup {
        &mut self.setup_params
    }
    fn load_from_juice(&self) -> &bool {
        &self.load_from_juice
    }
    fn load_from_juice_mut(&mut self) -> &mut bool {
        &mut self.load_from_juice
    }
}

pub static LOADREQUEST_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LoadRequest",
    name_hash: 4270470162,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LoadRequest as Default>::default())),
            create_boxed: || Box::new(<LoadRequest as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Filename",
                name_hash: 1135826884,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(LoadRequest, filename),
            },
            FieldInfoData {
                name: "SetupParams",
                name_hash: 652484222,
                flags: MemberInfoFlags::new(0),
                field_type: "LevelSetup",
                rust_offset: offset_of!(LoadRequest, setup_params),
            },
            FieldInfoData {
                name: "LoadFromJuice",
                name_hash: 399807781,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LoadRequest, load_from_juice),
            },
        ],
    }),
    array_type: Some(LOADREQUEST_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LoadRequest {
    fn type_info(&self) -> &'static TypeInfo {
        LOADREQUEST_TYPE_INFO
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


pub static LOADREQUEST_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LoadRequest-Array",
    name_hash: 3762215718,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("LoadRequest"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LoadGameBeginLoadMessage {
}

pub trait LoadGameBeginLoadMessageTrait: TypeObject {
}

impl LoadGameBeginLoadMessageTrait for LoadGameBeginLoadMessage {
}

pub static LOADGAMEBEGINLOADMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LoadGameBeginLoadMessage",
    name_hash: 2988989607,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LoadGameBeginLoadMessage as Default>::default())),
            create_boxed: || Box::new(<LoadGameBeginLoadMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for LoadGameBeginLoadMessage {
    fn type_info(&self) -> &'static TypeInfo {
        LOADGAMEBEGINLOADMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct SaveGameBeginSaveMessage {
}

pub trait SaveGameBeginSaveMessageTrait: TypeObject {
}

impl SaveGameBeginSaveMessageTrait for SaveGameBeginSaveMessage {
}

pub static SAVEGAMEBEGINSAVEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SaveGameBeginSaveMessage",
    name_hash: 1709001287,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SaveGameBeginSaveMessage as Default>::default())),
            create_boxed: || Box::new(<SaveGameBeginSaveMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for SaveGameBeginSaveMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SAVEGAMEBEGINSAVEMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct BlueprintBundleStreamedInMessage {
}

pub trait BlueprintBundleStreamedInMessageTrait: TypeObject {
}

impl BlueprintBundleStreamedInMessageTrait for BlueprintBundleStreamedInMessage {
}

pub static BLUEPRINTBUNDLESTREAMEDINMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundleStreamedInMessage",
    name_hash: 127673807,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BlueprintBundleStreamedInMessage as Default>::default())),
            create_boxed: || Box::new(<BlueprintBundleStreamedInMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for BlueprintBundleStreamedInMessage {
    fn type_info(&self) -> &'static TypeInfo {
        BLUEPRINTBUNDLESTREAMEDINMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct SessionPlayerJoinedMessage {
}

pub trait SessionPlayerJoinedMessageTrait: TypeObject {
}

impl SessionPlayerJoinedMessageTrait for SessionPlayerJoinedMessage {
}

pub static SESSIONPLAYERJOINEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SessionPlayerJoinedMessage",
    name_hash: 3328564608,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SessionPlayerJoinedMessage as Default>::default())),
            create_boxed: || Box::new(<SessionPlayerJoinedMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for SessionPlayerJoinedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SESSIONPLAYERJOINEDMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct SessionPlayerAuthenticatedMessage {
}

pub trait SessionPlayerAuthenticatedMessageTrait: TypeObject {
}

impl SessionPlayerAuthenticatedMessageTrait for SessionPlayerAuthenticatedMessage {
}

pub static SESSIONPLAYERAUTHENTICATEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SessionPlayerAuthenticatedMessage",
    name_hash: 1559045354,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SessionPlayerAuthenticatedMessage as Default>::default())),
            create_boxed: || Box::new(<SessionPlayerAuthenticatedMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for SessionPlayerAuthenticatedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SESSIONPLAYERAUTHENTICATEDMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct SessionPlayerLeftMessage {
}

pub trait SessionPlayerLeftMessageTrait: TypeObject {
}

impl SessionPlayerLeftMessageTrait for SessionPlayerLeftMessage {
}

pub static SESSIONPLAYERLEFTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SessionPlayerLeftMessage",
    name_hash: 3667231768,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SessionPlayerLeftMessage as Default>::default())),
            create_boxed: || Box::new(<SessionPlayerLeftMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for SessionPlayerLeftMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SESSIONPLAYERLEFTMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct CoreGameTimerMessage {
}

pub trait CoreGameTimerMessageTrait: TypeObject {
}

impl CoreGameTimerMessageTrait for CoreGameTimerMessage {
}

pub static COREGAMETIMERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreGameTimerMessage",
    name_hash: 1931130620,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoreGameTimerMessage as Default>::default())),
            create_boxed: || Box::new(<CoreGameTimerMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreGameTimerMessage {
    fn type_info(&self) -> &'static TypeInfo {
        COREGAMETIMERMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct CoreExitIngameMessage {
}

pub trait CoreExitIngameMessageTrait: TypeObject {
}

impl CoreExitIngameMessageTrait for CoreExitIngameMessage {
}

pub static COREEXITINGAMEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreExitIngameMessage",
    name_hash: 4033069116,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoreExitIngameMessage as Default>::default())),
            create_boxed: || Box::new(<CoreExitIngameMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreExitIngameMessage {
    fn type_info(&self) -> &'static TypeInfo {
        COREEXITINGAMEMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct CoreEnteredIngameMessage {
}

pub trait CoreEnteredIngameMessageTrait: TypeObject {
}

impl CoreEnteredIngameMessageTrait for CoreEnteredIngameMessage {
}

pub static COREENTEREDINGAMEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreEnteredIngameMessage",
    name_hash: 1756357301,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoreEnteredIngameMessage as Default>::default())),
            create_boxed: || Box::new(<CoreEnteredIngameMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreEnteredIngameMessage {
    fn type_info(&self) -> &'static TypeInfo {
        COREENTEREDINGAMEMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct PerformanceClientNetworkMessage {
}

pub trait PerformanceClientNetworkMessageTrait: TypeObject {
}

impl PerformanceClientNetworkMessageTrait for PerformanceClientNetworkMessage {
}

pub static PERFORMANCECLIENTNETWORKMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerformanceClientNetworkMessage",
    name_hash: 1521267281,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PerformanceClientNetworkMessage as Default>::default())),
            create_boxed: || Box::new(<PerformanceClientNetworkMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for PerformanceClientNetworkMessage {
    fn type_info(&self) -> &'static TypeInfo {
        PERFORMANCECLIENTNETWORKMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct PerformanceServerNetworkMessage {
}

pub trait PerformanceServerNetworkMessageTrait: TypeObject {
}

impl PerformanceServerNetworkMessageTrait for PerformanceServerNetworkMessage {
}

pub static PERFORMANCESERVERNETWORKMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerformanceServerNetworkMessage",
    name_hash: 1874105229,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PerformanceServerNetworkMessage as Default>::default())),
            create_boxed: || Box::new(<PerformanceServerNetworkMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for PerformanceServerNetworkMessage {
    fn type_info(&self) -> &'static TypeInfo {
        PERFORMANCESERVERNETWORKMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct PerformanceServerMessage {
}

pub trait PerformanceServerMessageTrait: TypeObject {
}

impl PerformanceServerMessageTrait for PerformanceServerMessage {
}

pub static PERFORMANCESERVERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerformanceServerMessage",
    name_hash: 2739721875,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PerformanceServerMessage as Default>::default())),
            create_boxed: || Box::new(<PerformanceServerMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for PerformanceServerMessage {
    fn type_info(&self) -> &'static TypeInfo {
        PERFORMANCESERVERMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct ProfileOptionsSettingsSavedMessage {
}

pub trait ProfileOptionsSettingsSavedMessageTrait: TypeObject {
}

impl ProfileOptionsSettingsSavedMessageTrait for ProfileOptionsSettingsSavedMessage {
}

pub static PROFILEOPTIONSSETTINGSSAVEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionsSettingsSavedMessage",
    name_hash: 302237013,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ProfileOptionsSettingsSavedMessage as Default>::default())),
            create_boxed: || Box::new(<ProfileOptionsSettingsSavedMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ProfileOptionsSettingsSavedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        PROFILEOPTIONSSETTINGSSAVEDMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct ProfileOptionsSettingsPreSaveMessage {
}

pub trait ProfileOptionsSettingsPreSaveMessageTrait: TypeObject {
}

impl ProfileOptionsSettingsPreSaveMessageTrait for ProfileOptionsSettingsPreSaveMessage {
}

pub static PROFILEOPTIONSSETTINGSPRESAVEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionsSettingsPreSaveMessage",
    name_hash: 172750198,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ProfileOptionsSettingsPreSaveMessage as Default>::default())),
            create_boxed: || Box::new(<ProfileOptionsSettingsPreSaveMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ProfileOptionsSettingsPreSaveMessage {
    fn type_info(&self) -> &'static TypeInfo {
        PROFILEOPTIONSSETTINGSPRESAVEMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct ProfileOptionsSettingsLoadedMessage {
}

pub trait ProfileOptionsSettingsLoadedMessageTrait: TypeObject {
}

impl ProfileOptionsSettingsLoadedMessageTrait for ProfileOptionsSettingsLoadedMessage {
}

pub static PROFILEOPTIONSSETTINGSLOADEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionsSettingsLoadedMessage",
    name_hash: 4147248855,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ProfileOptionsSettingsLoadedMessage as Default>::default())),
            create_boxed: || Box::new(<ProfileOptionsSettingsLoadedMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ProfileOptionsSettingsLoadedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        PROFILEOPTIONSSETTINGSLOADEDMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct ProfileOptionsApplyMessage {
}

pub trait ProfileOptionsApplyMessageTrait: TypeObject {
}

impl ProfileOptionsApplyMessageTrait for ProfileOptionsApplyMessage {
}

pub static PROFILEOPTIONSAPPLYMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionsApplyMessage",
    name_hash: 259176353,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ProfileOptionsApplyMessage as Default>::default())),
            create_boxed: || Box::new(<ProfileOptionsApplyMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ProfileOptionsApplyMessage {
    fn type_info(&self) -> &'static TypeInfo {
        PROFILEOPTIONSAPPLYMESSAGE_TYPE_INFO
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

#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ProfileOptionsLoadStatus {
    #[default]
    ProfileOptionsLoadStatus_Empty = 0,
    ProfileOptionsLoadStatus_Succeeded = 1,
    ProfileOptionsLoadStatus_Corrupt = 2,
}

pub static PROFILEOPTIONSLOADSTATUS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionsLoadStatus",
    name_hash: 541428812,
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(PROFILEOPTIONSLOADSTATUS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ProfileOptionsLoadStatus {
    fn type_info(&self) -> &'static TypeInfo {
        PROFILEOPTIONSLOADSTATUS_TYPE_INFO
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


pub static PROFILEOPTIONSLOADSTATUS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionsLoadStatus-Array",
    name_hash: 1760521464,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ProfileOptionsLoadStatus"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct StatisticsEventMessageBase {
}

pub trait StatisticsEventMessageBaseTrait: TypeObject {
}

impl StatisticsEventMessageBaseTrait for StatisticsEventMessageBase {
}

pub static STATISTICSEVENTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StatisticsEventMessageBase",
    name_hash: 324775570,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StatisticsEventMessageBase as Default>::default())),
            create_boxed: || Box::new(<StatisticsEventMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for StatisticsEventMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        STATISTICSEVENTMESSAGEBASE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct NetworkDisconnectedMessage {
}

pub trait NetworkDisconnectedMessageTrait: TypeObject {
}

impl NetworkDisconnectedMessageTrait for NetworkDisconnectedMessage {
}

pub static NETWORKDISCONNECTEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkDisconnectedMessage",
    name_hash: 778899633,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkDisconnectedMessage as Default>::default())),
            create_boxed: || Box::new(<NetworkDisconnectedMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkDisconnectedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKDISCONNECTEDMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct NetworkConnectedMessage {
}

pub trait NetworkConnectedMessageTrait: TypeObject {
}

impl NetworkConnectedMessageTrait for NetworkConnectedMessage {
}

pub static NETWORKCONNECTEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkConnectedMessage",
    name_hash: 3035850703,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkConnectedMessage as Default>::default())),
            create_boxed: || Box::new(<NetworkConnectedMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkConnectedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKCONNECTEDMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct TransformParameterEvent {
    pub _glacier_base: super::entity::EntityEvent,
}

pub trait TransformParameterEventTrait: super::entity::EntityEventTrait {
}

impl TransformParameterEventTrait for TransformParameterEvent {
}

impl super::entity::EntityEventTrait for TransformParameterEvent {
}

pub static TRANSFORMPARAMETEREVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformParameterEvent",
    name_hash: 3420352140,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYEVENT_TYPE_INFO),
        super_class_offset: offset_of!(TransformParameterEvent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TransformParameterEvent as Default>::default())),
            create_boxed: || Box::new(<TransformParameterEvent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(TRANSFORMPARAMETEREVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TransformParameterEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TRANSFORMPARAMETEREVENT_TYPE_INFO
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


pub static TRANSFORMPARAMETEREVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformParameterEvent-Array",
    name_hash: 2247425720,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("TransformParameterEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PlayerEventBase {
    pub _glacier_base: super::entity::EntityEvent,
}

pub trait PlayerEventBaseTrait: super::entity::EntityEventTrait {
}

impl PlayerEventBaseTrait for PlayerEventBase {
}

impl super::entity::EntityEventTrait for PlayerEventBase {
}

pub static PLAYEREVENTBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayerEventBase",
    name_hash: 1198427439,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYEVENT_TYPE_INFO),
        super_class_offset: offset_of!(PlayerEventBase, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PlayerEventBase as Default>::default())),
            create_boxed: || Box::new(<PlayerEventBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(PLAYEREVENTBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PlayerEventBase {
    fn type_info(&self) -> &'static TypeInfo {
        PLAYEREVENTBASE_TYPE_INFO
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


pub static PLAYEREVENTBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayerEventBase-Array",
    name_hash: 3211860635,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("PlayerEventBase"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct IntParameterEvent {
    pub _glacier_base: super::entity::EntityEvent,
}

pub trait IntParameterEventTrait: super::entity::EntityEventTrait {
}

impl IntParameterEventTrait for IntParameterEvent {
}

impl super::entity::EntityEventTrait for IntParameterEvent {
}

pub static INTPARAMETEREVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntParameterEvent",
    name_hash: 896005075,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYEVENT_TYPE_INFO),
        super_class_offset: offset_of!(IntParameterEvent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IntParameterEvent as Default>::default())),
            create_boxed: || Box::new(<IntParameterEvent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(INTPARAMETEREVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IntParameterEvent {
    fn type_info(&self) -> &'static TypeInfo {
        INTPARAMETEREVENT_TYPE_INFO
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


pub static INTPARAMETEREVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntParameterEvent-Array",
    name_hash: 719601383,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("IntParameterEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct FloatParameterEvent {
    pub _glacier_base: super::entity::EntityEvent,
}

pub trait FloatParameterEventTrait: super::entity::EntityEventTrait {
}

impl FloatParameterEventTrait for FloatParameterEvent {
}

impl super::entity::EntityEventTrait for FloatParameterEvent {
}

pub static FLOATPARAMETEREVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatParameterEvent",
    name_hash: 3642931568,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYEVENT_TYPE_INFO),
        super_class_offset: offset_of!(FloatParameterEvent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FloatParameterEvent as Default>::default())),
            create_boxed: || Box::new(<FloatParameterEvent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(FLOATPARAMETEREVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FloatParameterEvent {
    fn type_info(&self) -> &'static TypeInfo {
        FLOATPARAMETEREVENT_TYPE_INFO
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


pub static FLOATPARAMETEREVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatParameterEvent-Array",
    name_hash: 4247091012,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("FloatParameterEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EntityParameterEvent {
    pub _glacier_base: super::entity::EntityEvent,
}

pub trait EntityParameterEventTrait: super::entity::EntityEventTrait {
}

impl EntityParameterEventTrait for EntityParameterEvent {
}

impl super::entity::EntityEventTrait for EntityParameterEvent {
}

pub static ENTITYPARAMETEREVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityParameterEvent",
    name_hash: 2976736187,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYEVENT_TYPE_INFO),
        super_class_offset: offset_of!(EntityParameterEvent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EntityParameterEvent as Default>::default())),
            create_boxed: || Box::new(<EntityParameterEvent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(ENTITYPARAMETEREVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EntityParameterEvent {
    fn type_info(&self) -> &'static TypeInfo {
        ENTITYPARAMETEREVENT_TYPE_INFO
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


pub static ENTITYPARAMETEREVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityParameterEvent-Array",
    name_hash: 2318002959,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("EntityParameterEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ComponentParameterEvent {
    pub _glacier_base: super::entity::EntityEvent,
}

pub trait ComponentParameterEventTrait: super::entity::EntityEventTrait {
}

impl ComponentParameterEventTrait for ComponentParameterEvent {
}

impl super::entity::EntityEventTrait for ComponentParameterEvent {
}

pub static COMPONENTPARAMETEREVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ComponentParameterEvent",
    name_hash: 1608172975,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYEVENT_TYPE_INFO),
        super_class_offset: offset_of!(ComponentParameterEvent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ComponentParameterEvent as Default>::default())),
            create_boxed: || Box::new(<ComponentParameterEvent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(COMPONENTPARAMETEREVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ComponentParameterEvent {
    fn type_info(&self) -> &'static TypeInfo {
        COMPONENTPARAMETEREVENT_TYPE_INFO
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


pub static COMPONENTPARAMETEREVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ComponentParameterEvent-Array",
    name_hash: 3007689499,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ComponentParameterEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CollisionEvent {
    pub _glacier_base: super::entity::EntityEvent,
}

pub trait CollisionEventTrait: super::entity::EntityEventTrait {
}

impl CollisionEventTrait for CollisionEvent {
}

impl super::entity::EntityEventTrait for CollisionEvent {
}

pub static COLLISIONEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CollisionEvent",
    name_hash: 729471991,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYEVENT_TYPE_INFO),
        super_class_offset: offset_of!(CollisionEvent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CollisionEvent as Default>::default())),
            create_boxed: || Box::new(<CollisionEvent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(COLLISIONEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CollisionEvent {
    fn type_info(&self) -> &'static TypeInfo {
        COLLISIONEVENT_TYPE_INFO
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


pub static COLLISIONEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CollisionEvent-Array",
    name_hash: 210036163,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("CollisionEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct HealthEntityFactory {
    pub _glacier_base: super::physics::IglooEntityFactory,
}

pub trait HealthEntityFactoryTrait: super::physics::IglooEntityFactoryTrait {
}

impl HealthEntityFactoryTrait for HealthEntityFactory {
}

impl super::physics::IglooEntityFactoryTrait for HealthEntityFactory {
}

pub static HEALTHENTITYFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthEntityFactory",
    name_hash: 3570472310,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::IGLOOENTITYFACTORY_TYPE_INFO),
        super_class_offset: offset_of!(HealthEntityFactory, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<HealthEntityFactory as Default>::default())),
            create_boxed: || Box::new(<HealthEntityFactory as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(HEALTHENTITYFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for HealthEntityFactory {
    fn type_info(&self) -> &'static TypeInfo {
        HEALTHENTITYFACTORY_TYPE_INFO
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


pub static HEALTHENTITYFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthEntityFactory-Array",
    name_hash: 739555906,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("HealthEntityFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct HealthComponent {
    pub _glacier_base: super::entity::Component,
}

pub trait HealthComponentTrait: super::entity::ComponentTrait {
}

impl HealthComponentTrait for HealthComponent {
}

impl super::entity::ComponentTrait for HealthComponent {
}

impl super::entity::EntityBusPeerTrait for HealthComponent {
}

pub static HEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthComponent",
    name_hash: 2458742998,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::COMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(HealthComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<HealthComponent as Default>::default())),
            create_boxed: || Box::new(<HealthComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(HEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for HealthComponent {
    fn type_info(&self) -> &'static TypeInfo {
        HEALTHCOMPONENT_TYPE_INFO
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


pub static HEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthComponent-Array",
    name_hash: 3070006882,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("HealthComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientDamageInfo {
    pub _glacier_base: DamageInfo,
}

pub trait ClientDamageInfoTrait: DamageInfoTrait {
}

impl ClientDamageInfoTrait for ClientDamageInfo {
}

impl DamageInfoTrait for ClientDamageInfo {
}

pub static CLIENTDAMAGEINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDamageInfo",
    name_hash: 2673077017,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DAMAGEINFO_TYPE_INFO),
        super_class_offset: offset_of!(ClientDamageInfo, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientDamageInfo as Default>::default())),
            create_boxed: || Box::new(<ClientDamageInfo as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTDAMAGEINFO_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientDamageInfo {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTDAMAGEINFO_TYPE_INFO
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


pub static CLIENTDAMAGEINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDamageInfo-Array",
    name_hash: 2043922605,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ClientDamageInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerDamageInfo {
    pub _glacier_base: DamageInfo,
}

pub trait ServerDamageInfoTrait: DamageInfoTrait {
}

impl ServerDamageInfoTrait for ServerDamageInfo {
}

impl DamageInfoTrait for ServerDamageInfo {
}

pub static SERVERDAMAGEINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDamageInfo",
    name_hash: 2530100293,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DAMAGEINFO_TYPE_INFO),
        super_class_offset: offset_of!(ServerDamageInfo, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerDamageInfo as Default>::default())),
            create_boxed: || Box::new(<ServerDamageInfo as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERDAMAGEINFO_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerDamageInfo {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERDAMAGEINFO_TYPE_INFO
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


pub static SERVERDAMAGEINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDamageInfo-Array",
    name_hash: 3907361905,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ServerDamageInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DamageInfo {
}

pub trait DamageInfoTrait: TypeObject {
}

impl DamageInfoTrait for DamageInfo {
}

pub static DAMAGEINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DamageInfo",
    name_hash: 3580830944,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DamageInfo as Default>::default())),
            create_boxed: || Box::new(<DamageInfo as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DAMAGEINFO_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DamageInfo {
    fn type_info(&self) -> &'static TypeInfo {
        DAMAGEINFO_TYPE_INFO
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


pub static DAMAGEINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DamageInfo-Array",
    name_hash: 1197587156,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("DamageInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DefaultHealthComponent {
    pub _glacier_base: HealthComponent,
}

pub trait DefaultHealthComponentTrait: HealthComponentTrait {
}

impl DefaultHealthComponentTrait for DefaultHealthComponent {
}

impl HealthComponentTrait for DefaultHealthComponent {
}

impl super::entity::ComponentTrait for DefaultHealthComponent {
}

impl super::entity::EntityBusPeerTrait for DefaultHealthComponent {
}

pub static DEFAULTHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DefaultHealthComponent",
    name_hash: 1308566109,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(HEALTHCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(DefaultHealthComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DefaultHealthComponent as Default>::default())),
            create_boxed: || Box::new(<DefaultHealthComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DEFAULTHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DefaultHealthComponent {
    fn type_info(&self) -> &'static TypeInfo {
        DEFAULTHEALTHCOMPONENT_TYPE_INFO
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


pub static DEFAULTHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DefaultHealthComponent-Array",
    name_hash: 1456938601,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("DefaultHealthComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct GameWorldRayCaster {
}

pub trait GameWorldRayCasterTrait: TypeObject {
}

impl GameWorldRayCasterTrait for GameWorldRayCaster {
}

pub static GAMEWORLDRAYCASTER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameWorldRayCaster",
    name_hash: 2166445169,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GameWorldRayCaster as Default>::default())),
            create_boxed: || Box::new(<GameWorldRayCaster as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(GAMEWORLDRAYCASTER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for GameWorldRayCaster {
    fn type_info(&self) -> &'static TypeInfo {
        GAMEWORLDRAYCASTER_TYPE_INFO
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


pub static GAMEWORLDRAYCASTER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameWorldRayCaster-Array",
    name_hash: 2505561925,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("GameWorldRayCaster"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct GameComponentEntity {
    pub _glacier_base: super::entity::ComponentEntity,
}

pub trait GameComponentEntityTrait: super::entity::ComponentEntityTrait {
}

impl GameComponentEntityTrait for GameComponentEntity {
}

impl super::entity::ComponentEntityTrait for GameComponentEntity {
}

impl super::entity::SpatialEntityTrait for GameComponentEntity {
}

impl super::entity::EntityTrait for GameComponentEntity {
}

impl super::entity::EntityBusPeerTrait for GameComponentEntity {
}

pub static GAMECOMPONENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameComponentEntity",
    name_hash: 1845345919,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::COMPONENTENTITY_TYPE_INFO),
        super_class_offset: offset_of!(GameComponentEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GameComponentEntity as Default>::default())),
            create_boxed: || Box::new(<GameComponentEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(GAMECOMPONENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for GameComponentEntity {
    fn type_info(&self) -> &'static TypeInfo {
        GAMECOMPONENTENTITY_TYPE_INFO
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


pub static GAMECOMPONENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameComponentEntity-Array",
    name_hash: 1371997259,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("GameComponentEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct GameComponent {
    pub _glacier_base: super::entity::Component,
}

pub trait GameComponentTrait: super::entity::ComponentTrait {
}

impl GameComponentTrait for GameComponent {
}

impl super::entity::ComponentTrait for GameComponent {
}

impl super::entity::EntityBusPeerTrait for GameComponent {
}

pub static GAMECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameComponent",
    name_hash: 1051033636,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::COMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(GameComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GameComponent as Default>::default())),
            create_boxed: || Box::new(<GameComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(GAMECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for GameComponent {
    fn type_info(&self) -> &'static TypeInfo {
        GAMECOMPONENT_TYPE_INFO
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


pub static GAMECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameComponent-Array",
    name_hash: 2638427024,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("GameComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PartComponentCreatedEntityInfo {
}

pub trait PartComponentCreatedEntityInfoTrait: TypeObject {
}

impl PartComponentCreatedEntityInfoTrait for PartComponentCreatedEntityInfo {
}

pub static PARTCOMPONENTCREATEDENTITYINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartComponentCreatedEntityInfo",
    name_hash: 3384888936,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PartComponentCreatedEntityInfo as Default>::default())),
            create_boxed: || Box::new(<PartComponentCreatedEntityInfo as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(PARTCOMPONENTCREATEDENTITYINFO_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PartComponentCreatedEntityInfo {
    fn type_info(&self) -> &'static TypeInfo {
        PARTCOMPONENTCREATEDENTITYINFO_TYPE_INFO
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


pub static PARTCOMPONENTCREATEDENTITYINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartComponentCreatedEntityInfo-Array",
    name_hash: 2223792220,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("PartComponentCreatedEntityInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SubView {
}

pub trait SubViewTrait: TypeObject {
}

impl SubViewTrait for SubView {
}

pub static SUBVIEW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubView",
    name_hash: 2135521228,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SubView as Default>::default())),
            create_boxed: || Box::new(<SubView as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SUBVIEW_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SubView {
    fn type_info(&self) -> &'static TypeInfo {
        SUBVIEW_TYPE_INFO
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


pub static SUBVIEW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubView-Array",
    name_hash: 4044748920,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("SubView"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ShaderParameterEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ShaderParameterEntityTrait: super::entity::EntityTrait {
}

impl ShaderParameterEntityTrait for ShaderParameterEntity {
}

impl super::entity::EntityTrait for ShaderParameterEntity {
}

impl super::entity::EntityBusPeerTrait for ShaderParameterEntity {
}

pub static SHADERPARAMETERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterEntity",
    name_hash: 3425409310,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ShaderParameterEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ShaderParameterEntity as Default>::default())),
            create_boxed: || Box::new(<ShaderParameterEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SHADERPARAMETERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ShaderParameterEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SHADERPARAMETERENTITY_TYPE_INFO
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


pub static SHADERPARAMETERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterEntity-Array",
    name_hash: 3226929706,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ShaderParameterEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct NetworkServerDiagnosticsMessage {
}

pub trait NetworkServerDiagnosticsMessageTrait: TypeObject {
}

impl NetworkServerDiagnosticsMessageTrait for NetworkServerDiagnosticsMessage {
}

pub static NETWORKSERVERDIAGNOSTICSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkServerDiagnosticsMessage",
    name_hash: 3986037025,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkServerDiagnosticsMessage as Default>::default())),
            create_boxed: || Box::new(<NetworkServerDiagnosticsMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkServerDiagnosticsMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKSERVERDIAGNOSTICSMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct NetworkCameraReplayMessage {
}

pub trait NetworkCameraReplayMessageTrait: TypeObject {
}

impl NetworkCameraReplayMessageTrait for NetworkCameraReplayMessage {
}

pub static NETWORKCAMERAREPLAYMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkCameraReplayMessage",
    name_hash: 2070077786,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkCameraReplayMessage as Default>::default())),
            create_boxed: || Box::new(<NetworkCameraReplayMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkCameraReplayMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKCAMERAREPLAYMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct NetworkCameraFreeCameraMessage {
}

pub trait NetworkCameraFreeCameraMessageTrait: TypeObject {
}

impl NetworkCameraFreeCameraMessageTrait for NetworkCameraFreeCameraMessage {
}

pub static NETWORKCAMERAFREECAMERAMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkCameraFreeCameraMessage",
    name_hash: 1484442148,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkCameraFreeCameraMessage as Default>::default())),
            create_boxed: || Box::new(<NetworkCameraFreeCameraMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for NetworkCameraFreeCameraMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKCAMERAFREECAMERAMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct NetworkSynchronizeInternetSimulationStateMessage {
}

pub trait NetworkSynchronizeInternetSimulationStateMessageTrait: TypeObject {
}

impl NetworkSynchronizeInternetSimulationStateMessageTrait for NetworkSynchronizeInternetSimulationStateMessage {
}

pub static NETWORKSYNCHRONIZEINTERNETSIMULATIONSTATEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkSynchronizeInternetSimulationStateMessage",
    name_hash: 2869440677,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkSynchronizeInternetSimulationStateMessage as Default>::default())),
            create_boxed: || Box::new(<NetworkSynchronizeInternetSimulationStateMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkSynchronizeInternetSimulationStateMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKSYNCHRONIZEINTERNETSIMULATIONSTATEMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct NetworkChangeGameSettingMessage {
}

pub trait NetworkChangeGameSettingMessageTrait: TypeObject {
}

impl NetworkChangeGameSettingMessageTrait for NetworkChangeGameSettingMessage {
}

pub static NETWORKCHANGEGAMESETTINGMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkChangeGameSettingMessage",
    name_hash: 3263281166,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkChangeGameSettingMessage as Default>::default())),
            create_boxed: || Box::new(<NetworkChangeGameSettingMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkChangeGameSettingMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKCHANGEGAMESETTINGMESSAGE_TYPE_INFO
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

#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ChangeGameSettingType {
    #[default]
    CGSNone = 0,
    CGSInvertVerticalLookAxis = 1,
    CGSCount = 2,
}

pub static CHANGEGAMESETTINGTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ChangeGameSettingType",
    name_hash: 146895235,
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(CHANGEGAMESETTINGTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ChangeGameSettingType {
    fn type_info(&self) -> &'static TypeInfo {
        CHANGEGAMESETTINGTYPE_TYPE_INFO
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


pub static CHANGEGAMESETTINGTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ChangeGameSettingType-Array",
    name_hash: 2182251575,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ChangeGameSettingType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct NetworkDestroyLocalPlayerMessage {
}

pub trait NetworkDestroyLocalPlayerMessageTrait: TypeObject {
}

impl NetworkDestroyLocalPlayerMessageTrait for NetworkDestroyLocalPlayerMessage {
}

pub static NETWORKDESTROYLOCALPLAYERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkDestroyLocalPlayerMessage",
    name_hash: 2338651084,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkDestroyLocalPlayerMessage as Default>::default())),
            create_boxed: || Box::new(<NetworkDestroyLocalPlayerMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkDestroyLocalPlayerMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKDESTROYLOCALPLAYERMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct NetworkCreatePlayerMessage {
}

pub trait NetworkCreatePlayerMessageTrait: TypeObject {
}

impl NetworkCreatePlayerMessageTrait for NetworkCreatePlayerMessage {
}

pub static NETWORKCREATEPLAYERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkCreatePlayerMessage",
    name_hash: 3523530503,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkCreatePlayerMessage as Default>::default())),
            create_boxed: || Box::new(<NetworkCreatePlayerMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkCreatePlayerMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKCREATEPLAYERMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct NetworkTinyEventMessage {
}

pub trait NetworkTinyEventMessageTrait: TypeObject {
}

impl NetworkTinyEventMessageTrait for NetworkTinyEventMessage {
}

pub static NETWORKTINYEVENTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkTinyEventMessage",
    name_hash: 1458874806,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkTinyEventMessage as Default>::default())),
            create_boxed: || Box::new(<NetworkTinyEventMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkTinyEventMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKTINYEVENTMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct NetworkLoadLevelMessage {
}

pub trait NetworkLoadLevelMessageTrait: TypeObject {
}

impl NetworkLoadLevelMessageTrait for NetworkLoadLevelMessage {
}

pub static NETWORKLOADLEVELMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkLoadLevelMessage",
    name_hash: 1292955648,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkLoadLevelMessage as Default>::default())),
            create_boxed: || Box::new(<NetworkLoadLevelMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkLoadLevelMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKLOADLEVELMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct NetworkLevelLoadedAckMessage {
}

pub trait NetworkLevelLoadedAckMessageTrait: TypeObject {
}

impl NetworkLevelLoadedAckMessageTrait for NetworkLevelLoadedAckMessage {
}

pub static NETWORKLEVELLOADEDACKMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkLevelLoadedAckMessage",
    name_hash: 113453192,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkLevelLoadedAckMessage as Default>::default())),
            create_boxed: || Box::new(<NetworkLevelLoadedAckMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkLevelLoadedAckMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKLEVELLOADEDACKMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct NetworkTimeSyncMessage {
}

pub trait NetworkTimeSyncMessageTrait: TypeObject {
}

impl NetworkTimeSyncMessageTrait for NetworkTimeSyncMessage {
}

pub static NETWORKTIMESYNCMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkTimeSyncMessage",
    name_hash: 1132805922,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkTimeSyncMessage as Default>::default())),
            create_boxed: || Box::new(<NetworkTimeSyncMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkTimeSyncMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKTIMESYNCMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct LoadLevelInfo {
    pub setup: super::entity::LevelSetup,
    pub blueprint_bundle_preloads: Vec<BoxedTypeObject /* BlueprintBundlePreloadInfo */>,
    pub level_sequence_number: u32,
}

pub trait LoadLevelInfoTrait: TypeObject {
    fn setup(&self) -> &super::entity::LevelSetup;
    fn setup_mut(&mut self) -> &mut super::entity::LevelSetup;
    fn blueprint_bundle_preloads(&self) -> &Vec<BoxedTypeObject /* BlueprintBundlePreloadInfo */>;
    fn blueprint_bundle_preloads_mut(&mut self) -> &mut Vec<BoxedTypeObject /* BlueprintBundlePreloadInfo */>;
    fn level_sequence_number(&self) -> &u32;
    fn level_sequence_number_mut(&mut self) -> &mut u32;
}

impl LoadLevelInfoTrait for LoadLevelInfo {
    fn setup(&self) -> &super::entity::LevelSetup {
        &self.setup
    }
    fn setup_mut(&mut self) -> &mut super::entity::LevelSetup {
        &mut self.setup
    }
    fn blueprint_bundle_preloads(&self) -> &Vec<BoxedTypeObject /* BlueprintBundlePreloadInfo */> {
        &self.blueprint_bundle_preloads
    }
    fn blueprint_bundle_preloads_mut(&mut self) -> &mut Vec<BoxedTypeObject /* BlueprintBundlePreloadInfo */> {
        &mut self.blueprint_bundle_preloads
    }
    fn level_sequence_number(&self) -> &u32 {
        &self.level_sequence_number
    }
    fn level_sequence_number_mut(&mut self) -> &mut u32 {
        &mut self.level_sequence_number
    }
}

pub static LOADLEVELINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LoadLevelInfo",
    name_hash: 3347319547,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LoadLevelInfo as Default>::default())),
            create_boxed: || Box::new(<LoadLevelInfo as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Setup",
                name_hash: 231277282,
                flags: MemberInfoFlags::new(0),
                field_type: "LevelSetup",
                rust_offset: offset_of!(LoadLevelInfo, setup),
            },
            FieldInfoData {
                name: "BlueprintBundlePreloads",
                name_hash: 3066680716,
                flags: MemberInfoFlags::new(144),
                field_type: "BlueprintBundlePreloadInfo-Array",
                rust_offset: offset_of!(LoadLevelInfo, blueprint_bundle_preloads),
            },
            FieldInfoData {
                name: "LevelSequenceNumber",
                name_hash: 4171232943,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(LoadLevelInfo, level_sequence_number),
            },
        ],
    }),
    array_type: Some(LOADLEVELINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LoadLevelInfo {
    fn type_info(&self) -> &'static TypeInfo {
        LOADLEVELINFO_TYPE_INFO
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


pub static LOADLEVELINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LoadLevelInfo-Array",
    name_hash: 3719627471,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("LoadLevelInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct BlueprintBundlePreloadInfo {
    pub name: String,
    pub compartment: i32,
    pub parent_compartment: i32,
}

pub trait BlueprintBundlePreloadInfoTrait: TypeObject {
    fn name(&self) -> &String;
    fn name_mut(&mut self) -> &mut String;
    fn compartment(&self) -> &i32;
    fn compartment_mut(&mut self) -> &mut i32;
    fn parent_compartment(&self) -> &i32;
    fn parent_compartment_mut(&mut self) -> &mut i32;
}

impl BlueprintBundlePreloadInfoTrait for BlueprintBundlePreloadInfo {
    fn name(&self) -> &String {
        &self.name
    }
    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    fn compartment(&self) -> &i32 {
        &self.compartment
    }
    fn compartment_mut(&mut self) -> &mut i32 {
        &mut self.compartment
    }
    fn parent_compartment(&self) -> &i32 {
        &self.parent_compartment
    }
    fn parent_compartment_mut(&mut self) -> &mut i32 {
        &mut self.parent_compartment
    }
}

pub static BLUEPRINTBUNDLEPRELOADINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundlePreloadInfo",
    name_hash: 2740453201,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BlueprintBundlePreloadInfo as Default>::default())),
            create_boxed: || Box::new(<BlueprintBundlePreloadInfo as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                name_hash: 2088949890,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(BlueprintBundlePreloadInfo, name),
            },
            FieldInfoData {
                name: "Compartment",
                name_hash: 1500500641,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(BlueprintBundlePreloadInfo, compartment),
            },
            FieldInfoData {
                name: "ParentCompartment",
                name_hash: 1376031133,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(BlueprintBundlePreloadInfo, parent_compartment),
            },
        ],
    }),
    array_type: Some(BLUEPRINTBUNDLEPRELOADINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BlueprintBundlePreloadInfo {
    fn type_info(&self) -> &'static TypeInfo {
        BLUEPRINTBUNDLEPRELOADINFO_TYPE_INFO
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


pub static BLUEPRINTBUNDLEPRELOADINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundlePreloadInfo-Array",
    name_hash: 4041388389,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("BlueprintBundlePreloadInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum TinyEvent {
    #[default]
    TinyEvent_AckTimeSyncDone = 1,
    TinyEvent_AckLevelLinked = 2,
    TinyEvent_AckLevelRestarted = 3,
    TinyEvent_AckEnterPatchRecvState = 4,
    TinyEvent_AckAuthentication = 5,
    TinyEvent_NackAuthentication = 6,
    TinyEvent_AckExitLevel = 7,
    TinyEvent_CmdLinkLevel = 8,
    TinyEvent_CmdEnterPatchRecvState = 9,
    TinyEvent_CmdExitLevel = 10,
    TinyEvent_CmdContinueLevel = 11,
    TinyEvent_StatControllableRubberbanding = 12,
    TinyEvent_StatWorldRubberbanding = 13,
}

pub static TINYEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TinyEvent",
    name_hash: 3341734595,
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(TINYEVENT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TinyEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TINYEVENT_TYPE_INFO
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


pub static TINYEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TinyEvent-Array",
    name_hash: 2760087031,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("TinyEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientToServerLocalPlayer {
}

pub trait ClientToServerLocalPlayerTrait: TypeObject {
}

impl ClientToServerLocalPlayerTrait for ClientToServerLocalPlayer {
}

pub static CLIENTTOSERVERLOCALPLAYER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientToServerLocalPlayer",
    name_hash: 2792198492,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientToServerLocalPlayer as Default>::default())),
            create_boxed: || Box::new(<ClientToServerLocalPlayer as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTTOSERVERLOCALPLAYER_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ClientToServerLocalPlayer {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTTOSERVERLOCALPLAYER_TYPE_INFO
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


pub static CLIENTTOSERVERLOCALPLAYER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientToServerLocalPlayer-Array",
    name_hash: 2681862632,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ClientToServerLocalPlayer"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientToServerLocalPlayerView {
    pub is_active: bool,
    pub camera_pos: super::core::Vec3,
    pub camera_forward: super::core::Vec3,
    pub camera_fov: u32,
}

pub trait ClientToServerLocalPlayerViewTrait: TypeObject {
    fn is_active(&self) -> &bool;
    fn is_active_mut(&mut self) -> &mut bool;
    fn camera_pos(&self) -> &super::core::Vec3;
    fn camera_pos_mut(&mut self) -> &mut super::core::Vec3;
    fn camera_forward(&self) -> &super::core::Vec3;
    fn camera_forward_mut(&mut self) -> &mut super::core::Vec3;
    fn camera_fov(&self) -> &u32;
    fn camera_fov_mut(&mut self) -> &mut u32;
}

impl ClientToServerLocalPlayerViewTrait for ClientToServerLocalPlayerView {
    fn is_active(&self) -> &bool {
        &self.is_active
    }
    fn is_active_mut(&mut self) -> &mut bool {
        &mut self.is_active
    }
    fn camera_pos(&self) -> &super::core::Vec3 {
        &self.camera_pos
    }
    fn camera_pos_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.camera_pos
    }
    fn camera_forward(&self) -> &super::core::Vec3 {
        &self.camera_forward
    }
    fn camera_forward_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.camera_forward
    }
    fn camera_fov(&self) -> &u32 {
        &self.camera_fov
    }
    fn camera_fov_mut(&mut self) -> &mut u32 {
        &mut self.camera_fov
    }
}

pub static CLIENTTOSERVERLOCALPLAYERVIEW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientToServerLocalPlayerView",
    name_hash: 1528909649,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientToServerLocalPlayerView as Default>::default())),
            create_boxed: || Box::new(<ClientToServerLocalPlayerView as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "IsActive",
                name_hash: 1010522579,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClientToServerLocalPlayerView, is_active),
            },
            FieldInfoData {
                name: "CameraPos",
                name_hash: 1789521808,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(ClientToServerLocalPlayerView, camera_pos),
            },
            FieldInfoData {
                name: "CameraForward",
                name_hash: 3988314535,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(ClientToServerLocalPlayerView, camera_forward),
            },
            FieldInfoData {
                name: "CameraFov",
                name_hash: 1789541379,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ClientToServerLocalPlayerView, camera_fov),
            },
        ],
    }),
    array_type: Some(CLIENTTOSERVERLOCALPLAYERVIEW_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ClientToServerLocalPlayerView {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTTOSERVERLOCALPLAYERVIEW_TYPE_INFO
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


pub static CLIENTTOSERVERLOCALPLAYERVIEW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientToServerLocalPlayerView-Array",
    name_hash: 3462922597,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ClientToServerLocalPlayerView"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ClientToServerConstants {
    #[default]
    ClientToServerConstants_LocalPlayerCount = 8,
    ClientToServerConstants_LocalPlayerViewCount = 6,
}

pub static CLIENTTOSERVERCONSTANTS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientToServerConstants",
    name_hash: 158759055,
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(CLIENTTOSERVERCONSTANTS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ClientToServerConstants {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTTOSERVERCONSTANTS_TYPE_INFO
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


pub static CLIENTTOSERVERCONSTANTS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientToServerConstants-Array",
    name_hash: 2985284923,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ClientToServerConstants"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct InputModifierEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub enabled: bool,
    pub action: i32,
    pub offset: f32,
    pub scale: f32,
}

pub trait InputModifierEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn enabled(&self) -> &bool;
    fn enabled_mut(&mut self) -> &mut bool;
    fn action(&self) -> &i32;
    fn action_mut(&mut self) -> &mut i32;
    fn offset(&self) -> &f32;
    fn offset_mut(&mut self) -> &mut f32;
    fn scale(&self) -> &f32;
    fn scale_mut(&mut self) -> &mut f32;
}

impl InputModifierEntityDataTrait for InputModifierEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
    fn action(&self) -> &i32 {
        &self.action
    }
    fn action_mut(&mut self) -> &mut i32 {
        &mut self.action
    }
    fn offset(&self) -> &f32 {
        &self.offset
    }
    fn offset_mut(&mut self) -> &mut f32 {
        &mut self.offset
    }
    fn scale(&self) -> &f32 {
        &self.scale
    }
    fn scale_mut(&mut self) -> &mut f32 {
        &mut self.scale
    }
}

impl super::entity::EntityDataTrait for InputModifierEntityData {
}

impl super::entity::GameObjectDataTrait for InputModifierEntityData {
}

impl super::core::DataBusPeerTrait for InputModifierEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for InputModifierEntityData {
}

impl super::core::DataContainerTrait for InputModifierEntityData {
}

pub static INPUTMODIFIERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputModifierEntityData",
    name_hash: 3817738703,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(InputModifierEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InputModifierEntityData as Default>::default())),
            create_boxed: || Box::new(<InputModifierEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                name_hash: 229961746,
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(InputModifierEntityData, realm),
            },
            FieldInfoData {
                name: "Enabled",
                name_hash: 2662400,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(InputModifierEntityData, enabled),
            },
            FieldInfoData {
                name: "Action",
                name_hash: 2484178491,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(InputModifierEntityData, action),
            },
            FieldInfoData {
                name: "Offset",
                name_hash: 2871410728,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InputModifierEntityData, offset),
            },
            FieldInfoData {
                name: "Scale",
                name_hash: 231223453,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InputModifierEntityData, scale),
            },
        ],
    }),
    array_type: Some(INPUTMODIFIERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for InputModifierEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        INPUTMODIFIERENTITYDATA_TYPE_INFO
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


pub static INPUTMODIFIERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputModifierEntityData-Array",
    name_hash: 1154817787,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("InputModifierEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct GameplaySpawnReferenceObjectData {
    pub _glacier_base: super::entity::SpatialReferenceObjectData,
    pub extra_spawn_data: Vec<Option<LockedTypeObject /* ExtraSpawnData */>>,
    pub enabled: bool,
    pub location_name_sid: String,
    pub location_text_sid: String,
    pub initial_auto_spawn: bool,
    pub auto_spawn: bool,
    pub queue_spawn_event: bool,
    pub use_as_spawn_point: bool,
    pub initial_spawn_delay: f32,
    pub spawn_delay: f32,
    pub max_count: i32,
    pub max_count_simultaneously: i32,
    pub total_count_simultaneously_of_type: i32,
    pub max_spawn_in_frame: i32,
    pub only_send_event_for_human_players: bool,
    pub controllable_transform: super::core::LinearTransform,
    pub controllable_input: super::core::LinearTransform,
}

pub trait GameplaySpawnReferenceObjectDataTrait: super::entity::SpatialReferenceObjectDataTrait {
    fn extra_spawn_data(&self) -> &Vec<Option<LockedTypeObject /* ExtraSpawnData */>>;
    fn extra_spawn_data_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* ExtraSpawnData */>>;
    fn enabled(&self) -> &bool;
    fn enabled_mut(&mut self) -> &mut bool;
    fn location_name_sid(&self) -> &String;
    fn location_name_sid_mut(&mut self) -> &mut String;
    fn location_text_sid(&self) -> &String;
    fn location_text_sid_mut(&mut self) -> &mut String;
    fn initial_auto_spawn(&self) -> &bool;
    fn initial_auto_spawn_mut(&mut self) -> &mut bool;
    fn auto_spawn(&self) -> &bool;
    fn auto_spawn_mut(&mut self) -> &mut bool;
    fn queue_spawn_event(&self) -> &bool;
    fn queue_spawn_event_mut(&mut self) -> &mut bool;
    fn use_as_spawn_point(&self) -> &bool;
    fn use_as_spawn_point_mut(&mut self) -> &mut bool;
    fn initial_spawn_delay(&self) -> &f32;
    fn initial_spawn_delay_mut(&mut self) -> &mut f32;
    fn spawn_delay(&self) -> &f32;
    fn spawn_delay_mut(&mut self) -> &mut f32;
    fn max_count(&self) -> &i32;
    fn max_count_mut(&mut self) -> &mut i32;
    fn max_count_simultaneously(&self) -> &i32;
    fn max_count_simultaneously_mut(&mut self) -> &mut i32;
    fn total_count_simultaneously_of_type(&self) -> &i32;
    fn total_count_simultaneously_of_type_mut(&mut self) -> &mut i32;
    fn max_spawn_in_frame(&self) -> &i32;
    fn max_spawn_in_frame_mut(&mut self) -> &mut i32;
    fn only_send_event_for_human_players(&self) -> &bool;
    fn only_send_event_for_human_players_mut(&mut self) -> &mut bool;
    fn controllable_transform(&self) -> &super::core::LinearTransform;
    fn controllable_transform_mut(&mut self) -> &mut super::core::LinearTransform;
    fn controllable_input(&self) -> &super::core::LinearTransform;
    fn controllable_input_mut(&mut self) -> &mut super::core::LinearTransform;
}

impl GameplaySpawnReferenceObjectDataTrait for GameplaySpawnReferenceObjectData {
    fn extra_spawn_data(&self) -> &Vec<Option<LockedTypeObject /* ExtraSpawnData */>> {
        &self.extra_spawn_data
    }
    fn extra_spawn_data_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* ExtraSpawnData */>> {
        &mut self.extra_spawn_data
    }
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
    fn location_name_sid(&self) -> &String {
        &self.location_name_sid
    }
    fn location_name_sid_mut(&mut self) -> &mut String {
        &mut self.location_name_sid
    }
    fn location_text_sid(&self) -> &String {
        &self.location_text_sid
    }
    fn location_text_sid_mut(&mut self) -> &mut String {
        &mut self.location_text_sid
    }
    fn initial_auto_spawn(&self) -> &bool {
        &self.initial_auto_spawn
    }
    fn initial_auto_spawn_mut(&mut self) -> &mut bool {
        &mut self.initial_auto_spawn
    }
    fn auto_spawn(&self) -> &bool {
        &self.auto_spawn
    }
    fn auto_spawn_mut(&mut self) -> &mut bool {
        &mut self.auto_spawn
    }
    fn queue_spawn_event(&self) -> &bool {
        &self.queue_spawn_event
    }
    fn queue_spawn_event_mut(&mut self) -> &mut bool {
        &mut self.queue_spawn_event
    }
    fn use_as_spawn_point(&self) -> &bool {
        &self.use_as_spawn_point
    }
    fn use_as_spawn_point_mut(&mut self) -> &mut bool {
        &mut self.use_as_spawn_point
    }
    fn initial_spawn_delay(&self) -> &f32 {
        &self.initial_spawn_delay
    }
    fn initial_spawn_delay_mut(&mut self) -> &mut f32 {
        &mut self.initial_spawn_delay
    }
    fn spawn_delay(&self) -> &f32 {
        &self.spawn_delay
    }
    fn spawn_delay_mut(&mut self) -> &mut f32 {
        &mut self.spawn_delay
    }
    fn max_count(&self) -> &i32 {
        &self.max_count
    }
    fn max_count_mut(&mut self) -> &mut i32 {
        &mut self.max_count
    }
    fn max_count_simultaneously(&self) -> &i32 {
        &self.max_count_simultaneously
    }
    fn max_count_simultaneously_mut(&mut self) -> &mut i32 {
        &mut self.max_count_simultaneously
    }
    fn total_count_simultaneously_of_type(&self) -> &i32 {
        &self.total_count_simultaneously_of_type
    }
    fn total_count_simultaneously_of_type_mut(&mut self) -> &mut i32 {
        &mut self.total_count_simultaneously_of_type
    }
    fn max_spawn_in_frame(&self) -> &i32 {
        &self.max_spawn_in_frame
    }
    fn max_spawn_in_frame_mut(&mut self) -> &mut i32 {
        &mut self.max_spawn_in_frame
    }
    fn only_send_event_for_human_players(&self) -> &bool {
        &self.only_send_event_for_human_players
    }
    fn only_send_event_for_human_players_mut(&mut self) -> &mut bool {
        &mut self.only_send_event_for_human_players
    }
    fn controllable_transform(&self) -> &super::core::LinearTransform {
        &self.controllable_transform
    }
    fn controllable_transform_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.controllable_transform
    }
    fn controllable_input(&self) -> &super::core::LinearTransform {
        &self.controllable_input
    }
    fn controllable_input_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.controllable_input
    }
}

impl super::entity::SpatialReferenceObjectDataTrait for GameplaySpawnReferenceObjectData {
    fn local_player_id(&self) -> &super::core::LocalPlayerId {
        self._glacier_base.local_player_id()
    }
    fn local_player_id_mut(&mut self) -> &mut super::core::LocalPlayerId {
        self._glacier_base.local_player_id_mut()
    }
}

impl super::entity::ReferenceObjectDataTrait for GameplaySpawnReferenceObjectData {
    fn blueprint_transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.blueprint_transform()
    }
    fn blueprint_transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.blueprint_transform_mut()
    }
    fn blueprint(&self) -> &Option<LockedTypeObject /* super::entity::Blueprint */> {
        self._glacier_base.blueprint()
    }
    fn blueprint_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::Blueprint */> {
        self._glacier_base.blueprint_mut()
    }
    fn object_variation(&self) -> &Option<LockedTypeObject /* super::entity::ObjectVariation */> {
        self._glacier_base.object_variation()
    }
    fn object_variation_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::ObjectVariation */> {
        self._glacier_base.object_variation_mut()
    }
    fn stream_realm(&self) -> &super::entity::StreamRealm {
        self._glacier_base.stream_realm()
    }
    fn stream_realm_mut(&mut self) -> &mut super::entity::StreamRealm {
        self._glacier_base.stream_realm_mut()
    }
    fn radiosity_type_override(&self) -> &super::core::RadiosityTypeOverride {
        self._glacier_base.radiosity_type_override()
    }
    fn radiosity_type_override_mut(&mut self) -> &mut super::core::RadiosityTypeOverride {
        self._glacier_base.radiosity_type_override_mut()
    }
    fn lightmap_resolution_scale(&self) -> &u32 {
        self._glacier_base.lightmap_resolution_scale()
    }
    fn lightmap_resolution_scale_mut(&mut self) -> &mut u32 {
        self._glacier_base.lightmap_resolution_scale_mut()
    }
    fn lightmap_scale_with_size(&self) -> &bool {
        self._glacier_base.lightmap_scale_with_size()
    }
    fn lightmap_scale_with_size_mut(&mut self) -> &mut bool {
        self._glacier_base.lightmap_scale_with_size_mut()
    }
    fn rendering_overrides(&self) -> &super::core::RenderingOverrides {
        self._glacier_base.rendering_overrides()
    }
    fn rendering_overrides_mut(&mut self) -> &mut super::core::RenderingOverrides {
        self._glacier_base.rendering_overrides_mut()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
    fn excluded_mut(&mut self) -> &mut bool {
        self._glacier_base.excluded_mut()
    }
    fn create_indestructible_entity(&self) -> &bool {
        self._glacier_base.create_indestructible_entity()
    }
    fn create_indestructible_entity_mut(&mut self) -> &mut bool {
        self._glacier_base.create_indestructible_entity_mut()
    }
}

impl super::entity::GameObjectDataTrait for GameplaySpawnReferenceObjectData {
}

impl super::core::DataBusPeerTrait for GameplaySpawnReferenceObjectData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for GameplaySpawnReferenceObjectData {
}

impl super::core::DataContainerTrait for GameplaySpawnReferenceObjectData {
}

pub static GAMEPLAYSPAWNREFERENCEOBJECTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameplaySpawnReferenceObjectData",
    name_hash: 2653936090,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALREFERENCEOBJECTDATA_TYPE_INFO),
        super_class_offset: offset_of!(GameplaySpawnReferenceObjectData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GameplaySpawnReferenceObjectData as Default>::default())),
            create_boxed: || Box::new(<GameplaySpawnReferenceObjectData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ExtraSpawnData",
                name_hash: 3841934868,
                flags: MemberInfoFlags::new(144),
                field_type: "ExtraSpawnData-Array",
                rust_offset: offset_of!(GameplaySpawnReferenceObjectData, extra_spawn_data),
            },
            FieldInfoData {
                name: "Enabled",
                name_hash: 2662400,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GameplaySpawnReferenceObjectData, enabled),
            },
            FieldInfoData {
                name: "LocationNameSid",
                name_hash: 1490142753,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(GameplaySpawnReferenceObjectData, location_name_sid),
            },
            FieldInfoData {
                name: "LocationTextSid",
                name_hash: 2025703195,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(GameplaySpawnReferenceObjectData, location_text_sid),
            },
            FieldInfoData {
                name: "InitialAutoSpawn",
                name_hash: 3746373231,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GameplaySpawnReferenceObjectData, initial_auto_spawn),
            },
            FieldInfoData {
                name: "AutoSpawn",
                name_hash: 792472241,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GameplaySpawnReferenceObjectData, auto_spawn),
            },
            FieldInfoData {
                name: "QueueSpawnEvent",
                name_hash: 938847395,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GameplaySpawnReferenceObjectData, queue_spawn_event),
            },
            FieldInfoData {
                name: "UseAsSpawnPoint",
                name_hash: 3790673059,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GameplaySpawnReferenceObjectData, use_as_spawn_point),
            },
            FieldInfoData {
                name: "InitialSpawnDelay",
                name_hash: 2240280789,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(GameplaySpawnReferenceObjectData, initial_spawn_delay),
            },
            FieldInfoData {
                name: "SpawnDelay",
                name_hash: 3473198411,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(GameplaySpawnReferenceObjectData, spawn_delay),
            },
            FieldInfoData {
                name: "MaxCount",
                name_hash: 415061138,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(GameplaySpawnReferenceObjectData, max_count),
            },
            FieldInfoData {
                name: "MaxCountSimultaneously",
                name_hash: 2090824542,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(GameplaySpawnReferenceObjectData, max_count_simultaneously),
            },
            FieldInfoData {
                name: "TotalCountSimultaneouslyOfType",
                name_hash: 3044908761,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(GameplaySpawnReferenceObjectData, total_count_simultaneously_of_type),
            },
            FieldInfoData {
                name: "MaxSpawnInFrame",
                name_hash: 672198416,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(GameplaySpawnReferenceObjectData, max_spawn_in_frame),
            },
            FieldInfoData {
                name: "OnlySendEventForHumanPlayers",
                name_hash: 1401029797,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GameplaySpawnReferenceObjectData, only_send_event_for_human_players),
            },
            FieldInfoData {
                name: "ControllableTransform",
                name_hash: 2085022312,
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(GameplaySpawnReferenceObjectData, controllable_transform),
            },
            FieldInfoData {
                name: "ControllableInput",
                name_hash: 2033256658,
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(GameplaySpawnReferenceObjectData, controllable_input),
            },
        ],
    }),
    array_type: Some(GAMEPLAYSPAWNREFERENCEOBJECTDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for GameplaySpawnReferenceObjectData {
    fn type_info(&self) -> &'static TypeInfo {
        GAMEPLAYSPAWNREFERENCEOBJECTDATA_TYPE_INFO
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


pub static GAMEPLAYSPAWNREFERENCEOBJECTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameplaySpawnReferenceObjectData-Array",
    name_hash: 944336238,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("GameplaySpawnReferenceObjectData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ExtraSpawnData {
    pub _glacier_base: super::core::DataContainer,
}

pub trait ExtraSpawnDataTrait: super::core::DataContainerTrait {
}

impl ExtraSpawnDataTrait for ExtraSpawnData {
}

impl super::core::DataContainerTrait for ExtraSpawnData {
}

pub static EXTRASPAWNDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExtraSpawnData",
    name_hash: 3841934868,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(ExtraSpawnData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ExtraSpawnData as Default>::default())),
            create_boxed: || Box::new(<ExtraSpawnData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(EXTRASPAWNDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ExtraSpawnData {
    fn type_info(&self) -> &'static TypeInfo {
        EXTRASPAWNDATA_TYPE_INFO
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


pub static EXTRASPAWNDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExtraSpawnData-Array",
    name_hash: 709681696,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ExtraSpawnData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct GameplayTeamEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub team: Option<LockedTypeObject /* GameplayTeamData */>,
    pub id: TeamId,
}

pub trait GameplayTeamEntityDataTrait: super::entity::EntityDataTrait {
    fn team(&self) -> &Option<LockedTypeObject /* GameplayTeamData */>;
    fn team_mut(&mut self) -> &mut Option<LockedTypeObject /* GameplayTeamData */>;
    fn id(&self) -> &TeamId;
    fn id_mut(&mut self) -> &mut TeamId;
}

impl GameplayTeamEntityDataTrait for GameplayTeamEntityData {
    fn team(&self) -> &Option<LockedTypeObject /* GameplayTeamData */> {
        &self.team
    }
    fn team_mut(&mut self) -> &mut Option<LockedTypeObject /* GameplayTeamData */> {
        &mut self.team
    }
    fn id(&self) -> &TeamId {
        &self.id
    }
    fn id_mut(&mut self) -> &mut TeamId {
        &mut self.id
    }
}

impl super::entity::EntityDataTrait for GameplayTeamEntityData {
}

impl super::entity::GameObjectDataTrait for GameplayTeamEntityData {
}

impl super::core::DataBusPeerTrait for GameplayTeamEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for GameplayTeamEntityData {
}

impl super::core::DataContainerTrait for GameplayTeamEntityData {
}

pub static GAMEPLAYTEAMENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameplayTeamEntityData",
    name_hash: 3188917177,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(GameplayTeamEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GameplayTeamEntityData as Default>::default())),
            create_boxed: || Box::new(<GameplayTeamEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Team",
                name_hash: 2089309528,
                flags: MemberInfoFlags::new(0),
                field_type: "GameplayTeamData",
                rust_offset: offset_of!(GameplayTeamEntityData, team),
            },
            FieldInfoData {
                name: "Id",
                name_hash: 5862152,
                flags: MemberInfoFlags::new(0),
                field_type: "TeamId",
                rust_offset: offset_of!(GameplayTeamEntityData, id),
            },
        ],
    }),
    array_type: Some(GAMEPLAYTEAMENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for GameplayTeamEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        GAMEPLAYTEAMENTITYDATA_TYPE_INFO
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


pub static GAMEPLAYTEAMENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameplayTeamEntityData-Array",
    name_hash: 2320437773,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("GameplayTeamEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct GameplayTeamData {
    pub _glacier_base: super::core::DataContainerPolicyAsset,
    pub faction: FactionId,
}

pub trait GameplayTeamDataTrait: super::core::DataContainerPolicyAssetTrait {
    fn faction(&self) -> &FactionId;
    fn faction_mut(&mut self) -> &mut FactionId;
}

impl GameplayTeamDataTrait for GameplayTeamData {
    fn faction(&self) -> &FactionId {
        &self.faction
    }
    fn faction_mut(&mut self) -> &mut FactionId {
        &mut self.faction
    }
}

impl super::core::DataContainerPolicyAssetTrait for GameplayTeamData {
}

impl super::core::AssetTrait for GameplayTeamData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for GameplayTeamData {
}

pub static GAMEPLAYTEAMDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameplayTeamData",
    name_hash: 210181474,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINERPOLICYASSET_TYPE_INFO),
        super_class_offset: offset_of!(GameplayTeamData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GameplayTeamData as Default>::default())),
            create_boxed: || Box::new(<GameplayTeamData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Faction",
                name_hash: 1419764413,
                flags: MemberInfoFlags::new(0),
                field_type: "FactionId",
                rust_offset: offset_of!(GameplayTeamData, faction),
            },
        ],
    }),
    array_type: Some(GAMEPLAYTEAMDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for GameplayTeamData {
    fn type_info(&self) -> &'static TypeInfo {
        GAMEPLAYTEAMDATA_TYPE_INFO
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


pub static GAMEPLAYTEAMDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameplayTeamData-Array",
    name_hash: 3042856022,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("GameplayTeamData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SubLevelCollectionEntityData {
    pub _glacier_base: DetachableSubWorldCollectionBaseData,
    pub stream_realm: super::entity::StreamRealm,
    pub active_index: i32,
    pub visible: bool,
}

pub trait SubLevelCollectionEntityDataTrait: DetachableSubWorldCollectionBaseDataTrait {
    fn stream_realm(&self) -> &super::entity::StreamRealm;
    fn stream_realm_mut(&mut self) -> &mut super::entity::StreamRealm;
    fn active_index(&self) -> &i32;
    fn active_index_mut(&mut self) -> &mut i32;
    fn visible(&self) -> &bool;
    fn visible_mut(&mut self) -> &mut bool;
}

impl SubLevelCollectionEntityDataTrait for SubLevelCollectionEntityData {
    fn stream_realm(&self) -> &super::entity::StreamRealm {
        &self.stream_realm
    }
    fn stream_realm_mut(&mut self) -> &mut super::entity::StreamRealm {
        &mut self.stream_realm
    }
    fn active_index(&self) -> &i32 {
        &self.active_index
    }
    fn active_index_mut(&mut self) -> &mut i32 {
        &mut self.active_index
    }
    fn visible(&self) -> &bool {
        &self.visible
    }
    fn visible_mut(&mut self) -> &mut bool {
        &mut self.visible
    }
}

impl DetachableSubWorldCollectionBaseDataTrait for SubLevelCollectionEntityData {
    fn sub_level_refs(&self) -> &Vec<Option<LockedTypeObject /* super::entity::SubWorldReferenceObjectData */>> {
        self._glacier_base.sub_level_refs()
    }
    fn sub_level_refs_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::SubWorldReferenceObjectData */>> {
        self._glacier_base.sub_level_refs_mut()
    }
    fn object_layers(&self) -> &i32 {
        self._glacier_base.object_layers()
    }
    fn object_layers_mut(&mut self) -> &mut i32 {
        self._glacier_base.object_layers_mut()
    }
}

impl super::entity::EntityDataTrait for SubLevelCollectionEntityData {
}

impl super::entity::GameObjectDataTrait for SubLevelCollectionEntityData {
}

impl super::core::DataBusPeerTrait for SubLevelCollectionEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for SubLevelCollectionEntityData {
}

impl super::core::DataContainerTrait for SubLevelCollectionEntityData {
}

pub static SUBLEVELCOLLECTIONENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelCollectionEntityData",
    name_hash: 471554314,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DETACHABLESUBWORLDCOLLECTIONBASEDATA_TYPE_INFO),
        super_class_offset: offset_of!(SubLevelCollectionEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SubLevelCollectionEntityData as Default>::default())),
            create_boxed: || Box::new(<SubLevelCollectionEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "StreamRealm",
                name_hash: 1844114478,
                flags: MemberInfoFlags::new(0),
                field_type: "StreamRealm",
                rust_offset: offset_of!(SubLevelCollectionEntityData, stream_realm),
            },
            FieldInfoData {
                name: "ActiveIndex",
                name_hash: 1397492695,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(SubLevelCollectionEntityData, active_index),
            },
            FieldInfoData {
                name: "Visible",
                name_hash: 901540267,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SubLevelCollectionEntityData, visible),
            },
        ],
    }),
    array_type: Some(SUBLEVELCOLLECTIONENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SubLevelCollectionEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        SUBLEVELCOLLECTIONENTITYDATA_TYPE_INFO
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


pub static SUBLEVELCOLLECTIONENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelCollectionEntityData-Array",
    name_hash: 3292812350,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("SubLevelCollectionEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DetachableSubWorldCollectionBaseData {
    pub _glacier_base: super::entity::EntityData,
    pub sub_level_refs: Vec<Option<LockedTypeObject /* super::entity::SubWorldReferenceObjectData */>>,
    pub object_layers: i32,
}

pub trait DetachableSubWorldCollectionBaseDataTrait: super::entity::EntityDataTrait {
    fn sub_level_refs(&self) -> &Vec<Option<LockedTypeObject /* super::entity::SubWorldReferenceObjectData */>>;
    fn sub_level_refs_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::SubWorldReferenceObjectData */>>;
    fn object_layers(&self) -> &i32;
    fn object_layers_mut(&mut self) -> &mut i32;
}

impl DetachableSubWorldCollectionBaseDataTrait for DetachableSubWorldCollectionBaseData {
    fn sub_level_refs(&self) -> &Vec<Option<LockedTypeObject /* super::entity::SubWorldReferenceObjectData */>> {
        &self.sub_level_refs
    }
    fn sub_level_refs_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::SubWorldReferenceObjectData */>> {
        &mut self.sub_level_refs
    }
    fn object_layers(&self) -> &i32 {
        &self.object_layers
    }
    fn object_layers_mut(&mut self) -> &mut i32 {
        &mut self.object_layers
    }
}

impl super::entity::EntityDataTrait for DetachableSubWorldCollectionBaseData {
}

impl super::entity::GameObjectDataTrait for DetachableSubWorldCollectionBaseData {
}

impl super::core::DataBusPeerTrait for DetachableSubWorldCollectionBaseData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for DetachableSubWorldCollectionBaseData {
}

impl super::core::DataContainerTrait for DetachableSubWorldCollectionBaseData {
}

pub static DETACHABLESUBWORLDCOLLECTIONBASEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DetachableSubWorldCollectionBaseData",
    name_hash: 37671429,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(DetachableSubWorldCollectionBaseData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DetachableSubWorldCollectionBaseData as Default>::default())),
            create_boxed: || Box::new(<DetachableSubWorldCollectionBaseData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "SubLevelRefs",
                name_hash: 1494190037,
                flags: MemberInfoFlags::new(144),
                field_type: "SubWorldReferenceObjectData-Array",
                rust_offset: offset_of!(DetachableSubWorldCollectionBaseData, sub_level_refs),
            },
            FieldInfoData {
                name: "ObjectLayers",
                name_hash: 3459181984,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(DetachableSubWorldCollectionBaseData, object_layers),
            },
        ],
    }),
    array_type: Some(DETACHABLESUBWORLDCOLLECTIONBASEDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DetachableSubWorldCollectionBaseData {
    fn type_info(&self) -> &'static TypeInfo {
        DETACHABLESUBWORLDCOLLECTIONBASEDATA_TYPE_INFO
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


pub static DETACHABLESUBWORLDCOLLECTIONBASEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DetachableSubWorldCollectionBaseData-Array",
    name_hash: 3225254321,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("DetachableSubWorldCollectionBaseData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DemoClientReadyForPlaybackMessage {
}

pub trait DemoClientReadyForPlaybackMessageTrait: TypeObject {
}

impl DemoClientReadyForPlaybackMessageTrait for DemoClientReadyForPlaybackMessage {
}

pub static DEMOCLIENTREADYFORPLAYBACKMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DemoClientReadyForPlaybackMessage",
    name_hash: 1182485899,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DemoClientReadyForPlaybackMessage as Default>::default())),
            create_boxed: || Box::new(<DemoClientReadyForPlaybackMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for DemoClientReadyForPlaybackMessage {
    fn type_info(&self) -> &'static TypeInfo {
        DEMOCLIENTREADYFORPLAYBACKMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct NetworkLogicFireDoublePlayerEventMessageBase {
}

pub trait NetworkLogicFireDoublePlayerEventMessageBaseTrait: TypeObject {
}

impl NetworkLogicFireDoublePlayerEventMessageBaseTrait for NetworkLogicFireDoublePlayerEventMessageBase {
}

pub static NETWORKLOGICFIREDOUBLEPLAYEREVENTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkLogicFireDoublePlayerEventMessageBase",
    name_hash: 967012729,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkLogicFireDoublePlayerEventMessageBase as Default>::default())),
            create_boxed: || Box::new(<NetworkLogicFireDoublePlayerEventMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for NetworkLogicFireDoublePlayerEventMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKLOGICFIREDOUBLEPLAYEREVENTMESSAGEBASE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct NetworkLogicFirePlayerEventMessageBase {
}

pub trait NetworkLogicFirePlayerEventMessageBaseTrait: TypeObject {
}

impl NetworkLogicFirePlayerEventMessageBaseTrait for NetworkLogicFirePlayerEventMessageBase {
}

pub static NETWORKLOGICFIREPLAYEREVENTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkLogicFirePlayerEventMessageBase",
    name_hash: 1581933228,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkLogicFirePlayerEventMessageBase as Default>::default())),
            create_boxed: || Box::new(<NetworkLogicFirePlayerEventMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for NetworkLogicFirePlayerEventMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKLOGICFIREPLAYEREVENTMESSAGEBASE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct NetworkLogicFireEventMessageBase {
}

pub trait NetworkLogicFireEventMessageBaseTrait: TypeObject {
}

impl NetworkLogicFireEventMessageBaseTrait for NetworkLogicFireEventMessageBase {
}

pub static NETWORKLOGICFIREEVENTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkLogicFireEventMessageBase",
    name_hash: 1295621023,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkLogicFireEventMessageBase as Default>::default())),
            create_boxed: || Box::new(<NetworkLogicFireEventMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for NetworkLogicFireEventMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKLOGICFIREEVENTMESSAGEBASE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct SubLevelFromClientSubLevelUnloadedMessage {
}

pub trait SubLevelFromClientSubLevelUnloadedMessageTrait: TypeObject {
}

impl SubLevelFromClientSubLevelUnloadedMessageTrait for SubLevelFromClientSubLevelUnloadedMessage {
}

pub static SUBLEVELFROMCLIENTSUBLEVELUNLOADEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelFromClientSubLevelUnloadedMessage",
    name_hash: 1702948189,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SubLevelFromClientSubLevelUnloadedMessage as Default>::default())),
            create_boxed: || Box::new(<SubLevelFromClientSubLevelUnloadedMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for SubLevelFromClientSubLevelUnloadedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SUBLEVELFROMCLIENTSUBLEVELUNLOADEDMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct SubLevelFromClientSubLevelLoadedMessage {
}

pub trait SubLevelFromClientSubLevelLoadedMessageTrait: TypeObject {
}

impl SubLevelFromClientSubLevelLoadedMessageTrait for SubLevelFromClientSubLevelLoadedMessage {
}

pub static SUBLEVELFROMCLIENTSUBLEVELLOADEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelFromClientSubLevelLoadedMessage",
    name_hash: 2508512518,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SubLevelFromClientSubLevelLoadedMessage as Default>::default())),
            create_boxed: || Box::new(<SubLevelFromClientSubLevelLoadedMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for SubLevelFromClientSubLevelLoadedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SUBLEVELFROMCLIENTSUBLEVELLOADEDMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct SubLevelFromClientRequestBundleBaselineMessage {
}

pub trait SubLevelFromClientRequestBundleBaselineMessageTrait: TypeObject {
}

impl SubLevelFromClientRequestBundleBaselineMessageTrait for SubLevelFromClientRequestBundleBaselineMessage {
}

pub static SUBLEVELFROMCLIENTREQUESTBUNDLEBASELINEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelFromClientRequestBundleBaselineMessage",
    name_hash: 927755949,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SubLevelFromClientRequestBundleBaselineMessage as Default>::default())),
            create_boxed: || Box::new(<SubLevelFromClientRequestBundleBaselineMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for SubLevelFromClientRequestBundleBaselineMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SUBLEVELFROMCLIENTREQUESTBUNDLEBASELINEMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct SubLevelToClientDropBundleBaselineMessage {
}

pub trait SubLevelToClientDropBundleBaselineMessageTrait: TypeObject {
}

impl SubLevelToClientDropBundleBaselineMessageTrait for SubLevelToClientDropBundleBaselineMessage {
}

pub static SUBLEVELTOCLIENTDROPBUNDLEBASELINEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelToClientDropBundleBaselineMessage",
    name_hash: 225486104,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SubLevelToClientDropBundleBaselineMessage as Default>::default())),
            create_boxed: || Box::new(<SubLevelToClientDropBundleBaselineMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for SubLevelToClientDropBundleBaselineMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SUBLEVELTOCLIENTDROPBUNDLEBASELINEMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct SubLevelToClientUnloadRequestsMessage {
}

pub trait SubLevelToClientUnloadRequestsMessageTrait: TypeObject {
}

impl SubLevelToClientUnloadRequestsMessageTrait for SubLevelToClientUnloadRequestsMessage {
}

pub static SUBLEVELTOCLIENTUNLOADREQUESTSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelToClientUnloadRequestsMessage",
    name_hash: 658182593,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SubLevelToClientUnloadRequestsMessage as Default>::default())),
            create_boxed: || Box::new(<SubLevelToClientUnloadRequestsMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for SubLevelToClientUnloadRequestsMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SUBLEVELTOCLIENTUNLOADREQUESTSMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct SubLevelToClientLoadRequestsMessage {
}

pub trait SubLevelToClientLoadRequestsMessageTrait: TypeObject {
}

impl SubLevelToClientLoadRequestsMessageTrait for SubLevelToClientLoadRequestsMessage {
}

pub static SUBLEVELTOCLIENTLOADREQUESTSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelToClientLoadRequestsMessage",
    name_hash: 698834202,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SubLevelToClientLoadRequestsMessage as Default>::default())),
            create_boxed: || Box::new(<SubLevelToClientLoadRequestsMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for SubLevelToClientLoadRequestsMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SUBLEVELTOCLIENTLOADREQUESTSMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct SubLevelBundleInfo {
    pub sub_level_name_inx: u16,
    pub sub_level_id: u16,
    pub parent_sub_level_id: u16,
    pub owner_unique_id: u32,
    pub compartment_index: i32,
    pub priority: u8,
    pub is_peer_filtered: bool,
    pub heap_info: super::entity::BundleHeapInfo,
    pub bundle_type: super::entity::BundleType,
    pub bundle_settings_info: super::entity::BundleSettingsInfo,
}

pub trait SubLevelBundleInfoTrait: TypeObject {
    fn sub_level_name_inx(&self) -> &u16;
    fn sub_level_name_inx_mut(&mut self) -> &mut u16;
    fn sub_level_id(&self) -> &u16;
    fn sub_level_id_mut(&mut self) -> &mut u16;
    fn parent_sub_level_id(&self) -> &u16;
    fn parent_sub_level_id_mut(&mut self) -> &mut u16;
    fn owner_unique_id(&self) -> &u32;
    fn owner_unique_id_mut(&mut self) -> &mut u32;
    fn compartment_index(&self) -> &i32;
    fn compartment_index_mut(&mut self) -> &mut i32;
    fn priority(&self) -> &u8;
    fn priority_mut(&mut self) -> &mut u8;
    fn is_peer_filtered(&self) -> &bool;
    fn is_peer_filtered_mut(&mut self) -> &mut bool;
    fn heap_info(&self) -> &super::entity::BundleHeapInfo;
    fn heap_info_mut(&mut self) -> &mut super::entity::BundleHeapInfo;
    fn bundle_type(&self) -> &super::entity::BundleType;
    fn bundle_type_mut(&mut self) -> &mut super::entity::BundleType;
    fn bundle_settings_info(&self) -> &super::entity::BundleSettingsInfo;
    fn bundle_settings_info_mut(&mut self) -> &mut super::entity::BundleSettingsInfo;
}

impl SubLevelBundleInfoTrait for SubLevelBundleInfo {
    fn sub_level_name_inx(&self) -> &u16 {
        &self.sub_level_name_inx
    }
    fn sub_level_name_inx_mut(&mut self) -> &mut u16 {
        &mut self.sub_level_name_inx
    }
    fn sub_level_id(&self) -> &u16 {
        &self.sub_level_id
    }
    fn sub_level_id_mut(&mut self) -> &mut u16 {
        &mut self.sub_level_id
    }
    fn parent_sub_level_id(&self) -> &u16 {
        &self.parent_sub_level_id
    }
    fn parent_sub_level_id_mut(&mut self) -> &mut u16 {
        &mut self.parent_sub_level_id
    }
    fn owner_unique_id(&self) -> &u32 {
        &self.owner_unique_id
    }
    fn owner_unique_id_mut(&mut self) -> &mut u32 {
        &mut self.owner_unique_id
    }
    fn compartment_index(&self) -> &i32 {
        &self.compartment_index
    }
    fn compartment_index_mut(&mut self) -> &mut i32 {
        &mut self.compartment_index
    }
    fn priority(&self) -> &u8 {
        &self.priority
    }
    fn priority_mut(&mut self) -> &mut u8 {
        &mut self.priority
    }
    fn is_peer_filtered(&self) -> &bool {
        &self.is_peer_filtered
    }
    fn is_peer_filtered_mut(&mut self) -> &mut bool {
        &mut self.is_peer_filtered
    }
    fn heap_info(&self) -> &super::entity::BundleHeapInfo {
        &self.heap_info
    }
    fn heap_info_mut(&mut self) -> &mut super::entity::BundleHeapInfo {
        &mut self.heap_info
    }
    fn bundle_type(&self) -> &super::entity::BundleType {
        &self.bundle_type
    }
    fn bundle_type_mut(&mut self) -> &mut super::entity::BundleType {
        &mut self.bundle_type
    }
    fn bundle_settings_info(&self) -> &super::entity::BundleSettingsInfo {
        &self.bundle_settings_info
    }
    fn bundle_settings_info_mut(&mut self) -> &mut super::entity::BundleSettingsInfo {
        &mut self.bundle_settings_info
    }
}

pub static SUBLEVELBUNDLEINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelBundleInfo",
    name_hash: 666514317,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SubLevelBundleInfo as Default>::default())),
            create_boxed: || Box::new(<SubLevelBundleInfo as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "SubLevelNameInx",
                name_hash: 1233169263,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(SubLevelBundleInfo, sub_level_name_inx),
            },
            FieldInfoData {
                name: "SubLevelId",
                name_hash: 569302650,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(SubLevelBundleInfo, sub_level_id),
            },
            FieldInfoData {
                name: "ParentSubLevelId",
                name_hash: 1945668742,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(SubLevelBundleInfo, parent_sub_level_id),
            },
            FieldInfoData {
                name: "OwnerUniqueId",
                name_hash: 3711214202,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(SubLevelBundleInfo, owner_unique_id),
            },
            FieldInfoData {
                name: "CompartmentIndex",
                name_hash: 3856546367,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(SubLevelBundleInfo, compartment_index),
            },
            FieldInfoData {
                name: "Priority",
                name_hash: 3062102871,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(SubLevelBundleInfo, priority),
            },
            FieldInfoData {
                name: "IsPeerFiltered",
                name_hash: 3885213692,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SubLevelBundleInfo, is_peer_filtered),
            },
            FieldInfoData {
                name: "HeapInfo",
                name_hash: 1892552087,
                flags: MemberInfoFlags::new(0),
                field_type: "BundleHeapInfo",
                rust_offset: offset_of!(SubLevelBundleInfo, heap_info),
            },
            FieldInfoData {
                name: "BundleType",
                name_hash: 460109193,
                flags: MemberInfoFlags::new(0),
                field_type: "BundleType",
                rust_offset: offset_of!(SubLevelBundleInfo, bundle_type),
            },
            FieldInfoData {
                name: "BundleSettingsInfo",
                name_hash: 2785571642,
                flags: MemberInfoFlags::new(0),
                field_type: "BundleSettingsInfo",
                rust_offset: offset_of!(SubLevelBundleInfo, bundle_settings_info),
            },
        ],
    }),
    array_type: Some(SUBLEVELBUNDLEINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SubLevelBundleInfo {
    fn type_info(&self) -> &'static TypeInfo {
        SUBLEVELBUNDLEINFO_TYPE_INFO
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


pub static SUBLEVELBUNDLEINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelBundleInfo-Array",
    name_hash: 1721456441,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("SubLevelBundleInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SubLevelToClientSubLevelNameMessage {
}

pub trait SubLevelToClientSubLevelNameMessageTrait: TypeObject {
}

impl SubLevelToClientSubLevelNameMessageTrait for SubLevelToClientSubLevelNameMessage {
}

pub static SUBLEVELTOCLIENTSUBLEVELNAMEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelToClientSubLevelNameMessage",
    name_hash: 3109713707,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SubLevelToClientSubLevelNameMessage as Default>::default())),
            create_boxed: || Box::new(<SubLevelToClientSubLevelNameMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for SubLevelToClientSubLevelNameMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SUBLEVELTOCLIENTSUBLEVELNAMEMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct BundleNameAndIndex {
    pub index: u16,
    pub name: String,
}

pub trait BundleNameAndIndexTrait: TypeObject {
    fn index(&self) -> &u16;
    fn index_mut(&mut self) -> &mut u16;
    fn name(&self) -> &String;
    fn name_mut(&mut self) -> &mut String;
}

impl BundleNameAndIndexTrait for BundleNameAndIndex {
    fn index(&self) -> &u16 {
        &self.index
    }
    fn index_mut(&mut self) -> &mut u16 {
        &mut self.index
    }
    fn name(&self) -> &String {
        &self.name
    }
    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
}

pub static BUNDLENAMEANDINDEX_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BundleNameAndIndex",
    name_hash: 1865825379,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BundleNameAndIndex as Default>::default())),
            create_boxed: || Box::new(<BundleNameAndIndex as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Index",
                name_hash: 214509467,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(BundleNameAndIndex, index),
            },
            FieldInfoData {
                name: "Name",
                name_hash: 2088949890,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(BundleNameAndIndex, name),
            },
        ],
    }),
    array_type: Some(BUNDLENAMEANDINDEX_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BundleNameAndIndex {
    fn type_info(&self) -> &'static TypeInfo {
        BUNDLENAMEANDINDEX_TYPE_INFO
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


pub static BUNDLENAMEANDINDEX_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BundleNameAndIndex-Array",
    name_hash: 1402220631,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("BundleNameAndIndex"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ControllableToComponentsOnTeleportedMessage {
}

pub trait ControllableToComponentsOnTeleportedMessageTrait: TypeObject {
}

impl ControllableToComponentsOnTeleportedMessageTrait for ControllableToComponentsOnTeleportedMessage {
}

pub static CONTROLLABLETOCOMPONENTSONTELEPORTEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ControllableToComponentsOnTeleportedMessage",
    name_hash: 742101833,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ControllableToComponentsOnTeleportedMessage as Default>::default())),
            create_boxed: || Box::new(<ControllableToComponentsOnTeleportedMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ControllableToComponentsOnTeleportedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CONTROLLABLETOCOMPONENTSONTELEPORTEDMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerControllableToComponentsPlayerExitMessage {
}

pub trait ServerControllableToComponentsPlayerExitMessageTrait: TypeObject {
}

impl ServerControllableToComponentsPlayerExitMessageTrait for ServerControllableToComponentsPlayerExitMessage {
}

pub static SERVERCONTROLLABLETOCOMPONENTSPLAYEREXITMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerControllableToComponentsPlayerExitMessage",
    name_hash: 3465941342,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerControllableToComponentsPlayerExitMessage as Default>::default())),
            create_boxed: || Box::new(<ServerControllableToComponentsPlayerExitMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerControllableToComponentsPlayerExitMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCONTROLLABLETOCOMPONENTSPLAYEREXITMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerControllableToComponentsPlayerEnteredMessage {
}

pub trait ServerControllableToComponentsPlayerEnteredMessageTrait: TypeObject {
}

impl ServerControllableToComponentsPlayerEnteredMessageTrait for ServerControllableToComponentsPlayerEnteredMessage {
}

pub static SERVERCONTROLLABLETOCOMPONENTSPLAYERENTEREDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerControllableToComponentsPlayerEnteredMessage",
    name_hash: 2196253175,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerControllableToComponentsPlayerEnteredMessage as Default>::default())),
            create_boxed: || Box::new(<ServerControllableToComponentsPlayerEnteredMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerControllableToComponentsPlayerEnteredMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCONTROLLABLETOCOMPONENTSPLAYERENTEREDMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientControllableToComponentsPlayerExitMessage {
}

pub trait ClientControllableToComponentsPlayerExitMessageTrait: TypeObject {
}

impl ClientControllableToComponentsPlayerExitMessageTrait for ClientControllableToComponentsPlayerExitMessage {
}

pub static CLIENTCONTROLLABLETOCOMPONENTSPLAYEREXITMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientControllableToComponentsPlayerExitMessage",
    name_hash: 1832457986,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientControllableToComponentsPlayerExitMessage as Default>::default())),
            create_boxed: || Box::new(<ClientControllableToComponentsPlayerExitMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientControllableToComponentsPlayerExitMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCONTROLLABLETOCOMPONENTSPLAYEREXITMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientControllableToComponentsPlayerEnteredMessage {
}

pub trait ClientControllableToComponentsPlayerEnteredMessageTrait: TypeObject {
}

impl ClientControllableToComponentsPlayerEnteredMessageTrait for ClientControllableToComponentsPlayerEnteredMessage {
}

pub static CLIENTCONTROLLABLETOCOMPONENTSPLAYERENTEREDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientControllableToComponentsPlayerEnteredMessage",
    name_hash: 1740305323,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientControllableToComponentsPlayerEnteredMessage as Default>::default())),
            create_boxed: || Box::new(<ClientControllableToComponentsPlayerEnteredMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientControllableToComponentsPlayerEnteredMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCONTROLLABLETOCOMPONENTSPLAYERENTEREDMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct ControllableEntityData {
    pub _glacier_base: super::physics::GamePhysicsEntityData,
    pub use_prediction: bool,
    pub default_team: TeamId,
    pub reset_team_on_last_player_exits: bool,
    pub immortal: bool,
    pub fake_immortal: bool,
    pub low_health_threshold: f32,
    pub material_pair: super::entity::MaterialDecl,
    pub force_foreground_rendering: bool,
    pub suppressed_inputs: Vec<i32>,
}

pub trait ControllableEntityDataTrait: super::physics::GamePhysicsEntityDataTrait {
    fn use_prediction(&self) -> &bool;
    fn use_prediction_mut(&mut self) -> &mut bool;
    fn default_team(&self) -> &TeamId;
    fn default_team_mut(&mut self) -> &mut TeamId;
    fn reset_team_on_last_player_exits(&self) -> &bool;
    fn reset_team_on_last_player_exits_mut(&mut self) -> &mut bool;
    fn immortal(&self) -> &bool;
    fn immortal_mut(&mut self) -> &mut bool;
    fn fake_immortal(&self) -> &bool;
    fn fake_immortal_mut(&mut self) -> &mut bool;
    fn low_health_threshold(&self) -> &f32;
    fn low_health_threshold_mut(&mut self) -> &mut f32;
    fn material_pair(&self) -> &super::entity::MaterialDecl;
    fn material_pair_mut(&mut self) -> &mut super::entity::MaterialDecl;
    fn force_foreground_rendering(&self) -> &bool;
    fn force_foreground_rendering_mut(&mut self) -> &mut bool;
    fn suppressed_inputs(&self) -> &Vec<i32>;
    fn suppressed_inputs_mut(&mut self) -> &mut Vec<i32>;
}

impl ControllableEntityDataTrait for ControllableEntityData {
    fn use_prediction(&self) -> &bool {
        &self.use_prediction
    }
    fn use_prediction_mut(&mut self) -> &mut bool {
        &mut self.use_prediction
    }
    fn default_team(&self) -> &TeamId {
        &self.default_team
    }
    fn default_team_mut(&mut self) -> &mut TeamId {
        &mut self.default_team
    }
    fn reset_team_on_last_player_exits(&self) -> &bool {
        &self.reset_team_on_last_player_exits
    }
    fn reset_team_on_last_player_exits_mut(&mut self) -> &mut bool {
        &mut self.reset_team_on_last_player_exits
    }
    fn immortal(&self) -> &bool {
        &self.immortal
    }
    fn immortal_mut(&mut self) -> &mut bool {
        &mut self.immortal
    }
    fn fake_immortal(&self) -> &bool {
        &self.fake_immortal
    }
    fn fake_immortal_mut(&mut self) -> &mut bool {
        &mut self.fake_immortal
    }
    fn low_health_threshold(&self) -> &f32 {
        &self.low_health_threshold
    }
    fn low_health_threshold_mut(&mut self) -> &mut f32 {
        &mut self.low_health_threshold
    }
    fn material_pair(&self) -> &super::entity::MaterialDecl {
        &self.material_pair
    }
    fn material_pair_mut(&mut self) -> &mut super::entity::MaterialDecl {
        &mut self.material_pair
    }
    fn force_foreground_rendering(&self) -> &bool {
        &self.force_foreground_rendering
    }
    fn force_foreground_rendering_mut(&mut self) -> &mut bool {
        &mut self.force_foreground_rendering
    }
    fn suppressed_inputs(&self) -> &Vec<i32> {
        &self.suppressed_inputs
    }
    fn suppressed_inputs_mut(&mut self) -> &mut Vec<i32> {
        &mut self.suppressed_inputs
    }
}

impl super::physics::GamePhysicsEntityDataTrait for ControllableEntityData {
}

impl super::entity::GameComponentEntityDataTrait for ControllableEntityData {
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::entity::ComponentEntityDataTrait for ControllableEntityData {
    fn components(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components_mut()
    }
    fn part_bounding_boxes(&self) -> &Vec<BoxedTypeObject /* super::core::AxisAlignedBox */> {
        self._glacier_base.part_bounding_boxes()
    }
    fn part_bounding_boxes_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::core::AxisAlignedBox */> {
        self._glacier_base.part_bounding_boxes_mut()
    }
    fn client_runtime_component_count(&self) -> &u8 {
        self._glacier_base.client_runtime_component_count()
    }
    fn client_runtime_component_count_mut(&mut self) -> &mut u8 {
        self._glacier_base.client_runtime_component_count_mut()
    }
    fn server_runtime_component_count(&self) -> &u8 {
        self._glacier_base.server_runtime_component_count()
    }
    fn server_runtime_component_count_mut(&mut self) -> &mut u8 {
        self._glacier_base.server_runtime_component_count_mut()
    }
    fn client_runtime_transformation_count(&self) -> &u8 {
        self._glacier_base.client_runtime_transformation_count()
    }
    fn client_runtime_transformation_count_mut(&mut self) -> &mut u8 {
        self._glacier_base.client_runtime_transformation_count_mut()
    }
    fn server_runtime_transformation_count(&self) -> &u8 {
        self._glacier_base.server_runtime_transformation_count()
    }
    fn server_runtime_transformation_count_mut(&mut self) -> &mut u8 {
        self._glacier_base.server_runtime_transformation_count_mut()
    }
}

impl super::entity::SpatialEntityDataTrait for ControllableEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::entity::EntityDataTrait for ControllableEntityData {
}

impl super::entity::GameObjectDataTrait for ControllableEntityData {
}

impl super::core::DataBusPeerTrait for ControllableEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ControllableEntityData {
}

impl super::core::DataContainerTrait for ControllableEntityData {
}

pub static CONTROLLABLEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ControllableEntityData",
    name_hash: 2171182991,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::GAMEPHYSICSENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(ControllableEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ControllableEntityData as Default>::default())),
            create_boxed: || Box::new(<ControllableEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "UsePrediction",
                name_hash: 1407937619,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ControllableEntityData, use_prediction),
            },
            FieldInfoData {
                name: "DefaultTeam",
                name_hash: 2015081331,
                flags: MemberInfoFlags::new(0),
                field_type: "TeamId",
                rust_offset: offset_of!(ControllableEntityData, default_team),
            },
            FieldInfoData {
                name: "ResetTeamOnLastPlayerExits",
                name_hash: 3958982630,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ControllableEntityData, reset_team_on_last_player_exits),
            },
            FieldInfoData {
                name: "Immortal",
                name_hash: 922982664,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ControllableEntityData, immortal),
            },
            FieldInfoData {
                name: "FakeImmortal",
                name_hash: 875726273,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ControllableEntityData, fake_immortal),
            },
            FieldInfoData {
                name: "LowHealthThreshold",
                name_hash: 1532389018,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ControllableEntityData, low_health_threshold),
            },
            FieldInfoData {
                name: "MaterialPair",
                name_hash: 161392100,
                flags: MemberInfoFlags::new(0),
                field_type: "MaterialDecl",
                rust_offset: offset_of!(ControllableEntityData, material_pair),
            },
            FieldInfoData {
                name: "ForceForegroundRendering",
                name_hash: 251630665,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ControllableEntityData, force_foreground_rendering),
            },
            FieldInfoData {
                name: "SuppressedInputs",
                name_hash: 2986464400,
                flags: MemberInfoFlags::new(144),
                field_type: "Int32-Array",
                rust_offset: offset_of!(ControllableEntityData, suppressed_inputs),
            },
        ],
    }),
    array_type: Some(CONTROLLABLEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ControllableEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CONTROLLABLEENTITYDATA_TYPE_INFO
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


pub static CONTROLLABLEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ControllableEntityData-Array",
    name_hash: 862207035,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ControllableEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EntryComponentData {
    pub _glacier_base: super::entity::GameComponentData,
    pub entry_class: EntryClass,
    pub a_i_data: Option<LockedTypeObject /* GameAIEntryData */>,
    pub forbidden_for_human: bool,
    pub input_graph: Option<LockedTypeObject /* super::input_shared::InputGraph */>,
    pub input_concept_definition: Option<LockedTypeObject /* super::input_shared::InputActionMapsData */>,
    pub action_map_settings_flip_y_scheme_override: i32,
    pub input_mapping: Option<LockedTypeObject /* super::input_shared::InputConceptToEntryInputActionMappings */>,
    pub input_curves: Vec<Option<LockedTypeObject /* InputCurveData */>>,
    pub hud_data: EntryComponentHudData,
    pub entry_order_number: i32,
    pub enter_impulse: f32,
    pub entry_radius: f32,
    pub is_allowed_to_exit_in_air: bool,
    pub clear_path_to_exit_point_start_offset: super::core::Vec3,
    pub is_shielded: bool,
    pub show_soldier_in_entry: bool,
    pub show1p_soldier_in_entry: bool,
    pub soldier_transition_invisble_time: f32,
    pub entry_spotting_settings: EntrySpottingSettings,
    pub show_soldier_weapon_in_entry: bool,
    pub show1p_soldier_in_entry_for_player_only: bool,
    pub show3p_soldier_weapon_in_entry: bool,
    pub show_soldier_gear_in_entry: bool,
    pub pose_constraints: PoseConstraintsData,
    pub use_local_transform: bool,
    pub trigger_event_on_key: i32,
    pub allow_ragdoll_from_entry: bool,
    pub camera_index: i32,
}

pub trait EntryComponentDataTrait: super::entity::GameComponentDataTrait {
    fn entry_class(&self) -> &EntryClass;
    fn entry_class_mut(&mut self) -> &mut EntryClass;
    fn a_i_data(&self) -> &Option<LockedTypeObject /* GameAIEntryData */>;
    fn a_i_data_mut(&mut self) -> &mut Option<LockedTypeObject /* GameAIEntryData */>;
    fn forbidden_for_human(&self) -> &bool;
    fn forbidden_for_human_mut(&mut self) -> &mut bool;
    fn input_graph(&self) -> &Option<LockedTypeObject /* super::input_shared::InputGraph */>;
    fn input_graph_mut(&mut self) -> &mut Option<LockedTypeObject /* super::input_shared::InputGraph */>;
    fn input_concept_definition(&self) -> &Option<LockedTypeObject /* super::input_shared::InputActionMapsData */>;
    fn input_concept_definition_mut(&mut self) -> &mut Option<LockedTypeObject /* super::input_shared::InputActionMapsData */>;
    fn action_map_settings_flip_y_scheme_override(&self) -> &i32;
    fn action_map_settings_flip_y_scheme_override_mut(&mut self) -> &mut i32;
    fn input_mapping(&self) -> &Option<LockedTypeObject /* super::input_shared::InputConceptToEntryInputActionMappings */>;
    fn input_mapping_mut(&mut self) -> &mut Option<LockedTypeObject /* super::input_shared::InputConceptToEntryInputActionMappings */>;
    fn input_curves(&self) -> &Vec<Option<LockedTypeObject /* InputCurveData */>>;
    fn input_curves_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* InputCurveData */>>;
    fn hud_data(&self) -> &EntryComponentHudData;
    fn hud_data_mut(&mut self) -> &mut EntryComponentHudData;
    fn entry_order_number(&self) -> &i32;
    fn entry_order_number_mut(&mut self) -> &mut i32;
    fn enter_impulse(&self) -> &f32;
    fn enter_impulse_mut(&mut self) -> &mut f32;
    fn entry_radius(&self) -> &f32;
    fn entry_radius_mut(&mut self) -> &mut f32;
    fn is_allowed_to_exit_in_air(&self) -> &bool;
    fn is_allowed_to_exit_in_air_mut(&mut self) -> &mut bool;
    fn clear_path_to_exit_point_start_offset(&self) -> &super::core::Vec3;
    fn clear_path_to_exit_point_start_offset_mut(&mut self) -> &mut super::core::Vec3;
    fn is_shielded(&self) -> &bool;
    fn is_shielded_mut(&mut self) -> &mut bool;
    fn show_soldier_in_entry(&self) -> &bool;
    fn show_soldier_in_entry_mut(&mut self) -> &mut bool;
    fn show1p_soldier_in_entry(&self) -> &bool;
    fn show1p_soldier_in_entry_mut(&mut self) -> &mut bool;
    fn soldier_transition_invisble_time(&self) -> &f32;
    fn soldier_transition_invisble_time_mut(&mut self) -> &mut f32;
    fn entry_spotting_settings(&self) -> &EntrySpottingSettings;
    fn entry_spotting_settings_mut(&mut self) -> &mut EntrySpottingSettings;
    fn show_soldier_weapon_in_entry(&self) -> &bool;
    fn show_soldier_weapon_in_entry_mut(&mut self) -> &mut bool;
    fn show1p_soldier_in_entry_for_player_only(&self) -> &bool;
    fn show1p_soldier_in_entry_for_player_only_mut(&mut self) -> &mut bool;
    fn show3p_soldier_weapon_in_entry(&self) -> &bool;
    fn show3p_soldier_weapon_in_entry_mut(&mut self) -> &mut bool;
    fn show_soldier_gear_in_entry(&self) -> &bool;
    fn show_soldier_gear_in_entry_mut(&mut self) -> &mut bool;
    fn pose_constraints(&self) -> &PoseConstraintsData;
    fn pose_constraints_mut(&mut self) -> &mut PoseConstraintsData;
    fn use_local_transform(&self) -> &bool;
    fn use_local_transform_mut(&mut self) -> &mut bool;
    fn trigger_event_on_key(&self) -> &i32;
    fn trigger_event_on_key_mut(&mut self) -> &mut i32;
    fn allow_ragdoll_from_entry(&self) -> &bool;
    fn allow_ragdoll_from_entry_mut(&mut self) -> &mut bool;
    fn camera_index(&self) -> &i32;
    fn camera_index_mut(&mut self) -> &mut i32;
}

impl EntryComponentDataTrait for EntryComponentData {
    fn entry_class(&self) -> &EntryClass {
        &self.entry_class
    }
    fn entry_class_mut(&mut self) -> &mut EntryClass {
        &mut self.entry_class
    }
    fn a_i_data(&self) -> &Option<LockedTypeObject /* GameAIEntryData */> {
        &self.a_i_data
    }
    fn a_i_data_mut(&mut self) -> &mut Option<LockedTypeObject /* GameAIEntryData */> {
        &mut self.a_i_data
    }
    fn forbidden_for_human(&self) -> &bool {
        &self.forbidden_for_human
    }
    fn forbidden_for_human_mut(&mut self) -> &mut bool {
        &mut self.forbidden_for_human
    }
    fn input_graph(&self) -> &Option<LockedTypeObject /* super::input_shared::InputGraph */> {
        &self.input_graph
    }
    fn input_graph_mut(&mut self) -> &mut Option<LockedTypeObject /* super::input_shared::InputGraph */> {
        &mut self.input_graph
    }
    fn input_concept_definition(&self) -> &Option<LockedTypeObject /* super::input_shared::InputActionMapsData */> {
        &self.input_concept_definition
    }
    fn input_concept_definition_mut(&mut self) -> &mut Option<LockedTypeObject /* super::input_shared::InputActionMapsData */> {
        &mut self.input_concept_definition
    }
    fn action_map_settings_flip_y_scheme_override(&self) -> &i32 {
        &self.action_map_settings_flip_y_scheme_override
    }
    fn action_map_settings_flip_y_scheme_override_mut(&mut self) -> &mut i32 {
        &mut self.action_map_settings_flip_y_scheme_override
    }
    fn input_mapping(&self) -> &Option<LockedTypeObject /* super::input_shared::InputConceptToEntryInputActionMappings */> {
        &self.input_mapping
    }
    fn input_mapping_mut(&mut self) -> &mut Option<LockedTypeObject /* super::input_shared::InputConceptToEntryInputActionMappings */> {
        &mut self.input_mapping
    }
    fn input_curves(&self) -> &Vec<Option<LockedTypeObject /* InputCurveData */>> {
        &self.input_curves
    }
    fn input_curves_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* InputCurveData */>> {
        &mut self.input_curves
    }
    fn hud_data(&self) -> &EntryComponentHudData {
        &self.hud_data
    }
    fn hud_data_mut(&mut self) -> &mut EntryComponentHudData {
        &mut self.hud_data
    }
    fn entry_order_number(&self) -> &i32 {
        &self.entry_order_number
    }
    fn entry_order_number_mut(&mut self) -> &mut i32 {
        &mut self.entry_order_number
    }
    fn enter_impulse(&self) -> &f32 {
        &self.enter_impulse
    }
    fn enter_impulse_mut(&mut self) -> &mut f32 {
        &mut self.enter_impulse
    }
    fn entry_radius(&self) -> &f32 {
        &self.entry_radius
    }
    fn entry_radius_mut(&mut self) -> &mut f32 {
        &mut self.entry_radius
    }
    fn is_allowed_to_exit_in_air(&self) -> &bool {
        &self.is_allowed_to_exit_in_air
    }
    fn is_allowed_to_exit_in_air_mut(&mut self) -> &mut bool {
        &mut self.is_allowed_to_exit_in_air
    }
    fn clear_path_to_exit_point_start_offset(&self) -> &super::core::Vec3 {
        &self.clear_path_to_exit_point_start_offset
    }
    fn clear_path_to_exit_point_start_offset_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.clear_path_to_exit_point_start_offset
    }
    fn is_shielded(&self) -> &bool {
        &self.is_shielded
    }
    fn is_shielded_mut(&mut self) -> &mut bool {
        &mut self.is_shielded
    }
    fn show_soldier_in_entry(&self) -> &bool {
        &self.show_soldier_in_entry
    }
    fn show_soldier_in_entry_mut(&mut self) -> &mut bool {
        &mut self.show_soldier_in_entry
    }
    fn show1p_soldier_in_entry(&self) -> &bool {
        &self.show1p_soldier_in_entry
    }
    fn show1p_soldier_in_entry_mut(&mut self) -> &mut bool {
        &mut self.show1p_soldier_in_entry
    }
    fn soldier_transition_invisble_time(&self) -> &f32 {
        &self.soldier_transition_invisble_time
    }
    fn soldier_transition_invisble_time_mut(&mut self) -> &mut f32 {
        &mut self.soldier_transition_invisble_time
    }
    fn entry_spotting_settings(&self) -> &EntrySpottingSettings {
        &self.entry_spotting_settings
    }
    fn entry_spotting_settings_mut(&mut self) -> &mut EntrySpottingSettings {
        &mut self.entry_spotting_settings
    }
    fn show_soldier_weapon_in_entry(&self) -> &bool {
        &self.show_soldier_weapon_in_entry
    }
    fn show_soldier_weapon_in_entry_mut(&mut self) -> &mut bool {
        &mut self.show_soldier_weapon_in_entry
    }
    fn show1p_soldier_in_entry_for_player_only(&self) -> &bool {
        &self.show1p_soldier_in_entry_for_player_only
    }
    fn show1p_soldier_in_entry_for_player_only_mut(&mut self) -> &mut bool {
        &mut self.show1p_soldier_in_entry_for_player_only
    }
    fn show3p_soldier_weapon_in_entry(&self) -> &bool {
        &self.show3p_soldier_weapon_in_entry
    }
    fn show3p_soldier_weapon_in_entry_mut(&mut self) -> &mut bool {
        &mut self.show3p_soldier_weapon_in_entry
    }
    fn show_soldier_gear_in_entry(&self) -> &bool {
        &self.show_soldier_gear_in_entry
    }
    fn show_soldier_gear_in_entry_mut(&mut self) -> &mut bool {
        &mut self.show_soldier_gear_in_entry
    }
    fn pose_constraints(&self) -> &PoseConstraintsData {
        &self.pose_constraints
    }
    fn pose_constraints_mut(&mut self) -> &mut PoseConstraintsData {
        &mut self.pose_constraints
    }
    fn use_local_transform(&self) -> &bool {
        &self.use_local_transform
    }
    fn use_local_transform_mut(&mut self) -> &mut bool {
        &mut self.use_local_transform
    }
    fn trigger_event_on_key(&self) -> &i32 {
        &self.trigger_event_on_key
    }
    fn trigger_event_on_key_mut(&mut self) -> &mut i32 {
        &mut self.trigger_event_on_key
    }
    fn allow_ragdoll_from_entry(&self) -> &bool {
        &self.allow_ragdoll_from_entry
    }
    fn allow_ragdoll_from_entry_mut(&mut self) -> &mut bool {
        &mut self.allow_ragdoll_from_entry
    }
    fn camera_index(&self) -> &i32 {
        &self.camera_index
    }
    fn camera_index_mut(&mut self) -> &mut i32 {
        &mut self.camera_index
    }
}

impl super::entity::GameComponentDataTrait for EntryComponentData {
}

impl super::entity::ComponentDataTrait for EntryComponentData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
    fn components(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components_mut()
    }
    fn client_index(&self) -> &u8 {
        self._glacier_base.client_index()
    }
    fn client_index_mut(&mut self) -> &mut u8 {
        self._glacier_base.client_index_mut()
    }
    fn server_index(&self) -> &u8 {
        self._glacier_base.server_index()
    }
    fn server_index_mut(&mut self) -> &mut u8 {
        self._glacier_base.server_index_mut()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
    fn excluded_mut(&mut self) -> &mut bool {
        self._glacier_base.excluded_mut()
    }
}

impl super::entity::GameObjectDataTrait for EntryComponentData {
}

impl super::core::DataBusPeerTrait for EntryComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for EntryComponentData {
}

impl super::core::DataContainerTrait for EntryComponentData {
}

pub static ENTRYCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntryComponentData",
    name_hash: 2741094222,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::GAMECOMPONENTDATA_TYPE_INFO),
        super_class_offset: offset_of!(EntryComponentData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EntryComponentData as Default>::default())),
            create_boxed: || Box::new(<EntryComponentData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "EntryClass",
                name_hash: 292493759,
                flags: MemberInfoFlags::new(0),
                field_type: "EntryClass",
                rust_offset: offset_of!(EntryComponentData, entry_class),
            },
            FieldInfoData {
                name: "AIData",
                name_hash: 2529950045,
                flags: MemberInfoFlags::new(0),
                field_type: "GameAIEntryData",
                rust_offset: offset_of!(EntryComponentData, a_i_data),
            },
            FieldInfoData {
                name: "ForbiddenForHuman",
                name_hash: 606394362,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EntryComponentData, forbidden_for_human),
            },
            FieldInfoData {
                name: "InputGraph",
                name_hash: 1611054975,
                flags: MemberInfoFlags::new(0),
                field_type: "InputGraph",
                rust_offset: offset_of!(EntryComponentData, input_graph),
            },
            FieldInfoData {
                name: "InputConceptDefinition",
                name_hash: 1770668454,
                flags: MemberInfoFlags::new(0),
                field_type: "InputActionMapsData",
                rust_offset: offset_of!(EntryComponentData, input_concept_definition),
            },
            FieldInfoData {
                name: "ActionMapSettingsFlipYSchemeOverride",
                name_hash: 60876361,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(EntryComponentData, action_map_settings_flip_y_scheme_override),
            },
            FieldInfoData {
                name: "InputMapping",
                name_hash: 1744270687,
                flags: MemberInfoFlags::new(0),
                field_type: "InputConceptToEntryInputActionMappings",
                rust_offset: offset_of!(EntryComponentData, input_mapping),
            },
            FieldInfoData {
                name: "InputCurves",
                name_hash: 1465820471,
                flags: MemberInfoFlags::new(144),
                field_type: "InputCurveData-Array",
                rust_offset: offset_of!(EntryComponentData, input_curves),
            },
            FieldInfoData {
                name: "HudData",
                name_hash: 2639805868,
                flags: MemberInfoFlags::new(0),
                field_type: "EntryComponentHudData",
                rust_offset: offset_of!(EntryComponentData, hud_data),
            },
            FieldInfoData {
                name: "EntryOrderNumber",
                name_hash: 4225856860,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(EntryComponentData, entry_order_number),
            },
            FieldInfoData {
                name: "EnterImpulse",
                name_hash: 3008280342,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EntryComponentData, enter_impulse),
            },
            FieldInfoData {
                name: "EntryRadius",
                name_hash: 386452841,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EntryComponentData, entry_radius),
            },
            FieldInfoData {
                name: "IsAllowedToExitInAir",
                name_hash: 4064693665,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EntryComponentData, is_allowed_to_exit_in_air),
            },
            FieldInfoData {
                name: "ClearPathToExitPointStartOffset",
                name_hash: 3896255723,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(EntryComponentData, clear_path_to_exit_point_start_offset),
            },
            FieldInfoData {
                name: "IsShielded",
                name_hash: 2971034625,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EntryComponentData, is_shielded),
            },
            FieldInfoData {
                name: "ShowSoldierInEntry",
                name_hash: 2961492191,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EntryComponentData, show_soldier_in_entry),
            },
            FieldInfoData {
                name: "Show1pSoldierInEntry",
                name_hash: 1428104062,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EntryComponentData, show1p_soldier_in_entry),
            },
            FieldInfoData {
                name: "SoldierTransitionInvisbleTime",
                name_hash: 1720869397,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EntryComponentData, soldier_transition_invisble_time),
            },
            FieldInfoData {
                name: "EntrySpottingSettings",
                name_hash: 811356248,
                flags: MemberInfoFlags::new(0),
                field_type: "EntrySpottingSettings",
                rust_offset: offset_of!(EntryComponentData, entry_spotting_settings),
            },
            FieldInfoData {
                name: "ShowSoldierWeaponInEntry",
                name_hash: 4236823389,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EntryComponentData, show_soldier_weapon_in_entry),
            },
            FieldInfoData {
                name: "Show1pSoldierInEntryForPlayerOnly",
                name_hash: 1703931682,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EntryComponentData, show1p_soldier_in_entry_for_player_only),
            },
            FieldInfoData {
                name: "Show3pSoldierWeaponInEntry",
                name_hash: 240179134,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EntryComponentData, show3p_soldier_weapon_in_entry),
            },
            FieldInfoData {
                name: "ShowSoldierGearInEntry",
                name_hash: 1847936462,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EntryComponentData, show_soldier_gear_in_entry),
            },
            FieldInfoData {
                name: "PoseConstraints",
                name_hash: 4147330298,
                flags: MemberInfoFlags::new(0),
                field_type: "PoseConstraintsData",
                rust_offset: offset_of!(EntryComponentData, pose_constraints),
            },
            FieldInfoData {
                name: "UseLocalTransform",
                name_hash: 1309397031,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EntryComponentData, use_local_transform),
            },
            FieldInfoData {
                name: "TriggerEventOnKey",
                name_hash: 1329740711,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(EntryComponentData, trigger_event_on_key),
            },
            FieldInfoData {
                name: "AllowRagdollFromEntry",
                name_hash: 3770234945,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EntryComponentData, allow_ragdoll_from_entry),
            },
            FieldInfoData {
                name: "CameraIndex",
                name_hash: 3187238978,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(EntryComponentData, camera_index),
            },
        ],
    }),
    array_type: Some(ENTRYCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EntryComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        ENTRYCOMPONENTDATA_TYPE_INFO
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


pub static ENTRYCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntryComponentData-Array",
    name_hash: 390357242,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("EntryComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct InputCurveData {
    pub _glacier_base: super::core::DataContainer,
    pub affected_inputs: Vec<i32>,
    pub input_modifier_curve: Vec<BoxedTypeObject /* super::core::Vec2 */>,
    pub handle_multiple_inputs_as_square: bool,
}

pub trait InputCurveDataTrait: super::core::DataContainerTrait {
    fn affected_inputs(&self) -> &Vec<i32>;
    fn affected_inputs_mut(&mut self) -> &mut Vec<i32>;
    fn input_modifier_curve(&self) -> &Vec<BoxedTypeObject /* super::core::Vec2 */>;
    fn input_modifier_curve_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::core::Vec2 */>;
    fn handle_multiple_inputs_as_square(&self) -> &bool;
    fn handle_multiple_inputs_as_square_mut(&mut self) -> &mut bool;
}

impl InputCurveDataTrait for InputCurveData {
    fn affected_inputs(&self) -> &Vec<i32> {
        &self.affected_inputs
    }
    fn affected_inputs_mut(&mut self) -> &mut Vec<i32> {
        &mut self.affected_inputs
    }
    fn input_modifier_curve(&self) -> &Vec<BoxedTypeObject /* super::core::Vec2 */> {
        &self.input_modifier_curve
    }
    fn input_modifier_curve_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::core::Vec2 */> {
        &mut self.input_modifier_curve
    }
    fn handle_multiple_inputs_as_square(&self) -> &bool {
        &self.handle_multiple_inputs_as_square
    }
    fn handle_multiple_inputs_as_square_mut(&mut self) -> &mut bool {
        &mut self.handle_multiple_inputs_as_square
    }
}

impl super::core::DataContainerTrait for InputCurveData {
}

pub static INPUTCURVEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputCurveData",
    name_hash: 3709481204,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(InputCurveData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InputCurveData as Default>::default())),
            create_boxed: || Box::new(<InputCurveData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "AffectedInputs",
                name_hash: 337107346,
                flags: MemberInfoFlags::new(144),
                field_type: "Int32-Array",
                rust_offset: offset_of!(InputCurveData, affected_inputs),
            },
            FieldInfoData {
                name: "InputModifierCurve",
                name_hash: 1217881747,
                flags: MemberInfoFlags::new(144),
                field_type: "Vec2-Array",
                rust_offset: offset_of!(InputCurveData, input_modifier_curve),
            },
            FieldInfoData {
                name: "HandleMultipleInputsAsSquare",
                name_hash: 1190597481,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(InputCurveData, handle_multiple_inputs_as_square),
            },
        ],
    }),
    array_type: Some(INPUTCURVEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for InputCurveData {
    fn type_info(&self) -> &'static TypeInfo {
        INPUTCURVEDATA_TYPE_INFO
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


pub static INPUTCURVEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputCurveData-Array",
    name_hash: 3472374976,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("InputCurveData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum EntrySpottingSettings {
    #[default]
    ESSDefault = 0,
    ESSSendAndReceive = 1,
    ESSReceive = 2,
    ESSCount = 3,
}

pub static ENTRYSPOTTINGSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntrySpottingSettings",
    name_hash: 811356248,
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(ENTRYSPOTTINGSETTINGS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EntrySpottingSettings {
    fn type_info(&self) -> &'static TypeInfo {
        ENTRYSPOTTINGSETTINGS_TYPE_INFO
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


pub static ENTRYSPOTTINGSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntrySpottingSettings-Array",
    name_hash: 4133243628,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("EntrySpottingSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EntryComponentHudData {
    pub index: i32,
    pub frustum: bool,
    pub visible: bool,
    pub seat_type: EntrySeatType,
    pub maximize_mini_map_on_entry: bool,
}

pub trait EntryComponentHudDataTrait: TypeObject {
    fn index(&self) -> &i32;
    fn index_mut(&mut self) -> &mut i32;
    fn frustum(&self) -> &bool;
    fn frustum_mut(&mut self) -> &mut bool;
    fn visible(&self) -> &bool;
    fn visible_mut(&mut self) -> &mut bool;
    fn seat_type(&self) -> &EntrySeatType;
    fn seat_type_mut(&mut self) -> &mut EntrySeatType;
    fn maximize_mini_map_on_entry(&self) -> &bool;
    fn maximize_mini_map_on_entry_mut(&mut self) -> &mut bool;
}

impl EntryComponentHudDataTrait for EntryComponentHudData {
    fn index(&self) -> &i32 {
        &self.index
    }
    fn index_mut(&mut self) -> &mut i32 {
        &mut self.index
    }
    fn frustum(&self) -> &bool {
        &self.frustum
    }
    fn frustum_mut(&mut self) -> &mut bool {
        &mut self.frustum
    }
    fn visible(&self) -> &bool {
        &self.visible
    }
    fn visible_mut(&mut self) -> &mut bool {
        &mut self.visible
    }
    fn seat_type(&self) -> &EntrySeatType {
        &self.seat_type
    }
    fn seat_type_mut(&mut self) -> &mut EntrySeatType {
        &mut self.seat_type
    }
    fn maximize_mini_map_on_entry(&self) -> &bool {
        &self.maximize_mini_map_on_entry
    }
    fn maximize_mini_map_on_entry_mut(&mut self) -> &mut bool {
        &mut self.maximize_mini_map_on_entry
    }
}

pub static ENTRYCOMPONENTHUDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntryComponentHudData",
    name_hash: 4289455543,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EntryComponentHudData as Default>::default())),
            create_boxed: || Box::new(<EntryComponentHudData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Index",
                name_hash: 214509467,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(EntryComponentHudData, index),
            },
            FieldInfoData {
                name: "Frustum",
                name_hash: 2068536187,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EntryComponentHudData, frustum),
            },
            FieldInfoData {
                name: "Visible",
                name_hash: 901540267,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EntryComponentHudData, visible),
            },
            FieldInfoData {
                name: "SeatType",
                name_hash: 1128962654,
                flags: MemberInfoFlags::new(0),
                field_type: "EntrySeatType",
                rust_offset: offset_of!(EntryComponentHudData, seat_type),
            },
            FieldInfoData {
                name: "MaximizeMiniMapOnEntry",
                name_hash: 2821599657,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EntryComponentHudData, maximize_mini_map_on_entry),
            },
        ],
    }),
    array_type: Some(ENTRYCOMPONENTHUDDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for EntryComponentHudData {
    fn type_info(&self) -> &'static TypeInfo {
        ENTRYCOMPONENTHUDDATA_TYPE_INFO
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


pub static ENTRYCOMPONENTHUDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntryComponentHudData-Array",
    name_hash: 8535811,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("EntryComponentHudData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum EntrySeatType {
    #[default]
    EST_Driver = 0,
    EST_Gunner = 1,
    EST_Passenger = 2,
}

pub static ENTRYSEATTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntrySeatType",
    name_hash: 3712343082,
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(ENTRYSEATTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EntrySeatType {
    fn type_info(&self) -> &'static TypeInfo {
        ENTRYSEATTYPE_TYPE_INFO
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


pub static ENTRYSEATTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntrySeatType-Array",
    name_hash: 1563473566,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("EntrySeatType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct FOVTransitionData {
    pub _glacier_base: super::core::DataContainer,
    pub transition_type: FOVTransitionType,
    pub shape: f32,
    pub invert: bool,
    pub start_delay: f32,
    pub start_jump: f32,
    pub end_early: f32,
}

pub trait FOVTransitionDataTrait: super::core::DataContainerTrait {
    fn transition_type(&self) -> &FOVTransitionType;
    fn transition_type_mut(&mut self) -> &mut FOVTransitionType;
    fn shape(&self) -> &f32;
    fn shape_mut(&mut self) -> &mut f32;
    fn invert(&self) -> &bool;
    fn invert_mut(&mut self) -> &mut bool;
    fn start_delay(&self) -> &f32;
    fn start_delay_mut(&mut self) -> &mut f32;
    fn start_jump(&self) -> &f32;
    fn start_jump_mut(&mut self) -> &mut f32;
    fn end_early(&self) -> &f32;
    fn end_early_mut(&mut self) -> &mut f32;
}

impl FOVTransitionDataTrait for FOVTransitionData {
    fn transition_type(&self) -> &FOVTransitionType {
        &self.transition_type
    }
    fn transition_type_mut(&mut self) -> &mut FOVTransitionType {
        &mut self.transition_type
    }
    fn shape(&self) -> &f32 {
        &self.shape
    }
    fn shape_mut(&mut self) -> &mut f32 {
        &mut self.shape
    }
    fn invert(&self) -> &bool {
        &self.invert
    }
    fn invert_mut(&mut self) -> &mut bool {
        &mut self.invert
    }
    fn start_delay(&self) -> &f32 {
        &self.start_delay
    }
    fn start_delay_mut(&mut self) -> &mut f32 {
        &mut self.start_delay
    }
    fn start_jump(&self) -> &f32 {
        &self.start_jump
    }
    fn start_jump_mut(&mut self) -> &mut f32 {
        &mut self.start_jump
    }
    fn end_early(&self) -> &f32 {
        &self.end_early
    }
    fn end_early_mut(&mut self) -> &mut f32 {
        &mut self.end_early
    }
}

impl super::core::DataContainerTrait for FOVTransitionData {
}

pub static FOVTRANSITIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FOVTransitionData",
    name_hash: 1253932069,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(FOVTransitionData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FOVTransitionData as Default>::default())),
            create_boxed: || Box::new(<FOVTransitionData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "TransitionType",
                name_hash: 3976914738,
                flags: MemberInfoFlags::new(0),
                field_type: "FOVTransitionType",
                rust_offset: offset_of!(FOVTransitionData, transition_type),
            },
            FieldInfoData {
                name: "Shape",
                name_hash: 231753450,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FOVTransitionData, shape),
            },
            FieldInfoData {
                name: "Invert",
                name_hash: 2784486295,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FOVTransitionData, invert),
            },
            FieldInfoData {
                name: "StartDelay",
                name_hash: 2731915920,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FOVTransitionData, start_delay),
            },
            FieldInfoData {
                name: "StartJump",
                name_hash: 3727372263,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FOVTransitionData, start_jump),
            },
            FieldInfoData {
                name: "EndEarly",
                name_hash: 4006043081,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FOVTransitionData, end_early),
            },
        ],
    }),
    array_type: Some(FOVTRANSITIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FOVTransitionData {
    fn type_info(&self) -> &'static TypeInfo {
        FOVTRANSITIONDATA_TYPE_INFO
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


pub static FOVTRANSITIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FOVTransitionData-Array",
    name_hash: 1195340945,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("FOVTransitionData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum FOVTransitionType {
    #[default]
    FOVTransitionType_Linear = 0,
    FOVTransitionType_Smooth = 1,
    FOVTransitionType_Count = 2,
}

pub static FOVTRANSITIONTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FOVTransitionType",
    name_hash: 1253383085,
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(FOVTRANSITIONTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for FOVTransitionType {
    fn type_info(&self) -> &'static TypeInfo {
        FOVTRANSITIONTYPE_TYPE_INFO
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


pub static FOVTRANSITIONTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FOVTransitionType-Array",
    name_hash: 1339035673,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("FOVTransitionType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CameraTransitionEffectDataPack {
    pub _glacier_base: super::core::DataContainer,
    pub default_transition_in: CameraTransitionEffectData,
    pub default_transition_out: CameraTransitionEffectData,
}

pub trait CameraTransitionEffectDataPackTrait: super::core::DataContainerTrait {
    fn default_transition_in(&self) -> &CameraTransitionEffectData;
    fn default_transition_in_mut(&mut self) -> &mut CameraTransitionEffectData;
    fn default_transition_out(&self) -> &CameraTransitionEffectData;
    fn default_transition_out_mut(&mut self) -> &mut CameraTransitionEffectData;
}

impl CameraTransitionEffectDataPackTrait for CameraTransitionEffectDataPack {
    fn default_transition_in(&self) -> &CameraTransitionEffectData {
        &self.default_transition_in
    }
    fn default_transition_in_mut(&mut self) -> &mut CameraTransitionEffectData {
        &mut self.default_transition_in
    }
    fn default_transition_out(&self) -> &CameraTransitionEffectData {
        &self.default_transition_out
    }
    fn default_transition_out_mut(&mut self) -> &mut CameraTransitionEffectData {
        &mut self.default_transition_out
    }
}

impl super::core::DataContainerTrait for CameraTransitionEffectDataPack {
}

pub static CAMERATRANSITIONEFFECTDATAPACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraTransitionEffectDataPack",
    name_hash: 3431562285,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(CameraTransitionEffectDataPack, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CameraTransitionEffectDataPack as Default>::default())),
            create_boxed: || Box::new(<CameraTransitionEffectDataPack as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "DefaultTransitionIn",
                name_hash: 571763814,
                flags: MemberInfoFlags::new(0),
                field_type: "CameraTransitionEffectData",
                rust_offset: offset_of!(CameraTransitionEffectDataPack, default_transition_in),
            },
            FieldInfoData {
                name: "DefaultTransitionOut",
                name_hash: 1688339567,
                flags: MemberInfoFlags::new(0),
                field_type: "CameraTransitionEffectData",
                rust_offset: offset_of!(CameraTransitionEffectDataPack, default_transition_out),
            },
        ],
    }),
    array_type: Some(CAMERATRANSITIONEFFECTDATAPACK_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CameraTransitionEffectDataPack {
    fn type_info(&self) -> &'static TypeInfo {
        CAMERATRANSITIONEFFECTDATAPACK_TYPE_INFO
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


pub static CAMERATRANSITIONEFFECTDATAPACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraTransitionEffectDataPack-Array",
    name_hash: 881512601,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("CameraTransitionEffectDataPack"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CameraTransitionEffectData {
    pub transition_type: CameraTransitionType,
    pub priority: CameraTransitionPriority,
    pub ease_type: CameraTransitionEase,
    pub duration: f32,
    pub spring_constant: f32,
    pub spring_max_velocity: f32,
}

pub trait CameraTransitionEffectDataTrait: TypeObject {
    fn transition_type(&self) -> &CameraTransitionType;
    fn transition_type_mut(&mut self) -> &mut CameraTransitionType;
    fn priority(&self) -> &CameraTransitionPriority;
    fn priority_mut(&mut self) -> &mut CameraTransitionPriority;
    fn ease_type(&self) -> &CameraTransitionEase;
    fn ease_type_mut(&mut self) -> &mut CameraTransitionEase;
    fn duration(&self) -> &f32;
    fn duration_mut(&mut self) -> &mut f32;
    fn spring_constant(&self) -> &f32;
    fn spring_constant_mut(&mut self) -> &mut f32;
    fn spring_max_velocity(&self) -> &f32;
    fn spring_max_velocity_mut(&mut self) -> &mut f32;
}

impl CameraTransitionEffectDataTrait for CameraTransitionEffectData {
    fn transition_type(&self) -> &CameraTransitionType {
        &self.transition_type
    }
    fn transition_type_mut(&mut self) -> &mut CameraTransitionType {
        &mut self.transition_type
    }
    fn priority(&self) -> &CameraTransitionPriority {
        &self.priority
    }
    fn priority_mut(&mut self) -> &mut CameraTransitionPriority {
        &mut self.priority
    }
    fn ease_type(&self) -> &CameraTransitionEase {
        &self.ease_type
    }
    fn ease_type_mut(&mut self) -> &mut CameraTransitionEase {
        &mut self.ease_type
    }
    fn duration(&self) -> &f32 {
        &self.duration
    }
    fn duration_mut(&mut self) -> &mut f32 {
        &mut self.duration
    }
    fn spring_constant(&self) -> &f32 {
        &self.spring_constant
    }
    fn spring_constant_mut(&mut self) -> &mut f32 {
        &mut self.spring_constant
    }
    fn spring_max_velocity(&self) -> &f32 {
        &self.spring_max_velocity
    }
    fn spring_max_velocity_mut(&mut self) -> &mut f32 {
        &mut self.spring_max_velocity
    }
}

pub static CAMERATRANSITIONEFFECTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraTransitionEffectData",
    name_hash: 1235337332,
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CameraTransitionEffectData as Default>::default())),
            create_boxed: || Box::new(<CameraTransitionEffectData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "TransitionType",
                name_hash: 3976914738,
                flags: MemberInfoFlags::new(0),
                field_type: "CameraTransitionType",
                rust_offset: offset_of!(CameraTransitionEffectData, transition_type),
            },
            FieldInfoData {
                name: "Priority",
                name_hash: 3062102871,
                flags: MemberInfoFlags::new(0),
                field_type: "CameraTransitionPriority",
                rust_offset: offset_of!(CameraTransitionEffectData, priority),
            },
            FieldInfoData {
                name: "EaseType",
                name_hash: 3611554575,
                flags: MemberInfoFlags::new(0),
                field_type: "CameraTransitionEase",
                rust_offset: offset_of!(CameraTransitionEffectData, ease_type),
            },
            FieldInfoData {
                name: "Duration",
                name_hash: 1828507227,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CameraTransitionEffectData, duration),
            },
            FieldInfoData {
                name: "SpringConstant",
                name_hash: 3561415946,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CameraTransitionEffectData, spring_constant),
            },
            FieldInfoData {
                name: "SpringMaxVelocity",
                name_hash: 2631243383,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CameraTransitionEffectData, spring_max_velocity),
            },
        ],
    }),
    array_type: Some(CAMERATRANSITIONEFFECTDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for CameraTransitionEffectData {
    fn type_info(&self) -> &'static TypeInfo {
        CAMERATRANSITIONEFFECTDATA_TYPE_INFO
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


pub static CAMERATRANSITIONEFFECTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraTransitionEffectData-Array",
    name_hash: 2311941184,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("CameraTransitionEffectData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum CameraTransitionPriority {
    #[default]
    CameraTransitionPriority_Low = 0,
    CameraTransitionPriority_Medium = 1,
    CameraTransitionPriority_High = 2,
    CameraTransitionPriority_Count = 3,
}

pub static CAMERATRANSITIONPRIORITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraTransitionPriority",
    name_hash: 3612878305,
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(CAMERATRANSITIONPRIORITY_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CameraTransitionPriority {
    fn type_info(&self) -> &'static TypeInfo {
        CAMERATRANSITIONPRIORITY_TYPE_INFO
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


pub static CAMERATRANSITIONPRIORITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraTransitionPriority-Array",
    name_hash: 892534997,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("CameraTransitionPriority"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum CameraTransitionEase {
    #[default]
    CameraTransitionEase_InOut = 0,
    CameraTransitionEase_In = 1,
    CameraTransitionEase_Out = 2,
    CameraTransitionEase_Count = 3,
}

pub static CAMERATRANSITIONEASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraTransitionEase",
    name_hash: 1187199489,
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(CAMERATRANSITIONEASE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CameraTransitionEase {
    fn type_info(&self) -> &'static TypeInfo {
        CAMERATRANSITIONEASE_TYPE_INFO
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


pub static CAMERATRANSITIONEASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraTransitionEase-Array",
    name_hash: 3744478133,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("CameraTransitionEase"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum CameraTransitionType {
    #[default]
    CameraTransitionType_NotSpecified = 0,
    CameraTransitionType_Cut = 1,
    CameraTransitionType_Linear = 2,
    CameraTransitionType_Quadratic = 3,
    CameraTransitionType_Cubic = 4,
    CameraTransitionType_Exp = 5,
    CameraTransitionType_Spring = 6,
    CameraTransitionType_Count = 7,
}

pub static CAMERATRANSITIONTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraTransitionType",
    name_hash: 1186610571,
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(CAMERATRANSITIONTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CameraTransitionType {
    fn type_info(&self) -> &'static TypeInfo {
        CAMERATRANSITIONTYPE_TYPE_INFO
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


pub static CAMERATRANSITIONTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraTransitionType-Array",
    name_hash: 83078207,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("CameraTransitionType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TargetCameraData {
    pub _glacier_base: CameraData,
    pub hud: Vec<Option<LockedTypeObject /* HudData */>>,
    pub use_transform_space_hierarchy: bool,
}

pub trait TargetCameraDataTrait: CameraDataTrait {
    fn hud(&self) -> &Vec<Option<LockedTypeObject /* HudData */>>;
    fn hud_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* HudData */>>;
    fn use_transform_space_hierarchy(&self) -> &bool;
    fn use_transform_space_hierarchy_mut(&mut self) -> &mut bool;
}

impl TargetCameraDataTrait for TargetCameraData {
    fn hud(&self) -> &Vec<Option<LockedTypeObject /* HudData */>> {
        &self.hud
    }
    fn hud_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* HudData */>> {
        &mut self.hud
    }
    fn use_transform_space_hierarchy(&self) -> &bool {
        &self.use_transform_space_hierarchy
    }
    fn use_transform_space_hierarchy_mut(&mut self) -> &mut bool {
        &mut self.use_transform_space_hierarchy
    }
}

impl CameraDataTrait for TargetCameraData {
    fn occlusion_ray_offset(&self) -> &super::core::Vec3 {
        self._glacier_base.occlusion_ray_offset()
    }
    fn occlusion_ray_offset_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.occlusion_ray_offset_mut()
    }
    fn pre_fade_time(&self) -> &f32 {
        self._glacier_base.pre_fade_time()
    }
    fn pre_fade_time_mut(&mut self) -> &mut f32 {
        self._glacier_base.pre_fade_time_mut()
    }
    fn fade_time(&self) -> &f32 {
        self._glacier_base.fade_time()
    }
    fn fade_time_mut(&mut self) -> &mut f32 {
        self._glacier_base.fade_time_mut()
    }
    fn fade_wait_time(&self) -> &f32 {
        self._glacier_base.fade_wait_time()
    }
    fn fade_wait_time_mut(&mut self) -> &mut f32 {
        self._glacier_base.fade_wait_time_mut()
    }
    fn stay_faded_while_streaming(&self) -> &bool {
        self._glacier_base.stay_faded_while_streaming()
    }
    fn stay_faded_while_streaming_mut(&mut self) -> &mut bool {
        self._glacier_base.stay_faded_while_streaming_mut()
    }
    fn near_plane(&self) -> &f32 {
        self._glacier_base.near_plane()
    }
    fn near_plane_mut(&mut self) -> &mut f32 {
        self._glacier_base.near_plane_mut()
    }
    fn shadow_view_distance_scale(&self) -> &f32 {
        self._glacier_base.shadow_view_distance_scale()
    }
    fn shadow_view_distance_scale_mut(&mut self) -> &mut f32 {
        self._glacier_base.shadow_view_distance_scale_mut()
    }
    fn sound_occlusion(&self) -> &f32 {
        self._glacier_base.sound_occlusion()
    }
    fn sound_occlusion_mut(&mut self) -> &mut f32 {
        self._glacier_base.sound_occlusion_mut()
    }
    fn sound_listener_radius(&self) -> &f32 {
        self._glacier_base.sound_listener_radius()
    }
    fn sound_listener_radius_mut(&mut self) -> &mut f32 {
        self._glacier_base.sound_listener_radius_mut()
    }
    fn sound_listener_fov(&self) -> &f32 {
        self._glacier_base.sound_listener_fov()
    }
    fn sound_listener_fov_mut(&mut self) -> &mut f32 {
        self._glacier_base.sound_listener_fov_mut()
    }
    fn shake_factor(&self) -> &f32 {
        self._glacier_base.shake_factor()
    }
    fn shake_factor_mut(&mut self) -> &mut f32 {
        self._glacier_base.shake_factor_mut()
    }
    fn view_id(&self) -> &super::render_base::LocalPlayerViewId {
        self._glacier_base.view_id()
    }
    fn view_id_mut(&mut self) -> &mut super::render_base::LocalPlayerViewId {
        self._glacier_base.view_id_mut()
    }
    fn transition_data_pack(&self) -> &Option<LockedTypeObject /* CameraTransitionEffectDataPack */> {
        self._glacier_base.transition_data_pack()
    }
    fn transition_data_pack_mut(&mut self) -> &mut Option<LockedTypeObject /* CameraTransitionEffectDataPack */> {
        self._glacier_base.transition_data_pack_mut()
    }
}

impl super::entity::GameObjectDataTrait for TargetCameraData {
}

impl super::core::DataBusPeerTrait for TargetCameraData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for TargetCameraData {
}

impl super::core::DataContainerTrait for TargetCameraData {
}

pub static TARGETCAMERADATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TargetCameraData",
    name_hash: 1675301469,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CAMERADATA_TYPE_INFO),
        super_class_offset: offset_of!(TargetCameraData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TargetCameraData as Default>::default())),
            create_boxed: || Box::new(<TargetCameraData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Hud",
                name_hash: 193458844,
                flags: MemberInfoFlags::new(144),
                field_type: "HudData-Array",
                rust_offset: offset_of!(TargetCameraData, hud),
            },
            FieldInfoData {
                name: "UseTransformSpaceHierarchy",
                name_hash: 1249151385,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TargetCameraData, use_transform_space_hierarchy),
            },
        ],
    }),
    array_type: Some(TARGETCAMERADATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TargetCameraData {
    fn type_info(&self) -> &'static TypeInfo {
        TARGETCAMERADATA_TYPE_INFO
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


pub static TARGETCAMERADATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TargetCameraData-Array",
    name_hash: 3186036329,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("TargetCameraData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct GameModeViewDefinition {
    pub _glacier_base: super::core::Asset,
    pub game_mode_name: String,
    pub view_definitions: Vec<BoxedTypeObject /* PlayerViewDefinition */>,
}

pub trait GameModeViewDefinitionTrait: super::core::AssetTrait {
    fn game_mode_name(&self) -> &String;
    fn game_mode_name_mut(&mut self) -> &mut String;
    fn view_definitions(&self) -> &Vec<BoxedTypeObject /* PlayerViewDefinition */>;
    fn view_definitions_mut(&mut self) -> &mut Vec<BoxedTypeObject /* PlayerViewDefinition */>;
}

impl GameModeViewDefinitionTrait for GameModeViewDefinition {
    fn game_mode_name(&self) -> &String {
        &self.game_mode_name
    }
    fn game_mode_name_mut(&mut self) -> &mut String {
        &mut self.game_mode_name
    }
    fn view_definitions(&self) -> &Vec<BoxedTypeObject /* PlayerViewDefinition */> {
        &self.view_definitions
    }
    fn view_definitions_mut(&mut self) -> &mut Vec<BoxedTypeObject /* PlayerViewDefinition */> {
        &mut self.view_definitions
    }
}

impl super::core::AssetTrait for GameModeViewDefinition {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for GameModeViewDefinition {
}

pub static GAMEMODEVIEWDEFINITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameModeViewDefinition",
    name_hash: 3684567664,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(GameModeViewDefinition, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GameModeViewDefinition as Default>::default())),
            create_boxed: || Box::new(<GameModeViewDefinition as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "GameModeName",
                name_hash: 2292982255,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(GameModeViewDefinition, game_mode_name),
            },
            FieldInfoData {
                name: "ViewDefinitions",
                name_hash: 3389051406,
                flags: MemberInfoFlags::new(144),
                field_type: "PlayerViewDefinition-Array",
                rust_offset: offset_of!(GameModeViewDefinition, view_definitions),
            },
        ],
    }),
    array_type: Some(GAMEMODEVIEWDEFINITION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GameModeViewDefinition {
    fn type_info(&self) -> &'static TypeInfo {
        GAMEMODEVIEWDEFINITION_TYPE_INFO
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


pub static GAMEMODEVIEWDEFINITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameModeViewDefinition-Array",
    name_hash: 3709438020,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("GameModeViewDefinition"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PlayerViewDefinition {
    pub local_player_id: super::core::LocalPlayerId,
    pub views: Vec<BoxedTypeObject /* super::render_base::ViewDefinition */>,
}

pub trait PlayerViewDefinitionTrait: TypeObject {
    fn local_player_id(&self) -> &super::core::LocalPlayerId;
    fn local_player_id_mut(&mut self) -> &mut super::core::LocalPlayerId;
    fn views(&self) -> &Vec<BoxedTypeObject /* super::render_base::ViewDefinition */>;
    fn views_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::render_base::ViewDefinition */>;
}

impl PlayerViewDefinitionTrait for PlayerViewDefinition {
    fn local_player_id(&self) -> &super::core::LocalPlayerId {
        &self.local_player_id
    }
    fn local_player_id_mut(&mut self) -> &mut super::core::LocalPlayerId {
        &mut self.local_player_id
    }
    fn views(&self) -> &Vec<BoxedTypeObject /* super::render_base::ViewDefinition */> {
        &self.views
    }
    fn views_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::render_base::ViewDefinition */> {
        &mut self.views
    }
}

pub static PLAYERVIEWDEFINITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayerViewDefinition",
    name_hash: 1220932590,
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PlayerViewDefinition as Default>::default())),
            create_boxed: || Box::new(<PlayerViewDefinition as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "LocalPlayerId",
                name_hash: 1029133718,
                flags: MemberInfoFlags::new(0),
                field_type: "LocalPlayerId",
                rust_offset: offset_of!(PlayerViewDefinition, local_player_id),
            },
            FieldInfoData {
                name: "Views",
                name_hash: 225652731,
                flags: MemberInfoFlags::new(144),
                field_type: "ViewDefinition-Array",
                rust_offset: offset_of!(PlayerViewDefinition, views),
            },
        ],
    }),
    array_type: Some(PLAYERVIEWDEFINITION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PlayerViewDefinition {
    fn type_info(&self) -> &'static TypeInfo {
        PLAYERVIEWDEFINITION_TYPE_INFO
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


pub static PLAYERVIEWDEFINITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayerViewDefinition-Array",
    name_hash: 3083013338,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("PlayerViewDefinition"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum CameraIds {
    #[default]
    NoCameraId = 0,
    FreeCameraId = 1,
    EntryCameraId = 2,
    CameraIdCount = 3,
}

pub static CAMERAIDS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraIds",
    name_hash: 1789538274,
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(CAMERAIDS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CameraIds {
    fn type_info(&self) -> &'static TypeInfo {
        CAMERAIDS_TYPE_INFO
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


pub static CAMERAIDS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraIds-Array",
    name_hash: 4146413270,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("CameraIds"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CameraData {
    pub _glacier_base: super::entity::GameObjectData,
    pub occlusion_ray_offset: super::core::Vec3,
    pub pre_fade_time: f32,
    pub fade_time: f32,
    pub fade_wait_time: f32,
    pub stay_faded_while_streaming: bool,
    pub near_plane: f32,
    pub shadow_view_distance_scale: f32,
    pub sound_occlusion: f32,
    pub sound_listener_radius: f32,
    pub sound_listener_fov: f32,
    pub shake_factor: f32,
    pub view_id: super::render_base::LocalPlayerViewId,
    pub transition_data_pack: Option<LockedTypeObject /* CameraTransitionEffectDataPack */>,
}

pub trait CameraDataTrait: super::entity::GameObjectDataTrait {
    fn occlusion_ray_offset(&self) -> &super::core::Vec3;
    fn occlusion_ray_offset_mut(&mut self) -> &mut super::core::Vec3;
    fn pre_fade_time(&self) -> &f32;
    fn pre_fade_time_mut(&mut self) -> &mut f32;
    fn fade_time(&self) -> &f32;
    fn fade_time_mut(&mut self) -> &mut f32;
    fn fade_wait_time(&self) -> &f32;
    fn fade_wait_time_mut(&mut self) -> &mut f32;
    fn stay_faded_while_streaming(&self) -> &bool;
    fn stay_faded_while_streaming_mut(&mut self) -> &mut bool;
    fn near_plane(&self) -> &f32;
    fn near_plane_mut(&mut self) -> &mut f32;
    fn shadow_view_distance_scale(&self) -> &f32;
    fn shadow_view_distance_scale_mut(&mut self) -> &mut f32;
    fn sound_occlusion(&self) -> &f32;
    fn sound_occlusion_mut(&mut self) -> &mut f32;
    fn sound_listener_radius(&self) -> &f32;
    fn sound_listener_radius_mut(&mut self) -> &mut f32;
    fn sound_listener_fov(&self) -> &f32;
    fn sound_listener_fov_mut(&mut self) -> &mut f32;
    fn shake_factor(&self) -> &f32;
    fn shake_factor_mut(&mut self) -> &mut f32;
    fn view_id(&self) -> &super::render_base::LocalPlayerViewId;
    fn view_id_mut(&mut self) -> &mut super::render_base::LocalPlayerViewId;
    fn transition_data_pack(&self) -> &Option<LockedTypeObject /* CameraTransitionEffectDataPack */>;
    fn transition_data_pack_mut(&mut self) -> &mut Option<LockedTypeObject /* CameraTransitionEffectDataPack */>;
}

impl CameraDataTrait for CameraData {
    fn occlusion_ray_offset(&self) -> &super::core::Vec3 {
        &self.occlusion_ray_offset
    }
    fn occlusion_ray_offset_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.occlusion_ray_offset
    }
    fn pre_fade_time(&self) -> &f32 {
        &self.pre_fade_time
    }
    fn pre_fade_time_mut(&mut self) -> &mut f32 {
        &mut self.pre_fade_time
    }
    fn fade_time(&self) -> &f32 {
        &self.fade_time
    }
    fn fade_time_mut(&mut self) -> &mut f32 {
        &mut self.fade_time
    }
    fn fade_wait_time(&self) -> &f32 {
        &self.fade_wait_time
    }
    fn fade_wait_time_mut(&mut self) -> &mut f32 {
        &mut self.fade_wait_time
    }
    fn stay_faded_while_streaming(&self) -> &bool {
        &self.stay_faded_while_streaming
    }
    fn stay_faded_while_streaming_mut(&mut self) -> &mut bool {
        &mut self.stay_faded_while_streaming
    }
    fn near_plane(&self) -> &f32 {
        &self.near_plane
    }
    fn near_plane_mut(&mut self) -> &mut f32 {
        &mut self.near_plane
    }
    fn shadow_view_distance_scale(&self) -> &f32 {
        &self.shadow_view_distance_scale
    }
    fn shadow_view_distance_scale_mut(&mut self) -> &mut f32 {
        &mut self.shadow_view_distance_scale
    }
    fn sound_occlusion(&self) -> &f32 {
        &self.sound_occlusion
    }
    fn sound_occlusion_mut(&mut self) -> &mut f32 {
        &mut self.sound_occlusion
    }
    fn sound_listener_radius(&self) -> &f32 {
        &self.sound_listener_radius
    }
    fn sound_listener_radius_mut(&mut self) -> &mut f32 {
        &mut self.sound_listener_radius
    }
    fn sound_listener_fov(&self) -> &f32 {
        &self.sound_listener_fov
    }
    fn sound_listener_fov_mut(&mut self) -> &mut f32 {
        &mut self.sound_listener_fov
    }
    fn shake_factor(&self) -> &f32 {
        &self.shake_factor
    }
    fn shake_factor_mut(&mut self) -> &mut f32 {
        &mut self.shake_factor
    }
    fn view_id(&self) -> &super::render_base::LocalPlayerViewId {
        &self.view_id
    }
    fn view_id_mut(&mut self) -> &mut super::render_base::LocalPlayerViewId {
        &mut self.view_id
    }
    fn transition_data_pack(&self) -> &Option<LockedTypeObject /* CameraTransitionEffectDataPack */> {
        &self.transition_data_pack
    }
    fn transition_data_pack_mut(&mut self) -> &mut Option<LockedTypeObject /* CameraTransitionEffectDataPack */> {
        &mut self.transition_data_pack
    }
}

impl super::entity::GameObjectDataTrait for CameraData {
}

impl super::core::DataBusPeerTrait for CameraData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for CameraData {
}

impl super::core::DataContainerTrait for CameraData {
}

pub static CAMERADATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraData",
    name_hash: 3220090252,
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::GAMEOBJECTDATA_TYPE_INFO),
        super_class_offset: offset_of!(CameraData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CameraData as Default>::default())),
            create_boxed: || Box::new(<CameraData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "OcclusionRayOffset",
                name_hash: 2352466319,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(CameraData, occlusion_ray_offset),
            },
            FieldInfoData {
                name: "PreFadeTime",
                name_hash: 3677592337,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CameraData, pre_fade_time),
            },
            FieldInfoData {
                name: "FadeTime",
                name_hash: 4001222838,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CameraData, fade_time),
            },
            FieldInfoData {
                name: "FadeWaitTime",
                name_hash: 3603496477,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CameraData, fade_wait_time),
            },
            FieldInfoData {
                name: "StayFadedWhileStreaming",
                name_hash: 4069555099,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CameraData, stay_faded_while_streaming),
            },
            FieldInfoData {
                name: "NearPlane",
                name_hash: 3156145579,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CameraData, near_plane),
            },
            FieldInfoData {
                name: "ShadowViewDistanceScale",
                name_hash: 2622540501,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CameraData, shadow_view_distance_scale),
            },
            FieldInfoData {
                name: "SoundOcclusion",
                name_hash: 3140497259,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CameraData, sound_occlusion),
            },
            FieldInfoData {
                name: "SoundListenerRadius",
                name_hash: 3076143232,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CameraData, sound_listener_radius),
            },
            FieldInfoData {
                name: "SoundListenerFov",
                name_hash: 968288039,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CameraData, sound_listener_fov),
            },
            FieldInfoData {
                name: "ShakeFactor",
                name_hash: 1800518652,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CameraData, shake_factor),
            },
            FieldInfoData {
                name: "ViewId",
                name_hash: 3151570821,
                flags: MemberInfoFlags::new(0),
                field_type: "LocalPlayerViewId",
                rust_offset: offset_of!(CameraData, view_id),
            },
            FieldInfoData {
                name: "TransitionDataPack",
                name_hash: 600691939,
                flags: MemberInfoFlags::new(0),
                field_type: "CameraTransitionEffectDataPack",
                rust_offset: offset_of!(CameraData, transition_data_pack),
            },
        ],
    }),
    array_type: Some(CAMERADATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CameraData {
    fn type_info(&self) -> &'static TypeInfo {
        CAMERADATA_TYPE_INFO
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


pub static CAMERADATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraData-Array",
    name_hash: 3673718200,
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("CameraData"),
    array_type: None,
    alignment: 8,
};


