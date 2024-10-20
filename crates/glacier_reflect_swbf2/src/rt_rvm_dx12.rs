use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct RvmDx12RtSettings {
    pub _glacier_base: super::rvm_common::RvmCommonSettings,
    pub enabled: bool,
}

pub trait RvmDx12RtSettingsTrait: super::rvm_common::RvmCommonSettingsTrait {
    fn enabled(&self) -> &bool;
}

impl RvmDx12RtSettingsTrait for RvmDx12RtSettings {
    fn enabled(&self) -> &bool {
        &self.enabled
    }
}

impl super::rvm_common::RvmCommonSettingsTrait for RvmDx12RtSettings {
    fn on_demand_building_enable(&self) -> &bool {
        self._glacier_base.on_demand_building_enable()
    }
}

impl super::core::DataContainerTrait for RvmDx12RtSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static RVMDX12RTSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmDx12RtSettings",
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMCOMMONSETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmDx12RtSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enabled",
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
}


pub static RVMDX12RTSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmDx12RtSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("RvmDx12RtSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RtTlasNullInstructionFactory as Default>::default())),
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
}


pub static DX12RTTLASNULLINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtTlasNullInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtTlasNullInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RtTlasDynamicInstructionFactory as Default>::default())),
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
}


pub static DX12RTTLASDYNAMICINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtTlasDynamicInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtTlasDynamicInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RtTlasStaticInstructionFactory as Default>::default())),
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
}


pub static DX12RTTLASSTATICINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtTlasStaticInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtTlasStaticInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RtShaderTableRecordWriterInstructionFactory as Default>::default())),
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
}


pub static DX12RTSHADERTABLERECORDWRITERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtShaderTableRecordWriterInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtShaderTableRecordWriterInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RtNullHitShaderInstructionFactory as Default>::default())),
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
}


pub static DX12RTNULLHITSHADERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtNullHitShaderInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtNullHitShaderInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RtHitCollectionInstructionFactory as Default>::default())),
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
}


pub static DX12RTHITCOLLECTIONINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtHitCollectionInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtHitCollectionInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RtDynamicBlasBuildInstructionFactory as Default>::default())),
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
}


pub static DX12RTDYNAMICBLASBUILDINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtDynamicBlasBuildInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtDynamicBlasBuildInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RtStaticBlasBuildInstructionFactory as Default>::default())),
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
}


pub static DX12RTSTATICBLASBUILDINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtStaticBlasBuildInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtStaticBlasBuildInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RtHitShaderConstantInstructionFactory as Default>::default())),
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
}


pub static DX12RTHITSHADERCONSTANTINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtHitShaderConstantInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtHitShaderConstantInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RtIndexBufferInstructionFactory as Default>::default())),
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
}


pub static DX12RTINDEXBUFFERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtIndexBufferInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtIndexBufferInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RtSortInstructionFactory as Default>::default())),
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
}


pub static DX12RTSORTINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtSortInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtSortInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RtDispatchInstructionFactory as Default>::default())),
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
}


pub static DX12RTDISPATCHINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtDispatchInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtDispatchInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12RtTlasNullData {
}

pub trait RvmSerializedDb_ns_Dx12RtTlasNullDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12RtTlasNullDataTrait for RvmSerializedDb_ns_Dx12RtTlasNullData {
}

pub static RVMSERIALIZEDDB_NS_DX12RTTLASNULLDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtTlasNullData",
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12RtTlasNullData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12RTTLASNULLDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12RtTlasNullData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTTLASNULLDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12RtTlasDynamicData {
}

pub trait RvmSerializedDb_ns_Dx12RtTlasDynamicDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12RtTlasDynamicDataTrait for RvmSerializedDb_ns_Dx12RtTlasDynamicData {
}

pub static RVMSERIALIZEDDB_NS_DX12RTTLASDYNAMICDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtTlasDynamicData",
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12RtTlasDynamicData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12RTTLASDYNAMICDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12RtTlasDynamicData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTTLASDYNAMICDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12RtTlasStaticData {
}

pub trait RvmSerializedDb_ns_Dx12RtTlasStaticDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12RtTlasStaticDataTrait for RvmSerializedDb_ns_Dx12RtTlasStaticData {
}

pub static RVMSERIALIZEDDB_NS_DX12RTTLASSTATICDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtTlasStaticData",
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12RtTlasStaticData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12RTTLASSTATICDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12RtTlasStaticData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTTLASSTATICDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12RtTlasBaseData {
}

pub trait RvmSerializedDb_ns_Dx12RtTlasBaseDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12RtTlasBaseDataTrait for RvmSerializedDb_ns_Dx12RtTlasBaseData {
}

pub static RVMSERIALIZEDDB_NS_DX12RTTLASBASEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtTlasBaseData",
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12RtTlasBaseData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12RTTLASBASEDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12RtTlasBaseData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTTLASBASEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12RtShaderTableRecordWriterData {
}

pub trait RvmSerializedDb_ns_Dx12RtShaderTableRecordWriterDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12RtShaderTableRecordWriterDataTrait for RvmSerializedDb_ns_Dx12RtShaderTableRecordWriterData {
}

pub static RVMSERIALIZEDDB_NS_DX12RTSHADERTABLERECORDWRITERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtShaderTableRecordWriterData",
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12RtShaderTableRecordWriterData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12RTSHADERTABLERECORDWRITERDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12RtShaderTableRecordWriterData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTSHADERTABLERECORDWRITERDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12RtNullHitShaderData {
}

pub trait RvmSerializedDb_ns_Dx12RtNullHitShaderDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12RtNullHitShaderDataTrait for RvmSerializedDb_ns_Dx12RtNullHitShaderData {
}

pub static RVMSERIALIZEDDB_NS_DX12RTNULLHITSHADERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtNullHitShaderData",
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12RtNullHitShaderData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12RTNULLHITSHADERDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12RtNullHitShaderData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTNULLHITSHADERDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12RtHitCollectionData {
}

pub trait RvmSerializedDb_ns_Dx12RtHitCollectionDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12RtHitCollectionDataTrait for RvmSerializedDb_ns_Dx12RtHitCollectionData {
}

pub static RVMSERIALIZEDDB_NS_DX12RTHITCOLLECTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtHitCollectionData",
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12RtHitCollectionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12RTHITCOLLECTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx12RtHitCollectionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTHITCOLLECTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12RtStaticBlasBuildData {
}

pub trait RvmSerializedDb_ns_Dx12RtStaticBlasBuildDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12RtStaticBlasBuildDataTrait for RvmSerializedDb_ns_Dx12RtStaticBlasBuildData {
}

pub static RVMSERIALIZEDDB_NS_DX12RTSTATICBLASBUILDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtStaticBlasBuildData",
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12RtStaticBlasBuildData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12RTSTATICBLASBUILDDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12RtStaticBlasBuildData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTSTATICBLASBUILDDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12RtDynamicBlasBuildData {
}

pub trait RvmSerializedDb_ns_Dx12RtDynamicBlasBuildDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12RtDynamicBlasBuildDataTrait for RvmSerializedDb_ns_Dx12RtDynamicBlasBuildData {
}

pub static RVMSERIALIZEDDB_NS_DX12RTDYNAMICBLASBUILDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtDynamicBlasBuildData",
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12RtDynamicBlasBuildData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12RTDYNAMICBLASBUILDDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12RtDynamicBlasBuildData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTDYNAMICBLASBUILDDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12RtBlasBaseData {
}

pub trait RvmSerializedDb_ns_Dx12RtBlasBaseDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12RtBlasBaseDataTrait for RvmSerializedDb_ns_Dx12RtBlasBaseData {
}

pub static RVMSERIALIZEDDB_NS_DX12RTBLASBASEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtBlasBaseData",
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12RtBlasBaseData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12RTBLASBASEDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12RtBlasBaseData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTBLASBASEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12RtHitShaderConstantData {
}

pub trait RvmSerializedDb_ns_Dx12RtHitShaderConstantDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12RtHitShaderConstantDataTrait for RvmSerializedDb_ns_Dx12RtHitShaderConstantData {
}

pub static RVMSERIALIZEDDB_NS_DX12RTHITSHADERCONSTANTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtHitShaderConstantData",
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12RtHitShaderConstantData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12RTHITSHADERCONSTANTDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12RtHitShaderConstantData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTHITSHADERCONSTANTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12RtIndexBufferData {
}

pub trait RvmSerializedDb_ns_Dx12RtIndexBufferDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12RtIndexBufferDataTrait for RvmSerializedDb_ns_Dx12RtIndexBufferData {
}

pub static RVMSERIALIZEDDB_NS_DX12RTINDEXBUFFERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtIndexBufferData",
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12RtIndexBufferData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12RTINDEXBUFFERDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12RtIndexBufferData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTINDEXBUFFERDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12RtSortData {
}

pub trait RvmSerializedDb_ns_Dx12RtSortDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12RtSortDataTrait for RvmSerializedDb_ns_Dx12RtSortData {
}

pub static RVMSERIALIZEDDB_NS_DX12RTSORTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtSortData",
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12RtSortData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12RTSORTDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12RtSortData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTSORTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12RtDispatchData {
}

pub trait RvmSerializedDb_ns_Dx12RtDispatchDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12RtDispatchDataTrait for RvmSerializedDb_ns_Dx12RtDispatchData {
}

pub static RVMSERIALIZEDDB_NS_DX12RTDISPATCHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtDispatchData",
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12RtDispatchData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12RTDISPATCHDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RvmSerializedDb_ns_Dx12RtDispatchData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTDISPATCHDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct Dx12RtRvmBackendConfig {
    pub _glacier_base: super::rvm_dx12_pc::Dx12PcRvmBackendConfig,
    pub enable_all_shader_graphs: bool,
    pub enabled_shader_graphs: Vec<Option<Arc<Mutex<dyn super::render::ShaderGraphTrait>>>>,
}

pub trait Dx12RtRvmBackendConfigTrait: super::rvm_dx12_pc::Dx12PcRvmBackendConfigTrait {
    fn enable_all_shader_graphs(&self) -> &bool;
    fn enabled_shader_graphs(&self) -> &Vec<Option<Arc<Mutex<dyn super::render::ShaderGraphTrait>>>>;
}

impl Dx12RtRvmBackendConfigTrait for Dx12RtRvmBackendConfig {
    fn enable_all_shader_graphs(&self) -> &bool {
        &self.enable_all_shader_graphs
    }
    fn enabled_shader_graphs(&self) -> &Vec<Option<Arc<Mutex<dyn super::render::ShaderGraphTrait>>>> {
        &self.enabled_shader_graphs
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
}

impl super::core::DataContainerTrait for Dx12RtRvmBackendConfig {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static DX12RTRVMBACKENDCONFIG_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtRvmBackendConfig",
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_dx12_pc::DX12PCRVMBACKENDCONFIG_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RtRvmBackendConfig as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "EnableAllShaderGraphs",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(Dx12RtRvmBackendConfig, enable_all_shader_graphs),
            },
            FieldInfoData {
                name: "EnabledShaderGraphs",
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
}


pub static DX12RTRVMBACKENDCONFIG_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtRvmBackendConfig-Array",
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtRvmBackendConfig"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12RtCollectionPreloadOp {
}

pub trait RvmSerializedDb_ns_Dx12RtCollectionPreloadOpTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12RtCollectionPreloadOpTrait for RvmSerializedDb_ns_Dx12RtCollectionPreloadOp {
}

pub static RVMSERIALIZEDDB_NS_DX12RTCOLLECTIONPRELOADOP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtCollectionPreloadOp",
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12RtCollectionPreloadOp as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12RTCOLLECTIONPRELOADOP_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx12RtCollectionPreloadOp {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTCOLLECTIONPRELOADOP_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMCOMMONDATABASELOADER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RtRvmDatabaseLoader as Default>::default())),
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
}


pub static DX12RTRVMDATABASELOADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtRvmDatabaseLoader-Array",
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtRvmDatabaseLoader"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_dx12_pc::DX12PCRVMDATABASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12RtRvmDatabase as Default>::default())),
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
}


pub static DX12RTRVMDATABASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtRvmDatabase-Array",
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtRvmDatabase"),
    array_type: None,
    alignment: 8,
};


