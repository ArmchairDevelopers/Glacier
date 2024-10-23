use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_nv_rvm_dx11_types(registry: &mut TypeRegistry) {
    registry.register_type(DX11NVDRAWSTATEFTINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX11NVDRAWSTATEFTINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DX11NVVIEWSTATEFTINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX11NVVIEWSTATEFTINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DX11NVDRAWSTATEDEPTHINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX11NVDRAWSTATEDEPTHINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DX11NVVIEWSTATEDEPTHINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX11NVVIEWSTATEDEPTHINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11NVVIEWSTATEDEPTHINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11NVDRAWSTATEINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11NVVIEWSTATEINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(DX11NVRVMBACKENDCONFIG_TYPE_INFO);
    registry.register_type(DX11NVRVMBACKENDCONFIG_ARRAY_TYPE_INFO);
    registry.register_type(DX11NVRVMDATABASELOADER_TYPE_INFO);
    registry.register_type(DX11NVRVMDATABASELOADER_ARRAY_TYPE_INFO);
    registry.register_type(DX11NVRVMDATABASE_TYPE_INFO);
    registry.register_type(DX11NVRVMDATABASE_ARRAY_TYPE_INFO);
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct Dx11NvDrawStateFTInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx11NvDrawStateFTInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx11NvDrawStateFTInstructionFactoryTrait for Dx11NvDrawStateFTInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx11NvDrawStateFTInstructionFactory {
}

pub static DX11NVDRAWSTATEFTINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11NvDrawStateFTInstructionFactory",
    name_hash: 901096744,
    flags: MemberInfoFlags::new(101),
    module: "NvRvmDx11",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        super_class_offset: offset_of!(Dx11NvDrawStateFTInstructionFactory, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx11NvDrawStateFTInstructionFactory as Default>::default())),
            create_boxed: || Box::new(<Dx11NvDrawStateFTInstructionFactory as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX11NVDRAWSTATEFTINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11NvDrawStateFTInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX11NVDRAWSTATEFTINSTRUCTIONFACTORY_TYPE_INFO
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


pub static DX11NVDRAWSTATEFTINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11NvDrawStateFTInstructionFactory-Array",
    name_hash: 3965372060,
    flags: MemberInfoFlags::new(145),
    module: "NvRvmDx11",
    data: TypeInfoData::Array("Dx11NvDrawStateFTInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Dx11NvViewStateFTInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx11NvViewStateFTInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx11NvViewStateFTInstructionFactoryTrait for Dx11NvViewStateFTInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx11NvViewStateFTInstructionFactory {
}

pub static DX11NVVIEWSTATEFTINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11NvViewStateFTInstructionFactory",
    name_hash: 2737440709,
    flags: MemberInfoFlags::new(101),
    module: "NvRvmDx11",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        super_class_offset: offset_of!(Dx11NvViewStateFTInstructionFactory, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx11NvViewStateFTInstructionFactory as Default>::default())),
            create_boxed: || Box::new(<Dx11NvViewStateFTInstructionFactory as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX11NVVIEWSTATEFTINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11NvViewStateFTInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX11NVVIEWSTATEFTINSTRUCTIONFACTORY_TYPE_INFO
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


pub static DX11NVVIEWSTATEFTINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11NvViewStateFTInstructionFactory-Array",
    name_hash: 2800091121,
    flags: MemberInfoFlags::new(145),
    module: "NvRvmDx11",
    data: TypeInfoData::Array("Dx11NvViewStateFTInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Dx11NvDrawStateDepthInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx11NvDrawStateDepthInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx11NvDrawStateDepthInstructionFactoryTrait for Dx11NvDrawStateDepthInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx11NvDrawStateDepthInstructionFactory {
}

pub static DX11NVDRAWSTATEDEPTHINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11NvDrawStateDepthInstructionFactory",
    name_hash: 2643064407,
    flags: MemberInfoFlags::new(101),
    module: "NvRvmDx11",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        super_class_offset: offset_of!(Dx11NvDrawStateDepthInstructionFactory, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx11NvDrawStateDepthInstructionFactory as Default>::default())),
            create_boxed: || Box::new(<Dx11NvDrawStateDepthInstructionFactory as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX11NVDRAWSTATEDEPTHINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11NvDrawStateDepthInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX11NVDRAWSTATEDEPTHINSTRUCTIONFACTORY_TYPE_INFO
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


pub static DX11NVDRAWSTATEDEPTHINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11NvDrawStateDepthInstructionFactory-Array",
    name_hash: 928894819,
    flags: MemberInfoFlags::new(145),
    module: "NvRvmDx11",
    data: TypeInfoData::Array("Dx11NvDrawStateDepthInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Dx11NvViewStateDepthInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx11NvViewStateDepthInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx11NvViewStateDepthInstructionFactoryTrait for Dx11NvViewStateDepthInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx11NvViewStateDepthInstructionFactory {
}

pub static DX11NVVIEWSTATEDEPTHINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11NvViewStateDepthInstructionFactory",
    name_hash: 73482138,
    flags: MemberInfoFlags::new(101),
    module: "NvRvmDx11",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        super_class_offset: offset_of!(Dx11NvViewStateDepthInstructionFactory, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx11NvViewStateDepthInstructionFactory as Default>::default())),
            create_boxed: || Box::new(<Dx11NvViewStateDepthInstructionFactory as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX11NVVIEWSTATEDEPTHINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11NvViewStateDepthInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX11NVVIEWSTATEDEPTHINSTRUCTIONFACTORY_TYPE_INFO
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


pub static DX11NVVIEWSTATEDEPTHINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11NvViewStateDepthInstructionFactory-Array",
    name_hash: 730849966,
    flags: MemberInfoFlags::new(145),
    module: "NvRvmDx11",
    data: TypeInfoData::Array("Dx11NvViewStateDepthInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsDx11NvViewStateDepthInstructionData {
}

pub trait RvmSerializedDbnsDx11NvViewStateDepthInstructionDataTrait: TypeObject {
}

impl RvmSerializedDbnsDx11NvViewStateDepthInstructionDataTrait for RvmSerializedDbnsDx11NvViewStateDepthInstructionData {
}

pub static RVMSERIALIZEDDB_NS_DX11NVVIEWSTATEDEPTHINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11NvViewStateDepthInstructionData",
    name_hash: 1520331774,
    flags: MemberInfoFlags::new(53321),
    module: "NvRvmDx11",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx11NvViewStateDepthInstructionData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDx11NvViewStateDepthInstructionData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX11NVVIEWSTATEDEPTHINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDbnsDx11NvViewStateDepthInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11NVVIEWSTATEDEPTHINSTRUCTIONDATA_TYPE_INFO
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
pub struct RvmSerializedDbnsDx11NvDrawStateInstructionData {
}

pub trait RvmSerializedDbnsDx11NvDrawStateInstructionDataTrait: TypeObject {
}

impl RvmSerializedDbnsDx11NvDrawStateInstructionDataTrait for RvmSerializedDbnsDx11NvDrawStateInstructionData {
}

pub static RVMSERIALIZEDDB_NS_DX11NVDRAWSTATEINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11NvDrawStateInstructionData",
    name_hash: 4275256734,
    flags: MemberInfoFlags::new(53321),
    module: "NvRvmDx11",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx11NvDrawStateInstructionData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDx11NvDrawStateInstructionData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX11NVDRAWSTATEINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDbnsDx11NvDrawStateInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11NVDRAWSTATEINSTRUCTIONDATA_TYPE_INFO
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
pub struct RvmSerializedDbnsDx11NvViewStateInstructionData {
}

pub trait RvmSerializedDbnsDx11NvViewStateInstructionDataTrait: TypeObject {
}

impl RvmSerializedDbnsDx11NvViewStateInstructionDataTrait for RvmSerializedDbnsDx11NvViewStateInstructionData {
}

pub static RVMSERIALIZEDDB_NS_DX11NVVIEWSTATEINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11NvViewStateInstructionData",
    name_hash: 1486634195,
    flags: MemberInfoFlags::new(53321),
    module: "NvRvmDx11",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx11NvViewStateInstructionData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDx11NvViewStateInstructionData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX11NVVIEWSTATEINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDbnsDx11NvViewStateInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11NVVIEWSTATEINSTRUCTIONDATA_TYPE_INFO
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
pub struct Dx11NvRvmBackendConfig {
    pub _glacier_base: super::rvm_dx11_pc::Dx11RvmBackendConfig,
}

pub trait Dx11NvRvmBackendConfigTrait: super::rvm_dx11_pc::Dx11RvmBackendConfigTrait {
}

impl Dx11NvRvmBackendConfigTrait for Dx11NvRvmBackendConfig {
}

impl super::rvm_dx11_pc::Dx11RvmBackendConfigTrait for Dx11NvRvmBackendConfig {
}

impl super::rvm_common::RvmBackendConfigTrait for Dx11NvRvmBackendConfig {
}

impl super::core::AssetTrait for Dx11NvRvmBackendConfig {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for Dx11NvRvmBackendConfig {
}

pub static DX11NVRVMBACKENDCONFIG_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11NvRvmBackendConfig",
    name_hash: 643630886,
    flags: MemberInfoFlags::new(101),
    module: "NvRvmDx11",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_dx11_pc::DX11RVMBACKENDCONFIG_TYPE_INFO),
        super_class_offset: offset_of!(Dx11NvRvmBackendConfig, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx11NvRvmBackendConfig as Default>::default())),
            create_boxed: || Box::new(<Dx11NvRvmBackendConfig as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX11NVRVMBACKENDCONFIG_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11NvRvmBackendConfig {
    fn type_info(&self) -> &'static TypeInfo {
        DX11NVRVMBACKENDCONFIG_TYPE_INFO
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


pub static DX11NVRVMBACKENDCONFIG_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11NvRvmBackendConfig-Array",
    name_hash: 3262935954,
    flags: MemberInfoFlags::new(145),
    module: "NvRvmDx11",
    data: TypeInfoData::Array("Dx11NvRvmBackendConfig"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Dx11NvRvmDatabaseLoader {
    pub _glacier_base: super::rvm_common::RvmCommonDatabaseLoader,
}

pub trait Dx11NvRvmDatabaseLoaderTrait: super::rvm_common::RvmCommonDatabaseLoaderTrait {
}

impl Dx11NvRvmDatabaseLoaderTrait for Dx11NvRvmDatabaseLoader {
}

impl super::rvm_common::RvmCommonDatabaseLoaderTrait for Dx11NvRvmDatabaseLoader {
}

impl super::render::RvmDatabaseLoaderTrait for Dx11NvRvmDatabaseLoader {
}

pub static DX11NVRVMDATABASELOADER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11NvRvmDatabaseLoader",
    name_hash: 3860666236,
    flags: MemberInfoFlags::new(101),
    module: "NvRvmDx11",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMCOMMONDATABASELOADER_TYPE_INFO),
        super_class_offset: offset_of!(Dx11NvRvmDatabaseLoader, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx11NvRvmDatabaseLoader as Default>::default())),
            create_boxed: || Box::new(<Dx11NvRvmDatabaseLoader as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX11NVRVMDATABASELOADER_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for Dx11NvRvmDatabaseLoader {
    fn type_info(&self) -> &'static TypeInfo {
        DX11NVRVMDATABASELOADER_TYPE_INFO
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


pub static DX11NVRVMDATABASELOADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11NvRvmDatabaseLoader-Array",
    name_hash: 2746048328,
    flags: MemberInfoFlags::new(145),
    module: "NvRvmDx11",
    data: TypeInfoData::Array("Dx11NvRvmDatabaseLoader"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Dx11NvRvmDatabase {
    pub _glacier_base: super::rvm_dx11_pc::Dx11RvmDatabase,
}

pub trait Dx11NvRvmDatabaseTrait: super::rvm_dx11_pc::Dx11RvmDatabaseTrait {
}

impl Dx11NvRvmDatabaseTrait for Dx11NvRvmDatabase {
}

impl super::rvm_dx11_pc::Dx11RvmDatabaseTrait for Dx11NvRvmDatabase {
}

impl super::rvm_common::BaseRvmDatabaseTrait for Dx11NvRvmDatabase {
}

impl super::render::RvmLegacyDatabaseTrait for Dx11NvRvmDatabase {
}

impl super::render::RvmDatabaseTrait for Dx11NvRvmDatabase {
}

impl super::core::IResourceObjectTrait for Dx11NvRvmDatabase {
}

pub static DX11NVRVMDATABASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11NvRvmDatabase",
    name_hash: 2700385677,
    flags: MemberInfoFlags::new(101),
    module: "NvRvmDx11",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_dx11_pc::DX11RVMDATABASE_TYPE_INFO),
        super_class_offset: offset_of!(Dx11NvRvmDatabase, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx11NvRvmDatabase as Default>::default())),
            create_boxed: || Box::new(<Dx11NvRvmDatabase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX11NVRVMDATABASE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11NvRvmDatabase {
    fn type_info(&self) -> &'static TypeInfo {
        DX11NVRVMDATABASE_TYPE_INFO
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


pub static DX11NVRVMDATABASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11NvRvmDatabase-Array",
    name_hash: 8208697,
    flags: MemberInfoFlags::new(145),
    module: "NvRvmDx11",
    data: TypeInfoData::Array("Dx11NvRvmDatabase"),
    array_type: None,
    alignment: 8,
};


