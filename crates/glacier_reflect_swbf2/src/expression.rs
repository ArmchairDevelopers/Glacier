use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct ExpressionConstantEmptyArrayPatch {
    pub register_offset: u32,
    pub data_offset: u32,
}

pub trait ExpressionConstantEmptyArrayPatchTrait: TypeObject {
    fn register_offset(&self) -> &u32;
    fn data_offset(&self) -> &u32;
}

impl ExpressionConstantEmptyArrayPatchTrait for ExpressionConstantEmptyArrayPatch {
    fn register_offset(&self) -> &u32 {
        &self.register_offset
    }
    fn data_offset(&self) -> &u32 {
        &self.data_offset
    }
}

pub static EXPRESSIONCONSTANTEMPTYARRAYPATCH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionConstantEmptyArrayPatch",
    flags: MemberInfoFlags::new(36937),
    module: "Expression",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ExpressionConstantEmptyArrayPatch as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "RegisterOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ExpressionConstantEmptyArrayPatch, register_offset),
            },
            FieldInfoData {
                name: "DataOffset",
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
}


pub static EXPRESSIONCONSTANTEMPTYARRAYPATCH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionConstantEmptyArrayPatch-Array",
    flags: MemberInfoFlags::new(145),
    module: "Expression",
    data: TypeInfoData::Array("ExpressionConstantEmptyArrayPatch"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ExpressionConstantReferencePatch {
    pub value: Option<Arc<Mutex<dyn super::core::DataContainerTrait>>>,
    pub register_offset: u32,
    pub data_offset: u32,
}

pub trait ExpressionConstantReferencePatchTrait: TypeObject {
    fn value(&self) -> &Option<Arc<Mutex<dyn super::core::DataContainerTrait>>>;
    fn register_offset(&self) -> &u32;
    fn data_offset(&self) -> &u32;
}

impl ExpressionConstantReferencePatchTrait for ExpressionConstantReferencePatch {
    fn value(&self) -> &Option<Arc<Mutex<dyn super::core::DataContainerTrait>>> {
        &self.value
    }
    fn register_offset(&self) -> &u32 {
        &self.register_offset
    }
    fn data_offset(&self) -> &u32 {
        &self.data_offset
    }
}

pub static EXPRESSIONCONSTANTREFERENCEPATCH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionConstantReferencePatch",
    flags: MemberInfoFlags::new(73),
    module: "Expression",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ExpressionConstantReferencePatch as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "DataContainer",
                rust_offset: offset_of!(ExpressionConstantReferencePatch, value),
            },
            FieldInfoData {
                name: "RegisterOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ExpressionConstantReferencePatch, register_offset),
            },
            FieldInfoData {
                name: "DataOffset",
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
}


pub static EXPRESSIONCONSTANTREFERENCEPATCH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionConstantReferencePatch-Array",
    flags: MemberInfoFlags::new(145),
    module: "Expression",
    data: TypeInfoData::Array("ExpressionConstantReferencePatch"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ExpressionNopPatch {
    pub value: glacier_reflect::builtin::TypeRef,
    pub register_offset: u32,
}

pub trait ExpressionNopPatchTrait: TypeObject {
    fn value(&self) -> &glacier_reflect::builtin::TypeRef;
    fn register_offset(&self) -> &u32;
}

impl ExpressionNopPatchTrait for ExpressionNopPatch {
    fn value(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.value
    }
    fn register_offset(&self) -> &u32 {
        &self.register_offset
    }
}

pub static EXPRESSIONNOPPATCH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionNopPatch",
    flags: MemberInfoFlags::new(73),
    module: "Expression",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ExpressionNopPatch as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "TypeRef",
                rust_offset: offset_of!(ExpressionNopPatch, value),
            },
            FieldInfoData {
                name: "RegisterOffset",
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
}


pub static EXPRESSIONNOPPATCH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionNopPatch-Array",
    flags: MemberInfoFlags::new(145),
    module: "Expression",
    data: TypeInfoData::Array("ExpressionNopPatch"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ExpressionStateData {
    pub graph_port_offset: u32,
    pub port_type: glacier_reflect::builtin::TypeRef,
}

pub trait ExpressionStateDataTrait: TypeObject {
    fn graph_port_offset(&self) -> &u32;
    fn port_type(&self) -> &glacier_reflect::builtin::TypeRef;
}

impl ExpressionStateDataTrait for ExpressionStateData {
    fn graph_port_offset(&self) -> &u32 {
        &self.graph_port_offset
    }
    fn port_type(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.port_type
    }
}

pub static EXPRESSIONSTATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionStateData",
    flags: MemberInfoFlags::new(73),
    module: "Expression",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ExpressionStateData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "GraphPortOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ExpressionStateData, graph_port_offset),
            },
            FieldInfoData {
                name: "PortType",
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
}


pub static EXPRESSIONSTATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionStateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Expression",
    data: TypeInfoData::Array("ExpressionStateData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ExpressionPropertyData {
    pub property_name_hash: u32,
    pub graph_port_offset: u32,
    pub port_type: glacier_reflect::builtin::TypeRef,
    pub is_reference: bool,
}

pub trait ExpressionPropertyDataTrait: TypeObject {
    fn property_name_hash(&self) -> &u32;
    fn graph_port_offset(&self) -> &u32;
    fn port_type(&self) -> &glacier_reflect::builtin::TypeRef;
    fn is_reference(&self) -> &bool;
}

impl ExpressionPropertyDataTrait for ExpressionPropertyData {
    fn property_name_hash(&self) -> &u32 {
        &self.property_name_hash
    }
    fn graph_port_offset(&self) -> &u32 {
        &self.graph_port_offset
    }
    fn port_type(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.port_type
    }
    fn is_reference(&self) -> &bool {
        &self.is_reference
    }
}

pub static EXPRESSIONPROPERTYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionPropertyData",
    flags: MemberInfoFlags::new(73),
    module: "Expression",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ExpressionPropertyData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "PropertyNameHash",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ExpressionPropertyData, property_name_hash),
            },
            FieldInfoData {
                name: "GraphPortOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ExpressionPropertyData, graph_port_offset),
            },
            FieldInfoData {
                name: "PortType",
                flags: MemberInfoFlags::new(0),
                field_type: "TypeRef",
                rust_offset: offset_of!(ExpressionPropertyData, port_type),
            },
            FieldInfoData {
                name: "IsReference",
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
}


pub static EXPRESSIONPROPERTYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionPropertyData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Expression",
    data: TypeInfoData::Array("ExpressionPropertyData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    fn node_name_hash(&self) -> &u32;
    fn port_name_hash(&self) -> &u32;
    fn graph_port_offset(&self) -> &u32;
    fn port_direction(&self) -> &ExpressionPortDirection;
    fn port_type(&self) -> &glacier_reflect::builtin::TypeRef;
}

impl ExpressionPortDataTrait for ExpressionPortData {
    fn port_key(&self) -> &u64 {
        &self.port_key
    }
    fn node_name_hash(&self) -> &u32 {
        &self.node_name_hash
    }
    fn port_name_hash(&self) -> &u32 {
        &self.port_name_hash
    }
    fn graph_port_offset(&self) -> &u32 {
        &self.graph_port_offset
    }
    fn port_direction(&self) -> &ExpressionPortDirection {
        &self.port_direction
    }
    fn port_type(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.port_type
    }
}

pub static EXPRESSIONPORTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionPortData",
    flags: MemberInfoFlags::new(73),
    module: "Expression",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ExpressionPortData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "PortKey",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint64",
                rust_offset: offset_of!(ExpressionPortData, port_key),
            },
            FieldInfoData {
                name: "NodeNameHash",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ExpressionPortData, node_name_hash),
            },
            FieldInfoData {
                name: "PortNameHash",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ExpressionPortData, port_name_hash),
            },
            FieldInfoData {
                name: "GraphPortOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ExpressionPortData, graph_port_offset),
            },
            FieldInfoData {
                name: "PortDirection",
                flags: MemberInfoFlags::new(0),
                field_type: "ExpressionPortDirection",
                rust_offset: offset_of!(ExpressionPortData, port_direction),
            },
            FieldInfoData {
                name: "PortType",
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
}


pub static EXPRESSIONPORTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionPortData-Array",
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
}


pub static EXPRESSIONPORTDIRECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionPortDirection-Array",
    flags: MemberInfoFlags::new(145),
    module: "Expression",
    data: TypeInfoData::Array("ExpressionPortDirection"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ExpressionNodeGraphData {
    pub _glacier_base: super::core::DataContainer,
    pub ports: Vec<ExpressionPortData>,
    pub inputs: Vec<ExpressionPropertyData>,
    pub outputs: Vec<ExpressionPropertyData>,
    pub states: Vec<ExpressionStateData>,
    pub resource: glacier_reflect::builtin::ResourceRef,
    pub nop_patches: Vec<ExpressionNopPatch>,
    pub constant_reference_patches: Vec<ExpressionConstantReferencePatch>,
    pub constant_empty_array_patches: Vec<ExpressionConstantEmptyArrayPatch>,
}

pub trait ExpressionNodeGraphDataTrait: super::core::DataContainerTrait {
    fn ports(&self) -> &Vec<ExpressionPortData>;
    fn inputs(&self) -> &Vec<ExpressionPropertyData>;
    fn outputs(&self) -> &Vec<ExpressionPropertyData>;
    fn states(&self) -> &Vec<ExpressionStateData>;
    fn resource(&self) -> &glacier_reflect::builtin::ResourceRef;
    fn nop_patches(&self) -> &Vec<ExpressionNopPatch>;
    fn constant_reference_patches(&self) -> &Vec<ExpressionConstantReferencePatch>;
    fn constant_empty_array_patches(&self) -> &Vec<ExpressionConstantEmptyArrayPatch>;
}

impl ExpressionNodeGraphDataTrait for ExpressionNodeGraphData {
    fn ports(&self) -> &Vec<ExpressionPortData> {
        &self.ports
    }
    fn inputs(&self) -> &Vec<ExpressionPropertyData> {
        &self.inputs
    }
    fn outputs(&self) -> &Vec<ExpressionPropertyData> {
        &self.outputs
    }
    fn states(&self) -> &Vec<ExpressionStateData> {
        &self.states
    }
    fn resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.resource
    }
    fn nop_patches(&self) -> &Vec<ExpressionNopPatch> {
        &self.nop_patches
    }
    fn constant_reference_patches(&self) -> &Vec<ExpressionConstantReferencePatch> {
        &self.constant_reference_patches
    }
    fn constant_empty_array_patches(&self) -> &Vec<ExpressionConstantEmptyArrayPatch> {
        &self.constant_empty_array_patches
    }
}

impl super::core::DataContainerTrait for ExpressionNodeGraphData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static EXPRESSIONNODEGRAPHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionNodeGraphData",
    flags: MemberInfoFlags::new(101),
    module: "Expression",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ExpressionNodeGraphData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Ports",
                flags: MemberInfoFlags::new(144),
                field_type: "ExpressionPortData-Array",
                rust_offset: offset_of!(ExpressionNodeGraphData, ports),
            },
            FieldInfoData {
                name: "Inputs",
                flags: MemberInfoFlags::new(144),
                field_type: "ExpressionPropertyData-Array",
                rust_offset: offset_of!(ExpressionNodeGraphData, inputs),
            },
            FieldInfoData {
                name: "Outputs",
                flags: MemberInfoFlags::new(144),
                field_type: "ExpressionPropertyData-Array",
                rust_offset: offset_of!(ExpressionNodeGraphData, outputs),
            },
            FieldInfoData {
                name: "States",
                flags: MemberInfoFlags::new(144),
                field_type: "ExpressionStateData-Array",
                rust_offset: offset_of!(ExpressionNodeGraphData, states),
            },
            FieldInfoData {
                name: "Resource",
                flags: MemberInfoFlags::new(0),
                field_type: "ResourceRef",
                rust_offset: offset_of!(ExpressionNodeGraphData, resource),
            },
            FieldInfoData {
                name: "NopPatches",
                flags: MemberInfoFlags::new(144),
                field_type: "ExpressionNopPatch-Array",
                rust_offset: offset_of!(ExpressionNodeGraphData, nop_patches),
            },
            FieldInfoData {
                name: "ConstantReferencePatches",
                flags: MemberInfoFlags::new(144),
                field_type: "ExpressionConstantReferencePatch-Array",
                rust_offset: offset_of!(ExpressionNodeGraphData, constant_reference_patches),
            },
            FieldInfoData {
                name: "ConstantEmptyArrayPatches",
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
}


pub static EXPRESSIONNODEGRAPHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionNodeGraphData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Expression",
    data: TypeInfoData::Array("ExpressionNodeGraphData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ReferencedType {
    pub blox_data_type_hash: u32,
    pub type_info: glacier_reflect::builtin::TypeRef,
}

pub trait ReferencedTypeTrait: TypeObject {
    fn blox_data_type_hash(&self) -> &u32;
    fn type_info(&self) -> &glacier_reflect::builtin::TypeRef;
}

impl ReferencedTypeTrait for ReferencedType {
    fn blox_data_type_hash(&self) -> &u32 {
        &self.blox_data_type_hash
    }
    fn type_info(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.type_info
    }
}

pub static REFERENCEDTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReferencedType",
    flags: MemberInfoFlags::new(73),
    module: "Expression",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ReferencedType as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "BloxDataTypeHash",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ReferencedType, blox_data_type_hash),
            },
            FieldInfoData {
                name: "TypeInfo",
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
}


pub static REFERENCEDTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReferencedType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Expression",
    data: TypeInfoData::Array("ReferencedType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ExpressionFunctionTypeInfoAsset {
    pub _glacier_base: super::core::FunctionTypeInfoAsset,
    pub graph_data: Option<Arc<Mutex<dyn ExpressionNodeGraphDataTrait>>>,
}

pub trait ExpressionFunctionTypeInfoAssetTrait: super::core::FunctionTypeInfoAssetTrait {
    fn graph_data(&self) -> &Option<Arc<Mutex<dyn ExpressionNodeGraphDataTrait>>>;
}

impl ExpressionFunctionTypeInfoAssetTrait for ExpressionFunctionTypeInfoAsset {
    fn graph_data(&self) -> &Option<Arc<Mutex<dyn ExpressionNodeGraphDataTrait>>> {
        &self.graph_data
    }
}

impl super::core::FunctionTypeInfoAssetTrait for ExpressionFunctionTypeInfoAsset {
    fn parameters(&self) -> &Vec<Option<Arc<Mutex<dyn super::core::TypeInfoParameterDataContainerTrait>>>> {
        self._glacier_base.parameters()
    }
    fn owner(&self) -> &Option<Arc<Mutex<dyn super::core::ClassInfoAssetTrait>>> {
        self._glacier_base.owner()
    }
}

impl super::core::TypeInfoAssetTrait for ExpressionFunctionTypeInfoAsset {
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

impl super::core::AssetTrait for ExpressionFunctionTypeInfoAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for ExpressionFunctionTypeInfoAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static EXPRESSIONFUNCTIONTYPEINFOASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionFunctionTypeInfoAsset",
    flags: MemberInfoFlags::new(101),
    module: "Expression",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::FUNCTIONTYPEINFOASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ExpressionFunctionTypeInfoAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "GraphData",
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
}


pub static EXPRESSIONFUNCTIONTYPEINFOASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionFunctionTypeInfoAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Expression",
    data: TypeInfoData::Array("ExpressionFunctionTypeInfoAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SerializedExpressionNodeGraph {
}

pub trait SerializedExpressionNodeGraphTrait: TypeObject {
}

impl SerializedExpressionNodeGraphTrait for SerializedExpressionNodeGraph {
}

pub static SERIALIZEDEXPRESSIONNODEGRAPH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SerializedExpressionNodeGraph",
    flags: MemberInfoFlags::new(101),
    module: "Expression",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SerializedExpressionNodeGraph as Default>::default())),
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
}


pub static SERIALIZEDEXPRESSIONNODEGRAPH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SerializedExpressionNodeGraph-Array",
    flags: MemberInfoFlags::new(145),
    module: "Expression",
    data: TypeInfoData::Array("SerializedExpressionNodeGraph"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ExpressionRuntimeContext {
}

pub trait ExpressionRuntimeContextTrait: TypeObject {
}

impl ExpressionRuntimeContextTrait for ExpressionRuntimeContext {
}

pub static EXPRESSIONRUNTIMECONTEXT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionRuntimeContext",
    flags: MemberInfoFlags::new(101),
    module: "Expression",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ExpressionRuntimeContext as Default>::default())),
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
}


pub static EXPRESSIONRUNTIMECONTEXT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionRuntimeContext-Array",
    flags: MemberInfoFlags::new(145),
    module: "Expression",
    data: TypeInfoData::Array("ExpressionRuntimeContext"),
    array_type: None,
    alignment: 8,
};


