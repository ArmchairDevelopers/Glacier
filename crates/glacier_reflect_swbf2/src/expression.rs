use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ExpressionConstantEmptyArrayPatch {
    pub register_offset: u32,
    pub data_offset: u32,
}

pub const EXPRESSIONCONSTANTEMPTYARRAYPATCH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionConstantEmptyArrayPatch",
    flags: MemberInfoFlags::new(36937),
    module: "Expression",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "RegisterOffset",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ExpressionConstantEmptyArrayPatch, register_offset),
            },
            FieldInfoData {
                name: "DataOffset",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ExpressionConstantEmptyArrayPatch, data_offset),
            },
        ],
    }),
    array_type: Some(EXPRESSIONCONSTANTEMPTYARRAYPATCH_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for ExpressionConstantEmptyArrayPatch {
    fn type_info() -> &'static TypeInfo {
        EXPRESSIONCONSTANTEMPTYARRAYPATCH_TYPE_INFO
    }
}


pub const EXPRESSIONCONSTANTEMPTYARRAYPATCH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionConstantEmptyArrayPatch-Array",
    flags: MemberInfoFlags::new(145),
    module: "Expression",
    data: TypeInfoData::Array("ExpressionConstantEmptyArrayPatch-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ExpressionConstantReferencePatch {
    pub value: super::core::DataContainer,
    pub register_offset: u32,
    pub data_offset: u32,
}

pub const EXPRESSIONCONSTANTREFERENCEPATCH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionConstantReferencePatch",
    flags: MemberInfoFlags::new(73),
    module: "Expression",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: DATACONTAINER_TYPE_INFO,
                rust_offset: offset_of!(ExpressionConstantReferencePatch, value),
            },
            FieldInfoData {
                name: "RegisterOffset",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ExpressionConstantReferencePatch, register_offset),
            },
            FieldInfoData {
                name: "DataOffset",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ExpressionConstantReferencePatch, data_offset),
            },
        ],
    }),
    array_type: Some(EXPRESSIONCONSTANTREFERENCEPATCH_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ExpressionConstantReferencePatch {
    fn type_info() -> &'static TypeInfo {
        EXPRESSIONCONSTANTREFERENCEPATCH_TYPE_INFO
    }
}


pub const EXPRESSIONCONSTANTREFERENCEPATCH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionConstantReferencePatch-Array",
    flags: MemberInfoFlags::new(145),
    module: "Expression",
    data: TypeInfoData::Array("ExpressionConstantReferencePatch-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ExpressionNopPatch {
    pub value: super::core::TypeRef,
    pub register_offset: u32,
}

pub const EXPRESSIONNOPPATCH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionNopPatch",
    flags: MemberInfoFlags::new(73),
    module: "Expression",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: TYPEREF_TYPE_INFO,
                rust_offset: offset_of!(ExpressionNopPatch, value),
            },
            FieldInfoData {
                name: "RegisterOffset",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ExpressionNopPatch, register_offset),
            },
        ],
    }),
    array_type: Some(EXPRESSIONNOPPATCH_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ExpressionNopPatch {
    fn type_info() -> &'static TypeInfo {
        EXPRESSIONNOPPATCH_TYPE_INFO
    }
}


pub const EXPRESSIONNOPPATCH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionNopPatch-Array",
    flags: MemberInfoFlags::new(145),
    module: "Expression",
    data: TypeInfoData::Array("ExpressionNopPatch-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ExpressionStateData {
    pub graph_port_offset: u32,
    pub port_type: super::core::TypeRef,
}

pub const EXPRESSIONSTATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionStateData",
    flags: MemberInfoFlags::new(73),
    module: "Expression",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "GraphPortOffset",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ExpressionStateData, graph_port_offset),
            },
            FieldInfoData {
                name: "PortType",
                flags: MemberInfoFlags::new(0),
                field_type: TYPEREF_TYPE_INFO,
                rust_offset: offset_of!(ExpressionStateData, port_type),
            },
        ],
    }),
    array_type: Some(EXPRESSIONSTATEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ExpressionStateData {
    fn type_info() -> &'static TypeInfo {
        EXPRESSIONSTATEDATA_TYPE_INFO
    }
}


pub const EXPRESSIONSTATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionStateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Expression",
    data: TypeInfoData::Array("ExpressionStateData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ExpressionPropertyData {
    pub property_name_hash: u32,
    pub graph_port_offset: u32,
    pub port_type: super::core::TypeRef,
    pub is_reference: bool,
}

pub const EXPRESSIONPROPERTYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionPropertyData",
    flags: MemberInfoFlags::new(73),
    module: "Expression",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "PropertyNameHash",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ExpressionPropertyData, property_name_hash),
            },
            FieldInfoData {
                name: "GraphPortOffset",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ExpressionPropertyData, graph_port_offset),
            },
            FieldInfoData {
                name: "PortType",
                flags: MemberInfoFlags::new(0),
                field_type: TYPEREF_TYPE_INFO,
                rust_offset: offset_of!(ExpressionPropertyData, port_type),
            },
            FieldInfoData {
                name: "IsReference",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ExpressionPropertyData, is_reference),
            },
        ],
    }),
    array_type: Some(EXPRESSIONPROPERTYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ExpressionPropertyData {
    fn type_info() -> &'static TypeInfo {
        EXPRESSIONPROPERTYDATA_TYPE_INFO
    }
}


pub const EXPRESSIONPROPERTYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionPropertyData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Expression",
    data: TypeInfoData::Array("ExpressionPropertyData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ExpressionPortData {
    pub port_key: u64,
    pub node_name_hash: u32,
    pub port_name_hash: u32,
    pub graph_port_offset: u32,
    pub port_direction: ExpressionPortDirection,
    pub port_type: super::core::TypeRef,
}

pub const EXPRESSIONPORTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionPortData",
    flags: MemberInfoFlags::new(73),
    module: "Expression",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "PortKey",
                flags: MemberInfoFlags::new(0),
                field_type: UINT64_TYPE_INFO,
                rust_offset: offset_of!(ExpressionPortData, port_key),
            },
            FieldInfoData {
                name: "NodeNameHash",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ExpressionPortData, node_name_hash),
            },
            FieldInfoData {
                name: "PortNameHash",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ExpressionPortData, port_name_hash),
            },
            FieldInfoData {
                name: "GraphPortOffset",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ExpressionPortData, graph_port_offset),
            },
            FieldInfoData {
                name: "PortDirection",
                flags: MemberInfoFlags::new(0),
                field_type: EXPRESSIONPORTDIRECTION_TYPE_INFO,
                rust_offset: offset_of!(ExpressionPortData, port_direction),
            },
            FieldInfoData {
                name: "PortType",
                flags: MemberInfoFlags::new(0),
                field_type: TYPEREF_TYPE_INFO,
                rust_offset: offset_of!(ExpressionPortData, port_type),
            },
        ],
    }),
    array_type: Some(EXPRESSIONPORTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ExpressionPortData {
    fn type_info() -> &'static TypeInfo {
        EXPRESSIONPORTDATA_TYPE_INFO
    }
}


pub const EXPRESSIONPORTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionPortData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Expression",
    data: TypeInfoData::Array("ExpressionPortData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ExpressionPortDirection {
    #[default]
    ExpressionPortDirection_In = 0,
    ExpressionPortDirection_Out = 1,
}

pub const EXPRESSIONPORTDIRECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionPortDirection",
    flags: MemberInfoFlags::new(49429),
    module: "Expression",
    data: TypeInfoData::Enum,
    array_type: Some(EXPRESSIONPORTDIRECTION_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ExpressionPortDirection {
    fn type_info() -> &'static TypeInfo {
        EXPRESSIONPORTDIRECTION_TYPE_INFO
    }
}


pub const EXPRESSIONPORTDIRECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionPortDirection-Array",
    flags: MemberInfoFlags::new(145),
    module: "Expression",
    data: TypeInfoData::Array("ExpressionPortDirection-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ExpressionNodeGraphData {
    pub ports: Vec<ExpressionPortData>,
    pub inputs: Vec<ExpressionPropertyData>,
    pub outputs: Vec<ExpressionPropertyData>,
    pub states: Vec<ExpressionStateData>,
    pub resource: super::core::ResourceRef,
    pub nop_patches: Vec<ExpressionNopPatch>,
    pub constant_reference_patches: Vec<ExpressionConstantReferencePatch>,
    pub constant_empty_array_patches: Vec<ExpressionConstantEmptyArrayPatch>,
}

pub const EXPRESSIONNODEGRAPHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionNodeGraphData",
    flags: MemberInfoFlags::new(101),
    module: "Expression",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Ports",
                flags: MemberInfoFlags::new(144),
                field_type: EXPRESSIONPORTDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ExpressionNodeGraphData, ports),
            },
            FieldInfoData {
                name: "Inputs",
                flags: MemberInfoFlags::new(144),
                field_type: EXPRESSIONPROPERTYDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ExpressionNodeGraphData, inputs),
            },
            FieldInfoData {
                name: "Outputs",
                flags: MemberInfoFlags::new(144),
                field_type: EXPRESSIONPROPERTYDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ExpressionNodeGraphData, outputs),
            },
            FieldInfoData {
                name: "States",
                flags: MemberInfoFlags::new(144),
                field_type: EXPRESSIONSTATEDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ExpressionNodeGraphData, states),
            },
            FieldInfoData {
                name: "Resource",
                flags: MemberInfoFlags::new(0),
                field_type: RESOURCEREF_TYPE_INFO,
                rust_offset: offset_of!(ExpressionNodeGraphData, resource),
            },
            FieldInfoData {
                name: "NopPatches",
                flags: MemberInfoFlags::new(144),
                field_type: EXPRESSIONNOPPATCH_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ExpressionNodeGraphData, nop_patches),
            },
            FieldInfoData {
                name: "ConstantReferencePatches",
                flags: MemberInfoFlags::new(144),
                field_type: EXPRESSIONCONSTANTREFERENCEPATCH_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ExpressionNodeGraphData, constant_reference_patches),
            },
            FieldInfoData {
                name: "ConstantEmptyArrayPatches",
                flags: MemberInfoFlags::new(144),
                field_type: EXPRESSIONCONSTANTEMPTYARRAYPATCH_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ExpressionNodeGraphData, constant_empty_array_patches),
            },
        ],
    }),
    array_type: Some(EXPRESSIONNODEGRAPHDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ExpressionNodeGraphData {
    fn type_info() -> &'static TypeInfo {
        EXPRESSIONNODEGRAPHDATA_TYPE_INFO
    }
}


pub const EXPRESSIONNODEGRAPHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionNodeGraphData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Expression",
    data: TypeInfoData::Array("ExpressionNodeGraphData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ReferencedType {
    pub blox_data_type_hash: u32,
    pub type_info: super::core::TypeRef,
}

pub const REFERENCEDTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReferencedType",
    flags: MemberInfoFlags::new(73),
    module: "Expression",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "BloxDataTypeHash",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ReferencedType, blox_data_type_hash),
            },
            FieldInfoData {
                name: "TypeInfo",
                flags: MemberInfoFlags::new(0),
                field_type: TYPEREF_TYPE_INFO,
                rust_offset: offset_of!(ReferencedType, type_info),
            },
        ],
    }),
    array_type: Some(REFERENCEDTYPE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ReferencedType {
    fn type_info() -> &'static TypeInfo {
        REFERENCEDTYPE_TYPE_INFO
    }
}


pub const REFERENCEDTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReferencedType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Expression",
    data: TypeInfoData::Array("ReferencedType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ExpressionFunctionTypeInfoAsset {
    pub graph_data: ExpressionNodeGraphData,
}

pub const EXPRESSIONFUNCTIONTYPEINFOASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionFunctionTypeInfoAsset",
    flags: MemberInfoFlags::new(101),
    module: "Expression",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(FUNCTIONTYPEINFOASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "GraphData",
                flags: MemberInfoFlags::new(0),
                field_type: EXPRESSIONNODEGRAPHDATA_TYPE_INFO,
                rust_offset: offset_of!(ExpressionFunctionTypeInfoAsset, graph_data),
            },
        ],
    }),
    array_type: Some(EXPRESSIONFUNCTIONTYPEINFOASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ExpressionFunctionTypeInfoAsset {
    fn type_info() -> &'static TypeInfo {
        EXPRESSIONFUNCTIONTYPEINFOASSET_TYPE_INFO
    }
}


pub const EXPRESSIONFUNCTIONTYPEINFOASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionFunctionTypeInfoAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Expression",
    data: TypeInfoData::Array("ExpressionFunctionTypeInfoAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SerializedExpressionNodeGraph {
}

pub const SERIALIZEDEXPRESSIONNODEGRAPH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SerializedExpressionNodeGraph",
    flags: MemberInfoFlags::new(101),
    module: "Expression",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(SERIALIZEDEXPRESSIONNODEGRAPH_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SerializedExpressionNodeGraph {
    fn type_info() -> &'static TypeInfo {
        SERIALIZEDEXPRESSIONNODEGRAPH_TYPE_INFO
    }
}


pub const SERIALIZEDEXPRESSIONNODEGRAPH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SerializedExpressionNodeGraph-Array",
    flags: MemberInfoFlags::new(145),
    module: "Expression",
    data: TypeInfoData::Array("SerializedExpressionNodeGraph-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ExpressionRuntimeContext {
}

pub const EXPRESSIONRUNTIMECONTEXT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionRuntimeContext",
    flags: MemberInfoFlags::new(101),
    module: "Expression",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(EXPRESSIONRUNTIMECONTEXT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ExpressionRuntimeContext {
    fn type_info() -> &'static TypeInfo {
        EXPRESSIONRUNTIMECONTEXT_TYPE_INFO
    }
}


pub const EXPRESSIONRUNTIMECONTEXT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExpressionRuntimeContext-Array",
    flags: MemberInfoFlags::new(145),
    module: "Expression",
    data: TypeInfoData::Array("ExpressionRuntimeContext-Array"),
    array_type: None,
    alignment: 8,
};


