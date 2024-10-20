use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmDx12RtSettings {
    pub enabled: bool,
}

pub const RVMDX12RTSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmDx12RtSettings",
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMCOMMONSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RvmDx12RtSettings, enabled),
            },
        ],
    }),
    array_type: Some(RVMDX12RTSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmDx12RtSettings {
    fn type_info() -> &'static TypeInfo {
        RVMDX12RTSETTINGS_TYPE_INFO
    }
}


pub const RVMDX12RTSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmDx12RtSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("RvmDx12RtSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12RtTlasNullInstructionFactory {
}

pub const DX12RTTLASNULLINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtTlasNullInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12RTTLASNULLINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12RtTlasNullInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX12RTTLASNULLINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX12RTTLASNULLINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtTlasNullInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtTlasNullInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12RtTlasDynamicInstructionFactory {
}

pub const DX12RTTLASDYNAMICINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtTlasDynamicInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12RTTLASDYNAMICINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12RtTlasDynamicInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX12RTTLASDYNAMICINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX12RTTLASDYNAMICINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtTlasDynamicInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtTlasDynamicInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12RtTlasStaticInstructionFactory {
}

pub const DX12RTTLASSTATICINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtTlasStaticInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12RTTLASSTATICINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12RtTlasStaticInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX12RTTLASSTATICINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX12RTTLASSTATICINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtTlasStaticInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtTlasStaticInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12RtShaderTableRecordWriterInstructionFactory {
}

pub const DX12RTSHADERTABLERECORDWRITERINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtShaderTableRecordWriterInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12RTSHADERTABLERECORDWRITERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12RtShaderTableRecordWriterInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX12RTSHADERTABLERECORDWRITERINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX12RTSHADERTABLERECORDWRITERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtShaderTableRecordWriterInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtShaderTableRecordWriterInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12RtNullHitShaderInstructionFactory {
}

pub const DX12RTNULLHITSHADERINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtNullHitShaderInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12RTNULLHITSHADERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12RtNullHitShaderInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX12RTNULLHITSHADERINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX12RTNULLHITSHADERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtNullHitShaderInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtNullHitShaderInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12RtHitCollectionInstructionFactory {
}

pub const DX12RTHITCOLLECTIONINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtHitCollectionInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12RTHITCOLLECTIONINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12RtHitCollectionInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX12RTHITCOLLECTIONINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX12RTHITCOLLECTIONINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtHitCollectionInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtHitCollectionInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12RtDynamicBlasBuildInstructionFactory {
}

pub const DX12RTDYNAMICBLASBUILDINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtDynamicBlasBuildInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12RTDYNAMICBLASBUILDINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12RtDynamicBlasBuildInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX12RTDYNAMICBLASBUILDINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX12RTDYNAMICBLASBUILDINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtDynamicBlasBuildInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtDynamicBlasBuildInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12RtStaticBlasBuildInstructionFactory {
}

pub const DX12RTSTATICBLASBUILDINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtStaticBlasBuildInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12RTSTATICBLASBUILDINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12RtStaticBlasBuildInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX12RTSTATICBLASBUILDINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX12RTSTATICBLASBUILDINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtStaticBlasBuildInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtStaticBlasBuildInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12RtHitShaderConstantInstructionFactory {
}

pub const DX12RTHITSHADERCONSTANTINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtHitShaderConstantInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12RTHITSHADERCONSTANTINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12RtHitShaderConstantInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX12RTHITSHADERCONSTANTINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX12RTHITSHADERCONSTANTINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtHitShaderConstantInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtHitShaderConstantInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12RtIndexBufferInstructionFactory {
}

pub const DX12RTINDEXBUFFERINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtIndexBufferInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12RTINDEXBUFFERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12RtIndexBufferInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX12RTINDEXBUFFERINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX12RTINDEXBUFFERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtIndexBufferInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtIndexBufferInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12RtSortInstructionFactory {
}

pub const DX12RTSORTINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtSortInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12RTSORTINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12RtSortInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX12RTSORTINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX12RTSORTINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtSortInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtSortInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12RtDispatchInstructionFactory {
}

pub const DX12RTDISPATCHINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtDispatchInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12RTDISPATCHINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12RtDispatchInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX12RTDISPATCHINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX12RTDISPATCHINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtDispatchInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtDispatchInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12RtTlasNullData {
}

pub const RVMSERIALIZEDDB_NS_DX12RTTLASNULLDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtTlasNullData",
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12RTTLASNULLDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12RtTlasNullData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTTLASNULLDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12RtTlasDynamicData {
}

pub const RVMSERIALIZEDDB_NS_DX12RTTLASDYNAMICDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtTlasDynamicData",
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12RTTLASDYNAMICDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12RtTlasDynamicData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTTLASDYNAMICDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12RtTlasStaticData {
}

pub const RVMSERIALIZEDDB_NS_DX12RTTLASSTATICDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtTlasStaticData",
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12RTTLASSTATICDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12RtTlasStaticData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTTLASSTATICDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12RtTlasBaseData {
}

pub const RVMSERIALIZEDDB_NS_DX12RTTLASBASEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtTlasBaseData",
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12RTTLASBASEDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12RtTlasBaseData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTTLASBASEDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12RtShaderTableRecordWriterData {
}

pub const RVMSERIALIZEDDB_NS_DX12RTSHADERTABLERECORDWRITERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtShaderTableRecordWriterData",
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12RTSHADERTABLERECORDWRITERDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12RtShaderTableRecordWriterData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTSHADERTABLERECORDWRITERDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12RtNullHitShaderData {
}

pub const RVMSERIALIZEDDB_NS_DX12RTNULLHITSHADERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtNullHitShaderData",
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12RTNULLHITSHADERDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12RtNullHitShaderData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTNULLHITSHADERDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12RtHitCollectionData {
}

pub const RVMSERIALIZEDDB_NS_DX12RTHITCOLLECTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtHitCollectionData",
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12RTHITCOLLECTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx12RtHitCollectionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTHITCOLLECTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12RtStaticBlasBuildData {
}

pub const RVMSERIALIZEDDB_NS_DX12RTSTATICBLASBUILDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtStaticBlasBuildData",
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12RTSTATICBLASBUILDDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12RtStaticBlasBuildData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTSTATICBLASBUILDDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12RtDynamicBlasBuildData {
}

pub const RVMSERIALIZEDDB_NS_DX12RTDYNAMICBLASBUILDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtDynamicBlasBuildData",
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12RTDYNAMICBLASBUILDDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12RtDynamicBlasBuildData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTDYNAMICBLASBUILDDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12RtBlasBaseData {
}

pub const RVMSERIALIZEDDB_NS_DX12RTBLASBASEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtBlasBaseData",
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12RTBLASBASEDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12RtBlasBaseData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTBLASBASEDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12RtHitShaderConstantData {
}

pub const RVMSERIALIZEDDB_NS_DX12RTHITSHADERCONSTANTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtHitShaderConstantData",
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12RTHITSHADERCONSTANTDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12RtHitShaderConstantData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTHITSHADERCONSTANTDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12RtIndexBufferData {
}

pub const RVMSERIALIZEDDB_NS_DX12RTINDEXBUFFERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtIndexBufferData",
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12RTINDEXBUFFERDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12RtIndexBufferData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTINDEXBUFFERDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12RtSortData {
}

pub const RVMSERIALIZEDDB_NS_DX12RTSORTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtSortData",
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12RTSORTDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12RtSortData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTSORTDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12RtDispatchData {
}

pub const RVMSERIALIZEDDB_NS_DX12RTDISPATCHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtDispatchData",
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12RTDISPATCHDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RvmSerializedDb_ns_Dx12RtDispatchData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTDISPATCHDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12RtRvmBackendConfig {
    pub enable_all_shader_graphs: bool,
    pub enabled_shader_graphs: Vec<super::render::ShaderGraph>,
}

pub const DX12RTRVMBACKENDCONFIG_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtRvmBackendConfig",
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DX12PCRVMBACKENDCONFIG_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "EnableAllShaderGraphs",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx12RtRvmBackendConfig, enable_all_shader_graphs),
            },
            FieldInfoData {
                name: "EnabledShaderGraphs",
                flags: MemberInfoFlags::new(144),
                field_type: SHADERGRAPH_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(Dx12RtRvmBackendConfig, enabled_shader_graphs),
            },
        ],
    }),
    array_type: Some(DX12RTRVMBACKENDCONFIG_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12RtRvmBackendConfig {
    fn type_info() -> &'static TypeInfo {
        DX12RTRVMBACKENDCONFIG_TYPE_INFO
    }
}


pub const DX12RTRVMBACKENDCONFIG_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtRvmBackendConfig-Array",
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtRvmBackendConfig-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12RtCollectionPreloadOp {
}

pub const RVMSERIALIZEDDB_NS_DX12RTCOLLECTIONPRELOADOP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtCollectionPreloadOp",
    flags: MemberInfoFlags::new(53321),
    module: "RtRvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12RTCOLLECTIONPRELOADOP_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx12RtCollectionPreloadOp {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12RTCOLLECTIONPRELOADOP_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12RtRvmDatabaseLoader {
}

pub const DX12RTRVMDATABASELOADER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtRvmDatabaseLoader",
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMCOMMONDATABASELOADER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12RTRVMDATABASELOADER_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for Dx12RtRvmDatabaseLoader {
    fn type_info() -> &'static TypeInfo {
        DX12RTRVMDATABASELOADER_TYPE_INFO
    }
}


pub const DX12RTRVMDATABASELOADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtRvmDatabaseLoader-Array",
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtRvmDatabaseLoader-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12RtRvmDatabase {
}

pub const DX12RTRVMDATABASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtRvmDatabase",
    flags: MemberInfoFlags::new(101),
    module: "RtRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DX12PCRVMDATABASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12RTRVMDATABASE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12RtRvmDatabase {
    fn type_info() -> &'static TypeInfo {
        DX12RTRVMDATABASE_TYPE_INFO
    }
}


pub const DX12RTRVMDATABASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RtRvmDatabase-Array",
    flags: MemberInfoFlags::new(145),
    module: "RtRvmDx12",
    data: TypeInfoData::Array("Dx12RtRvmDatabase-Array"),
    array_type: None,
    alignment: 8,
};


