use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_expression_types(registry: &mut TypeRegistry) {
    registry.register_type(EXPRESSIONCONSTANTEMPTYARRAYPATCH_TYPE_INFO);
    registry.register_type(EXPRESSIONCONSTANTEMPTYARRAYPATCH_ARRAY_TYPE_INFO);
    registry.register_type(EXPRESSIONCONSTANTREFERENCEPATCH_TYPE_INFO);
    registry.register_type(EXPRESSIONCONSTANTREFERENCEPATCH_ARRAY_TYPE_INFO);
    registry.register_type(EXPRESSIONNOPPATCH_TYPE_INFO);
    registry.register_type(EXPRESSIONNOPPATCH_ARRAY_TYPE_INFO);
    registry.register_type(EXPRESSIONSTATEDATA_TYPE_INFO);
    registry.register_type(EXPRESSIONSTATEDATA_ARRAY_TYPE_INFO);
    registry.register_type(EXPRESSIONPROPERTYDATA_TYPE_INFO);
    registry.register_type(EXPRESSIONPROPERTYDATA_ARRAY_TYPE_INFO);
    registry.register_type(EXPRESSIONPORTDATA_TYPE_INFO);
    registry.register_type(EXPRESSIONPORTDATA_ARRAY_TYPE_INFO);
    registry.register_type(EXPRESSIONPORTDIRECTION_TYPE_INFO);
    registry.register_type(EXPRESSIONPORTDIRECTION_ARRAY_TYPE_INFO);
    registry.register_type(EXPRESSIONNODEGRAPHDATA_TYPE_INFO);
    registry.register_type(EXPRESSIONNODEGRAPHDATA_ARRAY_TYPE_INFO);
    registry.register_type(REFERENCEDTYPE_TYPE_INFO);
    registry.register_type(REFERENCEDTYPE_ARRAY_TYPE_INFO);
    registry.register_type(EXPRESSIONFUNCTIONTYPEINFOASSET_TYPE_INFO);
    registry.register_type(EXPRESSIONFUNCTIONTYPEINFOASSET_ARRAY_TYPE_INFO);
    registry.register_type(SERIALIZEDEXPRESSIONNODEGRAPH_TYPE_INFO);
    registry.register_type(SERIALIZEDEXPRESSIONNODEGRAPH_ARRAY_TYPE_INFO);
    registry.register_type(EXPRESSIONRUNTIMECONTEXT_TYPE_INFO);
    registry.register_type(EXPRESSIONRUNTIMECONTEXT_ARRAY_TYPE_INFO);
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct ExpressionConstantEmptyArrayPatch {
    pub register_offset: u32,
    pub data_offset: u32,
}

pub trait ExpressionConstantEmptyArrayPatchTrait: TypeObject {
    fn register_offset(&self) -> &u32;
    fn register_offset_mut(&mut self) -> &mut u32;
    fn data_offset(&self) -> &u32;
    fn data_offset_mut(&mut self) -> &mut u32;
}

impl ExpressionConstantEmptyArrayPatchTrait for ExpressionConstantEmptyArrayPatch {
    fn register_offset(&self) -> &u32 {
        &self.register_offset
    }
    fn register_offset_mut(&mut self) -> &mut u32 {
        &mut self.register_offset
    }
    fn data_offset(&self) -> &u32 {
        &self.data_offset
    }
    fn data_offset_mut(&mut self) -> &mut u32 {
        &mut self.data_offset
    }
}

pub static EXPRESSIONCONSTANTEMPTYARRAYPATCH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionConstantEmptyArrayPatch",
    name_hash: 4255777323,
    flags: MemberInfoFlags::new(36937),
    module: "Expression",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ExpressionConstantEmptyArrayPatch as Default>::default())),
            create_boxed: || Box::new(<ExpressionConstantEmptyArrayPatch as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "RegisterOffset",
                name_hash: 4247693665,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ExpressionConstantEmptyArrayPatch, register_offset),
            },
            FieldInfoData {
                name: "DataOffset",
                name_hash: 289826904,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ExpressionConstantEmptyArrayPatch, data_offset),
            },
        ],
    }),
    array_type: Some(EXPRESSIONCONSTANTEMPTYARRAYPATCH_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for ExpressionConstantEmptyArrayPatch {
    fn type_info(&self) -> &'static TypeInfo {
        EXPRESSIONCONSTANTEMPTYARRAYPATCH_TYPE_INFO
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


pub static EXPRESSIONCONSTANTEMPTYARRAYPATCH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionConstantEmptyArrayPatch-Array",
    name_hash: 846242207,
    flags: MemberInfoFlags::new(145),
    module: "Expression",
    data: TypeInfoData::Array("ExpressionConstantEmptyArrayPatch"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ExpressionConstantReferencePatch {
    pub value: Option<LockedTypeObject /* super::core::DataContainer */>,
    pub register_offset: u32,
    pub data_offset: u32,
}

pub trait ExpressionConstantReferencePatchTrait: TypeObject {
    fn value(&self) -> &Option<LockedTypeObject /* super::core::DataContainer */>;
    fn value_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::DataContainer */>;
    fn register_offset(&self) -> &u32;
    fn register_offset_mut(&mut self) -> &mut u32;
    fn data_offset(&self) -> &u32;
    fn data_offset_mut(&mut self) -> &mut u32;
}

impl ExpressionConstantReferencePatchTrait for ExpressionConstantReferencePatch {
    fn value(&self) -> &Option<LockedTypeObject /* super::core::DataContainer */> {
        &self.value
    }
    fn value_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::DataContainer */> {
        &mut self.value
    }
    fn register_offset(&self) -> &u32 {
        &self.register_offset
    }
    fn register_offset_mut(&mut self) -> &mut u32 {
        &mut self.register_offset
    }
    fn data_offset(&self) -> &u32 {
        &self.data_offset
    }
    fn data_offset_mut(&mut self) -> &mut u32 {
        &mut self.data_offset
    }
}

pub static EXPRESSIONCONSTANTREFERENCEPATCH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionConstantReferencePatch",
    name_hash: 3353193868,
    flags: MemberInfoFlags::new(73),
    module: "Expression",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ExpressionConstantReferencePatch as Default>::default())),
            create_boxed: || Box::new(<ExpressionConstantReferencePatch as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
                flags: MemberInfoFlags::new(0),
                field_type: "DataContainer",
                rust_offset: offset_of!(ExpressionConstantReferencePatch, value),
            },
            FieldInfoData {
                name: "RegisterOffset",
                name_hash: 4247693665,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ExpressionConstantReferencePatch, register_offset),
            },
            FieldInfoData {
                name: "DataOffset",
                name_hash: 289826904,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ExpressionConstantReferencePatch, data_offset),
            },
        ],
    }),
    array_type: Some(EXPRESSIONCONSTANTREFERENCEPATCH_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ExpressionConstantReferencePatch {
    fn type_info(&self) -> &'static TypeInfo {
        EXPRESSIONCONSTANTREFERENCEPATCH_TYPE_INFO
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


pub static EXPRESSIONCONSTANTREFERENCEPATCH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionConstantReferencePatch-Array",
    name_hash: 3660021176,
    flags: MemberInfoFlags::new(145),
    module: "Expression",
    data: TypeInfoData::Array("ExpressionConstantReferencePatch"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ExpressionNopPatch {
    pub value: glacier_reflect::builtin::TypeRef,
    pub register_offset: u32,
}

pub trait ExpressionNopPatchTrait: TypeObject {
    fn value(&self) -> &glacier_reflect::builtin::TypeRef;
    fn value_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef;
    fn register_offset(&self) -> &u32;
    fn register_offset_mut(&mut self) -> &mut u32;
}

impl ExpressionNopPatchTrait for ExpressionNopPatch {
    fn value(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.value
    }
    fn value_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef {
        &mut self.value
    }
    fn register_offset(&self) -> &u32 {
        &self.register_offset
    }
    fn register_offset_mut(&mut self) -> &mut u32 {
        &mut self.register_offset
    }
}

pub static EXPRESSIONNOPPATCH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionNopPatch",
    name_hash: 4157606248,
    flags: MemberInfoFlags::new(73),
    module: "Expression",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ExpressionNopPatch as Default>::default())),
            create_boxed: || Box::new(<ExpressionNopPatch as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
                flags: MemberInfoFlags::new(0),
                field_type: "TypeRef",
                rust_offset: offset_of!(ExpressionNopPatch, value),
            },
            FieldInfoData {
                name: "RegisterOffset",
                name_hash: 4247693665,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ExpressionNopPatch, register_offset),
            },
        ],
    }),
    array_type: Some(EXPRESSIONNOPPATCH_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ExpressionNopPatch {
    fn type_info(&self) -> &'static TypeInfo {
        EXPRESSIONNOPPATCH_TYPE_INFO
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


pub static EXPRESSIONNOPPATCH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionNopPatch-Array",
    name_hash: 1333484380,
    flags: MemberInfoFlags::new(145),
    module: "Expression",
    data: TypeInfoData::Array("ExpressionNopPatch"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ExpressionStateData {
    pub graph_port_offset: u32,
    pub port_type: glacier_reflect::builtin::TypeRef,
}

pub trait ExpressionStateDataTrait: TypeObject {
    fn graph_port_offset(&self) -> &u32;
    fn graph_port_offset_mut(&mut self) -> &mut u32;
    fn port_type(&self) -> &glacier_reflect::builtin::TypeRef;
    fn port_type_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef;
}

impl ExpressionStateDataTrait for ExpressionStateData {
    fn graph_port_offset(&self) -> &u32 {
        &self.graph_port_offset
    }
    fn graph_port_offset_mut(&mut self) -> &mut u32 {
        &mut self.graph_port_offset
    }
    fn port_type(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.port_type
    }
    fn port_type_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef {
        &mut self.port_type
    }
}

pub static EXPRESSIONSTATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionStateData",
    name_hash: 602274672,
    flags: MemberInfoFlags::new(73),
    module: "Expression",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ExpressionStateData as Default>::default())),
            create_boxed: || Box::new(<ExpressionStateData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "GraphPortOffset",
                name_hash: 3218825597,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ExpressionStateData, graph_port_offset),
            },
            FieldInfoData {
                name: "PortType",
                name_hash: 3471149956,
                flags: MemberInfoFlags::new(0),
                field_type: "TypeRef",
                rust_offset: offset_of!(ExpressionStateData, port_type),
            },
        ],
    }),
    array_type: Some(EXPRESSIONSTATEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ExpressionStateData {
    fn type_info(&self) -> &'static TypeInfo {
        EXPRESSIONSTATEDATA_TYPE_INFO
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


pub static EXPRESSIONSTATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionStateData-Array",
    name_hash: 2446473540,
    flags: MemberInfoFlags::new(145),
    module: "Expression",
    data: TypeInfoData::Array("ExpressionStateData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ExpressionPropertyData {
    pub property_name_hash: u32,
    pub graph_port_offset: u32,
    pub port_type: glacier_reflect::builtin::TypeRef,
    pub is_reference: bool,
}

pub trait ExpressionPropertyDataTrait: TypeObject {
    fn property_name_hash(&self) -> &u32;
    fn property_name_hash_mut(&mut self) -> &mut u32;
    fn graph_port_offset(&self) -> &u32;
    fn graph_port_offset_mut(&mut self) -> &mut u32;
    fn port_type(&self) -> &glacier_reflect::builtin::TypeRef;
    fn port_type_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef;
    fn is_reference(&self) -> &bool;
    fn is_reference_mut(&mut self) -> &mut bool;
}

impl ExpressionPropertyDataTrait for ExpressionPropertyData {
    fn property_name_hash(&self) -> &u32 {
        &self.property_name_hash
    }
    fn property_name_hash_mut(&mut self) -> &mut u32 {
        &mut self.property_name_hash
    }
    fn graph_port_offset(&self) -> &u32 {
        &self.graph_port_offset
    }
    fn graph_port_offset_mut(&mut self) -> &mut u32 {
        &mut self.graph_port_offset
    }
    fn port_type(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.port_type
    }
    fn port_type_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef {
        &mut self.port_type
    }
    fn is_reference(&self) -> &bool {
        &self.is_reference
    }
    fn is_reference_mut(&mut self) -> &mut bool {
        &mut self.is_reference
    }
}

pub static EXPRESSIONPROPERTYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionPropertyData",
    name_hash: 1414954656,
    flags: MemberInfoFlags::new(73),
    module: "Expression",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ExpressionPropertyData as Default>::default())),
            create_boxed: || Box::new(<ExpressionPropertyData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "PropertyNameHash",
                name_hash: 1411787191,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ExpressionPropertyData, property_name_hash),
            },
            FieldInfoData {
                name: "GraphPortOffset",
                name_hash: 3218825597,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ExpressionPropertyData, graph_port_offset),
            },
            FieldInfoData {
                name: "PortType",
                name_hash: 3471149956,
                flags: MemberInfoFlags::new(0),
                field_type: "TypeRef",
                rust_offset: offset_of!(ExpressionPropertyData, port_type),
            },
            FieldInfoData {
                name: "IsReference",
                name_hash: 2152857716,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ExpressionPropertyData, is_reference),
            },
        ],
    }),
    array_type: Some(EXPRESSIONPROPERTYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ExpressionPropertyData {
    fn type_info(&self) -> &'static TypeInfo {
        EXPRESSIONPROPERTYDATA_TYPE_INFO
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


pub static EXPRESSIONPROPERTYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionPropertyData-Array",
    name_hash: 3675874324,
    flags: MemberInfoFlags::new(145),
    module: "Expression",
    data: TypeInfoData::Array("ExpressionPropertyData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ExpressionPortData {
    pub port_key: u64,
    pub node_name_hash: u32,
    pub port_name_hash: u32,
    pub graph_port_offset: u32,
    pub port_direction: ExpressionPortDirection,
    pub port_type: glacier_reflect::builtin::TypeRef,
}

pub trait ExpressionPortDataTrait: TypeObject {
    fn port_key(&self) -> &u64;
    fn port_key_mut(&mut self) -> &mut u64;
    fn node_name_hash(&self) -> &u32;
    fn node_name_hash_mut(&mut self) -> &mut u32;
    fn port_name_hash(&self) -> &u32;
    fn port_name_hash_mut(&mut self) -> &mut u32;
    fn graph_port_offset(&self) -> &u32;
    fn graph_port_offset_mut(&mut self) -> &mut u32;
    fn port_direction(&self) -> &ExpressionPortDirection;
    fn port_direction_mut(&mut self) -> &mut ExpressionPortDirection;
    fn port_type(&self) -> &glacier_reflect::builtin::TypeRef;
    fn port_type_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef;
}

impl ExpressionPortDataTrait for ExpressionPortData {
    fn port_key(&self) -> &u64 {
        &self.port_key
    }
    fn port_key_mut(&mut self) -> &mut u64 {
        &mut self.port_key
    }
    fn node_name_hash(&self) -> &u32 {
        &self.node_name_hash
    }
    fn node_name_hash_mut(&mut self) -> &mut u32 {
        &mut self.node_name_hash
    }
    fn port_name_hash(&self) -> &u32 {
        &self.port_name_hash
    }
    fn port_name_hash_mut(&mut self) -> &mut u32 {
        &mut self.port_name_hash
    }
    fn graph_port_offset(&self) -> &u32 {
        &self.graph_port_offset
    }
    fn graph_port_offset_mut(&mut self) -> &mut u32 {
        &mut self.graph_port_offset
    }
    fn port_direction(&self) -> &ExpressionPortDirection {
        &self.port_direction
    }
    fn port_direction_mut(&mut self) -> &mut ExpressionPortDirection {
        &mut self.port_direction
    }
    fn port_type(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.port_type
    }
    fn port_type_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef {
        &mut self.port_type
    }
}

pub static EXPRESSIONPORTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionPortData",
    name_hash: 1465409342,
    flags: MemberInfoFlags::new(73),
    module: "Expression",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ExpressionPortData as Default>::default())),
            create_boxed: || Box::new(<ExpressionPortData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "PortKey",
                name_hash: 4270021003,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint64",
                rust_offset: offset_of!(ExpressionPortData, port_key),
            },
            FieldInfoData {
                name: "NodeNameHash",
                name_hash: 2504890800,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ExpressionPortData, node_name_hash),
            },
            FieldInfoData {
                name: "PortNameHash",
                name_hash: 836661545,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ExpressionPortData, port_name_hash),
            },
            FieldInfoData {
                name: "GraphPortOffset",
                name_hash: 3218825597,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ExpressionPortData, graph_port_offset),
            },
            FieldInfoData {
                name: "PortDirection",
                name_hash: 2787370009,
                flags: MemberInfoFlags::new(0),
                field_type: "ExpressionPortDirection",
                rust_offset: offset_of!(ExpressionPortData, port_direction),
            },
            FieldInfoData {
                name: "PortType",
                name_hash: 3471149956,
                flags: MemberInfoFlags::new(0),
                field_type: "TypeRef",
                rust_offset: offset_of!(ExpressionPortData, port_type),
            },
        ],
    }),
    array_type: Some(EXPRESSIONPORTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ExpressionPortData {
    fn type_info(&self) -> &'static TypeInfo {
        EXPRESSIONPORTDATA_TYPE_INFO
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


pub static EXPRESSIONPORTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionPortData-Array",
    name_hash: 2594959242,
    flags: MemberInfoFlags::new(145),
    module: "Expression",
    data: TypeInfoData::Array("ExpressionPortData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ExpressionPortDirection {
    #[default]
    ExpressionPortDirection_In = 0,
    ExpressionPortDirection_Out = 1,
}

pub static EXPRESSIONPORTDIRECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionPortDirection",
    name_hash: 3593761579,
    flags: MemberInfoFlags::new(49429),
    module: "Expression",
    data: TypeInfoData::Enum,
    array_type: Some(EXPRESSIONPORTDIRECTION_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ExpressionPortDirection {
    fn type_info(&self) -> &'static TypeInfo {
        EXPRESSIONPORTDIRECTION_TYPE_INFO
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


pub static EXPRESSIONPORTDIRECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionPortDirection-Array",
    name_hash: 1061933727,
    flags: MemberInfoFlags::new(145),
    module: "Expression",
    data: TypeInfoData::Array("ExpressionPortDirection"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ExpressionNodeGraphData {
    pub _glacier_base: super::core::DataContainer,
    pub ports: Vec<BoxedTypeObject /* ExpressionPortData */>,
    pub inputs: Vec<BoxedTypeObject /* ExpressionPropertyData */>,
    pub outputs: Vec<BoxedTypeObject /* ExpressionPropertyData */>,
    pub states: Vec<BoxedTypeObject /* ExpressionStateData */>,
    pub resource: glacier_reflect::builtin::ResourceRef,
    pub nop_patches: Vec<BoxedTypeObject /* ExpressionNopPatch */>,
    pub constant_reference_patches: Vec<BoxedTypeObject /* ExpressionConstantReferencePatch */>,
    pub constant_empty_array_patches: Vec<BoxedTypeObject /* ExpressionConstantEmptyArrayPatch */>,
}

pub trait ExpressionNodeGraphDataTrait: super::core::DataContainerTrait {
    fn ports(&self) -> &Vec<BoxedTypeObject /* ExpressionPortData */>;
    fn ports_mut(&mut self) -> &mut Vec<BoxedTypeObject /* ExpressionPortData */>;
    fn inputs(&self) -> &Vec<BoxedTypeObject /* ExpressionPropertyData */>;
    fn inputs_mut(&mut self) -> &mut Vec<BoxedTypeObject /* ExpressionPropertyData */>;
    fn outputs(&self) -> &Vec<BoxedTypeObject /* ExpressionPropertyData */>;
    fn outputs_mut(&mut self) -> &mut Vec<BoxedTypeObject /* ExpressionPropertyData */>;
    fn states(&self) -> &Vec<BoxedTypeObject /* ExpressionStateData */>;
    fn states_mut(&mut self) -> &mut Vec<BoxedTypeObject /* ExpressionStateData */>;
    fn resource(&self) -> &glacier_reflect::builtin::ResourceRef;
    fn resource_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef;
    fn nop_patches(&self) -> &Vec<BoxedTypeObject /* ExpressionNopPatch */>;
    fn nop_patches_mut(&mut self) -> &mut Vec<BoxedTypeObject /* ExpressionNopPatch */>;
    fn constant_reference_patches(&self) -> &Vec<BoxedTypeObject /* ExpressionConstantReferencePatch */>;
    fn constant_reference_patches_mut(&mut self) -> &mut Vec<BoxedTypeObject /* ExpressionConstantReferencePatch */>;
    fn constant_empty_array_patches(&self) -> &Vec<BoxedTypeObject /* ExpressionConstantEmptyArrayPatch */>;
    fn constant_empty_array_patches_mut(&mut self) -> &mut Vec<BoxedTypeObject /* ExpressionConstantEmptyArrayPatch */>;
}

impl ExpressionNodeGraphDataTrait for ExpressionNodeGraphData {
    fn ports(&self) -> &Vec<BoxedTypeObject /* ExpressionPortData */> {
        &self.ports
    }
    fn ports_mut(&mut self) -> &mut Vec<BoxedTypeObject /* ExpressionPortData */> {
        &mut self.ports
    }
    fn inputs(&self) -> &Vec<BoxedTypeObject /* ExpressionPropertyData */> {
        &self.inputs
    }
    fn inputs_mut(&mut self) -> &mut Vec<BoxedTypeObject /* ExpressionPropertyData */> {
        &mut self.inputs
    }
    fn outputs(&self) -> &Vec<BoxedTypeObject /* ExpressionPropertyData */> {
        &self.outputs
    }
    fn outputs_mut(&mut self) -> &mut Vec<BoxedTypeObject /* ExpressionPropertyData */> {
        &mut self.outputs
    }
    fn states(&self) -> &Vec<BoxedTypeObject /* ExpressionStateData */> {
        &self.states
    }
    fn states_mut(&mut self) -> &mut Vec<BoxedTypeObject /* ExpressionStateData */> {
        &mut self.states
    }
    fn resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.resource
    }
    fn resource_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef {
        &mut self.resource
    }
    fn nop_patches(&self) -> &Vec<BoxedTypeObject /* ExpressionNopPatch */> {
        &self.nop_patches
    }
    fn nop_patches_mut(&mut self) -> &mut Vec<BoxedTypeObject /* ExpressionNopPatch */> {
        &mut self.nop_patches
    }
    fn constant_reference_patches(&self) -> &Vec<BoxedTypeObject /* ExpressionConstantReferencePatch */> {
        &self.constant_reference_patches
    }
    fn constant_reference_patches_mut(&mut self) -> &mut Vec<BoxedTypeObject /* ExpressionConstantReferencePatch */> {
        &mut self.constant_reference_patches
    }
    fn constant_empty_array_patches(&self) -> &Vec<BoxedTypeObject /* ExpressionConstantEmptyArrayPatch */> {
        &self.constant_empty_array_patches
    }
    fn constant_empty_array_patches_mut(&mut self) -> &mut Vec<BoxedTypeObject /* ExpressionConstantEmptyArrayPatch */> {
        &mut self.constant_empty_array_patches
    }
}

impl super::core::DataContainerTrait for ExpressionNodeGraphData {
}

pub static EXPRESSIONNODEGRAPHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionNodeGraphData",
    name_hash: 1382756619,
    flags: MemberInfoFlags::new(101),
    module: "Expression",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(ExpressionNodeGraphData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ExpressionNodeGraphData as Default>::default())),
            create_boxed: || Box::new(<ExpressionNodeGraphData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Ports",
                name_hash: 232670415,
                flags: MemberInfoFlags::new(144),
                field_type: "ExpressionPortData-Array",
                rust_offset: offset_of!(ExpressionNodeGraphData, ports),
            },
            FieldInfoData {
                name: "Inputs",
                name_hash: 2784267136,
                flags: MemberInfoFlags::new(144),
                field_type: "ExpressionPropertyData-Array",
                rust_offset: offset_of!(ExpressionNodeGraphData, inputs),
            },
            FieldInfoData {
                name: "Outputs",
                name_hash: 1070022089,
                flags: MemberInfoFlags::new(144),
                field_type: "ExpressionPropertyData-Array",
                rust_offset: offset_of!(ExpressionNodeGraphData, outputs),
            },
            FieldInfoData {
                name: "States",
                name_hash: 3319729985,
                flags: MemberInfoFlags::new(144),
                field_type: "ExpressionStateData-Array",
                rust_offset: offset_of!(ExpressionNodeGraphData, states),
            },
            FieldInfoData {
                name: "Resource",
                name_hash: 74513935,
                flags: MemberInfoFlags::new(0),
                field_type: "ResourceRef",
                rust_offset: offset_of!(ExpressionNodeGraphData, resource),
            },
            FieldInfoData {
                name: "NopPatches",
                name_hash: 3118327852,
                flags: MemberInfoFlags::new(144),
                field_type: "ExpressionNopPatch-Array",
                rust_offset: offset_of!(ExpressionNodeGraphData, nop_patches),
            },
            FieldInfoData {
                name: "ConstantReferencePatches",
                name_hash: 2936288136,
                flags: MemberInfoFlags::new(144),
                field_type: "ExpressionConstantReferencePatch-Array",
                rust_offset: offset_of!(ExpressionNodeGraphData, constant_reference_patches),
            },
            FieldInfoData {
                name: "ConstantEmptyArrayPatches",
                name_hash: 3332690095,
                flags: MemberInfoFlags::new(144),
                field_type: "ExpressionConstantEmptyArrayPatch-Array",
                rust_offset: offset_of!(ExpressionNodeGraphData, constant_empty_array_patches),
            },
        ],
    }),
    array_type: Some(EXPRESSIONNODEGRAPHDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ExpressionNodeGraphData {
    fn type_info(&self) -> &'static TypeInfo {
        EXPRESSIONNODEGRAPHDATA_TYPE_INFO
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


pub static EXPRESSIONNODEGRAPHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionNodeGraphData-Array",
    name_hash: 426590143,
    flags: MemberInfoFlags::new(145),
    module: "Expression",
    data: TypeInfoData::Array("ExpressionNodeGraphData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ReferencedType {
    pub blox_data_type_hash: u32,
    pub type_info: glacier_reflect::builtin::TypeRef,
}

pub trait ReferencedTypeTrait: TypeObject {
    fn blox_data_type_hash(&self) -> &u32;
    fn blox_data_type_hash_mut(&mut self) -> &mut u32;
    fn type_info(&self) -> &glacier_reflect::builtin::TypeRef;
    fn type_info_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef;
}

impl ReferencedTypeTrait for ReferencedType {
    fn blox_data_type_hash(&self) -> &u32 {
        &self.blox_data_type_hash
    }
    fn blox_data_type_hash_mut(&mut self) -> &mut u32 {
        &mut self.blox_data_type_hash
    }
    fn type_info(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.type_info
    }
    fn type_info_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef {
        &mut self.type_info
    }
}

pub static REFERENCEDTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReferencedType",
    name_hash: 3667949874,
    flags: MemberInfoFlags::new(73),
    module: "Expression",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ReferencedType as Default>::default())),
            create_boxed: || Box::new(<ReferencedType as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "BloxDataTypeHash",
                name_hash: 2952898822,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ReferencedType, blox_data_type_hash),
            },
            FieldInfoData {
                name: "TypeInfo",
                name_hash: 351127923,
                flags: MemberInfoFlags::new(0),
                field_type: "TypeRef",
                rust_offset: offset_of!(ReferencedType, type_info),
            },
        ],
    }),
    array_type: Some(REFERENCEDTYPE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ReferencedType {
    fn type_info(&self) -> &'static TypeInfo {
        REFERENCEDTYPE_TYPE_INFO
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


pub static REFERENCEDTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReferencedType-Array",
    name_hash: 2053144454,
    flags: MemberInfoFlags::new(145),
    module: "Expression",
    data: TypeInfoData::Array("ReferencedType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ExpressionFunctionTypeInfoAsset {
    pub _glacier_base: super::core::FunctionTypeInfoAsset,
    pub graph_data: Option<LockedTypeObject /* ExpressionNodeGraphData */>,
}

pub trait ExpressionFunctionTypeInfoAssetTrait: super::core::FunctionTypeInfoAssetTrait {
    fn graph_data(&self) -> &Option<LockedTypeObject /* ExpressionNodeGraphData */>;
    fn graph_data_mut(&mut self) -> &mut Option<LockedTypeObject /* ExpressionNodeGraphData */>;
}

impl ExpressionFunctionTypeInfoAssetTrait for ExpressionFunctionTypeInfoAsset {
    fn graph_data(&self) -> &Option<LockedTypeObject /* ExpressionNodeGraphData */> {
        &self.graph_data
    }
    fn graph_data_mut(&mut self) -> &mut Option<LockedTypeObject /* ExpressionNodeGraphData */> {
        &mut self.graph_data
    }
}

impl super::core::FunctionTypeInfoAssetTrait for ExpressionFunctionTypeInfoAsset {
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

impl super::core::TypeInfoAssetTrait for ExpressionFunctionTypeInfoAsset {
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

impl super::core::AssetTrait for ExpressionFunctionTypeInfoAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for ExpressionFunctionTypeInfoAsset {
}

pub static EXPRESSIONFUNCTIONTYPEINFOASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionFunctionTypeInfoAsset",
    name_hash: 2844968051,
    flags: MemberInfoFlags::new(101),
    module: "Expression",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::FUNCTIONTYPEINFOASSET_TYPE_INFO),
        super_class_offset: offset_of!(ExpressionFunctionTypeInfoAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ExpressionFunctionTypeInfoAsset as Default>::default())),
            create_boxed: || Box::new(<ExpressionFunctionTypeInfoAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "GraphData",
                name_hash: 1669336409,
                flags: MemberInfoFlags::new(0),
                field_type: "ExpressionNodeGraphData",
                rust_offset: offset_of!(ExpressionFunctionTypeInfoAsset, graph_data),
            },
        ],
    }),
    array_type: Some(EXPRESSIONFUNCTIONTYPEINFOASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ExpressionFunctionTypeInfoAsset {
    fn type_info(&self) -> &'static TypeInfo {
        EXPRESSIONFUNCTIONTYPEINFOASSET_TYPE_INFO
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


pub static EXPRESSIONFUNCTIONTYPEINFOASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionFunctionTypeInfoAsset-Array",
    name_hash: 2731114567,
    flags: MemberInfoFlags::new(145),
    module: "Expression",
    data: TypeInfoData::Array("ExpressionFunctionTypeInfoAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SerializedExpressionNodeGraph {
}

pub trait SerializedExpressionNodeGraphTrait: TypeObject {
}

impl SerializedExpressionNodeGraphTrait for SerializedExpressionNodeGraph {
}

pub static SERIALIZEDEXPRESSIONNODEGRAPH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SerializedExpressionNodeGraph",
    name_hash: 2327728009,
    flags: MemberInfoFlags::new(101),
    module: "Expression",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SerializedExpressionNodeGraph as Default>::default())),
            create_boxed: || Box::new(<SerializedExpressionNodeGraph as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERIALIZEDEXPRESSIONNODEGRAPH_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SerializedExpressionNodeGraph {
    fn type_info(&self) -> &'static TypeInfo {
        SERIALIZEDEXPRESSIONNODEGRAPH_TYPE_INFO
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


pub static SERIALIZEDEXPRESSIONNODEGRAPH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SerializedExpressionNodeGraph-Array",
    name_hash: 973391165,
    flags: MemberInfoFlags::new(145),
    module: "Expression",
    data: TypeInfoData::Array("SerializedExpressionNodeGraph"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ExpressionRuntimeContext {
}

pub trait ExpressionRuntimeContextTrait: TypeObject {
}

impl ExpressionRuntimeContextTrait for ExpressionRuntimeContext {
}

pub static EXPRESSIONRUNTIMECONTEXT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionRuntimeContext",
    name_hash: 3571789524,
    flags: MemberInfoFlags::new(101),
    module: "Expression",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ExpressionRuntimeContext as Default>::default())),
            create_boxed: || Box::new(<ExpressionRuntimeContext as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(EXPRESSIONRUNTIMECONTEXT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ExpressionRuntimeContext {
    fn type_info(&self) -> &'static TypeInfo {
        EXPRESSIONRUNTIMECONTEXT_TYPE_INFO
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


pub static EXPRESSIONRUNTIMECONTEXT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionRuntimeContext-Array",
    name_hash: 1030904672,
    flags: MemberInfoFlags::new(145),
    module: "Expression",
    data: TypeInfoData::Array("ExpressionRuntimeContext"),
    array_type: None,
    alignment: 8,
};


