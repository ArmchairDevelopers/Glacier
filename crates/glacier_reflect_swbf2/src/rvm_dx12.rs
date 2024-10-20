use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_rvm_dx12_types(registry: &mut TypeRegistry) {
    registry.register_type(RVMSERIALIZEDDB_NS_DX12VERTEXBUFFERVIEWINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12VIEWSTATEINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(DX12RVMBACKENDCONFIG_TYPE_INFO);
    registry.register_type(DX12RVMBACKENDCONFIG_ARRAY_TYPE_INFO);
    registry.register_type(D3D12_GPU_DESCRIPTOR_HANDLE_TYPE_INFO);
    registry.register_type(D3D12_VERTEX_BUFFER_VIEW_TYPE_INFO);
    registry.register_type(DX12RVMINPUTELEMENT_TYPE_INFO);
    registry.register_type(DX12RVMSHADER_TYPE_INFO);
    registry.register_type(DX12RVMROOTSIGNATURE_TYPE_INFO);
    registry.register_type(DX12RVMDATABASE_TYPE_INFO);
    registry.register_type(DX12RVMDATABASE_ARRAY_TYPE_INFO);
    registry.register_type(D3D12_VIEWPORT_TYPE_INFO);
    registry.register_type(D3D12_PIPELINE_STATE_FLAGS_TYPE_INFO);
    registry.register_type(D3D12_CACHED_PIPELINE_STATE_TYPE_INFO);
    registry.register_type(DXGI_SAMPLE_DESC_TYPE_INFO);
    registry.register_type(D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_TYPE_INFO);
    registry.register_type(DXGI_FORMAT_TYPE_INFO);
    registry.register_type(D3D12_PRIMITIVE_TOPOLOGY_TYPE_INFO);
    registry.register_type(D3D12_PRIMITIVE_TOPOLOGY_TYPE_TYPE_INFO);
    registry.register_type(DX12RVMSODECLARATIONENTRY_TYPE_INFO);
    registry.register_type(DX12RVMDEPTHSTENCILDESC_TYPE_INFO);
    registry.register_type(DX12RVMRASTERIZERDESC_TYPE_INFO);
    registry.register_type(DX12RVMBLENDDESC_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12ROOTDESCRIPTORTABLEASSEMBLYDEBUGINFO_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12BLENDSTATEDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12ROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12ROOTWRITEOP_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12LEGACYDRAWSTATEBUILDERINSTRUCTIONBATCHDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12LEGACYDRAWSTATEBUILDERINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12SHADERDISPATCHDRAWINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12ROOTSIGNATUREDEBUGINFO_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12TABLEDEBUGINFO_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12TABLEDEBUGENTRY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12INPUTELEMENT_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12SHADER_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12BINARYBLOB_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12SHADERSTATE_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12VertexBufferViewInstructionData {
}

pub const RVMSERIALIZEDDB_NS_DX12VERTEXBUFFERVIEWINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12VertexBufferViewInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12VERTEXBUFFERVIEWINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx12VertexBufferViewInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12VERTEXBUFFERVIEWINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12ViewStateInstructionData {
}

pub const RVMSERIALIZEDDB_NS_DX12VIEWSTATEINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12ViewStateInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12VIEWSTATEINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12ViewStateInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12VIEWSTATEINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12RvmBackendConfig {
}

pub const DX12RVMBACKENDCONFIG_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmBackendConfig",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMBACKENDCONFIG_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12RVMBACKENDCONFIG_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12RvmBackendConfig {
    fn type_info() -> &'static TypeInfo {
        DX12RVMBACKENDCONFIG_TYPE_INFO
    }
}


pub const DX12RVMBACKENDCONFIG_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmBackendConfig-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12",
    data: TypeInfoData::Array("Dx12RvmBackendConfig-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct D3D12_GPU_DESCRIPTOR_HANDLE {
}

pub const D3D12_GPU_DESCRIPTOR_HANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_GPU_DESCRIPTOR_HANDLE",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(D3D12_GPU_DESCRIPTOR_HANDLE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for D3D12_GPU_DESCRIPTOR_HANDLE {
    fn type_info() -> &'static TypeInfo {
        D3D12_GPU_DESCRIPTOR_HANDLE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct D3D12_VERTEX_BUFFER_VIEW {
}

pub const D3D12_VERTEX_BUFFER_VIEW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_VERTEX_BUFFER_VIEW",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(D3D12_VERTEX_BUFFER_VIEW_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for D3D12_VERTEX_BUFFER_VIEW {
    fn type_info() -> &'static TypeInfo {
        D3D12_VERTEX_BUFFER_VIEW_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12RvmInputElement {
}

pub const DX12RVMINPUTELEMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmInputElement",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(DX12RVMINPUTELEMENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12RvmInputElement {
    fn type_info() -> &'static TypeInfo {
        DX12RVMINPUTELEMENT_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12RvmShader {
}

pub const DX12RVMSHADER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmShader",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(DX12RVMSHADER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12RvmShader {
    fn type_info() -> &'static TypeInfo {
        DX12RVMSHADER_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12RvmRootSignature {
}

pub const DX12RVMROOTSIGNATURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmRootSignature",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(DX12RVMROOTSIGNATURE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12RvmRootSignature {
    fn type_info() -> &'static TypeInfo {
        DX12RVMROOTSIGNATURE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12RvmDatabase {
}

pub const DX12RVMDATABASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmDatabase",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BASERVMDATABASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12RVMDATABASE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12RvmDatabase {
    fn type_info() -> &'static TypeInfo {
        DX12RVMDATABASE_TYPE_INFO
    }
}


pub const DX12RVMDATABASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmDatabase-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12",
    data: TypeInfoData::Array("Dx12RvmDatabase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct D3D12_VIEWPORT {
}

pub const D3D12_VIEWPORT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_VIEWPORT",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(D3D12_VIEWPORT_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for D3D12_VIEWPORT {
    fn type_info() -> &'static TypeInfo {
        D3D12_VIEWPORT_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct D3D12_PIPELINE_STATE_FLAGS {
}

pub const D3D12_PIPELINE_STATE_FLAGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_PIPELINE_STATE_FLAGS",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(D3D12_PIPELINE_STATE_FLAGS_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for D3D12_PIPELINE_STATE_FLAGS {
    fn type_info() -> &'static TypeInfo {
        D3D12_PIPELINE_STATE_FLAGS_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct D3D12_CACHED_PIPELINE_STATE {
}

pub const D3D12_CACHED_PIPELINE_STATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_CACHED_PIPELINE_STATE",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(D3D12_CACHED_PIPELINE_STATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for D3D12_CACHED_PIPELINE_STATE {
    fn type_info() -> &'static TypeInfo {
        D3D12_CACHED_PIPELINE_STATE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DXGI_SAMPLE_DESC {
}

pub const DXGI_SAMPLE_DESC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DXGI_SAMPLE_DESC",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(DXGI_SAMPLE_DESC_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for DXGI_SAMPLE_DESC {
    fn type_info() -> &'static TypeInfo {
        DXGI_SAMPLE_DESC_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct D3D12_INDEX_BUFFER_STRIP_CUT_VALUE {
}

pub const D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_INDEX_BUFFER_STRIP_CUT_VALUE",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for D3D12_INDEX_BUFFER_STRIP_CUT_VALUE {
    fn type_info() -> &'static TypeInfo {
        D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DXGI_FORMAT {
}

pub const DXGI_FORMAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DXGI_FORMAT",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(DXGI_FORMAT_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for DXGI_FORMAT {
    fn type_info() -> &'static TypeInfo {
        DXGI_FORMAT_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct D3D12_PRIMITIVE_TOPOLOGY {
}

pub const D3D12_PRIMITIVE_TOPOLOGY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_PRIMITIVE_TOPOLOGY",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(D3D12_PRIMITIVE_TOPOLOGY_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for D3D12_PRIMITIVE_TOPOLOGY {
    fn type_info() -> &'static TypeInfo {
        D3D12_PRIMITIVE_TOPOLOGY_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct D3D12_PRIMITIVE_TOPOLOGY_TYPE {
}

pub const D3D12_PRIMITIVE_TOPOLOGY_TYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_PRIMITIVE_TOPOLOGY_TYPE",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(D3D12_PRIMITIVE_TOPOLOGY_TYPE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for D3D12_PRIMITIVE_TOPOLOGY_TYPE {
    fn type_info() -> &'static TypeInfo {
        D3D12_PRIMITIVE_TOPOLOGY_TYPE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12RvmSoDeclarationEntry {
}

pub const DX12RVMSODECLARATIONENTRY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmSoDeclarationEntry",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(DX12RVMSODECLARATIONENTRY_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for Dx12RvmSoDeclarationEntry {
    fn type_info() -> &'static TypeInfo {
        DX12RVMSODECLARATIONENTRY_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12RvmDepthStencilDesc {
}

pub const DX12RVMDEPTHSTENCILDESC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmDepthStencilDesc",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(DX12RVMDEPTHSTENCILDESC_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for Dx12RvmDepthStencilDesc {
    fn type_info() -> &'static TypeInfo {
        DX12RVMDEPTHSTENCILDESC_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12RvmRasterizerDesc {
}

pub const DX12RVMRASTERIZERDESC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmRasterizerDesc",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(DX12RVMRASTERIZERDESC_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for Dx12RvmRasterizerDesc {
    fn type_info() -> &'static TypeInfo {
        DX12RVMRASTERIZERDESC_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12RvmBlendDesc {
}

pub const DX12RVMBLENDDESC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmBlendDesc",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(DX12RVMBLENDDESC_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for Dx12RvmBlendDesc {
    fn type_info() -> &'static TypeInfo {
        DX12RVMBLENDDESC_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12RootDescriptorTableAssemblyDebugInfo {
}

pub const RVMSERIALIZEDDB_NS_DX12ROOTDESCRIPTORTABLEASSEMBLYDEBUGINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RootDescriptorTableAssemblyDebugInfo",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12ROOTDESCRIPTORTABLEASSEMBLYDEBUGINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx12RootDescriptorTableAssemblyDebugInfo {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12ROOTDESCRIPTORTABLEASSEMBLYDEBUGINFO_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12BlendStateData {
}

pub const RVMSERIALIZEDDB_NS_DX12BLENDSTATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12BlendStateData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12BLENDSTATEDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RvmSerializedDb_ns_Dx12BlendStateData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12BLENDSTATEDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12RootDescriptorTableAssemblyInstructionData {
}

pub const RVMSERIALIZEDDB_NS_DX12ROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RootDescriptorTableAssemblyInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12ROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx12RootDescriptorTableAssemblyInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12ROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12RootWriteOp {
}

pub const RVMSERIALIZEDDB_NS_DX12ROOTWRITEOP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RootWriteOp",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12ROOTWRITEOP_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12RootWriteOp {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12ROOTWRITEOP_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12LegacyDrawStateBuilderInstructionBatchData {
}

pub const RVMSERIALIZEDDB_NS_DX12LEGACYDRAWSTATEBUILDERINSTRUCTIONBATCHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12LegacyDrawStateBuilderInstructionBatchData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12LEGACYDRAWSTATEBUILDERINSTRUCTIONBATCHDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx12LegacyDrawStateBuilderInstructionBatchData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12LEGACYDRAWSTATEBUILDERINSTRUCTIONBATCHDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12LegacyDrawStateBuilderInstructionData {
}

pub const RVMSERIALIZEDDB_NS_DX12LEGACYDRAWSTATEBUILDERINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12LegacyDrawStateBuilderInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12LEGACYDRAWSTATEBUILDERINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx12LegacyDrawStateBuilderInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12LEGACYDRAWSTATEBUILDERINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12ShaderDispatchDrawInstructionData {
}

pub const RVMSERIALIZEDDB_NS_DX12SHADERDISPATCHDRAWINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12ShaderDispatchDrawInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12SHADERDISPATCHDRAWINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12ShaderDispatchDrawInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12SHADERDISPATCHDRAWINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12RootSignatureDebugInfo {
}

pub const RVMSERIALIZEDDB_NS_DX12ROOTSIGNATUREDEBUGINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RootSignatureDebugInfo",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12ROOTSIGNATUREDEBUGINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx12RootSignatureDebugInfo {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12ROOTSIGNATUREDEBUGINFO_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12TableDebugInfo {
}

pub const RVMSERIALIZEDDB_NS_DX12TABLEDEBUGINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12TableDebugInfo",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12TABLEDEBUGINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx12TableDebugInfo {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12TABLEDEBUGINFO_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12TableDebugEntry {
}

pub const RVMSERIALIZEDDB_NS_DX12TABLEDEBUGENTRY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12TableDebugEntry",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12TABLEDEBUGENTRY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx12TableDebugEntry {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12TABLEDEBUGENTRY_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12InputElement {
}

pub const RVMSERIALIZEDDB_NS_DX12INPUTELEMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12InputElement",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12INPUTELEMENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx12InputElement {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12INPUTELEMENT_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12Shader {
}

pub const RVMSERIALIZEDDB_NS_DX12SHADER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12Shader",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12SHADER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx12Shader {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12SHADER_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12BinaryBlob {
}

pub const RVMSERIALIZEDDB_NS_DX12BINARYBLOB_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12BinaryBlob",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12BINARYBLOB_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RvmSerializedDb_ns_Dx12BinaryBlob {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12BINARYBLOB_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12ShaderState {
}

pub const RVMSERIALIZEDDB_NS_DX12SHADERSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12ShaderState",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12SHADERSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RvmSerializedDb_ns_Dx12ShaderState {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12SHADERSTATE_TYPE_INFO
    }
}

