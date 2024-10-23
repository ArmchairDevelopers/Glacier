use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsDx12VertexBufferViewInstructionData {
}

pub trait RvmSerializedDbnsDx12VertexBufferViewInstructionDataTrait: TypeObject {
}

impl RvmSerializedDbnsDx12VertexBufferViewInstructionDataTrait for RvmSerializedDbnsDx12VertexBufferViewInstructionData {
}

pub static RVMSERIALIZEDDB_NS_DX12VERTEXBUFFERVIEWINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12VertexBufferViewInstructionData",
    name_hash: 2215087543,
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx12VertexBufferViewInstructionData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDx12VertexBufferViewInstructionData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12VERTEXBUFFERVIEWINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsDx12VertexBufferViewInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12VERTEXBUFFERVIEWINSTRUCTIONDATA_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsDx12ViewStateInstructionData {
}

pub trait RvmSerializedDbnsDx12ViewStateInstructionDataTrait: TypeObject {
}

impl RvmSerializedDbnsDx12ViewStateInstructionDataTrait for RvmSerializedDbnsDx12ViewStateInstructionData {
}

pub static RVMSERIALIZEDDB_NS_DX12VIEWSTATEINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12ViewStateInstructionData",
    name_hash: 461961128,
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx12ViewStateInstructionData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDx12ViewStateInstructionData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12VIEWSTATEINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDbnsDx12ViewStateInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12VIEWSTATEINSTRUCTIONDATA_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
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
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for Dx12RvmBackendConfig {
}

pub static DX12RVMBACKENDCONFIG_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmBackendConfig",
    name_hash: 2086552221,
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMBACKENDCONFIG_TYPE_INFO),
        super_class_offset: offset_of!(Dx12RvmBackendConfig, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RvmBackendConfig as Default>::default())),
            create_boxed: || Box::new(<Dx12RvmBackendConfig as Default>::default()),
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


pub static DX12RVMBACKENDCONFIG_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmBackendConfig-Array",
    name_hash: 526478889,
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12",
    data: TypeInfoData::Array("Dx12RvmBackendConfig"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct D3D12GPUDESCRIPTORHANDLE {
}

pub trait D3D12GPUDESCRIPTORHANDLETrait: TypeObject {
}

impl D3D12GPUDESCRIPTORHANDLETrait for D3D12GPUDESCRIPTORHANDLE {
}

pub static D3D12_GPU_DESCRIPTOR_HANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_GPU_DESCRIPTOR_HANDLE",
    name_hash: 1052301873,
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<D3D12GPUDESCRIPTORHANDLE as Default>::default())),
            create_boxed: || Box::new(<D3D12GPUDESCRIPTORHANDLE as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::D3D12_GPU_DESCRIPTOR_HANDLE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for D3D12GPUDESCRIPTORHANDLE {
    fn type_info(&self) -> &'static TypeInfo {
        D3D12_GPU_DESCRIPTOR_HANDLE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct D3D12VERTEXBUFFERVIEW {
}

pub trait D3D12VERTEXBUFFERVIEWTrait: TypeObject {
}

impl D3D12VERTEXBUFFERVIEWTrait for D3D12VERTEXBUFFERVIEW {
}

pub static D3D12_VERTEX_BUFFER_VIEW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_VERTEX_BUFFER_VIEW",
    name_hash: 1968292079,
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<D3D12VERTEXBUFFERVIEW as Default>::default())),
            create_boxed: || Box::new(<D3D12VERTEXBUFFERVIEW as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::D3D12_VERTEX_BUFFER_VIEW_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for D3D12VERTEXBUFFERVIEW {
    fn type_info(&self) -> &'static TypeInfo {
        D3D12_VERTEX_BUFFER_VIEW_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct Dx12RvmInputElement {
}

pub trait Dx12RvmInputElementTrait: TypeObject {
}

impl Dx12RvmInputElementTrait for Dx12RvmInputElement {
}

pub static DX12RVMINPUTELEMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmInputElement",
    name_hash: 3792252891,
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RvmInputElement as Default>::default())),
            create_boxed: || Box::new(<Dx12RvmInputElement as Default>::default()),
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct Dx12RvmShader {
}

pub trait Dx12RvmShaderTrait: TypeObject {
}

impl Dx12RvmShaderTrait for Dx12RvmShader {
}

pub static DX12RVMSHADER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmShader",
    name_hash: 3676932762,
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RvmShader as Default>::default())),
            create_boxed: || Box::new(<Dx12RvmShader as Default>::default()),
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct Dx12RvmRootSignature {
}

pub trait Dx12RvmRootSignatureTrait: TypeObject {
}

impl Dx12RvmRootSignatureTrait for Dx12RvmRootSignature {
}

pub static DX12RVMROOTSIGNATURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmRootSignature",
    name_hash: 2767206481,
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RvmRootSignature as Default>::default())),
            create_boxed: || Box::new(<Dx12RvmRootSignature as Default>::default()),
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

#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 357230998,
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::BASERVMDATABASE_TYPE_INFO),
        super_class_offset: offset_of!(Dx12RvmDatabase, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RvmDatabase as Default>::default())),
            create_boxed: || Box::new(<Dx12RvmDatabase as Default>::default()),
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


pub static DX12RVMDATABASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmDatabase-Array",
    name_hash: 2454806178,
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12",
    data: TypeInfoData::Array("Dx12RvmDatabase"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct D3D12VIEWPORT {
}

pub trait D3D12VIEWPORTTrait: TypeObject {
}

impl D3D12VIEWPORTTrait for D3D12VIEWPORT {
}

pub static D3D12_VIEWPORT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_VIEWPORT",
    name_hash: 3076772958,
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<D3D12VIEWPORT as Default>::default())),
            create_boxed: || Box::new(<D3D12VIEWPORT as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::D3D12_VIEWPORT_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for D3D12VIEWPORT {
    fn type_info(&self) -> &'static TypeInfo {
        D3D12_VIEWPORT_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct D3D12PIPELINESTATEFLAGS {
}

pub trait D3D12PIPELINESTATEFLAGSTrait: TypeObject {
}

impl D3D12PIPELINESTATEFLAGSTrait for D3D12PIPELINESTATEFLAGS {
}

pub static D3D12_PIPELINE_STATE_FLAGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_PIPELINE_STATE_FLAGS",
    name_hash: 4065535168,
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<D3D12PIPELINESTATEFLAGS as Default>::default())),
            create_boxed: || Box::new(<D3D12PIPELINESTATEFLAGS as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::D3D12_PIPELINE_STATE_FLAGS_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for D3D12PIPELINESTATEFLAGS {
    fn type_info(&self) -> &'static TypeInfo {
        D3D12_PIPELINE_STATE_FLAGS_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct D3D12CACHEDPIPELINESTATE {
}

pub trait D3D12CACHEDPIPELINESTATETrait: TypeObject {
}

impl D3D12CACHEDPIPELINESTATETrait for D3D12CACHEDPIPELINESTATE {
}

pub static D3D12_CACHED_PIPELINE_STATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_CACHED_PIPELINE_STATE",
    name_hash: 231477687,
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<D3D12CACHEDPIPELINESTATE as Default>::default())),
            create_boxed: || Box::new(<D3D12CACHEDPIPELINESTATE as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::D3D12_CACHED_PIPELINE_STATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for D3D12CACHEDPIPELINESTATE {
    fn type_info(&self) -> &'static TypeInfo {
        D3D12_CACHED_PIPELINE_STATE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct DXGISAMPLEDESC {
}

pub trait DXGISAMPLEDESCTrait: TypeObject {
}

impl DXGISAMPLEDESCTrait for DXGISAMPLEDESC {
}

pub static DXGI_SAMPLE_DESC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DXGI_SAMPLE_DESC",
    name_hash: 2785642656,
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DXGISAMPLEDESC as Default>::default())),
            create_boxed: || Box::new(<DXGISAMPLEDESC as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::DXGI_SAMPLE_DESC_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for DXGISAMPLEDESC {
    fn type_info(&self) -> &'static TypeInfo {
        DXGI_SAMPLE_DESC_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct D3D12INDEXBUFFERSTRIPCUTVALUE {
}

pub trait D3D12INDEXBUFFERSTRIPCUTVALUETrait: TypeObject {
}

impl D3D12INDEXBUFFERSTRIPCUTVALUETrait for D3D12INDEXBUFFERSTRIPCUTVALUE {
}

pub static D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_INDEX_BUFFER_STRIP_CUT_VALUE",
    name_hash: 458481681,
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<D3D12INDEXBUFFERSTRIPCUTVALUE as Default>::default())),
            create_boxed: || Box::new(<D3D12INDEXBUFFERSTRIPCUTVALUE as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for D3D12INDEXBUFFERSTRIPCUTVALUE {
    fn type_info(&self) -> &'static TypeInfo {
        D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct DXGIFORMAT {
}

pub trait DXGIFORMATTrait: TypeObject {
}

impl DXGIFORMATTrait for DXGIFORMAT {
}

pub static DXGI_FORMAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DXGI_FORMAT",
    name_hash: 1630219179,
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DXGIFORMAT as Default>::default())),
            create_boxed: || Box::new(<DXGIFORMAT as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::DXGI_FORMAT_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for DXGIFORMAT {
    fn type_info(&self) -> &'static TypeInfo {
        DXGI_FORMAT_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct D3D12PRIMITIVETOPOLOGY {
}

pub trait D3D12PRIMITIVETOPOLOGYTrait: TypeObject {
}

impl D3D12PRIMITIVETOPOLOGYTrait for D3D12PRIMITIVETOPOLOGY {
}

pub static D3D12_PRIMITIVE_TOPOLOGY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_PRIMITIVE_TOPOLOGY",
    name_hash: 2283325709,
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<D3D12PRIMITIVETOPOLOGY as Default>::default())),
            create_boxed: || Box::new(<D3D12PRIMITIVETOPOLOGY as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::D3D12_PRIMITIVE_TOPOLOGY_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for D3D12PRIMITIVETOPOLOGY {
    fn type_info(&self) -> &'static TypeInfo {
        D3D12_PRIMITIVE_TOPOLOGY_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct D3D12PRIMITIVETOPOLOGYTYPE {
}

pub trait D3D12PRIMITIVETOPOLOGYTYPETrait: TypeObject {
}

impl D3D12PRIMITIVETOPOLOGYTYPETrait for D3D12PRIMITIVETOPOLOGYTYPE {
}

pub static D3D12_PRIMITIVE_TOPOLOGY_TYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_PRIMITIVE_TOPOLOGY_TYPE",
    name_hash: 1464862634,
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<D3D12PRIMITIVETOPOLOGYTYPE as Default>::default())),
            create_boxed: || Box::new(<D3D12PRIMITIVETOPOLOGYTYPE as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::D3D12_PRIMITIVE_TOPOLOGY_TYPE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for D3D12PRIMITIVETOPOLOGYTYPE {
    fn type_info(&self) -> &'static TypeInfo {
        D3D12_PRIMITIVE_TOPOLOGY_TYPE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct Dx12RvmSoDeclarationEntry {
}

pub trait Dx12RvmSoDeclarationEntryTrait: TypeObject {
}

impl Dx12RvmSoDeclarationEntryTrait for Dx12RvmSoDeclarationEntry {
}

pub static DX12RVMSODECLARATIONENTRY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmSoDeclarationEntry",
    name_hash: 1521086555,
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RvmSoDeclarationEntry as Default>::default())),
            create_boxed: || Box::new(<Dx12RvmSoDeclarationEntry as Default>::default()),
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct Dx12RvmDepthStencilDesc {
}

pub trait Dx12RvmDepthStencilDescTrait: TypeObject {
}

impl Dx12RvmDepthStencilDescTrait for Dx12RvmDepthStencilDesc {
}

pub static DX12RVMDEPTHSTENCILDESC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmDepthStencilDesc",
    name_hash: 3664277285,
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RvmDepthStencilDesc as Default>::default())),
            create_boxed: || Box::new(<Dx12RvmDepthStencilDesc as Default>::default()),
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct Dx12RvmRasterizerDesc {
}

pub trait Dx12RvmRasterizerDescTrait: TypeObject {
}

impl Dx12RvmRasterizerDescTrait for Dx12RvmRasterizerDesc {
}

pub static DX12RVMRASTERIZERDESC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmRasterizerDesc",
    name_hash: 968716773,
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RvmRasterizerDesc as Default>::default())),
            create_boxed: || Box::new(<Dx12RvmRasterizerDesc as Default>::default()),
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct Dx12RvmBlendDesc {
}

pub trait Dx12RvmBlendDescTrait: TypeObject {
}

impl Dx12RvmBlendDescTrait for Dx12RvmBlendDesc {
}

pub static DX12RVMBLENDDESC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmBlendDesc",
    name_hash: 3162989539,
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RvmBlendDesc as Default>::default())),
            create_boxed: || Box::new(<Dx12RvmBlendDesc as Default>::default()),
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsDx12RootDescriptorTableAssemblyDebugInfo {
}

pub trait RvmSerializedDbnsDx12RootDescriptorTableAssemblyDebugInfoTrait: TypeObject {
}

impl RvmSerializedDbnsDx12RootDescriptorTableAssemblyDebugInfoTrait for RvmSerializedDbnsDx12RootDescriptorTableAssemblyDebugInfo {
}

pub static RVMSERIALIZEDDB_NS_DX12ROOTDESCRIPTORTABLEASSEMBLYDEBUGINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RootDescriptorTableAssemblyDebugInfo",
    name_hash: 3471357168,
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx12RootDescriptorTableAssemblyDebugInfo as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDx12RootDescriptorTableAssemblyDebugInfo as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12ROOTDESCRIPTORTABLEASSEMBLYDEBUGINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsDx12RootDescriptorTableAssemblyDebugInfo {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12ROOTDESCRIPTORTABLEASSEMBLYDEBUGINFO_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsDx12BlendStateData {
}

pub trait RvmSerializedDbnsDx12BlendStateDataTrait: TypeObject {
}

impl RvmSerializedDbnsDx12BlendStateDataTrait for RvmSerializedDbnsDx12BlendStateData {
}

pub static RVMSERIALIZEDDB_NS_DX12BLENDSTATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12BlendStateData",
    name_hash: 878882684,
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx12BlendStateData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDx12BlendStateData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12BLENDSTATEDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RvmSerializedDbnsDx12BlendStateData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12BLENDSTATEDATA_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsDx12RootDescriptorTableAssemblyInstructionData {
}

pub trait RvmSerializedDbnsDx12RootDescriptorTableAssemblyInstructionDataTrait: TypeObject {
}

impl RvmSerializedDbnsDx12RootDescriptorTableAssemblyInstructionDataTrait for RvmSerializedDbnsDx12RootDescriptorTableAssemblyInstructionData {
}

pub static RVMSERIALIZEDDB_NS_DX12ROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RootDescriptorTableAssemblyInstructionData",
    name_hash: 3274420743,
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx12RootDescriptorTableAssemblyInstructionData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDx12RootDescriptorTableAssemblyInstructionData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12ROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsDx12RootDescriptorTableAssemblyInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12ROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONDATA_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsDx12RootWriteOp {
}

pub trait RvmSerializedDbnsDx12RootWriteOpTrait: TypeObject {
}

impl RvmSerializedDbnsDx12RootWriteOpTrait for RvmSerializedDbnsDx12RootWriteOp {
}

pub static RVMSERIALIZEDDB_NS_DX12ROOTWRITEOP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RootWriteOp",
    name_hash: 1627462398,
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx12RootWriteOp as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDx12RootWriteOp as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12ROOTWRITEOP_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDbnsDx12RootWriteOp {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12ROOTWRITEOP_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsDx12LegacyDrawStateBuilderInstructionBatchData {
}

pub trait RvmSerializedDbnsDx12LegacyDrawStateBuilderInstructionBatchDataTrait: TypeObject {
}

impl RvmSerializedDbnsDx12LegacyDrawStateBuilderInstructionBatchDataTrait for RvmSerializedDbnsDx12LegacyDrawStateBuilderInstructionBatchData {
}

pub static RVMSERIALIZEDDB_NS_DX12LEGACYDRAWSTATEBUILDERINSTRUCTIONBATCHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12LegacyDrawStateBuilderInstructionBatchData",
    name_hash: 347975885,
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx12LegacyDrawStateBuilderInstructionBatchData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDx12LegacyDrawStateBuilderInstructionBatchData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12LEGACYDRAWSTATEBUILDERINSTRUCTIONBATCHDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsDx12LegacyDrawStateBuilderInstructionBatchData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12LEGACYDRAWSTATEBUILDERINSTRUCTIONBATCHDATA_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsDx12LegacyDrawStateBuilderInstructionData {
}

pub trait RvmSerializedDbnsDx12LegacyDrawStateBuilderInstructionDataTrait: TypeObject {
}

impl RvmSerializedDbnsDx12LegacyDrawStateBuilderInstructionDataTrait for RvmSerializedDbnsDx12LegacyDrawStateBuilderInstructionData {
}

pub static RVMSERIALIZEDDB_NS_DX12LEGACYDRAWSTATEBUILDERINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12LegacyDrawStateBuilderInstructionData",
    name_hash: 1855928305,
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx12LegacyDrawStateBuilderInstructionData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDx12LegacyDrawStateBuilderInstructionData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12LEGACYDRAWSTATEBUILDERINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsDx12LegacyDrawStateBuilderInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12LEGACYDRAWSTATEBUILDERINSTRUCTIONDATA_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsDx12ShaderDispatchDrawInstructionData {
}

pub trait RvmSerializedDbnsDx12ShaderDispatchDrawInstructionDataTrait: TypeObject {
}

impl RvmSerializedDbnsDx12ShaderDispatchDrawInstructionDataTrait for RvmSerializedDbnsDx12ShaderDispatchDrawInstructionData {
}

pub static RVMSERIALIZEDDB_NS_DX12SHADERDISPATCHDRAWINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12ShaderDispatchDrawInstructionData",
    name_hash: 4280471467,
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx12ShaderDispatchDrawInstructionData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDx12ShaderDispatchDrawInstructionData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12SHADERDISPATCHDRAWINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDbnsDx12ShaderDispatchDrawInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12SHADERDISPATCHDRAWINSTRUCTIONDATA_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsDx12RootSignatureDebugInfo {
}

pub trait RvmSerializedDbnsDx12RootSignatureDebugInfoTrait: TypeObject {
}

impl RvmSerializedDbnsDx12RootSignatureDebugInfoTrait for RvmSerializedDbnsDx12RootSignatureDebugInfo {
}

pub static RVMSERIALIZEDDB_NS_DX12ROOTSIGNATUREDEBUGINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RootSignatureDebugInfo",
    name_hash: 4261286215,
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx12RootSignatureDebugInfo as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDx12RootSignatureDebugInfo as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12ROOTSIGNATUREDEBUGINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsDx12RootSignatureDebugInfo {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12ROOTSIGNATUREDEBUGINFO_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsDx12TableDebugInfo {
}

pub trait RvmSerializedDbnsDx12TableDebugInfoTrait: TypeObject {
}

impl RvmSerializedDbnsDx12TableDebugInfoTrait for RvmSerializedDbnsDx12TableDebugInfo {
}

pub static RVMSERIALIZEDDB_NS_DX12TABLEDEBUGINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12TableDebugInfo",
    name_hash: 3772344155,
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx12TableDebugInfo as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDx12TableDebugInfo as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12TABLEDEBUGINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsDx12TableDebugInfo {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12TABLEDEBUGINFO_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsDx12TableDebugEntry {
}

pub trait RvmSerializedDbnsDx12TableDebugEntryTrait: TypeObject {
}

impl RvmSerializedDbnsDx12TableDebugEntryTrait for RvmSerializedDbnsDx12TableDebugEntry {
}

pub static RVMSERIALIZEDDB_NS_DX12TABLEDEBUGENTRY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12TableDebugEntry",
    name_hash: 4214876385,
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx12TableDebugEntry as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDx12TableDebugEntry as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12TABLEDEBUGENTRY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsDx12TableDebugEntry {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12TABLEDEBUGENTRY_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsDx12InputElement {
}

pub trait RvmSerializedDbnsDx12InputElementTrait: TypeObject {
}

impl RvmSerializedDbnsDx12InputElementTrait for RvmSerializedDbnsDx12InputElement {
}

pub static RVMSERIALIZEDDB_NS_DX12INPUTELEMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12InputElement",
    name_hash: 2297885618,
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx12InputElement as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDx12InputElement as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12INPUTELEMENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsDx12InputElement {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12INPUTELEMENT_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsDx12Shader {
}

pub trait RvmSerializedDbnsDx12ShaderTrait: TypeObject {
}

impl RvmSerializedDbnsDx12ShaderTrait for RvmSerializedDbnsDx12Shader {
}

pub static RVMSERIALIZEDDB_NS_DX12SHADER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12Shader",
    name_hash: 396895731,
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx12Shader as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDx12Shader as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12SHADER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsDx12Shader {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12SHADER_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsDx12BinaryBlob {
}

pub trait RvmSerializedDbnsDx12BinaryBlobTrait: TypeObject {
}

impl RvmSerializedDbnsDx12BinaryBlobTrait for RvmSerializedDbnsDx12BinaryBlob {
}

pub static RVMSERIALIZEDDB_NS_DX12BINARYBLOB_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12BinaryBlob",
    name_hash: 1085845654,
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx12BinaryBlob as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDx12BinaryBlob as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12BINARYBLOB_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RvmSerializedDbnsDx12BinaryBlob {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12BINARYBLOB_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsDx12ShaderState {
}

pub trait RvmSerializedDbnsDx12ShaderStateTrait: TypeObject {
}

impl RvmSerializedDbnsDx12ShaderStateTrait for RvmSerializedDbnsDx12ShaderState {
}

pub static RVMSERIALIZEDDB_NS_DX12SHADERSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12ShaderState",
    name_hash: 2696736004,
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx12ShaderState as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDx12ShaderState as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12SHADERSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RvmSerializedDbnsDx12ShaderState {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12SHADERSTATE_TYPE_INFO
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

