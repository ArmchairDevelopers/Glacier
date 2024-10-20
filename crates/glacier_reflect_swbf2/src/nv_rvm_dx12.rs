use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12NvRvmBackendConfig {
}

pub const DX12NVRVMBACKENDCONFIG_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12NvRvmBackendConfig",
    flags: MemberInfoFlags::new(101),
    module: "NvRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DX12PCRVMBACKENDCONFIG_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12NVRVMBACKENDCONFIG_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12NvRvmBackendConfig {
    fn type_info() -> &'static TypeInfo {
        DX12NVRVMBACKENDCONFIG_TYPE_INFO
    }
}


pub const DX12NVRVMBACKENDCONFIG_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12NvRvmBackendConfig-Array",
    flags: MemberInfoFlags::new(145),
    module: "NvRvmDx12",
    data: TypeInfoData::Array("Dx12NvRvmBackendConfig-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12NvRvmRootSignature {
}

pub const DX12NVRVMROOTSIGNATURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12NvRvmRootSignature",
    flags: MemberInfoFlags::new(53321),
    module: "NvRvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(DX12NVRVMROOTSIGNATURE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12NvRvmRootSignature {
    fn type_info() -> &'static TypeInfo {
        DX12NVRVMROOTSIGNATURE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12NvRvmConstantBufferAssemblyInstructionFactory {
}

pub const DX12NVRVMCONSTANTBUFFERASSEMBLYINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12NvRvmConstantBufferAssemblyInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "NvRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12NVRVMCONSTANTBUFFERASSEMBLYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12NvRvmConstantBufferAssemblyInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX12NVRVMCONSTANTBUFFERASSEMBLYINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX12NVRVMCONSTANTBUFFERASSEMBLYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12NvRvmConstantBufferAssemblyInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "NvRvmDx12",
    data: TypeInfoData::Array("Dx12NvRvmConstantBufferAssemblyInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12NvRvmLegacyDrawStateBuilderInstructionFactory {
}

pub const DX12NVRVMLEGACYDRAWSTATEBUILDERINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12NvRvmLegacyDrawStateBuilderInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "NvRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12NVRVMLEGACYDRAWSTATEBUILDERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12NvRvmLegacyDrawStateBuilderInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX12NVRVMLEGACYDRAWSTATEBUILDERINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX12NVRVMLEGACYDRAWSTATEBUILDERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12NvRvmLegacyDrawStateBuilderInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "NvRvmDx12",
    data: TypeInfoData::Array("Dx12NvRvmLegacyDrawStateBuilderInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12NvRvmDescriptorTableAssemblyInstructionFactory {
}

pub const DX12NVRVMDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12NvRvmDescriptorTableAssemblyInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "NvRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12NVRVMDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12NvRvmDescriptorTableAssemblyInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX12NVRVMDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX12NVRVMDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12NvRvmDescriptorTableAssemblyInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "NvRvmDx12",
    data: TypeInfoData::Array("Dx12NvRvmDescriptorTableAssemblyInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12NvRvmDispatchInstructionFactory {
}

pub const DX12NVRVMDISPATCHINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12NvRvmDispatchInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "NvRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12NVRVMDISPATCHINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12NvRvmDispatchInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX12NVRVMDISPATCHINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX12NVRVMDISPATCHINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12NvRvmDispatchInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "NvRvmDx12",
    data: TypeInfoData::Array("Dx12NvRvmDispatchInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12NvRvmDatabaseLoader {
}

pub const DX12NVRVMDATABASELOADER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12NvRvmDatabaseLoader",
    flags: MemberInfoFlags::new(101),
    module: "NvRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMCOMMONDATABASELOADER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12NVRVMDATABASELOADER_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for Dx12NvRvmDatabaseLoader {
    fn type_info() -> &'static TypeInfo {
        DX12NVRVMDATABASELOADER_TYPE_INFO
    }
}


pub const DX12NVRVMDATABASELOADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12NvRvmDatabaseLoader-Array",
    flags: MemberInfoFlags::new(145),
    module: "NvRvmDx12",
    data: TypeInfoData::Array("Dx12NvRvmDatabaseLoader-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12NvRvmDatabase {
}

pub const DX12NVRVMDATABASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12NvRvmDatabase",
    flags: MemberInfoFlags::new(101),
    module: "NvRvmDx12",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DX12PCRVMDATABASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12NVRVMDATABASE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12NvRvmDatabase {
    fn type_info() -> &'static TypeInfo {
        DX12NVRVMDATABASE_TYPE_INFO
    }
}


pub const DX12NVRVMDATABASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12NvRvmDatabase-Array",
    flags: MemberInfoFlags::new(145),
    module: "NvRvmDx12",
    data: TypeInfoData::Array("Dx12NvRvmDatabase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12NvDescriptorTableAssemblyInstructionData {
}

pub const RVMSERIALIZEDDB_NS_DX12NVDESCRIPTORTABLEASSEMBLYINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12NvDescriptorTableAssemblyInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "NvRvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12NVDESCRIPTORTABLEASSEMBLYINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx12NvDescriptorTableAssemblyInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12NVDESCRIPTORTABLEASSEMBLYINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12NvLegacyDrawStateBuilderInstructionBatchData {
}

pub const RVMSERIALIZEDDB_NS_DX12NVLEGACYDRAWSTATEBUILDERINSTRUCTIONBATCHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12NvLegacyDrawStateBuilderInstructionBatchData",
    flags: MemberInfoFlags::new(53321),
    module: "NvRvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12NVLEGACYDRAWSTATEBUILDERINSTRUCTIONBATCHDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx12NvLegacyDrawStateBuilderInstructionBatchData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12NVLEGACYDRAWSTATEBUILDERINSTRUCTIONBATCHDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12NvLegacyDrawStateBuilderInstructionData {
}

pub const RVMSERIALIZEDDB_NS_DX12NVLEGACYDRAWSTATEBUILDERINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12NvLegacyDrawStateBuilderInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "NvRvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12NVLEGACYDRAWSTATEBUILDERINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx12NvLegacyDrawStateBuilderInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12NVLEGACYDRAWSTATEBUILDERINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12NvConstantBufferAssemblyInstructionData {
}

pub const RVMSERIALIZEDDB_NS_DX12NVCONSTANTBUFFERASSEMBLYINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12NvConstantBufferAssemblyInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "NvRvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12NVCONSTANTBUFFERASSEMBLYINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RvmSerializedDb_ns_Dx12NvConstantBufferAssemblyInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12NVCONSTANTBUFFERASSEMBLYINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12NvDescriptorTable {
}

pub const RVMSERIALIZEDDB_NS_DX12NVDESCRIPTORTABLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12NvDescriptorTable",
    flags: MemberInfoFlags::new(53321),
    module: "NvRvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12NVDESCRIPTORTABLE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12NvDescriptorTable {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12NVDESCRIPTORTABLE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12NvRvmRootSignature {
}

pub const RVMSERIALIZEDDB_NS_DX12NVRVMROOTSIGNATURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12NvRvmRootSignature",
    flags: MemberInfoFlags::new(53321),
    module: "NvRvmDx12",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12NVRVMROOTSIGNATURE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx12NvRvmRootSignature {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12NVRVMROOTSIGNATURE_TYPE_INFO
    }
}

