use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct SelectableActionEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait SelectableActionEntityTrait: super::entity::EntityTrait {
}

impl SelectableActionEntityTrait for SelectableActionEntity {
}

impl super::entity::EntityTrait for SelectableActionEntity {
}

impl super::entity::EntityBusPeerTrait for SelectableActionEntity {
}

pub static SELECTABLEACTIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectableActionEntity",
    name_hash: 2529648738,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(SelectableActionEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SelectableActionEntity as Default>::default())),
            create_boxed: || Box::new(<SelectableActionEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SELECTABLEACTIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SelectableActionEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SELECTABLEACTIONENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SELECTABLEACTIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectableActionEntity-Array",
    name_hash: 1837420374,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("SelectableActionEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct RumbleEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait RumbleEntityTrait: super::entity::EntityTrait {
}

impl RumbleEntityTrait for RumbleEntity {
}

impl super::entity::EntityTrait for RumbleEntity {
}

impl super::entity::EntityBusPeerTrait for RumbleEntity {
}

pub static RUMBLEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RumbleEntity",
    name_hash: 2313185055,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(RumbleEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RumbleEntity as Default>::default())),
            create_boxed: || Box::new(<RumbleEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(RUMBLEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RumbleEntity {
    fn type_info(&self) -> &'static TypeInfo {
        RUMBLEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static RUMBLEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RumbleEntity-Array",
    name_hash: 239430571,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("RumbleEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct RandomActionSelectorEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait RandomActionSelectorEntityTrait: super::entity::EntityTrait {
}

impl RandomActionSelectorEntityTrait for RandomActionSelectorEntity {
}

impl super::entity::EntityTrait for RandomActionSelectorEntity {
}

impl super::entity::EntityBusPeerTrait for RandomActionSelectorEntity {
}

pub static RANDOMACTIONSELECTORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomActionSelectorEntity",
    name_hash: 3276855470,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(RandomActionSelectorEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RandomActionSelectorEntity as Default>::default())),
            create_boxed: || Box::new(<RandomActionSelectorEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(RANDOMACTIONSELECTORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RandomActionSelectorEntity {
    fn type_info(&self) -> &'static TypeInfo {
        RANDOMACTIONSELECTORENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static RANDOMACTIONSELECTORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomActionSelectorEntity-Array",
    name_hash: 350667034,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("RandomActionSelectorEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PropertyStatusEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait PropertyStatusEntityTrait: super::entity::EntityTrait {
}

impl PropertyStatusEntityTrait for PropertyStatusEntity {
}

impl super::entity::EntityTrait for PropertyStatusEntity {
}

impl super::entity::EntityBusPeerTrait for PropertyStatusEntity {
}

pub static PROPERTYSTATUSENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyStatusEntity",
    name_hash: 3990186349,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(PropertyStatusEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PropertyStatusEntity as Default>::default())),
            create_boxed: || Box::new(<PropertyStatusEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(PROPERTYSTATUSENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PropertyStatusEntity {
    fn type_info(&self) -> &'static TypeInfo {
        PROPERTYSTATUSENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static PROPERTYSTATUSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyStatusEntity-Array",
    name_hash: 3560173145,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("PropertyStatusEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PropertySelectEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait PropertySelectEntityTrait: super::entity::EntityTrait {
}

impl PropertySelectEntityTrait for PropertySelectEntity {
}

impl super::entity::EntityTrait for PropertySelectEntity {
}

impl super::entity::EntityBusPeerTrait for PropertySelectEntity {
}

pub static PROPERTYSELECTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertySelectEntity",
    name_hash: 1045063313,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(PropertySelectEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PropertySelectEntity as Default>::default())),
            create_boxed: || Box::new(<PropertySelectEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(PROPERTYSELECTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PropertySelectEntity {
    fn type_info(&self) -> &'static TypeInfo {
        PROPERTYSELECTENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static PROPERTYSELECTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertySelectEntity-Array",
    name_hash: 1863944229,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("PropertySelectEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PrioritizedBoolEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait PrioritizedBoolEntityTrait: super::entity::EntityTrait {
}

impl PrioritizedBoolEntityTrait for PrioritizedBoolEntity {
}

impl super::entity::EntityTrait for PrioritizedBoolEntity {
}

impl super::entity::EntityBusPeerTrait for PrioritizedBoolEntity {
}

pub static PRIORITIZEDBOOLENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PrioritizedBoolEntity",
    name_hash: 3050450985,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(PrioritizedBoolEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PrioritizedBoolEntity as Default>::default())),
            create_boxed: || Box::new(<PrioritizedBoolEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRIORITIZEDBOOLENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PrioritizedBoolEntity {
    fn type_info(&self) -> &'static TypeInfo {
        PRIORITIZEDBOOLENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static PRIORITIZEDBOOLENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PrioritizedBoolEntity-Array",
    name_hash: 3310016157,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("PrioritizedBoolEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct NestedConditionalPropertyEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait NestedConditionalPropertyEntityTrait: super::entity::EntityTrait {
}

impl NestedConditionalPropertyEntityTrait for NestedConditionalPropertyEntity {
}

impl super::entity::EntityTrait for NestedConditionalPropertyEntity {
}

impl super::entity::EntityBusPeerTrait for NestedConditionalPropertyEntity {
}

pub static NESTEDCONDITIONALPROPERTYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NestedConditionalPropertyEntity",
    name_hash: 1681382762,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(NestedConditionalPropertyEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NestedConditionalPropertyEntity as Default>::default())),
            create_boxed: || Box::new(<NestedConditionalPropertyEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(NESTEDCONDITIONALPROPERTYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for NestedConditionalPropertyEntity {
    fn type_info(&self) -> &'static TypeInfo {
        NESTEDCONDITIONALPROPERTYENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static NESTEDCONDITIONALPROPERTYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NestedConditionalPropertyEntity-Array",
    name_hash: 18770014,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("NestedConditionalPropertyEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MultiPropertyGateEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait MultiPropertyGateEntityTrait: super::entity::EntityTrait {
}

impl MultiPropertyGateEntityTrait for MultiPropertyGateEntity {
}

impl super::entity::EntityTrait for MultiPropertyGateEntity {
}

impl super::entity::EntityBusPeerTrait for MultiPropertyGateEntity {
}

pub static MULTIPROPERTYGATEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiPropertyGateEntity",
    name_hash: 881985127,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(MultiPropertyGateEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MultiPropertyGateEntity as Default>::default())),
            create_boxed: || Box::new(<MultiPropertyGateEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(MULTIPROPERTYGATEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MultiPropertyGateEntity {
    fn type_info(&self) -> &'static TypeInfo {
        MULTIPROPERTYGATEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static MULTIPROPERTYGATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiPropertyGateEntity-Array",
    name_hash: 106899539,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("MultiPropertyGateEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LogitechLEDPulseEffectEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait LogitechLEDPulseEffectEntityTrait: super::entity::EntityTrait {
}

impl LogitechLEDPulseEffectEntityTrait for LogitechLEDPulseEffectEntity {
}

impl super::entity::EntityTrait for LogitechLEDPulseEffectEntity {
}

impl super::entity::EntityBusPeerTrait for LogitechLEDPulseEffectEntity {
}

pub static LOGITECHLEDPULSEEFFECTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDPulseEffectEntity",
    name_hash: 2148190092,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(LogitechLEDPulseEffectEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LogitechLEDPulseEffectEntity as Default>::default())),
            create_boxed: || Box::new(<LogitechLEDPulseEffectEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(LOGITECHLEDPULSEEFFECTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LogitechLEDPulseEffectEntity {
    fn type_info(&self) -> &'static TypeInfo {
        LOGITECHLEDPULSEEFFECTENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static LOGITECHLEDPULSEEFFECTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDPulseEffectEntity-Array",
    name_hash: 2856327096,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("LogitechLEDPulseEffectEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LogitechLEDInputConceptIdentifierEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait LogitechLEDInputConceptIdentifierEntityTrait: super::entity::EntityTrait {
}

impl LogitechLEDInputConceptIdentifierEntityTrait for LogitechLEDInputConceptIdentifierEntity {
}

impl super::entity::EntityTrait for LogitechLEDInputConceptIdentifierEntity {
}

impl super::entity::EntityBusPeerTrait for LogitechLEDInputConceptIdentifierEntity {
}

pub static LOGITECHLEDINPUTCONCEPTIDENTIFIERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDInputConceptIdentifierEntity",
    name_hash: 601183857,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(LogitechLEDInputConceptIdentifierEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LogitechLEDInputConceptIdentifierEntity as Default>::default())),
            create_boxed: || Box::new(<LogitechLEDInputConceptIdentifierEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(LOGITECHLEDINPUTCONCEPTIDENTIFIERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LogitechLEDInputConceptIdentifierEntity {
    fn type_info(&self) -> &'static TypeInfo {
        LOGITECHLEDINPUTCONCEPTIDENTIFIERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static LOGITECHLEDINPUTCONCEPTIDENTIFIERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDInputConceptIdentifierEntity-Array",
    name_hash: 1669683525,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("LogitechLEDInputConceptIdentifierEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LogitechLEDFadeEffectEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait LogitechLEDFadeEffectEntityTrait: super::entity::EntityTrait {
}

impl LogitechLEDFadeEffectEntityTrait for LogitechLEDFadeEffectEntity {
}

impl super::entity::EntityTrait for LogitechLEDFadeEffectEntity {
}

impl super::entity::EntityBusPeerTrait for LogitechLEDFadeEffectEntity {
}

pub static LOGITECHLEDFADEEFFECTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDFadeEffectEntity",
    name_hash: 4140811029,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(LogitechLEDFadeEffectEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LogitechLEDFadeEffectEntity as Default>::default())),
            create_boxed: || Box::new(<LogitechLEDFadeEffectEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(LOGITECHLEDFADEEFFECTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LogitechLEDFadeEffectEntity {
    fn type_info(&self) -> &'static TypeInfo {
        LOGITECHLEDFADEEFFECTENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static LOGITECHLEDFADEEFFECTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDFadeEffectEntity-Array",
    name_hash: 1133638305,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("LogitechLEDFadeEffectEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LogitechLEDConstantEffectEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait LogitechLEDConstantEffectEntityTrait: super::entity::EntityTrait {
}

impl LogitechLEDConstantEffectEntityTrait for LogitechLEDConstantEffectEntity {
}

impl super::entity::EntityTrait for LogitechLEDConstantEffectEntity {
}

impl super::entity::EntityBusPeerTrait for LogitechLEDConstantEffectEntity {
}

pub static LOGITECHLEDCONSTANTEFFECTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDConstantEffectEntity",
    name_hash: 2378077869,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(LogitechLEDConstantEffectEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LogitechLEDConstantEffectEntity as Default>::default())),
            create_boxed: || Box::new(<LogitechLEDConstantEffectEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(LOGITECHLEDCONSTANTEFFECTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LogitechLEDConstantEffectEntity {
    fn type_info(&self) -> &'static TypeInfo {
        LOGITECHLEDCONSTANTEFFECTENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static LOGITECHLEDCONSTANTEFFECTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDConstantEffectEntity-Array",
    name_hash: 818531097,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("LogitechLEDConstantEffectEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LogitechLEDConditionalInputConceptIdentifierEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait LogitechLEDConditionalInputConceptIdentifierEntityTrait: super::entity::EntityTrait {
}

impl LogitechLEDConditionalInputConceptIdentifierEntityTrait for LogitechLEDConditionalInputConceptIdentifierEntity {
}

impl super::entity::EntityTrait for LogitechLEDConditionalInputConceptIdentifierEntity {
}

impl super::entity::EntityBusPeerTrait for LogitechLEDConditionalInputConceptIdentifierEntity {
}

pub static LOGITECHLEDCONDITIONALINPUTCONCEPTIDENTIFIERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDConditionalInputConceptIdentifierEntity",
    name_hash: 3069367023,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(LogitechLEDConditionalInputConceptIdentifierEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LogitechLEDConditionalInputConceptIdentifierEntity as Default>::default())),
            create_boxed: || Box::new(<LogitechLEDConditionalInputConceptIdentifierEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(LOGITECHLEDCONDITIONALINPUTCONCEPTIDENTIFIERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LogitechLEDConditionalInputConceptIdentifierEntity {
    fn type_info(&self) -> &'static TypeInfo {
        LOGITECHLEDCONDITIONALINPUTCONCEPTIDENTIFIERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static LOGITECHLEDCONDITIONALINPUTCONCEPTIDENTIFIERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDConditionalInputConceptIdentifierEntity-Array",
    name_hash: 4173770971,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("LogitechLEDConditionalInputConceptIdentifierEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LogitechLEDBarEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait LogitechLEDBarEntityTrait: super::entity::EntityTrait {
}

impl LogitechLEDBarEntityTrait for LogitechLEDBarEntity {
}

impl super::entity::EntityTrait for LogitechLEDBarEntity {
}

impl super::entity::EntityBusPeerTrait for LogitechLEDBarEntity {
}

pub static LOGITECHLEDBARENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDBarEntity",
    name_hash: 1466086997,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(LogitechLEDBarEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LogitechLEDBarEntity as Default>::default())),
            create_boxed: || Box::new(<LogitechLEDBarEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(LOGITECHLEDBARENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LogitechLEDBarEntity {
    fn type_info(&self) -> &'static TypeInfo {
        LOGITECHLEDBARENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static LOGITECHLEDBARENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDBarEntity-Array",
    name_hash: 119166561,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("LogitechLEDBarEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LocalLocatorEntity {
    pub _glacier_base: super::entity::SpatialEntity,
}

pub trait LocalLocatorEntityTrait: super::entity::SpatialEntityTrait {
}

impl LocalLocatorEntityTrait for LocalLocatorEntity {
}

impl super::entity::SpatialEntityTrait for LocalLocatorEntity {
}

impl super::entity::EntityTrait for LocalLocatorEntity {
}

impl super::entity::EntityBusPeerTrait for LocalLocatorEntity {
}

pub static LOCALLOCATORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalLocatorEntity",
    name_hash: 3813372699,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITY_TYPE_INFO),
        super_class_offset: offset_of!(LocalLocatorEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LocalLocatorEntity as Default>::default())),
            create_boxed: || Box::new(<LocalLocatorEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(LOCALLOCATORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LocalLocatorEntity {
    fn type_info(&self) -> &'static TypeInfo {
        LOCALLOCATORENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static LOCALLOCATORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalLocatorEntity-Array",
    name_hash: 629787055,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("LocalLocatorEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CoolDownGateEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait CoolDownGateEntityTrait: super::entity::EntityTrait {
}

impl CoolDownGateEntityTrait for CoolDownGateEntity {
}

impl super::entity::EntityTrait for CoolDownGateEntity {
}

impl super::entity::EntityBusPeerTrait for CoolDownGateEntity {
}

pub static COOLDOWNGATEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoolDownGateEntity",
    name_hash: 4112849972,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(CoolDownGateEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoolDownGateEntity as Default>::default())),
            create_boxed: || Box::new(<CoolDownGateEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(COOLDOWNGATEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CoolDownGateEntity {
    fn type_info(&self) -> &'static TypeInfo {
        COOLDOWNGATEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static COOLDOWNGATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoolDownGateEntity-Array",
    name_hash: 1134204800,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("CoolDownGateEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ConditionalPropertyEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ConditionalPropertyEntityTrait: super::entity::EntityTrait {
}

impl ConditionalPropertyEntityTrait for ConditionalPropertyEntity {
}

impl super::entity::EntityTrait for ConditionalPropertyEntity {
}

impl super::entity::EntityBusPeerTrait for ConditionalPropertyEntity {
}

pub static CONDITIONALPROPERTYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalPropertyEntity",
    name_hash: 822939047,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ConditionalPropertyEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ConditionalPropertyEntity as Default>::default())),
            create_boxed: || Box::new(<ConditionalPropertyEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CONDITIONALPROPERTYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ConditionalPropertyEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CONDITIONALPROPERTYENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CONDITIONALPROPERTYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalPropertyEntity-Array",
    name_hash: 1083088659,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ConditionalPropertyEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientValueMatchEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientValueMatchEntityTrait: super::entity::EntityTrait {
}

impl ClientValueMatchEntityTrait for ClientValueMatchEntity {
}

impl super::entity::EntityTrait for ClientValueMatchEntity {
}

impl super::entity::EntityBusPeerTrait for ClientValueMatchEntity {
}

pub static CLIENTVALUEMATCHENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientValueMatchEntity",
    name_hash: 951701279,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientValueMatchEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientValueMatchEntity as Default>::default())),
            create_boxed: || Box::new(<ClientValueMatchEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVALUEMATCHENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientValueMatchEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTVALUEMATCHENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTVALUEMATCHENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientValueMatchEntity-Array",
    name_hash: 1686950827,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientValueMatchEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientRaycastDirectionEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientRaycastDirectionEntityTrait: super::entity::EntityTrait {
}

impl ClientRaycastDirectionEntityTrait for ClientRaycastDirectionEntity {
}

impl super::entity::EntityTrait for ClientRaycastDirectionEntity {
}

impl super::entity::EntityBusPeerTrait for ClientRaycastDirectionEntity {
}

pub static CLIENTRAYCASTDIRECTIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientRaycastDirectionEntity",
    name_hash: 1618005325,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientRaycastDirectionEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientRaycastDirectionEntity as Default>::default())),
            create_boxed: || Box::new(<ClientRaycastDirectionEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTRAYCASTDIRECTIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientRaycastDirectionEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTRAYCASTDIRECTIONENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTRAYCASTDIRECTIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientRaycastDirectionEntity-Array",
    name_hash: 285884793,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientRaycastDirectionEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientDiceDebugUIInputFlowSimulatonEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientDiceDebugUIInputFlowSimulatonEntityTrait: super::entity::EntityTrait {
}

impl ClientDiceDebugUIInputFlowSimulatonEntityTrait for ClientDiceDebugUIInputFlowSimulatonEntity {
}

impl super::entity::EntityTrait for ClientDiceDebugUIInputFlowSimulatonEntity {
}

impl super::entity::EntityBusPeerTrait for ClientDiceDebugUIInputFlowSimulatonEntity {
}

pub static CLIENTDICEDEBUGUIINPUTFLOWSIMULATONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDiceDebugUIInputFlowSimulatonEntity",
    name_hash: 3660047583,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientDiceDebugUIInputFlowSimulatonEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientDiceDebugUIInputFlowSimulatonEntity as Default>::default())),
            create_boxed: || Box::new(<ClientDiceDebugUIInputFlowSimulatonEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTDICEDEBUGUIINPUTFLOWSIMULATONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientDiceDebugUIInputFlowSimulatonEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTDICEDEBUGUIINPUTFLOWSIMULATONENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTDICEDEBUGUIINPUTFLOWSIMULATONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDiceDebugUIInputFlowSimulatonEntity-Array",
    name_hash: 433322475,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientDiceDebugUIInputFlowSimulatonEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerCharacterDefinitionSpawnEntity {
    pub _glacier_base: super::game_server::ServerCharacterSpawnEntity,
}

pub trait ServerCharacterDefinitionSpawnEntityTrait: super::game_server::ServerCharacterSpawnEntityTrait {
}

impl ServerCharacterDefinitionSpawnEntityTrait for ServerCharacterDefinitionSpawnEntity {
}

impl super::game_server::ServerCharacterSpawnEntityTrait for ServerCharacterDefinitionSpawnEntity {
}

impl super::game_server::ServerSpawnEntityTrait for ServerCharacterDefinitionSpawnEntity {
}

impl super::entity::SpatialEntityTrait for ServerCharacterDefinitionSpawnEntity {
}

impl super::entity::EntityTrait for ServerCharacterDefinitionSpawnEntity {
}

impl super::entity::EntityBusPeerTrait for ServerCharacterDefinitionSpawnEntity {
}

pub static SERVERCHARACTERDEFINITIONSPAWNENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterDefinitionSpawnEntity",
    name_hash: 1047772236,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_server::SERVERCHARACTERSPAWNENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerCharacterDefinitionSpawnEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCharacterDefinitionSpawnEntity as Default>::default())),
            create_boxed: || Box::new(<ServerCharacterDefinitionSpawnEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCHARACTERDEFINITIONSPAWNENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCharacterDefinitionSpawnEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCHARACTERDEFINITIONSPAWNENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERCHARACTERDEFINITIONSPAWNENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterDefinitionSpawnEntity-Array",
    name_hash: 3707870456,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ServerCharacterDefinitionSpawnEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientCharacterDefinitionSpawnEntity {
    pub _glacier_base: super::game_client::ClientCharacterSpawnEntity,
}

pub trait ClientCharacterDefinitionSpawnEntityTrait: super::game_client::ClientCharacterSpawnEntityTrait {
}

impl ClientCharacterDefinitionSpawnEntityTrait for ClientCharacterDefinitionSpawnEntity {
}

impl super::game_client::ClientCharacterSpawnEntityTrait for ClientCharacterDefinitionSpawnEntity {
}

impl super::gameplay_client_server::ClientSpawnEntityTrait for ClientCharacterDefinitionSpawnEntity {
}

impl super::entity::SpatialEntityTrait for ClientCharacterDefinitionSpawnEntity {
}

impl super::entity::EntityTrait for ClientCharacterDefinitionSpawnEntity {
}

impl super::entity::EntityBusPeerTrait for ClientCharacterDefinitionSpawnEntity {
}

pub static CLIENTCHARACTERDEFINITIONSPAWNENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterDefinitionSpawnEntity",
    name_hash: 542359824,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_client::CLIENTCHARACTERSPAWNENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientCharacterDefinitionSpawnEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCharacterDefinitionSpawnEntity as Default>::default())),
            create_boxed: || Box::new(<ClientCharacterDefinitionSpawnEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCHARACTERDEFINITIONSPAWNENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCharacterDefinitionSpawnEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCHARACTERDEFINITIONSPAWNENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTCHARACTERDEFINITIONSPAWNENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterDefinitionSpawnEntity-Array",
    name_hash: 2928799524,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientCharacterDefinitionSpawnEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientCharacterDefinitionComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientCharacterDefinitionComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientCharacterDefinitionComponentTrait for ClientCharacterDefinitionComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientCharacterDefinitionComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientCharacterDefinitionComponent {
}

impl super::entity::ComponentTrait for ClientCharacterDefinitionComponent {
}

impl super::entity::EntityBusPeerTrait for ClientCharacterDefinitionComponent {
}

pub static CLIENTCHARACTERDEFINITIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterDefinitionComponent",
    name_hash: 1000515039,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientCharacterDefinitionComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCharacterDefinitionComponent as Default>::default())),
            create_boxed: || Box::new(<ClientCharacterDefinitionComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCHARACTERDEFINITIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCharacterDefinitionComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCHARACTERDEFINITIONCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTCHARACTERDEFINITIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterDefinitionComponent-Array",
    name_hash: 3600513771,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientCharacterDefinitionComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WhooshbyPlayerEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait WhooshbyPlayerEntityTrait: super::entity::EntityTrait {
}

impl WhooshbyPlayerEntityTrait for WhooshbyPlayerEntity {
}

impl super::entity::EntityTrait for WhooshbyPlayerEntity {
}

impl super::entity::EntityBusPeerTrait for WhooshbyPlayerEntity {
}

pub static WHOOSHBYPLAYERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WhooshbyPlayerEntity",
    name_hash: 2039736818,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(WhooshbyPlayerEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WhooshbyPlayerEntity as Default>::default())),
            create_boxed: || Box::new(<WhooshbyPlayerEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(WHOOSHBYPLAYERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for WhooshbyPlayerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        WHOOSHBYPLAYERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static WHOOSHBYPLAYERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WhooshbyPlayerEntity-Array",
    name_hash: 1887781062,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("WhooshbyPlayerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AudioProximityReverbEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait AudioProximityReverbEntityTrait: super::entity::EntityTrait {
}

impl AudioProximityReverbEntityTrait for AudioProximityReverbEntity {
}

impl super::entity::EntityTrait for AudioProximityReverbEntity {
}

impl super::entity::EntityBusPeerTrait for AudioProximityReverbEntity {
}

pub static AUDIOPROXIMITYREVERBENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioProximityReverbEntity",
    name_hash: 866717705,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(AudioProximityReverbEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AudioProximityReverbEntity as Default>::default())),
            create_boxed: || Box::new(<AudioProximityReverbEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(AUDIOPROXIMITYREVERBENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AudioProximityReverbEntity {
    fn type_info(&self) -> &'static TypeInfo {
        AUDIOPROXIMITYREVERBENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static AUDIOPROXIMITYREVERBENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioProximityReverbEntity-Array",
    name_hash: 2501764541,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("AudioProximityReverbEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AudioProximityManager {
}

pub trait AudioProximityManagerTrait: TypeObject {
}

impl AudioProximityManagerTrait for AudioProximityManager {
}

pub static AUDIOPROXIMITYMANAGER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioProximityManager",
    name_hash: 2728998485,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AudioProximityManager as Default>::default())),
            create_boxed: || Box::new(<AudioProximityManager as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(AUDIOPROXIMITYMANAGER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AudioProximityManager {
    fn type_info(&self) -> &'static TypeInfo {
        AUDIOPROXIMITYMANAGER_TYPE_INFO
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


pub static AUDIOPROXIMITYMANAGER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioProximityManager-Array",
    name_hash: 1296094817,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("AudioProximityManager"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AudioProximityDetectorReaderEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait AudioProximityDetectorReaderEntityTrait: super::entity::EntityTrait {
}

impl AudioProximityDetectorReaderEntityTrait for AudioProximityDetectorReaderEntity {
}

impl super::entity::EntityTrait for AudioProximityDetectorReaderEntity {
}

impl super::entity::EntityBusPeerTrait for AudioProximityDetectorReaderEntity {
}

pub static AUDIOPROXIMITYDETECTORREADERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioProximityDetectorReaderEntity",
    name_hash: 4192527458,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(AudioProximityDetectorReaderEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AudioProximityDetectorReaderEntity as Default>::default())),
            create_boxed: || Box::new(<AudioProximityDetectorReaderEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(AUDIOPROXIMITYDETECTORREADERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AudioProximityDetectorReaderEntity {
    fn type_info(&self) -> &'static TypeInfo {
        AUDIOPROXIMITYDETECTORREADERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static AUDIOPROXIMITYDETECTORREADERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioProximityDetectorReaderEntity-Array",
    name_hash: 4189410134,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("AudioProximityDetectorReaderEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AudioProximityDetectorEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait AudioProximityDetectorEntityTrait: super::entity::EntityTrait {
}

impl AudioProximityDetectorEntityTrait for AudioProximityDetectorEntity {
}

impl super::entity::EntityTrait for AudioProximityDetectorEntity {
}

impl super::entity::EntityBusPeerTrait for AudioProximityDetectorEntity {
}

pub static AUDIOPROXIMITYDETECTORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioProximityDetectorEntity",
    name_hash: 2725200071,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(AudioProximityDetectorEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AudioProximityDetectorEntity as Default>::default())),
            create_boxed: || Box::new(<AudioProximityDetectorEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(AUDIOPROXIMITYDETECTORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AudioProximityDetectorEntity {
    fn type_info(&self) -> &'static TypeInfo {
        AUDIOPROXIMITYDETECTORENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static AUDIOPROXIMITYDETECTORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioProximityDetectorEntity-Array",
    name_hash: 2535338483,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("AudioProximityDetectorEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct VehicleSoundEntity {
    pub _glacier_base: DiceSoundEntity,
}

pub trait VehicleSoundEntityTrait: DiceSoundEntityTrait {
}

impl VehicleSoundEntityTrait for VehicleSoundEntity {
}

impl DiceSoundEntityTrait for VehicleSoundEntity {
}

impl super::entity::EntityTrait for VehicleSoundEntity {
}

impl super::entity::EntityBusPeerTrait for VehicleSoundEntity {
}

pub static VEHICLESOUNDENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleSoundEntity",
    name_hash: 2660178405,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DICESOUNDENTITY_TYPE_INFO),
        super_class_offset: offset_of!(VehicleSoundEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VehicleSoundEntity as Default>::default())),
            create_boxed: || Box::new(<VehicleSoundEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(VEHICLESOUNDENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for VehicleSoundEntity {
    fn type_info(&self) -> &'static TypeInfo {
        VEHICLESOUNDENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static VEHICLESOUNDENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleSoundEntity-Array",
    name_hash: 2953880273,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("VehicleSoundEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SoundProviderEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait SoundProviderEntityTrait: super::entity::EntityTrait {
}

impl SoundProviderEntityTrait for SoundProviderEntity {
}

impl super::entity::EntityTrait for SoundProviderEntity {
}

impl super::entity::EntityBusPeerTrait for SoundProviderEntity {
}

pub static SOUNDPROVIDERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoundProviderEntity",
    name_hash: 2980195612,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(SoundProviderEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SoundProviderEntity as Default>::default())),
            create_boxed: || Box::new(<SoundProviderEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SOUNDPROVIDERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SoundProviderEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SOUNDPROVIDERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SOUNDPROVIDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoundProviderEntity-Array",
    name_hash: 2009097512,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("SoundProviderEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DiceSoundSpatialEntity {
    pub _glacier_base: super::entity::SpatialEntity,
}

pub trait DiceSoundSpatialEntityTrait: super::entity::SpatialEntityTrait {
}

impl DiceSoundSpatialEntityTrait for DiceSoundSpatialEntity {
}

impl super::entity::SpatialEntityTrait for DiceSoundSpatialEntity {
}

impl super::entity::EntityTrait for DiceSoundSpatialEntity {
}

impl super::entity::EntityBusPeerTrait for DiceSoundSpatialEntity {
}

pub static DICESOUNDSPATIALENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceSoundSpatialEntity",
    name_hash: 771398436,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITY_TYPE_INFO),
        super_class_offset: offset_of!(DiceSoundSpatialEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DiceSoundSpatialEntity as Default>::default())),
            create_boxed: || Box::new(<DiceSoundSpatialEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DICESOUNDSPATIALENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DiceSoundSpatialEntity {
    fn type_info(&self) -> &'static TypeInfo {
        DICESOUNDSPATIALENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DICESOUNDSPATIALENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceSoundSpatialEntity-Array",
    name_hash: 1556123280,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("DiceSoundSpatialEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DiceSoundEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait DiceSoundEntityTrait: super::entity::EntityTrait {
}

impl DiceSoundEntityTrait for DiceSoundEntity {
}

impl super::entity::EntityTrait for DiceSoundEntity {
}

impl super::entity::EntityBusPeerTrait for DiceSoundEntity {
}

pub static DICESOUNDENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceSoundEntity",
    name_hash: 3706766710,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(DiceSoundEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DiceSoundEntity as Default>::default())),
            create_boxed: || Box::new(<DiceSoundEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DICESOUNDENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DiceSoundEntity {
    fn type_info(&self) -> &'static TypeInfo {
        DICESOUNDENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DICESOUNDENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceSoundEntity-Array",
    name_hash: 2570611266,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("DiceSoundEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DiceSoundAreaEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait DiceSoundAreaEntityTrait: super::entity::EntityTrait {
}

impl DiceSoundAreaEntityTrait for DiceSoundAreaEntity {
}

impl super::entity::EntityTrait for DiceSoundAreaEntity {
}

impl super::entity::EntityBusPeerTrait for DiceSoundAreaEntity {
}

pub static DICESOUNDAREAENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceSoundAreaEntity",
    name_hash: 3710834369,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(DiceSoundAreaEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DiceSoundAreaEntity as Default>::default())),
            create_boxed: || Box::new(<DiceSoundAreaEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DICESOUNDAREAENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DiceSoundAreaEntity {
    fn type_info(&self) -> &'static TypeInfo {
        DICESOUNDAREAENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DICESOUNDAREAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceSoundAreaEntity-Array",
    name_hash: 2311741685,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("DiceSoundAreaEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientVoiceOverSourceEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientVoiceOverSourceEntityTrait: super::entity::EntityTrait {
}

impl ClientVoiceOverSourceEntityTrait for ClientVoiceOverSourceEntity {
}

impl super::entity::EntityTrait for ClientVoiceOverSourceEntity {
}

impl super::entity::EntityBusPeerTrait for ClientVoiceOverSourceEntity {
}

pub static CLIENTVOICEOVERSOURCEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverSourceEntity",
    name_hash: 4262552322,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientVoiceOverSourceEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientVoiceOverSourceEntity as Default>::default())),
            create_boxed: || Box::new(<ClientVoiceOverSourceEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVOICEOVERSOURCEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientVoiceOverSourceEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTVOICEOVERSOURCEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTVOICEOVERSOURCEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverSourceEntity-Array",
    name_hash: 1251639862,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientVoiceOverSourceEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientVoiceOverSetLanguageEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientVoiceOverSetLanguageEntityTrait: super::entity::EntityTrait {
}

impl ClientVoiceOverSetLanguageEntityTrait for ClientVoiceOverSetLanguageEntity {
}

impl super::entity::EntityTrait for ClientVoiceOverSetLanguageEntity {
}

impl super::entity::EntityBusPeerTrait for ClientVoiceOverSetLanguageEntity {
}

pub static CLIENTVOICEOVERSETLANGUAGEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverSetLanguageEntity",
    name_hash: 3401907215,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientVoiceOverSetLanguageEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientVoiceOverSetLanguageEntity as Default>::default())),
            create_boxed: || Box::new(<ClientVoiceOverSetLanguageEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVOICEOVERSETLANGUAGEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientVoiceOverSetLanguageEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTVOICEOVERSETLANGUAGEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTVOICEOVERSETLANGUAGEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverSetLanguageEntity-Array",
    name_hash: 1072887995,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientVoiceOverSetLanguageEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientVoiceOverLocatorEntity {
    pub _glacier_base: super::entity::SpatialEntity,
}

pub trait ClientVoiceOverLocatorEntityTrait: super::entity::SpatialEntityTrait {
}

impl ClientVoiceOverLocatorEntityTrait for ClientVoiceOverLocatorEntity {
}

impl super::entity::SpatialEntityTrait for ClientVoiceOverLocatorEntity {
}

impl super::entity::EntityTrait for ClientVoiceOverLocatorEntity {
}

impl super::entity::EntityBusPeerTrait for ClientVoiceOverLocatorEntity {
}

pub static CLIENTVOICEOVERLOCATORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverLocatorEntity",
    name_hash: 3862629239,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientVoiceOverLocatorEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientVoiceOverLocatorEntity as Default>::default())),
            create_boxed: || Box::new(<ClientVoiceOverLocatorEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVOICEOVERLOCATORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientVoiceOverLocatorEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTVOICEOVERLOCATORENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTVOICEOVERLOCATORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverLocatorEntity-Array",
    name_hash: 1166718275,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientVoiceOverLocatorEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientVoiceOverIntervalEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientVoiceOverIntervalEntityTrait: super::entity::EntityTrait {
}

impl ClientVoiceOverIntervalEntityTrait for ClientVoiceOverIntervalEntity {
}

impl super::entity::EntityTrait for ClientVoiceOverIntervalEntity {
}

impl super::entity::EntityBusPeerTrait for ClientVoiceOverIntervalEntity {
}

pub static CLIENTVOICEOVERINTERVALENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverIntervalEntity",
    name_hash: 1279032928,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientVoiceOverIntervalEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientVoiceOverIntervalEntity as Default>::default())),
            create_boxed: || Box::new(<ClientVoiceOverIntervalEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVOICEOVERINTERVALENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientVoiceOverIntervalEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTVOICEOVERINTERVALENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTVOICEOVERINTERVALENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverIntervalEntity-Array",
    name_hash: 1613509716,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientVoiceOverIntervalEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientVoiceOverConversationCheckEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientVoiceOverConversationCheckEntityTrait: super::entity::EntityTrait {
}

impl ClientVoiceOverConversationCheckEntityTrait for ClientVoiceOverConversationCheckEntity {
}

impl super::entity::EntityTrait for ClientVoiceOverConversationCheckEntity {
}

impl super::entity::EntityBusPeerTrait for ClientVoiceOverConversationCheckEntity {
}

pub static CLIENTVOICEOVERCONVERSATIONCHECKENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverConversationCheckEntity",
    name_hash: 171534068,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientVoiceOverConversationCheckEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientVoiceOverConversationCheckEntity as Default>::default())),
            create_boxed: || Box::new(<ClientVoiceOverConversationCheckEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVOICEOVERCONVERSATIONCHECKENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientVoiceOverConversationCheckEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTVOICEOVERCONVERSATIONCHECKENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTVOICEOVERCONVERSATIONCHECKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverConversationCheckEntity-Array",
    name_hash: 3760714432,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientVoiceOverConversationCheckEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientVoiceOverContextAreaResultEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientVoiceOverContextAreaResultEntityTrait: super::entity::EntityTrait {
}

impl ClientVoiceOverContextAreaResultEntityTrait for ClientVoiceOverContextAreaResultEntity {
}

impl super::entity::EntityTrait for ClientVoiceOverContextAreaResultEntity {
}

impl super::entity::EntityBusPeerTrait for ClientVoiceOverContextAreaResultEntity {
}

pub static CLIENTVOICEOVERCONTEXTAREARESULTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverContextAreaResultEntity",
    name_hash: 3675994142,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientVoiceOverContextAreaResultEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientVoiceOverContextAreaResultEntity as Default>::default())),
            create_boxed: || Box::new(<ClientVoiceOverContextAreaResultEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVOICEOVERCONTEXTAREARESULTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientVoiceOverContextAreaResultEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTVOICEOVERCONTEXTAREARESULTENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTVOICEOVERCONTEXTAREARESULTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverContextAreaResultEntity-Array",
    name_hash: 1916922154,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientVoiceOverContextAreaResultEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientVoiceOverContextAreaEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientVoiceOverContextAreaEntityTrait: super::entity::EntityTrait {
}

impl ClientVoiceOverContextAreaEntityTrait for ClientVoiceOverContextAreaEntity {
}

impl super::entity::EntityTrait for ClientVoiceOverContextAreaEntity {
}

impl super::entity::EntityBusPeerTrait for ClientVoiceOverContextAreaEntity {
}

pub static CLIENTVOICEOVERCONTEXTAREAENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverContextAreaEntity",
    name_hash: 2364905815,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientVoiceOverContextAreaEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientVoiceOverContextAreaEntity as Default>::default())),
            create_boxed: || Box::new(<ClientVoiceOverContextAreaEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVOICEOVERCONTEXTAREAENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientVoiceOverContextAreaEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTVOICEOVERCONTEXTAREAENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTVOICEOVERCONTEXTAREAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverContextAreaEntity-Array",
    name_hash: 532132451,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientVoiceOverContextAreaEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientSoundAssetDataEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientSoundAssetDataEntityTrait: super::entity::EntityTrait {
}

impl ClientSoundAssetDataEntityTrait for ClientSoundAssetDataEntity {
}

impl super::entity::EntityTrait for ClientSoundAssetDataEntity {
}

impl super::entity::EntityBusPeerTrait for ClientSoundAssetDataEntity {
}

pub static CLIENTSOUNDASSETDATAENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoundAssetDataEntity",
    name_hash: 1679588100,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientSoundAssetDataEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSoundAssetDataEntity as Default>::default())),
            create_boxed: || Box::new(<ClientSoundAssetDataEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSOUNDASSETDATAENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSoundAssetDataEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSOUNDASSETDATAENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTSOUNDASSETDATAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoundAssetDataEntity-Array",
    name_hash: 3391819568,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientSoundAssetDataEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientSoundActivityTesterEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientSoundActivityTesterEntityTrait: super::entity::EntityTrait {
}

impl ClientSoundActivityTesterEntityTrait for ClientSoundActivityTesterEntity {
}

impl super::entity::EntityTrait for ClientSoundActivityTesterEntity {
}

impl super::entity::EntityBusPeerTrait for ClientSoundActivityTesterEntity {
}

pub static CLIENTSOUNDACTIVITYTESTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoundActivityTesterEntity",
    name_hash: 2491758376,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientSoundActivityTesterEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSoundActivityTesterEntity as Default>::default())),
            create_boxed: || Box::new(<ClientSoundActivityTesterEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSOUNDACTIVITYTESTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSoundActivityTesterEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSOUNDACTIVITYTESTERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTSOUNDACTIVITYTESTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoundActivityTesterEntity-Array",
    name_hash: 2702170268,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientSoundActivityTesterEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientMusicEventPriorityEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientMusicEventPriorityEntityTrait: super::entity::EntityTrait {
}

impl ClientMusicEventPriorityEntityTrait for ClientMusicEventPriorityEntity {
}

impl super::entity::EntityTrait for ClientMusicEventPriorityEntity {
}

impl super::entity::EntityBusPeerTrait for ClientMusicEventPriorityEntity {
}

pub static CLIENTMUSICEVENTPRIORITYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMusicEventPriorityEntity",
    name_hash: 965933080,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientMusicEventPriorityEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientMusicEventPriorityEntity as Default>::default())),
            create_boxed: || Box::new(<ClientMusicEventPriorityEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTMUSICEVENTPRIORITYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientMusicEventPriorityEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTMUSICEVENTPRIORITYENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTMUSICEVENTPRIORITYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMusicEventPriorityEntity-Array",
    name_hash: 2157822508,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientMusicEventPriorityEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AudioCurveFactorEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait AudioCurveFactorEntityTrait: super::entity::EntityTrait {
}

impl AudioCurveFactorEntityTrait for AudioCurveFactorEntity {
}

impl super::entity::EntityTrait for AudioCurveFactorEntity {
}

impl super::entity::EntityBusPeerTrait for AudioCurveFactorEntity {
}

pub static AUDIOCURVEFACTORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioCurveFactorEntity",
    name_hash: 3975631314,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(AudioCurveFactorEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AudioCurveFactorEntity as Default>::default())),
            create_boxed: || Box::new(<AudioCurveFactorEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(AUDIOCURVEFACTORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AudioCurveFactorEntity {
    fn type_info(&self) -> &'static TypeInfo {
        AUDIOCURVEFACTORENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static AUDIOCURVEFACTORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioCurveFactorEntity-Array",
    name_hash: 1246464358,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("AudioCurveFactorEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientDebrisClusterSoundsComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientDebrisClusterSoundsComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientDebrisClusterSoundsComponentTrait for ClientDebrisClusterSoundsComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientDebrisClusterSoundsComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientDebrisClusterSoundsComponent {
}

impl super::entity::ComponentTrait for ClientDebrisClusterSoundsComponent {
}

impl super::entity::EntityBusPeerTrait for ClientDebrisClusterSoundsComponent {
}

pub static CLIENTDEBRISCLUSTERSOUNDSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDebrisClusterSoundsComponent",
    name_hash: 3908943586,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientDebrisClusterSoundsComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientDebrisClusterSoundsComponent as Default>::default())),
            create_boxed: || Box::new(<ClientDebrisClusterSoundsComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTDEBRISCLUSTERSOUNDSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientDebrisClusterSoundsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTDEBRISCLUSTERSOUNDSCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTDEBRISCLUSTERSOUNDSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDebrisClusterSoundsComponent-Array",
    name_hash: 1873284566,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientDebrisClusterSoundsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AnselFreeCamera {
    pub _glacier_base: super::gameplay_sim::Camera,
}

pub trait AnselFreeCameraTrait: super::gameplay_sim::CameraTrait {
}

impl AnselFreeCameraTrait for AnselFreeCamera {
}

impl super::gameplay_sim::CameraTrait for AnselFreeCamera {
}

pub static ANSELFREECAMERA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnselFreeCamera",
    name_hash: 1309504509,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_sim::CAMERA_TYPE_INFO),
        super_class_offset: offset_of!(AnselFreeCamera, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AnselFreeCamera as Default>::default())),
            create_boxed: || Box::new(<AnselFreeCamera as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(ANSELFREECAMERA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AnselFreeCamera {
    fn type_info(&self) -> &'static TypeInfo {
        ANSELFREECAMERA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static ANSELFREECAMERA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnselFreeCamera-Array",
    name_hash: 2440863945,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("AnselFreeCamera"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DofReaderEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait DofReaderEntityTrait: super::entity::EntityTrait {
}

impl DofReaderEntityTrait for DofReaderEntity {
}

impl super::entity::EntityTrait for DofReaderEntity {
}

impl super::entity::EntityBusPeerTrait for DofReaderEntity {
}

pub static DOFREADERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DofReaderEntity",
    name_hash: 466404214,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(DofReaderEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DofReaderEntity as Default>::default())),
            create_boxed: || Box::new(<DofReaderEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DOFREADERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DofReaderEntity {
    fn type_info(&self) -> &'static TypeInfo {
        DOFREADERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DOFREADERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DofReaderEntity-Array",
    name_hash: 2560288834,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("DofReaderEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AnimatableCullingEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait AnimatableCullingEntityTrait: super::entity::EntityTrait {
}

impl AnimatableCullingEntityTrait for AnimatableCullingEntity {
}

impl super::entity::EntityTrait for AnimatableCullingEntity {
}

impl super::entity::EntityBusPeerTrait for AnimatableCullingEntity {
}

pub static ANIMATABLECULLINGENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimatableCullingEntity",
    name_hash: 2466926140,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(AnimatableCullingEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AnimatableCullingEntity as Default>::default())),
            create_boxed: || Box::new(<AnimatableCullingEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(ANIMATABLECULLINGENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AnimatableCullingEntity {
    fn type_info(&self) -> &'static TypeInfo {
        ANIMATABLECULLINGENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static ANIMATABLECULLINGENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimatableCullingEntity-Array",
    name_hash: 463250824,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("AnimatableCullingEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientEmitTraceBookmarkEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientEmitTraceBookmarkEntityTrait: super::entity::EntityTrait {
}

impl ClientEmitTraceBookmarkEntityTrait for ClientEmitTraceBookmarkEntity {
}

impl super::entity::EntityTrait for ClientEmitTraceBookmarkEntity {
}

impl super::entity::EntityBusPeerTrait for ClientEmitTraceBookmarkEntity {
}

pub static CLIENTEMITTRACEBOOKMARKENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEmitTraceBookmarkEntity",
    name_hash: 2916255663,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientEmitTraceBookmarkEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientEmitTraceBookmarkEntity as Default>::default())),
            create_boxed: || Box::new(<ClientEmitTraceBookmarkEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTEMITTRACEBOOKMARKENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientEmitTraceBookmarkEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTEMITTRACEBOOKMARKENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTEMITTRACEBOOKMARKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEmitTraceBookmarkEntity-Array",
    name_hash: 8844571,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientEmitTraceBookmarkEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientCharacterProxyEntity {
    pub _glacier_base: ClientBlueprintProxyEntity,
}

pub trait ClientCharacterProxyEntityTrait: ClientBlueprintProxyEntityTrait {
}

impl ClientCharacterProxyEntityTrait for ClientCharacterProxyEntity {
}

impl ClientBlueprintProxyEntityTrait for ClientCharacterProxyEntity {
}

impl super::dice_commons_shared::BlueprintProxyEntityBaseTrait for ClientCharacterProxyEntity {
}

impl super::entity::EntityTrait for ClientCharacterProxyEntity {
}

impl super::entity::EntityBusPeerTrait for ClientCharacterProxyEntity {
}

pub static CLIENTCHARACTERPROXYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterProxyEntity",
    name_hash: 4134881746,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTBLUEPRINTPROXYENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientCharacterProxyEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCharacterProxyEntity as Default>::default())),
            create_boxed: || Box::new(<ClientCharacterProxyEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCHARACTERPROXYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCharacterProxyEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCHARACTERPROXYENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTCHARACTERPROXYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterProxyEntity-Array",
    name_hash: 677609830,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientCharacterProxyEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientCameraShakeEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientCameraShakeEntityTrait: super::entity::EntityTrait {
}

impl ClientCameraShakeEntityTrait for ClientCameraShakeEntity {
}

impl super::entity::EntityTrait for ClientCameraShakeEntity {
}

impl super::entity::EntityBusPeerTrait for ClientCameraShakeEntity {
}

pub static CLIENTCAMERASHAKEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCameraShakeEntity",
    name_hash: 2821199690,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientCameraShakeEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCameraShakeEntity as Default>::default())),
            create_boxed: || Box::new(<ClientCameraShakeEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCAMERASHAKEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCameraShakeEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCAMERASHAKEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTCAMERASHAKEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCameraShakeEntity-Array",
    name_hash: 709296894,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientCameraShakeEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientBlueprintSpawnEntity {
    pub _glacier_base: BlueprintSpawnEntity,
}

pub trait ClientBlueprintSpawnEntityTrait: BlueprintSpawnEntityTrait {
}

impl ClientBlueprintSpawnEntityTrait for ClientBlueprintSpawnEntity {
}

impl BlueprintSpawnEntityTrait for ClientBlueprintSpawnEntity {
}

impl super::entity::SpatialEntityTrait for ClientBlueprintSpawnEntity {
}

impl super::entity::EntityTrait for ClientBlueprintSpawnEntity {
}

impl super::entity::EntityBusPeerTrait for ClientBlueprintSpawnEntity {
}

pub static CLIENTBLUEPRINTSPAWNENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBlueprintSpawnEntity",
    name_hash: 2507654355,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BLUEPRINTSPAWNENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientBlueprintSpawnEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientBlueprintSpawnEntity as Default>::default())),
            create_boxed: || Box::new(<ClientBlueprintSpawnEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTBLUEPRINTSPAWNENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientBlueprintSpawnEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTBLUEPRINTSPAWNENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTBLUEPRINTSPAWNENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBlueprintSpawnEntity-Array",
    name_hash: 1707921383,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientBlueprintSpawnEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientBlueprintProxyEntity {
    pub _glacier_base: super::dice_commons_shared::BlueprintProxyEntityBase,
}

pub trait ClientBlueprintProxyEntityTrait: super::dice_commons_shared::BlueprintProxyEntityBaseTrait {
}

impl ClientBlueprintProxyEntityTrait for ClientBlueprintProxyEntity {
}

impl super::dice_commons_shared::BlueprintProxyEntityBaseTrait for ClientBlueprintProxyEntity {
}

impl super::entity::EntityTrait for ClientBlueprintProxyEntity {
}

impl super::entity::EntityBusPeerTrait for ClientBlueprintProxyEntity {
}

pub static CLIENTBLUEPRINTPROXYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBlueprintProxyEntity",
    name_hash: 3936542180,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::dice_commons_shared::BLUEPRINTPROXYENTITYBASE_TYPE_INFO),
        super_class_offset: offset_of!(ClientBlueprintProxyEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientBlueprintProxyEntity as Default>::default())),
            create_boxed: || Box::new(<ClientBlueprintProxyEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTBLUEPRINTPROXYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientBlueprintProxyEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTBLUEPRINTPROXYENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTBLUEPRINTPROXYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBlueprintProxyEntity-Array",
    name_hash: 771482064,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientBlueprintProxyEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientActorEntity {
    pub _glacier_base: super::gameplay_client_server::ClientPhysicsEntity,
}

pub trait ClientActorEntityTrait: super::gameplay_client_server::ClientPhysicsEntityTrait {
}

impl ClientActorEntityTrait for ClientActorEntity {
}

impl super::gameplay_client_server::ClientPhysicsEntityTrait for ClientActorEntity {
}

impl super::gameplay_client_server::ClientGameComponentEntityTrait for ClientActorEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ClientActorEntity {
}

impl super::entity::ComponentEntityTrait for ClientActorEntity {
}

impl super::entity::SpatialEntityTrait for ClientActorEntity {
}

impl super::entity::EntityTrait for ClientActorEntity {
}

impl super::entity::EntityBusPeerTrait for ClientActorEntity {
}

pub static CLIENTACTORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientActorEntity",
    name_hash: 2905112556,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTPHYSICSENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientActorEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientActorEntity as Default>::default())),
            create_boxed: || Box::new(<ClientActorEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTACTORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientActorEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTACTORENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTACTORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientActorEntity-Array",
    name_hash: 2637324248,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientActorEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct BlueprintSpawnEntity {
    pub _glacier_base: super::entity::SpatialEntity,
}

pub trait BlueprintSpawnEntityTrait: super::entity::SpatialEntityTrait {
}

impl BlueprintSpawnEntityTrait for BlueprintSpawnEntity {
}

impl super::entity::SpatialEntityTrait for BlueprintSpawnEntity {
}

impl super::entity::EntityTrait for BlueprintSpawnEntity {
}

impl super::entity::EntityBusPeerTrait for BlueprintSpawnEntity {
}

pub static BLUEPRINTSPAWNENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintSpawnEntity",
    name_hash: 735020458,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITY_TYPE_INFO),
        super_class_offset: offset_of!(BlueprintSpawnEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BlueprintSpawnEntity as Default>::default())),
            create_boxed: || Box::new(<BlueprintSpawnEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(BLUEPRINTSPAWNENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BlueprintSpawnEntity {
    fn type_info(&self) -> &'static TypeInfo {
        BLUEPRINTSPAWNENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static BLUEPRINTSPAWNENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintSpawnEntity-Array",
    name_hash: 1572044830,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("BlueprintSpawnEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DiceUITypingInputListenerElement {
    pub _glacier_base: super::game_client_u_i::UIElementEntity,
}

pub trait DiceUITypingInputListenerElementTrait: super::game_client_u_i::UIElementEntityTrait {
}

impl DiceUITypingInputListenerElementTrait for DiceUITypingInputListenerElement {
}

impl super::game_client_u_i::UIElementEntityTrait for DiceUITypingInputListenerElement {
}

impl super::entity::EntityTrait for DiceUITypingInputListenerElement {
}

impl super::entity::EntityBusPeerTrait for DiceUITypingInputListenerElement {
}

pub static DICEUITYPINGINPUTLISTENERELEMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUITypingInputListenerElement",
    name_hash: 3571916441,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_client_u_i::UIELEMENTENTITY_TYPE_INFO),
        super_class_offset: offset_of!(DiceUITypingInputListenerElement, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DiceUITypingInputListenerElement as Default>::default())),
            create_boxed: || Box::new(<DiceUITypingInputListenerElement as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DICEUITYPINGINPUTLISTENERELEMENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DiceUITypingInputListenerElement {
    fn type_info(&self) -> &'static TypeInfo {
        DICEUITYPINGINPUTLISTENERELEMENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DICEUITYPINGINPUTLISTENERELEMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUITypingInputListenerElement-Array",
    name_hash: 2833226797,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("DiceUITypingInputListenerElement"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DiceUIMouseInputListenerElement {
    pub _glacier_base: super::game_client_u_i::UIElementEntity,
}

pub trait DiceUIMouseInputListenerElementTrait: super::game_client_u_i::UIElementEntityTrait {
}

impl DiceUIMouseInputListenerElementTrait for DiceUIMouseInputListenerElement {
}

impl super::game_client_u_i::UIElementEntityTrait for DiceUIMouseInputListenerElement {
}

impl super::entity::EntityTrait for DiceUIMouseInputListenerElement {
}

impl super::entity::EntityBusPeerTrait for DiceUIMouseInputListenerElement {
}

pub static DICEUIMOUSEINPUTLISTENERELEMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIMouseInputListenerElement",
    name_hash: 1194863589,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_client_u_i::UIELEMENTENTITY_TYPE_INFO),
        super_class_offset: offset_of!(DiceUIMouseInputListenerElement, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DiceUIMouseInputListenerElement as Default>::default())),
            create_boxed: || Box::new(<DiceUIMouseInputListenerElement as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DICEUIMOUSEINPUTLISTENERELEMENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DiceUIMouseInputListenerElement {
    fn type_info(&self) -> &'static TypeInfo {
        DICEUIMOUSEINPUTLISTENERELEMENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DICEUIMOUSEINPUTLISTENERELEMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIMouseInputListenerElement-Array",
    name_hash: 197080273,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("DiceUIMouseInputListenerElement"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DiceUIInputBlockerElement {
    pub _glacier_base: super::game_client_u_i::UIElementEntity,
}

pub trait DiceUIInputBlockerElementTrait: super::game_client_u_i::UIElementEntityTrait {
}

impl DiceUIInputBlockerElementTrait for DiceUIInputBlockerElement {
}

impl super::game_client_u_i::UIElementEntityTrait for DiceUIInputBlockerElement {
}

impl super::entity::EntityTrait for DiceUIInputBlockerElement {
}

impl super::entity::EntityBusPeerTrait for DiceUIInputBlockerElement {
}

pub static DICEUIINPUTBLOCKERELEMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIInputBlockerElement",
    name_hash: 952614116,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_client_u_i::UIELEMENTENTITY_TYPE_INFO),
        super_class_offset: offset_of!(DiceUIInputBlockerElement, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DiceUIInputBlockerElement as Default>::default())),
            create_boxed: || Box::new(<DiceUIInputBlockerElement as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DICEUIINPUTBLOCKERELEMENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DiceUIInputBlockerElement {
    fn type_info(&self) -> &'static TypeInfo {
        DICEUIINPUTBLOCKERELEMENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DICEUIINPUTBLOCKERELEMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIInputBlockerElement-Array",
    name_hash: 2181824720,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("DiceUIInputBlockerElement"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DiceUIInputActionListenerElement {
    pub _glacier_base: super::game_client_u_i::UIElementEntity,
}

pub trait DiceUIInputActionListenerElementTrait: super::game_client_u_i::UIElementEntityTrait {
}

impl DiceUIInputActionListenerElementTrait for DiceUIInputActionListenerElement {
}

impl super::game_client_u_i::UIElementEntityTrait for DiceUIInputActionListenerElement {
}

impl super::entity::EntityTrait for DiceUIInputActionListenerElement {
}

impl super::entity::EntityBusPeerTrait for DiceUIInputActionListenerElement {
}

pub static DICEUIINPUTACTIONLISTENERELEMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIInputActionListenerElement",
    name_hash: 2435106330,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_client_u_i::UIELEMENTENTITY_TYPE_INFO),
        super_class_offset: offset_of!(DiceUIInputActionListenerElement, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DiceUIInputActionListenerElement as Default>::default())),
            create_boxed: || Box::new(<DiceUIInputActionListenerElement as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DICEUIINPUTACTIONLISTENERELEMENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DiceUIInputActionListenerElement {
    fn type_info(&self) -> &'static TypeInfo {
        DICEUIINPUTACTIONLISTENERELEMENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DICEUIINPUTACTIONLISTENERELEMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIInputActionListenerElement-Array",
    name_hash: 2004764974,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("DiceUIInputActionListenerElement"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DiceUIAnalogStickInputListenerElement {
    pub _glacier_base: super::game_client_u_i::UIElementEntity,
}

pub trait DiceUIAnalogStickInputListenerElementTrait: super::game_client_u_i::UIElementEntityTrait {
}

impl DiceUIAnalogStickInputListenerElementTrait for DiceUIAnalogStickInputListenerElement {
}

impl super::game_client_u_i::UIElementEntityTrait for DiceUIAnalogStickInputListenerElement {
}

impl super::entity::EntityTrait for DiceUIAnalogStickInputListenerElement {
}

impl super::entity::EntityBusPeerTrait for DiceUIAnalogStickInputListenerElement {
}

pub static DICEUIANALOGSTICKINPUTLISTENERELEMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIAnalogStickInputListenerElement",
    name_hash: 2675985352,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_client_u_i::UIELEMENTENTITY_TYPE_INFO),
        super_class_offset: offset_of!(DiceUIAnalogStickInputListenerElement, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DiceUIAnalogStickInputListenerElement as Default>::default())),
            create_boxed: || Box::new(<DiceUIAnalogStickInputListenerElement as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DICEUIANALOGSTICKINPUTLISTENERELEMENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DiceUIAnalogStickInputListenerElement {
    fn type_info(&self) -> &'static TypeInfo {
        DICEUIANALOGSTICKINPUTLISTENERELEMENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DICEUIANALOGSTICKINPUTLISTENERELEMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIAnalogStickInputListenerElement-Array",
    name_hash: 4208729724,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("DiceUIAnalogStickInputListenerElement"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DiceUIAnalogPadInputListenerElement {
    pub _glacier_base: super::game_client_u_i::UIElementEntity,
}

pub trait DiceUIAnalogPadInputListenerElementTrait: super::game_client_u_i::UIElementEntityTrait {
}

impl DiceUIAnalogPadInputListenerElementTrait for DiceUIAnalogPadInputListenerElement {
}

impl super::game_client_u_i::UIElementEntityTrait for DiceUIAnalogPadInputListenerElement {
}

impl super::entity::EntityTrait for DiceUIAnalogPadInputListenerElement {
}

impl super::entity::EntityBusPeerTrait for DiceUIAnalogPadInputListenerElement {
}

pub static DICEUIANALOGPADINPUTLISTENERELEMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIAnalogPadInputListenerElement",
    name_hash: 236731675,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_client_u_i::UIELEMENTENTITY_TYPE_INFO),
        super_class_offset: offset_of!(DiceUIAnalogPadInputListenerElement, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DiceUIAnalogPadInputListenerElement as Default>::default())),
            create_boxed: || Box::new(<DiceUIAnalogPadInputListenerElement as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DICEUIANALOGPADINPUTLISTENERELEMENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DiceUIAnalogPadInputListenerElement {
    fn type_info(&self) -> &'static TypeInfo {
        DICEUIANALOGPADINPUTLISTENERELEMENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DICEUIANALOGPADINPUTLISTENERELEMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceUIAnalogPadInputListenerElement-Array",
    name_hash: 2689209263,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("DiceUIAnalogPadInputListenerElement"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientDiceUIInputManagerEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientDiceUIInputManagerEntityTrait: super::entity::EntityTrait {
}

impl ClientDiceUIInputManagerEntityTrait for ClientDiceUIInputManagerEntity {
}

impl super::entity::EntityTrait for ClientDiceUIInputManagerEntity {
}

impl super::entity::EntityBusPeerTrait for ClientDiceUIInputManagerEntity {
}

pub static CLIENTDICEUIINPUTMANAGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDiceUIInputManagerEntity",
    name_hash: 2908919829,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientDiceUIInputManagerEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientDiceUIInputManagerEntity as Default>::default())),
            create_boxed: || Box::new(<ClientDiceUIInputManagerEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTDICEUIINPUTMANAGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientDiceUIInputManagerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTDICEUIINPUTMANAGERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTDICEUIINPUTMANAGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDiceUIInputManagerEntity-Array",
    name_hash: 1412466593,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientDiceUIInputManagerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LocalizedStringIdPickerEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait LocalizedStringIdPickerEntityTrait: super::entity::EntityTrait {
}

impl LocalizedStringIdPickerEntityTrait for LocalizedStringIdPickerEntity {
}

impl super::entity::EntityTrait for LocalizedStringIdPickerEntity {
}

impl super::entity::EntityBusPeerTrait for LocalizedStringIdPickerEntity {
}

pub static LOCALIZEDSTRINGIDPICKERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedStringIdPickerEntity",
    name_hash: 3814712607,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(LocalizedStringIdPickerEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LocalizedStringIdPickerEntity as Default>::default())),
            create_boxed: || Box::new(<LocalizedStringIdPickerEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(LOCALIZEDSTRINGIDPICKERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LocalizedStringIdPickerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        LOCALIZEDSTRINGIDPICKERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static LOCALIZEDSTRINGIDPICKERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedStringIdPickerEntity-Array",
    name_hash: 1961585067,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("LocalizedStringIdPickerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CheckedLocalizedStringEntity {
    pub _glacier_base: super::u_i_incubator::LocalizedStringEntityBase,
}

pub trait CheckedLocalizedStringEntityTrait: super::u_i_incubator::LocalizedStringEntityBaseTrait {
}

impl CheckedLocalizedStringEntityTrait for CheckedLocalizedStringEntity {
}

impl super::u_i_incubator::LocalizedStringEntityBaseTrait for CheckedLocalizedStringEntity {
}

impl super::entity::EntityTrait for CheckedLocalizedStringEntity {
}

impl super::entity::EntityBusPeerTrait for CheckedLocalizedStringEntity {
}

pub static CHECKEDLOCALIZEDSTRINGENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CheckedLocalizedStringEntity",
    name_hash: 2015385651,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::u_i_incubator::LOCALIZEDSTRINGENTITYBASE_TYPE_INFO),
        super_class_offset: offset_of!(CheckedLocalizedStringEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CheckedLocalizedStringEntity as Default>::default())),
            create_boxed: || Box::new(<CheckedLocalizedStringEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CHECKEDLOCALIZEDSTRINGENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CheckedLocalizedStringEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CHECKEDLOCALIZEDSTRINGENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CHECKEDLOCALIZEDSTRINGENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CheckedLocalizedStringEntity-Array",
    name_hash: 3403867015,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("CheckedLocalizedStringEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerLocatorComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerLocatorComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerLocatorComponentTrait for ServerLocatorComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerLocatorComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerLocatorComponent {
}

impl super::entity::ComponentTrait for ServerLocatorComponent {
}

impl super::entity::EntityBusPeerTrait for ServerLocatorComponent {
}

pub static SERVERLOCATORCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLocatorComponent",
    name_hash: 2740574183,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerLocatorComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerLocatorComponent as Default>::default())),
            create_boxed: || Box::new(<ServerLocatorComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERLOCATORCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerLocatorComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERLOCATORCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERLOCATORCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLocatorComponent-Array",
    name_hash: 4145945043,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ServerLocatorComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerActorPhysicsComponent {
    pub _glacier_base: super::game_server::ServerStaticModelPhysicsComponent,
}

pub trait ServerActorPhysicsComponentTrait: super::game_server::ServerStaticModelPhysicsComponentTrait {
}

impl ServerActorPhysicsComponentTrait for ServerActorPhysicsComponent {
}

impl super::game_server::ServerStaticModelPhysicsComponentTrait for ServerActorPhysicsComponent {
}

impl super::physics::PartPhysicsComponentTrait for ServerActorPhysicsComponent {
}

impl super::physics::PhysicsComponentTrait for ServerActorPhysicsComponent {
}

impl super::entity::ComponentTrait for ServerActorPhysicsComponent {
}

impl super::entity::EntityBusPeerTrait for ServerActorPhysicsComponent {
}

pub static SERVERACTORPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerActorPhysicsComponent",
    name_hash: 4291484399,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_server::SERVERSTATICMODELPHYSICSCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerActorPhysicsComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerActorPhysicsComponent as Default>::default())),
            create_boxed: || Box::new(<ServerActorPhysicsComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERACTORPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerActorPhysicsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERACTORPHYSICSCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERACTORPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerActorPhysicsComponent-Array",
    name_hash: 1434761435,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ServerActorPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerActorCustomizationComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerActorCustomizationComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerActorCustomizationComponentTrait for ServerActorCustomizationComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerActorCustomizationComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerActorCustomizationComponent {
}

impl super::entity::ComponentTrait for ServerActorCustomizationComponent {
}

impl super::entity::EntityBusPeerTrait for ServerActorCustomizationComponent {
}

pub static SERVERACTORCUSTOMIZATIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerActorCustomizationComponent",
    name_hash: 402713369,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerActorCustomizationComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerActorCustomizationComponent as Default>::default())),
            create_boxed: || Box::new(<ServerActorCustomizationComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERACTORCUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerActorCustomizationComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERACTORCUSTOMIZATIONCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERACTORCUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerActorCustomizationComponent-Array",
    name_hash: 4245677,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ServerActorCustomizationComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientMaterialBasedEffectComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientMaterialBasedEffectComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientMaterialBasedEffectComponentTrait for ClientMaterialBasedEffectComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientMaterialBasedEffectComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientMaterialBasedEffectComponent {
}

impl super::entity::ComponentTrait for ClientMaterialBasedEffectComponent {
}

impl super::entity::EntityBusPeerTrait for ClientMaterialBasedEffectComponent {
}

pub static CLIENTMATERIALBASEDEFFECTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMaterialBasedEffectComponent",
    name_hash: 3453778398,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientMaterialBasedEffectComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientMaterialBasedEffectComponent as Default>::default())),
            create_boxed: || Box::new(<ClientMaterialBasedEffectComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTMATERIALBASEDEFFECTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientMaterialBasedEffectComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTMATERIALBASEDEFFECTCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTMATERIALBASEDEFFECTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMaterialBasedEffectComponent-Array",
    name_hash: 2997098,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientMaterialBasedEffectComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientLocatorComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientLocatorComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientLocatorComponentTrait for ClientLocatorComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientLocatorComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientLocatorComponent {
}

impl super::entity::ComponentTrait for ClientLocatorComponent {
}

impl super::entity::EntityBusPeerTrait for ClientLocatorComponent {
}

pub static CLIENTLOCATORCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLocatorComponent",
    name_hash: 3598499771,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientLocatorComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientLocatorComponent as Default>::default())),
            create_boxed: || Box::new(<ClientLocatorComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTLOCATORCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientLocatorComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTLOCATORCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTLOCATORCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLocatorComponent-Array",
    name_hash: 3964552975,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientLocatorComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientActorPhysicsComponent {
    pub _glacier_base: super::game_client::ClientStaticModelPhysicsComponent,
}

pub trait ClientActorPhysicsComponentTrait: super::game_client::ClientStaticModelPhysicsComponentTrait {
}

impl ClientActorPhysicsComponentTrait for ClientActorPhysicsComponent {
}

impl super::game_client::ClientStaticModelPhysicsComponentTrait for ClientActorPhysicsComponent {
}

impl super::physics::PartPhysicsComponentTrait for ClientActorPhysicsComponent {
}

impl super::physics::PhysicsComponentTrait for ClientActorPhysicsComponent {
}

impl super::entity::ComponentTrait for ClientActorPhysicsComponent {
}

impl super::entity::EntityBusPeerTrait for ClientActorPhysicsComponent {
}

pub static CLIENTACTORPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientActorPhysicsComponent",
    name_hash: 3359283635,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_client::CLIENTSTATICMODELPHYSICSCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientActorPhysicsComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientActorPhysicsComponent as Default>::default())),
            create_boxed: || Box::new(<ClientActorPhysicsComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTACTORPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientActorPhysicsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTACTORPHYSICSCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTACTORPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientActorPhysicsComponent-Array",
    name_hash: 1292387591,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientActorPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientActorCustomizationComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientActorCustomizationComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientActorCustomizationComponentTrait for ClientActorCustomizationComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientActorCustomizationComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientActorCustomizationComponent {
}

impl super::entity::ComponentTrait for ClientActorCustomizationComponent {
}

impl super::entity::EntityBusPeerTrait for ClientActorCustomizationComponent {
}

pub static CLIENTACTORCUSTOMIZATIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientActorCustomizationComponent",
    name_hash: 105729989,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientActorCustomizationComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientActorCustomizationComponent as Default>::default())),
            create_boxed: || Box::new(<ClientActorCustomizationComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTACTORCUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientActorCustomizationComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTACTORCUSTOMIZATIONCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTACTORCUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientActorCustomizationComponent-Array",
    name_hash: 4183771121,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ClientActorCustomizationComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SimpleRotationEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait SimpleRotationEntityTrait: super::entity::EntityTrait {
}

impl SimpleRotationEntityTrait for SimpleRotationEntity {
}

impl super::entity::EntityTrait for SimpleRotationEntity {
}

impl super::entity::EntityBusPeerTrait for SimpleRotationEntity {
}

pub static SIMPLEROTATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimpleRotationEntity",
    name_hash: 1237673380,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(SimpleRotationEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SimpleRotationEntity as Default>::default())),
            create_boxed: || Box::new(<SimpleRotationEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SIMPLEROTATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SimpleRotationEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SIMPLEROTATIONENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SIMPLEROTATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimpleRotationEntity-Array",
    name_hash: 2568567056,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("SimpleRotationEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ShadowplayHighlightEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ShadowplayHighlightEntityTrait: super::entity::EntityTrait {
}

impl ShadowplayHighlightEntityTrait for ShadowplayHighlightEntity {
}

impl super::entity::EntityTrait for ShadowplayHighlightEntity {
}

impl super::entity::EntityBusPeerTrait for ShadowplayHighlightEntity {
}

pub static SHADOWPLAYHIGHLIGHTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowplayHighlightEntity",
    name_hash: 4167155852,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ShadowplayHighlightEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ShadowplayHighlightEntity as Default>::default())),
            create_boxed: || Box::new(<ShadowplayHighlightEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SHADOWPLAYHIGHLIGHTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ShadowplayHighlightEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SHADOWPLAYHIGHLIGHTENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SHADOWPLAYHIGHLIGHTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowplayHighlightEntity-Array",
    name_hash: 4029534392,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ShadowplayHighlightEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerGhostEntityIdOwnerWrapperEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerGhostEntityIdOwnerWrapperEntityTrait: super::entity::EntityTrait {
}

impl ServerGhostEntityIdOwnerWrapperEntityTrait for ServerGhostEntityIdOwnerWrapperEntity {
}

impl super::entity::EntityTrait for ServerGhostEntityIdOwnerWrapperEntity {
}

impl super::entity::EntityBusPeerTrait for ServerGhostEntityIdOwnerWrapperEntity {
}

pub static SERVERGHOSTENTITYIDOWNERWRAPPERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGhostEntityIdOwnerWrapperEntity",
    name_hash: 3041255960,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerGhostEntityIdOwnerWrapperEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGhostEntityIdOwnerWrapperEntity as Default>::default())),
            create_boxed: || Box::new(<ServerGhostEntityIdOwnerWrapperEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERGHOSTENTITYIDOWNERWRAPPERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerGhostEntityIdOwnerWrapperEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGHOSTENTITYIDOWNERWRAPPERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERGHOSTENTITYIDOWNERWRAPPERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGhostEntityIdOwnerWrapperEntity-Array",
    name_hash: 894970924,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ServerGhostEntityIdOwnerWrapperEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerCharacterProxyEntity {
    pub _glacier_base: ServerBlueprintProxyEntity,
}

pub trait ServerCharacterProxyEntityTrait: ServerBlueprintProxyEntityTrait {
}

impl ServerCharacterProxyEntityTrait for ServerCharacterProxyEntity {
}

impl ServerBlueprintProxyEntityTrait for ServerCharacterProxyEntity {
}

impl super::dice_commons_shared::BlueprintProxyEntityBaseTrait for ServerCharacterProxyEntity {
}

impl super::entity::EntityTrait for ServerCharacterProxyEntity {
}

impl super::entity::EntityBusPeerTrait for ServerCharacterProxyEntity {
}

pub static SERVERCHARACTERPROXYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterProxyEntity",
    name_hash: 397649422,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERBLUEPRINTPROXYENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerCharacterProxyEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCharacterProxyEntity as Default>::default())),
            create_boxed: || Box::new(<ServerCharacterProxyEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCHARACTERPROXYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCharacterProxyEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCHARACTERPROXYENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERCHARACTERPROXYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterProxyEntity-Array",
    name_hash: 2002915130,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ServerCharacterProxyEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerBlueprintSpawnEntity {
    pub _glacier_base: BlueprintSpawnEntity,
}

pub trait ServerBlueprintSpawnEntityTrait: BlueprintSpawnEntityTrait {
}

impl ServerBlueprintSpawnEntityTrait for ServerBlueprintSpawnEntity {
}

impl BlueprintSpawnEntityTrait for ServerBlueprintSpawnEntity {
}

impl super::entity::SpatialEntityTrait for ServerBlueprintSpawnEntity {
}

impl super::entity::EntityTrait for ServerBlueprintSpawnEntity {
}

impl super::entity::EntityBusPeerTrait for ServerBlueprintSpawnEntity {
}

pub static SERVERBLUEPRINTSPAWNENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBlueprintSpawnEntity",
    name_hash: 752216591,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BLUEPRINTSPAWNENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerBlueprintSpawnEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerBlueprintSpawnEntity as Default>::default())),
            create_boxed: || Box::new(<ServerBlueprintSpawnEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERBLUEPRINTSPAWNENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerBlueprintSpawnEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERBLUEPRINTSPAWNENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERBLUEPRINTSPAWNENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBlueprintSpawnEntity-Array",
    name_hash: 2457757371,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ServerBlueprintSpawnEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerBlueprintProxyEntity {
    pub _glacier_base: super::dice_commons_shared::BlueprintProxyEntityBase,
}

pub trait ServerBlueprintProxyEntityTrait: super::dice_commons_shared::BlueprintProxyEntityBaseTrait {
}

impl ServerBlueprintProxyEntityTrait for ServerBlueprintProxyEntity {
}

impl super::dice_commons_shared::BlueprintProxyEntityBaseTrait for ServerBlueprintProxyEntity {
}

impl super::entity::EntityTrait for ServerBlueprintProxyEntity {
}

impl super::entity::EntityBusPeerTrait for ServerBlueprintProxyEntity {
}

pub static SERVERBLUEPRINTPROXYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBlueprintProxyEntity",
    name_hash: 1034689336,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::dice_commons_shared::BLUEPRINTPROXYENTITYBASE_TYPE_INFO),
        super_class_offset: offset_of!(ServerBlueprintProxyEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerBlueprintProxyEntity as Default>::default())),
            create_boxed: || Box::new(<ServerBlueprintProxyEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERBLUEPRINTPROXYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerBlueprintProxyEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERBLUEPRINTPROXYENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERBLUEPRINTPROXYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBlueprintProxyEntity-Array",
    name_hash: 3160704140,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ServerBlueprintProxyEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerActorEntity {
    pub _glacier_base: super::game_server::ServerPhysicsEntity,
}

pub trait ServerActorEntityTrait: super::game_server::ServerPhysicsEntityTrait {
}

impl ServerActorEntityTrait for ServerActorEntity {
}

impl super::game_server::ServerPhysicsEntityTrait for ServerActorEntity {
}

impl super::game_server::ServerGameComponentEntityTrait for ServerActorEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ServerActorEntity {
}

impl super::entity::ComponentEntityTrait for ServerActorEntity {
}

impl super::entity::SpatialEntityTrait for ServerActorEntity {
}

impl super::entity::EntityTrait for ServerActorEntity {
}

impl super::entity::EntityBusPeerTrait for ServerActorEntity {
}

pub static SERVERACTORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerActorEntity",
    name_hash: 3212922160,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_server::SERVERPHYSICSENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerActorEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerActorEntity as Default>::default())),
            create_boxed: || Box::new(<ServerActorEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERACTORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerActorEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERACTORENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERACTORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerActorEntity-Array",
    name_hash: 660068996,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("ServerActorEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DiceJobSchedulerSettings {
    pub _glacier_base: super::core::SystemSettings,
    pub config_file_name: String,
}

pub trait DiceJobSchedulerSettingsTrait: super::core::SystemSettingsTrait {
    fn config_file_name(&self) -> &String;
    fn config_file_name_mut(&mut self) -> &mut String;
}

impl DiceJobSchedulerSettingsTrait for DiceJobSchedulerSettings {
    fn config_file_name(&self) -> &String {
        &self.config_file_name
    }
    fn config_file_name_mut(&mut self) -> &mut String {
        &mut self.config_file_name
    }
}

impl super::core::SystemSettingsTrait for DiceJobSchedulerSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl super::core::DataContainerTrait for DiceJobSchedulerSettings {
}

pub static DICEJOBSCHEDULERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceJobSchedulerSettings",
    name_hash: 1124379355,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        super_class_offset: offset_of!(DiceJobSchedulerSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DiceJobSchedulerSettings as Default>::default())),
            create_boxed: || Box::new(<DiceJobSchedulerSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ConfigFileName",
                name_hash: 946103566,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(DiceJobSchedulerSettings, config_file_name),
            },
        ],
    }),
    array_type: Some(DICEJOBSCHEDULERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DiceJobSchedulerSettings {
    fn type_info(&self) -> &'static TypeInfo {
        DICEJOBSCHEDULERSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DICEJOBSCHEDULERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceJobSchedulerSettings-Array",
    name_hash: 560828911,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("DiceJobSchedulerSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LogitechLEDSettings {
    pub _glacier_base: super::core::SystemSettings,
    pub enable: bool,
    pub effect_start_wait_time: f32,
}

pub trait LogitechLEDSettingsTrait: super::core::SystemSettingsTrait {
    fn enable(&self) -> &bool;
    fn enable_mut(&mut self) -> &mut bool;
    fn effect_start_wait_time(&self) -> &f32;
    fn effect_start_wait_time_mut(&mut self) -> &mut f32;
}

impl LogitechLEDSettingsTrait for LogitechLEDSettings {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn enable_mut(&mut self) -> &mut bool {
        &mut self.enable
    }
    fn effect_start_wait_time(&self) -> &f32 {
        &self.effect_start_wait_time
    }
    fn effect_start_wait_time_mut(&mut self) -> &mut f32 {
        &mut self.effect_start_wait_time
    }
}

impl super::core::SystemSettingsTrait for LogitechLEDSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl super::core::DataContainerTrait for LogitechLEDSettings {
}

pub static LOGITECHLEDSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDSettings",
    name_hash: 3133799066,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        super_class_offset: offset_of!(LogitechLEDSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LogitechLEDSettings as Default>::default())),
            create_boxed: || Box::new(<LogitechLEDSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                name_hash: 2342790116,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LogitechLEDSettings, enable),
            },
            FieldInfoData {
                name: "EffectStartWaitTime",
                name_hash: 2931508876,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LogitechLEDSettings, effect_start_wait_time),
            },
        ],
    }),
    array_type: Some(LOGITECHLEDSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LogitechLEDSettings {
    fn type_info(&self) -> &'static TypeInfo {
        LOGITECHLEDSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static LOGITECHLEDSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogitechLEDSettings-Array",
    name_hash: 3663257006,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("LogitechLEDSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AnselSettings {
    pub _glacier_base: super::core::SystemSettings,
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

pub trait AnselSettingsTrait: super::core::SystemSettingsTrait {
    fn enable(&self) -> &bool;
    fn enable_mut(&mut self) -> &mut bool;
    fn config_right(&self) -> &super::core::Vec3;
    fn config_right_mut(&mut self) -> &mut super::core::Vec3;
    fn config_up(&self) -> &super::core::Vec3;
    fn config_up_mut(&mut self) -> &mut super::core::Vec3;
    fn config_forward(&self) -> &super::core::Vec3;
    fn config_forward_mut(&mut self) -> &mut super::core::Vec3;
    fn config_translational_speed_in_world_units_per_second(&self) -> &f32;
    fn config_translational_speed_in_world_units_per_second_mut(&mut self) -> &mut f32;
    fn config_rotational_speed_in_degrees_per_second(&self) -> &f32;
    fn config_rotational_speed_in_degrees_per_second_mut(&mut self) -> &mut f32;
    fn config_meters_in_world_unit(&self) -> &f32;
    fn config_meters_in_world_unit_mut(&mut self) -> &mut f32;
    fn config_capture_latency(&self) -> &u32;
    fn config_capture_latency_mut(&mut self) -> &mut u32;
    fn config_capture_settle_latency(&self) -> &u32;
    fn config_capture_settle_latency_mut(&mut self) -> &mut u32;
    fn config_title_name_utf8(&self) -> &String;
    fn config_title_name_utf8_mut(&mut self) -> &mut String;
    fn session_config_maximum_fov_in_degrees(&self) -> &f32;
    fn session_config_maximum_fov_in_degrees_mut(&mut self) -> &mut f32;
    fn camera_radius(&self) -> &f32;
    fn camera_radius_mut(&mut self) -> &mut f32;
    fn camera_max_wander_distance(&self) -> &f32;
    fn camera_max_wander_distance_mut(&mut self) -> &mut f32;
    fn camera_offset_f_p_s(&self) -> &super::core::Vec3;
    fn camera_offset_f_p_s_mut(&mut self) -> &mut super::core::Vec3;
    fn camera_priority(&self) -> &i32;
    fn camera_priority_mut(&mut self) -> &mut i32;
}

impl AnselSettingsTrait for AnselSettings {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn enable_mut(&mut self) -> &mut bool {
        &mut self.enable
    }
    fn config_right(&self) -> &super::core::Vec3 {
        &self.config_right
    }
    fn config_right_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.config_right
    }
    fn config_up(&self) -> &super::core::Vec3 {
        &self.config_up
    }
    fn config_up_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.config_up
    }
    fn config_forward(&self) -> &super::core::Vec3 {
        &self.config_forward
    }
    fn config_forward_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.config_forward
    }
    fn config_translational_speed_in_world_units_per_second(&self) -> &f32 {
        &self.config_translational_speed_in_world_units_per_second
    }
    fn config_translational_speed_in_world_units_per_second_mut(&mut self) -> &mut f32 {
        &mut self.config_translational_speed_in_world_units_per_second
    }
    fn config_rotational_speed_in_degrees_per_second(&self) -> &f32 {
        &self.config_rotational_speed_in_degrees_per_second
    }
    fn config_rotational_speed_in_degrees_per_second_mut(&mut self) -> &mut f32 {
        &mut self.config_rotational_speed_in_degrees_per_second
    }
    fn config_meters_in_world_unit(&self) -> &f32 {
        &self.config_meters_in_world_unit
    }
    fn config_meters_in_world_unit_mut(&mut self) -> &mut f32 {
        &mut self.config_meters_in_world_unit
    }
    fn config_capture_latency(&self) -> &u32 {
        &self.config_capture_latency
    }
    fn config_capture_latency_mut(&mut self) -> &mut u32 {
        &mut self.config_capture_latency
    }
    fn config_capture_settle_latency(&self) -> &u32 {
        &self.config_capture_settle_latency
    }
    fn config_capture_settle_latency_mut(&mut self) -> &mut u32 {
        &mut self.config_capture_settle_latency
    }
    fn config_title_name_utf8(&self) -> &String {
        &self.config_title_name_utf8
    }
    fn config_title_name_utf8_mut(&mut self) -> &mut String {
        &mut self.config_title_name_utf8
    }
    fn session_config_maximum_fov_in_degrees(&self) -> &f32 {
        &self.session_config_maximum_fov_in_degrees
    }
    fn session_config_maximum_fov_in_degrees_mut(&mut self) -> &mut f32 {
        &mut self.session_config_maximum_fov_in_degrees
    }
    fn camera_radius(&self) -> &f32 {
        &self.camera_radius
    }
    fn camera_radius_mut(&mut self) -> &mut f32 {
        &mut self.camera_radius
    }
    fn camera_max_wander_distance(&self) -> &f32 {
        &self.camera_max_wander_distance
    }
    fn camera_max_wander_distance_mut(&mut self) -> &mut f32 {
        &mut self.camera_max_wander_distance
    }
    fn camera_offset_f_p_s(&self) -> &super::core::Vec3 {
        &self.camera_offset_f_p_s
    }
    fn camera_offset_f_p_s_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.camera_offset_f_p_s
    }
    fn camera_priority(&self) -> &i32 {
        &self.camera_priority
    }
    fn camera_priority_mut(&mut self) -> &mut i32 {
        &mut self.camera_priority
    }
}

impl super::core::SystemSettingsTrait for AnselSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl super::core::DataContainerTrait for AnselSettings {
}

pub static ANSELSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnselSettings",
    name_hash: 1277452885,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        super_class_offset: offset_of!(AnselSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AnselSettings as Default>::default())),
            create_boxed: || Box::new(<AnselSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                name_hash: 2342790116,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AnselSettings, enable),
            },
            FieldInfoData {
                name: "ConfigRight",
                name_hash: 547442543,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AnselSettings, config_right),
            },
            FieldInfoData {
                name: "ConfigUp",
                name_hash: 317085418,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AnselSettings, config_up),
            },
            FieldInfoData {
                name: "ConfigForward",
                name_hash: 3502037524,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AnselSettings, config_forward),
            },
            FieldInfoData {
                name: "ConfigTranslationalSpeedInWorldUnitsPerSecond",
                name_hash: 2742657577,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AnselSettings, config_translational_speed_in_world_units_per_second),
            },
            FieldInfoData {
                name: "ConfigRotationalSpeedInDegreesPerSecond",
                name_hash: 3044911622,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AnselSettings, config_rotational_speed_in_degrees_per_second),
            },
            FieldInfoData {
                name: "ConfigMetersInWorldUnit",
                name_hash: 3801437876,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AnselSettings, config_meters_in_world_unit),
            },
            FieldInfoData {
                name: "ConfigCaptureLatency",
                name_hash: 3254394339,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(AnselSettings, config_capture_latency),
            },
            FieldInfoData {
                name: "ConfigCaptureSettleLatency",
                name_hash: 134014172,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(AnselSettings, config_capture_settle_latency),
            },
            FieldInfoData {
                name: "ConfigTitleNameUtf8",
                name_hash: 279348215,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(AnselSettings, config_title_name_utf8),
            },
            FieldInfoData {
                name: "SessionConfigMaximumFovInDegrees",
                name_hash: 370033862,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AnselSettings, session_config_maximum_fov_in_degrees),
            },
            FieldInfoData {
                name: "CameraRadius",
                name_hash: 1745836836,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AnselSettings, camera_radius),
            },
            FieldInfoData {
                name: "CameraMaxWanderDistance",
                name_hash: 3805355232,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AnselSettings, camera_max_wander_distance),
            },
            FieldInfoData {
                name: "CameraOffsetFPS",
                name_hash: 3233752180,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AnselSettings, camera_offset_f_p_s),
            },
            FieldInfoData {
                name: "CameraPriority",
                name_hash: 3636719022,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AnselSettings, camera_priority),
            },
        ],
    }),
    array_type: Some(ANSELSETTINGS_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AnselSettings {
    fn type_info(&self) -> &'static TypeInfo {
        ANSELSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static ANSELSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnselSettings-Array",
    name_hash: 3916235361,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("AnselSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WidgetReferenceEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait WidgetReferenceEntityTrait: super::entity::EntityTrait {
}

impl WidgetReferenceEntityTrait for WidgetReferenceEntity {
}

impl super::entity::EntityTrait for WidgetReferenceEntity {
}

impl super::entity::EntityBusPeerTrait for WidgetReferenceEntity {
}

pub static WIDGETREFERENCEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WidgetReferenceEntity",
    name_hash: 3854993625,
    flags: MemberInfoFlags::new(101),
    module: "DiceCommons",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(WidgetReferenceEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WidgetReferenceEntity as Default>::default())),
            create_boxed: || Box::new(<WidgetReferenceEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(WIDGETREFERENCEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for WidgetReferenceEntity {
    fn type_info(&self) -> &'static TypeInfo {
        WIDGETREFERENCEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static WIDGETREFERENCEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WidgetReferenceEntity-Array",
    name_hash: 2012329709,
    flags: MemberInfoFlags::new(145),
    module: "DiceCommons",
    data: TypeInfoData::Array("WidgetReferenceEntity"),
    array_type: None,
    alignment: 8,
};


