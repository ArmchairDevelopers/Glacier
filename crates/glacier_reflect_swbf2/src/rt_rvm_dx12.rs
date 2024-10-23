use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_rt_rvm_dx12_types(registry: &mut TypeRegistry) {
    registry.register_type(RVMDX12RTSETTINGS_TYPE_INFO);
    registry.register_type(RVMDX12RTSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(DX12RTTLASNULLINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX12RTTLASNULLINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DX12RTTLASDYNAMICINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX12RTTLASDYNAMICINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DX12RTTLASSTATICINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX12RTTLASSTATICINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DX12RTSHADERTABLERECORDWRITERINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX12RTSHADERTABLERECORDWRITERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DX12RTNULLHITSHADERINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX12RTNULLHITSHADERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DX12RTHITCOLLECTIONINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX12RTHITCOLLECTIONINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DX12RTDYNAMICBLASBUILDINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX12RTDYNAMICBLASBUILDINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DX12RTSTATICBLASBUILDINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX12RTSTATICBLASBUILDINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DX12RTHITSHADERCONSTANTINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX12RTHITSHADERCONSTANTINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DX12RTINDEXBUFFERINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX12RTINDEXBUFFERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DX12RTSORTINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX12RTSORTINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DX12RTDISPATCHINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX12RTDISPATCHINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12RTTLASNULLDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12RTTLASDYNAMICDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12RTTLASSTATICDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12RTTLASBASEDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12RTSHADERTABLERECORDWRITERDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12RTNULLHITSHADERDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12RTHITCOLLECTIONDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12RTSTATICBLASBUILDDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12RTDYNAMICBLASBUILDDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12RTBLASBASEDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12RTHITSHADERCONSTANTDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12RTINDEXBUFFERDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12RTSORTDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12RTDISPATCHDATA_TYPE_INFO);
    registry.register_type(DX12RTRVMBACKENDCONFIG_TYPE_INFO);
    registry.register_type(DX12RTRVMBACKENDCONFIG_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12RTCOLLECTIONPRELOADOP_TYPE_INFO);
    registry.register_type(DX12RTRVMDATABASELOADER_TYPE_INFO);
    registry.register_type(DX12RTRVMDATABASELOADER_ARRAY_TYPE_INFO);
    registry.register_type(DX12RTRVMDATABASE_TYPE_INFO);
    registry.register_type(DX12RTRVMDATABASE_ARRAY_TYPE_INFO);
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmDx12RtSettings {
    pub _glacier_base: super::rvm_common::RvmCommonSettings,
    pub enabled: bool,
}

pub trait RvmDx12RtSettingsTrait: super::rvm_common::RvmCommonSettingsTrait {
    fn enabled(&self) -> &bool;
    fn enabled_mut(&mut self) -> &mut bool;
}

impl RvmDx12RtSettingsTrait for RvmDx12RtSettings {
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
}

impl super::rvm_common::RvmCommonSettingsTrait for RvmDx12RtSettings {
    fn on_demand_building_enable(&self) -> &bool {
        self._glacier_base.on_demand_building_enable()
    }
    fn on_demand_building_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.on_demand_building_enable_mut()
    }
}

impl super::core::DataContainerTrait for RvmDx12RtSettings {
}

pub static RVMDX12RTSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmDx12RtSettings",
    name_hash: 1723617104,
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMCOMMONSETTINGS_TYPE_INFO),
        super_class_offset: offset_of!(RvmDx12RtSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmDx12RtSettings as Default>::default())),
            create_boxed: || Box::new(<RvmDx12RtSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Enabled",
                name_hash: 2662400,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RvmDx12RtSettings, enabled),
            },
        ],
    }),
    array_type: Some(RVMDX12RTSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmDx12RtSettings {
    fn type_info(&self) -> &'static TypeInfo {
        RVMDX12RTSETTINGS_TYPE_INFO
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


pub static RVMDX12RTSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmDx12RtSettings-Array",
    name_hash: 939535844,
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("RvmDx12RtSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Dx12RtTlasNullInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx12RtTlasNullInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx12RtTlasNullInstructionFactoryTrait for Dx12RtTlasNullInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx12RtTlasNullInstructionFactory {
}

pub static DX12RTTLASNULLINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtTlasNullInstructionFactory",
    name_hash: 1375951073,
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        super_class_offset: offset_of!(Dx12RtTlasNullInstructionFactory, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RtTlasNullInstructionFactory as Default>::default())),
            create_boxed: || Box::new(<Dx12RtTlasNullInstructionFactory as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12RTTLASNULLINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12RtTlasNullInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX12RTTLASNULLINSTRUCTIONFACTORY_TYPE_INFO
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


pub static DX12RTTLASNULLINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtTlasNullInstructionFactory-Array",
    name_hash: 2654499797,
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtTlasNullInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Dx12RtTlasDynamicInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx12RtTlasDynamicInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx12RtTlasDynamicInstructionFactoryTrait for Dx12RtTlasDynamicInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx12RtTlasDynamicInstructionFactory {
}

pub static DX12RTTLASDYNAMICINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtTlasDynamicInstructionFactory",
    name_hash: 1263905263,
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        super_class_offset: offset_of!(Dx12RtTlasDynamicInstructionFactory, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RtTlasDynamicInstructionFactory as Default>::default())),
            create_boxed: || Box::new(<Dx12RtTlasDynamicInstructionFactory as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12RTTLASDYNAMICINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12RtTlasDynamicInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX12RTTLASDYNAMICINSTRUCTIONFACTORY_TYPE_INFO
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


pub static DX12RTTLASDYNAMICINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtTlasDynamicInstructionFactory-Array",
    name_hash: 4155426779,
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtTlasDynamicInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Dx12RtTlasStaticInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx12RtTlasStaticInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx12RtTlasStaticInstructionFactoryTrait for Dx12RtTlasStaticInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx12RtTlasStaticInstructionFactory {
}

pub static DX12RTTLASSTATICINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtTlasStaticInstructionFactory",
    name_hash: 4163746882,
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        super_class_offset: offset_of!(Dx12RtTlasStaticInstructionFactory, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RtTlasStaticInstructionFactory as Default>::default())),
            create_boxed: || Box::new(<Dx12RtTlasStaticInstructionFactory as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12RTTLASSTATICINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12RtTlasStaticInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX12RTTLASSTATICINSTRUCTIONFACTORY_TYPE_INFO
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


pub static DX12RTTLASSTATICINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtTlasStaticInstructionFactory-Array",
    name_hash: 2964655094,
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtTlasStaticInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Dx12RtShaderTableRecordWriterInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx12RtShaderTableRecordWriterInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx12RtShaderTableRecordWriterInstructionFactoryTrait for Dx12RtShaderTableRecordWriterInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx12RtShaderTableRecordWriterInstructionFactory {
}

pub static DX12RTSHADERTABLERECORDWRITERINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtShaderTableRecordWriterInstructionFactory",
    name_hash: 1624653125,
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        super_class_offset: offset_of!(Dx12RtShaderTableRecordWriterInstructionFactory, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RtShaderTableRecordWriterInstructionFactory as Default>::default())),
            create_boxed: || Box::new(<Dx12RtShaderTableRecordWriterInstructionFactory as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12RTSHADERTABLERECORDWRITERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12RtShaderTableRecordWriterInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX12RTSHADERTABLERECORDWRITERINSTRUCTIONFACTORY_TYPE_INFO
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


pub static DX12RTSHADERTABLERECORDWRITERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtShaderTableRecordWriterInstructionFactory-Array",
    name_hash: 1094997361,
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtShaderTableRecordWriterInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Dx12RtNullHitShaderInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx12RtNullHitShaderInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx12RtNullHitShaderInstructionFactoryTrait for Dx12RtNullHitShaderInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx12RtNullHitShaderInstructionFactory {
}

pub static DX12RTNULLHITSHADERINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtNullHitShaderInstructionFactory",
    name_hash: 1209752375,
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        super_class_offset: offset_of!(Dx12RtNullHitShaderInstructionFactory, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RtNullHitShaderInstructionFactory as Default>::default())),
            create_boxed: || Box::new(<Dx12RtNullHitShaderInstructionFactory as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12RTNULLHITSHADERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12RtNullHitShaderInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX12RTNULLHITSHADERINSTRUCTIONFACTORY_TYPE_INFO
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


pub static DX12RTNULLHITSHADERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtNullHitShaderInstructionFactory-Array",
    name_hash: 2064897155,
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtNullHitShaderInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Dx12RtHitCollectionInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx12RtHitCollectionInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx12RtHitCollectionInstructionFactoryTrait for Dx12RtHitCollectionInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx12RtHitCollectionInstructionFactory {
}

pub static DX12RTHITCOLLECTIONINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtHitCollectionInstructionFactory",
    name_hash: 123814963,
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        super_class_offset: offset_of!(Dx12RtHitCollectionInstructionFactory, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RtHitCollectionInstructionFactory as Default>::default())),
            create_boxed: || Box::new(<Dx12RtHitCollectionInstructionFactory as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12RTHITCOLLECTIONINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12RtHitCollectionInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX12RTHITCOLLECTIONINSTRUCTIONFACTORY_TYPE_INFO
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


pub static DX12RTHITCOLLECTIONINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtHitCollectionInstructionFactory-Array",
    name_hash: 2049232775,
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtHitCollectionInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Dx12RtDynamicBlasBuildInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx12RtDynamicBlasBuildInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx12RtDynamicBlasBuildInstructionFactoryTrait for Dx12RtDynamicBlasBuildInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx12RtDynamicBlasBuildInstructionFactory {
}

pub static DX12RTDYNAMICBLASBUILDINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtDynamicBlasBuildInstructionFactory",
    name_hash: 1801547823,
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        super_class_offset: offset_of!(Dx12RtDynamicBlasBuildInstructionFactory, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RtDynamicBlasBuildInstructionFactory as Default>::default())),
            create_boxed: || Box::new(<Dx12RtDynamicBlasBuildInstructionFactory as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12RTDYNAMICBLASBUILDINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12RtDynamicBlasBuildInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX12RTDYNAMICBLASBUILDINSTRUCTIONFACTORY_TYPE_INFO
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


pub static DX12RTDYNAMICBLASBUILDINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtDynamicBlasBuildInstructionFactory-Array",
    name_hash: 1958886811,
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtDynamicBlasBuildInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Dx12RtStaticBlasBuildInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx12RtStaticBlasBuildInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx12RtStaticBlasBuildInstructionFactoryTrait for Dx12RtStaticBlasBuildInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx12RtStaticBlasBuildInstructionFactory {
}

pub static DX12RTSTATICBLASBUILDINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtStaticBlasBuildInstructionFactory",
    name_hash: 865047714,
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        super_class_offset: offset_of!(Dx12RtStaticBlasBuildInstructionFactory, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RtStaticBlasBuildInstructionFactory as Default>::default())),
            create_boxed: || Box::new(<Dx12RtStaticBlasBuildInstructionFactory as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12RTSTATICBLASBUILDINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12RtStaticBlasBuildInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX12RTSTATICBLASBUILDINSTRUCTIONFACTORY_TYPE_INFO
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


pub static DX12RTSTATICBLASBUILDINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtStaticBlasBuildInstructionFactory-Array",
    name_hash: 2625860374,
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtStaticBlasBuildInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Dx12RtHitShaderConstantInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx12RtHitShaderConstantInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx12RtHitShaderConstantInstructionFactoryTrait for Dx12RtHitShaderConstantInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx12RtHitShaderConstantInstructionFactory {
}

pub static DX12RTHITSHADERCONSTANTINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtHitShaderConstantInstructionFactory",
    name_hash: 4272375858,
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        super_class_offset: offset_of!(Dx12RtHitShaderConstantInstructionFactory, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RtHitShaderConstantInstructionFactory as Default>::default())),
            create_boxed: || Box::new(<Dx12RtHitShaderConstantInstructionFactory as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12RTHITSHADERCONSTANTINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12RtHitShaderConstantInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX12RTHITSHADERCONSTANTINSTRUCTIONFACTORY_TYPE_INFO
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


pub static DX12RTHITSHADERCONSTANTINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtHitShaderConstantInstructionFactory-Array",
    name_hash: 1644596870,
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtHitShaderConstantInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Dx12RtIndexBufferInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx12RtIndexBufferInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx12RtIndexBufferInstructionFactoryTrait for Dx12RtIndexBufferInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx12RtIndexBufferInstructionFactory {
}

pub static DX12RTINDEXBUFFERINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtIndexBufferInstructionFactory",
    name_hash: 2730903918,
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        super_class_offset: offset_of!(Dx12RtIndexBufferInstructionFactory, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RtIndexBufferInstructionFactory as Default>::default())),
            create_boxed: || Box::new(<Dx12RtIndexBufferInstructionFactory as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12RTINDEXBUFFERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12RtIndexBufferInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX12RTINDEXBUFFERINSTRUCTIONFACTORY_TYPE_INFO
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


pub static DX12RTINDEXBUFFERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtIndexBufferInstructionFactory-Array",
    name_hash: 621642330,
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtIndexBufferInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Dx12RtSortInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx12RtSortInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx12RtSortInstructionFactoryTrait for Dx12RtSortInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx12RtSortInstructionFactory {
}

pub static DX12RTSORTINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtSortInstructionFactory",
    name_hash: 564348234,
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        super_class_offset: offset_of!(Dx12RtSortInstructionFactory, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RtSortInstructionFactory as Default>::default())),
            create_boxed: || Box::new(<Dx12RtSortInstructionFactory as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12RTSORTINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12RtSortInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX12RTSORTINSTRUCTIONFACTORY_TYPE_INFO
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


pub static DX12RTSORTINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtSortInstructionFactory-Array",
    name_hash: 2330767614,
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtSortInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Dx12RtDispatchInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx12RtDispatchInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx12RtDispatchInstructionFactoryTrait for Dx12RtDispatchInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx12RtDispatchInstructionFactory {
}

pub static DX12RTDISPATCHINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtDispatchInstructionFactory",
    name_hash: 372110848,
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        super_class_offset: offset_of!(Dx12RtDispatchInstructionFactory, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RtDispatchInstructionFactory as Default>::default())),
            create_boxed: || Box::new(<Dx12RtDispatchInstructionFactory as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12RTDISPATCHINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12RtDispatchInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX12RTDISPATCHINSTRUCTIONFACTORY_TYPE_INFO
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


pub static DX12RTDISPATCHINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtDispatchInstructionFactory-Array",
    name_hash: 518855732,
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtDispatchInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsDx12RtTlasNullData {
}

pub trait RvmSerializedDbnsDx12RtTlasNullDataTrait: TypeObject {
}

impl RvmSerializedDbnsDx12RtTlasNullDataTrait for RvmSerializedDbnsDx12RtTlasNullData {
}

pub static RVMSERIALIZEDDB_NS_DX12RTTLASNULLDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtTlasNullData",
    name_hash: 2220003485,
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx12RtTlasNullData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDx12RtTlasNullData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12RTTLASNULLDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDbnsDx12RtTlasNullData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTTLASNULLDATA_TYPE_INFO
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
pub struct RvmSerializedDbnsDx12RtTlasDynamicData {
}

pub trait RvmSerializedDbnsDx12RtTlasDynamicDataTrait: TypeObject {
}

impl RvmSerializedDbnsDx12RtTlasDynamicDataTrait for RvmSerializedDbnsDx12RtTlasDynamicData {
}

pub static RVMSERIALIZEDDB_NS_DX12RTTLASDYNAMICDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtTlasDynamicData",
    name_hash: 2534554323,
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx12RtTlasDynamicData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDx12RtTlasDynamicData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12RTTLASDYNAMICDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDbnsDx12RtTlasDynamicData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTTLASDYNAMICDATA_TYPE_INFO
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
pub struct RvmSerializedDbnsDx12RtTlasStaticData {
}

pub trait RvmSerializedDbnsDx12RtTlasStaticDataTrait: TypeObject {
}

impl RvmSerializedDbnsDx12RtTlasStaticDataTrait for RvmSerializedDbnsDx12RtTlasStaticData {
}

pub static RVMSERIALIZEDDB_NS_DX12RTTLASSTATICDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtTlasStaticData",
    name_hash: 1588572030,
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx12RtTlasStaticData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDx12RtTlasStaticData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12RTTLASSTATICDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDbnsDx12RtTlasStaticData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTTLASSTATICDATA_TYPE_INFO
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
pub struct RvmSerializedDbnsDx12RtTlasBaseData {
}

pub trait RvmSerializedDbnsDx12RtTlasBaseDataTrait: TypeObject {
}

impl RvmSerializedDbnsDx12RtTlasBaseDataTrait for RvmSerializedDbnsDx12RtTlasBaseData {
}

pub static RVMSERIALIZEDDB_NS_DX12RTTLASBASEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtTlasBaseData",
    name_hash: 1651926579,
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx12RtTlasBaseData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDx12RtTlasBaseData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12RTTLASBASEDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDbnsDx12RtTlasBaseData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTTLASBASEDATA_TYPE_INFO
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
pub struct RvmSerializedDbnsDx12RtShaderTableRecordWriterData {
}

pub trait RvmSerializedDbnsDx12RtShaderTableRecordWriterDataTrait: TypeObject {
}

impl RvmSerializedDbnsDx12RtShaderTableRecordWriterDataTrait for RvmSerializedDbnsDx12RtShaderTableRecordWriterData {
}

pub static RVMSERIALIZEDDB_NS_DX12RTSHADERTABLERECORDWRITERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtShaderTableRecordWriterData",
    name_hash: 1832392313,
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx12RtShaderTableRecordWriterData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDx12RtShaderTableRecordWriterData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12RTSHADERTABLERECORDWRITERDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDbnsDx12RtShaderTableRecordWriterData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTSHADERTABLERECORDWRITERDATA_TYPE_INFO
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
pub struct RvmSerializedDbnsDx12RtNullHitShaderData {
}

pub trait RvmSerializedDbnsDx12RtNullHitShaderDataTrait: TypeObject {
}

impl RvmSerializedDbnsDx12RtNullHitShaderDataTrait for RvmSerializedDbnsDx12RtNullHitShaderData {
}

pub static RVMSERIALIZEDDB_NS_DX12RTNULLHITSHADERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtNullHitShaderData",
    name_hash: 1020845771,
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx12RtNullHitShaderData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDx12RtNullHitShaderData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12RTNULLHITSHADERDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDbnsDx12RtNullHitShaderData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTNULLHITSHADERDATA_TYPE_INFO
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
pub struct RvmSerializedDbnsDx12RtHitCollectionData {
}

pub trait RvmSerializedDbnsDx12RtHitCollectionDataTrait: TypeObject {
}

impl RvmSerializedDbnsDx12RtHitCollectionDataTrait for RvmSerializedDbnsDx12RtHitCollectionData {
}

pub static RVMSERIALIZEDDB_NS_DX12RTHITCOLLECTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtHitCollectionData",
    name_hash: 1409427087,
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx12RtHitCollectionData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDx12RtHitCollectionData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12RTHITCOLLECTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsDx12RtHitCollectionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTHITCOLLECTIONDATA_TYPE_INFO
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
pub struct RvmSerializedDbnsDx12RtStaticBlasBuildData {
}

pub trait RvmSerializedDbnsDx12RtStaticBlasBuildDataTrait: TypeObject {
}

impl RvmSerializedDbnsDx12RtStaticBlasBuildDataTrait for RvmSerializedDbnsDx12RtStaticBlasBuildData {
}

pub static RVMSERIALIZEDDB_NS_DX12RTSTATICBLASBUILDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtStaticBlasBuildData",
    name_hash: 3918625566,
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx12RtStaticBlasBuildData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDx12RtStaticBlasBuildData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12RTSTATICBLASBUILDDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDbnsDx12RtStaticBlasBuildData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTSTATICBLASBUILDDATA_TYPE_INFO
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
pub struct RvmSerializedDbnsDx12RtDynamicBlasBuildData {
}

pub trait RvmSerializedDbnsDx12RtDynamicBlasBuildDataTrait: TypeObject {
}

impl RvmSerializedDbnsDx12RtDynamicBlasBuildDataTrait for RvmSerializedDbnsDx12RtDynamicBlasBuildData {
}

pub static RVMSERIALIZEDDB_NS_DX12RTDYNAMICBLASBUILDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtDynamicBlasBuildData",
    name_hash: 2022263187,
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx12RtDynamicBlasBuildData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDx12RtDynamicBlasBuildData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12RTDYNAMICBLASBUILDDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDbnsDx12RtDynamicBlasBuildData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTDYNAMICBLASBUILDDATA_TYPE_INFO
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
pub struct RvmSerializedDbnsDx12RtBlasBaseData {
}

pub trait RvmSerializedDbnsDx12RtBlasBaseDataTrait: TypeObject {
}

impl RvmSerializedDbnsDx12RtBlasBaseDataTrait for RvmSerializedDbnsDx12RtBlasBaseData {
}

pub static RVMSERIALIZEDDB_NS_DX12RTBLASBASEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtBlasBaseData",
    name_hash: 1682547301,
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx12RtBlasBaseData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDx12RtBlasBaseData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12RTBLASBASEDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDbnsDx12RtBlasBaseData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTBLASBASEDATA_TYPE_INFO
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
pub struct RvmSerializedDbnsDx12RtHitShaderConstantData {
}

pub trait RvmSerializedDbnsDx12RtHitShaderConstantDataTrait: TypeObject {
}

impl RvmSerializedDbnsDx12RtHitShaderConstantDataTrait for RvmSerializedDbnsDx12RtHitShaderConstantData {
}

pub static RVMSERIALIZEDDB_NS_DX12RTHITSHADERCONSTANTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtHitShaderConstantData",
    name_hash: 6550542,
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx12RtHitShaderConstantData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDx12RtHitShaderConstantData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12RTHITSHADERCONSTANTDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDbnsDx12RtHitShaderConstantData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTHITSHADERCONSTANTDATA_TYPE_INFO
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
pub struct RvmSerializedDbnsDx12RtIndexBufferData {
}

pub trait RvmSerializedDbnsDx12RtIndexBufferDataTrait: TypeObject {
}

impl RvmSerializedDbnsDx12RtIndexBufferDataTrait for RvmSerializedDbnsDx12RtIndexBufferData {
}

pub static RVMSERIALIZEDDB_NS_DX12RTINDEXBUFFERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtIndexBufferData",
    name_hash: 1817985810,
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx12RtIndexBufferData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDx12RtIndexBufferData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12RTINDEXBUFFERDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDbnsDx12RtIndexBufferData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTINDEXBUFFERDATA_TYPE_INFO
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
pub struct RvmSerializedDbnsDx12RtSortData {
}

pub trait RvmSerializedDbnsDx12RtSortDataTrait: TypeObject {
}

impl RvmSerializedDbnsDx12RtSortDataTrait for RvmSerializedDbnsDx12RtSortData {
}

pub static RVMSERIALIZEDDB_NS_DX12RTSORTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtSortData",
    name_hash: 3630776118,
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx12RtSortData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDx12RtSortData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12RTSORTDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDbnsDx12RtSortData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTSORTDATA_TYPE_INFO
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
pub struct RvmSerializedDbnsDx12RtDispatchData {
}

pub trait RvmSerializedDbnsDx12RtDispatchDataTrait: TypeObject {
}

impl RvmSerializedDbnsDx12RtDispatchDataTrait for RvmSerializedDbnsDx12RtDispatchData {
}

pub static RVMSERIALIZEDDB_NS_DX12RTDISPATCHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtDispatchData",
    name_hash: 2147461756,
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx12RtDispatchData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDx12RtDispatchData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12RTDISPATCHDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RvmSerializedDbnsDx12RtDispatchData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTDISPATCHDATA_TYPE_INFO
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
pub struct Dx12RtRvmBackendConfig {
    pub _glacier_base: super::rvm_dx12_pc::Dx12PcRvmBackendConfig,
    pub enable_all_shader_graphs: bool,
    pub enabled_shader_graphs: Vec<Option<LockedTypeObject /* super::render::ShaderGraph */>>,
}

pub trait Dx12RtRvmBackendConfigTrait: super::rvm_dx12_pc::Dx12PcRvmBackendConfigTrait {
    fn enable_all_shader_graphs(&self) -> &bool;
    fn enable_all_shader_graphs_mut(&mut self) -> &mut bool;
    fn enabled_shader_graphs(&self) -> &Vec<Option<LockedTypeObject /* super::render::ShaderGraph */>>;
    fn enabled_shader_graphs_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::render::ShaderGraph */>>;
}

impl Dx12RtRvmBackendConfigTrait for Dx12RtRvmBackendConfig {
    fn enable_all_shader_graphs(&self) -> &bool {
        &self.enable_all_shader_graphs
    }
    fn enable_all_shader_graphs_mut(&mut self) -> &mut bool {
        &mut self.enable_all_shader_graphs
    }
    fn enabled_shader_graphs(&self) -> &Vec<Option<LockedTypeObject /* super::render::ShaderGraph */>> {
        &self.enabled_shader_graphs
    }
    fn enabled_shader_graphs_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::render::ShaderGraph */>> {
        &mut self.enabled_shader_graphs
    }
}

impl super::rvm_dx12_pc::Dx12PcRvmBackendConfigTrait for Dx12RtRvmBackendConfig {
}

impl super::rvm_dx12::Dx12RvmBackendConfigTrait for Dx12RtRvmBackendConfig {
}

impl super::rvm_common::RvmBackendConfigTrait for Dx12RtRvmBackendConfig {
}

impl super::core::AssetTrait for Dx12RtRvmBackendConfig {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for Dx12RtRvmBackendConfig {
}

pub static DX12RTRVMBACKENDCONFIG_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtRvmBackendConfig",
    name_hash: 4141283515,
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_dx12_pc::DX12PCRVMBACKENDCONFIG_TYPE_INFO),
        super_class_offset: offset_of!(Dx12RtRvmBackendConfig, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RtRvmBackendConfig as Default>::default())),
            create_boxed: || Box::new(<Dx12RtRvmBackendConfig as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "EnableAllShaderGraphs",
                name_hash: 4137414739,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(Dx12RtRvmBackendConfig, enable_all_shader_graphs),
            },
            FieldInfoData {
                name: "EnabledShaderGraphs",
                name_hash: 4283349622,
                flags: MemberInfoFlags::new(144),
                field_type: "ShaderGraph-Array",
                rust_offset: offset_of!(Dx12RtRvmBackendConfig, enabled_shader_graphs),
            },
        ],
    }),
    array_type: Some(DX12RTRVMBACKENDCONFIG_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12RtRvmBackendConfig {
    fn type_info(&self) -> &'static TypeInfo {
        DX12RTRVMBACKENDCONFIG_TYPE_INFO
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


pub static DX12RTRVMBACKENDCONFIG_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtRvmBackendConfig-Array",
    name_hash: 531578895,
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtRvmBackendConfig"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsDx12RtCollectionPreloadOp {
}

pub trait RvmSerializedDbnsDx12RtCollectionPreloadOpTrait: TypeObject {
}

impl RvmSerializedDbnsDx12RtCollectionPreloadOpTrait for RvmSerializedDbnsDx12RtCollectionPreloadOp {
}

pub static RVMSERIALIZEDDB_NS_DX12RTCOLLECTIONPRELOADOP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtCollectionPreloadOp",
    name_hash: 1375535668,
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx12RtCollectionPreloadOp as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDx12RtCollectionPreloadOp as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12RTCOLLECTIONPRELOADOP_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsDx12RtCollectionPreloadOp {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTCOLLECTIONPRELOADOP_TYPE_INFO
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
pub struct Dx12RtRvmDatabaseLoader {
    pub _glacier_base: super::rvm_common::RvmCommonDatabaseLoader,
}

pub trait Dx12RtRvmDatabaseLoaderTrait: super::rvm_common::RvmCommonDatabaseLoaderTrait {
}

impl Dx12RtRvmDatabaseLoaderTrait for Dx12RtRvmDatabaseLoader {
}

impl super::rvm_common::RvmCommonDatabaseLoaderTrait for Dx12RtRvmDatabaseLoader {
}

impl super::render::RvmDatabaseLoaderTrait for Dx12RtRvmDatabaseLoader {
}

pub static DX12RTRVMDATABASELOADER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtRvmDatabaseLoader",
    name_hash: 2246500289,
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMCOMMONDATABASELOADER_TYPE_INFO),
        super_class_offset: offset_of!(Dx12RtRvmDatabaseLoader, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RtRvmDatabaseLoader as Default>::default())),
            create_boxed: || Box::new(<Dx12RtRvmDatabaseLoader as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12RTRVMDATABASELOADER_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for Dx12RtRvmDatabaseLoader {
    fn type_info(&self) -> &'static TypeInfo {
        DX12RTRVMDATABASELOADER_TYPE_INFO
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


pub static DX12RTRVMDATABASELOADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtRvmDatabaseLoader-Array",
    name_hash: 2829396469,
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtRvmDatabaseLoader"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Dx12RtRvmDatabase {
    pub _glacier_base: super::rvm_dx12_pc::Dx12PcRvmDatabase,
}

pub trait Dx12RtRvmDatabaseTrait: super::rvm_dx12_pc::Dx12PcRvmDatabaseTrait {
}

impl Dx12RtRvmDatabaseTrait for Dx12RtRvmDatabase {
}

impl super::rvm_dx12_pc::Dx12PcRvmDatabaseTrait for Dx12RtRvmDatabase {
}

impl super::rvm_dx12::Dx12RvmDatabaseTrait for Dx12RtRvmDatabase {
}

impl super::rvm_common::BaseRvmDatabaseTrait for Dx12RtRvmDatabase {
}

impl super::render::RvmLegacyDatabaseTrait for Dx12RtRvmDatabase {
}

impl super::render::RvmDatabaseTrait for Dx12RtRvmDatabase {
}

impl super::core::IResourceObjectTrait for Dx12RtRvmDatabase {
}

pub static DX12RTRVMDATABASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtRvmDatabase",
    name_hash: 4127919088,
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_dx12_pc::DX12PCRVMDATABASE_TYPE_INFO),
        super_class_offset: offset_of!(Dx12RtRvmDatabase, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RtRvmDatabase as Default>::default())),
            create_boxed: || Box::new(<Dx12RtRvmDatabase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12RTRVMDATABASE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12RtRvmDatabase {
    fn type_info(&self) -> &'static TypeInfo {
        DX12RTRVMDATABASE_TYPE_INFO
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


pub static DX12RTRVMDATABASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtRvmDatabase-Array",
    name_hash: 3433146820,
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtRvmDatabase"),
    array_type: None,
    alignment: 8,
};


