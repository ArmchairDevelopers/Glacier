use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_game_client_types(registry: &mut TypeRegistry) {
    registry.register_type(CHASECAMERA_TYPE_INFO);
    registry.register_type(CHASECAMERA_ARRAY_TYPE_INFO);
    registry.register_type(CAMERATRACKCAMERA_TYPE_INFO);
    registry.register_type(CAMERATRACKCAMERA_ARRAY_TYPE_INFO);
    registry.register_type(CAMERATRACKWITHSCENECAMERA_TYPE_INFO);
    registry.register_type(CAMERATRACKWITHSCENECAMERA_ARRAY_TYPE_INFO);
    registry.register_type(CAMERATRACKBASE_TYPE_INFO);
    registry.register_type(CAMERATRACKBASE_ARRAY_TYPE_INFO);
    registry.register_type(CAMERALOOKATLAYERTRACK_TYPE_INFO);
    registry.register_type(CAMERALOOKATLAYERTRACK_ARRAY_TYPE_INFO);
    registry.register_type(CAMERAFOCUSTRACK_TYPE_INFO);
    registry.register_type(CAMERAFOCUSTRACK_ARRAY_TYPE_INFO);
    registry.register_type(CAMERADOFTRACK_TYPE_INFO);
    registry.register_type(CAMERADOFTRACK_ARRAY_TYPE_INFO);
    registry.register_type(CAMERADIRECTORTRACKBASE_TYPE_INFO);
    registry.register_type(CAMERADIRECTORTRACKBASE_ARRAY_TYPE_INFO);
    registry.register_type(CAMERADIRECTORPROXYTRACK_TYPE_INFO);
    registry.register_type(CAMERADIRECTORPROXYTRACK_ARRAY_TYPE_INFO);
    registry.register_type(CAMERACUTVISIBILITYTRACK_TYPE_INFO);
    registry.register_type(CAMERACUTVISIBILITYTRACK_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPROXYBANKENTITY_TYPE_INFO);
    registry.register_type(CLIENTPROXYBANKENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPLAYANIMATIONENTITY_TYPE_INFO);
    registry.register_type(CLIENTPLAYANIMATIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPA2TARGETCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTPA2TARGETCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(OBJECTHIGHLIGHTENTITY_TYPE_INFO);
    registry.register_type(OBJECTHIGHLIGHTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(MEMORYPOOLCONTROLENTITY_TYPE_INFO);
    registry.register_type(MEMORYPOOLCONTROLENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTWORLDRENDERERSWITCHENTITY_TYPE_INFO);
    registry.register_type(CLIENTWORLDRENDERERSWITCHENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTTEXTUREASSETENTITY_TYPE_INFO);
    registry.register_type(CLIENTTEXTUREASSETENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTTERRAINSHADERPARAMETERENTITY_TYPE_INFO);
    registry.register_type(CLIENTTERRAINSHADERPARAMETERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPERFJOURNALBOOKMARKLABELENTITY_TYPE_INFO);
    registry.register_type(CLIENTPERFJOURNALBOOKMARKLABELENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPERFJOURNALBOOKMARKENTITY_TYPE_INFO);
    registry.register_type(CLIENTPERFJOURNALBOOKMARKENTITY_ARRAY_TYPE_INFO);
    registry.register_type(GAMECLIENTCONNECTION_TYPE_INFO);
    registry.register_type(GAMECLIENTCONNECTION_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPLAYERINPUTTRIGGERENTITY_TYPE_INFO);
    registry.register_type(CLIENTPLAYERINPUTTRIGGERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCOMBATAREATRIGGERENTITY_TYPE_INFO);
    registry.register_type(CLIENTCOMBATAREATRIGGERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCLEARAREATRIGGERENTITY_TYPE_INFO);
    registry.register_type(CLIENTCLEARAREATRIGGERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CAMERAENTERAREATRIGGERENTITY_TYPE_INFO);
    registry.register_type(CAMERAENTERAREATRIGGERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTVEHICLESPAWNENTITY_TYPE_INFO);
    registry.register_type(CLIENTVEHICLESPAWNENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTLOCALPLAYERPROXYENTITYTRACKDATA_TYPE_INFO);
    registry.register_type(CLIENTLOCALPLAYERPROXYENTITYTRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(PA2LOOKATCONFIGURATION_TYPE_INFO);
    registry.register_type(PA2LOOKATCONFIGURATION_ARRAY_TYPE_INFO);
    registry.register_type(PA2FLOATLOOKATGAMESTATESETTING_TYPE_INFO);
    registry.register_type(PA2FLOATLOOKATGAMESTATESETTING_ARRAY_TYPE_INFO);
    registry.register_type(PA2BOOLLOOKATGAMESTATESETTING_TYPE_INFO);
    registry.register_type(PA2BOOLLOOKATGAMESTATESETTING_ARRAY_TYPE_INFO);
    registry.register_type(PA2LOOKATGAMESTATESETTING_TYPE_INFO);
    registry.register_type(PA2LOOKATGAMESTATESETTING_ARRAY_TYPE_INFO);
    registry.register_type(PA2LOOKATTRACKDATA_TYPE_INFO);
    registry.register_type(PA2LOOKATTRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(PA2LOOKATKEYFRAME_TYPE_INFO);
    registry.register_type(PA2LOOKATKEYFRAME_ARRAY_TYPE_INFO);
    registry.register_type(PA2LOOKATTARGET_TYPE_INFO);
    registry.register_type(PA2LOOKATTARGET_ARRAY_TYPE_INFO);
    registry.register_type(LIGHTINGTRACKDATA_TYPE_INFO);
    registry.register_type(LIGHTINGTRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(LIGHTTRACKDATA_TYPE_INFO);
    registry.register_type(LIGHTTRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(ORBISSYSTEMCOMPANIONUTILEVENTMESSAGEBASE_TYPE_INFO);
    registry.register_type(ORBISSYSTEMCOMPANIONHTTPDEVENTMESSAGEBASE_TYPE_INFO);
    registry.register_type(ORBISSYSTEMUSEREVENTMESSAGEBASE_TYPE_INFO);
    registry.register_type(ORBISSYSTEMTITLEEXTERNALEVENTMESSAGEBASE_TYPE_INFO);
    registry.register_type(ORBISSYSTEMUIOVERLAIDSTATUSCHANGEEVENTMESSAGE_TYPE_INFO);
    registry.register_type(ORBISSYSTEMBACKGROUNDSTATUSCHANGEEVENTMESSAGE_TYPE_INFO);
    registry.register_type(PERFJOURNALBOOKMARKLABELENTITYDATA_TYPE_INFO);
    registry.register_type(PERFJOURNALBOOKMARKLABELENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PERFJOURNALBOOKMARKENTITYDATA_TYPE_INFO);
    registry.register_type(PERFJOURNALBOOKMARKENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(GETFLOATPROFILEOPTION_FLOAT32_LOCALPLAYERID_PROFILEOPTIONSTYPE_CSTRING_FLOAT32__TYPE_INFO);
    registry.register_type(GETINTPROFILEOPTION_INT32_LOCALPLAYERID_PROFILEOPTIONSTYPE_CSTRING_INT32__TYPE_INFO);
    registry.register_type(GETBOOLPROFILEOPTION_BOOLEAN_LOCALPLAYERID_PROFILEOPTIONSTYPE_CSTRING_BOOLEAN__TYPE_INFO);
    registry.register_type(UIEXITTOMENUREASONMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTCOMMANDERPLAYERCHANGEDMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTSTATECHANGINGSTATEMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTGAMEPLAYCONTROLLABLELOWHEALTHMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTGAMEPLAYSOLDIERHITMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTCAMERASHOWKILLERMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTCOLLISIONPROJECTILEIMPACTMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTCOLLISIONSPAWNEXPLOSIONMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTCOLLISIONEXPLOSIONPACKDETONATEDMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTCOLLISIONEXPLOSIONPACKPLACEDMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTCOLLISIONGRENADECOLLISIONMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTSOUNDVOICEOVERFINISHEDMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTVEHICLECRITICALDAMAGEMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTENTITYDUMMYTOGETTHECLIENTENTITYMESSAGECATEGORYMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTCHARACTERSPAWNDONEMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTCHARACTERLOCALPLAYERDELETEDMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTCONTROLLABLELOCALPLAYERSETMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTWEAPONCOMPONENTWEAPONONUNSPAWNMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTWEAPONCOMPONENTWEAPONONSPAWNMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTVEHICLESETREMOTECONTROLLEDDAMAGEGIVERMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTVEHICLESPAWNDONEMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTPLAYERUNLOCKSUPDATEDMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTPLAYERONWEAPONUNDEPLOYFINISHEDMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTPLAYERSELECTEDCUSTOMIZATIONMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTPLAYERSELECTEDUNLOCKSMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTPLAYERMANDOWNMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTPLAYERWEAPONPICKUPMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTPLAYERAMMOPICKUPMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTPLAYERKILLEDMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTPLAYERENTEREXITVEHICLEMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTMETRICSPAUSEGAMEMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTMETRICSUIACTIONMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTMETRICSPOPUISCREENMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTMETRICSPUSHUISCREENMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTGAMETYPE_TYPE_INFO);
    registry.register_type(CLIENTGAMETYPE_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSTATE_TYPE_INFO);
    registry.register_type(CLIENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(SUBLEVELVISIBLEENTITYDATA_TYPE_INFO);
    registry.register_type(SUBLEVELVISIBLEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTGROUPCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTGROUPCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTEVENTSYNCENTITY_TYPE_INFO);
    registry.register_type(CLIENTEVENTSYNCENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTBLUEPRINTENTITY_TYPE_INFO);
    registry.register_type(CLIENTBLUEPRINTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(VOICEOVERMANAGERCONTROLENTITY_TYPE_INFO);
    registry.register_type(VOICEOVERMANAGERCONTROLENTITY_ARRAY_TYPE_INFO);
    registry.register_type(VOICEOVEREVENTENTITY_TYPE_INFO);
    registry.register_type(VOICEOVEREVENTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(VOICEOVERCONVERSATIONENTITY_TYPE_INFO);
    registry.register_type(VOICEOVERCONVERSATIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTVOICEOVERANIMATIONCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTVOICEOVERANIMATIONCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SOUNDLISTENERENTITY_TYPE_INFO);
    registry.register_type(SOUNDLISTENERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SOUNDAREAENTITY_TYPE_INFO);
    registry.register_type(SOUNDAREAENTITY_ARRAY_TYPE_INFO);
    registry.register_type(DETACHEDSOUNDLISTENERENTITY_TYPE_INFO);
    registry.register_type(DETACHEDSOUNDLISTENERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTWATERPHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTWATERPHYSICSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTWATERHEALTHCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTWATERHEALTHCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTTERRAINPHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTTERRAINPHYSICSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTTERRAINHEALTHCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTTERRAINHEALTHCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSTATICMODELPHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTSTATICMODELPHYSICSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSTATICMODELHEALTHCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTSTATICMODELHEALTHCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSTATICMODELGROUPPHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTSTATICMODELGROUPPHYSICSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSTATICMODELGROUPHEALTHCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTSTATICMODELGROUPHEALTHCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTBOXREFLECTIONVOLUMECOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTBOXREFLECTIONVOLUMECOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSPHEREREFLECTIONVOLUMECOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTSPHEREREFLECTIONVOLUMECOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTREFLECTIONVOLUMECOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTREFLECTIONVOLUMECOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTLIGHTCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTLIGHTCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTLENSFLARECOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTLENSFLARECOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTEFFECTCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTEFFECTCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTDYNAMICMODELPHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTDYNAMICMODELPHYSICSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTDYNAMICMODELHEALTHCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTDYNAMICMODELHEALTHCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTBANGERPHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTBANGERPHYSICSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTBANGERHEALTHCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTBANGERHEALTHCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSPECTATORSUBVIEW_TYPE_INFO);
    registry.register_type(CLIENTSPECTATORSUBVIEW_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTGAMESPLINEENTITY_TYPE_INFO);
    registry.register_type(CLIENTGAMESPLINEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTAREAIMMUNITYCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTAREAIMMUNITYCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTDYNAMICFIREENTITY_TYPE_INFO);
    registry.register_type(CLIENTDYNAMICFIREENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTWARNINGSYSTEMCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTWARNINGSYSTEMCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTUNLOCKCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTUNLOCKCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPLAYERENTRYCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTPLAYERENTRYCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTGAMEENTRYCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTGAMEENTRYCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCHARACTERENTITY_TYPE_INFO);
    registry.register_type(CLIENTCHARACTERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTWARPANIMATIONCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTWARPANIMATIONCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTVEHICLEENTRYLISTENERCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTVEHICLEENTRYLISTENERCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCHARACTERPHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTCHARACTERPHYSICSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCHARACTERMESHCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTCHARACTERMESHCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCHARACTERMASTERPHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTCHARACTERMASTERPHYSICSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCHARACTERHEALTHCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTCHARACTERHEALTHCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCHARACTERCUSTOMIZATIONCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTCHARACTERCUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCHARACTERCAMERACOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTCHARACTERCAMERACOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CHARACTERTARGETCAMERACALLBACK_TYPE_INFO);
    registry.register_type(CHARACTERTARGETCAMERACALLBACK_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCAMERARELATIVEINPUTCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTCAMERARELATIVEINPUTCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTANTINPUTCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTANTINPUTCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPLAYVFXENTITY_TYPE_INFO);
    registry.register_type(CLIENTPLAYVFXENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCHARACTERSPAWNENTITY_TYPE_INFO);
    registry.register_type(CLIENTCHARACTERSPAWNENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTTEAMFILTERENTITY_TYPE_INFO);
    registry.register_type(CLIENTTEAMFILTERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSYNCEDSEQUENCEENTITY_TYPE_INFO);
    registry.register_type(CLIENTSYNCEDSEQUENCEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSTREAMINGGATEENTITY_TYPE_INFO);
    registry.register_type(CLIENTSTREAMINGGATEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPLAYERITERATORENTITY_TYPE_INFO);
    registry.register_type(CLIENTPLAYERITERATORENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTLOCALPLAYERFILTERENTITY_TYPE_INFO);
    registry.register_type(CLIENTLOCALPLAYERFILTERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPLAYERFILTERENTITY_TYPE_INFO);
    registry.register_type(CLIENTPLAYERFILTERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTOBJECTIVEENTITY_TYPE_INFO);
    registry.register_type(CLIENTOBJECTIVEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTOBJECTAREAQUERYENTITY_TYPE_INFO);
    registry.register_type(CLIENTOBJECTAREAQUERYENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTMESSAGEENTITY_TYPE_INFO);
    registry.register_type(CLIENTMESSAGEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTMAPMARKERENTITY_TYPE_INFO);
    registry.register_type(CLIENTMAPMARKERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTLOCALPLAYEREVENTENTITY_TYPE_INFO);
    registry.register_type(CLIENTLOCALPLAYEREVENTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTEVENTMEMORYENTITY_TYPE_INFO);
    registry.register_type(CLIENTEVENTMEMORYENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTFADETRACK_TYPE_INFO);
    registry.register_type(CLIENTFADETRACK_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTFADEENTITY_TYPE_INFO);
    registry.register_type(CLIENTFADEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTDOUBLEPLAYEREVENT_TYPE_INFO);
    registry.register_type(CLIENTDOUBLEPLAYEREVENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTTEAMENTITY_TYPE_INFO);
    registry.register_type(CLIENTTEAMENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTLADDERCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTLADDERCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTMULTIPLEACTORSCENARIOENTITY_TYPE_INFO);
    registry.register_type(CLIENTMULTIPLEACTORSCENARIOENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTMODELANIMATIONENTITY_TYPE_INFO);
    registry.register_type(CLIENTMODELANIMATIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTJOINTVALIDATIONENTITY_TYPE_INFO);
    registry.register_type(CLIENTJOINTVALIDATIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTFBPROXYCONTROLLERENTITY_TYPE_INFO);
    registry.register_type(CLIENTFBPROXYCONTROLLERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTFACEPOSERENTITY_TYPE_INFO);
    registry.register_type(CLIENTFACEPOSERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCHARACTERINVEHICLESCENARIOENTITY_TYPE_INFO);
    registry.register_type(CLIENTCHARACTERINVEHICLESCENARIOENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTANTSIGNALTRACK_TYPE_INFO);
    registry.register_type(CLIENTANTSIGNALTRACK_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTWRITEANTGAMESTATEENTITY_TYPE_INFO);
    registry.register_type(CLIENTWRITEANTGAMESTATEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTREADANTGAMESTATEENTITY_TYPE_INFO);
    registry.register_type(CLIENTREADANTGAMESTATEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTANTEVENTENTITY_TYPE_INFO);
    registry.register_type(CLIENTANTEVENTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTANTCONTROLTRACK_TYPE_INFO);
    registry.register_type(CLIENTANTCONTROLTRACK_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTANTANIMATABLEENTITY_TYPE_INFO);
    registry.register_type(CLIENTANTANIMATABLEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTANIMATIONSIGNALENTITY_TYPE_INFO);
    registry.register_type(CLIENTANIMATIONSIGNALENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTANIMATIONPOSETRACK_TYPE_INFO);
    registry.register_type(CLIENTANIMATIONPOSETRACK_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTANIMATIONENUMERATIONCHOICEENTITY_TYPE_INFO);
    registry.register_type(CLIENTANIMATIONENUMERATIONCHOICEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTANIMATIONENUMERATIONENTITY_TYPE_INFO);
    registry.register_type(CLIENTANIMATIONENUMERATIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCHARACTERPOSEVALIDATETRACK_TYPE_INFO);
    registry.register_type(CLIENTCHARACTERPOSEVALIDATETRACK_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCHARACTERPOSERECORDTRACK_TYPE_INFO);
    registry.register_type(CLIENTCHARACTERPOSERECORDTRACK_ARRAY_TYPE_INFO);
    registry.register_type(MESHEMITTERCONTROLENTITY_TYPE_INFO);
    registry.register_type(MESHEMITTERCONTROLENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTROTORCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTROTORCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTMESHCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTMESHCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTIKCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTIKCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTFAKEPHYSICSMESHCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTFAKEPHYSICSMESHCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTENGINECOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTENGINECOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCHILDCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTCHILDCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCHILDBARRELCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTCHILDBARRELCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCHASSISCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTCHASSISCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCAMERACOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTCAMERACOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCOMPONENTTARGETCAMERACALLBACK_TYPE_INFO);
    registry.register_type(CLIENTCOMPONENTTARGETCAMERACALLBACK_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTGAMEPLAYEREXTENT_TYPE_INFO);
    registry.register_type(CLIENTGAMEPLAYEREXTENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTGAMEPLAYERINTERNALEXTENT_TYPE_INFO);
    registry.register_type(CLIENTGAMEPLAYERINTERNALEXTENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTEXPLOSIONENTITY_TYPE_INFO);
    registry.register_type(CLIENTEXPLOSIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTDYNAMICAVOIDANCEENTITY_TYPE_INFO);
    registry.register_type(CLIENTDYNAMICAVOIDANCEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTVEHICLEENTITY_TYPE_INFO);
    registry.register_type(CLIENTVEHICLEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTANIMATEDDRIVERENTITY_TYPE_INFO);
    registry.register_type(CLIENTANIMATEDDRIVERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTWHEELCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTWHEELCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTWATERDEPTHCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTWATERDEPTHCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTVEHICLEPHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTVEHICLEPHYSICSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTVEHICLEMESHCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTVEHICLEMESHCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTVEHICLEHEALTHCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTVEHICLEHEALTHCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTVEHICLEENTRYCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTVEHICLEENTRYCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTVEHICLECUSTOMIZATIONCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTVEHICLECUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTTRACKWHEELCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTTRACKWHEELCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTTRACKCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTTRACKCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSTANCEFILTERCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTSTANCEFILTERCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTEDGEMODELCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTEDGEMODELCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(DESTRUCTIONVOLUMECOMPONENT_TYPE_INFO);
    registry.register_type(DESTRUCTIONVOLUMECOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSTATICMODELNETWORKDESTRUCTIONCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTSTATICMODELNETWORKDESTRUCTIONCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTGAMESUBVIEW_TYPE_INFO);
    registry.register_type(CLIENTGAMESUBVIEW_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTLOCALPLAYERPROXYENTITYTRACK_TYPE_INFO);
    registry.register_type(CLIENTLOCALPLAYERPROXYENTITYTRACK_ARRAY_TYPE_INFO);
    registry.register_type(PA2LOOKATTRACK_TYPE_INFO);
    registry.register_type(PA2LOOKATTRACK_ARRAY_TYPE_INFO);
    registry.register_type(LIGHTTRACK_TYPE_INFO);
    registry.register_type(LIGHTTRACK_ARRAY_TYPE_INFO);
    registry.register_type(LIGHTINGTRACK_TYPE_INFO);
    registry.register_type(LIGHTINGTRACK_ARRAY_TYPE_INFO);
    registry.register_type(SUBLEVELVISIBLEENTITY_TYPE_INFO);
    registry.register_type(SUBLEVELVISIBLEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSUBLEVELENTITY_TYPE_INFO);
    registry.register_type(CLIENTSUBLEVELENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSUBLEVELCOLLECTIONENTITY_TYPE_INFO);
    registry.register_type(CLIENTSUBLEVELCOLLECTIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTLOCALPLAYERGATEENTITY_TYPE_INFO);
    registry.register_type(CLIENTLOCALPLAYERGATEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTANTDRIVENCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTANTDRIVENCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTWARPANIMATIONENTITY_TYPE_INFO);
    registry.register_type(CLIENTWARPANIMATIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPHYSICSDRIVENANIMATIONENTITY_TYPE_INFO);
    registry.register_type(CLIENTPHYSICSDRIVENANIMATIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCANNEDSCENARIOENTITY_TYPE_INFO);
    registry.register_type(CLIENTCANNEDSCENARIOENTITY_ARRAY_TYPE_INFO);
    registry.register_type(VEHICLECAMERA_TYPE_INFO);
    registry.register_type(VEHICLECAMERA_ARRAY_TYPE_INFO);
    registry.register_type(STATICCAMERA_TYPE_INFO);
    registry.register_type(STATICCAMERA_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICALCAMERATRACK_TYPE_INFO);
    registry.register_type(PHYSICALCAMERATRACK_ARRAY_TYPE_INFO);
    registry.register_type(FPSCAMERA_TYPE_INFO);
    registry.register_type(FPSCAMERA_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPLAYERCAMERAENTITY_TYPE_INFO);
    registry.register_type(CLIENTPLAYERCAMERAENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTGAMEPLAYCAMERATRACK_TYPE_INFO);
    registry.register_type(CLIENTGAMEPLAYCAMERATRACK_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCAMERATRACK_TYPE_INFO);
    registry.register_type(CLIENTCAMERATRACK_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTTARGETCAMERAENTITY_TYPE_INFO);
    registry.register_type(CLIENTTARGETCAMERAENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTLOOKATCAMERAENTITY_TYPE_INFO);
    registry.register_type(CLIENTLOOKATCAMERAENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCAMERAENTITY_TYPE_INFO);
    registry.register_type(CLIENTCAMERAENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCAMERAENTITYBASE_TYPE_INFO);
    registry.register_type(CLIENTCAMERAENTITYBASE_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCAMERADIRECTORTRACK_TYPE_INFO);
    registry.register_type(CLIENTCAMERADIRECTORTRACK_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTANTANIMATABLECAMERATRACK_TYPE_INFO);
    registry.register_type(CLIENTANTANIMATABLECAMERATRACK_ARRAY_TYPE_INFO);
    registry.register_type(IUISYSTEM_TYPE_INFO);
    registry.register_type(IUISYSTEM_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTWATERENTITY_TYPE_INFO);
    registry.register_type(CLIENTWATERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTTERRAINENTITY_TYPE_INFO);
    registry.register_type(CLIENTTERRAINENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSTATICMODELGROUPENTITY_TYPE_INFO);
    registry.register_type(CLIENTSTATICMODELGROUPENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSTATICMODELENTITY_TYPE_INFO);
    registry.register_type(CLIENTSTATICMODELENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTLIGHTDIMMERENTITY_TYPE_INFO);
    registry.register_type(CLIENTLIGHTDIMMERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTLADDERENTITY_TYPE_INFO);
    registry.register_type(CLIENTLADDERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTDYNAMICMODELENTITY_TYPE_INFO);
    registry.register_type(CLIENTDYNAMICMODELENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTBANGERENTITY_TYPE_INFO);
    registry.register_type(CLIENTBANGERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(LOGICVISUALENVIRONMENTENTITY_TYPE_INFO);
    registry.register_type(LOGICVISUALENVIRONMENTENTITY_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ChaseCamera {
}

pub const CHASECAMERA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ChaseCamera",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TARGETCAMERA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CHASECAMERA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ChaseCamera {
    fn type_info() -> &'static TypeInfo {
        CHASECAMERA_TYPE_INFO
    }
}


pub const CHASECAMERA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ChaseCamera-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ChaseCamera-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CameraTrackCamera {
}

pub const CAMERATRACKCAMERA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraTrackCamera",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CAMERA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CAMERATRACKCAMERA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CameraTrackCamera {
    fn type_info() -> &'static TypeInfo {
        CAMERATRACKCAMERA_TYPE_INFO
    }
}


pub const CAMERATRACKCAMERA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraTrackCamera-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("CameraTrackCamera-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CameraTrackWithSceneCamera {
}

pub const CAMERATRACKWITHSCENECAMERA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraTrackWithSceneCamera",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CAMERATRACKBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CAMERATRACKWITHSCENECAMERA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CameraTrackWithSceneCamera {
    fn type_info() -> &'static TypeInfo {
        CAMERATRACKWITHSCENECAMERA_TYPE_INFO
    }
}


pub const CAMERATRACKWITHSCENECAMERA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraTrackWithSceneCamera-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("CameraTrackWithSceneCamera-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CameraTrackBase {
}

pub const CAMERATRACKBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraTrackBase",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CAMERATRACKBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CameraTrackBase {
    fn type_info() -> &'static TypeInfo {
        CAMERATRACKBASE_TYPE_INFO
    }
}


pub const CAMERATRACKBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraTrackBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("CameraTrackBase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CameraLookAtLayerTrack {
}

pub const CAMERALOOKATLAYERTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraLookAtLayerTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TRANSFORMLAYER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CAMERALOOKATLAYERTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CameraLookAtLayerTrack {
    fn type_info() -> &'static TypeInfo {
        CAMERALOOKATLAYERTRACK_TYPE_INFO
    }
}


pub const CAMERALOOKATLAYERTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraLookAtLayerTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("CameraLookAtLayerTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CameraFocusTrack {
}

pub const CAMERAFOCUSTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraFocusTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CAMERAFOCUSTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CameraFocusTrack {
    fn type_info() -> &'static TypeInfo {
        CAMERAFOCUSTRACK_TYPE_INFO
    }
}


pub const CAMERAFOCUSTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraFocusTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("CameraFocusTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CameraDofTrack {
}

pub const CAMERADOFTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraDofTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CAMERADOFTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CameraDofTrack {
    fn type_info() -> &'static TypeInfo {
        CAMERADOFTRACK_TYPE_INFO
    }
}


pub const CAMERADOFTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraDofTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("CameraDofTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CameraDirectorTrackBase {
}

pub const CAMERADIRECTORTRACKBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraDirectorTrackBase",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CAMERADIRECTORTRACKBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CameraDirectorTrackBase {
    fn type_info() -> &'static TypeInfo {
        CAMERADIRECTORTRACKBASE_TYPE_INFO
    }
}


pub const CAMERADIRECTORTRACKBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraDirectorTrackBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("CameraDirectorTrackBase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CameraDirectorProxyTrack {
}

pub const CAMERADIRECTORPROXYTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraDirectorProxyTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CAMERADIRECTORPROXYTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CameraDirectorProxyTrack {
    fn type_info() -> &'static TypeInfo {
        CAMERADIRECTORPROXYTRACK_TYPE_INFO
    }
}


pub const CAMERADIRECTORPROXYTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraDirectorProxyTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("CameraDirectorProxyTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CameraCutVisibilityTrack {
}

pub const CAMERACUTVISIBILITYTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraCutVisibilityTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CAMERACUTVISIBILITYTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CameraCutVisibilityTrack {
    fn type_info() -> &'static TypeInfo {
        CAMERACUTVISIBILITYTRACK_TYPE_INFO
    }
}


pub const CAMERACUTVISIBILITYTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraCutVisibilityTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("CameraCutVisibilityTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientProxyBankEntity {
}

pub const CLIENTPROXYBANKENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientProxyBankEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPROXYBANKENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientProxyBankEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTPROXYBANKENTITY_TYPE_INFO
    }
}


pub const CLIENTPROXYBANKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientProxyBankEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientProxyBankEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPlayAnimationEntity {
}

pub const CLIENTPLAYANIMATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayAnimationEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPLAYANIMATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPlayAnimationEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTPLAYANIMATIONENTITY_TYPE_INFO
    }
}


pub const CLIENTPLAYANIMATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayAnimationEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientPlayAnimationEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPA2TargetComponent {
}

pub const CLIENTPA2TARGETCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPA2TargetComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPA2TARGETCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPA2TargetComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTPA2TARGETCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTPA2TARGETCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPA2TargetComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientPA2TargetComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ObjectHighlightEntity {
}

pub const OBJECTHIGHLIGHTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectHighlightEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(OBJECTHIGHLIGHTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ObjectHighlightEntity {
    fn type_info() -> &'static TypeInfo {
        OBJECTHIGHLIGHTENTITY_TYPE_INFO
    }
}


pub const OBJECTHIGHLIGHTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectHighlightEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ObjectHighlightEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MemoryPoolControlEntity {
}

pub const MEMORYPOOLCONTROLENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryPoolControlEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(MEMORYPOOLCONTROLENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MemoryPoolControlEntity {
    fn type_info() -> &'static TypeInfo {
        MEMORYPOOLCONTROLENTITY_TYPE_INFO
    }
}


pub const MEMORYPOOLCONTROLENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryPoolControlEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("MemoryPoolControlEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientWorldRendererSwitchEntity {
}

pub const CLIENTWORLDRENDERERSWITCHENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWorldRendererSwitchEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWORLDRENDERERSWITCHENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWorldRendererSwitchEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTWORLDRENDERERSWITCHENTITY_TYPE_INFO
    }
}


pub const CLIENTWORLDRENDERERSWITCHENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWorldRendererSwitchEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientWorldRendererSwitchEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientTextureAssetEntity {
}

pub const CLIENTTEXTUREASSETENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTextureAssetEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTTEXTUREASSETENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientTextureAssetEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTTEXTUREASSETENTITY_TYPE_INFO
    }
}


pub const CLIENTTEXTUREASSETENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTextureAssetEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientTextureAssetEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientTerrainShaderParameterEntity {
}

pub const CLIENTTERRAINSHADERPARAMETERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTerrainShaderParameterEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTTERRAINSHADERPARAMETERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientTerrainShaderParameterEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTTERRAINSHADERPARAMETERENTITY_TYPE_INFO
    }
}


pub const CLIENTTERRAINSHADERPARAMETERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTerrainShaderParameterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientTerrainShaderParameterEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPerfJournalBookmarkLabelEntity {
}

pub const CLIENTPERFJOURNALBOOKMARKLABELENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPerfJournalBookmarkLabelEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPERFJOURNALBOOKMARKLABELENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPerfJournalBookmarkLabelEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTPERFJOURNALBOOKMARKLABELENTITY_TYPE_INFO
    }
}


pub const CLIENTPERFJOURNALBOOKMARKLABELENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPerfJournalBookmarkLabelEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientPerfJournalBookmarkLabelEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPerfJournalBookmarkEntity {
}

pub const CLIENTPERFJOURNALBOOKMARKENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPerfJournalBookmarkEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPERFJOURNALBOOKMARKENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPerfJournalBookmarkEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTPERFJOURNALBOOKMARKENTITY_TYPE_INFO
    }
}


pub const CLIENTPERFJOURNALBOOKMARKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPerfJournalBookmarkEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientPerfJournalBookmarkEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GameClientConnection {
}

pub const GAMECLIENTCONNECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameClientConnection",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTCONNECTION_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(GAMECLIENTCONNECTION_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for GameClientConnection {
    fn type_info() -> &'static TypeInfo {
        GAMECLIENTCONNECTION_TYPE_INFO
    }
}


pub const GAMECLIENTCONNECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameClientConnection-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("GameClientConnection-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPlayerInputTriggerEntity {
}

pub const CLIENTPLAYERINPUTTRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerInputTriggerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPLAYERINPUTTRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPlayerInputTriggerEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTPLAYERINPUTTRIGGERENTITY_TYPE_INFO
    }
}


pub const CLIENTPLAYERINPUTTRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerInputTriggerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientPlayerInputTriggerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCombatAreaTriggerEntity {
}

pub const CLIENTCOMBATAREATRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCombatAreaTriggerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMBATAREAENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCOMBATAREATRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCombatAreaTriggerEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTCOMBATAREATRIGGERENTITY_TYPE_INFO
    }
}


pub const CLIENTCOMBATAREATRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCombatAreaTriggerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientCombatAreaTriggerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientClearAreaTriggerEntity {
}

pub const CLIENTCLEARAREATRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientClearAreaTriggerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCLEARAREATRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientClearAreaTriggerEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTCLEARAREATRIGGERENTITY_TYPE_INFO
    }
}


pub const CLIENTCLEARAREATRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientClearAreaTriggerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientClearAreaTriggerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CameraEnterAreaTriggerEntity {
}

pub const CAMERAENTERAREATRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraEnterAreaTriggerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CAMERAENTERAREATRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CameraEnterAreaTriggerEntity {
    fn type_info() -> &'static TypeInfo {
        CAMERAENTERAREATRIGGERENTITY_TYPE_INFO
    }
}


pub const CAMERAENTERAREATRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraEnterAreaTriggerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("CameraEnterAreaTriggerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientVehicleSpawnEntity {
}

pub const CLIENTVEHICLESPAWNENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVehicleSpawnEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTSPAWNENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVEHICLESPAWNENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientVehicleSpawnEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTVEHICLESPAWNENTITY_TYPE_INFO
    }
}


pub const CLIENTVEHICLESPAWNENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVehicleSpawnEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientVehicleSpawnEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientLocalPlayerProxyEntityTrackData {
}

pub const CLIENTLOCALPLAYERPROXYENTITYTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLocalPlayerProxyEntityTrackData",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TEMPLATEDPROXYENTITYTRACKDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTLOCALPLAYERPROXYENTITYTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ClientLocalPlayerProxyEntityTrackData {
    fn type_info() -> &'static TypeInfo {
        CLIENTLOCALPLAYERPROXYENTITYTRACKDATA_TYPE_INFO
    }
}


pub const CLIENTLOCALPLAYERPROXYENTITYTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLocalPlayerProxyEntityTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientLocalPlayerProxyEntityTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PA2LookAtConfiguration {
    pub on_game_states: Vec<PA2LookAtGameStateSetting>,
    pub off_game_states: Vec<PA2LookAtGameStateSetting>,
    pub controller_game_state: super::ant::AntRef,
    pub default_on_controller: i32,
    pub off_controller: i32,
    pub target_game_state: super::ant::AntRef,
    pub snap_to_target_game_state: super::ant::AntRef,
}

pub const PA2LOOKATCONFIGURATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2LookAtConfiguration",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "OnGameStates",
                flags: MemberInfoFlags::new(144),
                field_type: PA2LOOKATGAMESTATESETTING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(PA2LookAtConfiguration, on_game_states),
            },
            FieldInfoData {
                name: "OffGameStates",
                flags: MemberInfoFlags::new(144),
                field_type: PA2LOOKATGAMESTATESETTING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(PA2LookAtConfiguration, off_game_states),
            },
            FieldInfoData {
                name: "ControllerGameState",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(PA2LookAtConfiguration, controller_game_state),
            },
            FieldInfoData {
                name: "DefaultOnController",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PA2LookAtConfiguration, default_on_controller),
            },
            FieldInfoData {
                name: "OffController",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PA2LookAtConfiguration, off_controller),
            },
            FieldInfoData {
                name: "TargetGameState",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(PA2LookAtConfiguration, target_game_state),
            },
            FieldInfoData {
                name: "SnapToTargetGameState",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(PA2LookAtConfiguration, snap_to_target_game_state),
            },
        ],
    }),
    array_type: Some(PA2LOOKATCONFIGURATION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PA2LookAtConfiguration {
    fn type_info() -> &'static TypeInfo {
        PA2LOOKATCONFIGURATION_TYPE_INFO
    }
}


pub const PA2LOOKATCONFIGURATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2LookAtConfiguration-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("PA2LookAtConfiguration-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PA2FloatLookAtGameStateSetting {
    pub value: f32,
}

pub const PA2FLOATLOOKATGAMESTATESETTING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2FloatLookAtGameStateSetting",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PA2LOOKATGAMESTATESETTING_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PA2FloatLookAtGameStateSetting, value),
            },
        ],
    }),
    array_type: Some(PA2FLOATLOOKATGAMESTATESETTING_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PA2FloatLookAtGameStateSetting {
    fn type_info() -> &'static TypeInfo {
        PA2FLOATLOOKATGAMESTATESETTING_TYPE_INFO
    }
}


pub const PA2FLOATLOOKATGAMESTATESETTING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2FloatLookAtGameStateSetting-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("PA2FloatLookAtGameStateSetting-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PA2BoolLookAtGameStateSetting {
    pub value: bool,
}

pub const PA2BOOLLOOKATGAMESTATESETTING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2BoolLookAtGameStateSetting",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PA2LOOKATGAMESTATESETTING_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PA2BoolLookAtGameStateSetting, value),
            },
        ],
    }),
    array_type: Some(PA2BOOLLOOKATGAMESTATESETTING_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PA2BoolLookAtGameStateSetting {
    fn type_info() -> &'static TypeInfo {
        PA2BOOLLOOKATGAMESTATESETTING_TYPE_INFO
    }
}


pub const PA2BOOLLOOKATGAMESTATESETTING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2BoolLookAtGameStateSetting-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("PA2BoolLookAtGameStateSetting-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PA2LookAtGameStateSetting {
    pub game_state: super::ant::AntRef,
}

pub const PA2LOOKATGAMESTATESETTING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2LookAtGameStateSetting",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "GameState",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(PA2LookAtGameStateSetting, game_state),
            },
        ],
    }),
    array_type: Some(PA2LOOKATGAMESTATESETTING_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PA2LookAtGameStateSetting {
    fn type_info() -> &'static TypeInfo {
        PA2LOOKATGAMESTATESETTING_TYPE_INFO
    }
}


pub const PA2LOOKATGAMESTATESETTING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2LookAtGameStateSetting-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("PA2LookAtGameStateSetting-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PA2LookAtTrackData {
    pub keyframes: Vec<PA2LookAtKeyframe>,
}

pub const PA2LOOKATTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2LookAtTrackData",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACKDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Keyframes",
                flags: MemberInfoFlags::new(144),
                field_type: PA2LOOKATKEYFRAME_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(PA2LookAtTrackData, keyframes),
            },
        ],
    }),
    array_type: Some(PA2LOOKATTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PA2LookAtTrackData {
    fn type_info() -> &'static TypeInfo {
        PA2LOOKATTRACKDATA_TYPE_INFO
    }
}


pub const PA2LOOKATTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2LookAtTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("PA2LookAtTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PA2LookAtKeyframe {
    pub time: f32,
    pub target: PA2LookAtTarget,
    pub controller: i32,
    pub snap_to_target: bool,
}

pub const PA2LOOKATKEYFRAME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2LookAtKeyframe",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINEKEYFRAMEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Time",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PA2LookAtKeyframe, time),
            },
            FieldInfoData {
                name: "Target",
                flags: MemberInfoFlags::new(0),
                field_type: PA2LOOKATTARGET_TYPE_INFO,
                rust_offset: offset_of!(PA2LookAtKeyframe, target),
            },
            FieldInfoData {
                name: "Controller",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PA2LookAtKeyframe, controller),
            },
            FieldInfoData {
                name: "SnapToTarget",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PA2LookAtKeyframe, snap_to_target),
            },
        ],
    }),
    array_type: Some(PA2LOOKATKEYFRAME_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PA2LookAtKeyframe {
    fn type_info() -> &'static TypeInfo {
        PA2LOOKATKEYFRAME_TYPE_INFO
    }
}


pub const PA2LOOKATKEYFRAME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2LookAtKeyframe-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("PA2LookAtKeyframe-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PA2LookAtTarget {
    pub entity_track: super::timeline::EntityTrackBaseData,
}

pub const PA2LOOKATTARGET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2LookAtTarget",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "EntityTrack",
                flags: MemberInfoFlags::new(0),
                field_type: ENTITYTRACKBASEDATA_TYPE_INFO,
                rust_offset: offset_of!(PA2LookAtTarget, entity_track),
            },
        ],
    }),
    array_type: Some(PA2LOOKATTARGET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PA2LookAtTarget {
    fn type_info() -> &'static TypeInfo {
        PA2LOOKATTARGET_TYPE_INFO
    }
}


pub const PA2LOOKATTARGET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2LookAtTarget-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("PA2LookAtTarget-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LightingTrackData {
    pub children: Vec<LightTrackData>,
}

pub const LIGHTINGTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightingTrackData",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACKDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Children",
                flags: MemberInfoFlags::new(144),
                field_type: LIGHTTRACKDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LightingTrackData, children),
            },
        ],
    }),
    array_type: Some(LIGHTINGTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LightingTrackData {
    fn type_info() -> &'static TypeInfo {
        LIGHTINGTRACKDATA_TYPE_INFO
    }
}


pub const LIGHTINGTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightingTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("LightingTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LightTrackData {
}

pub const LIGHTTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightTrackData",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYTRACKBASEDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LIGHTTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LightTrackData {
    fn type_info() -> &'static TypeInfo {
        LIGHTTRACKDATA_TYPE_INFO
    }
}


pub const LIGHTTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("LightTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OrbisSystemCompanionUtilEventMessageBase {
}

pub const ORBISSYSTEMCOMPANIONUTILEVENTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OrbisSystemCompanionUtilEventMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for OrbisSystemCompanionUtilEventMessageBase {
    fn type_info() -> &'static TypeInfo {
        ORBISSYSTEMCOMPANIONUTILEVENTMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OrbisSystemCompanionHttpdEventMessageBase {
}

pub const ORBISSYSTEMCOMPANIONHTTPDEVENTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OrbisSystemCompanionHttpdEventMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for OrbisSystemCompanionHttpdEventMessageBase {
    fn type_info() -> &'static TypeInfo {
        ORBISSYSTEMCOMPANIONHTTPDEVENTMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OrbisSystemUserEventMessageBase {
}

pub const ORBISSYSTEMUSEREVENTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OrbisSystemUserEventMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for OrbisSystemUserEventMessageBase {
    fn type_info() -> &'static TypeInfo {
        ORBISSYSTEMUSEREVENTMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OrbisSystemTitleExternalEventMessageBase {
}

pub const ORBISSYSTEMTITLEEXTERNALEVENTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OrbisSystemTitleExternalEventMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for OrbisSystemTitleExternalEventMessageBase {
    fn type_info() -> &'static TypeInfo {
        ORBISSYSTEMTITLEEXTERNALEVENTMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OrbisSystemUiOverlaidStatusChangeEventMessage {
}

pub const ORBISSYSTEMUIOVERLAIDSTATUSCHANGEEVENTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OrbisSystemUiOverlaidStatusChangeEventMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for OrbisSystemUiOverlaidStatusChangeEventMessage {
    fn type_info() -> &'static TypeInfo {
        ORBISSYSTEMUIOVERLAIDSTATUSCHANGEEVENTMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OrbisSystemBackgroundStatusChangeEventMessage {
}

pub const ORBISSYSTEMBACKGROUNDSTATUSCHANGEEVENTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OrbisSystemBackgroundStatusChangeEventMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for OrbisSystemBackgroundStatusChangeEventMessage {
    fn type_info() -> &'static TypeInfo {
        ORBISSYSTEMBACKGROUNDSTATUSCHANGEEVENTMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PerfJournalBookmarkLabelEntityData {
    pub realm: super::core::Realm,
    pub bookmark_label_name: String,
    pub enable: bool,
}

pub const PERFJOURNALBOOKMARKLABELENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerfJournalBookmarkLabelEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(PerfJournalBookmarkLabelEntityData, realm),
            },
            FieldInfoData {
                name: "BookmarkLabelName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(PerfJournalBookmarkLabelEntityData, bookmark_label_name),
            },
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PerfJournalBookmarkLabelEntityData, enable),
            },
        ],
    }),
    array_type: Some(PERFJOURNALBOOKMARKLABELENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PerfJournalBookmarkLabelEntityData {
    fn type_info() -> &'static TypeInfo {
        PERFJOURNALBOOKMARKLABELENTITYDATA_TYPE_INFO
    }
}


pub const PERFJOURNALBOOKMARKLABELENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerfJournalBookmarkLabelEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("PerfJournalBookmarkLabelEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PerfJournalBookmarkEntityData {
    pub realm: super::core::Realm,
    pub enable: bool,
}

pub const PERFJOURNALBOOKMARKENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerfJournalBookmarkEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(PerfJournalBookmarkEntityData, realm),
            },
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PerfJournalBookmarkEntityData, enable),
            },
        ],
    }),
    array_type: Some(PERFJOURNALBOOKMARKENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PerfJournalBookmarkEntityData {
    fn type_info() -> &'static TypeInfo {
        PERFJOURNALBOOKMARKENTITYDATA_TYPE_INFO
    }
}


pub const PERFJOURNALBOOKMARKENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerfJournalBookmarkEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("PerfJournalBookmarkEntityData-Array"),
    array_type: None,
    alignment: 8,
};



pub const GETFLOATPROFILEOPTION_FLOAT32_LOCALPLAYERID_PROFILEOPTIONSTYPE_CSTRING_FLOAT32__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GetFloatProfileOption(Float32,LocalPlayerId,ProfileOptionsType,CString,Float32)",
    flags: MemberInfoFlags::new(793),
    module: "GameClient",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};



pub const GETINTPROFILEOPTION_INT32_LOCALPLAYERID_PROFILEOPTIONSTYPE_CSTRING_INT32__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GetIntProfileOption(Int32,LocalPlayerId,ProfileOptionsType,CString,Int32)",
    flags: MemberInfoFlags::new(793),
    module: "GameClient",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};



pub const GETBOOLPROFILEOPTION_BOOLEAN_LOCALPLAYERID_PROFILEOPTIONSTYPE_CSTRING_BOOLEAN__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GetBoolProfileOption(Boolean,LocalPlayerId,ProfileOptionsType,CString,Boolean)",
    flags: MemberInfoFlags::new(793),
    module: "GameClient",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIExitToMenuReasonMessage {
}

pub const UIEXITTOMENUREASONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIExitToMenuReasonMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UIExitToMenuReasonMessage {
    fn type_info() -> &'static TypeInfo {
        UIEXITTOMENUREASONMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCommanderPlayerChangedMessage {
}

pub const CLIENTCOMMANDERPLAYERCHANGEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCommanderPlayerChangedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientCommanderPlayerChangedMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTCOMMANDERPLAYERCHANGEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientStateChangingStateMessage {
}

pub const CLIENTSTATECHANGINGSTATEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStateChangingStateMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientStateChangingStateMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTSTATECHANGINGSTATEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientGameplayControllableLowHealthMessage {
}

pub const CLIENTGAMEPLAYCONTROLLABLELOWHEALTHMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameplayControllableLowHealthMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientGameplayControllableLowHealthMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTGAMEPLAYCONTROLLABLELOWHEALTHMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientGameplaySoldierHitMessage {
}

pub const CLIENTGAMEPLAYSOLDIERHITMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameplaySoldierHitMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientGameplaySoldierHitMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTGAMEPLAYSOLDIERHITMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCameraShowKillerMessage {
}

pub const CLIENTCAMERASHOWKILLERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCameraShowKillerMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientCameraShowKillerMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTCAMERASHOWKILLERMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCollisionProjectileImpactMessage {
}

pub const CLIENTCOLLISIONPROJECTILEIMPACTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCollisionProjectileImpactMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ClientCollisionProjectileImpactMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTCOLLISIONPROJECTILEIMPACTMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCollisionSpawnExplosionMessage {
}

pub const CLIENTCOLLISIONSPAWNEXPLOSIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCollisionSpawnExplosionMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ClientCollisionSpawnExplosionMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTCOLLISIONSPAWNEXPLOSIONMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCollisionExplosionPackDetonatedMessage {
}

pub const CLIENTCOLLISIONEXPLOSIONPACKDETONATEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCollisionExplosionPackDetonatedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientCollisionExplosionPackDetonatedMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTCOLLISIONEXPLOSIONPACKDETONATEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCollisionExplosionPackPlacedMessage {
}

pub const CLIENTCOLLISIONEXPLOSIONPACKPLACEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCollisionExplosionPackPlacedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientCollisionExplosionPackPlacedMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTCOLLISIONEXPLOSIONPACKPLACEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCollisionGrenadeCollisionMessage {
}

pub const CLIENTCOLLISIONGRENADECOLLISIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCollisionGrenadeCollisionMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ClientCollisionGrenadeCollisionMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTCOLLISIONGRENADECOLLISIONMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientSoundVoiceOverFinishedMessage {
}

pub const CLIENTSOUNDVOICEOVERFINISHEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoundVoiceOverFinishedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientSoundVoiceOverFinishedMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTSOUNDVOICEOVERFINISHEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientVehicleCriticalDamageMessage {
}

pub const CLIENTVEHICLECRITICALDAMAGEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVehicleCriticalDamageMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientVehicleCriticalDamageMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTVEHICLECRITICALDAMAGEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientEntityDummyToGetTheClientEntityMessageCategoryMessage {
}

pub const CLIENTENTITYDUMMYTOGETTHECLIENTENTITYMESSAGECATEGORYMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEntityDummyToGetTheClientEntityMessageCategoryMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientEntityDummyToGetTheClientEntityMessageCategoryMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTENTITYDUMMYTOGETTHECLIENTENTITYMESSAGECATEGORYMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCharacterSpawnDoneMessage {
}

pub const CLIENTCHARACTERSPAWNDONEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterSpawnDoneMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientCharacterSpawnDoneMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTCHARACTERSPAWNDONEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCharacterLocalPlayerDeletedMessage {
}

pub const CLIENTCHARACTERLOCALPLAYERDELETEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterLocalPlayerDeletedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientCharacterLocalPlayerDeletedMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTCHARACTERLOCALPLAYERDELETEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientControllableLocalPlayerSetMessage {
}

pub const CLIENTCONTROLLABLELOCALPLAYERSETMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientControllableLocalPlayerSetMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientControllableLocalPlayerSetMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTCONTROLLABLELOCALPLAYERSETMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientWeaponComponentWeaponOnUnspawnMessage {
}

pub const CLIENTWEAPONCOMPONENTWEAPONONUNSPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponComponentWeaponOnUnspawnMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientWeaponComponentWeaponOnUnspawnMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTWEAPONCOMPONENTWEAPONONUNSPAWNMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientWeaponComponentWeaponOnSpawnMessage {
}

pub const CLIENTWEAPONCOMPONENTWEAPONONSPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponComponentWeaponOnSpawnMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientWeaponComponentWeaponOnSpawnMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTWEAPONCOMPONENTWEAPONONSPAWNMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientVehicleSetRemoteControlledDamageGiverMessage {
}

pub const CLIENTVEHICLESETREMOTECONTROLLEDDAMAGEGIVERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVehicleSetRemoteControlledDamageGiverMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientVehicleSetRemoteControlledDamageGiverMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTVEHICLESETREMOTECONTROLLEDDAMAGEGIVERMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientVehicleSpawnDoneMessage {
}

pub const CLIENTVEHICLESPAWNDONEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVehicleSpawnDoneMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientVehicleSpawnDoneMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTVEHICLESPAWNDONEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPlayerUnlocksUpdatedMessage {
}

pub const CLIENTPLAYERUNLOCKSUPDATEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerUnlocksUpdatedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientPlayerUnlocksUpdatedMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTPLAYERUNLOCKSUPDATEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPlayerOnWeaponUndeployFinishedMessage {
}

pub const CLIENTPLAYERONWEAPONUNDEPLOYFINISHEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerOnWeaponUndeployFinishedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientPlayerOnWeaponUndeployFinishedMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTPLAYERONWEAPONUNDEPLOYFINISHEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPlayerSelectedCustomizationMessage {
}

pub const CLIENTPLAYERSELECTEDCUSTOMIZATIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerSelectedCustomizationMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientPlayerSelectedCustomizationMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTPLAYERSELECTEDCUSTOMIZATIONMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPlayerSelectedUnlocksMessage {
}

pub const CLIENTPLAYERSELECTEDUNLOCKSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerSelectedUnlocksMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientPlayerSelectedUnlocksMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTPLAYERSELECTEDUNLOCKSMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPlayerManDownMessage {
}

pub const CLIENTPLAYERMANDOWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerManDownMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientPlayerManDownMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTPLAYERMANDOWNMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPlayerWeaponPickupMessage {
}

pub const CLIENTPLAYERWEAPONPICKUPMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerWeaponPickupMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientPlayerWeaponPickupMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTPLAYERWEAPONPICKUPMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPlayerAmmoPickupMessage {
}

pub const CLIENTPLAYERAMMOPICKUPMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerAmmoPickupMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientPlayerAmmoPickupMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTPLAYERAMMOPICKUPMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPlayerKilledMessage {
}

pub const CLIENTPLAYERKILLEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerKilledMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientPlayerKilledMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTPLAYERKILLEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPlayerEnterExitVehicleMessage {
}

pub const CLIENTPLAYERENTEREXITVEHICLEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerEnterExitVehicleMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientPlayerEnterExitVehicleMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTPLAYERENTEREXITVEHICLEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientMetricsPauseGameMessage {
}

pub const CLIENTMETRICSPAUSEGAMEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMetricsPauseGameMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientMetricsPauseGameMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTMETRICSPAUSEGAMEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientMetricsUIActionMessage {
}

pub const CLIENTMETRICSUIACTIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMetricsUIActionMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameClient",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientMetricsUIActionMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTMETRICSUIACTIONMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientMetricsPopUIScreenMessage {
}

pub const CLIENTMETRICSPOPUISCREENMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMetricsPopUIScreenMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientMetricsPopUIScreenMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTMETRICSPOPUISCREENMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientMetricsPushUIScreenMessage {
}

pub const CLIENTMETRICSPUSHUISCREENMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMetricsPushUIScreenMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientMetricsPushUIScreenMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTMETRICSPUSHUISCREENMESSAGE_TYPE_INFO
    }
}

#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ClientGameType {
    #[default]
    ClientGameType_SinglePlayer = 0,
    ClientGameType_Hosted = 1,
    ClientGameType_MpTutorial = 2,
    ClientGameType_Joined = 3,
}

pub const CLIENTGAMETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameType",
    flags: MemberInfoFlags::new(49429),
    module: "GameClient",
    data: TypeInfoData::Enum,
    array_type: Some(CLIENTGAMETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ClientGameType {
    fn type_info() -> &'static TypeInfo {
        CLIENTGAMETYPE_TYPE_INFO
    }
}


pub const CLIENTGAMETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameType-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientGameType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ClientState {
    #[default]
    ClientState_WaitingForStaticBundleLoad = 0,
    ClientState_LoadProfileOptions = 1,
    ClientState_LostConnection = 2,
    ClientState_WaitingForUnload = 3,
    ClientState_Startup = 4,
    ClientState_StartServer = 5,
    ClientState_WaitingForLevel = 6,
    ClientState_StartLoadingLevel = 7,
    ClientState_WaitingForLevelLoaded = 8,
    ClientState_WaitingForLevelLink = 9,
    ClientState_LevelLinked = 10,
    ClientState_WaitingForGhosts = 11,
    ClientState_Ingame = 12,
    ClientState_LeaveIngame = 13,
    ClientState_ConnectToServer = 14,
    ClientState_ShuttingDown = 15,
    ClientState_Shutdown = 16,
    ClientState_None = 17,
}

pub const CLIENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientState",
    flags: MemberInfoFlags::new(49429),
    module: "GameClient",
    data: TypeInfoData::Enum,
    array_type: Some(CLIENTSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ClientState {
    fn type_info() -> &'static TypeInfo {
        CLIENTSTATE_TYPE_INFO
    }
}


pub const CLIENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SubLevelVisibleEntityData {
}

pub const SUBLEVELVISIBLEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelVisibleEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SUBLEVELVISIBLEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SubLevelVisibleEntityData {
    fn type_info() -> &'static TypeInfo {
        SUBLEVELVISIBLEENTITYDATA_TYPE_INFO
    }
}


pub const SUBLEVELVISIBLEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelVisibleEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("SubLevelVisibleEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientGroupComponent {
}

pub const CLIENTGROUPCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGroupComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTGROUPCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientGroupComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTGROUPCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTGROUPCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGroupComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientGroupComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientEventSyncEntity {
}

pub const CLIENTEVENTSYNCENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEventSyncEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTEVENTSYNCENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientEventSyncEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTEVENTSYNCENTITY_TYPE_INFO
    }
}


pub const CLIENTEVENTSYNCENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEventSyncEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientEventSyncEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientBlueprintEntity {
}

pub const CLIENTBLUEPRINTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBlueprintEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTBLUEPRINTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientBlueprintEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTBLUEPRINTENTITY_TYPE_INFO
    }
}


pub const CLIENTBLUEPRINTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBlueprintEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientBlueprintEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VoiceOverManagerControlEntity {
}

pub const VOICEOVERMANAGERCONTROLENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceOverManagerControlEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VOICEOVERMANAGERCONTROLENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for VoiceOverManagerControlEntity {
    fn type_info() -> &'static TypeInfo {
        VOICEOVERMANAGERCONTROLENTITY_TYPE_INFO
    }
}


pub const VOICEOVERMANAGERCONTROLENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceOverManagerControlEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("VoiceOverManagerControlEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VoiceOverEventEntity {
}

pub const VOICEOVEREVENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceOverEventEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VOICEOVEREVENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for VoiceOverEventEntity {
    fn type_info() -> &'static TypeInfo {
        VOICEOVEREVENTENTITY_TYPE_INFO
    }
}


pub const VOICEOVEREVENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceOverEventEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("VoiceOverEventEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VoiceOverConversationEntity {
}

pub const VOICEOVERCONVERSATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceOverConversationEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VOICEOVERCONVERSATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for VoiceOverConversationEntity {
    fn type_info() -> &'static TypeInfo {
        VOICEOVERCONVERSATIONENTITY_TYPE_INFO
    }
}


pub const VOICEOVERCONVERSATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceOverConversationEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("VoiceOverConversationEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientVoiceOverAnimationComponent {
}

pub const CLIENTVOICEOVERANIMATIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverAnimationComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVOICEOVERANIMATIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientVoiceOverAnimationComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTVOICEOVERANIMATIONCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTVOICEOVERANIMATIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverAnimationComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientVoiceOverAnimationComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SoundListenerEntity {
}

pub const SOUNDLISTENERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoundListenerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SOUNDLISTENERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SoundListenerEntity {
    fn type_info() -> &'static TypeInfo {
        SOUNDLISTENERENTITY_TYPE_INFO
    }
}


pub const SOUNDLISTENERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoundListenerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("SoundListenerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SoundAreaEntity {
}

pub const SOUNDAREAENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoundAreaEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SOUNDAREAENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SoundAreaEntity {
    fn type_info() -> &'static TypeInfo {
        SOUNDAREAENTITY_TYPE_INFO
    }
}


pub const SOUNDAREAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoundAreaEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("SoundAreaEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DetachedSoundListenerEntity {
}

pub const DETACHEDSOUNDLISTENERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DetachedSoundListenerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DETACHEDSOUNDLISTENERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DetachedSoundListenerEntity {
    fn type_info() -> &'static TypeInfo {
        DETACHEDSOUNDLISTENERENTITY_TYPE_INFO
    }
}


pub const DETACHEDSOUNDLISTENERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DetachedSoundListenerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("DetachedSoundListenerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientWaterPhysicsComponent {
}

pub const CLIENTWATERPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWaterPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWATERPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWaterPhysicsComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTWATERPHYSICSCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTWATERPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWaterPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientWaterPhysicsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientWaterHealthComponent {
}

pub const CLIENTWATERHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWaterHealthComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMEHEALTHCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWATERHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWaterHealthComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTWATERHEALTHCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTWATERHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWaterHealthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientWaterHealthComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientTerrainPhysicsComponent {
}

pub const CLIENTTERRAINPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTerrainPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTTERRAINPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientTerrainPhysicsComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTTERRAINPHYSICSCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTTERRAINPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTerrainPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientTerrainPhysicsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientTerrainHealthComponent {
}

pub const CLIENTTERRAINHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTerrainHealthComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMEHEALTHCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTTERRAINHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientTerrainHealthComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTTERRAINHEALTHCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTTERRAINHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTerrainHealthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientTerrainHealthComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientStaticModelPhysicsComponent {
}

pub const CLIENTSTATICMODELPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticModelPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PARTPHYSICSCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSTATICMODELPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientStaticModelPhysicsComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTSTATICMODELPHYSICSCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTSTATICMODELPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticModelPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientStaticModelPhysicsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientStaticModelHealthComponent {
}

pub const CLIENTSTATICMODELHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticModelHealthComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMEHEALTHCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSTATICMODELHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientStaticModelHealthComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTSTATICMODELHEALTHCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTSTATICMODELHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticModelHealthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientStaticModelHealthComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientStaticModelGroupPhysicsComponent {
}

pub const CLIENTSTATICMODELGROUPPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticModelGroupPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GROUPPHYSICSCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSTATICMODELGROUPPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientStaticModelGroupPhysicsComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTSTATICMODELGROUPPHYSICSCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTSTATICMODELGROUPPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticModelGroupPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientStaticModelGroupPhysicsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientStaticModelGroupHealthComponent {
}

pub const CLIENTSTATICMODELGROUPHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticModelGroupHealthComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMEHEALTHCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSTATICMODELGROUPHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientStaticModelGroupHealthComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTSTATICMODELGROUPHEALTHCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTSTATICMODELGROUPHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticModelGroupHealthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientStaticModelGroupHealthComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientBoxReflectionVolumeComponent {
}

pub const CLIENTBOXREFLECTIONVOLUMECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBoxReflectionVolumeComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTREFLECTIONVOLUMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTBOXREFLECTIONVOLUMECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientBoxReflectionVolumeComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTBOXREFLECTIONVOLUMECOMPONENT_TYPE_INFO
    }
}


pub const CLIENTBOXREFLECTIONVOLUMECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBoxReflectionVolumeComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientBoxReflectionVolumeComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientSphereReflectionVolumeComponent {
}

pub const CLIENTSPHEREREFLECTIONVOLUMECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSphereReflectionVolumeComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTREFLECTIONVOLUMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSPHEREREFLECTIONVOLUMECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSphereReflectionVolumeComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTSPHEREREFLECTIONVOLUMECOMPONENT_TYPE_INFO
    }
}


pub const CLIENTSPHEREREFLECTIONVOLUMECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSphereReflectionVolumeComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientSphereReflectionVolumeComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientReflectionVolumeComponent {
}

pub const CLIENTREFLECTIONVOLUMECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientReflectionVolumeComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTREFLECTIONVOLUMECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientReflectionVolumeComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTREFLECTIONVOLUMECOMPONENT_TYPE_INFO
    }
}


pub const CLIENTREFLECTIONVOLUMECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientReflectionVolumeComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientReflectionVolumeComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientLightComponent {
}

pub const CLIENTLIGHTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLightComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTLIGHTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientLightComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTLIGHTCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTLIGHTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLightComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientLightComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientLensFlareComponent {
}

pub const CLIENTLENSFLARECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLensFlareComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTLENSFLARECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientLensFlareComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTLENSFLARECOMPONENT_TYPE_INFO
    }
}


pub const CLIENTLENSFLARECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLensFlareComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientLensFlareComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientEffectComponent {
}

pub const CLIENTEFFECTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEffectComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTEFFECTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientEffectComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTEFFECTCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTEFFECTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEffectComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientEffectComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientDynamicModelPhysicsComponent {
}

pub const CLIENTDYNAMICMODELPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDynamicModelPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PARTPHYSICSCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTDYNAMICMODELPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientDynamicModelPhysicsComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTDYNAMICMODELPHYSICSCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTDYNAMICMODELPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDynamicModelPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientDynamicModelPhysicsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientDynamicModelHealthComponent {
}

pub const CLIENTDYNAMICMODELHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDynamicModelHealthComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMEHEALTHCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTDYNAMICMODELHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientDynamicModelHealthComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTDYNAMICMODELHEALTHCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTDYNAMICMODELHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDynamicModelHealthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientDynamicModelHealthComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientBangerPhysicsComponent {
}

pub const CLIENTBANGERPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBangerPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PARTPHYSICSCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTBANGERPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientBangerPhysicsComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTBANGERPHYSICSCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTBANGERPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBangerPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientBangerPhysicsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientBangerHealthComponent {
}

pub const CLIENTBANGERHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBangerHealthComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMEHEALTHCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTBANGERHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientBangerHealthComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTBANGERHEALTHCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTBANGERHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBangerHealthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientBangerHealthComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientSpectatorSubView {
}

pub const CLIENTSPECTATORSUBVIEW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSpectatorSubView",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTSUBVIEW_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSPECTATORSUBVIEW_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSpectatorSubView {
    fn type_info() -> &'static TypeInfo {
        CLIENTSPECTATORSUBVIEW_TYPE_INFO
    }
}


pub const CLIENTSPECTATORSUBVIEW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSpectatorSubView-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientSpectatorSubView-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientGameSplineEntity {
}

pub const CLIENTGAMESPLINEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameSplineEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTGAMESPLINEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientGameSplineEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTGAMESPLINEENTITY_TYPE_INFO
    }
}


pub const CLIENTGAMESPLINEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameSplineEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientGameSplineEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientAreaImmunityComponent {
}

pub const CLIENTAREAIMMUNITYCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAreaImmunityComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTAREAIMMUNITYCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAreaImmunityComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTAREAIMMUNITYCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTAREAIMMUNITYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAreaImmunityComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientAreaImmunityComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientDynamicFireEntity {
}

pub const CLIENTDYNAMICFIREENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDynamicFireEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENTENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTDYNAMICFIREENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientDynamicFireEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTDYNAMICFIREENTITY_TYPE_INFO
    }
}


pub const CLIENTDYNAMICFIREENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDynamicFireEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientDynamicFireEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientWarningSystemComponent {
}

pub const CLIENTWARNINGSYSTEMCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWarningSystemComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWARNINGSYSTEMCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWarningSystemComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTWARNINGSYSTEMCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTWARNINGSYSTEMCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWarningSystemComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientWarningSystemComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientUnlockComponent {
}

pub const CLIENTUNLOCKCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientUnlockComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTUNLOCKCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientUnlockComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTUNLOCKCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTUNLOCKCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientUnlockComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientUnlockComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPlayerEntryComponent {
}

pub const CLIENTPLAYERENTRYCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerEntryComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMEENTRYCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPLAYERENTRYCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPlayerEntryComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTPLAYERENTRYCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTPLAYERENTRYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerEntryComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientPlayerEntryComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientGameEntryComponent {
}

pub const CLIENTGAMEENTRYCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameEntryComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTENTRYCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTGAMEENTRYCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientGameEntryComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTGAMEENTRYCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTGAMEENTRYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameEntryComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientGameEntryComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCharacterEntity {
}

pub const CLIENTCHARACTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTCONTROLLABLEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCHARACTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCharacterEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTCHARACTERENTITY_TYPE_INFO
    }
}


pub const CLIENTCHARACTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientCharacterEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientWarpAnimationComponent {
}

pub const CLIENTWARPANIMATIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWarpAnimationComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWARPANIMATIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWarpAnimationComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTWARPANIMATIONCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTWARPANIMATIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWarpAnimationComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientWarpAnimationComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientVehicleEntryListenerComponent {
}

pub const CLIENTVEHICLEENTRYLISTENERCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVehicleEntryListenerComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVEHICLEENTRYLISTENERCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientVehicleEntryListenerComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTVEHICLEENTRYLISTENERCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTVEHICLEENTRYLISTENERCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVehicleEntryListenerComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientVehicleEntryListenerComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCharacterPhysicsComponent {
}

pub const CLIENTCHARACTERPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCHARACTERPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCharacterPhysicsComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTCHARACTERPHYSICSCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTCHARACTERPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientCharacterPhysicsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCharacterMeshComponent {
}

pub const CLIENTCHARACTERMESHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterMeshComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCHARACTERMESHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCharacterMeshComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTCHARACTERMESHCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTCHARACTERMESHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterMeshComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientCharacterMeshComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCharacterMasterPhysicsComponent {
}

pub const CLIENTCHARACTERMASTERPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterMasterPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CHARACTERPHYSICSCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCHARACTERMASTERPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCharacterMasterPhysicsComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTCHARACTERMASTERPHYSICSCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTCHARACTERMASTERPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterMasterPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientCharacterMasterPhysicsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCharacterHealthComponent {
}

pub const CLIENTCHARACTERHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterHealthComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTCONTROLLABLEHEALTHCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCHARACTERHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCharacterHealthComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTCHARACTERHEALTHCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTCHARACTERHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterHealthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientCharacterHealthComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCharacterCustomizationComponent {
}

pub const CLIENTCHARACTERCUSTOMIZATIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterCustomizationComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCHARACTERCUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCharacterCustomizationComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTCHARACTERCUSTOMIZATIONCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTCHARACTERCUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterCustomizationComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientCharacterCustomizationComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCharacterCameraComponent {
}

pub const CLIENTCHARACTERCAMERACOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterCameraComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCHARACTERCAMERACOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCharacterCameraComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTCHARACTERCAMERACOMPONENT_TYPE_INFO
    }
}


pub const CLIENTCHARACTERCAMERACOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterCameraComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientCharacterCameraComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CharacterTargetCameraCallback {
}

pub const CHARACTERTARGETCAMERACALLBACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterTargetCameraCallback",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TARGETCAMERACALLBACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CHARACTERTARGETCAMERACALLBACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CharacterTargetCameraCallback {
    fn type_info() -> &'static TypeInfo {
        CHARACTERTARGETCAMERACALLBACK_TYPE_INFO
    }
}


pub const CHARACTERTARGETCAMERACALLBACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterTargetCameraCallback-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("CharacterTargetCameraCallback-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCameraRelativeInputComponent {
}

pub const CLIENTCAMERARELATIVEINPUTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCameraRelativeInputComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCAMERARELATIVEINPUTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCameraRelativeInputComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTCAMERARELATIVEINPUTCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTCAMERARELATIVEINPUTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCameraRelativeInputComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientCameraRelativeInputComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientAntInputComponent {
}

pub const CLIENTANTINPUTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAntInputComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTANTINPUTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAntInputComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTANTINPUTCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTANTINPUTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAntInputComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientAntInputComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPlayVFXEntity {
}

pub const CLIENTPLAYVFXENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayVFXEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPLAYVFXENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPlayVFXEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTPLAYVFXENTITY_TYPE_INFO
    }
}


pub const CLIENTPLAYVFXENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayVFXEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientPlayVFXEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCharacterSpawnEntity {
}

pub const CLIENTCHARACTERSPAWNENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterSpawnEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTSPAWNENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCHARACTERSPAWNENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCharacterSpawnEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTCHARACTERSPAWNENTITY_TYPE_INFO
    }
}


pub const CLIENTCHARACTERSPAWNENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterSpawnEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientCharacterSpawnEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientTeamFilterEntity {
}

pub const CLIENTTEAMFILTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTeamFilterEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTTEAMFILTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientTeamFilterEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTTEAMFILTERENTITY_TYPE_INFO
    }
}


pub const CLIENTTEAMFILTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTeamFilterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientTeamFilterEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientSyncedSequenceEntity {
}

pub const CLIENTSYNCEDSEQUENCEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSyncedSequenceEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SEQUENCEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSYNCEDSEQUENCEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSyncedSequenceEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTSYNCEDSEQUENCEENTITY_TYPE_INFO
    }
}


pub const CLIENTSYNCEDSEQUENCEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSyncedSequenceEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientSyncedSequenceEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientStreamingGateEntity {
}

pub const CLIENTSTREAMINGGATEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStreamingGateEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSTREAMINGGATEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientStreamingGateEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTSTREAMINGGATEENTITY_TYPE_INFO
    }
}


pub const CLIENTSTREAMINGGATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStreamingGateEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientStreamingGateEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPlayerIteratorEntity {
}

pub const CLIENTPLAYERITERATORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerIteratorEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PLAYERITERATORENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPLAYERITERATORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPlayerIteratorEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTPLAYERITERATORENTITY_TYPE_INFO
    }
}


pub const CLIENTPLAYERITERATORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerIteratorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientPlayerIteratorEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientLocalPlayerFilterEntity {
}

pub const CLIENTLOCALPLAYERFILTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLocalPlayerFilterEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTLOCALPLAYERFILTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientLocalPlayerFilterEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTLOCALPLAYERFILTERENTITY_TYPE_INFO
    }
}


pub const CLIENTLOCALPLAYERFILTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLocalPlayerFilterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientLocalPlayerFilterEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPlayerFilterEntity {
}

pub const CLIENTPLAYERFILTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerFilterEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPLAYERFILTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPlayerFilterEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTPLAYERFILTERENTITY_TYPE_INFO
    }
}


pub const CLIENTPLAYERFILTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerFilterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientPlayerFilterEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientObjectiveEntity {
}

pub const CLIENTOBJECTIVEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientObjectiveEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTOBJECTIVEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientObjectiveEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTOBJECTIVEENTITY_TYPE_INFO
    }
}


pub const CLIENTOBJECTIVEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientObjectiveEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientObjectiveEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientObjectAreaQueryEntity {
}

pub const CLIENTOBJECTAREAQUERYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientObjectAreaQueryEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(OBJECTAREAQUERYENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTOBJECTAREAQUERYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientObjectAreaQueryEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTOBJECTAREAQUERYENTITY_TYPE_INFO
    }
}


pub const CLIENTOBJECTAREAQUERYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientObjectAreaQueryEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientObjectAreaQueryEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientMessageEntity {
}

pub const CLIENTMESSAGEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMessageEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTMESSAGEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientMessageEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTMESSAGEENTITY_TYPE_INFO
    }
}


pub const CLIENTMESSAGEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMessageEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientMessageEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientMapMarkerEntity {
}

pub const CLIENTMAPMARKERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMapMarkerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTMAPMARKERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientMapMarkerEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTMAPMARKERENTITY_TYPE_INFO
    }
}


pub const CLIENTMAPMARKERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMapMarkerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientMapMarkerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientLocalPlayerEventEntity {
}

pub const CLIENTLOCALPLAYEREVENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLocalPlayerEventEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTLOCALPLAYEREVENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientLocalPlayerEventEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTLOCALPLAYEREVENTENTITY_TYPE_INFO
    }
}


pub const CLIENTLOCALPLAYEREVENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLocalPlayerEventEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientLocalPlayerEventEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientEventMemoryEntity {
}

pub const CLIENTEVENTMEMORYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEventMemoryEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTEVENTMEMORYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientEventMemoryEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTEVENTMEMORYENTITY_TYPE_INFO
    }
}


pub const CLIENTEVENTMEMORYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEventMemoryEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientEventMemoryEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientFadeTrack {
}

pub const CLIENTFADETRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientFadeTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTFADETRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientFadeTrack {
    fn type_info() -> &'static TypeInfo {
        CLIENTFADETRACK_TYPE_INFO
    }
}


pub const CLIENTFADETRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientFadeTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientFadeTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientFadeEntity {
}

pub const CLIENTFADEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientFadeEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTFADEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientFadeEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTFADEENTITY_TYPE_INFO
    }
}


pub const CLIENTFADEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientFadeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientFadeEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientDoublePlayerEvent {
}

pub const CLIENTDOUBLEPLAYEREVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDoublePlayerEvent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTPLAYEREVENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTDOUBLEPLAYEREVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientDoublePlayerEvent {
    fn type_info() -> &'static TypeInfo {
        CLIENTDOUBLEPLAYEREVENT_TYPE_INFO
    }
}


pub const CLIENTDOUBLEPLAYEREVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDoublePlayerEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientDoublePlayerEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientTeamEntity {
}

pub const CLIENTTEAMENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTeamEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTTEAMENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientTeamEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTTEAMENTITY_TYPE_INFO
    }
}


pub const CLIENTTEAMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTeamEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientTeamEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientLadderComponent {
}

pub const CLIENTLADDERCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLadderComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTLADDERCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientLadderComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTLADDERCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTLADDERCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLadderComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientLadderComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientMultipleActorScenarioEntity {
}

pub const CLIENTMULTIPLEACTORSCENARIOENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMultipleActorScenarioEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTMULTIPLEACTORSCENARIOENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientMultipleActorScenarioEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTMULTIPLEACTORSCENARIOENTITY_TYPE_INFO
    }
}


pub const CLIENTMULTIPLEACTORSCENARIOENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMultipleActorScenarioEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientMultipleActorScenarioEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientModelAnimationEntity {
}

pub const CLIENTMODELANIMATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientModelAnimationEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MODELANIMATIONENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTMODELANIMATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientModelAnimationEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTMODELANIMATIONENTITY_TYPE_INFO
    }
}


pub const CLIENTMODELANIMATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientModelAnimationEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientModelAnimationEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientJointValidationEntity {
}

pub const CLIENTJOINTVALIDATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientJointValidationEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTJOINTVALIDATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientJointValidationEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTJOINTVALIDATIONENTITY_TYPE_INFO
    }
}


pub const CLIENTJOINTVALIDATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientJointValidationEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientJointValidationEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientFbProxyControllerEntity {
}

pub const CLIENTFBPROXYCONTROLLERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientFbProxyControllerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTFBPROXYCONTROLLERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientFbProxyControllerEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTFBPROXYCONTROLLERENTITY_TYPE_INFO
    }
}


pub const CLIENTFBPROXYCONTROLLERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientFbProxyControllerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientFbProxyControllerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientFaceposerEntity {
}

pub const CLIENTFACEPOSERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientFaceposerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTFACEPOSERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientFaceposerEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTFACEPOSERENTITY_TYPE_INFO
    }
}


pub const CLIENTFACEPOSERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientFaceposerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientFaceposerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCharacterInVehicleScenarioEntity {
}

pub const CLIENTCHARACTERINVEHICLESCENARIOENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterInVehicleScenarioEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCHARACTERINVEHICLESCENARIOENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCharacterInVehicleScenarioEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTCHARACTERINVEHICLESCENARIOENTITY_TYPE_INFO
    }
}


pub const CLIENTCHARACTERINVEHICLESCENARIOENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterInVehicleScenarioEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientCharacterInVehicleScenarioEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientANTSignalTrack {
}

pub const CLIENTANTSIGNALTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientANTSignalTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ANTSIGNALTRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTANTSIGNALTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientANTSignalTrack {
    fn type_info() -> &'static TypeInfo {
        CLIENTANTSIGNALTRACK_TYPE_INFO
    }
}


pub const CLIENTANTSIGNALTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientANTSignalTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientANTSignalTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientWriteAntGameStateEntity {
}

pub const CLIENTWRITEANTGAMESTATEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWriteAntGameStateEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWRITEANTGAMESTATEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWriteAntGameStateEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTWRITEANTGAMESTATEENTITY_TYPE_INFO
    }
}


pub const CLIENTWRITEANTGAMESTATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWriteAntGameStateEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientWriteAntGameStateEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientReadAntGameStateEntity {
}

pub const CLIENTREADANTGAMESTATEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientReadAntGameStateEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTREADANTGAMESTATEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientReadAntGameStateEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTREADANTGAMESTATEENTITY_TYPE_INFO
    }
}


pub const CLIENTREADANTGAMESTATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientReadAntGameStateEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientReadAntGameStateEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientAntEventEntity {
}

pub const CLIENTANTEVENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAntEventEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTANTEVENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAntEventEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTANTEVENTENTITY_TYPE_INFO
    }
}


pub const CLIENTANTEVENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAntEventEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientAntEventEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientANTControlTrack {
}

pub const CLIENTANTCONTROLTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientANTControlTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ANTCONTROLTRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTANTCONTROLTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientANTControlTrack {
    fn type_info() -> &'static TypeInfo {
        CLIENTANTCONTROLTRACK_TYPE_INFO
    }
}


pub const CLIENTANTCONTROLTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientANTControlTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientANTControlTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientAntAnimatableEntity {
}

pub const CLIENTANTANIMATABLEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAntAnimatableEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ANTANIMATABLEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTANTANIMATABLEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAntAnimatableEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTANTANIMATABLEENTITY_TYPE_INFO
    }
}


pub const CLIENTANTANIMATABLEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAntAnimatableEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientAntAnimatableEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientAnimationSignalEntity {
}

pub const CLIENTANIMATIONSIGNALENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAnimationSignalEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTANIMATIONSIGNALENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAnimationSignalEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTANIMATIONSIGNALENTITY_TYPE_INFO
    }
}


pub const CLIENTANIMATIONSIGNALENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAnimationSignalEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientAnimationSignalEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientAnimationPoseTrack {
}

pub const CLIENTANIMATIONPOSETRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAnimationPoseTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ANIMATIONPOSETRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTANIMATIONPOSETRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAnimationPoseTrack {
    fn type_info() -> &'static TypeInfo {
        CLIENTANIMATIONPOSETRACK_TYPE_INFO
    }
}


pub const CLIENTANIMATIONPOSETRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAnimationPoseTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientAnimationPoseTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientAnimationEnumerationChoiceEntity {
}

pub const CLIENTANIMATIONENUMERATIONCHOICEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAnimationEnumerationChoiceEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTANIMATIONENUMERATIONCHOICEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAnimationEnumerationChoiceEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTANIMATIONENUMERATIONCHOICEENTITY_TYPE_INFO
    }
}


pub const CLIENTANIMATIONENUMERATIONCHOICEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAnimationEnumerationChoiceEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientAnimationEnumerationChoiceEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientAnimationEnumerationEntity {
}

pub const CLIENTANIMATIONENUMERATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAnimationEnumerationEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTANIMATIONENUMERATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAnimationEnumerationEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTANIMATIONENUMERATIONENTITY_TYPE_INFO
    }
}


pub const CLIENTANIMATIONENUMERATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAnimationEnumerationEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientAnimationEnumerationEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCharacterPoseValidateTrack {
}

pub const CLIENTCHARACTERPOSEVALIDATETRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterPoseValidateTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LINKTRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCHARACTERPOSEVALIDATETRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCharacterPoseValidateTrack {
    fn type_info() -> &'static TypeInfo {
        CLIENTCHARACTERPOSEVALIDATETRACK_TYPE_INFO
    }
}


pub const CLIENTCHARACTERPOSEVALIDATETRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterPoseValidateTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientCharacterPoseValidateTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCharacterPoseRecordTrack {
}

pub const CLIENTCHARACTERPOSERECORDTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterPoseRecordTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LINKTRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCHARACTERPOSERECORDTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCharacterPoseRecordTrack {
    fn type_info() -> &'static TypeInfo {
        CLIENTCHARACTERPOSERECORDTRACK_TYPE_INFO
    }
}


pub const CLIENTCHARACTERPOSERECORDTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterPoseRecordTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientCharacterPoseRecordTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MeshEmitterControlEntity {
}

pub const MESHEMITTERCONTROLENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshEmitterControlEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(MESHEMITTERCONTROLENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MeshEmitterControlEntity {
    fn type_info() -> &'static TypeInfo {
        MESHEMITTERCONTROLENTITY_TYPE_INFO
    }
}


pub const MESHEMITTERCONTROLENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshEmitterControlEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("MeshEmitterControlEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientRotorComponent {
}

pub const CLIENTROTORCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientRotorComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTROTORCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientRotorComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTROTORCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTROTORCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientRotorComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientRotorComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientMeshComponent {
}

pub const CLIENTMESHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMeshComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTMESHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientMeshComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTMESHCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTMESHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMeshComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientMeshComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientIKComponent {
}

pub const CLIENTIKCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientIKComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTIKCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientIKComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTIKCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTIKCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientIKComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientIKComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientFakePhysicsMeshComponent {
}

pub const CLIENTFAKEPHYSICSMESHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientFakePhysicsMeshComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTMESHCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTFAKEPHYSICSMESHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientFakePhysicsMeshComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTFAKEPHYSICSMESHCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTFAKEPHYSICSMESHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientFakePhysicsMeshComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientFakePhysicsMeshComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientEngineComponent {
}

pub const CLIENTENGINECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEngineComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTENGINECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientEngineComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTENGINECOMPONENT_TYPE_INFO
    }
}


pub const CLIENTENGINECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEngineComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientEngineComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientChildComponent {
}

pub const CLIENTCHILDCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientChildComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCHILDCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientChildComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTCHILDCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTCHILDCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientChildComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientChildComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientChildBarrelComponent {
}

pub const CLIENTCHILDBARRELCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientChildBarrelComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTCHILDCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCHILDBARRELCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientChildBarrelComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTCHILDBARRELCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTCHILDBARRELCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientChildBarrelComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientChildBarrelComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientChassisComponent {
}

pub const CLIENTCHASSISCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientChassisComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTPARTCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCHASSISCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientChassisComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTCHASSISCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTCHASSISCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientChassisComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientChassisComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCameraComponent {
}

pub const CLIENTCAMERACOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCameraComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCAMERACOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCameraComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTCAMERACOMPONENT_TYPE_INFO
    }
}


pub const CLIENTCAMERACOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCameraComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientCameraComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientComponentTargetCameraCallback {
}

pub const CLIENTCOMPONENTTARGETCAMERACALLBACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientComponentTargetCameraCallback",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TARGETCAMERACALLBACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCOMPONENTTARGETCAMERACALLBACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientComponentTargetCameraCallback {
    fn type_info() -> &'static TypeInfo {
        CLIENTCOMPONENTTARGETCAMERACALLBACK_TYPE_INFO
    }
}


pub const CLIENTCOMPONENTTARGETCAMERACALLBACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientComponentTargetCameraCallback-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientComponentTargetCameraCallback-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientGamePlayerExtent {
}

pub const CLIENTGAMEPLAYEREXTENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGamePlayerExtent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTPLAYEREXTENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTGAMEPLAYEREXTENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientGamePlayerExtent {
    fn type_info() -> &'static TypeInfo {
        CLIENTGAMEPLAYEREXTENT_TYPE_INFO
    }
}


pub const CLIENTGAMEPLAYEREXTENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGamePlayerExtent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientGamePlayerExtent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientGamePlayerInternalExtent {
}

pub const CLIENTGAMEPLAYERINTERNALEXTENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGamePlayerInternalExtent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTPLAYEREXTENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTGAMEPLAYERINTERNALEXTENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientGamePlayerInternalExtent {
    fn type_info() -> &'static TypeInfo {
        CLIENTGAMEPLAYERINTERNALEXTENT_TYPE_INFO
    }
}


pub const CLIENTGAMEPLAYERINTERNALEXTENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGamePlayerInternalExtent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientGamePlayerInternalExtent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientExplosionEntity {
}

pub const CLIENTEXPLOSIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientExplosionEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EXPLOSIONENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTEXPLOSIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientExplosionEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTEXPLOSIONENTITY_TYPE_INFO
    }
}


pub const CLIENTEXPLOSIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientExplosionEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientExplosionEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientDynamicAvoidanceEntity {
}

pub const CLIENTDYNAMICAVOIDANCEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDynamicAvoidanceEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTDYNAMICAVOIDANCEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientDynamicAvoidanceEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTDYNAMICAVOIDANCEENTITY_TYPE_INFO
    }
}


pub const CLIENTDYNAMICAVOIDANCEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDynamicAvoidanceEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientDynamicAvoidanceEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientVehicleEntity {
}

pub const CLIENTVEHICLEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVehicleEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTCONTROLLABLEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVEHICLEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientVehicleEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTVEHICLEENTITY_TYPE_INFO
    }
}


pub const CLIENTVEHICLEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVehicleEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientVehicleEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientAnimatedDriverEntity {
}

pub const CLIENTANIMATEDDRIVERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAnimatedDriverEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTANIMATEDDRIVERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAnimatedDriverEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTANIMATEDDRIVERENTITY_TYPE_INFO
    }
}


pub const CLIENTANIMATEDDRIVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAnimatedDriverEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientAnimatedDriverEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientWheelComponent {
}

pub const CLIENTWHEELCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWheelComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWHEELCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWheelComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTWHEELCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTWHEELCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWheelComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientWheelComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientWaterDepthComponent {
}

pub const CLIENTWATERDEPTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWaterDepthComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWATERDEPTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWaterDepthComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTWATERDEPTHCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTWATERDEPTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWaterDepthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientWaterDepthComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientVehiclePhysicsComponent {
}

pub const CLIENTVEHICLEPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVehiclePhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PARTPHYSICSCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVEHICLEPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientVehiclePhysicsComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTVEHICLEPHYSICSCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTVEHICLEPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVehiclePhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientVehiclePhysicsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientVehicleMeshComponent {
}

pub const CLIENTVEHICLEMESHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVehicleMeshComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVEHICLEMESHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientVehicleMeshComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTVEHICLEMESHCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTVEHICLEMESHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVehicleMeshComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientVehicleMeshComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientVehicleHealthComponent {
}

pub const CLIENTVEHICLEHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVehicleHealthComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTCONTROLLABLEHEALTHCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVEHICLEHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientVehicleHealthComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTVEHICLEHEALTHCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTVEHICLEHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVehicleHealthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientVehicleHealthComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientVehicleEntryComponent {
}

pub const CLIENTVEHICLEENTRYCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVehicleEntryComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTPLAYERENTRYCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVEHICLEENTRYCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientVehicleEntryComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTVEHICLEENTRYCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTVEHICLEENTRYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVehicleEntryComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientVehicleEntryComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientVehicleCustomizationComponent {
}

pub const CLIENTVEHICLECUSTOMIZATIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVehicleCustomizationComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVEHICLECUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientVehicleCustomizationComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTVEHICLECUSTOMIZATIONCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTVEHICLECUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVehicleCustomizationComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientVehicleCustomizationComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientTrackWheelComponent {
}

pub const CLIENTTRACKWHEELCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTrackWheelComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTWHEELCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTTRACKWHEELCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientTrackWheelComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTTRACKWHEELCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTTRACKWHEELCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTrackWheelComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientTrackWheelComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientTrackComponent {
}

pub const CLIENTTRACKCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTrackComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTMESHCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTTRACKCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientTrackComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTTRACKCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTTRACKCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTrackComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientTrackComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientStanceFilterComponent {
}

pub const CLIENTSTANCEFILTERCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStanceFilterComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSTANCEFILTERCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientStanceFilterComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTSTANCEFILTERCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTSTANCEFILTERCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStanceFilterComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientStanceFilterComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientEdgeModelComponent {
}

pub const CLIENTEDGEMODELCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEdgeModelComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTEDGEMODELCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientEdgeModelComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTEDGEMODELCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTEDGEMODELCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEdgeModelComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientEdgeModelComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DestructionVolumeComponent {
}

pub const DESTRUCTIONVOLUMECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionVolumeComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DESTRUCTIONVOLUMECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DestructionVolumeComponent {
    fn type_info() -> &'static TypeInfo {
        DESTRUCTIONVOLUMECOMPONENT_TYPE_INFO
    }
}


pub const DESTRUCTIONVOLUMECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionVolumeComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("DestructionVolumeComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientStaticModelNetworkDestructionComponent {
}

pub const CLIENTSTATICMODELNETWORKDESTRUCTIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticModelNetworkDestructionComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSTATICMODELNETWORKDESTRUCTIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientStaticModelNetworkDestructionComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTSTATICMODELNETWORKDESTRUCTIONCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTSTATICMODELNETWORKDESTRUCTIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticModelNetworkDestructionComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientStaticModelNetworkDestructionComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientGameSubView {
}

pub const CLIENTGAMESUBVIEW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameSubView",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTSUBVIEW_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTGAMESUBVIEW_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientGameSubView {
    fn type_info() -> &'static TypeInfo {
        CLIENTGAMESUBVIEW_TYPE_INFO
    }
}


pub const CLIENTGAMESUBVIEW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameSubView-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientGameSubView-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientLocalPlayerProxyEntityTrack {
}

pub const CLIENTLOCALPLAYERPROXYENTITYTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLocalPlayerProxyEntityTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TEMPLATEDPROXYENTITYTRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTLOCALPLAYERPROXYENTITYTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientLocalPlayerProxyEntityTrack {
    fn type_info() -> &'static TypeInfo {
        CLIENTLOCALPLAYERPROXYENTITYTRACK_TYPE_INFO
    }
}


pub const CLIENTLOCALPLAYERPROXYENTITYTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLocalPlayerProxyEntityTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientLocalPlayerProxyEntityTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PA2LookAtTrack {
}

pub const PA2LOOKATTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2LookAtTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LINKTRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PA2LOOKATTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PA2LookAtTrack {
    fn type_info() -> &'static TypeInfo {
        PA2LOOKATTRACK_TYPE_INFO
    }
}


pub const PA2LOOKATTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2LookAtTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("PA2LookAtTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LightTrack {
}

pub const LIGHTTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYTRACKBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LIGHTTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LightTrack {
    fn type_info() -> &'static TypeInfo {
        LIGHTTRACK_TYPE_INFO
    }
}


pub const LIGHTTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("LightTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LightingTrack {
}

pub const LIGHTINGTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightingTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LIGHTINGTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LightingTrack {
    fn type_info() -> &'static TypeInfo {
        LIGHTINGTRACK_TYPE_INFO
    }
}


pub const LIGHTINGTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightingTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("LightingTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SubLevelVisibleEntity {
}

pub const SUBLEVELVISIBLEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelVisibleEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SUBLEVELVISIBLEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SubLevelVisibleEntity {
    fn type_info() -> &'static TypeInfo {
        SUBLEVELVISIBLEENTITY_TYPE_INFO
    }
}


pub const SUBLEVELVISIBLEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelVisibleEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("SubLevelVisibleEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientSubLevelEntity {
}

pub const CLIENTSUBLEVELENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSubLevelEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SUBLEVELENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSUBLEVELENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSubLevelEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTSUBLEVELENTITY_TYPE_INFO
    }
}


pub const CLIENTSUBLEVELENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSubLevelEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientSubLevelEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientSubLevelCollectionEntity {
}

pub const CLIENTSUBLEVELCOLLECTIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSubLevelCollectionEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SUBLEVELCOLLECTIONENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSUBLEVELCOLLECTIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSubLevelCollectionEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTSUBLEVELCOLLECTIONENTITY_TYPE_INFO
    }
}


pub const CLIENTSUBLEVELCOLLECTIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSubLevelCollectionEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientSubLevelCollectionEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientLocalPlayerGateEntity {
}

pub const CLIENTLOCALPLAYERGATEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLocalPlayerGateEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTLOCALPLAYERGATEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientLocalPlayerGateEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTLOCALPLAYERGATEENTITY_TYPE_INFO
    }
}


pub const CLIENTLOCALPLAYERGATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLocalPlayerGateEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientLocalPlayerGateEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientAntDrivenComponent {
}

pub const CLIENTANTDRIVENCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAntDrivenComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTANTDRIVENCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAntDrivenComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTANTDRIVENCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTANTDRIVENCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAntDrivenComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientAntDrivenComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientWarpAnimationEntity {
}

pub const CLIENTWARPANIMATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWarpAnimationEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWARPANIMATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWarpAnimationEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTWARPANIMATIONENTITY_TYPE_INFO
    }
}


pub const CLIENTWARPANIMATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWarpAnimationEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientWarpAnimationEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPhysicsDrivenAnimationEntity {
}

pub const CLIENTPHYSICSDRIVENANIMATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPhysicsDrivenAnimationEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSDRIVENANIMATIONENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPHYSICSDRIVENANIMATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPhysicsDrivenAnimationEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTPHYSICSDRIVENANIMATIONENTITY_TYPE_INFO
    }
}


pub const CLIENTPHYSICSDRIVENANIMATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPhysicsDrivenAnimationEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientPhysicsDrivenAnimationEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCannedScenarioEntity {
}

pub const CLIENTCANNEDSCENARIOENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCannedScenarioEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCANNEDSCENARIOENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCannedScenarioEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTCANNEDSCENARIOENTITY_TYPE_INFO
    }
}


pub const CLIENTCANNEDSCENARIOENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCannedScenarioEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientCannedScenarioEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VehicleCamera {
}

pub const VEHICLECAMERA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleCamera",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TARGETCAMERA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VEHICLECAMERA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for VehicleCamera {
    fn type_info() -> &'static TypeInfo {
        VEHICLECAMERA_TYPE_INFO
    }
}


pub const VEHICLECAMERA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleCamera-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("VehicleCamera-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StaticCamera {
}

pub const STATICCAMERA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticCamera",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TARGETCAMERA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(STATICCAMERA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for StaticCamera {
    fn type_info() -> &'static TypeInfo {
        STATICCAMERA_TYPE_INFO
    }
}


pub const STATICCAMERA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticCamera-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("StaticCamera-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PhysicalCameraTrack {
}

pub const PHYSICALCAMERATRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicalCameraTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CAMERATRACKWITHSCENECAMERA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PHYSICALCAMERATRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PhysicalCameraTrack {
    fn type_info() -> &'static TypeInfo {
        PHYSICALCAMERATRACK_TYPE_INFO
    }
}


pub const PHYSICALCAMERATRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicalCameraTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("PhysicalCameraTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FPSCamera {
}

pub const FPSCAMERA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FPSCamera",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TARGETCAMERA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FPSCAMERA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FPSCamera {
    fn type_info() -> &'static TypeInfo {
        FPSCAMERA_TYPE_INFO
    }
}


pub const FPSCAMERA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FPSCamera-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("FPSCamera-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPlayerCameraEntity {
}

pub const CLIENTPLAYERCAMERAENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerCameraEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTCAMERAENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPLAYERCAMERAENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPlayerCameraEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTPLAYERCAMERAENTITY_TYPE_INFO
    }
}


pub const CLIENTPLAYERCAMERAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerCameraEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientPlayerCameraEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientGameplayCameraTrack {
}

pub const CLIENTGAMEPLAYCAMERATRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameplayCameraTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CAMERATRACKBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTGAMEPLAYCAMERATRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientGameplayCameraTrack {
    fn type_info() -> &'static TypeInfo {
        CLIENTGAMEPLAYCAMERATRACK_TYPE_INFO
    }
}


pub const CLIENTGAMEPLAYCAMERATRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameplayCameraTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientGameplayCameraTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCameraTrack {
}

pub const CLIENTCAMERATRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCameraTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICALCAMERATRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCAMERATRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCameraTrack {
    fn type_info() -> &'static TypeInfo {
        CLIENTCAMERATRACK_TYPE_INFO
    }
}


pub const CLIENTCAMERATRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCameraTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientCameraTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientTargetCameraEntity {
}

pub const CLIENTTARGETCAMERAENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTargetCameraEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTCAMERAENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTTARGETCAMERAENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientTargetCameraEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTTARGETCAMERAENTITY_TYPE_INFO
    }
}


pub const CLIENTTARGETCAMERAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTargetCameraEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientTargetCameraEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientLookAtCameraEntity {
}

pub const CLIENTLOOKATCAMERAENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLookAtCameraEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTCAMERAENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTLOOKATCAMERAENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientLookAtCameraEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTLOOKATCAMERAENTITY_TYPE_INFO
    }
}


pub const CLIENTLOOKATCAMERAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLookAtCameraEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientLookAtCameraEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCameraEntity {
}

pub const CLIENTCAMERAENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCameraEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTCAMERAENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCAMERAENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCameraEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTCAMERAENTITY_TYPE_INFO
    }
}


pub const CLIENTCAMERAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCameraEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientCameraEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCameraEntityBase {
}

pub const CLIENTCAMERAENTITYBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCameraEntityBase",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCAMERAENTITYBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCameraEntityBase {
    fn type_info() -> &'static TypeInfo {
        CLIENTCAMERAENTITYBASE_TYPE_INFO
    }
}


pub const CLIENTCAMERAENTITYBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCameraEntityBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientCameraEntityBase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCameraDirectorTrack {
}

pub const CLIENTCAMERADIRECTORTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCameraDirectorTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CAMERADIRECTORTRACKBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCAMERADIRECTORTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCameraDirectorTrack {
    fn type_info() -> &'static TypeInfo {
        CLIENTCAMERADIRECTORTRACK_TYPE_INFO
    }
}


pub const CLIENTCAMERADIRECTORTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCameraDirectorTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientCameraDirectorTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientANTAnimatableCameraTrack {
}

pub const CLIENTANTANIMATABLECAMERATRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientANTAnimatableCameraTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICALCAMERATRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTANTANIMATABLECAMERATRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientANTAnimatableCameraTrack {
    fn type_info() -> &'static TypeInfo {
        CLIENTANTANIMATABLECAMERATRACK_TYPE_INFO
    }
}


pub const CLIENTANTANIMATABLECAMERATRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientANTAnimatableCameraTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientANTAnimatableCameraTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IUISystem {
}

pub const IUISYSTEM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IUISystem",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(IUISYSTEM_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IUISystem {
    fn type_info() -> &'static TypeInfo {
        IUISYSTEM_TYPE_INFO
    }
}


pub const IUISYSTEM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IUISystem-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("IUISystem-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientWaterEntity {
}

pub const CLIENTWATERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWaterEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTPHYSICSENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWATERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWaterEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTWATERENTITY_TYPE_INFO
    }
}


pub const CLIENTWATERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWaterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientWaterEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientTerrainEntity {
}

pub const CLIENTTERRAINENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTerrainEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTPHYSICSENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTTERRAINENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientTerrainEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTTERRAINENTITY_TYPE_INFO
    }
}


pub const CLIENTTERRAINENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTerrainEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientTerrainEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientStaticModelGroupEntity {
}

pub const CLIENTSTATICMODELGROUPENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticModelGroupEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTPHYSICSENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSTATICMODELGROUPENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientStaticModelGroupEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTSTATICMODELGROUPENTITY_TYPE_INFO
    }
}


pub const CLIENTSTATICMODELGROUPENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticModelGroupEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientStaticModelGroupEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientStaticModelEntity {
}

pub const CLIENTSTATICMODELENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticModelEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTPHYSICSENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSTATICMODELENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientStaticModelEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTSTATICMODELENTITY_TYPE_INFO
    }
}


pub const CLIENTSTATICMODELENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticModelEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientStaticModelEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientLightDimmerEntity {
}

pub const CLIENTLIGHTDIMMERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLightDimmerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTLIGHTDIMMERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientLightDimmerEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTLIGHTDIMMERENTITY_TYPE_INFO
    }
}


pub const CLIENTLIGHTDIMMERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLightDimmerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientLightDimmerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientLadderEntity {
}

pub const CLIENTLADDERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLadderEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTSTATICMODELENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTLADDERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientLadderEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTLADDERENTITY_TYPE_INFO
    }
}


pub const CLIENTLADDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLadderEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientLadderEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientDynamicModelEntity {
}

pub const CLIENTDYNAMICMODELENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDynamicModelEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTPHYSICSENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTDYNAMICMODELENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientDynamicModelEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTDYNAMICMODELENTITY_TYPE_INFO
    }
}


pub const CLIENTDYNAMICMODELENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDynamicModelEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientDynamicModelEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientBangerEntity {
}

pub const CLIENTBANGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBangerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTPHYSICSENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTBANGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientBangerEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTBANGERENTITY_TYPE_INFO
    }
}


pub const CLIENTBANGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBangerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientBangerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LogicVisualEnvironmentEntity {
}

pub const LOGICVISUALENVIRONMENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogicVisualEnvironmentEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LOGICVISUALENVIRONMENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LogicVisualEnvironmentEntity {
    fn type_info() -> &'static TypeInfo {
        LOGICVISUALENVIRONMENTENTITY_TYPE_INFO
    }
}


pub const LOGICVISUALENVIRONMENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogicVisualEnvironmentEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("LogicVisualEnvironmentEntity-Array"),
    array_type: None,
    alignment: 8,
};


