use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct ChaseCamera {
    pub _glacier_base: super::gameplay_sim::TargetCamera,
}

pub trait ChaseCameraTrait: super::gameplay_sim::TargetCameraTrait {
}

impl ChaseCameraTrait for ChaseCamera {
}

impl super::gameplay_sim::TargetCameraTrait for ChaseCamera {
}

impl super::gameplay_sim::CameraTrait for ChaseCamera {
}

pub static CHASECAMERA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ChaseCamera",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_sim::TARGETCAMERA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ChaseCamera as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CHASECAMERA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ChaseCamera {
    fn type_info(&self) -> &'static TypeInfo {
        CHASECAMERA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CHASECAMERA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ChaseCamera-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ChaseCamera"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CameraTrackCamera {
    pub _glacier_base: super::gameplay_sim::Camera,
}

pub trait CameraTrackCameraTrait: super::gameplay_sim::CameraTrait {
}

impl CameraTrackCameraTrait for CameraTrackCamera {
}

impl super::gameplay_sim::CameraTrait for CameraTrackCamera {
}

pub static CAMERATRACKCAMERA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraTrackCamera",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_sim::CAMERA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CameraTrackCamera as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CAMERATRACKCAMERA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CameraTrackCamera {
    fn type_info(&self) -> &'static TypeInfo {
        CAMERATRACKCAMERA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CAMERATRACKCAMERA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraTrackCamera-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("CameraTrackCamera"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CameraTrackWithSceneCamera {
    pub _glacier_base: CameraTrackBase,
}

pub trait CameraTrackWithSceneCameraTrait: CameraTrackBaseTrait {
}

impl CameraTrackWithSceneCameraTrait for CameraTrackWithSceneCamera {
}

impl CameraTrackBaseTrait for CameraTrackWithSceneCamera {
}

impl super::timeline::TimelineTrackTrait for CameraTrackWithSceneCamera {
}

pub static CAMERATRACKWITHSCENECAMERA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraTrackWithSceneCamera",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CAMERATRACKBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CameraTrackWithSceneCamera as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CAMERATRACKWITHSCENECAMERA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CameraTrackWithSceneCamera {
    fn type_info(&self) -> &'static TypeInfo {
        CAMERATRACKWITHSCENECAMERA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CAMERATRACKWITHSCENECAMERA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraTrackWithSceneCamera-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("CameraTrackWithSceneCamera"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CameraTrackBase {
    pub _glacier_base: super::timeline::TimelineTrack,
}

pub trait CameraTrackBaseTrait: super::timeline::TimelineTrackTrait {
}

impl CameraTrackBaseTrait for CameraTrackBase {
}

impl super::timeline::TimelineTrackTrait for CameraTrackBase {
}

pub static CAMERATRACKBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraTrackBase",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::TIMELINETRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CameraTrackBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CAMERATRACKBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CameraTrackBase {
    fn type_info(&self) -> &'static TypeInfo {
        CAMERATRACKBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CAMERATRACKBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraTrackBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("CameraTrackBase"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CameraLookAtLayerTrack {
    pub _glacier_base: super::timeline::TransformLayer,
}

pub trait CameraLookAtLayerTrackTrait: super::timeline::TransformLayerTrait {
}

impl CameraLookAtLayerTrackTrait for CameraLookAtLayerTrack {
}

impl super::timeline::TransformLayerTrait for CameraLookAtLayerTrack {
}

impl super::timeline::TimelineTrackTrait for CameraLookAtLayerTrack {
}

pub static CAMERALOOKATLAYERTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraLookAtLayerTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::TRANSFORMLAYER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CameraLookAtLayerTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CAMERALOOKATLAYERTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CameraLookAtLayerTrack {
    fn type_info(&self) -> &'static TypeInfo {
        CAMERALOOKATLAYERTRACK_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CAMERALOOKATLAYERTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraLookAtLayerTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("CameraLookAtLayerTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CameraFocusTrack {
    pub _glacier_base: super::timeline::TimelineTrack,
}

pub trait CameraFocusTrackTrait: super::timeline::TimelineTrackTrait {
}

impl CameraFocusTrackTrait for CameraFocusTrack {
}

impl super::timeline::TimelineTrackTrait for CameraFocusTrack {
}

pub static CAMERAFOCUSTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraFocusTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::TIMELINETRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CameraFocusTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CAMERAFOCUSTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CameraFocusTrack {
    fn type_info(&self) -> &'static TypeInfo {
        CAMERAFOCUSTRACK_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CAMERAFOCUSTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraFocusTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("CameraFocusTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CameraDofTrack {
    pub _glacier_base: super::timeline::TimelineTrack,
}

pub trait CameraDofTrackTrait: super::timeline::TimelineTrackTrait {
}

impl CameraDofTrackTrait for CameraDofTrack {
}

impl super::timeline::TimelineTrackTrait for CameraDofTrack {
}

pub static CAMERADOFTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraDofTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::TIMELINETRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CameraDofTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CAMERADOFTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CameraDofTrack {
    fn type_info(&self) -> &'static TypeInfo {
        CAMERADOFTRACK_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CAMERADOFTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraDofTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("CameraDofTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CameraDirectorTrackBase {
    pub _glacier_base: super::timeline::TimelineTrack,
}

pub trait CameraDirectorTrackBaseTrait: super::timeline::TimelineTrackTrait {
}

impl CameraDirectorTrackBaseTrait for CameraDirectorTrackBase {
}

impl super::timeline::TimelineTrackTrait for CameraDirectorTrackBase {
}

pub static CAMERADIRECTORTRACKBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraDirectorTrackBase",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::TIMELINETRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CameraDirectorTrackBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CAMERADIRECTORTRACKBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CameraDirectorTrackBase {
    fn type_info(&self) -> &'static TypeInfo {
        CAMERADIRECTORTRACKBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CAMERADIRECTORTRACKBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraDirectorTrackBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("CameraDirectorTrackBase"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CameraDirectorProxyTrack {
    pub _glacier_base: super::timeline::TimelineTrack,
}

pub trait CameraDirectorProxyTrackTrait: super::timeline::TimelineTrackTrait {
}

impl CameraDirectorProxyTrackTrait for CameraDirectorProxyTrack {
}

impl super::timeline::TimelineTrackTrait for CameraDirectorProxyTrack {
}

pub static CAMERADIRECTORPROXYTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraDirectorProxyTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::TIMELINETRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CameraDirectorProxyTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CAMERADIRECTORPROXYTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CameraDirectorProxyTrack {
    fn type_info(&self) -> &'static TypeInfo {
        CAMERADIRECTORPROXYTRACK_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CAMERADIRECTORPROXYTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraDirectorProxyTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("CameraDirectorProxyTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CameraCutVisibilityTrack {
    pub _glacier_base: super::timeline::TimelineTrack,
}

pub trait CameraCutVisibilityTrackTrait: super::timeline::TimelineTrackTrait {
}

impl CameraCutVisibilityTrackTrait for CameraCutVisibilityTrack {
}

impl super::timeline::TimelineTrackTrait for CameraCutVisibilityTrack {
}

pub static CAMERACUTVISIBILITYTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraCutVisibilityTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::TIMELINETRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CameraCutVisibilityTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CAMERACUTVISIBILITYTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CameraCutVisibilityTrack {
    fn type_info(&self) -> &'static TypeInfo {
        CAMERACUTVISIBILITYTRACK_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CAMERACUTVISIBILITYTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraCutVisibilityTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("CameraCutVisibilityTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientProxyBankEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientProxyBankEntityTrait: super::entity::EntityTrait {
}

impl ClientProxyBankEntityTrait for ClientProxyBankEntity {
}

impl super::entity::EntityTrait for ClientProxyBankEntity {
}

impl super::entity::EntityBusPeerTrait for ClientProxyBankEntity {
}

pub static CLIENTPROXYBANKENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientProxyBankEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientProxyBankEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPROXYBANKENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientProxyBankEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPROXYBANKENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTPROXYBANKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientProxyBankEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientProxyBankEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientPlayAnimationEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientPlayAnimationEntityTrait: super::entity::EntityTrait {
}

impl ClientPlayAnimationEntityTrait for ClientPlayAnimationEntity {
}

impl super::entity::EntityTrait for ClientPlayAnimationEntity {
}

impl super::entity::EntityBusPeerTrait for ClientPlayAnimationEntity {
}

pub static CLIENTPLAYANIMATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayAnimationEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPlayAnimationEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPLAYANIMATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPlayAnimationEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPLAYANIMATIONENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTPLAYANIMATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayAnimationEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientPlayAnimationEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientPA2TargetComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientPA2TargetComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientPA2TargetComponentTrait for ClientPA2TargetComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientPA2TargetComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientPA2TargetComponent {
}

impl super::entity::ComponentTrait for ClientPA2TargetComponent {
}

impl super::entity::EntityBusPeerTrait for ClientPA2TargetComponent {
}

pub static CLIENTPA2TARGETCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPA2TargetComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPA2TargetComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPA2TARGETCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPA2TargetComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPA2TARGETCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTPA2TARGETCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPA2TargetComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientPA2TargetComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ObjectHighlightEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ObjectHighlightEntityTrait: super::entity::EntityTrait {
}

impl ObjectHighlightEntityTrait for ObjectHighlightEntity {
}

impl super::entity::EntityTrait for ObjectHighlightEntity {
}

impl super::entity::EntityBusPeerTrait for ObjectHighlightEntity {
}

pub static OBJECTHIGHLIGHTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectHighlightEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ObjectHighlightEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(OBJECTHIGHLIGHTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ObjectHighlightEntity {
    fn type_info(&self) -> &'static TypeInfo {
        OBJECTHIGHLIGHTENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static OBJECTHIGHLIGHTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectHighlightEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ObjectHighlightEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MemoryPoolControlEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait MemoryPoolControlEntityTrait: super::entity::EntityTrait {
}

impl MemoryPoolControlEntityTrait for MemoryPoolControlEntity {
}

impl super::entity::EntityTrait for MemoryPoolControlEntity {
}

impl super::entity::EntityBusPeerTrait for MemoryPoolControlEntity {
}

pub static MEMORYPOOLCONTROLENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryPoolControlEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MemoryPoolControlEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(MEMORYPOOLCONTROLENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MemoryPoolControlEntity {
    fn type_info(&self) -> &'static TypeInfo {
        MEMORYPOOLCONTROLENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MEMORYPOOLCONTROLENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryPoolControlEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("MemoryPoolControlEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientWorldRendererSwitchEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientWorldRendererSwitchEntityTrait: super::entity::EntityTrait {
}

impl ClientWorldRendererSwitchEntityTrait for ClientWorldRendererSwitchEntity {
}

impl super::entity::EntityTrait for ClientWorldRendererSwitchEntity {
}

impl super::entity::EntityBusPeerTrait for ClientWorldRendererSwitchEntity {
}

pub static CLIENTWORLDRENDERERSWITCHENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWorldRendererSwitchEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientWorldRendererSwitchEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWORLDRENDERERSWITCHENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWorldRendererSwitchEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTWORLDRENDERERSWITCHENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTWORLDRENDERERSWITCHENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWorldRendererSwitchEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientWorldRendererSwitchEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientTextureAssetEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientTextureAssetEntityTrait: super::entity::EntityTrait {
}

impl ClientTextureAssetEntityTrait for ClientTextureAssetEntity {
}

impl super::entity::EntityTrait for ClientTextureAssetEntity {
}

impl super::entity::EntityBusPeerTrait for ClientTextureAssetEntity {
}

pub static CLIENTTEXTUREASSETENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTextureAssetEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientTextureAssetEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTTEXTUREASSETENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientTextureAssetEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTTEXTUREASSETENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTTEXTUREASSETENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTextureAssetEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientTextureAssetEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientTerrainShaderParameterEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientTerrainShaderParameterEntityTrait: super::entity::EntityTrait {
}

impl ClientTerrainShaderParameterEntityTrait for ClientTerrainShaderParameterEntity {
}

impl super::entity::EntityTrait for ClientTerrainShaderParameterEntity {
}

impl super::entity::EntityBusPeerTrait for ClientTerrainShaderParameterEntity {
}

pub static CLIENTTERRAINSHADERPARAMETERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTerrainShaderParameterEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientTerrainShaderParameterEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTTERRAINSHADERPARAMETERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientTerrainShaderParameterEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTTERRAINSHADERPARAMETERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTTERRAINSHADERPARAMETERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTerrainShaderParameterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientTerrainShaderParameterEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientPerfJournalBookmarkLabelEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientPerfJournalBookmarkLabelEntityTrait: super::entity::EntityTrait {
}

impl ClientPerfJournalBookmarkLabelEntityTrait for ClientPerfJournalBookmarkLabelEntity {
}

impl super::entity::EntityTrait for ClientPerfJournalBookmarkLabelEntity {
}

impl super::entity::EntityBusPeerTrait for ClientPerfJournalBookmarkLabelEntity {
}

pub static CLIENTPERFJOURNALBOOKMARKLABELENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPerfJournalBookmarkLabelEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPerfJournalBookmarkLabelEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPERFJOURNALBOOKMARKLABELENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPerfJournalBookmarkLabelEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPERFJOURNALBOOKMARKLABELENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTPERFJOURNALBOOKMARKLABELENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPerfJournalBookmarkLabelEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientPerfJournalBookmarkLabelEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientPerfJournalBookmarkEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientPerfJournalBookmarkEntityTrait: super::entity::EntityTrait {
}

impl ClientPerfJournalBookmarkEntityTrait for ClientPerfJournalBookmarkEntity {
}

impl super::entity::EntityTrait for ClientPerfJournalBookmarkEntity {
}

impl super::entity::EntityBusPeerTrait for ClientPerfJournalBookmarkEntity {
}

pub static CLIENTPERFJOURNALBOOKMARKENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPerfJournalBookmarkEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPerfJournalBookmarkEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPERFJOURNALBOOKMARKENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPerfJournalBookmarkEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPERFJOURNALBOOKMARKENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTPERFJOURNALBOOKMARKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPerfJournalBookmarkEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientPerfJournalBookmarkEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct GameClientConnection {
    pub _glacier_base: super::gameplay_client_server::ClientConnection,
}

pub trait GameClientConnectionTrait: super::gameplay_client_server::ClientConnectionTrait {
}

impl GameClientConnectionTrait for GameClientConnection {
}

impl super::gameplay_client_server::ClientConnectionTrait for GameClientConnection {
}

impl super::network::EngineConnectionPeerTrait for GameClientConnection {
}

impl super::network::EngineConnectionTrait for GameClientConnection {
}

pub static GAMECLIENTCONNECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameClientConnection",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTCONNECTION_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GameClientConnection as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(GAMECLIENTCONNECTION_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for GameClientConnection {
    fn type_info(&self) -> &'static TypeInfo {
        GAMECLIENTCONNECTION_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static GAMECLIENTCONNECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameClientConnection-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("GameClientConnection"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientPlayerInputTriggerEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientPlayerInputTriggerEntityTrait: super::entity::EntityTrait {
}

impl ClientPlayerInputTriggerEntityTrait for ClientPlayerInputTriggerEntity {
}

impl super::entity::EntityTrait for ClientPlayerInputTriggerEntity {
}

impl super::entity::EntityBusPeerTrait for ClientPlayerInputTriggerEntity {
}

pub static CLIENTPLAYERINPUTTRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerInputTriggerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPlayerInputTriggerEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPLAYERINPUTTRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPlayerInputTriggerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPLAYERINPUTTRIGGERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTPLAYERINPUTTRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerInputTriggerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientPlayerInputTriggerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientCombatAreaTriggerEntity {
    pub _glacier_base: super::game_common::CombatAreaEntity,
}

pub trait ClientCombatAreaTriggerEntityTrait: super::game_common::CombatAreaEntityTrait {
}

impl ClientCombatAreaTriggerEntityTrait for ClientCombatAreaTriggerEntity {
}

impl super::game_common::CombatAreaEntityTrait for ClientCombatAreaTriggerEntity {
}

impl super::entity::EntityTrait for ClientCombatAreaTriggerEntity {
}

impl super::entity::EntityBusPeerTrait for ClientCombatAreaTriggerEntity {
}

pub static CLIENTCOMBATAREATRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCombatAreaTriggerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_common::COMBATAREAENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCombatAreaTriggerEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCOMBATAREATRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCombatAreaTriggerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCOMBATAREATRIGGERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTCOMBATAREATRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCombatAreaTriggerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientCombatAreaTriggerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientClearAreaTriggerEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientClearAreaTriggerEntityTrait: super::entity::EntityTrait {
}

impl ClientClearAreaTriggerEntityTrait for ClientClearAreaTriggerEntity {
}

impl super::entity::EntityTrait for ClientClearAreaTriggerEntity {
}

impl super::entity::EntityBusPeerTrait for ClientClearAreaTriggerEntity {
}

pub static CLIENTCLEARAREATRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientClearAreaTriggerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientClearAreaTriggerEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCLEARAREATRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientClearAreaTriggerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCLEARAREATRIGGERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTCLEARAREATRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientClearAreaTriggerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientClearAreaTriggerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CameraEnterAreaTriggerEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait CameraEnterAreaTriggerEntityTrait: super::entity::EntityTrait {
}

impl CameraEnterAreaTriggerEntityTrait for CameraEnterAreaTriggerEntity {
}

impl super::entity::EntityTrait for CameraEnterAreaTriggerEntity {
}

impl super::entity::EntityBusPeerTrait for CameraEnterAreaTriggerEntity {
}

pub static CAMERAENTERAREATRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraEnterAreaTriggerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CameraEnterAreaTriggerEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CAMERAENTERAREATRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CameraEnterAreaTriggerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CAMERAENTERAREATRIGGERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CAMERAENTERAREATRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraEnterAreaTriggerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("CameraEnterAreaTriggerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientVehicleSpawnEntity {
    pub _glacier_base: super::gameplay_client_server::ClientSpawnEntity,
}

pub trait ClientVehicleSpawnEntityTrait: super::gameplay_client_server::ClientSpawnEntityTrait {
}

impl ClientVehicleSpawnEntityTrait for ClientVehicleSpawnEntity {
}

impl super::gameplay_client_server::ClientSpawnEntityTrait for ClientVehicleSpawnEntity {
}

impl super::entity::SpatialEntityTrait for ClientVehicleSpawnEntity {
}

impl super::entity::EntityTrait for ClientVehicleSpawnEntity {
}

impl super::entity::EntityBusPeerTrait for ClientVehicleSpawnEntity {
}

pub static CLIENTVEHICLESPAWNENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVehicleSpawnEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTSPAWNENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientVehicleSpawnEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVEHICLESPAWNENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientVehicleSpawnEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTVEHICLESPAWNENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTVEHICLESPAWNENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVehicleSpawnEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientVehicleSpawnEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientLocalPlayerProxyEntityTrackData {
    pub _glacier_base: super::timeline::TemplatedProxyEntityTrackData,
}

pub trait ClientLocalPlayerProxyEntityTrackDataTrait: super::timeline::TemplatedProxyEntityTrackDataTrait {
}

impl ClientLocalPlayerProxyEntityTrackDataTrait for ClientLocalPlayerProxyEntityTrackData {
}

impl super::timeline::TemplatedProxyEntityTrackDataTrait for ClientLocalPlayerProxyEntityTrackData {
}

impl super::timeline::ProxyEntityTrackDataTrait for ClientLocalPlayerProxyEntityTrackData {
}

impl super::timeline::EntityTrackBaseDataTrait for ClientLocalPlayerProxyEntityTrackData {
    fn children(&self) -> &Vec<Option<Arc<Mutex<dyn super::timeline::TimelineTrackDataTrait>>>> {
        self._glacier_base.children()
    }
    fn inherited_to_child_conversation_lines(&self) -> &bool {
        self._glacier_base.inherited_to_child_conversation_lines()
    }
    fn required(&self) -> &bool {
        self._glacier_base.required()
    }
    fn handle_deleted_entity(&self) -> &bool {
        self._glacier_base.handle_deleted_entity()
    }
    fn entity_sharing_policy(&self) -> &super::timeline::EntityTrackSharingPolicy {
        self._glacier_base.entity_sharing_policy()
    }
}

impl super::timeline::TimelineTrackDataTrait for ClientLocalPlayerProxyEntityTrackData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn super::timeline::TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
}

impl super::entity::GameObjectDataTrait for ClientLocalPlayerProxyEntityTrackData {
}

impl super::core::DataBusPeerTrait for ClientLocalPlayerProxyEntityTrackData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for ClientLocalPlayerProxyEntityTrackData {
}

impl super::core::DataContainerTrait for ClientLocalPlayerProxyEntityTrackData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static CLIENTLOCALPLAYERPROXYENTITYTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLocalPlayerProxyEntityTrackData",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::TEMPLATEDPROXYENTITYTRACKDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientLocalPlayerProxyEntityTrackData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTLOCALPLAYERPROXYENTITYTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ClientLocalPlayerProxyEntityTrackData {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTLOCALPLAYERPROXYENTITYTRACKDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTLOCALPLAYERPROXYENTITYTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLocalPlayerProxyEntityTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientLocalPlayerProxyEntityTrackData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PA2LookAtConfiguration {
    pub _glacier_base: super::core::SystemSettings,
    pub on_game_states: Vec<Option<Arc<Mutex<dyn PA2LookAtGameStateSettingTrait>>>>,
    pub off_game_states: Vec<Option<Arc<Mutex<dyn PA2LookAtGameStateSettingTrait>>>>,
    pub controller_game_state: super::ant::AntRef,
    pub default_on_controller: i32,
    pub off_controller: i32,
    pub target_game_state: super::ant::AntRef,
    pub snap_to_target_game_state: super::ant::AntRef,
}

pub trait PA2LookAtConfigurationTrait: super::core::SystemSettingsTrait {
    fn on_game_states(&self) -> &Vec<Option<Arc<Mutex<dyn PA2LookAtGameStateSettingTrait>>>>;
    fn off_game_states(&self) -> &Vec<Option<Arc<Mutex<dyn PA2LookAtGameStateSettingTrait>>>>;
    fn controller_game_state(&self) -> &super::ant::AntRef;
    fn default_on_controller(&self) -> &i32;
    fn off_controller(&self) -> &i32;
    fn target_game_state(&self) -> &super::ant::AntRef;
    fn snap_to_target_game_state(&self) -> &super::ant::AntRef;
}

impl PA2LookAtConfigurationTrait for PA2LookAtConfiguration {
    fn on_game_states(&self) -> &Vec<Option<Arc<Mutex<dyn PA2LookAtGameStateSettingTrait>>>> {
        &self.on_game_states
    }
    fn off_game_states(&self) -> &Vec<Option<Arc<Mutex<dyn PA2LookAtGameStateSettingTrait>>>> {
        &self.off_game_states
    }
    fn controller_game_state(&self) -> &super::ant::AntRef {
        &self.controller_game_state
    }
    fn default_on_controller(&self) -> &i32 {
        &self.default_on_controller
    }
    fn off_controller(&self) -> &i32 {
        &self.off_controller
    }
    fn target_game_state(&self) -> &super::ant::AntRef {
        &self.target_game_state
    }
    fn snap_to_target_game_state(&self) -> &super::ant::AntRef {
        &self.snap_to_target_game_state
    }
}

impl super::core::SystemSettingsTrait for PA2LookAtConfiguration {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
}

impl super::core::DataContainerTrait for PA2LookAtConfiguration {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PA2LOOKATCONFIGURATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2LookAtConfiguration",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PA2LookAtConfiguration as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "OnGameStates",
                flags: MemberInfoFlags::new(144),
                field_type: "PA2LookAtGameStateSetting-Array",
                rust_offset: offset_of!(PA2LookAtConfiguration, on_game_states),
            },
            FieldInfoData {
                name: "OffGameStates",
                flags: MemberInfoFlags::new(144),
                field_type: "PA2LookAtGameStateSetting-Array",
                rust_offset: offset_of!(PA2LookAtConfiguration, off_game_states),
            },
            FieldInfoData {
                name: "ControllerGameState",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(PA2LookAtConfiguration, controller_game_state),
            },
            FieldInfoData {
                name: "DefaultOnController",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PA2LookAtConfiguration, default_on_controller),
            },
            FieldInfoData {
                name: "OffController",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PA2LookAtConfiguration, off_controller),
            },
            FieldInfoData {
                name: "TargetGameState",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(PA2LookAtConfiguration, target_game_state),
            },
            FieldInfoData {
                name: "SnapToTargetGameState",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(PA2LookAtConfiguration, snap_to_target_game_state),
            },
        ],
    }),
    array_type: Some(PA2LOOKATCONFIGURATION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PA2LookAtConfiguration {
    fn type_info(&self) -> &'static TypeInfo {
        PA2LOOKATCONFIGURATION_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PA2LOOKATCONFIGURATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2LookAtConfiguration-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("PA2LookAtConfiguration"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PA2FloatLookAtGameStateSetting {
    pub _glacier_base: PA2LookAtGameStateSetting,
    pub value: f32,
}

pub trait PA2FloatLookAtGameStateSettingTrait: PA2LookAtGameStateSettingTrait {
    fn value(&self) -> &f32;
}

impl PA2FloatLookAtGameStateSettingTrait for PA2FloatLookAtGameStateSetting {
    fn value(&self) -> &f32 {
        &self.value
    }
}

impl PA2LookAtGameStateSettingTrait for PA2FloatLookAtGameStateSetting {
    fn game_state(&self) -> &super::ant::AntRef {
        self._glacier_base.game_state()
    }
}

impl super::core::DataContainerTrait for PA2FloatLookAtGameStateSetting {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PA2FLOATLOOKATGAMESTATESETTING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2FloatLookAtGameStateSetting",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PA2LOOKATGAMESTATESETTING_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PA2FloatLookAtGameStateSetting as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PA2FloatLookAtGameStateSetting, value),
            },
        ],
    }),
    array_type: Some(PA2FLOATLOOKATGAMESTATESETTING_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PA2FloatLookAtGameStateSetting {
    fn type_info(&self) -> &'static TypeInfo {
        PA2FLOATLOOKATGAMESTATESETTING_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PA2FLOATLOOKATGAMESTATESETTING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2FloatLookAtGameStateSetting-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("PA2FloatLookAtGameStateSetting"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PA2BoolLookAtGameStateSetting {
    pub _glacier_base: PA2LookAtGameStateSetting,
    pub value: bool,
}

pub trait PA2BoolLookAtGameStateSettingTrait: PA2LookAtGameStateSettingTrait {
    fn value(&self) -> &bool;
}

impl PA2BoolLookAtGameStateSettingTrait for PA2BoolLookAtGameStateSetting {
    fn value(&self) -> &bool {
        &self.value
    }
}

impl PA2LookAtGameStateSettingTrait for PA2BoolLookAtGameStateSetting {
    fn game_state(&self) -> &super::ant::AntRef {
        self._glacier_base.game_state()
    }
}

impl super::core::DataContainerTrait for PA2BoolLookAtGameStateSetting {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PA2BOOLLOOKATGAMESTATESETTING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2BoolLookAtGameStateSetting",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PA2LOOKATGAMESTATESETTING_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PA2BoolLookAtGameStateSetting as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PA2BoolLookAtGameStateSetting, value),
            },
        ],
    }),
    array_type: Some(PA2BOOLLOOKATGAMESTATESETTING_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PA2BoolLookAtGameStateSetting {
    fn type_info(&self) -> &'static TypeInfo {
        PA2BOOLLOOKATGAMESTATESETTING_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PA2BOOLLOOKATGAMESTATESETTING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2BoolLookAtGameStateSetting-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("PA2BoolLookAtGameStateSetting"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PA2LookAtGameStateSetting {
    pub _glacier_base: super::core::DataContainer,
    pub game_state: super::ant::AntRef,
}

pub trait PA2LookAtGameStateSettingTrait: super::core::DataContainerTrait {
    fn game_state(&self) -> &super::ant::AntRef;
}

impl PA2LookAtGameStateSettingTrait for PA2LookAtGameStateSetting {
    fn game_state(&self) -> &super::ant::AntRef {
        &self.game_state
    }
}

impl super::core::DataContainerTrait for PA2LookAtGameStateSetting {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PA2LOOKATGAMESTATESETTING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2LookAtGameStateSetting",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PA2LookAtGameStateSetting as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "GameState",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(PA2LookAtGameStateSetting, game_state),
            },
        ],
    }),
    array_type: Some(PA2LOOKATGAMESTATESETTING_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PA2LookAtGameStateSetting {
    fn type_info(&self) -> &'static TypeInfo {
        PA2LOOKATGAMESTATESETTING_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PA2LOOKATGAMESTATESETTING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2LookAtGameStateSetting-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("PA2LookAtGameStateSetting"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PA2LookAtTrackData {
    pub _glacier_base: super::timeline::TimelineTrackData,
    pub keyframes: Vec<Option<Arc<Mutex<dyn PA2LookAtKeyframeTrait>>>>,
}

pub trait PA2LookAtTrackDataTrait: super::timeline::TimelineTrackDataTrait {
    fn keyframes(&self) -> &Vec<Option<Arc<Mutex<dyn PA2LookAtKeyframeTrait>>>>;
}

impl PA2LookAtTrackDataTrait for PA2LookAtTrackData {
    fn keyframes(&self) -> &Vec<Option<Arc<Mutex<dyn PA2LookAtKeyframeTrait>>>> {
        &self.keyframes
    }
}

impl super::timeline::TimelineTrackDataTrait for PA2LookAtTrackData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn super::timeline::TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
}

impl super::entity::GameObjectDataTrait for PA2LookAtTrackData {
}

impl super::core::DataBusPeerTrait for PA2LookAtTrackData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for PA2LookAtTrackData {
}

impl super::core::DataContainerTrait for PA2LookAtTrackData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PA2LOOKATTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2LookAtTrackData",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::TIMELINETRACKDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PA2LookAtTrackData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Keyframes",
                flags: MemberInfoFlags::new(144),
                field_type: "PA2LookAtKeyframe-Array",
                rust_offset: offset_of!(PA2LookAtTrackData, keyframes),
            },
        ],
    }),
    array_type: Some(PA2LOOKATTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PA2LookAtTrackData {
    fn type_info(&self) -> &'static TypeInfo {
        PA2LOOKATTRACKDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PA2LOOKATTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2LookAtTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("PA2LookAtTrackData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PA2LookAtKeyframe {
    pub _glacier_base: super::timeline::TimelineKeyframeData,
    pub time: f32,
    pub target: Option<Arc<Mutex<dyn PA2LookAtTargetTrait>>>,
    pub controller: i32,
    pub snap_to_target: bool,
}

pub trait PA2LookAtKeyframeTrait: super::timeline::TimelineKeyframeDataTrait {
    fn time(&self) -> &f32;
    fn target(&self) -> &Option<Arc<Mutex<dyn PA2LookAtTargetTrait>>>;
    fn controller(&self) -> &i32;
    fn snap_to_target(&self) -> &bool;
}

impl PA2LookAtKeyframeTrait for PA2LookAtKeyframe {
    fn time(&self) -> &f32 {
        &self.time
    }
    fn target(&self) -> &Option<Arc<Mutex<dyn PA2LookAtTargetTrait>>> {
        &self.target
    }
    fn controller(&self) -> &i32 {
        &self.controller
    }
    fn snap_to_target(&self) -> &bool {
        &self.snap_to_target
    }
}

impl super::timeline::TimelineKeyframeDataTrait for PA2LookAtKeyframe {
}

impl super::core::DataContainerTrait for PA2LookAtKeyframe {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PA2LOOKATKEYFRAME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2LookAtKeyframe",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::TIMELINEKEYFRAMEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PA2LookAtKeyframe as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Time",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PA2LookAtKeyframe, time),
            },
            FieldInfoData {
                name: "Target",
                flags: MemberInfoFlags::new(0),
                field_type: "PA2LookAtTarget",
                rust_offset: offset_of!(PA2LookAtKeyframe, target),
            },
            FieldInfoData {
                name: "Controller",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PA2LookAtKeyframe, controller),
            },
            FieldInfoData {
                name: "SnapToTarget",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PA2LookAtKeyframe, snap_to_target),
            },
        ],
    }),
    array_type: Some(PA2LOOKATKEYFRAME_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PA2LookAtKeyframe {
    fn type_info(&self) -> &'static TypeInfo {
        PA2LOOKATKEYFRAME_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PA2LOOKATKEYFRAME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2LookAtKeyframe-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("PA2LookAtKeyframe"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PA2LookAtTarget {
    pub _glacier_base: super::core::DataContainer,
    pub entity_track: Option<Arc<Mutex<dyn super::timeline::EntityTrackBaseDataTrait>>>,
}

pub trait PA2LookAtTargetTrait: super::core::DataContainerTrait {
    fn entity_track(&self) -> &Option<Arc<Mutex<dyn super::timeline::EntityTrackBaseDataTrait>>>;
}

impl PA2LookAtTargetTrait for PA2LookAtTarget {
    fn entity_track(&self) -> &Option<Arc<Mutex<dyn super::timeline::EntityTrackBaseDataTrait>>> {
        &self.entity_track
    }
}

impl super::core::DataContainerTrait for PA2LookAtTarget {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PA2LOOKATTARGET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2LookAtTarget",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PA2LookAtTarget as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "EntityTrack",
                flags: MemberInfoFlags::new(0),
                field_type: "EntityTrackBaseData",
                rust_offset: offset_of!(PA2LookAtTarget, entity_track),
            },
        ],
    }),
    array_type: Some(PA2LOOKATTARGET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PA2LookAtTarget {
    fn type_info(&self) -> &'static TypeInfo {
        PA2LOOKATTARGET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PA2LOOKATTARGET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2LookAtTarget-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("PA2LookAtTarget"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LightingTrackData {
    pub _glacier_base: super::timeline::TimelineTrackData,
    pub children: Vec<Option<Arc<Mutex<dyn LightTrackDataTrait>>>>,
}

pub trait LightingTrackDataTrait: super::timeline::TimelineTrackDataTrait {
    fn children(&self) -> &Vec<Option<Arc<Mutex<dyn LightTrackDataTrait>>>>;
}

impl LightingTrackDataTrait for LightingTrackData {
    fn children(&self) -> &Vec<Option<Arc<Mutex<dyn LightTrackDataTrait>>>> {
        &self.children
    }
}

impl super::timeline::TimelineTrackDataTrait for LightingTrackData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn super::timeline::TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
}

impl super::entity::GameObjectDataTrait for LightingTrackData {
}

impl super::core::DataBusPeerTrait for LightingTrackData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for LightingTrackData {
}

impl super::core::DataContainerTrait for LightingTrackData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static LIGHTINGTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightingTrackData",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::TIMELINETRACKDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LightingTrackData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Children",
                flags: MemberInfoFlags::new(144),
                field_type: "LightTrackData-Array",
                rust_offset: offset_of!(LightingTrackData, children),
            },
        ],
    }),
    array_type: Some(LIGHTINGTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LightingTrackData {
    fn type_info(&self) -> &'static TypeInfo {
        LIGHTINGTRACKDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LIGHTINGTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightingTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("LightingTrackData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LightTrackData {
    pub _glacier_base: super::timeline::EntityTrackBaseData,
}

pub trait LightTrackDataTrait: super::timeline::EntityTrackBaseDataTrait {
}

impl LightTrackDataTrait for LightTrackData {
}

impl super::timeline::EntityTrackBaseDataTrait for LightTrackData {
    fn children(&self) -> &Vec<Option<Arc<Mutex<dyn super::timeline::TimelineTrackDataTrait>>>> {
        self._glacier_base.children()
    }
    fn inherited_to_child_conversation_lines(&self) -> &bool {
        self._glacier_base.inherited_to_child_conversation_lines()
    }
    fn required(&self) -> &bool {
        self._glacier_base.required()
    }
    fn handle_deleted_entity(&self) -> &bool {
        self._glacier_base.handle_deleted_entity()
    }
    fn entity_sharing_policy(&self) -> &super::timeline::EntityTrackSharingPolicy {
        self._glacier_base.entity_sharing_policy()
    }
}

impl super::timeline::TimelineTrackDataTrait for LightTrackData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn super::timeline::TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
}

impl super::entity::GameObjectDataTrait for LightTrackData {
}

impl super::core::DataBusPeerTrait for LightTrackData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for LightTrackData {
}

impl super::core::DataContainerTrait for LightTrackData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static LIGHTTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightTrackData",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::ENTITYTRACKBASEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LightTrackData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LIGHTTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LightTrackData {
    fn type_info(&self) -> &'static TypeInfo {
        LIGHTTRACKDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LIGHTTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("LightTrackData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct OrbisSystemCompanionUtilEventMessageBase {
}

pub trait OrbisSystemCompanionUtilEventMessageBaseTrait: TypeObject {
}

impl OrbisSystemCompanionUtilEventMessageBaseTrait for OrbisSystemCompanionUtilEventMessageBase {
}

pub static ORBISSYSTEMCOMPANIONUTILEVENTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OrbisSystemCompanionUtilEventMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OrbisSystemCompanionUtilEventMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for OrbisSystemCompanionUtilEventMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        ORBISSYSTEMCOMPANIONUTILEVENTMESSAGEBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct OrbisSystemCompanionHttpdEventMessageBase {
}

pub trait OrbisSystemCompanionHttpdEventMessageBaseTrait: TypeObject {
}

impl OrbisSystemCompanionHttpdEventMessageBaseTrait for OrbisSystemCompanionHttpdEventMessageBase {
}

pub static ORBISSYSTEMCOMPANIONHTTPDEVENTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OrbisSystemCompanionHttpdEventMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OrbisSystemCompanionHttpdEventMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for OrbisSystemCompanionHttpdEventMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        ORBISSYSTEMCOMPANIONHTTPDEVENTMESSAGEBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct OrbisSystemUserEventMessageBase {
}

pub trait OrbisSystemUserEventMessageBaseTrait: TypeObject {
}

impl OrbisSystemUserEventMessageBaseTrait for OrbisSystemUserEventMessageBase {
}

pub static ORBISSYSTEMUSEREVENTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OrbisSystemUserEventMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OrbisSystemUserEventMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for OrbisSystemUserEventMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        ORBISSYSTEMUSEREVENTMESSAGEBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct OrbisSystemTitleExternalEventMessageBase {
}

pub trait OrbisSystemTitleExternalEventMessageBaseTrait: TypeObject {
}

impl OrbisSystemTitleExternalEventMessageBaseTrait for OrbisSystemTitleExternalEventMessageBase {
}

pub static ORBISSYSTEMTITLEEXTERNALEVENTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OrbisSystemTitleExternalEventMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OrbisSystemTitleExternalEventMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for OrbisSystemTitleExternalEventMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        ORBISSYSTEMTITLEEXTERNALEVENTMESSAGEBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct OrbisSystemUiOverlaidStatusChangeEventMessage {
}

pub trait OrbisSystemUiOverlaidStatusChangeEventMessageTrait: TypeObject {
}

impl OrbisSystemUiOverlaidStatusChangeEventMessageTrait for OrbisSystemUiOverlaidStatusChangeEventMessage {
}

pub static ORBISSYSTEMUIOVERLAIDSTATUSCHANGEEVENTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OrbisSystemUiOverlaidStatusChangeEventMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OrbisSystemUiOverlaidStatusChangeEventMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for OrbisSystemUiOverlaidStatusChangeEventMessage {
    fn type_info(&self) -> &'static TypeInfo {
        ORBISSYSTEMUIOVERLAIDSTATUSCHANGEEVENTMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct OrbisSystemBackgroundStatusChangeEventMessage {
}

pub trait OrbisSystemBackgroundStatusChangeEventMessageTrait: TypeObject {
}

impl OrbisSystemBackgroundStatusChangeEventMessageTrait for OrbisSystemBackgroundStatusChangeEventMessage {
}

pub static ORBISSYSTEMBACKGROUNDSTATUSCHANGEEVENTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OrbisSystemBackgroundStatusChangeEventMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OrbisSystemBackgroundStatusChangeEventMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for OrbisSystemBackgroundStatusChangeEventMessage {
    fn type_info(&self) -> &'static TypeInfo {
        ORBISSYSTEMBACKGROUNDSTATUSCHANGEEVENTMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct PerfJournalBookmarkLabelEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub bookmark_label_name: String,
    pub enable: bool,
}

pub trait PerfJournalBookmarkLabelEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn bookmark_label_name(&self) -> &String;
    fn enable(&self) -> &bool;
}

impl PerfJournalBookmarkLabelEntityDataTrait for PerfJournalBookmarkLabelEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn bookmark_label_name(&self) -> &String {
        &self.bookmark_label_name
    }
    fn enable(&self) -> &bool {
        &self.enable
    }
}

impl super::entity::EntityDataTrait for PerfJournalBookmarkLabelEntityData {
}

impl super::entity::GameObjectDataTrait for PerfJournalBookmarkLabelEntityData {
}

impl super::core::DataBusPeerTrait for PerfJournalBookmarkLabelEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for PerfJournalBookmarkLabelEntityData {
}

impl super::core::DataContainerTrait for PerfJournalBookmarkLabelEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PERFJOURNALBOOKMARKLABELENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerfJournalBookmarkLabelEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PerfJournalBookmarkLabelEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(PerfJournalBookmarkLabelEntityData, realm),
            },
            FieldInfoData {
                name: "BookmarkLabelName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(PerfJournalBookmarkLabelEntityData, bookmark_label_name),
            },
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PerfJournalBookmarkLabelEntityData, enable),
            },
        ],
    }),
    array_type: Some(PERFJOURNALBOOKMARKLABELENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PerfJournalBookmarkLabelEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        PERFJOURNALBOOKMARKLABELENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PERFJOURNALBOOKMARKLABELENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerfJournalBookmarkLabelEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("PerfJournalBookmarkLabelEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PerfJournalBookmarkEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub enable: bool,
}

pub trait PerfJournalBookmarkEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn enable(&self) -> &bool;
}

impl PerfJournalBookmarkEntityDataTrait for PerfJournalBookmarkEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn enable(&self) -> &bool {
        &self.enable
    }
}

impl super::entity::EntityDataTrait for PerfJournalBookmarkEntityData {
}

impl super::entity::GameObjectDataTrait for PerfJournalBookmarkEntityData {
}

impl super::core::DataBusPeerTrait for PerfJournalBookmarkEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for PerfJournalBookmarkEntityData {
}

impl super::core::DataContainerTrait for PerfJournalBookmarkEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PERFJOURNALBOOKMARKENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerfJournalBookmarkEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PerfJournalBookmarkEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(PerfJournalBookmarkEntityData, realm),
            },
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PerfJournalBookmarkEntityData, enable),
            },
        ],
    }),
    array_type: Some(PERFJOURNALBOOKMARKENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PerfJournalBookmarkEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        PERFJOURNALBOOKMARKENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PERFJOURNALBOOKMARKENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerfJournalBookmarkEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("PerfJournalBookmarkEntityData"),
    array_type: None,
    alignment: 8,
};



pub static GETFLOATPROFILEOPTION_FLOAT32_LOCALPLAYERID_PROFILEOPTIONSTYPE_CSTRING_FLOAT32__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GetFloatProfileOption(Float32,LocalPlayerId,ProfileOptionsType,CString,Float32)",
    flags: MemberInfoFlags::new(793),
    module: "GameClient",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};



pub static GETINTPROFILEOPTION_INT32_LOCALPLAYERID_PROFILEOPTIONSTYPE_CSTRING_INT32__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GetIntProfileOption(Int32,LocalPlayerId,ProfileOptionsType,CString,Int32)",
    flags: MemberInfoFlags::new(793),
    module: "GameClient",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};



pub static GETBOOLPROFILEOPTION_BOOLEAN_LOCALPLAYERID_PROFILEOPTIONSTYPE_CSTRING_BOOLEAN__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GetBoolProfileOption(Boolean,LocalPlayerId,ProfileOptionsType,CString,Boolean)",
    flags: MemberInfoFlags::new(793),
    module: "GameClient",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIExitToMenuReasonMessage {
}

pub trait UIExitToMenuReasonMessageTrait: TypeObject {
}

impl UIExitToMenuReasonMessageTrait for UIExitToMenuReasonMessage {
}

pub static UIEXITTOMENUREASONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIExitToMenuReasonMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIExitToMenuReasonMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UIExitToMenuReasonMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UIEXITTOMENUREASONMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientCommanderPlayerChangedMessage {
}

pub trait ClientCommanderPlayerChangedMessageTrait: TypeObject {
}

impl ClientCommanderPlayerChangedMessageTrait for ClientCommanderPlayerChangedMessage {
}

pub static CLIENTCOMMANDERPLAYERCHANGEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCommanderPlayerChangedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCommanderPlayerChangedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientCommanderPlayerChangedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCOMMANDERPLAYERCHANGEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientStateChangingStateMessage {
}

pub trait ClientStateChangingStateMessageTrait: TypeObject {
}

impl ClientStateChangingStateMessageTrait for ClientStateChangingStateMessage {
}

pub static CLIENTSTATECHANGINGSTATEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStateChangingStateMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientStateChangingStateMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientStateChangingStateMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSTATECHANGINGSTATEMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientGameplayControllableLowHealthMessage {
}

pub trait ClientGameplayControllableLowHealthMessageTrait: TypeObject {
}

impl ClientGameplayControllableLowHealthMessageTrait for ClientGameplayControllableLowHealthMessage {
}

pub static CLIENTGAMEPLAYCONTROLLABLELOWHEALTHMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameplayControllableLowHealthMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientGameplayControllableLowHealthMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientGameplayControllableLowHealthMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTGAMEPLAYCONTROLLABLELOWHEALTHMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientGameplaySoldierHitMessage {
}

pub trait ClientGameplaySoldierHitMessageTrait: TypeObject {
}

impl ClientGameplaySoldierHitMessageTrait for ClientGameplaySoldierHitMessage {
}

pub static CLIENTGAMEPLAYSOLDIERHITMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameplaySoldierHitMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientGameplaySoldierHitMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientGameplaySoldierHitMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTGAMEPLAYSOLDIERHITMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientCameraShowKillerMessage {
}

pub trait ClientCameraShowKillerMessageTrait: TypeObject {
}

impl ClientCameraShowKillerMessageTrait for ClientCameraShowKillerMessage {
}

pub static CLIENTCAMERASHOWKILLERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCameraShowKillerMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCameraShowKillerMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientCameraShowKillerMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCAMERASHOWKILLERMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientCollisionProjectileImpactMessage {
}

pub trait ClientCollisionProjectileImpactMessageTrait: TypeObject {
}

impl ClientCollisionProjectileImpactMessageTrait for ClientCollisionProjectileImpactMessage {
}

pub static CLIENTCOLLISIONPROJECTILEIMPACTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCollisionProjectileImpactMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCollisionProjectileImpactMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ClientCollisionProjectileImpactMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCOLLISIONPROJECTILEIMPACTMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientCollisionSpawnExplosionMessage {
}

pub trait ClientCollisionSpawnExplosionMessageTrait: TypeObject {
}

impl ClientCollisionSpawnExplosionMessageTrait for ClientCollisionSpawnExplosionMessage {
}

pub static CLIENTCOLLISIONSPAWNEXPLOSIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCollisionSpawnExplosionMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCollisionSpawnExplosionMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ClientCollisionSpawnExplosionMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCOLLISIONSPAWNEXPLOSIONMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientCollisionExplosionPackDetonatedMessage {
}

pub trait ClientCollisionExplosionPackDetonatedMessageTrait: TypeObject {
}

impl ClientCollisionExplosionPackDetonatedMessageTrait for ClientCollisionExplosionPackDetonatedMessage {
}

pub static CLIENTCOLLISIONEXPLOSIONPACKDETONATEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCollisionExplosionPackDetonatedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCollisionExplosionPackDetonatedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientCollisionExplosionPackDetonatedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCOLLISIONEXPLOSIONPACKDETONATEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientCollisionExplosionPackPlacedMessage {
}

pub trait ClientCollisionExplosionPackPlacedMessageTrait: TypeObject {
}

impl ClientCollisionExplosionPackPlacedMessageTrait for ClientCollisionExplosionPackPlacedMessage {
}

pub static CLIENTCOLLISIONEXPLOSIONPACKPLACEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCollisionExplosionPackPlacedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCollisionExplosionPackPlacedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientCollisionExplosionPackPlacedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCOLLISIONEXPLOSIONPACKPLACEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientCollisionGrenadeCollisionMessage {
}

pub trait ClientCollisionGrenadeCollisionMessageTrait: TypeObject {
}

impl ClientCollisionGrenadeCollisionMessageTrait for ClientCollisionGrenadeCollisionMessage {
}

pub static CLIENTCOLLISIONGRENADECOLLISIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCollisionGrenadeCollisionMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCollisionGrenadeCollisionMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ClientCollisionGrenadeCollisionMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCOLLISIONGRENADECOLLISIONMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientSoundVoiceOverFinishedMessage {
}

pub trait ClientSoundVoiceOverFinishedMessageTrait: TypeObject {
}

impl ClientSoundVoiceOverFinishedMessageTrait for ClientSoundVoiceOverFinishedMessage {
}

pub static CLIENTSOUNDVOICEOVERFINISHEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoundVoiceOverFinishedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSoundVoiceOverFinishedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientSoundVoiceOverFinishedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSOUNDVOICEOVERFINISHEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientVehicleCriticalDamageMessage {
}

pub trait ClientVehicleCriticalDamageMessageTrait: TypeObject {
}

impl ClientVehicleCriticalDamageMessageTrait for ClientVehicleCriticalDamageMessage {
}

pub static CLIENTVEHICLECRITICALDAMAGEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVehicleCriticalDamageMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientVehicleCriticalDamageMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientVehicleCriticalDamageMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTVEHICLECRITICALDAMAGEMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientEntityDummyToGetTheClientEntityMessageCategoryMessage {
}

pub trait ClientEntityDummyToGetTheClientEntityMessageCategoryMessageTrait: TypeObject {
}

impl ClientEntityDummyToGetTheClientEntityMessageCategoryMessageTrait for ClientEntityDummyToGetTheClientEntityMessageCategoryMessage {
}

pub static CLIENTENTITYDUMMYTOGETTHECLIENTENTITYMESSAGECATEGORYMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEntityDummyToGetTheClientEntityMessageCategoryMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientEntityDummyToGetTheClientEntityMessageCategoryMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientEntityDummyToGetTheClientEntityMessageCategoryMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTENTITYDUMMYTOGETTHECLIENTENTITYMESSAGECATEGORYMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientCharacterSpawnDoneMessage {
}

pub trait ClientCharacterSpawnDoneMessageTrait: TypeObject {
}

impl ClientCharacterSpawnDoneMessageTrait for ClientCharacterSpawnDoneMessage {
}

pub static CLIENTCHARACTERSPAWNDONEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterSpawnDoneMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCharacterSpawnDoneMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientCharacterSpawnDoneMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCHARACTERSPAWNDONEMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientCharacterLocalPlayerDeletedMessage {
}

pub trait ClientCharacterLocalPlayerDeletedMessageTrait: TypeObject {
}

impl ClientCharacterLocalPlayerDeletedMessageTrait for ClientCharacterLocalPlayerDeletedMessage {
}

pub static CLIENTCHARACTERLOCALPLAYERDELETEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterLocalPlayerDeletedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCharacterLocalPlayerDeletedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientCharacterLocalPlayerDeletedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCHARACTERLOCALPLAYERDELETEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientControllableLocalPlayerSetMessage {
}

pub trait ClientControllableLocalPlayerSetMessageTrait: TypeObject {
}

impl ClientControllableLocalPlayerSetMessageTrait for ClientControllableLocalPlayerSetMessage {
}

pub static CLIENTCONTROLLABLELOCALPLAYERSETMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientControllableLocalPlayerSetMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientControllableLocalPlayerSetMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientControllableLocalPlayerSetMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCONTROLLABLELOCALPLAYERSETMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientWeaponComponentWeaponOnUnspawnMessage {
}

pub trait ClientWeaponComponentWeaponOnUnspawnMessageTrait: TypeObject {
}

impl ClientWeaponComponentWeaponOnUnspawnMessageTrait for ClientWeaponComponentWeaponOnUnspawnMessage {
}

pub static CLIENTWEAPONCOMPONENTWEAPONONUNSPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponComponentWeaponOnUnspawnMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientWeaponComponentWeaponOnUnspawnMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientWeaponComponentWeaponOnUnspawnMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTWEAPONCOMPONENTWEAPONONUNSPAWNMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientWeaponComponentWeaponOnSpawnMessage {
}

pub trait ClientWeaponComponentWeaponOnSpawnMessageTrait: TypeObject {
}

impl ClientWeaponComponentWeaponOnSpawnMessageTrait for ClientWeaponComponentWeaponOnSpawnMessage {
}

pub static CLIENTWEAPONCOMPONENTWEAPONONSPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponComponentWeaponOnSpawnMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientWeaponComponentWeaponOnSpawnMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientWeaponComponentWeaponOnSpawnMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTWEAPONCOMPONENTWEAPONONSPAWNMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientVehicleSetRemoteControlledDamageGiverMessage {
}

pub trait ClientVehicleSetRemoteControlledDamageGiverMessageTrait: TypeObject {
}

impl ClientVehicleSetRemoteControlledDamageGiverMessageTrait for ClientVehicleSetRemoteControlledDamageGiverMessage {
}

pub static CLIENTVEHICLESETREMOTECONTROLLEDDAMAGEGIVERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVehicleSetRemoteControlledDamageGiverMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientVehicleSetRemoteControlledDamageGiverMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientVehicleSetRemoteControlledDamageGiverMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTVEHICLESETREMOTECONTROLLEDDAMAGEGIVERMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientVehicleSpawnDoneMessage {
}

pub trait ClientVehicleSpawnDoneMessageTrait: TypeObject {
}

impl ClientVehicleSpawnDoneMessageTrait for ClientVehicleSpawnDoneMessage {
}

pub static CLIENTVEHICLESPAWNDONEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVehicleSpawnDoneMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientVehicleSpawnDoneMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientVehicleSpawnDoneMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTVEHICLESPAWNDONEMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientPlayerUnlocksUpdatedMessage {
}

pub trait ClientPlayerUnlocksUpdatedMessageTrait: TypeObject {
}

impl ClientPlayerUnlocksUpdatedMessageTrait for ClientPlayerUnlocksUpdatedMessage {
}

pub static CLIENTPLAYERUNLOCKSUPDATEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerUnlocksUpdatedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPlayerUnlocksUpdatedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientPlayerUnlocksUpdatedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPLAYERUNLOCKSUPDATEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientPlayerOnWeaponUndeployFinishedMessage {
}

pub trait ClientPlayerOnWeaponUndeployFinishedMessageTrait: TypeObject {
}

impl ClientPlayerOnWeaponUndeployFinishedMessageTrait for ClientPlayerOnWeaponUndeployFinishedMessage {
}

pub static CLIENTPLAYERONWEAPONUNDEPLOYFINISHEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerOnWeaponUndeployFinishedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPlayerOnWeaponUndeployFinishedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientPlayerOnWeaponUndeployFinishedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPLAYERONWEAPONUNDEPLOYFINISHEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientPlayerSelectedCustomizationMessage {
}

pub trait ClientPlayerSelectedCustomizationMessageTrait: TypeObject {
}

impl ClientPlayerSelectedCustomizationMessageTrait for ClientPlayerSelectedCustomizationMessage {
}

pub static CLIENTPLAYERSELECTEDCUSTOMIZATIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerSelectedCustomizationMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPlayerSelectedCustomizationMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientPlayerSelectedCustomizationMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPLAYERSELECTEDCUSTOMIZATIONMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientPlayerSelectedUnlocksMessage {
}

pub trait ClientPlayerSelectedUnlocksMessageTrait: TypeObject {
}

impl ClientPlayerSelectedUnlocksMessageTrait for ClientPlayerSelectedUnlocksMessage {
}

pub static CLIENTPLAYERSELECTEDUNLOCKSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerSelectedUnlocksMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPlayerSelectedUnlocksMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientPlayerSelectedUnlocksMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPLAYERSELECTEDUNLOCKSMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientPlayerManDownMessage {
}

pub trait ClientPlayerManDownMessageTrait: TypeObject {
}

impl ClientPlayerManDownMessageTrait for ClientPlayerManDownMessage {
}

pub static CLIENTPLAYERMANDOWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerManDownMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPlayerManDownMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientPlayerManDownMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPLAYERMANDOWNMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientPlayerWeaponPickupMessage {
}

pub trait ClientPlayerWeaponPickupMessageTrait: TypeObject {
}

impl ClientPlayerWeaponPickupMessageTrait for ClientPlayerWeaponPickupMessage {
}

pub static CLIENTPLAYERWEAPONPICKUPMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerWeaponPickupMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPlayerWeaponPickupMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientPlayerWeaponPickupMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPLAYERWEAPONPICKUPMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientPlayerAmmoPickupMessage {
}

pub trait ClientPlayerAmmoPickupMessageTrait: TypeObject {
}

impl ClientPlayerAmmoPickupMessageTrait for ClientPlayerAmmoPickupMessage {
}

pub static CLIENTPLAYERAMMOPICKUPMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerAmmoPickupMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPlayerAmmoPickupMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientPlayerAmmoPickupMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPLAYERAMMOPICKUPMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientPlayerKilledMessage {
}

pub trait ClientPlayerKilledMessageTrait: TypeObject {
}

impl ClientPlayerKilledMessageTrait for ClientPlayerKilledMessage {
}

pub static CLIENTPLAYERKILLEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerKilledMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPlayerKilledMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientPlayerKilledMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPLAYERKILLEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientPlayerEnterExitVehicleMessage {
}

pub trait ClientPlayerEnterExitVehicleMessageTrait: TypeObject {
}

impl ClientPlayerEnterExitVehicleMessageTrait for ClientPlayerEnterExitVehicleMessage {
}

pub static CLIENTPLAYERENTEREXITVEHICLEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerEnterExitVehicleMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPlayerEnterExitVehicleMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientPlayerEnterExitVehicleMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPLAYERENTEREXITVEHICLEMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientMetricsPauseGameMessage {
}

pub trait ClientMetricsPauseGameMessageTrait: TypeObject {
}

impl ClientMetricsPauseGameMessageTrait for ClientMetricsPauseGameMessage {
}

pub static CLIENTMETRICSPAUSEGAMEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMetricsPauseGameMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientMetricsPauseGameMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientMetricsPauseGameMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTMETRICSPAUSEGAMEMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientMetricsUIActionMessage {
}

pub trait ClientMetricsUIActionMessageTrait: TypeObject {
}

impl ClientMetricsUIActionMessageTrait for ClientMetricsUIActionMessage {
}

pub static CLIENTMETRICSUIACTIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMetricsUIActionMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameClient",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientMetricsUIActionMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientMetricsUIActionMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTMETRICSUIACTIONMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientMetricsPopUIScreenMessage {
}

pub trait ClientMetricsPopUIScreenMessageTrait: TypeObject {
}

impl ClientMetricsPopUIScreenMessageTrait for ClientMetricsPopUIScreenMessage {
}

pub static CLIENTMETRICSPOPUISCREENMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMetricsPopUIScreenMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientMetricsPopUIScreenMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientMetricsPopUIScreenMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTMETRICSPOPUISCREENMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientMetricsPushUIScreenMessage {
}

pub trait ClientMetricsPushUIScreenMessageTrait: TypeObject {
}

impl ClientMetricsPushUIScreenMessageTrait for ClientMetricsPushUIScreenMessage {
}

pub static CLIENTMETRICSPUSHUISCREENMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMetricsPushUIScreenMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClient",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientMetricsPushUIScreenMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientMetricsPushUIScreenMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTMETRICSPUSHUISCREENMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ClientGameType {
    #[default]
    ClientGameType_SinglePlayer = 0,
    ClientGameType_Hosted = 1,
    ClientGameType_MpTutorial = 2,
    ClientGameType_Joined = 3,
}

pub static CLIENTGAMETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameType",
    flags: MemberInfoFlags::new(49429),
    module: "GameClient",
    data: TypeInfoData::Enum,
    array_type: Some(CLIENTGAMETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ClientGameType {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTGAMETYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTGAMETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameType-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientGameType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static CLIENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientState",
    flags: MemberInfoFlags::new(49429),
    module: "GameClient",
    data: TypeInfoData::Enum,
    array_type: Some(CLIENTSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ClientState {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SubLevelVisibleEntityData {
    pub _glacier_base: super::entity::EntityData,
}

pub trait SubLevelVisibleEntityDataTrait: super::entity::EntityDataTrait {
}

impl SubLevelVisibleEntityDataTrait for SubLevelVisibleEntityData {
}

impl super::entity::EntityDataTrait for SubLevelVisibleEntityData {
}

impl super::entity::GameObjectDataTrait for SubLevelVisibleEntityData {
}

impl super::core::DataBusPeerTrait for SubLevelVisibleEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for SubLevelVisibleEntityData {
}

impl super::core::DataContainerTrait for SubLevelVisibleEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static SUBLEVELVISIBLEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelVisibleEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SubLevelVisibleEntityData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SUBLEVELVISIBLEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SubLevelVisibleEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        SUBLEVELVISIBLEENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SUBLEVELVISIBLEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelVisibleEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("SubLevelVisibleEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientGroupComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientGroupComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientGroupComponentTrait for ClientGroupComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientGroupComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientGroupComponent {
}

impl super::entity::ComponentTrait for ClientGroupComponent {
}

impl super::entity::EntityBusPeerTrait for ClientGroupComponent {
}

pub static CLIENTGROUPCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGroupComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientGroupComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTGROUPCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientGroupComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTGROUPCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTGROUPCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGroupComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientGroupComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientEventSyncEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientEventSyncEntityTrait: super::entity::EntityTrait {
}

impl ClientEventSyncEntityTrait for ClientEventSyncEntity {
}

impl super::entity::EntityTrait for ClientEventSyncEntity {
}

impl super::entity::EntityBusPeerTrait for ClientEventSyncEntity {
}

pub static CLIENTEVENTSYNCENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEventSyncEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientEventSyncEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTEVENTSYNCENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientEventSyncEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTEVENTSYNCENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTEVENTSYNCENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEventSyncEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientEventSyncEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientBlueprintEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientBlueprintEntityTrait: super::entity::EntityTrait {
}

impl ClientBlueprintEntityTrait for ClientBlueprintEntity {
}

impl super::entity::EntityTrait for ClientBlueprintEntity {
}

impl super::entity::EntityBusPeerTrait for ClientBlueprintEntity {
}

pub static CLIENTBLUEPRINTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBlueprintEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientBlueprintEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTBLUEPRINTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientBlueprintEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTBLUEPRINTENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTBLUEPRINTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBlueprintEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientBlueprintEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VoiceOverManagerControlEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait VoiceOverManagerControlEntityTrait: super::entity::EntityTrait {
}

impl VoiceOverManagerControlEntityTrait for VoiceOverManagerControlEntity {
}

impl super::entity::EntityTrait for VoiceOverManagerControlEntity {
}

impl super::entity::EntityBusPeerTrait for VoiceOverManagerControlEntity {
}

pub static VOICEOVERMANAGERCONTROLENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceOverManagerControlEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VoiceOverManagerControlEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(VOICEOVERMANAGERCONTROLENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for VoiceOverManagerControlEntity {
    fn type_info(&self) -> &'static TypeInfo {
        VOICEOVERMANAGERCONTROLENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VOICEOVERMANAGERCONTROLENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceOverManagerControlEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("VoiceOverManagerControlEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VoiceOverEventEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait VoiceOverEventEntityTrait: super::entity::EntityTrait {
}

impl VoiceOverEventEntityTrait for VoiceOverEventEntity {
}

impl super::entity::EntityTrait for VoiceOverEventEntity {
}

impl super::entity::EntityBusPeerTrait for VoiceOverEventEntity {
}

pub static VOICEOVEREVENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceOverEventEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VoiceOverEventEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(VOICEOVEREVENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for VoiceOverEventEntity {
    fn type_info(&self) -> &'static TypeInfo {
        VOICEOVEREVENTENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VOICEOVEREVENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceOverEventEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("VoiceOverEventEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VoiceOverConversationEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait VoiceOverConversationEntityTrait: super::entity::EntityTrait {
}

impl VoiceOverConversationEntityTrait for VoiceOverConversationEntity {
}

impl super::entity::EntityTrait for VoiceOverConversationEntity {
}

impl super::entity::EntityBusPeerTrait for VoiceOverConversationEntity {
}

pub static VOICEOVERCONVERSATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceOverConversationEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VoiceOverConversationEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(VOICEOVERCONVERSATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for VoiceOverConversationEntity {
    fn type_info(&self) -> &'static TypeInfo {
        VOICEOVERCONVERSATIONENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VOICEOVERCONVERSATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceOverConversationEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("VoiceOverConversationEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientVoiceOverAnimationComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientVoiceOverAnimationComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientVoiceOverAnimationComponentTrait for ClientVoiceOverAnimationComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientVoiceOverAnimationComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientVoiceOverAnimationComponent {
}

impl super::entity::ComponentTrait for ClientVoiceOverAnimationComponent {
}

impl super::entity::EntityBusPeerTrait for ClientVoiceOverAnimationComponent {
}

pub static CLIENTVOICEOVERANIMATIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverAnimationComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientVoiceOverAnimationComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVOICEOVERANIMATIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientVoiceOverAnimationComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTVOICEOVERANIMATIONCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTVOICEOVERANIMATIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverAnimationComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientVoiceOverAnimationComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SoundListenerEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait SoundListenerEntityTrait: super::entity::EntityTrait {
}

impl SoundListenerEntityTrait for SoundListenerEntity {
}

impl super::entity::EntityTrait for SoundListenerEntity {
}

impl super::entity::EntityBusPeerTrait for SoundListenerEntity {
}

pub static SOUNDLISTENERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoundListenerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SoundListenerEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SOUNDLISTENERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SoundListenerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SOUNDLISTENERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SOUNDLISTENERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoundListenerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("SoundListenerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SoundAreaEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait SoundAreaEntityTrait: super::entity::EntityTrait {
}

impl SoundAreaEntityTrait for SoundAreaEntity {
}

impl super::entity::EntityTrait for SoundAreaEntity {
}

impl super::entity::EntityBusPeerTrait for SoundAreaEntity {
}

pub static SOUNDAREAENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoundAreaEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SoundAreaEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SOUNDAREAENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SoundAreaEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SOUNDAREAENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SOUNDAREAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoundAreaEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("SoundAreaEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DetachedSoundListenerEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait DetachedSoundListenerEntityTrait: super::entity::EntityTrait {
}

impl DetachedSoundListenerEntityTrait for DetachedSoundListenerEntity {
}

impl super::entity::EntityTrait for DetachedSoundListenerEntity {
}

impl super::entity::EntityBusPeerTrait for DetachedSoundListenerEntity {
}

pub static DETACHEDSOUNDLISTENERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DetachedSoundListenerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DetachedSoundListenerEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DETACHEDSOUNDLISTENERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DetachedSoundListenerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        DETACHEDSOUNDLISTENERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DETACHEDSOUNDLISTENERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DetachedSoundListenerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("DetachedSoundListenerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientWaterPhysicsComponent {
    pub _glacier_base: super::physics::PhysicsComponent,
}

pub trait ClientWaterPhysicsComponentTrait: super::physics::PhysicsComponentTrait {
}

impl ClientWaterPhysicsComponentTrait for ClientWaterPhysicsComponent {
}

impl super::physics::PhysicsComponentTrait for ClientWaterPhysicsComponent {
}

impl super::entity::ComponentTrait for ClientWaterPhysicsComponent {
}

impl super::entity::EntityBusPeerTrait for ClientWaterPhysicsComponent {
}

pub static CLIENTWATERPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWaterPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::PHYSICSCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientWaterPhysicsComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWATERPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWaterPhysicsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTWATERPHYSICSCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTWATERPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWaterPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientWaterPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientWaterHealthComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameHealthComponent,
}

pub trait ClientWaterHealthComponentTrait: super::gameplay_client_server::ClientGameHealthComponentTrait {
}

impl ClientWaterHealthComponentTrait for ClientWaterHealthComponent {
}

impl super::gameplay_client_server::ClientGameHealthComponentTrait for ClientWaterHealthComponent {
}

impl super::gameplay_sim::HealthComponentTrait for ClientWaterHealthComponent {
}

impl super::entity::ComponentTrait for ClientWaterHealthComponent {
}

impl super::entity::EntityBusPeerTrait for ClientWaterHealthComponent {
}

pub static CLIENTWATERHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWaterHealthComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMEHEALTHCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientWaterHealthComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWATERHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWaterHealthComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTWATERHEALTHCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTWATERHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWaterHealthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientWaterHealthComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientTerrainPhysicsComponent {
    pub _glacier_base: super::physics::PhysicsComponent,
}

pub trait ClientTerrainPhysicsComponentTrait: super::physics::PhysicsComponentTrait {
}

impl ClientTerrainPhysicsComponentTrait for ClientTerrainPhysicsComponent {
}

impl super::physics::PhysicsComponentTrait for ClientTerrainPhysicsComponent {
}

impl super::entity::ComponentTrait for ClientTerrainPhysicsComponent {
}

impl super::entity::EntityBusPeerTrait for ClientTerrainPhysicsComponent {
}

pub static CLIENTTERRAINPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTerrainPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::PHYSICSCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientTerrainPhysicsComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTTERRAINPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientTerrainPhysicsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTTERRAINPHYSICSCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTTERRAINPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTerrainPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientTerrainPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientTerrainHealthComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameHealthComponent,
}

pub trait ClientTerrainHealthComponentTrait: super::gameplay_client_server::ClientGameHealthComponentTrait {
}

impl ClientTerrainHealthComponentTrait for ClientTerrainHealthComponent {
}

impl super::gameplay_client_server::ClientGameHealthComponentTrait for ClientTerrainHealthComponent {
}

impl super::gameplay_sim::HealthComponentTrait for ClientTerrainHealthComponent {
}

impl super::entity::ComponentTrait for ClientTerrainHealthComponent {
}

impl super::entity::EntityBusPeerTrait for ClientTerrainHealthComponent {
}

pub static CLIENTTERRAINHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTerrainHealthComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMEHEALTHCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientTerrainHealthComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTTERRAINHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientTerrainHealthComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTTERRAINHEALTHCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTTERRAINHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTerrainHealthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientTerrainHealthComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientStaticModelPhysicsComponent {
    pub _glacier_base: super::physics::PartPhysicsComponent,
}

pub trait ClientStaticModelPhysicsComponentTrait: super::physics::PartPhysicsComponentTrait {
}

impl ClientStaticModelPhysicsComponentTrait for ClientStaticModelPhysicsComponent {
}

impl super::physics::PartPhysicsComponentTrait for ClientStaticModelPhysicsComponent {
}

impl super::physics::PhysicsComponentTrait for ClientStaticModelPhysicsComponent {
}

impl super::entity::ComponentTrait for ClientStaticModelPhysicsComponent {
}

impl super::entity::EntityBusPeerTrait for ClientStaticModelPhysicsComponent {
}

pub static CLIENTSTATICMODELPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticModelPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::PARTPHYSICSCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientStaticModelPhysicsComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSTATICMODELPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientStaticModelPhysicsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSTATICMODELPHYSICSCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTSTATICMODELPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticModelPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientStaticModelPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientStaticModelHealthComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameHealthComponent,
}

pub trait ClientStaticModelHealthComponentTrait: super::gameplay_client_server::ClientGameHealthComponentTrait {
}

impl ClientStaticModelHealthComponentTrait for ClientStaticModelHealthComponent {
}

impl super::gameplay_client_server::ClientGameHealthComponentTrait for ClientStaticModelHealthComponent {
}

impl super::gameplay_sim::HealthComponentTrait for ClientStaticModelHealthComponent {
}

impl super::entity::ComponentTrait for ClientStaticModelHealthComponent {
}

impl super::entity::EntityBusPeerTrait for ClientStaticModelHealthComponent {
}

pub static CLIENTSTATICMODELHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticModelHealthComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMEHEALTHCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientStaticModelHealthComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSTATICMODELHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientStaticModelHealthComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSTATICMODELHEALTHCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTSTATICMODELHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticModelHealthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientStaticModelHealthComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientStaticModelGroupPhysicsComponent {
    pub _glacier_base: super::physics::GroupPhysicsComponent,
}

pub trait ClientStaticModelGroupPhysicsComponentTrait: super::physics::GroupPhysicsComponentTrait {
}

impl ClientStaticModelGroupPhysicsComponentTrait for ClientStaticModelGroupPhysicsComponent {
}

impl super::physics::GroupPhysicsComponentTrait for ClientStaticModelGroupPhysicsComponent {
}

impl super::physics::PhysicsComponentTrait for ClientStaticModelGroupPhysicsComponent {
}

impl super::entity::ComponentTrait for ClientStaticModelGroupPhysicsComponent {
}

impl super::entity::EntityBusPeerTrait for ClientStaticModelGroupPhysicsComponent {
}

pub static CLIENTSTATICMODELGROUPPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticModelGroupPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::GROUPPHYSICSCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientStaticModelGroupPhysicsComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSTATICMODELGROUPPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientStaticModelGroupPhysicsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSTATICMODELGROUPPHYSICSCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTSTATICMODELGROUPPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticModelGroupPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientStaticModelGroupPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientStaticModelGroupHealthComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameHealthComponent,
}

pub trait ClientStaticModelGroupHealthComponentTrait: super::gameplay_client_server::ClientGameHealthComponentTrait {
}

impl ClientStaticModelGroupHealthComponentTrait for ClientStaticModelGroupHealthComponent {
}

impl super::gameplay_client_server::ClientGameHealthComponentTrait for ClientStaticModelGroupHealthComponent {
}

impl super::gameplay_sim::HealthComponentTrait for ClientStaticModelGroupHealthComponent {
}

impl super::entity::ComponentTrait for ClientStaticModelGroupHealthComponent {
}

impl super::entity::EntityBusPeerTrait for ClientStaticModelGroupHealthComponent {
}

pub static CLIENTSTATICMODELGROUPHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticModelGroupHealthComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMEHEALTHCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientStaticModelGroupHealthComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSTATICMODELGROUPHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientStaticModelGroupHealthComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSTATICMODELGROUPHEALTHCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTSTATICMODELGROUPHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticModelGroupHealthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientStaticModelGroupHealthComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientBoxReflectionVolumeComponent {
    pub _glacier_base: ClientReflectionVolumeComponent,
}

pub trait ClientBoxReflectionVolumeComponentTrait: ClientReflectionVolumeComponentTrait {
}

impl ClientBoxReflectionVolumeComponentTrait for ClientBoxReflectionVolumeComponent {
}

impl ClientReflectionVolumeComponentTrait for ClientBoxReflectionVolumeComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientBoxReflectionVolumeComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientBoxReflectionVolumeComponent {
}

impl super::entity::ComponentTrait for ClientBoxReflectionVolumeComponent {
}

impl super::entity::EntityBusPeerTrait for ClientBoxReflectionVolumeComponent {
}

pub static CLIENTBOXREFLECTIONVOLUMECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBoxReflectionVolumeComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTREFLECTIONVOLUMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientBoxReflectionVolumeComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTBOXREFLECTIONVOLUMECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientBoxReflectionVolumeComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTBOXREFLECTIONVOLUMECOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTBOXREFLECTIONVOLUMECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBoxReflectionVolumeComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientBoxReflectionVolumeComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientSphereReflectionVolumeComponent {
    pub _glacier_base: ClientReflectionVolumeComponent,
}

pub trait ClientSphereReflectionVolumeComponentTrait: ClientReflectionVolumeComponentTrait {
}

impl ClientSphereReflectionVolumeComponentTrait for ClientSphereReflectionVolumeComponent {
}

impl ClientReflectionVolumeComponentTrait for ClientSphereReflectionVolumeComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientSphereReflectionVolumeComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientSphereReflectionVolumeComponent {
}

impl super::entity::ComponentTrait for ClientSphereReflectionVolumeComponent {
}

impl super::entity::EntityBusPeerTrait for ClientSphereReflectionVolumeComponent {
}

pub static CLIENTSPHEREREFLECTIONVOLUMECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSphereReflectionVolumeComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTREFLECTIONVOLUMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSphereReflectionVolumeComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSPHEREREFLECTIONVOLUMECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSphereReflectionVolumeComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSPHEREREFLECTIONVOLUMECOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTSPHEREREFLECTIONVOLUMECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSphereReflectionVolumeComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientSphereReflectionVolumeComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientReflectionVolumeComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientReflectionVolumeComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientReflectionVolumeComponentTrait for ClientReflectionVolumeComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientReflectionVolumeComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientReflectionVolumeComponent {
}

impl super::entity::ComponentTrait for ClientReflectionVolumeComponent {
}

impl super::entity::EntityBusPeerTrait for ClientReflectionVolumeComponent {
}

pub static CLIENTREFLECTIONVOLUMECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientReflectionVolumeComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientReflectionVolumeComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTREFLECTIONVOLUMECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientReflectionVolumeComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTREFLECTIONVOLUMECOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTREFLECTIONVOLUMECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientReflectionVolumeComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientReflectionVolumeComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientLightComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientLightComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientLightComponentTrait for ClientLightComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientLightComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientLightComponent {
}

impl super::entity::ComponentTrait for ClientLightComponent {
}

impl super::entity::EntityBusPeerTrait for ClientLightComponent {
}

pub static CLIENTLIGHTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLightComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientLightComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTLIGHTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientLightComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTLIGHTCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTLIGHTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLightComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientLightComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientLensFlareComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientLensFlareComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientLensFlareComponentTrait for ClientLensFlareComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientLensFlareComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientLensFlareComponent {
}

impl super::entity::ComponentTrait for ClientLensFlareComponent {
}

impl super::entity::EntityBusPeerTrait for ClientLensFlareComponent {
}

pub static CLIENTLENSFLARECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLensFlareComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientLensFlareComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTLENSFLARECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientLensFlareComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTLENSFLARECOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTLENSFLARECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLensFlareComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientLensFlareComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientEffectComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientEffectComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientEffectComponentTrait for ClientEffectComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientEffectComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientEffectComponent {
}

impl super::entity::ComponentTrait for ClientEffectComponent {
}

impl super::entity::EntityBusPeerTrait for ClientEffectComponent {
}

pub static CLIENTEFFECTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEffectComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientEffectComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTEFFECTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientEffectComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTEFFECTCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTEFFECTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEffectComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientEffectComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientDynamicModelPhysicsComponent {
    pub _glacier_base: super::physics::PartPhysicsComponent,
}

pub trait ClientDynamicModelPhysicsComponentTrait: super::physics::PartPhysicsComponentTrait {
}

impl ClientDynamicModelPhysicsComponentTrait for ClientDynamicModelPhysicsComponent {
}

impl super::physics::PartPhysicsComponentTrait for ClientDynamicModelPhysicsComponent {
}

impl super::physics::PhysicsComponentTrait for ClientDynamicModelPhysicsComponent {
}

impl super::entity::ComponentTrait for ClientDynamicModelPhysicsComponent {
}

impl super::entity::EntityBusPeerTrait for ClientDynamicModelPhysicsComponent {
}

pub static CLIENTDYNAMICMODELPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDynamicModelPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::PARTPHYSICSCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientDynamicModelPhysicsComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTDYNAMICMODELPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientDynamicModelPhysicsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTDYNAMICMODELPHYSICSCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTDYNAMICMODELPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDynamicModelPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientDynamicModelPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientDynamicModelHealthComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameHealthComponent,
}

pub trait ClientDynamicModelHealthComponentTrait: super::gameplay_client_server::ClientGameHealthComponentTrait {
}

impl ClientDynamicModelHealthComponentTrait for ClientDynamicModelHealthComponent {
}

impl super::gameplay_client_server::ClientGameHealthComponentTrait for ClientDynamicModelHealthComponent {
}

impl super::gameplay_sim::HealthComponentTrait for ClientDynamicModelHealthComponent {
}

impl super::entity::ComponentTrait for ClientDynamicModelHealthComponent {
}

impl super::entity::EntityBusPeerTrait for ClientDynamicModelHealthComponent {
}

pub static CLIENTDYNAMICMODELHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDynamicModelHealthComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMEHEALTHCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientDynamicModelHealthComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTDYNAMICMODELHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientDynamicModelHealthComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTDYNAMICMODELHEALTHCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTDYNAMICMODELHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDynamicModelHealthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientDynamicModelHealthComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientBangerPhysicsComponent {
    pub _glacier_base: super::physics::PartPhysicsComponent,
}

pub trait ClientBangerPhysicsComponentTrait: super::physics::PartPhysicsComponentTrait {
}

impl ClientBangerPhysicsComponentTrait for ClientBangerPhysicsComponent {
}

impl super::physics::PartPhysicsComponentTrait for ClientBangerPhysicsComponent {
}

impl super::physics::PhysicsComponentTrait for ClientBangerPhysicsComponent {
}

impl super::entity::ComponentTrait for ClientBangerPhysicsComponent {
}

impl super::entity::EntityBusPeerTrait for ClientBangerPhysicsComponent {
}

pub static CLIENTBANGERPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBangerPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::PARTPHYSICSCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientBangerPhysicsComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTBANGERPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientBangerPhysicsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTBANGERPHYSICSCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTBANGERPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBangerPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientBangerPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientBangerHealthComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameHealthComponent,
}

pub trait ClientBangerHealthComponentTrait: super::gameplay_client_server::ClientGameHealthComponentTrait {
}

impl ClientBangerHealthComponentTrait for ClientBangerHealthComponent {
}

impl super::gameplay_client_server::ClientGameHealthComponentTrait for ClientBangerHealthComponent {
}

impl super::gameplay_sim::HealthComponentTrait for ClientBangerHealthComponent {
}

impl super::entity::ComponentTrait for ClientBangerHealthComponent {
}

impl super::entity::EntityBusPeerTrait for ClientBangerHealthComponent {
}

pub static CLIENTBANGERHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBangerHealthComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMEHEALTHCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientBangerHealthComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTBANGERHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientBangerHealthComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTBANGERHEALTHCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTBANGERHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBangerHealthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientBangerHealthComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientSpectatorSubView {
    pub _glacier_base: super::gameplay_client_server::ClientSubView,
}

pub trait ClientSpectatorSubViewTrait: super::gameplay_client_server::ClientSubViewTrait {
}

impl ClientSpectatorSubViewTrait for ClientSpectatorSubView {
}

impl super::gameplay_client_server::ClientSubViewTrait for ClientSpectatorSubView {
}

impl super::gameplay_sim::SubViewTrait for ClientSpectatorSubView {
}

pub static CLIENTSPECTATORSUBVIEW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSpectatorSubView",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTSUBVIEW_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSpectatorSubView as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSPECTATORSUBVIEW_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSpectatorSubView {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSPECTATORSUBVIEW_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTSPECTATORSUBVIEW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSpectatorSubView-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientSpectatorSubView"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientGameSplineEntity {
    pub _glacier_base: super::entity::SpatialEntity,
}

pub trait ClientGameSplineEntityTrait: super::entity::SpatialEntityTrait {
}

impl ClientGameSplineEntityTrait for ClientGameSplineEntity {
}

impl super::entity::SpatialEntityTrait for ClientGameSplineEntity {
}

impl super::entity::EntityTrait for ClientGameSplineEntity {
}

impl super::entity::EntityBusPeerTrait for ClientGameSplineEntity {
}

pub static CLIENTGAMESPLINEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameSplineEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientGameSplineEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTGAMESPLINEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientGameSplineEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTGAMESPLINEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTGAMESPLINEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameSplineEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientGameSplineEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientAreaImmunityComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientAreaImmunityComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientAreaImmunityComponentTrait for ClientAreaImmunityComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientAreaImmunityComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientAreaImmunityComponent {
}

impl super::entity::ComponentTrait for ClientAreaImmunityComponent {
}

impl super::entity::EntityBusPeerTrait for ClientAreaImmunityComponent {
}

pub static CLIENTAREAIMMUNITYCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAreaImmunityComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAreaImmunityComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTAREAIMMUNITYCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAreaImmunityComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTAREAIMMUNITYCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTAREAIMMUNITYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAreaImmunityComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientAreaImmunityComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientDynamicFireEntity {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponentEntity,
}

pub trait ClientDynamicFireEntityTrait: super::gameplay_client_server::ClientGameComponentEntityTrait {
}

impl ClientDynamicFireEntityTrait for ClientDynamicFireEntity {
}

impl super::gameplay_client_server::ClientGameComponentEntityTrait for ClientDynamicFireEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ClientDynamicFireEntity {
}

impl super::entity::ComponentEntityTrait for ClientDynamicFireEntity {
}

impl super::entity::SpatialEntityTrait for ClientDynamicFireEntity {
}

impl super::entity::EntityTrait for ClientDynamicFireEntity {
}

impl super::entity::EntityBusPeerTrait for ClientDynamicFireEntity {
}

pub static CLIENTDYNAMICFIREENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDynamicFireEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENTENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientDynamicFireEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTDYNAMICFIREENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientDynamicFireEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTDYNAMICFIREENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTDYNAMICFIREENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDynamicFireEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientDynamicFireEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientWarningSystemComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientWarningSystemComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientWarningSystemComponentTrait for ClientWarningSystemComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientWarningSystemComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientWarningSystemComponent {
}

impl super::entity::ComponentTrait for ClientWarningSystemComponent {
}

impl super::entity::EntityBusPeerTrait for ClientWarningSystemComponent {
}

pub static CLIENTWARNINGSYSTEMCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWarningSystemComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientWarningSystemComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWARNINGSYSTEMCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWarningSystemComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTWARNINGSYSTEMCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTWARNINGSYSTEMCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWarningSystemComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientWarningSystemComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientUnlockComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientUnlockComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientUnlockComponentTrait for ClientUnlockComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientUnlockComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientUnlockComponent {
}

impl super::entity::ComponentTrait for ClientUnlockComponent {
}

impl super::entity::EntityBusPeerTrait for ClientUnlockComponent {
}

pub static CLIENTUNLOCKCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientUnlockComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientUnlockComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTUNLOCKCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientUnlockComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTUNLOCKCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTUNLOCKCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientUnlockComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientUnlockComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientPlayerEntryComponent {
    pub _glacier_base: ClientGameEntryComponent,
}

pub trait ClientPlayerEntryComponentTrait: ClientGameEntryComponentTrait {
}

impl ClientPlayerEntryComponentTrait for ClientPlayerEntryComponent {
}

impl ClientGameEntryComponentTrait for ClientPlayerEntryComponent {
}

impl super::gameplay_client_server::ClientEntryComponentTrait for ClientPlayerEntryComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientPlayerEntryComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientPlayerEntryComponent {
}

impl super::entity::ComponentTrait for ClientPlayerEntryComponent {
}

impl super::entity::EntityBusPeerTrait for ClientPlayerEntryComponent {
}

pub static CLIENTPLAYERENTRYCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerEntryComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMEENTRYCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPlayerEntryComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPLAYERENTRYCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPlayerEntryComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPLAYERENTRYCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTPLAYERENTRYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerEntryComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientPlayerEntryComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientGameEntryComponent {
    pub _glacier_base: super::gameplay_client_server::ClientEntryComponent,
}

pub trait ClientGameEntryComponentTrait: super::gameplay_client_server::ClientEntryComponentTrait {
}

impl ClientGameEntryComponentTrait for ClientGameEntryComponent {
}

impl super::gameplay_client_server::ClientEntryComponentTrait for ClientGameEntryComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientGameEntryComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientGameEntryComponent {
}

impl super::entity::ComponentTrait for ClientGameEntryComponent {
}

impl super::entity::EntityBusPeerTrait for ClientGameEntryComponent {
}

pub static CLIENTGAMEENTRYCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameEntryComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTENTRYCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientGameEntryComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTGAMEENTRYCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientGameEntryComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTGAMEENTRYCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTGAMEENTRYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameEntryComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientGameEntryComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientCharacterEntity {
    pub _glacier_base: super::gameplay_client_server::ClientControllableEntity,
}

pub trait ClientCharacterEntityTrait: super::gameplay_client_server::ClientControllableEntityTrait {
}

impl ClientCharacterEntityTrait for ClientCharacterEntity {
}

impl super::gameplay_client_server::ClientControllableEntityTrait for ClientCharacterEntity {
}

impl super::gameplay_client_server::ClientPhysicsEntityTrait for ClientCharacterEntity {
}

impl super::gameplay_client_server::ClientGameComponentEntityTrait for ClientCharacterEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ClientCharacterEntity {
}

impl super::entity::ComponentEntityTrait for ClientCharacterEntity {
}

impl super::entity::SpatialEntityTrait for ClientCharacterEntity {
}

impl super::entity::EntityTrait for ClientCharacterEntity {
}

impl super::entity::EntityBusPeerTrait for ClientCharacterEntity {
}

pub static CLIENTCHARACTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTCONTROLLABLEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCharacterEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCHARACTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCharacterEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCHARACTERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTCHARACTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientCharacterEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientWarpAnimationComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientWarpAnimationComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientWarpAnimationComponentTrait for ClientWarpAnimationComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientWarpAnimationComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientWarpAnimationComponent {
}

impl super::entity::ComponentTrait for ClientWarpAnimationComponent {
}

impl super::entity::EntityBusPeerTrait for ClientWarpAnimationComponent {
}

pub static CLIENTWARPANIMATIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWarpAnimationComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientWarpAnimationComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWARPANIMATIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWarpAnimationComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTWARPANIMATIONCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTWARPANIMATIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWarpAnimationComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientWarpAnimationComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientVehicleEntryListenerComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientVehicleEntryListenerComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientVehicleEntryListenerComponentTrait for ClientVehicleEntryListenerComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientVehicleEntryListenerComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientVehicleEntryListenerComponent {
}

impl super::entity::ComponentTrait for ClientVehicleEntryListenerComponent {
}

impl super::entity::EntityBusPeerTrait for ClientVehicleEntryListenerComponent {
}

pub static CLIENTVEHICLEENTRYLISTENERCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVehicleEntryListenerComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientVehicleEntryListenerComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVEHICLEENTRYLISTENERCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientVehicleEntryListenerComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTVEHICLEENTRYLISTENERCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTVEHICLEENTRYLISTENERCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVehicleEntryListenerComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientVehicleEntryListenerComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientCharacterPhysicsComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientCharacterPhysicsComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientCharacterPhysicsComponentTrait for ClientCharacterPhysicsComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientCharacterPhysicsComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientCharacterPhysicsComponent {
}

impl super::entity::ComponentTrait for ClientCharacterPhysicsComponent {
}

impl super::entity::EntityBusPeerTrait for ClientCharacterPhysicsComponent {
}

pub static CLIENTCHARACTERPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCharacterPhysicsComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCHARACTERPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCharacterPhysicsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCHARACTERPHYSICSCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTCHARACTERPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientCharacterPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientCharacterMeshComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientCharacterMeshComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientCharacterMeshComponentTrait for ClientCharacterMeshComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientCharacterMeshComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientCharacterMeshComponent {
}

impl super::entity::ComponentTrait for ClientCharacterMeshComponent {
}

impl super::entity::EntityBusPeerTrait for ClientCharacterMeshComponent {
}

pub static CLIENTCHARACTERMESHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterMeshComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCharacterMeshComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCHARACTERMESHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCharacterMeshComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCHARACTERMESHCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTCHARACTERMESHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterMeshComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientCharacterMeshComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientCharacterMasterPhysicsComponent {
    pub _glacier_base: super::physics::CharacterPhysicsComponent,
}

pub trait ClientCharacterMasterPhysicsComponentTrait: super::physics::CharacterPhysicsComponentTrait {
}

impl ClientCharacterMasterPhysicsComponentTrait for ClientCharacterMasterPhysicsComponent {
}

impl super::physics::CharacterPhysicsComponentTrait for ClientCharacterMasterPhysicsComponent {
}

impl super::physics::PhysicsComponentTrait for ClientCharacterMasterPhysicsComponent {
}

impl super::entity::ComponentTrait for ClientCharacterMasterPhysicsComponent {
}

impl super::entity::EntityBusPeerTrait for ClientCharacterMasterPhysicsComponent {
}

pub static CLIENTCHARACTERMASTERPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterMasterPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::CHARACTERPHYSICSCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCharacterMasterPhysicsComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCHARACTERMASTERPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCharacterMasterPhysicsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCHARACTERMASTERPHYSICSCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTCHARACTERMASTERPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterMasterPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientCharacterMasterPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientCharacterHealthComponent {
    pub _glacier_base: super::gameplay_client_server::ClientControllableHealthComponent,
}

pub trait ClientCharacterHealthComponentTrait: super::gameplay_client_server::ClientControllableHealthComponentTrait {
}

impl ClientCharacterHealthComponentTrait for ClientCharacterHealthComponent {
}

impl super::gameplay_client_server::ClientControllableHealthComponentTrait for ClientCharacterHealthComponent {
}

impl super::gameplay_client_server::ClientGameHealthComponentTrait for ClientCharacterHealthComponent {
}

impl super::gameplay_sim::HealthComponentTrait for ClientCharacterHealthComponent {
}

impl super::entity::ComponentTrait for ClientCharacterHealthComponent {
}

impl super::entity::EntityBusPeerTrait for ClientCharacterHealthComponent {
}

pub static CLIENTCHARACTERHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterHealthComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTCONTROLLABLEHEALTHCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCharacterHealthComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCHARACTERHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCharacterHealthComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCHARACTERHEALTHCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTCHARACTERHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterHealthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientCharacterHealthComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientCharacterCustomizationComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientCharacterCustomizationComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientCharacterCustomizationComponentTrait for ClientCharacterCustomizationComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientCharacterCustomizationComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientCharacterCustomizationComponent {
}

impl super::entity::ComponentTrait for ClientCharacterCustomizationComponent {
}

impl super::entity::EntityBusPeerTrait for ClientCharacterCustomizationComponent {
}

pub static CLIENTCHARACTERCUSTOMIZATIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterCustomizationComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCharacterCustomizationComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCHARACTERCUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCharacterCustomizationComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCHARACTERCUSTOMIZATIONCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTCHARACTERCUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterCustomizationComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientCharacterCustomizationComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientCharacterCameraComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientCharacterCameraComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientCharacterCameraComponentTrait for ClientCharacterCameraComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientCharacterCameraComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientCharacterCameraComponent {
}

impl super::entity::ComponentTrait for ClientCharacterCameraComponent {
}

impl super::entity::EntityBusPeerTrait for ClientCharacterCameraComponent {
}

pub static CLIENTCHARACTERCAMERACOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterCameraComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCharacterCameraComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCHARACTERCAMERACOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCharacterCameraComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCHARACTERCAMERACOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTCHARACTERCAMERACOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterCameraComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientCharacterCameraComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CharacterTargetCameraCallback {
    pub _glacier_base: super::gameplay_sim::TargetCameraCallback,
}

pub trait CharacterTargetCameraCallbackTrait: super::gameplay_sim::TargetCameraCallbackTrait {
}

impl CharacterTargetCameraCallbackTrait for CharacterTargetCameraCallback {
}

impl super::gameplay_sim::TargetCameraCallbackTrait for CharacterTargetCameraCallback {
}

pub static CHARACTERTARGETCAMERACALLBACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterTargetCameraCallback",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_sim::TARGETCAMERACALLBACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CharacterTargetCameraCallback as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CHARACTERTARGETCAMERACALLBACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CharacterTargetCameraCallback {
    fn type_info(&self) -> &'static TypeInfo {
        CHARACTERTARGETCAMERACALLBACK_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CHARACTERTARGETCAMERACALLBACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterTargetCameraCallback-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("CharacterTargetCameraCallback"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientCameraRelativeInputComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientCameraRelativeInputComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientCameraRelativeInputComponentTrait for ClientCameraRelativeInputComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientCameraRelativeInputComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientCameraRelativeInputComponent {
}

impl super::entity::ComponentTrait for ClientCameraRelativeInputComponent {
}

impl super::entity::EntityBusPeerTrait for ClientCameraRelativeInputComponent {
}

pub static CLIENTCAMERARELATIVEINPUTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCameraRelativeInputComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCameraRelativeInputComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCAMERARELATIVEINPUTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCameraRelativeInputComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCAMERARELATIVEINPUTCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTCAMERARELATIVEINPUTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCameraRelativeInputComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientCameraRelativeInputComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientAntInputComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientAntInputComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientAntInputComponentTrait for ClientAntInputComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientAntInputComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientAntInputComponent {
}

impl super::entity::ComponentTrait for ClientAntInputComponent {
}

impl super::entity::EntityBusPeerTrait for ClientAntInputComponent {
}

pub static CLIENTANTINPUTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAntInputComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAntInputComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTANTINPUTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAntInputComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTANTINPUTCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTANTINPUTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAntInputComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientAntInputComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientPlayVFXEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientPlayVFXEntityTrait: super::entity::EntityTrait {
}

impl ClientPlayVFXEntityTrait for ClientPlayVFXEntity {
}

impl super::entity::EntityTrait for ClientPlayVFXEntity {
}

impl super::entity::EntityBusPeerTrait for ClientPlayVFXEntity {
}

pub static CLIENTPLAYVFXENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayVFXEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPlayVFXEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPLAYVFXENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPlayVFXEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPLAYVFXENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTPLAYVFXENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayVFXEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientPlayVFXEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientCharacterSpawnEntity {
    pub _glacier_base: super::gameplay_client_server::ClientSpawnEntity,
}

pub trait ClientCharacterSpawnEntityTrait: super::gameplay_client_server::ClientSpawnEntityTrait {
}

impl ClientCharacterSpawnEntityTrait for ClientCharacterSpawnEntity {
}

impl super::gameplay_client_server::ClientSpawnEntityTrait for ClientCharacterSpawnEntity {
}

impl super::entity::SpatialEntityTrait for ClientCharacterSpawnEntity {
}

impl super::entity::EntityTrait for ClientCharacterSpawnEntity {
}

impl super::entity::EntityBusPeerTrait for ClientCharacterSpawnEntity {
}

pub static CLIENTCHARACTERSPAWNENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterSpawnEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTSPAWNENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCharacterSpawnEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCHARACTERSPAWNENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCharacterSpawnEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCHARACTERSPAWNENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTCHARACTERSPAWNENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterSpawnEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientCharacterSpawnEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientTeamFilterEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientTeamFilterEntityTrait: super::entity::EntityTrait {
}

impl ClientTeamFilterEntityTrait for ClientTeamFilterEntity {
}

impl super::entity::EntityTrait for ClientTeamFilterEntity {
}

impl super::entity::EntityBusPeerTrait for ClientTeamFilterEntity {
}

pub static CLIENTTEAMFILTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTeamFilterEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientTeamFilterEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTTEAMFILTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientTeamFilterEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTTEAMFILTERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTTEAMFILTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTeamFilterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientTeamFilterEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientSyncedSequenceEntity {
    pub _glacier_base: super::entity::SequenceEntity,
}

pub trait ClientSyncedSequenceEntityTrait: super::entity::SequenceEntityTrait {
}

impl ClientSyncedSequenceEntityTrait for ClientSyncedSequenceEntity {
}

impl super::entity::SequenceEntityTrait for ClientSyncedSequenceEntity {
}

impl super::entity::EntityTrait for ClientSyncedSequenceEntity {
}

impl super::entity::EntityBusPeerTrait for ClientSyncedSequenceEntity {
}

pub static CLIENTSYNCEDSEQUENCEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSyncedSequenceEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SEQUENCEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSyncedSequenceEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSYNCEDSEQUENCEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSyncedSequenceEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSYNCEDSEQUENCEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTSYNCEDSEQUENCEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSyncedSequenceEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientSyncedSequenceEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientStreamingGateEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientStreamingGateEntityTrait: super::entity::EntityTrait {
}

impl ClientStreamingGateEntityTrait for ClientStreamingGateEntity {
}

impl super::entity::EntityTrait for ClientStreamingGateEntity {
}

impl super::entity::EntityBusPeerTrait for ClientStreamingGateEntity {
}

pub static CLIENTSTREAMINGGATEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStreamingGateEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientStreamingGateEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSTREAMINGGATEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientStreamingGateEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSTREAMINGGATEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTSTREAMINGGATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStreamingGateEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientStreamingGateEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientPlayerIteratorEntity {
    pub _glacier_base: super::game_common::PlayerIteratorEntity,
}

pub trait ClientPlayerIteratorEntityTrait: super::game_common::PlayerIteratorEntityTrait {
}

impl ClientPlayerIteratorEntityTrait for ClientPlayerIteratorEntity {
}

impl super::game_common::PlayerIteratorEntityTrait for ClientPlayerIteratorEntity {
}

impl super::entity::EntityTrait for ClientPlayerIteratorEntity {
}

impl super::entity::EntityBusPeerTrait for ClientPlayerIteratorEntity {
}

pub static CLIENTPLAYERITERATORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerIteratorEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_common::PLAYERITERATORENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPlayerIteratorEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPLAYERITERATORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPlayerIteratorEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPLAYERITERATORENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTPLAYERITERATORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerIteratorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientPlayerIteratorEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientLocalPlayerFilterEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientLocalPlayerFilterEntityTrait: super::entity::EntityTrait {
}

impl ClientLocalPlayerFilterEntityTrait for ClientLocalPlayerFilterEntity {
}

impl super::entity::EntityTrait for ClientLocalPlayerFilterEntity {
}

impl super::entity::EntityBusPeerTrait for ClientLocalPlayerFilterEntity {
}

pub static CLIENTLOCALPLAYERFILTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLocalPlayerFilterEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientLocalPlayerFilterEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTLOCALPLAYERFILTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientLocalPlayerFilterEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTLOCALPLAYERFILTERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTLOCALPLAYERFILTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLocalPlayerFilterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientLocalPlayerFilterEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientPlayerFilterEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientPlayerFilterEntityTrait: super::entity::EntityTrait {
}

impl ClientPlayerFilterEntityTrait for ClientPlayerFilterEntity {
}

impl super::entity::EntityTrait for ClientPlayerFilterEntity {
}

impl super::entity::EntityBusPeerTrait for ClientPlayerFilterEntity {
}

pub static CLIENTPLAYERFILTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerFilterEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPlayerFilterEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPLAYERFILTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPlayerFilterEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPLAYERFILTERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTPLAYERFILTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerFilterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientPlayerFilterEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientObjectiveEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientObjectiveEntityTrait: super::entity::EntityTrait {
}

impl ClientObjectiveEntityTrait for ClientObjectiveEntity {
}

impl super::entity::EntityTrait for ClientObjectiveEntity {
}

impl super::entity::EntityBusPeerTrait for ClientObjectiveEntity {
}

pub static CLIENTOBJECTIVEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientObjectiveEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientObjectiveEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTOBJECTIVEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientObjectiveEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTOBJECTIVEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTOBJECTIVEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientObjectiveEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientObjectiveEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientObjectAreaQueryEntity {
    pub _glacier_base: super::game_common::ObjectAreaQueryEntity,
}

pub trait ClientObjectAreaQueryEntityTrait: super::game_common::ObjectAreaQueryEntityTrait {
}

impl ClientObjectAreaQueryEntityTrait for ClientObjectAreaQueryEntity {
}

impl super::game_common::ObjectAreaQueryEntityTrait for ClientObjectAreaQueryEntity {
}

impl super::entity::EntityTrait for ClientObjectAreaQueryEntity {
}

impl super::entity::EntityBusPeerTrait for ClientObjectAreaQueryEntity {
}

pub static CLIENTOBJECTAREAQUERYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientObjectAreaQueryEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_common::OBJECTAREAQUERYENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientObjectAreaQueryEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTOBJECTAREAQUERYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientObjectAreaQueryEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTOBJECTAREAQUERYENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTOBJECTAREAQUERYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientObjectAreaQueryEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientObjectAreaQueryEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientMessageEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientMessageEntityTrait: super::entity::EntityTrait {
}

impl ClientMessageEntityTrait for ClientMessageEntity {
}

impl super::entity::EntityTrait for ClientMessageEntity {
}

impl super::entity::EntityBusPeerTrait for ClientMessageEntity {
}

pub static CLIENTMESSAGEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMessageEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientMessageEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTMESSAGEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientMessageEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTMESSAGEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTMESSAGEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMessageEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientMessageEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientMapMarkerEntity {
    pub _glacier_base: super::entity::SpatialEntity,
}

pub trait ClientMapMarkerEntityTrait: super::entity::SpatialEntityTrait {
}

impl ClientMapMarkerEntityTrait for ClientMapMarkerEntity {
}

impl super::entity::SpatialEntityTrait for ClientMapMarkerEntity {
}

impl super::entity::EntityTrait for ClientMapMarkerEntity {
}

impl super::entity::EntityBusPeerTrait for ClientMapMarkerEntity {
}

pub static CLIENTMAPMARKERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMapMarkerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientMapMarkerEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTMAPMARKERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientMapMarkerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTMAPMARKERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTMAPMARKERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMapMarkerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientMapMarkerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientLocalPlayerEventEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientLocalPlayerEventEntityTrait: super::entity::EntityTrait {
}

impl ClientLocalPlayerEventEntityTrait for ClientLocalPlayerEventEntity {
}

impl super::entity::EntityTrait for ClientLocalPlayerEventEntity {
}

impl super::entity::EntityBusPeerTrait for ClientLocalPlayerEventEntity {
}

pub static CLIENTLOCALPLAYEREVENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLocalPlayerEventEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientLocalPlayerEventEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTLOCALPLAYEREVENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientLocalPlayerEventEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTLOCALPLAYEREVENTENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTLOCALPLAYEREVENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLocalPlayerEventEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientLocalPlayerEventEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientEventMemoryEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientEventMemoryEntityTrait: super::entity::EntityTrait {
}

impl ClientEventMemoryEntityTrait for ClientEventMemoryEntity {
}

impl super::entity::EntityTrait for ClientEventMemoryEntity {
}

impl super::entity::EntityBusPeerTrait for ClientEventMemoryEntity {
}

pub static CLIENTEVENTMEMORYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEventMemoryEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientEventMemoryEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTEVENTMEMORYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientEventMemoryEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTEVENTMEMORYENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTEVENTMEMORYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEventMemoryEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientEventMemoryEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientFadeTrack {
    pub _glacier_base: super::timeline::TimelineTrack,
}

pub trait ClientFadeTrackTrait: super::timeline::TimelineTrackTrait {
}

impl ClientFadeTrackTrait for ClientFadeTrack {
}

impl super::timeline::TimelineTrackTrait for ClientFadeTrack {
}

pub static CLIENTFADETRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientFadeTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::TIMELINETRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientFadeTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTFADETRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientFadeTrack {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTFADETRACK_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTFADETRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientFadeTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientFadeTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientFadeEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientFadeEntityTrait: super::entity::EntityTrait {
}

impl ClientFadeEntityTrait for ClientFadeEntity {
}

impl super::entity::EntityTrait for ClientFadeEntity {
}

impl super::entity::EntityBusPeerTrait for ClientFadeEntity {
}

pub static CLIENTFADEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientFadeEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientFadeEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTFADEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientFadeEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTFADEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTFADEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientFadeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientFadeEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientDoublePlayerEvent {
    pub _glacier_base: super::gameplay_client_server::ClientPlayerEvent,
}

pub trait ClientDoublePlayerEventTrait: super::gameplay_client_server::ClientPlayerEventTrait {
}

impl ClientDoublePlayerEventTrait for ClientDoublePlayerEvent {
}

impl super::gameplay_client_server::ClientPlayerEventTrait for ClientDoublePlayerEvent {
}

impl super::gameplay_sim::PlayerEventBaseTrait for ClientDoublePlayerEvent {
}

impl super::entity::EntityEventTrait for ClientDoublePlayerEvent {
}

pub static CLIENTDOUBLEPLAYEREVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDoublePlayerEvent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTPLAYEREVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientDoublePlayerEvent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTDOUBLEPLAYEREVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientDoublePlayerEvent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTDOUBLEPLAYEREVENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTDOUBLEPLAYEREVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDoublePlayerEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientDoublePlayerEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientTeamEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientTeamEntityTrait: super::entity::EntityTrait {
}

impl ClientTeamEntityTrait for ClientTeamEntity {
}

impl super::entity::EntityTrait for ClientTeamEntity {
}

impl super::entity::EntityBusPeerTrait for ClientTeamEntity {
}

pub static CLIENTTEAMENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTeamEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientTeamEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTTEAMENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientTeamEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTTEAMENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTTEAMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTeamEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientTeamEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientLadderComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientLadderComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientLadderComponentTrait for ClientLadderComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientLadderComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientLadderComponent {
}

impl super::entity::ComponentTrait for ClientLadderComponent {
}

impl super::entity::EntityBusPeerTrait for ClientLadderComponent {
}

pub static CLIENTLADDERCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLadderComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientLadderComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTLADDERCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientLadderComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTLADDERCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTLADDERCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLadderComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientLadderComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientMultipleActorScenarioEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientMultipleActorScenarioEntityTrait: super::entity::EntityTrait {
}

impl ClientMultipleActorScenarioEntityTrait for ClientMultipleActorScenarioEntity {
}

impl super::entity::EntityTrait for ClientMultipleActorScenarioEntity {
}

impl super::entity::EntityBusPeerTrait for ClientMultipleActorScenarioEntity {
}

pub static CLIENTMULTIPLEACTORSCENARIOENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMultipleActorScenarioEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientMultipleActorScenarioEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTMULTIPLEACTORSCENARIOENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientMultipleActorScenarioEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTMULTIPLEACTORSCENARIOENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTMULTIPLEACTORSCENARIOENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMultipleActorScenarioEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientMultipleActorScenarioEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientModelAnimationEntity {
    pub _glacier_base: super::game_common::ModelAnimationEntity,
}

pub trait ClientModelAnimationEntityTrait: super::game_common::ModelAnimationEntityTrait {
}

impl ClientModelAnimationEntityTrait for ClientModelAnimationEntity {
}

impl super::game_common::ModelAnimationEntityTrait for ClientModelAnimationEntity {
}

impl super::entity::EntityTrait for ClientModelAnimationEntity {
}

impl super::entity::EntityBusPeerTrait for ClientModelAnimationEntity {
}

pub static CLIENTMODELANIMATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientModelAnimationEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_common::MODELANIMATIONENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientModelAnimationEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTMODELANIMATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientModelAnimationEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTMODELANIMATIONENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTMODELANIMATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientModelAnimationEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientModelAnimationEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientJointValidationEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientJointValidationEntityTrait: super::entity::EntityTrait {
}

impl ClientJointValidationEntityTrait for ClientJointValidationEntity {
}

impl super::entity::EntityTrait for ClientJointValidationEntity {
}

impl super::entity::EntityBusPeerTrait for ClientJointValidationEntity {
}

pub static CLIENTJOINTVALIDATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientJointValidationEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientJointValidationEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTJOINTVALIDATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientJointValidationEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTJOINTVALIDATIONENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTJOINTVALIDATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientJointValidationEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientJointValidationEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientFbProxyControllerEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientFbProxyControllerEntityTrait: super::entity::EntityTrait {
}

impl ClientFbProxyControllerEntityTrait for ClientFbProxyControllerEntity {
}

impl super::entity::EntityTrait for ClientFbProxyControllerEntity {
}

impl super::entity::EntityBusPeerTrait for ClientFbProxyControllerEntity {
}

pub static CLIENTFBPROXYCONTROLLERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientFbProxyControllerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientFbProxyControllerEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTFBPROXYCONTROLLERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientFbProxyControllerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTFBPROXYCONTROLLERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTFBPROXYCONTROLLERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientFbProxyControllerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientFbProxyControllerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientFaceposerEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientFaceposerEntityTrait: super::entity::EntityTrait {
}

impl ClientFaceposerEntityTrait for ClientFaceposerEntity {
}

impl super::entity::EntityTrait for ClientFaceposerEntity {
}

impl super::entity::EntityBusPeerTrait for ClientFaceposerEntity {
}

pub static CLIENTFACEPOSERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientFaceposerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientFaceposerEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTFACEPOSERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientFaceposerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTFACEPOSERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTFACEPOSERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientFaceposerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientFaceposerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientCharacterInVehicleScenarioEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientCharacterInVehicleScenarioEntityTrait: super::entity::EntityTrait {
}

impl ClientCharacterInVehicleScenarioEntityTrait for ClientCharacterInVehicleScenarioEntity {
}

impl super::entity::EntityTrait for ClientCharacterInVehicleScenarioEntity {
}

impl super::entity::EntityBusPeerTrait for ClientCharacterInVehicleScenarioEntity {
}

pub static CLIENTCHARACTERINVEHICLESCENARIOENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterInVehicleScenarioEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCharacterInVehicleScenarioEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCHARACTERINVEHICLESCENARIOENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCharacterInVehicleScenarioEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCHARACTERINVEHICLESCENARIOENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTCHARACTERINVEHICLESCENARIOENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterInVehicleScenarioEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientCharacterInVehicleScenarioEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientANTSignalTrack {
    pub _glacier_base: super::game_common::ANTSignalTrack,
}

pub trait ClientANTSignalTrackTrait: super::game_common::ANTSignalTrackTrait {
}

impl ClientANTSignalTrackTrait for ClientANTSignalTrack {
}

impl super::game_common::ANTSignalTrackTrait for ClientANTSignalTrack {
}

impl super::timeline::LinkTrackTrait for ClientANTSignalTrack {
}

impl super::timeline::TimelineTrackTrait for ClientANTSignalTrack {
}

pub static CLIENTANTSIGNALTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientANTSignalTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_common::ANTSIGNALTRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientANTSignalTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTANTSIGNALTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientANTSignalTrack {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTANTSIGNALTRACK_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTANTSIGNALTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientANTSignalTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientANTSignalTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientWriteAntGameStateEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientWriteAntGameStateEntityTrait: super::entity::EntityTrait {
}

impl ClientWriteAntGameStateEntityTrait for ClientWriteAntGameStateEntity {
}

impl super::entity::EntityTrait for ClientWriteAntGameStateEntity {
}

impl super::entity::EntityBusPeerTrait for ClientWriteAntGameStateEntity {
}

pub static CLIENTWRITEANTGAMESTATEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWriteAntGameStateEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientWriteAntGameStateEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWRITEANTGAMESTATEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWriteAntGameStateEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTWRITEANTGAMESTATEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTWRITEANTGAMESTATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWriteAntGameStateEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientWriteAntGameStateEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientReadAntGameStateEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientReadAntGameStateEntityTrait: super::entity::EntityTrait {
}

impl ClientReadAntGameStateEntityTrait for ClientReadAntGameStateEntity {
}

impl super::entity::EntityTrait for ClientReadAntGameStateEntity {
}

impl super::entity::EntityBusPeerTrait for ClientReadAntGameStateEntity {
}

pub static CLIENTREADANTGAMESTATEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientReadAntGameStateEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientReadAntGameStateEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTREADANTGAMESTATEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientReadAntGameStateEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTREADANTGAMESTATEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTREADANTGAMESTATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientReadAntGameStateEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientReadAntGameStateEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientAntEventEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientAntEventEntityTrait: super::entity::EntityTrait {
}

impl ClientAntEventEntityTrait for ClientAntEventEntity {
}

impl super::entity::EntityTrait for ClientAntEventEntity {
}

impl super::entity::EntityBusPeerTrait for ClientAntEventEntity {
}

pub static CLIENTANTEVENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAntEventEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAntEventEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTANTEVENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAntEventEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTANTEVENTENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTANTEVENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAntEventEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientAntEventEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientANTControlTrack {
    pub _glacier_base: super::game_common::ANTControlTrack,
}

pub trait ClientANTControlTrackTrait: super::game_common::ANTControlTrackTrait {
}

impl ClientANTControlTrackTrait for ClientANTControlTrack {
}

impl super::game_common::ANTControlTrackTrait for ClientANTControlTrack {
}

impl super::timeline::LinkTrackTrait for ClientANTControlTrack {
}

impl super::timeline::TimelineTrackTrait for ClientANTControlTrack {
}

pub static CLIENTANTCONTROLTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientANTControlTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_common::ANTCONTROLTRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientANTControlTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTANTCONTROLTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientANTControlTrack {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTANTCONTROLTRACK_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTANTCONTROLTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientANTControlTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientANTControlTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientAntAnimatableEntity {
    pub _glacier_base: super::gameplay_sim::AntAnimatableEntity,
}

pub trait ClientAntAnimatableEntityTrait: super::gameplay_sim::AntAnimatableEntityTrait {
}

impl ClientAntAnimatableEntityTrait for ClientAntAnimatableEntity {
}

impl super::gameplay_sim::AntAnimatableEntityTrait for ClientAntAnimatableEntity {
}

impl super::entity::EntityTrait for ClientAntAnimatableEntity {
}

impl super::entity::EntityBusPeerTrait for ClientAntAnimatableEntity {
}

pub static CLIENTANTANIMATABLEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAntAnimatableEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_sim::ANTANIMATABLEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAntAnimatableEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTANTANIMATABLEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAntAnimatableEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTANTANIMATABLEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTANTANIMATABLEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAntAnimatableEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientAntAnimatableEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientAnimationSignalEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientAnimationSignalEntityTrait: super::entity::EntityTrait {
}

impl ClientAnimationSignalEntityTrait for ClientAnimationSignalEntity {
}

impl super::entity::EntityTrait for ClientAnimationSignalEntity {
}

impl super::entity::EntityBusPeerTrait for ClientAnimationSignalEntity {
}

pub static CLIENTANIMATIONSIGNALENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAnimationSignalEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAnimationSignalEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTANIMATIONSIGNALENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAnimationSignalEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTANIMATIONSIGNALENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTANIMATIONSIGNALENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAnimationSignalEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientAnimationSignalEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientAnimationPoseTrack {
    pub _glacier_base: super::game_common::AnimationPoseTrack,
}

pub trait ClientAnimationPoseTrackTrait: super::game_common::AnimationPoseTrackTrait {
}

impl ClientAnimationPoseTrackTrait for ClientAnimationPoseTrack {
}

impl super::game_common::AnimationPoseTrackTrait for ClientAnimationPoseTrack {
}

impl super::timeline::LinkTrackTrait for ClientAnimationPoseTrack {
}

impl super::timeline::TimelineTrackTrait for ClientAnimationPoseTrack {
}

pub static CLIENTANIMATIONPOSETRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAnimationPoseTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_common::ANIMATIONPOSETRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAnimationPoseTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTANIMATIONPOSETRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAnimationPoseTrack {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTANIMATIONPOSETRACK_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTANIMATIONPOSETRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAnimationPoseTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientAnimationPoseTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientAnimationEnumerationChoiceEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientAnimationEnumerationChoiceEntityTrait: super::entity::EntityTrait {
}

impl ClientAnimationEnumerationChoiceEntityTrait for ClientAnimationEnumerationChoiceEntity {
}

impl super::entity::EntityTrait for ClientAnimationEnumerationChoiceEntity {
}

impl super::entity::EntityBusPeerTrait for ClientAnimationEnumerationChoiceEntity {
}

pub static CLIENTANIMATIONENUMERATIONCHOICEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAnimationEnumerationChoiceEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAnimationEnumerationChoiceEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTANIMATIONENUMERATIONCHOICEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAnimationEnumerationChoiceEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTANIMATIONENUMERATIONCHOICEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTANIMATIONENUMERATIONCHOICEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAnimationEnumerationChoiceEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientAnimationEnumerationChoiceEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientAnimationEnumerationEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientAnimationEnumerationEntityTrait: super::entity::EntityTrait {
}

impl ClientAnimationEnumerationEntityTrait for ClientAnimationEnumerationEntity {
}

impl super::entity::EntityTrait for ClientAnimationEnumerationEntity {
}

impl super::entity::EntityBusPeerTrait for ClientAnimationEnumerationEntity {
}

pub static CLIENTANIMATIONENUMERATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAnimationEnumerationEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAnimationEnumerationEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTANIMATIONENUMERATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAnimationEnumerationEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTANIMATIONENUMERATIONENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTANIMATIONENUMERATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAnimationEnumerationEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientAnimationEnumerationEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientCharacterPoseValidateTrack {
    pub _glacier_base: super::timeline::LinkTrack,
}

pub trait ClientCharacterPoseValidateTrackTrait: super::timeline::LinkTrackTrait {
}

impl ClientCharacterPoseValidateTrackTrait for ClientCharacterPoseValidateTrack {
}

impl super::timeline::LinkTrackTrait for ClientCharacterPoseValidateTrack {
}

impl super::timeline::TimelineTrackTrait for ClientCharacterPoseValidateTrack {
}

pub static CLIENTCHARACTERPOSEVALIDATETRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterPoseValidateTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::LINKTRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCharacterPoseValidateTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCHARACTERPOSEVALIDATETRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCharacterPoseValidateTrack {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCHARACTERPOSEVALIDATETRACK_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTCHARACTERPOSEVALIDATETRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterPoseValidateTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientCharacterPoseValidateTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientCharacterPoseRecordTrack {
    pub _glacier_base: super::timeline::LinkTrack,
}

pub trait ClientCharacterPoseRecordTrackTrait: super::timeline::LinkTrackTrait {
}

impl ClientCharacterPoseRecordTrackTrait for ClientCharacterPoseRecordTrack {
}

impl super::timeline::LinkTrackTrait for ClientCharacterPoseRecordTrack {
}

impl super::timeline::TimelineTrackTrait for ClientCharacterPoseRecordTrack {
}

pub static CLIENTCHARACTERPOSERECORDTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterPoseRecordTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::LINKTRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCharacterPoseRecordTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCHARACTERPOSERECORDTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCharacterPoseRecordTrack {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCHARACTERPOSERECORDTRACK_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTCHARACTERPOSERECORDTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterPoseRecordTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientCharacterPoseRecordTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MeshEmitterControlEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait MeshEmitterControlEntityTrait: super::entity::EntityTrait {
}

impl MeshEmitterControlEntityTrait for MeshEmitterControlEntity {
}

impl super::entity::EntityTrait for MeshEmitterControlEntity {
}

impl super::entity::EntityBusPeerTrait for MeshEmitterControlEntity {
}

pub static MESHEMITTERCONTROLENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshEmitterControlEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MeshEmitterControlEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(MESHEMITTERCONTROLENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MeshEmitterControlEntity {
    fn type_info(&self) -> &'static TypeInfo {
        MESHEMITTERCONTROLENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MESHEMITTERCONTROLENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshEmitterControlEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("MeshEmitterControlEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientRotorComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientRotorComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientRotorComponentTrait for ClientRotorComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientRotorComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientRotorComponent {
}

impl super::entity::ComponentTrait for ClientRotorComponent {
}

impl super::entity::EntityBusPeerTrait for ClientRotorComponent {
}

pub static CLIENTROTORCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientRotorComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientRotorComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTROTORCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientRotorComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTROTORCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTROTORCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientRotorComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientRotorComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientMeshComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientMeshComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientMeshComponentTrait for ClientMeshComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientMeshComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientMeshComponent {
}

impl super::entity::ComponentTrait for ClientMeshComponent {
}

impl super::entity::EntityBusPeerTrait for ClientMeshComponent {
}

pub static CLIENTMESHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMeshComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientMeshComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTMESHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientMeshComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTMESHCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTMESHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMeshComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientMeshComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientIKComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientIKComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientIKComponentTrait for ClientIKComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientIKComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientIKComponent {
}

impl super::entity::ComponentTrait for ClientIKComponent {
}

impl super::entity::EntityBusPeerTrait for ClientIKComponent {
}

pub static CLIENTIKCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientIKComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientIKComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTIKCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientIKComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTIKCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTIKCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientIKComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientIKComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientFakePhysicsMeshComponent {
    pub _glacier_base: ClientMeshComponent,
}

pub trait ClientFakePhysicsMeshComponentTrait: ClientMeshComponentTrait {
}

impl ClientFakePhysicsMeshComponentTrait for ClientFakePhysicsMeshComponent {
}

impl ClientMeshComponentTrait for ClientFakePhysicsMeshComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientFakePhysicsMeshComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientFakePhysicsMeshComponent {
}

impl super::entity::ComponentTrait for ClientFakePhysicsMeshComponent {
}

impl super::entity::EntityBusPeerTrait for ClientFakePhysicsMeshComponent {
}

pub static CLIENTFAKEPHYSICSMESHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientFakePhysicsMeshComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTMESHCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientFakePhysicsMeshComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTFAKEPHYSICSMESHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientFakePhysicsMeshComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTFAKEPHYSICSMESHCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTFAKEPHYSICSMESHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientFakePhysicsMeshComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientFakePhysicsMeshComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientEngineComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientEngineComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientEngineComponentTrait for ClientEngineComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientEngineComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientEngineComponent {
}

impl super::entity::ComponentTrait for ClientEngineComponent {
}

impl super::entity::EntityBusPeerTrait for ClientEngineComponent {
}

pub static CLIENTENGINECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEngineComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientEngineComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTENGINECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientEngineComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTENGINECOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTENGINECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEngineComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientEngineComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientChildComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientChildComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientChildComponentTrait for ClientChildComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientChildComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientChildComponent {
}

impl super::entity::ComponentTrait for ClientChildComponent {
}

impl super::entity::EntityBusPeerTrait for ClientChildComponent {
}

pub static CLIENTCHILDCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientChildComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientChildComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCHILDCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientChildComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCHILDCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTCHILDCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientChildComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientChildComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientChildBarrelComponent {
    pub _glacier_base: ClientChildComponent,
}

pub trait ClientChildBarrelComponentTrait: ClientChildComponentTrait {
}

impl ClientChildBarrelComponentTrait for ClientChildBarrelComponent {
}

impl ClientChildComponentTrait for ClientChildBarrelComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientChildBarrelComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientChildBarrelComponent {
}

impl super::entity::ComponentTrait for ClientChildBarrelComponent {
}

impl super::entity::EntityBusPeerTrait for ClientChildBarrelComponent {
}

pub static CLIENTCHILDBARRELCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientChildBarrelComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTCHILDCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientChildBarrelComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCHILDBARRELCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientChildBarrelComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCHILDBARRELCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTCHILDBARRELCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientChildBarrelComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientChildBarrelComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientChassisComponent {
    pub _glacier_base: super::gameplay_client_server::ClientPartComponent,
}

pub trait ClientChassisComponentTrait: super::gameplay_client_server::ClientPartComponentTrait {
}

impl ClientChassisComponentTrait for ClientChassisComponent {
}

impl super::gameplay_client_server::ClientPartComponentTrait for ClientChassisComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientChassisComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientChassisComponent {
}

impl super::entity::ComponentTrait for ClientChassisComponent {
}

impl super::entity::EntityBusPeerTrait for ClientChassisComponent {
}

pub static CLIENTCHASSISCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientChassisComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTPARTCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientChassisComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCHASSISCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientChassisComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCHASSISCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTCHASSISCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientChassisComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientChassisComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientCameraComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientCameraComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientCameraComponentTrait for ClientCameraComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientCameraComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientCameraComponent {
}

impl super::entity::ComponentTrait for ClientCameraComponent {
}

impl super::entity::EntityBusPeerTrait for ClientCameraComponent {
}

pub static CLIENTCAMERACOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCameraComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCameraComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCAMERACOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCameraComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCAMERACOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTCAMERACOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCameraComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientCameraComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientComponentTargetCameraCallback {
    pub _glacier_base: super::gameplay_sim::TargetCameraCallback,
}

pub trait ClientComponentTargetCameraCallbackTrait: super::gameplay_sim::TargetCameraCallbackTrait {
}

impl ClientComponentTargetCameraCallbackTrait for ClientComponentTargetCameraCallback {
}

impl super::gameplay_sim::TargetCameraCallbackTrait for ClientComponentTargetCameraCallback {
}

pub static CLIENTCOMPONENTTARGETCAMERACALLBACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientComponentTargetCameraCallback",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_sim::TARGETCAMERACALLBACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientComponentTargetCameraCallback as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCOMPONENTTARGETCAMERACALLBACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientComponentTargetCameraCallback {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCOMPONENTTARGETCAMERACALLBACK_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTCOMPONENTTARGETCAMERACALLBACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientComponentTargetCameraCallback-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientComponentTargetCameraCallback"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientGamePlayerExtent {
    pub _glacier_base: super::gameplay_client_server::ClientPlayerExtent,
}

pub trait ClientGamePlayerExtentTrait: super::gameplay_client_server::ClientPlayerExtentTrait {
}

impl ClientGamePlayerExtentTrait for ClientGamePlayerExtent {
}

impl super::gameplay_client_server::ClientPlayerExtentTrait for ClientGamePlayerExtent {
}

pub static CLIENTGAMEPLAYEREXTENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGamePlayerExtent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTPLAYEREXTENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientGamePlayerExtent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTGAMEPLAYEREXTENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientGamePlayerExtent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTGAMEPLAYEREXTENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTGAMEPLAYEREXTENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGamePlayerExtent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientGamePlayerExtent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientGamePlayerInternalExtent {
    pub _glacier_base: super::gameplay_client_server::ClientPlayerExtent,
}

pub trait ClientGamePlayerInternalExtentTrait: super::gameplay_client_server::ClientPlayerExtentTrait {
}

impl ClientGamePlayerInternalExtentTrait for ClientGamePlayerInternalExtent {
}

impl super::gameplay_client_server::ClientPlayerExtentTrait for ClientGamePlayerInternalExtent {
}

pub static CLIENTGAMEPLAYERINTERNALEXTENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGamePlayerInternalExtent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTPLAYEREXTENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientGamePlayerInternalExtent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTGAMEPLAYERINTERNALEXTENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientGamePlayerInternalExtent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTGAMEPLAYERINTERNALEXTENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTGAMEPLAYERINTERNALEXTENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGamePlayerInternalExtent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientGamePlayerInternalExtent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientExplosionEntity {
    pub _glacier_base: super::game_common::ExplosionEntity,
}

pub trait ClientExplosionEntityTrait: super::game_common::ExplosionEntityTrait {
}

impl ClientExplosionEntityTrait for ClientExplosionEntity {
}

impl super::game_common::ExplosionEntityTrait for ClientExplosionEntity {
}

impl super::entity::EntityTrait for ClientExplosionEntity {
}

impl super::entity::EntityBusPeerTrait for ClientExplosionEntity {
}

pub static CLIENTEXPLOSIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientExplosionEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_common::EXPLOSIONENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientExplosionEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTEXPLOSIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientExplosionEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTEXPLOSIONENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTEXPLOSIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientExplosionEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientExplosionEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientDynamicAvoidanceEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientDynamicAvoidanceEntityTrait: super::entity::EntityTrait {
}

impl ClientDynamicAvoidanceEntityTrait for ClientDynamicAvoidanceEntity {
}

impl super::entity::EntityTrait for ClientDynamicAvoidanceEntity {
}

impl super::entity::EntityBusPeerTrait for ClientDynamicAvoidanceEntity {
}

pub static CLIENTDYNAMICAVOIDANCEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDynamicAvoidanceEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientDynamicAvoidanceEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTDYNAMICAVOIDANCEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientDynamicAvoidanceEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTDYNAMICAVOIDANCEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTDYNAMICAVOIDANCEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDynamicAvoidanceEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientDynamicAvoidanceEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientVehicleEntity {
    pub _glacier_base: super::gameplay_client_server::ClientControllableEntity,
}

pub trait ClientVehicleEntityTrait: super::gameplay_client_server::ClientControllableEntityTrait {
}

impl ClientVehicleEntityTrait for ClientVehicleEntity {
}

impl super::gameplay_client_server::ClientControllableEntityTrait for ClientVehicleEntity {
}

impl super::gameplay_client_server::ClientPhysicsEntityTrait for ClientVehicleEntity {
}

impl super::gameplay_client_server::ClientGameComponentEntityTrait for ClientVehicleEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ClientVehicleEntity {
}

impl super::entity::ComponentEntityTrait for ClientVehicleEntity {
}

impl super::entity::SpatialEntityTrait for ClientVehicleEntity {
}

impl super::entity::EntityTrait for ClientVehicleEntity {
}

impl super::entity::EntityBusPeerTrait for ClientVehicleEntity {
}

pub static CLIENTVEHICLEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVehicleEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTCONTROLLABLEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientVehicleEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVEHICLEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientVehicleEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTVEHICLEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTVEHICLEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVehicleEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientVehicleEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientAnimatedDriverEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientAnimatedDriverEntityTrait: super::entity::EntityTrait {
}

impl ClientAnimatedDriverEntityTrait for ClientAnimatedDriverEntity {
}

impl super::entity::EntityTrait for ClientAnimatedDriverEntity {
}

impl super::entity::EntityBusPeerTrait for ClientAnimatedDriverEntity {
}

pub static CLIENTANIMATEDDRIVERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAnimatedDriverEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAnimatedDriverEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTANIMATEDDRIVERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAnimatedDriverEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTANIMATEDDRIVERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTANIMATEDDRIVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAnimatedDriverEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientAnimatedDriverEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientWheelComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientWheelComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientWheelComponentTrait for ClientWheelComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientWheelComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientWheelComponent {
}

impl super::entity::ComponentTrait for ClientWheelComponent {
}

impl super::entity::EntityBusPeerTrait for ClientWheelComponent {
}

pub static CLIENTWHEELCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWheelComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientWheelComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWHEELCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWheelComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTWHEELCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTWHEELCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWheelComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientWheelComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientWaterDepthComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientWaterDepthComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientWaterDepthComponentTrait for ClientWaterDepthComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientWaterDepthComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientWaterDepthComponent {
}

impl super::entity::ComponentTrait for ClientWaterDepthComponent {
}

impl super::entity::EntityBusPeerTrait for ClientWaterDepthComponent {
}

pub static CLIENTWATERDEPTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWaterDepthComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientWaterDepthComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWATERDEPTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWaterDepthComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTWATERDEPTHCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTWATERDEPTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWaterDepthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientWaterDepthComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientVehiclePhysicsComponent {
    pub _glacier_base: super::physics::PartPhysicsComponent,
}

pub trait ClientVehiclePhysicsComponentTrait: super::physics::PartPhysicsComponentTrait {
}

impl ClientVehiclePhysicsComponentTrait for ClientVehiclePhysicsComponent {
}

impl super::physics::PartPhysicsComponentTrait for ClientVehiclePhysicsComponent {
}

impl super::physics::PhysicsComponentTrait for ClientVehiclePhysicsComponent {
}

impl super::entity::ComponentTrait for ClientVehiclePhysicsComponent {
}

impl super::entity::EntityBusPeerTrait for ClientVehiclePhysicsComponent {
}

pub static CLIENTVEHICLEPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVehiclePhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::PARTPHYSICSCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientVehiclePhysicsComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVEHICLEPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientVehiclePhysicsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTVEHICLEPHYSICSCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTVEHICLEPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVehiclePhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientVehiclePhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientVehicleMeshComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientVehicleMeshComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientVehicleMeshComponentTrait for ClientVehicleMeshComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientVehicleMeshComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientVehicleMeshComponent {
}

impl super::entity::ComponentTrait for ClientVehicleMeshComponent {
}

impl super::entity::EntityBusPeerTrait for ClientVehicleMeshComponent {
}

pub static CLIENTVEHICLEMESHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVehicleMeshComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientVehicleMeshComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVEHICLEMESHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientVehicleMeshComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTVEHICLEMESHCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTVEHICLEMESHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVehicleMeshComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientVehicleMeshComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientVehicleHealthComponent {
    pub _glacier_base: super::gameplay_client_server::ClientControllableHealthComponent,
}

pub trait ClientVehicleHealthComponentTrait: super::gameplay_client_server::ClientControllableHealthComponentTrait {
}

impl ClientVehicleHealthComponentTrait for ClientVehicleHealthComponent {
}

impl super::gameplay_client_server::ClientControllableHealthComponentTrait for ClientVehicleHealthComponent {
}

impl super::gameplay_client_server::ClientGameHealthComponentTrait for ClientVehicleHealthComponent {
}

impl super::gameplay_sim::HealthComponentTrait for ClientVehicleHealthComponent {
}

impl super::entity::ComponentTrait for ClientVehicleHealthComponent {
}

impl super::entity::EntityBusPeerTrait for ClientVehicleHealthComponent {
}

pub static CLIENTVEHICLEHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVehicleHealthComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTCONTROLLABLEHEALTHCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientVehicleHealthComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVEHICLEHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientVehicleHealthComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTVEHICLEHEALTHCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTVEHICLEHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVehicleHealthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientVehicleHealthComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientVehicleEntryComponent {
    pub _glacier_base: ClientPlayerEntryComponent,
}

pub trait ClientVehicleEntryComponentTrait: ClientPlayerEntryComponentTrait {
}

impl ClientVehicleEntryComponentTrait for ClientVehicleEntryComponent {
}

impl ClientPlayerEntryComponentTrait for ClientVehicleEntryComponent {
}

impl ClientGameEntryComponentTrait for ClientVehicleEntryComponent {
}

impl super::gameplay_client_server::ClientEntryComponentTrait for ClientVehicleEntryComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientVehicleEntryComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientVehicleEntryComponent {
}

impl super::entity::ComponentTrait for ClientVehicleEntryComponent {
}

impl super::entity::EntityBusPeerTrait for ClientVehicleEntryComponent {
}

pub static CLIENTVEHICLEENTRYCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVehicleEntryComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTPLAYERENTRYCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientVehicleEntryComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVEHICLEENTRYCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientVehicleEntryComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTVEHICLEENTRYCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTVEHICLEENTRYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVehicleEntryComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientVehicleEntryComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientVehicleCustomizationComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientVehicleCustomizationComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientVehicleCustomizationComponentTrait for ClientVehicleCustomizationComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientVehicleCustomizationComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientVehicleCustomizationComponent {
}

impl super::entity::ComponentTrait for ClientVehicleCustomizationComponent {
}

impl super::entity::EntityBusPeerTrait for ClientVehicleCustomizationComponent {
}

pub static CLIENTVEHICLECUSTOMIZATIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVehicleCustomizationComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientVehicleCustomizationComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVEHICLECUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientVehicleCustomizationComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTVEHICLECUSTOMIZATIONCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTVEHICLECUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVehicleCustomizationComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientVehicleCustomizationComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientTrackWheelComponent {
    pub _glacier_base: ClientWheelComponent,
}

pub trait ClientTrackWheelComponentTrait: ClientWheelComponentTrait {
}

impl ClientTrackWheelComponentTrait for ClientTrackWheelComponent {
}

impl ClientWheelComponentTrait for ClientTrackWheelComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientTrackWheelComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientTrackWheelComponent {
}

impl super::entity::ComponentTrait for ClientTrackWheelComponent {
}

impl super::entity::EntityBusPeerTrait for ClientTrackWheelComponent {
}

pub static CLIENTTRACKWHEELCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTrackWheelComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTWHEELCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientTrackWheelComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTTRACKWHEELCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientTrackWheelComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTTRACKWHEELCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTTRACKWHEELCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTrackWheelComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientTrackWheelComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientTrackComponent {
    pub _glacier_base: ClientMeshComponent,
}

pub trait ClientTrackComponentTrait: ClientMeshComponentTrait {
}

impl ClientTrackComponentTrait for ClientTrackComponent {
}

impl ClientMeshComponentTrait for ClientTrackComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientTrackComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientTrackComponent {
}

impl super::entity::ComponentTrait for ClientTrackComponent {
}

impl super::entity::EntityBusPeerTrait for ClientTrackComponent {
}

pub static CLIENTTRACKCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTrackComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTMESHCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientTrackComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTTRACKCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientTrackComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTTRACKCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTTRACKCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTrackComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientTrackComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientStanceFilterComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientStanceFilterComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientStanceFilterComponentTrait for ClientStanceFilterComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientStanceFilterComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientStanceFilterComponent {
}

impl super::entity::ComponentTrait for ClientStanceFilterComponent {
}

impl super::entity::EntityBusPeerTrait for ClientStanceFilterComponent {
}

pub static CLIENTSTANCEFILTERCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStanceFilterComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientStanceFilterComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSTANCEFILTERCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientStanceFilterComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSTANCEFILTERCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTSTANCEFILTERCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStanceFilterComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientStanceFilterComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientEdgeModelComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientEdgeModelComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientEdgeModelComponentTrait for ClientEdgeModelComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientEdgeModelComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientEdgeModelComponent {
}

impl super::entity::ComponentTrait for ClientEdgeModelComponent {
}

impl super::entity::EntityBusPeerTrait for ClientEdgeModelComponent {
}

pub static CLIENTEDGEMODELCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEdgeModelComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientEdgeModelComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTEDGEMODELCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientEdgeModelComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTEDGEMODELCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTEDGEMODELCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEdgeModelComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientEdgeModelComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DestructionVolumeComponent {
    pub _glacier_base: super::entity::Component,
}

pub trait DestructionVolumeComponentTrait: super::entity::ComponentTrait {
}

impl DestructionVolumeComponentTrait for DestructionVolumeComponent {
}

impl super::entity::ComponentTrait for DestructionVolumeComponent {
}

impl super::entity::EntityBusPeerTrait for DestructionVolumeComponent {
}

pub static DESTRUCTIONVOLUMECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionVolumeComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::COMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DestructionVolumeComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DESTRUCTIONVOLUMECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DestructionVolumeComponent {
    fn type_info(&self) -> &'static TypeInfo {
        DESTRUCTIONVOLUMECOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DESTRUCTIONVOLUMECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionVolumeComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("DestructionVolumeComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientStaticModelNetworkDestructionComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientStaticModelNetworkDestructionComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientStaticModelNetworkDestructionComponentTrait for ClientStaticModelNetworkDestructionComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientStaticModelNetworkDestructionComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientStaticModelNetworkDestructionComponent {
}

impl super::entity::ComponentTrait for ClientStaticModelNetworkDestructionComponent {
}

impl super::entity::EntityBusPeerTrait for ClientStaticModelNetworkDestructionComponent {
}

pub static CLIENTSTATICMODELNETWORKDESTRUCTIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticModelNetworkDestructionComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientStaticModelNetworkDestructionComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSTATICMODELNETWORKDESTRUCTIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientStaticModelNetworkDestructionComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSTATICMODELNETWORKDESTRUCTIONCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTSTATICMODELNETWORKDESTRUCTIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticModelNetworkDestructionComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientStaticModelNetworkDestructionComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientGameSubView {
    pub _glacier_base: super::gameplay_client_server::ClientSubView,
}

pub trait ClientGameSubViewTrait: super::gameplay_client_server::ClientSubViewTrait {
}

impl ClientGameSubViewTrait for ClientGameSubView {
}

impl super::gameplay_client_server::ClientSubViewTrait for ClientGameSubView {
}

impl super::gameplay_sim::SubViewTrait for ClientGameSubView {
}

pub static CLIENTGAMESUBVIEW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameSubView",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTSUBVIEW_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientGameSubView as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTGAMESUBVIEW_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientGameSubView {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTGAMESUBVIEW_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTGAMESUBVIEW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameSubView-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientGameSubView"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientLocalPlayerProxyEntityTrack {
    pub _glacier_base: super::timeline::TemplatedProxyEntityTrack,
}

pub trait ClientLocalPlayerProxyEntityTrackTrait: super::timeline::TemplatedProxyEntityTrackTrait {
}

impl ClientLocalPlayerProxyEntityTrackTrait for ClientLocalPlayerProxyEntityTrack {
}

impl super::timeline::TemplatedProxyEntityTrackTrait for ClientLocalPlayerProxyEntityTrack {
}

impl super::timeline::ProxyEntityTrackTrait for ClientLocalPlayerProxyEntityTrack {
}

impl super::timeline::EntityTrackBaseTrait for ClientLocalPlayerProxyEntityTrack {
}

impl super::timeline::TimelineTrackTrait for ClientLocalPlayerProxyEntityTrack {
}

pub static CLIENTLOCALPLAYERPROXYENTITYTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLocalPlayerProxyEntityTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::TEMPLATEDPROXYENTITYTRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientLocalPlayerProxyEntityTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTLOCALPLAYERPROXYENTITYTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientLocalPlayerProxyEntityTrack {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTLOCALPLAYERPROXYENTITYTRACK_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTLOCALPLAYERPROXYENTITYTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLocalPlayerProxyEntityTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientLocalPlayerProxyEntityTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PA2LookAtTrack {
    pub _glacier_base: super::timeline::LinkTrack,
}

pub trait PA2LookAtTrackTrait: super::timeline::LinkTrackTrait {
}

impl PA2LookAtTrackTrait for PA2LookAtTrack {
}

impl super::timeline::LinkTrackTrait for PA2LookAtTrack {
}

impl super::timeline::TimelineTrackTrait for PA2LookAtTrack {
}

pub static PA2LOOKATTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2LookAtTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::LINKTRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PA2LookAtTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PA2LOOKATTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PA2LookAtTrack {
    fn type_info(&self) -> &'static TypeInfo {
        PA2LOOKATTRACK_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PA2LOOKATTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2LookAtTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("PA2LookAtTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LightTrack {
    pub _glacier_base: super::timeline::EntityTrackBase,
}

pub trait LightTrackTrait: super::timeline::EntityTrackBaseTrait {
}

impl LightTrackTrait for LightTrack {
}

impl super::timeline::EntityTrackBaseTrait for LightTrack {
}

impl super::timeline::TimelineTrackTrait for LightTrack {
}

pub static LIGHTTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::ENTITYTRACKBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LightTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LIGHTTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LightTrack {
    fn type_info(&self) -> &'static TypeInfo {
        LIGHTTRACK_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LIGHTTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("LightTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LightingTrack {
    pub _glacier_base: super::timeline::TimelineTrack,
}

pub trait LightingTrackTrait: super::timeline::TimelineTrackTrait {
}

impl LightingTrackTrait for LightingTrack {
}

impl super::timeline::TimelineTrackTrait for LightingTrack {
}

pub static LIGHTINGTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightingTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::TIMELINETRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LightingTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LIGHTINGTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LightingTrack {
    fn type_info(&self) -> &'static TypeInfo {
        LIGHTINGTRACK_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LIGHTINGTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightingTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("LightingTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SubLevelVisibleEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait SubLevelVisibleEntityTrait: super::entity::EntityTrait {
}

impl SubLevelVisibleEntityTrait for SubLevelVisibleEntity {
}

impl super::entity::EntityTrait for SubLevelVisibleEntity {
}

impl super::entity::EntityBusPeerTrait for SubLevelVisibleEntity {
}

pub static SUBLEVELVISIBLEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelVisibleEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SubLevelVisibleEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SUBLEVELVISIBLEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SubLevelVisibleEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SUBLEVELVISIBLEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SUBLEVELVISIBLEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelVisibleEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("SubLevelVisibleEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientSubLevelEntity {
    pub _glacier_base: super::entity::SubLevelEntity,
}

pub trait ClientSubLevelEntityTrait: super::entity::SubLevelEntityTrait {
}

impl ClientSubLevelEntityTrait for ClientSubLevelEntity {
}

impl super::entity::SubLevelEntityTrait for ClientSubLevelEntity {
}

impl super::entity::EntityTrait for ClientSubLevelEntity {
}

impl super::entity::EntityBusPeerTrait for ClientSubLevelEntity {
}

pub static CLIENTSUBLEVELENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSubLevelEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SUBLEVELENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSubLevelEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSUBLEVELENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSubLevelEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSUBLEVELENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTSUBLEVELENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSubLevelEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientSubLevelEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientSubLevelCollectionEntity {
    pub _glacier_base: super::game_common::SubLevelCollectionEntityBase,
}

pub trait ClientSubLevelCollectionEntityTrait: super::game_common::SubLevelCollectionEntityBaseTrait {
}

impl ClientSubLevelCollectionEntityTrait for ClientSubLevelCollectionEntity {
}

impl super::game_common::SubLevelCollectionEntityBaseTrait for ClientSubLevelCollectionEntity {
}

impl super::entity::EntityTrait for ClientSubLevelCollectionEntity {
}

impl super::entity::EntityBusPeerTrait for ClientSubLevelCollectionEntity {
}

pub static CLIENTSUBLEVELCOLLECTIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSubLevelCollectionEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_common::SUBLEVELCOLLECTIONENTITYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSubLevelCollectionEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSUBLEVELCOLLECTIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSubLevelCollectionEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSUBLEVELCOLLECTIONENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTSUBLEVELCOLLECTIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSubLevelCollectionEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientSubLevelCollectionEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientLocalPlayerGateEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientLocalPlayerGateEntityTrait: super::entity::EntityTrait {
}

impl ClientLocalPlayerGateEntityTrait for ClientLocalPlayerGateEntity {
}

impl super::entity::EntityTrait for ClientLocalPlayerGateEntity {
}

impl super::entity::EntityBusPeerTrait for ClientLocalPlayerGateEntity {
}

pub static CLIENTLOCALPLAYERGATEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLocalPlayerGateEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientLocalPlayerGateEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTLOCALPLAYERGATEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientLocalPlayerGateEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTLOCALPLAYERGATEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTLOCALPLAYERGATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLocalPlayerGateEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientLocalPlayerGateEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientAntDrivenComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientAntDrivenComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientAntDrivenComponentTrait for ClientAntDrivenComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientAntDrivenComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientAntDrivenComponent {
}

impl super::entity::ComponentTrait for ClientAntDrivenComponent {
}

impl super::entity::EntityBusPeerTrait for ClientAntDrivenComponent {
}

pub static CLIENTANTDRIVENCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAntDrivenComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAntDrivenComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTANTDRIVENCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAntDrivenComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTANTDRIVENCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTANTDRIVENCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAntDrivenComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientAntDrivenComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientWarpAnimationEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientWarpAnimationEntityTrait: super::entity::EntityTrait {
}

impl ClientWarpAnimationEntityTrait for ClientWarpAnimationEntity {
}

impl super::entity::EntityTrait for ClientWarpAnimationEntity {
}

impl super::entity::EntityBusPeerTrait for ClientWarpAnimationEntity {
}

pub static CLIENTWARPANIMATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWarpAnimationEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientWarpAnimationEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWARPANIMATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWarpAnimationEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTWARPANIMATIONENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTWARPANIMATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWarpAnimationEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientWarpAnimationEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientPhysicsDrivenAnimationEntity {
    pub _glacier_base: super::game_common::PhysicsDrivenAnimationEntity,
}

pub trait ClientPhysicsDrivenAnimationEntityTrait: super::game_common::PhysicsDrivenAnimationEntityTrait {
}

impl ClientPhysicsDrivenAnimationEntityTrait for ClientPhysicsDrivenAnimationEntity {
}

impl super::game_common::PhysicsDrivenAnimationEntityTrait for ClientPhysicsDrivenAnimationEntity {
}

impl super::entity::EntityTrait for ClientPhysicsDrivenAnimationEntity {
}

impl super::entity::EntityBusPeerTrait for ClientPhysicsDrivenAnimationEntity {
}

pub static CLIENTPHYSICSDRIVENANIMATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPhysicsDrivenAnimationEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_common::PHYSICSDRIVENANIMATIONENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPhysicsDrivenAnimationEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPHYSICSDRIVENANIMATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPhysicsDrivenAnimationEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPHYSICSDRIVENANIMATIONENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTPHYSICSDRIVENANIMATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPhysicsDrivenAnimationEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientPhysicsDrivenAnimationEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientCannedScenarioEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientCannedScenarioEntityTrait: super::entity::EntityTrait {
}

impl ClientCannedScenarioEntityTrait for ClientCannedScenarioEntity {
}

impl super::entity::EntityTrait for ClientCannedScenarioEntity {
}

impl super::entity::EntityBusPeerTrait for ClientCannedScenarioEntity {
}

pub static CLIENTCANNEDSCENARIOENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCannedScenarioEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCannedScenarioEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCANNEDSCENARIOENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCannedScenarioEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCANNEDSCENARIOENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTCANNEDSCENARIOENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCannedScenarioEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientCannedScenarioEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VehicleCamera {
    pub _glacier_base: super::gameplay_sim::TargetCamera,
}

pub trait VehicleCameraTrait: super::gameplay_sim::TargetCameraTrait {
}

impl VehicleCameraTrait for VehicleCamera {
}

impl super::gameplay_sim::TargetCameraTrait for VehicleCamera {
}

impl super::gameplay_sim::CameraTrait for VehicleCamera {
}

pub static VEHICLECAMERA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleCamera",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_sim::TARGETCAMERA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VehicleCamera as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(VEHICLECAMERA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for VehicleCamera {
    fn type_info(&self) -> &'static TypeInfo {
        VEHICLECAMERA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VEHICLECAMERA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleCamera-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("VehicleCamera"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct StaticCamera {
    pub _glacier_base: super::gameplay_sim::TargetCamera,
}

pub trait StaticCameraTrait: super::gameplay_sim::TargetCameraTrait {
}

impl StaticCameraTrait for StaticCamera {
}

impl super::gameplay_sim::TargetCameraTrait for StaticCamera {
}

impl super::gameplay_sim::CameraTrait for StaticCamera {
}

pub static STATICCAMERA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticCamera",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_sim::TARGETCAMERA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StaticCamera as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(STATICCAMERA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for StaticCamera {
    fn type_info(&self) -> &'static TypeInfo {
        STATICCAMERA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static STATICCAMERA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticCamera-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("StaticCamera"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicalCameraTrack {
    pub _glacier_base: CameraTrackWithSceneCamera,
}

pub trait PhysicalCameraTrackTrait: CameraTrackWithSceneCameraTrait {
}

impl PhysicalCameraTrackTrait for PhysicalCameraTrack {
}

impl CameraTrackWithSceneCameraTrait for PhysicalCameraTrack {
}

impl CameraTrackBaseTrait for PhysicalCameraTrack {
}

impl super::timeline::TimelineTrackTrait for PhysicalCameraTrack {
}

pub static PHYSICALCAMERATRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicalCameraTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CAMERATRACKWITHSCENECAMERA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicalCameraTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PHYSICALCAMERATRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PhysicalCameraTrack {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICALCAMERATRACK_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICALCAMERATRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicalCameraTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("PhysicalCameraTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FPSCamera {
    pub _glacier_base: super::gameplay_sim::TargetCamera,
}

pub trait FPSCameraTrait: super::gameplay_sim::TargetCameraTrait {
}

impl FPSCameraTrait for FPSCamera {
}

impl super::gameplay_sim::TargetCameraTrait for FPSCamera {
}

impl super::gameplay_sim::CameraTrait for FPSCamera {
}

pub static FPSCAMERA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FPSCamera",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_sim::TARGETCAMERA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FPSCamera as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(FPSCAMERA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FPSCamera {
    fn type_info(&self) -> &'static TypeInfo {
        FPSCAMERA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FPSCAMERA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FPSCamera-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("FPSCamera"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientPlayerCameraEntity {
    pub _glacier_base: ClientCameraEntityBase,
}

pub trait ClientPlayerCameraEntityTrait: ClientCameraEntityBaseTrait {
}

impl ClientPlayerCameraEntityTrait for ClientPlayerCameraEntity {
}

impl ClientCameraEntityBaseTrait for ClientPlayerCameraEntity {
}

impl super::entity::EntityTrait for ClientPlayerCameraEntity {
}

impl super::entity::EntityBusPeerTrait for ClientPlayerCameraEntity {
}

pub static CLIENTPLAYERCAMERAENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerCameraEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTCAMERAENTITYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPlayerCameraEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPLAYERCAMERAENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPlayerCameraEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPLAYERCAMERAENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTPLAYERCAMERAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerCameraEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientPlayerCameraEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientGameplayCameraTrack {
    pub _glacier_base: CameraTrackBase,
}

pub trait ClientGameplayCameraTrackTrait: CameraTrackBaseTrait {
}

impl ClientGameplayCameraTrackTrait for ClientGameplayCameraTrack {
}

impl CameraTrackBaseTrait for ClientGameplayCameraTrack {
}

impl super::timeline::TimelineTrackTrait for ClientGameplayCameraTrack {
}

pub static CLIENTGAMEPLAYCAMERATRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameplayCameraTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CAMERATRACKBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientGameplayCameraTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTGAMEPLAYCAMERATRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientGameplayCameraTrack {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTGAMEPLAYCAMERATRACK_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTGAMEPLAYCAMERATRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameplayCameraTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientGameplayCameraTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientCameraTrack {
    pub _glacier_base: PhysicalCameraTrack,
}

pub trait ClientCameraTrackTrait: PhysicalCameraTrackTrait {
}

impl ClientCameraTrackTrait for ClientCameraTrack {
}

impl PhysicalCameraTrackTrait for ClientCameraTrack {
}

impl CameraTrackWithSceneCameraTrait for ClientCameraTrack {
}

impl CameraTrackBaseTrait for ClientCameraTrack {
}

impl super::timeline::TimelineTrackTrait for ClientCameraTrack {
}

pub static CLIENTCAMERATRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCameraTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICALCAMERATRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCameraTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCAMERATRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCameraTrack {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCAMERATRACK_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTCAMERATRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCameraTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientCameraTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientTargetCameraEntity {
    pub _glacier_base: ClientCameraEntityBase,
}

pub trait ClientTargetCameraEntityTrait: ClientCameraEntityBaseTrait {
}

impl ClientTargetCameraEntityTrait for ClientTargetCameraEntity {
}

impl ClientCameraEntityBaseTrait for ClientTargetCameraEntity {
}

impl super::entity::EntityTrait for ClientTargetCameraEntity {
}

impl super::entity::EntityBusPeerTrait for ClientTargetCameraEntity {
}

pub static CLIENTTARGETCAMERAENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTargetCameraEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTCAMERAENTITYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientTargetCameraEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTTARGETCAMERAENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientTargetCameraEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTTARGETCAMERAENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTTARGETCAMERAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTargetCameraEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientTargetCameraEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientLookAtCameraEntity {
    pub _glacier_base: ClientCameraEntity,
}

pub trait ClientLookAtCameraEntityTrait: ClientCameraEntityTrait {
}

impl ClientLookAtCameraEntityTrait for ClientLookAtCameraEntity {
}

impl ClientCameraEntityTrait for ClientLookAtCameraEntity {
}

impl ClientCameraEntityBaseTrait for ClientLookAtCameraEntity {
}

impl super::entity::EntityTrait for ClientLookAtCameraEntity {
}

impl super::entity::EntityBusPeerTrait for ClientLookAtCameraEntity {
}

pub static CLIENTLOOKATCAMERAENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLookAtCameraEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTCAMERAENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientLookAtCameraEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTLOOKATCAMERAENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientLookAtCameraEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTLOOKATCAMERAENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTLOOKATCAMERAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLookAtCameraEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientLookAtCameraEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientCameraEntity {
    pub _glacier_base: ClientCameraEntityBase,
}

pub trait ClientCameraEntityTrait: ClientCameraEntityBaseTrait {
}

impl ClientCameraEntityTrait for ClientCameraEntity {
}

impl ClientCameraEntityBaseTrait for ClientCameraEntity {
}

impl super::entity::EntityTrait for ClientCameraEntity {
}

impl super::entity::EntityBusPeerTrait for ClientCameraEntity {
}

pub static CLIENTCAMERAENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCameraEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTCAMERAENTITYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCameraEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCAMERAENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCameraEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCAMERAENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTCAMERAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCameraEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientCameraEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientCameraEntityBase {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientCameraEntityBaseTrait: super::entity::EntityTrait {
}

impl ClientCameraEntityBaseTrait for ClientCameraEntityBase {
}

impl super::entity::EntityTrait for ClientCameraEntityBase {
}

impl super::entity::EntityBusPeerTrait for ClientCameraEntityBase {
}

pub static CLIENTCAMERAENTITYBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCameraEntityBase",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCameraEntityBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCAMERAENTITYBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCameraEntityBase {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCAMERAENTITYBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTCAMERAENTITYBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCameraEntityBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientCameraEntityBase"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientCameraDirectorTrack {
    pub _glacier_base: CameraDirectorTrackBase,
}

pub trait ClientCameraDirectorTrackTrait: CameraDirectorTrackBaseTrait {
}

impl ClientCameraDirectorTrackTrait for ClientCameraDirectorTrack {
}

impl CameraDirectorTrackBaseTrait for ClientCameraDirectorTrack {
}

impl super::timeline::TimelineTrackTrait for ClientCameraDirectorTrack {
}

pub static CLIENTCAMERADIRECTORTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCameraDirectorTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CAMERADIRECTORTRACKBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCameraDirectorTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCAMERADIRECTORTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCameraDirectorTrack {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCAMERADIRECTORTRACK_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTCAMERADIRECTORTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCameraDirectorTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientCameraDirectorTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientANTAnimatableCameraTrack {
    pub _glacier_base: PhysicalCameraTrack,
}

pub trait ClientANTAnimatableCameraTrackTrait: PhysicalCameraTrackTrait {
}

impl ClientANTAnimatableCameraTrackTrait for ClientANTAnimatableCameraTrack {
}

impl PhysicalCameraTrackTrait for ClientANTAnimatableCameraTrack {
}

impl CameraTrackWithSceneCameraTrait for ClientANTAnimatableCameraTrack {
}

impl CameraTrackBaseTrait for ClientANTAnimatableCameraTrack {
}

impl super::timeline::TimelineTrackTrait for ClientANTAnimatableCameraTrack {
}

pub static CLIENTANTANIMATABLECAMERATRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientANTAnimatableCameraTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICALCAMERATRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientANTAnimatableCameraTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTANTANIMATABLECAMERATRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientANTAnimatableCameraTrack {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTANTANIMATABLECAMERATRACK_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTANTANIMATABLECAMERATRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientANTAnimatableCameraTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientANTAnimatableCameraTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct IUISystem {
}

pub trait IUISystemTrait: TypeObject {
}

impl IUISystemTrait for IUISystem {
}

pub static IUISYSTEM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IUISystem",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IUISystem as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(IUISYSTEM_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IUISystem {
    fn type_info(&self) -> &'static TypeInfo {
        IUISYSTEM_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static IUISYSTEM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IUISystem-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("IUISystem"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientWaterEntity {
    pub _glacier_base: super::gameplay_client_server::ClientPhysicsEntity,
}

pub trait ClientWaterEntityTrait: super::gameplay_client_server::ClientPhysicsEntityTrait {
}

impl ClientWaterEntityTrait for ClientWaterEntity {
}

impl super::gameplay_client_server::ClientPhysicsEntityTrait for ClientWaterEntity {
}

impl super::gameplay_client_server::ClientGameComponentEntityTrait for ClientWaterEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ClientWaterEntity {
}

impl super::entity::ComponentEntityTrait for ClientWaterEntity {
}

impl super::entity::SpatialEntityTrait for ClientWaterEntity {
}

impl super::entity::EntityTrait for ClientWaterEntity {
}

impl super::entity::EntityBusPeerTrait for ClientWaterEntity {
}

pub static CLIENTWATERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWaterEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTPHYSICSENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientWaterEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWATERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWaterEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTWATERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTWATERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWaterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientWaterEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientTerrainEntity {
    pub _glacier_base: super::gameplay_client_server::ClientPhysicsEntity,
}

pub trait ClientTerrainEntityTrait: super::gameplay_client_server::ClientPhysicsEntityTrait {
}

impl ClientTerrainEntityTrait for ClientTerrainEntity {
}

impl super::gameplay_client_server::ClientPhysicsEntityTrait for ClientTerrainEntity {
}

impl super::gameplay_client_server::ClientGameComponentEntityTrait for ClientTerrainEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ClientTerrainEntity {
}

impl super::entity::ComponentEntityTrait for ClientTerrainEntity {
}

impl super::entity::SpatialEntityTrait for ClientTerrainEntity {
}

impl super::entity::EntityTrait for ClientTerrainEntity {
}

impl super::entity::EntityBusPeerTrait for ClientTerrainEntity {
}

pub static CLIENTTERRAINENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTerrainEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTPHYSICSENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientTerrainEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTTERRAINENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientTerrainEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTTERRAINENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTTERRAINENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTerrainEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientTerrainEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientStaticModelGroupEntity {
    pub _glacier_base: super::gameplay_client_server::ClientPhysicsEntity,
}

pub trait ClientStaticModelGroupEntityTrait: super::gameplay_client_server::ClientPhysicsEntityTrait {
}

impl ClientStaticModelGroupEntityTrait for ClientStaticModelGroupEntity {
}

impl super::gameplay_client_server::ClientPhysicsEntityTrait for ClientStaticModelGroupEntity {
}

impl super::gameplay_client_server::ClientGameComponentEntityTrait for ClientStaticModelGroupEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ClientStaticModelGroupEntity {
}

impl super::entity::ComponentEntityTrait for ClientStaticModelGroupEntity {
}

impl super::entity::SpatialEntityTrait for ClientStaticModelGroupEntity {
}

impl super::entity::EntityTrait for ClientStaticModelGroupEntity {
}

impl super::entity::EntityBusPeerTrait for ClientStaticModelGroupEntity {
}

pub static CLIENTSTATICMODELGROUPENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticModelGroupEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTPHYSICSENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientStaticModelGroupEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSTATICMODELGROUPENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientStaticModelGroupEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSTATICMODELGROUPENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTSTATICMODELGROUPENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticModelGroupEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientStaticModelGroupEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientStaticModelEntity {
    pub _glacier_base: super::gameplay_client_server::ClientPhysicsEntity,
}

pub trait ClientStaticModelEntityTrait: super::gameplay_client_server::ClientPhysicsEntityTrait {
}

impl ClientStaticModelEntityTrait for ClientStaticModelEntity {
}

impl super::gameplay_client_server::ClientPhysicsEntityTrait for ClientStaticModelEntity {
}

impl super::gameplay_client_server::ClientGameComponentEntityTrait for ClientStaticModelEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ClientStaticModelEntity {
}

impl super::entity::ComponentEntityTrait for ClientStaticModelEntity {
}

impl super::entity::SpatialEntityTrait for ClientStaticModelEntity {
}

impl super::entity::EntityTrait for ClientStaticModelEntity {
}

impl super::entity::EntityBusPeerTrait for ClientStaticModelEntity {
}

pub static CLIENTSTATICMODELENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticModelEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTPHYSICSENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientStaticModelEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSTATICMODELENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientStaticModelEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSTATICMODELENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTSTATICMODELENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticModelEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientStaticModelEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientLightDimmerEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientLightDimmerEntityTrait: super::entity::EntityTrait {
}

impl ClientLightDimmerEntityTrait for ClientLightDimmerEntity {
}

impl super::entity::EntityTrait for ClientLightDimmerEntity {
}

impl super::entity::EntityBusPeerTrait for ClientLightDimmerEntity {
}

pub static CLIENTLIGHTDIMMERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLightDimmerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientLightDimmerEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTLIGHTDIMMERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientLightDimmerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTLIGHTDIMMERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTLIGHTDIMMERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLightDimmerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientLightDimmerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientLadderEntity {
    pub _glacier_base: ClientStaticModelEntity,
}

pub trait ClientLadderEntityTrait: ClientStaticModelEntityTrait {
}

impl ClientLadderEntityTrait for ClientLadderEntity {
}

impl ClientStaticModelEntityTrait for ClientLadderEntity {
}

impl super::gameplay_client_server::ClientPhysicsEntityTrait for ClientLadderEntity {
}

impl super::gameplay_client_server::ClientGameComponentEntityTrait for ClientLadderEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ClientLadderEntity {
}

impl super::entity::ComponentEntityTrait for ClientLadderEntity {
}

impl super::entity::SpatialEntityTrait for ClientLadderEntity {
}

impl super::entity::EntityTrait for ClientLadderEntity {
}

impl super::entity::EntityBusPeerTrait for ClientLadderEntity {
}

pub static CLIENTLADDERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLadderEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTSTATICMODELENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientLadderEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTLADDERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientLadderEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTLADDERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTLADDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLadderEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientLadderEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientDynamicModelEntity {
    pub _glacier_base: super::gameplay_client_server::ClientPhysicsEntity,
}

pub trait ClientDynamicModelEntityTrait: super::gameplay_client_server::ClientPhysicsEntityTrait {
}

impl ClientDynamicModelEntityTrait for ClientDynamicModelEntity {
}

impl super::gameplay_client_server::ClientPhysicsEntityTrait for ClientDynamicModelEntity {
}

impl super::gameplay_client_server::ClientGameComponentEntityTrait for ClientDynamicModelEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ClientDynamicModelEntity {
}

impl super::entity::ComponentEntityTrait for ClientDynamicModelEntity {
}

impl super::entity::SpatialEntityTrait for ClientDynamicModelEntity {
}

impl super::entity::EntityTrait for ClientDynamicModelEntity {
}

impl super::entity::EntityBusPeerTrait for ClientDynamicModelEntity {
}

pub static CLIENTDYNAMICMODELENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDynamicModelEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTPHYSICSENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientDynamicModelEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTDYNAMICMODELENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientDynamicModelEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTDYNAMICMODELENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTDYNAMICMODELENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDynamicModelEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientDynamicModelEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientBangerEntity {
    pub _glacier_base: super::gameplay_client_server::ClientPhysicsEntity,
}

pub trait ClientBangerEntityTrait: super::gameplay_client_server::ClientPhysicsEntityTrait {
}

impl ClientBangerEntityTrait for ClientBangerEntity {
}

impl super::gameplay_client_server::ClientPhysicsEntityTrait for ClientBangerEntity {
}

impl super::gameplay_client_server::ClientGameComponentEntityTrait for ClientBangerEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ClientBangerEntity {
}

impl super::entity::ComponentEntityTrait for ClientBangerEntity {
}

impl super::entity::SpatialEntityTrait for ClientBangerEntity {
}

impl super::entity::EntityTrait for ClientBangerEntity {
}

impl super::entity::EntityBusPeerTrait for ClientBangerEntity {
}

pub static CLIENTBANGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBangerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTPHYSICSENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientBangerEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTBANGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientBangerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTBANGERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTBANGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBangerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("ClientBangerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LogicVisualEnvironmentEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait LogicVisualEnvironmentEntityTrait: super::entity::EntityTrait {
}

impl LogicVisualEnvironmentEntityTrait for LogicVisualEnvironmentEntity {
}

impl super::entity::EntityTrait for LogicVisualEnvironmentEntity {
}

impl super::entity::EntityBusPeerTrait for LogicVisualEnvironmentEntity {
}

pub static LOGICVISUALENVIRONMENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogicVisualEnvironmentEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClient",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LogicVisualEnvironmentEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LOGICVISUALENVIRONMENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LogicVisualEnvironmentEntity {
    fn type_info(&self) -> &'static TypeInfo {
        LOGICVISUALENVIRONMENTENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LOGICVISUALENVIRONMENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogicVisualEnvironmentEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClient",
    data: TypeInfoData::Array("LogicVisualEnvironmentEntity"),
    array_type: None,
    alignment: 8,
};


