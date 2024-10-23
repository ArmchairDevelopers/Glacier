use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_nv_rvm_dx12_types(registry: &mut TypeRegistry) {
    registry.register_type(DX12NVRVMBACKENDCONFIG_TYPE_INFO);
    registry.register_type(DX12NVRVMBACKENDCONFIG_ARRAY_TYPE_INFO);
    registry.register_type(DX12NVRVMROOTSIGNATURE_TYPE_INFO);
    registry.register_type(DX12NVRVMCONSTANTBUFFERASSEMBLYINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX12NVRVMCONSTANTBUFFERASSEMBLYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DX12NVRVMLEGACYDRAWSTATEBUILDERINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX12NVRVMLEGACYDRAWSTATEBUILDERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DX12NVRVMDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX12NVRVMDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DX12NVRVMDISPATCHINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX12NVRVMDISPATCHINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DX12NVRVMDATABASELOADER_TYPE_INFO);
    registry.register_type(DX12NVRVMDATABASELOADER_ARRAY_TYPE_INFO);
    registry.register_type(DX12NVRVMDATABASE_TYPE_INFO);
    registry.register_type(DX12NVRVMDATABASE_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12NVDESCRIPTORTABLEASSEMBLYINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12NVLEGACYDRAWSTATEBUILDERINSTRUCTIONBATCHDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12NVLEGACYDRAWSTATEBUILDERINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12NVCONSTANTBUFFERASSEMBLYINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12NVDESCRIPTORTABLE_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12NVRVMROOTSIGNATURE_TYPE_INFO);
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct Dx12NvRvmBackendConfig {
    pub _glacier_base: super::rvm_dx12_pc::Dx12PcRvmBackendConfig,
}

pub trait Dx12NvRvmBackendConfigTrait: super::rvm_dx12_pc::Dx12PcRvmBackendConfigTrait {
}

impl Dx12NvRvmBackendConfigTrait for Dx12NvRvmBackendConfig {
}

impl super::rvm_dx12_pc::Dx12PcRvmBackendConfigTrait for Dx12NvRvmBackendConfig {
}

impl super::rvm_dx12::Dx12RvmBackendConfigTrait for Dx12NvRvmBackendConfig {
}

impl super::rvm_common::RvmBackendConfigTrait for Dx12NvRvmBackendConfig {
}

impl super::core::AssetTrait for Dx12NvRvmBackendConfig {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for Dx12NvRvmBackendConfig {
}

pub static DX12NVRVMBACKENDCONFIG_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12NvRvmBackendConfig",
    name_hash: 222804901,
    flags: MemberInfoFlags::new(101),
    module: "NvRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_dx12_pc::DX12PCRVMBACKENDCONFIG_TYPE_INFO),
        super_class_offset: offset_of!(Dx12NvRvmBackendConfig, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12NvRvmBackendConfig as Default>::default())),
            create_boxed: || Box::new(<Dx12NvRvmBackendConfig as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12NVRVMBACKENDCONFIG_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12NvRvmBackendConfig {
    fn type_info(&self) -> &'static TypeInfo {
        DX12NVRVMBACKENDCONFIG_TYPE_INFO
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


pub static DX12NVRVMBACKENDCONFIG_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12NvRvmBackendConfig-Array",
    name_hash: 1532711953,
    flags: MemberInfoFlags::new(145),
    module: "NvRvmDx12",
    data: TypeInfoData::Array("Dx12NvRvmBackendConfig"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Dx12NvRvmRootSignature {
}

pub trait Dx12NvRvmRootSignatureTrait: TypeObject {
}

impl Dx12NvRvmRootSignatureTrait for Dx12NvRvmRootSignature {
}

pub static DX12NVRVMROOTSIGNATURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12NvRvmRootSignature",
    name_hash: 224866665,
    flags: MemberInfoFlags::new(53321),
    module: "NvRvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12NvRvmRootSignature as Default>::default())),
            create_boxed: || Box::new(<Dx12NvRvmRootSignature as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::DX12NVRVMROOTSIGNATURE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12NvRvmRootSignature {
    fn type_info(&self) -> &'static TypeInfo {
        DX12NVRVMROOTSIGNATURE_TYPE_INFO
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
pub struct Dx12NvRvmConstantBufferAssemblyInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx12NvRvmConstantBufferAssemblyInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx12NvRvmConstantBufferAssemblyInstructionFactoryTrait for Dx12NvRvmConstantBufferAssemblyInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx12NvRvmConstantBufferAssemblyInstructionFactory {
}

pub static DX12NVRVMCONSTANTBUFFERASSEMBLYINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12NvRvmConstantBufferAssemblyInstructionFactory",
    name_hash: 395768583,
    flags: MemberInfoFlags::new(101),
    module: "NvRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        super_class_offset: offset_of!(Dx12NvRvmConstantBufferAssemblyInstructionFactory, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12NvRvmConstantBufferAssemblyInstructionFactory as Default>::default())),
            create_boxed: || Box::new(<Dx12NvRvmConstantBufferAssemblyInstructionFactory as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12NVRVMCONSTANTBUFFERASSEMBLYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12NvRvmConstantBufferAssemblyInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX12NVRVMCONSTANTBUFFERASSEMBLYINSTRUCTIONFACTORY_TYPE_INFO
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


pub static DX12NVRVMCONSTANTBUFFERASSEMBLYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12NvRvmConstantBufferAssemblyInstructionFactory-Array",
    name_hash: 1403179955,
    flags: MemberInfoFlags::new(145),
    module: "NvRvmDx12",
    data: TypeInfoData::Array("Dx12NvRvmConstantBufferAssemblyInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Dx12NvRvmLegacyDrawStateBuilderInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx12NvRvmLegacyDrawStateBuilderInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx12NvRvmLegacyDrawStateBuilderInstructionFactoryTrait for Dx12NvRvmLegacyDrawStateBuilderInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx12NvRvmLegacyDrawStateBuilderInstructionFactory {
}

pub static DX12NVRVMLEGACYDRAWSTATEBUILDERINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12NvRvmLegacyDrawStateBuilderInstructionFactory",
    name_hash: 3503894724,
    flags: MemberInfoFlags::new(101),
    module: "NvRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        super_class_offset: offset_of!(Dx12NvRvmLegacyDrawStateBuilderInstructionFactory, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12NvRvmLegacyDrawStateBuilderInstructionFactory as Default>::default())),
            create_boxed: || Box::new(<Dx12NvRvmLegacyDrawStateBuilderInstructionFactory as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12NVRVMLEGACYDRAWSTATEBUILDERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12NvRvmLegacyDrawStateBuilderInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX12NVRVMLEGACYDRAWSTATEBUILDERINSTRUCTIONFACTORY_TYPE_INFO
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


pub static DX12NVRVMLEGACYDRAWSTATEBUILDERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12NvRvmLegacyDrawStateBuilderInstructionFactory-Array",
    name_hash: 2236421488,
    flags: MemberInfoFlags::new(145),
    module: "NvRvmDx12",
    data: TypeInfoData::Array("Dx12NvRvmLegacyDrawStateBuilderInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Dx12NvRvmDescriptorTableAssemblyInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx12NvRvmDescriptorTableAssemblyInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx12NvRvmDescriptorTableAssemblyInstructionFactoryTrait for Dx12NvRvmDescriptorTableAssemblyInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx12NvRvmDescriptorTableAssemblyInstructionFactory {
}

pub static DX12NVRVMDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12NvRvmDescriptorTableAssemblyInstructionFactory",
    name_hash: 3690563604,
    flags: MemberInfoFlags::new(101),
    module: "NvRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        super_class_offset: offset_of!(Dx12NvRvmDescriptorTableAssemblyInstructionFactory, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12NvRvmDescriptorTableAssemblyInstructionFactory as Default>::default())),
            create_boxed: || Box::new(<Dx12NvRvmDescriptorTableAssemblyInstructionFactory as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12NVRVMDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12NvRvmDescriptorTableAssemblyInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX12NVRVMDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_TYPE_INFO
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


pub static DX12NVRVMDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12NvRvmDescriptorTableAssemblyInstructionFactory-Array",
    name_hash: 2020647968,
    flags: MemberInfoFlags::new(145),
    module: "NvRvmDx12",
    data: TypeInfoData::Array("Dx12NvRvmDescriptorTableAssemblyInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Dx12NvRvmDispatchInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx12NvRvmDispatchInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx12NvRvmDispatchInstructionFactoryTrait for Dx12NvRvmDispatchInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx12NvRvmDispatchInstructionFactory {
}

pub static DX12NVRVMDISPATCHINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12NvRvmDispatchInstructionFactory",
    name_hash: 112413783,
    flags: MemberInfoFlags::new(101),
    module: "NvRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        super_class_offset: offset_of!(Dx12NvRvmDispatchInstructionFactory, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12NvRvmDispatchInstructionFactory as Default>::default())),
            create_boxed: || Box::new(<Dx12NvRvmDispatchInstructionFactory as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12NVRVMDISPATCHINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12NvRvmDispatchInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX12NVRVMDISPATCHINSTRUCTIONFACTORY_TYPE_INFO
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


pub static DX12NVRVMDISPATCHINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12NvRvmDispatchInstructionFactory-Array",
    name_hash: 732931427,
    flags: MemberInfoFlags::new(145),
    module: "NvRvmDx12",
    data: TypeInfoData::Array("Dx12NvRvmDispatchInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Dx12NvRvmDatabaseLoader {
    pub _glacier_base: super::rvm_common::RvmCommonDatabaseLoader,
}

pub trait Dx12NvRvmDatabaseLoaderTrait: super::rvm_common::RvmCommonDatabaseLoaderTrait {
}

impl Dx12NvRvmDatabaseLoaderTrait for Dx12NvRvmDatabaseLoader {
}

impl super::rvm_common::RvmCommonDatabaseLoaderTrait for Dx12NvRvmDatabaseLoader {
}

impl super::render::RvmDatabaseLoaderTrait for Dx12NvRvmDatabaseLoader {
}

pub static DX12NVRVMDATABASELOADER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12NvRvmDatabaseLoader",
    name_hash: 3425495583,
    flags: MemberInfoFlags::new(101),
    module: "NvRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMCOMMONDATABASELOADER_TYPE_INFO),
        super_class_offset: offset_of!(Dx12NvRvmDatabaseLoader, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12NvRvmDatabaseLoader as Default>::default())),
            create_boxed: || Box::new(<Dx12NvRvmDatabaseLoader as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12NVRVMDATABASELOADER_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for Dx12NvRvmDatabaseLoader {
    fn type_info(&self) -> &'static TypeInfo {
        DX12NVRVMDATABASELOADER_TYPE_INFO
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


pub static DX12NVRVMDATABASELOADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12NvRvmDatabaseLoader-Array",
    name_hash: 3869781675,
    flags: MemberInfoFlags::new(145),
    module: "NvRvmDx12",
    data: TypeInfoData::Array("Dx12NvRvmDatabaseLoader"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Dx12NvRvmDatabase {
    pub _glacier_base: super::rvm_dx12_pc::Dx12PcRvmDatabase,
}

pub trait Dx12NvRvmDatabaseTrait: super::rvm_dx12_pc::Dx12PcRvmDatabaseTrait {
}

impl Dx12NvRvmDatabaseTrait for Dx12NvRvmDatabase {
}

impl super::rvm_dx12_pc::Dx12PcRvmDatabaseTrait for Dx12NvRvmDatabase {
}

impl super::rvm_dx12::Dx12RvmDatabaseTrait for Dx12NvRvmDatabase {
}

impl super::rvm_common::BaseRvmDatabaseTrait for Dx12NvRvmDatabase {
}

impl super::render::RvmLegacyDatabaseTrait for Dx12NvRvmDatabase {
}

impl super::render::RvmDatabaseTrait for Dx12NvRvmDatabase {
}

impl super::core::IResourceObjectTrait for Dx12NvRvmDatabase {
}

pub static DX12NVRVMDATABASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12NvRvmDatabase",
    name_hash: 3823535918,
    flags: MemberInfoFlags::new(101),
    module: "NvRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_dx12_pc::DX12PCRVMDATABASE_TYPE_INFO),
        super_class_offset: offset_of!(Dx12NvRvmDatabase, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12NvRvmDatabase as Default>::default())),
            create_boxed: || Box::new(<Dx12NvRvmDatabase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12NVRVMDATABASE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12NvRvmDatabase {
    fn type_info(&self) -> &'static TypeInfo {
        DX12NVRVMDATABASE_TYPE_INFO
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


pub static DX12NVRVMDATABASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12NvRvmDatabase-Array",
    name_hash: 598036378,
    flags: MemberInfoFlags::new(145),
    module: "NvRvmDx12",
    data: TypeInfoData::Array("Dx12NvRvmDatabase"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsDx12NvDescriptorTableAssemblyInstructionData {
}

pub trait RvmSerializedDbnsDx12NvDescriptorTableAssemblyInstructionDataTrait: TypeObject {
}

impl RvmSerializedDbnsDx12NvDescriptorTableAssemblyInstructionDataTrait for RvmSerializedDbnsDx12NvDescriptorTableAssemblyInstructionData {
}

pub static RVMSERIALIZEDDB_NS_DX12NVDESCRIPTORTABLEASSEMBLYINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12NvDescriptorTableAssemblyInstructionData",
    name_hash: 1237580921,
    flags: MemberInfoFlags::new(53321),
    module: "NvRvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx12NvDescriptorTableAssemblyInstructionData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDx12NvDescriptorTableAssemblyInstructionData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12NVDESCRIPTORTABLEASSEMBLYINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsDx12NvDescriptorTableAssemblyInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12NVDESCRIPTORTABLEASSEMBLYINSTRUCTIONDATA_TYPE_INFO
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
pub struct RvmSerializedDbnsDx12NvLegacyDrawStateBuilderInstructionBatchData {
}

pub trait RvmSerializedDbnsDx12NvLegacyDrawStateBuilderInstructionBatchDataTrait: TypeObject {
}

impl RvmSerializedDbnsDx12NvLegacyDrawStateBuilderInstructionBatchDataTrait for RvmSerializedDbnsDx12NvLegacyDrawStateBuilderInstructionBatchData {
}

pub static RVMSERIALIZEDDB_NS_DX12NVLEGACYDRAWSTATEBUILDERINSTRUCTIONBATCHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12NvLegacyDrawStateBuilderInstructionBatchData",
    name_hash: 2372238645,
    flags: MemberInfoFlags::new(53321),
    module: "NvRvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx12NvLegacyDrawStateBuilderInstructionBatchData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDx12NvLegacyDrawStateBuilderInstructionBatchData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12NVLEGACYDRAWSTATEBUILDERINSTRUCTIONBATCHDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsDx12NvLegacyDrawStateBuilderInstructionBatchData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12NVLEGACYDRAWSTATEBUILDERINSTRUCTIONBATCHDATA_TYPE_INFO
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
pub struct RvmSerializedDbnsDx12NvLegacyDrawStateBuilderInstructionData {
}

pub trait RvmSerializedDbnsDx12NvLegacyDrawStateBuilderInstructionDataTrait: TypeObject {
}

impl RvmSerializedDbnsDx12NvLegacyDrawStateBuilderInstructionDataTrait for RvmSerializedDbnsDx12NvLegacyDrawStateBuilderInstructionData {
}

pub static RVMSERIALIZEDDB_NS_DX12NVLEGACYDRAWSTATEBUILDERINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12NvLegacyDrawStateBuilderInstructionData",
    name_hash: 2641418569,
    flags: MemberInfoFlags::new(53321),
    module: "NvRvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx12NvLegacyDrawStateBuilderInstructionData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDx12NvLegacyDrawStateBuilderInstructionData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12NVLEGACYDRAWSTATEBUILDERINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsDx12NvLegacyDrawStateBuilderInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12NVLEGACYDRAWSTATEBUILDERINSTRUCTIONDATA_TYPE_INFO
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
pub struct RvmSerializedDbnsDx12NvConstantBufferAssemblyInstructionData {
}

pub trait RvmSerializedDbnsDx12NvConstantBufferAssemblyInstructionDataTrait: TypeObject {
}

impl RvmSerializedDbnsDx12NvConstantBufferAssemblyInstructionDataTrait for RvmSerializedDbnsDx12NvConstantBufferAssemblyInstructionData {
}

pub static RVMSERIALIZEDDB_NS_DX12NVCONSTANTBUFFERASSEMBLYINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12NvConstantBufferAssemblyInstructionData",
    name_hash: 1261634474,
    flags: MemberInfoFlags::new(53321),
    module: "NvRvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx12NvConstantBufferAssemblyInstructionData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDx12NvConstantBufferAssemblyInstructionData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12NVCONSTANTBUFFERASSEMBLYINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RvmSerializedDbnsDx12NvConstantBufferAssemblyInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12NVCONSTANTBUFFERASSEMBLYINSTRUCTIONDATA_TYPE_INFO
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
pub struct RvmSerializedDbnsDx12NvDescriptorTable {
}

pub trait RvmSerializedDbnsDx12NvDescriptorTableTrait: TypeObject {
}

impl RvmSerializedDbnsDx12NvDescriptorTableTrait for RvmSerializedDbnsDx12NvDescriptorTable {
}

pub static RVMSERIALIZEDDB_NS_DX12NVDESCRIPTORTABLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12NvDescriptorTable",
    name_hash: 494473839,
    flags: MemberInfoFlags::new(53321),
    module: "NvRvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx12NvDescriptorTable as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDx12NvDescriptorTable as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12NVDESCRIPTORTABLE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDbnsDx12NvDescriptorTable {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12NVDESCRIPTORTABLE_TYPE_INFO
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
pub struct RvmSerializedDbnsDx12NvRvmRootSignature {
}

pub trait RvmSerializedDbnsDx12NvRvmRootSignatureTrait: TypeObject {
}

impl RvmSerializedDbnsDx12NvRvmRootSignatureTrait for RvmSerializedDbnsDx12NvRvmRootSignature {
}

pub static RVMSERIALIZEDDB_NS_DX12NVRVMROOTSIGNATURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12NvRvmRootSignature",
    name_hash: 1364446921,
    flags: MemberInfoFlags::new(53321),
    module: "NvRvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx12NvRvmRootSignature as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDx12NvRvmRootSignature as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12NVRVMROOTSIGNATURE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsDx12NvRvmRootSignature {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12NVRVMROOTSIGNATURE_TYPE_INFO
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

