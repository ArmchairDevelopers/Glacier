use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TargetCamera {
}

pub const TARGETCAMERA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TargetCamera",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CAMERA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TARGETCAMERA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TargetCamera {
    fn type_info() -> &'static TypeInfo {
        TARGETCAMERA_TYPE_INFO
    }
}


pub const TARGETCAMERA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TargetCamera-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("TargetCamera-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TargetCameraCallback {
}

pub const TARGETCAMERACALLBACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TargetCameraCallback",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(TARGETCAMERACALLBACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TargetCameraCallback {
    fn type_info() -> &'static TypeInfo {
        TARGETCAMERACALLBACK_TYPE_INFO
    }
}


pub const TARGETCAMERACALLBACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TargetCameraCallback-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("TargetCameraCallback-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FreeCamera {
}

pub const FREECAMERA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FreeCamera",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CAMERA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FREECAMERA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FreeCamera {
    fn type_info() -> &'static TypeInfo {
        FREECAMERA_TYPE_INFO
    }
}


pub const FREECAMERA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FreeCamera-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("FreeCamera-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Camera {
}

pub const CAMERA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Camera",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(CAMERA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Camera {
    fn type_info() -> &'static TypeInfo {
        CAMERA_TYPE_INFO
    }
}


pub const CAMERA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Camera-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("Camera-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AntAnimatableEntity {
}

pub const ANTANIMATABLEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntAnimatableEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ANTANIMATABLEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AntAnimatableEntity {
    fn type_info() -> &'static TypeInfo {
        ANTANIMATABLEENTITY_TYPE_INFO
    }
}


pub const ANTANIMATABLEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntAnimatableEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("AntAnimatableEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DifficultyDatasBase {
}

pub const DIFFICULTYDATASBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DifficultyDatasBase",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DIFFICULTYDATASBASE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DifficultyDatasBase {
    fn type_info() -> &'static TypeInfo {
        DIFFICULTYDATASBASE_TYPE_INFO
    }
}


pub const DIFFICULTYDATASBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DifficultyDatasBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("DifficultyDatasBase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum LockType {
    #[default]
    LockAlways = 0,
    LockOnRadar = 1,
    LockOnHeat = 2,
    LockOnLaserPainted = 3,
    LockNever = 4,
    LockTypeCount = 5,
}

pub const LOCKTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LockType",
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(LOCKTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LockType {
    fn type_info() -> &'static TypeInfo {
        LOCKTYPE_TYPE_INFO
    }
}


pub const LOCKTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LockType-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("LockType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ToolData {
    pub is_always_active: bool,
}

pub const TOOLDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ToolData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "IsAlwaysActive",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ToolData, is_always_active),
            },
        ],
    }),
    array_type: Some(TOOLDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ToolData {
    fn type_info() -> &'static TypeInfo {
        TOOLDATA_TYPE_INFO
    }
}


pub const TOOLDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ToolData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ToolData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum VoiceChannel {
    #[default]
    VoiceChannel_Off = 0,
    VoiceChannel_Team = 1,
    VoiceChannel_Squad = 2,
    VoiceChannel_Loopback = 3,
    VoiceChannel_Count = 4,
}

pub const VOICECHANNEL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceChannel",
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(VOICECHANNEL_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for VoiceChannel {
    fn type_info() -> &'static TypeInfo {
        VOICECHANNEL_TYPE_INFO
    }
}


pub const VOICECHANNEL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceChannel-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("VoiceChannel-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
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

pub const SQUADID_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SquadId",
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(SQUADID_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SquadId {
    fn type_info() -> &'static TypeInfo {
        SQUADID_TYPE_INFO
    }
}


pub const SQUADID_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SquadId-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("SquadId-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum FactionId {
    #[default]
    FactionNeutral = 0,
    FactionUS = 1,
    FactionRUS = 2,
    FactionMEC = 3,
    FactionIdCount = 4,
    FactionInvalid = 5,
}

pub const FACTIONID_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FactionId",
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(FACTIONID_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for FactionId {
    fn type_info() -> &'static TypeInfo {
        FACTIONID_TYPE_INFO
    }
}


pub const FACTIONID_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FactionId-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("FactionId-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
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

pub const TEAMID_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TeamId",
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(TEAMID_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TeamId {
    fn type_info() -> &'static TypeInfo {
        TEAMID_TYPE_INFO
    }
}


pub const TEAMID_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TeamId-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("TeamId-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum IKEffectorEnum {
    #[default]
    IKLeftHand = 0,
    IKRightHand = 1,
}

pub const IKEFFECTORENUM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IKEffectorEnum",
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(IKEFFECTORENUM_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for IKEffectorEnum {
    fn type_info() -> &'static TypeInfo {
        IKEFFECTORENUM_TYPE_INFO
    }
}


pub const IKEFFECTORENUM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IKEffectorEnum-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("IKEffectorEnum-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum RotationAxis {
    #[default]
    raX = 0,
    raY = 1,
    raZ = 2,
}

pub const ROTATIONAXIS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RotationAxis",
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(ROTATIONAXIS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RotationAxis {
    fn type_info() -> &'static TypeInfo {
        ROTATIONAXIS_TYPE_INFO
    }
}


pub const ROTATIONAXIS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RotationAxis-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("RotationAxis-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum WheelPhysicsType {
    #[default]
    wptNormal = 0,
    wptFollow = 1,
    wptStatic = 2,
}

pub const WHEELPHYSICSTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WheelPhysicsType",
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(WHEELPHYSICSTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for WheelPhysicsType {
    fn type_info() -> &'static TypeInfo {
        WHEELPHYSICSTYPE_TYPE_INFO
    }
}


pub const WHEELPHYSICSTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WheelPhysicsType-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("WheelPhysicsType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EntryClass {
    #[default]
    ecPrimary = 0,
    ecSecondary = 1,
}

pub const ENTRYCLASS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntryClass",
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(ENTRYCLASS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EntryClass {
    fn type_info() -> &'static TypeInfo {
        ENTRYCLASS_TYPE_INFO
    }
}


pub const ENTRYCLASS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntryClass-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("EntryClass-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ShaderParameterComponentData {
    pub shader_parameter_vectors: Vec<ShaderParameterVector>,
    pub shader_parameter_texture: Vec<ShaderParameterTexture>,
}

pub const SHADERPARAMETERCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterComponentData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMECOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ShaderParameterVectors",
                flags: MemberInfoFlags::new(144),
                field_type: SHADERPARAMETERVECTOR_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ShaderParameterComponentData, shader_parameter_vectors),
            },
            FieldInfoData {
                name: "ShaderParameterTexture",
                flags: MemberInfoFlags::new(144),
                field_type: SHADERPARAMETERTEXTURE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ShaderParameterComponentData, shader_parameter_texture),
            },
        ],
    }),
    array_type: Some(SHADERPARAMETERCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ShaderParameterComponentData {
    fn type_info() -> &'static TypeInfo {
        SHADERPARAMETERCOMPONENTDATA_TYPE_INFO
    }
}


pub const SHADERPARAMETERCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ShaderParameterComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShaderParameterTexture {
    pub parameter_name: String,
    pub texture: super::render_base::TextureBaseAsset,
}

pub const SHADERPARAMETERTEXTURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterTexture",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ShaderParameterTexture, parameter_name),
            },
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(ShaderParameterTexture, texture),
            },
        ],
    }),
    array_type: Some(SHADERPARAMETERTEXTURE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ShaderParameterTexture {
    fn type_info() -> &'static TypeInfo {
        SHADERPARAMETERTEXTURE_TYPE_INFO
    }
}


pub const SHADERPARAMETERTEXTURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterTexture-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ShaderParameterTexture-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ShaderParameterVector {
    pub parameter_name: String,
    pub value: super::core::Vec4,
}

pub const SHADERPARAMETERVECTOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterVector",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ShaderParameterVector, parameter_name),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(ShaderParameterVector, value),
            },
        ],
    }),
    array_type: Some(SHADERPARAMETERVECTOR_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ShaderParameterVector {
    fn type_info() -> &'static TypeInfo {
        SHADERPARAMETERVECTOR_TYPE_INFO
    }
}


pub const SHADERPARAMETERVECTOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterVector-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ShaderParameterVector-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ComponentEnabledMessage {
}

pub const COMPONENTENABLEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ComponentEnabledMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ComponentEnabledMessage {
    fn type_info() -> &'static TypeInfo {
        COMPONENTENABLEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ControllableHealthComponentData {
}

pub const CONTROLLABLEHEALTHCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ControllableHealthComponentData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMEHEALTHCOMPONENTDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CONTROLLABLEHEALTHCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ControllableHealthComponentData {
    fn type_info() -> &'static TypeInfo {
        CONTROLLABLEHEALTHCOMPONENTDATA_TYPE_INFO
    }
}


pub const CONTROLLABLEHEALTHCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ControllableHealthComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ControllableHealthComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct GameHealthComponentData {
}

pub const GAMEHEALTHCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameHealthComponentData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(HEALTHCOMPONENTDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(GAMEHEALTHCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for GameHealthComponentData {
    fn type_info() -> &'static TypeInfo {
        GAMEHEALTHCOMPONENTDATA_TYPE_INFO
    }
}


pub const GAMEHEALTHCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameHealthComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("GameHealthComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DefaultHealthComponentData {
}

pub const DEFAULTHEALTHCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DefaultHealthComponentData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(HEALTHCOMPONENTDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DEFAULTHEALTHCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DefaultHealthComponentData {
    fn type_info() -> &'static TypeInfo {
        DEFAULTHEALTHCOMPONENTDATA_TYPE_INFO
    }
}


pub const DEFAULTHEALTHCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DefaultHealthComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("DefaultHealthComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct HealthComponentData {
}

pub const HEALTHCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthComponentData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENTDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(HEALTHCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for HealthComponentData {
    fn type_info() -> &'static TypeInfo {
        HEALTHCOMPONENTDATA_TYPE_INFO
    }
}


pub const HEALTHCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("HealthComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HealthComponentOnDamageMessage {
}

pub const HEALTHCOMPONENTONDAMAGEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthComponentOnDamageMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for HealthComponentOnDamageMessage {
    fn type_info() -> &'static TypeInfo {
        HEALTHCOMPONENTONDAMAGEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ControllablePhysicsComponentData {
}

pub const CONTROLLABLEPHYSICSCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ControllablePhysicsComponentData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMEPHYSICSCOMPONENTDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CONTROLLABLEPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ControllablePhysicsComponentData {
    fn type_info() -> &'static TypeInfo {
        CONTROLLABLEPHYSICSCOMPONENTDATA_TYPE_INFO
    }
}


pub const CONTROLLABLEPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ControllablePhysicsComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ControllablePhysicsComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct GamePhysicsComponentData {
    pub realm: super::core::Realm,
    pub effect_parameters: Vec<super::effect_base::EffectParameter>,
}

pub const GAMEPHYSICSCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GamePhysicsComponentData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(GamePhysicsComponentData, realm),
            },
            FieldInfoData {
                name: "EffectParameters",
                flags: MemberInfoFlags::new(144),
                field_type: EFFECTPARAMETER_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(GamePhysicsComponentData, effect_parameters),
            },
        ],
    }),
    array_type: Some(GAMEPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for GamePhysicsComponentData {
    fn type_info() -> &'static TypeInfo {
        GAMEPHYSICSCOMPONENTDATA_TYPE_INFO
    }
}


pub const GAMEPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GamePhysicsComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("GamePhysicsComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TwoPartComponentData {
}

pub const TWOPARTCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TwoPartComponentData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMECOMPONENTDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TWOPARTCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TwoPartComponentData {
    fn type_info() -> &'static TypeInfo {
        TWOPARTCOMPONENTDATA_TYPE_INFO
    }
}


pub const TWOPARTCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TwoPartComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("TwoPartComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PartComponentData {
    pub health_states: Vec<HealthStateData>,
    pub part_links: Vec<PartLinkData>,
    pub is_supported: bool,
    pub is_fragile: bool,
    pub is_networkable: bool,
    pub animate_physics: bool,
}

pub const PARTCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartComponentData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMECOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "HealthStates",
                flags: MemberInfoFlags::new(144),
                field_type: HEALTHSTATEDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(PartComponentData, health_states),
            },
            FieldInfoData {
                name: "PartLinks",
                flags: MemberInfoFlags::new(144),
                field_type: PARTLINKDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(PartComponentData, part_links),
            },
            FieldInfoData {
                name: "IsSupported",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PartComponentData, is_supported),
            },
            FieldInfoData {
                name: "IsFragile",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PartComponentData, is_fragile),
            },
            FieldInfoData {
                name: "IsNetworkable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PartComponentData, is_networkable),
            },
            FieldInfoData {
                name: "AnimatePhysics",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PartComponentData, animate_physics),
            },
        ],
    }),
    array_type: Some(PARTCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PartComponentData {
    fn type_info() -> &'static TypeInfo {
        PARTCOMPONENTDATA_TYPE_INFO
    }
}


pub const PARTCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("PartComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PartLinkData {
    pub part_component_index1: u32,
    pub part_component_index2: u32,
}

pub const PARTLINKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartLinkData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "PartComponentIndex1",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PartLinkData, part_component_index1),
            },
            FieldInfoData {
                name: "PartComponentIndex2",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PartLinkData, part_component_index2),
            },
        ],
    }),
    array_type: Some(PARTLINKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PartLinkData {
    fn type_info() -> &'static TypeInfo {
        PARTLINKDATA_TYPE_INFO
    }
}


pub const PARTLINKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartLinkData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("PartLinkData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum PartComponentConstants {
    #[default]
    MaxHealthStateCount = 256,
}

pub const PARTCOMPONENTCONSTANTS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartComponentConstants",
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(PARTCOMPONENTCONSTANTS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PartComponentConstants {
    fn type_info() -> &'static TypeInfo {
        PARTCOMPONENTCONSTANTS_TYPE_INFO
    }
}


pub const PARTCOMPONENTCONSTANTS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartComponentConstants-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("PartComponentConstants-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct HealthStateData {
    pub objects: Vec<super::entity::GameObjectData>,
    pub loose_part_physics: Vec<super::physics::NetworkableLoosePartPhysicsData>,
    pub spawned_banger_blueprint: super::entity::ObjectBlueprint,
    pub copy_damage_to_banger: bool,
    pub spawned_banger_impulse_params: BangerSpawnImpulseParams,
    pub physics_enabled: bool,
    pub health: f32,
    pub spawn_objects_culling_params: HealthStateSpawnObjectsCullingParams,
    pub part_index: u32,
    pub can_support_other_parts: bool,
}

pub const HEALTHSTATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthStateData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMEOBJECTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Objects",
                flags: MemberInfoFlags::new(144),
                field_type: GAMEOBJECTDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(HealthStateData, objects),
            },
            FieldInfoData {
                name: "LoosePartPhysics",
                flags: MemberInfoFlags::new(144),
                field_type: NETWORKABLELOOSEPARTPHYSICSDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(HealthStateData, loose_part_physics),
            },
            FieldInfoData {
                name: "SpawnedBangerBlueprint",
                flags: MemberInfoFlags::new(0),
                field_type: OBJECTBLUEPRINT_TYPE_INFO,
                rust_offset: offset_of!(HealthStateData, spawned_banger_blueprint),
            },
            FieldInfoData {
                name: "CopyDamageToBanger",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(HealthStateData, copy_damage_to_banger),
            },
            FieldInfoData {
                name: "SpawnedBangerImpulseParams",
                flags: MemberInfoFlags::new(0),
                field_type: BANGERSPAWNIMPULSEPARAMS_TYPE_INFO,
                rust_offset: offset_of!(HealthStateData, spawned_banger_impulse_params),
            },
            FieldInfoData {
                name: "PhysicsEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(HealthStateData, physics_enabled),
            },
            FieldInfoData {
                name: "Health",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HealthStateData, health),
            },
            FieldInfoData {
                name: "SpawnObjectsCullingParams",
                flags: MemberInfoFlags::new(0),
                field_type: HEALTHSTATESPAWNOBJECTSCULLINGPARAMS_TYPE_INFO,
                rust_offset: offset_of!(HealthStateData, spawn_objects_culling_params),
            },
            FieldInfoData {
                name: "PartIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(HealthStateData, part_index),
            },
            FieldInfoData {
                name: "CanSupportOtherParts",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(HealthStateData, can_support_other_parts),
            },
        ],
    }),
    array_type: Some(HEALTHSTATEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for HealthStateData {
    fn type_info() -> &'static TypeInfo {
        HEALTHSTATEDATA_TYPE_INFO
    }
}


pub const HEALTHSTATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthStateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("HealthStateData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct HealthStateSpawnObjectsCullingParams {
    pub debris_cull_distance: f32,
    pub debris_cull_distance_outside_view: f32,
    pub sound_cull_distance: f32,
    pub sound_cull_distance_outside_view: f32,
    pub effect_cull_distance: f32,
    pub effect_cull_distance_outside_view: f32,
}

pub const HEALTHSTATESPAWNOBJECTSCULLINGPARAMS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthStateSpawnObjectsCullingParams",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "DebrisCullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HealthStateSpawnObjectsCullingParams, debris_cull_distance),
            },
            FieldInfoData {
                name: "DebrisCullDistanceOutsideView",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HealthStateSpawnObjectsCullingParams, debris_cull_distance_outside_view),
            },
            FieldInfoData {
                name: "SoundCullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HealthStateSpawnObjectsCullingParams, sound_cull_distance),
            },
            FieldInfoData {
                name: "SoundCullDistanceOutsideView",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HealthStateSpawnObjectsCullingParams, sound_cull_distance_outside_view),
            },
            FieldInfoData {
                name: "EffectCullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HealthStateSpawnObjectsCullingParams, effect_cull_distance),
            },
            FieldInfoData {
                name: "EffectCullDistanceOutsideView",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HealthStateSpawnObjectsCullingParams, effect_cull_distance_outside_view),
            },
        ],
    }),
    array_type: Some(HEALTHSTATESPAWNOBJECTSCULLINGPARAMS_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for HealthStateSpawnObjectsCullingParams {
    fn type_info() -> &'static TypeInfo {
        HEALTHSTATESPAWNOBJECTSCULLINGPARAMS_TYPE_INFO
    }
}


pub const HEALTHSTATESPAWNOBJECTSCULLINGPARAMS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthStateSpawnObjectsCullingParams-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("HealthStateSpawnObjectsCullingParams-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct BangerSpawnImpulseParams {
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

pub const BANGERSPAWNIMPULSEPARAMS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BangerSpawnImpulseParams",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MinHorizontalAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BangerSpawnImpulseParams, min_horizontal_angle),
            },
            FieldInfoData {
                name: "MaxHorizontalAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BangerSpawnImpulseParams, max_horizontal_angle),
            },
            FieldInfoData {
                name: "MinVerticalAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BangerSpawnImpulseParams, min_vertical_angle),
            },
            FieldInfoData {
                name: "MaxVerticalAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BangerSpawnImpulseParams, max_vertical_angle),
            },
            FieldInfoData {
                name: "MinStrength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BangerSpawnImpulseParams, min_strength),
            },
            FieldInfoData {
                name: "MaxStrength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BangerSpawnImpulseParams, max_strength),
            },
            FieldInfoData {
                name: "WaterStrengthModifier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BangerSpawnImpulseParams, water_strength_modifier),
            },
            FieldInfoData {
                name: "LinearDamping",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BangerSpawnImpulseParams, linear_damping),
            },
            FieldInfoData {
                name: "AngularDamping",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BangerSpawnImpulseParams, angular_damping),
            },
        ],
    }),
    array_type: Some(BANGERSPAWNIMPULSEPARAMS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BangerSpawnImpulseParams {
    fn type_info() -> &'static TypeInfo {
        BANGERSPAWNIMPULSEPARAMS_TYPE_INFO
    }
}


pub const BANGERSPAWNIMPULSEPARAMS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BangerSpawnImpulseParams-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("BangerSpawnImpulseParams-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PartComponentTrackHealthStateSpawnsMessage {
}

pub const PARTCOMPONENTTRACKHEALTHSTATESPAWNSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartComponentTrackHealthStateSpawnsMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for PartComponentTrackHealthStateSpawnsMessage {
    fn type_info() -> &'static TypeInfo {
        PARTCOMPONENTTRACKHEALTHSTATESPAWNSMESSAGE_TYPE_INFO
    }
}

#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum TrackHealthStateSpawns {
    #[default]
    TrackHealthStateSpawns_All = 0,
    TrackHealthStateSpawns_AllButLastHealthState = 1,
    TrackHealthStateSpawns_None = 2,
}

pub const TRACKHEALTHSTATESPAWNS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TrackHealthStateSpawns",
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(TRACKHEALTHSTATESPAWNS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TrackHealthStateSpawns {
    fn type_info() -> &'static TypeInfo {
        TRACKHEALTHSTATESPAWNS_TYPE_INFO
    }
}


pub const TRACKHEALTHSTATESPAWNS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TrackHealthStateSpawns-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("TrackHealthStateSpawns-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BlueprintBundleStatusEntityData {
    pub stream_realm: super::entity::StreamRealm,
    pub bundle_name: String,
}

pub const BLUEPRINTBUNDLESTATUSENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundleStatusEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "StreamRealm",
                flags: MemberInfoFlags::new(0),
                field_type: STREAMREALM_TYPE_INFO,
                rust_offset: offset_of!(BlueprintBundleStatusEntityData, stream_realm),
            },
            FieldInfoData {
                name: "BundleName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(BlueprintBundleStatusEntityData, bundle_name),
            },
        ],
    }),
    array_type: Some(BLUEPRINTBUNDLESTATUSENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BlueprintBundleStatusEntityData {
    fn type_info() -> &'static TypeInfo {
        BLUEPRINTBUNDLESTATUSENTITYDATA_TYPE_INFO
    }
}


pub const BLUEPRINTBUNDLESTATUSENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundleStatusEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("BlueprintBundleStatusEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BlueprintBundleCollectionEntityData {
    pub stream_realm: super::entity::StreamRealm,
    pub collection: BlueprintBundleCollection,
    pub active_index: i32,
}

pub const BLUEPRINTBUNDLECOLLECTIONENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundleCollectionEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "StreamRealm",
                flags: MemberInfoFlags::new(0),
                field_type: STREAMREALM_TYPE_INFO,
                rust_offset: offset_of!(BlueprintBundleCollectionEntityData, stream_realm),
            },
            FieldInfoData {
                name: "Collection",
                flags: MemberInfoFlags::new(0),
                field_type: BLUEPRINTBUNDLECOLLECTION_TYPE_INFO,
                rust_offset: offset_of!(BlueprintBundleCollectionEntityData, collection),
            },
            FieldInfoData {
                name: "ActiveIndex",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(BlueprintBundleCollectionEntityData, active_index),
            },
        ],
    }),
    array_type: Some(BLUEPRINTBUNDLECOLLECTIONENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BlueprintBundleCollectionEntityData {
    fn type_info() -> &'static TypeInfo {
        BLUEPRINTBUNDLECOLLECTIONENTITYDATA_TYPE_INFO
    }
}


pub const BLUEPRINTBUNDLECOLLECTIONENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundleCollectionEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("BlueprintBundleCollectionEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BlueprintBundleEntityData {
    pub stream_realm: super::entity::StreamRealm,
    pub bundle: BlueprintBundleReference,
}

pub const BLUEPRINTBUNDLEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundleEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "StreamRealm",
                flags: MemberInfoFlags::new(0),
                field_type: STREAMREALM_TYPE_INFO,
                rust_offset: offset_of!(BlueprintBundleEntityData, stream_realm),
            },
            FieldInfoData {
                name: "Bundle",
                flags: MemberInfoFlags::new(0),
                field_type: BLUEPRINTBUNDLEREFERENCE_TYPE_INFO,
                rust_offset: offset_of!(BlueprintBundleEntityData, bundle),
            },
        ],
    }),
    array_type: Some(BLUEPRINTBUNDLEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BlueprintBundleEntityData {
    fn type_info() -> &'static TypeInfo {
        BLUEPRINTBUNDLEENTITYDATA_TYPE_INFO
    }
}


pub const BLUEPRINTBUNDLEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundleEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("BlueprintBundleEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RuntimeAssetBlueprint {
    pub asset: super::core::Asset,
}

pub const RUNTIMEASSETBLUEPRINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RuntimeAssetBlueprint",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BLUEPRINT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Asset",
                flags: MemberInfoFlags::new(0),
                field_type: ASSET_TYPE_INFO,
                rust_offset: offset_of!(RuntimeAssetBlueprint, asset),
            },
        ],
    }),
    array_type: Some(RUNTIMEASSETBLUEPRINT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RuntimeAssetBlueprint {
    fn type_info() -> &'static TypeInfo {
        RUNTIMEASSETBLUEPRINT_TYPE_INFO
    }
}


pub const RUNTIMEASSETBLUEPRINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RuntimeAssetBlueprint-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("RuntimeAssetBlueprint-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AutoGeneratedBundle {
    pub blueprint_bundle_reference: BlueprintBundleReference,
}

pub const AUTOGENERATEDBUNDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoGeneratedBundle",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "BlueprintBundleReference",
                flags: MemberInfoFlags::new(0),
                field_type: BLUEPRINTBUNDLEREFERENCE_TYPE_INFO,
                rust_offset: offset_of!(AutoGeneratedBundle, blueprint_bundle_reference),
            },
        ],
    }),
    array_type: Some(AUTOGENERATEDBUNDLE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AutoGeneratedBundle {
    fn type_info() -> &'static TypeInfo {
        AUTOGENERATEDBUNDLE_TYPE_INFO
    }
}


pub const AUTOGENERATEDBUNDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoGeneratedBundle-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("AutoGeneratedBundle-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BlueprintBundleCollection {
    pub bundles: Vec<BlueprintBundleReference>,
}

pub const BLUEPRINTBUNDLECOLLECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundleCollection",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Bundles",
                flags: MemberInfoFlags::new(144),
                field_type: BLUEPRINTBUNDLEREFERENCE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(BlueprintBundleCollection, bundles),
            },
        ],
    }),
    array_type: Some(BLUEPRINTBUNDLECOLLECTION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BlueprintBundleCollection {
    fn type_info() -> &'static TypeInfo {
        BLUEPRINTBUNDLECOLLECTION_TYPE_INFO
    }
}


pub const BLUEPRINTBUNDLECOLLECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundleCollection-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("BlueprintBundleCollection-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BlueprintBundleReference {
    pub name: String,
    pub settings: BlueprintBundleSettings,
    pub parents: Vec<super::entity::SharedBundleReference>,
    pub contains_controllable: bool,
}

pub const BLUEPRINTBUNDLEREFERENCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundleReference",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(BlueprintBundleReference, name),
            },
            FieldInfoData {
                name: "Settings",
                flags: MemberInfoFlags::new(0),
                field_type: BLUEPRINTBUNDLESETTINGS_TYPE_INFO,
                rust_offset: offset_of!(BlueprintBundleReference, settings),
            },
            FieldInfoData {
                name: "Parents",
                flags: MemberInfoFlags::new(144),
                field_type: SHAREDBUNDLEREFERENCE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(BlueprintBundleReference, parents),
            },
            FieldInfoData {
                name: "ContainsControllable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BlueprintBundleReference, contains_controllable),
            },
        ],
    }),
    array_type: Some(BLUEPRINTBUNDLEREFERENCE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BlueprintBundleReference {
    fn type_info() -> &'static TypeInfo {
        BLUEPRINTBUNDLEREFERENCE_TYPE_INFO
    }
}


pub const BLUEPRINTBUNDLEREFERENCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundleReference-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("BlueprintBundleReference-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DynamicBlueprintBundleAsset {
    pub blueprint_bundle: BlueprintBundle,
}

pub const DYNAMICBLUEPRINTBUNDLEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicBlueprintBundleAsset",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DYNAMICBUNDLEASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "BlueprintBundle",
                flags: MemberInfoFlags::new(0),
                field_type: BLUEPRINTBUNDLE_TYPE_INFO,
                rust_offset: offset_of!(DynamicBlueprintBundleAsset, blueprint_bundle),
            },
        ],
    }),
    array_type: Some(DYNAMICBLUEPRINTBUNDLEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DynamicBlueprintBundleAsset {
    fn type_info() -> &'static TypeInfo {
        DYNAMICBLUEPRINTBUNDLEASSET_TYPE_INFO
    }
}


pub const DYNAMICBLUEPRINTBUNDLEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicBlueprintBundleAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("DynamicBlueprintBundleAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DynamicBundleAsset {
}

pub const DYNAMICBUNDLEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicBundleAsset",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINERPOLICYASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DYNAMICBUNDLEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DynamicBundleAsset {
    fn type_info() -> &'static TypeInfo {
        DYNAMICBUNDLEASSET_TYPE_INFO
    }
}


pub const DYNAMICBUNDLEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicBundleAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("DynamicBundleAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BlueprintBundle {
    pub blueprint: super::entity::Blueprint,
}

pub const BLUEPRINTBUNDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundle",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SHAREDBUNDLEBASEASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Blueprint",
                flags: MemberInfoFlags::new(0),
                field_type: BLUEPRINT_TYPE_INFO,
                rust_offset: offset_of!(BlueprintBundle, blueprint),
            },
        ],
    }),
    array_type: Some(BLUEPRINTBUNDLE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BlueprintBundle {
    fn type_info() -> &'static TypeInfo {
        BLUEPRINTBUNDLE_TYPE_INFO
    }
}


pub const BLUEPRINTBUNDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundle-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("BlueprintBundle-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BlueprintBundleSettings {
    pub heap: super::entity::BundleHeapInfo,
    pub bundle_settings_info: super::entity::BundleSettingsInfo,
    pub dynamic_bundle: bool,
}

pub const BLUEPRINTBUNDLESETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundleSettings",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Heap",
                flags: MemberInfoFlags::new(0),
                field_type: BUNDLEHEAPINFO_TYPE_INFO,
                rust_offset: offset_of!(BlueprintBundleSettings, heap),
            },
            FieldInfoData {
                name: "BundleSettingsInfo",
                flags: MemberInfoFlags::new(0),
                field_type: BUNDLESETTINGSINFO_TYPE_INFO,
                rust_offset: offset_of!(BlueprintBundleSettings, bundle_settings_info),
            },
            FieldInfoData {
                name: "DynamicBundle",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BlueprintBundleSettings, dynamic_bundle),
            },
        ],
    }),
    array_type: Some(BLUEPRINTBUNDLESETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BlueprintBundleSettings {
    fn type_info() -> &'static TypeInfo {
        BLUEPRINTBUNDLESETTINGS_TYPE_INFO
    }
}


pub const BLUEPRINTBUNDLESETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundleSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("BlueprintBundleSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AntAnimatableEntityData {
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

pub const ANTANIMATABLEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntAnimatableEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(AntAnimatableEntityData, realm),
            },
            FieldInfoData {
                name: "SubRealm",
                flags: MemberInfoFlags::new(0),
                field_type: SUBREALM_TYPE_INFO,
                rust_offset: offset_of!(AntAnimatableEntityData, sub_realm),
            },
            FieldInfoData {
                name: "AnimationData",
                flags: MemberInfoFlags::new(0),
                field_type: ANTANIMATIONHANDLERDATA_TYPE_INFO,
                rust_offset: offset_of!(AntAnimatableEntityData, animation_data),
            },
            FieldInfoData {
                name: "AutoActivate",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AntAnimatableEntityData, auto_activate),
            },
            FieldInfoData {
                name: "InitialAnimationControlMode",
                flags: MemberInfoFlags::new(0),
                field_type: ANIMATIONCONTROLMODEENUM_TYPE_INFO,
                rust_offset: offset_of!(AntAnimatableEntityData, initial_animation_control_mode),
            },
            FieldInfoData {
                name: "InitialForceDisableCulling",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AntAnimatableEntityData, initial_force_disable_culling),
            },
            FieldInfoData {
                name: "DisableAutoDistanceCulling",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AntAnimatableEntityData, disable_auto_distance_culling),
            },
            FieldInfoData {
                name: "Interpolation",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AntAnimatableEntityData, interpolation),
            },
            FieldInfoData {
                name: "JointOutputMode",
                flags: MemberInfoFlags::new(0),
                field_type: JOINTOUTPUTMODEENUM_TYPE_INFO,
                rust_offset: offset_of!(AntAnimatableEntityData, joint_output_mode),
            },
            FieldInfoData {
                name: "JointOutputPropertyIds",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(AntAnimatableEntityData, joint_output_property_ids),
            },
            FieldInfoData {
                name: "UseSimpleSkeletonCompression",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AntAnimatableEntityData, use_simple_skeleton_compression),
            },
        ],
    }),
    array_type: Some(ANTANIMATABLEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AntAnimatableEntityData {
    fn type_info() -> &'static TypeInfo {
        ANTANIMATABLEENTITYDATA_TYPE_INFO
    }
}


pub const ANTANIMATABLEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntAnimatableEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("AntAnimatableEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AntAnimatableComponentMeshData {
    pub mesh_render_type: AntAnimatableComponentMeshRenderType,
    pub mesh_render_context: AntAnimatableComponentMeshRenderContext,
    pub mesh: super::render_base::MeshBaseAsset,
    pub camera_relative_scale_x: f32,
    pub camera_relative_scale_y: f32,
    pub camera_relative_scale_z: f32,
    pub camera_relative_offset_x: f32,
    pub camera_relative_offset_y: f32,
    pub camera_relative_offset_z: f32,
}

pub const ANTANIMATABLECOMPONENTMESHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntAnimatableComponentMeshData",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "MeshRenderType",
                flags: MemberInfoFlags::new(0),
                field_type: ANTANIMATABLECOMPONENTMESHRENDERTYPE_TYPE_INFO,
                rust_offset: offset_of!(AntAnimatableComponentMeshData, mesh_render_type),
            },
            FieldInfoData {
                name: "MeshRenderContext",
                flags: MemberInfoFlags::new(0),
                field_type: ANTANIMATABLECOMPONENTMESHRENDERCONTEXT_TYPE_INFO,
                rust_offset: offset_of!(AntAnimatableComponentMeshData, mesh_render_context),
            },
            FieldInfoData {
                name: "Mesh",
                flags: MemberInfoFlags::new(0),
                field_type: MESHBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(AntAnimatableComponentMeshData, mesh),
            },
            FieldInfoData {
                name: "CameraRelativeScaleX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AntAnimatableComponentMeshData, camera_relative_scale_x),
            },
            FieldInfoData {
                name: "CameraRelativeScaleY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AntAnimatableComponentMeshData, camera_relative_scale_y),
            },
            FieldInfoData {
                name: "CameraRelativeScaleZ",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AntAnimatableComponentMeshData, camera_relative_scale_z),
            },
            FieldInfoData {
                name: "CameraRelativeOffsetX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AntAnimatableComponentMeshData, camera_relative_offset_x),
            },
            FieldInfoData {
                name: "CameraRelativeOffsetY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AntAnimatableComponentMeshData, camera_relative_offset_y),
            },
            FieldInfoData {
                name: "CameraRelativeOffsetZ",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AntAnimatableComponentMeshData, camera_relative_offset_z),
            },
        ],
    }),
    array_type: Some(ANTANIMATABLECOMPONENTMESHDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AntAnimatableComponentMeshData {
    fn type_info() -> &'static TypeInfo {
        ANTANIMATABLECOMPONENTMESHDATA_TYPE_INFO
    }
}


pub const ANTANIMATABLECOMPONENTMESHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntAnimatableComponentMeshData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("AntAnimatableComponentMeshData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum AnimationControlModeEnum {
    #[default]
    AnimationControlModeEnum_PlaceTrajectoryAtObject = 0,
    AnimationControlModeEnum_PlaceObjectAtTrajectory = 1,
    AnimationControlModeEnum_DisconnectTrajectoryAndObject = 2,
}

pub const ANIMATIONCONTROLMODEENUM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimationControlModeEnum",
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(ANIMATIONCONTROLMODEENUM_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AnimationControlModeEnum {
    fn type_info() -> &'static TypeInfo {
        ANIMATIONCONTROLMODEENUM_TYPE_INFO
    }
}


pub const ANIMATIONCONTROLMODEENUM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimationControlModeEnum-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("AnimationControlModeEnum-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum JointOutputModeEnum {
    #[default]
    JointOutputModeEnum_NoOutput = 0,
    JointOutputModeEnum_AllJoints = 1,
    JointOutputModeEnum_GameplayBones = 2,
    JointOutputModeEnum_NoOutputUpdateCulled = 3,
    JointOutputModeEnum_GameplayBonesUpdateCulled = 4,
}

pub const JOINTOUTPUTMODEENUM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "JointOutputModeEnum",
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(JOINTOUTPUTMODEENUM_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for JointOutputModeEnum {
    fn type_info() -> &'static TypeInfo {
        JOINTOUTPUTMODEENUM_TYPE_INFO
    }
}


pub const JOINTOUTPUTMODEENUM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "JointOutputModeEnum-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("JointOutputModeEnum-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum AntAnimatableComponentMeshRenderContext {
    #[default]
    AntAnimatableComponentMeshRenderContext_World = 0,
    AntAnimatableComponentMeshRenderContext_Foreground = 1,
    AntAnimatableComponentMeshRenderContext_CameraRelative = 2,
}

pub const ANTANIMATABLECOMPONENTMESHRENDERCONTEXT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntAnimatableComponentMeshRenderContext",
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(ANTANIMATABLECOMPONENTMESHRENDERCONTEXT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AntAnimatableComponentMeshRenderContext {
    fn type_info() -> &'static TypeInfo {
        ANTANIMATABLECOMPONENTMESHRENDERCONTEXT_TYPE_INFO
    }
}


pub const ANTANIMATABLECOMPONENTMESHRENDERCONTEXT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntAnimatableComponentMeshRenderContext-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("AntAnimatableComponentMeshRenderContext-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum AntAnimatableComponentMeshRenderType {
    #[default]
    AntAnimatableComponentMeshRenderType_Default = 0,
    AntAnimatableComponentMeshRenderType_ShadowOnly = 1,
    AntAnimatableComponentMeshRenderType_NoShadow = 2,
}

pub const ANTANIMATABLECOMPONENTMESHRENDERTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntAnimatableComponentMeshRenderType",
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(ANTANIMATABLECOMPONENTMESHRENDERTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AntAnimatableComponentMeshRenderType {
    fn type_info() -> &'static TypeInfo {
        ANTANIMATABLECOMPONENTMESHRENDERTYPE_TYPE_INFO
    }
}


pub const ANTANIMATABLECOMPONENTMESHRENDERTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntAnimatableComponentMeshRenderType-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("AntAnimatableComponentMeshRenderType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
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

pub const ANTANIMATIONHANDLERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntAnimationHandlerData",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Animatable",
                flags: MemberInfoFlags::new(0),
                field_type: ANTANIMATABLEDATA_TYPE_INFO,
                rust_offset: offset_of!(AntAnimationHandlerData, animatable),
            },
            FieldInfoData {
                name: "RootController",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(AntAnimationHandlerData, root_controller),
            },
            FieldInfoData {
                name: "LodBinding",
                flags: MemberInfoFlags::new(0),
                field_type: LODBINDING_TYPE_INFO,
                rust_offset: offset_of!(AntAnimationHandlerData, lod_binding),
            },
            FieldInfoData {
                name: "AnimationControlBinding",
                flags: MemberInfoFlags::new(0),
                field_type: ANIMATIONCONTROLBINDING_TYPE_INFO,
                rust_offset: offset_of!(AntAnimationHandlerData, animation_control_binding),
            },
            FieldInfoData {
                name: "LodDistanceScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AntAnimationHandlerData, lod_distance_scale),
            },
            FieldInfoData {
                name: "ReportBackFromAnt",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AntAnimationHandlerData, report_back_from_ant),
            },
            FieldInfoData {
                name: "EnableMasterSlaveCopy",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AntAnimationHandlerData, enable_master_slave_copy),
            },
            FieldInfoData {
                name: "ForceServerAsSlave",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AntAnimationHandlerData, force_server_as_slave),
            },
            FieldInfoData {
                name: "IsProp",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AntAnimationHandlerData, is_prop),
            },
            FieldInfoData {
                name: "JointOutputFieldHashes",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(AntAnimationHandlerData, joint_output_field_hashes),
            },
            FieldInfoData {
                name: "JointEnabledFieldHashes",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(AntAnimationHandlerData, joint_enabled_field_hashes),
            },
        ],
    }),
    array_type: Some(ANTANIMATIONHANDLERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AntAnimationHandlerData {
    fn type_info() -> &'static TypeInfo {
        ANTANIMATIONHANDLERDATA_TYPE_INFO
    }
}


pub const ANTANIMATIONHANDLERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntAnimationHandlerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("AntAnimationHandlerData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AnimationControlBinding {
    pub animation_velocity_to_physics: super::ant::AntRef,
}

pub const ANIMATIONCONTROLBINDING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimationControlBinding",
    flags: MemberInfoFlags::new(32841),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "AnimationVelocityToPhysics",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(AnimationControlBinding, animation_velocity_to_physics),
            },
        ],
    }),
    array_type: Some(ANIMATIONCONTROLBINDING_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for AnimationControlBinding {
    fn type_info() -> &'static TypeInfo {
        ANIMATIONCONTROLBINDING_TYPE_INFO
    }
}


pub const ANIMATIONCONTROLBINDING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimationControlBinding-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("AnimationControlBinding-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LodBinding {
    pub distance_from_camera: super::ant::AntRef,
    pub pixel_size: super::ant::AntRef,
    pub had_visual_update: super::ant::AntRef,
    pub reset_controller: super::ant::AntRef,
}

pub const LODBINDING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LodBinding",
    flags: MemberInfoFlags::new(32841),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "DistanceFromCamera",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(LodBinding, distance_from_camera),
            },
            FieldInfoData {
                name: "PixelSize",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(LodBinding, pixel_size),
            },
            FieldInfoData {
                name: "HadVisualUpdate",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(LodBinding, had_visual_update),
            },
            FieldInfoData {
                name: "ResetController",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(LodBinding, reset_controller),
            },
        ],
    }),
    array_type: Some(LODBINDING_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for LodBinding {
    fn type_info() -> &'static TypeInfo {
        LODBINDING_TYPE_INFO
    }
}


pub const LODBINDING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LodBinding-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("LodBinding-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AntEnumeration {
    pub ant_asset: super::ant::AntRef,
    pub value: i32,
}

pub const ANTENUMERATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntEnumeration",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "AntAsset",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(AntEnumeration, ant_asset),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AntEnumeration, value),
            },
        ],
    }),
    array_type: Some(ANTENUMERATION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AntEnumeration {
    fn type_info() -> &'static TypeInfo {
        ANTENUMERATION_TYPE_INFO
    }
}


pub const ANTENUMERATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntEnumeration-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("AntEnumeration-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GameAIWeaponData {
}

pub const GAMEAIWEAPONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameAIWeaponData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(GAMEAIWEAPONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GameAIWeaponData {
    fn type_info() -> &'static TypeInfo {
        GAMEAIWEAPONDATA_TYPE_INFO
    }
}


pub const GAMEAIWEAPONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameAIWeaponData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("GameAIWeaponData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GameAIEntryData {
    pub ai2_data: GameAIEntryData,
}

pub const GAMEAIENTRYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameAIEntryData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Ai2Data",
                flags: MemberInfoFlags::new(0),
                field_type: GAMEAIENTRYDATA_TYPE_INFO,
                rust_offset: offset_of!(GameAIEntryData, ai2_data),
            },
        ],
    }),
    array_type: Some(GAMEAIENTRYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GameAIEntryData {
    fn type_info() -> &'static TypeInfo {
        GAMEAIENTRYDATA_TYPE_INFO
    }
}


pub const GAMEAIENTRYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameAIEntryData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("GameAIEntryData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PoseConstraintsData {
    pub stand_pose: bool,
    pub crouch_pose: bool,
    pub prone_pose: bool,
}

pub const POSECONSTRAINTSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PoseConstraintsData",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "StandPose",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PoseConstraintsData, stand_pose),
            },
            FieldInfoData {
                name: "CrouchPose",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PoseConstraintsData, crouch_pose),
            },
            FieldInfoData {
                name: "PronePose",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PoseConstraintsData, prone_pose),
            },
        ],
    }),
    array_type: Some(POSECONSTRAINTSDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PoseConstraintsData {
    fn type_info() -> &'static TypeInfo {
        POSECONSTRAINTSDATA_TYPE_INFO
    }
}


pub const POSECONSTRAINTSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PoseConstraintsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("PoseConstraintsData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AimingConstraintsData {
    pub min_yaw: f32,
    pub max_yaw: f32,
    pub min_pitch: f32,
    pub max_pitch: f32,
}

pub const AIMINGCONSTRAINTSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingConstraintsData",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "MinYaw",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingConstraintsData, min_yaw),
            },
            FieldInfoData {
                name: "MaxYaw",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingConstraintsData, max_yaw),
            },
            FieldInfoData {
                name: "MinPitch",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingConstraintsData, min_pitch),
            },
            FieldInfoData {
                name: "MaxPitch",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingConstraintsData, max_pitch),
            },
        ],
    }),
    array_type: Some(AIMINGCONSTRAINTSDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for AimingConstraintsData {
    fn type_info() -> &'static TypeInfo {
        AIMINGCONSTRAINTSDATA_TYPE_INFO
    }
}


pub const AIMINGCONSTRAINTSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingConstraintsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("AimingConstraintsData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TextureAssetEntityData {
    pub realm: super::core::Realm,
    pub default_texture_asset: super::render_base::TextureBaseAsset,
}

pub const TEXTUREASSETENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureAssetEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(TextureAssetEntityData, realm),
            },
            FieldInfoData {
                name: "DefaultTextureAsset",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(TextureAssetEntityData, default_texture_asset),
            },
        ],
    }),
    array_type: Some(TEXTUREASSETENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TextureAssetEntityData {
    fn type_info() -> &'static TypeInfo {
        TEXTUREASSETENTITYDATA_TYPE_INFO
    }
}


pub const TEXTUREASSETENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureAssetEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("TextureAssetEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TerrainShaderParameterEntityData {
    pub shader_parameters: Vec<TerrainShaderParameter>,
}

pub const TERRAINSHADERPARAMETERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainShaderParameterEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ShaderParameters",
                flags: MemberInfoFlags::new(144),
                field_type: TERRAINSHADERPARAMETER_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(TerrainShaderParameterEntityData, shader_parameters),
            },
        ],
    }),
    array_type: Some(TERRAINSHADERPARAMETERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TerrainShaderParameterEntityData {
    fn type_info() -> &'static TypeInfo {
        TERRAINSHADERPARAMETERENTITYDATA_TYPE_INFO
    }
}


pub const TERRAINSHADERPARAMETERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainShaderParameterEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("TerrainShaderParameterEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TerrainShaderParameter {
    pub parameter_desc: super::render_base::ShaderParameterDesc,
    pub parameter_type: TerrainShaderParameterDataType,
    pub parameter_name: String,
}

pub const TERRAINSHADERPARAMETER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainShaderParameter",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ParameterDesc",
                flags: MemberInfoFlags::new(0),
                field_type: SHADERPARAMETERDESC_TYPE_INFO,
                rust_offset: offset_of!(TerrainShaderParameter, parameter_desc),
            },
            FieldInfoData {
                name: "ParameterType",
                flags: MemberInfoFlags::new(0),
                field_type: TERRAINSHADERPARAMETERDATATYPE_TYPE_INFO,
                rust_offset: offset_of!(TerrainShaderParameter, parameter_type),
            },
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TerrainShaderParameter, parameter_name),
            },
        ],
    }),
    array_type: Some(TERRAINSHADERPARAMETER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TerrainShaderParameter {
    fn type_info() -> &'static TypeInfo {
        TERRAINSHADERPARAMETER_TYPE_INFO
    }
}


pub const TERRAINSHADERPARAMETER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainShaderParameter-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("TerrainShaderParameter-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum TerrainShaderParameterDataType {
    #[default]
    TerrainShaderParameterDataType_Bool = 0,
    TerrainShaderParameterDataType_Scalar = 1,
    TerrainShaderParameterDataType_Vector2 = 2,
    TerrainShaderParameterDataType_Vector3 = 3,
    TerrainShaderParameterDataType_Vector4 = 4,
}

pub const TERRAINSHADERPARAMETERDATATYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainShaderParameterDataType",
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(TERRAINSHADERPARAMETERDATATYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TerrainShaderParameterDataType {
    fn type_info() -> &'static TypeInfo {
        TERRAINSHADERPARAMETERDATATYPE_TYPE_INFO
    }
}


pub const TERRAINSHADERPARAMETERDATATYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainShaderParameterDataType-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("TerrainShaderParameterDataType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ShaderParameterEntityData {
    pub parameter_desc: super::render_base::ShaderParameterDesc,
    pub parameter_type: ShaderParameterDataType,
    pub vec_param: super::core::Vec4,
    pub bool_param: bool,
    pub texture_param: super::render_base::TextureBaseAsset,
    pub conditional_param: u32,
}

pub const SHADERPARAMETERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ParameterDesc",
                flags: MemberInfoFlags::new(0),
                field_type: SHADERPARAMETERDESC_TYPE_INFO,
                rust_offset: offset_of!(ShaderParameterEntityData, parameter_desc),
            },
            FieldInfoData {
                name: "ParameterType",
                flags: MemberInfoFlags::new(0),
                field_type: SHADERPARAMETERDATATYPE_TYPE_INFO,
                rust_offset: offset_of!(ShaderParameterEntityData, parameter_type),
            },
            FieldInfoData {
                name: "VecParam",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(ShaderParameterEntityData, vec_param),
            },
            FieldInfoData {
                name: "BoolParam",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ShaderParameterEntityData, bool_param),
            },
            FieldInfoData {
                name: "TextureParam",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(ShaderParameterEntityData, texture_param),
            },
            FieldInfoData {
                name: "ConditionalParam",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ShaderParameterEntityData, conditional_param),
            },
        ],
    }),
    array_type: Some(SHADERPARAMETERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ShaderParameterEntityData {
    fn type_info() -> &'static TypeInfo {
        SHADERPARAMETERENTITYDATA_TYPE_INFO
    }
}


pub const SHADERPARAMETERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ShaderParameterEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ShaderParameterDataType {
    #[default]
    ShaderParameterDataType_Vector = 0,
    ShaderParameterDataType_Bool = 1,
    ShaderParameterDataType_Texture = 2,
    ShaderParameterDataType_Conditional = 3,
}

pub const SHADERPARAMETERDATATYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterDataType",
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(SHADERPARAMETERDATATYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShaderParameterDataType {
    fn type_info() -> &'static TypeInfo {
        SHADERPARAMETERDATATYPE_TYPE_INFO
    }
}


pub const SHADERPARAMETERDATATYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterDataType-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ShaderParameterDataType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ANTLayerBlendType {
    #[default]
    ANTLayerBlendType_OverrideBlend = 0,
    ANTLayerBlendType_AdditiveBlend = 1,
    ANTLayerBlendType_SubtractiveBlend = 2,
}

pub const ANTLAYERBLENDTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTLayerBlendType",
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(ANTLAYERBLENDTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ANTLayerBlendType {
    fn type_info() -> &'static TypeInfo {
        ANTLAYERBLENDTYPE_TYPE_INFO
    }
}


pub const ANTLAYERBLENDTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTLayerBlendType-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ANTLayerBlendType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
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

pub const ANTCLIPENDRULE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTClipEndRule",
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(ANTCLIPENDRULE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ANTClipEndRule {
    fn type_info() -> &'static TypeInfo {
        ANTCLIPENDRULE_TYPE_INFO
    }
}


pub const ANTCLIPENDRULE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTClipEndRule-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ANTClipEndRule-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ANTClipStartRule {
    #[default]
    ANTClipStartRule_None = 0,
    ANTClipStartRule_HoldStartFrameFromStartOfTimeline = 1,
}

pub const ANTCLIPSTARTRULE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTClipStartRule",
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(ANTCLIPSTARTRULE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ANTClipStartRule {
    fn type_info() -> &'static TypeInfo {
        ANTCLIPSTARTRULE_TYPE_INFO
    }
}


pub const ANTCLIPSTARTRULE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTClipStartRule-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ANTClipStartRule-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SubViewData {
}

pub const SUBVIEWDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubViewData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SUBVIEWDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SubViewData {
    fn type_info() -> &'static TypeInfo {
        SUBVIEWDATA_TYPE_INFO
    }
}


pub const SUBVIEWDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubViewData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("SubViewData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PlayerViewData {
    pub sub_views: Vec<SubViewData>,
}

pub const PLAYERVIEWDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayerViewData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "SubViews",
                flags: MemberInfoFlags::new(144),
                field_type: SUBVIEWDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(PlayerViewData, sub_views),
            },
        ],
    }),
    array_type: Some(PLAYERVIEWDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PlayerViewData {
    fn type_info() -> &'static TypeInfo {
        PLAYERVIEWDATA_TYPE_INFO
    }
}


pub const PLAYERVIEWDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayerViewData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("PlayerViewData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PlayerData {
    pub player_view: PlayerViewData,
}

pub const PLAYERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayerData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "PlayerView",
                flags: MemberInfoFlags::new(0),
                field_type: PLAYERVIEWDATA_TYPE_INFO,
                rust_offset: offset_of!(PlayerData, player_view),
            },
        ],
    }),
    array_type: Some(PLAYERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PlayerData {
    fn type_info() -> &'static TypeInfo {
        PLAYERDATA_TYPE_INFO
    }
}


pub const PLAYERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("PlayerData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkedOnlineId {
    pub native_data: u64,
}

pub const NETWORKEDONLINEID_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkedOnlineId",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "NativeData",
                flags: MemberInfoFlags::new(0),
                field_type: UINT64_TYPE_INFO,
                rust_offset: offset_of!(NetworkedOnlineId, native_data),
            },
        ],
    }),
    array_type: Some(NETWORKEDONLINEID_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NetworkedOnlineId {
    fn type_info() -> &'static TypeInfo {
        NETWORKEDONLINEID_TYPE_INFO
    }
}


pub const NETWORKEDONLINEID_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkedOnlineId-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("NetworkedOnlineId-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum OnlineIdConstants {
    #[default]
    OnlineIdConstants_IdLength = 17,
}

pub const ONLINEIDCONSTANTS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineIdConstants",
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(ONLINEIDCONSTANTS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for OnlineIdConstants {
    fn type_info() -> &'static TypeInfo {
        ONLINEIDCONSTANTS_TYPE_INFO
    }
}


pub const ONLINEIDCONSTANTS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineIdConstants-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("OnlineIdConstants-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UINetworkPlayerDisconnectMessage {
}

pub const UINETWORKPLAYERDISCONNECTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UINetworkPlayerDisconnectMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UINetworkPlayerDisconnectMessage {
    fn type_info() -> &'static TypeInfo {
        UINETWORKPLAYERDISCONNECTMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UINetworkPlayerConnectMessage {
}

pub const UINETWORKPLAYERCONNECTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UINetworkPlayerConnectMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UINetworkPlayerConnectMessage {
    fn type_info() -> &'static TypeInfo {
        UINETWORKPLAYERCONNECTMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct NetworkSettings {
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

pub const NETWORKSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkSettings",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ProtocolVersion",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(NetworkSettings, protocol_version),
            },
            FieldInfoData {
                name: "TitleId",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(NetworkSettings, title_id),
            },
            FieldInfoData {
                name: "ClientPort",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(NetworkSettings, client_port),
            },
            FieldInfoData {
                name: "ServerPort",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(NetworkSettings, server_port),
            },
            FieldInfoData {
                name: "MaxGhostCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(NetworkSettings, max_ghost_count),
            },
            FieldInfoData {
                name: "MaxClientToServerGhostCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(NetworkSettings, max_client_to_server_ghost_count),
            },
            FieldInfoData {
                name: "MaxClientCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(NetworkSettings, max_client_count),
            },
            FieldInfoData {
                name: "MaxClientFrameSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(NetworkSettings, max_client_frame_size),
            },
            FieldInfoData {
                name: "MaxServerFrameSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(NetworkSettings, max_server_frame_size),
            },
            FieldInfoData {
                name: "MaxNumVoipPeers",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(NetworkSettings, max_num_voip_peers),
            },
            FieldInfoData {
                name: "ServerAddress",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(NetworkSettings, server_address),
            },
            FieldInfoData {
                name: "ClientConnectionDebugFilePrefix",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(NetworkSettings, client_connection_debug_file_prefix),
            },
            FieldInfoData {
                name: "ServerConnectionDebugFilePrefix",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(NetworkSettings, server_connection_debug_file_prefix),
            },
            FieldInfoData {
                name: "SinglePlayerTimeNudgeBias",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(NetworkSettings, single_player_time_nudge_bias),
            },
            FieldInfoData {
                name: "SinglePlayerTimeNudge",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(NetworkSettings, single_player_time_nudge),
            },
            FieldInfoData {
                name: "SinglePlayerAutomaticTimeNudge",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(NetworkSettings, single_player_automatic_time_nudge),
            },
            FieldInfoData {
                name: "MemorySocketTimeNudgeBias",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(NetworkSettings, memory_socket_time_nudge_bias),
            },
            FieldInfoData {
                name: "MemorySocketTimeNudge",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(NetworkSettings, memory_socket_time_nudge),
            },
            FieldInfoData {
                name: "MemorySocketAutomaticTimeNudge",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(NetworkSettings, memory_socket_automatic_time_nudge),
            },
            FieldInfoData {
                name: "LocalHostTimeNudgeBias",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(NetworkSettings, local_host_time_nudge_bias),
            },
            FieldInfoData {
                name: "LocalHostTimeNudge",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(NetworkSettings, local_host_time_nudge),
            },
            FieldInfoData {
                name: "LocalHostAutomaticTimeNudge",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(NetworkSettings, local_host_automatic_time_nudge),
            },
            FieldInfoData {
                name: "DefaultTimeNudgeBias",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(NetworkSettings, default_time_nudge_bias),
            },
            FieldInfoData {
                name: "DefaultTimeNudge",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(NetworkSettings, default_time_nudge),
            },
            FieldInfoData {
                name: "DefaultAutomaticTimeNudge",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(NetworkSettings, default_automatic_time_nudge),
            },
            FieldInfoData {
                name: "IncrementServerPortOnFail",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(NetworkSettings, increment_server_port_on_fail),
            },
            FieldInfoData {
                name: "UseFrameManager",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(NetworkSettings, use_frame_manager),
            },
            FieldInfoData {
                name: "TimeSyncEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(NetworkSettings, time_sync_enabled),
            },
            FieldInfoData {
                name: "ConnectTimeout",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(NetworkSettings, connect_timeout),
            },
            FieldInfoData {
                name: "PacketLossLogInterval",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(NetworkSettings, packet_loss_log_interval),
            },
            FieldInfoData {
                name: "ValidLocalPlayersMask",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(NetworkSettings, valid_local_players_mask),
            },
            FieldInfoData {
                name: "DesiredLocalPlayersMask",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(NetworkSettings, desired_local_players_mask),
            },
            FieldInfoData {
                name: "PersistentLocalPlayersMask",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(NetworkSettings, persistent_local_players_mask),
            },
            FieldInfoData {
                name: "MLUREnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(NetworkSettings, m_l_u_r_enabled),
            },
            FieldInfoData {
                name: "SinglePlayerMaxMessagesPerNetworkFrame",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(NetworkSettings, single_player_max_messages_per_network_frame),
            },
            FieldInfoData {
                name: "MaxMessagesPerNetworkFrame",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(NetworkSettings, max_messages_per_network_frame),
            },
        ],
    }),
    array_type: Some(NETWORKSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NetworkSettings {
    fn type_info() -> &'static TypeInfo {
        NETWORKSETTINGS_TYPE_INFO
    }
}


pub const NETWORKSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("NetworkSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VersionData {
    pub disclaimer: String,
    pub version: i32,
    pub date_time: String,
    pub branch_id: String,
    pub data_branch_id: String,
    pub game_name: String,
}

pub const VERSIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VersionData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "disclaimer",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(VersionData, disclaimer),
            },
            FieldInfoData {
                name: "Version",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(VersionData, version),
            },
            FieldInfoData {
                name: "DateTime",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(VersionData, date_time),
            },
            FieldInfoData {
                name: "BranchId",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(VersionData, branch_id),
            },
            FieldInfoData {
                name: "DataBranchId",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(VersionData, data_branch_id),
            },
            FieldInfoData {
                name: "GameName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(VersionData, game_name),
            },
        ],
    }),
    array_type: Some(VERSIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VersionData {
    fn type_info() -> &'static TypeInfo {
        VERSIONDATA_TYPE_INFO
    }
}


pub const VERSIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VersionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("VersionData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UISettings {
    pub system: super::u_i::UISystemType,
    pub profile_options: ProfileOptionsAsset,
    pub language: super::core::LanguageFormat,
    pub draw_enable: bool,
    pub enable_jobs: bool,
    pub scaleform_safe_mode: bool,
    pub localization: super::u_i::LocalizationAsset,
    pub font_configuration: UIFontConfigurationAssetBase,
    pub allow_slow_texture_loading_path: bool,
    pub allow_layer_name_access: bool,
}

pub const UISETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UISettings",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "System",
                flags: MemberInfoFlags::new(0),
                field_type: UISYSTEMTYPE_TYPE_INFO,
                rust_offset: offset_of!(UISettings, system),
            },
            FieldInfoData {
                name: "ProfileOptions",
                flags: MemberInfoFlags::new(0),
                field_type: PROFILEOPTIONSASSET_TYPE_INFO,
                rust_offset: offset_of!(UISettings, profile_options),
            },
            FieldInfoData {
                name: "Language",
                flags: MemberInfoFlags::new(0),
                field_type: LANGUAGEFORMAT_TYPE_INFO,
                rust_offset: offset_of!(UISettings, language),
            },
            FieldInfoData {
                name: "DrawEnable",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UISettings, draw_enable),
            },
            FieldInfoData {
                name: "EnableJobs",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UISettings, enable_jobs),
            },
            FieldInfoData {
                name: "ScaleformSafeMode",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UISettings, scaleform_safe_mode),
            },
            FieldInfoData {
                name: "Localization",
                flags: MemberInfoFlags::new(0),
                field_type: LOCALIZATIONASSET_TYPE_INFO,
                rust_offset: offset_of!(UISettings, localization),
            },
            FieldInfoData {
                name: "FontConfiguration",
                flags: MemberInfoFlags::new(0),
                field_type: UIFONTCONFIGURATIONASSETBASE_TYPE_INFO,
                rust_offset: offset_of!(UISettings, font_configuration),
            },
            FieldInfoData {
                name: "AllowSlowTextureLoadingPath",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UISettings, allow_slow_texture_loading_path),
            },
            FieldInfoData {
                name: "AllowLayerNameAccess",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UISettings, allow_layer_name_access),
            },
        ],
    }),
    array_type: Some(UISETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UISettings {
    fn type_info() -> &'static TypeInfo {
        UISETTINGS_TYPE_INFO
    }
}


pub const UISETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UISettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("UISettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIFontConfigurationAssetBase {
}

pub const UIFONTCONFIGURATIONASSETBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIFontConfigurationAssetBase",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(UIFONTCONFIGURATIONASSETBASE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIFontConfigurationAssetBase {
    fn type_info() -> &'static TypeInfo {
        UIFONTCONFIGURATIONASSETBASE_TYPE_INFO
    }
}


pub const UIFONTCONFIGURATIONASSETBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIFontConfigurationAssetBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("UIFontConfigurationAssetBase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WaterHealthComponentData {
}

pub const WATERHEALTHCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterHealthComponentData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMEHEALTHCOMPONENTDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WATERHEALTHCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WaterHealthComponentData {
    fn type_info() -> &'static TypeInfo {
        WATERHEALTHCOMPONENTDATA_TYPE_INFO
    }
}


pub const WATERHEALTHCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterHealthComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("WaterHealthComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TerrainHealthComponentData {
}

pub const TERRAINHEALTHCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainHealthComponentData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMEHEALTHCOMPONENTDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TERRAINHEALTHCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TerrainHealthComponentData {
    fn type_info() -> &'static TypeInfo {
        TERRAINHEALTHCOMPONENTDATA_TYPE_INFO
    }
}


pub const TERRAINHEALTHCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainHealthComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("TerrainHealthComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WaterPhysicsComponentData {
}

pub const WATERPHYSICSCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterPhysicsComponentData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMEPHYSICSCOMPONENTDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WATERPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WaterPhysicsComponentData {
    fn type_info() -> &'static TypeInfo {
        WATERPHYSICSCOMPONENTDATA_TYPE_INFO
    }
}


pub const WATERPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterPhysicsComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("WaterPhysicsComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TerrainPhysicsComponentData {
}

pub const TERRAINPHYSICSCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainPhysicsComponentData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMEPHYSICSCOMPONENTDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TERRAINPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TerrainPhysicsComponentData {
    fn type_info() -> &'static TypeInfo {
        TERRAINPHYSICSCOMPONENTDATA_TYPE_INFO
    }
}


pub const TERRAINPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainPhysicsComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("TerrainPhysicsComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WaterEntityData {
}

pub const WATERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMEPHYSICSENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WATERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WaterEntityData {
    fn type_info() -> &'static TypeInfo {
        WATERENTITYDATA_TYPE_INFO
    }
}


pub const WATERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("WaterEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TerrainEntityData {
    pub terrain_asset: super::terrain_base::TerrainBaseAsset,
    pub water_material: super::entity::MaterialDecl,
    pub visible: bool,
    pub decals_resource: super::core::ResourceRef,
}

pub const TERRAINENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMEPHYSICSENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "TerrainAsset",
                flags: MemberInfoFlags::new(0),
                field_type: TERRAINBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(TerrainEntityData, terrain_asset),
            },
            FieldInfoData {
                name: "WaterMaterial",
                flags: MemberInfoFlags::new(0),
                field_type: MATERIALDECL_TYPE_INFO,
                rust_offset: offset_of!(TerrainEntityData, water_material),
            },
            FieldInfoData {
                name: "Visible",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TerrainEntityData, visible),
            },
            FieldInfoData {
                name: "DecalsResource",
                flags: MemberInfoFlags::new(0),
                field_type: RESOURCEREF_TYPE_INFO,
                rust_offset: offset_of!(TerrainEntityData, decals_resource),
            },
        ],
    }),
    array_type: Some(TERRAINENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TerrainEntityData {
    fn type_info() -> &'static TypeInfo {
        TERRAINENTITYDATA_TYPE_INFO
    }
}


pub const TERRAINENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("TerrainEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProfileOptionDataBinary {
    pub max_length: i32,
}

pub const PROFILEOPTIONDATABINARY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionDataBinary",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROFILEOPTIONDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MaxLength",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ProfileOptionDataBinary, max_length),
            },
        ],
    }),
    array_type: Some(PROFILEOPTIONDATABINARY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProfileOptionDataBinary {
    fn type_info() -> &'static TypeInfo {
        PROFILEOPTIONDATABINARY_TYPE_INFO
    }
}


pub const PROFILEOPTIONDATABINARY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionDataBinary-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ProfileOptionDataBinary-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProfileOptionDataString {
    pub max_length: i32,
    pub value: String,
}

pub const PROFILEOPTIONDATASTRING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionDataString",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROFILEOPTIONDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MaxLength",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ProfileOptionDataString, max_length),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ProfileOptionDataString, value),
            },
        ],
    }),
    array_type: Some(PROFILEOPTIONDATASTRING_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProfileOptionDataString {
    fn type_info() -> &'static TypeInfo {
        PROFILEOPTIONDATASTRING_TYPE_INFO
    }
}


pub const PROFILEOPTIONDATASTRING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionDataString-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ProfileOptionDataString-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProfileOptionDataBool {
    pub value: bool,
}

pub const PROFILEOPTIONDATABOOL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionDataBool",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROFILEOPTIONDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ProfileOptionDataBool, value),
            },
        ],
    }),
    array_type: Some(PROFILEOPTIONDATABOOL_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProfileOptionDataBool {
    fn type_info() -> &'static TypeInfo {
        PROFILEOPTIONDATABOOL_TYPE_INFO
    }
}


pub const PROFILEOPTIONDATABOOL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionDataBool-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ProfileOptionDataBool-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProfileOptionDataEnum {
    pub items: Vec<ProfileOptionDataEnumItem>,
}

pub const PROFILEOPTIONDATAENUM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionDataEnum",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROFILEOPTIONDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Items",
                flags: MemberInfoFlags::new(144),
                field_type: PROFILEOPTIONDATAENUMITEM_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ProfileOptionDataEnum, items),
            },
        ],
    }),
    array_type: Some(PROFILEOPTIONDATAENUM_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProfileOptionDataEnum {
    fn type_info() -> &'static TypeInfo {
        PROFILEOPTIONDATAENUM_TYPE_INFO
    }
}


pub const PROFILEOPTIONDATAENUM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionDataEnum-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ProfileOptionDataEnum-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProfileOptionDataEnumItem {
    pub display_name: String,
    pub default: bool,
}

pub const PROFILEOPTIONDATAENUMITEM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionDataEnumItem",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "DisplayName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ProfileOptionDataEnumItem, display_name),
            },
            FieldInfoData {
                name: "Default",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ProfileOptionDataEnumItem, default),
            },
        ],
    }),
    array_type: Some(PROFILEOPTIONDATAENUMITEM_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProfileOptionDataEnumItem {
    fn type_info() -> &'static TypeInfo {
        PROFILEOPTIONDATAENUMITEM_TYPE_INFO
    }
}


pub const PROFILEOPTIONDATAENUMITEM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionDataEnumItem-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ProfileOptionDataEnumItem-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ProfileOptionDataFloat {
    pub min: f32,
    pub max: f32,
    pub value: f32,
    pub step: f32,
}

pub const PROFILEOPTIONDATAFLOAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionDataFloat",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROFILEOPTIONDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Min",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ProfileOptionDataFloat, min),
            },
            FieldInfoData {
                name: "Max",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ProfileOptionDataFloat, max),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ProfileOptionDataFloat, value),
            },
            FieldInfoData {
                name: "Step",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ProfileOptionDataFloat, step),
            },
        ],
    }),
    array_type: Some(PROFILEOPTIONDATAFLOAT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProfileOptionDataFloat {
    fn type_info() -> &'static TypeInfo {
        PROFILEOPTIONDATAFLOAT_TYPE_INFO
    }
}


pub const PROFILEOPTIONDATAFLOAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionDataFloat-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ProfileOptionDataFloat-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProfileOptionDataInt {
    pub min: i32,
    pub max: i32,
    pub value: i32,
    pub step: i32,
}

pub const PROFILEOPTIONDATAINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionDataInt",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROFILEOPTIONDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Min",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ProfileOptionDataInt, min),
            },
            FieldInfoData {
                name: "Max",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ProfileOptionDataInt, max),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ProfileOptionDataInt, value),
            },
            FieldInfoData {
                name: "Step",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ProfileOptionDataInt, step),
            },
        ],
    }),
    array_type: Some(PROFILEOPTIONDATAINT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProfileOptionDataInt {
    fn type_info() -> &'static TypeInfo {
        PROFILEOPTIONDATAINT_TYPE_INFO
    }
}


pub const PROFILEOPTIONDATAINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionDataInt-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ProfileOptionDataInt-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProfileOptionData {
    pub unique_id: String,
    pub category: ProfileOptionsType,
    pub local_player_specific: bool,
    pub is_boot_option: bool,
}

pub const PROFILEOPTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "UniqueId",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ProfileOptionData, unique_id),
            },
            FieldInfoData {
                name: "Category",
                flags: MemberInfoFlags::new(0),
                field_type: PROFILEOPTIONSTYPE_TYPE_INFO,
                rust_offset: offset_of!(ProfileOptionData, category),
            },
            FieldInfoData {
                name: "LocalPlayerSpecific",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ProfileOptionData, local_player_specific),
            },
            FieldInfoData {
                name: "IsBootOption",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ProfileOptionData, is_boot_option),
            },
        ],
    }),
    array_type: Some(PROFILEOPTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProfileOptionData {
    fn type_info() -> &'static TypeInfo {
        PROFILEOPTIONDATA_TYPE_INFO
    }
}


pub const PROFILEOPTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ProfileOptionData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProfileOptionsAsset {
    pub file_name: String,
    pub content_name: String,
    pub file_size: u32,
    pub auto_save_on_quit: bool,
    pub options: Vec<ProfileOptionData>,
    pub options_ps3: Vec<ProfileOptionData>,
    pub options_xenon: Vec<ProfileOptionData>,
    pub options_gen4a: Vec<ProfileOptionData>,
    pub options_gen4b: Vec<ProfileOptionData>,
    pub options_win: Vec<ProfileOptionData>,
    pub options_android: Vec<ProfileOptionData>,
    pub optionsi_o_s: Vec<ProfileOptionData>,
}

pub const PROFILEOPTIONSASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionsAsset",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "FileName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ProfileOptionsAsset, file_name),
            },
            FieldInfoData {
                name: "ContentName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ProfileOptionsAsset, content_name),
            },
            FieldInfoData {
                name: "FileSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ProfileOptionsAsset, file_size),
            },
            FieldInfoData {
                name: "AutoSaveOnQuit",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ProfileOptionsAsset, auto_save_on_quit),
            },
            FieldInfoData {
                name: "Options",
                flags: MemberInfoFlags::new(144),
                field_type: PROFILEOPTIONDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ProfileOptionsAsset, options),
            },
            FieldInfoData {
                name: "OptionsPs3",
                flags: MemberInfoFlags::new(144),
                field_type: PROFILEOPTIONDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ProfileOptionsAsset, options_ps3),
            },
            FieldInfoData {
                name: "OptionsXenon",
                flags: MemberInfoFlags::new(144),
                field_type: PROFILEOPTIONDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ProfileOptionsAsset, options_xenon),
            },
            FieldInfoData {
                name: "OptionsGen4a",
                flags: MemberInfoFlags::new(144),
                field_type: PROFILEOPTIONDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ProfileOptionsAsset, options_gen4a),
            },
            FieldInfoData {
                name: "OptionsGen4b",
                flags: MemberInfoFlags::new(144),
                field_type: PROFILEOPTIONDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ProfileOptionsAsset, options_gen4b),
            },
            FieldInfoData {
                name: "OptionsWin",
                flags: MemberInfoFlags::new(144),
                field_type: PROFILEOPTIONDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ProfileOptionsAsset, options_win),
            },
            FieldInfoData {
                name: "OptionsAndroid",
                flags: MemberInfoFlags::new(144),
                field_type: PROFILEOPTIONDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ProfileOptionsAsset, options_android),
            },
            FieldInfoData {
                name: "OptionsiOS",
                flags: MemberInfoFlags::new(144),
                field_type: PROFILEOPTIONDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ProfileOptionsAsset, optionsi_o_s),
            },
        ],
    }),
    array_type: Some(PROFILEOPTIONSASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProfileOptionsAsset {
    fn type_info() -> &'static TypeInfo {
        PROFILEOPTIONSASSET_TYPE_INFO
    }
}


pub const PROFILEOPTIONSASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionsAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ProfileOptionsAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BinaryOption {
    pub name: String,
}

pub const BINARYOPTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BinaryOption",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(BinaryOption, name),
            },
        ],
    }),
    array_type: Some(BINARYOPTION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BinaryOption {
    fn type_info() -> &'static TypeInfo {
        BINARYOPTION_TYPE_INFO
    }
}


pub const BINARYOPTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BinaryOption-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("BinaryOption-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StringOption {
    pub name: String,
    pub value: String,
}

pub const STRINGOPTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StringOption",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(StringOption, name),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(StringOption, value),
            },
        ],
    }),
    array_type: Some(STRINGOPTION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StringOption {
    fn type_info() -> &'static TypeInfo {
        STRINGOPTION_TYPE_INFO
    }
}


pub const STRINGOPTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StringOption-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("StringOption-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IntOption {
    pub name: String,
    pub value: i32,
}

pub const INTOPTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntOption",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(IntOption, name),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(IntOption, value),
            },
        ],
    }),
    array_type: Some(INTOPTION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IntOption {
    fn type_info() -> &'static TypeInfo {
        INTOPTION_TYPE_INFO
    }
}


pub const INTOPTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntOption-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("IntOption-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FloatOption {
    pub name: String,
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub step: f32,
}

pub const FLOATOPTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatOption",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(FloatOption, name),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FloatOption, value),
            },
            FieldInfoData {
                name: "Min",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FloatOption, min),
            },
            FieldInfoData {
                name: "Max",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FloatOption, max),
            },
            FieldInfoData {
                name: "Step",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FloatOption, step),
            },
        ],
    }),
    array_type: Some(FLOATOPTION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FloatOption {
    fn type_info() -> &'static TypeInfo {
        FLOATOPTION_TYPE_INFO
    }
}


pub const FLOATOPTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatOption-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("FloatOption-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
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

pub const PROFILEOPTIONSTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionsType",
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(PROFILEOPTIONSTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ProfileOptionsType {
    fn type_info() -> &'static TypeInfo {
        PROFILEOPTIONSTYPE_TYPE_INFO
    }
}


pub const PROFILEOPTIONSTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionsType-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ProfileOptionsType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MaterialRelationTerrainDestructionData {
    pub dynamic_decal_template: super::terrain_base::TerrainDynamicDecalTemplateData,
}

pub const MATERIALRELATIONTERRAINDESTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialRelationTerrainDestructionData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSPROPERTYRELATIONPROPERTYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DynamicDecalTemplate",
                flags: MemberInfoFlags::new(0),
                field_type: TERRAINDYNAMICDECALTEMPLATEDATA_TYPE_INFO,
                rust_offset: offset_of!(MaterialRelationTerrainDestructionData, dynamic_decal_template),
            },
        ],
    }),
    array_type: Some(MATERIALRELATIONTERRAINDESTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MaterialRelationTerrainDestructionData {
    fn type_info() -> &'static TypeInfo {
        MATERIALRELATIONTERRAINDESTRUCTIONDATA_TYPE_INFO
    }
}


pub const MATERIALRELATIONTERRAINDESTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialRelationTerrainDestructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("MaterialRelationTerrainDestructionData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MaterialPropertyTerrainData {
    pub dirt_trigger_color: super::core::Vec3,
    pub destruction_effect: super::effect_base::EffectBlueprint,
    pub dirt_trigger_factor: f32,
    pub mask_material: super::entity::MaterialDecl,
}

pub const MATERIALPROPERTYTERRAINDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialPropertyTerrainData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSMATERIALRELATIONPROPERTYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DirtTriggerColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(MaterialPropertyTerrainData, dirt_trigger_color),
            },
            FieldInfoData {
                name: "DestructionEffect",
                flags: MemberInfoFlags::new(0),
                field_type: EFFECTBLUEPRINT_TYPE_INFO,
                rust_offset: offset_of!(MaterialPropertyTerrainData, destruction_effect),
            },
            FieldInfoData {
                name: "DirtTriggerFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MaterialPropertyTerrainData, dirt_trigger_factor),
            },
            FieldInfoData {
                name: "MaskMaterial",
                flags: MemberInfoFlags::new(0),
                field_type: MATERIALDECL_TYPE_INFO,
                rust_offset: offset_of!(MaterialPropertyTerrainData, mask_material),
            },
        ],
    }),
    array_type: Some(MATERIALPROPERTYTERRAINDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for MaterialPropertyTerrainData {
    fn type_info() -> &'static TypeInfo {
        MATERIALPROPERTYTERRAINDATA_TYPE_INFO
    }
}


pub const MATERIALPROPERTYTERRAINDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialPropertyTerrainData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("MaterialPropertyTerrainData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MaterialRelationSoundData {
    pub impact_sound: super::audio::SoundAsset,
    pub impact_sound_event: super::audio::AudioGraphEvent,
    pub scrape_sound: super::audio::SoundAsset,
    pub scrape_length: f32,
    pub scrape_impact_sound_event: super::audio::AudioGraphEvent,
}

pub const MATERIALRELATIONSOUNDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialRelationSoundData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSMATERIALRELATIONPROPERTYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ImpactSound",
                flags: MemberInfoFlags::new(0),
                field_type: SOUNDASSET_TYPE_INFO,
                rust_offset: offset_of!(MaterialRelationSoundData, impact_sound),
            },
            FieldInfoData {
                name: "ImpactSoundEvent",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHEVENT_TYPE_INFO,
                rust_offset: offset_of!(MaterialRelationSoundData, impact_sound_event),
            },
            FieldInfoData {
                name: "ScrapeSound",
                flags: MemberInfoFlags::new(0),
                field_type: SOUNDASSET_TYPE_INFO,
                rust_offset: offset_of!(MaterialRelationSoundData, scrape_sound),
            },
            FieldInfoData {
                name: "ScrapeLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MaterialRelationSoundData, scrape_length),
            },
            FieldInfoData {
                name: "ScrapeImpactSoundEvent",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHEVENT_TYPE_INFO,
                rust_offset: offset_of!(MaterialRelationSoundData, scrape_impact_sound_event),
            },
        ],
    }),
    array_type: Some(MATERIALRELATIONSOUNDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MaterialRelationSoundData {
    fn type_info() -> &'static TypeInfo {
        MATERIALRELATIONSOUNDDATA_TYPE_INFO
    }
}


pub const MATERIALRELATIONSOUNDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialRelationSoundData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("MaterialRelationSoundData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MaterialPropertySoundData {
    pub impact_sound: super::audio::SoundAsset,
    pub scrape_sound: super::audio::SoundAsset,
    pub scrape_length: f32,
    pub licensee_sound_data: MaterialPropertyLicenseeSoundData,
    pub softness: f32,
    pub material_sound_id: f32,
}

pub const MATERIALPROPERTYSOUNDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialPropertySoundData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSMATERIALRELATIONPROPERTYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ImpactSound",
                flags: MemberInfoFlags::new(0),
                field_type: SOUNDASSET_TYPE_INFO,
                rust_offset: offset_of!(MaterialPropertySoundData, impact_sound),
            },
            FieldInfoData {
                name: "ScrapeSound",
                flags: MemberInfoFlags::new(0),
                field_type: SOUNDASSET_TYPE_INFO,
                rust_offset: offset_of!(MaterialPropertySoundData, scrape_sound),
            },
            FieldInfoData {
                name: "ScrapeLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MaterialPropertySoundData, scrape_length),
            },
            FieldInfoData {
                name: "LicenseeSoundData",
                flags: MemberInfoFlags::new(0),
                field_type: MATERIALPROPERTYLICENSEESOUNDDATA_TYPE_INFO,
                rust_offset: offset_of!(MaterialPropertySoundData, licensee_sound_data),
            },
            FieldInfoData {
                name: "Softness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MaterialPropertySoundData, softness),
            },
            FieldInfoData {
                name: "MaterialSoundId",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MaterialPropertySoundData, material_sound_id),
            },
        ],
    }),
    array_type: Some(MATERIALPROPERTYSOUNDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MaterialPropertySoundData {
    fn type_info() -> &'static TypeInfo {
        MATERIALPROPERTYSOUNDDATA_TYPE_INFO
    }
}


pub const MATERIALPROPERTYSOUNDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialPropertySoundData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("MaterialPropertySoundData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MaterialPropertyLicenseeSoundData {
}

pub const MATERIALPROPERTYLICENSEESOUNDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialPropertyLicenseeSoundData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(MATERIALPROPERTYLICENSEESOUNDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MaterialPropertyLicenseeSoundData {
    fn type_info() -> &'static TypeInfo {
        MATERIALPROPERTYLICENSEESOUNDDATA_TYPE_INFO
    }
}


pub const MATERIALPROPERTYLICENSEESOUNDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialPropertyLicenseeSoundData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("MaterialPropertyLicenseeSoundData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MaterialPropertyFireData {
    pub is_burnable: bool,
}

pub const MATERIALPROPERTYFIREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialPropertyFireData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSMATERIALRELATIONPROPERTYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "IsBurnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MaterialPropertyFireData, is_burnable),
            },
        ],
    }),
    array_type: Some(MATERIALPROPERTYFIREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MaterialPropertyFireData {
    fn type_info() -> &'static TypeInfo {
        MATERIALPROPERTYFIREDATA_TYPE_INFO
    }
}


pub const MATERIALPROPERTYFIREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialPropertyFireData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("MaterialPropertyFireData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MaterialRelationVehicleData {
    pub chassi_effect: super::effect_base::EffectBlueprint,
    pub track_effect: super::effect_base::EffectBlueprint,
    pub wheel_effect: super::effect_base::EffectBlueprint,
    pub ground_effect: super::effect_base::EffectBlueprint,
}

pub const MATERIALRELATIONVEHICLEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialRelationVehicleData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSMATERIALRELATIONPROPERTYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ChassiEffect",
                flags: MemberInfoFlags::new(0),
                field_type: EFFECTBLUEPRINT_TYPE_INFO,
                rust_offset: offset_of!(MaterialRelationVehicleData, chassi_effect),
            },
            FieldInfoData {
                name: "TrackEffect",
                flags: MemberInfoFlags::new(0),
                field_type: EFFECTBLUEPRINT_TYPE_INFO,
                rust_offset: offset_of!(MaterialRelationVehicleData, track_effect),
            },
            FieldInfoData {
                name: "WheelEffect",
                flags: MemberInfoFlags::new(0),
                field_type: EFFECTBLUEPRINT_TYPE_INFO,
                rust_offset: offset_of!(MaterialRelationVehicleData, wheel_effect),
            },
            FieldInfoData {
                name: "GroundEffect",
                flags: MemberInfoFlags::new(0),
                field_type: EFFECTBLUEPRINT_TYPE_INFO,
                rust_offset: offset_of!(MaterialRelationVehicleData, ground_effect),
            },
        ],
    }),
    array_type: Some(MATERIALRELATIONVEHICLEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MaterialRelationVehicleData {
    fn type_info() -> &'static TypeInfo {
        MATERIALRELATIONVEHICLEDATA_TYPE_INFO
    }
}


pub const MATERIALRELATIONVEHICLEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialRelationVehicleData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("MaterialRelationVehicleData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MaterialRelationPenetrationData {
    pub never_penetrate: bool,
}

pub const MATERIALRELATIONPENETRATIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialRelationPenetrationData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSPROPERTYRELATIONPROPERTYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "NeverPenetrate",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MaterialRelationPenetrationData, never_penetrate),
            },
        ],
    }),
    array_type: Some(MATERIALRELATIONPENETRATIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MaterialRelationPenetrationData {
    fn type_info() -> &'static TypeInfo {
        MATERIALRELATIONPENETRATIONDATA_TYPE_INFO
    }
}


pub const MATERIALRELATIONPENETRATIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialRelationPenetrationData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("MaterialRelationPenetrationData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MaterialRelationDamageData {
    pub collision_damage_multiplier: f32,
    pub collision_damage_threshold: f32,
    pub damage_protection_multiplier: f32,
    pub damage_penetration_multiplier: f32,
    pub damage_protection_threshold: f32,
    pub explosion_cover_damage_modifier: f32,
    pub inflicts_demolition_damage: bool,
    pub allow_client_destruction: bool,
}

pub const MATERIALRELATIONDAMAGEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialRelationDamageData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSPROPERTYRELATIONPROPERTYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "CollisionDamageMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MaterialRelationDamageData, collision_damage_multiplier),
            },
            FieldInfoData {
                name: "CollisionDamageThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MaterialRelationDamageData, collision_damage_threshold),
            },
            FieldInfoData {
                name: "DamageProtectionMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MaterialRelationDamageData, damage_protection_multiplier),
            },
            FieldInfoData {
                name: "DamagePenetrationMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MaterialRelationDamageData, damage_penetration_multiplier),
            },
            FieldInfoData {
                name: "DamageProtectionThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MaterialRelationDamageData, damage_protection_threshold),
            },
            FieldInfoData {
                name: "ExplosionCoverDamageModifier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MaterialRelationDamageData, explosion_cover_damage_modifier),
            },
            FieldInfoData {
                name: "InflictsDemolitionDamage",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MaterialRelationDamageData, inflicts_demolition_damage),
            },
            FieldInfoData {
                name: "AllowClientDestruction",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MaterialRelationDamageData, allow_client_destruction),
            },
        ],
    }),
    array_type: Some(MATERIALRELATIONDAMAGEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MaterialRelationDamageData {
    fn type_info() -> &'static TypeInfo {
        MATERIALRELATIONDAMAGEDATA_TYPE_INFO
    }
}


pub const MATERIALRELATIONDAMAGEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialRelationDamageData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("MaterialRelationDamageData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MaterialRelationDynamicFireData {
    pub can_set_fire: bool,
    pub fire_damage: f32,
    pub cell_damage_radius: u32,
}

pub const MATERIALRELATIONDYNAMICFIREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialRelationDynamicFireData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSMATERIALRELATIONPROPERTYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "CanSetFire",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MaterialRelationDynamicFireData, can_set_fire),
            },
            FieldInfoData {
                name: "FireDamage",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MaterialRelationDynamicFireData, fire_damage),
            },
            FieldInfoData {
                name: "CellDamageRadius",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MaterialRelationDynamicFireData, cell_damage_radius),
            },
        ],
    }),
    array_type: Some(MATERIALRELATIONDYNAMICFIREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MaterialRelationDynamicFireData {
    fn type_info() -> &'static TypeInfo {
        MATERIALRELATIONDYNAMICFIREDATA_TYPE_INFO
    }
}


pub const MATERIALRELATIONDYNAMICFIREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialRelationDynamicFireData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("MaterialRelationDynamicFireData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MaterialRelationEffectData {
    pub impact_effects: Vec<EffectWithSpeedRange>,
    pub impact_debris: Vec<super::entity::ObjectBlueprint>,
    pub impact_effect_max_spread_angle: f32,
    pub mirror_impact_direction: bool,
    pub enable_inherited_velocity: bool,
}

pub const MATERIALRELATIONEFFECTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialRelationEffectData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSMATERIALRELATIONPROPERTYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ImpactEffects",
                flags: MemberInfoFlags::new(144),
                field_type: EFFECTWITHSPEEDRANGE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MaterialRelationEffectData, impact_effects),
            },
            FieldInfoData {
                name: "ImpactDebris",
                flags: MemberInfoFlags::new(144),
                field_type: OBJECTBLUEPRINT_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MaterialRelationEffectData, impact_debris),
            },
            FieldInfoData {
                name: "ImpactEffectMaxSpreadAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MaterialRelationEffectData, impact_effect_max_spread_angle),
            },
            FieldInfoData {
                name: "MirrorImpactDirection",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MaterialRelationEffectData, mirror_impact_direction),
            },
            FieldInfoData {
                name: "EnableInheritedVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MaterialRelationEffectData, enable_inherited_velocity),
            },
        ],
    }),
    array_type: Some(MATERIALRELATIONEFFECTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MaterialRelationEffectData {
    fn type_info() -> &'static TypeInfo {
        MATERIALRELATIONEFFECTDATA_TYPE_INFO
    }
}


pub const MATERIALRELATIONEFFECTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialRelationEffectData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("MaterialRelationEffectData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MaterialPropertyImpulseData {
    pub impulse_absorption_multiplier: f32,
}

pub const MATERIALPROPERTYIMPULSEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialPropertyImpulseData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSPROPERTYRELATIONPROPERTYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ImpulseAbsorptionMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MaterialPropertyImpulseData, impulse_absorption_multiplier),
            },
        ],
    }),
    array_type: Some(MATERIALPROPERTYIMPULSEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MaterialPropertyImpulseData {
    fn type_info() -> &'static TypeInfo {
        MATERIALPROPERTYIMPULSEDATA_TYPE_INFO
    }
}


pub const MATERIALPROPERTYIMPULSEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialPropertyImpulseData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("MaterialPropertyImpulseData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MaterialPropertyHealthData {
    pub health: f32,
}

pub const MATERIALPROPERTYHEALTHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialPropertyHealthData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSPROPERTYRELATIONPROPERTYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Health",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MaterialPropertyHealthData, health),
            },
        ],
    }),
    array_type: Some(MATERIALPROPERTYHEALTHDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MaterialPropertyHealthData {
    fn type_info() -> &'static TypeInfo {
        MATERIALPROPERTYHEALTHDATA_TYPE_INFO
    }
}


pub const MATERIALPROPERTYHEALTHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialPropertyHealthData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("MaterialPropertyHealthData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MaterialRelationDecalData {
    pub decal: super::render_base::DecalTemplateBaseAsset,
    pub exit_decal: super::render_base::DecalTemplateBaseAsset,
}

pub const MATERIALRELATIONDECALDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialRelationDecalData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSMATERIALRELATIONPROPERTYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Decal",
                flags: MemberInfoFlags::new(0),
                field_type: DECALTEMPLATEBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(MaterialRelationDecalData, decal),
            },
            FieldInfoData {
                name: "ExitDecal",
                flags: MemberInfoFlags::new(0),
                field_type: DECALTEMPLATEBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(MaterialRelationDecalData, exit_decal),
            },
        ],
    }),
    array_type: Some(MATERIALRELATIONDECALDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MaterialRelationDecalData {
    fn type_info() -> &'static TypeInfo {
        MATERIALRELATIONDECALDATA_TYPE_INFO
    }
}


pub const MATERIALRELATIONDECALDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialRelationDecalData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("MaterialRelationDecalData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MaterialPropertyEffectData {
    pub impact_effects: Vec<EffectWithSpeedRange>,
}

pub const MATERIALPROPERTYEFFECTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialPropertyEffectData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSMATERIALRELATIONPROPERTYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ImpactEffects",
                flags: MemberInfoFlags::new(144),
                field_type: EFFECTWITHSPEEDRANGE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MaterialPropertyEffectData, impact_effects),
            },
        ],
    }),
    array_type: Some(MATERIALPROPERTYEFFECTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MaterialPropertyEffectData {
    fn type_info() -> &'static TypeInfo {
        MATERIALPROPERTYEFFECTDATA_TYPE_INFO
    }
}


pub const MATERIALPROPERTYEFFECTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialPropertyEffectData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("MaterialPropertyEffectData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EffectWithSpeedRange {
    pub effect: super::effect_base::EffectBlueprint,
    pub min_speed: f32,
    pub max_speed: f32,
}

pub const EFFECTWITHSPEEDRANGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectWithSpeedRange",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Effect",
                flags: MemberInfoFlags::new(0),
                field_type: EFFECTBLUEPRINT_TYPE_INFO,
                rust_offset: offset_of!(EffectWithSpeedRange, effect),
            },
            FieldInfoData {
                name: "MinSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EffectWithSpeedRange, min_speed),
            },
            FieldInfoData {
                name: "MaxSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EffectWithSpeedRange, max_speed),
            },
        ],
    }),
    array_type: Some(EFFECTWITHSPEEDRANGE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EffectWithSpeedRange {
    fn type_info() -> &'static TypeInfo {
        EFFECTWITHSPEEDRANGE_TYPE_INFO
    }
}


pub const EFFECTWITHSPEEDRANGE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectWithSpeedRange-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("EffectWithSpeedRange-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LevelDescriptionAsset {
    pub level_name: String,
    pub categories: Vec<LevelDescriptionInclusionCategory>,
    pub level_descriptions: Vec<LevelDescription>,
    pub bundles: Vec<LevelBundleLoad>,
    pub start_points: Vec<LevelStartPoint>,
    pub super_bundles: Vec<String>,
    pub level_guid: super::core::Guid,
}

pub const LEVELDESCRIPTIONASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelDescriptionAsset",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "LevelName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(LevelDescriptionAsset, level_name),
            },
            FieldInfoData {
                name: "Categories",
                flags: MemberInfoFlags::new(144),
                field_type: LEVELDESCRIPTIONINCLUSIONCATEGORY_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LevelDescriptionAsset, categories),
            },
            FieldInfoData {
                name: "LevelDescriptions",
                flags: MemberInfoFlags::new(144),
                field_type: LEVELDESCRIPTION_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LevelDescriptionAsset, level_descriptions),
            },
            FieldInfoData {
                name: "Bundles",
                flags: MemberInfoFlags::new(144),
                field_type: LEVELBUNDLELOAD_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LevelDescriptionAsset, bundles),
            },
            FieldInfoData {
                name: "StartPoints",
                flags: MemberInfoFlags::new(144),
                field_type: LEVELSTARTPOINT_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LevelDescriptionAsset, start_points),
            },
            FieldInfoData {
                name: "SuperBundles",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LevelDescriptionAsset, super_bundles),
            },
            FieldInfoData {
                name: "LevelGuid",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(LevelDescriptionAsset, level_guid),
            },
        ],
    }),
    array_type: Some(LEVELDESCRIPTIONASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LevelDescriptionAsset {
    fn type_info() -> &'static TypeInfo {
        LEVELDESCRIPTIONASSET_TYPE_INFO
    }
}


pub const LEVELDESCRIPTIONASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelDescriptionAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("LevelDescriptionAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LevelStartPoint {
    pub name: String,
    pub autoload_sublevels: Vec<String>,
    pub is_default: bool,
}

pub const LEVELSTARTPOINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelStartPoint",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(LevelStartPoint, name),
            },
            FieldInfoData {
                name: "AutoloadSublevels",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LevelStartPoint, autoload_sublevels),
            },
            FieldInfoData {
                name: "IsDefault",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LevelStartPoint, is_default),
            },
        ],
    }),
    array_type: Some(LEVELSTARTPOINT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LevelStartPoint {
    fn type_info() -> &'static TypeInfo {
        LEVELSTARTPOINT_TYPE_INFO
    }
}


pub const LEVELSTARTPOINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelStartPoint-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("LevelStartPoint-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LevelBundleLoad {
    pub name: String,
    pub try_keep_between_levels: bool,
}

pub const LEVELBUNDLELOAD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelBundleLoad",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(LevelBundleLoad, name),
            },
            FieldInfoData {
                name: "TryKeepBetweenLevels",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LevelBundleLoad, try_keep_between_levels),
            },
        ],
    }),
    array_type: Some(LEVELBUNDLELOAD_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LevelBundleLoad {
    fn type_info() -> &'static TypeInfo {
        LEVELBUNDLELOAD_TYPE_INFO
    }
}


pub const LEVELBUNDLELOAD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelBundleLoad-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("LevelBundleLoad-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LevelDescriptionInclusionCategory {
    pub category: String,
    pub mode: Vec<String>,
}

pub const LEVELDESCRIPTIONINCLUSIONCATEGORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelDescriptionInclusionCategory",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Category",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(LevelDescriptionInclusionCategory, category),
            },
            FieldInfoData {
                name: "Mode",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LevelDescriptionInclusionCategory, mode),
            },
        ],
    }),
    array_type: Some(LEVELDESCRIPTIONINCLUSIONCATEGORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LevelDescriptionInclusionCategory {
    fn type_info() -> &'static TypeInfo {
        LEVELDESCRIPTIONINCLUSIONCATEGORY_TYPE_INFO
    }
}


pub const LEVELDESCRIPTIONINCLUSIONCATEGORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelDescriptionInclusionCategory-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("LevelDescriptionInclusionCategory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WorldRenderLevelDescriptionComponent {
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

pub const WORLDRENDERLEVELDESCRIPTIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldRenderLevelDescriptionComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LEVELDESCRIPTIONCOMPONENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ShadowmapSliceCountOffset",
                flags: MemberInfoFlags::new(0),
                field_type: PLATFORMSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(WorldRenderLevelDescriptionComponent, shadowmap_slice_count_offset),
            },
            FieldInfoData {
                name: "ShadowmapSliceCountMin",
                flags: MemberInfoFlags::new(0),
                field_type: PLATFORMSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(WorldRenderLevelDescriptionComponent, shadowmap_slice_count_min),
            },
            FieldInfoData {
                name: "ShadowmapSliceCountMax",
                flags: MemberInfoFlags::new(0),
                field_type: PLATFORMSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(WorldRenderLevelDescriptionComponent, shadowmap_slice_count_max),
            },
            FieldInfoData {
                name: "ShadowmapSliceResolutionScale",
                flags: MemberInfoFlags::new(0),
                field_type: PLATFORMSCALABLEFLOAT_TYPE_INFO,
                rust_offset: offset_of!(WorldRenderLevelDescriptionComponent, shadowmap_slice_resolution_scale),
            },
            FieldInfoData {
                name: "SpotlightShadowCount",
                flags: MemberInfoFlags::new(0),
                field_type: PLATFORMSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(WorldRenderLevelDescriptionComponent, spotlight_shadow_count),
            },
            FieldInfoData {
                name: "TransparentDofEnable",
                flags: MemberInfoFlags::new(0),
                field_type: PLATFORMSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(WorldRenderLevelDescriptionComponent, transparent_dof_enable),
            },
            FieldInfoData {
                name: "SpriteDofEnable",
                flags: MemberInfoFlags::new(0),
                field_type: PLATFORMSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(WorldRenderLevelDescriptionComponent, sprite_dof_enable),
            },
            FieldInfoData {
                name: "SpriteDofMaxRadiusGatherPass",
                flags: MemberInfoFlags::new(0),
                field_type: PLATFORMSCALABLEFLOAT_TYPE_INFO,
                rust_offset: offset_of!(WorldRenderLevelDescriptionComponent, sprite_dof_max_radius_gather_pass),
            },
            FieldInfoData {
                name: "LocalPlanarReflectionsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: PLATFORMSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(WorldRenderLevelDescriptionComponent, local_planar_reflections_enable),
            },
            FieldInfoData {
                name: "LocalPlanarReflectionConvolutionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: PLATFORMSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(WorldRenderLevelDescriptionComponent, local_planar_reflection_convolution_enable),
            },
            FieldInfoData {
                name: "LocalPlanarReflectionForceLowestLodEnable",
                flags: MemberInfoFlags::new(0),
                field_type: PLATFORMSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(WorldRenderLevelDescriptionComponent, local_planar_reflection_force_lowest_lod_enable),
            },
            FieldInfoData {
                name: "LocalLightShadowAtlasSlotCount",
                flags: MemberInfoFlags::new(0),
                field_type: PLATFORMSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(WorldRenderLevelDescriptionComponent, local_light_shadow_atlas_slot_count),
            },
            FieldInfoData {
                name: "LocalLightShadowAtlasSlotResolution",
                flags: MemberInfoFlags::new(0),
                field_type: PLATFORMSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(WorldRenderLevelDescriptionComponent, local_light_shadow_atlas_slot_resolution),
            },
            FieldInfoData {
                name: "LocalLightShadowResolutionHigh",
                flags: MemberInfoFlags::new(0),
                field_type: PLATFORMSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(WorldRenderLevelDescriptionComponent, local_light_shadow_resolution_high),
            },
            FieldInfoData {
                name: "LocalLightShadowResolutionUltra",
                flags: MemberInfoFlags::new(0),
                field_type: PLATFORMSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(WorldRenderLevelDescriptionComponent, local_light_shadow_resolution_ultra),
            },
            FieldInfoData {
                name: "ProceduralSkyReceiveHeightFog",
                flags: MemberInfoFlags::new(0),
                field_type: PLATFORMSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(WorldRenderLevelDescriptionComponent, procedural_sky_receive_height_fog),
            },
        ],
    }),
    array_type: Some(WORLDRENDERLEVELDESCRIPTIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WorldRenderLevelDescriptionComponent {
    fn type_info() -> &'static TypeInfo {
        WORLDRENDERLEVELDESCRIPTIONCOMPONENT_TYPE_INFO
    }
}


pub const WORLDRENDERLEVELDESCRIPTIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldRenderLevelDescriptionComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("WorldRenderLevelDescriptionComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MemoryLevelDescriptionComponent {
    pub texture_streaming_pool_size: super::core::PlatformScalableInt,
    pub mesh_streaming_pool_size: super::core::PlatformScalableInt,
    pub emitter_base_atlas_width: super::core::PlatformScalableInt,
    pub emitter_base_atlas_height: super::core::PlatformScalableInt,
    pub emitter_base_atlas_mipmap_count: super::core::PlatformScalableInt,
    pub emitter_normal_atlas_width: super::core::PlatformScalableInt,
    pub emitter_normal_atlas_height: super::core::PlatformScalableInt,
    pub emitter_normal_atlas_mipmap_count: super::core::PlatformScalableInt,
}

pub const MEMORYLEVELDESCRIPTIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryLevelDescriptionComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LEVELDESCRIPTIONCOMPONENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "TextureStreamingPoolSize",
                flags: MemberInfoFlags::new(0),
                field_type: PLATFORMSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(MemoryLevelDescriptionComponent, texture_streaming_pool_size),
            },
            FieldInfoData {
                name: "MeshStreamingPoolSize",
                flags: MemberInfoFlags::new(0),
                field_type: PLATFORMSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(MemoryLevelDescriptionComponent, mesh_streaming_pool_size),
            },
            FieldInfoData {
                name: "EmitterBaseAtlasWidth",
                flags: MemberInfoFlags::new(0),
                field_type: PLATFORMSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(MemoryLevelDescriptionComponent, emitter_base_atlas_width),
            },
            FieldInfoData {
                name: "EmitterBaseAtlasHeight",
                flags: MemberInfoFlags::new(0),
                field_type: PLATFORMSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(MemoryLevelDescriptionComponent, emitter_base_atlas_height),
            },
            FieldInfoData {
                name: "EmitterBaseAtlasMipmapCount",
                flags: MemberInfoFlags::new(0),
                field_type: PLATFORMSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(MemoryLevelDescriptionComponent, emitter_base_atlas_mipmap_count),
            },
            FieldInfoData {
                name: "EmitterNormalAtlasWidth",
                flags: MemberInfoFlags::new(0),
                field_type: PLATFORMSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(MemoryLevelDescriptionComponent, emitter_normal_atlas_width),
            },
            FieldInfoData {
                name: "EmitterNormalAtlasHeight",
                flags: MemberInfoFlags::new(0),
                field_type: PLATFORMSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(MemoryLevelDescriptionComponent, emitter_normal_atlas_height),
            },
            FieldInfoData {
                name: "EmitterNormalAtlasMipmapCount",
                flags: MemberInfoFlags::new(0),
                field_type: PLATFORMSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(MemoryLevelDescriptionComponent, emitter_normal_atlas_mipmap_count),
            },
        ],
    }),
    array_type: Some(MEMORYLEVELDESCRIPTIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MemoryLevelDescriptionComponent {
    fn type_info() -> &'static TypeInfo {
        MEMORYLEVELDESCRIPTIONCOMPONENT_TYPE_INFO
    }
}


pub const MEMORYLEVELDESCRIPTIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryLevelDescriptionComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("MemoryLevelDescriptionComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LevelDescription {
    pub name: String,
    pub sub_name: String,
    pub description: String,
    pub is_singleplayer: bool,
    pub is_coop: bool,
    pub is_menu: bool,
    pub is_epilogue: bool,
    pub components: Vec<LevelDescriptionComponent>,
}

pub const LEVELDESCRIPTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelDescription",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(LevelDescription, name),
            },
            FieldInfoData {
                name: "SubName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(LevelDescription, sub_name),
            },
            FieldInfoData {
                name: "Description",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(LevelDescription, description),
            },
            FieldInfoData {
                name: "IsSingleplayer",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LevelDescription, is_singleplayer),
            },
            FieldInfoData {
                name: "IsCoop",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LevelDescription, is_coop),
            },
            FieldInfoData {
                name: "IsMenu",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LevelDescription, is_menu),
            },
            FieldInfoData {
                name: "IsEpilogue",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LevelDescription, is_epilogue),
            },
            FieldInfoData {
                name: "Components",
                flags: MemberInfoFlags::new(144),
                field_type: LEVELDESCRIPTIONCOMPONENT_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LevelDescription, components),
            },
        ],
    }),
    array_type: Some(LEVELDESCRIPTION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LevelDescription {
    fn type_info() -> &'static TypeInfo {
        LEVELDESCRIPTION_TYPE_INFO
    }
}


pub const LEVELDESCRIPTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelDescription-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("LevelDescription-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LevelDescriptionComponent {
}

pub const LEVELDESCRIPTIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelDescriptionComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LEVELDESCRIPTIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LevelDescriptionComponent {
    fn type_info() -> &'static TypeInfo {
        LEVELDESCRIPTIONCOMPONENT_TYPE_INFO
    }
}


pub const LEVELDESCRIPTIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelDescriptionComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("LevelDescriptionComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LevelData {
    pub a_i_system: GameAISystem,
    pub a_i2_system: GameAISystem,
    pub game_configuration_name: String,
    pub default_f_o_v: f32,
    pub infantry_f_o_v_multiplier: f32,
    pub stream_pool_preset: super::audio::StreamPoolPreset,
    pub voice_over_system: super::audio::VoiceOverSystemAsset,
    pub voice_over_logic: Vec<super::audio::VoiceOverLogicAsset>,
    pub max_vehicle_height: f32,
    pub huge_broad_phase: bool,
    pub enlighten_shader_database: super::render_base::EnlightenShaderDatabaseBaseAsset,
    pub ant_project_asset: super::ant::AntProjectAsset,
    pub aerial_heightmap_data: String,
    pub free_streaming_enable: bool,
    pub camera_modes: Vec<CameraModeAsset>,
    pub camera_transitions: Vec<CameraTransition>,
    pub preload_info: LevelPreloadInfo,
    pub face_animation_wave_mappings: FaceAnimationWaveMappings,
    pub auto_load_bundles: Vec<String>,
    pub unlock_id_table: UnlockIdTable,
    pub autoloaded_detached_sub_world_datas: Vec<super::entity::SubWorldReferenceObjectData>,
}

pub const LEVELDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WORLDDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "AISystem",
                flags: MemberInfoFlags::new(0),
                field_type: GAMEAISYSTEM_TYPE_INFO,
                rust_offset: offset_of!(LevelData, a_i_system),
            },
            FieldInfoData {
                name: "AI2System",
                flags: MemberInfoFlags::new(0),
                field_type: GAMEAISYSTEM_TYPE_INFO,
                rust_offset: offset_of!(LevelData, a_i2_system),
            },
            FieldInfoData {
                name: "GameConfigurationName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(LevelData, game_configuration_name),
            },
            FieldInfoData {
                name: "DefaultFOV",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LevelData, default_f_o_v),
            },
            FieldInfoData {
                name: "InfantryFOVMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LevelData, infantry_f_o_v_multiplier),
            },
            FieldInfoData {
                name: "StreamPoolPreset",
                flags: MemberInfoFlags::new(0),
                field_type: STREAMPOOLPRESET_TYPE_INFO,
                rust_offset: offset_of!(LevelData, stream_pool_preset),
            },
            FieldInfoData {
                name: "VoiceOverSystem",
                flags: MemberInfoFlags::new(0),
                field_type: VOICEOVERSYSTEMASSET_TYPE_INFO,
                rust_offset: offset_of!(LevelData, voice_over_system),
            },
            FieldInfoData {
                name: "VoiceOverLogic",
                flags: MemberInfoFlags::new(144),
                field_type: VOICEOVERLOGICASSET_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LevelData, voice_over_logic),
            },
            FieldInfoData {
                name: "MaxVehicleHeight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LevelData, max_vehicle_height),
            },
            FieldInfoData {
                name: "HugeBroadPhase",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LevelData, huge_broad_phase),
            },
            FieldInfoData {
                name: "EnlightenShaderDatabase",
                flags: MemberInfoFlags::new(0),
                field_type: ENLIGHTENSHADERDATABASEBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(LevelData, enlighten_shader_database),
            },
            FieldInfoData {
                name: "AntProjectAsset",
                flags: MemberInfoFlags::new(0),
                field_type: ANTPROJECTASSET_TYPE_INFO,
                rust_offset: offset_of!(LevelData, ant_project_asset),
            },
            FieldInfoData {
                name: "AerialHeightmapData",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(LevelData, aerial_heightmap_data),
            },
            FieldInfoData {
                name: "FreeStreamingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LevelData, free_streaming_enable),
            },
            FieldInfoData {
                name: "CameraModes",
                flags: MemberInfoFlags::new(144),
                field_type: CAMERAMODEASSET_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LevelData, camera_modes),
            },
            FieldInfoData {
                name: "CameraTransitions",
                flags: MemberInfoFlags::new(144),
                field_type: CAMERATRANSITION_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LevelData, camera_transitions),
            },
            FieldInfoData {
                name: "PreloadInfo",
                flags: MemberInfoFlags::new(0),
                field_type: LEVELPRELOADINFO_TYPE_INFO,
                rust_offset: offset_of!(LevelData, preload_info),
            },
            FieldInfoData {
                name: "FaceAnimationWaveMappings",
                flags: MemberInfoFlags::new(0),
                field_type: FACEANIMATIONWAVEMAPPINGS_TYPE_INFO,
                rust_offset: offset_of!(LevelData, face_animation_wave_mappings),
            },
            FieldInfoData {
                name: "AutoLoadBundles",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LevelData, auto_load_bundles),
            },
            FieldInfoData {
                name: "UnlockIdTable",
                flags: MemberInfoFlags::new(0),
                field_type: UNLOCKIDTABLE_TYPE_INFO,
                rust_offset: offset_of!(LevelData, unlock_id_table),
            },
            FieldInfoData {
                name: "AutoloadedDetachedSubWorldDatas",
                flags: MemberInfoFlags::new(144),
                field_type: SUBWORLDREFERENCEOBJECTDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LevelData, autoloaded_detached_sub_world_datas),
            },
        ],
    }),
    array_type: Some(LEVELDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LevelData {
    fn type_info() -> &'static TypeInfo {
        LEVELDATA_TYPE_INFO
    }
}


pub const LEVELDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("LevelData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UnlockIdTable {
    pub identifiers: Vec<u32>,
}

pub const UNLOCKIDTABLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UnlockIdTable",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Identifiers",
                flags: MemberInfoFlags::new(144),
                field_type: UINT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(UnlockIdTable, identifiers),
            },
        ],
    }),
    array_type: Some(UNLOCKIDTABLE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UnlockIdTable {
    fn type_info() -> &'static TypeInfo {
        UNLOCKIDTABLE_TYPE_INFO
    }
}


pub const UNLOCKIDTABLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UnlockIdTable-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("UnlockIdTable-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LevelDataComponent {
}

pub const LEVELDATACOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelDataComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LEVELDATACOMPONENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LevelDataComponent {
    fn type_info() -> &'static TypeInfo {
        LEVELDATACOMPONENT_TYPE_INFO
    }
}


pub const LEVELDATACOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelDataComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("LevelDataComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LevelPreloadInfo {
    pub preloaded_blueprint_bundles: Vec<String>,
    pub sub_level_preload_info_map: Vec<SubLevelPreloadInfo>,
}

pub const LEVELPRELOADINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelPreloadInfo",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "PreloadedBlueprintBundles",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LevelPreloadInfo, preloaded_blueprint_bundles),
            },
            FieldInfoData {
                name: "SubLevelPreloadInfoMap",
                flags: MemberInfoFlags::new(144),
                field_type: SUBLEVELPRELOADINFO_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LevelPreloadInfo, sub_level_preload_info_map),
            },
        ],
    }),
    array_type: Some(LEVELPRELOADINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LevelPreloadInfo {
    fn type_info() -> &'static TypeInfo {
        LEVELPRELOADINFO_TYPE_INFO
    }
}


pub const LEVELPRELOADINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelPreloadInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("LevelPreloadInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SubLevelPreloadInfo {
    pub sub_level_bundle_path: String,
    pub preloaded_blueprint_bundles: Vec<String>,
}

pub const SUBLEVELPRELOADINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelPreloadInfo",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "SubLevelBundlePath",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(SubLevelPreloadInfo, sub_level_bundle_path),
            },
            FieldInfoData {
                name: "PreloadedBlueprintBundles",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SubLevelPreloadInfo, preloaded_blueprint_bundles),
            },
        ],
    }),
    array_type: Some(SUBLEVELPRELOADINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SubLevelPreloadInfo {
    fn type_info() -> &'static TypeInfo {
        SUBLEVELPRELOADINFO_TYPE_INFO
    }
}


pub const SUBLEVELPRELOADINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelPreloadInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("SubLevelPreloadInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CameraTransition {
}

pub const CAMERATRANSITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraTransition",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CAMERATRANSITION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CameraTransition {
    fn type_info() -> &'static TypeInfo {
        CAMERATRANSITION_TYPE_INFO
    }
}


pub const CAMERATRANSITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraTransition-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("CameraTransition-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CameraModeAsset {
}

pub const CAMERAMODEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraModeAsset",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CAMERAMODEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CameraModeAsset {
    fn type_info() -> &'static TypeInfo {
        CAMERAMODEASSET_TYPE_INFO
    }
}


pub const CAMERAMODEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraModeAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("CameraModeAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GameAISystem {
    pub excluded_game_mode_list: Vec<String>,
}

pub const GAMEAISYSTEM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameAISystem",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ExcludedGameModeList",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(GameAISystem, excluded_game_mode_list),
            },
        ],
    }),
    array_type: Some(GAMEAISYSTEM_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GameAISystem {
    fn type_info() -> &'static TypeInfo {
        GAMEAISYSTEM_TYPE_INFO
    }
}


pub const GAMEAISYSTEM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameAISystem-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("GameAISystem-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FaceAnimationWaveMappingsAsset {
    pub mappings: Vec<FaceAnimationWaveMappings>,
}

pub const FACEANIMATIONWAVEMAPPINGSASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FaceAnimationWaveMappingsAsset",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Mappings",
                flags: MemberInfoFlags::new(144),
                field_type: FACEANIMATIONWAVEMAPPINGS_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(FaceAnimationWaveMappingsAsset, mappings),
            },
        ],
    }),
    array_type: Some(FACEANIMATIONWAVEMAPPINGSASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FaceAnimationWaveMappingsAsset {
    fn type_info() -> &'static TypeInfo {
        FACEANIMATIONWAVEMAPPINGSASSET_TYPE_INFO
    }
}


pub const FACEANIMATIONWAVEMAPPINGSASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FaceAnimationWaveMappingsAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("FaceAnimationWaveMappingsAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FaceAnimationWaveMappings {
    pub mappings: Vec<FaceAnimationWaveMapping>,
}

pub const FACEANIMATIONWAVEMAPPINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FaceAnimationWaveMappings",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Mappings",
                flags: MemberInfoFlags::new(144),
                field_type: FACEANIMATIONWAVEMAPPING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(FaceAnimationWaveMappings, mappings),
            },
        ],
    }),
    array_type: Some(FACEANIMATIONWAVEMAPPINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FaceAnimationWaveMappings {
    fn type_info() -> &'static TypeInfo {
        FACEANIMATIONWAVEMAPPINGS_TYPE_INFO
    }
}


pub const FACEANIMATIONWAVEMAPPINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FaceAnimationWaveMappings-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("FaceAnimationWaveMappings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FaceAnimationWaveMapping {
    pub wave_name_hash: i32,
    pub wave_variation_index: u16,
    pub facial_animation: super::ant::AntRef,
}

pub const FACEANIMATIONWAVEMAPPING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FaceAnimationWaveMapping",
    flags: MemberInfoFlags::new(32841),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "WaveNameHash",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(FaceAnimationWaveMapping, wave_name_hash),
            },
            FieldInfoData {
                name: "WaveVariationIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(FaceAnimationWaveMapping, wave_variation_index),
            },
            FieldInfoData {
                name: "FacialAnimation",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(FaceAnimationWaveMapping, facial_animation),
            },
        ],
    }),
    array_type: Some(FACEANIMATIONWAVEMAPPING_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for FaceAnimationWaveMapping {
    fn type_info() -> &'static TypeInfo {
        FACEANIMATIONWAVEMAPPING_TYPE_INFO
    }
}


pub const FACEANIMATIONWAVEMAPPING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FaceAnimationWaveMapping-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("FaceAnimationWaveMapping-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct BigWorldSettingsAsset {
    pub settings: Vec<BigWorldSetting>,
}

pub const BIGWORLDSETTINGSASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BigWorldSettingsAsset",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Settings",
                flags: MemberInfoFlags::new(144),
                field_type: BIGWORLDSETTING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(BigWorldSettingsAsset, settings),
            },
        ],
    }),
    array_type: Some(BIGWORLDSETTINGSASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BigWorldSettingsAsset {
    fn type_info() -> &'static TypeInfo {
        BIGWORLDSETTINGSASSET_TYPE_INFO
    }
}


pub const BIGWORLDSETTINGSASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BigWorldSettingsAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("BigWorldSettingsAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct BigWorldSetting {
    pub sound: super::audio::SoundAsset,
    pub min_distance: i32,
    pub max_distance: i32,
    pub min_delay_time_in_minutes: f32,
    pub max_delay_time_in_minutes: f32,
}

pub const BIGWORLDSETTING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BigWorldSetting",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Sound",
                flags: MemberInfoFlags::new(0),
                field_type: SOUNDASSET_TYPE_INFO,
                rust_offset: offset_of!(BigWorldSetting, sound),
            },
            FieldInfoData {
                name: "MinDistance",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(BigWorldSetting, min_distance),
            },
            FieldInfoData {
                name: "MaxDistance",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(BigWorldSetting, max_distance),
            },
            FieldInfoData {
                name: "MinDelayTimeInMinutes",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BigWorldSetting, min_delay_time_in_minutes),
            },
            FieldInfoData {
                name: "MaxDelayTimeInMinutes",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BigWorldSetting, max_delay_time_in_minutes),
            },
        ],
    }),
    array_type: Some(BIGWORLDSETTING_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BigWorldSetting {
    fn type_info() -> &'static TypeInfo {
        BIGWORLDSETTING_TYPE_INFO
    }
}


pub const BIGWORLDSETTING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BigWorldSetting-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("BigWorldSetting-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LevelReportingAsset {
    pub built_levels: Vec<super::core::Guid>,
}

pub const LEVELREPORTINGASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelReportingAsset",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "BuiltLevels",
                flags: MemberInfoFlags::new(144),
                field_type: GUID_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LevelReportingAsset, built_levels),
            },
        ],
    }),
    array_type: Some(LEVELREPORTINGASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LevelReportingAsset {
    fn type_info() -> &'static TypeInfo {
        LEVELREPORTINGASSET_TYPE_INFO
    }
}


pub const LEVELREPORTINGASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelReportingAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("LevelReportingAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HudData {
}

pub const HUDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HudData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(HUDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for HudData {
    fn type_info() -> &'static TypeInfo {
        HUDDATA_TYPE_INFO
    }
}


pub const HUDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HudData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("HudData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct HIKData {
    pub reach_t: f32,
    pub reach_r: f32,
    pub pull: f32,
    pub resist: f32,
}

pub const HIKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HIKData",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ReachT",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HIKData, reach_t),
            },
            FieldInfoData {
                name: "ReachR",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HIKData, reach_r),
            },
            FieldInfoData {
                name: "Pull",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HIKData, pull),
            },
            FieldInfoData {
                name: "Resist",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HIKData, resist),
            },
        ],
    }),
    array_type: Some(HIKDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for HIKData {
    fn type_info() -> &'static TypeInfo {
        HIKDATA_TYPE_INFO
    }
}


pub const HIKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HIKData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("HIKData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GameAnimationSettings {
    pub ant_on_client_only_gamemodes: Vec<String>,
    pub server_enable: bool,
    pub client_enable: bool,
}

pub const GAMEANIMATIONSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameAnimationSettings",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "AntOnClientOnlyGamemodes",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(GameAnimationSettings, ant_on_client_only_gamemodes),
            },
            FieldInfoData {
                name: "ServerEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameAnimationSettings, server_enable),
            },
            FieldInfoData {
                name: "ClientEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameAnimationSettings, client_enable),
            },
        ],
    }),
    array_type: Some(GAMEANIMATIONSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GameAnimationSettings {
    fn type_info() -> &'static TypeInfo {
        GAMEANIMATIONSETTINGS_TYPE_INFO
    }
}


pub const GAMEANIMATIONSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameAnimationSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("GameAnimationSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DemoSettings {
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

pub const DEMOSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DemoSettings",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "RecordDemoFileName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(DemoSettings, record_demo_file_name),
            },
            FieldInfoData {
                name: "PlaybackDemoFileName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(DemoSettings, playback_demo_file_name),
            },
            FieldInfoData {
                name: "TimeDemo",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(DemoSettings, time_demo),
            },
            FieldInfoData {
                name: "LockToPlayerName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(DemoSettings, lock_to_player_name),
            },
            FieldInfoData {
                name: "ChangePlayerInterval",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DemoSettings, change_player_interval),
            },
            FieldInfoData {
                name: "ForcedDeltaTickCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DemoSettings, forced_delta_tick_count),
            },
            FieldInfoData {
                name: "StartProfilingOnFrame",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DemoSettings, start_profiling_on_frame),
            },
            FieldInfoData {
                name: "StopProfilingOnFrame",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DemoSettings, stop_profiling_on_frame),
            },
            FieldInfoData {
                name: "PauseOnStartup",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DemoSettings, pause_on_startup),
            },
            FieldInfoData {
                name: "AllowOverwrite",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DemoSettings, allow_overwrite),
            },
            FieldInfoData {
                name: "LogPerformance",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DemoSettings, log_performance),
            },
            FieldInfoData {
                name: "SuppressDebugLog",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DemoSettings, suppress_debug_log),
            },
            FieldInfoData {
                name: "ShutdownOnDemoComplete",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DemoSettings, shutdown_on_demo_complete),
            },
            FieldInfoData {
                name: "LoopingDemo",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DemoSettings, looping_demo),
            },
            FieldInfoData {
                name: "LockToRandomPlayer",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DemoSettings, lock_to_random_player),
            },
            FieldInfoData {
                name: "TakeScreenshotOnFrame",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DemoSettings, take_screenshot_on_frame),
            },
        ],
    }),
    array_type: Some(DEMOSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DemoSettings {
    fn type_info() -> &'static TypeInfo {
        DEMOSETTINGS_TYPE_INFO
    }
}


pub const DEMOSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DemoSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("DemoSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PerformanceTrackerSettings {
    pub enabled: bool,
    pub interval: f32,
    pub supress_performance_stats_on_idle: bool,
    pub supress_performance_stats_until_spawned: bool,
    pub juice_log_performance: bool,
}

pub const PERFORMANCETRACKERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerformanceTrackerSettings",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PerformanceTrackerSettings, enabled),
            },
            FieldInfoData {
                name: "Interval",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PerformanceTrackerSettings, interval),
            },
            FieldInfoData {
                name: "SupressPerformanceStatsOnIdle",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PerformanceTrackerSettings, supress_performance_stats_on_idle),
            },
            FieldInfoData {
                name: "SupressPerformanceStatsUntilSpawned",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PerformanceTrackerSettings, supress_performance_stats_until_spawned),
            },
            FieldInfoData {
                name: "JuiceLogPerformance",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PerformanceTrackerSettings, juice_log_performance),
            },
        ],
    }),
    array_type: Some(PERFORMANCETRACKERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PerformanceTrackerSettings {
    fn type_info() -> &'static TypeInfo {
        PERFORMANCETRACKERSETTINGS_TYPE_INFO
    }
}


pub const PERFORMANCETRACKERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerformanceTrackerSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("PerformanceTrackerSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct GameTimeSettings {
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

pub const GAMETIMESETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameTimeSettings",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "UseWaitableTimers",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameTimeSettings, use_waitable_timers),
            },
            FieldInfoData {
                name: "MaxSimFps",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameTimeSettings, max_sim_fps),
            },
            FieldInfoData {
                name: "ForceSimRate",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(GameTimeSettings, force_sim_rate),
            },
            FieldInfoData {
                name: "MaxVirtualTicks",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(GameTimeSettings, max_virtual_ticks),
            },
            FieldInfoData {
                name: "MaxVariableFps",
                flags: MemberInfoFlags::new(8192),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameTimeSettings, max_variable_fps),
            },
            FieldInfoData {
                name: "MaxInactiveVariableFps",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameTimeSettings, max_inactive_variable_fps),
            },
            FieldInfoData {
                name: "ForceDeltaTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameTimeSettings, force_delta_time),
            },
            FieldInfoData {
                name: "ForceUseSleepTimer",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameTimeSettings, force_use_sleep_timer),
            },
            FieldInfoData {
                name: "TimeScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameTimeSettings, time_scale),
            },
            FieldInfoData {
                name: "DebugFrameDelayMs",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(GameTimeSettings, debug_frame_delay_ms),
            },
            FieldInfoData {
                name: "DedicatedServerSleepInMsDuringLoading",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(GameTimeSettings, dedicated_server_sleep_in_ms_during_loading),
            },
        ],
    }),
    array_type: Some(GAMETIMESETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GameTimeSettings {
    fn type_info() -> &'static TypeInfo {
        GAMETIMESETTINGS_TYPE_INFO
    }
}


pub const GAMETIMESETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameTimeSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("GameTimeSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SyncedGameSettings {
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

pub const SYNCEDGAMESETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SyncedGameSettings",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DisableToggleEntryCamera",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SyncedGameSettings, disable_toggle_entry_camera),
            },
            FieldInfoData {
                name: "DisableRegenerateHealth",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SyncedGameSettings, disable_regenerate_health),
            },
            FieldInfoData {
                name: "EnableFriendlyFire",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SyncedGameSettings, enable_friendly_fire),
            },
            FieldInfoData {
                name: "AllowClientSideDamageArbitration",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SyncedGameSettings, allow_client_side_damage_arbitration),
            },
            FieldInfoData {
                name: "DifficultyIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SyncedGameSettings, difficulty_index),
            },
            FieldInfoData {
                name: "BulletDamageModifier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SyncedGameSettings, bullet_damage_modifier),
            },
            FieldInfoData {
                name: "MaxAllowedLatency",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SyncedGameSettings, max_allowed_latency),
            },
            FieldInfoData {
                name: "FrameHistoryTimeMax",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SyncedGameSettings, frame_history_time_max),
            },
            FieldInfoData {
                name: "FrameHistoryTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SyncedGameSettings, frame_history_time),
            },
            FieldInfoData {
                name: "MoveManagerOutgoingFrequencyDivider",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SyncedGameSettings, move_manager_outgoing_frequency_divider),
            },
            FieldInfoData {
                name: "MoveManagerSinglePlayerOutgoingFrequencyDivider",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SyncedGameSettings, move_manager_single_player_outgoing_frequency_divider),
            },
            FieldInfoData {
                name: "MaxCorrectionUpdateCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SyncedGameSettings, max_correction_update_count),
            },
            FieldInfoData {
                name: "EnableAutomaticCorrectionUpdateCount",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SyncedGameSettings, enable_automatic_correction_update_count),
            },
        ],
    }),
    array_type: Some(SYNCEDGAMESETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SyncedGameSettings {
    fn type_info() -> &'static TypeInfo {
        SYNCEDGAMESETTINGS_TYPE_INFO
    }
}


pub const SYNCEDGAMESETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SyncedGameSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("SyncedGameSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct GameSettings {
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
    pub game_mode_view_definitions: Vec<GameModeViewDefinition>,
    pub spawn_max_local_players_on_startup: bool,
    pub default_team_id: TeamId,
    pub version: VersionData,
    pub layer_inclusion_table: super::entity::SubWorldInclusion,
    pub default_layer_inclusion: String,
    pub time_to_wait_for_quit_task_completion: f32,
    pub player: PlayerData,
    pub difficulty_index: i32,
    pub game_settings_components: Vec<GameSettingsComponent>,
}

pub const GAMESETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameSettings",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MaxPlayerCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(GameSettings, max_player_count),
            },
            FieldInfoData {
                name: "MaxSpectatorCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(GameSettings, max_spectator_count),
            },
            FieldInfoData {
                name: "LogFileEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameSettings, log_file_enable),
            },
            FieldInfoData {
                name: "LogFileCollisionMode",
                flags: MemberInfoFlags::new(0),
                field_type: LOGFILECOLLISIONMODE_TYPE_INFO,
                rust_offset: offset_of!(GameSettings, log_file_collision_mode),
            },
            FieldInfoData {
                name: "LogFileRotationHistoryLength",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(GameSettings, log_file_rotation_history_length),
            },
            FieldInfoData {
                name: "Level",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(GameSettings, level),
            },
            FieldInfoData {
                name: "StartPoint",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(GameSettings, start_point),
            },
            FieldInfoData {
                name: "InstallationLevel",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(GameSettings, installation_level),
            },
            FieldInfoData {
                name: "InstallationStartPoint",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(GameSettings, installation_start_point),
            },
            FieldInfoData {
                name: "InstallationDefaultLayerInclusion",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(GameSettings, installation_default_layer_inclusion),
            },
            FieldInfoData {
                name: "ResourceRefreshAlwaysAllowed",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameSettings, resource_refresh_always_allowed),
            },
            FieldInfoData {
                name: "ActiveGameModeViewDefinition",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(GameSettings, active_game_mode_view_definition),
            },
            FieldInfoData {
                name: "GameModeViewDefinitions",
                flags: MemberInfoFlags::new(144),
                field_type: GAMEMODEVIEWDEFINITION_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(GameSettings, game_mode_view_definitions),
            },
            FieldInfoData {
                name: "SpawnMaxLocalPlayersOnStartup",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameSettings, spawn_max_local_players_on_startup),
            },
            FieldInfoData {
                name: "DefaultTeamId",
                flags: MemberInfoFlags::new(0),
                field_type: TEAMID_TYPE_INFO,
                rust_offset: offset_of!(GameSettings, default_team_id),
            },
            FieldInfoData {
                name: "Version",
                flags: MemberInfoFlags::new(0),
                field_type: VERSIONDATA_TYPE_INFO,
                rust_offset: offset_of!(GameSettings, version),
            },
            FieldInfoData {
                name: "LayerInclusionTable",
                flags: MemberInfoFlags::new(0),
                field_type: SUBWORLDINCLUSION_TYPE_INFO,
                rust_offset: offset_of!(GameSettings, layer_inclusion_table),
            },
            FieldInfoData {
                name: "DefaultLayerInclusion",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(GameSettings, default_layer_inclusion),
            },
            FieldInfoData {
                name: "TimeToWaitForQuitTaskCompletion",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameSettings, time_to_wait_for_quit_task_completion),
            },
            FieldInfoData {
                name: "Player",
                flags: MemberInfoFlags::new(0),
                field_type: PLAYERDATA_TYPE_INFO,
                rust_offset: offset_of!(GameSettings, player),
            },
            FieldInfoData {
                name: "DifficultyIndex",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(GameSettings, difficulty_index),
            },
            FieldInfoData {
                name: "GameSettingsComponents",
                flags: MemberInfoFlags::new(144),
                field_type: GAMESETTINGSCOMPONENT_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(GameSettings, game_settings_components),
            },
        ],
    }),
    array_type: Some(GAMESETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GameSettings {
    fn type_info() -> &'static TypeInfo {
        GAMESETTINGS_TYPE_INFO
    }
}


pub const GAMESETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("GameSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GameSettingsComponent {
}

pub const GAMESETTINGSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameSettingsComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(GAMESETTINGSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GameSettingsComponent {
    fn type_info() -> &'static TypeInfo {
        GAMESETTINGSCOMPONENT_TYPE_INFO
    }
}


pub const GAMESETTINGSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameSettingsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("GameSettingsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum LogFileCollisionMode {
    #[default]
    LFCM_Overwrite = 0,
    LFCM_Rotate = 1,
    LFCM_TimeStamp = 2,
}

pub const LOGFILECOLLISIONMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogFileCollisionMode",
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(LOGFILECOLLISIONMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LogFileCollisionMode {
    fn type_info() -> &'static TypeInfo {
        LOGFILECOLLISIONMODE_TYPE_INFO
    }
}


pub const LOGFILECOLLISIONMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogFileCollisionMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("LogFileCollisionMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CoreDebugReadProfileGameDataMessage {
}

pub const COREDEBUGREADPROFILEGAMEDATAMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreDebugReadProfileGameDataMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreDebugReadProfileGameDataMessage {
    fn type_info() -> &'static TypeInfo {
        COREDEBUGREADPROFILEGAMEDATAMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CoreReadSaveGameDataDoneMessage {
}

pub const COREREADSAVEGAMEDATADONEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreReadSaveGameDataDoneMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreReadSaveGameDataDoneMessage {
    fn type_info() -> &'static TypeInfo {
        COREREADSAVEGAMEDATADONEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CoreReadSaveGameDoneMessage {
}

pub const COREREADSAVEGAMEDONEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreReadSaveGameDoneMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreReadSaveGameDoneMessage {
    fn type_info() -> &'static TypeInfo {
        COREREADSAVEGAMEDONEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CoreReadSaveGameDataMessage {
}

pub const COREREADSAVEGAMEDATAMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreReadSaveGameDataMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreReadSaveGameDataMessage {
    fn type_info() -> &'static TypeInfo {
        COREREADSAVEGAMEDATAMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CoreWriteSaveGameDoneMessage {
}

pub const COREWRITESAVEGAMEDONEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreWriteSaveGameDoneMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreWriteSaveGameDoneMessage {
    fn type_info() -> &'static TypeInfo {
        COREWRITESAVEGAMEDONEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CoreWriteSaveGameMessage {
}

pub const COREWRITESAVEGAMEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreWriteSaveGameMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreWriteSaveGameMessage {
    fn type_info() -> &'static TypeInfo {
        COREWRITESAVEGAMEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CoreWriteProfileGameMessage {
}

pub const COREWRITEPROFILEGAMEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreWriteProfileGameMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreWriteProfileGameMessage {
    fn type_info() -> &'static TypeInfo {
        COREWRITEPROFILEGAMEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SaveGameHeaderEntry {
    pub key: i32,
    pub value: String,
}

pub const SAVEGAMEHEADERENTRY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SaveGameHeaderEntry",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Key",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(SaveGameHeaderEntry, key),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(SaveGameHeaderEntry, value),
            },
        ],
    }),
    array_type: Some(SAVEGAMEHEADERENTRY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SaveGameHeaderEntry {
    fn type_info() -> &'static TypeInfo {
        SAVEGAMEHEADERENTRY_TYPE_INFO
    }
}


pub const SAVEGAMEHEADERENTRY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SaveGameHeaderEntry-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("SaveGameHeaderEntry-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CoreClientPreLoadCompleteMessage {
}

pub const CORECLIENTPRELOADCOMPLETEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreClientPreLoadCompleteMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreClientPreLoadCompleteMessage {
    fn type_info() -> &'static TypeInfo {
        CORECLIENTPRELOADCOMPLETEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CoreClientPreLoadMessage {
}

pub const CORECLIENTPRELOADMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreClientPreLoadMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreClientPreLoadMessage {
    fn type_info() -> &'static TypeInfo {
        CORECLIENTPRELOADMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CoreClientPreSaveCompleteMessage {
}

pub const CORECLIENTPRESAVECOMPLETEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreClientPreSaveCompleteMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreClientPreSaveCompleteMessage {
    fn type_info() -> &'static TypeInfo {
        CORECLIENTPRESAVECOMPLETEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CoreClientPreSaveMessage {
}

pub const CORECLIENTPRESAVEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreClientPreSaveMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreClientPreSaveMessage {
    fn type_info() -> &'static TypeInfo {
        CORECLIENTPRESAVEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LoadGameLoadRequestedMessage {
}

pub const LOADGAMELOADREQUESTEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LoadGameLoadRequestedMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for LoadGameLoadRequestedMessage {
    fn type_info() -> &'static TypeInfo {
        LOADGAMELOADREQUESTEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LoadRequest {
    pub filename: String,
    pub setup_params: super::entity::LevelSetup,
    pub load_from_juice: bool,
}

pub const LOADREQUEST_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LoadRequest",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Filename",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(LoadRequest, filename),
            },
            FieldInfoData {
                name: "SetupParams",
                flags: MemberInfoFlags::new(0),
                field_type: LEVELSETUP_TYPE_INFO,
                rust_offset: offset_of!(LoadRequest, setup_params),
            },
            FieldInfoData {
                name: "LoadFromJuice",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LoadRequest, load_from_juice),
            },
        ],
    }),
    array_type: Some(LOADREQUEST_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LoadRequest {
    fn type_info() -> &'static TypeInfo {
        LOADREQUEST_TYPE_INFO
    }
}


pub const LOADREQUEST_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LoadRequest-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("LoadRequest-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LoadGameBeginLoadMessage {
}

pub const LOADGAMEBEGINLOADMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LoadGameBeginLoadMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for LoadGameBeginLoadMessage {
    fn type_info() -> &'static TypeInfo {
        LOADGAMEBEGINLOADMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SaveGameBeginSaveMessage {
}

pub const SAVEGAMEBEGINSAVEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SaveGameBeginSaveMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for SaveGameBeginSaveMessage {
    fn type_info() -> &'static TypeInfo {
        SAVEGAMEBEGINSAVEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BlueprintBundleStreamedInMessage {
}

pub const BLUEPRINTBUNDLESTREAMEDINMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundleStreamedInMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for BlueprintBundleStreamedInMessage {
    fn type_info() -> &'static TypeInfo {
        BLUEPRINTBUNDLESTREAMEDINMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SessionPlayerJoinedMessage {
}

pub const SESSIONPLAYERJOINEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SessionPlayerJoinedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for SessionPlayerJoinedMessage {
    fn type_info() -> &'static TypeInfo {
        SESSIONPLAYERJOINEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SessionPlayerAuthenticatedMessage {
}

pub const SESSIONPLAYERAUTHENTICATEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SessionPlayerAuthenticatedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for SessionPlayerAuthenticatedMessage {
    fn type_info() -> &'static TypeInfo {
        SESSIONPLAYERAUTHENTICATEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SessionPlayerLeftMessage {
}

pub const SESSIONPLAYERLEFTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SessionPlayerLeftMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for SessionPlayerLeftMessage {
    fn type_info() -> &'static TypeInfo {
        SESSIONPLAYERLEFTMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CoreGameTimerMessage {
}

pub const COREGAMETIMERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreGameTimerMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreGameTimerMessage {
    fn type_info() -> &'static TypeInfo {
        COREGAMETIMERMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CoreExitIngameMessage {
}

pub const COREEXITINGAMEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreExitIngameMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreExitIngameMessage {
    fn type_info() -> &'static TypeInfo {
        COREEXITINGAMEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CoreEnteredIngameMessage {
}

pub const COREENTEREDINGAMEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreEnteredIngameMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreEnteredIngameMessage {
    fn type_info() -> &'static TypeInfo {
        COREENTEREDINGAMEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PerformanceClientNetworkMessage {
}

pub const PERFORMANCECLIENTNETWORKMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerformanceClientNetworkMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for PerformanceClientNetworkMessage {
    fn type_info() -> &'static TypeInfo {
        PERFORMANCECLIENTNETWORKMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PerformanceServerNetworkMessage {
}

pub const PERFORMANCESERVERNETWORKMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerformanceServerNetworkMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for PerformanceServerNetworkMessage {
    fn type_info() -> &'static TypeInfo {
        PERFORMANCESERVERNETWORKMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PerformanceServerMessage {
}

pub const PERFORMANCESERVERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerformanceServerMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for PerformanceServerMessage {
    fn type_info() -> &'static TypeInfo {
        PERFORMANCESERVERMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProfileOptionsSettingsSavedMessage {
}

pub const PROFILEOPTIONSSETTINGSSAVEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionsSettingsSavedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ProfileOptionsSettingsSavedMessage {
    fn type_info() -> &'static TypeInfo {
        PROFILEOPTIONSSETTINGSSAVEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProfileOptionsSettingsPreSaveMessage {
}

pub const PROFILEOPTIONSSETTINGSPRESAVEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionsSettingsPreSaveMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ProfileOptionsSettingsPreSaveMessage {
    fn type_info() -> &'static TypeInfo {
        PROFILEOPTIONSSETTINGSPRESAVEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProfileOptionsSettingsLoadedMessage {
}

pub const PROFILEOPTIONSSETTINGSLOADEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionsSettingsLoadedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ProfileOptionsSettingsLoadedMessage {
    fn type_info() -> &'static TypeInfo {
        PROFILEOPTIONSSETTINGSLOADEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProfileOptionsApplyMessage {
}

pub const PROFILEOPTIONSAPPLYMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionsApplyMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ProfileOptionsApplyMessage {
    fn type_info() -> &'static TypeInfo {
        PROFILEOPTIONSAPPLYMESSAGE_TYPE_INFO
    }
}

#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ProfileOptionsLoadStatus {
    #[default]
    ProfileOptionsLoadStatus_Empty = 0,
    ProfileOptionsLoadStatus_Succeeded = 1,
    ProfileOptionsLoadStatus_Corrupt = 2,
}

pub const PROFILEOPTIONSLOADSTATUS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionsLoadStatus",
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(PROFILEOPTIONSLOADSTATUS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ProfileOptionsLoadStatus {
    fn type_info() -> &'static TypeInfo {
        PROFILEOPTIONSLOADSTATUS_TYPE_INFO
    }
}


pub const PROFILEOPTIONSLOADSTATUS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionsLoadStatus-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ProfileOptionsLoadStatus-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StatisticsEventMessageBase {
}

pub const STATISTICSEVENTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StatisticsEventMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for StatisticsEventMessageBase {
    fn type_info() -> &'static TypeInfo {
        STATISTICSEVENTMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkDisconnectedMessage {
}

pub const NETWORKDISCONNECTEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkDisconnectedMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkDisconnectedMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKDISCONNECTEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkConnectedMessage {
}

pub const NETWORKCONNECTEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkConnectedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkConnectedMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKCONNECTEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TransformParameterEvent {
}

pub const TRANSFORMPARAMETEREVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformParameterEvent",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYEVENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TRANSFORMPARAMETEREVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TransformParameterEvent {
    fn type_info() -> &'static TypeInfo {
        TRANSFORMPARAMETEREVENT_TYPE_INFO
    }
}


pub const TRANSFORMPARAMETEREVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformParameterEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("TransformParameterEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PlayerEventBase {
}

pub const PLAYEREVENTBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayerEventBase",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYEVENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PLAYEREVENTBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PlayerEventBase {
    fn type_info() -> &'static TypeInfo {
        PLAYEREVENTBASE_TYPE_INFO
    }
}


pub const PLAYEREVENTBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayerEventBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("PlayerEventBase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IntParameterEvent {
}

pub const INTPARAMETEREVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntParameterEvent",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYEVENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(INTPARAMETEREVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IntParameterEvent {
    fn type_info() -> &'static TypeInfo {
        INTPARAMETEREVENT_TYPE_INFO
    }
}


pub const INTPARAMETEREVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntParameterEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("IntParameterEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FloatParameterEvent {
}

pub const FLOATPARAMETEREVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatParameterEvent",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYEVENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FLOATPARAMETEREVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FloatParameterEvent {
    fn type_info() -> &'static TypeInfo {
        FLOATPARAMETEREVENT_TYPE_INFO
    }
}


pub const FLOATPARAMETEREVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatParameterEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("FloatParameterEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EntityParameterEvent {
}

pub const ENTITYPARAMETEREVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityParameterEvent",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYEVENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ENTITYPARAMETEREVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EntityParameterEvent {
    fn type_info() -> &'static TypeInfo {
        ENTITYPARAMETEREVENT_TYPE_INFO
    }
}


pub const ENTITYPARAMETEREVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityParameterEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("EntityParameterEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ComponentParameterEvent {
}

pub const COMPONENTPARAMETEREVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ComponentParameterEvent",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYEVENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(COMPONENTPARAMETEREVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ComponentParameterEvent {
    fn type_info() -> &'static TypeInfo {
        COMPONENTPARAMETEREVENT_TYPE_INFO
    }
}


pub const COMPONENTPARAMETEREVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ComponentParameterEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ComponentParameterEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CollisionEvent {
}

pub const COLLISIONEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CollisionEvent",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYEVENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(COLLISIONEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CollisionEvent {
    fn type_info() -> &'static TypeInfo {
        COLLISIONEVENT_TYPE_INFO
    }
}


pub const COLLISIONEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CollisionEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("CollisionEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HealthEntityFactory {
}

pub const HEALTHENTITYFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthEntityFactory",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IGLOOENTITYFACTORY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(HEALTHENTITYFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for HealthEntityFactory {
    fn type_info() -> &'static TypeInfo {
        HEALTHENTITYFACTORY_TYPE_INFO
    }
}


pub const HEALTHENTITYFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthEntityFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("HealthEntityFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HealthComponent {
}

pub const HEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(HEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for HealthComponent {
    fn type_info() -> &'static TypeInfo {
        HEALTHCOMPONENT_TYPE_INFO
    }
}


pub const HEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("HealthComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientDamageInfo {
}

pub const CLIENTDAMAGEINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDamageInfo",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DAMAGEINFO_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTDAMAGEINFO_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientDamageInfo {
    fn type_info() -> &'static TypeInfo {
        CLIENTDAMAGEINFO_TYPE_INFO
    }
}


pub const CLIENTDAMAGEINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDamageInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ClientDamageInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerDamageInfo {
}

pub const SERVERDAMAGEINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDamageInfo",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DAMAGEINFO_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERDAMAGEINFO_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerDamageInfo {
    fn type_info() -> &'static TypeInfo {
        SERVERDAMAGEINFO_TYPE_INFO
    }
}


pub const SERVERDAMAGEINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDamageInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ServerDamageInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DamageInfo {
}

pub const DAMAGEINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DamageInfo",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(DAMAGEINFO_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DamageInfo {
    fn type_info() -> &'static TypeInfo {
        DAMAGEINFO_TYPE_INFO
    }
}


pub const DAMAGEINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DamageInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("DamageInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DefaultHealthComponent {
}

pub const DEFAULTHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DefaultHealthComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(HEALTHCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DEFAULTHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DefaultHealthComponent {
    fn type_info() -> &'static TypeInfo {
        DEFAULTHEALTHCOMPONENT_TYPE_INFO
    }
}


pub const DEFAULTHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DefaultHealthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("DefaultHealthComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GameWorldRayCaster {
}

pub const GAMEWORLDRAYCASTER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameWorldRayCaster",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(GAMEWORLDRAYCASTER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for GameWorldRayCaster {
    fn type_info() -> &'static TypeInfo {
        GAMEWORLDRAYCASTER_TYPE_INFO
    }
}


pub const GAMEWORLDRAYCASTER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameWorldRayCaster-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("GameWorldRayCaster-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GameComponentEntity {
}

pub const GAMECOMPONENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameComponentEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENTENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(GAMECOMPONENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for GameComponentEntity {
    fn type_info() -> &'static TypeInfo {
        GAMECOMPONENTENTITY_TYPE_INFO
    }
}


pub const GAMECOMPONENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameComponentEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("GameComponentEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GameComponent {
}

pub const GAMECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(GAMECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for GameComponent {
    fn type_info() -> &'static TypeInfo {
        GAMECOMPONENT_TYPE_INFO
    }
}


pub const GAMECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("GameComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PartComponentCreatedEntityInfo {
}

pub const PARTCOMPONENTCREATEDENTITYINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartComponentCreatedEntityInfo",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(PARTCOMPONENTCREATEDENTITYINFO_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PartComponentCreatedEntityInfo {
    fn type_info() -> &'static TypeInfo {
        PARTCOMPONENTCREATEDENTITYINFO_TYPE_INFO
    }
}


pub const PARTCOMPONENTCREATEDENTITYINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartComponentCreatedEntityInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("PartComponentCreatedEntityInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SubView {
}

pub const SUBVIEW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubView",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(SUBVIEW_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SubView {
    fn type_info() -> &'static TypeInfo {
        SUBVIEW_TYPE_INFO
    }
}


pub const SUBVIEW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubView-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("SubView-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShaderParameterEntity {
}

pub const SHADERPARAMETERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SHADERPARAMETERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ShaderParameterEntity {
    fn type_info() -> &'static TypeInfo {
        SHADERPARAMETERENTITY_TYPE_INFO
    }
}


pub const SHADERPARAMETERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ShaderParameterEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkServerDiagnosticsMessage {
}

pub const NETWORKSERVERDIAGNOSTICSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkServerDiagnosticsMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkServerDiagnosticsMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKSERVERDIAGNOSTICSMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkCameraReplayMessage {
}

pub const NETWORKCAMERAREPLAYMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkCameraReplayMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkCameraReplayMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKCAMERAREPLAYMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkCameraFreeCameraMessage {
}

pub const NETWORKCAMERAFREECAMERAMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkCameraFreeCameraMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for NetworkCameraFreeCameraMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKCAMERAFREECAMERAMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkSynchronizeInternetSimulationStateMessage {
}

pub const NETWORKSYNCHRONIZEINTERNETSIMULATIONSTATEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkSynchronizeInternetSimulationStateMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkSynchronizeInternetSimulationStateMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKSYNCHRONIZEINTERNETSIMULATIONSTATEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkChangeGameSettingMessage {
}

pub const NETWORKCHANGEGAMESETTINGMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkChangeGameSettingMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkChangeGameSettingMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKCHANGEGAMESETTINGMESSAGE_TYPE_INFO
    }
}

#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ChangeGameSettingType {
    #[default]
    CGSNone = 0,
    CGSInvertVerticalLookAxis = 1,
    CGSCount = 2,
}

pub const CHANGEGAMESETTINGTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ChangeGameSettingType",
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(CHANGEGAMESETTINGTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ChangeGameSettingType {
    fn type_info() -> &'static TypeInfo {
        CHANGEGAMESETTINGTYPE_TYPE_INFO
    }
}


pub const CHANGEGAMESETTINGTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ChangeGameSettingType-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ChangeGameSettingType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkDestroyLocalPlayerMessage {
}

pub const NETWORKDESTROYLOCALPLAYERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkDestroyLocalPlayerMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkDestroyLocalPlayerMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKDESTROYLOCALPLAYERMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkCreatePlayerMessage {
}

pub const NETWORKCREATEPLAYERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkCreatePlayerMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkCreatePlayerMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKCREATEPLAYERMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkTinyEventMessage {
}

pub const NETWORKTINYEVENTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkTinyEventMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkTinyEventMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKTINYEVENTMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkLoadLevelMessage {
}

pub const NETWORKLOADLEVELMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkLoadLevelMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkLoadLevelMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKLOADLEVELMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkLevelLoadedAckMessage {
}

pub const NETWORKLEVELLOADEDACKMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkLevelLoadedAckMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkLevelLoadedAckMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKLEVELLOADEDACKMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkTimeSyncMessage {
}

pub const NETWORKTIMESYNCMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkTimeSyncMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkTimeSyncMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKTIMESYNCMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LoadLevelInfo {
    pub setup: super::entity::LevelSetup,
    pub blueprint_bundle_preloads: Vec<BlueprintBundlePreloadInfo>,
    pub level_sequence_number: u32,
}

pub const LOADLEVELINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LoadLevelInfo",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Setup",
                flags: MemberInfoFlags::new(0),
                field_type: LEVELSETUP_TYPE_INFO,
                rust_offset: offset_of!(LoadLevelInfo, setup),
            },
            FieldInfoData {
                name: "BlueprintBundlePreloads",
                flags: MemberInfoFlags::new(144),
                field_type: BLUEPRINTBUNDLEPRELOADINFO_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LoadLevelInfo, blueprint_bundle_preloads),
            },
            FieldInfoData {
                name: "LevelSequenceNumber",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(LoadLevelInfo, level_sequence_number),
            },
        ],
    }),
    array_type: Some(LOADLEVELINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LoadLevelInfo {
    fn type_info() -> &'static TypeInfo {
        LOADLEVELINFO_TYPE_INFO
    }
}


pub const LOADLEVELINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LoadLevelInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("LoadLevelInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BlueprintBundlePreloadInfo {
    pub name: String,
    pub compartment: i32,
    pub parent_compartment: i32,
}

pub const BLUEPRINTBUNDLEPRELOADINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundlePreloadInfo",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(BlueprintBundlePreloadInfo, name),
            },
            FieldInfoData {
                name: "Compartment",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(BlueprintBundlePreloadInfo, compartment),
            },
            FieldInfoData {
                name: "ParentCompartment",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(BlueprintBundlePreloadInfo, parent_compartment),
            },
        ],
    }),
    array_type: Some(BLUEPRINTBUNDLEPRELOADINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BlueprintBundlePreloadInfo {
    fn type_info() -> &'static TypeInfo {
        BLUEPRINTBUNDLEPRELOADINFO_TYPE_INFO
    }
}


pub const BLUEPRINTBUNDLEPRELOADINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundlePreloadInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("BlueprintBundlePreloadInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
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

pub const TINYEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TinyEvent",
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(TINYEVENT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TinyEvent {
    fn type_info() -> &'static TypeInfo {
        TINYEVENT_TYPE_INFO
    }
}


pub const TINYEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TinyEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("TinyEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientToServerLocalPlayer {
}

pub const CLIENTTOSERVERLOCALPLAYER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientToServerLocalPlayer",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(CLIENTTOSERVERLOCALPLAYER_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ClientToServerLocalPlayer {
    fn type_info() -> &'static TypeInfo {
        CLIENTTOSERVERLOCALPLAYER_TYPE_INFO
    }
}


pub const CLIENTTOSERVERLOCALPLAYER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientToServerLocalPlayer-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ClientToServerLocalPlayer-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ClientToServerLocalPlayerView {
    pub is_active: bool,
    pub camera_pos: super::core::Vec3,
    pub camera_forward: super::core::Vec3,
    pub camera_fov: u32,
}

pub const CLIENTTOSERVERLOCALPLAYERVIEW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientToServerLocalPlayerView",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "IsActive",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClientToServerLocalPlayerView, is_active),
            },
            FieldInfoData {
                name: "CameraPos",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ClientToServerLocalPlayerView, camera_pos),
            },
            FieldInfoData {
                name: "CameraForward",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ClientToServerLocalPlayerView, camera_forward),
            },
            FieldInfoData {
                name: "CameraFov",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ClientToServerLocalPlayerView, camera_fov),
            },
        ],
    }),
    array_type: Some(CLIENTTOSERVERLOCALPLAYERVIEW_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ClientToServerLocalPlayerView {
    fn type_info() -> &'static TypeInfo {
        CLIENTTOSERVERLOCALPLAYERVIEW_TYPE_INFO
    }
}


pub const CLIENTTOSERVERLOCALPLAYERVIEW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientToServerLocalPlayerView-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ClientToServerLocalPlayerView-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ClientToServerConstants {
    #[default]
    ClientToServerConstants_LocalPlayerCount = 8,
    ClientToServerConstants_LocalPlayerViewCount = 6,
}

pub const CLIENTTOSERVERCONSTANTS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientToServerConstants",
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(CLIENTTOSERVERCONSTANTS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ClientToServerConstants {
    fn type_info() -> &'static TypeInfo {
        CLIENTTOSERVERCONSTANTS_TYPE_INFO
    }
}


pub const CLIENTTOSERVERCONSTANTS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientToServerConstants-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ClientToServerConstants-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct InputModifierEntityData {
    pub realm: super::core::Realm,
    pub enabled: bool,
    pub action: i32,
    pub offset: f32,
    pub scale: f32,
}

pub const INPUTMODIFIERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputModifierEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(InputModifierEntityData, realm),
            },
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(InputModifierEntityData, enabled),
            },
            FieldInfoData {
                name: "Action",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(InputModifierEntityData, action),
            },
            FieldInfoData {
                name: "Offset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(InputModifierEntityData, offset),
            },
            FieldInfoData {
                name: "Scale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(InputModifierEntityData, scale),
            },
        ],
    }),
    array_type: Some(INPUTMODIFIERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for InputModifierEntityData {
    fn type_info() -> &'static TypeInfo {
        INPUTMODIFIERENTITYDATA_TYPE_INFO
    }
}


pub const INPUTMODIFIERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputModifierEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("InputModifierEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct GameplaySpawnReferenceObjectData {
    pub extra_spawn_data: Vec<ExtraSpawnData>,
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

pub const GAMEPLAYSPAWNREFERENCEOBJECTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameplaySpawnReferenceObjectData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALREFERENCEOBJECTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ExtraSpawnData",
                flags: MemberInfoFlags::new(144),
                field_type: EXTRASPAWNDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(GameplaySpawnReferenceObjectData, extra_spawn_data),
            },
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameplaySpawnReferenceObjectData, enabled),
            },
            FieldInfoData {
                name: "LocationNameSid",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(GameplaySpawnReferenceObjectData, location_name_sid),
            },
            FieldInfoData {
                name: "LocationTextSid",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(GameplaySpawnReferenceObjectData, location_text_sid),
            },
            FieldInfoData {
                name: "InitialAutoSpawn",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameplaySpawnReferenceObjectData, initial_auto_spawn),
            },
            FieldInfoData {
                name: "AutoSpawn",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameplaySpawnReferenceObjectData, auto_spawn),
            },
            FieldInfoData {
                name: "QueueSpawnEvent",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameplaySpawnReferenceObjectData, queue_spawn_event),
            },
            FieldInfoData {
                name: "UseAsSpawnPoint",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameplaySpawnReferenceObjectData, use_as_spawn_point),
            },
            FieldInfoData {
                name: "InitialSpawnDelay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameplaySpawnReferenceObjectData, initial_spawn_delay),
            },
            FieldInfoData {
                name: "SpawnDelay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameplaySpawnReferenceObjectData, spawn_delay),
            },
            FieldInfoData {
                name: "MaxCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(GameplaySpawnReferenceObjectData, max_count),
            },
            FieldInfoData {
                name: "MaxCountSimultaneously",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(GameplaySpawnReferenceObjectData, max_count_simultaneously),
            },
            FieldInfoData {
                name: "TotalCountSimultaneouslyOfType",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(GameplaySpawnReferenceObjectData, total_count_simultaneously_of_type),
            },
            FieldInfoData {
                name: "MaxSpawnInFrame",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(GameplaySpawnReferenceObjectData, max_spawn_in_frame),
            },
            FieldInfoData {
                name: "OnlySendEventForHumanPlayers",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameplaySpawnReferenceObjectData, only_send_event_for_human_players),
            },
            FieldInfoData {
                name: "ControllableTransform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(GameplaySpawnReferenceObjectData, controllable_transform),
            },
            FieldInfoData {
                name: "ControllableInput",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(GameplaySpawnReferenceObjectData, controllable_input),
            },
        ],
    }),
    array_type: Some(GAMEPLAYSPAWNREFERENCEOBJECTDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for GameplaySpawnReferenceObjectData {
    fn type_info() -> &'static TypeInfo {
        GAMEPLAYSPAWNREFERENCEOBJECTDATA_TYPE_INFO
    }
}


pub const GAMEPLAYSPAWNREFERENCEOBJECTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameplaySpawnReferenceObjectData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("GameplaySpawnReferenceObjectData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ExtraSpawnData {
}

pub const EXTRASPAWNDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExtraSpawnData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(EXTRASPAWNDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ExtraSpawnData {
    fn type_info() -> &'static TypeInfo {
        EXTRASPAWNDATA_TYPE_INFO
    }
}


pub const EXTRASPAWNDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExtraSpawnData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ExtraSpawnData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GameplayTeamEntityData {
    pub team: GameplayTeamData,
    pub id: TeamId,
}

pub const GAMEPLAYTEAMENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameplayTeamEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Team",
                flags: MemberInfoFlags::new(0),
                field_type: GAMEPLAYTEAMDATA_TYPE_INFO,
                rust_offset: offset_of!(GameplayTeamEntityData, team),
            },
            FieldInfoData {
                name: "Id",
                flags: MemberInfoFlags::new(0),
                field_type: TEAMID_TYPE_INFO,
                rust_offset: offset_of!(GameplayTeamEntityData, id),
            },
        ],
    }),
    array_type: Some(GAMEPLAYTEAMENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for GameplayTeamEntityData {
    fn type_info() -> &'static TypeInfo {
        GAMEPLAYTEAMENTITYDATA_TYPE_INFO
    }
}


pub const GAMEPLAYTEAMENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameplayTeamEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("GameplayTeamEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GameplayTeamData {
    pub faction: FactionId,
}

pub const GAMEPLAYTEAMDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameplayTeamData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINERPOLICYASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Faction",
                flags: MemberInfoFlags::new(0),
                field_type: FACTIONID_TYPE_INFO,
                rust_offset: offset_of!(GameplayTeamData, faction),
            },
        ],
    }),
    array_type: Some(GAMEPLAYTEAMDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for GameplayTeamData {
    fn type_info() -> &'static TypeInfo {
        GAMEPLAYTEAMDATA_TYPE_INFO
    }
}


pub const GAMEPLAYTEAMDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameplayTeamData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("GameplayTeamData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SubLevelCollectionEntityData {
    pub stream_realm: super::entity::StreamRealm,
    pub active_index: i32,
    pub visible: bool,
}

pub const SUBLEVELCOLLECTIONENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelCollectionEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DETACHABLESUBWORLDCOLLECTIONBASEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "StreamRealm",
                flags: MemberInfoFlags::new(0),
                field_type: STREAMREALM_TYPE_INFO,
                rust_offset: offset_of!(SubLevelCollectionEntityData, stream_realm),
            },
            FieldInfoData {
                name: "ActiveIndex",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(SubLevelCollectionEntityData, active_index),
            },
            FieldInfoData {
                name: "Visible",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SubLevelCollectionEntityData, visible),
            },
        ],
    }),
    array_type: Some(SUBLEVELCOLLECTIONENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SubLevelCollectionEntityData {
    fn type_info() -> &'static TypeInfo {
        SUBLEVELCOLLECTIONENTITYDATA_TYPE_INFO
    }
}


pub const SUBLEVELCOLLECTIONENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelCollectionEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("SubLevelCollectionEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DetachableSubWorldCollectionBaseData {
    pub sub_level_refs: Vec<super::entity::SubWorldReferenceObjectData>,
    pub object_layers: i32,
}

pub const DETACHABLESUBWORLDCOLLECTIONBASEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DetachableSubWorldCollectionBaseData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "SubLevelRefs",
                flags: MemberInfoFlags::new(144),
                field_type: SUBWORLDREFERENCEOBJECTDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(DetachableSubWorldCollectionBaseData, sub_level_refs),
            },
            FieldInfoData {
                name: "ObjectLayers",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(DetachableSubWorldCollectionBaseData, object_layers),
            },
        ],
    }),
    array_type: Some(DETACHABLESUBWORLDCOLLECTIONBASEDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DetachableSubWorldCollectionBaseData {
    fn type_info() -> &'static TypeInfo {
        DETACHABLESUBWORLDCOLLECTIONBASEDATA_TYPE_INFO
    }
}


pub const DETACHABLESUBWORLDCOLLECTIONBASEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DetachableSubWorldCollectionBaseData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("DetachableSubWorldCollectionBaseData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DemoClientReadyForPlaybackMessage {
}

pub const DEMOCLIENTREADYFORPLAYBACKMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DemoClientReadyForPlaybackMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for DemoClientReadyForPlaybackMessage {
    fn type_info() -> &'static TypeInfo {
        DEMOCLIENTREADYFORPLAYBACKMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkLogicFireDoublePlayerEventMessageBase {
}

pub const NETWORKLOGICFIREDOUBLEPLAYEREVENTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkLogicFireDoublePlayerEventMessageBase",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for NetworkLogicFireDoublePlayerEventMessageBase {
    fn type_info() -> &'static TypeInfo {
        NETWORKLOGICFIREDOUBLEPLAYEREVENTMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkLogicFirePlayerEventMessageBase {
}

pub const NETWORKLOGICFIREPLAYEREVENTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkLogicFirePlayerEventMessageBase",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for NetworkLogicFirePlayerEventMessageBase {
    fn type_info() -> &'static TypeInfo {
        NETWORKLOGICFIREPLAYEREVENTMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkLogicFireEventMessageBase {
}

pub const NETWORKLOGICFIREEVENTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkLogicFireEventMessageBase",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for NetworkLogicFireEventMessageBase {
    fn type_info() -> &'static TypeInfo {
        NETWORKLOGICFIREEVENTMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SubLevelFromClientSubLevelUnloadedMessage {
}

pub const SUBLEVELFROMCLIENTSUBLEVELUNLOADEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelFromClientSubLevelUnloadedMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for SubLevelFromClientSubLevelUnloadedMessage {
    fn type_info() -> &'static TypeInfo {
        SUBLEVELFROMCLIENTSUBLEVELUNLOADEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SubLevelFromClientSubLevelLoadedMessage {
}

pub const SUBLEVELFROMCLIENTSUBLEVELLOADEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelFromClientSubLevelLoadedMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for SubLevelFromClientSubLevelLoadedMessage {
    fn type_info() -> &'static TypeInfo {
        SUBLEVELFROMCLIENTSUBLEVELLOADEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SubLevelFromClientRequestBundleBaselineMessage {
}

pub const SUBLEVELFROMCLIENTREQUESTBUNDLEBASELINEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelFromClientRequestBundleBaselineMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for SubLevelFromClientRequestBundleBaselineMessage {
    fn type_info() -> &'static TypeInfo {
        SUBLEVELFROMCLIENTREQUESTBUNDLEBASELINEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SubLevelToClientDropBundleBaselineMessage {
}

pub const SUBLEVELTOCLIENTDROPBUNDLEBASELINEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelToClientDropBundleBaselineMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for SubLevelToClientDropBundleBaselineMessage {
    fn type_info() -> &'static TypeInfo {
        SUBLEVELTOCLIENTDROPBUNDLEBASELINEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SubLevelToClientUnloadRequestsMessage {
}

pub const SUBLEVELTOCLIENTUNLOADREQUESTSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelToClientUnloadRequestsMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for SubLevelToClientUnloadRequestsMessage {
    fn type_info() -> &'static TypeInfo {
        SUBLEVELTOCLIENTUNLOADREQUESTSMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SubLevelToClientLoadRequestsMessage {
}

pub const SUBLEVELTOCLIENTLOADREQUESTSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelToClientLoadRequestsMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for SubLevelToClientLoadRequestsMessage {
    fn type_info() -> &'static TypeInfo {
        SUBLEVELTOCLIENTLOADREQUESTSMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
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

pub const SUBLEVELBUNDLEINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelBundleInfo",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "SubLevelNameInx",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(SubLevelBundleInfo, sub_level_name_inx),
            },
            FieldInfoData {
                name: "SubLevelId",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(SubLevelBundleInfo, sub_level_id),
            },
            FieldInfoData {
                name: "ParentSubLevelId",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(SubLevelBundleInfo, parent_sub_level_id),
            },
            FieldInfoData {
                name: "OwnerUniqueId",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SubLevelBundleInfo, owner_unique_id),
            },
            FieldInfoData {
                name: "CompartmentIndex",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(SubLevelBundleInfo, compartment_index),
            },
            FieldInfoData {
                name: "Priority",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(SubLevelBundleInfo, priority),
            },
            FieldInfoData {
                name: "IsPeerFiltered",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SubLevelBundleInfo, is_peer_filtered),
            },
            FieldInfoData {
                name: "HeapInfo",
                flags: MemberInfoFlags::new(0),
                field_type: BUNDLEHEAPINFO_TYPE_INFO,
                rust_offset: offset_of!(SubLevelBundleInfo, heap_info),
            },
            FieldInfoData {
                name: "BundleType",
                flags: MemberInfoFlags::new(0),
                field_type: BUNDLETYPE_TYPE_INFO,
                rust_offset: offset_of!(SubLevelBundleInfo, bundle_type),
            },
            FieldInfoData {
                name: "BundleSettingsInfo",
                flags: MemberInfoFlags::new(0),
                field_type: BUNDLESETTINGSINFO_TYPE_INFO,
                rust_offset: offset_of!(SubLevelBundleInfo, bundle_settings_info),
            },
        ],
    }),
    array_type: Some(SUBLEVELBUNDLEINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SubLevelBundleInfo {
    fn type_info() -> &'static TypeInfo {
        SUBLEVELBUNDLEINFO_TYPE_INFO
    }
}


pub const SUBLEVELBUNDLEINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelBundleInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("SubLevelBundleInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SubLevelToClientSubLevelNameMessage {
}

pub const SUBLEVELTOCLIENTSUBLEVELNAMEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelToClientSubLevelNameMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for SubLevelToClientSubLevelNameMessage {
    fn type_info() -> &'static TypeInfo {
        SUBLEVELTOCLIENTSUBLEVELNAMEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BundleNameAndIndex {
    pub index: u16,
    pub name: String,
}

pub const BUNDLENAMEANDINDEX_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BundleNameAndIndex",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Index",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(BundleNameAndIndex, index),
            },
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(BundleNameAndIndex, name),
            },
        ],
    }),
    array_type: Some(BUNDLENAMEANDINDEX_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BundleNameAndIndex {
    fn type_info() -> &'static TypeInfo {
        BUNDLENAMEANDINDEX_TYPE_INFO
    }
}


pub const BUNDLENAMEANDINDEX_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BundleNameAndIndex-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("BundleNameAndIndex-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ControllableToComponentsOnTeleportedMessage {
}

pub const CONTROLLABLETOCOMPONENTSONTELEPORTEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ControllableToComponentsOnTeleportedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ControllableToComponentsOnTeleportedMessage {
    fn type_info() -> &'static TypeInfo {
        CONTROLLABLETOCOMPONENTSONTELEPORTEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerControllableToComponentsPlayerExitMessage {
}

pub const SERVERCONTROLLABLETOCOMPONENTSPLAYEREXITMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerControllableToComponentsPlayerExitMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerControllableToComponentsPlayerExitMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERCONTROLLABLETOCOMPONENTSPLAYEREXITMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerControllableToComponentsPlayerEnteredMessage {
}

pub const SERVERCONTROLLABLETOCOMPONENTSPLAYERENTEREDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerControllableToComponentsPlayerEnteredMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerControllableToComponentsPlayerEnteredMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERCONTROLLABLETOCOMPONENTSPLAYERENTEREDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientControllableToComponentsPlayerExitMessage {
}

pub const CLIENTCONTROLLABLETOCOMPONENTSPLAYEREXITMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientControllableToComponentsPlayerExitMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientControllableToComponentsPlayerExitMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTCONTROLLABLETOCOMPONENTSPLAYEREXITMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientControllableToComponentsPlayerEnteredMessage {
}

pub const CLIENTCONTROLLABLETOCOMPONENTSPLAYERENTEREDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientControllableToComponentsPlayerEnteredMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientControllableToComponentsPlayerEnteredMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTCONTROLLABLETOCOMPONENTSPLAYERENTEREDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ControllableEntityData {
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

pub const CONTROLLABLEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ControllableEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMEPHYSICSENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "UsePrediction",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ControllableEntityData, use_prediction),
            },
            FieldInfoData {
                name: "DefaultTeam",
                flags: MemberInfoFlags::new(0),
                field_type: TEAMID_TYPE_INFO,
                rust_offset: offset_of!(ControllableEntityData, default_team),
            },
            FieldInfoData {
                name: "ResetTeamOnLastPlayerExits",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ControllableEntityData, reset_team_on_last_player_exits),
            },
            FieldInfoData {
                name: "Immortal",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ControllableEntityData, immortal),
            },
            FieldInfoData {
                name: "FakeImmortal",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ControllableEntityData, fake_immortal),
            },
            FieldInfoData {
                name: "LowHealthThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ControllableEntityData, low_health_threshold),
            },
            FieldInfoData {
                name: "MaterialPair",
                flags: MemberInfoFlags::new(0),
                field_type: MATERIALDECL_TYPE_INFO,
                rust_offset: offset_of!(ControllableEntityData, material_pair),
            },
            FieldInfoData {
                name: "ForceForegroundRendering",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ControllableEntityData, force_foreground_rendering),
            },
            FieldInfoData {
                name: "SuppressedInputs",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ControllableEntityData, suppressed_inputs),
            },
        ],
    }),
    array_type: Some(CONTROLLABLEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ControllableEntityData {
    fn type_info() -> &'static TypeInfo {
        CONTROLLABLEENTITYDATA_TYPE_INFO
    }
}


pub const CONTROLLABLEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ControllableEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("ControllableEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EntryComponentData {
    pub entry_class: EntryClass,
    pub a_i_data: GameAIEntryData,
    pub forbidden_for_human: bool,
    pub input_graph: super::input_shared::InputGraph,
    pub input_concept_definition: super::input_shared::InputActionMapsData,
    pub action_map_settings_flip_y_scheme_override: i32,
    pub input_mapping: super::input_shared::InputConceptToEntryInputActionMappings,
    pub input_curves: Vec<InputCurveData>,
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

pub const ENTRYCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntryComponentData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMECOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "EntryClass",
                flags: MemberInfoFlags::new(0),
                field_type: ENTRYCLASS_TYPE_INFO,
                rust_offset: offset_of!(EntryComponentData, entry_class),
            },
            FieldInfoData {
                name: "AIData",
                flags: MemberInfoFlags::new(0),
                field_type: GAMEAIENTRYDATA_TYPE_INFO,
                rust_offset: offset_of!(EntryComponentData, a_i_data),
            },
            FieldInfoData {
                name: "ForbiddenForHuman",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EntryComponentData, forbidden_for_human),
            },
            FieldInfoData {
                name: "InputGraph",
                flags: MemberInfoFlags::new(0),
                field_type: INPUTGRAPH_TYPE_INFO,
                rust_offset: offset_of!(EntryComponentData, input_graph),
            },
            FieldInfoData {
                name: "InputConceptDefinition",
                flags: MemberInfoFlags::new(0),
                field_type: INPUTACTIONMAPSDATA_TYPE_INFO,
                rust_offset: offset_of!(EntryComponentData, input_concept_definition),
            },
            FieldInfoData {
                name: "ActionMapSettingsFlipYSchemeOverride",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(EntryComponentData, action_map_settings_flip_y_scheme_override),
            },
            FieldInfoData {
                name: "InputMapping",
                flags: MemberInfoFlags::new(0),
                field_type: INPUTCONCEPTTOENTRYINPUTACTIONMAPPINGS_TYPE_INFO,
                rust_offset: offset_of!(EntryComponentData, input_mapping),
            },
            FieldInfoData {
                name: "InputCurves",
                flags: MemberInfoFlags::new(144),
                field_type: INPUTCURVEDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EntryComponentData, input_curves),
            },
            FieldInfoData {
                name: "HudData",
                flags: MemberInfoFlags::new(0),
                field_type: ENTRYCOMPONENTHUDDATA_TYPE_INFO,
                rust_offset: offset_of!(EntryComponentData, hud_data),
            },
            FieldInfoData {
                name: "EntryOrderNumber",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(EntryComponentData, entry_order_number),
            },
            FieldInfoData {
                name: "EnterImpulse",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EntryComponentData, enter_impulse),
            },
            FieldInfoData {
                name: "EntryRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EntryComponentData, entry_radius),
            },
            FieldInfoData {
                name: "IsAllowedToExitInAir",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EntryComponentData, is_allowed_to_exit_in_air),
            },
            FieldInfoData {
                name: "ClearPathToExitPointStartOffset",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(EntryComponentData, clear_path_to_exit_point_start_offset),
            },
            FieldInfoData {
                name: "IsShielded",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EntryComponentData, is_shielded),
            },
            FieldInfoData {
                name: "ShowSoldierInEntry",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EntryComponentData, show_soldier_in_entry),
            },
            FieldInfoData {
                name: "Show1pSoldierInEntry",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EntryComponentData, show1p_soldier_in_entry),
            },
            FieldInfoData {
                name: "SoldierTransitionInvisbleTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EntryComponentData, soldier_transition_invisble_time),
            },
            FieldInfoData {
                name: "EntrySpottingSettings",
                flags: MemberInfoFlags::new(0),
                field_type: ENTRYSPOTTINGSETTINGS_TYPE_INFO,
                rust_offset: offset_of!(EntryComponentData, entry_spotting_settings),
            },
            FieldInfoData {
                name: "ShowSoldierWeaponInEntry",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EntryComponentData, show_soldier_weapon_in_entry),
            },
            FieldInfoData {
                name: "Show1pSoldierInEntryForPlayerOnly",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EntryComponentData, show1p_soldier_in_entry_for_player_only),
            },
            FieldInfoData {
                name: "Show3pSoldierWeaponInEntry",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EntryComponentData, show3p_soldier_weapon_in_entry),
            },
            FieldInfoData {
                name: "ShowSoldierGearInEntry",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EntryComponentData, show_soldier_gear_in_entry),
            },
            FieldInfoData {
                name: "PoseConstraints",
                flags: MemberInfoFlags::new(0),
                field_type: POSECONSTRAINTSDATA_TYPE_INFO,
                rust_offset: offset_of!(EntryComponentData, pose_constraints),
            },
            FieldInfoData {
                name: "UseLocalTransform",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EntryComponentData, use_local_transform),
            },
            FieldInfoData {
                name: "TriggerEventOnKey",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(EntryComponentData, trigger_event_on_key),
            },
            FieldInfoData {
                name: "AllowRagdollFromEntry",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EntryComponentData, allow_ragdoll_from_entry),
            },
            FieldInfoData {
                name: "CameraIndex",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(EntryComponentData, camera_index),
            },
        ],
    }),
    array_type: Some(ENTRYCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EntryComponentData {
    fn type_info() -> &'static TypeInfo {
        ENTRYCOMPONENTDATA_TYPE_INFO
    }
}


pub const ENTRYCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntryComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("EntryComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct InputCurveData {
    pub affected_inputs: Vec<i32>,
    pub input_modifier_curve: Vec<super::core::Vec2>,
    pub handle_multiple_inputs_as_square: bool,
}

pub const INPUTCURVEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputCurveData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "AffectedInputs",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(InputCurveData, affected_inputs),
            },
            FieldInfoData {
                name: "InputModifierCurve",
                flags: MemberInfoFlags::new(144),
                field_type: VEC2_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(InputCurveData, input_modifier_curve),
            },
            FieldInfoData {
                name: "HandleMultipleInputsAsSquare",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(InputCurveData, handle_multiple_inputs_as_square),
            },
        ],
    }),
    array_type: Some(INPUTCURVEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for InputCurveData {
    fn type_info() -> &'static TypeInfo {
        INPUTCURVEDATA_TYPE_INFO
    }
}


pub const INPUTCURVEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputCurveData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("InputCurveData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EntrySpottingSettings {
    #[default]
    ESSDefault = 0,
    ESSSendAndReceive = 1,
    ESSReceive = 2,
    ESSCount = 3,
}

pub const ENTRYSPOTTINGSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntrySpottingSettings",
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(ENTRYSPOTTINGSETTINGS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EntrySpottingSettings {
    fn type_info() -> &'static TypeInfo {
        ENTRYSPOTTINGSETTINGS_TYPE_INFO
    }
}


pub const ENTRYSPOTTINGSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntrySpottingSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("EntrySpottingSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EntryComponentHudData {
    pub index: i32,
    pub frustum: bool,
    pub visible: bool,
    pub seat_type: EntrySeatType,
    pub maximize_mini_map_on_entry: bool,
}

pub const ENTRYCOMPONENTHUDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntryComponentHudData",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Index",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(EntryComponentHudData, index),
            },
            FieldInfoData {
                name: "Frustum",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EntryComponentHudData, frustum),
            },
            FieldInfoData {
                name: "Visible",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EntryComponentHudData, visible),
            },
            FieldInfoData {
                name: "SeatType",
                flags: MemberInfoFlags::new(0),
                field_type: ENTRYSEATTYPE_TYPE_INFO,
                rust_offset: offset_of!(EntryComponentHudData, seat_type),
            },
            FieldInfoData {
                name: "MaximizeMiniMapOnEntry",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EntryComponentHudData, maximize_mini_map_on_entry),
            },
        ],
    }),
    array_type: Some(ENTRYCOMPONENTHUDDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for EntryComponentHudData {
    fn type_info() -> &'static TypeInfo {
        ENTRYCOMPONENTHUDDATA_TYPE_INFO
    }
}


pub const ENTRYCOMPONENTHUDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntryComponentHudData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("EntryComponentHudData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EntrySeatType {
    #[default]
    EST_Driver = 0,
    EST_Gunner = 1,
    EST_Passenger = 2,
}

pub const ENTRYSEATTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntrySeatType",
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(ENTRYSEATTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EntrySeatType {
    fn type_info() -> &'static TypeInfo {
        ENTRYSEATTYPE_TYPE_INFO
    }
}


pub const ENTRYSEATTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntrySeatType-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("EntrySeatType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FOVTransitionData {
    pub transition_type: FOVTransitionType,
    pub shape: f32,
    pub invert: bool,
    pub start_delay: f32,
    pub start_jump: f32,
    pub end_early: f32,
}

pub const FOVTRANSITIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FOVTransitionData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "TransitionType",
                flags: MemberInfoFlags::new(0),
                field_type: FOVTRANSITIONTYPE_TYPE_INFO,
                rust_offset: offset_of!(FOVTransitionData, transition_type),
            },
            FieldInfoData {
                name: "Shape",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FOVTransitionData, shape),
            },
            FieldInfoData {
                name: "Invert",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FOVTransitionData, invert),
            },
            FieldInfoData {
                name: "StartDelay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FOVTransitionData, start_delay),
            },
            FieldInfoData {
                name: "StartJump",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FOVTransitionData, start_jump),
            },
            FieldInfoData {
                name: "EndEarly",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FOVTransitionData, end_early),
            },
        ],
    }),
    array_type: Some(FOVTRANSITIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FOVTransitionData {
    fn type_info() -> &'static TypeInfo {
        FOVTRANSITIONDATA_TYPE_INFO
    }
}


pub const FOVTRANSITIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FOVTransitionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("FOVTransitionData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum FOVTransitionType {
    #[default]
    FOVTransitionType_Linear = 0,
    FOVTransitionType_Smooth = 1,
    FOVTransitionType_Count = 2,
}

pub const FOVTRANSITIONTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FOVTransitionType",
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(FOVTRANSITIONTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for FOVTransitionType {
    fn type_info() -> &'static TypeInfo {
        FOVTRANSITIONTYPE_TYPE_INFO
    }
}


pub const FOVTRANSITIONTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FOVTransitionType-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("FOVTransitionType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CameraTransitionEffectDataPack {
    pub default_transition_in: CameraTransitionEffectData,
    pub default_transition_out: CameraTransitionEffectData,
}

pub const CAMERATRANSITIONEFFECTDATAPACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraTransitionEffectDataPack",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DefaultTransitionIn",
                flags: MemberInfoFlags::new(0),
                field_type: CAMERATRANSITIONEFFECTDATA_TYPE_INFO,
                rust_offset: offset_of!(CameraTransitionEffectDataPack, default_transition_in),
            },
            FieldInfoData {
                name: "DefaultTransitionOut",
                flags: MemberInfoFlags::new(0),
                field_type: CAMERATRANSITIONEFFECTDATA_TYPE_INFO,
                rust_offset: offset_of!(CameraTransitionEffectDataPack, default_transition_out),
            },
        ],
    }),
    array_type: Some(CAMERATRANSITIONEFFECTDATAPACK_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CameraTransitionEffectDataPack {
    fn type_info() -> &'static TypeInfo {
        CAMERATRANSITIONEFFECTDATAPACK_TYPE_INFO
    }
}


pub const CAMERATRANSITIONEFFECTDATAPACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraTransitionEffectDataPack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("CameraTransitionEffectDataPack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CameraTransitionEffectData {
    pub transition_type: CameraTransitionType,
    pub priority: CameraTransitionPriority,
    pub ease_type: CameraTransitionEase,
    pub duration: f32,
    pub spring_constant: f32,
    pub spring_max_velocity: f32,
}

pub const CAMERATRANSITIONEFFECTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraTransitionEffectData",
    flags: MemberInfoFlags::new(36937),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "TransitionType",
                flags: MemberInfoFlags::new(0),
                field_type: CAMERATRANSITIONTYPE_TYPE_INFO,
                rust_offset: offset_of!(CameraTransitionEffectData, transition_type),
            },
            FieldInfoData {
                name: "Priority",
                flags: MemberInfoFlags::new(0),
                field_type: CAMERATRANSITIONPRIORITY_TYPE_INFO,
                rust_offset: offset_of!(CameraTransitionEffectData, priority),
            },
            FieldInfoData {
                name: "EaseType",
                flags: MemberInfoFlags::new(0),
                field_type: CAMERATRANSITIONEASE_TYPE_INFO,
                rust_offset: offset_of!(CameraTransitionEffectData, ease_type),
            },
            FieldInfoData {
                name: "Duration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CameraTransitionEffectData, duration),
            },
            FieldInfoData {
                name: "SpringConstant",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CameraTransitionEffectData, spring_constant),
            },
            FieldInfoData {
                name: "SpringMaxVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CameraTransitionEffectData, spring_max_velocity),
            },
        ],
    }),
    array_type: Some(CAMERATRANSITIONEFFECTDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for CameraTransitionEffectData {
    fn type_info() -> &'static TypeInfo {
        CAMERATRANSITIONEFFECTDATA_TYPE_INFO
    }
}


pub const CAMERATRANSITIONEFFECTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraTransitionEffectData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("CameraTransitionEffectData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum CameraTransitionPriority {
    #[default]
    CameraTransitionPriority_Low = 0,
    CameraTransitionPriority_Medium = 1,
    CameraTransitionPriority_High = 2,
    CameraTransitionPriority_Count = 3,
}

pub const CAMERATRANSITIONPRIORITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraTransitionPriority",
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(CAMERATRANSITIONPRIORITY_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CameraTransitionPriority {
    fn type_info() -> &'static TypeInfo {
        CAMERATRANSITIONPRIORITY_TYPE_INFO
    }
}


pub const CAMERATRANSITIONPRIORITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraTransitionPriority-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("CameraTransitionPriority-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum CameraTransitionEase {
    #[default]
    CameraTransitionEase_InOut = 0,
    CameraTransitionEase_In = 1,
    CameraTransitionEase_Out = 2,
    CameraTransitionEase_Count = 3,
}

pub const CAMERATRANSITIONEASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraTransitionEase",
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(CAMERATRANSITIONEASE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CameraTransitionEase {
    fn type_info() -> &'static TypeInfo {
        CAMERATRANSITIONEASE_TYPE_INFO
    }
}


pub const CAMERATRANSITIONEASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraTransitionEase-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("CameraTransitionEase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
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

pub const CAMERATRANSITIONTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraTransitionType",
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(CAMERATRANSITIONTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CameraTransitionType {
    fn type_info() -> &'static TypeInfo {
        CAMERATRANSITIONTYPE_TYPE_INFO
    }
}


pub const CAMERATRANSITIONTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraTransitionType-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("CameraTransitionType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TargetCameraData {
    pub hud: Vec<HudData>,
    pub use_transform_space_hierarchy: bool,
}

pub const TARGETCAMERADATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TargetCameraData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CAMERADATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Hud",
                flags: MemberInfoFlags::new(144),
                field_type: HUDDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(TargetCameraData, hud),
            },
            FieldInfoData {
                name: "UseTransformSpaceHierarchy",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TargetCameraData, use_transform_space_hierarchy),
            },
        ],
    }),
    array_type: Some(TARGETCAMERADATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TargetCameraData {
    fn type_info() -> &'static TypeInfo {
        TARGETCAMERADATA_TYPE_INFO
    }
}


pub const TARGETCAMERADATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TargetCameraData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("TargetCameraData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct GameModeViewDefinition {
    pub game_mode_name: String,
    pub view_definitions: Vec<PlayerViewDefinition>,
}

pub const GAMEMODEVIEWDEFINITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameModeViewDefinition",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "GameModeName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(GameModeViewDefinition, game_mode_name),
            },
            FieldInfoData {
                name: "ViewDefinitions",
                flags: MemberInfoFlags::new(144),
                field_type: PLAYERVIEWDEFINITION_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(GameModeViewDefinition, view_definitions),
            },
        ],
    }),
    array_type: Some(GAMEMODEVIEWDEFINITION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GameModeViewDefinition {
    fn type_info() -> &'static TypeInfo {
        GAMEMODEVIEWDEFINITION_TYPE_INFO
    }
}


pub const GAMEMODEVIEWDEFINITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameModeViewDefinition-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("GameModeViewDefinition-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PlayerViewDefinition {
    pub local_player_id: super::core::LocalPlayerId,
    pub views: Vec<super::render_base::ViewDefinition>,
}

pub const PLAYERVIEWDEFINITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayerViewDefinition",
    flags: MemberInfoFlags::new(73),
    module: "GameplaySim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "LocalPlayerId",
                flags: MemberInfoFlags::new(0),
                field_type: LOCALPLAYERID_TYPE_INFO,
                rust_offset: offset_of!(PlayerViewDefinition, local_player_id),
            },
            FieldInfoData {
                name: "Views",
                flags: MemberInfoFlags::new(144),
                field_type: VIEWDEFINITION_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(PlayerViewDefinition, views),
            },
        ],
    }),
    array_type: Some(PLAYERVIEWDEFINITION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PlayerViewDefinition {
    fn type_info() -> &'static TypeInfo {
        PLAYERVIEWDEFINITION_TYPE_INFO
    }
}


pub const PLAYERVIEWDEFINITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayerViewDefinition-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("PlayerViewDefinition-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum CameraIds {
    #[default]
    NoCameraId = 0,
    FreeCameraId = 1,
    EntryCameraId = 2,
    CameraIdCount = 3,
}

pub const CAMERAIDS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraIds",
    flags: MemberInfoFlags::new(49429),
    module: "GameplaySim",
    data: TypeInfoData::Enum,
    array_type: Some(CAMERAIDS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CameraIds {
    fn type_info() -> &'static TypeInfo {
        CAMERAIDS_TYPE_INFO
    }
}


pub const CAMERAIDS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraIds-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("CameraIds-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CameraData {
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
    pub transition_data_pack: CameraTransitionEffectDataPack,
}

pub const CAMERADATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraData",
    flags: MemberInfoFlags::new(101),
    module: "GameplaySim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMEOBJECTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "OcclusionRayOffset",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(CameraData, occlusion_ray_offset),
            },
            FieldInfoData {
                name: "PreFadeTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CameraData, pre_fade_time),
            },
            FieldInfoData {
                name: "FadeTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CameraData, fade_time),
            },
            FieldInfoData {
                name: "FadeWaitTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CameraData, fade_wait_time),
            },
            FieldInfoData {
                name: "StayFadedWhileStreaming",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CameraData, stay_faded_while_streaming),
            },
            FieldInfoData {
                name: "NearPlane",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CameraData, near_plane),
            },
            FieldInfoData {
                name: "ShadowViewDistanceScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CameraData, shadow_view_distance_scale),
            },
            FieldInfoData {
                name: "SoundOcclusion",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CameraData, sound_occlusion),
            },
            FieldInfoData {
                name: "SoundListenerRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CameraData, sound_listener_radius),
            },
            FieldInfoData {
                name: "SoundListenerFov",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CameraData, sound_listener_fov),
            },
            FieldInfoData {
                name: "ShakeFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CameraData, shake_factor),
            },
            FieldInfoData {
                name: "ViewId",
                flags: MemberInfoFlags::new(0),
                field_type: LOCALPLAYERVIEWID_TYPE_INFO,
                rust_offset: offset_of!(CameraData, view_id),
            },
            FieldInfoData {
                name: "TransitionDataPack",
                flags: MemberInfoFlags::new(0),
                field_type: CAMERATRANSITIONEFFECTDATAPACK_TYPE_INFO,
                rust_offset: offset_of!(CameraData, transition_data_pack),
            },
        ],
    }),
    array_type: Some(CAMERADATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CameraData {
    fn type_info() -> &'static TypeInfo {
        CAMERADATA_TYPE_INFO
    }
}


pub const CAMERADATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplaySim",
    data: TypeInfoData::Array("CameraData-Array"),
    array_type: None,
    alignment: 8,
};


