use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_quickscope_shared_types(registry: &mut TypeRegistry) {
    registry.register_type(QUICKSCOPECONTROLENTITYDATA_TYPE_INFO);
    registry.register_type(QUICKSCOPECONTROLENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(QUICKSCOPETEST_TYPE_INFO);
    registry.register_type(QUICKSCOPETEST_ARRAY_TYPE_INFO);
    registry.register_type(QUICKSCOPEBUDGETSASSET_TYPE_INFO);
    registry.register_type(QUICKSCOPEBUDGETSASSET_ARRAY_TYPE_INFO);
    registry.register_type(QUICKSCOPEBUDGETENTRY_TYPE_INFO);
    registry.register_type(QUICKSCOPEBUDGETENTRY_ARRAY_TYPE_INFO);
    registry.register_type(QUICKSCOPEPLATFORMVALUE_TYPE_INFO);
    registry.register_type(QUICKSCOPEPLATFORMVALUE_ARRAY_TYPE_INFO);
    registry.register_type(QUICKSCOPEPLATFORM_TYPE_INFO);
    registry.register_type(QUICKSCOPEPLATFORM_ARRAY_TYPE_INFO);
    registry.register_type(QUICKSCOPECATEGORIESASSET_TYPE_INFO);
    registry.register_type(QUICKSCOPECATEGORIESASSET_ARRAY_TYPE_INFO);
    registry.register_type(QUICKSCOPECATEGORY_TYPE_INFO);
    registry.register_type(QUICKSCOPECATEGORY_ARRAY_TYPE_INFO);
    registry.register_type(QUICKSCOPELEVELDATA_TYPE_INFO);
    registry.register_type(QUICKSCOPELEVELDATA_ARRAY_TYPE_INFO);
    registry.register_type(QUICKSCOPEPROCESSORTYPE_TYPE_INFO);
    registry.register_type(QUICKSCOPEPROCESSORTYPE_ARRAY_TYPE_INFO);
    registry.register_type(QUICKSCOPEFRAMETYPE_TYPE_INFO);
    registry.register_type(QUICKSCOPEFRAMETYPE_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct QuickscopeControlEntityData {
    pub realm: super::core::Realm,
    pub label: String,
}

pub const QUICKSCOPECONTROLENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeControlEntityData",
    flags: MemberInfoFlags::new(101),
    module: "QuickscopeShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(QuickscopeControlEntityData, realm),
            },
            FieldInfoData {
                name: "Label",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(QuickscopeControlEntityData, label),
            },
        ],
    }),
    array_type: Some(QUICKSCOPECONTROLENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for QuickscopeControlEntityData {
    fn type_info() -> &'static TypeInfo {
        QUICKSCOPECONTROLENTITYDATA_TYPE_INFO
    }
}


pub const QUICKSCOPECONTROLENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeControlEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "QuickscopeShared",
    data: TypeInfoData::Array("QuickscopeControlEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct QuickscopeTest {
    pub number: i32,
}

pub const QUICKSCOPETEST_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeTest",
    flags: MemberInfoFlags::new(101),
    module: "QuickscopeShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Number",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(QuickscopeTest, number),
            },
        ],
    }),
    array_type: Some(QUICKSCOPETEST_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for QuickscopeTest {
    fn type_info() -> &'static TypeInfo {
        QUICKSCOPETEST_TYPE_INFO
    }
}


pub const QUICKSCOPETEST_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeTest-Array",
    flags: MemberInfoFlags::new(145),
    module: "QuickscopeShared",
    data: TypeInfoData::Array("QuickscopeTest-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct QuickscopeBudgetsAsset {
    pub entries: Vec<QuickscopeBudgetEntry>,
    pub resolution_targets: Vec<QuickscopePlatformValue>,
}

pub const QUICKSCOPEBUDGETSASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeBudgetsAsset",
    flags: MemberInfoFlags::new(101),
    module: "QuickscopeShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Entries",
                flags: MemberInfoFlags::new(144),
                field_type: QUICKSCOPEBUDGETENTRY_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(QuickscopeBudgetsAsset, entries),
            },
            FieldInfoData {
                name: "ResolutionTargets",
                flags: MemberInfoFlags::new(144),
                field_type: QUICKSCOPEPLATFORMVALUE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(QuickscopeBudgetsAsset, resolution_targets),
            },
        ],
    }),
    array_type: Some(QUICKSCOPEBUDGETSASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for QuickscopeBudgetsAsset {
    fn type_info() -> &'static TypeInfo {
        QUICKSCOPEBUDGETSASSET_TYPE_INFO
    }
}


pub const QUICKSCOPEBUDGETSASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeBudgetsAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "QuickscopeShared",
    data: TypeInfoData::Array("QuickscopeBudgetsAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct QuickscopeBudgetEntry {
    pub category_name: String,
    pub budgets: Vec<QuickscopePlatformValue>,
}

pub const QUICKSCOPEBUDGETENTRY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeBudgetEntry",
    flags: MemberInfoFlags::new(73),
    module: "QuickscopeShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "CategoryName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(QuickscopeBudgetEntry, category_name),
            },
            FieldInfoData {
                name: "Budgets",
                flags: MemberInfoFlags::new(144),
                field_type: QUICKSCOPEPLATFORMVALUE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(QuickscopeBudgetEntry, budgets),
            },
        ],
    }),
    array_type: Some(QUICKSCOPEBUDGETENTRY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for QuickscopeBudgetEntry {
    fn type_info() -> &'static TypeInfo {
        QUICKSCOPEBUDGETENTRY_TYPE_INFO
    }
}


pub const QUICKSCOPEBUDGETENTRY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeBudgetEntry-Array",
    flags: MemberInfoFlags::new(145),
    module: "QuickscopeShared",
    data: TypeInfoData::Array("QuickscopeBudgetEntry-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct QuickscopePlatformValue {
    pub platform: QuickscopePlatform,
    pub value: f32,
}

pub const QUICKSCOPEPLATFORMVALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopePlatformValue",
    flags: MemberInfoFlags::new(36937),
    module: "QuickscopeShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Platform",
                flags: MemberInfoFlags::new(0),
                field_type: QUICKSCOPEPLATFORM_TYPE_INFO,
                rust_offset: offset_of!(QuickscopePlatformValue, platform),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(QuickscopePlatformValue, value),
            },
        ],
    }),
    array_type: Some(QUICKSCOPEPLATFORMVALUE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for QuickscopePlatformValue {
    fn type_info() -> &'static TypeInfo {
        QUICKSCOPEPLATFORMVALUE_TYPE_INFO
    }
}


pub const QUICKSCOPEPLATFORMVALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopePlatformValue-Array",
    flags: MemberInfoFlags::new(145),
    module: "QuickscopeShared",
    data: TypeInfoData::Array("QuickscopePlatformValue-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum QuickscopePlatform {
    #[default]
    QuickscopePlatform_Win64 = 0,
    QuickscopePlatform_Orbis = 1,
    QuickscopePlatform_Durango = 2,
    QuickscopePlatform_Neo = 3,
    QuickscopePlatform_Scorpio = 4,
    QuickscopePlatform_Win64Dx12 = 5,
    QuickscopePlatform_Unknown = 6,
}

pub const QUICKSCOPEPLATFORM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopePlatform",
    flags: MemberInfoFlags::new(49429),
    module: "QuickscopeShared",
    data: TypeInfoData::Enum,
    array_type: Some(QUICKSCOPEPLATFORM_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for QuickscopePlatform {
    fn type_info() -> &'static TypeInfo {
        QUICKSCOPEPLATFORM_TYPE_INFO
    }
}


pub const QUICKSCOPEPLATFORM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopePlatform-Array",
    flags: MemberInfoFlags::new(145),
    module: "QuickscopeShared",
    data: TypeInfoData::Array("QuickscopePlatform-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct QuickscopeCategoriesAsset {
    pub categories: Vec<QuickscopeCategory>,
}

pub const QUICKSCOPECATEGORIESASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeCategoriesAsset",
    flags: MemberInfoFlags::new(101),
    module: "QuickscopeShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Categories",
                flags: MemberInfoFlags::new(144),
                field_type: QUICKSCOPECATEGORY_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(QuickscopeCategoriesAsset, categories),
            },
        ],
    }),
    array_type: Some(QUICKSCOPECATEGORIESASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for QuickscopeCategoriesAsset {
    fn type_info() -> &'static TypeInfo {
        QUICKSCOPECATEGORIESASSET_TYPE_INFO
    }
}


pub const QUICKSCOPECATEGORIESASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeCategoriesAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "QuickscopeShared",
    data: TypeInfoData::Array("QuickscopeCategoriesAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct QuickscopeCategory {
    pub name: String,
    pub frame: QuickscopeFrameType,
    pub processor: QuickscopeProcessorType,
    pub include_scope: Vec<String>,
    pub exclude_scope: Vec<String>,
}

pub const QUICKSCOPECATEGORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeCategory",
    flags: MemberInfoFlags::new(73),
    module: "QuickscopeShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(QuickscopeCategory, name),
            },
            FieldInfoData {
                name: "Frame",
                flags: MemberInfoFlags::new(0),
                field_type: QUICKSCOPEFRAMETYPE_TYPE_INFO,
                rust_offset: offset_of!(QuickscopeCategory, frame),
            },
            FieldInfoData {
                name: "Processor",
                flags: MemberInfoFlags::new(0),
                field_type: QUICKSCOPEPROCESSORTYPE_TYPE_INFO,
                rust_offset: offset_of!(QuickscopeCategory, processor),
            },
            FieldInfoData {
                name: "IncludeScope",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(QuickscopeCategory, include_scope),
            },
            FieldInfoData {
                name: "ExcludeScope",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(QuickscopeCategory, exclude_scope),
            },
        ],
    }),
    array_type: Some(QUICKSCOPECATEGORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for QuickscopeCategory {
    fn type_info() -> &'static TypeInfo {
        QUICKSCOPECATEGORY_TYPE_INFO
    }
}


pub const QUICKSCOPECATEGORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeCategory-Array",
    flags: MemberInfoFlags::new(145),
    module: "QuickscopeShared",
    data: TypeInfoData::Array("QuickscopeCategory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct QuickscopeLevelData {
    pub categories: QuickscopeCategoriesAsset,
    pub budgets: QuickscopeBudgetsAsset,
}

pub const QUICKSCOPELEVELDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeLevelData",
    flags: MemberInfoFlags::new(101),
    module: "QuickscopeShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SUBWORLDDATACOMPONENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Categories",
                flags: MemberInfoFlags::new(0),
                field_type: QUICKSCOPECATEGORIESASSET_TYPE_INFO,
                rust_offset: offset_of!(QuickscopeLevelData, categories),
            },
            FieldInfoData {
                name: "Budgets",
                flags: MemberInfoFlags::new(0),
                field_type: QUICKSCOPEBUDGETSASSET_TYPE_INFO,
                rust_offset: offset_of!(QuickscopeLevelData, budgets),
            },
        ],
    }),
    array_type: Some(QUICKSCOPELEVELDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for QuickscopeLevelData {
    fn type_info() -> &'static TypeInfo {
        QUICKSCOPELEVELDATA_TYPE_INFO
    }
}


pub const QUICKSCOPELEVELDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeLevelData-Array",
    flags: MemberInfoFlags::new(145),
    module: "QuickscopeShared",
    data: TypeInfoData::Array("QuickscopeLevelData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum QuickscopeProcessorType {
    #[default]
    QuickscopeProcessorType_CPU = 0,
    QuickscopeProcessorType_Compute = 1,
    QuickscopeProcessorType_GPU = 2,
    QuickscopeProcessorType_Copy = 3,
}

pub const QUICKSCOPEPROCESSORTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeProcessorType",
    flags: MemberInfoFlags::new(49429),
    module: "QuickscopeShared",
    data: TypeInfoData::Enum,
    array_type: Some(QUICKSCOPEPROCESSORTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for QuickscopeProcessorType {
    fn type_info() -> &'static TypeInfo {
        QUICKSCOPEPROCESSORTYPE_TYPE_INFO
    }
}


pub const QUICKSCOPEPROCESSORTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeProcessorType-Array",
    flags: MemberInfoFlags::new(145),
    module: "QuickscopeShared",
    data: TypeInfoData::Array("QuickscopeProcessorType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum QuickscopeFrameType {
    #[default]
    QuickscopeFrameType_Main = 0,
    QuickscopeFrameType_ClientSim = 1,
    QuickscopeFrameType_Render = 2,
    QuickscopeFrameType_Audio = 3,
}

pub const QUICKSCOPEFRAMETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeFrameType",
    flags: MemberInfoFlags::new(49429),
    module: "QuickscopeShared",
    data: TypeInfoData::Enum,
    array_type: Some(QUICKSCOPEFRAMETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for QuickscopeFrameType {
    fn type_info() -> &'static TypeInfo {
        QUICKSCOPEFRAMETYPE_TYPE_INFO
    }
}


pub const QUICKSCOPEFRAMETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeFrameType-Array",
    flags: MemberInfoFlags::new(145),
    module: "QuickscopeShared",
    data: TypeInfoData::Array("QuickscopeFrameType-Array"),
    array_type: None,
    alignment: 8,
};


