use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct QuickscopeControlEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub label: String,
}

pub trait QuickscopeControlEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn label(&self) -> &String;
}

impl QuickscopeControlEntityDataTrait for QuickscopeControlEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn label(&self) -> &String {
        &self.label
    }
}

impl super::entity::EntityDataTrait for QuickscopeControlEntityData {
}

impl super::entity::GameObjectDataTrait for QuickscopeControlEntityData {
}

impl super::core::DataBusPeerTrait for QuickscopeControlEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for QuickscopeControlEntityData {
}

impl super::core::DataContainerTrait for QuickscopeControlEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static QUICKSCOPECONTROLENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeControlEntityData",
    flags: MemberInfoFlags::new(101),
    module: "QuickscopeShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<QuickscopeControlEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(QuickscopeControlEntityData, realm),
            },
            FieldInfoData {
                name: "Label",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(QuickscopeControlEntityData, label),
            },
        ],
    }),
    array_type: Some(QUICKSCOPECONTROLENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for QuickscopeControlEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        QUICKSCOPECONTROLENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static QUICKSCOPECONTROLENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeControlEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "QuickscopeShared",
    data: TypeInfoData::Array("QuickscopeControlEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct QuickscopeTest {
    pub _glacier_base: super::core::Asset,
    pub number: i32,
}

pub trait QuickscopeTestTrait: super::core::AssetTrait {
    fn number(&self) -> &i32;
}

impl QuickscopeTestTrait for QuickscopeTest {
    fn number(&self) -> &i32 {
        &self.number
    }
}

impl super::core::AssetTrait for QuickscopeTest {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for QuickscopeTest {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static QUICKSCOPETEST_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeTest",
    flags: MemberInfoFlags::new(101),
    module: "QuickscopeShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<QuickscopeTest as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Number",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(QuickscopeTest, number),
            },
        ],
    }),
    array_type: Some(QUICKSCOPETEST_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for QuickscopeTest {
    fn type_info(&self) -> &'static TypeInfo {
        QUICKSCOPETEST_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static QUICKSCOPETEST_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeTest-Array",
    flags: MemberInfoFlags::new(145),
    module: "QuickscopeShared",
    data: TypeInfoData::Array("QuickscopeTest"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct QuickscopeBudgetsAsset {
    pub _glacier_base: super::core::Asset,
    pub entries: Vec<QuickscopeBudgetEntry>,
    pub resolution_targets: Vec<QuickscopePlatformValue>,
}

pub trait QuickscopeBudgetsAssetTrait: super::core::AssetTrait {
    fn entries(&self) -> &Vec<QuickscopeBudgetEntry>;
    fn resolution_targets(&self) -> &Vec<QuickscopePlatformValue>;
}

impl QuickscopeBudgetsAssetTrait for QuickscopeBudgetsAsset {
    fn entries(&self) -> &Vec<QuickscopeBudgetEntry> {
        &self.entries
    }
    fn resolution_targets(&self) -> &Vec<QuickscopePlatformValue> {
        &self.resolution_targets
    }
}

impl super::core::AssetTrait for QuickscopeBudgetsAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for QuickscopeBudgetsAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static QUICKSCOPEBUDGETSASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeBudgetsAsset",
    flags: MemberInfoFlags::new(101),
    module: "QuickscopeShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<QuickscopeBudgetsAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Entries",
                flags: MemberInfoFlags::new(144),
                field_type: "QuickscopeBudgetEntry-Array",
                rust_offset: offset_of!(QuickscopeBudgetsAsset, entries),
            },
            FieldInfoData {
                name: "ResolutionTargets",
                flags: MemberInfoFlags::new(144),
                field_type: "QuickscopePlatformValue-Array",
                rust_offset: offset_of!(QuickscopeBudgetsAsset, resolution_targets),
            },
        ],
    }),
    array_type: Some(QUICKSCOPEBUDGETSASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for QuickscopeBudgetsAsset {
    fn type_info(&self) -> &'static TypeInfo {
        QUICKSCOPEBUDGETSASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static QUICKSCOPEBUDGETSASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeBudgetsAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "QuickscopeShared",
    data: TypeInfoData::Array("QuickscopeBudgetsAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct QuickscopeBudgetEntry {
    pub category_name: String,
    pub budgets: Vec<QuickscopePlatformValue>,
}

pub trait QuickscopeBudgetEntryTrait: TypeObject {
    fn category_name(&self) -> &String;
    fn budgets(&self) -> &Vec<QuickscopePlatformValue>;
}

impl QuickscopeBudgetEntryTrait for QuickscopeBudgetEntry {
    fn category_name(&self) -> &String {
        &self.category_name
    }
    fn budgets(&self) -> &Vec<QuickscopePlatformValue> {
        &self.budgets
    }
}

pub static QUICKSCOPEBUDGETENTRY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeBudgetEntry",
    flags: MemberInfoFlags::new(73),
    module: "QuickscopeShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<QuickscopeBudgetEntry as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "CategoryName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(QuickscopeBudgetEntry, category_name),
            },
            FieldInfoData {
                name: "Budgets",
                flags: MemberInfoFlags::new(144),
                field_type: "QuickscopePlatformValue-Array",
                rust_offset: offset_of!(QuickscopeBudgetEntry, budgets),
            },
        ],
    }),
    array_type: Some(QUICKSCOPEBUDGETENTRY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for QuickscopeBudgetEntry {
    fn type_info(&self) -> &'static TypeInfo {
        QUICKSCOPEBUDGETENTRY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static QUICKSCOPEBUDGETENTRY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeBudgetEntry-Array",
    flags: MemberInfoFlags::new(145),
    module: "QuickscopeShared",
    data: TypeInfoData::Array("QuickscopeBudgetEntry"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct QuickscopePlatformValue {
    pub platform: QuickscopePlatform,
    pub value: f32,
}

pub trait QuickscopePlatformValueTrait: TypeObject {
    fn platform(&self) -> &QuickscopePlatform;
    fn value(&self) -> &f32;
}

impl QuickscopePlatformValueTrait for QuickscopePlatformValue {
    fn platform(&self) -> &QuickscopePlatform {
        &self.platform
    }
    fn value(&self) -> &f32 {
        &self.value
    }
}

pub static QUICKSCOPEPLATFORMVALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopePlatformValue",
    flags: MemberInfoFlags::new(36937),
    module: "QuickscopeShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<QuickscopePlatformValue as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Platform",
                flags: MemberInfoFlags::new(0),
                field_type: "QuickscopePlatform",
                rust_offset: offset_of!(QuickscopePlatformValue, platform),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(QuickscopePlatformValue, value),
            },
        ],
    }),
    array_type: Some(QUICKSCOPEPLATFORMVALUE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for QuickscopePlatformValue {
    fn type_info(&self) -> &'static TypeInfo {
        QUICKSCOPEPLATFORMVALUE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static QUICKSCOPEPLATFORMVALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopePlatformValue-Array",
    flags: MemberInfoFlags::new(145),
    module: "QuickscopeShared",
    data: TypeInfoData::Array("QuickscopePlatformValue"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static QUICKSCOPEPLATFORM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopePlatform",
    flags: MemberInfoFlags::new(49429),
    module: "QuickscopeShared",
    data: TypeInfoData::Enum,
    array_type: Some(QUICKSCOPEPLATFORM_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for QuickscopePlatform {
    fn type_info(&self) -> &'static TypeInfo {
        QUICKSCOPEPLATFORM_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static QUICKSCOPEPLATFORM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopePlatform-Array",
    flags: MemberInfoFlags::new(145),
    module: "QuickscopeShared",
    data: TypeInfoData::Array("QuickscopePlatform"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct QuickscopeCategoriesAsset {
    pub _glacier_base: super::core::Asset,
    pub categories: Vec<QuickscopeCategory>,
}

pub trait QuickscopeCategoriesAssetTrait: super::core::AssetTrait {
    fn categories(&self) -> &Vec<QuickscopeCategory>;
}

impl QuickscopeCategoriesAssetTrait for QuickscopeCategoriesAsset {
    fn categories(&self) -> &Vec<QuickscopeCategory> {
        &self.categories
    }
}

impl super::core::AssetTrait for QuickscopeCategoriesAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for QuickscopeCategoriesAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static QUICKSCOPECATEGORIESASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeCategoriesAsset",
    flags: MemberInfoFlags::new(101),
    module: "QuickscopeShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<QuickscopeCategoriesAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Categories",
                flags: MemberInfoFlags::new(144),
                field_type: "QuickscopeCategory-Array",
                rust_offset: offset_of!(QuickscopeCategoriesAsset, categories),
            },
        ],
    }),
    array_type: Some(QUICKSCOPECATEGORIESASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for QuickscopeCategoriesAsset {
    fn type_info(&self) -> &'static TypeInfo {
        QUICKSCOPECATEGORIESASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static QUICKSCOPECATEGORIESASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeCategoriesAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "QuickscopeShared",
    data: TypeInfoData::Array("QuickscopeCategoriesAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct QuickscopeCategory {
    pub name: String,
    pub frame: QuickscopeFrameType,
    pub processor: QuickscopeProcessorType,
    pub include_scope: Vec<String>,
    pub exclude_scope: Vec<String>,
}

pub trait QuickscopeCategoryTrait: TypeObject {
    fn name(&self) -> &String;
    fn frame(&self) -> &QuickscopeFrameType;
    fn processor(&self) -> &QuickscopeProcessorType;
    fn include_scope(&self) -> &Vec<String>;
    fn exclude_scope(&self) -> &Vec<String>;
}

impl QuickscopeCategoryTrait for QuickscopeCategory {
    fn name(&self) -> &String {
        &self.name
    }
    fn frame(&self) -> &QuickscopeFrameType {
        &self.frame
    }
    fn processor(&self) -> &QuickscopeProcessorType {
        &self.processor
    }
    fn include_scope(&self) -> &Vec<String> {
        &self.include_scope
    }
    fn exclude_scope(&self) -> &Vec<String> {
        &self.exclude_scope
    }
}

pub static QUICKSCOPECATEGORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeCategory",
    flags: MemberInfoFlags::new(73),
    module: "QuickscopeShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<QuickscopeCategory as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(QuickscopeCategory, name),
            },
            FieldInfoData {
                name: "Frame",
                flags: MemberInfoFlags::new(0),
                field_type: "QuickscopeFrameType",
                rust_offset: offset_of!(QuickscopeCategory, frame),
            },
            FieldInfoData {
                name: "Processor",
                flags: MemberInfoFlags::new(0),
                field_type: "QuickscopeProcessorType",
                rust_offset: offset_of!(QuickscopeCategory, processor),
            },
            FieldInfoData {
                name: "IncludeScope",
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(QuickscopeCategory, include_scope),
            },
            FieldInfoData {
                name: "ExcludeScope",
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(QuickscopeCategory, exclude_scope),
            },
        ],
    }),
    array_type: Some(QUICKSCOPECATEGORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for QuickscopeCategory {
    fn type_info(&self) -> &'static TypeInfo {
        QUICKSCOPECATEGORY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static QUICKSCOPECATEGORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeCategory-Array",
    flags: MemberInfoFlags::new(145),
    module: "QuickscopeShared",
    data: TypeInfoData::Array("QuickscopeCategory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct QuickscopeLevelData {
    pub _glacier_base: super::entity::SubWorldDataComponent,
    pub categories: Option<Arc<Mutex<dyn QuickscopeCategoriesAssetTrait>>>,
    pub budgets: Option<Arc<Mutex<dyn QuickscopeBudgetsAssetTrait>>>,
}

pub trait QuickscopeLevelDataTrait: super::entity::SubWorldDataComponentTrait {
    fn categories(&self) -> &Option<Arc<Mutex<dyn QuickscopeCategoriesAssetTrait>>>;
    fn budgets(&self) -> &Option<Arc<Mutex<dyn QuickscopeBudgetsAssetTrait>>>;
}

impl QuickscopeLevelDataTrait for QuickscopeLevelData {
    fn categories(&self) -> &Option<Arc<Mutex<dyn QuickscopeCategoriesAssetTrait>>> {
        &self.categories
    }
    fn budgets(&self) -> &Option<Arc<Mutex<dyn QuickscopeBudgetsAssetTrait>>> {
        &self.budgets
    }
}

impl super::entity::SubWorldDataComponentTrait for QuickscopeLevelData {
}

impl super::core::DataContainerTrait for QuickscopeLevelData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static QUICKSCOPELEVELDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeLevelData",
    flags: MemberInfoFlags::new(101),
    module: "QuickscopeShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SUBWORLDDATACOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<QuickscopeLevelData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Categories",
                flags: MemberInfoFlags::new(0),
                field_type: "QuickscopeCategoriesAsset",
                rust_offset: offset_of!(QuickscopeLevelData, categories),
            },
            FieldInfoData {
                name: "Budgets",
                flags: MemberInfoFlags::new(0),
                field_type: "QuickscopeBudgetsAsset",
                rust_offset: offset_of!(QuickscopeLevelData, budgets),
            },
        ],
    }),
    array_type: Some(QUICKSCOPELEVELDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for QuickscopeLevelData {
    fn type_info(&self) -> &'static TypeInfo {
        QUICKSCOPELEVELDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static QUICKSCOPELEVELDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeLevelData-Array",
    flags: MemberInfoFlags::new(145),
    module: "QuickscopeShared",
    data: TypeInfoData::Array("QuickscopeLevelData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum QuickscopeProcessorType {
    #[default]
    QuickscopeProcessorType_CPU = 0,
    QuickscopeProcessorType_Compute = 1,
    QuickscopeProcessorType_GPU = 2,
    QuickscopeProcessorType_Copy = 3,
}

pub static QUICKSCOPEPROCESSORTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeProcessorType",
    flags: MemberInfoFlags::new(49429),
    module: "QuickscopeShared",
    data: TypeInfoData::Enum,
    array_type: Some(QUICKSCOPEPROCESSORTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for QuickscopeProcessorType {
    fn type_info(&self) -> &'static TypeInfo {
        QUICKSCOPEPROCESSORTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static QUICKSCOPEPROCESSORTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeProcessorType-Array",
    flags: MemberInfoFlags::new(145),
    module: "QuickscopeShared",
    data: TypeInfoData::Array("QuickscopeProcessorType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum QuickscopeFrameType {
    #[default]
    QuickscopeFrameType_Main = 0,
    QuickscopeFrameType_ClientSim = 1,
    QuickscopeFrameType_Render = 2,
    QuickscopeFrameType_Audio = 3,
}

pub static QUICKSCOPEFRAMETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeFrameType",
    flags: MemberInfoFlags::new(49429),
    module: "QuickscopeShared",
    data: TypeInfoData::Enum,
    array_type: Some(QUICKSCOPEFRAMETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for QuickscopeFrameType {
    fn type_info(&self) -> &'static TypeInfo {
        QUICKSCOPEFRAMETYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static QUICKSCOPEFRAMETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeFrameType-Array",
    flags: MemberInfoFlags::new(145),
    module: "QuickscopeShared",
    data: TypeInfoData::Array("QuickscopeFrameType"),
    array_type: None,
    alignment: 8,
};


