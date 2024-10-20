use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_rvm_dx12_pc_types(registry: &mut TypeRegistry) {
    registry.register_type(DX12PCRVMDATABASE_TYPE_INFO);
    registry.register_type(DX12PCRVMDATABASE_ARRAY_TYPE_INFO);
    registry.register_type(DX12PCRVMBACKEND_TYPE_INFO);
    registry.register_type(DX12PCRVMBACKEND_ARRAY_TYPE_INFO);
    registry.register_type(DX12PCRVMBACKENDFACTORY_TYPE_INFO);
    registry.register_type(DX12PCRVMBACKENDFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(RVMDX12PCSETTINGS_TYPE_INFO);
    registry.register_type(RVMDX12PCSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(DX12PCCOMPUTEPSOINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX12PCCOMPUTEPSOINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DX12PCCSSKINNINGDISPATCHINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX12PCCSSKINNINGDISPATCHINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DX12PCCSSKINNINGBUFFERINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX12PCCSSKINNINGBUFFERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DX12PCCSSKINNINGPARAMSINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX12PCCSSKINNINGPARAMSINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DX12PCVERTEXBUFFERINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX12PCVERTEXBUFFERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DX12PCVERTEXBUFFERVIEWINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX12PCVERTEXBUFFERVIEWINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DX12PCRVMVIEWSTATEINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX12PCRVMVIEWSTATEINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12PCCOMPUTEPSOINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12PCCSSKINNINGDISPATCHINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12PCCSSKINNINGBUFFERINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12PCCSSKINNINGPARAMSINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12PCVERTEXBUFFERINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(DX12PCCOMMANDLISTFRAGMENTSLOT_TYPE_INFO);
    registry.register_type(DX12PCCOMMANDLISTFRAGMENTSLOT_ARRAY_TYPE_INFO);
    registry.register_type(DX12PCRVMBACKENDCONFIG_TYPE_INFO);
    registry.register_type(DX12PCRVMBACKENDCONFIG_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12PCSAMPLERPOINTER_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12PCSAMPLERTABLEWRITERINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12PCRVMDESCRIPTORTABLEASSEMBLYINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12PCSHADERDISPATCHDRAWINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12PCDISPATCHINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(DX12PCRVMSAMPLERTABLEWRITERINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX12PCRVMSAMPLERTABLEWRITERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DX12PCRVMSHADERTABLEROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX12PCRVMSHADERTABLEROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DX12PCRVMCOMPUTEROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX12PCRVMCOMPUTEROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DX12PCRVMGRAPHICSROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX12PCRVMGRAPHICSROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DX12PCRVMINSTANCETABLEASSEMBLYINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX12PCRVMINSTANCETABLEASSEMBLYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DX12PCRVMDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX12PCRVMDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DX12PCRVMTABLEASSEMBLYINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX12PCRVMTABLEASSEMBLYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DX12PCRVMLEGACYDRAWSTATEBUILDERINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX12PCRVMLEGACYDRAWSTATEBUILDERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DX12PCRVMSHADERDISPATCHLEGACYDRAWINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX12PCRVMSHADERDISPATCHLEGACYDRAWINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DX12PCRVMDISPATCHINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX12PCRVMDISPATCHINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12PCPSOPRELOADOP_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12PCSAMPLER_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12PCROOTSIGNATURE_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12PcRvmDatabase {
}

pub const DX12PCRVMDATABASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmDatabase",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DX12RVMDATABASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12PCRVMDATABASE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12PcRvmDatabase {
    fn type_info() -> &'static TypeInfo {
        DX12PCRVMDATABASE_TYPE_INFO
    }
}


pub const DX12PCRVMDATABASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmDatabase-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcRvmDatabase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12PcRvmBackend {
}

pub const DX12PCRVMBACKEND_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmBackend",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMMONRVMBACKEND_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12PCRVMBACKEND_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Dx12PcRvmBackend {
    fn type_info() -> &'static TypeInfo {
        DX12PCRVMBACKEND_TYPE_INFO
    }
}


pub const DX12PCRVMBACKEND_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmBackend-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcRvmBackend-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12PcRvmBackendFactory {
}

pub const DX12PCRVMBACKENDFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmBackendFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMBACKENDFACTORY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12PCRVMBACKENDFACTORY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Dx12PcRvmBackendFactory {
    fn type_info() -> &'static TypeInfo {
        DX12PCRVMBACKENDFACTORY_TYPE_INFO
    }
}


pub const DX12PCRVMBACKENDFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmBackendFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcRvmBackendFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmDx12PcSettings {
    pub enabled: bool,
    pub enable_bindless: bool,
    pub sampler_descriptor_heap_size: i32,
    pub draw_descriptor_heap_debug: bool,
    pub draw_video_memory_pool_debug: bool,
    pub draw_slab_allocator_stats: bool,
    pub track_slab_allocations: bool,
}

pub const RVMDX12PCSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmDx12PcSettings",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMCOMMONSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RvmDx12PcSettings, enabled),
            },
            FieldInfoData {
                name: "EnableBindless",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RvmDx12PcSettings, enable_bindless),
            },
            FieldInfoData {
                name: "SamplerDescriptorHeapSize",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(RvmDx12PcSettings, sampler_descriptor_heap_size),
            },
            FieldInfoData {
                name: "DrawDescriptorHeapDebug",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RvmDx12PcSettings, draw_descriptor_heap_debug),
            },
            FieldInfoData {
                name: "DrawVideoMemoryPoolDebug",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RvmDx12PcSettings, draw_video_memory_pool_debug),
            },
            FieldInfoData {
                name: "DrawSlabAllocatorStats",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RvmDx12PcSettings, draw_slab_allocator_stats),
            },
            FieldInfoData {
                name: "TrackSlabAllocations",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RvmDx12PcSettings, track_slab_allocations),
            },
        ],
    }),
    array_type: Some(RVMDX12PCSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmDx12PcSettings {
    fn type_info() -> &'static TypeInfo {
        RVMDX12PCSETTINGS_TYPE_INFO
    }
}


pub const RVMDX12PCSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmDx12PcSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("RvmDx12PcSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12PcComputePsoInstructionFactory {
}

pub const DX12PCCOMPUTEPSOINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcComputePsoInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12PCCOMPUTEPSOINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12PcComputePsoInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX12PCCOMPUTEPSOINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX12PCCOMPUTEPSOINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcComputePsoInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcComputePsoInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12PcCsSkinningDispatchInstructionFactory {
}

pub const DX12PCCSSKINNINGDISPATCHINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcCsSkinningDispatchInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12PCCSSKINNINGDISPATCHINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12PcCsSkinningDispatchInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX12PCCSSKINNINGDISPATCHINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX12PCCSSKINNINGDISPATCHINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcCsSkinningDispatchInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcCsSkinningDispatchInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12PcCsSkinningBufferInstructionFactory {
}

pub const DX12PCCSSKINNINGBUFFERINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcCsSkinningBufferInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12PCCSSKINNINGBUFFERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12PcCsSkinningBufferInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX12PCCSSKINNINGBUFFERINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX12PCCSSKINNINGBUFFERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcCsSkinningBufferInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcCsSkinningBufferInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12PcCsSkinningParamsInstructionFactory {
}

pub const DX12PCCSSKINNINGPARAMSINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcCsSkinningParamsInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12PCCSSKINNINGPARAMSINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12PcCsSkinningParamsInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX12PCCSSKINNINGPARAMSINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX12PCCSSKINNINGPARAMSINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcCsSkinningParamsInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcCsSkinningParamsInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12PcVertexBufferInstructionFactory {
}

pub const DX12PCVERTEXBUFFERINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcVertexBufferInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12PCVERTEXBUFFERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12PcVertexBufferInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX12PCVERTEXBUFFERINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX12PCVERTEXBUFFERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcVertexBufferInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcVertexBufferInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12PcVertexBufferViewInstructionFactory {
}

pub const DX12PCVERTEXBUFFERVIEWINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcVertexBufferViewInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12PCVERTEXBUFFERVIEWINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12PcVertexBufferViewInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX12PCVERTEXBUFFERVIEWINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX12PCVERTEXBUFFERVIEWINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcVertexBufferViewInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcVertexBufferViewInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12PcRvmViewStateInstructionFactory {
}

pub const DX12PCRVMVIEWSTATEINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmViewStateInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12PCRVMVIEWSTATEINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12PcRvmViewStateInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX12PCRVMVIEWSTATEINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX12PCRVMVIEWSTATEINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmViewStateInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcRvmViewStateInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12PcComputePsoInstructionData {
}

pub const RVMSERIALIZEDDB_NS_DX12PCCOMPUTEPSOINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcComputePsoInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12PCCOMPUTEPSOINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12PcComputePsoInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12PCCOMPUTEPSOINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12PcCsSkinningDispatchInstructionData {
}

pub const RVMSERIALIZEDDB_NS_DX12PCCSSKINNINGDISPATCHINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcCsSkinningDispatchInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12PCCSSKINNINGDISPATCHINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12PcCsSkinningDispatchInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12PCCSSKINNINGDISPATCHINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12PcCsSkinningBufferInstructionData {
}

pub const RVMSERIALIZEDDB_NS_DX12PCCSSKINNINGBUFFERINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcCsSkinningBufferInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12PCCSSKINNINGBUFFERINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12PcCsSkinningBufferInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12PCCSSKINNINGBUFFERINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12PcCsSkinningParamsInstructionData {
}

pub const RVMSERIALIZEDDB_NS_DX12PCCSSKINNINGPARAMSINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcCsSkinningParamsInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12PCCSSKINNINGPARAMSINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12PcCsSkinningParamsInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12PCCSSKINNINGPARAMSINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12PcVertexBufferInstructionData {
}

pub const RVMSERIALIZEDDB_NS_DX12PCVERTEXBUFFERINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcVertexBufferInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12PCVERTEXBUFFERINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12PcVertexBufferInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12PCVERTEXBUFFERINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum Dx12PcCommandListFragmentSlot {
    #[default]
    Dx12PcCommandListFragment_Draw = 0,
    Dx12PcCommandListFragmentCount = 1,
}

pub const DX12PCCOMMANDLISTFRAGMENTSLOT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcCommandListFragmentSlot",
    flags: MemberInfoFlags::new(49429),
    module: "RvmDx12Pc",
    data: TypeInfoData::Enum,
    array_type: Some(DX12PCCOMMANDLISTFRAGMENTSLOT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for Dx12PcCommandListFragmentSlot {
    fn type_info() -> &'static TypeInfo {
        DX12PCCOMMANDLISTFRAGMENTSLOT_TYPE_INFO
    }
}


pub const DX12PCCOMMANDLISTFRAGMENTSLOT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcCommandListFragmentSlot-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcCommandListFragmentSlot-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12PcRvmBackendConfig {
}

pub const DX12PCRVMBACKENDCONFIG_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmBackendConfig",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DX12RVMBACKENDCONFIG_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12PCRVMBACKENDCONFIG_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12PcRvmBackendConfig {
    fn type_info() -> &'static TypeInfo {
        DX12PCRVMBACKENDCONFIG_TYPE_INFO
    }
}


pub const DX12PCRVMBACKENDCONFIG_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmBackendConfig-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcRvmBackendConfig-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12PcSamplerPointer {
}

pub const RVMSERIALIZEDDB_NS_DX12PCSAMPLERPOINTER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcSamplerPointer",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12PCSAMPLERPOINTER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx12PcSamplerPointer {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12PCSAMPLERPOINTER_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12PcSamplerTableWriterInstructionData {
}

pub const RVMSERIALIZEDDB_NS_DX12PCSAMPLERTABLEWRITERINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcSamplerTableWriterInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12PCSAMPLERTABLEWRITERINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx12PcSamplerTableWriterInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12PCSAMPLERTABLEWRITERINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12PcRvmDescriptorTableAssemblyInstructionData {
}

pub const RVMSERIALIZEDDB_NS_DX12PCRVMDESCRIPTORTABLEASSEMBLYINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcRvmDescriptorTableAssemblyInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12PCRVMDESCRIPTORTABLEASSEMBLYINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx12PcRvmDescriptorTableAssemblyInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12PCRVMDESCRIPTORTABLEASSEMBLYINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12PcShaderDispatchDrawInstructionData {
}

pub const RVMSERIALIZEDDB_NS_DX12PCSHADERDISPATCHDRAWINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcShaderDispatchDrawInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12PCSHADERDISPATCHDRAWINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12PcShaderDispatchDrawInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12PCSHADERDISPATCHDRAWINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12PcDispatchInstructionData {
}

pub const RVMSERIALIZEDDB_NS_DX12PCDISPATCHINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcDispatchInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12PCDISPATCHINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx12PcDispatchInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12PCDISPATCHINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12PcRvmSamplerTableWriterInstructionFactory {
}

pub const DX12PCRVMSAMPLERTABLEWRITERINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmSamplerTableWriterInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12PCRVMSAMPLERTABLEWRITERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12PcRvmSamplerTableWriterInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX12PCRVMSAMPLERTABLEWRITERINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX12PCRVMSAMPLERTABLEWRITERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmSamplerTableWriterInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcRvmSamplerTableWriterInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12PcRvmShaderTableRootDescriptorTableAssemblyInstructionFactory {
}

pub const DX12PCRVMSHADERTABLEROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmShaderTableRootDescriptorTableAssemblyInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12PCRVMSHADERTABLEROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12PcRvmShaderTableRootDescriptorTableAssemblyInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX12PCRVMSHADERTABLEROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX12PCRVMSHADERTABLEROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmShaderTableRootDescriptorTableAssemblyInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcRvmShaderTableRootDescriptorTableAssemblyInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12PcRvmComputeRootDescriptorTableAssemblyInstructionFactory {
}

pub const DX12PCRVMCOMPUTEROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmComputeRootDescriptorTableAssemblyInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12PCRVMCOMPUTEROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12PcRvmComputeRootDescriptorTableAssemblyInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX12PCRVMCOMPUTEROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX12PCRVMCOMPUTEROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmComputeRootDescriptorTableAssemblyInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcRvmComputeRootDescriptorTableAssemblyInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12PcRvmGraphicsRootDescriptorTableAssemblyInstructionFactory {
}

pub const DX12PCRVMGRAPHICSROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmGraphicsRootDescriptorTableAssemblyInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12PCRVMGRAPHICSROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12PcRvmGraphicsRootDescriptorTableAssemblyInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX12PCRVMGRAPHICSROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX12PCRVMGRAPHICSROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmGraphicsRootDescriptorTableAssemblyInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcRvmGraphicsRootDescriptorTableAssemblyInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12PcRvmInstanceTableAssemblyInstructionFactory {
}

pub const DX12PCRVMINSTANCETABLEASSEMBLYINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmInstanceTableAssemblyInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12PCRVMINSTANCETABLEASSEMBLYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12PcRvmInstanceTableAssemblyInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX12PCRVMINSTANCETABLEASSEMBLYINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX12PCRVMINSTANCETABLEASSEMBLYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmInstanceTableAssemblyInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcRvmInstanceTableAssemblyInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12PcRvmDescriptorTableAssemblyInstructionFactory {
}

pub const DX12PCRVMDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmDescriptorTableAssemblyInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12PCRVMDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12PcRvmDescriptorTableAssemblyInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX12PCRVMDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX12PCRVMDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmDescriptorTableAssemblyInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcRvmDescriptorTableAssemblyInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12PcRvmTableAssemblyInstructionFactory {
}

pub const DX12PCRVMTABLEASSEMBLYINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmTableAssemblyInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12PCRVMTABLEASSEMBLYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12PcRvmTableAssemblyInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX12PCRVMTABLEASSEMBLYINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX12PCRVMTABLEASSEMBLYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmTableAssemblyInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcRvmTableAssemblyInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12PcRvmLegacyDrawStateBuilderInstructionFactory {
}

pub const DX12PCRVMLEGACYDRAWSTATEBUILDERINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmLegacyDrawStateBuilderInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12PCRVMLEGACYDRAWSTATEBUILDERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12PcRvmLegacyDrawStateBuilderInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX12PCRVMLEGACYDRAWSTATEBUILDERINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX12PCRVMLEGACYDRAWSTATEBUILDERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmLegacyDrawStateBuilderInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcRvmLegacyDrawStateBuilderInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12PcRvmShaderDispatchLegacyDrawInstructionFactory {
}

pub const DX12PCRVMSHADERDISPATCHLEGACYDRAWINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmShaderDispatchLegacyDrawInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12PCRVMSHADERDISPATCHLEGACYDRAWINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12PcRvmShaderDispatchLegacyDrawInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX12PCRVMSHADERDISPATCHLEGACYDRAWINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX12PCRVMSHADERDISPATCHLEGACYDRAWINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmShaderDispatchLegacyDrawInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcRvmShaderDispatchLegacyDrawInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12PcRvmDispatchInstructionFactory {
}

pub const DX12PCRVMDISPATCHINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmDispatchInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12PCRVMDISPATCHINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12PcRvmDispatchInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX12PCRVMDISPATCHINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX12PCRVMDISPATCHINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmDispatchInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcRvmDispatchInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12PcPSOPreloadOp {
}

pub const RVMSERIALIZEDDB_NS_DX12PCPSOPRELOADOP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcPSOPreloadOp",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12PCPSOPRELOADOP_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx12PcPSOPreloadOp {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12PCPSOPRELOADOP_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12PcSampler {
}

pub const RVMSERIALIZEDDB_NS_DX12PCSAMPLER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcSampler",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12PCSAMPLER_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RvmSerializedDb_ns_Dx12PcSampler {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12PCSAMPLER_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx12PcRootSignature {
}

pub const RVMSERIALIZEDDB_NS_DX12PCROOTSIGNATURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcRootSignature",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX12PCROOTSIGNATURE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx12PcRootSignature {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12PCROOTSIGNATURE_TYPE_INFO
    }
}

