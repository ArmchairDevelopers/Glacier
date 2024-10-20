use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_u_i_incubator_shared_types(registry: &mut TypeRegistry) {
    registry.register_type(UIMASKINGWIDGETENTITYDATA_TYPE_INFO);
    registry.register_type(UIMASKINGWIDGETENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(STRINGSWITCHCASEENTITYDATA_TYPE_INFO);
    registry.register_type(STRINGSWITCHCASEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(STRINGSWITCHCOMPARETYPE_TYPE_INFO);
    registry.register_type(STRINGSWITCHCOMPARETYPE_ARRAY_TYPE_INFO);
    registry.register_type(INTEGERSWITCHCASEENTITYDATA_TYPE_INFO);
    registry.register_type(INTEGERSWITCHCASEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(BASESWITCHCASEENTITYDATA_TYPE_INFO);
    registry.register_type(BASESWITCHCASEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(STATENODEENTITYDATA_TYPE_INFO);
    registry.register_type(STATENODEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(STATENAVEVENTINFO_TYPE_INFO);
    registry.register_type(STATENAVEVENTINFO_ARRAY_TYPE_INFO);
    registry.register_type(STATENODEENTITYBASEDATA_TYPE_INFO);
    registry.register_type(STATENODEENTITYBASEDATA_ARRAY_TYPE_INFO);
    registry.register_type(SELECTOBJECTENTITYDATA_TYPE_INFO);
    registry.register_type(SELECTOBJECTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(QUITGAMEENTITYDATA_TYPE_INFO);
    registry.register_type(QUITGAMEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(TRANSFORMINTERPOLATORENTITYDATA_TYPE_INFO);
    registry.register_type(TRANSFORMINTERPOLATORENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(VEC4INTERPOLATORENTITYDATA_TYPE_INFO);
    registry.register_type(VEC4INTERPOLATORENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(VEC3INTERPOLATORENTITYDATA_TYPE_INFO);
    registry.register_type(VEC3INTERPOLATORENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(VEC2INTERPOLATORENTITYDATA_TYPE_INFO);
    registry.register_type(VEC2INTERPOLATORENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(FLOATINTERPOLATORENTITYDATA_TYPE_INFO);
    registry.register_type(FLOATINTERPOLATORENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PROPERTYINTERPOLATORENTITYDATA_TYPE_INFO);
    registry.register_type(PROPERTYINTERPOLATORENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PROPERTYINTERPOLATIONMODE_TYPE_INFO);
    registry.register_type(PROPERTYINTERPOLATIONMODE_ARRAY_TYPE_INFO);
    registry.register_type(PROPERTYINTERPOLATIONTYPE_TYPE_INFO);
    registry.register_type(PROPERTYINTERPOLATIONTYPE_ARRAY_TYPE_INFO);
    registry.register_type(OBJECTHUBENTITYDATA_TYPE_INFO);
    registry.register_type(OBJECTHUBENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(MATHINTOPENTITYDATA_TYPE_INFO);
    registry.register_type(MATHINTOPENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(INTMATHOP_TYPE_INFO);
    registry.register_type(INTMATHOP_ARRAY_TYPE_INFO);
    registry.register_type(LOCALIZEDSTRINGENTITYDATA_TYPE_INFO);
    registry.register_type(LOCALIZEDSTRINGENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(LOCALIZEDSTRINGENTITYBASEDATA_TYPE_INFO);
    registry.register_type(LOCALIZEDSTRINGENTITYBASEDATA_ARRAY_TYPE_INFO);
    registry.register_type(LOCALIZEDSTRINGARGUMENTTYPE_TYPE_INFO);
    registry.register_type(LOCALIZEDSTRINGARGUMENTTYPE_ARRAY_TYPE_INFO);
    registry.register_type(LOCALIZEDSTRINGID_TYPE_INFO);
    registry.register_type(LOCALIZEDSTRINGID_ARRAY_TYPE_INFO);
    registry.register_type(LOCALIZEDSTRING_TYPE_INFO);
    registry.register_type(LOCALIZEDSTRING_ARRAY_TYPE_INFO);
    registry.register_type(LOCALIZEDSTRINGENCODING_TYPE_INFO);
    registry.register_type(LOCALIZEDSTRINGENCODING_ARRAY_TYPE_INFO);
    registry.register_type(TEXTURESWITCHENTITYDATA_TYPE_INFO);
    registry.register_type(TEXTURESWITCHENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(FBUILISTITEMWIDGETENTITYDATA_TYPE_INFO);
    registry.register_type(FBUILISTITEMWIDGETENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(FBUILISTELEMENTENTITYDATA_TYPE_INFO);
    registry.register_type(FBUILISTELEMENTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(FBUISLICEDTEXTUREELEMENTENTITYDATA_TYPE_INFO);
    registry.register_type(FBUISLICEDTEXTUREELEMENTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(FBUIMOVIEELEMENTENTITYDATA_TYPE_INFO);
    registry.register_type(FBUIMOVIEELEMENTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(FBUIDYNAMICTEXTUREELEMENTENTITYDATA_TYPE_INFO);
    registry.register_type(FBUIDYNAMICTEXTUREELEMENTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(FBUISTATICTEXTUREELEMENTENTITYDATA_TYPE_INFO);
    registry.register_type(FBUISTATICTEXTUREELEMENTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(FBUILABELELEMENTENTITYDATA_TYPE_INFO);
    registry.register_type(FBUILABELELEMENTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWSLICEDTEXTURECOMMANDDYNAMICSTATE_TYPE_INFO);
    registry.register_type(UIDRAWSLICEDTEXTURECOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWSLICEDTEXTURECOMMANDSTATICSTATE_TYPE_INFO);
    registry.register_type(UIDRAWSLICEDTEXTURECOMMANDSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(DYNAMICCASTENTITYDATA_TYPE_INFO);
    registry.register_type(DYNAMICCASTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CONFIGENTITYDATA_TYPE_INFO);
    registry.register_type(CONFIGENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(STRINGLISTASSET_TYPE_INFO);
    registry.register_type(STRINGLISTASSET_ARRAY_TYPE_INFO);
    registry.register_type(INTLISTASSET_TYPE_INFO);
    registry.register_type(INTLISTASSET_ARRAY_TYPE_INFO);
    registry.register_type(FLOATLISTASSET_TYPE_INFO);
    registry.register_type(FLOATLISTASSET_ARRAY_TYPE_INFO);
    registry.register_type(COLORLISTASSET_TYPE_INFO);
    registry.register_type(COLORLISTASSET_ARRAY_TYPE_INFO);
    registry.register_type(VEC3LISTASSET_TYPE_INFO);
    registry.register_type(VEC3LISTASSET_ARRAY_TYPE_INFO);
    registry.register_type(CONFIGLISTASSET_TYPE_INFO);
    registry.register_type(CONFIGLISTASSET_ARRAY_TYPE_INFO);
    registry.register_type(CONFIGENTITYASSETDATA_TYPE_INFO);
    registry.register_type(CONFIGENTITYASSETDATA_ARRAY_TYPE_INFO);
    registry.register_type(STRINGDATA_TYPE_INFO);
    registry.register_type(STRINGDATA_ARRAY_TYPE_INFO);
    registry.register_type(INTDATA_TYPE_INFO);
    registry.register_type(INTDATA_ARRAY_TYPE_INFO);
    registry.register_type(FLOATDATA_TYPE_INFO);
    registry.register_type(FLOATDATA_ARRAY_TYPE_INFO);
    registry.register_type(COLORDATA_TYPE_INFO);
    registry.register_type(COLORDATA_ARRAY_TYPE_INFO);
    registry.register_type(VEC3DATA_TYPE_INFO);
    registry.register_type(VEC3DATA_ARRAY_TYPE_INFO);
    registry.register_type(CONDITIONALSTRINGENTITYDATA_TYPE_INFO);
    registry.register_type(CONDITIONALSTRINGENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CONDITIONALTRANSFORMENTITYDATA_TYPE_INFO);
    registry.register_type(CONDITIONALTRANSFORMENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CONDITIONALVEC4ENTITYDATA_TYPE_INFO);
    registry.register_type(CONDITIONALVEC4ENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CONDITIONALVEC3ENTITYDATA_TYPE_INFO);
    registry.register_type(CONDITIONALVEC3ENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CONDITIONALVEC2ENTITYDATA_TYPE_INFO);
    registry.register_type(CONDITIONALVEC2ENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CONDITIONALFLOATENTITYDATA_TYPE_INFO);
    registry.register_type(CONDITIONALFLOATENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CONDITIONALINTENTITYDATA_TYPE_INFO);
    registry.register_type(CONDITIONALINTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CONDITIONALSTATEENTITYDATA_TYPE_INFO);
    registry.register_type(CONDITIONALSTATEENTITYDATA_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Debug)]
pub struct UIMaskingWidgetEntityData {
    pub mask_texture_id: String,
    pub mask_alpha: f32,
    pub invert_mask: bool,
}

pub const UIMASKINGWIDGETENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMaskingWidgetEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIWIDGETENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MaskTextureId",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(UIMaskingWidgetEntityData, mask_texture_id),
            },
            FieldInfoData {
                name: "MaskAlpha",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIMaskingWidgetEntityData, mask_alpha),
            },
            FieldInfoData {
                name: "InvertMask",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIMaskingWidgetEntityData, invert_mask),
            },
        ],
    }),
    array_type: Some(UIMASKINGWIDGETENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIMaskingWidgetEntityData {
    fn type_info() -> &'static TypeInfo {
        UIMASKINGWIDGETENTITYDATA_TYPE_INFO
    }
}


pub const UIMASKINGWIDGETENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMaskingWidgetEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("UIMaskingWidgetEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StringSwitchCaseEntityData {
    pub cases: Vec<String>,
    pub in_value: String,
    pub case_sensitive_compare: bool,
    pub compare_type: StringSwitchCompareType,
}

pub const STRINGSWITCHCASEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StringSwitchCaseEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BASESWITCHCASEENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Cases",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(StringSwitchCaseEntityData, cases),
            },
            FieldInfoData {
                name: "InValue",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(StringSwitchCaseEntityData, in_value),
            },
            FieldInfoData {
                name: "CaseSensitiveCompare",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(StringSwitchCaseEntityData, case_sensitive_compare),
            },
            FieldInfoData {
                name: "CompareType",
                flags: MemberInfoFlags::new(0),
                field_type: STRINGSWITCHCOMPARETYPE_TYPE_INFO,
                rust_offset: offset_of!(StringSwitchCaseEntityData, compare_type),
            },
        ],
    }),
    array_type: Some(STRINGSWITCHCASEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StringSwitchCaseEntityData {
    fn type_info() -> &'static TypeInfo {
        STRINGSWITCHCASEENTITYDATA_TYPE_INFO
    }
}


pub const STRINGSWITCHCASEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StringSwitchCaseEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("StringSwitchCaseEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum StringSwitchCompareType {
    #[default]
    StringSwitchCompareType_Equals = 0,
    StringSwitchCompareType_Contains = 1,
    StringSwitchCompareType_StartsWith = 2,
    StringSwitchCompareType_EndsWith = 3,
}

pub const STRINGSWITCHCOMPARETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StringSwitchCompareType",
    flags: MemberInfoFlags::new(49429),
    module: "UIIncubatorShared",
    data: TypeInfoData::Enum,
    array_type: Some(STRINGSWITCHCOMPARETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for StringSwitchCompareType {
    fn type_info() -> &'static TypeInfo {
        STRINGSWITCHCOMPARETYPE_TYPE_INFO
    }
}


pub const STRINGSWITCHCOMPARETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StringSwitchCompareType-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("StringSwitchCompareType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IntegerSwitchCaseEntityData {
    pub cases: Vec<i32>,
    pub in_value: i32,
}

pub const INTEGERSWITCHCASEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntegerSwitchCaseEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BASESWITCHCASEENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Cases",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(IntegerSwitchCaseEntityData, cases),
            },
            FieldInfoData {
                name: "InValue",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(IntegerSwitchCaseEntityData, in_value),
            },
        ],
    }),
    array_type: Some(INTEGERSWITCHCASEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IntegerSwitchCaseEntityData {
    fn type_info() -> &'static TypeInfo {
        INTEGERSWITCHCASEENTITYDATA_TYPE_INFO
    }
}


pub const INTEGERSWITCHCASEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntegerSwitchCaseEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("IntegerSwitchCaseEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BaseSwitchCaseEntityData {
    pub realm: super::core::Realm,
    pub trigger_on_property_change: bool,
    pub trigger_on_start: bool,
}

pub const BASESWITCHCASEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BaseSwitchCaseEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(BaseSwitchCaseEntityData, realm),
            },
            FieldInfoData {
                name: "TriggerOnPropertyChange",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BaseSwitchCaseEntityData, trigger_on_property_change),
            },
            FieldInfoData {
                name: "TriggerOnStart",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BaseSwitchCaseEntityData, trigger_on_start),
            },
        ],
    }),
    array_type: Some(BASESWITCHCASEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BaseSwitchCaseEntityData {
    fn type_info() -> &'static TypeInfo {
        BASESWITCHCASEENTITYDATA_TYPE_INFO
    }
}


pub const BASESWITCHCASEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BaseSwitchCaseEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("BaseSwitchCaseEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StateNodeEntityData {
    pub event_triggers_info: Vec<StateNavEventInfo>,
    pub consumed_events_info: Vec<StateNavEventInfo>,
}

pub const STATENODEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StateNodeEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(STATENODEENTITYBASEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "EventTriggersInfo",
                flags: MemberInfoFlags::new(144),
                field_type: STATENAVEVENTINFO_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(StateNodeEntityData, event_triggers_info),
            },
            FieldInfoData {
                name: "ConsumedEventsInfo",
                flags: MemberInfoFlags::new(144),
                field_type: STATENAVEVENTINFO_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(StateNodeEntityData, consumed_events_info),
            },
        ],
    }),
    array_type: Some(STATENODEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StateNodeEntityData {
    fn type_info() -> &'static TypeInfo {
        STATENODEENTITYDATA_TYPE_INFO
    }
}


pub const STATENODEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StateNodeEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("StateNodeEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StateNavEventInfo {
    pub trigger_event_hash: i32,
    pub on_event_hash: i32,
}

pub const STATENAVEVENTINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StateNavEventInfo",
    flags: MemberInfoFlags::new(36937),
    module: "UIIncubatorShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "TriggerEventHash",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(StateNavEventInfo, trigger_event_hash),
            },
            FieldInfoData {
                name: "OnEventHash",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(StateNavEventInfo, on_event_hash),
            },
        ],
    }),
    array_type: Some(STATENAVEVENTINFO_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for StateNavEventInfo {
    fn type_info() -> &'static TypeInfo {
        STATENAVEVENTINFO_TYPE_INFO
    }
}


pub const STATENAVEVENTINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StateNavEventInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("StateNavEventInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StateNodeEntityBaseData {
    pub state_name: String,
    pub realm: super::core::Realm,
}

pub const STATENODEENTITYBASEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StateNodeEntityBaseData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "StateName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(StateNodeEntityBaseData, state_name),
            },
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(StateNodeEntityBaseData, realm),
            },
        ],
    }),
    array_type: Some(STATENODEENTITYBASEDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for StateNodeEntityBaseData {
    fn type_info() -> &'static TypeInfo {
        STATENODEENTITYBASEDATA_TYPE_INFO
    }
}


pub const STATENODEENTITYBASEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StateNodeEntityBaseData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("StateNodeEntityBaseData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SelectObjectEntityData {
    pub realm: super::core::Realm,
    pub inputs: Vec<String>,
    pub input_select: i32,
    pub dynamic_input_data_type: u32,
}

pub const SELECTOBJECTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectObjectEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(SelectObjectEntityData, realm),
            },
            FieldInfoData {
                name: "Inputs",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SelectObjectEntityData, inputs),
            },
            FieldInfoData {
                name: "InputSelect",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(SelectObjectEntityData, input_select),
            },
            FieldInfoData {
                name: "DynamicInputDataType",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SelectObjectEntityData, dynamic_input_data_type),
            },
        ],
    }),
    array_type: Some(SELECTOBJECTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SelectObjectEntityData {
    fn type_info() -> &'static TypeInfo {
        SELECTOBJECTENTITYDATA_TYPE_INFO
    }
}


pub const SELECTOBJECTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectObjectEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("SelectObjectEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct QuitGameEntityData {
}

pub const QUITGAMEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuitGameEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(QUITGAMEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for QuitGameEntityData {
    fn type_info() -> &'static TypeInfo {
        QUITGAMEENTITYDATA_TYPE_INFO
    }
}


pub const QUITGAMEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuitGameEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("QuitGameEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TransformInterpolatorEntityData {
    pub r#in: super::core::LinearTransform,
    pub default_value: super::core::LinearTransform,
}

pub const TRANSFORMINTERPOLATORENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformInterpolatorEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROPERTYINTERPOLATORENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "In",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(TransformInterpolatorEntityData, r#in),
            },
            FieldInfoData {
                name: "DefaultValue",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(TransformInterpolatorEntityData, default_value),
            },
        ],
    }),
    array_type: Some(TRANSFORMINTERPOLATORENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TransformInterpolatorEntityData {
    fn type_info() -> &'static TypeInfo {
        TRANSFORMINTERPOLATORENTITYDATA_TYPE_INFO
    }
}


pub const TRANSFORMINTERPOLATORENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformInterpolatorEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("TransformInterpolatorEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct Vec4InterpolatorEntityData {
    pub r#in: super::core::Vec4,
    pub default_value: super::core::Vec4,
    pub use_velocity: bool,
    pub velocity: f32,
}

pub const VEC4INTERPOLATORENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec4InterpolatorEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROPERTYINTERPOLATORENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "In",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(Vec4InterpolatorEntityData, r#in),
            },
            FieldInfoData {
                name: "DefaultValue",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(Vec4InterpolatorEntityData, default_value),
            },
            FieldInfoData {
                name: "UseVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Vec4InterpolatorEntityData, use_velocity),
            },
            FieldInfoData {
                name: "Velocity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Vec4InterpolatorEntityData, velocity),
            },
        ],
    }),
    array_type: Some(VEC4INTERPOLATORENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for Vec4InterpolatorEntityData {
    fn type_info() -> &'static TypeInfo {
        VEC4INTERPOLATORENTITYDATA_TYPE_INFO
    }
}


pub const VEC4INTERPOLATORENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec4InterpolatorEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("Vec4InterpolatorEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct Vec3InterpolatorEntityData {
    pub r#in: super::core::Vec3,
    pub default_value: super::core::Vec3,
    pub use_velocity: bool,
    pub velocity: f32,
}

pub const VEC3INTERPOLATORENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3InterpolatorEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROPERTYINTERPOLATORENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "In",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(Vec3InterpolatorEntityData, r#in),
            },
            FieldInfoData {
                name: "DefaultValue",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(Vec3InterpolatorEntityData, default_value),
            },
            FieldInfoData {
                name: "UseVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Vec3InterpolatorEntityData, use_velocity),
            },
            FieldInfoData {
                name: "Velocity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Vec3InterpolatorEntityData, velocity),
            },
        ],
    }),
    array_type: Some(VEC3INTERPOLATORENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for Vec3InterpolatorEntityData {
    fn type_info() -> &'static TypeInfo {
        VEC3INTERPOLATORENTITYDATA_TYPE_INFO
    }
}


pub const VEC3INTERPOLATORENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3InterpolatorEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("Vec3InterpolatorEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct Vec2InterpolatorEntityData {
    pub r#in: super::core::Vec2,
    pub default_value: super::core::Vec2,
    pub use_velocity: bool,
    pub velocity: f32,
}

pub const VEC2INTERPOLATORENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec2InterpolatorEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROPERTYINTERPOLATORENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "In",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(Vec2InterpolatorEntityData, r#in),
            },
            FieldInfoData {
                name: "DefaultValue",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(Vec2InterpolatorEntityData, default_value),
            },
            FieldInfoData {
                name: "UseVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Vec2InterpolatorEntityData, use_velocity),
            },
            FieldInfoData {
                name: "Velocity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Vec2InterpolatorEntityData, velocity),
            },
        ],
    }),
    array_type: Some(VEC2INTERPOLATORENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Vec2InterpolatorEntityData {
    fn type_info() -> &'static TypeInfo {
        VEC2INTERPOLATORENTITYDATA_TYPE_INFO
    }
}


pub const VEC2INTERPOLATORENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec2InterpolatorEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("Vec2InterpolatorEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FloatInterpolatorEntityData {
    pub r#in: f32,
    pub default_value: f32,
    pub use_velocity: bool,
    pub velocity: f32,
}

pub const FLOATINTERPOLATORENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatInterpolatorEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROPERTYINTERPOLATORENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "In",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FloatInterpolatorEntityData, r#in),
            },
            FieldInfoData {
                name: "DefaultValue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FloatInterpolatorEntityData, default_value),
            },
            FieldInfoData {
                name: "UseVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FloatInterpolatorEntityData, use_velocity),
            },
            FieldInfoData {
                name: "Velocity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FloatInterpolatorEntityData, velocity),
            },
        ],
    }),
    array_type: Some(FLOATINTERPOLATORENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FloatInterpolatorEntityData {
    fn type_info() -> &'static TypeInfo {
        FLOATINTERPOLATORENTITYDATA_TYPE_INFO
    }
}


pub const FLOATINTERPOLATORENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatInterpolatorEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("FloatInterpolatorEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PropertyInterpolatorEntityData {
    pub realm: super::core::Realm,
    pub interpolation_type: PropertyInterpolationType,
    pub interpolation_mode: PropertyInterpolationMode,
    pub duration: f32,
    pub use_real_time_clock: bool,
    pub force_frame_correct_output: bool,
}

pub const PROPERTYINTERPOLATORENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyInterpolatorEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(PropertyInterpolatorEntityData, realm),
            },
            FieldInfoData {
                name: "InterpolationType",
                flags: MemberInfoFlags::new(0),
                field_type: PROPERTYINTERPOLATIONTYPE_TYPE_INFO,
                rust_offset: offset_of!(PropertyInterpolatorEntityData, interpolation_type),
            },
            FieldInfoData {
                name: "InterpolationMode",
                flags: MemberInfoFlags::new(0),
                field_type: PROPERTYINTERPOLATIONMODE_TYPE_INFO,
                rust_offset: offset_of!(PropertyInterpolatorEntityData, interpolation_mode),
            },
            FieldInfoData {
                name: "Duration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PropertyInterpolatorEntityData, duration),
            },
            FieldInfoData {
                name: "UseRealTimeClock",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PropertyInterpolatorEntityData, use_real_time_clock),
            },
            FieldInfoData {
                name: "ForceFrameCorrectOutput",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PropertyInterpolatorEntityData, force_frame_correct_output),
            },
        ],
    }),
    array_type: Some(PROPERTYINTERPOLATORENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PropertyInterpolatorEntityData {
    fn type_info() -> &'static TypeInfo {
        PROPERTYINTERPOLATORENTITYDATA_TYPE_INFO
    }
}


pub const PROPERTYINTERPOLATORENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyInterpolatorEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("PropertyInterpolatorEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum PropertyInterpolationMode {
    #[default]
    PropertyInterpolationMode_EaseIn = 0,
    PropertyInterpolationMode_EaseOut = 1,
    PropertyInterpolationMode_EaseInOut = 2,
    PropertyInterpolationMode_EaseOutIn = 3,
    PropertyInterpolationMode_Count = 4,
}

pub const PROPERTYINTERPOLATIONMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyInterpolationMode",
    flags: MemberInfoFlags::new(49429),
    module: "UIIncubatorShared",
    data: TypeInfoData::Enum,
    array_type: Some(PROPERTYINTERPOLATIONMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PropertyInterpolationMode {
    fn type_info() -> &'static TypeInfo {
        PROPERTYINTERPOLATIONMODE_TYPE_INFO
    }
}


pub const PROPERTYINTERPOLATIONMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyInterpolationMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("PropertyInterpolationMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum PropertyInterpolationType {
    #[default]
    PropertyInterpolationType_Linear = 0,
    PropertyInterpolationType_Quad = 1,
    PropertyInterpolationType_Cubic = 2,
    PropertyInterpolationType_Quart = 3,
    PropertyInterpolationType_Quint = 4,
    PropertyInterpolationType_Expo = 5,
    PropertyInterpolationType_Sine = 6,
    PropertyInterpolationType_Circ = 7,
    PropertyInterpolationType_Back = 8,
    PropertyInterpolationType_Elastic = 9,
    PropertyInterpolationType_Bounce = 10,
    PropertyInterpolationType_Count = 11,
}

pub const PROPERTYINTERPOLATIONTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyInterpolationType",
    flags: MemberInfoFlags::new(49429),
    module: "UIIncubatorShared",
    data: TypeInfoData::Enum,
    array_type: Some(PROPERTYINTERPOLATIONTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PropertyInterpolationType {
    fn type_info() -> &'static TypeInfo {
        PROPERTYINTERPOLATIONTYPE_TYPE_INFO
    }
}


pub const PROPERTYINTERPOLATIONTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyInterpolationType-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("PropertyInterpolationType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ObjectHubEntityData {
    pub realm: super::core::Realm,
    pub input_select: i32,
    pub dynamic_input_data_type: u32,
}

pub const OBJECTHUBENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectHubEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(ObjectHubEntityData, realm),
            },
            FieldInfoData {
                name: "InputSelect",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ObjectHubEntityData, input_select),
            },
            FieldInfoData {
                name: "DynamicInputDataType",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ObjectHubEntityData, dynamic_input_data_type),
            },
        ],
    }),
    array_type: Some(OBJECTHUBENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ObjectHubEntityData {
    fn type_info() -> &'static TypeInfo {
        OBJECTHUBENTITYDATA_TYPE_INFO
    }
}


pub const OBJECTHUBENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectHubEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("ObjectHubEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MathIntOpEntityData {
    pub realm: super::core::Realm,
    pub operators: Vec<IntMathOp>,
}

pub const MATHINTOPENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MathIntOpEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(MathIntOpEntityData, realm),
            },
            FieldInfoData {
                name: "Operators",
                flags: MemberInfoFlags::new(144),
                field_type: INTMATHOP_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MathIntOpEntityData, operators),
            },
        ],
    }),
    array_type: Some(MATHINTOPENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MathIntOpEntityData {
    fn type_info() -> &'static TypeInfo {
        MATHINTOPENTITYDATA_TYPE_INFO
    }
}


pub const MATHINTOPENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MathIntOpEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("MathIntOpEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum IntMathOp {
    #[default]
    IntMathOp_Add = 0,
    IntMathOp_Subtract = 1,
    IntMathOp_Multiply = 2,
    IntMathOp_Divide = 3,
    IntMathOp_Min = 4,
    IntMathOp_Max = 5,
    IntMathOp_Modulo = 6,
    IntMathOp_Exponent = 7,
}

pub const INTMATHOP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntMathOp",
    flags: MemberInfoFlags::new(49429),
    module: "UIIncubatorShared",
    data: TypeInfoData::Enum,
    array_type: Some(INTMATHOP_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for IntMathOp {
    fn type_info() -> &'static TypeInfo {
        INTMATHOP_TYPE_INFO
    }
}


pub const INTMATHOP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntMathOp-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("IntMathOp-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LocalizedStringEntityData {
    pub sid: String,
}

pub const LOCALIZEDSTRINGENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedStringEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCALIZEDSTRINGENTITYBASEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Sid",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(LocalizedStringEntityData, sid),
            },
        ],
    }),
    array_type: Some(LOCALIZEDSTRINGENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LocalizedStringEntityData {
    fn type_info() -> &'static TypeInfo {
        LOCALIZEDSTRINGENTITYDATA_TYPE_INFO
    }
}


pub const LOCALIZEDSTRINGENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedStringEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("LocalizedStringEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LocalizedStringEntityBaseData {
    pub realm: super::core::Realm,
    pub arguments: Vec<LocalizedStringArgumentType>,
    pub argument_hashes: Vec<u32>,
}

pub const LOCALIZEDSTRINGENTITYBASEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedStringEntityBaseData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(LocalizedStringEntityBaseData, realm),
            },
            FieldInfoData {
                name: "Arguments",
                flags: MemberInfoFlags::new(144),
                field_type: LOCALIZEDSTRINGARGUMENTTYPE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LocalizedStringEntityBaseData, arguments),
            },
            FieldInfoData {
                name: "ArgumentHashes",
                flags: MemberInfoFlags::new(144),
                field_type: UINT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LocalizedStringEntityBaseData, argument_hashes),
            },
        ],
    }),
    array_type: Some(LOCALIZEDSTRINGENTITYBASEDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LocalizedStringEntityBaseData {
    fn type_info() -> &'static TypeInfo {
        LOCALIZEDSTRINGENTITYBASEDATA_TYPE_INFO
    }
}


pub const LOCALIZEDSTRINGENTITYBASEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedStringEntityBaseData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("LocalizedStringEntityBaseData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum LocalizedStringArgumentType {
    #[default]
    LocalizedStringArgumentType_Integer = 0,
    LocalizedStringArgumentType_Long = 1,
    LocalizedStringArgumentType_Float = 2,
    LocalizedStringArgumentType_String = 3,
    LocalizedStringArgumentType_LocalizedString = 4,
    LocalizedStringArgumentType_Date = 5,
    LocalizedStringArgumentType_Time = 6,
}

pub const LOCALIZEDSTRINGARGUMENTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedStringArgumentType",
    flags: MemberInfoFlags::new(49429),
    module: "UIIncubatorShared",
    data: TypeInfoData::Enum,
    array_type: Some(LOCALIZEDSTRINGARGUMENTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LocalizedStringArgumentType {
    fn type_info() -> &'static TypeInfo {
        LOCALIZEDSTRINGARGUMENTTYPE_TYPE_INFO
    }
}


pub const LOCALIZEDSTRINGARGUMENTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedStringArgumentType-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("LocalizedStringArgumentType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LocalizedStringId {
    pub string_hash: i32,
}

pub const LOCALIZEDSTRINGID_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedStringId",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "StringHash",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(LocalizedStringId, string_hash),
            },
        ],
    }),
    array_type: Some(LOCALIZEDSTRINGID_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LocalizedStringId {
    fn type_info() -> &'static TypeInfo {
        LOCALIZEDSTRINGID_TYPE_INFO
    }
}


pub const LOCALIZEDSTRINGID_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedStringId-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("LocalizedStringId-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LocalizedString {
    pub string: String,
    pub encoding: LocalizedStringEncoding,
}

pub const LOCALIZEDSTRING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedString",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "String",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(LocalizedString, string),
            },
            FieldInfoData {
                name: "Encoding",
                flags: MemberInfoFlags::new(0),
                field_type: LOCALIZEDSTRINGENCODING_TYPE_INFO,
                rust_offset: offset_of!(LocalizedString, encoding),
            },
        ],
    }),
    array_type: Some(LOCALIZEDSTRING_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LocalizedString {
    fn type_info() -> &'static TypeInfo {
        LOCALIZEDSTRING_TYPE_INFO
    }
}


pub const LOCALIZEDSTRING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedString-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("LocalizedString-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum LocalizedStringEncoding {
    #[default]
    LocalizedStringEncoding_Unknown = 0,
    LocalizedStringEncoding_Packed = 1,
    LocalizedStringEncoding_UTF8 = 2,
}

pub const LOCALIZEDSTRINGENCODING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedStringEncoding",
    flags: MemberInfoFlags::new(49429),
    module: "UIIncubatorShared",
    data: TypeInfoData::Enum,
    array_type: Some(LOCALIZEDSTRINGENCODING_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LocalizedStringEncoding {
    fn type_info() -> &'static TypeInfo {
        LOCALIZEDSTRINGENCODING_TYPE_INFO
    }
}


pub const LOCALIZEDSTRINGENCODING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedStringEncoding-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("LocalizedStringEncoding-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TextureSwitchEntityData {
    pub textures: Vec<super::render::TextureAsset>,
}

pub const TEXTURESWITCHENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureSwitchEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Textures",
                flags: MemberInfoFlags::new(144),
                field_type: TEXTUREASSET_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(TextureSwitchEntityData, textures),
            },
        ],
    }),
    array_type: Some(TEXTURESWITCHENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TextureSwitchEntityData {
    fn type_info() -> &'static TypeInfo {
        TEXTURESWITCHENTITYDATA_TYPE_INFO
    }
}


pub const TEXTURESWITCHENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureSwitchEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("TextureSwitchEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FBUIListItemWidgetEntityData {
    pub out_type_hash: i32,
}

pub const FBUILISTITEMWIDGETENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUIListItemWidgetEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIWIDGETENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "OutTypeHash",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(FBUIListItemWidgetEntityData, out_type_hash),
            },
        ],
    }),
    array_type: Some(FBUILISTITEMWIDGETENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FBUIListItemWidgetEntityData {
    fn type_info() -> &'static TypeInfo {
        FBUILISTITEMWIDGETENTITYDATA_TYPE_INFO
    }
}


pub const FBUILISTITEMWIDGETENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUIListItemWidgetEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("FBUIListItemWidgetEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FBUIListElementEntityData {
    pub in_array: super::core::DataContainer,
    pub row_template: super::game_shared_u_i::UIWidgetBlueprint,
    pub row_height: u32,
}

pub const FBUILISTELEMENTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUIListElementEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIELEMENTENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "InArray",
                flags: MemberInfoFlags::new(0),
                field_type: DATACONTAINER_TYPE_INFO,
                rust_offset: offset_of!(FBUIListElementEntityData, in_array),
            },
            FieldInfoData {
                name: "RowTemplate",
                flags: MemberInfoFlags::new(0),
                field_type: UIWIDGETBLUEPRINT_TYPE_INFO,
                rust_offset: offset_of!(FBUIListElementEntityData, row_template),
            },
            FieldInfoData {
                name: "RowHeight",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FBUIListElementEntityData, row_height),
            },
        ],
    }),
    array_type: Some(FBUILISTELEMENTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FBUIListElementEntityData {
    fn type_info() -> &'static TypeInfo {
        FBUILISTELEMENTENTITYDATA_TYPE_INFO
    }
}


pub const FBUILISTELEMENTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUIListElementEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("FBUIListElementEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FBUISlicedTextureElementEntityData {
    pub texture: super::game_shared_u_i::UIAutoMappedTexture,
    pub slice_left: i32,
    pub slice_top: i32,
    pub slice_right: i32,
    pub slice_bottom: i32,
    pub padding_left: i32,
    pub padding_top: i32,
    pub padding_right: i32,
    pub padding_bottom: i32,
    pub fill_center: bool,
}

pub const FBUISLICEDTEXTUREELEMENTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUISlicedTextureElementEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIELEMENTENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: UIAUTOMAPPEDTEXTURE_TYPE_INFO,
                rust_offset: offset_of!(FBUISlicedTextureElementEntityData, texture),
            },
            FieldInfoData {
                name: "SliceLeft",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(FBUISlicedTextureElementEntityData, slice_left),
            },
            FieldInfoData {
                name: "SliceTop",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(FBUISlicedTextureElementEntityData, slice_top),
            },
            FieldInfoData {
                name: "SliceRight",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(FBUISlicedTextureElementEntityData, slice_right),
            },
            FieldInfoData {
                name: "SliceBottom",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(FBUISlicedTextureElementEntityData, slice_bottom),
            },
            FieldInfoData {
                name: "PaddingLeft",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(FBUISlicedTextureElementEntityData, padding_left),
            },
            FieldInfoData {
                name: "PaddingTop",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(FBUISlicedTextureElementEntityData, padding_top),
            },
            FieldInfoData {
                name: "PaddingRight",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(FBUISlicedTextureElementEntityData, padding_right),
            },
            FieldInfoData {
                name: "PaddingBottom",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(FBUISlicedTextureElementEntityData, padding_bottom),
            },
            FieldInfoData {
                name: "FillCenter",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FBUISlicedTextureElementEntityData, fill_center),
            },
        ],
    }),
    array_type: Some(FBUISLICEDTEXTUREELEMENTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FBUISlicedTextureElementEntityData {
    fn type_info() -> &'static TypeInfo {
        FBUISLICEDTEXTUREELEMENTENTITYDATA_TYPE_INFO
    }
}


pub const FBUISLICEDTEXTUREELEMENTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUISlicedTextureElementEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("FBUISlicedTextureElementEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FBUIMovieElementEntityData {
    pub movie: super::movie_base::MovieTextureBaseAsset,
    pub r#loop: bool,
    pub auto_start: bool,
    pub preload: bool,
    pub fullscreen: bool,
    pub volume: f32,
}

pub const FBUIMOVIEELEMENTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUIMovieElementEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIELEMENTENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Movie",
                flags: MemberInfoFlags::new(0),
                field_type: MOVIETEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(FBUIMovieElementEntityData, movie),
            },
            FieldInfoData {
                name: "Loop",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FBUIMovieElementEntityData, r#loop),
            },
            FieldInfoData {
                name: "AutoStart",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FBUIMovieElementEntityData, auto_start),
            },
            FieldInfoData {
                name: "Preload",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FBUIMovieElementEntityData, preload),
            },
            FieldInfoData {
                name: "Fullscreen",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FBUIMovieElementEntityData, fullscreen),
            },
            FieldInfoData {
                name: "Volume",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FBUIMovieElementEntityData, volume),
            },
        ],
    }),
    array_type: Some(FBUIMOVIEELEMENTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FBUIMovieElementEntityData {
    fn type_info() -> &'static TypeInfo {
        FBUIMOVIEELEMENTENTITYDATA_TYPE_INFO
    }
}


pub const FBUIMOVIEELEMENTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUIMovieElementEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("FBUIMovieElementEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FBUIDynamicTextureElementEntityData {
    pub uv_rect: super::core::Vec4,
    pub address_u: super::render_base::TextureAddress,
    pub address_v: super::render_base::TextureAddress,
    pub texture: super::render_base::TextureBaseAsset,
}

pub const FBUIDYNAMICTEXTUREELEMENTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUIDynamicTextureElementEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIELEMENTENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "UvRect",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(FBUIDynamicTextureElementEntityData, uv_rect),
            },
            FieldInfoData {
                name: "AddressU",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREADDRESS_TYPE_INFO,
                rust_offset: offset_of!(FBUIDynamicTextureElementEntityData, address_u),
            },
            FieldInfoData {
                name: "AddressV",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREADDRESS_TYPE_INFO,
                rust_offset: offset_of!(FBUIDynamicTextureElementEntityData, address_v),
            },
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(FBUIDynamicTextureElementEntityData, texture),
            },
        ],
    }),
    array_type: Some(FBUIDYNAMICTEXTUREELEMENTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FBUIDynamicTextureElementEntityData {
    fn type_info() -> &'static TypeInfo {
        FBUIDYNAMICTEXTUREELEMENTENTITYDATA_TYPE_INFO
    }
}


pub const FBUIDYNAMICTEXTUREELEMENTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUIDynamicTextureElementEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("FBUIDynamicTextureElementEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FBUIStaticTextureElementEntityData {
    pub uv_rect: super::core::Vec4,
    pub address_u: super::render_base::TextureAddress,
    pub address_v: super::render_base::TextureAddress,
    pub texture: super::game_shared_u_i::UIAutoMappedTexture,
}

pub const FBUISTATICTEXTUREELEMENTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUIStaticTextureElementEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIELEMENTENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "UvRect",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(FBUIStaticTextureElementEntityData, uv_rect),
            },
            FieldInfoData {
                name: "AddressU",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREADDRESS_TYPE_INFO,
                rust_offset: offset_of!(FBUIStaticTextureElementEntityData, address_u),
            },
            FieldInfoData {
                name: "AddressV",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREADDRESS_TYPE_INFO,
                rust_offset: offset_of!(FBUIStaticTextureElementEntityData, address_v),
            },
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: UIAUTOMAPPEDTEXTURE_TYPE_INFO,
                rust_offset: offset_of!(FBUIStaticTextureElementEntityData, texture),
            },
        ],
    }),
    array_type: Some(FBUISTATICTEXTUREELEMENTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FBUIStaticTextureElementEntityData {
    fn type_info() -> &'static TypeInfo {
        FBUISTATICTEXTUREELEMENTENTITYDATA_TYPE_INFO
    }
}


pub const FBUISTATICTEXTUREELEMENTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUIStaticTextureElementEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("FBUIStaticTextureElementEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FBUILabelElementEntityData {
    pub text: String,
    pub font_style: super::game_shared_u_i::UIElementFontStyle,
    pub word_wrap: bool,
    pub font_effect: super::game_shared_u_i::UIElementFontEffect,
    pub horizontal_alignment: super::game_base::UIElementAlignment,
    pub vertical_alignment: super::game_base::UIElementAlignment,
}

pub const FBUILABELELEMENTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUILabelElementEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIELEMENTENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Text",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(FBUILabelElementEntityData, text),
            },
            FieldInfoData {
                name: "FontStyle",
                flags: MemberInfoFlags::new(0),
                field_type: UIELEMENTFONTSTYLE_TYPE_INFO,
                rust_offset: offset_of!(FBUILabelElementEntityData, font_style),
            },
            FieldInfoData {
                name: "WordWrap",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FBUILabelElementEntityData, word_wrap),
            },
            FieldInfoData {
                name: "FontEffect",
                flags: MemberInfoFlags::new(0),
                field_type: UIELEMENTFONTEFFECT_TYPE_INFO,
                rust_offset: offset_of!(FBUILabelElementEntityData, font_effect),
            },
            FieldInfoData {
                name: "HorizontalAlignment",
                flags: MemberInfoFlags::new(0),
                field_type: UIELEMENTALIGNMENT_TYPE_INFO,
                rust_offset: offset_of!(FBUILabelElementEntityData, horizontal_alignment),
            },
            FieldInfoData {
                name: "VerticalAlignment",
                flags: MemberInfoFlags::new(0),
                field_type: UIELEMENTALIGNMENT_TYPE_INFO,
                rust_offset: offset_of!(FBUILabelElementEntityData, vertical_alignment),
            },
        ],
    }),
    array_type: Some(FBUILABELELEMENTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FBUILabelElementEntityData {
    fn type_info() -> &'static TypeInfo {
        FBUILABELELEMENTENTITYDATA_TYPE_INFO
    }
}


pub const FBUILABELELEMENTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUILabelElementEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("FBUILabelElementEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIDrawSlicedTextureCommandDynamicState {
    pub rect: super::core::Vec4,
    pub field_flag_changed0: u8,
}

pub const UIDRAWSLICEDTEXTURECOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawSlicedTextureCommandDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "UIIncubatorShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Rect",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(UIDrawSlicedTextureCommandDynamicState, rect),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIDrawSlicedTextureCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWSLICEDTEXTURECOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIDrawSlicedTextureCommandDynamicState {
    fn type_info() -> &'static TypeInfo {
        UIDRAWSLICEDTEXTURECOMMANDDYNAMICSTATE_TYPE_INFO
    }
}


pub const UIDRAWSLICEDTEXTURECOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawSlicedTextureCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("UIDrawSlicedTextureCommandDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIDrawSlicedTextureCommandStaticState {
    pub slice_left: i32,
    pub slice_top: i32,
    pub slice_right: i32,
    pub slice_bottom: i32,
    pub padding_left: i32,
    pub padding_top: i32,
    pub padding_right: i32,
    pub padding_bottom: i32,
    pub fill_center: bool,
    pub uv_rect: super::core::Vec4,
    pub texture_handle: super::render_base::TextureResourceHandle,
    pub field_flag_changed0: u16,
}

pub const UIDRAWSLICEDTEXTURECOMMANDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawSlicedTextureCommandStaticState",
    flags: MemberInfoFlags::new(73),
    module: "UIIncubatorShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "SliceLeft",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(UIDrawSlicedTextureCommandStaticState, slice_left),
            },
            FieldInfoData {
                name: "SliceTop",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(UIDrawSlicedTextureCommandStaticState, slice_top),
            },
            FieldInfoData {
                name: "SliceRight",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(UIDrawSlicedTextureCommandStaticState, slice_right),
            },
            FieldInfoData {
                name: "SliceBottom",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(UIDrawSlicedTextureCommandStaticState, slice_bottom),
            },
            FieldInfoData {
                name: "PaddingLeft",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(UIDrawSlicedTextureCommandStaticState, padding_left),
            },
            FieldInfoData {
                name: "PaddingTop",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(UIDrawSlicedTextureCommandStaticState, padding_top),
            },
            FieldInfoData {
                name: "PaddingRight",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(UIDrawSlicedTextureCommandStaticState, padding_right),
            },
            FieldInfoData {
                name: "PaddingBottom",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(UIDrawSlicedTextureCommandStaticState, padding_bottom),
            },
            FieldInfoData {
                name: "FillCenter",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIDrawSlicedTextureCommandStaticState, fill_center),
            },
            FieldInfoData {
                name: "UvRect",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(UIDrawSlicedTextureCommandStaticState, uv_rect),
            },
            FieldInfoData {
                name: "TextureHandle",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTURERESOURCEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(UIDrawSlicedTextureCommandStaticState, texture_handle),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(UIDrawSlicedTextureCommandStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWSLICEDTEXTURECOMMANDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIDrawSlicedTextureCommandStaticState {
    fn type_info() -> &'static TypeInfo {
        UIDRAWSLICEDTEXTURECOMMANDSTATICSTATE_TYPE_INFO
    }
}


pub const UIDRAWSLICEDTEXTURECOMMANDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawSlicedTextureCommandStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("UIDrawSlicedTextureCommandStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DynamicCastEntityData {
    pub realm: super::core::Realm,
    pub in_data: super::core::DataContainer,
    pub dynamic_output_data_type: u32,
}

pub const DYNAMICCASTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicCastEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(DynamicCastEntityData, realm),
            },
            FieldInfoData {
                name: "InData",
                flags: MemberInfoFlags::new(0),
                field_type: DATACONTAINER_TYPE_INFO,
                rust_offset: offset_of!(DynamicCastEntityData, in_data),
            },
            FieldInfoData {
                name: "DynamicOutputDataType",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicCastEntityData, dynamic_output_data_type),
            },
        ],
    }),
    array_type: Some(DYNAMICCASTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DynamicCastEntityData {
    fn type_info() -> &'static TypeInfo {
        DYNAMICCASTENTITYDATA_TYPE_INFO
    }
}


pub const DYNAMICCASTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicCastEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("DynamicCastEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ConfigEntityData {
    pub realm: super::core::Realm,
    pub asset_to_output: ConfigEntityAssetData,
    pub needed_to_create_ouputs: f32,
}

pub const CONFIGENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConfigEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(ConfigEntityData, realm),
            },
            FieldInfoData {
                name: "AssetToOutput",
                flags: MemberInfoFlags::new(0),
                field_type: CONFIGENTITYASSETDATA_TYPE_INFO,
                rust_offset: offset_of!(ConfigEntityData, asset_to_output),
            },
            FieldInfoData {
                name: "neededToCreateOuputs",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ConfigEntityData, needed_to_create_ouputs),
            },
        ],
    }),
    array_type: Some(CONFIGENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ConfigEntityData {
    fn type_info() -> &'static TypeInfo {
        CONFIGENTITYDATA_TYPE_INFO
    }
}


pub const CONFIGENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConfigEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("ConfigEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StringListAsset {
    pub string_types: Vec<StringData>,
}

pub const STRINGLISTASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StringListAsset",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CONFIGLISTASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "StringTypes",
                flags: MemberInfoFlags::new(144),
                field_type: STRINGDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(StringListAsset, string_types),
            },
        ],
    }),
    array_type: Some(STRINGLISTASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StringListAsset {
    fn type_info() -> &'static TypeInfo {
        STRINGLISTASSET_TYPE_INFO
    }
}


pub const STRINGLISTASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StringListAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("StringListAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IntListAsset {
    pub int_types: Vec<IntData>,
}

pub const INTLISTASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntListAsset",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CONFIGLISTASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "IntTypes",
                flags: MemberInfoFlags::new(144),
                field_type: INTDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(IntListAsset, int_types),
            },
        ],
    }),
    array_type: Some(INTLISTASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IntListAsset {
    fn type_info() -> &'static TypeInfo {
        INTLISTASSET_TYPE_INFO
    }
}


pub const INTLISTASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntListAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("IntListAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FloatListAsset {
    pub float_types: Vec<FloatData>,
}

pub const FLOATLISTASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatListAsset",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CONFIGLISTASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "FloatTypes",
                flags: MemberInfoFlags::new(144),
                field_type: FLOATDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(FloatListAsset, float_types),
            },
        ],
    }),
    array_type: Some(FLOATLISTASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FloatListAsset {
    fn type_info() -> &'static TypeInfo {
        FLOATLISTASSET_TYPE_INFO
    }
}


pub const FLOATLISTASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatListAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("FloatListAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ColorListAsset {
    pub color_types: Vec<ColorData>,
}

pub const COLORLISTASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ColorListAsset",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CONFIGLISTASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ColorTypes",
                flags: MemberInfoFlags::new(144),
                field_type: COLORDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ColorListAsset, color_types),
            },
        ],
    }),
    array_type: Some(COLORLISTASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ColorListAsset {
    fn type_info() -> &'static TypeInfo {
        COLORLISTASSET_TYPE_INFO
    }
}


pub const COLORLISTASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ColorListAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("ColorListAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct Vec3ListAsset {
    pub vec3_types: Vec<Vec3Data>,
}

pub const VEC3LISTASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3ListAsset",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CONFIGLISTASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Vec3Types",
                flags: MemberInfoFlags::new(144),
                field_type: VEC3DATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(Vec3ListAsset, vec3_types),
            },
        ],
    }),
    array_type: Some(VEC3LISTASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Vec3ListAsset {
    fn type_info() -> &'static TypeInfo {
        VEC3LISTASSET_TYPE_INFO
    }
}


pub const VEC3LISTASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3ListAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("Vec3ListAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ConfigListAsset {
}

pub const CONFIGLISTASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConfigListAsset",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CONFIGLISTASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ConfigListAsset {
    fn type_info() -> &'static TypeInfo {
        CONFIGLISTASSET_TYPE_INFO
    }
}


pub const CONFIGLISTASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConfigListAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("ConfigListAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ConfigEntityAssetData {
    pub data_lists: Vec<ConfigListAsset>,
}

pub const CONFIGENTITYASSETDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConfigEntityAssetData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINERPOLICYASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DataLists",
                flags: MemberInfoFlags::new(144),
                field_type: CONFIGLISTASSET_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ConfigEntityAssetData, data_lists),
            },
        ],
    }),
    array_type: Some(CONFIGENTITYASSETDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ConfigEntityAssetData {
    fn type_info() -> &'static TypeInfo {
        CONFIGENTITYASSETDATA_TYPE_INFO
    }
}


pub const CONFIGENTITYASSETDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConfigEntityAssetData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("ConfigEntityAssetData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StringData {
    pub name: String,
    pub string_value: String,
}

pub const STRINGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StringData",
    flags: MemberInfoFlags::new(73),
    module: "UIIncubatorShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(StringData, name),
            },
            FieldInfoData {
                name: "StringValue",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(StringData, string_value),
            },
        ],
    }),
    array_type: Some(STRINGDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StringData {
    fn type_info() -> &'static TypeInfo {
        STRINGDATA_TYPE_INFO
    }
}


pub const STRINGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StringData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("StringData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IntData {
    pub name: String,
    pub int_value: i32,
}

pub const INTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntData",
    flags: MemberInfoFlags::new(73),
    module: "UIIncubatorShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(IntData, name),
            },
            FieldInfoData {
                name: "IntValue",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(IntData, int_value),
            },
        ],
    }),
    array_type: Some(INTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IntData {
    fn type_info() -> &'static TypeInfo {
        INTDATA_TYPE_INFO
    }
}


pub const INTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("IntData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FloatData {
    pub name: String,
    pub float_value: f32,
}

pub const FLOATDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatData",
    flags: MemberInfoFlags::new(73),
    module: "UIIncubatorShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(FloatData, name),
            },
            FieldInfoData {
                name: "FloatValue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FloatData, float_value),
            },
        ],
    }),
    array_type: Some(FLOATDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FloatData {
    fn type_info() -> &'static TypeInfo {
        FLOATDATA_TYPE_INFO
    }
}


pub const FLOATDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("FloatData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ColorData {
    pub name: String,
    pub color_value: super::core::Vec3,
}

pub const COLORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ColorData",
    flags: MemberInfoFlags::new(73),
    module: "UIIncubatorShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ColorData, name),
            },
            FieldInfoData {
                name: "ColorValue",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ColorData, color_value),
            },
        ],
    }),
    array_type: Some(COLORDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ColorData {
    fn type_info() -> &'static TypeInfo {
        COLORDATA_TYPE_INFO
    }
}


pub const COLORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ColorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("ColorData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct Vec3Data {
    pub name: String,
    pub vec3_value: super::core::Vec3,
}

pub const VEC3DATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3Data",
    flags: MemberInfoFlags::new(73),
    module: "UIIncubatorShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(Vec3Data, name),
            },
            FieldInfoData {
                name: "Vec3Value",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(Vec3Data, vec3_value),
            },
        ],
    }),
    array_type: Some(VEC3DATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for Vec3Data {
    fn type_info() -> &'static TypeInfo {
        VEC3DATA_TYPE_INFO
    }
}


pub const VEC3DATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3Data-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("Vec3Data-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ConditionalStringEntityData {
    pub value_if_true: String,
    pub value_if_false: String,
}

pub const CONDITIONALSTRINGENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalStringEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CONDITIONALSTATEENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ValueIfTrue",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ConditionalStringEntityData, value_if_true),
            },
            FieldInfoData {
                name: "ValueIfFalse",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ConditionalStringEntityData, value_if_false),
            },
        ],
    }),
    array_type: Some(CONDITIONALSTRINGENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ConditionalStringEntityData {
    fn type_info() -> &'static TypeInfo {
        CONDITIONALSTRINGENTITYDATA_TYPE_INFO
    }
}


pub const CONDITIONALSTRINGENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalStringEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("ConditionalStringEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ConditionalTransformEntityData {
    pub value_if_true: super::core::LinearTransform,
    pub value_if_false: super::core::LinearTransform,
}

pub const CONDITIONALTRANSFORMENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalTransformEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CONDITIONALSTATEENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ValueIfTrue",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(ConditionalTransformEntityData, value_if_true),
            },
            FieldInfoData {
                name: "ValueIfFalse",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(ConditionalTransformEntityData, value_if_false),
            },
        ],
    }),
    array_type: Some(CONDITIONALTRANSFORMENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ConditionalTransformEntityData {
    fn type_info() -> &'static TypeInfo {
        CONDITIONALTRANSFORMENTITYDATA_TYPE_INFO
    }
}


pub const CONDITIONALTRANSFORMENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalTransformEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("ConditionalTransformEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ConditionalVec4EntityData {
    pub value_if_true: super::core::Vec4,
    pub value_if_false: super::core::Vec4,
}

pub const CONDITIONALVEC4ENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalVec4EntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CONDITIONALSTATEENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ValueIfTrue",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(ConditionalVec4EntityData, value_if_true),
            },
            FieldInfoData {
                name: "ValueIfFalse",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(ConditionalVec4EntityData, value_if_false),
            },
        ],
    }),
    array_type: Some(CONDITIONALVEC4ENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ConditionalVec4EntityData {
    fn type_info() -> &'static TypeInfo {
        CONDITIONALVEC4ENTITYDATA_TYPE_INFO
    }
}


pub const CONDITIONALVEC4ENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalVec4EntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("ConditionalVec4EntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ConditionalVec3EntityData {
    pub value_if_true: super::core::Vec3,
    pub value_if_false: super::core::Vec3,
}

pub const CONDITIONALVEC3ENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalVec3EntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CONDITIONALSTATEENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ValueIfTrue",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ConditionalVec3EntityData, value_if_true),
            },
            FieldInfoData {
                name: "ValueIfFalse",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ConditionalVec3EntityData, value_if_false),
            },
        ],
    }),
    array_type: Some(CONDITIONALVEC3ENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ConditionalVec3EntityData {
    fn type_info() -> &'static TypeInfo {
        CONDITIONALVEC3ENTITYDATA_TYPE_INFO
    }
}


pub const CONDITIONALVEC3ENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalVec3EntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("ConditionalVec3EntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ConditionalVec2EntityData {
    pub value_if_true: super::core::Vec2,
    pub value_if_false: super::core::Vec2,
}

pub const CONDITIONALVEC2ENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalVec2EntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CONDITIONALSTATEENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ValueIfTrue",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(ConditionalVec2EntityData, value_if_true),
            },
            FieldInfoData {
                name: "ValueIfFalse",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(ConditionalVec2EntityData, value_if_false),
            },
        ],
    }),
    array_type: Some(CONDITIONALVEC2ENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ConditionalVec2EntityData {
    fn type_info() -> &'static TypeInfo {
        CONDITIONALVEC2ENTITYDATA_TYPE_INFO
    }
}


pub const CONDITIONALVEC2ENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalVec2EntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("ConditionalVec2EntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ConditionalFloatEntityData {
    pub value_if_true: f32,
    pub value_if_false: f32,
}

pub const CONDITIONALFLOATENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalFloatEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CONDITIONALSTATEENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ValueIfTrue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ConditionalFloatEntityData, value_if_true),
            },
            FieldInfoData {
                name: "ValueIfFalse",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ConditionalFloatEntityData, value_if_false),
            },
        ],
    }),
    array_type: Some(CONDITIONALFLOATENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ConditionalFloatEntityData {
    fn type_info() -> &'static TypeInfo {
        CONDITIONALFLOATENTITYDATA_TYPE_INFO
    }
}


pub const CONDITIONALFLOATENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalFloatEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("ConditionalFloatEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ConditionalIntEntityData {
    pub value_if_true: i32,
    pub value_if_false: i32,
}

pub const CONDITIONALINTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalIntEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CONDITIONALSTATEENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ValueIfTrue",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ConditionalIntEntityData, value_if_true),
            },
            FieldInfoData {
                name: "ValueIfFalse",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ConditionalIntEntityData, value_if_false),
            },
        ],
    }),
    array_type: Some(CONDITIONALINTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ConditionalIntEntityData {
    fn type_info() -> &'static TypeInfo {
        CONDITIONALINTENTITYDATA_TYPE_INFO
    }
}


pub const CONDITIONALINTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalIntEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("ConditionalIntEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ConditionalStateEntityData {
    pub realm: super::core::Realm,
    pub condition: bool,
}

pub const CONDITIONALSTATEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalStateEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(ConditionalStateEntityData, realm),
            },
            FieldInfoData {
                name: "Condition",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ConditionalStateEntityData, condition),
            },
        ],
    }),
    array_type: Some(CONDITIONALSTATEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ConditionalStateEntityData {
    fn type_info() -> &'static TypeInfo {
        CONDITIONALSTATEENTITYDATA_TYPE_INFO
    }
}


pub const CONDITIONALSTATEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalStateEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("ConditionalStateEntityData-Array"),
    array_type: None,
    alignment: 8,
};


