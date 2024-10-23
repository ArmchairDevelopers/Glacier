use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_dice_commons_shared_types(registry: &mut TypeRegistry) {
    registry.register_type(BLUEPRINTPROXYENTITYBASE_TYPE_INFO);
    registry.register_type(BLUEPRINTPROXYENTITYBASE_ARRAY_TYPE_INFO);
    registry.register_type(DRAWTEXT2D_INT32_INT32_VEC4_VEC4__TYPE_INFO);
    registry.register_type(DRAWTEXT2D_INT32_INT32_VEC4_VEC3__TYPE_INFO);
    registry.register_type(DRAWTEXT2D_INT32_INT32_VEC4_INT32__TYPE_INFO);
    registry.register_type(DRAWTEXT2D_INT32_INT32_VEC4_FLOAT32__TYPE_INFO);
    registry.register_type(DRAWCOORDINATESYSTEM_MAT4__TYPE_INFO);
    registry.register_type(VALUEMATCHENTITYDATA_TYPE_INFO);
    registry.register_type(VALUEMATCHENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(MATCHANDTRIGGERITEM_TYPE_INFO);
    registry.register_type(MATCHANDTRIGGERITEM_ARRAY_TYPE_INFO);
    registry.register_type(TIMEEVENTTRIGGER_TYPE_INFO);
    registry.register_type(TIMEEVENTTRIGGER_ARRAY_TYPE_INFO);
    registry.register_type(INPUTVALUEMATCH_TYPE_INFO);
    registry.register_type(INPUTVALUEMATCH_ARRAY_TYPE_INFO);
    registry.register_type(SIMPLEROTATIONENTITYDATA_TYPE_INFO);
    registry.register_type(SIMPLEROTATIONENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PINGPONGROTATIONTRANSFORMMODIFIERDATA_TYPE_INFO);
    registry.register_type(PINGPONGROTATIONTRANSFORMMODIFIERDATA_ARRAY_TYPE_INFO);
    registry.register_type(ROTATIONTRANSFORMMODIFIERDATA_TYPE_INFO);
    registry.register_type(ROTATIONTRANSFORMMODIFIERDATA_ARRAY_TYPE_INFO);
    registry.register_type(TRANSFORMMODIFIERDATA_TYPE_INFO);
    registry.register_type(TRANSFORMMODIFIERDATA_ARRAY_TYPE_INFO);
    registry.register_type(TRANSFORMMODIFIERTYPE_TYPE_INFO);
    registry.register_type(TRANSFORMMODIFIERTYPE_ARRAY_TYPE_INFO);
    registry.register_type(SHADOWPLAYHIGHLIGHTENTITYDATA_TYPE_INFO);
    registry.register_type(SHADOWPLAYHIGHLIGHTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SHADOWPLAYSETTINGS_TYPE_INFO);
    registry.register_type(SHADOWPLAYSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(SELECTABLEACTIONENTITYDATA_TYPE_INFO);
    registry.register_type(SELECTABLEACTIONENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(RUMBLEENTITYDATA_TYPE_INFO);
    registry.register_type(RUMBLEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(RAYCASTDIRECTIONENTITYDATA_TYPE_INFO);
    registry.register_type(RAYCASTDIRECTIONENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(RANDOMACTIONSELECTORENTITYDATA_TYPE_INFO);
    registry.register_type(RANDOMACTIONSELECTORENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PROPERTYSTATUSENTITYDATA_TYPE_INFO);
    registry.register_type(PROPERTYSTATUSENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PROPERTYSELECTENTITYDATA_TYPE_INFO);
    registry.register_type(PROPERTYSELECTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PRIORITIZEDBOOLENTITYDATA_TYPE_INFO);
    registry.register_type(PRIORITIZEDBOOLENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(NESTEDCONDITIONALPROPERTYENTITYDATA_TYPE_INFO);
    registry.register_type(NESTEDCONDITIONALPROPERTYENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(MULTIPROPERTYGATEENTITYDATA_TYPE_INFO);
    registry.register_type(MULTIPROPERTYGATEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(MULTIPROPERTYGATEPROPERTYINFO_TYPE_INFO);
    registry.register_type(MULTIPROPERTYGATEPROPERTYINFO_ARRAY_TYPE_INFO);
    registry.register_type(LOGITECHLEDPULSEEFFECTENTITYDATA_TYPE_INFO);
    registry.register_type(LOGITECHLEDPULSEEFFECTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(LOGITECHLEDINPUTCONCEPTIDENTIFIERENTITYDATA_TYPE_INFO);
    registry.register_type(LOGITECHLEDINPUTCONCEPTIDENTIFIERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(LOGITECHLEDFADEEFFECTENTITYDATA_TYPE_INFO);
    registry.register_type(LOGITECHLEDFADEEFFECTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(LOGITECHLEDCONSTANTEFFECTENTITYDATA_TYPE_INFO);
    registry.register_type(LOGITECHLEDCONSTANTEFFECTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(LOGITECHLEDCONDITIONALINPUTCONCEPTIDENTIFIERENTITYDATA_TYPE_INFO);
    registry.register_type(LOGITECHLEDCONDITIONALINPUTCONCEPTIDENTIFIERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(LOGITECHLEDBARENTITYDATA_TYPE_INFO);
    registry.register_type(LOGITECHLEDBARENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(LOCALLOCATORENTITYDATA_TYPE_INFO);
    registry.register_type(LOCALLOCATORENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(WAVESELECTORNODEDATA_TYPE_INFO);
    registry.register_type(WAVESELECTORNODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(INPUTWAVESGROUP_TYPE_INFO);
    registry.register_type(INPUTWAVESGROUP_ARRAY_TYPE_INFO);
    registry.register_type(VALUECACHENODEDATA_TYPE_INFO);
    registry.register_type(VALUECACHENODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(SYSTEMINFORMATIONNODEDATA_TYPE_INFO);
    registry.register_type(SYSTEMINFORMATIONNODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(SYNCEDRANDOMINTNODEDATA_TYPE_INFO);
    registry.register_type(SYNCEDRANDOMINTNODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(STEPLOGICSAMPLERNODECONFIGDATA_TYPE_INFO);
    registry.register_type(STEPLOGICSAMPLERNODECONFIGDATA_ARRAY_TYPE_INFO);
    registry.register_type(STEPLOGICSAMPLERNODEDATA_TYPE_INFO);
    registry.register_type(STEPLOGICSAMPLERNODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(STEPLOGICSAMPLERNODEDEBUGDATA_TYPE_INFO);
    registry.register_type(STEPLOGICSAMPLERNODEDEBUGDATA_ARRAY_TYPE_INFO);
    registry.register_type(STEPLOGICSAMPLERPLUGINS_TYPE_INFO);
    registry.register_type(STEPLOGICSAMPLERPLUGINS_ARRAY_TYPE_INFO);
    registry.register_type(SORTVALUESNODEDATA_TYPE_INFO);
    registry.register_type(SORTVALUESNODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(SORTVALUESGROUP_TYPE_INFO);
    registry.register_type(SORTVALUESGROUP_ARRAY_TYPE_INFO);
    registry.register_type(SEQUENCERNODEDATA_TYPE_INFO);
    registry.register_type(SEQUENCERNODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(TIMEMODE_TYPE_INFO);
    registry.register_type(TIMEMODE_ARRAY_TYPE_INFO);
    registry.register_type(SATURATIONNODEDATA_TYPE_INFO);
    registry.register_type(SATURATIONNODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(SATURATIONTYPES_TYPE_INFO);
    registry.register_type(SATURATIONTYPES_ARRAY_TYPE_INFO);
    registry.register_type(RUNONCENODEDATA_TYPE_INFO);
    registry.register_type(RUNONCENODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(RAYCASTNODEDATA_TYPE_INFO);
    registry.register_type(RAYCASTNODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(RANDOMINTEGERNODEDATA_TYPE_INFO);
    registry.register_type(RANDOMINTEGERNODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PROFILEOPTIONSREADERNODEDATA_TYPE_INFO);
    registry.register_type(PROFILEOPTIONSREADERNODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PROFILEOPTIONSGROUP_TYPE_INFO);
    registry.register_type(PROFILEOPTIONSGROUP_ARRAY_TYPE_INFO);
    registry.register_type(PASSBYDETECTORNODEDATA_TYPE_INFO);
    registry.register_type(PASSBYDETECTORNODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PARAMETERSMOOTHERNODEDATA_TYPE_INFO);
    registry.register_type(PARAMETERSMOOTHERNODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(NANCHECKNODEDATA_TYPE_INFO);
    registry.register_type(NANCHECKNODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(LOUDNESSNODEDATA_TYPE_INFO);
    registry.register_type(LOUDNESSNODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(LISTENERNODEDATA_TYPE_INFO);
    registry.register_type(LISTENERNODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(INDEXMAPPERNODECONFIGDATA_TYPE_INFO);
    registry.register_type(INDEXMAPPERNODECONFIGDATA_ARRAY_TYPE_INFO);
    registry.register_type(INDEXMAPPERCONFIGGROUP_TYPE_INFO);
    registry.register_type(INDEXMAPPERCONFIGGROUP_ARRAY_TYPE_INFO);
    registry.register_type(INDEXMAPPERNODEDATA_TYPE_INFO);
    registry.register_type(INDEXMAPPERNODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(INDEXMAPPERGROUP_TYPE_INFO);
    registry.register_type(INDEXMAPPERGROUP_ARRAY_TYPE_INFO);
    registry.register_type(GATENODEDATA_TYPE_INFO);
    registry.register_type(GATENODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(FORCEISNOTLOOPINGNODEDATA_TYPE_INFO);
    registry.register_type(FORCEISNOTLOOPINGNODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(FORCEISLOOPINGNODEDATA_TYPE_INFO);
    registry.register_type(FORCEISLOOPINGNODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(EVENTSELECTORNODEDATA_TYPE_INFO);
    registry.register_type(EVENTSELECTORNODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(INPUTEVENTSGROUP_TYPE_INFO);
    registry.register_type(INPUTEVENTSGROUP_ARRAY_TYPE_INFO);
    registry.register_type(EVENTGATECONDITIONVALUENODEDATA_TYPE_INFO);
    registry.register_type(EVENTGATECONDITIONVALUENODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(EVENTGATECONDITIONVALUETYPE_TYPE_INFO);
    registry.register_type(EVENTGATECONDITIONVALUETYPE_ARRAY_TYPE_INFO);
    registry.register_type(DICEPHYSICSNODEDATA_TYPE_INFO);
    registry.register_type(DICEPHYSICSNODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(DICEDIVISIBLELOOPPLAYERNODEDATA_TYPE_INFO);
    registry.register_type(DICEDIVISIBLELOOPPLAYERNODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(DICEDIVISIBLELOOPPLAYERPLUGINS_TYPE_INFO);
    registry.register_type(DICEDIVISIBLELOOPPLAYERPLUGINS_ARRAY_TYPE_INFO);
    registry.register_type(DICEADSRNODEDATA_TYPE_INFO);
    registry.register_type(DICEADSRNODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(COUNTERNODEDATA_TYPE_INFO);
    registry.register_type(COUNTERNODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(CONFIGURABLERANGEMAPPERNODEDATA_TYPE_INFO);
    registry.register_type(CONFIGURABLERANGEMAPPERNODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(CONFIGURABLERANGEMAPPERENTRY_TYPE_INFO);
    registry.register_type(CONFIGURABLERANGEMAPPERENTRY_ARRAY_TYPE_INFO);
    registry.register_type(CONDITIONVALUENODEDATA_TYPE_INFO);
    registry.register_type(CONDITIONVALUENODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(CONDITIONVALUEGROUP_TYPE_INFO);
    registry.register_type(CONDITIONVALUEGROUP_ARRAY_TYPE_INFO);
    registry.register_type(CONDITIONVALUETYPE_TYPE_INFO);
    registry.register_type(CONDITIONVALUETYPE_ARRAY_TYPE_INFO);
    registry.register_type(COMPAREVALUENODEDATA_TYPE_INFO);
    registry.register_type(COMPAREVALUENODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(COMPAREVALUEGROUP_TYPE_INFO);
    registry.register_type(COMPAREVALUEGROUP_ARRAY_TYPE_INFO);
    registry.register_type(COMPAREVALUECONDITIONTYPE_TYPE_INFO);
    registry.register_type(COMPAREVALUECONDITIONTYPE_ARRAY_TYPE_INFO);
    registry.register_type(CLAMPNODEDATA_TYPE_INFO);
    registry.register_type(CLAMPNODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(AUDIOENVELOPESWITCHERNODEDATA_TYPE_INFO);
    registry.register_type(AUDIOENVELOPESWITCHERNODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(AUDIOENVELOPESWITCHERNODECONFIGDATA_TYPE_INFO);
    registry.register_type(AUDIOENVELOPESWITCHERNODECONFIGDATA_ARRAY_TYPE_INFO);
    registry.register_type(AUDIOENVELOPENODEDATA_TYPE_INFO);
    registry.register_type(AUDIOENVELOPENODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(AUDIOENVELOPEASSET_TYPE_INFO);
    registry.register_type(AUDIOENVELOPEASSET_ARRAY_TYPE_INFO);
    registry.register_type(AUDIOENVELOPE_TYPE_INFO);
    registry.register_type(AUDIOENVELOPE_ARRAY_TYPE_INFO);
    registry.register_type(AUDIOENVELOPEPOINT_TYPE_INFO);
    registry.register_type(AUDIOENVELOPEPOINT_ARRAY_TYPE_INFO);
    registry.register_type(SNAPTOGRIDGRANULARITY_TYPE_INFO);
    registry.register_type(SNAPTOGRIDGRANULARITY_ARRAY_TYPE_INFO);
    registry.register_type(AUDIOENVELOPELINETYPE_TYPE_INFO);
    registry.register_type(AUDIOENVELOPELINETYPE_ARRAY_TYPE_INFO);
    registry.register_type(ASSETCROSSFADERNODEDATA_TYPE_INFO);
    registry.register_type(ASSETCROSSFADERNODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(ARONESHOTNODEDATA_TYPE_INFO);
    registry.register_type(ARONESHOTNODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(ARLOOPINGNODEDATA_TYPE_INFO);
    registry.register_type(ARLOOPINGNODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(ARFLIPFLOPNODEDATA_TYPE_INFO);
    registry.register_type(ARFLIPFLOPNODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(LISTENERAREATYPENODEDATA_TYPE_INFO);
    registry.register_type(LISTENERAREATYPENODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(AREATYPENODEDATA_TYPE_INFO);
    registry.register_type(AREATYPENODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(VOICEOVERSOURCEENTITYDATA_TYPE_INFO);
    registry.register_type(VOICEOVERSOURCEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(VOICEOVERSETLANGUAGEENTITYDATA_TYPE_INFO);
    registry.register_type(VOICEOVERSETLANGUAGEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(LANGUAGECOLLECTION_TYPE_INFO);
    registry.register_type(LANGUAGECOLLECTION_ARRAY_TYPE_INFO);
    registry.register_type(LANGUAGECOLLECTIONELEMENT_TYPE_INFO);
    registry.register_type(LANGUAGECOLLECTIONELEMENT_ARRAY_TYPE_INFO);
    registry.register_type(VOICEOVERLOCATORENTITYDATA_TYPE_INFO);
    registry.register_type(VOICEOVERLOCATORENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENTRYCONTROLLABLEATTACHDATA_TYPE_INFO);
    registry.register_type(ENTRYCONTROLLABLEATTACHDATA_ARRAY_TYPE_INFO);
    registry.register_type(CONTROLLABLEATTACHDATA_TYPE_INFO);
    registry.register_type(CONTROLLABLEATTACHDATA_ARRAY_TYPE_INFO);
    registry.register_type(MODELANIMATIONATTACHDATA_TYPE_INFO);
    registry.register_type(MODELANIMATIONATTACHDATA_ARRAY_TYPE_INFO);
    registry.register_type(ANIMATABLEATTACHDATA_TYPE_INFO);
    registry.register_type(ANIMATABLEATTACHDATA_ARRAY_TYPE_INFO);
    registry.register_type(COMPONENTATTACHDATA_TYPE_INFO);
    registry.register_type(COMPONENTATTACHDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENTITYATTACHDATA_TYPE_INFO);
    registry.register_type(ENTITYATTACHDATA_ARRAY_TYPE_INFO);
    registry.register_type(CONTROLLABLETRANSFORMLINKDATA_TYPE_INFO);
    registry.register_type(CONTROLLABLETRANSFORMLINKDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENTRYCONTROLLABLETRANSFORMLINKDATA_TYPE_INFO);
    registry.register_type(ENTRYCONTROLLABLETRANSFORMLINKDATA_ARRAY_TYPE_INFO);
    registry.register_type(COMPONENTTRANSFORMLINKDATA_TYPE_INFO);
    registry.register_type(COMPONENTTRANSFORMLINKDATA_ARRAY_TYPE_INFO);
    registry.register_type(MODELANIMATIONTRANSFORMLINKDATA_TYPE_INFO);
    registry.register_type(MODELANIMATIONTRANSFORMLINKDATA_ARRAY_TYPE_INFO);
    registry.register_type(ANIMATABLETRANSFORMLINKDATA_TYPE_INFO);
    registry.register_type(ANIMATABLETRANSFORMLINKDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENTITYCENTERTRANSFORMLINKDATA_TYPE_INFO);
    registry.register_type(ENTITYCENTERTRANSFORMLINKDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENTITYTRANSFORMLINKDATA_TYPE_INFO);
    registry.register_type(ENTITYTRANSFORMLINKDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENTITYLINKDATA_TYPE_INFO);
    registry.register_type(ENTITYLINKDATA_ARRAY_TYPE_INFO);
    registry.register_type(OFFSETMODIFICATIONDATA_TYPE_INFO);
    registry.register_type(OFFSETMODIFICATIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(COORDINATEMODIFICATIONDATA_TYPE_INFO);
    registry.register_type(COORDINATEMODIFICATIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(WIDGETREFERENCEENTITYDATA_TYPE_INFO);
    registry.register_type(WIDGETREFERENCEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(DICEUIVECTORSHAPEASSET_TYPE_INFO);
    registry.register_type(DICEUIVECTORSHAPEASSET_ARRAY_TYPE_INFO);
    registry.register_type(DICEUIVECTORSHAPE_TYPE_INFO);
    registry.register_type(DICEUIVECTORSHAPE_ARRAY_TYPE_INFO);
    registry.register_type(DICEUIVECTORPATH_TYPE_INFO);
    registry.register_type(DICEUIVECTORPATH_ARRAY_TYPE_INFO);
    registry.register_type(DICEUIVECTORPATHCORNER_TYPE_INFO);
    registry.register_type(DICEUIVECTORPATHCORNER_ARRAY_TYPE_INFO);
    registry.register_type(DICEUIVECTORPATHCORNERTYPE_TYPE_INFO);
    registry.register_type(DICEUIVECTORPATHCORNERTYPE_ARRAY_TYPE_INFO);
    registry.register_type(DICEUIVECTORSHAPEDRAWSTYLE_TYPE_INFO);
    registry.register_type(DICEUIVECTORSHAPEDRAWSTYLE_ARRAY_TYPE_INFO);
    registry.register_type(DICEUIVECTORSHAPECAPTYPE_TYPE_INFO);
    registry.register_type(DICEUIVECTORSHAPECAPTYPE_ARRAY_TYPE_INFO);
    registry.register_type(LOCALIZEDSTRINGIDPICKERENTITYDATA_TYPE_INFO);
    registry.register_type(LOCALIZEDSTRINGIDPICKERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CHECKEDLOCALIZEDSTRINGENTITYDATA_TYPE_INFO);
    registry.register_type(CHECKEDLOCALIZEDSTRINGENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(MATERIALRELATIONTRIGGARABLEEFFECTDATA_TYPE_INFO);
    registry.register_type(MATERIALRELATIONTRIGGARABLEEFFECTDATA_ARRAY_TYPE_INFO);
    registry.register_type(MATERIALBASEDEFFECTCOMPONENTDATA_TYPE_INFO);
    registry.register_type(MATERIALBASEDEFFECTCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(SNAPTYPE_TYPE_INFO);
    registry.register_type(SNAPTYPE_ARRAY_TYPE_INFO);
    registry.register_type(LOCATORCOMPONENTDATA_TYPE_INFO);
    registry.register_type(LOCATORCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(ACTORPHYSICSCOMPONENTDATA_TYPE_INFO);
    registry.register_type(ACTORPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(ACTORCUSTOMIZATIONCOMPONENTDATA_TYPE_INFO);
    registry.register_type(ACTORCUSTOMIZATIONCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(ACTORCUSTOMIZATIONDATA_TYPE_INFO);
    registry.register_type(ACTORCUSTOMIZATIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(DRAWVERLETCAPSULE_MAT4_FLOAT32_FLOAT32_VEC4__TYPE_INFO);
    registry.register_type(COOLDOWNGATEENTITYDATA_TYPE_INFO);
    registry.register_type(COOLDOWNGATEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CONDITIONALPROPERTYENTITYDATA_TYPE_INFO);
    registry.register_type(CONDITIONALPROPERTYENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTEMITTRACEBOOKMARKENTITYDATA_TYPE_INFO);
    registry.register_type(CLIENTEMITTRACEBOOKMARKENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CAMERASHAKEENTITYDATA_TYPE_INFO);
    registry.register_type(CAMERASHAKEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CAMERASHAKEAXISDATA_TYPE_INFO);
    registry.register_type(CAMERASHAKEAXISDATA_ARRAY_TYPE_INFO);
    registry.register_type(BLUEPRINTSPAWNREFERENCEOBJECTDATA_TYPE_INFO);
    registry.register_type(BLUEPRINTSPAWNREFERENCEOBJECTDATA_ARRAY_TYPE_INFO);
    registry.register_type(CHARACTERPROXYDATA_TYPE_INFO);
    registry.register_type(CHARACTERPROXYDATA_ARRAY_TYPE_INFO);
    registry.register_type(BLUEPRINTPROXYDATA_TYPE_INFO);
    registry.register_type(BLUEPRINTPROXYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PROXYPROPERTYCONTAINER_TYPE_INFO);
    registry.register_type(PROXYPROPERTYCONTAINER_ARRAY_TYPE_INFO);
    registry.register_type(BLUEPRINTPROXYPROPERTYFILTERDATA_TYPE_INFO);
    registry.register_type(BLUEPRINTPROXYPROPERTYFILTERDATA_ARRAY_TYPE_INFO);
    registry.register_type(ACTORENTITYDATA_TYPE_INFO);
    registry.register_type(ACTORENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(DICEUIANALOGPADTYPE_TYPE_INFO);
    registry.register_type(DICEUIANALOGPADTYPE_ARRAY_TYPE_INFO);
    registry.register_type(DICEUIINPUTMANAGERSETTINGS_TYPE_INFO);
    registry.register_type(DICEUIINPUTMANAGERSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(DICEUITYPINGINPUTLISTENERELEMENTDATA_TYPE_INFO);
    registry.register_type(DICEUITYPINGINPUTLISTENERELEMENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(DICEUIMOUSEINPUTLISTENERELEMENTDATA_TYPE_INFO);
    registry.register_type(DICEUIMOUSEINPUTLISTENERELEMENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(DICEUIINPUTACTIONLISTENERELEMENTDATA_TYPE_INFO);
    registry.register_type(DICEUIINPUTACTIONLISTENERELEMENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(DICEUIINPUTBLOCKERELEMENTDATA_TYPE_INFO);
    registry.register_type(DICEUIINPUTBLOCKERELEMENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(DICEUIANALOGSTICKINPUTLISTENERELEMENTDATA_TYPE_INFO);
    registry.register_type(DICEUIANALOGSTICKINPUTLISTENERELEMENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(DICEUIANALOGSTICK_TYPE_INFO);
    registry.register_type(DICEUIANALOGSTICK_ARRAY_TYPE_INFO);
    registry.register_type(DICEUIANALOGPADINPUTLISTENERELEMENTDATA_TYPE_INFO);
    registry.register_type(DICEUIANALOGPADINPUTLISTENERELEMENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(DICEUIINPUTMANAGERENTITYDATA_TYPE_INFO);
    registry.register_type(DICEUIINPUTMANAGERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(DICEDEBUGUIINPUTFLOWSIMULATIONDATA_TYPE_INFO);
    registry.register_type(DICEDEBUGUIINPUTFLOWSIMULATIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(DICEUIINPUTFLOWACTION_TYPE_INFO);
    registry.register_type(DICEUIINPUTFLOWACTION_ARRAY_TYPE_INFO);
    registry.register_type(CHARACTERDEFINITIONCOMPONENTDATA_TYPE_INFO);
    registry.register_type(CHARACTERDEFINITIONCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(CHARACTERDEFINITIONSPAWNDATA_TYPE_INFO);
    registry.register_type(CHARACTERDEFINITIONSPAWNDATA_ARRAY_TYPE_INFO);
    registry.register_type(CHARACTERDEFINITION_TYPE_INFO);
    registry.register_type(CHARACTERDEFINITION_ARRAY_TYPE_INFO);
    registry.register_type(CHARACTERDEFINITIONMESH_TYPE_INFO);
    registry.register_type(CHARACTERDEFINITIONMESH_ARRAY_TYPE_INFO);
    registry.register_type(DICEAUDIOSETTINGS_TYPE_INFO);
    registry.register_type(DICEAUDIOSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(DISTANCESCOPESTAGEDATA_TYPE_INFO);
    registry.register_type(DISTANCESCOPESTAGEDATA_ARRAY_TYPE_INFO);
    registry.register_type(COMBOSCOPESTAGEDATA_TYPE_INFO);
    registry.register_type(COMBOSCOPESTAGEDATA_ARRAY_TYPE_INFO);
    registry.register_type(ANGULARSCOPESTAGEDATA_TYPE_INFO);
    registry.register_type(ANGULARSCOPESTAGEDATA_ARRAY_TYPE_INFO);
    registry.register_type(WHOOSHBYPLAYERENTITYDATA_TYPE_INFO);
    registry.register_type(WHOOSHBYPLAYERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(AUDIOPROXIMITYREVERBENTITYDATA_TYPE_INFO);
    registry.register_type(AUDIOPROXIMITYREVERBENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(AUDIOPROXIMITYREVERBDATA_TYPE_INFO);
    registry.register_type(AUDIOPROXIMITYREVERBDATA_ARRAY_TYPE_INFO);
    registry.register_type(AUDIOPROXIMITYDETECTORREADERENTITYDATA_TYPE_INFO);
    registry.register_type(AUDIOPROXIMITYDETECTORREADERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(AUDIOPROXIMITYDETECTORENTITYDATA_TYPE_INFO);
    registry.register_type(AUDIOPROXIMITYDETECTORENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PROXIMITYDETECTORTYPE_TYPE_INFO);
    registry.register_type(PROXIMITYDETECTORTYPE_ARRAY_TYPE_INFO);
    registry.register_type(VOICEOVERINTERVALENTITYDATA_TYPE_INFO);
    registry.register_type(VOICEOVERINTERVALENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(VOICEOVERCONVERSATIONCHECKENTITYDATA_TYPE_INFO);
    registry.register_type(VOICEOVERCONVERSATIONCHECKENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(VOICEOVERCONTEXTAREARESULTENTITYDATA_TYPE_INFO);
    registry.register_type(VOICEOVERCONTEXTAREARESULTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(VOICEOVERCONTEXTAREAENTITYDATA_TYPE_INFO);
    registry.register_type(VOICEOVERCONTEXTAREAENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(VEHICLESOUNDENTITYDATA_TYPE_INFO);
    registry.register_type(VEHICLESOUNDENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SOUNDPROVIDERENTITYDATA_TYPE_INFO);
    registry.register_type(SOUNDPROVIDERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SOUNDASSETDATAENTITYDATA_TYPE_INFO);
    registry.register_type(SOUNDASSETDATAENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SOUNDACTIVITYTESTERENTITYDATA_TYPE_INFO);
    registry.register_type(SOUNDACTIVITYTESTERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(MUSICEVENTPRIORITYENTITYDATA_TYPE_INFO);
    registry.register_type(MUSICEVENTPRIORITYENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(DICESOUNDSPATIALENTITYDATA_TYPE_INFO);
    registry.register_type(DICESOUNDSPATIALENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(DICESOUNDENTITYDATA_TYPE_INFO);
    registry.register_type(DICESOUNDENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(DICESOUNDAREAENTITYDATA_TYPE_INFO);
    registry.register_type(DICESOUNDAREAENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(AUDIOCURVEFACTORENTITYDATA_TYPE_INFO);
    registry.register_type(AUDIOCURVEFACTORENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(DEBRISCLUSTERSOUNDSCOMPONENTDATA_TYPE_INFO);
    registry.register_type(DEBRISCLUSTERSOUNDSCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(DEBRISCLUSTERSOUND_TYPE_INFO);
    registry.register_type(DEBRISCLUSTERSOUND_ARRAY_TYPE_INFO);
    registry.register_type(DOFREADERENTITYDATA_TYPE_INFO);
    registry.register_type(DOFREADERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(ANIMATABLECULLINGENTITYDATA_TYPE_INFO);
    registry.register_type(ANIMATABLECULLINGENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENTITYCULLINGLEVEL_TYPE_INFO);
    registry.register_type(ENTITYCULLINGLEVEL_ARRAY_TYPE_INFO);
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct BlueprintProxyEntityBase {
    pub _glacier_base: super::entity::Entity,
}

pub trait BlueprintProxyEntityBaseTrait: super::entity::EntityTrait {
}

impl BlueprintProxyEntityBaseTrait for BlueprintProxyEntityBase {
}

impl super::entity::EntityTrait for BlueprintProxyEntityBase {
}

impl super::entity::EntityBusPeerTrait for BlueprintProxyEntityBase {
}

pub static BLUEPRINTPROXYENTITYBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintProxyEntityBase",
    name_hash: 3640473672,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(BlueprintProxyEntityBase, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BlueprintProxyEntityBase as Default>::default())),
            create_boxed: || Box::new(<BlueprintProxyEntityBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(BLUEPRINTPROXYENTITYBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BlueprintProxyEntityBase {
    fn type_info(&self) -> &'static TypeInfo {
        BLUEPRINTPROXYENTITYBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static BLUEPRINTPROXYENTITYBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintProxyEntityBase-Array",
    name_hash: 2390256380,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("BlueprintProxyEntityBase"),
    array_type: None,
    alignment: 8,
};



pub static DRAWTEXT2D_INT32_INT32_VEC4_VEC4__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DrawText2d(Int32,Int32,Vec4,Vec4)",
    name_hash: 580428867,
    flags: MemberInfoFlags::new(793),
    module: "DiceCommonsShared",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};



pub static DRAWTEXT2D_INT32_INT32_VEC4_VEC3__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DrawText2d(Int32,Int32,Vec4,Vec3)",
    name_hash: 580429028,
    flags: MemberInfoFlags::new(793),
    module: "DiceCommonsShared",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};



pub static DRAWTEXT2D_INT32_INT32_VEC4_INT32__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DrawText2d(Int32,Int32,Vec4,Int32)",
    name_hash: 1379711925,
    flags: MemberInfoFlags::new(793),
    module: "DiceCommonsShared",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};



pub static DRAWTEXT2D_INT32_INT32_VEC4_FLOAT32__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DrawText2d(Int32,Int32,Vec4,Float32)",
    name_hash: 3167238710,
    flags: MemberInfoFlags::new(793),
    module: "DiceCommonsShared",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};



pub static DRAWCOORDINATESYSTEM_MAT4__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DrawCoordinateSystem(Mat4)",
    name_hash: 583096175,
    flags: MemberInfoFlags::new(793),
    module: "DiceCommonsShared",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ValueMatchEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub debug_text_color: super::core::Vec3,
    pub use_external_time: bool,
    pub time_scale: f32,
    pub time_offset: f32,
    pub input_values_names: Vec<String>,
    pub match_and_trigger_array: Vec<BoxedTypeObject /* MatchAndTriggerItem */>,
}

pub trait ValueMatchEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn debug_text_color(&self) -> &super::core::Vec3;
    fn debug_text_color_mut(&mut self) -> &mut super::core::Vec3;
    fn use_external_time(&self) -> &bool;
    fn use_external_time_mut(&mut self) -> &mut bool;
    fn time_scale(&self) -> &f32;
    fn time_scale_mut(&mut self) -> &mut f32;
    fn time_offset(&self) -> &f32;
    fn time_offset_mut(&mut self) -> &mut f32;
    fn input_values_names(&self) -> &Vec<String>;
    fn input_values_names_mut(&mut self) -> &mut Vec<String>;
    fn match_and_trigger_array(&self) -> &Vec<BoxedTypeObject /* MatchAndTriggerItem */>;
    fn match_and_trigger_array_mut(&mut self) -> &mut Vec<BoxedTypeObject /* MatchAndTriggerItem */>;
}

impl ValueMatchEntityDataTrait for ValueMatchEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn debug_text_color(&self) -> &super::core::Vec3 {
        &self.debug_text_color
    }
    fn debug_text_color_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.debug_text_color
    }
    fn use_external_time(&self) -> &bool {
        &self.use_external_time
    }
    fn use_external_time_mut(&mut self) -> &mut bool {
        &mut self.use_external_time
    }
    fn time_scale(&self) -> &f32 {
        &self.time_scale
    }
    fn time_scale_mut(&mut self) -> &mut f32 {
        &mut self.time_scale
    }
    fn time_offset(&self) -> &f32 {
        &self.time_offset
    }
    fn time_offset_mut(&mut self) -> &mut f32 {
        &mut self.time_offset
    }
    fn input_values_names(&self) -> &Vec<String> {
        &self.input_values_names
    }
    fn input_values_names_mut(&mut self) -> &mut Vec<String> {
        &mut self.input_values_names
    }
    fn match_and_trigger_array(&self) -> &Vec<BoxedTypeObject /* MatchAndTriggerItem */> {
        &self.match_and_trigger_array
    }
    fn match_and_trigger_array_mut(&mut self) -> &mut Vec<BoxedTypeObject /* MatchAndTriggerItem */> {
        &mut self.match_and_trigger_array
    }
}

impl super::entity::EntityDataTrait for ValueMatchEntityData {
}

impl super::entity::GameObjectDataTrait for ValueMatchEntityData {
}

impl super::core::DataBusPeerTrait for ValueMatchEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ValueMatchEntityData {
}

impl super::core::DataContainerTrait for ValueMatchEntityData {
}

pub static VALUEMATCHENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ValueMatchEntityData",
    name_hash: 1835921238,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(ValueMatchEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ValueMatchEntityData as Default>::default())),
            create_boxed: || Box::new(<ValueMatchEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                name_hash: 229961746,
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(ValueMatchEntityData, realm),
            },
            FieldInfoData {
                name: "DebugTextColor",
                name_hash: 2811759668,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(ValueMatchEntityData, debug_text_color),
            },
            FieldInfoData {
                name: "UseExternalTime",
                name_hash: 2902061742,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ValueMatchEntityData, use_external_time),
            },
            FieldInfoData {
                name: "TimeScale",
                name_hash: 169511528,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ValueMatchEntityData, time_scale),
            },
            FieldInfoData {
                name: "TimeOffset",
                name_hash: 2388918461,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ValueMatchEntityData, time_offset),
            },
            FieldInfoData {
                name: "InputValuesNames",
                name_hash: 3085011391,
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(ValueMatchEntityData, input_values_names),
            },
            FieldInfoData {
                name: "MatchAndTriggerArray",
                name_hash: 654948636,
                flags: MemberInfoFlags::new(144),
                field_type: "MatchAndTriggerItem-Array",
                rust_offset: offset_of!(ValueMatchEntityData, match_and_trigger_array),
            },
        ],
    }),
    array_type: Some(VALUEMATCHENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ValueMatchEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        VALUEMATCHENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static VALUEMATCHENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ValueMatchEntityData-Array",
    name_hash: 3772904674,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ValueMatchEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MatchAndTriggerItem {
    pub input_values_to_match: Vec<BoxedTypeObject /* InputValueMatch */>,
    pub timed_event_triggers: Vec<BoxedTypeObject /* TimeEventTrigger */>,
    pub stop_at: f32,
}

pub trait MatchAndTriggerItemTrait: TypeObject {
    fn input_values_to_match(&self) -> &Vec<BoxedTypeObject /* InputValueMatch */>;
    fn input_values_to_match_mut(&mut self) -> &mut Vec<BoxedTypeObject /* InputValueMatch */>;
    fn timed_event_triggers(&self) -> &Vec<BoxedTypeObject /* TimeEventTrigger */>;
    fn timed_event_triggers_mut(&mut self) -> &mut Vec<BoxedTypeObject /* TimeEventTrigger */>;
    fn stop_at(&self) -> &f32;
    fn stop_at_mut(&mut self) -> &mut f32;
}

impl MatchAndTriggerItemTrait for MatchAndTriggerItem {
    fn input_values_to_match(&self) -> &Vec<BoxedTypeObject /* InputValueMatch */> {
        &self.input_values_to_match
    }
    fn input_values_to_match_mut(&mut self) -> &mut Vec<BoxedTypeObject /* InputValueMatch */> {
        &mut self.input_values_to_match
    }
    fn timed_event_triggers(&self) -> &Vec<BoxedTypeObject /* TimeEventTrigger */> {
        &self.timed_event_triggers
    }
    fn timed_event_triggers_mut(&mut self) -> &mut Vec<BoxedTypeObject /* TimeEventTrigger */> {
        &mut self.timed_event_triggers
    }
    fn stop_at(&self) -> &f32 {
        &self.stop_at
    }
    fn stop_at_mut(&mut self) -> &mut f32 {
        &mut self.stop_at
    }
}

pub static MATCHANDTRIGGERITEM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MatchAndTriggerItem",
    name_hash: 2623143408,
    flags: MemberInfoFlags::new(73),
    module: "DiceCommonsShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MatchAndTriggerItem as Default>::default())),
            create_boxed: || Box::new(<MatchAndTriggerItem as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "InputValuesToMatch",
                name_hash: 3365398947,
                flags: MemberInfoFlags::new(144),
                field_type: "InputValueMatch-Array",
                rust_offset: offset_of!(MatchAndTriggerItem, input_values_to_match),
            },
            FieldInfoData {
                name: "TimedEventTriggers",
                name_hash: 3088010771,
                flags: MemberInfoFlags::new(144),
                field_type: "TimeEventTrigger-Array",
                rust_offset: offset_of!(MatchAndTriggerItem, timed_event_triggers),
            },
            FieldInfoData {
                name: "StopAt",
                name_hash: 3320223400,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MatchAndTriggerItem, stop_at),
            },
        ],
    }),
    array_type: Some(MATCHANDTRIGGERITEM_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MatchAndTriggerItem {
    fn type_info(&self) -> &'static TypeInfo {
        MATCHANDTRIGGERITEM_TYPE_INFO
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


pub static MATCHANDTRIGGERITEM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MatchAndTriggerItem-Array",
    name_hash: 48110532,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("MatchAndTriggerItem"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TimeEventTrigger {
    pub output_event_hash: i32,
    pub time_to_trigger_event: f32,
}

pub trait TimeEventTriggerTrait: TypeObject {
    fn output_event_hash(&self) -> &i32;
    fn output_event_hash_mut(&mut self) -> &mut i32;
    fn time_to_trigger_event(&self) -> &f32;
    fn time_to_trigger_event_mut(&mut self) -> &mut f32;
}

impl TimeEventTriggerTrait for TimeEventTrigger {
    fn output_event_hash(&self) -> &i32 {
        &self.output_event_hash
    }
    fn output_event_hash_mut(&mut self) -> &mut i32 {
        &mut self.output_event_hash
    }
    fn time_to_trigger_event(&self) -> &f32 {
        &self.time_to_trigger_event
    }
    fn time_to_trigger_event_mut(&mut self) -> &mut f32 {
        &mut self.time_to_trigger_event
    }
}

pub static TIMEEVENTTRIGGER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimeEventTrigger",
    name_hash: 2615459204,
    flags: MemberInfoFlags::new(32841),
    module: "DiceCommonsShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TimeEventTrigger as Default>::default())),
            create_boxed: || Box::new(<TimeEventTrigger as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "OutputEventHash",
                name_hash: 1931060516,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TimeEventTrigger, output_event_hash),
            },
            FieldInfoData {
                name: "TimeToTriggerEvent",
                name_hash: 3227863039,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TimeEventTrigger, time_to_trigger_event),
            },
        ],
    }),
    array_type: Some(TIMEEVENTTRIGGER_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for TimeEventTrigger {
    fn type_info(&self) -> &'static TypeInfo {
        TIMEEVENTTRIGGER_TYPE_INFO
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


pub static TIMEEVENTTRIGGER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimeEventTrigger-Array",
    name_hash: 3099275696,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("TimeEventTrigger"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct InputValueMatch {
    pub input_values_names_index: i32,
    pub value_to_match: i32,
}

pub trait InputValueMatchTrait: TypeObject {
    fn input_values_names_index(&self) -> &i32;
    fn input_values_names_index_mut(&mut self) -> &mut i32;
    fn value_to_match(&self) -> &i32;
    fn value_to_match_mut(&mut self) -> &mut i32;
}

impl InputValueMatchTrait for InputValueMatch {
    fn input_values_names_index(&self) -> &i32 {
        &self.input_values_names_index
    }
    fn input_values_names_index_mut(&mut self) -> &mut i32 {
        &mut self.input_values_names_index
    }
    fn value_to_match(&self) -> &i32 {
        &self.value_to_match
    }
    fn value_to_match_mut(&mut self) -> &mut i32 {
        &mut self.value_to_match
    }
}

pub static INPUTVALUEMATCH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputValueMatch",
    name_hash: 2226808363,
    flags: MemberInfoFlags::new(32841),
    module: "DiceCommonsShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InputValueMatch as Default>::default())),
            create_boxed: || Box::new(<InputValueMatch as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "InputValuesNamesIndex",
                name_hash: 551550689,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(InputValueMatch, input_values_names_index),
            },
            FieldInfoData {
                name: "ValueToMatch",
                name_hash: 4228831462,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(InputValueMatch, value_to_match),
            },
        ],
    }),
    array_type: Some(INPUTVALUEMATCH_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for InputValueMatch {
    fn type_info(&self) -> &'static TypeInfo {
        INPUTVALUEMATCH_TYPE_INFO
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


pub static INPUTVALUEMATCH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputValueMatch-Array",
    name_hash: 2235303327,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("InputValueMatch"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SimpleRotationEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub auto_start: bool,
    pub verify_entity_and_component_links: bool,
    pub transform_modifiers: Vec<Option<LockedTypeObject /* TransformModifierData */>>,
    pub rotation_speed_multiplier: f32,
}

pub trait SimpleRotationEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn auto_start(&self) -> &bool;
    fn auto_start_mut(&mut self) -> &mut bool;
    fn verify_entity_and_component_links(&self) -> &bool;
    fn verify_entity_and_component_links_mut(&mut self) -> &mut bool;
    fn transform_modifiers(&self) -> &Vec<Option<LockedTypeObject /* TransformModifierData */>>;
    fn transform_modifiers_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* TransformModifierData */>>;
    fn rotation_speed_multiplier(&self) -> &f32;
    fn rotation_speed_multiplier_mut(&mut self) -> &mut f32;
}

impl SimpleRotationEntityDataTrait for SimpleRotationEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn auto_start(&self) -> &bool {
        &self.auto_start
    }
    fn auto_start_mut(&mut self) -> &mut bool {
        &mut self.auto_start
    }
    fn verify_entity_and_component_links(&self) -> &bool {
        &self.verify_entity_and_component_links
    }
    fn verify_entity_and_component_links_mut(&mut self) -> &mut bool {
        &mut self.verify_entity_and_component_links
    }
    fn transform_modifiers(&self) -> &Vec<Option<LockedTypeObject /* TransformModifierData */>> {
        &self.transform_modifiers
    }
    fn transform_modifiers_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* TransformModifierData */>> {
        &mut self.transform_modifiers
    }
    fn rotation_speed_multiplier(&self) -> &f32 {
        &self.rotation_speed_multiplier
    }
    fn rotation_speed_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.rotation_speed_multiplier
    }
}

impl super::entity::EntityDataTrait for SimpleRotationEntityData {
}

impl super::entity::GameObjectDataTrait for SimpleRotationEntityData {
}

impl super::core::DataBusPeerTrait for SimpleRotationEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for SimpleRotationEntityData {
}

impl super::core::DataContainerTrait for SimpleRotationEntityData {
}

pub static SIMPLEROTATIONENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimpleRotationEntityData",
    name_hash: 3550933140,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(SimpleRotationEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SimpleRotationEntityData as Default>::default())),
            create_boxed: || Box::new(<SimpleRotationEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                name_hash: 229961746,
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(SimpleRotationEntityData, realm),
            },
            FieldInfoData {
                name: "AutoStart",
                name_hash: 792615882,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SimpleRotationEntityData, auto_start),
            },
            FieldInfoData {
                name: "VerifyEntityAndComponentLinks",
                name_hash: 1965497630,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SimpleRotationEntityData, verify_entity_and_component_links),
            },
            FieldInfoData {
                name: "TransformModifiers",
                name_hash: 1961377965,
                flags: MemberInfoFlags::new(144),
                field_type: "TransformModifierData-Array",
                rust_offset: offset_of!(SimpleRotationEntityData, transform_modifiers),
            },
            FieldInfoData {
                name: "RotationSpeedMultiplier",
                name_hash: 2729011741,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SimpleRotationEntityData, rotation_speed_multiplier),
            },
        ],
    }),
    array_type: Some(SIMPLEROTATIONENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SimpleRotationEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        SIMPLEROTATIONENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SIMPLEROTATIONENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimpleRotationEntityData-Array",
    name_hash: 3819596960,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("SimpleRotationEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PingPongRotationTransformModifierData {
    pub _glacier_base: TransformModifierData,
    pub min_angle_in_radians: f32,
    pub max_angle_in_radians: f32,
    pub min_angle_in_degrees: f32,
    pub max_angle_in_degrees: f32,
    pub frequency: f32,
}

pub trait PingPongRotationTransformModifierDataTrait: TransformModifierDataTrait {
    fn min_angle_in_radians(&self) -> &f32;
    fn min_angle_in_radians_mut(&mut self) -> &mut f32;
    fn max_angle_in_radians(&self) -> &f32;
    fn max_angle_in_radians_mut(&mut self) -> &mut f32;
    fn min_angle_in_degrees(&self) -> &f32;
    fn min_angle_in_degrees_mut(&mut self) -> &mut f32;
    fn max_angle_in_degrees(&self) -> &f32;
    fn max_angle_in_degrees_mut(&mut self) -> &mut f32;
    fn frequency(&self) -> &f32;
    fn frequency_mut(&mut self) -> &mut f32;
}

impl PingPongRotationTransformModifierDataTrait for PingPongRotationTransformModifierData {
    fn min_angle_in_radians(&self) -> &f32 {
        &self.min_angle_in_radians
    }
    fn min_angle_in_radians_mut(&mut self) -> &mut f32 {
        &mut self.min_angle_in_radians
    }
    fn max_angle_in_radians(&self) -> &f32 {
        &self.max_angle_in_radians
    }
    fn max_angle_in_radians_mut(&mut self) -> &mut f32 {
        &mut self.max_angle_in_radians
    }
    fn min_angle_in_degrees(&self) -> &f32 {
        &self.min_angle_in_degrees
    }
    fn min_angle_in_degrees_mut(&mut self) -> &mut f32 {
        &mut self.min_angle_in_degrees
    }
    fn max_angle_in_degrees(&self) -> &f32 {
        &self.max_angle_in_degrees
    }
    fn max_angle_in_degrees_mut(&mut self) -> &mut f32 {
        &mut self.max_angle_in_degrees
    }
    fn frequency(&self) -> &f32 {
        &self.frequency
    }
    fn frequency_mut(&mut self) -> &mut f32 {
        &mut self.frequency
    }
}

impl TransformModifierDataTrait for PingPongRotationTransformModifierData {
    fn random_timing_to_apply(&self) -> &f32 {
        self._glacier_base.random_timing_to_apply()
    }
    fn random_timing_to_apply_mut(&mut self) -> &mut f32 {
        self._glacier_base.random_timing_to_apply_mut()
    }
    fn rotation_axis(&self) -> &super::gameplay_sim::RotationAxis {
        self._glacier_base.rotation_axis()
    }
    fn rotation_axis_mut(&mut self) -> &mut super::gameplay_sim::RotationAxis {
        self._glacier_base.rotation_axis_mut()
    }
    fn rotation_axis_vec(&self) -> &super::core::Vec3 {
        self._glacier_base.rotation_axis_vec()
    }
    fn rotation_axis_vec_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.rotation_axis_vec_mut()
    }
}

impl super::core::DataContainerTrait for PingPongRotationTransformModifierData {
}

pub static PINGPONGROTATIONTRANSFORMMODIFIERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PingPongRotationTransformModifierData",
    name_hash: 538483388,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TRANSFORMMODIFIERDATA_TYPE_INFO),
        super_class_offset: offset_of!(PingPongRotationTransformModifierData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PingPongRotationTransformModifierData as Default>::default())),
            create_boxed: || Box::new(<PingPongRotationTransformModifierData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "MinAngleInRadians",
                name_hash: 1924757771,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PingPongRotationTransformModifierData, min_angle_in_radians),
            },
            FieldInfoData {
                name: "MaxAngleInRadians",
                name_hash: 1855270293,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PingPongRotationTransformModifierData, max_angle_in_radians),
            },
            FieldInfoData {
                name: "MinAngleInDegrees",
                name_hash: 155107214,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PingPongRotationTransformModifierData, min_angle_in_degrees),
            },
            FieldInfoData {
                name: "MaxAngleInDegrees",
                name_hash: 3631677200,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PingPongRotationTransformModifierData, max_angle_in_degrees),
            },
            FieldInfoData {
                name: "Frequency",
                name_hash: 4112821953,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PingPongRotationTransformModifierData, frequency),
            },
        ],
    }),
    array_type: Some(PINGPONGROTATIONTRANSFORMMODIFIERDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PingPongRotationTransformModifierData {
    fn type_info(&self) -> &'static TypeInfo {
        PINGPONGROTATIONTRANSFORMMODIFIERDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static PINGPONGROTATIONTRANSFORMMODIFIERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PingPongRotationTransformModifierData-Array",
    name_hash: 2186433032,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("PingPongRotationTransformModifierData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct RotationTransformModifierData {
    pub _glacier_base: TransformModifierData,
    pub radians_per_second: f32,
    pub degrees_per_second: f32,
}

pub trait RotationTransformModifierDataTrait: TransformModifierDataTrait {
    fn radians_per_second(&self) -> &f32;
    fn radians_per_second_mut(&mut self) -> &mut f32;
    fn degrees_per_second(&self) -> &f32;
    fn degrees_per_second_mut(&mut self) -> &mut f32;
}

impl RotationTransformModifierDataTrait for RotationTransformModifierData {
    fn radians_per_second(&self) -> &f32 {
        &self.radians_per_second
    }
    fn radians_per_second_mut(&mut self) -> &mut f32 {
        &mut self.radians_per_second
    }
    fn degrees_per_second(&self) -> &f32 {
        &self.degrees_per_second
    }
    fn degrees_per_second_mut(&mut self) -> &mut f32 {
        &mut self.degrees_per_second
    }
}

impl TransformModifierDataTrait for RotationTransformModifierData {
    fn random_timing_to_apply(&self) -> &f32 {
        self._glacier_base.random_timing_to_apply()
    }
    fn random_timing_to_apply_mut(&mut self) -> &mut f32 {
        self._glacier_base.random_timing_to_apply_mut()
    }
    fn rotation_axis(&self) -> &super::gameplay_sim::RotationAxis {
        self._glacier_base.rotation_axis()
    }
    fn rotation_axis_mut(&mut self) -> &mut super::gameplay_sim::RotationAxis {
        self._glacier_base.rotation_axis_mut()
    }
    fn rotation_axis_vec(&self) -> &super::core::Vec3 {
        self._glacier_base.rotation_axis_vec()
    }
    fn rotation_axis_vec_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.rotation_axis_vec_mut()
    }
}

impl super::core::DataContainerTrait for RotationTransformModifierData {
}

pub static ROTATIONTRANSFORMMODIFIERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RotationTransformModifierData",
    name_hash: 2145976058,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TRANSFORMMODIFIERDATA_TYPE_INFO),
        super_class_offset: offset_of!(RotationTransformModifierData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RotationTransformModifierData as Default>::default())),
            create_boxed: || Box::new(<RotationTransformModifierData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "RadiansPerSecond",
                name_hash: 3087318736,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RotationTransformModifierData, radians_per_second),
            },
            FieldInfoData {
                name: "DegreesPerSecond",
                name_hash: 1000646773,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RotationTransformModifierData, degrees_per_second),
            },
        ],
    }),
    array_type: Some(ROTATIONTRANSFORMMODIFIERDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RotationTransformModifierData {
    fn type_info(&self) -> &'static TypeInfo {
        ROTATIONTRANSFORMMODIFIERDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static ROTATIONTRANSFORMMODIFIERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RotationTransformModifierData-Array",
    name_hash: 3960828366,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("RotationTransformModifierData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TransformModifierData {
    pub _glacier_base: super::core::DataContainer,
    pub random_timing_to_apply: f32,
    pub rotation_axis: super::gameplay_sim::RotationAxis,
    pub rotation_axis_vec: super::core::Vec3,
}

pub trait TransformModifierDataTrait: super::core::DataContainerTrait {
    fn random_timing_to_apply(&self) -> &f32;
    fn random_timing_to_apply_mut(&mut self) -> &mut f32;
    fn rotation_axis(&self) -> &super::gameplay_sim::RotationAxis;
    fn rotation_axis_mut(&mut self) -> &mut super::gameplay_sim::RotationAxis;
    fn rotation_axis_vec(&self) -> &super::core::Vec3;
    fn rotation_axis_vec_mut(&mut self) -> &mut super::core::Vec3;
}

impl TransformModifierDataTrait for TransformModifierData {
    fn random_timing_to_apply(&self) -> &f32 {
        &self.random_timing_to_apply
    }
    fn random_timing_to_apply_mut(&mut self) -> &mut f32 {
        &mut self.random_timing_to_apply
    }
    fn rotation_axis(&self) -> &super::gameplay_sim::RotationAxis {
        &self.rotation_axis
    }
    fn rotation_axis_mut(&mut self) -> &mut super::gameplay_sim::RotationAxis {
        &mut self.rotation_axis
    }
    fn rotation_axis_vec(&self) -> &super::core::Vec3 {
        &self.rotation_axis_vec
    }
    fn rotation_axis_vec_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.rotation_axis_vec
    }
}

impl super::core::DataContainerTrait for TransformModifierData {
}

pub static TRANSFORMMODIFIERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformModifierData",
    name_hash: 1330919726,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(TransformModifierData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TransformModifierData as Default>::default())),
            create_boxed: || Box::new(<TransformModifierData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "RandomTimingToApply",
                name_hash: 2128868353,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TransformModifierData, random_timing_to_apply),
            },
            FieldInfoData {
                name: "RotationAxis",
                name_hash: 3148542130,
                flags: MemberInfoFlags::new(0),
                field_type: "RotationAxis",
                rust_offset: offset_of!(TransformModifierData, rotation_axis),
            },
            FieldInfoData {
                name: "RotationAxisVec",
                name_hash: 2539995906,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TransformModifierData, rotation_axis_vec),
            },
        ],
    }),
    array_type: Some(TRANSFORMMODIFIERDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TransformModifierData {
    fn type_info(&self) -> &'static TypeInfo {
        TRANSFORMMODIFIERDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static TRANSFORMMODIFIERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformModifierData-Array",
    name_hash: 2869985690,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("TransformModifierData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum TransformModifierType {
    #[default]
    TransformModifierType_Invalid = 0,
    TransformModifierType_Rotation = 1,
    TransformModifierType_PingPongRotation = 2,
}

pub static TRANSFORMMODIFIERTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformModifierType",
    name_hash: 1330353318,
    flags: MemberInfoFlags::new(49429),
    module: "DiceCommonsShared",
    data: TypeInfoData::Enum,
    array_type: Some(TRANSFORMMODIFIERTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TransformModifierType {
    fn type_info(&self) -> &'static TypeInfo {
        TRANSFORMMODIFIERTYPE_TYPE_INFO
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


pub static TRANSFORMMODIFIERTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformModifierType-Array",
    name_hash: 3061784850,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("TransformModifierType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ShadowplayHighlightEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub award_name_string_id: Option<LockedTypeObject /* super::u_i_incubator_shared::LocalizedStringId */>,
    pub start_delta_time: i32,
    pub end_delta_time: i32,
}

pub trait ShadowplayHighlightEntityDataTrait: super::entity::EntityDataTrait {
    fn award_name_string_id(&self) -> &Option<LockedTypeObject /* super::u_i_incubator_shared::LocalizedStringId */>;
    fn award_name_string_id_mut(&mut self) -> &mut Option<LockedTypeObject /* super::u_i_incubator_shared::LocalizedStringId */>;
    fn start_delta_time(&self) -> &i32;
    fn start_delta_time_mut(&mut self) -> &mut i32;
    fn end_delta_time(&self) -> &i32;
    fn end_delta_time_mut(&mut self) -> &mut i32;
}

impl ShadowplayHighlightEntityDataTrait for ShadowplayHighlightEntityData {
    fn award_name_string_id(&self) -> &Option<LockedTypeObject /* super::u_i_incubator_shared::LocalizedStringId */> {
        &self.award_name_string_id
    }
    fn award_name_string_id_mut(&mut self) -> &mut Option<LockedTypeObject /* super::u_i_incubator_shared::LocalizedStringId */> {
        &mut self.award_name_string_id
    }
    fn start_delta_time(&self) -> &i32 {
        &self.start_delta_time
    }
    fn start_delta_time_mut(&mut self) -> &mut i32 {
        &mut self.start_delta_time
    }
    fn end_delta_time(&self) -> &i32 {
        &self.end_delta_time
    }
    fn end_delta_time_mut(&mut self) -> &mut i32 {
        &mut self.end_delta_time
    }
}

impl super::entity::EntityDataTrait for ShadowplayHighlightEntityData {
}

impl super::entity::GameObjectDataTrait for ShadowplayHighlightEntityData {
}

impl super::core::DataBusPeerTrait for ShadowplayHighlightEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ShadowplayHighlightEntityData {
}

impl super::core::DataContainerTrait for ShadowplayHighlightEntityData {
}

pub static SHADOWPLAYHIGHLIGHTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowplayHighlightEntityData",
    name_hash: 3712452540,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(ShadowplayHighlightEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ShadowplayHighlightEntityData as Default>::default())),
            create_boxed: || Box::new(<ShadowplayHighlightEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "AwardNameStringId",
                name_hash: 2120058907,
                flags: MemberInfoFlags::new(0),
                field_type: "LocalizedStringId",
                rust_offset: offset_of!(ShadowplayHighlightEntityData, award_name_string_id),
            },
            FieldInfoData {
                name: "StartDeltaTime",
                name_hash: 1703830312,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ShadowplayHighlightEntityData, start_delta_time),
            },
            FieldInfoData {
                name: "EndDeltaTime",
                name_hash: 3493111367,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ShadowplayHighlightEntityData, end_delta_time),
            },
        ],
    }),
    array_type: Some(SHADOWPLAYHIGHLIGHTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ShadowplayHighlightEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        SHADOWPLAYHIGHLIGHTENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SHADOWPLAYHIGHLIGHTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowplayHighlightEntityData-Array",
    name_hash: 2487156488,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ShadowplayHighlightEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ShadowplaySettings {
    pub _glacier_base: super::core::SystemSettings,
    pub enable: bool,
    pub display_summary: bool,
}

pub trait ShadowplaySettingsTrait: super::core::SystemSettingsTrait {
    fn enable(&self) -> &bool;
    fn enable_mut(&mut self) -> &mut bool;
    fn display_summary(&self) -> &bool;
    fn display_summary_mut(&mut self) -> &mut bool;
}

impl ShadowplaySettingsTrait for ShadowplaySettings {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn enable_mut(&mut self) -> &mut bool {
        &mut self.enable
    }
    fn display_summary(&self) -> &bool {
        &self.display_summary
    }
    fn display_summary_mut(&mut self) -> &mut bool {
        &mut self.display_summary
    }
}

impl super::core::SystemSettingsTrait for ShadowplaySettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl super::core::DataContainerTrait for ShadowplaySettings {
}

pub static SHADOWPLAYSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowplaySettings",
    name_hash: 1685254242,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        super_class_offset: offset_of!(ShadowplaySettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ShadowplaySettings as Default>::default())),
            create_boxed: || Box::new(<ShadowplaySettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                name_hash: 2342790116,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ShadowplaySettings, enable),
            },
            FieldInfoData {
                name: "DisplaySummary",
                name_hash: 4177096403,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ShadowplaySettings, display_summary),
            },
        ],
    }),
    array_type: Some(SHADOWPLAYSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ShadowplaySettings {
    fn type_info(&self) -> &'static TypeInfo {
        SHADOWPLAYSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SHADOWPLAYSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowplaySettings-Array",
    name_hash: 1369202518,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ShadowplaySettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SelectableActionEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub enabled: bool,
    pub cooldown_time: f64,
    pub allowed_select_count: u32,
    pub priority: u32,
}

pub trait SelectableActionEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn enabled(&self) -> &bool;
    fn enabled_mut(&mut self) -> &mut bool;
    fn cooldown_time(&self) -> &f64;
    fn cooldown_time_mut(&mut self) -> &mut f64;
    fn allowed_select_count(&self) -> &u32;
    fn allowed_select_count_mut(&mut self) -> &mut u32;
    fn priority(&self) -> &u32;
    fn priority_mut(&mut self) -> &mut u32;
}

impl SelectableActionEntityDataTrait for SelectableActionEntityData {
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
    fn cooldown_time(&self) -> &f64 {
        &self.cooldown_time
    }
    fn cooldown_time_mut(&mut self) -> &mut f64 {
        &mut self.cooldown_time
    }
    fn allowed_select_count(&self) -> &u32 {
        &self.allowed_select_count
    }
    fn allowed_select_count_mut(&mut self) -> &mut u32 {
        &mut self.allowed_select_count
    }
    fn priority(&self) -> &u32 {
        &self.priority
    }
    fn priority_mut(&mut self) -> &mut u32 {
        &mut self.priority
    }
}

impl super::entity::EntityDataTrait for SelectableActionEntityData {
}

impl super::entity::GameObjectDataTrait for SelectableActionEntityData {
}

impl super::core::DataBusPeerTrait for SelectableActionEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for SelectableActionEntityData {
}

impl super::core::DataContainerTrait for SelectableActionEntityData {
}

pub static SELECTABLEACTIONENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectableActionEntityData",
    name_hash: 1921686866,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(SelectableActionEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SelectableActionEntityData as Default>::default())),
            create_boxed: || Box::new(<SelectableActionEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                name_hash: 229961746,
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(SelectableActionEntityData, realm),
            },
            FieldInfoData {
                name: "Enabled",
                name_hash: 2662400,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SelectableActionEntityData, enabled),
            },
            FieldInfoData {
                name: "CooldownTime",
                name_hash: 613462861,
                flags: MemberInfoFlags::new(0),
                field_type: "Float64",
                rust_offset: offset_of!(SelectableActionEntityData, cooldown_time),
            },
            FieldInfoData {
                name: "AllowedSelectCount",
                name_hash: 3103866710,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(SelectableActionEntityData, allowed_select_count),
            },
            FieldInfoData {
                name: "Priority",
                name_hash: 3062102871,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(SelectableActionEntityData, priority),
            },
        ],
    }),
    array_type: Some(SELECTABLEACTIONENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SelectableActionEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        SELECTABLEACTIONENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SELECTABLEACTIONENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectableActionEntityData-Array",
    name_hash: 1057584358,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("SelectableActionEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct RumbleEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub enabled: bool,
    pub duration: f32,
    pub low: f32,
    pub high: f32,
}

pub trait RumbleEntityDataTrait: super::entity::EntityDataTrait {
    fn enabled(&self) -> &bool;
    fn enabled_mut(&mut self) -> &mut bool;
    fn duration(&self) -> &f32;
    fn duration_mut(&mut self) -> &mut f32;
    fn low(&self) -> &f32;
    fn low_mut(&mut self) -> &mut f32;
    fn high(&self) -> &f32;
    fn high_mut(&mut self) -> &mut f32;
}

impl RumbleEntityDataTrait for RumbleEntityData {
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
    fn duration(&self) -> &f32 {
        &self.duration
    }
    fn duration_mut(&mut self) -> &mut f32 {
        &mut self.duration
    }
    fn low(&self) -> &f32 {
        &self.low
    }
    fn low_mut(&mut self) -> &mut f32 {
        &mut self.low
    }
    fn high(&self) -> &f32 {
        &self.high
    }
    fn high_mut(&mut self) -> &mut f32 {
        &mut self.high
    }
}

impl super::entity::EntityDataTrait for RumbleEntityData {
}

impl super::entity::GameObjectDataTrait for RumbleEntityData {
}

impl super::core::DataBusPeerTrait for RumbleEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for RumbleEntityData {
}

impl super::core::DataContainerTrait for RumbleEntityData {
}

pub static RUMBLEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RumbleEntityData",
    name_hash: 3284741103,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(RumbleEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RumbleEntityData as Default>::default())),
            create_boxed: || Box::new(<RumbleEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Enabled",
                name_hash: 2662400,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RumbleEntityData, enabled),
            },
            FieldInfoData {
                name: "Duration",
                name_hash: 1828507227,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RumbleEntityData, duration),
            },
            FieldInfoData {
                name: "Low",
                name_hash: 193454161,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RumbleEntityData, low),
            },
            FieldInfoData {
                name: "High",
                name_hash: 2089152523,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RumbleEntityData, high),
            },
        ],
    }),
    array_type: Some(RUMBLEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RumbleEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        RUMBLEENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static RUMBLEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RumbleEntityData-Array",
    name_hash: 3443247579,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("RumbleEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct RaycastDirectionEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub attach: Option<LockedTypeObject /* EntityAttachData */>,
    pub continuous_update: bool,
    pub transform: super::core::LinearTransform,
    pub ray_horizontal_angle: f32,
    pub ray_vertical_angle: f32,
    pub ray_distance: f32,
    pub lock_horizontal_rotation: bool,
    pub lock_vertical_rotation: bool,
}

pub trait RaycastDirectionEntityDataTrait: super::entity::EntityDataTrait {
    fn attach(&self) -> &Option<LockedTypeObject /* EntityAttachData */>;
    fn attach_mut(&mut self) -> &mut Option<LockedTypeObject /* EntityAttachData */>;
    fn continuous_update(&self) -> &bool;
    fn continuous_update_mut(&mut self) -> &mut bool;
    fn transform(&self) -> &super::core::LinearTransform;
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform;
    fn ray_horizontal_angle(&self) -> &f32;
    fn ray_horizontal_angle_mut(&mut self) -> &mut f32;
    fn ray_vertical_angle(&self) -> &f32;
    fn ray_vertical_angle_mut(&mut self) -> &mut f32;
    fn ray_distance(&self) -> &f32;
    fn ray_distance_mut(&mut self) -> &mut f32;
    fn lock_horizontal_rotation(&self) -> &bool;
    fn lock_horizontal_rotation_mut(&mut self) -> &mut bool;
    fn lock_vertical_rotation(&self) -> &bool;
    fn lock_vertical_rotation_mut(&mut self) -> &mut bool;
}

impl RaycastDirectionEntityDataTrait for RaycastDirectionEntityData {
    fn attach(&self) -> &Option<LockedTypeObject /* EntityAttachData */> {
        &self.attach
    }
    fn attach_mut(&mut self) -> &mut Option<LockedTypeObject /* EntityAttachData */> {
        &mut self.attach
    }
    fn continuous_update(&self) -> &bool {
        &self.continuous_update
    }
    fn continuous_update_mut(&mut self) -> &mut bool {
        &mut self.continuous_update
    }
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.transform
    }
    fn ray_horizontal_angle(&self) -> &f32 {
        &self.ray_horizontal_angle
    }
    fn ray_horizontal_angle_mut(&mut self) -> &mut f32 {
        &mut self.ray_horizontal_angle
    }
    fn ray_vertical_angle(&self) -> &f32 {
        &self.ray_vertical_angle
    }
    fn ray_vertical_angle_mut(&mut self) -> &mut f32 {
        &mut self.ray_vertical_angle
    }
    fn ray_distance(&self) -> &f32 {
        &self.ray_distance
    }
    fn ray_distance_mut(&mut self) -> &mut f32 {
        &mut self.ray_distance
    }
    fn lock_horizontal_rotation(&self) -> &bool {
        &self.lock_horizontal_rotation
    }
    fn lock_horizontal_rotation_mut(&mut self) -> &mut bool {
        &mut self.lock_horizontal_rotation
    }
    fn lock_vertical_rotation(&self) -> &bool {
        &self.lock_vertical_rotation
    }
    fn lock_vertical_rotation_mut(&mut self) -> &mut bool {
        &mut self.lock_vertical_rotation
    }
}

impl super::entity::EntityDataTrait for RaycastDirectionEntityData {
}

impl super::entity::GameObjectDataTrait for RaycastDirectionEntityData {
}

impl super::core::DataBusPeerTrait for RaycastDirectionEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for RaycastDirectionEntityData {
}

impl super::core::DataContainerTrait for RaycastDirectionEntityData {
}

pub static RAYCASTDIRECTIONENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RaycastDirectionEntityData",
    name_hash: 559823876,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(RaycastDirectionEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RaycastDirectionEntityData as Default>::default())),
            create_boxed: || Box::new(<RaycastDirectionEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Attach",
                name_hash: 2500885102,
                flags: MemberInfoFlags::new(0),
                field_type: "EntityAttachData",
                rust_offset: offset_of!(RaycastDirectionEntityData, attach),
            },
            FieldInfoData {
                name: "ContinuousUpdate",
                name_hash: 1186338873,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RaycastDirectionEntityData, continuous_update),
            },
            FieldInfoData {
                name: "Transform",
                name_hash: 2270319721,
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(RaycastDirectionEntityData, transform),
            },
            FieldInfoData {
                name: "RayHorizontalAngle",
                name_hash: 1283751856,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RaycastDirectionEntityData, ray_horizontal_angle),
            },
            FieldInfoData {
                name: "RayVerticalAngle",
                name_hash: 761316956,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RaycastDirectionEntityData, ray_vertical_angle),
            },
            FieldInfoData {
                name: "RayDistance",
                name_hash: 223111116,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RaycastDirectionEntityData, ray_distance),
            },
            FieldInfoData {
                name: "LockHorizontalRotation",
                name_hash: 3309499332,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RaycastDirectionEntityData, lock_horizontal_rotation),
            },
            FieldInfoData {
                name: "LockVerticalRotation",
                name_hash: 3233359208,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RaycastDirectionEntityData, lock_vertical_rotation),
            },
        ],
    }),
    array_type: Some(RAYCASTDIRECTIONENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RaycastDirectionEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        RAYCASTDIRECTIONENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static RAYCASTDIRECTIONENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RaycastDirectionEntityData-Array",
    name_hash: 1543082032,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("RaycastDirectionEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct RandomActionSelectorEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub select_actions_deterministically: bool,
}

pub trait RandomActionSelectorEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn select_actions_deterministically(&self) -> &bool;
    fn select_actions_deterministically_mut(&mut self) -> &mut bool;
}

impl RandomActionSelectorEntityDataTrait for RandomActionSelectorEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn select_actions_deterministically(&self) -> &bool {
        &self.select_actions_deterministically
    }
    fn select_actions_deterministically_mut(&mut self) -> &mut bool {
        &mut self.select_actions_deterministically
    }
}

impl super::entity::EntityDataTrait for RandomActionSelectorEntityData {
}

impl super::entity::GameObjectDataTrait for RandomActionSelectorEntityData {
}

impl super::core::DataBusPeerTrait for RandomActionSelectorEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for RandomActionSelectorEntityData {
}

impl super::core::DataContainerTrait for RandomActionSelectorEntityData {
}

pub static RANDOMACTIONSELECTORENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomActionSelectorEntityData",
    name_hash: 1008899998,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(RandomActionSelectorEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RandomActionSelectorEntityData as Default>::default())),
            create_boxed: || Box::new(<RandomActionSelectorEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                name_hash: 229961746,
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(RandomActionSelectorEntityData, realm),
            },
            FieldInfoData {
                name: "SelectActionsDeterministically",
                name_hash: 2309709396,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RandomActionSelectorEntityData, select_actions_deterministically),
            },
        ],
    }),
    array_type: Some(RANDOMACTIONSELECTORENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RandomActionSelectorEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        RANDOMACTIONSELECTORENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static RANDOMACTIONSELECTORENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomActionSelectorEntityData-Array",
    name_hash: 2519918762,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("RandomActionSelectorEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PropertyStatusEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub type_hash: i32,
    pub in_hash: i32,
    pub out_hash: i32,
    pub always_send_events: bool,
}

pub trait PropertyStatusEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn type_hash(&self) -> &i32;
    fn type_hash_mut(&mut self) -> &mut i32;
    fn in_hash(&self) -> &i32;
    fn in_hash_mut(&mut self) -> &mut i32;
    fn out_hash(&self) -> &i32;
    fn out_hash_mut(&mut self) -> &mut i32;
    fn always_send_events(&self) -> &bool;
    fn always_send_events_mut(&mut self) -> &mut bool;
}

impl PropertyStatusEntityDataTrait for PropertyStatusEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn type_hash(&self) -> &i32 {
        &self.type_hash
    }
    fn type_hash_mut(&mut self) -> &mut i32 {
        &mut self.type_hash
    }
    fn in_hash(&self) -> &i32 {
        &self.in_hash
    }
    fn in_hash_mut(&mut self) -> &mut i32 {
        &mut self.in_hash
    }
    fn out_hash(&self) -> &i32 {
        &self.out_hash
    }
    fn out_hash_mut(&mut self) -> &mut i32 {
        &mut self.out_hash
    }
    fn always_send_events(&self) -> &bool {
        &self.always_send_events
    }
    fn always_send_events_mut(&mut self) -> &mut bool {
        &mut self.always_send_events
    }
}

impl super::entity::EntityDataTrait for PropertyStatusEntityData {
}

impl super::entity::GameObjectDataTrait for PropertyStatusEntityData {
}

impl super::core::DataBusPeerTrait for PropertyStatusEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for PropertyStatusEntityData {
}

impl super::core::DataContainerTrait for PropertyStatusEntityData {
}

pub static PROPERTYSTATUSENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyStatusEntityData",
    name_hash: 1144369309,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(PropertyStatusEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PropertyStatusEntityData as Default>::default())),
            create_boxed: || Box::new(<PropertyStatusEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                name_hash: 229961746,
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(PropertyStatusEntityData, realm),
            },
            FieldInfoData {
                name: "TypeHash",
                name_hash: 351092271,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PropertyStatusEntityData, type_hash),
            },
            FieldInfoData {
                name: "InHash",
                name_hash: 2782918800,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PropertyStatusEntityData, in_hash),
            },
            FieldInfoData {
                name: "OutHash",
                name_hash: 1070246745,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PropertyStatusEntityData, out_hash),
            },
            FieldInfoData {
                name: "AlwaysSendEvents",
                name_hash: 3595656247,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PropertyStatusEntityData, always_send_events),
            },
        ],
    }),
    array_type: Some(PROPERTYSTATUSENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PropertyStatusEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        PROPERTYSTATUSENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static PROPERTYSTATUSENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyStatusEntityData-Array",
    name_hash: 2996001833,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("PropertyStatusEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PropertySelectEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub type_hash: i32,
    pub input_property_hashes: Vec<i32>,
    pub input_event_hashes: Vec<i32>,
    pub out_hash: i32,
    pub input_count: u32,
    pub selected_index: i32,
    pub auto_select_on_property_changed: bool,
}

pub trait PropertySelectEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn type_hash(&self) -> &i32;
    fn type_hash_mut(&mut self) -> &mut i32;
    fn input_property_hashes(&self) -> &Vec<i32>;
    fn input_property_hashes_mut(&mut self) -> &mut Vec<i32>;
    fn input_event_hashes(&self) -> &Vec<i32>;
    fn input_event_hashes_mut(&mut self) -> &mut Vec<i32>;
    fn out_hash(&self) -> &i32;
    fn out_hash_mut(&mut self) -> &mut i32;
    fn input_count(&self) -> &u32;
    fn input_count_mut(&mut self) -> &mut u32;
    fn selected_index(&self) -> &i32;
    fn selected_index_mut(&mut self) -> &mut i32;
    fn auto_select_on_property_changed(&self) -> &bool;
    fn auto_select_on_property_changed_mut(&mut self) -> &mut bool;
}

impl PropertySelectEntityDataTrait for PropertySelectEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn type_hash(&self) -> &i32 {
        &self.type_hash
    }
    fn type_hash_mut(&mut self) -> &mut i32 {
        &mut self.type_hash
    }
    fn input_property_hashes(&self) -> &Vec<i32> {
        &self.input_property_hashes
    }
    fn input_property_hashes_mut(&mut self) -> &mut Vec<i32> {
        &mut self.input_property_hashes
    }
    fn input_event_hashes(&self) -> &Vec<i32> {
        &self.input_event_hashes
    }
    fn input_event_hashes_mut(&mut self) -> &mut Vec<i32> {
        &mut self.input_event_hashes
    }
    fn out_hash(&self) -> &i32 {
        &self.out_hash
    }
    fn out_hash_mut(&mut self) -> &mut i32 {
        &mut self.out_hash
    }
    fn input_count(&self) -> &u32 {
        &self.input_count
    }
    fn input_count_mut(&mut self) -> &mut u32 {
        &mut self.input_count
    }
    fn selected_index(&self) -> &i32 {
        &self.selected_index
    }
    fn selected_index_mut(&mut self) -> &mut i32 {
        &mut self.selected_index
    }
    fn auto_select_on_property_changed(&self) -> &bool {
        &self.auto_select_on_property_changed
    }
    fn auto_select_on_property_changed_mut(&mut self) -> &mut bool {
        &mut self.auto_select_on_property_changed
    }
}

impl super::entity::EntityDataTrait for PropertySelectEntityData {
}

impl super::entity::GameObjectDataTrait for PropertySelectEntityData {
}

impl super::core::DataBusPeerTrait for PropertySelectEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for PropertySelectEntityData {
}

impl super::core::DataContainerTrait for PropertySelectEntityData {
}

pub static PROPERTYSELECTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertySelectEntityData",
    name_hash: 2473858657,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(PropertySelectEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PropertySelectEntityData as Default>::default())),
            create_boxed: || Box::new(<PropertySelectEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                name_hash: 229961746,
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(PropertySelectEntityData, realm),
            },
            FieldInfoData {
                name: "TypeHash",
                name_hash: 351092271,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PropertySelectEntityData, type_hash),
            },
            FieldInfoData {
                name: "InputPropertyHashes",
                name_hash: 1551936464,
                flags: MemberInfoFlags::new(144),
                field_type: "Int32-Array",
                rust_offset: offset_of!(PropertySelectEntityData, input_property_hashes),
            },
            FieldInfoData {
                name: "InputEventHashes",
                name_hash: 4043293339,
                flags: MemberInfoFlags::new(144),
                field_type: "Int32-Array",
                rust_offset: offset_of!(PropertySelectEntityData, input_event_hashes),
            },
            FieldInfoData {
                name: "OutHash",
                name_hash: 1070246745,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PropertySelectEntityData, out_hash),
            },
            FieldInfoData {
                name: "InputCount",
                name_hash: 1607263120,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(PropertySelectEntityData, input_count),
            },
            FieldInfoData {
                name: "SelectedIndex",
                name_hash: 3097538226,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PropertySelectEntityData, selected_index),
            },
            FieldInfoData {
                name: "AutoSelectOnPropertyChanged",
                name_hash: 3865195046,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PropertySelectEntityData, auto_select_on_property_changed),
            },
        ],
    }),
    array_type: Some(PROPERTYSELECTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PropertySelectEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        PROPERTYSELECTENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static PROPERTYSELECTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertySelectEntityData-Array",
    name_hash: 1872843093,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("PropertySelectEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PrioritizedBoolEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub input_count: i32,
}

pub trait PrioritizedBoolEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn input_count(&self) -> &i32;
    fn input_count_mut(&mut self) -> &mut i32;
}

impl PrioritizedBoolEntityDataTrait for PrioritizedBoolEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn input_count(&self) -> &i32 {
        &self.input_count
    }
    fn input_count_mut(&mut self) -> &mut i32 {
        &mut self.input_count
    }
}

impl super::entity::EntityDataTrait for PrioritizedBoolEntityData {
}

impl super::entity::GameObjectDataTrait for PrioritizedBoolEntityData {
}

impl super::core::DataBusPeerTrait for PrioritizedBoolEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for PrioritizedBoolEntityData {
}

impl super::core::DataContainerTrait for PrioritizedBoolEntityData {
}

pub static PRIORITIZEDBOOLENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PrioritizedBoolEntityData",
    name_hash: 3056580441,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(PrioritizedBoolEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PrioritizedBoolEntityData as Default>::default())),
            create_boxed: || Box::new(<PrioritizedBoolEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                name_hash: 229961746,
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(PrioritizedBoolEntityData, realm),
            },
            FieldInfoData {
                name: "InputCount",
                name_hash: 1607263120,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PrioritizedBoolEntityData, input_count),
            },
        ],
    }),
    array_type: Some(PRIORITIZEDBOOLENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PrioritizedBoolEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        PRIORITIZEDBOOLENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static PRIORITIZEDBOOLENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PrioritizedBoolEntityData-Array",
    name_hash: 2804139373,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("PrioritizedBoolEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct NestedConditionalPropertyEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub type_hash: i32,
    pub evaluate_on_event: bool,
    pub condition_hashes: Vec<i32>,
    pub value_hashes: Vec<i32>,
}

pub trait NestedConditionalPropertyEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn type_hash(&self) -> &i32;
    fn type_hash_mut(&mut self) -> &mut i32;
    fn evaluate_on_event(&self) -> &bool;
    fn evaluate_on_event_mut(&mut self) -> &mut bool;
    fn condition_hashes(&self) -> &Vec<i32>;
    fn condition_hashes_mut(&mut self) -> &mut Vec<i32>;
    fn value_hashes(&self) -> &Vec<i32>;
    fn value_hashes_mut(&mut self) -> &mut Vec<i32>;
}

impl NestedConditionalPropertyEntityDataTrait for NestedConditionalPropertyEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn type_hash(&self) -> &i32 {
        &self.type_hash
    }
    fn type_hash_mut(&mut self) -> &mut i32 {
        &mut self.type_hash
    }
    fn evaluate_on_event(&self) -> &bool {
        &self.evaluate_on_event
    }
    fn evaluate_on_event_mut(&mut self) -> &mut bool {
        &mut self.evaluate_on_event
    }
    fn condition_hashes(&self) -> &Vec<i32> {
        &self.condition_hashes
    }
    fn condition_hashes_mut(&mut self) -> &mut Vec<i32> {
        &mut self.condition_hashes
    }
    fn value_hashes(&self) -> &Vec<i32> {
        &self.value_hashes
    }
    fn value_hashes_mut(&mut self) -> &mut Vec<i32> {
        &mut self.value_hashes
    }
}

impl super::entity::EntityDataTrait for NestedConditionalPropertyEntityData {
}

impl super::entity::GameObjectDataTrait for NestedConditionalPropertyEntityData {
}

impl super::core::DataBusPeerTrait for NestedConditionalPropertyEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for NestedConditionalPropertyEntityData {
}

impl super::core::DataContainerTrait for NestedConditionalPropertyEntityData {
}

pub static NESTEDCONDITIONALPROPERTYENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NestedConditionalPropertyEntityData",
    name_hash: 1317166682,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(NestedConditionalPropertyEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NestedConditionalPropertyEntityData as Default>::default())),
            create_boxed: || Box::new(<NestedConditionalPropertyEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                name_hash: 229961746,
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(NestedConditionalPropertyEntityData, realm),
            },
            FieldInfoData {
                name: "TypeHash",
                name_hash: 351092271,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(NestedConditionalPropertyEntityData, type_hash),
            },
            FieldInfoData {
                name: "EvaluateOnEvent",
                name_hash: 3873438067,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(NestedConditionalPropertyEntityData, evaluate_on_event),
            },
            FieldInfoData {
                name: "ConditionHashes",
                name_hash: 2090199826,
                flags: MemberInfoFlags::new(144),
                field_type: "Int32-Array",
                rust_offset: offset_of!(NestedConditionalPropertyEntityData, condition_hashes),
            },
            FieldInfoData {
                name: "ValueHashes",
                name_hash: 137208842,
                flags: MemberInfoFlags::new(144),
                field_type: "Int32-Array",
                rust_offset: offset_of!(NestedConditionalPropertyEntityData, value_hashes),
            },
        ],
    }),
    array_type: Some(NESTEDCONDITIONALPROPERTYENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NestedConditionalPropertyEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        NESTEDCONDITIONALPROPERTYENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static NESTEDCONDITIONALPROPERTYENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NestedConditionalPropertyEntityData-Array",
    name_hash: 3800912366,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("NestedConditionalPropertyEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MultiPropertyGateEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub runtime_properties: Vec<BoxedTypeObject /* MultiPropertyGatePropertyInfo */>,
    pub write_properties_on_open_gate: bool,
    pub open: bool,
}

pub trait MultiPropertyGateEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn runtime_properties(&self) -> &Vec<BoxedTypeObject /* MultiPropertyGatePropertyInfo */>;
    fn runtime_properties_mut(&mut self) -> &mut Vec<BoxedTypeObject /* MultiPropertyGatePropertyInfo */>;
    fn write_properties_on_open_gate(&self) -> &bool;
    fn write_properties_on_open_gate_mut(&mut self) -> &mut bool;
    fn open(&self) -> &bool;
    fn open_mut(&mut self) -> &mut bool;
}

impl MultiPropertyGateEntityDataTrait for MultiPropertyGateEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn runtime_properties(&self) -> &Vec<BoxedTypeObject /* MultiPropertyGatePropertyInfo */> {
        &self.runtime_properties
    }
    fn runtime_properties_mut(&mut self) -> &mut Vec<BoxedTypeObject /* MultiPropertyGatePropertyInfo */> {
        &mut self.runtime_properties
    }
    fn write_properties_on_open_gate(&self) -> &bool {
        &self.write_properties_on_open_gate
    }
    fn write_properties_on_open_gate_mut(&mut self) -> &mut bool {
        &mut self.write_properties_on_open_gate
    }
    fn open(&self) -> &bool {
        &self.open
    }
    fn open_mut(&mut self) -> &mut bool {
        &mut self.open
    }
}

impl super::entity::EntityDataTrait for MultiPropertyGateEntityData {
}

impl super::entity::GameObjectDataTrait for MultiPropertyGateEntityData {
}

impl super::core::DataBusPeerTrait for MultiPropertyGateEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for MultiPropertyGateEntityData {
}

impl super::core::DataContainerTrait for MultiPropertyGateEntityData {
}

pub static MULTIPROPERTYGATEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiPropertyGateEntityData",
    name_hash: 2705717655,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(MultiPropertyGateEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MultiPropertyGateEntityData as Default>::default())),
            create_boxed: || Box::new(<MultiPropertyGateEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                name_hash: 229961746,
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(MultiPropertyGateEntityData, realm),
            },
            FieldInfoData {
                name: "RuntimeProperties",
                name_hash: 2340289272,
                flags: MemberInfoFlags::new(144),
                field_type: "MultiPropertyGatePropertyInfo-Array",
                rust_offset: offset_of!(MultiPropertyGateEntityData, runtime_properties),
            },
            FieldInfoData {
                name: "WritePropertiesOnOpenGate",
                name_hash: 3208744923,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MultiPropertyGateEntityData, write_properties_on_open_gate),
            },
            FieldInfoData {
                name: "Open",
                name_hash: 2089008817,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MultiPropertyGateEntityData, open),
            },
        ],
    }),
    array_type: Some(MULTIPROPERTYGATEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MultiPropertyGateEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        MULTIPROPERTYGATEENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static MULTIPROPERTYGATEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiPropertyGateEntityData-Array",
    name_hash: 1320124963,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("MultiPropertyGateEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MultiPropertyGatePropertyInfo {
    pub type_hash: u32,
    pub property_hash: u32,
}

pub trait MultiPropertyGatePropertyInfoTrait: TypeObject {
    fn type_hash(&self) -> &u32;
    fn type_hash_mut(&mut self) -> &mut u32;
    fn property_hash(&self) -> &u32;
    fn property_hash_mut(&mut self) -> &mut u32;
}

impl MultiPropertyGatePropertyInfoTrait for MultiPropertyGatePropertyInfo {
    fn type_hash(&self) -> &u32 {
        &self.type_hash
    }
    fn type_hash_mut(&mut self) -> &mut u32 {
        &mut self.type_hash
    }
    fn property_hash(&self) -> &u32 {
        &self.property_hash
    }
    fn property_hash_mut(&mut self) -> &mut u32 {
        &mut self.property_hash
    }
}

pub static MULTIPROPERTYGATEPROPERTYINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiPropertyGatePropertyInfo",
    name_hash: 176770325,
    flags: MemberInfoFlags::new(36937),
    module: "DiceCommonsShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MultiPropertyGatePropertyInfo as Default>::default())),
            create_boxed: || Box::new(<MultiPropertyGatePropertyInfo as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "TypeHash",
                name_hash: 351092271,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(MultiPropertyGatePropertyInfo, type_hash),
            },
            FieldInfoData {
                name: "PropertyHash",
                name_hash: 3997998064,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(MultiPropertyGatePropertyInfo, property_hash),
            },
        ],
    }),
    array_type: Some(MULTIPROPERTYGATEPROPERTYINFO_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for MultiPropertyGatePropertyInfo {
    fn type_info(&self) -> &'static TypeInfo {
        MULTIPROPERTYGATEPROPERTYINFO_TYPE_INFO
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


pub static MULTIPROPERTYGATEPROPERTYINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiPropertyGatePropertyInfo-Array",
    name_hash: 1657470113,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("MultiPropertyGatePropertyInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LogitechLEDPulseEffectEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub enabled: bool,
    pub color1: super::core::Vec3,
    pub color2: super::core::Vec3,
    pub duration: f32,
}

pub trait LogitechLEDPulseEffectEntityDataTrait: super::entity::EntityDataTrait {
    fn enabled(&self) -> &bool;
    fn enabled_mut(&mut self) -> &mut bool;
    fn color1(&self) -> &super::core::Vec3;
    fn color1_mut(&mut self) -> &mut super::core::Vec3;
    fn color2(&self) -> &super::core::Vec3;
    fn color2_mut(&mut self) -> &mut super::core::Vec3;
    fn duration(&self) -> &f32;
    fn duration_mut(&mut self) -> &mut f32;
}

impl LogitechLEDPulseEffectEntityDataTrait for LogitechLEDPulseEffectEntityData {
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
    fn color1(&self) -> &super::core::Vec3 {
        &self.color1
    }
    fn color1_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.color1
    }
    fn color2(&self) -> &super::core::Vec3 {
        &self.color2
    }
    fn color2_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.color2
    }
    fn duration(&self) -> &f32 {
        &self.duration
    }
    fn duration_mut(&mut self) -> &mut f32 {
        &mut self.duration
    }
}

impl super::entity::EntityDataTrait for LogitechLEDPulseEffectEntityData {
}

impl super::entity::GameObjectDataTrait for LogitechLEDPulseEffectEntityData {
}

impl super::core::DataBusPeerTrait for LogitechLEDPulseEffectEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for LogitechLEDPulseEffectEntityData {
}

impl super::core::DataContainerTrait for LogitechLEDPulseEffectEntityData {
}

pub static LOGITECHLEDPULSEEFFECTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDPulseEffectEntityData",
    name_hash: 2417757884,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(LogitechLEDPulseEffectEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LogitechLEDPulseEffectEntityData as Default>::default())),
            create_boxed: || Box::new(<LogitechLEDPulseEffectEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Enabled",
                name_hash: 2662400,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LogitechLEDPulseEffectEntityData, enabled),
            },
            FieldInfoData {
                name: "Color1",
                name_hash: 2713814217,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(LogitechLEDPulseEffectEntityData, color1),
            },
            FieldInfoData {
                name: "Color2",
                name_hash: 2713814218,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(LogitechLEDPulseEffectEntityData, color2),
            },
            FieldInfoData {
                name: "Duration",
                name_hash: 1828507227,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LogitechLEDPulseEffectEntityData, duration),
            },
        ],
    }),
    array_type: Some(LOGITECHLEDPULSEEFFECTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LogitechLEDPulseEffectEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        LOGITECHLEDPULSEEFFECTENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static LOGITECHLEDPULSEEFFECTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDPulseEffectEntityData-Array",
    name_hash: 203212296,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("LogitechLEDPulseEffectEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LogitechLEDInputConceptIdentifierEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub enabled: bool,
    pub color: super::core::Vec3,
    pub input_concept_identifiers: Vec<i32>,
}

pub trait LogitechLEDInputConceptIdentifierEntityDataTrait: super::entity::EntityDataTrait {
    fn enabled(&self) -> &bool;
    fn enabled_mut(&mut self) -> &mut bool;
    fn color(&self) -> &super::core::Vec3;
    fn color_mut(&mut self) -> &mut super::core::Vec3;
    fn input_concept_identifiers(&self) -> &Vec<i32>;
    fn input_concept_identifiers_mut(&mut self) -> &mut Vec<i32>;
}

impl LogitechLEDInputConceptIdentifierEntityDataTrait for LogitechLEDInputConceptIdentifierEntityData {
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
    fn color(&self) -> &super::core::Vec3 {
        &self.color
    }
    fn color_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.color
    }
    fn input_concept_identifiers(&self) -> &Vec<i32> {
        &self.input_concept_identifiers
    }
    fn input_concept_identifiers_mut(&mut self) -> &mut Vec<i32> {
        &mut self.input_concept_identifiers
    }
}

impl super::entity::EntityDataTrait for LogitechLEDInputConceptIdentifierEntityData {
}

impl super::entity::GameObjectDataTrait for LogitechLEDInputConceptIdentifierEntityData {
}

impl super::core::DataBusPeerTrait for LogitechLEDInputConceptIdentifierEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for LogitechLEDInputConceptIdentifierEntityData {
}

impl super::core::DataContainerTrait for LogitechLEDInputConceptIdentifierEntityData {
}

pub static LOGITECHLEDINPUTCONCEPTIDENTIFIERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDInputConceptIdentifierEntityData",
    name_hash: 582016385,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(LogitechLEDInputConceptIdentifierEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LogitechLEDInputConceptIdentifierEntityData as Default>::default())),
            create_boxed: || Box::new(<LogitechLEDInputConceptIdentifierEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Enabled",
                name_hash: 2662400,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LogitechLEDInputConceptIdentifierEntityData, enabled),
            },
            FieldInfoData {
                name: "Color",
                name_hash: 212387320,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(LogitechLEDInputConceptIdentifierEntityData, color),
            },
            FieldInfoData {
                name: "InputConceptIdentifiers",
                name_hash: 2927413955,
                flags: MemberInfoFlags::new(144),
                field_type: "Int32-Array",
                rust_offset: offset_of!(LogitechLEDInputConceptIdentifierEntityData, input_concept_identifiers),
            },
        ],
    }),
    array_type: Some(LOGITECHLEDINPUTCONCEPTIDENTIFIERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LogitechLEDInputConceptIdentifierEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        LOGITECHLEDINPUTCONCEPTIDENTIFIERENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static LOGITECHLEDINPUTCONCEPTIDENTIFIERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDInputConceptIdentifierEntityData-Array",
    name_hash: 178943797,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("LogitechLEDInputConceptIdentifierEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LogitechLEDFadeEffectEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub enabled: bool,
    pub color1: super::core::Vec3,
    pub color2: super::core::Vec3,
    pub duration: f32,
}

pub trait LogitechLEDFadeEffectEntityDataTrait: super::entity::EntityDataTrait {
    fn enabled(&self) -> &bool;
    fn enabled_mut(&mut self) -> &mut bool;
    fn color1(&self) -> &super::core::Vec3;
    fn color1_mut(&mut self) -> &mut super::core::Vec3;
    fn color2(&self) -> &super::core::Vec3;
    fn color2_mut(&mut self) -> &mut super::core::Vec3;
    fn duration(&self) -> &f32;
    fn duration_mut(&mut self) -> &mut f32;
}

impl LogitechLEDFadeEffectEntityDataTrait for LogitechLEDFadeEffectEntityData {
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
    fn color1(&self) -> &super::core::Vec3 {
        &self.color1
    }
    fn color1_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.color1
    }
    fn color2(&self) -> &super::core::Vec3 {
        &self.color2
    }
    fn color2_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.color2
    }
    fn duration(&self) -> &f32 {
        &self.duration
    }
    fn duration_mut(&mut self) -> &mut f32 {
        &mut self.duration
    }
}

impl super::entity::EntityDataTrait for LogitechLEDFadeEffectEntityData {
}

impl super::entity::GameObjectDataTrait for LogitechLEDFadeEffectEntityData {
}

impl super::core::DataBusPeerTrait for LogitechLEDFadeEffectEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for LogitechLEDFadeEffectEntityData {
}

impl super::core::DataContainerTrait for LogitechLEDFadeEffectEntityData {
}

pub static LOGITECHLEDFADEEFFECTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDFadeEffectEntityData",
    name_hash: 2425860837,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(LogitechLEDFadeEffectEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LogitechLEDFadeEffectEntityData as Default>::default())),
            create_boxed: || Box::new(<LogitechLEDFadeEffectEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Enabled",
                name_hash: 2662400,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LogitechLEDFadeEffectEntityData, enabled),
            },
            FieldInfoData {
                name: "Color1",
                name_hash: 2713814217,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(LogitechLEDFadeEffectEntityData, color1),
            },
            FieldInfoData {
                name: "Color2",
                name_hash: 2713814218,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(LogitechLEDFadeEffectEntityData, color2),
            },
            FieldInfoData {
                name: "Duration",
                name_hash: 1828507227,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LogitechLEDFadeEffectEntityData, duration),
            },
        ],
    }),
    array_type: Some(LOGITECHLEDFADEEFFECTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LogitechLEDFadeEffectEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        LOGITECHLEDFADEEFFECTENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static LOGITECHLEDFADEEFFECTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDFadeEffectEntityData-Array",
    name_hash: 2523397073,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("LogitechLEDFadeEffectEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LogitechLEDConstantEffectEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub enabled: bool,
    pub color: super::core::Vec3,
}

pub trait LogitechLEDConstantEffectEntityDataTrait: super::entity::EntityDataTrait {
    fn enabled(&self) -> &bool;
    fn enabled_mut(&mut self) -> &mut bool;
    fn color(&self) -> &super::core::Vec3;
    fn color_mut(&mut self) -> &mut super::core::Vec3;
}

impl LogitechLEDConstantEffectEntityDataTrait for LogitechLEDConstantEffectEntityData {
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
    fn color(&self) -> &super::core::Vec3 {
        &self.color
    }
    fn color_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.color
    }
}

impl super::entity::EntityDataTrait for LogitechLEDConstantEffectEntityData {
}

impl super::entity::GameObjectDataTrait for LogitechLEDConstantEffectEntityData {
}

impl super::core::DataBusPeerTrait for LogitechLEDConstantEffectEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for LogitechLEDConstantEffectEntityData {
}

impl super::core::DataContainerTrait for LogitechLEDConstantEffectEntityData {
}

pub static LOGITECHLEDCONSTANTEFFECTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDConstantEffectEntityData",
    name_hash: 3811532253,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(LogitechLEDConstantEffectEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LogitechLEDConstantEffectEntityData as Default>::default())),
            create_boxed: || Box::new(<LogitechLEDConstantEffectEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Enabled",
                name_hash: 2662400,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LogitechLEDConstantEffectEntityData, enabled),
            },
            FieldInfoData {
                name: "Color",
                name_hash: 212387320,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(LogitechLEDConstantEffectEntityData, color),
            },
        ],
    }),
    array_type: Some(LOGITECHLEDCONSTANTEFFECTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LogitechLEDConstantEffectEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        LOGITECHLEDCONSTANTEFFECTENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static LOGITECHLEDCONSTANTEFFECTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDConstantEffectEntityData-Array",
    name_hash: 958790121,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("LogitechLEDConstantEffectEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LogitechLEDConditionalInputConceptIdentifierEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub enabled: bool,
    pub color_condition: bool,
    pub condition_true_color: super::core::Vec3,
    pub condition_false_color: super::core::Vec3,
    pub input_concept_identifiers: Vec<i32>,
}

pub trait LogitechLEDConditionalInputConceptIdentifierEntityDataTrait: super::entity::EntityDataTrait {
    fn enabled(&self) -> &bool;
    fn enabled_mut(&mut self) -> &mut bool;
    fn color_condition(&self) -> &bool;
    fn color_condition_mut(&mut self) -> &mut bool;
    fn condition_true_color(&self) -> &super::core::Vec3;
    fn condition_true_color_mut(&mut self) -> &mut super::core::Vec3;
    fn condition_false_color(&self) -> &super::core::Vec3;
    fn condition_false_color_mut(&mut self) -> &mut super::core::Vec3;
    fn input_concept_identifiers(&self) -> &Vec<i32>;
    fn input_concept_identifiers_mut(&mut self) -> &mut Vec<i32>;
}

impl LogitechLEDConditionalInputConceptIdentifierEntityDataTrait for LogitechLEDConditionalInputConceptIdentifierEntityData {
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
    fn color_condition(&self) -> &bool {
        &self.color_condition
    }
    fn color_condition_mut(&mut self) -> &mut bool {
        &mut self.color_condition
    }
    fn condition_true_color(&self) -> &super::core::Vec3 {
        &self.condition_true_color
    }
    fn condition_true_color_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.condition_true_color
    }
    fn condition_false_color(&self) -> &super::core::Vec3 {
        &self.condition_false_color
    }
    fn condition_false_color_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.condition_false_color
    }
    fn input_concept_identifiers(&self) -> &Vec<i32> {
        &self.input_concept_identifiers
    }
    fn input_concept_identifiers_mut(&mut self) -> &mut Vec<i32> {
        &mut self.input_concept_identifiers
    }
}

impl super::entity::EntityDataTrait for LogitechLEDConditionalInputConceptIdentifierEntityData {
}

impl super::entity::GameObjectDataTrait for LogitechLEDConditionalInputConceptIdentifierEntityData {
}

impl super::core::DataBusPeerTrait for LogitechLEDConditionalInputConceptIdentifierEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for LogitechLEDConditionalInputConceptIdentifierEntityData {
}

impl super::core::DataContainerTrait for LogitechLEDConditionalInputConceptIdentifierEntityData {
}

pub static LOGITECHLEDCONDITIONALINPUTCONCEPTIDENTIFIERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDConditionalInputConceptIdentifierEntityData",
    name_hash: 3368667679,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(LogitechLEDConditionalInputConceptIdentifierEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LogitechLEDConditionalInputConceptIdentifierEntityData as Default>::default())),
            create_boxed: || Box::new(<LogitechLEDConditionalInputConceptIdentifierEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Enabled",
                name_hash: 2662400,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LogitechLEDConditionalInputConceptIdentifierEntityData, enabled),
            },
            FieldInfoData {
                name: "ColorCondition",
                name_hash: 3724299563,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LogitechLEDConditionalInputConceptIdentifierEntityData, color_condition),
            },
            FieldInfoData {
                name: "ConditionTrueColor",
                name_hash: 2787653757,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(LogitechLEDConditionalInputConceptIdentifierEntityData, condition_true_color),
            },
            FieldInfoData {
                name: "ConditionFalseColor",
                name_hash: 2636737494,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(LogitechLEDConditionalInputConceptIdentifierEntityData, condition_false_color),
            },
            FieldInfoData {
                name: "InputConceptIdentifiers",
                name_hash: 2927413955,
                flags: MemberInfoFlags::new(144),
                field_type: "Int32-Array",
                rust_offset: offset_of!(LogitechLEDConditionalInputConceptIdentifierEntityData, input_concept_identifiers),
            },
        ],
    }),
    array_type: Some(LOGITECHLEDCONDITIONALINPUTCONCEPTIDENTIFIERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LogitechLEDConditionalInputConceptIdentifierEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        LOGITECHLEDCONDITIONALINPUTCONCEPTIDENTIFIERENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static LOGITECHLEDCONDITIONALINPUTCONCEPTIDENTIFIERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDConditionalInputConceptIdentifierEntityData-Array",
    name_hash: 3714911915,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("LogitechLEDConditionalInputConceptIdentifierEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LogitechLEDBarEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub enabled: bool,
    pub value: f32,
    pub bar_color: super::core::Vec3,
    pub background_color: super::core::Vec3,
    pub input_device_keys: Vec<super::input_shared::InputDeviceKeys>,
}

pub trait LogitechLEDBarEntityDataTrait: super::entity::EntityDataTrait {
    fn enabled(&self) -> &bool;
    fn enabled_mut(&mut self) -> &mut bool;
    fn value(&self) -> &f32;
    fn value_mut(&mut self) -> &mut f32;
    fn bar_color(&self) -> &super::core::Vec3;
    fn bar_color_mut(&mut self) -> &mut super::core::Vec3;
    fn background_color(&self) -> &super::core::Vec3;
    fn background_color_mut(&mut self) -> &mut super::core::Vec3;
    fn input_device_keys(&self) -> &Vec<super::input_shared::InputDeviceKeys>;
    fn input_device_keys_mut(&mut self) -> &mut Vec<super::input_shared::InputDeviceKeys>;
}

impl LogitechLEDBarEntityDataTrait for LogitechLEDBarEntityData {
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
    fn value(&self) -> &f32 {
        &self.value
    }
    fn value_mut(&mut self) -> &mut f32 {
        &mut self.value
    }
    fn bar_color(&self) -> &super::core::Vec3 {
        &self.bar_color
    }
    fn bar_color_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.bar_color
    }
    fn background_color(&self) -> &super::core::Vec3 {
        &self.background_color
    }
    fn background_color_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.background_color
    }
    fn input_device_keys(&self) -> &Vec<super::input_shared::InputDeviceKeys> {
        &self.input_device_keys
    }
    fn input_device_keys_mut(&mut self) -> &mut Vec<super::input_shared::InputDeviceKeys> {
        &mut self.input_device_keys
    }
}

impl super::entity::EntityDataTrait for LogitechLEDBarEntityData {
}

impl super::entity::GameObjectDataTrait for LogitechLEDBarEntityData {
}

impl super::core::DataBusPeerTrait for LogitechLEDBarEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for LogitechLEDBarEntityData {
}

impl super::core::DataContainerTrait for LogitechLEDBarEntityData {
}

pub static LOGITECHLEDBARENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDBarEntityData",
    name_hash: 464127269,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(LogitechLEDBarEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LogitechLEDBarEntityData as Default>::default())),
            create_boxed: || Box::new(<LogitechLEDBarEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Enabled",
                name_hash: 2662400,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LogitechLEDBarEntityData, enabled),
            },
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LogitechLEDBarEntityData, value),
            },
            FieldInfoData {
                name: "BarColor",
                name_hash: 3186704841,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(LogitechLEDBarEntityData, bar_color),
            },
            FieldInfoData {
                name: "BackgroundColor",
                name_hash: 2973036470,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(LogitechLEDBarEntityData, background_color),
            },
            FieldInfoData {
                name: "InputDeviceKeys",
                name_hash: 62416367,
                flags: MemberInfoFlags::new(144),
                field_type: "InputDeviceKeys-Array",
                rust_offset: offset_of!(LogitechLEDBarEntityData, input_device_keys),
            },
        ],
    }),
    array_type: Some(LOGITECHLEDBARENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LogitechLEDBarEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        LOGITECHLEDBARENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static LOGITECHLEDBARENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDBarEntityData-Array",
    name_hash: 61783441,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("LogitechLEDBarEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LocalLocatorEntityData {
    pub _glacier_base: super::entity::SpatialEntityData,
    pub realm: super::core::Realm,
}

pub trait LocalLocatorEntityDataTrait: super::entity::SpatialEntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
}

impl LocalLocatorEntityDataTrait for LocalLocatorEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
}

impl super::entity::SpatialEntityDataTrait for LocalLocatorEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::entity::EntityDataTrait for LocalLocatorEntityData {
}

impl super::entity::GameObjectDataTrait for LocalLocatorEntityData {
}

impl super::core::DataBusPeerTrait for LocalLocatorEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for LocalLocatorEntityData {
}

impl super::core::DataContainerTrait for LocalLocatorEntityData {
}

pub static LOCALLOCATORENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalLocatorEntityData",
    name_hash: 3012966379,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(LocalLocatorEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LocalLocatorEntityData as Default>::default())),
            create_boxed: || Box::new(<LocalLocatorEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                name_hash: 229961746,
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(LocalLocatorEntityData, realm),
            },
        ],
    }),
    array_type: Some(LOCALLOCATORENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LocalLocatorEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        LOCALLOCATORENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static LOCALLOCATORENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalLocatorEntityData-Array",
    name_hash: 1013065695,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("LocalLocatorEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WaveSelectorNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
    pub output_wave: super::audio::AudioGraphNodePort,
    pub value: super::audio::AudioGraphNodePort,
    pub default_case_value: u32,
    pub input_waves: Vec<Option<LockedTypeObject /* InputWavesGroup */>>,
}

pub trait WaveSelectorNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
    fn output_wave(&self) -> &super::audio::AudioGraphNodePort;
    fn output_wave_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn value(&self) -> &super::audio::AudioGraphNodePort;
    fn value_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn default_case_value(&self) -> &u32;
    fn default_case_value_mut(&mut self) -> &mut u32;
    fn input_waves(&self) -> &Vec<Option<LockedTypeObject /* InputWavesGroup */>>;
    fn input_waves_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* InputWavesGroup */>>;
}

impl WaveSelectorNodeDataTrait for WaveSelectorNodeData {
    fn output_wave(&self) -> &super::audio::AudioGraphNodePort {
        &self.output_wave
    }
    fn output_wave_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.output_wave
    }
    fn value(&self) -> &super::audio::AudioGraphNodePort {
        &self.value
    }
    fn value_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.value
    }
    fn default_case_value(&self) -> &u32 {
        &self.default_case_value
    }
    fn default_case_value_mut(&mut self) -> &mut u32 {
        &mut self.default_case_value
    }
    fn input_waves(&self) -> &Vec<Option<LockedTypeObject /* InputWavesGroup */>> {
        &self.input_waves
    }
    fn input_waves_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* InputWavesGroup */>> {
        &mut self.input_waves
    }
}

impl super::audio::AudioGraphNodeDataTrait for WaveSelectorNodeData {
}

impl super::core::DataContainerTrait for WaveSelectorNodeData {
}

pub static WAVESELECTORNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaveSelectorNodeData",
    name_hash: 3348933509,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        super_class_offset: offset_of!(WaveSelectorNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaveSelectorNodeData as Default>::default())),
            create_boxed: || Box::new(<WaveSelectorNodeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "OutputWave",
                name_hash: 543005759,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(WaveSelectorNodeData, output_wave),
            },
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(WaveSelectorNodeData, value),
            },
            FieldInfoData {
                name: "DefaultCaseValue",
                name_hash: 3296679953,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WaveSelectorNodeData, default_case_value),
            },
            FieldInfoData {
                name: "InputWaves",
                name_hash: 1592421669,
                flags: MemberInfoFlags::new(144),
                field_type: "InputWavesGroup-Array",
                rust_offset: offset_of!(WaveSelectorNodeData, input_waves),
            },
        ],
    }),
    array_type: Some(WAVESELECTORNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WaveSelectorNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        WAVESELECTORNODEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static WAVESELECTORNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaveSelectorNodeData-Array",
    name_hash: 4059591473,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("WaveSelectorNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct InputWavesGroup {
    pub _glacier_base: super::audio::AudioGraphNodePortGroup,
    pub default_wave: Option<LockedTypeObject /* super::audio::SoundWaveAssetBase */>,
    pub wave: super::audio::AudioGraphNodePort,
}

pub trait InputWavesGroupTrait: super::audio::AudioGraphNodePortGroupTrait {
    fn default_wave(&self) -> &Option<LockedTypeObject /* super::audio::SoundWaveAssetBase */>;
    fn default_wave_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundWaveAssetBase */>;
    fn wave(&self) -> &super::audio::AudioGraphNodePort;
    fn wave_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
}

impl InputWavesGroupTrait for InputWavesGroup {
    fn default_wave(&self) -> &Option<LockedTypeObject /* super::audio::SoundWaveAssetBase */> {
        &self.default_wave
    }
    fn default_wave_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundWaveAssetBase */> {
        &mut self.default_wave
    }
    fn wave(&self) -> &super::audio::AudioGraphNodePort {
        &self.wave
    }
    fn wave_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.wave
    }
}

impl super::audio::AudioGraphNodePortGroupTrait for InputWavesGroup {
}

impl super::core::DataContainerTrait for InputWavesGroup {
}

pub static INPUTWAVESGROUP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputWavesGroup",
    name_hash: 3573542266,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEPORTGROUP_TYPE_INFO),
        super_class_offset: offset_of!(InputWavesGroup, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InputWavesGroup as Default>::default())),
            create_boxed: || Box::new(<InputWavesGroup as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "DefaultWave",
                name_hash: 2015103915,
                flags: MemberInfoFlags::new(0),
                field_type: "SoundWaveAssetBase",
                rust_offset: offset_of!(InputWavesGroup, default_wave),
            },
            FieldInfoData {
                name: "Wave",
                name_hash: 2089277184,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(InputWavesGroup, wave),
            },
        ],
    }),
    array_type: Some(INPUTWAVESGROUP_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for InputWavesGroup {
    fn type_info(&self) -> &'static TypeInfo {
        INPUTWAVESGROUP_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static INPUTWAVESGROUP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputWavesGroup-Array",
    name_hash: 1531265102,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("InputWavesGroup"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ValueCacheNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
    pub store: super::audio::AudioGraphNodePort,
    pub value: super::audio::AudioGraphNodePort,
    pub out: super::audio::AudioGraphNodePort,
    pub default_value: f32,
    pub set_initial_value_as_default: bool,
}

pub trait ValueCacheNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
    fn store(&self) -> &super::audio::AudioGraphNodePort;
    fn store_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn value(&self) -> &super::audio::AudioGraphNodePort;
    fn value_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn out(&self) -> &super::audio::AudioGraphNodePort;
    fn out_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn default_value(&self) -> &f32;
    fn default_value_mut(&mut self) -> &mut f32;
    fn set_initial_value_as_default(&self) -> &bool;
    fn set_initial_value_as_default_mut(&mut self) -> &mut bool;
}

impl ValueCacheNodeDataTrait for ValueCacheNodeData {
    fn store(&self) -> &super::audio::AudioGraphNodePort {
        &self.store
    }
    fn store_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.store
    }
    fn value(&self) -> &super::audio::AudioGraphNodePort {
        &self.value
    }
    fn value_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.value
    }
    fn out(&self) -> &super::audio::AudioGraphNodePort {
        &self.out
    }
    fn out_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.out
    }
    fn default_value(&self) -> &f32 {
        &self.default_value
    }
    fn default_value_mut(&mut self) -> &mut f32 {
        &mut self.default_value
    }
    fn set_initial_value_as_default(&self) -> &bool {
        &self.set_initial_value_as_default
    }
    fn set_initial_value_as_default_mut(&mut self) -> &mut bool {
        &mut self.set_initial_value_as_default
    }
}

impl super::audio::AudioGraphNodeDataTrait for ValueCacheNodeData {
}

impl super::core::DataContainerTrait for ValueCacheNodeData {
}

pub static VALUECACHENODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ValueCacheNodeData",
    name_hash: 461901522,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        super_class_offset: offset_of!(ValueCacheNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ValueCacheNodeData as Default>::default())),
            create_boxed: || Box::new(<ValueCacheNodeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Store",
                name_hash: 230763322,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ValueCacheNodeData, store),
            },
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ValueCacheNodeData, value),
            },
            FieldInfoData {
                name: "Out",
                name_hash: 193453899,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ValueCacheNodeData, out),
            },
            FieldInfoData {
                name: "DefaultValue",
                name_hash: 2066049125,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ValueCacheNodeData, default_value),
            },
            FieldInfoData {
                name: "SetInitialValueAsDefault",
                name_hash: 47763307,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ValueCacheNodeData, set_initial_value_as_default),
            },
        ],
    }),
    array_type: Some(VALUECACHENODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ValueCacheNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        VALUECACHENODEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static VALUECACHENODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ValueCacheNodeData-Array",
    name_hash: 3820357222,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ValueCacheNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SystemInformationNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
    pub channel_count: super::audio::AudioGraphNodePort,
}

pub trait SystemInformationNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
    fn channel_count(&self) -> &super::audio::AudioGraphNodePort;
    fn channel_count_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
}

impl SystemInformationNodeDataTrait for SystemInformationNodeData {
    fn channel_count(&self) -> &super::audio::AudioGraphNodePort {
        &self.channel_count
    }
    fn channel_count_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.channel_count
    }
}

impl super::audio::AudioGraphNodeDataTrait for SystemInformationNodeData {
}

impl super::core::DataContainerTrait for SystemInformationNodeData {
}

pub static SYSTEMINFORMATIONNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SystemInformationNodeData",
    name_hash: 655608444,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        super_class_offset: offset_of!(SystemInformationNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SystemInformationNodeData as Default>::default())),
            create_boxed: || Box::new(<SystemInformationNodeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ChannelCount",
                name_hash: 1014205285,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(SystemInformationNodeData, channel_count),
            },
        ],
    }),
    array_type: Some(SYSTEMINFORMATIONNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SystemInformationNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        SYSTEMINFORMATIONNODEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SYSTEMINFORMATIONNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SystemInformationNodeData-Array",
    name_hash: 1033785928,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("SystemInformationNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SyncedRandomIntNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
    pub trigger: super::audio::AudioGraphNodePort,
    pub triggered: super::audio::AudioGraphNodePort,
    pub out: super::audio::AudioGraphNodePort,
    pub min: i32,
    pub max: i32,
    pub high_precision: bool,
}

pub trait SyncedRandomIntNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
    fn trigger(&self) -> &super::audio::AudioGraphNodePort;
    fn trigger_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn triggered(&self) -> &super::audio::AudioGraphNodePort;
    fn triggered_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn out(&self) -> &super::audio::AudioGraphNodePort;
    fn out_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn min(&self) -> &i32;
    fn min_mut(&mut self) -> &mut i32;
    fn max(&self) -> &i32;
    fn max_mut(&mut self) -> &mut i32;
    fn high_precision(&self) -> &bool;
    fn high_precision_mut(&mut self) -> &mut bool;
}

impl SyncedRandomIntNodeDataTrait for SyncedRandomIntNodeData {
    fn trigger(&self) -> &super::audio::AudioGraphNodePort {
        &self.trigger
    }
    fn trigger_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.trigger
    }
    fn triggered(&self) -> &super::audio::AudioGraphNodePort {
        &self.triggered
    }
    fn triggered_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.triggered
    }
    fn out(&self) -> &super::audio::AudioGraphNodePort {
        &self.out
    }
    fn out_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.out
    }
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
    fn high_precision(&self) -> &bool {
        &self.high_precision
    }
    fn high_precision_mut(&mut self) -> &mut bool {
        &mut self.high_precision
    }
}

impl super::audio::AudioGraphNodeDataTrait for SyncedRandomIntNodeData {
}

impl super::core::DataContainerTrait for SyncedRandomIntNodeData {
}

pub static SYNCEDRANDOMINTNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SyncedRandomIntNodeData",
    name_hash: 1682677755,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        super_class_offset: offset_of!(SyncedRandomIntNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SyncedRandomIntNodeData as Default>::default())),
            create_boxed: || Box::new(<SyncedRandomIntNodeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Trigger",
                name_hash: 2606354109,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(SyncedRandomIntNodeData, trigger),
            },
            FieldInfoData {
                name: "Triggered",
                name_hash: 3641208156,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(SyncedRandomIntNodeData, triggered),
            },
            FieldInfoData {
                name: "Out",
                name_hash: 193453899,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(SyncedRandomIntNodeData, out),
            },
            FieldInfoData {
                name: "Min",
                name_hash: 193446607,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(SyncedRandomIntNodeData, min),
            },
            FieldInfoData {
                name: "Max",
                name_hash: 193446865,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(SyncedRandomIntNodeData, max),
            },
            FieldInfoData {
                name: "HighPrecision",
                name_hash: 2143407709,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SyncedRandomIntNodeData, high_precision),
            },
        ],
    }),
    array_type: Some(SYNCEDRANDOMINTNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SyncedRandomIntNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        SYNCEDRANDOMINTNODEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SYNCEDRANDOMINTNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SyncedRandomIntNodeData-Array",
    name_hash: 1494767567,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("SyncedRandomIntNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct StepLogicSamplerNodeConfigData {
    pub _glacier_base: super::audio::AudioGraphNodeConfigData,
    pub amplitude: f32,
    pub pitch: f32,
    pub rate_of_fire: f32,
    pub wave: Option<LockedTypeObject /* super::audio::SoundWaveAssetBase */>,
    pub maximum_rate_of_fire: f32,
    pub fade_type: super::audio::GainFaderFadeType,
}

pub trait StepLogicSamplerNodeConfigDataTrait: super::audio::AudioGraphNodeConfigDataTrait {
    fn amplitude(&self) -> &f32;
    fn amplitude_mut(&mut self) -> &mut f32;
    fn pitch(&self) -> &f32;
    fn pitch_mut(&mut self) -> &mut f32;
    fn rate_of_fire(&self) -> &f32;
    fn rate_of_fire_mut(&mut self) -> &mut f32;
    fn wave(&self) -> &Option<LockedTypeObject /* super::audio::SoundWaveAssetBase */>;
    fn wave_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundWaveAssetBase */>;
    fn maximum_rate_of_fire(&self) -> &f32;
    fn maximum_rate_of_fire_mut(&mut self) -> &mut f32;
    fn fade_type(&self) -> &super::audio::GainFaderFadeType;
    fn fade_type_mut(&mut self) -> &mut super::audio::GainFaderFadeType;
}

impl StepLogicSamplerNodeConfigDataTrait for StepLogicSamplerNodeConfigData {
    fn amplitude(&self) -> &f32 {
        &self.amplitude
    }
    fn amplitude_mut(&mut self) -> &mut f32 {
        &mut self.amplitude
    }
    fn pitch(&self) -> &f32 {
        &self.pitch
    }
    fn pitch_mut(&mut self) -> &mut f32 {
        &mut self.pitch
    }
    fn rate_of_fire(&self) -> &f32 {
        &self.rate_of_fire
    }
    fn rate_of_fire_mut(&mut self) -> &mut f32 {
        &mut self.rate_of_fire
    }
    fn wave(&self) -> &Option<LockedTypeObject /* super::audio::SoundWaveAssetBase */> {
        &self.wave
    }
    fn wave_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundWaveAssetBase */> {
        &mut self.wave
    }
    fn maximum_rate_of_fire(&self) -> &f32 {
        &self.maximum_rate_of_fire
    }
    fn maximum_rate_of_fire_mut(&mut self) -> &mut f32 {
        &mut self.maximum_rate_of_fire
    }
    fn fade_type(&self) -> &super::audio::GainFaderFadeType {
        &self.fade_type
    }
    fn fade_type_mut(&mut self) -> &mut super::audio::GainFaderFadeType {
        &mut self.fade_type
    }
}

impl super::audio::AudioGraphNodeConfigDataTrait for StepLogicSamplerNodeConfigData {
    fn node(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphNodeData */> {
        self._glacier_base.node()
    }
    fn node_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphNodeData */> {
        self._glacier_base.node_mut()
    }
    fn configured_property_flags(&self) -> &u64 {
        self._glacier_base.configured_property_flags()
    }
    fn configured_property_flags_mut(&mut self) -> &mut u64 {
        self._glacier_base.configured_property_flags_mut()
    }
}

impl super::core::DataContainerTrait for StepLogicSamplerNodeConfigData {
}

pub static STEPLOGICSAMPLERNODECONFIGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StepLogicSamplerNodeConfigData",
    name_hash: 3425852055,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODECONFIGDATA_TYPE_INFO),
        super_class_offset: offset_of!(StepLogicSamplerNodeConfigData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StepLogicSamplerNodeConfigData as Default>::default())),
            create_boxed: || Box::new(<StepLogicSamplerNodeConfigData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Amplitude",
                name_hash: 698564572,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(StepLogicSamplerNodeConfigData, amplitude),
            },
            FieldInfoData {
                name: "Pitch",
                name_hash: 232604323,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(StepLogicSamplerNodeConfigData, pitch),
            },
            FieldInfoData {
                name: "RateOfFire",
                name_hash: 3866082710,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(StepLogicSamplerNodeConfigData, rate_of_fire),
            },
            FieldInfoData {
                name: "Wave",
                name_hash: 2089277184,
                flags: MemberInfoFlags::new(0),
                field_type: "SoundWaveAssetBase",
                rust_offset: offset_of!(StepLogicSamplerNodeConfigData, wave),
            },
            FieldInfoData {
                name: "MaximumRateOfFire",
                name_hash: 3281074206,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(StepLogicSamplerNodeConfigData, maximum_rate_of_fire),
            },
            FieldInfoData {
                name: "FadeType",
                name_hash: 4001206363,
                flags: MemberInfoFlags::new(0),
                field_type: "GainFaderFadeType",
                rust_offset: offset_of!(StepLogicSamplerNodeConfigData, fade_type),
            },
        ],
    }),
    array_type: Some(STEPLOGICSAMPLERNODECONFIGDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StepLogicSamplerNodeConfigData {
    fn type_info(&self) -> &'static TypeInfo {
        STEPLOGICSAMPLERNODECONFIGDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static STEPLOGICSAMPLERNODECONFIGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StepLogicSamplerNodeConfigData-Array",
    name_hash: 3416302371,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("StepLogicSamplerNodeConfigData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct StepLogicSamplerNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
    pub external_wave: super::audio::AudioGraphNodePort,
    pub pitch: super::audio::AudioGraphNodePort,
    pub amplitude: super::audio::AudioGraphNodePort,
    pub rate_of_fire: super::audio::AudioGraphNodePort,
    pub trigger: super::audio::AudioGraphNodePort,
    pub release: super::audio::AudioGraphNodePort,
    pub output: super::audio::AudioGraphNodePort,
    pub finished: super::audio::AudioGraphNodePort,
    pub wave: Option<LockedTypeObject /* super::audio::SoundWaveAssetBase */>,
    pub plugins: Vec<BoxedTypeObject /* StepLogicSamplerPlugins */>,
    pub pitch_source: Option<LockedTypeObject /* super::audio::OutputNodeData */>,
    pub maximum_rate_of_fire: f32,
    pub fade_type: super::audio::GainFaderFadeType,
    pub sampler_node_debug: Option<LockedTypeObject /* StepLogicSamplerNodeDebugData */>,
}

pub trait StepLogicSamplerNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
    fn external_wave(&self) -> &super::audio::AudioGraphNodePort;
    fn external_wave_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn pitch(&self) -> &super::audio::AudioGraphNodePort;
    fn pitch_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn amplitude(&self) -> &super::audio::AudioGraphNodePort;
    fn amplitude_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn rate_of_fire(&self) -> &super::audio::AudioGraphNodePort;
    fn rate_of_fire_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn trigger(&self) -> &super::audio::AudioGraphNodePort;
    fn trigger_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn release(&self) -> &super::audio::AudioGraphNodePort;
    fn release_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn output(&self) -> &super::audio::AudioGraphNodePort;
    fn output_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn finished(&self) -> &super::audio::AudioGraphNodePort;
    fn finished_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn wave(&self) -> &Option<LockedTypeObject /* super::audio::SoundWaveAssetBase */>;
    fn wave_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundWaveAssetBase */>;
    fn plugins(&self) -> &Vec<BoxedTypeObject /* StepLogicSamplerPlugins */>;
    fn plugins_mut(&mut self) -> &mut Vec<BoxedTypeObject /* StepLogicSamplerPlugins */>;
    fn pitch_source(&self) -> &Option<LockedTypeObject /* super::audio::OutputNodeData */>;
    fn pitch_source_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::OutputNodeData */>;
    fn maximum_rate_of_fire(&self) -> &f32;
    fn maximum_rate_of_fire_mut(&mut self) -> &mut f32;
    fn fade_type(&self) -> &super::audio::GainFaderFadeType;
    fn fade_type_mut(&mut self) -> &mut super::audio::GainFaderFadeType;
    fn sampler_node_debug(&self) -> &Option<LockedTypeObject /* StepLogicSamplerNodeDebugData */>;
    fn sampler_node_debug_mut(&mut self) -> &mut Option<LockedTypeObject /* StepLogicSamplerNodeDebugData */>;
}

impl StepLogicSamplerNodeDataTrait for StepLogicSamplerNodeData {
    fn external_wave(&self) -> &super::audio::AudioGraphNodePort {
        &self.external_wave
    }
    fn external_wave_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.external_wave
    }
    fn pitch(&self) -> &super::audio::AudioGraphNodePort {
        &self.pitch
    }
    fn pitch_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.pitch
    }
    fn amplitude(&self) -> &super::audio::AudioGraphNodePort {
        &self.amplitude
    }
    fn amplitude_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.amplitude
    }
    fn rate_of_fire(&self) -> &super::audio::AudioGraphNodePort {
        &self.rate_of_fire
    }
    fn rate_of_fire_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.rate_of_fire
    }
    fn trigger(&self) -> &super::audio::AudioGraphNodePort {
        &self.trigger
    }
    fn trigger_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.trigger
    }
    fn release(&self) -> &super::audio::AudioGraphNodePort {
        &self.release
    }
    fn release_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.release
    }
    fn output(&self) -> &super::audio::AudioGraphNodePort {
        &self.output
    }
    fn output_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.output
    }
    fn finished(&self) -> &super::audio::AudioGraphNodePort {
        &self.finished
    }
    fn finished_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.finished
    }
    fn wave(&self) -> &Option<LockedTypeObject /* super::audio::SoundWaveAssetBase */> {
        &self.wave
    }
    fn wave_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundWaveAssetBase */> {
        &mut self.wave
    }
    fn plugins(&self) -> &Vec<BoxedTypeObject /* StepLogicSamplerPlugins */> {
        &self.plugins
    }
    fn plugins_mut(&mut self) -> &mut Vec<BoxedTypeObject /* StepLogicSamplerPlugins */> {
        &mut self.plugins
    }
    fn pitch_source(&self) -> &Option<LockedTypeObject /* super::audio::OutputNodeData */> {
        &self.pitch_source
    }
    fn pitch_source_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::OutputNodeData */> {
        &mut self.pitch_source
    }
    fn maximum_rate_of_fire(&self) -> &f32 {
        &self.maximum_rate_of_fire
    }
    fn maximum_rate_of_fire_mut(&mut self) -> &mut f32 {
        &mut self.maximum_rate_of_fire
    }
    fn fade_type(&self) -> &super::audio::GainFaderFadeType {
        &self.fade_type
    }
    fn fade_type_mut(&mut self) -> &mut super::audio::GainFaderFadeType {
        &mut self.fade_type
    }
    fn sampler_node_debug(&self) -> &Option<LockedTypeObject /* StepLogicSamplerNodeDebugData */> {
        &self.sampler_node_debug
    }
    fn sampler_node_debug_mut(&mut self) -> &mut Option<LockedTypeObject /* StepLogicSamplerNodeDebugData */> {
        &mut self.sampler_node_debug
    }
}

impl super::audio::AudioGraphNodeDataTrait for StepLogicSamplerNodeData {
}

impl super::core::DataContainerTrait for StepLogicSamplerNodeData {
}

pub static STEPLOGICSAMPLERNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StepLogicSamplerNodeData",
    name_hash: 4264281149,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        super_class_offset: offset_of!(StepLogicSamplerNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StepLogicSamplerNodeData as Default>::default())),
            create_boxed: || Box::new(<StepLogicSamplerNodeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ExternalWave",
                name_hash: 2162866621,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(StepLogicSamplerNodeData, external_wave),
            },
            FieldInfoData {
                name: "Pitch",
                name_hash: 232604323,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(StepLogicSamplerNodeData, pitch),
            },
            FieldInfoData {
                name: "Amplitude",
                name_hash: 698564572,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(StepLogicSamplerNodeData, amplitude),
            },
            FieldInfoData {
                name: "RateOfFire",
                name_hash: 3866082710,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(StepLogicSamplerNodeData, rate_of_fire),
            },
            FieldInfoData {
                name: "Trigger",
                name_hash: 2606354109,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(StepLogicSamplerNodeData, trigger),
            },
            FieldInfoData {
                name: "Release",
                name_hash: 1335266828,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(StepLogicSamplerNodeData, release),
            },
            FieldInfoData {
                name: "Output",
                name_hash: 2895736442,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(StepLogicSamplerNodeData, output),
            },
            FieldInfoData {
                name: "Finished",
                name_hash: 1223765815,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(StepLogicSamplerNodeData, finished),
            },
            FieldInfoData {
                name: "Wave",
                name_hash: 2089277184,
                flags: MemberInfoFlags::new(0),
                field_type: "SoundWaveAssetBase",
                rust_offset: offset_of!(StepLogicSamplerNodeData, wave),
            },
            FieldInfoData {
                name: "Plugins",
                name_hash: 14514271,
                flags: MemberInfoFlags::new(144),
                field_type: "StepLogicSamplerPlugins-Array",
                rust_offset: offset_of!(StepLogicSamplerNodeData, plugins),
            },
            FieldInfoData {
                name: "PitchSource",
                name_hash: 2033117566,
                flags: MemberInfoFlags::new(0),
                field_type: "OutputNodeData",
                rust_offset: offset_of!(StepLogicSamplerNodeData, pitch_source),
            },
            FieldInfoData {
                name: "MaximumRateOfFire",
                name_hash: 3281074206,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(StepLogicSamplerNodeData, maximum_rate_of_fire),
            },
            FieldInfoData {
                name: "FadeType",
                name_hash: 4001206363,
                flags: MemberInfoFlags::new(0),
                field_type: "GainFaderFadeType",
                rust_offset: offset_of!(StepLogicSamplerNodeData, fade_type),
            },
            FieldInfoData {
                name: "SamplerNodeDebug",
                name_hash: 298896992,
                flags: MemberInfoFlags::new(0),
                field_type: "StepLogicSamplerNodeDebugData",
                rust_offset: offset_of!(StepLogicSamplerNodeData, sampler_node_debug),
            },
        ],
    }),
    array_type: Some(STEPLOGICSAMPLERNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StepLogicSamplerNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        STEPLOGICSAMPLERNODEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static STEPLOGICSAMPLERNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StepLogicSamplerNodeData-Array",
    name_hash: 3361071241,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("StepLogicSamplerNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct StepLogicSamplerNodeDebugData {
    pub _glacier_base: super::core::DataContainer,
    pub enable_debug: bool,
    pub debug_text_x_pos: i32,
    pub debug_text_y_pos: i32,
    pub event_display_time: f32,
    pub sampler_debug_info_color: super::core::Vec3,
    pub properties_debug_info_color: super::core::Vec3,
    pub events_debug_info_color: super::core::Vec3,
    pub external_wave_debug_info_color: super::core::Vec3,
    pub mute_sampler: bool,
}

pub trait StepLogicSamplerNodeDebugDataTrait: super::core::DataContainerTrait {
    fn enable_debug(&self) -> &bool;
    fn enable_debug_mut(&mut self) -> &mut bool;
    fn debug_text_x_pos(&self) -> &i32;
    fn debug_text_x_pos_mut(&mut self) -> &mut i32;
    fn debug_text_y_pos(&self) -> &i32;
    fn debug_text_y_pos_mut(&mut self) -> &mut i32;
    fn event_display_time(&self) -> &f32;
    fn event_display_time_mut(&mut self) -> &mut f32;
    fn sampler_debug_info_color(&self) -> &super::core::Vec3;
    fn sampler_debug_info_color_mut(&mut self) -> &mut super::core::Vec3;
    fn properties_debug_info_color(&self) -> &super::core::Vec3;
    fn properties_debug_info_color_mut(&mut self) -> &mut super::core::Vec3;
    fn events_debug_info_color(&self) -> &super::core::Vec3;
    fn events_debug_info_color_mut(&mut self) -> &mut super::core::Vec3;
    fn external_wave_debug_info_color(&self) -> &super::core::Vec3;
    fn external_wave_debug_info_color_mut(&mut self) -> &mut super::core::Vec3;
    fn mute_sampler(&self) -> &bool;
    fn mute_sampler_mut(&mut self) -> &mut bool;
}

impl StepLogicSamplerNodeDebugDataTrait for StepLogicSamplerNodeDebugData {
    fn enable_debug(&self) -> &bool {
        &self.enable_debug
    }
    fn enable_debug_mut(&mut self) -> &mut bool {
        &mut self.enable_debug
    }
    fn debug_text_x_pos(&self) -> &i32 {
        &self.debug_text_x_pos
    }
    fn debug_text_x_pos_mut(&mut self) -> &mut i32 {
        &mut self.debug_text_x_pos
    }
    fn debug_text_y_pos(&self) -> &i32 {
        &self.debug_text_y_pos
    }
    fn debug_text_y_pos_mut(&mut self) -> &mut i32 {
        &mut self.debug_text_y_pos
    }
    fn event_display_time(&self) -> &f32 {
        &self.event_display_time
    }
    fn event_display_time_mut(&mut self) -> &mut f32 {
        &mut self.event_display_time
    }
    fn sampler_debug_info_color(&self) -> &super::core::Vec3 {
        &self.sampler_debug_info_color
    }
    fn sampler_debug_info_color_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.sampler_debug_info_color
    }
    fn properties_debug_info_color(&self) -> &super::core::Vec3 {
        &self.properties_debug_info_color
    }
    fn properties_debug_info_color_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.properties_debug_info_color
    }
    fn events_debug_info_color(&self) -> &super::core::Vec3 {
        &self.events_debug_info_color
    }
    fn events_debug_info_color_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.events_debug_info_color
    }
    fn external_wave_debug_info_color(&self) -> &super::core::Vec3 {
        &self.external_wave_debug_info_color
    }
    fn external_wave_debug_info_color_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.external_wave_debug_info_color
    }
    fn mute_sampler(&self) -> &bool {
        &self.mute_sampler
    }
    fn mute_sampler_mut(&mut self) -> &mut bool {
        &mut self.mute_sampler
    }
}

impl super::core::DataContainerTrait for StepLogicSamplerNodeDebugData {
}

pub static STEPLOGICSAMPLERNODEDEBUGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StepLogicSamplerNodeDebugData",
    name_hash: 3730871692,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(StepLogicSamplerNodeDebugData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StepLogicSamplerNodeDebugData as Default>::default())),
            create_boxed: || Box::new(<StepLogicSamplerNodeDebugData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "EnableDebug",
                name_hash: 634281781,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(StepLogicSamplerNodeDebugData, enable_debug),
            },
            FieldInfoData {
                name: "DebugTextXPos",
                name_hash: 3339242877,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(StepLogicSamplerNodeDebugData, debug_text_x_pos),
            },
            FieldInfoData {
                name: "DebugTextYPos",
                name_hash: 3339341980,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(StepLogicSamplerNodeDebugData, debug_text_y_pos),
            },
            FieldInfoData {
                name: "EventDisplayTime",
                name_hash: 830993990,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(StepLogicSamplerNodeDebugData, event_display_time),
            },
            FieldInfoData {
                name: "SamplerDebugInfoColor",
                name_hash: 3619838419,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(StepLogicSamplerNodeDebugData, sampler_debug_info_color),
            },
            FieldInfoData {
                name: "PropertiesDebugInfoColor",
                name_hash: 1983868326,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(StepLogicSamplerNodeDebugData, properties_debug_info_color),
            },
            FieldInfoData {
                name: "EventsDebugInfoColor",
                name_hash: 256006744,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(StepLogicSamplerNodeDebugData, events_debug_info_color),
            },
            FieldInfoData {
                name: "ExternalWaveDebugInfoColor",
                name_hash: 3326041183,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(StepLogicSamplerNodeDebugData, external_wave_debug_info_color),
            },
            FieldInfoData {
                name: "MuteSampler",
                name_hash: 2296045208,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(StepLogicSamplerNodeDebugData, mute_sampler),
            },
        ],
    }),
    array_type: Some(STEPLOGICSAMPLERNODEDEBUGDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for StepLogicSamplerNodeDebugData {
    fn type_info(&self) -> &'static TypeInfo {
        STEPLOGICSAMPLERNODEDEBUGDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static STEPLOGICSAMPLERNODEDEBUGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StepLogicSamplerNodeDebugData-Array",
    name_hash: 288908728,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("StepLogicSamplerNodeDebugData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct StepLogicSamplerPlugins {
    pub snd_player: super::audio::SoundGraphPluginRef,
    pub resample: super::audio::SoundGraphPluginRef,
    pub pause: super::audio::SoundGraphPluginRef,
    pub gain: super::audio::SoundGraphPluginRef,
    pub gain_fader: super::audio::SoundGraphPluginRef,
}

pub trait StepLogicSamplerPluginsTrait: TypeObject {
    fn snd_player(&self) -> &super::audio::SoundGraphPluginRef;
    fn snd_player_mut(&mut self) -> &mut super::audio::SoundGraphPluginRef;
    fn resample(&self) -> &super::audio::SoundGraphPluginRef;
    fn resample_mut(&mut self) -> &mut super::audio::SoundGraphPluginRef;
    fn pause(&self) -> &super::audio::SoundGraphPluginRef;
    fn pause_mut(&mut self) -> &mut super::audio::SoundGraphPluginRef;
    fn gain(&self) -> &super::audio::SoundGraphPluginRef;
    fn gain_mut(&mut self) -> &mut super::audio::SoundGraphPluginRef;
    fn gain_fader(&self) -> &super::audio::SoundGraphPluginRef;
    fn gain_fader_mut(&mut self) -> &mut super::audio::SoundGraphPluginRef;
}

impl StepLogicSamplerPluginsTrait for StepLogicSamplerPlugins {
    fn snd_player(&self) -> &super::audio::SoundGraphPluginRef {
        &self.snd_player
    }
    fn snd_player_mut(&mut self) -> &mut super::audio::SoundGraphPluginRef {
        &mut self.snd_player
    }
    fn resample(&self) -> &super::audio::SoundGraphPluginRef {
        &self.resample
    }
    fn resample_mut(&mut self) -> &mut super::audio::SoundGraphPluginRef {
        &mut self.resample
    }
    fn pause(&self) -> &super::audio::SoundGraphPluginRef {
        &self.pause
    }
    fn pause_mut(&mut self) -> &mut super::audio::SoundGraphPluginRef {
        &mut self.pause
    }
    fn gain(&self) -> &super::audio::SoundGraphPluginRef {
        &self.gain
    }
    fn gain_mut(&mut self) -> &mut super::audio::SoundGraphPluginRef {
        &mut self.gain
    }
    fn gain_fader(&self) -> &super::audio::SoundGraphPluginRef {
        &self.gain_fader
    }
    fn gain_fader_mut(&mut self) -> &mut super::audio::SoundGraphPluginRef {
        &mut self.gain_fader
    }
}

pub static STEPLOGICSAMPLERPLUGINS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StepLogicSamplerPlugins",
    name_hash: 217050807,
    flags: MemberInfoFlags::new(36937),
    module: "DiceCommonsShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StepLogicSamplerPlugins as Default>::default())),
            create_boxed: || Box::new(<StepLogicSamplerPlugins as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "SndPlayer",
                name_hash: 728257487,
                flags: MemberInfoFlags::new(0),
                field_type: "SoundGraphPluginRef",
                rust_offset: offset_of!(StepLogicSamplerPlugins, snd_player),
            },
            FieldInfoData {
                name: "Resample",
                name_hash: 53347764,
                flags: MemberInfoFlags::new(0),
                field_type: "SoundGraphPluginRef",
                rust_offset: offset_of!(StepLogicSamplerPlugins, resample),
            },
            FieldInfoData {
                name: "Pause",
                name_hash: 232316407,
                flags: MemberInfoFlags::new(0),
                field_type: "SoundGraphPluginRef",
                rust_offset: offset_of!(StepLogicSamplerPlugins, pause),
            },
            FieldInfoData {
                name: "Gain",
                name_hash: 2088703076,
                flags: MemberInfoFlags::new(0),
                field_type: "SoundGraphPluginRef",
                rust_offset: offset_of!(StepLogicSamplerPlugins, gain),
            },
            FieldInfoData {
                name: "GainFader",
                name_hash: 2943317296,
                flags: MemberInfoFlags::new(0),
                field_type: "SoundGraphPluginRef",
                rust_offset: offset_of!(StepLogicSamplerPlugins, gain_fader),
            },
        ],
    }),
    array_type: Some(STEPLOGICSAMPLERPLUGINS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for StepLogicSamplerPlugins {
    fn type_info(&self) -> &'static TypeInfo {
        STEPLOGICSAMPLERPLUGINS_TYPE_INFO
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


pub static STEPLOGICSAMPLERPLUGINS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StepLogicSamplerPlugins-Array",
    name_hash: 3169388547,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("StepLogicSamplerPlugins"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SortValuesNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
    pub sort_values: Vec<Option<LockedTypeObject /* SortValuesGroup */>>,
}

pub trait SortValuesNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
    fn sort_values(&self) -> &Vec<Option<LockedTypeObject /* SortValuesGroup */>>;
    fn sort_values_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* SortValuesGroup */>>;
}

impl SortValuesNodeDataTrait for SortValuesNodeData {
    fn sort_values(&self) -> &Vec<Option<LockedTypeObject /* SortValuesGroup */>> {
        &self.sort_values
    }
    fn sort_values_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* SortValuesGroup */>> {
        &mut self.sort_values
    }
}

impl super::audio::AudioGraphNodeDataTrait for SortValuesNodeData {
}

impl super::core::DataContainerTrait for SortValuesNodeData {
}

pub static SORTVALUESNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SortValuesNodeData",
    name_hash: 3112727191,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        super_class_offset: offset_of!(SortValuesNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SortValuesNodeData as Default>::default())),
            create_boxed: || Box::new(<SortValuesNodeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "SortValues",
                name_hash: 1083437895,
                flags: MemberInfoFlags::new(144),
                field_type: "SortValuesGroup-Array",
                rust_offset: offset_of!(SortValuesNodeData, sort_values),
            },
        ],
    }),
    array_type: Some(SORTVALUESNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SortValuesNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        SORTVALUESNODEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SORTVALUESNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SortValuesNodeData-Array",
    name_hash: 1462287139,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("SortValuesNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SortValuesGroup {
    pub _glacier_base: super::audio::AudioGraphNodePortGroup,
    pub r#in: super::audio::AudioGraphNodePort,
    pub sorted_value: super::audio::AudioGraphNodePort,
    pub input_index: super::audio::AudioGraphNodePort,
}

pub trait SortValuesGroupTrait: super::audio::AudioGraphNodePortGroupTrait {
    fn r#in(&self) -> &super::audio::AudioGraphNodePort;
    fn r#in_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn sorted_value(&self) -> &super::audio::AudioGraphNodePort;
    fn sorted_value_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn input_index(&self) -> &super::audio::AudioGraphNodePort;
    fn input_index_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
}

impl SortValuesGroupTrait for SortValuesGroup {
    fn r#in(&self) -> &super::audio::AudioGraphNodePort {
        &self.r#in
    }
    fn r#in_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.r#in
    }
    fn sorted_value(&self) -> &super::audio::AudioGraphNodePort {
        &self.sorted_value
    }
    fn sorted_value_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.sorted_value
    }
    fn input_index(&self) -> &super::audio::AudioGraphNodePort {
        &self.input_index
    }
    fn input_index_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.input_index
    }
}

impl super::audio::AudioGraphNodePortGroupTrait for SortValuesGroup {
}

impl super::core::DataContainerTrait for SortValuesGroup {
}

pub static SORTVALUESGROUP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SortValuesGroup",
    name_hash: 327050968,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEPORTGROUP_TYPE_INFO),
        super_class_offset: offset_of!(SortValuesGroup, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SortValuesGroup as Default>::default())),
            create_boxed: || Box::new(<SortValuesGroup as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "In",
                name_hash: 5862146,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(SortValuesGroup, r#in),
            },
            FieldInfoData {
                name: "SortedValue",
                name_hash: 3558803253,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(SortValuesGroup, sorted_value),
            },
            FieldInfoData {
                name: "InputIndex",
                name_hash: 1613850061,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(SortValuesGroup, input_index),
            },
        ],
    }),
    array_type: Some(SORTVALUESGROUP_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SortValuesGroup {
    fn type_info(&self) -> &'static TypeInfo {
        SORTVALUESGROUP_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SORTVALUESGROUP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SortValuesGroup-Array",
    name_hash: 693453164,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("SortValuesGroup"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SequencerNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
    pub start: super::audio::AudioGraphNodePort,
    pub stop: super::audio::AudioGraphNodePort,
    pub time: super::audio::AudioGraphNodePort,
    pub on_started: super::audio::AudioGraphNodePort,
    pub on_stopped: super::audio::AudioGraphNodePort,
    pub out: super::audio::AudioGraphNodePort,
    pub delay: super::audio::AudioGraphNodePort,
    pub time_mode: TimeMode,
}

pub trait SequencerNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
    fn start(&self) -> &super::audio::AudioGraphNodePort;
    fn start_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn stop(&self) -> &super::audio::AudioGraphNodePort;
    fn stop_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn time(&self) -> &super::audio::AudioGraphNodePort;
    fn time_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn on_started(&self) -> &super::audio::AudioGraphNodePort;
    fn on_started_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn on_stopped(&self) -> &super::audio::AudioGraphNodePort;
    fn on_stopped_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn out(&self) -> &super::audio::AudioGraphNodePort;
    fn out_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn delay(&self) -> &super::audio::AudioGraphNodePort;
    fn delay_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn time_mode(&self) -> &TimeMode;
    fn time_mode_mut(&mut self) -> &mut TimeMode;
}

impl SequencerNodeDataTrait for SequencerNodeData {
    fn start(&self) -> &super::audio::AudioGraphNodePort {
        &self.start
    }
    fn start_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.start
    }
    fn stop(&self) -> &super::audio::AudioGraphNodePort {
        &self.stop
    }
    fn stop_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.stop
    }
    fn time(&self) -> &super::audio::AudioGraphNodePort {
        &self.time
    }
    fn time_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.time
    }
    fn on_started(&self) -> &super::audio::AudioGraphNodePort {
        &self.on_started
    }
    fn on_started_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.on_started
    }
    fn on_stopped(&self) -> &super::audio::AudioGraphNodePort {
        &self.on_stopped
    }
    fn on_stopped_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.on_stopped
    }
    fn out(&self) -> &super::audio::AudioGraphNodePort {
        &self.out
    }
    fn out_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.out
    }
    fn delay(&self) -> &super::audio::AudioGraphNodePort {
        &self.delay
    }
    fn delay_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.delay
    }
    fn time_mode(&self) -> &TimeMode {
        &self.time_mode
    }
    fn time_mode_mut(&mut self) -> &mut TimeMode {
        &mut self.time_mode
    }
}

impl super::audio::AudioGraphNodeDataTrait for SequencerNodeData {
}

impl super::core::DataContainerTrait for SequencerNodeData {
}

pub static SEQUENCERNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SequencerNodeData",
    name_hash: 359611352,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        super_class_offset: offset_of!(SequencerNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SequencerNodeData as Default>::default())),
            create_boxed: || Box::new(<SequencerNodeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Start",
                name_hash: 230748069,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(SequencerNodeData, start),
            },
            FieldInfoData {
                name: "Stop",
                name_hash: 2089401213,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(SequencerNodeData, stop),
            },
            FieldInfoData {
                name: "Time",
                name_hash: 2089313744,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(SequencerNodeData, time),
            },
            FieldInfoData {
                name: "OnStarted",
                name_hash: 3101601957,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(SequencerNodeData, on_started),
            },
            FieldInfoData {
                name: "OnStopped",
                name_hash: 3117993581,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(SequencerNodeData, on_stopped),
            },
            FieldInfoData {
                name: "Out",
                name_hash: 193453899,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(SequencerNodeData, out),
            },
            FieldInfoData {
                name: "Delay",
                name_hash: 208768368,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(SequencerNodeData, delay),
            },
            FieldInfoData {
                name: "TimeMode",
                name_hash: 2999663827,
                flags: MemberInfoFlags::new(0),
                field_type: "TimeMode",
                rust_offset: offset_of!(SequencerNodeData, time_mode),
            },
        ],
    }),
    array_type: Some(SEQUENCERNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SequencerNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        SEQUENCERNODEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SEQUENCERNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SequencerNodeData-Array",
    name_hash: 332519020,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("SequencerNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum TimeMode {
    #[default]
    TimeMode_Frequency = 0,
    TimeMode_Period = 1,
}

pub static TIMEMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimeMode",
    name_hash: 2999663827,
    flags: MemberInfoFlags::new(49429),
    module: "DiceCommonsShared",
    data: TypeInfoData::Enum,
    array_type: Some(TIMEMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TimeMode {
    fn type_info(&self) -> &'static TypeInfo {
        TIMEMODE_TYPE_INFO
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


pub static TIMEMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimeMode-Array",
    name_hash: 1104824295,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("TimeMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SaturationNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
    pub r#in: super::audio::AudioGraphNodePort,
    pub gain: super::audio::AudioGraphNodePort,
    pub d_c_bias: super::audio::AudioGraphNodePort,
    pub level: super::audio::AudioGraphNodePort,
    pub mix: super::audio::AudioGraphNodePort,
    pub saturation_type: SaturationTypes,
    pub out: super::audio::AudioGraphNodePort,
    pub saturation_plugin: super::audio::SoundGraphPluginRef,
}

pub trait SaturationNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
    fn r#in(&self) -> &super::audio::AudioGraphNodePort;
    fn r#in_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn gain(&self) -> &super::audio::AudioGraphNodePort;
    fn gain_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn d_c_bias(&self) -> &super::audio::AudioGraphNodePort;
    fn d_c_bias_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn level(&self) -> &super::audio::AudioGraphNodePort;
    fn level_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn mix(&self) -> &super::audio::AudioGraphNodePort;
    fn mix_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn saturation_type(&self) -> &SaturationTypes;
    fn saturation_type_mut(&mut self) -> &mut SaturationTypes;
    fn out(&self) -> &super::audio::AudioGraphNodePort;
    fn out_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn saturation_plugin(&self) -> &super::audio::SoundGraphPluginRef;
    fn saturation_plugin_mut(&mut self) -> &mut super::audio::SoundGraphPluginRef;
}

impl SaturationNodeDataTrait for SaturationNodeData {
    fn r#in(&self) -> &super::audio::AudioGraphNodePort {
        &self.r#in
    }
    fn r#in_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.r#in
    }
    fn gain(&self) -> &super::audio::AudioGraphNodePort {
        &self.gain
    }
    fn gain_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.gain
    }
    fn d_c_bias(&self) -> &super::audio::AudioGraphNodePort {
        &self.d_c_bias
    }
    fn d_c_bias_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.d_c_bias
    }
    fn level(&self) -> &super::audio::AudioGraphNodePort {
        &self.level
    }
    fn level_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.level
    }
    fn mix(&self) -> &super::audio::AudioGraphNodePort {
        &self.mix
    }
    fn mix_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.mix
    }
    fn saturation_type(&self) -> &SaturationTypes {
        &self.saturation_type
    }
    fn saturation_type_mut(&mut self) -> &mut SaturationTypes {
        &mut self.saturation_type
    }
    fn out(&self) -> &super::audio::AudioGraphNodePort {
        &self.out
    }
    fn out_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.out
    }
    fn saturation_plugin(&self) -> &super::audio::SoundGraphPluginRef {
        &self.saturation_plugin
    }
    fn saturation_plugin_mut(&mut self) -> &mut super::audio::SoundGraphPluginRef {
        &mut self.saturation_plugin
    }
}

impl super::audio::AudioGraphNodeDataTrait for SaturationNodeData {
}

impl super::core::DataContainerTrait for SaturationNodeData {
}

pub static SATURATIONNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SaturationNodeData",
    name_hash: 2739493865,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        super_class_offset: offset_of!(SaturationNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SaturationNodeData as Default>::default())),
            create_boxed: || Box::new(<SaturationNodeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "In",
                name_hash: 5862146,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(SaturationNodeData, r#in),
            },
            FieldInfoData {
                name: "Gain",
                name_hash: 2088703076,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(SaturationNodeData, gain),
            },
            FieldInfoData {
                name: "DCBias",
                name_hash: 2559466715,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(SaturationNodeData, d_c_bias),
            },
            FieldInfoData {
                name: "Level",
                name_hash: 218262515,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(SaturationNodeData, level),
            },
            FieldInfoData {
                name: "Mix",
                name_hash: 193446617,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(SaturationNodeData, mix),
            },
            FieldInfoData {
                name: "SaturationType",
                name_hash: 2558972865,
                flags: MemberInfoFlags::new(0),
                field_type: "SaturationTypes",
                rust_offset: offset_of!(SaturationNodeData, saturation_type),
            },
            FieldInfoData {
                name: "Out",
                name_hash: 193453899,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(SaturationNodeData, out),
            },
            FieldInfoData {
                name: "SaturationPlugin",
                name_hash: 3411805456,
                flags: MemberInfoFlags::new(0),
                field_type: "SoundGraphPluginRef",
                rust_offset: offset_of!(SaturationNodeData, saturation_plugin),
            },
        ],
    }),
    array_type: Some(SATURATIONNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SaturationNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        SATURATIONNODEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SATURATIONNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SaturationNodeData-Array",
    name_hash: 2100564701,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("SaturationNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum SaturationTypes {
    #[default]
    SaturationTypes_Tape = 0,
    SaturationTypes_Tape2HQ = 1,
    SaturationTypes_Overdrive = 2,
    SaturationTypes_Tube = 3,
    SaturationTypes_Tube2HQ = 4,
    SaturationTypes_FuzzHQ = 5,
    SaturationTypes_Distortion = 6,
    SaturationTypes_Metal = 7,
    SaturationTypes_Foldback = 8,
    SaturationTypes_HalfWaveRectifier = 9,
    SaturationTypes_FullWaveRectifier = 10,
}

pub static SATURATIONTYPES_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SaturationTypes",
    name_hash: 2841725842,
    flags: MemberInfoFlags::new(49429),
    module: "DiceCommonsShared",
    data: TypeInfoData::Enum,
    array_type: Some(SATURATIONTYPES_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SaturationTypes {
    fn type_info(&self) -> &'static TypeInfo {
        SATURATIONTYPES_TYPE_INFO
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


pub static SATURATIONTYPES_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SaturationTypes-Array",
    name_hash: 149525158,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("SaturationTypes"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct RunOnceNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
    pub r#in: super::audio::AudioGraphNodePort,
    pub reset: super::audio::AudioGraphNodePort,
    pub out: super::audio::AudioGraphNodePort,
    pub start_closed: bool,
}

pub trait RunOnceNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
    fn r#in(&self) -> &super::audio::AudioGraphNodePort;
    fn r#in_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn reset(&self) -> &super::audio::AudioGraphNodePort;
    fn reset_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn out(&self) -> &super::audio::AudioGraphNodePort;
    fn out_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn start_closed(&self) -> &bool;
    fn start_closed_mut(&mut self) -> &mut bool;
}

impl RunOnceNodeDataTrait for RunOnceNodeData {
    fn r#in(&self) -> &super::audio::AudioGraphNodePort {
        &self.r#in
    }
    fn r#in_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.r#in
    }
    fn reset(&self) -> &super::audio::AudioGraphNodePort {
        &self.reset
    }
    fn reset_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.reset
    }
    fn out(&self) -> &super::audio::AudioGraphNodePort {
        &self.out
    }
    fn out_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.out
    }
    fn start_closed(&self) -> &bool {
        &self.start_closed
    }
    fn start_closed_mut(&mut self) -> &mut bool {
        &mut self.start_closed
    }
}

impl super::audio::AudioGraphNodeDataTrait for RunOnceNodeData {
}

impl super::core::DataContainerTrait for RunOnceNodeData {
}

pub static RUNONCENODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RunOnceNodeData",
    name_hash: 3521389307,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        super_class_offset: offset_of!(RunOnceNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RunOnceNodeData as Default>::default())),
            create_boxed: || Box::new(<RunOnceNodeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "In",
                name_hash: 5862146,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(RunOnceNodeData, r#in),
            },
            FieldInfoData {
                name: "Reset",
                name_hash: 229946160,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(RunOnceNodeData, reset),
            },
            FieldInfoData {
                name: "Out",
                name_hash: 193453899,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(RunOnceNodeData, out),
            },
            FieldInfoData {
                name: "StartClosed",
                name_hash: 77276087,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RunOnceNodeData, start_closed),
            },
        ],
    }),
    array_type: Some(RUNONCENODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RunOnceNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        RUNONCENODEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static RUNONCENODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RunOnceNodeData-Array",
    name_hash: 2781289167,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("RunOnceNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct RaycastNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
    pub send: super::audio::AudioGraphNodePort,
    pub hit: super::audio::AudioGraphNodePort,
    pub miss: super::audio::AudioGraphNodePort,
    pub left: f32,
    pub up: f32,
    pub forward: f32,
    pub see_through: bool,
    pub penetrable: bool,
    pub include_terrain: bool,
    pub include_water: bool,
    pub include_characters: bool,
    pub include_vehicles: bool,
    pub include_ragdolls: bool,
    pub include_fixed: bool,
    pub include_keyframed: bool,
    pub include_dynamic: bool,
    pub detailed_query_mode: bool,
    pub enable_debug: bool,
    pub material_id: super::audio::AudioGraphNodePort,
}

pub trait RaycastNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
    fn send(&self) -> &super::audio::AudioGraphNodePort;
    fn send_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn hit(&self) -> &super::audio::AudioGraphNodePort;
    fn hit_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn miss(&self) -> &super::audio::AudioGraphNodePort;
    fn miss_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn left(&self) -> &f32;
    fn left_mut(&mut self) -> &mut f32;
    fn up(&self) -> &f32;
    fn up_mut(&mut self) -> &mut f32;
    fn forward(&self) -> &f32;
    fn forward_mut(&mut self) -> &mut f32;
    fn see_through(&self) -> &bool;
    fn see_through_mut(&mut self) -> &mut bool;
    fn penetrable(&self) -> &bool;
    fn penetrable_mut(&mut self) -> &mut bool;
    fn include_terrain(&self) -> &bool;
    fn include_terrain_mut(&mut self) -> &mut bool;
    fn include_water(&self) -> &bool;
    fn include_water_mut(&mut self) -> &mut bool;
    fn include_characters(&self) -> &bool;
    fn include_characters_mut(&mut self) -> &mut bool;
    fn include_vehicles(&self) -> &bool;
    fn include_vehicles_mut(&mut self) -> &mut bool;
    fn include_ragdolls(&self) -> &bool;
    fn include_ragdolls_mut(&mut self) -> &mut bool;
    fn include_fixed(&self) -> &bool;
    fn include_fixed_mut(&mut self) -> &mut bool;
    fn include_keyframed(&self) -> &bool;
    fn include_keyframed_mut(&mut self) -> &mut bool;
    fn include_dynamic(&self) -> &bool;
    fn include_dynamic_mut(&mut self) -> &mut bool;
    fn detailed_query_mode(&self) -> &bool;
    fn detailed_query_mode_mut(&mut self) -> &mut bool;
    fn enable_debug(&self) -> &bool;
    fn enable_debug_mut(&mut self) -> &mut bool;
    fn material_id(&self) -> &super::audio::AudioGraphNodePort;
    fn material_id_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
}

impl RaycastNodeDataTrait for RaycastNodeData {
    fn send(&self) -> &super::audio::AudioGraphNodePort {
        &self.send
    }
    fn send_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.send
    }
    fn hit(&self) -> &super::audio::AudioGraphNodePort {
        &self.hit
    }
    fn hit_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.hit
    }
    fn miss(&self) -> &super::audio::AudioGraphNodePort {
        &self.miss
    }
    fn miss_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.miss
    }
    fn left(&self) -> &f32 {
        &self.left
    }
    fn left_mut(&mut self) -> &mut f32 {
        &mut self.left
    }
    fn up(&self) -> &f32 {
        &self.up
    }
    fn up_mut(&mut self) -> &mut f32 {
        &mut self.up
    }
    fn forward(&self) -> &f32 {
        &self.forward
    }
    fn forward_mut(&mut self) -> &mut f32 {
        &mut self.forward
    }
    fn see_through(&self) -> &bool {
        &self.see_through
    }
    fn see_through_mut(&mut self) -> &mut bool {
        &mut self.see_through
    }
    fn penetrable(&self) -> &bool {
        &self.penetrable
    }
    fn penetrable_mut(&mut self) -> &mut bool {
        &mut self.penetrable
    }
    fn include_terrain(&self) -> &bool {
        &self.include_terrain
    }
    fn include_terrain_mut(&mut self) -> &mut bool {
        &mut self.include_terrain
    }
    fn include_water(&self) -> &bool {
        &self.include_water
    }
    fn include_water_mut(&mut self) -> &mut bool {
        &mut self.include_water
    }
    fn include_characters(&self) -> &bool {
        &self.include_characters
    }
    fn include_characters_mut(&mut self) -> &mut bool {
        &mut self.include_characters
    }
    fn include_vehicles(&self) -> &bool {
        &self.include_vehicles
    }
    fn include_vehicles_mut(&mut self) -> &mut bool {
        &mut self.include_vehicles
    }
    fn include_ragdolls(&self) -> &bool {
        &self.include_ragdolls
    }
    fn include_ragdolls_mut(&mut self) -> &mut bool {
        &mut self.include_ragdolls
    }
    fn include_fixed(&self) -> &bool {
        &self.include_fixed
    }
    fn include_fixed_mut(&mut self) -> &mut bool {
        &mut self.include_fixed
    }
    fn include_keyframed(&self) -> &bool {
        &self.include_keyframed
    }
    fn include_keyframed_mut(&mut self) -> &mut bool {
        &mut self.include_keyframed
    }
    fn include_dynamic(&self) -> &bool {
        &self.include_dynamic
    }
    fn include_dynamic_mut(&mut self) -> &mut bool {
        &mut self.include_dynamic
    }
    fn detailed_query_mode(&self) -> &bool {
        &self.detailed_query_mode
    }
    fn detailed_query_mode_mut(&mut self) -> &mut bool {
        &mut self.detailed_query_mode
    }
    fn enable_debug(&self) -> &bool {
        &self.enable_debug
    }
    fn enable_debug_mut(&mut self) -> &mut bool {
        &mut self.enable_debug
    }
    fn material_id(&self) -> &super::audio::AudioGraphNodePort {
        &self.material_id
    }
    fn material_id_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.material_id
    }
}

impl super::audio::AudioGraphNodeDataTrait for RaycastNodeData {
}

impl super::core::DataContainerTrait for RaycastNodeData {
}

pub static RAYCASTNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RaycastNodeData",
    name_hash: 352526490,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        super_class_offset: offset_of!(RaycastNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RaycastNodeData as Default>::default())),
            create_boxed: || Box::new(<RaycastNodeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Send",
                name_hash: 2089417369,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(RaycastNodeData, send),
            },
            FieldInfoData {
                name: "Hit",
                name_hash: 193458192,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(RaycastNodeData, hit),
            },
            FieldInfoData {
                name: "Miss",
                name_hash: 2088770913,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(RaycastNodeData, miss),
            },
            FieldInfoData {
                name: "Left",
                name_hash: 2089021886,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RaycastNodeData, left),
            },
            FieldInfoData {
                name: "Up",
                name_hash: 5862272,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RaycastNodeData, up),
            },
            FieldInfoData {
                name: "Forward",
                name_hash: 1986470206,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RaycastNodeData, forward),
            },
            FieldInfoData {
                name: "SeeThrough",
                name_hash: 753481485,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RaycastNodeData, see_through),
            },
            FieldInfoData {
                name: "Penetrable",
                name_hash: 1887765847,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RaycastNodeData, penetrable),
            },
            FieldInfoData {
                name: "IncludeTerrain",
                name_hash: 106414606,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RaycastNodeData, include_terrain),
            },
            FieldInfoData {
                name: "IncludeWater",
                name_hash: 796013708,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RaycastNodeData, include_water),
            },
            FieldInfoData {
                name: "IncludeCharacters",
                name_hash: 3133666035,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RaycastNodeData, include_characters),
            },
            FieldInfoData {
                name: "IncludeVehicles",
                name_hash: 4052507474,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RaycastNodeData, include_vehicles),
            },
            FieldInfoData {
                name: "IncludeRagdolls",
                name_hash: 2446437525,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RaycastNodeData, include_ragdolls),
            },
            FieldInfoData {
                name: "IncludeFixed",
                name_hash: 813351567,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RaycastNodeData, include_fixed),
            },
            FieldInfoData {
                name: "IncludeKeyframed",
                name_hash: 149633399,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RaycastNodeData, include_keyframed),
            },
            FieldInfoData {
                name: "IncludeDynamic",
                name_hash: 2793557740,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RaycastNodeData, include_dynamic),
            },
            FieldInfoData {
                name: "DetailedQueryMode",
                name_hash: 2780273852,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RaycastNodeData, detailed_query_mode),
            },
            FieldInfoData {
                name: "EnableDebug",
                name_hash: 634281781,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RaycastNodeData, enable_debug),
            },
            FieldInfoData {
                name: "MaterialId",
                name_hash: 1778871203,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(RaycastNodeData, material_id),
            },
        ],
    }),
    array_type: Some(RAYCASTNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RaycastNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        RAYCASTNODEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static RAYCASTNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RaycastNodeData-Array",
    name_hash: 1616282542,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("RaycastNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct RandomIntegerNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
    pub trigger: super::audio::AudioGraphNodePort,
    pub out: super::audio::AudioGraphNodePort,
    pub min: i32,
    pub max: i32,
}

pub trait RandomIntegerNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
    fn trigger(&self) -> &super::audio::AudioGraphNodePort;
    fn trigger_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn out(&self) -> &super::audio::AudioGraphNodePort;
    fn out_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn min(&self) -> &i32;
    fn min_mut(&mut self) -> &mut i32;
    fn max(&self) -> &i32;
    fn max_mut(&mut self) -> &mut i32;
}

impl RandomIntegerNodeDataTrait for RandomIntegerNodeData {
    fn trigger(&self) -> &super::audio::AudioGraphNodePort {
        &self.trigger
    }
    fn trigger_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.trigger
    }
    fn out(&self) -> &super::audio::AudioGraphNodePort {
        &self.out
    }
    fn out_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.out
    }
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
}

impl super::audio::AudioGraphNodeDataTrait for RandomIntegerNodeData {
}

impl super::core::DataContainerTrait for RandomIntegerNodeData {
}

pub static RANDOMINTEGERNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomIntegerNodeData",
    name_hash: 1413461704,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        super_class_offset: offset_of!(RandomIntegerNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RandomIntegerNodeData as Default>::default())),
            create_boxed: || Box::new(<RandomIntegerNodeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Trigger",
                name_hash: 2606354109,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(RandomIntegerNodeData, trigger),
            },
            FieldInfoData {
                name: "Out",
                name_hash: 193453899,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(RandomIntegerNodeData, out),
            },
            FieldInfoData {
                name: "Min",
                name_hash: 193446607,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(RandomIntegerNodeData, min),
            },
            FieldInfoData {
                name: "Max",
                name_hash: 193446865,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(RandomIntegerNodeData, max),
            },
        ],
    }),
    array_type: Some(RANDOMINTEGERNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RandomIntegerNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        RANDOMINTEGERNODEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static RANDOMINTEGERNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomIntegerNodeData-Array",
    name_hash: 2587871612,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("RandomIntegerNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ProfileOptionsReaderNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
    pub profile_options: Vec<Option<LockedTypeObject /* ProfileOptionsGroup */>>,
}

pub trait ProfileOptionsReaderNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
    fn profile_options(&self) -> &Vec<Option<LockedTypeObject /* ProfileOptionsGroup */>>;
    fn profile_options_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* ProfileOptionsGroup */>>;
}

impl ProfileOptionsReaderNodeDataTrait for ProfileOptionsReaderNodeData {
    fn profile_options(&self) -> &Vec<Option<LockedTypeObject /* ProfileOptionsGroup */>> {
        &self.profile_options
    }
    fn profile_options_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* ProfileOptionsGroup */>> {
        &mut self.profile_options
    }
}

impl super::audio::AudioGraphNodeDataTrait for ProfileOptionsReaderNodeData {
}

impl super::core::DataContainerTrait for ProfileOptionsReaderNodeData {
}

pub static PROFILEOPTIONSREADERNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionsReaderNodeData",
    name_hash: 637130603,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        super_class_offset: offset_of!(ProfileOptionsReaderNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ProfileOptionsReaderNodeData as Default>::default())),
            create_boxed: || Box::new(<ProfileOptionsReaderNodeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ProfileOptions",
                name_hash: 1140566110,
                flags: MemberInfoFlags::new(144),
                field_type: "ProfileOptionsGroup-Array",
                rust_offset: offset_of!(ProfileOptionsReaderNodeData, profile_options),
            },
        ],
    }),
    array_type: Some(PROFILEOPTIONSREADERNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProfileOptionsReaderNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        PROFILEOPTIONSREADERNODEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static PROFILEOPTIONSREADERNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionsReaderNodeData-Array",
    name_hash: 2514446175,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ProfileOptionsReaderNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ProfileOptionsGroup {
    pub _glacier_base: super::audio::AudioGraphNodePortGroup,
    pub profile_option_asset: Option<LockedTypeObject /* super::gameplay_sim::ProfileOptionData */>,
    pub profile_option_value: super::audio::AudioGraphNodePort,
}

pub trait ProfileOptionsGroupTrait: super::audio::AudioGraphNodePortGroupTrait {
    fn profile_option_asset(&self) -> &Option<LockedTypeObject /* super::gameplay_sim::ProfileOptionData */>;
    fn profile_option_asset_mut(&mut self) -> &mut Option<LockedTypeObject /* super::gameplay_sim::ProfileOptionData */>;
    fn profile_option_value(&self) -> &super::audio::AudioGraphNodePort;
    fn profile_option_value_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
}

impl ProfileOptionsGroupTrait for ProfileOptionsGroup {
    fn profile_option_asset(&self) -> &Option<LockedTypeObject /* super::gameplay_sim::ProfileOptionData */> {
        &self.profile_option_asset
    }
    fn profile_option_asset_mut(&mut self) -> &mut Option<LockedTypeObject /* super::gameplay_sim::ProfileOptionData */> {
        &mut self.profile_option_asset
    }
    fn profile_option_value(&self) -> &super::audio::AudioGraphNodePort {
        &self.profile_option_value
    }
    fn profile_option_value_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.profile_option_value
    }
}

impl super::audio::AudioGraphNodePortGroupTrait for ProfileOptionsGroup {
}

impl super::core::DataContainerTrait for ProfileOptionsGroup {
}

pub static PROFILEOPTIONSGROUP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionsGroup",
    name_hash: 3132571489,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEPORTGROUP_TYPE_INFO),
        super_class_offset: offset_of!(ProfileOptionsGroup, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ProfileOptionsGroup as Default>::default())),
            create_boxed: || Box::new(<ProfileOptionsGroup as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ProfileOptionAsset",
                name_hash: 2969954429,
                flags: MemberInfoFlags::new(0),
                field_type: "ProfileOptionData",
                rust_offset: offset_of!(ProfileOptionsGroup, profile_option_asset),
            },
            FieldInfoData {
                name: "ProfileOptionValue",
                name_hash: 2989336038,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ProfileOptionsGroup, profile_option_value),
            },
        ],
    }),
    array_type: Some(PROFILEOPTIONSGROUP_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProfileOptionsGroup {
    fn type_info(&self) -> &'static TypeInfo {
        PROFILEOPTIONSGROUP_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static PROFILEOPTIONSGROUP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionsGroup-Array",
    name_hash: 3504290389,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ProfileOptionsGroup"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PassbyDetectorNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
    pub time_to_apex: super::audio::AudioGraphNodePort,
    pub extra_distance: super::audio::AudioGraphNodePort,
    pub speed_threshold: super::audio::AudioGraphNodePort,
    pub relative_speed_threshold: super::audio::AudioGraphNodePort,
    pub cooldown_time: super::audio::AudioGraphNodePort,
    pub trigger: super::audio::AudioGraphNodePort,
    pub cancel: super::audio::AudioGraphNodePort,
    pub relative_speed_smoothing_rate: f32,
}

pub trait PassbyDetectorNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
    fn time_to_apex(&self) -> &super::audio::AudioGraphNodePort;
    fn time_to_apex_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn extra_distance(&self) -> &super::audio::AudioGraphNodePort;
    fn extra_distance_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn speed_threshold(&self) -> &super::audio::AudioGraphNodePort;
    fn speed_threshold_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn relative_speed_threshold(&self) -> &super::audio::AudioGraphNodePort;
    fn relative_speed_threshold_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn cooldown_time(&self) -> &super::audio::AudioGraphNodePort;
    fn cooldown_time_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn trigger(&self) -> &super::audio::AudioGraphNodePort;
    fn trigger_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn cancel(&self) -> &super::audio::AudioGraphNodePort;
    fn cancel_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn relative_speed_smoothing_rate(&self) -> &f32;
    fn relative_speed_smoothing_rate_mut(&mut self) -> &mut f32;
}

impl PassbyDetectorNodeDataTrait for PassbyDetectorNodeData {
    fn time_to_apex(&self) -> &super::audio::AudioGraphNodePort {
        &self.time_to_apex
    }
    fn time_to_apex_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.time_to_apex
    }
    fn extra_distance(&self) -> &super::audio::AudioGraphNodePort {
        &self.extra_distance
    }
    fn extra_distance_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.extra_distance
    }
    fn speed_threshold(&self) -> &super::audio::AudioGraphNodePort {
        &self.speed_threshold
    }
    fn speed_threshold_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.speed_threshold
    }
    fn relative_speed_threshold(&self) -> &super::audio::AudioGraphNodePort {
        &self.relative_speed_threshold
    }
    fn relative_speed_threshold_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.relative_speed_threshold
    }
    fn cooldown_time(&self) -> &super::audio::AudioGraphNodePort {
        &self.cooldown_time
    }
    fn cooldown_time_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.cooldown_time
    }
    fn trigger(&self) -> &super::audio::AudioGraphNodePort {
        &self.trigger
    }
    fn trigger_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.trigger
    }
    fn cancel(&self) -> &super::audio::AudioGraphNodePort {
        &self.cancel
    }
    fn cancel_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.cancel
    }
    fn relative_speed_smoothing_rate(&self) -> &f32 {
        &self.relative_speed_smoothing_rate
    }
    fn relative_speed_smoothing_rate_mut(&mut self) -> &mut f32 {
        &mut self.relative_speed_smoothing_rate
    }
}

impl super::audio::AudioGraphNodeDataTrait for PassbyDetectorNodeData {
}

impl super::core::DataContainerTrait for PassbyDetectorNodeData {
}

pub static PASSBYDETECTORNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PassbyDetectorNodeData",
    name_hash: 4169749605,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        super_class_offset: offset_of!(PassbyDetectorNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PassbyDetectorNodeData as Default>::default())),
            create_boxed: || Box::new(<PassbyDetectorNodeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "TimeToApex",
                name_hash: 1571641607,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(PassbyDetectorNodeData, time_to_apex),
            },
            FieldInfoData {
                name: "ExtraDistance",
                name_hash: 1781032892,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(PassbyDetectorNodeData, extra_distance),
            },
            FieldInfoData {
                name: "SpeedThreshold",
                name_hash: 1215144053,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(PassbyDetectorNodeData, speed_threshold),
            },
            FieldInfoData {
                name: "RelativeSpeedThreshold",
                name_hash: 2814872897,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(PassbyDetectorNodeData, relative_speed_threshold),
            },
            FieldInfoData {
                name: "CooldownTime",
                name_hash: 613462861,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(PassbyDetectorNodeData, cooldown_time),
            },
            FieldInfoData {
                name: "Trigger",
                name_hash: 2606354109,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(PassbyDetectorNodeData, trigger),
            },
            FieldInfoData {
                name: "Cancel",
                name_hash: 2716109795,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(PassbyDetectorNodeData, cancel),
            },
            FieldInfoData {
                name: "RelativeSpeedSmoothingRate",
                name_hash: 342726486,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PassbyDetectorNodeData, relative_speed_smoothing_rate),
            },
        ],
    }),
    array_type: Some(PASSBYDETECTORNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PassbyDetectorNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        PASSBYDETECTORNODEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static PASSBYDETECTORNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PassbyDetectorNodeData-Array",
    name_hash: 1282194769,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("PassbyDetectorNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ParameterSmootherNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
    pub r#in: super::audio::AudioGraphNodePort,
    pub out: super::audio::AudioGraphNodePort,
    pub smoothing_time: super::audio::AudioGraphNodePort,
}

pub trait ParameterSmootherNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
    fn r#in(&self) -> &super::audio::AudioGraphNodePort;
    fn r#in_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn out(&self) -> &super::audio::AudioGraphNodePort;
    fn out_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn smoothing_time(&self) -> &super::audio::AudioGraphNodePort;
    fn smoothing_time_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
}

impl ParameterSmootherNodeDataTrait for ParameterSmootherNodeData {
    fn r#in(&self) -> &super::audio::AudioGraphNodePort {
        &self.r#in
    }
    fn r#in_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.r#in
    }
    fn out(&self) -> &super::audio::AudioGraphNodePort {
        &self.out
    }
    fn out_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.out
    }
    fn smoothing_time(&self) -> &super::audio::AudioGraphNodePort {
        &self.smoothing_time
    }
    fn smoothing_time_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.smoothing_time
    }
}

impl super::audio::AudioGraphNodeDataTrait for ParameterSmootherNodeData {
}

impl super::core::DataContainerTrait for ParameterSmootherNodeData {
}

pub static PARAMETERSMOOTHERNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ParameterSmootherNodeData",
    name_hash: 2858087593,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        super_class_offset: offset_of!(ParameterSmootherNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ParameterSmootherNodeData as Default>::default())),
            create_boxed: || Box::new(<ParameterSmootherNodeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "In",
                name_hash: 5862146,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ParameterSmootherNodeData, r#in),
            },
            FieldInfoData {
                name: "Out",
                name_hash: 193453899,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ParameterSmootherNodeData, out),
            },
            FieldInfoData {
                name: "SmoothingTime",
                name_hash: 2916906290,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ParameterSmootherNodeData, smoothing_time),
            },
        ],
    }),
    array_type: Some(PARAMETERSMOOTHERNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ParameterSmootherNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        PARAMETERSMOOTHERNODEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static PARAMETERSMOOTHERNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ParameterSmootherNodeData-Array",
    name_hash: 326433565,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ParameterSmootherNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct NanCheckNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
    pub r#in: super::audio::AudioGraphNodePort,
    pub out: super::audio::AudioGraphNodePort,
    pub name_non_meta: String,
    pub nan_check_plugin: super::audio::SoundGraphPluginRef,
}

pub trait NanCheckNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
    fn r#in(&self) -> &super::audio::AudioGraphNodePort;
    fn r#in_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn out(&self) -> &super::audio::AudioGraphNodePort;
    fn out_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn name_non_meta(&self) -> &String;
    fn name_non_meta_mut(&mut self) -> &mut String;
    fn nan_check_plugin(&self) -> &super::audio::SoundGraphPluginRef;
    fn nan_check_plugin_mut(&mut self) -> &mut super::audio::SoundGraphPluginRef;
}

impl NanCheckNodeDataTrait for NanCheckNodeData {
    fn r#in(&self) -> &super::audio::AudioGraphNodePort {
        &self.r#in
    }
    fn r#in_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.r#in
    }
    fn out(&self) -> &super::audio::AudioGraphNodePort {
        &self.out
    }
    fn out_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.out
    }
    fn name_non_meta(&self) -> &String {
        &self.name_non_meta
    }
    fn name_non_meta_mut(&mut self) -> &mut String {
        &mut self.name_non_meta
    }
    fn nan_check_plugin(&self) -> &super::audio::SoundGraphPluginRef {
        &self.nan_check_plugin
    }
    fn nan_check_plugin_mut(&mut self) -> &mut super::audio::SoundGraphPluginRef {
        &mut self.nan_check_plugin
    }
}

impl super::audio::AudioGraphNodeDataTrait for NanCheckNodeData {
}

impl super::core::DataContainerTrait for NanCheckNodeData {
}

pub static NANCHECKNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NanCheckNodeData",
    name_hash: 3270054194,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        super_class_offset: offset_of!(NanCheckNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NanCheckNodeData as Default>::default())),
            create_boxed: || Box::new(<NanCheckNodeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "In",
                name_hash: 5862146,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(NanCheckNodeData, r#in),
            },
            FieldInfoData {
                name: "Out",
                name_hash: 193453899,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(NanCheckNodeData, out),
            },
            FieldInfoData {
                name: "NameNonMeta",
                name_hash: 666851312,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(NanCheckNodeData, name_non_meta),
            },
            FieldInfoData {
                name: "NanCheckPlugin",
                name_hash: 2199393547,
                flags: MemberInfoFlags::new(0),
                field_type: "SoundGraphPluginRef",
                rust_offset: offset_of!(NanCheckNodeData, nan_check_plugin),
            },
        ],
    }),
    array_type: Some(NANCHECKNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NanCheckNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        NANCHECKNODEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static NANCHECKNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NanCheckNodeData-Array",
    name_hash: 213587846,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("NanCheckNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LoudnessNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
    pub r#in: super::audio::AudioGraphNodePort,
    pub out: super::audio::AudioGraphNodePort,
    pub momentary: super::audio::AudioGraphNodePort,
    pub integrated: super::audio::AudioGraphNodePort,
    pub reset: super::audio::AudioGraphNodePort,
    pub plugin: super::audio::SoundGraphPluginRef,
    pub trace: bool,
    pub trace_label: String,
    pub integration_time: f32,
    pub enable_clamp: bool,
}

pub trait LoudnessNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
    fn r#in(&self) -> &super::audio::AudioGraphNodePort;
    fn r#in_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn out(&self) -> &super::audio::AudioGraphNodePort;
    fn out_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn momentary(&self) -> &super::audio::AudioGraphNodePort;
    fn momentary_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn integrated(&self) -> &super::audio::AudioGraphNodePort;
    fn integrated_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn reset(&self) -> &super::audio::AudioGraphNodePort;
    fn reset_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn plugin(&self) -> &super::audio::SoundGraphPluginRef;
    fn plugin_mut(&mut self) -> &mut super::audio::SoundGraphPluginRef;
    fn trace(&self) -> &bool;
    fn trace_mut(&mut self) -> &mut bool;
    fn trace_label(&self) -> &String;
    fn trace_label_mut(&mut self) -> &mut String;
    fn integration_time(&self) -> &f32;
    fn integration_time_mut(&mut self) -> &mut f32;
    fn enable_clamp(&self) -> &bool;
    fn enable_clamp_mut(&mut self) -> &mut bool;
}

impl LoudnessNodeDataTrait for LoudnessNodeData {
    fn r#in(&self) -> &super::audio::AudioGraphNodePort {
        &self.r#in
    }
    fn r#in_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.r#in
    }
    fn out(&self) -> &super::audio::AudioGraphNodePort {
        &self.out
    }
    fn out_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.out
    }
    fn momentary(&self) -> &super::audio::AudioGraphNodePort {
        &self.momentary
    }
    fn momentary_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.momentary
    }
    fn integrated(&self) -> &super::audio::AudioGraphNodePort {
        &self.integrated
    }
    fn integrated_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.integrated
    }
    fn reset(&self) -> &super::audio::AudioGraphNodePort {
        &self.reset
    }
    fn reset_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.reset
    }
    fn plugin(&self) -> &super::audio::SoundGraphPluginRef {
        &self.plugin
    }
    fn plugin_mut(&mut self) -> &mut super::audio::SoundGraphPluginRef {
        &mut self.plugin
    }
    fn trace(&self) -> &bool {
        &self.trace
    }
    fn trace_mut(&mut self) -> &mut bool {
        &mut self.trace
    }
    fn trace_label(&self) -> &String {
        &self.trace_label
    }
    fn trace_label_mut(&mut self) -> &mut String {
        &mut self.trace_label
    }
    fn integration_time(&self) -> &f32 {
        &self.integration_time
    }
    fn integration_time_mut(&mut self) -> &mut f32 {
        &mut self.integration_time
    }
    fn enable_clamp(&self) -> &bool {
        &self.enable_clamp
    }
    fn enable_clamp_mut(&mut self) -> &mut bool {
        &mut self.enable_clamp
    }
}

impl super::audio::AudioGraphNodeDataTrait for LoudnessNodeData {
}

impl super::core::DataContainerTrait for LoudnessNodeData {
}

pub static LOUDNESSNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LoudnessNodeData",
    name_hash: 1968575212,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        super_class_offset: offset_of!(LoudnessNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LoudnessNodeData as Default>::default())),
            create_boxed: || Box::new(<LoudnessNodeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "In",
                name_hash: 5862146,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(LoudnessNodeData, r#in),
            },
            FieldInfoData {
                name: "Out",
                name_hash: 193453899,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(LoudnessNodeData, out),
            },
            FieldInfoData {
                name: "Momentary",
                name_hash: 3118863551,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(LoudnessNodeData, momentary),
            },
            FieldInfoData {
                name: "Integrated",
                name_hash: 1769896434,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(LoudnessNodeData, integrated),
            },
            FieldInfoData {
                name: "Reset",
                name_hash: 229946160,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(LoudnessNodeData, reset),
            },
            FieldInfoData {
                name: "Plugin",
                name_hash: 3384353452,
                flags: MemberInfoFlags::new(0),
                field_type: "SoundGraphPluginRef",
                rust_offset: offset_of!(LoudnessNodeData, plugin),
            },
            FieldInfoData {
                name: "Trace",
                name_hash: 227189956,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LoudnessNodeData, trace),
            },
            FieldInfoData {
                name: "TraceLabel",
                name_hash: 1727876738,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(LoudnessNodeData, trace_label),
            },
            FieldInfoData {
                name: "IntegrationTime",
                name_hash: 3046154350,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LoudnessNodeData, integration_time),
            },
            FieldInfoData {
                name: "EnableClamp",
                name_hash: 644913175,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LoudnessNodeData, enable_clamp),
            },
        ],
    }),
    array_type: Some(LOUDNESSNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LoudnessNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        LOUDNESSNODEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static LOUDNESSNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LoudnessNodeData-Array",
    name_hash: 3288478424,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("LoudnessNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ListenerNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
    pub world_x: super::audio::AudioGraphNodePort,
    pub world_y: super::audio::AudioGraphNodePort,
    pub world_z: super::audio::AudioGraphNodePort,
}

pub trait ListenerNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
    fn world_x(&self) -> &super::audio::AudioGraphNodePort;
    fn world_x_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn world_y(&self) -> &super::audio::AudioGraphNodePort;
    fn world_y_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn world_z(&self) -> &super::audio::AudioGraphNodePort;
    fn world_z_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
}

impl ListenerNodeDataTrait for ListenerNodeData {
    fn world_x(&self) -> &super::audio::AudioGraphNodePort {
        &self.world_x
    }
    fn world_x_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.world_x
    }
    fn world_y(&self) -> &super::audio::AudioGraphNodePort {
        &self.world_y
    }
    fn world_y_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.world_y
    }
    fn world_z(&self) -> &super::audio::AudioGraphNodePort {
        &self.world_z
    }
    fn world_z_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.world_z
    }
}

impl super::audio::AudioGraphNodeDataTrait for ListenerNodeData {
}

impl super::core::DataContainerTrait for ListenerNodeData {
}

pub static LISTENERNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ListenerNodeData",
    name_hash: 3401605451,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        super_class_offset: offset_of!(ListenerNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ListenerNodeData as Default>::default())),
            create_boxed: || Box::new(<ListenerNodeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "WorldX",
                name_hash: 3192402367,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ListenerNodeData, world_x),
            },
            FieldInfoData {
                name: "WorldY",
                name_hash: 3192402366,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ListenerNodeData, world_y),
            },
            FieldInfoData {
                name: "WorldZ",
                name_hash: 3192402365,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ListenerNodeData, world_z),
            },
        ],
    }),
    array_type: Some(LISTENERNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ListenerNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        LISTENERNODEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static LISTENERNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ListenerNodeData-Array",
    name_hash: 3726991487,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ListenerNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct IndexMapperNodeConfigData {
    pub _glacier_base: super::audio::AudioGraphNodeConfigData,
    pub dimension_groups: Vec<BoxedTypeObject /* IndexMapperConfigGroup */>,
}

pub trait IndexMapperNodeConfigDataTrait: super::audio::AudioGraphNodeConfigDataTrait {
    fn dimension_groups(&self) -> &Vec<BoxedTypeObject /* IndexMapperConfigGroup */>;
    fn dimension_groups_mut(&mut self) -> &mut Vec<BoxedTypeObject /* IndexMapperConfigGroup */>;
}

impl IndexMapperNodeConfigDataTrait for IndexMapperNodeConfigData {
    fn dimension_groups(&self) -> &Vec<BoxedTypeObject /* IndexMapperConfigGroup */> {
        &self.dimension_groups
    }
    fn dimension_groups_mut(&mut self) -> &mut Vec<BoxedTypeObject /* IndexMapperConfigGroup */> {
        &mut self.dimension_groups
    }
}

impl super::audio::AudioGraphNodeConfigDataTrait for IndexMapperNodeConfigData {
    fn node(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphNodeData */> {
        self._glacier_base.node()
    }
    fn node_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphNodeData */> {
        self._glacier_base.node_mut()
    }
    fn configured_property_flags(&self) -> &u64 {
        self._glacier_base.configured_property_flags()
    }
    fn configured_property_flags_mut(&mut self) -> &mut u64 {
        self._glacier_base.configured_property_flags_mut()
    }
}

impl super::core::DataContainerTrait for IndexMapperNodeConfigData {
}

pub static INDEXMAPPERNODECONFIGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IndexMapperNodeConfigData",
    name_hash: 2604324378,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODECONFIGDATA_TYPE_INFO),
        super_class_offset: offset_of!(IndexMapperNodeConfigData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IndexMapperNodeConfigData as Default>::default())),
            create_boxed: || Box::new(<IndexMapperNodeConfigData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "DimensionGroups",
                name_hash: 2367791513,
                flags: MemberInfoFlags::new(144),
                field_type: "IndexMapperConfigGroup-Array",
                rust_offset: offset_of!(IndexMapperNodeConfigData, dimension_groups),
            },
        ],
    }),
    array_type: Some(INDEXMAPPERNODECONFIGDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IndexMapperNodeConfigData {
    fn type_info(&self) -> &'static TypeInfo {
        INDEXMAPPERNODECONFIGDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static INDEXMAPPERNODECONFIGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IndexMapperNodeConfigData-Array",
    name_hash: 3722467630,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("IndexMapperNodeConfigData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct IndexMapperConfigGroup {
    pub dimension_size: u32,
}

pub trait IndexMapperConfigGroupTrait: TypeObject {
    fn dimension_size(&self) -> &u32;
    fn dimension_size_mut(&mut self) -> &mut u32;
}

impl IndexMapperConfigGroupTrait for IndexMapperConfigGroup {
    fn dimension_size(&self) -> &u32 {
        &self.dimension_size
    }
    fn dimension_size_mut(&mut self) -> &mut u32 {
        &mut self.dimension_size
    }
}

pub static INDEXMAPPERCONFIGGROUP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IndexMapperConfigGroup",
    name_hash: 202840821,
    flags: MemberInfoFlags::new(36937),
    module: "DiceCommonsShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IndexMapperConfigGroup as Default>::default())),
            create_boxed: || Box::new(<IndexMapperConfigGroup as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "DimensionSize",
                name_hash: 1484701488,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(IndexMapperConfigGroup, dimension_size),
            },
        ],
    }),
    array_type: Some(INDEXMAPPERCONFIGGROUP_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for IndexMapperConfigGroup {
    fn type_info(&self) -> &'static TypeInfo {
        INDEXMAPPERCONFIGGROUP_TYPE_INFO
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


pub static INDEXMAPPERCONFIGGROUP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IndexMapperConfigGroup-Array",
    name_hash: 3983027137,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("IndexMapperConfigGroup"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct IndexMapperNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
    pub dimension_groups: Vec<Option<LockedTypeObject /* IndexMapperGroup */>>,
    pub result: super::audio::AudioGraphNodePort,
}

pub trait IndexMapperNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
    fn dimension_groups(&self) -> &Vec<Option<LockedTypeObject /* IndexMapperGroup */>>;
    fn dimension_groups_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* IndexMapperGroup */>>;
    fn result(&self) -> &super::audio::AudioGraphNodePort;
    fn result_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
}

impl IndexMapperNodeDataTrait for IndexMapperNodeData {
    fn dimension_groups(&self) -> &Vec<Option<LockedTypeObject /* IndexMapperGroup */>> {
        &self.dimension_groups
    }
    fn dimension_groups_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* IndexMapperGroup */>> {
        &mut self.dimension_groups
    }
    fn result(&self) -> &super::audio::AudioGraphNodePort {
        &self.result
    }
    fn result_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.result
    }
}

impl super::audio::AudioGraphNodeDataTrait for IndexMapperNodeData {
}

impl super::core::DataContainerTrait for IndexMapperNodeData {
}

pub static INDEXMAPPERNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IndexMapperNodeData",
    name_hash: 4157740912,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        super_class_offset: offset_of!(IndexMapperNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IndexMapperNodeData as Default>::default())),
            create_boxed: || Box::new(<IndexMapperNodeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "DimensionGroups",
                name_hash: 2367791513,
                flags: MemberInfoFlags::new(144),
                field_type: "IndexMapperGroup-Array",
                rust_offset: offset_of!(IndexMapperNodeData, dimension_groups),
            },
            FieldInfoData {
                name: "Result",
                name_hash: 3293273164,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(IndexMapperNodeData, result),
            },
        ],
    }),
    array_type: Some(INDEXMAPPERNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IndexMapperNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        INDEXMAPPERNODEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static INDEXMAPPERNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IndexMapperNodeData-Array",
    name_hash: 90395972,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("IndexMapperNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct IndexMapperGroup {
    pub _glacier_base: super::audio::AudioGraphNodePortGroup,
    pub guid: glacier_util::guid::Guid,
    pub dimension_size: u32,
    pub r#in: super::audio::AudioGraphNodePort,
}

pub trait IndexMapperGroupTrait: super::audio::AudioGraphNodePortGroupTrait {
    fn guid(&self) -> &glacier_util::guid::Guid;
    fn guid_mut(&mut self) -> &mut glacier_util::guid::Guid;
    fn dimension_size(&self) -> &u32;
    fn dimension_size_mut(&mut self) -> &mut u32;
    fn r#in(&self) -> &super::audio::AudioGraphNodePort;
    fn r#in_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
}

impl IndexMapperGroupTrait for IndexMapperGroup {
    fn guid(&self) -> &glacier_util::guid::Guid {
        &self.guid
    }
    fn guid_mut(&mut self) -> &mut glacier_util::guid::Guid {
        &mut self.guid
    }
    fn dimension_size(&self) -> &u32 {
        &self.dimension_size
    }
    fn dimension_size_mut(&mut self) -> &mut u32 {
        &mut self.dimension_size
    }
    fn r#in(&self) -> &super::audio::AudioGraphNodePort {
        &self.r#in
    }
    fn r#in_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.r#in
    }
}

impl super::audio::AudioGraphNodePortGroupTrait for IndexMapperGroup {
}

impl super::core::DataContainerTrait for IndexMapperGroup {
}

pub static INDEXMAPPERGROUP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IndexMapperGroup",
    name_hash: 2288920735,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEPORTGROUP_TYPE_INFO),
        super_class_offset: offset_of!(IndexMapperGroup, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IndexMapperGroup as Default>::default())),
            create_boxed: || Box::new(<IndexMapperGroup as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Guid",
                name_hash: 2088724858,
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(IndexMapperGroup, guid),
            },
            FieldInfoData {
                name: "DimensionSize",
                name_hash: 1484701488,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(IndexMapperGroup, dimension_size),
            },
            FieldInfoData {
                name: "In",
                name_hash: 5862146,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(IndexMapperGroup, r#in),
            },
        ],
    }),
    array_type: Some(INDEXMAPPERGROUP_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IndexMapperGroup {
    fn type_info(&self) -> &'static TypeInfo {
        INDEXMAPPERGROUP_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static INDEXMAPPERGROUP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IndexMapperGroup-Array",
    name_hash: 662588715,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("IndexMapperGroup"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct GateNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
    pub r#in: super::audio::AudioGraphNodePort,
    pub open: super::audio::AudioGraphNodePort,
    pub close: super::audio::AudioGraphNodePort,
    pub out: super::audio::AudioGraphNodePort,
    pub start_opened: bool,
}

pub trait GateNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
    fn r#in(&self) -> &super::audio::AudioGraphNodePort;
    fn r#in_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn open(&self) -> &super::audio::AudioGraphNodePort;
    fn open_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn close(&self) -> &super::audio::AudioGraphNodePort;
    fn close_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn out(&self) -> &super::audio::AudioGraphNodePort;
    fn out_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn start_opened(&self) -> &bool;
    fn start_opened_mut(&mut self) -> &mut bool;
}

impl GateNodeDataTrait for GateNodeData {
    fn r#in(&self) -> &super::audio::AudioGraphNodePort {
        &self.r#in
    }
    fn r#in_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.r#in
    }
    fn open(&self) -> &super::audio::AudioGraphNodePort {
        &self.open
    }
    fn open_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.open
    }
    fn close(&self) -> &super::audio::AudioGraphNodePort {
        &self.close
    }
    fn close_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.close
    }
    fn out(&self) -> &super::audio::AudioGraphNodePort {
        &self.out
    }
    fn out_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.out
    }
    fn start_opened(&self) -> &bool {
        &self.start_opened
    }
    fn start_opened_mut(&mut self) -> &mut bool {
        &mut self.start_opened
    }
}

impl super::audio::AudioGraphNodeDataTrait for GateNodeData {
}

impl super::core::DataContainerTrait for GateNodeData {
}

pub static GATENODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GateNodeData",
    name_hash: 743464962,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        super_class_offset: offset_of!(GateNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GateNodeData as Default>::default())),
            create_boxed: || Box::new(<GateNodeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "In",
                name_hash: 5862146,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(GateNodeData, r#in),
            },
            FieldInfoData {
                name: "Open",
                name_hash: 2089008817,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(GateNodeData, open),
            },
            FieldInfoData {
                name: "Close",
                name_hash: 212633683,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(GateNodeData, close),
            },
            FieldInfoData {
                name: "Out",
                name_hash: 193453899,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(GateNodeData, out),
            },
            FieldInfoData {
                name: "StartOpened",
                name_hash: 248233008,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GateNodeData, start_opened),
            },
        ],
    }),
    array_type: Some(GATENODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GateNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        GATENODEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static GATENODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GateNodeData-Array",
    name_hash: 4251139894,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("GateNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ForceIsNotLoopingNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
}

pub trait ForceIsNotLoopingNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
}

impl ForceIsNotLoopingNodeDataTrait for ForceIsNotLoopingNodeData {
}

impl super::audio::AudioGraphNodeDataTrait for ForceIsNotLoopingNodeData {
}

impl super::core::DataContainerTrait for ForceIsNotLoopingNodeData {
}

pub static FORCEISNOTLOOPINGNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForceIsNotLoopingNodeData",
    name_hash: 573049819,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        super_class_offset: offset_of!(ForceIsNotLoopingNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ForceIsNotLoopingNodeData as Default>::default())),
            create_boxed: || Box::new(<ForceIsNotLoopingNodeData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(FORCEISNOTLOOPINGNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ForceIsNotLoopingNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        FORCEISNOTLOOPINGNODEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static FORCEISNOTLOOPINGNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForceIsNotLoopingNodeData-Array",
    name_hash: 1607840495,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ForceIsNotLoopingNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ForceIsLoopingNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
}

pub trait ForceIsLoopingNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
}

impl ForceIsLoopingNodeDataTrait for ForceIsLoopingNodeData {
}

impl super::audio::AudioGraphNodeDataTrait for ForceIsLoopingNodeData {
}

impl super::core::DataContainerTrait for ForceIsLoopingNodeData {
}

pub static FORCEISLOOPINGNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForceIsLoopingNodeData",
    name_hash: 791098446,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        super_class_offset: offset_of!(ForceIsLoopingNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ForceIsLoopingNodeData as Default>::default())),
            create_boxed: || Box::new(<ForceIsLoopingNodeData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(FORCEISLOOPINGNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ForceIsLoopingNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        FORCEISLOOPINGNODEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static FORCEISLOOPINGNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForceIsLoopingNodeData-Array",
    name_hash: 782732794,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ForceIsLoopingNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EventSelectorNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
    pub index: super::audio::AudioGraphNodePort,
    pub input_events: Vec<Option<LockedTypeObject /* InputEventsGroup */>>,
    pub out: super::audio::AudioGraphNodePort,
}

pub trait EventSelectorNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
    fn index(&self) -> &super::audio::AudioGraphNodePort;
    fn index_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn input_events(&self) -> &Vec<Option<LockedTypeObject /* InputEventsGroup */>>;
    fn input_events_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* InputEventsGroup */>>;
    fn out(&self) -> &super::audio::AudioGraphNodePort;
    fn out_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
}

impl EventSelectorNodeDataTrait for EventSelectorNodeData {
    fn index(&self) -> &super::audio::AudioGraphNodePort {
        &self.index
    }
    fn index_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.index
    }
    fn input_events(&self) -> &Vec<Option<LockedTypeObject /* InputEventsGroup */>> {
        &self.input_events
    }
    fn input_events_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* InputEventsGroup */>> {
        &mut self.input_events
    }
    fn out(&self) -> &super::audio::AudioGraphNodePort {
        &self.out
    }
    fn out_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.out
    }
}

impl super::audio::AudioGraphNodeDataTrait for EventSelectorNodeData {
}

impl super::core::DataContainerTrait for EventSelectorNodeData {
}

pub static EVENTSELECTORNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventSelectorNodeData",
    name_hash: 2019867820,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        super_class_offset: offset_of!(EventSelectorNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EventSelectorNodeData as Default>::default())),
            create_boxed: || Box::new(<EventSelectorNodeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Index",
                name_hash: 214509467,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(EventSelectorNodeData, index),
            },
            FieldInfoData {
                name: "InputEvents",
                name_hash: 1542460652,
                flags: MemberInfoFlags::new(144),
                field_type: "InputEventsGroup-Array",
                rust_offset: offset_of!(EventSelectorNodeData, input_events),
            },
            FieldInfoData {
                name: "Out",
                name_hash: 193453899,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(EventSelectorNodeData, out),
            },
        ],
    }),
    array_type: Some(EVENTSELECTORNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EventSelectorNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        EVENTSELECTORNODEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static EVENTSELECTORNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventSelectorNodeData-Array",
    name_hash: 1344089112,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("EventSelectorNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct InputEventsGroup {
    pub _glacier_base: super::audio::AudioGraphNodePortGroup,
    pub r#in: super::audio::AudioGraphNodePort,
}

pub trait InputEventsGroupTrait: super::audio::AudioGraphNodePortGroupTrait {
    fn r#in(&self) -> &super::audio::AudioGraphNodePort;
    fn r#in_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
}

impl InputEventsGroupTrait for InputEventsGroup {
    fn r#in(&self) -> &super::audio::AudioGraphNodePort {
        &self.r#in
    }
    fn r#in_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.r#in
    }
}

impl super::audio::AudioGraphNodePortGroupTrait for InputEventsGroup {
}

impl super::core::DataContainerTrait for InputEventsGroup {
}

pub static INPUTEVENTSGROUP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputEventsGroup",
    name_hash: 457420499,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEPORTGROUP_TYPE_INFO),
        super_class_offset: offset_of!(InputEventsGroup, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InputEventsGroup as Default>::default())),
            create_boxed: || Box::new(<InputEventsGroup as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "In",
                name_hash: 5862146,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(InputEventsGroup, r#in),
            },
        ],
    }),
    array_type: Some(INPUTEVENTSGROUP_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for InputEventsGroup {
    fn type_info(&self) -> &'static TypeInfo {
        INPUTEVENTSGROUP_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static INPUTEVENTSGROUP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputEventsGroup-Array",
    name_hash: 2472557031,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("InputEventsGroup"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EventGateConditionValueNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
    pub condition: EventGateConditionValueType,
    pub r#in: super::audio::AudioGraphNodePort,
    pub in_value: super::audio::AudioGraphNodePort,
    pub cool_down_time: super::audio::AudioGraphNodePort,
    pub enable: super::audio::AudioGraphNodePort,
    pub out: super::audio::AudioGraphNodePort,
    pub out_value: super::audio::AudioGraphNodePort,
}

pub trait EventGateConditionValueNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
    fn condition(&self) -> &EventGateConditionValueType;
    fn condition_mut(&mut self) -> &mut EventGateConditionValueType;
    fn r#in(&self) -> &super::audio::AudioGraphNodePort;
    fn r#in_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn in_value(&self) -> &super::audio::AudioGraphNodePort;
    fn in_value_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn cool_down_time(&self) -> &super::audio::AudioGraphNodePort;
    fn cool_down_time_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn enable(&self) -> &super::audio::AudioGraphNodePort;
    fn enable_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn out(&self) -> &super::audio::AudioGraphNodePort;
    fn out_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn out_value(&self) -> &super::audio::AudioGraphNodePort;
    fn out_value_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
}

impl EventGateConditionValueNodeDataTrait for EventGateConditionValueNodeData {
    fn condition(&self) -> &EventGateConditionValueType {
        &self.condition
    }
    fn condition_mut(&mut self) -> &mut EventGateConditionValueType {
        &mut self.condition
    }
    fn r#in(&self) -> &super::audio::AudioGraphNodePort {
        &self.r#in
    }
    fn r#in_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.r#in
    }
    fn in_value(&self) -> &super::audio::AudioGraphNodePort {
        &self.in_value
    }
    fn in_value_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.in_value
    }
    fn cool_down_time(&self) -> &super::audio::AudioGraphNodePort {
        &self.cool_down_time
    }
    fn cool_down_time_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.cool_down_time
    }
    fn enable(&self) -> &super::audio::AudioGraphNodePort {
        &self.enable
    }
    fn enable_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.enable
    }
    fn out(&self) -> &super::audio::AudioGraphNodePort {
        &self.out
    }
    fn out_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.out
    }
    fn out_value(&self) -> &super::audio::AudioGraphNodePort {
        &self.out_value
    }
    fn out_value_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.out_value
    }
}

impl super::audio::AudioGraphNodeDataTrait for EventGateConditionValueNodeData {
}

impl super::core::DataContainerTrait for EventGateConditionValueNodeData {
}

pub static EVENTGATECONDITIONVALUENODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventGateConditionValueNodeData",
    name_hash: 2539977750,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        super_class_offset: offset_of!(EventGateConditionValueNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EventGateConditionValueNodeData as Default>::default())),
            create_boxed: || Box::new(<EventGateConditionValueNodeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Condition",
                name_hash: 1800624758,
                flags: MemberInfoFlags::new(0),
                field_type: "EventGateConditionValueType",
                rust_offset: offset_of!(EventGateConditionValueNodeData, condition),
            },
            FieldInfoData {
                name: "In",
                name_hash: 5862146,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(EventGateConditionValueNodeData, r#in),
            },
            FieldInfoData {
                name: "InValue",
                name_hash: 1658462601,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(EventGateConditionValueNodeData, in_value),
            },
            FieldInfoData {
                name: "CoolDownTime",
                name_hash: 282296301,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(EventGateConditionValueNodeData, cool_down_time),
            },
            FieldInfoData {
                name: "Enable",
                name_hash: 2342790116,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(EventGateConditionValueNodeData, enable),
            },
            FieldInfoData {
                name: "Out",
                name_hash: 193453899,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(EventGateConditionValueNodeData, out),
            },
            FieldInfoData {
                name: "OutValue",
                name_hash: 993810272,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(EventGateConditionValueNodeData, out_value),
            },
        ],
    }),
    array_type: Some(EVENTGATECONDITIONVALUENODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EventGateConditionValueNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        EVENTGATECONDITIONVALUENODEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static EVENTGATECONDITIONVALUENODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventGateConditionValueNodeData-Array",
    name_hash: 3578745122,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("EventGateConditionValueNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum EventGateConditionValueType {
    #[default]
    EventGateConditionValueType_Equal = 0,
    EventGateConditionValueType_NotEqual = 1,
    EventGateConditionValueType_Less = 2,
    EventGateConditionValueType_Greater = 3,
    EventGateConditionValueType_LessOrEqual = 4,
    EventGateConditionValueType_GreaterOrEqual = 5,
    EventGateConditionValueType_AND = 6,
    EventGateConditionValueType_OR = 7,
}

pub static EVENTGATECONDITIONVALUETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventGateConditionValueType",
    name_hash: 921384574,
    flags: MemberInfoFlags::new(49429),
    module: "DiceCommonsShared",
    data: TypeInfoData::Enum,
    array_type: Some(EVENTGATECONDITIONVALUETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EventGateConditionValueType {
    fn type_info(&self) -> &'static TypeInfo {
        EVENTGATECONDITIONVALUETYPE_TYPE_INFO
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


pub static EVENTGATECONDITIONVALUETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventGateConditionValueType-Array",
    name_hash: 3187865418,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("EventGateConditionValueType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DicePhysicsNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
    pub angular_velocity_x: super::audio::AudioGraphNodePort,
    pub angular_velocity_y: super::audio::AudioGraphNodePort,
    pub angular_velocity_z: super::audio::AudioGraphNodePort,
    pub signed_speed: bool,
}

pub trait DicePhysicsNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
    fn angular_velocity_x(&self) -> &super::audio::AudioGraphNodePort;
    fn angular_velocity_x_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn angular_velocity_y(&self) -> &super::audio::AudioGraphNodePort;
    fn angular_velocity_y_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn angular_velocity_z(&self) -> &super::audio::AudioGraphNodePort;
    fn angular_velocity_z_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn signed_speed(&self) -> &bool;
    fn signed_speed_mut(&mut self) -> &mut bool;
}

impl DicePhysicsNodeDataTrait for DicePhysicsNodeData {
    fn angular_velocity_x(&self) -> &super::audio::AudioGraphNodePort {
        &self.angular_velocity_x
    }
    fn angular_velocity_x_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.angular_velocity_x
    }
    fn angular_velocity_y(&self) -> &super::audio::AudioGraphNodePort {
        &self.angular_velocity_y
    }
    fn angular_velocity_y_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.angular_velocity_y
    }
    fn angular_velocity_z(&self) -> &super::audio::AudioGraphNodePort {
        &self.angular_velocity_z
    }
    fn angular_velocity_z_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.angular_velocity_z
    }
    fn signed_speed(&self) -> &bool {
        &self.signed_speed
    }
    fn signed_speed_mut(&mut self) -> &mut bool {
        &mut self.signed_speed
    }
}

impl super::audio::AudioGraphNodeDataTrait for DicePhysicsNodeData {
}

impl super::core::DataContainerTrait for DicePhysicsNodeData {
}

pub static DICEPHYSICSNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DicePhysicsNodeData",
    name_hash: 238857365,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        super_class_offset: offset_of!(DicePhysicsNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DicePhysicsNodeData as Default>::default())),
            create_boxed: || Box::new(<DicePhysicsNodeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "AngularVelocity_X",
                name_hash: 3631900631,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(DicePhysicsNodeData, angular_velocity_x),
            },
            FieldInfoData {
                name: "AngularVelocity_Y",
                name_hash: 3631900630,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(DicePhysicsNodeData, angular_velocity_y),
            },
            FieldInfoData {
                name: "AngularVelocity_Z",
                name_hash: 3631900629,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(DicePhysicsNodeData, angular_velocity_z),
            },
            FieldInfoData {
                name: "SignedSpeed",
                name_hash: 565498448,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DicePhysicsNodeData, signed_speed),
            },
        ],
    }),
    array_type: Some(DICEPHYSICSNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DicePhysicsNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        DICEPHYSICSNODEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DICEPHYSICSNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DicePhysicsNodeData-Array",
    name_hash: 2053635105,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DicePhysicsNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DiceDivisibleLoopPlayerNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
    pub start: super::audio::AudioGraphNodePort,
    pub stop: super::audio::AudioGraphNodePort,
    pub amplitude: super::audio::AudioGraphNodePort,
    pub freeze_segment: super::audio::AudioGraphNodePort,
    pub output: super::audio::AudioGraphNodePort,
    pub wave: Option<LockedTypeObject /* super::audio::SoundWaveAsset */>,
    pub external_wave: super::audio::AudioGraphNodePort,
    pub plugins: Vec<BoxedTypeObject /* DiceDivisibleLoopPlayerPlugins */>,
    pub cross_fade_length: f32,
    pub start_at_random_position: bool,
}

pub trait DiceDivisibleLoopPlayerNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
    fn start(&self) -> &super::audio::AudioGraphNodePort;
    fn start_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn stop(&self) -> &super::audio::AudioGraphNodePort;
    fn stop_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn amplitude(&self) -> &super::audio::AudioGraphNodePort;
    fn amplitude_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn freeze_segment(&self) -> &super::audio::AudioGraphNodePort;
    fn freeze_segment_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn output(&self) -> &super::audio::AudioGraphNodePort;
    fn output_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn wave(&self) -> &Option<LockedTypeObject /* super::audio::SoundWaveAsset */>;
    fn wave_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundWaveAsset */>;
    fn external_wave(&self) -> &super::audio::AudioGraphNodePort;
    fn external_wave_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn plugins(&self) -> &Vec<BoxedTypeObject /* DiceDivisibleLoopPlayerPlugins */>;
    fn plugins_mut(&mut self) -> &mut Vec<BoxedTypeObject /* DiceDivisibleLoopPlayerPlugins */>;
    fn cross_fade_length(&self) -> &f32;
    fn cross_fade_length_mut(&mut self) -> &mut f32;
    fn start_at_random_position(&self) -> &bool;
    fn start_at_random_position_mut(&mut self) -> &mut bool;
}

impl DiceDivisibleLoopPlayerNodeDataTrait for DiceDivisibleLoopPlayerNodeData {
    fn start(&self) -> &super::audio::AudioGraphNodePort {
        &self.start
    }
    fn start_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.start
    }
    fn stop(&self) -> &super::audio::AudioGraphNodePort {
        &self.stop
    }
    fn stop_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.stop
    }
    fn amplitude(&self) -> &super::audio::AudioGraphNodePort {
        &self.amplitude
    }
    fn amplitude_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.amplitude
    }
    fn freeze_segment(&self) -> &super::audio::AudioGraphNodePort {
        &self.freeze_segment
    }
    fn freeze_segment_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.freeze_segment
    }
    fn output(&self) -> &super::audio::AudioGraphNodePort {
        &self.output
    }
    fn output_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.output
    }
    fn wave(&self) -> &Option<LockedTypeObject /* super::audio::SoundWaveAsset */> {
        &self.wave
    }
    fn wave_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundWaveAsset */> {
        &mut self.wave
    }
    fn external_wave(&self) -> &super::audio::AudioGraphNodePort {
        &self.external_wave
    }
    fn external_wave_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.external_wave
    }
    fn plugins(&self) -> &Vec<BoxedTypeObject /* DiceDivisibleLoopPlayerPlugins */> {
        &self.plugins
    }
    fn plugins_mut(&mut self) -> &mut Vec<BoxedTypeObject /* DiceDivisibleLoopPlayerPlugins */> {
        &mut self.plugins
    }
    fn cross_fade_length(&self) -> &f32 {
        &self.cross_fade_length
    }
    fn cross_fade_length_mut(&mut self) -> &mut f32 {
        &mut self.cross_fade_length
    }
    fn start_at_random_position(&self) -> &bool {
        &self.start_at_random_position
    }
    fn start_at_random_position_mut(&mut self) -> &mut bool {
        &mut self.start_at_random_position
    }
}

impl super::audio::AudioGraphNodeDataTrait for DiceDivisibleLoopPlayerNodeData {
}

impl super::core::DataContainerTrait for DiceDivisibleLoopPlayerNodeData {
}

pub static DICEDIVISIBLELOOPPLAYERNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceDivisibleLoopPlayerNodeData",
    name_hash: 3736821138,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        super_class_offset: offset_of!(DiceDivisibleLoopPlayerNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DiceDivisibleLoopPlayerNodeData as Default>::default())),
            create_boxed: || Box::new(<DiceDivisibleLoopPlayerNodeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Start",
                name_hash: 230748069,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(DiceDivisibleLoopPlayerNodeData, start),
            },
            FieldInfoData {
                name: "Stop",
                name_hash: 2089401213,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(DiceDivisibleLoopPlayerNodeData, stop),
            },
            FieldInfoData {
                name: "Amplitude",
                name_hash: 698564572,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(DiceDivisibleLoopPlayerNodeData, amplitude),
            },
            FieldInfoData {
                name: "FreezeSegment",
                name_hash: 3532981773,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(DiceDivisibleLoopPlayerNodeData, freeze_segment),
            },
            FieldInfoData {
                name: "Output",
                name_hash: 2895736442,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(DiceDivisibleLoopPlayerNodeData, output),
            },
            FieldInfoData {
                name: "Wave",
                name_hash: 2089277184,
                flags: MemberInfoFlags::new(0),
                field_type: "SoundWaveAsset",
                rust_offset: offset_of!(DiceDivisibleLoopPlayerNodeData, wave),
            },
            FieldInfoData {
                name: "ExternalWave",
                name_hash: 2162866621,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(DiceDivisibleLoopPlayerNodeData, external_wave),
            },
            FieldInfoData {
                name: "Plugins",
                name_hash: 14514271,
                flags: MemberInfoFlags::new(144),
                field_type: "DiceDivisibleLoopPlayerPlugins-Array",
                rust_offset: offset_of!(DiceDivisibleLoopPlayerNodeData, plugins),
            },
            FieldInfoData {
                name: "CrossFadeLength",
                name_hash: 2603726337,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DiceDivisibleLoopPlayerNodeData, cross_fade_length),
            },
            FieldInfoData {
                name: "StartAtRandomPosition",
                name_hash: 663970258,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DiceDivisibleLoopPlayerNodeData, start_at_random_position),
            },
        ],
    }),
    array_type: Some(DICEDIVISIBLELOOPPLAYERNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DiceDivisibleLoopPlayerNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        DICEDIVISIBLELOOPPLAYERNODEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DICEDIVISIBLELOOPPLAYERNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceDivisibleLoopPlayerNodeData-Array",
    name_hash: 4060685478,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceDivisibleLoopPlayerNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DiceDivisibleLoopPlayerPlugins {
    pub snd_player: super::audio::SoundGraphPluginRef,
    pub pause: super::audio::SoundGraphPluginRef,
    pub gain: super::audio::SoundGraphPluginRef,
    pub gain_fader: super::audio::SoundGraphPluginRef,
}

pub trait DiceDivisibleLoopPlayerPluginsTrait: TypeObject {
    fn snd_player(&self) -> &super::audio::SoundGraphPluginRef;
    fn snd_player_mut(&mut self) -> &mut super::audio::SoundGraphPluginRef;
    fn pause(&self) -> &super::audio::SoundGraphPluginRef;
    fn pause_mut(&mut self) -> &mut super::audio::SoundGraphPluginRef;
    fn gain(&self) -> &super::audio::SoundGraphPluginRef;
    fn gain_mut(&mut self) -> &mut super::audio::SoundGraphPluginRef;
    fn gain_fader(&self) -> &super::audio::SoundGraphPluginRef;
    fn gain_fader_mut(&mut self) -> &mut super::audio::SoundGraphPluginRef;
}

impl DiceDivisibleLoopPlayerPluginsTrait for DiceDivisibleLoopPlayerPlugins {
    fn snd_player(&self) -> &super::audio::SoundGraphPluginRef {
        &self.snd_player
    }
    fn snd_player_mut(&mut self) -> &mut super::audio::SoundGraphPluginRef {
        &mut self.snd_player
    }
    fn pause(&self) -> &super::audio::SoundGraphPluginRef {
        &self.pause
    }
    fn pause_mut(&mut self) -> &mut super::audio::SoundGraphPluginRef {
        &mut self.pause
    }
    fn gain(&self) -> &super::audio::SoundGraphPluginRef {
        &self.gain
    }
    fn gain_mut(&mut self) -> &mut super::audio::SoundGraphPluginRef {
        &mut self.gain
    }
    fn gain_fader(&self) -> &super::audio::SoundGraphPluginRef {
        &self.gain_fader
    }
    fn gain_fader_mut(&mut self) -> &mut super::audio::SoundGraphPluginRef {
        &mut self.gain_fader
    }
}

pub static DICEDIVISIBLELOOPPLAYERPLUGINS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceDivisibleLoopPlayerPlugins",
    name_hash: 577531768,
    flags: MemberInfoFlags::new(36937),
    module: "DiceCommonsShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DiceDivisibleLoopPlayerPlugins as Default>::default())),
            create_boxed: || Box::new(<DiceDivisibleLoopPlayerPlugins as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "SndPlayer",
                name_hash: 728257487,
                flags: MemberInfoFlags::new(0),
                field_type: "SoundGraphPluginRef",
                rust_offset: offset_of!(DiceDivisibleLoopPlayerPlugins, snd_player),
            },
            FieldInfoData {
                name: "Pause",
                name_hash: 232316407,
                flags: MemberInfoFlags::new(0),
                field_type: "SoundGraphPluginRef",
                rust_offset: offset_of!(DiceDivisibleLoopPlayerPlugins, pause),
            },
            FieldInfoData {
                name: "Gain",
                name_hash: 2088703076,
                flags: MemberInfoFlags::new(0),
                field_type: "SoundGraphPluginRef",
                rust_offset: offset_of!(DiceDivisibleLoopPlayerPlugins, gain),
            },
            FieldInfoData {
                name: "GainFader",
                name_hash: 2943317296,
                flags: MemberInfoFlags::new(0),
                field_type: "SoundGraphPluginRef",
                rust_offset: offset_of!(DiceDivisibleLoopPlayerPlugins, gain_fader),
            },
        ],
    }),
    array_type: Some(DICEDIVISIBLELOOPPLAYERPLUGINS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DiceDivisibleLoopPlayerPlugins {
    fn type_info(&self) -> &'static TypeInfo {
        DICEDIVISIBLELOOPPLAYERPLUGINS_TYPE_INFO
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


pub static DICEDIVISIBLELOOPPLAYERPLUGINS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceDivisibleLoopPlayerPlugins-Array",
    name_hash: 2195654988,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceDivisibleLoopPlayerPlugins"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DiceAdsrNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
    pub trigger: super::audio::AudioGraphNodePort,
    pub release: super::audio::AudioGraphNodePort,
    pub a: super::audio::AudioGraphNodePort,
    pub d: super::audio::AudioGraphNodePort,
    pub s: super::audio::AudioGraphNodePort,
    pub r: super::audio::AudioGraphNodePort,
    pub value: super::audio::AudioGraphNodePort,
    pub trigger_output: super::audio::AudioGraphNodePort,
    pub finished: super::audio::AudioGraphNodePort,
    pub always_start_from_zero: bool,
    pub scale: f32,
}

pub trait DiceAdsrNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
    fn trigger(&self) -> &super::audio::AudioGraphNodePort;
    fn trigger_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn release(&self) -> &super::audio::AudioGraphNodePort;
    fn release_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn a(&self) -> &super::audio::AudioGraphNodePort;
    fn a_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn d(&self) -> &super::audio::AudioGraphNodePort;
    fn d_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn s(&self) -> &super::audio::AudioGraphNodePort;
    fn s_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn r(&self) -> &super::audio::AudioGraphNodePort;
    fn r_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn value(&self) -> &super::audio::AudioGraphNodePort;
    fn value_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn trigger_output(&self) -> &super::audio::AudioGraphNodePort;
    fn trigger_output_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn finished(&self) -> &super::audio::AudioGraphNodePort;
    fn finished_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn always_start_from_zero(&self) -> &bool;
    fn always_start_from_zero_mut(&mut self) -> &mut bool;
    fn scale(&self) -> &f32;
    fn scale_mut(&mut self) -> &mut f32;
}

impl DiceAdsrNodeDataTrait for DiceAdsrNodeData {
    fn trigger(&self) -> &super::audio::AudioGraphNodePort {
        &self.trigger
    }
    fn trigger_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.trigger
    }
    fn release(&self) -> &super::audio::AudioGraphNodePort {
        &self.release
    }
    fn release_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.release
    }
    fn a(&self) -> &super::audio::AudioGraphNodePort {
        &self.a
    }
    fn a_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.a
    }
    fn d(&self) -> &super::audio::AudioGraphNodePort {
        &self.d
    }
    fn d_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.d
    }
    fn s(&self) -> &super::audio::AudioGraphNodePort {
        &self.s
    }
    fn s_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.s
    }
    fn r(&self) -> &super::audio::AudioGraphNodePort {
        &self.r
    }
    fn r_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.r
    }
    fn value(&self) -> &super::audio::AudioGraphNodePort {
        &self.value
    }
    fn value_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.value
    }
    fn trigger_output(&self) -> &super::audio::AudioGraphNodePort {
        &self.trigger_output
    }
    fn trigger_output_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.trigger_output
    }
    fn finished(&self) -> &super::audio::AudioGraphNodePort {
        &self.finished
    }
    fn finished_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.finished
    }
    fn always_start_from_zero(&self) -> &bool {
        &self.always_start_from_zero
    }
    fn always_start_from_zero_mut(&mut self) -> &mut bool {
        &mut self.always_start_from_zero
    }
    fn scale(&self) -> &f32 {
        &self.scale
    }
    fn scale_mut(&mut self) -> &mut f32 {
        &mut self.scale
    }
}

impl super::audio::AudioGraphNodeDataTrait for DiceAdsrNodeData {
}

impl super::core::DataContainerTrait for DiceAdsrNodeData {
}

pub static DICEADSRNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceAdsrNodeData",
    name_hash: 2994863034,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        super_class_offset: offset_of!(DiceAdsrNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DiceAdsrNodeData as Default>::default())),
            create_boxed: || Box::new(<DiceAdsrNodeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Trigger",
                name_hash: 2606354109,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(DiceAdsrNodeData, trigger),
            },
            FieldInfoData {
                name: "Release",
                name_hash: 1335266828,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(DiceAdsrNodeData, release),
            },
            FieldInfoData {
                name: "A",
                name_hash: 177636,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(DiceAdsrNodeData, a),
            },
            FieldInfoData {
                name: "D",
                name_hash: 177633,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(DiceAdsrNodeData, d),
            },
            FieldInfoData {
                name: "S",
                name_hash: 177654,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(DiceAdsrNodeData, s),
            },
            FieldInfoData {
                name: "R",
                name_hash: 177655,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(DiceAdsrNodeData, r),
            },
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(DiceAdsrNodeData, value),
            },
            FieldInfoData {
                name: "TriggerOutput",
                name_hash: 2046336834,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(DiceAdsrNodeData, trigger_output),
            },
            FieldInfoData {
                name: "Finished",
                name_hash: 1223765815,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(DiceAdsrNodeData, finished),
            },
            FieldInfoData {
                name: "AlwaysStartFromZero",
                name_hash: 2877047584,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DiceAdsrNodeData, always_start_from_zero),
            },
            FieldInfoData {
                name: "Scale",
                name_hash: 231223453,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DiceAdsrNodeData, scale),
            },
        ],
    }),
    array_type: Some(DICEADSRNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DiceAdsrNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        DICEADSRNODEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DICEADSRNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceAdsrNodeData-Array",
    name_hash: 3380166158,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceAdsrNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CounterNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
    pub increment: super::audio::AudioGraphNodePort,
    pub decrement: super::audio::AudioGraphNodePort,
    pub reset: super::audio::AudioGraphNodePort,
    pub has_changed: super::audio::AudioGraphNodePort,
    pub output_value: super::audio::AudioGraphNodePort,
    pub start_value: f32,
    pub count_step: f32,
}

pub trait CounterNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
    fn increment(&self) -> &super::audio::AudioGraphNodePort;
    fn increment_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn decrement(&self) -> &super::audio::AudioGraphNodePort;
    fn decrement_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn reset(&self) -> &super::audio::AudioGraphNodePort;
    fn reset_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn has_changed(&self) -> &super::audio::AudioGraphNodePort;
    fn has_changed_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn output_value(&self) -> &super::audio::AudioGraphNodePort;
    fn output_value_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn start_value(&self) -> &f32;
    fn start_value_mut(&mut self) -> &mut f32;
    fn count_step(&self) -> &f32;
    fn count_step_mut(&mut self) -> &mut f32;
}

impl CounterNodeDataTrait for CounterNodeData {
    fn increment(&self) -> &super::audio::AudioGraphNodePort {
        &self.increment
    }
    fn increment_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.increment
    }
    fn decrement(&self) -> &super::audio::AudioGraphNodePort {
        &self.decrement
    }
    fn decrement_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.decrement
    }
    fn reset(&self) -> &super::audio::AudioGraphNodePort {
        &self.reset
    }
    fn reset_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.reset
    }
    fn has_changed(&self) -> &super::audio::AudioGraphNodePort {
        &self.has_changed
    }
    fn has_changed_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.has_changed
    }
    fn output_value(&self) -> &super::audio::AudioGraphNodePort {
        &self.output_value
    }
    fn output_value_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.output_value
    }
    fn start_value(&self) -> &f32 {
        &self.start_value
    }
    fn start_value_mut(&mut self) -> &mut f32 {
        &mut self.start_value
    }
    fn count_step(&self) -> &f32 {
        &self.count_step
    }
    fn count_step_mut(&mut self) -> &mut f32 {
        &mut self.count_step
    }
}

impl super::audio::AudioGraphNodeDataTrait for CounterNodeData {
}

impl super::core::DataContainerTrait for CounterNodeData {
}

pub static COUNTERNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CounterNodeData",
    name_hash: 3382684897,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        super_class_offset: offset_of!(CounterNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CounterNodeData as Default>::default())),
            create_boxed: || Box::new(<CounterNodeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Increment",
                name_hash: 1736970756,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(CounterNodeData, increment),
            },
            FieldInfoData {
                name: "Decrement",
                name_hash: 3899315554,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(CounterNodeData, decrement),
            },
            FieldInfoData {
                name: "Reset",
                name_hash: 229946160,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(CounterNodeData, reset),
            },
            FieldInfoData {
                name: "HasChanged",
                name_hash: 2470958941,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(CounterNodeData, has_changed),
            },
            FieldInfoData {
                name: "OutputValue",
                name_hash: 731416433,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(CounterNodeData, output_value),
            },
            FieldInfoData {
                name: "StartValue",
                name_hash: 2748522638,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CounterNodeData, start_value),
            },
            FieldInfoData {
                name: "CountStep",
                name_hash: 1973422516,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CounterNodeData, count_step),
            },
        ],
    }),
    array_type: Some(COUNTERNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CounterNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        COUNTERNODEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static COUNTERNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CounterNodeData-Array",
    name_hash: 1992542165,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("CounterNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ConfigurableRangeMapperNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
    pub input_value: super::audio::AudioGraphNodePort,
    pub output_value: super::audio::AudioGraphNodePort,
    pub default_output_value: super::audio::AudioGraphNodePort,
    pub default_output_value_enabled: bool,
    pub ranges: Vec<Option<LockedTypeObject /* ConfigurableRangeMapperEntry */>>,
}

pub trait ConfigurableRangeMapperNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
    fn input_value(&self) -> &super::audio::AudioGraphNodePort;
    fn input_value_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn output_value(&self) -> &super::audio::AudioGraphNodePort;
    fn output_value_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn default_output_value(&self) -> &super::audio::AudioGraphNodePort;
    fn default_output_value_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn default_output_value_enabled(&self) -> &bool;
    fn default_output_value_enabled_mut(&mut self) -> &mut bool;
    fn ranges(&self) -> &Vec<Option<LockedTypeObject /* ConfigurableRangeMapperEntry */>>;
    fn ranges_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* ConfigurableRangeMapperEntry */>>;
}

impl ConfigurableRangeMapperNodeDataTrait for ConfigurableRangeMapperNodeData {
    fn input_value(&self) -> &super::audio::AudioGraphNodePort {
        &self.input_value
    }
    fn input_value_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.input_value
    }
    fn output_value(&self) -> &super::audio::AudioGraphNodePort {
        &self.output_value
    }
    fn output_value_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.output_value
    }
    fn default_output_value(&self) -> &super::audio::AudioGraphNodePort {
        &self.default_output_value
    }
    fn default_output_value_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.default_output_value
    }
    fn default_output_value_enabled(&self) -> &bool {
        &self.default_output_value_enabled
    }
    fn default_output_value_enabled_mut(&mut self) -> &mut bool {
        &mut self.default_output_value_enabled
    }
    fn ranges(&self) -> &Vec<Option<LockedTypeObject /* ConfigurableRangeMapperEntry */>> {
        &self.ranges
    }
    fn ranges_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* ConfigurableRangeMapperEntry */>> {
        &mut self.ranges
    }
}

impl super::audio::AudioGraphNodeDataTrait for ConfigurableRangeMapperNodeData {
}

impl super::core::DataContainerTrait for ConfigurableRangeMapperNodeData {
}

pub static CONFIGURABLERANGEMAPPERNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConfigurableRangeMapperNodeData",
    name_hash: 593716374,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        super_class_offset: offset_of!(ConfigurableRangeMapperNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ConfigurableRangeMapperNodeData as Default>::default())),
            create_boxed: || Box::new(<ConfigurableRangeMapperNodeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "InputValue",
                name_hash: 1591366968,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ConfigurableRangeMapperNodeData, input_value),
            },
            FieldInfoData {
                name: "OutputValue",
                name_hash: 731416433,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ConfigurableRangeMapperNodeData, output_value),
            },
            FieldInfoData {
                name: "DefaultOutputValue",
                name_hash: 2261994170,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ConfigurableRangeMapperNodeData, default_output_value),
            },
            FieldInfoData {
                name: "DefaultOutputValueEnabled",
                name_hash: 4075741855,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ConfigurableRangeMapperNodeData, default_output_value_enabled),
            },
            FieldInfoData {
                name: "Ranges",
                name_hash: 3298755849,
                flags: MemberInfoFlags::new(144),
                field_type: "ConfigurableRangeMapperEntry-Array",
                rust_offset: offset_of!(ConfigurableRangeMapperNodeData, ranges),
            },
        ],
    }),
    array_type: Some(CONFIGURABLERANGEMAPPERNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ConfigurableRangeMapperNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        CONFIGURABLERANGEMAPPERNODEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CONFIGURABLERANGEMAPPERNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConfigurableRangeMapperNodeData-Array",
    name_hash: 228202914,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ConfigurableRangeMapperNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ConfigurableRangeMapperEntry {
    pub _glacier_base: super::audio::AudioGraphNodePortGroup,
    pub range_start: super::audio::AudioGraphNodePort,
    pub range_end: super::audio::AudioGraphNodePort,
    pub output_value: super::audio::AudioGraphNodePort,
}

pub trait ConfigurableRangeMapperEntryTrait: super::audio::AudioGraphNodePortGroupTrait {
    fn range_start(&self) -> &super::audio::AudioGraphNodePort;
    fn range_start_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn range_end(&self) -> &super::audio::AudioGraphNodePort;
    fn range_end_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn output_value(&self) -> &super::audio::AudioGraphNodePort;
    fn output_value_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
}

impl ConfigurableRangeMapperEntryTrait for ConfigurableRangeMapperEntry {
    fn range_start(&self) -> &super::audio::AudioGraphNodePort {
        &self.range_start
    }
    fn range_start_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.range_start
    }
    fn range_end(&self) -> &super::audio::AudioGraphNodePort {
        &self.range_end
    }
    fn range_end_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.range_end
    }
    fn output_value(&self) -> &super::audio::AudioGraphNodePort {
        &self.output_value
    }
    fn output_value_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.output_value
    }
}

impl super::audio::AudioGraphNodePortGroupTrait for ConfigurableRangeMapperEntry {
}

impl super::core::DataContainerTrait for ConfigurableRangeMapperEntry {
}

pub static CONFIGURABLERANGEMAPPERENTRY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConfigurableRangeMapperEntry",
    name_hash: 3876249042,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEPORTGROUP_TYPE_INFO),
        super_class_offset: offset_of!(ConfigurableRangeMapperEntry, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ConfigurableRangeMapperEntry as Default>::default())),
            create_boxed: || Box::new(<ConfigurableRangeMapperEntry as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "RangeStart",
                name_hash: 1501057914,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ConfigurableRangeMapperEntry, range_start),
            },
            FieldInfoData {
                name: "RangeEnd",
                name_hash: 1752521717,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ConfigurableRangeMapperEntry, range_end),
            },
            FieldInfoData {
                name: "OutputValue",
                name_hash: 731416433,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ConfigurableRangeMapperEntry, output_value),
            },
        ],
    }),
    array_type: Some(CONFIGURABLERANGEMAPPERENTRY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ConfigurableRangeMapperEntry {
    fn type_info(&self) -> &'static TypeInfo {
        CONFIGURABLERANGEMAPPERENTRY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CONFIGURABLERANGEMAPPERENTRY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConfigurableRangeMapperEntry-Array",
    name_hash: 534254950,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ConfigurableRangeMapperEntry"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ConditionValueNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
    pub conditions: Vec<Option<LockedTypeObject /* ConditionValueGroup */>>,
}

pub trait ConditionValueNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
    fn conditions(&self) -> &Vec<Option<LockedTypeObject /* ConditionValueGroup */>>;
    fn conditions_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* ConditionValueGroup */>>;
}

impl ConditionValueNodeDataTrait for ConditionValueNodeData {
    fn conditions(&self) -> &Vec<Option<LockedTypeObject /* ConditionValueGroup */>> {
        &self.conditions
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* ConditionValueGroup */>> {
        &mut self.conditions
    }
}

impl super::audio::AudioGraphNodeDataTrait for ConditionValueNodeData {
}

impl super::core::DataContainerTrait for ConditionValueNodeData {
}

pub static CONDITIONVALUENODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionValueNodeData",
    name_hash: 458375597,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        super_class_offset: offset_of!(ConditionValueNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ConditionValueNodeData as Default>::default())),
            create_boxed: || Box::new(<ConditionValueNodeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Conditions",
                name_hash: 3586042181,
                flags: MemberInfoFlags::new(144),
                field_type: "ConditionValueGroup-Array",
                rust_offset: offset_of!(ConditionValueNodeData, conditions),
            },
        ],
    }),
    array_type: Some(CONDITIONVALUENODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ConditionValueNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        CONDITIONVALUENODEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CONDITIONVALUENODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionValueNodeData-Array",
    name_hash: 2268182041,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ConditionValueNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ConditionValueGroup {
    pub _glacier_base: super::audio::AudioGraphNodePortGroup,
    pub x: super::audio::AudioGraphNodePort,
    pub y: super::audio::AudioGraphNodePort,
    pub out: super::audio::AudioGraphNodePort,
    pub condition: ConditionValueType,
    pub value_if_true: f32,
    pub value_if_false: f32,
}

pub trait ConditionValueGroupTrait: super::audio::AudioGraphNodePortGroupTrait {
    fn x(&self) -> &super::audio::AudioGraphNodePort;
    fn x_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn y(&self) -> &super::audio::AudioGraphNodePort;
    fn y_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn out(&self) -> &super::audio::AudioGraphNodePort;
    fn out_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn condition(&self) -> &ConditionValueType;
    fn condition_mut(&mut self) -> &mut ConditionValueType;
    fn value_if_true(&self) -> &f32;
    fn value_if_true_mut(&mut self) -> &mut f32;
    fn value_if_false(&self) -> &f32;
    fn value_if_false_mut(&mut self) -> &mut f32;
}

impl ConditionValueGroupTrait for ConditionValueGroup {
    fn x(&self) -> &super::audio::AudioGraphNodePort {
        &self.x
    }
    fn x_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.x
    }
    fn y(&self) -> &super::audio::AudioGraphNodePort {
        &self.y
    }
    fn y_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.y
    }
    fn out(&self) -> &super::audio::AudioGraphNodePort {
        &self.out
    }
    fn out_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.out
    }
    fn condition(&self) -> &ConditionValueType {
        &self.condition
    }
    fn condition_mut(&mut self) -> &mut ConditionValueType {
        &mut self.condition
    }
    fn value_if_true(&self) -> &f32 {
        &self.value_if_true
    }
    fn value_if_true_mut(&mut self) -> &mut f32 {
        &mut self.value_if_true
    }
    fn value_if_false(&self) -> &f32 {
        &self.value_if_false
    }
    fn value_if_false_mut(&mut self) -> &mut f32 {
        &mut self.value_if_false
    }
}

impl super::audio::AudioGraphNodePortGroupTrait for ConditionValueGroup {
}

impl super::core::DataContainerTrait for ConditionValueGroup {
}

pub static CONDITIONVALUEGROUP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionValueGroup",
    name_hash: 1801563298,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEPORTGROUP_TYPE_INFO),
        super_class_offset: offset_of!(ConditionValueGroup, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ConditionValueGroup as Default>::default())),
            create_boxed: || Box::new(<ConditionValueGroup as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "X",
                name_hash: 177661,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ConditionValueGroup, x),
            },
            FieldInfoData {
                name: "Y",
                name_hash: 177660,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ConditionValueGroup, y),
            },
            FieldInfoData {
                name: "Out",
                name_hash: 193453899,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ConditionValueGroup, out),
            },
            FieldInfoData {
                name: "Condition",
                name_hash: 1800624758,
                flags: MemberInfoFlags::new(0),
                field_type: "ConditionValueType",
                rust_offset: offset_of!(ConditionValueGroup, condition),
            },
            FieldInfoData {
                name: "ValueIfTrue",
                name_hash: 88991863,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ConditionValueGroup, value_if_true),
            },
            FieldInfoData {
                name: "ValueIfFalse",
                name_hash: 2915912540,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ConditionValueGroup, value_if_false),
            },
        ],
    }),
    array_type: Some(CONDITIONVALUEGROUP_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ConditionValueGroup {
    fn type_info(&self) -> &'static TypeInfo {
        CONDITIONVALUEGROUP_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CONDITIONVALUEGROUP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionValueGroup-Array",
    name_hash: 1602718486,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ConditionValueGroup"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ConditionValueType {
    #[default]
    ConditionValueType_Equal = 0,
    ConditionValueType_NotEqual = 1,
    ConditionValueType_Less = 2,
    ConditionValueType_Greater = 3,
    ConditionValueType_LessOrEqual = 4,
    ConditionValueType_GreaterOrEqual = 5,
    ConditionValueType_AND = 6,
    ConditionValueType_OR = 7,
}

pub static CONDITIONVALUETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionValueType",
    name_hash: 1485461637,
    flags: MemberInfoFlags::new(49429),
    module: "DiceCommonsShared",
    data: TypeInfoData::Enum,
    array_type: Some(CONDITIONVALUETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ConditionValueType {
    fn type_info(&self) -> &'static TypeInfo {
        CONDITIONVALUETYPE_TYPE_INFO
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


pub static CONDITIONVALUETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionValueType-Array",
    name_hash: 3685998641,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ConditionValueType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CompareValueNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
    pub conditions: Vec<Option<LockedTypeObject /* CompareValueGroup */>>,
}

pub trait CompareValueNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
    fn conditions(&self) -> &Vec<Option<LockedTypeObject /* CompareValueGroup */>>;
    fn conditions_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* CompareValueGroup */>>;
}

impl CompareValueNodeDataTrait for CompareValueNodeData {
    fn conditions(&self) -> &Vec<Option<LockedTypeObject /* CompareValueGroup */>> {
        &self.conditions
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* CompareValueGroup */>> {
        &mut self.conditions
    }
}

impl super::audio::AudioGraphNodeDataTrait for CompareValueNodeData {
}

impl super::core::DataContainerTrait for CompareValueNodeData {
}

pub static COMPAREVALUENODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareValueNodeData",
    name_hash: 72138777,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        super_class_offset: offset_of!(CompareValueNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CompareValueNodeData as Default>::default())),
            create_boxed: || Box::new(<CompareValueNodeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Conditions",
                name_hash: 3586042181,
                flags: MemberInfoFlags::new(144),
                field_type: "CompareValueGroup-Array",
                rust_offset: offset_of!(CompareValueNodeData, conditions),
            },
        ],
    }),
    array_type: Some(COMPAREVALUENODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CompareValueNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        COMPAREVALUENODEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static COMPAREVALUENODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareValueNodeData-Array",
    name_hash: 3981991341,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("CompareValueNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CompareValueGroup {
    pub _glacier_base: super::audio::AudioGraphNodePortGroup,
    pub evaluate: super::audio::AudioGraphNodePort,
    pub x: super::audio::AudioGraphNodePort,
    pub y: super::audio::AudioGraphNodePort,
    pub r#true: super::audio::AudioGraphNodePort,
    pub r#false: super::audio::AudioGraphNodePort,
    pub condition: CompareValueConditionType,
    pub auto_evaluate: bool,
}

pub trait CompareValueGroupTrait: super::audio::AudioGraphNodePortGroupTrait {
    fn evaluate(&self) -> &super::audio::AudioGraphNodePort;
    fn evaluate_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn x(&self) -> &super::audio::AudioGraphNodePort;
    fn x_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn y(&self) -> &super::audio::AudioGraphNodePort;
    fn y_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn r#true(&self) -> &super::audio::AudioGraphNodePort;
    fn r#true_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn r#false(&self) -> &super::audio::AudioGraphNodePort;
    fn r#false_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn condition(&self) -> &CompareValueConditionType;
    fn condition_mut(&mut self) -> &mut CompareValueConditionType;
    fn auto_evaluate(&self) -> &bool;
    fn auto_evaluate_mut(&mut self) -> &mut bool;
}

impl CompareValueGroupTrait for CompareValueGroup {
    fn evaluate(&self) -> &super::audio::AudioGraphNodePort {
        &self.evaluate
    }
    fn evaluate_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.evaluate
    }
    fn x(&self) -> &super::audio::AudioGraphNodePort {
        &self.x
    }
    fn x_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.x
    }
    fn y(&self) -> &super::audio::AudioGraphNodePort {
        &self.y
    }
    fn y_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.y
    }
    fn r#true(&self) -> &super::audio::AudioGraphNodePort {
        &self.r#true
    }
    fn r#true_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.r#true
    }
    fn r#false(&self) -> &super::audio::AudioGraphNodePort {
        &self.r#false
    }
    fn r#false_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.r#false
    }
    fn condition(&self) -> &CompareValueConditionType {
        &self.condition
    }
    fn condition_mut(&mut self) -> &mut CompareValueConditionType {
        &mut self.condition
    }
    fn auto_evaluate(&self) -> &bool {
        &self.auto_evaluate
    }
    fn auto_evaluate_mut(&mut self) -> &mut bool {
        &mut self.auto_evaluate
    }
}

impl super::audio::AudioGraphNodePortGroupTrait for CompareValueGroup {
}

impl super::core::DataContainerTrait for CompareValueGroup {
}

pub static COMPAREVALUEGROUP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareValueGroup",
    name_hash: 181092630,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEPORTGROUP_TYPE_INFO),
        super_class_offset: offset_of!(CompareValueGroup, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CompareValueGroup as Default>::default())),
            create_boxed: || Box::new(<CompareValueGroup as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Evaluate",
                name_hash: 1831937182,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(CompareValueGroup, evaluate),
            },
            FieldInfoData {
                name: "X",
                name_hash: 177661,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(CompareValueGroup, x),
            },
            FieldInfoData {
                name: "Y",
                name_hash: 177660,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(CompareValueGroup, y),
            },
            FieldInfoData {
                name: "True",
                name_hash: 2089293587,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(CompareValueGroup, r#true),
            },
            FieldInfoData {
                name: "False",
                name_hash: 206401336,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(CompareValueGroup, r#false),
            },
            FieldInfoData {
                name: "Condition",
                name_hash: 1800624758,
                flags: MemberInfoFlags::new(0),
                field_type: "CompareValueConditionType",
                rust_offset: offset_of!(CompareValueGroup, condition),
            },
            FieldInfoData {
                name: "AutoEvaluate",
                name_hash: 110934545,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CompareValueGroup, auto_evaluate),
            },
        ],
    }),
    array_type: Some(COMPAREVALUEGROUP_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CompareValueGroup {
    fn type_info(&self) -> &'static TypeInfo {
        COMPAREVALUEGROUP_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static COMPAREVALUEGROUP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareValueGroup-Array",
    name_hash: 798316066,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("CompareValueGroup"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum CompareValueConditionType {
    #[default]
    CompareValueConditionType_Equal = 0,
    CompareValueConditionType_NotEqual = 1,
    CompareValueConditionType_Less = 2,
    CompareValueConditionType_Greater = 3,
    CompareValueConditionType_LessOrEqual = 4,
    CompareValueConditionType_GreaterOrEqual = 5,
    CompareValueConditionType_AND = 6,
    CompareValueConditionType_OR = 7,
}

pub static COMPAREVALUECONDITIONTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareValueConditionType",
    name_hash: 947219202,
    flags: MemberInfoFlags::new(49429),
    module: "DiceCommonsShared",
    data: TypeInfoData::Enum,
    array_type: Some(COMPAREVALUECONDITIONTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CompareValueConditionType {
    fn type_info(&self) -> &'static TypeInfo {
        COMPAREVALUECONDITIONTYPE_TYPE_INFO
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


pub static COMPAREVALUECONDITIONTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareValueConditionType-Array",
    name_hash: 2447673910,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("CompareValueConditionType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClampNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
    pub r#in: super::audio::AudioGraphNodePort,
    pub out: super::audio::AudioGraphNodePort,
    pub clamp_min: f32,
    pub clamp_max: f32,
}

pub trait ClampNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
    fn r#in(&self) -> &super::audio::AudioGraphNodePort;
    fn r#in_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn out(&self) -> &super::audio::AudioGraphNodePort;
    fn out_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn clamp_min(&self) -> &f32;
    fn clamp_min_mut(&mut self) -> &mut f32;
    fn clamp_max(&self) -> &f32;
    fn clamp_max_mut(&mut self) -> &mut f32;
}

impl ClampNodeDataTrait for ClampNodeData {
    fn r#in(&self) -> &super::audio::AudioGraphNodePort {
        &self.r#in
    }
    fn r#in_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.r#in
    }
    fn out(&self) -> &super::audio::AudioGraphNodePort {
        &self.out
    }
    fn out_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.out
    }
    fn clamp_min(&self) -> &f32 {
        &self.clamp_min
    }
    fn clamp_min_mut(&mut self) -> &mut f32 {
        &mut self.clamp_min
    }
    fn clamp_max(&self) -> &f32 {
        &self.clamp_max
    }
    fn clamp_max_mut(&mut self) -> &mut f32 {
        &mut self.clamp_max
    }
}

impl super::audio::AudioGraphNodeDataTrait for ClampNodeData {
}

impl super::core::DataContainerTrait for ClampNodeData {
}

pub static CLAMPNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClampNodeData",
    name_hash: 236878406,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        super_class_offset: offset_of!(ClampNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClampNodeData as Default>::default())),
            create_boxed: || Box::new(<ClampNodeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "In",
                name_hash: 5862146,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ClampNodeData, r#in),
            },
            FieldInfoData {
                name: "Out",
                name_hash: 193453899,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ClampNodeData, out),
            },
            FieldInfoData {
                name: "ClampMin",
                name_hash: 722785724,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ClampNodeData, clamp_min),
            },
            FieldInfoData {
                name: "ClampMax",
                name_hash: 722785954,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ClampNodeData, clamp_max),
            },
        ],
    }),
    array_type: Some(CLAMPNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ClampNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        CLAMPNODEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLAMPNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClampNodeData-Array",
    name_hash: 1425799154,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ClampNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AudioEnvelopeSwitcherNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
    pub index: super::audio::AudioGraphNodePort,
    pub advance: super::audio::AudioGraphNodePort,
    pub audio_envelope: super::audio::AudioGraphNodePort,
    pub index_changed: super::audio::AudioGraphNodePort,
    pub audio_envelopes: Vec<Option<LockedTypeObject /* AudioEnvelopeAsset */>>,
    pub default_index: f32,
    pub is_random: bool,
    pub random_start_index: bool,
}

pub trait AudioEnvelopeSwitcherNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
    fn index(&self) -> &super::audio::AudioGraphNodePort;
    fn index_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn advance(&self) -> &super::audio::AudioGraphNodePort;
    fn advance_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn audio_envelope(&self) -> &super::audio::AudioGraphNodePort;
    fn audio_envelope_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn index_changed(&self) -> &super::audio::AudioGraphNodePort;
    fn index_changed_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn audio_envelopes(&self) -> &Vec<Option<LockedTypeObject /* AudioEnvelopeAsset */>>;
    fn audio_envelopes_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* AudioEnvelopeAsset */>>;
    fn default_index(&self) -> &f32;
    fn default_index_mut(&mut self) -> &mut f32;
    fn is_random(&self) -> &bool;
    fn is_random_mut(&mut self) -> &mut bool;
    fn random_start_index(&self) -> &bool;
    fn random_start_index_mut(&mut self) -> &mut bool;
}

impl AudioEnvelopeSwitcherNodeDataTrait for AudioEnvelopeSwitcherNodeData {
    fn index(&self) -> &super::audio::AudioGraphNodePort {
        &self.index
    }
    fn index_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.index
    }
    fn advance(&self) -> &super::audio::AudioGraphNodePort {
        &self.advance
    }
    fn advance_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.advance
    }
    fn audio_envelope(&self) -> &super::audio::AudioGraphNodePort {
        &self.audio_envelope
    }
    fn audio_envelope_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.audio_envelope
    }
    fn index_changed(&self) -> &super::audio::AudioGraphNodePort {
        &self.index_changed
    }
    fn index_changed_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.index_changed
    }
    fn audio_envelopes(&self) -> &Vec<Option<LockedTypeObject /* AudioEnvelopeAsset */>> {
        &self.audio_envelopes
    }
    fn audio_envelopes_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* AudioEnvelopeAsset */>> {
        &mut self.audio_envelopes
    }
    fn default_index(&self) -> &f32 {
        &self.default_index
    }
    fn default_index_mut(&mut self) -> &mut f32 {
        &mut self.default_index
    }
    fn is_random(&self) -> &bool {
        &self.is_random
    }
    fn is_random_mut(&mut self) -> &mut bool {
        &mut self.is_random
    }
    fn random_start_index(&self) -> &bool {
        &self.random_start_index
    }
    fn random_start_index_mut(&mut self) -> &mut bool {
        &mut self.random_start_index
    }
}

impl super::audio::AudioGraphNodeDataTrait for AudioEnvelopeSwitcherNodeData {
}

impl super::core::DataContainerTrait for AudioEnvelopeSwitcherNodeData {
}

pub static AUDIOENVELOPESWITCHERNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioEnvelopeSwitcherNodeData",
    name_hash: 1142898184,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        super_class_offset: offset_of!(AudioEnvelopeSwitcherNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AudioEnvelopeSwitcherNodeData as Default>::default())),
            create_boxed: || Box::new(<AudioEnvelopeSwitcherNodeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Index",
                name_hash: 214509467,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(AudioEnvelopeSwitcherNodeData, index),
            },
            FieldInfoData {
                name: "Advance",
                name_hash: 343579199,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(AudioEnvelopeSwitcherNodeData, advance),
            },
            FieldInfoData {
                name: "AudioEnvelope",
                name_hash: 4008589053,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(AudioEnvelopeSwitcherNodeData, audio_envelope),
            },
            FieldInfoData {
                name: "IndexChanged",
                name_hash: 3560418393,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(AudioEnvelopeSwitcherNodeData, index_changed),
            },
            FieldInfoData {
                name: "AudioEnvelopes",
                name_hash: 3434419950,
                flags: MemberInfoFlags::new(144),
                field_type: "AudioEnvelopeAsset-Array",
                rust_offset: offset_of!(AudioEnvelopeSwitcherNodeData, audio_envelopes),
            },
            FieldInfoData {
                name: "DefaultIndex",
                name_hash: 2048165968,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AudioEnvelopeSwitcherNodeData, default_index),
            },
            FieldInfoData {
                name: "IsRandom",
                name_hash: 421699588,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AudioEnvelopeSwitcherNodeData, is_random),
            },
            FieldInfoData {
                name: "RandomStartIndex",
                name_hash: 3711152288,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AudioEnvelopeSwitcherNodeData, random_start_index),
            },
        ],
    }),
    array_type: Some(AUDIOENVELOPESWITCHERNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AudioEnvelopeSwitcherNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        AUDIOENVELOPESWITCHERNODEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static AUDIOENVELOPESWITCHERNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioEnvelopeSwitcherNodeData-Array",
    name_hash: 3778265148,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("AudioEnvelopeSwitcherNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AudioEnvelopeSwitcherNodeConfigData {
    pub _glacier_base: super::audio::AudioGraphNodeConfigData,
    pub audio_envelopes: Vec<Option<LockedTypeObject /* AudioEnvelopeAsset */>>,
}

pub trait AudioEnvelopeSwitcherNodeConfigDataTrait: super::audio::AudioGraphNodeConfigDataTrait {
    fn audio_envelopes(&self) -> &Vec<Option<LockedTypeObject /* AudioEnvelopeAsset */>>;
    fn audio_envelopes_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* AudioEnvelopeAsset */>>;
}

impl AudioEnvelopeSwitcherNodeConfigDataTrait for AudioEnvelopeSwitcherNodeConfigData {
    fn audio_envelopes(&self) -> &Vec<Option<LockedTypeObject /* AudioEnvelopeAsset */>> {
        &self.audio_envelopes
    }
    fn audio_envelopes_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* AudioEnvelopeAsset */>> {
        &mut self.audio_envelopes
    }
}

impl super::audio::AudioGraphNodeConfigDataTrait for AudioEnvelopeSwitcherNodeConfigData {
    fn node(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphNodeData */> {
        self._glacier_base.node()
    }
    fn node_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphNodeData */> {
        self._glacier_base.node_mut()
    }
    fn configured_property_flags(&self) -> &u64 {
        self._glacier_base.configured_property_flags()
    }
    fn configured_property_flags_mut(&mut self) -> &mut u64 {
        self._glacier_base.configured_property_flags_mut()
    }
}

impl super::core::DataContainerTrait for AudioEnvelopeSwitcherNodeConfigData {
}

pub static AUDIOENVELOPESWITCHERNODECONFIGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioEnvelopeSwitcherNodeConfigData",
    name_hash: 3427925730,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODECONFIGDATA_TYPE_INFO),
        super_class_offset: offset_of!(AudioEnvelopeSwitcherNodeConfigData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AudioEnvelopeSwitcherNodeConfigData as Default>::default())),
            create_boxed: || Box::new(<AudioEnvelopeSwitcherNodeConfigData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "AudioEnvelopes",
                name_hash: 3434419950,
                flags: MemberInfoFlags::new(144),
                field_type: "AudioEnvelopeAsset-Array",
                rust_offset: offset_of!(AudioEnvelopeSwitcherNodeConfigData, audio_envelopes),
            },
        ],
    }),
    array_type: Some(AUDIOENVELOPESWITCHERNODECONFIGDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AudioEnvelopeSwitcherNodeConfigData {
    fn type_info(&self) -> &'static TypeInfo {
        AUDIOENVELOPESWITCHERNODECONFIGDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static AUDIOENVELOPESWITCHERNODECONFIGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioEnvelopeSwitcherNodeConfigData-Array",
    name_hash: 288116182,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("AudioEnvelopeSwitcherNodeConfigData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AudioEnvelopeNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
    pub x: super::audio::AudioGraphNodePort,
    pub envelope_in: super::audio::AudioGraphNodePort,
    pub y: super::audio::AudioGraphNodePort,
    pub region: super::audio::AudioGraphNodePort,
    pub envelope: AudioEnvelope,
    pub envelope_asset: Option<LockedTypeObject /* AudioEnvelopeAsset */>,
}

pub trait AudioEnvelopeNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
    fn x(&self) -> &super::audio::AudioGraphNodePort;
    fn x_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn envelope_in(&self) -> &super::audio::AudioGraphNodePort;
    fn envelope_in_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn y(&self) -> &super::audio::AudioGraphNodePort;
    fn y_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn region(&self) -> &super::audio::AudioGraphNodePort;
    fn region_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn envelope(&self) -> &AudioEnvelope;
    fn envelope_mut(&mut self) -> &mut AudioEnvelope;
    fn envelope_asset(&self) -> &Option<LockedTypeObject /* AudioEnvelopeAsset */>;
    fn envelope_asset_mut(&mut self) -> &mut Option<LockedTypeObject /* AudioEnvelopeAsset */>;
}

impl AudioEnvelopeNodeDataTrait for AudioEnvelopeNodeData {
    fn x(&self) -> &super::audio::AudioGraphNodePort {
        &self.x
    }
    fn x_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.x
    }
    fn envelope_in(&self) -> &super::audio::AudioGraphNodePort {
        &self.envelope_in
    }
    fn envelope_in_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.envelope_in
    }
    fn y(&self) -> &super::audio::AudioGraphNodePort {
        &self.y
    }
    fn y_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.y
    }
    fn region(&self) -> &super::audio::AudioGraphNodePort {
        &self.region
    }
    fn region_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.region
    }
    fn envelope(&self) -> &AudioEnvelope {
        &self.envelope
    }
    fn envelope_mut(&mut self) -> &mut AudioEnvelope {
        &mut self.envelope
    }
    fn envelope_asset(&self) -> &Option<LockedTypeObject /* AudioEnvelopeAsset */> {
        &self.envelope_asset
    }
    fn envelope_asset_mut(&mut self) -> &mut Option<LockedTypeObject /* AudioEnvelopeAsset */> {
        &mut self.envelope_asset
    }
}

impl super::audio::AudioGraphNodeDataTrait for AudioEnvelopeNodeData {
}

impl super::core::DataContainerTrait for AudioEnvelopeNodeData {
}

pub static AUDIOENVELOPENODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioEnvelopeNodeData",
    name_hash: 875857581,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        super_class_offset: offset_of!(AudioEnvelopeNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AudioEnvelopeNodeData as Default>::default())),
            create_boxed: || Box::new(<AudioEnvelopeNodeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "X",
                name_hash: 177661,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(AudioEnvelopeNodeData, x),
            },
            FieldInfoData {
                name: "EnvelopeIn",
                name_hash: 2922457036,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(AudioEnvelopeNodeData, envelope_in),
            },
            FieldInfoData {
                name: "Y",
                name_hash: 177660,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(AudioEnvelopeNodeData, y),
            },
            FieldInfoData {
                name: "Region",
                name_hash: 3293978493,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(AudioEnvelopeNodeData, region),
            },
            FieldInfoData {
                name: "Envelope",
                name_hash: 365527499,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioEnvelope",
                rust_offset: offset_of!(AudioEnvelopeNodeData, envelope),
            },
            FieldInfoData {
                name: "EnvelopeAsset",
                name_hash: 3807726331,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioEnvelopeAsset",
                rust_offset: offset_of!(AudioEnvelopeNodeData, envelope_asset),
            },
        ],
    }),
    array_type: Some(AUDIOENVELOPENODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AudioEnvelopeNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        AUDIOENVELOPENODEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static AUDIOENVELOPENODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioEnvelopeNodeData-Array",
    name_hash: 3019094809,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("AudioEnvelopeNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AudioEnvelopeAsset {
    pub _glacier_base: super::core::Asset,
    pub envelope: AudioEnvelope,
}

pub trait AudioEnvelopeAssetTrait: super::core::AssetTrait {
    fn envelope(&self) -> &AudioEnvelope;
    fn envelope_mut(&mut self) -> &mut AudioEnvelope;
}

impl AudioEnvelopeAssetTrait for AudioEnvelopeAsset {
    fn envelope(&self) -> &AudioEnvelope {
        &self.envelope
    }
    fn envelope_mut(&mut self) -> &mut AudioEnvelope {
        &mut self.envelope
    }
}

impl super::core::AssetTrait for AudioEnvelopeAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for AudioEnvelopeAsset {
}

pub static AUDIOENVELOPEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioEnvelopeAsset",
    name_hash: 3167107789,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(AudioEnvelopeAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AudioEnvelopeAsset as Default>::default())),
            create_boxed: || Box::new(<AudioEnvelopeAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Envelope",
                name_hash: 365527499,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioEnvelope",
                rust_offset: offset_of!(AudioEnvelopeAsset, envelope),
            },
        ],
    }),
    array_type: Some(AUDIOENVELOPEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AudioEnvelopeAsset {
    fn type_info(&self) -> &'static TypeInfo {
        AUDIOENVELOPEASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static AUDIOENVELOPEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioEnvelopeAsset-Array",
    name_hash: 2719731449,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("AudioEnvelopeAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AudioEnvelope {
    pub points: Vec<BoxedTypeObject /* AudioEnvelopePoint */>,
    pub default_curve_type: AudioEnvelopeLineType,
}

pub trait AudioEnvelopeTrait: TypeObject {
    fn points(&self) -> &Vec<BoxedTypeObject /* AudioEnvelopePoint */>;
    fn points_mut(&mut self) -> &mut Vec<BoxedTypeObject /* AudioEnvelopePoint */>;
    fn default_curve_type(&self) -> &AudioEnvelopeLineType;
    fn default_curve_type_mut(&mut self) -> &mut AudioEnvelopeLineType;
}

impl AudioEnvelopeTrait for AudioEnvelope {
    fn points(&self) -> &Vec<BoxedTypeObject /* AudioEnvelopePoint */> {
        &self.points
    }
    fn points_mut(&mut self) -> &mut Vec<BoxedTypeObject /* AudioEnvelopePoint */> {
        &mut self.points
    }
    fn default_curve_type(&self) -> &AudioEnvelopeLineType {
        &self.default_curve_type
    }
    fn default_curve_type_mut(&mut self) -> &mut AudioEnvelopeLineType {
        &mut self.default_curve_type
    }
}

pub static AUDIOENVELOPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioEnvelope",
    name_hash: 4008589053,
    flags: MemberInfoFlags::new(73),
    module: "DiceCommonsShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AudioEnvelope as Default>::default())),
            create_boxed: || Box::new(<AudioEnvelope as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Points",
                name_hash: 3383606106,
                flags: MemberInfoFlags::new(144),
                field_type: "AudioEnvelopePoint-Array",
                rust_offset: offset_of!(AudioEnvelope, points),
            },
            FieldInfoData {
                name: "DefaultCurveType",
                name_hash: 3178483873,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioEnvelopeLineType",
                rust_offset: offset_of!(AudioEnvelope, default_curve_type),
            },
        ],
    }),
    array_type: Some(AUDIOENVELOPE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AudioEnvelope {
    fn type_info(&self) -> &'static TypeInfo {
        AUDIOENVELOPE_TYPE_INFO
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


pub static AUDIOENVELOPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioEnvelope-Array",
    name_hash: 4137919433,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("AudioEnvelope"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AudioEnvelopePoint {
    pub point: super::core::Vec2,
    pub line_type: AudioEnvelopeLineType,
    pub curve_scale: f32,
    pub is_region_boundary: bool,
}

pub trait AudioEnvelopePointTrait: TypeObject {
    fn point(&self) -> &super::core::Vec2;
    fn point_mut(&mut self) -> &mut super::core::Vec2;
    fn line_type(&self) -> &AudioEnvelopeLineType;
    fn line_type_mut(&mut self) -> &mut AudioEnvelopeLineType;
    fn curve_scale(&self) -> &f32;
    fn curve_scale_mut(&mut self) -> &mut f32;
    fn is_region_boundary(&self) -> &bool;
    fn is_region_boundary_mut(&mut self) -> &mut bool;
}

impl AudioEnvelopePointTrait for AudioEnvelopePoint {
    fn point(&self) -> &super::core::Vec2 {
        &self.point
    }
    fn point_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.point
    }
    fn line_type(&self) -> &AudioEnvelopeLineType {
        &self.line_type
    }
    fn line_type_mut(&mut self) -> &mut AudioEnvelopeLineType {
        &mut self.line_type
    }
    fn curve_scale(&self) -> &f32 {
        &self.curve_scale
    }
    fn curve_scale_mut(&mut self) -> &mut f32 {
        &mut self.curve_scale
    }
    fn is_region_boundary(&self) -> &bool {
        &self.is_region_boundary
    }
    fn is_region_boundary_mut(&mut self) -> &mut bool {
        &mut self.is_region_boundary
    }
}

pub static AUDIOENVELOPEPOINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioEnvelopePoint",
    name_hash: 3146953457,
    flags: MemberInfoFlags::new(36937),
    module: "DiceCommonsShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AudioEnvelopePoint as Default>::default())),
            create_boxed: || Box::new(<AudioEnvelopePoint as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Point",
                name_hash: 232684041,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(AudioEnvelopePoint, point),
            },
            FieldInfoData {
                name: "LineType",
                name_hash: 2762496595,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioEnvelopeLineType",
                rust_offset: offset_of!(AudioEnvelopePoint, line_type),
            },
            FieldInfoData {
                name: "CurveScale",
                name_hash: 1883346154,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AudioEnvelopePoint, curve_scale),
            },
            FieldInfoData {
                name: "IsRegionBoundary",
                name_hash: 2612688127,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AudioEnvelopePoint, is_region_boundary),
            },
        ],
    }),
    array_type: Some(AUDIOENVELOPEPOINT_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for AudioEnvelopePoint {
    fn type_info(&self) -> &'static TypeInfo {
        AUDIOENVELOPEPOINT_TYPE_INFO
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


pub static AUDIOENVELOPEPOINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioEnvelopePoint-Array",
    name_hash: 2146833861,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("AudioEnvelopePoint"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum SnapToGridGranularity {
    #[default]
    SnapToGridGranularity_10 = 10,
    SnapToGridGranularity_20 = 20,
    SnapToGridGranularity_30 = 30,
    SnapToGridGranularity_40 = 40,
    SnapToGridGranularity_50 = 50,
    SnapToGridGranularity_60 = 60,
    SnapToGridGranularity_70 = 70,
    SnapToGridGranularity_80 = 80,
    SnapToGridGranularity_90 = 90,
    SnapToGridGranularity_100 = 100,
}

pub static SNAPTOGRIDGRANULARITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnapToGridGranularity",
    name_hash: 1199894654,
    flags: MemberInfoFlags::new(49429),
    module: "DiceCommonsShared",
    data: TypeInfoData::Enum,
    array_type: Some(SNAPTOGRIDGRANULARITY_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SnapToGridGranularity {
    fn type_info(&self) -> &'static TypeInfo {
        SNAPTOGRIDGRANULARITY_TYPE_INFO
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


pub static SNAPTOGRIDGRANULARITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnapToGridGranularity-Array",
    name_hash: 3984601418,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("SnapToGridGranularity"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum AudioEnvelopeLineType {
    #[default]
    AudioEnvelopeLineType_Linear = 0,
    AudioEnvelopeLineType_Reciprocal = 1,
    AudioEnvelopeLineType_InverseReciprocal = 2,
    AudioEnvelopeLineType_SCurve = 3,
    AudioEnvelopeLineType_Sine = 4,
    AudioEnvelopeLineType_Exponential = 5,
}

pub static AUDIOENVELOPELINETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioEnvelopeLineType",
    name_hash: 1938686763,
    flags: MemberInfoFlags::new(49429),
    module: "DiceCommonsShared",
    data: TypeInfoData::Enum,
    array_type: Some(AUDIOENVELOPELINETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AudioEnvelopeLineType {
    fn type_info(&self) -> &'static TypeInfo {
        AUDIOENVELOPELINETYPE_TYPE_INFO
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


pub static AUDIOENVELOPELINETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioEnvelopeLineType-Array",
    name_hash: 2106679967,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("AudioEnvelopeLineType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AssetCrossfaderNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
    pub asset_in: super::audio::AudioGraphNodePort,
    pub crossfade_time: super::audio::AudioGraphNodePort,
    pub force_crossfade: super::audio::AudioGraphNodePort,
    pub asset_a: super::audio::AudioGraphNodePort,
    pub amplitude_a: super::audio::AudioGraphNodePort,
    pub trigger_a: super::audio::AudioGraphNodePort,
    pub release_a: super::audio::AudioGraphNodePort,
    pub asset_b: super::audio::AudioGraphNodePort,
    pub amplitude_b: super::audio::AudioGraphNodePort,
    pub trigger_b: super::audio::AudioGraphNodePort,
    pub release_b: super::audio::AudioGraphNodePort,
    pub auto_crossfade_on_asset_change: bool,
}

pub trait AssetCrossfaderNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
    fn asset_in(&self) -> &super::audio::AudioGraphNodePort;
    fn asset_in_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn crossfade_time(&self) -> &super::audio::AudioGraphNodePort;
    fn crossfade_time_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn force_crossfade(&self) -> &super::audio::AudioGraphNodePort;
    fn force_crossfade_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn asset_a(&self) -> &super::audio::AudioGraphNodePort;
    fn asset_a_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn amplitude_a(&self) -> &super::audio::AudioGraphNodePort;
    fn amplitude_a_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn trigger_a(&self) -> &super::audio::AudioGraphNodePort;
    fn trigger_a_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn release_a(&self) -> &super::audio::AudioGraphNodePort;
    fn release_a_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn asset_b(&self) -> &super::audio::AudioGraphNodePort;
    fn asset_b_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn amplitude_b(&self) -> &super::audio::AudioGraphNodePort;
    fn amplitude_b_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn trigger_b(&self) -> &super::audio::AudioGraphNodePort;
    fn trigger_b_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn release_b(&self) -> &super::audio::AudioGraphNodePort;
    fn release_b_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn auto_crossfade_on_asset_change(&self) -> &bool;
    fn auto_crossfade_on_asset_change_mut(&mut self) -> &mut bool;
}

impl AssetCrossfaderNodeDataTrait for AssetCrossfaderNodeData {
    fn asset_in(&self) -> &super::audio::AudioGraphNodePort {
        &self.asset_in
    }
    fn asset_in_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.asset_in
    }
    fn crossfade_time(&self) -> &super::audio::AudioGraphNodePort {
        &self.crossfade_time
    }
    fn crossfade_time_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.crossfade_time
    }
    fn force_crossfade(&self) -> &super::audio::AudioGraphNodePort {
        &self.force_crossfade
    }
    fn force_crossfade_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.force_crossfade
    }
    fn asset_a(&self) -> &super::audio::AudioGraphNodePort {
        &self.asset_a
    }
    fn asset_a_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.asset_a
    }
    fn amplitude_a(&self) -> &super::audio::AudioGraphNodePort {
        &self.amplitude_a
    }
    fn amplitude_a_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.amplitude_a
    }
    fn trigger_a(&self) -> &super::audio::AudioGraphNodePort {
        &self.trigger_a
    }
    fn trigger_a_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.trigger_a
    }
    fn release_a(&self) -> &super::audio::AudioGraphNodePort {
        &self.release_a
    }
    fn release_a_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.release_a
    }
    fn asset_b(&self) -> &super::audio::AudioGraphNodePort {
        &self.asset_b
    }
    fn asset_b_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.asset_b
    }
    fn amplitude_b(&self) -> &super::audio::AudioGraphNodePort {
        &self.amplitude_b
    }
    fn amplitude_b_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.amplitude_b
    }
    fn trigger_b(&self) -> &super::audio::AudioGraphNodePort {
        &self.trigger_b
    }
    fn trigger_b_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.trigger_b
    }
    fn release_b(&self) -> &super::audio::AudioGraphNodePort {
        &self.release_b
    }
    fn release_b_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.release_b
    }
    fn auto_crossfade_on_asset_change(&self) -> &bool {
        &self.auto_crossfade_on_asset_change
    }
    fn auto_crossfade_on_asset_change_mut(&mut self) -> &mut bool {
        &mut self.auto_crossfade_on_asset_change
    }
}

impl super::audio::AudioGraphNodeDataTrait for AssetCrossfaderNodeData {
}

impl super::core::DataContainerTrait for AssetCrossfaderNodeData {
}

pub static ASSETCROSSFADERNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AssetCrossfaderNodeData",
    name_hash: 421688431,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        super_class_offset: offset_of!(AssetCrossfaderNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AssetCrossfaderNodeData as Default>::default())),
            create_boxed: || Box::new(<AssetCrossfaderNodeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "AssetIn",
                name_hash: 969624626,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(AssetCrossfaderNodeData, asset_in),
            },
            FieldInfoData {
                name: "CrossfadeTime",
                name_hash: 3357526152,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(AssetCrossfaderNodeData, crossfade_time),
            },
            FieldInfoData {
                name: "ForceCrossfade",
                name_hash: 1478536256,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(AssetCrossfaderNodeData, force_crossfade),
            },
            FieldInfoData {
                name: "AssetA",
                name_hash: 2502242516,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(AssetCrossfaderNodeData, asset_a),
            },
            FieldInfoData {
                name: "AmplitudeA",
                name_hash: 1577794333,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(AssetCrossfaderNodeData, amplitude_a),
            },
            FieldInfoData {
                name: "TriggerA",
                name_hash: 110339612,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(AssetCrossfaderNodeData, trigger_a),
            },
            FieldInfoData {
                name: "ReleaseA",
                name_hash: 1114132429,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(AssetCrossfaderNodeData, release_a),
            },
            FieldInfoData {
                name: "AssetB",
                name_hash: 2502242519,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(AssetCrossfaderNodeData, asset_b),
            },
            FieldInfoData {
                name: "AmplitudeB",
                name_hash: 1577794334,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(AssetCrossfaderNodeData, amplitude_b),
            },
            FieldInfoData {
                name: "TriggerB",
                name_hash: 110339615,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(AssetCrossfaderNodeData, trigger_b),
            },
            FieldInfoData {
                name: "ReleaseB",
                name_hash: 1114132430,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(AssetCrossfaderNodeData, release_b),
            },
            FieldInfoData {
                name: "AutoCrossfadeOnAssetChange",
                name_hash: 3782600773,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AssetCrossfaderNodeData, auto_crossfade_on_asset_change),
            },
        ],
    }),
    array_type: Some(ASSETCROSSFADERNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AssetCrossfaderNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        ASSETCROSSFADERNODEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static ASSETCROSSFADERNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AssetCrossfaderNodeData-Array",
    name_hash: 3750326875,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("AssetCrossfaderNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ArOneShotNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
    pub start: super::audio::AudioGraphNodePort,
    pub attack: super::audio::AudioGraphNodePort,
    pub hold: super::audio::AudioGraphNodePort,
    pub release: super::audio::AudioGraphNodePort,
    pub power: f32,
    pub value: super::audio::AudioGraphNodePort,
    pub started: super::audio::AudioGraphNodePort,
    pub stopped: super::audio::AudioGraphNodePort,
}

pub trait ArOneShotNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
    fn start(&self) -> &super::audio::AudioGraphNodePort;
    fn start_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn attack(&self) -> &super::audio::AudioGraphNodePort;
    fn attack_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn hold(&self) -> &super::audio::AudioGraphNodePort;
    fn hold_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn release(&self) -> &super::audio::AudioGraphNodePort;
    fn release_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn power(&self) -> &f32;
    fn power_mut(&mut self) -> &mut f32;
    fn value(&self) -> &super::audio::AudioGraphNodePort;
    fn value_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn started(&self) -> &super::audio::AudioGraphNodePort;
    fn started_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn stopped(&self) -> &super::audio::AudioGraphNodePort;
    fn stopped_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
}

impl ArOneShotNodeDataTrait for ArOneShotNodeData {
    fn start(&self) -> &super::audio::AudioGraphNodePort {
        &self.start
    }
    fn start_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.start
    }
    fn attack(&self) -> &super::audio::AudioGraphNodePort {
        &self.attack
    }
    fn attack_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.attack
    }
    fn hold(&self) -> &super::audio::AudioGraphNodePort {
        &self.hold
    }
    fn hold_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.hold
    }
    fn release(&self) -> &super::audio::AudioGraphNodePort {
        &self.release
    }
    fn release_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.release
    }
    fn power(&self) -> &f32 {
        &self.power
    }
    fn power_mut(&mut self) -> &mut f32 {
        &mut self.power
    }
    fn value(&self) -> &super::audio::AudioGraphNodePort {
        &self.value
    }
    fn value_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.value
    }
    fn started(&self) -> &super::audio::AudioGraphNodePort {
        &self.started
    }
    fn started_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.started
    }
    fn stopped(&self) -> &super::audio::AudioGraphNodePort {
        &self.stopped
    }
    fn stopped_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.stopped
    }
}

impl super::audio::AudioGraphNodeDataTrait for ArOneShotNodeData {
}

impl super::core::DataContainerTrait for ArOneShotNodeData {
}

pub static ARONESHOTNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ArOneShotNodeData",
    name_hash: 3828808098,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        super_class_offset: offset_of!(ArOneShotNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ArOneShotNodeData as Default>::default())),
            create_boxed: || Box::new(<ArOneShotNodeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Start",
                name_hash: 230748069,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ArOneShotNodeData, start),
            },
            FieldInfoData {
                name: "Attack",
                name_hash: 2500885101,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ArOneShotNodeData, attack),
            },
            FieldInfoData {
                name: "Hold",
                name_hash: 2089155178,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ArOneShotNodeData, hold),
            },
            FieldInfoData {
                name: "Release",
                name_hash: 1335266828,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ArOneShotNodeData, release),
            },
            FieldInfoData {
                name: "Power",
                name_hash: 232673018,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ArOneShotNodeData, power),
            },
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ArOneShotNodeData, value),
            },
            FieldInfoData {
                name: "Started",
                name_hash: 2176542788,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ArOneShotNodeData, started),
            },
            FieldInfoData {
                name: "Stopped",
                name_hash: 2193212940,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ArOneShotNodeData, stopped),
            },
        ],
    }),
    array_type: Some(ARONESHOTNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ArOneShotNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        ARONESHOTNODEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static ARONESHOTNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ArOneShotNodeData-Array",
    name_hash: 2825033750,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ArOneShotNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ArLoopingNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
    pub start: super::audio::AudioGraphNodePort,
    pub stop: super::audio::AudioGraphNodePort,
    pub attack: super::audio::AudioGraphNodePort,
    pub release: super::audio::AudioGraphNodePort,
    pub power: f32,
    pub value: super::audio::AudioGraphNodePort,
    pub started: super::audio::AudioGraphNodePort,
    pub stopped: super::audio::AudioGraphNodePort,
}

pub trait ArLoopingNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
    fn start(&self) -> &super::audio::AudioGraphNodePort;
    fn start_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn stop(&self) -> &super::audio::AudioGraphNodePort;
    fn stop_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn attack(&self) -> &super::audio::AudioGraphNodePort;
    fn attack_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn release(&self) -> &super::audio::AudioGraphNodePort;
    fn release_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn power(&self) -> &f32;
    fn power_mut(&mut self) -> &mut f32;
    fn value(&self) -> &super::audio::AudioGraphNodePort;
    fn value_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn started(&self) -> &super::audio::AudioGraphNodePort;
    fn started_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn stopped(&self) -> &super::audio::AudioGraphNodePort;
    fn stopped_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
}

impl ArLoopingNodeDataTrait for ArLoopingNodeData {
    fn start(&self) -> &super::audio::AudioGraphNodePort {
        &self.start
    }
    fn start_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.start
    }
    fn stop(&self) -> &super::audio::AudioGraphNodePort {
        &self.stop
    }
    fn stop_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.stop
    }
    fn attack(&self) -> &super::audio::AudioGraphNodePort {
        &self.attack
    }
    fn attack_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.attack
    }
    fn release(&self) -> &super::audio::AudioGraphNodePort {
        &self.release
    }
    fn release_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.release
    }
    fn power(&self) -> &f32 {
        &self.power
    }
    fn power_mut(&mut self) -> &mut f32 {
        &mut self.power
    }
    fn value(&self) -> &super::audio::AudioGraphNodePort {
        &self.value
    }
    fn value_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.value
    }
    fn started(&self) -> &super::audio::AudioGraphNodePort {
        &self.started
    }
    fn started_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.started
    }
    fn stopped(&self) -> &super::audio::AudioGraphNodePort {
        &self.stopped
    }
    fn stopped_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.stopped
    }
}

impl super::audio::AudioGraphNodeDataTrait for ArLoopingNodeData {
}

impl super::core::DataContainerTrait for ArLoopingNodeData {
}

pub static ARLOOPINGNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ArLoopingNodeData",
    name_hash: 3601108282,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        super_class_offset: offset_of!(ArLoopingNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ArLoopingNodeData as Default>::default())),
            create_boxed: || Box::new(<ArLoopingNodeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Start",
                name_hash: 230748069,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ArLoopingNodeData, start),
            },
            FieldInfoData {
                name: "Stop",
                name_hash: 2089401213,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ArLoopingNodeData, stop),
            },
            FieldInfoData {
                name: "Attack",
                name_hash: 2500885101,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ArLoopingNodeData, attack),
            },
            FieldInfoData {
                name: "Release",
                name_hash: 1335266828,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ArLoopingNodeData, release),
            },
            FieldInfoData {
                name: "Power",
                name_hash: 232673018,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ArLoopingNodeData, power),
            },
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ArLoopingNodeData, value),
            },
            FieldInfoData {
                name: "Started",
                name_hash: 2176542788,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ArLoopingNodeData, started),
            },
            FieldInfoData {
                name: "Stopped",
                name_hash: 2193212940,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ArLoopingNodeData, stopped),
            },
        ],
    }),
    array_type: Some(ARLOOPINGNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ArLoopingNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        ARLOOPINGNODEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static ARLOOPINGNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ArLoopingNodeData-Array",
    name_hash: 950202254,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ArLoopingNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ArFlipFlopNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
    pub start: super::audio::AudioGraphNodePort,
    pub stop: super::audio::AudioGraphNodePort,
    pub stop_end_cycle: super::audio::AudioGraphNodePort,
    pub attack: super::audio::AudioGraphNodePort,
    pub hold: super::audio::AudioGraphNodePort,
    pub release: super::audio::AudioGraphNodePort,
    pub power: f32,
    pub value: super::audio::AudioGraphNodePort,
    pub started: super::audio::AudioGraphNodePort,
    pub end_attack: super::audio::AudioGraphNodePort,
    pub end_hold: super::audio::AudioGraphNodePort,
    pub end_release: super::audio::AudioGraphNodePort,
    pub stopped: super::audio::AudioGraphNodePort,
}

pub trait ArFlipFlopNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
    fn start(&self) -> &super::audio::AudioGraphNodePort;
    fn start_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn stop(&self) -> &super::audio::AudioGraphNodePort;
    fn stop_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn stop_end_cycle(&self) -> &super::audio::AudioGraphNodePort;
    fn stop_end_cycle_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn attack(&self) -> &super::audio::AudioGraphNodePort;
    fn attack_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn hold(&self) -> &super::audio::AudioGraphNodePort;
    fn hold_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn release(&self) -> &super::audio::AudioGraphNodePort;
    fn release_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn power(&self) -> &f32;
    fn power_mut(&mut self) -> &mut f32;
    fn value(&self) -> &super::audio::AudioGraphNodePort;
    fn value_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn started(&self) -> &super::audio::AudioGraphNodePort;
    fn started_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn end_attack(&self) -> &super::audio::AudioGraphNodePort;
    fn end_attack_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn end_hold(&self) -> &super::audio::AudioGraphNodePort;
    fn end_hold_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn end_release(&self) -> &super::audio::AudioGraphNodePort;
    fn end_release_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn stopped(&self) -> &super::audio::AudioGraphNodePort;
    fn stopped_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
}

impl ArFlipFlopNodeDataTrait for ArFlipFlopNodeData {
    fn start(&self) -> &super::audio::AudioGraphNodePort {
        &self.start
    }
    fn start_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.start
    }
    fn stop(&self) -> &super::audio::AudioGraphNodePort {
        &self.stop
    }
    fn stop_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.stop
    }
    fn stop_end_cycle(&self) -> &super::audio::AudioGraphNodePort {
        &self.stop_end_cycle
    }
    fn stop_end_cycle_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.stop_end_cycle
    }
    fn attack(&self) -> &super::audio::AudioGraphNodePort {
        &self.attack
    }
    fn attack_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.attack
    }
    fn hold(&self) -> &super::audio::AudioGraphNodePort {
        &self.hold
    }
    fn hold_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.hold
    }
    fn release(&self) -> &super::audio::AudioGraphNodePort {
        &self.release
    }
    fn release_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.release
    }
    fn power(&self) -> &f32 {
        &self.power
    }
    fn power_mut(&mut self) -> &mut f32 {
        &mut self.power
    }
    fn value(&self) -> &super::audio::AudioGraphNodePort {
        &self.value
    }
    fn value_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.value
    }
    fn started(&self) -> &super::audio::AudioGraphNodePort {
        &self.started
    }
    fn started_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.started
    }
    fn end_attack(&self) -> &super::audio::AudioGraphNodePort {
        &self.end_attack
    }
    fn end_attack_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.end_attack
    }
    fn end_hold(&self) -> &super::audio::AudioGraphNodePort {
        &self.end_hold
    }
    fn end_hold_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.end_hold
    }
    fn end_release(&self) -> &super::audio::AudioGraphNodePort {
        &self.end_release
    }
    fn end_release_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.end_release
    }
    fn stopped(&self) -> &super::audio::AudioGraphNodePort {
        &self.stopped
    }
    fn stopped_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.stopped
    }
}

impl super::audio::AudioGraphNodeDataTrait for ArFlipFlopNodeData {
}

impl super::core::DataContainerTrait for ArFlipFlopNodeData {
}

pub static ARFLIPFLOPNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ArFlipFlopNodeData",
    name_hash: 2945097856,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        super_class_offset: offset_of!(ArFlipFlopNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ArFlipFlopNodeData as Default>::default())),
            create_boxed: || Box::new(<ArFlipFlopNodeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Start",
                name_hash: 230748069,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ArFlipFlopNodeData, start),
            },
            FieldInfoData {
                name: "Stop",
                name_hash: 2089401213,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ArFlipFlopNodeData, stop),
            },
            FieldInfoData {
                name: "StopEndCycle",
                name_hash: 150469666,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ArFlipFlopNodeData, stop_end_cycle),
            },
            FieldInfoData {
                name: "Attack",
                name_hash: 2500885101,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ArFlipFlopNodeData, attack),
            },
            FieldInfoData {
                name: "Hold",
                name_hash: 2089155178,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ArFlipFlopNodeData, hold),
            },
            FieldInfoData {
                name: "Release",
                name_hash: 1335266828,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ArFlipFlopNodeData, release),
            },
            FieldInfoData {
                name: "Power",
                name_hash: 232673018,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ArFlipFlopNodeData, power),
            },
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ArFlipFlopNodeData, value),
            },
            FieldInfoData {
                name: "Started",
                name_hash: 2176542788,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ArFlipFlopNodeData, started),
            },
            FieldInfoData {
                name: "EndAttack",
                name_hash: 3215395202,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ArFlipFlopNodeData, end_attack),
            },
            FieldInfoData {
                name: "EndHold",
                name_hash: 4285547909,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ArFlipFlopNodeData, end_hold),
            },
            FieldInfoData {
                name: "EndRelease",
                name_hash: 3772678659,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ArFlipFlopNodeData, end_release),
            },
            FieldInfoData {
                name: "Stopped",
                name_hash: 2193212940,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ArFlipFlopNodeData, stopped),
            },
        ],
    }),
    array_type: Some(ARFLIPFLOPNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ArFlipFlopNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        ARFLIPFLOPNODEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static ARFLIPFLOPNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ArFlipFlopNodeData-Array",
    name_hash: 4139755188,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ArFlipFlopNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ListenerAreaTypeNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
    pub area_type: super::audio::AudioGraphNodePort,
    pub use_default_listener: bool,
    pub listener_id: i32,
}

pub trait ListenerAreaTypeNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
    fn area_type(&self) -> &super::audio::AudioGraphNodePort;
    fn area_type_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn use_default_listener(&self) -> &bool;
    fn use_default_listener_mut(&mut self) -> &mut bool;
    fn listener_id(&self) -> &i32;
    fn listener_id_mut(&mut self) -> &mut i32;
}

impl ListenerAreaTypeNodeDataTrait for ListenerAreaTypeNodeData {
    fn area_type(&self) -> &super::audio::AudioGraphNodePort {
        &self.area_type
    }
    fn area_type_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.area_type
    }
    fn use_default_listener(&self) -> &bool {
        &self.use_default_listener
    }
    fn use_default_listener_mut(&mut self) -> &mut bool {
        &mut self.use_default_listener
    }
    fn listener_id(&self) -> &i32 {
        &self.listener_id
    }
    fn listener_id_mut(&mut self) -> &mut i32 {
        &mut self.listener_id
    }
}

impl super::audio::AudioGraphNodeDataTrait for ListenerAreaTypeNodeData {
}

impl super::core::DataContainerTrait for ListenerAreaTypeNodeData {
}

pub static LISTENERAREATYPENODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ListenerAreaTypeNodeData",
    name_hash: 2566463620,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        super_class_offset: offset_of!(ListenerAreaTypeNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ListenerAreaTypeNodeData as Default>::default())),
            create_boxed: || Box::new(<ListenerAreaTypeNodeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "AreaType",
                name_hash: 3772263626,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(ListenerAreaTypeNodeData, area_type),
            },
            FieldInfoData {
                name: "UseDefaultListener",
                name_hash: 829611027,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ListenerAreaTypeNodeData, use_default_listener),
            },
            FieldInfoData {
                name: "ListenerId",
                name_hash: 1750046998,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ListenerAreaTypeNodeData, listener_id),
            },
        ],
    }),
    array_type: Some(LISTENERAREATYPENODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ListenerAreaTypeNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        LISTENERAREATYPENODEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static LISTENERAREATYPENODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ListenerAreaTypeNodeData-Array",
    name_hash: 4199404720,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ListenerAreaTypeNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AreaTypeNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
    pub check: super::audio::AudioGraphNodePort,
    pub result: super::audio::AudioGraphNodePort,
    pub area_type: super::audio::AudioGraphNodePort,
}

pub trait AreaTypeNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
    fn check(&self) -> &super::audio::AudioGraphNodePort;
    fn check_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn result(&self) -> &super::audio::AudioGraphNodePort;
    fn result_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn area_type(&self) -> &super::audio::AudioGraphNodePort;
    fn area_type_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
}

impl AreaTypeNodeDataTrait for AreaTypeNodeData {
    fn check(&self) -> &super::audio::AudioGraphNodePort {
        &self.check
    }
    fn check_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.check
    }
    fn result(&self) -> &super::audio::AudioGraphNodePort {
        &self.result
    }
    fn result_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.result
    }
    fn area_type(&self) -> &super::audio::AudioGraphNodePort {
        &self.area_type
    }
    fn area_type_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.area_type
    }
}

impl super::audio::AudioGraphNodeDataTrait for AreaTypeNodeData {
}

impl super::core::DataContainerTrait for AreaTypeNodeData {
}

pub static AREATYPENODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AreaTypeNodeData",
    name_hash: 1869836570,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        super_class_offset: offset_of!(AreaTypeNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AreaTypeNodeData as Default>::default())),
            create_boxed: || Box::new(<AreaTypeNodeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Check",
                name_hash: 212774723,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(AreaTypeNodeData, check),
            },
            FieldInfoData {
                name: "Result",
                name_hash: 3293273164,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(AreaTypeNodeData, result),
            },
            FieldInfoData {
                name: "AreaType",
                name_hash: 3772263626,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(AreaTypeNodeData, area_type),
            },
        ],
    }),
    array_type: Some(AREATYPENODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AreaTypeNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        AREATYPENODEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static AREATYPENODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AreaTypeNodeData-Array",
    name_hash: 2128688174,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("AreaTypeNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct VoiceOverSourceEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub voice_over_info: Option<LockedTypeObject /* super::audio::EntityVoiceOverInfo */>,
    pub transform: super::core::LinearTransform,
}

pub trait VoiceOverSourceEntityDataTrait: super::entity::EntityDataTrait {
    fn voice_over_info(&self) -> &Option<LockedTypeObject /* super::audio::EntityVoiceOverInfo */>;
    fn voice_over_info_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::EntityVoiceOverInfo */>;
    fn transform(&self) -> &super::core::LinearTransform;
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform;
}

impl VoiceOverSourceEntityDataTrait for VoiceOverSourceEntityData {
    fn voice_over_info(&self) -> &Option<LockedTypeObject /* super::audio::EntityVoiceOverInfo */> {
        &self.voice_over_info
    }
    fn voice_over_info_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::EntityVoiceOverInfo */> {
        &mut self.voice_over_info
    }
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.transform
    }
}

impl super::entity::EntityDataTrait for VoiceOverSourceEntityData {
}

impl super::entity::GameObjectDataTrait for VoiceOverSourceEntityData {
}

impl super::core::DataBusPeerTrait for VoiceOverSourceEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for VoiceOverSourceEntityData {
}

impl super::core::DataContainerTrait for VoiceOverSourceEntityData {
}

pub static VOICEOVERSOURCEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceOverSourceEntityData",
    name_hash: 3576294123,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(VoiceOverSourceEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VoiceOverSourceEntityData as Default>::default())),
            create_boxed: || Box::new(<VoiceOverSourceEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "VoiceOverInfo",
                name_hash: 1260547539,
                flags: MemberInfoFlags::new(0),
                field_type: "EntityVoiceOverInfo",
                rust_offset: offset_of!(VoiceOverSourceEntityData, voice_over_info),
            },
            FieldInfoData {
                name: "Transform",
                name_hash: 2270319721,
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(VoiceOverSourceEntityData, transform),
            },
        ],
    }),
    array_type: Some(VOICEOVERSOURCEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for VoiceOverSourceEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        VOICEOVERSOURCEENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static VOICEOVERSOURCEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceOverSourceEntityData-Array",
    name_hash: 4172290783,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("VoiceOverSourceEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct VoiceOverSetLanguageEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub language_index: i32,
}

pub trait VoiceOverSetLanguageEntityDataTrait: super::entity::EntityDataTrait {
    fn language_index(&self) -> &i32;
    fn language_index_mut(&mut self) -> &mut i32;
}

impl VoiceOverSetLanguageEntityDataTrait for VoiceOverSetLanguageEntityData {
    fn language_index(&self) -> &i32 {
        &self.language_index
    }
    fn language_index_mut(&mut self) -> &mut i32 {
        &mut self.language_index
    }
}

impl super::entity::EntityDataTrait for VoiceOverSetLanguageEntityData {
}

impl super::entity::GameObjectDataTrait for VoiceOverSetLanguageEntityData {
}

impl super::core::DataBusPeerTrait for VoiceOverSetLanguageEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for VoiceOverSetLanguageEntityData {
}

impl super::core::DataContainerTrait for VoiceOverSetLanguageEntityData {
}

pub static VOICEOVERSETLANGUAGEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceOverSetLanguageEntityData",
    name_hash: 3742091654,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(VoiceOverSetLanguageEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VoiceOverSetLanguageEntityData as Default>::default())),
            create_boxed: || Box::new(<VoiceOverSetLanguageEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "LanguageIndex",
                name_hash: 2501859817,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(VoiceOverSetLanguageEntityData, language_index),
            },
        ],
    }),
    array_type: Some(VOICEOVERSETLANGUAGEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VoiceOverSetLanguageEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        VOICEOVERSETLANGUAGEENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static VOICEOVERSETLANGUAGEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceOverSetLanguageEntityData-Array",
    name_hash: 3517413042,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("VoiceOverSetLanguageEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LanguageCollection {
    pub _glacier_base: super::core::DataContainer,
    pub languages_string: Vec<Option<LockedTypeObject /* LanguageCollectionElement */>>,
}

pub trait LanguageCollectionTrait: super::core::DataContainerTrait {
    fn languages_string(&self) -> &Vec<Option<LockedTypeObject /* LanguageCollectionElement */>>;
    fn languages_string_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* LanguageCollectionElement */>>;
}

impl LanguageCollectionTrait for LanguageCollection {
    fn languages_string(&self) -> &Vec<Option<LockedTypeObject /* LanguageCollectionElement */>> {
        &self.languages_string
    }
    fn languages_string_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* LanguageCollectionElement */>> {
        &mut self.languages_string
    }
}

impl super::core::DataContainerTrait for LanguageCollection {
}

pub static LANGUAGECOLLECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LanguageCollection",
    name_hash: 190370273,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(LanguageCollection, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LanguageCollection as Default>::default())),
            create_boxed: || Box::new(<LanguageCollection as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "LanguagesString",
                name_hash: 194844401,
                flags: MemberInfoFlags::new(144),
                field_type: "LanguageCollectionElement-Array",
                rust_offset: offset_of!(LanguageCollection, languages_string),
            },
        ],
    }),
    array_type: Some(LANGUAGECOLLECTION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LanguageCollection {
    fn type_info(&self) -> &'static TypeInfo {
        LANGUAGECOLLECTION_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static LANGUAGECOLLECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LanguageCollection-Array",
    name_hash: 2315562197,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("LanguageCollection"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LanguageCollectionElement {
    pub _glacier_base: super::core::DataContainer,
    pub language: String,
    pub language_string_id: Option<LockedTypeObject /* super::u_i_incubator_shared::LocalizedStringId */>,
}

pub trait LanguageCollectionElementTrait: super::core::DataContainerTrait {
    fn language(&self) -> &String;
    fn language_mut(&mut self) -> &mut String;
    fn language_string_id(&self) -> &Option<LockedTypeObject /* super::u_i_incubator_shared::LocalizedStringId */>;
    fn language_string_id_mut(&mut self) -> &mut Option<LockedTypeObject /* super::u_i_incubator_shared::LocalizedStringId */>;
}

impl LanguageCollectionElementTrait for LanguageCollectionElement {
    fn language(&self) -> &String {
        &self.language
    }
    fn language_mut(&mut self) -> &mut String {
        &mut self.language
    }
    fn language_string_id(&self) -> &Option<LockedTypeObject /* super::u_i_incubator_shared::LocalizedStringId */> {
        &self.language_string_id
    }
    fn language_string_id_mut(&mut self) -> &mut Option<LockedTypeObject /* super::u_i_incubator_shared::LocalizedStringId */> {
        &mut self.language_string_id
    }
}

impl super::core::DataContainerTrait for LanguageCollectionElement {
}

pub static LANGUAGECOLLECTIONELEMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LanguageCollectionElement",
    name_hash: 596688831,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(LanguageCollectionElement, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LanguageCollectionElement as Default>::default())),
            create_boxed: || Box::new(<LanguageCollectionElement as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Language",
                name_hash: 3872303031,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(LanguageCollectionElement, language),
            },
            FieldInfoData {
                name: "LanguageStringId",
                name_hash: 4241723759,
                flags: MemberInfoFlags::new(0),
                field_type: "LocalizedStringId",
                rust_offset: offset_of!(LanguageCollectionElement, language_string_id),
            },
        ],
    }),
    array_type: Some(LANGUAGECOLLECTIONELEMENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LanguageCollectionElement {
    fn type_info(&self) -> &'static TypeInfo {
        LANGUAGECOLLECTIONELEMENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static LANGUAGECOLLECTIONELEMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LanguageCollectionElement-Array",
    name_hash: 3651269899,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("LanguageCollectionElement"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct VoiceOverLocatorEntityData {
    pub _glacier_base: super::entity::SpatialEntityData,
    pub realm: super::core::Realm,
    pub voice_over_info: Option<LockedTypeObject /* super::audio::EntityVoiceOverInfo */>,
}

pub trait VoiceOverLocatorEntityDataTrait: super::entity::SpatialEntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn voice_over_info(&self) -> &Option<LockedTypeObject /* super::audio::EntityVoiceOverInfo */>;
    fn voice_over_info_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::EntityVoiceOverInfo */>;
}

impl VoiceOverLocatorEntityDataTrait for VoiceOverLocatorEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn voice_over_info(&self) -> &Option<LockedTypeObject /* super::audio::EntityVoiceOverInfo */> {
        &self.voice_over_info
    }
    fn voice_over_info_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::EntityVoiceOverInfo */> {
        &mut self.voice_over_info
    }
}

impl super::entity::SpatialEntityDataTrait for VoiceOverLocatorEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::entity::EntityDataTrait for VoiceOverLocatorEntityData {
}

impl super::entity::GameObjectDataTrait for VoiceOverLocatorEntityData {
}

impl super::core::DataBusPeerTrait for VoiceOverLocatorEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for VoiceOverLocatorEntityData {
}

impl super::core::DataContainerTrait for VoiceOverLocatorEntityData {
}

pub static VOICEOVERLOCATORENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceOverLocatorEntityData",
    name_hash: 595727166,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(VoiceOverLocatorEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VoiceOverLocatorEntityData as Default>::default())),
            create_boxed: || Box::new(<VoiceOverLocatorEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                name_hash: 229961746,
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(VoiceOverLocatorEntityData, realm),
            },
            FieldInfoData {
                name: "VoiceOverInfo",
                name_hash: 1260547539,
                flags: MemberInfoFlags::new(0),
                field_type: "EntityVoiceOverInfo",
                rust_offset: offset_of!(VoiceOverLocatorEntityData, voice_over_info),
            },
        ],
    }),
    array_type: Some(VOICEOVERLOCATORENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for VoiceOverLocatorEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        VOICEOVERLOCATORENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static VOICEOVERLOCATORENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceOverLocatorEntityData-Array",
    name_hash: 340566922,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("VoiceOverLocatorEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EntryControllableAttachData {
    pub _glacier_base: EntityAttachData,
    pub bone: super::entity::GameplayBones,
}

pub trait EntryControllableAttachDataTrait: EntityAttachDataTrait {
    fn bone(&self) -> &super::entity::GameplayBones;
    fn bone_mut(&mut self) -> &mut super::entity::GameplayBones;
}

impl EntryControllableAttachDataTrait for EntryControllableAttachData {
    fn bone(&self) -> &super::entity::GameplayBones {
        &self.bone
    }
    fn bone_mut(&mut self) -> &mut super::entity::GameplayBones {
        &mut self.bone
    }
}

impl EntityAttachDataTrait for EntryControllableAttachData {
    fn property_name(&self) -> &String {
        self._glacier_base.property_name()
    }
    fn property_name_mut(&mut self) -> &mut String {
        self._glacier_base.property_name_mut()
    }
    fn has_dynamic_transform_space_offset(&self) -> &bool {
        self._glacier_base.has_dynamic_transform_space_offset()
    }
    fn has_dynamic_transform_space_offset_mut(&mut self) -> &mut bool {
        self._glacier_base.has_dynamic_transform_space_offset_mut()
    }
    fn use_rotation(&self) -> &bool {
        self._glacier_base.use_rotation()
    }
    fn use_rotation_mut(&mut self) -> &mut bool {
        self._glacier_base.use_rotation_mut()
    }
    fn coordinate_space(&self) -> &Option<LockedTypeObject /* CoordinateModificationData */> {
        self._glacier_base.coordinate_space()
    }
    fn coordinate_space_mut(&mut self) -> &mut Option<LockedTypeObject /* CoordinateModificationData */> {
        self._glacier_base.coordinate_space_mut()
    }
    fn offset(&self) -> &Option<LockedTypeObject /* OffsetModificationData */> {
        self._glacier_base.offset()
    }
    fn offset_mut(&mut self) -> &mut Option<LockedTypeObject /* OffsetModificationData */> {
        self._glacier_base.offset_mut()
    }
}

impl EntityLinkDataTrait for EntryControllableAttachData {
    fn link_name(&self) -> &String {
        self._glacier_base.link_name()
    }
    fn link_name_mut(&mut self) -> &mut String {
        self._glacier_base.link_name_mut()
    }
}

impl super::core::DataContainerTrait for EntryControllableAttachData {
}

pub static ENTRYCONTROLLABLEATTACHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntryControllableAttachData",
    name_hash: 1533787147,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYATTACHDATA_TYPE_INFO),
        super_class_offset: offset_of!(EntryControllableAttachData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EntryControllableAttachData as Default>::default())),
            create_boxed: || Box::new(<EntryControllableAttachData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Bone",
                name_hash: 2088812771,
                flags: MemberInfoFlags::new(0),
                field_type: "GameplayBones",
                rust_offset: offset_of!(EntryControllableAttachData, bone),
            },
        ],
    }),
    array_type: Some(ENTRYCONTROLLABLEATTACHDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EntryControllableAttachData {
    fn type_info(&self) -> &'static TypeInfo {
        ENTRYCONTROLLABLEATTACHDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static ENTRYCONTROLLABLEATTACHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntryControllableAttachData-Array",
    name_hash: 3244952255,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("EntryControllableAttachData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ControllableAttachData {
    pub _glacier_base: AnimatableAttachData,
}

pub trait ControllableAttachDataTrait: AnimatableAttachDataTrait {
}

impl ControllableAttachDataTrait for ControllableAttachData {
}

impl AnimatableAttachDataTrait for ControllableAttachData {
    fn bone(&self) -> &super::entity::GameplayBones {
        self._glacier_base.bone()
    }
    fn bone_mut(&mut self) -> &mut super::entity::GameplayBones {
        self._glacier_base.bone_mut()
    }
}

impl EntityAttachDataTrait for ControllableAttachData {
    fn property_name(&self) -> &String {
        self._glacier_base.property_name()
    }
    fn property_name_mut(&mut self) -> &mut String {
        self._glacier_base.property_name_mut()
    }
    fn has_dynamic_transform_space_offset(&self) -> &bool {
        self._glacier_base.has_dynamic_transform_space_offset()
    }
    fn has_dynamic_transform_space_offset_mut(&mut self) -> &mut bool {
        self._glacier_base.has_dynamic_transform_space_offset_mut()
    }
    fn use_rotation(&self) -> &bool {
        self._glacier_base.use_rotation()
    }
    fn use_rotation_mut(&mut self) -> &mut bool {
        self._glacier_base.use_rotation_mut()
    }
    fn coordinate_space(&self) -> &Option<LockedTypeObject /* CoordinateModificationData */> {
        self._glacier_base.coordinate_space()
    }
    fn coordinate_space_mut(&mut self) -> &mut Option<LockedTypeObject /* CoordinateModificationData */> {
        self._glacier_base.coordinate_space_mut()
    }
    fn offset(&self) -> &Option<LockedTypeObject /* OffsetModificationData */> {
        self._glacier_base.offset()
    }
    fn offset_mut(&mut self) -> &mut Option<LockedTypeObject /* OffsetModificationData */> {
        self._glacier_base.offset_mut()
    }
}

impl EntityLinkDataTrait for ControllableAttachData {
    fn link_name(&self) -> &String {
        self._glacier_base.link_name()
    }
    fn link_name_mut(&mut self) -> &mut String {
        self._glacier_base.link_name_mut()
    }
}

impl super::core::DataContainerTrait for ControllableAttachData {
}

pub static CONTROLLABLEATTACHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ControllableAttachData",
    name_hash: 4107383039,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ANIMATABLEATTACHDATA_TYPE_INFO),
        super_class_offset: offset_of!(ControllableAttachData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ControllableAttachData as Default>::default())),
            create_boxed: || Box::new(<ControllableAttachData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CONTROLLABLEATTACHDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ControllableAttachData {
    fn type_info(&self) -> &'static TypeInfo {
        CONTROLLABLEATTACHDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CONTROLLABLEATTACHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ControllableAttachData-Array",
    name_hash: 1969673931,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ControllableAttachData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ModelAnimationAttachData {
    pub _glacier_base: EntityAttachData,
    pub bone: super::entity::GameplayBones,
}

pub trait ModelAnimationAttachDataTrait: EntityAttachDataTrait {
    fn bone(&self) -> &super::entity::GameplayBones;
    fn bone_mut(&mut self) -> &mut super::entity::GameplayBones;
}

impl ModelAnimationAttachDataTrait for ModelAnimationAttachData {
    fn bone(&self) -> &super::entity::GameplayBones {
        &self.bone
    }
    fn bone_mut(&mut self) -> &mut super::entity::GameplayBones {
        &mut self.bone
    }
}

impl EntityAttachDataTrait for ModelAnimationAttachData {
    fn property_name(&self) -> &String {
        self._glacier_base.property_name()
    }
    fn property_name_mut(&mut self) -> &mut String {
        self._glacier_base.property_name_mut()
    }
    fn has_dynamic_transform_space_offset(&self) -> &bool {
        self._glacier_base.has_dynamic_transform_space_offset()
    }
    fn has_dynamic_transform_space_offset_mut(&mut self) -> &mut bool {
        self._glacier_base.has_dynamic_transform_space_offset_mut()
    }
    fn use_rotation(&self) -> &bool {
        self._glacier_base.use_rotation()
    }
    fn use_rotation_mut(&mut self) -> &mut bool {
        self._glacier_base.use_rotation_mut()
    }
    fn coordinate_space(&self) -> &Option<LockedTypeObject /* CoordinateModificationData */> {
        self._glacier_base.coordinate_space()
    }
    fn coordinate_space_mut(&mut self) -> &mut Option<LockedTypeObject /* CoordinateModificationData */> {
        self._glacier_base.coordinate_space_mut()
    }
    fn offset(&self) -> &Option<LockedTypeObject /* OffsetModificationData */> {
        self._glacier_base.offset()
    }
    fn offset_mut(&mut self) -> &mut Option<LockedTypeObject /* OffsetModificationData */> {
        self._glacier_base.offset_mut()
    }
}

impl EntityLinkDataTrait for ModelAnimationAttachData {
    fn link_name(&self) -> &String {
        self._glacier_base.link_name()
    }
    fn link_name_mut(&mut self) -> &mut String {
        self._glacier_base.link_name_mut()
    }
}

impl super::core::DataContainerTrait for ModelAnimationAttachData {
}

pub static MODELANIMATIONATTACHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ModelAnimationAttachData",
    name_hash: 1632689511,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYATTACHDATA_TYPE_INFO),
        super_class_offset: offset_of!(ModelAnimationAttachData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ModelAnimationAttachData as Default>::default())),
            create_boxed: || Box::new(<ModelAnimationAttachData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Bone",
                name_hash: 2088812771,
                flags: MemberInfoFlags::new(0),
                field_type: "GameplayBones",
                rust_offset: offset_of!(ModelAnimationAttachData, bone),
            },
        ],
    }),
    array_type: Some(MODELANIMATIONATTACHDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ModelAnimationAttachData {
    fn type_info(&self) -> &'static TypeInfo {
        MODELANIMATIONATTACHDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static MODELANIMATIONATTACHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ModelAnimationAttachData-Array",
    name_hash: 1923301203,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ModelAnimationAttachData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AnimatableAttachData {
    pub _glacier_base: EntityAttachData,
    pub bone: super::entity::GameplayBones,
}

pub trait AnimatableAttachDataTrait: EntityAttachDataTrait {
    fn bone(&self) -> &super::entity::GameplayBones;
    fn bone_mut(&mut self) -> &mut super::entity::GameplayBones;
}

impl AnimatableAttachDataTrait for AnimatableAttachData {
    fn bone(&self) -> &super::entity::GameplayBones {
        &self.bone
    }
    fn bone_mut(&mut self) -> &mut super::entity::GameplayBones {
        &mut self.bone
    }
}

impl EntityAttachDataTrait for AnimatableAttachData {
    fn property_name(&self) -> &String {
        self._glacier_base.property_name()
    }
    fn property_name_mut(&mut self) -> &mut String {
        self._glacier_base.property_name_mut()
    }
    fn has_dynamic_transform_space_offset(&self) -> &bool {
        self._glacier_base.has_dynamic_transform_space_offset()
    }
    fn has_dynamic_transform_space_offset_mut(&mut self) -> &mut bool {
        self._glacier_base.has_dynamic_transform_space_offset_mut()
    }
    fn use_rotation(&self) -> &bool {
        self._glacier_base.use_rotation()
    }
    fn use_rotation_mut(&mut self) -> &mut bool {
        self._glacier_base.use_rotation_mut()
    }
    fn coordinate_space(&self) -> &Option<LockedTypeObject /* CoordinateModificationData */> {
        self._glacier_base.coordinate_space()
    }
    fn coordinate_space_mut(&mut self) -> &mut Option<LockedTypeObject /* CoordinateModificationData */> {
        self._glacier_base.coordinate_space_mut()
    }
    fn offset(&self) -> &Option<LockedTypeObject /* OffsetModificationData */> {
        self._glacier_base.offset()
    }
    fn offset_mut(&mut self) -> &mut Option<LockedTypeObject /* OffsetModificationData */> {
        self._glacier_base.offset_mut()
    }
}

impl EntityLinkDataTrait for AnimatableAttachData {
    fn link_name(&self) -> &String {
        self._glacier_base.link_name()
    }
    fn link_name_mut(&mut self) -> &mut String {
        self._glacier_base.link_name_mut()
    }
}

impl super::core::DataContainerTrait for AnimatableAttachData {
}

pub static ANIMATABLEATTACHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimatableAttachData",
    name_hash: 240555818,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYATTACHDATA_TYPE_INFO),
        super_class_offset: offset_of!(AnimatableAttachData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AnimatableAttachData as Default>::default())),
            create_boxed: || Box::new(<AnimatableAttachData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Bone",
                name_hash: 2088812771,
                flags: MemberInfoFlags::new(0),
                field_type: "GameplayBones",
                rust_offset: offset_of!(AnimatableAttachData, bone),
            },
        ],
    }),
    array_type: Some(ANIMATABLEATTACHDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AnimatableAttachData {
    fn type_info(&self) -> &'static TypeInfo {
        ANIMATABLEATTACHDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static ANIMATABLEATTACHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimatableAttachData-Array",
    name_hash: 3835802014,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("AnimatableAttachData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ComponentAttachData {
    pub _glacier_base: EntityAttachData,
}

pub trait ComponentAttachDataTrait: EntityAttachDataTrait {
}

impl ComponentAttachDataTrait for ComponentAttachData {
}

impl EntityAttachDataTrait for ComponentAttachData {
    fn property_name(&self) -> &String {
        self._glacier_base.property_name()
    }
    fn property_name_mut(&mut self) -> &mut String {
        self._glacier_base.property_name_mut()
    }
    fn has_dynamic_transform_space_offset(&self) -> &bool {
        self._glacier_base.has_dynamic_transform_space_offset()
    }
    fn has_dynamic_transform_space_offset_mut(&mut self) -> &mut bool {
        self._glacier_base.has_dynamic_transform_space_offset_mut()
    }
    fn use_rotation(&self) -> &bool {
        self._glacier_base.use_rotation()
    }
    fn use_rotation_mut(&mut self) -> &mut bool {
        self._glacier_base.use_rotation_mut()
    }
    fn coordinate_space(&self) -> &Option<LockedTypeObject /* CoordinateModificationData */> {
        self._glacier_base.coordinate_space()
    }
    fn coordinate_space_mut(&mut self) -> &mut Option<LockedTypeObject /* CoordinateModificationData */> {
        self._glacier_base.coordinate_space_mut()
    }
    fn offset(&self) -> &Option<LockedTypeObject /* OffsetModificationData */> {
        self._glacier_base.offset()
    }
    fn offset_mut(&mut self) -> &mut Option<LockedTypeObject /* OffsetModificationData */> {
        self._glacier_base.offset_mut()
    }
}

impl EntityLinkDataTrait for ComponentAttachData {
    fn link_name(&self) -> &String {
        self._glacier_base.link_name()
    }
    fn link_name_mut(&mut self) -> &mut String {
        self._glacier_base.link_name_mut()
    }
}

impl super::core::DataContainerTrait for ComponentAttachData {
}

pub static COMPONENTATTACHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ComponentAttachData",
    name_hash: 330490193,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYATTACHDATA_TYPE_INFO),
        super_class_offset: offset_of!(ComponentAttachData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ComponentAttachData as Default>::default())),
            create_boxed: || Box::new(<ComponentAttachData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(COMPONENTATTACHDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ComponentAttachData {
    fn type_info(&self) -> &'static TypeInfo {
        COMPONENTATTACHDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static COMPONENTATTACHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ComponentAttachData-Array",
    name_hash: 2791248741,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ComponentAttachData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EntityAttachData {
    pub _glacier_base: EntityLinkData,
    pub property_name: String,
    pub has_dynamic_transform_space_offset: bool,
    pub use_rotation: bool,
    pub coordinate_space: Option<LockedTypeObject /* CoordinateModificationData */>,
    pub offset: Option<LockedTypeObject /* OffsetModificationData */>,
}

pub trait EntityAttachDataTrait: EntityLinkDataTrait {
    fn property_name(&self) -> &String;
    fn property_name_mut(&mut self) -> &mut String;
    fn has_dynamic_transform_space_offset(&self) -> &bool;
    fn has_dynamic_transform_space_offset_mut(&mut self) -> &mut bool;
    fn use_rotation(&self) -> &bool;
    fn use_rotation_mut(&mut self) -> &mut bool;
    fn coordinate_space(&self) -> &Option<LockedTypeObject /* CoordinateModificationData */>;
    fn coordinate_space_mut(&mut self) -> &mut Option<LockedTypeObject /* CoordinateModificationData */>;
    fn offset(&self) -> &Option<LockedTypeObject /* OffsetModificationData */>;
    fn offset_mut(&mut self) -> &mut Option<LockedTypeObject /* OffsetModificationData */>;
}

impl EntityAttachDataTrait for EntityAttachData {
    fn property_name(&self) -> &String {
        &self.property_name
    }
    fn property_name_mut(&mut self) -> &mut String {
        &mut self.property_name
    }
    fn has_dynamic_transform_space_offset(&self) -> &bool {
        &self.has_dynamic_transform_space_offset
    }
    fn has_dynamic_transform_space_offset_mut(&mut self) -> &mut bool {
        &mut self.has_dynamic_transform_space_offset
    }
    fn use_rotation(&self) -> &bool {
        &self.use_rotation
    }
    fn use_rotation_mut(&mut self) -> &mut bool {
        &mut self.use_rotation
    }
    fn coordinate_space(&self) -> &Option<LockedTypeObject /* CoordinateModificationData */> {
        &self.coordinate_space
    }
    fn coordinate_space_mut(&mut self) -> &mut Option<LockedTypeObject /* CoordinateModificationData */> {
        &mut self.coordinate_space
    }
    fn offset(&self) -> &Option<LockedTypeObject /* OffsetModificationData */> {
        &self.offset
    }
    fn offset_mut(&mut self) -> &mut Option<LockedTypeObject /* OffsetModificationData */> {
        &mut self.offset
    }
}

impl EntityLinkDataTrait for EntityAttachData {
    fn link_name(&self) -> &String {
        self._glacier_base.link_name()
    }
    fn link_name_mut(&mut self) -> &mut String {
        self._glacier_base.link_name_mut()
    }
}

impl super::core::DataContainerTrait for EntityAttachData {
}

pub static ENTITYATTACHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityAttachData",
    name_hash: 2642520645,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYLINKDATA_TYPE_INFO),
        super_class_offset: offset_of!(EntityAttachData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EntityAttachData as Default>::default())),
            create_boxed: || Box::new(<EntityAttachData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "PropertyName",
                name_hash: 3998208485,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(EntityAttachData, property_name),
            },
            FieldInfoData {
                name: "HasDynamicTransformSpaceOffset",
                name_hash: 262235855,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EntityAttachData, has_dynamic_transform_space_offset),
            },
            FieldInfoData {
                name: "UseRotation",
                name_hash: 1742642130,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EntityAttachData, use_rotation),
            },
            FieldInfoData {
                name: "CoordinateSpace",
                name_hash: 4130195427,
                flags: MemberInfoFlags::new(0),
                field_type: "CoordinateModificationData",
                rust_offset: offset_of!(EntityAttachData, coordinate_space),
            },
            FieldInfoData {
                name: "Offset",
                name_hash: 2871410728,
                flags: MemberInfoFlags::new(0),
                field_type: "OffsetModificationData",
                rust_offset: offset_of!(EntityAttachData, offset),
            },
        ],
    }),
    array_type: Some(ENTITYATTACHDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EntityAttachData {
    fn type_info(&self) -> &'static TypeInfo {
        ENTITYATTACHDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static ENTITYATTACHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityAttachData-Array",
    name_hash: 2651685489,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("EntityAttachData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ControllableTransformLinkData {
    pub _glacier_base: AnimatableTransformLinkData,
}

pub trait ControllableTransformLinkDataTrait: AnimatableTransformLinkDataTrait {
}

impl ControllableTransformLinkDataTrait for ControllableTransformLinkData {
}

impl AnimatableTransformLinkDataTrait for ControllableTransformLinkData {
    fn bone(&self) -> &super::entity::GameplayBones {
        self._glacier_base.bone()
    }
    fn bone_mut(&mut self) -> &mut super::entity::GameplayBones {
        self._glacier_base.bone_mut()
    }
}

impl EntityTransformLinkDataTrait for ControllableTransformLinkData {
}

impl EntityLinkDataTrait for ControllableTransformLinkData {
    fn link_name(&self) -> &String {
        self._glacier_base.link_name()
    }
    fn link_name_mut(&mut self) -> &mut String {
        self._glacier_base.link_name_mut()
    }
}

impl super::core::DataContainerTrait for ControllableTransformLinkData {
}

pub static CONTROLLABLETRANSFORMLINKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ControllableTransformLinkData",
    name_hash: 153697336,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ANIMATABLETRANSFORMLINKDATA_TYPE_INFO),
        super_class_offset: offset_of!(ControllableTransformLinkData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ControllableTransformLinkData as Default>::default())),
            create_boxed: || Box::new(<ControllableTransformLinkData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CONTROLLABLETRANSFORMLINKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ControllableTransformLinkData {
    fn type_info(&self) -> &'static TypeInfo {
        CONTROLLABLETRANSFORMLINKDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CONTROLLABLETRANSFORMLINKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ControllableTransformLinkData-Array",
    name_hash: 3010880908,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ControllableTransformLinkData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EntryControllableTransformLinkData {
    pub _glacier_base: EntityTransformLinkData,
    pub bone: super::entity::GameplayBones,
}

pub trait EntryControllableTransformLinkDataTrait: EntityTransformLinkDataTrait {
    fn bone(&self) -> &super::entity::GameplayBones;
    fn bone_mut(&mut self) -> &mut super::entity::GameplayBones;
}

impl EntryControllableTransformLinkDataTrait for EntryControllableTransformLinkData {
    fn bone(&self) -> &super::entity::GameplayBones {
        &self.bone
    }
    fn bone_mut(&mut self) -> &mut super::entity::GameplayBones {
        &mut self.bone
    }
}

impl EntityTransformLinkDataTrait for EntryControllableTransformLinkData {
}

impl EntityLinkDataTrait for EntryControllableTransformLinkData {
    fn link_name(&self) -> &String {
        self._glacier_base.link_name()
    }
    fn link_name_mut(&mut self) -> &mut String {
        self._glacier_base.link_name_mut()
    }
}

impl super::core::DataContainerTrait for EntryControllableTransformLinkData {
}

pub static ENTRYCONTROLLABLETRANSFORMLINKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntryControllableTransformLinkData",
    name_hash: 2988851404,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYTRANSFORMLINKDATA_TYPE_INFO),
        super_class_offset: offset_of!(EntryControllableTransformLinkData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EntryControllableTransformLinkData as Default>::default())),
            create_boxed: || Box::new(<EntryControllableTransformLinkData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Bone",
                name_hash: 2088812771,
                flags: MemberInfoFlags::new(0),
                field_type: "GameplayBones",
                rust_offset: offset_of!(EntryControllableTransformLinkData, bone),
            },
        ],
    }),
    array_type: Some(ENTRYCONTROLLABLETRANSFORMLINKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EntryControllableTransformLinkData {
    fn type_info(&self) -> &'static TypeInfo {
        ENTRYCONTROLLABLETRANSFORMLINKDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static ENTRYCONTROLLABLETRANSFORMLINKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntryControllableTransformLinkData-Array",
    name_hash: 1386578296,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("EntryControllableTransformLinkData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ComponentTransformLinkData {
    pub _glacier_base: EntityTransformLinkData,
}

pub trait ComponentTransformLinkDataTrait: EntityTransformLinkDataTrait {
}

impl ComponentTransformLinkDataTrait for ComponentTransformLinkData {
}

impl EntityTransformLinkDataTrait for ComponentTransformLinkData {
}

impl EntityLinkDataTrait for ComponentTransformLinkData {
    fn link_name(&self) -> &String {
        self._glacier_base.link_name()
    }
    fn link_name_mut(&mut self) -> &mut String {
        self._glacier_base.link_name_mut()
    }
}

impl super::core::DataContainerTrait for ComponentTransformLinkData {
}

pub static COMPONENTTRANSFORMLINKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ComponentTransformLinkData",
    name_hash: 3918222166,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYTRANSFORMLINKDATA_TYPE_INFO),
        super_class_offset: offset_of!(ComponentTransformLinkData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ComponentTransformLinkData as Default>::default())),
            create_boxed: || Box::new(<ComponentTransformLinkData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(COMPONENTTRANSFORMLINKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ComponentTransformLinkData {
    fn type_info(&self) -> &'static TypeInfo {
        COMPONENTTRANSFORMLINKDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static COMPONENTTRANSFORMLINKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ComponentTransformLinkData-Array",
    name_hash: 4269431010,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ComponentTransformLinkData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ModelAnimationTransformLinkData {
    pub _glacier_base: EntityTransformLinkData,
    pub bone: super::entity::GameplayBones,
}

pub trait ModelAnimationTransformLinkDataTrait: EntityTransformLinkDataTrait {
    fn bone(&self) -> &super::entity::GameplayBones;
    fn bone_mut(&mut self) -> &mut super::entity::GameplayBones;
}

impl ModelAnimationTransformLinkDataTrait for ModelAnimationTransformLinkData {
    fn bone(&self) -> &super::entity::GameplayBones {
        &self.bone
    }
    fn bone_mut(&mut self) -> &mut super::entity::GameplayBones {
        &mut self.bone
    }
}

impl EntityTransformLinkDataTrait for ModelAnimationTransformLinkData {
}

impl EntityLinkDataTrait for ModelAnimationTransformLinkData {
    fn link_name(&self) -> &String {
        self._glacier_base.link_name()
    }
    fn link_name_mut(&mut self) -> &mut String {
        self._glacier_base.link_name_mut()
    }
}

impl super::core::DataContainerTrait for ModelAnimationTransformLinkData {
}

pub static MODELANIMATIONTRANSFORMLINKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ModelAnimationTransformLinkData",
    name_hash: 3424450464,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYTRANSFORMLINKDATA_TYPE_INFO),
        super_class_offset: offset_of!(ModelAnimationTransformLinkData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ModelAnimationTransformLinkData as Default>::default())),
            create_boxed: || Box::new(<ModelAnimationTransformLinkData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Bone",
                name_hash: 2088812771,
                flags: MemberInfoFlags::new(0),
                field_type: "GameplayBones",
                rust_offset: offset_of!(ModelAnimationTransformLinkData, bone),
            },
        ],
    }),
    array_type: Some(MODELANIMATIONTRANSFORMLINKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ModelAnimationTransformLinkData {
    fn type_info(&self) -> &'static TypeInfo {
        MODELANIMATIONTRANSFORMLINKDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static MODELANIMATIONTRANSFORMLINKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ModelAnimationTransformLinkData-Array",
    name_hash: 3591806228,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ModelAnimationTransformLinkData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AnimatableTransformLinkData {
    pub _glacier_base: EntityTransformLinkData,
    pub bone: super::entity::GameplayBones,
}

pub trait AnimatableTransformLinkDataTrait: EntityTransformLinkDataTrait {
    fn bone(&self) -> &super::entity::GameplayBones;
    fn bone_mut(&mut self) -> &mut super::entity::GameplayBones;
}

impl AnimatableTransformLinkDataTrait for AnimatableTransformLinkData {
    fn bone(&self) -> &super::entity::GameplayBones {
        &self.bone
    }
    fn bone_mut(&mut self) -> &mut super::entity::GameplayBones {
        &mut self.bone
    }
}

impl EntityTransformLinkDataTrait for AnimatableTransformLinkData {
}

impl EntityLinkDataTrait for AnimatableTransformLinkData {
    fn link_name(&self) -> &String {
        self._glacier_base.link_name()
    }
    fn link_name_mut(&mut self) -> &mut String {
        self._glacier_base.link_name_mut()
    }
}

impl super::core::DataContainerTrait for AnimatableTransformLinkData {
}

pub static ANIMATABLETRANSFORMLINKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimatableTransformLinkData",
    name_hash: 1397797645,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYTRANSFORMLINKDATA_TYPE_INFO),
        super_class_offset: offset_of!(AnimatableTransformLinkData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AnimatableTransformLinkData as Default>::default())),
            create_boxed: || Box::new(<AnimatableTransformLinkData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Bone",
                name_hash: 2088812771,
                flags: MemberInfoFlags::new(0),
                field_type: "GameplayBones",
                rust_offset: offset_of!(AnimatableTransformLinkData, bone),
            },
        ],
    }),
    array_type: Some(ANIMATABLETRANSFORMLINKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AnimatableTransformLinkData {
    fn type_info(&self) -> &'static TypeInfo {
        ANIMATABLETRANSFORMLINKDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static ANIMATABLETRANSFORMLINKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimatableTransformLinkData-Array",
    name_hash: 200505017,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("AnimatableTransformLinkData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EntityCenterTransformLinkData {
    pub _glacier_base: EntityTransformLinkData,
}

pub trait EntityCenterTransformLinkDataTrait: EntityTransformLinkDataTrait {
}

impl EntityCenterTransformLinkDataTrait for EntityCenterTransformLinkData {
}

impl EntityTransformLinkDataTrait for EntityCenterTransformLinkData {
}

impl EntityLinkDataTrait for EntityCenterTransformLinkData {
    fn link_name(&self) -> &String {
        self._glacier_base.link_name()
    }
    fn link_name_mut(&mut self) -> &mut String {
        self._glacier_base.link_name_mut()
    }
}

impl super::core::DataContainerTrait for EntityCenterTransformLinkData {
}

pub static ENTITYCENTERTRANSFORMLINKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityCenterTransformLinkData",
    name_hash: 4276905353,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYTRANSFORMLINKDATA_TYPE_INFO),
        super_class_offset: offset_of!(EntityCenterTransformLinkData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EntityCenterTransformLinkData as Default>::default())),
            create_boxed: || Box::new(<EntityCenterTransformLinkData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(ENTITYCENTERTRANSFORMLINKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EntityCenterTransformLinkData {
    fn type_info(&self) -> &'static TypeInfo {
        ENTITYCENTERTRANSFORMLINKDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static ENTITYCENTERTRANSFORMLINKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityCenterTransformLinkData-Array",
    name_hash: 434854717,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("EntityCenterTransformLinkData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EntityTransformLinkData {
    pub _glacier_base: EntityLinkData,
}

pub trait EntityTransformLinkDataTrait: EntityLinkDataTrait {
}

impl EntityTransformLinkDataTrait for EntityTransformLinkData {
}

impl EntityLinkDataTrait for EntityTransformLinkData {
    fn link_name(&self) -> &String {
        self._glacier_base.link_name()
    }
    fn link_name_mut(&mut self) -> &mut String {
        self._glacier_base.link_name_mut()
    }
}

impl super::core::DataContainerTrait for EntityTransformLinkData {
}

pub static ENTITYTRANSFORMLINKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityTransformLinkData",
    name_hash: 188361922,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYLINKDATA_TYPE_INFO),
        super_class_offset: offset_of!(EntityTransformLinkData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EntityTransformLinkData as Default>::default())),
            create_boxed: || Box::new(<EntityTransformLinkData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(ENTITYTRANSFORMLINKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EntityTransformLinkData {
    fn type_info(&self) -> &'static TypeInfo {
        ENTITYTRANSFORMLINKDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static ENTITYTRANSFORMLINKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityTransformLinkData-Array",
    name_hash: 3564362870,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("EntityTransformLinkData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EntityLinkData {
    pub _glacier_base: super::core::DataContainer,
    pub link_name: String,
}

pub trait EntityLinkDataTrait: super::core::DataContainerTrait {
    fn link_name(&self) -> &String;
    fn link_name_mut(&mut self) -> &mut String;
}

impl EntityLinkDataTrait for EntityLinkData {
    fn link_name(&self) -> &String {
        &self.link_name
    }
    fn link_name_mut(&mut self) -> &mut String {
        &mut self.link_name
    }
}

impl super::core::DataContainerTrait for EntityLinkData {
}

pub static ENTITYLINKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityLinkData",
    name_hash: 2885135822,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(EntityLinkData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EntityLinkData as Default>::default())),
            create_boxed: || Box::new(<EntityLinkData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "LinkName",
                name_hash: 2750159970,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(EntityLinkData, link_name),
            },
        ],
    }),
    array_type: Some(ENTITYLINKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EntityLinkData {
    fn type_info(&self) -> &'static TypeInfo {
        ENTITYLINKDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static ENTITYLINKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityLinkData-Array",
    name_hash: 3410831226,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("EntityLinkData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct OffsetModificationData {
    pub _glacier_base: super::core::DataContainer,
    pub offset_x_axis_in_world_space: bool,
    pub offset_y_axis_in_world_space: bool,
    pub offset_z_axis_in_world_space: bool,
    pub offset: super::core::LinearTransform,
}

pub trait OffsetModificationDataTrait: super::core::DataContainerTrait {
    fn offset_x_axis_in_world_space(&self) -> &bool;
    fn offset_x_axis_in_world_space_mut(&mut self) -> &mut bool;
    fn offset_y_axis_in_world_space(&self) -> &bool;
    fn offset_y_axis_in_world_space_mut(&mut self) -> &mut bool;
    fn offset_z_axis_in_world_space(&self) -> &bool;
    fn offset_z_axis_in_world_space_mut(&mut self) -> &mut bool;
    fn offset(&self) -> &super::core::LinearTransform;
    fn offset_mut(&mut self) -> &mut super::core::LinearTransform;
}

impl OffsetModificationDataTrait for OffsetModificationData {
    fn offset_x_axis_in_world_space(&self) -> &bool {
        &self.offset_x_axis_in_world_space
    }
    fn offset_x_axis_in_world_space_mut(&mut self) -> &mut bool {
        &mut self.offset_x_axis_in_world_space
    }
    fn offset_y_axis_in_world_space(&self) -> &bool {
        &self.offset_y_axis_in_world_space
    }
    fn offset_y_axis_in_world_space_mut(&mut self) -> &mut bool {
        &mut self.offset_y_axis_in_world_space
    }
    fn offset_z_axis_in_world_space(&self) -> &bool {
        &self.offset_z_axis_in_world_space
    }
    fn offset_z_axis_in_world_space_mut(&mut self) -> &mut bool {
        &mut self.offset_z_axis_in_world_space
    }
    fn offset(&self) -> &super::core::LinearTransform {
        &self.offset
    }
    fn offset_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.offset
    }
}

impl super::core::DataContainerTrait for OffsetModificationData {
}

pub static OFFSETMODIFICATIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OffsetModificationData",
    name_hash: 497203078,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(OffsetModificationData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OffsetModificationData as Default>::default())),
            create_boxed: || Box::new(<OffsetModificationData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "OffsetXAxisInWorldSpace",
                name_hash: 1519201938,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OffsetModificationData, offset_x_axis_in_world_space),
            },
            FieldInfoData {
                name: "OffsetYAxisInWorldSpace",
                name_hash: 3463175187,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OffsetModificationData, offset_y_axis_in_world_space),
            },
            FieldInfoData {
                name: "OffsetZAxisInWorldSpace",
                name_hash: 1673850640,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OffsetModificationData, offset_z_axis_in_world_space),
            },
            FieldInfoData {
                name: "Offset",
                name_hash: 2871410728,
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(OffsetModificationData, offset),
            },
        ],
    }),
    array_type: Some(OFFSETMODIFICATIONDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for OffsetModificationData {
    fn type_info(&self) -> &'static TypeInfo {
        OFFSETMODIFICATIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static OFFSETMODIFICATIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OffsetModificationData-Array",
    name_hash: 3443057842,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("OffsetModificationData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CoordinateModificationData {
    pub _glacier_base: super::core::DataContainer,
    pub left: super::entity::ModifierAxis,
    pub up: super::entity::ModifierAxis,
    pub forward: super::entity::ModifierAxis,
    pub invert_left: bool,
    pub invert_up: bool,
    pub invert_forward: bool,
}

pub trait CoordinateModificationDataTrait: super::core::DataContainerTrait {
    fn left(&self) -> &super::entity::ModifierAxis;
    fn left_mut(&mut self) -> &mut super::entity::ModifierAxis;
    fn up(&self) -> &super::entity::ModifierAxis;
    fn up_mut(&mut self) -> &mut super::entity::ModifierAxis;
    fn forward(&self) -> &super::entity::ModifierAxis;
    fn forward_mut(&mut self) -> &mut super::entity::ModifierAxis;
    fn invert_left(&self) -> &bool;
    fn invert_left_mut(&mut self) -> &mut bool;
    fn invert_up(&self) -> &bool;
    fn invert_up_mut(&mut self) -> &mut bool;
    fn invert_forward(&self) -> &bool;
    fn invert_forward_mut(&mut self) -> &mut bool;
}

impl CoordinateModificationDataTrait for CoordinateModificationData {
    fn left(&self) -> &super::entity::ModifierAxis {
        &self.left
    }
    fn left_mut(&mut self) -> &mut super::entity::ModifierAxis {
        &mut self.left
    }
    fn up(&self) -> &super::entity::ModifierAxis {
        &self.up
    }
    fn up_mut(&mut self) -> &mut super::entity::ModifierAxis {
        &mut self.up
    }
    fn forward(&self) -> &super::entity::ModifierAxis {
        &self.forward
    }
    fn forward_mut(&mut self) -> &mut super::entity::ModifierAxis {
        &mut self.forward
    }
    fn invert_left(&self) -> &bool {
        &self.invert_left
    }
    fn invert_left_mut(&mut self) -> &mut bool {
        &mut self.invert_left
    }
    fn invert_up(&self) -> &bool {
        &self.invert_up
    }
    fn invert_up_mut(&mut self) -> &mut bool {
        &mut self.invert_up
    }
    fn invert_forward(&self) -> &bool {
        &self.invert_forward
    }
    fn invert_forward_mut(&mut self) -> &mut bool {
        &mut self.invert_forward
    }
}

impl super::core::DataContainerTrait for CoordinateModificationData {
}

pub static COORDINATEMODIFICATIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoordinateModificationData",
    name_hash: 102932745,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(CoordinateModificationData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoordinateModificationData as Default>::default())),
            create_boxed: || Box::new(<CoordinateModificationData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Left",
                name_hash: 2089021886,
                flags: MemberInfoFlags::new(0),
                field_type: "ModifierAxis",
                rust_offset: offset_of!(CoordinateModificationData, left),
            },
            FieldInfoData {
                name: "Up",
                name_hash: 5862272,
                flags: MemberInfoFlags::new(0),
                field_type: "ModifierAxis",
                rust_offset: offset_of!(CoordinateModificationData, up),
            },
            FieldInfoData {
                name: "Forward",
                name_hash: 1986470206,
                flags: MemberInfoFlags::new(0),
                field_type: "ModifierAxis",
                rust_offset: offset_of!(CoordinateModificationData, forward),
            },
            FieldInfoData {
                name: "InvertLeft",
                name_hash: 3753810092,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CoordinateModificationData, invert_left),
            },
            FieldInfoData {
                name: "InvertUp",
                name_hash: 58661394,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CoordinateModificationData, invert_up),
            },
            FieldInfoData {
                name: "InvertForward",
                name_hash: 3827158252,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CoordinateModificationData, invert_forward),
            },
        ],
    }),
    array_type: Some(COORDINATEMODIFICATIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CoordinateModificationData {
    fn type_info(&self) -> &'static TypeInfo {
        COORDINATEMODIFICATIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static COORDINATEMODIFICATIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoordinateModificationData-Array",
    name_hash: 1522546365,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("CoordinateModificationData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WidgetReferenceEntityData {
    pub _glacier_base: super::entity::LogicReferenceObjectData,
}

pub trait WidgetReferenceEntityDataTrait: super::entity::LogicReferenceObjectDataTrait {
}

impl WidgetReferenceEntityDataTrait for WidgetReferenceEntityData {
}

impl super::entity::LogicReferenceObjectDataTrait for WidgetReferenceEntityData {
    fn local_player_id(&self) -> &super::core::LocalPlayerId {
        self._glacier_base.local_player_id()
    }
    fn local_player_id_mut(&mut self) -> &mut super::core::LocalPlayerId {
        self._glacier_base.local_player_id_mut()
    }
    fn sub_realm(&self) -> &super::entity::SubRealm {
        self._glacier_base.sub_realm()
    }
    fn sub_realm_mut(&mut self) -> &mut super::entity::SubRealm {
        self._glacier_base.sub_realm_mut()
    }
}

impl super::entity::ReferenceObjectDataTrait for WidgetReferenceEntityData {
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

impl super::entity::GameObjectDataTrait for WidgetReferenceEntityData {
}

impl super::core::DataBusPeerTrait for WidgetReferenceEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for WidgetReferenceEntityData {
}

impl super::core::DataContainerTrait for WidgetReferenceEntityData {
}

pub static WIDGETREFERENCEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WidgetReferenceEntityData",
    name_hash: 83876777,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::LOGICREFERENCEOBJECTDATA_TYPE_INFO),
        super_class_offset: offset_of!(WidgetReferenceEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WidgetReferenceEntityData as Default>::default())),
            create_boxed: || Box::new(<WidgetReferenceEntityData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(WIDGETREFERENCEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WidgetReferenceEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        WIDGETREFERENCEENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static WIDGETREFERENCEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WidgetReferenceEntityData-Array",
    name_hash: 2783650333,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("WidgetReferenceEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DiceUIVectorShapeAsset {
    pub _glacier_base: super::core::DataContainerPolicyAsset,
    pub shapes: Vec<BoxedTypeObject /* DiceUIVectorShape */>,
    pub layout_rect: super::core::Vec4,
}

pub trait DiceUIVectorShapeAssetTrait: super::core::DataContainerPolicyAssetTrait {
    fn shapes(&self) -> &Vec<BoxedTypeObject /* DiceUIVectorShape */>;
    fn shapes_mut(&mut self) -> &mut Vec<BoxedTypeObject /* DiceUIVectorShape */>;
    fn layout_rect(&self) -> &super::core::Vec4;
    fn layout_rect_mut(&mut self) -> &mut super::core::Vec4;
}

impl DiceUIVectorShapeAssetTrait for DiceUIVectorShapeAsset {
    fn shapes(&self) -> &Vec<BoxedTypeObject /* DiceUIVectorShape */> {
        &self.shapes
    }
    fn shapes_mut(&mut self) -> &mut Vec<BoxedTypeObject /* DiceUIVectorShape */> {
        &mut self.shapes
    }
    fn layout_rect(&self) -> &super::core::Vec4 {
        &self.layout_rect
    }
    fn layout_rect_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.layout_rect
    }
}

impl super::core::DataContainerPolicyAssetTrait for DiceUIVectorShapeAsset {
}

impl super::core::AssetTrait for DiceUIVectorShapeAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for DiceUIVectorShapeAsset {
}

pub static DICEUIVECTORSHAPEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIVectorShapeAsset",
    name_hash: 393991732,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINERPOLICYASSET_TYPE_INFO),
        super_class_offset: offset_of!(DiceUIVectorShapeAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DiceUIVectorShapeAsset as Default>::default())),
            create_boxed: || Box::new(<DiceUIVectorShapeAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Shapes",
                name_hash: 3352896601,
                flags: MemberInfoFlags::new(144),
                field_type: "DiceUIVectorShape-Array",
                rust_offset: offset_of!(DiceUIVectorShapeAsset, shapes),
            },
            FieldInfoData {
                name: "LayoutRect",
                name_hash: 1931697087,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(DiceUIVectorShapeAsset, layout_rect),
            },
        ],
    }),
    array_type: Some(DICEUIVECTORSHAPEASSET_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DiceUIVectorShapeAsset {
    fn type_info(&self) -> &'static TypeInfo {
        DICEUIVECTORSHAPEASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DICEUIVECTORSHAPEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIVectorShapeAsset-Array",
    name_hash: 513069440,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceUIVectorShapeAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DiceUIVectorShape {
    pub color: super::core::Vec3,
    pub alpha: f32,
    pub specify_inner_outer_widths: bool,
    pub inner_width: f32,
    pub outer_width: f32,
    pub line_width: f32,
    pub start_cap_type: DiceUIVectorShapeCapType,
    pub end_cap_type: DiceUIVectorShapeCapType,
    pub draw_style: DiceUIVectorShapeDrawStyle,
    pub path: DiceUIVectorPath,
    pub inner_paths: Vec<BoxedTypeObject /* DiceUIVectorPath */>,
    pub triangulated_points: Vec<BoxedTypeObject /* super::core::Vec2 */>,
    pub triangulated_expansions: Vec<BoxedTypeObject /* super::core::Vec2 */>,
    pub triangulated_colors: Vec<BoxedTypeObject /* super::core::Vec4 */>,
    pub triangulated_indices: Vec<u16>,
}

pub trait DiceUIVectorShapeTrait: TypeObject {
    fn color(&self) -> &super::core::Vec3;
    fn color_mut(&mut self) -> &mut super::core::Vec3;
    fn alpha(&self) -> &f32;
    fn alpha_mut(&mut self) -> &mut f32;
    fn specify_inner_outer_widths(&self) -> &bool;
    fn specify_inner_outer_widths_mut(&mut self) -> &mut bool;
    fn inner_width(&self) -> &f32;
    fn inner_width_mut(&mut self) -> &mut f32;
    fn outer_width(&self) -> &f32;
    fn outer_width_mut(&mut self) -> &mut f32;
    fn line_width(&self) -> &f32;
    fn line_width_mut(&mut self) -> &mut f32;
    fn start_cap_type(&self) -> &DiceUIVectorShapeCapType;
    fn start_cap_type_mut(&mut self) -> &mut DiceUIVectorShapeCapType;
    fn end_cap_type(&self) -> &DiceUIVectorShapeCapType;
    fn end_cap_type_mut(&mut self) -> &mut DiceUIVectorShapeCapType;
    fn draw_style(&self) -> &DiceUIVectorShapeDrawStyle;
    fn draw_style_mut(&mut self) -> &mut DiceUIVectorShapeDrawStyle;
    fn path(&self) -> &DiceUIVectorPath;
    fn path_mut(&mut self) -> &mut DiceUIVectorPath;
    fn inner_paths(&self) -> &Vec<BoxedTypeObject /* DiceUIVectorPath */>;
    fn inner_paths_mut(&mut self) -> &mut Vec<BoxedTypeObject /* DiceUIVectorPath */>;
    fn triangulated_points(&self) -> &Vec<BoxedTypeObject /* super::core::Vec2 */>;
    fn triangulated_points_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::core::Vec2 */>;
    fn triangulated_expansions(&self) -> &Vec<BoxedTypeObject /* super::core::Vec2 */>;
    fn triangulated_expansions_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::core::Vec2 */>;
    fn triangulated_colors(&self) -> &Vec<BoxedTypeObject /* super::core::Vec4 */>;
    fn triangulated_colors_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::core::Vec4 */>;
    fn triangulated_indices(&self) -> &Vec<u16>;
    fn triangulated_indices_mut(&mut self) -> &mut Vec<u16>;
}

impl DiceUIVectorShapeTrait for DiceUIVectorShape {
    fn color(&self) -> &super::core::Vec3 {
        &self.color
    }
    fn color_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.color
    }
    fn alpha(&self) -> &f32 {
        &self.alpha
    }
    fn alpha_mut(&mut self) -> &mut f32 {
        &mut self.alpha
    }
    fn specify_inner_outer_widths(&self) -> &bool {
        &self.specify_inner_outer_widths
    }
    fn specify_inner_outer_widths_mut(&mut self) -> &mut bool {
        &mut self.specify_inner_outer_widths
    }
    fn inner_width(&self) -> &f32 {
        &self.inner_width
    }
    fn inner_width_mut(&mut self) -> &mut f32 {
        &mut self.inner_width
    }
    fn outer_width(&self) -> &f32 {
        &self.outer_width
    }
    fn outer_width_mut(&mut self) -> &mut f32 {
        &mut self.outer_width
    }
    fn line_width(&self) -> &f32 {
        &self.line_width
    }
    fn line_width_mut(&mut self) -> &mut f32 {
        &mut self.line_width
    }
    fn start_cap_type(&self) -> &DiceUIVectorShapeCapType {
        &self.start_cap_type
    }
    fn start_cap_type_mut(&mut self) -> &mut DiceUIVectorShapeCapType {
        &mut self.start_cap_type
    }
    fn end_cap_type(&self) -> &DiceUIVectorShapeCapType {
        &self.end_cap_type
    }
    fn end_cap_type_mut(&mut self) -> &mut DiceUIVectorShapeCapType {
        &mut self.end_cap_type
    }
    fn draw_style(&self) -> &DiceUIVectorShapeDrawStyle {
        &self.draw_style
    }
    fn draw_style_mut(&mut self) -> &mut DiceUIVectorShapeDrawStyle {
        &mut self.draw_style
    }
    fn path(&self) -> &DiceUIVectorPath {
        &self.path
    }
    fn path_mut(&mut self) -> &mut DiceUIVectorPath {
        &mut self.path
    }
    fn inner_paths(&self) -> &Vec<BoxedTypeObject /* DiceUIVectorPath */> {
        &self.inner_paths
    }
    fn inner_paths_mut(&mut self) -> &mut Vec<BoxedTypeObject /* DiceUIVectorPath */> {
        &mut self.inner_paths
    }
    fn triangulated_points(&self) -> &Vec<BoxedTypeObject /* super::core::Vec2 */> {
        &self.triangulated_points
    }
    fn triangulated_points_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::core::Vec2 */> {
        &mut self.triangulated_points
    }
    fn triangulated_expansions(&self) -> &Vec<BoxedTypeObject /* super::core::Vec2 */> {
        &self.triangulated_expansions
    }
    fn triangulated_expansions_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::core::Vec2 */> {
        &mut self.triangulated_expansions
    }
    fn triangulated_colors(&self) -> &Vec<BoxedTypeObject /* super::core::Vec4 */> {
        &self.triangulated_colors
    }
    fn triangulated_colors_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::core::Vec4 */> {
        &mut self.triangulated_colors
    }
    fn triangulated_indices(&self) -> &Vec<u16> {
        &self.triangulated_indices
    }
    fn triangulated_indices_mut(&mut self) -> &mut Vec<u16> {
        &mut self.triangulated_indices
    }
}

pub static DICEUIVECTORSHAPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIVectorShape",
    name_hash: 3977270884,
    flags: MemberInfoFlags::new(73),
    module: "DiceCommonsShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DiceUIVectorShape as Default>::default())),
            create_boxed: || Box::new(<DiceUIVectorShape as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Color",
                name_hash: 212387320,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(DiceUIVectorShape, color),
            },
            FieldInfoData {
                name: "Alpha",
                name_hash: 205677681,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DiceUIVectorShape, alpha),
            },
            FieldInfoData {
                name: "SpecifyInnerOuterWidths",
                name_hash: 2633981732,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DiceUIVectorShape, specify_inner_outer_widths),
            },
            FieldInfoData {
                name: "InnerWidth",
                name_hash: 3360296221,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DiceUIVectorShape, inner_width),
            },
            FieldInfoData {
                name: "OuterWidth",
                name_hash: 4272208858,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DiceUIVectorShape, outer_width),
            },
            FieldInfoData {
                name: "LineWidth",
                name_hash: 962699949,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DiceUIVectorShape, line_width),
            },
            FieldInfoData {
                name: "StartCapType",
                name_hash: 2680344719,
                flags: MemberInfoFlags::new(0),
                field_type: "DiceUIVectorShapeCapType",
                rust_offset: offset_of!(DiceUIVectorShape, start_cap_type),
            },
            FieldInfoData {
                name: "EndCapType",
                name_hash: 3317185248,
                flags: MemberInfoFlags::new(0),
                field_type: "DiceUIVectorShapeCapType",
                rust_offset: offset_of!(DiceUIVectorShape, end_cap_type),
            },
            FieldInfoData {
                name: "DrawStyle",
                name_hash: 2413167986,
                flags: MemberInfoFlags::new(0),
                field_type: "DiceUIVectorShapeDrawStyle",
                rust_offset: offset_of!(DiceUIVectorShape, draw_style),
            },
            FieldInfoData {
                name: "Path",
                name_hash: 2089448296,
                flags: MemberInfoFlags::new(0),
                field_type: "DiceUIVectorPath",
                rust_offset: offset_of!(DiceUIVectorShape, path),
            },
            FieldInfoData {
                name: "InnerPaths",
                name_hash: 3361792709,
                flags: MemberInfoFlags::new(144),
                field_type: "DiceUIVectorPath-Array",
                rust_offset: offset_of!(DiceUIVectorShape, inner_paths),
            },
            FieldInfoData {
                name: "TriangulatedPoints",
                name_hash: 931656624,
                flags: MemberInfoFlags::new(144),
                field_type: "Vec2-Array",
                rust_offset: offset_of!(DiceUIVectorShape, triangulated_points),
            },
            FieldInfoData {
                name: "TriangulatedExpansions",
                name_hash: 2612011461,
                flags: MemberInfoFlags::new(144),
                field_type: "Vec2-Array",
                rust_offset: offset_of!(DiceUIVectorShape, triangulated_expansions),
            },
            FieldInfoData {
                name: "TriangulatedColors",
                name_hash: 123993569,
                flags: MemberInfoFlags::new(144),
                field_type: "Vec4-Array",
                rust_offset: offset_of!(DiceUIVectorShape, triangulated_colors),
            },
            FieldInfoData {
                name: "TriangulatedIndices",
                name_hash: 1550552400,
                flags: MemberInfoFlags::new(144),
                field_type: "Uint16-Array",
                rust_offset: offset_of!(DiceUIVectorShape, triangulated_indices),
            },
        ],
    }),
    array_type: Some(DICEUIVECTORSHAPE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DiceUIVectorShape {
    fn type_info(&self) -> &'static TypeInfo {
        DICEUIVECTORSHAPE_TYPE_INFO
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


pub static DICEUIVECTORSHAPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIVectorShape-Array",
    name_hash: 1472313936,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceUIVectorShape"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DiceUIVectorPath {
    pub corners: Vec<BoxedTypeObject /* DiceUIVectorPathCorner */>,
}

pub trait DiceUIVectorPathTrait: TypeObject {
    fn corners(&self) -> &Vec<BoxedTypeObject /* DiceUIVectorPathCorner */>;
    fn corners_mut(&mut self) -> &mut Vec<BoxedTypeObject /* DiceUIVectorPathCorner */>;
}

impl DiceUIVectorPathTrait for DiceUIVectorPath {
    fn corners(&self) -> &Vec<BoxedTypeObject /* DiceUIVectorPathCorner */> {
        &self.corners
    }
    fn corners_mut(&mut self) -> &mut Vec<BoxedTypeObject /* DiceUIVectorPathCorner */> {
        &mut self.corners
    }
}

pub static DICEUIVECTORPATH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIVectorPath",
    name_hash: 4285518374,
    flags: MemberInfoFlags::new(73),
    module: "DiceCommonsShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DiceUIVectorPath as Default>::default())),
            create_boxed: || Box::new(<DiceUIVectorPath as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Corners",
                name_hash: 3677527825,
                flags: MemberInfoFlags::new(144),
                field_type: "DiceUIVectorPathCorner-Array",
                rust_offset: offset_of!(DiceUIVectorPath, corners),
            },
        ],
    }),
    array_type: Some(DICEUIVECTORPATH_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DiceUIVectorPath {
    fn type_info(&self) -> &'static TypeInfo {
        DICEUIVECTORPATH_TYPE_INFO
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


pub static DICEUIVECTORPATH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIVectorPath-Array",
    name_hash: 255557266,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceUIVectorPath"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DiceUIVectorPathCorner {
    pub corner_type: DiceUIVectorPathCornerType,
    pub radius: f32,
    pub position: super::core::Vec2,
    pub expansion: super::core::Vec2,
    pub color: super::core::Vec3,
    pub alpha: f32,
}

pub trait DiceUIVectorPathCornerTrait: TypeObject {
    fn corner_type(&self) -> &DiceUIVectorPathCornerType;
    fn corner_type_mut(&mut self) -> &mut DiceUIVectorPathCornerType;
    fn radius(&self) -> &f32;
    fn radius_mut(&mut self) -> &mut f32;
    fn position(&self) -> &super::core::Vec2;
    fn position_mut(&mut self) -> &mut super::core::Vec2;
    fn expansion(&self) -> &super::core::Vec2;
    fn expansion_mut(&mut self) -> &mut super::core::Vec2;
    fn color(&self) -> &super::core::Vec3;
    fn color_mut(&mut self) -> &mut super::core::Vec3;
    fn alpha(&self) -> &f32;
    fn alpha_mut(&mut self) -> &mut f32;
}

impl DiceUIVectorPathCornerTrait for DiceUIVectorPathCorner {
    fn corner_type(&self) -> &DiceUIVectorPathCornerType {
        &self.corner_type
    }
    fn corner_type_mut(&mut self) -> &mut DiceUIVectorPathCornerType {
        &mut self.corner_type
    }
    fn radius(&self) -> &f32 {
        &self.radius
    }
    fn radius_mut(&mut self) -> &mut f32 {
        &mut self.radius
    }
    fn position(&self) -> &super::core::Vec2 {
        &self.position
    }
    fn position_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.position
    }
    fn expansion(&self) -> &super::core::Vec2 {
        &self.expansion
    }
    fn expansion_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.expansion
    }
    fn color(&self) -> &super::core::Vec3 {
        &self.color
    }
    fn color_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.color
    }
    fn alpha(&self) -> &f32 {
        &self.alpha
    }
    fn alpha_mut(&mut self) -> &mut f32 {
        &mut self.alpha
    }
}

pub static DICEUIVECTORPATHCORNER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIVectorPathCorner",
    name_hash: 2952203457,
    flags: MemberInfoFlags::new(36937),
    module: "DiceCommonsShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DiceUIVectorPathCorner as Default>::default())),
            create_boxed: || Box::new(<DiceUIVectorPathCorner as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "CornerType",
                name_hash: 3174963866,
                flags: MemberInfoFlags::new(0),
                field_type: "DiceUIVectorPathCornerType",
                rust_offset: offset_of!(DiceUIVectorPathCorner, corner_type),
            },
            FieldInfoData {
                name: "Radius",
                name_hash: 3298407133,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DiceUIVectorPathCorner, radius),
            },
            FieldInfoData {
                name: "Position",
                name_hash: 3402582524,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(DiceUIVectorPathCorner, position),
            },
            FieldInfoData {
                name: "Expansion",
                name_hash: 2333524892,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(DiceUIVectorPathCorner, expansion),
            },
            FieldInfoData {
                name: "Color",
                name_hash: 212387320,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(DiceUIVectorPathCorner, color),
            },
            FieldInfoData {
                name: "Alpha",
                name_hash: 205677681,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DiceUIVectorPathCorner, alpha),
            },
        ],
    }),
    array_type: Some(DICEUIVECTORPATHCORNER_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DiceUIVectorPathCorner {
    fn type_info(&self) -> &'static TypeInfo {
        DICEUIVECTORPATHCORNER_TYPE_INFO
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


pub static DICEUIVECTORPATHCORNER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIVectorPathCorner-Array",
    name_hash: 459478773,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceUIVectorPathCorner"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum DiceUIVectorPathCornerType {
    #[default]
    DiceUIVectorPathCornerType_Bevel = 0,
    DiceUIVectorPathCornerType_Miter = 1,
    DiceUIVectorPathCornerType_Rounded = 2,
}

pub static DICEUIVECTORPATHCORNERTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIVectorPathCornerType",
    name_hash: 3123231801,
    flags: MemberInfoFlags::new(49429),
    module: "DiceCommonsShared",
    data: TypeInfoData::Enum,
    array_type: Some(DICEUIVECTORPATHCORNERTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DiceUIVectorPathCornerType {
    fn type_info(&self) -> &'static TypeInfo {
        DICEUIVECTORPATHCORNERTYPE_TYPE_INFO
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


pub static DICEUIVECTORPATHCORNERTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIVectorPathCornerType-Array",
    name_hash: 3672976013,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceUIVectorPathCornerType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum DiceUIVectorShapeDrawStyle {
    #[default]
    DiceUIVectorShapeDrawStyle_Lines = 0,
    DiceUIVectorShapeDrawStyle_Outline = 1,
    DiceUIVectorShapeDrawStyle_Filled = 2,
}

pub static DICEUIVECTORSHAPEDRAWSTYLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIVectorShapeDrawStyle",
    name_hash: 3065074291,
    flags: MemberInfoFlags::new(49429),
    module: "DiceCommonsShared",
    data: TypeInfoData::Enum,
    array_type: Some(DICEUIVECTORSHAPEDRAWSTYLE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DiceUIVectorShapeDrawStyle {
    fn type_info(&self) -> &'static TypeInfo {
        DICEUIVECTORSHAPEDRAWSTYLE_TYPE_INFO
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


pub static DICEUIVECTORSHAPEDRAWSTYLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIVectorShapeDrawStyle-Array",
    name_hash: 1545244231,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceUIVectorShapeDrawStyle"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum DiceUIVectorShapeCapType {
    #[default]
    DiceUIVectorShapeCapType_None = 0,
    DiceUIVectorShapeCapType_Square = 1,
    DiceUIVectorShapeCapType_Rounded = 2,
}

pub static DICEUIVECTORSHAPECAPTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIVectorShapeCapType",
    name_hash: 2344815982,
    flags: MemberInfoFlags::new(49429),
    module: "DiceCommonsShared",
    data: TypeInfoData::Enum,
    array_type: Some(DICEUIVECTORSHAPECAPTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DiceUIVectorShapeCapType {
    fn type_info(&self) -> &'static TypeInfo {
        DICEUIVECTORSHAPECAPTYPE_TYPE_INFO
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


pub static DICEUIVECTORSHAPECAPTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIVectorShapeCapType-Array",
    name_hash: 1905083994,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceUIVectorShapeCapType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LocalizedStringIdPickerEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub sid: String,
}

pub trait LocalizedStringIdPickerEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn sid(&self) -> &String;
    fn sid_mut(&mut self) -> &mut String;
}

impl LocalizedStringIdPickerEntityDataTrait for LocalizedStringIdPickerEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn sid(&self) -> &String {
        &self.sid
    }
    fn sid_mut(&mut self) -> &mut String {
        &mut self.sid
    }
}

impl super::entity::EntityDataTrait for LocalizedStringIdPickerEntityData {
}

impl super::entity::GameObjectDataTrait for LocalizedStringIdPickerEntityData {
}

impl super::core::DataBusPeerTrait for LocalizedStringIdPickerEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for LocalizedStringIdPickerEntityData {
}

impl super::core::DataContainerTrait for LocalizedStringIdPickerEntityData {
}

pub static LOCALIZEDSTRINGIDPICKERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedStringIdPickerEntityData",
    name_hash: 2899814895,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(LocalizedStringIdPickerEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LocalizedStringIdPickerEntityData as Default>::default())),
            create_boxed: || Box::new(<LocalizedStringIdPickerEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                name_hash: 229961746,
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(LocalizedStringIdPickerEntityData, realm),
            },
            FieldInfoData {
                name: "Sid",
                name_hash: 193466587,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(LocalizedStringIdPickerEntityData, sid),
            },
        ],
    }),
    array_type: Some(LOCALIZEDSTRINGIDPICKERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LocalizedStringIdPickerEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        LOCALIZEDSTRINGIDPICKERENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static LOCALIZEDSTRINGIDPICKERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedStringIdPickerEntityData-Array",
    name_hash: 4071671771,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("LocalizedStringIdPickerEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CheckedLocalizedStringEntityData {
    pub _glacier_base: super::u_i_incubator_shared::LocalizedStringEntityBaseData,
    pub dynamic_string_id: Option<LockedTypeObject /* super::u_i_incubator_shared::LocalizedStringId */>,
    pub debug_string: String,
    pub sid: String,
}

pub trait CheckedLocalizedStringEntityDataTrait: super::u_i_incubator_shared::LocalizedStringEntityBaseDataTrait {
    fn dynamic_string_id(&self) -> &Option<LockedTypeObject /* super::u_i_incubator_shared::LocalizedStringId */>;
    fn dynamic_string_id_mut(&mut self) -> &mut Option<LockedTypeObject /* super::u_i_incubator_shared::LocalizedStringId */>;
    fn debug_string(&self) -> &String;
    fn debug_string_mut(&mut self) -> &mut String;
    fn sid(&self) -> &String;
    fn sid_mut(&mut self) -> &mut String;
}

impl CheckedLocalizedStringEntityDataTrait for CheckedLocalizedStringEntityData {
    fn dynamic_string_id(&self) -> &Option<LockedTypeObject /* super::u_i_incubator_shared::LocalizedStringId */> {
        &self.dynamic_string_id
    }
    fn dynamic_string_id_mut(&mut self) -> &mut Option<LockedTypeObject /* super::u_i_incubator_shared::LocalizedStringId */> {
        &mut self.dynamic_string_id
    }
    fn debug_string(&self) -> &String {
        &self.debug_string
    }
    fn debug_string_mut(&mut self) -> &mut String {
        &mut self.debug_string
    }
    fn sid(&self) -> &String {
        &self.sid
    }
    fn sid_mut(&mut self) -> &mut String {
        &mut self.sid
    }
}

impl super::u_i_incubator_shared::LocalizedStringEntityBaseDataTrait for CheckedLocalizedStringEntityData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        self._glacier_base.realm_mut()
    }
    fn arguments(&self) -> &Vec<super::u_i_incubator_shared::LocalizedStringArgumentType> {
        self._glacier_base.arguments()
    }
    fn arguments_mut(&mut self) -> &mut Vec<super::u_i_incubator_shared::LocalizedStringArgumentType> {
        self._glacier_base.arguments_mut()
    }
    fn argument_hashes(&self) -> &Vec<u32> {
        self._glacier_base.argument_hashes()
    }
    fn argument_hashes_mut(&mut self) -> &mut Vec<u32> {
        self._glacier_base.argument_hashes_mut()
    }
}

impl super::entity::EntityDataTrait for CheckedLocalizedStringEntityData {
}

impl super::entity::GameObjectDataTrait for CheckedLocalizedStringEntityData {
}

impl super::core::DataBusPeerTrait for CheckedLocalizedStringEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for CheckedLocalizedStringEntityData {
}

impl super::core::DataContainerTrait for CheckedLocalizedStringEntityData {
}

pub static CHECKEDLOCALIZEDSTRINGENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CheckedLocalizedStringEntityData",
    name_hash: 3293385027,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::u_i_incubator_shared::LOCALIZEDSTRINGENTITYBASEDATA_TYPE_INFO),
        super_class_offset: offset_of!(CheckedLocalizedStringEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CheckedLocalizedStringEntityData as Default>::default())),
            create_boxed: || Box::new(<CheckedLocalizedStringEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "DynamicStringId",
                name_hash: 1367661832,
                flags: MemberInfoFlags::new(0),
                field_type: "LocalizedStringId",
                rust_offset: offset_of!(CheckedLocalizedStringEntityData, dynamic_string_id),
            },
            FieldInfoData {
                name: "DebugString",
                name_hash: 3982518753,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(CheckedLocalizedStringEntityData, debug_string),
            },
            FieldInfoData {
                name: "Sid",
                name_hash: 193466587,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(CheckedLocalizedStringEntityData, sid),
            },
        ],
    }),
    array_type: Some(CHECKEDLOCALIZEDSTRINGENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CheckedLocalizedStringEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CHECKEDLOCALIZEDSTRINGENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CHECKEDLOCALIZEDSTRINGENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CheckedLocalizedStringEntityData-Array",
    name_hash: 2549527671,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("CheckedLocalizedStringEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MaterialRelationTriggarableEffectData {
    pub _glacier_base: super::entity::PhysicsMaterialRelationPropertyData,
    pub effect: Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>,
}

pub trait MaterialRelationTriggarableEffectDataTrait: super::entity::PhysicsMaterialRelationPropertyDataTrait {
    fn effect(&self) -> &Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>;
    fn effect_mut(&mut self) -> &mut Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>;
}

impl MaterialRelationTriggarableEffectDataTrait for MaterialRelationTriggarableEffectData {
    fn effect(&self) -> &Option<LockedTypeObject /* super::effect_base::EffectBlueprint */> {
        &self.effect
    }
    fn effect_mut(&mut self) -> &mut Option<LockedTypeObject /* super::effect_base::EffectBlueprint */> {
        &mut self.effect
    }
}

impl super::entity::PhysicsMaterialRelationPropertyDataTrait for MaterialRelationTriggarableEffectData {
}

impl super::entity::MaterialRelationPropertyDataTrait for MaterialRelationTriggarableEffectData {
}

impl super::core::DataContainerTrait for MaterialRelationTriggarableEffectData {
}

pub static MATERIALRELATIONTRIGGARABLEEFFECTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialRelationTriggarableEffectData",
    name_hash: 2497643865,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::PHYSICSMATERIALRELATIONPROPERTYDATA_TYPE_INFO),
        super_class_offset: offset_of!(MaterialRelationTriggarableEffectData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MaterialRelationTriggarableEffectData as Default>::default())),
            create_boxed: || Box::new(<MaterialRelationTriggarableEffectData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Effect",
                name_hash: 2332983090,
                flags: MemberInfoFlags::new(0),
                field_type: "EffectBlueprint",
                rust_offset: offset_of!(MaterialRelationTriggarableEffectData, effect),
            },
        ],
    }),
    array_type: Some(MATERIALRELATIONTRIGGARABLEEFFECTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MaterialRelationTriggarableEffectData {
    fn type_info(&self) -> &'static TypeInfo {
        MATERIALRELATIONTRIGGARABLEEFFECTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static MATERIALRELATIONTRIGGARABLEEFFECTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialRelationTriggarableEffectData-Array",
    name_hash: 3415577453,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("MaterialRelationTriggarableEffectData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MaterialBasedEffectComponentData {
    pub _glacier_base: super::entity::GameComponentData,
    pub auto_start: bool,
    pub override_height: f32,
    pub local_player_only: bool,
    pub material: super::entity::MaterialDecl,
    pub transform_effect_with_component: bool,
    pub snapping: SnapType,
    pub use_ray_cast: bool,
    pub spawn_effect_on_lookup_location: bool,
    pub ray_direction: super::core::Vec3,
    pub ray_distance: f32,
    pub see_through: bool,
    pub penetrable: bool,
    pub include_terrain: bool,
    pub max_instances: i32,
    pub effect_parameters: Vec<Option<LockedTypeObject /* super::effect_base::EffectParameter */>>,
}

pub trait MaterialBasedEffectComponentDataTrait: super::entity::GameComponentDataTrait {
    fn auto_start(&self) -> &bool;
    fn auto_start_mut(&mut self) -> &mut bool;
    fn override_height(&self) -> &f32;
    fn override_height_mut(&mut self) -> &mut f32;
    fn local_player_only(&self) -> &bool;
    fn local_player_only_mut(&mut self) -> &mut bool;
    fn material(&self) -> &super::entity::MaterialDecl;
    fn material_mut(&mut self) -> &mut super::entity::MaterialDecl;
    fn transform_effect_with_component(&self) -> &bool;
    fn transform_effect_with_component_mut(&mut self) -> &mut bool;
    fn snapping(&self) -> &SnapType;
    fn snapping_mut(&mut self) -> &mut SnapType;
    fn use_ray_cast(&self) -> &bool;
    fn use_ray_cast_mut(&mut self) -> &mut bool;
    fn spawn_effect_on_lookup_location(&self) -> &bool;
    fn spawn_effect_on_lookup_location_mut(&mut self) -> &mut bool;
    fn ray_direction(&self) -> &super::core::Vec3;
    fn ray_direction_mut(&mut self) -> &mut super::core::Vec3;
    fn ray_distance(&self) -> &f32;
    fn ray_distance_mut(&mut self) -> &mut f32;
    fn see_through(&self) -> &bool;
    fn see_through_mut(&mut self) -> &mut bool;
    fn penetrable(&self) -> &bool;
    fn penetrable_mut(&mut self) -> &mut bool;
    fn include_terrain(&self) -> &bool;
    fn include_terrain_mut(&mut self) -> &mut bool;
    fn max_instances(&self) -> &i32;
    fn max_instances_mut(&mut self) -> &mut i32;
    fn effect_parameters(&self) -> &Vec<Option<LockedTypeObject /* super::effect_base::EffectParameter */>>;
    fn effect_parameters_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::effect_base::EffectParameter */>>;
}

impl MaterialBasedEffectComponentDataTrait for MaterialBasedEffectComponentData {
    fn auto_start(&self) -> &bool {
        &self.auto_start
    }
    fn auto_start_mut(&mut self) -> &mut bool {
        &mut self.auto_start
    }
    fn override_height(&self) -> &f32 {
        &self.override_height
    }
    fn override_height_mut(&mut self) -> &mut f32 {
        &mut self.override_height
    }
    fn local_player_only(&self) -> &bool {
        &self.local_player_only
    }
    fn local_player_only_mut(&mut self) -> &mut bool {
        &mut self.local_player_only
    }
    fn material(&self) -> &super::entity::MaterialDecl {
        &self.material
    }
    fn material_mut(&mut self) -> &mut super::entity::MaterialDecl {
        &mut self.material
    }
    fn transform_effect_with_component(&self) -> &bool {
        &self.transform_effect_with_component
    }
    fn transform_effect_with_component_mut(&mut self) -> &mut bool {
        &mut self.transform_effect_with_component
    }
    fn snapping(&self) -> &SnapType {
        &self.snapping
    }
    fn snapping_mut(&mut self) -> &mut SnapType {
        &mut self.snapping
    }
    fn use_ray_cast(&self) -> &bool {
        &self.use_ray_cast
    }
    fn use_ray_cast_mut(&mut self) -> &mut bool {
        &mut self.use_ray_cast
    }
    fn spawn_effect_on_lookup_location(&self) -> &bool {
        &self.spawn_effect_on_lookup_location
    }
    fn spawn_effect_on_lookup_location_mut(&mut self) -> &mut bool {
        &mut self.spawn_effect_on_lookup_location
    }
    fn ray_direction(&self) -> &super::core::Vec3 {
        &self.ray_direction
    }
    fn ray_direction_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.ray_direction
    }
    fn ray_distance(&self) -> &f32 {
        &self.ray_distance
    }
    fn ray_distance_mut(&mut self) -> &mut f32 {
        &mut self.ray_distance
    }
    fn see_through(&self) -> &bool {
        &self.see_through
    }
    fn see_through_mut(&mut self) -> &mut bool {
        &mut self.see_through
    }
    fn penetrable(&self) -> &bool {
        &self.penetrable
    }
    fn penetrable_mut(&mut self) -> &mut bool {
        &mut self.penetrable
    }
    fn include_terrain(&self) -> &bool {
        &self.include_terrain
    }
    fn include_terrain_mut(&mut self) -> &mut bool {
        &mut self.include_terrain
    }
    fn max_instances(&self) -> &i32 {
        &self.max_instances
    }
    fn max_instances_mut(&mut self) -> &mut i32 {
        &mut self.max_instances
    }
    fn effect_parameters(&self) -> &Vec<Option<LockedTypeObject /* super::effect_base::EffectParameter */>> {
        &self.effect_parameters
    }
    fn effect_parameters_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::effect_base::EffectParameter */>> {
        &mut self.effect_parameters
    }
}

impl super::entity::GameComponentDataTrait for MaterialBasedEffectComponentData {
}

impl super::entity::ComponentDataTrait for MaterialBasedEffectComponentData {
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

impl super::entity::GameObjectDataTrait for MaterialBasedEffectComponentData {
}

impl super::core::DataBusPeerTrait for MaterialBasedEffectComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for MaterialBasedEffectComponentData {
}

impl super::core::DataContainerTrait for MaterialBasedEffectComponentData {
}

pub static MATERIALBASEDEFFECTCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialBasedEffectComponentData",
    name_hash: 2892251863,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::GAMECOMPONENTDATA_TYPE_INFO),
        super_class_offset: offset_of!(MaterialBasedEffectComponentData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MaterialBasedEffectComponentData as Default>::default())),
            create_boxed: || Box::new(<MaterialBasedEffectComponentData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "AutoStart",
                name_hash: 792615882,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MaterialBasedEffectComponentData, auto_start),
            },
            FieldInfoData {
                name: "OverrideHeight",
                name_hash: 3774759278,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MaterialBasedEffectComponentData, override_height),
            },
            FieldInfoData {
                name: "LocalPlayerOnly",
                name_hash: 4035338447,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MaterialBasedEffectComponentData, local_player_only),
            },
            FieldInfoData {
                name: "Material",
                name_hash: 845639918,
                flags: MemberInfoFlags::new(0),
                field_type: "MaterialDecl",
                rust_offset: offset_of!(MaterialBasedEffectComponentData, material),
            },
            FieldInfoData {
                name: "TransformEffectWithComponent",
                name_hash: 4239562419,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MaterialBasedEffectComponentData, transform_effect_with_component),
            },
            FieldInfoData {
                name: "Snapping",
                name_hash: 705021177,
                flags: MemberInfoFlags::new(0),
                field_type: "SnapType",
                rust_offset: offset_of!(MaterialBasedEffectComponentData, snapping),
            },
            FieldInfoData {
                name: "UseRayCast",
                name_hash: 3612013193,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MaterialBasedEffectComponentData, use_ray_cast),
            },
            FieldInfoData {
                name: "SpawnEffectOnLookupLocation",
                name_hash: 635963575,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MaterialBasedEffectComponentData, spawn_effect_on_lookup_location),
            },
            FieldInfoData {
                name: "RayDirection",
                name_hash: 574551946,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(MaterialBasedEffectComponentData, ray_direction),
            },
            FieldInfoData {
                name: "RayDistance",
                name_hash: 223111116,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MaterialBasedEffectComponentData, ray_distance),
            },
            FieldInfoData {
                name: "SeeThrough",
                name_hash: 753481485,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MaterialBasedEffectComponentData, see_through),
            },
            FieldInfoData {
                name: "Penetrable",
                name_hash: 1887765847,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MaterialBasedEffectComponentData, penetrable),
            },
            FieldInfoData {
                name: "IncludeTerrain",
                name_hash: 106414606,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MaterialBasedEffectComponentData, include_terrain),
            },
            FieldInfoData {
                name: "MaxInstances",
                name_hash: 3494173515,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(MaterialBasedEffectComponentData, max_instances),
            },
            FieldInfoData {
                name: "EffectParameters",
                name_hash: 929782248,
                flags: MemberInfoFlags::new(144),
                field_type: "EffectParameter-Array",
                rust_offset: offset_of!(MaterialBasedEffectComponentData, effect_parameters),
            },
        ],
    }),
    array_type: Some(MATERIALBASEDEFFECTCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for MaterialBasedEffectComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        MATERIALBASEDEFFECTCOMPONENTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static MATERIALBASEDEFFECTCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialBasedEffectComponentData-Array",
    name_hash: 1222605795,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("MaterialBasedEffectComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum SnapType {
    #[default]
    mbeNoSnap = 0,
    mbeSnapToTerrain = 1,
    mbeSnapToWater = 2,
    mbeSnapToClosest = 3,
}

pub static SNAPTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnapType",
    name_hash: 704065105,
    flags: MemberInfoFlags::new(49429),
    module: "DiceCommonsShared",
    data: TypeInfoData::Enum,
    array_type: Some(SNAPTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SnapType {
    fn type_info(&self) -> &'static TypeInfo {
        SNAPTYPE_TYPE_INFO
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


pub static SNAPTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnapType-Array",
    name_hash: 2749964389,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("SnapType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LocatorComponentData {
    pub _glacier_base: super::entity::GameComponentData,
    pub realm: super::core::Realm,
    pub is_used_as_link_target: bool,
}

pub trait LocatorComponentDataTrait: super::entity::GameComponentDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn is_used_as_link_target(&self) -> &bool;
    fn is_used_as_link_target_mut(&mut self) -> &mut bool;
}

impl LocatorComponentDataTrait for LocatorComponentData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn is_used_as_link_target(&self) -> &bool {
        &self.is_used_as_link_target
    }
    fn is_used_as_link_target_mut(&mut self) -> &mut bool {
        &mut self.is_used_as_link_target
    }
}

impl super::entity::GameComponentDataTrait for LocatorComponentData {
}

impl super::entity::ComponentDataTrait for LocatorComponentData {
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

impl super::entity::GameObjectDataTrait for LocatorComponentData {
}

impl super::core::DataBusPeerTrait for LocatorComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for LocatorComponentData {
}

impl super::core::DataContainerTrait for LocatorComponentData {
}

pub static LOCATORCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocatorComponentData",
    name_hash: 1367760434,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::GAMECOMPONENTDATA_TYPE_INFO),
        super_class_offset: offset_of!(LocatorComponentData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LocatorComponentData as Default>::default())),
            create_boxed: || Box::new(<LocatorComponentData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                name_hash: 229961746,
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(LocatorComponentData, realm),
            },
            FieldInfoData {
                name: "IsUsedAsLinkTarget",
                name_hash: 3118722907,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LocatorComponentData, is_used_as_link_target),
            },
        ],
    }),
    array_type: Some(LOCATORCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LocatorComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        LOCATORCOMPONENTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static LOCATORCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocatorComponentData-Array",
    name_hash: 2707301510,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("LocatorComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ActorPhysicsComponentData {
    pub _glacier_base: super::game_shared::StaticModelPhysicsComponentData,
}

pub trait ActorPhysicsComponentDataTrait: super::game_shared::StaticModelPhysicsComponentDataTrait {
}

impl ActorPhysicsComponentDataTrait for ActorPhysicsComponentData {
}

impl super::game_shared::StaticModelPhysicsComponentDataTrait for ActorPhysicsComponentData {
}

impl super::gameplay_sim::GamePhysicsComponentDataTrait for ActorPhysicsComponentData {
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

impl super::physics::PhysicsComponentDataTrait for ActorPhysicsComponentData {
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

impl super::entity::ComponentDataTrait for ActorPhysicsComponentData {
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

impl super::entity::GameObjectDataTrait for ActorPhysicsComponentData {
}

impl super::core::DataBusPeerTrait for ActorPhysicsComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ActorPhysicsComponentData {
}

impl super::core::DataContainerTrait for ActorPhysicsComponentData {
}

pub static ACTORPHYSICSCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ActorPhysicsComponentData",
    name_hash: 1923830682,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_shared::STATICMODELPHYSICSCOMPONENTDATA_TYPE_INFO),
        super_class_offset: offset_of!(ActorPhysicsComponentData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ActorPhysicsComponentData as Default>::default())),
            create_boxed: || Box::new(<ActorPhysicsComponentData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(ACTORPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ActorPhysicsComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        ACTORPHYSICSCOMPONENTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static ACTORPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ActorPhysicsComponentData-Array",
    name_hash: 907605678,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ActorPhysicsComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ActorCustomizationComponentData {
    pub _glacier_base: super::entity::GameComponentData,
    pub customization: Option<LockedTypeObject /* ActorCustomizationData */>,
}

pub trait ActorCustomizationComponentDataTrait: super::entity::GameComponentDataTrait {
    fn customization(&self) -> &Option<LockedTypeObject /* ActorCustomizationData */>;
    fn customization_mut(&mut self) -> &mut Option<LockedTypeObject /* ActorCustomizationData */>;
}

impl ActorCustomizationComponentDataTrait for ActorCustomizationComponentData {
    fn customization(&self) -> &Option<LockedTypeObject /* ActorCustomizationData */> {
        &self.customization
    }
    fn customization_mut(&mut self) -> &mut Option<LockedTypeObject /* ActorCustomizationData */> {
        &mut self.customization
    }
}

impl super::entity::GameComponentDataTrait for ActorCustomizationComponentData {
}

impl super::entity::ComponentDataTrait for ActorCustomizationComponentData {
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

impl super::entity::GameObjectDataTrait for ActorCustomizationComponentData {
}

impl super::core::DataBusPeerTrait for ActorCustomizationComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ActorCustomizationComponentData {
}

impl super::core::DataContainerTrait for ActorCustomizationComponentData {
}

pub static ACTORCUSTOMIZATIONCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ActorCustomizationComponentData",
    name_hash: 2545198508,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::GAMECOMPONENTDATA_TYPE_INFO),
        super_class_offset: offset_of!(ActorCustomizationComponentData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ActorCustomizationComponentData as Default>::default())),
            create_boxed: || Box::new(<ActorCustomizationComponentData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Customization",
                name_hash: 1998291608,
                flags: MemberInfoFlags::new(0),
                field_type: "ActorCustomizationData",
                rust_offset: offset_of!(ActorCustomizationComponentData, customization),
            },
        ],
    }),
    array_type: Some(ACTORCUSTOMIZATIONCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ActorCustomizationComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        ACTORCUSTOMIZATIONCOMPONENTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static ACTORCUSTOMIZATIONCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ActorCustomizationComponentData-Array",
    name_hash: 804771096,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ActorCustomizationComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ActorCustomizationData {
    pub _glacier_base: super::core::DataContainerPolicyAsset,
    pub visual_groups: Vec<BoxedTypeObject /* super::game_shared::CustomizeVisual */>,
    pub ant_game_states: Vec<Option<LockedTypeObject /* super::game_shared::WriteAntGameStateData */>>,
}

pub trait ActorCustomizationDataTrait: super::core::DataContainerPolicyAssetTrait {
    fn visual_groups(&self) -> &Vec<BoxedTypeObject /* super::game_shared::CustomizeVisual */>;
    fn visual_groups_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::game_shared::CustomizeVisual */>;
    fn ant_game_states(&self) -> &Vec<Option<LockedTypeObject /* super::game_shared::WriteAntGameStateData */>>;
    fn ant_game_states_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::game_shared::WriteAntGameStateData */>>;
}

impl ActorCustomizationDataTrait for ActorCustomizationData {
    fn visual_groups(&self) -> &Vec<BoxedTypeObject /* super::game_shared::CustomizeVisual */> {
        &self.visual_groups
    }
    fn visual_groups_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::game_shared::CustomizeVisual */> {
        &mut self.visual_groups
    }
    fn ant_game_states(&self) -> &Vec<Option<LockedTypeObject /* super::game_shared::WriteAntGameStateData */>> {
        &self.ant_game_states
    }
    fn ant_game_states_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::game_shared::WriteAntGameStateData */>> {
        &mut self.ant_game_states
    }
}

impl super::core::DataContainerPolicyAssetTrait for ActorCustomizationData {
}

impl super::core::AssetTrait for ActorCustomizationData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for ActorCustomizationData {
}

pub static ACTORCUSTOMIZATIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ActorCustomizationData",
    name_hash: 4276156579,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINERPOLICYASSET_TYPE_INFO),
        super_class_offset: offset_of!(ActorCustomizationData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ActorCustomizationData as Default>::default())),
            create_boxed: || Box::new(<ActorCustomizationData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "VisualGroups",
                name_hash: 1154342877,
                flags: MemberInfoFlags::new(144),
                field_type: "CustomizeVisual-Array",
                rust_offset: offset_of!(ActorCustomizationData, visual_groups),
            },
            FieldInfoData {
                name: "AntGameStates",
                name_hash: 1141188948,
                flags: MemberInfoFlags::new(144),
                field_type: "WriteAntGameStateData-Array",
                rust_offset: offset_of!(ActorCustomizationData, ant_game_states),
            },
        ],
    }),
    array_type: Some(ACTORCUSTOMIZATIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ActorCustomizationData {
    fn type_info(&self) -> &'static TypeInfo {
        ACTORCUSTOMIZATIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static ACTORCUSTOMIZATIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ActorCustomizationData-Array",
    name_hash: 1965497367,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ActorCustomizationData"),
    array_type: None,
    alignment: 8,
};



pub static DRAWVERLETCAPSULE_MAT4_FLOAT32_FLOAT32_VEC4__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DrawVerletCapsule(Mat4,Float32,Float32,Vec4)",
    name_hash: 122907841,
    flags: MemberInfoFlags::new(793),
    module: "DiceCommonsShared",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CoolDownGateEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub cool_down_time: f32,
    pub start_opened: bool,
}

pub trait CoolDownGateEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn cool_down_time(&self) -> &f32;
    fn cool_down_time_mut(&mut self) -> &mut f32;
    fn start_opened(&self) -> &bool;
    fn start_opened_mut(&mut self) -> &mut bool;
}

impl CoolDownGateEntityDataTrait for CoolDownGateEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn cool_down_time(&self) -> &f32 {
        &self.cool_down_time
    }
    fn cool_down_time_mut(&mut self) -> &mut f32 {
        &mut self.cool_down_time
    }
    fn start_opened(&self) -> &bool {
        &self.start_opened
    }
    fn start_opened_mut(&mut self) -> &mut bool {
        &mut self.start_opened
    }
}

impl super::entity::EntityDataTrait for CoolDownGateEntityData {
}

impl super::entity::GameObjectDataTrait for CoolDownGateEntityData {
}

impl super::core::DataBusPeerTrait for CoolDownGateEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for CoolDownGateEntityData {
}

impl super::core::DataContainerTrait for CoolDownGateEntityData {
}

pub static COOLDOWNGATEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoolDownGateEntityData",
    name_hash: 4263471876,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(CoolDownGateEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoolDownGateEntityData as Default>::default())),
            create_boxed: || Box::new(<CoolDownGateEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                name_hash: 229961746,
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(CoolDownGateEntityData, realm),
            },
            FieldInfoData {
                name: "CoolDownTime",
                name_hash: 282296301,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CoolDownGateEntityData, cool_down_time),
            },
            FieldInfoData {
                name: "StartOpened",
                name_hash: 248233008,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CoolDownGateEntityData, start_opened),
            },
        ],
    }),
    array_type: Some(COOLDOWNGATEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CoolDownGateEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        COOLDOWNGATEENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static COOLDOWNGATEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoolDownGateEntityData-Array",
    name_hash: 4137811760,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("CoolDownGateEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ConditionalPropertyEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub condition: bool,
    pub type_hash: i32,
    pub value_if_false_property_hash: i32,
    pub value_if_true_property_hash: i32,
    pub out_hash: i32,
}

pub trait ConditionalPropertyEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn condition(&self) -> &bool;
    fn condition_mut(&mut self) -> &mut bool;
    fn type_hash(&self) -> &i32;
    fn type_hash_mut(&mut self) -> &mut i32;
    fn value_if_false_property_hash(&self) -> &i32;
    fn value_if_false_property_hash_mut(&mut self) -> &mut i32;
    fn value_if_true_property_hash(&self) -> &i32;
    fn value_if_true_property_hash_mut(&mut self) -> &mut i32;
    fn out_hash(&self) -> &i32;
    fn out_hash_mut(&mut self) -> &mut i32;
}

impl ConditionalPropertyEntityDataTrait for ConditionalPropertyEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn condition(&self) -> &bool {
        &self.condition
    }
    fn condition_mut(&mut self) -> &mut bool {
        &mut self.condition
    }
    fn type_hash(&self) -> &i32 {
        &self.type_hash
    }
    fn type_hash_mut(&mut self) -> &mut i32 {
        &mut self.type_hash
    }
    fn value_if_false_property_hash(&self) -> &i32 {
        &self.value_if_false_property_hash
    }
    fn value_if_false_property_hash_mut(&mut self) -> &mut i32 {
        &mut self.value_if_false_property_hash
    }
    fn value_if_true_property_hash(&self) -> &i32 {
        &self.value_if_true_property_hash
    }
    fn value_if_true_property_hash_mut(&mut self) -> &mut i32 {
        &mut self.value_if_true_property_hash
    }
    fn out_hash(&self) -> &i32 {
        &self.out_hash
    }
    fn out_hash_mut(&mut self) -> &mut i32 {
        &mut self.out_hash
    }
}

impl super::entity::EntityDataTrait for ConditionalPropertyEntityData {
}

impl super::entity::GameObjectDataTrait for ConditionalPropertyEntityData {
}

impl super::core::DataBusPeerTrait for ConditionalPropertyEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ConditionalPropertyEntityData {
}

impl super::core::DataContainerTrait for ConditionalPropertyEntityData {
}

pub static CONDITIONALPROPERTYENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalPropertyEntityData",
    name_hash: 3871015639,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(ConditionalPropertyEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ConditionalPropertyEntityData as Default>::default())),
            create_boxed: || Box::new(<ConditionalPropertyEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                name_hash: 229961746,
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(ConditionalPropertyEntityData, realm),
            },
            FieldInfoData {
                name: "Condition",
                name_hash: 1800624758,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ConditionalPropertyEntityData, condition),
            },
            FieldInfoData {
                name: "TypeHash",
                name_hash: 351092271,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ConditionalPropertyEntityData, type_hash),
            },
            FieldInfoData {
                name: "ValueIfFalsePropertyHash",
                name_hash: 2636843625,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ConditionalPropertyEntityData, value_if_false_property_hash),
            },
            FieldInfoData {
                name: "ValueIfTruePropertyHash",
                name_hash: 1305170882,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ConditionalPropertyEntityData, value_if_true_property_hash),
            },
            FieldInfoData {
                name: "OutHash",
                name_hash: 1070246745,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ConditionalPropertyEntityData, out_hash),
            },
        ],
    }),
    array_type: Some(CONDITIONALPROPERTYENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ConditionalPropertyEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CONDITIONALPROPERTYENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CONDITIONALPROPERTYENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalPropertyEntityData-Array",
    name_hash: 424033251,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ConditionalPropertyEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientEmitTraceBookmarkEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub bookmark_name: String,
    pub bookmark_description: String,
}

pub trait ClientEmitTraceBookmarkEntityDataTrait: super::entity::EntityDataTrait {
    fn bookmark_name(&self) -> &String;
    fn bookmark_name_mut(&mut self) -> &mut String;
    fn bookmark_description(&self) -> &String;
    fn bookmark_description_mut(&mut self) -> &mut String;
}

impl ClientEmitTraceBookmarkEntityDataTrait for ClientEmitTraceBookmarkEntityData {
    fn bookmark_name(&self) -> &String {
        &self.bookmark_name
    }
    fn bookmark_name_mut(&mut self) -> &mut String {
        &mut self.bookmark_name
    }
    fn bookmark_description(&self) -> &String {
        &self.bookmark_description
    }
    fn bookmark_description_mut(&mut self) -> &mut String {
        &mut self.bookmark_description
    }
}

impl super::entity::EntityDataTrait for ClientEmitTraceBookmarkEntityData {
}

impl super::entity::GameObjectDataTrait for ClientEmitTraceBookmarkEntityData {
}

impl super::core::DataBusPeerTrait for ClientEmitTraceBookmarkEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ClientEmitTraceBookmarkEntityData {
}

impl super::core::DataContainerTrait for ClientEmitTraceBookmarkEntityData {
}

pub static CLIENTEMITTRACEBOOKMARKENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEmitTraceBookmarkEntityData",
    name_hash: 3728621791,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(ClientEmitTraceBookmarkEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientEmitTraceBookmarkEntityData as Default>::default())),
            create_boxed: || Box::new(<ClientEmitTraceBookmarkEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "BookmarkName",
                name_hash: 86413758,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ClientEmitTraceBookmarkEntityData, bookmark_name),
            },
            FieldInfoData {
                name: "BookmarkDescription",
                name_hash: 3331945567,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ClientEmitTraceBookmarkEntityData, bookmark_description),
            },
        ],
    }),
    array_type: Some(CLIENTEMITTRACEBOOKMARKENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ClientEmitTraceBookmarkEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTEMITTRACEBOOKMARKENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTEMITTRACEBOOKMARKENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEmitTraceBookmarkEntityData-Array",
    name_hash: 1145001451,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ClientEmitTraceBookmarkEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CameraShakeEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub local_player: super::core::LocalPlayerId,
    pub transform: super::core::LinearTransform,
    pub enabled: bool,
    pub trigger_shake_profile: Option<LockedTypeObject /* super::core::FloatCurve */>,
    pub pitch: CameraShakeAxisData,
    pub yaw: CameraShakeAxisData,
    pub roll: CameraShakeAxisData,
    pub fade_out_start_distance: f32,
    pub fade_out_end_distance: f32,
    pub trigger_shake_time: f32,
    pub amplitude: f32,
    pub intensity: f32,
}

pub trait CameraShakeEntityDataTrait: super::entity::EntityDataTrait {
    fn local_player(&self) -> &super::core::LocalPlayerId;
    fn local_player_mut(&mut self) -> &mut super::core::LocalPlayerId;
    fn transform(&self) -> &super::core::LinearTransform;
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform;
    fn enabled(&self) -> &bool;
    fn enabled_mut(&mut self) -> &mut bool;
    fn trigger_shake_profile(&self) -> &Option<LockedTypeObject /* super::core::FloatCurve */>;
    fn trigger_shake_profile_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::FloatCurve */>;
    fn pitch(&self) -> &CameraShakeAxisData;
    fn pitch_mut(&mut self) -> &mut CameraShakeAxisData;
    fn yaw(&self) -> &CameraShakeAxisData;
    fn yaw_mut(&mut self) -> &mut CameraShakeAxisData;
    fn roll(&self) -> &CameraShakeAxisData;
    fn roll_mut(&mut self) -> &mut CameraShakeAxisData;
    fn fade_out_start_distance(&self) -> &f32;
    fn fade_out_start_distance_mut(&mut self) -> &mut f32;
    fn fade_out_end_distance(&self) -> &f32;
    fn fade_out_end_distance_mut(&mut self) -> &mut f32;
    fn trigger_shake_time(&self) -> &f32;
    fn trigger_shake_time_mut(&mut self) -> &mut f32;
    fn amplitude(&self) -> &f32;
    fn amplitude_mut(&mut self) -> &mut f32;
    fn intensity(&self) -> &f32;
    fn intensity_mut(&mut self) -> &mut f32;
}

impl CameraShakeEntityDataTrait for CameraShakeEntityData {
    fn local_player(&self) -> &super::core::LocalPlayerId {
        &self.local_player
    }
    fn local_player_mut(&mut self) -> &mut super::core::LocalPlayerId {
        &mut self.local_player
    }
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.transform
    }
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
    fn trigger_shake_profile(&self) -> &Option<LockedTypeObject /* super::core::FloatCurve */> {
        &self.trigger_shake_profile
    }
    fn trigger_shake_profile_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::FloatCurve */> {
        &mut self.trigger_shake_profile
    }
    fn pitch(&self) -> &CameraShakeAxisData {
        &self.pitch
    }
    fn pitch_mut(&mut self) -> &mut CameraShakeAxisData {
        &mut self.pitch
    }
    fn yaw(&self) -> &CameraShakeAxisData {
        &self.yaw
    }
    fn yaw_mut(&mut self) -> &mut CameraShakeAxisData {
        &mut self.yaw
    }
    fn roll(&self) -> &CameraShakeAxisData {
        &self.roll
    }
    fn roll_mut(&mut self) -> &mut CameraShakeAxisData {
        &mut self.roll
    }
    fn fade_out_start_distance(&self) -> &f32 {
        &self.fade_out_start_distance
    }
    fn fade_out_start_distance_mut(&mut self) -> &mut f32 {
        &mut self.fade_out_start_distance
    }
    fn fade_out_end_distance(&self) -> &f32 {
        &self.fade_out_end_distance
    }
    fn fade_out_end_distance_mut(&mut self) -> &mut f32 {
        &mut self.fade_out_end_distance
    }
    fn trigger_shake_time(&self) -> &f32 {
        &self.trigger_shake_time
    }
    fn trigger_shake_time_mut(&mut self) -> &mut f32 {
        &mut self.trigger_shake_time
    }
    fn amplitude(&self) -> &f32 {
        &self.amplitude
    }
    fn amplitude_mut(&mut self) -> &mut f32 {
        &mut self.amplitude
    }
    fn intensity(&self) -> &f32 {
        &self.intensity
    }
    fn intensity_mut(&mut self) -> &mut f32 {
        &mut self.intensity
    }
}

impl super::entity::EntityDataTrait for CameraShakeEntityData {
}

impl super::entity::GameObjectDataTrait for CameraShakeEntityData {
}

impl super::core::DataBusPeerTrait for CameraShakeEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for CameraShakeEntityData {
}

impl super::core::DataContainerTrait for CameraShakeEntityData {
}

pub static CAMERASHAKEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraShakeEntityData",
    name_hash: 2171412963,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(CameraShakeEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CameraShakeEntityData as Default>::default())),
            create_boxed: || Box::new(<CameraShakeEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "LocalPlayer",
                name_hash: 162647195,
                flags: MemberInfoFlags::new(0),
                field_type: "LocalPlayerId",
                rust_offset: offset_of!(CameraShakeEntityData, local_player),
            },
            FieldInfoData {
                name: "Transform",
                name_hash: 2270319721,
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(CameraShakeEntityData, transform),
            },
            FieldInfoData {
                name: "Enabled",
                name_hash: 2662400,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CameraShakeEntityData, enabled),
            },
            FieldInfoData {
                name: "TriggerShakeProfile",
                name_hash: 2086463298,
                flags: MemberInfoFlags::new(0),
                field_type: "FloatCurve",
                rust_offset: offset_of!(CameraShakeEntityData, trigger_shake_profile),
            },
            FieldInfoData {
                name: "Pitch",
                name_hash: 232604323,
                flags: MemberInfoFlags::new(0),
                field_type: "CameraShakeAxisData",
                rust_offset: offset_of!(CameraShakeEntityData, pitch),
            },
            FieldInfoData {
                name: "Yaw",
                name_hash: 193468618,
                flags: MemberInfoFlags::new(0),
                field_type: "CameraShakeAxisData",
                rust_offset: offset_of!(CameraShakeEntityData, yaw),
            },
            FieldInfoData {
                name: "Roll",
                name_hash: 2089387576,
                flags: MemberInfoFlags::new(0),
                field_type: "CameraShakeAxisData",
                rust_offset: offset_of!(CameraShakeEntityData, roll),
            },
            FieldInfoData {
                name: "FadeOutStartDistance",
                name_hash: 86428302,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CameraShakeEntityData, fade_out_start_distance),
            },
            FieldInfoData {
                name: "FadeOutEndDistance",
                name_hash: 1804887425,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CameraShakeEntityData, fade_out_end_distance),
            },
            FieldInfoData {
                name: "TriggerShakeTime",
                name_hash: 2969295228,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CameraShakeEntityData, trigger_shake_time),
            },
            FieldInfoData {
                name: "Amplitude",
                name_hash: 698564572,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CameraShakeEntityData, amplitude),
            },
            FieldInfoData {
                name: "Intensity",
                name_hash: 3836394730,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CameraShakeEntityData, intensity),
            },
        ],
    }),
    array_type: Some(CAMERASHAKEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CameraShakeEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CAMERASHAKEENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CAMERASHAKEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraShakeEntityData-Array",
    name_hash: 197211607,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("CameraShakeEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CameraShakeAxisData {
    pub intensity: f32,
    pub sin_range: f32,
    pub sin_frequency: f32,
    pub noise_range: f32,
    pub noise_frequency: f32,
}

pub trait CameraShakeAxisDataTrait: TypeObject {
    fn intensity(&self) -> &f32;
    fn intensity_mut(&mut self) -> &mut f32;
    fn sin_range(&self) -> &f32;
    fn sin_range_mut(&mut self) -> &mut f32;
    fn sin_frequency(&self) -> &f32;
    fn sin_frequency_mut(&mut self) -> &mut f32;
    fn noise_range(&self) -> &f32;
    fn noise_range_mut(&mut self) -> &mut f32;
    fn noise_frequency(&self) -> &f32;
    fn noise_frequency_mut(&mut self) -> &mut f32;
}

impl CameraShakeAxisDataTrait for CameraShakeAxisData {
    fn intensity(&self) -> &f32 {
        &self.intensity
    }
    fn intensity_mut(&mut self) -> &mut f32 {
        &mut self.intensity
    }
    fn sin_range(&self) -> &f32 {
        &self.sin_range
    }
    fn sin_range_mut(&mut self) -> &mut f32 {
        &mut self.sin_range
    }
    fn sin_frequency(&self) -> &f32 {
        &self.sin_frequency
    }
    fn sin_frequency_mut(&mut self) -> &mut f32 {
        &mut self.sin_frequency
    }
    fn noise_range(&self) -> &f32 {
        &self.noise_range
    }
    fn noise_range_mut(&mut self) -> &mut f32 {
        &mut self.noise_range
    }
    fn noise_frequency(&self) -> &f32 {
        &self.noise_frequency
    }
    fn noise_frequency_mut(&mut self) -> &mut f32 {
        &mut self.noise_frequency
    }
}

pub static CAMERASHAKEAXISDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraShakeAxisData",
    name_hash: 3234524123,
    flags: MemberInfoFlags::new(36937),
    module: "DiceCommonsShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CameraShakeAxisData as Default>::default())),
            create_boxed: || Box::new(<CameraShakeAxisData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Intensity",
                name_hash: 3836394730,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CameraShakeAxisData, intensity),
            },
            FieldInfoData {
                name: "SinRange",
                name_hash: 3041495790,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CameraShakeAxisData, sin_range),
            },
            FieldInfoData {
                name: "SinFrequency",
                name_hash: 3087547541,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CameraShakeAxisData, sin_frequency),
            },
            FieldInfoData {
                name: "NoiseRange",
                name_hash: 1482091268,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CameraShakeAxisData, noise_range),
            },
            FieldInfoData {
                name: "NoiseFrequency",
                name_hash: 1382679103,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CameraShakeAxisData, noise_frequency),
            },
        ],
    }),
    array_type: Some(CAMERASHAKEAXISDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for CameraShakeAxisData {
    fn type_info(&self) -> &'static TypeInfo {
        CAMERASHAKEAXISDATA_TYPE_INFO
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


pub static CAMERASHAKEAXISDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraShakeAxisData-Array",
    name_hash: 3844117231,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("CameraShakeAxisData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct BlueprintSpawnReferenceObjectData {
    pub _glacier_base: super::entity::ReferenceObjectData,
    pub realm: super::core::Realm,
    pub initial_auto_spawn: bool,
    pub auto_spawn: bool,
    pub queue_spawn_event: bool,
    pub use_as_spawn_point: bool,
    pub spawns_occupy_locations: bool,
    pub initial_spawn_delay: f32,
    pub spawn_delay: f32,
    pub max_count: i32,
    pub max_count_simultaneously: i32,
}

pub trait BlueprintSpawnReferenceObjectDataTrait: super::entity::ReferenceObjectDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn initial_auto_spawn(&self) -> &bool;
    fn initial_auto_spawn_mut(&mut self) -> &mut bool;
    fn auto_spawn(&self) -> &bool;
    fn auto_spawn_mut(&mut self) -> &mut bool;
    fn queue_spawn_event(&self) -> &bool;
    fn queue_spawn_event_mut(&mut self) -> &mut bool;
    fn use_as_spawn_point(&self) -> &bool;
    fn use_as_spawn_point_mut(&mut self) -> &mut bool;
    fn spawns_occupy_locations(&self) -> &bool;
    fn spawns_occupy_locations_mut(&mut self) -> &mut bool;
    fn initial_spawn_delay(&self) -> &f32;
    fn initial_spawn_delay_mut(&mut self) -> &mut f32;
    fn spawn_delay(&self) -> &f32;
    fn spawn_delay_mut(&mut self) -> &mut f32;
    fn max_count(&self) -> &i32;
    fn max_count_mut(&mut self) -> &mut i32;
    fn max_count_simultaneously(&self) -> &i32;
    fn max_count_simultaneously_mut(&mut self) -> &mut i32;
}

impl BlueprintSpawnReferenceObjectDataTrait for BlueprintSpawnReferenceObjectData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
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
    fn spawns_occupy_locations(&self) -> &bool {
        &self.spawns_occupy_locations
    }
    fn spawns_occupy_locations_mut(&mut self) -> &mut bool {
        &mut self.spawns_occupy_locations
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
}

impl super::entity::ReferenceObjectDataTrait for BlueprintSpawnReferenceObjectData {
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

impl super::entity::GameObjectDataTrait for BlueprintSpawnReferenceObjectData {
}

impl super::core::DataBusPeerTrait for BlueprintSpawnReferenceObjectData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for BlueprintSpawnReferenceObjectData {
}

impl super::core::DataContainerTrait for BlueprintSpawnReferenceObjectData {
}

pub static BLUEPRINTSPAWNREFERENCEOBJECTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintSpawnReferenceObjectData",
    name_hash: 2687487327,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::REFERENCEOBJECTDATA_TYPE_INFO),
        super_class_offset: offset_of!(BlueprintSpawnReferenceObjectData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BlueprintSpawnReferenceObjectData as Default>::default())),
            create_boxed: || Box::new(<BlueprintSpawnReferenceObjectData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                name_hash: 229961746,
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(BlueprintSpawnReferenceObjectData, realm),
            },
            FieldInfoData {
                name: "InitialAutoSpawn",
                name_hash: 3746373231,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(BlueprintSpawnReferenceObjectData, initial_auto_spawn),
            },
            FieldInfoData {
                name: "AutoSpawn",
                name_hash: 792472241,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(BlueprintSpawnReferenceObjectData, auto_spawn),
            },
            FieldInfoData {
                name: "QueueSpawnEvent",
                name_hash: 938847395,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(BlueprintSpawnReferenceObjectData, queue_spawn_event),
            },
            FieldInfoData {
                name: "UseAsSpawnPoint",
                name_hash: 3790673059,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(BlueprintSpawnReferenceObjectData, use_as_spawn_point),
            },
            FieldInfoData {
                name: "SpawnsOccupyLocations",
                name_hash: 2733514544,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(BlueprintSpawnReferenceObjectData, spawns_occupy_locations),
            },
            FieldInfoData {
                name: "InitialSpawnDelay",
                name_hash: 2240280789,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BlueprintSpawnReferenceObjectData, initial_spawn_delay),
            },
            FieldInfoData {
                name: "SpawnDelay",
                name_hash: 3473198411,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BlueprintSpawnReferenceObjectData, spawn_delay),
            },
            FieldInfoData {
                name: "MaxCount",
                name_hash: 415061138,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(BlueprintSpawnReferenceObjectData, max_count),
            },
            FieldInfoData {
                name: "MaxCountSimultaneously",
                name_hash: 2090824542,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(BlueprintSpawnReferenceObjectData, max_count_simultaneously),
            },
        ],
    }),
    array_type: Some(BLUEPRINTSPAWNREFERENCEOBJECTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for BlueprintSpawnReferenceObjectData {
    fn type_info(&self) -> &'static TypeInfo {
        BLUEPRINTSPAWNREFERENCEOBJECTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static BLUEPRINTSPAWNREFERENCEOBJECTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintSpawnReferenceObjectData-Array",
    name_hash: 1296581227,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("BlueprintSpawnReferenceObjectData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CharacterProxyData {
    pub _glacier_base: BlueprintProxyData,
    pub template: Option<LockedTypeObject /* super::game_shared::CharacterSpawnTemplateData */>,
    pub use_local_player_character: bool,
}

pub trait CharacterProxyDataTrait: BlueprintProxyDataTrait {
    fn template(&self) -> &Option<LockedTypeObject /* super::game_shared::CharacterSpawnTemplateData */>;
    fn template_mut(&mut self) -> &mut Option<LockedTypeObject /* super::game_shared::CharacterSpawnTemplateData */>;
    fn use_local_player_character(&self) -> &bool;
    fn use_local_player_character_mut(&mut self) -> &mut bool;
}

impl CharacterProxyDataTrait for CharacterProxyData {
    fn template(&self) -> &Option<LockedTypeObject /* super::game_shared::CharacterSpawnTemplateData */> {
        &self.template
    }
    fn template_mut(&mut self) -> &mut Option<LockedTypeObject /* super::game_shared::CharacterSpawnTemplateData */> {
        &mut self.template
    }
    fn use_local_player_character(&self) -> &bool {
        &self.use_local_player_character
    }
    fn use_local_player_character_mut(&mut self) -> &mut bool {
        &mut self.use_local_player_character
    }
}

impl BlueprintProxyDataTrait for CharacterProxyData {
    fn preview_in_game_view(&self) -> &bool {
        self._glacier_base.preview_in_game_view()
    }
    fn preview_in_game_view_mut(&mut self) -> &mut bool {
        self._glacier_base.preview_in_game_view_mut()
    }
    fn preview_spawn_position(&self) -> &super::core::LinearTransform {
        self._glacier_base.preview_spawn_position()
    }
    fn preview_spawn_position_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.preview_spawn_position_mut()
    }
    fn connected_properties(&self) -> &Vec<BoxedTypeObject /* ProxyPropertyContainer */> {
        self._glacier_base.connected_properties()
    }
    fn connected_properties_mut(&mut self) -> &mut Vec<BoxedTypeObject /* ProxyPropertyContainer */> {
        self._glacier_base.connected_properties_mut()
    }
}

impl BlueprintProxyPropertyFilterDataTrait for CharacterProxyData {
}

impl super::entity::LogicReferenceObjectDataTrait for CharacterProxyData {
    fn local_player_id(&self) -> &super::core::LocalPlayerId {
        self._glacier_base.local_player_id()
    }
    fn local_player_id_mut(&mut self) -> &mut super::core::LocalPlayerId {
        self._glacier_base.local_player_id_mut()
    }
    fn sub_realm(&self) -> &super::entity::SubRealm {
        self._glacier_base.sub_realm()
    }
    fn sub_realm_mut(&mut self) -> &mut super::entity::SubRealm {
        self._glacier_base.sub_realm_mut()
    }
}

impl super::entity::ReferenceObjectDataTrait for CharacterProxyData {
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

impl super::entity::GameObjectDataTrait for CharacterProxyData {
}

impl super::core::DataBusPeerTrait for CharacterProxyData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for CharacterProxyData {
}

impl super::core::DataContainerTrait for CharacterProxyData {
}

pub static CHARACTERPROXYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterProxyData",
    name_hash: 3947672896,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BLUEPRINTPROXYDATA_TYPE_INFO),
        super_class_offset: offset_of!(CharacterProxyData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CharacterProxyData as Default>::default())),
            create_boxed: || Box::new(<CharacterProxyData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Template",
                name_hash: 2427043285,
                flags: MemberInfoFlags::new(0),
                field_type: "CharacterSpawnTemplateData",
                rust_offset: offset_of!(CharacterProxyData, template),
            },
            FieldInfoData {
                name: "UseLocalPlayerCharacter",
                name_hash: 2421011425,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CharacterProxyData, use_local_player_character),
            },
        ],
    }),
    array_type: Some(CHARACTERPROXYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CharacterProxyData {
    fn type_info(&self) -> &'static TypeInfo {
        CHARACTERPROXYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CHARACTERPROXYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterProxyData-Array",
    name_hash: 3442625524,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("CharacterProxyData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct BlueprintProxyData {
    pub _glacier_base: BlueprintProxyPropertyFilterData,
    pub preview_in_game_view: bool,
    pub preview_spawn_position: super::core::LinearTransform,
    pub connected_properties: Vec<BoxedTypeObject /* ProxyPropertyContainer */>,
}

pub trait BlueprintProxyDataTrait: BlueprintProxyPropertyFilterDataTrait {
    fn preview_in_game_view(&self) -> &bool;
    fn preview_in_game_view_mut(&mut self) -> &mut bool;
    fn preview_spawn_position(&self) -> &super::core::LinearTransform;
    fn preview_spawn_position_mut(&mut self) -> &mut super::core::LinearTransform;
    fn connected_properties(&self) -> &Vec<BoxedTypeObject /* ProxyPropertyContainer */>;
    fn connected_properties_mut(&mut self) -> &mut Vec<BoxedTypeObject /* ProxyPropertyContainer */>;
}

impl BlueprintProxyDataTrait for BlueprintProxyData {
    fn preview_in_game_view(&self) -> &bool {
        &self.preview_in_game_view
    }
    fn preview_in_game_view_mut(&mut self) -> &mut bool {
        &mut self.preview_in_game_view
    }
    fn preview_spawn_position(&self) -> &super::core::LinearTransform {
        &self.preview_spawn_position
    }
    fn preview_spawn_position_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.preview_spawn_position
    }
    fn connected_properties(&self) -> &Vec<BoxedTypeObject /* ProxyPropertyContainer */> {
        &self.connected_properties
    }
    fn connected_properties_mut(&mut self) -> &mut Vec<BoxedTypeObject /* ProxyPropertyContainer */> {
        &mut self.connected_properties
    }
}

impl BlueprintProxyPropertyFilterDataTrait for BlueprintProxyData {
}

impl super::entity::LogicReferenceObjectDataTrait for BlueprintProxyData {
    fn local_player_id(&self) -> &super::core::LocalPlayerId {
        self._glacier_base.local_player_id()
    }
    fn local_player_id_mut(&mut self) -> &mut super::core::LocalPlayerId {
        self._glacier_base.local_player_id_mut()
    }
    fn sub_realm(&self) -> &super::entity::SubRealm {
        self._glacier_base.sub_realm()
    }
    fn sub_realm_mut(&mut self) -> &mut super::entity::SubRealm {
        self._glacier_base.sub_realm_mut()
    }
}

impl super::entity::ReferenceObjectDataTrait for BlueprintProxyData {
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

impl super::entity::GameObjectDataTrait for BlueprintProxyData {
}

impl super::core::DataBusPeerTrait for BlueprintProxyData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for BlueprintProxyData {
}

impl super::core::DataContainerTrait for BlueprintProxyData {
}

pub static BLUEPRINTPROXYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintProxyData",
    name_hash: 2492724278,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BLUEPRINTPROXYPROPERTYFILTERDATA_TYPE_INFO),
        super_class_offset: offset_of!(BlueprintProxyData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BlueprintProxyData as Default>::default())),
            create_boxed: || Box::new(<BlueprintProxyData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "PreviewInGameView",
                name_hash: 2397157355,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(BlueprintProxyData, preview_in_game_view),
            },
            FieldInfoData {
                name: "PreviewSpawnPosition",
                name_hash: 3504274989,
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(BlueprintProxyData, preview_spawn_position),
            },
            FieldInfoData {
                name: "ConnectedProperties",
                name_hash: 2818888859,
                flags: MemberInfoFlags::new(144),
                field_type: "ProxyPropertyContainer-Array",
                rust_offset: offset_of!(BlueprintProxyData, connected_properties),
            },
        ],
    }),
    array_type: Some(BLUEPRINTPROXYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for BlueprintProxyData {
    fn type_info(&self) -> &'static TypeInfo {
        BLUEPRINTPROXYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static BLUEPRINTPROXYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintProxyData-Array",
    name_hash: 4211337346,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("BlueprintProxyData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ProxyPropertyContainer {
    pub target_realm: super::core::Realm,
    pub target_field_id: i32,
    pub property_type_hash: u32,
}

pub trait ProxyPropertyContainerTrait: TypeObject {
    fn target_realm(&self) -> &super::core::Realm;
    fn target_realm_mut(&mut self) -> &mut super::core::Realm;
    fn target_field_id(&self) -> &i32;
    fn target_field_id_mut(&mut self) -> &mut i32;
    fn property_type_hash(&self) -> &u32;
    fn property_type_hash_mut(&mut self) -> &mut u32;
}

impl ProxyPropertyContainerTrait for ProxyPropertyContainer {
    fn target_realm(&self) -> &super::core::Realm {
        &self.target_realm
    }
    fn target_realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.target_realm
    }
    fn target_field_id(&self) -> &i32 {
        &self.target_field_id
    }
    fn target_field_id_mut(&mut self) -> &mut i32 {
        &mut self.target_field_id
    }
    fn property_type_hash(&self) -> &u32 {
        &self.property_type_hash
    }
    fn property_type_hash_mut(&mut self) -> &mut u32 {
        &mut self.property_type_hash
    }
}

pub static PROXYPROPERTYCONTAINER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProxyPropertyContainer",
    name_hash: 4123386473,
    flags: MemberInfoFlags::new(36937),
    module: "DiceCommonsShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ProxyPropertyContainer as Default>::default())),
            create_boxed: || Box::new(<ProxyPropertyContainer as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "TargetRealm",
                name_hash: 2239377635,
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(ProxyPropertyContainer, target_realm),
            },
            FieldInfoData {
                name: "TargetFieldId",
                name_hash: 2227058747,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ProxyPropertyContainer, target_field_id),
            },
            FieldInfoData {
                name: "PropertyTypeHash",
                name_hash: 3738736008,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ProxyPropertyContainer, property_type_hash),
            },
        ],
    }),
    array_type: Some(PROXYPROPERTYCONTAINER_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for ProxyPropertyContainer {
    fn type_info(&self) -> &'static TypeInfo {
        PROXYPROPERTYCONTAINER_TYPE_INFO
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


pub static PROXYPROPERTYCONTAINER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProxyPropertyContainer-Array",
    name_hash: 899381597,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ProxyPropertyContainer"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct BlueprintProxyPropertyFilterData {
    pub _glacier_base: super::entity::LogicReferenceObjectData,
}

pub trait BlueprintProxyPropertyFilterDataTrait: super::entity::LogicReferenceObjectDataTrait {
}

impl BlueprintProxyPropertyFilterDataTrait for BlueprintProxyPropertyFilterData {
}

impl super::entity::LogicReferenceObjectDataTrait for BlueprintProxyPropertyFilterData {
    fn local_player_id(&self) -> &super::core::LocalPlayerId {
        self._glacier_base.local_player_id()
    }
    fn local_player_id_mut(&mut self) -> &mut super::core::LocalPlayerId {
        self._glacier_base.local_player_id_mut()
    }
    fn sub_realm(&self) -> &super::entity::SubRealm {
        self._glacier_base.sub_realm()
    }
    fn sub_realm_mut(&mut self) -> &mut super::entity::SubRealm {
        self._glacier_base.sub_realm_mut()
    }
}

impl super::entity::ReferenceObjectDataTrait for BlueprintProxyPropertyFilterData {
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

impl super::entity::GameObjectDataTrait for BlueprintProxyPropertyFilterData {
}

impl super::core::DataBusPeerTrait for BlueprintProxyPropertyFilterData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for BlueprintProxyPropertyFilterData {
}

impl super::core::DataContainerTrait for BlueprintProxyPropertyFilterData {
}

pub static BLUEPRINTPROXYPROPERTYFILTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintProxyPropertyFilterData",
    name_hash: 3901477329,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::LOGICREFERENCEOBJECTDATA_TYPE_INFO),
        super_class_offset: offset_of!(BlueprintProxyPropertyFilterData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BlueprintProxyPropertyFilterData as Default>::default())),
            create_boxed: || Box::new(<BlueprintProxyPropertyFilterData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(BLUEPRINTPROXYPROPERTYFILTERDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for BlueprintProxyPropertyFilterData {
    fn type_info(&self) -> &'static TypeInfo {
        BLUEPRINTPROXYPROPERTYFILTERDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static BLUEPRINTPROXYPROPERTYFILTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintProxyPropertyFilterData-Array",
    name_hash: 3544867813,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("BlueprintProxyPropertyFilterData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ActorEntityData {
    pub _glacier_base: super::physics::GamePhysicsEntityData,
    pub realm: super::core::Realm,
    pub render_skeleton_base_pose: super::core::SparseTransformArray,
    pub mesh_type: super::render_base::MeshType,
    pub mesh_part_count: u32,
    pub update_animatable_transform: bool,
    pub update_physics_transform: bool,
    pub server_physics_enabled: bool,
    pub is_findable: bool,
    pub enable_updates: bool,
}

pub trait ActorEntityDataTrait: super::physics::GamePhysicsEntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn render_skeleton_base_pose(&self) -> &super::core::SparseTransformArray;
    fn render_skeleton_base_pose_mut(&mut self) -> &mut super::core::SparseTransformArray;
    fn mesh_type(&self) -> &super::render_base::MeshType;
    fn mesh_type_mut(&mut self) -> &mut super::render_base::MeshType;
    fn mesh_part_count(&self) -> &u32;
    fn mesh_part_count_mut(&mut self) -> &mut u32;
    fn update_animatable_transform(&self) -> &bool;
    fn update_animatable_transform_mut(&mut self) -> &mut bool;
    fn update_physics_transform(&self) -> &bool;
    fn update_physics_transform_mut(&mut self) -> &mut bool;
    fn server_physics_enabled(&self) -> &bool;
    fn server_physics_enabled_mut(&mut self) -> &mut bool;
    fn is_findable(&self) -> &bool;
    fn is_findable_mut(&mut self) -> &mut bool;
    fn enable_updates(&self) -> &bool;
    fn enable_updates_mut(&mut self) -> &mut bool;
}

impl ActorEntityDataTrait for ActorEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn render_skeleton_base_pose(&self) -> &super::core::SparseTransformArray {
        &self.render_skeleton_base_pose
    }
    fn render_skeleton_base_pose_mut(&mut self) -> &mut super::core::SparseTransformArray {
        &mut self.render_skeleton_base_pose
    }
    fn mesh_type(&self) -> &super::render_base::MeshType {
        &self.mesh_type
    }
    fn mesh_type_mut(&mut self) -> &mut super::render_base::MeshType {
        &mut self.mesh_type
    }
    fn mesh_part_count(&self) -> &u32 {
        &self.mesh_part_count
    }
    fn mesh_part_count_mut(&mut self) -> &mut u32 {
        &mut self.mesh_part_count
    }
    fn update_animatable_transform(&self) -> &bool {
        &self.update_animatable_transform
    }
    fn update_animatable_transform_mut(&mut self) -> &mut bool {
        &mut self.update_animatable_transform
    }
    fn update_physics_transform(&self) -> &bool {
        &self.update_physics_transform
    }
    fn update_physics_transform_mut(&mut self) -> &mut bool {
        &mut self.update_physics_transform
    }
    fn server_physics_enabled(&self) -> &bool {
        &self.server_physics_enabled
    }
    fn server_physics_enabled_mut(&mut self) -> &mut bool {
        &mut self.server_physics_enabled
    }
    fn is_findable(&self) -> &bool {
        &self.is_findable
    }
    fn is_findable_mut(&mut self) -> &mut bool {
        &mut self.is_findable
    }
    fn enable_updates(&self) -> &bool {
        &self.enable_updates
    }
    fn enable_updates_mut(&mut self) -> &mut bool {
        &mut self.enable_updates
    }
}

impl super::physics::GamePhysicsEntityDataTrait for ActorEntityData {
}

impl super::entity::GameComponentEntityDataTrait for ActorEntityData {
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::entity::ComponentEntityDataTrait for ActorEntityData {
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

impl super::entity::SpatialEntityDataTrait for ActorEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::entity::EntityDataTrait for ActorEntityData {
}

impl super::entity::GameObjectDataTrait for ActorEntityData {
}

impl super::core::DataBusPeerTrait for ActorEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ActorEntityData {
}

impl super::core::DataContainerTrait for ActorEntityData {
}

pub static ACTORENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ActorEntityData",
    name_hash: 713811397,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::GAMEPHYSICSENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(ActorEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ActorEntityData as Default>::default())),
            create_boxed: || Box::new(<ActorEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                name_hash: 229961746,
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(ActorEntityData, realm),
            },
            FieldInfoData {
                name: "RenderSkeletonBasePose",
                name_hash: 3869533106,
                flags: MemberInfoFlags::new(0),
                field_type: "SparseTransformArray",
                rust_offset: offset_of!(ActorEntityData, render_skeleton_base_pose),
            },
            FieldInfoData {
                name: "MeshType",
                name_hash: 1821946062,
                flags: MemberInfoFlags::new(0),
                field_type: "MeshType",
                rust_offset: offset_of!(ActorEntityData, mesh_type),
            },
            FieldInfoData {
                name: "MeshPartCount",
                name_hash: 3449407426,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ActorEntityData, mesh_part_count),
            },
            FieldInfoData {
                name: "UpdateAnimatableTransform",
                name_hash: 239356140,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ActorEntityData, update_animatable_transform),
            },
            FieldInfoData {
                name: "UpdatePhysicsTransform",
                name_hash: 4210204083,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ActorEntityData, update_physics_transform),
            },
            FieldInfoData {
                name: "ServerPhysicsEnabled",
                name_hash: 1632705006,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ActorEntityData, server_physics_enabled),
            },
            FieldInfoData {
                name: "IsFindable",
                name_hash: 3472545872,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ActorEntityData, is_findable),
            },
            FieldInfoData {
                name: "EnableUpdates",
                name_hash: 771920806,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ActorEntityData, enable_updates),
            },
        ],
    }),
    array_type: Some(ACTORENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ActorEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        ACTORENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static ACTORENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ActorEntityData-Array",
    name_hash: 1941331441,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ActorEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum DiceUIAnalogPadType {
    #[default]
    DiceUIAnalogPadType_LeftStick = 0,
    DiceUIAnalogPadType_RightStick = 1,
    DiceUIAnalogPadType_LeftTrigger = 2,
    DiceUIAnalogPadType_RightTrigger = 3,
    DiceUIAnalogPadType_Count = 4,
}

pub static DICEUIANALOGPADTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIAnalogPadType",
    name_hash: 3651708469,
    flags: MemberInfoFlags::new(49429),
    module: "DiceCommonsShared",
    data: TypeInfoData::Enum,
    array_type: Some(DICEUIANALOGPADTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DiceUIAnalogPadType {
    fn type_info(&self) -> &'static TypeInfo {
        DICEUIANALOGPADTYPE_TYPE_INFO
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


pub static DICEUIANALOGPADTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIAnalogPadType-Array",
    name_hash: 4160022145,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceUIAnalogPadType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DiceUIInputManagerSettings {
    pub _glacier_base: super::core::SystemSettings,
    pub automatic_typing_mode: bool,
    pub treat_touch_as_mouse: bool,
    pub scroll_wheel_dead_zone: f32,
    pub double_click_time: f32,
}

pub trait DiceUIInputManagerSettingsTrait: super::core::SystemSettingsTrait {
    fn automatic_typing_mode(&self) -> &bool;
    fn automatic_typing_mode_mut(&mut self) -> &mut bool;
    fn treat_touch_as_mouse(&self) -> &bool;
    fn treat_touch_as_mouse_mut(&mut self) -> &mut bool;
    fn scroll_wheel_dead_zone(&self) -> &f32;
    fn scroll_wheel_dead_zone_mut(&mut self) -> &mut f32;
    fn double_click_time(&self) -> &f32;
    fn double_click_time_mut(&mut self) -> &mut f32;
}

impl DiceUIInputManagerSettingsTrait for DiceUIInputManagerSettings {
    fn automatic_typing_mode(&self) -> &bool {
        &self.automatic_typing_mode
    }
    fn automatic_typing_mode_mut(&mut self) -> &mut bool {
        &mut self.automatic_typing_mode
    }
    fn treat_touch_as_mouse(&self) -> &bool {
        &self.treat_touch_as_mouse
    }
    fn treat_touch_as_mouse_mut(&mut self) -> &mut bool {
        &mut self.treat_touch_as_mouse
    }
    fn scroll_wheel_dead_zone(&self) -> &f32 {
        &self.scroll_wheel_dead_zone
    }
    fn scroll_wheel_dead_zone_mut(&mut self) -> &mut f32 {
        &mut self.scroll_wheel_dead_zone
    }
    fn double_click_time(&self) -> &f32 {
        &self.double_click_time
    }
    fn double_click_time_mut(&mut self) -> &mut f32 {
        &mut self.double_click_time
    }
}

impl super::core::SystemSettingsTrait for DiceUIInputManagerSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl super::core::DataContainerTrait for DiceUIInputManagerSettings {
}

pub static DICEUIINPUTMANAGERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIInputManagerSettings",
    name_hash: 3485643250,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        super_class_offset: offset_of!(DiceUIInputManagerSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DiceUIInputManagerSettings as Default>::default())),
            create_boxed: || Box::new(<DiceUIInputManagerSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "AutomaticTypingMode",
                name_hash: 3636332102,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DiceUIInputManagerSettings, automatic_typing_mode),
            },
            FieldInfoData {
                name: "TreatTouchAsMouse",
                name_hash: 2532280485,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DiceUIInputManagerSettings, treat_touch_as_mouse),
            },
            FieldInfoData {
                name: "ScrollWheelDeadZone",
                name_hash: 4199146305,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DiceUIInputManagerSettings, scroll_wheel_dead_zone),
            },
            FieldInfoData {
                name: "DoubleClickTime",
                name_hash: 4105019307,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DiceUIInputManagerSettings, double_click_time),
            },
        ],
    }),
    array_type: Some(DICEUIINPUTMANAGERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DiceUIInputManagerSettings {
    fn type_info(&self) -> &'static TypeInfo {
        DICEUIINPUTMANAGERSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DICEUIINPUTMANAGERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIInputManagerSettings-Array",
    name_hash: 2963933382,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceUIInputManagerSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DiceUITypingInputListenerElementData {
    pub _glacier_base: super::game_shared_u_i::UIElementEntityData,
    pub max_text_length: u32,
    pub default_text: String,
    pub title: String,
    pub description: String,
    pub input_text: String,
    pub allow_multiline: bool,
    pub abort_on_escape: bool,
}

pub trait DiceUITypingInputListenerElementDataTrait: super::game_shared_u_i::UIElementEntityDataTrait {
    fn max_text_length(&self) -> &u32;
    fn max_text_length_mut(&mut self) -> &mut u32;
    fn default_text(&self) -> &String;
    fn default_text_mut(&mut self) -> &mut String;
    fn title(&self) -> &String;
    fn title_mut(&mut self) -> &mut String;
    fn description(&self) -> &String;
    fn description_mut(&mut self) -> &mut String;
    fn input_text(&self) -> &String;
    fn input_text_mut(&mut self) -> &mut String;
    fn allow_multiline(&self) -> &bool;
    fn allow_multiline_mut(&mut self) -> &mut bool;
    fn abort_on_escape(&self) -> &bool;
    fn abort_on_escape_mut(&mut self) -> &mut bool;
}

impl DiceUITypingInputListenerElementDataTrait for DiceUITypingInputListenerElementData {
    fn max_text_length(&self) -> &u32 {
        &self.max_text_length
    }
    fn max_text_length_mut(&mut self) -> &mut u32 {
        &mut self.max_text_length
    }
    fn default_text(&self) -> &String {
        &self.default_text
    }
    fn default_text_mut(&mut self) -> &mut String {
        &mut self.default_text
    }
    fn title(&self) -> &String {
        &self.title
    }
    fn title_mut(&mut self) -> &mut String {
        &mut self.title
    }
    fn description(&self) -> &String {
        &self.description
    }
    fn description_mut(&mut self) -> &mut String {
        &mut self.description
    }
    fn input_text(&self) -> &String {
        &self.input_text
    }
    fn input_text_mut(&mut self) -> &mut String {
        &mut self.input_text
    }
    fn allow_multiline(&self) -> &bool {
        &self.allow_multiline
    }
    fn allow_multiline_mut(&mut self) -> &mut bool {
        &mut self.allow_multiline
    }
    fn abort_on_escape(&self) -> &bool {
        &self.abort_on_escape
    }
    fn abort_on_escape_mut(&mut self) -> &mut bool {
        &mut self.abort_on_escape
    }
}

impl super::game_shared_u_i::UIElementEntityDataTrait for DiceUITypingInputListenerElementData {
    fn instance_name(&self) -> &String {
        self._glacier_base.instance_name()
    }
    fn instance_name_mut(&mut self) -> &mut String {
        self._glacier_base.instance_name_mut()
    }
    fn instance_name_hash(&self) -> &u32 {
        self._glacier_base.instance_name_hash()
    }
    fn instance_name_hash_mut(&mut self) -> &mut u32 {
        self._glacier_base.instance_name_hash_mut()
    }
    fn transform_pivot(&self) -> &super::core::Vec3 {
        self._glacier_base.transform_pivot()
    }
    fn transform_pivot_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.transform_pivot_mut()
    }
    fn size(&self) -> &super::core::Vec2 {
        self._glacier_base.size()
    }
    fn size_mut(&mut self) -> &mut super::core::Vec2 {
        self._glacier_base.size_mut()
    }
    fn layout_mode(&self) -> &super::game_shared_u_i::UILayoutMode {
        self._glacier_base.layout_mode()
    }
    fn layout_mode_mut(&mut self) -> &mut super::game_shared_u_i::UILayoutMode {
        self._glacier_base.layout_mode_mut()
    }
    fn offset(&self) -> &super::game_shared_u_i::UIElementOffset {
        self._glacier_base.offset()
    }
    fn offset_mut(&mut self) -> &mut super::game_shared_u_i::UIElementOffset {
        self._glacier_base.offset_mut()
    }
    fn anchor(&self) -> &super::game_shared_u_i::UIElementAnchor {
        self._glacier_base.anchor()
    }
    fn anchor_mut(&mut self) -> &mut super::game_shared_u_i::UIElementAnchor {
        self._glacier_base.anchor_mut()
    }
    fn position(&self) -> &super::game_shared_u_i::UIElementOffset {
        self._glacier_base.position()
    }
    fn position_mut(&mut self) -> &mut super::game_shared_u_i::UIElementOffset {
        self._glacier_base.position_mut()
    }
    fn expansion(&self) -> &super::game_shared_u_i::UIElementRectExpansion {
        self._glacier_base.expansion()
    }
    fn expansion_mut(&mut self) -> &mut super::game_shared_u_i::UIElementRectExpansion {
        self._glacier_base.expansion_mut()
    }
    fn visible(&self) -> &bool {
        self._glacier_base.visible()
    }
    fn visible_mut(&mut self) -> &mut bool {
        self._glacier_base.visible_mut()
    }
    fn color(&self) -> &super::core::Vec3 {
        self._glacier_base.color()
    }
    fn color_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.color_mut()
    }
    fn alpha(&self) -> &f32 {
        self._glacier_base.alpha()
    }
    fn alpha_mut(&mut self) -> &mut f32 {
        self._glacier_base.alpha_mut()
    }
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::entity::EntityDataTrait for DiceUITypingInputListenerElementData {
}

impl super::entity::GameObjectDataTrait for DiceUITypingInputListenerElementData {
}

impl super::core::DataBusPeerTrait for DiceUITypingInputListenerElementData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for DiceUITypingInputListenerElementData {
}

impl super::core::DataContainerTrait for DiceUITypingInputListenerElementData {
}

pub static DICEUITYPINGINPUTLISTENERELEMENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUITypingInputListenerElementData",
    name_hash: 440242793,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_shared_u_i::UIELEMENTENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(DiceUITypingInputListenerElementData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DiceUITypingInputListenerElementData as Default>::default())),
            create_boxed: || Box::new(<DiceUITypingInputListenerElementData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "MaxTextLength",
                name_hash: 3158252880,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DiceUITypingInputListenerElementData, max_text_length),
            },
            FieldInfoData {
                name: "DefaultText",
                name_hash: 2015080531,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(DiceUITypingInputListenerElementData, default_text),
            },
            FieldInfoData {
                name: "Title",
                name_hash: 227868805,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(DiceUITypingInputListenerElementData, title),
            },
            FieldInfoData {
                name: "Description",
                name_hash: 1636673251,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(DiceUITypingInputListenerElementData, description),
            },
            FieldInfoData {
                name: "InputText",
                name_hash: 2651441390,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(DiceUITypingInputListenerElementData, input_text),
            },
            FieldInfoData {
                name: "AllowMultiline",
                name_hash: 45788987,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DiceUITypingInputListenerElementData, allow_multiline),
            },
            FieldInfoData {
                name: "AbortOnEscape",
                name_hash: 2566849615,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DiceUITypingInputListenerElementData, abort_on_escape),
            },
        ],
    }),
    array_type: Some(DICEUITYPINGINPUTLISTENERELEMENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DiceUITypingInputListenerElementData {
    fn type_info(&self) -> &'static TypeInfo {
        DICEUITYPINGINPUTLISTENERELEMENTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DICEUITYPINGINPUTLISTENERELEMENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUITypingInputListenerElementData-Array",
    name_hash: 2702649693,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceUITypingInputListenerElementData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DiceUIMouseInputListenerElementData {
    pub _glacier_base: super::game_shared_u_i::UIElementEntityData,
    pub mouse_button: super::u_i::UIMouseButton,
    pub consume_input: bool,
    pub full_screen: bool,
}

pub trait DiceUIMouseInputListenerElementDataTrait: super::game_shared_u_i::UIElementEntityDataTrait {
    fn mouse_button(&self) -> &super::u_i::UIMouseButton;
    fn mouse_button_mut(&mut self) -> &mut super::u_i::UIMouseButton;
    fn consume_input(&self) -> &bool;
    fn consume_input_mut(&mut self) -> &mut bool;
    fn full_screen(&self) -> &bool;
    fn full_screen_mut(&mut self) -> &mut bool;
}

impl DiceUIMouseInputListenerElementDataTrait for DiceUIMouseInputListenerElementData {
    fn mouse_button(&self) -> &super::u_i::UIMouseButton {
        &self.mouse_button
    }
    fn mouse_button_mut(&mut self) -> &mut super::u_i::UIMouseButton {
        &mut self.mouse_button
    }
    fn consume_input(&self) -> &bool {
        &self.consume_input
    }
    fn consume_input_mut(&mut self) -> &mut bool {
        &mut self.consume_input
    }
    fn full_screen(&self) -> &bool {
        &self.full_screen
    }
    fn full_screen_mut(&mut self) -> &mut bool {
        &mut self.full_screen
    }
}

impl super::game_shared_u_i::UIElementEntityDataTrait for DiceUIMouseInputListenerElementData {
    fn instance_name(&self) -> &String {
        self._glacier_base.instance_name()
    }
    fn instance_name_mut(&mut self) -> &mut String {
        self._glacier_base.instance_name_mut()
    }
    fn instance_name_hash(&self) -> &u32 {
        self._glacier_base.instance_name_hash()
    }
    fn instance_name_hash_mut(&mut self) -> &mut u32 {
        self._glacier_base.instance_name_hash_mut()
    }
    fn transform_pivot(&self) -> &super::core::Vec3 {
        self._glacier_base.transform_pivot()
    }
    fn transform_pivot_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.transform_pivot_mut()
    }
    fn size(&self) -> &super::core::Vec2 {
        self._glacier_base.size()
    }
    fn size_mut(&mut self) -> &mut super::core::Vec2 {
        self._glacier_base.size_mut()
    }
    fn layout_mode(&self) -> &super::game_shared_u_i::UILayoutMode {
        self._glacier_base.layout_mode()
    }
    fn layout_mode_mut(&mut self) -> &mut super::game_shared_u_i::UILayoutMode {
        self._glacier_base.layout_mode_mut()
    }
    fn offset(&self) -> &super::game_shared_u_i::UIElementOffset {
        self._glacier_base.offset()
    }
    fn offset_mut(&mut self) -> &mut super::game_shared_u_i::UIElementOffset {
        self._glacier_base.offset_mut()
    }
    fn anchor(&self) -> &super::game_shared_u_i::UIElementAnchor {
        self._glacier_base.anchor()
    }
    fn anchor_mut(&mut self) -> &mut super::game_shared_u_i::UIElementAnchor {
        self._glacier_base.anchor_mut()
    }
    fn position(&self) -> &super::game_shared_u_i::UIElementOffset {
        self._glacier_base.position()
    }
    fn position_mut(&mut self) -> &mut super::game_shared_u_i::UIElementOffset {
        self._glacier_base.position_mut()
    }
    fn expansion(&self) -> &super::game_shared_u_i::UIElementRectExpansion {
        self._glacier_base.expansion()
    }
    fn expansion_mut(&mut self) -> &mut super::game_shared_u_i::UIElementRectExpansion {
        self._glacier_base.expansion_mut()
    }
    fn visible(&self) -> &bool {
        self._glacier_base.visible()
    }
    fn visible_mut(&mut self) -> &mut bool {
        self._glacier_base.visible_mut()
    }
    fn color(&self) -> &super::core::Vec3 {
        self._glacier_base.color()
    }
    fn color_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.color_mut()
    }
    fn alpha(&self) -> &f32 {
        self._glacier_base.alpha()
    }
    fn alpha_mut(&mut self) -> &mut f32 {
        self._glacier_base.alpha_mut()
    }
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::entity::EntityDataTrait for DiceUIMouseInputListenerElementData {
}

impl super::entity::GameObjectDataTrait for DiceUIMouseInputListenerElementData {
}

impl super::core::DataBusPeerTrait for DiceUIMouseInputListenerElementData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for DiceUIMouseInputListenerElementData {
}

impl super::core::DataContainerTrait for DiceUIMouseInputListenerElementData {
}

pub static DICEUIMOUSEINPUTLISTENERELEMENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIMouseInputListenerElementData",
    name_hash: 1034219285,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_shared_u_i::UIELEMENTENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(DiceUIMouseInputListenerElementData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DiceUIMouseInputListenerElementData as Default>::default())),
            create_boxed: || Box::new(<DiceUIMouseInputListenerElementData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "MouseButton",
                name_hash: 1234605106,
                flags: MemberInfoFlags::new(0),
                field_type: "UIMouseButton",
                rust_offset: offset_of!(DiceUIMouseInputListenerElementData, mouse_button),
            },
            FieldInfoData {
                name: "ConsumeInput",
                name_hash: 182566815,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DiceUIMouseInputListenerElementData, consume_input),
            },
            FieldInfoData {
                name: "FullScreen",
                name_hash: 3418622618,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DiceUIMouseInputListenerElementData, full_screen),
            },
        ],
    }),
    array_type: Some(DICEUIMOUSEINPUTLISTENERELEMENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DiceUIMouseInputListenerElementData {
    fn type_info(&self) -> &'static TypeInfo {
        DICEUIMOUSEINPUTLISTENERELEMENTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DICEUIMOUSEINPUTLISTENERELEMENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIMouseInputListenerElementData-Array",
    name_hash: 2560106145,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceUIMouseInputListenerElementData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DiceUIInputActionListenerElementData {
    pub _glacier_base: super::game_shared_u_i::UIElementEntityData,
    pub input_action: super::u_i::UIInputAction,
    pub consume_input: bool,
}

pub trait DiceUIInputActionListenerElementDataTrait: super::game_shared_u_i::UIElementEntityDataTrait {
    fn input_action(&self) -> &super::u_i::UIInputAction;
    fn input_action_mut(&mut self) -> &mut super::u_i::UIInputAction;
    fn consume_input(&self) -> &bool;
    fn consume_input_mut(&mut self) -> &mut bool;
}

impl DiceUIInputActionListenerElementDataTrait for DiceUIInputActionListenerElementData {
    fn input_action(&self) -> &super::u_i::UIInputAction {
        &self.input_action
    }
    fn input_action_mut(&mut self) -> &mut super::u_i::UIInputAction {
        &mut self.input_action
    }
    fn consume_input(&self) -> &bool {
        &self.consume_input
    }
    fn consume_input_mut(&mut self) -> &mut bool {
        &mut self.consume_input
    }
}

impl super::game_shared_u_i::UIElementEntityDataTrait for DiceUIInputActionListenerElementData {
    fn instance_name(&self) -> &String {
        self._glacier_base.instance_name()
    }
    fn instance_name_mut(&mut self) -> &mut String {
        self._glacier_base.instance_name_mut()
    }
    fn instance_name_hash(&self) -> &u32 {
        self._glacier_base.instance_name_hash()
    }
    fn instance_name_hash_mut(&mut self) -> &mut u32 {
        self._glacier_base.instance_name_hash_mut()
    }
    fn transform_pivot(&self) -> &super::core::Vec3 {
        self._glacier_base.transform_pivot()
    }
    fn transform_pivot_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.transform_pivot_mut()
    }
    fn size(&self) -> &super::core::Vec2 {
        self._glacier_base.size()
    }
    fn size_mut(&mut self) -> &mut super::core::Vec2 {
        self._glacier_base.size_mut()
    }
    fn layout_mode(&self) -> &super::game_shared_u_i::UILayoutMode {
        self._glacier_base.layout_mode()
    }
    fn layout_mode_mut(&mut self) -> &mut super::game_shared_u_i::UILayoutMode {
        self._glacier_base.layout_mode_mut()
    }
    fn offset(&self) -> &super::game_shared_u_i::UIElementOffset {
        self._glacier_base.offset()
    }
    fn offset_mut(&mut self) -> &mut super::game_shared_u_i::UIElementOffset {
        self._glacier_base.offset_mut()
    }
    fn anchor(&self) -> &super::game_shared_u_i::UIElementAnchor {
        self._glacier_base.anchor()
    }
    fn anchor_mut(&mut self) -> &mut super::game_shared_u_i::UIElementAnchor {
        self._glacier_base.anchor_mut()
    }
    fn position(&self) -> &super::game_shared_u_i::UIElementOffset {
        self._glacier_base.position()
    }
    fn position_mut(&mut self) -> &mut super::game_shared_u_i::UIElementOffset {
        self._glacier_base.position_mut()
    }
    fn expansion(&self) -> &super::game_shared_u_i::UIElementRectExpansion {
        self._glacier_base.expansion()
    }
    fn expansion_mut(&mut self) -> &mut super::game_shared_u_i::UIElementRectExpansion {
        self._glacier_base.expansion_mut()
    }
    fn visible(&self) -> &bool {
        self._glacier_base.visible()
    }
    fn visible_mut(&mut self) -> &mut bool {
        self._glacier_base.visible_mut()
    }
    fn color(&self) -> &super::core::Vec3 {
        self._glacier_base.color()
    }
    fn color_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.color_mut()
    }
    fn alpha(&self) -> &f32 {
        self._glacier_base.alpha()
    }
    fn alpha_mut(&mut self) -> &mut f32 {
        self._glacier_base.alpha_mut()
    }
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::entity::EntityDataTrait for DiceUIInputActionListenerElementData {
}

impl super::entity::GameObjectDataTrait for DiceUIInputActionListenerElementData {
}

impl super::core::DataBusPeerTrait for DiceUIInputActionListenerElementData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for DiceUIInputActionListenerElementData {
}

impl super::core::DataContainerTrait for DiceUIInputActionListenerElementData {
}

pub static DICEUIINPUTACTIONLISTENERELEMENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIInputActionListenerElementData",
    name_hash: 2211242538,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_shared_u_i::UIELEMENTENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(DiceUIInputActionListenerElementData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DiceUIInputActionListenerElementData as Default>::default())),
            create_boxed: || Box::new(<DiceUIInputActionListenerElementData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "InputAction",
                name_hash: 1407707693,
                flags: MemberInfoFlags::new(0),
                field_type: "UIInputAction",
                rust_offset: offset_of!(DiceUIInputActionListenerElementData, input_action),
            },
            FieldInfoData {
                name: "ConsumeInput",
                name_hash: 182566815,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DiceUIInputActionListenerElementData, consume_input),
            },
        ],
    }),
    array_type: Some(DICEUIINPUTACTIONLISTENERELEMENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DiceUIInputActionListenerElementData {
    fn type_info(&self) -> &'static TypeInfo {
        DICEUIINPUTACTIONLISTENERELEMENTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DICEUIINPUTACTIONLISTENERELEMENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIInputActionListenerElementData-Array",
    name_hash: 1784954014,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceUIInputActionListenerElementData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DiceUIInputBlockerElementData {
    pub _glacier_base: super::game_shared_u_i::UIElementEntityData,
    pub full_screen: bool,
}

pub trait DiceUIInputBlockerElementDataTrait: super::game_shared_u_i::UIElementEntityDataTrait {
    fn full_screen(&self) -> &bool;
    fn full_screen_mut(&mut self) -> &mut bool;
}

impl DiceUIInputBlockerElementDataTrait for DiceUIInputBlockerElementData {
    fn full_screen(&self) -> &bool {
        &self.full_screen
    }
    fn full_screen_mut(&mut self) -> &mut bool {
        &mut self.full_screen
    }
}

impl super::game_shared_u_i::UIElementEntityDataTrait for DiceUIInputBlockerElementData {
    fn instance_name(&self) -> &String {
        self._glacier_base.instance_name()
    }
    fn instance_name_mut(&mut self) -> &mut String {
        self._glacier_base.instance_name_mut()
    }
    fn instance_name_hash(&self) -> &u32 {
        self._glacier_base.instance_name_hash()
    }
    fn instance_name_hash_mut(&mut self) -> &mut u32 {
        self._glacier_base.instance_name_hash_mut()
    }
    fn transform_pivot(&self) -> &super::core::Vec3 {
        self._glacier_base.transform_pivot()
    }
    fn transform_pivot_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.transform_pivot_mut()
    }
    fn size(&self) -> &super::core::Vec2 {
        self._glacier_base.size()
    }
    fn size_mut(&mut self) -> &mut super::core::Vec2 {
        self._glacier_base.size_mut()
    }
    fn layout_mode(&self) -> &super::game_shared_u_i::UILayoutMode {
        self._glacier_base.layout_mode()
    }
    fn layout_mode_mut(&mut self) -> &mut super::game_shared_u_i::UILayoutMode {
        self._glacier_base.layout_mode_mut()
    }
    fn offset(&self) -> &super::game_shared_u_i::UIElementOffset {
        self._glacier_base.offset()
    }
    fn offset_mut(&mut self) -> &mut super::game_shared_u_i::UIElementOffset {
        self._glacier_base.offset_mut()
    }
    fn anchor(&self) -> &super::game_shared_u_i::UIElementAnchor {
        self._glacier_base.anchor()
    }
    fn anchor_mut(&mut self) -> &mut super::game_shared_u_i::UIElementAnchor {
        self._glacier_base.anchor_mut()
    }
    fn position(&self) -> &super::game_shared_u_i::UIElementOffset {
        self._glacier_base.position()
    }
    fn position_mut(&mut self) -> &mut super::game_shared_u_i::UIElementOffset {
        self._glacier_base.position_mut()
    }
    fn expansion(&self) -> &super::game_shared_u_i::UIElementRectExpansion {
        self._glacier_base.expansion()
    }
    fn expansion_mut(&mut self) -> &mut super::game_shared_u_i::UIElementRectExpansion {
        self._glacier_base.expansion_mut()
    }
    fn visible(&self) -> &bool {
        self._glacier_base.visible()
    }
    fn visible_mut(&mut self) -> &mut bool {
        self._glacier_base.visible_mut()
    }
    fn color(&self) -> &super::core::Vec3 {
        self._glacier_base.color()
    }
    fn color_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.color_mut()
    }
    fn alpha(&self) -> &f32 {
        self._glacier_base.alpha()
    }
    fn alpha_mut(&mut self) -> &mut f32 {
        self._glacier_base.alpha_mut()
    }
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::entity::EntityDataTrait for DiceUIInputBlockerElementData {
}

impl super::entity::GameObjectDataTrait for DiceUIInputBlockerElementData {
}

impl super::core::DataBusPeerTrait for DiceUIInputBlockerElementData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for DiceUIInputBlockerElementData {
}

impl super::core::DataContainerTrait for DiceUIInputBlockerElementData {
}

pub static DICEUIINPUTBLOCKERELEMENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIInputBlockerElementData",
    name_hash: 2654914516,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_shared_u_i::UIELEMENTENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(DiceUIInputBlockerElementData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DiceUIInputBlockerElementData as Default>::default())),
            create_boxed: || Box::new(<DiceUIInputBlockerElementData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "FullScreen",
                name_hash: 3418622618,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DiceUIInputBlockerElementData, full_screen),
            },
        ],
    }),
    array_type: Some(DICEUIINPUTBLOCKERELEMENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DiceUIInputBlockerElementData {
    fn type_info(&self) -> &'static TypeInfo {
        DICEUIINPUTBLOCKERELEMENTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DICEUIINPUTBLOCKERELEMENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIInputBlockerElementData-Array",
    name_hash: 1943712864,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceUIInputBlockerElementData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DiceUIAnalogStickInputListenerElementData {
    pub _glacier_base: super::game_shared_u_i::UIElementEntityData,
    pub analog_stick: DiceUIAnalogStick,
    pub consume_input: bool,
    pub flip_y_axis: bool,
    pub trigger_threshold: f32,
    pub dead_zone: f32,
}

pub trait DiceUIAnalogStickInputListenerElementDataTrait: super::game_shared_u_i::UIElementEntityDataTrait {
    fn analog_stick(&self) -> &DiceUIAnalogStick;
    fn analog_stick_mut(&mut self) -> &mut DiceUIAnalogStick;
    fn consume_input(&self) -> &bool;
    fn consume_input_mut(&mut self) -> &mut bool;
    fn flip_y_axis(&self) -> &bool;
    fn flip_y_axis_mut(&mut self) -> &mut bool;
    fn trigger_threshold(&self) -> &f32;
    fn trigger_threshold_mut(&mut self) -> &mut f32;
    fn dead_zone(&self) -> &f32;
    fn dead_zone_mut(&mut self) -> &mut f32;
}

impl DiceUIAnalogStickInputListenerElementDataTrait for DiceUIAnalogStickInputListenerElementData {
    fn analog_stick(&self) -> &DiceUIAnalogStick {
        &self.analog_stick
    }
    fn analog_stick_mut(&mut self) -> &mut DiceUIAnalogStick {
        &mut self.analog_stick
    }
    fn consume_input(&self) -> &bool {
        &self.consume_input
    }
    fn consume_input_mut(&mut self) -> &mut bool {
        &mut self.consume_input
    }
    fn flip_y_axis(&self) -> &bool {
        &self.flip_y_axis
    }
    fn flip_y_axis_mut(&mut self) -> &mut bool {
        &mut self.flip_y_axis
    }
    fn trigger_threshold(&self) -> &f32 {
        &self.trigger_threshold
    }
    fn trigger_threshold_mut(&mut self) -> &mut f32 {
        &mut self.trigger_threshold
    }
    fn dead_zone(&self) -> &f32 {
        &self.dead_zone
    }
    fn dead_zone_mut(&mut self) -> &mut f32 {
        &mut self.dead_zone
    }
}

impl super::game_shared_u_i::UIElementEntityDataTrait for DiceUIAnalogStickInputListenerElementData {
    fn instance_name(&self) -> &String {
        self._glacier_base.instance_name()
    }
    fn instance_name_mut(&mut self) -> &mut String {
        self._glacier_base.instance_name_mut()
    }
    fn instance_name_hash(&self) -> &u32 {
        self._glacier_base.instance_name_hash()
    }
    fn instance_name_hash_mut(&mut self) -> &mut u32 {
        self._glacier_base.instance_name_hash_mut()
    }
    fn transform_pivot(&self) -> &super::core::Vec3 {
        self._glacier_base.transform_pivot()
    }
    fn transform_pivot_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.transform_pivot_mut()
    }
    fn size(&self) -> &super::core::Vec2 {
        self._glacier_base.size()
    }
    fn size_mut(&mut self) -> &mut super::core::Vec2 {
        self._glacier_base.size_mut()
    }
    fn layout_mode(&self) -> &super::game_shared_u_i::UILayoutMode {
        self._glacier_base.layout_mode()
    }
    fn layout_mode_mut(&mut self) -> &mut super::game_shared_u_i::UILayoutMode {
        self._glacier_base.layout_mode_mut()
    }
    fn offset(&self) -> &super::game_shared_u_i::UIElementOffset {
        self._glacier_base.offset()
    }
    fn offset_mut(&mut self) -> &mut super::game_shared_u_i::UIElementOffset {
        self._glacier_base.offset_mut()
    }
    fn anchor(&self) -> &super::game_shared_u_i::UIElementAnchor {
        self._glacier_base.anchor()
    }
    fn anchor_mut(&mut self) -> &mut super::game_shared_u_i::UIElementAnchor {
        self._glacier_base.anchor_mut()
    }
    fn position(&self) -> &super::game_shared_u_i::UIElementOffset {
        self._glacier_base.position()
    }
    fn position_mut(&mut self) -> &mut super::game_shared_u_i::UIElementOffset {
        self._glacier_base.position_mut()
    }
    fn expansion(&self) -> &super::game_shared_u_i::UIElementRectExpansion {
        self._glacier_base.expansion()
    }
    fn expansion_mut(&mut self) -> &mut super::game_shared_u_i::UIElementRectExpansion {
        self._glacier_base.expansion_mut()
    }
    fn visible(&self) -> &bool {
        self._glacier_base.visible()
    }
    fn visible_mut(&mut self) -> &mut bool {
        self._glacier_base.visible_mut()
    }
    fn color(&self) -> &super::core::Vec3 {
        self._glacier_base.color()
    }
    fn color_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.color_mut()
    }
    fn alpha(&self) -> &f32 {
        self._glacier_base.alpha()
    }
    fn alpha_mut(&mut self) -> &mut f32 {
        self._glacier_base.alpha_mut()
    }
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::entity::EntityDataTrait for DiceUIAnalogStickInputListenerElementData {
}

impl super::entity::GameObjectDataTrait for DiceUIAnalogStickInputListenerElementData {
}

impl super::core::DataBusPeerTrait for DiceUIAnalogStickInputListenerElementData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for DiceUIAnalogStickInputListenerElementData {
}

impl super::core::DataContainerTrait for DiceUIAnalogStickInputListenerElementData {
}

pub static DICEUIANALOGSTICKINPUTLISTENERELEMENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIAnalogStickInputListenerElementData",
    name_hash: 3132208120,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_shared_u_i::UIELEMENTENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(DiceUIAnalogStickInputListenerElementData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DiceUIAnalogStickInputListenerElementData as Default>::default())),
            create_boxed: || Box::new(<DiceUIAnalogStickInputListenerElementData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "AnalogStick",
                name_hash: 737758793,
                flags: MemberInfoFlags::new(0),
                field_type: "DiceUIAnalogStick",
                rust_offset: offset_of!(DiceUIAnalogStickInputListenerElementData, analog_stick),
            },
            FieldInfoData {
                name: "ConsumeInput",
                name_hash: 182566815,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DiceUIAnalogStickInputListenerElementData, consume_input),
            },
            FieldInfoData {
                name: "FlipYAxis",
                name_hash: 1346913068,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DiceUIAnalogStickInputListenerElementData, flip_y_axis),
            },
            FieldInfoData {
                name: "TriggerThreshold",
                name_hash: 2293412650,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DiceUIAnalogStickInputListenerElementData, trigger_threshold),
            },
            FieldInfoData {
                name: "DeadZone",
                name_hash: 3291734079,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DiceUIAnalogStickInputListenerElementData, dead_zone),
            },
        ],
    }),
    array_type: Some(DICEUIANALOGSTICKINPUTLISTENERELEMENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DiceUIAnalogStickInputListenerElementData {
    fn type_info(&self) -> &'static TypeInfo {
        DICEUIANALOGSTICKINPUTLISTENERELEMENTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DICEUIANALOGSTICKINPUTLISTENERELEMENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIAnalogStickInputListenerElementData-Array",
    name_hash: 1881517516,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceUIAnalogStickInputListenerElementData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum DiceUIAnalogStick {
    #[default]
    DiceUIAnalogStick_Left = 0,
    DiceUIAnalogStick_Right = 1,
}

pub static DICEUIANALOGSTICK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIAnalogStick",
    name_hash: 2703673502,
    flags: MemberInfoFlags::new(49429),
    module: "DiceCommonsShared",
    data: TypeInfoData::Enum,
    array_type: Some(DICEUIANALOGSTICK_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DiceUIAnalogStick {
    fn type_info(&self) -> &'static TypeInfo {
        DICEUIANALOGSTICK_TYPE_INFO
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


pub static DICEUIANALOGSTICK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIAnalogStick-Array",
    name_hash: 2480396714,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceUIAnalogStick"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DiceUIAnalogPadInputListenerElementData {
    pub _glacier_base: super::game_shared_u_i::UIElementEntityData,
    pub analog_pad_type: DiceUIAnalogPadType,
    pub consume_input: bool,
}

pub trait DiceUIAnalogPadInputListenerElementDataTrait: super::game_shared_u_i::UIElementEntityDataTrait {
    fn analog_pad_type(&self) -> &DiceUIAnalogPadType;
    fn analog_pad_type_mut(&mut self) -> &mut DiceUIAnalogPadType;
    fn consume_input(&self) -> &bool;
    fn consume_input_mut(&mut self) -> &mut bool;
}

impl DiceUIAnalogPadInputListenerElementDataTrait for DiceUIAnalogPadInputListenerElementData {
    fn analog_pad_type(&self) -> &DiceUIAnalogPadType {
        &self.analog_pad_type
    }
    fn analog_pad_type_mut(&mut self) -> &mut DiceUIAnalogPadType {
        &mut self.analog_pad_type
    }
    fn consume_input(&self) -> &bool {
        &self.consume_input
    }
    fn consume_input_mut(&mut self) -> &mut bool {
        &mut self.consume_input
    }
}

impl super::game_shared_u_i::UIElementEntityDataTrait for DiceUIAnalogPadInputListenerElementData {
    fn instance_name(&self) -> &String {
        self._glacier_base.instance_name()
    }
    fn instance_name_mut(&mut self) -> &mut String {
        self._glacier_base.instance_name_mut()
    }
    fn instance_name_hash(&self) -> &u32 {
        self._glacier_base.instance_name_hash()
    }
    fn instance_name_hash_mut(&mut self) -> &mut u32 {
        self._glacier_base.instance_name_hash_mut()
    }
    fn transform_pivot(&self) -> &super::core::Vec3 {
        self._glacier_base.transform_pivot()
    }
    fn transform_pivot_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.transform_pivot_mut()
    }
    fn size(&self) -> &super::core::Vec2 {
        self._glacier_base.size()
    }
    fn size_mut(&mut self) -> &mut super::core::Vec2 {
        self._glacier_base.size_mut()
    }
    fn layout_mode(&self) -> &super::game_shared_u_i::UILayoutMode {
        self._glacier_base.layout_mode()
    }
    fn layout_mode_mut(&mut self) -> &mut super::game_shared_u_i::UILayoutMode {
        self._glacier_base.layout_mode_mut()
    }
    fn offset(&self) -> &super::game_shared_u_i::UIElementOffset {
        self._glacier_base.offset()
    }
    fn offset_mut(&mut self) -> &mut super::game_shared_u_i::UIElementOffset {
        self._glacier_base.offset_mut()
    }
    fn anchor(&self) -> &super::game_shared_u_i::UIElementAnchor {
        self._glacier_base.anchor()
    }
    fn anchor_mut(&mut self) -> &mut super::game_shared_u_i::UIElementAnchor {
        self._glacier_base.anchor_mut()
    }
    fn position(&self) -> &super::game_shared_u_i::UIElementOffset {
        self._glacier_base.position()
    }
    fn position_mut(&mut self) -> &mut super::game_shared_u_i::UIElementOffset {
        self._glacier_base.position_mut()
    }
    fn expansion(&self) -> &super::game_shared_u_i::UIElementRectExpansion {
        self._glacier_base.expansion()
    }
    fn expansion_mut(&mut self) -> &mut super::game_shared_u_i::UIElementRectExpansion {
        self._glacier_base.expansion_mut()
    }
    fn visible(&self) -> &bool {
        self._glacier_base.visible()
    }
    fn visible_mut(&mut self) -> &mut bool {
        self._glacier_base.visible_mut()
    }
    fn color(&self) -> &super::core::Vec3 {
        self._glacier_base.color()
    }
    fn color_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.color_mut()
    }
    fn alpha(&self) -> &f32 {
        self._glacier_base.alpha()
    }
    fn alpha_mut(&mut self) -> &mut f32 {
        self._glacier_base.alpha_mut()
    }
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::entity::EntityDataTrait for DiceUIAnalogPadInputListenerElementData {
}

impl super::entity::GameObjectDataTrait for DiceUIAnalogPadInputListenerElementData {
}

impl super::core::DataBusPeerTrait for DiceUIAnalogPadInputListenerElementData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for DiceUIAnalogPadInputListenerElementData {
}

impl super::core::DataContainerTrait for DiceUIAnalogPadInputListenerElementData {
}

pub static DICEUIANALOGPADINPUTLISTENERELEMENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIAnalogPadInputListenerElementData",
    name_hash: 230425067,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_shared_u_i::UIELEMENTENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(DiceUIAnalogPadInputListenerElementData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DiceUIAnalogPadInputListenerElementData as Default>::default())),
            create_boxed: || Box::new(<DiceUIAnalogPadInputListenerElementData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "AnalogPadType",
                name_hash: 2857694370,
                flags: MemberInfoFlags::new(0),
                field_type: "DiceUIAnalogPadType",
                rust_offset: offset_of!(DiceUIAnalogPadInputListenerElementData, analog_pad_type),
            },
            FieldInfoData {
                name: "ConsumeInput",
                name_hash: 182566815,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DiceUIAnalogPadInputListenerElementData, consume_input),
            },
        ],
    }),
    array_type: Some(DICEUIANALOGPADINPUTLISTENERELEMENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DiceUIAnalogPadInputListenerElementData {
    fn type_info(&self) -> &'static TypeInfo {
        DICEUIANALOGPADINPUTLISTENERELEMENTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DICEUIANALOGPADINPUTLISTENERELEMENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIAnalogPadInputListenerElementData-Array",
    name_hash: 1379365343,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceUIAnalogPadInputListenerElementData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DiceUIInputManagerEntityData {
    pub _glacier_base: super::entity::EntityData,
}

pub trait DiceUIInputManagerEntityDataTrait: super::entity::EntityDataTrait {
}

impl DiceUIInputManagerEntityDataTrait for DiceUIInputManagerEntityData {
}

impl super::entity::EntityDataTrait for DiceUIInputManagerEntityData {
}

impl super::entity::GameObjectDataTrait for DiceUIInputManagerEntityData {
}

impl super::core::DataBusPeerTrait for DiceUIInputManagerEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for DiceUIInputManagerEntityData {
}

impl super::core::DataContainerTrait for DiceUIInputManagerEntityData {
}

pub static DICEUIINPUTMANAGERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIInputManagerEntityData",
    name_hash: 3825231324,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(DiceUIInputManagerEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DiceUIInputManagerEntityData as Default>::default())),
            create_boxed: || Box::new(<DiceUIInputManagerEntityData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DICEUIINPUTMANAGERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DiceUIInputManagerEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        DICEUIINPUTMANAGERENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DICEUIINPUTMANAGERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIInputManagerEntityData-Array",
    name_hash: 4200214120,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceUIInputManagerEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DiceDebugUIInputFlowSimulationData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub title: String,
    pub force_game_pad_on_windows: bool,
    pub player: i32,
    pub actions: Vec<BoxedTypeObject /* DiceUIInputFlowAction */>,
}

pub trait DiceDebugUIInputFlowSimulationDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn title(&self) -> &String;
    fn title_mut(&mut self) -> &mut String;
    fn force_game_pad_on_windows(&self) -> &bool;
    fn force_game_pad_on_windows_mut(&mut self) -> &mut bool;
    fn player(&self) -> &i32;
    fn player_mut(&mut self) -> &mut i32;
    fn actions(&self) -> &Vec<BoxedTypeObject /* DiceUIInputFlowAction */>;
    fn actions_mut(&mut self) -> &mut Vec<BoxedTypeObject /* DiceUIInputFlowAction */>;
}

impl DiceDebugUIInputFlowSimulationDataTrait for DiceDebugUIInputFlowSimulationData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn title(&self) -> &String {
        &self.title
    }
    fn title_mut(&mut self) -> &mut String {
        &mut self.title
    }
    fn force_game_pad_on_windows(&self) -> &bool {
        &self.force_game_pad_on_windows
    }
    fn force_game_pad_on_windows_mut(&mut self) -> &mut bool {
        &mut self.force_game_pad_on_windows
    }
    fn player(&self) -> &i32 {
        &self.player
    }
    fn player_mut(&mut self) -> &mut i32 {
        &mut self.player
    }
    fn actions(&self) -> &Vec<BoxedTypeObject /* DiceUIInputFlowAction */> {
        &self.actions
    }
    fn actions_mut(&mut self) -> &mut Vec<BoxedTypeObject /* DiceUIInputFlowAction */> {
        &mut self.actions
    }
}

impl super::entity::EntityDataTrait for DiceDebugUIInputFlowSimulationData {
}

impl super::entity::GameObjectDataTrait for DiceDebugUIInputFlowSimulationData {
}

impl super::core::DataBusPeerTrait for DiceDebugUIInputFlowSimulationData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for DiceDebugUIInputFlowSimulationData {
}

impl super::core::DataContainerTrait for DiceDebugUIInputFlowSimulationData {
}

pub static DICEDEBUGUIINPUTFLOWSIMULATIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceDebugUIInputFlowSimulationData",
    name_hash: 1964669572,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(DiceDebugUIInputFlowSimulationData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DiceDebugUIInputFlowSimulationData as Default>::default())),
            create_boxed: || Box::new(<DiceDebugUIInputFlowSimulationData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                name_hash: 229961746,
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(DiceDebugUIInputFlowSimulationData, realm),
            },
            FieldInfoData {
                name: "Title",
                name_hash: 227868805,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(DiceDebugUIInputFlowSimulationData, title),
            },
            FieldInfoData {
                name: "ForceGamePadOnWindows",
                name_hash: 801791037,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DiceDebugUIInputFlowSimulationData, force_game_pad_on_windows),
            },
            FieldInfoData {
                name: "Player",
                name_hash: 3384765366,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(DiceDebugUIInputFlowSimulationData, player),
            },
            FieldInfoData {
                name: "Actions",
                name_hash: 373511656,
                flags: MemberInfoFlags::new(144),
                field_type: "DiceUIInputFlowAction-Array",
                rust_offset: offset_of!(DiceDebugUIInputFlowSimulationData, actions),
            },
        ],
    }),
    array_type: Some(DICEDEBUGUIINPUTFLOWSIMULATIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DiceDebugUIInputFlowSimulationData {
    fn type_info(&self) -> &'static TypeInfo {
        DICEDEBUGUIINPUTFLOWSIMULATIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DICEDEBUGUIINPUTFLOWSIMULATIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceDebugUIInputFlowSimulationData-Array",
    name_hash: 2760552112,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceDebugUIInputFlowSimulationData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DiceUIInputFlowAction {
    pub name: String,
    pub action: super::u_i::UIInputAction,
    pub press_duration: f32,
    pub wait_after_release: f32,
}

pub trait DiceUIInputFlowActionTrait: TypeObject {
    fn name(&self) -> &String;
    fn name_mut(&mut self) -> &mut String;
    fn action(&self) -> &super::u_i::UIInputAction;
    fn action_mut(&mut self) -> &mut super::u_i::UIInputAction;
    fn press_duration(&self) -> &f32;
    fn press_duration_mut(&mut self) -> &mut f32;
    fn wait_after_release(&self) -> &f32;
    fn wait_after_release_mut(&mut self) -> &mut f32;
}

impl DiceUIInputFlowActionTrait for DiceUIInputFlowAction {
    fn name(&self) -> &String {
        &self.name
    }
    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    fn action(&self) -> &super::u_i::UIInputAction {
        &self.action
    }
    fn action_mut(&mut self) -> &mut super::u_i::UIInputAction {
        &mut self.action
    }
    fn press_duration(&self) -> &f32 {
        &self.press_duration
    }
    fn press_duration_mut(&mut self) -> &mut f32 {
        &mut self.press_duration
    }
    fn wait_after_release(&self) -> &f32 {
        &self.wait_after_release
    }
    fn wait_after_release_mut(&mut self) -> &mut f32 {
        &mut self.wait_after_release
    }
}

pub static DICEUIINPUTFLOWACTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIInputFlowAction",
    name_hash: 1236891880,
    flags: MemberInfoFlags::new(73),
    module: "DiceCommonsShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DiceUIInputFlowAction as Default>::default())),
            create_boxed: || Box::new(<DiceUIInputFlowAction as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                name_hash: 2088949890,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(DiceUIInputFlowAction, name),
            },
            FieldInfoData {
                name: "Action",
                name_hash: 2484178491,
                flags: MemberInfoFlags::new(0),
                field_type: "UIInputAction",
                rust_offset: offset_of!(DiceUIInputFlowAction, action),
            },
            FieldInfoData {
                name: "PressDuration",
                name_hash: 676661148,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DiceUIInputFlowAction, press_duration),
            },
            FieldInfoData {
                name: "WaitAfterRelease",
                name_hash: 4063898915,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DiceUIInputFlowAction, wait_after_release),
            },
        ],
    }),
    array_type: Some(DICEUIINPUTFLOWACTION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DiceUIInputFlowAction {
    fn type_info(&self) -> &'static TypeInfo {
        DICEUIINPUTFLOWACTION_TYPE_INFO
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


pub static DICEUIINPUTFLOWACTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIInputFlowAction-Array",
    name_hash: 2845469404,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceUIInputFlowAction"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CharacterDefinitionComponentData {
    pub _glacier_base: super::entity::GameComponentData,
}

pub trait CharacterDefinitionComponentDataTrait: super::entity::GameComponentDataTrait {
}

impl CharacterDefinitionComponentDataTrait for CharacterDefinitionComponentData {
}

impl super::entity::GameComponentDataTrait for CharacterDefinitionComponentData {
}

impl super::entity::ComponentDataTrait for CharacterDefinitionComponentData {
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

impl super::entity::GameObjectDataTrait for CharacterDefinitionComponentData {
}

impl super::core::DataBusPeerTrait for CharacterDefinitionComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for CharacterDefinitionComponentData {
}

impl super::core::DataContainerTrait for CharacterDefinitionComponentData {
}

pub static CHARACTERDEFINITIONCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterDefinitionComponentData",
    name_hash: 3461110102,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::GAMECOMPONENTDATA_TYPE_INFO),
        super_class_offset: offset_of!(CharacterDefinitionComponentData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CharacterDefinitionComponentData as Default>::default())),
            create_boxed: || Box::new(<CharacterDefinitionComponentData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CHARACTERDEFINITIONCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CharacterDefinitionComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        CHARACTERDEFINITIONCOMPONENTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CHARACTERDEFINITIONCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterDefinitionComponentData-Array",
    name_hash: 2372394722,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("CharacterDefinitionComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CharacterDefinitionSpawnData {
    pub _glacier_base: super::game_shared::CharacterSpawnReferenceObjectData,
    pub character_definition: Option<LockedTypeObject /* CharacterDefinition */>,
}

pub trait CharacterDefinitionSpawnDataTrait: super::game_shared::CharacterSpawnReferenceObjectDataTrait {
    fn character_definition(&self) -> &Option<LockedTypeObject /* CharacterDefinition */>;
    fn character_definition_mut(&mut self) -> &mut Option<LockedTypeObject /* CharacterDefinition */>;
}

impl CharacterDefinitionSpawnDataTrait for CharacterDefinitionSpawnData {
    fn character_definition(&self) -> &Option<LockedTypeObject /* CharacterDefinition */> {
        &self.character_definition
    }
    fn character_definition_mut(&mut self) -> &mut Option<LockedTypeObject /* CharacterDefinition */> {
        &mut self.character_definition
    }
}

impl super::game_shared::CharacterSpawnReferenceObjectDataTrait for CharacterDefinitionSpawnData {
    fn vehicle_entry_index(&self) -> &u32 {
        self._glacier_base.vehicle_entry_index()
    }
    fn vehicle_entry_index_mut(&mut self) -> &mut u32 {
        self._glacier_base.vehicle_entry_index_mut()
    }
    fn allow_fallback_on_next_availabe_vehicle_entry(&self) -> &bool {
        self._glacier_base.allow_fallback_on_next_availabe_vehicle_entry()
    }
    fn allow_fallback_on_next_availabe_vehicle_entry_mut(&mut self) -> &mut bool {
        self._glacier_base.allow_fallback_on_next_availabe_vehicle_entry_mut()
    }
    fn template(&self) -> &Option<LockedTypeObject /* super::game_shared::CharacterSpawnTemplateData */> {
        self._glacier_base.template()
    }
    fn template_mut(&mut self) -> &mut Option<LockedTypeObject /* super::game_shared::CharacterSpawnTemplateData */> {
        self._glacier_base.template_mut()
    }
    fn spawn_visible(&self) -> &bool {
        self._glacier_base.spawn_visible()
    }
    fn spawn_visible_mut(&mut self) -> &mut bool {
        self._glacier_base.spawn_visible_mut()
    }
    fn human_target_preference(&self) -> &f32 {
        self._glacier_base.human_target_preference()
    }
    fn human_target_preference_mut(&mut self) -> &mut f32 {
        self._glacier_base.human_target_preference_mut()
    }
    fn is_target(&self) -> &bool {
        self._glacier_base.is_target()
    }
    fn is_target_mut(&mut self) -> &mut bool {
        self._glacier_base.is_target_mut()
    }
    fn affect_minimap_position(&self) -> &bool {
        self._glacier_base.affect_minimap_position()
    }
    fn affect_minimap_position_mut(&mut self) -> &mut bool {
        self._glacier_base.affect_minimap_position_mut()
    }
    fn show_as_label_only(&self) -> &bool {
        self._glacier_base.show_as_label_only()
    }
    fn show_as_label_only_mut(&mut self) -> &mut bool {
        self._glacier_base.show_as_label_only_mut()
    }
    fn show_in_menu(&self) -> &bool {
        self._glacier_base.show_in_menu()
    }
    fn show_in_menu_mut(&mut self) -> &mut bool {
        self._glacier_base.show_in_menu_mut()
    }
    fn menu_show_order(&self) -> &i32 {
        self._glacier_base.menu_show_order()
    }
    fn menu_show_order_mut(&mut self) -> &mut i32 {
        self._glacier_base.menu_show_order_mut()
    }
}

impl super::game_shared::SpawnReferenceObjectDataTrait for CharacterDefinitionSpawnData {
    fn team(&self) -> &super::gameplay_sim::TeamId {
        self._glacier_base.team()
    }
    fn team_mut(&mut self) -> &mut super::gameplay_sim::TeamId {
        self._glacier_base.team_mut()
    }
    fn locked_team(&self) -> &bool {
        self._glacier_base.locked_team()
    }
    fn locked_team_mut(&mut self) -> &mut bool {
        self._glacier_base.locked_team_mut()
    }
    fn spawn_area_radius(&self) -> &f32 {
        self._glacier_base.spawn_area_radius()
    }
    fn spawn_area_radius_mut(&mut self) -> &mut f32 {
        self._glacier_base.spawn_area_radius_mut()
    }
    fn spawn_protection_radius(&self) -> &f32 {
        self._glacier_base.spawn_protection_radius()
    }
    fn spawn_protection_radius_mut(&mut self) -> &mut f32 {
        self._glacier_base.spawn_protection_radius_mut()
    }
    fn spawn_protection_check_all_teams(&self) -> &bool {
        self._glacier_base.spawn_protection_check_all_teams()
    }
    fn spawn_protection_check_all_teams_mut(&mut self) -> &mut bool {
        self._glacier_base.spawn_protection_check_all_teams_mut()
    }
    fn spawn_protection_friendly_killed_count(&self) -> &u32 {
        self._glacier_base.spawn_protection_friendly_killed_count()
    }
    fn spawn_protection_friendly_killed_count_mut(&mut self) -> &mut u32 {
        self._glacier_base.spawn_protection_friendly_killed_count_mut()
    }
    fn spawn_protection_friendly_killed_time(&self) -> &f32 {
        self._glacier_base.spawn_protection_friendly_killed_time()
    }
    fn spawn_protection_friendly_killed_time_mut(&mut self) -> &mut f32 {
        self._glacier_base.spawn_protection_friendly_killed_time_mut()
    }
    fn clear_bangers_on_spawn(&self) -> &bool {
        self._glacier_base.clear_bangers_on_spawn()
    }
    fn clear_bangers_on_spawn_mut(&mut self) -> &mut bool {
        self._glacier_base.clear_bangers_on_spawn_mut()
    }
    fn try_to_spawn_out_of_sight(&self) -> &bool {
        self._glacier_base.try_to_spawn_out_of_sight()
    }
    fn try_to_spawn_out_of_sight_mut(&mut self) -> &mut bool {
        self._glacier_base.try_to_spawn_out_of_sight_mut()
    }
    fn send_weapon_events(&self) -> &bool {
        self._glacier_base.send_weapon_events()
    }
    fn send_weapon_events_mut(&mut self) -> &mut bool {
        self._glacier_base.send_weapon_events_mut()
    }
    fn take_control_on_transform_change(&self) -> &bool {
        self._glacier_base.take_control_on_transform_change()
    }
    fn take_control_on_transform_change_mut(&mut self) -> &mut bool {
        self._glacier_base.take_control_on_transform_change_mut()
    }
    fn return_control_on_idle(&self) -> &bool {
        self._glacier_base.return_control_on_idle()
    }
    fn return_control_on_idle_mut(&mut self) -> &mut bool {
        self._glacier_base.return_control_on_idle_mut()
    }
    fn take_control_entry_index(&self) -> &i32 {
        self._glacier_base.take_control_entry_index()
    }
    fn take_control_entry_index_mut(&mut self) -> &mut i32 {
        self._glacier_base.take_control_entry_index_mut()
    }
    fn rotation_yaw(&self) -> &f32 {
        self._glacier_base.rotation_yaw()
    }
    fn rotation_yaw_mut(&mut self) -> &mut f32 {
        self._glacier_base.rotation_yaw_mut()
    }
    fn rotation_pitch(&self) -> &f32 {
        self._glacier_base.rotation_pitch()
    }
    fn rotation_pitch_mut(&mut self) -> &mut f32 {
        self._glacier_base.rotation_pitch_mut()
    }
    fn rotation_roll(&self) -> &f32 {
        self._glacier_base.rotation_roll()
    }
    fn rotation_roll_mut(&mut self) -> &mut f32 {
        self._glacier_base.rotation_roll_mut()
    }
    fn throttle(&self) -> &f32 {
        self._glacier_base.throttle()
    }
    fn throttle_mut(&mut self) -> &mut f32 {
        self._glacier_base.throttle_mut()
    }
    fn time_delta_type(&self) -> &super::entity::TimeDeltaType {
        self._glacier_base.time_delta_type()
    }
    fn time_delta_type_mut(&mut self) -> &mut super::entity::TimeDeltaType {
        self._glacier_base.time_delta_type_mut()
    }
    fn max_unspawn_in_frame(&self) -> &i32 {
        self._glacier_base.max_unspawn_in_frame()
    }
    fn max_unspawn_in_frame_mut(&mut self) -> &mut i32 {
        self._glacier_base.max_unspawn_in_frame_mut()
    }
}

impl super::gameplay_sim::GameplaySpawnReferenceObjectDataTrait for CharacterDefinitionSpawnData {
    fn extra_spawn_data(&self) -> &Vec<Option<LockedTypeObject /* super::gameplay_sim::ExtraSpawnData */>> {
        self._glacier_base.extra_spawn_data()
    }
    fn extra_spawn_data_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::gameplay_sim::ExtraSpawnData */>> {
        self._glacier_base.extra_spawn_data_mut()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
    fn location_name_sid(&self) -> &String {
        self._glacier_base.location_name_sid()
    }
    fn location_name_sid_mut(&mut self) -> &mut String {
        self._glacier_base.location_name_sid_mut()
    }
    fn location_text_sid(&self) -> &String {
        self._glacier_base.location_text_sid()
    }
    fn location_text_sid_mut(&mut self) -> &mut String {
        self._glacier_base.location_text_sid_mut()
    }
    fn initial_auto_spawn(&self) -> &bool {
        self._glacier_base.initial_auto_spawn()
    }
    fn initial_auto_spawn_mut(&mut self) -> &mut bool {
        self._glacier_base.initial_auto_spawn_mut()
    }
    fn auto_spawn(&self) -> &bool {
        self._glacier_base.auto_spawn()
    }
    fn auto_spawn_mut(&mut self) -> &mut bool {
        self._glacier_base.auto_spawn_mut()
    }
    fn queue_spawn_event(&self) -> &bool {
        self._glacier_base.queue_spawn_event()
    }
    fn queue_spawn_event_mut(&mut self) -> &mut bool {
        self._glacier_base.queue_spawn_event_mut()
    }
    fn use_as_spawn_point(&self) -> &bool {
        self._glacier_base.use_as_spawn_point()
    }
    fn use_as_spawn_point_mut(&mut self) -> &mut bool {
        self._glacier_base.use_as_spawn_point_mut()
    }
    fn initial_spawn_delay(&self) -> &f32 {
        self._glacier_base.initial_spawn_delay()
    }
    fn initial_spawn_delay_mut(&mut self) -> &mut f32 {
        self._glacier_base.initial_spawn_delay_mut()
    }
    fn spawn_delay(&self) -> &f32 {
        self._glacier_base.spawn_delay()
    }
    fn spawn_delay_mut(&mut self) -> &mut f32 {
        self._glacier_base.spawn_delay_mut()
    }
    fn max_count(&self) -> &i32 {
        self._glacier_base.max_count()
    }
    fn max_count_mut(&mut self) -> &mut i32 {
        self._glacier_base.max_count_mut()
    }
    fn max_count_simultaneously(&self) -> &i32 {
        self._glacier_base.max_count_simultaneously()
    }
    fn max_count_simultaneously_mut(&mut self) -> &mut i32 {
        self._glacier_base.max_count_simultaneously_mut()
    }
    fn total_count_simultaneously_of_type(&self) -> &i32 {
        self._glacier_base.total_count_simultaneously_of_type()
    }
    fn total_count_simultaneously_of_type_mut(&mut self) -> &mut i32 {
        self._glacier_base.total_count_simultaneously_of_type_mut()
    }
    fn max_spawn_in_frame(&self) -> &i32 {
        self._glacier_base.max_spawn_in_frame()
    }
    fn max_spawn_in_frame_mut(&mut self) -> &mut i32 {
        self._glacier_base.max_spawn_in_frame_mut()
    }
    fn only_send_event_for_human_players(&self) -> &bool {
        self._glacier_base.only_send_event_for_human_players()
    }
    fn only_send_event_for_human_players_mut(&mut self) -> &mut bool {
        self._glacier_base.only_send_event_for_human_players_mut()
    }
    fn controllable_transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.controllable_transform()
    }
    fn controllable_transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.controllable_transform_mut()
    }
    fn controllable_input(&self) -> &super::core::LinearTransform {
        self._glacier_base.controllable_input()
    }
    fn controllable_input_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.controllable_input_mut()
    }
}

impl super::entity::SpatialReferenceObjectDataTrait for CharacterDefinitionSpawnData {
    fn local_player_id(&self) -> &super::core::LocalPlayerId {
        self._glacier_base.local_player_id()
    }
    fn local_player_id_mut(&mut self) -> &mut super::core::LocalPlayerId {
        self._glacier_base.local_player_id_mut()
    }
}

impl super::entity::ReferenceObjectDataTrait for CharacterDefinitionSpawnData {
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

impl super::entity::GameObjectDataTrait for CharacterDefinitionSpawnData {
}

impl super::core::DataBusPeerTrait for CharacterDefinitionSpawnData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for CharacterDefinitionSpawnData {
}

impl super::core::DataContainerTrait for CharacterDefinitionSpawnData {
}

pub static CHARACTERDEFINITIONSPAWNDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterDefinitionSpawnData",
    name_hash: 744430210,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_shared::CHARACTERSPAWNREFERENCEOBJECTDATA_TYPE_INFO),
        super_class_offset: offset_of!(CharacterDefinitionSpawnData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CharacterDefinitionSpawnData as Default>::default())),
            create_boxed: || Box::new(<CharacterDefinitionSpawnData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "CharacterDefinition",
                name_hash: 1778073705,
                flags: MemberInfoFlags::new(0),
                field_type: "CharacterDefinition",
                rust_offset: offset_of!(CharacterDefinitionSpawnData, character_definition),
            },
        ],
    }),
    array_type: Some(CHARACTERDEFINITIONSPAWNDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CharacterDefinitionSpawnData {
    fn type_info(&self) -> &'static TypeInfo {
        CHARACTERDEFINITIONSPAWNDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CHARACTERDEFINITIONSPAWNDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterDefinitionSpawnData-Array",
    name_hash: 2637420982,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("CharacterDefinitionSpawnData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CharacterDefinition {
    pub _glacier_base: super::core::DataContainerPolicyAsset,
    pub character_blueprint: Option<LockedTypeObject /* super::game_shared::CharacterBlueprint */>,
    pub face_poser_library: super::ant::AntRef,
    pub meshes: Vec<Option<LockedTypeObject /* CharacterDefinitionMesh */>>,
}

pub trait CharacterDefinitionTrait: super::core::DataContainerPolicyAssetTrait {
    fn character_blueprint(&self) -> &Option<LockedTypeObject /* super::game_shared::CharacterBlueprint */>;
    fn character_blueprint_mut(&mut self) -> &mut Option<LockedTypeObject /* super::game_shared::CharacterBlueprint */>;
    fn face_poser_library(&self) -> &super::ant::AntRef;
    fn face_poser_library_mut(&mut self) -> &mut super::ant::AntRef;
    fn meshes(&self) -> &Vec<Option<LockedTypeObject /* CharacterDefinitionMesh */>>;
    fn meshes_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* CharacterDefinitionMesh */>>;
}

impl CharacterDefinitionTrait for CharacterDefinition {
    fn character_blueprint(&self) -> &Option<LockedTypeObject /* super::game_shared::CharacterBlueprint */> {
        &self.character_blueprint
    }
    fn character_blueprint_mut(&mut self) -> &mut Option<LockedTypeObject /* super::game_shared::CharacterBlueprint */> {
        &mut self.character_blueprint
    }
    fn face_poser_library(&self) -> &super::ant::AntRef {
        &self.face_poser_library
    }
    fn face_poser_library_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.face_poser_library
    }
    fn meshes(&self) -> &Vec<Option<LockedTypeObject /* CharacterDefinitionMesh */>> {
        &self.meshes
    }
    fn meshes_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* CharacterDefinitionMesh */>> {
        &mut self.meshes
    }
}

impl super::core::DataContainerPolicyAssetTrait for CharacterDefinition {
}

impl super::core::AssetTrait for CharacterDefinition {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for CharacterDefinition {
}

pub static CHARACTERDEFINITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterDefinition",
    name_hash: 1778073705,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINERPOLICYASSET_TYPE_INFO),
        super_class_offset: offset_of!(CharacterDefinition, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CharacterDefinition as Default>::default())),
            create_boxed: || Box::new(<CharacterDefinition as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "CharacterBlueprint",
                name_hash: 1806128211,
                flags: MemberInfoFlags::new(0),
                field_type: "CharacterBlueprint",
                rust_offset: offset_of!(CharacterDefinition, character_blueprint),
            },
            FieldInfoData {
                name: "FacePoserLibrary",
                name_hash: 2878950592,
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(CharacterDefinition, face_poser_library),
            },
            FieldInfoData {
                name: "Meshes",
                name_hash: 2648066496,
                flags: MemberInfoFlags::new(144),
                field_type: "CharacterDefinitionMesh-Array",
                rust_offset: offset_of!(CharacterDefinition, meshes),
            },
        ],
    }),
    array_type: Some(CHARACTERDEFINITION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CharacterDefinition {
    fn type_info(&self) -> &'static TypeInfo {
        CHARACTERDEFINITION_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CHARACTERDEFINITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterDefinition-Array",
    name_hash: 534730589,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("CharacterDefinition"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CharacterDefinitionMesh {
    pub _glacier_base: super::core::DataContainer,
    pub guid: glacier_util::guid::Guid,
    pub object_tag: String,
    pub mesh_asset: Option<LockedTypeObject /* super::render_base::MeshBaseAsset */>,
    pub attach_to_joint: String,
    pub attach_offset: super::core::LinearTransform,
    pub visible: bool,
}

pub trait CharacterDefinitionMeshTrait: super::core::DataContainerTrait {
    fn guid(&self) -> &glacier_util::guid::Guid;
    fn guid_mut(&mut self) -> &mut glacier_util::guid::Guid;
    fn object_tag(&self) -> &String;
    fn object_tag_mut(&mut self) -> &mut String;
    fn mesh_asset(&self) -> &Option<LockedTypeObject /* super::render_base::MeshBaseAsset */>;
    fn mesh_asset_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::MeshBaseAsset */>;
    fn attach_to_joint(&self) -> &String;
    fn attach_to_joint_mut(&mut self) -> &mut String;
    fn attach_offset(&self) -> &super::core::LinearTransform;
    fn attach_offset_mut(&mut self) -> &mut super::core::LinearTransform;
    fn visible(&self) -> &bool;
    fn visible_mut(&mut self) -> &mut bool;
}

impl CharacterDefinitionMeshTrait for CharacterDefinitionMesh {
    fn guid(&self) -> &glacier_util::guid::Guid {
        &self.guid
    }
    fn guid_mut(&mut self) -> &mut glacier_util::guid::Guid {
        &mut self.guid
    }
    fn object_tag(&self) -> &String {
        &self.object_tag
    }
    fn object_tag_mut(&mut self) -> &mut String {
        &mut self.object_tag
    }
    fn mesh_asset(&self) -> &Option<LockedTypeObject /* super::render_base::MeshBaseAsset */> {
        &self.mesh_asset
    }
    fn mesh_asset_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::MeshBaseAsset */> {
        &mut self.mesh_asset
    }
    fn attach_to_joint(&self) -> &String {
        &self.attach_to_joint
    }
    fn attach_to_joint_mut(&mut self) -> &mut String {
        &mut self.attach_to_joint
    }
    fn attach_offset(&self) -> &super::core::LinearTransform {
        &self.attach_offset
    }
    fn attach_offset_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.attach_offset
    }
    fn visible(&self) -> &bool {
        &self.visible
    }
    fn visible_mut(&mut self) -> &mut bool {
        &mut self.visible
    }
}

impl super::core::DataContainerTrait for CharacterDefinitionMesh {
}

pub static CHARACTERDEFINITIONMESH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterDefinitionMesh",
    name_hash: 2099712026,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(CharacterDefinitionMesh, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CharacterDefinitionMesh as Default>::default())),
            create_boxed: || Box::new(<CharacterDefinitionMesh as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Guid",
                name_hash: 2088724858,
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(CharacterDefinitionMesh, guid),
            },
            FieldInfoData {
                name: "ObjectTag",
                name_hash: 3207461890,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(CharacterDefinitionMesh, object_tag),
            },
            FieldInfoData {
                name: "MeshAsset",
                name_hash: 15738982,
                flags: MemberInfoFlags::new(0),
                field_type: "MeshBaseAsset",
                rust_offset: offset_of!(CharacterDefinitionMesh, mesh_asset),
            },
            FieldInfoData {
                name: "AttachToJoint",
                name_hash: 1013700163,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(CharacterDefinitionMesh, attach_to_joint),
            },
            FieldInfoData {
                name: "AttachOffset",
                name_hash: 4087467523,
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(CharacterDefinitionMesh, attach_offset),
            },
            FieldInfoData {
                name: "Visible",
                name_hash: 901540267,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CharacterDefinitionMesh, visible),
            },
        ],
    }),
    array_type: Some(CHARACTERDEFINITIONMESH_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CharacterDefinitionMesh {
    fn type_info(&self) -> &'static TypeInfo {
        CHARACTERDEFINITIONMESH_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CHARACTERDEFINITIONMESH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterDefinitionMesh-Array",
    name_hash: 848302894,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("CharacterDefinitionMesh"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DiceAudioSettings {
    pub _glacier_base: super::core::SystemSettings,
    pub obstruction_max_queries_per_frame: u32,
    pub obstruction_query_stage_threshold: f32,
    pub obstruction_max_obstruction: f32,
    pub obstruction_max_obstruction_distance: f32,
    pub obstruction_relative_velocity_threshold: f32,
    pub obstruction_max_inactive_time: f32,
    pub obstruction_use_radius_angle_as_obstruction_value: bool,
    pub obstruction_multi_stage_raycasts_enabled: bool,
    pub obstruction_multi_stage_raycasts_outer_distance: f32,
    pub obstruction_multi_stage_raycasts_second_stage_scalar: f32,
    pub obstruction_multi_stage_raycasts_attack_speed: f32,
    pub obstruction_multi_stage_raycasts_release_speed: f32,
    pub obstruction_multi_stage_raycasts_first_stage_angle: f32,
    pub obstruction_multi_stage_raycasts_max_obstruction: f32,
}

pub trait DiceAudioSettingsTrait: super::core::SystemSettingsTrait {
    fn obstruction_max_queries_per_frame(&self) -> &u32;
    fn obstruction_max_queries_per_frame_mut(&mut self) -> &mut u32;
    fn obstruction_query_stage_threshold(&self) -> &f32;
    fn obstruction_query_stage_threshold_mut(&mut self) -> &mut f32;
    fn obstruction_max_obstruction(&self) -> &f32;
    fn obstruction_max_obstruction_mut(&mut self) -> &mut f32;
    fn obstruction_max_obstruction_distance(&self) -> &f32;
    fn obstruction_max_obstruction_distance_mut(&mut self) -> &mut f32;
    fn obstruction_relative_velocity_threshold(&self) -> &f32;
    fn obstruction_relative_velocity_threshold_mut(&mut self) -> &mut f32;
    fn obstruction_max_inactive_time(&self) -> &f32;
    fn obstruction_max_inactive_time_mut(&mut self) -> &mut f32;
    fn obstruction_use_radius_angle_as_obstruction_value(&self) -> &bool;
    fn obstruction_use_radius_angle_as_obstruction_value_mut(&mut self) -> &mut bool;
    fn obstruction_multi_stage_raycasts_enabled(&self) -> &bool;
    fn obstruction_multi_stage_raycasts_enabled_mut(&mut self) -> &mut bool;
    fn obstruction_multi_stage_raycasts_outer_distance(&self) -> &f32;
    fn obstruction_multi_stage_raycasts_outer_distance_mut(&mut self) -> &mut f32;
    fn obstruction_multi_stage_raycasts_second_stage_scalar(&self) -> &f32;
    fn obstruction_multi_stage_raycasts_second_stage_scalar_mut(&mut self) -> &mut f32;
    fn obstruction_multi_stage_raycasts_attack_speed(&self) -> &f32;
    fn obstruction_multi_stage_raycasts_attack_speed_mut(&mut self) -> &mut f32;
    fn obstruction_multi_stage_raycasts_release_speed(&self) -> &f32;
    fn obstruction_multi_stage_raycasts_release_speed_mut(&mut self) -> &mut f32;
    fn obstruction_multi_stage_raycasts_first_stage_angle(&self) -> &f32;
    fn obstruction_multi_stage_raycasts_first_stage_angle_mut(&mut self) -> &mut f32;
    fn obstruction_multi_stage_raycasts_max_obstruction(&self) -> &f32;
    fn obstruction_multi_stage_raycasts_max_obstruction_mut(&mut self) -> &mut f32;
}

impl DiceAudioSettingsTrait for DiceAudioSettings {
    fn obstruction_max_queries_per_frame(&self) -> &u32 {
        &self.obstruction_max_queries_per_frame
    }
    fn obstruction_max_queries_per_frame_mut(&mut self) -> &mut u32 {
        &mut self.obstruction_max_queries_per_frame
    }
    fn obstruction_query_stage_threshold(&self) -> &f32 {
        &self.obstruction_query_stage_threshold
    }
    fn obstruction_query_stage_threshold_mut(&mut self) -> &mut f32 {
        &mut self.obstruction_query_stage_threshold
    }
    fn obstruction_max_obstruction(&self) -> &f32 {
        &self.obstruction_max_obstruction
    }
    fn obstruction_max_obstruction_mut(&mut self) -> &mut f32 {
        &mut self.obstruction_max_obstruction
    }
    fn obstruction_max_obstruction_distance(&self) -> &f32 {
        &self.obstruction_max_obstruction_distance
    }
    fn obstruction_max_obstruction_distance_mut(&mut self) -> &mut f32 {
        &mut self.obstruction_max_obstruction_distance
    }
    fn obstruction_relative_velocity_threshold(&self) -> &f32 {
        &self.obstruction_relative_velocity_threshold
    }
    fn obstruction_relative_velocity_threshold_mut(&mut self) -> &mut f32 {
        &mut self.obstruction_relative_velocity_threshold
    }
    fn obstruction_max_inactive_time(&self) -> &f32 {
        &self.obstruction_max_inactive_time
    }
    fn obstruction_max_inactive_time_mut(&mut self) -> &mut f32 {
        &mut self.obstruction_max_inactive_time
    }
    fn obstruction_use_radius_angle_as_obstruction_value(&self) -> &bool {
        &self.obstruction_use_radius_angle_as_obstruction_value
    }
    fn obstruction_use_radius_angle_as_obstruction_value_mut(&mut self) -> &mut bool {
        &mut self.obstruction_use_radius_angle_as_obstruction_value
    }
    fn obstruction_multi_stage_raycasts_enabled(&self) -> &bool {
        &self.obstruction_multi_stage_raycasts_enabled
    }
    fn obstruction_multi_stage_raycasts_enabled_mut(&mut self) -> &mut bool {
        &mut self.obstruction_multi_stage_raycasts_enabled
    }
    fn obstruction_multi_stage_raycasts_outer_distance(&self) -> &f32 {
        &self.obstruction_multi_stage_raycasts_outer_distance
    }
    fn obstruction_multi_stage_raycasts_outer_distance_mut(&mut self) -> &mut f32 {
        &mut self.obstruction_multi_stage_raycasts_outer_distance
    }
    fn obstruction_multi_stage_raycasts_second_stage_scalar(&self) -> &f32 {
        &self.obstruction_multi_stage_raycasts_second_stage_scalar
    }
    fn obstruction_multi_stage_raycasts_second_stage_scalar_mut(&mut self) -> &mut f32 {
        &mut self.obstruction_multi_stage_raycasts_second_stage_scalar
    }
    fn obstruction_multi_stage_raycasts_attack_speed(&self) -> &f32 {
        &self.obstruction_multi_stage_raycasts_attack_speed
    }
    fn obstruction_multi_stage_raycasts_attack_speed_mut(&mut self) -> &mut f32 {
        &mut self.obstruction_multi_stage_raycasts_attack_speed
    }
    fn obstruction_multi_stage_raycasts_release_speed(&self) -> &f32 {
        &self.obstruction_multi_stage_raycasts_release_speed
    }
    fn obstruction_multi_stage_raycasts_release_speed_mut(&mut self) -> &mut f32 {
        &mut self.obstruction_multi_stage_raycasts_release_speed
    }
    fn obstruction_multi_stage_raycasts_first_stage_angle(&self) -> &f32 {
        &self.obstruction_multi_stage_raycasts_first_stage_angle
    }
    fn obstruction_multi_stage_raycasts_first_stage_angle_mut(&mut self) -> &mut f32 {
        &mut self.obstruction_multi_stage_raycasts_first_stage_angle
    }
    fn obstruction_multi_stage_raycasts_max_obstruction(&self) -> &f32 {
        &self.obstruction_multi_stage_raycasts_max_obstruction
    }
    fn obstruction_multi_stage_raycasts_max_obstruction_mut(&mut self) -> &mut f32 {
        &mut self.obstruction_multi_stage_raycasts_max_obstruction
    }
}

impl super::core::SystemSettingsTrait for DiceAudioSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl super::core::DataContainerTrait for DiceAudioSettings {
}

pub static DICEAUDIOSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceAudioSettings",
    name_hash: 1333680029,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        super_class_offset: offset_of!(DiceAudioSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DiceAudioSettings as Default>::default())),
            create_boxed: || Box::new(<DiceAudioSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ObstructionMaxQueriesPerFrame",
                name_hash: 2517616949,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DiceAudioSettings, obstruction_max_queries_per_frame),
            },
            FieldInfoData {
                name: "ObstructionQueryStageThreshold",
                name_hash: 2970800622,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DiceAudioSettings, obstruction_query_stage_threshold),
            },
            FieldInfoData {
                name: "ObstructionMaxObstruction",
                name_hash: 3451241617,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DiceAudioSettings, obstruction_max_obstruction),
            },
            FieldInfoData {
                name: "ObstructionMaxObstructionDistance",
                name_hash: 2843254354,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DiceAudioSettings, obstruction_max_obstruction_distance),
            },
            FieldInfoData {
                name: "ObstructionRelativeVelocityThreshold",
                name_hash: 3918973603,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DiceAudioSettings, obstruction_relative_velocity_threshold),
            },
            FieldInfoData {
                name: "ObstructionMaxInactiveTime",
                name_hash: 1092294557,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DiceAudioSettings, obstruction_max_inactive_time),
            },
            FieldInfoData {
                name: "ObstructionUseRadiusAngleAsObstructionValue",
                name_hash: 1949904102,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DiceAudioSettings, obstruction_use_radius_angle_as_obstruction_value),
            },
            FieldInfoData {
                name: "ObstructionMultiStageRaycastsEnabled",
                name_hash: 3057402371,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DiceAudioSettings, obstruction_multi_stage_raycasts_enabled),
            },
            FieldInfoData {
                name: "ObstructionMultiStageRaycastsOuterDistance",
                name_hash: 2297805116,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DiceAudioSettings, obstruction_multi_stage_raycasts_outer_distance),
            },
            FieldInfoData {
                name: "ObstructionMultiStageRaycastsSecondStageScalar",
                name_hash: 3038712764,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DiceAudioSettings, obstruction_multi_stage_raycasts_second_stage_scalar),
            },
            FieldInfoData {
                name: "ObstructionMultiStageRaycastsAttackSpeed",
                name_hash: 60475369,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DiceAudioSettings, obstruction_multi_stage_raycasts_attack_speed),
            },
            FieldInfoData {
                name: "ObstructionMultiStageRaycastsReleaseSpeed",
                name_hash: 2712638536,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DiceAudioSettings, obstruction_multi_stage_raycasts_release_speed),
            },
            FieldInfoData {
                name: "ObstructionMultiStageRaycastsFirstStageAngle",
                name_hash: 257111897,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DiceAudioSettings, obstruction_multi_stage_raycasts_first_stage_angle),
            },
            FieldInfoData {
                name: "ObstructionMultiStageRaycastsMaxObstruction",
                name_hash: 2820832576,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DiceAudioSettings, obstruction_multi_stage_raycasts_max_obstruction),
            },
        ],
    }),
    array_type: Some(DICEAUDIOSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DiceAudioSettings {
    fn type_info(&self) -> &'static TypeInfo {
        DICEAUDIOSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DICEAUDIOSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceAudioSettings-Array",
    name_hash: 1367163689,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceAudioSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DistanceScopeStageData {
    pub _glacier_base: super::audio::SoundScopeStageData,
    pub distance: f32,
}

pub trait DistanceScopeStageDataTrait: super::audio::SoundScopeStageDataTrait {
    fn distance(&self) -> &f32;
    fn distance_mut(&mut self) -> &mut f32;
}

impl DistanceScopeStageDataTrait for DistanceScopeStageData {
    fn distance(&self) -> &f32 {
        &self.distance
    }
    fn distance_mut(&mut self) -> &mut f32 {
        &mut self.distance
    }
}

impl super::audio::SoundScopeStageDataTrait for DistanceScopeStageData {
}

impl super::core::DataContainerTrait for DistanceScopeStageData {
}

pub static DISTANCESCOPESTAGEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistanceScopeStageData",
    name_hash: 2703334584,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::SOUNDSCOPESTAGEDATA_TYPE_INFO),
        super_class_offset: offset_of!(DistanceScopeStageData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DistanceScopeStageData as Default>::default())),
            create_boxed: || Box::new(<DistanceScopeStageData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Distance",
                name_hash: 408560070,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DistanceScopeStageData, distance),
            },
        ],
    }),
    array_type: Some(DISTANCESCOPESTAGEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DistanceScopeStageData {
    fn type_info(&self) -> &'static TypeInfo {
        DISTANCESCOPESTAGEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DISTANCESCOPESTAGEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistanceScopeStageData-Array",
    name_hash: 3109365260,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DistanceScopeStageData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ComboScopeStageData {
    pub _glacier_base: super::audio::SoundScopeStageData,
    pub newest_count: u32,
    pub newest_threshold: f32,
    pub closest_count: u32,
}

pub trait ComboScopeStageDataTrait: super::audio::SoundScopeStageDataTrait {
    fn newest_count(&self) -> &u32;
    fn newest_count_mut(&mut self) -> &mut u32;
    fn newest_threshold(&self) -> &f32;
    fn newest_threshold_mut(&mut self) -> &mut f32;
    fn closest_count(&self) -> &u32;
    fn closest_count_mut(&mut self) -> &mut u32;
}

impl ComboScopeStageDataTrait for ComboScopeStageData {
    fn newest_count(&self) -> &u32 {
        &self.newest_count
    }
    fn newest_count_mut(&mut self) -> &mut u32 {
        &mut self.newest_count
    }
    fn newest_threshold(&self) -> &f32 {
        &self.newest_threshold
    }
    fn newest_threshold_mut(&mut self) -> &mut f32 {
        &mut self.newest_threshold
    }
    fn closest_count(&self) -> &u32 {
        &self.closest_count
    }
    fn closest_count_mut(&mut self) -> &mut u32 {
        &mut self.closest_count
    }
}

impl super::audio::SoundScopeStageDataTrait for ComboScopeStageData {
}

impl super::core::DataContainerTrait for ComboScopeStageData {
}

pub static COMBOSCOPESTAGEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ComboScopeStageData",
    name_hash: 3578339511,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::SOUNDSCOPESTAGEDATA_TYPE_INFO),
        super_class_offset: offset_of!(ComboScopeStageData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ComboScopeStageData as Default>::default())),
            create_boxed: || Box::new(<ComboScopeStageData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "NewestCount",
                name_hash: 4238783608,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ComboScopeStageData, newest_count),
            },
            FieldInfoData {
                name: "NewestThreshold",
                name_hash: 645237420,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ComboScopeStageData, newest_threshold),
            },
            FieldInfoData {
                name: "ClosestCount",
                name_hash: 4143870103,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ComboScopeStageData, closest_count),
            },
        ],
    }),
    array_type: Some(COMBOSCOPESTAGEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ComboScopeStageData {
    fn type_info(&self) -> &'static TypeInfo {
        COMBOSCOPESTAGEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static COMBOSCOPESTAGEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ComboScopeStageData-Array",
    name_hash: 2212477443,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ComboScopeStageData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AngularScopeStageData {
    pub _glacier_base: super::audio::SoundScopeStageData,
    pub inner_angle: f32,
    pub outer_angle: f32,
    pub scalling_factor: f32,
    pub count: u32,
}

pub trait AngularScopeStageDataTrait: super::audio::SoundScopeStageDataTrait {
    fn inner_angle(&self) -> &f32;
    fn inner_angle_mut(&mut self) -> &mut f32;
    fn outer_angle(&self) -> &f32;
    fn outer_angle_mut(&mut self) -> &mut f32;
    fn scalling_factor(&self) -> &f32;
    fn scalling_factor_mut(&mut self) -> &mut f32;
    fn count(&self) -> &u32;
    fn count_mut(&mut self) -> &mut u32;
}

impl AngularScopeStageDataTrait for AngularScopeStageData {
    fn inner_angle(&self) -> &f32 {
        &self.inner_angle
    }
    fn inner_angle_mut(&mut self) -> &mut f32 {
        &mut self.inner_angle
    }
    fn outer_angle(&self) -> &f32 {
        &self.outer_angle
    }
    fn outer_angle_mut(&mut self) -> &mut f32 {
        &mut self.outer_angle
    }
    fn scalling_factor(&self) -> &f32 {
        &self.scalling_factor
    }
    fn scalling_factor_mut(&mut self) -> &mut f32 {
        &mut self.scalling_factor
    }
    fn count(&self) -> &u32 {
        &self.count
    }
    fn count_mut(&mut self) -> &mut u32 {
        &mut self.count
    }
}

impl super::audio::SoundScopeStageDataTrait for AngularScopeStageData {
}

impl super::core::DataContainerTrait for AngularScopeStageData {
}

pub static ANGULARSCOPESTAGEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngularScopeStageData",
    name_hash: 265677017,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::SOUNDSCOPESTAGEDATA_TYPE_INFO),
        super_class_offset: offset_of!(AngularScopeStageData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AngularScopeStageData as Default>::default())),
            create_boxed: || Box::new(<AngularScopeStageData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "InnerAngle",
                name_hash: 3372545274,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AngularScopeStageData, inner_angle),
            },
            FieldInfoData {
                name: "OuterAngle",
                name_hash: 4288979453,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AngularScopeStageData, outer_angle),
            },
            FieldInfoData {
                name: "ScallingFactor",
                name_hash: 3321416025,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AngularScopeStageData, scalling_factor),
            },
            FieldInfoData {
                name: "Count",
                name_hash: 212413894,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(AngularScopeStageData, count),
            },
        ],
    }),
    array_type: Some(ANGULARSCOPESTAGEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AngularScopeStageData {
    fn type_info(&self) -> &'static TypeInfo {
        ANGULARSCOPESTAGEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static ANGULARSCOPESTAGEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngularScopeStageData-Array",
    name_hash: 2282297069,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("AngularScopeStageData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WhooshbyPlayerEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub whooshby_closing_sound: Option<LockedTypeObject /* super::audio::SoundAsset */>,
    pub whooshby_trigger_closing_distance_threshold: f32,
    pub whooshby_separating_sound: Option<LockedTypeObject /* super::audio::SoundAsset */>,
    pub whooshby_trigger_separating_distance_threshold: f32,
}

pub trait WhooshbyPlayerEntityDataTrait: super::entity::EntityDataTrait {
    fn whooshby_closing_sound(&self) -> &Option<LockedTypeObject /* super::audio::SoundAsset */>;
    fn whooshby_closing_sound_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundAsset */>;
    fn whooshby_trigger_closing_distance_threshold(&self) -> &f32;
    fn whooshby_trigger_closing_distance_threshold_mut(&mut self) -> &mut f32;
    fn whooshby_separating_sound(&self) -> &Option<LockedTypeObject /* super::audio::SoundAsset */>;
    fn whooshby_separating_sound_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundAsset */>;
    fn whooshby_trigger_separating_distance_threshold(&self) -> &f32;
    fn whooshby_trigger_separating_distance_threshold_mut(&mut self) -> &mut f32;
}

impl WhooshbyPlayerEntityDataTrait for WhooshbyPlayerEntityData {
    fn whooshby_closing_sound(&self) -> &Option<LockedTypeObject /* super::audio::SoundAsset */> {
        &self.whooshby_closing_sound
    }
    fn whooshby_closing_sound_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundAsset */> {
        &mut self.whooshby_closing_sound
    }
    fn whooshby_trigger_closing_distance_threshold(&self) -> &f32 {
        &self.whooshby_trigger_closing_distance_threshold
    }
    fn whooshby_trigger_closing_distance_threshold_mut(&mut self) -> &mut f32 {
        &mut self.whooshby_trigger_closing_distance_threshold
    }
    fn whooshby_separating_sound(&self) -> &Option<LockedTypeObject /* super::audio::SoundAsset */> {
        &self.whooshby_separating_sound
    }
    fn whooshby_separating_sound_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundAsset */> {
        &mut self.whooshby_separating_sound
    }
    fn whooshby_trigger_separating_distance_threshold(&self) -> &f32 {
        &self.whooshby_trigger_separating_distance_threshold
    }
    fn whooshby_trigger_separating_distance_threshold_mut(&mut self) -> &mut f32 {
        &mut self.whooshby_trigger_separating_distance_threshold
    }
}

impl super::entity::EntityDataTrait for WhooshbyPlayerEntityData {
}

impl super::entity::GameObjectDataTrait for WhooshbyPlayerEntityData {
}

impl super::core::DataBusPeerTrait for WhooshbyPlayerEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for WhooshbyPlayerEntityData {
}

impl super::core::DataContainerTrait for WhooshbyPlayerEntityData {
}

pub static WHOOSHBYPLAYERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WhooshbyPlayerEntityData",
    name_hash: 2493606594,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(WhooshbyPlayerEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WhooshbyPlayerEntityData as Default>::default())),
            create_boxed: || Box::new(<WhooshbyPlayerEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "WhooshbyClosingSound",
                name_hash: 1404201962,
                flags: MemberInfoFlags::new(0),
                field_type: "SoundAsset",
                rust_offset: offset_of!(WhooshbyPlayerEntityData, whooshby_closing_sound),
            },
            FieldInfoData {
                name: "WhooshbyTriggerClosingDistanceThreshold",
                name_hash: 1180011205,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WhooshbyPlayerEntityData, whooshby_trigger_closing_distance_threshold),
            },
            FieldInfoData {
                name: "WhooshbySeparatingSound",
                name_hash: 1070957881,
                flags: MemberInfoFlags::new(0),
                field_type: "SoundAsset",
                rust_offset: offset_of!(WhooshbyPlayerEntityData, whooshby_separating_sound),
            },
            FieldInfoData {
                name: "WhooshbyTriggerSeparatingDistanceThreshold",
                name_hash: 1988125078,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WhooshbyPlayerEntityData, whooshby_trigger_separating_distance_threshold),
            },
        ],
    }),
    array_type: Some(WHOOSHBYPLAYERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WhooshbyPlayerEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        WHOOSHBYPLAYERENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static WHOOSHBYPLAYERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WhooshbyPlayerEntityData-Array",
    name_hash: 701602422,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("WhooshbyPlayerEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AudioProximityReverbEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub reverbs: Vec<BoxedTypeObject /* AudioProximityReverbData */>,
    pub indooriness_attack: f32,
    pub indooriness_decay: f32,
    pub surface_closeness_attack: f32,
    pub surface_closeness_decay: f32,
    pub local_player_id: super::core::LocalPlayerId,
}

pub trait AudioProximityReverbEntityDataTrait: super::entity::EntityDataTrait {
    fn reverbs(&self) -> &Vec<BoxedTypeObject /* AudioProximityReverbData */>;
    fn reverbs_mut(&mut self) -> &mut Vec<BoxedTypeObject /* AudioProximityReverbData */>;
    fn indooriness_attack(&self) -> &f32;
    fn indooriness_attack_mut(&mut self) -> &mut f32;
    fn indooriness_decay(&self) -> &f32;
    fn indooriness_decay_mut(&mut self) -> &mut f32;
    fn surface_closeness_attack(&self) -> &f32;
    fn surface_closeness_attack_mut(&mut self) -> &mut f32;
    fn surface_closeness_decay(&self) -> &f32;
    fn surface_closeness_decay_mut(&mut self) -> &mut f32;
    fn local_player_id(&self) -> &super::core::LocalPlayerId;
    fn local_player_id_mut(&mut self) -> &mut super::core::LocalPlayerId;
}

impl AudioProximityReverbEntityDataTrait for AudioProximityReverbEntityData {
    fn reverbs(&self) -> &Vec<BoxedTypeObject /* AudioProximityReverbData */> {
        &self.reverbs
    }
    fn reverbs_mut(&mut self) -> &mut Vec<BoxedTypeObject /* AudioProximityReverbData */> {
        &mut self.reverbs
    }
    fn indooriness_attack(&self) -> &f32 {
        &self.indooriness_attack
    }
    fn indooriness_attack_mut(&mut self) -> &mut f32 {
        &mut self.indooriness_attack
    }
    fn indooriness_decay(&self) -> &f32 {
        &self.indooriness_decay
    }
    fn indooriness_decay_mut(&mut self) -> &mut f32 {
        &mut self.indooriness_decay
    }
    fn surface_closeness_attack(&self) -> &f32 {
        &self.surface_closeness_attack
    }
    fn surface_closeness_attack_mut(&mut self) -> &mut f32 {
        &mut self.surface_closeness_attack
    }
    fn surface_closeness_decay(&self) -> &f32 {
        &self.surface_closeness_decay
    }
    fn surface_closeness_decay_mut(&mut self) -> &mut f32 {
        &mut self.surface_closeness_decay
    }
    fn local_player_id(&self) -> &super::core::LocalPlayerId {
        &self.local_player_id
    }
    fn local_player_id_mut(&mut self) -> &mut super::core::LocalPlayerId {
        &mut self.local_player_id
    }
}

impl super::entity::EntityDataTrait for AudioProximityReverbEntityData {
}

impl super::entity::GameObjectDataTrait for AudioProximityReverbEntityData {
}

impl super::core::DataBusPeerTrait for AudioProximityReverbEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for AudioProximityReverbEntityData {
}

impl super::core::DataContainerTrait for AudioProximityReverbEntityData {
}

pub static AUDIOPROXIMITYREVERBENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioProximityReverbEntityData",
    name_hash: 41598969,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(AudioProximityReverbEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AudioProximityReverbEntityData as Default>::default())),
            create_boxed: || Box::new(<AudioProximityReverbEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Reverbs",
                name_hash: 1309051938,
                flags: MemberInfoFlags::new(144),
                field_type: "AudioProximityReverbData-Array",
                rust_offset: offset_of!(AudioProximityReverbEntityData, reverbs),
            },
            FieldInfoData {
                name: "IndoorinessAttack",
                name_hash: 842649822,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AudioProximityReverbEntityData, indooriness_attack),
            },
            FieldInfoData {
                name: "IndoorinessDecay",
                name_hash: 3406622636,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AudioProximityReverbEntityData, indooriness_decay),
            },
            FieldInfoData {
                name: "SurfaceClosenessAttack",
                name_hash: 1696478021,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AudioProximityReverbEntityData, surface_closeness_attack),
            },
            FieldInfoData {
                name: "SurfaceClosenessDecay",
                name_hash: 1997571031,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AudioProximityReverbEntityData, surface_closeness_decay),
            },
            FieldInfoData {
                name: "LocalPlayerId",
                name_hash: 1029133718,
                flags: MemberInfoFlags::new(0),
                field_type: "LocalPlayerId",
                rust_offset: offset_of!(AudioProximityReverbEntityData, local_player_id),
            },
        ],
    }),
    array_type: Some(AUDIOPROXIMITYREVERBENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AudioProximityReverbEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        AUDIOPROXIMITYREVERBENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static AUDIOPROXIMITYREVERBENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioProximityReverbEntityData-Array",
    name_hash: 1282621133,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("AudioProximityReverbEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AudioProximityReverbData {
    pub impulse_response: Option<LockedTypeObject /* super::audio::ImpulseResponseAsset */>,
    pub gain: f32,
    pub indooriness_response: Option<LockedTypeObject /* super::core::FloatCurve */>,
    pub surface_closeness_response: Option<LockedTypeObject /* super::core::FloatCurve */>,
}

pub trait AudioProximityReverbDataTrait: TypeObject {
    fn impulse_response(&self) -> &Option<LockedTypeObject /* super::audio::ImpulseResponseAsset */>;
    fn impulse_response_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::ImpulseResponseAsset */>;
    fn gain(&self) -> &f32;
    fn gain_mut(&mut self) -> &mut f32;
    fn indooriness_response(&self) -> &Option<LockedTypeObject /* super::core::FloatCurve */>;
    fn indooriness_response_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::FloatCurve */>;
    fn surface_closeness_response(&self) -> &Option<LockedTypeObject /* super::core::FloatCurve */>;
    fn surface_closeness_response_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::FloatCurve */>;
}

impl AudioProximityReverbDataTrait for AudioProximityReverbData {
    fn impulse_response(&self) -> &Option<LockedTypeObject /* super::audio::ImpulseResponseAsset */> {
        &self.impulse_response
    }
    fn impulse_response_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::ImpulseResponseAsset */> {
        &mut self.impulse_response
    }
    fn gain(&self) -> &f32 {
        &self.gain
    }
    fn gain_mut(&mut self) -> &mut f32 {
        &mut self.gain
    }
    fn indooriness_response(&self) -> &Option<LockedTypeObject /* super::core::FloatCurve */> {
        &self.indooriness_response
    }
    fn indooriness_response_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::FloatCurve */> {
        &mut self.indooriness_response
    }
    fn surface_closeness_response(&self) -> &Option<LockedTypeObject /* super::core::FloatCurve */> {
        &self.surface_closeness_response
    }
    fn surface_closeness_response_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::FloatCurve */> {
        &mut self.surface_closeness_response
    }
}

pub static AUDIOPROXIMITYREVERBDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioProximityReverbData",
    name_hash: 3384744098,
    flags: MemberInfoFlags::new(73),
    module: "DiceCommonsShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AudioProximityReverbData as Default>::default())),
            create_boxed: || Box::new(<AudioProximityReverbData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ImpulseResponse",
                name_hash: 182557405,
                flags: MemberInfoFlags::new(0),
                field_type: "ImpulseResponseAsset",
                rust_offset: offset_of!(AudioProximityReverbData, impulse_response),
            },
            FieldInfoData {
                name: "Gain",
                name_hash: 2088703076,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AudioProximityReverbData, gain),
            },
            FieldInfoData {
                name: "IndoorinessResponse",
                name_hash: 1763378293,
                flags: MemberInfoFlags::new(0),
                field_type: "FloatCurve",
                rust_offset: offset_of!(AudioProximityReverbData, indooriness_response),
            },
            FieldInfoData {
                name: "SurfaceClosenessResponse",
                name_hash: 1466470062,
                flags: MemberInfoFlags::new(0),
                field_type: "FloatCurve",
                rust_offset: offset_of!(AudioProximityReverbData, surface_closeness_response),
            },
        ],
    }),
    array_type: Some(AUDIOPROXIMITYREVERBDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AudioProximityReverbData {
    fn type_info(&self) -> &'static TypeInfo {
        AUDIOPROXIMITYREVERBDATA_TYPE_INFO
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


pub static AUDIOPROXIMITYREVERBDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioProximityReverbData-Array",
    name_hash: 3779458838,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("AudioProximityReverbData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AudioProximityDetectorReaderEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub scaled_enclosedness_min: f32,
    pub scaled_enclosedness_max: f32,
    pub scaled_enclosedness_attack: f32,
    pub scaled_enclosedness_decay: f32,
    pub local_player_id: super::core::LocalPlayerId,
}

pub trait AudioProximityDetectorReaderEntityDataTrait: super::entity::EntityDataTrait {
    fn scaled_enclosedness_min(&self) -> &f32;
    fn scaled_enclosedness_min_mut(&mut self) -> &mut f32;
    fn scaled_enclosedness_max(&self) -> &f32;
    fn scaled_enclosedness_max_mut(&mut self) -> &mut f32;
    fn scaled_enclosedness_attack(&self) -> &f32;
    fn scaled_enclosedness_attack_mut(&mut self) -> &mut f32;
    fn scaled_enclosedness_decay(&self) -> &f32;
    fn scaled_enclosedness_decay_mut(&mut self) -> &mut f32;
    fn local_player_id(&self) -> &super::core::LocalPlayerId;
    fn local_player_id_mut(&mut self) -> &mut super::core::LocalPlayerId;
}

impl AudioProximityDetectorReaderEntityDataTrait for AudioProximityDetectorReaderEntityData {
    fn scaled_enclosedness_min(&self) -> &f32 {
        &self.scaled_enclosedness_min
    }
    fn scaled_enclosedness_min_mut(&mut self) -> &mut f32 {
        &mut self.scaled_enclosedness_min
    }
    fn scaled_enclosedness_max(&self) -> &f32 {
        &self.scaled_enclosedness_max
    }
    fn scaled_enclosedness_max_mut(&mut self) -> &mut f32 {
        &mut self.scaled_enclosedness_max
    }
    fn scaled_enclosedness_attack(&self) -> &f32 {
        &self.scaled_enclosedness_attack
    }
    fn scaled_enclosedness_attack_mut(&mut self) -> &mut f32 {
        &mut self.scaled_enclosedness_attack
    }
    fn scaled_enclosedness_decay(&self) -> &f32 {
        &self.scaled_enclosedness_decay
    }
    fn scaled_enclosedness_decay_mut(&mut self) -> &mut f32 {
        &mut self.scaled_enclosedness_decay
    }
    fn local_player_id(&self) -> &super::core::LocalPlayerId {
        &self.local_player_id
    }
    fn local_player_id_mut(&mut self) -> &mut super::core::LocalPlayerId {
        &mut self.local_player_id
    }
}

impl super::entity::EntityDataTrait for AudioProximityDetectorReaderEntityData {
}

impl super::entity::GameObjectDataTrait for AudioProximityDetectorReaderEntityData {
}

impl super::core::DataBusPeerTrait for AudioProximityDetectorReaderEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for AudioProximityDetectorReaderEntityData {
}

impl super::core::DataContainerTrait for AudioProximityDetectorReaderEntityData {
}

pub static AUDIOPROXIMITYDETECTORREADERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioProximityDetectorReaderEntityData",
    name_hash: 1892294994,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(AudioProximityDetectorReaderEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AudioProximityDetectorReaderEntityData as Default>::default())),
            create_boxed: || Box::new(<AudioProximityDetectorReaderEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ScaledEnclosednessMin",
                name_hash: 1351434113,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AudioProximityDetectorReaderEntityData, scaled_enclosedness_min),
            },
            FieldInfoData {
                name: "ScaledEnclosednessMax",
                name_hash: 1351433887,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AudioProximityDetectorReaderEntityData, scaled_enclosedness_max),
            },
            FieldInfoData {
                name: "ScaledEnclosednessAttack",
                name_hash: 3464067427,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AudioProximityDetectorReaderEntityData, scaled_enclosedness_attack),
            },
            FieldInfoData {
                name: "ScaledEnclosednessDecay",
                name_hash: 2850154353,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AudioProximityDetectorReaderEntityData, scaled_enclosedness_decay),
            },
            FieldInfoData {
                name: "LocalPlayerId",
                name_hash: 1029133718,
                flags: MemberInfoFlags::new(0),
                field_type: "LocalPlayerId",
                rust_offset: offset_of!(AudioProximityDetectorReaderEntityData, local_player_id),
            },
        ],
    }),
    array_type: Some(AUDIOPROXIMITYDETECTORREADERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AudioProximityDetectorReaderEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        AUDIOPROXIMITYDETECTORREADERENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static AUDIOPROXIMITYDETECTORREADERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioProximityDetectorReaderEntityData-Array",
    name_hash: 3084646630,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("AudioProximityDetectorReaderEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AudioProximityDetectorEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub offset: super::core::Vec3,
    pub raycast_radius: f32,
    pub raycast_count: u32,
    pub forward_bias_max_distance: f32,
    pub forward_bias_max_speed: f32,
    pub forward_bias_min_speed: f32,
    pub proximity_type: ProximityDetectorType,
}

pub trait AudioProximityDetectorEntityDataTrait: super::entity::EntityDataTrait {
    fn offset(&self) -> &super::core::Vec3;
    fn offset_mut(&mut self) -> &mut super::core::Vec3;
    fn raycast_radius(&self) -> &f32;
    fn raycast_radius_mut(&mut self) -> &mut f32;
    fn raycast_count(&self) -> &u32;
    fn raycast_count_mut(&mut self) -> &mut u32;
    fn forward_bias_max_distance(&self) -> &f32;
    fn forward_bias_max_distance_mut(&mut self) -> &mut f32;
    fn forward_bias_max_speed(&self) -> &f32;
    fn forward_bias_max_speed_mut(&mut self) -> &mut f32;
    fn forward_bias_min_speed(&self) -> &f32;
    fn forward_bias_min_speed_mut(&mut self) -> &mut f32;
    fn proximity_type(&self) -> &ProximityDetectorType;
    fn proximity_type_mut(&mut self) -> &mut ProximityDetectorType;
}

impl AudioProximityDetectorEntityDataTrait for AudioProximityDetectorEntityData {
    fn offset(&self) -> &super::core::Vec3 {
        &self.offset
    }
    fn offset_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.offset
    }
    fn raycast_radius(&self) -> &f32 {
        &self.raycast_radius
    }
    fn raycast_radius_mut(&mut self) -> &mut f32 {
        &mut self.raycast_radius
    }
    fn raycast_count(&self) -> &u32 {
        &self.raycast_count
    }
    fn raycast_count_mut(&mut self) -> &mut u32 {
        &mut self.raycast_count
    }
    fn forward_bias_max_distance(&self) -> &f32 {
        &self.forward_bias_max_distance
    }
    fn forward_bias_max_distance_mut(&mut self) -> &mut f32 {
        &mut self.forward_bias_max_distance
    }
    fn forward_bias_max_speed(&self) -> &f32 {
        &self.forward_bias_max_speed
    }
    fn forward_bias_max_speed_mut(&mut self) -> &mut f32 {
        &mut self.forward_bias_max_speed
    }
    fn forward_bias_min_speed(&self) -> &f32 {
        &self.forward_bias_min_speed
    }
    fn forward_bias_min_speed_mut(&mut self) -> &mut f32 {
        &mut self.forward_bias_min_speed
    }
    fn proximity_type(&self) -> &ProximityDetectorType {
        &self.proximity_type
    }
    fn proximity_type_mut(&mut self) -> &mut ProximityDetectorType {
        &mut self.proximity_type
    }
}

impl super::entity::EntityDataTrait for AudioProximityDetectorEntityData {
}

impl super::entity::GameObjectDataTrait for AudioProximityDetectorEntityData {
}

impl super::core::DataBusPeerTrait for AudioProximityDetectorEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for AudioProximityDetectorEntityData {
}

impl super::core::DataContainerTrait for AudioProximityDetectorEntityData {
}

pub static AUDIOPROXIMITYDETECTORENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioProximityDetectorEntityData",
    name_hash: 3594558647,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(AudioProximityDetectorEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AudioProximityDetectorEntityData as Default>::default())),
            create_boxed: || Box::new(<AudioProximityDetectorEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Offset",
                name_hash: 2871410728,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AudioProximityDetectorEntityData, offset),
            },
            FieldInfoData {
                name: "RaycastRadius",
                name_hash: 223104402,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AudioProximityDetectorEntityData, raycast_radius),
            },
            FieldInfoData {
                name: "RaycastCount",
                name_hash: 3370456553,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(AudioProximityDetectorEntityData, raycast_count),
            },
            FieldInfoData {
                name: "ForwardBiasMaxDistance",
                name_hash: 3070366448,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AudioProximityDetectorEntityData, forward_bias_max_distance),
            },
            FieldInfoData {
                name: "ForwardBiasMaxSpeed",
                name_hash: 3032848820,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AudioProximityDetectorEntityData, forward_bias_max_speed),
            },
            FieldInfoData {
                name: "ForwardBiasMinSpeed",
                name_hash: 1562782570,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AudioProximityDetectorEntityData, forward_bias_min_speed),
            },
            FieldInfoData {
                name: "ProximityType",
                name_hash: 259289960,
                flags: MemberInfoFlags::new(0),
                field_type: "ProximityDetectorType",
                rust_offset: offset_of!(AudioProximityDetectorEntityData, proximity_type),
            },
        ],
    }),
    array_type: Some(AUDIOPROXIMITYDETECTORENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AudioProximityDetectorEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        AUDIOPROXIMITYDETECTORENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static AUDIOPROXIMITYDETECTORENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioProximityDetectorEntityData-Array",
    name_hash: 1468282371,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("AudioProximityDetectorEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ProximityDetectorType {
    #[default]
    ProximityDetectorType_Audio = 0,
    ProximityDetectorType_Targeting = 1,
}

pub static PROXIMITYDETECTORTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProximityDetectorType",
    name_hash: 2150450642,
    flags: MemberInfoFlags::new(49429),
    module: "DiceCommonsShared",
    data: TypeInfoData::Enum,
    array_type: Some(PROXIMITYDETECTORTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ProximityDetectorType {
    fn type_info(&self) -> &'static TypeInfo {
        PROXIMITYDETECTORTYPE_TYPE_INFO
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


pub static PROXIMITYDETECTORTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProximityDetectorType-Array",
    name_hash: 858291558,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ProximityDetectorType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct VoiceOverIntervalEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub interval: Option<LockedTypeObject /* super::audio::VoiceOverInterval */>,
    pub time_threshold: f32,
    pub reset_if_threshold_reached: bool,
}

pub trait VoiceOverIntervalEntityDataTrait: super::entity::EntityDataTrait {
    fn interval(&self) -> &Option<LockedTypeObject /* super::audio::VoiceOverInterval */>;
    fn interval_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::VoiceOverInterval */>;
    fn time_threshold(&self) -> &f32;
    fn time_threshold_mut(&mut self) -> &mut f32;
    fn reset_if_threshold_reached(&self) -> &bool;
    fn reset_if_threshold_reached_mut(&mut self) -> &mut bool;
}

impl VoiceOverIntervalEntityDataTrait for VoiceOverIntervalEntityData {
    fn interval(&self) -> &Option<LockedTypeObject /* super::audio::VoiceOverInterval */> {
        &self.interval
    }
    fn interval_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::VoiceOverInterval */> {
        &mut self.interval
    }
    fn time_threshold(&self) -> &f32 {
        &self.time_threshold
    }
    fn time_threshold_mut(&mut self) -> &mut f32 {
        &mut self.time_threshold
    }
    fn reset_if_threshold_reached(&self) -> &bool {
        &self.reset_if_threshold_reached
    }
    fn reset_if_threshold_reached_mut(&mut self) -> &mut bool {
        &mut self.reset_if_threshold_reached
    }
}

impl super::entity::EntityDataTrait for VoiceOverIntervalEntityData {
}

impl super::entity::GameObjectDataTrait for VoiceOverIntervalEntityData {
}

impl super::core::DataBusPeerTrait for VoiceOverIntervalEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for VoiceOverIntervalEntityData {
}

impl super::core::DataContainerTrait for VoiceOverIntervalEntityData {
}

pub static VOICEOVERINTERVALENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceOverIntervalEntityData",
    name_hash: 1295999817,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(VoiceOverIntervalEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VoiceOverIntervalEntityData as Default>::default())),
            create_boxed: || Box::new(<VoiceOverIntervalEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Interval",
                name_hash: 4280103418,
                flags: MemberInfoFlags::new(0),
                field_type: "VoiceOverInterval",
                rust_offset: offset_of!(VoiceOverIntervalEntityData, interval),
            },
            FieldInfoData {
                name: "TimeThreshold",
                name_hash: 780400743,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VoiceOverIntervalEntityData, time_threshold),
            },
            FieldInfoData {
                name: "ResetIfThresholdReached",
                name_hash: 2737647636,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VoiceOverIntervalEntityData, reset_if_threshold_reached),
            },
        ],
    }),
    array_type: Some(VOICEOVERINTERVALENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VoiceOverIntervalEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        VOICEOVERINTERVALENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static VOICEOVERINTERVALENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceOverIntervalEntityData-Array",
    name_hash: 1588701565,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("VoiceOverIntervalEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct VoiceOverConversationCheckEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub queue_group: Option<LockedTypeObject /* super::audio::VoiceOverConversationQueueGroup */>,
    pub continuous_update: bool,
}

pub trait VoiceOverConversationCheckEntityDataTrait: super::entity::EntityDataTrait {
    fn queue_group(&self) -> &Option<LockedTypeObject /* super::audio::VoiceOverConversationQueueGroup */>;
    fn queue_group_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::VoiceOverConversationQueueGroup */>;
    fn continuous_update(&self) -> &bool;
    fn continuous_update_mut(&mut self) -> &mut bool;
}

impl VoiceOverConversationCheckEntityDataTrait for VoiceOverConversationCheckEntityData {
    fn queue_group(&self) -> &Option<LockedTypeObject /* super::audio::VoiceOverConversationQueueGroup */> {
        &self.queue_group
    }
    fn queue_group_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::VoiceOverConversationQueueGroup */> {
        &mut self.queue_group
    }
    fn continuous_update(&self) -> &bool {
        &self.continuous_update
    }
    fn continuous_update_mut(&mut self) -> &mut bool {
        &mut self.continuous_update
    }
}

impl super::entity::EntityDataTrait for VoiceOverConversationCheckEntityData {
}

impl super::entity::GameObjectDataTrait for VoiceOverConversationCheckEntityData {
}

impl super::core::DataBusPeerTrait for VoiceOverConversationCheckEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for VoiceOverConversationCheckEntityData {
}

impl super::core::DataContainerTrait for VoiceOverConversationCheckEntityData {
}

pub static VOICEOVERCONVERSATIONCHECKENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceOverConversationCheckEntityData",
    name_hash: 3097889405,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(VoiceOverConversationCheckEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VoiceOverConversationCheckEntityData as Default>::default())),
            create_boxed: || Box::new(<VoiceOverConversationCheckEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "QueueGroup",
                name_hash: 2016027659,
                flags: MemberInfoFlags::new(0),
                field_type: "VoiceOverConversationQueueGroup",
                rust_offset: offset_of!(VoiceOverConversationCheckEntityData, queue_group),
            },
            FieldInfoData {
                name: "ContinuousUpdate",
                name_hash: 1186338873,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VoiceOverConversationCheckEntityData, continuous_update),
            },
        ],
    }),
    array_type: Some(VOICEOVERCONVERSATIONCHECKENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VoiceOverConversationCheckEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        VOICEOVERCONVERSATIONCHECKENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static VOICEOVERCONVERSATIONCHECKENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceOverConversationCheckEntityData-Array",
    name_hash: 2865502025,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("VoiceOverConversationCheckEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct VoiceOverContextAreaResultEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub position: super::core::Vec3,
}

pub trait VoiceOverContextAreaResultEntityDataTrait: super::entity::EntityDataTrait {
    fn position(&self) -> &super::core::Vec3;
    fn position_mut(&mut self) -> &mut super::core::Vec3;
}

impl VoiceOverContextAreaResultEntityDataTrait for VoiceOverContextAreaResultEntityData {
    fn position(&self) -> &super::core::Vec3 {
        &self.position
    }
    fn position_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.position
    }
}

impl super::entity::EntityDataTrait for VoiceOverContextAreaResultEntityData {
}

impl super::entity::GameObjectDataTrait for VoiceOverContextAreaResultEntityData {
}

impl super::core::DataBusPeerTrait for VoiceOverContextAreaResultEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for VoiceOverContextAreaResultEntityData {
}

impl super::core::DataContainerTrait for VoiceOverContextAreaResultEntityData {
}

pub static VOICEOVERCONTEXTAREARESULTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceOverContextAreaResultEntityData",
    name_hash: 2862442903,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(VoiceOverContextAreaResultEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VoiceOverContextAreaResultEntityData as Default>::default())),
            create_boxed: || Box::new(<VoiceOverContextAreaResultEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Position",
                name_hash: 3402582524,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(VoiceOverContextAreaResultEntityData, position),
            },
        ],
    }),
    array_type: Some(VOICEOVERCONTEXTAREARESULTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for VoiceOverContextAreaResultEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        VOICEOVERCONTEXTAREARESULTENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static VOICEOVERCONTEXTAREARESULTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceOverContextAreaResultEntityData-Array",
    name_hash: 937095715,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("VoiceOverContextAreaResultEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct VoiceOverContextAreaEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub enable_on_creation: bool,
    pub context_id: i32,
}

pub trait VoiceOverContextAreaEntityDataTrait: super::entity::EntityDataTrait {
    fn enable_on_creation(&self) -> &bool;
    fn enable_on_creation_mut(&mut self) -> &mut bool;
    fn context_id(&self) -> &i32;
    fn context_id_mut(&mut self) -> &mut i32;
}

impl VoiceOverContextAreaEntityDataTrait for VoiceOverContextAreaEntityData {
    fn enable_on_creation(&self) -> &bool {
        &self.enable_on_creation
    }
    fn enable_on_creation_mut(&mut self) -> &mut bool {
        &mut self.enable_on_creation
    }
    fn context_id(&self) -> &i32 {
        &self.context_id
    }
    fn context_id_mut(&mut self) -> &mut i32 {
        &mut self.context_id
    }
}

impl super::entity::EntityDataTrait for VoiceOverContextAreaEntityData {
}

impl super::entity::GameObjectDataTrait for VoiceOverContextAreaEntityData {
}

impl super::core::DataBusPeerTrait for VoiceOverContextAreaEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for VoiceOverContextAreaEntityData {
}

impl super::core::DataContainerTrait for VoiceOverContextAreaEntityData {
}

pub static VOICEOVERCONTEXTAREAENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceOverContextAreaEntityData",
    name_hash: 2269253214,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(VoiceOverContextAreaEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VoiceOverContextAreaEntityData as Default>::default())),
            create_boxed: || Box::new(<VoiceOverContextAreaEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "EnableOnCreation",
                name_hash: 348824620,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VoiceOverContextAreaEntityData, enable_on_creation),
            },
            FieldInfoData {
                name: "ContextId",
                name_hash: 2421062423,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(VoiceOverContextAreaEntityData, context_id),
            },
        ],
    }),
    array_type: Some(VOICEOVERCONTEXTAREAENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VoiceOverContextAreaEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        VOICEOVERCONTEXTAREAENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static VOICEOVERCONTEXTAREAENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceOverContextAreaEntityData-Array",
    name_hash: 2576396266,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("VoiceOverContextAreaEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct VehicleSoundEntityData {
    pub _glacier_base: DiceSoundEntityData,
    pub velocity_parameter: Option<LockedTypeObject /* super::audio::AudioGraphParameter */>,
    pub angular_velocity_parameter: Option<LockedTypeObject /* super::audio::AudioGraphParameter */>,
    pub g_force_parameter: Option<LockedTypeObject /* super::audio::AudioGraphParameter */>,
    pub local_player_in_vehicle_parameter: Option<LockedTypeObject /* super::audio::AudioGraphParameter */>,
    pub interior_cam_parameter: Option<LockedTypeObject /* super::audio::AudioGraphParameter */>,
    pub in_driver_pos_parameter: Option<LockedTypeObject /* super::audio::AudioGraphParameter */>,
    pub roll_parameter: Option<LockedTypeObject /* super::audio::AudioGraphParameter */>,
    pub tilt_parameter: Option<LockedTypeObject /* super::audio::AudioGraphParameter */>,
    pub yaw_parameter: Option<LockedTypeObject /* super::audio::AudioGraphParameter */>,
    pub roll_speed_parameter: Option<LockedTypeObject /* super::audio::AudioGraphParameter */>,
    pub tilt_speed_parameter: Option<LockedTypeObject /* super::audio::AudioGraphParameter */>,
    pub yaw_speed_parameter: Option<LockedTypeObject /* super::audio::AudioGraphParameter */>,
    pub throttle_input_parameter: Option<LockedTypeObject /* super::audio::AudioGraphParameter */>,
    pub roll_input_parameter: Option<LockedTypeObject /* super::audio::AudioGraphParameter */>,
    pub yaw_input_parameter: Option<LockedTypeObject /* super::audio::AudioGraphParameter */>,
    pub tilt_input_parameter: Option<LockedTypeObject /* super::audio::AudioGraphParameter */>,
    pub camera_fov_parameter: Option<LockedTypeObject /* super::audio::AudioGraphParameter */>,
    pub free_camera_active_parameter: Option<LockedTypeObject /* super::audio::AudioGraphParameter */>,
    pub throttle_input_action: i32,
    pub roll_input_action: i32,
    pub yaw_input_action: i32,
    pub tilt_input_action: i32,
    pub proximity_output_max_speed: f32,
    pub proximity_output: Option<LockedTypeObject /* super::audio::OutputNodeData */>,
    pub proximity_enabled_parameter: Option<LockedTypeObject /* super::audio::AudioGraphParameter */>,
    pub proximity_distance_parameter: Option<LockedTypeObject /* super::audio::AudioGraphParameter */>,
    pub proximity_enclosedness_parameter: Option<LockedTypeObject /* super::audio::AudioGraphParameter */>,
}

pub trait VehicleSoundEntityDataTrait: DiceSoundEntityDataTrait {
    fn velocity_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn velocity_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn angular_velocity_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn angular_velocity_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn g_force_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn g_force_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn local_player_in_vehicle_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn local_player_in_vehicle_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn interior_cam_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn interior_cam_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn in_driver_pos_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn in_driver_pos_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn roll_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn roll_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn tilt_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn tilt_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn yaw_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn yaw_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn roll_speed_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn roll_speed_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn tilt_speed_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn tilt_speed_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn yaw_speed_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn yaw_speed_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn throttle_input_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn throttle_input_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn roll_input_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn roll_input_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn yaw_input_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn yaw_input_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn tilt_input_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn tilt_input_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn camera_fov_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn camera_fov_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn free_camera_active_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn free_camera_active_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn throttle_input_action(&self) -> &i32;
    fn throttle_input_action_mut(&mut self) -> &mut i32;
    fn roll_input_action(&self) -> &i32;
    fn roll_input_action_mut(&mut self) -> &mut i32;
    fn yaw_input_action(&self) -> &i32;
    fn yaw_input_action_mut(&mut self) -> &mut i32;
    fn tilt_input_action(&self) -> &i32;
    fn tilt_input_action_mut(&mut self) -> &mut i32;
    fn proximity_output_max_speed(&self) -> &f32;
    fn proximity_output_max_speed_mut(&mut self) -> &mut f32;
    fn proximity_output(&self) -> &Option<LockedTypeObject /* super::audio::OutputNodeData */>;
    fn proximity_output_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::OutputNodeData */>;
    fn proximity_enabled_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn proximity_enabled_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn proximity_distance_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn proximity_distance_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn proximity_enclosedness_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
    fn proximity_enclosedness_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */>;
}

impl VehicleSoundEntityDataTrait for VehicleSoundEntityData {
    fn velocity_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &self.velocity_parameter
    }
    fn velocity_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &mut self.velocity_parameter
    }
    fn angular_velocity_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &self.angular_velocity_parameter
    }
    fn angular_velocity_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &mut self.angular_velocity_parameter
    }
    fn g_force_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &self.g_force_parameter
    }
    fn g_force_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &mut self.g_force_parameter
    }
    fn local_player_in_vehicle_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &self.local_player_in_vehicle_parameter
    }
    fn local_player_in_vehicle_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &mut self.local_player_in_vehicle_parameter
    }
    fn interior_cam_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &self.interior_cam_parameter
    }
    fn interior_cam_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &mut self.interior_cam_parameter
    }
    fn in_driver_pos_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &self.in_driver_pos_parameter
    }
    fn in_driver_pos_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &mut self.in_driver_pos_parameter
    }
    fn roll_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &self.roll_parameter
    }
    fn roll_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &mut self.roll_parameter
    }
    fn tilt_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &self.tilt_parameter
    }
    fn tilt_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &mut self.tilt_parameter
    }
    fn yaw_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &self.yaw_parameter
    }
    fn yaw_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &mut self.yaw_parameter
    }
    fn roll_speed_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &self.roll_speed_parameter
    }
    fn roll_speed_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &mut self.roll_speed_parameter
    }
    fn tilt_speed_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &self.tilt_speed_parameter
    }
    fn tilt_speed_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &mut self.tilt_speed_parameter
    }
    fn yaw_speed_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &self.yaw_speed_parameter
    }
    fn yaw_speed_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &mut self.yaw_speed_parameter
    }
    fn throttle_input_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &self.throttle_input_parameter
    }
    fn throttle_input_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &mut self.throttle_input_parameter
    }
    fn roll_input_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &self.roll_input_parameter
    }
    fn roll_input_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &mut self.roll_input_parameter
    }
    fn yaw_input_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &self.yaw_input_parameter
    }
    fn yaw_input_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &mut self.yaw_input_parameter
    }
    fn tilt_input_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &self.tilt_input_parameter
    }
    fn tilt_input_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &mut self.tilt_input_parameter
    }
    fn camera_fov_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &self.camera_fov_parameter
    }
    fn camera_fov_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &mut self.camera_fov_parameter
    }
    fn free_camera_active_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &self.free_camera_active_parameter
    }
    fn free_camera_active_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &mut self.free_camera_active_parameter
    }
    fn throttle_input_action(&self) -> &i32 {
        &self.throttle_input_action
    }
    fn throttle_input_action_mut(&mut self) -> &mut i32 {
        &mut self.throttle_input_action
    }
    fn roll_input_action(&self) -> &i32 {
        &self.roll_input_action
    }
    fn roll_input_action_mut(&mut self) -> &mut i32 {
        &mut self.roll_input_action
    }
    fn yaw_input_action(&self) -> &i32 {
        &self.yaw_input_action
    }
    fn yaw_input_action_mut(&mut self) -> &mut i32 {
        &mut self.yaw_input_action
    }
    fn tilt_input_action(&self) -> &i32 {
        &self.tilt_input_action
    }
    fn tilt_input_action_mut(&mut self) -> &mut i32 {
        &mut self.tilt_input_action
    }
    fn proximity_output_max_speed(&self) -> &f32 {
        &self.proximity_output_max_speed
    }
    fn proximity_output_max_speed_mut(&mut self) -> &mut f32 {
        &mut self.proximity_output_max_speed
    }
    fn proximity_output(&self) -> &Option<LockedTypeObject /* super::audio::OutputNodeData */> {
        &self.proximity_output
    }
    fn proximity_output_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::OutputNodeData */> {
        &mut self.proximity_output
    }
    fn proximity_enabled_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &self.proximity_enabled_parameter
    }
    fn proximity_enabled_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &mut self.proximity_enabled_parameter
    }
    fn proximity_distance_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &self.proximity_distance_parameter
    }
    fn proximity_distance_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &mut self.proximity_distance_parameter
    }
    fn proximity_enclosedness_parameter(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &self.proximity_enclosedness_parameter
    }
    fn proximity_enclosedness_parameter_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphParameter */> {
        &mut self.proximity_enclosedness_parameter
    }
}

impl DiceSoundEntityDataTrait for VehicleSoundEntityData {
    fn attach(&self) -> &Option<LockedTypeObject /* EntityAttachData */> {
        self._glacier_base.attach()
    }
    fn attach_mut(&mut self) -> &mut Option<LockedTypeObject /* EntityAttachData */> {
        self._glacier_base.attach_mut()
    }
    fn update_position(&self) -> &bool {
        self._glacier_base.update_position()
    }
    fn update_position_mut(&mut self) -> &mut bool {
        self._glacier_base.update_position_mut()
    }
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
    fn default_sound(&self) -> &Option<LockedTypeObject /* super::audio::SoundAsset */> {
        self._glacier_base.default_sound()
    }
    fn default_sound_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundAsset */> {
        self._glacier_base.default_sound_mut()
    }
    fn sound(&self) -> &Option<LockedTypeObject /* super::audio::SoundAsset */> {
        self._glacier_base.sound()
    }
    fn sound_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundAsset */> {
        self._glacier_base.sound_mut()
    }
    fn play_on_creation(&self) -> &bool {
        self._glacier_base.play_on_creation()
    }
    fn play_on_creation_mut(&mut self) -> &mut bool {
        self._glacier_base.play_on_creation_mut()
    }
    fn enable_on_creation(&self) -> &bool {
        self._glacier_base.enable_on_creation()
    }
    fn enable_on_creation_mut(&mut self) -> &mut bool {
        self._glacier_base.enable_on_creation_mut()
    }
    fn forget_on_destroy(&self) -> &bool {
        self._glacier_base.forget_on_destroy()
    }
    fn forget_on_destroy_mut(&mut self) -> &mut bool {
        self._glacier_base.forget_on_destroy_mut()
    }
    fn master_amplitude(&self) -> &f32 {
        self._glacier_base.master_amplitude()
    }
    fn master_amplitude_mut(&mut self) -> &mut f32 {
        self._glacier_base.master_amplitude_mut()
    }
}

impl super::entity::EntityDataTrait for VehicleSoundEntityData {
}

impl super::entity::GameObjectDataTrait for VehicleSoundEntityData {
}

impl super::core::DataBusPeerTrait for VehicleSoundEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for VehicleSoundEntityData {
}

impl super::core::DataContainerTrait for VehicleSoundEntityData {
}

pub static VEHICLESOUNDENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleSoundEntityData",
    name_hash: 583195925,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DICESOUNDENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(VehicleSoundEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VehicleSoundEntityData as Default>::default())),
            create_boxed: || Box::new(<VehicleSoundEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "VelocityParameter",
                name_hash: 1765011227,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphParameter",
                rust_offset: offset_of!(VehicleSoundEntityData, velocity_parameter),
            },
            FieldInfoData {
                name: "AngularVelocityParameter",
                name_hash: 170905785,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphParameter",
                rust_offset: offset_of!(VehicleSoundEntityData, angular_velocity_parameter),
            },
            FieldInfoData {
                name: "GForceParameter",
                name_hash: 988470806,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphParameter",
                rust_offset: offset_of!(VehicleSoundEntityData, g_force_parameter),
            },
            FieldInfoData {
                name: "LocalPlayerInVehicleParameter",
                name_hash: 1678460301,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphParameter",
                rust_offset: offset_of!(VehicleSoundEntityData, local_player_in_vehicle_parameter),
            },
            FieldInfoData {
                name: "InteriorCamParameter",
                name_hash: 163137203,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphParameter",
                rust_offset: offset_of!(VehicleSoundEntityData, interior_cam_parameter),
            },
            FieldInfoData {
                name: "InDriverPosParameter",
                name_hash: 522157913,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphParameter",
                rust_offset: offset_of!(VehicleSoundEntityData, in_driver_pos_parameter),
            },
            FieldInfoData {
                name: "RollParameter",
                name_hash: 2474888305,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphParameter",
                rust_offset: offset_of!(VehicleSoundEntityData, roll_parameter),
            },
            FieldInfoData {
                name: "TiltParameter",
                name_hash: 1883291817,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphParameter",
                rust_offset: offset_of!(VehicleSoundEntityData, tilt_parameter),
            },
            FieldInfoData {
                name: "YawParameter",
                name_hash: 1246478403,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphParameter",
                rust_offset: offset_of!(VehicleSoundEntityData, yaw_parameter),
            },
            FieldInfoData {
                name: "RollSpeedParameter",
                name_hash: 1760745974,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphParameter",
                rust_offset: offset_of!(VehicleSoundEntityData, roll_speed_parameter),
            },
            FieldInfoData {
                name: "TiltSpeedParameter",
                name_hash: 1761905070,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphParameter",
                rust_offset: offset_of!(VehicleSoundEntityData, tilt_speed_parameter),
            },
            FieldInfoData {
                name: "YawSpeedParameter",
                name_hash: 3224277508,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphParameter",
                rust_offset: offset_of!(VehicleSoundEntityData, yaw_speed_parameter),
            },
            FieldInfoData {
                name: "ThrottleInputParameter",
                name_hash: 1917009874,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphParameter",
                rust_offset: offset_of!(VehicleSoundEntityData, throttle_input_parameter),
            },
            FieldInfoData {
                name: "RollInputParameter",
                name_hash: 397814599,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphParameter",
                rust_offset: offset_of!(VehicleSoundEntityData, roll_input_parameter),
            },
            FieldInfoData {
                name: "YawInputParameter",
                name_hash: 428484981,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphParameter",
                rust_offset: offset_of!(VehicleSoundEntityData, yaw_input_parameter),
            },
            FieldInfoData {
                name: "TiltInputParameter",
                name_hash: 351633695,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphParameter",
                rust_offset: offset_of!(VehicleSoundEntityData, tilt_input_parameter),
            },
            FieldInfoData {
                name: "CameraFovParameter",
                name_hash: 1360546346,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphParameter",
                rust_offset: offset_of!(VehicleSoundEntityData, camera_fov_parameter),
            },
            FieldInfoData {
                name: "FreeCameraActiveParameter",
                name_hash: 3808990381,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphParameter",
                rust_offset: offset_of!(VehicleSoundEntityData, free_camera_active_parameter),
            },
            FieldInfoData {
                name: "ThrottleInputAction",
                name_hash: 4031576069,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(VehicleSoundEntityData, throttle_input_action),
            },
            FieldInfoData {
                name: "RollInputAction",
                name_hash: 9564976,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(VehicleSoundEntityData, roll_input_action),
            },
            FieldInfoData {
                name: "YawInputAction",
                name_hash: 1432982466,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(VehicleSoundEntityData, yaw_input_action),
            },
            FieldInfoData {
                name: "TiltInputAction",
                name_hash: 3635993704,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(VehicleSoundEntityData, tilt_input_action),
            },
            FieldInfoData {
                name: "ProximityOutputMaxSpeed",
                name_hash: 2100738620,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleSoundEntityData, proximity_output_max_speed),
            },
            FieldInfoData {
                name: "ProximityOutput",
                name_hash: 3964770799,
                flags: MemberInfoFlags::new(0),
                field_type: "OutputNodeData",
                rust_offset: offset_of!(VehicleSoundEntityData, proximity_output),
            },
            FieldInfoData {
                name: "ProximityEnabledParameter",
                name_hash: 348168028,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphParameter",
                rust_offset: offset_of!(VehicleSoundEntityData, proximity_enabled_parameter),
            },
            FieldInfoData {
                name: "ProximityDistanceParameter",
                name_hash: 2383269178,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphParameter",
                rust_offset: offset_of!(VehicleSoundEntityData, proximity_distance_parameter),
            },
            FieldInfoData {
                name: "ProximityEnclosednessParameter",
                name_hash: 3377740939,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphParameter",
                rust_offset: offset_of!(VehicleSoundEntityData, proximity_enclosedness_parameter),
            },
        ],
    }),
    array_type: Some(VEHICLESOUNDENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for VehicleSoundEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        VEHICLESOUNDENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static VEHICLESOUNDENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleSoundEntityData-Array",
    name_hash: 3287321761,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("VehicleSoundEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SoundProviderEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub sound_bank: Vec<Option<LockedTypeObject /* super::audio::SoundAsset */>>,
    pub index: i32,
}

pub trait SoundProviderEntityDataTrait: super::entity::EntityDataTrait {
    fn sound_bank(&self) -> &Vec<Option<LockedTypeObject /* super::audio::SoundAsset */>>;
    fn sound_bank_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::audio::SoundAsset */>>;
    fn index(&self) -> &i32;
    fn index_mut(&mut self) -> &mut i32;
}

impl SoundProviderEntityDataTrait for SoundProviderEntityData {
    fn sound_bank(&self) -> &Vec<Option<LockedTypeObject /* super::audio::SoundAsset */>> {
        &self.sound_bank
    }
    fn sound_bank_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::audio::SoundAsset */>> {
        &mut self.sound_bank
    }
    fn index(&self) -> &i32 {
        &self.index
    }
    fn index_mut(&mut self) -> &mut i32 {
        &mut self.index
    }
}

impl super::entity::EntityDataTrait for SoundProviderEntityData {
}

impl super::entity::GameObjectDataTrait for SoundProviderEntityData {
}

impl super::core::DataBusPeerTrait for SoundProviderEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for SoundProviderEntityData {
}

impl super::core::DataContainerTrait for SoundProviderEntityData {
}

pub static SOUNDPROVIDERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoundProviderEntityData",
    name_hash: 3809196076,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(SoundProviderEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SoundProviderEntityData as Default>::default())),
            create_boxed: || Box::new(<SoundProviderEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "SoundBank",
                name_hash: 523835744,
                flags: MemberInfoFlags::new(144),
                field_type: "SoundAsset-Array",
                rust_offset: offset_of!(SoundProviderEntityData, sound_bank),
            },
            FieldInfoData {
                name: "Index",
                name_hash: 214509467,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(SoundProviderEntityData, index),
            },
        ],
    }),
    array_type: Some(SOUNDPROVIDERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SoundProviderEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        SOUNDPROVIDERENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SOUNDPROVIDERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoundProviderEntityData-Array",
    name_hash: 1978845080,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("SoundProviderEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SoundAssetDataEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub sound_assets: Vec<Option<LockedTypeObject /* super::audio::SoundAsset */>>,
    pub data_assets: Vec<Option<LockedTypeObject /* super::audio::SoundDataAsset */>>,
}

pub trait SoundAssetDataEntityDataTrait: super::entity::EntityDataTrait {
    fn sound_assets(&self) -> &Vec<Option<LockedTypeObject /* super::audio::SoundAsset */>>;
    fn sound_assets_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::audio::SoundAsset */>>;
    fn data_assets(&self) -> &Vec<Option<LockedTypeObject /* super::audio::SoundDataAsset */>>;
    fn data_assets_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::audio::SoundDataAsset */>>;
}

impl SoundAssetDataEntityDataTrait for SoundAssetDataEntityData {
    fn sound_assets(&self) -> &Vec<Option<LockedTypeObject /* super::audio::SoundAsset */>> {
        &self.sound_assets
    }
    fn sound_assets_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::audio::SoundAsset */>> {
        &mut self.sound_assets
    }
    fn data_assets(&self) -> &Vec<Option<LockedTypeObject /* super::audio::SoundDataAsset */>> {
        &self.data_assets
    }
    fn data_assets_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::audio::SoundDataAsset */>> {
        &mut self.data_assets
    }
}

impl super::entity::EntityDataTrait for SoundAssetDataEntityData {
}

impl super::entity::GameObjectDataTrait for SoundAssetDataEntityData {
}

impl super::core::DataBusPeerTrait for SoundAssetDataEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for SoundAssetDataEntityData {
}

impl super::core::DataContainerTrait for SoundAssetDataEntityData {
}

pub static SOUNDASSETDATAENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoundAssetDataEntityData",
    name_hash: 598897741,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(SoundAssetDataEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SoundAssetDataEntityData as Default>::default())),
            create_boxed: || Box::new(<SoundAssetDataEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "SoundAssets",
                name_hash: 3728303845,
                flags: MemberInfoFlags::new(144),
                field_type: "SoundAsset-Array",
                rust_offset: offset_of!(SoundAssetDataEntityData, sound_assets),
            },
            FieldInfoData {
                name: "DataAssets",
                name_hash: 4187781142,
                flags: MemberInfoFlags::new(144),
                field_type: "SoundDataAsset-Array",
                rust_offset: offset_of!(SoundAssetDataEntityData, data_assets),
            },
        ],
    }),
    array_type: Some(SOUNDASSETDATAENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SoundAssetDataEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        SOUNDASSETDATAENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SOUNDASSETDATAENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoundAssetDataEntityData-Array",
    name_hash: 3199019129,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("SoundAssetDataEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SoundActivityTesterEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub test_case_name: String,
    pub time_out: f32,
    pub auto_success: bool,
    pub auto_start: bool,
    pub detect_at_least_one_asset: bool,
    pub assets_to_track: Vec<Option<LockedTypeObject /* super::core::Asset */>>,
}

pub trait SoundActivityTesterEntityDataTrait: super::entity::EntityDataTrait {
    fn test_case_name(&self) -> &String;
    fn test_case_name_mut(&mut self) -> &mut String;
    fn time_out(&self) -> &f32;
    fn time_out_mut(&mut self) -> &mut f32;
    fn auto_success(&self) -> &bool;
    fn auto_success_mut(&mut self) -> &mut bool;
    fn auto_start(&self) -> &bool;
    fn auto_start_mut(&mut self) -> &mut bool;
    fn detect_at_least_one_asset(&self) -> &bool;
    fn detect_at_least_one_asset_mut(&mut self) -> &mut bool;
    fn assets_to_track(&self) -> &Vec<Option<LockedTypeObject /* super::core::Asset */>>;
    fn assets_to_track_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::core::Asset */>>;
}

impl SoundActivityTesterEntityDataTrait for SoundActivityTesterEntityData {
    fn test_case_name(&self) -> &String {
        &self.test_case_name
    }
    fn test_case_name_mut(&mut self) -> &mut String {
        &mut self.test_case_name
    }
    fn time_out(&self) -> &f32 {
        &self.time_out
    }
    fn time_out_mut(&mut self) -> &mut f32 {
        &mut self.time_out
    }
    fn auto_success(&self) -> &bool {
        &self.auto_success
    }
    fn auto_success_mut(&mut self) -> &mut bool {
        &mut self.auto_success
    }
    fn auto_start(&self) -> &bool {
        &self.auto_start
    }
    fn auto_start_mut(&mut self) -> &mut bool {
        &mut self.auto_start
    }
    fn detect_at_least_one_asset(&self) -> &bool {
        &self.detect_at_least_one_asset
    }
    fn detect_at_least_one_asset_mut(&mut self) -> &mut bool {
        &mut self.detect_at_least_one_asset
    }
    fn assets_to_track(&self) -> &Vec<Option<LockedTypeObject /* super::core::Asset */>> {
        &self.assets_to_track
    }
    fn assets_to_track_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::core::Asset */>> {
        &mut self.assets_to_track
    }
}

impl super::entity::EntityDataTrait for SoundActivityTesterEntityData {
}

impl super::entity::GameObjectDataTrait for SoundActivityTesterEntityData {
}

impl super::core::DataBusPeerTrait for SoundActivityTesterEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for SoundActivityTesterEntityData {
}

impl super::core::DataContainerTrait for SoundActivityTesterEntityData {
}

pub static SOUNDACTIVITYTESTERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoundActivityTesterEntityData",
    name_hash: 3601952641,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(SoundActivityTesterEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SoundActivityTesterEntityData as Default>::default())),
            create_boxed: || Box::new(<SoundActivityTesterEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "TestCaseName",
                name_hash: 2495365472,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(SoundActivityTesterEntityData, test_case_name),
            },
            FieldInfoData {
                name: "TimeOut",
                name_hash: 3344659518,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoundActivityTesterEntityData, time_out),
            },
            FieldInfoData {
                name: "AutoSuccess",
                name_hash: 4205869673,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SoundActivityTesterEntityData, auto_success),
            },
            FieldInfoData {
                name: "AutoStart",
                name_hash: 792615882,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SoundActivityTesterEntityData, auto_start),
            },
            FieldInfoData {
                name: "DetectAtLeastOneAsset",
                name_hash: 2876581676,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SoundActivityTesterEntityData, detect_at_least_one_asset),
            },
            FieldInfoData {
                name: "AssetsToTrack",
                name_hash: 2369138322,
                flags: MemberInfoFlags::new(144),
                field_type: "Asset-Array",
                rust_offset: offset_of!(SoundActivityTesterEntityData, assets_to_track),
            },
        ],
    }),
    array_type: Some(SOUNDACTIVITYTESTERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SoundActivityTesterEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        SOUNDACTIVITYTESTERENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SOUNDACTIVITYTESTERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoundActivityTesterEntityData-Array",
    name_hash: 1968212277,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("SoundActivityTesterEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MusicEventPriorityEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub music_event_names: Vec<String>,
}

pub trait MusicEventPriorityEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn music_event_names(&self) -> &Vec<String>;
    fn music_event_names_mut(&mut self) -> &mut Vec<String>;
}

impl MusicEventPriorityEntityDataTrait for MusicEventPriorityEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn music_event_names(&self) -> &Vec<String> {
        &self.music_event_names
    }
    fn music_event_names_mut(&mut self) -> &mut Vec<String> {
        &mut self.music_event_names
    }
}

impl super::entity::EntityDataTrait for MusicEventPriorityEntityData {
}

impl super::entity::GameObjectDataTrait for MusicEventPriorityEntityData {
}

impl super::core::DataBusPeerTrait for MusicEventPriorityEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for MusicEventPriorityEntityData {
}

impl super::core::DataContainerTrait for MusicEventPriorityEntityData {
}

pub static MUSICEVENTPRIORITYENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MusicEventPriorityEntityData",
    name_hash: 1222755153,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(MusicEventPriorityEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MusicEventPriorityEntityData as Default>::default())),
            create_boxed: || Box::new(<MusicEventPriorityEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                name_hash: 229961746,
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(MusicEventPriorityEntityData, realm),
            },
            FieldInfoData {
                name: "MusicEventNames",
                name_hash: 1509486588,
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(MusicEventPriorityEntityData, music_event_names),
            },
        ],
    }),
    array_type: Some(MUSICEVENTPRIORITYENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MusicEventPriorityEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        MUSICEVENTPRIORITYENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static MUSICEVENTPRIORITYENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MusicEventPriorityEntityData-Array",
    name_hash: 1873769829,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("MusicEventPriorityEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DiceSoundSpatialEntityData {
    pub _glacier_base: super::entity::SpatialEntityData,
    pub sound: Option<LockedTypeObject /* super::audio::SoundAsset */>,
    pub play_on_creation: bool,
    pub amplitude: f32,
}

pub trait DiceSoundSpatialEntityDataTrait: super::entity::SpatialEntityDataTrait {
    fn sound(&self) -> &Option<LockedTypeObject /* super::audio::SoundAsset */>;
    fn sound_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundAsset */>;
    fn play_on_creation(&self) -> &bool;
    fn play_on_creation_mut(&mut self) -> &mut bool;
    fn amplitude(&self) -> &f32;
    fn amplitude_mut(&mut self) -> &mut f32;
}

impl DiceSoundSpatialEntityDataTrait for DiceSoundSpatialEntityData {
    fn sound(&self) -> &Option<LockedTypeObject /* super::audio::SoundAsset */> {
        &self.sound
    }
    fn sound_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundAsset */> {
        &mut self.sound
    }
    fn play_on_creation(&self) -> &bool {
        &self.play_on_creation
    }
    fn play_on_creation_mut(&mut self) -> &mut bool {
        &mut self.play_on_creation
    }
    fn amplitude(&self) -> &f32 {
        &self.amplitude
    }
    fn amplitude_mut(&mut self) -> &mut f32 {
        &mut self.amplitude
    }
}

impl super::entity::SpatialEntityDataTrait for DiceSoundSpatialEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::entity::EntityDataTrait for DiceSoundSpatialEntityData {
}

impl super::entity::GameObjectDataTrait for DiceSoundSpatialEntityData {
}

impl super::core::DataBusPeerTrait for DiceSoundSpatialEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for DiceSoundSpatialEntityData {
}

impl super::core::DataContainerTrait for DiceSoundSpatialEntityData {
}

pub static DICESOUNDSPATIALENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceSoundSpatialEntityData",
    name_hash: 2457527828,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(DiceSoundSpatialEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DiceSoundSpatialEntityData as Default>::default())),
            create_boxed: || Box::new(<DiceSoundSpatialEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Sound",
                name_hash: 231353798,
                flags: MemberInfoFlags::new(0),
                field_type: "SoundAsset",
                rust_offset: offset_of!(DiceSoundSpatialEntityData, sound),
            },
            FieldInfoData {
                name: "PlayOnCreation",
                name_hash: 2168204873,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DiceSoundSpatialEntityData, play_on_creation),
            },
            FieldInfoData {
                name: "Amplitude",
                name_hash: 698564572,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DiceSoundSpatialEntityData, amplitude),
            },
        ],
    }),
    array_type: Some(DICESOUNDSPATIALENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DiceSoundSpatialEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        DICESOUNDSPATIALENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DICESOUNDSPATIALENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceSoundSpatialEntityData-Array",
    name_hash: 771785248,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceSoundSpatialEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DiceSoundEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub attach: Option<LockedTypeObject /* EntityAttachData */>,
    pub update_position: bool,
    pub transform: super::core::LinearTransform,
    pub default_sound: Option<LockedTypeObject /* super::audio::SoundAsset */>,
    pub sound: Option<LockedTypeObject /* super::audio::SoundAsset */>,
    pub play_on_creation: bool,
    pub enable_on_creation: bool,
    pub forget_on_destroy: bool,
    pub master_amplitude: f32,
}

pub trait DiceSoundEntityDataTrait: super::entity::EntityDataTrait {
    fn attach(&self) -> &Option<LockedTypeObject /* EntityAttachData */>;
    fn attach_mut(&mut self) -> &mut Option<LockedTypeObject /* EntityAttachData */>;
    fn update_position(&self) -> &bool;
    fn update_position_mut(&mut self) -> &mut bool;
    fn transform(&self) -> &super::core::LinearTransform;
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform;
    fn default_sound(&self) -> &Option<LockedTypeObject /* super::audio::SoundAsset */>;
    fn default_sound_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundAsset */>;
    fn sound(&self) -> &Option<LockedTypeObject /* super::audio::SoundAsset */>;
    fn sound_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundAsset */>;
    fn play_on_creation(&self) -> &bool;
    fn play_on_creation_mut(&mut self) -> &mut bool;
    fn enable_on_creation(&self) -> &bool;
    fn enable_on_creation_mut(&mut self) -> &mut bool;
    fn forget_on_destroy(&self) -> &bool;
    fn forget_on_destroy_mut(&mut self) -> &mut bool;
    fn master_amplitude(&self) -> &f32;
    fn master_amplitude_mut(&mut self) -> &mut f32;
}

impl DiceSoundEntityDataTrait for DiceSoundEntityData {
    fn attach(&self) -> &Option<LockedTypeObject /* EntityAttachData */> {
        &self.attach
    }
    fn attach_mut(&mut self) -> &mut Option<LockedTypeObject /* EntityAttachData */> {
        &mut self.attach
    }
    fn update_position(&self) -> &bool {
        &self.update_position
    }
    fn update_position_mut(&mut self) -> &mut bool {
        &mut self.update_position
    }
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.transform
    }
    fn default_sound(&self) -> &Option<LockedTypeObject /* super::audio::SoundAsset */> {
        &self.default_sound
    }
    fn default_sound_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundAsset */> {
        &mut self.default_sound
    }
    fn sound(&self) -> &Option<LockedTypeObject /* super::audio::SoundAsset */> {
        &self.sound
    }
    fn sound_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundAsset */> {
        &mut self.sound
    }
    fn play_on_creation(&self) -> &bool {
        &self.play_on_creation
    }
    fn play_on_creation_mut(&mut self) -> &mut bool {
        &mut self.play_on_creation
    }
    fn enable_on_creation(&self) -> &bool {
        &self.enable_on_creation
    }
    fn enable_on_creation_mut(&mut self) -> &mut bool {
        &mut self.enable_on_creation
    }
    fn forget_on_destroy(&self) -> &bool {
        &self.forget_on_destroy
    }
    fn forget_on_destroy_mut(&mut self) -> &mut bool {
        &mut self.forget_on_destroy
    }
    fn master_amplitude(&self) -> &f32 {
        &self.master_amplitude
    }
    fn master_amplitude_mut(&mut self) -> &mut f32 {
        &mut self.master_amplitude
    }
}

impl super::entity::EntityDataTrait for DiceSoundEntityData {
}

impl super::entity::GameObjectDataTrait for DiceSoundEntityData {
}

impl super::core::DataBusPeerTrait for DiceSoundEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for DiceSoundEntityData {
}

impl super::core::DataContainerTrait for DiceSoundEntityData {
}

pub static DICESOUNDENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceSoundEntityData",
    name_hash: 3393455686,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(DiceSoundEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DiceSoundEntityData as Default>::default())),
            create_boxed: || Box::new(<DiceSoundEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Attach",
                name_hash: 2500885102,
                flags: MemberInfoFlags::new(0),
                field_type: "EntityAttachData",
                rust_offset: offset_of!(DiceSoundEntityData, attach),
            },
            FieldInfoData {
                name: "UpdatePosition",
                name_hash: 1876148909,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DiceSoundEntityData, update_position),
            },
            FieldInfoData {
                name: "Transform",
                name_hash: 2270319721,
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(DiceSoundEntityData, transform),
            },
            FieldInfoData {
                name: "DefaultSound",
                name_hash: 2078444557,
                flags: MemberInfoFlags::new(0),
                field_type: "SoundAsset",
                rust_offset: offset_of!(DiceSoundEntityData, default_sound),
            },
            FieldInfoData {
                name: "Sound",
                name_hash: 231353798,
                flags: MemberInfoFlags::new(0),
                field_type: "SoundAsset",
                rust_offset: offset_of!(DiceSoundEntityData, sound),
            },
            FieldInfoData {
                name: "PlayOnCreation",
                name_hash: 2168204873,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DiceSoundEntityData, play_on_creation),
            },
            FieldInfoData {
                name: "EnableOnCreation",
                name_hash: 348824620,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DiceSoundEntityData, enable_on_creation),
            },
            FieldInfoData {
                name: "ForgetOnDestroy",
                name_hash: 618562347,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DiceSoundEntityData, forget_on_destroy),
            },
            FieldInfoData {
                name: "MasterAmplitude",
                name_hash: 1033950080,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DiceSoundEntityData, master_amplitude),
            },
        ],
    }),
    array_type: Some(DICESOUNDENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DiceSoundEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        DICESOUNDENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DICESOUNDENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceSoundEntityData-Array",
    name_hash: 2988082162,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceSoundEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DiceSoundAreaEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub sound: Option<LockedTypeObject /* super::audio::SoundAsset */>,
    pub big_world: Option<LockedTypeObject /* super::gameplay_sim::BigWorldSettingsAsset */>,
    pub perimeter_size: f32,
    pub relevance_multiplier: f32,
    pub min_relevance_budget: f32,
    pub relevance_falloff: super::audio::FadeCurveType,
    pub enable_on_creation: bool,
    pub priority: f32,
    pub use_legacy_behavior: bool,
    pub face_listener: bool,
    pub ignore_vertical_perimeter: bool,
    pub area_type: i32,
}

pub trait DiceSoundAreaEntityDataTrait: super::entity::EntityDataTrait {
    fn sound(&self) -> &Option<LockedTypeObject /* super::audio::SoundAsset */>;
    fn sound_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundAsset */>;
    fn big_world(&self) -> &Option<LockedTypeObject /* super::gameplay_sim::BigWorldSettingsAsset */>;
    fn big_world_mut(&mut self) -> &mut Option<LockedTypeObject /* super::gameplay_sim::BigWorldSettingsAsset */>;
    fn perimeter_size(&self) -> &f32;
    fn perimeter_size_mut(&mut self) -> &mut f32;
    fn relevance_multiplier(&self) -> &f32;
    fn relevance_multiplier_mut(&mut self) -> &mut f32;
    fn min_relevance_budget(&self) -> &f32;
    fn min_relevance_budget_mut(&mut self) -> &mut f32;
    fn relevance_falloff(&self) -> &super::audio::FadeCurveType;
    fn relevance_falloff_mut(&mut self) -> &mut super::audio::FadeCurveType;
    fn enable_on_creation(&self) -> &bool;
    fn enable_on_creation_mut(&mut self) -> &mut bool;
    fn priority(&self) -> &f32;
    fn priority_mut(&mut self) -> &mut f32;
    fn use_legacy_behavior(&self) -> &bool;
    fn use_legacy_behavior_mut(&mut self) -> &mut bool;
    fn face_listener(&self) -> &bool;
    fn face_listener_mut(&mut self) -> &mut bool;
    fn ignore_vertical_perimeter(&self) -> &bool;
    fn ignore_vertical_perimeter_mut(&mut self) -> &mut bool;
    fn area_type(&self) -> &i32;
    fn area_type_mut(&mut self) -> &mut i32;
}

impl DiceSoundAreaEntityDataTrait for DiceSoundAreaEntityData {
    fn sound(&self) -> &Option<LockedTypeObject /* super::audio::SoundAsset */> {
        &self.sound
    }
    fn sound_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundAsset */> {
        &mut self.sound
    }
    fn big_world(&self) -> &Option<LockedTypeObject /* super::gameplay_sim::BigWorldSettingsAsset */> {
        &self.big_world
    }
    fn big_world_mut(&mut self) -> &mut Option<LockedTypeObject /* super::gameplay_sim::BigWorldSettingsAsset */> {
        &mut self.big_world
    }
    fn perimeter_size(&self) -> &f32 {
        &self.perimeter_size
    }
    fn perimeter_size_mut(&mut self) -> &mut f32 {
        &mut self.perimeter_size
    }
    fn relevance_multiplier(&self) -> &f32 {
        &self.relevance_multiplier
    }
    fn relevance_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.relevance_multiplier
    }
    fn min_relevance_budget(&self) -> &f32 {
        &self.min_relevance_budget
    }
    fn min_relevance_budget_mut(&mut self) -> &mut f32 {
        &mut self.min_relevance_budget
    }
    fn relevance_falloff(&self) -> &super::audio::FadeCurveType {
        &self.relevance_falloff
    }
    fn relevance_falloff_mut(&mut self) -> &mut super::audio::FadeCurveType {
        &mut self.relevance_falloff
    }
    fn enable_on_creation(&self) -> &bool {
        &self.enable_on_creation
    }
    fn enable_on_creation_mut(&mut self) -> &mut bool {
        &mut self.enable_on_creation
    }
    fn priority(&self) -> &f32 {
        &self.priority
    }
    fn priority_mut(&mut self) -> &mut f32 {
        &mut self.priority
    }
    fn use_legacy_behavior(&self) -> &bool {
        &self.use_legacy_behavior
    }
    fn use_legacy_behavior_mut(&mut self) -> &mut bool {
        &mut self.use_legacy_behavior
    }
    fn face_listener(&self) -> &bool {
        &self.face_listener
    }
    fn face_listener_mut(&mut self) -> &mut bool {
        &mut self.face_listener
    }
    fn ignore_vertical_perimeter(&self) -> &bool {
        &self.ignore_vertical_perimeter
    }
    fn ignore_vertical_perimeter_mut(&mut self) -> &mut bool {
        &mut self.ignore_vertical_perimeter
    }
    fn area_type(&self) -> &i32 {
        &self.area_type
    }
    fn area_type_mut(&mut self) -> &mut i32 {
        &mut self.area_type
    }
}

impl super::entity::EntityDataTrait for DiceSoundAreaEntityData {
}

impl super::entity::GameObjectDataTrait for DiceSoundAreaEntityData {
}

impl super::core::DataBusPeerTrait for DiceSoundAreaEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for DiceSoundAreaEntityData {
}

impl super::core::DataContainerTrait for DiceSoundAreaEntityData {
}

pub static DICESOUNDAREAENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceSoundAreaEntityData",
    name_hash: 4063027633,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(DiceSoundAreaEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DiceSoundAreaEntityData as Default>::default())),
            create_boxed: || Box::new(<DiceSoundAreaEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Sound",
                name_hash: 231353798,
                flags: MemberInfoFlags::new(0),
                field_type: "SoundAsset",
                rust_offset: offset_of!(DiceSoundAreaEntityData, sound),
            },
            FieldInfoData {
                name: "BigWorld",
                name_hash: 4205700235,
                flags: MemberInfoFlags::new(0),
                field_type: "BigWorldSettingsAsset",
                rust_offset: offset_of!(DiceSoundAreaEntityData, big_world),
            },
            FieldInfoData {
                name: "PerimeterSize",
                name_hash: 156564773,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DiceSoundAreaEntityData, perimeter_size),
            },
            FieldInfoData {
                name: "RelevanceMultiplier",
                name_hash: 3495794351,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DiceSoundAreaEntityData, relevance_multiplier),
            },
            FieldInfoData {
                name: "MinRelevanceBudget",
                name_hash: 4050735531,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DiceSoundAreaEntityData, min_relevance_budget),
            },
            FieldInfoData {
                name: "RelevanceFalloff",
                name_hash: 1856356236,
                flags: MemberInfoFlags::new(0),
                field_type: "FadeCurveType",
                rust_offset: offset_of!(DiceSoundAreaEntityData, relevance_falloff),
            },
            FieldInfoData {
                name: "EnableOnCreation",
                name_hash: 348824620,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DiceSoundAreaEntityData, enable_on_creation),
            },
            FieldInfoData {
                name: "Priority",
                name_hash: 3062102871,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DiceSoundAreaEntityData, priority),
            },
            FieldInfoData {
                name: "UseLegacyBehavior",
                name_hash: 4066039615,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DiceSoundAreaEntityData, use_legacy_behavior),
            },
            FieldInfoData {
                name: "FaceListener",
                name_hash: 3563530970,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DiceSoundAreaEntityData, face_listener),
            },
            FieldInfoData {
                name: "IgnoreVerticalPerimeter",
                name_hash: 2464526538,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DiceSoundAreaEntityData, ignore_vertical_perimeter),
            },
            FieldInfoData {
                name: "AreaType",
                name_hash: 3772263626,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(DiceSoundAreaEntityData, area_type),
            },
        ],
    }),
    array_type: Some(DICESOUNDAREAENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DiceSoundAreaEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        DICESOUNDAREAENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DICESOUNDAREAENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceSoundAreaEntityData-Array",
    name_hash: 1800934405,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceSoundAreaEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AudioCurveFactorEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub curves_guids: Vec<glacier_util::guid::Guid>,
    pub factor: f32,
    pub reset_factor_on_destroy: bool,
}

pub trait AudioCurveFactorEntityDataTrait: super::entity::EntityDataTrait {
    fn curves_guids(&self) -> &Vec<glacier_util::guid::Guid>;
    fn curves_guids_mut(&mut self) -> &mut Vec<glacier_util::guid::Guid>;
    fn factor(&self) -> &f32;
    fn factor_mut(&mut self) -> &mut f32;
    fn reset_factor_on_destroy(&self) -> &bool;
    fn reset_factor_on_destroy_mut(&mut self) -> &mut bool;
}

impl AudioCurveFactorEntityDataTrait for AudioCurveFactorEntityData {
    fn curves_guids(&self) -> &Vec<glacier_util::guid::Guid> {
        &self.curves_guids
    }
    fn curves_guids_mut(&mut self) -> &mut Vec<glacier_util::guid::Guid> {
        &mut self.curves_guids
    }
    fn factor(&self) -> &f32 {
        &self.factor
    }
    fn factor_mut(&mut self) -> &mut f32 {
        &mut self.factor
    }
    fn reset_factor_on_destroy(&self) -> &bool {
        &self.reset_factor_on_destroy
    }
    fn reset_factor_on_destroy_mut(&mut self) -> &mut bool {
        &mut self.reset_factor_on_destroy
    }
}

impl super::entity::EntityDataTrait for AudioCurveFactorEntityData {
}

impl super::entity::GameObjectDataTrait for AudioCurveFactorEntityData {
}

impl super::core::DataBusPeerTrait for AudioCurveFactorEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for AudioCurveFactorEntityData {
}

impl super::core::DataContainerTrait for AudioCurveFactorEntityData {
}

pub static AUDIOCURVEFACTORENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioCurveFactorEntityData",
    name_hash: 1496763106,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(AudioCurveFactorEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AudioCurveFactorEntityData as Default>::default())),
            create_boxed: || Box::new(<AudioCurveFactorEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "CurvesGuids",
                name_hash: 3160072301,
                flags: MemberInfoFlags::new(144),
                field_type: "Guid-Array",
                rust_offset: offset_of!(AudioCurveFactorEntityData, curves_guids),
            },
            FieldInfoData {
                name: "Factor",
                name_hash: 2515882920,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AudioCurveFactorEntityData, factor),
            },
            FieldInfoData {
                name: "ResetFactorOnDestroy",
                name_hash: 1464748510,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AudioCurveFactorEntityData, reset_factor_on_destroy),
            },
        ],
    }),
    array_type: Some(AUDIOCURVEFACTORENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AudioCurveFactorEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        AUDIOCURVEFACTORENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static AUDIOCURVEFACTORENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioCurveFactorEntityData-Array",
    name_hash: 561650134,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("AudioCurveFactorEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DebrisClusterSoundsComponentData {
    pub _glacier_base: super::entity::GameComponentData,
    pub debris_cluster_sounds: Vec<BoxedTypeObject /* DebrisClusterSound */>,
}

pub trait DebrisClusterSoundsComponentDataTrait: super::entity::GameComponentDataTrait {
    fn debris_cluster_sounds(&self) -> &Vec<BoxedTypeObject /* DebrisClusterSound */>;
    fn debris_cluster_sounds_mut(&mut self) -> &mut Vec<BoxedTypeObject /* DebrisClusterSound */>;
}

impl DebrisClusterSoundsComponentDataTrait for DebrisClusterSoundsComponentData {
    fn debris_cluster_sounds(&self) -> &Vec<BoxedTypeObject /* DebrisClusterSound */> {
        &self.debris_cluster_sounds
    }
    fn debris_cluster_sounds_mut(&mut self) -> &mut Vec<BoxedTypeObject /* DebrisClusterSound */> {
        &mut self.debris_cluster_sounds
    }
}

impl super::entity::GameComponentDataTrait for DebrisClusterSoundsComponentData {
}

impl super::entity::ComponentDataTrait for DebrisClusterSoundsComponentData {
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

impl super::entity::GameObjectDataTrait for DebrisClusterSoundsComponentData {
}

impl super::core::DataBusPeerTrait for DebrisClusterSoundsComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for DebrisClusterSoundsComponentData {
}

impl super::core::DataContainerTrait for DebrisClusterSoundsComponentData {
}

pub static DEBRISCLUSTERSOUNDSCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisClusterSoundsComponentData",
    name_hash: 673743339,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::GAMECOMPONENTDATA_TYPE_INFO),
        super_class_offset: offset_of!(DebrisClusterSoundsComponentData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DebrisClusterSoundsComponentData as Default>::default())),
            create_boxed: || Box::new(<DebrisClusterSoundsComponentData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "DebrisClusterSounds",
                name_hash: 1334050548,
                flags: MemberInfoFlags::new(144),
                field_type: "DebrisClusterSound-Array",
                rust_offset: offset_of!(DebrisClusterSoundsComponentData, debris_cluster_sounds),
            },
        ],
    }),
    array_type: Some(DEBRISCLUSTERSOUNDSCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DebrisClusterSoundsComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        DEBRISCLUSTERSOUNDSCOMPONENTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DEBRISCLUSTERSOUNDSCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisClusterSoundsComponentData-Array",
    name_hash: 2885939679,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DebrisClusterSoundsComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DebrisClusterSound {
    pub activation_sound: Option<LockedTypeObject /* super::audio::SoundAsset */>,
    pub collision_sound: Option<LockedTypeObject /* super::audio::SoundAsset */>,
    pub collision_sound_speed_threshold: f32,
    pub part_index: u32,
}

pub trait DebrisClusterSoundTrait: TypeObject {
    fn activation_sound(&self) -> &Option<LockedTypeObject /* super::audio::SoundAsset */>;
    fn activation_sound_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundAsset */>;
    fn collision_sound(&self) -> &Option<LockedTypeObject /* super::audio::SoundAsset */>;
    fn collision_sound_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundAsset */>;
    fn collision_sound_speed_threshold(&self) -> &f32;
    fn collision_sound_speed_threshold_mut(&mut self) -> &mut f32;
    fn part_index(&self) -> &u32;
    fn part_index_mut(&mut self) -> &mut u32;
}

impl DebrisClusterSoundTrait for DebrisClusterSound {
    fn activation_sound(&self) -> &Option<LockedTypeObject /* super::audio::SoundAsset */> {
        &self.activation_sound
    }
    fn activation_sound_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundAsset */> {
        &mut self.activation_sound
    }
    fn collision_sound(&self) -> &Option<LockedTypeObject /* super::audio::SoundAsset */> {
        &self.collision_sound
    }
    fn collision_sound_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundAsset */> {
        &mut self.collision_sound
    }
    fn collision_sound_speed_threshold(&self) -> &f32 {
        &self.collision_sound_speed_threshold
    }
    fn collision_sound_speed_threshold_mut(&mut self) -> &mut f32 {
        &mut self.collision_sound_speed_threshold
    }
    fn part_index(&self) -> &u32 {
        &self.part_index
    }
    fn part_index_mut(&mut self) -> &mut u32 {
        &mut self.part_index
    }
}

pub static DEBRISCLUSTERSOUND_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisClusterSound",
    name_hash: 170576295,
    flags: MemberInfoFlags::new(73),
    module: "DiceCommonsShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DebrisClusterSound as Default>::default())),
            create_boxed: || Box::new(<DebrisClusterSound as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ActivationSound",
                name_hash: 1740567090,
                flags: MemberInfoFlags::new(0),
                field_type: "SoundAsset",
                rust_offset: offset_of!(DebrisClusterSound, activation_sound),
            },
            FieldInfoData {
                name: "CollisionSound",
                name_hash: 699473880,
                flags: MemberInfoFlags::new(0),
                field_type: "SoundAsset",
                rust_offset: offset_of!(DebrisClusterSound, collision_sound),
            },
            FieldInfoData {
                name: "CollisionSoundSpeedThreshold",
                name_hash: 2566530984,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebrisClusterSound, collision_sound_speed_threshold),
            },
            FieldInfoData {
                name: "PartIndex",
                name_hash: 3213901068,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DebrisClusterSound, part_index),
            },
        ],
    }),
    array_type: Some(DEBRISCLUSTERSOUND_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DebrisClusterSound {
    fn type_info(&self) -> &'static TypeInfo {
        DEBRISCLUSTERSOUND_TYPE_INFO
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


pub static DEBRISCLUSTERSOUND_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisClusterSound-Array",
    name_hash: 2148096787,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DebrisClusterSound"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DofReaderEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub dof_set: super::ant::AntRef,
    pub dof_set_name: String,
    pub dof_names_hash_id: Vec<i32>,
    pub start_reading_continously_on_spawn: bool,
    pub read_once_on_spawn: bool,
}

pub trait DofReaderEntityDataTrait: super::entity::EntityDataTrait {
    fn dof_set(&self) -> &super::ant::AntRef;
    fn dof_set_mut(&mut self) -> &mut super::ant::AntRef;
    fn dof_set_name(&self) -> &String;
    fn dof_set_name_mut(&mut self) -> &mut String;
    fn dof_names_hash_id(&self) -> &Vec<i32>;
    fn dof_names_hash_id_mut(&mut self) -> &mut Vec<i32>;
    fn start_reading_continously_on_spawn(&self) -> &bool;
    fn start_reading_continously_on_spawn_mut(&mut self) -> &mut bool;
    fn read_once_on_spawn(&self) -> &bool;
    fn read_once_on_spawn_mut(&mut self) -> &mut bool;
}

impl DofReaderEntityDataTrait for DofReaderEntityData {
    fn dof_set(&self) -> &super::ant::AntRef {
        &self.dof_set
    }
    fn dof_set_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.dof_set
    }
    fn dof_set_name(&self) -> &String {
        &self.dof_set_name
    }
    fn dof_set_name_mut(&mut self) -> &mut String {
        &mut self.dof_set_name
    }
    fn dof_names_hash_id(&self) -> &Vec<i32> {
        &self.dof_names_hash_id
    }
    fn dof_names_hash_id_mut(&mut self) -> &mut Vec<i32> {
        &mut self.dof_names_hash_id
    }
    fn start_reading_continously_on_spawn(&self) -> &bool {
        &self.start_reading_continously_on_spawn
    }
    fn start_reading_continously_on_spawn_mut(&mut self) -> &mut bool {
        &mut self.start_reading_continously_on_spawn
    }
    fn read_once_on_spawn(&self) -> &bool {
        &self.read_once_on_spawn
    }
    fn read_once_on_spawn_mut(&mut self) -> &mut bool {
        &mut self.read_once_on_spawn
    }
}

impl super::entity::EntityDataTrait for DofReaderEntityData {
}

impl super::entity::GameObjectDataTrait for DofReaderEntityData {
}

impl super::core::DataBusPeerTrait for DofReaderEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for DofReaderEntityData {
}

impl super::core::DataContainerTrait for DofReaderEntityData {
}

pub static DOFREADERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DofReaderEntityData",
    name_hash: 4075750470,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(DofReaderEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DofReaderEntityData as Default>::default())),
            create_boxed: || Box::new(<DofReaderEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "DofSet",
                name_hash: 2610327466,
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(DofReaderEntityData, dof_set),
            },
            FieldInfoData {
                name: "DofSetName",
                name_hash: 1528058605,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(DofReaderEntityData, dof_set_name),
            },
            FieldInfoData {
                name: "DofNamesHashId",
                name_hash: 3978875715,
                flags: MemberInfoFlags::new(144),
                field_type: "Int32-Array",
                rust_offset: offset_of!(DofReaderEntityData, dof_names_hash_id),
            },
            FieldInfoData {
                name: "StartReadingContinouslyOnSpawn",
                name_hash: 3590049344,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DofReaderEntityData, start_reading_continously_on_spawn),
            },
            FieldInfoData {
                name: "ReadOnceOnSpawn",
                name_hash: 773521066,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DofReaderEntityData, read_once_on_spawn),
            },
        ],
    }),
    array_type: Some(DOFREADERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DofReaderEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        DOFREADERENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DOFREADERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DofReaderEntityData-Array",
    name_hash: 2055995890,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DofReaderEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AnimatableCullingEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub culling_level: EntityCullingLevel,
    pub culling_distance: f32,
    pub force_cull: bool,
    pub enabled: bool,
}

pub trait AnimatableCullingEntityDataTrait: super::entity::EntityDataTrait {
    fn culling_level(&self) -> &EntityCullingLevel;
    fn culling_level_mut(&mut self) -> &mut EntityCullingLevel;
    fn culling_distance(&self) -> &f32;
    fn culling_distance_mut(&mut self) -> &mut f32;
    fn force_cull(&self) -> &bool;
    fn force_cull_mut(&mut self) -> &mut bool;
    fn enabled(&self) -> &bool;
    fn enabled_mut(&mut self) -> &mut bool;
}

impl AnimatableCullingEntityDataTrait for AnimatableCullingEntityData {
    fn culling_level(&self) -> &EntityCullingLevel {
        &self.culling_level
    }
    fn culling_level_mut(&mut self) -> &mut EntityCullingLevel {
        &mut self.culling_level
    }
    fn culling_distance(&self) -> &f32 {
        &self.culling_distance
    }
    fn culling_distance_mut(&mut self) -> &mut f32 {
        &mut self.culling_distance
    }
    fn force_cull(&self) -> &bool {
        &self.force_cull
    }
    fn force_cull_mut(&mut self) -> &mut bool {
        &mut self.force_cull
    }
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
}

impl super::entity::EntityDataTrait for AnimatableCullingEntityData {
}

impl super::entity::GameObjectDataTrait for AnimatableCullingEntityData {
}

impl super::core::DataBusPeerTrait for AnimatableCullingEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for AnimatableCullingEntityData {
}

impl super::core::DataContainerTrait for AnimatableCullingEntityData {
}

pub static ANIMATABLECULLINGENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimatableCullingEntityData",
    name_hash: 2413715724,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(AnimatableCullingEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AnimatableCullingEntityData as Default>::default())),
            create_boxed: || Box::new(<AnimatableCullingEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "CullingLevel",
                name_hash: 730117541,
                flags: MemberInfoFlags::new(0),
                field_type: "EntityCullingLevel",
                rust_offset: offset_of!(AnimatableCullingEntityData, culling_level),
            },
            FieldInfoData {
                name: "CullingDistance",
                name_hash: 1073297232,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AnimatableCullingEntityData, culling_distance),
            },
            FieldInfoData {
                name: "ForceCull",
                name_hash: 3676201198,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AnimatableCullingEntityData, force_cull),
            },
            FieldInfoData {
                name: "Enabled",
                name_hash: 2662400,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AnimatableCullingEntityData, enabled),
            },
        ],
    }),
    array_type: Some(ANIMATABLECULLINGENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AnimatableCullingEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        ANIMATABLECULLINGENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static ANIMATABLECULLINGENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimatableCullingEntityData-Array",
    name_hash: 398512440,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("AnimatableCullingEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum EntityCullingLevel {
    #[default]
    EntityCullingLevel_InView = 0,
    EntityCullingLevel_Distance = 1,
    EntityCullingLevel_InViewAndDistance = 2,
    EntityCullingLevel_ForceCull = 3,
    EntityCullingLevel_DisableCulling = 4,
}

pub static ENTITYCULLINGLEVEL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityCullingLevel",
    name_hash: 1834009150,
    flags: MemberInfoFlags::new(49429),
    module: "DiceCommonsShared",
    data: TypeInfoData::Enum,
    array_type: Some(ENTITYCULLINGLEVEL_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EntityCullingLevel {
    fn type_info(&self) -> &'static TypeInfo {
        ENTITYCULLINGLEVEL_TYPE_INFO
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


pub static ENTITYCULLINGLEVEL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityCullingLevel-Array",
    name_hash: 270701194,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("EntityCullingLevel"),
    array_type: None,
    alignment: 8,
};


