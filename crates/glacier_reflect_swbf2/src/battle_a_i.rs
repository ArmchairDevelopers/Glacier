use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_battle_a_i_types(registry: &mut TypeRegistry) {
    registry.register_type(SERVERSIMPLEDRIVERCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERSIMPLEDRIVERCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(WSSERVERDOGFIGHTINGENTITY_TYPE_INFO);
    registry.register_type(WSSERVERDOGFIGHTINGENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSTRAFERUNMANEUVERENTITY_TYPE_INFO);
    registry.register_type(SERVERSTRAFERUNMANEUVERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSTALLTURNMANEUVERENTITY_TYPE_INFO);
    registry.register_type(SERVERSTALLTURNMANEUVERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSPLITSMANEUVERENTITY_TYPE_INFO);
    registry.register_type(SERVERSPLITSMANEUVERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSPINDESCENTMANEUVERENTITY_TYPE_INFO);
    registry.register_type(SERVERSPINDESCENTMANEUVERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSIDETOSIDEMANEUVERENTITY_TYPE_INFO);
    registry.register_type(SERVERSIDETOSIDEMANEUVERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSETWAYPOINTSENTITY_TYPE_INFO);
    registry.register_type(SERVERSETWAYPOINTSENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERPROTECTBASEMANEUVERENTITY_TYPE_INFO);
    registry.register_type(SERVERPROTECTBASEMANEUVERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERPILOTENTITYBASE_TYPE_INFO);
    registry.register_type(SERVERPILOTENTITYBASE_ARRAY_TYPE_INFO);
    registry.register_type(SERVERPILOTENTITY_TYPE_INFO);
    registry.register_type(SERVERPILOTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERMANEUVERSELECTORENTITY_TYPE_INFO);
    registry.register_type(SERVERMANEUVERSELECTORENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERIMMELMANNMANEUVERENTITY_TYPE_INFO);
    registry.register_type(SERVERIMMELMANNMANEUVERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERHEAVYPLANEPILOTENTITY_TYPE_INFO);
    registry.register_type(SERVERHEAVYPLANEPILOTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERFOLLOWWAYPOINTSMANEUVERENTITY_TYPE_INFO);
    registry.register_type(SERVERFOLLOWWAYPOINTSMANEUVERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERFLYTOMANEUVERENTITY_TYPE_INFO);
    registry.register_type(SERVERFLYTOMANEUVERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERENFORCEALTITUDEMANEUVERENTITY_TYPE_INFO);
    registry.register_type(SERVERENFORCEALTITUDEMANEUVERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO);
    registry.register_type(SERVERDOGFIGHTMANEUVERENTITYBASE_ARRAY_TYPE_INFO);
    registry.register_type(SERVERDOGFIGHTINGENTITY_TYPE_INFO);
    registry.register_type(SERVERDOGFIGHTINGENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERDEFENSIVEMANEUVERSELECTORENTITY_TYPE_INFO);
    registry.register_type(SERVERDEFENSIVEMANEUVERSELECTORENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCREATEDISTANCEMANEUVERENTITY_TYPE_INFO);
    registry.register_type(SERVERCREATEDISTANCEMANEUVERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCOLLISIONAVOIDANCEMANEUVERENTITY_TYPE_INFO);
    registry.register_type(SERVERCOLLISIONAVOIDANCEMANEUVERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERBASICDEFENSIVEMANEUVERENTITY_TYPE_INFO);
    registry.register_type(SERVERBASICDEFENSIVEMANEUVERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERBASICATTACKMANEUVERENTITY_TYPE_INFO);
    registry.register_type(SERVERBASICATTACKMANEUVERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERBARRELROLLMANEUVERENTITY_TYPE_INFO);
    registry.register_type(SERVERBARRELROLLMANEUVERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAIRTARGETSELECTORENTITY_TYPE_INFO);
    registry.register_type(SERVERAIRTARGETSELECTORENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAIRCOLLISIONAVOIDANCEENTITY_TYPE_INFO);
    registry.register_type(SERVERAIRCOLLISIONAVOIDANCEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(BFSERVERDOGFIGHTINGENTITY_TYPE_INFO);
    registry.register_type(BFSERVERDOGFIGHTINGENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERWAYPOINTSWALKERENTITY_TYPE_INFO);
    registry.register_type(SERVERWAYPOINTSWALKERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAIPROXIMITYREACTIONSCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERAIPROXIMITYREACTIONSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAILOCOCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERAILOCOCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(AIBLOCKERENTITY_TYPE_INFO);
    registry.register_type(AIBLOCKERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTAIPROXIMITYREACTIONSCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTAIPROXIMITYREACTIONSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTAILOCOCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTAILOCOCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SPATIALANALYZER_TYPE_INFO);
    registry.register_type(SPATIALANALYZER_ARRAY_TYPE_INFO);
    registry.register_type(SERVERPLAYERVEHICLEPROXIMITYENTITY_TYPE_INFO);
    registry.register_type(SERVERPLAYERVEHICLEPROXIMITYENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERINVESTIGATESETTINGSOVERRIDE_TYPE_INFO);
    registry.register_type(SERVERINVESTIGATESETTINGSOVERRIDE_ARRAY_TYPE_INFO);
    registry.register_type(SERVERDAMAGEMODIFIERENTITY_TYPE_INFO);
    registry.register_type(SERVERDAMAGEMODIFIERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERATTACKPOINTENTITY_TYPE_INFO);
    registry.register_type(SERVERATTACKPOINTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAIVEHICLECOMBATENTITY_TYPE_INFO);
    registry.register_type(SERVERAIVEHICLECOMBATENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAITEMPLATEFILTERENTITY_TYPE_INFO);
    registry.register_type(SERVERAITEMPLATEFILTERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAITELEPORTENTITY_TYPE_INFO);
    registry.register_type(SERVERAITELEPORTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAISYSTEMENTITY_TYPE_INFO);
    registry.register_type(SERVERAISYSTEMENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAISTATEENTITY_TYPE_INFO);
    registry.register_type(SERVERAISTATEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAISOUNDENTITY_TYPE_INFO);
    registry.register_type(SERVERAISOUNDENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAICANCELORDER_TYPE_INFO);
    registry.register_type(SERVERAICANCELORDER_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAIGOTOPLACEORDERENTITYDATA_TYPE_INFO);
    registry.register_type(SERVERAIGOTOPLACEORDERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAIFOLLOWWAYPOINTSORDER_TYPE_INFO);
    registry.register_type(SERVERAIFOLLOWWAYPOINTSORDER_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAIORDERENTITYBASE_TYPE_INFO);
    registry.register_type(SERVERAIORDERENTITYBASE_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAISELFDESTRUCTENTITY_TYPE_INFO);
    registry.register_type(SERVERAISELFDESTRUCTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAICOVERZONESOVERRIDEENTITY_TYPE_INFO);
    registry.register_type(SERVERAICOVERZONESOVERRIDEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAIAWARENESSENTITY_TYPE_INFO);
    registry.register_type(SERVERAIAWARENESSENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAIWEAPONSLOTOVERRIDEENTITY_TYPE_INFO);
    registry.register_type(SERVERAIWEAPONSLOTOVERRIDEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAITARGETCOORDINATORFILTERENTITY_TYPE_INFO);
    registry.register_type(SERVERAITARGETCOORDINATORFILTERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAITARGETCOORDINATORENTITY_TYPE_INFO);
    registry.register_type(SERVERAITARGETCOORDINATORENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCLOAKINGMODIFIERENTITY_TYPE_INFO);
    registry.register_type(SERVERCLOAKINGMODIFIERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSENSINGAREAMODIFIERENTITY_TYPE_INFO);
    registry.register_type(SERVERSENSINGAREAMODIFIERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAISTEALTHENTITY_TYPE_INFO);
    registry.register_type(SERVERAISTEALTHENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAIBUDDYFOLLOWENTITY_TYPE_INFO);
    registry.register_type(SERVERAIBUDDYFOLLOWENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAIFOLLOWOBJECTENTITY_TYPE_INFO);
    registry.register_type(SERVERAIFOLLOWOBJECTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAIPREFERREDAREAENTITY_TYPE_INFO);
    registry.register_type(SERVERAIPREFERREDAREAENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAISOUNDAREAENTITY_TYPE_INFO);
    registry.register_type(SERVERAISOUNDAREAENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAICOMBATGROUPENTITY_TYPE_INFO);
    registry.register_type(SERVERAICOMBATGROUPENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAICOVERQUERYENTITY_TYPE_INFO);
    registry.register_type(SERVERAICOVERQUERYENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAITACTICENTITY_TYPE_INFO);
    registry.register_type(SERVERAITACTICENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAISHOOTATTARGETSENTITY_TYPE_INFO);
    registry.register_type(SERVERAISHOOTATTARGETSENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAIUSEWAYPOINTSENTITY_TYPE_INFO);
    registry.register_type(SERVERAIUSEWAYPOINTSENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAIUSECOVERENTITY_TYPE_INFO);
    registry.register_type(SERVERAIUSECOVERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAIWEAPONOVERRIDEENTITY_TYPE_INFO);
    registry.register_type(SERVERAIWEAPONOVERRIDEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAIVEHICLEBEHAVIORENTITY_TYPE_INFO);
    registry.register_type(SERVERAIVEHICLEBEHAVIORENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAIHEARINGPARAMETERENTITY_TYPE_INFO);
    registry.register_type(SERVERAIHEARINGPARAMETERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAISENSINGPARAMETERENTITY_TYPE_INFO);
    registry.register_type(SERVERAISENSINGPARAMETERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAIIDLEBEHAVIORENTITY_TYPE_INFO);
    registry.register_type(SERVERAIIDLEBEHAVIORENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAITARGETINGENTITY_TYPE_INFO);
    registry.register_type(SERVERAITARGETINGENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAICOMBATBEHAVIORENTITY_TYPE_INFO);
    registry.register_type(SERVERAICOMBATBEHAVIORENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAIFOLLOWAREAENTITY_TYPE_INFO);
    registry.register_type(SERVERAIFOLLOWAREAENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAIFORBIDDENAREAENTITY_TYPE_INFO);
    registry.register_type(SERVERAIFORBIDDENAREAENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAIFRIENDLYAREAENTITY_TYPE_INFO);
    registry.register_type(SERVERAIFRIENDLYAREAENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAIFLANKINGCORRIDORENTITY_TYPE_INFO);
    registry.register_type(SERVERAIFLANKINGCORRIDORENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAISEARCHAREAENTITY_TYPE_INFO);
    registry.register_type(SERVERAISEARCHAREAENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAIDEFENDAREAENTITY_TYPE_INFO);
    registry.register_type(SERVERAIDEFENDAREAENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAIPARAMETERWITHSHAPEENTITY_TYPE_INFO);
    registry.register_type(SERVERAIPARAMETERWITHSHAPEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAIPARAMETERENTITY_TYPE_INFO);
    registry.register_type(SERVERAIPARAMETERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAIOBSTACLECONTROLLERENTITY_TYPE_INFO);
    registry.register_type(SERVERAIOBSTACLECONTROLLERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAIKILLCOUNTERENTITY_TYPE_INFO);
    registry.register_type(SERVERAIKILLCOUNTERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAIFIREPATTERNENTITY_TYPE_INFO);
    registry.register_type(SERVERAIFIREPATTERNENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAIENCOUNTERMANAGERENTITY_TYPE_INFO);
    registry.register_type(SERVERAIENCOUNTERMANAGERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAIDEBUGPROXY_TYPE_INFO);
    registry.register_type(SERVERAIDEBUGPROXY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERACTIONENTITY_TYPE_INFO);
    registry.register_type(SERVERACTIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(AICONCEALMENTVOLUMEENTITY_TYPE_INFO);
    registry.register_type(AICONCEALMENTVOLUMEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTAITEMPLATEFILTERENTITY_TYPE_INFO);
    registry.register_type(CLIENTAITEMPLATEFILTERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTAISTATEENTITY_TYPE_INFO);
    registry.register_type(CLIENTAISTATEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTAIPHYSICSDRIVENANIMATIONENTITY_TYPE_INFO);
    registry.register_type(CLIENTAIPHYSICSDRIVENANIMATIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTAICOLLISIONAVOIDANCESETUPENTITY_TYPE_INFO);
    registry.register_type(CLIENTAICOLLISIONAVOIDANCESETUPENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTACTIONENTITY_TYPE_INFO);
    registry.register_type(CLIENTACTIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(AITEMPLATECONDITIONEVALUATOR_TYPE_INFO);
    registry.register_type(AITEMPLATECONDITIONEVALUATOR_ARRAY_TYPE_INFO);
    registry.register_type(INSIDEVOLUMESCONDITIONEVALUATOR_TYPE_INFO);
    registry.register_type(INSIDEVOLUMESCONDITIONEVALUATOR_ARRAY_TYPE_INFO);
    registry.register_type(PROBABILITYCONDITIONEVALUATOR_TYPE_INFO);
    registry.register_type(PROBABILITYCONDITIONEVALUATOR_ARRAY_TYPE_INFO);
    registry.register_type(FACINGCONDITIONEVALUATOR_TYPE_INFO);
    registry.register_type(FACINGCONDITIONEVALUATOR_ARRAY_TYPE_INFO);
    registry.register_type(AIORDERCOORDINATORCONDITIONEVALUATOR_TYPE_INFO);
    registry.register_type(AIORDERCOORDINATORCONDITIONEVALUATOR_ARRAY_TYPE_INFO);
    registry.register_type(AISTATECONDITIONEVALUATOR_TYPE_INFO);
    registry.register_type(AISTATECONDITIONEVALUATOR_ARRAY_TYPE_INFO);
    registry.register_type(RANGECONDITIONEVALUATOR_TYPE_INFO);
    registry.register_type(RANGECONDITIONEVALUATOR_ARRAY_TYPE_INFO);
    registry.register_type(IACTIONCONDITIONEVALUATOR_TYPE_INFO);
    registry.register_type(IACTIONCONDITIONEVALUATOR_ARRAY_TYPE_INFO);
    registry.register_type(SERVERPOINTOFINTERESTCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERPOINTOFINTERESTCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCOVERENTITY_TYPE_INFO);
    registry.register_type(SERVERCOVERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCOVERGROUPENTITY_TYPE_INFO);
    registry.register_type(SERVERCOVERGROUPENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAIVEHICLEAIMINGCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERAIVEHICLEAIMINGCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAITARGETCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERAITARGETCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAISUPPRESSWEAPONFIRINGCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERAISUPPRESSWEAPONFIRINGCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAISPOTTINGCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERAISPOTTINGCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAISMOKEVOLUMECOMPONENT_TYPE_INFO);
    registry.register_type(SERVERAISMOKEVOLUMECOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAIPROJECTILECOMPONENT_TYPE_INFO);
    registry.register_type(SERVERAIPROJECTILECOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAIORDERCOORDINATORCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERAIORDERCOORDINATORCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAIMELEECOMPONENT_TYPE_INFO);
    registry.register_type(SERVERAIMELEECOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAIENTRYCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERAIENTRYCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAICUSTOMINPUTCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERAICUSTOMINPUTCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAIBUCKETSYSTEMCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERAIBUCKETSYSTEMCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAIANCHORTOPOINTCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERAIANCHORTOPOINTCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTAISPOTTINGCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTAISPOTTINGCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTAIPROJECTILECOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTAIPROJECTILECOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTAIORDERCOORDINATORCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTAIORDERCOORDINATORCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTAIMELEECOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTAIMELEECOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTAIANCHORTOPOINTCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTAIANCHORTOPOINTCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(COVERSCOREMODIFIERENTITY_TYPE_INFO);
    registry.register_type(COVERSCOREMODIFIERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERPATHLINKENTITY_TYPE_INFO);
    registry.register_type(SERVERPATHLINKENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERPATHFINDINGSTREAMENTITY_TYPE_INFO);
    registry.register_type(SERVERPATHFINDINGSTREAMENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERNAVPOWEROBSTACLECOMPONENT_TYPE_INFO);
    registry.register_type(SERVERNAVPOWEROBSTACLECOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(PATHLINKENTITY_TYPE_INFO);
    registry.register_type(PATHLINKENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERWAYPOINTTRIGGERENTITY_TYPE_INFO);
    registry.register_type(SERVERWAYPOINTTRIGGERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERPATHFOLLOWINGCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERPATHFOLLOWINGCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERPATHFINDINGOVERRIDE_TYPE_INFO);
    registry.register_type(SERVERPATHFINDINGOVERRIDE_ARRAY_TYPE_INFO);
    registry.register_type(SERVERFOLLOWWAYPOINTSENTITY_TYPE_INFO);
    registry.register_type(SERVERFOLLOWWAYPOINTSENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERFOLLOWOBJECTENTITY_TYPE_INFO);
    registry.register_type(SERVERFOLLOWOBJECTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SPATIALANALYZERDATA_TYPE_INFO);
    registry.register_type(SPATIALANALYZERDATA_ARRAY_TYPE_INFO);
    registry.register_type(ATTACKPOINTENTITYDATA_TYPE_INFO);
    registry.register_type(ATTACKPOINTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(ACTIONENTITYDATA_TYPE_INFO);
    registry.register_type(ACTIONENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(VALIDINSTATE_TYPE_INFO);
    registry.register_type(VALIDINSTATE_ARRAY_TYPE_INFO);
    registry.register_type(ACTIONPRIORITY_TYPE_INFO);
    registry.register_type(ACTIONPRIORITY_ARRAY_TYPE_INFO);
    registry.register_type(ACTIONSTATE_TYPE_INFO);
    registry.register_type(ACTIONSTATE_ARRAY_TYPE_INFO);
    registry.register_type(AITEMPLATECONDITION_TYPE_INFO);
    registry.register_type(AITEMPLATECONDITION_ARRAY_TYPE_INFO);
    registry.register_type(INSIDEVOLUMESCONDITION_TYPE_INFO);
    registry.register_type(INSIDEVOLUMESCONDITION_ARRAY_TYPE_INFO);
    registry.register_type(PROBABILITYCONDITION_TYPE_INFO);
    registry.register_type(PROBABILITYCONDITION_ARRAY_TYPE_INFO);
    registry.register_type(FACINGCONDITION_TYPE_INFO);
    registry.register_type(FACINGCONDITION_ARRAY_TYPE_INFO);
    registry.register_type(AIORDERCOORDINATORCONDITION_TYPE_INFO);
    registry.register_type(AIORDERCOORDINATORCONDITION_ARRAY_TYPE_INFO);
    registry.register_type(AISTATECONDITION_TYPE_INFO);
    registry.register_type(AISTATECONDITION_ARRAY_TYPE_INFO);
    registry.register_type(CONDITIONAISTATES_TYPE_INFO);
    registry.register_type(CONDITIONAISTATES_ARRAY_TYPE_INFO);
    registry.register_type(RANGECONDITION_TYPE_INFO);
    registry.register_type(RANGECONDITION_ARRAY_TYPE_INFO);
    registry.register_type(ACTIONCONDITION_TYPE_INFO);
    registry.register_type(ACTIONCONDITION_ARRAY_TYPE_INFO);
    registry.register_type(PREFERREDCOVERSCOREDATA_TYPE_INFO);
    registry.register_type(PREFERREDCOVERSCOREDATA_ARRAY_TYPE_INFO);
    registry.register_type(PREFERREDAREASCOREDATA_TYPE_INFO);
    registry.register_type(PREFERREDAREASCOREDATA_ARRAY_TYPE_INFO);
    registry.register_type(BASESHAPEWITHOFFSET_TYPE_INFO);
    registry.register_type(BASESHAPEWITHOFFSET_ARRAY_TYPE_INFO);
    registry.register_type(CUSTOMCOVERSCOREDATA_TYPE_INFO);
    registry.register_type(CUSTOMCOVERSCOREDATA_ARRAY_TYPE_INFO);
    registry.register_type(REDUCEPATHDISTANCETOFOLLOWOBJECTSCOREDATA_TYPE_INFO);
    registry.register_type(REDUCEPATHDISTANCETOFOLLOWOBJECTSCOREDATA_ARRAY_TYPE_INFO);
    registry.register_type(REDUCEPATHDISTANCETOTARGETSCOREDATA_TYPE_INFO);
    registry.register_type(REDUCEPATHDISTANCETOTARGETSCOREDATA_ARRAY_TYPE_INFO);
    registry.register_type(LINEOFFIRESCOREDATA_TYPE_INFO);
    registry.register_type(LINEOFFIRESCOREDATA_ARRAY_TYPE_INFO);
    registry.register_type(NAVPROBESCOREDATA_TYPE_INFO);
    registry.register_type(NAVPROBESCOREDATA_ARRAY_TYPE_INFO);
    registry.register_type(BEHINDFOLLOWOBJECTSCOREDATA_TYPE_INFO);
    registry.register_type(BEHINDFOLLOWOBJECTSCOREDATA_ARRAY_TYPE_INFO);
    registry.register_type(ANGLEFROMFOLLOWOBJECTAIMDIRECTIONDATA_TYPE_INFO);
    registry.register_type(ANGLEFROMFOLLOWOBJECTAIMDIRECTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(FOLLOWOBJECTRETICLEAVOIDANCEDATA_TYPE_INFO);
    registry.register_type(FOLLOWOBJECTRETICLEAVOIDANCEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PATHAVOIDANCESCOREDATA_TYPE_INFO);
    registry.register_type(PATHAVOIDANCESCOREDATA_ARRAY_TYPE_INFO);
    registry.register_type(PATHDISTANCESCOREDATA_TYPE_INFO);
    registry.register_type(PATHDISTANCESCOREDATA_ARRAY_TYPE_INFO);
    registry.register_type(REJECTUNREACHABLECOVERSCOREDATA_TYPE_INFO);
    registry.register_type(REJECTUNREACHABLECOVERSCOREDATA_ARRAY_TYPE_INFO);
    registry.register_type(DISTANCETOCORPSEDATA_TYPE_INFO);
    registry.register_type(DISTANCETOCORPSEDATA_ARRAY_TYPE_INFO);
    registry.register_type(DISTANCETOCLOSESTENEMYCOVERSCOREDATA_TYPE_INFO);
    registry.register_type(DISTANCETOCLOSESTENEMYCOVERSCOREDATA_ARRAY_TYPE_INFO);
    registry.register_type(DISTANCETOCLOSESTFRIENDLYSCOREDATA_TYPE_INFO);
    registry.register_type(DISTANCETOCLOSESTFRIENDLYSCOREDATA_ARRAY_TYPE_INFO);
    registry.register_type(EXPOSURETOMULTITARGETSSCOREDATA_TYPE_INFO);
    registry.register_type(EXPOSURETOMULTITARGETSSCOREDATA_ARRAY_TYPE_INFO);
    registry.register_type(DISTANCETOMULTITARGETSSCOREDATA_TYPE_INFO);
    registry.register_type(DISTANCETOMULTITARGETSSCOREDATA_ARRAY_TYPE_INFO);
    registry.register_type(ANGLETOMULTITARGETSSCOREDATA_TYPE_INFO);
    registry.register_type(ANGLETOMULTITARGETSSCOREDATA_ARRAY_TYPE_INFO);
    registry.register_type(MULTITARGETSCORER_TYPE_INFO);
    registry.register_type(MULTITARGETSCORER_ARRAY_TYPE_INFO);
    registry.register_type(MULTITARGETSCORINGMODE_TYPE_INFO);
    registry.register_type(MULTITARGETSCORINGMODE_ARRAY_TYPE_INFO);
    registry.register_type(TOWARDSPREFERREDWEAPONRANGESCOREDATA_TYPE_INFO);
    registry.register_type(TOWARDSPREFERREDWEAPONRANGESCOREDATA_ARRAY_TYPE_INFO);
    registry.register_type(PREFERREDWEAPONRANGESCOREDATA_TYPE_INFO);
    registry.register_type(PREFERREDWEAPONRANGESCOREDATA_ARRAY_TYPE_INFO);
    registry.register_type(PROJECTEDDISTANCESCOREDATA_TYPE_INFO);
    registry.register_type(PROJECTEDDISTANCESCOREDATA_ARRAY_TYPE_INFO);
    registry.register_type(COVERSCOREDIRECTION_TYPE_INFO);
    registry.register_type(COVERSCOREDIRECTION_ARRAY_TYPE_INFO);
    registry.register_type(DISTANCETOACTORSCOREDATA_TYPE_INFO);
    registry.register_type(DISTANCETOACTORSCOREDATA_ARRAY_TYPE_INFO);
    registry.register_type(ANGLEFROMPATHTRAJECTORYSCOREDATA_TYPE_INFO);
    registry.register_type(ANGLEFROMPATHTRAJECTORYSCOREDATA_ARRAY_TYPE_INFO);
    registry.register_type(ANGLEFROMREFERENCEDIRECTIONSCOREDATA_TYPE_INFO);
    registry.register_type(ANGLEFROMREFERENCEDIRECTIONSCOREDATA_ARRAY_TYPE_INFO);
    registry.register_type(ANGLETOACTORSCOREDATA2_TYPE_INFO);
    registry.register_type(ANGLETOACTORSCOREDATA2_ARRAY_TYPE_INFO);
    registry.register_type(SCORECURVEFORFILTER_TYPE_INFO);
    registry.register_type(SCORECURVEFORFILTER_ARRAY_TYPE_INFO);
    registry.register_type(ANGLETOACTORSCOREDATA_TYPE_INFO);
    registry.register_type(ANGLETOACTORSCOREDATA_ARRAY_TYPE_INFO);
    registry.register_type(COVERSCOREDATAWITHREFPOS_TYPE_INFO);
    registry.register_type(COVERSCOREDATAWITHREFPOS_ARRAY_TYPE_INFO);
    registry.register_type(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO);
    registry.register_type(COVERSCOREDATAWITHSCORECURVE_ARRAY_TYPE_INFO);
    registry.register_type(CURRENTCOVERBONUSSCOREDATA_TYPE_INFO);
    registry.register_type(CURRENTCOVERBONUSSCOREDATA_ARRAY_TYPE_INFO);
    registry.register_type(COVERFILTERSCOREDATA_TYPE_INFO);
    registry.register_type(COVERFILTERSCOREDATA_ARRAY_TYPE_INFO);
    registry.register_type(COVERSCOREDATA_TYPE_INFO);
    registry.register_type(COVERSCOREDATA_ARRAY_TYPE_INFO);
    registry.register_type(COVERSCOREPOSITION_TYPE_INFO);
    registry.register_type(COVERSCOREPOSITION_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSimpleDriverComponent {
}

pub const SERVERSIMPLEDRIVERCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSimpleDriverComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSIMPLEDRIVERCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSimpleDriverComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERSIMPLEDRIVERCOMPONENT_TYPE_INFO
    }
}


pub const SERVERSIMPLEDRIVERCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSimpleDriverComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerSimpleDriverComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WSServerDogFightingEntity {
}

pub const WSSERVERDOGFIGHTINGENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WSServerDogFightingEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTINGENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WSSERVERDOGFIGHTINGENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for WSServerDogFightingEntity {
    fn type_info() -> &'static TypeInfo {
        WSSERVERDOGFIGHTINGENTITY_TYPE_INFO
    }
}


pub const WSSERVERDOGFIGHTINGENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WSServerDogFightingEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("WSServerDogFightingEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerStrafeRunManeuverEntity {
}

pub const SERVERSTRAFERUNMANEUVERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStrafeRunManeuverEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSTRAFERUNMANEUVERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerStrafeRunManeuverEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERSTRAFERUNMANEUVERENTITY_TYPE_INFO
    }
}


pub const SERVERSTRAFERUNMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStrafeRunManeuverEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerStrafeRunManeuverEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerStallTurnManeuverEntity {
}

pub const SERVERSTALLTURNMANEUVERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStallTurnManeuverEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSTALLTURNMANEUVERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerStallTurnManeuverEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERSTALLTURNMANEUVERENTITY_TYPE_INFO
    }
}


pub const SERVERSTALLTURNMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStallTurnManeuverEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerStallTurnManeuverEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSplitSManeuverEntity {
}

pub const SERVERSPLITSMANEUVERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSplitSManeuverEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSPLITSMANEUVERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSplitSManeuverEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERSPLITSMANEUVERENTITY_TYPE_INFO
    }
}


pub const SERVERSPLITSMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSplitSManeuverEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerSplitSManeuverEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSpinDescentManeuverEntity {
}

pub const SERVERSPINDESCENTMANEUVERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSpinDescentManeuverEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSPINDESCENTMANEUVERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSpinDescentManeuverEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERSPINDESCENTMANEUVERENTITY_TYPE_INFO
    }
}


pub const SERVERSPINDESCENTMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSpinDescentManeuverEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerSpinDescentManeuverEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSideToSideManeuverEntity {
}

pub const SERVERSIDETOSIDEMANEUVERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSideToSideManeuverEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSIDETOSIDEMANEUVERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSideToSideManeuverEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERSIDETOSIDEMANEUVERENTITY_TYPE_INFO
    }
}


pub const SERVERSIDETOSIDEMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSideToSideManeuverEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerSideToSideManeuverEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSetWaypointsEntity {
}

pub const SERVERSETWAYPOINTSENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSetWaypointsEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSETWAYPOINTSENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSetWaypointsEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERSETWAYPOINTSENTITY_TYPE_INFO
    }
}


pub const SERVERSETWAYPOINTSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSetWaypointsEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerSetWaypointsEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerProtectBaseManeuverEntity {
}

pub const SERVERPROTECTBASEMANEUVERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerProtectBaseManeuverEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERPROTECTBASEMANEUVERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerProtectBaseManeuverEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERPROTECTBASEMANEUVERENTITY_TYPE_INFO
    }
}


pub const SERVERPROTECTBASEMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerProtectBaseManeuverEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerProtectBaseManeuverEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPilotEntityBase {
}

pub const SERVERPILOTENTITYBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPilotEntityBase",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERPILOTENTITYBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPilotEntityBase {
    fn type_info() -> &'static TypeInfo {
        SERVERPILOTENTITYBASE_TYPE_INFO
    }
}


pub const SERVERPILOTENTITYBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPilotEntityBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerPilotEntityBase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPilotEntity {
}

pub const SERVERPILOTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPilotEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERPILOTENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERPILOTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPilotEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERPILOTENTITY_TYPE_INFO
    }
}


pub const SERVERPILOTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPilotEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerPilotEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerManeuverSelectorEntity {
}

pub const SERVERMANEUVERSELECTORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerManeuverSelectorEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERMANEUVERSELECTORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerManeuverSelectorEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERMANEUVERSELECTORENTITY_TYPE_INFO
    }
}


pub const SERVERMANEUVERSELECTORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerManeuverSelectorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerManeuverSelectorEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerImmelmannManeuverEntity {
}

pub const SERVERIMMELMANNMANEUVERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerImmelmannManeuverEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERIMMELMANNMANEUVERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerImmelmannManeuverEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERIMMELMANNMANEUVERENTITY_TYPE_INFO
    }
}


pub const SERVERIMMELMANNMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerImmelmannManeuverEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerImmelmannManeuverEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerHeavyPlanePilotEntity {
}

pub const SERVERHEAVYPLANEPILOTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerHeavyPlanePilotEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERPILOTENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERHEAVYPLANEPILOTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerHeavyPlanePilotEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERHEAVYPLANEPILOTENTITY_TYPE_INFO
    }
}


pub const SERVERHEAVYPLANEPILOTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerHeavyPlanePilotEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerHeavyPlanePilotEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerFollowWaypointsManeuverEntity {
}

pub const SERVERFOLLOWWAYPOINTSMANEUVERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerFollowWaypointsManeuverEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERFOLLOWWAYPOINTSMANEUVERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerFollowWaypointsManeuverEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERFOLLOWWAYPOINTSMANEUVERENTITY_TYPE_INFO
    }
}


pub const SERVERFOLLOWWAYPOINTSMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerFollowWaypointsManeuverEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerFollowWaypointsManeuverEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerFlyToManeuverEntity {
}

pub const SERVERFLYTOMANEUVERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerFlyToManeuverEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERFLYTOMANEUVERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerFlyToManeuverEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERFLYTOMANEUVERENTITY_TYPE_INFO
    }
}


pub const SERVERFLYTOMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerFlyToManeuverEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerFlyToManeuverEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerEnforceAltitudeManeuverEntity {
}

pub const SERVERENFORCEALTITUDEMANEUVERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEnforceAltitudeManeuverEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERENFORCEALTITUDEMANEUVERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerEnforceAltitudeManeuverEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERENFORCEALTITUDEMANEUVERENTITY_TYPE_INFO
    }
}


pub const SERVERENFORCEALTITUDEMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEnforceAltitudeManeuverEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerEnforceAltitudeManeuverEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerDogFightManeuverEntityBase {
}

pub const SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDogFightManeuverEntityBase",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerDogFightManeuverEntityBase {
    fn type_info() -> &'static TypeInfo {
        SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO
    }
}


pub const SERVERDOGFIGHTMANEUVERENTITYBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDogFightManeuverEntityBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerDogFightManeuverEntityBase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerDogFightingEntity {
}

pub const SERVERDOGFIGHTINGENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDogFightingEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERDOGFIGHTINGENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerDogFightingEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERDOGFIGHTINGENTITY_TYPE_INFO
    }
}


pub const SERVERDOGFIGHTINGENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDogFightingEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerDogFightingEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerDefensiveManeuverSelectorEntity {
}

pub const SERVERDEFENSIVEMANEUVERSELECTORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDefensiveManeuverSelectorEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERMANEUVERSELECTORENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERDEFENSIVEMANEUVERSELECTORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerDefensiveManeuverSelectorEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERDEFENSIVEMANEUVERSELECTORENTITY_TYPE_INFO
    }
}


pub const SERVERDEFENSIVEMANEUVERSELECTORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDefensiveManeuverSelectorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerDefensiveManeuverSelectorEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCreateDistanceManeuverEntity {
}

pub const SERVERCREATEDISTANCEMANEUVERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreateDistanceManeuverEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERCREATEDISTANCEMANEUVERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCreateDistanceManeuverEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERCREATEDISTANCEMANEUVERENTITY_TYPE_INFO
    }
}


pub const SERVERCREATEDISTANCEMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreateDistanceManeuverEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerCreateDistanceManeuverEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCollisionAvoidanceManeuverEntity {
}

pub const SERVERCOLLISIONAVOIDANCEMANEUVERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCollisionAvoidanceManeuverEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERCOLLISIONAVOIDANCEMANEUVERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCollisionAvoidanceManeuverEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERCOLLISIONAVOIDANCEMANEUVERENTITY_TYPE_INFO
    }
}


pub const SERVERCOLLISIONAVOIDANCEMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCollisionAvoidanceManeuverEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerCollisionAvoidanceManeuverEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerBasicDefensiveManeuverEntity {
}

pub const SERVERBASICDEFENSIVEMANEUVERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBasicDefensiveManeuverEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERBASICDEFENSIVEMANEUVERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerBasicDefensiveManeuverEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERBASICDEFENSIVEMANEUVERENTITY_TYPE_INFO
    }
}


pub const SERVERBASICDEFENSIVEMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBasicDefensiveManeuverEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerBasicDefensiveManeuverEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerBasicAttackManeuverEntity {
}

pub const SERVERBASICATTACKMANEUVERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBasicAttackManeuverEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERBASICATTACKMANEUVERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerBasicAttackManeuverEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERBASICATTACKMANEUVERENTITY_TYPE_INFO
    }
}


pub const SERVERBASICATTACKMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBasicAttackManeuverEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerBasicAttackManeuverEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerBarrelRollManeuverEntity {
}

pub const SERVERBARRELROLLMANEUVERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBarrelRollManeuverEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERBARRELROLLMANEUVERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerBarrelRollManeuverEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERBARRELROLLMANEUVERENTITY_TYPE_INFO
    }
}


pub const SERVERBARRELROLLMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBarrelRollManeuverEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerBarrelRollManeuverEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAirTargetSelectorEntity {
}

pub const SERVERAIRTARGETSELECTORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAirTargetSelectorEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIRTARGETSELECTORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAirTargetSelectorEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAIRTARGETSELECTORENTITY_TYPE_INFO
    }
}


pub const SERVERAIRTARGETSELECTORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAirTargetSelectorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAirTargetSelectorEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAirCollisionAvoidanceEntity {
}

pub const SERVERAIRCOLLISIONAVOIDANCEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAirCollisionAvoidanceEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIRCOLLISIONAVOIDANCEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAirCollisionAvoidanceEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAIRCOLLISIONAVOIDANCEENTITY_TYPE_INFO
    }
}


pub const SERVERAIRCOLLISIONAVOIDANCEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAirCollisionAvoidanceEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAirCollisionAvoidanceEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BFServerDogFightingEntity {
}

pub const BFSERVERDOGFIGHTINGENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BFServerDogFightingEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTINGENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(BFSERVERDOGFIGHTINGENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BFServerDogFightingEntity {
    fn type_info() -> &'static TypeInfo {
        BFSERVERDOGFIGHTINGENTITY_TYPE_INFO
    }
}


pub const BFSERVERDOGFIGHTINGENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BFServerDogFightingEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("BFServerDogFightingEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerWaypointsWalkerEntity {
}

pub const SERVERWAYPOINTSWALKERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWaypointsWalkerEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERWAYPOINTSWALKERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerWaypointsWalkerEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERWAYPOINTSWALKERENTITY_TYPE_INFO
    }
}


pub const SERVERWAYPOINTSWALKERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWaypointsWalkerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerWaypointsWalkerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAIProximityReactionsComponent {
}

pub const SERVERAIPROXIMITYREACTIONSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIProximityReactionsComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIPROXIMITYREACTIONSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIProximityReactionsComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERAIPROXIMITYREACTIONSCOMPONENT_TYPE_INFO
    }
}


pub const SERVERAIPROXIMITYREACTIONSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIProximityReactionsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIProximityReactionsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAILocoComponent {
}

pub const SERVERAILOCOCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAILocoComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAILOCOCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAILocoComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERAILOCOCOMPONENT_TYPE_INFO
    }
}


pub const SERVERAILOCOCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAILocoComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAILocoComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AIBlockerEntity {
}

pub const AIBLOCKERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIBlockerEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(AIBLOCKERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AIBlockerEntity {
    fn type_info() -> &'static TypeInfo {
        AIBLOCKERENTITY_TYPE_INFO
    }
}


pub const AIBLOCKERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIBlockerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AIBlockerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientAIProximityReactionsComponent {
}

pub const CLIENTAIPROXIMITYREACTIONSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAIProximityReactionsComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTAIPROXIMITYREACTIONSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAIProximityReactionsComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTAIPROXIMITYREACTIONSCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTAIPROXIMITYREACTIONSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAIProximityReactionsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ClientAIProximityReactionsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientAILocoComponent {
}

pub const CLIENTAILOCOCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAILocoComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTAILOCOCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAILocoComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTAILOCOCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTAILOCOCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAILocoComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ClientAILocoComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SpatialAnalyzer {
}

pub const SPATIALANALYZER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpatialAnalyzer",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SPATIALANALYZER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SpatialAnalyzer {
    fn type_info() -> &'static TypeInfo {
        SPATIALANALYZER_TYPE_INFO
    }
}


pub const SPATIALANALYZER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpatialAnalyzer-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("SpatialAnalyzer-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPlayerVehicleProximityEntity {
}

pub const SERVERPLAYERVEHICLEPROXIMITYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerVehicleProximityEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERPLAYERVEHICLEPROXIMITYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPlayerVehicleProximityEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERPLAYERVEHICLEPROXIMITYENTITY_TYPE_INFO
    }
}


pub const SERVERPLAYERVEHICLEPROXIMITYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerVehicleProximityEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerPlayerVehicleProximityEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerInvestigateSettingsOverride {
}

pub const SERVERINVESTIGATESETTINGSOVERRIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerInvestigateSettingsOverride",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERINVESTIGATESETTINGSOVERRIDE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerInvestigateSettingsOverride {
    fn type_info() -> &'static TypeInfo {
        SERVERINVESTIGATESETTINGSOVERRIDE_TYPE_INFO
    }
}


pub const SERVERINVESTIGATESETTINGSOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerInvestigateSettingsOverride-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerInvestigateSettingsOverride-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerDamageModifierEntity {
}

pub const SERVERDAMAGEMODIFIERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDamageModifierEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERDAMAGEMODIFIERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerDamageModifierEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERDAMAGEMODIFIERENTITY_TYPE_INFO
    }
}


pub const SERVERDAMAGEMODIFIERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDamageModifierEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerDamageModifierEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAttackPointEntity {
}

pub const SERVERATTACKPOINTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAttackPointEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERATTACKPOINTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAttackPointEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERATTACKPOINTENTITY_TYPE_INFO
    }
}


pub const SERVERATTACKPOINTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAttackPointEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAttackPointEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAIVehicleCombatEntity {
}

pub const SERVERAIVEHICLECOMBATENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIVehicleCombatEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIVEHICLECOMBATENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIVehicleCombatEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAIVEHICLECOMBATENTITY_TYPE_INFO
    }
}


pub const SERVERAIVEHICLECOMBATENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIVehicleCombatEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIVehicleCombatEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAITemplateFilterEntity {
}

pub const SERVERAITEMPLATEFILTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAITemplateFilterEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAITEMPLATEFILTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAITemplateFilterEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAITEMPLATEFILTERENTITY_TYPE_INFO
    }
}


pub const SERVERAITEMPLATEFILTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAITemplateFilterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAITemplateFilterEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAITeleportEntity {
}

pub const SERVERAITELEPORTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAITeleportEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAITELEPORTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAITeleportEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAITELEPORTENTITY_TYPE_INFO
    }
}


pub const SERVERAITELEPORTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAITeleportEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAITeleportEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAISystemEntity {
}

pub const SERVERAISYSTEMENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISystemEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAISYSTEMENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAISystemEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAISYSTEMENTITY_TYPE_INFO
    }
}


pub const SERVERAISYSTEMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISystemEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAISystemEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAIStateEntity {
}

pub const SERVERAISTATEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIStateEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAISTATEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIStateEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAISTATEENTITY_TYPE_INFO
    }
}


pub const SERVERAISTATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIStateEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIStateEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAISoundEntity {
}

pub const SERVERAISOUNDENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISoundEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAISOUNDENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAISoundEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAISOUNDENTITY_TYPE_INFO
    }
}


pub const SERVERAISOUNDENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISoundEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAISoundEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAICancelOrder {
}

pub const SERVERAICANCELORDER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAICancelOrder",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIORDERENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAICANCELORDER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAICancelOrder {
    fn type_info() -> &'static TypeInfo {
        SERVERAICANCELORDER_TYPE_INFO
    }
}


pub const SERVERAICANCELORDER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAICancelOrder-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAICancelOrder-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAIGotoPlaceOrderEntityData {
}

pub const SERVERAIGOTOPLACEORDERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIGotoPlaceOrderEntityData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIORDERENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIGOTOPLACEORDERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIGotoPlaceOrderEntityData {
    fn type_info() -> &'static TypeInfo {
        SERVERAIGOTOPLACEORDERENTITYDATA_TYPE_INFO
    }
}


pub const SERVERAIGOTOPLACEORDERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIGotoPlaceOrderEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIGotoPlaceOrderEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAIFollowWaypointsOrder {
}

pub const SERVERAIFOLLOWWAYPOINTSORDER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIFollowWaypointsOrder",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIORDERENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIFOLLOWWAYPOINTSORDER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIFollowWaypointsOrder {
    fn type_info() -> &'static TypeInfo {
        SERVERAIFOLLOWWAYPOINTSORDER_TYPE_INFO
    }
}


pub const SERVERAIFOLLOWWAYPOINTSORDER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIFollowWaypointsOrder-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIFollowWaypointsOrder-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAIOrderEntityBase {
}

pub const SERVERAIORDERENTITYBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIOrderEntityBase",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIORDERENTITYBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIOrderEntityBase {
    fn type_info() -> &'static TypeInfo {
        SERVERAIORDERENTITYBASE_TYPE_INFO
    }
}


pub const SERVERAIORDERENTITYBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIOrderEntityBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIOrderEntityBase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAISelfDestructEntity {
}

pub const SERVERAISELFDESTRUCTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISelfDestructEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAISELFDESTRUCTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAISelfDestructEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAISELFDESTRUCTENTITY_TYPE_INFO
    }
}


pub const SERVERAISELFDESTRUCTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISelfDestructEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAISelfDestructEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAICoverZonesOverrideEntity {
}

pub const SERVERAICOVERZONESOVERRIDEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAICoverZonesOverrideEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAICOVERZONESOVERRIDEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAICoverZonesOverrideEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAICOVERZONESOVERRIDEENTITY_TYPE_INFO
    }
}


pub const SERVERAICOVERZONESOVERRIDEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAICoverZonesOverrideEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAICoverZonesOverrideEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAIAwarenessEntity {
}

pub const SERVERAIAWARENESSENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIAwarenessEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIAWARENESSENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIAwarenessEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAIAWARENESSENTITY_TYPE_INFO
    }
}


pub const SERVERAIAWARENESSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIAwarenessEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIAwarenessEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAIWeaponSlotOverrideEntity {
}

pub const SERVERAIWEAPONSLOTOVERRIDEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIWeaponSlotOverrideEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIWEAPONSLOTOVERRIDEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIWeaponSlotOverrideEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAIWEAPONSLOTOVERRIDEENTITY_TYPE_INFO
    }
}


pub const SERVERAIWEAPONSLOTOVERRIDEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIWeaponSlotOverrideEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIWeaponSlotOverrideEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAITargetCoordinatorFilterEntity {
}

pub const SERVERAITARGETCOORDINATORFILTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAITargetCoordinatorFilterEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAITARGETCOORDINATORFILTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAITargetCoordinatorFilterEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAITARGETCOORDINATORFILTERENTITY_TYPE_INFO
    }
}


pub const SERVERAITARGETCOORDINATORFILTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAITargetCoordinatorFilterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAITargetCoordinatorFilterEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAITargetCoordinatorEntity {
}

pub const SERVERAITARGETCOORDINATORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAITargetCoordinatorEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAITARGETCOORDINATORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAITargetCoordinatorEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAITARGETCOORDINATORENTITY_TYPE_INFO
    }
}


pub const SERVERAITARGETCOORDINATORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAITargetCoordinatorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAITargetCoordinatorEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCloakingModifierEntity {
}

pub const SERVERCLOAKINGMODIFIERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCloakingModifierEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERCLOAKINGMODIFIERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCloakingModifierEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERCLOAKINGMODIFIERENTITY_TYPE_INFO
    }
}


pub const SERVERCLOAKINGMODIFIERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCloakingModifierEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerCloakingModifierEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSensingAreaModifierEntity {
}

pub const SERVERSENSINGAREAMODIFIERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSensingAreaModifierEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSENSINGAREAMODIFIERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSensingAreaModifierEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERSENSINGAREAMODIFIERENTITY_TYPE_INFO
    }
}


pub const SERVERSENSINGAREAMODIFIERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSensingAreaModifierEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerSensingAreaModifierEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAIStealthEntity {
}

pub const SERVERAISTEALTHENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIStealthEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAISTEALTHENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIStealthEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAISTEALTHENTITY_TYPE_INFO
    }
}


pub const SERVERAISTEALTHENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIStealthEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIStealthEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAIBuddyFollowEntity {
}

pub const SERVERAIBUDDYFOLLOWENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIBuddyFollowEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIBUDDYFOLLOWENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIBuddyFollowEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAIBUDDYFOLLOWENTITY_TYPE_INFO
    }
}


pub const SERVERAIBUDDYFOLLOWENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIBuddyFollowEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIBuddyFollowEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAIFollowObjectEntity {
}

pub const SERVERAIFOLLOWOBJECTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIFollowObjectEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIFOLLOWOBJECTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIFollowObjectEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAIFOLLOWOBJECTENTITY_TYPE_INFO
    }
}


pub const SERVERAIFOLLOWOBJECTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIFollowObjectEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIFollowObjectEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAIPreferredAreaEntity {
}

pub const SERVERAIPREFERREDAREAENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIPreferredAreaEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERWITHSHAPEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIPREFERREDAREAENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIPreferredAreaEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAIPREFERREDAREAENTITY_TYPE_INFO
    }
}


pub const SERVERAIPREFERREDAREAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIPreferredAreaEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIPreferredAreaEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAISoundAreaEntity {
}

pub const SERVERAISOUNDAREAENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISoundAreaEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERWITHSHAPEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAISOUNDAREAENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAISoundAreaEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAISOUNDAREAENTITY_TYPE_INFO
    }
}


pub const SERVERAISOUNDAREAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISoundAreaEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAISoundAreaEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAICombatGroupEntity {
}

pub const SERVERAICOMBATGROUPENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAICombatGroupEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAICOMBATGROUPENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAICombatGroupEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAICOMBATGROUPENTITY_TYPE_INFO
    }
}


pub const SERVERAICOMBATGROUPENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAICombatGroupEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAICombatGroupEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAICoverQueryEntity {
}

pub const SERVERAICOVERQUERYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAICoverQueryEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAICOVERQUERYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAICoverQueryEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAICOVERQUERYENTITY_TYPE_INFO
    }
}


pub const SERVERAICOVERQUERYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAICoverQueryEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAICoverQueryEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAITacticEntity {
}

pub const SERVERAITACTICENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAITacticEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAITACTICENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAITacticEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAITACTICENTITY_TYPE_INFO
    }
}


pub const SERVERAITACTICENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAITacticEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAITacticEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAIShootAtTargetsEntity {
}

pub const SERVERAISHOOTATTARGETSENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIShootAtTargetsEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAISHOOTATTARGETSENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIShootAtTargetsEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAISHOOTATTARGETSENTITY_TYPE_INFO
    }
}


pub const SERVERAISHOOTATTARGETSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIShootAtTargetsEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIShootAtTargetsEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAIUseWaypointsEntity {
}

pub const SERVERAIUSEWAYPOINTSENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIUseWaypointsEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIUSEWAYPOINTSENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIUseWaypointsEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAIUSEWAYPOINTSENTITY_TYPE_INFO
    }
}


pub const SERVERAIUSEWAYPOINTSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIUseWaypointsEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIUseWaypointsEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAIUseCoverEntity {
}

pub const SERVERAIUSECOVERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIUseCoverEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIUSECOVERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIUseCoverEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAIUSECOVERENTITY_TYPE_INFO
    }
}


pub const SERVERAIUSECOVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIUseCoverEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIUseCoverEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAIWeaponOverrideEntity {
}

pub const SERVERAIWEAPONOVERRIDEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIWeaponOverrideEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIWEAPONOVERRIDEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIWeaponOverrideEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAIWEAPONOVERRIDEENTITY_TYPE_INFO
    }
}


pub const SERVERAIWEAPONOVERRIDEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIWeaponOverrideEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIWeaponOverrideEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAIVehicleBehaviorEntity {
}

pub const SERVERAIVEHICLEBEHAVIORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIVehicleBehaviorEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIVEHICLEBEHAVIORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIVehicleBehaviorEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAIVEHICLEBEHAVIORENTITY_TYPE_INFO
    }
}


pub const SERVERAIVEHICLEBEHAVIORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIVehicleBehaviorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIVehicleBehaviorEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAIHearingParameterEntity {
}

pub const SERVERAIHEARINGPARAMETERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIHearingParameterEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIHEARINGPARAMETERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIHearingParameterEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAIHEARINGPARAMETERENTITY_TYPE_INFO
    }
}


pub const SERVERAIHEARINGPARAMETERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIHearingParameterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIHearingParameterEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAISensingParameterEntity {
}

pub const SERVERAISENSINGPARAMETERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISensingParameterEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAISENSINGPARAMETERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAISensingParameterEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAISENSINGPARAMETERENTITY_TYPE_INFO
    }
}


pub const SERVERAISENSINGPARAMETERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISensingParameterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAISensingParameterEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAIIdleBehaviorEntity {
}

pub const SERVERAIIDLEBEHAVIORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIIdleBehaviorEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIIDLEBEHAVIORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIIdleBehaviorEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAIIDLEBEHAVIORENTITY_TYPE_INFO
    }
}


pub const SERVERAIIDLEBEHAVIORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIIdleBehaviorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIIdleBehaviorEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAITargetingEntity {
}

pub const SERVERAITARGETINGENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAITargetingEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAITARGETINGENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAITargetingEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAITARGETINGENTITY_TYPE_INFO
    }
}


pub const SERVERAITARGETINGENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAITargetingEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAITargetingEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAICombatBehaviorEntity {
}

pub const SERVERAICOMBATBEHAVIORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAICombatBehaviorEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAICOMBATBEHAVIORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAICombatBehaviorEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAICOMBATBEHAVIORENTITY_TYPE_INFO
    }
}


pub const SERVERAICOMBATBEHAVIORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAICombatBehaviorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAICombatBehaviorEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAIFollowAreaEntity {
}

pub const SERVERAIFOLLOWAREAENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIFollowAreaEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERWITHSHAPEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIFOLLOWAREAENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIFollowAreaEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAIFOLLOWAREAENTITY_TYPE_INFO
    }
}


pub const SERVERAIFOLLOWAREAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIFollowAreaEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIFollowAreaEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAIForbiddenAreaEntity {
}

pub const SERVERAIFORBIDDENAREAENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIForbiddenAreaEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERWITHSHAPEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIFORBIDDENAREAENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIForbiddenAreaEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAIFORBIDDENAREAENTITY_TYPE_INFO
    }
}


pub const SERVERAIFORBIDDENAREAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIForbiddenAreaEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIForbiddenAreaEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAIFriendlyAreaEntity {
}

pub const SERVERAIFRIENDLYAREAENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIFriendlyAreaEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERWITHSHAPEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIFRIENDLYAREAENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIFriendlyAreaEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAIFRIENDLYAREAENTITY_TYPE_INFO
    }
}


pub const SERVERAIFRIENDLYAREAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIFriendlyAreaEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIFriendlyAreaEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAIFlankingCorridorEntity {
}

pub const SERVERAIFLANKINGCORRIDORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIFlankingCorridorEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERWITHSHAPEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIFLANKINGCORRIDORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIFlankingCorridorEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAIFLANKINGCORRIDORENTITY_TYPE_INFO
    }
}


pub const SERVERAIFLANKINGCORRIDORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIFlankingCorridorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIFlankingCorridorEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAISearchAreaEntity {
}

pub const SERVERAISEARCHAREAENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISearchAreaEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERWITHSHAPEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAISEARCHAREAENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAISearchAreaEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAISEARCHAREAENTITY_TYPE_INFO
    }
}


pub const SERVERAISEARCHAREAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISearchAreaEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAISearchAreaEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAIDefendAreaEntity {
}

pub const SERVERAIDEFENDAREAENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIDefendAreaEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERWITHSHAPEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIDEFENDAREAENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIDefendAreaEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAIDEFENDAREAENTITY_TYPE_INFO
    }
}


pub const SERVERAIDEFENDAREAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIDefendAreaEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIDefendAreaEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAIParameterWithShapeEntity {
}

pub const SERVERAIPARAMETERWITHSHAPEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIParameterWithShapeEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIPARAMETERWITHSHAPEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIParameterWithShapeEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAIPARAMETERWITHSHAPEENTITY_TYPE_INFO
    }
}


pub const SERVERAIPARAMETERWITHSHAPEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIParameterWithShapeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIParameterWithShapeEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAIParameterEntity {
}

pub const SERVERAIPARAMETERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIParameterEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIPARAMETERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIParameterEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAIPARAMETERENTITY_TYPE_INFO
    }
}


pub const SERVERAIPARAMETERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIParameterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIParameterEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAIObstacleControllerEntity {
}

pub const SERVERAIOBSTACLECONTROLLERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIObstacleControllerEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIOBSTACLECONTROLLERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIObstacleControllerEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAIOBSTACLECONTROLLERENTITY_TYPE_INFO
    }
}


pub const SERVERAIOBSTACLECONTROLLERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIObstacleControllerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIObstacleControllerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAIKillCounterEntity {
}

pub const SERVERAIKILLCOUNTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIKillCounterEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIKILLCOUNTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIKillCounterEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAIKILLCOUNTERENTITY_TYPE_INFO
    }
}


pub const SERVERAIKILLCOUNTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIKillCounterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIKillCounterEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAIFirePatternEntity {
}

pub const SERVERAIFIREPATTERNENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIFirePatternEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIFIREPATTERNENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIFirePatternEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAIFIREPATTERNENTITY_TYPE_INFO
    }
}


pub const SERVERAIFIREPATTERNENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIFirePatternEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIFirePatternEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAIEncounterManagerEntity {
}

pub const SERVERAIENCOUNTERMANAGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIEncounterManagerEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIENCOUNTERMANAGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIEncounterManagerEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAIENCOUNTERMANAGERENTITY_TYPE_INFO
    }
}


pub const SERVERAIENCOUNTERMANAGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIEncounterManagerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIEncounterManagerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAIDebugProxy {
}

pub const SERVERAIDEBUGPROXY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIDebugProxy",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIDEBUGPROXY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIDebugProxy {
    fn type_info() -> &'static TypeInfo {
        SERVERAIDEBUGPROXY_TYPE_INFO
    }
}


pub const SERVERAIDEBUGPROXY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIDebugProxy-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIDebugProxy-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerActionEntity {
}

pub const SERVERACTIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerActionEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERACTIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerActionEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERACTIONENTITY_TYPE_INFO
    }
}


pub const SERVERACTIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerActionEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerActionEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AIConcealmentVolumeEntity {
}

pub const AICONCEALMENTVOLUMEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIConcealmentVolumeEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(AICONCEALMENTVOLUMEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AIConcealmentVolumeEntity {
    fn type_info() -> &'static TypeInfo {
        AICONCEALMENTVOLUMEENTITY_TYPE_INFO
    }
}


pub const AICONCEALMENTVOLUMEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIConcealmentVolumeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AIConcealmentVolumeEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientAITemplateFilterEntity {
}

pub const CLIENTAITEMPLATEFILTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAITemplateFilterEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTAITEMPLATEFILTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAITemplateFilterEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTAITEMPLATEFILTERENTITY_TYPE_INFO
    }
}


pub const CLIENTAITEMPLATEFILTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAITemplateFilterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ClientAITemplateFilterEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientAIStateEntity {
}

pub const CLIENTAISTATEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAIStateEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTAISTATEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAIStateEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTAISTATEENTITY_TYPE_INFO
    }
}


pub const CLIENTAISTATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAIStateEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ClientAIStateEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientAIPhysicsDrivenAnimationEntity {
}

pub const CLIENTAIPHYSICSDRIVENANIMATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAIPhysicsDrivenAnimationEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTAIPHYSICSDRIVENANIMATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAIPhysicsDrivenAnimationEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTAIPHYSICSDRIVENANIMATIONENTITY_TYPE_INFO
    }
}


pub const CLIENTAIPHYSICSDRIVENANIMATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAIPhysicsDrivenAnimationEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ClientAIPhysicsDrivenAnimationEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientAICollisionAvoidanceSetupEntity {
}

pub const CLIENTAICOLLISIONAVOIDANCESETUPENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAICollisionAvoidanceSetupEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTAICOLLISIONAVOIDANCESETUPENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAICollisionAvoidanceSetupEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTAICOLLISIONAVOIDANCESETUPENTITY_TYPE_INFO
    }
}


pub const CLIENTAICOLLISIONAVOIDANCESETUPENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAICollisionAvoidanceSetupEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ClientAICollisionAvoidanceSetupEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientActionEntity {
}

pub const CLIENTACTIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientActionEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTACTIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientActionEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTACTIONENTITY_TYPE_INFO
    }
}


pub const CLIENTACTIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientActionEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ClientActionEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AITemplateConditionEvaluator {
}

pub const AITEMPLATECONDITIONEVALUATOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AITemplateConditionEvaluator",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IACTIONCONDITIONEVALUATOR_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(AITEMPLATECONDITIONEVALUATOR_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AITemplateConditionEvaluator {
    fn type_info() -> &'static TypeInfo {
        AITEMPLATECONDITIONEVALUATOR_TYPE_INFO
    }
}


pub const AITEMPLATECONDITIONEVALUATOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AITemplateConditionEvaluator-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AITemplateConditionEvaluator-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct InsideVolumesConditionEvaluator {
}

pub const INSIDEVOLUMESCONDITIONEVALUATOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InsideVolumesConditionEvaluator",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IACTIONCONDITIONEVALUATOR_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(INSIDEVOLUMESCONDITIONEVALUATOR_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for InsideVolumesConditionEvaluator {
    fn type_info() -> &'static TypeInfo {
        INSIDEVOLUMESCONDITIONEVALUATOR_TYPE_INFO
    }
}


pub const INSIDEVOLUMESCONDITIONEVALUATOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InsideVolumesConditionEvaluator-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("InsideVolumesConditionEvaluator-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProbabilityConditionEvaluator {
}

pub const PROBABILITYCONDITIONEVALUATOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProbabilityConditionEvaluator",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IACTIONCONDITIONEVALUATOR_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PROBABILITYCONDITIONEVALUATOR_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ProbabilityConditionEvaluator {
    fn type_info() -> &'static TypeInfo {
        PROBABILITYCONDITIONEVALUATOR_TYPE_INFO
    }
}


pub const PROBABILITYCONDITIONEVALUATOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProbabilityConditionEvaluator-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ProbabilityConditionEvaluator-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FacingConditionEvaluator {
}

pub const FACINGCONDITIONEVALUATOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FacingConditionEvaluator",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IACTIONCONDITIONEVALUATOR_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FACINGCONDITIONEVALUATOR_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FacingConditionEvaluator {
    fn type_info() -> &'static TypeInfo {
        FACINGCONDITIONEVALUATOR_TYPE_INFO
    }
}


pub const FACINGCONDITIONEVALUATOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FacingConditionEvaluator-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("FacingConditionEvaluator-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AIOrderCoordinatorConditionEvaluator {
}

pub const AIORDERCOORDINATORCONDITIONEVALUATOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIOrderCoordinatorConditionEvaluator",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IACTIONCONDITIONEVALUATOR_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(AIORDERCOORDINATORCONDITIONEVALUATOR_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AIOrderCoordinatorConditionEvaluator {
    fn type_info() -> &'static TypeInfo {
        AIORDERCOORDINATORCONDITIONEVALUATOR_TYPE_INFO
    }
}


pub const AIORDERCOORDINATORCONDITIONEVALUATOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIOrderCoordinatorConditionEvaluator-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AIOrderCoordinatorConditionEvaluator-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AIStateConditionEvaluator {
}

pub const AISTATECONDITIONEVALUATOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIStateConditionEvaluator",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IACTIONCONDITIONEVALUATOR_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(AISTATECONDITIONEVALUATOR_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AIStateConditionEvaluator {
    fn type_info() -> &'static TypeInfo {
        AISTATECONDITIONEVALUATOR_TYPE_INFO
    }
}


pub const AISTATECONDITIONEVALUATOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIStateConditionEvaluator-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AIStateConditionEvaluator-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RangeConditionEvaluator {
}

pub const RANGECONDITIONEVALUATOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RangeConditionEvaluator",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IACTIONCONDITIONEVALUATOR_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(RANGECONDITIONEVALUATOR_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RangeConditionEvaluator {
    fn type_info() -> &'static TypeInfo {
        RANGECONDITIONEVALUATOR_TYPE_INFO
    }
}


pub const RANGECONDITIONEVALUATOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RangeConditionEvaluator-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("RangeConditionEvaluator-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IActionConditionEvaluator {
}

pub const IACTIONCONDITIONEVALUATOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IActionConditionEvaluator",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(IACTIONCONDITIONEVALUATOR_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IActionConditionEvaluator {
    fn type_info() -> &'static TypeInfo {
        IACTIONCONDITIONEVALUATOR_TYPE_INFO
    }
}


pub const IACTIONCONDITIONEVALUATOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IActionConditionEvaluator-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("IActionConditionEvaluator-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPointOfInterestComponent {
}

pub const SERVERPOINTOFINTERESTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPointOfInterestComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERPOINTOFINTERESTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPointOfInterestComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERPOINTOFINTERESTCOMPONENT_TYPE_INFO
    }
}


pub const SERVERPOINTOFINTERESTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPointOfInterestComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerPointOfInterestComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCoverEntity {
}

pub const SERVERCOVERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCoverEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERCOVERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCoverEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERCOVERENTITY_TYPE_INFO
    }
}


pub const SERVERCOVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCoverEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerCoverEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCoverGroupEntity {
}

pub const SERVERCOVERGROUPENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCoverGroupEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERCOVERGROUPENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCoverGroupEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERCOVERGROUPENTITY_TYPE_INFO
    }
}


pub const SERVERCOVERGROUPENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCoverGroupEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerCoverGroupEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAIVehicleAimingComponent {
}

pub const SERVERAIVEHICLEAIMINGCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIVehicleAimingComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIVEHICLEAIMINGCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIVehicleAimingComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERAIVEHICLEAIMINGCOMPONENT_TYPE_INFO
    }
}


pub const SERVERAIVEHICLEAIMINGCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIVehicleAimingComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIVehicleAimingComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAITargetComponent {
}

pub const SERVERAITARGETCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAITargetComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAITARGETCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAITargetComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERAITARGETCOMPONENT_TYPE_INFO
    }
}


pub const SERVERAITARGETCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAITargetComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAITargetComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAISuppressWeaponFiringComponent {
}

pub const SERVERAISUPPRESSWEAPONFIRINGCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISuppressWeaponFiringComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAISUPPRESSWEAPONFIRINGCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAISuppressWeaponFiringComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERAISUPPRESSWEAPONFIRINGCOMPONENT_TYPE_INFO
    }
}


pub const SERVERAISUPPRESSWEAPONFIRINGCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISuppressWeaponFiringComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAISuppressWeaponFiringComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAISpottingComponent {
}

pub const SERVERAISPOTTINGCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISpottingComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAISPOTTINGCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAISpottingComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERAISPOTTINGCOMPONENT_TYPE_INFO
    }
}


pub const SERVERAISPOTTINGCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISpottingComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAISpottingComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAISmokeVolumeComponent {
}

pub const SERVERAISMOKEVOLUMECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISmokeVolumeComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAISMOKEVOLUMECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAISmokeVolumeComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERAISMOKEVOLUMECOMPONENT_TYPE_INFO
    }
}


pub const SERVERAISMOKEVOLUMECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISmokeVolumeComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAISmokeVolumeComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAIProjectileComponent {
}

pub const SERVERAIPROJECTILECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIProjectileComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIPROJECTILECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIProjectileComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERAIPROJECTILECOMPONENT_TYPE_INFO
    }
}


pub const SERVERAIPROJECTILECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIProjectileComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIProjectileComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAIOrderCoordinatorComponent {
}

pub const SERVERAIORDERCOORDINATORCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIOrderCoordinatorComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIORDERCOORDINATORCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIOrderCoordinatorComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERAIORDERCOORDINATORCOMPONENT_TYPE_INFO
    }
}


pub const SERVERAIORDERCOORDINATORCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIOrderCoordinatorComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIOrderCoordinatorComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAIMeleeComponent {
}

pub const SERVERAIMELEECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIMeleeComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIMELEECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIMeleeComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERAIMELEECOMPONENT_TYPE_INFO
    }
}


pub const SERVERAIMELEECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIMeleeComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIMeleeComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAIEntryComponent {
}

pub const SERVERAIENTRYCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIEntryComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIENTRYCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIEntryComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERAIENTRYCOMPONENT_TYPE_INFO
    }
}


pub const SERVERAIENTRYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIEntryComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIEntryComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAICustomInputComponent {
}

pub const SERVERAICUSTOMINPUTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAICustomInputComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAICUSTOMINPUTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAICustomInputComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERAICUSTOMINPUTCOMPONENT_TYPE_INFO
    }
}


pub const SERVERAICUSTOMINPUTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAICustomInputComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAICustomInputComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAIBucketSystemComponent {
}

pub const SERVERAIBUCKETSYSTEMCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIBucketSystemComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIBUCKETSYSTEMCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIBucketSystemComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERAIBUCKETSYSTEMCOMPONENT_TYPE_INFO
    }
}


pub const SERVERAIBUCKETSYSTEMCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIBucketSystemComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIBucketSystemComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAIAnchorToPointComponent {
}

pub const SERVERAIANCHORTOPOINTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIAnchorToPointComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIANCHORTOPOINTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIAnchorToPointComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERAIANCHORTOPOINTCOMPONENT_TYPE_INFO
    }
}


pub const SERVERAIANCHORTOPOINTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIAnchorToPointComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIAnchorToPointComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientAISpottingComponent {
}

pub const CLIENTAISPOTTINGCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAISpottingComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTAISPOTTINGCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAISpottingComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTAISPOTTINGCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTAISPOTTINGCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAISpottingComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ClientAISpottingComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientAIProjectileComponent {
}

pub const CLIENTAIPROJECTILECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAIProjectileComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTAIPROJECTILECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAIProjectileComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTAIPROJECTILECOMPONENT_TYPE_INFO
    }
}


pub const CLIENTAIPROJECTILECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAIProjectileComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ClientAIProjectileComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientAIOrderCoordinatorComponent {
}

pub const CLIENTAIORDERCOORDINATORCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAIOrderCoordinatorComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTAIORDERCOORDINATORCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAIOrderCoordinatorComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTAIORDERCOORDINATORCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTAIORDERCOORDINATORCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAIOrderCoordinatorComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ClientAIOrderCoordinatorComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientAIMeleeComponent {
}

pub const CLIENTAIMELEECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAIMeleeComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTAIMELEECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAIMeleeComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTAIMELEECOMPONENT_TYPE_INFO
    }
}


pub const CLIENTAIMELEECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAIMeleeComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ClientAIMeleeComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientAIAnchorToPointComponent {
}

pub const CLIENTAIANCHORTOPOINTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAIAnchorToPointComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTAIANCHORTOPOINTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAIAnchorToPointComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTAIANCHORTOPOINTCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTAIANCHORTOPOINTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAIAnchorToPointComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ClientAIAnchorToPointComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CoverScoreModifierEntity {
}

pub const COVERSCOREMODIFIERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoverScoreModifierEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(COVERSCOREMODIFIERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CoverScoreModifierEntity {
    fn type_info() -> &'static TypeInfo {
        COVERSCOREMODIFIERENTITY_TYPE_INFO
    }
}


pub const COVERSCOREMODIFIERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoverScoreModifierEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("CoverScoreModifierEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPathLinkEntity {
}

pub const SERVERPATHLINKENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPathLinkEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PATHLINKENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERPATHLINKENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPathLinkEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERPATHLINKENTITY_TYPE_INFO
    }
}


pub const SERVERPATHLINKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPathLinkEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerPathLinkEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPathfindingStreamEntity {
}

pub const SERVERPATHFINDINGSTREAMENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPathfindingStreamEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERPATHFINDINGSTREAMENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPathfindingStreamEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERPATHFINDINGSTREAMENTITY_TYPE_INFO
    }
}


pub const SERVERPATHFINDINGSTREAMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPathfindingStreamEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerPathfindingStreamEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerNavPowerObstacleComponent {
}

pub const SERVERNAVPOWEROBSTACLECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerNavPowerObstacleComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERNAVPOWEROBSTACLECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerNavPowerObstacleComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERNAVPOWEROBSTACLECOMPONENT_TYPE_INFO
    }
}


pub const SERVERNAVPOWEROBSTACLECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerNavPowerObstacleComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerNavPowerObstacleComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PathLinkEntity {
}

pub const PATHLINKENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathLinkEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PATHLINKENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PathLinkEntity {
    fn type_info() -> &'static TypeInfo {
        PATHLINKENTITY_TYPE_INFO
    }
}


pub const PATHLINKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathLinkEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("PathLinkEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerWaypointTriggerEntity {
}

pub const SERVERWAYPOINTTRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWaypointTriggerEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERWAYPOINTTRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerWaypointTriggerEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERWAYPOINTTRIGGERENTITY_TYPE_INFO
    }
}


pub const SERVERWAYPOINTTRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWaypointTriggerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerWaypointTriggerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPathFollowingComponent {
}

pub const SERVERPATHFOLLOWINGCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPathFollowingComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERPATHFOLLOWINGCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPathFollowingComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERPATHFOLLOWINGCOMPONENT_TYPE_INFO
    }
}


pub const SERVERPATHFOLLOWINGCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPathFollowingComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerPathFollowingComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPathfindingOverride {
}

pub const SERVERPATHFINDINGOVERRIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPathfindingOverride",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERPATHFINDINGOVERRIDE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPathfindingOverride {
    fn type_info() -> &'static TypeInfo {
        SERVERPATHFINDINGOVERRIDE_TYPE_INFO
    }
}


pub const SERVERPATHFINDINGOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPathfindingOverride-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerPathfindingOverride-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerFollowWaypointsEntity {
}

pub const SERVERFOLLOWWAYPOINTSENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerFollowWaypointsEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERFOLLOWWAYPOINTSENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerFollowWaypointsEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERFOLLOWWAYPOINTSENTITY_TYPE_INFO
    }
}


pub const SERVERFOLLOWWAYPOINTSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerFollowWaypointsEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerFollowWaypointsEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerFollowObjectEntity {
}

pub const SERVERFOLLOWOBJECTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerFollowObjectEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERFOLLOWOBJECTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerFollowObjectEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERFOLLOWOBJECTENTITY_TYPE_INFO
    }
}


pub const SERVERFOLLOWOBJECTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerFollowObjectEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerFollowObjectEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SpatialAnalyzerData {
    pub realm: super::core::Realm,
    pub reference_transform: super::core::LinearTransform,
    pub analysis_f_o_v: f32,
    pub sweep_steps: u32,
    pub edge_distance_threshold: f32,
    pub near_limit_distance: f32,
    pub far_limit_distance: f32,
}

pub const SPATIALANALYZERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpatialAnalyzerData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(SpatialAnalyzerData, realm),
            },
            FieldInfoData {
                name: "ReferenceTransform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(SpatialAnalyzerData, reference_transform),
            },
            FieldInfoData {
                name: "AnalysisFOV",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpatialAnalyzerData, analysis_f_o_v),
            },
            FieldInfoData {
                name: "SweepSteps",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SpatialAnalyzerData, sweep_steps),
            },
            FieldInfoData {
                name: "EdgeDistanceThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpatialAnalyzerData, edge_distance_threshold),
            },
            FieldInfoData {
                name: "NearLimitDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpatialAnalyzerData, near_limit_distance),
            },
            FieldInfoData {
                name: "FarLimitDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpatialAnalyzerData, far_limit_distance),
            },
        ],
    }),
    array_type: Some(SPATIALANALYZERDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SpatialAnalyzerData {
    fn type_info() -> &'static TypeInfo {
        SPATIALANALYZERDATA_TYPE_INFO
    }
}


pub const SPATIALANALYZERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpatialAnalyzerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("SpatialAnalyzerData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AttackPointEntityData {
    pub enabled: bool,
    pub only_accept_linked_spawners: bool,
    pub team_id: super::gameplay_sim::TeamId,
    pub max_target_count_for_l_o_s_check: i32,
    pub navmesh_layer: i32,
    pub ground_transform_horizontal_offset: f32,
}

pub const ATTACKPOINTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AttackPointEntityData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AttackPointEntityData, enabled),
            },
            FieldInfoData {
                name: "OnlyAcceptLinkedSpawners",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AttackPointEntityData, only_accept_linked_spawners),
            },
            FieldInfoData {
                name: "TeamId",
                flags: MemberInfoFlags::new(0),
                field_type: TEAMID_TYPE_INFO,
                rust_offset: offset_of!(AttackPointEntityData, team_id),
            },
            FieldInfoData {
                name: "MaxTargetCountForLOSCheck",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AttackPointEntityData, max_target_count_for_l_o_s_check),
            },
            FieldInfoData {
                name: "NavmeshLayer",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AttackPointEntityData, navmesh_layer),
            },
            FieldInfoData {
                name: "GroundTransformHorizontalOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AttackPointEntityData, ground_transform_horizontal_offset),
            },
        ],
    }),
    array_type: Some(ATTACKPOINTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AttackPointEntityData {
    fn type_info() -> &'static TypeInfo {
        ATTACKPOINTENTITYDATA_TYPE_INFO
    }
}


pub const ATTACKPOINTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AttackPointEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AttackPointEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ActionEntityData {
    pub enabled: bool,
    pub conditions: Vec<ActionCondition>,
    pub move_to_action_before_executing: bool,
    pub align_to_action_before_executing: bool,
    pub wait_time_before_executing: f32,
    pub cooldown_time: f32,
    pub timeout_duration: f32,
    pub abort_when_alerted: bool,
    pub abort_when_timed_out: bool,
    pub valid_in_state: ValidInState,
    pub action_priority: ActionPriority,
    pub should_be_executed_by_closest_a_i: bool,
    pub only_execute_for_fitness_valid_a_i: bool,
    pub use_actual_path_distance: bool,
    pub is_linked_action: bool,
    pub restrict_to_linked_soldiers: bool,
}

pub const ACTIONENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ActionEntityData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ActionEntityData, enabled),
            },
            FieldInfoData {
                name: "Conditions",
                flags: MemberInfoFlags::new(144),
                field_type: ACTIONCONDITION_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ActionEntityData, conditions),
            },
            FieldInfoData {
                name: "MoveToActionBeforeExecuting",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ActionEntityData, move_to_action_before_executing),
            },
            FieldInfoData {
                name: "AlignToActionBeforeExecuting",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ActionEntityData, align_to_action_before_executing),
            },
            FieldInfoData {
                name: "WaitTimeBeforeExecuting",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ActionEntityData, wait_time_before_executing),
            },
            FieldInfoData {
                name: "CooldownTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ActionEntityData, cooldown_time),
            },
            FieldInfoData {
                name: "TimeoutDuration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ActionEntityData, timeout_duration),
            },
            FieldInfoData {
                name: "AbortWhenAlerted",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ActionEntityData, abort_when_alerted),
            },
            FieldInfoData {
                name: "AbortWhenTimedOut",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ActionEntityData, abort_when_timed_out),
            },
            FieldInfoData {
                name: "ValidInState",
                flags: MemberInfoFlags::new(0),
                field_type: VALIDINSTATE_TYPE_INFO,
                rust_offset: offset_of!(ActionEntityData, valid_in_state),
            },
            FieldInfoData {
                name: "ActionPriority",
                flags: MemberInfoFlags::new(0),
                field_type: ACTIONPRIORITY_TYPE_INFO,
                rust_offset: offset_of!(ActionEntityData, action_priority),
            },
            FieldInfoData {
                name: "ShouldBeExecutedByClosestAI",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ActionEntityData, should_be_executed_by_closest_a_i),
            },
            FieldInfoData {
                name: "OnlyExecuteForFitnessValidAI",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ActionEntityData, only_execute_for_fitness_valid_a_i),
            },
            FieldInfoData {
                name: "UseActualPathDistance",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ActionEntityData, use_actual_path_distance),
            },
            FieldInfoData {
                name: "IsLinkedAction",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ActionEntityData, is_linked_action),
            },
            FieldInfoData {
                name: "RestrictToLinkedSoldiers",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ActionEntityData, restrict_to_linked_soldiers),
            },
        ],
    }),
    array_type: Some(ACTIONENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ActionEntityData {
    fn type_info() -> &'static TypeInfo {
        ACTIONENTITYDATA_TYPE_INFO
    }
}


pub const ACTIONENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ActionEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ActionEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ValidInState {
    #[default]
    ValidInState_Idle = 0,
    ValidInState_Search = 1,
    ValidInState_Combat = 2,
    ValidInState_Any = 3,
    ValidInState_Count = 4,
}

pub const VALIDINSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ValidInState",
    flags: MemberInfoFlags::new(49429),
    module: "BattleAI",
    data: TypeInfoData::Enum,
    array_type: Some(VALIDINSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ValidInState {
    fn type_info() -> &'static TypeInfo {
        VALIDINSTATE_TYPE_INFO
    }
}


pub const VALIDINSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ValidInState-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ValidInState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ActionPriority {
    #[default]
    ActionPriority_Idle = 0,
    ActionPriority_Investigate = 1,
    ActionPriority_Search = 2,
    ActionPriority_Count = 3,
}

pub const ACTIONPRIORITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ActionPriority",
    flags: MemberInfoFlags::new(49429),
    module: "BattleAI",
    data: TypeInfoData::Enum,
    array_type: Some(ACTIONPRIORITY_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ActionPriority {
    fn type_info() -> &'static TypeInfo {
        ACTIONPRIORITY_TYPE_INFO
    }
}


pub const ACTIONPRIORITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ActionPriority-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ActionPriority-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ActionState {
    #[default]
    ActionState_Idle = 0,
    ActionState_Approach = 1,
    ActionState_Running = 2,
    ActionState_Cooldown = 3,
    ActionState_Count = 4,
}

pub const ACTIONSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ActionState",
    flags: MemberInfoFlags::new(49429),
    module: "BattleAI",
    data: TypeInfoData::Enum,
    array_type: Some(ACTIONSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ActionState {
    fn type_info() -> &'static TypeInfo {
        ACTIONSTATE_TYPE_INFO
    }
}


pub const ACTIONSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ActionState-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ActionState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AITemplateCondition {
    pub templates: Vec<String>,
}

pub const AITEMPLATECONDITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AITemplateCondition",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ACTIONCONDITION_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Templates",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(AITemplateCondition, templates),
            },
        ],
    }),
    array_type: Some(AITEMPLATECONDITION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AITemplateCondition {
    fn type_info() -> &'static TypeInfo {
        AITEMPLATECONDITION_TYPE_INFO
    }
}


pub const AITEMPLATECONDITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AITemplateCondition-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AITemplateCondition-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct InsideVolumesCondition {
    pub station_inside_user_search_area: bool,
    pub station_inside_user_defend_area: bool,
}

pub const INSIDEVOLUMESCONDITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InsideVolumesCondition",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ACTIONCONDITION_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "StationInsideUserSearchArea",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(InsideVolumesCondition, station_inside_user_search_area),
            },
            FieldInfoData {
                name: "StationInsideUserDefendArea",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(InsideVolumesCondition, station_inside_user_defend_area),
            },
        ],
    }),
    array_type: Some(INSIDEVOLUMESCONDITION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for InsideVolumesCondition {
    fn type_info() -> &'static TypeInfo {
        INSIDEVOLUMESCONDITION_TYPE_INFO
    }
}


pub const INSIDEVOLUMESCONDITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InsideVolumesCondition-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("InsideVolumesCondition-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ProbabilityCondition {
    pub probability: f32,
}

pub const PROBABILITYCONDITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProbabilityCondition",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ACTIONCONDITION_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Probability",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ProbabilityCondition, probability),
            },
        ],
    }),
    array_type: Some(PROBABILITYCONDITION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProbabilityCondition {
    fn type_info() -> &'static TypeInfo {
        PROBABILITYCONDITION_TYPE_INFO
    }
}


pub const PROBABILITYCONDITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProbabilityCondition-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ProbabilityCondition-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FacingCondition {
    pub max_angle: f32,
}

pub const FACINGCONDITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FacingCondition",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ACTIONCONDITION_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MaxAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FacingCondition, max_angle),
            },
        ],
    }),
    array_type: Some(FACINGCONDITION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FacingCondition {
    fn type_info() -> &'static TypeInfo {
        FACINGCONDITION_TYPE_INFO
    }
}


pub const FACINGCONDITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FacingCondition-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("FacingCondition-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AIOrderCoordinatorCondition {
    pub only_valid_when_expecting_self_orders: bool,
}

pub const AIORDERCOORDINATORCONDITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIOrderCoordinatorCondition",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ACTIONCONDITION_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "OnlyValidWhenExpectingSelfOrders",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AIOrderCoordinatorCondition, only_valid_when_expecting_self_orders),
            },
        ],
    }),
    array_type: Some(AIORDERCOORDINATORCONDITION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AIOrderCoordinatorCondition {
    fn type_info() -> &'static TypeInfo {
        AIORDERCOORDINATORCONDITION_TYPE_INFO
    }
}


pub const AIORDERCOORDINATORCONDITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIOrderCoordinatorCondition-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AIOrderCoordinatorCondition-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AIStateCondition {
    pub a_i_state: ConditionAIStates,
    pub restrict_search_area_to_secondary_search: bool,
}

pub const AISTATECONDITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIStateCondition",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ACTIONCONDITION_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "AIState",
                flags: MemberInfoFlags::new(0),
                field_type: CONDITIONAISTATES_TYPE_INFO,
                rust_offset: offset_of!(AIStateCondition, a_i_state),
            },
            FieldInfoData {
                name: "RestrictSearchAreaToSecondarySearch",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AIStateCondition, restrict_search_area_to_secondary_search),
            },
        ],
    }),
    array_type: Some(AISTATECONDITION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AIStateCondition {
    fn type_info() -> &'static TypeInfo {
        AISTATECONDITION_TYPE_INFO
    }
}


pub const AISTATECONDITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIStateCondition-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AIStateCondition-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ConditionAIStates {
    #[default]
    ConditionAIStates_Idle = 0,
    ConditionAIStates_Idle_Investigate = 1,
    ConditionAIStates_Combat = 2,
    ConditionAIStates_Combat_Investigate = 3,
    ConditionAIStates_Combat_SearchArea = 4,
    ConditionAIStates_SearchArea = 5,
}

pub const CONDITIONAISTATES_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionAIStates",
    flags: MemberInfoFlags::new(49429),
    module: "BattleAI",
    data: TypeInfoData::Enum,
    array_type: Some(CONDITIONAISTATES_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ConditionAIStates {
    fn type_info() -> &'static TypeInfo {
        CONDITIONAISTATES_TYPE_INFO
    }
}


pub const CONDITIONAISTATES_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionAIStates-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ConditionAIStates-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RangeCondition {
    pub radius: f32,
}

pub const RANGECONDITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RangeCondition",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ACTIONCONDITION_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RangeCondition, radius),
            },
        ],
    }),
    array_type: Some(RANGECONDITION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RangeCondition {
    fn type_info() -> &'static TypeInfo {
        RANGECONDITION_TYPE_INFO
    }
}


pub const RANGECONDITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RangeCondition-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("RangeCondition-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ActionCondition {
}

pub const ACTIONCONDITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ActionCondition",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ACTIONCONDITION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ActionCondition {
    fn type_info() -> &'static TypeInfo {
        ACTIONCONDITION_TYPE_INFO
    }
}


pub const ACTIONCONDITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ActionCondition-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ActionCondition-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PreferredCoverScoreData {
    pub preferred_cover_id: u32,
    pub score: f32,
}

pub const PREFERREDCOVERSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PreferredCoverScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CUSTOMCOVERSCOREDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "PreferredCoverId",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PreferredCoverScoreData, preferred_cover_id),
            },
            FieldInfoData {
                name: "Score",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PreferredCoverScoreData, score),
            },
        ],
    }),
    array_type: Some(PREFERREDCOVERSCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PreferredCoverScoreData {
    fn type_info() -> &'static TypeInfo {
        PREFERREDCOVERSCOREDATA_TYPE_INFO
    }
}


pub const PREFERREDCOVERSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PreferredCoverScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("PreferredCoverScoreData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PreferredAreaScoreData {
    pub shapes: Vec<BaseShapeWithOffset>,
    pub score: f32,
}

pub const PREFERREDAREASCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PreferredAreaScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CUSTOMCOVERSCOREDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Shapes",
                flags: MemberInfoFlags::new(144),
                field_type: BASESHAPEWITHOFFSET_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(PreferredAreaScoreData, shapes),
            },
            FieldInfoData {
                name: "Score",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PreferredAreaScoreData, score),
            },
        ],
    }),
    array_type: Some(PREFERREDAREASCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PreferredAreaScoreData {
    fn type_info() -> &'static TypeInfo {
        PREFERREDAREASCOREDATA_TYPE_INFO
    }
}


pub const PREFERREDAREASCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PreferredAreaScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("PreferredAreaScoreData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct BaseShapeWithOffset {
    pub shape: super::entity::BaseShapeData,
    pub offset: super::core::LinearTransform,
    pub owner_transform: super::core::LinearTransform,
}

pub const BASESHAPEWITHOFFSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BaseShapeWithOffset",
    flags: MemberInfoFlags::new(73),
    module: "BattleAI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Shape",
                flags: MemberInfoFlags::new(0),
                field_type: BASESHAPEDATA_TYPE_INFO,
                rust_offset: offset_of!(BaseShapeWithOffset, shape),
            },
            FieldInfoData {
                name: "Offset",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(BaseShapeWithOffset, offset),
            },
            FieldInfoData {
                name: "OwnerTransform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(BaseShapeWithOffset, owner_transform),
            },
        ],
    }),
    array_type: Some(BASESHAPEWITHOFFSET_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for BaseShapeWithOffset {
    fn type_info() -> &'static TypeInfo {
        BASESHAPEWITHOFFSET_TYPE_INFO
    }
}


pub const BASESHAPEWITHOFFSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BaseShapeWithOffset-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("BaseShapeWithOffset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CustomCoverScoreData {
}

pub const CUSTOMCOVERSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomCoverScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CUSTOMCOVERSCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CustomCoverScoreData {
    fn type_info() -> &'static TypeInfo {
        CUSTOMCOVERSCOREDATA_TYPE_INFO
    }
}


pub const CUSTOMCOVERSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomCoverScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("CustomCoverScoreData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ReducePathDistanceToFollowObjectScoreData {
}

pub const REDUCEPATHDISTANCETOFOLLOWOBJECTSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReducePathDistanceToFollowObjectScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(REDUCEPATHDISTANCETOFOLLOWOBJECTSCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ReducePathDistanceToFollowObjectScoreData {
    fn type_info() -> &'static TypeInfo {
        REDUCEPATHDISTANCETOFOLLOWOBJECTSCOREDATA_TYPE_INFO
    }
}


pub const REDUCEPATHDISTANCETOFOLLOWOBJECTSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReducePathDistanceToFollowObjectScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ReducePathDistanceToFollowObjectScoreData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ReducePathDistanceToTargetScoreData {
}

pub const REDUCEPATHDISTANCETOTARGETSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReducePathDistanceToTargetScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(REDUCEPATHDISTANCETOTARGETSCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ReducePathDistanceToTargetScoreData {
    fn type_info() -> &'static TypeInfo {
        REDUCEPATHDISTANCETOTARGETSCOREDATA_TYPE_INFO
    }
}


pub const REDUCEPATHDISTANCETOTARGETSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReducePathDistanceToTargetScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ReducePathDistanceToTargetScoreData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LineOfFireScoreData {
    pub override_open_cover_height: f32,
}

pub const LINEOFFIRESCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LineOfFireScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "OverrideOpenCoverHeight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LineOfFireScoreData, override_open_cover_height),
            },
        ],
    }),
    array_type: Some(LINEOFFIRESCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LineOfFireScoreData {
    fn type_info() -> &'static TypeInfo {
        LINEOFFIRESCOREDATA_TYPE_INFO
    }
}


pub const LINEOFFIRESCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LineOfFireScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("LineOfFireScoreData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct NavProbeScoreData {
    pub ref_position: CoverScorePosition,
    pub score: f32,
    pub max_probe_distance: f32,
}

pub const NAVPROBESCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NavProbeScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "RefPosition",
                flags: MemberInfoFlags::new(0),
                field_type: COVERSCOREPOSITION_TYPE_INFO,
                rust_offset: offset_of!(NavProbeScoreData, ref_position),
            },
            FieldInfoData {
                name: "Score",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(NavProbeScoreData, score),
            },
            FieldInfoData {
                name: "MaxProbeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(NavProbeScoreData, max_probe_distance),
            },
        ],
    }),
    array_type: Some(NAVPROBESCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NavProbeScoreData {
    fn type_info() -> &'static TypeInfo {
        NAVPROBESCOREDATA_TYPE_INFO
    }
}


pub const NAVPROBESCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NavProbeScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("NavProbeScoreData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct BehindFollowObjectScoreData {
    pub max_enemy_distance: f32,
    pub enemy_position_weight_curve: super::core::FloatCurve,
}

pub const BEHINDFOLLOWOBJECTSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BehindFollowObjectScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MaxEnemyDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BehindFollowObjectScoreData, max_enemy_distance),
            },
            FieldInfoData {
                name: "EnemyPositionWeightCurve",
                flags: MemberInfoFlags::new(0),
                field_type: FLOATCURVE_TYPE_INFO,
                rust_offset: offset_of!(BehindFollowObjectScoreData, enemy_position_weight_curve),
            },
        ],
    }),
    array_type: Some(BEHINDFOLLOWOBJECTSCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BehindFollowObjectScoreData {
    fn type_info() -> &'static TypeInfo {
        BEHINDFOLLOWOBJECTSCOREDATA_TYPE_INFO
    }
}


pub const BEHINDFOLLOWOBJECTSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BehindFollowObjectScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("BehindFollowObjectScoreData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AngleFromFollowObjectAimDirectionData {
    pub apply_only_if_aim_direction_stable: bool,
    pub max_follow_object_speed: f32,
}

pub const ANGLEFROMFOLLOWOBJECTAIMDIRECTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngleFromFollowObjectAimDirectionData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ApplyOnlyIfAimDirectionStable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AngleFromFollowObjectAimDirectionData, apply_only_if_aim_direction_stable),
            },
            FieldInfoData {
                name: "MaxFollowObjectSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AngleFromFollowObjectAimDirectionData, max_follow_object_speed),
            },
        ],
    }),
    array_type: Some(ANGLEFROMFOLLOWOBJECTAIMDIRECTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AngleFromFollowObjectAimDirectionData {
    fn type_info() -> &'static TypeInfo {
        ANGLEFROMFOLLOWOBJECTAIMDIRECTIONDATA_TYPE_INFO
    }
}


pub const ANGLEFROMFOLLOWOBJECTAIMDIRECTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngleFromFollowObjectAimDirectionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AngleFromFollowObjectAimDirectionData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FollowObjectReticleAvoidanceData {
    pub apply_only_if_aim_direction_stable: bool,
    pub max_follow_object_speed: f32,
    pub soldier_radius_expansion: f32,
    pub path_look_ahead_distance: f32,
}

pub const FOLLOWOBJECTRETICLEAVOIDANCEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FollowObjectReticleAvoidanceData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ApplyOnlyIfAimDirectionStable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FollowObjectReticleAvoidanceData, apply_only_if_aim_direction_stable),
            },
            FieldInfoData {
                name: "MaxFollowObjectSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FollowObjectReticleAvoidanceData, max_follow_object_speed),
            },
            FieldInfoData {
                name: "SoldierRadiusExpansion",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FollowObjectReticleAvoidanceData, soldier_radius_expansion),
            },
            FieldInfoData {
                name: "PathLookAheadDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FollowObjectReticleAvoidanceData, path_look_ahead_distance),
            },
        ],
    }),
    array_type: Some(FOLLOWOBJECTRETICLEAVOIDANCEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FollowObjectReticleAvoidanceData {
    fn type_info() -> &'static TypeInfo {
        FOLLOWOBJECTRETICLEAVOIDANCEDATA_TYPE_INFO
    }
}


pub const FOLLOWOBJECTRETICLEAVOIDANCEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FollowObjectReticleAvoidanceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("FollowObjectReticleAvoidanceData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PathAvoidanceScoreData {
    pub avoid_by_type_data: Vec<super::battle_a_i_shared::CoverQueryPathEnemyAvoidanceByTypeData>,
    pub max_search_distance: f32,
    pub reject_cover_beyond_search_distance: bool,
    pub inner_zone_score: f32,
    pub outer_zone_score: f32,
    pub not_passing_avoidance_area_score: f32,
    pub scores_per_zone: Vec<f32>,
}

pub const PATHAVOIDANCESCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathAvoidanceScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "AvoidByTypeData",
                flags: MemberInfoFlags::new(144),
                field_type: COVERQUERYPATHENEMYAVOIDANCEBYTYPEDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(PathAvoidanceScoreData, avoid_by_type_data),
            },
            FieldInfoData {
                name: "MaxSearchDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PathAvoidanceScoreData, max_search_distance),
            },
            FieldInfoData {
                name: "RejectCoverBeyondSearchDistance",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PathAvoidanceScoreData, reject_cover_beyond_search_distance),
            },
            FieldInfoData {
                name: "InnerZoneScore",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PathAvoidanceScoreData, inner_zone_score),
            },
            FieldInfoData {
                name: "OuterZoneScore",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PathAvoidanceScoreData, outer_zone_score),
            },
            FieldInfoData {
                name: "NotPassingAvoidanceAreaScore",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PathAvoidanceScoreData, not_passing_avoidance_area_score),
            },
            FieldInfoData {
                name: "ScoresPerZone",
                flags: MemberInfoFlags::new(144),
                field_type: FLOAT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(PathAvoidanceScoreData, scores_per_zone),
            },
        ],
    }),
    array_type: Some(PATHAVOIDANCESCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PathAvoidanceScoreData {
    fn type_info() -> &'static TypeInfo {
        PATHAVOIDANCESCOREDATA_TYPE_INFO
    }
}


pub const PATHAVOIDANCESCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathAvoidanceScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("PathAvoidanceScoreData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PathDistanceScoreData {
    pub max_search_distance: f32,
    pub precise_path: bool,
    pub reject_cover_beyond_search_distance: bool,
}

pub const PATHDISTANCESCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathDistanceScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHREFPOS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MaxSearchDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PathDistanceScoreData, max_search_distance),
            },
            FieldInfoData {
                name: "PrecisePath",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PathDistanceScoreData, precise_path),
            },
            FieldInfoData {
                name: "RejectCoverBeyondSearchDistance",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PathDistanceScoreData, reject_cover_beyond_search_distance),
            },
        ],
    }),
    array_type: Some(PATHDISTANCESCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PathDistanceScoreData {
    fn type_info() -> &'static TypeInfo {
        PATHDISTANCESCOREDATA_TYPE_INFO
    }
}


pub const PATHDISTANCESCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathDistanceScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("PathDistanceScoreData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RejectUnreachableCoverScoreData {
    pub ref_position: CoverScorePosition,
}

pub const REJECTUNREACHABLECOVERSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RejectUnreachableCoverScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "RefPosition",
                flags: MemberInfoFlags::new(0),
                field_type: COVERSCOREPOSITION_TYPE_INFO,
                rust_offset: offset_of!(RejectUnreachableCoverScoreData, ref_position),
            },
        ],
    }),
    array_type: Some(REJECTUNREACHABLECOVERSCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RejectUnreachableCoverScoreData {
    fn type_info() -> &'static TypeInfo {
        REJECTUNREACHABLECOVERSCOREDATA_TYPE_INFO
    }
}


pub const REJECTUNREACHABLECOVERSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RejectUnreachableCoverScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("RejectUnreachableCoverScoreData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DistanceToCorpseData {
    pub max_time_since_death: f32,
}

pub const DISTANCETOCORPSEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistanceToCorpseData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MaxTimeSinceDeath",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DistanceToCorpseData, max_time_since_death),
            },
        ],
    }),
    array_type: Some(DISTANCETOCORPSEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DistanceToCorpseData {
    fn type_info() -> &'static TypeInfo {
        DISTANCETOCORPSEDATA_TYPE_INFO
    }
}


pub const DISTANCETOCORPSEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistanceToCorpseData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("DistanceToCorpseData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DistanceToClosestEnemyCoverScoreData {
}

pub const DISTANCETOCLOSESTENEMYCOVERSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistanceToClosestEnemyCoverScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DISTANCETOCLOSESTENEMYCOVERSCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DistanceToClosestEnemyCoverScoreData {
    fn type_info() -> &'static TypeInfo {
        DISTANCETOCLOSESTENEMYCOVERSCOREDATA_TYPE_INFO
    }
}


pub const DISTANCETOCLOSESTENEMYCOVERSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistanceToClosestEnemyCoverScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("DistanceToClosestEnemyCoverScoreData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DistanceToClosestFriendlyScoreData {
}

pub const DISTANCETOCLOSESTFRIENDLYSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistanceToClosestFriendlyScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DISTANCETOCLOSESTFRIENDLYSCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DistanceToClosestFriendlyScoreData {
    fn type_info() -> &'static TypeInfo {
        DISTANCETOCLOSESTFRIENDLYSCOREDATA_TYPE_INFO
    }
}


pub const DISTANCETOCLOSESTFRIENDLYSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistanceToClosestFriendlyScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("DistanceToClosestFriendlyScoreData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ExposureToMultiTargetsScoreData {
    pub exclude_primary_target: bool,
    pub ref_position_for_target_filtering: CoverScorePosition,
    pub target_significance_distance_curve: super::core::FloatCurve,
    pub max_target_distance: f32,
    pub max_distance_ratio_from_closest_target: f32,
    pub min_target_distance_to_always_be_counted: f32,
    pub max_distance_ratio_from_closest_exposed_target: f32,
    pub max_distance_to_fully_reject_exposed_target: f32,
}

pub const EXPOSURETOMULTITARGETSSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExposureToMultiTargetsScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ExcludePrimaryTarget",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ExposureToMultiTargetsScoreData, exclude_primary_target),
            },
            FieldInfoData {
                name: "RefPositionForTargetFiltering",
                flags: MemberInfoFlags::new(0),
                field_type: COVERSCOREPOSITION_TYPE_INFO,
                rust_offset: offset_of!(ExposureToMultiTargetsScoreData, ref_position_for_target_filtering),
            },
            FieldInfoData {
                name: "TargetSignificanceDistanceCurve",
                flags: MemberInfoFlags::new(0),
                field_type: FLOATCURVE_TYPE_INFO,
                rust_offset: offset_of!(ExposureToMultiTargetsScoreData, target_significance_distance_curve),
            },
            FieldInfoData {
                name: "MaxTargetDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ExposureToMultiTargetsScoreData, max_target_distance),
            },
            FieldInfoData {
                name: "MaxDistanceRatioFromClosestTarget",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ExposureToMultiTargetsScoreData, max_distance_ratio_from_closest_target),
            },
            FieldInfoData {
                name: "MinTargetDistanceToAlwaysBeCounted",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ExposureToMultiTargetsScoreData, min_target_distance_to_always_be_counted),
            },
            FieldInfoData {
                name: "MaxDistanceRatioFromClosestExposedTarget",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ExposureToMultiTargetsScoreData, max_distance_ratio_from_closest_exposed_target),
            },
            FieldInfoData {
                name: "MaxDistanceToFullyRejectExposedTarget",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ExposureToMultiTargetsScoreData, max_distance_to_fully_reject_exposed_target),
            },
        ],
    }),
    array_type: Some(EXPOSURETOMULTITARGETSSCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ExposureToMultiTargetsScoreData {
    fn type_info() -> &'static TypeInfo {
        EXPOSURETOMULTITARGETSSCOREDATA_TYPE_INFO
    }
}


pub const EXPOSURETOMULTITARGETSSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExposureToMultiTargetsScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ExposureToMultiTargetsScoreData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DistanceToMultiTargetsScoreData {
}

pub const DISTANCETOMULTITARGETSSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistanceToMultiTargetsScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MULTITARGETSCORER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DISTANCETOMULTITARGETSSCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DistanceToMultiTargetsScoreData {
    fn type_info() -> &'static TypeInfo {
        DISTANCETOMULTITARGETSSCOREDATA_TYPE_INFO
    }
}


pub const DISTANCETOMULTITARGETSSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistanceToMultiTargetsScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("DistanceToMultiTargetsScoreData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AngleToMultiTargetsScoreData {
}

pub const ANGLETOMULTITARGETSSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngleToMultiTargetsScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MULTITARGETSCORER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ANGLETOMULTITARGETSSCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AngleToMultiTargetsScoreData {
    fn type_info() -> &'static TypeInfo {
        ANGLETOMULTITARGETSSCOREDATA_TYPE_INFO
    }
}


pub const ANGLETOMULTITARGETSSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngleToMultiTargetsScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AngleToMultiTargetsScoreData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MultiTargetScorer {
    pub exclude_primary_target: bool,
    pub max_distance_to_target: f32,
    pub scoring_mode: MultiTargetScoringMode,
}

pub const MULTITARGETSCORER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiTargetScorer",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ExcludePrimaryTarget",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MultiTargetScorer, exclude_primary_target),
            },
            FieldInfoData {
                name: "MaxDistanceToTarget",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MultiTargetScorer, max_distance_to_target),
            },
            FieldInfoData {
                name: "ScoringMode",
                flags: MemberInfoFlags::new(0),
                field_type: MULTITARGETSCORINGMODE_TYPE_INFO,
                rust_offset: offset_of!(MultiTargetScorer, scoring_mode),
            },
        ],
    }),
    array_type: Some(MULTITARGETSCORER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MultiTargetScorer {
    fn type_info() -> &'static TypeInfo {
        MULTITARGETSCORER_TYPE_INFO
    }
}


pub const MULTITARGETSCORER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiTargetScorer-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("MultiTargetScorer-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum MultiTargetScoringMode {
    #[default]
    MultiTargetScoringMode_MostSignificant = 0,
    MultiTargetScoringMode_Average = 1,
}

pub const MULTITARGETSCORINGMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiTargetScoringMode",
    flags: MemberInfoFlags::new(49429),
    module: "BattleAI",
    data: TypeInfoData::Enum,
    array_type: Some(MULTITARGETSCORINGMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MultiTargetScoringMode {
    fn type_info() -> &'static TypeInfo {
        MULTITARGETSCORINGMODE_TYPE_INFO
    }
}


pub const MULTITARGETSCORINGMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiTargetScoringMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("MultiTargetScoringMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TowardsPreferredWeaponRangeScoreData {
    pub outside_preferred_range_score_curve: super::core::FloatCurve,
    pub inside_preferred_range_score_curve: super::core::FloatCurve,
}

pub const TOWARDSPREFERREDWEAPONRANGESCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TowardsPreferredWeaponRangeScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "OutsidePreferredRangeScoreCurve",
                flags: MemberInfoFlags::new(0),
                field_type: FLOATCURVE_TYPE_INFO,
                rust_offset: offset_of!(TowardsPreferredWeaponRangeScoreData, outside_preferred_range_score_curve),
            },
            FieldInfoData {
                name: "InsidePreferredRangeScoreCurve",
                flags: MemberInfoFlags::new(0),
                field_type: FLOATCURVE_TYPE_INFO,
                rust_offset: offset_of!(TowardsPreferredWeaponRangeScoreData, inside_preferred_range_score_curve),
            },
        ],
    }),
    array_type: Some(TOWARDSPREFERREDWEAPONRANGESCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TowardsPreferredWeaponRangeScoreData {
    fn type_info() -> &'static TypeInfo {
        TOWARDSPREFERREDWEAPONRANGESCOREDATA_TYPE_INFO
    }
}


pub const TOWARDSPREFERREDWEAPONRANGESCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TowardsPreferredWeaponRangeScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("TowardsPreferredWeaponRangeScoreData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PreferredWeaponRangeScoreData {
}

pub const PREFERREDWEAPONRANGESCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PreferredWeaponRangeScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PREFERREDWEAPONRANGESCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PreferredWeaponRangeScoreData {
    fn type_info() -> &'static TypeInfo {
        PREFERREDWEAPONRANGESCOREDATA_TYPE_INFO
    }
}


pub const PREFERREDWEAPONRANGESCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PreferredWeaponRangeScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("PreferredWeaponRangeScoreData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ProjectedDistanceScoreData {
    pub ref_direction: CoverScoreDirection,
    pub flip_ref_direction: bool,
}

pub const PROJECTEDDISTANCESCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProjectedDistanceScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHREFPOS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "RefDirection",
                flags: MemberInfoFlags::new(0),
                field_type: COVERSCOREDIRECTION_TYPE_INFO,
                rust_offset: offset_of!(ProjectedDistanceScoreData, ref_direction),
            },
            FieldInfoData {
                name: "FlipRefDirection",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ProjectedDistanceScoreData, flip_ref_direction),
            },
        ],
    }),
    array_type: Some(PROJECTEDDISTANCESCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProjectedDistanceScoreData {
    fn type_info() -> &'static TypeInfo {
        PROJECTEDDISTANCESCOREDATA_TYPE_INFO
    }
}


pub const PROJECTEDDISTANCESCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProjectedDistanceScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ProjectedDistanceScoreData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum CoverScoreDirection {
    #[default]
    CoverScoreDirection_SoldierToTarget = 0,
    CoverScoreDirection_SoldierToFollowObject = 1,
    CoverScoreDirection_WorldUpDirection = 2,
}

pub const COVERSCOREDIRECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoverScoreDirection",
    flags: MemberInfoFlags::new(49429),
    module: "BattleAI",
    data: TypeInfoData::Enum,
    array_type: Some(COVERSCOREDIRECTION_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CoverScoreDirection {
    fn type_info() -> &'static TypeInfo {
        COVERSCOREDIRECTION_TYPE_INFO
    }
}


pub const COVERSCOREDIRECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoverScoreDirection-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("CoverScoreDirection-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DistanceToActorScoreData {
}

pub const DISTANCETOACTORSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistanceToActorScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHREFPOS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DISTANCETOACTORSCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DistanceToActorScoreData {
    fn type_info() -> &'static TypeInfo {
        DISTANCETOACTORSCOREDATA_TYPE_INFO
    }
}


pub const DISTANCETOACTORSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistanceToActorScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("DistanceToActorScoreData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AngleFromPathTrajectoryScoreData {
    pub path_look_ahead_distance: f32,
}

pub const ANGLEFROMPATHTRAJECTORYSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngleFromPathTrajectoryScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "PathLookAheadDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AngleFromPathTrajectoryScoreData, path_look_ahead_distance),
            },
        ],
    }),
    array_type: Some(ANGLEFROMPATHTRAJECTORYSCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AngleFromPathTrajectoryScoreData {
    fn type_info() -> &'static TypeInfo {
        ANGLEFROMPATHTRAJECTORYSCOREDATA_TYPE_INFO
    }
}


pub const ANGLEFROMPATHTRAJECTORYSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngleFromPathTrajectoryScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AngleFromPathTrajectoryScoreData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AngleFromReferenceDirectionScoreData {
    pub ref_dir_from_pos: CoverScorePosition,
    pub ref_dir_to_pos: CoverScorePosition,
}

pub const ANGLEFROMREFERENCEDIRECTIONSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngleFromReferenceDirectionScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "RefDirFromPos",
                flags: MemberInfoFlags::new(0),
                field_type: COVERSCOREPOSITION_TYPE_INFO,
                rust_offset: offset_of!(AngleFromReferenceDirectionScoreData, ref_dir_from_pos),
            },
            FieldInfoData {
                name: "RefDirToPos",
                flags: MemberInfoFlags::new(0),
                field_type: COVERSCOREPOSITION_TYPE_INFO,
                rust_offset: offset_of!(AngleFromReferenceDirectionScoreData, ref_dir_to_pos),
            },
        ],
    }),
    array_type: Some(ANGLEFROMREFERENCEDIRECTIONSCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AngleFromReferenceDirectionScoreData {
    fn type_info() -> &'static TypeInfo {
        ANGLEFROMREFERENCEDIRECTIONSCOREDATA_TYPE_INFO
    }
}


pub const ANGLEFROMREFERENCEDIRECTIONSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngleFromReferenceDirectionScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AngleFromReferenceDirectionScoreData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AngleToActorScoreData2 {
    pub ref_position: CoverScorePosition,
    pub scores_for_filter: Vec<ScoreCurveForFilter>,
}

pub const ANGLETOACTORSCOREDATA2_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngleToActorScoreData2",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "RefPosition",
                flags: MemberInfoFlags::new(0),
                field_type: COVERSCOREPOSITION_TYPE_INFO,
                rust_offset: offset_of!(AngleToActorScoreData2, ref_position),
            },
            FieldInfoData {
                name: "ScoresForFilter",
                flags: MemberInfoFlags::new(144),
                field_type: SCORECURVEFORFILTER_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(AngleToActorScoreData2, scores_for_filter),
            },
        ],
    }),
    array_type: Some(ANGLETOACTORSCOREDATA2_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AngleToActorScoreData2 {
    fn type_info() -> &'static TypeInfo {
        ANGLETOACTORSCOREDATA2_TYPE_INFO
    }
}


pub const ANGLETOACTORSCOREDATA2_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngleToActorScoreData2-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AngleToActorScoreData2-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ScoreCurveForFilter {
    pub score_curve: super::core::FloatCurve,
    pub runtime_filter: u32,
}

pub const SCORECURVEFORFILTER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScoreCurveForFilter",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ScoreCurve",
                flags: MemberInfoFlags::new(0),
                field_type: FLOATCURVE_TYPE_INFO,
                rust_offset: offset_of!(ScoreCurveForFilter, score_curve),
            },
            FieldInfoData {
                name: "RuntimeFilter",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ScoreCurveForFilter, runtime_filter),
            },
        ],
    }),
    array_type: Some(SCORECURVEFORFILTER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ScoreCurveForFilter {
    fn type_info() -> &'static TypeInfo {
        SCORECURVEFORFILTER_TYPE_INFO
    }
}


pub const SCORECURVEFORFILTER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScoreCurveForFilter-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ScoreCurveForFilter-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AngleToActorScoreData {
}

pub const ANGLETOACTORSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngleToActorScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHREFPOS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ANGLETOACTORSCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AngleToActorScoreData {
    fn type_info() -> &'static TypeInfo {
        ANGLETOACTORSCOREDATA_TYPE_INFO
    }
}


pub const ANGLETOACTORSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngleToActorScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AngleToActorScoreData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CoverScoreDataWithRefPos {
    pub ref_position: CoverScorePosition,
}

pub const COVERSCOREDATAWITHREFPOS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoverScoreDataWithRefPos",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "RefPosition",
                flags: MemberInfoFlags::new(0),
                field_type: COVERSCOREPOSITION_TYPE_INFO,
                rust_offset: offset_of!(CoverScoreDataWithRefPos, ref_position),
            },
        ],
    }),
    array_type: Some(COVERSCOREDATAWITHREFPOS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CoverScoreDataWithRefPos {
    fn type_info() -> &'static TypeInfo {
        COVERSCOREDATAWITHREFPOS_TYPE_INFO
    }
}


pub const COVERSCOREDATAWITHREFPOS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoverScoreDataWithRefPos-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("CoverScoreDataWithRefPos-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CoverScoreDataWithScoreCurve {
    pub score_curve: super::core::FloatCurve,
    pub score_curve_scale: f32,
    pub score_curve_max_y: f32,
}

pub const COVERSCOREDATAWITHSCORECURVE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoverScoreDataWithScoreCurve",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ScoreCurve",
                flags: MemberInfoFlags::new(0),
                field_type: FLOATCURVE_TYPE_INFO,
                rust_offset: offset_of!(CoverScoreDataWithScoreCurve, score_curve),
            },
            FieldInfoData {
                name: "ScoreCurveScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CoverScoreDataWithScoreCurve, score_curve_scale),
            },
            FieldInfoData {
                name: "ScoreCurveMaxY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CoverScoreDataWithScoreCurve, score_curve_max_y),
            },
        ],
    }),
    array_type: Some(COVERSCOREDATAWITHSCORECURVE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CoverScoreDataWithScoreCurve {
    fn type_info() -> &'static TypeInfo {
        COVERSCOREDATAWITHSCORECURVE_TYPE_INFO
    }
}


pub const COVERSCOREDATAWITHSCORECURVE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoverScoreDataWithScoreCurve-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("CoverScoreDataWithScoreCurve-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CurrentCoverBonusScoreData {
    pub bonus_score: f32,
}

pub const CURRENTCOVERBONUSSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CurrentCoverBonusScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "BonusScore",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CurrentCoverBonusScoreData, bonus_score),
            },
        ],
    }),
    array_type: Some(CURRENTCOVERBONUSSCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CurrentCoverBonusScoreData {
    fn type_info() -> &'static TypeInfo {
        CURRENTCOVERBONUSSCOREDATA_TYPE_INFO
    }
}


pub const CURRENTCOVERBONUSSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CurrentCoverBonusScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("CurrentCoverBonusScoreData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CoverFilterScoreData {
    pub matching_score: f32,
}

pub const COVERFILTERSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoverFilterScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MatchingScore",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CoverFilterScoreData, matching_score),
            },
        ],
    }),
    array_type: Some(COVERFILTERSCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CoverFilterScoreData {
    fn type_info() -> &'static TypeInfo {
        COVERFILTERSCOREDATA_TYPE_INFO
    }
}


pub const COVERFILTERSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoverFilterScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("CoverFilterScoreData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CoverScoreData {
    pub comment: String,
    pub enabled: bool,
}

pub const COVERSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoverScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATABASE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Comment",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(CoverScoreData, comment),
            },
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CoverScoreData, enabled),
            },
        ],
    }),
    array_type: Some(COVERSCOREDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CoverScoreData {
    fn type_info() -> &'static TypeInfo {
        COVERSCOREDATA_TYPE_INFO
    }
}


pub const COVERSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoverScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("CoverScoreData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum CoverScorePosition {
    #[default]
    CoverScorePosition_Soldier = 0,
    CoverScorePosition_Target = 1,
    CoverScorePosition_FollowObject = 2,
    CoverScorePosition_CriticalThreat = 3,
    CoverScorePosition_Count = 4,
}

pub const COVERSCOREPOSITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoverScorePosition",
    flags: MemberInfoFlags::new(49429),
    module: "BattleAI",
    data: TypeInfoData::Enum,
    array_type: Some(COVERSCOREPOSITION_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CoverScorePosition {
    fn type_info() -> &'static TypeInfo {
        COVERSCOREPOSITION_TYPE_INFO
    }
}


pub const COVERSCOREPOSITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoverScorePosition-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("CoverScorePosition-Array"),
    array_type: None,
    alignment: 8,
};


