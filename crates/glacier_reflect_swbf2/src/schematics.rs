use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct SetVariableTypeInfoAsset {
    pub _glacier_base: super::core::FunctionTypeInfoAsset,
    pub class_type: glacier_reflect::builtin::TypeRef,
    pub field_type: glacier_reflect::builtin::TypeRef,
    pub field_offset: u32,
}

pub trait SetVariableTypeInfoAssetTrait: super::core::FunctionTypeInfoAssetTrait {
    fn class_type(&self) -> &glacier_reflect::builtin::TypeRef;
    fn field_type(&self) -> &glacier_reflect::builtin::TypeRef;
    fn field_offset(&self) -> &u32;
}

impl SetVariableTypeInfoAssetTrait for SetVariableTypeInfoAsset {
    fn class_type(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.class_type
    }
    fn field_type(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.field_type
    }
    fn field_offset(&self) -> &u32 {
        &self.field_offset
    }
}

impl super::core::FunctionTypeInfoAssetTrait for SetVariableTypeInfoAsset {
    fn parameters(&self) -> &Vec<Option<Arc<Mutex<dyn super::core::TypeInfoParameterDataContainerTrait>>>> {
        self._glacier_base.parameters()
    }
    fn owner(&self) -> &Option<Arc<Mutex<dyn super::core::ClassInfoAssetTrait>>> {
        self._glacier_base.owner()
    }
}

impl super::core::TypeInfoAssetTrait for SetVariableTypeInfoAsset {
    fn module_name(&self) -> &String {
        self._glacier_base.module_name()
    }
    fn type_name(&self) -> &String {
        self._glacier_base.type_name()
    }
    fn is_meta(&self) -> &bool {
        self._glacier_base.is_meta()
    }
    fn attributes(&self) -> &Vec<Option<Arc<Mutex<dyn super::core::TypeInfoAttributeTrait>>>> {
        self._glacier_base.attributes()
    }
    fn is_native(&self) -> &bool {
        self._glacier_base.is_native()
    }
}

impl super::core::AssetTrait for SetVariableTypeInfoAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for SetVariableTypeInfoAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static SETVARIABLETYPEINFOASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SetVariableTypeInfoAsset",
    flags: MemberInfoFlags::new(101),
    module: "Schematics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::FUNCTIONTYPEINFOASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SetVariableTypeInfoAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ClassType",
                flags: MemberInfoFlags::new(0),
                field_type: "TypeRef",
                rust_offset: offset_of!(SetVariableTypeInfoAsset, class_type),
            },
            FieldInfoData {
                name: "FieldType",
                flags: MemberInfoFlags::new(0),
                field_type: "TypeRef",
                rust_offset: offset_of!(SetVariableTypeInfoAsset, field_type),
            },
            FieldInfoData {
                name: "FieldOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(SetVariableTypeInfoAsset, field_offset),
            },
        ],
    }),
    array_type: Some(SETVARIABLETYPEINFOASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SetVariableTypeInfoAsset {
    fn type_info(&self) -> &'static TypeInfo {
        SETVARIABLETYPEINFOASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SETVARIABLETYPEINFOASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SetVariableTypeInfoAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SetVariableTypeInfoAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct GetVariableTypeInfoAsset {
    pub _glacier_base: super::core::FunctionTypeInfoAsset,
    pub class_type: glacier_reflect::builtin::TypeRef,
    pub field_type: glacier_reflect::builtin::TypeRef,
    pub field_offset: u32,
}

pub trait GetVariableTypeInfoAssetTrait: super::core::FunctionTypeInfoAssetTrait {
    fn class_type(&self) -> &glacier_reflect::builtin::TypeRef;
    fn field_type(&self) -> &glacier_reflect::builtin::TypeRef;
    fn field_offset(&self) -> &u32;
}

impl GetVariableTypeInfoAssetTrait for GetVariableTypeInfoAsset {
    fn class_type(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.class_type
    }
    fn field_type(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.field_type
    }
    fn field_offset(&self) -> &u32 {
        &self.field_offset
    }
}

impl super::core::FunctionTypeInfoAssetTrait for GetVariableTypeInfoAsset {
    fn parameters(&self) -> &Vec<Option<Arc<Mutex<dyn super::core::TypeInfoParameterDataContainerTrait>>>> {
        self._glacier_base.parameters()
    }
    fn owner(&self) -> &Option<Arc<Mutex<dyn super::core::ClassInfoAssetTrait>>> {
        self._glacier_base.owner()
    }
}

impl super::core::TypeInfoAssetTrait for GetVariableTypeInfoAsset {
    fn module_name(&self) -> &String {
        self._glacier_base.module_name()
    }
    fn type_name(&self) -> &String {
        self._glacier_base.type_name()
    }
    fn is_meta(&self) -> &bool {
        self._glacier_base.is_meta()
    }
    fn attributes(&self) -> &Vec<Option<Arc<Mutex<dyn super::core::TypeInfoAttributeTrait>>>> {
        self._glacier_base.attributes()
    }
    fn is_native(&self) -> &bool {
        self._glacier_base.is_native()
    }
}

impl super::core::AssetTrait for GetVariableTypeInfoAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for GetVariableTypeInfoAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static GETVARIABLETYPEINFOASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GetVariableTypeInfoAsset",
    flags: MemberInfoFlags::new(101),
    module: "Schematics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::FUNCTIONTYPEINFOASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GetVariableTypeInfoAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ClassType",
                flags: MemberInfoFlags::new(0),
                field_type: "TypeRef",
                rust_offset: offset_of!(GetVariableTypeInfoAsset, class_type),
            },
            FieldInfoData {
                name: "FieldType",
                flags: MemberInfoFlags::new(0),
                field_type: "TypeRef",
                rust_offset: offset_of!(GetVariableTypeInfoAsset, field_type),
            },
            FieldInfoData {
                name: "FieldOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(GetVariableTypeInfoAsset, field_offset),
            },
        ],
    }),
    array_type: Some(GETVARIABLETYPEINFOASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GetVariableTypeInfoAsset {
    fn type_info(&self) -> &'static TypeInfo {
        GETVARIABLETYPEINFOASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static GETVARIABLETYPEINFOASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GetVariableTypeInfoAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("GetVariableTypeInfoAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SchematicsUpdatePassAsset {
    pub _glacier_base: super::core::Asset,
    pub depends_on: Vec<Option<Arc<Mutex<dyn SchematicsUpdatePassAssetTrait>>>>,
}

pub trait SchematicsUpdatePassAssetTrait: super::core::AssetTrait {
    fn depends_on(&self) -> &Vec<Option<Arc<Mutex<dyn SchematicsUpdatePassAssetTrait>>>>;
}

impl SchematicsUpdatePassAssetTrait for SchematicsUpdatePassAsset {
    fn depends_on(&self) -> &Vec<Option<Arc<Mutex<dyn SchematicsUpdatePassAssetTrait>>>> {
        &self.depends_on
    }
}

impl super::core::AssetTrait for SchematicsUpdatePassAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for SchematicsUpdatePassAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static SCHEMATICSUPDATEPASSASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsUpdatePassAsset",
    flags: MemberInfoFlags::new(101),
    module: "Schematics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SchematicsUpdatePassAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "DependsOn",
                flags: MemberInfoFlags::new(144),
                field_type: "SchematicsUpdatePassAsset-Array",
                rust_offset: offset_of!(SchematicsUpdatePassAsset, depends_on),
            },
        ],
    }),
    array_type: Some(SCHEMATICSUPDATEPASSASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SchematicsUpdatePassAsset {
    fn type_info(&self) -> &'static TypeInfo {
        SCHEMATICSUPDATEPASSASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SCHEMATICSUPDATEPASSASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsUpdatePassAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SchematicsUpdatePassAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SchematicsBasePatchData {
    pub _glacier_base: super::core::DataContainer,
}

pub trait SchematicsBasePatchDataTrait: super::core::DataContainerTrait {
}

impl SchematicsBasePatchDataTrait for SchematicsBasePatchData {
}

impl super::core::DataContainerTrait for SchematicsBasePatchData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static SCHEMATICSBASEPATCHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsBasePatchData",
    flags: MemberInfoFlags::new(101),
    module: "Schematics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SchematicsBasePatchData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SCHEMATICSBASEPATCHDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SchematicsBasePatchData {
    fn type_info(&self) -> &'static TypeInfo {
        SCHEMATICSBASEPATCHDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SCHEMATICSBASEPATCHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsBasePatchData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SchematicsBasePatchData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SchematicsBaseAsset {
    pub _glacier_base: super::core::Asset,
}

pub trait SchematicsBaseAssetTrait: super::core::AssetTrait {
}

impl SchematicsBaseAssetTrait for SchematicsBaseAsset {
}

impl super::core::AssetTrait for SchematicsBaseAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for SchematicsBaseAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static SCHEMATICSBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "Schematics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SchematicsBaseAsset as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SCHEMATICSBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SchematicsBaseAsset {
    fn type_info(&self) -> &'static TypeInfo {
        SCHEMATICSBASEASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SCHEMATICSBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SchematicsBaseAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SchematicsPatchData {
    pub _glacier_base: SchematicsBasePatchData,
    pub field_patches: Vec<SchematicsFieldPatch>,
    pub constructor_patches: Vec<SchematicsParameterPatch>,
    pub nested_patches: Vec<SchematicsNestedPatch>,
    pub observer_patches: Vec<SchematicsObserverPatch>,
}

pub trait SchematicsPatchDataTrait: SchematicsBasePatchDataTrait {
    fn field_patches(&self) -> &Vec<SchematicsFieldPatch>;
    fn constructor_patches(&self) -> &Vec<SchematicsParameterPatch>;
    fn nested_patches(&self) -> &Vec<SchematicsNestedPatch>;
    fn observer_patches(&self) -> &Vec<SchematicsObserverPatch>;
}

impl SchematicsPatchDataTrait for SchematicsPatchData {
    fn field_patches(&self) -> &Vec<SchematicsFieldPatch> {
        &self.field_patches
    }
    fn constructor_patches(&self) -> &Vec<SchematicsParameterPatch> {
        &self.constructor_patches
    }
    fn nested_patches(&self) -> &Vec<SchematicsNestedPatch> {
        &self.nested_patches
    }
    fn observer_patches(&self) -> &Vec<SchematicsObserverPatch> {
        &self.observer_patches
    }
}

impl SchematicsBasePatchDataTrait for SchematicsPatchData {
}

impl super::core::DataContainerTrait for SchematicsPatchData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static SCHEMATICSPATCHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsPatchData",
    flags: MemberInfoFlags::new(101),
    module: "Schematics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SCHEMATICSBASEPATCHDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SchematicsPatchData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "FieldPatches",
                flags: MemberInfoFlags::new(144),
                field_type: "SchematicsFieldPatch-Array",
                rust_offset: offset_of!(SchematicsPatchData, field_patches),
            },
            FieldInfoData {
                name: "ConstructorPatches",
                flags: MemberInfoFlags::new(144),
                field_type: "SchematicsParameterPatch-Array",
                rust_offset: offset_of!(SchematicsPatchData, constructor_patches),
            },
            FieldInfoData {
                name: "NestedPatches",
                flags: MemberInfoFlags::new(144),
                field_type: "SchematicsNestedPatch-Array",
                rust_offset: offset_of!(SchematicsPatchData, nested_patches),
            },
            FieldInfoData {
                name: "ObserverPatches",
                flags: MemberInfoFlags::new(144),
                field_type: "SchematicsObserverPatch-Array",
                rust_offset: offset_of!(SchematicsPatchData, observer_patches),
            },
        ],
    }),
    array_type: Some(SCHEMATICSPATCHDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SchematicsPatchData {
    fn type_info(&self) -> &'static TypeInfo {
        SCHEMATICSPATCHDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SCHEMATICSPATCHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsPatchData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SchematicsPatchData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SchematicsObserverPatch {
    pub field_offset: u16,
    pub function: glacier_reflect::builtin::TypeRef,
    pub update_pass: Option<Arc<Mutex<dyn SchematicsUpdatePassAssetTrait>>>,
}

pub trait SchematicsObserverPatchTrait: TypeObject {
    fn field_offset(&self) -> &u16;
    fn function(&self) -> &glacier_reflect::builtin::TypeRef;
    fn update_pass(&self) -> &Option<Arc<Mutex<dyn SchematicsUpdatePassAssetTrait>>>;
}

impl SchematicsObserverPatchTrait for SchematicsObserverPatch {
    fn field_offset(&self) -> &u16 {
        &self.field_offset
    }
    fn function(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.function
    }
    fn update_pass(&self) -> &Option<Arc<Mutex<dyn SchematicsUpdatePassAssetTrait>>> {
        &self.update_pass
    }
}

pub static SCHEMATICSOBSERVERPATCH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsObserverPatch",
    flags: MemberInfoFlags::new(73),
    module: "Schematics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SchematicsObserverPatch as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "FieldOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(SchematicsObserverPatch, field_offset),
            },
            FieldInfoData {
                name: "Function",
                flags: MemberInfoFlags::new(0),
                field_type: "TypeRef",
                rust_offset: offset_of!(SchematicsObserverPatch, function),
            },
            FieldInfoData {
                name: "UpdatePass",
                flags: MemberInfoFlags::new(0),
                field_type: "SchematicsUpdatePassAsset",
                rust_offset: offset_of!(SchematicsObserverPatch, update_pass),
            },
        ],
    }),
    array_type: Some(SCHEMATICSOBSERVERPATCH_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SchematicsObserverPatch {
    fn type_info(&self) -> &'static TypeInfo {
        SCHEMATICSOBSERVERPATCH_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SCHEMATICSOBSERVERPATCH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsObserverPatch-Array",
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SchematicsObserverPatch"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SchematicsNestedPatch {
    pub target_offset: u16,
    pub data: Option<Arc<Mutex<dyn SchematicsPatchDataTrait>>>,
}

pub trait SchematicsNestedPatchTrait: TypeObject {
    fn target_offset(&self) -> &u16;
    fn data(&self) -> &Option<Arc<Mutex<dyn SchematicsPatchDataTrait>>>;
}

impl SchematicsNestedPatchTrait for SchematicsNestedPatch {
    fn target_offset(&self) -> &u16 {
        &self.target_offset
    }
    fn data(&self) -> &Option<Arc<Mutex<dyn SchematicsPatchDataTrait>>> {
        &self.data
    }
}

pub static SCHEMATICSNESTEDPATCH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsNestedPatch",
    flags: MemberInfoFlags::new(73),
    module: "Schematics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SchematicsNestedPatch as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TargetOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(SchematicsNestedPatch, target_offset),
            },
            FieldInfoData {
                name: "Data",
                flags: MemberInfoFlags::new(0),
                field_type: "SchematicsPatchData",
                rust_offset: offset_of!(SchematicsNestedPatch, data),
            },
        ],
    }),
    array_type: Some(SCHEMATICSNESTEDPATCH_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SchematicsNestedPatch {
    fn type_info(&self) -> &'static TypeInfo {
        SCHEMATICSNESTEDPATCH_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SCHEMATICSNESTEDPATCH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsNestedPatch-Array",
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SchematicsNestedPatch"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SchematicsParameterPatch {
    pub value: glacier_reflect::builtin::BoxedValueRef,
    pub parameter_index: u8,
    pub target_offset: u16,
}

pub trait SchematicsParameterPatchTrait: TypeObject {
    fn value(&self) -> &glacier_reflect::builtin::BoxedValueRef;
    fn parameter_index(&self) -> &u8;
    fn target_offset(&self) -> &u16;
}

impl SchematicsParameterPatchTrait for SchematicsParameterPatch {
    fn value(&self) -> &glacier_reflect::builtin::BoxedValueRef {
        &self.value
    }
    fn parameter_index(&self) -> &u8 {
        &self.parameter_index
    }
    fn target_offset(&self) -> &u16 {
        &self.target_offset
    }
}

pub static SCHEMATICSPARAMETERPATCH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsParameterPatch",
    flags: MemberInfoFlags::new(73),
    module: "Schematics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SchematicsParameterPatch as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "BoxedValueRef",
                rust_offset: offset_of!(SchematicsParameterPatch, value),
            },
            FieldInfoData {
                name: "ParameterIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(SchematicsParameterPatch, parameter_index),
            },
            FieldInfoData {
                name: "TargetOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(SchematicsParameterPatch, target_offset),
            },
        ],
    }),
    array_type: Some(SCHEMATICSPARAMETERPATCH_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SchematicsParameterPatch {
    fn type_info(&self) -> &'static TypeInfo {
        SCHEMATICSPARAMETERPATCH_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SCHEMATICSPARAMETERPATCH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsParameterPatch-Array",
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SchematicsParameterPatch"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SchematicsFieldPatch {
    pub value: glacier_reflect::builtin::BoxedValueRef,
    pub target_offset: u16,
}

pub trait SchematicsFieldPatchTrait: TypeObject {
    fn value(&self) -> &glacier_reflect::builtin::BoxedValueRef;
    fn target_offset(&self) -> &u16;
}

impl SchematicsFieldPatchTrait for SchematicsFieldPatch {
    fn value(&self) -> &glacier_reflect::builtin::BoxedValueRef {
        &self.value
    }
    fn target_offset(&self) -> &u16 {
        &self.target_offset
    }
}

pub static SCHEMATICSFIELDPATCH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsFieldPatch",
    flags: MemberInfoFlags::new(73),
    module: "Schematics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SchematicsFieldPatch as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "BoxedValueRef",
                rust_offset: offset_of!(SchematicsFieldPatch, value),
            },
            FieldInfoData {
                name: "TargetOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(SchematicsFieldPatch, target_offset),
            },
        ],
    }),
    array_type: Some(SCHEMATICSFIELDPATCH_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SchematicsFieldPatch {
    fn type_info(&self) -> &'static TypeInfo {
        SCHEMATICSFIELDPATCH_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SCHEMATICSFIELDPATCH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsFieldPatch-Array",
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SchematicsFieldPatch"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ConstField {
    pub value: glacier_reflect::builtin::BoxedValueRef,
}

pub trait ConstFieldTrait: TypeObject {
    fn value(&self) -> &glacier_reflect::builtin::BoxedValueRef;
}

impl ConstFieldTrait for ConstField {
    fn value(&self) -> &glacier_reflect::builtin::BoxedValueRef {
        &self.value
    }
}

pub static CONSTFIELD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConstField",
    flags: MemberInfoFlags::new(73),
    module: "Schematics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ConstField as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "BoxedValueRef",
                rust_offset: offset_of!(ConstField, value),
            },
        ],
    }),
    array_type: Some(CONSTFIELD_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ConstField {
    fn type_info(&self) -> &'static TypeInfo {
        CONSTFIELD_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CONSTFIELD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConstField-Array",
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("ConstField"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AutoCreatedDispatcher {
    pub field_offset: u16,
    pub parameter_type: glacier_reflect::builtin::TypeRef,
}

pub trait AutoCreatedDispatcherTrait: TypeObject {
    fn field_offset(&self) -> &u16;
    fn parameter_type(&self) -> &glacier_reflect::builtin::TypeRef;
}

impl AutoCreatedDispatcherTrait for AutoCreatedDispatcher {
    fn field_offset(&self) -> &u16 {
        &self.field_offset
    }
    fn parameter_type(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.parameter_type
    }
}

pub static AUTOCREATEDDISPATCHER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoCreatedDispatcher",
    flags: MemberInfoFlags::new(73),
    module: "Schematics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutoCreatedDispatcher as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "FieldOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(AutoCreatedDispatcher, field_offset),
            },
            FieldInfoData {
                name: "ParameterType",
                flags: MemberInfoFlags::new(0),
                field_type: "TypeRef",
                rust_offset: offset_of!(AutoCreatedDispatcher, parameter_type),
            },
        ],
    }),
    array_type: Some(AUTOCREATEDDISPATCHER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AutoCreatedDispatcher {
    fn type_info(&self) -> &'static TypeInfo {
        AUTOCREATEDDISPATCHER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static AUTOCREATEDDISPATCHER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoCreatedDispatcher-Array",
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("AutoCreatedDispatcher"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AutoCreatedField {
    pub field_offset: u16,
    pub asset: Option<Arc<Mutex<dyn SchematicsAssetTrait>>>,
    pub patch_data: Option<Arc<Mutex<dyn SchematicsPatchDataTrait>>>,
}

pub trait AutoCreatedFieldTrait: TypeObject {
    fn field_offset(&self) -> &u16;
    fn asset(&self) -> &Option<Arc<Mutex<dyn SchematicsAssetTrait>>>;
    fn patch_data(&self) -> &Option<Arc<Mutex<dyn SchematicsPatchDataTrait>>>;
}

impl AutoCreatedFieldTrait for AutoCreatedField {
    fn field_offset(&self) -> &u16 {
        &self.field_offset
    }
    fn asset(&self) -> &Option<Arc<Mutex<dyn SchematicsAssetTrait>>> {
        &self.asset
    }
    fn patch_data(&self) -> &Option<Arc<Mutex<dyn SchematicsPatchDataTrait>>> {
        &self.patch_data
    }
}

pub static AUTOCREATEDFIELD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoCreatedField",
    flags: MemberInfoFlags::new(73),
    module: "Schematics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutoCreatedField as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "FieldOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(AutoCreatedField, field_offset),
            },
            FieldInfoData {
                name: "Asset",
                flags: MemberInfoFlags::new(0),
                field_type: "SchematicsAsset",
                rust_offset: offset_of!(AutoCreatedField, asset),
            },
            FieldInfoData {
                name: "PatchData",
                flags: MemberInfoFlags::new(0),
                field_type: "SchematicsPatchData",
                rust_offset: offset_of!(AutoCreatedField, patch_data),
            },
        ],
    }),
    array_type: Some(AUTOCREATEDFIELD_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AutoCreatedField {
    fn type_info(&self) -> &'static TypeInfo {
        AUTOCREATEDFIELD_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static AUTOCREATEDFIELD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoCreatedField-Array",
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("AutoCreatedField"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EventObserverEntry {
    pub function: glacier_reflect::builtin::TypeRef,
    pub update_pass: Option<Arc<Mutex<dyn SchematicsUpdatePassAssetTrait>>>,
}

pub trait EventObserverEntryTrait: TypeObject {
    fn function(&self) -> &glacier_reflect::builtin::TypeRef;
    fn update_pass(&self) -> &Option<Arc<Mutex<dyn SchematicsUpdatePassAssetTrait>>>;
}

impl EventObserverEntryTrait for EventObserverEntry {
    fn function(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.function
    }
    fn update_pass(&self) -> &Option<Arc<Mutex<dyn SchematicsUpdatePassAssetTrait>>> {
        &self.update_pass
    }
}

pub static EVENTOBSERVERENTRY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventObserverEntry",
    flags: MemberInfoFlags::new(73),
    module: "Schematics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EventObserverEntry as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Function",
                flags: MemberInfoFlags::new(0),
                field_type: "TypeRef",
                rust_offset: offset_of!(EventObserverEntry, function),
            },
            FieldInfoData {
                name: "UpdatePass",
                flags: MemberInfoFlags::new(0),
                field_type: "SchematicsUpdatePassAsset",
                rust_offset: offset_of!(EventObserverEntry, update_pass),
            },
        ],
    }),
    array_type: Some(EVENTOBSERVERENTRY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EventObserverEntry {
    fn type_info(&self) -> &'static TypeInfo {
        EVENTOBSERVERENTRY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static EVENTOBSERVERENTRY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventObserverEntry-Array",
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("EventObserverEntry"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SchematicsAsset {
    pub _glacier_base: SchematicsBaseAsset,
    pub instance_type: glacier_reflect::builtin::TypeRef,
    pub constructor_function: glacier_reflect::builtin::TypeRef,
    pub destructor_function: glacier_reflect::builtin::TypeRef,
    pub tweaker_function: glacier_reflect::builtin::TypeRef,
    pub auto_created_fields: Vec<AutoCreatedField>,
    pub auto_created_dispatchers: Vec<AutoCreatedDispatcher>,
    pub patch_data: Option<Arc<Mutex<dyn SchematicsPatchDataTrait>>>,
    pub observers: Vec<EventObserverEntry>,
    pub const_fields: Vec<ConstField>,
}

pub trait SchematicsAssetTrait: SchematicsBaseAssetTrait {
    fn instance_type(&self) -> &glacier_reflect::builtin::TypeRef;
    fn constructor_function(&self) -> &glacier_reflect::builtin::TypeRef;
    fn destructor_function(&self) -> &glacier_reflect::builtin::TypeRef;
    fn tweaker_function(&self) -> &glacier_reflect::builtin::TypeRef;
    fn auto_created_fields(&self) -> &Vec<AutoCreatedField>;
    fn auto_created_dispatchers(&self) -> &Vec<AutoCreatedDispatcher>;
    fn patch_data(&self) -> &Option<Arc<Mutex<dyn SchematicsPatchDataTrait>>>;
    fn observers(&self) -> &Vec<EventObserverEntry>;
    fn const_fields(&self) -> &Vec<ConstField>;
}

impl SchematicsAssetTrait for SchematicsAsset {
    fn instance_type(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.instance_type
    }
    fn constructor_function(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.constructor_function
    }
    fn destructor_function(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.destructor_function
    }
    fn tweaker_function(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.tweaker_function
    }
    fn auto_created_fields(&self) -> &Vec<AutoCreatedField> {
        &self.auto_created_fields
    }
    fn auto_created_dispatchers(&self) -> &Vec<AutoCreatedDispatcher> {
        &self.auto_created_dispatchers
    }
    fn patch_data(&self) -> &Option<Arc<Mutex<dyn SchematicsPatchDataTrait>>> {
        &self.patch_data
    }
    fn observers(&self) -> &Vec<EventObserverEntry> {
        &self.observers
    }
    fn const_fields(&self) -> &Vec<ConstField> {
        &self.const_fields
    }
}

impl SchematicsBaseAssetTrait for SchematicsAsset {
}

impl super::core::AssetTrait for SchematicsAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for SchematicsAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static SCHEMATICSASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsAsset",
    flags: MemberInfoFlags::new(101),
    module: "Schematics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SCHEMATICSBASEASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SchematicsAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "InstanceType",
                flags: MemberInfoFlags::new(0),
                field_type: "TypeRef",
                rust_offset: offset_of!(SchematicsAsset, instance_type),
            },
            FieldInfoData {
                name: "ConstructorFunction",
                flags: MemberInfoFlags::new(0),
                field_type: "TypeRef",
                rust_offset: offset_of!(SchematicsAsset, constructor_function),
            },
            FieldInfoData {
                name: "DestructorFunction",
                flags: MemberInfoFlags::new(0),
                field_type: "TypeRef",
                rust_offset: offset_of!(SchematicsAsset, destructor_function),
            },
            FieldInfoData {
                name: "TweakerFunction",
                flags: MemberInfoFlags::new(0),
                field_type: "TypeRef",
                rust_offset: offset_of!(SchematicsAsset, tweaker_function),
            },
            FieldInfoData {
                name: "AutoCreatedFields",
                flags: MemberInfoFlags::new(144),
                field_type: "AutoCreatedField-Array",
                rust_offset: offset_of!(SchematicsAsset, auto_created_fields),
            },
            FieldInfoData {
                name: "AutoCreatedDispatchers",
                flags: MemberInfoFlags::new(144),
                field_type: "AutoCreatedDispatcher-Array",
                rust_offset: offset_of!(SchematicsAsset, auto_created_dispatchers),
            },
            FieldInfoData {
                name: "PatchData",
                flags: MemberInfoFlags::new(0),
                field_type: "SchematicsPatchData",
                rust_offset: offset_of!(SchematicsAsset, patch_data),
            },
            FieldInfoData {
                name: "Observers",
                flags: MemberInfoFlags::new(144),
                field_type: "EventObserverEntry-Array",
                rust_offset: offset_of!(SchematicsAsset, observers),
            },
            FieldInfoData {
                name: "ConstFields",
                flags: MemberInfoFlags::new(144),
                field_type: "ConstField-Array",
                rust_offset: offset_of!(SchematicsAsset, const_fields),
            },
        ],
    }),
    array_type: Some(SCHEMATICSASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SchematicsAsset {
    fn type_info(&self) -> &'static TypeInfo {
        SCHEMATICSASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SCHEMATICSASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SchematicsAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SchematicsInstance {
}

pub trait SchematicsInstanceTrait: TypeObject {
}

impl SchematicsInstanceTrait for SchematicsInstance {
}

pub static SCHEMATICSINSTANCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsInstance",
    flags: MemberInfoFlags::new(101),
    module: "Schematics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SchematicsInstance as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SCHEMATICSINSTANCE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SchematicsInstance {
    fn type_info(&self) -> &'static TypeInfo {
        SCHEMATICSINSTANCE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SCHEMATICSINSTANCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsInstance-Array",
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SchematicsInstance"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SchematicsEventDispatcher {
    pub _glacier_base: super::core::EventDispatcher,
}

pub trait SchematicsEventDispatcherTrait: super::core::EventDispatcherTrait {
}

impl SchematicsEventDispatcherTrait for SchematicsEventDispatcher {
}

impl super::core::EventDispatcherTrait for SchematicsEventDispatcher {
}

pub static SCHEMATICSEVENTDISPATCHER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsEventDispatcher",
    flags: MemberInfoFlags::new(101),
    module: "Schematics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::EVENTDISPATCHER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SchematicsEventDispatcher as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SCHEMATICSEVENTDISPATCHER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SchematicsEventDispatcher {
    fn type_info(&self) -> &'static TypeInfo {
        SCHEMATICSEVENTDISPATCHER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SCHEMATICSEVENTDISPATCHER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsEventDispatcher-Array",
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SchematicsEventDispatcher"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SchematicsContext {
}

pub trait SchematicsContextTrait: TypeObject {
}

impl SchematicsContextTrait for SchematicsContext {
}

pub static SCHEMATICSCONTEXT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsContext",
    flags: MemberInfoFlags::new(101),
    module: "Schematics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SchematicsContext as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SCHEMATICSCONTEXT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SchematicsContext {
    fn type_info(&self) -> &'static TypeInfo {
        SCHEMATICSCONTEXT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SCHEMATICSCONTEXT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsContext-Array",
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SchematicsContext"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SchematicsPipelineBuilder {
}

pub trait SchematicsPipelineBuilderTrait: TypeObject {
}

impl SchematicsPipelineBuilderTrait for SchematicsPipelineBuilder {
}

pub static SCHEMATICSPIPELINEBUILDER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsPipelineBuilder",
    flags: MemberInfoFlags::new(101),
    module: "Schematics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SchematicsPipelineBuilder as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SCHEMATICSPIPELINEBUILDER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SchematicsPipelineBuilder {
    fn type_info(&self) -> &'static TypeInfo {
        SCHEMATICSPIPELINEBUILDER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SCHEMATICSPIPELINEBUILDER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsPipelineBuilder-Array",
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SchematicsPipelineBuilder"),
    array_type: None,
    alignment: 8,
};


