use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_game_server_types(registry: &mut TypeRegistry) {
    registry.register_type(SERVERBANGERENTITY_TYPE_INFO);
    registry.register_type(SERVERBANGERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERWATERPHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERWATERPHYSICSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERWATERHEALTHCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERWATERHEALTHCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERTERRAINPHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERTERRAINPHYSICSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERTERRAINHEALTHCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERTERRAINHEALTHCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSTATICMODELPHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERSTATICMODELPHYSICSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSTATICMODELHEALTHCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERSTATICMODELHEALTHCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSTATICMODELGROUPPHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERSTATICMODELGROUPPHYSICSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSTATICMODELGROUPHEALTHCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERSTATICMODELGROUPHEALTHCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERPARTCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERPARTCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERPART_TYPE_INFO);
    registry.register_type(SERVERPART_ARRAY_TYPE_INFO);
    registry.register_type(SERVERGAMEHEALTHCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERGAMEHEALTHCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERBONECOMPONENT_TYPE_INFO);
    registry.register_type(SERVERBONECOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERPHYSICSENTITY_TYPE_INFO);
    registry.register_type(SERVERPHYSICSENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERLOCALPLAYERGATEENTITY_TYPE_INFO);
    registry.register_type(SERVERLOCALPLAYERGATEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERGROUPCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERGROUPCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERGHOSTENTITYOWNERWRAPPERENTITY_TYPE_INFO);
    registry.register_type(SERVERGHOSTENTITYOWNERWRAPPERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERGAMECOMPONENTENTITY_TYPE_INFO);
    registry.register_type(SERVERGAMECOMPONENTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVEREVENTSYNCENTITY_TYPE_INFO);
    registry.register_type(SERVEREVENTSYNCENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERPLACEHOLDERENTITY_TYPE_INFO);
    registry.register_type(SERVERPLACEHOLDERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERBLUEPRINTENTITY_TYPE_INFO);
    registry.register_type(SERVERBLUEPRINTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERRECORDROOTTRANSFORMTRACK_TYPE_INFO);
    registry.register_type(SERVERRECORDROOTTRANSFORMTRACK_ARRAY_TYPE_INFO);
    registry.register_type(SERVERRECORDENTRYINPUTTRACK_TYPE_INFO);
    registry.register_type(SERVERRECORDENTRYINPUTTRACK_ARRAY_TYPE_INFO);
    registry.register_type(SERVERPLAYANIMATIONENTITY_TYPE_INFO);
    registry.register_type(SERVERPLAYANIMATIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERMULTIPLEACTORSCENARIOENTITY_TYPE_INFO);
    registry.register_type(SERVERMULTIPLEACTORSCENARIOENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERMODELANIMATIONENTITY_TYPE_INFO);
    registry.register_type(SERVERMODELANIMATIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERFBPROXYCONTROLLERENTITY_TYPE_INFO);
    registry.register_type(SERVERFBPROXYCONTROLLERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCHARACTERINVEHICLESCENARIOENTITY_TYPE_INFO);
    registry.register_type(SERVERCHARACTERINVEHICLESCENARIOENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERANTSIGNALTRACK_TYPE_INFO);
    registry.register_type(SERVERANTSIGNALTRACK_ARRAY_TYPE_INFO);
    registry.register_type(SERVERWRITEANTGAMESTATEENTITY_TYPE_INFO);
    registry.register_type(SERVERWRITEANTGAMESTATEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERREADANTGAMESTATEENTITY_TYPE_INFO);
    registry.register_type(SERVERREADANTGAMESTATEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERANTCONTROLTRACK_TYPE_INFO);
    registry.register_type(SERVERANTCONTROLTRACK_ARRAY_TYPE_INFO);
    registry.register_type(SERVERANTANIMATABLEENTITY_TYPE_INFO);
    registry.register_type(SERVERANTANIMATABLEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERANIMATIONPOSETRACK_TYPE_INFO);
    registry.register_type(SERVERANIMATIONPOSETRACK_ARRAY_TYPE_INFO);
    registry.register_type(SERVERANIMATIONENUMERATIONCHOICEENTITY_TYPE_INFO);
    registry.register_type(SERVERANIMATIONENUMERATIONCHOICEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERANIMATIONENUMERATIONENTITY_TYPE_INFO);
    registry.register_type(SERVERANIMATIONENUMERATIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVEREDGEMODELCOMPONENT_TYPE_INFO);
    registry.register_type(SERVEREDGEMODELCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSTATICMODELNETWORKDESTRUCTIONCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERSTATICMODELNETWORKDESTRUCTIONCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERPLAYERINPUTTRIGGERENTITY_TYPE_INFO);
    registry.register_type(SERVERPLAYERINPUTTRIGGERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERMULTIPLETRIGGERENTITY_TYPE_INFO);
    registry.register_type(SERVERMULTIPLETRIGGERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERKILLALLENTITY_TYPE_INFO);
    registry.register_type(SERVERKILLALLENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERDELAYTRIGGERENTITY_TYPE_INFO);
    registry.register_type(SERVERDELAYTRIGGERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERDEATHAREATRIGGERENTITY_TYPE_INFO);
    registry.register_type(SERVERDEATHAREATRIGGERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERDAMAGEAREATRIGGERENTITY_TYPE_INFO);
    registry.register_type(SERVERDAMAGEAREATRIGGERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCOMBATAREATRIGGERENTITY_TYPE_INFO);
    registry.register_type(SERVERCOMBATAREATRIGGERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCOMBATACTIONTRIGGERENTITY_TYPE_INFO);
    registry.register_type(SERVERCOMBATACTIONTRIGGERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCLEARAREATRIGGERENTITY_TYPE_INFO);
    registry.register_type(SERVERCLEARAREATRIGGERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAREATRIGGERENTITY_TYPE_INFO);
    registry.register_type(SERVERAREATRIGGERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERVEHICLESPAWNENTITY_TYPE_INFO);
    registry.register_type(SERVERVEHICLESPAWNENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSPAWNENTITYCREATEDENTITYINFO_TYPE_INFO);
    registry.register_type(SERVERSPAWNENTITYCREATEDENTITYINFO_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSPAWNENTITY_TYPE_INFO);
    registry.register_type(SERVERSPAWNENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERORIGINATINGLOCATIONSPAWNCONTEXT_TYPE_INFO);
    registry.register_type(SERVERORIGINATINGLOCATIONSPAWNCONTEXT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERORIGINATINGBLUEPRINTSPAWNCONTEXT_TYPE_INFO);
    registry.register_type(SERVERORIGINATINGBLUEPRINTSPAWNCONTEXT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCHARACTERSPAWNENTITY_TYPE_INFO);
    registry.register_type(SERVERCHARACTERSPAWNENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERTELEPORTENTITY_TYPE_INFO);
    registry.register_type(SERVERTELEPORTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERTEAMFILTERENTITY_TYPE_INFO);
    registry.register_type(SERVERTEAMFILTERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERTEAMENTITY_TYPE_INFO);
    registry.register_type(SERVERTEAMENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERTACTICALOBJECTIVEENTITY_TYPE_INFO);
    registry.register_type(SERVERTACTICALOBJECTIVEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERLADDERCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERLADDERCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVEREXPLOSIONENTITY_TYPE_INFO);
    registry.register_type(SERVEREXPLOSIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERDYNAMICAVOIDANCEENTITY_TYPE_INFO);
    registry.register_type(SERVERDYNAMICAVOIDANCEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERVEHICLEENTITY_TYPE_INFO);
    registry.register_type(SERVERVEHICLEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERWHEELCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERWHEELCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERVEHICLEPHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERVEHICLEPHYSICSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERVEHICLEHEALTHCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERVEHICLEHEALTHCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERVEHICLEENTRYCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERVEHICLEENTRYCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERVEHICLECUSTOMIZATIONCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERVEHICLECUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERTRACKWHEELCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERTRACKWHEELCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERTRACKCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERTRACKCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSTANCEFILTERCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERSTANCEFILTERCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERROTORCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERROTORCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERMESHCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERMESHCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERENGINECOMPONENT_TYPE_INFO);
    registry.register_type(SERVERENGINECOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERENTRYCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERENTRYCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERDRIVERSTATICOBJECTCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERDRIVERSTATICOBJECTCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERDRIVERCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERDRIVERCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCONTROLLABLEHEALTHCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERCONTROLLABLEHEALTHCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCHARACTERENTRYCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERCHARACTERENTRYCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCHARACTERENTITY_TYPE_INFO);
    registry.register_type(SERVERCHARACTERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CHARACTERSERVERPLAYEREXTENT_TYPE_INFO);
    registry.register_type(CHARACTERSERVERPLAYEREXTENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERWARPANIMATIONCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERWARPANIMATIONCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERVEHICLEENTRYLISTENERCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERVEHICLEENTRYLISTENERCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCHARACTERPHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERCHARACTERPHYSICSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCHARACTERMASTERPHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERCHARACTERMASTERPHYSICSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCHARACTERHEALTHCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERCHARACTERHEALTHCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCHARACTERCUSTOMIZATIONCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERCHARACTERCUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCHARACTERCAMERACOMPONENT_TYPE_INFO);
    registry.register_type(SERVERCHARACTERCAMERACOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERANTINPUTCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERANTINPUTCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERANTDRIVENCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERANTDRIVENCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERWARPANIMATIONENTITY_TYPE_INFO);
    registry.register_type(SERVERWARPANIMATIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERPHYSICSDRIVENANIMATIONENTITY_TYPE_INFO);
    registry.register_type(SERVERPHYSICSDRIVENANIMATIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCANNEDSCENARIOENTITY_TYPE_INFO);
    registry.register_type(SERVERCANNEDSCENARIOENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERRECORDVEHICLETRACK_TYPE_INFO);
    registry.register_type(SERVERRECORDVEHICLETRACK_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSYNCEDSEQUENCEENTITY_TYPE_INFO);
    registry.register_type(SERVERSYNCEDSEQUENCEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSPEEDEVENTGATEENTITY_TYPE_INFO);
    registry.register_type(SERVERSPEEDEVENTGATEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSAVEGAMELOADEDENTITY_TYPE_INFO);
    registry.register_type(SERVERSAVEGAMELOADEDENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSAVEENTITY_TYPE_INFO);
    registry.register_type(SERVERSAVEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERPLAYERITERATORENTITY_TYPE_INFO);
    registry.register_type(SERVERPLAYERITERATORENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERPLAYERFILTERENTITY_TYPE_INFO);
    registry.register_type(SERVERPLAYERFILTERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVEROBJECTIVEENTITY_TYPE_INFO);
    registry.register_type(SERVEROBJECTIVEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVEROBJECTAREAQUERYENTITY_TYPE_INFO);
    registry.register_type(SERVEROBJECTAREAQUERYENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERMAPMARKERENTITY_TYPE_INFO);
    registry.register_type(SERVERMAPMARKERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERLEVELCONTROLENTITY_TYPE_INFO);
    registry.register_type(SERVERLEVELCONTROLENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERINPUTRESTRICTIONENTITY_TYPE_INFO);
    registry.register_type(SERVERINPUTRESTRICTIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERHUMANPLAYERENTITY_TYPE_INFO);
    registry.register_type(SERVERHUMANPLAYERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERHUMANPLAYERPROXYENTITY_TYPE_INFO);
    registry.register_type(SERVERHUMANPLAYERPROXYENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVEREVENTMEMORYENTITY_TYPE_INFO);
    registry.register_type(SERVEREVENTMEMORYENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVEREVENTIFSWITCHENTITY_TYPE_INFO);
    registry.register_type(SERVEREVENTIFSWITCHENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCUSTOMIZECHARACTERENTITY_TYPE_INFO);
    registry.register_type(SERVERCUSTOMIZECHARACTERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAREAQUERYENTITY_TYPE_INFO);
    registry.register_type(SERVERAREAQUERYENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERPLAYEREVENT_TYPE_INFO);
    registry.register_type(SERVERPLAYEREVENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERDOUBLEPLAYEREVENT_TYPE_INFO);
    registry.register_type(SERVERDOUBLEPLAYEREVENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERDAMAGEGIVEREVENT_TYPE_INFO);
    registry.register_type(SERVERDAMAGEGIVEREVENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERPREDESTRUCTIONENTITY_TYPE_INFO);
    registry.register_type(SERVERPREDESTRUCTIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERBANGERPHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERBANGERPHYSICSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERBANGERHEALTHMODULE_TYPE_INFO);
    registry.register_type(SERVERBANGERHEALTHMODULE_ARRAY_TYPE_INFO);
    registry.register_type(SERVERBANGERHEALTHCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERBANGERHEALTHCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSUBVIEW_TYPE_INFO);
    registry.register_type(SERVERSUBVIEW_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSPECTATORSUBVIEW_TYPE_INFO);
    registry.register_type(SERVERSPECTATORSUBVIEW_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSPECTATORSUBVIEWBASE_TYPE_INFO);
    registry.register_type(SERVERSPECTATORSUBVIEWBASE_ARRAY_TYPE_INFO);
    registry.register_type(SERVERGAMESUBVIEW_TYPE_INFO);
    registry.register_type(SERVERGAMESUBVIEW_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCONNECTION_TYPE_INFO);
    registry.register_type(SERVERCONNECTION_ARRAY_TYPE_INFO);
    registry.register_type(SERVERVEHICLESTATETRIGGERENTITY_TYPE_INFO);
    registry.register_type(SERVERVEHICLESTATETRIGGERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERUNDERFIRETRIGGERENTITY_TYPE_INFO);
    registry.register_type(SERVERUNDERFIRETRIGGERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCHARACTERLOOKATTRIGGERENTITY_TYPE_INFO);
    registry.register_type(SERVERCHARACTERLOOKATTRIGGERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERTRIGGERENTITY_TYPE_INFO);
    registry.register_type(SERVERTRIGGERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERPLAYERTAKEOVERTRIGGERENTITY_TYPE_INFO);
    registry.register_type(SERVERPLAYERTAKEOVERTRIGGERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCHILDCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERCHILDCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCHILDBARRELCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERCHILDBARRELCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCHASSISCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERCHASSISCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCAMERACOMPONENT_TYPE_INFO);
    registry.register_type(SERVERCAMERACOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERPLAYEREXTENT_TYPE_INFO);
    registry.register_type(SERVERPLAYEREXTENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERGAMEPLAYEREXTENT_TYPE_INFO);
    registry.register_type(SERVERGAMEPLAYEREXTENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERGAMEPLAYERINTERNALEXTENT_TYPE_INFO);
    registry.register_type(SERVERGAMEPLAYERINTERNALEXTENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERGAMESPLINEENTITY_TYPE_INFO);
    registry.register_type(SERVERGAMESPLINEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAREAIMMUNITYCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERAREAIMMUNITYCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERDYNAMICFIREENTITY_TYPE_INFO);
    registry.register_type(SERVERDYNAMICFIREENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCONTROLLABLEENTITY_TYPE_INFO);
    registry.register_type(SERVERCONTROLLABLEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERWARNINGSYSTEMCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERWARNINGSYSTEMCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERUNLOCKCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERUNLOCKCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERPLAYERENTRYCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERPLAYERENTRYCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERGAMEENTRYCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERGAMEENTRYCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSUBLEVELENTITY_TYPE_INFO);
    registry.register_type(SERVERSUBLEVELENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSUBLEVELCOLLECTIONENTITY_TYPE_INFO);
    registry.register_type(SERVERSUBLEVELCOLLECTIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSTARTPOINTENTITY_TYPE_INFO);
    registry.register_type(SERVERSTARTPOINTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(BANGERHEALTHMODULEDATA_TYPE_INFO);
    registry.register_type(BANGERHEALTHMODULEDATA_ARRAY_TYPE_INFO);
    registry.register_type(SERVERPROJECTILEONSPAWNMESSAGE_TYPE_INFO);
    registry.register_type(AIDIRECTORSTATEMESSAGE_TYPE_INFO);
    registry.register_type(AISPAWNBOTMESSAGE_TYPE_INFO);
    registry.register_type(AIPLAYERENABLEASTARGETMESSAGE_TYPE_INFO);
    registry.register_type(SERVERMISSIONOBJECTIVECOMPLETEDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERROUNDINTERRUPTEDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERROUNDOVERMESSAGE_TYPE_INFO);
    registry.register_type(SERVERROUNDRESETMESSAGE_TYPE_INFO);
    registry.register_type(SERVERGAMEPLAYCHECKPOINTACTIVATEDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERGAMEPLAYCHECKPOINTTRIGGEREDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERGAMEMODERESETMESSAGE_TYPE_INFO);
    registry.register_type(SERVERGAMEPLAYSETPOSTROUNDLOGICMESSAGE_TYPE_INFO);
    registry.register_type(SERVERGAMEPLAYSETPREROUNDLOGICMESSAGE_TYPE_INFO);
    registry.register_type(SERVERGAMEPLAYGAMEMODERESETMESSAGE_TYPE_INFO);
    registry.register_type(SERVERGAMEPLAYSERVERPLAYERMENUCANCELMESSAGE_TYPE_INFO);
    registry.register_type(SERVERGAMEPLAYSERVERPLAYERMENUOKMESSAGE_TYPE_INFO);
    registry.register_type(SERVERGAMEPLAYPREVIOUSWEATHERSTATEMESSAGE_TYPE_INFO);
    registry.register_type(SERVERGAMEPLAYFIGHTHARDERMESSAGE_TYPE_INFO);
    registry.register_type(SERVERGAMEPLAYDESERTERRETURNMESSAGE_TYPE_INFO);
    registry.register_type(SERVERGAMEPLAYDESERTERMESSAGE_TYPE_INFO);
    registry.register_type(SERVERGAMEPLAYPLAYERMENUCANCELMESSAGE_TYPE_INFO);
    registry.register_type(SERVERGAMEPLAYPLAYERMENUOKMESSAGE_TYPE_INFO);
    registry.register_type(SERVERGAMEPLAYVOICEOVERFINISHEDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERSTATICMODELDAMAGEDPARTBYPLAYERMESSAGE_TYPE_INFO);
    registry.register_type(SERVERSTATICMODELDESTROYEDPARTMESSAGE_TYPE_INFO);
    registry.register_type(SERVERSTATICMODELGROUPDESTROYEDPARTMESSAGE_TYPE_INFO);
    registry.register_type(SERVERSTATICMODELDESTROYEDALLCOLLAPSABLEPARTSMESSAGE_TYPE_INFO);
    registry.register_type(SERVERSTATICMODELSPAWNMESSAGE_TYPE_INFO);
    registry.register_type(SERVERCOLLISIONEXPLOSIONPACKDESTROYEDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERCOLLISIONEXPLOSIONPACKPLACEDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERCOLLISIONEXPLOSIONUNSPAWNMESSAGE_TYPE_INFO);
    registry.register_type(SERVERCOLLISIONEXPLOSIONDAMAGEMESSAGE_TYPE_INFO);
    registry.register_type(SERVERCOLLISIONEXPLOSIONSPAWNMESSAGE_TYPE_INFO);
    registry.register_type(SERVERCOLLISIONPROJECTILEIMPACTMESSAGE_TYPE_INFO);
    registry.register_type(SERVERCOLLISIONPROJECTILEFIREMESSAGE_TYPE_INFO);
    registry.register_type(SERVERCOLLISIONGRENADECOLLISIONMESSAGE_TYPE_INFO);
    registry.register_type(SERVERCOLLISIONGRENADETHROWMESSAGE_TYPE_INFO);
    registry.register_type(SERVERWEAPONCOMPONENTWEAPONONUNSPAWNMESSAGE_TYPE_INFO);
    registry.register_type(SERVERWEAPONCOMPONENTWEAPONONSPAWNMESSAGE_TYPE_INFO);
    registry.register_type(SERVERENTITYPICKUPONUNSPAWNMESSAGE_TYPE_INFO);
    registry.register_type(SERVERENTITYPICKUPONSPAWNMESSAGE_TYPE_INFO);
    registry.register_type(SERVERENTITYBANGERENTITYONUNSPAWNMESSAGE_TYPE_INFO);
    registry.register_type(SERVERENTITYBANGERENTITYONSPAWNMESSAGE_TYPE_INFO);
    registry.register_type(SERVERCLUBMEMBERDELETEDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERCLUBMEMBERCREATEDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERVEHICLELOCKABLEMESSAGE_TYPE_INFO);
    registry.register_type(SERVERVEHICLEEXITMESSAGE_TYPE_INFO);
    registry.register_type(SERVERVEHICLEENTERMESSAGE_TYPE_INFO);
    registry.register_type(SERVERVEHICLEDISABLEDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERVEHICLEDAMAGEMESSAGE_TYPE_INFO);
    registry.register_type(SERVERVEHICLEENTERRESTRICTIONMESSAGE_TYPE_INFO);
    registry.register_type(SERVERVEHICLEUNSPAWNMESSAGE_TYPE_INFO);
    registry.register_type(SERVERVEHICLESPAWNDONEMESSAGE_TYPE_INFO);
    registry.register_type(SERVERVEHICLEFORCEARMAMENTRETURNMESSAGE_TYPE_INFO);
    registry.register_type(SERVERVEHICLESETREMOTECONTROLLEDDAMAGEGIVERMESSAGE_TYPE_INFO);
    registry.register_type(SERVERVEHICLESWITCHTEAMMESSAGE_TYPE_INFO);
    registry.register_type(SERVERVEHICLEDESTROYEDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERPLAYERDISCONNECTMESSAGE_TYPE_INFO);
    registry.register_type(SERVERPLAYERSTARTEDFIREMESSAGE_TYPE_INFO);
    registry.register_type(SERVERPLAYERONPICKUPMESSAGE_TYPE_INFO);
    registry.register_type(PICKUPITEMTYPE_TYPE_INFO);
    registry.register_type(PICKUPITEMTYPE_ARRAY_TYPE_INFO);
    registry.register_type(SERVERPLAYERCHATMESSAGE_TYPE_INFO);
    registry.register_type(SERVERPLAYEREXITENTRYMESSAGE_TYPE_INFO);
    registry.register_type(SERVERPLAYERENTERENTRYMESSAGE_TYPE_INFO);
    registry.register_type(SERVERPLAYERABOUTTOCLEARCHARACTERMESSAGE_TYPE_INFO);
    registry.register_type(SERVERPLAYERINSTANTSUICIDEMESSAGE_TYPE_INFO);
    registry.register_type(SERVERPLAYERKILLEDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERPLAYERMANUALLYSELECTEDSPAWNPOINTMESSAGE_TYPE_INFO);
    registry.register_type(SERVERPLAYERCHANGECHATCHANNELMESSAGE_TYPE_INFO);
    registry.register_type(SERVERPLAYERSWITCHTEAMMESSAGE_TYPE_INFO);
    registry.register_type(SERVERPLAYERKITPICKEDUPMESSAGE_TYPE_INFO);
    registry.register_type(SERVERPLAYERKITREPLACEDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERPLAYERCHANGEDCHARACTERMESSAGE_TYPE_INFO);
    registry.register_type(SERVERPLAYERREVIVEREFUSEDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERPLAYERREVIVEACCEPTEDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERPLAYERREVIVEMESSAGE_TYPE_INFO);
    registry.register_type(SERVERPLAYERLEFTLEVELMESSAGE_TYPE_INFO);
    registry.register_type(SERVERPLAYERRELEASINGLEVELMESSAGE_TYPE_INFO);
    registry.register_type(SERVERPLAYERENTEREDLEVELMESSAGE_TYPE_INFO);
    registry.register_type(SERVERPLAYERLEVELLOADEDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERPLAYERDEBUGFRIENDZONESPAWNMESSAGE_TYPE_INFO);
    registry.register_type(SERVERPLAYERRESPAWNMESSAGE_TYPE_INFO);
    registry.register_type(SERVERPLAYERDESTROYMESSAGE_TYPE_INFO);
    registry.register_type(SERVERPLAYERCREATEDFORCONNECTIONMESSAGE_TYPE_INFO);
    registry.register_type(SERVERPLAYERCREATEMESSAGE_TYPE_INFO);
    registry.register_type(SERVERPLAYERABOUTTOCREATEFORCONNECTIONMESSAGE_TYPE_INFO);
    registry.register_type(SERVERCHARACTERCHARACTERDAMAGEMESSAGE_TYPE_INFO);
    registry.register_type(SERVERCHARACTERKILLEDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERMETRICSDETONATEEXPLOSIONMESSAGE_TYPE_INFO);
    registry.register_type(SERVERMETRICSOBJECTIVESUCCESSMESSAGE_TYPE_INFO);
    registry.register_type(SERVERMETRICSSAVEGAMESAVEDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERMETRICSSAVEGAMELOADEDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERWATERENTITY_TYPE_INFO);
    registry.register_type(SERVERWATERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERTERRAINENTITY_TYPE_INFO);
    registry.register_type(SERVERTERRAINENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERTERRAINDYNAMICDECALENTITY_TYPE_INFO);
    registry.register_type(SERVERTERRAINDYNAMICDECALENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSTATICMODELGROUPENTITY_TYPE_INFO);
    registry.register_type(SERVERSTATICMODELGROUPENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSTATICMODELENTITY_TYPE_INFO);
    registry.register_type(SERVERSTATICMODELENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERLADDERENTITY_TYPE_INFO);
    registry.register_type(SERVERLADDERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERINTERACTABLESTATICMODELENTITY_TYPE_INFO);
    registry.register_type(SERVERINTERACTABLESTATICMODELENTITY_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerBangerEntity {
}

pub const SERVERBANGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBangerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERPHYSICSENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERBANGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerBangerEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERBANGERENTITY_TYPE_INFO
    }
}


pub const SERVERBANGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBangerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerBangerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerWaterPhysicsComponent {
}

pub const SERVERWATERPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWaterPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERWATERPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerWaterPhysicsComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERWATERPHYSICSCOMPONENT_TYPE_INFO
    }
}


pub const SERVERWATERPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWaterPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerWaterPhysicsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerWaterHealthComponent {
}

pub const SERVERWATERHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWaterHealthComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMEHEALTHCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERWATERHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerWaterHealthComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERWATERHEALTHCOMPONENT_TYPE_INFO
    }
}


pub const SERVERWATERHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWaterHealthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerWaterHealthComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerTerrainPhysicsComponent {
}

pub const SERVERTERRAINPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTerrainPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERTERRAINPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerTerrainPhysicsComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERTERRAINPHYSICSCOMPONENT_TYPE_INFO
    }
}


pub const SERVERTERRAINPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTerrainPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerTerrainPhysicsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerTerrainHealthComponent {
}

pub const SERVERTERRAINHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTerrainHealthComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMEHEALTHCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERTERRAINHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerTerrainHealthComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERTERRAINHEALTHCOMPONENT_TYPE_INFO
    }
}


pub const SERVERTERRAINHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTerrainHealthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerTerrainHealthComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerStaticModelPhysicsComponent {
}

pub const SERVERSTATICMODELPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticModelPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PARTPHYSICSCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSTATICMODELPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerStaticModelPhysicsComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERSTATICMODELPHYSICSCOMPONENT_TYPE_INFO
    }
}


pub const SERVERSTATICMODELPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticModelPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerStaticModelPhysicsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerStaticModelHealthComponent {
}

pub const SERVERSTATICMODELHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticModelHealthComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMEHEALTHCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSTATICMODELHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerStaticModelHealthComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERSTATICMODELHEALTHCOMPONENT_TYPE_INFO
    }
}


pub const SERVERSTATICMODELHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticModelHealthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerStaticModelHealthComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerStaticModelGroupPhysicsComponent {
}

pub const SERVERSTATICMODELGROUPPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticModelGroupPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GROUPPHYSICSCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSTATICMODELGROUPPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerStaticModelGroupPhysicsComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERSTATICMODELGROUPPHYSICSCOMPONENT_TYPE_INFO
    }
}


pub const SERVERSTATICMODELGROUPPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticModelGroupPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerStaticModelGroupPhysicsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerStaticModelGroupHealthComponent {
}

pub const SERVERSTATICMODELGROUPHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticModelGroupHealthComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMEHEALTHCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSTATICMODELGROUPHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerStaticModelGroupHealthComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERSTATICMODELGROUPHEALTHCOMPONENT_TYPE_INFO
    }
}


pub const SERVERSTATICMODELGROUPHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticModelGroupHealthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerStaticModelGroupHealthComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPartComponent {
}

pub const SERVERPARTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPartComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERPARTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPartComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERPARTCOMPONENT_TYPE_INFO
    }
}


pub const SERVERPARTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPartComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerPartComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPart {
}

pub const SERVERPART_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPart",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PART_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERPART_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPart {
    fn type_info() -> &'static TypeInfo {
        SERVERPART_TYPE_INFO
    }
}


pub const SERVERPART_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPart-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerPart-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGameHealthComponent {
}

pub const SERVERGAMEHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameHealthComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(HEALTHCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERGAMEHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerGameHealthComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERGAMEHEALTHCOMPONENT_TYPE_INFO
    }
}


pub const SERVERGAMEHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameHealthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerGameHealthComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerBoneComponent {
}

pub const SERVERBONECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBoneComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERBONECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerBoneComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERBONECOMPONENT_TYPE_INFO
    }
}


pub const SERVERBONECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBoneComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerBoneComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPhysicsEntity {
}

pub const SERVERPHYSICSENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPhysicsEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENTENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERPHYSICSENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPhysicsEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERPHYSICSENTITY_TYPE_INFO
    }
}


pub const SERVERPHYSICSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPhysicsEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerPhysicsEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerLocalPlayerGateEntity {
}

pub const SERVERLOCALPLAYERGATEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLocalPlayerGateEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERLOCALPLAYERGATEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerLocalPlayerGateEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERLOCALPLAYERGATEENTITY_TYPE_INFO
    }
}


pub const SERVERLOCALPLAYERGATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLocalPlayerGateEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerLocalPlayerGateEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGroupComponent {
}

pub const SERVERGROUPCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGroupComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERGROUPCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerGroupComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERGROUPCOMPONENT_TYPE_INFO
    }
}


pub const SERVERGROUPCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGroupComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerGroupComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGhostEntityOwnerWrapperEntity {
}

pub const SERVERGHOSTENTITYOWNERWRAPPERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGhostEntityOwnerWrapperEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERGHOSTENTITYOWNERWRAPPERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerGhostEntityOwnerWrapperEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERGHOSTENTITYOWNERWRAPPERENTITY_TYPE_INFO
    }
}


pub const SERVERGHOSTENTITYOWNERWRAPPERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGhostEntityOwnerWrapperEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerGhostEntityOwnerWrapperEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGameComponentEntity {
}

pub const SERVERGAMECOMPONENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameComponentEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMECOMPONENTENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERGAMECOMPONENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerGameComponentEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERGAMECOMPONENTENTITY_TYPE_INFO
    }
}


pub const SERVERGAMECOMPONENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameComponentEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerGameComponentEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerEventSyncEntity {
}

pub const SERVEREVENTSYNCENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEventSyncEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVEREVENTSYNCENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerEventSyncEntity {
    fn type_info() -> &'static TypeInfo {
        SERVEREVENTSYNCENTITY_TYPE_INFO
    }
}


pub const SERVEREVENTSYNCENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEventSyncEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerEventSyncEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPlaceHolderEntity {
}

pub const SERVERPLACEHOLDERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlaceHolderEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERPLACEHOLDERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPlaceHolderEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERPLACEHOLDERENTITY_TYPE_INFO
    }
}


pub const SERVERPLACEHOLDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlaceHolderEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerPlaceHolderEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerBlueprintEntity {
}

pub const SERVERBLUEPRINTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBlueprintEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERBLUEPRINTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerBlueprintEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERBLUEPRINTENTITY_TYPE_INFO
    }
}


pub const SERVERBLUEPRINTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBlueprintEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerBlueprintEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerRecordRootTransformTrack {
}

pub const SERVERRECORDROOTTRANSFORMTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerRecordRootTransformTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RECORDTRACKBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERRECORDROOTTRANSFORMTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerRecordRootTransformTrack {
    fn type_info() -> &'static TypeInfo {
        SERVERRECORDROOTTRANSFORMTRACK_TYPE_INFO
    }
}


pub const SERVERRECORDROOTTRANSFORMTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerRecordRootTransformTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerRecordRootTransformTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerRecordEntryInputTrack {
}

pub const SERVERRECORDENTRYINPUTTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerRecordEntryInputTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RECORDTRACKBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERRECORDENTRYINPUTTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerRecordEntryInputTrack {
    fn type_info() -> &'static TypeInfo {
        SERVERRECORDENTRYINPUTTRACK_TYPE_INFO
    }
}


pub const SERVERRECORDENTRYINPUTTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerRecordEntryInputTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerRecordEntryInputTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPlayAnimationEntity {
}

pub const SERVERPLAYANIMATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayAnimationEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERPLAYANIMATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPlayAnimationEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERPLAYANIMATIONENTITY_TYPE_INFO
    }
}


pub const SERVERPLAYANIMATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayAnimationEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerPlayAnimationEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerMultipleActorScenarioEntity {
}

pub const SERVERMULTIPLEACTORSCENARIOENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMultipleActorScenarioEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERMULTIPLEACTORSCENARIOENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerMultipleActorScenarioEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERMULTIPLEACTORSCENARIOENTITY_TYPE_INFO
    }
}


pub const SERVERMULTIPLEACTORSCENARIOENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMultipleActorScenarioEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerMultipleActorScenarioEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerModelAnimationEntity {
}

pub const SERVERMODELANIMATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerModelAnimationEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MODELANIMATIONENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERMODELANIMATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerModelAnimationEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERMODELANIMATIONENTITY_TYPE_INFO
    }
}


pub const SERVERMODELANIMATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerModelAnimationEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerModelAnimationEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerFbProxyControllerEntity {
}

pub const SERVERFBPROXYCONTROLLERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerFbProxyControllerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERFBPROXYCONTROLLERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerFbProxyControllerEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERFBPROXYCONTROLLERENTITY_TYPE_INFO
    }
}


pub const SERVERFBPROXYCONTROLLERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerFbProxyControllerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerFbProxyControllerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCharacterInVehicleScenarioEntity {
}

pub const SERVERCHARACTERINVEHICLESCENARIOENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterInVehicleScenarioEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERCHARACTERINVEHICLESCENARIOENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCharacterInVehicleScenarioEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERCHARACTERINVEHICLESCENARIOENTITY_TYPE_INFO
    }
}


pub const SERVERCHARACTERINVEHICLESCENARIOENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterInVehicleScenarioEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerCharacterInVehicleScenarioEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerANTSignalTrack {
}

pub const SERVERANTSIGNALTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerANTSignalTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ANTSIGNALTRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERANTSIGNALTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerANTSignalTrack {
    fn type_info() -> &'static TypeInfo {
        SERVERANTSIGNALTRACK_TYPE_INFO
    }
}


pub const SERVERANTSIGNALTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerANTSignalTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerANTSignalTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerWriteAntGameStateEntity {
}

pub const SERVERWRITEANTGAMESTATEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWriteAntGameStateEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERWRITEANTGAMESTATEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerWriteAntGameStateEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERWRITEANTGAMESTATEENTITY_TYPE_INFO
    }
}


pub const SERVERWRITEANTGAMESTATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWriteAntGameStateEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerWriteAntGameStateEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerReadAntGameStateEntity {
}

pub const SERVERREADANTGAMESTATEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerReadAntGameStateEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERREADANTGAMESTATEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerReadAntGameStateEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERREADANTGAMESTATEENTITY_TYPE_INFO
    }
}


pub const SERVERREADANTGAMESTATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerReadAntGameStateEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerReadAntGameStateEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerANTControlTrack {
}

pub const SERVERANTCONTROLTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerANTControlTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ANTCONTROLTRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERANTCONTROLTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerANTControlTrack {
    fn type_info() -> &'static TypeInfo {
        SERVERANTCONTROLTRACK_TYPE_INFO
    }
}


pub const SERVERANTCONTROLTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerANTControlTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerANTControlTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAntAnimatableEntity {
}

pub const SERVERANTANIMATABLEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAntAnimatableEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ANTANIMATABLEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERANTANIMATABLEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAntAnimatableEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERANTANIMATABLEENTITY_TYPE_INFO
    }
}


pub const SERVERANTANIMATABLEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAntAnimatableEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerAntAnimatableEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAnimationPoseTrack {
}

pub const SERVERANIMATIONPOSETRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAnimationPoseTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ANIMATIONPOSETRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERANIMATIONPOSETRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAnimationPoseTrack {
    fn type_info() -> &'static TypeInfo {
        SERVERANIMATIONPOSETRACK_TYPE_INFO
    }
}


pub const SERVERANIMATIONPOSETRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAnimationPoseTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerAnimationPoseTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAnimationEnumerationChoiceEntity {
}

pub const SERVERANIMATIONENUMERATIONCHOICEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAnimationEnumerationChoiceEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERANIMATIONENUMERATIONCHOICEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAnimationEnumerationChoiceEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERANIMATIONENUMERATIONCHOICEENTITY_TYPE_INFO
    }
}


pub const SERVERANIMATIONENUMERATIONCHOICEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAnimationEnumerationChoiceEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerAnimationEnumerationChoiceEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAnimationEnumerationEntity {
}

pub const SERVERANIMATIONENUMERATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAnimationEnumerationEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERANIMATIONENUMERATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAnimationEnumerationEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERANIMATIONENUMERATIONENTITY_TYPE_INFO
    }
}


pub const SERVERANIMATIONENUMERATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAnimationEnumerationEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerAnimationEnumerationEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerEdgeModelComponent {
}

pub const SERVEREDGEMODELCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEdgeModelComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVEREDGEMODELCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerEdgeModelComponent {
    fn type_info() -> &'static TypeInfo {
        SERVEREDGEMODELCOMPONENT_TYPE_INFO
    }
}


pub const SERVEREDGEMODELCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEdgeModelComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerEdgeModelComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerStaticModelNetworkDestructionComponent {
}

pub const SERVERSTATICMODELNETWORKDESTRUCTIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticModelNetworkDestructionComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSTATICMODELNETWORKDESTRUCTIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerStaticModelNetworkDestructionComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERSTATICMODELNETWORKDESTRUCTIONCOMPONENT_TYPE_INFO
    }
}


pub const SERVERSTATICMODELNETWORKDESTRUCTIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticModelNetworkDestructionComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerStaticModelNetworkDestructionComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPlayerInputTriggerEntity {
}

pub const SERVERPLAYERINPUTTRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerInputTriggerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERPLAYERINPUTTRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPlayerInputTriggerEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERPLAYERINPUTTRIGGERENTITY_TYPE_INFO
    }
}


pub const SERVERPLAYERINPUTTRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerInputTriggerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerPlayerInputTriggerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerMultipleTriggerEntity {
}

pub const SERVERMULTIPLETRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMultipleTriggerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERTRIGGERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERMULTIPLETRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerMultipleTriggerEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERMULTIPLETRIGGERENTITY_TYPE_INFO
    }
}


pub const SERVERMULTIPLETRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMultipleTriggerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerMultipleTriggerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerKillAllEntity {
}

pub const SERVERKILLALLENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerKillAllEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERKILLALLENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerKillAllEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERKILLALLENTITY_TYPE_INFO
    }
}


pub const SERVERKILLALLENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerKillAllEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerKillAllEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerDelayTriggerEntity {
}

pub const SERVERDELAYTRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDelayTriggerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERTRIGGERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERDELAYTRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerDelayTriggerEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERDELAYTRIGGERENTITY_TYPE_INFO
    }
}


pub const SERVERDELAYTRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDelayTriggerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerDelayTriggerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerDeathAreaTriggerEntity {
}

pub const SERVERDEATHAREATRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDeathAreaTriggerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERTRIGGERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERDEATHAREATRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerDeathAreaTriggerEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERDEATHAREATRIGGERENTITY_TYPE_INFO
    }
}


pub const SERVERDEATHAREATRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDeathAreaTriggerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerDeathAreaTriggerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerDamageAreaTriggerEntity {
}

pub const SERVERDAMAGEAREATRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDamageAreaTriggerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERTRIGGERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERDAMAGEAREATRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerDamageAreaTriggerEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERDAMAGEAREATRIGGERENTITY_TYPE_INFO
    }
}


pub const SERVERDAMAGEAREATRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDamageAreaTriggerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerDamageAreaTriggerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCombatAreaTriggerEntity {
}

pub const SERVERCOMBATAREATRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCombatAreaTriggerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMBATAREAENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERCOMBATAREATRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCombatAreaTriggerEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERCOMBATAREATRIGGERENTITY_TYPE_INFO
    }
}


pub const SERVERCOMBATAREATRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCombatAreaTriggerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerCombatAreaTriggerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCombatActionTriggerEntity {
}

pub const SERVERCOMBATACTIONTRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCombatActionTriggerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERTRIGGERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERCOMBATACTIONTRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCombatActionTriggerEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERCOMBATACTIONTRIGGERENTITY_TYPE_INFO
    }
}


pub const SERVERCOMBATACTIONTRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCombatActionTriggerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerCombatActionTriggerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerClearAreaTriggerEntity {
}

pub const SERVERCLEARAREATRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerClearAreaTriggerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERCLEARAREATRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerClearAreaTriggerEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERCLEARAREATRIGGERENTITY_TYPE_INFO
    }
}


pub const SERVERCLEARAREATRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerClearAreaTriggerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerClearAreaTriggerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAreaTriggerEntity {
}

pub const SERVERAREATRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAreaTriggerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERTRIGGERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAREATRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAreaTriggerEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAREATRIGGERENTITY_TYPE_INFO
    }
}


pub const SERVERAREATRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAreaTriggerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerAreaTriggerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerVehicleSpawnEntity {
}

pub const SERVERVEHICLESPAWNENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleSpawnEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERSPAWNENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERVEHICLESPAWNENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerVehicleSpawnEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERVEHICLESPAWNENTITY_TYPE_INFO
    }
}


pub const SERVERVEHICLESPAWNENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleSpawnEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerVehicleSpawnEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSpawnEntityCreatedEntityInfo {
}

pub const SERVERSPAWNENTITYCREATEDENTITYINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSpawnEntityCreatedEntityInfo",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(SERVERSPAWNENTITYCREATEDENTITYINFO_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSpawnEntityCreatedEntityInfo {
    fn type_info() -> &'static TypeInfo {
        SERVERSPAWNENTITYCREATEDENTITYINFO_TYPE_INFO
    }
}


pub const SERVERSPAWNENTITYCREATEDENTITYINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSpawnEntityCreatedEntityInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerSpawnEntityCreatedEntityInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSpawnEntity {
}

pub const SERVERSPAWNENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSpawnEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSPAWNENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSpawnEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERSPAWNENTITY_TYPE_INFO
    }
}


pub const SERVERSPAWNENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSpawnEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerSpawnEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerOriginatingLocationSpawnContext {
}

pub const SERVERORIGINATINGLOCATIONSPAWNCONTEXT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerOriginatingLocationSpawnContext",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(USERSPAWNCONTEXT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERORIGINATINGLOCATIONSPAWNCONTEXT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerOriginatingLocationSpawnContext {
    fn type_info() -> &'static TypeInfo {
        SERVERORIGINATINGLOCATIONSPAWNCONTEXT_TYPE_INFO
    }
}


pub const SERVERORIGINATINGLOCATIONSPAWNCONTEXT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerOriginatingLocationSpawnContext-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerOriginatingLocationSpawnContext-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerOriginatingBlueprintSpawnContext {
}

pub const SERVERORIGINATINGBLUEPRINTSPAWNCONTEXT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerOriginatingBlueprintSpawnContext",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(USERSPAWNCONTEXT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERORIGINATINGBLUEPRINTSPAWNCONTEXT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerOriginatingBlueprintSpawnContext {
    fn type_info() -> &'static TypeInfo {
        SERVERORIGINATINGBLUEPRINTSPAWNCONTEXT_TYPE_INFO
    }
}


pub const SERVERORIGINATINGBLUEPRINTSPAWNCONTEXT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerOriginatingBlueprintSpawnContext-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerOriginatingBlueprintSpawnContext-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCharacterSpawnEntity {
}

pub const SERVERCHARACTERSPAWNENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterSpawnEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERSPAWNENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERCHARACTERSPAWNENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCharacterSpawnEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERCHARACTERSPAWNENTITY_TYPE_INFO
    }
}


pub const SERVERCHARACTERSPAWNENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterSpawnEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerCharacterSpawnEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerTeleportEntity {
}

pub const SERVERTELEPORTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTeleportEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERTELEPORTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerTeleportEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERTELEPORTENTITY_TYPE_INFO
    }
}


pub const SERVERTELEPORTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTeleportEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerTeleportEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerTeamFilterEntity {
}

pub const SERVERTEAMFILTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTeamFilterEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERTEAMFILTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerTeamFilterEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERTEAMFILTERENTITY_TYPE_INFO
    }
}


pub const SERVERTEAMFILTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTeamFilterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerTeamFilterEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerTeamEntity {
}

pub const SERVERTEAMENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTeamEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERTEAMENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerTeamEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERTEAMENTITY_TYPE_INFO
    }
}


pub const SERVERTEAMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTeamEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerTeamEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerTacticalObjectiveEntity {
}

pub const SERVERTACTICALOBJECTIVEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTacticalObjectiveEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERTACTICALOBJECTIVEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerTacticalObjectiveEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERTACTICALOBJECTIVEENTITY_TYPE_INFO
    }
}


pub const SERVERTACTICALOBJECTIVEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTacticalObjectiveEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerTacticalObjectiveEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerLadderComponent {
}

pub const SERVERLADDERCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLadderComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERLADDERCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerLadderComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERLADDERCOMPONENT_TYPE_INFO
    }
}


pub const SERVERLADDERCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLadderComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerLadderComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerExplosionEntity {
}

pub const SERVEREXPLOSIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerExplosionEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EXPLOSIONENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVEREXPLOSIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerExplosionEntity {
    fn type_info() -> &'static TypeInfo {
        SERVEREXPLOSIONENTITY_TYPE_INFO
    }
}


pub const SERVEREXPLOSIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerExplosionEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerExplosionEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerDynamicAvoidanceEntity {
}

pub const SERVERDYNAMICAVOIDANCEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDynamicAvoidanceEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERDYNAMICAVOIDANCEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerDynamicAvoidanceEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERDYNAMICAVOIDANCEENTITY_TYPE_INFO
    }
}


pub const SERVERDYNAMICAVOIDANCEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDynamicAvoidanceEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerDynamicAvoidanceEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerVehicleEntity {
}

pub const SERVERVEHICLEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERCONTROLLABLEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERVEHICLEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerVehicleEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERVEHICLEENTITY_TYPE_INFO
    }
}


pub const SERVERVEHICLEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerVehicleEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerWheelComponent {
}

pub const SERVERWHEELCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWheelComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERWHEELCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerWheelComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERWHEELCOMPONENT_TYPE_INFO
    }
}


pub const SERVERWHEELCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWheelComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerWheelComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerVehiclePhysicsComponent {
}

pub const SERVERVEHICLEPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehiclePhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PARTPHYSICSCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERVEHICLEPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerVehiclePhysicsComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERVEHICLEPHYSICSCOMPONENT_TYPE_INFO
    }
}


pub const SERVERVEHICLEPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehiclePhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerVehiclePhysicsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerVehicleHealthComponent {
}

pub const SERVERVEHICLEHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleHealthComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERCONTROLLABLEHEALTHCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERVEHICLEHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerVehicleHealthComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERVEHICLEHEALTHCOMPONENT_TYPE_INFO
    }
}


pub const SERVERVEHICLEHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleHealthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerVehicleHealthComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerVehicleEntryComponent {
}

pub const SERVERVEHICLEENTRYCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleEntryComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERPLAYERENTRYCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERVEHICLEENTRYCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerVehicleEntryComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERVEHICLEENTRYCOMPONENT_TYPE_INFO
    }
}


pub const SERVERVEHICLEENTRYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleEntryComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerVehicleEntryComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerVehicleCustomizationComponent {
}

pub const SERVERVEHICLECUSTOMIZATIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleCustomizationComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERVEHICLECUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerVehicleCustomizationComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERVEHICLECUSTOMIZATIONCOMPONENT_TYPE_INFO
    }
}


pub const SERVERVEHICLECUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleCustomizationComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerVehicleCustomizationComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerTrackWheelComponent {
}

pub const SERVERTRACKWHEELCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTrackWheelComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERWHEELCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERTRACKWHEELCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerTrackWheelComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERTRACKWHEELCOMPONENT_TYPE_INFO
    }
}


pub const SERVERTRACKWHEELCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTrackWheelComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerTrackWheelComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerTrackComponent {
}

pub const SERVERTRACKCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTrackComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERMESHCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERTRACKCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerTrackComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERTRACKCOMPONENT_TYPE_INFO
    }
}


pub const SERVERTRACKCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTrackComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerTrackComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerStanceFilterComponent {
}

pub const SERVERSTANCEFILTERCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStanceFilterComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSTANCEFILTERCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerStanceFilterComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERSTANCEFILTERCOMPONENT_TYPE_INFO
    }
}


pub const SERVERSTANCEFILTERCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStanceFilterComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerStanceFilterComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerRotorComponent {
}

pub const SERVERROTORCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerRotorComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERROTORCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerRotorComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERROTORCOMPONENT_TYPE_INFO
    }
}


pub const SERVERROTORCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerRotorComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerRotorComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerMeshComponent {
}

pub const SERVERMESHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMeshComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERMESHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerMeshComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERMESHCOMPONENT_TYPE_INFO
    }
}


pub const SERVERMESHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMeshComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerMeshComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerEngineComponent {
}

pub const SERVERENGINECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEngineComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERENGINECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerEngineComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERENGINECOMPONENT_TYPE_INFO
    }
}


pub const SERVERENGINECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEngineComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerEngineComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerEntryComponent {
}

pub const SERVERENTRYCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEntryComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERENTRYCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerEntryComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERENTRYCOMPONENT_TYPE_INFO
    }
}


pub const SERVERENTRYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEntryComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerEntryComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerDriverStaticObjectComponent {
}

pub const SERVERDRIVERSTATICOBJECTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDriverStaticObjectComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERDRIVERSTATICOBJECTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerDriverStaticObjectComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERDRIVERSTATICOBJECTCOMPONENT_TYPE_INFO
    }
}


pub const SERVERDRIVERSTATICOBJECTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDriverStaticObjectComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerDriverStaticObjectComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerDriverComponent {
}

pub const SERVERDRIVERCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDriverComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERDRIVERCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerDriverComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERDRIVERCOMPONENT_TYPE_INFO
    }
}


pub const SERVERDRIVERCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDriverComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerDriverComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerControllableHealthComponent {
}

pub const SERVERCONTROLLABLEHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerControllableHealthComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMEHEALTHCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERCONTROLLABLEHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerControllableHealthComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERCONTROLLABLEHEALTHCOMPONENT_TYPE_INFO
    }
}


pub const SERVERCONTROLLABLEHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerControllableHealthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerControllableHealthComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCharacterEntryComponent {
}

pub const SERVERCHARACTERENTRYCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterEntryComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMEENTRYCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERCHARACTERENTRYCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCharacterEntryComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERCHARACTERENTRYCOMPONENT_TYPE_INFO
    }
}


pub const SERVERCHARACTERENTRYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterEntryComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerCharacterEntryComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCharacterEntity {
}

pub const SERVERCHARACTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERCONTROLLABLEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERCHARACTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCharacterEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERCHARACTERENTITY_TYPE_INFO
    }
}


pub const SERVERCHARACTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerCharacterEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CharacterServerPlayerExtent {
}

pub const CHARACTERSERVERPLAYEREXTENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterServerPlayerExtent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMEPLAYERINTERNALEXTENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CHARACTERSERVERPLAYEREXTENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CharacterServerPlayerExtent {
    fn type_info() -> &'static TypeInfo {
        CHARACTERSERVERPLAYEREXTENT_TYPE_INFO
    }
}


pub const CHARACTERSERVERPLAYEREXTENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterServerPlayerExtent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("CharacterServerPlayerExtent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerWarpAnimationComponent {
}

pub const SERVERWARPANIMATIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWarpAnimationComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERWARPANIMATIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerWarpAnimationComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERWARPANIMATIONCOMPONENT_TYPE_INFO
    }
}


pub const SERVERWARPANIMATIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWarpAnimationComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerWarpAnimationComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerVehicleEntryListenerComponent {
}

pub const SERVERVEHICLEENTRYLISTENERCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleEntryListenerComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERVEHICLEENTRYLISTENERCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerVehicleEntryListenerComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERVEHICLEENTRYLISTENERCOMPONENT_TYPE_INFO
    }
}


pub const SERVERVEHICLEENTRYLISTENERCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleEntryListenerComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerVehicleEntryListenerComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCharacterPhysicsComponent {
}

pub const SERVERCHARACTERPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERCHARACTERPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCharacterPhysicsComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERCHARACTERPHYSICSCOMPONENT_TYPE_INFO
    }
}


pub const SERVERCHARACTERPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerCharacterPhysicsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCharacterMasterPhysicsComponent {
}

pub const SERVERCHARACTERMASTERPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterMasterPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CHARACTERPHYSICSCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERCHARACTERMASTERPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCharacterMasterPhysicsComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERCHARACTERMASTERPHYSICSCOMPONENT_TYPE_INFO
    }
}


pub const SERVERCHARACTERMASTERPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterMasterPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerCharacterMasterPhysicsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCharacterHealthComponent {
}

pub const SERVERCHARACTERHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterHealthComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERCONTROLLABLEHEALTHCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERCHARACTERHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCharacterHealthComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERCHARACTERHEALTHCOMPONENT_TYPE_INFO
    }
}


pub const SERVERCHARACTERHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterHealthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerCharacterHealthComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCharacterCustomizationComponent {
}

pub const SERVERCHARACTERCUSTOMIZATIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterCustomizationComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERCHARACTERCUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCharacterCustomizationComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERCHARACTERCUSTOMIZATIONCOMPONENT_TYPE_INFO
    }
}


pub const SERVERCHARACTERCUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterCustomizationComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerCharacterCustomizationComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCharacterCameraComponent {
}

pub const SERVERCHARACTERCAMERACOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterCameraComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERCHARACTERCAMERACOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCharacterCameraComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERCHARACTERCAMERACOMPONENT_TYPE_INFO
    }
}


pub const SERVERCHARACTERCAMERACOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterCameraComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerCharacterCameraComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAntInputComponent {
}

pub const SERVERANTINPUTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAntInputComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERANTINPUTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAntInputComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERANTINPUTCOMPONENT_TYPE_INFO
    }
}


pub const SERVERANTINPUTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAntInputComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerAntInputComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAntDrivenComponent {
}

pub const SERVERANTDRIVENCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAntDrivenComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERANTDRIVENCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAntDrivenComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERANTDRIVENCOMPONENT_TYPE_INFO
    }
}


pub const SERVERANTDRIVENCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAntDrivenComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerAntDrivenComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerWarpAnimationEntity {
}

pub const SERVERWARPANIMATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWarpAnimationEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERWARPANIMATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerWarpAnimationEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERWARPANIMATIONENTITY_TYPE_INFO
    }
}


pub const SERVERWARPANIMATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWarpAnimationEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerWarpAnimationEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPhysicsDrivenAnimationEntity {
}

pub const SERVERPHYSICSDRIVENANIMATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPhysicsDrivenAnimationEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSDRIVENANIMATIONENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERPHYSICSDRIVENANIMATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPhysicsDrivenAnimationEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERPHYSICSDRIVENANIMATIONENTITY_TYPE_INFO
    }
}


pub const SERVERPHYSICSDRIVENANIMATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPhysicsDrivenAnimationEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerPhysicsDrivenAnimationEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCannedScenarioEntity {
}

pub const SERVERCANNEDSCENARIOENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCannedScenarioEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERCANNEDSCENARIOENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCannedScenarioEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERCANNEDSCENARIOENTITY_TYPE_INFO
    }
}


pub const SERVERCANNEDSCENARIOENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCannedScenarioEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerCannedScenarioEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerRecordVehicleTrack {
}

pub const SERVERRECORDVEHICLETRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerRecordVehicleTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RECORDTRACKBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERRECORDVEHICLETRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerRecordVehicleTrack {
    fn type_info() -> &'static TypeInfo {
        SERVERRECORDVEHICLETRACK_TYPE_INFO
    }
}


pub const SERVERRECORDVEHICLETRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerRecordVehicleTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerRecordVehicleTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSyncedSequenceEntity {
}

pub const SERVERSYNCEDSEQUENCEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSyncedSequenceEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SEQUENCEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSYNCEDSEQUENCEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSyncedSequenceEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERSYNCEDSEQUENCEENTITY_TYPE_INFO
    }
}


pub const SERVERSYNCEDSEQUENCEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSyncedSequenceEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerSyncedSequenceEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSpeedEventGateEntity {
}

pub const SERVERSPEEDEVENTGATEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSpeedEventGateEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSPEEDEVENTGATEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSpeedEventGateEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERSPEEDEVENTGATEENTITY_TYPE_INFO
    }
}


pub const SERVERSPEEDEVENTGATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSpeedEventGateEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerSpeedEventGateEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSaveGameLoadedEntity {
}

pub const SERVERSAVEGAMELOADEDENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSaveGameLoadedEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSAVEGAMELOADEDENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSaveGameLoadedEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERSAVEGAMELOADEDENTITY_TYPE_INFO
    }
}


pub const SERVERSAVEGAMELOADEDENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSaveGameLoadedEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerSaveGameLoadedEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSaveEntity {
}

pub const SERVERSAVEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSaveEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSAVEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSaveEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERSAVEENTITY_TYPE_INFO
    }
}


pub const SERVERSAVEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSaveEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerSaveEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPlayerIteratorEntity {
}

pub const SERVERPLAYERITERATORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerIteratorEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PLAYERITERATORENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERPLAYERITERATORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPlayerIteratorEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERPLAYERITERATORENTITY_TYPE_INFO
    }
}


pub const SERVERPLAYERITERATORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerIteratorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerPlayerIteratorEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPlayerFilterEntity {
}

pub const SERVERPLAYERFILTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerFilterEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERPLAYERFILTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPlayerFilterEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERPLAYERFILTERENTITY_TYPE_INFO
    }
}


pub const SERVERPLAYERFILTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerFilterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerPlayerFilterEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerObjectiveEntity {
}

pub const SERVEROBJECTIVEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerObjectiveEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVEROBJECTIVEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerObjectiveEntity {
    fn type_info() -> &'static TypeInfo {
        SERVEROBJECTIVEENTITY_TYPE_INFO
    }
}


pub const SERVEROBJECTIVEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerObjectiveEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerObjectiveEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerObjectAreaQueryEntity {
}

pub const SERVEROBJECTAREAQUERYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerObjectAreaQueryEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(OBJECTAREAQUERYENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVEROBJECTAREAQUERYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerObjectAreaQueryEntity {
    fn type_info() -> &'static TypeInfo {
        SERVEROBJECTAREAQUERYENTITY_TYPE_INFO
    }
}


pub const SERVEROBJECTAREAQUERYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerObjectAreaQueryEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerObjectAreaQueryEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerMapMarkerEntity {
}

pub const SERVERMAPMARKERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMapMarkerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERMAPMARKERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerMapMarkerEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERMAPMARKERENTITY_TYPE_INFO
    }
}


pub const SERVERMAPMARKERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMapMarkerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerMapMarkerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerLevelControlEntity {
}

pub const SERVERLEVELCONTROLENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLevelControlEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERLEVELCONTROLENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerLevelControlEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERLEVELCONTROLENTITY_TYPE_INFO
    }
}


pub const SERVERLEVELCONTROLENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLevelControlEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerLevelControlEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerInputRestrictionEntity {
}

pub const SERVERINPUTRESTRICTIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerInputRestrictionEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERINPUTRESTRICTIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerInputRestrictionEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERINPUTRESTRICTIONENTITY_TYPE_INFO
    }
}


pub const SERVERINPUTRESTRICTIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerInputRestrictionEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerInputRestrictionEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerHumanPlayerEntity {
}

pub const SERVERHUMANPLAYERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerHumanPlayerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERHUMANPLAYERPROXYENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERHUMANPLAYERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerHumanPlayerEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERHUMANPLAYERENTITY_TYPE_INFO
    }
}


pub const SERVERHUMANPLAYERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerHumanPlayerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerHumanPlayerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerHumanPlayerProxyEntity {
}

pub const SERVERHUMANPLAYERPROXYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerHumanPlayerProxyEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERHUMANPLAYERPROXYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerHumanPlayerProxyEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERHUMANPLAYERPROXYENTITY_TYPE_INFO
    }
}


pub const SERVERHUMANPLAYERPROXYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerHumanPlayerProxyEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerHumanPlayerProxyEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerEventMemoryEntity {
}

pub const SERVEREVENTMEMORYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEventMemoryEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVEREVENTMEMORYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerEventMemoryEntity {
    fn type_info() -> &'static TypeInfo {
        SERVEREVENTMEMORYENTITY_TYPE_INFO
    }
}


pub const SERVEREVENTMEMORYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEventMemoryEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerEventMemoryEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerEventIfSwitchEntity {
}

pub const SERVEREVENTIFSWITCHENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEventIfSwitchEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVEREVENTIFSWITCHENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerEventIfSwitchEntity {
    fn type_info() -> &'static TypeInfo {
        SERVEREVENTIFSWITCHENTITY_TYPE_INFO
    }
}


pub const SERVEREVENTIFSWITCHENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEventIfSwitchEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerEventIfSwitchEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCustomizeCharacterEntity {
}

pub const SERVERCUSTOMIZECHARACTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCustomizeCharacterEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERCUSTOMIZECHARACTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCustomizeCharacterEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERCUSTOMIZECHARACTERENTITY_TYPE_INFO
    }
}


pub const SERVERCUSTOMIZECHARACTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCustomizeCharacterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerCustomizeCharacterEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAreaQueryEntity {
}

pub const SERVERAREAQUERYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAreaQueryEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAREAQUERYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAreaQueryEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAREAQUERYENTITY_TYPE_INFO
    }
}


pub const SERVERAREAQUERYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAreaQueryEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerAreaQueryEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPlayerEvent {
}

pub const SERVERPLAYEREVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerEvent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PLAYEREVENTBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERPLAYEREVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPlayerEvent {
    fn type_info() -> &'static TypeInfo {
        SERVERPLAYEREVENT_TYPE_INFO
    }
}


pub const SERVERPLAYEREVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerPlayerEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerDoublePlayerEvent {
}

pub const SERVERDOUBLEPLAYEREVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDoublePlayerEvent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERPLAYEREVENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERDOUBLEPLAYEREVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerDoublePlayerEvent {
    fn type_info() -> &'static TypeInfo {
        SERVERDOUBLEPLAYEREVENT_TYPE_INFO
    }
}


pub const SERVERDOUBLEPLAYEREVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDoublePlayerEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerDoublePlayerEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerDamageGiverEvent {
}

pub const SERVERDAMAGEGIVEREVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDamageGiverEvent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERPLAYEREVENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERDAMAGEGIVEREVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerDamageGiverEvent {
    fn type_info() -> &'static TypeInfo {
        SERVERDAMAGEGIVEREVENT_TYPE_INFO
    }
}


pub const SERVERDAMAGEGIVEREVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDamageGiverEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerDamageGiverEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPredestructionEntity {
}

pub const SERVERPREDESTRUCTIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPredestructionEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERPREDESTRUCTIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPredestructionEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERPREDESTRUCTIONENTITY_TYPE_INFO
    }
}


pub const SERVERPREDESTRUCTIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPredestructionEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerPredestructionEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerBangerPhysicsComponent {
}

pub const SERVERBANGERPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBangerPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PARTPHYSICSCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERBANGERPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerBangerPhysicsComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERBANGERPHYSICSCOMPONENT_TYPE_INFO
    }
}


pub const SERVERBANGERPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBangerPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerBangerPhysicsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerBangerHealthModule {
}

pub const SERVERBANGERHEALTHMODULE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBangerHealthModule",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERBANGERHEALTHMODULE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerBangerHealthModule {
    fn type_info() -> &'static TypeInfo {
        SERVERBANGERHEALTHMODULE_TYPE_INFO
    }
}


pub const SERVERBANGERHEALTHMODULE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBangerHealthModule-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerBangerHealthModule-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerBangerHealthComponent {
}

pub const SERVERBANGERHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBangerHealthComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMEHEALTHCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERBANGERHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerBangerHealthComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERBANGERHEALTHCOMPONENT_TYPE_INFO
    }
}


pub const SERVERBANGERHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBangerHealthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerBangerHealthComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSubView {
}

pub const SERVERSUBVIEW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSubView",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SUBVIEW_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSUBVIEW_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSubView {
    fn type_info() -> &'static TypeInfo {
        SERVERSUBVIEW_TYPE_INFO
    }
}


pub const SERVERSUBVIEW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSubView-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerSubView-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSpectatorSubView {
}

pub const SERVERSPECTATORSUBVIEW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSpectatorSubView",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERSPECTATORSUBVIEWBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSPECTATORSUBVIEW_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSpectatorSubView {
    fn type_info() -> &'static TypeInfo {
        SERVERSPECTATORSUBVIEW_TYPE_INFO
    }
}


pub const SERVERSPECTATORSUBVIEW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSpectatorSubView-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerSpectatorSubView-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSpectatorSubViewBase {
}

pub const SERVERSPECTATORSUBVIEWBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSpectatorSubViewBase",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERSUBVIEW_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSPECTATORSUBVIEWBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSpectatorSubViewBase {
    fn type_info() -> &'static TypeInfo {
        SERVERSPECTATORSUBVIEWBASE_TYPE_INFO
    }
}


pub const SERVERSPECTATORSUBVIEWBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSpectatorSubViewBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerSpectatorSubViewBase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGameSubView {
}

pub const SERVERGAMESUBVIEW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameSubView",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERSUBVIEW_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERGAMESUBVIEW_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerGameSubView {
    fn type_info() -> &'static TypeInfo {
        SERVERGAMESUBVIEW_TYPE_INFO
    }
}


pub const SERVERGAMESUBVIEW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameSubView-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerGameSubView-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerConnection {
}

pub const SERVERCONNECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerConnection",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENGINECONNECTIONPEER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERCONNECTION_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerConnection {
    fn type_info() -> &'static TypeInfo {
        SERVERCONNECTION_TYPE_INFO
    }
}


pub const SERVERCONNECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerConnection-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerConnection-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerVehicleStateTriggerEntity {
}

pub const SERVERVEHICLESTATETRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleStateTriggerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERTRIGGERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERVEHICLESTATETRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerVehicleStateTriggerEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERVEHICLESTATETRIGGERENTITY_TYPE_INFO
    }
}


pub const SERVERVEHICLESTATETRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleStateTriggerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerVehicleStateTriggerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerUnderFireTriggerEntity {
}

pub const SERVERUNDERFIRETRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerUnderFireTriggerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERTRIGGERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERUNDERFIRETRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerUnderFireTriggerEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERUNDERFIRETRIGGERENTITY_TYPE_INFO
    }
}


pub const SERVERUNDERFIRETRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerUnderFireTriggerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerUnderFireTriggerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCharacterLookAtTriggerEntity {
}

pub const SERVERCHARACTERLOOKATTRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterLookAtTriggerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERTRIGGERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERCHARACTERLOOKATTRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCharacterLookAtTriggerEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERCHARACTERLOOKATTRIGGERENTITY_TYPE_INFO
    }
}


pub const SERVERCHARACTERLOOKATTRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterLookAtTriggerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerCharacterLookAtTriggerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerTriggerEntity {
}

pub const SERVERTRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTriggerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERTRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerTriggerEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERTRIGGERENTITY_TYPE_INFO
    }
}


pub const SERVERTRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTriggerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerTriggerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPlayerTakeOverTriggerEntity {
}

pub const SERVERPLAYERTAKEOVERTRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerTakeOverTriggerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERPLAYERTAKEOVERTRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPlayerTakeOverTriggerEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERPLAYERTAKEOVERTRIGGERENTITY_TYPE_INFO
    }
}


pub const SERVERPLAYERTAKEOVERTRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerTakeOverTriggerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerPlayerTakeOverTriggerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerChildComponent {
}

pub const SERVERCHILDCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerChildComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERCHILDCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerChildComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERCHILDCOMPONENT_TYPE_INFO
    }
}


pub const SERVERCHILDCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerChildComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerChildComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerChildBarrelComponent {
}

pub const SERVERCHILDBARRELCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerChildBarrelComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERCHILDCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERCHILDBARRELCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerChildBarrelComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERCHILDBARRELCOMPONENT_TYPE_INFO
    }
}


pub const SERVERCHILDBARRELCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerChildBarrelComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerChildBarrelComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerChassisComponent {
}

pub const SERVERCHASSISCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerChassisComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERPARTCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERCHASSISCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerChassisComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERCHASSISCOMPONENT_TYPE_INFO
    }
}


pub const SERVERCHASSISCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerChassisComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerChassisComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCameraComponent {
}

pub const SERVERCAMERACOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCameraComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERCAMERACOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCameraComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERCAMERACOMPONENT_TYPE_INFO
    }
}


pub const SERVERCAMERACOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCameraComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerCameraComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPlayerExtent {
}

pub const SERVERPLAYEREXTENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerExtent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(SERVERPLAYEREXTENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPlayerExtent {
    fn type_info() -> &'static TypeInfo {
        SERVERPLAYEREXTENT_TYPE_INFO
    }
}


pub const SERVERPLAYEREXTENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerExtent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerPlayerExtent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGamePlayerExtent {
}

pub const SERVERGAMEPLAYEREXTENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGamePlayerExtent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERPLAYEREXTENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERGAMEPLAYEREXTENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerGamePlayerExtent {
    fn type_info() -> &'static TypeInfo {
        SERVERGAMEPLAYEREXTENT_TYPE_INFO
    }
}


pub const SERVERGAMEPLAYEREXTENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGamePlayerExtent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerGamePlayerExtent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGamePlayerInternalExtent {
}

pub const SERVERGAMEPLAYERINTERNALEXTENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGamePlayerInternalExtent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERPLAYEREXTENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERGAMEPLAYERINTERNALEXTENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerGamePlayerInternalExtent {
    fn type_info() -> &'static TypeInfo {
        SERVERGAMEPLAYERINTERNALEXTENT_TYPE_INFO
    }
}


pub const SERVERGAMEPLAYERINTERNALEXTENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGamePlayerInternalExtent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerGamePlayerInternalExtent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGameSplineEntity {
}

pub const SERVERGAMESPLINEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameSplineEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERGAMESPLINEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerGameSplineEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERGAMESPLINEENTITY_TYPE_INFO
    }
}


pub const SERVERGAMESPLINEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameSplineEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerGameSplineEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAreaImmunityComponent {
}

pub const SERVERAREAIMMUNITYCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAreaImmunityComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAREAIMMUNITYCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAreaImmunityComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERAREAIMMUNITYCOMPONENT_TYPE_INFO
    }
}


pub const SERVERAREAIMMUNITYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAreaImmunityComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerAreaImmunityComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerDynamicFireEntity {
}

pub const SERVERDYNAMICFIREENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDynamicFireEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENTENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERDYNAMICFIREENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerDynamicFireEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERDYNAMICFIREENTITY_TYPE_INFO
    }
}


pub const SERVERDYNAMICFIREENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDynamicFireEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerDynamicFireEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerControllableEntity {
}

pub const SERVERCONTROLLABLEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerControllableEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERPHYSICSENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERCONTROLLABLEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerControllableEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERCONTROLLABLEENTITY_TYPE_INFO
    }
}


pub const SERVERCONTROLLABLEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerControllableEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerControllableEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerWarningSystemComponent {
}

pub const SERVERWARNINGSYSTEMCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWarningSystemComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERWARNINGSYSTEMCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerWarningSystemComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERWARNINGSYSTEMCOMPONENT_TYPE_INFO
    }
}


pub const SERVERWARNINGSYSTEMCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWarningSystemComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerWarningSystemComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerUnlockComponent {
}

pub const SERVERUNLOCKCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerUnlockComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERUNLOCKCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerUnlockComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERUNLOCKCOMPONENT_TYPE_INFO
    }
}


pub const SERVERUNLOCKCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerUnlockComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerUnlockComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPlayerEntryComponent {
}

pub const SERVERPLAYERENTRYCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerEntryComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMEENTRYCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERPLAYERENTRYCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPlayerEntryComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERPLAYERENTRYCOMPONENT_TYPE_INFO
    }
}


pub const SERVERPLAYERENTRYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerEntryComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerPlayerEntryComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGameEntryComponent {
}

pub const SERVERGAMEENTRYCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameEntryComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERENTRYCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERGAMEENTRYCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerGameEntryComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERGAMEENTRYCOMPONENT_TYPE_INFO
    }
}


pub const SERVERGAMEENTRYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameEntryComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerGameEntryComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSubLevelEntity {
}

pub const SERVERSUBLEVELENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSubLevelEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SUBLEVELENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSUBLEVELENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSubLevelEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERSUBLEVELENTITY_TYPE_INFO
    }
}


pub const SERVERSUBLEVELENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSubLevelEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerSubLevelEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSubLevelCollectionEntity {
}

pub const SERVERSUBLEVELCOLLECTIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSubLevelCollectionEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SUBLEVELCOLLECTIONENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSUBLEVELCOLLECTIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSubLevelCollectionEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERSUBLEVELCOLLECTIONENTITY_TYPE_INFO
    }
}


pub const SERVERSUBLEVELCOLLECTIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSubLevelCollectionEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerSubLevelCollectionEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerStartPointEntity {
}

pub const SERVERSTARTPOINTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStartPointEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSTARTPOINTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerStartPointEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERSTARTPOINTENTITY_TYPE_INFO
    }
}


pub const SERVERSTARTPOINTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStartPointEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerStartPointEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct BangerHealthModuleData {
    pub health: f32,
    pub material_pair: super::entity::MaterialDecl,
}

pub const BANGERHEALTHMODULEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BangerHealthModuleData",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMECOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Health",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BangerHealthModuleData, health),
            },
            FieldInfoData {
                name: "MaterialPair",
                flags: MemberInfoFlags::new(0),
                field_type: MATERIALDECL_TYPE_INFO,
                rust_offset: offset_of!(BangerHealthModuleData, material_pair),
            },
        ],
    }),
    array_type: Some(BANGERHEALTHMODULEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for BangerHealthModuleData {
    fn type_info() -> &'static TypeInfo {
        BANGERHEALTHMODULEDATA_TYPE_INFO
    }
}


pub const BANGERHEALTHMODULEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BangerHealthModuleData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("BangerHealthModuleData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerProjectileOnSpawnMessage {
}

pub const SERVERPROJECTILEONSPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerProjectileOnSpawnMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerProjectileOnSpawnMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERPROJECTILEONSPAWNMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AIDirectorStateMessage {
}

pub const AIDIRECTORSTATEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIDirectorStateMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for AIDirectorStateMessage {
    fn type_info() -> &'static TypeInfo {
        AIDIRECTORSTATEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AISpawnBotMessage {
}

pub const AISPAWNBOTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AISpawnBotMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for AISpawnBotMessage {
    fn type_info() -> &'static TypeInfo {
        AISPAWNBOTMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AIPlayerEnableAsTargetMessage {
}

pub const AIPLAYERENABLEASTARGETMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIPlayerEnableAsTargetMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for AIPlayerEnableAsTargetMessage {
    fn type_info() -> &'static TypeInfo {
        AIPLAYERENABLEASTARGETMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerMissionObjectiveCompletedMessage {
}

pub const SERVERMISSIONOBJECTIVECOMPLETEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMissionObjectiveCompletedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerMissionObjectiveCompletedMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERMISSIONOBJECTIVECOMPLETEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerRoundInterruptedMessage {
}

pub const SERVERROUNDINTERRUPTEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerRoundInterruptedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerRoundInterruptedMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERROUNDINTERRUPTEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerRoundOverMessage {
}

pub const SERVERROUNDOVERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerRoundOverMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerRoundOverMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERROUNDOVERMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerRoundResetMessage {
}

pub const SERVERROUNDRESETMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerRoundResetMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerRoundResetMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERROUNDRESETMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGameplayCheckpointActivatedMessage {
}

pub const SERVERGAMEPLAYCHECKPOINTACTIVATEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameplayCheckpointActivatedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerGameplayCheckpointActivatedMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERGAMEPLAYCHECKPOINTACTIVATEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGameplayCheckpointTriggeredMessage {
}

pub const SERVERGAMEPLAYCHECKPOINTTRIGGEREDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameplayCheckpointTriggeredMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerGameplayCheckpointTriggeredMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERGAMEPLAYCHECKPOINTTRIGGEREDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGameModeResetMessage {
}

pub const SERVERGAMEMODERESETMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameModeResetMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerGameModeResetMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERGAMEMODERESETMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGameplaySetPostRoundLogicMessage {
}

pub const SERVERGAMEPLAYSETPOSTROUNDLOGICMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameplaySetPostRoundLogicMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerGameplaySetPostRoundLogicMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERGAMEPLAYSETPOSTROUNDLOGICMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGameplaySetPreRoundLogicMessage {
}

pub const SERVERGAMEPLAYSETPREROUNDLOGICMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameplaySetPreRoundLogicMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerGameplaySetPreRoundLogicMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERGAMEPLAYSETPREROUNDLOGICMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGameplayGameModeResetMessage {
}

pub const SERVERGAMEPLAYGAMEMODERESETMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameplayGameModeResetMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerGameplayGameModeResetMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERGAMEPLAYGAMEMODERESETMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGameplayServerPlayerMenuCancelMessage {
}

pub const SERVERGAMEPLAYSERVERPLAYERMENUCANCELMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameplayServerPlayerMenuCancelMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerGameplayServerPlayerMenuCancelMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERGAMEPLAYSERVERPLAYERMENUCANCELMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGameplayServerPlayerMenuOkMessage {
}

pub const SERVERGAMEPLAYSERVERPLAYERMENUOKMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameplayServerPlayerMenuOkMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerGameplayServerPlayerMenuOkMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERGAMEPLAYSERVERPLAYERMENUOKMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGameplayPreviousWeatherStateMessage {
}

pub const SERVERGAMEPLAYPREVIOUSWEATHERSTATEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameplayPreviousWeatherStateMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerGameplayPreviousWeatherStateMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERGAMEPLAYPREVIOUSWEATHERSTATEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGameplayFightHarderMessage {
}

pub const SERVERGAMEPLAYFIGHTHARDERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameplayFightHarderMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerGameplayFightHarderMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERGAMEPLAYFIGHTHARDERMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGameplayDeserterReturnMessage {
}

pub const SERVERGAMEPLAYDESERTERRETURNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameplayDeserterReturnMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerGameplayDeserterReturnMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERGAMEPLAYDESERTERRETURNMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGameplayDeserterMessage {
}

pub const SERVERGAMEPLAYDESERTERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameplayDeserterMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerGameplayDeserterMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERGAMEPLAYDESERTERMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGameplayPlayerMenuCancelMessage {
}

pub const SERVERGAMEPLAYPLAYERMENUCANCELMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameplayPlayerMenuCancelMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerGameplayPlayerMenuCancelMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERGAMEPLAYPLAYERMENUCANCELMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGameplayPlayerMenuOkMessage {
}

pub const SERVERGAMEPLAYPLAYERMENUOKMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameplayPlayerMenuOkMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerGameplayPlayerMenuOkMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERGAMEPLAYPLAYERMENUOKMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGameplayVoiceOverFinishedMessage {
}

pub const SERVERGAMEPLAYVOICEOVERFINISHEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameplayVoiceOverFinishedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerGameplayVoiceOverFinishedMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERGAMEPLAYVOICEOVERFINISHEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerStaticModelDamagedPartByPlayerMessage {
}

pub const SERVERSTATICMODELDAMAGEDPARTBYPLAYERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticModelDamagedPartByPlayerMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerStaticModelDamagedPartByPlayerMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERSTATICMODELDAMAGEDPARTBYPLAYERMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerStaticModelDestroyedPartMessage {
}

pub const SERVERSTATICMODELDESTROYEDPARTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticModelDestroyedPartMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ServerStaticModelDestroyedPartMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERSTATICMODELDESTROYEDPARTMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerStaticModelGroupDestroyedPartMessage {
}

pub const SERVERSTATICMODELGROUPDESTROYEDPARTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticModelGroupDestroyedPartMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ServerStaticModelGroupDestroyedPartMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERSTATICMODELGROUPDESTROYEDPARTMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerStaticModelDestroyedAllCollapsablePartsMessage {
}

pub const SERVERSTATICMODELDESTROYEDALLCOLLAPSABLEPARTSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticModelDestroyedAllCollapsablePartsMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ServerStaticModelDestroyedAllCollapsablePartsMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERSTATICMODELDESTROYEDALLCOLLAPSABLEPARTSMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerStaticModelSpawnMessage {
}

pub const SERVERSTATICMODELSPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticModelSpawnMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ServerStaticModelSpawnMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERSTATICMODELSPAWNMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCollisionExplosionPackDestroyedMessage {
}

pub const SERVERCOLLISIONEXPLOSIONPACKDESTROYEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCollisionExplosionPackDestroyedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerCollisionExplosionPackDestroyedMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERCOLLISIONEXPLOSIONPACKDESTROYEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCollisionExplosionPackPlacedMessage {
}

pub const SERVERCOLLISIONEXPLOSIONPACKPLACEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCollisionExplosionPackPlacedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerCollisionExplosionPackPlacedMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERCOLLISIONEXPLOSIONPACKPLACEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCollisionExplosionUnSpawnMessage {
}

pub const SERVERCOLLISIONEXPLOSIONUNSPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCollisionExplosionUnSpawnMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerCollisionExplosionUnSpawnMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERCOLLISIONEXPLOSIONUNSPAWNMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCollisionExplosionDamageMessage {
}

pub const SERVERCOLLISIONEXPLOSIONDAMAGEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCollisionExplosionDamageMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ServerCollisionExplosionDamageMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERCOLLISIONEXPLOSIONDAMAGEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCollisionExplosionSpawnMessage {
}

pub const SERVERCOLLISIONEXPLOSIONSPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCollisionExplosionSpawnMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ServerCollisionExplosionSpawnMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERCOLLISIONEXPLOSIONSPAWNMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCollisionProjectileImpactMessage {
}

pub const SERVERCOLLISIONPROJECTILEIMPACTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCollisionProjectileImpactMessage",
    flags: MemberInfoFlags::new(32841),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ServerCollisionProjectileImpactMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERCOLLISIONPROJECTILEIMPACTMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCollisionProjectileFireMessage {
}

pub const SERVERCOLLISIONPROJECTILEFIREMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCollisionProjectileFireMessage",
    flags: MemberInfoFlags::new(32841),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ServerCollisionProjectileFireMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERCOLLISIONPROJECTILEFIREMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCollisionGrenadeCollisionMessage {
}

pub const SERVERCOLLISIONGRENADECOLLISIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCollisionGrenadeCollisionMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ServerCollisionGrenadeCollisionMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERCOLLISIONGRENADECOLLISIONMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCollisionGrenadeThrowMessage {
}

pub const SERVERCOLLISIONGRENADETHROWMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCollisionGrenadeThrowMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ServerCollisionGrenadeThrowMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERCOLLISIONGRENADETHROWMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerWeaponComponentWeaponOnUnspawnMessage {
}

pub const SERVERWEAPONCOMPONENTWEAPONONUNSPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponComponentWeaponOnUnspawnMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerWeaponComponentWeaponOnUnspawnMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERWEAPONCOMPONENTWEAPONONUNSPAWNMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerWeaponComponentWeaponOnSpawnMessage {
}

pub const SERVERWEAPONCOMPONENTWEAPONONSPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponComponentWeaponOnSpawnMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerWeaponComponentWeaponOnSpawnMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERWEAPONCOMPONENTWEAPONONSPAWNMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerEntityPickupOnUnspawnMessage {
}

pub const SERVERENTITYPICKUPONUNSPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEntityPickupOnUnspawnMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerEntityPickupOnUnspawnMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERENTITYPICKUPONUNSPAWNMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerEntityPickupOnSpawnMessage {
}

pub const SERVERENTITYPICKUPONSPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEntityPickupOnSpawnMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerEntityPickupOnSpawnMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERENTITYPICKUPONSPAWNMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerEntityBangerEntityOnUnspawnMessage {
}

pub const SERVERENTITYBANGERENTITYONUNSPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEntityBangerEntityOnUnspawnMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerEntityBangerEntityOnUnspawnMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERENTITYBANGERENTITYONUNSPAWNMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerEntityBangerEntityOnSpawnMessage {
}

pub const SERVERENTITYBANGERENTITYONSPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEntityBangerEntityOnSpawnMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerEntityBangerEntityOnSpawnMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERENTITYBANGERENTITYONSPAWNMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerClubMemberDeletedMessage {
}

pub const SERVERCLUBMEMBERDELETEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerClubMemberDeletedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerClubMemberDeletedMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERCLUBMEMBERDELETEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerClubMemberCreatedMessage {
}

pub const SERVERCLUBMEMBERCREATEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerClubMemberCreatedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerClubMemberCreatedMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERCLUBMEMBERCREATEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerVehicleLockableMessage {
}

pub const SERVERVEHICLELOCKABLEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleLockableMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerVehicleLockableMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERVEHICLELOCKABLEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerVehicleExitMessage {
}

pub const SERVERVEHICLEEXITMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleExitMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerVehicleExitMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERVEHICLEEXITMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerVehicleEnterMessage {
}

pub const SERVERVEHICLEENTERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleEnterMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerVehicleEnterMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERVEHICLEENTERMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerVehicleDisabledMessage {
}

pub const SERVERVEHICLEDISABLEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleDisabledMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerVehicleDisabledMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERVEHICLEDISABLEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerVehicleDamageMessage {
}

pub const SERVERVEHICLEDAMAGEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleDamageMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerVehicleDamageMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERVEHICLEDAMAGEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerVehicleEnterRestrictionMessage {
}

pub const SERVERVEHICLEENTERRESTRICTIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleEnterRestrictionMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerVehicleEnterRestrictionMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERVEHICLEENTERRESTRICTIONMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerVehicleUnspawnMessage {
}

pub const SERVERVEHICLEUNSPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleUnspawnMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerVehicleUnspawnMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERVEHICLEUNSPAWNMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerVehicleSpawnDoneMessage {
}

pub const SERVERVEHICLESPAWNDONEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleSpawnDoneMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerVehicleSpawnDoneMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERVEHICLESPAWNDONEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerVehicleForceArmamentReturnMessage {
}

pub const SERVERVEHICLEFORCEARMAMENTRETURNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleForceArmamentReturnMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerVehicleForceArmamentReturnMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERVEHICLEFORCEARMAMENTRETURNMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerVehicleSetRemoteControlledDamageGiverMessage {
}

pub const SERVERVEHICLESETREMOTECONTROLLEDDAMAGEGIVERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleSetRemoteControlledDamageGiverMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerVehicleSetRemoteControlledDamageGiverMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERVEHICLESETREMOTECONTROLLEDDAMAGEGIVERMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerVehicleSwitchTeamMessage {
}

pub const SERVERVEHICLESWITCHTEAMMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleSwitchTeamMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerVehicleSwitchTeamMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERVEHICLESWITCHTEAMMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerVehicleDestroyedMessage {
}

pub const SERVERVEHICLEDESTROYEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleDestroyedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerVehicleDestroyedMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERVEHICLEDESTROYEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPlayerDisconnectMessage {
}

pub const SERVERPLAYERDISCONNECTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerDisconnectMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerDisconnectMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERPLAYERDISCONNECTMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPlayerStartedFireMessage {
}

pub const SERVERPLAYERSTARTEDFIREMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerStartedFireMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerStartedFireMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERPLAYERSTARTEDFIREMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPlayerOnPickupMessage {
}

pub const SERVERPLAYERONPICKUPMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerOnPickupMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerOnPickupMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERPLAYERONPICKUPMESSAGE_TYPE_INFO
    }
}

#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum PickupItemType {
    #[default]
    PITWeapon = 0,
    PITAmmo = 1,
}

pub const PICKUPITEMTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PickupItemType",
    flags: MemberInfoFlags::new(49429),
    module: "GameServer",
    data: TypeInfoData::Enum,
    array_type: Some(PICKUPITEMTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PickupItemType {
    fn type_info() -> &'static TypeInfo {
        PICKUPITEMTYPE_TYPE_INFO
    }
}


pub const PICKUPITEMTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PickupItemType-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("PickupItemType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPlayerChatMessage {
}

pub const SERVERPLAYERCHATMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerChatMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerChatMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERPLAYERCHATMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPlayerExitEntryMessage {
}

pub const SERVERPLAYEREXITENTRYMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerExitEntryMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerExitEntryMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERPLAYEREXITENTRYMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPlayerEnterEntryMessage {
}

pub const SERVERPLAYERENTERENTRYMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerEnterEntryMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerEnterEntryMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERPLAYERENTERENTRYMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPlayerAboutToClearCharacterMessage {
}

pub const SERVERPLAYERABOUTTOCLEARCHARACTERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerAboutToClearCharacterMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerAboutToClearCharacterMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERPLAYERABOUTTOCLEARCHARACTERMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPlayerInstantSuicideMessage {
}

pub const SERVERPLAYERINSTANTSUICIDEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerInstantSuicideMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerInstantSuicideMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERPLAYERINSTANTSUICIDEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPlayerKilledMessage {
}

pub const SERVERPLAYERKILLEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerKilledMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ServerPlayerKilledMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERPLAYERKILLEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPlayerManuallySelectedSpawnPointMessage {
}

pub const SERVERPLAYERMANUALLYSELECTEDSPAWNPOINTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerManuallySelectedSpawnPointMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerManuallySelectedSpawnPointMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERPLAYERMANUALLYSELECTEDSPAWNPOINTMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPlayerChangeChatChannelMessage {
}

pub const SERVERPLAYERCHANGECHATCHANNELMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerChangeChatChannelMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerChangeChatChannelMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERPLAYERCHANGECHATCHANNELMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPlayerSwitchTeamMessage {
}

pub const SERVERPLAYERSWITCHTEAMMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerSwitchTeamMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerSwitchTeamMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERPLAYERSWITCHTEAMMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPlayerKitPickedupMessage {
}

pub const SERVERPLAYERKITPICKEDUPMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerKitPickedupMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerKitPickedupMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERPLAYERKITPICKEDUPMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPlayerKitReplacedMessage {
}

pub const SERVERPLAYERKITREPLACEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerKitReplacedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerKitReplacedMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERPLAYERKITREPLACEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPlayerChangedCharacterMessage {
}

pub const SERVERPLAYERCHANGEDCHARACTERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerChangedCharacterMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerChangedCharacterMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERPLAYERCHANGEDCHARACTERMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPlayerReviveRefusedMessage {
}

pub const SERVERPLAYERREVIVEREFUSEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerReviveRefusedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerReviveRefusedMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERPLAYERREVIVEREFUSEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPlayerReviveAcceptedMessage {
}

pub const SERVERPLAYERREVIVEACCEPTEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerReviveAcceptedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerReviveAcceptedMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERPLAYERREVIVEACCEPTEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPlayerReviveMessage {
}

pub const SERVERPLAYERREVIVEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerReviveMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerReviveMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERPLAYERREVIVEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPlayerLeftLevelMessage {
}

pub const SERVERPLAYERLEFTLEVELMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerLeftLevelMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerLeftLevelMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERPLAYERLEFTLEVELMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPlayerReleasingLevelMessage {
}

pub const SERVERPLAYERRELEASINGLEVELMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerReleasingLevelMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerReleasingLevelMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERPLAYERRELEASINGLEVELMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPlayerEnteredLevelMessage {
}

pub const SERVERPLAYERENTEREDLEVELMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerEnteredLevelMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerEnteredLevelMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERPLAYERENTEREDLEVELMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPlayerLevelLoadedMessage {
}

pub const SERVERPLAYERLEVELLOADEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerLevelLoadedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerLevelLoadedMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERPLAYERLEVELLOADEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPlayerDebugFriendZoneSpawnMessage {
}

pub const SERVERPLAYERDEBUGFRIENDZONESPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerDebugFriendZoneSpawnMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ServerPlayerDebugFriendZoneSpawnMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERPLAYERDEBUGFRIENDZONESPAWNMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPlayerRespawnMessage {
}

pub const SERVERPLAYERRESPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerRespawnMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerRespawnMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERPLAYERRESPAWNMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPlayerDestroyMessage {
}

pub const SERVERPLAYERDESTROYMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerDestroyMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerDestroyMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERPLAYERDESTROYMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPlayerCreatedForConnectionMessage {
}

pub const SERVERPLAYERCREATEDFORCONNECTIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerCreatedForConnectionMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerCreatedForConnectionMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERPLAYERCREATEDFORCONNECTIONMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPlayerCreateMessage {
}

pub const SERVERPLAYERCREATEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerCreateMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerCreateMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERPLAYERCREATEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPlayerAboutToCreateForConnectionMessage {
}

pub const SERVERPLAYERABOUTTOCREATEFORCONNECTIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerAboutToCreateForConnectionMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerAboutToCreateForConnectionMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERPLAYERABOUTTOCREATEFORCONNECTIONMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCharacterCharacterDamageMessage {
}

pub const SERVERCHARACTERCHARACTERDAMAGEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterCharacterDamageMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerCharacterCharacterDamageMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERCHARACTERCHARACTERDAMAGEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCharacterKilledMessage {
}

pub const SERVERCHARACTERKILLEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterKilledMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerCharacterKilledMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERCHARACTERKILLEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerMetricsDetonateExplosionMessage {
}

pub const SERVERMETRICSDETONATEEXPLOSIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMetricsDetonateExplosionMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ServerMetricsDetonateExplosionMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERMETRICSDETONATEEXPLOSIONMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerMetricsObjectiveSuccessMessage {
}

pub const SERVERMETRICSOBJECTIVESUCCESSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMetricsObjectiveSuccessMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerMetricsObjectiveSuccessMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERMETRICSOBJECTIVESUCCESSMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerMetricsSaveGameSavedMessage {
}

pub const SERVERMETRICSSAVEGAMESAVEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMetricsSaveGameSavedMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerMetricsSaveGameSavedMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERMETRICSSAVEGAMESAVEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerMetricsSaveGameLoadedMessage {
}

pub const SERVERMETRICSSAVEGAMELOADEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMetricsSaveGameLoadedMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerMetricsSaveGameLoadedMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERMETRICSSAVEGAMELOADEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerWaterEntity {
}

pub const SERVERWATERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWaterEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERPHYSICSENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERWATERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerWaterEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERWATERENTITY_TYPE_INFO
    }
}


pub const SERVERWATERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWaterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerWaterEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerTerrainEntity {
}

pub const SERVERTERRAINENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTerrainEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERPHYSICSENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERTERRAINENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerTerrainEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERTERRAINENTITY_TYPE_INFO
    }
}


pub const SERVERTERRAINENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTerrainEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerTerrainEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerTerrainDynamicDecalEntity {
}

pub const SERVERTERRAINDYNAMICDECALENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTerrainDynamicDecalEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERTERRAINDYNAMICDECALENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerTerrainDynamicDecalEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERTERRAINDYNAMICDECALENTITY_TYPE_INFO
    }
}


pub const SERVERTERRAINDYNAMICDECALENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTerrainDynamicDecalEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerTerrainDynamicDecalEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerStaticModelGroupEntity {
}

pub const SERVERSTATICMODELGROUPENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticModelGroupEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERPHYSICSENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSTATICMODELGROUPENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerStaticModelGroupEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERSTATICMODELGROUPENTITY_TYPE_INFO
    }
}


pub const SERVERSTATICMODELGROUPENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticModelGroupEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerStaticModelGroupEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerStaticModelEntity {
}

pub const SERVERSTATICMODELENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticModelEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERPHYSICSENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSTATICMODELENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerStaticModelEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERSTATICMODELENTITY_TYPE_INFO
    }
}


pub const SERVERSTATICMODELENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticModelEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerStaticModelEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerLadderEntity {
}

pub const SERVERLADDERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLadderEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERSTATICMODELENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERLADDERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerLadderEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERLADDERENTITY_TYPE_INFO
    }
}


pub const SERVERLADDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLadderEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerLadderEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerInteractableStaticModelEntity {
}

pub const SERVERINTERACTABLESTATICMODELENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerInteractableStaticModelEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERSTATICMODELENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERINTERACTABLESTATICMODELENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerInteractableStaticModelEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERINTERACTABLESTATICMODELENTITY_TYPE_INFO
    }
}


pub const SERVERINTERACTABLESTATICMODELENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerInteractableStaticModelEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerInteractableStaticModelEntity-Array"),
    array_type: None,
    alignment: 8,
};


