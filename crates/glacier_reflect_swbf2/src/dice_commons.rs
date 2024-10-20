use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_dice_commons_types(registry: &mut TypeRegistry) {
    registry.register_type(SELECTABLEACTIONENTITY_TYPE_INFO);
    registry.register_type(SELECTABLEACTIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(RUMBLEENTITY_TYPE_INFO);
    registry.register_type(RUMBLEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(RANDOMACTIONSELECTORENTITY_TYPE_INFO);
    registry.register_type(RANDOMACTIONSELECTORENTITY_ARRAY_TYPE_INFO);
    registry.register_type(PROPERTYSTATUSENTITY_TYPE_INFO);
    registry.register_type(PROPERTYSTATUSENTITY_ARRAY_TYPE_INFO);
    registry.register_type(PROPERTYSELECTENTITY_TYPE_INFO);
    registry.register_type(PROPERTYSELECTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(PRIORITIZEDBOOLENTITY_TYPE_INFO);
    registry.register_type(PRIORITIZEDBOOLENTITY_ARRAY_TYPE_INFO);
    registry.register_type(NESTEDCONDITIONALPROPERTYENTITY_TYPE_INFO);
    registry.register_type(NESTEDCONDITIONALPROPERTYENTITY_ARRAY_TYPE_INFO);
    registry.register_type(MULTIPROPERTYGATEENTITY_TYPE_INFO);
    registry.register_type(MULTIPROPERTYGATEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(LOGITECHLEDPULSEEFFECTENTITY_TYPE_INFO);
    registry.register_type(LOGITECHLEDPULSEEFFECTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(LOGITECHLEDINPUTCONCEPTIDENTIFIERENTITY_TYPE_INFO);
    registry.register_type(LOGITECHLEDINPUTCONCEPTIDENTIFIERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(LOGITECHLEDFADEEFFECTENTITY_TYPE_INFO);
    registry.register_type(LOGITECHLEDFADEEFFECTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(LOGITECHLEDCONSTANTEFFECTENTITY_TYPE_INFO);
    registry.register_type(LOGITECHLEDCONSTANTEFFECTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(LOGITECHLEDCONDITIONALINPUTCONCEPTIDENTIFIERENTITY_TYPE_INFO);
    registry.register_type(LOGITECHLEDCONDITIONALINPUTCONCEPTIDENTIFIERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(LOGITECHLEDBARENTITY_TYPE_INFO);
    registry.register_type(LOGITECHLEDBARENTITY_ARRAY_TYPE_INFO);
    registry.register_type(LOCALLOCATORENTITY_TYPE_INFO);
    registry.register_type(LOCALLOCATORENTITY_ARRAY_TYPE_INFO);
    registry.register_type(COOLDOWNGATEENTITY_TYPE_INFO);
    registry.register_type(COOLDOWNGATEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CONDITIONALPROPERTYENTITY_TYPE_INFO);
    registry.register_type(CONDITIONALPROPERTYENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTVALUEMATCHENTITY_TYPE_INFO);
    registry.register_type(CLIENTVALUEMATCHENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTRAYCASTDIRECTIONENTITY_TYPE_INFO);
    registry.register_type(CLIENTRAYCASTDIRECTIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTDICEDEBUGUIINPUTFLOWSIMULATONENTITY_TYPE_INFO);
    registry.register_type(CLIENTDICEDEBUGUIINPUTFLOWSIMULATONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCHARACTERDEFINITIONSPAWNENTITY_TYPE_INFO);
    registry.register_type(SERVERCHARACTERDEFINITIONSPAWNENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCHARACTERDEFINITIONSPAWNENTITY_TYPE_INFO);
    registry.register_type(CLIENTCHARACTERDEFINITIONSPAWNENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCHARACTERDEFINITIONCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTCHARACTERDEFINITIONCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(WHOOSHBYPLAYERENTITY_TYPE_INFO);
    registry.register_type(WHOOSHBYPLAYERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(AUDIOPROXIMITYREVERBENTITY_TYPE_INFO);
    registry.register_type(AUDIOPROXIMITYREVERBENTITY_ARRAY_TYPE_INFO);
    registry.register_type(AUDIOPROXIMITYMANAGER_TYPE_INFO);
    registry.register_type(AUDIOPROXIMITYMANAGER_ARRAY_TYPE_INFO);
    registry.register_type(AUDIOPROXIMITYDETECTORREADERENTITY_TYPE_INFO);
    registry.register_type(AUDIOPROXIMITYDETECTORREADERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(AUDIOPROXIMITYDETECTORENTITY_TYPE_INFO);
    registry.register_type(AUDIOPROXIMITYDETECTORENTITY_ARRAY_TYPE_INFO);
    registry.register_type(VEHICLESOUNDENTITY_TYPE_INFO);
    registry.register_type(VEHICLESOUNDENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SOUNDPROVIDERENTITY_TYPE_INFO);
    registry.register_type(SOUNDPROVIDERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(DICESOUNDSPATIALENTITY_TYPE_INFO);
    registry.register_type(DICESOUNDSPATIALENTITY_ARRAY_TYPE_INFO);
    registry.register_type(DICESOUNDENTITY_TYPE_INFO);
    registry.register_type(DICESOUNDENTITY_ARRAY_TYPE_INFO);
    registry.register_type(DICESOUNDAREAENTITY_TYPE_INFO);
    registry.register_type(DICESOUNDAREAENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTVOICEOVERSOURCEENTITY_TYPE_INFO);
    registry.register_type(CLIENTVOICEOVERSOURCEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTVOICEOVERSETLANGUAGEENTITY_TYPE_INFO);
    registry.register_type(CLIENTVOICEOVERSETLANGUAGEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTVOICEOVERLOCATORENTITY_TYPE_INFO);
    registry.register_type(CLIENTVOICEOVERLOCATORENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTVOICEOVERINTERVALENTITY_TYPE_INFO);
    registry.register_type(CLIENTVOICEOVERINTERVALENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTVOICEOVERCONVERSATIONCHECKENTITY_TYPE_INFO);
    registry.register_type(CLIENTVOICEOVERCONVERSATIONCHECKENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTVOICEOVERCONTEXTAREARESULTENTITY_TYPE_INFO);
    registry.register_type(CLIENTVOICEOVERCONTEXTAREARESULTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTVOICEOVERCONTEXTAREAENTITY_TYPE_INFO);
    registry.register_type(CLIENTVOICEOVERCONTEXTAREAENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSOUNDASSETDATAENTITY_TYPE_INFO);
    registry.register_type(CLIENTSOUNDASSETDATAENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSOUNDACTIVITYTESTERENTITY_TYPE_INFO);
    registry.register_type(CLIENTSOUNDACTIVITYTESTERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTMUSICEVENTPRIORITYENTITY_TYPE_INFO);
    registry.register_type(CLIENTMUSICEVENTPRIORITYENTITY_ARRAY_TYPE_INFO);
    registry.register_type(AUDIOCURVEFACTORENTITY_TYPE_INFO);
    registry.register_type(AUDIOCURVEFACTORENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTDEBRISCLUSTERSOUNDSCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTDEBRISCLUSTERSOUNDSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(ANSELFREECAMERA_TYPE_INFO);
    registry.register_type(ANSELFREECAMERA_ARRAY_TYPE_INFO);
    registry.register_type(DOFREADERENTITY_TYPE_INFO);
    registry.register_type(DOFREADERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(ANIMATABLECULLINGENTITY_TYPE_INFO);
    registry.register_type(ANIMATABLECULLINGENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTEMITTRACEBOOKMARKENTITY_TYPE_INFO);
    registry.register_type(CLIENTEMITTRACEBOOKMARKENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCHARACTERPROXYENTITY_TYPE_INFO);
    registry.register_type(CLIENTCHARACTERPROXYENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCAMERASHAKEENTITY_TYPE_INFO);
    registry.register_type(CLIENTCAMERASHAKEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTBLUEPRINTSPAWNENTITY_TYPE_INFO);
    registry.register_type(CLIENTBLUEPRINTSPAWNENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTBLUEPRINTPROXYENTITY_TYPE_INFO);
    registry.register_type(CLIENTBLUEPRINTPROXYENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTACTORENTITY_TYPE_INFO);
    registry.register_type(CLIENTACTORENTITY_ARRAY_TYPE_INFO);
    registry.register_type(BLUEPRINTSPAWNENTITY_TYPE_INFO);
    registry.register_type(BLUEPRINTSPAWNENTITY_ARRAY_TYPE_INFO);
    registry.register_type(DICEUITYPINGINPUTLISTENERELEMENT_TYPE_INFO);
    registry.register_type(DICEUITYPINGINPUTLISTENERELEMENT_ARRAY_TYPE_INFO);
    registry.register_type(DICEUIMOUSEINPUTLISTENERELEMENT_TYPE_INFO);
    registry.register_type(DICEUIMOUSEINPUTLISTENERELEMENT_ARRAY_TYPE_INFO);
    registry.register_type(DICEUIINPUTBLOCKERELEMENT_TYPE_INFO);
    registry.register_type(DICEUIINPUTBLOCKERELEMENT_ARRAY_TYPE_INFO);
    registry.register_type(DICEUIINPUTACTIONLISTENERELEMENT_TYPE_INFO);
    registry.register_type(DICEUIINPUTACTIONLISTENERELEMENT_ARRAY_TYPE_INFO);
    registry.register_type(DICEUIANALOGSTICKINPUTLISTENERELEMENT_TYPE_INFO);
    registry.register_type(DICEUIANALOGSTICKINPUTLISTENERELEMENT_ARRAY_TYPE_INFO);
    registry.register_type(DICEUIANALOGPADINPUTLISTENERELEMENT_TYPE_INFO);
    registry.register_type(DICEUIANALOGPADINPUTLISTENERELEMENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTDICEUIINPUTMANAGERENTITY_TYPE_INFO);
    registry.register_type(CLIENTDICEUIINPUTMANAGERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(LOCALIZEDSTRINGIDPICKERENTITY_TYPE_INFO);
    registry.register_type(LOCALIZEDSTRINGIDPICKERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CHECKEDLOCALIZEDSTRINGENTITY_TYPE_INFO);
    registry.register_type(CHECKEDLOCALIZEDSTRINGENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERLOCATORCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERLOCATORCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERACTORPHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERACTORPHYSICSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERACTORCUSTOMIZATIONCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERACTORCUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTMATERIALBASEDEFFECTCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTMATERIALBASEDEFFECTCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTLOCATORCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTLOCATORCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTACTORPHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTACTORPHYSICSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTACTORCUSTOMIZATIONCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTACTORCUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SIMPLEROTATIONENTITY_TYPE_INFO);
    registry.register_type(SIMPLEROTATIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SHADOWPLAYHIGHLIGHTENTITY_TYPE_INFO);
    registry.register_type(SHADOWPLAYHIGHLIGHTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERGHOSTENTITYIDOWNERWRAPPERENTITY_TYPE_INFO);
    registry.register_type(SERVERGHOSTENTITYIDOWNERWRAPPERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCHARACTERPROXYENTITY_TYPE_INFO);
    registry.register_type(SERVERCHARACTERPROXYENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERBLUEPRINTSPAWNENTITY_TYPE_INFO);
    registry.register_type(SERVERBLUEPRINTSPAWNENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERBLUEPRINTPROXYENTITY_TYPE_INFO);
    registry.register_type(SERVERBLUEPRINTPROXYENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERACTORENTITY_TYPE_INFO);
    registry.register_type(SERVERACTORENTITY_ARRAY_TYPE_INFO);
    registry.register_type(DICEJOBSCHEDULERSETTINGS_TYPE_INFO);
    registry.register_type(DICEJOBSCHEDULERSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(LOGITECHLEDSETTINGS_TYPE_INFO);
    registry.register_type(LOGITECHLEDSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(ANSELSETTINGS_TYPE_INFO);
    registry.register_type(ANSELSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(WIDGETREFERENCEENTITY_TYPE_INFO);
    registry.register_type(WIDGETREFERENCEENTITY_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SelectableActionEntity {
}

pub const SELECTABLEACTIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectableActionEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SELECTABLEACTIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SelectableActionEntity {
    fn type_info() -> &'static TypeInfo {
        SELECTABLEACTIONENTITY_TYPE_INFO
    }
}


pub const SELECTABLEACTIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectableActionEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("SelectableActionEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RumbleEntity {
}

pub const RUMBLEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RumbleEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(RUMBLEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RumbleEntity {
    fn type_info() -> &'static TypeInfo {
        RUMBLEENTITY_TYPE_INFO
    }
}


pub const RUMBLEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RumbleEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("RumbleEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RandomActionSelectorEntity {
}

pub const RANDOMACTIONSELECTORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomActionSelectorEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(RANDOMACTIONSELECTORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RandomActionSelectorEntity {
    fn type_info() -> &'static TypeInfo {
        RANDOMACTIONSELECTORENTITY_TYPE_INFO
    }
}


pub const RANDOMACTIONSELECTORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomActionSelectorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("RandomActionSelectorEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PropertyStatusEntity {
}

pub const PROPERTYSTATUSENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyStatusEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PROPERTYSTATUSENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PropertyStatusEntity {
    fn type_info() -> &'static TypeInfo {
        PROPERTYSTATUSENTITY_TYPE_INFO
    }
}


pub const PROPERTYSTATUSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyStatusEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("PropertyStatusEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PropertySelectEntity {
}

pub const PROPERTYSELECTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertySelectEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PROPERTYSELECTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PropertySelectEntity {
    fn type_info() -> &'static TypeInfo {
        PROPERTYSELECTENTITY_TYPE_INFO
    }
}


pub const PROPERTYSELECTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertySelectEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("PropertySelectEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PrioritizedBoolEntity {
}

pub const PRIORITIZEDBOOLENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PrioritizedBoolEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRIORITIZEDBOOLENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PrioritizedBoolEntity {
    fn type_info() -> &'static TypeInfo {
        PRIORITIZEDBOOLENTITY_TYPE_INFO
    }
}


pub const PRIORITIZEDBOOLENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PrioritizedBoolEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("PrioritizedBoolEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NestedConditionalPropertyEntity {
}

pub const NESTEDCONDITIONALPROPERTYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NestedConditionalPropertyEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(NESTEDCONDITIONALPROPERTYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for NestedConditionalPropertyEntity {
    fn type_info() -> &'static TypeInfo {
        NESTEDCONDITIONALPROPERTYENTITY_TYPE_INFO
    }
}


pub const NESTEDCONDITIONALPROPERTYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NestedConditionalPropertyEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("NestedConditionalPropertyEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MultiPropertyGateEntity {
}

pub const MULTIPROPERTYGATEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiPropertyGateEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(MULTIPROPERTYGATEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MultiPropertyGateEntity {
    fn type_info() -> &'static TypeInfo {
        MULTIPROPERTYGATEENTITY_TYPE_INFO
    }
}


pub const MULTIPROPERTYGATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiPropertyGateEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("MultiPropertyGateEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LogitechLEDPulseEffectEntity {
}

pub const LOGITECHLEDPULSEEFFECTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDPulseEffectEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LOGITECHLEDPULSEEFFECTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LogitechLEDPulseEffectEntity {
    fn type_info() -> &'static TypeInfo {
        LOGITECHLEDPULSEEFFECTENTITY_TYPE_INFO
    }
}


pub const LOGITECHLEDPULSEEFFECTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDPulseEffectEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("LogitechLEDPulseEffectEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LogitechLEDInputConceptIdentifierEntity {
}

pub const LOGITECHLEDINPUTCONCEPTIDENTIFIERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDInputConceptIdentifierEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LOGITECHLEDINPUTCONCEPTIDENTIFIERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LogitechLEDInputConceptIdentifierEntity {
    fn type_info() -> &'static TypeInfo {
        LOGITECHLEDINPUTCONCEPTIDENTIFIERENTITY_TYPE_INFO
    }
}


pub const LOGITECHLEDINPUTCONCEPTIDENTIFIERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDInputConceptIdentifierEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("LogitechLEDInputConceptIdentifierEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LogitechLEDFadeEffectEntity {
}

pub const LOGITECHLEDFADEEFFECTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDFadeEffectEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LOGITECHLEDFADEEFFECTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LogitechLEDFadeEffectEntity {
    fn type_info() -> &'static TypeInfo {
        LOGITECHLEDFADEEFFECTENTITY_TYPE_INFO
    }
}


pub const LOGITECHLEDFADEEFFECTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDFadeEffectEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("LogitechLEDFadeEffectEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LogitechLEDConstantEffectEntity {
}

pub const LOGITECHLEDCONSTANTEFFECTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDConstantEffectEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LOGITECHLEDCONSTANTEFFECTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LogitechLEDConstantEffectEntity {
    fn type_info() -> &'static TypeInfo {
        LOGITECHLEDCONSTANTEFFECTENTITY_TYPE_INFO
    }
}


pub const LOGITECHLEDCONSTANTEFFECTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDConstantEffectEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("LogitechLEDConstantEffectEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LogitechLEDConditionalInputConceptIdentifierEntity {
}

pub const LOGITECHLEDCONDITIONALINPUTCONCEPTIDENTIFIERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDConditionalInputConceptIdentifierEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LOGITECHLEDCONDITIONALINPUTCONCEPTIDENTIFIERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LogitechLEDConditionalInputConceptIdentifierEntity {
    fn type_info() -> &'static TypeInfo {
        LOGITECHLEDCONDITIONALINPUTCONCEPTIDENTIFIERENTITY_TYPE_INFO
    }
}


pub const LOGITECHLEDCONDITIONALINPUTCONCEPTIDENTIFIERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDConditionalInputConceptIdentifierEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("LogitechLEDConditionalInputConceptIdentifierEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LogitechLEDBarEntity {
}

pub const LOGITECHLEDBARENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDBarEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LOGITECHLEDBARENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LogitechLEDBarEntity {
    fn type_info() -> &'static TypeInfo {
        LOGITECHLEDBARENTITY_TYPE_INFO
    }
}


pub const LOGITECHLEDBARENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDBarEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("LogitechLEDBarEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LocalLocatorEntity {
}

pub const LOCALLOCATORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalLocatorEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LOCALLOCATORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LocalLocatorEntity {
    fn type_info() -> &'static TypeInfo {
        LOCALLOCATORENTITY_TYPE_INFO
    }
}


pub const LOCALLOCATORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalLocatorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("LocalLocatorEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CoolDownGateEntity {
}

pub const COOLDOWNGATEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoolDownGateEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(COOLDOWNGATEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CoolDownGateEntity {
    fn type_info() -> &'static TypeInfo {
        COOLDOWNGATEENTITY_TYPE_INFO
    }
}


pub const COOLDOWNGATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoolDownGateEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("CoolDownGateEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ConditionalPropertyEntity {
}

pub const CONDITIONALPROPERTYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalPropertyEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CONDITIONALPROPERTYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ConditionalPropertyEntity {
    fn type_info() -> &'static TypeInfo {
        CONDITIONALPROPERTYENTITY_TYPE_INFO
    }
}


pub const CONDITIONALPROPERTYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalPropertyEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ConditionalPropertyEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientValueMatchEntity {
}

pub const CLIENTVALUEMATCHENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientValueMatchEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVALUEMATCHENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientValueMatchEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTVALUEMATCHENTITY_TYPE_INFO
    }
}


pub const CLIENTVALUEMATCHENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientValueMatchEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientValueMatchEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientRaycastDirectionEntity {
}

pub const CLIENTRAYCASTDIRECTIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientRaycastDirectionEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTRAYCASTDIRECTIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientRaycastDirectionEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTRAYCASTDIRECTIONENTITY_TYPE_INFO
    }
}


pub const CLIENTRAYCASTDIRECTIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientRaycastDirectionEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientRaycastDirectionEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientDiceDebugUIInputFlowSimulatonEntity {
}

pub const CLIENTDICEDEBUGUIINPUTFLOWSIMULATONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDiceDebugUIInputFlowSimulatonEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTDICEDEBUGUIINPUTFLOWSIMULATONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientDiceDebugUIInputFlowSimulatonEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTDICEDEBUGUIINPUTFLOWSIMULATONENTITY_TYPE_INFO
    }
}


pub const CLIENTDICEDEBUGUIINPUTFLOWSIMULATONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDiceDebugUIInputFlowSimulatonEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientDiceDebugUIInputFlowSimulatonEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCharacterDefinitionSpawnEntity {
}

pub const SERVERCHARACTERDEFINITIONSPAWNENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterDefinitionSpawnEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERCHARACTERSPAWNENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERCHARACTERDEFINITIONSPAWNENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCharacterDefinitionSpawnEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERCHARACTERDEFINITIONSPAWNENTITY_TYPE_INFO
    }
}


pub const SERVERCHARACTERDEFINITIONSPAWNENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterDefinitionSpawnEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ServerCharacterDefinitionSpawnEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCharacterDefinitionSpawnEntity {
}

pub const CLIENTCHARACTERDEFINITIONSPAWNENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterDefinitionSpawnEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTCHARACTERSPAWNENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCHARACTERDEFINITIONSPAWNENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCharacterDefinitionSpawnEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTCHARACTERDEFINITIONSPAWNENTITY_TYPE_INFO
    }
}


pub const CLIENTCHARACTERDEFINITIONSPAWNENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterDefinitionSpawnEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientCharacterDefinitionSpawnEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCharacterDefinitionComponent {
}

pub const CLIENTCHARACTERDEFINITIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterDefinitionComponent",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCHARACTERDEFINITIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCharacterDefinitionComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTCHARACTERDEFINITIONCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTCHARACTERDEFINITIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterDefinitionComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientCharacterDefinitionComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WhooshbyPlayerEntity {
}

pub const WHOOSHBYPLAYERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WhooshbyPlayerEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WHOOSHBYPLAYERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for WhooshbyPlayerEntity {
    fn type_info() -> &'static TypeInfo {
        WHOOSHBYPLAYERENTITY_TYPE_INFO
    }
}


pub const WHOOSHBYPLAYERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WhooshbyPlayerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("WhooshbyPlayerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AudioProximityReverbEntity {
}

pub const AUDIOPROXIMITYREVERBENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioProximityReverbEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(AUDIOPROXIMITYREVERBENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AudioProximityReverbEntity {
    fn type_info() -> &'static TypeInfo {
        AUDIOPROXIMITYREVERBENTITY_TYPE_INFO
    }
}


pub const AUDIOPROXIMITYREVERBENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioProximityReverbEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("AudioProximityReverbEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AudioProximityManager {
}

pub const AUDIOPROXIMITYMANAGER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioProximityManager",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(AUDIOPROXIMITYMANAGER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AudioProximityManager {
    fn type_info() -> &'static TypeInfo {
        AUDIOPROXIMITYMANAGER_TYPE_INFO
    }
}


pub const AUDIOPROXIMITYMANAGER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioProximityManager-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("AudioProximityManager-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AudioProximityDetectorReaderEntity {
}

pub const AUDIOPROXIMITYDETECTORREADERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioProximityDetectorReaderEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(AUDIOPROXIMITYDETECTORREADERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AudioProximityDetectorReaderEntity {
    fn type_info() -> &'static TypeInfo {
        AUDIOPROXIMITYDETECTORREADERENTITY_TYPE_INFO
    }
}


pub const AUDIOPROXIMITYDETECTORREADERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioProximityDetectorReaderEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("AudioProximityDetectorReaderEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AudioProximityDetectorEntity {
}

pub const AUDIOPROXIMITYDETECTORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioProximityDetectorEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(AUDIOPROXIMITYDETECTORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AudioProximityDetectorEntity {
    fn type_info() -> &'static TypeInfo {
        AUDIOPROXIMITYDETECTORENTITY_TYPE_INFO
    }
}


pub const AUDIOPROXIMITYDETECTORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioProximityDetectorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("AudioProximityDetectorEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VehicleSoundEntity {
}

pub const VEHICLESOUNDENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleSoundEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DICESOUNDENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VEHICLESOUNDENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for VehicleSoundEntity {
    fn type_info() -> &'static TypeInfo {
        VEHICLESOUNDENTITY_TYPE_INFO
    }
}


pub const VEHICLESOUNDENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleSoundEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("VehicleSoundEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SoundProviderEntity {
}

pub const SOUNDPROVIDERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoundProviderEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SOUNDPROVIDERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SoundProviderEntity {
    fn type_info() -> &'static TypeInfo {
        SOUNDPROVIDERENTITY_TYPE_INFO
    }
}


pub const SOUNDPROVIDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoundProviderEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("SoundProviderEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DiceSoundSpatialEntity {
}

pub const DICESOUNDSPATIALENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceSoundSpatialEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DICESOUNDSPATIALENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DiceSoundSpatialEntity {
    fn type_info() -> &'static TypeInfo {
        DICESOUNDSPATIALENTITY_TYPE_INFO
    }
}


pub const DICESOUNDSPATIALENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceSoundSpatialEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("DiceSoundSpatialEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DiceSoundEntity {
}

pub const DICESOUNDENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceSoundEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DICESOUNDENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DiceSoundEntity {
    fn type_info() -> &'static TypeInfo {
        DICESOUNDENTITY_TYPE_INFO
    }
}


pub const DICESOUNDENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceSoundEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("DiceSoundEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DiceSoundAreaEntity {
}

pub const DICESOUNDAREAENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceSoundAreaEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DICESOUNDAREAENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DiceSoundAreaEntity {
    fn type_info() -> &'static TypeInfo {
        DICESOUNDAREAENTITY_TYPE_INFO
    }
}


pub const DICESOUNDAREAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceSoundAreaEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("DiceSoundAreaEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientVoiceOverSourceEntity {
}

pub const CLIENTVOICEOVERSOURCEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverSourceEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVOICEOVERSOURCEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientVoiceOverSourceEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTVOICEOVERSOURCEENTITY_TYPE_INFO
    }
}


pub const CLIENTVOICEOVERSOURCEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverSourceEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientVoiceOverSourceEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientVoiceOverSetLanguageEntity {
}

pub const CLIENTVOICEOVERSETLANGUAGEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverSetLanguageEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVOICEOVERSETLANGUAGEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientVoiceOverSetLanguageEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTVOICEOVERSETLANGUAGEENTITY_TYPE_INFO
    }
}


pub const CLIENTVOICEOVERSETLANGUAGEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverSetLanguageEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientVoiceOverSetLanguageEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientVoiceOverLocatorEntity {
}

pub const CLIENTVOICEOVERLOCATORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverLocatorEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVOICEOVERLOCATORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientVoiceOverLocatorEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTVOICEOVERLOCATORENTITY_TYPE_INFO
    }
}


pub const CLIENTVOICEOVERLOCATORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverLocatorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientVoiceOverLocatorEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientVoiceOverIntervalEntity {
}

pub const CLIENTVOICEOVERINTERVALENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverIntervalEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVOICEOVERINTERVALENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientVoiceOverIntervalEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTVOICEOVERINTERVALENTITY_TYPE_INFO
    }
}


pub const CLIENTVOICEOVERINTERVALENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverIntervalEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientVoiceOverIntervalEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientVoiceOverConversationCheckEntity {
}

pub const CLIENTVOICEOVERCONVERSATIONCHECKENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverConversationCheckEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVOICEOVERCONVERSATIONCHECKENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientVoiceOverConversationCheckEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTVOICEOVERCONVERSATIONCHECKENTITY_TYPE_INFO
    }
}


pub const CLIENTVOICEOVERCONVERSATIONCHECKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverConversationCheckEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientVoiceOverConversationCheckEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientVoiceOverContextAreaResultEntity {
}

pub const CLIENTVOICEOVERCONTEXTAREARESULTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverContextAreaResultEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVOICEOVERCONTEXTAREARESULTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientVoiceOverContextAreaResultEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTVOICEOVERCONTEXTAREARESULTENTITY_TYPE_INFO
    }
}


pub const CLIENTVOICEOVERCONTEXTAREARESULTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverContextAreaResultEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientVoiceOverContextAreaResultEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientVoiceOverContextAreaEntity {
}

pub const CLIENTVOICEOVERCONTEXTAREAENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverContextAreaEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVOICEOVERCONTEXTAREAENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientVoiceOverContextAreaEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTVOICEOVERCONTEXTAREAENTITY_TYPE_INFO
    }
}


pub const CLIENTVOICEOVERCONTEXTAREAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverContextAreaEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientVoiceOverContextAreaEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientSoundAssetDataEntity {
}

pub const CLIENTSOUNDASSETDATAENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoundAssetDataEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSOUNDASSETDATAENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSoundAssetDataEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTSOUNDASSETDATAENTITY_TYPE_INFO
    }
}


pub const CLIENTSOUNDASSETDATAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoundAssetDataEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientSoundAssetDataEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientSoundActivityTesterEntity {
}

pub const CLIENTSOUNDACTIVITYTESTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoundActivityTesterEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSOUNDACTIVITYTESTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSoundActivityTesterEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTSOUNDACTIVITYTESTERENTITY_TYPE_INFO
    }
}


pub const CLIENTSOUNDACTIVITYTESTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoundActivityTesterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientSoundActivityTesterEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientMusicEventPriorityEntity {
}

pub const CLIENTMUSICEVENTPRIORITYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMusicEventPriorityEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTMUSICEVENTPRIORITYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientMusicEventPriorityEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTMUSICEVENTPRIORITYENTITY_TYPE_INFO
    }
}


pub const CLIENTMUSICEVENTPRIORITYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMusicEventPriorityEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientMusicEventPriorityEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AudioCurveFactorEntity {
}

pub const AUDIOCURVEFACTORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioCurveFactorEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(AUDIOCURVEFACTORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AudioCurveFactorEntity {
    fn type_info() -> &'static TypeInfo {
        AUDIOCURVEFACTORENTITY_TYPE_INFO
    }
}


pub const AUDIOCURVEFACTORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioCurveFactorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("AudioCurveFactorEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientDebrisClusterSoundsComponent {
}

pub const CLIENTDEBRISCLUSTERSOUNDSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDebrisClusterSoundsComponent",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTDEBRISCLUSTERSOUNDSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientDebrisClusterSoundsComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTDEBRISCLUSTERSOUNDSCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTDEBRISCLUSTERSOUNDSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDebrisClusterSoundsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientDebrisClusterSoundsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AnselFreeCamera {
}

pub const ANSELFREECAMERA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnselFreeCamera",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CAMERA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ANSELFREECAMERA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AnselFreeCamera {
    fn type_info() -> &'static TypeInfo {
        ANSELFREECAMERA_TYPE_INFO
    }
}


pub const ANSELFREECAMERA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnselFreeCamera-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("AnselFreeCamera-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DofReaderEntity {
}

pub const DOFREADERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DofReaderEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DOFREADERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DofReaderEntity {
    fn type_info() -> &'static TypeInfo {
        DOFREADERENTITY_TYPE_INFO
    }
}


pub const DOFREADERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DofReaderEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("DofReaderEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AnimatableCullingEntity {
}

pub const ANIMATABLECULLINGENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimatableCullingEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ANIMATABLECULLINGENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AnimatableCullingEntity {
    fn type_info() -> &'static TypeInfo {
        ANIMATABLECULLINGENTITY_TYPE_INFO
    }
}


pub const ANIMATABLECULLINGENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimatableCullingEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("AnimatableCullingEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientEmitTraceBookmarkEntity {
}

pub const CLIENTEMITTRACEBOOKMARKENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEmitTraceBookmarkEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTEMITTRACEBOOKMARKENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientEmitTraceBookmarkEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTEMITTRACEBOOKMARKENTITY_TYPE_INFO
    }
}


pub const CLIENTEMITTRACEBOOKMARKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEmitTraceBookmarkEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientEmitTraceBookmarkEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCharacterProxyEntity {
}

pub const CLIENTCHARACTERPROXYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterProxyEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTBLUEPRINTPROXYENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCHARACTERPROXYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCharacterProxyEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTCHARACTERPROXYENTITY_TYPE_INFO
    }
}


pub const CLIENTCHARACTERPROXYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterProxyEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientCharacterProxyEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCameraShakeEntity {
}

pub const CLIENTCAMERASHAKEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCameraShakeEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCAMERASHAKEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCameraShakeEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTCAMERASHAKEENTITY_TYPE_INFO
    }
}


pub const CLIENTCAMERASHAKEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCameraShakeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientCameraShakeEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientBlueprintSpawnEntity {
}

pub const CLIENTBLUEPRINTSPAWNENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBlueprintSpawnEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BLUEPRINTSPAWNENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTBLUEPRINTSPAWNENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientBlueprintSpawnEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTBLUEPRINTSPAWNENTITY_TYPE_INFO
    }
}


pub const CLIENTBLUEPRINTSPAWNENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBlueprintSpawnEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientBlueprintSpawnEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientBlueprintProxyEntity {
}

pub const CLIENTBLUEPRINTPROXYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBlueprintProxyEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BLUEPRINTPROXYENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTBLUEPRINTPROXYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientBlueprintProxyEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTBLUEPRINTPROXYENTITY_TYPE_INFO
    }
}


pub const CLIENTBLUEPRINTPROXYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBlueprintProxyEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientBlueprintProxyEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientActorEntity {
}

pub const CLIENTACTORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientActorEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTPHYSICSENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTACTORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientActorEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTACTORENTITY_TYPE_INFO
    }
}


pub const CLIENTACTORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientActorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientActorEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BlueprintSpawnEntity {
}

pub const BLUEPRINTSPAWNENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintSpawnEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(BLUEPRINTSPAWNENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BlueprintSpawnEntity {
    fn type_info() -> &'static TypeInfo {
        BLUEPRINTSPAWNENTITY_TYPE_INFO
    }
}


pub const BLUEPRINTSPAWNENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintSpawnEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("BlueprintSpawnEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DiceUITypingInputListenerElement {
}

pub const DICEUITYPINGINPUTLISTENERELEMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUITypingInputListenerElement",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIELEMENTENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DICEUITYPINGINPUTLISTENERELEMENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DiceUITypingInputListenerElement {
    fn type_info() -> &'static TypeInfo {
        DICEUITYPINGINPUTLISTENERELEMENT_TYPE_INFO
    }
}


pub const DICEUITYPINGINPUTLISTENERELEMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUITypingInputListenerElement-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("DiceUITypingInputListenerElement-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DiceUIMouseInputListenerElement {
}

pub const DICEUIMOUSEINPUTLISTENERELEMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIMouseInputListenerElement",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIELEMENTENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DICEUIMOUSEINPUTLISTENERELEMENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DiceUIMouseInputListenerElement {
    fn type_info() -> &'static TypeInfo {
        DICEUIMOUSEINPUTLISTENERELEMENT_TYPE_INFO
    }
}


pub const DICEUIMOUSEINPUTLISTENERELEMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIMouseInputListenerElement-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("DiceUIMouseInputListenerElement-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DiceUIInputBlockerElement {
}

pub const DICEUIINPUTBLOCKERELEMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIInputBlockerElement",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIELEMENTENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DICEUIINPUTBLOCKERELEMENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DiceUIInputBlockerElement {
    fn type_info() -> &'static TypeInfo {
        DICEUIINPUTBLOCKERELEMENT_TYPE_INFO
    }
}


pub const DICEUIINPUTBLOCKERELEMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIInputBlockerElement-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("DiceUIInputBlockerElement-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DiceUIInputActionListenerElement {
}

pub const DICEUIINPUTACTIONLISTENERELEMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIInputActionListenerElement",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIELEMENTENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DICEUIINPUTACTIONLISTENERELEMENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DiceUIInputActionListenerElement {
    fn type_info() -> &'static TypeInfo {
        DICEUIINPUTACTIONLISTENERELEMENT_TYPE_INFO
    }
}


pub const DICEUIINPUTACTIONLISTENERELEMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIInputActionListenerElement-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("DiceUIInputActionListenerElement-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DiceUIAnalogStickInputListenerElement {
}

pub const DICEUIANALOGSTICKINPUTLISTENERELEMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIAnalogStickInputListenerElement",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIELEMENTENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DICEUIANALOGSTICKINPUTLISTENERELEMENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DiceUIAnalogStickInputListenerElement {
    fn type_info() -> &'static TypeInfo {
        DICEUIANALOGSTICKINPUTLISTENERELEMENT_TYPE_INFO
    }
}


pub const DICEUIANALOGSTICKINPUTLISTENERELEMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIAnalogStickInputListenerElement-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("DiceUIAnalogStickInputListenerElement-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DiceUIAnalogPadInputListenerElement {
}

pub const DICEUIANALOGPADINPUTLISTENERELEMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIAnalogPadInputListenerElement",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIELEMENTENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DICEUIANALOGPADINPUTLISTENERELEMENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DiceUIAnalogPadInputListenerElement {
    fn type_info() -> &'static TypeInfo {
        DICEUIANALOGPADINPUTLISTENERELEMENT_TYPE_INFO
    }
}


pub const DICEUIANALOGPADINPUTLISTENERELEMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIAnalogPadInputListenerElement-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("DiceUIAnalogPadInputListenerElement-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientDiceUIInputManagerEntity {
}

pub const CLIENTDICEUIINPUTMANAGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDiceUIInputManagerEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTDICEUIINPUTMANAGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientDiceUIInputManagerEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTDICEUIINPUTMANAGERENTITY_TYPE_INFO
    }
}


pub const CLIENTDICEUIINPUTMANAGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDiceUIInputManagerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientDiceUIInputManagerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LocalizedStringIdPickerEntity {
}

pub const LOCALIZEDSTRINGIDPICKERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedStringIdPickerEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LOCALIZEDSTRINGIDPICKERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LocalizedStringIdPickerEntity {
    fn type_info() -> &'static TypeInfo {
        LOCALIZEDSTRINGIDPICKERENTITY_TYPE_INFO
    }
}


pub const LOCALIZEDSTRINGIDPICKERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedStringIdPickerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("LocalizedStringIdPickerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CheckedLocalizedStringEntity {
}

pub const CHECKEDLOCALIZEDSTRINGENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CheckedLocalizedStringEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCALIZEDSTRINGENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CHECKEDLOCALIZEDSTRINGENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CheckedLocalizedStringEntity {
    fn type_info() -> &'static TypeInfo {
        CHECKEDLOCALIZEDSTRINGENTITY_TYPE_INFO
    }
}


pub const CHECKEDLOCALIZEDSTRINGENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CheckedLocalizedStringEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("CheckedLocalizedStringEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerLocatorComponent {
}

pub const SERVERLOCATORCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLocatorComponent",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERLOCATORCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerLocatorComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERLOCATORCOMPONENT_TYPE_INFO
    }
}


pub const SERVERLOCATORCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLocatorComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ServerLocatorComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerActorPhysicsComponent {
}

pub const SERVERACTORPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerActorPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERSTATICMODELPHYSICSCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERACTORPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerActorPhysicsComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERACTORPHYSICSCOMPONENT_TYPE_INFO
    }
}


pub const SERVERACTORPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerActorPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ServerActorPhysicsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerActorCustomizationComponent {
}

pub const SERVERACTORCUSTOMIZATIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerActorCustomizationComponent",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERACTORCUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerActorCustomizationComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERACTORCUSTOMIZATIONCOMPONENT_TYPE_INFO
    }
}


pub const SERVERACTORCUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerActorCustomizationComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ServerActorCustomizationComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientMaterialBasedEffectComponent {
}

pub const CLIENTMATERIALBASEDEFFECTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMaterialBasedEffectComponent",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTMATERIALBASEDEFFECTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientMaterialBasedEffectComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTMATERIALBASEDEFFECTCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTMATERIALBASEDEFFECTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMaterialBasedEffectComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientMaterialBasedEffectComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientLocatorComponent {
}

pub const CLIENTLOCATORCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLocatorComponent",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTLOCATORCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientLocatorComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTLOCATORCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTLOCATORCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLocatorComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientLocatorComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientActorPhysicsComponent {
}

pub const CLIENTACTORPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientActorPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTSTATICMODELPHYSICSCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTACTORPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientActorPhysicsComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTACTORPHYSICSCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTACTORPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientActorPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientActorPhysicsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientActorCustomizationComponent {
}

pub const CLIENTACTORCUSTOMIZATIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientActorCustomizationComponent",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTACTORCUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientActorCustomizationComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTACTORCUSTOMIZATIONCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTACTORCUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientActorCustomizationComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientActorCustomizationComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SimpleRotationEntity {
}

pub const SIMPLEROTATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimpleRotationEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SIMPLEROTATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SimpleRotationEntity {
    fn type_info() -> &'static TypeInfo {
        SIMPLEROTATIONENTITY_TYPE_INFO
    }
}


pub const SIMPLEROTATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimpleRotationEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("SimpleRotationEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShadowplayHighlightEntity {
}

pub const SHADOWPLAYHIGHLIGHTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowplayHighlightEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SHADOWPLAYHIGHLIGHTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ShadowplayHighlightEntity {
    fn type_info() -> &'static TypeInfo {
        SHADOWPLAYHIGHLIGHTENTITY_TYPE_INFO
    }
}


pub const SHADOWPLAYHIGHLIGHTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowplayHighlightEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ShadowplayHighlightEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGhostEntityIdOwnerWrapperEntity {
}

pub const SERVERGHOSTENTITYIDOWNERWRAPPERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGhostEntityIdOwnerWrapperEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERGHOSTENTITYIDOWNERWRAPPERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerGhostEntityIdOwnerWrapperEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERGHOSTENTITYIDOWNERWRAPPERENTITY_TYPE_INFO
    }
}


pub const SERVERGHOSTENTITYIDOWNERWRAPPERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGhostEntityIdOwnerWrapperEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ServerGhostEntityIdOwnerWrapperEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCharacterProxyEntity {
}

pub const SERVERCHARACTERPROXYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterProxyEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERBLUEPRINTPROXYENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERCHARACTERPROXYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCharacterProxyEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERCHARACTERPROXYENTITY_TYPE_INFO
    }
}


pub const SERVERCHARACTERPROXYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterProxyEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ServerCharacterProxyEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerBlueprintSpawnEntity {
}

pub const SERVERBLUEPRINTSPAWNENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBlueprintSpawnEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BLUEPRINTSPAWNENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERBLUEPRINTSPAWNENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerBlueprintSpawnEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERBLUEPRINTSPAWNENTITY_TYPE_INFO
    }
}


pub const SERVERBLUEPRINTSPAWNENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBlueprintSpawnEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ServerBlueprintSpawnEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerBlueprintProxyEntity {
}

pub const SERVERBLUEPRINTPROXYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBlueprintProxyEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BLUEPRINTPROXYENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERBLUEPRINTPROXYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerBlueprintProxyEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERBLUEPRINTPROXYENTITY_TYPE_INFO
    }
}


pub const SERVERBLUEPRINTPROXYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBlueprintProxyEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ServerBlueprintProxyEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerActorEntity {
}

pub const SERVERACTORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerActorEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERPHYSICSENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERACTORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerActorEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERACTORENTITY_TYPE_INFO
    }
}


pub const SERVERACTORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerActorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ServerActorEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DiceJobSchedulerSettings {
    pub config_file_name: String,
}

pub const DICEJOBSCHEDULERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceJobSchedulerSettings",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ConfigFileName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(DiceJobSchedulerSettings, config_file_name),
            },
        ],
    }),
    array_type: Some(DICEJOBSCHEDULERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DiceJobSchedulerSettings {
    fn type_info() -> &'static TypeInfo {
        DICEJOBSCHEDULERSETTINGS_TYPE_INFO
    }
}


pub const DICEJOBSCHEDULERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceJobSchedulerSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("DiceJobSchedulerSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LogitechLEDSettings {
    pub enable: bool,
    pub effect_start_wait_time: f32,
}

pub const LOGITECHLEDSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDSettings",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LogitechLEDSettings, enable),
            },
            FieldInfoData {
                name: "EffectStartWaitTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LogitechLEDSettings, effect_start_wait_time),
            },
        ],
    }),
    array_type: Some(LOGITECHLEDSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LogitechLEDSettings {
    fn type_info() -> &'static TypeInfo {
        LOGITECHLEDSETTINGS_TYPE_INFO
    }
}


pub const LOGITECHLEDSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("LogitechLEDSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AnselSettings {
    pub enable: bool,
    pub config_right: super::core::Vec3,
    pub config_up: super::core::Vec3,
    pub config_forward: super::core::Vec3,
    pub config_translational_speed_in_world_units_per_second: f32,
    pub config_rotational_speed_in_degrees_per_second: f32,
    pub config_meters_in_world_unit: f32,
    pub config_capture_latency: u32,
    pub config_capture_settle_latency: u32,
    pub config_title_name_utf8: String,
    pub session_config_maximum_fov_in_degrees: f32,
    pub camera_radius: f32,
    pub camera_max_wander_distance: f32,
    pub camera_offset_f_p_s: super::core::Vec3,
    pub camera_priority: i32,
}

pub const ANSELSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnselSettings",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AnselSettings, enable),
            },
            FieldInfoData {
                name: "ConfigRight",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AnselSettings, config_right),
            },
            FieldInfoData {
                name: "ConfigUp",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AnselSettings, config_up),
            },
            FieldInfoData {
                name: "ConfigForward",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AnselSettings, config_forward),
            },
            FieldInfoData {
                name: "ConfigTranslationalSpeedInWorldUnitsPerSecond",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AnselSettings, config_translational_speed_in_world_units_per_second),
            },
            FieldInfoData {
                name: "ConfigRotationalSpeedInDegreesPerSecond",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AnselSettings, config_rotational_speed_in_degrees_per_second),
            },
            FieldInfoData {
                name: "ConfigMetersInWorldUnit",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AnselSettings, config_meters_in_world_unit),
            },
            FieldInfoData {
                name: "ConfigCaptureLatency",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(AnselSettings, config_capture_latency),
            },
            FieldInfoData {
                name: "ConfigCaptureSettleLatency",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(AnselSettings, config_capture_settle_latency),
            },
            FieldInfoData {
                name: "ConfigTitleNameUtf8",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(AnselSettings, config_title_name_utf8),
            },
            FieldInfoData {
                name: "SessionConfigMaximumFovInDegrees",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AnselSettings, session_config_maximum_fov_in_degrees),
            },
            FieldInfoData {
                name: "CameraRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AnselSettings, camera_radius),
            },
            FieldInfoData {
                name: "CameraMaxWanderDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AnselSettings, camera_max_wander_distance),
            },
            FieldInfoData {
                name: "CameraOffsetFPS",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AnselSettings, camera_offset_f_p_s),
            },
            FieldInfoData {
                name: "CameraPriority",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AnselSettings, camera_priority),
            },
        ],
    }),
    array_type: Some(ANSELSETTINGS_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AnselSettings {
    fn type_info() -> &'static TypeInfo {
        ANSELSETTINGS_TYPE_INFO
    }
}


pub const ANSELSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnselSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("AnselSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WidgetReferenceEntity {
}

pub const WIDGETREFERENCEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WidgetReferenceEntity",
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WIDGETREFERENCEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for WidgetReferenceEntity {
    fn type_info() -> &'static TypeInfo {
        WIDGETREFERENCEENTITY_TYPE_INFO
    }
}


pub const WIDGETREFERENCEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WidgetReferenceEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("WidgetReferenceEntity-Array"),
    array_type: None,
    alignment: 8,
};


