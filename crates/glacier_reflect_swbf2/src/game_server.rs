use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerBangerEntity {
    pub _glacier_base: ServerPhysicsEntity,
}

pub trait ServerBangerEntityTrait: ServerPhysicsEntityTrait {
}

impl ServerBangerEntityTrait for ServerBangerEntity {
}

impl ServerPhysicsEntityTrait for ServerBangerEntity {
}

impl ServerGameComponentEntityTrait for ServerBangerEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ServerBangerEntity {
}

impl super::entity::ComponentEntityTrait for ServerBangerEntity {
}

impl super::entity::SpatialEntityTrait for ServerBangerEntity {
}

impl super::entity::EntityTrait for ServerBangerEntity {
}

impl super::entity::EntityBusPeerTrait for ServerBangerEntity {
}

pub static SERVERBANGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBangerEntity",
    name_hash: 177069094,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERPHYSICSENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerBangerEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerBangerEntity as Default>::default())),
            create_boxed: || Box::new(<ServerBangerEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERBANGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerBangerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERBANGERENTITY_TYPE_INFO
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


pub static SERVERBANGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBangerEntity-Array",
    name_hash: 1060702354,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerBangerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerWaterPhysicsComponent {
    pub _glacier_base: super::physics::PhysicsComponent,
}

pub trait ServerWaterPhysicsComponentTrait: super::physics::PhysicsComponentTrait {
}

impl ServerWaterPhysicsComponentTrait for ServerWaterPhysicsComponent {
}

impl super::physics::PhysicsComponentTrait for ServerWaterPhysicsComponent {
}

impl super::entity::ComponentTrait for ServerWaterPhysicsComponent {
}

impl super::entity::EntityBusPeerTrait for ServerWaterPhysicsComponent {
}

pub static SERVERWATERPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWaterPhysicsComponent",
    name_hash: 608461297,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::PHYSICSCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerWaterPhysicsComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerWaterPhysicsComponent as Default>::default())),
            create_boxed: || Box::new(<ServerWaterPhysicsComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERWATERPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerWaterPhysicsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERWATERPHYSICSCOMPONENT_TYPE_INFO
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


pub static SERVERWATERPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWaterPhysicsComponent-Array",
    name_hash: 4176413893,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerWaterPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerWaterHealthComponent {
    pub _glacier_base: ServerGameHealthComponent,
}

pub trait ServerWaterHealthComponentTrait: ServerGameHealthComponentTrait {
}

impl ServerWaterHealthComponentTrait for ServerWaterHealthComponent {
}

impl ServerGameHealthComponentTrait for ServerWaterHealthComponent {
}

impl super::gameplay_sim::HealthComponentTrait for ServerWaterHealthComponent {
}

impl super::entity::ComponentTrait for ServerWaterHealthComponent {
}

impl super::entity::EntityBusPeerTrait for ServerWaterHealthComponent {
}

pub static SERVERWATERHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWaterHealthComponent",
    name_hash: 2660830950,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMEHEALTHCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerWaterHealthComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerWaterHealthComponent as Default>::default())),
            create_boxed: || Box::new(<ServerWaterHealthComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERWATERHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerWaterHealthComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERWATERHEALTHCOMPONENT_TYPE_INFO
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


pub static SERVERWATERHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWaterHealthComponent-Array",
    name_hash: 29297618,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerWaterHealthComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerTerrainPhysicsComponent {
    pub _glacier_base: super::physics::PhysicsComponent,
}

pub trait ServerTerrainPhysicsComponentTrait: super::physics::PhysicsComponentTrait {
}

impl ServerTerrainPhysicsComponentTrait for ServerTerrainPhysicsComponent {
}

impl super::physics::PhysicsComponentTrait for ServerTerrainPhysicsComponent {
}

impl super::entity::ComponentTrait for ServerTerrainPhysicsComponent {
}

impl super::entity::EntityBusPeerTrait for ServerTerrainPhysicsComponent {
}

pub static SERVERTERRAINPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTerrainPhysicsComponent",
    name_hash: 1322126067,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::PHYSICSCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerTerrainPhysicsComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerTerrainPhysicsComponent as Default>::default())),
            create_boxed: || Box::new(<ServerTerrainPhysicsComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERTERRAINPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerTerrainPhysicsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERTERRAINPHYSICSCOMPONENT_TYPE_INFO
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


pub static SERVERTERRAINPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTerrainPhysicsComponent-Array",
    name_hash: 4060063431,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerTerrainPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerTerrainHealthComponent {
    pub _glacier_base: ServerGameHealthComponent,
}

pub trait ServerTerrainHealthComponentTrait: ServerGameHealthComponentTrait {
}

impl ServerTerrainHealthComponentTrait for ServerTerrainHealthComponent {
}

impl ServerGameHealthComponentTrait for ServerTerrainHealthComponent {
}

impl super::gameplay_sim::HealthComponentTrait for ServerTerrainHealthComponent {
}

impl super::entity::ComponentTrait for ServerTerrainHealthComponent {
}

impl super::entity::EntityBusPeerTrait for ServerTerrainHealthComponent {
}

pub static SERVERTERRAINHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTerrainHealthComponent",
    name_hash: 787148836,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMEHEALTHCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerTerrainHealthComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerTerrainHealthComponent as Default>::default())),
            create_boxed: || Box::new(<ServerTerrainHealthComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERTERRAINHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerTerrainHealthComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERTERRAINHEALTHCOMPONENT_TYPE_INFO
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


pub static SERVERTERRAINHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTerrainHealthComponent-Array",
    name_hash: 1742513040,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerTerrainHealthComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerStaticModelPhysicsComponent {
    pub _glacier_base: super::physics::PartPhysicsComponent,
}

pub trait ServerStaticModelPhysicsComponentTrait: super::physics::PartPhysicsComponentTrait {
}

impl ServerStaticModelPhysicsComponentTrait for ServerStaticModelPhysicsComponent {
}

impl super::physics::PartPhysicsComponentTrait for ServerStaticModelPhysicsComponent {
}

impl super::physics::PhysicsComponentTrait for ServerStaticModelPhysicsComponent {
}

impl super::entity::ComponentTrait for ServerStaticModelPhysicsComponent {
}

impl super::entity::EntityBusPeerTrait for ServerStaticModelPhysicsComponent {
}

pub static SERVERSTATICMODELPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticModelPhysicsComponent",
    name_hash: 3097834163,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::PARTPHYSICSCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerStaticModelPhysicsComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerStaticModelPhysicsComponent as Default>::default())),
            create_boxed: || Box::new(<ServerStaticModelPhysicsComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSTATICMODELPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerStaticModelPhysicsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSTATICMODELPHYSICSCOMPONENT_TYPE_INFO
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


pub static SERVERSTATICMODELPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticModelPhysicsComponent-Array",
    name_hash: 4081163783,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerStaticModelPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerStaticModelHealthComponent {
    pub _glacier_base: ServerGameHealthComponent,
}

pub trait ServerStaticModelHealthComponentTrait: ServerGameHealthComponentTrait {
}

impl ServerStaticModelHealthComponentTrait for ServerStaticModelHealthComponent {
}

impl ServerGameHealthComponentTrait for ServerStaticModelHealthComponent {
}

impl super::gameplay_sim::HealthComponentTrait for ServerStaticModelHealthComponent {
}

impl super::entity::ComponentTrait for ServerStaticModelHealthComponent {
}

impl super::entity::EntityBusPeerTrait for ServerStaticModelHealthComponent {
}

pub static SERVERSTATICMODELHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticModelHealthComponent",
    name_hash: 3647615332,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMEHEALTHCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerStaticModelHealthComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerStaticModelHealthComponent as Default>::default())),
            create_boxed: || Box::new(<ServerStaticModelHealthComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSTATICMODELHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerStaticModelHealthComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSTATICMODELHEALTHCOMPONENT_TYPE_INFO
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


pub static SERVERSTATICMODELHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticModelHealthComponent-Array",
    name_hash: 1960039760,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerStaticModelHealthComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerStaticModelGroupPhysicsComponent {
    pub _glacier_base: super::physics::GroupPhysicsComponent,
}

pub trait ServerStaticModelGroupPhysicsComponentTrait: super::physics::GroupPhysicsComponentTrait {
}

impl ServerStaticModelGroupPhysicsComponentTrait for ServerStaticModelGroupPhysicsComponent {
}

impl super::physics::GroupPhysicsComponentTrait for ServerStaticModelGroupPhysicsComponent {
}

impl super::physics::PhysicsComponentTrait for ServerStaticModelGroupPhysicsComponent {
}

impl super::entity::ComponentTrait for ServerStaticModelGroupPhysicsComponent {
}

impl super::entity::EntityBusPeerTrait for ServerStaticModelGroupPhysicsComponent {
}

pub static SERVERSTATICMODELGROUPPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticModelGroupPhysicsComponent",
    name_hash: 2546038252,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::GROUPPHYSICSCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerStaticModelGroupPhysicsComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerStaticModelGroupPhysicsComponent as Default>::default())),
            create_boxed: || Box::new(<ServerStaticModelGroupPhysicsComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSTATICMODELGROUPPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerStaticModelGroupPhysicsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSTATICMODELGROUPPHYSICSCOMPONENT_TYPE_INFO
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


pub static SERVERSTATICMODELGROUPPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticModelGroupPhysicsComponent-Array",
    name_hash: 2418791896,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerStaticModelGroupPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerStaticModelGroupHealthComponent {
    pub _glacier_base: ServerGameHealthComponent,
}

pub trait ServerStaticModelGroupHealthComponentTrait: ServerGameHealthComponentTrait {
}

impl ServerStaticModelGroupHealthComponentTrait for ServerStaticModelGroupHealthComponent {
}

impl ServerGameHealthComponentTrait for ServerStaticModelGroupHealthComponent {
}

impl super::gameplay_sim::HealthComponentTrait for ServerStaticModelGroupHealthComponent {
}

impl super::entity::ComponentTrait for ServerStaticModelGroupHealthComponent {
}

impl super::entity::EntityBusPeerTrait for ServerStaticModelGroupHealthComponent {
}

pub static SERVERSTATICMODELGROUPHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticModelGroupHealthComponent",
    name_hash: 3960975067,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMEHEALTHCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerStaticModelGroupHealthComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerStaticModelGroupHealthComponent as Default>::default())),
            create_boxed: || Box::new(<ServerStaticModelGroupHealthComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSTATICMODELGROUPHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerStaticModelGroupHealthComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSTATICMODELGROUPHEALTHCOMPONENT_TYPE_INFO
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


pub static SERVERSTATICMODELGROUPHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticModelGroupHealthComponent-Array",
    name_hash: 4122187247,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerStaticModelGroupHealthComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerPartComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerPartComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerPartComponentTrait for ServerPartComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerPartComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerPartComponent {
}

impl super::entity::ComponentTrait for ServerPartComponent {
}

impl super::entity::EntityBusPeerTrait for ServerPartComponent {
}

pub static SERVERPARTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPartComponent",
    name_hash: 1134549912,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerPartComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPartComponent as Default>::default())),
            create_boxed: || Box::new(<ServerPartComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERPARTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPartComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPARTCOMPONENT_TYPE_INFO
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


pub static SERVERPARTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPartComponent-Array",
    name_hash: 1151813036,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerPartComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerPart {
    pub _glacier_base: super::entity::Part,
}

pub trait ServerPartTrait: super::entity::PartTrait {
}

impl ServerPartTrait for ServerPart {
}

impl super::entity::PartTrait for ServerPart {
}

impl super::entity::ComponentTrait for ServerPart {
}

impl super::entity::EntityBusPeerTrait for ServerPart {
}

pub static SERVERPART_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPart",
    name_hash: 1802184791,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::PART_TYPE_INFO),
        super_class_offset: offset_of!(ServerPart, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPart as Default>::default())),
            create_boxed: || Box::new(<ServerPart as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERPART_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPart {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPART_TYPE_INFO
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


pub static SERVERPART_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPart-Array",
    name_hash: 165445987,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerPart"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerGameHealthComponent {
    pub _glacier_base: super::gameplay_sim::HealthComponent,
}

pub trait ServerGameHealthComponentTrait: super::gameplay_sim::HealthComponentTrait {
}

impl ServerGameHealthComponentTrait for ServerGameHealthComponent {
}

impl super::gameplay_sim::HealthComponentTrait for ServerGameHealthComponent {
}

impl super::entity::ComponentTrait for ServerGameHealthComponent {
}

impl super::entity::EntityBusPeerTrait for ServerGameHealthComponent {
}

pub static SERVERGAMEHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameHealthComponent",
    name_hash: 2909977021,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_sim::HEALTHCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerGameHealthComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGameHealthComponent as Default>::default())),
            create_boxed: || Box::new(<ServerGameHealthComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERGAMEHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerGameHealthComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGAMEHEALTHCOMPONENT_TYPE_INFO
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


pub static SERVERGAMEHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameHealthComponent-Array",
    name_hash: 1298069001,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerGameHealthComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerBoneComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerBoneComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerBoneComponentTrait for ServerBoneComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerBoneComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerBoneComponent {
}

impl super::entity::ComponentTrait for ServerBoneComponent {
}

impl super::entity::EntityBusPeerTrait for ServerBoneComponent {
}

pub static SERVERBONECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBoneComponent",
    name_hash: 223276137,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerBoneComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerBoneComponent as Default>::default())),
            create_boxed: || Box::new(<ServerBoneComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERBONECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerBoneComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERBONECOMPONENT_TYPE_INFO
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


pub static SERVERBONECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBoneComponent-Array",
    name_hash: 3594781533,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerBoneComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerPhysicsEntity {
    pub _glacier_base: ServerGameComponentEntity,
}

pub trait ServerPhysicsEntityTrait: ServerGameComponentEntityTrait {
}

impl ServerPhysicsEntityTrait for ServerPhysicsEntity {
}

impl ServerGameComponentEntityTrait for ServerPhysicsEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ServerPhysicsEntity {
}

impl super::entity::ComponentEntityTrait for ServerPhysicsEntity {
}

impl super::entity::SpatialEntityTrait for ServerPhysicsEntity {
}

impl super::entity::EntityTrait for ServerPhysicsEntity {
}

impl super::entity::EntityBusPeerTrait for ServerPhysicsEntity {
}

pub static SERVERPHYSICSENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPhysicsEntity",
    name_hash: 4084903408,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENTENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerPhysicsEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPhysicsEntity as Default>::default())),
            create_boxed: || Box::new(<ServerPhysicsEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERPHYSICSENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPhysicsEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPHYSICSENTITY_TYPE_INFO
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


pub static SERVERPHYSICSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPhysicsEntity-Array",
    name_hash: 1002621892,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerPhysicsEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerLocalPlayerGateEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerLocalPlayerGateEntityTrait: super::entity::EntityTrait {
}

impl ServerLocalPlayerGateEntityTrait for ServerLocalPlayerGateEntity {
}

impl super::entity::EntityTrait for ServerLocalPlayerGateEntity {
}

impl super::entity::EntityBusPeerTrait for ServerLocalPlayerGateEntity {
}

pub static SERVERLOCALPLAYERGATEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLocalPlayerGateEntity",
    name_hash: 3474516626,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerLocalPlayerGateEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerLocalPlayerGateEntity as Default>::default())),
            create_boxed: || Box::new(<ServerLocalPlayerGateEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERLOCALPLAYERGATEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerLocalPlayerGateEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERLOCALPLAYERGATEENTITY_TYPE_INFO
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


pub static SERVERLOCALPLAYERGATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLocalPlayerGateEntity-Array",
    name_hash: 3348918694,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerLocalPlayerGateEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerGroupComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerGroupComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerGroupComponentTrait for ServerGroupComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerGroupComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerGroupComponent {
}

impl super::entity::ComponentTrait for ServerGroupComponent {
}

impl super::entity::EntityBusPeerTrait for ServerGroupComponent {
}

pub static SERVERGROUPCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGroupComponent",
    name_hash: 986700368,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerGroupComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGroupComponent as Default>::default())),
            create_boxed: || Box::new(<ServerGroupComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERGROUPCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerGroupComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGROUPCOMPONENT_TYPE_INFO
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


pub static SERVERGROUPCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGroupComponent-Array",
    name_hash: 1833957604,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerGroupComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerGhostEntityOwnerWrapperEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerGhostEntityOwnerWrapperEntityTrait: super::entity::EntityTrait {
}

impl ServerGhostEntityOwnerWrapperEntityTrait for ServerGhostEntityOwnerWrapperEntity {
}

impl super::entity::EntityTrait for ServerGhostEntityOwnerWrapperEntity {
}

impl super::entity::EntityBusPeerTrait for ServerGhostEntityOwnerWrapperEntity {
}

pub static SERVERGHOSTENTITYOWNERWRAPPERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGhostEntityOwnerWrapperEntity",
    name_hash: 3919508053,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerGhostEntityOwnerWrapperEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGhostEntityOwnerWrapperEntity as Default>::default())),
            create_boxed: || Box::new(<ServerGhostEntityOwnerWrapperEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERGHOSTENTITYOWNERWRAPPERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerGhostEntityOwnerWrapperEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGHOSTENTITYOWNERWRAPPERENTITY_TYPE_INFO
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


pub static SERVERGHOSTENTITYOWNERWRAPPERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGhostEntityOwnerWrapperEntity-Array",
    name_hash: 4237529697,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerGhostEntityOwnerWrapperEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerGameComponentEntity {
    pub _glacier_base: super::gameplay_sim::GameComponentEntity,
}

pub trait ServerGameComponentEntityTrait: super::gameplay_sim::GameComponentEntityTrait {
}

impl ServerGameComponentEntityTrait for ServerGameComponentEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ServerGameComponentEntity {
}

impl super::entity::ComponentEntityTrait for ServerGameComponentEntity {
}

impl super::entity::SpatialEntityTrait for ServerGameComponentEntity {
}

impl super::entity::EntityTrait for ServerGameComponentEntity {
}

impl super::entity::EntityBusPeerTrait for ServerGameComponentEntity {
}

pub static SERVERGAMECOMPONENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameComponentEntity",
    name_hash: 4125047994,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_sim::GAMECOMPONENTENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerGameComponentEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGameComponentEntity as Default>::default())),
            create_boxed: || Box::new(<ServerGameComponentEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERGAMECOMPONENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerGameComponentEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGAMECOMPONENTENTITY_TYPE_INFO
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


pub static SERVERGAMECOMPONENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameComponentEntity-Array",
    name_hash: 3892526862,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerGameComponentEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerEventSyncEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerEventSyncEntityTrait: super::entity::EntityTrait {
}

impl ServerEventSyncEntityTrait for ServerEventSyncEntity {
}

impl super::entity::EntityTrait for ServerEventSyncEntity {
}

impl super::entity::EntityBusPeerTrait for ServerEventSyncEntity {
}

pub static SERVEREVENTSYNCENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEventSyncEntity",
    name_hash: 3529219056,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerEventSyncEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerEventSyncEntity as Default>::default())),
            create_boxed: || Box::new(<ServerEventSyncEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVEREVENTSYNCENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerEventSyncEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVEREVENTSYNCENTITY_TYPE_INFO
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


pub static SERVEREVENTSYNCENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEventSyncEntity-Array",
    name_hash: 1730034116,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerEventSyncEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerPlaceHolderEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerPlaceHolderEntityTrait: super::entity::EntityTrait {
}

impl ServerPlaceHolderEntityTrait for ServerPlaceHolderEntity {
}

impl super::entity::EntityTrait for ServerPlaceHolderEntity {
}

impl super::entity::EntityBusPeerTrait for ServerPlaceHolderEntity {
}

pub static SERVERPLACEHOLDERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlaceHolderEntity",
    name_hash: 1188940952,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerPlaceHolderEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPlaceHolderEntity as Default>::default())),
            create_boxed: || Box::new(<ServerPlaceHolderEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERPLACEHOLDERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPlaceHolderEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPLACEHOLDERENTITY_TYPE_INFO
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


pub static SERVERPLACEHOLDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlaceHolderEntity-Array",
    name_hash: 2497050284,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerPlaceHolderEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerBlueprintEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerBlueprintEntityTrait: super::entity::EntityTrait {
}

impl ServerBlueprintEntityTrait for ServerBlueprintEntity {
}

impl super::entity::EntityTrait for ServerBlueprintEntity {
}

impl super::entity::EntityBusPeerTrait for ServerBlueprintEntity {
}

pub static SERVERBLUEPRINTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBlueprintEntity",
    name_hash: 3251184916,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerBlueprintEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerBlueprintEntity as Default>::default())),
            create_boxed: || Box::new(<ServerBlueprintEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERBLUEPRINTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerBlueprintEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERBLUEPRINTENTITY_TYPE_INFO
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


pub static SERVERBLUEPRINTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBlueprintEntity-Array",
    name_hash: 3032613152,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerBlueprintEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerRecordRootTransformTrack {
    pub _glacier_base: super::timeline::RecordTrackBase,
}

pub trait ServerRecordRootTransformTrackTrait: super::timeline::RecordTrackBaseTrait {
}

impl ServerRecordRootTransformTrackTrait for ServerRecordRootTransformTrack {
}

impl super::timeline::RecordTrackBaseTrait for ServerRecordRootTransformTrack {
}

impl super::timeline::LinkTrackTrait for ServerRecordRootTransformTrack {
}

impl super::timeline::TimelineTrackTrait for ServerRecordRootTransformTrack {
}

pub static SERVERRECORDROOTTRANSFORMTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerRecordRootTransformTrack",
    name_hash: 3406459144,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::RECORDTRACKBASE_TYPE_INFO),
        super_class_offset: offset_of!(ServerRecordRootTransformTrack, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerRecordRootTransformTrack as Default>::default())),
            create_boxed: || Box::new(<ServerRecordRootTransformTrack as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERRECORDROOTTRANSFORMTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerRecordRootTransformTrack {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERRECORDROOTTRANSFORMTRACK_TYPE_INFO
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


pub static SERVERRECORDROOTTRANSFORMTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerRecordRootTransformTrack-Array",
    name_hash: 2256089916,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerRecordRootTransformTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerRecordEntryInputTrack {
    pub _glacier_base: super::timeline::RecordTrackBase,
}

pub trait ServerRecordEntryInputTrackTrait: super::timeline::RecordTrackBaseTrait {
}

impl ServerRecordEntryInputTrackTrait for ServerRecordEntryInputTrack {
}

impl super::timeline::RecordTrackBaseTrait for ServerRecordEntryInputTrack {
}

impl super::timeline::LinkTrackTrait for ServerRecordEntryInputTrack {
}

impl super::timeline::TimelineTrackTrait for ServerRecordEntryInputTrack {
}

pub static SERVERRECORDENTRYINPUTTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerRecordEntryInputTrack",
    name_hash: 92384704,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::RECORDTRACKBASE_TYPE_INFO),
        super_class_offset: offset_of!(ServerRecordEntryInputTrack, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerRecordEntryInputTrack as Default>::default())),
            create_boxed: || Box::new(<ServerRecordEntryInputTrack as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERRECORDENTRYINPUTTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerRecordEntryInputTrack {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERRECORDENTRYINPUTTRACK_TYPE_INFO
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


pub static SERVERRECORDENTRYINPUTTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerRecordEntryInputTrack-Array",
    name_hash: 2791285876,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerRecordEntryInputTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerPlayAnimationEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerPlayAnimationEntityTrait: super::entity::EntityTrait {
}

impl ServerPlayAnimationEntityTrait for ServerPlayAnimationEntity {
}

impl super::entity::EntityTrait for ServerPlayAnimationEntity {
}

impl super::entity::EntityBusPeerTrait for ServerPlayAnimationEntity {
}

pub static SERVERPLAYANIMATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayAnimationEntity",
    name_hash: 384935657,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerPlayAnimationEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPlayAnimationEntity as Default>::default())),
            create_boxed: || Box::new(<ServerPlayAnimationEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERPLAYANIMATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPlayAnimationEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPLAYANIMATIONENTITY_TYPE_INFO
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


pub static SERVERPLAYANIMATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayAnimationEntity-Array",
    name_hash: 1588108765,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerPlayAnimationEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerMultipleActorScenarioEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerMultipleActorScenarioEntityTrait: super::entity::EntityTrait {
}

impl ServerMultipleActorScenarioEntityTrait for ServerMultipleActorScenarioEntity {
}

impl super::entity::EntityTrait for ServerMultipleActorScenarioEntity {
}

impl super::entity::EntityBusPeerTrait for ServerMultipleActorScenarioEntity {
}

pub static SERVERMULTIPLEACTORSCENARIOENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMultipleActorScenarioEntity",
    name_hash: 1551030446,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerMultipleActorScenarioEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerMultipleActorScenarioEntity as Default>::default())),
            create_boxed: || Box::new(<ServerMultipleActorScenarioEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERMULTIPLEACTORSCENARIOENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerMultipleActorScenarioEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERMULTIPLEACTORSCENARIOENTITY_TYPE_INFO
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


pub static SERVERMULTIPLEACTORSCENARIOENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMultipleActorScenarioEntity-Array",
    name_hash: 2139668762,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerMultipleActorScenarioEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerModelAnimationEntity {
    pub _glacier_base: super::game_common::ModelAnimationEntity,
}

pub trait ServerModelAnimationEntityTrait: super::game_common::ModelAnimationEntityTrait {
}

impl ServerModelAnimationEntityTrait for ServerModelAnimationEntity {
}

impl super::game_common::ModelAnimationEntityTrait for ServerModelAnimationEntity {
}

impl super::entity::EntityTrait for ServerModelAnimationEntity {
}

impl super::entity::EntityBusPeerTrait for ServerModelAnimationEntity {
}

pub static SERVERMODELANIMATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerModelAnimationEntity",
    name_hash: 3810902338,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_common::MODELANIMATIONENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerModelAnimationEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerModelAnimationEntity as Default>::default())),
            create_boxed: || Box::new(<ServerModelAnimationEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERMODELANIMATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerModelAnimationEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERMODELANIMATIONENTITY_TYPE_INFO
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


pub static SERVERMODELANIMATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerModelAnimationEntity-Array",
    name_hash: 3638350070,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerModelAnimationEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerFbProxyControllerEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerFbProxyControllerEntityTrait: super::entity::EntityTrait {
}

impl ServerFbProxyControllerEntityTrait for ServerFbProxyControllerEntity {
}

impl super::entity::EntityTrait for ServerFbProxyControllerEntity {
}

impl super::entity::EntityBusPeerTrait for ServerFbProxyControllerEntity {
}

pub static SERVERFBPROXYCONTROLLERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerFbProxyControllerEntity",
    name_hash: 1632609647,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerFbProxyControllerEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerFbProxyControllerEntity as Default>::default())),
            create_boxed: || Box::new(<ServerFbProxyControllerEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERFBPROXYCONTROLLERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerFbProxyControllerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERFBPROXYCONTROLLERENTITY_TYPE_INFO
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


pub static SERVERFBPROXYCONTROLLERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerFbProxyControllerEntity-Array",
    name_hash: 3138872155,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerFbProxyControllerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerCharacterInVehicleScenarioEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerCharacterInVehicleScenarioEntityTrait: super::entity::EntityTrait {
}

impl ServerCharacterInVehicleScenarioEntityTrait for ServerCharacterInVehicleScenarioEntity {
}

impl super::entity::EntityTrait for ServerCharacterInVehicleScenarioEntity {
}

impl super::entity::EntityBusPeerTrait for ServerCharacterInVehicleScenarioEntity {
}

pub static SERVERCHARACTERINVEHICLESCENARIOENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterInVehicleScenarioEntity",
    name_hash: 1186067,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerCharacterInVehicleScenarioEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCharacterInVehicleScenarioEntity as Default>::default())),
            create_boxed: || Box::new(<ServerCharacterInVehicleScenarioEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCHARACTERINVEHICLESCENARIOENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCharacterInVehicleScenarioEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCHARACTERINVEHICLESCENARIOENTITY_TYPE_INFO
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


pub static SERVERCHARACTERINVEHICLESCENARIOENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterInVehicleScenarioEntity-Array",
    name_hash: 1068864423,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerCharacterInVehicleScenarioEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerANTSignalTrack {
    pub _glacier_base: super::game_common::ANTSignalTrack,
}

pub trait ServerANTSignalTrackTrait: super::game_common::ANTSignalTrackTrait {
}

impl ServerANTSignalTrackTrait for ServerANTSignalTrack {
}

impl super::game_common::ANTSignalTrackTrait for ServerANTSignalTrack {
}

impl super::timeline::LinkTrackTrait for ServerANTSignalTrack {
}

impl super::timeline::TimelineTrackTrait for ServerANTSignalTrack {
}

pub static SERVERANTSIGNALTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerANTSignalTrack",
    name_hash: 3176010346,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_common::ANTSIGNALTRACK_TYPE_INFO),
        super_class_offset: offset_of!(ServerANTSignalTrack, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerANTSignalTrack as Default>::default())),
            create_boxed: || Box::new(<ServerANTSignalTrack as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERANTSIGNALTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerANTSignalTrack {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERANTSIGNALTRACK_TYPE_INFO
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


pub static SERVERANTSIGNALTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerANTSignalTrack-Array",
    name_hash: 558390622,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerANTSignalTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerWriteAntGameStateEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerWriteAntGameStateEntityTrait: super::entity::EntityTrait {
}

impl ServerWriteAntGameStateEntityTrait for ServerWriteAntGameStateEntity {
}

impl super::entity::EntityTrait for ServerWriteAntGameStateEntity {
}

impl super::entity::EntityBusPeerTrait for ServerWriteAntGameStateEntity {
}

pub static SERVERWRITEANTGAMESTATEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWriteAntGameStateEntity",
    name_hash: 1780553444,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerWriteAntGameStateEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerWriteAntGameStateEntity as Default>::default())),
            create_boxed: || Box::new(<ServerWriteAntGameStateEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERWRITEANTGAMESTATEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerWriteAntGameStateEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERWRITEANTGAMESTATEENTITY_TYPE_INFO
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


pub static SERVERWRITEANTGAMESTATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWriteAntGameStateEntity-Array",
    name_hash: 2340477648,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerWriteAntGameStateEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerReadAntGameStateEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerReadAntGameStateEntityTrait: super::entity::EntityTrait {
}

impl ServerReadAntGameStateEntityTrait for ServerReadAntGameStateEntity {
}

impl super::entity::EntityTrait for ServerReadAntGameStateEntity {
}

impl super::entity::EntityBusPeerTrait for ServerReadAntGameStateEntity {
}

pub static SERVERREADANTGAMESTATEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerReadAntGameStateEntity",
    name_hash: 2015098155,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerReadAntGameStateEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerReadAntGameStateEntity as Default>::default())),
            create_boxed: || Box::new(<ServerReadAntGameStateEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERREADANTGAMESTATEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerReadAntGameStateEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERREADANTGAMESTATEENTITY_TYPE_INFO
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


pub static SERVERREADANTGAMESTATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerReadAntGameStateEntity-Array",
    name_hash: 2236404895,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerReadAntGameStateEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerANTControlTrack {
    pub _glacier_base: super::game_common::ANTControlTrack,
}

pub trait ServerANTControlTrackTrait: super::game_common::ANTControlTrackTrait {
}

impl ServerANTControlTrackTrait for ServerANTControlTrack {
}

impl super::game_common::ANTControlTrackTrait for ServerANTControlTrack {
}

impl super::timeline::LinkTrackTrait for ServerANTControlTrack {
}

impl super::timeline::TimelineTrackTrait for ServerANTControlTrack {
}

pub static SERVERANTCONTROLTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerANTControlTrack",
    name_hash: 2633114611,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_common::ANTCONTROLTRACK_TYPE_INFO),
        super_class_offset: offset_of!(ServerANTControlTrack, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerANTControlTrack as Default>::default())),
            create_boxed: || Box::new(<ServerANTControlTrack as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERANTCONTROLTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerANTControlTrack {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERANTCONTROLTRACK_TYPE_INFO
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


pub static SERVERANTCONTROLTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerANTControlTrack-Array",
    name_hash: 3138551751,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerANTControlTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerAntAnimatableEntity {
    pub _glacier_base: super::gameplay_sim::AntAnimatableEntity,
}

pub trait ServerAntAnimatableEntityTrait: super::gameplay_sim::AntAnimatableEntityTrait {
}

impl ServerAntAnimatableEntityTrait for ServerAntAnimatableEntity {
}

impl super::gameplay_sim::AntAnimatableEntityTrait for ServerAntAnimatableEntity {
}

impl super::entity::EntityTrait for ServerAntAnimatableEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAntAnimatableEntity {
}

pub static SERVERANTANIMATABLEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAntAnimatableEntity",
    name_hash: 2420928852,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_sim::ANTANIMATABLEENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAntAnimatableEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAntAnimatableEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAntAnimatableEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERANTANIMATABLEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAntAnimatableEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERANTANIMATABLEENTITY_TYPE_INFO
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


pub static SERVERANTANIMATABLEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAntAnimatableEntity-Array",
    name_hash: 2866904544,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerAntAnimatableEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerAnimationPoseTrack {
    pub _glacier_base: super::game_common::AnimationPoseTrack,
}

pub trait ServerAnimationPoseTrackTrait: super::game_common::AnimationPoseTrackTrait {
}

impl ServerAnimationPoseTrackTrait for ServerAnimationPoseTrack {
}

impl super::game_common::AnimationPoseTrackTrait for ServerAnimationPoseTrack {
}

impl super::timeline::LinkTrackTrait for ServerAnimationPoseTrack {
}

impl super::timeline::TimelineTrackTrait for ServerAnimationPoseTrack {
}

pub static SERVERANIMATIONPOSETRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAnimationPoseTrack",
    name_hash: 423212208,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_common::ANIMATIONPOSETRACK_TYPE_INFO),
        super_class_offset: offset_of!(ServerAnimationPoseTrack, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAnimationPoseTrack as Default>::default())),
            create_boxed: || Box::new(<ServerAnimationPoseTrack as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERANIMATIONPOSETRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAnimationPoseTrack {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERANIMATIONPOSETRACK_TYPE_INFO
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


pub static SERVERANIMATIONPOSETRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAnimationPoseTrack-Array",
    name_hash: 379576836,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerAnimationPoseTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerAnimationEnumerationChoiceEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerAnimationEnumerationChoiceEntityTrait: super::entity::EntityTrait {
}

impl ServerAnimationEnumerationChoiceEntityTrait for ServerAnimationEnumerationChoiceEntity {
}

impl super::entity::EntityTrait for ServerAnimationEnumerationChoiceEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAnimationEnumerationChoiceEntity {
}

pub static SERVERANIMATIONENUMERATIONCHOICEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAnimationEnumerationChoiceEntity",
    name_hash: 169723903,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAnimationEnumerationChoiceEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAnimationEnumerationChoiceEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAnimationEnumerationChoiceEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERANIMATIONENUMERATIONCHOICEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAnimationEnumerationChoiceEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERANIMATIONENUMERATIONCHOICEENTITY_TYPE_INFO
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


pub static SERVERANIMATIONENUMERATIONCHOICEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAnimationEnumerationChoiceEntity-Array",
    name_hash: 3475516875,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerAnimationEnumerationChoiceEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerAnimationEnumerationEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerAnimationEnumerationEntityTrait: super::entity::EntityTrait {
}

impl ServerAnimationEnumerationEntityTrait for ServerAnimationEnumerationEntity {
}

impl super::entity::EntityTrait for ServerAnimationEnumerationEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAnimationEnumerationEntity {
}

pub static SERVERANIMATIONENUMERATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAnimationEnumerationEntity",
    name_hash: 4271133364,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAnimationEnumerationEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAnimationEnumerationEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAnimationEnumerationEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERANIMATIONENUMERATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAnimationEnumerationEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERANIMATIONENUMERATIONENTITY_TYPE_INFO
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


pub static SERVERANIMATIONENUMERATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAnimationEnumerationEntity-Array",
    name_hash: 2855841280,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerAnimationEnumerationEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerEdgeModelComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerEdgeModelComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerEdgeModelComponentTrait for ServerEdgeModelComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerEdgeModelComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerEdgeModelComponent {
}

impl super::entity::ComponentTrait for ServerEdgeModelComponent {
}

impl super::entity::EntityBusPeerTrait for ServerEdgeModelComponent {
}

pub static SERVEREDGEMODELCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEdgeModelComponent",
    name_hash: 3641264579,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerEdgeModelComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerEdgeModelComponent as Default>::default())),
            create_boxed: || Box::new(<ServerEdgeModelComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVEREDGEMODELCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerEdgeModelComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVEREDGEMODELCOMPONENT_TYPE_INFO
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


pub static SERVEREDGEMODELCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEdgeModelComponent-Array",
    name_hash: 2216381687,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerEdgeModelComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerStaticModelNetworkDestructionComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerStaticModelNetworkDestructionComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerStaticModelNetworkDestructionComponentTrait for ServerStaticModelNetworkDestructionComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerStaticModelNetworkDestructionComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerStaticModelNetworkDestructionComponent {
}

impl super::entity::ComponentTrait for ServerStaticModelNetworkDestructionComponent {
}

impl super::entity::EntityBusPeerTrait for ServerStaticModelNetworkDestructionComponent {
}

pub static SERVERSTATICMODELNETWORKDESTRUCTIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticModelNetworkDestructionComponent",
    name_hash: 1055596632,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerStaticModelNetworkDestructionComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerStaticModelNetworkDestructionComponent as Default>::default())),
            create_boxed: || Box::new(<ServerStaticModelNetworkDestructionComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSTATICMODELNETWORKDESTRUCTIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerStaticModelNetworkDestructionComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSTATICMODELNETWORKDESTRUCTIONCOMPONENT_TYPE_INFO
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


pub static SERVERSTATICMODELNETWORKDESTRUCTIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticModelNetworkDestructionComponent-Array",
    name_hash: 2294749932,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerStaticModelNetworkDestructionComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerPlayerInputTriggerEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerPlayerInputTriggerEntityTrait: super::entity::EntityTrait {
}

impl ServerPlayerInputTriggerEntityTrait for ServerPlayerInputTriggerEntity {
}

impl super::entity::EntityTrait for ServerPlayerInputTriggerEntity {
}

impl super::entity::EntityBusPeerTrait for ServerPlayerInputTriggerEntity {
}

pub static SERVERPLAYERINPUTTRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerInputTriggerEntity",
    name_hash: 1269121510,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerPlayerInputTriggerEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPlayerInputTriggerEntity as Default>::default())),
            create_boxed: || Box::new(<ServerPlayerInputTriggerEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERPLAYERINPUTTRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPlayerInputTriggerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPLAYERINPUTTRIGGERENTITY_TYPE_INFO
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


pub static SERVERPLAYERINPUTTRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerInputTriggerEntity-Array",
    name_hash: 568688338,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerPlayerInputTriggerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerMultipleTriggerEntity {
    pub _glacier_base: ServerTriggerEntity,
}

pub trait ServerMultipleTriggerEntityTrait: ServerTriggerEntityTrait {
}

impl ServerMultipleTriggerEntityTrait for ServerMultipleTriggerEntity {
}

impl ServerTriggerEntityTrait for ServerMultipleTriggerEntity {
}

impl super::entity::EntityTrait for ServerMultipleTriggerEntity {
}

impl super::entity::EntityBusPeerTrait for ServerMultipleTriggerEntity {
}

pub static SERVERMULTIPLETRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMultipleTriggerEntity",
    name_hash: 2124405075,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERTRIGGERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerMultipleTriggerEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerMultipleTriggerEntity as Default>::default())),
            create_boxed: || Box::new(<ServerMultipleTriggerEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERMULTIPLETRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerMultipleTriggerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERMULTIPLETRIGGERENTITY_TYPE_INFO
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


pub static SERVERMULTIPLETRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMultipleTriggerEntity-Array",
    name_hash: 2530362471,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerMultipleTriggerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerKillAllEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerKillAllEntityTrait: super::entity::EntityTrait {
}

impl ServerKillAllEntityTrait for ServerKillAllEntity {
}

impl super::entity::EntityTrait for ServerKillAllEntity {
}

impl super::entity::EntityBusPeerTrait for ServerKillAllEntity {
}

pub static SERVERKILLALLENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerKillAllEntity",
    name_hash: 47563512,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerKillAllEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerKillAllEntity as Default>::default())),
            create_boxed: || Box::new(<ServerKillAllEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERKILLALLENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerKillAllEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERKILLALLENTITY_TYPE_INFO
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


pub static SERVERKILLALLENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerKillAllEntity-Array",
    name_hash: 3973643468,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerKillAllEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerDelayTriggerEntity {
    pub _glacier_base: ServerTriggerEntity,
}

pub trait ServerDelayTriggerEntityTrait: ServerTriggerEntityTrait {
}

impl ServerDelayTriggerEntityTrait for ServerDelayTriggerEntity {
}

impl ServerTriggerEntityTrait for ServerDelayTriggerEntity {
}

impl super::entity::EntityTrait for ServerDelayTriggerEntity {
}

impl super::entity::EntityBusPeerTrait for ServerDelayTriggerEntity {
}

pub static SERVERDELAYTRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDelayTriggerEntity",
    name_hash: 2036334230,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERTRIGGERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerDelayTriggerEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerDelayTriggerEntity as Default>::default())),
            create_boxed: || Box::new(<ServerDelayTriggerEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERDELAYTRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerDelayTriggerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERDELAYTRIGGERENTITY_TYPE_INFO
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


pub static SERVERDELAYTRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDelayTriggerEntity-Array",
    name_hash: 2032284578,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerDelayTriggerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerDeathAreaTriggerEntity {
    pub _glacier_base: ServerTriggerEntity,
}

pub trait ServerDeathAreaTriggerEntityTrait: ServerTriggerEntityTrait {
}

impl ServerDeathAreaTriggerEntityTrait for ServerDeathAreaTriggerEntity {
}

impl ServerTriggerEntityTrait for ServerDeathAreaTriggerEntity {
}

impl super::entity::EntityTrait for ServerDeathAreaTriggerEntity {
}

impl super::entity::EntityBusPeerTrait for ServerDeathAreaTriggerEntity {
}

pub static SERVERDEATHAREATRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDeathAreaTriggerEntity",
    name_hash: 2941840840,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERTRIGGERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerDeathAreaTriggerEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerDeathAreaTriggerEntity as Default>::default())),
            create_boxed: || Box::new(<ServerDeathAreaTriggerEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERDEATHAREATRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerDeathAreaTriggerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERDEATHAREATRIGGERENTITY_TYPE_INFO
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


pub static SERVERDEATHAREATRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDeathAreaTriggerEntity-Array",
    name_hash: 3878043772,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerDeathAreaTriggerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerDamageAreaTriggerEntity {
    pub _glacier_base: ServerTriggerEntity,
}

pub trait ServerDamageAreaTriggerEntityTrait: ServerTriggerEntityTrait {
}

impl ServerDamageAreaTriggerEntityTrait for ServerDamageAreaTriggerEntity {
}

impl ServerTriggerEntityTrait for ServerDamageAreaTriggerEntity {
}

impl super::entity::EntityTrait for ServerDamageAreaTriggerEntity {
}

impl super::entity::EntityBusPeerTrait for ServerDamageAreaTriggerEntity {
}

pub static SERVERDAMAGEAREATRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDamageAreaTriggerEntity",
    name_hash: 1545289791,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERTRIGGERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerDamageAreaTriggerEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerDamageAreaTriggerEntity as Default>::default())),
            create_boxed: || Box::new(<ServerDamageAreaTriggerEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERDAMAGEAREATRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerDamageAreaTriggerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERDAMAGEAREATRIGGERENTITY_TYPE_INFO
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


pub static SERVERDAMAGEAREATRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDamageAreaTriggerEntity-Array",
    name_hash: 1831523723,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerDamageAreaTriggerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerCombatAreaTriggerEntity {
    pub _glacier_base: super::game_common::CombatAreaEntity,
}

pub trait ServerCombatAreaTriggerEntityTrait: super::game_common::CombatAreaEntityTrait {
}

impl ServerCombatAreaTriggerEntityTrait for ServerCombatAreaTriggerEntity {
}

impl super::game_common::CombatAreaEntityTrait for ServerCombatAreaTriggerEntity {
}

impl super::entity::EntityTrait for ServerCombatAreaTriggerEntity {
}

impl super::entity::EntityBusPeerTrait for ServerCombatAreaTriggerEntity {
}

pub static SERVERCOMBATAREATRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCombatAreaTriggerEntity",
    name_hash: 1376469474,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_common::COMBATAREAENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerCombatAreaTriggerEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCombatAreaTriggerEntity as Default>::default())),
            create_boxed: || Box::new(<ServerCombatAreaTriggerEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCOMBATAREATRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCombatAreaTriggerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCOMBATAREATRIGGERENTITY_TYPE_INFO
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


pub static SERVERCOMBATAREATRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCombatAreaTriggerEntity-Array",
    name_hash: 3146502358,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerCombatAreaTriggerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerCombatActionTriggerEntity {
    pub _glacier_base: ServerTriggerEntity,
}

pub trait ServerCombatActionTriggerEntityTrait: ServerTriggerEntityTrait {
}

impl ServerCombatActionTriggerEntityTrait for ServerCombatActionTriggerEntity {
}

impl ServerTriggerEntityTrait for ServerCombatActionTriggerEntity {
}

impl super::entity::EntityTrait for ServerCombatActionTriggerEntity {
}

impl super::entity::EntityBusPeerTrait for ServerCombatActionTriggerEntity {
}

pub static SERVERCOMBATACTIONTRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCombatActionTriggerEntity",
    name_hash: 2087270091,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERTRIGGERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerCombatActionTriggerEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCombatActionTriggerEntity as Default>::default())),
            create_boxed: || Box::new(<ServerCombatActionTriggerEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCOMBATACTIONTRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCombatActionTriggerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCOMBATACTIONTRIGGERENTITY_TYPE_INFO
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


pub static SERVERCOMBATACTIONTRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCombatActionTriggerEntity-Array",
    name_hash: 3749205503,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerCombatActionTriggerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerClearAreaTriggerEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerClearAreaTriggerEntityTrait: super::entity::EntityTrait {
}

impl ServerClearAreaTriggerEntityTrait for ServerClearAreaTriggerEntity {
}

impl super::entity::EntityTrait for ServerClearAreaTriggerEntity {
}

impl super::entity::EntityBusPeerTrait for ServerClearAreaTriggerEntity {
}

pub static SERVERCLEARAREATRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerClearAreaTriggerEntity",
    name_hash: 2594847405,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerClearAreaTriggerEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerClearAreaTriggerEntity as Default>::default())),
            create_boxed: || Box::new(<ServerClearAreaTriggerEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCLEARAREATRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerClearAreaTriggerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCLEARAREATRIGGERENTITY_TYPE_INFO
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


pub static SERVERCLEARAREATRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerClearAreaTriggerEntity-Array",
    name_hash: 1356885785,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerClearAreaTriggerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerAreaTriggerEntity {
    pub _glacier_base: ServerTriggerEntity,
}

pub trait ServerAreaTriggerEntityTrait: ServerTriggerEntityTrait {
}

impl ServerAreaTriggerEntityTrait for ServerAreaTriggerEntity {
}

impl ServerTriggerEntityTrait for ServerAreaTriggerEntity {
}

impl super::entity::EntityTrait for ServerAreaTriggerEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAreaTriggerEntity {
}

pub static SERVERAREATRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAreaTriggerEntity",
    name_hash: 2219296308,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERTRIGGERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAreaTriggerEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAreaTriggerEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAreaTriggerEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAREATRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAreaTriggerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAREATRIGGERENTITY_TYPE_INFO
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


pub static SERVERAREATRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAreaTriggerEntity-Array",
    name_hash: 1351926144,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerAreaTriggerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerVehicleSpawnEntity {
    pub _glacier_base: ServerSpawnEntity,
}

pub trait ServerVehicleSpawnEntityTrait: ServerSpawnEntityTrait {
}

impl ServerVehicleSpawnEntityTrait for ServerVehicleSpawnEntity {
}

impl ServerSpawnEntityTrait for ServerVehicleSpawnEntity {
}

impl super::entity::SpatialEntityTrait for ServerVehicleSpawnEntity {
}

impl super::entity::EntityTrait for ServerVehicleSpawnEntity {
}

impl super::entity::EntityBusPeerTrait for ServerVehicleSpawnEntity {
}

pub static SERVERVEHICLESPAWNENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleSpawnEntity",
    name_hash: 2615528152,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERSPAWNENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerVehicleSpawnEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerVehicleSpawnEntity as Default>::default())),
            create_boxed: || Box::new(<ServerVehicleSpawnEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERVEHICLESPAWNENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerVehicleSpawnEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERVEHICLESPAWNENTITY_TYPE_INFO
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


pub static SERVERVEHICLESPAWNENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleSpawnEntity-Array",
    name_hash: 3803686252,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerVehicleSpawnEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerSpawnEntityCreatedEntityInfo {
}

pub trait ServerSpawnEntityCreatedEntityInfoTrait: TypeObject {
}

impl ServerSpawnEntityCreatedEntityInfoTrait for ServerSpawnEntityCreatedEntityInfo {
}

pub static SERVERSPAWNENTITYCREATEDENTITYINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSpawnEntityCreatedEntityInfo",
    name_hash: 1957968853,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSpawnEntityCreatedEntityInfo as Default>::default())),
            create_boxed: || Box::new(<ServerSpawnEntityCreatedEntityInfo as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSPAWNENTITYCREATEDENTITYINFO_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSpawnEntityCreatedEntityInfo {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSPAWNENTITYCREATEDENTITYINFO_TYPE_INFO
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


pub static SERVERSPAWNENTITYCREATEDENTITYINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSpawnEntityCreatedEntityInfo-Array",
    name_hash: 2614754273,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerSpawnEntityCreatedEntityInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerSpawnEntity {
    pub _glacier_base: super::entity::SpatialEntity,
}

pub trait ServerSpawnEntityTrait: super::entity::SpatialEntityTrait {
}

impl ServerSpawnEntityTrait for ServerSpawnEntity {
}

impl super::entity::SpatialEntityTrait for ServerSpawnEntity {
}

impl super::entity::EntityTrait for ServerSpawnEntity {
}

impl super::entity::EntityBusPeerTrait for ServerSpawnEntity {
}

pub static SERVERSPAWNENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSpawnEntity",
    name_hash: 1577221888,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerSpawnEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSpawnEntity as Default>::default())),
            create_boxed: || Box::new(<ServerSpawnEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSPAWNENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSpawnEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSPAWNENTITY_TYPE_INFO
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


pub static SERVERSPAWNENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSpawnEntity-Array",
    name_hash: 3762578740,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerSpawnEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerOriginatingLocationSpawnContext {
    pub _glacier_base: super::game_common::UserSpawnContext,
}

pub trait ServerOriginatingLocationSpawnContextTrait: super::game_common::UserSpawnContextTrait {
}

impl ServerOriginatingLocationSpawnContextTrait for ServerOriginatingLocationSpawnContext {
}

impl super::game_common::UserSpawnContextTrait for ServerOriginatingLocationSpawnContext {
}

pub static SERVERORIGINATINGLOCATIONSPAWNCONTEXT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerOriginatingLocationSpawnContext",
    name_hash: 3830769112,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_common::USERSPAWNCONTEXT_TYPE_INFO),
        super_class_offset: offset_of!(ServerOriginatingLocationSpawnContext, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerOriginatingLocationSpawnContext as Default>::default())),
            create_boxed: || Box::new(<ServerOriginatingLocationSpawnContext as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERORIGINATINGLOCATIONSPAWNCONTEXT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerOriginatingLocationSpawnContext {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERORIGINATINGLOCATIONSPAWNCONTEXT_TYPE_INFO
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


pub static SERVERORIGINATINGLOCATIONSPAWNCONTEXT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerOriginatingLocationSpawnContext-Array",
    name_hash: 3872325740,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerOriginatingLocationSpawnContext"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerOriginatingBlueprintSpawnContext {
    pub _glacier_base: super::game_common::UserSpawnContext,
}

pub trait ServerOriginatingBlueprintSpawnContextTrait: super::game_common::UserSpawnContextTrait {
}

impl ServerOriginatingBlueprintSpawnContextTrait for ServerOriginatingBlueprintSpawnContext {
}

impl super::game_common::UserSpawnContextTrait for ServerOriginatingBlueprintSpawnContext {
}

pub static SERVERORIGINATINGBLUEPRINTSPAWNCONTEXT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerOriginatingBlueprintSpawnContext",
    name_hash: 3115015978,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_common::USERSPAWNCONTEXT_TYPE_INFO),
        super_class_offset: offset_of!(ServerOriginatingBlueprintSpawnContext, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerOriginatingBlueprintSpawnContext as Default>::default())),
            create_boxed: || Box::new(<ServerOriginatingBlueprintSpawnContext as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERORIGINATINGBLUEPRINTSPAWNCONTEXT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerOriginatingBlueprintSpawnContext {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERORIGINATINGBLUEPRINTSPAWNCONTEXT_TYPE_INFO
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


pub static SERVERORIGINATINGBLUEPRINTSPAWNCONTEXT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerOriginatingBlueprintSpawnContext-Array",
    name_hash: 1047296414,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerOriginatingBlueprintSpawnContext"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerCharacterSpawnEntity {
    pub _glacier_base: ServerSpawnEntity,
}

pub trait ServerCharacterSpawnEntityTrait: ServerSpawnEntityTrait {
}

impl ServerCharacterSpawnEntityTrait for ServerCharacterSpawnEntity {
}

impl ServerSpawnEntityTrait for ServerCharacterSpawnEntity {
}

impl super::entity::SpatialEntityTrait for ServerCharacterSpawnEntity {
}

impl super::entity::EntityTrait for ServerCharacterSpawnEntity {
}

impl super::entity::EntityBusPeerTrait for ServerCharacterSpawnEntity {
}

pub static SERVERCHARACTERSPAWNENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterSpawnEntity",
    name_hash: 2056684025,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERSPAWNENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerCharacterSpawnEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCharacterSpawnEntity as Default>::default())),
            create_boxed: || Box::new(<ServerCharacterSpawnEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCHARACTERSPAWNENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCharacterSpawnEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCHARACTERSPAWNENTITY_TYPE_INFO
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


pub static SERVERCHARACTERSPAWNENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterSpawnEntity-Array",
    name_hash: 3470295245,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerCharacterSpawnEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerTeleportEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerTeleportEntityTrait: super::entity::EntityTrait {
}

impl ServerTeleportEntityTrait for ServerTeleportEntity {
}

impl super::entity::EntityTrait for ServerTeleportEntity {
}

impl super::entity::EntityBusPeerTrait for ServerTeleportEntity {
}

pub static SERVERTELEPORTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTeleportEntity",
    name_hash: 860018778,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerTeleportEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerTeleportEntity as Default>::default())),
            create_boxed: || Box::new(<ServerTeleportEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERTELEPORTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerTeleportEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERTELEPORTENTITY_TYPE_INFO
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


pub static SERVERTELEPORTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTeleportEntity-Array",
    name_hash: 2554416110,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerTeleportEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerTeamFilterEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerTeamFilterEntityTrait: super::entity::EntityTrait {
}

impl ServerTeamFilterEntityTrait for ServerTeamFilterEntity {
}

impl super::entity::EntityTrait for ServerTeamFilterEntity {
}

impl super::entity::EntityBusPeerTrait for ServerTeamFilterEntity {
}

pub static SERVERTEAMFILTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTeamFilterEntity",
    name_hash: 2648992806,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerTeamFilterEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerTeamFilterEntity as Default>::default())),
            create_boxed: || Box::new(<ServerTeamFilterEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERTEAMFILTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerTeamFilterEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERTEAMFILTERENTITY_TYPE_INFO
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


pub static SERVERTEAMFILTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTeamFilterEntity-Array",
    name_hash: 1027315858,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerTeamFilterEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerTeamEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerTeamEntityTrait: super::entity::EntityTrait {
}

impl ServerTeamEntityTrait for ServerTeamEntity {
}

impl super::entity::EntityTrait for ServerTeamEntity {
}

impl super::entity::EntityBusPeerTrait for ServerTeamEntity {
}

pub static SERVERTEAMENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTeamEntity",
    name_hash: 2618728934,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerTeamEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerTeamEntity as Default>::default())),
            create_boxed: || Box::new(<ServerTeamEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERTEAMENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerTeamEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERTEAMENTITY_TYPE_INFO
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


pub static SERVERTEAMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTeamEntity-Array",
    name_hash: 434036434,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerTeamEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerTacticalObjectiveEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerTacticalObjectiveEntityTrait: super::entity::EntityTrait {
}

impl ServerTacticalObjectiveEntityTrait for ServerTacticalObjectiveEntity {
}

impl super::entity::EntityTrait for ServerTacticalObjectiveEntity {
}

impl super::entity::EntityBusPeerTrait for ServerTacticalObjectiveEntity {
}

pub static SERVERTACTICALOBJECTIVEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTacticalObjectiveEntity",
    name_hash: 1417089457,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerTacticalObjectiveEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerTacticalObjectiveEntity as Default>::default())),
            create_boxed: || Box::new(<ServerTacticalObjectiveEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERTACTICALOBJECTIVEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerTacticalObjectiveEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERTACTICALOBJECTIVEENTITY_TYPE_INFO
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


pub static SERVERTACTICALOBJECTIVEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTacticalObjectiveEntity-Array",
    name_hash: 3364242437,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerTacticalObjectiveEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerLadderComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerLadderComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerLadderComponentTrait for ServerLadderComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerLadderComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerLadderComponent {
}

impl super::entity::ComponentTrait for ServerLadderComponent {
}

impl super::entity::EntityBusPeerTrait for ServerLadderComponent {
}

pub static SERVERLADDERCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLadderComponent",
    name_hash: 3454057109,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerLadderComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerLadderComponent as Default>::default())),
            create_boxed: || Box::new(<ServerLadderComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERLADDERCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerLadderComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERLADDERCOMPONENT_TYPE_INFO
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


pub static SERVERLADDERCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLadderComponent-Array",
    name_hash: 341740065,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerLadderComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerExplosionEntity {
    pub _glacier_base: super::game_common::ExplosionEntity,
}

pub trait ServerExplosionEntityTrait: super::game_common::ExplosionEntityTrait {
}

impl ServerExplosionEntityTrait for ServerExplosionEntity {
}

impl super::game_common::ExplosionEntityTrait for ServerExplosionEntity {
}

impl super::entity::EntityTrait for ServerExplosionEntity {
}

impl super::entity::EntityBusPeerTrait for ServerExplosionEntity {
}

pub static SERVEREXPLOSIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerExplosionEntity",
    name_hash: 431442030,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_common::EXPLOSIONENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerExplosionEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerExplosionEntity as Default>::default())),
            create_boxed: || Box::new(<ServerExplosionEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVEREXPLOSIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerExplosionEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVEREXPLOSIONENTITY_TYPE_INFO
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


pub static SERVEREXPLOSIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerExplosionEntity-Array",
    name_hash: 1384235354,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerExplosionEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerDynamicAvoidanceEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerDynamicAvoidanceEntityTrait: super::entity::EntityTrait {
}

impl ServerDynamicAvoidanceEntityTrait for ServerDynamicAvoidanceEntity {
}

impl super::entity::EntityTrait for ServerDynamicAvoidanceEntity {
}

impl super::entity::EntityBusPeerTrait for ServerDynamicAvoidanceEntity {
}

pub static SERVERDYNAMICAVOIDANCEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDynamicAvoidanceEntity",
    name_hash: 1121017618,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerDynamicAvoidanceEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerDynamicAvoidanceEntity as Default>::default())),
            create_boxed: || Box::new(<ServerDynamicAvoidanceEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERDYNAMICAVOIDANCEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerDynamicAvoidanceEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERDYNAMICAVOIDANCEENTITY_TYPE_INFO
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


pub static SERVERDYNAMICAVOIDANCEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDynamicAvoidanceEntity-Array",
    name_hash: 100288038,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerDynamicAvoidanceEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerVehicleEntity {
    pub _glacier_base: ServerControllableEntity,
}

pub trait ServerVehicleEntityTrait: ServerControllableEntityTrait {
}

impl ServerVehicleEntityTrait for ServerVehicleEntity {
}

impl ServerControllableEntityTrait for ServerVehicleEntity {
}

impl ServerPhysicsEntityTrait for ServerVehicleEntity {
}

impl ServerGameComponentEntityTrait for ServerVehicleEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ServerVehicleEntity {
}

impl super::entity::ComponentEntityTrait for ServerVehicleEntity {
}

impl super::entity::SpatialEntityTrait for ServerVehicleEntity {
}

impl super::entity::EntityTrait for ServerVehicleEntity {
}

impl super::entity::EntityBusPeerTrait for ServerVehicleEntity {
}

pub static SERVERVEHICLEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleEntity",
    name_hash: 3278676003,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERCONTROLLABLEENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerVehicleEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerVehicleEntity as Default>::default())),
            create_boxed: || Box::new(<ServerVehicleEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERVEHICLEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerVehicleEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERVEHICLEENTITY_TYPE_INFO
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


pub static SERVERVEHICLEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleEntity-Array",
    name_hash: 4038222743,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerVehicleEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerWheelComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerWheelComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerWheelComponentTrait for ServerWheelComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerWheelComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerWheelComponent {
}

impl super::entity::ComponentTrait for ServerWheelComponent {
}

impl super::entity::EntityBusPeerTrait for ServerWheelComponent {
}

pub static SERVERWHEELCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWheelComponent",
    name_hash: 2554328092,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerWheelComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerWheelComponent as Default>::default())),
            create_boxed: || Box::new(<ServerWheelComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERWHEELCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerWheelComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERWHEELCOMPONENT_TYPE_INFO
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


pub static SERVERWHEELCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWheelComponent-Array",
    name_hash: 2773969960,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerWheelComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerVehiclePhysicsComponent {
    pub _glacier_base: super::physics::PartPhysicsComponent,
}

pub trait ServerVehiclePhysicsComponentTrait: super::physics::PartPhysicsComponentTrait {
}

impl ServerVehiclePhysicsComponentTrait for ServerVehiclePhysicsComponent {
}

impl super::physics::PartPhysicsComponentTrait for ServerVehiclePhysicsComponent {
}

impl super::physics::PhysicsComponentTrait for ServerVehiclePhysicsComponent {
}

impl super::entity::ComponentTrait for ServerVehiclePhysicsComponent {
}

impl super::entity::EntityBusPeerTrait for ServerVehiclePhysicsComponent {
}

pub static SERVERVEHICLEPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehiclePhysicsComponent",
    name_hash: 89803836,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::PARTPHYSICSCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerVehiclePhysicsComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerVehiclePhysicsComponent as Default>::default())),
            create_boxed: || Box::new(<ServerVehiclePhysicsComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERVEHICLEPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerVehiclePhysicsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERVEHICLEPHYSICSCOMPONENT_TYPE_INFO
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


pub static SERVERVEHICLEPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehiclePhysicsComponent-Array",
    name_hash: 359867272,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerVehiclePhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerVehicleHealthComponent {
    pub _glacier_base: ServerControllableHealthComponent,
}

pub trait ServerVehicleHealthComponentTrait: ServerControllableHealthComponentTrait {
}

impl ServerVehicleHealthComponentTrait for ServerVehicleHealthComponent {
}

impl ServerControllableHealthComponentTrait for ServerVehicleHealthComponent {
}

impl ServerGameHealthComponentTrait for ServerVehicleHealthComponent {
}

impl super::gameplay_sim::HealthComponentTrait for ServerVehicleHealthComponent {
}

impl super::entity::ComponentTrait for ServerVehicleHealthComponent {
}

impl super::entity::EntityBusPeerTrait for ServerVehicleHealthComponent {
}

pub static SERVERVEHICLEHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleHealthComponent",
    name_hash: 1964400779,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERCONTROLLABLEHEALTHCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerVehicleHealthComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerVehicleHealthComponent as Default>::default())),
            create_boxed: || Box::new(<ServerVehicleHealthComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERVEHICLEHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerVehicleHealthComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERVEHICLEHEALTHCOMPONENT_TYPE_INFO
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


pub static SERVERVEHICLEHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleHealthComponent-Array",
    name_hash: 3198029631,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerVehicleHealthComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerVehicleEntryComponent {
    pub _glacier_base: ServerPlayerEntryComponent,
}

pub trait ServerVehicleEntryComponentTrait: ServerPlayerEntryComponentTrait {
}

impl ServerVehicleEntryComponentTrait for ServerVehicleEntryComponent {
}

impl ServerPlayerEntryComponentTrait for ServerVehicleEntryComponent {
}

impl ServerGameEntryComponentTrait for ServerVehicleEntryComponent {
}

impl ServerEntryComponentTrait for ServerVehicleEntryComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerVehicleEntryComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerVehicleEntryComponent {
}

impl super::entity::ComponentTrait for ServerVehicleEntryComponent {
}

impl super::entity::EntityBusPeerTrait for ServerVehicleEntryComponent {
}

pub static SERVERVEHICLEENTRYCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleEntryComponent",
    name_hash: 1756080835,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERPLAYERENTRYCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerVehicleEntryComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerVehicleEntryComponent as Default>::default())),
            create_boxed: || Box::new(<ServerVehicleEntryComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERVEHICLEENTRYCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerVehicleEntryComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERVEHICLEENTRYCOMPONENT_TYPE_INFO
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


pub static SERVERVEHICLEENTRYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleEntryComponent-Array",
    name_hash: 2520935927,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerVehicleEntryComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerVehicleCustomizationComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerVehicleCustomizationComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerVehicleCustomizationComponentTrait for ServerVehicleCustomizationComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerVehicleCustomizationComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerVehicleCustomizationComponent {
}

impl super::entity::ComponentTrait for ServerVehicleCustomizationComponent {
}

impl super::entity::EntityBusPeerTrait for ServerVehicleCustomizationComponent {
}

pub static SERVERVEHICLECUSTOMIZATIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleCustomizationComponent",
    name_hash: 3237061514,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerVehicleCustomizationComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerVehicleCustomizationComponent as Default>::default())),
            create_boxed: || Box::new(<ServerVehicleCustomizationComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERVEHICLECUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerVehicleCustomizationComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERVEHICLECUSTOMIZATIONCOMPONENT_TYPE_INFO
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


pub static SERVERVEHICLECUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleCustomizationComponent-Array",
    name_hash: 71859902,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerVehicleCustomizationComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerTrackWheelComponent {
    pub _glacier_base: ServerWheelComponent,
}

pub trait ServerTrackWheelComponentTrait: ServerWheelComponentTrait {
}

impl ServerTrackWheelComponentTrait for ServerTrackWheelComponent {
}

impl ServerWheelComponentTrait for ServerTrackWheelComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerTrackWheelComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerTrackWheelComponent {
}

impl super::entity::ComponentTrait for ServerTrackWheelComponent {
}

impl super::entity::EntityBusPeerTrait for ServerTrackWheelComponent {
}

pub static SERVERTRACKWHEELCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTrackWheelComponent",
    name_hash: 321277555,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERWHEELCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerTrackWheelComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerTrackWheelComponent as Default>::default())),
            create_boxed: || Box::new(<ServerTrackWheelComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERTRACKWHEELCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerTrackWheelComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERTRACKWHEELCOMPONENT_TYPE_INFO
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


pub static SERVERTRACKWHEELCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTrackWheelComponent-Array",
    name_hash: 2472315463,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerTrackWheelComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerTrackComponent {
    pub _glacier_base: ServerMeshComponent,
}

pub trait ServerTrackComponentTrait: ServerMeshComponentTrait {
}

impl ServerTrackComponentTrait for ServerTrackComponent {
}

impl ServerMeshComponentTrait for ServerTrackComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerTrackComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerTrackComponent {
}

impl super::entity::ComponentTrait for ServerTrackComponent {
}

impl super::entity::EntityBusPeerTrait for ServerTrackComponent {
}

pub static SERVERTRACKCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTrackComponent",
    name_hash: 1509671104,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERMESHCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerTrackComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerTrackComponent as Default>::default())),
            create_boxed: || Box::new(<ServerTrackComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERTRACKCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerTrackComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERTRACKCOMPONENT_TYPE_INFO
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


pub static SERVERTRACKCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTrackComponent-Array",
    name_hash: 2161899380,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerTrackComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerStanceFilterComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerStanceFilterComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerStanceFilterComponentTrait for ServerStanceFilterComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerStanceFilterComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerStanceFilterComponent {
}

impl super::entity::ComponentTrait for ServerStanceFilterComponent {
}

impl super::entity::EntityBusPeerTrait for ServerStanceFilterComponent {
}

pub static SERVERSTANCEFILTERCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStanceFilterComponent",
    name_hash: 3660844417,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerStanceFilterComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerStanceFilterComponent as Default>::default())),
            create_boxed: || Box::new(<ServerStanceFilterComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSTANCEFILTERCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerStanceFilterComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSTANCEFILTERCOMPONENT_TYPE_INFO
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


pub static SERVERSTANCEFILTERCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStanceFilterComponent-Array",
    name_hash: 64071477,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerStanceFilterComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerRotorComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerRotorComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerRotorComponentTrait for ServerRotorComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerRotorComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerRotorComponent {
}

impl super::entity::ComponentTrait for ServerRotorComponent {
}

impl super::entity::EntityBusPeerTrait for ServerRotorComponent {
}

pub static SERVERROTORCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerRotorComponent",
    name_hash: 987902491,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerRotorComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerRotorComponent as Default>::default())),
            create_boxed: || Box::new(<ServerRotorComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERROTORCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerRotorComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERROTORCOMPONENT_TYPE_INFO
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


pub static SERVERROTORCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerRotorComponent-Array",
    name_hash: 150194351,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerRotorComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerMeshComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerMeshComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerMeshComponentTrait for ServerMeshComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerMeshComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerMeshComponent {
}

impl super::entity::ComponentTrait for ServerMeshComponent {
}

impl super::entity::EntityBusPeerTrait for ServerMeshComponent {
}

pub static SERVERMESHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMeshComponent",
    name_hash: 2154735580,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerMeshComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerMeshComponent as Default>::default())),
            create_boxed: || Box::new(<ServerMeshComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERMESHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerMeshComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERMESHCOMPONENT_TYPE_INFO
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


pub static SERVERMESHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMeshComponent-Array",
    name_hash: 888270952,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerMeshComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerEngineComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerEngineComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerEngineComponentTrait for ServerEngineComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerEngineComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerEngineComponent {
}

impl super::entity::ComponentTrait for ServerEngineComponent {
}

impl super::entity::EntityBusPeerTrait for ServerEngineComponent {
}

pub static SERVERENGINECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEngineComponent",
    name_hash: 2407187169,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerEngineComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerEngineComponent as Default>::default())),
            create_boxed: || Box::new(<ServerEngineComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERENGINECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerEngineComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERENGINECOMPONENT_TYPE_INFO
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


pub static SERVERENGINECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEngineComponent-Array",
    name_hash: 1981242837,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerEngineComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerEntryComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerEntryComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerEntryComponentTrait for ServerEntryComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerEntryComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerEntryComponent {
}

impl super::entity::ComponentTrait for ServerEntryComponent {
}

impl super::entity::EntityBusPeerTrait for ServerEntryComponent {
}

pub static SERVERENTRYCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEntryComponent",
    name_hash: 3503890075,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerEntryComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerEntryComponent as Default>::default())),
            create_boxed: || Box::new(<ServerEntryComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERENTRYCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerEntryComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERENTRYCOMPONENT_TYPE_INFO
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


pub static SERVERENTRYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEntryComponent-Array",
    name_hash: 3968472367,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerEntryComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerDriverStaticObjectComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerDriverStaticObjectComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerDriverStaticObjectComponentTrait for ServerDriverStaticObjectComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerDriverStaticObjectComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerDriverStaticObjectComponent {
}

impl super::entity::ComponentTrait for ServerDriverStaticObjectComponent {
}

impl super::entity::EntityBusPeerTrait for ServerDriverStaticObjectComponent {
}

pub static SERVERDRIVERSTATICOBJECTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDriverStaticObjectComponent",
    name_hash: 1249446332,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerDriverStaticObjectComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerDriverStaticObjectComponent as Default>::default())),
            create_boxed: || Box::new(<ServerDriverStaticObjectComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERDRIVERSTATICOBJECTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerDriverStaticObjectComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERDRIVERSTATICOBJECTCOMPONENT_TYPE_INFO
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


pub static SERVERDRIVERSTATICOBJECTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDriverStaticObjectComponent-Array",
    name_hash: 1963589896,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerDriverStaticObjectComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerDriverComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerDriverComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerDriverComponentTrait for ServerDriverComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerDriverComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerDriverComponent {
}

impl super::entity::ComponentTrait for ServerDriverComponent {
}

impl super::entity::EntityBusPeerTrait for ServerDriverComponent {
}

pub static SERVERDRIVERCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDriverComponent",
    name_hash: 2214803825,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerDriverComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerDriverComponent as Default>::default())),
            create_boxed: || Box::new(<ServerDriverComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERDRIVERCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerDriverComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERDRIVERCOMPONENT_TYPE_INFO
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


pub static SERVERDRIVERCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDriverComponent-Array",
    name_hash: 1431403589,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerDriverComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerControllableHealthComponent {
    pub _glacier_base: ServerGameHealthComponent,
}

pub trait ServerControllableHealthComponentTrait: ServerGameHealthComponentTrait {
}

impl ServerControllableHealthComponentTrait for ServerControllableHealthComponent {
}

impl ServerGameHealthComponentTrait for ServerControllableHealthComponent {
}

impl super::gameplay_sim::HealthComponentTrait for ServerControllableHealthComponent {
}

impl super::entity::ComponentTrait for ServerControllableHealthComponent {
}

impl super::entity::EntityBusPeerTrait for ServerControllableHealthComponent {
}

pub static SERVERCONTROLLABLEHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerControllableHealthComponent",
    name_hash: 538642578,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMEHEALTHCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerControllableHealthComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerControllableHealthComponent as Default>::default())),
            create_boxed: || Box::new(<ServerControllableHealthComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCONTROLLABLEHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerControllableHealthComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCONTROLLABLEHEALTHCOMPONENT_TYPE_INFO
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


pub static SERVERCONTROLLABLEHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerControllableHealthComponent-Array",
    name_hash: 2772242342,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerControllableHealthComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerCharacterEntryComponent {
    pub _glacier_base: ServerGameEntryComponent,
}

pub trait ServerCharacterEntryComponentTrait: ServerGameEntryComponentTrait {
}

impl ServerCharacterEntryComponentTrait for ServerCharacterEntryComponent {
}

impl ServerGameEntryComponentTrait for ServerCharacterEntryComponent {
}

impl ServerEntryComponentTrait for ServerCharacterEntryComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerCharacterEntryComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerCharacterEntryComponent {
}

impl super::entity::ComponentTrait for ServerCharacterEntryComponent {
}

impl super::entity::EntityBusPeerTrait for ServerCharacterEntryComponent {
}

pub static SERVERCHARACTERENTRYCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterEntryComponent",
    name_hash: 1432047426,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMEENTRYCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerCharacterEntryComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCharacterEntryComponent as Default>::default())),
            create_boxed: || Box::new(<ServerCharacterEntryComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCHARACTERENTRYCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCharacterEntryComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCHARACTERENTRYCOMPONENT_TYPE_INFO
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


pub static SERVERCHARACTERENTRYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterEntryComponent-Array",
    name_hash: 746905334,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerCharacterEntryComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerCharacterEntity {
    pub _glacier_base: ServerControllableEntity,
}

pub trait ServerCharacterEntityTrait: ServerControllableEntityTrait {
}

impl ServerCharacterEntityTrait for ServerCharacterEntity {
}

impl ServerControllableEntityTrait for ServerCharacterEntity {
}

impl ServerPhysicsEntityTrait for ServerCharacterEntity {
}

impl ServerGameComponentEntityTrait for ServerCharacterEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ServerCharacterEntity {
}

impl super::entity::ComponentEntityTrait for ServerCharacterEntity {
}

impl super::entity::SpatialEntityTrait for ServerCharacterEntity {
}

impl super::entity::EntityTrait for ServerCharacterEntity {
}

impl super::entity::EntityBusPeerTrait for ServerCharacterEntity {
}

pub static SERVERCHARACTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterEntity",
    name_hash: 2978141730,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERCONTROLLABLEENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerCharacterEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCharacterEntity as Default>::default())),
            create_boxed: || Box::new(<ServerCharacterEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCHARACTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCharacterEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCHARACTERENTITY_TYPE_INFO
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


pub static SERVERCHARACTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterEntity-Array",
    name_hash: 2336347286,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerCharacterEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CharacterServerPlayerExtent {
    pub _glacier_base: ServerGamePlayerInternalExtent,
}

pub trait CharacterServerPlayerExtentTrait: ServerGamePlayerInternalExtentTrait {
}

impl CharacterServerPlayerExtentTrait for CharacterServerPlayerExtent {
}

impl ServerGamePlayerInternalExtentTrait for CharacterServerPlayerExtent {
}

impl ServerPlayerExtentTrait for CharacterServerPlayerExtent {
}

pub static CHARACTERSERVERPLAYEREXTENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterServerPlayerExtent",
    name_hash: 1807198908,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMEPLAYERINTERNALEXTENT_TYPE_INFO),
        super_class_offset: offset_of!(CharacterServerPlayerExtent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CharacterServerPlayerExtent as Default>::default())),
            create_boxed: || Box::new(<CharacterServerPlayerExtent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CHARACTERSERVERPLAYEREXTENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CharacterServerPlayerExtent {
    fn type_info(&self) -> &'static TypeInfo {
        CHARACTERSERVERPLAYEREXTENT_TYPE_INFO
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


pub static CHARACTERSERVERPLAYEREXTENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterServerPlayerExtent-Array",
    name_hash: 1828545032,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("CharacterServerPlayerExtent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerWarpAnimationComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerWarpAnimationComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerWarpAnimationComponentTrait for ServerWarpAnimationComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerWarpAnimationComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerWarpAnimationComponent {
}

impl super::entity::ComponentTrait for ServerWarpAnimationComponent {
}

impl super::entity::EntityBusPeerTrait for ServerWarpAnimationComponent {
}

pub static SERVERWARPANIMATIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWarpAnimationComponent",
    name_hash: 2591588237,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerWarpAnimationComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerWarpAnimationComponent as Default>::default())),
            create_boxed: || Box::new(<ServerWarpAnimationComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERWARPANIMATIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerWarpAnimationComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERWARPANIMATIONCOMPONENT_TYPE_INFO
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


pub static SERVERWARPANIMATIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWarpAnimationComponent-Array",
    name_hash: 3118637881,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerWarpAnimationComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerVehicleEntryListenerComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerVehicleEntryListenerComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerVehicleEntryListenerComponentTrait for ServerVehicleEntryListenerComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerVehicleEntryListenerComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerVehicleEntryListenerComponent {
}

impl super::entity::ComponentTrait for ServerVehicleEntryListenerComponent {
}

impl super::entity::EntityBusPeerTrait for ServerVehicleEntryListenerComponent {
}

pub static SERVERVEHICLEENTRYLISTENERCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleEntryListenerComponent",
    name_hash: 4258433885,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerVehicleEntryListenerComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerVehicleEntryListenerComponent as Default>::default())),
            create_boxed: || Box::new(<ServerVehicleEntryListenerComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERVEHICLEENTRYLISTENERCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerVehicleEntryListenerComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERVEHICLEENTRYLISTENERCOMPONENT_TYPE_INFO
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


pub static SERVERVEHICLEENTRYLISTENERCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleEntryListenerComponent-Array",
    name_hash: 4225058665,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerVehicleEntryListenerComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerCharacterPhysicsComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerCharacterPhysicsComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerCharacterPhysicsComponentTrait for ServerCharacterPhysicsComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerCharacterPhysicsComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerCharacterPhysicsComponent {
}

impl super::entity::ComponentTrait for ServerCharacterPhysicsComponent {
}

impl super::entity::EntityBusPeerTrait for ServerCharacterPhysicsComponent {
}

pub static SERVERCHARACTERPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterPhysicsComponent",
    name_hash: 1430294525,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerCharacterPhysicsComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCharacterPhysicsComponent as Default>::default())),
            create_boxed: || Box::new(<ServerCharacterPhysicsComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCHARACTERPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCharacterPhysicsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCHARACTERPHYSICSCOMPONENT_TYPE_INFO
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


pub static SERVERCHARACTERPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterPhysicsComponent-Array",
    name_hash: 1368702153,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerCharacterPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerCharacterMasterPhysicsComponent {
    pub _glacier_base: super::physics::CharacterPhysicsComponent,
}

pub trait ServerCharacterMasterPhysicsComponentTrait: super::physics::CharacterPhysicsComponentTrait {
}

impl ServerCharacterMasterPhysicsComponentTrait for ServerCharacterMasterPhysicsComponent {
}

impl super::physics::CharacterPhysicsComponentTrait for ServerCharacterMasterPhysicsComponent {
}

impl super::physics::PhysicsComponentTrait for ServerCharacterMasterPhysicsComponent {
}

impl super::entity::ComponentTrait for ServerCharacterMasterPhysicsComponent {
}

impl super::entity::EntityBusPeerTrait for ServerCharacterMasterPhysicsComponent {
}

pub static SERVERCHARACTERMASTERPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterMasterPhysicsComponent",
    name_hash: 925912289,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::CHARACTERPHYSICSCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerCharacterMasterPhysicsComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCharacterMasterPhysicsComponent as Default>::default())),
            create_boxed: || Box::new(<ServerCharacterMasterPhysicsComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCHARACTERMASTERPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCharacterMasterPhysicsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCHARACTERMASTERPHYSICSCOMPONENT_TYPE_INFO
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


pub static SERVERCHARACTERMASTERPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterMasterPhysicsComponent-Array",
    name_hash: 910976981,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerCharacterMasterPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerCharacterHealthComponent {
    pub _glacier_base: ServerControllableHealthComponent,
}

pub trait ServerCharacterHealthComponentTrait: ServerControllableHealthComponentTrait {
}

impl ServerCharacterHealthComponentTrait for ServerCharacterHealthComponent {
}

impl ServerControllableHealthComponentTrait for ServerCharacterHealthComponent {
}

impl ServerGameHealthComponentTrait for ServerCharacterHealthComponent {
}

impl super::gameplay_sim::HealthComponentTrait for ServerCharacterHealthComponent {
}

impl super::entity::ComponentTrait for ServerCharacterHealthComponent {
}

impl super::entity::EntityBusPeerTrait for ServerCharacterHealthComponent {
}

pub static SERVERCHARACTERHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterHealthComponent",
    name_hash: 2290659946,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERCONTROLLABLEHEALTHCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerCharacterHealthComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCharacterHealthComponent as Default>::default())),
            create_boxed: || Box::new(<ServerCharacterHealthComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCHARACTERHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCharacterHealthComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCHARACTERHEALTHCOMPONENT_TYPE_INFO
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


pub static SERVERCHARACTERHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterHealthComponent-Array",
    name_hash: 1442512222,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerCharacterHealthComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerCharacterCustomizationComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerCharacterCustomizationComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerCharacterCustomizationComponentTrait for ServerCharacterCustomizationComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerCharacterCustomizationComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerCharacterCustomizationComponent {
}

impl super::entity::ComponentTrait for ServerCharacterCustomizationComponent {
}

impl super::entity::EntityBusPeerTrait for ServerCharacterCustomizationComponent {
}

pub static SERVERCHARACTERCUSTOMIZATIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterCustomizationComponent",
    name_hash: 2379094411,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerCharacterCustomizationComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCharacterCustomizationComponent as Default>::default())),
            create_boxed: || Box::new(<ServerCharacterCustomizationComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCHARACTERCUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCharacterCustomizationComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCHARACTERCUSTOMIZATIONCOMPONENT_TYPE_INFO
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


pub static SERVERCHARACTERCUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterCustomizationComponent-Array",
    name_hash: 444368959,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerCharacterCustomizationComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerCharacterCameraComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerCharacterCameraComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerCharacterCameraComponentTrait for ServerCharacterCameraComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerCharacterCameraComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerCharacterCameraComponent {
}

impl super::entity::ComponentTrait for ServerCharacterCameraComponent {
}

impl super::entity::EntityBusPeerTrait for ServerCharacterCameraComponent {
}

pub static SERVERCHARACTERCAMERACOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterCameraComponent",
    name_hash: 2419289839,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerCharacterCameraComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCharacterCameraComponent as Default>::default())),
            create_boxed: || Box::new(<ServerCharacterCameraComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCHARACTERCAMERACOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCharacterCameraComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCHARACTERCAMERACOMPONENT_TYPE_INFO
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


pub static SERVERCHARACTERCAMERACOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterCameraComponent-Array",
    name_hash: 1057770715,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerCharacterCameraComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerAntInputComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerAntInputComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerAntInputComponentTrait for ServerAntInputComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerAntInputComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerAntInputComponent {
}

impl super::entity::ComponentTrait for ServerAntInputComponent {
}

impl super::entity::EntityBusPeerTrait for ServerAntInputComponent {
}

pub static SERVERANTINPUTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAntInputComponent",
    name_hash: 1642605154,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerAntInputComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAntInputComponent as Default>::default())),
            create_boxed: || Box::new(<ServerAntInputComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERANTINPUTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAntInputComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERANTINPUTCOMPONENT_TYPE_INFO
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


pub static SERVERANTINPUTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAntInputComponent-Array",
    name_hash: 2224363862,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerAntInputComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerAntDrivenComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerAntDrivenComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerAntDrivenComponentTrait for ServerAntDrivenComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerAntDrivenComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerAntDrivenComponent {
}

impl super::entity::ComponentTrait for ServerAntDrivenComponent {
}

impl super::entity::EntityBusPeerTrait for ServerAntDrivenComponent {
}

pub static SERVERANTDRIVENCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAntDrivenComponent",
    name_hash: 4205348310,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerAntDrivenComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAntDrivenComponent as Default>::default())),
            create_boxed: || Box::new(<ServerAntDrivenComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERANTDRIVENCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAntDrivenComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERANTDRIVENCOMPONENT_TYPE_INFO
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


pub static SERVERANTDRIVENCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAntDrivenComponent-Array",
    name_hash: 240348002,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerAntDrivenComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerWarpAnimationEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerWarpAnimationEntityTrait: super::entity::EntityTrait {
}

impl ServerWarpAnimationEntityTrait for ServerWarpAnimationEntity {
}

impl super::entity::EntityTrait for ServerWarpAnimationEntity {
}

impl super::entity::EntityBusPeerTrait for ServerWarpAnimationEntity {
}

pub static SERVERWARPANIMATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWarpAnimationEntity",
    name_hash: 3305139577,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerWarpAnimationEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerWarpAnimationEntity as Default>::default())),
            create_boxed: || Box::new(<ServerWarpAnimationEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERWARPANIMATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerWarpAnimationEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERWARPANIMATIONENTITY_TYPE_INFO
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


pub static SERVERWARPANIMATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWarpAnimationEntity-Array",
    name_hash: 3068464205,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerWarpAnimationEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerPhysicsDrivenAnimationEntity {
    pub _glacier_base: super::game_common::PhysicsDrivenAnimationEntity,
}

pub trait ServerPhysicsDrivenAnimationEntityTrait: super::game_common::PhysicsDrivenAnimationEntityTrait {
}

impl ServerPhysicsDrivenAnimationEntityTrait for ServerPhysicsDrivenAnimationEntity {
}

impl super::game_common::PhysicsDrivenAnimationEntityTrait for ServerPhysicsDrivenAnimationEntity {
}

impl super::entity::EntityTrait for ServerPhysicsDrivenAnimationEntity {
}

impl super::entity::EntityBusPeerTrait for ServerPhysicsDrivenAnimationEntity {
}

pub static SERVERPHYSICSDRIVENANIMATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPhysicsDrivenAnimationEntity",
    name_hash: 2818009572,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_common::PHYSICSDRIVENANIMATIONENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerPhysicsDrivenAnimationEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPhysicsDrivenAnimationEntity as Default>::default())),
            create_boxed: || Box::new(<ServerPhysicsDrivenAnimationEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERPHYSICSDRIVENANIMATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPhysicsDrivenAnimationEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPHYSICSDRIVENANIMATIONENTITY_TYPE_INFO
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


pub static SERVERPHYSICSDRIVENANIMATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPhysicsDrivenAnimationEntity-Array",
    name_hash: 1938386384,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerPhysicsDrivenAnimationEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerCannedScenarioEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerCannedScenarioEntityTrait: super::entity::EntityTrait {
}

impl ServerCannedScenarioEntityTrait for ServerCannedScenarioEntity {
}

impl super::entity::EntityTrait for ServerCannedScenarioEntity {
}

impl super::entity::EntityBusPeerTrait for ServerCannedScenarioEntity {
}

pub static SERVERCANNEDSCENARIOENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCannedScenarioEntity",
    name_hash: 3972757750,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerCannedScenarioEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCannedScenarioEntity as Default>::default())),
            create_boxed: || Box::new(<ServerCannedScenarioEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCANNEDSCENARIOENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCannedScenarioEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCANNEDSCENARIOENTITY_TYPE_INFO
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


pub static SERVERCANNEDSCENARIOENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCannedScenarioEntity-Array",
    name_hash: 439074242,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerCannedScenarioEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerRecordVehicleTrack {
    pub _glacier_base: super::timeline::RecordTrackBase,
}

pub trait ServerRecordVehicleTrackTrait: super::timeline::RecordTrackBaseTrait {
}

impl ServerRecordVehicleTrackTrait for ServerRecordVehicleTrack {
}

impl super::timeline::RecordTrackBaseTrait for ServerRecordVehicleTrack {
}

impl super::timeline::LinkTrackTrait for ServerRecordVehicleTrack {
}

impl super::timeline::TimelineTrackTrait for ServerRecordVehicleTrack {
}

pub static SERVERRECORDVEHICLETRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerRecordVehicleTrack",
    name_hash: 1212729370,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::RECORDTRACKBASE_TYPE_INFO),
        super_class_offset: offset_of!(ServerRecordVehicleTrack, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerRecordVehicleTrack as Default>::default())),
            create_boxed: || Box::new(<ServerRecordVehicleTrack as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERRECORDVEHICLETRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerRecordVehicleTrack {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERRECORDVEHICLETRACK_TYPE_INFO
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


pub static SERVERRECORDVEHICLETRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerRecordVehicleTrack-Array",
    name_hash: 3994798894,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerRecordVehicleTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerSyncedSequenceEntity {
    pub _glacier_base: super::entity::SequenceEntity,
}

pub trait ServerSyncedSequenceEntityTrait: super::entity::SequenceEntityTrait {
}

impl ServerSyncedSequenceEntityTrait for ServerSyncedSequenceEntity {
}

impl super::entity::SequenceEntityTrait for ServerSyncedSequenceEntity {
}

impl super::entity::EntityTrait for ServerSyncedSequenceEntity {
}

impl super::entity::EntityBusPeerTrait for ServerSyncedSequenceEntity {
}

pub static SERVERSYNCEDSEQUENCEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSyncedSequenceEntity",
    name_hash: 248206370,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SEQUENCEENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerSyncedSequenceEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSyncedSequenceEntity as Default>::default())),
            create_boxed: || Box::new(<ServerSyncedSequenceEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSYNCEDSEQUENCEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSyncedSequenceEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSYNCEDSEQUENCEENTITY_TYPE_INFO
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


pub static SERVERSYNCEDSEQUENCEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSyncedSequenceEntity-Array",
    name_hash: 3899708054,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerSyncedSequenceEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerSpeedEventGateEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerSpeedEventGateEntityTrait: super::entity::EntityTrait {
}

impl ServerSpeedEventGateEntityTrait for ServerSpeedEventGateEntity {
}

impl super::entity::EntityTrait for ServerSpeedEventGateEntity {
}

impl super::entity::EntityBusPeerTrait for ServerSpeedEventGateEntity {
}

pub static SERVERSPEEDEVENTGATEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSpeedEventGateEntity",
    name_hash: 54088999,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerSpeedEventGateEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSpeedEventGateEntity as Default>::default())),
            create_boxed: || Box::new(<ServerSpeedEventGateEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSPEEDEVENTGATEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSpeedEventGateEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSPEEDEVENTGATEENTITY_TYPE_INFO
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


pub static SERVERSPEEDEVENTGATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSpeedEventGateEntity-Array",
    name_hash: 3887236755,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerSpeedEventGateEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerSaveGameLoadedEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerSaveGameLoadedEntityTrait: super::entity::EntityTrait {
}

impl ServerSaveGameLoadedEntityTrait for ServerSaveGameLoadedEntity {
}

impl super::entity::EntityTrait for ServerSaveGameLoadedEntity {
}

impl super::entity::EntityBusPeerTrait for ServerSaveGameLoadedEntity {
}

pub static SERVERSAVEGAMELOADEDENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSaveGameLoadedEntity",
    name_hash: 1969990195,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerSaveGameLoadedEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSaveGameLoadedEntity as Default>::default())),
            create_boxed: || Box::new(<ServerSaveGameLoadedEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSAVEGAMELOADEDENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSaveGameLoadedEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSAVEGAMELOADEDENTITY_TYPE_INFO
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


pub static SERVERSAVEGAMELOADEDENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSaveGameLoadedEntity-Array",
    name_hash: 2078520711,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerSaveGameLoadedEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerSaveEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerSaveEntityTrait: super::entity::EntityTrait {
}

impl ServerSaveEntityTrait for ServerSaveEntity {
}

impl super::entity::EntityTrait for ServerSaveEntity {
}

impl super::entity::EntityBusPeerTrait for ServerSaveEntity {
}

pub static SERVERSAVEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSaveEntity",
    name_hash: 1642661306,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerSaveEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSaveEntity as Default>::default())),
            create_boxed: || Box::new(<ServerSaveEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSAVEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSaveEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSAVEENTITY_TYPE_INFO
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


pub static SERVERSAVEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSaveEntity-Array",
    name_hash: 4034906126,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerSaveEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerPlayerIteratorEntity {
    pub _glacier_base: super::game_common::PlayerIteratorEntity,
}

pub trait ServerPlayerIteratorEntityTrait: super::game_common::PlayerIteratorEntityTrait {
}

impl ServerPlayerIteratorEntityTrait for ServerPlayerIteratorEntity {
}

impl super::game_common::PlayerIteratorEntityTrait for ServerPlayerIteratorEntity {
}

impl super::entity::EntityTrait for ServerPlayerIteratorEntity {
}

impl super::entity::EntityBusPeerTrait for ServerPlayerIteratorEntity {
}

pub static SERVERPLAYERITERATORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerIteratorEntity",
    name_hash: 1849610282,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_common::PLAYERITERATORENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerPlayerIteratorEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPlayerIteratorEntity as Default>::default())),
            create_boxed: || Box::new(<ServerPlayerIteratorEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERPLAYERITERATORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPlayerIteratorEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPLAYERITERATORENTITY_TYPE_INFO
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


pub static SERVERPLAYERITERATORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerIteratorEntity-Array",
    name_hash: 1242606238,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerPlayerIteratorEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerPlayerFilterEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerPlayerFilterEntityTrait: super::entity::EntityTrait {
}

impl ServerPlayerFilterEntityTrait for ServerPlayerFilterEntity {
}

impl super::entity::EntityTrait for ServerPlayerFilterEntity {
}

impl super::entity::EntityBusPeerTrait for ServerPlayerFilterEntity {
}

pub static SERVERPLAYERFILTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerFilterEntity",
    name_hash: 256907720,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerPlayerFilterEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPlayerFilterEntity as Default>::default())),
            create_boxed: || Box::new(<ServerPlayerFilterEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERPLAYERFILTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPlayerFilterEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPLAYERFILTERENTITY_TYPE_INFO
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


pub static SERVERPLAYERFILTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerFilterEntity-Array",
    name_hash: 651062396,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerPlayerFilterEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerObjectiveEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerObjectiveEntityTrait: super::entity::EntityTrait {
}

impl ServerObjectiveEntityTrait for ServerObjectiveEntity {
}

impl super::entity::EntityTrait for ServerObjectiveEntity {
}

impl super::entity::EntityBusPeerTrait for ServerObjectiveEntity {
}

pub static SERVEROBJECTIVEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerObjectiveEntity",
    name_hash: 1467175188,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerObjectiveEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerObjectiveEntity as Default>::default())),
            create_boxed: || Box::new(<ServerObjectiveEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVEROBJECTIVEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerObjectiveEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVEROBJECTIVEENTITY_TYPE_INFO
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


pub static SERVEROBJECTIVEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerObjectiveEntity-Array",
    name_hash: 1683828000,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerObjectiveEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerObjectAreaQueryEntity {
    pub _glacier_base: super::game_common::ObjectAreaQueryEntity,
}

pub trait ServerObjectAreaQueryEntityTrait: super::game_common::ObjectAreaQueryEntityTrait {
}

impl ServerObjectAreaQueryEntityTrait for ServerObjectAreaQueryEntity {
}

impl super::game_common::ObjectAreaQueryEntityTrait for ServerObjectAreaQueryEntity {
}

impl super::entity::EntityTrait for ServerObjectAreaQueryEntity {
}

impl super::entity::EntityBusPeerTrait for ServerObjectAreaQueryEntity {
}

pub static SERVEROBJECTAREAQUERYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerObjectAreaQueryEntity",
    name_hash: 2939796435,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_common::OBJECTAREAQUERYENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerObjectAreaQueryEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerObjectAreaQueryEntity as Default>::default())),
            create_boxed: || Box::new(<ServerObjectAreaQueryEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVEROBJECTAREAQUERYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerObjectAreaQueryEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVEROBJECTAREAQUERYENTITY_TYPE_INFO
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


pub static SERVEROBJECTAREAQUERYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerObjectAreaQueryEntity-Array",
    name_hash: 2095843047,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerObjectAreaQueryEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerMapMarkerEntity {
    pub _glacier_base: super::entity::SpatialEntity,
}

pub trait ServerMapMarkerEntityTrait: super::entity::SpatialEntityTrait {
}

impl ServerMapMarkerEntityTrait for ServerMapMarkerEntity {
}

impl super::entity::SpatialEntityTrait for ServerMapMarkerEntity {
}

impl super::entity::EntityTrait for ServerMapMarkerEntity {
}

impl super::entity::EntityBusPeerTrait for ServerMapMarkerEntity {
}

pub static SERVERMAPMARKERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMapMarkerEntity",
    name_hash: 2608266277,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerMapMarkerEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerMapMarkerEntity as Default>::default())),
            create_boxed: || Box::new(<ServerMapMarkerEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERMAPMARKERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerMapMarkerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERMAPMARKERENTITY_TYPE_INFO
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


pub static SERVERMAPMARKERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMapMarkerEntity-Array",
    name_hash: 2635658385,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerMapMarkerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerLevelControlEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerLevelControlEntityTrait: super::entity::EntityTrait {
}

impl ServerLevelControlEntityTrait for ServerLevelControlEntity {
}

impl super::entity::EntityTrait for ServerLevelControlEntity {
}

impl super::entity::EntityBusPeerTrait for ServerLevelControlEntity {
}

pub static SERVERLEVELCONTROLENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLevelControlEntity",
    name_hash: 733815082,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerLevelControlEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerLevelControlEntity as Default>::default())),
            create_boxed: || Box::new(<ServerLevelControlEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERLEVELCONTROLENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerLevelControlEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERLEVELCONTROLENTITY_TYPE_INFO
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


pub static SERVERLEVELCONTROLENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLevelControlEntity-Array",
    name_hash: 3678911390,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerLevelControlEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerInputRestrictionEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerInputRestrictionEntityTrait: super::entity::EntityTrait {
}

impl ServerInputRestrictionEntityTrait for ServerInputRestrictionEntity {
}

impl super::entity::EntityTrait for ServerInputRestrictionEntity {
}

impl super::entity::EntityBusPeerTrait for ServerInputRestrictionEntity {
}

pub static SERVERINPUTRESTRICTIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerInputRestrictionEntity",
    name_hash: 1710058553,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerInputRestrictionEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerInputRestrictionEntity as Default>::default())),
            create_boxed: || Box::new(<ServerInputRestrictionEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERINPUTRESTRICTIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerInputRestrictionEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERINPUTRESTRICTIONENTITY_TYPE_INFO
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


pub static SERVERINPUTRESTRICTIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerInputRestrictionEntity-Array",
    name_hash: 1400232589,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerInputRestrictionEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerHumanPlayerEntity {
    pub _glacier_base: ServerHumanPlayerProxyEntity,
}

pub trait ServerHumanPlayerEntityTrait: ServerHumanPlayerProxyEntityTrait {
}

impl ServerHumanPlayerEntityTrait for ServerHumanPlayerEntity {
}

impl ServerHumanPlayerProxyEntityTrait for ServerHumanPlayerEntity {
}

impl super::entity::EntityTrait for ServerHumanPlayerEntity {
}

impl super::entity::EntityBusPeerTrait for ServerHumanPlayerEntity {
}

pub static SERVERHUMANPLAYERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerHumanPlayerEntity",
    name_hash: 1686106007,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERHUMANPLAYERPROXYENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerHumanPlayerEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerHumanPlayerEntity as Default>::default())),
            create_boxed: || Box::new(<ServerHumanPlayerEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERHUMANPLAYERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerHumanPlayerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERHUMANPLAYERENTITY_TYPE_INFO
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


pub static SERVERHUMANPLAYERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerHumanPlayerEntity-Array",
    name_hash: 1326053411,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerHumanPlayerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerHumanPlayerProxyEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerHumanPlayerProxyEntityTrait: super::entity::EntityTrait {
}

impl ServerHumanPlayerProxyEntityTrait for ServerHumanPlayerProxyEntity {
}

impl super::entity::EntityTrait for ServerHumanPlayerProxyEntity {
}

impl super::entity::EntityBusPeerTrait for ServerHumanPlayerProxyEntity {
}

pub static SERVERHUMANPLAYERPROXYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerHumanPlayerProxyEntity",
    name_hash: 1414146843,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerHumanPlayerProxyEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerHumanPlayerProxyEntity as Default>::default())),
            create_boxed: || Box::new(<ServerHumanPlayerProxyEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERHUMANPLAYERPROXYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerHumanPlayerProxyEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERHUMANPLAYERPROXYENTITY_TYPE_INFO
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


pub static SERVERHUMANPLAYERPROXYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerHumanPlayerProxyEntity-Array",
    name_hash: 3098706351,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerHumanPlayerProxyEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerEventMemoryEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerEventMemoryEntityTrait: super::entity::EntityTrait {
}

impl ServerEventMemoryEntityTrait for ServerEventMemoryEntity {
}

impl super::entity::EntityTrait for ServerEventMemoryEntity {
}

impl super::entity::EntityBusPeerTrait for ServerEventMemoryEntity {
}

pub static SERVEREVENTMEMORYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEventMemoryEntity",
    name_hash: 2861803094,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerEventMemoryEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerEventMemoryEntity as Default>::default())),
            create_boxed: || Box::new(<ServerEventMemoryEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVEREVENTMEMORYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerEventMemoryEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVEREVENTMEMORYENTITY_TYPE_INFO
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


pub static SERVEREVENTMEMORYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEventMemoryEntity-Array",
    name_hash: 2268392418,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerEventMemoryEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerEventIfSwitchEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerEventIfSwitchEntityTrait: super::entity::EntityTrait {
}

impl ServerEventIfSwitchEntityTrait for ServerEventIfSwitchEntity {
}

impl super::entity::EntityTrait for ServerEventIfSwitchEntity {
}

impl super::entity::EntityBusPeerTrait for ServerEventIfSwitchEntity {
}

pub static SERVEREVENTIFSWITCHENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEventIfSwitchEntity",
    name_hash: 3484792874,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerEventIfSwitchEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerEventIfSwitchEntity as Default>::default())),
            create_boxed: || Box::new(<ServerEventIfSwitchEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVEREVENTIFSWITCHENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerEventIfSwitchEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVEREVENTIFSWITCHENTITY_TYPE_INFO
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


pub static SERVEREVENTIFSWITCHENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEventIfSwitchEntity-Array",
    name_hash: 857051806,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerEventIfSwitchEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerCustomizeCharacterEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerCustomizeCharacterEntityTrait: super::entity::EntityTrait {
}

impl ServerCustomizeCharacterEntityTrait for ServerCustomizeCharacterEntity {
}

impl super::entity::EntityTrait for ServerCustomizeCharacterEntity {
}

impl super::entity::EntityBusPeerTrait for ServerCustomizeCharacterEntity {
}

pub static SERVERCUSTOMIZECHARACTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCustomizeCharacterEntity",
    name_hash: 702791175,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerCustomizeCharacterEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCustomizeCharacterEntity as Default>::default())),
            create_boxed: || Box::new(<ServerCustomizeCharacterEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCUSTOMIZECHARACTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCustomizeCharacterEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCUSTOMIZECHARACTERENTITY_TYPE_INFO
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


pub static SERVERCUSTOMIZECHARACTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCustomizeCharacterEntity-Array",
    name_hash: 1562107571,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerCustomizeCharacterEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerAreaQueryEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerAreaQueryEntityTrait: super::entity::EntityTrait {
}

impl ServerAreaQueryEntityTrait for ServerAreaQueryEntity {
}

impl super::entity::EntityTrait for ServerAreaQueryEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAreaQueryEntity {
}

pub static SERVERAREAQUERYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAreaQueryEntity",
    name_hash: 3911119046,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAreaQueryEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAreaQueryEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAreaQueryEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAREAQUERYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAreaQueryEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAREAQUERYENTITY_TYPE_INFO
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


pub static SERVERAREAQUERYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAreaQueryEntity-Array",
    name_hash: 1159417458,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerAreaQueryEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerPlayerEvent {
    pub _glacier_base: super::gameplay_sim::PlayerEventBase,
}

pub trait ServerPlayerEventTrait: super::gameplay_sim::PlayerEventBaseTrait {
}

impl ServerPlayerEventTrait for ServerPlayerEvent {
}

impl super::gameplay_sim::PlayerEventBaseTrait for ServerPlayerEvent {
}

impl super::entity::EntityEventTrait for ServerPlayerEvent {
}

pub static SERVERPLAYEREVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerEvent",
    name_hash: 4111026559,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_sim::PLAYEREVENTBASE_TYPE_INFO),
        super_class_offset: offset_of!(ServerPlayerEvent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPlayerEvent as Default>::default())),
            create_boxed: || Box::new(<ServerPlayerEvent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERPLAYEREVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPlayerEvent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPLAYEREVENT_TYPE_INFO
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


pub static SERVERPLAYEREVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerEvent-Array",
    name_hash: 2483998539,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerPlayerEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerDoublePlayerEvent {
    pub _glacier_base: ServerPlayerEvent,
}

pub trait ServerDoublePlayerEventTrait: ServerPlayerEventTrait {
}

impl ServerDoublePlayerEventTrait for ServerDoublePlayerEvent {
}

impl ServerPlayerEventTrait for ServerDoublePlayerEvent {
}

impl super::gameplay_sim::PlayerEventBaseTrait for ServerDoublePlayerEvent {
}

impl super::entity::EntityEventTrait for ServerDoublePlayerEvent {
}

pub static SERVERDOUBLEPLAYEREVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDoublePlayerEvent",
    name_hash: 3866715274,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERPLAYEREVENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerDoublePlayerEvent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerDoublePlayerEvent as Default>::default())),
            create_boxed: || Box::new(<ServerDoublePlayerEvent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERDOUBLEPLAYEREVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerDoublePlayerEvent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERDOUBLEPLAYEREVENT_TYPE_INFO
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


pub static SERVERDOUBLEPLAYEREVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDoublePlayerEvent-Array",
    name_hash: 2598174654,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerDoublePlayerEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerDamageGiverEvent {
    pub _glacier_base: ServerPlayerEvent,
}

pub trait ServerDamageGiverEventTrait: ServerPlayerEventTrait {
}

impl ServerDamageGiverEventTrait for ServerDamageGiverEvent {
}

impl ServerPlayerEventTrait for ServerDamageGiverEvent {
}

impl super::gameplay_sim::PlayerEventBaseTrait for ServerDamageGiverEvent {
}

impl super::entity::EntityEventTrait for ServerDamageGiverEvent {
}

pub static SERVERDAMAGEGIVEREVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDamageGiverEvent",
    name_hash: 3902893416,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERPLAYEREVENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerDamageGiverEvent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerDamageGiverEvent as Default>::default())),
            create_boxed: || Box::new(<ServerDamageGiverEvent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERDAMAGEGIVEREVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerDamageGiverEvent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERDAMAGEGIVEREVENT_TYPE_INFO
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


pub static SERVERDAMAGEGIVEREVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDamageGiverEvent-Array",
    name_hash: 2563227484,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerDamageGiverEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerPredestructionEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerPredestructionEntityTrait: super::entity::EntityTrait {
}

impl ServerPredestructionEntityTrait for ServerPredestructionEntity {
}

impl super::entity::EntityTrait for ServerPredestructionEntity {
}

impl super::entity::EntityBusPeerTrait for ServerPredestructionEntity {
}

pub static SERVERPREDESTRUCTIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPredestructionEntity",
    name_hash: 2880150722,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerPredestructionEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPredestructionEntity as Default>::default())),
            create_boxed: || Box::new(<ServerPredestructionEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERPREDESTRUCTIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPredestructionEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPREDESTRUCTIONENTITY_TYPE_INFO
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


pub static SERVERPREDESTRUCTIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPredestructionEntity-Array",
    name_hash: 3224980598,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerPredestructionEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerBangerPhysicsComponent {
    pub _glacier_base: super::physics::PartPhysicsComponent,
}

pub trait ServerBangerPhysicsComponentTrait: super::physics::PartPhysicsComponentTrait {
}

impl ServerBangerPhysicsComponentTrait for ServerBangerPhysicsComponent {
}

impl super::physics::PartPhysicsComponentTrait for ServerBangerPhysicsComponent {
}

impl super::physics::PhysicsComponentTrait for ServerBangerPhysicsComponent {
}

impl super::entity::ComponentTrait for ServerBangerPhysicsComponent {
}

impl super::entity::EntityBusPeerTrait for ServerBangerPhysicsComponent {
}

pub static SERVERBANGERPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBangerPhysicsComponent",
    name_hash: 2563328761,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::PARTPHYSICSCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerBangerPhysicsComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerBangerPhysicsComponent as Default>::default())),
            create_boxed: || Box::new(<ServerBangerPhysicsComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERBANGERPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerBangerPhysicsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERBANGERPHYSICSCOMPONENT_TYPE_INFO
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


pub static SERVERBANGERPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBangerPhysicsComponent-Array",
    name_hash: 3292400077,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerBangerPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerBangerHealthModule {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerBangerHealthModuleTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerBangerHealthModuleTrait for ServerBangerHealthModule {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerBangerHealthModule {
}

impl super::gameplay_sim::GameComponentTrait for ServerBangerHealthModule {
}

impl super::entity::ComponentTrait for ServerBangerHealthModule {
}

impl super::entity::EntityBusPeerTrait for ServerBangerHealthModule {
}

pub static SERVERBANGERHEALTHMODULE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBangerHealthModule",
    name_hash: 1404665531,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerBangerHealthModule, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerBangerHealthModule as Default>::default())),
            create_boxed: || Box::new(<ServerBangerHealthModule as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERBANGERHEALTHMODULE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerBangerHealthModule {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERBANGERHEALTHMODULE_TYPE_INFO
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


pub static SERVERBANGERHEALTHMODULE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBangerHealthModule-Array",
    name_hash: 433997327,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerBangerHealthModule"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerBangerHealthComponent {
    pub _glacier_base: ServerGameHealthComponent,
}

pub trait ServerBangerHealthComponentTrait: ServerGameHealthComponentTrait {
}

impl ServerBangerHealthComponentTrait for ServerBangerHealthComponent {
}

impl ServerGameHealthComponentTrait for ServerBangerHealthComponent {
}

impl super::gameplay_sim::HealthComponentTrait for ServerBangerHealthComponent {
}

impl super::entity::ComponentTrait for ServerBangerHealthComponent {
}

impl super::entity::EntityBusPeerTrait for ServerBangerHealthComponent {
}

pub static SERVERBANGERHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBangerHealthComponent",
    name_hash: 2605561582,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMEHEALTHCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerBangerHealthComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerBangerHealthComponent as Default>::default())),
            create_boxed: || Box::new(<ServerBangerHealthComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERBANGERHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerBangerHealthComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERBANGERHEALTHCOMPONENT_TYPE_INFO
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


pub static SERVERBANGERHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBangerHealthComponent-Array",
    name_hash: 802230234,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerBangerHealthComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerSubView {
    pub _glacier_base: super::gameplay_sim::SubView,
}

pub trait ServerSubViewTrait: super::gameplay_sim::SubViewTrait {
}

impl ServerSubViewTrait for ServerSubView {
}

impl super::gameplay_sim::SubViewTrait for ServerSubView {
}

pub static SERVERSUBVIEW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSubView",
    name_hash: 2702664841,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_sim::SUBVIEW_TYPE_INFO),
        super_class_offset: offset_of!(ServerSubView, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSubView as Default>::default())),
            create_boxed: || Box::new(<ServerSubView as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSUBVIEW_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSubView {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSUBVIEW_TYPE_INFO
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


pub static SERVERSUBVIEW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSubView-Array",
    name_hash: 1981389373,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerSubView"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerSpectatorSubView {
    pub _glacier_base: ServerSpectatorSubViewBase,
}

pub trait ServerSpectatorSubViewTrait: ServerSpectatorSubViewBaseTrait {
}

impl ServerSpectatorSubViewTrait for ServerSpectatorSubView {
}

impl ServerSpectatorSubViewBaseTrait for ServerSpectatorSubView {
}

impl ServerSubViewTrait for ServerSpectatorSubView {
}

impl super::gameplay_sim::SubViewTrait for ServerSpectatorSubView {
}

pub static SERVERSPECTATORSUBVIEW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSpectatorSubView",
    name_hash: 843954960,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERSPECTATORSUBVIEWBASE_TYPE_INFO),
        super_class_offset: offset_of!(ServerSpectatorSubView, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSpectatorSubView as Default>::default())),
            create_boxed: || Box::new(<ServerSpectatorSubView as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSPECTATORSUBVIEW_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSpectatorSubView {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSPECTATORSUBVIEW_TYPE_INFO
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


pub static SERVERSPECTATORSUBVIEW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSpectatorSubView-Array",
    name_hash: 4137642276,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerSpectatorSubView"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerSpectatorSubViewBase {
    pub _glacier_base: ServerSubView,
}

pub trait ServerSpectatorSubViewBaseTrait: ServerSubViewTrait {
}

impl ServerSpectatorSubViewBaseTrait for ServerSpectatorSubViewBase {
}

impl ServerSubViewTrait for ServerSpectatorSubViewBase {
}

impl super::gameplay_sim::SubViewTrait for ServerSpectatorSubViewBase {
}

pub static SERVERSPECTATORSUBVIEWBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSpectatorSubViewBase",
    name_hash: 3388639813,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERSUBVIEW_TYPE_INFO),
        super_class_offset: offset_of!(ServerSpectatorSubViewBase, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSpectatorSubViewBase as Default>::default())),
            create_boxed: || Box::new(<ServerSpectatorSubViewBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSPECTATORSUBVIEWBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSpectatorSubViewBase {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSPECTATORSUBVIEWBASE_TYPE_INFO
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


pub static SERVERSPECTATORSUBVIEWBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSpectatorSubViewBase-Array",
    name_hash: 967729777,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerSpectatorSubViewBase"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerGameSubView {
    pub _glacier_base: ServerSubView,
}

pub trait ServerGameSubViewTrait: ServerSubViewTrait {
}

impl ServerGameSubViewTrait for ServerGameSubView {
}

impl ServerSubViewTrait for ServerGameSubView {
}

impl super::gameplay_sim::SubViewTrait for ServerGameSubView {
}

pub static SERVERGAMESUBVIEW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameSubView",
    name_hash: 3560962407,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERSUBVIEW_TYPE_INFO),
        super_class_offset: offset_of!(ServerGameSubView, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGameSubView as Default>::default())),
            create_boxed: || Box::new(<ServerGameSubView as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERGAMESUBVIEW_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerGameSubView {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGAMESUBVIEW_TYPE_INFO
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


pub static SERVERGAMESUBVIEW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameSubView-Array",
    name_hash: 2071419731,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerGameSubView"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerConnection {
    pub _glacier_base: super::network::EngineConnectionPeer,
}

pub trait ServerConnectionTrait: super::network::EngineConnectionPeerTrait {
}

impl ServerConnectionTrait for ServerConnection {
}

impl super::network::EngineConnectionPeerTrait for ServerConnection {
}

impl super::network::EngineConnectionTrait for ServerConnection {
}

pub static SERVERCONNECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerConnection",
    name_hash: 3928021046,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::network::ENGINECONNECTIONPEER_TYPE_INFO),
        super_class_offset: offset_of!(ServerConnection, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerConnection as Default>::default())),
            create_boxed: || Box::new(<ServerConnection as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCONNECTION_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerConnection {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCONNECTION_TYPE_INFO
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


pub static SERVERCONNECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerConnection-Array",
    name_hash: 665570434,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerConnection"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerVehicleStateTriggerEntity {
    pub _glacier_base: ServerTriggerEntity,
}

pub trait ServerVehicleStateTriggerEntityTrait: ServerTriggerEntityTrait {
}

impl ServerVehicleStateTriggerEntityTrait for ServerVehicleStateTriggerEntity {
}

impl ServerTriggerEntityTrait for ServerVehicleStateTriggerEntity {
}

impl super::entity::EntityTrait for ServerVehicleStateTriggerEntity {
}

impl super::entity::EntityBusPeerTrait for ServerVehicleStateTriggerEntity {
}

pub static SERVERVEHICLESTATETRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleStateTriggerEntity",
    name_hash: 2155831820,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERTRIGGERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerVehicleStateTriggerEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerVehicleStateTriggerEntity as Default>::default())),
            create_boxed: || Box::new(<ServerVehicleStateTriggerEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERVEHICLESTATETRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerVehicleStateTriggerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERVEHICLESTATETRIGGERENTITY_TYPE_INFO
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


pub static SERVERVEHICLESTATETRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleStateTriggerEntity-Array",
    name_hash: 928944696,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerVehicleStateTriggerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerUnderFireTriggerEntity {
    pub _glacier_base: ServerTriggerEntity,
}

pub trait ServerUnderFireTriggerEntityTrait: ServerTriggerEntityTrait {
}

impl ServerUnderFireTriggerEntityTrait for ServerUnderFireTriggerEntity {
}

impl ServerTriggerEntityTrait for ServerUnderFireTriggerEntity {
}

impl super::entity::EntityTrait for ServerUnderFireTriggerEntity {
}

impl super::entity::EntityBusPeerTrait for ServerUnderFireTriggerEntity {
}

pub static SERVERUNDERFIRETRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerUnderFireTriggerEntity",
    name_hash: 3284172467,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERTRIGGERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerUnderFireTriggerEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerUnderFireTriggerEntity as Default>::default())),
            create_boxed: || Box::new(<ServerUnderFireTriggerEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERUNDERFIRETRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerUnderFireTriggerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERUNDERFIRETRIGGERENTITY_TYPE_INFO
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


pub static SERVERUNDERFIRETRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerUnderFireTriggerEntity-Array",
    name_hash: 4037405191,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerUnderFireTriggerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerCharacterLookAtTriggerEntity {
    pub _glacier_base: ServerTriggerEntity,
}

pub trait ServerCharacterLookAtTriggerEntityTrait: ServerTriggerEntityTrait {
}

impl ServerCharacterLookAtTriggerEntityTrait for ServerCharacterLookAtTriggerEntity {
}

impl ServerTriggerEntityTrait for ServerCharacterLookAtTriggerEntity {
}

impl super::entity::EntityTrait for ServerCharacterLookAtTriggerEntity {
}

impl super::entity::EntityBusPeerTrait for ServerCharacterLookAtTriggerEntity {
}

pub static SERVERCHARACTERLOOKATTRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterLookAtTriggerEntity",
    name_hash: 2680361672,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERTRIGGERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerCharacterLookAtTriggerEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCharacterLookAtTriggerEntity as Default>::default())),
            create_boxed: || Box::new(<ServerCharacterLookAtTriggerEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCHARACTERLOOKATTRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCharacterLookAtTriggerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCHARACTERLOOKATTRIGGERENTITY_TYPE_INFO
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


pub static SERVERCHARACTERLOOKATTRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterLookAtTriggerEntity-Array",
    name_hash: 702031228,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerCharacterLookAtTriggerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerTriggerEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerTriggerEntityTrait: super::entity::EntityTrait {
}

impl ServerTriggerEntityTrait for ServerTriggerEntity {
}

impl super::entity::EntityTrait for ServerTriggerEntity {
}

impl super::entity::EntityBusPeerTrait for ServerTriggerEntity {
}

pub static SERVERTRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTriggerEntity",
    name_hash: 2601801827,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerTriggerEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerTriggerEntity as Default>::default())),
            create_boxed: || Box::new(<ServerTriggerEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERTRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerTriggerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERTRIGGERENTITY_TYPE_INFO
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


pub static SERVERTRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTriggerEntity-Array",
    name_hash: 3432467543,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerTriggerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerPlayerTakeOverTriggerEntity {
    pub _glacier_base: super::entity::SpatialEntity,
}

pub trait ServerPlayerTakeOverTriggerEntityTrait: super::entity::SpatialEntityTrait {
}

impl ServerPlayerTakeOverTriggerEntityTrait for ServerPlayerTakeOverTriggerEntity {
}

impl super::entity::SpatialEntityTrait for ServerPlayerTakeOverTriggerEntity {
}

impl super::entity::EntityTrait for ServerPlayerTakeOverTriggerEntity {
}

impl super::entity::EntityBusPeerTrait for ServerPlayerTakeOverTriggerEntity {
}

pub static SERVERPLAYERTAKEOVERTRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerTakeOverTriggerEntity",
    name_hash: 3034856485,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerPlayerTakeOverTriggerEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPlayerTakeOverTriggerEntity as Default>::default())),
            create_boxed: || Box::new(<ServerPlayerTakeOverTriggerEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERPLAYERTAKEOVERTRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPlayerTakeOverTriggerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPLAYERTAKEOVERTRIGGERENTITY_TYPE_INFO
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


pub static SERVERPLAYERTAKEOVERTRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerTakeOverTriggerEntity-Array",
    name_hash: 3816174737,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerPlayerTakeOverTriggerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerChildComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerChildComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerChildComponentTrait for ServerChildComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerChildComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerChildComponent {
}

impl super::entity::ComponentTrait for ServerChildComponent {
}

impl super::entity::EntityBusPeerTrait for ServerChildComponent {
}

pub static SERVERCHILDCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerChildComponent",
    name_hash: 1207385157,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerChildComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerChildComponent as Default>::default())),
            create_boxed: || Box::new(<ServerChildComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCHILDCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerChildComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCHILDCOMPONENT_TYPE_INFO
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


pub static SERVERCHILDCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerChildComponent-Array",
    name_hash: 1122538609,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerChildComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerChildBarrelComponent {
    pub _glacier_base: ServerChildComponent,
}

pub trait ServerChildBarrelComponentTrait: ServerChildComponentTrait {
}

impl ServerChildBarrelComponentTrait for ServerChildBarrelComponent {
}

impl ServerChildComponentTrait for ServerChildBarrelComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerChildBarrelComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerChildBarrelComponent {
}

impl super::entity::ComponentTrait for ServerChildBarrelComponent {
}

impl super::entity::EntityBusPeerTrait for ServerChildBarrelComponent {
}

pub static SERVERCHILDBARRELCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerChildBarrelComponent",
    name_hash: 2072029327,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERCHILDCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerChildBarrelComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerChildBarrelComponent as Default>::default())),
            create_boxed: || Box::new(<ServerChildBarrelComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCHILDBARRELCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerChildBarrelComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCHILDBARRELCOMPONENT_TYPE_INFO
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


pub static SERVERCHILDBARRELCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerChildBarrelComponent-Array",
    name_hash: 4007527739,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerChildBarrelComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerChassisComponent {
    pub _glacier_base: ServerPartComponent,
}

pub trait ServerChassisComponentTrait: ServerPartComponentTrait {
}

impl ServerChassisComponentTrait for ServerChassisComponent {
}

impl ServerPartComponentTrait for ServerChassisComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerChassisComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerChassisComponent {
}

impl super::entity::ComponentTrait for ServerChassisComponent {
}

impl super::entity::EntityBusPeerTrait for ServerChassisComponent {
}

pub static SERVERCHASSISCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerChassisComponent",
    name_hash: 3963530399,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERPARTCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerChassisComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerChassisComponent as Default>::default())),
            create_boxed: || Box::new(<ServerChassisComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCHASSISCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerChassisComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCHASSISCOMPONENT_TYPE_INFO
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


pub static SERVERCHASSISCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerChassisComponent-Array",
    name_hash: 782356779,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerChassisComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerCameraComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerCameraComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerCameraComponentTrait for ServerCameraComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerCameraComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerCameraComponent {
}

impl super::entity::ComponentTrait for ServerCameraComponent {
}

impl super::entity::EntityBusPeerTrait for ServerCameraComponent {
}

pub static SERVERCAMERACOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCameraComponent",
    name_hash: 361934422,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerCameraComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCameraComponent as Default>::default())),
            create_boxed: || Box::new(<ServerCameraComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCAMERACOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCameraComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCAMERACOMPONENT_TYPE_INFO
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


pub static SERVERCAMERACOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCameraComponent-Array",
    name_hash: 3344561122,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerCameraComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerPlayerExtent {
}

pub trait ServerPlayerExtentTrait: TypeObject {
}

impl ServerPlayerExtentTrait for ServerPlayerExtent {
}

pub static SERVERPLAYEREXTENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerExtent",
    name_hash: 2541409925,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPlayerExtent as Default>::default())),
            create_boxed: || Box::new(<ServerPlayerExtent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERPLAYEREXTENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPlayerExtent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPLAYEREXTENT_TYPE_INFO
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


pub static SERVERPLAYEREXTENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerExtent-Array",
    name_hash: 3096108593,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerPlayerExtent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerGamePlayerExtent {
    pub _glacier_base: ServerPlayerExtent,
}

pub trait ServerGamePlayerExtentTrait: ServerPlayerExtentTrait {
}

impl ServerGamePlayerExtentTrait for ServerGamePlayerExtent {
}

impl ServerPlayerExtentTrait for ServerGamePlayerExtent {
}

pub static SERVERGAMEPLAYEREXTENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGamePlayerExtent",
    name_hash: 4073287147,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERPLAYEREXTENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerGamePlayerExtent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGamePlayerExtent as Default>::default())),
            create_boxed: || Box::new(<ServerGamePlayerExtent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERGAMEPLAYEREXTENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerGamePlayerExtent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGAMEPLAYEREXTENT_TYPE_INFO
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


pub static SERVERGAMEPLAYEREXTENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGamePlayerExtent-Array",
    name_hash: 2661670367,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerGamePlayerExtent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerGamePlayerInternalExtent {
    pub _glacier_base: ServerPlayerExtent,
}

pub trait ServerGamePlayerInternalExtentTrait: ServerPlayerExtentTrait {
}

impl ServerGamePlayerInternalExtentTrait for ServerGamePlayerInternalExtent {
}

impl ServerPlayerExtentTrait for ServerGamePlayerInternalExtent {
}

pub static SERVERGAMEPLAYERINTERNALEXTENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGamePlayerInternalExtent",
    name_hash: 3816605260,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERPLAYEREXTENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerGamePlayerInternalExtent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGamePlayerInternalExtent as Default>::default())),
            create_boxed: || Box::new(<ServerGamePlayerInternalExtent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERGAMEPLAYERINTERNALEXTENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerGamePlayerInternalExtent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGAMEPLAYERINTERNALEXTENT_TYPE_INFO
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


pub static SERVERGAMEPLAYERINTERNALEXTENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGamePlayerInternalExtent-Array",
    name_hash: 724182776,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerGamePlayerInternalExtent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerGameSplineEntity {
    pub _glacier_base: super::entity::SpatialEntity,
}

pub trait ServerGameSplineEntityTrait: super::entity::SpatialEntityTrait {
}

impl ServerGameSplineEntityTrait for ServerGameSplineEntity {
}

impl super::entity::SpatialEntityTrait for ServerGameSplineEntity {
}

impl super::entity::EntityTrait for ServerGameSplineEntity {
}

impl super::entity::EntityBusPeerTrait for ServerGameSplineEntity {
}

pub static SERVERGAMESPLINEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameSplineEntity",
    name_hash: 3967131032,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerGameSplineEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGameSplineEntity as Default>::default())),
            create_boxed: || Box::new(<ServerGameSplineEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERGAMESPLINEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerGameSplineEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGAMESPLINEENTITY_TYPE_INFO
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


pub static SERVERGAMESPLINEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameSplineEntity-Array",
    name_hash: 457607084,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerGameSplineEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerAreaImmunityComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerAreaImmunityComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerAreaImmunityComponentTrait for ServerAreaImmunityComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerAreaImmunityComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerAreaImmunityComponent {
}

impl super::entity::ComponentTrait for ServerAreaImmunityComponent {
}

impl super::entity::EntityBusPeerTrait for ServerAreaImmunityComponent {
}

pub static SERVERAREAIMMUNITYCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAreaImmunityComponent",
    name_hash: 3978357998,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerAreaImmunityComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAreaImmunityComponent as Default>::default())),
            create_boxed: || Box::new(<ServerAreaImmunityComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAREAIMMUNITYCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAreaImmunityComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAREAIMMUNITYCOMPONENT_TYPE_INFO
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


pub static SERVERAREAIMMUNITYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAreaImmunityComponent-Array",
    name_hash: 481609178,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerAreaImmunityComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerDynamicFireEntity {
    pub _glacier_base: ServerGameComponentEntity,
}

pub trait ServerDynamicFireEntityTrait: ServerGameComponentEntityTrait {
}

impl ServerDynamicFireEntityTrait for ServerDynamicFireEntity {
}

impl ServerGameComponentEntityTrait for ServerDynamicFireEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ServerDynamicFireEntity {
}

impl super::entity::ComponentEntityTrait for ServerDynamicFireEntity {
}

impl super::entity::SpatialEntityTrait for ServerDynamicFireEntity {
}

impl super::entity::EntityTrait for ServerDynamicFireEntity {
}

impl super::entity::EntityBusPeerTrait for ServerDynamicFireEntity {
}

pub static SERVERDYNAMICFIREENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDynamicFireEntity",
    name_hash: 938874678,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENTENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerDynamicFireEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerDynamicFireEntity as Default>::default())),
            create_boxed: || Box::new(<ServerDynamicFireEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERDYNAMICFIREENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerDynamicFireEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERDYNAMICFIREENTITY_TYPE_INFO
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


pub static SERVERDYNAMICFIREENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDynamicFireEntity-Array",
    name_hash: 1446267778,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerDynamicFireEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerControllableEntity {
    pub _glacier_base: ServerPhysicsEntity,
}

pub trait ServerControllableEntityTrait: ServerPhysicsEntityTrait {
}

impl ServerControllableEntityTrait for ServerControllableEntity {
}

impl ServerPhysicsEntityTrait for ServerControllableEntity {
}

impl ServerGameComponentEntityTrait for ServerControllableEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ServerControllableEntity {
}

impl super::entity::ComponentEntityTrait for ServerControllableEntity {
}

impl super::entity::SpatialEntityTrait for ServerControllableEntity {
}

impl super::entity::EntityTrait for ServerControllableEntity {
}

impl super::entity::EntityBusPeerTrait for ServerControllableEntity {
}

pub static SERVERCONTROLLABLEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerControllableEntity",
    name_hash: 45698650,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERPHYSICSENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerControllableEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerControllableEntity as Default>::default())),
            create_boxed: || Box::new(<ServerControllableEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCONTROLLABLEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerControllableEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCONTROLLABLEENTITY_TYPE_INFO
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


pub static SERVERCONTROLLABLEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerControllableEntity-Array",
    name_hash: 3686894,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerControllableEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerWarningSystemComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerWarningSystemComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerWarningSystemComponentTrait for ServerWarningSystemComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerWarningSystemComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerWarningSystemComponent {
}

impl super::entity::ComponentTrait for ServerWarningSystemComponent {
}

impl super::entity::EntityBusPeerTrait for ServerWarningSystemComponent {
}

pub static SERVERWARNINGSYSTEMCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWarningSystemComponent",
    name_hash: 1310497088,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerWarningSystemComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerWarningSystemComponent as Default>::default())),
            create_boxed: || Box::new(<ServerWarningSystemComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERWARNINGSYSTEMCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerWarningSystemComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERWARNINGSYSTEMCOMPONENT_TYPE_INFO
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


pub static SERVERWARNINGSYSTEMCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWarningSystemComponent-Array",
    name_hash: 2908565492,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerWarningSystemComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerUnlockComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerUnlockComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerUnlockComponentTrait for ServerUnlockComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerUnlockComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerUnlockComponent {
}

impl super::entity::ComponentTrait for ServerUnlockComponent {
}

impl super::entity::EntityBusPeerTrait for ServerUnlockComponent {
}

pub static SERVERUNLOCKCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerUnlockComponent",
    name_hash: 2983573823,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerUnlockComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerUnlockComponent as Default>::default())),
            create_boxed: || Box::new(<ServerUnlockComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERUNLOCKCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerUnlockComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERUNLOCKCOMPONENT_TYPE_INFO
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


pub static SERVERUNLOCKCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerUnlockComponent-Array",
    name_hash: 3978038923,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerUnlockComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerPlayerEntryComponent {
    pub _glacier_base: ServerGameEntryComponent,
}

pub trait ServerPlayerEntryComponentTrait: ServerGameEntryComponentTrait {
}

impl ServerPlayerEntryComponentTrait for ServerPlayerEntryComponent {
}

impl ServerGameEntryComponentTrait for ServerPlayerEntryComponent {
}

impl ServerEntryComponentTrait for ServerPlayerEntryComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerPlayerEntryComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerPlayerEntryComponent {
}

impl super::entity::ComponentTrait for ServerPlayerEntryComponent {
}

impl super::entity::EntityBusPeerTrait for ServerPlayerEntryComponent {
}

pub static SERVERPLAYERENTRYCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerEntryComponent",
    name_hash: 2202942888,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMEENTRYCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerPlayerEntryComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPlayerEntryComponent as Default>::default())),
            create_boxed: || Box::new(<ServerPlayerEntryComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERPLAYERENTRYCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPlayerEntryComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPLAYERENTRYCOMPONENT_TYPE_INFO
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


pub static SERVERPLAYERENTRYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerEntryComponent-Array",
    name_hash: 488881948,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerPlayerEntryComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerGameEntryComponent {
    pub _glacier_base: ServerEntryComponent,
}

pub trait ServerGameEntryComponentTrait: ServerEntryComponentTrait {
}

impl ServerGameEntryComponentTrait for ServerGameEntryComponent {
}

impl ServerEntryComponentTrait for ServerGameEntryComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerGameEntryComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerGameEntryComponent {
}

impl super::entity::ComponentTrait for ServerGameEntryComponent {
}

impl super::entity::EntityBusPeerTrait for ServerGameEntryComponent {
}

pub static SERVERGAMEENTRYCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameEntryComponent",
    name_hash: 2902730101,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERENTRYCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerGameEntryComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGameEntryComponent as Default>::default())),
            create_boxed: || Box::new(<ServerGameEntryComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERGAMEENTRYCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerGameEntryComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGAMEENTRYCOMPONENT_TYPE_INFO
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


pub static SERVERGAMEENTRYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameEntryComponent-Array",
    name_hash: 261297729,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerGameEntryComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerSubLevelEntity {
    pub _glacier_base: super::entity::SubLevelEntity,
}

pub trait ServerSubLevelEntityTrait: super::entity::SubLevelEntityTrait {
}

impl ServerSubLevelEntityTrait for ServerSubLevelEntity {
}

impl super::entity::SubLevelEntityTrait for ServerSubLevelEntity {
}

impl super::entity::EntityTrait for ServerSubLevelEntity {
}

impl super::entity::EntityBusPeerTrait for ServerSubLevelEntity {
}

pub static SERVERSUBLEVELENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSubLevelEntity",
    name_hash: 2082044361,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SUBLEVELENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerSubLevelEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSubLevelEntity as Default>::default())),
            create_boxed: || Box::new(<ServerSubLevelEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSUBLEVELENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSubLevelEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSUBLEVELENTITY_TYPE_INFO
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


pub static SERVERSUBLEVELENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSubLevelEntity-Array",
    name_hash: 3345129469,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerSubLevelEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerSubLevelCollectionEntity {
    pub _glacier_base: super::game_common::SubLevelCollectionEntityBase,
}

pub trait ServerSubLevelCollectionEntityTrait: super::game_common::SubLevelCollectionEntityBaseTrait {
}

impl ServerSubLevelCollectionEntityTrait for ServerSubLevelCollectionEntity {
}

impl super::game_common::SubLevelCollectionEntityBaseTrait for ServerSubLevelCollectionEntity {
}

impl super::entity::EntityTrait for ServerSubLevelCollectionEntity {
}

impl super::entity::EntityBusPeerTrait for ServerSubLevelCollectionEntity {
}

pub static SERVERSUBLEVELCOLLECTIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSubLevelCollectionEntity",
    name_hash: 954114783,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_common::SUBLEVELCOLLECTIONENTITYBASE_TYPE_INFO),
        super_class_offset: offset_of!(ServerSubLevelCollectionEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSubLevelCollectionEntity as Default>::default())),
            create_boxed: || Box::new(<ServerSubLevelCollectionEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSUBLEVELCOLLECTIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSubLevelCollectionEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSUBLEVELCOLLECTIONENTITY_TYPE_INFO
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


pub static SERVERSUBLEVELCOLLECTIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSubLevelCollectionEntity-Array",
    name_hash: 3793827819,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerSubLevelCollectionEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerStartPointEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerStartPointEntityTrait: super::entity::EntityTrait {
}

impl ServerStartPointEntityTrait for ServerStartPointEntity {
}

impl super::entity::EntityTrait for ServerStartPointEntity {
}

impl super::entity::EntityBusPeerTrait for ServerStartPointEntity {
}

pub static SERVERSTARTPOINTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStartPointEntity",
    name_hash: 3213889879,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerStartPointEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerStartPointEntity as Default>::default())),
            create_boxed: || Box::new(<ServerStartPointEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSTARTPOINTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerStartPointEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSTARTPOINTENTITY_TYPE_INFO
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


pub static SERVERSTARTPOINTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStartPointEntity-Array",
    name_hash: 2857118307,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerStartPointEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct BangerHealthModuleData {
    pub _glacier_base: super::entity::GameComponentData,
    pub health: f32,
    pub material_pair: super::entity::MaterialDecl,
}

pub trait BangerHealthModuleDataTrait: super::entity::GameComponentDataTrait {
    fn health(&self) -> &f32;
    fn health_mut(&mut self) -> &mut f32;
    fn material_pair(&self) -> &super::entity::MaterialDecl;
    fn material_pair_mut(&mut self) -> &mut super::entity::MaterialDecl;
}

impl BangerHealthModuleDataTrait for BangerHealthModuleData {
    fn health(&self) -> &f32 {
        &self.health
    }
    fn health_mut(&mut self) -> &mut f32 {
        &mut self.health
    }
    fn material_pair(&self) -> &super::entity::MaterialDecl {
        &self.material_pair
    }
    fn material_pair_mut(&mut self) -> &mut super::entity::MaterialDecl {
        &mut self.material_pair
    }
}

impl super::entity::GameComponentDataTrait for BangerHealthModuleData {
}

impl super::entity::ComponentDataTrait for BangerHealthModuleData {
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

impl super::entity::GameObjectDataTrait for BangerHealthModuleData {
}

impl super::core::DataBusPeerTrait for BangerHealthModuleData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for BangerHealthModuleData {
}

impl super::core::DataContainerTrait for BangerHealthModuleData {
}

pub static BANGERHEALTHMODULEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BangerHealthModuleData",
    name_hash: 2903521390,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::GAMECOMPONENTDATA_TYPE_INFO),
        super_class_offset: offset_of!(BangerHealthModuleData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BangerHealthModuleData as Default>::default())),
            create_boxed: || Box::new(<BangerHealthModuleData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Health",
                name_hash: 3054337113,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BangerHealthModuleData, health),
            },
            FieldInfoData {
                name: "MaterialPair",
                name_hash: 161392100,
                flags: MemberInfoFlags::new(0),
                field_type: "MaterialDecl",
                rust_offset: offset_of!(BangerHealthModuleData, material_pair),
            },
        ],
    }),
    array_type: Some(BANGERHEALTHMODULEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for BangerHealthModuleData {
    fn type_info(&self) -> &'static TypeInfo {
        BANGERHEALTHMODULEDATA_TYPE_INFO
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


pub static BANGERHEALTHMODULEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BangerHealthModuleData-Array",
    name_hash: 2697900378,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("BangerHealthModuleData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerProjectileOnSpawnMessage {
}

pub trait ServerProjectileOnSpawnMessageTrait: TypeObject {
}

impl ServerProjectileOnSpawnMessageTrait for ServerProjectileOnSpawnMessage {
}

pub static SERVERPROJECTILEONSPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerProjectileOnSpawnMessage",
    name_hash: 3928241828,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerProjectileOnSpawnMessage as Default>::default())),
            create_boxed: || Box::new(<ServerProjectileOnSpawnMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerProjectileOnSpawnMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPROJECTILEONSPAWNMESSAGE_TYPE_INFO
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
pub struct AIDirectorStateMessage {
}

pub trait AIDirectorStateMessageTrait: TypeObject {
}

impl AIDirectorStateMessageTrait for AIDirectorStateMessage {
}

pub static AIDIRECTORSTATEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIDirectorStateMessage",
    name_hash: 3880441281,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AIDirectorStateMessage as Default>::default())),
            create_boxed: || Box::new(<AIDirectorStateMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for AIDirectorStateMessage {
    fn type_info(&self) -> &'static TypeInfo {
        AIDIRECTORSTATEMESSAGE_TYPE_INFO
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
pub struct AISpawnBotMessage {
}

pub trait AISpawnBotMessageTrait: TypeObject {
}

impl AISpawnBotMessageTrait for AISpawnBotMessage {
}

pub static AISPAWNBOTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AISpawnBotMessage",
    name_hash: 2493743044,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AISpawnBotMessage as Default>::default())),
            create_boxed: || Box::new(<AISpawnBotMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for AISpawnBotMessage {
    fn type_info(&self) -> &'static TypeInfo {
        AISPAWNBOTMESSAGE_TYPE_INFO
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
pub struct AIPlayerEnableAsTargetMessage {
}

pub trait AIPlayerEnableAsTargetMessageTrait: TypeObject {
}

impl AIPlayerEnableAsTargetMessageTrait for AIPlayerEnableAsTargetMessage {
}

pub static AIPLAYERENABLEASTARGETMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIPlayerEnableAsTargetMessage",
    name_hash: 1673589527,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AIPlayerEnableAsTargetMessage as Default>::default())),
            create_boxed: || Box::new(<AIPlayerEnableAsTargetMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for AIPlayerEnableAsTargetMessage {
    fn type_info(&self) -> &'static TypeInfo {
        AIPLAYERENABLEASTARGETMESSAGE_TYPE_INFO
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
pub struct ServerMissionObjectiveCompletedMessage {
}

pub trait ServerMissionObjectiveCompletedMessageTrait: TypeObject {
}

impl ServerMissionObjectiveCompletedMessageTrait for ServerMissionObjectiveCompletedMessage {
}

pub static SERVERMISSIONOBJECTIVECOMPLETEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMissionObjectiveCompletedMessage",
    name_hash: 1330562245,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerMissionObjectiveCompletedMessage as Default>::default())),
            create_boxed: || Box::new(<ServerMissionObjectiveCompletedMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerMissionObjectiveCompletedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERMISSIONOBJECTIVECOMPLETEDMESSAGE_TYPE_INFO
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
pub struct ServerRoundInterruptedMessage {
}

pub trait ServerRoundInterruptedMessageTrait: TypeObject {
}

impl ServerRoundInterruptedMessageTrait for ServerRoundInterruptedMessage {
}

pub static SERVERROUNDINTERRUPTEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerRoundInterruptedMessage",
    name_hash: 3388851375,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerRoundInterruptedMessage as Default>::default())),
            create_boxed: || Box::new(<ServerRoundInterruptedMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerRoundInterruptedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERROUNDINTERRUPTEDMESSAGE_TYPE_INFO
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
pub struct ServerRoundOverMessage {
}

pub trait ServerRoundOverMessageTrait: TypeObject {
}

impl ServerRoundOverMessageTrait for ServerRoundOverMessage {
}

pub static SERVERROUNDOVERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerRoundOverMessage",
    name_hash: 381336391,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerRoundOverMessage as Default>::default())),
            create_boxed: || Box::new(<ServerRoundOverMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerRoundOverMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERROUNDOVERMESSAGE_TYPE_INFO
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
pub struct ServerRoundResetMessage {
}

pub trait ServerRoundResetMessageTrait: TypeObject {
}

impl ServerRoundResetMessageTrait for ServerRoundResetMessage {
}

pub static SERVERROUNDRESETMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerRoundResetMessage",
    name_hash: 1270406748,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerRoundResetMessage as Default>::default())),
            create_boxed: || Box::new(<ServerRoundResetMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerRoundResetMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERROUNDRESETMESSAGE_TYPE_INFO
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
pub struct ServerGameplayCheckpointActivatedMessage {
}

pub trait ServerGameplayCheckpointActivatedMessageTrait: TypeObject {
}

impl ServerGameplayCheckpointActivatedMessageTrait for ServerGameplayCheckpointActivatedMessage {
}

pub static SERVERGAMEPLAYCHECKPOINTACTIVATEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameplayCheckpointActivatedMessage",
    name_hash: 4192104662,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGameplayCheckpointActivatedMessage as Default>::default())),
            create_boxed: || Box::new(<ServerGameplayCheckpointActivatedMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerGameplayCheckpointActivatedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGAMEPLAYCHECKPOINTACTIVATEDMESSAGE_TYPE_INFO
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
pub struct ServerGameplayCheckpointTriggeredMessage {
}

pub trait ServerGameplayCheckpointTriggeredMessageTrait: TypeObject {
}

impl ServerGameplayCheckpointTriggeredMessageTrait for ServerGameplayCheckpointTriggeredMessage {
}

pub static SERVERGAMEPLAYCHECKPOINTTRIGGEREDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameplayCheckpointTriggeredMessage",
    name_hash: 1028232370,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGameplayCheckpointTriggeredMessage as Default>::default())),
            create_boxed: || Box::new(<ServerGameplayCheckpointTriggeredMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerGameplayCheckpointTriggeredMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGAMEPLAYCHECKPOINTTRIGGEREDMESSAGE_TYPE_INFO
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
pub struct ServerGameModeResetMessage {
}

pub trait ServerGameModeResetMessageTrait: TypeObject {
}

impl ServerGameModeResetMessageTrait for ServerGameModeResetMessage {
}

pub static SERVERGAMEMODERESETMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameModeResetMessage",
    name_hash: 766613715,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGameModeResetMessage as Default>::default())),
            create_boxed: || Box::new(<ServerGameModeResetMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerGameModeResetMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGAMEMODERESETMESSAGE_TYPE_INFO
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
pub struct ServerGameplaySetPostRoundLogicMessage {
}

pub trait ServerGameplaySetPostRoundLogicMessageTrait: TypeObject {
}

impl ServerGameplaySetPostRoundLogicMessageTrait for ServerGameplaySetPostRoundLogicMessage {
}

pub static SERVERGAMEPLAYSETPOSTROUNDLOGICMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameplaySetPostRoundLogicMessage",
    name_hash: 4127160663,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGameplaySetPostRoundLogicMessage as Default>::default())),
            create_boxed: || Box::new(<ServerGameplaySetPostRoundLogicMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerGameplaySetPostRoundLogicMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGAMEPLAYSETPOSTROUNDLOGICMESSAGE_TYPE_INFO
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
pub struct ServerGameplaySetPreRoundLogicMessage {
}

pub trait ServerGameplaySetPreRoundLogicMessageTrait: TypeObject {
}

impl ServerGameplaySetPreRoundLogicMessageTrait for ServerGameplaySetPreRoundLogicMessage {
}

pub static SERVERGAMEPLAYSETPREROUNDLOGICMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameplaySetPreRoundLogicMessage",
    name_hash: 2447290856,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGameplaySetPreRoundLogicMessage as Default>::default())),
            create_boxed: || Box::new(<ServerGameplaySetPreRoundLogicMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerGameplaySetPreRoundLogicMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGAMEPLAYSETPREROUNDLOGICMESSAGE_TYPE_INFO
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
pub struct ServerGameplayGameModeResetMessage {
}

pub trait ServerGameplayGameModeResetMessageTrait: TypeObject {
}

impl ServerGameplayGameModeResetMessageTrait for ServerGameplayGameModeResetMessage {
}

pub static SERVERGAMEPLAYGAMEMODERESETMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameplayGameModeResetMessage",
    name_hash: 2723631769,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGameplayGameModeResetMessage as Default>::default())),
            create_boxed: || Box::new(<ServerGameplayGameModeResetMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerGameplayGameModeResetMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGAMEPLAYGAMEMODERESETMESSAGE_TYPE_INFO
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
pub struct ServerGameplayServerPlayerMenuCancelMessage {
}

pub trait ServerGameplayServerPlayerMenuCancelMessageTrait: TypeObject {
}

impl ServerGameplayServerPlayerMenuCancelMessageTrait for ServerGameplayServerPlayerMenuCancelMessage {
}

pub static SERVERGAMEPLAYSERVERPLAYERMENUCANCELMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameplayServerPlayerMenuCancelMessage",
    name_hash: 2406017730,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGameplayServerPlayerMenuCancelMessage as Default>::default())),
            create_boxed: || Box::new(<ServerGameplayServerPlayerMenuCancelMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerGameplayServerPlayerMenuCancelMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGAMEPLAYSERVERPLAYERMENUCANCELMESSAGE_TYPE_INFO
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
pub struct ServerGameplayServerPlayerMenuOkMessage {
}

pub trait ServerGameplayServerPlayerMenuOkMessageTrait: TypeObject {
}

impl ServerGameplayServerPlayerMenuOkMessageTrait for ServerGameplayServerPlayerMenuOkMessage {
}

pub static SERVERGAMEPLAYSERVERPLAYERMENUOKMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameplayServerPlayerMenuOkMessage",
    name_hash: 3693418848,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGameplayServerPlayerMenuOkMessage as Default>::default())),
            create_boxed: || Box::new(<ServerGameplayServerPlayerMenuOkMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerGameplayServerPlayerMenuOkMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGAMEPLAYSERVERPLAYERMENUOKMESSAGE_TYPE_INFO
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
pub struct ServerGameplayPreviousWeatherStateMessage {
}

pub trait ServerGameplayPreviousWeatherStateMessageTrait: TypeObject {
}

impl ServerGameplayPreviousWeatherStateMessageTrait for ServerGameplayPreviousWeatherStateMessage {
}

pub static SERVERGAMEPLAYPREVIOUSWEATHERSTATEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameplayPreviousWeatherStateMessage",
    name_hash: 1698878623,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGameplayPreviousWeatherStateMessage as Default>::default())),
            create_boxed: || Box::new(<ServerGameplayPreviousWeatherStateMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerGameplayPreviousWeatherStateMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGAMEPLAYPREVIOUSWEATHERSTATEMESSAGE_TYPE_INFO
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
pub struct ServerGameplayFightHarderMessage {
}

pub trait ServerGameplayFightHarderMessageTrait: TypeObject {
}

impl ServerGameplayFightHarderMessageTrait for ServerGameplayFightHarderMessage {
}

pub static SERVERGAMEPLAYFIGHTHARDERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameplayFightHarderMessage",
    name_hash: 1560940477,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGameplayFightHarderMessage as Default>::default())),
            create_boxed: || Box::new(<ServerGameplayFightHarderMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerGameplayFightHarderMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGAMEPLAYFIGHTHARDERMESSAGE_TYPE_INFO
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
pub struct ServerGameplayDeserterReturnMessage {
}

pub trait ServerGameplayDeserterReturnMessageTrait: TypeObject {
}

impl ServerGameplayDeserterReturnMessageTrait for ServerGameplayDeserterReturnMessage {
}

pub static SERVERGAMEPLAYDESERTERRETURNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameplayDeserterReturnMessage",
    name_hash: 2373434253,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGameplayDeserterReturnMessage as Default>::default())),
            create_boxed: || Box::new(<ServerGameplayDeserterReturnMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerGameplayDeserterReturnMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGAMEPLAYDESERTERRETURNMESSAGE_TYPE_INFO
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
pub struct ServerGameplayDeserterMessage {
}

pub trait ServerGameplayDeserterMessageTrait: TypeObject {
}

impl ServerGameplayDeserterMessageTrait for ServerGameplayDeserterMessage {
}

pub static SERVERGAMEPLAYDESERTERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameplayDeserterMessage",
    name_hash: 947658279,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGameplayDeserterMessage as Default>::default())),
            create_boxed: || Box::new(<ServerGameplayDeserterMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerGameplayDeserterMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGAMEPLAYDESERTERMESSAGE_TYPE_INFO
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
pub struct ServerGameplayPlayerMenuCancelMessage {
}

pub trait ServerGameplayPlayerMenuCancelMessageTrait: TypeObject {
}

impl ServerGameplayPlayerMenuCancelMessageTrait for ServerGameplayPlayerMenuCancelMessage {
}

pub static SERVERGAMEPLAYPLAYERMENUCANCELMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameplayPlayerMenuCancelMessage",
    name_hash: 476842887,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGameplayPlayerMenuCancelMessage as Default>::default())),
            create_boxed: || Box::new(<ServerGameplayPlayerMenuCancelMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerGameplayPlayerMenuCancelMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGAMEPLAYPLAYERMENUCANCELMESSAGE_TYPE_INFO
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
pub struct ServerGameplayPlayerMenuOkMessage {
}

pub trait ServerGameplayPlayerMenuOkMessageTrait: TypeObject {
}

impl ServerGameplayPlayerMenuOkMessageTrait for ServerGameplayPlayerMenuOkMessage {
}

pub static SERVERGAMEPLAYPLAYERMENUOKMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameplayPlayerMenuOkMessage",
    name_hash: 4234465381,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGameplayPlayerMenuOkMessage as Default>::default())),
            create_boxed: || Box::new(<ServerGameplayPlayerMenuOkMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerGameplayPlayerMenuOkMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGAMEPLAYPLAYERMENUOKMESSAGE_TYPE_INFO
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
pub struct ServerGameplayVoiceOverFinishedMessage {
}

pub trait ServerGameplayVoiceOverFinishedMessageTrait: TypeObject {
}

impl ServerGameplayVoiceOverFinishedMessageTrait for ServerGameplayVoiceOverFinishedMessage {
}

pub static SERVERGAMEPLAYVOICEOVERFINISHEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameplayVoiceOverFinishedMessage",
    name_hash: 2600279499,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGameplayVoiceOverFinishedMessage as Default>::default())),
            create_boxed: || Box::new(<ServerGameplayVoiceOverFinishedMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerGameplayVoiceOverFinishedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGAMEPLAYVOICEOVERFINISHEDMESSAGE_TYPE_INFO
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
pub struct ServerStaticModelDamagedPartByPlayerMessage {
}

pub trait ServerStaticModelDamagedPartByPlayerMessageTrait: TypeObject {
}

impl ServerStaticModelDamagedPartByPlayerMessageTrait for ServerStaticModelDamagedPartByPlayerMessage {
}

pub static SERVERSTATICMODELDAMAGEDPARTBYPLAYERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticModelDamagedPartByPlayerMessage",
    name_hash: 822978124,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerStaticModelDamagedPartByPlayerMessage as Default>::default())),
            create_boxed: || Box::new(<ServerStaticModelDamagedPartByPlayerMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerStaticModelDamagedPartByPlayerMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSTATICMODELDAMAGEDPARTBYPLAYERMESSAGE_TYPE_INFO
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
pub struct ServerStaticModelDestroyedPartMessage {
}

pub trait ServerStaticModelDestroyedPartMessageTrait: TypeObject {
}

impl ServerStaticModelDestroyedPartMessageTrait for ServerStaticModelDestroyedPartMessage {
}

pub static SERVERSTATICMODELDESTROYEDPARTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticModelDestroyedPartMessage",
    name_hash: 1668365640,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerStaticModelDestroyedPartMessage as Default>::default())),
            create_boxed: || Box::new(<ServerStaticModelDestroyedPartMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ServerStaticModelDestroyedPartMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSTATICMODELDESTROYEDPARTMESSAGE_TYPE_INFO
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
pub struct ServerStaticModelGroupDestroyedPartMessage {
}

pub trait ServerStaticModelGroupDestroyedPartMessageTrait: TypeObject {
}

impl ServerStaticModelGroupDestroyedPartMessageTrait for ServerStaticModelGroupDestroyedPartMessage {
}

pub static SERVERSTATICMODELGROUPDESTROYEDPARTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticModelGroupDestroyedPartMessage",
    name_hash: 188772375,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerStaticModelGroupDestroyedPartMessage as Default>::default())),
            create_boxed: || Box::new(<ServerStaticModelGroupDestroyedPartMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ServerStaticModelGroupDestroyedPartMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSTATICMODELGROUPDESTROYEDPARTMESSAGE_TYPE_INFO
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
pub struct ServerStaticModelDestroyedAllCollapsablePartsMessage {
}

pub trait ServerStaticModelDestroyedAllCollapsablePartsMessageTrait: TypeObject {
}

impl ServerStaticModelDestroyedAllCollapsablePartsMessageTrait for ServerStaticModelDestroyedAllCollapsablePartsMessage {
}

pub static SERVERSTATICMODELDESTROYEDALLCOLLAPSABLEPARTSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticModelDestroyedAllCollapsablePartsMessage",
    name_hash: 559150686,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerStaticModelDestroyedAllCollapsablePartsMessage as Default>::default())),
            create_boxed: || Box::new(<ServerStaticModelDestroyedAllCollapsablePartsMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ServerStaticModelDestroyedAllCollapsablePartsMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSTATICMODELDESTROYEDALLCOLLAPSABLEPARTSMESSAGE_TYPE_INFO
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
pub struct ServerStaticModelSpawnMessage {
}

pub trait ServerStaticModelSpawnMessageTrait: TypeObject {
}

impl ServerStaticModelSpawnMessageTrait for ServerStaticModelSpawnMessage {
}

pub static SERVERSTATICMODELSPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticModelSpawnMessage",
    name_hash: 3070748359,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerStaticModelSpawnMessage as Default>::default())),
            create_boxed: || Box::new(<ServerStaticModelSpawnMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ServerStaticModelSpawnMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSTATICMODELSPAWNMESSAGE_TYPE_INFO
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
pub struct ServerCollisionExplosionPackDestroyedMessage {
}

pub trait ServerCollisionExplosionPackDestroyedMessageTrait: TypeObject {
}

impl ServerCollisionExplosionPackDestroyedMessageTrait for ServerCollisionExplosionPackDestroyedMessage {
}

pub static SERVERCOLLISIONEXPLOSIONPACKDESTROYEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCollisionExplosionPackDestroyedMessage",
    name_hash: 4178419770,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCollisionExplosionPackDestroyedMessage as Default>::default())),
            create_boxed: || Box::new(<ServerCollisionExplosionPackDestroyedMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerCollisionExplosionPackDestroyedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCOLLISIONEXPLOSIONPACKDESTROYEDMESSAGE_TYPE_INFO
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
pub struct ServerCollisionExplosionPackPlacedMessage {
}

pub trait ServerCollisionExplosionPackPlacedMessageTrait: TypeObject {
}

impl ServerCollisionExplosionPackPlacedMessageTrait for ServerCollisionExplosionPackPlacedMessage {
}

pub static SERVERCOLLISIONEXPLOSIONPACKPLACEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCollisionExplosionPackPlacedMessage",
    name_hash: 3605193638,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCollisionExplosionPackPlacedMessage as Default>::default())),
            create_boxed: || Box::new(<ServerCollisionExplosionPackPlacedMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerCollisionExplosionPackPlacedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCOLLISIONEXPLOSIONPACKPLACEDMESSAGE_TYPE_INFO
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
pub struct ServerCollisionExplosionUnSpawnMessage {
}

pub trait ServerCollisionExplosionUnSpawnMessageTrait: TypeObject {
}

impl ServerCollisionExplosionUnSpawnMessageTrait for ServerCollisionExplosionUnSpawnMessage {
}

pub static SERVERCOLLISIONEXPLOSIONUNSPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCollisionExplosionUnSpawnMessage",
    name_hash: 2935348064,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCollisionExplosionUnSpawnMessage as Default>::default())),
            create_boxed: || Box::new(<ServerCollisionExplosionUnSpawnMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerCollisionExplosionUnSpawnMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCOLLISIONEXPLOSIONUNSPAWNMESSAGE_TYPE_INFO
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
pub struct ServerCollisionExplosionDamageMessage {
}

pub trait ServerCollisionExplosionDamageMessageTrait: TypeObject {
}

impl ServerCollisionExplosionDamageMessageTrait for ServerCollisionExplosionDamageMessage {
}

pub static SERVERCOLLISIONEXPLOSIONDAMAGEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCollisionExplosionDamageMessage",
    name_hash: 3725500491,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCollisionExplosionDamageMessage as Default>::default())),
            create_boxed: || Box::new(<ServerCollisionExplosionDamageMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ServerCollisionExplosionDamageMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCOLLISIONEXPLOSIONDAMAGEMESSAGE_TYPE_INFO
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
pub struct ServerCollisionExplosionSpawnMessage {
}

pub trait ServerCollisionExplosionSpawnMessageTrait: TypeObject {
}

impl ServerCollisionExplosionSpawnMessageTrait for ServerCollisionExplosionSpawnMessage {
}

pub static SERVERCOLLISIONEXPLOSIONSPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCollisionExplosionSpawnMessage",
    name_hash: 58123131,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCollisionExplosionSpawnMessage as Default>::default())),
            create_boxed: || Box::new(<ServerCollisionExplosionSpawnMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ServerCollisionExplosionSpawnMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCOLLISIONEXPLOSIONSPAWNMESSAGE_TYPE_INFO
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
pub struct ServerCollisionProjectileImpactMessage {
}

pub trait ServerCollisionProjectileImpactMessageTrait: TypeObject {
}

impl ServerCollisionProjectileImpactMessageTrait for ServerCollisionProjectileImpactMessage {
}

pub static SERVERCOLLISIONPROJECTILEIMPACTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCollisionProjectileImpactMessage",
    name_hash: 1157204162,
    flags: MemberInfoFlags::new(32841),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCollisionProjectileImpactMessage as Default>::default())),
            create_boxed: || Box::new(<ServerCollisionProjectileImpactMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ServerCollisionProjectileImpactMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCOLLISIONPROJECTILEIMPACTMESSAGE_TYPE_INFO
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
pub struct ServerCollisionProjectileFireMessage {
}

pub trait ServerCollisionProjectileFireMessageTrait: TypeObject {
}

impl ServerCollisionProjectileFireMessageTrait for ServerCollisionProjectileFireMessage {
}

pub static SERVERCOLLISIONPROJECTILEFIREMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCollisionProjectileFireMessage",
    name_hash: 2936571736,
    flags: MemberInfoFlags::new(32841),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCollisionProjectileFireMessage as Default>::default())),
            create_boxed: || Box::new(<ServerCollisionProjectileFireMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ServerCollisionProjectileFireMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCOLLISIONPROJECTILEFIREMESSAGE_TYPE_INFO
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
pub struct ServerCollisionGrenadeCollisionMessage {
}

pub trait ServerCollisionGrenadeCollisionMessageTrait: TypeObject {
}

impl ServerCollisionGrenadeCollisionMessageTrait for ServerCollisionGrenadeCollisionMessage {
}

pub static SERVERCOLLISIONGRENADECOLLISIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCollisionGrenadeCollisionMessage",
    name_hash: 3808506965,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCollisionGrenadeCollisionMessage as Default>::default())),
            create_boxed: || Box::new(<ServerCollisionGrenadeCollisionMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ServerCollisionGrenadeCollisionMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCOLLISIONGRENADECOLLISIONMESSAGE_TYPE_INFO
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
pub struct ServerCollisionGrenadeThrowMessage {
}

pub trait ServerCollisionGrenadeThrowMessageTrait: TypeObject {
}

impl ServerCollisionGrenadeThrowMessageTrait for ServerCollisionGrenadeThrowMessage {
}

pub static SERVERCOLLISIONGRENADETHROWMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCollisionGrenadeThrowMessage",
    name_hash: 458752477,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCollisionGrenadeThrowMessage as Default>::default())),
            create_boxed: || Box::new(<ServerCollisionGrenadeThrowMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ServerCollisionGrenadeThrowMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCOLLISIONGRENADETHROWMESSAGE_TYPE_INFO
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
pub struct ServerWeaponComponentWeaponOnUnspawnMessage {
}

pub trait ServerWeaponComponentWeaponOnUnspawnMessageTrait: TypeObject {
}

impl ServerWeaponComponentWeaponOnUnspawnMessageTrait for ServerWeaponComponentWeaponOnUnspawnMessage {
}

pub static SERVERWEAPONCOMPONENTWEAPONONUNSPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponComponentWeaponOnUnspawnMessage",
    name_hash: 3389882629,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerWeaponComponentWeaponOnUnspawnMessage as Default>::default())),
            create_boxed: || Box::new(<ServerWeaponComponentWeaponOnUnspawnMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerWeaponComponentWeaponOnUnspawnMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERWEAPONCOMPONENTWEAPONONUNSPAWNMESSAGE_TYPE_INFO
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
pub struct ServerWeaponComponentWeaponOnSpawnMessage {
}

pub trait ServerWeaponComponentWeaponOnSpawnMessageTrait: TypeObject {
}

impl ServerWeaponComponentWeaponOnSpawnMessageTrait for ServerWeaponComponentWeaponOnSpawnMessage {
}

pub static SERVERWEAPONCOMPONENTWEAPONONSPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponComponentWeaponOnSpawnMessage",
    name_hash: 1742299582,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerWeaponComponentWeaponOnSpawnMessage as Default>::default())),
            create_boxed: || Box::new(<ServerWeaponComponentWeaponOnSpawnMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerWeaponComponentWeaponOnSpawnMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERWEAPONCOMPONENTWEAPONONSPAWNMESSAGE_TYPE_INFO
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
pub struct ServerEntityPickupOnUnspawnMessage {
}

pub trait ServerEntityPickupOnUnspawnMessageTrait: TypeObject {
}

impl ServerEntityPickupOnUnspawnMessageTrait for ServerEntityPickupOnUnspawnMessage {
}

pub static SERVERENTITYPICKUPONUNSPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEntityPickupOnUnspawnMessage",
    name_hash: 1008656709,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerEntityPickupOnUnspawnMessage as Default>::default())),
            create_boxed: || Box::new(<ServerEntityPickupOnUnspawnMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerEntityPickupOnUnspawnMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERENTITYPICKUPONUNSPAWNMESSAGE_TYPE_INFO
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
pub struct ServerEntityPickupOnSpawnMessage {
}

pub trait ServerEntityPickupOnSpawnMessageTrait: TypeObject {
}

impl ServerEntityPickupOnSpawnMessageTrait for ServerEntityPickupOnSpawnMessage {
}

pub static SERVERENTITYPICKUPONSPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEntityPickupOnSpawnMessage",
    name_hash: 3707912702,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerEntityPickupOnSpawnMessage as Default>::default())),
            create_boxed: || Box::new(<ServerEntityPickupOnSpawnMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerEntityPickupOnSpawnMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERENTITYPICKUPONSPAWNMESSAGE_TYPE_INFO
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
pub struct ServerEntityBangerEntityOnUnspawnMessage {
}

pub trait ServerEntityBangerEntityOnUnspawnMessageTrait: TypeObject {
}

impl ServerEntityBangerEntityOnUnspawnMessageTrait for ServerEntityBangerEntityOnUnspawnMessage {
}

pub static SERVERENTITYBANGERENTITYONUNSPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEntityBangerEntityOnUnspawnMessage",
    name_hash: 1318770295,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerEntityBangerEntityOnUnspawnMessage as Default>::default())),
            create_boxed: || Box::new(<ServerEntityBangerEntityOnUnspawnMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerEntityBangerEntityOnUnspawnMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERENTITYBANGERENTITYONUNSPAWNMESSAGE_TYPE_INFO
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
pub struct ServerEntityBangerEntityOnSpawnMessage {
}

pub trait ServerEntityBangerEntityOnSpawnMessageTrait: TypeObject {
}

impl ServerEntityBangerEntityOnSpawnMessageTrait for ServerEntityBangerEntityOnSpawnMessage {
}

pub static SERVERENTITYBANGERENTITYONSPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEntityBangerEntityOnSpawnMessage",
    name_hash: 4199579788,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerEntityBangerEntityOnSpawnMessage as Default>::default())),
            create_boxed: || Box::new(<ServerEntityBangerEntityOnSpawnMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerEntityBangerEntityOnSpawnMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERENTITYBANGERENTITYONSPAWNMESSAGE_TYPE_INFO
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
pub struct ServerClubMemberDeletedMessage {
}

pub trait ServerClubMemberDeletedMessageTrait: TypeObject {
}

impl ServerClubMemberDeletedMessageTrait for ServerClubMemberDeletedMessage {
}

pub static SERVERCLUBMEMBERDELETEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerClubMemberDeletedMessage",
    name_hash: 389187582,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerClubMemberDeletedMessage as Default>::default())),
            create_boxed: || Box::new(<ServerClubMemberDeletedMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerClubMemberDeletedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCLUBMEMBERDELETEDMESSAGE_TYPE_INFO
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
pub struct ServerClubMemberCreatedMessage {
}

pub trait ServerClubMemberCreatedMessageTrait: TypeObject {
}

impl ServerClubMemberCreatedMessageTrait for ServerClubMemberCreatedMessage {
}

pub static SERVERCLUBMEMBERCREATEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerClubMemberCreatedMessage",
    name_hash: 3330220003,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerClubMemberCreatedMessage as Default>::default())),
            create_boxed: || Box::new(<ServerClubMemberCreatedMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerClubMemberCreatedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCLUBMEMBERCREATEDMESSAGE_TYPE_INFO
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
pub struct ServerVehicleLockableMessage {
}

pub trait ServerVehicleLockableMessageTrait: TypeObject {
}

impl ServerVehicleLockableMessageTrait for ServerVehicleLockableMessage {
}

pub static SERVERVEHICLELOCKABLEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleLockableMessage",
    name_hash: 3950011090,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerVehicleLockableMessage as Default>::default())),
            create_boxed: || Box::new(<ServerVehicleLockableMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerVehicleLockableMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERVEHICLELOCKABLEMESSAGE_TYPE_INFO
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
pub struct ServerVehicleExitMessage {
}

pub trait ServerVehicleExitMessageTrait: TypeObject {
}

impl ServerVehicleExitMessageTrait for ServerVehicleExitMessage {
}

pub static SERVERVEHICLEEXITMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleExitMessage",
    name_hash: 118781299,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerVehicleExitMessage as Default>::default())),
            create_boxed: || Box::new(<ServerVehicleExitMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerVehicleExitMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERVEHICLEEXITMESSAGE_TYPE_INFO
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
pub struct ServerVehicleEnterMessage {
}

pub trait ServerVehicleEnterMessageTrait: TypeObject {
}

impl ServerVehicleEnterMessageTrait for ServerVehicleEnterMessage {
}

pub static SERVERVEHICLEENTERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleEnterMessage",
    name_hash: 3196821307,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerVehicleEnterMessage as Default>::default())),
            create_boxed: || Box::new(<ServerVehicleEnterMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerVehicleEnterMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERVEHICLEENTERMESSAGE_TYPE_INFO
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
pub struct ServerVehicleDisabledMessage {
}

pub trait ServerVehicleDisabledMessageTrait: TypeObject {
}

impl ServerVehicleDisabledMessageTrait for ServerVehicleDisabledMessage {
}

pub static SERVERVEHICLEDISABLEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleDisabledMessage",
    name_hash: 735256739,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerVehicleDisabledMessage as Default>::default())),
            create_boxed: || Box::new(<ServerVehicleDisabledMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerVehicleDisabledMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERVEHICLEDISABLEDMESSAGE_TYPE_INFO
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
pub struct ServerVehicleDamageMessage {
}

pub trait ServerVehicleDamageMessageTrait: TypeObject {
}

impl ServerVehicleDamageMessageTrait for ServerVehicleDamageMessage {
}

pub static SERVERVEHICLEDAMAGEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleDamageMessage",
    name_hash: 2473406616,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerVehicleDamageMessage as Default>::default())),
            create_boxed: || Box::new(<ServerVehicleDamageMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerVehicleDamageMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERVEHICLEDAMAGEMESSAGE_TYPE_INFO
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
pub struct ServerVehicleEnterRestrictionMessage {
}

pub trait ServerVehicleEnterRestrictionMessageTrait: TypeObject {
}

impl ServerVehicleEnterRestrictionMessageTrait for ServerVehicleEnterRestrictionMessage {
}

pub static SERVERVEHICLEENTERRESTRICTIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleEnterRestrictionMessage",
    name_hash: 4131851279,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerVehicleEnterRestrictionMessage as Default>::default())),
            create_boxed: || Box::new(<ServerVehicleEnterRestrictionMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerVehicleEnterRestrictionMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERVEHICLEENTERRESTRICTIONMESSAGE_TYPE_INFO
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
pub struct ServerVehicleUnspawnMessage {
}

pub trait ServerVehicleUnspawnMessageTrait: TypeObject {
}

impl ServerVehicleUnspawnMessageTrait for ServerVehicleUnspawnMessage {
}

pub static SERVERVEHICLEUNSPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleUnspawnMessage",
    name_hash: 2058033011,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerVehicleUnspawnMessage as Default>::default())),
            create_boxed: || Box::new(<ServerVehicleUnspawnMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerVehicleUnspawnMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERVEHICLEUNSPAWNMESSAGE_TYPE_INFO
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
pub struct ServerVehicleSpawnDoneMessage {
}

pub trait ServerVehicleSpawnDoneMessageTrait: TypeObject {
}

impl ServerVehicleSpawnDoneMessageTrait for ServerVehicleSpawnDoneMessage {
}

pub static SERVERVEHICLESPAWNDONEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleSpawnDoneMessage",
    name_hash: 2157901736,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerVehicleSpawnDoneMessage as Default>::default())),
            create_boxed: || Box::new(<ServerVehicleSpawnDoneMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerVehicleSpawnDoneMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERVEHICLESPAWNDONEMESSAGE_TYPE_INFO
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
pub struct ServerVehicleForceArmamentReturnMessage {
}

pub trait ServerVehicleForceArmamentReturnMessageTrait: TypeObject {
}

impl ServerVehicleForceArmamentReturnMessageTrait for ServerVehicleForceArmamentReturnMessage {
}

pub static SERVERVEHICLEFORCEARMAMENTRETURNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleForceArmamentReturnMessage",
    name_hash: 3583485865,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerVehicleForceArmamentReturnMessage as Default>::default())),
            create_boxed: || Box::new(<ServerVehicleForceArmamentReturnMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerVehicleForceArmamentReturnMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERVEHICLEFORCEARMAMENTRETURNMESSAGE_TYPE_INFO
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
pub struct ServerVehicleSetRemoteControlledDamageGiverMessage {
}

pub trait ServerVehicleSetRemoteControlledDamageGiverMessageTrait: TypeObject {
}

impl ServerVehicleSetRemoteControlledDamageGiverMessageTrait for ServerVehicleSetRemoteControlledDamageGiverMessage {
}

pub static SERVERVEHICLESETREMOTECONTROLLEDDAMAGEGIVERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleSetRemoteControlledDamageGiverMessage",
    name_hash: 479116507,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerVehicleSetRemoteControlledDamageGiverMessage as Default>::default())),
            create_boxed: || Box::new(<ServerVehicleSetRemoteControlledDamageGiverMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerVehicleSetRemoteControlledDamageGiverMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERVEHICLESETREMOTECONTROLLEDDAMAGEGIVERMESSAGE_TYPE_INFO
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
pub struct ServerVehicleSwitchTeamMessage {
}

pub trait ServerVehicleSwitchTeamMessageTrait: TypeObject {
}

impl ServerVehicleSwitchTeamMessageTrait for ServerVehicleSwitchTeamMessage {
}

pub static SERVERVEHICLESWITCHTEAMMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleSwitchTeamMessage",
    name_hash: 357883516,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerVehicleSwitchTeamMessage as Default>::default())),
            create_boxed: || Box::new(<ServerVehicleSwitchTeamMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerVehicleSwitchTeamMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERVEHICLESWITCHTEAMMESSAGE_TYPE_INFO
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
pub struct ServerVehicleDestroyedMessage {
}

pub trait ServerVehicleDestroyedMessageTrait: TypeObject {
}

impl ServerVehicleDestroyedMessageTrait for ServerVehicleDestroyedMessage {
}

pub static SERVERVEHICLEDESTROYEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVehicleDestroyedMessage",
    name_hash: 37016272,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerVehicleDestroyedMessage as Default>::default())),
            create_boxed: || Box::new(<ServerVehicleDestroyedMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerVehicleDestroyedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERVEHICLEDESTROYEDMESSAGE_TYPE_INFO
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
pub struct ServerPlayerDisconnectMessage {
}

pub trait ServerPlayerDisconnectMessageTrait: TypeObject {
}

impl ServerPlayerDisconnectMessageTrait for ServerPlayerDisconnectMessage {
}

pub static SERVERPLAYERDISCONNECTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerDisconnectMessage",
    name_hash: 3043927960,
    flags: MemberInfoFlags::new(73),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPlayerDisconnectMessage as Default>::default())),
            create_boxed: || Box::new(<ServerPlayerDisconnectMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerDisconnectMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPLAYERDISCONNECTMESSAGE_TYPE_INFO
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
pub struct ServerPlayerStartedFireMessage {
}

pub trait ServerPlayerStartedFireMessageTrait: TypeObject {
}

impl ServerPlayerStartedFireMessageTrait for ServerPlayerStartedFireMessage {
}

pub static SERVERPLAYERSTARTEDFIREMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerStartedFireMessage",
    name_hash: 1906140353,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPlayerStartedFireMessage as Default>::default())),
            create_boxed: || Box::new(<ServerPlayerStartedFireMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerStartedFireMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPLAYERSTARTEDFIREMESSAGE_TYPE_INFO
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
pub struct ServerPlayerOnPickupMessage {
}

pub trait ServerPlayerOnPickupMessageTrait: TypeObject {
}

impl ServerPlayerOnPickupMessageTrait for ServerPlayerOnPickupMessage {
}

pub static SERVERPLAYERONPICKUPMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerOnPickupMessage",
    name_hash: 2395502221,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPlayerOnPickupMessage as Default>::default())),
            create_boxed: || Box::new(<ServerPlayerOnPickupMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerOnPickupMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPLAYERONPICKUPMESSAGE_TYPE_INFO
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
pub enum PickupItemType {
    #[default]
    PITWeapon = 0,
    PITAmmo = 1,
}

pub static PICKUPITEMTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PickupItemType",
    name_hash: 2937439548,
    flags: MemberInfoFlags::new(49429),
    module: "GameServer",
    data: TypeInfoData::Enum,
    array_type: Some(PICKUPITEMTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PickupItemType {
    fn type_info(&self) -> &'static TypeInfo {
        PICKUPITEMTYPE_TYPE_INFO
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


pub static PICKUPITEMTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PickupItemType-Array",
    name_hash: 639982728,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("PickupItemType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerPlayerChatMessage {
}

pub trait ServerPlayerChatMessageTrait: TypeObject {
}

impl ServerPlayerChatMessageTrait for ServerPlayerChatMessage {
}

pub static SERVERPLAYERCHATMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerChatMessage",
    name_hash: 3687249766,
    flags: MemberInfoFlags::new(73),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPlayerChatMessage as Default>::default())),
            create_boxed: || Box::new(<ServerPlayerChatMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerChatMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPLAYERCHATMESSAGE_TYPE_INFO
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
pub struct ServerPlayerExitEntryMessage {
}

pub trait ServerPlayerExitEntryMessageTrait: TypeObject {
}

impl ServerPlayerExitEntryMessageTrait for ServerPlayerExitEntryMessage {
}

pub static SERVERPLAYEREXITENTRYMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerExitEntryMessage",
    name_hash: 370370828,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPlayerExitEntryMessage as Default>::default())),
            create_boxed: || Box::new(<ServerPlayerExitEntryMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerExitEntryMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPLAYEREXITENTRYMESSAGE_TYPE_INFO
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
pub struct ServerPlayerEnterEntryMessage {
}

pub trait ServerPlayerEnterEntryMessageTrait: TypeObject {
}

impl ServerPlayerEnterEntryMessageTrait for ServerPlayerEnterEntryMessage {
}

pub static SERVERPLAYERENTERENTRYMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerEnterEntryMessage",
    name_hash: 1178721508,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPlayerEnterEntryMessage as Default>::default())),
            create_boxed: || Box::new(<ServerPlayerEnterEntryMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerEnterEntryMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPLAYERENTERENTRYMESSAGE_TYPE_INFO
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
pub struct ServerPlayerAboutToClearCharacterMessage {
}

pub trait ServerPlayerAboutToClearCharacterMessageTrait: TypeObject {
}

impl ServerPlayerAboutToClearCharacterMessageTrait for ServerPlayerAboutToClearCharacterMessage {
}

pub static SERVERPLAYERABOUTTOCLEARCHARACTERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerAboutToClearCharacterMessage",
    name_hash: 2782240814,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPlayerAboutToClearCharacterMessage as Default>::default())),
            create_boxed: || Box::new(<ServerPlayerAboutToClearCharacterMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerAboutToClearCharacterMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPLAYERABOUTTOCLEARCHARACTERMESSAGE_TYPE_INFO
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
pub struct ServerPlayerInstantSuicideMessage {
}

pub trait ServerPlayerInstantSuicideMessageTrait: TypeObject {
}

impl ServerPlayerInstantSuicideMessageTrait for ServerPlayerInstantSuicideMessage {
}

pub static SERVERPLAYERINSTANTSUICIDEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerInstantSuicideMessage",
    name_hash: 2213431495,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPlayerInstantSuicideMessage as Default>::default())),
            create_boxed: || Box::new(<ServerPlayerInstantSuicideMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerInstantSuicideMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPLAYERINSTANTSUICIDEMESSAGE_TYPE_INFO
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
pub struct ServerPlayerKilledMessage {
}

pub trait ServerPlayerKilledMessageTrait: TypeObject {
}

impl ServerPlayerKilledMessageTrait for ServerPlayerKilledMessage {
}

pub static SERVERPLAYERKILLEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerKilledMessage",
    name_hash: 3349920219,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPlayerKilledMessage as Default>::default())),
            create_boxed: || Box::new(<ServerPlayerKilledMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ServerPlayerKilledMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPLAYERKILLEDMESSAGE_TYPE_INFO
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
pub struct ServerPlayerManuallySelectedSpawnPointMessage {
}

pub trait ServerPlayerManuallySelectedSpawnPointMessageTrait: TypeObject {
}

impl ServerPlayerManuallySelectedSpawnPointMessageTrait for ServerPlayerManuallySelectedSpawnPointMessage {
}

pub static SERVERPLAYERMANUALLYSELECTEDSPAWNPOINTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerManuallySelectedSpawnPointMessage",
    name_hash: 1131473193,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPlayerManuallySelectedSpawnPointMessage as Default>::default())),
            create_boxed: || Box::new(<ServerPlayerManuallySelectedSpawnPointMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerManuallySelectedSpawnPointMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPLAYERMANUALLYSELECTEDSPAWNPOINTMESSAGE_TYPE_INFO
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
pub struct ServerPlayerChangeChatChannelMessage {
}

pub trait ServerPlayerChangeChatChannelMessageTrait: TypeObject {
}

impl ServerPlayerChangeChatChannelMessageTrait for ServerPlayerChangeChatChannelMessage {
}

pub static SERVERPLAYERCHANGECHATCHANNELMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerChangeChatChannelMessage",
    name_hash: 841705347,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPlayerChangeChatChannelMessage as Default>::default())),
            create_boxed: || Box::new(<ServerPlayerChangeChatChannelMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerChangeChatChannelMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPLAYERCHANGECHATCHANNELMESSAGE_TYPE_INFO
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
pub struct ServerPlayerSwitchTeamMessage {
}

pub trait ServerPlayerSwitchTeamMessageTrait: TypeObject {
}

impl ServerPlayerSwitchTeamMessageTrait for ServerPlayerSwitchTeamMessage {
}

pub static SERVERPLAYERSWITCHTEAMMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerSwitchTeamMessage",
    name_hash: 3670462135,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPlayerSwitchTeamMessage as Default>::default())),
            create_boxed: || Box::new(<ServerPlayerSwitchTeamMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerSwitchTeamMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPLAYERSWITCHTEAMMESSAGE_TYPE_INFO
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
pub struct ServerPlayerKitPickedupMessage {
}

pub trait ServerPlayerKitPickedupMessageTrait: TypeObject {
}

impl ServerPlayerKitPickedupMessageTrait for ServerPlayerKitPickedupMessage {
}

pub static SERVERPLAYERKITPICKEDUPMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerKitPickedupMessage",
    name_hash: 3021255419,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPlayerKitPickedupMessage as Default>::default())),
            create_boxed: || Box::new(<ServerPlayerKitPickedupMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerKitPickedupMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPLAYERKITPICKEDUPMESSAGE_TYPE_INFO
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
pub struct ServerPlayerKitReplacedMessage {
}

pub trait ServerPlayerKitReplacedMessageTrait: TypeObject {
}

impl ServerPlayerKitReplacedMessageTrait for ServerPlayerKitReplacedMessage {
}

pub static SERVERPLAYERKITREPLACEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerKitReplacedMessage",
    name_hash: 3636399398,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPlayerKitReplacedMessage as Default>::default())),
            create_boxed: || Box::new(<ServerPlayerKitReplacedMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerKitReplacedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPLAYERKITREPLACEDMESSAGE_TYPE_INFO
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
pub struct ServerPlayerChangedCharacterMessage {
}

pub trait ServerPlayerChangedCharacterMessageTrait: TypeObject {
}

impl ServerPlayerChangedCharacterMessageTrait for ServerPlayerChangedCharacterMessage {
}

pub static SERVERPLAYERCHANGEDCHARACTERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerChangedCharacterMessage",
    name_hash: 1090141955,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPlayerChangedCharacterMessage as Default>::default())),
            create_boxed: || Box::new(<ServerPlayerChangedCharacterMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerChangedCharacterMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPLAYERCHANGEDCHARACTERMESSAGE_TYPE_INFO
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
pub struct ServerPlayerReviveRefusedMessage {
}

pub trait ServerPlayerReviveRefusedMessageTrait: TypeObject {
}

impl ServerPlayerReviveRefusedMessageTrait for ServerPlayerReviveRefusedMessage {
}

pub static SERVERPLAYERREVIVEREFUSEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerReviveRefusedMessage",
    name_hash: 3602871989,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPlayerReviveRefusedMessage as Default>::default())),
            create_boxed: || Box::new(<ServerPlayerReviveRefusedMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerReviveRefusedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPLAYERREVIVEREFUSEDMESSAGE_TYPE_INFO
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
pub struct ServerPlayerReviveAcceptedMessage {
}

pub trait ServerPlayerReviveAcceptedMessageTrait: TypeObject {
}

impl ServerPlayerReviveAcceptedMessageTrait for ServerPlayerReviveAcceptedMessage {
}

pub static SERVERPLAYERREVIVEACCEPTEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerReviveAcceptedMessage",
    name_hash: 2485552546,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPlayerReviveAcceptedMessage as Default>::default())),
            create_boxed: || Box::new(<ServerPlayerReviveAcceptedMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerReviveAcceptedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPLAYERREVIVEACCEPTEDMESSAGE_TYPE_INFO
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
pub struct ServerPlayerReviveMessage {
}

pub trait ServerPlayerReviveMessageTrait: TypeObject {
}

impl ServerPlayerReviveMessageTrait for ServerPlayerReviveMessage {
}

pub static SERVERPLAYERREVIVEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerReviveMessage",
    name_hash: 632760579,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPlayerReviveMessage as Default>::default())),
            create_boxed: || Box::new(<ServerPlayerReviveMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerReviveMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPLAYERREVIVEMESSAGE_TYPE_INFO
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
pub struct ServerPlayerLeftLevelMessage {
}

pub trait ServerPlayerLeftLevelMessageTrait: TypeObject {
}

impl ServerPlayerLeftLevelMessageTrait for ServerPlayerLeftLevelMessage {
}

pub static SERVERPLAYERLEFTLEVELMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerLeftLevelMessage",
    name_hash: 258059669,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPlayerLeftLevelMessage as Default>::default())),
            create_boxed: || Box::new(<ServerPlayerLeftLevelMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerLeftLevelMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPLAYERLEFTLEVELMESSAGE_TYPE_INFO
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
pub struct ServerPlayerReleasingLevelMessage {
}

pub trait ServerPlayerReleasingLevelMessageTrait: TypeObject {
}

impl ServerPlayerReleasingLevelMessageTrait for ServerPlayerReleasingLevelMessage {
}

pub static SERVERPLAYERRELEASINGLEVELMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerReleasingLevelMessage",
    name_hash: 2215478754,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPlayerReleasingLevelMessage as Default>::default())),
            create_boxed: || Box::new(<ServerPlayerReleasingLevelMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerReleasingLevelMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPLAYERRELEASINGLEVELMESSAGE_TYPE_INFO
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
pub struct ServerPlayerEnteredLevelMessage {
}

pub trait ServerPlayerEnteredLevelMessageTrait: TypeObject {
}

impl ServerPlayerEnteredLevelMessageTrait for ServerPlayerEnteredLevelMessage {
}

pub static SERVERPLAYERENTEREDLEVELMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerEnteredLevelMessage",
    name_hash: 163048007,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPlayerEnteredLevelMessage as Default>::default())),
            create_boxed: || Box::new(<ServerPlayerEnteredLevelMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerEnteredLevelMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPLAYERENTEREDLEVELMESSAGE_TYPE_INFO
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
pub struct ServerPlayerLevelLoadedMessage {
}

pub trait ServerPlayerLevelLoadedMessageTrait: TypeObject {
}

impl ServerPlayerLevelLoadedMessageTrait for ServerPlayerLevelLoadedMessage {
}

pub static SERVERPLAYERLEVELLOADEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerLevelLoadedMessage",
    name_hash: 1389366249,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPlayerLevelLoadedMessage as Default>::default())),
            create_boxed: || Box::new(<ServerPlayerLevelLoadedMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerLevelLoadedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPLAYERLEVELLOADEDMESSAGE_TYPE_INFO
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
pub struct ServerPlayerDebugFriendZoneSpawnMessage {
}

pub trait ServerPlayerDebugFriendZoneSpawnMessageTrait: TypeObject {
}

impl ServerPlayerDebugFriendZoneSpawnMessageTrait for ServerPlayerDebugFriendZoneSpawnMessage {
}

pub static SERVERPLAYERDEBUGFRIENDZONESPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerDebugFriendZoneSpawnMessage",
    name_hash: 172219454,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPlayerDebugFriendZoneSpawnMessage as Default>::default())),
            create_boxed: || Box::new(<ServerPlayerDebugFriendZoneSpawnMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ServerPlayerDebugFriendZoneSpawnMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPLAYERDEBUGFRIENDZONESPAWNMESSAGE_TYPE_INFO
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
pub struct ServerPlayerRespawnMessage {
}

pub trait ServerPlayerRespawnMessageTrait: TypeObject {
}

impl ServerPlayerRespawnMessageTrait for ServerPlayerRespawnMessage {
}

pub static SERVERPLAYERRESPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerRespawnMessage",
    name_hash: 3375736180,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPlayerRespawnMessage as Default>::default())),
            create_boxed: || Box::new(<ServerPlayerRespawnMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerRespawnMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPLAYERRESPAWNMESSAGE_TYPE_INFO
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
pub struct ServerPlayerDestroyMessage {
}

pub trait ServerPlayerDestroyMessageTrait: TypeObject {
}

impl ServerPlayerDestroyMessageTrait for ServerPlayerDestroyMessage {
}

pub static SERVERPLAYERDESTROYMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerDestroyMessage",
    name_hash: 3770668410,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPlayerDestroyMessage as Default>::default())),
            create_boxed: || Box::new(<ServerPlayerDestroyMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerDestroyMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPLAYERDESTROYMESSAGE_TYPE_INFO
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
pub struct ServerPlayerCreatedForConnectionMessage {
}

pub trait ServerPlayerCreatedForConnectionMessageTrait: TypeObject {
}

impl ServerPlayerCreatedForConnectionMessageTrait for ServerPlayerCreatedForConnectionMessage {
}

pub static SERVERPLAYERCREATEDFORCONNECTIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerCreatedForConnectionMessage",
    name_hash: 3266297109,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPlayerCreatedForConnectionMessage as Default>::default())),
            create_boxed: || Box::new(<ServerPlayerCreatedForConnectionMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerCreatedForConnectionMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPLAYERCREATEDFORCONNECTIONMESSAGE_TYPE_INFO
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
pub struct ServerPlayerCreateMessage {
}

pub trait ServerPlayerCreateMessageTrait: TypeObject {
}

impl ServerPlayerCreateMessageTrait for ServerPlayerCreateMessage {
}

pub static SERVERPLAYERCREATEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerCreateMessage",
    name_hash: 810674236,
    flags: MemberInfoFlags::new(73),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPlayerCreateMessage as Default>::default())),
            create_boxed: || Box::new(<ServerPlayerCreateMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerCreateMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPLAYERCREATEMESSAGE_TYPE_INFO
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
pub struct ServerPlayerAboutToCreateForConnectionMessage {
}

pub trait ServerPlayerAboutToCreateForConnectionMessageTrait: TypeObject {
}

impl ServerPlayerAboutToCreateForConnectionMessageTrait for ServerPlayerAboutToCreateForConnectionMessage {
}

pub static SERVERPLAYERABOUTTOCREATEFORCONNECTIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerAboutToCreateForConnectionMessage",
    name_hash: 1231268711,
    flags: MemberInfoFlags::new(73),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPlayerAboutToCreateForConnectionMessage as Default>::default())),
            create_boxed: || Box::new(<ServerPlayerAboutToCreateForConnectionMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPlayerAboutToCreateForConnectionMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPLAYERABOUTTOCREATEFORCONNECTIONMESSAGE_TYPE_INFO
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
pub struct ServerCharacterCharacterDamageMessage {
}

pub trait ServerCharacterCharacterDamageMessageTrait: TypeObject {
}

impl ServerCharacterCharacterDamageMessageTrait for ServerCharacterCharacterDamageMessage {
}

pub static SERVERCHARACTERCHARACTERDAMAGEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterCharacterDamageMessage",
    name_hash: 2371731360,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCharacterCharacterDamageMessage as Default>::default())),
            create_boxed: || Box::new(<ServerCharacterCharacterDamageMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerCharacterCharacterDamageMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCHARACTERCHARACTERDAMAGEMESSAGE_TYPE_INFO
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
pub struct ServerCharacterKilledMessage {
}

pub trait ServerCharacterKilledMessageTrait: TypeObject {
}

impl ServerCharacterKilledMessageTrait for ServerCharacterKilledMessage {
}

pub static SERVERCHARACTERKILLEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterKilledMessage",
    name_hash: 1860476145,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCharacterKilledMessage as Default>::default())),
            create_boxed: || Box::new(<ServerCharacterKilledMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerCharacterKilledMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCHARACTERKILLEDMESSAGE_TYPE_INFO
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
pub struct ServerMetricsDetonateExplosionMessage {
}

pub trait ServerMetricsDetonateExplosionMessageTrait: TypeObject {
}

impl ServerMetricsDetonateExplosionMessageTrait for ServerMetricsDetonateExplosionMessage {
}

pub static SERVERMETRICSDETONATEEXPLOSIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMetricsDetonateExplosionMessage",
    name_hash: 3847313741,
    flags: MemberInfoFlags::new(36937),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerMetricsDetonateExplosionMessage as Default>::default())),
            create_boxed: || Box::new(<ServerMetricsDetonateExplosionMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ServerMetricsDetonateExplosionMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERMETRICSDETONATEEXPLOSIONMESSAGE_TYPE_INFO
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
pub struct ServerMetricsObjectiveSuccessMessage {
}

pub trait ServerMetricsObjectiveSuccessMessageTrait: TypeObject {
}

impl ServerMetricsObjectiveSuccessMessageTrait for ServerMetricsObjectiveSuccessMessage {
}

pub static SERVERMETRICSOBJECTIVESUCCESSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMetricsObjectiveSuccessMessage",
    name_hash: 1613303952,
    flags: MemberInfoFlags::new(73),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerMetricsObjectiveSuccessMessage as Default>::default())),
            create_boxed: || Box::new(<ServerMetricsObjectiveSuccessMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerMetricsObjectiveSuccessMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERMETRICSOBJECTIVESUCCESSMESSAGE_TYPE_INFO
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
pub struct ServerMetricsSaveGameSavedMessage {
}

pub trait ServerMetricsSaveGameSavedMessageTrait: TypeObject {
}

impl ServerMetricsSaveGameSavedMessageTrait for ServerMetricsSaveGameSavedMessage {
}

pub static SERVERMETRICSSAVEGAMESAVEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMetricsSaveGameSavedMessage",
    name_hash: 1894707990,
    flags: MemberInfoFlags::new(73),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerMetricsSaveGameSavedMessage as Default>::default())),
            create_boxed: || Box::new(<ServerMetricsSaveGameSavedMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerMetricsSaveGameSavedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERMETRICSSAVEGAMESAVEDMESSAGE_TYPE_INFO
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
pub struct ServerMetricsSaveGameLoadedMessage {
}

pub trait ServerMetricsSaveGameLoadedMessageTrait: TypeObject {
}

impl ServerMetricsSaveGameLoadedMessageTrait for ServerMetricsSaveGameLoadedMessage {
}

pub static SERVERMETRICSSAVEGAMELOADEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMetricsSaveGameLoadedMessage",
    name_hash: 3170584244,
    flags: MemberInfoFlags::new(73),
    module: "GameServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerMetricsSaveGameLoadedMessage as Default>::default())),
            create_boxed: || Box::new(<ServerMetricsSaveGameLoadedMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerMetricsSaveGameLoadedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERMETRICSSAVEGAMELOADEDMESSAGE_TYPE_INFO
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
pub struct ServerWaterEntity {
    pub _glacier_base: ServerPhysicsEntity,
}

pub trait ServerWaterEntityTrait: ServerPhysicsEntityTrait {
}

impl ServerWaterEntityTrait for ServerWaterEntity {
}

impl ServerPhysicsEntityTrait for ServerWaterEntity {
}

impl ServerGameComponentEntityTrait for ServerWaterEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ServerWaterEntity {
}

impl super::entity::ComponentEntityTrait for ServerWaterEntity {
}

impl super::entity::SpatialEntityTrait for ServerWaterEntity {
}

impl super::entity::EntityTrait for ServerWaterEntity {
}

impl super::entity::EntityBusPeerTrait for ServerWaterEntity {
}

pub static SERVERWATERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWaterEntity",
    name_hash: 4075966254,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERPHYSICSENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerWaterEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerWaterEntity as Default>::default())),
            create_boxed: || Box::new(<ServerWaterEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERWATERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerWaterEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERWATERENTITY_TYPE_INFO
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


pub static SERVERWATERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWaterEntity-Array",
    name_hash: 2389120922,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerWaterEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerTerrainEntity {
    pub _glacier_base: ServerPhysicsEntity,
}

pub trait ServerTerrainEntityTrait: ServerPhysicsEntityTrait {
}

impl ServerTerrainEntityTrait for ServerTerrainEntity {
}

impl ServerPhysicsEntityTrait for ServerTerrainEntity {
}

impl ServerGameComponentEntityTrait for ServerTerrainEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ServerTerrainEntity {
}

impl super::entity::ComponentEntityTrait for ServerTerrainEntity {
}

impl super::entity::SpatialEntityTrait for ServerTerrainEntity {
}

impl super::entity::EntityTrait for ServerTerrainEntity {
}

impl super::entity::EntityBusPeerTrait for ServerTerrainEntity {
}

pub static SERVERTERRAINENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTerrainEntity",
    name_hash: 1086222380,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERPHYSICSENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerTerrainEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerTerrainEntity as Default>::default())),
            create_boxed: || Box::new(<ServerTerrainEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERTERRAINENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerTerrainEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERTERRAINENTITY_TYPE_INFO
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


pub static SERVERTERRAINENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTerrainEntity-Array",
    name_hash: 2899279768,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerTerrainEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerTerrainDynamicDecalEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerTerrainDynamicDecalEntityTrait: super::entity::EntityTrait {
}

impl ServerTerrainDynamicDecalEntityTrait for ServerTerrainDynamicDecalEntity {
}

impl super::entity::EntityTrait for ServerTerrainDynamicDecalEntity {
}

impl super::entity::EntityBusPeerTrait for ServerTerrainDynamicDecalEntity {
}

pub static SERVERTERRAINDYNAMICDECALENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTerrainDynamicDecalEntity",
    name_hash: 1455104630,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerTerrainDynamicDecalEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerTerrainDynamicDecalEntity as Default>::default())),
            create_boxed: || Box::new(<ServerTerrainDynamicDecalEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERTERRAINDYNAMICDECALENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerTerrainDynamicDecalEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERTERRAINDYNAMICDECALENTITY_TYPE_INFO
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


pub static SERVERTERRAINDYNAMICDECALENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTerrainDynamicDecalEntity-Array",
    name_hash: 2861860674,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerTerrainDynamicDecalEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerStaticModelGroupEntity {
    pub _glacier_base: ServerPhysicsEntity,
}

pub trait ServerStaticModelGroupEntityTrait: ServerPhysicsEntityTrait {
}

impl ServerStaticModelGroupEntityTrait for ServerStaticModelGroupEntity {
}

impl ServerPhysicsEntityTrait for ServerStaticModelGroupEntity {
}

impl ServerGameComponentEntityTrait for ServerStaticModelGroupEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ServerStaticModelGroupEntity {
}

impl super::entity::ComponentEntityTrait for ServerStaticModelGroupEntity {
}

impl super::entity::SpatialEntityTrait for ServerStaticModelGroupEntity {
}

impl super::entity::EntityTrait for ServerStaticModelGroupEntity {
}

impl super::entity::EntityBusPeerTrait for ServerStaticModelGroupEntity {
}

pub static SERVERSTATICMODELGROUPENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticModelGroupEntity",
    name_hash: 3791240435,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERPHYSICSENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerStaticModelGroupEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerStaticModelGroupEntity as Default>::default())),
            create_boxed: || Box::new(<ServerStaticModelGroupEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSTATICMODELGROUPENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerStaticModelGroupEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSTATICMODELGROUPENTITY_TYPE_INFO
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


pub static SERVERSTATICMODELGROUPENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticModelGroupEntity-Array",
    name_hash: 4155037895,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerStaticModelGroupEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerStaticModelEntity {
    pub _glacier_base: ServerPhysicsEntity,
}

pub trait ServerStaticModelEntityTrait: ServerPhysicsEntityTrait {
}

impl ServerStaticModelEntityTrait for ServerStaticModelEntity {
}

impl ServerPhysicsEntityTrait for ServerStaticModelEntity {
}

impl ServerGameComponentEntityTrait for ServerStaticModelEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ServerStaticModelEntity {
}

impl super::entity::ComponentEntityTrait for ServerStaticModelEntity {
}

impl super::entity::SpatialEntityTrait for ServerStaticModelEntity {
}

impl super::entity::EntityTrait for ServerStaticModelEntity {
}

impl super::entity::EntityBusPeerTrait for ServerStaticModelEntity {
}

pub static SERVERSTATICMODELENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticModelEntity",
    name_hash: 634378476,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERPHYSICSENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerStaticModelEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerStaticModelEntity as Default>::default())),
            create_boxed: || Box::new(<ServerStaticModelEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSTATICMODELENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerStaticModelEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSTATICMODELENTITY_TYPE_INFO
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


pub static SERVERSTATICMODELENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticModelEntity-Array",
    name_hash: 3057154264,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerStaticModelEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerLadderEntity {
    pub _glacier_base: ServerStaticModelEntity,
}

pub trait ServerLadderEntityTrait: ServerStaticModelEntityTrait {
}

impl ServerLadderEntityTrait for ServerLadderEntity {
}

impl ServerStaticModelEntityTrait for ServerLadderEntity {
}

impl ServerPhysicsEntityTrait for ServerLadderEntity {
}

impl ServerGameComponentEntityTrait for ServerLadderEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ServerLadderEntity {
}

impl super::entity::ComponentEntityTrait for ServerLadderEntity {
}

impl super::entity::SpatialEntityTrait for ServerLadderEntity {
}

impl super::entity::EntityTrait for ServerLadderEntity {
}

impl super::entity::EntityBusPeerTrait for ServerLadderEntity {
}

pub static SERVERLADDERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLadderEntity",
    name_hash: 3197603169,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERSTATICMODELENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerLadderEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerLadderEntity as Default>::default())),
            create_boxed: || Box::new(<ServerLadderEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERLADDERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerLadderEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERLADDERENTITY_TYPE_INFO
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


pub static SERVERLADDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLadderEntity-Array",
    name_hash: 2067925077,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerLadderEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerInteractableStaticModelEntity {
    pub _glacier_base: ServerStaticModelEntity,
}

pub trait ServerInteractableStaticModelEntityTrait: ServerStaticModelEntityTrait {
}

impl ServerInteractableStaticModelEntityTrait for ServerInteractableStaticModelEntity {
}

impl ServerStaticModelEntityTrait for ServerInteractableStaticModelEntity {
}

impl ServerPhysicsEntityTrait for ServerInteractableStaticModelEntity {
}

impl ServerGameComponentEntityTrait for ServerInteractableStaticModelEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ServerInteractableStaticModelEntity {
}

impl super::entity::ComponentEntityTrait for ServerInteractableStaticModelEntity {
}

impl super::entity::SpatialEntityTrait for ServerInteractableStaticModelEntity {
}

impl super::entity::EntityTrait for ServerInteractableStaticModelEntity {
}

impl super::entity::EntityBusPeerTrait for ServerInteractableStaticModelEntity {
}

pub static SERVERINTERACTABLESTATICMODELENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerInteractableStaticModelEntity",
    name_hash: 3849645620,
    flags: MemberInfoFlags::new(101),
    module: "GameServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERSTATICMODELENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerInteractableStaticModelEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerInteractableStaticModelEntity as Default>::default())),
            create_boxed: || Box::new(<ServerInteractableStaticModelEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERINTERACTABLESTATICMODELENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerInteractableStaticModelEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERINTERACTABLESTATICMODELENTITY_TYPE_INFO
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


pub static SERVERINTERACTABLESTATICMODELENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerInteractableStaticModelEntity-Array",
    name_hash: 956295552,
    flags: MemberInfoFlags::new(145),
    module: "GameServer",
    data: TypeInfoData::Array("ServerInteractableStaticModelEntity"),
    array_type: None,
    alignment: 8,
};


