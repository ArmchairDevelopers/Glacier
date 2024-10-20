use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12VertexBufferViewInstructionData {
}

pub trait RvmSerializedDb_ns_Dx12VertexBufferViewInstructionDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12VertexBufferViewInstructionDataTrait for RvmSerializedDb_ns_Dx12VertexBufferViewInstructionData {
}

pub static RVMSERIALIZEDDB_NS_DX12VERTEXBUFFERVIEWINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12VertexBufferViewInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12VertexBufferViewInstructionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12VERTEXBUFFERVIEWINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx12VertexBufferViewInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12VERTEXBUFFERVIEWINSTRUCTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12ViewStateInstructionData {
}

pub trait RvmSerializedDb_ns_Dx12ViewStateInstructionDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12ViewStateInstructionDataTrait for RvmSerializedDb_ns_Dx12ViewStateInstructionData {
}

pub static RVMSERIALIZEDDB_NS_DX12VIEWSTATEINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12ViewStateInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12ViewStateInstructionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12VIEWSTATEINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12ViewStateInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12VIEWSTATEINSTRUCTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct Dx12RvmBackendConfig {
    pub _glacier_base: super::rvm_common::RvmBackendConfig,
}

pub trait Dx12RvmBackendConfigTrait: super::rvm_common::RvmBackendConfigTrait {
}

impl Dx12RvmBackendConfigTrait for Dx12RvmBackendConfig {
}

impl super::rvm_common::RvmBackendConfigTrait for Dx12RvmBackendConfig {
}

impl super::core::AssetTrait for Dx12RvmBackendConfig {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for Dx12RvmBackendConfig {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static DX12RVMBACKENDCONFIG_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmBackendConfig",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMBACKENDCONFIG_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RvmBackendConfig as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12RVMBACKENDCONFIG_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12RvmBackendConfig {
    fn type_info(&self) -> &'static TypeInfo {
        DX12RVMBACKENDCONFIG_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DX12RVMBACKENDCONFIG_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmBackendConfig-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12",
    data: TypeInfoData::Array("Dx12RvmBackendConfig"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct D3D12_GPU_DESCRIPTOR_HANDLE {
}

pub trait D3D12_GPU_DESCRIPTOR_HANDLETrait: TypeObject {
}

impl D3D12_GPU_DESCRIPTOR_HANDLETrait for D3D12_GPU_DESCRIPTOR_HANDLE {
}

pub static D3D12_GPU_DESCRIPTOR_HANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_GPU_DESCRIPTOR_HANDLE",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<D3D12_GPU_DESCRIPTOR_HANDLE as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::D3D12_GPU_DESCRIPTOR_HANDLE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for D3D12_GPU_DESCRIPTOR_HANDLE {
    fn type_info(&self) -> &'static TypeInfo {
        D3D12_GPU_DESCRIPTOR_HANDLE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct D3D12_VERTEX_BUFFER_VIEW {
}

pub trait D3D12_VERTEX_BUFFER_VIEWTrait: TypeObject {
}

impl D3D12_VERTEX_BUFFER_VIEWTrait for D3D12_VERTEX_BUFFER_VIEW {
}

pub static D3D12_VERTEX_BUFFER_VIEW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_VERTEX_BUFFER_VIEW",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<D3D12_VERTEX_BUFFER_VIEW as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::D3D12_VERTEX_BUFFER_VIEW_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for D3D12_VERTEX_BUFFER_VIEW {
    fn type_info(&self) -> &'static TypeInfo {
        D3D12_VERTEX_BUFFER_VIEW_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct Dx12RvmInputElement {
}

pub trait Dx12RvmInputElementTrait: TypeObject {
}

impl Dx12RvmInputElementTrait for Dx12RvmInputElement {
}

pub static DX12RVMINPUTELEMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmInputElement",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RvmInputElement as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::DX12RVMINPUTELEMENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12RvmInputElement {
    fn type_info(&self) -> &'static TypeInfo {
        DX12RVMINPUTELEMENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct Dx12RvmShader {
}

pub trait Dx12RvmShaderTrait: TypeObject {
}

impl Dx12RvmShaderTrait for Dx12RvmShader {
}

pub static DX12RVMSHADER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmShader",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RvmShader as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::DX12RVMSHADER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12RvmShader {
    fn type_info(&self) -> &'static TypeInfo {
        DX12RVMSHADER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct Dx12RvmRootSignature {
}

pub trait Dx12RvmRootSignatureTrait: TypeObject {
}

impl Dx12RvmRootSignatureTrait for Dx12RvmRootSignature {
}

pub static DX12RVMROOTSIGNATURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmRootSignature",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RvmRootSignature as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::DX12RVMROOTSIGNATURE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12RvmRootSignature {
    fn type_info(&self) -> &'static TypeInfo {
        DX12RVMROOTSIGNATURE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct Dx12RvmDatabase {
    pub _glacier_base: super::rvm_common::BaseRvmDatabase,
}

pub trait Dx12RvmDatabaseTrait: super::rvm_common::BaseRvmDatabaseTrait {
}

impl Dx12RvmDatabaseTrait for Dx12RvmDatabase {
}

impl super::rvm_common::BaseRvmDatabaseTrait for Dx12RvmDatabase {
}

impl super::render::RvmLegacyDatabaseTrait for Dx12RvmDatabase {
}

impl super::render::RvmDatabaseTrait for Dx12RvmDatabase {
}

impl super::core::IResourceObjectTrait for Dx12RvmDatabase {
}

pub static DX12RVMDATABASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmDatabase",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::BASERVMDATABASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RvmDatabase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12RVMDATABASE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12RvmDatabase {
    fn type_info(&self) -> &'static TypeInfo {
        DX12RVMDATABASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DX12RVMDATABASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmDatabase-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12",
    data: TypeInfoData::Array("Dx12RvmDatabase"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct D3D12_VIEWPORT {
}

pub trait D3D12_VIEWPORTTrait: TypeObject {
}

impl D3D12_VIEWPORTTrait for D3D12_VIEWPORT {
}

pub static D3D12_VIEWPORT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_VIEWPORT",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<D3D12_VIEWPORT as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::D3D12_VIEWPORT_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for D3D12_VIEWPORT {
    fn type_info(&self) -> &'static TypeInfo {
        D3D12_VIEWPORT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct D3D12_PIPELINE_STATE_FLAGS {
}

pub trait D3D12_PIPELINE_STATE_FLAGSTrait: TypeObject {
}

impl D3D12_PIPELINE_STATE_FLAGSTrait for D3D12_PIPELINE_STATE_FLAGS {
}

pub static D3D12_PIPELINE_STATE_FLAGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_PIPELINE_STATE_FLAGS",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<D3D12_PIPELINE_STATE_FLAGS as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::D3D12_PIPELINE_STATE_FLAGS_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for D3D12_PIPELINE_STATE_FLAGS {
    fn type_info(&self) -> &'static TypeInfo {
        D3D12_PIPELINE_STATE_FLAGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct D3D12_CACHED_PIPELINE_STATE {
}

pub trait D3D12_CACHED_PIPELINE_STATETrait: TypeObject {
}

impl D3D12_CACHED_PIPELINE_STATETrait for D3D12_CACHED_PIPELINE_STATE {
}

pub static D3D12_CACHED_PIPELINE_STATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_CACHED_PIPELINE_STATE",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<D3D12_CACHED_PIPELINE_STATE as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::D3D12_CACHED_PIPELINE_STATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for D3D12_CACHED_PIPELINE_STATE {
    fn type_info(&self) -> &'static TypeInfo {
        D3D12_CACHED_PIPELINE_STATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct DXGI_SAMPLE_DESC {
}

pub trait DXGI_SAMPLE_DESCTrait: TypeObject {
}

impl DXGI_SAMPLE_DESCTrait for DXGI_SAMPLE_DESC {
}

pub static DXGI_SAMPLE_DESC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DXGI_SAMPLE_DESC",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DXGI_SAMPLE_DESC as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::DXGI_SAMPLE_DESC_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for DXGI_SAMPLE_DESC {
    fn type_info(&self) -> &'static TypeInfo {
        DXGI_SAMPLE_DESC_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct D3D12_INDEX_BUFFER_STRIP_CUT_VALUE {
}

pub trait D3D12_INDEX_BUFFER_STRIP_CUT_VALUETrait: TypeObject {
}

impl D3D12_INDEX_BUFFER_STRIP_CUT_VALUETrait for D3D12_INDEX_BUFFER_STRIP_CUT_VALUE {
}

pub static D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_INDEX_BUFFER_STRIP_CUT_VALUE",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<D3D12_INDEX_BUFFER_STRIP_CUT_VALUE as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for D3D12_INDEX_BUFFER_STRIP_CUT_VALUE {
    fn type_info(&self) -> &'static TypeInfo {
        D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct DXGI_FORMAT {
}

pub trait DXGI_FORMATTrait: TypeObject {
}

impl DXGI_FORMATTrait for DXGI_FORMAT {
}

pub static DXGI_FORMAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DXGI_FORMAT",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DXGI_FORMAT as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::DXGI_FORMAT_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for DXGI_FORMAT {
    fn type_info(&self) -> &'static TypeInfo {
        DXGI_FORMAT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct D3D12_PRIMITIVE_TOPOLOGY {
}

pub trait D3D12_PRIMITIVE_TOPOLOGYTrait: TypeObject {
}

impl D3D12_PRIMITIVE_TOPOLOGYTrait for D3D12_PRIMITIVE_TOPOLOGY {
}

pub static D3D12_PRIMITIVE_TOPOLOGY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_PRIMITIVE_TOPOLOGY",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<D3D12_PRIMITIVE_TOPOLOGY as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::D3D12_PRIMITIVE_TOPOLOGY_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for D3D12_PRIMITIVE_TOPOLOGY {
    fn type_info(&self) -> &'static TypeInfo {
        D3D12_PRIMITIVE_TOPOLOGY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct D3D12_PRIMITIVE_TOPOLOGY_TYPE {
}

pub trait D3D12_PRIMITIVE_TOPOLOGY_TYPETrait: TypeObject {
}

impl D3D12_PRIMITIVE_TOPOLOGY_TYPETrait for D3D12_PRIMITIVE_TOPOLOGY_TYPE {
}

pub static D3D12_PRIMITIVE_TOPOLOGY_TYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_PRIMITIVE_TOPOLOGY_TYPE",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<D3D12_PRIMITIVE_TOPOLOGY_TYPE as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::D3D12_PRIMITIVE_TOPOLOGY_TYPE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for D3D12_PRIMITIVE_TOPOLOGY_TYPE {
    fn type_info(&self) -> &'static TypeInfo {
        D3D12_PRIMITIVE_TOPOLOGY_TYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct Dx12RvmSoDeclarationEntry {
}

pub trait Dx12RvmSoDeclarationEntryTrait: TypeObject {
}

impl Dx12RvmSoDeclarationEntryTrait for Dx12RvmSoDeclarationEntry {
}

pub static DX12RVMSODECLARATIONENTRY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmSoDeclarationEntry",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RvmSoDeclarationEntry as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::DX12RVMSODECLARATIONENTRY_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for Dx12RvmSoDeclarationEntry {
    fn type_info(&self) -> &'static TypeInfo {
        DX12RVMSODECLARATIONENTRY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct Dx12RvmDepthStencilDesc {
}

pub trait Dx12RvmDepthStencilDescTrait: TypeObject {
}

impl Dx12RvmDepthStencilDescTrait for Dx12RvmDepthStencilDesc {
}

pub static DX12RVMDEPTHSTENCILDESC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmDepthStencilDesc",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RvmDepthStencilDesc as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::DX12RVMDEPTHSTENCILDESC_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for Dx12RvmDepthStencilDesc {
    fn type_info(&self) -> &'static TypeInfo {
        DX12RVMDEPTHSTENCILDESC_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct Dx12RvmRasterizerDesc {
}

pub trait Dx12RvmRasterizerDescTrait: TypeObject {
}

impl Dx12RvmRasterizerDescTrait for Dx12RvmRasterizerDesc {
}

pub static DX12RVMRASTERIZERDESC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmRasterizerDesc",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RvmRasterizerDesc as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::DX12RVMRASTERIZERDESC_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for Dx12RvmRasterizerDesc {
    fn type_info(&self) -> &'static TypeInfo {
        DX12RVMRASTERIZERDESC_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct Dx12RvmBlendDesc {
}

pub trait Dx12RvmBlendDescTrait: TypeObject {
}

impl Dx12RvmBlendDescTrait for Dx12RvmBlendDesc {
}

pub static DX12RVMBLENDDESC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmBlendDesc",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RvmBlendDesc as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::DX12RVMBLENDDESC_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for Dx12RvmBlendDesc {
    fn type_info(&self) -> &'static TypeInfo {
        DX12RVMBLENDDESC_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12RootDescriptorTableAssemblyDebugInfo {
}

pub trait RvmSerializedDb_ns_Dx12RootDescriptorTableAssemblyDebugInfoTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12RootDescriptorTableAssemblyDebugInfoTrait for RvmSerializedDb_ns_Dx12RootDescriptorTableAssemblyDebugInfo {
}

pub static RVMSERIALIZEDDB_NS_DX12ROOTDESCRIPTORTABLEASSEMBLYDEBUGINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RootDescriptorTableAssemblyDebugInfo",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12RootDescriptorTableAssemblyDebugInfo as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12ROOTDESCRIPTORTABLEASSEMBLYDEBUGINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx12RootDescriptorTableAssemblyDebugInfo {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12ROOTDESCRIPTORTABLEASSEMBLYDEBUGINFO_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12BlendStateData {
}

pub trait RvmSerializedDb_ns_Dx12BlendStateDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12BlendStateDataTrait for RvmSerializedDb_ns_Dx12BlendStateData {
}

pub static RVMSERIALIZEDDB_NS_DX12BLENDSTATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12BlendStateData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12BlendStateData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12BLENDSTATEDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RvmSerializedDb_ns_Dx12BlendStateData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12BLENDSTATEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12RootDescriptorTableAssemblyInstructionData {
}

pub trait RvmSerializedDb_ns_Dx12RootDescriptorTableAssemblyInstructionDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12RootDescriptorTableAssemblyInstructionDataTrait for RvmSerializedDb_ns_Dx12RootDescriptorTableAssemblyInstructionData {
}

pub static RVMSERIALIZEDDB_NS_DX12ROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RootDescriptorTableAssemblyInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12RootDescriptorTableAssemblyInstructionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12ROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx12RootDescriptorTableAssemblyInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12ROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12RootWriteOp {
}

pub trait RvmSerializedDb_ns_Dx12RootWriteOpTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12RootWriteOpTrait for RvmSerializedDb_ns_Dx12RootWriteOp {
}

pub static RVMSERIALIZEDDB_NS_DX12ROOTWRITEOP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RootWriteOp",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12RootWriteOp as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12ROOTWRITEOP_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12RootWriteOp {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12ROOTWRITEOP_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12LegacyDrawStateBuilderInstructionBatchData {
}

pub trait RvmSerializedDb_ns_Dx12LegacyDrawStateBuilderInstructionBatchDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12LegacyDrawStateBuilderInstructionBatchDataTrait for RvmSerializedDb_ns_Dx12LegacyDrawStateBuilderInstructionBatchData {
}

pub static RVMSERIALIZEDDB_NS_DX12LEGACYDRAWSTATEBUILDERINSTRUCTIONBATCHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12LegacyDrawStateBuilderInstructionBatchData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12LegacyDrawStateBuilderInstructionBatchData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12LEGACYDRAWSTATEBUILDERINSTRUCTIONBATCHDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx12LegacyDrawStateBuilderInstructionBatchData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12LEGACYDRAWSTATEBUILDERINSTRUCTIONBATCHDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12LegacyDrawStateBuilderInstructionData {
}

pub trait RvmSerializedDb_ns_Dx12LegacyDrawStateBuilderInstructionDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12LegacyDrawStateBuilderInstructionDataTrait for RvmSerializedDb_ns_Dx12LegacyDrawStateBuilderInstructionData {
}

pub static RVMSERIALIZEDDB_NS_DX12LEGACYDRAWSTATEBUILDERINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12LegacyDrawStateBuilderInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12LegacyDrawStateBuilderInstructionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12LEGACYDRAWSTATEBUILDERINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx12LegacyDrawStateBuilderInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12LEGACYDRAWSTATEBUILDERINSTRUCTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12ShaderDispatchDrawInstructionData {
}

pub trait RvmSerializedDb_ns_Dx12ShaderDispatchDrawInstructionDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12ShaderDispatchDrawInstructionDataTrait for RvmSerializedDb_ns_Dx12ShaderDispatchDrawInstructionData {
}

pub static RVMSERIALIZEDDB_NS_DX12SHADERDISPATCHDRAWINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12ShaderDispatchDrawInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12ShaderDispatchDrawInstructionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12SHADERDISPATCHDRAWINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12ShaderDispatchDrawInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12SHADERDISPATCHDRAWINSTRUCTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12RootSignatureDebugInfo {
}

pub trait RvmSerializedDb_ns_Dx12RootSignatureDebugInfoTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12RootSignatureDebugInfoTrait for RvmSerializedDb_ns_Dx12RootSignatureDebugInfo {
}

pub static RVMSERIALIZEDDB_NS_DX12ROOTSIGNATUREDEBUGINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RootSignatureDebugInfo",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12RootSignatureDebugInfo as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12ROOTSIGNATUREDEBUGINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx12RootSignatureDebugInfo {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12ROOTSIGNATUREDEBUGINFO_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12TableDebugInfo {
}

pub trait RvmSerializedDb_ns_Dx12TableDebugInfoTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12TableDebugInfoTrait for RvmSerializedDb_ns_Dx12TableDebugInfo {
}

pub static RVMSERIALIZEDDB_NS_DX12TABLEDEBUGINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12TableDebugInfo",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12TableDebugInfo as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12TABLEDEBUGINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx12TableDebugInfo {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12TABLEDEBUGINFO_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12TableDebugEntry {
}

pub trait RvmSerializedDb_ns_Dx12TableDebugEntryTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12TableDebugEntryTrait for RvmSerializedDb_ns_Dx12TableDebugEntry {
}

pub static RVMSERIALIZEDDB_NS_DX12TABLEDEBUGENTRY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12TableDebugEntry",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12TableDebugEntry as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12TABLEDEBUGENTRY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx12TableDebugEntry {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12TABLEDEBUGENTRY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12InputElement {
}

pub trait RvmSerializedDb_ns_Dx12InputElementTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12InputElementTrait for RvmSerializedDb_ns_Dx12InputElement {
}

pub static RVMSERIALIZEDDB_NS_DX12INPUTELEMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12InputElement",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12InputElement as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12INPUTELEMENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx12InputElement {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12INPUTELEMENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12Shader {
}

pub trait RvmSerializedDb_ns_Dx12ShaderTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12ShaderTrait for RvmSerializedDb_ns_Dx12Shader {
}

pub static RVMSERIALIZEDDB_NS_DX12SHADER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12Shader",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12Shader as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12SHADER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx12Shader {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12SHADER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12BinaryBlob {
}

pub trait RvmSerializedDb_ns_Dx12BinaryBlobTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12BinaryBlobTrait for RvmSerializedDb_ns_Dx12BinaryBlob {
}

pub static RVMSERIALIZEDDB_NS_DX12BINARYBLOB_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12BinaryBlob",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12BinaryBlob as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12BINARYBLOB_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RvmSerializedDb_ns_Dx12BinaryBlob {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12BINARYBLOB_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12ShaderState {
}

pub trait RvmSerializedDb_ns_Dx12ShaderStateTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12ShaderStateTrait for RvmSerializedDb_ns_Dx12ShaderState {
}

pub static RVMSERIALIZEDDB_NS_DX12SHADERSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12ShaderState",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12ShaderState as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12SHADERSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RvmSerializedDb_ns_Dx12ShaderState {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12SHADERSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

