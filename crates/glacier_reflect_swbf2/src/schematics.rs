use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct SetVariableTypeInfoAsset {
    pub _glacier_base: super::core::FunctionTypeInfoAsset,
    pub class_type: glacier_reflect::builtin::TypeRef,
    pub field_type: glacier_reflect::builtin::TypeRef,
    pub field_offset: u32,
}

pub trait SetVariableTypeInfoAssetTrait: super::core::FunctionTypeInfoAssetTrait {
    fn class_type(&self) -> &glacier_reflect::builtin::TypeRef;
    fn class_type_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef;
    fn field_type(&self) -> &glacier_reflect::builtin::TypeRef;
    fn field_type_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef;
    fn field_offset(&self) -> &u32;
    fn field_offset_mut(&mut self) -> &mut u32;
}

impl SetVariableTypeInfoAssetTrait for SetVariableTypeInfoAsset {
    fn class_type(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.class_type
    }
    fn class_type_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef {
        &mut self.class_type
    }
    fn field_type(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.field_type
    }
    fn field_type_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef {
        &mut self.field_type
    }
    fn field_offset(&self) -> &u32 {
        &self.field_offset
    }
    fn field_offset_mut(&mut self) -> &mut u32 {
        &mut self.field_offset
    }
}

impl super::core::FunctionTypeInfoAssetTrait for SetVariableTypeInfoAsset {
    fn parameters(&self) -> &Vec<Option<LockedTypeObject /* super::core::TypeInfoParameterDataContainer */>> {
        self._glacier_base.parameters()
    }
    fn parameters_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::core::TypeInfoParameterDataContainer */>> {
        self._glacier_base.parameters_mut()
    }
    fn owner(&self) -> &Option<LockedTypeObject /* super::core::ClassInfoAsset */> {
        self._glacier_base.owner()
    }
    fn owner_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::ClassInfoAsset */> {
        self._glacier_base.owner_mut()
    }
}

impl super::core::TypeInfoAssetTrait for SetVariableTypeInfoAsset {
    fn module_name(&self) -> &String {
        self._glacier_base.module_name()
    }
    fn module_name_mut(&mut self) -> &mut String {
        self._glacier_base.module_name_mut()
    }
    fn type_name(&self) -> &String {
        self._glacier_base.type_name()
    }
    fn type_name_mut(&mut self) -> &mut String {
        self._glacier_base.type_name_mut()
    }
    fn is_meta(&self) -> &bool {
        self._glacier_base.is_meta()
    }
    fn is_meta_mut(&mut self) -> &mut bool {
        self._glacier_base.is_meta_mut()
    }
    fn attributes(&self) -> &Vec<Option<LockedTypeObject /* super::core::TypeInfoAttribute */>> {
        self._glacier_base.attributes()
    }
    fn attributes_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::core::TypeInfoAttribute */>> {
        self._glacier_base.attributes_mut()
    }
    fn is_native(&self) -> &bool {
        self._glacier_base.is_native()
    }
    fn is_native_mut(&mut self) -> &mut bool {
        self._glacier_base.is_native_mut()
    }
}

impl super::core::AssetTrait for SetVariableTypeInfoAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for SetVariableTypeInfoAsset {
}

pub static SETVARIABLETYPEINFOASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SetVariableTypeInfoAsset",
    name_hash: 3237290311,
    flags: MemberInfoFlags::new(101),
    module: "Schematics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::FUNCTIONTYPEINFOASSET_TYPE_INFO),
        super_class_offset: offset_of!(SetVariableTypeInfoAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SetVariableTypeInfoAsset as Default>::default())),
            create_boxed: || Box::new(<SetVariableTypeInfoAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ClassType",
                name_hash: 3204124979,
                flags: MemberInfoFlags::new(0),
                field_type: "TypeRef",
                rust_offset: offset_of!(SetVariableTypeInfoAsset, class_type),
            },
            FieldInfoData {
                name: "FieldType",
                name_hash: 3059058943,
                flags: MemberInfoFlags::new(0),
                field_type: "TypeRef",
                rust_offset: offset_of!(SetVariableTypeInfoAsset, field_type),
            },
            FieldInfoData {
                name: "FieldOffset",
                name_hash: 2228247466,
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


pub static SETVARIABLETYPEINFOASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SetVariableTypeInfoAsset-Array",
    name_hash: 933917299,
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SetVariableTypeInfoAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct GetVariableTypeInfoAsset {
    pub _glacier_base: super::core::FunctionTypeInfoAsset,
    pub class_type: glacier_reflect::builtin::TypeRef,
    pub field_type: glacier_reflect::builtin::TypeRef,
    pub field_offset: u32,
}

pub trait GetVariableTypeInfoAssetTrait: super::core::FunctionTypeInfoAssetTrait {
    fn class_type(&self) -> &glacier_reflect::builtin::TypeRef;
    fn class_type_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef;
    fn field_type(&self) -> &glacier_reflect::builtin::TypeRef;
    fn field_type_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef;
    fn field_offset(&self) -> &u32;
    fn field_offset_mut(&mut self) -> &mut u32;
}

impl GetVariableTypeInfoAssetTrait for GetVariableTypeInfoAsset {
    fn class_type(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.class_type
    }
    fn class_type_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef {
        &mut self.class_type
    }
    fn field_type(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.field_type
    }
    fn field_type_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef {
        &mut self.field_type
    }
    fn field_offset(&self) -> &u32 {
        &self.field_offset
    }
    fn field_offset_mut(&mut self) -> &mut u32 {
        &mut self.field_offset
    }
}

impl super::core::FunctionTypeInfoAssetTrait for GetVariableTypeInfoAsset {
    fn parameters(&self) -> &Vec<Option<LockedTypeObject /* super::core::TypeInfoParameterDataContainer */>> {
        self._glacier_base.parameters()
    }
    fn parameters_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::core::TypeInfoParameterDataContainer */>> {
        self._glacier_base.parameters_mut()
    }
    fn owner(&self) -> &Option<LockedTypeObject /* super::core::ClassInfoAsset */> {
        self._glacier_base.owner()
    }
    fn owner_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::ClassInfoAsset */> {
        self._glacier_base.owner_mut()
    }
}

impl super::core::TypeInfoAssetTrait for GetVariableTypeInfoAsset {
    fn module_name(&self) -> &String {
        self._glacier_base.module_name()
    }
    fn module_name_mut(&mut self) -> &mut String {
        self._glacier_base.module_name_mut()
    }
    fn type_name(&self) -> &String {
        self._glacier_base.type_name()
    }
    fn type_name_mut(&mut self) -> &mut String {
        self._glacier_base.type_name_mut()
    }
    fn is_meta(&self) -> &bool {
        self._glacier_base.is_meta()
    }
    fn is_meta_mut(&mut self) -> &mut bool {
        self._glacier_base.is_meta_mut()
    }
    fn attributes(&self) -> &Vec<Option<LockedTypeObject /* super::core::TypeInfoAttribute */>> {
        self._glacier_base.attributes()
    }
    fn attributes_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::core::TypeInfoAttribute */>> {
        self._glacier_base.attributes_mut()
    }
    fn is_native(&self) -> &bool {
        self._glacier_base.is_native()
    }
    fn is_native_mut(&mut self) -> &mut bool {
        self._glacier_base.is_native_mut()
    }
}

impl super::core::AssetTrait for GetVariableTypeInfoAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for GetVariableTypeInfoAsset {
}

pub static GETVARIABLETYPEINFOASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GetVariableTypeInfoAsset",
    name_hash: 4192372691,
    flags: MemberInfoFlags::new(101),
    module: "Schematics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::FUNCTIONTYPEINFOASSET_TYPE_INFO),
        super_class_offset: offset_of!(GetVariableTypeInfoAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GetVariableTypeInfoAsset as Default>::default())),
            create_boxed: || Box::new(<GetVariableTypeInfoAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ClassType",
                name_hash: 3204124979,
                flags: MemberInfoFlags::new(0),
                field_type: "TypeRef",
                rust_offset: offset_of!(GetVariableTypeInfoAsset, class_type),
            },
            FieldInfoData {
                name: "FieldType",
                name_hash: 3059058943,
                flags: MemberInfoFlags::new(0),
                field_type: "TypeRef",
                rust_offset: offset_of!(GetVariableTypeInfoAsset, field_type),
            },
            FieldInfoData {
                name: "FieldOffset",
                name_hash: 2228247466,
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


pub static GETVARIABLETYPEINFOASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GetVariableTypeInfoAsset-Array",
    name_hash: 2382222055,
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("GetVariableTypeInfoAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SchematicsUpdatePassAsset {
    pub _glacier_base: super::core::Asset,
    pub depends_on: Vec<Option<LockedTypeObject /* SchematicsUpdatePassAsset */>>,
}

pub trait SchematicsUpdatePassAssetTrait: super::core::AssetTrait {
    fn depends_on(&self) -> &Vec<Option<LockedTypeObject /* SchematicsUpdatePassAsset */>>;
    fn depends_on_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* SchematicsUpdatePassAsset */>>;
}

impl SchematicsUpdatePassAssetTrait for SchematicsUpdatePassAsset {
    fn depends_on(&self) -> &Vec<Option<LockedTypeObject /* SchematicsUpdatePassAsset */>> {
        &self.depends_on
    }
    fn depends_on_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* SchematicsUpdatePassAsset */>> {
        &mut self.depends_on
    }
}

impl super::core::AssetTrait for SchematicsUpdatePassAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for SchematicsUpdatePassAsset {
}

pub static SCHEMATICSUPDATEPASSASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsUpdatePassAsset",
    name_hash: 2308917865,
    flags: MemberInfoFlags::new(101),
    module: "Schematics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(SchematicsUpdatePassAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SchematicsUpdatePassAsset as Default>::default())),
            create_boxed: || Box::new(<SchematicsUpdatePassAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "DependsOn",
                name_hash: 1865230697,
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


pub static SCHEMATICSUPDATEPASSASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsUpdatePassAsset-Array",
    name_hash: 820175197,
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SchematicsUpdatePassAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SchematicsBasePatchData {
    pub _glacier_base: super::core::DataContainer,
}

pub trait SchematicsBasePatchDataTrait: super::core::DataContainerTrait {
}

impl SchematicsBasePatchDataTrait for SchematicsBasePatchData {
}

impl super::core::DataContainerTrait for SchematicsBasePatchData {
}

pub static SCHEMATICSBASEPATCHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsBasePatchData",
    name_hash: 3241964658,
    flags: MemberInfoFlags::new(101),
    module: "Schematics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(SchematicsBasePatchData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SchematicsBasePatchData as Default>::default())),
            create_boxed: || Box::new(<SchematicsBasePatchData as Default>::default()),
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


pub static SCHEMATICSBASEPATCHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsBasePatchData-Array",
    name_hash: 940288838,
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SchematicsBasePatchData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for SchematicsBaseAsset {
}

pub static SCHEMATICSBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsBaseAsset",
    name_hash: 2709129500,
    flags: MemberInfoFlags::new(101),
    module: "Schematics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(SchematicsBaseAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SchematicsBaseAsset as Default>::default())),
            create_boxed: || Box::new(<SchematicsBaseAsset as Default>::default()),
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


pub static SCHEMATICSBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsBaseAsset-Array",
    name_hash: 3104653608,
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SchematicsBaseAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SchematicsPatchData {
    pub _glacier_base: SchematicsBasePatchData,
    pub field_patches: Vec<BoxedTypeObject /* SchematicsFieldPatch */>,
    pub constructor_patches: Vec<BoxedTypeObject /* SchematicsParameterPatch */>,
    pub nested_patches: Vec<BoxedTypeObject /* SchematicsNestedPatch */>,
    pub observer_patches: Vec<BoxedTypeObject /* SchematicsObserverPatch */>,
}

pub trait SchematicsPatchDataTrait: SchematicsBasePatchDataTrait {
    fn field_patches(&self) -> &Vec<BoxedTypeObject /* SchematicsFieldPatch */>;
    fn field_patches_mut(&mut self) -> &mut Vec<BoxedTypeObject /* SchematicsFieldPatch */>;
    fn constructor_patches(&self) -> &Vec<BoxedTypeObject /* SchematicsParameterPatch */>;
    fn constructor_patches_mut(&mut self) -> &mut Vec<BoxedTypeObject /* SchematicsParameterPatch */>;
    fn nested_patches(&self) -> &Vec<BoxedTypeObject /* SchematicsNestedPatch */>;
    fn nested_patches_mut(&mut self) -> &mut Vec<BoxedTypeObject /* SchematicsNestedPatch */>;
    fn observer_patches(&self) -> &Vec<BoxedTypeObject /* SchematicsObserverPatch */>;
    fn observer_patches_mut(&mut self) -> &mut Vec<BoxedTypeObject /* SchematicsObserverPatch */>;
}

impl SchematicsPatchDataTrait for SchematicsPatchData {
    fn field_patches(&self) -> &Vec<BoxedTypeObject /* SchematicsFieldPatch */> {
        &self.field_patches
    }
    fn field_patches_mut(&mut self) -> &mut Vec<BoxedTypeObject /* SchematicsFieldPatch */> {
        &mut self.field_patches
    }
    fn constructor_patches(&self) -> &Vec<BoxedTypeObject /* SchematicsParameterPatch */> {
        &self.constructor_patches
    }
    fn constructor_patches_mut(&mut self) -> &mut Vec<BoxedTypeObject /* SchematicsParameterPatch */> {
        &mut self.constructor_patches
    }
    fn nested_patches(&self) -> &Vec<BoxedTypeObject /* SchematicsNestedPatch */> {
        &self.nested_patches
    }
    fn nested_patches_mut(&mut self) -> &mut Vec<BoxedTypeObject /* SchematicsNestedPatch */> {
        &mut self.nested_patches
    }
    fn observer_patches(&self) -> &Vec<BoxedTypeObject /* SchematicsObserverPatch */> {
        &self.observer_patches
    }
    fn observer_patches_mut(&mut self) -> &mut Vec<BoxedTypeObject /* SchematicsObserverPatch */> {
        &mut self.observer_patches
    }
}

impl SchematicsBasePatchDataTrait for SchematicsPatchData {
}

impl super::core::DataContainerTrait for SchematicsPatchData {
}

pub static SCHEMATICSPATCHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsPatchData",
    name_hash: 1220470471,
    flags: MemberInfoFlags::new(101),
    module: "Schematics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SCHEMATICSBASEPATCHDATA_TYPE_INFO),
        super_class_offset: offset_of!(SchematicsPatchData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SchematicsPatchData as Default>::default())),
            create_boxed: || Box::new(<SchematicsPatchData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "FieldPatches",
                name_hash: 559521567,
                flags: MemberInfoFlags::new(144),
                field_type: "SchematicsFieldPatch-Array",
                rust_offset: offset_of!(SchematicsPatchData, field_patches),
            },
            FieldInfoData {
                name: "ConstructorPatches",
                name_hash: 3404014837,
                flags: MemberInfoFlags::new(144),
                field_type: "SchematicsParameterPatch-Array",
                rust_offset: offset_of!(SchematicsPatchData, constructor_patches),
            },
            FieldInfoData {
                name: "NestedPatches",
                name_hash: 8607440,
                flags: MemberInfoFlags::new(144),
                field_type: "SchematicsNestedPatch-Array",
                rust_offset: offset_of!(SchematicsPatchData, nested_patches),
            },
            FieldInfoData {
                name: "ObserverPatches",
                name_hash: 3150743765,
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


pub static SCHEMATICSPATCHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsPatchData-Array",
    name_hash: 3222427635,
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SchematicsPatchData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SchematicsObserverPatch {
    pub field_offset: u16,
    pub function: glacier_reflect::builtin::TypeRef,
    pub update_pass: Option<LockedTypeObject /* SchematicsUpdatePassAsset */>,
}

pub trait SchematicsObserverPatchTrait: TypeObject {
    fn field_offset(&self) -> &u16;
    fn field_offset_mut(&mut self) -> &mut u16;
    fn function(&self) -> &glacier_reflect::builtin::TypeRef;
    fn function_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef;
    fn update_pass(&self) -> &Option<LockedTypeObject /* SchematicsUpdatePassAsset */>;
    fn update_pass_mut(&mut self) -> &mut Option<LockedTypeObject /* SchematicsUpdatePassAsset */>;
}

impl SchematicsObserverPatchTrait for SchematicsObserverPatch {
    fn field_offset(&self) -> &u16 {
        &self.field_offset
    }
    fn field_offset_mut(&mut self) -> &mut u16 {
        &mut self.field_offset
    }
    fn function(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.function
    }
    fn function_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef {
        &mut self.function
    }
    fn update_pass(&self) -> &Option<LockedTypeObject /* SchematicsUpdatePassAsset */> {
        &self.update_pass
    }
    fn update_pass_mut(&mut self) -> &mut Option<LockedTypeObject /* SchematicsUpdatePassAsset */> {
        &mut self.update_pass
    }
}

pub static SCHEMATICSOBSERVERPATCH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsObserverPatch",
    name_hash: 3461047615,
    flags: MemberInfoFlags::new(73),
    module: "Schematics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SchematicsObserverPatch as Default>::default())),
            create_boxed: || Box::new(<SchematicsObserverPatch as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "FieldOffset",
                name_hash: 2228247466,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(SchematicsObserverPatch, field_offset),
            },
            FieldInfoData {
                name: "Function",
                name_hash: 4136871687,
                flags: MemberInfoFlags::new(0),
                field_type: "TypeRef",
                rust_offset: offset_of!(SchematicsObserverPatch, function),
            },
            FieldInfoData {
                name: "UpdatePass",
                name_hash: 2270785669,
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


pub static SCHEMATICSOBSERVERPATCH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsObserverPatch-Array",
    name_hash: 3995253387,
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SchematicsObserverPatch"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SchematicsNestedPatch {
    pub target_offset: u16,
    pub data: Option<LockedTypeObject /* SchematicsPatchData */>,
}

pub trait SchematicsNestedPatchTrait: TypeObject {
    fn target_offset(&self) -> &u16;
    fn target_offset_mut(&mut self) -> &mut u16;
    fn data(&self) -> &Option<LockedTypeObject /* SchematicsPatchData */>;
    fn data_mut(&mut self) -> &mut Option<LockedTypeObject /* SchematicsPatchData */>;
}

impl SchematicsNestedPatchTrait for SchematicsNestedPatch {
    fn target_offset(&self) -> &u16 {
        &self.target_offset
    }
    fn target_offset_mut(&mut self) -> &mut u16 {
        &mut self.target_offset
    }
    fn data(&self) -> &Option<LockedTypeObject /* SchematicsPatchData */> {
        &self.data
    }
    fn data_mut(&mut self) -> &mut Option<LockedTypeObject /* SchematicsPatchData */> {
        &mut self.data
    }
}

pub static SCHEMATICSNESTEDPATCH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsNestedPatch",
    name_hash: 1901574522,
    flags: MemberInfoFlags::new(73),
    module: "Schematics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SchematicsNestedPatch as Default>::default())),
            create_boxed: || Box::new(<SchematicsNestedPatch as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "TargetOffset",
                name_hash: 1634518457,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(SchematicsNestedPatch, target_offset),
            },
            FieldInfoData {
                name: "Data",
                name_hash: 2088730869,
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


pub static SCHEMATICSNESTEDPATCH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsNestedPatch-Array",
    name_hash: 88476750,
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SchematicsNestedPatch"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SchematicsParameterPatch {
    pub value: glacier_reflect::builtin::BoxedValueRef,
    pub parameter_index: u8,
    pub target_offset: u16,
}

pub trait SchematicsParameterPatchTrait: TypeObject {
    fn value(&self) -> &glacier_reflect::builtin::BoxedValueRef;
    fn value_mut(&mut self) -> &mut glacier_reflect::builtin::BoxedValueRef;
    fn parameter_index(&self) -> &u8;
    fn parameter_index_mut(&mut self) -> &mut u8;
    fn target_offset(&self) -> &u16;
    fn target_offset_mut(&mut self) -> &mut u16;
}

impl SchematicsParameterPatchTrait for SchematicsParameterPatch {
    fn value(&self) -> &glacier_reflect::builtin::BoxedValueRef {
        &self.value
    }
    fn value_mut(&mut self) -> &mut glacier_reflect::builtin::BoxedValueRef {
        &mut self.value
    }
    fn parameter_index(&self) -> &u8 {
        &self.parameter_index
    }
    fn parameter_index_mut(&mut self) -> &mut u8 {
        &mut self.parameter_index
    }
    fn target_offset(&self) -> &u16 {
        &self.target_offset
    }
    fn target_offset_mut(&mut self) -> &mut u16 {
        &mut self.target_offset
    }
}

pub static SCHEMATICSPARAMETERPATCH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsParameterPatch",
    name_hash: 17287038,
    flags: MemberInfoFlags::new(73),
    module: "Schematics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SchematicsParameterPatch as Default>::default())),
            create_boxed: || Box::new(<SchematicsParameterPatch as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
                flags: MemberInfoFlags::new(0),
                field_type: "BoxedValueRef",
                rust_offset: offset_of!(SchematicsParameterPatch, value),
            },
            FieldInfoData {
                name: "ParameterIndex",
                name_hash: 241877522,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(SchematicsParameterPatch, parameter_index),
            },
            FieldInfoData {
                name: "TargetOffset",
                name_hash: 1634518457,
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


pub static SCHEMATICSPARAMETERPATCH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsParameterPatch-Array",
    name_hash: 1770539082,
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SchematicsParameterPatch"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SchematicsFieldPatch {
    pub value: glacier_reflect::builtin::BoxedValueRef,
    pub target_offset: u16,
}

pub trait SchematicsFieldPatchTrait: TypeObject {
    fn value(&self) -> &glacier_reflect::builtin::BoxedValueRef;
    fn value_mut(&mut self) -> &mut glacier_reflect::builtin::BoxedValueRef;
    fn target_offset(&self) -> &u16;
    fn target_offset_mut(&mut self) -> &mut u16;
}

impl SchematicsFieldPatchTrait for SchematicsFieldPatch {
    fn value(&self) -> &glacier_reflect::builtin::BoxedValueRef {
        &self.value
    }
    fn value_mut(&mut self) -> &mut glacier_reflect::builtin::BoxedValueRef {
        &mut self.value
    }
    fn target_offset(&self) -> &u16 {
        &self.target_offset
    }
    fn target_offset_mut(&mut self) -> &mut u16 {
        &mut self.target_offset
    }
}

pub static SCHEMATICSFIELDPATCH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsFieldPatch",
    name_hash: 14962165,
    flags: MemberInfoFlags::new(73),
    module: "Schematics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SchematicsFieldPatch as Default>::default())),
            create_boxed: || Box::new(<SchematicsFieldPatch as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
                flags: MemberInfoFlags::new(0),
                field_type: "BoxedValueRef",
                rust_offset: offset_of!(SchematicsFieldPatch, value),
            },
            FieldInfoData {
                name: "TargetOffset",
                name_hash: 1634518457,
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


pub static SCHEMATICSFIELDPATCH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsFieldPatch-Array",
    name_hash: 1645190849,
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SchematicsFieldPatch"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ConstField {
    pub value: glacier_reflect::builtin::BoxedValueRef,
}

pub trait ConstFieldTrait: TypeObject {
    fn value(&self) -> &glacier_reflect::builtin::BoxedValueRef;
    fn value_mut(&mut self) -> &mut glacier_reflect::builtin::BoxedValueRef;
}

impl ConstFieldTrait for ConstField {
    fn value(&self) -> &glacier_reflect::builtin::BoxedValueRef {
        &self.value
    }
    fn value_mut(&mut self) -> &mut glacier_reflect::builtin::BoxedValueRef {
        &mut self.value
    }
}

pub static CONSTFIELD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConstField",
    name_hash: 1879578114,
    flags: MemberInfoFlags::new(73),
    module: "Schematics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ConstField as Default>::default())),
            create_boxed: || Box::new(<ConstField as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
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


pub static CONSTFIELD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConstField-Array",
    name_hash: 3582686518,
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("ConstField"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AutoCreatedDispatcher {
    pub field_offset: u16,
    pub parameter_type: glacier_reflect::builtin::TypeRef,
}

pub trait AutoCreatedDispatcherTrait: TypeObject {
    fn field_offset(&self) -> &u16;
    fn field_offset_mut(&mut self) -> &mut u16;
    fn parameter_type(&self) -> &glacier_reflect::builtin::TypeRef;
    fn parameter_type_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef;
}

impl AutoCreatedDispatcherTrait for AutoCreatedDispatcher {
    fn field_offset(&self) -> &u16 {
        &self.field_offset
    }
    fn field_offset_mut(&mut self) -> &mut u16 {
        &mut self.field_offset
    }
    fn parameter_type(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.parameter_type
    }
    fn parameter_type_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef {
        &mut self.parameter_type
    }
}

pub static AUTOCREATEDDISPATCHER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoCreatedDispatcher",
    name_hash: 592348397,
    flags: MemberInfoFlags::new(73),
    module: "Schematics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutoCreatedDispatcher as Default>::default())),
            create_boxed: || Box::new(<AutoCreatedDispatcher as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "FieldOffset",
                name_hash: 2228247466,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(AutoCreatedDispatcher, field_offset),
            },
            FieldInfoData {
                name: "ParameterType",
                name_hash: 1569850964,
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


pub static AUTOCREATEDDISPATCHER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoCreatedDispatcher-Array",
    name_hash: 1643358681,
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("AutoCreatedDispatcher"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AutoCreatedField {
    pub field_offset: u16,
    pub asset: Option<LockedTypeObject /* SchematicsAsset */>,
    pub patch_data: Option<LockedTypeObject /* SchematicsPatchData */>,
}

pub trait AutoCreatedFieldTrait: TypeObject {
    fn field_offset(&self) -> &u16;
    fn field_offset_mut(&mut self) -> &mut u16;
    fn asset(&self) -> &Option<LockedTypeObject /* SchematicsAsset */>;
    fn asset_mut(&mut self) -> &mut Option<LockedTypeObject /* SchematicsAsset */>;
    fn patch_data(&self) -> &Option<LockedTypeObject /* SchematicsPatchData */>;
    fn patch_data_mut(&mut self) -> &mut Option<LockedTypeObject /* SchematicsPatchData */>;
}

impl AutoCreatedFieldTrait for AutoCreatedField {
    fn field_offset(&self) -> &u16 {
        &self.field_offset
    }
    fn field_offset_mut(&mut self) -> &mut u16 {
        &mut self.field_offset
    }
    fn asset(&self) -> &Option<LockedTypeObject /* SchematicsAsset */> {
        &self.asset
    }
    fn asset_mut(&mut self) -> &mut Option<LockedTypeObject /* SchematicsAsset */> {
        &mut self.asset
    }
    fn patch_data(&self) -> &Option<LockedTypeObject /* SchematicsPatchData */> {
        &self.patch_data
    }
    fn patch_data_mut(&mut self) -> &mut Option<LockedTypeObject /* SchematicsPatchData */> {
        &mut self.patch_data
    }
}

pub static AUTOCREATEDFIELD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoCreatedField",
    name_hash: 2502409896,
    flags: MemberInfoFlags::new(73),
    module: "Schematics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutoCreatedField as Default>::default())),
            create_boxed: || Box::new(<AutoCreatedField as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "FieldOffset",
                name_hash: 2228247466,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(AutoCreatedField, field_offset),
            },
            FieldInfoData {
                name: "Asset",
                name_hash: 205976053,
                flags: MemberInfoFlags::new(0),
                field_type: "SchematicsAsset",
                rust_offset: offset_of!(AutoCreatedField, asset),
            },
            FieldInfoData {
                name: "PatchData",
                name_hash: 158111963,
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


pub static AUTOCREATEDFIELD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoCreatedField-Array",
    name_hash: 2254045212,
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("AutoCreatedField"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EventObserverEntry {
    pub function: glacier_reflect::builtin::TypeRef,
    pub update_pass: Option<LockedTypeObject /* SchematicsUpdatePassAsset */>,
}

pub trait EventObserverEntryTrait: TypeObject {
    fn function(&self) -> &glacier_reflect::builtin::TypeRef;
    fn function_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef;
    fn update_pass(&self) -> &Option<LockedTypeObject /* SchematicsUpdatePassAsset */>;
    fn update_pass_mut(&mut self) -> &mut Option<LockedTypeObject /* SchematicsUpdatePassAsset */>;
}

impl EventObserverEntryTrait for EventObserverEntry {
    fn function(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.function
    }
    fn function_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef {
        &mut self.function
    }
    fn update_pass(&self) -> &Option<LockedTypeObject /* SchematicsUpdatePassAsset */> {
        &self.update_pass
    }
    fn update_pass_mut(&mut self) -> &mut Option<LockedTypeObject /* SchematicsUpdatePassAsset */> {
        &mut self.update_pass
    }
}

pub static EVENTOBSERVERENTRY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventObserverEntry",
    name_hash: 333314837,
    flags: MemberInfoFlags::new(73),
    module: "Schematics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EventObserverEntry as Default>::default())),
            create_boxed: || Box::new(<EventObserverEntry as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Function",
                name_hash: 4136871687,
                flags: MemberInfoFlags::new(0),
                field_type: "TypeRef",
                rust_offset: offset_of!(EventObserverEntry, function),
            },
            FieldInfoData {
                name: "UpdatePass",
                name_hash: 2270785669,
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


pub static EVENTOBSERVERENTRY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventObserverEntry-Array",
    name_hash: 812264097,
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("EventObserverEntry"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SchematicsAsset {
    pub _glacier_base: SchematicsBaseAsset,
    pub instance_type: glacier_reflect::builtin::TypeRef,
    pub constructor_function: glacier_reflect::builtin::TypeRef,
    pub destructor_function: glacier_reflect::builtin::TypeRef,
    pub tweaker_function: glacier_reflect::builtin::TypeRef,
    pub auto_created_fields: Vec<BoxedTypeObject /* AutoCreatedField */>,
    pub auto_created_dispatchers: Vec<BoxedTypeObject /* AutoCreatedDispatcher */>,
    pub patch_data: Option<LockedTypeObject /* SchematicsPatchData */>,
    pub observers: Vec<BoxedTypeObject /* EventObserverEntry */>,
    pub const_fields: Vec<BoxedTypeObject /* ConstField */>,
}

pub trait SchematicsAssetTrait: SchematicsBaseAssetTrait {
    fn instance_type(&self) -> &glacier_reflect::builtin::TypeRef;
    fn instance_type_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef;
    fn constructor_function(&self) -> &glacier_reflect::builtin::TypeRef;
    fn constructor_function_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef;
    fn destructor_function(&self) -> &glacier_reflect::builtin::TypeRef;
    fn destructor_function_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef;
    fn tweaker_function(&self) -> &glacier_reflect::builtin::TypeRef;
    fn tweaker_function_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef;
    fn auto_created_fields(&self) -> &Vec<BoxedTypeObject /* AutoCreatedField */>;
    fn auto_created_fields_mut(&mut self) -> &mut Vec<BoxedTypeObject /* AutoCreatedField */>;
    fn auto_created_dispatchers(&self) -> &Vec<BoxedTypeObject /* AutoCreatedDispatcher */>;
    fn auto_created_dispatchers_mut(&mut self) -> &mut Vec<BoxedTypeObject /* AutoCreatedDispatcher */>;
    fn patch_data(&self) -> &Option<LockedTypeObject /* SchematicsPatchData */>;
    fn patch_data_mut(&mut self) -> &mut Option<LockedTypeObject /* SchematicsPatchData */>;
    fn observers(&self) -> &Vec<BoxedTypeObject /* EventObserverEntry */>;
    fn observers_mut(&mut self) -> &mut Vec<BoxedTypeObject /* EventObserverEntry */>;
    fn const_fields(&self) -> &Vec<BoxedTypeObject /* ConstField */>;
    fn const_fields_mut(&mut self) -> &mut Vec<BoxedTypeObject /* ConstField */>;
}

impl SchematicsAssetTrait for SchematicsAsset {
    fn instance_type(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.instance_type
    }
    fn instance_type_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef {
        &mut self.instance_type
    }
    fn constructor_function(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.constructor_function
    }
    fn constructor_function_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef {
        &mut self.constructor_function
    }
    fn destructor_function(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.destructor_function
    }
    fn destructor_function_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef {
        &mut self.destructor_function
    }
    fn tweaker_function(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.tweaker_function
    }
    fn tweaker_function_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef {
        &mut self.tweaker_function
    }
    fn auto_created_fields(&self) -> &Vec<BoxedTypeObject /* AutoCreatedField */> {
        &self.auto_created_fields
    }
    fn auto_created_fields_mut(&mut self) -> &mut Vec<BoxedTypeObject /* AutoCreatedField */> {
        &mut self.auto_created_fields
    }
    fn auto_created_dispatchers(&self) -> &Vec<BoxedTypeObject /* AutoCreatedDispatcher */> {
        &self.auto_created_dispatchers
    }
    fn auto_created_dispatchers_mut(&mut self) -> &mut Vec<BoxedTypeObject /* AutoCreatedDispatcher */> {
        &mut self.auto_created_dispatchers
    }
    fn patch_data(&self) -> &Option<LockedTypeObject /* SchematicsPatchData */> {
        &self.patch_data
    }
    fn patch_data_mut(&mut self) -> &mut Option<LockedTypeObject /* SchematicsPatchData */> {
        &mut self.patch_data
    }
    fn observers(&self) -> &Vec<BoxedTypeObject /* EventObserverEntry */> {
        &self.observers
    }
    fn observers_mut(&mut self) -> &mut Vec<BoxedTypeObject /* EventObserverEntry */> {
        &mut self.observers
    }
    fn const_fields(&self) -> &Vec<BoxedTypeObject /* ConstField */> {
        &self.const_fields
    }
    fn const_fields_mut(&mut self) -> &mut Vec<BoxedTypeObject /* ConstField */> {
        &mut self.const_fields
    }
}

impl SchematicsBaseAssetTrait for SchematicsAsset {
}

impl super::core::AssetTrait for SchematicsAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for SchematicsAsset {
}

pub static SCHEMATICSASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsAsset",
    name_hash: 3227443945,
    flags: MemberInfoFlags::new(101),
    module: "Schematics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SCHEMATICSBASEASSET_TYPE_INFO),
        super_class_offset: offset_of!(SchematicsAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SchematicsAsset as Default>::default())),
            create_boxed: || Box::new(<SchematicsAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "InstanceType",
                name_hash: 1187858388,
                flags: MemberInfoFlags::new(0),
                field_type: "TypeRef",
                rust_offset: offset_of!(SchematicsAsset, instance_type),
            },
            FieldInfoData {
                name: "ConstructorFunction",
                name_hash: 525597295,
                flags: MemberInfoFlags::new(0),
                field_type: "TypeRef",
                rust_offset: offset_of!(SchematicsAsset, constructor_function),
            },
            FieldInfoData {
                name: "DestructorFunction",
                name_hash: 2669482188,
                flags: MemberInfoFlags::new(0),
                field_type: "TypeRef",
                rust_offset: offset_of!(SchematicsAsset, destructor_function),
            },
            FieldInfoData {
                name: "TweakerFunction",
                name_hash: 532040220,
                flags: MemberInfoFlags::new(0),
                field_type: "TypeRef",
                rust_offset: offset_of!(SchematicsAsset, tweaker_function),
            },
            FieldInfoData {
                name: "AutoCreatedFields",
                name_hash: 975147995,
                flags: MemberInfoFlags::new(144),
                field_type: "AutoCreatedField-Array",
                rust_offset: offset_of!(SchematicsAsset, auto_created_fields),
            },
            FieldInfoData {
                name: "AutoCreatedDispatchers",
                name_hash: 2367628030,
                flags: MemberInfoFlags::new(144),
                field_type: "AutoCreatedDispatcher-Array",
                rust_offset: offset_of!(SchematicsAsset, auto_created_dispatchers),
            },
            FieldInfoData {
                name: "PatchData",
                name_hash: 158111963,
                flags: MemberInfoFlags::new(0),
                field_type: "SchematicsPatchData",
                rust_offset: offset_of!(SchematicsAsset, patch_data),
            },
            FieldInfoData {
                name: "Observers",
                name_hash: 109672798,
                flags: MemberInfoFlags::new(144),
                field_type: "EventObserverEntry-Array",
                rust_offset: offset_of!(SchematicsAsset, observers),
            },
            FieldInfoData {
                name: "ConstFields",
                name_hash: 1896535601,
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


pub static SCHEMATICSASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsAsset-Array",
    name_hash: 1276303837,
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SchematicsAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SchematicsInstance {
}

pub trait SchematicsInstanceTrait: TypeObject {
}

impl SchematicsInstanceTrait for SchematicsInstance {
}

pub static SCHEMATICSINSTANCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsInstance",
    name_hash: 3404185328,
    flags: MemberInfoFlags::new(101),
    module: "Schematics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SchematicsInstance as Default>::default())),
            create_boxed: || Box::new(<SchematicsInstance as Default>::default()),
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


pub static SCHEMATICSINSTANCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsInstance-Array",
    name_hash: 3472399556,
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SchematicsInstance"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2574906386,
    flags: MemberInfoFlags::new(101),
    module: "Schematics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::EVENTDISPATCHER_TYPE_INFO),
        super_class_offset: offset_of!(SchematicsEventDispatcher, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SchematicsEventDispatcher as Default>::default())),
            create_boxed: || Box::new(<SchematicsEventDispatcher as Default>::default()),
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


pub static SCHEMATICSEVENTDISPATCHER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsEventDispatcher-Array",
    name_hash: 1610521382,
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SchematicsEventDispatcher"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SchematicsContext {
}

pub trait SchematicsContextTrait: TypeObject {
}

impl SchematicsContextTrait for SchematicsContext {
}

pub static SCHEMATICSCONTEXT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsContext",
    name_hash: 702616838,
    flags: MemberInfoFlags::new(101),
    module: "Schematics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SchematicsContext as Default>::default())),
            create_boxed: || Box::new(<SchematicsContext as Default>::default()),
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


pub static SCHEMATICSCONTEXT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsContext-Array",
    name_hash: 4112998962,
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SchematicsContext"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SchematicsPipelineBuilder {
}

pub trait SchematicsPipelineBuilderTrait: TypeObject {
}

impl SchematicsPipelineBuilderTrait for SchematicsPipelineBuilder {
}

pub static SCHEMATICSPIPELINEBUILDER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsPipelineBuilder",
    name_hash: 832808090,
    flags: MemberInfoFlags::new(101),
    module: "Schematics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SchematicsPipelineBuilder as Default>::default())),
            create_boxed: || Box::new(<SchematicsPipelineBuilder as Default>::default()),
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


pub static SCHEMATICSPIPELINEBUILDER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicsPipelineBuilder-Array",
    name_hash: 2085226414,
    flags: MemberInfoFlags::new(145),
    module: "Schematics",
    data: TypeInfoData::Array("SchematicsPipelineBuilder"),
    array_type: None,
    alignment: 8,
};


