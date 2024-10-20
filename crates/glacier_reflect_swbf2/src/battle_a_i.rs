use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct ServerSimpleDriverComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerSimpleDriverComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerSimpleDriverComponentTrait for ServerSimpleDriverComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerSimpleDriverComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerSimpleDriverComponent {
}

impl super::entity::ComponentTrait for ServerSimpleDriverComponent {
}

impl super::entity::EntityBusPeerTrait for ServerSimpleDriverComponent {
}

pub static SERVERSIMPLEDRIVERCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSimpleDriverComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSimpleDriverComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSIMPLEDRIVERCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSimpleDriverComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSIMPLEDRIVERCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERSIMPLEDRIVERCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSimpleDriverComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerSimpleDriverComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WSServerDogFightingEntity {
    pub _glacier_base: ServerDogFightingEntity,
}

pub trait WSServerDogFightingEntityTrait: ServerDogFightingEntityTrait {
}

impl WSServerDogFightingEntityTrait for WSServerDogFightingEntity {
}

impl ServerDogFightingEntityTrait for WSServerDogFightingEntity {
}

impl super::entity::EntityTrait for WSServerDogFightingEntity {
}

impl super::entity::EntityBusPeerTrait for WSServerDogFightingEntity {
}

pub static WSSERVERDOGFIGHTINGENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WSServerDogFightingEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTINGENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WSServerDogFightingEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(WSSERVERDOGFIGHTINGENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for WSServerDogFightingEntity {
    fn type_info(&self) -> &'static TypeInfo {
        WSSERVERDOGFIGHTINGENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WSSERVERDOGFIGHTINGENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WSServerDogFightingEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("WSServerDogFightingEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerStrafeRunManeuverEntity {
    pub _glacier_base: ServerDogFightManeuverEntityBase,
}

pub trait ServerStrafeRunManeuverEntityTrait: ServerDogFightManeuverEntityBaseTrait {
}

impl ServerStrafeRunManeuverEntityTrait for ServerStrafeRunManeuverEntity {
}

impl ServerDogFightManeuverEntityBaseTrait for ServerStrafeRunManeuverEntity {
}

impl super::entity::EntityTrait for ServerStrafeRunManeuverEntity {
}

impl super::entity::EntityBusPeerTrait for ServerStrafeRunManeuverEntity {
}

pub static SERVERSTRAFERUNMANEUVERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStrafeRunManeuverEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerStrafeRunManeuverEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSTRAFERUNMANEUVERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerStrafeRunManeuverEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSTRAFERUNMANEUVERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERSTRAFERUNMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStrafeRunManeuverEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerStrafeRunManeuverEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerStallTurnManeuverEntity {
    pub _glacier_base: ServerDogFightManeuverEntityBase,
}

pub trait ServerStallTurnManeuverEntityTrait: ServerDogFightManeuverEntityBaseTrait {
}

impl ServerStallTurnManeuverEntityTrait for ServerStallTurnManeuverEntity {
}

impl ServerDogFightManeuverEntityBaseTrait for ServerStallTurnManeuverEntity {
}

impl super::entity::EntityTrait for ServerStallTurnManeuverEntity {
}

impl super::entity::EntityBusPeerTrait for ServerStallTurnManeuverEntity {
}

pub static SERVERSTALLTURNMANEUVERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStallTurnManeuverEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerStallTurnManeuverEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSTALLTURNMANEUVERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerStallTurnManeuverEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSTALLTURNMANEUVERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERSTALLTURNMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStallTurnManeuverEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerStallTurnManeuverEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerSplitSManeuverEntity {
    pub _glacier_base: ServerDogFightManeuverEntityBase,
}

pub trait ServerSplitSManeuverEntityTrait: ServerDogFightManeuverEntityBaseTrait {
}

impl ServerSplitSManeuverEntityTrait for ServerSplitSManeuverEntity {
}

impl ServerDogFightManeuverEntityBaseTrait for ServerSplitSManeuverEntity {
}

impl super::entity::EntityTrait for ServerSplitSManeuverEntity {
}

impl super::entity::EntityBusPeerTrait for ServerSplitSManeuverEntity {
}

pub static SERVERSPLITSMANEUVERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSplitSManeuverEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSplitSManeuverEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSPLITSMANEUVERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSplitSManeuverEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSPLITSMANEUVERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERSPLITSMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSplitSManeuverEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerSplitSManeuverEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerSpinDescentManeuverEntity {
    pub _glacier_base: ServerDogFightManeuverEntityBase,
}

pub trait ServerSpinDescentManeuverEntityTrait: ServerDogFightManeuverEntityBaseTrait {
}

impl ServerSpinDescentManeuverEntityTrait for ServerSpinDescentManeuverEntity {
}

impl ServerDogFightManeuverEntityBaseTrait for ServerSpinDescentManeuverEntity {
}

impl super::entity::EntityTrait for ServerSpinDescentManeuverEntity {
}

impl super::entity::EntityBusPeerTrait for ServerSpinDescentManeuverEntity {
}

pub static SERVERSPINDESCENTMANEUVERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSpinDescentManeuverEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSpinDescentManeuverEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSPINDESCENTMANEUVERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSpinDescentManeuverEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSPINDESCENTMANEUVERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERSPINDESCENTMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSpinDescentManeuverEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerSpinDescentManeuverEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerSideToSideManeuverEntity {
    pub _glacier_base: ServerDogFightManeuverEntityBase,
}

pub trait ServerSideToSideManeuverEntityTrait: ServerDogFightManeuverEntityBaseTrait {
}

impl ServerSideToSideManeuverEntityTrait for ServerSideToSideManeuverEntity {
}

impl ServerDogFightManeuverEntityBaseTrait for ServerSideToSideManeuverEntity {
}

impl super::entity::EntityTrait for ServerSideToSideManeuverEntity {
}

impl super::entity::EntityBusPeerTrait for ServerSideToSideManeuverEntity {
}

pub static SERVERSIDETOSIDEMANEUVERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSideToSideManeuverEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSideToSideManeuverEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSIDETOSIDEMANEUVERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSideToSideManeuverEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSIDETOSIDEMANEUVERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERSIDETOSIDEMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSideToSideManeuverEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerSideToSideManeuverEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerSetWaypointsEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerSetWaypointsEntityTrait: super::entity::EntityTrait {
}

impl ServerSetWaypointsEntityTrait for ServerSetWaypointsEntity {
}

impl super::entity::EntityTrait for ServerSetWaypointsEntity {
}

impl super::entity::EntityBusPeerTrait for ServerSetWaypointsEntity {
}

pub static SERVERSETWAYPOINTSENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSetWaypointsEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSetWaypointsEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSETWAYPOINTSENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSetWaypointsEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSETWAYPOINTSENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERSETWAYPOINTSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSetWaypointsEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerSetWaypointsEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerProtectBaseManeuverEntity {
    pub _glacier_base: ServerDogFightManeuverEntityBase,
}

pub trait ServerProtectBaseManeuverEntityTrait: ServerDogFightManeuverEntityBaseTrait {
}

impl ServerProtectBaseManeuverEntityTrait for ServerProtectBaseManeuverEntity {
}

impl ServerDogFightManeuverEntityBaseTrait for ServerProtectBaseManeuverEntity {
}

impl super::entity::EntityTrait for ServerProtectBaseManeuverEntity {
}

impl super::entity::EntityBusPeerTrait for ServerProtectBaseManeuverEntity {
}

pub static SERVERPROTECTBASEMANEUVERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerProtectBaseManeuverEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerProtectBaseManeuverEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERPROTECTBASEMANEUVERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerProtectBaseManeuverEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPROTECTBASEMANEUVERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERPROTECTBASEMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerProtectBaseManeuverEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerProtectBaseManeuverEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerPilotEntityBase {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerPilotEntityBaseTrait: super::entity::EntityTrait {
}

impl ServerPilotEntityBaseTrait for ServerPilotEntityBase {
}

impl super::entity::EntityTrait for ServerPilotEntityBase {
}

impl super::entity::EntityBusPeerTrait for ServerPilotEntityBase {
}

pub static SERVERPILOTENTITYBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPilotEntityBase",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPilotEntityBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERPILOTENTITYBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPilotEntityBase {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPILOTENTITYBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERPILOTENTITYBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPilotEntityBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerPilotEntityBase"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerPilotEntity {
    pub _glacier_base: ServerPilotEntityBase,
}

pub trait ServerPilotEntityTrait: ServerPilotEntityBaseTrait {
}

impl ServerPilotEntityTrait for ServerPilotEntity {
}

impl ServerPilotEntityBaseTrait for ServerPilotEntity {
}

impl super::entity::EntityTrait for ServerPilotEntity {
}

impl super::entity::EntityBusPeerTrait for ServerPilotEntity {
}

pub static SERVERPILOTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPilotEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERPILOTENTITYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPilotEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERPILOTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPilotEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPILOTENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERPILOTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPilotEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerPilotEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerManeuverSelectorEntity {
    pub _glacier_base: ServerDogFightManeuverEntityBase,
}

pub trait ServerManeuverSelectorEntityTrait: ServerDogFightManeuverEntityBaseTrait {
}

impl ServerManeuverSelectorEntityTrait for ServerManeuverSelectorEntity {
}

impl ServerDogFightManeuverEntityBaseTrait for ServerManeuverSelectorEntity {
}

impl super::entity::EntityTrait for ServerManeuverSelectorEntity {
}

impl super::entity::EntityBusPeerTrait for ServerManeuverSelectorEntity {
}

pub static SERVERMANEUVERSELECTORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerManeuverSelectorEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerManeuverSelectorEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERMANEUVERSELECTORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerManeuverSelectorEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERMANEUVERSELECTORENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERMANEUVERSELECTORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerManeuverSelectorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerManeuverSelectorEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerImmelmannManeuverEntity {
    pub _glacier_base: ServerDogFightManeuverEntityBase,
}

pub trait ServerImmelmannManeuverEntityTrait: ServerDogFightManeuverEntityBaseTrait {
}

impl ServerImmelmannManeuverEntityTrait for ServerImmelmannManeuverEntity {
}

impl ServerDogFightManeuverEntityBaseTrait for ServerImmelmannManeuverEntity {
}

impl super::entity::EntityTrait for ServerImmelmannManeuverEntity {
}

impl super::entity::EntityBusPeerTrait for ServerImmelmannManeuverEntity {
}

pub static SERVERIMMELMANNMANEUVERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerImmelmannManeuverEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerImmelmannManeuverEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERIMMELMANNMANEUVERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerImmelmannManeuverEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERIMMELMANNMANEUVERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERIMMELMANNMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerImmelmannManeuverEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerImmelmannManeuverEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerHeavyPlanePilotEntity {
    pub _glacier_base: ServerPilotEntity,
}

pub trait ServerHeavyPlanePilotEntityTrait: ServerPilotEntityTrait {
}

impl ServerHeavyPlanePilotEntityTrait for ServerHeavyPlanePilotEntity {
}

impl ServerPilotEntityTrait for ServerHeavyPlanePilotEntity {
}

impl ServerPilotEntityBaseTrait for ServerHeavyPlanePilotEntity {
}

impl super::entity::EntityTrait for ServerHeavyPlanePilotEntity {
}

impl super::entity::EntityBusPeerTrait for ServerHeavyPlanePilotEntity {
}

pub static SERVERHEAVYPLANEPILOTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerHeavyPlanePilotEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERPILOTENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerHeavyPlanePilotEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERHEAVYPLANEPILOTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerHeavyPlanePilotEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERHEAVYPLANEPILOTENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERHEAVYPLANEPILOTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerHeavyPlanePilotEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerHeavyPlanePilotEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerFollowWaypointsManeuverEntity {
    pub _glacier_base: ServerDogFightManeuverEntityBase,
}

pub trait ServerFollowWaypointsManeuverEntityTrait: ServerDogFightManeuverEntityBaseTrait {
}

impl ServerFollowWaypointsManeuverEntityTrait for ServerFollowWaypointsManeuverEntity {
}

impl ServerDogFightManeuverEntityBaseTrait for ServerFollowWaypointsManeuverEntity {
}

impl super::entity::EntityTrait for ServerFollowWaypointsManeuverEntity {
}

impl super::entity::EntityBusPeerTrait for ServerFollowWaypointsManeuverEntity {
}

pub static SERVERFOLLOWWAYPOINTSMANEUVERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerFollowWaypointsManeuverEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerFollowWaypointsManeuverEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERFOLLOWWAYPOINTSMANEUVERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerFollowWaypointsManeuverEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERFOLLOWWAYPOINTSMANEUVERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERFOLLOWWAYPOINTSMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerFollowWaypointsManeuverEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerFollowWaypointsManeuverEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerFlyToManeuverEntity {
    pub _glacier_base: ServerDogFightManeuverEntityBase,
}

pub trait ServerFlyToManeuverEntityTrait: ServerDogFightManeuverEntityBaseTrait {
}

impl ServerFlyToManeuverEntityTrait for ServerFlyToManeuverEntity {
}

impl ServerDogFightManeuverEntityBaseTrait for ServerFlyToManeuverEntity {
}

impl super::entity::EntityTrait for ServerFlyToManeuverEntity {
}

impl super::entity::EntityBusPeerTrait for ServerFlyToManeuverEntity {
}

pub static SERVERFLYTOMANEUVERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerFlyToManeuverEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerFlyToManeuverEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERFLYTOMANEUVERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerFlyToManeuverEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERFLYTOMANEUVERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERFLYTOMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerFlyToManeuverEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerFlyToManeuverEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerEnforceAltitudeManeuverEntity {
    pub _glacier_base: ServerDogFightManeuverEntityBase,
}

pub trait ServerEnforceAltitudeManeuverEntityTrait: ServerDogFightManeuverEntityBaseTrait {
}

impl ServerEnforceAltitudeManeuverEntityTrait for ServerEnforceAltitudeManeuverEntity {
}

impl ServerDogFightManeuverEntityBaseTrait for ServerEnforceAltitudeManeuverEntity {
}

impl super::entity::EntityTrait for ServerEnforceAltitudeManeuverEntity {
}

impl super::entity::EntityBusPeerTrait for ServerEnforceAltitudeManeuverEntity {
}

pub static SERVERENFORCEALTITUDEMANEUVERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEnforceAltitudeManeuverEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerEnforceAltitudeManeuverEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERENFORCEALTITUDEMANEUVERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerEnforceAltitudeManeuverEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERENFORCEALTITUDEMANEUVERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERENFORCEALTITUDEMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEnforceAltitudeManeuverEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerEnforceAltitudeManeuverEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerDogFightManeuverEntityBase {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerDogFightManeuverEntityBaseTrait: super::entity::EntityTrait {
}

impl ServerDogFightManeuverEntityBaseTrait for ServerDogFightManeuverEntityBase {
}

impl super::entity::EntityTrait for ServerDogFightManeuverEntityBase {
}

impl super::entity::EntityBusPeerTrait for ServerDogFightManeuverEntityBase {
}

pub static SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDogFightManeuverEntityBase",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerDogFightManeuverEntityBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerDogFightManeuverEntityBase {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERDOGFIGHTMANEUVERENTITYBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDogFightManeuverEntityBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerDogFightManeuverEntityBase"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerDogFightingEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerDogFightingEntityTrait: super::entity::EntityTrait {
}

impl ServerDogFightingEntityTrait for ServerDogFightingEntity {
}

impl super::entity::EntityTrait for ServerDogFightingEntity {
}

impl super::entity::EntityBusPeerTrait for ServerDogFightingEntity {
}

pub static SERVERDOGFIGHTINGENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDogFightingEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerDogFightingEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERDOGFIGHTINGENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerDogFightingEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERDOGFIGHTINGENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERDOGFIGHTINGENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDogFightingEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerDogFightingEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerDefensiveManeuverSelectorEntity {
    pub _glacier_base: ServerManeuverSelectorEntity,
}

pub trait ServerDefensiveManeuverSelectorEntityTrait: ServerManeuverSelectorEntityTrait {
}

impl ServerDefensiveManeuverSelectorEntityTrait for ServerDefensiveManeuverSelectorEntity {
}

impl ServerManeuverSelectorEntityTrait for ServerDefensiveManeuverSelectorEntity {
}

impl ServerDogFightManeuverEntityBaseTrait for ServerDefensiveManeuverSelectorEntity {
}

impl super::entity::EntityTrait for ServerDefensiveManeuverSelectorEntity {
}

impl super::entity::EntityBusPeerTrait for ServerDefensiveManeuverSelectorEntity {
}

pub static SERVERDEFENSIVEMANEUVERSELECTORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDefensiveManeuverSelectorEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERMANEUVERSELECTORENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerDefensiveManeuverSelectorEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERDEFENSIVEMANEUVERSELECTORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerDefensiveManeuverSelectorEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERDEFENSIVEMANEUVERSELECTORENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERDEFENSIVEMANEUVERSELECTORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDefensiveManeuverSelectorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerDefensiveManeuverSelectorEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerCreateDistanceManeuverEntity {
    pub _glacier_base: ServerDogFightManeuverEntityBase,
}

pub trait ServerCreateDistanceManeuverEntityTrait: ServerDogFightManeuverEntityBaseTrait {
}

impl ServerCreateDistanceManeuverEntityTrait for ServerCreateDistanceManeuverEntity {
}

impl ServerDogFightManeuverEntityBaseTrait for ServerCreateDistanceManeuverEntity {
}

impl super::entity::EntityTrait for ServerCreateDistanceManeuverEntity {
}

impl super::entity::EntityBusPeerTrait for ServerCreateDistanceManeuverEntity {
}

pub static SERVERCREATEDISTANCEMANEUVERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreateDistanceManeuverEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCreateDistanceManeuverEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCREATEDISTANCEMANEUVERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCreateDistanceManeuverEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCREATEDISTANCEMANEUVERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERCREATEDISTANCEMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreateDistanceManeuverEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerCreateDistanceManeuverEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerCollisionAvoidanceManeuverEntity {
    pub _glacier_base: ServerDogFightManeuverEntityBase,
}

pub trait ServerCollisionAvoidanceManeuverEntityTrait: ServerDogFightManeuverEntityBaseTrait {
}

impl ServerCollisionAvoidanceManeuverEntityTrait for ServerCollisionAvoidanceManeuverEntity {
}

impl ServerDogFightManeuverEntityBaseTrait for ServerCollisionAvoidanceManeuverEntity {
}

impl super::entity::EntityTrait for ServerCollisionAvoidanceManeuverEntity {
}

impl super::entity::EntityBusPeerTrait for ServerCollisionAvoidanceManeuverEntity {
}

pub static SERVERCOLLISIONAVOIDANCEMANEUVERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCollisionAvoidanceManeuverEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCollisionAvoidanceManeuverEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCOLLISIONAVOIDANCEMANEUVERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCollisionAvoidanceManeuverEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCOLLISIONAVOIDANCEMANEUVERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERCOLLISIONAVOIDANCEMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCollisionAvoidanceManeuverEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerCollisionAvoidanceManeuverEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerBasicDefensiveManeuverEntity {
    pub _glacier_base: ServerDogFightManeuverEntityBase,
}

pub trait ServerBasicDefensiveManeuverEntityTrait: ServerDogFightManeuverEntityBaseTrait {
}

impl ServerBasicDefensiveManeuverEntityTrait for ServerBasicDefensiveManeuverEntity {
}

impl ServerDogFightManeuverEntityBaseTrait for ServerBasicDefensiveManeuverEntity {
}

impl super::entity::EntityTrait for ServerBasicDefensiveManeuverEntity {
}

impl super::entity::EntityBusPeerTrait for ServerBasicDefensiveManeuverEntity {
}

pub static SERVERBASICDEFENSIVEMANEUVERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBasicDefensiveManeuverEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerBasicDefensiveManeuverEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERBASICDEFENSIVEMANEUVERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerBasicDefensiveManeuverEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERBASICDEFENSIVEMANEUVERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERBASICDEFENSIVEMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBasicDefensiveManeuverEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerBasicDefensiveManeuverEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerBasicAttackManeuverEntity {
    pub _glacier_base: ServerDogFightManeuverEntityBase,
}

pub trait ServerBasicAttackManeuverEntityTrait: ServerDogFightManeuverEntityBaseTrait {
}

impl ServerBasicAttackManeuverEntityTrait for ServerBasicAttackManeuverEntity {
}

impl ServerDogFightManeuverEntityBaseTrait for ServerBasicAttackManeuverEntity {
}

impl super::entity::EntityTrait for ServerBasicAttackManeuverEntity {
}

impl super::entity::EntityBusPeerTrait for ServerBasicAttackManeuverEntity {
}

pub static SERVERBASICATTACKMANEUVERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBasicAttackManeuverEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerBasicAttackManeuverEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERBASICATTACKMANEUVERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerBasicAttackManeuverEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERBASICATTACKMANEUVERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERBASICATTACKMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBasicAttackManeuverEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerBasicAttackManeuverEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerBarrelRollManeuverEntity {
    pub _glacier_base: ServerDogFightManeuverEntityBase,
}

pub trait ServerBarrelRollManeuverEntityTrait: ServerDogFightManeuverEntityBaseTrait {
}

impl ServerBarrelRollManeuverEntityTrait for ServerBarrelRollManeuverEntity {
}

impl ServerDogFightManeuverEntityBaseTrait for ServerBarrelRollManeuverEntity {
}

impl super::entity::EntityTrait for ServerBarrelRollManeuverEntity {
}

impl super::entity::EntityBusPeerTrait for ServerBarrelRollManeuverEntity {
}

pub static SERVERBARRELROLLMANEUVERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBarrelRollManeuverEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerBarrelRollManeuverEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERBARRELROLLMANEUVERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerBarrelRollManeuverEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERBARRELROLLMANEUVERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERBARRELROLLMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBarrelRollManeuverEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerBarrelRollManeuverEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAirTargetSelectorEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerAirTargetSelectorEntityTrait: super::entity::EntityTrait {
}

impl ServerAirTargetSelectorEntityTrait for ServerAirTargetSelectorEntity {
}

impl super::entity::EntityTrait for ServerAirTargetSelectorEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAirTargetSelectorEntity {
}

pub static SERVERAIRTARGETSELECTORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAirTargetSelectorEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAirTargetSelectorEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIRTARGETSELECTORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAirTargetSelectorEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAIRTARGETSELECTORENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAIRTARGETSELECTORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAirTargetSelectorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAirTargetSelectorEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAirCollisionAvoidanceEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerAirCollisionAvoidanceEntityTrait: super::entity::EntityTrait {
}

impl ServerAirCollisionAvoidanceEntityTrait for ServerAirCollisionAvoidanceEntity {
}

impl super::entity::EntityTrait for ServerAirCollisionAvoidanceEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAirCollisionAvoidanceEntity {
}

pub static SERVERAIRCOLLISIONAVOIDANCEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAirCollisionAvoidanceEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAirCollisionAvoidanceEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIRCOLLISIONAVOIDANCEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAirCollisionAvoidanceEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAIRCOLLISIONAVOIDANCEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAIRCOLLISIONAVOIDANCEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAirCollisionAvoidanceEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAirCollisionAvoidanceEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BFServerDogFightingEntity {
    pub _glacier_base: ServerDogFightingEntity,
}

pub trait BFServerDogFightingEntityTrait: ServerDogFightingEntityTrait {
}

impl BFServerDogFightingEntityTrait for BFServerDogFightingEntity {
}

impl ServerDogFightingEntityTrait for BFServerDogFightingEntity {
}

impl super::entity::EntityTrait for BFServerDogFightingEntity {
}

impl super::entity::EntityBusPeerTrait for BFServerDogFightingEntity {
}

pub static BFSERVERDOGFIGHTINGENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BFServerDogFightingEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTINGENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BFServerDogFightingEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(BFSERVERDOGFIGHTINGENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BFServerDogFightingEntity {
    fn type_info(&self) -> &'static TypeInfo {
        BFSERVERDOGFIGHTINGENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static BFSERVERDOGFIGHTINGENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BFServerDogFightingEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("BFServerDogFightingEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerWaypointsWalkerEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerWaypointsWalkerEntityTrait: super::entity::EntityTrait {
}

impl ServerWaypointsWalkerEntityTrait for ServerWaypointsWalkerEntity {
}

impl super::entity::EntityTrait for ServerWaypointsWalkerEntity {
}

impl super::entity::EntityBusPeerTrait for ServerWaypointsWalkerEntity {
}

pub static SERVERWAYPOINTSWALKERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWaypointsWalkerEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerWaypointsWalkerEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERWAYPOINTSWALKERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerWaypointsWalkerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERWAYPOINTSWALKERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERWAYPOINTSWALKERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWaypointsWalkerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerWaypointsWalkerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAIProximityReactionsComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerAIProximityReactionsComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerAIProximityReactionsComponentTrait for ServerAIProximityReactionsComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerAIProximityReactionsComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerAIProximityReactionsComponent {
}

impl super::entity::ComponentTrait for ServerAIProximityReactionsComponent {
}

impl super::entity::EntityBusPeerTrait for ServerAIProximityReactionsComponent {
}

pub static SERVERAIPROXIMITYREACTIONSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIProximityReactionsComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIProximityReactionsComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIPROXIMITYREACTIONSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIProximityReactionsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAIPROXIMITYREACTIONSCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAIPROXIMITYREACTIONSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIProximityReactionsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIProximityReactionsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAILocoComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerAILocoComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerAILocoComponentTrait for ServerAILocoComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerAILocoComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerAILocoComponent {
}

impl super::entity::ComponentTrait for ServerAILocoComponent {
}

impl super::entity::EntityBusPeerTrait for ServerAILocoComponent {
}

pub static SERVERAILOCOCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAILocoComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAILocoComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAILOCOCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAILocoComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAILOCOCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAILOCOCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAILocoComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAILocoComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AIBlockerEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait AIBlockerEntityTrait: super::entity::EntityTrait {
}

impl AIBlockerEntityTrait for AIBlockerEntity {
}

impl super::entity::EntityTrait for AIBlockerEntity {
}

impl super::entity::EntityBusPeerTrait for AIBlockerEntity {
}

pub static AIBLOCKERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIBlockerEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AIBlockerEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(AIBLOCKERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AIBlockerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        AIBLOCKERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static AIBLOCKERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIBlockerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AIBlockerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientAIProximityReactionsComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientAIProximityReactionsComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientAIProximityReactionsComponentTrait for ClientAIProximityReactionsComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientAIProximityReactionsComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientAIProximityReactionsComponent {
}

impl super::entity::ComponentTrait for ClientAIProximityReactionsComponent {
}

impl super::entity::EntityBusPeerTrait for ClientAIProximityReactionsComponent {
}

pub static CLIENTAIPROXIMITYREACTIONSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAIProximityReactionsComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAIProximityReactionsComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTAIPROXIMITYREACTIONSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAIProximityReactionsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTAIPROXIMITYREACTIONSCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTAIPROXIMITYREACTIONSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAIProximityReactionsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ClientAIProximityReactionsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientAILocoComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientAILocoComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientAILocoComponentTrait for ClientAILocoComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientAILocoComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientAILocoComponent {
}

impl super::entity::ComponentTrait for ClientAILocoComponent {
}

impl super::entity::EntityBusPeerTrait for ClientAILocoComponent {
}

pub static CLIENTAILOCOCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAILocoComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAILocoComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTAILOCOCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAILocoComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTAILOCOCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTAILOCOCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAILocoComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ClientAILocoComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SpatialAnalyzer {
    pub _glacier_base: super::entity::Entity,
}

pub trait SpatialAnalyzerTrait: super::entity::EntityTrait {
}

impl SpatialAnalyzerTrait for SpatialAnalyzer {
}

impl super::entity::EntityTrait for SpatialAnalyzer {
}

impl super::entity::EntityBusPeerTrait for SpatialAnalyzer {
}

pub static SPATIALANALYZER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpatialAnalyzer",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SpatialAnalyzer as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SPATIALANALYZER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SpatialAnalyzer {
    fn type_info(&self) -> &'static TypeInfo {
        SPATIALANALYZER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SPATIALANALYZER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpatialAnalyzer-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("SpatialAnalyzer"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerPlayerVehicleProximityEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerPlayerVehicleProximityEntityTrait: super::entity::EntityTrait {
}

impl ServerPlayerVehicleProximityEntityTrait for ServerPlayerVehicleProximityEntity {
}

impl super::entity::EntityTrait for ServerPlayerVehicleProximityEntity {
}

impl super::entity::EntityBusPeerTrait for ServerPlayerVehicleProximityEntity {
}

pub static SERVERPLAYERVEHICLEPROXIMITYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerVehicleProximityEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPlayerVehicleProximityEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERPLAYERVEHICLEPROXIMITYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPlayerVehicleProximityEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPLAYERVEHICLEPROXIMITYENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERPLAYERVEHICLEPROXIMITYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerVehicleProximityEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerPlayerVehicleProximityEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerInvestigateSettingsOverride {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerInvestigateSettingsOverrideTrait: super::entity::EntityTrait {
}

impl ServerInvestigateSettingsOverrideTrait for ServerInvestigateSettingsOverride {
}

impl super::entity::EntityTrait for ServerInvestigateSettingsOverride {
}

impl super::entity::EntityBusPeerTrait for ServerInvestigateSettingsOverride {
}

pub static SERVERINVESTIGATESETTINGSOVERRIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerInvestigateSettingsOverride",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerInvestigateSettingsOverride as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERINVESTIGATESETTINGSOVERRIDE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerInvestigateSettingsOverride {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERINVESTIGATESETTINGSOVERRIDE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERINVESTIGATESETTINGSOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerInvestigateSettingsOverride-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerInvestigateSettingsOverride"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerDamageModifierEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerDamageModifierEntityTrait: super::entity::EntityTrait {
}

impl ServerDamageModifierEntityTrait for ServerDamageModifierEntity {
}

impl super::entity::EntityTrait for ServerDamageModifierEntity {
}

impl super::entity::EntityBusPeerTrait for ServerDamageModifierEntity {
}

pub static SERVERDAMAGEMODIFIERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDamageModifierEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerDamageModifierEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERDAMAGEMODIFIERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerDamageModifierEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERDAMAGEMODIFIERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERDAMAGEMODIFIERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDamageModifierEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerDamageModifierEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAttackPointEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerAttackPointEntityTrait: super::entity::EntityTrait {
}

impl ServerAttackPointEntityTrait for ServerAttackPointEntity {
}

impl super::entity::EntityTrait for ServerAttackPointEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAttackPointEntity {
}

pub static SERVERATTACKPOINTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAttackPointEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAttackPointEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERATTACKPOINTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAttackPointEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERATTACKPOINTENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERATTACKPOINTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAttackPointEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAttackPointEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAIVehicleCombatEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerAIVehicleCombatEntityTrait: super::entity::EntityTrait {
}

impl ServerAIVehicleCombatEntityTrait for ServerAIVehicleCombatEntity {
}

impl super::entity::EntityTrait for ServerAIVehicleCombatEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAIVehicleCombatEntity {
}

pub static SERVERAIVEHICLECOMBATENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIVehicleCombatEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIVehicleCombatEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIVEHICLECOMBATENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIVehicleCombatEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAIVEHICLECOMBATENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAIVEHICLECOMBATENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIVehicleCombatEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIVehicleCombatEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAITemplateFilterEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerAITemplateFilterEntityTrait: super::entity::EntityTrait {
}

impl ServerAITemplateFilterEntityTrait for ServerAITemplateFilterEntity {
}

impl super::entity::EntityTrait for ServerAITemplateFilterEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAITemplateFilterEntity {
}

pub static SERVERAITEMPLATEFILTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAITemplateFilterEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAITemplateFilterEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAITEMPLATEFILTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAITemplateFilterEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAITEMPLATEFILTERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAITEMPLATEFILTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAITemplateFilterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAITemplateFilterEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAITeleportEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerAITeleportEntityTrait: super::entity::EntityTrait {
}

impl ServerAITeleportEntityTrait for ServerAITeleportEntity {
}

impl super::entity::EntityTrait for ServerAITeleportEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAITeleportEntity {
}

pub static SERVERAITELEPORTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAITeleportEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAITeleportEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAITELEPORTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAITeleportEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAITELEPORTENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAITELEPORTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAITeleportEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAITeleportEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAISystemEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerAISystemEntityTrait: super::entity::EntityTrait {
}

impl ServerAISystemEntityTrait for ServerAISystemEntity {
}

impl super::entity::EntityTrait for ServerAISystemEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAISystemEntity {
}

pub static SERVERAISYSTEMENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISystemEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAISystemEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAISYSTEMENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAISystemEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAISYSTEMENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAISYSTEMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISystemEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAISystemEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAIStateEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerAIStateEntityTrait: super::entity::EntityTrait {
}

impl ServerAIStateEntityTrait for ServerAIStateEntity {
}

impl super::entity::EntityTrait for ServerAIStateEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAIStateEntity {
}

pub static SERVERAISTATEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIStateEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIStateEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAISTATEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIStateEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAISTATEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAISTATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIStateEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIStateEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAISoundEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerAISoundEntityTrait: super::entity::EntityTrait {
}

impl ServerAISoundEntityTrait for ServerAISoundEntity {
}

impl super::entity::EntityTrait for ServerAISoundEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAISoundEntity {
}

pub static SERVERAISOUNDENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISoundEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAISoundEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAISOUNDENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAISoundEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAISOUNDENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAISOUNDENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISoundEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAISoundEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAICancelOrder {
    pub _glacier_base: ServerAIOrderEntityBase,
}

pub trait ServerAICancelOrderTrait: ServerAIOrderEntityBaseTrait {
}

impl ServerAICancelOrderTrait for ServerAICancelOrder {
}

impl ServerAIOrderEntityBaseTrait for ServerAICancelOrder {
}

impl super::entity::EntityTrait for ServerAICancelOrder {
}

impl super::entity::EntityBusPeerTrait for ServerAICancelOrder {
}

pub static SERVERAICANCELORDER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAICancelOrder",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIORDERENTITYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAICancelOrder as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAICANCELORDER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAICancelOrder {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAICANCELORDER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAICANCELORDER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAICancelOrder-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAICancelOrder"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAIGotoPlaceOrderEntityData {
    pub _glacier_base: ServerAIOrderEntityBase,
}

pub trait ServerAIGotoPlaceOrderEntityDataTrait: ServerAIOrderEntityBaseTrait {
}

impl ServerAIGotoPlaceOrderEntityDataTrait for ServerAIGotoPlaceOrderEntityData {
}

impl ServerAIOrderEntityBaseTrait for ServerAIGotoPlaceOrderEntityData {
}

impl super::entity::EntityTrait for ServerAIGotoPlaceOrderEntityData {
}

impl super::entity::EntityBusPeerTrait for ServerAIGotoPlaceOrderEntityData {
}

pub static SERVERAIGOTOPLACEORDERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIGotoPlaceOrderEntityData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIORDERENTITYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIGotoPlaceOrderEntityData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIGOTOPLACEORDERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIGotoPlaceOrderEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAIGOTOPLACEORDERENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAIGOTOPLACEORDERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIGotoPlaceOrderEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIGotoPlaceOrderEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAIFollowWaypointsOrder {
    pub _glacier_base: ServerAIOrderEntityBase,
}

pub trait ServerAIFollowWaypointsOrderTrait: ServerAIOrderEntityBaseTrait {
}

impl ServerAIFollowWaypointsOrderTrait for ServerAIFollowWaypointsOrder {
}

impl ServerAIOrderEntityBaseTrait for ServerAIFollowWaypointsOrder {
}

impl super::entity::EntityTrait for ServerAIFollowWaypointsOrder {
}

impl super::entity::EntityBusPeerTrait for ServerAIFollowWaypointsOrder {
}

pub static SERVERAIFOLLOWWAYPOINTSORDER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIFollowWaypointsOrder",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIORDERENTITYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIFollowWaypointsOrder as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIFOLLOWWAYPOINTSORDER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIFollowWaypointsOrder {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAIFOLLOWWAYPOINTSORDER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAIFOLLOWWAYPOINTSORDER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIFollowWaypointsOrder-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIFollowWaypointsOrder"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAIOrderEntityBase {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerAIOrderEntityBaseTrait: super::entity::EntityTrait {
}

impl ServerAIOrderEntityBaseTrait for ServerAIOrderEntityBase {
}

impl super::entity::EntityTrait for ServerAIOrderEntityBase {
}

impl super::entity::EntityBusPeerTrait for ServerAIOrderEntityBase {
}

pub static SERVERAIORDERENTITYBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIOrderEntityBase",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIOrderEntityBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIORDERENTITYBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIOrderEntityBase {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAIORDERENTITYBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAIORDERENTITYBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIOrderEntityBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIOrderEntityBase"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAISelfDestructEntity {
    pub _glacier_base: ServerAIParameterEntity,
}

pub trait ServerAISelfDestructEntityTrait: ServerAIParameterEntityTrait {
}

impl ServerAISelfDestructEntityTrait for ServerAISelfDestructEntity {
}

impl ServerAIParameterEntityTrait for ServerAISelfDestructEntity {
}

impl super::entity::EntityTrait for ServerAISelfDestructEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAISelfDestructEntity {
}

pub static SERVERAISELFDESTRUCTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISelfDestructEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAISelfDestructEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAISELFDESTRUCTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAISelfDestructEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAISELFDESTRUCTENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAISELFDESTRUCTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISelfDestructEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAISelfDestructEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAICoverZonesOverrideEntity {
    pub _glacier_base: ServerAIParameterEntity,
}

pub trait ServerAICoverZonesOverrideEntityTrait: ServerAIParameterEntityTrait {
}

impl ServerAICoverZonesOverrideEntityTrait for ServerAICoverZonesOverrideEntity {
}

impl ServerAIParameterEntityTrait for ServerAICoverZonesOverrideEntity {
}

impl super::entity::EntityTrait for ServerAICoverZonesOverrideEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAICoverZonesOverrideEntity {
}

pub static SERVERAICOVERZONESOVERRIDEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAICoverZonesOverrideEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAICoverZonesOverrideEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAICOVERZONESOVERRIDEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAICoverZonesOverrideEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAICOVERZONESOVERRIDEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAICOVERZONESOVERRIDEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAICoverZonesOverrideEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAICoverZonesOverrideEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAIAwarenessEntity {
    pub _glacier_base: ServerAIParameterEntity,
}

pub trait ServerAIAwarenessEntityTrait: ServerAIParameterEntityTrait {
}

impl ServerAIAwarenessEntityTrait for ServerAIAwarenessEntity {
}

impl ServerAIParameterEntityTrait for ServerAIAwarenessEntity {
}

impl super::entity::EntityTrait for ServerAIAwarenessEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAIAwarenessEntity {
}

pub static SERVERAIAWARENESSENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIAwarenessEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIAwarenessEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIAWARENESSENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIAwarenessEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAIAWARENESSENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAIAWARENESSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIAwarenessEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIAwarenessEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAIWeaponSlotOverrideEntity {
    pub _glacier_base: ServerAIParameterEntity,
}

pub trait ServerAIWeaponSlotOverrideEntityTrait: ServerAIParameterEntityTrait {
}

impl ServerAIWeaponSlotOverrideEntityTrait for ServerAIWeaponSlotOverrideEntity {
}

impl ServerAIParameterEntityTrait for ServerAIWeaponSlotOverrideEntity {
}

impl super::entity::EntityTrait for ServerAIWeaponSlotOverrideEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAIWeaponSlotOverrideEntity {
}

pub static SERVERAIWEAPONSLOTOVERRIDEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIWeaponSlotOverrideEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIWeaponSlotOverrideEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIWEAPONSLOTOVERRIDEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIWeaponSlotOverrideEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAIWEAPONSLOTOVERRIDEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAIWEAPONSLOTOVERRIDEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIWeaponSlotOverrideEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIWeaponSlotOverrideEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAITargetCoordinatorFilterEntity {
    pub _glacier_base: ServerAIParameterEntity,
}

pub trait ServerAITargetCoordinatorFilterEntityTrait: ServerAIParameterEntityTrait {
}

impl ServerAITargetCoordinatorFilterEntityTrait for ServerAITargetCoordinatorFilterEntity {
}

impl ServerAIParameterEntityTrait for ServerAITargetCoordinatorFilterEntity {
}

impl super::entity::EntityTrait for ServerAITargetCoordinatorFilterEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAITargetCoordinatorFilterEntity {
}

pub static SERVERAITARGETCOORDINATORFILTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAITargetCoordinatorFilterEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAITargetCoordinatorFilterEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAITARGETCOORDINATORFILTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAITargetCoordinatorFilterEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAITARGETCOORDINATORFILTERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAITARGETCOORDINATORFILTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAITargetCoordinatorFilterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAITargetCoordinatorFilterEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAITargetCoordinatorEntity {
    pub _glacier_base: ServerAIParameterEntity,
}

pub trait ServerAITargetCoordinatorEntityTrait: ServerAIParameterEntityTrait {
}

impl ServerAITargetCoordinatorEntityTrait for ServerAITargetCoordinatorEntity {
}

impl ServerAIParameterEntityTrait for ServerAITargetCoordinatorEntity {
}

impl super::entity::EntityTrait for ServerAITargetCoordinatorEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAITargetCoordinatorEntity {
}

pub static SERVERAITARGETCOORDINATORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAITargetCoordinatorEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAITargetCoordinatorEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAITARGETCOORDINATORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAITargetCoordinatorEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAITARGETCOORDINATORENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAITARGETCOORDINATORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAITargetCoordinatorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAITargetCoordinatorEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerCloakingModifierEntity {
    pub _glacier_base: ServerAIParameterEntity,
}

pub trait ServerCloakingModifierEntityTrait: ServerAIParameterEntityTrait {
}

impl ServerCloakingModifierEntityTrait for ServerCloakingModifierEntity {
}

impl ServerAIParameterEntityTrait for ServerCloakingModifierEntity {
}

impl super::entity::EntityTrait for ServerCloakingModifierEntity {
}

impl super::entity::EntityBusPeerTrait for ServerCloakingModifierEntity {
}

pub static SERVERCLOAKINGMODIFIERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCloakingModifierEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCloakingModifierEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCLOAKINGMODIFIERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCloakingModifierEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCLOAKINGMODIFIERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERCLOAKINGMODIFIERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCloakingModifierEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerCloakingModifierEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerSensingAreaModifierEntity {
    pub _glacier_base: ServerAIParameterEntity,
}

pub trait ServerSensingAreaModifierEntityTrait: ServerAIParameterEntityTrait {
}

impl ServerSensingAreaModifierEntityTrait for ServerSensingAreaModifierEntity {
}

impl ServerAIParameterEntityTrait for ServerSensingAreaModifierEntity {
}

impl super::entity::EntityTrait for ServerSensingAreaModifierEntity {
}

impl super::entity::EntityBusPeerTrait for ServerSensingAreaModifierEntity {
}

pub static SERVERSENSINGAREAMODIFIERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSensingAreaModifierEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSensingAreaModifierEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSENSINGAREAMODIFIERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSensingAreaModifierEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSENSINGAREAMODIFIERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERSENSINGAREAMODIFIERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSensingAreaModifierEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerSensingAreaModifierEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAIStealthEntity {
    pub _glacier_base: ServerAIParameterEntity,
}

pub trait ServerAIStealthEntityTrait: ServerAIParameterEntityTrait {
}

impl ServerAIStealthEntityTrait for ServerAIStealthEntity {
}

impl ServerAIParameterEntityTrait for ServerAIStealthEntity {
}

impl super::entity::EntityTrait for ServerAIStealthEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAIStealthEntity {
}

pub static SERVERAISTEALTHENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIStealthEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIStealthEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAISTEALTHENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIStealthEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAISTEALTHENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAISTEALTHENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIStealthEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIStealthEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAIBuddyFollowEntity {
    pub _glacier_base: ServerAIParameterEntity,
}

pub trait ServerAIBuddyFollowEntityTrait: ServerAIParameterEntityTrait {
}

impl ServerAIBuddyFollowEntityTrait for ServerAIBuddyFollowEntity {
}

impl ServerAIParameterEntityTrait for ServerAIBuddyFollowEntity {
}

impl super::entity::EntityTrait for ServerAIBuddyFollowEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAIBuddyFollowEntity {
}

pub static SERVERAIBUDDYFOLLOWENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIBuddyFollowEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIBuddyFollowEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIBUDDYFOLLOWENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIBuddyFollowEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAIBUDDYFOLLOWENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAIBUDDYFOLLOWENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIBuddyFollowEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIBuddyFollowEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAIFollowObjectEntity {
    pub _glacier_base: ServerAIParameterEntity,
}

pub trait ServerAIFollowObjectEntityTrait: ServerAIParameterEntityTrait {
}

impl ServerAIFollowObjectEntityTrait for ServerAIFollowObjectEntity {
}

impl ServerAIParameterEntityTrait for ServerAIFollowObjectEntity {
}

impl super::entity::EntityTrait for ServerAIFollowObjectEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAIFollowObjectEntity {
}

pub static SERVERAIFOLLOWOBJECTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIFollowObjectEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIFollowObjectEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIFOLLOWOBJECTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIFollowObjectEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAIFOLLOWOBJECTENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAIFOLLOWOBJECTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIFollowObjectEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIFollowObjectEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAIPreferredAreaEntity {
    pub _glacier_base: ServerAIParameterWithShapeEntity,
}

pub trait ServerAIPreferredAreaEntityTrait: ServerAIParameterWithShapeEntityTrait {
}

impl ServerAIPreferredAreaEntityTrait for ServerAIPreferredAreaEntity {
}

impl ServerAIParameterWithShapeEntityTrait for ServerAIPreferredAreaEntity {
}

impl ServerAIParameterEntityTrait for ServerAIPreferredAreaEntity {
}

impl super::entity::EntityTrait for ServerAIPreferredAreaEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAIPreferredAreaEntity {
}

pub static SERVERAIPREFERREDAREAENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIPreferredAreaEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERWITHSHAPEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIPreferredAreaEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIPREFERREDAREAENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIPreferredAreaEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAIPREFERREDAREAENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAIPREFERREDAREAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIPreferredAreaEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIPreferredAreaEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAISoundAreaEntity {
    pub _glacier_base: ServerAIParameterWithShapeEntity,
}

pub trait ServerAISoundAreaEntityTrait: ServerAIParameterWithShapeEntityTrait {
}

impl ServerAISoundAreaEntityTrait for ServerAISoundAreaEntity {
}

impl ServerAIParameterWithShapeEntityTrait for ServerAISoundAreaEntity {
}

impl ServerAIParameterEntityTrait for ServerAISoundAreaEntity {
}

impl super::entity::EntityTrait for ServerAISoundAreaEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAISoundAreaEntity {
}

pub static SERVERAISOUNDAREAENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISoundAreaEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERWITHSHAPEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAISoundAreaEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAISOUNDAREAENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAISoundAreaEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAISOUNDAREAENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAISOUNDAREAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISoundAreaEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAISoundAreaEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAICombatGroupEntity {
    pub _glacier_base: ServerAIParameterEntity,
}

pub trait ServerAICombatGroupEntityTrait: ServerAIParameterEntityTrait {
}

impl ServerAICombatGroupEntityTrait for ServerAICombatGroupEntity {
}

impl ServerAIParameterEntityTrait for ServerAICombatGroupEntity {
}

impl super::entity::EntityTrait for ServerAICombatGroupEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAICombatGroupEntity {
}

pub static SERVERAICOMBATGROUPENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAICombatGroupEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAICombatGroupEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAICOMBATGROUPENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAICombatGroupEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAICOMBATGROUPENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAICOMBATGROUPENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAICombatGroupEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAICombatGroupEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAICoverQueryEntity {
    pub _glacier_base: ServerAIParameterEntity,
}

pub trait ServerAICoverQueryEntityTrait: ServerAIParameterEntityTrait {
}

impl ServerAICoverQueryEntityTrait for ServerAICoverQueryEntity {
}

impl ServerAIParameterEntityTrait for ServerAICoverQueryEntity {
}

impl super::entity::EntityTrait for ServerAICoverQueryEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAICoverQueryEntity {
}

pub static SERVERAICOVERQUERYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAICoverQueryEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAICoverQueryEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAICOVERQUERYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAICoverQueryEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAICOVERQUERYENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAICOVERQUERYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAICoverQueryEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAICoverQueryEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAITacticEntity {
    pub _glacier_base: ServerAIParameterEntity,
}

pub trait ServerAITacticEntityTrait: ServerAIParameterEntityTrait {
}

impl ServerAITacticEntityTrait for ServerAITacticEntity {
}

impl ServerAIParameterEntityTrait for ServerAITacticEntity {
}

impl super::entity::EntityTrait for ServerAITacticEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAITacticEntity {
}

pub static SERVERAITACTICENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAITacticEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAITacticEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAITACTICENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAITacticEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAITACTICENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAITACTICENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAITacticEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAITacticEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAIShootAtTargetsEntity {
    pub _glacier_base: ServerAIParameterEntity,
}

pub trait ServerAIShootAtTargetsEntityTrait: ServerAIParameterEntityTrait {
}

impl ServerAIShootAtTargetsEntityTrait for ServerAIShootAtTargetsEntity {
}

impl ServerAIParameterEntityTrait for ServerAIShootAtTargetsEntity {
}

impl super::entity::EntityTrait for ServerAIShootAtTargetsEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAIShootAtTargetsEntity {
}

pub static SERVERAISHOOTATTARGETSENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIShootAtTargetsEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIShootAtTargetsEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAISHOOTATTARGETSENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIShootAtTargetsEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAISHOOTATTARGETSENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAISHOOTATTARGETSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIShootAtTargetsEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIShootAtTargetsEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAIUseWaypointsEntity {
    pub _glacier_base: ServerAIParameterEntity,
}

pub trait ServerAIUseWaypointsEntityTrait: ServerAIParameterEntityTrait {
}

impl ServerAIUseWaypointsEntityTrait for ServerAIUseWaypointsEntity {
}

impl ServerAIParameterEntityTrait for ServerAIUseWaypointsEntity {
}

impl super::entity::EntityTrait for ServerAIUseWaypointsEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAIUseWaypointsEntity {
}

pub static SERVERAIUSEWAYPOINTSENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIUseWaypointsEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIUseWaypointsEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIUSEWAYPOINTSENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIUseWaypointsEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAIUSEWAYPOINTSENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAIUSEWAYPOINTSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIUseWaypointsEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIUseWaypointsEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAIUseCoverEntity {
    pub _glacier_base: ServerAIParameterEntity,
}

pub trait ServerAIUseCoverEntityTrait: ServerAIParameterEntityTrait {
}

impl ServerAIUseCoverEntityTrait for ServerAIUseCoverEntity {
}

impl ServerAIParameterEntityTrait for ServerAIUseCoverEntity {
}

impl super::entity::EntityTrait for ServerAIUseCoverEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAIUseCoverEntity {
}

pub static SERVERAIUSECOVERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIUseCoverEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIUseCoverEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIUSECOVERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIUseCoverEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAIUSECOVERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAIUSECOVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIUseCoverEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIUseCoverEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAIWeaponOverrideEntity {
    pub _glacier_base: ServerAIParameterEntity,
}

pub trait ServerAIWeaponOverrideEntityTrait: ServerAIParameterEntityTrait {
}

impl ServerAIWeaponOverrideEntityTrait for ServerAIWeaponOverrideEntity {
}

impl ServerAIParameterEntityTrait for ServerAIWeaponOverrideEntity {
}

impl super::entity::EntityTrait for ServerAIWeaponOverrideEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAIWeaponOverrideEntity {
}

pub static SERVERAIWEAPONOVERRIDEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIWeaponOverrideEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIWeaponOverrideEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIWEAPONOVERRIDEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIWeaponOverrideEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAIWEAPONOVERRIDEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAIWEAPONOVERRIDEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIWeaponOverrideEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIWeaponOverrideEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAIVehicleBehaviorEntity {
    pub _glacier_base: ServerAIParameterEntity,
}

pub trait ServerAIVehicleBehaviorEntityTrait: ServerAIParameterEntityTrait {
}

impl ServerAIVehicleBehaviorEntityTrait for ServerAIVehicleBehaviorEntity {
}

impl ServerAIParameterEntityTrait for ServerAIVehicleBehaviorEntity {
}

impl super::entity::EntityTrait for ServerAIVehicleBehaviorEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAIVehicleBehaviorEntity {
}

pub static SERVERAIVEHICLEBEHAVIORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIVehicleBehaviorEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIVehicleBehaviorEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIVEHICLEBEHAVIORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIVehicleBehaviorEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAIVEHICLEBEHAVIORENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAIVEHICLEBEHAVIORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIVehicleBehaviorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIVehicleBehaviorEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAIHearingParameterEntity {
    pub _glacier_base: ServerAIParameterEntity,
}

pub trait ServerAIHearingParameterEntityTrait: ServerAIParameterEntityTrait {
}

impl ServerAIHearingParameterEntityTrait for ServerAIHearingParameterEntity {
}

impl ServerAIParameterEntityTrait for ServerAIHearingParameterEntity {
}

impl super::entity::EntityTrait for ServerAIHearingParameterEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAIHearingParameterEntity {
}

pub static SERVERAIHEARINGPARAMETERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIHearingParameterEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIHearingParameterEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIHEARINGPARAMETERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIHearingParameterEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAIHEARINGPARAMETERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAIHEARINGPARAMETERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIHearingParameterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIHearingParameterEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAISensingParameterEntity {
    pub _glacier_base: ServerAIParameterEntity,
}

pub trait ServerAISensingParameterEntityTrait: ServerAIParameterEntityTrait {
}

impl ServerAISensingParameterEntityTrait for ServerAISensingParameterEntity {
}

impl ServerAIParameterEntityTrait for ServerAISensingParameterEntity {
}

impl super::entity::EntityTrait for ServerAISensingParameterEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAISensingParameterEntity {
}

pub static SERVERAISENSINGPARAMETERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISensingParameterEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAISensingParameterEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAISENSINGPARAMETERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAISensingParameterEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAISENSINGPARAMETERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAISENSINGPARAMETERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISensingParameterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAISensingParameterEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAIIdleBehaviorEntity {
    pub _glacier_base: ServerAIParameterEntity,
}

pub trait ServerAIIdleBehaviorEntityTrait: ServerAIParameterEntityTrait {
}

impl ServerAIIdleBehaviorEntityTrait for ServerAIIdleBehaviorEntity {
}

impl ServerAIParameterEntityTrait for ServerAIIdleBehaviorEntity {
}

impl super::entity::EntityTrait for ServerAIIdleBehaviorEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAIIdleBehaviorEntity {
}

pub static SERVERAIIDLEBEHAVIORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIIdleBehaviorEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIIdleBehaviorEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIIDLEBEHAVIORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIIdleBehaviorEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAIIDLEBEHAVIORENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAIIDLEBEHAVIORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIIdleBehaviorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIIdleBehaviorEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAITargetingEntity {
    pub _glacier_base: ServerAIParameterEntity,
}

pub trait ServerAITargetingEntityTrait: ServerAIParameterEntityTrait {
}

impl ServerAITargetingEntityTrait for ServerAITargetingEntity {
}

impl ServerAIParameterEntityTrait for ServerAITargetingEntity {
}

impl super::entity::EntityTrait for ServerAITargetingEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAITargetingEntity {
}

pub static SERVERAITARGETINGENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAITargetingEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAITargetingEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAITARGETINGENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAITargetingEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAITARGETINGENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAITARGETINGENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAITargetingEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAITargetingEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAICombatBehaviorEntity {
    pub _glacier_base: ServerAIParameterEntity,
}

pub trait ServerAICombatBehaviorEntityTrait: ServerAIParameterEntityTrait {
}

impl ServerAICombatBehaviorEntityTrait for ServerAICombatBehaviorEntity {
}

impl ServerAIParameterEntityTrait for ServerAICombatBehaviorEntity {
}

impl super::entity::EntityTrait for ServerAICombatBehaviorEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAICombatBehaviorEntity {
}

pub static SERVERAICOMBATBEHAVIORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAICombatBehaviorEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAICombatBehaviorEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAICOMBATBEHAVIORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAICombatBehaviorEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAICOMBATBEHAVIORENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAICOMBATBEHAVIORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAICombatBehaviorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAICombatBehaviorEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAIFollowAreaEntity {
    pub _glacier_base: ServerAIParameterWithShapeEntity,
}

pub trait ServerAIFollowAreaEntityTrait: ServerAIParameterWithShapeEntityTrait {
}

impl ServerAIFollowAreaEntityTrait for ServerAIFollowAreaEntity {
}

impl ServerAIParameterWithShapeEntityTrait for ServerAIFollowAreaEntity {
}

impl ServerAIParameterEntityTrait for ServerAIFollowAreaEntity {
}

impl super::entity::EntityTrait for ServerAIFollowAreaEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAIFollowAreaEntity {
}

pub static SERVERAIFOLLOWAREAENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIFollowAreaEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERWITHSHAPEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIFollowAreaEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIFOLLOWAREAENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIFollowAreaEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAIFOLLOWAREAENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAIFOLLOWAREAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIFollowAreaEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIFollowAreaEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAIForbiddenAreaEntity {
    pub _glacier_base: ServerAIParameterWithShapeEntity,
}

pub trait ServerAIForbiddenAreaEntityTrait: ServerAIParameterWithShapeEntityTrait {
}

impl ServerAIForbiddenAreaEntityTrait for ServerAIForbiddenAreaEntity {
}

impl ServerAIParameterWithShapeEntityTrait for ServerAIForbiddenAreaEntity {
}

impl ServerAIParameterEntityTrait for ServerAIForbiddenAreaEntity {
}

impl super::entity::EntityTrait for ServerAIForbiddenAreaEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAIForbiddenAreaEntity {
}

pub static SERVERAIFORBIDDENAREAENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIForbiddenAreaEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERWITHSHAPEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIForbiddenAreaEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIFORBIDDENAREAENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIForbiddenAreaEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAIFORBIDDENAREAENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAIFORBIDDENAREAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIForbiddenAreaEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIForbiddenAreaEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAIFriendlyAreaEntity {
    pub _glacier_base: ServerAIParameterWithShapeEntity,
}

pub trait ServerAIFriendlyAreaEntityTrait: ServerAIParameterWithShapeEntityTrait {
}

impl ServerAIFriendlyAreaEntityTrait for ServerAIFriendlyAreaEntity {
}

impl ServerAIParameterWithShapeEntityTrait for ServerAIFriendlyAreaEntity {
}

impl ServerAIParameterEntityTrait for ServerAIFriendlyAreaEntity {
}

impl super::entity::EntityTrait for ServerAIFriendlyAreaEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAIFriendlyAreaEntity {
}

pub static SERVERAIFRIENDLYAREAENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIFriendlyAreaEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERWITHSHAPEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIFriendlyAreaEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIFRIENDLYAREAENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIFriendlyAreaEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAIFRIENDLYAREAENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAIFRIENDLYAREAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIFriendlyAreaEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIFriendlyAreaEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAIFlankingCorridorEntity {
    pub _glacier_base: ServerAIParameterWithShapeEntity,
}

pub trait ServerAIFlankingCorridorEntityTrait: ServerAIParameterWithShapeEntityTrait {
}

impl ServerAIFlankingCorridorEntityTrait for ServerAIFlankingCorridorEntity {
}

impl ServerAIParameterWithShapeEntityTrait for ServerAIFlankingCorridorEntity {
}

impl ServerAIParameterEntityTrait for ServerAIFlankingCorridorEntity {
}

impl super::entity::EntityTrait for ServerAIFlankingCorridorEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAIFlankingCorridorEntity {
}

pub static SERVERAIFLANKINGCORRIDORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIFlankingCorridorEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERWITHSHAPEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIFlankingCorridorEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIFLANKINGCORRIDORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIFlankingCorridorEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAIFLANKINGCORRIDORENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAIFLANKINGCORRIDORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIFlankingCorridorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIFlankingCorridorEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAISearchAreaEntity {
    pub _glacier_base: ServerAIParameterWithShapeEntity,
}

pub trait ServerAISearchAreaEntityTrait: ServerAIParameterWithShapeEntityTrait {
}

impl ServerAISearchAreaEntityTrait for ServerAISearchAreaEntity {
}

impl ServerAIParameterWithShapeEntityTrait for ServerAISearchAreaEntity {
}

impl ServerAIParameterEntityTrait for ServerAISearchAreaEntity {
}

impl super::entity::EntityTrait for ServerAISearchAreaEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAISearchAreaEntity {
}

pub static SERVERAISEARCHAREAENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISearchAreaEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERWITHSHAPEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAISearchAreaEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAISEARCHAREAENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAISearchAreaEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAISEARCHAREAENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAISEARCHAREAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISearchAreaEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAISearchAreaEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAIDefendAreaEntity {
    pub _glacier_base: ServerAIParameterWithShapeEntity,
}

pub trait ServerAIDefendAreaEntityTrait: ServerAIParameterWithShapeEntityTrait {
}

impl ServerAIDefendAreaEntityTrait for ServerAIDefendAreaEntity {
}

impl ServerAIParameterWithShapeEntityTrait for ServerAIDefendAreaEntity {
}

impl ServerAIParameterEntityTrait for ServerAIDefendAreaEntity {
}

impl super::entity::EntityTrait for ServerAIDefendAreaEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAIDefendAreaEntity {
}

pub static SERVERAIDEFENDAREAENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIDefendAreaEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERWITHSHAPEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIDefendAreaEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIDEFENDAREAENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIDefendAreaEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAIDEFENDAREAENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAIDEFENDAREAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIDefendAreaEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIDefendAreaEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAIParameterWithShapeEntity {
    pub _glacier_base: ServerAIParameterEntity,
}

pub trait ServerAIParameterWithShapeEntityTrait: ServerAIParameterEntityTrait {
}

impl ServerAIParameterWithShapeEntityTrait for ServerAIParameterWithShapeEntity {
}

impl ServerAIParameterEntityTrait for ServerAIParameterWithShapeEntity {
}

impl super::entity::EntityTrait for ServerAIParameterWithShapeEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAIParameterWithShapeEntity {
}

pub static SERVERAIPARAMETERWITHSHAPEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIParameterWithShapeEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIParameterWithShapeEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIPARAMETERWITHSHAPEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIParameterWithShapeEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAIPARAMETERWITHSHAPEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAIPARAMETERWITHSHAPEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIParameterWithShapeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIParameterWithShapeEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAIParameterEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerAIParameterEntityTrait: super::entity::EntityTrait {
}

impl ServerAIParameterEntityTrait for ServerAIParameterEntity {
}

impl super::entity::EntityTrait for ServerAIParameterEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAIParameterEntity {
}

pub static SERVERAIPARAMETERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIParameterEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIParameterEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIPARAMETERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIParameterEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAIPARAMETERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAIPARAMETERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIParameterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIParameterEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAIObstacleControllerEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerAIObstacleControllerEntityTrait: super::entity::EntityTrait {
}

impl ServerAIObstacleControllerEntityTrait for ServerAIObstacleControllerEntity {
}

impl super::entity::EntityTrait for ServerAIObstacleControllerEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAIObstacleControllerEntity {
}

pub static SERVERAIOBSTACLECONTROLLERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIObstacleControllerEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIObstacleControllerEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIOBSTACLECONTROLLERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIObstacleControllerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAIOBSTACLECONTROLLERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAIOBSTACLECONTROLLERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIObstacleControllerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIObstacleControllerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAIKillCounterEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerAIKillCounterEntityTrait: super::entity::EntityTrait {
}

impl ServerAIKillCounterEntityTrait for ServerAIKillCounterEntity {
}

impl super::entity::EntityTrait for ServerAIKillCounterEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAIKillCounterEntity {
}

pub static SERVERAIKILLCOUNTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIKillCounterEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIKillCounterEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIKILLCOUNTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIKillCounterEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAIKILLCOUNTERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAIKILLCOUNTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIKillCounterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIKillCounterEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAIFirePatternEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerAIFirePatternEntityTrait: super::entity::EntityTrait {
}

impl ServerAIFirePatternEntityTrait for ServerAIFirePatternEntity {
}

impl super::entity::EntityTrait for ServerAIFirePatternEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAIFirePatternEntity {
}

pub static SERVERAIFIREPATTERNENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIFirePatternEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIFirePatternEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIFIREPATTERNENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIFirePatternEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAIFIREPATTERNENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAIFIREPATTERNENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIFirePatternEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIFirePatternEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAIEncounterManagerEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerAIEncounterManagerEntityTrait: super::entity::EntityTrait {
}

impl ServerAIEncounterManagerEntityTrait for ServerAIEncounterManagerEntity {
}

impl super::entity::EntityTrait for ServerAIEncounterManagerEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAIEncounterManagerEntity {
}

pub static SERVERAIENCOUNTERMANAGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIEncounterManagerEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIEncounterManagerEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIENCOUNTERMANAGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIEncounterManagerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAIENCOUNTERMANAGERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAIENCOUNTERMANAGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIEncounterManagerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIEncounterManagerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAIDebugProxy {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerAIDebugProxyTrait: super::entity::EntityTrait {
}

impl ServerAIDebugProxyTrait for ServerAIDebugProxy {
}

impl super::entity::EntityTrait for ServerAIDebugProxy {
}

impl super::entity::EntityBusPeerTrait for ServerAIDebugProxy {
}

pub static SERVERAIDEBUGPROXY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIDebugProxy",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIDebugProxy as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIDEBUGPROXY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIDebugProxy {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAIDEBUGPROXY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAIDEBUGPROXY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIDebugProxy-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIDebugProxy"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerActionEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerActionEntityTrait: super::entity::EntityTrait {
}

impl ServerActionEntityTrait for ServerActionEntity {
}

impl super::entity::EntityTrait for ServerActionEntity {
}

impl super::entity::EntityBusPeerTrait for ServerActionEntity {
}

pub static SERVERACTIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerActionEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerActionEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERACTIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerActionEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERACTIONENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERACTIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerActionEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerActionEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AIConcealmentVolumeEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait AIConcealmentVolumeEntityTrait: super::entity::EntityTrait {
}

impl AIConcealmentVolumeEntityTrait for AIConcealmentVolumeEntity {
}

impl super::entity::EntityTrait for AIConcealmentVolumeEntity {
}

impl super::entity::EntityBusPeerTrait for AIConcealmentVolumeEntity {
}

pub static AICONCEALMENTVOLUMEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIConcealmentVolumeEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AIConcealmentVolumeEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(AICONCEALMENTVOLUMEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AIConcealmentVolumeEntity {
    fn type_info(&self) -> &'static TypeInfo {
        AICONCEALMENTVOLUMEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static AICONCEALMENTVOLUMEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIConcealmentVolumeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AIConcealmentVolumeEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientAITemplateFilterEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientAITemplateFilterEntityTrait: super::entity::EntityTrait {
}

impl ClientAITemplateFilterEntityTrait for ClientAITemplateFilterEntity {
}

impl super::entity::EntityTrait for ClientAITemplateFilterEntity {
}

impl super::entity::EntityBusPeerTrait for ClientAITemplateFilterEntity {
}

pub static CLIENTAITEMPLATEFILTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAITemplateFilterEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAITemplateFilterEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTAITEMPLATEFILTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAITemplateFilterEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTAITEMPLATEFILTERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTAITEMPLATEFILTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAITemplateFilterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ClientAITemplateFilterEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientAIStateEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientAIStateEntityTrait: super::entity::EntityTrait {
}

impl ClientAIStateEntityTrait for ClientAIStateEntity {
}

impl super::entity::EntityTrait for ClientAIStateEntity {
}

impl super::entity::EntityBusPeerTrait for ClientAIStateEntity {
}

pub static CLIENTAISTATEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAIStateEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAIStateEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTAISTATEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAIStateEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTAISTATEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTAISTATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAIStateEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ClientAIStateEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientAIPhysicsDrivenAnimationEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientAIPhysicsDrivenAnimationEntityTrait: super::entity::EntityTrait {
}

impl ClientAIPhysicsDrivenAnimationEntityTrait for ClientAIPhysicsDrivenAnimationEntity {
}

impl super::entity::EntityTrait for ClientAIPhysicsDrivenAnimationEntity {
}

impl super::entity::EntityBusPeerTrait for ClientAIPhysicsDrivenAnimationEntity {
}

pub static CLIENTAIPHYSICSDRIVENANIMATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAIPhysicsDrivenAnimationEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAIPhysicsDrivenAnimationEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTAIPHYSICSDRIVENANIMATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAIPhysicsDrivenAnimationEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTAIPHYSICSDRIVENANIMATIONENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTAIPHYSICSDRIVENANIMATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAIPhysicsDrivenAnimationEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ClientAIPhysicsDrivenAnimationEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientAICollisionAvoidanceSetupEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientAICollisionAvoidanceSetupEntityTrait: super::entity::EntityTrait {
}

impl ClientAICollisionAvoidanceSetupEntityTrait for ClientAICollisionAvoidanceSetupEntity {
}

impl super::entity::EntityTrait for ClientAICollisionAvoidanceSetupEntity {
}

impl super::entity::EntityBusPeerTrait for ClientAICollisionAvoidanceSetupEntity {
}

pub static CLIENTAICOLLISIONAVOIDANCESETUPENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAICollisionAvoidanceSetupEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAICollisionAvoidanceSetupEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTAICOLLISIONAVOIDANCESETUPENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAICollisionAvoidanceSetupEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTAICOLLISIONAVOIDANCESETUPENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTAICOLLISIONAVOIDANCESETUPENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAICollisionAvoidanceSetupEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ClientAICollisionAvoidanceSetupEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientActionEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientActionEntityTrait: super::entity::EntityTrait {
}

impl ClientActionEntityTrait for ClientActionEntity {
}

impl super::entity::EntityTrait for ClientActionEntity {
}

impl super::entity::EntityBusPeerTrait for ClientActionEntity {
}

pub static CLIENTACTIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientActionEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientActionEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTACTIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientActionEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTACTIONENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTACTIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientActionEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ClientActionEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AITemplateConditionEvaluator {
    pub _glacier_base: IActionConditionEvaluator,
}

pub trait AITemplateConditionEvaluatorTrait: IActionConditionEvaluatorTrait {
}

impl AITemplateConditionEvaluatorTrait for AITemplateConditionEvaluator {
}

impl IActionConditionEvaluatorTrait for AITemplateConditionEvaluator {
}

pub static AITEMPLATECONDITIONEVALUATOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AITemplateConditionEvaluator",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IACTIONCONDITIONEVALUATOR_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AITemplateConditionEvaluator as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(AITEMPLATECONDITIONEVALUATOR_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AITemplateConditionEvaluator {
    fn type_info(&self) -> &'static TypeInfo {
        AITEMPLATECONDITIONEVALUATOR_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static AITEMPLATECONDITIONEVALUATOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AITemplateConditionEvaluator-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AITemplateConditionEvaluator"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct InsideVolumesConditionEvaluator {
    pub _glacier_base: IActionConditionEvaluator,
}

pub trait InsideVolumesConditionEvaluatorTrait: IActionConditionEvaluatorTrait {
}

impl InsideVolumesConditionEvaluatorTrait for InsideVolumesConditionEvaluator {
}

impl IActionConditionEvaluatorTrait for InsideVolumesConditionEvaluator {
}

pub static INSIDEVOLUMESCONDITIONEVALUATOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InsideVolumesConditionEvaluator",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IACTIONCONDITIONEVALUATOR_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InsideVolumesConditionEvaluator as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(INSIDEVOLUMESCONDITIONEVALUATOR_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for InsideVolumesConditionEvaluator {
    fn type_info(&self) -> &'static TypeInfo {
        INSIDEVOLUMESCONDITIONEVALUATOR_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static INSIDEVOLUMESCONDITIONEVALUATOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InsideVolumesConditionEvaluator-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("InsideVolumesConditionEvaluator"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ProbabilityConditionEvaluator {
    pub _glacier_base: IActionConditionEvaluator,
}

pub trait ProbabilityConditionEvaluatorTrait: IActionConditionEvaluatorTrait {
}

impl ProbabilityConditionEvaluatorTrait for ProbabilityConditionEvaluator {
}

impl IActionConditionEvaluatorTrait for ProbabilityConditionEvaluator {
}

pub static PROBABILITYCONDITIONEVALUATOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProbabilityConditionEvaluator",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IACTIONCONDITIONEVALUATOR_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ProbabilityConditionEvaluator as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PROBABILITYCONDITIONEVALUATOR_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ProbabilityConditionEvaluator {
    fn type_info(&self) -> &'static TypeInfo {
        PROBABILITYCONDITIONEVALUATOR_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PROBABILITYCONDITIONEVALUATOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProbabilityConditionEvaluator-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ProbabilityConditionEvaluator"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FacingConditionEvaluator {
    pub _glacier_base: IActionConditionEvaluator,
}

pub trait FacingConditionEvaluatorTrait: IActionConditionEvaluatorTrait {
}

impl FacingConditionEvaluatorTrait for FacingConditionEvaluator {
}

impl IActionConditionEvaluatorTrait for FacingConditionEvaluator {
}

pub static FACINGCONDITIONEVALUATOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FacingConditionEvaluator",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IACTIONCONDITIONEVALUATOR_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FacingConditionEvaluator as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(FACINGCONDITIONEVALUATOR_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FacingConditionEvaluator {
    fn type_info(&self) -> &'static TypeInfo {
        FACINGCONDITIONEVALUATOR_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FACINGCONDITIONEVALUATOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FacingConditionEvaluator-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("FacingConditionEvaluator"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AIOrderCoordinatorConditionEvaluator {
    pub _glacier_base: IActionConditionEvaluator,
}

pub trait AIOrderCoordinatorConditionEvaluatorTrait: IActionConditionEvaluatorTrait {
}

impl AIOrderCoordinatorConditionEvaluatorTrait for AIOrderCoordinatorConditionEvaluator {
}

impl IActionConditionEvaluatorTrait for AIOrderCoordinatorConditionEvaluator {
}

pub static AIORDERCOORDINATORCONDITIONEVALUATOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIOrderCoordinatorConditionEvaluator",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IACTIONCONDITIONEVALUATOR_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AIOrderCoordinatorConditionEvaluator as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(AIORDERCOORDINATORCONDITIONEVALUATOR_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AIOrderCoordinatorConditionEvaluator {
    fn type_info(&self) -> &'static TypeInfo {
        AIORDERCOORDINATORCONDITIONEVALUATOR_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static AIORDERCOORDINATORCONDITIONEVALUATOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIOrderCoordinatorConditionEvaluator-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AIOrderCoordinatorConditionEvaluator"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AIStateConditionEvaluator {
    pub _glacier_base: IActionConditionEvaluator,
}

pub trait AIStateConditionEvaluatorTrait: IActionConditionEvaluatorTrait {
}

impl AIStateConditionEvaluatorTrait for AIStateConditionEvaluator {
}

impl IActionConditionEvaluatorTrait for AIStateConditionEvaluator {
}

pub static AISTATECONDITIONEVALUATOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIStateConditionEvaluator",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IACTIONCONDITIONEVALUATOR_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AIStateConditionEvaluator as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(AISTATECONDITIONEVALUATOR_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AIStateConditionEvaluator {
    fn type_info(&self) -> &'static TypeInfo {
        AISTATECONDITIONEVALUATOR_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static AISTATECONDITIONEVALUATOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIStateConditionEvaluator-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AIStateConditionEvaluator"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RangeConditionEvaluator {
    pub _glacier_base: IActionConditionEvaluator,
}

pub trait RangeConditionEvaluatorTrait: IActionConditionEvaluatorTrait {
}

impl RangeConditionEvaluatorTrait for RangeConditionEvaluator {
}

impl IActionConditionEvaluatorTrait for RangeConditionEvaluator {
}

pub static RANGECONDITIONEVALUATOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RangeConditionEvaluator",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IACTIONCONDITIONEVALUATOR_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RangeConditionEvaluator as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(RANGECONDITIONEVALUATOR_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RangeConditionEvaluator {
    fn type_info(&self) -> &'static TypeInfo {
        RANGECONDITIONEVALUATOR_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RANGECONDITIONEVALUATOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RangeConditionEvaluator-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("RangeConditionEvaluator"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct IActionConditionEvaluator {
}

pub trait IActionConditionEvaluatorTrait: TypeObject {
}

impl IActionConditionEvaluatorTrait for IActionConditionEvaluator {
}

pub static IACTIONCONDITIONEVALUATOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IActionConditionEvaluator",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IActionConditionEvaluator as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(IACTIONCONDITIONEVALUATOR_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IActionConditionEvaluator {
    fn type_info(&self) -> &'static TypeInfo {
        IACTIONCONDITIONEVALUATOR_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static IACTIONCONDITIONEVALUATOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IActionConditionEvaluator-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("IActionConditionEvaluator"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerPointOfInterestComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerPointOfInterestComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerPointOfInterestComponentTrait for ServerPointOfInterestComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerPointOfInterestComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerPointOfInterestComponent {
}

impl super::entity::ComponentTrait for ServerPointOfInterestComponent {
}

impl super::entity::EntityBusPeerTrait for ServerPointOfInterestComponent {
}

pub static SERVERPOINTOFINTERESTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPointOfInterestComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPointOfInterestComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERPOINTOFINTERESTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPointOfInterestComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPOINTOFINTERESTCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERPOINTOFINTERESTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPointOfInterestComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerPointOfInterestComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerCoverEntity {
    pub _glacier_base: super::entity::SpatialEntity,
}

pub trait ServerCoverEntityTrait: super::entity::SpatialEntityTrait {
}

impl ServerCoverEntityTrait for ServerCoverEntity {
}

impl super::entity::SpatialEntityTrait for ServerCoverEntity {
}

impl super::entity::EntityTrait for ServerCoverEntity {
}

impl super::entity::EntityBusPeerTrait for ServerCoverEntity {
}

pub static SERVERCOVERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCoverEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCoverEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCOVERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCoverEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCOVERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERCOVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCoverEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerCoverEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerCoverGroupEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerCoverGroupEntityTrait: super::entity::EntityTrait {
}

impl ServerCoverGroupEntityTrait for ServerCoverGroupEntity {
}

impl super::entity::EntityTrait for ServerCoverGroupEntity {
}

impl super::entity::EntityBusPeerTrait for ServerCoverGroupEntity {
}

pub static SERVERCOVERGROUPENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCoverGroupEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCoverGroupEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCOVERGROUPENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCoverGroupEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCOVERGROUPENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERCOVERGROUPENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCoverGroupEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerCoverGroupEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAIVehicleAimingComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerAIVehicleAimingComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerAIVehicleAimingComponentTrait for ServerAIVehicleAimingComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerAIVehicleAimingComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerAIVehicleAimingComponent {
}

impl super::entity::ComponentTrait for ServerAIVehicleAimingComponent {
}

impl super::entity::EntityBusPeerTrait for ServerAIVehicleAimingComponent {
}

pub static SERVERAIVEHICLEAIMINGCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIVehicleAimingComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIVehicleAimingComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIVEHICLEAIMINGCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIVehicleAimingComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAIVEHICLEAIMINGCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAIVEHICLEAIMINGCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIVehicleAimingComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIVehicleAimingComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAITargetComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerAITargetComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerAITargetComponentTrait for ServerAITargetComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerAITargetComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerAITargetComponent {
}

impl super::entity::ComponentTrait for ServerAITargetComponent {
}

impl super::entity::EntityBusPeerTrait for ServerAITargetComponent {
}

pub static SERVERAITARGETCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAITargetComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAITargetComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAITARGETCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAITargetComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAITARGETCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAITARGETCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAITargetComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAITargetComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAISuppressWeaponFiringComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerAISuppressWeaponFiringComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerAISuppressWeaponFiringComponentTrait for ServerAISuppressWeaponFiringComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerAISuppressWeaponFiringComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerAISuppressWeaponFiringComponent {
}

impl super::entity::ComponentTrait for ServerAISuppressWeaponFiringComponent {
}

impl super::entity::EntityBusPeerTrait for ServerAISuppressWeaponFiringComponent {
}

pub static SERVERAISUPPRESSWEAPONFIRINGCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISuppressWeaponFiringComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAISuppressWeaponFiringComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAISUPPRESSWEAPONFIRINGCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAISuppressWeaponFiringComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAISUPPRESSWEAPONFIRINGCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAISUPPRESSWEAPONFIRINGCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISuppressWeaponFiringComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAISuppressWeaponFiringComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAISpottingComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerAISpottingComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerAISpottingComponentTrait for ServerAISpottingComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerAISpottingComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerAISpottingComponent {
}

impl super::entity::ComponentTrait for ServerAISpottingComponent {
}

impl super::entity::EntityBusPeerTrait for ServerAISpottingComponent {
}

pub static SERVERAISPOTTINGCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISpottingComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAISpottingComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAISPOTTINGCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAISpottingComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAISPOTTINGCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAISPOTTINGCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISpottingComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAISpottingComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAISmokeVolumeComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerAISmokeVolumeComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerAISmokeVolumeComponentTrait for ServerAISmokeVolumeComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerAISmokeVolumeComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerAISmokeVolumeComponent {
}

impl super::entity::ComponentTrait for ServerAISmokeVolumeComponent {
}

impl super::entity::EntityBusPeerTrait for ServerAISmokeVolumeComponent {
}

pub static SERVERAISMOKEVOLUMECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISmokeVolumeComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAISmokeVolumeComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAISMOKEVOLUMECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAISmokeVolumeComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAISMOKEVOLUMECOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAISMOKEVOLUMECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISmokeVolumeComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAISmokeVolumeComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAIProjectileComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerAIProjectileComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerAIProjectileComponentTrait for ServerAIProjectileComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerAIProjectileComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerAIProjectileComponent {
}

impl super::entity::ComponentTrait for ServerAIProjectileComponent {
}

impl super::entity::EntityBusPeerTrait for ServerAIProjectileComponent {
}

pub static SERVERAIPROJECTILECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIProjectileComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIProjectileComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIPROJECTILECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIProjectileComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAIPROJECTILECOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAIPROJECTILECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIProjectileComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIProjectileComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAIOrderCoordinatorComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerAIOrderCoordinatorComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerAIOrderCoordinatorComponentTrait for ServerAIOrderCoordinatorComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerAIOrderCoordinatorComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerAIOrderCoordinatorComponent {
}

impl super::entity::ComponentTrait for ServerAIOrderCoordinatorComponent {
}

impl super::entity::EntityBusPeerTrait for ServerAIOrderCoordinatorComponent {
}

pub static SERVERAIORDERCOORDINATORCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIOrderCoordinatorComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIOrderCoordinatorComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIORDERCOORDINATORCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIOrderCoordinatorComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAIORDERCOORDINATORCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAIORDERCOORDINATORCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIOrderCoordinatorComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIOrderCoordinatorComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAIMeleeComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerAIMeleeComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerAIMeleeComponentTrait for ServerAIMeleeComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerAIMeleeComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerAIMeleeComponent {
}

impl super::entity::ComponentTrait for ServerAIMeleeComponent {
}

impl super::entity::EntityBusPeerTrait for ServerAIMeleeComponent {
}

pub static SERVERAIMELEECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIMeleeComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIMeleeComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIMELEECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIMeleeComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAIMELEECOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAIMELEECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIMeleeComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIMeleeComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAIEntryComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerAIEntryComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerAIEntryComponentTrait for ServerAIEntryComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerAIEntryComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerAIEntryComponent {
}

impl super::entity::ComponentTrait for ServerAIEntryComponent {
}

impl super::entity::EntityBusPeerTrait for ServerAIEntryComponent {
}

pub static SERVERAIENTRYCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIEntryComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIEntryComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIENTRYCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIEntryComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAIENTRYCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAIENTRYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIEntryComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIEntryComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAICustomInputComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerAICustomInputComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerAICustomInputComponentTrait for ServerAICustomInputComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerAICustomInputComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerAICustomInputComponent {
}

impl super::entity::ComponentTrait for ServerAICustomInputComponent {
}

impl super::entity::EntityBusPeerTrait for ServerAICustomInputComponent {
}

pub static SERVERAICUSTOMINPUTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAICustomInputComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAICustomInputComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAICUSTOMINPUTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAICustomInputComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAICUSTOMINPUTCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAICUSTOMINPUTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAICustomInputComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAICustomInputComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAIBucketSystemComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerAIBucketSystemComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerAIBucketSystemComponentTrait for ServerAIBucketSystemComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerAIBucketSystemComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerAIBucketSystemComponent {
}

impl super::entity::ComponentTrait for ServerAIBucketSystemComponent {
}

impl super::entity::EntityBusPeerTrait for ServerAIBucketSystemComponent {
}

pub static SERVERAIBUCKETSYSTEMCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIBucketSystemComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIBucketSystemComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIBUCKETSYSTEMCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIBucketSystemComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAIBUCKETSYSTEMCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAIBUCKETSYSTEMCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIBucketSystemComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIBucketSystemComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAIAnchorToPointComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerAIAnchorToPointComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerAIAnchorToPointComponentTrait for ServerAIAnchorToPointComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerAIAnchorToPointComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerAIAnchorToPointComponent {
}

impl super::entity::ComponentTrait for ServerAIAnchorToPointComponent {
}

impl super::entity::EntityBusPeerTrait for ServerAIAnchorToPointComponent {
}

pub static SERVERAIANCHORTOPOINTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIAnchorToPointComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIAnchorToPointComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAIANCHORTOPOINTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAIAnchorToPointComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAIANCHORTOPOINTCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERAIANCHORTOPOINTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIAnchorToPointComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIAnchorToPointComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientAISpottingComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientAISpottingComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientAISpottingComponentTrait for ClientAISpottingComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientAISpottingComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientAISpottingComponent {
}

impl super::entity::ComponentTrait for ClientAISpottingComponent {
}

impl super::entity::EntityBusPeerTrait for ClientAISpottingComponent {
}

pub static CLIENTAISPOTTINGCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAISpottingComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAISpottingComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTAISPOTTINGCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAISpottingComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTAISPOTTINGCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTAISPOTTINGCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAISpottingComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ClientAISpottingComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientAIProjectileComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientAIProjectileComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientAIProjectileComponentTrait for ClientAIProjectileComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientAIProjectileComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientAIProjectileComponent {
}

impl super::entity::ComponentTrait for ClientAIProjectileComponent {
}

impl super::entity::EntityBusPeerTrait for ClientAIProjectileComponent {
}

pub static CLIENTAIPROJECTILECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAIProjectileComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAIProjectileComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTAIPROJECTILECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAIProjectileComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTAIPROJECTILECOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTAIPROJECTILECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAIProjectileComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ClientAIProjectileComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientAIOrderCoordinatorComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientAIOrderCoordinatorComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientAIOrderCoordinatorComponentTrait for ClientAIOrderCoordinatorComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientAIOrderCoordinatorComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientAIOrderCoordinatorComponent {
}

impl super::entity::ComponentTrait for ClientAIOrderCoordinatorComponent {
}

impl super::entity::EntityBusPeerTrait for ClientAIOrderCoordinatorComponent {
}

pub static CLIENTAIORDERCOORDINATORCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAIOrderCoordinatorComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAIOrderCoordinatorComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTAIORDERCOORDINATORCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAIOrderCoordinatorComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTAIORDERCOORDINATORCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTAIORDERCOORDINATORCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAIOrderCoordinatorComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ClientAIOrderCoordinatorComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientAIMeleeComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientAIMeleeComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientAIMeleeComponentTrait for ClientAIMeleeComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientAIMeleeComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientAIMeleeComponent {
}

impl super::entity::ComponentTrait for ClientAIMeleeComponent {
}

impl super::entity::EntityBusPeerTrait for ClientAIMeleeComponent {
}

pub static CLIENTAIMELEECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAIMeleeComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAIMeleeComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTAIMELEECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAIMeleeComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTAIMELEECOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTAIMELEECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAIMeleeComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ClientAIMeleeComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientAIAnchorToPointComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientAIAnchorToPointComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientAIAnchorToPointComponentTrait for ClientAIAnchorToPointComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientAIAnchorToPointComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientAIAnchorToPointComponent {
}

impl super::entity::ComponentTrait for ClientAIAnchorToPointComponent {
}

impl super::entity::EntityBusPeerTrait for ClientAIAnchorToPointComponent {
}

pub static CLIENTAIANCHORTOPOINTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAIAnchorToPointComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAIAnchorToPointComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTAIANCHORTOPOINTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAIAnchorToPointComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTAIANCHORTOPOINTCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTAIANCHORTOPOINTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAIAnchorToPointComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ClientAIAnchorToPointComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CoverScoreModifierEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait CoverScoreModifierEntityTrait: super::entity::EntityTrait {
}

impl CoverScoreModifierEntityTrait for CoverScoreModifierEntity {
}

impl super::entity::EntityTrait for CoverScoreModifierEntity {
}

impl super::entity::EntityBusPeerTrait for CoverScoreModifierEntity {
}

pub static COVERSCOREMODIFIERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoverScoreModifierEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoverScoreModifierEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(COVERSCOREMODIFIERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CoverScoreModifierEntity {
    fn type_info(&self) -> &'static TypeInfo {
        COVERSCOREMODIFIERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static COVERSCOREMODIFIERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoverScoreModifierEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("CoverScoreModifierEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerPathLinkEntity {
    pub _glacier_base: PathLinkEntity,
}

pub trait ServerPathLinkEntityTrait: PathLinkEntityTrait {
}

impl ServerPathLinkEntityTrait for ServerPathLinkEntity {
}

impl PathLinkEntityTrait for ServerPathLinkEntity {
}

impl super::entity::SpatialEntityTrait for ServerPathLinkEntity {
}

impl super::entity::EntityTrait for ServerPathLinkEntity {
}

impl super::entity::EntityBusPeerTrait for ServerPathLinkEntity {
}

pub static SERVERPATHLINKENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPathLinkEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PATHLINKENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPathLinkEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERPATHLINKENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPathLinkEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPATHLINKENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERPATHLINKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPathLinkEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerPathLinkEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerPathfindingStreamEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerPathfindingStreamEntityTrait: super::entity::EntityTrait {
}

impl ServerPathfindingStreamEntityTrait for ServerPathfindingStreamEntity {
}

impl super::entity::EntityTrait for ServerPathfindingStreamEntity {
}

impl super::entity::EntityBusPeerTrait for ServerPathfindingStreamEntity {
}

pub static SERVERPATHFINDINGSTREAMENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPathfindingStreamEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPathfindingStreamEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERPATHFINDINGSTREAMENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPathfindingStreamEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPATHFINDINGSTREAMENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERPATHFINDINGSTREAMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPathfindingStreamEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerPathfindingStreamEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerNavPowerObstacleComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerNavPowerObstacleComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerNavPowerObstacleComponentTrait for ServerNavPowerObstacleComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerNavPowerObstacleComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerNavPowerObstacleComponent {
}

impl super::entity::ComponentTrait for ServerNavPowerObstacleComponent {
}

impl super::entity::EntityBusPeerTrait for ServerNavPowerObstacleComponent {
}

pub static SERVERNAVPOWEROBSTACLECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerNavPowerObstacleComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerNavPowerObstacleComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERNAVPOWEROBSTACLECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerNavPowerObstacleComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERNAVPOWEROBSTACLECOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERNAVPOWEROBSTACLECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerNavPowerObstacleComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerNavPowerObstacleComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PathLinkEntity {
    pub _glacier_base: super::entity::SpatialEntity,
}

pub trait PathLinkEntityTrait: super::entity::SpatialEntityTrait {
}

impl PathLinkEntityTrait for PathLinkEntity {
}

impl super::entity::SpatialEntityTrait for PathLinkEntity {
}

impl super::entity::EntityTrait for PathLinkEntity {
}

impl super::entity::EntityBusPeerTrait for PathLinkEntity {
}

pub static PATHLINKENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathLinkEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PathLinkEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PATHLINKENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PathLinkEntity {
    fn type_info(&self) -> &'static TypeInfo {
        PATHLINKENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PATHLINKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathLinkEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("PathLinkEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerWaypointTriggerEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerWaypointTriggerEntityTrait: super::entity::EntityTrait {
}

impl ServerWaypointTriggerEntityTrait for ServerWaypointTriggerEntity {
}

impl super::entity::EntityTrait for ServerWaypointTriggerEntity {
}

impl super::entity::EntityBusPeerTrait for ServerWaypointTriggerEntity {
}

pub static SERVERWAYPOINTTRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWaypointTriggerEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerWaypointTriggerEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERWAYPOINTTRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerWaypointTriggerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERWAYPOINTTRIGGERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERWAYPOINTTRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWaypointTriggerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerWaypointTriggerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerPathFollowingComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerPathFollowingComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerPathFollowingComponentTrait for ServerPathFollowingComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerPathFollowingComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerPathFollowingComponent {
}

impl super::entity::ComponentTrait for ServerPathFollowingComponent {
}

impl super::entity::EntityBusPeerTrait for ServerPathFollowingComponent {
}

pub static SERVERPATHFOLLOWINGCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPathFollowingComponent",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPathFollowingComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERPATHFOLLOWINGCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPathFollowingComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPATHFOLLOWINGCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERPATHFOLLOWINGCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPathFollowingComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerPathFollowingComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerPathfindingOverride {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerPathfindingOverrideTrait: super::entity::EntityTrait {
}

impl ServerPathfindingOverrideTrait for ServerPathfindingOverride {
}

impl super::entity::EntityTrait for ServerPathfindingOverride {
}

impl super::entity::EntityBusPeerTrait for ServerPathfindingOverride {
}

pub static SERVERPATHFINDINGOVERRIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPathfindingOverride",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPathfindingOverride as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERPATHFINDINGOVERRIDE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPathfindingOverride {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPATHFINDINGOVERRIDE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERPATHFINDINGOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPathfindingOverride-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerPathfindingOverride"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerFollowWaypointsEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerFollowWaypointsEntityTrait: super::entity::EntityTrait {
}

impl ServerFollowWaypointsEntityTrait for ServerFollowWaypointsEntity {
}

impl super::entity::EntityTrait for ServerFollowWaypointsEntity {
}

impl super::entity::EntityBusPeerTrait for ServerFollowWaypointsEntity {
}

pub static SERVERFOLLOWWAYPOINTSENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerFollowWaypointsEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerFollowWaypointsEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERFOLLOWWAYPOINTSENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerFollowWaypointsEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERFOLLOWWAYPOINTSENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERFOLLOWWAYPOINTSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerFollowWaypointsEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerFollowWaypointsEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerFollowObjectEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerFollowObjectEntityTrait: super::entity::EntityTrait {
}

impl ServerFollowObjectEntityTrait for ServerFollowObjectEntity {
}

impl super::entity::EntityTrait for ServerFollowObjectEntity {
}

impl super::entity::EntityBusPeerTrait for ServerFollowObjectEntity {
}

pub static SERVERFOLLOWOBJECTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerFollowObjectEntity",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerFollowObjectEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERFOLLOWOBJECTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerFollowObjectEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERFOLLOWOBJECTENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERFOLLOWOBJECTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerFollowObjectEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerFollowObjectEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SpatialAnalyzerData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub reference_transform: super::core::LinearTransform,
    pub analysis_f_o_v: f32,
    pub sweep_steps: u32,
    pub edge_distance_threshold: f32,
    pub near_limit_distance: f32,
    pub far_limit_distance: f32,
}

pub trait SpatialAnalyzerDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn reference_transform(&self) -> &super::core::LinearTransform;
    fn analysis_f_o_v(&self) -> &f32;
    fn sweep_steps(&self) -> &u32;
    fn edge_distance_threshold(&self) -> &f32;
    fn near_limit_distance(&self) -> &f32;
    fn far_limit_distance(&self) -> &f32;
}

impl SpatialAnalyzerDataTrait for SpatialAnalyzerData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn reference_transform(&self) -> &super::core::LinearTransform {
        &self.reference_transform
    }
    fn analysis_f_o_v(&self) -> &f32 {
        &self.analysis_f_o_v
    }
    fn sweep_steps(&self) -> &u32 {
        &self.sweep_steps
    }
    fn edge_distance_threshold(&self) -> &f32 {
        &self.edge_distance_threshold
    }
    fn near_limit_distance(&self) -> &f32 {
        &self.near_limit_distance
    }
    fn far_limit_distance(&self) -> &f32 {
        &self.far_limit_distance
    }
}

impl super::entity::EntityDataTrait for SpatialAnalyzerData {
}

impl super::entity::GameObjectDataTrait for SpatialAnalyzerData {
}

impl super::core::DataBusPeerTrait for SpatialAnalyzerData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for SpatialAnalyzerData {
}

impl super::core::DataContainerTrait for SpatialAnalyzerData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static SPATIALANALYZERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpatialAnalyzerData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SpatialAnalyzerData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(SpatialAnalyzerData, realm),
            },
            FieldInfoData {
                name: "ReferenceTransform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(SpatialAnalyzerData, reference_transform),
            },
            FieldInfoData {
                name: "AnalysisFOV",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SpatialAnalyzerData, analysis_f_o_v),
            },
            FieldInfoData {
                name: "SweepSteps",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(SpatialAnalyzerData, sweep_steps),
            },
            FieldInfoData {
                name: "EdgeDistanceThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SpatialAnalyzerData, edge_distance_threshold),
            },
            FieldInfoData {
                name: "NearLimitDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SpatialAnalyzerData, near_limit_distance),
            },
            FieldInfoData {
                name: "FarLimitDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SpatialAnalyzerData, far_limit_distance),
            },
        ],
    }),
    array_type: Some(SPATIALANALYZERDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SpatialAnalyzerData {
    fn type_info(&self) -> &'static TypeInfo {
        SPATIALANALYZERDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SPATIALANALYZERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpatialAnalyzerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("SpatialAnalyzerData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AttackPointEntityData {
    pub _glacier_base: super::entity::SpatialEntityData,
    pub enabled: bool,
    pub only_accept_linked_spawners: bool,
    pub team_id: super::gameplay_sim::TeamId,
    pub max_target_count_for_l_o_s_check: i32,
    pub navmesh_layer: i32,
    pub ground_transform_horizontal_offset: f32,
}

pub trait AttackPointEntityDataTrait: super::entity::SpatialEntityDataTrait {
    fn enabled(&self) -> &bool;
    fn only_accept_linked_spawners(&self) -> &bool;
    fn team_id(&self) -> &super::gameplay_sim::TeamId;
    fn max_target_count_for_l_o_s_check(&self) -> &i32;
    fn navmesh_layer(&self) -> &i32;
    fn ground_transform_horizontal_offset(&self) -> &f32;
}

impl AttackPointEntityDataTrait for AttackPointEntityData {
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn only_accept_linked_spawners(&self) -> &bool {
        &self.only_accept_linked_spawners
    }
    fn team_id(&self) -> &super::gameplay_sim::TeamId {
        &self.team_id
    }
    fn max_target_count_for_l_o_s_check(&self) -> &i32 {
        &self.max_target_count_for_l_o_s_check
    }
    fn navmesh_layer(&self) -> &i32 {
        &self.navmesh_layer
    }
    fn ground_transform_horizontal_offset(&self) -> &f32 {
        &self.ground_transform_horizontal_offset
    }
}

impl super::entity::SpatialEntityDataTrait for AttackPointEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
}

impl super::entity::EntityDataTrait for AttackPointEntityData {
}

impl super::entity::GameObjectDataTrait for AttackPointEntityData {
}

impl super::core::DataBusPeerTrait for AttackPointEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for AttackPointEntityData {
}

impl super::core::DataContainerTrait for AttackPointEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static ATTACKPOINTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AttackPointEntityData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AttackPointEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AttackPointEntityData, enabled),
            },
            FieldInfoData {
                name: "OnlyAcceptLinkedSpawners",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AttackPointEntityData, only_accept_linked_spawners),
            },
            FieldInfoData {
                name: "TeamId",
                flags: MemberInfoFlags::new(0),
                field_type: "TeamId",
                rust_offset: offset_of!(AttackPointEntityData, team_id),
            },
            FieldInfoData {
                name: "MaxTargetCountForLOSCheck",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AttackPointEntityData, max_target_count_for_l_o_s_check),
            },
            FieldInfoData {
                name: "NavmeshLayer",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AttackPointEntityData, navmesh_layer),
            },
            FieldInfoData {
                name: "GroundTransformHorizontalOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AttackPointEntityData, ground_transform_horizontal_offset),
            },
        ],
    }),
    array_type: Some(ATTACKPOINTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AttackPointEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        ATTACKPOINTENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ATTACKPOINTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AttackPointEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AttackPointEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ActionEntityData {
    pub _glacier_base: super::entity::SpatialEntityData,
    pub enabled: bool,
    pub conditions: Vec<Option<Arc<Mutex<dyn ActionConditionTrait>>>>,
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

pub trait ActionEntityDataTrait: super::entity::SpatialEntityDataTrait {
    fn enabled(&self) -> &bool;
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn ActionConditionTrait>>>>;
    fn move_to_action_before_executing(&self) -> &bool;
    fn align_to_action_before_executing(&self) -> &bool;
    fn wait_time_before_executing(&self) -> &f32;
    fn cooldown_time(&self) -> &f32;
    fn timeout_duration(&self) -> &f32;
    fn abort_when_alerted(&self) -> &bool;
    fn abort_when_timed_out(&self) -> &bool;
    fn valid_in_state(&self) -> &ValidInState;
    fn action_priority(&self) -> &ActionPriority;
    fn should_be_executed_by_closest_a_i(&self) -> &bool;
    fn only_execute_for_fitness_valid_a_i(&self) -> &bool;
    fn use_actual_path_distance(&self) -> &bool;
    fn is_linked_action(&self) -> &bool;
    fn restrict_to_linked_soldiers(&self) -> &bool;
}

impl ActionEntityDataTrait for ActionEntityData {
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn ActionConditionTrait>>>> {
        &self.conditions
    }
    fn move_to_action_before_executing(&self) -> &bool {
        &self.move_to_action_before_executing
    }
    fn align_to_action_before_executing(&self) -> &bool {
        &self.align_to_action_before_executing
    }
    fn wait_time_before_executing(&self) -> &f32 {
        &self.wait_time_before_executing
    }
    fn cooldown_time(&self) -> &f32 {
        &self.cooldown_time
    }
    fn timeout_duration(&self) -> &f32 {
        &self.timeout_duration
    }
    fn abort_when_alerted(&self) -> &bool {
        &self.abort_when_alerted
    }
    fn abort_when_timed_out(&self) -> &bool {
        &self.abort_when_timed_out
    }
    fn valid_in_state(&self) -> &ValidInState {
        &self.valid_in_state
    }
    fn action_priority(&self) -> &ActionPriority {
        &self.action_priority
    }
    fn should_be_executed_by_closest_a_i(&self) -> &bool {
        &self.should_be_executed_by_closest_a_i
    }
    fn only_execute_for_fitness_valid_a_i(&self) -> &bool {
        &self.only_execute_for_fitness_valid_a_i
    }
    fn use_actual_path_distance(&self) -> &bool {
        &self.use_actual_path_distance
    }
    fn is_linked_action(&self) -> &bool {
        &self.is_linked_action
    }
    fn restrict_to_linked_soldiers(&self) -> &bool {
        &self.restrict_to_linked_soldiers
    }
}

impl super::entity::SpatialEntityDataTrait for ActionEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
}

impl super::entity::EntityDataTrait for ActionEntityData {
}

impl super::entity::GameObjectDataTrait for ActionEntityData {
}

impl super::core::DataBusPeerTrait for ActionEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for ActionEntityData {
}

impl super::core::DataContainerTrait for ActionEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static ACTIONENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ActionEntityData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ActionEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ActionEntityData, enabled),
            },
            FieldInfoData {
                name: "Conditions",
                flags: MemberInfoFlags::new(144),
                field_type: "ActionCondition-Array",
                rust_offset: offset_of!(ActionEntityData, conditions),
            },
            FieldInfoData {
                name: "MoveToActionBeforeExecuting",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ActionEntityData, move_to_action_before_executing),
            },
            FieldInfoData {
                name: "AlignToActionBeforeExecuting",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ActionEntityData, align_to_action_before_executing),
            },
            FieldInfoData {
                name: "WaitTimeBeforeExecuting",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ActionEntityData, wait_time_before_executing),
            },
            FieldInfoData {
                name: "CooldownTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ActionEntityData, cooldown_time),
            },
            FieldInfoData {
                name: "TimeoutDuration",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ActionEntityData, timeout_duration),
            },
            FieldInfoData {
                name: "AbortWhenAlerted",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ActionEntityData, abort_when_alerted),
            },
            FieldInfoData {
                name: "AbortWhenTimedOut",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ActionEntityData, abort_when_timed_out),
            },
            FieldInfoData {
                name: "ValidInState",
                flags: MemberInfoFlags::new(0),
                field_type: "ValidInState",
                rust_offset: offset_of!(ActionEntityData, valid_in_state),
            },
            FieldInfoData {
                name: "ActionPriority",
                flags: MemberInfoFlags::new(0),
                field_type: "ActionPriority",
                rust_offset: offset_of!(ActionEntityData, action_priority),
            },
            FieldInfoData {
                name: "ShouldBeExecutedByClosestAI",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ActionEntityData, should_be_executed_by_closest_a_i),
            },
            FieldInfoData {
                name: "OnlyExecuteForFitnessValidAI",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ActionEntityData, only_execute_for_fitness_valid_a_i),
            },
            FieldInfoData {
                name: "UseActualPathDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ActionEntityData, use_actual_path_distance),
            },
            FieldInfoData {
                name: "IsLinkedAction",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ActionEntityData, is_linked_action),
            },
            FieldInfoData {
                name: "RestrictToLinkedSoldiers",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ActionEntityData, restrict_to_linked_soldiers),
            },
        ],
    }),
    array_type: Some(ACTIONENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ActionEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        ACTIONENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ACTIONENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ActionEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ActionEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ValidInState {
    #[default]
    ValidInState_Idle = 0,
    ValidInState_Search = 1,
    ValidInState_Combat = 2,
    ValidInState_Any = 3,
    ValidInState_Count = 4,
}

pub static VALIDINSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ValidInState",
    flags: MemberInfoFlags::new(49429),
    module: "BattleAI",
    data: TypeInfoData::Enum,
    array_type: Some(VALIDINSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ValidInState {
    fn type_info(&self) -> &'static TypeInfo {
        VALIDINSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VALIDINSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ValidInState-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ValidInState"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ActionPriority {
    #[default]
    ActionPriority_Idle = 0,
    ActionPriority_Investigate = 1,
    ActionPriority_Search = 2,
    ActionPriority_Count = 3,
}

pub static ACTIONPRIORITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ActionPriority",
    flags: MemberInfoFlags::new(49429),
    module: "BattleAI",
    data: TypeInfoData::Enum,
    array_type: Some(ACTIONPRIORITY_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ActionPriority {
    fn type_info(&self) -> &'static TypeInfo {
        ACTIONPRIORITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ACTIONPRIORITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ActionPriority-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ActionPriority"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ActionState {
    #[default]
    ActionState_Idle = 0,
    ActionState_Approach = 1,
    ActionState_Running = 2,
    ActionState_Cooldown = 3,
    ActionState_Count = 4,
}

pub static ACTIONSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ActionState",
    flags: MemberInfoFlags::new(49429),
    module: "BattleAI",
    data: TypeInfoData::Enum,
    array_type: Some(ACTIONSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ActionState {
    fn type_info(&self) -> &'static TypeInfo {
        ACTIONSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ACTIONSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ActionState-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ActionState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AITemplateCondition {
    pub _glacier_base: ActionCondition,
    pub templates: Vec<String>,
}

pub trait AITemplateConditionTrait: ActionConditionTrait {
    fn templates(&self) -> &Vec<String>;
}

impl AITemplateConditionTrait for AITemplateCondition {
    fn templates(&self) -> &Vec<String> {
        &self.templates
    }
}

impl ActionConditionTrait for AITemplateCondition {
}

impl super::core::DataContainerTrait for AITemplateCondition {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static AITEMPLATECONDITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AITemplateCondition",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ACTIONCONDITION_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AITemplateCondition as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Templates",
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(AITemplateCondition, templates),
            },
        ],
    }),
    array_type: Some(AITEMPLATECONDITION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AITemplateCondition {
    fn type_info(&self) -> &'static TypeInfo {
        AITEMPLATECONDITION_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static AITEMPLATECONDITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AITemplateCondition-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AITemplateCondition"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct InsideVolumesCondition {
    pub _glacier_base: ActionCondition,
    pub station_inside_user_search_area: bool,
    pub station_inside_user_defend_area: bool,
}

pub trait InsideVolumesConditionTrait: ActionConditionTrait {
    fn station_inside_user_search_area(&self) -> &bool;
    fn station_inside_user_defend_area(&self) -> &bool;
}

impl InsideVolumesConditionTrait for InsideVolumesCondition {
    fn station_inside_user_search_area(&self) -> &bool {
        &self.station_inside_user_search_area
    }
    fn station_inside_user_defend_area(&self) -> &bool {
        &self.station_inside_user_defend_area
    }
}

impl ActionConditionTrait for InsideVolumesCondition {
}

impl super::core::DataContainerTrait for InsideVolumesCondition {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static INSIDEVOLUMESCONDITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InsideVolumesCondition",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ACTIONCONDITION_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InsideVolumesCondition as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "StationInsideUserSearchArea",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(InsideVolumesCondition, station_inside_user_search_area),
            },
            FieldInfoData {
                name: "StationInsideUserDefendArea",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(InsideVolumesCondition, station_inside_user_defend_area),
            },
        ],
    }),
    array_type: Some(INSIDEVOLUMESCONDITION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for InsideVolumesCondition {
    fn type_info(&self) -> &'static TypeInfo {
        INSIDEVOLUMESCONDITION_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static INSIDEVOLUMESCONDITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InsideVolumesCondition-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("InsideVolumesCondition"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ProbabilityCondition {
    pub _glacier_base: ActionCondition,
    pub probability: f32,
}

pub trait ProbabilityConditionTrait: ActionConditionTrait {
    fn probability(&self) -> &f32;
}

impl ProbabilityConditionTrait for ProbabilityCondition {
    fn probability(&self) -> &f32 {
        &self.probability
    }
}

impl ActionConditionTrait for ProbabilityCondition {
}

impl super::core::DataContainerTrait for ProbabilityCondition {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PROBABILITYCONDITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProbabilityCondition",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ACTIONCONDITION_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ProbabilityCondition as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Probability",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ProbabilityCondition, probability),
            },
        ],
    }),
    array_type: Some(PROBABILITYCONDITION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProbabilityCondition {
    fn type_info(&self) -> &'static TypeInfo {
        PROBABILITYCONDITION_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PROBABILITYCONDITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProbabilityCondition-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ProbabilityCondition"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FacingCondition {
    pub _glacier_base: ActionCondition,
    pub max_angle: f32,
}

pub trait FacingConditionTrait: ActionConditionTrait {
    fn max_angle(&self) -> &f32;
}

impl FacingConditionTrait for FacingCondition {
    fn max_angle(&self) -> &f32 {
        &self.max_angle
    }
}

impl ActionConditionTrait for FacingCondition {
}

impl super::core::DataContainerTrait for FacingCondition {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static FACINGCONDITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FacingCondition",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ACTIONCONDITION_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FacingCondition as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MaxAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FacingCondition, max_angle),
            },
        ],
    }),
    array_type: Some(FACINGCONDITION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FacingCondition {
    fn type_info(&self) -> &'static TypeInfo {
        FACINGCONDITION_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FACINGCONDITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FacingCondition-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("FacingCondition"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AIOrderCoordinatorCondition {
    pub _glacier_base: ActionCondition,
    pub only_valid_when_expecting_self_orders: bool,
}

pub trait AIOrderCoordinatorConditionTrait: ActionConditionTrait {
    fn only_valid_when_expecting_self_orders(&self) -> &bool;
}

impl AIOrderCoordinatorConditionTrait for AIOrderCoordinatorCondition {
    fn only_valid_when_expecting_self_orders(&self) -> &bool {
        &self.only_valid_when_expecting_self_orders
    }
}

impl ActionConditionTrait for AIOrderCoordinatorCondition {
}

impl super::core::DataContainerTrait for AIOrderCoordinatorCondition {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static AIORDERCOORDINATORCONDITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIOrderCoordinatorCondition",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ACTIONCONDITION_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AIOrderCoordinatorCondition as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "OnlyValidWhenExpectingSelfOrders",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AIOrderCoordinatorCondition, only_valid_when_expecting_self_orders),
            },
        ],
    }),
    array_type: Some(AIORDERCOORDINATORCONDITION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AIOrderCoordinatorCondition {
    fn type_info(&self) -> &'static TypeInfo {
        AIORDERCOORDINATORCONDITION_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static AIORDERCOORDINATORCONDITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIOrderCoordinatorCondition-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AIOrderCoordinatorCondition"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AIStateCondition {
    pub _glacier_base: ActionCondition,
    pub a_i_state: ConditionAIStates,
    pub restrict_search_area_to_secondary_search: bool,
}

pub trait AIStateConditionTrait: ActionConditionTrait {
    fn a_i_state(&self) -> &ConditionAIStates;
    fn restrict_search_area_to_secondary_search(&self) -> &bool;
}

impl AIStateConditionTrait for AIStateCondition {
    fn a_i_state(&self) -> &ConditionAIStates {
        &self.a_i_state
    }
    fn restrict_search_area_to_secondary_search(&self) -> &bool {
        &self.restrict_search_area_to_secondary_search
    }
}

impl ActionConditionTrait for AIStateCondition {
}

impl super::core::DataContainerTrait for AIStateCondition {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static AISTATECONDITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIStateCondition",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ACTIONCONDITION_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AIStateCondition as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "AIState",
                flags: MemberInfoFlags::new(0),
                field_type: "ConditionAIStates",
                rust_offset: offset_of!(AIStateCondition, a_i_state),
            },
            FieldInfoData {
                name: "RestrictSearchAreaToSecondarySearch",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AIStateCondition, restrict_search_area_to_secondary_search),
            },
        ],
    }),
    array_type: Some(AISTATECONDITION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AIStateCondition {
    fn type_info(&self) -> &'static TypeInfo {
        AISTATECONDITION_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static AISTATECONDITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIStateCondition-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AIStateCondition"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ConditionAIStates {
    #[default]
    ConditionAIStates_Idle = 0,
    ConditionAIStates_Idle_Investigate = 1,
    ConditionAIStates_Combat = 2,
    ConditionAIStates_Combat_Investigate = 3,
    ConditionAIStates_Combat_SearchArea = 4,
    ConditionAIStates_SearchArea = 5,
}

pub static CONDITIONAISTATES_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionAIStates",
    flags: MemberInfoFlags::new(49429),
    module: "BattleAI",
    data: TypeInfoData::Enum,
    array_type: Some(CONDITIONAISTATES_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ConditionAIStates {
    fn type_info(&self) -> &'static TypeInfo {
        CONDITIONAISTATES_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CONDITIONAISTATES_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionAIStates-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ConditionAIStates"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RangeCondition {
    pub _glacier_base: ActionCondition,
    pub radius: f32,
}

pub trait RangeConditionTrait: ActionConditionTrait {
    fn radius(&self) -> &f32;
}

impl RangeConditionTrait for RangeCondition {
    fn radius(&self) -> &f32 {
        &self.radius
    }
}

impl ActionConditionTrait for RangeCondition {
}

impl super::core::DataContainerTrait for RangeCondition {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static RANGECONDITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RangeCondition",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ACTIONCONDITION_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RangeCondition as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RangeCondition, radius),
            },
        ],
    }),
    array_type: Some(RANGECONDITION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RangeCondition {
    fn type_info(&self) -> &'static TypeInfo {
        RANGECONDITION_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RANGECONDITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RangeCondition-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("RangeCondition"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ActionCondition {
    pub _glacier_base: super::core::DataContainer,
}

pub trait ActionConditionTrait: super::core::DataContainerTrait {
}

impl ActionConditionTrait for ActionCondition {
}

impl super::core::DataContainerTrait for ActionCondition {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static ACTIONCONDITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ActionCondition",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ActionCondition as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(ACTIONCONDITION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ActionCondition {
    fn type_info(&self) -> &'static TypeInfo {
        ACTIONCONDITION_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ACTIONCONDITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ActionCondition-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ActionCondition"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PreferredCoverScoreData {
    pub _glacier_base: CustomCoverScoreData,
    pub preferred_cover_id: u32,
    pub score: f32,
}

pub trait PreferredCoverScoreDataTrait: CustomCoverScoreDataTrait {
    fn preferred_cover_id(&self) -> &u32;
    fn score(&self) -> &f32;
}

impl PreferredCoverScoreDataTrait for PreferredCoverScoreData {
    fn preferred_cover_id(&self) -> &u32 {
        &self.preferred_cover_id
    }
    fn score(&self) -> &f32 {
        &self.score
    }
}

impl CustomCoverScoreDataTrait for PreferredCoverScoreData {
}

impl CoverScoreDataTrait for PreferredCoverScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for PreferredCoverScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
}

impl super::core::DataContainerTrait for PreferredCoverScoreData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PREFERREDCOVERSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PreferredCoverScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CUSTOMCOVERSCOREDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PreferredCoverScoreData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "PreferredCoverId",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(PreferredCoverScoreData, preferred_cover_id),
            },
            FieldInfoData {
                name: "Score",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PreferredCoverScoreData, score),
            },
        ],
    }),
    array_type: Some(PREFERREDCOVERSCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PreferredCoverScoreData {
    fn type_info(&self) -> &'static TypeInfo {
        PREFERREDCOVERSCOREDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PREFERREDCOVERSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PreferredCoverScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("PreferredCoverScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PreferredAreaScoreData {
    pub _glacier_base: CustomCoverScoreData,
    pub shapes: Vec<BaseShapeWithOffset>,
    pub score: f32,
}

pub trait PreferredAreaScoreDataTrait: CustomCoverScoreDataTrait {
    fn shapes(&self) -> &Vec<BaseShapeWithOffset>;
    fn score(&self) -> &f32;
}

impl PreferredAreaScoreDataTrait for PreferredAreaScoreData {
    fn shapes(&self) -> &Vec<BaseShapeWithOffset> {
        &self.shapes
    }
    fn score(&self) -> &f32 {
        &self.score
    }
}

impl CustomCoverScoreDataTrait for PreferredAreaScoreData {
}

impl CoverScoreDataTrait for PreferredAreaScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for PreferredAreaScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
}

impl super::core::DataContainerTrait for PreferredAreaScoreData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PREFERREDAREASCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PreferredAreaScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CUSTOMCOVERSCOREDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PreferredAreaScoreData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Shapes",
                flags: MemberInfoFlags::new(144),
                field_type: "BaseShapeWithOffset-Array",
                rust_offset: offset_of!(PreferredAreaScoreData, shapes),
            },
            FieldInfoData {
                name: "Score",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PreferredAreaScoreData, score),
            },
        ],
    }),
    array_type: Some(PREFERREDAREASCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PreferredAreaScoreData {
    fn type_info(&self) -> &'static TypeInfo {
        PREFERREDAREASCOREDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PREFERREDAREASCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PreferredAreaScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("PreferredAreaScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BaseShapeWithOffset {
    pub shape: Option<Arc<Mutex<dyn super::entity::BaseShapeDataTrait>>>,
    pub offset: super::core::LinearTransform,
    pub owner_transform: super::core::LinearTransform,
}

pub trait BaseShapeWithOffsetTrait: TypeObject {
    fn shape(&self) -> &Option<Arc<Mutex<dyn super::entity::BaseShapeDataTrait>>>;
    fn offset(&self) -> &super::core::LinearTransform;
    fn owner_transform(&self) -> &super::core::LinearTransform;
}

impl BaseShapeWithOffsetTrait for BaseShapeWithOffset {
    fn shape(&self) -> &Option<Arc<Mutex<dyn super::entity::BaseShapeDataTrait>>> {
        &self.shape
    }
    fn offset(&self) -> &super::core::LinearTransform {
        &self.offset
    }
    fn owner_transform(&self) -> &super::core::LinearTransform {
        &self.owner_transform
    }
}

pub static BASESHAPEWITHOFFSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BaseShapeWithOffset",
    flags: MemberInfoFlags::new(73),
    module: "BattleAI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BaseShapeWithOffset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Shape",
                flags: MemberInfoFlags::new(0),
                field_type: "BaseShapeData",
                rust_offset: offset_of!(BaseShapeWithOffset, shape),
            },
            FieldInfoData {
                name: "Offset",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(BaseShapeWithOffset, offset),
            },
            FieldInfoData {
                name: "OwnerTransform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(BaseShapeWithOffset, owner_transform),
            },
        ],
    }),
    array_type: Some(BASESHAPEWITHOFFSET_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for BaseShapeWithOffset {
    fn type_info(&self) -> &'static TypeInfo {
        BASESHAPEWITHOFFSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static BASESHAPEWITHOFFSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BaseShapeWithOffset-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("BaseShapeWithOffset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CustomCoverScoreData {
    pub _glacier_base: CoverScoreData,
}

pub trait CustomCoverScoreDataTrait: CoverScoreDataTrait {
}

impl CustomCoverScoreDataTrait for CustomCoverScoreData {
}

impl CoverScoreDataTrait for CustomCoverScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for CustomCoverScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
}

impl super::core::DataContainerTrait for CustomCoverScoreData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static CUSTOMCOVERSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomCoverScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CustomCoverScoreData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CUSTOMCOVERSCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CustomCoverScoreData {
    fn type_info(&self) -> &'static TypeInfo {
        CUSTOMCOVERSCOREDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CUSTOMCOVERSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomCoverScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("CustomCoverScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ReducePathDistanceToFollowObjectScoreData {
    pub _glacier_base: CoverScoreDataWithScoreCurve,
}

pub trait ReducePathDistanceToFollowObjectScoreDataTrait: CoverScoreDataWithScoreCurveTrait {
}

impl ReducePathDistanceToFollowObjectScoreDataTrait for ReducePathDistanceToFollowObjectScoreData {
}

impl CoverScoreDataWithScoreCurveTrait for ReducePathDistanceToFollowObjectScoreData {
    fn score_curve(&self) -> &Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>> {
        self._glacier_base.score_curve()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
}

impl CoverScoreDataTrait for ReducePathDistanceToFollowObjectScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for ReducePathDistanceToFollowObjectScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
}

impl super::core::DataContainerTrait for ReducePathDistanceToFollowObjectScoreData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static REDUCEPATHDISTANCETOFOLLOWOBJECTSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReducePathDistanceToFollowObjectScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ReducePathDistanceToFollowObjectScoreData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(REDUCEPATHDISTANCETOFOLLOWOBJECTSCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ReducePathDistanceToFollowObjectScoreData {
    fn type_info(&self) -> &'static TypeInfo {
        REDUCEPATHDISTANCETOFOLLOWOBJECTSCOREDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static REDUCEPATHDISTANCETOFOLLOWOBJECTSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReducePathDistanceToFollowObjectScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ReducePathDistanceToFollowObjectScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ReducePathDistanceToTargetScoreData {
    pub _glacier_base: CoverScoreDataWithScoreCurve,
}

pub trait ReducePathDistanceToTargetScoreDataTrait: CoverScoreDataWithScoreCurveTrait {
}

impl ReducePathDistanceToTargetScoreDataTrait for ReducePathDistanceToTargetScoreData {
}

impl CoverScoreDataWithScoreCurveTrait for ReducePathDistanceToTargetScoreData {
    fn score_curve(&self) -> &Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>> {
        self._glacier_base.score_curve()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
}

impl CoverScoreDataTrait for ReducePathDistanceToTargetScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for ReducePathDistanceToTargetScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
}

impl super::core::DataContainerTrait for ReducePathDistanceToTargetScoreData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static REDUCEPATHDISTANCETOTARGETSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReducePathDistanceToTargetScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ReducePathDistanceToTargetScoreData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(REDUCEPATHDISTANCETOTARGETSCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ReducePathDistanceToTargetScoreData {
    fn type_info(&self) -> &'static TypeInfo {
        REDUCEPATHDISTANCETOTARGETSCOREDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static REDUCEPATHDISTANCETOTARGETSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReducePathDistanceToTargetScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ReducePathDistanceToTargetScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LineOfFireScoreData {
    pub _glacier_base: CoverScoreDataWithScoreCurve,
    pub override_open_cover_height: f32,
}

pub trait LineOfFireScoreDataTrait: CoverScoreDataWithScoreCurveTrait {
    fn override_open_cover_height(&self) -> &f32;
}

impl LineOfFireScoreDataTrait for LineOfFireScoreData {
    fn override_open_cover_height(&self) -> &f32 {
        &self.override_open_cover_height
    }
}

impl CoverScoreDataWithScoreCurveTrait for LineOfFireScoreData {
    fn score_curve(&self) -> &Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>> {
        self._glacier_base.score_curve()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
}

impl CoverScoreDataTrait for LineOfFireScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for LineOfFireScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
}

impl super::core::DataContainerTrait for LineOfFireScoreData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static LINEOFFIRESCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LineOfFireScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LineOfFireScoreData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "OverrideOpenCoverHeight",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LineOfFireScoreData, override_open_cover_height),
            },
        ],
    }),
    array_type: Some(LINEOFFIRESCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LineOfFireScoreData {
    fn type_info(&self) -> &'static TypeInfo {
        LINEOFFIRESCOREDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LINEOFFIRESCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LineOfFireScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("LineOfFireScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct NavProbeScoreData {
    pub _glacier_base: CoverScoreData,
    pub ref_position: CoverScorePosition,
    pub score: f32,
    pub max_probe_distance: f32,
}

pub trait NavProbeScoreDataTrait: CoverScoreDataTrait {
    fn ref_position(&self) -> &CoverScorePosition;
    fn score(&self) -> &f32;
    fn max_probe_distance(&self) -> &f32;
}

impl NavProbeScoreDataTrait for NavProbeScoreData {
    fn ref_position(&self) -> &CoverScorePosition {
        &self.ref_position
    }
    fn score(&self) -> &f32 {
        &self.score
    }
    fn max_probe_distance(&self) -> &f32 {
        &self.max_probe_distance
    }
}

impl CoverScoreDataTrait for NavProbeScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for NavProbeScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
}

impl super::core::DataContainerTrait for NavProbeScoreData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static NAVPROBESCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NavProbeScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NavProbeScoreData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "RefPosition",
                flags: MemberInfoFlags::new(0),
                field_type: "CoverScorePosition",
                rust_offset: offset_of!(NavProbeScoreData, ref_position),
            },
            FieldInfoData {
                name: "Score",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NavProbeScoreData, score),
            },
            FieldInfoData {
                name: "MaxProbeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NavProbeScoreData, max_probe_distance),
            },
        ],
    }),
    array_type: Some(NAVPROBESCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NavProbeScoreData {
    fn type_info(&self) -> &'static TypeInfo {
        NAVPROBESCOREDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static NAVPROBESCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NavProbeScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("NavProbeScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BehindFollowObjectScoreData {
    pub _glacier_base: CoverScoreDataWithScoreCurve,
    pub max_enemy_distance: f32,
    pub enemy_position_weight_curve: Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>>,
}

pub trait BehindFollowObjectScoreDataTrait: CoverScoreDataWithScoreCurveTrait {
    fn max_enemy_distance(&self) -> &f32;
    fn enemy_position_weight_curve(&self) -> &Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>>;
}

impl BehindFollowObjectScoreDataTrait for BehindFollowObjectScoreData {
    fn max_enemy_distance(&self) -> &f32 {
        &self.max_enemy_distance
    }
    fn enemy_position_weight_curve(&self) -> &Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>> {
        &self.enemy_position_weight_curve
    }
}

impl CoverScoreDataWithScoreCurveTrait for BehindFollowObjectScoreData {
    fn score_curve(&self) -> &Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>> {
        self._glacier_base.score_curve()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
}

impl CoverScoreDataTrait for BehindFollowObjectScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for BehindFollowObjectScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
}

impl super::core::DataContainerTrait for BehindFollowObjectScoreData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static BEHINDFOLLOWOBJECTSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BehindFollowObjectScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BehindFollowObjectScoreData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MaxEnemyDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BehindFollowObjectScoreData, max_enemy_distance),
            },
            FieldInfoData {
                name: "EnemyPositionWeightCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "FloatCurve",
                rust_offset: offset_of!(BehindFollowObjectScoreData, enemy_position_weight_curve),
            },
        ],
    }),
    array_type: Some(BEHINDFOLLOWOBJECTSCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BehindFollowObjectScoreData {
    fn type_info(&self) -> &'static TypeInfo {
        BEHINDFOLLOWOBJECTSCOREDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static BEHINDFOLLOWOBJECTSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BehindFollowObjectScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("BehindFollowObjectScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AngleFromFollowObjectAimDirectionData {
    pub _glacier_base: CoverScoreDataWithScoreCurve,
    pub apply_only_if_aim_direction_stable: bool,
    pub max_follow_object_speed: f32,
}

pub trait AngleFromFollowObjectAimDirectionDataTrait: CoverScoreDataWithScoreCurveTrait {
    fn apply_only_if_aim_direction_stable(&self) -> &bool;
    fn max_follow_object_speed(&self) -> &f32;
}

impl AngleFromFollowObjectAimDirectionDataTrait for AngleFromFollowObjectAimDirectionData {
    fn apply_only_if_aim_direction_stable(&self) -> &bool {
        &self.apply_only_if_aim_direction_stable
    }
    fn max_follow_object_speed(&self) -> &f32 {
        &self.max_follow_object_speed
    }
}

impl CoverScoreDataWithScoreCurveTrait for AngleFromFollowObjectAimDirectionData {
    fn score_curve(&self) -> &Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>> {
        self._glacier_base.score_curve()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
}

impl CoverScoreDataTrait for AngleFromFollowObjectAimDirectionData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for AngleFromFollowObjectAimDirectionData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
}

impl super::core::DataContainerTrait for AngleFromFollowObjectAimDirectionData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static ANGLEFROMFOLLOWOBJECTAIMDIRECTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngleFromFollowObjectAimDirectionData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AngleFromFollowObjectAimDirectionData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ApplyOnlyIfAimDirectionStable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AngleFromFollowObjectAimDirectionData, apply_only_if_aim_direction_stable),
            },
            FieldInfoData {
                name: "MaxFollowObjectSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AngleFromFollowObjectAimDirectionData, max_follow_object_speed),
            },
        ],
    }),
    array_type: Some(ANGLEFROMFOLLOWOBJECTAIMDIRECTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AngleFromFollowObjectAimDirectionData {
    fn type_info(&self) -> &'static TypeInfo {
        ANGLEFROMFOLLOWOBJECTAIMDIRECTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ANGLEFROMFOLLOWOBJECTAIMDIRECTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngleFromFollowObjectAimDirectionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AngleFromFollowObjectAimDirectionData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FollowObjectReticleAvoidanceData {
    pub _glacier_base: CoverScoreDataWithScoreCurve,
    pub apply_only_if_aim_direction_stable: bool,
    pub max_follow_object_speed: f32,
    pub soldier_radius_expansion: f32,
    pub path_look_ahead_distance: f32,
}

pub trait FollowObjectReticleAvoidanceDataTrait: CoverScoreDataWithScoreCurveTrait {
    fn apply_only_if_aim_direction_stable(&self) -> &bool;
    fn max_follow_object_speed(&self) -> &f32;
    fn soldier_radius_expansion(&self) -> &f32;
    fn path_look_ahead_distance(&self) -> &f32;
}

impl FollowObjectReticleAvoidanceDataTrait for FollowObjectReticleAvoidanceData {
    fn apply_only_if_aim_direction_stable(&self) -> &bool {
        &self.apply_only_if_aim_direction_stable
    }
    fn max_follow_object_speed(&self) -> &f32 {
        &self.max_follow_object_speed
    }
    fn soldier_radius_expansion(&self) -> &f32 {
        &self.soldier_radius_expansion
    }
    fn path_look_ahead_distance(&self) -> &f32 {
        &self.path_look_ahead_distance
    }
}

impl CoverScoreDataWithScoreCurveTrait for FollowObjectReticleAvoidanceData {
    fn score_curve(&self) -> &Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>> {
        self._glacier_base.score_curve()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
}

impl CoverScoreDataTrait for FollowObjectReticleAvoidanceData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for FollowObjectReticleAvoidanceData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
}

impl super::core::DataContainerTrait for FollowObjectReticleAvoidanceData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static FOLLOWOBJECTRETICLEAVOIDANCEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FollowObjectReticleAvoidanceData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FollowObjectReticleAvoidanceData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ApplyOnlyIfAimDirectionStable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FollowObjectReticleAvoidanceData, apply_only_if_aim_direction_stable),
            },
            FieldInfoData {
                name: "MaxFollowObjectSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FollowObjectReticleAvoidanceData, max_follow_object_speed),
            },
            FieldInfoData {
                name: "SoldierRadiusExpansion",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FollowObjectReticleAvoidanceData, soldier_radius_expansion),
            },
            FieldInfoData {
                name: "PathLookAheadDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FollowObjectReticleAvoidanceData, path_look_ahead_distance),
            },
        ],
    }),
    array_type: Some(FOLLOWOBJECTRETICLEAVOIDANCEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FollowObjectReticleAvoidanceData {
    fn type_info(&self) -> &'static TypeInfo {
        FOLLOWOBJECTRETICLEAVOIDANCEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FOLLOWOBJECTRETICLEAVOIDANCEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FollowObjectReticleAvoidanceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("FollowObjectReticleAvoidanceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PathAvoidanceScoreData {
    pub _glacier_base: CoverScoreData,
    pub avoid_by_type_data: Vec<super::battle_a_i_shared::CoverQueryPathEnemyAvoidanceByTypeData>,
    pub max_search_distance: f32,
    pub reject_cover_beyond_search_distance: bool,
    pub inner_zone_score: f32,
    pub outer_zone_score: f32,
    pub not_passing_avoidance_area_score: f32,
    pub scores_per_zone: Vec<f32>,
}

pub trait PathAvoidanceScoreDataTrait: CoverScoreDataTrait {
    fn avoid_by_type_data(&self) -> &Vec<super::battle_a_i_shared::CoverQueryPathEnemyAvoidanceByTypeData>;
    fn max_search_distance(&self) -> &f32;
    fn reject_cover_beyond_search_distance(&self) -> &bool;
    fn inner_zone_score(&self) -> &f32;
    fn outer_zone_score(&self) -> &f32;
    fn not_passing_avoidance_area_score(&self) -> &f32;
    fn scores_per_zone(&self) -> &Vec<f32>;
}

impl PathAvoidanceScoreDataTrait for PathAvoidanceScoreData {
    fn avoid_by_type_data(&self) -> &Vec<super::battle_a_i_shared::CoverQueryPathEnemyAvoidanceByTypeData> {
        &self.avoid_by_type_data
    }
    fn max_search_distance(&self) -> &f32 {
        &self.max_search_distance
    }
    fn reject_cover_beyond_search_distance(&self) -> &bool {
        &self.reject_cover_beyond_search_distance
    }
    fn inner_zone_score(&self) -> &f32 {
        &self.inner_zone_score
    }
    fn outer_zone_score(&self) -> &f32 {
        &self.outer_zone_score
    }
    fn not_passing_avoidance_area_score(&self) -> &f32 {
        &self.not_passing_avoidance_area_score
    }
    fn scores_per_zone(&self) -> &Vec<f32> {
        &self.scores_per_zone
    }
}

impl CoverScoreDataTrait for PathAvoidanceScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for PathAvoidanceScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
}

impl super::core::DataContainerTrait for PathAvoidanceScoreData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PATHAVOIDANCESCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathAvoidanceScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PathAvoidanceScoreData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "AvoidByTypeData",
                flags: MemberInfoFlags::new(144),
                field_type: "CoverQueryPathEnemyAvoidanceByTypeData-Array",
                rust_offset: offset_of!(PathAvoidanceScoreData, avoid_by_type_data),
            },
            FieldInfoData {
                name: "MaxSearchDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PathAvoidanceScoreData, max_search_distance),
            },
            FieldInfoData {
                name: "RejectCoverBeyondSearchDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathAvoidanceScoreData, reject_cover_beyond_search_distance),
            },
            FieldInfoData {
                name: "InnerZoneScore",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PathAvoidanceScoreData, inner_zone_score),
            },
            FieldInfoData {
                name: "OuterZoneScore",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PathAvoidanceScoreData, outer_zone_score),
            },
            FieldInfoData {
                name: "NotPassingAvoidanceAreaScore",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PathAvoidanceScoreData, not_passing_avoidance_area_score),
            },
            FieldInfoData {
                name: "ScoresPerZone",
                flags: MemberInfoFlags::new(144),
                field_type: "Float32-Array",
                rust_offset: offset_of!(PathAvoidanceScoreData, scores_per_zone),
            },
        ],
    }),
    array_type: Some(PATHAVOIDANCESCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PathAvoidanceScoreData {
    fn type_info(&self) -> &'static TypeInfo {
        PATHAVOIDANCESCOREDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PATHAVOIDANCESCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathAvoidanceScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("PathAvoidanceScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PathDistanceScoreData {
    pub _glacier_base: CoverScoreDataWithRefPos,
    pub max_search_distance: f32,
    pub precise_path: bool,
    pub reject_cover_beyond_search_distance: bool,
}

pub trait PathDistanceScoreDataTrait: CoverScoreDataWithRefPosTrait {
    fn max_search_distance(&self) -> &f32;
    fn precise_path(&self) -> &bool;
    fn reject_cover_beyond_search_distance(&self) -> &bool;
}

impl PathDistanceScoreDataTrait for PathDistanceScoreData {
    fn max_search_distance(&self) -> &f32 {
        &self.max_search_distance
    }
    fn precise_path(&self) -> &bool {
        &self.precise_path
    }
    fn reject_cover_beyond_search_distance(&self) -> &bool {
        &self.reject_cover_beyond_search_distance
    }
}

impl CoverScoreDataWithRefPosTrait for PathDistanceScoreData {
    fn ref_position(&self) -> &CoverScorePosition {
        self._glacier_base.ref_position()
    }
}

impl CoverScoreDataWithScoreCurveTrait for PathDistanceScoreData {
    fn score_curve(&self) -> &Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>> {
        self._glacier_base.score_curve()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
}

impl CoverScoreDataTrait for PathDistanceScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for PathDistanceScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
}

impl super::core::DataContainerTrait for PathDistanceScoreData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PATHDISTANCESCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathDistanceScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHREFPOS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PathDistanceScoreData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MaxSearchDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PathDistanceScoreData, max_search_distance),
            },
            FieldInfoData {
                name: "PrecisePath",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathDistanceScoreData, precise_path),
            },
            FieldInfoData {
                name: "RejectCoverBeyondSearchDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathDistanceScoreData, reject_cover_beyond_search_distance),
            },
        ],
    }),
    array_type: Some(PATHDISTANCESCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PathDistanceScoreData {
    fn type_info(&self) -> &'static TypeInfo {
        PATHDISTANCESCOREDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PATHDISTANCESCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathDistanceScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("PathDistanceScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RejectUnreachableCoverScoreData {
    pub _glacier_base: CoverScoreData,
    pub ref_position: CoverScorePosition,
}

pub trait RejectUnreachableCoverScoreDataTrait: CoverScoreDataTrait {
    fn ref_position(&self) -> &CoverScorePosition;
}

impl RejectUnreachableCoverScoreDataTrait for RejectUnreachableCoverScoreData {
    fn ref_position(&self) -> &CoverScorePosition {
        &self.ref_position
    }
}

impl CoverScoreDataTrait for RejectUnreachableCoverScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for RejectUnreachableCoverScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
}

impl super::core::DataContainerTrait for RejectUnreachableCoverScoreData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static REJECTUNREACHABLECOVERSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RejectUnreachableCoverScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RejectUnreachableCoverScoreData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "RefPosition",
                flags: MemberInfoFlags::new(0),
                field_type: "CoverScorePosition",
                rust_offset: offset_of!(RejectUnreachableCoverScoreData, ref_position),
            },
        ],
    }),
    array_type: Some(REJECTUNREACHABLECOVERSCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RejectUnreachableCoverScoreData {
    fn type_info(&self) -> &'static TypeInfo {
        REJECTUNREACHABLECOVERSCOREDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static REJECTUNREACHABLECOVERSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RejectUnreachableCoverScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("RejectUnreachableCoverScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DistanceToCorpseData {
    pub _glacier_base: CoverScoreDataWithScoreCurve,
    pub max_time_since_death: f32,
}

pub trait DistanceToCorpseDataTrait: CoverScoreDataWithScoreCurveTrait {
    fn max_time_since_death(&self) -> &f32;
}

impl DistanceToCorpseDataTrait for DistanceToCorpseData {
    fn max_time_since_death(&self) -> &f32 {
        &self.max_time_since_death
    }
}

impl CoverScoreDataWithScoreCurveTrait for DistanceToCorpseData {
    fn score_curve(&self) -> &Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>> {
        self._glacier_base.score_curve()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
}

impl CoverScoreDataTrait for DistanceToCorpseData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for DistanceToCorpseData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
}

impl super::core::DataContainerTrait for DistanceToCorpseData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static DISTANCETOCORPSEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistanceToCorpseData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DistanceToCorpseData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MaxTimeSinceDeath",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DistanceToCorpseData, max_time_since_death),
            },
        ],
    }),
    array_type: Some(DISTANCETOCORPSEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DistanceToCorpseData {
    fn type_info(&self) -> &'static TypeInfo {
        DISTANCETOCORPSEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DISTANCETOCORPSEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistanceToCorpseData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("DistanceToCorpseData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DistanceToClosestEnemyCoverScoreData {
    pub _glacier_base: CoverScoreDataWithScoreCurve,
}

pub trait DistanceToClosestEnemyCoverScoreDataTrait: CoverScoreDataWithScoreCurveTrait {
}

impl DistanceToClosestEnemyCoverScoreDataTrait for DistanceToClosestEnemyCoverScoreData {
}

impl CoverScoreDataWithScoreCurveTrait for DistanceToClosestEnemyCoverScoreData {
    fn score_curve(&self) -> &Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>> {
        self._glacier_base.score_curve()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
}

impl CoverScoreDataTrait for DistanceToClosestEnemyCoverScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for DistanceToClosestEnemyCoverScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
}

impl super::core::DataContainerTrait for DistanceToClosestEnemyCoverScoreData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static DISTANCETOCLOSESTENEMYCOVERSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistanceToClosestEnemyCoverScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DistanceToClosestEnemyCoverScoreData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DISTANCETOCLOSESTENEMYCOVERSCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DistanceToClosestEnemyCoverScoreData {
    fn type_info(&self) -> &'static TypeInfo {
        DISTANCETOCLOSESTENEMYCOVERSCOREDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DISTANCETOCLOSESTENEMYCOVERSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistanceToClosestEnemyCoverScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("DistanceToClosestEnemyCoverScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DistanceToClosestFriendlyScoreData {
    pub _glacier_base: CoverScoreDataWithScoreCurve,
}

pub trait DistanceToClosestFriendlyScoreDataTrait: CoverScoreDataWithScoreCurveTrait {
}

impl DistanceToClosestFriendlyScoreDataTrait for DistanceToClosestFriendlyScoreData {
}

impl CoverScoreDataWithScoreCurveTrait for DistanceToClosestFriendlyScoreData {
    fn score_curve(&self) -> &Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>> {
        self._glacier_base.score_curve()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
}

impl CoverScoreDataTrait for DistanceToClosestFriendlyScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for DistanceToClosestFriendlyScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
}

impl super::core::DataContainerTrait for DistanceToClosestFriendlyScoreData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static DISTANCETOCLOSESTFRIENDLYSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistanceToClosestFriendlyScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DistanceToClosestFriendlyScoreData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DISTANCETOCLOSESTFRIENDLYSCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DistanceToClosestFriendlyScoreData {
    fn type_info(&self) -> &'static TypeInfo {
        DISTANCETOCLOSESTFRIENDLYSCOREDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DISTANCETOCLOSESTFRIENDLYSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistanceToClosestFriendlyScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("DistanceToClosestFriendlyScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ExposureToMultiTargetsScoreData {
    pub _glacier_base: CoverScoreDataWithScoreCurve,
    pub exclude_primary_target: bool,
    pub ref_position_for_target_filtering: CoverScorePosition,
    pub target_significance_distance_curve: Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>>,
    pub max_target_distance: f32,
    pub max_distance_ratio_from_closest_target: f32,
    pub min_target_distance_to_always_be_counted: f32,
    pub max_distance_ratio_from_closest_exposed_target: f32,
    pub max_distance_to_fully_reject_exposed_target: f32,
}

pub trait ExposureToMultiTargetsScoreDataTrait: CoverScoreDataWithScoreCurveTrait {
    fn exclude_primary_target(&self) -> &bool;
    fn ref_position_for_target_filtering(&self) -> &CoverScorePosition;
    fn target_significance_distance_curve(&self) -> &Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>>;
    fn max_target_distance(&self) -> &f32;
    fn max_distance_ratio_from_closest_target(&self) -> &f32;
    fn min_target_distance_to_always_be_counted(&self) -> &f32;
    fn max_distance_ratio_from_closest_exposed_target(&self) -> &f32;
    fn max_distance_to_fully_reject_exposed_target(&self) -> &f32;
}

impl ExposureToMultiTargetsScoreDataTrait for ExposureToMultiTargetsScoreData {
    fn exclude_primary_target(&self) -> &bool {
        &self.exclude_primary_target
    }
    fn ref_position_for_target_filtering(&self) -> &CoverScorePosition {
        &self.ref_position_for_target_filtering
    }
    fn target_significance_distance_curve(&self) -> &Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>> {
        &self.target_significance_distance_curve
    }
    fn max_target_distance(&self) -> &f32 {
        &self.max_target_distance
    }
    fn max_distance_ratio_from_closest_target(&self) -> &f32 {
        &self.max_distance_ratio_from_closest_target
    }
    fn min_target_distance_to_always_be_counted(&self) -> &f32 {
        &self.min_target_distance_to_always_be_counted
    }
    fn max_distance_ratio_from_closest_exposed_target(&self) -> &f32 {
        &self.max_distance_ratio_from_closest_exposed_target
    }
    fn max_distance_to_fully_reject_exposed_target(&self) -> &f32 {
        &self.max_distance_to_fully_reject_exposed_target
    }
}

impl CoverScoreDataWithScoreCurveTrait for ExposureToMultiTargetsScoreData {
    fn score_curve(&self) -> &Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>> {
        self._glacier_base.score_curve()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
}

impl CoverScoreDataTrait for ExposureToMultiTargetsScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for ExposureToMultiTargetsScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
}

impl super::core::DataContainerTrait for ExposureToMultiTargetsScoreData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static EXPOSURETOMULTITARGETSSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExposureToMultiTargetsScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ExposureToMultiTargetsScoreData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ExcludePrimaryTarget",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ExposureToMultiTargetsScoreData, exclude_primary_target),
            },
            FieldInfoData {
                name: "RefPositionForTargetFiltering",
                flags: MemberInfoFlags::new(0),
                field_type: "CoverScorePosition",
                rust_offset: offset_of!(ExposureToMultiTargetsScoreData, ref_position_for_target_filtering),
            },
            FieldInfoData {
                name: "TargetSignificanceDistanceCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "FloatCurve",
                rust_offset: offset_of!(ExposureToMultiTargetsScoreData, target_significance_distance_curve),
            },
            FieldInfoData {
                name: "MaxTargetDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ExposureToMultiTargetsScoreData, max_target_distance),
            },
            FieldInfoData {
                name: "MaxDistanceRatioFromClosestTarget",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ExposureToMultiTargetsScoreData, max_distance_ratio_from_closest_target),
            },
            FieldInfoData {
                name: "MinTargetDistanceToAlwaysBeCounted",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ExposureToMultiTargetsScoreData, min_target_distance_to_always_be_counted),
            },
            FieldInfoData {
                name: "MaxDistanceRatioFromClosestExposedTarget",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ExposureToMultiTargetsScoreData, max_distance_ratio_from_closest_exposed_target),
            },
            FieldInfoData {
                name: "MaxDistanceToFullyRejectExposedTarget",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ExposureToMultiTargetsScoreData, max_distance_to_fully_reject_exposed_target),
            },
        ],
    }),
    array_type: Some(EXPOSURETOMULTITARGETSSCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ExposureToMultiTargetsScoreData {
    fn type_info(&self) -> &'static TypeInfo {
        EXPOSURETOMULTITARGETSSCOREDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static EXPOSURETOMULTITARGETSSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExposureToMultiTargetsScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ExposureToMultiTargetsScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DistanceToMultiTargetsScoreData {
    pub _glacier_base: MultiTargetScorer,
}

pub trait DistanceToMultiTargetsScoreDataTrait: MultiTargetScorerTrait {
}

impl DistanceToMultiTargetsScoreDataTrait for DistanceToMultiTargetsScoreData {
}

impl MultiTargetScorerTrait for DistanceToMultiTargetsScoreData {
    fn exclude_primary_target(&self) -> &bool {
        self._glacier_base.exclude_primary_target()
    }
    fn max_distance_to_target(&self) -> &f32 {
        self._glacier_base.max_distance_to_target()
    }
    fn scoring_mode(&self) -> &MultiTargetScoringMode {
        self._glacier_base.scoring_mode()
    }
}

impl CoverScoreDataWithScoreCurveTrait for DistanceToMultiTargetsScoreData {
    fn score_curve(&self) -> &Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>> {
        self._glacier_base.score_curve()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
}

impl CoverScoreDataTrait for DistanceToMultiTargetsScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for DistanceToMultiTargetsScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
}

impl super::core::DataContainerTrait for DistanceToMultiTargetsScoreData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static DISTANCETOMULTITARGETSSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistanceToMultiTargetsScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MULTITARGETSCORER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DistanceToMultiTargetsScoreData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DISTANCETOMULTITARGETSSCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DistanceToMultiTargetsScoreData {
    fn type_info(&self) -> &'static TypeInfo {
        DISTANCETOMULTITARGETSSCOREDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DISTANCETOMULTITARGETSSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistanceToMultiTargetsScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("DistanceToMultiTargetsScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AngleToMultiTargetsScoreData {
    pub _glacier_base: MultiTargetScorer,
}

pub trait AngleToMultiTargetsScoreDataTrait: MultiTargetScorerTrait {
}

impl AngleToMultiTargetsScoreDataTrait for AngleToMultiTargetsScoreData {
}

impl MultiTargetScorerTrait for AngleToMultiTargetsScoreData {
    fn exclude_primary_target(&self) -> &bool {
        self._glacier_base.exclude_primary_target()
    }
    fn max_distance_to_target(&self) -> &f32 {
        self._glacier_base.max_distance_to_target()
    }
    fn scoring_mode(&self) -> &MultiTargetScoringMode {
        self._glacier_base.scoring_mode()
    }
}

impl CoverScoreDataWithScoreCurveTrait for AngleToMultiTargetsScoreData {
    fn score_curve(&self) -> &Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>> {
        self._glacier_base.score_curve()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
}

impl CoverScoreDataTrait for AngleToMultiTargetsScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for AngleToMultiTargetsScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
}

impl super::core::DataContainerTrait for AngleToMultiTargetsScoreData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static ANGLETOMULTITARGETSSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngleToMultiTargetsScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MULTITARGETSCORER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AngleToMultiTargetsScoreData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(ANGLETOMULTITARGETSSCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AngleToMultiTargetsScoreData {
    fn type_info(&self) -> &'static TypeInfo {
        ANGLETOMULTITARGETSSCOREDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ANGLETOMULTITARGETSSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngleToMultiTargetsScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AngleToMultiTargetsScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MultiTargetScorer {
    pub _glacier_base: CoverScoreDataWithScoreCurve,
    pub exclude_primary_target: bool,
    pub max_distance_to_target: f32,
    pub scoring_mode: MultiTargetScoringMode,
}

pub trait MultiTargetScorerTrait: CoverScoreDataWithScoreCurveTrait {
    fn exclude_primary_target(&self) -> &bool;
    fn max_distance_to_target(&self) -> &f32;
    fn scoring_mode(&self) -> &MultiTargetScoringMode;
}

impl MultiTargetScorerTrait for MultiTargetScorer {
    fn exclude_primary_target(&self) -> &bool {
        &self.exclude_primary_target
    }
    fn max_distance_to_target(&self) -> &f32 {
        &self.max_distance_to_target
    }
    fn scoring_mode(&self) -> &MultiTargetScoringMode {
        &self.scoring_mode
    }
}

impl CoverScoreDataWithScoreCurveTrait for MultiTargetScorer {
    fn score_curve(&self) -> &Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>> {
        self._glacier_base.score_curve()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
}

impl CoverScoreDataTrait for MultiTargetScorer {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for MultiTargetScorer {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
}

impl super::core::DataContainerTrait for MultiTargetScorer {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static MULTITARGETSCORER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiTargetScorer",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MultiTargetScorer as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ExcludePrimaryTarget",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MultiTargetScorer, exclude_primary_target),
            },
            FieldInfoData {
                name: "MaxDistanceToTarget",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MultiTargetScorer, max_distance_to_target),
            },
            FieldInfoData {
                name: "ScoringMode",
                flags: MemberInfoFlags::new(0),
                field_type: "MultiTargetScoringMode",
                rust_offset: offset_of!(MultiTargetScorer, scoring_mode),
            },
        ],
    }),
    array_type: Some(MULTITARGETSCORER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MultiTargetScorer {
    fn type_info(&self) -> &'static TypeInfo {
        MULTITARGETSCORER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MULTITARGETSCORER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiTargetScorer-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("MultiTargetScorer"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum MultiTargetScoringMode {
    #[default]
    MultiTargetScoringMode_MostSignificant = 0,
    MultiTargetScoringMode_Average = 1,
}

pub static MULTITARGETSCORINGMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiTargetScoringMode",
    flags: MemberInfoFlags::new(49429),
    module: "BattleAI",
    data: TypeInfoData::Enum,
    array_type: Some(MULTITARGETSCORINGMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MultiTargetScoringMode {
    fn type_info(&self) -> &'static TypeInfo {
        MULTITARGETSCORINGMODE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MULTITARGETSCORINGMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiTargetScoringMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("MultiTargetScoringMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TowardsPreferredWeaponRangeScoreData {
    pub _glacier_base: CoverScoreData,
    pub outside_preferred_range_score_curve: Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>>,
    pub inside_preferred_range_score_curve: Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>>,
}

pub trait TowardsPreferredWeaponRangeScoreDataTrait: CoverScoreDataTrait {
    fn outside_preferred_range_score_curve(&self) -> &Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>>;
    fn inside_preferred_range_score_curve(&self) -> &Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>>;
}

impl TowardsPreferredWeaponRangeScoreDataTrait for TowardsPreferredWeaponRangeScoreData {
    fn outside_preferred_range_score_curve(&self) -> &Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>> {
        &self.outside_preferred_range_score_curve
    }
    fn inside_preferred_range_score_curve(&self) -> &Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>> {
        &self.inside_preferred_range_score_curve
    }
}

impl CoverScoreDataTrait for TowardsPreferredWeaponRangeScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for TowardsPreferredWeaponRangeScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
}

impl super::core::DataContainerTrait for TowardsPreferredWeaponRangeScoreData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TOWARDSPREFERREDWEAPONRANGESCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TowardsPreferredWeaponRangeScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TowardsPreferredWeaponRangeScoreData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "OutsidePreferredRangeScoreCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "FloatCurve",
                rust_offset: offset_of!(TowardsPreferredWeaponRangeScoreData, outside_preferred_range_score_curve),
            },
            FieldInfoData {
                name: "InsidePreferredRangeScoreCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "FloatCurve",
                rust_offset: offset_of!(TowardsPreferredWeaponRangeScoreData, inside_preferred_range_score_curve),
            },
        ],
    }),
    array_type: Some(TOWARDSPREFERREDWEAPONRANGESCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TowardsPreferredWeaponRangeScoreData {
    fn type_info(&self) -> &'static TypeInfo {
        TOWARDSPREFERREDWEAPONRANGESCOREDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TOWARDSPREFERREDWEAPONRANGESCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TowardsPreferredWeaponRangeScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("TowardsPreferredWeaponRangeScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PreferredWeaponRangeScoreData {
    pub _glacier_base: CoverScoreDataWithScoreCurve,
}

pub trait PreferredWeaponRangeScoreDataTrait: CoverScoreDataWithScoreCurveTrait {
}

impl PreferredWeaponRangeScoreDataTrait for PreferredWeaponRangeScoreData {
}

impl CoverScoreDataWithScoreCurveTrait for PreferredWeaponRangeScoreData {
    fn score_curve(&self) -> &Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>> {
        self._glacier_base.score_curve()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
}

impl CoverScoreDataTrait for PreferredWeaponRangeScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for PreferredWeaponRangeScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
}

impl super::core::DataContainerTrait for PreferredWeaponRangeScoreData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PREFERREDWEAPONRANGESCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PreferredWeaponRangeScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PreferredWeaponRangeScoreData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PREFERREDWEAPONRANGESCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PreferredWeaponRangeScoreData {
    fn type_info(&self) -> &'static TypeInfo {
        PREFERREDWEAPONRANGESCOREDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PREFERREDWEAPONRANGESCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PreferredWeaponRangeScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("PreferredWeaponRangeScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ProjectedDistanceScoreData {
    pub _glacier_base: CoverScoreDataWithRefPos,
    pub ref_direction: CoverScoreDirection,
    pub flip_ref_direction: bool,
}

pub trait ProjectedDistanceScoreDataTrait: CoverScoreDataWithRefPosTrait {
    fn ref_direction(&self) -> &CoverScoreDirection;
    fn flip_ref_direction(&self) -> &bool;
}

impl ProjectedDistanceScoreDataTrait for ProjectedDistanceScoreData {
    fn ref_direction(&self) -> &CoverScoreDirection {
        &self.ref_direction
    }
    fn flip_ref_direction(&self) -> &bool {
        &self.flip_ref_direction
    }
}

impl CoverScoreDataWithRefPosTrait for ProjectedDistanceScoreData {
    fn ref_position(&self) -> &CoverScorePosition {
        self._glacier_base.ref_position()
    }
}

impl CoverScoreDataWithScoreCurveTrait for ProjectedDistanceScoreData {
    fn score_curve(&self) -> &Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>> {
        self._glacier_base.score_curve()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
}

impl CoverScoreDataTrait for ProjectedDistanceScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for ProjectedDistanceScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
}

impl super::core::DataContainerTrait for ProjectedDistanceScoreData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PROJECTEDDISTANCESCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProjectedDistanceScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHREFPOS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ProjectedDistanceScoreData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "RefDirection",
                flags: MemberInfoFlags::new(0),
                field_type: "CoverScoreDirection",
                rust_offset: offset_of!(ProjectedDistanceScoreData, ref_direction),
            },
            FieldInfoData {
                name: "FlipRefDirection",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ProjectedDistanceScoreData, flip_ref_direction),
            },
        ],
    }),
    array_type: Some(PROJECTEDDISTANCESCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProjectedDistanceScoreData {
    fn type_info(&self) -> &'static TypeInfo {
        PROJECTEDDISTANCESCOREDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PROJECTEDDISTANCESCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProjectedDistanceScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ProjectedDistanceScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum CoverScoreDirection {
    #[default]
    CoverScoreDirection_SoldierToTarget = 0,
    CoverScoreDirection_SoldierToFollowObject = 1,
    CoverScoreDirection_WorldUpDirection = 2,
}

pub static COVERSCOREDIRECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoverScoreDirection",
    flags: MemberInfoFlags::new(49429),
    module: "BattleAI",
    data: TypeInfoData::Enum,
    array_type: Some(COVERSCOREDIRECTION_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CoverScoreDirection {
    fn type_info(&self) -> &'static TypeInfo {
        COVERSCOREDIRECTION_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static COVERSCOREDIRECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoverScoreDirection-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("CoverScoreDirection"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DistanceToActorScoreData {
    pub _glacier_base: CoverScoreDataWithRefPos,
}

pub trait DistanceToActorScoreDataTrait: CoverScoreDataWithRefPosTrait {
}

impl DistanceToActorScoreDataTrait for DistanceToActorScoreData {
}

impl CoverScoreDataWithRefPosTrait for DistanceToActorScoreData {
    fn ref_position(&self) -> &CoverScorePosition {
        self._glacier_base.ref_position()
    }
}

impl CoverScoreDataWithScoreCurveTrait for DistanceToActorScoreData {
    fn score_curve(&self) -> &Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>> {
        self._glacier_base.score_curve()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
}

impl CoverScoreDataTrait for DistanceToActorScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for DistanceToActorScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
}

impl super::core::DataContainerTrait for DistanceToActorScoreData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static DISTANCETOACTORSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistanceToActorScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHREFPOS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DistanceToActorScoreData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DISTANCETOACTORSCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DistanceToActorScoreData {
    fn type_info(&self) -> &'static TypeInfo {
        DISTANCETOACTORSCOREDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DISTANCETOACTORSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistanceToActorScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("DistanceToActorScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AngleFromPathTrajectoryScoreData {
    pub _glacier_base: CoverScoreDataWithScoreCurve,
    pub path_look_ahead_distance: f32,
}

pub trait AngleFromPathTrajectoryScoreDataTrait: CoverScoreDataWithScoreCurveTrait {
    fn path_look_ahead_distance(&self) -> &f32;
}

impl AngleFromPathTrajectoryScoreDataTrait for AngleFromPathTrajectoryScoreData {
    fn path_look_ahead_distance(&self) -> &f32 {
        &self.path_look_ahead_distance
    }
}

impl CoverScoreDataWithScoreCurveTrait for AngleFromPathTrajectoryScoreData {
    fn score_curve(&self) -> &Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>> {
        self._glacier_base.score_curve()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
}

impl CoverScoreDataTrait for AngleFromPathTrajectoryScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for AngleFromPathTrajectoryScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
}

impl super::core::DataContainerTrait for AngleFromPathTrajectoryScoreData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static ANGLEFROMPATHTRAJECTORYSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngleFromPathTrajectoryScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AngleFromPathTrajectoryScoreData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "PathLookAheadDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AngleFromPathTrajectoryScoreData, path_look_ahead_distance),
            },
        ],
    }),
    array_type: Some(ANGLEFROMPATHTRAJECTORYSCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AngleFromPathTrajectoryScoreData {
    fn type_info(&self) -> &'static TypeInfo {
        ANGLEFROMPATHTRAJECTORYSCOREDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ANGLEFROMPATHTRAJECTORYSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngleFromPathTrajectoryScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AngleFromPathTrajectoryScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AngleFromReferenceDirectionScoreData {
    pub _glacier_base: CoverScoreDataWithScoreCurve,
    pub ref_dir_from_pos: CoverScorePosition,
    pub ref_dir_to_pos: CoverScorePosition,
}

pub trait AngleFromReferenceDirectionScoreDataTrait: CoverScoreDataWithScoreCurveTrait {
    fn ref_dir_from_pos(&self) -> &CoverScorePosition;
    fn ref_dir_to_pos(&self) -> &CoverScorePosition;
}

impl AngleFromReferenceDirectionScoreDataTrait for AngleFromReferenceDirectionScoreData {
    fn ref_dir_from_pos(&self) -> &CoverScorePosition {
        &self.ref_dir_from_pos
    }
    fn ref_dir_to_pos(&self) -> &CoverScorePosition {
        &self.ref_dir_to_pos
    }
}

impl CoverScoreDataWithScoreCurveTrait for AngleFromReferenceDirectionScoreData {
    fn score_curve(&self) -> &Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>> {
        self._glacier_base.score_curve()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
}

impl CoverScoreDataTrait for AngleFromReferenceDirectionScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for AngleFromReferenceDirectionScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
}

impl super::core::DataContainerTrait for AngleFromReferenceDirectionScoreData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static ANGLEFROMREFERENCEDIRECTIONSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngleFromReferenceDirectionScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AngleFromReferenceDirectionScoreData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "RefDirFromPos",
                flags: MemberInfoFlags::new(0),
                field_type: "CoverScorePosition",
                rust_offset: offset_of!(AngleFromReferenceDirectionScoreData, ref_dir_from_pos),
            },
            FieldInfoData {
                name: "RefDirToPos",
                flags: MemberInfoFlags::new(0),
                field_type: "CoverScorePosition",
                rust_offset: offset_of!(AngleFromReferenceDirectionScoreData, ref_dir_to_pos),
            },
        ],
    }),
    array_type: Some(ANGLEFROMREFERENCEDIRECTIONSCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AngleFromReferenceDirectionScoreData {
    fn type_info(&self) -> &'static TypeInfo {
        ANGLEFROMREFERENCEDIRECTIONSCOREDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ANGLEFROMREFERENCEDIRECTIONSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngleFromReferenceDirectionScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AngleFromReferenceDirectionScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AngleToActorScoreData2 {
    pub _glacier_base: CoverScoreData,
    pub ref_position: CoverScorePosition,
    pub scores_for_filter: Vec<Option<Arc<Mutex<dyn ScoreCurveForFilterTrait>>>>,
}

pub trait AngleToActorScoreData2Trait: CoverScoreDataTrait {
    fn ref_position(&self) -> &CoverScorePosition;
    fn scores_for_filter(&self) -> &Vec<Option<Arc<Mutex<dyn ScoreCurveForFilterTrait>>>>;
}

impl AngleToActorScoreData2Trait for AngleToActorScoreData2 {
    fn ref_position(&self) -> &CoverScorePosition {
        &self.ref_position
    }
    fn scores_for_filter(&self) -> &Vec<Option<Arc<Mutex<dyn ScoreCurveForFilterTrait>>>> {
        &self.scores_for_filter
    }
}

impl CoverScoreDataTrait for AngleToActorScoreData2 {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for AngleToActorScoreData2 {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
}

impl super::core::DataContainerTrait for AngleToActorScoreData2 {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static ANGLETOACTORSCOREDATA2_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngleToActorScoreData2",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AngleToActorScoreData2 as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "RefPosition",
                flags: MemberInfoFlags::new(0),
                field_type: "CoverScorePosition",
                rust_offset: offset_of!(AngleToActorScoreData2, ref_position),
            },
            FieldInfoData {
                name: "ScoresForFilter",
                flags: MemberInfoFlags::new(144),
                field_type: "ScoreCurveForFilter-Array",
                rust_offset: offset_of!(AngleToActorScoreData2, scores_for_filter),
            },
        ],
    }),
    array_type: Some(ANGLETOACTORSCOREDATA2_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AngleToActorScoreData2 {
    fn type_info(&self) -> &'static TypeInfo {
        ANGLETOACTORSCOREDATA2_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ANGLETOACTORSCOREDATA2_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngleToActorScoreData2-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AngleToActorScoreData2"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ScoreCurveForFilter {
    pub _glacier_base: super::core::DataContainer,
    pub score_curve: Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>>,
    pub runtime_filter: u32,
}

pub trait ScoreCurveForFilterTrait: super::core::DataContainerTrait {
    fn score_curve(&self) -> &Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>>;
    fn runtime_filter(&self) -> &u32;
}

impl ScoreCurveForFilterTrait for ScoreCurveForFilter {
    fn score_curve(&self) -> &Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>> {
        &self.score_curve
    }
    fn runtime_filter(&self) -> &u32 {
        &self.runtime_filter
    }
}

impl super::core::DataContainerTrait for ScoreCurveForFilter {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static SCORECURVEFORFILTER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScoreCurveForFilter",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ScoreCurveForFilter as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ScoreCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "FloatCurve",
                rust_offset: offset_of!(ScoreCurveForFilter, score_curve),
            },
            FieldInfoData {
                name: "RuntimeFilter",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ScoreCurveForFilter, runtime_filter),
            },
        ],
    }),
    array_type: Some(SCORECURVEFORFILTER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ScoreCurveForFilter {
    fn type_info(&self) -> &'static TypeInfo {
        SCORECURVEFORFILTER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SCORECURVEFORFILTER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScoreCurveForFilter-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ScoreCurveForFilter"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AngleToActorScoreData {
    pub _glacier_base: CoverScoreDataWithRefPos,
}

pub trait AngleToActorScoreDataTrait: CoverScoreDataWithRefPosTrait {
}

impl AngleToActorScoreDataTrait for AngleToActorScoreData {
}

impl CoverScoreDataWithRefPosTrait for AngleToActorScoreData {
    fn ref_position(&self) -> &CoverScorePosition {
        self._glacier_base.ref_position()
    }
}

impl CoverScoreDataWithScoreCurveTrait for AngleToActorScoreData {
    fn score_curve(&self) -> &Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>> {
        self._glacier_base.score_curve()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
}

impl CoverScoreDataTrait for AngleToActorScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for AngleToActorScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
}

impl super::core::DataContainerTrait for AngleToActorScoreData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static ANGLETOACTORSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngleToActorScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHREFPOS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AngleToActorScoreData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(ANGLETOACTORSCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AngleToActorScoreData {
    fn type_info(&self) -> &'static TypeInfo {
        ANGLETOACTORSCOREDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ANGLETOACTORSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngleToActorScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AngleToActorScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CoverScoreDataWithRefPos {
    pub _glacier_base: CoverScoreDataWithScoreCurve,
    pub ref_position: CoverScorePosition,
}

pub trait CoverScoreDataWithRefPosTrait: CoverScoreDataWithScoreCurveTrait {
    fn ref_position(&self) -> &CoverScorePosition;
}

impl CoverScoreDataWithRefPosTrait for CoverScoreDataWithRefPos {
    fn ref_position(&self) -> &CoverScorePosition {
        &self.ref_position
    }
}

impl CoverScoreDataWithScoreCurveTrait for CoverScoreDataWithRefPos {
    fn score_curve(&self) -> &Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>> {
        self._glacier_base.score_curve()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
}

impl CoverScoreDataTrait for CoverScoreDataWithRefPos {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for CoverScoreDataWithRefPos {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
}

impl super::core::DataContainerTrait for CoverScoreDataWithRefPos {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static COVERSCOREDATAWITHREFPOS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoverScoreDataWithRefPos",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoverScoreDataWithRefPos as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "RefPosition",
                flags: MemberInfoFlags::new(0),
                field_type: "CoverScorePosition",
                rust_offset: offset_of!(CoverScoreDataWithRefPos, ref_position),
            },
        ],
    }),
    array_type: Some(COVERSCOREDATAWITHREFPOS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CoverScoreDataWithRefPos {
    fn type_info(&self) -> &'static TypeInfo {
        COVERSCOREDATAWITHREFPOS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static COVERSCOREDATAWITHREFPOS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoverScoreDataWithRefPos-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("CoverScoreDataWithRefPos"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CoverScoreDataWithScoreCurve {
    pub _glacier_base: CoverScoreData,
    pub score_curve: Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>>,
    pub score_curve_scale: f32,
    pub score_curve_max_y: f32,
}

pub trait CoverScoreDataWithScoreCurveTrait: CoverScoreDataTrait {
    fn score_curve(&self) -> &Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>>;
    fn score_curve_scale(&self) -> &f32;
    fn score_curve_max_y(&self) -> &f32;
}

impl CoverScoreDataWithScoreCurveTrait for CoverScoreDataWithScoreCurve {
    fn score_curve(&self) -> &Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>> {
        &self.score_curve
    }
    fn score_curve_scale(&self) -> &f32 {
        &self.score_curve_scale
    }
    fn score_curve_max_y(&self) -> &f32 {
        &self.score_curve_max_y
    }
}

impl CoverScoreDataTrait for CoverScoreDataWithScoreCurve {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for CoverScoreDataWithScoreCurve {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
}

impl super::core::DataContainerTrait for CoverScoreDataWithScoreCurve {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static COVERSCOREDATAWITHSCORECURVE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoverScoreDataWithScoreCurve",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoverScoreDataWithScoreCurve as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ScoreCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "FloatCurve",
                rust_offset: offset_of!(CoverScoreDataWithScoreCurve, score_curve),
            },
            FieldInfoData {
                name: "ScoreCurveScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CoverScoreDataWithScoreCurve, score_curve_scale),
            },
            FieldInfoData {
                name: "ScoreCurveMaxY",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CoverScoreDataWithScoreCurve, score_curve_max_y),
            },
        ],
    }),
    array_type: Some(COVERSCOREDATAWITHSCORECURVE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CoverScoreDataWithScoreCurve {
    fn type_info(&self) -> &'static TypeInfo {
        COVERSCOREDATAWITHSCORECURVE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static COVERSCOREDATAWITHSCORECURVE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoverScoreDataWithScoreCurve-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("CoverScoreDataWithScoreCurve"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CurrentCoverBonusScoreData {
    pub _glacier_base: CoverScoreData,
    pub bonus_score: f32,
}

pub trait CurrentCoverBonusScoreDataTrait: CoverScoreDataTrait {
    fn bonus_score(&self) -> &f32;
}

impl CurrentCoverBonusScoreDataTrait for CurrentCoverBonusScoreData {
    fn bonus_score(&self) -> &f32 {
        &self.bonus_score
    }
}

impl CoverScoreDataTrait for CurrentCoverBonusScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for CurrentCoverBonusScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
}

impl super::core::DataContainerTrait for CurrentCoverBonusScoreData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static CURRENTCOVERBONUSSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CurrentCoverBonusScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CurrentCoverBonusScoreData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "BonusScore",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CurrentCoverBonusScoreData, bonus_score),
            },
        ],
    }),
    array_type: Some(CURRENTCOVERBONUSSCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CurrentCoverBonusScoreData {
    fn type_info(&self) -> &'static TypeInfo {
        CURRENTCOVERBONUSSCOREDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CURRENTCOVERBONUSSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CurrentCoverBonusScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("CurrentCoverBonusScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CoverFilterScoreData {
    pub _glacier_base: CoverScoreData,
    pub matching_score: f32,
}

pub trait CoverFilterScoreDataTrait: CoverScoreDataTrait {
    fn matching_score(&self) -> &f32;
}

impl CoverFilterScoreDataTrait for CoverFilterScoreData {
    fn matching_score(&self) -> &f32 {
        &self.matching_score
    }
}

impl CoverScoreDataTrait for CoverFilterScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for CoverFilterScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
}

impl super::core::DataContainerTrait for CoverFilterScoreData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static COVERFILTERSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoverFilterScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoverFilterScoreData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MatchingScore",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CoverFilterScoreData, matching_score),
            },
        ],
    }),
    array_type: Some(COVERFILTERSCOREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CoverFilterScoreData {
    fn type_info(&self) -> &'static TypeInfo {
        COVERFILTERSCOREDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static COVERFILTERSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoverFilterScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("CoverFilterScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CoverScoreData {
    pub _glacier_base: super::battle_a_i_shared::CoverScoreDataBase,
    pub comment: String,
    pub enabled: bool,
}

pub trait CoverScoreDataTrait: super::battle_a_i_shared::CoverScoreDataBaseTrait {
    fn comment(&self) -> &String;
    fn enabled(&self) -> &bool;
}

impl CoverScoreDataTrait for CoverScoreData {
    fn comment(&self) -> &String {
        &self.comment
    }
    fn enabled(&self) -> &bool {
        &self.enabled
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for CoverScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
}

impl super::core::DataContainerTrait for CoverScoreData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static COVERSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoverScoreData",
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::battle_a_i_shared::COVERSCOREDATABASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoverScoreData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Comment",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(CoverScoreData, comment),
            },
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CoverScoreData, enabled),
            },
        ],
    }),
    array_type: Some(COVERSCOREDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CoverScoreData {
    fn type_info(&self) -> &'static TypeInfo {
        COVERSCOREDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static COVERSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoverScoreData-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("CoverScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum CoverScorePosition {
    #[default]
    CoverScorePosition_Soldier = 0,
    CoverScorePosition_Target = 1,
    CoverScorePosition_FollowObject = 2,
    CoverScorePosition_CriticalThreat = 3,
    CoverScorePosition_Count = 4,
}

pub static COVERSCOREPOSITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoverScorePosition",
    flags: MemberInfoFlags::new(49429),
    module: "BattleAI",
    data: TypeInfoData::Enum,
    array_type: Some(COVERSCOREPOSITION_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CoverScorePosition {
    fn type_info(&self) -> &'static TypeInfo {
        COVERSCOREPOSITION_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static COVERSCOREPOSITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoverScorePosition-Array",
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("CoverScorePosition"),
    array_type: None,
    alignment: 8,
};


