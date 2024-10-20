use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BlueprintProxyEntityBase {
}

pub const BLUEPRINTPROXYENTITYBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintProxyEntityBase",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(BLUEPRINTPROXYENTITYBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BlueprintProxyEntityBase {
    fn type_info() -> &'static TypeInfo {
        BLUEPRINTPROXYENTITYBASE_TYPE_INFO
    }
}


pub const BLUEPRINTPROXYENTITYBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintProxyEntityBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("BlueprintProxyEntityBase-Array"),
    array_type: None,
    alignment: 8,
};



pub const DRAWTEXT2D_INT32_INT32_VEC4_VEC4__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DrawText2d(Int32,Int32,Vec4,Vec4)",
    flags: MemberInfoFlags::new(793),
    module: "DiceCommonsShared",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};



pub const DRAWTEXT2D_INT32_INT32_VEC4_VEC3__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DrawText2d(Int32,Int32,Vec4,Vec3)",
    flags: MemberInfoFlags::new(793),
    module: "DiceCommonsShared",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};



pub const DRAWTEXT2D_INT32_INT32_VEC4_INT32__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DrawText2d(Int32,Int32,Vec4,Int32)",
    flags: MemberInfoFlags::new(793),
    module: "DiceCommonsShared",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};



pub const DRAWTEXT2D_INT32_INT32_VEC4_FLOAT32__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DrawText2d(Int32,Int32,Vec4,Float32)",
    flags: MemberInfoFlags::new(793),
    module: "DiceCommonsShared",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};



pub const DRAWCOORDINATESYSTEM_MAT4__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DrawCoordinateSystem(Mat4)",
    flags: MemberInfoFlags::new(793),
    module: "DiceCommonsShared",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ValueMatchEntityData {
    pub realm: super::core::Realm,
    pub debug_text_color: super::core::Vec3,
    pub use_external_time: bool,
    pub time_scale: f32,
    pub time_offset: f32,
    pub input_values_names: Vec<String>,
    pub match_and_trigger_array: Vec<MatchAndTriggerItem>,
}

pub const VALUEMATCHENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ValueMatchEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(ValueMatchEntityData, realm),
            },
            FieldInfoData {
                name: "DebugTextColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ValueMatchEntityData, debug_text_color),
            },
            FieldInfoData {
                name: "UseExternalTime",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ValueMatchEntityData, use_external_time),
            },
            FieldInfoData {
                name: "TimeScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ValueMatchEntityData, time_scale),
            },
            FieldInfoData {
                name: "TimeOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ValueMatchEntityData, time_offset),
            },
            FieldInfoData {
                name: "InputValuesNames",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ValueMatchEntityData, input_values_names),
            },
            FieldInfoData {
                name: "MatchAndTriggerArray",
                flags: MemberInfoFlags::new(144),
                field_type: MATCHANDTRIGGERITEM_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ValueMatchEntityData, match_and_trigger_array),
            },
        ],
    }),
    array_type: Some(VALUEMATCHENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ValueMatchEntityData {
    fn type_info() -> &'static TypeInfo {
        VALUEMATCHENTITYDATA_TYPE_INFO
    }
}


pub const VALUEMATCHENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ValueMatchEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ValueMatchEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MatchAndTriggerItem {
    pub input_values_to_match: Vec<InputValueMatch>,
    pub timed_event_triggers: Vec<TimeEventTrigger>,
    pub stop_at: f32,
}

pub const MATCHANDTRIGGERITEM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MatchAndTriggerItem",
    flags: MemberInfoFlags::new(73),
    module: "DiceCommonsShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "InputValuesToMatch",
                flags: MemberInfoFlags::new(144),
                field_type: INPUTVALUEMATCH_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MatchAndTriggerItem, input_values_to_match),
            },
            FieldInfoData {
                name: "TimedEventTriggers",
                flags: MemberInfoFlags::new(144),
                field_type: TIMEEVENTTRIGGER_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MatchAndTriggerItem, timed_event_triggers),
            },
            FieldInfoData {
                name: "StopAt",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MatchAndTriggerItem, stop_at),
            },
        ],
    }),
    array_type: Some(MATCHANDTRIGGERITEM_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MatchAndTriggerItem {
    fn type_info() -> &'static TypeInfo {
        MATCHANDTRIGGERITEM_TYPE_INFO
    }
}


pub const MATCHANDTRIGGERITEM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MatchAndTriggerItem-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("MatchAndTriggerItem-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TimeEventTrigger {
    pub output_event_hash: i32,
    pub time_to_trigger_event: f32,
}

pub const TIMEEVENTTRIGGER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimeEventTrigger",
    flags: MemberInfoFlags::new(32841),
    module: "DiceCommonsShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "OutputEventHash",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TimeEventTrigger, output_event_hash),
            },
            FieldInfoData {
                name: "TimeToTriggerEvent",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TimeEventTrigger, time_to_trigger_event),
            },
        ],
    }),
    array_type: Some(TIMEEVENTTRIGGER_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for TimeEventTrigger {
    fn type_info() -> &'static TypeInfo {
        TIMEEVENTTRIGGER_TYPE_INFO
    }
}


pub const TIMEEVENTTRIGGER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimeEventTrigger-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("TimeEventTrigger-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct InputValueMatch {
    pub input_values_names_index: i32,
    pub value_to_match: i32,
}

pub const INPUTVALUEMATCH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputValueMatch",
    flags: MemberInfoFlags::new(32841),
    module: "DiceCommonsShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "InputValuesNamesIndex",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(InputValueMatch, input_values_names_index),
            },
            FieldInfoData {
                name: "ValueToMatch",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(InputValueMatch, value_to_match),
            },
        ],
    }),
    array_type: Some(INPUTVALUEMATCH_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for InputValueMatch {
    fn type_info() -> &'static TypeInfo {
        INPUTVALUEMATCH_TYPE_INFO
    }
}


pub const INPUTVALUEMATCH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputValueMatch-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("InputValueMatch-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SimpleRotationEntityData {
    pub realm: super::core::Realm,
    pub auto_start: bool,
    pub verify_entity_and_component_links: bool,
    pub transform_modifiers: Vec<TransformModifierData>,
    pub rotation_speed_multiplier: f32,
}

pub const SIMPLEROTATIONENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimpleRotationEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(SimpleRotationEntityData, realm),
            },
            FieldInfoData {
                name: "AutoStart",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SimpleRotationEntityData, auto_start),
            },
            FieldInfoData {
                name: "VerifyEntityAndComponentLinks",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SimpleRotationEntityData, verify_entity_and_component_links),
            },
            FieldInfoData {
                name: "TransformModifiers",
                flags: MemberInfoFlags::new(144),
                field_type: TRANSFORMMODIFIERDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SimpleRotationEntityData, transform_modifiers),
            },
            FieldInfoData {
                name: "RotationSpeedMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SimpleRotationEntityData, rotation_speed_multiplier),
            },
        ],
    }),
    array_type: Some(SIMPLEROTATIONENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SimpleRotationEntityData {
    fn type_info() -> &'static TypeInfo {
        SIMPLEROTATIONENTITYDATA_TYPE_INFO
    }
}


pub const SIMPLEROTATIONENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimpleRotationEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("SimpleRotationEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PingPongRotationTransformModifierData {
    pub min_angle_in_radians: f32,
    pub max_angle_in_radians: f32,
    pub min_angle_in_degrees: f32,
    pub max_angle_in_degrees: f32,
    pub frequency: f32,
}

pub const PINGPONGROTATIONTRANSFORMMODIFIERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PingPongRotationTransformModifierData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TRANSFORMMODIFIERDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MinAngleInRadians",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PingPongRotationTransformModifierData, min_angle_in_radians),
            },
            FieldInfoData {
                name: "MaxAngleInRadians",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PingPongRotationTransformModifierData, max_angle_in_radians),
            },
            FieldInfoData {
                name: "MinAngleInDegrees",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PingPongRotationTransformModifierData, min_angle_in_degrees),
            },
            FieldInfoData {
                name: "MaxAngleInDegrees",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PingPongRotationTransformModifierData, max_angle_in_degrees),
            },
            FieldInfoData {
                name: "Frequency",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PingPongRotationTransformModifierData, frequency),
            },
        ],
    }),
    array_type: Some(PINGPONGROTATIONTRANSFORMMODIFIERDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PingPongRotationTransformModifierData {
    fn type_info() -> &'static TypeInfo {
        PINGPONGROTATIONTRANSFORMMODIFIERDATA_TYPE_INFO
    }
}


pub const PINGPONGROTATIONTRANSFORMMODIFIERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PingPongRotationTransformModifierData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("PingPongRotationTransformModifierData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RotationTransformModifierData {
    pub radians_per_second: f32,
    pub degrees_per_second: f32,
}

pub const ROTATIONTRANSFORMMODIFIERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RotationTransformModifierData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TRANSFORMMODIFIERDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "RadiansPerSecond",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RotationTransformModifierData, radians_per_second),
            },
            FieldInfoData {
                name: "DegreesPerSecond",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RotationTransformModifierData, degrees_per_second),
            },
        ],
    }),
    array_type: Some(ROTATIONTRANSFORMMODIFIERDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RotationTransformModifierData {
    fn type_info() -> &'static TypeInfo {
        ROTATIONTRANSFORMMODIFIERDATA_TYPE_INFO
    }
}


pub const ROTATIONTRANSFORMMODIFIERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RotationTransformModifierData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("RotationTransformModifierData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TransformModifierData {
    pub random_timing_to_apply: f32,
    pub rotation_axis: super::gameplay_sim::RotationAxis,
    pub rotation_axis_vec: super::core::Vec3,
}

pub const TRANSFORMMODIFIERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformModifierData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "RandomTimingToApply",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TransformModifierData, random_timing_to_apply),
            },
            FieldInfoData {
                name: "RotationAxis",
                flags: MemberInfoFlags::new(0),
                field_type: ROTATIONAXIS_TYPE_INFO,
                rust_offset: offset_of!(TransformModifierData, rotation_axis),
            },
            FieldInfoData {
                name: "RotationAxisVec",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TransformModifierData, rotation_axis_vec),
            },
        ],
    }),
    array_type: Some(TRANSFORMMODIFIERDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TransformModifierData {
    fn type_info() -> &'static TypeInfo {
        TRANSFORMMODIFIERDATA_TYPE_INFO
    }
}


pub const TRANSFORMMODIFIERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformModifierData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("TransformModifierData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum TransformModifierType {
    #[default]
    TransformModifierType_Invalid = 0,
    TransformModifierType_Rotation = 1,
    TransformModifierType_PingPongRotation = 2,
}

pub const TRANSFORMMODIFIERTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformModifierType",
    flags: MemberInfoFlags::new(49429),
    module: "DiceCommonsShared",
    data: TypeInfoData::Enum,
    array_type: Some(TRANSFORMMODIFIERTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TransformModifierType {
    fn type_info() -> &'static TypeInfo {
        TRANSFORMMODIFIERTYPE_TYPE_INFO
    }
}


pub const TRANSFORMMODIFIERTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformModifierType-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("TransformModifierType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShadowplayHighlightEntityData {
    pub award_name_string_id: super::u_i_incubator_shared::LocalizedStringId,
    pub start_delta_time: i32,
    pub end_delta_time: i32,
}

pub const SHADOWPLAYHIGHLIGHTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowplayHighlightEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "AwardNameStringId",
                flags: MemberInfoFlags::new(0),
                field_type: LOCALIZEDSTRINGID_TYPE_INFO,
                rust_offset: offset_of!(ShadowplayHighlightEntityData, award_name_string_id),
            },
            FieldInfoData {
                name: "StartDeltaTime",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ShadowplayHighlightEntityData, start_delta_time),
            },
            FieldInfoData {
                name: "EndDeltaTime",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ShadowplayHighlightEntityData, end_delta_time),
            },
        ],
    }),
    array_type: Some(SHADOWPLAYHIGHLIGHTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ShadowplayHighlightEntityData {
    fn type_info() -> &'static TypeInfo {
        SHADOWPLAYHIGHLIGHTENTITYDATA_TYPE_INFO
    }
}


pub const SHADOWPLAYHIGHLIGHTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowplayHighlightEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ShadowplayHighlightEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShadowplaySettings {
    pub enable: bool,
    pub display_summary: bool,
}

pub const SHADOWPLAYSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowplaySettings",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ShadowplaySettings, enable),
            },
            FieldInfoData {
                name: "DisplaySummary",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ShadowplaySettings, display_summary),
            },
        ],
    }),
    array_type: Some(SHADOWPLAYSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ShadowplaySettings {
    fn type_info() -> &'static TypeInfo {
        SHADOWPLAYSETTINGS_TYPE_INFO
    }
}


pub const SHADOWPLAYSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowplaySettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ShadowplaySettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SelectableActionEntityData {
    pub realm: super::core::Realm,
    pub enabled: bool,
    pub cooldown_time: f64,
    pub allowed_select_count: u32,
    pub priority: u32,
}

pub const SELECTABLEACTIONENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectableActionEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(SelectableActionEntityData, realm),
            },
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SelectableActionEntityData, enabled),
            },
            FieldInfoData {
                name: "CooldownTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT64_TYPE_INFO,
                rust_offset: offset_of!(SelectableActionEntityData, cooldown_time),
            },
            FieldInfoData {
                name: "AllowedSelectCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SelectableActionEntityData, allowed_select_count),
            },
            FieldInfoData {
                name: "Priority",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SelectableActionEntityData, priority),
            },
        ],
    }),
    array_type: Some(SELECTABLEACTIONENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SelectableActionEntityData {
    fn type_info() -> &'static TypeInfo {
        SELECTABLEACTIONENTITYDATA_TYPE_INFO
    }
}


pub const SELECTABLEACTIONENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectableActionEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("SelectableActionEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RumbleEntityData {
    pub enabled: bool,
    pub duration: f32,
    pub low: f32,
    pub high: f32,
}

pub const RUMBLEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RumbleEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RumbleEntityData, enabled),
            },
            FieldInfoData {
                name: "Duration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RumbleEntityData, duration),
            },
            FieldInfoData {
                name: "Low",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RumbleEntityData, low),
            },
            FieldInfoData {
                name: "High",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RumbleEntityData, high),
            },
        ],
    }),
    array_type: Some(RUMBLEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RumbleEntityData {
    fn type_info() -> &'static TypeInfo {
        RUMBLEENTITYDATA_TYPE_INFO
    }
}


pub const RUMBLEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RumbleEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("RumbleEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RaycastDirectionEntityData {
    pub attach: EntityAttachData,
    pub continuous_update: bool,
    pub transform: super::core::LinearTransform,
    pub ray_horizontal_angle: f32,
    pub ray_vertical_angle: f32,
    pub ray_distance: f32,
    pub lock_horizontal_rotation: bool,
    pub lock_vertical_rotation: bool,
}

pub const RAYCASTDIRECTIONENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RaycastDirectionEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Attach",
                flags: MemberInfoFlags::new(0),
                field_type: ENTITYATTACHDATA_TYPE_INFO,
                rust_offset: offset_of!(RaycastDirectionEntityData, attach),
            },
            FieldInfoData {
                name: "ContinuousUpdate",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RaycastDirectionEntityData, continuous_update),
            },
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(RaycastDirectionEntityData, transform),
            },
            FieldInfoData {
                name: "RayHorizontalAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RaycastDirectionEntityData, ray_horizontal_angle),
            },
            FieldInfoData {
                name: "RayVerticalAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RaycastDirectionEntityData, ray_vertical_angle),
            },
            FieldInfoData {
                name: "RayDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RaycastDirectionEntityData, ray_distance),
            },
            FieldInfoData {
                name: "LockHorizontalRotation",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RaycastDirectionEntityData, lock_horizontal_rotation),
            },
            FieldInfoData {
                name: "LockVerticalRotation",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RaycastDirectionEntityData, lock_vertical_rotation),
            },
        ],
    }),
    array_type: Some(RAYCASTDIRECTIONENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RaycastDirectionEntityData {
    fn type_info() -> &'static TypeInfo {
        RAYCASTDIRECTIONENTITYDATA_TYPE_INFO
    }
}


pub const RAYCASTDIRECTIONENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RaycastDirectionEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("RaycastDirectionEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RandomActionSelectorEntityData {
    pub realm: super::core::Realm,
    pub select_actions_deterministically: bool,
}

pub const RANDOMACTIONSELECTORENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomActionSelectorEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(RandomActionSelectorEntityData, realm),
            },
            FieldInfoData {
                name: "SelectActionsDeterministically",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RandomActionSelectorEntityData, select_actions_deterministically),
            },
        ],
    }),
    array_type: Some(RANDOMACTIONSELECTORENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RandomActionSelectorEntityData {
    fn type_info() -> &'static TypeInfo {
        RANDOMACTIONSELECTORENTITYDATA_TYPE_INFO
    }
}


pub const RANDOMACTIONSELECTORENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomActionSelectorEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("RandomActionSelectorEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PropertyStatusEntityData {
    pub realm: super::core::Realm,
    pub type_hash: i32,
    pub in_hash: i32,
    pub out_hash: i32,
    pub always_send_events: bool,
}

pub const PROPERTYSTATUSENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyStatusEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(PropertyStatusEntityData, realm),
            },
            FieldInfoData {
                name: "TypeHash",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PropertyStatusEntityData, type_hash),
            },
            FieldInfoData {
                name: "InHash",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PropertyStatusEntityData, in_hash),
            },
            FieldInfoData {
                name: "OutHash",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PropertyStatusEntityData, out_hash),
            },
            FieldInfoData {
                name: "AlwaysSendEvents",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PropertyStatusEntityData, always_send_events),
            },
        ],
    }),
    array_type: Some(PROPERTYSTATUSENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PropertyStatusEntityData {
    fn type_info() -> &'static TypeInfo {
        PROPERTYSTATUSENTITYDATA_TYPE_INFO
    }
}


pub const PROPERTYSTATUSENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyStatusEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("PropertyStatusEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PropertySelectEntityData {
    pub realm: super::core::Realm,
    pub type_hash: i32,
    pub input_property_hashes: Vec<i32>,
    pub input_event_hashes: Vec<i32>,
    pub out_hash: i32,
    pub input_count: u32,
    pub selected_index: i32,
    pub auto_select_on_property_changed: bool,
}

pub const PROPERTYSELECTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertySelectEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(PropertySelectEntityData, realm),
            },
            FieldInfoData {
                name: "TypeHash",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PropertySelectEntityData, type_hash),
            },
            FieldInfoData {
                name: "InputPropertyHashes",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(PropertySelectEntityData, input_property_hashes),
            },
            FieldInfoData {
                name: "InputEventHashes",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(PropertySelectEntityData, input_event_hashes),
            },
            FieldInfoData {
                name: "OutHash",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PropertySelectEntityData, out_hash),
            },
            FieldInfoData {
                name: "InputCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PropertySelectEntityData, input_count),
            },
            FieldInfoData {
                name: "SelectedIndex",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PropertySelectEntityData, selected_index),
            },
            FieldInfoData {
                name: "AutoSelectOnPropertyChanged",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PropertySelectEntityData, auto_select_on_property_changed),
            },
        ],
    }),
    array_type: Some(PROPERTYSELECTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PropertySelectEntityData {
    fn type_info() -> &'static TypeInfo {
        PROPERTYSELECTENTITYDATA_TYPE_INFO
    }
}


pub const PROPERTYSELECTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertySelectEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("PropertySelectEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PrioritizedBoolEntityData {
    pub realm: super::core::Realm,
    pub input_count: i32,
}

pub const PRIORITIZEDBOOLENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PrioritizedBoolEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(PrioritizedBoolEntityData, realm),
            },
            FieldInfoData {
                name: "InputCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PrioritizedBoolEntityData, input_count),
            },
        ],
    }),
    array_type: Some(PRIORITIZEDBOOLENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PrioritizedBoolEntityData {
    fn type_info() -> &'static TypeInfo {
        PRIORITIZEDBOOLENTITYDATA_TYPE_INFO
    }
}


pub const PRIORITIZEDBOOLENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PrioritizedBoolEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("PrioritizedBoolEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NestedConditionalPropertyEntityData {
    pub realm: super::core::Realm,
    pub type_hash: i32,
    pub evaluate_on_event: bool,
    pub condition_hashes: Vec<i32>,
    pub value_hashes: Vec<i32>,
}

pub const NESTEDCONDITIONALPROPERTYENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NestedConditionalPropertyEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(NestedConditionalPropertyEntityData, realm),
            },
            FieldInfoData {
                name: "TypeHash",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(NestedConditionalPropertyEntityData, type_hash),
            },
            FieldInfoData {
                name: "EvaluateOnEvent",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(NestedConditionalPropertyEntityData, evaluate_on_event),
            },
            FieldInfoData {
                name: "ConditionHashes",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(NestedConditionalPropertyEntityData, condition_hashes),
            },
            FieldInfoData {
                name: "ValueHashes",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(NestedConditionalPropertyEntityData, value_hashes),
            },
        ],
    }),
    array_type: Some(NESTEDCONDITIONALPROPERTYENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NestedConditionalPropertyEntityData {
    fn type_info() -> &'static TypeInfo {
        NESTEDCONDITIONALPROPERTYENTITYDATA_TYPE_INFO
    }
}


pub const NESTEDCONDITIONALPROPERTYENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NestedConditionalPropertyEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("NestedConditionalPropertyEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MultiPropertyGateEntityData {
    pub realm: super::core::Realm,
    pub runtime_properties: Vec<MultiPropertyGatePropertyInfo>,
    pub write_properties_on_open_gate: bool,
    pub open: bool,
}

pub const MULTIPROPERTYGATEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiPropertyGateEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(MultiPropertyGateEntityData, realm),
            },
            FieldInfoData {
                name: "RuntimeProperties",
                flags: MemberInfoFlags::new(144),
                field_type: MULTIPROPERTYGATEPROPERTYINFO_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MultiPropertyGateEntityData, runtime_properties),
            },
            FieldInfoData {
                name: "WritePropertiesOnOpenGate",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MultiPropertyGateEntityData, write_properties_on_open_gate),
            },
            FieldInfoData {
                name: "Open",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MultiPropertyGateEntityData, open),
            },
        ],
    }),
    array_type: Some(MULTIPROPERTYGATEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MultiPropertyGateEntityData {
    fn type_info() -> &'static TypeInfo {
        MULTIPROPERTYGATEENTITYDATA_TYPE_INFO
    }
}


pub const MULTIPROPERTYGATEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiPropertyGateEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("MultiPropertyGateEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MultiPropertyGatePropertyInfo {
    pub type_hash: u32,
    pub property_hash: u32,
}

pub const MULTIPROPERTYGATEPROPERTYINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiPropertyGatePropertyInfo",
    flags: MemberInfoFlags::new(36937),
    module: "DiceCommonsShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "TypeHash",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MultiPropertyGatePropertyInfo, type_hash),
            },
            FieldInfoData {
                name: "PropertyHash",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MultiPropertyGatePropertyInfo, property_hash),
            },
        ],
    }),
    array_type: Some(MULTIPROPERTYGATEPROPERTYINFO_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for MultiPropertyGatePropertyInfo {
    fn type_info() -> &'static TypeInfo {
        MULTIPROPERTYGATEPROPERTYINFO_TYPE_INFO
    }
}


pub const MULTIPROPERTYGATEPROPERTYINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiPropertyGatePropertyInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("MultiPropertyGatePropertyInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LogitechLEDPulseEffectEntityData {
    pub enabled: bool,
    pub color1: super::core::Vec3,
    pub color2: super::core::Vec3,
    pub duration: f32,
}

pub const LOGITECHLEDPULSEEFFECTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDPulseEffectEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LogitechLEDPulseEffectEntityData, enabled),
            },
            FieldInfoData {
                name: "Color1",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(LogitechLEDPulseEffectEntityData, color1),
            },
            FieldInfoData {
                name: "Color2",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(LogitechLEDPulseEffectEntityData, color2),
            },
            FieldInfoData {
                name: "Duration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LogitechLEDPulseEffectEntityData, duration),
            },
        ],
    }),
    array_type: Some(LOGITECHLEDPULSEEFFECTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LogitechLEDPulseEffectEntityData {
    fn type_info() -> &'static TypeInfo {
        LOGITECHLEDPULSEEFFECTENTITYDATA_TYPE_INFO
    }
}


pub const LOGITECHLEDPULSEEFFECTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDPulseEffectEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("LogitechLEDPulseEffectEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LogitechLEDInputConceptIdentifierEntityData {
    pub enabled: bool,
    pub color: super::core::Vec3,
    pub input_concept_identifiers: Vec<i32>,
}

pub const LOGITECHLEDINPUTCONCEPTIDENTIFIERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDInputConceptIdentifierEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LogitechLEDInputConceptIdentifierEntityData, enabled),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(LogitechLEDInputConceptIdentifierEntityData, color),
            },
            FieldInfoData {
                name: "InputConceptIdentifiers",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LogitechLEDInputConceptIdentifierEntityData, input_concept_identifiers),
            },
        ],
    }),
    array_type: Some(LOGITECHLEDINPUTCONCEPTIDENTIFIERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LogitechLEDInputConceptIdentifierEntityData {
    fn type_info() -> &'static TypeInfo {
        LOGITECHLEDINPUTCONCEPTIDENTIFIERENTITYDATA_TYPE_INFO
    }
}


pub const LOGITECHLEDINPUTCONCEPTIDENTIFIERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDInputConceptIdentifierEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("LogitechLEDInputConceptIdentifierEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LogitechLEDFadeEffectEntityData {
    pub enabled: bool,
    pub color1: super::core::Vec3,
    pub color2: super::core::Vec3,
    pub duration: f32,
}

pub const LOGITECHLEDFADEEFFECTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDFadeEffectEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LogitechLEDFadeEffectEntityData, enabled),
            },
            FieldInfoData {
                name: "Color1",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(LogitechLEDFadeEffectEntityData, color1),
            },
            FieldInfoData {
                name: "Color2",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(LogitechLEDFadeEffectEntityData, color2),
            },
            FieldInfoData {
                name: "Duration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LogitechLEDFadeEffectEntityData, duration),
            },
        ],
    }),
    array_type: Some(LOGITECHLEDFADEEFFECTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LogitechLEDFadeEffectEntityData {
    fn type_info() -> &'static TypeInfo {
        LOGITECHLEDFADEEFFECTENTITYDATA_TYPE_INFO
    }
}


pub const LOGITECHLEDFADEEFFECTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDFadeEffectEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("LogitechLEDFadeEffectEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LogitechLEDConstantEffectEntityData {
    pub enabled: bool,
    pub color: super::core::Vec3,
}

pub const LOGITECHLEDCONSTANTEFFECTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDConstantEffectEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LogitechLEDConstantEffectEntityData, enabled),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(LogitechLEDConstantEffectEntityData, color),
            },
        ],
    }),
    array_type: Some(LOGITECHLEDCONSTANTEFFECTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LogitechLEDConstantEffectEntityData {
    fn type_info() -> &'static TypeInfo {
        LOGITECHLEDCONSTANTEFFECTENTITYDATA_TYPE_INFO
    }
}


pub const LOGITECHLEDCONSTANTEFFECTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDConstantEffectEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("LogitechLEDConstantEffectEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LogitechLEDConditionalInputConceptIdentifierEntityData {
    pub enabled: bool,
    pub color_condition: bool,
    pub condition_true_color: super::core::Vec3,
    pub condition_false_color: super::core::Vec3,
    pub input_concept_identifiers: Vec<i32>,
}

pub const LOGITECHLEDCONDITIONALINPUTCONCEPTIDENTIFIERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDConditionalInputConceptIdentifierEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LogitechLEDConditionalInputConceptIdentifierEntityData, enabled),
            },
            FieldInfoData {
                name: "ColorCondition",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LogitechLEDConditionalInputConceptIdentifierEntityData, color_condition),
            },
            FieldInfoData {
                name: "ConditionTrueColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(LogitechLEDConditionalInputConceptIdentifierEntityData, condition_true_color),
            },
            FieldInfoData {
                name: "ConditionFalseColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(LogitechLEDConditionalInputConceptIdentifierEntityData, condition_false_color),
            },
            FieldInfoData {
                name: "InputConceptIdentifiers",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LogitechLEDConditionalInputConceptIdentifierEntityData, input_concept_identifiers),
            },
        ],
    }),
    array_type: Some(LOGITECHLEDCONDITIONALINPUTCONCEPTIDENTIFIERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LogitechLEDConditionalInputConceptIdentifierEntityData {
    fn type_info() -> &'static TypeInfo {
        LOGITECHLEDCONDITIONALINPUTCONCEPTIDENTIFIERENTITYDATA_TYPE_INFO
    }
}


pub const LOGITECHLEDCONDITIONALINPUTCONCEPTIDENTIFIERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDConditionalInputConceptIdentifierEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("LogitechLEDConditionalInputConceptIdentifierEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LogitechLEDBarEntityData {
    pub enabled: bool,
    pub value: f32,
    pub bar_color: super::core::Vec3,
    pub background_color: super::core::Vec3,
    pub input_device_keys: Vec<super::input_shared::InputDeviceKeys>,
}

pub const LOGITECHLEDBARENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDBarEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LogitechLEDBarEntityData, enabled),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LogitechLEDBarEntityData, value),
            },
            FieldInfoData {
                name: "BarColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(LogitechLEDBarEntityData, bar_color),
            },
            FieldInfoData {
                name: "BackgroundColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(LogitechLEDBarEntityData, background_color),
            },
            FieldInfoData {
                name: "InputDeviceKeys",
                flags: MemberInfoFlags::new(144),
                field_type: INPUTDEVICEKEYS_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LogitechLEDBarEntityData, input_device_keys),
            },
        ],
    }),
    array_type: Some(LOGITECHLEDBARENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LogitechLEDBarEntityData {
    fn type_info() -> &'static TypeInfo {
        LOGITECHLEDBARENTITYDATA_TYPE_INFO
    }
}


pub const LOGITECHLEDBARENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDBarEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("LogitechLEDBarEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LocalLocatorEntityData {
    pub realm: super::core::Realm,
}

pub const LOCALLOCATORENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalLocatorEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(LocalLocatorEntityData, realm),
            },
        ],
    }),
    array_type: Some(LOCALLOCATORENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LocalLocatorEntityData {
    fn type_info() -> &'static TypeInfo {
        LOCALLOCATORENTITYDATA_TYPE_INFO
    }
}


pub const LOCALLOCATORENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalLocatorEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("LocalLocatorEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WaveSelectorNodeData {
    pub output_wave: super::audio::AudioGraphNodePort,
    pub value: super::audio::AudioGraphNodePort,
    pub default_case_value: u32,
    pub input_waves: Vec<InputWavesGroup>,
}

pub const WAVESELECTORNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaveSelectorNodeData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "OutputWave",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(WaveSelectorNodeData, output_wave),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(WaveSelectorNodeData, value),
            },
            FieldInfoData {
                name: "DefaultCaseValue",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(WaveSelectorNodeData, default_case_value),
            },
            FieldInfoData {
                name: "InputWaves",
                flags: MemberInfoFlags::new(144),
                field_type: INPUTWAVESGROUP_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(WaveSelectorNodeData, input_waves),
            },
        ],
    }),
    array_type: Some(WAVESELECTORNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WaveSelectorNodeData {
    fn type_info() -> &'static TypeInfo {
        WAVESELECTORNODEDATA_TYPE_INFO
    }
}


pub const WAVESELECTORNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaveSelectorNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("WaveSelectorNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct InputWavesGroup {
    pub default_wave: super::audio::SoundWaveAssetBase,
    pub wave: super::audio::AudioGraphNodePort,
}

pub const INPUTWAVESGROUP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputWavesGroup",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEPORTGROUP_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DefaultWave",
                flags: MemberInfoFlags::new(0),
                field_type: SOUNDWAVEASSETBASE_TYPE_INFO,
                rust_offset: offset_of!(InputWavesGroup, default_wave),
            },
            FieldInfoData {
                name: "Wave",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(InputWavesGroup, wave),
            },
        ],
    }),
    array_type: Some(INPUTWAVESGROUP_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for InputWavesGroup {
    fn type_info() -> &'static TypeInfo {
        INPUTWAVESGROUP_TYPE_INFO
    }
}


pub const INPUTWAVESGROUP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputWavesGroup-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("InputWavesGroup-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ValueCacheNodeData {
    pub store: super::audio::AudioGraphNodePort,
    pub value: super::audio::AudioGraphNodePort,
    pub out: super::audio::AudioGraphNodePort,
    pub default_value: f32,
    pub set_initial_value_as_default: bool,
}

pub const VALUECACHENODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ValueCacheNodeData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Store",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ValueCacheNodeData, store),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ValueCacheNodeData, value),
            },
            FieldInfoData {
                name: "Out",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ValueCacheNodeData, out),
            },
            FieldInfoData {
                name: "DefaultValue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ValueCacheNodeData, default_value),
            },
            FieldInfoData {
                name: "SetInitialValueAsDefault",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ValueCacheNodeData, set_initial_value_as_default),
            },
        ],
    }),
    array_type: Some(VALUECACHENODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ValueCacheNodeData {
    fn type_info() -> &'static TypeInfo {
        VALUECACHENODEDATA_TYPE_INFO
    }
}


pub const VALUECACHENODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ValueCacheNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ValueCacheNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SystemInformationNodeData {
    pub channel_count: super::audio::AudioGraphNodePort,
}

pub const SYSTEMINFORMATIONNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SystemInformationNodeData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ChannelCount",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(SystemInformationNodeData, channel_count),
            },
        ],
    }),
    array_type: Some(SYSTEMINFORMATIONNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SystemInformationNodeData {
    fn type_info() -> &'static TypeInfo {
        SYSTEMINFORMATIONNODEDATA_TYPE_INFO
    }
}


pub const SYSTEMINFORMATIONNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SystemInformationNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("SystemInformationNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SyncedRandomIntNodeData {
    pub trigger: super::audio::AudioGraphNodePort,
    pub triggered: super::audio::AudioGraphNodePort,
    pub out: super::audio::AudioGraphNodePort,
    pub min: i32,
    pub max: i32,
    pub high_precision: bool,
}

pub const SYNCEDRANDOMINTNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SyncedRandomIntNodeData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Trigger",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(SyncedRandomIntNodeData, trigger),
            },
            FieldInfoData {
                name: "Triggered",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(SyncedRandomIntNodeData, triggered),
            },
            FieldInfoData {
                name: "Out",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(SyncedRandomIntNodeData, out),
            },
            FieldInfoData {
                name: "Min",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(SyncedRandomIntNodeData, min),
            },
            FieldInfoData {
                name: "Max",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(SyncedRandomIntNodeData, max),
            },
            FieldInfoData {
                name: "HighPrecision",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SyncedRandomIntNodeData, high_precision),
            },
        ],
    }),
    array_type: Some(SYNCEDRANDOMINTNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SyncedRandomIntNodeData {
    fn type_info() -> &'static TypeInfo {
        SYNCEDRANDOMINTNODEDATA_TYPE_INFO
    }
}


pub const SYNCEDRANDOMINTNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SyncedRandomIntNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("SyncedRandomIntNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct StepLogicSamplerNodeConfigData {
    pub amplitude: f32,
    pub pitch: f32,
    pub rate_of_fire: f32,
    pub wave: super::audio::SoundWaveAssetBase,
    pub maximum_rate_of_fire: f32,
    pub fade_type: super::audio::GainFaderFadeType,
}

pub const STEPLOGICSAMPLERNODECONFIGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StepLogicSamplerNodeConfigData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODECONFIGDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Amplitude",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(StepLogicSamplerNodeConfigData, amplitude),
            },
            FieldInfoData {
                name: "Pitch",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(StepLogicSamplerNodeConfigData, pitch),
            },
            FieldInfoData {
                name: "RateOfFire",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(StepLogicSamplerNodeConfigData, rate_of_fire),
            },
            FieldInfoData {
                name: "Wave",
                flags: MemberInfoFlags::new(0),
                field_type: SOUNDWAVEASSETBASE_TYPE_INFO,
                rust_offset: offset_of!(StepLogicSamplerNodeConfigData, wave),
            },
            FieldInfoData {
                name: "MaximumRateOfFire",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(StepLogicSamplerNodeConfigData, maximum_rate_of_fire),
            },
            FieldInfoData {
                name: "FadeType",
                flags: MemberInfoFlags::new(0),
                field_type: GAINFADERFADETYPE_TYPE_INFO,
                rust_offset: offset_of!(StepLogicSamplerNodeConfigData, fade_type),
            },
        ],
    }),
    array_type: Some(STEPLOGICSAMPLERNODECONFIGDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StepLogicSamplerNodeConfigData {
    fn type_info() -> &'static TypeInfo {
        STEPLOGICSAMPLERNODECONFIGDATA_TYPE_INFO
    }
}


pub const STEPLOGICSAMPLERNODECONFIGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StepLogicSamplerNodeConfigData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("StepLogicSamplerNodeConfigData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct StepLogicSamplerNodeData {
    pub external_wave: super::audio::AudioGraphNodePort,
    pub pitch: super::audio::AudioGraphNodePort,
    pub amplitude: super::audio::AudioGraphNodePort,
    pub rate_of_fire: super::audio::AudioGraphNodePort,
    pub trigger: super::audio::AudioGraphNodePort,
    pub release: super::audio::AudioGraphNodePort,
    pub output: super::audio::AudioGraphNodePort,
    pub finished: super::audio::AudioGraphNodePort,
    pub wave: super::audio::SoundWaveAssetBase,
    pub plugins: Vec<StepLogicSamplerPlugins>,
    pub pitch_source: super::audio::OutputNodeData,
    pub maximum_rate_of_fire: f32,
    pub fade_type: super::audio::GainFaderFadeType,
    pub sampler_node_debug: StepLogicSamplerNodeDebugData,
}

pub const STEPLOGICSAMPLERNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StepLogicSamplerNodeData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ExternalWave",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(StepLogicSamplerNodeData, external_wave),
            },
            FieldInfoData {
                name: "Pitch",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(StepLogicSamplerNodeData, pitch),
            },
            FieldInfoData {
                name: "Amplitude",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(StepLogicSamplerNodeData, amplitude),
            },
            FieldInfoData {
                name: "RateOfFire",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(StepLogicSamplerNodeData, rate_of_fire),
            },
            FieldInfoData {
                name: "Trigger",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(StepLogicSamplerNodeData, trigger),
            },
            FieldInfoData {
                name: "Release",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(StepLogicSamplerNodeData, release),
            },
            FieldInfoData {
                name: "Output",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(StepLogicSamplerNodeData, output),
            },
            FieldInfoData {
                name: "Finished",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(StepLogicSamplerNodeData, finished),
            },
            FieldInfoData {
                name: "Wave",
                flags: MemberInfoFlags::new(0),
                field_type: SOUNDWAVEASSETBASE_TYPE_INFO,
                rust_offset: offset_of!(StepLogicSamplerNodeData, wave),
            },
            FieldInfoData {
                name: "Plugins",
                flags: MemberInfoFlags::new(144),
                field_type: STEPLOGICSAMPLERPLUGINS_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(StepLogicSamplerNodeData, plugins),
            },
            FieldInfoData {
                name: "PitchSource",
                flags: MemberInfoFlags::new(0),
                field_type: OUTPUTNODEDATA_TYPE_INFO,
                rust_offset: offset_of!(StepLogicSamplerNodeData, pitch_source),
            },
            FieldInfoData {
                name: "MaximumRateOfFire",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(StepLogicSamplerNodeData, maximum_rate_of_fire),
            },
            FieldInfoData {
                name: "FadeType",
                flags: MemberInfoFlags::new(0),
                field_type: GAINFADERFADETYPE_TYPE_INFO,
                rust_offset: offset_of!(StepLogicSamplerNodeData, fade_type),
            },
            FieldInfoData {
                name: "SamplerNodeDebug",
                flags: MemberInfoFlags::new(0),
                field_type: STEPLOGICSAMPLERNODEDEBUGDATA_TYPE_INFO,
                rust_offset: offset_of!(StepLogicSamplerNodeData, sampler_node_debug),
            },
        ],
    }),
    array_type: Some(STEPLOGICSAMPLERNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StepLogicSamplerNodeData {
    fn type_info() -> &'static TypeInfo {
        STEPLOGICSAMPLERNODEDATA_TYPE_INFO
    }
}


pub const STEPLOGICSAMPLERNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StepLogicSamplerNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("StepLogicSamplerNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct StepLogicSamplerNodeDebugData {
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

pub const STEPLOGICSAMPLERNODEDEBUGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StepLogicSamplerNodeDebugData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "EnableDebug",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(StepLogicSamplerNodeDebugData, enable_debug),
            },
            FieldInfoData {
                name: "DebugTextXPos",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(StepLogicSamplerNodeDebugData, debug_text_x_pos),
            },
            FieldInfoData {
                name: "DebugTextYPos",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(StepLogicSamplerNodeDebugData, debug_text_y_pos),
            },
            FieldInfoData {
                name: "EventDisplayTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(StepLogicSamplerNodeDebugData, event_display_time),
            },
            FieldInfoData {
                name: "SamplerDebugInfoColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(StepLogicSamplerNodeDebugData, sampler_debug_info_color),
            },
            FieldInfoData {
                name: "PropertiesDebugInfoColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(StepLogicSamplerNodeDebugData, properties_debug_info_color),
            },
            FieldInfoData {
                name: "EventsDebugInfoColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(StepLogicSamplerNodeDebugData, events_debug_info_color),
            },
            FieldInfoData {
                name: "ExternalWaveDebugInfoColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(StepLogicSamplerNodeDebugData, external_wave_debug_info_color),
            },
            FieldInfoData {
                name: "MuteSampler",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(StepLogicSamplerNodeDebugData, mute_sampler),
            },
        ],
    }),
    array_type: Some(STEPLOGICSAMPLERNODEDEBUGDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for StepLogicSamplerNodeDebugData {
    fn type_info() -> &'static TypeInfo {
        STEPLOGICSAMPLERNODEDEBUGDATA_TYPE_INFO
    }
}


pub const STEPLOGICSAMPLERNODEDEBUGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StepLogicSamplerNodeDebugData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("StepLogicSamplerNodeDebugData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StepLogicSamplerPlugins {
    pub snd_player: super::audio::SoundGraphPluginRef,
    pub resample: super::audio::SoundGraphPluginRef,
    pub pause: super::audio::SoundGraphPluginRef,
    pub gain: super::audio::SoundGraphPluginRef,
    pub gain_fader: super::audio::SoundGraphPluginRef,
}

pub const STEPLOGICSAMPLERPLUGINS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StepLogicSamplerPlugins",
    flags: MemberInfoFlags::new(36937),
    module: "DiceCommonsShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "SndPlayer",
                flags: MemberInfoFlags::new(0),
                field_type: SOUNDGRAPHPLUGINREF_TYPE_INFO,
                rust_offset: offset_of!(StepLogicSamplerPlugins, snd_player),
            },
            FieldInfoData {
                name: "Resample",
                flags: MemberInfoFlags::new(0),
                field_type: SOUNDGRAPHPLUGINREF_TYPE_INFO,
                rust_offset: offset_of!(StepLogicSamplerPlugins, resample),
            },
            FieldInfoData {
                name: "Pause",
                flags: MemberInfoFlags::new(0),
                field_type: SOUNDGRAPHPLUGINREF_TYPE_INFO,
                rust_offset: offset_of!(StepLogicSamplerPlugins, pause),
            },
            FieldInfoData {
                name: "Gain",
                flags: MemberInfoFlags::new(0),
                field_type: SOUNDGRAPHPLUGINREF_TYPE_INFO,
                rust_offset: offset_of!(StepLogicSamplerPlugins, gain),
            },
            FieldInfoData {
                name: "GainFader",
                flags: MemberInfoFlags::new(0),
                field_type: SOUNDGRAPHPLUGINREF_TYPE_INFO,
                rust_offset: offset_of!(StepLogicSamplerPlugins, gain_fader),
            },
        ],
    }),
    array_type: Some(STEPLOGICSAMPLERPLUGINS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for StepLogicSamplerPlugins {
    fn type_info() -> &'static TypeInfo {
        STEPLOGICSAMPLERPLUGINS_TYPE_INFO
    }
}


pub const STEPLOGICSAMPLERPLUGINS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StepLogicSamplerPlugins-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("StepLogicSamplerPlugins-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SortValuesNodeData {
    pub sort_values: Vec<SortValuesGroup>,
}

pub const SORTVALUESNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SortValuesNodeData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "SortValues",
                flags: MemberInfoFlags::new(144),
                field_type: SORTVALUESGROUP_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SortValuesNodeData, sort_values),
            },
        ],
    }),
    array_type: Some(SORTVALUESNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SortValuesNodeData {
    fn type_info() -> &'static TypeInfo {
        SORTVALUESNODEDATA_TYPE_INFO
    }
}


pub const SORTVALUESNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SortValuesNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("SortValuesNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SortValuesGroup {
    pub r#in: super::audio::AudioGraphNodePort,
    pub sorted_value: super::audio::AudioGraphNodePort,
    pub input_index: super::audio::AudioGraphNodePort,
}

pub const SORTVALUESGROUP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SortValuesGroup",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEPORTGROUP_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "In",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(SortValuesGroup, r#in),
            },
            FieldInfoData {
                name: "SortedValue",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(SortValuesGroup, sorted_value),
            },
            FieldInfoData {
                name: "InputIndex",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(SortValuesGroup, input_index),
            },
        ],
    }),
    array_type: Some(SORTVALUESGROUP_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SortValuesGroup {
    fn type_info() -> &'static TypeInfo {
        SORTVALUESGROUP_TYPE_INFO
    }
}


pub const SORTVALUESGROUP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SortValuesGroup-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("SortValuesGroup-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SequencerNodeData {
    pub start: super::audio::AudioGraphNodePort,
    pub stop: super::audio::AudioGraphNodePort,
    pub time: super::audio::AudioGraphNodePort,
    pub on_started: super::audio::AudioGraphNodePort,
    pub on_stopped: super::audio::AudioGraphNodePort,
    pub out: super::audio::AudioGraphNodePort,
    pub delay: super::audio::AudioGraphNodePort,
    pub time_mode: TimeMode,
}

pub const SEQUENCERNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SequencerNodeData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Start",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(SequencerNodeData, start),
            },
            FieldInfoData {
                name: "Stop",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(SequencerNodeData, stop),
            },
            FieldInfoData {
                name: "Time",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(SequencerNodeData, time),
            },
            FieldInfoData {
                name: "OnStarted",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(SequencerNodeData, on_started),
            },
            FieldInfoData {
                name: "OnStopped",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(SequencerNodeData, on_stopped),
            },
            FieldInfoData {
                name: "Out",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(SequencerNodeData, out),
            },
            FieldInfoData {
                name: "Delay",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(SequencerNodeData, delay),
            },
            FieldInfoData {
                name: "TimeMode",
                flags: MemberInfoFlags::new(0),
                field_type: TIMEMODE_TYPE_INFO,
                rust_offset: offset_of!(SequencerNodeData, time_mode),
            },
        ],
    }),
    array_type: Some(SEQUENCERNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SequencerNodeData {
    fn type_info() -> &'static TypeInfo {
        SEQUENCERNODEDATA_TYPE_INFO
    }
}


pub const SEQUENCERNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SequencerNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("SequencerNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum TimeMode {
    #[default]
    TimeMode_Frequency = 0,
    TimeMode_Period = 1,
}

pub const TIMEMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimeMode",
    flags: MemberInfoFlags::new(49429),
    module: "DiceCommonsShared",
    data: TypeInfoData::Enum,
    array_type: Some(TIMEMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TimeMode {
    fn type_info() -> &'static TypeInfo {
        TIMEMODE_TYPE_INFO
    }
}


pub const TIMEMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimeMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("TimeMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SaturationNodeData {
    pub r#in: super::audio::AudioGraphNodePort,
    pub gain: super::audio::AudioGraphNodePort,
    pub d_c_bias: super::audio::AudioGraphNodePort,
    pub level: super::audio::AudioGraphNodePort,
    pub mix: super::audio::AudioGraphNodePort,
    pub saturation_type: SaturationTypes,
    pub out: super::audio::AudioGraphNodePort,
    pub saturation_plugin: super::audio::SoundGraphPluginRef,
}

pub const SATURATIONNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SaturationNodeData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "In",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(SaturationNodeData, r#in),
            },
            FieldInfoData {
                name: "Gain",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(SaturationNodeData, gain),
            },
            FieldInfoData {
                name: "DCBias",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(SaturationNodeData, d_c_bias),
            },
            FieldInfoData {
                name: "Level",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(SaturationNodeData, level),
            },
            FieldInfoData {
                name: "Mix",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(SaturationNodeData, mix),
            },
            FieldInfoData {
                name: "SaturationType",
                flags: MemberInfoFlags::new(0),
                field_type: SATURATIONTYPES_TYPE_INFO,
                rust_offset: offset_of!(SaturationNodeData, saturation_type),
            },
            FieldInfoData {
                name: "Out",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(SaturationNodeData, out),
            },
            FieldInfoData {
                name: "SaturationPlugin",
                flags: MemberInfoFlags::new(0),
                field_type: SOUNDGRAPHPLUGINREF_TYPE_INFO,
                rust_offset: offset_of!(SaturationNodeData, saturation_plugin),
            },
        ],
    }),
    array_type: Some(SATURATIONNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SaturationNodeData {
    fn type_info() -> &'static TypeInfo {
        SATURATIONNODEDATA_TYPE_INFO
    }
}


pub const SATURATIONNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SaturationNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("SaturationNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
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

pub const SATURATIONTYPES_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SaturationTypes",
    flags: MemberInfoFlags::new(49429),
    module: "DiceCommonsShared",
    data: TypeInfoData::Enum,
    array_type: Some(SATURATIONTYPES_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SaturationTypes {
    fn type_info() -> &'static TypeInfo {
        SATURATIONTYPES_TYPE_INFO
    }
}


pub const SATURATIONTYPES_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SaturationTypes-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("SaturationTypes-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RunOnceNodeData {
    pub r#in: super::audio::AudioGraphNodePort,
    pub reset: super::audio::AudioGraphNodePort,
    pub out: super::audio::AudioGraphNodePort,
    pub start_closed: bool,
}

pub const RUNONCENODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RunOnceNodeData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "In",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(RunOnceNodeData, r#in),
            },
            FieldInfoData {
                name: "Reset",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(RunOnceNodeData, reset),
            },
            FieldInfoData {
                name: "Out",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(RunOnceNodeData, out),
            },
            FieldInfoData {
                name: "StartClosed",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RunOnceNodeData, start_closed),
            },
        ],
    }),
    array_type: Some(RUNONCENODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RunOnceNodeData {
    fn type_info() -> &'static TypeInfo {
        RUNONCENODEDATA_TYPE_INFO
    }
}


pub const RUNONCENODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RunOnceNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("RunOnceNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RaycastNodeData {
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

pub const RAYCASTNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RaycastNodeData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Send",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(RaycastNodeData, send),
            },
            FieldInfoData {
                name: "Hit",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(RaycastNodeData, hit),
            },
            FieldInfoData {
                name: "Miss",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(RaycastNodeData, miss),
            },
            FieldInfoData {
                name: "Left",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RaycastNodeData, left),
            },
            FieldInfoData {
                name: "Up",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RaycastNodeData, up),
            },
            FieldInfoData {
                name: "Forward",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RaycastNodeData, forward),
            },
            FieldInfoData {
                name: "SeeThrough",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RaycastNodeData, see_through),
            },
            FieldInfoData {
                name: "Penetrable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RaycastNodeData, penetrable),
            },
            FieldInfoData {
                name: "IncludeTerrain",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RaycastNodeData, include_terrain),
            },
            FieldInfoData {
                name: "IncludeWater",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RaycastNodeData, include_water),
            },
            FieldInfoData {
                name: "IncludeCharacters",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RaycastNodeData, include_characters),
            },
            FieldInfoData {
                name: "IncludeVehicles",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RaycastNodeData, include_vehicles),
            },
            FieldInfoData {
                name: "IncludeRagdolls",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RaycastNodeData, include_ragdolls),
            },
            FieldInfoData {
                name: "IncludeFixed",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RaycastNodeData, include_fixed),
            },
            FieldInfoData {
                name: "IncludeKeyframed",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RaycastNodeData, include_keyframed),
            },
            FieldInfoData {
                name: "IncludeDynamic",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RaycastNodeData, include_dynamic),
            },
            FieldInfoData {
                name: "DetailedQueryMode",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RaycastNodeData, detailed_query_mode),
            },
            FieldInfoData {
                name: "EnableDebug",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RaycastNodeData, enable_debug),
            },
            FieldInfoData {
                name: "MaterialId",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(RaycastNodeData, material_id),
            },
        ],
    }),
    array_type: Some(RAYCASTNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RaycastNodeData {
    fn type_info() -> &'static TypeInfo {
        RAYCASTNODEDATA_TYPE_INFO
    }
}


pub const RAYCASTNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RaycastNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("RaycastNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RandomIntegerNodeData {
    pub trigger: super::audio::AudioGraphNodePort,
    pub out: super::audio::AudioGraphNodePort,
    pub min: i32,
    pub max: i32,
}

pub const RANDOMINTEGERNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomIntegerNodeData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Trigger",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(RandomIntegerNodeData, trigger),
            },
            FieldInfoData {
                name: "Out",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(RandomIntegerNodeData, out),
            },
            FieldInfoData {
                name: "Min",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(RandomIntegerNodeData, min),
            },
            FieldInfoData {
                name: "Max",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(RandomIntegerNodeData, max),
            },
        ],
    }),
    array_type: Some(RANDOMINTEGERNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RandomIntegerNodeData {
    fn type_info() -> &'static TypeInfo {
        RANDOMINTEGERNODEDATA_TYPE_INFO
    }
}


pub const RANDOMINTEGERNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomIntegerNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("RandomIntegerNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ProfileOptionsReaderNodeData {
    pub profile_options: Vec<ProfileOptionsGroup>,
}

pub const PROFILEOPTIONSREADERNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionsReaderNodeData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ProfileOptions",
                flags: MemberInfoFlags::new(144),
                field_type: PROFILEOPTIONSGROUP_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ProfileOptionsReaderNodeData, profile_options),
            },
        ],
    }),
    array_type: Some(PROFILEOPTIONSREADERNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProfileOptionsReaderNodeData {
    fn type_info() -> &'static TypeInfo {
        PROFILEOPTIONSREADERNODEDATA_TYPE_INFO
    }
}


pub const PROFILEOPTIONSREADERNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionsReaderNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ProfileOptionsReaderNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ProfileOptionsGroup {
    pub profile_option_asset: super::gameplay_sim::ProfileOptionData,
    pub profile_option_value: super::audio::AudioGraphNodePort,
}

pub const PROFILEOPTIONSGROUP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionsGroup",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEPORTGROUP_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ProfileOptionAsset",
                flags: MemberInfoFlags::new(0),
                field_type: PROFILEOPTIONDATA_TYPE_INFO,
                rust_offset: offset_of!(ProfileOptionsGroup, profile_option_asset),
            },
            FieldInfoData {
                name: "ProfileOptionValue",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ProfileOptionsGroup, profile_option_value),
            },
        ],
    }),
    array_type: Some(PROFILEOPTIONSGROUP_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProfileOptionsGroup {
    fn type_info() -> &'static TypeInfo {
        PROFILEOPTIONSGROUP_TYPE_INFO
    }
}


pub const PROFILEOPTIONSGROUP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileOptionsGroup-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ProfileOptionsGroup-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PassbyDetectorNodeData {
    pub time_to_apex: super::audio::AudioGraphNodePort,
    pub extra_distance: super::audio::AudioGraphNodePort,
    pub speed_threshold: super::audio::AudioGraphNodePort,
    pub relative_speed_threshold: super::audio::AudioGraphNodePort,
    pub cooldown_time: super::audio::AudioGraphNodePort,
    pub trigger: super::audio::AudioGraphNodePort,
    pub cancel: super::audio::AudioGraphNodePort,
    pub relative_speed_smoothing_rate: f32,
}

pub const PASSBYDETECTORNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PassbyDetectorNodeData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "TimeToApex",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(PassbyDetectorNodeData, time_to_apex),
            },
            FieldInfoData {
                name: "ExtraDistance",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(PassbyDetectorNodeData, extra_distance),
            },
            FieldInfoData {
                name: "SpeedThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(PassbyDetectorNodeData, speed_threshold),
            },
            FieldInfoData {
                name: "RelativeSpeedThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(PassbyDetectorNodeData, relative_speed_threshold),
            },
            FieldInfoData {
                name: "CooldownTime",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(PassbyDetectorNodeData, cooldown_time),
            },
            FieldInfoData {
                name: "Trigger",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(PassbyDetectorNodeData, trigger),
            },
            FieldInfoData {
                name: "Cancel",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(PassbyDetectorNodeData, cancel),
            },
            FieldInfoData {
                name: "RelativeSpeedSmoothingRate",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PassbyDetectorNodeData, relative_speed_smoothing_rate),
            },
        ],
    }),
    array_type: Some(PASSBYDETECTORNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PassbyDetectorNodeData {
    fn type_info() -> &'static TypeInfo {
        PASSBYDETECTORNODEDATA_TYPE_INFO
    }
}


pub const PASSBYDETECTORNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PassbyDetectorNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("PassbyDetectorNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ParameterSmootherNodeData {
    pub r#in: super::audio::AudioGraphNodePort,
    pub out: super::audio::AudioGraphNodePort,
    pub smoothing_time: super::audio::AudioGraphNodePort,
}

pub const PARAMETERSMOOTHERNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ParameterSmootherNodeData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "In",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ParameterSmootherNodeData, r#in),
            },
            FieldInfoData {
                name: "Out",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ParameterSmootherNodeData, out),
            },
            FieldInfoData {
                name: "SmoothingTime",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ParameterSmootherNodeData, smoothing_time),
            },
        ],
    }),
    array_type: Some(PARAMETERSMOOTHERNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ParameterSmootherNodeData {
    fn type_info() -> &'static TypeInfo {
        PARAMETERSMOOTHERNODEDATA_TYPE_INFO
    }
}


pub const PARAMETERSMOOTHERNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ParameterSmootherNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ParameterSmootherNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct NanCheckNodeData {
    pub r#in: super::audio::AudioGraphNodePort,
    pub out: super::audio::AudioGraphNodePort,
    pub name_non_meta: String,
    pub nan_check_plugin: super::audio::SoundGraphPluginRef,
}

pub const NANCHECKNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NanCheckNodeData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "In",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(NanCheckNodeData, r#in),
            },
            FieldInfoData {
                name: "Out",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(NanCheckNodeData, out),
            },
            FieldInfoData {
                name: "NameNonMeta",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(NanCheckNodeData, name_non_meta),
            },
            FieldInfoData {
                name: "NanCheckPlugin",
                flags: MemberInfoFlags::new(0),
                field_type: SOUNDGRAPHPLUGINREF_TYPE_INFO,
                rust_offset: offset_of!(NanCheckNodeData, nan_check_plugin),
            },
        ],
    }),
    array_type: Some(NANCHECKNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NanCheckNodeData {
    fn type_info() -> &'static TypeInfo {
        NANCHECKNODEDATA_TYPE_INFO
    }
}


pub const NANCHECKNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NanCheckNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("NanCheckNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LoudnessNodeData {
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

pub const LOUDNESSNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LoudnessNodeData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "In",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(LoudnessNodeData, r#in),
            },
            FieldInfoData {
                name: "Out",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(LoudnessNodeData, out),
            },
            FieldInfoData {
                name: "Momentary",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(LoudnessNodeData, momentary),
            },
            FieldInfoData {
                name: "Integrated",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(LoudnessNodeData, integrated),
            },
            FieldInfoData {
                name: "Reset",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(LoudnessNodeData, reset),
            },
            FieldInfoData {
                name: "Plugin",
                flags: MemberInfoFlags::new(0),
                field_type: SOUNDGRAPHPLUGINREF_TYPE_INFO,
                rust_offset: offset_of!(LoudnessNodeData, plugin),
            },
            FieldInfoData {
                name: "Trace",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LoudnessNodeData, trace),
            },
            FieldInfoData {
                name: "TraceLabel",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(LoudnessNodeData, trace_label),
            },
            FieldInfoData {
                name: "IntegrationTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LoudnessNodeData, integration_time),
            },
            FieldInfoData {
                name: "EnableClamp",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LoudnessNodeData, enable_clamp),
            },
        ],
    }),
    array_type: Some(LOUDNESSNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LoudnessNodeData {
    fn type_info() -> &'static TypeInfo {
        LOUDNESSNODEDATA_TYPE_INFO
    }
}


pub const LOUDNESSNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LoudnessNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("LoudnessNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ListenerNodeData {
    pub world_x: super::audio::AudioGraphNodePort,
    pub world_y: super::audio::AudioGraphNodePort,
    pub world_z: super::audio::AudioGraphNodePort,
}

pub const LISTENERNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ListenerNodeData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "WorldX",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ListenerNodeData, world_x),
            },
            FieldInfoData {
                name: "WorldY",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ListenerNodeData, world_y),
            },
            FieldInfoData {
                name: "WorldZ",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ListenerNodeData, world_z),
            },
        ],
    }),
    array_type: Some(LISTENERNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ListenerNodeData {
    fn type_info() -> &'static TypeInfo {
        LISTENERNODEDATA_TYPE_INFO
    }
}


pub const LISTENERNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ListenerNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ListenerNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IndexMapperNodeConfigData {
    pub dimension_groups: Vec<IndexMapperConfigGroup>,
}

pub const INDEXMAPPERNODECONFIGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IndexMapperNodeConfigData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODECONFIGDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DimensionGroups",
                flags: MemberInfoFlags::new(144),
                field_type: INDEXMAPPERCONFIGGROUP_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(IndexMapperNodeConfigData, dimension_groups),
            },
        ],
    }),
    array_type: Some(INDEXMAPPERNODECONFIGDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IndexMapperNodeConfigData {
    fn type_info() -> &'static TypeInfo {
        INDEXMAPPERNODECONFIGDATA_TYPE_INFO
    }
}


pub const INDEXMAPPERNODECONFIGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IndexMapperNodeConfigData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("IndexMapperNodeConfigData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IndexMapperConfigGroup {
    pub dimension_size: u32,
}

pub const INDEXMAPPERCONFIGGROUP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IndexMapperConfigGroup",
    flags: MemberInfoFlags::new(36937),
    module: "DiceCommonsShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "DimensionSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(IndexMapperConfigGroup, dimension_size),
            },
        ],
    }),
    array_type: Some(INDEXMAPPERCONFIGGROUP_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for IndexMapperConfigGroup {
    fn type_info() -> &'static TypeInfo {
        INDEXMAPPERCONFIGGROUP_TYPE_INFO
    }
}


pub const INDEXMAPPERCONFIGGROUP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IndexMapperConfigGroup-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("IndexMapperConfigGroup-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct IndexMapperNodeData {
    pub dimension_groups: Vec<IndexMapperGroup>,
    pub result: super::audio::AudioGraphNodePort,
}

pub const INDEXMAPPERNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IndexMapperNodeData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DimensionGroups",
                flags: MemberInfoFlags::new(144),
                field_type: INDEXMAPPERGROUP_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(IndexMapperNodeData, dimension_groups),
            },
            FieldInfoData {
                name: "Result",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(IndexMapperNodeData, result),
            },
        ],
    }),
    array_type: Some(INDEXMAPPERNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IndexMapperNodeData {
    fn type_info() -> &'static TypeInfo {
        INDEXMAPPERNODEDATA_TYPE_INFO
    }
}


pub const INDEXMAPPERNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IndexMapperNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("IndexMapperNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct IndexMapperGroup {
    pub guid: super::core::Guid,
    pub dimension_size: u32,
    pub r#in: super::audio::AudioGraphNodePort,
}

pub const INDEXMAPPERGROUP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IndexMapperGroup",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEPORTGROUP_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Guid",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(IndexMapperGroup, guid),
            },
            FieldInfoData {
                name: "DimensionSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(IndexMapperGroup, dimension_size),
            },
            FieldInfoData {
                name: "In",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(IndexMapperGroup, r#in),
            },
        ],
    }),
    array_type: Some(INDEXMAPPERGROUP_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IndexMapperGroup {
    fn type_info() -> &'static TypeInfo {
        INDEXMAPPERGROUP_TYPE_INFO
    }
}


pub const INDEXMAPPERGROUP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IndexMapperGroup-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("IndexMapperGroup-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct GateNodeData {
    pub r#in: super::audio::AudioGraphNodePort,
    pub open: super::audio::AudioGraphNodePort,
    pub close: super::audio::AudioGraphNodePort,
    pub out: super::audio::AudioGraphNodePort,
    pub start_opened: bool,
}

pub const GATENODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GateNodeData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "In",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(GateNodeData, r#in),
            },
            FieldInfoData {
                name: "Open",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(GateNodeData, open),
            },
            FieldInfoData {
                name: "Close",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(GateNodeData, close),
            },
            FieldInfoData {
                name: "Out",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(GateNodeData, out),
            },
            FieldInfoData {
                name: "StartOpened",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GateNodeData, start_opened),
            },
        ],
    }),
    array_type: Some(GATENODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GateNodeData {
    fn type_info() -> &'static TypeInfo {
        GATENODEDATA_TYPE_INFO
    }
}


pub const GATENODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GateNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("GateNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ForceIsNotLoopingNodeData {
}

pub const FORCEISNOTLOOPINGNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForceIsNotLoopingNodeData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FORCEISNOTLOOPINGNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ForceIsNotLoopingNodeData {
    fn type_info() -> &'static TypeInfo {
        FORCEISNOTLOOPINGNODEDATA_TYPE_INFO
    }
}


pub const FORCEISNOTLOOPINGNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForceIsNotLoopingNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ForceIsNotLoopingNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ForceIsLoopingNodeData {
}

pub const FORCEISLOOPINGNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForceIsLoopingNodeData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FORCEISLOOPINGNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ForceIsLoopingNodeData {
    fn type_info() -> &'static TypeInfo {
        FORCEISLOOPINGNODEDATA_TYPE_INFO
    }
}


pub const FORCEISLOOPINGNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForceIsLoopingNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ForceIsLoopingNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EventSelectorNodeData {
    pub index: super::audio::AudioGraphNodePort,
    pub input_events: Vec<InputEventsGroup>,
    pub out: super::audio::AudioGraphNodePort,
}

pub const EVENTSELECTORNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventSelectorNodeData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Index",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(EventSelectorNodeData, index),
            },
            FieldInfoData {
                name: "InputEvents",
                flags: MemberInfoFlags::new(144),
                field_type: INPUTEVENTSGROUP_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EventSelectorNodeData, input_events),
            },
            FieldInfoData {
                name: "Out",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(EventSelectorNodeData, out),
            },
        ],
    }),
    array_type: Some(EVENTSELECTORNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EventSelectorNodeData {
    fn type_info() -> &'static TypeInfo {
        EVENTSELECTORNODEDATA_TYPE_INFO
    }
}


pub const EVENTSELECTORNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventSelectorNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("EventSelectorNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct InputEventsGroup {
    pub r#in: super::audio::AudioGraphNodePort,
}

pub const INPUTEVENTSGROUP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputEventsGroup",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEPORTGROUP_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "In",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(InputEventsGroup, r#in),
            },
        ],
    }),
    array_type: Some(INPUTEVENTSGROUP_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for InputEventsGroup {
    fn type_info() -> &'static TypeInfo {
        INPUTEVENTSGROUP_TYPE_INFO
    }
}


pub const INPUTEVENTSGROUP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputEventsGroup-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("InputEventsGroup-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EventGateConditionValueNodeData {
    pub condition: EventGateConditionValueType,
    pub r#in: super::audio::AudioGraphNodePort,
    pub in_value: super::audio::AudioGraphNodePort,
    pub cool_down_time: super::audio::AudioGraphNodePort,
    pub enable: super::audio::AudioGraphNodePort,
    pub out: super::audio::AudioGraphNodePort,
    pub out_value: super::audio::AudioGraphNodePort,
}

pub const EVENTGATECONDITIONVALUENODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventGateConditionValueNodeData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Condition",
                flags: MemberInfoFlags::new(0),
                field_type: EVENTGATECONDITIONVALUETYPE_TYPE_INFO,
                rust_offset: offset_of!(EventGateConditionValueNodeData, condition),
            },
            FieldInfoData {
                name: "In",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(EventGateConditionValueNodeData, r#in),
            },
            FieldInfoData {
                name: "InValue",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(EventGateConditionValueNodeData, in_value),
            },
            FieldInfoData {
                name: "CoolDownTime",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(EventGateConditionValueNodeData, cool_down_time),
            },
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(EventGateConditionValueNodeData, enable),
            },
            FieldInfoData {
                name: "Out",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(EventGateConditionValueNodeData, out),
            },
            FieldInfoData {
                name: "OutValue",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(EventGateConditionValueNodeData, out_value),
            },
        ],
    }),
    array_type: Some(EVENTGATECONDITIONVALUENODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EventGateConditionValueNodeData {
    fn type_info() -> &'static TypeInfo {
        EVENTGATECONDITIONVALUENODEDATA_TYPE_INFO
    }
}


pub const EVENTGATECONDITIONVALUENODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventGateConditionValueNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("EventGateConditionValueNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
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

pub const EVENTGATECONDITIONVALUETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventGateConditionValueType",
    flags: MemberInfoFlags::new(49429),
    module: "DiceCommonsShared",
    data: TypeInfoData::Enum,
    array_type: Some(EVENTGATECONDITIONVALUETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EventGateConditionValueType {
    fn type_info() -> &'static TypeInfo {
        EVENTGATECONDITIONVALUETYPE_TYPE_INFO
    }
}


pub const EVENTGATECONDITIONVALUETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventGateConditionValueType-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("EventGateConditionValueType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DicePhysicsNodeData {
    pub angular_velocity_x: super::audio::AudioGraphNodePort,
    pub angular_velocity_y: super::audio::AudioGraphNodePort,
    pub angular_velocity_z: super::audio::AudioGraphNodePort,
    pub signed_speed: bool,
}

pub const DICEPHYSICSNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DicePhysicsNodeData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "AngularVelocity_X",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(DicePhysicsNodeData, angular_velocity_x),
            },
            FieldInfoData {
                name: "AngularVelocity_Y",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(DicePhysicsNodeData, angular_velocity_y),
            },
            FieldInfoData {
                name: "AngularVelocity_Z",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(DicePhysicsNodeData, angular_velocity_z),
            },
            FieldInfoData {
                name: "SignedSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DicePhysicsNodeData, signed_speed),
            },
        ],
    }),
    array_type: Some(DICEPHYSICSNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DicePhysicsNodeData {
    fn type_info() -> &'static TypeInfo {
        DICEPHYSICSNODEDATA_TYPE_INFO
    }
}


pub const DICEPHYSICSNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DicePhysicsNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DicePhysicsNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DiceDivisibleLoopPlayerNodeData {
    pub start: super::audio::AudioGraphNodePort,
    pub stop: super::audio::AudioGraphNodePort,
    pub amplitude: super::audio::AudioGraphNodePort,
    pub freeze_segment: super::audio::AudioGraphNodePort,
    pub output: super::audio::AudioGraphNodePort,
    pub wave: super::audio::SoundWaveAsset,
    pub external_wave: super::audio::AudioGraphNodePort,
    pub plugins: Vec<DiceDivisibleLoopPlayerPlugins>,
    pub cross_fade_length: f32,
    pub start_at_random_position: bool,
}

pub const DICEDIVISIBLELOOPPLAYERNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceDivisibleLoopPlayerNodeData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Start",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(DiceDivisibleLoopPlayerNodeData, start),
            },
            FieldInfoData {
                name: "Stop",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(DiceDivisibleLoopPlayerNodeData, stop),
            },
            FieldInfoData {
                name: "Amplitude",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(DiceDivisibleLoopPlayerNodeData, amplitude),
            },
            FieldInfoData {
                name: "FreezeSegment",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(DiceDivisibleLoopPlayerNodeData, freeze_segment),
            },
            FieldInfoData {
                name: "Output",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(DiceDivisibleLoopPlayerNodeData, output),
            },
            FieldInfoData {
                name: "Wave",
                flags: MemberInfoFlags::new(0),
                field_type: SOUNDWAVEASSET_TYPE_INFO,
                rust_offset: offset_of!(DiceDivisibleLoopPlayerNodeData, wave),
            },
            FieldInfoData {
                name: "ExternalWave",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(DiceDivisibleLoopPlayerNodeData, external_wave),
            },
            FieldInfoData {
                name: "Plugins",
                flags: MemberInfoFlags::new(144),
                field_type: DICEDIVISIBLELOOPPLAYERPLUGINS_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(DiceDivisibleLoopPlayerNodeData, plugins),
            },
            FieldInfoData {
                name: "CrossFadeLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DiceDivisibleLoopPlayerNodeData, cross_fade_length),
            },
            FieldInfoData {
                name: "StartAtRandomPosition",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DiceDivisibleLoopPlayerNodeData, start_at_random_position),
            },
        ],
    }),
    array_type: Some(DICEDIVISIBLELOOPPLAYERNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DiceDivisibleLoopPlayerNodeData {
    fn type_info() -> &'static TypeInfo {
        DICEDIVISIBLELOOPPLAYERNODEDATA_TYPE_INFO
    }
}


pub const DICEDIVISIBLELOOPPLAYERNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceDivisibleLoopPlayerNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceDivisibleLoopPlayerNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DiceDivisibleLoopPlayerPlugins {
    pub snd_player: super::audio::SoundGraphPluginRef,
    pub pause: super::audio::SoundGraphPluginRef,
    pub gain: super::audio::SoundGraphPluginRef,
    pub gain_fader: super::audio::SoundGraphPluginRef,
}

pub const DICEDIVISIBLELOOPPLAYERPLUGINS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceDivisibleLoopPlayerPlugins",
    flags: MemberInfoFlags::new(36937),
    module: "DiceCommonsShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "SndPlayer",
                flags: MemberInfoFlags::new(0),
                field_type: SOUNDGRAPHPLUGINREF_TYPE_INFO,
                rust_offset: offset_of!(DiceDivisibleLoopPlayerPlugins, snd_player),
            },
            FieldInfoData {
                name: "Pause",
                flags: MemberInfoFlags::new(0),
                field_type: SOUNDGRAPHPLUGINREF_TYPE_INFO,
                rust_offset: offset_of!(DiceDivisibleLoopPlayerPlugins, pause),
            },
            FieldInfoData {
                name: "Gain",
                flags: MemberInfoFlags::new(0),
                field_type: SOUNDGRAPHPLUGINREF_TYPE_INFO,
                rust_offset: offset_of!(DiceDivisibleLoopPlayerPlugins, gain),
            },
            FieldInfoData {
                name: "GainFader",
                flags: MemberInfoFlags::new(0),
                field_type: SOUNDGRAPHPLUGINREF_TYPE_INFO,
                rust_offset: offset_of!(DiceDivisibleLoopPlayerPlugins, gain_fader),
            },
        ],
    }),
    array_type: Some(DICEDIVISIBLELOOPPLAYERPLUGINS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DiceDivisibleLoopPlayerPlugins {
    fn type_info() -> &'static TypeInfo {
        DICEDIVISIBLELOOPPLAYERPLUGINS_TYPE_INFO
    }
}


pub const DICEDIVISIBLELOOPPLAYERPLUGINS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceDivisibleLoopPlayerPlugins-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceDivisibleLoopPlayerPlugins-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DiceAdsrNodeData {
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

pub const DICEADSRNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceAdsrNodeData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Trigger",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(DiceAdsrNodeData, trigger),
            },
            FieldInfoData {
                name: "Release",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(DiceAdsrNodeData, release),
            },
            FieldInfoData {
                name: "A",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(DiceAdsrNodeData, a),
            },
            FieldInfoData {
                name: "D",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(DiceAdsrNodeData, d),
            },
            FieldInfoData {
                name: "S",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(DiceAdsrNodeData, s),
            },
            FieldInfoData {
                name: "R",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(DiceAdsrNodeData, r),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(DiceAdsrNodeData, value),
            },
            FieldInfoData {
                name: "TriggerOutput",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(DiceAdsrNodeData, trigger_output),
            },
            FieldInfoData {
                name: "Finished",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(DiceAdsrNodeData, finished),
            },
            FieldInfoData {
                name: "AlwaysStartFromZero",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DiceAdsrNodeData, always_start_from_zero),
            },
            FieldInfoData {
                name: "Scale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DiceAdsrNodeData, scale),
            },
        ],
    }),
    array_type: Some(DICEADSRNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DiceAdsrNodeData {
    fn type_info() -> &'static TypeInfo {
        DICEADSRNODEDATA_TYPE_INFO
    }
}


pub const DICEADSRNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceAdsrNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceAdsrNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CounterNodeData {
    pub increment: super::audio::AudioGraphNodePort,
    pub decrement: super::audio::AudioGraphNodePort,
    pub reset: super::audio::AudioGraphNodePort,
    pub has_changed: super::audio::AudioGraphNodePort,
    pub output_value: super::audio::AudioGraphNodePort,
    pub start_value: f32,
    pub count_step: f32,
}

pub const COUNTERNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CounterNodeData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Increment",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(CounterNodeData, increment),
            },
            FieldInfoData {
                name: "Decrement",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(CounterNodeData, decrement),
            },
            FieldInfoData {
                name: "Reset",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(CounterNodeData, reset),
            },
            FieldInfoData {
                name: "HasChanged",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(CounterNodeData, has_changed),
            },
            FieldInfoData {
                name: "OutputValue",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(CounterNodeData, output_value),
            },
            FieldInfoData {
                name: "StartValue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CounterNodeData, start_value),
            },
            FieldInfoData {
                name: "CountStep",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CounterNodeData, count_step),
            },
        ],
    }),
    array_type: Some(COUNTERNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CounterNodeData {
    fn type_info() -> &'static TypeInfo {
        COUNTERNODEDATA_TYPE_INFO
    }
}


pub const COUNTERNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CounterNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("CounterNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ConfigurableRangeMapperNodeData {
    pub input_value: super::audio::AudioGraphNodePort,
    pub output_value: super::audio::AudioGraphNodePort,
    pub default_output_value: super::audio::AudioGraphNodePort,
    pub default_output_value_enabled: bool,
    pub ranges: Vec<ConfigurableRangeMapperEntry>,
}

pub const CONFIGURABLERANGEMAPPERNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConfigurableRangeMapperNodeData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "InputValue",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ConfigurableRangeMapperNodeData, input_value),
            },
            FieldInfoData {
                name: "OutputValue",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ConfigurableRangeMapperNodeData, output_value),
            },
            FieldInfoData {
                name: "DefaultOutputValue",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ConfigurableRangeMapperNodeData, default_output_value),
            },
            FieldInfoData {
                name: "DefaultOutputValueEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ConfigurableRangeMapperNodeData, default_output_value_enabled),
            },
            FieldInfoData {
                name: "Ranges",
                flags: MemberInfoFlags::new(144),
                field_type: CONFIGURABLERANGEMAPPERENTRY_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ConfigurableRangeMapperNodeData, ranges),
            },
        ],
    }),
    array_type: Some(CONFIGURABLERANGEMAPPERNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ConfigurableRangeMapperNodeData {
    fn type_info() -> &'static TypeInfo {
        CONFIGURABLERANGEMAPPERNODEDATA_TYPE_INFO
    }
}


pub const CONFIGURABLERANGEMAPPERNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConfigurableRangeMapperNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ConfigurableRangeMapperNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ConfigurableRangeMapperEntry {
    pub range_start: super::audio::AudioGraphNodePort,
    pub range_end: super::audio::AudioGraphNodePort,
    pub output_value: super::audio::AudioGraphNodePort,
}

pub const CONFIGURABLERANGEMAPPERENTRY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConfigurableRangeMapperEntry",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEPORTGROUP_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "RangeStart",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ConfigurableRangeMapperEntry, range_start),
            },
            FieldInfoData {
                name: "RangeEnd",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ConfigurableRangeMapperEntry, range_end),
            },
            FieldInfoData {
                name: "OutputValue",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ConfigurableRangeMapperEntry, output_value),
            },
        ],
    }),
    array_type: Some(CONFIGURABLERANGEMAPPERENTRY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ConfigurableRangeMapperEntry {
    fn type_info() -> &'static TypeInfo {
        CONFIGURABLERANGEMAPPERENTRY_TYPE_INFO
    }
}


pub const CONFIGURABLERANGEMAPPERENTRY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConfigurableRangeMapperEntry-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ConfigurableRangeMapperEntry-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ConditionValueNodeData {
    pub conditions: Vec<ConditionValueGroup>,
}

pub const CONDITIONVALUENODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionValueNodeData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Conditions",
                flags: MemberInfoFlags::new(144),
                field_type: CONDITIONVALUEGROUP_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ConditionValueNodeData, conditions),
            },
        ],
    }),
    array_type: Some(CONDITIONVALUENODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ConditionValueNodeData {
    fn type_info() -> &'static TypeInfo {
        CONDITIONVALUENODEDATA_TYPE_INFO
    }
}


pub const CONDITIONVALUENODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionValueNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ConditionValueNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ConditionValueGroup {
    pub x: super::audio::AudioGraphNodePort,
    pub y: super::audio::AudioGraphNodePort,
    pub out: super::audio::AudioGraphNodePort,
    pub condition: ConditionValueType,
    pub value_if_true: f32,
    pub value_if_false: f32,
}

pub const CONDITIONVALUEGROUP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionValueGroup",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEPORTGROUP_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "X",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ConditionValueGroup, x),
            },
            FieldInfoData {
                name: "Y",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ConditionValueGroup, y),
            },
            FieldInfoData {
                name: "Out",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ConditionValueGroup, out),
            },
            FieldInfoData {
                name: "Condition",
                flags: MemberInfoFlags::new(0),
                field_type: CONDITIONVALUETYPE_TYPE_INFO,
                rust_offset: offset_of!(ConditionValueGroup, condition),
            },
            FieldInfoData {
                name: "ValueIfTrue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ConditionValueGroup, value_if_true),
            },
            FieldInfoData {
                name: "ValueIfFalse",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ConditionValueGroup, value_if_false),
            },
        ],
    }),
    array_type: Some(CONDITIONVALUEGROUP_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ConditionValueGroup {
    fn type_info() -> &'static TypeInfo {
        CONDITIONVALUEGROUP_TYPE_INFO
    }
}


pub const CONDITIONVALUEGROUP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionValueGroup-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ConditionValueGroup-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
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

pub const CONDITIONVALUETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionValueType",
    flags: MemberInfoFlags::new(49429),
    module: "DiceCommonsShared",
    data: TypeInfoData::Enum,
    array_type: Some(CONDITIONVALUETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ConditionValueType {
    fn type_info() -> &'static TypeInfo {
        CONDITIONVALUETYPE_TYPE_INFO
    }
}


pub const CONDITIONVALUETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionValueType-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ConditionValueType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CompareValueNodeData {
    pub conditions: Vec<CompareValueGroup>,
}

pub const COMPAREVALUENODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareValueNodeData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Conditions",
                flags: MemberInfoFlags::new(144),
                field_type: COMPAREVALUEGROUP_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(CompareValueNodeData, conditions),
            },
        ],
    }),
    array_type: Some(COMPAREVALUENODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CompareValueNodeData {
    fn type_info() -> &'static TypeInfo {
        COMPAREVALUENODEDATA_TYPE_INFO
    }
}


pub const COMPAREVALUENODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareValueNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("CompareValueNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CompareValueGroup {
    pub evaluate: super::audio::AudioGraphNodePort,
    pub x: super::audio::AudioGraphNodePort,
    pub y: super::audio::AudioGraphNodePort,
    pub r#true: super::audio::AudioGraphNodePort,
    pub r#false: super::audio::AudioGraphNodePort,
    pub condition: CompareValueConditionType,
    pub auto_evaluate: bool,
}

pub const COMPAREVALUEGROUP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareValueGroup",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEPORTGROUP_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Evaluate",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(CompareValueGroup, evaluate),
            },
            FieldInfoData {
                name: "X",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(CompareValueGroup, x),
            },
            FieldInfoData {
                name: "Y",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(CompareValueGroup, y),
            },
            FieldInfoData {
                name: "True",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(CompareValueGroup, r#true),
            },
            FieldInfoData {
                name: "False",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(CompareValueGroup, r#false),
            },
            FieldInfoData {
                name: "Condition",
                flags: MemberInfoFlags::new(0),
                field_type: COMPAREVALUECONDITIONTYPE_TYPE_INFO,
                rust_offset: offset_of!(CompareValueGroup, condition),
            },
            FieldInfoData {
                name: "AutoEvaluate",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CompareValueGroup, auto_evaluate),
            },
        ],
    }),
    array_type: Some(COMPAREVALUEGROUP_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CompareValueGroup {
    fn type_info() -> &'static TypeInfo {
        COMPAREVALUEGROUP_TYPE_INFO
    }
}


pub const COMPAREVALUEGROUP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareValueGroup-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("CompareValueGroup-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
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

pub const COMPAREVALUECONDITIONTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareValueConditionType",
    flags: MemberInfoFlags::new(49429),
    module: "DiceCommonsShared",
    data: TypeInfoData::Enum,
    array_type: Some(COMPAREVALUECONDITIONTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CompareValueConditionType {
    fn type_info() -> &'static TypeInfo {
        COMPAREVALUECONDITIONTYPE_TYPE_INFO
    }
}


pub const COMPAREVALUECONDITIONTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareValueConditionType-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("CompareValueConditionType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ClampNodeData {
    pub r#in: super::audio::AudioGraphNodePort,
    pub out: super::audio::AudioGraphNodePort,
    pub clamp_min: f32,
    pub clamp_max: f32,
}

pub const CLAMPNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClampNodeData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "In",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ClampNodeData, r#in),
            },
            FieldInfoData {
                name: "Out",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ClampNodeData, out),
            },
            FieldInfoData {
                name: "ClampMin",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ClampNodeData, clamp_min),
            },
            FieldInfoData {
                name: "ClampMax",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ClampNodeData, clamp_max),
            },
        ],
    }),
    array_type: Some(CLAMPNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ClampNodeData {
    fn type_info() -> &'static TypeInfo {
        CLAMPNODEDATA_TYPE_INFO
    }
}


pub const CLAMPNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClampNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ClampNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AudioEnvelopeSwitcherNodeData {
    pub index: super::audio::AudioGraphNodePort,
    pub advance: super::audio::AudioGraphNodePort,
    pub audio_envelope: super::audio::AudioGraphNodePort,
    pub index_changed: super::audio::AudioGraphNodePort,
    pub audio_envelopes: Vec<AudioEnvelopeAsset>,
    pub default_index: f32,
    pub is_random: bool,
    pub random_start_index: bool,
}

pub const AUDIOENVELOPESWITCHERNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioEnvelopeSwitcherNodeData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Index",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(AudioEnvelopeSwitcherNodeData, index),
            },
            FieldInfoData {
                name: "Advance",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(AudioEnvelopeSwitcherNodeData, advance),
            },
            FieldInfoData {
                name: "AudioEnvelope",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(AudioEnvelopeSwitcherNodeData, audio_envelope),
            },
            FieldInfoData {
                name: "IndexChanged",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(AudioEnvelopeSwitcherNodeData, index_changed),
            },
            FieldInfoData {
                name: "AudioEnvelopes",
                flags: MemberInfoFlags::new(144),
                field_type: AUDIOENVELOPEASSET_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(AudioEnvelopeSwitcherNodeData, audio_envelopes),
            },
            FieldInfoData {
                name: "DefaultIndex",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AudioEnvelopeSwitcherNodeData, default_index),
            },
            FieldInfoData {
                name: "IsRandom",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AudioEnvelopeSwitcherNodeData, is_random),
            },
            FieldInfoData {
                name: "RandomStartIndex",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AudioEnvelopeSwitcherNodeData, random_start_index),
            },
        ],
    }),
    array_type: Some(AUDIOENVELOPESWITCHERNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AudioEnvelopeSwitcherNodeData {
    fn type_info() -> &'static TypeInfo {
        AUDIOENVELOPESWITCHERNODEDATA_TYPE_INFO
    }
}


pub const AUDIOENVELOPESWITCHERNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioEnvelopeSwitcherNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("AudioEnvelopeSwitcherNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AudioEnvelopeSwitcherNodeConfigData {
    pub audio_envelopes: Vec<AudioEnvelopeAsset>,
}

pub const AUDIOENVELOPESWITCHERNODECONFIGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioEnvelopeSwitcherNodeConfigData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODECONFIGDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "AudioEnvelopes",
                flags: MemberInfoFlags::new(144),
                field_type: AUDIOENVELOPEASSET_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(AudioEnvelopeSwitcherNodeConfigData, audio_envelopes),
            },
        ],
    }),
    array_type: Some(AUDIOENVELOPESWITCHERNODECONFIGDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AudioEnvelopeSwitcherNodeConfigData {
    fn type_info() -> &'static TypeInfo {
        AUDIOENVELOPESWITCHERNODECONFIGDATA_TYPE_INFO
    }
}


pub const AUDIOENVELOPESWITCHERNODECONFIGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioEnvelopeSwitcherNodeConfigData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("AudioEnvelopeSwitcherNodeConfigData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AudioEnvelopeNodeData {
    pub x: super::audio::AudioGraphNodePort,
    pub envelope_in: super::audio::AudioGraphNodePort,
    pub y: super::audio::AudioGraphNodePort,
    pub region: super::audio::AudioGraphNodePort,
    pub envelope: AudioEnvelope,
    pub envelope_asset: AudioEnvelopeAsset,
}

pub const AUDIOENVELOPENODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioEnvelopeNodeData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "X",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(AudioEnvelopeNodeData, x),
            },
            FieldInfoData {
                name: "EnvelopeIn",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(AudioEnvelopeNodeData, envelope_in),
            },
            FieldInfoData {
                name: "Y",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(AudioEnvelopeNodeData, y),
            },
            FieldInfoData {
                name: "Region",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(AudioEnvelopeNodeData, region),
            },
            FieldInfoData {
                name: "Envelope",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOENVELOPE_TYPE_INFO,
                rust_offset: offset_of!(AudioEnvelopeNodeData, envelope),
            },
            FieldInfoData {
                name: "EnvelopeAsset",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOENVELOPEASSET_TYPE_INFO,
                rust_offset: offset_of!(AudioEnvelopeNodeData, envelope_asset),
            },
        ],
    }),
    array_type: Some(AUDIOENVELOPENODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AudioEnvelopeNodeData {
    fn type_info() -> &'static TypeInfo {
        AUDIOENVELOPENODEDATA_TYPE_INFO
    }
}


pub const AUDIOENVELOPENODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioEnvelopeNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("AudioEnvelopeNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AudioEnvelopeAsset {
    pub envelope: AudioEnvelope,
}

pub const AUDIOENVELOPEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioEnvelopeAsset",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Envelope",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOENVELOPE_TYPE_INFO,
                rust_offset: offset_of!(AudioEnvelopeAsset, envelope),
            },
        ],
    }),
    array_type: Some(AUDIOENVELOPEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AudioEnvelopeAsset {
    fn type_info() -> &'static TypeInfo {
        AUDIOENVELOPEASSET_TYPE_INFO
    }
}


pub const AUDIOENVELOPEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioEnvelopeAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("AudioEnvelopeAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AudioEnvelope {
    pub points: Vec<AudioEnvelopePoint>,
    pub default_curve_type: AudioEnvelopeLineType,
}

pub const AUDIOENVELOPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioEnvelope",
    flags: MemberInfoFlags::new(73),
    module: "DiceCommonsShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Points",
                flags: MemberInfoFlags::new(144),
                field_type: AUDIOENVELOPEPOINT_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(AudioEnvelope, points),
            },
            FieldInfoData {
                name: "DefaultCurveType",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOENVELOPELINETYPE_TYPE_INFO,
                rust_offset: offset_of!(AudioEnvelope, default_curve_type),
            },
        ],
    }),
    array_type: Some(AUDIOENVELOPE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AudioEnvelope {
    fn type_info() -> &'static TypeInfo {
        AUDIOENVELOPE_TYPE_INFO
    }
}


pub const AUDIOENVELOPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioEnvelope-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("AudioEnvelope-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AudioEnvelopePoint {
    pub point: super::core::Vec2,
    pub line_type: AudioEnvelopeLineType,
    pub curve_scale: f32,
    pub is_region_boundary: bool,
}

pub const AUDIOENVELOPEPOINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioEnvelopePoint",
    flags: MemberInfoFlags::new(36937),
    module: "DiceCommonsShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Point",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(AudioEnvelopePoint, point),
            },
            FieldInfoData {
                name: "LineType",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOENVELOPELINETYPE_TYPE_INFO,
                rust_offset: offset_of!(AudioEnvelopePoint, line_type),
            },
            FieldInfoData {
                name: "CurveScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AudioEnvelopePoint, curve_scale),
            },
            FieldInfoData {
                name: "IsRegionBoundary",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AudioEnvelopePoint, is_region_boundary),
            },
        ],
    }),
    array_type: Some(AUDIOENVELOPEPOINT_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for AudioEnvelopePoint {
    fn type_info() -> &'static TypeInfo {
        AUDIOENVELOPEPOINT_TYPE_INFO
    }
}


pub const AUDIOENVELOPEPOINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioEnvelopePoint-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("AudioEnvelopePoint-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
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

pub const SNAPTOGRIDGRANULARITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnapToGridGranularity",
    flags: MemberInfoFlags::new(49429),
    module: "DiceCommonsShared",
    data: TypeInfoData::Enum,
    array_type: Some(SNAPTOGRIDGRANULARITY_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SnapToGridGranularity {
    fn type_info() -> &'static TypeInfo {
        SNAPTOGRIDGRANULARITY_TYPE_INFO
    }
}


pub const SNAPTOGRIDGRANULARITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnapToGridGranularity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("SnapToGridGranularity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum AudioEnvelopeLineType {
    #[default]
    AudioEnvelopeLineType_Linear = 0,
    AudioEnvelopeLineType_Reciprocal = 1,
    AudioEnvelopeLineType_InverseReciprocal = 2,
    AudioEnvelopeLineType_SCurve = 3,
    AudioEnvelopeLineType_Sine = 4,
    AudioEnvelopeLineType_Exponential = 5,
}

pub const AUDIOENVELOPELINETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioEnvelopeLineType",
    flags: MemberInfoFlags::new(49429),
    module: "DiceCommonsShared",
    data: TypeInfoData::Enum,
    array_type: Some(AUDIOENVELOPELINETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AudioEnvelopeLineType {
    fn type_info() -> &'static TypeInfo {
        AUDIOENVELOPELINETYPE_TYPE_INFO
    }
}


pub const AUDIOENVELOPELINETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioEnvelopeLineType-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("AudioEnvelopeLineType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AssetCrossfaderNodeData {
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

pub const ASSETCROSSFADERNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AssetCrossfaderNodeData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "AssetIn",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(AssetCrossfaderNodeData, asset_in),
            },
            FieldInfoData {
                name: "CrossfadeTime",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(AssetCrossfaderNodeData, crossfade_time),
            },
            FieldInfoData {
                name: "ForceCrossfade",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(AssetCrossfaderNodeData, force_crossfade),
            },
            FieldInfoData {
                name: "AssetA",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(AssetCrossfaderNodeData, asset_a),
            },
            FieldInfoData {
                name: "AmplitudeA",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(AssetCrossfaderNodeData, amplitude_a),
            },
            FieldInfoData {
                name: "TriggerA",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(AssetCrossfaderNodeData, trigger_a),
            },
            FieldInfoData {
                name: "ReleaseA",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(AssetCrossfaderNodeData, release_a),
            },
            FieldInfoData {
                name: "AssetB",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(AssetCrossfaderNodeData, asset_b),
            },
            FieldInfoData {
                name: "AmplitudeB",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(AssetCrossfaderNodeData, amplitude_b),
            },
            FieldInfoData {
                name: "TriggerB",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(AssetCrossfaderNodeData, trigger_b),
            },
            FieldInfoData {
                name: "ReleaseB",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(AssetCrossfaderNodeData, release_b),
            },
            FieldInfoData {
                name: "AutoCrossfadeOnAssetChange",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AssetCrossfaderNodeData, auto_crossfade_on_asset_change),
            },
        ],
    }),
    array_type: Some(ASSETCROSSFADERNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AssetCrossfaderNodeData {
    fn type_info() -> &'static TypeInfo {
        ASSETCROSSFADERNODEDATA_TYPE_INFO
    }
}


pub const ASSETCROSSFADERNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AssetCrossfaderNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("AssetCrossfaderNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ArOneShotNodeData {
    pub start: super::audio::AudioGraphNodePort,
    pub attack: super::audio::AudioGraphNodePort,
    pub hold: super::audio::AudioGraphNodePort,
    pub release: super::audio::AudioGraphNodePort,
    pub power: f32,
    pub value: super::audio::AudioGraphNodePort,
    pub started: super::audio::AudioGraphNodePort,
    pub stopped: super::audio::AudioGraphNodePort,
}

pub const ARONESHOTNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ArOneShotNodeData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Start",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ArOneShotNodeData, start),
            },
            FieldInfoData {
                name: "Attack",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ArOneShotNodeData, attack),
            },
            FieldInfoData {
                name: "Hold",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ArOneShotNodeData, hold),
            },
            FieldInfoData {
                name: "Release",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ArOneShotNodeData, release),
            },
            FieldInfoData {
                name: "Power",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ArOneShotNodeData, power),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ArOneShotNodeData, value),
            },
            FieldInfoData {
                name: "Started",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ArOneShotNodeData, started),
            },
            FieldInfoData {
                name: "Stopped",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ArOneShotNodeData, stopped),
            },
        ],
    }),
    array_type: Some(ARONESHOTNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ArOneShotNodeData {
    fn type_info() -> &'static TypeInfo {
        ARONESHOTNODEDATA_TYPE_INFO
    }
}


pub const ARONESHOTNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ArOneShotNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ArOneShotNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ArLoopingNodeData {
    pub start: super::audio::AudioGraphNodePort,
    pub stop: super::audio::AudioGraphNodePort,
    pub attack: super::audio::AudioGraphNodePort,
    pub release: super::audio::AudioGraphNodePort,
    pub power: f32,
    pub value: super::audio::AudioGraphNodePort,
    pub started: super::audio::AudioGraphNodePort,
    pub stopped: super::audio::AudioGraphNodePort,
}

pub const ARLOOPINGNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ArLoopingNodeData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Start",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ArLoopingNodeData, start),
            },
            FieldInfoData {
                name: "Stop",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ArLoopingNodeData, stop),
            },
            FieldInfoData {
                name: "Attack",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ArLoopingNodeData, attack),
            },
            FieldInfoData {
                name: "Release",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ArLoopingNodeData, release),
            },
            FieldInfoData {
                name: "Power",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ArLoopingNodeData, power),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ArLoopingNodeData, value),
            },
            FieldInfoData {
                name: "Started",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ArLoopingNodeData, started),
            },
            FieldInfoData {
                name: "Stopped",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ArLoopingNodeData, stopped),
            },
        ],
    }),
    array_type: Some(ARLOOPINGNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ArLoopingNodeData {
    fn type_info() -> &'static TypeInfo {
        ARLOOPINGNODEDATA_TYPE_INFO
    }
}


pub const ARLOOPINGNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ArLoopingNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ArLoopingNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ArFlipFlopNodeData {
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

pub const ARFLIPFLOPNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ArFlipFlopNodeData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Start",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ArFlipFlopNodeData, start),
            },
            FieldInfoData {
                name: "Stop",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ArFlipFlopNodeData, stop),
            },
            FieldInfoData {
                name: "StopEndCycle",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ArFlipFlopNodeData, stop_end_cycle),
            },
            FieldInfoData {
                name: "Attack",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ArFlipFlopNodeData, attack),
            },
            FieldInfoData {
                name: "Hold",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ArFlipFlopNodeData, hold),
            },
            FieldInfoData {
                name: "Release",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ArFlipFlopNodeData, release),
            },
            FieldInfoData {
                name: "Power",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ArFlipFlopNodeData, power),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ArFlipFlopNodeData, value),
            },
            FieldInfoData {
                name: "Started",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ArFlipFlopNodeData, started),
            },
            FieldInfoData {
                name: "EndAttack",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ArFlipFlopNodeData, end_attack),
            },
            FieldInfoData {
                name: "EndHold",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ArFlipFlopNodeData, end_hold),
            },
            FieldInfoData {
                name: "EndRelease",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ArFlipFlopNodeData, end_release),
            },
            FieldInfoData {
                name: "Stopped",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ArFlipFlopNodeData, stopped),
            },
        ],
    }),
    array_type: Some(ARFLIPFLOPNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ArFlipFlopNodeData {
    fn type_info() -> &'static TypeInfo {
        ARFLIPFLOPNODEDATA_TYPE_INFO
    }
}


pub const ARFLIPFLOPNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ArFlipFlopNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ArFlipFlopNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ListenerAreaTypeNodeData {
    pub area_type: super::audio::AudioGraphNodePort,
    pub use_default_listener: bool,
    pub listener_id: i32,
}

pub const LISTENERAREATYPENODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ListenerAreaTypeNodeData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "AreaType",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(ListenerAreaTypeNodeData, area_type),
            },
            FieldInfoData {
                name: "UseDefaultListener",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ListenerAreaTypeNodeData, use_default_listener),
            },
            FieldInfoData {
                name: "ListenerId",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ListenerAreaTypeNodeData, listener_id),
            },
        ],
    }),
    array_type: Some(LISTENERAREATYPENODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ListenerAreaTypeNodeData {
    fn type_info() -> &'static TypeInfo {
        LISTENERAREATYPENODEDATA_TYPE_INFO
    }
}


pub const LISTENERAREATYPENODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ListenerAreaTypeNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ListenerAreaTypeNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AreaTypeNodeData {
    pub check: super::audio::AudioGraphNodePort,
    pub result: super::audio::AudioGraphNodePort,
    pub area_type: super::audio::AudioGraphNodePort,
}

pub const AREATYPENODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AreaTypeNodeData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Check",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(AreaTypeNodeData, check),
            },
            FieldInfoData {
                name: "Result",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(AreaTypeNodeData, result),
            },
            FieldInfoData {
                name: "AreaType",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(AreaTypeNodeData, area_type),
            },
        ],
    }),
    array_type: Some(AREATYPENODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AreaTypeNodeData {
    fn type_info() -> &'static TypeInfo {
        AREATYPENODEDATA_TYPE_INFO
    }
}


pub const AREATYPENODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AreaTypeNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("AreaTypeNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct VoiceOverSourceEntityData {
    pub voice_over_info: super::audio::EntityVoiceOverInfo,
    pub transform: super::core::LinearTransform,
}

pub const VOICEOVERSOURCEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceOverSourceEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "VoiceOverInfo",
                flags: MemberInfoFlags::new(0),
                field_type: ENTITYVOICEOVERINFO_TYPE_INFO,
                rust_offset: offset_of!(VoiceOverSourceEntityData, voice_over_info),
            },
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(VoiceOverSourceEntityData, transform),
            },
        ],
    }),
    array_type: Some(VOICEOVERSOURCEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for VoiceOverSourceEntityData {
    fn type_info() -> &'static TypeInfo {
        VOICEOVERSOURCEENTITYDATA_TYPE_INFO
    }
}


pub const VOICEOVERSOURCEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceOverSourceEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("VoiceOverSourceEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VoiceOverSetLanguageEntityData {
    pub language_index: i32,
}

pub const VOICEOVERSETLANGUAGEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceOverSetLanguageEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "LanguageIndex",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(VoiceOverSetLanguageEntityData, language_index),
            },
        ],
    }),
    array_type: Some(VOICEOVERSETLANGUAGEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VoiceOverSetLanguageEntityData {
    fn type_info() -> &'static TypeInfo {
        VOICEOVERSETLANGUAGEENTITYDATA_TYPE_INFO
    }
}


pub const VOICEOVERSETLANGUAGEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceOverSetLanguageEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("VoiceOverSetLanguageEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LanguageCollection {
    pub languages_string: Vec<LanguageCollectionElement>,
}

pub const LANGUAGECOLLECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LanguageCollection",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "LanguagesString",
                flags: MemberInfoFlags::new(144),
                field_type: LANGUAGECOLLECTIONELEMENT_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LanguageCollection, languages_string),
            },
        ],
    }),
    array_type: Some(LANGUAGECOLLECTION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LanguageCollection {
    fn type_info() -> &'static TypeInfo {
        LANGUAGECOLLECTION_TYPE_INFO
    }
}


pub const LANGUAGECOLLECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LanguageCollection-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("LanguageCollection-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LanguageCollectionElement {
    pub language: String,
    pub language_string_id: super::u_i_incubator_shared::LocalizedStringId,
}

pub const LANGUAGECOLLECTIONELEMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LanguageCollectionElement",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Language",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(LanguageCollectionElement, language),
            },
            FieldInfoData {
                name: "LanguageStringId",
                flags: MemberInfoFlags::new(0),
                field_type: LOCALIZEDSTRINGID_TYPE_INFO,
                rust_offset: offset_of!(LanguageCollectionElement, language_string_id),
            },
        ],
    }),
    array_type: Some(LANGUAGECOLLECTIONELEMENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LanguageCollectionElement {
    fn type_info() -> &'static TypeInfo {
        LANGUAGECOLLECTIONELEMENT_TYPE_INFO
    }
}


pub const LANGUAGECOLLECTIONELEMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LanguageCollectionElement-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("LanguageCollectionElement-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct VoiceOverLocatorEntityData {
    pub realm: super::core::Realm,
    pub voice_over_info: super::audio::EntityVoiceOverInfo,
}

pub const VOICEOVERLOCATORENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceOverLocatorEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(VoiceOverLocatorEntityData, realm),
            },
            FieldInfoData {
                name: "VoiceOverInfo",
                flags: MemberInfoFlags::new(0),
                field_type: ENTITYVOICEOVERINFO_TYPE_INFO,
                rust_offset: offset_of!(VoiceOverLocatorEntityData, voice_over_info),
            },
        ],
    }),
    array_type: Some(VOICEOVERLOCATORENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for VoiceOverLocatorEntityData {
    fn type_info() -> &'static TypeInfo {
        VOICEOVERLOCATORENTITYDATA_TYPE_INFO
    }
}


pub const VOICEOVERLOCATORENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceOverLocatorEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("VoiceOverLocatorEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EntryControllableAttachData {
    pub bone: super::entity::GameplayBones,
}

pub const ENTRYCONTROLLABLEATTACHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntryControllableAttachData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYATTACHDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Bone",
                flags: MemberInfoFlags::new(0),
                field_type: GAMEPLAYBONES_TYPE_INFO,
                rust_offset: offset_of!(EntryControllableAttachData, bone),
            },
        ],
    }),
    array_type: Some(ENTRYCONTROLLABLEATTACHDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EntryControllableAttachData {
    fn type_info() -> &'static TypeInfo {
        ENTRYCONTROLLABLEATTACHDATA_TYPE_INFO
    }
}


pub const ENTRYCONTROLLABLEATTACHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntryControllableAttachData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("EntryControllableAttachData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ControllableAttachData {
}

pub const CONTROLLABLEATTACHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ControllableAttachData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ANIMATABLEATTACHDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CONTROLLABLEATTACHDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ControllableAttachData {
    fn type_info() -> &'static TypeInfo {
        CONTROLLABLEATTACHDATA_TYPE_INFO
    }
}


pub const CONTROLLABLEATTACHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ControllableAttachData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ControllableAttachData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ModelAnimationAttachData {
    pub bone: super::entity::GameplayBones,
}

pub const MODELANIMATIONATTACHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ModelAnimationAttachData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYATTACHDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Bone",
                flags: MemberInfoFlags::new(0),
                field_type: GAMEPLAYBONES_TYPE_INFO,
                rust_offset: offset_of!(ModelAnimationAttachData, bone),
            },
        ],
    }),
    array_type: Some(MODELANIMATIONATTACHDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ModelAnimationAttachData {
    fn type_info() -> &'static TypeInfo {
        MODELANIMATIONATTACHDATA_TYPE_INFO
    }
}


pub const MODELANIMATIONATTACHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ModelAnimationAttachData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ModelAnimationAttachData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AnimatableAttachData {
    pub bone: super::entity::GameplayBones,
}

pub const ANIMATABLEATTACHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimatableAttachData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYATTACHDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Bone",
                flags: MemberInfoFlags::new(0),
                field_type: GAMEPLAYBONES_TYPE_INFO,
                rust_offset: offset_of!(AnimatableAttachData, bone),
            },
        ],
    }),
    array_type: Some(ANIMATABLEATTACHDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AnimatableAttachData {
    fn type_info() -> &'static TypeInfo {
        ANIMATABLEATTACHDATA_TYPE_INFO
    }
}


pub const ANIMATABLEATTACHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimatableAttachData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("AnimatableAttachData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ComponentAttachData {
}

pub const COMPONENTATTACHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ComponentAttachData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYATTACHDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(COMPONENTATTACHDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ComponentAttachData {
    fn type_info() -> &'static TypeInfo {
        COMPONENTATTACHDATA_TYPE_INFO
    }
}


pub const COMPONENTATTACHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ComponentAttachData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ComponentAttachData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EntityAttachData {
    pub property_name: String,
    pub has_dynamic_transform_space_offset: bool,
    pub use_rotation: bool,
    pub coordinate_space: CoordinateModificationData,
    pub offset: OffsetModificationData,
}

pub const ENTITYATTACHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityAttachData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYLINKDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "PropertyName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(EntityAttachData, property_name),
            },
            FieldInfoData {
                name: "HasDynamicTransformSpaceOffset",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EntityAttachData, has_dynamic_transform_space_offset),
            },
            FieldInfoData {
                name: "UseRotation",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EntityAttachData, use_rotation),
            },
            FieldInfoData {
                name: "CoordinateSpace",
                flags: MemberInfoFlags::new(0),
                field_type: COORDINATEMODIFICATIONDATA_TYPE_INFO,
                rust_offset: offset_of!(EntityAttachData, coordinate_space),
            },
            FieldInfoData {
                name: "Offset",
                flags: MemberInfoFlags::new(0),
                field_type: OFFSETMODIFICATIONDATA_TYPE_INFO,
                rust_offset: offset_of!(EntityAttachData, offset),
            },
        ],
    }),
    array_type: Some(ENTITYATTACHDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EntityAttachData {
    fn type_info() -> &'static TypeInfo {
        ENTITYATTACHDATA_TYPE_INFO
    }
}


pub const ENTITYATTACHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityAttachData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("EntityAttachData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ControllableTransformLinkData {
}

pub const CONTROLLABLETRANSFORMLINKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ControllableTransformLinkData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ANIMATABLETRANSFORMLINKDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CONTROLLABLETRANSFORMLINKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ControllableTransformLinkData {
    fn type_info() -> &'static TypeInfo {
        CONTROLLABLETRANSFORMLINKDATA_TYPE_INFO
    }
}


pub const CONTROLLABLETRANSFORMLINKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ControllableTransformLinkData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ControllableTransformLinkData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EntryControllableTransformLinkData {
    pub bone: super::entity::GameplayBones,
}

pub const ENTRYCONTROLLABLETRANSFORMLINKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntryControllableTransformLinkData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYTRANSFORMLINKDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Bone",
                flags: MemberInfoFlags::new(0),
                field_type: GAMEPLAYBONES_TYPE_INFO,
                rust_offset: offset_of!(EntryControllableTransformLinkData, bone),
            },
        ],
    }),
    array_type: Some(ENTRYCONTROLLABLETRANSFORMLINKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EntryControllableTransformLinkData {
    fn type_info() -> &'static TypeInfo {
        ENTRYCONTROLLABLETRANSFORMLINKDATA_TYPE_INFO
    }
}


pub const ENTRYCONTROLLABLETRANSFORMLINKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntryControllableTransformLinkData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("EntryControllableTransformLinkData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ComponentTransformLinkData {
}

pub const COMPONENTTRANSFORMLINKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ComponentTransformLinkData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYTRANSFORMLINKDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(COMPONENTTRANSFORMLINKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ComponentTransformLinkData {
    fn type_info() -> &'static TypeInfo {
        COMPONENTTRANSFORMLINKDATA_TYPE_INFO
    }
}


pub const COMPONENTTRANSFORMLINKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ComponentTransformLinkData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ComponentTransformLinkData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ModelAnimationTransformLinkData {
    pub bone: super::entity::GameplayBones,
}

pub const MODELANIMATIONTRANSFORMLINKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ModelAnimationTransformLinkData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYTRANSFORMLINKDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Bone",
                flags: MemberInfoFlags::new(0),
                field_type: GAMEPLAYBONES_TYPE_INFO,
                rust_offset: offset_of!(ModelAnimationTransformLinkData, bone),
            },
        ],
    }),
    array_type: Some(MODELANIMATIONTRANSFORMLINKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ModelAnimationTransformLinkData {
    fn type_info() -> &'static TypeInfo {
        MODELANIMATIONTRANSFORMLINKDATA_TYPE_INFO
    }
}


pub const MODELANIMATIONTRANSFORMLINKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ModelAnimationTransformLinkData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ModelAnimationTransformLinkData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AnimatableTransformLinkData {
    pub bone: super::entity::GameplayBones,
}

pub const ANIMATABLETRANSFORMLINKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimatableTransformLinkData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYTRANSFORMLINKDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Bone",
                flags: MemberInfoFlags::new(0),
                field_type: GAMEPLAYBONES_TYPE_INFO,
                rust_offset: offset_of!(AnimatableTransformLinkData, bone),
            },
        ],
    }),
    array_type: Some(ANIMATABLETRANSFORMLINKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AnimatableTransformLinkData {
    fn type_info() -> &'static TypeInfo {
        ANIMATABLETRANSFORMLINKDATA_TYPE_INFO
    }
}


pub const ANIMATABLETRANSFORMLINKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimatableTransformLinkData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("AnimatableTransformLinkData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EntityCenterTransformLinkData {
}

pub const ENTITYCENTERTRANSFORMLINKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityCenterTransformLinkData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYTRANSFORMLINKDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ENTITYCENTERTRANSFORMLINKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EntityCenterTransformLinkData {
    fn type_info() -> &'static TypeInfo {
        ENTITYCENTERTRANSFORMLINKDATA_TYPE_INFO
    }
}


pub const ENTITYCENTERTRANSFORMLINKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityCenterTransformLinkData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("EntityCenterTransformLinkData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EntityTransformLinkData {
}

pub const ENTITYTRANSFORMLINKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityTransformLinkData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYLINKDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ENTITYTRANSFORMLINKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EntityTransformLinkData {
    fn type_info() -> &'static TypeInfo {
        ENTITYTRANSFORMLINKDATA_TYPE_INFO
    }
}


pub const ENTITYTRANSFORMLINKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityTransformLinkData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("EntityTransformLinkData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EntityLinkData {
    pub link_name: String,
}

pub const ENTITYLINKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityLinkData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "LinkName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(EntityLinkData, link_name),
            },
        ],
    }),
    array_type: Some(ENTITYLINKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EntityLinkData {
    fn type_info() -> &'static TypeInfo {
        ENTITYLINKDATA_TYPE_INFO
    }
}


pub const ENTITYLINKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityLinkData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("EntityLinkData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct OffsetModificationData {
    pub offset_x_axis_in_world_space: bool,
    pub offset_y_axis_in_world_space: bool,
    pub offset_z_axis_in_world_space: bool,
    pub offset: super::core::LinearTransform,
}

pub const OFFSETMODIFICATIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OffsetModificationData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "OffsetXAxisInWorldSpace",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OffsetModificationData, offset_x_axis_in_world_space),
            },
            FieldInfoData {
                name: "OffsetYAxisInWorldSpace",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OffsetModificationData, offset_y_axis_in_world_space),
            },
            FieldInfoData {
                name: "OffsetZAxisInWorldSpace",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OffsetModificationData, offset_z_axis_in_world_space),
            },
            FieldInfoData {
                name: "Offset",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(OffsetModificationData, offset),
            },
        ],
    }),
    array_type: Some(OFFSETMODIFICATIONDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for OffsetModificationData {
    fn type_info() -> &'static TypeInfo {
        OFFSETMODIFICATIONDATA_TYPE_INFO
    }
}


pub const OFFSETMODIFICATIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OffsetModificationData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("OffsetModificationData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CoordinateModificationData {
    pub left: super::entity::ModifierAxis,
    pub up: super::entity::ModifierAxis,
    pub forward: super::entity::ModifierAxis,
    pub invert_left: bool,
    pub invert_up: bool,
    pub invert_forward: bool,
}

pub const COORDINATEMODIFICATIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoordinateModificationData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Left",
                flags: MemberInfoFlags::new(0),
                field_type: MODIFIERAXIS_TYPE_INFO,
                rust_offset: offset_of!(CoordinateModificationData, left),
            },
            FieldInfoData {
                name: "Up",
                flags: MemberInfoFlags::new(0),
                field_type: MODIFIERAXIS_TYPE_INFO,
                rust_offset: offset_of!(CoordinateModificationData, up),
            },
            FieldInfoData {
                name: "Forward",
                flags: MemberInfoFlags::new(0),
                field_type: MODIFIERAXIS_TYPE_INFO,
                rust_offset: offset_of!(CoordinateModificationData, forward),
            },
            FieldInfoData {
                name: "InvertLeft",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CoordinateModificationData, invert_left),
            },
            FieldInfoData {
                name: "InvertUp",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CoordinateModificationData, invert_up),
            },
            FieldInfoData {
                name: "InvertForward",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CoordinateModificationData, invert_forward),
            },
        ],
    }),
    array_type: Some(COORDINATEMODIFICATIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CoordinateModificationData {
    fn type_info() -> &'static TypeInfo {
        COORDINATEMODIFICATIONDATA_TYPE_INFO
    }
}


pub const COORDINATEMODIFICATIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoordinateModificationData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("CoordinateModificationData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WidgetReferenceEntityData {
}

pub const WIDGETREFERENCEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WidgetReferenceEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOGICREFERENCEOBJECTDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WIDGETREFERENCEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WidgetReferenceEntityData {
    fn type_info() -> &'static TypeInfo {
        WIDGETREFERENCEENTITYDATA_TYPE_INFO
    }
}


pub const WIDGETREFERENCEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WidgetReferenceEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("WidgetReferenceEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DiceUIVectorShapeAsset {
    pub shapes: Vec<DiceUIVectorShape>,
    pub layout_rect: super::core::Vec4,
}

pub const DICEUIVECTORSHAPEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIVectorShapeAsset",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINERPOLICYASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Shapes",
                flags: MemberInfoFlags::new(144),
                field_type: DICEUIVECTORSHAPE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(DiceUIVectorShapeAsset, shapes),
            },
            FieldInfoData {
                name: "LayoutRect",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(DiceUIVectorShapeAsset, layout_rect),
            },
        ],
    }),
    array_type: Some(DICEUIVECTORSHAPEASSET_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DiceUIVectorShapeAsset {
    fn type_info() -> &'static TypeInfo {
        DICEUIVECTORSHAPEASSET_TYPE_INFO
    }
}


pub const DICEUIVECTORSHAPEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIVectorShapeAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceUIVectorShapeAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
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
    pub inner_paths: Vec<DiceUIVectorPath>,
    pub triangulated_points: Vec<super::core::Vec2>,
    pub triangulated_expansions: Vec<super::core::Vec2>,
    pub triangulated_colors: Vec<super::core::Vec4>,
    pub triangulated_indices: Vec<u16>,
}

pub const DICEUIVECTORSHAPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIVectorShape",
    flags: MemberInfoFlags::new(73),
    module: "DiceCommonsShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(DiceUIVectorShape, color),
            },
            FieldInfoData {
                name: "Alpha",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DiceUIVectorShape, alpha),
            },
            FieldInfoData {
                name: "SpecifyInnerOuterWidths",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DiceUIVectorShape, specify_inner_outer_widths),
            },
            FieldInfoData {
                name: "InnerWidth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DiceUIVectorShape, inner_width),
            },
            FieldInfoData {
                name: "OuterWidth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DiceUIVectorShape, outer_width),
            },
            FieldInfoData {
                name: "LineWidth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DiceUIVectorShape, line_width),
            },
            FieldInfoData {
                name: "StartCapType",
                flags: MemberInfoFlags::new(0),
                field_type: DICEUIVECTORSHAPECAPTYPE_TYPE_INFO,
                rust_offset: offset_of!(DiceUIVectorShape, start_cap_type),
            },
            FieldInfoData {
                name: "EndCapType",
                flags: MemberInfoFlags::new(0),
                field_type: DICEUIVECTORSHAPECAPTYPE_TYPE_INFO,
                rust_offset: offset_of!(DiceUIVectorShape, end_cap_type),
            },
            FieldInfoData {
                name: "DrawStyle",
                flags: MemberInfoFlags::new(0),
                field_type: DICEUIVECTORSHAPEDRAWSTYLE_TYPE_INFO,
                rust_offset: offset_of!(DiceUIVectorShape, draw_style),
            },
            FieldInfoData {
                name: "Path",
                flags: MemberInfoFlags::new(0),
                field_type: DICEUIVECTORPATH_TYPE_INFO,
                rust_offset: offset_of!(DiceUIVectorShape, path),
            },
            FieldInfoData {
                name: "InnerPaths",
                flags: MemberInfoFlags::new(144),
                field_type: DICEUIVECTORPATH_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(DiceUIVectorShape, inner_paths),
            },
            FieldInfoData {
                name: "TriangulatedPoints",
                flags: MemberInfoFlags::new(144),
                field_type: VEC2_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(DiceUIVectorShape, triangulated_points),
            },
            FieldInfoData {
                name: "TriangulatedExpansions",
                flags: MemberInfoFlags::new(144),
                field_type: VEC2_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(DiceUIVectorShape, triangulated_expansions),
            },
            FieldInfoData {
                name: "TriangulatedColors",
                flags: MemberInfoFlags::new(144),
                field_type: VEC4_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(DiceUIVectorShape, triangulated_colors),
            },
            FieldInfoData {
                name: "TriangulatedIndices",
                flags: MemberInfoFlags::new(144),
                field_type: UINT16_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(DiceUIVectorShape, triangulated_indices),
            },
        ],
    }),
    array_type: Some(DICEUIVECTORSHAPE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DiceUIVectorShape {
    fn type_info() -> &'static TypeInfo {
        DICEUIVECTORSHAPE_TYPE_INFO
    }
}


pub const DICEUIVECTORSHAPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIVectorShape-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceUIVectorShape-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DiceUIVectorPath {
    pub corners: Vec<DiceUIVectorPathCorner>,
}

pub const DICEUIVECTORPATH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIVectorPath",
    flags: MemberInfoFlags::new(73),
    module: "DiceCommonsShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Corners",
                flags: MemberInfoFlags::new(144),
                field_type: DICEUIVECTORPATHCORNER_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(DiceUIVectorPath, corners),
            },
        ],
    }),
    array_type: Some(DICEUIVECTORPATH_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DiceUIVectorPath {
    fn type_info() -> &'static TypeInfo {
        DICEUIVECTORPATH_TYPE_INFO
    }
}


pub const DICEUIVECTORPATH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIVectorPath-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceUIVectorPath-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DiceUIVectorPathCorner {
    pub corner_type: DiceUIVectorPathCornerType,
    pub radius: f32,
    pub position: super::core::Vec2,
    pub expansion: super::core::Vec2,
    pub color: super::core::Vec3,
    pub alpha: f32,
}

pub const DICEUIVECTORPATHCORNER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIVectorPathCorner",
    flags: MemberInfoFlags::new(36937),
    module: "DiceCommonsShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "CornerType",
                flags: MemberInfoFlags::new(0),
                field_type: DICEUIVECTORPATHCORNERTYPE_TYPE_INFO,
                rust_offset: offset_of!(DiceUIVectorPathCorner, corner_type),
            },
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DiceUIVectorPathCorner, radius),
            },
            FieldInfoData {
                name: "Position",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(DiceUIVectorPathCorner, position),
            },
            FieldInfoData {
                name: "Expansion",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(DiceUIVectorPathCorner, expansion),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(DiceUIVectorPathCorner, color),
            },
            FieldInfoData {
                name: "Alpha",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DiceUIVectorPathCorner, alpha),
            },
        ],
    }),
    array_type: Some(DICEUIVECTORPATHCORNER_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DiceUIVectorPathCorner {
    fn type_info() -> &'static TypeInfo {
        DICEUIVECTORPATHCORNER_TYPE_INFO
    }
}


pub const DICEUIVECTORPATHCORNER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIVectorPathCorner-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceUIVectorPathCorner-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum DiceUIVectorPathCornerType {
    #[default]
    DiceUIVectorPathCornerType_Bevel = 0,
    DiceUIVectorPathCornerType_Miter = 1,
    DiceUIVectorPathCornerType_Rounded = 2,
}

pub const DICEUIVECTORPATHCORNERTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIVectorPathCornerType",
    flags: MemberInfoFlags::new(49429),
    module: "DiceCommonsShared",
    data: TypeInfoData::Enum,
    array_type: Some(DICEUIVECTORPATHCORNERTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DiceUIVectorPathCornerType {
    fn type_info() -> &'static TypeInfo {
        DICEUIVECTORPATHCORNERTYPE_TYPE_INFO
    }
}


pub const DICEUIVECTORPATHCORNERTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIVectorPathCornerType-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceUIVectorPathCornerType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum DiceUIVectorShapeDrawStyle {
    #[default]
    DiceUIVectorShapeDrawStyle_Lines = 0,
    DiceUIVectorShapeDrawStyle_Outline = 1,
    DiceUIVectorShapeDrawStyle_Filled = 2,
}

pub const DICEUIVECTORSHAPEDRAWSTYLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIVectorShapeDrawStyle",
    flags: MemberInfoFlags::new(49429),
    module: "DiceCommonsShared",
    data: TypeInfoData::Enum,
    array_type: Some(DICEUIVECTORSHAPEDRAWSTYLE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DiceUIVectorShapeDrawStyle {
    fn type_info() -> &'static TypeInfo {
        DICEUIVECTORSHAPEDRAWSTYLE_TYPE_INFO
    }
}


pub const DICEUIVECTORSHAPEDRAWSTYLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIVectorShapeDrawStyle-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceUIVectorShapeDrawStyle-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum DiceUIVectorShapeCapType {
    #[default]
    DiceUIVectorShapeCapType_None = 0,
    DiceUIVectorShapeCapType_Square = 1,
    DiceUIVectorShapeCapType_Rounded = 2,
}

pub const DICEUIVECTORSHAPECAPTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIVectorShapeCapType",
    flags: MemberInfoFlags::new(49429),
    module: "DiceCommonsShared",
    data: TypeInfoData::Enum,
    array_type: Some(DICEUIVECTORSHAPECAPTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DiceUIVectorShapeCapType {
    fn type_info() -> &'static TypeInfo {
        DICEUIVECTORSHAPECAPTYPE_TYPE_INFO
    }
}


pub const DICEUIVECTORSHAPECAPTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIVectorShapeCapType-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceUIVectorShapeCapType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LocalizedStringIdPickerEntityData {
    pub realm: super::core::Realm,
    pub sid: String,
}

pub const LOCALIZEDSTRINGIDPICKERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedStringIdPickerEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(LocalizedStringIdPickerEntityData, realm),
            },
            FieldInfoData {
                name: "Sid",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(LocalizedStringIdPickerEntityData, sid),
            },
        ],
    }),
    array_type: Some(LOCALIZEDSTRINGIDPICKERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LocalizedStringIdPickerEntityData {
    fn type_info() -> &'static TypeInfo {
        LOCALIZEDSTRINGIDPICKERENTITYDATA_TYPE_INFO
    }
}


pub const LOCALIZEDSTRINGIDPICKERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedStringIdPickerEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("LocalizedStringIdPickerEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CheckedLocalizedStringEntityData {
    pub dynamic_string_id: super::u_i_incubator_shared::LocalizedStringId,
    pub debug_string: String,
    pub sid: String,
}

pub const CHECKEDLOCALIZEDSTRINGENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CheckedLocalizedStringEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCALIZEDSTRINGENTITYBASEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DynamicStringId",
                flags: MemberInfoFlags::new(0),
                field_type: LOCALIZEDSTRINGID_TYPE_INFO,
                rust_offset: offset_of!(CheckedLocalizedStringEntityData, dynamic_string_id),
            },
            FieldInfoData {
                name: "DebugString",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(CheckedLocalizedStringEntityData, debug_string),
            },
            FieldInfoData {
                name: "Sid",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(CheckedLocalizedStringEntityData, sid),
            },
        ],
    }),
    array_type: Some(CHECKEDLOCALIZEDSTRINGENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CheckedLocalizedStringEntityData {
    fn type_info() -> &'static TypeInfo {
        CHECKEDLOCALIZEDSTRINGENTITYDATA_TYPE_INFO
    }
}


pub const CHECKEDLOCALIZEDSTRINGENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CheckedLocalizedStringEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("CheckedLocalizedStringEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MaterialRelationTriggarableEffectData {
    pub effect: super::effect_base::EffectBlueprint,
}

pub const MATERIALRELATIONTRIGGARABLEEFFECTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialRelationTriggarableEffectData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSMATERIALRELATIONPROPERTYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Effect",
                flags: MemberInfoFlags::new(0),
                field_type: EFFECTBLUEPRINT_TYPE_INFO,
                rust_offset: offset_of!(MaterialRelationTriggarableEffectData, effect),
            },
        ],
    }),
    array_type: Some(MATERIALRELATIONTRIGGARABLEEFFECTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MaterialRelationTriggarableEffectData {
    fn type_info() -> &'static TypeInfo {
        MATERIALRELATIONTRIGGARABLEEFFECTDATA_TYPE_INFO
    }
}


pub const MATERIALRELATIONTRIGGARABLEEFFECTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialRelationTriggarableEffectData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("MaterialRelationTriggarableEffectData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MaterialBasedEffectComponentData {
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
    pub effect_parameters: Vec<super::effect_base::EffectParameter>,
}

pub const MATERIALBASEDEFFECTCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialBasedEffectComponentData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMECOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "AutoStart",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MaterialBasedEffectComponentData, auto_start),
            },
            FieldInfoData {
                name: "OverrideHeight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MaterialBasedEffectComponentData, override_height),
            },
            FieldInfoData {
                name: "LocalPlayerOnly",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MaterialBasedEffectComponentData, local_player_only),
            },
            FieldInfoData {
                name: "Material",
                flags: MemberInfoFlags::new(0),
                field_type: MATERIALDECL_TYPE_INFO,
                rust_offset: offset_of!(MaterialBasedEffectComponentData, material),
            },
            FieldInfoData {
                name: "TransformEffectWithComponent",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MaterialBasedEffectComponentData, transform_effect_with_component),
            },
            FieldInfoData {
                name: "Snapping",
                flags: MemberInfoFlags::new(0),
                field_type: SNAPTYPE_TYPE_INFO,
                rust_offset: offset_of!(MaterialBasedEffectComponentData, snapping),
            },
            FieldInfoData {
                name: "UseRayCast",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MaterialBasedEffectComponentData, use_ray_cast),
            },
            FieldInfoData {
                name: "SpawnEffectOnLookupLocation",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MaterialBasedEffectComponentData, spawn_effect_on_lookup_location),
            },
            FieldInfoData {
                name: "RayDirection",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(MaterialBasedEffectComponentData, ray_direction),
            },
            FieldInfoData {
                name: "RayDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MaterialBasedEffectComponentData, ray_distance),
            },
            FieldInfoData {
                name: "SeeThrough",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MaterialBasedEffectComponentData, see_through),
            },
            FieldInfoData {
                name: "Penetrable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MaterialBasedEffectComponentData, penetrable),
            },
            FieldInfoData {
                name: "IncludeTerrain",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MaterialBasedEffectComponentData, include_terrain),
            },
            FieldInfoData {
                name: "MaxInstances",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(MaterialBasedEffectComponentData, max_instances),
            },
            FieldInfoData {
                name: "EffectParameters",
                flags: MemberInfoFlags::new(144),
                field_type: EFFECTPARAMETER_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MaterialBasedEffectComponentData, effect_parameters),
            },
        ],
    }),
    array_type: Some(MATERIALBASEDEFFECTCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for MaterialBasedEffectComponentData {
    fn type_info() -> &'static TypeInfo {
        MATERIALBASEDEFFECTCOMPONENTDATA_TYPE_INFO
    }
}


pub const MATERIALBASEDEFFECTCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialBasedEffectComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("MaterialBasedEffectComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum SnapType {
    #[default]
    mbeNoSnap = 0,
    mbeSnapToTerrain = 1,
    mbeSnapToWater = 2,
    mbeSnapToClosest = 3,
}

pub const SNAPTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnapType",
    flags: MemberInfoFlags::new(49429),
    module: "DiceCommonsShared",
    data: TypeInfoData::Enum,
    array_type: Some(SNAPTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SnapType {
    fn type_info() -> &'static TypeInfo {
        SNAPTYPE_TYPE_INFO
    }
}


pub const SNAPTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnapType-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("SnapType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LocatorComponentData {
    pub realm: super::core::Realm,
    pub is_used_as_link_target: bool,
}

pub const LOCATORCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocatorComponentData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMECOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(LocatorComponentData, realm),
            },
            FieldInfoData {
                name: "IsUsedAsLinkTarget",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LocatorComponentData, is_used_as_link_target),
            },
        ],
    }),
    array_type: Some(LOCATORCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LocatorComponentData {
    fn type_info() -> &'static TypeInfo {
        LOCATORCOMPONENTDATA_TYPE_INFO
    }
}


pub const LOCATORCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocatorComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("LocatorComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ActorPhysicsComponentData {
}

pub const ACTORPHYSICSCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ActorPhysicsComponentData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(STATICMODELPHYSICSCOMPONENTDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ACTORPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ActorPhysicsComponentData {
    fn type_info() -> &'static TypeInfo {
        ACTORPHYSICSCOMPONENTDATA_TYPE_INFO
    }
}


pub const ACTORPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ActorPhysicsComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ActorPhysicsComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ActorCustomizationComponentData {
    pub customization: ActorCustomizationData,
}

pub const ACTORCUSTOMIZATIONCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ActorCustomizationComponentData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMECOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Customization",
                flags: MemberInfoFlags::new(0),
                field_type: ACTORCUSTOMIZATIONDATA_TYPE_INFO,
                rust_offset: offset_of!(ActorCustomizationComponentData, customization),
            },
        ],
    }),
    array_type: Some(ACTORCUSTOMIZATIONCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ActorCustomizationComponentData {
    fn type_info() -> &'static TypeInfo {
        ACTORCUSTOMIZATIONCOMPONENTDATA_TYPE_INFO
    }
}


pub const ACTORCUSTOMIZATIONCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ActorCustomizationComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ActorCustomizationComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ActorCustomizationData {
    pub visual_groups: Vec<super::game_shared::CustomizeVisual>,
    pub ant_game_states: Vec<super::game_shared::WriteAntGameStateData>,
}

pub const ACTORCUSTOMIZATIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ActorCustomizationData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINERPOLICYASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "VisualGroups",
                flags: MemberInfoFlags::new(144),
                field_type: CUSTOMIZEVISUAL_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ActorCustomizationData, visual_groups),
            },
            FieldInfoData {
                name: "AntGameStates",
                flags: MemberInfoFlags::new(144),
                field_type: WRITEANTGAMESTATEDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ActorCustomizationData, ant_game_states),
            },
        ],
    }),
    array_type: Some(ACTORCUSTOMIZATIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ActorCustomizationData {
    fn type_info() -> &'static TypeInfo {
        ACTORCUSTOMIZATIONDATA_TYPE_INFO
    }
}


pub const ACTORCUSTOMIZATIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ActorCustomizationData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ActorCustomizationData-Array"),
    array_type: None,
    alignment: 8,
};



pub const DRAWVERLETCAPSULE_MAT4_FLOAT32_FLOAT32_VEC4__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DrawVerletCapsule(Mat4,Float32,Float32,Vec4)",
    flags: MemberInfoFlags::new(793),
    module: "DiceCommonsShared",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CoolDownGateEntityData {
    pub realm: super::core::Realm,
    pub cool_down_time: f32,
    pub start_opened: bool,
}

pub const COOLDOWNGATEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoolDownGateEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(CoolDownGateEntityData, realm),
            },
            FieldInfoData {
                name: "CoolDownTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CoolDownGateEntityData, cool_down_time),
            },
            FieldInfoData {
                name: "StartOpened",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CoolDownGateEntityData, start_opened),
            },
        ],
    }),
    array_type: Some(COOLDOWNGATEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CoolDownGateEntityData {
    fn type_info() -> &'static TypeInfo {
        COOLDOWNGATEENTITYDATA_TYPE_INFO
    }
}


pub const COOLDOWNGATEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoolDownGateEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("CoolDownGateEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ConditionalPropertyEntityData {
    pub realm: super::core::Realm,
    pub condition: bool,
    pub type_hash: i32,
    pub value_if_false_property_hash: i32,
    pub value_if_true_property_hash: i32,
    pub out_hash: i32,
}

pub const CONDITIONALPROPERTYENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalPropertyEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(ConditionalPropertyEntityData, realm),
            },
            FieldInfoData {
                name: "Condition",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ConditionalPropertyEntityData, condition),
            },
            FieldInfoData {
                name: "TypeHash",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ConditionalPropertyEntityData, type_hash),
            },
            FieldInfoData {
                name: "ValueIfFalsePropertyHash",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ConditionalPropertyEntityData, value_if_false_property_hash),
            },
            FieldInfoData {
                name: "ValueIfTruePropertyHash",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ConditionalPropertyEntityData, value_if_true_property_hash),
            },
            FieldInfoData {
                name: "OutHash",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ConditionalPropertyEntityData, out_hash),
            },
        ],
    }),
    array_type: Some(CONDITIONALPROPERTYENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ConditionalPropertyEntityData {
    fn type_info() -> &'static TypeInfo {
        CONDITIONALPROPERTYENTITYDATA_TYPE_INFO
    }
}


pub const CONDITIONALPROPERTYENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalPropertyEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ConditionalPropertyEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientEmitTraceBookmarkEntityData {
    pub bookmark_name: String,
    pub bookmark_description: String,
}

pub const CLIENTEMITTRACEBOOKMARKENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEmitTraceBookmarkEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "BookmarkName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ClientEmitTraceBookmarkEntityData, bookmark_name),
            },
            FieldInfoData {
                name: "BookmarkDescription",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ClientEmitTraceBookmarkEntityData, bookmark_description),
            },
        ],
    }),
    array_type: Some(CLIENTEMITTRACEBOOKMARKENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ClientEmitTraceBookmarkEntityData {
    fn type_info() -> &'static TypeInfo {
        CLIENTEMITTRACEBOOKMARKENTITYDATA_TYPE_INFO
    }
}


pub const CLIENTEMITTRACEBOOKMARKENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEmitTraceBookmarkEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ClientEmitTraceBookmarkEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CameraShakeEntityData {
    pub local_player: super::core::LocalPlayerId,
    pub transform: super::core::LinearTransform,
    pub enabled: bool,
    pub trigger_shake_profile: super::core::FloatCurve,
    pub pitch: CameraShakeAxisData,
    pub yaw: CameraShakeAxisData,
    pub roll: CameraShakeAxisData,
    pub fade_out_start_distance: f32,
    pub fade_out_end_distance: f32,
    pub trigger_shake_time: f32,
    pub amplitude: f32,
    pub intensity: f32,
}

pub const CAMERASHAKEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraShakeEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "LocalPlayer",
                flags: MemberInfoFlags::new(0),
                field_type: LOCALPLAYERID_TYPE_INFO,
                rust_offset: offset_of!(CameraShakeEntityData, local_player),
            },
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(CameraShakeEntityData, transform),
            },
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CameraShakeEntityData, enabled),
            },
            FieldInfoData {
                name: "TriggerShakeProfile",
                flags: MemberInfoFlags::new(0),
                field_type: FLOATCURVE_TYPE_INFO,
                rust_offset: offset_of!(CameraShakeEntityData, trigger_shake_profile),
            },
            FieldInfoData {
                name: "Pitch",
                flags: MemberInfoFlags::new(0),
                field_type: CAMERASHAKEAXISDATA_TYPE_INFO,
                rust_offset: offset_of!(CameraShakeEntityData, pitch),
            },
            FieldInfoData {
                name: "Yaw",
                flags: MemberInfoFlags::new(0),
                field_type: CAMERASHAKEAXISDATA_TYPE_INFO,
                rust_offset: offset_of!(CameraShakeEntityData, yaw),
            },
            FieldInfoData {
                name: "Roll",
                flags: MemberInfoFlags::new(0),
                field_type: CAMERASHAKEAXISDATA_TYPE_INFO,
                rust_offset: offset_of!(CameraShakeEntityData, roll),
            },
            FieldInfoData {
                name: "FadeOutStartDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CameraShakeEntityData, fade_out_start_distance),
            },
            FieldInfoData {
                name: "FadeOutEndDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CameraShakeEntityData, fade_out_end_distance),
            },
            FieldInfoData {
                name: "TriggerShakeTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CameraShakeEntityData, trigger_shake_time),
            },
            FieldInfoData {
                name: "Amplitude",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CameraShakeEntityData, amplitude),
            },
            FieldInfoData {
                name: "Intensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CameraShakeEntityData, intensity),
            },
        ],
    }),
    array_type: Some(CAMERASHAKEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CameraShakeEntityData {
    fn type_info() -> &'static TypeInfo {
        CAMERASHAKEENTITYDATA_TYPE_INFO
    }
}


pub const CAMERASHAKEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraShakeEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("CameraShakeEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CameraShakeAxisData {
    pub intensity: f32,
    pub sin_range: f32,
    pub sin_frequency: f32,
    pub noise_range: f32,
    pub noise_frequency: f32,
}

pub const CAMERASHAKEAXISDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraShakeAxisData",
    flags: MemberInfoFlags::new(36937),
    module: "DiceCommonsShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Intensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CameraShakeAxisData, intensity),
            },
            FieldInfoData {
                name: "SinRange",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CameraShakeAxisData, sin_range),
            },
            FieldInfoData {
                name: "SinFrequency",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CameraShakeAxisData, sin_frequency),
            },
            FieldInfoData {
                name: "NoiseRange",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CameraShakeAxisData, noise_range),
            },
            FieldInfoData {
                name: "NoiseFrequency",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CameraShakeAxisData, noise_frequency),
            },
        ],
    }),
    array_type: Some(CAMERASHAKEAXISDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for CameraShakeAxisData {
    fn type_info() -> &'static TypeInfo {
        CAMERASHAKEAXISDATA_TYPE_INFO
    }
}


pub const CAMERASHAKEAXISDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraShakeAxisData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("CameraShakeAxisData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct BlueprintSpawnReferenceObjectData {
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

pub const BLUEPRINTSPAWNREFERENCEOBJECTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintSpawnReferenceObjectData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(REFERENCEOBJECTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(BlueprintSpawnReferenceObjectData, realm),
            },
            FieldInfoData {
                name: "InitialAutoSpawn",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BlueprintSpawnReferenceObjectData, initial_auto_spawn),
            },
            FieldInfoData {
                name: "AutoSpawn",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BlueprintSpawnReferenceObjectData, auto_spawn),
            },
            FieldInfoData {
                name: "QueueSpawnEvent",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BlueprintSpawnReferenceObjectData, queue_spawn_event),
            },
            FieldInfoData {
                name: "UseAsSpawnPoint",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BlueprintSpawnReferenceObjectData, use_as_spawn_point),
            },
            FieldInfoData {
                name: "SpawnsOccupyLocations",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BlueprintSpawnReferenceObjectData, spawns_occupy_locations),
            },
            FieldInfoData {
                name: "InitialSpawnDelay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BlueprintSpawnReferenceObjectData, initial_spawn_delay),
            },
            FieldInfoData {
                name: "SpawnDelay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BlueprintSpawnReferenceObjectData, spawn_delay),
            },
            FieldInfoData {
                name: "MaxCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(BlueprintSpawnReferenceObjectData, max_count),
            },
            FieldInfoData {
                name: "MaxCountSimultaneously",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(BlueprintSpawnReferenceObjectData, max_count_simultaneously),
            },
        ],
    }),
    array_type: Some(BLUEPRINTSPAWNREFERENCEOBJECTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for BlueprintSpawnReferenceObjectData {
    fn type_info() -> &'static TypeInfo {
        BLUEPRINTSPAWNREFERENCEOBJECTDATA_TYPE_INFO
    }
}


pub const BLUEPRINTSPAWNREFERENCEOBJECTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintSpawnReferenceObjectData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("BlueprintSpawnReferenceObjectData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CharacterProxyData {
    pub template: super::game_shared::CharacterSpawnTemplateData,
    pub use_local_player_character: bool,
}

pub const CHARACTERPROXYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterProxyData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BLUEPRINTPROXYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Template",
                flags: MemberInfoFlags::new(0),
                field_type: CHARACTERSPAWNTEMPLATEDATA_TYPE_INFO,
                rust_offset: offset_of!(CharacterProxyData, template),
            },
            FieldInfoData {
                name: "UseLocalPlayerCharacter",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CharacterProxyData, use_local_player_character),
            },
        ],
    }),
    array_type: Some(CHARACTERPROXYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CharacterProxyData {
    fn type_info() -> &'static TypeInfo {
        CHARACTERPROXYDATA_TYPE_INFO
    }
}


pub const CHARACTERPROXYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterProxyData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("CharacterProxyData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct BlueprintProxyData {
    pub preview_in_game_view: bool,
    pub preview_spawn_position: super::core::LinearTransform,
    pub connected_properties: Vec<ProxyPropertyContainer>,
}

pub const BLUEPRINTPROXYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintProxyData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BLUEPRINTPROXYPROPERTYFILTERDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "PreviewInGameView",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BlueprintProxyData, preview_in_game_view),
            },
            FieldInfoData {
                name: "PreviewSpawnPosition",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(BlueprintProxyData, preview_spawn_position),
            },
            FieldInfoData {
                name: "ConnectedProperties",
                flags: MemberInfoFlags::new(144),
                field_type: PROXYPROPERTYCONTAINER_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(BlueprintProxyData, connected_properties),
            },
        ],
    }),
    array_type: Some(BLUEPRINTPROXYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for BlueprintProxyData {
    fn type_info() -> &'static TypeInfo {
        BLUEPRINTPROXYDATA_TYPE_INFO
    }
}


pub const BLUEPRINTPROXYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintProxyData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("BlueprintProxyData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProxyPropertyContainer {
    pub target_realm: super::core::Realm,
    pub target_field_id: i32,
    pub property_type_hash: u32,
}

pub const PROXYPROPERTYCONTAINER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProxyPropertyContainer",
    flags: MemberInfoFlags::new(36937),
    module: "DiceCommonsShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "TargetRealm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(ProxyPropertyContainer, target_realm),
            },
            FieldInfoData {
                name: "TargetFieldId",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ProxyPropertyContainer, target_field_id),
            },
            FieldInfoData {
                name: "PropertyTypeHash",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ProxyPropertyContainer, property_type_hash),
            },
        ],
    }),
    array_type: Some(PROXYPROPERTYCONTAINER_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for ProxyPropertyContainer {
    fn type_info() -> &'static TypeInfo {
        PROXYPROPERTYCONTAINER_TYPE_INFO
    }
}


pub const PROXYPROPERTYCONTAINER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProxyPropertyContainer-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ProxyPropertyContainer-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct BlueprintProxyPropertyFilterData {
}

pub const BLUEPRINTPROXYPROPERTYFILTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintProxyPropertyFilterData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOGICREFERENCEOBJECTDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(BLUEPRINTPROXYPROPERTYFILTERDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for BlueprintProxyPropertyFilterData {
    fn type_info() -> &'static TypeInfo {
        BLUEPRINTPROXYPROPERTYFILTERDATA_TYPE_INFO
    }
}


pub const BLUEPRINTPROXYPROPERTYFILTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintProxyPropertyFilterData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("BlueprintProxyPropertyFilterData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ActorEntityData {
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

pub const ACTORENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ActorEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMEPHYSICSENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(ActorEntityData, realm),
            },
            FieldInfoData {
                name: "RenderSkeletonBasePose",
                flags: MemberInfoFlags::new(0),
                field_type: SPARSETRANSFORMARRAY_TYPE_INFO,
                rust_offset: offset_of!(ActorEntityData, render_skeleton_base_pose),
            },
            FieldInfoData {
                name: "MeshType",
                flags: MemberInfoFlags::new(0),
                field_type: MESHTYPE_TYPE_INFO,
                rust_offset: offset_of!(ActorEntityData, mesh_type),
            },
            FieldInfoData {
                name: "MeshPartCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ActorEntityData, mesh_part_count),
            },
            FieldInfoData {
                name: "UpdateAnimatableTransform",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ActorEntityData, update_animatable_transform),
            },
            FieldInfoData {
                name: "UpdatePhysicsTransform",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ActorEntityData, update_physics_transform),
            },
            FieldInfoData {
                name: "ServerPhysicsEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ActorEntityData, server_physics_enabled),
            },
            FieldInfoData {
                name: "IsFindable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ActorEntityData, is_findable),
            },
            FieldInfoData {
                name: "EnableUpdates",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ActorEntityData, enable_updates),
            },
        ],
    }),
    array_type: Some(ACTORENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ActorEntityData {
    fn type_info() -> &'static TypeInfo {
        ACTORENTITYDATA_TYPE_INFO
    }
}


pub const ACTORENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ActorEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ActorEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum DiceUIAnalogPadType {
    #[default]
    DiceUIAnalogPadType_LeftStick = 0,
    DiceUIAnalogPadType_RightStick = 1,
    DiceUIAnalogPadType_LeftTrigger = 2,
    DiceUIAnalogPadType_RightTrigger = 3,
    DiceUIAnalogPadType_Count = 4,
}

pub const DICEUIANALOGPADTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIAnalogPadType",
    flags: MemberInfoFlags::new(49429),
    module: "DiceCommonsShared",
    data: TypeInfoData::Enum,
    array_type: Some(DICEUIANALOGPADTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DiceUIAnalogPadType {
    fn type_info() -> &'static TypeInfo {
        DICEUIANALOGPADTYPE_TYPE_INFO
    }
}


pub const DICEUIANALOGPADTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIAnalogPadType-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceUIAnalogPadType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DiceUIInputManagerSettings {
    pub automatic_typing_mode: bool,
    pub treat_touch_as_mouse: bool,
    pub scroll_wheel_dead_zone: f32,
    pub double_click_time: f32,
}

pub const DICEUIINPUTMANAGERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIInputManagerSettings",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "AutomaticTypingMode",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DiceUIInputManagerSettings, automatic_typing_mode),
            },
            FieldInfoData {
                name: "TreatTouchAsMouse",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DiceUIInputManagerSettings, treat_touch_as_mouse),
            },
            FieldInfoData {
                name: "ScrollWheelDeadZone",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DiceUIInputManagerSettings, scroll_wheel_dead_zone),
            },
            FieldInfoData {
                name: "DoubleClickTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DiceUIInputManagerSettings, double_click_time),
            },
        ],
    }),
    array_type: Some(DICEUIINPUTMANAGERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DiceUIInputManagerSettings {
    fn type_info() -> &'static TypeInfo {
        DICEUIINPUTMANAGERSETTINGS_TYPE_INFO
    }
}


pub const DICEUIINPUTMANAGERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIInputManagerSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceUIInputManagerSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DiceUITypingInputListenerElementData {
    pub max_text_length: u32,
    pub default_text: String,
    pub title: String,
    pub description: String,
    pub input_text: String,
    pub allow_multiline: bool,
    pub abort_on_escape: bool,
}

pub const DICEUITYPINGINPUTLISTENERELEMENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUITypingInputListenerElementData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIELEMENTENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MaxTextLength",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DiceUITypingInputListenerElementData, max_text_length),
            },
            FieldInfoData {
                name: "DefaultText",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(DiceUITypingInputListenerElementData, default_text),
            },
            FieldInfoData {
                name: "Title",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(DiceUITypingInputListenerElementData, title),
            },
            FieldInfoData {
                name: "Description",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(DiceUITypingInputListenerElementData, description),
            },
            FieldInfoData {
                name: "InputText",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(DiceUITypingInputListenerElementData, input_text),
            },
            FieldInfoData {
                name: "AllowMultiline",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DiceUITypingInputListenerElementData, allow_multiline),
            },
            FieldInfoData {
                name: "AbortOnEscape",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DiceUITypingInputListenerElementData, abort_on_escape),
            },
        ],
    }),
    array_type: Some(DICEUITYPINGINPUTLISTENERELEMENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DiceUITypingInputListenerElementData {
    fn type_info() -> &'static TypeInfo {
        DICEUITYPINGINPUTLISTENERELEMENTDATA_TYPE_INFO
    }
}


pub const DICEUITYPINGINPUTLISTENERELEMENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUITypingInputListenerElementData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceUITypingInputListenerElementData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DiceUIMouseInputListenerElementData {
    pub mouse_button: super::u_i::UIMouseButton,
    pub consume_input: bool,
    pub full_screen: bool,
}

pub const DICEUIMOUSEINPUTLISTENERELEMENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIMouseInputListenerElementData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIELEMENTENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MouseButton",
                flags: MemberInfoFlags::new(0),
                field_type: UIMOUSEBUTTON_TYPE_INFO,
                rust_offset: offset_of!(DiceUIMouseInputListenerElementData, mouse_button),
            },
            FieldInfoData {
                name: "ConsumeInput",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DiceUIMouseInputListenerElementData, consume_input),
            },
            FieldInfoData {
                name: "FullScreen",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DiceUIMouseInputListenerElementData, full_screen),
            },
        ],
    }),
    array_type: Some(DICEUIMOUSEINPUTLISTENERELEMENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DiceUIMouseInputListenerElementData {
    fn type_info() -> &'static TypeInfo {
        DICEUIMOUSEINPUTLISTENERELEMENTDATA_TYPE_INFO
    }
}


pub const DICEUIMOUSEINPUTLISTENERELEMENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIMouseInputListenerElementData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceUIMouseInputListenerElementData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DiceUIInputActionListenerElementData {
    pub input_action: super::u_i::UIInputAction,
    pub consume_input: bool,
}

pub const DICEUIINPUTACTIONLISTENERELEMENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIInputActionListenerElementData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIELEMENTENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "InputAction",
                flags: MemberInfoFlags::new(0),
                field_type: UIINPUTACTION_TYPE_INFO,
                rust_offset: offset_of!(DiceUIInputActionListenerElementData, input_action),
            },
            FieldInfoData {
                name: "ConsumeInput",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DiceUIInputActionListenerElementData, consume_input),
            },
        ],
    }),
    array_type: Some(DICEUIINPUTACTIONLISTENERELEMENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DiceUIInputActionListenerElementData {
    fn type_info() -> &'static TypeInfo {
        DICEUIINPUTACTIONLISTENERELEMENTDATA_TYPE_INFO
    }
}


pub const DICEUIINPUTACTIONLISTENERELEMENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIInputActionListenerElementData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceUIInputActionListenerElementData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DiceUIInputBlockerElementData {
    pub full_screen: bool,
}

pub const DICEUIINPUTBLOCKERELEMENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIInputBlockerElementData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIELEMENTENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "FullScreen",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DiceUIInputBlockerElementData, full_screen),
            },
        ],
    }),
    array_type: Some(DICEUIINPUTBLOCKERELEMENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DiceUIInputBlockerElementData {
    fn type_info() -> &'static TypeInfo {
        DICEUIINPUTBLOCKERELEMENTDATA_TYPE_INFO
    }
}


pub const DICEUIINPUTBLOCKERELEMENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIInputBlockerElementData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceUIInputBlockerElementData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DiceUIAnalogStickInputListenerElementData {
    pub analog_stick: DiceUIAnalogStick,
    pub consume_input: bool,
    pub flip_y_axis: bool,
    pub trigger_threshold: f32,
    pub dead_zone: f32,
}

pub const DICEUIANALOGSTICKINPUTLISTENERELEMENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIAnalogStickInputListenerElementData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIELEMENTENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "AnalogStick",
                flags: MemberInfoFlags::new(0),
                field_type: DICEUIANALOGSTICK_TYPE_INFO,
                rust_offset: offset_of!(DiceUIAnalogStickInputListenerElementData, analog_stick),
            },
            FieldInfoData {
                name: "ConsumeInput",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DiceUIAnalogStickInputListenerElementData, consume_input),
            },
            FieldInfoData {
                name: "FlipYAxis",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DiceUIAnalogStickInputListenerElementData, flip_y_axis),
            },
            FieldInfoData {
                name: "TriggerThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DiceUIAnalogStickInputListenerElementData, trigger_threshold),
            },
            FieldInfoData {
                name: "DeadZone",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DiceUIAnalogStickInputListenerElementData, dead_zone),
            },
        ],
    }),
    array_type: Some(DICEUIANALOGSTICKINPUTLISTENERELEMENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DiceUIAnalogStickInputListenerElementData {
    fn type_info() -> &'static TypeInfo {
        DICEUIANALOGSTICKINPUTLISTENERELEMENTDATA_TYPE_INFO
    }
}


pub const DICEUIANALOGSTICKINPUTLISTENERELEMENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIAnalogStickInputListenerElementData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceUIAnalogStickInputListenerElementData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum DiceUIAnalogStick {
    #[default]
    DiceUIAnalogStick_Left = 0,
    DiceUIAnalogStick_Right = 1,
}

pub const DICEUIANALOGSTICK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIAnalogStick",
    flags: MemberInfoFlags::new(49429),
    module: "DiceCommonsShared",
    data: TypeInfoData::Enum,
    array_type: Some(DICEUIANALOGSTICK_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DiceUIAnalogStick {
    fn type_info() -> &'static TypeInfo {
        DICEUIANALOGSTICK_TYPE_INFO
    }
}


pub const DICEUIANALOGSTICK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIAnalogStick-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceUIAnalogStick-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DiceUIAnalogPadInputListenerElementData {
    pub analog_pad_type: DiceUIAnalogPadType,
    pub consume_input: bool,
}

pub const DICEUIANALOGPADINPUTLISTENERELEMENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIAnalogPadInputListenerElementData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIELEMENTENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "AnalogPadType",
                flags: MemberInfoFlags::new(0),
                field_type: DICEUIANALOGPADTYPE_TYPE_INFO,
                rust_offset: offset_of!(DiceUIAnalogPadInputListenerElementData, analog_pad_type),
            },
            FieldInfoData {
                name: "ConsumeInput",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DiceUIAnalogPadInputListenerElementData, consume_input),
            },
        ],
    }),
    array_type: Some(DICEUIANALOGPADINPUTLISTENERELEMENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DiceUIAnalogPadInputListenerElementData {
    fn type_info() -> &'static TypeInfo {
        DICEUIANALOGPADINPUTLISTENERELEMENTDATA_TYPE_INFO
    }
}


pub const DICEUIANALOGPADINPUTLISTENERELEMENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIAnalogPadInputListenerElementData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceUIAnalogPadInputListenerElementData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DiceUIInputManagerEntityData {
}

pub const DICEUIINPUTMANAGERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIInputManagerEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DICEUIINPUTMANAGERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DiceUIInputManagerEntityData {
    fn type_info() -> &'static TypeInfo {
        DICEUIINPUTMANAGERENTITYDATA_TYPE_INFO
    }
}


pub const DICEUIINPUTMANAGERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIInputManagerEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceUIInputManagerEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DiceDebugUIInputFlowSimulationData {
    pub realm: super::core::Realm,
    pub title: String,
    pub force_game_pad_on_windows: bool,
    pub player: i32,
    pub actions: Vec<DiceUIInputFlowAction>,
}

pub const DICEDEBUGUIINPUTFLOWSIMULATIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceDebugUIInputFlowSimulationData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(DiceDebugUIInputFlowSimulationData, realm),
            },
            FieldInfoData {
                name: "Title",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(DiceDebugUIInputFlowSimulationData, title),
            },
            FieldInfoData {
                name: "ForceGamePadOnWindows",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DiceDebugUIInputFlowSimulationData, force_game_pad_on_windows),
            },
            FieldInfoData {
                name: "Player",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(DiceDebugUIInputFlowSimulationData, player),
            },
            FieldInfoData {
                name: "Actions",
                flags: MemberInfoFlags::new(144),
                field_type: DICEUIINPUTFLOWACTION_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(DiceDebugUIInputFlowSimulationData, actions),
            },
        ],
    }),
    array_type: Some(DICEDEBUGUIINPUTFLOWSIMULATIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DiceDebugUIInputFlowSimulationData {
    fn type_info() -> &'static TypeInfo {
        DICEDEBUGUIINPUTFLOWSIMULATIONDATA_TYPE_INFO
    }
}


pub const DICEDEBUGUIINPUTFLOWSIMULATIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceDebugUIInputFlowSimulationData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceDebugUIInputFlowSimulationData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DiceUIInputFlowAction {
    pub name: String,
    pub action: super::u_i::UIInputAction,
    pub press_duration: f32,
    pub wait_after_release: f32,
}

pub const DICEUIINPUTFLOWACTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIInputFlowAction",
    flags: MemberInfoFlags::new(73),
    module: "DiceCommonsShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(DiceUIInputFlowAction, name),
            },
            FieldInfoData {
                name: "Action",
                flags: MemberInfoFlags::new(0),
                field_type: UIINPUTACTION_TYPE_INFO,
                rust_offset: offset_of!(DiceUIInputFlowAction, action),
            },
            FieldInfoData {
                name: "PressDuration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DiceUIInputFlowAction, press_duration),
            },
            FieldInfoData {
                name: "WaitAfterRelease",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DiceUIInputFlowAction, wait_after_release),
            },
        ],
    }),
    array_type: Some(DICEUIINPUTFLOWACTION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DiceUIInputFlowAction {
    fn type_info() -> &'static TypeInfo {
        DICEUIINPUTFLOWACTION_TYPE_INFO
    }
}


pub const DICEUIINPUTFLOWACTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIInputFlowAction-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceUIInputFlowAction-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CharacterDefinitionComponentData {
}

pub const CHARACTERDEFINITIONCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterDefinitionComponentData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMECOMPONENTDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CHARACTERDEFINITIONCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CharacterDefinitionComponentData {
    fn type_info() -> &'static TypeInfo {
        CHARACTERDEFINITIONCOMPONENTDATA_TYPE_INFO
    }
}


pub const CHARACTERDEFINITIONCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterDefinitionComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("CharacterDefinitionComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CharacterDefinitionSpawnData {
    pub character_definition: CharacterDefinition,
}

pub const CHARACTERDEFINITIONSPAWNDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterDefinitionSpawnData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CHARACTERSPAWNREFERENCEOBJECTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "CharacterDefinition",
                flags: MemberInfoFlags::new(0),
                field_type: CHARACTERDEFINITION_TYPE_INFO,
                rust_offset: offset_of!(CharacterDefinitionSpawnData, character_definition),
            },
        ],
    }),
    array_type: Some(CHARACTERDEFINITIONSPAWNDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CharacterDefinitionSpawnData {
    fn type_info() -> &'static TypeInfo {
        CHARACTERDEFINITIONSPAWNDATA_TYPE_INFO
    }
}


pub const CHARACTERDEFINITIONSPAWNDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterDefinitionSpawnData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("CharacterDefinitionSpawnData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CharacterDefinition {
    pub character_blueprint: super::game_shared::CharacterBlueprint,
    pub face_poser_library: super::ant::AntRef,
    pub meshes: Vec<CharacterDefinitionMesh>,
}

pub const CHARACTERDEFINITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterDefinition",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINERPOLICYASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "CharacterBlueprint",
                flags: MemberInfoFlags::new(0),
                field_type: CHARACTERBLUEPRINT_TYPE_INFO,
                rust_offset: offset_of!(CharacterDefinition, character_blueprint),
            },
            FieldInfoData {
                name: "FacePoserLibrary",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(CharacterDefinition, face_poser_library),
            },
            FieldInfoData {
                name: "Meshes",
                flags: MemberInfoFlags::new(144),
                field_type: CHARACTERDEFINITIONMESH_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(CharacterDefinition, meshes),
            },
        ],
    }),
    array_type: Some(CHARACTERDEFINITION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CharacterDefinition {
    fn type_info() -> &'static TypeInfo {
        CHARACTERDEFINITION_TYPE_INFO
    }
}


pub const CHARACTERDEFINITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterDefinition-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("CharacterDefinition-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CharacterDefinitionMesh {
    pub guid: super::core::Guid,
    pub object_tag: String,
    pub mesh_asset: super::render_base::MeshBaseAsset,
    pub attach_to_joint: String,
    pub attach_offset: super::core::LinearTransform,
    pub visible: bool,
}

pub const CHARACTERDEFINITIONMESH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterDefinitionMesh",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Guid",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(CharacterDefinitionMesh, guid),
            },
            FieldInfoData {
                name: "ObjectTag",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(CharacterDefinitionMesh, object_tag),
            },
            FieldInfoData {
                name: "MeshAsset",
                flags: MemberInfoFlags::new(0),
                field_type: MESHBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(CharacterDefinitionMesh, mesh_asset),
            },
            FieldInfoData {
                name: "AttachToJoint",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(CharacterDefinitionMesh, attach_to_joint),
            },
            FieldInfoData {
                name: "AttachOffset",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(CharacterDefinitionMesh, attach_offset),
            },
            FieldInfoData {
                name: "Visible",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CharacterDefinitionMesh, visible),
            },
        ],
    }),
    array_type: Some(CHARACTERDEFINITIONMESH_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CharacterDefinitionMesh {
    fn type_info() -> &'static TypeInfo {
        CHARACTERDEFINITIONMESH_TYPE_INFO
    }
}


pub const CHARACTERDEFINITIONMESH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterDefinitionMesh-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("CharacterDefinitionMesh-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DiceAudioSettings {
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

pub const DICEAUDIOSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceAudioSettings",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ObstructionMaxQueriesPerFrame",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DiceAudioSettings, obstruction_max_queries_per_frame),
            },
            FieldInfoData {
                name: "ObstructionQueryStageThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DiceAudioSettings, obstruction_query_stage_threshold),
            },
            FieldInfoData {
                name: "ObstructionMaxObstruction",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DiceAudioSettings, obstruction_max_obstruction),
            },
            FieldInfoData {
                name: "ObstructionMaxObstructionDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DiceAudioSettings, obstruction_max_obstruction_distance),
            },
            FieldInfoData {
                name: "ObstructionRelativeVelocityThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DiceAudioSettings, obstruction_relative_velocity_threshold),
            },
            FieldInfoData {
                name: "ObstructionMaxInactiveTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DiceAudioSettings, obstruction_max_inactive_time),
            },
            FieldInfoData {
                name: "ObstructionUseRadiusAngleAsObstructionValue",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DiceAudioSettings, obstruction_use_radius_angle_as_obstruction_value),
            },
            FieldInfoData {
                name: "ObstructionMultiStageRaycastsEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DiceAudioSettings, obstruction_multi_stage_raycasts_enabled),
            },
            FieldInfoData {
                name: "ObstructionMultiStageRaycastsOuterDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DiceAudioSettings, obstruction_multi_stage_raycasts_outer_distance),
            },
            FieldInfoData {
                name: "ObstructionMultiStageRaycastsSecondStageScalar",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DiceAudioSettings, obstruction_multi_stage_raycasts_second_stage_scalar),
            },
            FieldInfoData {
                name: "ObstructionMultiStageRaycastsAttackSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DiceAudioSettings, obstruction_multi_stage_raycasts_attack_speed),
            },
            FieldInfoData {
                name: "ObstructionMultiStageRaycastsReleaseSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DiceAudioSettings, obstruction_multi_stage_raycasts_release_speed),
            },
            FieldInfoData {
                name: "ObstructionMultiStageRaycastsFirstStageAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DiceAudioSettings, obstruction_multi_stage_raycasts_first_stage_angle),
            },
            FieldInfoData {
                name: "ObstructionMultiStageRaycastsMaxObstruction",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DiceAudioSettings, obstruction_multi_stage_raycasts_max_obstruction),
            },
        ],
    }),
    array_type: Some(DICEAUDIOSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DiceAudioSettings {
    fn type_info() -> &'static TypeInfo {
        DICEAUDIOSETTINGS_TYPE_INFO
    }
}


pub const DICEAUDIOSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceAudioSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceAudioSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DistanceScopeStageData {
    pub distance: f32,
}

pub const DISTANCESCOPESTAGEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistanceScopeStageData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SOUNDSCOPESTAGEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Distance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DistanceScopeStageData, distance),
            },
        ],
    }),
    array_type: Some(DISTANCESCOPESTAGEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DistanceScopeStageData {
    fn type_info() -> &'static TypeInfo {
        DISTANCESCOPESTAGEDATA_TYPE_INFO
    }
}


pub const DISTANCESCOPESTAGEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistanceScopeStageData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DistanceScopeStageData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ComboScopeStageData {
    pub newest_count: u32,
    pub newest_threshold: f32,
    pub closest_count: u32,
}

pub const COMBOSCOPESTAGEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ComboScopeStageData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SOUNDSCOPESTAGEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "NewestCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ComboScopeStageData, newest_count),
            },
            FieldInfoData {
                name: "NewestThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ComboScopeStageData, newest_threshold),
            },
            FieldInfoData {
                name: "ClosestCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ComboScopeStageData, closest_count),
            },
        ],
    }),
    array_type: Some(COMBOSCOPESTAGEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ComboScopeStageData {
    fn type_info() -> &'static TypeInfo {
        COMBOSCOPESTAGEDATA_TYPE_INFO
    }
}


pub const COMBOSCOPESTAGEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ComboScopeStageData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ComboScopeStageData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AngularScopeStageData {
    pub inner_angle: f32,
    pub outer_angle: f32,
    pub scalling_factor: f32,
    pub count: u32,
}

pub const ANGULARSCOPESTAGEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngularScopeStageData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SOUNDSCOPESTAGEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "InnerAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AngularScopeStageData, inner_angle),
            },
            FieldInfoData {
                name: "OuterAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AngularScopeStageData, outer_angle),
            },
            FieldInfoData {
                name: "ScallingFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AngularScopeStageData, scalling_factor),
            },
            FieldInfoData {
                name: "Count",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(AngularScopeStageData, count),
            },
        ],
    }),
    array_type: Some(ANGULARSCOPESTAGEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AngularScopeStageData {
    fn type_info() -> &'static TypeInfo {
        ANGULARSCOPESTAGEDATA_TYPE_INFO
    }
}


pub const ANGULARSCOPESTAGEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AngularScopeStageData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("AngularScopeStageData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WhooshbyPlayerEntityData {
    pub whooshby_closing_sound: super::audio::SoundAsset,
    pub whooshby_trigger_closing_distance_threshold: f32,
    pub whooshby_separating_sound: super::audio::SoundAsset,
    pub whooshby_trigger_separating_distance_threshold: f32,
}

pub const WHOOSHBYPLAYERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WhooshbyPlayerEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "WhooshbyClosingSound",
                flags: MemberInfoFlags::new(0),
                field_type: SOUNDASSET_TYPE_INFO,
                rust_offset: offset_of!(WhooshbyPlayerEntityData, whooshby_closing_sound),
            },
            FieldInfoData {
                name: "WhooshbyTriggerClosingDistanceThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WhooshbyPlayerEntityData, whooshby_trigger_closing_distance_threshold),
            },
            FieldInfoData {
                name: "WhooshbySeparatingSound",
                flags: MemberInfoFlags::new(0),
                field_type: SOUNDASSET_TYPE_INFO,
                rust_offset: offset_of!(WhooshbyPlayerEntityData, whooshby_separating_sound),
            },
            FieldInfoData {
                name: "WhooshbyTriggerSeparatingDistanceThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WhooshbyPlayerEntityData, whooshby_trigger_separating_distance_threshold),
            },
        ],
    }),
    array_type: Some(WHOOSHBYPLAYERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WhooshbyPlayerEntityData {
    fn type_info() -> &'static TypeInfo {
        WHOOSHBYPLAYERENTITYDATA_TYPE_INFO
    }
}


pub const WHOOSHBYPLAYERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WhooshbyPlayerEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("WhooshbyPlayerEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AudioProximityReverbEntityData {
    pub reverbs: Vec<AudioProximityReverbData>,
    pub indooriness_attack: f32,
    pub indooriness_decay: f32,
    pub surface_closeness_attack: f32,
    pub surface_closeness_decay: f32,
    pub local_player_id: super::core::LocalPlayerId,
}

pub const AUDIOPROXIMITYREVERBENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioProximityReverbEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Reverbs",
                flags: MemberInfoFlags::new(144),
                field_type: AUDIOPROXIMITYREVERBDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(AudioProximityReverbEntityData, reverbs),
            },
            FieldInfoData {
                name: "IndoorinessAttack",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AudioProximityReverbEntityData, indooriness_attack),
            },
            FieldInfoData {
                name: "IndoorinessDecay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AudioProximityReverbEntityData, indooriness_decay),
            },
            FieldInfoData {
                name: "SurfaceClosenessAttack",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AudioProximityReverbEntityData, surface_closeness_attack),
            },
            FieldInfoData {
                name: "SurfaceClosenessDecay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AudioProximityReverbEntityData, surface_closeness_decay),
            },
            FieldInfoData {
                name: "LocalPlayerId",
                flags: MemberInfoFlags::new(0),
                field_type: LOCALPLAYERID_TYPE_INFO,
                rust_offset: offset_of!(AudioProximityReverbEntityData, local_player_id),
            },
        ],
    }),
    array_type: Some(AUDIOPROXIMITYREVERBENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AudioProximityReverbEntityData {
    fn type_info() -> &'static TypeInfo {
        AUDIOPROXIMITYREVERBENTITYDATA_TYPE_INFO
    }
}


pub const AUDIOPROXIMITYREVERBENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioProximityReverbEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("AudioProximityReverbEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AudioProximityReverbData {
    pub impulse_response: super::audio::ImpulseResponseAsset,
    pub gain: f32,
    pub indooriness_response: super::core::FloatCurve,
    pub surface_closeness_response: super::core::FloatCurve,
}

pub const AUDIOPROXIMITYREVERBDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioProximityReverbData",
    flags: MemberInfoFlags::new(73),
    module: "DiceCommonsShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ImpulseResponse",
                flags: MemberInfoFlags::new(0),
                field_type: IMPULSERESPONSEASSET_TYPE_INFO,
                rust_offset: offset_of!(AudioProximityReverbData, impulse_response),
            },
            FieldInfoData {
                name: "Gain",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AudioProximityReverbData, gain),
            },
            FieldInfoData {
                name: "IndoorinessResponse",
                flags: MemberInfoFlags::new(0),
                field_type: FLOATCURVE_TYPE_INFO,
                rust_offset: offset_of!(AudioProximityReverbData, indooriness_response),
            },
            FieldInfoData {
                name: "SurfaceClosenessResponse",
                flags: MemberInfoFlags::new(0),
                field_type: FLOATCURVE_TYPE_INFO,
                rust_offset: offset_of!(AudioProximityReverbData, surface_closeness_response),
            },
        ],
    }),
    array_type: Some(AUDIOPROXIMITYREVERBDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AudioProximityReverbData {
    fn type_info() -> &'static TypeInfo {
        AUDIOPROXIMITYREVERBDATA_TYPE_INFO
    }
}


pub const AUDIOPROXIMITYREVERBDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioProximityReverbData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("AudioProximityReverbData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AudioProximityDetectorReaderEntityData {
    pub scaled_enclosedness_min: f32,
    pub scaled_enclosedness_max: f32,
    pub scaled_enclosedness_attack: f32,
    pub scaled_enclosedness_decay: f32,
    pub local_player_id: super::core::LocalPlayerId,
}

pub const AUDIOPROXIMITYDETECTORREADERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioProximityDetectorReaderEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ScaledEnclosednessMin",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AudioProximityDetectorReaderEntityData, scaled_enclosedness_min),
            },
            FieldInfoData {
                name: "ScaledEnclosednessMax",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AudioProximityDetectorReaderEntityData, scaled_enclosedness_max),
            },
            FieldInfoData {
                name: "ScaledEnclosednessAttack",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AudioProximityDetectorReaderEntityData, scaled_enclosedness_attack),
            },
            FieldInfoData {
                name: "ScaledEnclosednessDecay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AudioProximityDetectorReaderEntityData, scaled_enclosedness_decay),
            },
            FieldInfoData {
                name: "LocalPlayerId",
                flags: MemberInfoFlags::new(0),
                field_type: LOCALPLAYERID_TYPE_INFO,
                rust_offset: offset_of!(AudioProximityDetectorReaderEntityData, local_player_id),
            },
        ],
    }),
    array_type: Some(AUDIOPROXIMITYDETECTORREADERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AudioProximityDetectorReaderEntityData {
    fn type_info() -> &'static TypeInfo {
        AUDIOPROXIMITYDETECTORREADERENTITYDATA_TYPE_INFO
    }
}


pub const AUDIOPROXIMITYDETECTORREADERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioProximityDetectorReaderEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("AudioProximityDetectorReaderEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AudioProximityDetectorEntityData {
    pub offset: super::core::Vec3,
    pub raycast_radius: f32,
    pub raycast_count: u32,
    pub forward_bias_max_distance: f32,
    pub forward_bias_max_speed: f32,
    pub forward_bias_min_speed: f32,
    pub proximity_type: ProximityDetectorType,
}

pub const AUDIOPROXIMITYDETECTORENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioProximityDetectorEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Offset",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AudioProximityDetectorEntityData, offset),
            },
            FieldInfoData {
                name: "RaycastRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AudioProximityDetectorEntityData, raycast_radius),
            },
            FieldInfoData {
                name: "RaycastCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(AudioProximityDetectorEntityData, raycast_count),
            },
            FieldInfoData {
                name: "ForwardBiasMaxDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AudioProximityDetectorEntityData, forward_bias_max_distance),
            },
            FieldInfoData {
                name: "ForwardBiasMaxSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AudioProximityDetectorEntityData, forward_bias_max_speed),
            },
            FieldInfoData {
                name: "ForwardBiasMinSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AudioProximityDetectorEntityData, forward_bias_min_speed),
            },
            FieldInfoData {
                name: "ProximityType",
                flags: MemberInfoFlags::new(0),
                field_type: PROXIMITYDETECTORTYPE_TYPE_INFO,
                rust_offset: offset_of!(AudioProximityDetectorEntityData, proximity_type),
            },
        ],
    }),
    array_type: Some(AUDIOPROXIMITYDETECTORENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AudioProximityDetectorEntityData {
    fn type_info() -> &'static TypeInfo {
        AUDIOPROXIMITYDETECTORENTITYDATA_TYPE_INFO
    }
}


pub const AUDIOPROXIMITYDETECTORENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioProximityDetectorEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("AudioProximityDetectorEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ProximityDetectorType {
    #[default]
    ProximityDetectorType_Audio = 0,
    ProximityDetectorType_Targeting = 1,
}

pub const PROXIMITYDETECTORTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProximityDetectorType",
    flags: MemberInfoFlags::new(49429),
    module: "DiceCommonsShared",
    data: TypeInfoData::Enum,
    array_type: Some(PROXIMITYDETECTORTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ProximityDetectorType {
    fn type_info() -> &'static TypeInfo {
        PROXIMITYDETECTORTYPE_TYPE_INFO
    }
}


pub const PROXIMITYDETECTORTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProximityDetectorType-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("ProximityDetectorType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct VoiceOverIntervalEntityData {
    pub interval: super::audio::VoiceOverInterval,
    pub time_threshold: f32,
    pub reset_if_threshold_reached: bool,
}

pub const VOICEOVERINTERVALENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceOverIntervalEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Interval",
                flags: MemberInfoFlags::new(0),
                field_type: VOICEOVERINTERVAL_TYPE_INFO,
                rust_offset: offset_of!(VoiceOverIntervalEntityData, interval),
            },
            FieldInfoData {
                name: "TimeThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VoiceOverIntervalEntityData, time_threshold),
            },
            FieldInfoData {
                name: "ResetIfThresholdReached",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VoiceOverIntervalEntityData, reset_if_threshold_reached),
            },
        ],
    }),
    array_type: Some(VOICEOVERINTERVALENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VoiceOverIntervalEntityData {
    fn type_info() -> &'static TypeInfo {
        VOICEOVERINTERVALENTITYDATA_TYPE_INFO
    }
}


pub const VOICEOVERINTERVALENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceOverIntervalEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("VoiceOverIntervalEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VoiceOverConversationCheckEntityData {
    pub queue_group: super::audio::VoiceOverConversationQueueGroup,
    pub continuous_update: bool,
}

pub const VOICEOVERCONVERSATIONCHECKENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceOverConversationCheckEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "QueueGroup",
                flags: MemberInfoFlags::new(0),
                field_type: VOICEOVERCONVERSATIONQUEUEGROUP_TYPE_INFO,
                rust_offset: offset_of!(VoiceOverConversationCheckEntityData, queue_group),
            },
            FieldInfoData {
                name: "ContinuousUpdate",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VoiceOverConversationCheckEntityData, continuous_update),
            },
        ],
    }),
    array_type: Some(VOICEOVERCONVERSATIONCHECKENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VoiceOverConversationCheckEntityData {
    fn type_info() -> &'static TypeInfo {
        VOICEOVERCONVERSATIONCHECKENTITYDATA_TYPE_INFO
    }
}


pub const VOICEOVERCONVERSATIONCHECKENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceOverConversationCheckEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("VoiceOverConversationCheckEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct VoiceOverContextAreaResultEntityData {
    pub position: super::core::Vec3,
}

pub const VOICEOVERCONTEXTAREARESULTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceOverContextAreaResultEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Position",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(VoiceOverContextAreaResultEntityData, position),
            },
        ],
    }),
    array_type: Some(VOICEOVERCONTEXTAREARESULTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for VoiceOverContextAreaResultEntityData {
    fn type_info() -> &'static TypeInfo {
        VOICEOVERCONTEXTAREARESULTENTITYDATA_TYPE_INFO
    }
}


pub const VOICEOVERCONTEXTAREARESULTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceOverContextAreaResultEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("VoiceOverContextAreaResultEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VoiceOverContextAreaEntityData {
    pub enable_on_creation: bool,
    pub context_id: i32,
}

pub const VOICEOVERCONTEXTAREAENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceOverContextAreaEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "EnableOnCreation",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VoiceOverContextAreaEntityData, enable_on_creation),
            },
            FieldInfoData {
                name: "ContextId",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(VoiceOverContextAreaEntityData, context_id),
            },
        ],
    }),
    array_type: Some(VOICEOVERCONTEXTAREAENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VoiceOverContextAreaEntityData {
    fn type_info() -> &'static TypeInfo {
        VOICEOVERCONTEXTAREAENTITYDATA_TYPE_INFO
    }
}


pub const VOICEOVERCONTEXTAREAENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VoiceOverContextAreaEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("VoiceOverContextAreaEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct VehicleSoundEntityData {
    pub velocity_parameter: super::audio::AudioGraphParameter,
    pub angular_velocity_parameter: super::audio::AudioGraphParameter,
    pub g_force_parameter: super::audio::AudioGraphParameter,
    pub local_player_in_vehicle_parameter: super::audio::AudioGraphParameter,
    pub interior_cam_parameter: super::audio::AudioGraphParameter,
    pub in_driver_pos_parameter: super::audio::AudioGraphParameter,
    pub roll_parameter: super::audio::AudioGraphParameter,
    pub tilt_parameter: super::audio::AudioGraphParameter,
    pub yaw_parameter: super::audio::AudioGraphParameter,
    pub roll_speed_parameter: super::audio::AudioGraphParameter,
    pub tilt_speed_parameter: super::audio::AudioGraphParameter,
    pub yaw_speed_parameter: super::audio::AudioGraphParameter,
    pub throttle_input_parameter: super::audio::AudioGraphParameter,
    pub roll_input_parameter: super::audio::AudioGraphParameter,
    pub yaw_input_parameter: super::audio::AudioGraphParameter,
    pub tilt_input_parameter: super::audio::AudioGraphParameter,
    pub camera_fov_parameter: super::audio::AudioGraphParameter,
    pub free_camera_active_parameter: super::audio::AudioGraphParameter,
    pub throttle_input_action: i32,
    pub roll_input_action: i32,
    pub yaw_input_action: i32,
    pub tilt_input_action: i32,
    pub proximity_output_max_speed: f32,
    pub proximity_output: super::audio::OutputNodeData,
    pub proximity_enabled_parameter: super::audio::AudioGraphParameter,
    pub proximity_distance_parameter: super::audio::AudioGraphParameter,
    pub proximity_enclosedness_parameter: super::audio::AudioGraphParameter,
}

pub const VEHICLESOUNDENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleSoundEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DICESOUNDENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "VelocityParameter",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHPARAMETER_TYPE_INFO,
                rust_offset: offset_of!(VehicleSoundEntityData, velocity_parameter),
            },
            FieldInfoData {
                name: "AngularVelocityParameter",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHPARAMETER_TYPE_INFO,
                rust_offset: offset_of!(VehicleSoundEntityData, angular_velocity_parameter),
            },
            FieldInfoData {
                name: "GForceParameter",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHPARAMETER_TYPE_INFO,
                rust_offset: offset_of!(VehicleSoundEntityData, g_force_parameter),
            },
            FieldInfoData {
                name: "LocalPlayerInVehicleParameter",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHPARAMETER_TYPE_INFO,
                rust_offset: offset_of!(VehicleSoundEntityData, local_player_in_vehicle_parameter),
            },
            FieldInfoData {
                name: "InteriorCamParameter",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHPARAMETER_TYPE_INFO,
                rust_offset: offset_of!(VehicleSoundEntityData, interior_cam_parameter),
            },
            FieldInfoData {
                name: "InDriverPosParameter",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHPARAMETER_TYPE_INFO,
                rust_offset: offset_of!(VehicleSoundEntityData, in_driver_pos_parameter),
            },
            FieldInfoData {
                name: "RollParameter",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHPARAMETER_TYPE_INFO,
                rust_offset: offset_of!(VehicleSoundEntityData, roll_parameter),
            },
            FieldInfoData {
                name: "TiltParameter",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHPARAMETER_TYPE_INFO,
                rust_offset: offset_of!(VehicleSoundEntityData, tilt_parameter),
            },
            FieldInfoData {
                name: "YawParameter",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHPARAMETER_TYPE_INFO,
                rust_offset: offset_of!(VehicleSoundEntityData, yaw_parameter),
            },
            FieldInfoData {
                name: "RollSpeedParameter",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHPARAMETER_TYPE_INFO,
                rust_offset: offset_of!(VehicleSoundEntityData, roll_speed_parameter),
            },
            FieldInfoData {
                name: "TiltSpeedParameter",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHPARAMETER_TYPE_INFO,
                rust_offset: offset_of!(VehicleSoundEntityData, tilt_speed_parameter),
            },
            FieldInfoData {
                name: "YawSpeedParameter",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHPARAMETER_TYPE_INFO,
                rust_offset: offset_of!(VehicleSoundEntityData, yaw_speed_parameter),
            },
            FieldInfoData {
                name: "ThrottleInputParameter",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHPARAMETER_TYPE_INFO,
                rust_offset: offset_of!(VehicleSoundEntityData, throttle_input_parameter),
            },
            FieldInfoData {
                name: "RollInputParameter",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHPARAMETER_TYPE_INFO,
                rust_offset: offset_of!(VehicleSoundEntityData, roll_input_parameter),
            },
            FieldInfoData {
                name: "YawInputParameter",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHPARAMETER_TYPE_INFO,
                rust_offset: offset_of!(VehicleSoundEntityData, yaw_input_parameter),
            },
            FieldInfoData {
                name: "TiltInputParameter",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHPARAMETER_TYPE_INFO,
                rust_offset: offset_of!(VehicleSoundEntityData, tilt_input_parameter),
            },
            FieldInfoData {
                name: "CameraFovParameter",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHPARAMETER_TYPE_INFO,
                rust_offset: offset_of!(VehicleSoundEntityData, camera_fov_parameter),
            },
            FieldInfoData {
                name: "FreeCameraActiveParameter",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHPARAMETER_TYPE_INFO,
                rust_offset: offset_of!(VehicleSoundEntityData, free_camera_active_parameter),
            },
            FieldInfoData {
                name: "ThrottleInputAction",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleSoundEntityData, throttle_input_action),
            },
            FieldInfoData {
                name: "RollInputAction",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleSoundEntityData, roll_input_action),
            },
            FieldInfoData {
                name: "YawInputAction",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleSoundEntityData, yaw_input_action),
            },
            FieldInfoData {
                name: "TiltInputAction",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleSoundEntityData, tilt_input_action),
            },
            FieldInfoData {
                name: "ProximityOutputMaxSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleSoundEntityData, proximity_output_max_speed),
            },
            FieldInfoData {
                name: "ProximityOutput",
                flags: MemberInfoFlags::new(0),
                field_type: OUTPUTNODEDATA_TYPE_INFO,
                rust_offset: offset_of!(VehicleSoundEntityData, proximity_output),
            },
            FieldInfoData {
                name: "ProximityEnabledParameter",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHPARAMETER_TYPE_INFO,
                rust_offset: offset_of!(VehicleSoundEntityData, proximity_enabled_parameter),
            },
            FieldInfoData {
                name: "ProximityDistanceParameter",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHPARAMETER_TYPE_INFO,
                rust_offset: offset_of!(VehicleSoundEntityData, proximity_distance_parameter),
            },
            FieldInfoData {
                name: "ProximityEnclosednessParameter",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHPARAMETER_TYPE_INFO,
                rust_offset: offset_of!(VehicleSoundEntityData, proximity_enclosedness_parameter),
            },
        ],
    }),
    array_type: Some(VEHICLESOUNDENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for VehicleSoundEntityData {
    fn type_info() -> &'static TypeInfo {
        VEHICLESOUNDENTITYDATA_TYPE_INFO
    }
}


pub const VEHICLESOUNDENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleSoundEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("VehicleSoundEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SoundProviderEntityData {
    pub sound_bank: Vec<super::audio::SoundAsset>,
    pub index: i32,
}

pub const SOUNDPROVIDERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoundProviderEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "SoundBank",
                flags: MemberInfoFlags::new(144),
                field_type: SOUNDASSET_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SoundProviderEntityData, sound_bank),
            },
            FieldInfoData {
                name: "Index",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(SoundProviderEntityData, index),
            },
        ],
    }),
    array_type: Some(SOUNDPROVIDERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SoundProviderEntityData {
    fn type_info() -> &'static TypeInfo {
        SOUNDPROVIDERENTITYDATA_TYPE_INFO
    }
}


pub const SOUNDPROVIDERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoundProviderEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("SoundProviderEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SoundAssetDataEntityData {
    pub sound_assets: Vec<super::audio::SoundAsset>,
    pub data_assets: Vec<super::audio::SoundDataAsset>,
}

pub const SOUNDASSETDATAENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoundAssetDataEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "SoundAssets",
                flags: MemberInfoFlags::new(144),
                field_type: SOUNDASSET_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SoundAssetDataEntityData, sound_assets),
            },
            FieldInfoData {
                name: "DataAssets",
                flags: MemberInfoFlags::new(144),
                field_type: SOUNDDATAASSET_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SoundAssetDataEntityData, data_assets),
            },
        ],
    }),
    array_type: Some(SOUNDASSETDATAENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SoundAssetDataEntityData {
    fn type_info() -> &'static TypeInfo {
        SOUNDASSETDATAENTITYDATA_TYPE_INFO
    }
}


pub const SOUNDASSETDATAENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoundAssetDataEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("SoundAssetDataEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SoundActivityTesterEntityData {
    pub test_case_name: String,
    pub time_out: f32,
    pub auto_success: bool,
    pub auto_start: bool,
    pub detect_at_least_one_asset: bool,
    pub assets_to_track: Vec<super::core::Asset>,
}

pub const SOUNDACTIVITYTESTERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoundActivityTesterEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "TestCaseName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(SoundActivityTesterEntityData, test_case_name),
            },
            FieldInfoData {
                name: "TimeOut",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SoundActivityTesterEntityData, time_out),
            },
            FieldInfoData {
                name: "AutoSuccess",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SoundActivityTesterEntityData, auto_success),
            },
            FieldInfoData {
                name: "AutoStart",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SoundActivityTesterEntityData, auto_start),
            },
            FieldInfoData {
                name: "DetectAtLeastOneAsset",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SoundActivityTesterEntityData, detect_at_least_one_asset),
            },
            FieldInfoData {
                name: "AssetsToTrack",
                flags: MemberInfoFlags::new(144),
                field_type: ASSET_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SoundActivityTesterEntityData, assets_to_track),
            },
        ],
    }),
    array_type: Some(SOUNDACTIVITYTESTERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SoundActivityTesterEntityData {
    fn type_info() -> &'static TypeInfo {
        SOUNDACTIVITYTESTERENTITYDATA_TYPE_INFO
    }
}


pub const SOUNDACTIVITYTESTERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoundActivityTesterEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("SoundActivityTesterEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MusicEventPriorityEntityData {
    pub realm: super::core::Realm,
    pub music_event_names: Vec<String>,
}

pub const MUSICEVENTPRIORITYENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MusicEventPriorityEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(MusicEventPriorityEntityData, realm),
            },
            FieldInfoData {
                name: "MusicEventNames",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MusicEventPriorityEntityData, music_event_names),
            },
        ],
    }),
    array_type: Some(MUSICEVENTPRIORITYENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MusicEventPriorityEntityData {
    fn type_info() -> &'static TypeInfo {
        MUSICEVENTPRIORITYENTITYDATA_TYPE_INFO
    }
}


pub const MUSICEVENTPRIORITYENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MusicEventPriorityEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("MusicEventPriorityEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DiceSoundSpatialEntityData {
    pub sound: super::audio::SoundAsset,
    pub play_on_creation: bool,
    pub amplitude: f32,
}

pub const DICESOUNDSPATIALENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceSoundSpatialEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Sound",
                flags: MemberInfoFlags::new(0),
                field_type: SOUNDASSET_TYPE_INFO,
                rust_offset: offset_of!(DiceSoundSpatialEntityData, sound),
            },
            FieldInfoData {
                name: "PlayOnCreation",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DiceSoundSpatialEntityData, play_on_creation),
            },
            FieldInfoData {
                name: "Amplitude",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DiceSoundSpatialEntityData, amplitude),
            },
        ],
    }),
    array_type: Some(DICESOUNDSPATIALENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DiceSoundSpatialEntityData {
    fn type_info() -> &'static TypeInfo {
        DICESOUNDSPATIALENTITYDATA_TYPE_INFO
    }
}


pub const DICESOUNDSPATIALENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceSoundSpatialEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceSoundSpatialEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DiceSoundEntityData {
    pub attach: EntityAttachData,
    pub update_position: bool,
    pub transform: super::core::LinearTransform,
    pub default_sound: super::audio::SoundAsset,
    pub sound: super::audio::SoundAsset,
    pub play_on_creation: bool,
    pub enable_on_creation: bool,
    pub forget_on_destroy: bool,
    pub master_amplitude: f32,
}

pub const DICESOUNDENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceSoundEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Attach",
                flags: MemberInfoFlags::new(0),
                field_type: ENTITYATTACHDATA_TYPE_INFO,
                rust_offset: offset_of!(DiceSoundEntityData, attach),
            },
            FieldInfoData {
                name: "UpdatePosition",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DiceSoundEntityData, update_position),
            },
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(DiceSoundEntityData, transform),
            },
            FieldInfoData {
                name: "DefaultSound",
                flags: MemberInfoFlags::new(0),
                field_type: SOUNDASSET_TYPE_INFO,
                rust_offset: offset_of!(DiceSoundEntityData, default_sound),
            },
            FieldInfoData {
                name: "Sound",
                flags: MemberInfoFlags::new(0),
                field_type: SOUNDASSET_TYPE_INFO,
                rust_offset: offset_of!(DiceSoundEntityData, sound),
            },
            FieldInfoData {
                name: "PlayOnCreation",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DiceSoundEntityData, play_on_creation),
            },
            FieldInfoData {
                name: "EnableOnCreation",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DiceSoundEntityData, enable_on_creation),
            },
            FieldInfoData {
                name: "ForgetOnDestroy",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DiceSoundEntityData, forget_on_destroy),
            },
            FieldInfoData {
                name: "MasterAmplitude",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DiceSoundEntityData, master_amplitude),
            },
        ],
    }),
    array_type: Some(DICESOUNDENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DiceSoundEntityData {
    fn type_info() -> &'static TypeInfo {
        DICESOUNDENTITYDATA_TYPE_INFO
    }
}


pub const DICESOUNDENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceSoundEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceSoundEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DiceSoundAreaEntityData {
    pub sound: super::audio::SoundAsset,
    pub big_world: super::gameplay_sim::BigWorldSettingsAsset,
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

pub const DICESOUNDAREAENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceSoundAreaEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Sound",
                flags: MemberInfoFlags::new(0),
                field_type: SOUNDASSET_TYPE_INFO,
                rust_offset: offset_of!(DiceSoundAreaEntityData, sound),
            },
            FieldInfoData {
                name: "BigWorld",
                flags: MemberInfoFlags::new(0),
                field_type: BIGWORLDSETTINGSASSET_TYPE_INFO,
                rust_offset: offset_of!(DiceSoundAreaEntityData, big_world),
            },
            FieldInfoData {
                name: "PerimeterSize",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DiceSoundAreaEntityData, perimeter_size),
            },
            FieldInfoData {
                name: "RelevanceMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DiceSoundAreaEntityData, relevance_multiplier),
            },
            FieldInfoData {
                name: "MinRelevanceBudget",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DiceSoundAreaEntityData, min_relevance_budget),
            },
            FieldInfoData {
                name: "RelevanceFalloff",
                flags: MemberInfoFlags::new(0),
                field_type: FADECURVETYPE_TYPE_INFO,
                rust_offset: offset_of!(DiceSoundAreaEntityData, relevance_falloff),
            },
            FieldInfoData {
                name: "EnableOnCreation",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DiceSoundAreaEntityData, enable_on_creation),
            },
            FieldInfoData {
                name: "Priority",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DiceSoundAreaEntityData, priority),
            },
            FieldInfoData {
                name: "UseLegacyBehavior",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DiceSoundAreaEntityData, use_legacy_behavior),
            },
            FieldInfoData {
                name: "FaceListener",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DiceSoundAreaEntityData, face_listener),
            },
            FieldInfoData {
                name: "IgnoreVerticalPerimeter",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DiceSoundAreaEntityData, ignore_vertical_perimeter),
            },
            FieldInfoData {
                name: "AreaType",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(DiceSoundAreaEntityData, area_type),
            },
        ],
    }),
    array_type: Some(DICESOUNDAREAENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DiceSoundAreaEntityData {
    fn type_info() -> &'static TypeInfo {
        DICESOUNDAREAENTITYDATA_TYPE_INFO
    }
}


pub const DICESOUNDAREAENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceSoundAreaEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DiceSoundAreaEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AudioCurveFactorEntityData {
    pub curves_guids: Vec<super::core::Guid>,
    pub factor: f32,
    pub reset_factor_on_destroy: bool,
}

pub const AUDIOCURVEFACTORENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioCurveFactorEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "CurvesGuids",
                flags: MemberInfoFlags::new(144),
                field_type: GUID_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(AudioCurveFactorEntityData, curves_guids),
            },
            FieldInfoData {
                name: "Factor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AudioCurveFactorEntityData, factor),
            },
            FieldInfoData {
                name: "ResetFactorOnDestroy",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AudioCurveFactorEntityData, reset_factor_on_destroy),
            },
        ],
    }),
    array_type: Some(AUDIOCURVEFACTORENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AudioCurveFactorEntityData {
    fn type_info() -> &'static TypeInfo {
        AUDIOCURVEFACTORENTITYDATA_TYPE_INFO
    }
}


pub const AUDIOCURVEFACTORENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioCurveFactorEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("AudioCurveFactorEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DebrisClusterSoundsComponentData {
    pub debris_cluster_sounds: Vec<DebrisClusterSound>,
}

pub const DEBRISCLUSTERSOUNDSCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisClusterSoundsComponentData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMECOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DebrisClusterSounds",
                flags: MemberInfoFlags::new(144),
                field_type: DEBRISCLUSTERSOUND_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterSoundsComponentData, debris_cluster_sounds),
            },
        ],
    }),
    array_type: Some(DEBRISCLUSTERSOUNDSCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DebrisClusterSoundsComponentData {
    fn type_info() -> &'static TypeInfo {
        DEBRISCLUSTERSOUNDSCOMPONENTDATA_TYPE_INFO
    }
}


pub const DEBRISCLUSTERSOUNDSCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisClusterSoundsComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DebrisClusterSoundsComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DebrisClusterSound {
    pub activation_sound: super::audio::SoundAsset,
    pub collision_sound: super::audio::SoundAsset,
    pub collision_sound_speed_threshold: f32,
    pub part_index: u32,
}

pub const DEBRISCLUSTERSOUND_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisClusterSound",
    flags: MemberInfoFlags::new(73),
    module: "DiceCommonsShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ActivationSound",
                flags: MemberInfoFlags::new(0),
                field_type: SOUNDASSET_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterSound, activation_sound),
            },
            FieldInfoData {
                name: "CollisionSound",
                flags: MemberInfoFlags::new(0),
                field_type: SOUNDASSET_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterSound, collision_sound),
            },
            FieldInfoData {
                name: "CollisionSoundSpeedThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterSound, collision_sound_speed_threshold),
            },
            FieldInfoData {
                name: "PartIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterSound, part_index),
            },
        ],
    }),
    array_type: Some(DEBRISCLUSTERSOUND_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DebrisClusterSound {
    fn type_info() -> &'static TypeInfo {
        DEBRISCLUSTERSOUND_TYPE_INFO
    }
}


pub const DEBRISCLUSTERSOUND_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisClusterSound-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DebrisClusterSound-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DofReaderEntityData {
    pub dof_set: super::ant::AntRef,
    pub dof_set_name: String,
    pub dof_names_hash_id: Vec<i32>,
    pub start_reading_continously_on_spawn: bool,
    pub read_once_on_spawn: bool,
}

pub const DOFREADERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DofReaderEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DofSet",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(DofReaderEntityData, dof_set),
            },
            FieldInfoData {
                name: "DofSetName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(DofReaderEntityData, dof_set_name),
            },
            FieldInfoData {
                name: "DofNamesHashId",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(DofReaderEntityData, dof_names_hash_id),
            },
            FieldInfoData {
                name: "StartReadingContinouslyOnSpawn",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DofReaderEntityData, start_reading_continously_on_spawn),
            },
            FieldInfoData {
                name: "ReadOnceOnSpawn",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DofReaderEntityData, read_once_on_spawn),
            },
        ],
    }),
    array_type: Some(DOFREADERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DofReaderEntityData {
    fn type_info() -> &'static TypeInfo {
        DOFREADERENTITYDATA_TYPE_INFO
    }
}


pub const DOFREADERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DofReaderEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("DofReaderEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AnimatableCullingEntityData {
    pub culling_level: EntityCullingLevel,
    pub culling_distance: f32,
    pub force_cull: bool,
    pub enabled: bool,
}

pub const ANIMATABLECULLINGENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimatableCullingEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommonsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "CullingLevel",
                flags: MemberInfoFlags::new(0),
                field_type: ENTITYCULLINGLEVEL_TYPE_INFO,
                rust_offset: offset_of!(AnimatableCullingEntityData, culling_level),
            },
            FieldInfoData {
                name: "CullingDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AnimatableCullingEntityData, culling_distance),
            },
            FieldInfoData {
                name: "ForceCull",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AnimatableCullingEntityData, force_cull),
            },
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AnimatableCullingEntityData, enabled),
            },
        ],
    }),
    array_type: Some(ANIMATABLECULLINGENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AnimatableCullingEntityData {
    fn type_info() -> &'static TypeInfo {
        ANIMATABLECULLINGENTITYDATA_TYPE_INFO
    }
}


pub const ANIMATABLECULLINGENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimatableCullingEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("AnimatableCullingEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EntityCullingLevel {
    #[default]
    EntityCullingLevel_InView = 0,
    EntityCullingLevel_Distance = 1,
    EntityCullingLevel_InViewAndDistance = 2,
    EntityCullingLevel_ForceCull = 3,
    EntityCullingLevel_DisableCulling = 4,
}

pub const ENTITYCULLINGLEVEL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityCullingLevel",
    flags: MemberInfoFlags::new(49429),
    module: "DiceCommonsShared",
    data: TypeInfoData::Enum,
    array_type: Some(ENTITYCULLINGLEVEL_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EntityCullingLevel {
    fn type_info() -> &'static TypeInfo {
        ENTITYCULLINGLEVEL_TYPE_INFO
    }
}


pub const ENTITYCULLINGLEVEL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityCullingLevel-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommonsShared",
    data: TypeInfoData::Array("EntityCullingLevel-Array"),
    array_type: None,
    alignment: 8,
};


