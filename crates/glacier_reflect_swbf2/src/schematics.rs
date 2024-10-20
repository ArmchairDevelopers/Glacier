use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_schematics_types(registry: &mut TypeRegistry) {
    registry.register_type(SETVARIABLETYPEINFOASSET_TYPE_INFO);
    registry.register_type(SETVARIABLETYPEINFOASSET_ARRAY_TYPE_INFO);
    registry.register_type(GETVARIABLETYPEINFOASSET_TYPE_INFO);
    registry.register_type(GETVARIABLETYPEINFOASSET_ARRAY_TYPE_INFO);
    registry.register_type(SCHEMATICSUPDATEPASSASSET_TYPE_INFO);
    registry.register_type(SCHEMATICSUPDATEPASSASSET_ARRAY_TYPE_INFO);
    registry.register_type(SCHEMATICSBASEPATCHDATA_TYPE_INFO);
    registry.register_type(SCHEMATICSBASEPATCHDATA_ARRAY_TYPE_INFO);
    registry.register_type(SCHEMATICSBASEASSET_TYPE_INFO);
    registry.register_type(SCHEMATICSBASEASSET_ARRAY_TYPE_INFO);
    registry.register_type(SCHEMATICSPATCHDATA_TYPE_INFO);
    registry.register_type(SCHEMATICSPATCHDATA_ARRAY_TYPE_INFO);
    registry.register_type(SCHEMATICSOBSERVERPATCH_TYPE_INFO);
    registry.register_type(SCHEMATICSOBSERVERPATCH_ARRAY_TYPE_INFO);
    registry.register_type(SCHEMATICSNESTEDPATCH_TYPE_INFO);
    registry.register_type(SCHEMATICSNESTEDPATCH_ARRAY_TYPE_INFO);
    registry.register_type(SCHEMATICSPARAMETERPATCH_TYPE_INFO);
    registry.register_type(SCHEMATICSPARAMETERPATCH_ARRAY_TYPE_INFO);
    registry.register_type(SCHEMATICSFIELDPATCH_TYPE_INFO);
    registry.register_type(SCHEMATICSFIELDPATCH_ARRAY_TYPE_INFO);
    registry.register_type(CONSTFIELD_TYPE_INFO);
    registry.register_type(CONSTFIELD_ARRAY_TYPE_INFO);
    registry.register_type(AUTOCREATEDDISPATCHER_TYPE_INFO);
    registry.register_type(AUTOCREATEDDISPATCHER_ARRAY_TYPE_INFO);
    registry.register_type(AUTOCREATEDFIELD_TYPE_INFO);
    registry.register_type(AUTOCREATEDFIELD_ARRAY_TYPE_INFO);
    registry.register_type(EVENTOBSERVERENTRY_TYPE_INFO);
    registry.register_type(EVENTOBSERVERENTRY_ARRAY_TYPE_INFO);
    registry.register_type(SCHEMATICSASSET_TYPE_INFO);
    registry.register_type(SCHEMATICSASSET_ARRAY_TYPE_INFO);
    registry.register_type(SCHEMATICSINSTANCE_TYPE_INFO);
    registry.register_type(SCHEMATICSINSTANCE_ARRAY_TYPE_INFO);
    registry.register_type(SCHEMATICSEVENTDISPATCHER_TYPE_INFO);
    registry.register_type(SCHEMATICSEVENTDISPATCHER_ARRAY_TYPE_INFO);
    registry.register_type(SCHEMATICSCONTEXT_TYPE_INFO);
    registry.register_type(SCHEMATICSCONTEXT_ARRAY_TYPE_INFO);
    registry.register_type(SCHEMATICSPIPELINEBUILDER_TYPE_INFO);
    registry.register_type(SCHEMATICSPIPELINEBUILDER_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SetVariableTypeInfoAsset {
    pub class_type: super::core::TypeRef,
    pub field_type: super::core::TypeRef,
    pub field_offset: u32,
}

pub const SETVARIABLETYPEINFOASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SetVariableTypeInfoAsset",
    flags: MemberInfoFlags::new(101),
    module: "Schematics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(FUNCTIONTYPEINFOASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ClassType",
                flags: MemberInfoFlags::new(0),
                field_type: TYPEREF_TYPE_INFO,
                rust_offset: offset_of!(SetVariableTypeInfoAsset, class_type),
            },
            FieldInfoData {
                name: "FieldType",
                flags: MemberInfoFlags::new(0),
                field_type: TYPEREF_TYPE_INFO,
                rust_offset: offset_of!(SetVariableTypeInfoAsset, field_type),
            },
            FieldInfoData {
                name: "FieldOffset",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SetVariableTypeInfoAsset, field_offset),
            },
        ],
    }),
    array_type: Some(SETVARIABLETYPEINFOASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SetVariableTypeInfoAsset {
    fn type_info() -> &'static TypeInfo {
        SETVARIABLETYPEINFOASSET_TYPE_INFO
    }
}


pub const SETVARIABLETYPEINFOASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SetVariableTypeInfoAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SetVariableTypeInfoAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GetVariableTypeInfoAsset {
    pub class_type: super::core::TypeRef,
    pub field_type: super::core::TypeRef,
    pub field_offset: u32,
}

pub const GETVARIABLETYPEINFOASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GetVariableTypeInfoAsset",
    flags: MemberInfoFlags::new(101),
    module: "Schematics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(FUNCTIONTYPEINFOASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ClassType",
                flags: MemberInfoFlags::new(0),
                field_type: TYPEREF_TYPE_INFO,
                rust_offset: offset_of!(GetVariableTypeInfoAsset, class_type),
            },
            FieldInfoData {
                name: "FieldType",
                flags: MemberInfoFlags::new(0),
                field_type: TYPEREF_TYPE_INFO,
                rust_offset: offset_of!(GetVariableTypeInfoAsset, field_type),
            },
            FieldInfoData {
                name: "FieldOffset",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(GetVariableTypeInfoAsset, field_offset),
            },
        ],
    }),
    array_type: Some(GETVARIABLETYPEINFOASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GetVariableTypeInfoAsset {
    fn type_info() -> &'static TypeInfo {
        GETVARIABLETYPEINFOASSET_TYPE_INFO
    }
}


pub const GETVARIABLETYPEINFOASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GetVariableTypeInfoAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("GetVariableTypeInfoAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SchematicsUpdatePassAsset {
    pub depends_on: Vec<SchematicsUpdatePassAsset>,
}

pub const SCHEMATICSUPDATEPASSASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsUpdatePassAsset",
    flags: MemberInfoFlags::new(101),
    module: "Schematics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DependsOn",
                flags: MemberInfoFlags::new(144),
                field_type: SCHEMATICSUPDATEPASSASSET_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SchematicsUpdatePassAsset, depends_on),
            },
        ],
    }),
    array_type: Some(SCHEMATICSUPDATEPASSASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SchematicsUpdatePassAsset {
    fn type_info() -> &'static TypeInfo {
        SCHEMATICSUPDATEPASSASSET_TYPE_INFO
    }
}


pub const SCHEMATICSUPDATEPASSASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsUpdatePassAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SchematicsUpdatePassAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SchematicsBasePatchData {
}

pub const SCHEMATICSBASEPATCHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsBasePatchData",
    flags: MemberInfoFlags::new(101),
    module: "Schematics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SCHEMATICSBASEPATCHDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SchematicsBasePatchData {
    fn type_info() -> &'static TypeInfo {
        SCHEMATICSBASEPATCHDATA_TYPE_INFO
    }
}


pub const SCHEMATICSBASEPATCHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsBasePatchData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SchematicsBasePatchData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SchematicsBaseAsset {
}

pub const SCHEMATICSBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "Schematics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SCHEMATICSBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SchematicsBaseAsset {
    fn type_info() -> &'static TypeInfo {
        SCHEMATICSBASEASSET_TYPE_INFO
    }
}


pub const SCHEMATICSBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SchematicsBaseAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SchematicsPatchData {
    pub field_patches: Vec<SchematicsFieldPatch>,
    pub constructor_patches: Vec<SchematicsParameterPatch>,
    pub nested_patches: Vec<SchematicsNestedPatch>,
    pub observer_patches: Vec<SchematicsObserverPatch>,
}

pub const SCHEMATICSPATCHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsPatchData",
    flags: MemberInfoFlags::new(101),
    module: "Schematics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SCHEMATICSBASEPATCHDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "FieldPatches",
                flags: MemberInfoFlags::new(144),
                field_type: SCHEMATICSFIELDPATCH_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SchematicsPatchData, field_patches),
            },
            FieldInfoData {
                name: "ConstructorPatches",
                flags: MemberInfoFlags::new(144),
                field_type: SCHEMATICSPARAMETERPATCH_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SchematicsPatchData, constructor_patches),
            },
            FieldInfoData {
                name: "NestedPatches",
                flags: MemberInfoFlags::new(144),
                field_type: SCHEMATICSNESTEDPATCH_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SchematicsPatchData, nested_patches),
            },
            FieldInfoData {
                name: "ObserverPatches",
                flags: MemberInfoFlags::new(144),
                field_type: SCHEMATICSOBSERVERPATCH_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SchematicsPatchData, observer_patches),
            },
        ],
    }),
    array_type: Some(SCHEMATICSPATCHDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SchematicsPatchData {
    fn type_info() -> &'static TypeInfo {
        SCHEMATICSPATCHDATA_TYPE_INFO
    }
}


pub const SCHEMATICSPATCHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsPatchData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SchematicsPatchData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SchematicsObserverPatch {
    pub field_offset: u16,
    pub function: super::core::TypeRef,
    pub update_pass: SchematicsUpdatePassAsset,
}

pub const SCHEMATICSOBSERVERPATCH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsObserverPatch",
    flags: MemberInfoFlags::new(73),
    module: "Schematics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "FieldOffset",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(SchematicsObserverPatch, field_offset),
            },
            FieldInfoData {
                name: "Function",
                flags: MemberInfoFlags::new(0),
                field_type: TYPEREF_TYPE_INFO,
                rust_offset: offset_of!(SchematicsObserverPatch, function),
            },
            FieldInfoData {
                name: "UpdatePass",
                flags: MemberInfoFlags::new(0),
                field_type: SCHEMATICSUPDATEPASSASSET_TYPE_INFO,
                rust_offset: offset_of!(SchematicsObserverPatch, update_pass),
            },
        ],
    }),
    array_type: Some(SCHEMATICSOBSERVERPATCH_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SchematicsObserverPatch {
    fn type_info() -> &'static TypeInfo {
        SCHEMATICSOBSERVERPATCH_TYPE_INFO
    }
}


pub const SCHEMATICSOBSERVERPATCH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsObserverPatch-Array",
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SchematicsObserverPatch-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SchematicsNestedPatch {
    pub target_offset: u16,
    pub data: SchematicsPatchData,
}

pub const SCHEMATICSNESTEDPATCH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsNestedPatch",
    flags: MemberInfoFlags::new(73),
    module: "Schematics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "TargetOffset",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(SchematicsNestedPatch, target_offset),
            },
            FieldInfoData {
                name: "Data",
                flags: MemberInfoFlags::new(0),
                field_type: SCHEMATICSPATCHDATA_TYPE_INFO,
                rust_offset: offset_of!(SchematicsNestedPatch, data),
            },
        ],
    }),
    array_type: Some(SCHEMATICSNESTEDPATCH_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SchematicsNestedPatch {
    fn type_info() -> &'static TypeInfo {
        SCHEMATICSNESTEDPATCH_TYPE_INFO
    }
}


pub const SCHEMATICSNESTEDPATCH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsNestedPatch-Array",
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SchematicsNestedPatch-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SchematicsParameterPatch {
    pub value: super::core::BoxedValueRef,
    pub parameter_index: u8,
    pub target_offset: u16,
}

pub const SCHEMATICSPARAMETERPATCH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsParameterPatch",
    flags: MemberInfoFlags::new(73),
    module: "Schematics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: BOXEDVALUEREF_TYPE_INFO,
                rust_offset: offset_of!(SchematicsParameterPatch, value),
            },
            FieldInfoData {
                name: "ParameterIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(SchematicsParameterPatch, parameter_index),
            },
            FieldInfoData {
                name: "TargetOffset",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(SchematicsParameterPatch, target_offset),
            },
        ],
    }),
    array_type: Some(SCHEMATICSPARAMETERPATCH_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SchematicsParameterPatch {
    fn type_info() -> &'static TypeInfo {
        SCHEMATICSPARAMETERPATCH_TYPE_INFO
    }
}


pub const SCHEMATICSPARAMETERPATCH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsParameterPatch-Array",
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SchematicsParameterPatch-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SchematicsFieldPatch {
    pub value: super::core::BoxedValueRef,
    pub target_offset: u16,
}

pub const SCHEMATICSFIELDPATCH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsFieldPatch",
    flags: MemberInfoFlags::new(73),
    module: "Schematics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: BOXEDVALUEREF_TYPE_INFO,
                rust_offset: offset_of!(SchematicsFieldPatch, value),
            },
            FieldInfoData {
                name: "TargetOffset",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(SchematicsFieldPatch, target_offset),
            },
        ],
    }),
    array_type: Some(SCHEMATICSFIELDPATCH_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SchematicsFieldPatch {
    fn type_info() -> &'static TypeInfo {
        SCHEMATICSFIELDPATCH_TYPE_INFO
    }
}


pub const SCHEMATICSFIELDPATCH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsFieldPatch-Array",
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SchematicsFieldPatch-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ConstField {
    pub value: super::core::BoxedValueRef,
}

pub const CONSTFIELD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConstField",
    flags: MemberInfoFlags::new(73),
    module: "Schematics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: BOXEDVALUEREF_TYPE_INFO,
                rust_offset: offset_of!(ConstField, value),
            },
        ],
    }),
    array_type: Some(CONSTFIELD_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ConstField {
    fn type_info() -> &'static TypeInfo {
        CONSTFIELD_TYPE_INFO
    }
}


pub const CONSTFIELD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConstField-Array",
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("ConstField-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AutoCreatedDispatcher {
    pub field_offset: u16,
    pub parameter_type: super::core::TypeRef,
}

pub const AUTOCREATEDDISPATCHER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoCreatedDispatcher",
    flags: MemberInfoFlags::new(73),
    module: "Schematics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "FieldOffset",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(AutoCreatedDispatcher, field_offset),
            },
            FieldInfoData {
                name: "ParameterType",
                flags: MemberInfoFlags::new(0),
                field_type: TYPEREF_TYPE_INFO,
                rust_offset: offset_of!(AutoCreatedDispatcher, parameter_type),
            },
        ],
    }),
    array_type: Some(AUTOCREATEDDISPATCHER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AutoCreatedDispatcher {
    fn type_info() -> &'static TypeInfo {
        AUTOCREATEDDISPATCHER_TYPE_INFO
    }
}


pub const AUTOCREATEDDISPATCHER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoCreatedDispatcher-Array",
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("AutoCreatedDispatcher-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AutoCreatedField {
    pub field_offset: u16,
    pub asset: SchematicsAsset,
    pub patch_data: SchematicsPatchData,
}

pub const AUTOCREATEDFIELD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoCreatedField",
    flags: MemberInfoFlags::new(73),
    module: "Schematics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "FieldOffset",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(AutoCreatedField, field_offset),
            },
            FieldInfoData {
                name: "Asset",
                flags: MemberInfoFlags::new(0),
                field_type: SCHEMATICSASSET_TYPE_INFO,
                rust_offset: offset_of!(AutoCreatedField, asset),
            },
            FieldInfoData {
                name: "PatchData",
                flags: MemberInfoFlags::new(0),
                field_type: SCHEMATICSPATCHDATA_TYPE_INFO,
                rust_offset: offset_of!(AutoCreatedField, patch_data),
            },
        ],
    }),
    array_type: Some(AUTOCREATEDFIELD_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AutoCreatedField {
    fn type_info() -> &'static TypeInfo {
        AUTOCREATEDFIELD_TYPE_INFO
    }
}


pub const AUTOCREATEDFIELD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoCreatedField-Array",
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("AutoCreatedField-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EventObserverEntry {
    pub function: super::core::TypeRef,
    pub update_pass: SchematicsUpdatePassAsset,
}

pub const EVENTOBSERVERENTRY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventObserverEntry",
    flags: MemberInfoFlags::new(73),
    module: "Schematics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Function",
                flags: MemberInfoFlags::new(0),
                field_type: TYPEREF_TYPE_INFO,
                rust_offset: offset_of!(EventObserverEntry, function),
            },
            FieldInfoData {
                name: "UpdatePass",
                flags: MemberInfoFlags::new(0),
                field_type: SCHEMATICSUPDATEPASSASSET_TYPE_INFO,
                rust_offset: offset_of!(EventObserverEntry, update_pass),
            },
        ],
    }),
    array_type: Some(EVENTOBSERVERENTRY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EventObserverEntry {
    fn type_info() -> &'static TypeInfo {
        EVENTOBSERVERENTRY_TYPE_INFO
    }
}


pub const EVENTOBSERVERENTRY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventObserverEntry-Array",
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("EventObserverEntry-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SchematicsAsset {
    pub instance_type: super::core::TypeRef,
    pub constructor_function: super::core::TypeRef,
    pub destructor_function: super::core::TypeRef,
    pub tweaker_function: super::core::TypeRef,
    pub auto_created_fields: Vec<AutoCreatedField>,
    pub auto_created_dispatchers: Vec<AutoCreatedDispatcher>,
    pub patch_data: SchematicsPatchData,
    pub observers: Vec<EventObserverEntry>,
    pub const_fields: Vec<ConstField>,
}

pub const SCHEMATICSASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsAsset",
    flags: MemberInfoFlags::new(101),
    module: "Schematics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SCHEMATICSBASEASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "InstanceType",
                flags: MemberInfoFlags::new(0),
                field_type: TYPEREF_TYPE_INFO,
                rust_offset: offset_of!(SchematicsAsset, instance_type),
            },
            FieldInfoData {
                name: "ConstructorFunction",
                flags: MemberInfoFlags::new(0),
                field_type: TYPEREF_TYPE_INFO,
                rust_offset: offset_of!(SchematicsAsset, constructor_function),
            },
            FieldInfoData {
                name: "DestructorFunction",
                flags: MemberInfoFlags::new(0),
                field_type: TYPEREF_TYPE_INFO,
                rust_offset: offset_of!(SchematicsAsset, destructor_function),
            },
            FieldInfoData {
                name: "TweakerFunction",
                flags: MemberInfoFlags::new(0),
                field_type: TYPEREF_TYPE_INFO,
                rust_offset: offset_of!(SchematicsAsset, tweaker_function),
            },
            FieldInfoData {
                name: "AutoCreatedFields",
                flags: MemberInfoFlags::new(144),
                field_type: AUTOCREATEDFIELD_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SchematicsAsset, auto_created_fields),
            },
            FieldInfoData {
                name: "AutoCreatedDispatchers",
                flags: MemberInfoFlags::new(144),
                field_type: AUTOCREATEDDISPATCHER_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SchematicsAsset, auto_created_dispatchers),
            },
            FieldInfoData {
                name: "PatchData",
                flags: MemberInfoFlags::new(0),
                field_type: SCHEMATICSPATCHDATA_TYPE_INFO,
                rust_offset: offset_of!(SchematicsAsset, patch_data),
            },
            FieldInfoData {
                name: "Observers",
                flags: MemberInfoFlags::new(144),
                field_type: EVENTOBSERVERENTRY_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SchematicsAsset, observers),
            },
            FieldInfoData {
                name: "ConstFields",
                flags: MemberInfoFlags::new(144),
                field_type: CONSTFIELD_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SchematicsAsset, const_fields),
            },
        ],
    }),
    array_type: Some(SCHEMATICSASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SchematicsAsset {
    fn type_info() -> &'static TypeInfo {
        SCHEMATICSASSET_TYPE_INFO
    }
}


pub const SCHEMATICSASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SchematicsAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SchematicsInstance {
}

pub const SCHEMATICSINSTANCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsInstance",
    flags: MemberInfoFlags::new(101),
    module: "Schematics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(SCHEMATICSINSTANCE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SchematicsInstance {
    fn type_info() -> &'static TypeInfo {
        SCHEMATICSINSTANCE_TYPE_INFO
    }
}


pub const SCHEMATICSINSTANCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsInstance-Array",
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SchematicsInstance-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SchematicsEventDispatcher {
}

pub const SCHEMATICSEVENTDISPATCHER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsEventDispatcher",
    flags: MemberInfoFlags::new(101),
    module: "Schematics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVENTDISPATCHER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SCHEMATICSEVENTDISPATCHER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SchematicsEventDispatcher {
    fn type_info() -> &'static TypeInfo {
        SCHEMATICSEVENTDISPATCHER_TYPE_INFO
    }
}


pub const SCHEMATICSEVENTDISPATCHER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsEventDispatcher-Array",
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SchematicsEventDispatcher-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SchematicsContext {
}

pub const SCHEMATICSCONTEXT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsContext",
    flags: MemberInfoFlags::new(101),
    module: "Schematics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(SCHEMATICSCONTEXT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SchematicsContext {
    fn type_info() -> &'static TypeInfo {
        SCHEMATICSCONTEXT_TYPE_INFO
    }
}


pub const SCHEMATICSCONTEXT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsContext-Array",
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SchematicsContext-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SchematicsPipelineBuilder {
}

pub const SCHEMATICSPIPELINEBUILDER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsPipelineBuilder",
    flags: MemberInfoFlags::new(101),
    module: "Schematics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(SCHEMATICSPIPELINEBUILDER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SchematicsPipelineBuilder {
    fn type_info() -> &'static TypeInfo {
        SCHEMATICSPIPELINEBUILDER_TYPE_INFO
    }
}


pub const SCHEMATICSPIPELINEBUILDER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsPipelineBuilder-Array",
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SchematicsPipelineBuilder-Array"),
    array_type: None,
    alignment: 8,
};


