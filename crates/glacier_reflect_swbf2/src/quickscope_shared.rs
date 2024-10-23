use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct QuickscopeControlEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub label: String,
}

pub trait QuickscopeControlEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn label(&self) -> &String;
    fn label_mut(&mut self) -> &mut String;
}

impl QuickscopeControlEntityDataTrait for QuickscopeControlEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn label(&self) -> &String {
        &self.label
    }
    fn label_mut(&mut self) -> &mut String {
        &mut self.label
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
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for QuickscopeControlEntityData {
}

impl super::core::DataContainerTrait for QuickscopeControlEntityData {
}

pub static QUICKSCOPECONTROLENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeControlEntityData",
    name_hash: 2446989702,
    flags: MemberInfoFlags::new(101),
    module: "QuickscopeShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(QuickscopeControlEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<QuickscopeControlEntityData as Default>::default())),
            create_boxed: || Box::new(<QuickscopeControlEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                name_hash: 229961746,
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(QuickscopeControlEntityData, realm),
            },
            FieldInfoData {
                name: "Label",
                name_hash: 218105699,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static QUICKSCOPECONTROLENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeControlEntityData-Array",
    name_hash: 435275442,
    flags: MemberInfoFlags::new(145),
    module: "QuickscopeShared",
    data: TypeInfoData::Array("QuickscopeControlEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct QuickscopeTest {
    pub _glacier_base: super::core::Asset,
    pub number: i32,
}

pub trait QuickscopeTestTrait: super::core::AssetTrait {
    fn number(&self) -> &i32;
    fn number_mut(&mut self) -> &mut i32;
}

impl QuickscopeTestTrait for QuickscopeTest {
    fn number(&self) -> &i32 {
        &self.number
    }
    fn number_mut(&mut self) -> &mut i32 {
        &mut self.number
    }
}

impl super::core::AssetTrait for QuickscopeTest {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for QuickscopeTest {
}

pub static QUICKSCOPETEST_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeTest",
    name_hash: 4007529148,
    flags: MemberInfoFlags::new(101),
    module: "QuickscopeShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(QuickscopeTest, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<QuickscopeTest as Default>::default())),
            create_boxed: || Box::new(<QuickscopeTest as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Number",
                name_hash: 2852165926,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static QUICKSCOPETEST_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeTest-Array",
    name_hash: 191611400,
    flags: MemberInfoFlags::new(145),
    module: "QuickscopeShared",
    data: TypeInfoData::Array("QuickscopeTest"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct QuickscopeBudgetsAsset {
    pub _glacier_base: super::core::Asset,
    pub entries: Vec<BoxedTypeObject /* QuickscopeBudgetEntry */>,
    pub resolution_targets: Vec<BoxedTypeObject /* QuickscopePlatformValue */>,
}

pub trait QuickscopeBudgetsAssetTrait: super::core::AssetTrait {
    fn entries(&self) -> &Vec<BoxedTypeObject /* QuickscopeBudgetEntry */>;
    fn entries_mut(&mut self) -> &mut Vec<BoxedTypeObject /* QuickscopeBudgetEntry */>;
    fn resolution_targets(&self) -> &Vec<BoxedTypeObject /* QuickscopePlatformValue */>;
    fn resolution_targets_mut(&mut self) -> &mut Vec<BoxedTypeObject /* QuickscopePlatformValue */>;
}

impl QuickscopeBudgetsAssetTrait for QuickscopeBudgetsAsset {
    fn entries(&self) -> &Vec<BoxedTypeObject /* QuickscopeBudgetEntry */> {
        &self.entries
    }
    fn entries_mut(&mut self) -> &mut Vec<BoxedTypeObject /* QuickscopeBudgetEntry */> {
        &mut self.entries
    }
    fn resolution_targets(&self) -> &Vec<BoxedTypeObject /* QuickscopePlatformValue */> {
        &self.resolution_targets
    }
    fn resolution_targets_mut(&mut self) -> &mut Vec<BoxedTypeObject /* QuickscopePlatformValue */> {
        &mut self.resolution_targets
    }
}

impl super::core::AssetTrait for QuickscopeBudgetsAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for QuickscopeBudgetsAsset {
}

pub static QUICKSCOPEBUDGETSASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeBudgetsAsset",
    name_hash: 165555948,
    flags: MemberInfoFlags::new(101),
    module: "QuickscopeShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(QuickscopeBudgetsAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<QuickscopeBudgetsAsset as Default>::default())),
            create_boxed: || Box::new(<QuickscopeBudgetsAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Entries",
                name_hash: 8238103,
                flags: MemberInfoFlags::new(144),
                field_type: "QuickscopeBudgetEntry-Array",
                rust_offset: offset_of!(QuickscopeBudgetsAsset, entries),
            },
            FieldInfoData {
                name: "ResolutionTargets",
                name_hash: 2321090633,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static QUICKSCOPEBUDGETSASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeBudgetsAsset-Array",
    name_hash: 4247736024,
    flags: MemberInfoFlags::new(145),
    module: "QuickscopeShared",
    data: TypeInfoData::Array("QuickscopeBudgetsAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct QuickscopeBudgetEntry {
    pub category_name: String,
    pub budgets: Vec<BoxedTypeObject /* QuickscopePlatformValue */>,
}

pub trait QuickscopeBudgetEntryTrait: TypeObject {
    fn category_name(&self) -> &String;
    fn category_name_mut(&mut self) -> &mut String;
    fn budgets(&self) -> &Vec<BoxedTypeObject /* QuickscopePlatformValue */>;
    fn budgets_mut(&mut self) -> &mut Vec<BoxedTypeObject /* QuickscopePlatformValue */>;
}

impl QuickscopeBudgetEntryTrait for QuickscopeBudgetEntry {
    fn category_name(&self) -> &String {
        &self.category_name
    }
    fn category_name_mut(&mut self) -> &mut String {
        &mut self.category_name
    }
    fn budgets(&self) -> &Vec<BoxedTypeObject /* QuickscopePlatformValue */> {
        &self.budgets
    }
    fn budgets_mut(&mut self) -> &mut Vec<BoxedTypeObject /* QuickscopePlatformValue */> {
        &mut self.budgets
    }
}

pub static QUICKSCOPEBUDGETENTRY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeBudgetEntry",
    name_hash: 3846491675,
    flags: MemberInfoFlags::new(73),
    module: "QuickscopeShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<QuickscopeBudgetEntry as Default>::default())),
            create_boxed: || Box::new(<QuickscopeBudgetEntry as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "CategoryName",
                name_hash: 1997430002,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(QuickscopeBudgetEntry, category_name),
            },
            FieldInfoData {
                name: "Budgets",
                name_hash: 2763087795,
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


pub static QUICKSCOPEBUDGETENTRY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeBudgetEntry-Array",
    name_hash: 4057687215,
    flags: MemberInfoFlags::new(145),
    module: "QuickscopeShared",
    data: TypeInfoData::Array("QuickscopeBudgetEntry"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct QuickscopePlatformValue {
    pub platform: QuickscopePlatform,
    pub value: f32,
}

pub trait QuickscopePlatformValueTrait: TypeObject {
    fn platform(&self) -> &QuickscopePlatform;
    fn platform_mut(&mut self) -> &mut QuickscopePlatform;
    fn value(&self) -> &f32;
    fn value_mut(&mut self) -> &mut f32;
}

impl QuickscopePlatformValueTrait for QuickscopePlatformValue {
    fn platform(&self) -> &QuickscopePlatform {
        &self.platform
    }
    fn platform_mut(&mut self) -> &mut QuickscopePlatform {
        &mut self.platform
    }
    fn value(&self) -> &f32 {
        &self.value
    }
    fn value_mut(&mut self) -> &mut f32 {
        &mut self.value
    }
}

pub static QUICKSCOPEPLATFORMVALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopePlatformValue",
    name_hash: 3880671390,
    flags: MemberInfoFlags::new(36937),
    module: "QuickscopeShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<QuickscopePlatformValue as Default>::default())),
            create_boxed: || Box::new(<QuickscopePlatformValue as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Platform",
                name_hash: 942751002,
                flags: MemberInfoFlags::new(0),
                field_type: "QuickscopePlatform",
                rust_offset: offset_of!(QuickscopePlatformValue, platform),
            },
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
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


pub static QUICKSCOPEPLATFORMVALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopePlatformValue-Array",
    name_hash: 1567320490,
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
    name_hash: 3010488757,
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


pub static QUICKSCOPEPLATFORM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopePlatform-Array",
    name_hash: 3831581185,
    flags: MemberInfoFlags::new(145),
    module: "QuickscopeShared",
    data: TypeInfoData::Array("QuickscopePlatform"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct QuickscopeCategoriesAsset {
    pub _glacier_base: super::core::Asset,
    pub categories: Vec<BoxedTypeObject /* QuickscopeCategory */>,
}

pub trait QuickscopeCategoriesAssetTrait: super::core::AssetTrait {
    fn categories(&self) -> &Vec<BoxedTypeObject /* QuickscopeCategory */>;
    fn categories_mut(&mut self) -> &mut Vec<BoxedTypeObject /* QuickscopeCategory */>;
}

impl QuickscopeCategoriesAssetTrait for QuickscopeCategoriesAsset {
    fn categories(&self) -> &Vec<BoxedTypeObject /* QuickscopeCategory */> {
        &self.categories
    }
    fn categories_mut(&mut self) -> &mut Vec<BoxedTypeObject /* QuickscopeCategory */> {
        &mut self.categories
    }
}

impl super::core::AssetTrait for QuickscopeCategoriesAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for QuickscopeCategoriesAsset {
}

pub static QUICKSCOPECATEGORIESASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeCategoriesAsset",
    name_hash: 2403708844,
    flags: MemberInfoFlags::new(101),
    module: "QuickscopeShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(QuickscopeCategoriesAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<QuickscopeCategoriesAsset as Default>::default())),
            create_boxed: || Box::new(<QuickscopeCategoriesAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Categories",
                name_hash: 1039077843,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static QUICKSCOPECATEGORIESASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeCategoriesAsset-Array",
    name_hash: 2527616792,
    flags: MemberInfoFlags::new(145),
    module: "QuickscopeShared",
    data: TypeInfoData::Array("QuickscopeCategoriesAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct QuickscopeCategory {
    pub name: String,
    pub frame: QuickscopeFrameType,
    pub processor: QuickscopeProcessorType,
    pub include_scope: Vec<String>,
    pub exclude_scope: Vec<String>,
}

pub trait QuickscopeCategoryTrait: TypeObject {
    fn name(&self) -> &String;
    fn name_mut(&mut self) -> &mut String;
    fn frame(&self) -> &QuickscopeFrameType;
    fn frame_mut(&mut self) -> &mut QuickscopeFrameType;
    fn processor(&self) -> &QuickscopeProcessorType;
    fn processor_mut(&mut self) -> &mut QuickscopeProcessorType;
    fn include_scope(&self) -> &Vec<String>;
    fn include_scope_mut(&mut self) -> &mut Vec<String>;
    fn exclude_scope(&self) -> &Vec<String>;
    fn exclude_scope_mut(&mut self) -> &mut Vec<String>;
}

impl QuickscopeCategoryTrait for QuickscopeCategory {
    fn name(&self) -> &String {
        &self.name
    }
    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    fn frame(&self) -> &QuickscopeFrameType {
        &self.frame
    }
    fn frame_mut(&mut self) -> &mut QuickscopeFrameType {
        &mut self.frame
    }
    fn processor(&self) -> &QuickscopeProcessorType {
        &self.processor
    }
    fn processor_mut(&mut self) -> &mut QuickscopeProcessorType {
        &mut self.processor
    }
    fn include_scope(&self) -> &Vec<String> {
        &self.include_scope
    }
    fn include_scope_mut(&mut self) -> &mut Vec<String> {
        &mut self.include_scope
    }
    fn exclude_scope(&self) -> &Vec<String> {
        &self.exclude_scope
    }
    fn exclude_scope_mut(&mut self) -> &mut Vec<String> {
        &mut self.exclude_scope
    }
}

pub static QUICKSCOPECATEGORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeCategory",
    name_hash: 2671955354,
    flags: MemberInfoFlags::new(73),
    module: "QuickscopeShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<QuickscopeCategory as Default>::default())),
            create_boxed: || Box::new(<QuickscopeCategory as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                name_hash: 2088949890,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(QuickscopeCategory, name),
            },
            FieldInfoData {
                name: "Frame",
                name_hash: 206997912,
                flags: MemberInfoFlags::new(0),
                field_type: "QuickscopeFrameType",
                rust_offset: offset_of!(QuickscopeCategory, frame),
            },
            FieldInfoData {
                name: "Processor",
                name_hash: 136630291,
                flags: MemberInfoFlags::new(0),
                field_type: "QuickscopeProcessorType",
                rust_offset: offset_of!(QuickscopeCategory, processor),
            },
            FieldInfoData {
                name: "IncludeScope",
                name_hash: 791040819,
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(QuickscopeCategory, include_scope),
            },
            FieldInfoData {
                name: "ExcludeScope",
                name_hash: 759250601,
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


pub static QUICKSCOPECATEGORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeCategory-Array",
    name_hash: 2028204206,
    flags: MemberInfoFlags::new(145),
    module: "QuickscopeShared",
    data: TypeInfoData::Array("QuickscopeCategory"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct QuickscopeLevelData {
    pub _glacier_base: super::entity::SubWorldDataComponent,
    pub categories: Option<LockedTypeObject /* QuickscopeCategoriesAsset */>,
    pub budgets: Option<LockedTypeObject /* QuickscopeBudgetsAsset */>,
}

pub trait QuickscopeLevelDataTrait: super::entity::SubWorldDataComponentTrait {
    fn categories(&self) -> &Option<LockedTypeObject /* QuickscopeCategoriesAsset */>;
    fn categories_mut(&mut self) -> &mut Option<LockedTypeObject /* QuickscopeCategoriesAsset */>;
    fn budgets(&self) -> &Option<LockedTypeObject /* QuickscopeBudgetsAsset */>;
    fn budgets_mut(&mut self) -> &mut Option<LockedTypeObject /* QuickscopeBudgetsAsset */>;
}

impl QuickscopeLevelDataTrait for QuickscopeLevelData {
    fn categories(&self) -> &Option<LockedTypeObject /* QuickscopeCategoriesAsset */> {
        &self.categories
    }
    fn categories_mut(&mut self) -> &mut Option<LockedTypeObject /* QuickscopeCategoriesAsset */> {
        &mut self.categories
    }
    fn budgets(&self) -> &Option<LockedTypeObject /* QuickscopeBudgetsAsset */> {
        &self.budgets
    }
    fn budgets_mut(&mut self) -> &mut Option<LockedTypeObject /* QuickscopeBudgetsAsset */> {
        &mut self.budgets
    }
}

impl super::entity::SubWorldDataComponentTrait for QuickscopeLevelData {
}

impl super::core::DataContainerTrait for QuickscopeLevelData {
}

pub static QUICKSCOPELEVELDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeLevelData",
    name_hash: 1612757836,
    flags: MemberInfoFlags::new(101),
    module: "QuickscopeShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SUBWORLDDATACOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(QuickscopeLevelData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<QuickscopeLevelData as Default>::default())),
            create_boxed: || Box::new(<QuickscopeLevelData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Categories",
                name_hash: 1039077843,
                flags: MemberInfoFlags::new(0),
                field_type: "QuickscopeCategoriesAsset",
                rust_offset: offset_of!(QuickscopeLevelData, categories),
            },
            FieldInfoData {
                name: "Budgets",
                name_hash: 2763087795,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static QUICKSCOPELEVELDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeLevelData-Array",
    name_hash: 3526482936,
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
    name_hash: 1862495460,
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


pub static QUICKSCOPEPROCESSORTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeProcessorType-Array",
    name_hash: 1112256720,
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
    name_hash: 2831633711,
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


pub static QUICKSCOPEFRAMETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeFrameType-Array",
    name_hash: 1884936859,
    flags: MemberInfoFlags::new(145),
    module: "QuickscopeShared",
    data: TypeInfoData::Array("QuickscopeFrameType"),
    array_type: None,
    alignment: 8,
};


