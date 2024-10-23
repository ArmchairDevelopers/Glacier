use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
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

#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1088656223,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerSimpleDriverComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSimpleDriverComponent as Default>::default())),
            create_boxed: || Box::new(<ServerSimpleDriverComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERSIMPLEDRIVERCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSimpleDriverComponent-Array",
    name_hash: 4080568427,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerSimpleDriverComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2598199207,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTINGENTITY_TYPE_INFO),
        super_class_offset: offset_of!(WSServerDogFightingEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WSServerDogFightingEntity as Default>::default())),
            create_boxed: || Box::new(<WSServerDogFightingEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static WSSERVERDOGFIGHTINGENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WSServerDogFightingEntity-Array",
    name_hash: 3990122771,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("WSServerDogFightingEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3530536758,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        super_class_offset: offset_of!(ServerStrafeRunManeuverEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerStrafeRunManeuverEntity as Default>::default())),
            create_boxed: || Box::new(<ServerStrafeRunManeuverEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERSTRAFERUNMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStrafeRunManeuverEntity-Array",
    name_hash: 2372266882,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerStrafeRunManeuverEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1959249747,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        super_class_offset: offset_of!(ServerStallTurnManeuverEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerStallTurnManeuverEntity as Default>::default())),
            create_boxed: || Box::new(<ServerStallTurnManeuverEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERSTALLTURNMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStallTurnManeuverEntity-Array",
    name_hash: 4071928423,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerStallTurnManeuverEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 162471753,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        super_class_offset: offset_of!(ServerSplitSManeuverEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSplitSManeuverEntity as Default>::default())),
            create_boxed: || Box::new(<ServerSplitSManeuverEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERSPLITSMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSplitSManeuverEntity-Array",
    name_hash: 922641789,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerSplitSManeuverEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1170004482,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        super_class_offset: offset_of!(ServerSpinDescentManeuverEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSpinDescentManeuverEntity as Default>::default())),
            create_boxed: || Box::new(<ServerSpinDescentManeuverEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERSPINDESCENTMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSpinDescentManeuverEntity-Array",
    name_hash: 3099801910,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerSpinDescentManeuverEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2209742163,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        super_class_offset: offset_of!(ServerSideToSideManeuverEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSideToSideManeuverEntity as Default>::default())),
            create_boxed: || Box::new(<ServerSideToSideManeuverEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERSIDETOSIDEMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSideToSideManeuverEntity-Array",
    name_hash: 3304286311,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerSideToSideManeuverEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 765247881,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerSetWaypointsEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSetWaypointsEntity as Default>::default())),
            create_boxed: || Box::new(<ServerSetWaypointsEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERSETWAYPOINTSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSetWaypointsEntity-Array",
    name_hash: 2054207293,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerSetWaypointsEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 972076886,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        super_class_offset: offset_of!(ServerProtectBaseManeuverEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerProtectBaseManeuverEntity as Default>::default())),
            create_boxed: || Box::new(<ServerProtectBaseManeuverEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERPROTECTBASEMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerProtectBaseManeuverEntity-Array",
    name_hash: 106609890,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerProtectBaseManeuverEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2160224096,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerPilotEntityBase, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPilotEntityBase as Default>::default())),
            create_boxed: || Box::new(<ServerPilotEntityBase as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERPILOTENTITYBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPilotEntityBase-Array",
    name_hash: 3854359892,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerPilotEntityBase"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2087056757,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERPILOTENTITYBASE_TYPE_INFO),
        super_class_offset: offset_of!(ServerPilotEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPilotEntity as Default>::default())),
            create_boxed: || Box::new(<ServerPilotEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERPILOTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPilotEntity-Array",
    name_hash: 1964041793,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerPilotEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2313391613,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        super_class_offset: offset_of!(ServerManeuverSelectorEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerManeuverSelectorEntity as Default>::default())),
            create_boxed: || Box::new(<ServerManeuverSelectorEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERMANEUVERSELECTORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerManeuverSelectorEntity-Array",
    name_hash: 1073625801,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerManeuverSelectorEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1688239876,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        super_class_offset: offset_of!(ServerImmelmannManeuverEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerImmelmannManeuverEntity as Default>::default())),
            create_boxed: || Box::new(<ServerImmelmannManeuverEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERIMMELMANNMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerImmelmannManeuverEntity-Array",
    name_hash: 111285040,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerImmelmannManeuverEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 392411776,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERPILOTENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerHeavyPlanePilotEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerHeavyPlanePilotEntity as Default>::default())),
            create_boxed: || Box::new(<ServerHeavyPlanePilotEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERHEAVYPLANEPILOTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerHeavyPlanePilotEntity-Array",
    name_hash: 2273591476,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerHeavyPlanePilotEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 259300681,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        super_class_offset: offset_of!(ServerFollowWaypointsManeuverEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerFollowWaypointsManeuverEntity as Default>::default())),
            create_boxed: || Box::new(<ServerFollowWaypointsManeuverEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERFOLLOWWAYPOINTSMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerFollowWaypointsManeuverEntity-Array",
    name_hash: 3865731965,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerFollowWaypointsManeuverEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2415273792,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        super_class_offset: offset_of!(ServerFlyToManeuverEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerFlyToManeuverEntity as Default>::default())),
            create_boxed: || Box::new(<ServerFlyToManeuverEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERFLYTOMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerFlyToManeuverEntity-Array",
    name_hash: 4222172660,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerFlyToManeuverEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1535128878,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        super_class_offset: offset_of!(ServerEnforceAltitudeManeuverEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerEnforceAltitudeManeuverEntity as Default>::default())),
            create_boxed: || Box::new(<ServerEnforceAltitudeManeuverEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERENFORCEALTITUDEMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEnforceAltitudeManeuverEntity-Array",
    name_hash: 1061780890,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerEnforceAltitudeManeuverEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1418785765,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerDogFightManeuverEntityBase, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerDogFightManeuverEntityBase as Default>::default())),
            create_boxed: || Box::new(<ServerDogFightManeuverEntityBase as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERDOGFIGHTMANEUVERENTITYBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDogFightManeuverEntityBase-Array",
    name_hash: 982383825,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerDogFightManeuverEntityBase"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3053015171,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerDogFightingEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerDogFightingEntity as Default>::default())),
            create_boxed: || Box::new(<ServerDogFightingEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERDOGFIGHTINGENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDogFightingEntity-Array",
    name_hash: 4151681847,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerDogFightingEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1643475160,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERMANEUVERSELECTORENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerDefensiveManeuverSelectorEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerDefensiveManeuverSelectorEntity as Default>::default())),
            create_boxed: || Box::new(<ServerDefensiveManeuverSelectorEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERDEFENSIVEMANEUVERSELECTORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDefensiveManeuverSelectorEntity-Array",
    name_hash: 4218145644,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerDefensiveManeuverSelectorEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2126358863,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        super_class_offset: offset_of!(ServerCreateDistanceManeuverEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCreateDistanceManeuverEntity as Default>::default())),
            create_boxed: || Box::new(<ServerCreateDistanceManeuverEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERCREATEDISTANCEMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreateDistanceManeuverEntity-Array",
    name_hash: 1580750459,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerCreateDistanceManeuverEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1885496490,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        super_class_offset: offset_of!(ServerCollisionAvoidanceManeuverEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCollisionAvoidanceManeuverEntity as Default>::default())),
            create_boxed: || Box::new(<ServerCollisionAvoidanceManeuverEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERCOLLISIONAVOIDANCEMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCollisionAvoidanceManeuverEntity-Array",
    name_hash: 1118936862,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerCollisionAvoidanceManeuverEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1415605207,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        super_class_offset: offset_of!(ServerBasicDefensiveManeuverEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerBasicDefensiveManeuverEntity as Default>::default())),
            create_boxed: || Box::new(<ServerBasicDefensiveManeuverEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERBASICDEFENSIVEMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBasicDefensiveManeuverEntity-Array",
    name_hash: 295068899,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerBasicDefensiveManeuverEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2022326490,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        super_class_offset: offset_of!(ServerBasicAttackManeuverEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerBasicAttackManeuverEntity as Default>::default())),
            create_boxed: || Box::new(<ServerBasicAttackManeuverEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERBASICATTACKMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBasicAttackManeuverEntity-Array",
    name_hash: 2627015790,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerBasicAttackManeuverEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2415372703,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTMANEUVERENTITYBASE_TYPE_INFO),
        super_class_offset: offset_of!(ServerBarrelRollManeuverEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerBarrelRollManeuverEntity as Default>::default())),
            create_boxed: || Box::new(<ServerBarrelRollManeuverEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERBARRELROLLMANEUVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBarrelRollManeuverEntity-Array",
    name_hash: 174362155,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerBarrelRollManeuverEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1157061125,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAirTargetSelectorEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAirTargetSelectorEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAirTargetSelectorEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAIRTARGETSELECTORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAirTargetSelectorEntity-Array",
    name_hash: 490471857,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAirTargetSelectorEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2831267907,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAirCollisionAvoidanceEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAirCollisionAvoidanceEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAirCollisionAvoidanceEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAIRCOLLISIONAVOIDANCEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAirCollisionAvoidanceEntity-Array",
    name_hash: 1091345271,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAirCollisionAvoidanceEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3849596551,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERDOGFIGHTINGENTITY_TYPE_INFO),
        super_class_offset: offset_of!(BFServerDogFightingEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BFServerDogFightingEntity as Default>::default())),
            create_boxed: || Box::new(<BFServerDogFightingEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static BFSERVERDOGFIGHTINGENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BFServerDogFightingEntity-Array",
    name_hash: 2086580019,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("BFServerDogFightingEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3195625741,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerWaypointsWalkerEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerWaypointsWalkerEntity as Default>::default())),
            create_boxed: || Box::new(<ServerWaypointsWalkerEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERWAYPOINTSWALKERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWaypointsWalkerEntity-Array",
    name_hash: 1248240825,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerWaypointsWalkerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3258031592,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerAIProximityReactionsComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIProximityReactionsComponent as Default>::default())),
            create_boxed: || Box::new(<ServerAIProximityReactionsComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAIPROXIMITYREACTIONSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIProximityReactionsComponent-Array",
    name_hash: 3336196060,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIProximityReactionsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2192276872,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerAILocoComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAILocoComponent as Default>::default())),
            create_boxed: || Box::new(<ServerAILocoComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAILOCOCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAILocoComponent-Array",
    name_hash: 3939606460,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAILocoComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2489186440,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(AIBlockerEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AIBlockerEntity as Default>::default())),
            create_boxed: || Box::new(<AIBlockerEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static AIBLOCKERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIBlockerEntity-Array",
    name_hash: 892623548,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AIBlockerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2147857332,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientAIProximityReactionsComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAIProximityReactionsComponent as Default>::default())),
            create_boxed: || Box::new(<ClientAIProximityReactionsComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTAIPROXIMITYREACTIONSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAIProximityReactionsComponent-Array",
    name_hash: 2372849408,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ClientAIProximityReactionsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3887527252,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientAILocoComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAILocoComponent as Default>::default())),
            create_boxed: || Box::new(<ClientAILocoComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTAILOCOCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAILocoComponent-Array",
    name_hash: 1897857504,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ClientAILocoComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 881208737,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(SpatialAnalyzer, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SpatialAnalyzer as Default>::default())),
            create_boxed: || Box::new(<SpatialAnalyzer as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SPATIALANALYZER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpatialAnalyzer-Array",
    name_hash: 3337066517,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("SpatialAnalyzer"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3314078757,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerPlayerVehicleProximityEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPlayerVehicleProximityEntity as Default>::default())),
            create_boxed: || Box::new(<ServerPlayerVehicleProximityEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERPLAYERVEHICLEPROXIMITYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPlayerVehicleProximityEntity-Array",
    name_hash: 1932315793,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerPlayerVehicleProximityEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1655542428,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerInvestigateSettingsOverride, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerInvestigateSettingsOverride as Default>::default())),
            create_boxed: || Box::new(<ServerInvestigateSettingsOverride as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERINVESTIGATESETTINGSOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerInvestigateSettingsOverride-Array",
    name_hash: 3156339368,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerInvestigateSettingsOverride"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1320489767,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerDamageModifierEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerDamageModifierEntity as Default>::default())),
            create_boxed: || Box::new(<ServerDamageModifierEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERDAMAGEMODIFIERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDamageModifierEntity-Array",
    name_hash: 1552204947,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerDamageModifierEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3043835647,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAttackPointEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAttackPointEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAttackPointEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERATTACKPOINTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAttackPointEntity-Array",
    name_hash: 3421824203,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAttackPointEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1627398877,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAIVehicleCombatEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIVehicleCombatEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAIVehicleCombatEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAIVEHICLECOMBATENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIVehicleCombatEntity-Array",
    name_hash: 1672904425,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIVehicleCombatEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1971847811,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAITemplateFilterEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAITemplateFilterEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAITemplateFilterEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAITEMPLATEFILTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAITemplateFilterEntity-Array",
    name_hash: 2814498103,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAITemplateFilterEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 834920050,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAITeleportEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAITeleportEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAITeleportEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAITELEPORTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAITeleportEntity-Array",
    name_hash: 1325045062,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAITeleportEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 152683094,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAISystemEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAISystemEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAISystemEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAISYSTEMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISystemEntity-Array",
    name_hash: 2430601186,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAISystemEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2610997124,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAIStateEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIStateEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAIStateEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAISTATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIStateEntity-Array",
    name_hash: 339647408,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIStateEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2844426480,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAISoundEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAISoundEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAISoundEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAISOUNDENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISoundEntity-Array",
    name_hash: 1092935364,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAISoundEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 809233344,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIORDERENTITYBASE_TYPE_INFO),
        super_class_offset: offset_of!(ServerAICancelOrder, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAICancelOrder as Default>::default())),
            create_boxed: || Box::new(<ServerAICancelOrder as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAICANCELORDER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAICancelOrder-Array",
    name_hash: 256926324,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAICancelOrder"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3987577029,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIORDERENTITYBASE_TYPE_INFO),
        super_class_offset: offset_of!(ServerAIGotoPlaceOrderEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIGotoPlaceOrderEntityData as Default>::default())),
            create_boxed: || Box::new(<ServerAIGotoPlaceOrderEntityData as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAIGOTOPLACEORDERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIGotoPlaceOrderEntityData-Array",
    name_hash: 1142051057,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIGotoPlaceOrderEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1217672551,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIORDERENTITYBASE_TYPE_INFO),
        super_class_offset: offset_of!(ServerAIFollowWaypointsOrder, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIFollowWaypointsOrder as Default>::default())),
            create_boxed: || Box::new(<ServerAIFollowWaypointsOrder as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAIFOLLOWWAYPOINTSORDER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIFollowWaypointsOrder-Array",
    name_hash: 2231932755,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIFollowWaypointsOrder"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3747142952,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAIOrderEntityBase, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIOrderEntityBase as Default>::default())),
            create_boxed: || Box::new(<ServerAIOrderEntityBase as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAIORDERENTITYBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIOrderEntityBase-Array",
    name_hash: 3246915228,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIOrderEntityBase"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 13496697,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAISelfDestructEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAISelfDestructEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAISelfDestructEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAISELFDESTRUCTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISelfDestructEntity-Array",
    name_hash: 3754266701,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAISelfDestructEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1239062983,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAICoverZonesOverrideEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAICoverZonesOverrideEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAICoverZonesOverrideEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAICOVERZONESOVERRIDEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAICoverZonesOverrideEntity-Array",
    name_hash: 3790097139,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAICoverZonesOverrideEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1113139416,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAIAwarenessEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIAwarenessEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAIAwarenessEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAIAWARENESSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIAwarenessEntity-Array",
    name_hash: 901350252,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIAwarenessEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1400704961,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAIWeaponSlotOverrideEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIWeaponSlotOverrideEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAIWeaponSlotOverrideEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAIWEAPONSLOTOVERRIDEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIWeaponSlotOverrideEntity-Array",
    name_hash: 3122616821,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIWeaponSlotOverrideEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 4201147000,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAITargetCoordinatorFilterEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAITargetCoordinatorFilterEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAITargetCoordinatorFilterEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAITARGETCOORDINATORFILTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAITargetCoordinatorFilterEntity-Array",
    name_hash: 2225816652,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAITargetCoordinatorFilterEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1736740280,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAITargetCoordinatorEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAITargetCoordinatorEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAITargetCoordinatorEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAITARGETCOORDINATORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAITargetCoordinatorEntity-Array",
    name_hash: 3563640588,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAITargetCoordinatorEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3763801990,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerCloakingModifierEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCloakingModifierEntity as Default>::default())),
            create_boxed: || Box::new(<ServerCloakingModifierEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERCLOAKINGMODIFIERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCloakingModifierEntity-Array",
    name_hash: 3419356338,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerCloakingModifierEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 4175337072,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerSensingAreaModifierEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSensingAreaModifierEntity as Default>::default())),
            create_boxed: || Box::new(<ServerSensingAreaModifierEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERSENSINGAREAMODIFIERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSensingAreaModifierEntity-Array",
    name_hash: 3281196100,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerSensingAreaModifierEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 4138742176,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAIStealthEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIStealthEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAIStealthEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAISTEALTHENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIStealthEntity-Array",
    name_hash: 346904340,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIStealthEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1463307692,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAIBuddyFollowEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIBuddyFollowEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAIBuddyFollowEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAIBUDDYFOLLOWENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIBuddyFollowEntity-Array",
    name_hash: 2107211032,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIBuddyFollowEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2688242231,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAIFollowObjectEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIFollowObjectEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAIFollowObjectEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAIFOLLOWOBJECTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIFollowObjectEntity-Array",
    name_hash: 2668694403,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIFollowObjectEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2254769601,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERWITHSHAPEENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAIPreferredAreaEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIPreferredAreaEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAIPreferredAreaEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAIPREFERREDAREAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIPreferredAreaEntity-Array",
    name_hash: 2257442805,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIPreferredAreaEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3192857159,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERWITHSHAPEENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAISoundAreaEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAISoundAreaEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAISoundAreaEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAISOUNDAREAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISoundAreaEntity-Array",
    name_hash: 1254175603,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAISoundAreaEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 848373498,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAICombatGroupEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAICombatGroupEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAICombatGroupEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAICOMBATGROUPENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAICombatGroupEntity-Array",
    name_hash: 373004750,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAICombatGroupEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 332724564,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAICoverQueryEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAICoverQueryEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAICoverQueryEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAICOVERQUERYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAICoverQueryEntity-Array",
    name_hash: 2762999264,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAICoverQueryEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2480823547,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAITacticEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAITacticEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAITacticEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAITACTICENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAITacticEntity-Array",
    name_hash: 755291343,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAITacticEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2855305451,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAIShootAtTargetsEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIShootAtTargetsEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAIShootAtTargetsEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAISHOOTATTARGETSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIShootAtTargetsEntity-Array",
    name_hash: 2580492511,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIShootAtTargetsEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2752478720,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAIUseWaypointsEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIUseWaypointsEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAIUseWaypointsEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAIUSEWAYPOINTSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIUseWaypointsEntity-Array",
    name_hash: 3251938356,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIUseWaypointsEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2100500317,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAIUseCoverEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIUseCoverEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAIUseCoverEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAIUSECOVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIUseCoverEntity-Array",
    name_hash: 1550766953,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIUseCoverEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3086974245,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAIWeaponOverrideEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIWeaponOverrideEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAIWeaponOverrideEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAIWEAPONOVERRIDEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIWeaponOverrideEntity-Array",
    name_hash: 406730129,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIWeaponOverrideEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3162182279,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAIVehicleBehaviorEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIVehicleBehaviorEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAIVehicleBehaviorEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAIVEHICLEBEHAVIORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIVehicleBehaviorEntity-Array",
    name_hash: 3724579635,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIVehicleBehaviorEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2765600164,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAIHearingParameterEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIHearingParameterEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAIHearingParameterEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAIHEARINGPARAMETERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIHearingParameterEntity-Array",
    name_hash: 2896660752,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIHearingParameterEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3322406257,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAISensingParameterEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAISensingParameterEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAISensingParameterEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAISENSINGPARAMETERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISensingParameterEntity-Array",
    name_hash: 723986501,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAISensingParameterEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2753939035,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAIIdleBehaviorEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIIdleBehaviorEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAIIdleBehaviorEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAIIDLEBEHAVIORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIIdleBehaviorEntity-Array",
    name_hash: 4174728559,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIIdleBehaviorEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3064010498,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAITargetingEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAITargetingEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAITargetingEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAITARGETINGENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAITargetingEntity-Array",
    name_hash: 3844159030,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAITargetingEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2000993161,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAICombatBehaviorEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAICombatBehaviorEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAICombatBehaviorEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAICOMBATBEHAVIORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAICombatBehaviorEntity-Array",
    name_hash: 2225877309,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAICombatBehaviorEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3970292021,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERWITHSHAPEENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAIFollowAreaEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIFollowAreaEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAIFollowAreaEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAIFOLLOWAREAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIFollowAreaEntity-Array",
    name_hash: 1858886017,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIFollowAreaEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3828152447,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERWITHSHAPEENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAIForbiddenAreaEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIForbiddenAreaEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAIForbiddenAreaEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAIFORBIDDENAREAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIForbiddenAreaEntity-Array",
    name_hash: 553238091,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIForbiddenAreaEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2552034275,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERWITHSHAPEENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAIFriendlyAreaEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIFriendlyAreaEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAIFriendlyAreaEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAIFRIENDLYAREAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIFriendlyAreaEntity-Array",
    name_hash: 1871087575,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIFriendlyAreaEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2066623329,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERWITHSHAPEENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAIFlankingCorridorEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIFlankingCorridorEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAIFlankingCorridorEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAIFLANKINGCORRIDORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIFlankingCorridorEntity-Array",
    name_hash: 1104619093,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIFlankingCorridorEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2377798442,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERWITHSHAPEENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAISearchAreaEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAISearchAreaEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAISearchAreaEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAISEARCHAREAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISearchAreaEntity-Array",
    name_hash: 3873598878,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAISearchAreaEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3981644236,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERWITHSHAPEENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAIDefendAreaEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIDefendAreaEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAIDefendAreaEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAIDEFENDAREAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIDefendAreaEntity-Array",
    name_hash: 2322272888,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIDefendAreaEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 628513975,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERAIPARAMETERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAIParameterWithShapeEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIParameterWithShapeEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAIParameterWithShapeEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAIPARAMETERWITHSHAPEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIParameterWithShapeEntity-Array",
    name_hash: 671741443,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIParameterWithShapeEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2615773850,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAIParameterEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIParameterEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAIParameterEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAIPARAMETERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIParameterEntity-Array",
    name_hash: 2565107118,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIParameterEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 4016394414,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAIObstacleControllerEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIObstacleControllerEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAIObstacleControllerEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAIOBSTACLECONTROLLERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIObstacleControllerEntity-Array",
    name_hash: 1282750746,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIObstacleControllerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1999190213,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAIKillCounterEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIKillCounterEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAIKillCounterEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAIKILLCOUNTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIKillCounterEntity-Array",
    name_hash: 600174833,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIKillCounterEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1383893571,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAIFirePatternEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIFirePatternEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAIFirePatternEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAIFIREPATTERNENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIFirePatternEntity-Array",
    name_hash: 337440119,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIFirePatternEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 67737695,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAIEncounterManagerEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIEncounterManagerEntity as Default>::default())),
            create_boxed: || Box::new(<ServerAIEncounterManagerEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAIENCOUNTERMANAGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIEncounterManagerEntity-Array",
    name_hash: 3725479275,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIEncounterManagerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1467373877,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerAIDebugProxy, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIDebugProxy as Default>::default())),
            create_boxed: || Box::new(<ServerAIDebugProxy as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAIDEBUGPROXY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIDebugProxy-Array",
    name_hash: 3556563329,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIDebugProxy"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2173746245,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerActionEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerActionEntity as Default>::default())),
            create_boxed: || Box::new(<ServerActionEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERACTIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerActionEntity-Array",
    name_hash: 2845971569,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerActionEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 4150134405,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(AIConcealmentVolumeEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AIConcealmentVolumeEntity as Default>::default())),
            create_boxed: || Box::new(<AIConcealmentVolumeEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static AICONCEALMENTVOLUMEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIConcealmentVolumeEntity-Array",
    name_hash: 571608625,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AIConcealmentVolumeEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3173534559,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientAITemplateFilterEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAITemplateFilterEntity as Default>::default())),
            create_boxed: || Box::new(<ClientAITemplateFilterEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTAITEMPLATEFILTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAITemplateFilterEntity-Array",
    name_hash: 1848066155,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ClientAITemplateFilterEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3823887192,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientAIStateEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAIStateEntity as Default>::default())),
            create_boxed: || Box::new(<ClientAIStateEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTAISTATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAIStateEntity-Array",
    name_hash: 2316180972,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ClientAIStateEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 654881872,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientAIPhysicsDrivenAnimationEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAIPhysicsDrivenAnimationEntity as Default>::default())),
            create_boxed: || Box::new(<ClientAIPhysicsDrivenAnimationEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTAIPHYSICSDRIVENANIMATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAIPhysicsDrivenAnimationEntity-Array",
    name_hash: 3141817060,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ClientAIPhysicsDrivenAnimationEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3320925514,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientAICollisionAvoidanceSetupEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAICollisionAvoidanceSetupEntity as Default>::default())),
            create_boxed: || Box::new(<ClientAICollisionAvoidanceSetupEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTAICOLLISIONAVOIDANCESETUPENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAICollisionAvoidanceSetupEntity-Array",
    name_hash: 2208348414,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ClientAICollisionAvoidanceSetupEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 522410777,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientActionEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientActionEntity as Default>::default())),
            create_boxed: || Box::new(<ClientActionEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTACTIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientActionEntity-Array",
    name_hash: 3547805869,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ClientActionEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2231810317,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IACTIONCONDITIONEVALUATOR_TYPE_INFO),
        super_class_offset: offset_of!(AITemplateConditionEvaluator, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AITemplateConditionEvaluator as Default>::default())),
            create_boxed: || Box::new(<AITemplateConditionEvaluator as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static AITEMPLATECONDITIONEVALUATOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AITemplateConditionEvaluator-Array",
    name_hash: 194378937,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AITemplateConditionEvaluator"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2294321682,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IACTIONCONDITIONEVALUATOR_TYPE_INFO),
        super_class_offset: offset_of!(InsideVolumesConditionEvaluator, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InsideVolumesConditionEvaluator as Default>::default())),
            create_boxed: || Box::new(<InsideVolumesConditionEvaluator as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static INSIDEVOLUMESCONDITIONEVALUATOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InsideVolumesConditionEvaluator-Array",
    name_hash: 2638461222,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("InsideVolumesConditionEvaluator"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3624370712,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IACTIONCONDITIONEVALUATOR_TYPE_INFO),
        super_class_offset: offset_of!(ProbabilityConditionEvaluator, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ProbabilityConditionEvaluator as Default>::default())),
            create_boxed: || Box::new(<ProbabilityConditionEvaluator as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static PROBABILITYCONDITIONEVALUATOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProbabilityConditionEvaluator-Array",
    name_hash: 181521452,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ProbabilityConditionEvaluator"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2697384817,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IACTIONCONDITIONEVALUATOR_TYPE_INFO),
        super_class_offset: offset_of!(FacingConditionEvaluator, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FacingConditionEvaluator as Default>::default())),
            create_boxed: || Box::new(<FacingConditionEvaluator as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static FACINGCONDITIONEVALUATOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FacingConditionEvaluator-Array",
    name_hash: 1088460357,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("FacingConditionEvaluator"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3786260137,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IACTIONCONDITIONEVALUATOR_TYPE_INFO),
        super_class_offset: offset_of!(AIOrderCoordinatorConditionEvaluator, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AIOrderCoordinatorConditionEvaluator as Default>::default())),
            create_boxed: || Box::new(<AIOrderCoordinatorConditionEvaluator as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static AIORDERCOORDINATORCONDITIONEVALUATOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIOrderCoordinatorConditionEvaluator-Array",
    name_hash: 1949779229,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AIOrderCoordinatorConditionEvaluator"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2093175530,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IACTIONCONDITIONEVALUATOR_TYPE_INFO),
        super_class_offset: offset_of!(AIStateConditionEvaluator, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AIStateConditionEvaluator as Default>::default())),
            create_boxed: || Box::new(<AIStateConditionEvaluator as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static AISTATECONDITIONEVALUATOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIStateConditionEvaluator-Array",
    name_hash: 4039900638,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AIStateConditionEvaluator"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 4077733130,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IACTIONCONDITIONEVALUATOR_TYPE_INFO),
        super_class_offset: offset_of!(RangeConditionEvaluator, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RangeConditionEvaluator as Default>::default())),
            create_boxed: || Box::new(<RangeConditionEvaluator as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static RANGECONDITIONEVALUATOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RangeConditionEvaluator-Array",
    name_hash: 4227285054,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("RangeConditionEvaluator"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct IActionConditionEvaluator {
}

pub trait IActionConditionEvaluatorTrait: TypeObject {
}

impl IActionConditionEvaluatorTrait for IActionConditionEvaluator {
}

pub static IACTIONCONDITIONEVALUATOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IActionConditionEvaluator",
    name_hash: 2078258018,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IActionConditionEvaluator as Default>::default())),
            create_boxed: || Box::new(<IActionConditionEvaluator as Default>::default()),
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


pub static IACTIONCONDITIONEVALUATOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IActionConditionEvaluator-Array",
    name_hash: 3863962198,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("IActionConditionEvaluator"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3213607596,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerPointOfInterestComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPointOfInterestComponent as Default>::default())),
            create_boxed: || Box::new(<ServerPointOfInterestComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERPOINTOFINTERESTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPointOfInterestComponent-Array",
    name_hash: 4126192152,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerPointOfInterestComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1225933046,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerCoverEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCoverEntity as Default>::default())),
            create_boxed: || Box::new(<ServerCoverEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERCOVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCoverEntity-Array",
    name_hash: 2456323522,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerCoverEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2128692905,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerCoverGroupEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCoverGroupEntity as Default>::default())),
            create_boxed: || Box::new(<ServerCoverGroupEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERCOVERGROUPENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCoverGroupEntity-Array",
    name_hash: 3689991453,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerCoverGroupEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3029209530,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerAIVehicleAimingComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIVehicleAimingComponent as Default>::default())),
            create_boxed: || Box::new(<ServerAIVehicleAimingComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAIVEHICLEAIMINGCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIVehicleAimingComponent-Array",
    name_hash: 2780681230,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIVehicleAimingComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3779887382,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerAITargetComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAITargetComponent as Default>::default())),
            create_boxed: || Box::new(<ServerAITargetComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAITARGETCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAITargetComponent-Array",
    name_hash: 2986776098,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAITargetComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3873541673,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerAISuppressWeaponFiringComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAISuppressWeaponFiringComponent as Default>::default())),
            create_boxed: || Box::new(<ServerAISuppressWeaponFiringComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAISUPPRESSWEAPONFIRINGCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISuppressWeaponFiringComponent-Array",
    name_hash: 669562013,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAISuppressWeaponFiringComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2046643275,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerAISpottingComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAISpottingComponent as Default>::default())),
            create_boxed: || Box::new(<ServerAISpottingComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAISPOTTINGCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISpottingComponent-Array",
    name_hash: 3223751551,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAISpottingComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2147201168,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerAISmokeVolumeComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAISmokeVolumeComponent as Default>::default())),
            create_boxed: || Box::new(<ServerAISmokeVolumeComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAISMOKEVOLUMECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAISmokeVolumeComponent-Array",
    name_hash: 189759140,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAISmokeVolumeComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 380367026,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerAIProjectileComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIProjectileComponent as Default>::default())),
            create_boxed: || Box::new(<ServerAIProjectileComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAIPROJECTILECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIProjectileComponent-Array",
    name_hash: 570922758,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIProjectileComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2095593619,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerAIOrderCoordinatorComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIOrderCoordinatorComponent as Default>::default())),
            create_boxed: || Box::new(<ServerAIOrderCoordinatorComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAIORDERCOORDINATORCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIOrderCoordinatorComponent-Array",
    name_hash: 1334137639,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIOrderCoordinatorComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2445722787,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerAIMeleeComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIMeleeComponent as Default>::default())),
            create_boxed: || Box::new(<ServerAIMeleeComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAIMELEECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIMeleeComponent-Array",
    name_hash: 1211492375,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIMeleeComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3642706227,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerAIEntryComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIEntryComponent as Default>::default())),
            create_boxed: || Box::new(<ServerAIEntryComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAIENTRYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIEntryComponent-Array",
    name_hash: 93492359,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIEntryComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1912123842,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerAICustomInputComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAICustomInputComponent as Default>::default())),
            create_boxed: || Box::new(<ServerAICustomInputComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAICUSTOMINPUTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAICustomInputComponent-Array",
    name_hash: 3544326518,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAICustomInputComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1692355244,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerAIBucketSystemComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIBucketSystemComponent as Default>::default())),
            create_boxed: || Box::new(<ServerAIBucketSystemComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAIBUCKETSYSTEMCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIBucketSystemComponent-Array",
    name_hash: 4257888792,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIBucketSystemComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 4240243433,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerAIAnchorToPointComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAIAnchorToPointComponent as Default>::default())),
            create_boxed: || Box::new(<ServerAIAnchorToPointComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERAIANCHORTOPOINTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAIAnchorToPointComponent-Array",
    name_hash: 1285747165,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerAIAnchorToPointComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1122960663,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientAISpottingComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAISpottingComponent as Default>::default())),
            create_boxed: || Box::new(<ClientAISpottingComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTAISPOTTINGCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAISpottingComponent-Array",
    name_hash: 148057507,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ClientAISpottingComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 660369774,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientAIProjectileComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAIProjectileComponent as Default>::default())),
            create_boxed: || Box::new(<ClientAIProjectileComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTAIPROJECTILECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAIProjectileComponent-Array",
    name_hash: 1537321562,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ClientAIProjectileComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1136438863,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientAIOrderCoordinatorComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAIOrderCoordinatorComponent as Default>::default())),
            create_boxed: || Box::new(<ClientAIOrderCoordinatorComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTAIORDERCOORDINATORCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAIOrderCoordinatorComponent-Array",
    name_hash: 2309430139,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ClientAIOrderCoordinatorComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3471044991,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientAIMeleeComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAIMeleeComponent as Default>::default())),
            create_boxed: || Box::new(<ClientAIMeleeComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTAIMELEECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAIMeleeComponent-Array",
    name_hash: 558331723,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ClientAIMeleeComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2486283189,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientAIAnchorToPointComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAIAnchorToPointComponent as Default>::default())),
            create_boxed: || Box::new(<ClientAIAnchorToPointComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTAIANCHORTOPOINTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAIAnchorToPointComponent-Array",
    name_hash: 361499649,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ClientAIAnchorToPointComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1508264940,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(CoverScoreModifierEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoverScoreModifierEntity as Default>::default())),
            create_boxed: || Box::new(<CoverScoreModifierEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static COVERSCOREMODIFIERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoverScoreModifierEntity-Array",
    name_hash: 2350459864,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("CoverScoreModifierEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1334888310,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PATHLINKENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerPathLinkEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPathLinkEntity as Default>::default())),
            create_boxed: || Box::new(<ServerPathLinkEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERPATHLINKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPathLinkEntity-Array",
    name_hash: 2018634818,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerPathLinkEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2759305935,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerPathfindingStreamEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPathfindingStreamEntity as Default>::default())),
            create_boxed: || Box::new(<ServerPathfindingStreamEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERPATHFINDINGSTREAMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPathfindingStreamEntity-Array",
    name_hash: 3738007547,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerPathfindingStreamEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 455747240,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerNavPowerObstacleComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerNavPowerObstacleComponent as Default>::default())),
            create_boxed: || Box::new(<ServerNavPowerObstacleComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERNAVPOWEROBSTACLECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerNavPowerObstacleComponent-Array",
    name_hash: 2928306204,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerNavPowerObstacleComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 39076755,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITY_TYPE_INFO),
        super_class_offset: offset_of!(PathLinkEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PathLinkEntity as Default>::default())),
            create_boxed: || Box::new(<PathLinkEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static PATHLINKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathLinkEntity-Array",
    name_hash: 4136476199,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("PathLinkEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1976268160,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerWaypointTriggerEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerWaypointTriggerEntity as Default>::default())),
            create_boxed: || Box::new(<ServerWaypointTriggerEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERWAYPOINTTRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWaypointTriggerEntity-Array",
    name_hash: 2710322100,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerWaypointTriggerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 4128533907,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerPathFollowingComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPathFollowingComponent as Default>::default())),
            create_boxed: || Box::new(<ServerPathFollowingComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERPATHFOLLOWINGCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPathFollowingComponent-Array",
    name_hash: 3359983655,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerPathFollowingComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2675173660,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerPathfindingOverride, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPathfindingOverride as Default>::default())),
            create_boxed: || Box::new(<ServerPathfindingOverride as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERPATHFINDINGOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPathfindingOverride-Array",
    name_hash: 2196709672,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerPathfindingOverride"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 470023322,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerFollowWaypointsEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerFollowWaypointsEntity as Default>::default())),
            create_boxed: || Box::new(<ServerFollowWaypointsEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERFOLLOWWAYPOINTSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerFollowWaypointsEntity-Array",
    name_hash: 3012321198,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerFollowWaypointsEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2522781151,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerFollowObjectEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerFollowObjectEntity as Default>::default())),
            create_boxed: || Box::new(<ServerFollowObjectEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERFOLLOWOBJECTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerFollowObjectEntity-Array",
    name_hash: 2861263595,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ServerFollowObjectEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn reference_transform(&self) -> &super::core::LinearTransform;
    fn reference_transform_mut(&mut self) -> &mut super::core::LinearTransform;
    fn analysis_f_o_v(&self) -> &f32;
    fn analysis_f_o_v_mut(&mut self) -> &mut f32;
    fn sweep_steps(&self) -> &u32;
    fn sweep_steps_mut(&mut self) -> &mut u32;
    fn edge_distance_threshold(&self) -> &f32;
    fn edge_distance_threshold_mut(&mut self) -> &mut f32;
    fn near_limit_distance(&self) -> &f32;
    fn near_limit_distance_mut(&mut self) -> &mut f32;
    fn far_limit_distance(&self) -> &f32;
    fn far_limit_distance_mut(&mut self) -> &mut f32;
}

impl SpatialAnalyzerDataTrait for SpatialAnalyzerData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn reference_transform(&self) -> &super::core::LinearTransform {
        &self.reference_transform
    }
    fn reference_transform_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.reference_transform
    }
    fn analysis_f_o_v(&self) -> &f32 {
        &self.analysis_f_o_v
    }
    fn analysis_f_o_v_mut(&mut self) -> &mut f32 {
        &mut self.analysis_f_o_v
    }
    fn sweep_steps(&self) -> &u32 {
        &self.sweep_steps
    }
    fn sweep_steps_mut(&mut self) -> &mut u32 {
        &mut self.sweep_steps
    }
    fn edge_distance_threshold(&self) -> &f32 {
        &self.edge_distance_threshold
    }
    fn edge_distance_threshold_mut(&mut self) -> &mut f32 {
        &mut self.edge_distance_threshold
    }
    fn near_limit_distance(&self) -> &f32 {
        &self.near_limit_distance
    }
    fn near_limit_distance_mut(&mut self) -> &mut f32 {
        &mut self.near_limit_distance
    }
    fn far_limit_distance(&self) -> &f32 {
        &self.far_limit_distance
    }
    fn far_limit_distance_mut(&mut self) -> &mut f32 {
        &mut self.far_limit_distance
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
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for SpatialAnalyzerData {
}

impl super::core::DataContainerTrait for SpatialAnalyzerData {
}

pub static SPATIALANALYZERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpatialAnalyzerData",
    name_hash: 1091940561,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(SpatialAnalyzerData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SpatialAnalyzerData as Default>::default())),
            create_boxed: || Box::new(<SpatialAnalyzerData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                name_hash: 229961746,
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(SpatialAnalyzerData, realm),
            },
            FieldInfoData {
                name: "ReferenceTransform",
                name_hash: 2231497250,
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(SpatialAnalyzerData, reference_transform),
            },
            FieldInfoData {
                name: "AnalysisFOV",
                name_hash: 1666260904,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SpatialAnalyzerData, analysis_f_o_v),
            },
            FieldInfoData {
                name: "SweepSteps",
                name_hash: 2857693904,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(SpatialAnalyzerData, sweep_steps),
            },
            FieldInfoData {
                name: "EdgeDistanceThreshold",
                name_hash: 1932875314,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SpatialAnalyzerData, edge_distance_threshold),
            },
            FieldInfoData {
                name: "NearLimitDistance",
                name_hash: 502671147,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SpatialAnalyzerData, near_limit_distance),
            },
            FieldInfoData {
                name: "FarLimitDistance",
                name_hash: 2293594054,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SPATIALANALYZERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpatialAnalyzerData-Array",
    name_hash: 2843706085,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("SpatialAnalyzerData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn enabled_mut(&mut self) -> &mut bool;
    fn only_accept_linked_spawners(&self) -> &bool;
    fn only_accept_linked_spawners_mut(&mut self) -> &mut bool;
    fn team_id(&self) -> &super::gameplay_sim::TeamId;
    fn team_id_mut(&mut self) -> &mut super::gameplay_sim::TeamId;
    fn max_target_count_for_l_o_s_check(&self) -> &i32;
    fn max_target_count_for_l_o_s_check_mut(&mut self) -> &mut i32;
    fn navmesh_layer(&self) -> &i32;
    fn navmesh_layer_mut(&mut self) -> &mut i32;
    fn ground_transform_horizontal_offset(&self) -> &f32;
    fn ground_transform_horizontal_offset_mut(&mut self) -> &mut f32;
}

impl AttackPointEntityDataTrait for AttackPointEntityData {
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
    fn only_accept_linked_spawners(&self) -> &bool {
        &self.only_accept_linked_spawners
    }
    fn only_accept_linked_spawners_mut(&mut self) -> &mut bool {
        &mut self.only_accept_linked_spawners
    }
    fn team_id(&self) -> &super::gameplay_sim::TeamId {
        &self.team_id
    }
    fn team_id_mut(&mut self) -> &mut super::gameplay_sim::TeamId {
        &mut self.team_id
    }
    fn max_target_count_for_l_o_s_check(&self) -> &i32 {
        &self.max_target_count_for_l_o_s_check
    }
    fn max_target_count_for_l_o_s_check_mut(&mut self) -> &mut i32 {
        &mut self.max_target_count_for_l_o_s_check
    }
    fn navmesh_layer(&self) -> &i32 {
        &self.navmesh_layer
    }
    fn navmesh_layer_mut(&mut self) -> &mut i32 {
        &mut self.navmesh_layer
    }
    fn ground_transform_horizontal_offset(&self) -> &f32 {
        &self.ground_transform_horizontal_offset
    }
    fn ground_transform_horizontal_offset_mut(&mut self) -> &mut f32 {
        &mut self.ground_transform_horizontal_offset
    }
}

impl super::entity::SpatialEntityDataTrait for AttackPointEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
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
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for AttackPointEntityData {
}

impl super::core::DataContainerTrait for AttackPointEntityData {
}

pub static ATTACKPOINTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AttackPointEntityData",
    name_hash: 3569094666,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(AttackPointEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AttackPointEntityData as Default>::default())),
            create_boxed: || Box::new(<AttackPointEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Enabled",
                name_hash: 2662400,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AttackPointEntityData, enabled),
            },
            FieldInfoData {
                name: "OnlyAcceptLinkedSpawners",
                name_hash: 1723023823,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AttackPointEntityData, only_accept_linked_spawners),
            },
            FieldInfoData {
                name: "TeamId",
                name_hash: 3220374101,
                flags: MemberInfoFlags::new(0),
                field_type: "TeamId",
                rust_offset: offset_of!(AttackPointEntityData, team_id),
            },
            FieldInfoData {
                name: "MaxTargetCountForLOSCheck",
                name_hash: 933233518,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AttackPointEntityData, max_target_count_for_l_o_s_check),
            },
            FieldInfoData {
                name: "NavmeshLayer",
                name_hash: 3234693996,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AttackPointEntityData, navmesh_layer),
            },
            FieldInfoData {
                name: "GroundTransformHorizontalOffset",
                name_hash: 3477250271,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static ATTACKPOINTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AttackPointEntityData-Array",
    name_hash: 1448233790,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AttackPointEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ActionEntityData {
    pub _glacier_base: super::entity::SpatialEntityData,
    pub enabled: bool,
    pub conditions: Vec<Option<LockedTypeObject /* ActionCondition */>>,
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
    fn enabled_mut(&mut self) -> &mut bool;
    fn conditions(&self) -> &Vec<Option<LockedTypeObject /* ActionCondition */>>;
    fn conditions_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* ActionCondition */>>;
    fn move_to_action_before_executing(&self) -> &bool;
    fn move_to_action_before_executing_mut(&mut self) -> &mut bool;
    fn align_to_action_before_executing(&self) -> &bool;
    fn align_to_action_before_executing_mut(&mut self) -> &mut bool;
    fn wait_time_before_executing(&self) -> &f32;
    fn wait_time_before_executing_mut(&mut self) -> &mut f32;
    fn cooldown_time(&self) -> &f32;
    fn cooldown_time_mut(&mut self) -> &mut f32;
    fn timeout_duration(&self) -> &f32;
    fn timeout_duration_mut(&mut self) -> &mut f32;
    fn abort_when_alerted(&self) -> &bool;
    fn abort_when_alerted_mut(&mut self) -> &mut bool;
    fn abort_when_timed_out(&self) -> &bool;
    fn abort_when_timed_out_mut(&mut self) -> &mut bool;
    fn valid_in_state(&self) -> &ValidInState;
    fn valid_in_state_mut(&mut self) -> &mut ValidInState;
    fn action_priority(&self) -> &ActionPriority;
    fn action_priority_mut(&mut self) -> &mut ActionPriority;
    fn should_be_executed_by_closest_a_i(&self) -> &bool;
    fn should_be_executed_by_closest_a_i_mut(&mut self) -> &mut bool;
    fn only_execute_for_fitness_valid_a_i(&self) -> &bool;
    fn only_execute_for_fitness_valid_a_i_mut(&mut self) -> &mut bool;
    fn use_actual_path_distance(&self) -> &bool;
    fn use_actual_path_distance_mut(&mut self) -> &mut bool;
    fn is_linked_action(&self) -> &bool;
    fn is_linked_action_mut(&mut self) -> &mut bool;
    fn restrict_to_linked_soldiers(&self) -> &bool;
    fn restrict_to_linked_soldiers_mut(&mut self) -> &mut bool;
}

impl ActionEntityDataTrait for ActionEntityData {
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
    fn conditions(&self) -> &Vec<Option<LockedTypeObject /* ActionCondition */>> {
        &self.conditions
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* ActionCondition */>> {
        &mut self.conditions
    }
    fn move_to_action_before_executing(&self) -> &bool {
        &self.move_to_action_before_executing
    }
    fn move_to_action_before_executing_mut(&mut self) -> &mut bool {
        &mut self.move_to_action_before_executing
    }
    fn align_to_action_before_executing(&self) -> &bool {
        &self.align_to_action_before_executing
    }
    fn align_to_action_before_executing_mut(&mut self) -> &mut bool {
        &mut self.align_to_action_before_executing
    }
    fn wait_time_before_executing(&self) -> &f32 {
        &self.wait_time_before_executing
    }
    fn wait_time_before_executing_mut(&mut self) -> &mut f32 {
        &mut self.wait_time_before_executing
    }
    fn cooldown_time(&self) -> &f32 {
        &self.cooldown_time
    }
    fn cooldown_time_mut(&mut self) -> &mut f32 {
        &mut self.cooldown_time
    }
    fn timeout_duration(&self) -> &f32 {
        &self.timeout_duration
    }
    fn timeout_duration_mut(&mut self) -> &mut f32 {
        &mut self.timeout_duration
    }
    fn abort_when_alerted(&self) -> &bool {
        &self.abort_when_alerted
    }
    fn abort_when_alerted_mut(&mut self) -> &mut bool {
        &mut self.abort_when_alerted
    }
    fn abort_when_timed_out(&self) -> &bool {
        &self.abort_when_timed_out
    }
    fn abort_when_timed_out_mut(&mut self) -> &mut bool {
        &mut self.abort_when_timed_out
    }
    fn valid_in_state(&self) -> &ValidInState {
        &self.valid_in_state
    }
    fn valid_in_state_mut(&mut self) -> &mut ValidInState {
        &mut self.valid_in_state
    }
    fn action_priority(&self) -> &ActionPriority {
        &self.action_priority
    }
    fn action_priority_mut(&mut self) -> &mut ActionPriority {
        &mut self.action_priority
    }
    fn should_be_executed_by_closest_a_i(&self) -> &bool {
        &self.should_be_executed_by_closest_a_i
    }
    fn should_be_executed_by_closest_a_i_mut(&mut self) -> &mut bool {
        &mut self.should_be_executed_by_closest_a_i
    }
    fn only_execute_for_fitness_valid_a_i(&self) -> &bool {
        &self.only_execute_for_fitness_valid_a_i
    }
    fn only_execute_for_fitness_valid_a_i_mut(&mut self) -> &mut bool {
        &mut self.only_execute_for_fitness_valid_a_i
    }
    fn use_actual_path_distance(&self) -> &bool {
        &self.use_actual_path_distance
    }
    fn use_actual_path_distance_mut(&mut self) -> &mut bool {
        &mut self.use_actual_path_distance
    }
    fn is_linked_action(&self) -> &bool {
        &self.is_linked_action
    }
    fn is_linked_action_mut(&mut self) -> &mut bool {
        &mut self.is_linked_action
    }
    fn restrict_to_linked_soldiers(&self) -> &bool {
        &self.restrict_to_linked_soldiers
    }
    fn restrict_to_linked_soldiers_mut(&mut self) -> &mut bool {
        &mut self.restrict_to_linked_soldiers
    }
}

impl super::entity::SpatialEntityDataTrait for ActionEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
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
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ActionEntityData {
}

impl super::core::DataContainerTrait for ActionEntityData {
}

pub static ACTIONENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ActionEntityData",
    name_hash: 210217168,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(ActionEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ActionEntityData as Default>::default())),
            create_boxed: || Box::new(<ActionEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Enabled",
                name_hash: 2662400,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ActionEntityData, enabled),
            },
            FieldInfoData {
                name: "Conditions",
                name_hash: 3586042181,
                flags: MemberInfoFlags::new(144),
                field_type: "ActionCondition-Array",
                rust_offset: offset_of!(ActionEntityData, conditions),
            },
            FieldInfoData {
                name: "MoveToActionBeforeExecuting",
                name_hash: 2221585426,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ActionEntityData, move_to_action_before_executing),
            },
            FieldInfoData {
                name: "AlignToActionBeforeExecuting",
                name_hash: 3821027342,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ActionEntityData, align_to_action_before_executing),
            },
            FieldInfoData {
                name: "WaitTimeBeforeExecuting",
                name_hash: 1818256952,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ActionEntityData, wait_time_before_executing),
            },
            FieldInfoData {
                name: "CooldownTime",
                name_hash: 613462861,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ActionEntityData, cooldown_time),
            },
            FieldInfoData {
                name: "TimeoutDuration",
                name_hash: 545950720,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ActionEntityData, timeout_duration),
            },
            FieldInfoData {
                name: "AbortWhenAlerted",
                name_hash: 1822696916,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ActionEntityData, abort_when_alerted),
            },
            FieldInfoData {
                name: "AbortWhenTimedOut",
                name_hash: 1788170436,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ActionEntityData, abort_when_timed_out),
            },
            FieldInfoData {
                name: "ValidInState",
                name_hash: 1850177507,
                flags: MemberInfoFlags::new(0),
                field_type: "ValidInState",
                rust_offset: offset_of!(ActionEntityData, valid_in_state),
            },
            FieldInfoData {
                name: "ActionPriority",
                name_hash: 680218665,
                flags: MemberInfoFlags::new(0),
                field_type: "ActionPriority",
                rust_offset: offset_of!(ActionEntityData, action_priority),
            },
            FieldInfoData {
                name: "ShouldBeExecutedByClosestAI",
                name_hash: 4167392658,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ActionEntityData, should_be_executed_by_closest_a_i),
            },
            FieldInfoData {
                name: "OnlyExecuteForFitnessValidAI",
                name_hash: 2582675291,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ActionEntityData, only_execute_for_fitness_valid_a_i),
            },
            FieldInfoData {
                name: "UseActualPathDistance",
                name_hash: 1611962694,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ActionEntityData, use_actual_path_distance),
            },
            FieldInfoData {
                name: "IsLinkedAction",
                name_hash: 4273074336,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ActionEntityData, is_linked_action),
            },
            FieldInfoData {
                name: "RestrictToLinkedSoldiers",
                name_hash: 1687053882,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static ACTIONENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ActionEntityData-Array",
    name_hash: 1179166564,
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
    name_hash: 1850177507,
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


pub static VALIDINSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ValidInState-Array",
    name_hash: 3296988119,
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
    name_hash: 680218665,
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


pub static ACTIONPRIORITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ActionPriority-Array",
    name_hash: 3864786589,
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
    name_hash: 2416108236,
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


pub static ACTIONSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ActionState-Array",
    name_hash: 2146673528,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ActionState"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AITemplateCondition {
    pub _glacier_base: ActionCondition,
    pub templates: Vec<String>,
}

pub trait AITemplateConditionTrait: ActionConditionTrait {
    fn templates(&self) -> &Vec<String>;
    fn templates_mut(&mut self) -> &mut Vec<String>;
}

impl AITemplateConditionTrait for AITemplateCondition {
    fn templates(&self) -> &Vec<String> {
        &self.templates
    }
    fn templates_mut(&mut self) -> &mut Vec<String> {
        &mut self.templates
    }
}

impl ActionConditionTrait for AITemplateCondition {
}

impl super::core::DataContainerTrait for AITemplateCondition {
}

pub static AITEMPLATECONDITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AITemplateCondition",
    name_hash: 339502862,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ACTIONCONDITION_TYPE_INFO),
        super_class_offset: offset_of!(AITemplateCondition, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AITemplateCondition as Default>::default())),
            create_boxed: || Box::new(<AITemplateCondition as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Templates",
                name_hash: 2783016966,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static AITEMPLATECONDITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AITemplateCondition-Array",
    name_hash: 1639419962,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AITemplateCondition"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct InsideVolumesCondition {
    pub _glacier_base: ActionCondition,
    pub station_inside_user_search_area: bool,
    pub station_inside_user_defend_area: bool,
}

pub trait InsideVolumesConditionTrait: ActionConditionTrait {
    fn station_inside_user_search_area(&self) -> &bool;
    fn station_inside_user_search_area_mut(&mut self) -> &mut bool;
    fn station_inside_user_defend_area(&self) -> &bool;
    fn station_inside_user_defend_area_mut(&mut self) -> &mut bool;
}

impl InsideVolumesConditionTrait for InsideVolumesCondition {
    fn station_inside_user_search_area(&self) -> &bool {
        &self.station_inside_user_search_area
    }
    fn station_inside_user_search_area_mut(&mut self) -> &mut bool {
        &mut self.station_inside_user_search_area
    }
    fn station_inside_user_defend_area(&self) -> &bool {
        &self.station_inside_user_defend_area
    }
    fn station_inside_user_defend_area_mut(&mut self) -> &mut bool {
        &mut self.station_inside_user_defend_area
    }
}

impl ActionConditionTrait for InsideVolumesCondition {
}

impl super::core::DataContainerTrait for InsideVolumesCondition {
}

pub static INSIDEVOLUMESCONDITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InsideVolumesCondition",
    name_hash: 4094836145,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ACTIONCONDITION_TYPE_INFO),
        super_class_offset: offset_of!(InsideVolumesCondition, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InsideVolumesCondition as Default>::default())),
            create_boxed: || Box::new(<InsideVolumesCondition as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "StationInsideUserSearchArea",
                name_hash: 1119639595,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(InsideVolumesCondition, station_inside_user_search_area),
            },
            FieldInfoData {
                name: "StationInsideUserDefendArea",
                name_hash: 3365259021,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static INSIDEVOLUMESCONDITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InsideVolumesCondition-Array",
    name_hash: 1926393861,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("InsideVolumesCondition"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ProbabilityCondition {
    pub _glacier_base: ActionCondition,
    pub probability: f32,
}

pub trait ProbabilityConditionTrait: ActionConditionTrait {
    fn probability(&self) -> &f32;
    fn probability_mut(&mut self) -> &mut f32;
}

impl ProbabilityConditionTrait for ProbabilityCondition {
    fn probability(&self) -> &f32 {
        &self.probability
    }
    fn probability_mut(&mut self) -> &mut f32 {
        &mut self.probability
    }
}

impl ActionConditionTrait for ProbabilityCondition {
}

impl super::core::DataContainerTrait for ProbabilityCondition {
}

pub static PROBABILITYCONDITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProbabilityCondition",
    name_hash: 751003515,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ACTIONCONDITION_TYPE_INFO),
        super_class_offset: offset_of!(ProbabilityCondition, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ProbabilityCondition as Default>::default())),
            create_boxed: || Box::new(<ProbabilityCondition as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Probability",
                name_hash: 35957416,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static PROBABILITYCONDITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProbabilityCondition-Array",
    name_hash: 1629994831,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ProbabilityCondition"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct FacingCondition {
    pub _glacier_base: ActionCondition,
    pub max_angle: f32,
}

pub trait FacingConditionTrait: ActionConditionTrait {
    fn max_angle(&self) -> &f32;
    fn max_angle_mut(&mut self) -> &mut f32;
}

impl FacingConditionTrait for FacingCondition {
    fn max_angle(&self) -> &f32 {
        &self.max_angle
    }
    fn max_angle_mut(&mut self) -> &mut f32 {
        &mut self.max_angle
    }
}

impl ActionConditionTrait for FacingCondition {
}

impl super::core::DataContainerTrait for FacingCondition {
}

pub static FACINGCONDITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FacingCondition",
    name_hash: 450369650,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ACTIONCONDITION_TYPE_INFO),
        super_class_offset: offset_of!(FacingCondition, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FacingCondition as Default>::default())),
            create_boxed: || Box::new(<FacingCondition as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "MaxAngle",
                name_hash: 417488496,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static FACINGCONDITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FacingCondition-Array",
    name_hash: 1758613318,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("FacingCondition"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AIOrderCoordinatorCondition {
    pub _glacier_base: ActionCondition,
    pub only_valid_when_expecting_self_orders: bool,
}

pub trait AIOrderCoordinatorConditionTrait: ActionConditionTrait {
    fn only_valid_when_expecting_self_orders(&self) -> &bool;
    fn only_valid_when_expecting_self_orders_mut(&mut self) -> &mut bool;
}

impl AIOrderCoordinatorConditionTrait for AIOrderCoordinatorCondition {
    fn only_valid_when_expecting_self_orders(&self) -> &bool {
        &self.only_valid_when_expecting_self_orders
    }
    fn only_valid_when_expecting_self_orders_mut(&mut self) -> &mut bool {
        &mut self.only_valid_when_expecting_self_orders
    }
}

impl ActionConditionTrait for AIOrderCoordinatorCondition {
}

impl super::core::DataContainerTrait for AIOrderCoordinatorCondition {
}

pub static AIORDERCOORDINATORCONDITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIOrderCoordinatorCondition",
    name_hash: 1245440042,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ACTIONCONDITION_TYPE_INFO),
        super_class_offset: offset_of!(AIOrderCoordinatorCondition, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AIOrderCoordinatorCondition as Default>::default())),
            create_boxed: || Box::new(<AIOrderCoordinatorCondition as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "OnlyValidWhenExpectingSelfOrders",
                name_hash: 2416964365,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static AIORDERCOORDINATORCONDITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIOrderCoordinatorCondition-Array",
    name_hash: 3850355358,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AIOrderCoordinatorCondition"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AIStateCondition {
    pub _glacier_base: ActionCondition,
    pub a_i_state: ConditionAIStates,
    pub restrict_search_area_to_secondary_search: bool,
}

pub trait AIStateConditionTrait: ActionConditionTrait {
    fn a_i_state(&self) -> &ConditionAIStates;
    fn a_i_state_mut(&mut self) -> &mut ConditionAIStates;
    fn restrict_search_area_to_secondary_search(&self) -> &bool;
    fn restrict_search_area_to_secondary_search_mut(&mut self) -> &mut bool;
}

impl AIStateConditionTrait for AIStateCondition {
    fn a_i_state(&self) -> &ConditionAIStates {
        &self.a_i_state
    }
    fn a_i_state_mut(&mut self) -> &mut ConditionAIStates {
        &mut self.a_i_state
    }
    fn restrict_search_area_to_secondary_search(&self) -> &bool {
        &self.restrict_search_area_to_secondary_search
    }
    fn restrict_search_area_to_secondary_search_mut(&mut self) -> &mut bool {
        &mut self.restrict_search_area_to_secondary_search
    }
}

impl ActionConditionTrait for AIStateCondition {
}

impl super::core::DataContainerTrait for AIStateCondition {
}

pub static AISTATECONDITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIStateCondition",
    name_hash: 670101769,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ACTIONCONDITION_TYPE_INFO),
        super_class_offset: offset_of!(AIStateCondition, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AIStateCondition as Default>::default())),
            create_boxed: || Box::new(<AIStateCondition as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "AIState",
                name_hash: 1905800346,
                flags: MemberInfoFlags::new(0),
                field_type: "ConditionAIStates",
                rust_offset: offset_of!(AIStateCondition, a_i_state),
            },
            FieldInfoData {
                name: "RestrictSearchAreaToSecondarySearch",
                name_hash: 2496691599,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static AISTATECONDITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIStateCondition-Array",
    name_hash: 1376421565,
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
    name_hash: 2657057242,
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


pub static CONDITIONAISTATES_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionAIStates-Array",
    name_hash: 3316714862,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ConditionAIStates"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct RangeCondition {
    pub _glacier_base: ActionCondition,
    pub radius: f32,
}

pub trait RangeConditionTrait: ActionConditionTrait {
    fn radius(&self) -> &f32;
    fn radius_mut(&mut self) -> &mut f32;
}

impl RangeConditionTrait for RangeCondition {
    fn radius(&self) -> &f32 {
        &self.radius
    }
    fn radius_mut(&mut self) -> &mut f32 {
        &mut self.radius
    }
}

impl ActionConditionTrait for RangeCondition {
}

impl super::core::DataContainerTrait for RangeCondition {
}

pub static RANGECONDITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RangeCondition",
    name_hash: 2728914345,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ACTIONCONDITION_TYPE_INFO),
        super_class_offset: offset_of!(RangeCondition, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RangeCondition as Default>::default())),
            create_boxed: || Box::new(<RangeCondition as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Radius",
                name_hash: 3298407133,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static RANGECONDITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RangeCondition-Array",
    name_hash: 2044245021,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("RangeCondition"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ActionCondition {
    pub _glacier_base: super::core::DataContainer,
}

pub trait ActionConditionTrait: super::core::DataContainerTrait {
}

impl ActionConditionTrait for ActionCondition {
}

impl super::core::DataContainerTrait for ActionCondition {
}

pub static ACTIONCONDITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ActionCondition",
    name_hash: 910352456,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(ActionCondition, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ActionCondition as Default>::default())),
            create_boxed: || Box::new(<ActionCondition as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static ACTIONCONDITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ActionCondition-Array",
    name_hash: 1130107644,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ActionCondition"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PreferredCoverScoreData {
    pub _glacier_base: CustomCoverScoreData,
    pub preferred_cover_id: u32,
    pub score: f32,
}

pub trait PreferredCoverScoreDataTrait: CustomCoverScoreDataTrait {
    fn preferred_cover_id(&self) -> &u32;
    fn preferred_cover_id_mut(&mut self) -> &mut u32;
    fn score(&self) -> &f32;
    fn score_mut(&mut self) -> &mut f32;
}

impl PreferredCoverScoreDataTrait for PreferredCoverScoreData {
    fn preferred_cover_id(&self) -> &u32 {
        &self.preferred_cover_id
    }
    fn preferred_cover_id_mut(&mut self) -> &mut u32 {
        &mut self.preferred_cover_id
    }
    fn score(&self) -> &f32 {
        &self.score
    }
    fn score_mut(&mut self) -> &mut f32 {
        &mut self.score
    }
}

impl CustomCoverScoreDataTrait for PreferredCoverScoreData {
}

impl CoverScoreDataTrait for PreferredCoverScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn comment_mut(&mut self) -> &mut String {
        self._glacier_base.comment_mut()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for PreferredCoverScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn id_mut(&mut self) -> &mut u32 {
        self._glacier_base.id_mut()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
    fn runtime_filter_mut(&mut self) -> &mut u32 {
        self._glacier_base.runtime_filter_mut()
    }
}

impl super::core::DataContainerTrait for PreferredCoverScoreData {
}

pub static PREFERREDCOVERSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PreferredCoverScoreData",
    name_hash: 1757541749,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CUSTOMCOVERSCOREDATA_TYPE_INFO),
        super_class_offset: offset_of!(PreferredCoverScoreData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PreferredCoverScoreData as Default>::default())),
            create_boxed: || Box::new(<PreferredCoverScoreData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "PreferredCoverId",
                name_hash: 304098496,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(PreferredCoverScoreData, preferred_cover_id),
            },
            FieldInfoData {
                name: "Score",
                name_hash: 231225165,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static PREFERREDCOVERSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PreferredCoverScoreData-Array",
    name_hash: 913106497,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("PreferredCoverScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PreferredAreaScoreData {
    pub _glacier_base: CustomCoverScoreData,
    pub shapes: Vec<BoxedTypeObject /* BaseShapeWithOffset */>,
    pub score: f32,
}

pub trait PreferredAreaScoreDataTrait: CustomCoverScoreDataTrait {
    fn shapes(&self) -> &Vec<BoxedTypeObject /* BaseShapeWithOffset */>;
    fn shapes_mut(&mut self) -> &mut Vec<BoxedTypeObject /* BaseShapeWithOffset */>;
    fn score(&self) -> &f32;
    fn score_mut(&mut self) -> &mut f32;
}

impl PreferredAreaScoreDataTrait for PreferredAreaScoreData {
    fn shapes(&self) -> &Vec<BoxedTypeObject /* BaseShapeWithOffset */> {
        &self.shapes
    }
    fn shapes_mut(&mut self) -> &mut Vec<BoxedTypeObject /* BaseShapeWithOffset */> {
        &mut self.shapes
    }
    fn score(&self) -> &f32 {
        &self.score
    }
    fn score_mut(&mut self) -> &mut f32 {
        &mut self.score
    }
}

impl CustomCoverScoreDataTrait for PreferredAreaScoreData {
}

impl CoverScoreDataTrait for PreferredAreaScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn comment_mut(&mut self) -> &mut String {
        self._glacier_base.comment_mut()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for PreferredAreaScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn id_mut(&mut self) -> &mut u32 {
        self._glacier_base.id_mut()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
    fn runtime_filter_mut(&mut self) -> &mut u32 {
        self._glacier_base.runtime_filter_mut()
    }
}

impl super::core::DataContainerTrait for PreferredAreaScoreData {
}

pub static PREFERREDAREASCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PreferredAreaScoreData",
    name_hash: 3004526479,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CUSTOMCOVERSCOREDATA_TYPE_INFO),
        super_class_offset: offset_of!(PreferredAreaScoreData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PreferredAreaScoreData as Default>::default())),
            create_boxed: || Box::new(<PreferredAreaScoreData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Shapes",
                name_hash: 3352896601,
                flags: MemberInfoFlags::new(144),
                field_type: "BaseShapeWithOffset-Array",
                rust_offset: offset_of!(PreferredAreaScoreData, shapes),
            },
            FieldInfoData {
                name: "Score",
                name_hash: 231225165,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static PREFERREDAREASCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PreferredAreaScoreData-Array",
    name_hash: 179047483,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("PreferredAreaScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct BaseShapeWithOffset {
    pub shape: Option<LockedTypeObject /* super::entity::BaseShapeData */>,
    pub offset: super::core::LinearTransform,
    pub owner_transform: super::core::LinearTransform,
}

pub trait BaseShapeWithOffsetTrait: TypeObject {
    fn shape(&self) -> &Option<LockedTypeObject /* super::entity::BaseShapeData */>;
    fn shape_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::BaseShapeData */>;
    fn offset(&self) -> &super::core::LinearTransform;
    fn offset_mut(&mut self) -> &mut super::core::LinearTransform;
    fn owner_transform(&self) -> &super::core::LinearTransform;
    fn owner_transform_mut(&mut self) -> &mut super::core::LinearTransform;
}

impl BaseShapeWithOffsetTrait for BaseShapeWithOffset {
    fn shape(&self) -> &Option<LockedTypeObject /* super::entity::BaseShapeData */> {
        &self.shape
    }
    fn shape_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::BaseShapeData */> {
        &mut self.shape
    }
    fn offset(&self) -> &super::core::LinearTransform {
        &self.offset
    }
    fn offset_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.offset
    }
    fn owner_transform(&self) -> &super::core::LinearTransform {
        &self.owner_transform
    }
    fn owner_transform_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.owner_transform
    }
}

pub static BASESHAPEWITHOFFSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BaseShapeWithOffset",
    name_hash: 397245168,
    flags: MemberInfoFlags::new(73),
    module: "BattleAI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BaseShapeWithOffset as Default>::default())),
            create_boxed: || Box::new(<BaseShapeWithOffset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Shape",
                name_hash: 231753450,
                flags: MemberInfoFlags::new(0),
                field_type: "BaseShapeData",
                rust_offset: offset_of!(BaseShapeWithOffset, shape),
            },
            FieldInfoData {
                name: "Offset",
                name_hash: 2871410728,
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(BaseShapeWithOffset, offset),
            },
            FieldInfoData {
                name: "OwnerTransform",
                name_hash: 281651048,
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


pub static BASESHAPEWITHOFFSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BaseShapeWithOffset-Array",
    name_hash: 2686146756,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("BaseShapeWithOffset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn comment_mut(&mut self) -> &mut String {
        self._glacier_base.comment_mut()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for CustomCoverScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn id_mut(&mut self) -> &mut u32 {
        self._glacier_base.id_mut()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
    fn runtime_filter_mut(&mut self) -> &mut u32 {
        self._glacier_base.runtime_filter_mut()
    }
}

impl super::core::DataContainerTrait for CustomCoverScoreData {
}

pub static CUSTOMCOVERSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomCoverScoreData",
    name_hash: 865234787,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATA_TYPE_INFO),
        super_class_offset: offset_of!(CustomCoverScoreData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CustomCoverScoreData as Default>::default())),
            create_boxed: || Box::new(<CustomCoverScoreData as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CUSTOMCOVERSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomCoverScoreData-Array",
    name_hash: 3339363159,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("CustomCoverScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ReducePathDistanceToFollowObjectScoreData {
    pub _glacier_base: CoverScoreDataWithScoreCurve,
}

pub trait ReducePathDistanceToFollowObjectScoreDataTrait: CoverScoreDataWithScoreCurveTrait {
}

impl ReducePathDistanceToFollowObjectScoreDataTrait for ReducePathDistanceToFollowObjectScoreData {
}

impl CoverScoreDataWithScoreCurveTrait for ReducePathDistanceToFollowObjectScoreData {
    fn score_curve(&self) -> &Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve()
    }
    fn score_curve_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve_mut()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_scale_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_scale_mut()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
    fn score_curve_max_y_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_max_y_mut()
    }
}

impl CoverScoreDataTrait for ReducePathDistanceToFollowObjectScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn comment_mut(&mut self) -> &mut String {
        self._glacier_base.comment_mut()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for ReducePathDistanceToFollowObjectScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn id_mut(&mut self) -> &mut u32 {
        self._glacier_base.id_mut()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
    fn runtime_filter_mut(&mut self) -> &mut u32 {
        self._glacier_base.runtime_filter_mut()
    }
}

impl super::core::DataContainerTrait for ReducePathDistanceToFollowObjectScoreData {
}

pub static REDUCEPATHDISTANCETOFOLLOWOBJECTSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReducePathDistanceToFollowObjectScoreData",
    name_hash: 2909939628,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        super_class_offset: offset_of!(ReducePathDistanceToFollowObjectScoreData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ReducePathDistanceToFollowObjectScoreData as Default>::default())),
            create_boxed: || Box::new(<ReducePathDistanceToFollowObjectScoreData as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static REDUCEPATHDISTANCETOFOLLOWOBJECTSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReducePathDistanceToFollowObjectScoreData-Array",
    name_hash: 4065253144,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ReducePathDistanceToFollowObjectScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ReducePathDistanceToTargetScoreData {
    pub _glacier_base: CoverScoreDataWithScoreCurve,
}

pub trait ReducePathDistanceToTargetScoreDataTrait: CoverScoreDataWithScoreCurveTrait {
}

impl ReducePathDistanceToTargetScoreDataTrait for ReducePathDistanceToTargetScoreData {
}

impl CoverScoreDataWithScoreCurveTrait for ReducePathDistanceToTargetScoreData {
    fn score_curve(&self) -> &Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve()
    }
    fn score_curve_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve_mut()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_scale_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_scale_mut()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
    fn score_curve_max_y_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_max_y_mut()
    }
}

impl CoverScoreDataTrait for ReducePathDistanceToTargetScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn comment_mut(&mut self) -> &mut String {
        self._glacier_base.comment_mut()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for ReducePathDistanceToTargetScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn id_mut(&mut self) -> &mut u32 {
        self._glacier_base.id_mut()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
    fn runtime_filter_mut(&mut self) -> &mut u32 {
        self._glacier_base.runtime_filter_mut()
    }
}

impl super::core::DataContainerTrait for ReducePathDistanceToTargetScoreData {
}

pub static REDUCEPATHDISTANCETOTARGETSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReducePathDistanceToTargetScoreData",
    name_hash: 1661991289,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        super_class_offset: offset_of!(ReducePathDistanceToTargetScoreData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ReducePathDistanceToTargetScoreData as Default>::default())),
            create_boxed: || Box::new(<ReducePathDistanceToTargetScoreData as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static REDUCEPATHDISTANCETOTARGETSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReducePathDistanceToTargetScoreData-Array",
    name_hash: 1030592589,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ReducePathDistanceToTargetScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LineOfFireScoreData {
    pub _glacier_base: CoverScoreDataWithScoreCurve,
    pub override_open_cover_height: f32,
}

pub trait LineOfFireScoreDataTrait: CoverScoreDataWithScoreCurveTrait {
    fn override_open_cover_height(&self) -> &f32;
    fn override_open_cover_height_mut(&mut self) -> &mut f32;
}

impl LineOfFireScoreDataTrait for LineOfFireScoreData {
    fn override_open_cover_height(&self) -> &f32 {
        &self.override_open_cover_height
    }
    fn override_open_cover_height_mut(&mut self) -> &mut f32 {
        &mut self.override_open_cover_height
    }
}

impl CoverScoreDataWithScoreCurveTrait for LineOfFireScoreData {
    fn score_curve(&self) -> &Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve()
    }
    fn score_curve_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve_mut()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_scale_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_scale_mut()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
    fn score_curve_max_y_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_max_y_mut()
    }
}

impl CoverScoreDataTrait for LineOfFireScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn comment_mut(&mut self) -> &mut String {
        self._glacier_base.comment_mut()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for LineOfFireScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn id_mut(&mut self) -> &mut u32 {
        self._glacier_base.id_mut()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
    fn runtime_filter_mut(&mut self) -> &mut u32 {
        self._glacier_base.runtime_filter_mut()
    }
}

impl super::core::DataContainerTrait for LineOfFireScoreData {
}

pub static LINEOFFIRESCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LineOfFireScoreData",
    name_hash: 1060326530,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        super_class_offset: offset_of!(LineOfFireScoreData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LineOfFireScoreData as Default>::default())),
            create_boxed: || Box::new(<LineOfFireScoreData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "OverrideOpenCoverHeight",
                name_hash: 4081422327,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static LINEOFFIRESCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LineOfFireScoreData-Array",
    name_hash: 1592101814,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("LineOfFireScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct NavProbeScoreData {
    pub _glacier_base: CoverScoreData,
    pub ref_position: CoverScorePosition,
    pub score: f32,
    pub max_probe_distance: f32,
}

pub trait NavProbeScoreDataTrait: CoverScoreDataTrait {
    fn ref_position(&self) -> &CoverScorePosition;
    fn ref_position_mut(&mut self) -> &mut CoverScorePosition;
    fn score(&self) -> &f32;
    fn score_mut(&mut self) -> &mut f32;
    fn max_probe_distance(&self) -> &f32;
    fn max_probe_distance_mut(&mut self) -> &mut f32;
}

impl NavProbeScoreDataTrait for NavProbeScoreData {
    fn ref_position(&self) -> &CoverScorePosition {
        &self.ref_position
    }
    fn ref_position_mut(&mut self) -> &mut CoverScorePosition {
        &mut self.ref_position
    }
    fn score(&self) -> &f32 {
        &self.score
    }
    fn score_mut(&mut self) -> &mut f32 {
        &mut self.score
    }
    fn max_probe_distance(&self) -> &f32 {
        &self.max_probe_distance
    }
    fn max_probe_distance_mut(&mut self) -> &mut f32 {
        &mut self.max_probe_distance
    }
}

impl CoverScoreDataTrait for NavProbeScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn comment_mut(&mut self) -> &mut String {
        self._glacier_base.comment_mut()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for NavProbeScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn id_mut(&mut self) -> &mut u32 {
        self._glacier_base.id_mut()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
    fn runtime_filter_mut(&mut self) -> &mut u32 {
        self._glacier_base.runtime_filter_mut()
    }
}

impl super::core::DataContainerTrait for NavProbeScoreData {
}

pub static NAVPROBESCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NavProbeScoreData",
    name_hash: 1639440078,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATA_TYPE_INFO),
        super_class_offset: offset_of!(NavProbeScoreData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NavProbeScoreData as Default>::default())),
            create_boxed: || Box::new(<NavProbeScoreData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "RefPosition",
                name_hash: 114250413,
                flags: MemberInfoFlags::new(0),
                field_type: "CoverScorePosition",
                rust_offset: offset_of!(NavProbeScoreData, ref_position),
            },
            FieldInfoData {
                name: "Score",
                name_hash: 231225165,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NavProbeScoreData, score),
            },
            FieldInfoData {
                name: "MaxProbeDistance",
                name_hash: 3580874296,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static NAVPROBESCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NavProbeScoreData-Array",
    name_hash: 3064862842,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("NavProbeScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct BehindFollowObjectScoreData {
    pub _glacier_base: CoverScoreDataWithScoreCurve,
    pub max_enemy_distance: f32,
    pub enemy_position_weight_curve: Option<LockedTypeObject /* super::core::FloatCurve */>,
}

pub trait BehindFollowObjectScoreDataTrait: CoverScoreDataWithScoreCurveTrait {
    fn max_enemy_distance(&self) -> &f32;
    fn max_enemy_distance_mut(&mut self) -> &mut f32;
    fn enemy_position_weight_curve(&self) -> &Option<LockedTypeObject /* super::core::FloatCurve */>;
    fn enemy_position_weight_curve_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::FloatCurve */>;
}

impl BehindFollowObjectScoreDataTrait for BehindFollowObjectScoreData {
    fn max_enemy_distance(&self) -> &f32 {
        &self.max_enemy_distance
    }
    fn max_enemy_distance_mut(&mut self) -> &mut f32 {
        &mut self.max_enemy_distance
    }
    fn enemy_position_weight_curve(&self) -> &Option<LockedTypeObject /* super::core::FloatCurve */> {
        &self.enemy_position_weight_curve
    }
    fn enemy_position_weight_curve_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::FloatCurve */> {
        &mut self.enemy_position_weight_curve
    }
}

impl CoverScoreDataWithScoreCurveTrait for BehindFollowObjectScoreData {
    fn score_curve(&self) -> &Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve()
    }
    fn score_curve_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve_mut()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_scale_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_scale_mut()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
    fn score_curve_max_y_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_max_y_mut()
    }
}

impl CoverScoreDataTrait for BehindFollowObjectScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn comment_mut(&mut self) -> &mut String {
        self._glacier_base.comment_mut()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for BehindFollowObjectScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn id_mut(&mut self) -> &mut u32 {
        self._glacier_base.id_mut()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
    fn runtime_filter_mut(&mut self) -> &mut u32 {
        self._glacier_base.runtime_filter_mut()
    }
}

impl super::core::DataContainerTrait for BehindFollowObjectScoreData {
}

pub static BEHINDFOLLOWOBJECTSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BehindFollowObjectScoreData",
    name_hash: 3057570549,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        super_class_offset: offset_of!(BehindFollowObjectScoreData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BehindFollowObjectScoreData as Default>::default())),
            create_boxed: || Box::new(<BehindFollowObjectScoreData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "MaxEnemyDistance",
                name_hash: 762942280,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BehindFollowObjectScoreData, max_enemy_distance),
            },
            FieldInfoData {
                name: "EnemyPositionWeightCurve",
                name_hash: 1514557681,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static BEHINDFOLLOWOBJECTSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BehindFollowObjectScoreData-Array",
    name_hash: 880403393,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("BehindFollowObjectScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AngleFromFollowObjectAimDirectionData {
    pub _glacier_base: CoverScoreDataWithScoreCurve,
    pub apply_only_if_aim_direction_stable: bool,
    pub max_follow_object_speed: f32,
}

pub trait AngleFromFollowObjectAimDirectionDataTrait: CoverScoreDataWithScoreCurveTrait {
    fn apply_only_if_aim_direction_stable(&self) -> &bool;
    fn apply_only_if_aim_direction_stable_mut(&mut self) -> &mut bool;
    fn max_follow_object_speed(&self) -> &f32;
    fn max_follow_object_speed_mut(&mut self) -> &mut f32;
}

impl AngleFromFollowObjectAimDirectionDataTrait for AngleFromFollowObjectAimDirectionData {
    fn apply_only_if_aim_direction_stable(&self) -> &bool {
        &self.apply_only_if_aim_direction_stable
    }
    fn apply_only_if_aim_direction_stable_mut(&mut self) -> &mut bool {
        &mut self.apply_only_if_aim_direction_stable
    }
    fn max_follow_object_speed(&self) -> &f32 {
        &self.max_follow_object_speed
    }
    fn max_follow_object_speed_mut(&mut self) -> &mut f32 {
        &mut self.max_follow_object_speed
    }
}

impl CoverScoreDataWithScoreCurveTrait for AngleFromFollowObjectAimDirectionData {
    fn score_curve(&self) -> &Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve()
    }
    fn score_curve_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve_mut()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_scale_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_scale_mut()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
    fn score_curve_max_y_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_max_y_mut()
    }
}

impl CoverScoreDataTrait for AngleFromFollowObjectAimDirectionData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn comment_mut(&mut self) -> &mut String {
        self._glacier_base.comment_mut()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for AngleFromFollowObjectAimDirectionData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn id_mut(&mut self) -> &mut u32 {
        self._glacier_base.id_mut()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
    fn runtime_filter_mut(&mut self) -> &mut u32 {
        self._glacier_base.runtime_filter_mut()
    }
}

impl super::core::DataContainerTrait for AngleFromFollowObjectAimDirectionData {
}

pub static ANGLEFROMFOLLOWOBJECTAIMDIRECTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngleFromFollowObjectAimDirectionData",
    name_hash: 1139160838,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        super_class_offset: offset_of!(AngleFromFollowObjectAimDirectionData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AngleFromFollowObjectAimDirectionData as Default>::default())),
            create_boxed: || Box::new(<AngleFromFollowObjectAimDirectionData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ApplyOnlyIfAimDirectionStable",
                name_hash: 3957151399,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AngleFromFollowObjectAimDirectionData, apply_only_if_aim_direction_stable),
            },
            FieldInfoData {
                name: "MaxFollowObjectSpeed",
                name_hash: 2109075890,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static ANGLEFROMFOLLOWOBJECTAIMDIRECTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngleFromFollowObjectAimDirectionData-Array",
    name_hash: 4210951218,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AngleFromFollowObjectAimDirectionData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct FollowObjectReticleAvoidanceData {
    pub _glacier_base: CoverScoreDataWithScoreCurve,
    pub apply_only_if_aim_direction_stable: bool,
    pub max_follow_object_speed: f32,
    pub soldier_radius_expansion: f32,
    pub path_look_ahead_distance: f32,
}

pub trait FollowObjectReticleAvoidanceDataTrait: CoverScoreDataWithScoreCurveTrait {
    fn apply_only_if_aim_direction_stable(&self) -> &bool;
    fn apply_only_if_aim_direction_stable_mut(&mut self) -> &mut bool;
    fn max_follow_object_speed(&self) -> &f32;
    fn max_follow_object_speed_mut(&mut self) -> &mut f32;
    fn soldier_radius_expansion(&self) -> &f32;
    fn soldier_radius_expansion_mut(&mut self) -> &mut f32;
    fn path_look_ahead_distance(&self) -> &f32;
    fn path_look_ahead_distance_mut(&mut self) -> &mut f32;
}

impl FollowObjectReticleAvoidanceDataTrait for FollowObjectReticleAvoidanceData {
    fn apply_only_if_aim_direction_stable(&self) -> &bool {
        &self.apply_only_if_aim_direction_stable
    }
    fn apply_only_if_aim_direction_stable_mut(&mut self) -> &mut bool {
        &mut self.apply_only_if_aim_direction_stable
    }
    fn max_follow_object_speed(&self) -> &f32 {
        &self.max_follow_object_speed
    }
    fn max_follow_object_speed_mut(&mut self) -> &mut f32 {
        &mut self.max_follow_object_speed
    }
    fn soldier_radius_expansion(&self) -> &f32 {
        &self.soldier_radius_expansion
    }
    fn soldier_radius_expansion_mut(&mut self) -> &mut f32 {
        &mut self.soldier_radius_expansion
    }
    fn path_look_ahead_distance(&self) -> &f32 {
        &self.path_look_ahead_distance
    }
    fn path_look_ahead_distance_mut(&mut self) -> &mut f32 {
        &mut self.path_look_ahead_distance
    }
}

impl CoverScoreDataWithScoreCurveTrait for FollowObjectReticleAvoidanceData {
    fn score_curve(&self) -> &Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve()
    }
    fn score_curve_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve_mut()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_scale_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_scale_mut()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
    fn score_curve_max_y_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_max_y_mut()
    }
}

impl CoverScoreDataTrait for FollowObjectReticleAvoidanceData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn comment_mut(&mut self) -> &mut String {
        self._glacier_base.comment_mut()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for FollowObjectReticleAvoidanceData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn id_mut(&mut self) -> &mut u32 {
        self._glacier_base.id_mut()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
    fn runtime_filter_mut(&mut self) -> &mut u32 {
        self._glacier_base.runtime_filter_mut()
    }
}

impl super::core::DataContainerTrait for FollowObjectReticleAvoidanceData {
}

pub static FOLLOWOBJECTRETICLEAVOIDANCEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FollowObjectReticleAvoidanceData",
    name_hash: 2025891757,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        super_class_offset: offset_of!(FollowObjectReticleAvoidanceData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FollowObjectReticleAvoidanceData as Default>::default())),
            create_boxed: || Box::new(<FollowObjectReticleAvoidanceData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ApplyOnlyIfAimDirectionStable",
                name_hash: 3957151399,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FollowObjectReticleAvoidanceData, apply_only_if_aim_direction_stable),
            },
            FieldInfoData {
                name: "MaxFollowObjectSpeed",
                name_hash: 2109075890,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FollowObjectReticleAvoidanceData, max_follow_object_speed),
            },
            FieldInfoData {
                name: "SoldierRadiusExpansion",
                name_hash: 2817440814,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FollowObjectReticleAvoidanceData, soldier_radius_expansion),
            },
            FieldInfoData {
                name: "PathLookAheadDistance",
                name_hash: 2330456549,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static FOLLOWOBJECTRETICLEAVOIDANCEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FollowObjectReticleAvoidanceData-Array",
    name_hash: 1934924825,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("FollowObjectReticleAvoidanceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PathAvoidanceScoreData {
    pub _glacier_base: CoverScoreData,
    pub avoid_by_type_data: Vec<BoxedTypeObject /* super::battle_a_i_shared::CoverQueryPathEnemyAvoidanceByTypeData */>,
    pub max_search_distance: f32,
    pub reject_cover_beyond_search_distance: bool,
    pub inner_zone_score: f32,
    pub outer_zone_score: f32,
    pub not_passing_avoidance_area_score: f32,
    pub scores_per_zone: Vec<f32>,
}

pub trait PathAvoidanceScoreDataTrait: CoverScoreDataTrait {
    fn avoid_by_type_data(&self) -> &Vec<BoxedTypeObject /* super::battle_a_i_shared::CoverQueryPathEnemyAvoidanceByTypeData */>;
    fn avoid_by_type_data_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::battle_a_i_shared::CoverQueryPathEnemyAvoidanceByTypeData */>;
    fn max_search_distance(&self) -> &f32;
    fn max_search_distance_mut(&mut self) -> &mut f32;
    fn reject_cover_beyond_search_distance(&self) -> &bool;
    fn reject_cover_beyond_search_distance_mut(&mut self) -> &mut bool;
    fn inner_zone_score(&self) -> &f32;
    fn inner_zone_score_mut(&mut self) -> &mut f32;
    fn outer_zone_score(&self) -> &f32;
    fn outer_zone_score_mut(&mut self) -> &mut f32;
    fn not_passing_avoidance_area_score(&self) -> &f32;
    fn not_passing_avoidance_area_score_mut(&mut self) -> &mut f32;
    fn scores_per_zone(&self) -> &Vec<f32>;
    fn scores_per_zone_mut(&mut self) -> &mut Vec<f32>;
}

impl PathAvoidanceScoreDataTrait for PathAvoidanceScoreData {
    fn avoid_by_type_data(&self) -> &Vec<BoxedTypeObject /* super::battle_a_i_shared::CoverQueryPathEnemyAvoidanceByTypeData */> {
        &self.avoid_by_type_data
    }
    fn avoid_by_type_data_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::battle_a_i_shared::CoverQueryPathEnemyAvoidanceByTypeData */> {
        &mut self.avoid_by_type_data
    }
    fn max_search_distance(&self) -> &f32 {
        &self.max_search_distance
    }
    fn max_search_distance_mut(&mut self) -> &mut f32 {
        &mut self.max_search_distance
    }
    fn reject_cover_beyond_search_distance(&self) -> &bool {
        &self.reject_cover_beyond_search_distance
    }
    fn reject_cover_beyond_search_distance_mut(&mut self) -> &mut bool {
        &mut self.reject_cover_beyond_search_distance
    }
    fn inner_zone_score(&self) -> &f32 {
        &self.inner_zone_score
    }
    fn inner_zone_score_mut(&mut self) -> &mut f32 {
        &mut self.inner_zone_score
    }
    fn outer_zone_score(&self) -> &f32 {
        &self.outer_zone_score
    }
    fn outer_zone_score_mut(&mut self) -> &mut f32 {
        &mut self.outer_zone_score
    }
    fn not_passing_avoidance_area_score(&self) -> &f32 {
        &self.not_passing_avoidance_area_score
    }
    fn not_passing_avoidance_area_score_mut(&mut self) -> &mut f32 {
        &mut self.not_passing_avoidance_area_score
    }
    fn scores_per_zone(&self) -> &Vec<f32> {
        &self.scores_per_zone
    }
    fn scores_per_zone_mut(&mut self) -> &mut Vec<f32> {
        &mut self.scores_per_zone
    }
}

impl CoverScoreDataTrait for PathAvoidanceScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn comment_mut(&mut self) -> &mut String {
        self._glacier_base.comment_mut()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for PathAvoidanceScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn id_mut(&mut self) -> &mut u32 {
        self._glacier_base.id_mut()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
    fn runtime_filter_mut(&mut self) -> &mut u32 {
        self._glacier_base.runtime_filter_mut()
    }
}

impl super::core::DataContainerTrait for PathAvoidanceScoreData {
}

pub static PATHAVOIDANCESCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathAvoidanceScoreData",
    name_hash: 2025071052,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATA_TYPE_INFO),
        super_class_offset: offset_of!(PathAvoidanceScoreData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PathAvoidanceScoreData as Default>::default())),
            create_boxed: || Box::new(<PathAvoidanceScoreData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "AvoidByTypeData",
                name_hash: 4158994339,
                flags: MemberInfoFlags::new(144),
                field_type: "CoverQueryPathEnemyAvoidanceByTypeData-Array",
                rust_offset: offset_of!(PathAvoidanceScoreData, avoid_by_type_data),
            },
            FieldInfoData {
                name: "MaxSearchDistance",
                name_hash: 1031572764,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PathAvoidanceScoreData, max_search_distance),
            },
            FieldInfoData {
                name: "RejectCoverBeyondSearchDistance",
                name_hash: 4261766033,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathAvoidanceScoreData, reject_cover_beyond_search_distance),
            },
            FieldInfoData {
                name: "InnerZoneScore",
                name_hash: 2862235053,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PathAvoidanceScoreData, inner_zone_score),
            },
            FieldInfoData {
                name: "OuterZoneScore",
                name_hash: 184103658,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PathAvoidanceScoreData, outer_zone_score),
            },
            FieldInfoData {
                name: "NotPassingAvoidanceAreaScore",
                name_hash: 2263386178,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PathAvoidanceScoreData, not_passing_avoidance_area_score),
            },
            FieldInfoData {
                name: "ScoresPerZone",
                name_hash: 3677565063,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static PATHAVOIDANCESCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathAvoidanceScoreData-Array",
    name_hash: 4267188856,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("PathAvoidanceScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PathDistanceScoreData {
    pub _glacier_base: CoverScoreDataWithRefPos,
    pub max_search_distance: f32,
    pub precise_path: bool,
    pub reject_cover_beyond_search_distance: bool,
}

pub trait PathDistanceScoreDataTrait: CoverScoreDataWithRefPosTrait {
    fn max_search_distance(&self) -> &f32;
    fn max_search_distance_mut(&mut self) -> &mut f32;
    fn precise_path(&self) -> &bool;
    fn precise_path_mut(&mut self) -> &mut bool;
    fn reject_cover_beyond_search_distance(&self) -> &bool;
    fn reject_cover_beyond_search_distance_mut(&mut self) -> &mut bool;
}

impl PathDistanceScoreDataTrait for PathDistanceScoreData {
    fn max_search_distance(&self) -> &f32 {
        &self.max_search_distance
    }
    fn max_search_distance_mut(&mut self) -> &mut f32 {
        &mut self.max_search_distance
    }
    fn precise_path(&self) -> &bool {
        &self.precise_path
    }
    fn precise_path_mut(&mut self) -> &mut bool {
        &mut self.precise_path
    }
    fn reject_cover_beyond_search_distance(&self) -> &bool {
        &self.reject_cover_beyond_search_distance
    }
    fn reject_cover_beyond_search_distance_mut(&mut self) -> &mut bool {
        &mut self.reject_cover_beyond_search_distance
    }
}

impl CoverScoreDataWithRefPosTrait for PathDistanceScoreData {
    fn ref_position(&self) -> &CoverScorePosition {
        self._glacier_base.ref_position()
    }
    fn ref_position_mut(&mut self) -> &mut CoverScorePosition {
        self._glacier_base.ref_position_mut()
    }
}

impl CoverScoreDataWithScoreCurveTrait for PathDistanceScoreData {
    fn score_curve(&self) -> &Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve()
    }
    fn score_curve_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve_mut()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_scale_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_scale_mut()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
    fn score_curve_max_y_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_max_y_mut()
    }
}

impl CoverScoreDataTrait for PathDistanceScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn comment_mut(&mut self) -> &mut String {
        self._glacier_base.comment_mut()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for PathDistanceScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn id_mut(&mut self) -> &mut u32 {
        self._glacier_base.id_mut()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
    fn runtime_filter_mut(&mut self) -> &mut u32 {
        self._glacier_base.runtime_filter_mut()
    }
}

impl super::core::DataContainerTrait for PathDistanceScoreData {
}

pub static PATHDISTANCESCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathDistanceScoreData",
    name_hash: 3353531155,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHREFPOS_TYPE_INFO),
        super_class_offset: offset_of!(PathDistanceScoreData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PathDistanceScoreData as Default>::default())),
            create_boxed: || Box::new(<PathDistanceScoreData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "MaxSearchDistance",
                name_hash: 1031572764,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PathDistanceScoreData, max_search_distance),
            },
            FieldInfoData {
                name: "PrecisePath",
                name_hash: 733837779,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathDistanceScoreData, precise_path),
            },
            FieldInfoData {
                name: "RejectCoverBeyondSearchDistance",
                name_hash: 4261766033,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static PATHDISTANCESCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathDistanceScoreData-Array",
    name_hash: 1940508071,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("PathDistanceScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct RejectUnreachableCoverScoreData {
    pub _glacier_base: CoverScoreData,
    pub ref_position: CoverScorePosition,
}

pub trait RejectUnreachableCoverScoreDataTrait: CoverScoreDataTrait {
    fn ref_position(&self) -> &CoverScorePosition;
    fn ref_position_mut(&mut self) -> &mut CoverScorePosition;
}

impl RejectUnreachableCoverScoreDataTrait for RejectUnreachableCoverScoreData {
    fn ref_position(&self) -> &CoverScorePosition {
        &self.ref_position
    }
    fn ref_position_mut(&mut self) -> &mut CoverScorePosition {
        &mut self.ref_position
    }
}

impl CoverScoreDataTrait for RejectUnreachableCoverScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn comment_mut(&mut self) -> &mut String {
        self._glacier_base.comment_mut()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for RejectUnreachableCoverScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn id_mut(&mut self) -> &mut u32 {
        self._glacier_base.id_mut()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
    fn runtime_filter_mut(&mut self) -> &mut u32 {
        self._glacier_base.runtime_filter_mut()
    }
}

impl super::core::DataContainerTrait for RejectUnreachableCoverScoreData {
}

pub static REJECTUNREACHABLECOVERSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RejectUnreachableCoverScoreData",
    name_hash: 2338817875,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATA_TYPE_INFO),
        super_class_offset: offset_of!(RejectUnreachableCoverScoreData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RejectUnreachableCoverScoreData as Default>::default())),
            create_boxed: || Box::new(<RejectUnreachableCoverScoreData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "RefPosition",
                name_hash: 114250413,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static REJECTUNREACHABLECOVERSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RejectUnreachableCoverScoreData-Array",
    name_hash: 4062474855,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("RejectUnreachableCoverScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DistanceToCorpseData {
    pub _glacier_base: CoverScoreDataWithScoreCurve,
    pub max_time_since_death: f32,
}

pub trait DistanceToCorpseDataTrait: CoverScoreDataWithScoreCurveTrait {
    fn max_time_since_death(&self) -> &f32;
    fn max_time_since_death_mut(&mut self) -> &mut f32;
}

impl DistanceToCorpseDataTrait for DistanceToCorpseData {
    fn max_time_since_death(&self) -> &f32 {
        &self.max_time_since_death
    }
    fn max_time_since_death_mut(&mut self) -> &mut f32 {
        &mut self.max_time_since_death
    }
}

impl CoverScoreDataWithScoreCurveTrait for DistanceToCorpseData {
    fn score_curve(&self) -> &Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve()
    }
    fn score_curve_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve_mut()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_scale_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_scale_mut()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
    fn score_curve_max_y_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_max_y_mut()
    }
}

impl CoverScoreDataTrait for DistanceToCorpseData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn comment_mut(&mut self) -> &mut String {
        self._glacier_base.comment_mut()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for DistanceToCorpseData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn id_mut(&mut self) -> &mut u32 {
        self._glacier_base.id_mut()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
    fn runtime_filter_mut(&mut self) -> &mut u32 {
        self._glacier_base.runtime_filter_mut()
    }
}

impl super::core::DataContainerTrait for DistanceToCorpseData {
}

pub static DISTANCETOCORPSEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistanceToCorpseData",
    name_hash: 2933869173,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        super_class_offset: offset_of!(DistanceToCorpseData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DistanceToCorpseData as Default>::default())),
            create_boxed: || Box::new(<DistanceToCorpseData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "MaxTimeSinceDeath",
                name_hash: 40258154,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DISTANCETOCORPSEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistanceToCorpseData-Array",
    name_hash: 4101372737,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("DistanceToCorpseData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DistanceToClosestEnemyCoverScoreData {
    pub _glacier_base: CoverScoreDataWithScoreCurve,
}

pub trait DistanceToClosestEnemyCoverScoreDataTrait: CoverScoreDataWithScoreCurveTrait {
}

impl DistanceToClosestEnemyCoverScoreDataTrait for DistanceToClosestEnemyCoverScoreData {
}

impl CoverScoreDataWithScoreCurveTrait for DistanceToClosestEnemyCoverScoreData {
    fn score_curve(&self) -> &Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve()
    }
    fn score_curve_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve_mut()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_scale_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_scale_mut()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
    fn score_curve_max_y_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_max_y_mut()
    }
}

impl CoverScoreDataTrait for DistanceToClosestEnemyCoverScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn comment_mut(&mut self) -> &mut String {
        self._glacier_base.comment_mut()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for DistanceToClosestEnemyCoverScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn id_mut(&mut self) -> &mut u32 {
        self._glacier_base.id_mut()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
    fn runtime_filter_mut(&mut self) -> &mut u32 {
        self._glacier_base.runtime_filter_mut()
    }
}

impl super::core::DataContainerTrait for DistanceToClosestEnemyCoverScoreData {
}

pub static DISTANCETOCLOSESTENEMYCOVERSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistanceToClosestEnemyCoverScoreData",
    name_hash: 2468367427,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        super_class_offset: offset_of!(DistanceToClosestEnemyCoverScoreData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DistanceToClosestEnemyCoverScoreData as Default>::default())),
            create_boxed: || Box::new(<DistanceToClosestEnemyCoverScoreData as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DISTANCETOCLOSESTENEMYCOVERSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistanceToClosestEnemyCoverScoreData-Array",
    name_hash: 2768908151,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("DistanceToClosestEnemyCoverScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DistanceToClosestFriendlyScoreData {
    pub _glacier_base: CoverScoreDataWithScoreCurve,
}

pub trait DistanceToClosestFriendlyScoreDataTrait: CoverScoreDataWithScoreCurveTrait {
}

impl DistanceToClosestFriendlyScoreDataTrait for DistanceToClosestFriendlyScoreData {
}

impl CoverScoreDataWithScoreCurveTrait for DistanceToClosestFriendlyScoreData {
    fn score_curve(&self) -> &Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve()
    }
    fn score_curve_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve_mut()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_scale_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_scale_mut()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
    fn score_curve_max_y_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_max_y_mut()
    }
}

impl CoverScoreDataTrait for DistanceToClosestFriendlyScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn comment_mut(&mut self) -> &mut String {
        self._glacier_base.comment_mut()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for DistanceToClosestFriendlyScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn id_mut(&mut self) -> &mut u32 {
        self._glacier_base.id_mut()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
    fn runtime_filter_mut(&mut self) -> &mut u32 {
        self._glacier_base.runtime_filter_mut()
    }
}

impl super::core::DataContainerTrait for DistanceToClosestFriendlyScoreData {
}

pub static DISTANCETOCLOSESTFRIENDLYSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistanceToClosestFriendlyScoreData",
    name_hash: 1460098931,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        super_class_offset: offset_of!(DistanceToClosestFriendlyScoreData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DistanceToClosestFriendlyScoreData as Default>::default())),
            create_boxed: || Box::new(<DistanceToClosestFriendlyScoreData as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DISTANCETOCLOSESTFRIENDLYSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistanceToClosestFriendlyScoreData-Array",
    name_hash: 1210078023,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("DistanceToClosestFriendlyScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ExposureToMultiTargetsScoreData {
    pub _glacier_base: CoverScoreDataWithScoreCurve,
    pub exclude_primary_target: bool,
    pub ref_position_for_target_filtering: CoverScorePosition,
    pub target_significance_distance_curve: Option<LockedTypeObject /* super::core::FloatCurve */>,
    pub max_target_distance: f32,
    pub max_distance_ratio_from_closest_target: f32,
    pub min_target_distance_to_always_be_counted: f32,
    pub max_distance_ratio_from_closest_exposed_target: f32,
    pub max_distance_to_fully_reject_exposed_target: f32,
}

pub trait ExposureToMultiTargetsScoreDataTrait: CoverScoreDataWithScoreCurveTrait {
    fn exclude_primary_target(&self) -> &bool;
    fn exclude_primary_target_mut(&mut self) -> &mut bool;
    fn ref_position_for_target_filtering(&self) -> &CoverScorePosition;
    fn ref_position_for_target_filtering_mut(&mut self) -> &mut CoverScorePosition;
    fn target_significance_distance_curve(&self) -> &Option<LockedTypeObject /* super::core::FloatCurve */>;
    fn target_significance_distance_curve_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::FloatCurve */>;
    fn max_target_distance(&self) -> &f32;
    fn max_target_distance_mut(&mut self) -> &mut f32;
    fn max_distance_ratio_from_closest_target(&self) -> &f32;
    fn max_distance_ratio_from_closest_target_mut(&mut self) -> &mut f32;
    fn min_target_distance_to_always_be_counted(&self) -> &f32;
    fn min_target_distance_to_always_be_counted_mut(&mut self) -> &mut f32;
    fn max_distance_ratio_from_closest_exposed_target(&self) -> &f32;
    fn max_distance_ratio_from_closest_exposed_target_mut(&mut self) -> &mut f32;
    fn max_distance_to_fully_reject_exposed_target(&self) -> &f32;
    fn max_distance_to_fully_reject_exposed_target_mut(&mut self) -> &mut f32;
}

impl ExposureToMultiTargetsScoreDataTrait for ExposureToMultiTargetsScoreData {
    fn exclude_primary_target(&self) -> &bool {
        &self.exclude_primary_target
    }
    fn exclude_primary_target_mut(&mut self) -> &mut bool {
        &mut self.exclude_primary_target
    }
    fn ref_position_for_target_filtering(&self) -> &CoverScorePosition {
        &self.ref_position_for_target_filtering
    }
    fn ref_position_for_target_filtering_mut(&mut self) -> &mut CoverScorePosition {
        &mut self.ref_position_for_target_filtering
    }
    fn target_significance_distance_curve(&self) -> &Option<LockedTypeObject /* super::core::FloatCurve */> {
        &self.target_significance_distance_curve
    }
    fn target_significance_distance_curve_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::FloatCurve */> {
        &mut self.target_significance_distance_curve
    }
    fn max_target_distance(&self) -> &f32 {
        &self.max_target_distance
    }
    fn max_target_distance_mut(&mut self) -> &mut f32 {
        &mut self.max_target_distance
    }
    fn max_distance_ratio_from_closest_target(&self) -> &f32 {
        &self.max_distance_ratio_from_closest_target
    }
    fn max_distance_ratio_from_closest_target_mut(&mut self) -> &mut f32 {
        &mut self.max_distance_ratio_from_closest_target
    }
    fn min_target_distance_to_always_be_counted(&self) -> &f32 {
        &self.min_target_distance_to_always_be_counted
    }
    fn min_target_distance_to_always_be_counted_mut(&mut self) -> &mut f32 {
        &mut self.min_target_distance_to_always_be_counted
    }
    fn max_distance_ratio_from_closest_exposed_target(&self) -> &f32 {
        &self.max_distance_ratio_from_closest_exposed_target
    }
    fn max_distance_ratio_from_closest_exposed_target_mut(&mut self) -> &mut f32 {
        &mut self.max_distance_ratio_from_closest_exposed_target
    }
    fn max_distance_to_fully_reject_exposed_target(&self) -> &f32 {
        &self.max_distance_to_fully_reject_exposed_target
    }
    fn max_distance_to_fully_reject_exposed_target_mut(&mut self) -> &mut f32 {
        &mut self.max_distance_to_fully_reject_exposed_target
    }
}

impl CoverScoreDataWithScoreCurveTrait for ExposureToMultiTargetsScoreData {
    fn score_curve(&self) -> &Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve()
    }
    fn score_curve_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve_mut()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_scale_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_scale_mut()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
    fn score_curve_max_y_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_max_y_mut()
    }
}

impl CoverScoreDataTrait for ExposureToMultiTargetsScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn comment_mut(&mut self) -> &mut String {
        self._glacier_base.comment_mut()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for ExposureToMultiTargetsScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn id_mut(&mut self) -> &mut u32 {
        self._glacier_base.id_mut()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
    fn runtime_filter_mut(&mut self) -> &mut u32 {
        self._glacier_base.runtime_filter_mut()
    }
}

impl super::core::DataContainerTrait for ExposureToMultiTargetsScoreData {
}

pub static EXPOSURETOMULTITARGETSSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExposureToMultiTargetsScoreData",
    name_hash: 2389778142,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        super_class_offset: offset_of!(ExposureToMultiTargetsScoreData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ExposureToMultiTargetsScoreData as Default>::default())),
            create_boxed: || Box::new(<ExposureToMultiTargetsScoreData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ExcludePrimaryTarget",
                name_hash: 3931146494,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ExposureToMultiTargetsScoreData, exclude_primary_target),
            },
            FieldInfoData {
                name: "RefPositionForTargetFiltering",
                name_hash: 43030183,
                flags: MemberInfoFlags::new(0),
                field_type: "CoverScorePosition",
                rust_offset: offset_of!(ExposureToMultiTargetsScoreData, ref_position_for_target_filtering),
            },
            FieldInfoData {
                name: "TargetSignificanceDistanceCurve",
                name_hash: 2799969439,
                flags: MemberInfoFlags::new(0),
                field_type: "FloatCurve",
                rust_offset: offset_of!(ExposureToMultiTargetsScoreData, target_significance_distance_curve),
            },
            FieldInfoData {
                name: "MaxTargetDistance",
                name_hash: 2052129987,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ExposureToMultiTargetsScoreData, max_target_distance),
            },
            FieldInfoData {
                name: "MaxDistanceRatioFromClosestTarget",
                name_hash: 2132588581,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ExposureToMultiTargetsScoreData, max_distance_ratio_from_closest_target),
            },
            FieldInfoData {
                name: "MinTargetDistanceToAlwaysBeCounted",
                name_hash: 3442472018,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ExposureToMultiTargetsScoreData, min_target_distance_to_always_be_counted),
            },
            FieldInfoData {
                name: "MaxDistanceRatioFromClosestExposedTarget",
                name_hash: 2693217781,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ExposureToMultiTargetsScoreData, max_distance_ratio_from_closest_exposed_target),
            },
            FieldInfoData {
                name: "MaxDistanceToFullyRejectExposedTarget",
                name_hash: 2898557165,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static EXPOSURETOMULTITARGETSSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExposureToMultiTargetsScoreData-Array",
    name_hash: 686432362,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ExposureToMultiTargetsScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn exclude_primary_target_mut(&mut self) -> &mut bool {
        self._glacier_base.exclude_primary_target_mut()
    }
    fn max_distance_to_target(&self) -> &f32 {
        self._glacier_base.max_distance_to_target()
    }
    fn max_distance_to_target_mut(&mut self) -> &mut f32 {
        self._glacier_base.max_distance_to_target_mut()
    }
    fn scoring_mode(&self) -> &MultiTargetScoringMode {
        self._glacier_base.scoring_mode()
    }
    fn scoring_mode_mut(&mut self) -> &mut MultiTargetScoringMode {
        self._glacier_base.scoring_mode_mut()
    }
}

impl CoverScoreDataWithScoreCurveTrait for DistanceToMultiTargetsScoreData {
    fn score_curve(&self) -> &Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve()
    }
    fn score_curve_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve_mut()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_scale_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_scale_mut()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
    fn score_curve_max_y_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_max_y_mut()
    }
}

impl CoverScoreDataTrait for DistanceToMultiTargetsScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn comment_mut(&mut self) -> &mut String {
        self._glacier_base.comment_mut()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for DistanceToMultiTargetsScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn id_mut(&mut self) -> &mut u32 {
        self._glacier_base.id_mut()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
    fn runtime_filter_mut(&mut self) -> &mut u32 {
        self._glacier_base.runtime_filter_mut()
    }
}

impl super::core::DataContainerTrait for DistanceToMultiTargetsScoreData {
}

pub static DISTANCETOMULTITARGETSSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistanceToMultiTargetsScoreData",
    name_hash: 3048369006,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MULTITARGETSCORER_TYPE_INFO),
        super_class_offset: offset_of!(DistanceToMultiTargetsScoreData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DistanceToMultiTargetsScoreData as Default>::default())),
            create_boxed: || Box::new(<DistanceToMultiTargetsScoreData as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DISTANCETOMULTITARGETSSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistanceToMultiTargetsScoreData-Array",
    name_hash: 4111148122,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("DistanceToMultiTargetsScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn exclude_primary_target_mut(&mut self) -> &mut bool {
        self._glacier_base.exclude_primary_target_mut()
    }
    fn max_distance_to_target(&self) -> &f32 {
        self._glacier_base.max_distance_to_target()
    }
    fn max_distance_to_target_mut(&mut self) -> &mut f32 {
        self._glacier_base.max_distance_to_target_mut()
    }
    fn scoring_mode(&self) -> &MultiTargetScoringMode {
        self._glacier_base.scoring_mode()
    }
    fn scoring_mode_mut(&mut self) -> &mut MultiTargetScoringMode {
        self._glacier_base.scoring_mode_mut()
    }
}

impl CoverScoreDataWithScoreCurveTrait for AngleToMultiTargetsScoreData {
    fn score_curve(&self) -> &Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve()
    }
    fn score_curve_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve_mut()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_scale_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_scale_mut()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
    fn score_curve_max_y_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_max_y_mut()
    }
}

impl CoverScoreDataTrait for AngleToMultiTargetsScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn comment_mut(&mut self) -> &mut String {
        self._glacier_base.comment_mut()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for AngleToMultiTargetsScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn id_mut(&mut self) -> &mut u32 {
        self._glacier_base.id_mut()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
    fn runtime_filter_mut(&mut self) -> &mut u32 {
        self._glacier_base.runtime_filter_mut()
    }
}

impl super::core::DataContainerTrait for AngleToMultiTargetsScoreData {
}

pub static ANGLETOMULTITARGETSSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngleToMultiTargetsScoreData",
    name_hash: 1952442508,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MULTITARGETSCORER_TYPE_INFO),
        super_class_offset: offset_of!(AngleToMultiTargetsScoreData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AngleToMultiTargetsScoreData as Default>::default())),
            create_boxed: || Box::new(<AngleToMultiTargetsScoreData as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static ANGLETOMULTITARGETSSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngleToMultiTargetsScoreData-Array",
    name_hash: 204077240,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AngleToMultiTargetsScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MultiTargetScorer {
    pub _glacier_base: CoverScoreDataWithScoreCurve,
    pub exclude_primary_target: bool,
    pub max_distance_to_target: f32,
    pub scoring_mode: MultiTargetScoringMode,
}

pub trait MultiTargetScorerTrait: CoverScoreDataWithScoreCurveTrait {
    fn exclude_primary_target(&self) -> &bool;
    fn exclude_primary_target_mut(&mut self) -> &mut bool;
    fn max_distance_to_target(&self) -> &f32;
    fn max_distance_to_target_mut(&mut self) -> &mut f32;
    fn scoring_mode(&self) -> &MultiTargetScoringMode;
    fn scoring_mode_mut(&mut self) -> &mut MultiTargetScoringMode;
}

impl MultiTargetScorerTrait for MultiTargetScorer {
    fn exclude_primary_target(&self) -> &bool {
        &self.exclude_primary_target
    }
    fn exclude_primary_target_mut(&mut self) -> &mut bool {
        &mut self.exclude_primary_target
    }
    fn max_distance_to_target(&self) -> &f32 {
        &self.max_distance_to_target
    }
    fn max_distance_to_target_mut(&mut self) -> &mut f32 {
        &mut self.max_distance_to_target
    }
    fn scoring_mode(&self) -> &MultiTargetScoringMode {
        &self.scoring_mode
    }
    fn scoring_mode_mut(&mut self) -> &mut MultiTargetScoringMode {
        &mut self.scoring_mode
    }
}

impl CoverScoreDataWithScoreCurveTrait for MultiTargetScorer {
    fn score_curve(&self) -> &Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve()
    }
    fn score_curve_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve_mut()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_scale_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_scale_mut()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
    fn score_curve_max_y_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_max_y_mut()
    }
}

impl CoverScoreDataTrait for MultiTargetScorer {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn comment_mut(&mut self) -> &mut String {
        self._glacier_base.comment_mut()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for MultiTargetScorer {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn id_mut(&mut self) -> &mut u32 {
        self._glacier_base.id_mut()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
    fn runtime_filter_mut(&mut self) -> &mut u32 {
        self._glacier_base.runtime_filter_mut()
    }
}

impl super::core::DataContainerTrait for MultiTargetScorer {
}

pub static MULTITARGETSCORER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiTargetScorer",
    name_hash: 1360430087,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        super_class_offset: offset_of!(MultiTargetScorer, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MultiTargetScorer as Default>::default())),
            create_boxed: || Box::new(<MultiTargetScorer as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ExcludePrimaryTarget",
                name_hash: 3931146494,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MultiTargetScorer, exclude_primary_target),
            },
            FieldInfoData {
                name: "MaxDistanceToTarget",
                name_hash: 1927959608,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MultiTargetScorer, max_distance_to_target),
            },
            FieldInfoData {
                name: "ScoringMode",
                name_hash: 3421499531,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static MULTITARGETSCORER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiTargetScorer-Array",
    name_hash: 751510707,
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
    name_hash: 2967768339,
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


pub static MULTITARGETSCORINGMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiTargetScoringMode-Array",
    name_hash: 697239463,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("MultiTargetScoringMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TowardsPreferredWeaponRangeScoreData {
    pub _glacier_base: CoverScoreData,
    pub outside_preferred_range_score_curve: Option<LockedTypeObject /* super::core::FloatCurve */>,
    pub inside_preferred_range_score_curve: Option<LockedTypeObject /* super::core::FloatCurve */>,
}

pub trait TowardsPreferredWeaponRangeScoreDataTrait: CoverScoreDataTrait {
    fn outside_preferred_range_score_curve(&self) -> &Option<LockedTypeObject /* super::core::FloatCurve */>;
    fn outside_preferred_range_score_curve_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::FloatCurve */>;
    fn inside_preferred_range_score_curve(&self) -> &Option<LockedTypeObject /* super::core::FloatCurve */>;
    fn inside_preferred_range_score_curve_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::FloatCurve */>;
}

impl TowardsPreferredWeaponRangeScoreDataTrait for TowardsPreferredWeaponRangeScoreData {
    fn outside_preferred_range_score_curve(&self) -> &Option<LockedTypeObject /* super::core::FloatCurve */> {
        &self.outside_preferred_range_score_curve
    }
    fn outside_preferred_range_score_curve_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::FloatCurve */> {
        &mut self.outside_preferred_range_score_curve
    }
    fn inside_preferred_range_score_curve(&self) -> &Option<LockedTypeObject /* super::core::FloatCurve */> {
        &self.inside_preferred_range_score_curve
    }
    fn inside_preferred_range_score_curve_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::FloatCurve */> {
        &mut self.inside_preferred_range_score_curve
    }
}

impl CoverScoreDataTrait for TowardsPreferredWeaponRangeScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn comment_mut(&mut self) -> &mut String {
        self._glacier_base.comment_mut()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for TowardsPreferredWeaponRangeScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn id_mut(&mut self) -> &mut u32 {
        self._glacier_base.id_mut()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
    fn runtime_filter_mut(&mut self) -> &mut u32 {
        self._glacier_base.runtime_filter_mut()
    }
}

impl super::core::DataContainerTrait for TowardsPreferredWeaponRangeScoreData {
}

pub static TOWARDSPREFERREDWEAPONRANGESCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TowardsPreferredWeaponRangeScoreData",
    name_hash: 1787094413,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATA_TYPE_INFO),
        super_class_offset: offset_of!(TowardsPreferredWeaponRangeScoreData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TowardsPreferredWeaponRangeScoreData as Default>::default())),
            create_boxed: || Box::new(<TowardsPreferredWeaponRangeScoreData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "OutsidePreferredRangeScoreCurve",
                name_hash: 2236023989,
                flags: MemberInfoFlags::new(0),
                field_type: "FloatCurve",
                rust_offset: offset_of!(TowardsPreferredWeaponRangeScoreData, outside_preferred_range_score_curve),
            },
            FieldInfoData {
                name: "InsidePreferredRangeScoreCurve",
                name_hash: 2779002812,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static TOWARDSPREFERREDWEAPONRANGESCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TowardsPreferredWeaponRangeScoreData-Array",
    name_hash: 388204857,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("TowardsPreferredWeaponRangeScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PreferredWeaponRangeScoreData {
    pub _glacier_base: CoverScoreDataWithScoreCurve,
}

pub trait PreferredWeaponRangeScoreDataTrait: CoverScoreDataWithScoreCurveTrait {
}

impl PreferredWeaponRangeScoreDataTrait for PreferredWeaponRangeScoreData {
}

impl CoverScoreDataWithScoreCurveTrait for PreferredWeaponRangeScoreData {
    fn score_curve(&self) -> &Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve()
    }
    fn score_curve_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve_mut()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_scale_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_scale_mut()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
    fn score_curve_max_y_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_max_y_mut()
    }
}

impl CoverScoreDataTrait for PreferredWeaponRangeScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn comment_mut(&mut self) -> &mut String {
        self._glacier_base.comment_mut()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for PreferredWeaponRangeScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn id_mut(&mut self) -> &mut u32 {
        self._glacier_base.id_mut()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
    fn runtime_filter_mut(&mut self) -> &mut u32 {
        self._glacier_base.runtime_filter_mut()
    }
}

impl super::core::DataContainerTrait for PreferredWeaponRangeScoreData {
}

pub static PREFERREDWEAPONRANGESCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PreferredWeaponRangeScoreData",
    name_hash: 2845847077,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        super_class_offset: offset_of!(PreferredWeaponRangeScoreData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PreferredWeaponRangeScoreData as Default>::default())),
            create_boxed: || Box::new(<PreferredWeaponRangeScoreData as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static PREFERREDWEAPONRANGESCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PreferredWeaponRangeScoreData-Array",
    name_hash: 3319768721,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("PreferredWeaponRangeScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ProjectedDistanceScoreData {
    pub _glacier_base: CoverScoreDataWithRefPos,
    pub ref_direction: CoverScoreDirection,
    pub flip_ref_direction: bool,
}

pub trait ProjectedDistanceScoreDataTrait: CoverScoreDataWithRefPosTrait {
    fn ref_direction(&self) -> &CoverScoreDirection;
    fn ref_direction_mut(&mut self) -> &mut CoverScoreDirection;
    fn flip_ref_direction(&self) -> &bool;
    fn flip_ref_direction_mut(&mut self) -> &mut bool;
}

impl ProjectedDistanceScoreDataTrait for ProjectedDistanceScoreData {
    fn ref_direction(&self) -> &CoverScoreDirection {
        &self.ref_direction
    }
    fn ref_direction_mut(&mut self) -> &mut CoverScoreDirection {
        &mut self.ref_direction
    }
    fn flip_ref_direction(&self) -> &bool {
        &self.flip_ref_direction
    }
    fn flip_ref_direction_mut(&mut self) -> &mut bool {
        &mut self.flip_ref_direction
    }
}

impl CoverScoreDataWithRefPosTrait for ProjectedDistanceScoreData {
    fn ref_position(&self) -> &CoverScorePosition {
        self._glacier_base.ref_position()
    }
    fn ref_position_mut(&mut self) -> &mut CoverScorePosition {
        self._glacier_base.ref_position_mut()
    }
}

impl CoverScoreDataWithScoreCurveTrait for ProjectedDistanceScoreData {
    fn score_curve(&self) -> &Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve()
    }
    fn score_curve_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve_mut()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_scale_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_scale_mut()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
    fn score_curve_max_y_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_max_y_mut()
    }
}

impl CoverScoreDataTrait for ProjectedDistanceScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn comment_mut(&mut self) -> &mut String {
        self._glacier_base.comment_mut()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for ProjectedDistanceScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn id_mut(&mut self) -> &mut u32 {
        self._glacier_base.id_mut()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
    fn runtime_filter_mut(&mut self) -> &mut u32 {
        self._glacier_base.runtime_filter_mut()
    }
}

impl super::core::DataContainerTrait for ProjectedDistanceScoreData {
}

pub static PROJECTEDDISTANCESCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProjectedDistanceScoreData",
    name_hash: 4258192362,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHREFPOS_TYPE_INFO),
        super_class_offset: offset_of!(ProjectedDistanceScoreData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ProjectedDistanceScoreData as Default>::default())),
            create_boxed: || Box::new(<ProjectedDistanceScoreData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "RefDirection",
                name_hash: 1804247153,
                flags: MemberInfoFlags::new(0),
                field_type: "CoverScoreDirection",
                rust_offset: offset_of!(ProjectedDistanceScoreData, ref_direction),
            },
            FieldInfoData {
                name: "FlipRefDirection",
                name_hash: 2846396834,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static PROJECTEDDISTANCESCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProjectedDistanceScoreData-Array",
    name_hash: 3097118430,
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
    name_hash: 3557376357,
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


pub static COVERSCOREDIRECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoverScoreDirection-Array",
    name_hash: 896166481,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("CoverScoreDirection"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn ref_position_mut(&mut self) -> &mut CoverScorePosition {
        self._glacier_base.ref_position_mut()
    }
}

impl CoverScoreDataWithScoreCurveTrait for DistanceToActorScoreData {
    fn score_curve(&self) -> &Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve()
    }
    fn score_curve_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve_mut()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_scale_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_scale_mut()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
    fn score_curve_max_y_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_max_y_mut()
    }
}

impl CoverScoreDataTrait for DistanceToActorScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn comment_mut(&mut self) -> &mut String {
        self._glacier_base.comment_mut()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for DistanceToActorScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn id_mut(&mut self) -> &mut u32 {
        self._glacier_base.id_mut()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
    fn runtime_filter_mut(&mut self) -> &mut u32 {
        self._glacier_base.runtime_filter_mut()
    }
}

impl super::core::DataContainerTrait for DistanceToActorScoreData {
}

pub static DISTANCETOACTORSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistanceToActorScoreData",
    name_hash: 2456301646,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHREFPOS_TYPE_INFO),
        super_class_offset: offset_of!(DistanceToActorScoreData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DistanceToActorScoreData as Default>::default())),
            create_boxed: || Box::new(<DistanceToActorScoreData as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DISTANCETOACTORSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistanceToActorScoreData-Array",
    name_hash: 1437960698,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("DistanceToActorScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AngleFromPathTrajectoryScoreData {
    pub _glacier_base: CoverScoreDataWithScoreCurve,
    pub path_look_ahead_distance: f32,
}

pub trait AngleFromPathTrajectoryScoreDataTrait: CoverScoreDataWithScoreCurveTrait {
    fn path_look_ahead_distance(&self) -> &f32;
    fn path_look_ahead_distance_mut(&mut self) -> &mut f32;
}

impl AngleFromPathTrajectoryScoreDataTrait for AngleFromPathTrajectoryScoreData {
    fn path_look_ahead_distance(&self) -> &f32 {
        &self.path_look_ahead_distance
    }
    fn path_look_ahead_distance_mut(&mut self) -> &mut f32 {
        &mut self.path_look_ahead_distance
    }
}

impl CoverScoreDataWithScoreCurveTrait for AngleFromPathTrajectoryScoreData {
    fn score_curve(&self) -> &Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve()
    }
    fn score_curve_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve_mut()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_scale_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_scale_mut()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
    fn score_curve_max_y_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_max_y_mut()
    }
}

impl CoverScoreDataTrait for AngleFromPathTrajectoryScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn comment_mut(&mut self) -> &mut String {
        self._glacier_base.comment_mut()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for AngleFromPathTrajectoryScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn id_mut(&mut self) -> &mut u32 {
        self._glacier_base.id_mut()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
    fn runtime_filter_mut(&mut self) -> &mut u32 {
        self._glacier_base.runtime_filter_mut()
    }
}

impl super::core::DataContainerTrait for AngleFromPathTrajectoryScoreData {
}

pub static ANGLEFROMPATHTRAJECTORYSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngleFromPathTrajectoryScoreData",
    name_hash: 592728572,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        super_class_offset: offset_of!(AngleFromPathTrajectoryScoreData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AngleFromPathTrajectoryScoreData as Default>::default())),
            create_boxed: || Box::new(<AngleFromPathTrajectoryScoreData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "PathLookAheadDistance",
                name_hash: 2330456549,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static ANGLEFROMPATHTRAJECTORYSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngleFromPathTrajectoryScoreData-Array",
    name_hash: 3708189128,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AngleFromPathTrajectoryScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AngleFromReferenceDirectionScoreData {
    pub _glacier_base: CoverScoreDataWithScoreCurve,
    pub ref_dir_from_pos: CoverScorePosition,
    pub ref_dir_to_pos: CoverScorePosition,
}

pub trait AngleFromReferenceDirectionScoreDataTrait: CoverScoreDataWithScoreCurveTrait {
    fn ref_dir_from_pos(&self) -> &CoverScorePosition;
    fn ref_dir_from_pos_mut(&mut self) -> &mut CoverScorePosition;
    fn ref_dir_to_pos(&self) -> &CoverScorePosition;
    fn ref_dir_to_pos_mut(&mut self) -> &mut CoverScorePosition;
}

impl AngleFromReferenceDirectionScoreDataTrait for AngleFromReferenceDirectionScoreData {
    fn ref_dir_from_pos(&self) -> &CoverScorePosition {
        &self.ref_dir_from_pos
    }
    fn ref_dir_from_pos_mut(&mut self) -> &mut CoverScorePosition {
        &mut self.ref_dir_from_pos
    }
    fn ref_dir_to_pos(&self) -> &CoverScorePosition {
        &self.ref_dir_to_pos
    }
    fn ref_dir_to_pos_mut(&mut self) -> &mut CoverScorePosition {
        &mut self.ref_dir_to_pos
    }
}

impl CoverScoreDataWithScoreCurveTrait for AngleFromReferenceDirectionScoreData {
    fn score_curve(&self) -> &Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve()
    }
    fn score_curve_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve_mut()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_scale_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_scale_mut()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
    fn score_curve_max_y_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_max_y_mut()
    }
}

impl CoverScoreDataTrait for AngleFromReferenceDirectionScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn comment_mut(&mut self) -> &mut String {
        self._glacier_base.comment_mut()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for AngleFromReferenceDirectionScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn id_mut(&mut self) -> &mut u32 {
        self._glacier_base.id_mut()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
    fn runtime_filter_mut(&mut self) -> &mut u32 {
        self._glacier_base.runtime_filter_mut()
    }
}

impl super::core::DataContainerTrait for AngleFromReferenceDirectionScoreData {
}

pub static ANGLEFROMREFERENCEDIRECTIONSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngleFromReferenceDirectionScoreData",
    name_hash: 3460904804,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        super_class_offset: offset_of!(AngleFromReferenceDirectionScoreData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AngleFromReferenceDirectionScoreData as Default>::default())),
            create_boxed: || Box::new(<AngleFromReferenceDirectionScoreData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "RefDirFromPos",
                name_hash: 4037155889,
                flags: MemberInfoFlags::new(0),
                field_type: "CoverScorePosition",
                rust_offset: offset_of!(AngleFromReferenceDirectionScoreData, ref_dir_from_pos),
            },
            FieldInfoData {
                name: "RefDirToPos",
                name_hash: 1468045244,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static ANGLEFROMREFERENCEDIRECTIONSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngleFromReferenceDirectionScoreData-Array",
    name_hash: 845110096,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AngleFromReferenceDirectionScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AngleToActorScoreData2 {
    pub _glacier_base: CoverScoreData,
    pub ref_position: CoverScorePosition,
    pub scores_for_filter: Vec<Option<LockedTypeObject /* ScoreCurveForFilter */>>,
}

pub trait AngleToActorScoreData2Trait: CoverScoreDataTrait {
    fn ref_position(&self) -> &CoverScorePosition;
    fn ref_position_mut(&mut self) -> &mut CoverScorePosition;
    fn scores_for_filter(&self) -> &Vec<Option<LockedTypeObject /* ScoreCurveForFilter */>>;
    fn scores_for_filter_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* ScoreCurveForFilter */>>;
}

impl AngleToActorScoreData2Trait for AngleToActorScoreData2 {
    fn ref_position(&self) -> &CoverScorePosition {
        &self.ref_position
    }
    fn ref_position_mut(&mut self) -> &mut CoverScorePosition {
        &mut self.ref_position
    }
    fn scores_for_filter(&self) -> &Vec<Option<LockedTypeObject /* ScoreCurveForFilter */>> {
        &self.scores_for_filter
    }
    fn scores_for_filter_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* ScoreCurveForFilter */>> {
        &mut self.scores_for_filter
    }
}

impl CoverScoreDataTrait for AngleToActorScoreData2 {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn comment_mut(&mut self) -> &mut String {
        self._glacier_base.comment_mut()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for AngleToActorScoreData2 {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn id_mut(&mut self) -> &mut u32 {
        self._glacier_base.id_mut()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
    fn runtime_filter_mut(&mut self) -> &mut u32 {
        self._glacier_base.runtime_filter_mut()
    }
}

impl super::core::DataContainerTrait for AngleToActorScoreData2 {
}

pub static ANGLETOACTORSCOREDATA2_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngleToActorScoreData2",
    name_hash: 2801177630,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATA_TYPE_INFO),
        super_class_offset: offset_of!(AngleToActorScoreData2, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AngleToActorScoreData2 as Default>::default())),
            create_boxed: || Box::new(<AngleToActorScoreData2 as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "RefPosition",
                name_hash: 114250413,
                flags: MemberInfoFlags::new(0),
                field_type: "CoverScorePosition",
                rust_offset: offset_of!(AngleToActorScoreData2, ref_position),
            },
            FieldInfoData {
                name: "ScoresForFilter",
                name_hash: 3649816933,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static ANGLETOACTORSCOREDATA2_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngleToActorScoreData2-Array",
    name_hash: 1681016106,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AngleToActorScoreData2"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ScoreCurveForFilter {
    pub _glacier_base: super::core::DataContainer,
    pub score_curve: Option<LockedTypeObject /* super::core::FloatCurve */>,
    pub runtime_filter: u32,
}

pub trait ScoreCurveForFilterTrait: super::core::DataContainerTrait {
    fn score_curve(&self) -> &Option<LockedTypeObject /* super::core::FloatCurve */>;
    fn score_curve_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::FloatCurve */>;
    fn runtime_filter(&self) -> &u32;
    fn runtime_filter_mut(&mut self) -> &mut u32;
}

impl ScoreCurveForFilterTrait for ScoreCurveForFilter {
    fn score_curve(&self) -> &Option<LockedTypeObject /* super::core::FloatCurve */> {
        &self.score_curve
    }
    fn score_curve_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::FloatCurve */> {
        &mut self.score_curve
    }
    fn runtime_filter(&self) -> &u32 {
        &self.runtime_filter
    }
    fn runtime_filter_mut(&mut self) -> &mut u32 {
        &mut self.runtime_filter
    }
}

impl super::core::DataContainerTrait for ScoreCurveForFilter {
}

pub static SCORECURVEFORFILTER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScoreCurveForFilter",
    name_hash: 3196619041,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(ScoreCurveForFilter, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ScoreCurveForFilter as Default>::default())),
            create_boxed: || Box::new(<ScoreCurveForFilter as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ScoreCurve",
                name_hash: 3850135130,
                flags: MemberInfoFlags::new(0),
                field_type: "FloatCurve",
                rust_offset: offset_of!(ScoreCurveForFilter, score_curve),
            },
            FieldInfoData {
                name: "RuntimeFilter",
                name_hash: 1515778553,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SCORECURVEFORFILTER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScoreCurveForFilter-Array",
    name_hash: 3139474325,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("ScoreCurveForFilter"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn ref_position_mut(&mut self) -> &mut CoverScorePosition {
        self._glacier_base.ref_position_mut()
    }
}

impl CoverScoreDataWithScoreCurveTrait for AngleToActorScoreData {
    fn score_curve(&self) -> &Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve()
    }
    fn score_curve_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve_mut()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_scale_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_scale_mut()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
    fn score_curve_max_y_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_max_y_mut()
    }
}

impl CoverScoreDataTrait for AngleToActorScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn comment_mut(&mut self) -> &mut String {
        self._glacier_base.comment_mut()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for AngleToActorScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn id_mut(&mut self) -> &mut u32 {
        self._glacier_base.id_mut()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
    fn runtime_filter_mut(&mut self) -> &mut u32 {
        self._glacier_base.runtime_filter_mut()
    }
}

impl super::core::DataContainerTrait for AngleToActorScoreData {
}

pub static ANGLETOACTORSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngleToActorScoreData",
    name_hash: 1126088364,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHREFPOS_TYPE_INFO),
        super_class_offset: offset_of!(AngleToActorScoreData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AngleToActorScoreData as Default>::default())),
            create_boxed: || Box::new(<AngleToActorScoreData as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static ANGLETOACTORSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngleToActorScoreData-Array",
    name_hash: 3000020504,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("AngleToActorScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CoverScoreDataWithRefPos {
    pub _glacier_base: CoverScoreDataWithScoreCurve,
    pub ref_position: CoverScorePosition,
}

pub trait CoverScoreDataWithRefPosTrait: CoverScoreDataWithScoreCurveTrait {
    fn ref_position(&self) -> &CoverScorePosition;
    fn ref_position_mut(&mut self) -> &mut CoverScorePosition;
}

impl CoverScoreDataWithRefPosTrait for CoverScoreDataWithRefPos {
    fn ref_position(&self) -> &CoverScorePosition {
        &self.ref_position
    }
    fn ref_position_mut(&mut self) -> &mut CoverScorePosition {
        &mut self.ref_position
    }
}

impl CoverScoreDataWithScoreCurveTrait for CoverScoreDataWithRefPos {
    fn score_curve(&self) -> &Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve()
    }
    fn score_curve_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::FloatCurve */> {
        self._glacier_base.score_curve_mut()
    }
    fn score_curve_scale(&self) -> &f32 {
        self._glacier_base.score_curve_scale()
    }
    fn score_curve_scale_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_scale_mut()
    }
    fn score_curve_max_y(&self) -> &f32 {
        self._glacier_base.score_curve_max_y()
    }
    fn score_curve_max_y_mut(&mut self) -> &mut f32 {
        self._glacier_base.score_curve_max_y_mut()
    }
}

impl CoverScoreDataTrait for CoverScoreDataWithRefPos {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn comment_mut(&mut self) -> &mut String {
        self._glacier_base.comment_mut()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for CoverScoreDataWithRefPos {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn id_mut(&mut self) -> &mut u32 {
        self._glacier_base.id_mut()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
    fn runtime_filter_mut(&mut self) -> &mut u32 {
        self._glacier_base.runtime_filter_mut()
    }
}

impl super::core::DataContainerTrait for CoverScoreDataWithRefPos {
}

pub static COVERSCOREDATAWITHREFPOS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoverScoreDataWithRefPos",
    name_hash: 1733761039,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATAWITHSCORECURVE_TYPE_INFO),
        super_class_offset: offset_of!(CoverScoreDataWithRefPos, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoverScoreDataWithRefPos as Default>::default())),
            create_boxed: || Box::new(<CoverScoreDataWithRefPos as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "RefPosition",
                name_hash: 114250413,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static COVERSCOREDATAWITHREFPOS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoverScoreDataWithRefPos-Array",
    name_hash: 3769242811,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("CoverScoreDataWithRefPos"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CoverScoreDataWithScoreCurve {
    pub _glacier_base: CoverScoreData,
    pub score_curve: Option<LockedTypeObject /* super::core::FloatCurve */>,
    pub score_curve_scale: f32,
    pub score_curve_max_y: f32,
}

pub trait CoverScoreDataWithScoreCurveTrait: CoverScoreDataTrait {
    fn score_curve(&self) -> &Option<LockedTypeObject /* super::core::FloatCurve */>;
    fn score_curve_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::FloatCurve */>;
    fn score_curve_scale(&self) -> &f32;
    fn score_curve_scale_mut(&mut self) -> &mut f32;
    fn score_curve_max_y(&self) -> &f32;
    fn score_curve_max_y_mut(&mut self) -> &mut f32;
}

impl CoverScoreDataWithScoreCurveTrait for CoverScoreDataWithScoreCurve {
    fn score_curve(&self) -> &Option<LockedTypeObject /* super::core::FloatCurve */> {
        &self.score_curve
    }
    fn score_curve_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::FloatCurve */> {
        &mut self.score_curve
    }
    fn score_curve_scale(&self) -> &f32 {
        &self.score_curve_scale
    }
    fn score_curve_scale_mut(&mut self) -> &mut f32 {
        &mut self.score_curve_scale
    }
    fn score_curve_max_y(&self) -> &f32 {
        &self.score_curve_max_y
    }
    fn score_curve_max_y_mut(&mut self) -> &mut f32 {
        &mut self.score_curve_max_y
    }
}

impl CoverScoreDataTrait for CoverScoreDataWithScoreCurve {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn comment_mut(&mut self) -> &mut String {
        self._glacier_base.comment_mut()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for CoverScoreDataWithScoreCurve {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn id_mut(&mut self) -> &mut u32 {
        self._glacier_base.id_mut()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
    fn runtime_filter_mut(&mut self) -> &mut u32 {
        self._glacier_base.runtime_filter_mut()
    }
}

impl super::core::DataContainerTrait for CoverScoreDataWithScoreCurve {
}

pub static COVERSCOREDATAWITHSCORECURVE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoverScoreDataWithScoreCurve",
    name_hash: 3461241901,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATA_TYPE_INFO),
        super_class_offset: offset_of!(CoverScoreDataWithScoreCurve, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoverScoreDataWithScoreCurve as Default>::default())),
            create_boxed: || Box::new(<CoverScoreDataWithScoreCurve as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ScoreCurve",
                name_hash: 3850135130,
                flags: MemberInfoFlags::new(0),
                field_type: "FloatCurve",
                rust_offset: offset_of!(CoverScoreDataWithScoreCurve, score_curve),
            },
            FieldInfoData {
                name: "ScoreCurveScale",
                name_hash: 1983536674,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CoverScoreDataWithScoreCurve, score_curve_scale),
            },
            FieldInfoData {
                name: "ScoreCurveMaxY",
                name_hash: 2143159799,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static COVERSCOREDATAWITHSCORECURVE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoverScoreDataWithScoreCurve-Array",
    name_hash: 1853337753,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("CoverScoreDataWithScoreCurve"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CurrentCoverBonusScoreData {
    pub _glacier_base: CoverScoreData,
    pub bonus_score: f32,
}

pub trait CurrentCoverBonusScoreDataTrait: CoverScoreDataTrait {
    fn bonus_score(&self) -> &f32;
    fn bonus_score_mut(&mut self) -> &mut f32;
}

impl CurrentCoverBonusScoreDataTrait for CurrentCoverBonusScoreData {
    fn bonus_score(&self) -> &f32 {
        &self.bonus_score
    }
    fn bonus_score_mut(&mut self) -> &mut f32 {
        &mut self.bonus_score
    }
}

impl CoverScoreDataTrait for CurrentCoverBonusScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn comment_mut(&mut self) -> &mut String {
        self._glacier_base.comment_mut()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for CurrentCoverBonusScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn id_mut(&mut self) -> &mut u32 {
        self._glacier_base.id_mut()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
    fn runtime_filter_mut(&mut self) -> &mut u32 {
        self._glacier_base.runtime_filter_mut()
    }
}

impl super::core::DataContainerTrait for CurrentCoverBonusScoreData {
}

pub static CURRENTCOVERBONUSSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CurrentCoverBonusScoreData",
    name_hash: 1069847676,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATA_TYPE_INFO),
        super_class_offset: offset_of!(CurrentCoverBonusScoreData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CurrentCoverBonusScoreData as Default>::default())),
            create_boxed: || Box::new(<CurrentCoverBonusScoreData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "BonusScore",
                name_hash: 711777544,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CURRENTCOVERBONUSSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CurrentCoverBonusScoreData-Array",
    name_hash: 2386467912,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("CurrentCoverBonusScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CoverFilterScoreData {
    pub _glacier_base: CoverScoreData,
    pub matching_score: f32,
}

pub trait CoverFilterScoreDataTrait: CoverScoreDataTrait {
    fn matching_score(&self) -> &f32;
    fn matching_score_mut(&mut self) -> &mut f32;
}

impl CoverFilterScoreDataTrait for CoverFilterScoreData {
    fn matching_score(&self) -> &f32 {
        &self.matching_score
    }
    fn matching_score_mut(&mut self) -> &mut f32 {
        &mut self.matching_score
    }
}

impl CoverScoreDataTrait for CoverFilterScoreData {
    fn comment(&self) -> &String {
        self._glacier_base.comment()
    }
    fn comment_mut(&mut self) -> &mut String {
        self._glacier_base.comment_mut()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for CoverFilterScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn id_mut(&mut self) -> &mut u32 {
        self._glacier_base.id_mut()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
    fn runtime_filter_mut(&mut self) -> &mut u32 {
        self._glacier_base.runtime_filter_mut()
    }
}

impl super::core::DataContainerTrait for CoverFilterScoreData {
}

pub static COVERFILTERSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoverFilterScoreData",
    name_hash: 2173618672,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COVERSCOREDATA_TYPE_INFO),
        super_class_offset: offset_of!(CoverFilterScoreData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoverFilterScoreData as Default>::default())),
            create_boxed: || Box::new(<CoverFilterScoreData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "MatchingScore",
                name_hash: 1737084798,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static COVERFILTERSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoverFilterScoreData-Array",
    name_hash: 1736173508,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("CoverFilterScoreData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CoverScoreData {
    pub _glacier_base: super::battle_a_i_shared::CoverScoreDataBase,
    pub comment: String,
    pub enabled: bool,
}

pub trait CoverScoreDataTrait: super::battle_a_i_shared::CoverScoreDataBaseTrait {
    fn comment(&self) -> &String;
    fn comment_mut(&mut self) -> &mut String;
    fn enabled(&self) -> &bool;
    fn enabled_mut(&mut self) -> &mut bool;
}

impl CoverScoreDataTrait for CoverScoreData {
    fn comment(&self) -> &String {
        &self.comment
    }
    fn comment_mut(&mut self) -> &mut String {
        &mut self.comment
    }
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
}

impl super::battle_a_i_shared::CoverScoreDataBaseTrait for CoverScoreData {
    fn id(&self) -> &u32 {
        self._glacier_base.id()
    }
    fn id_mut(&mut self) -> &mut u32 {
        self._glacier_base.id_mut()
    }
    fn runtime_filter(&self) -> &u32 {
        self._glacier_base.runtime_filter()
    }
    fn runtime_filter_mut(&mut self) -> &mut u32 {
        self._glacier_base.runtime_filter_mut()
    }
}

impl super::core::DataContainerTrait for CoverScoreData {
}

pub static COVERSCOREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoverScoreData",
    name_hash: 1191527472,
    flags: MemberInfoFlags::new(101),
    module: "BattleAI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::battle_a_i_shared::COVERSCOREDATABASE_TYPE_INFO),
        super_class_offset: offset_of!(CoverScoreData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoverScoreData as Default>::default())),
            create_boxed: || Box::new(<CoverScoreData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Comment",
                name_hash: 3657623350,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(CoverScoreData, comment),
            },
            FieldInfoData {
                name: "Enabled",
                name_hash: 2662400,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static COVERSCOREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoverScoreData-Array",
    name_hash: 1975062916,
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
    name_hash: 966364473,
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


pub static COVERSCOREPOSITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoverScorePosition-Array",
    name_hash: 2046213005,
    flags: MemberInfoFlags::new(145),
    module: "BattleAI",
    data: TypeInfoData::Array("CoverScorePosition"),
    array_type: None,
    alignment: 8,
};


