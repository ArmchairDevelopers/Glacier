use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct Dx12PcRvmDatabase {
    pub _glacier_base: super::rvm_dx12::Dx12RvmDatabase,
}

pub trait Dx12PcRvmDatabaseTrait: super::rvm_dx12::Dx12RvmDatabaseTrait {
}

impl Dx12PcRvmDatabaseTrait for Dx12PcRvmDatabase {
}

impl super::rvm_dx12::Dx12RvmDatabaseTrait for Dx12PcRvmDatabase {
}

impl super::rvm_common::BaseRvmDatabaseTrait for Dx12PcRvmDatabase {
}

impl super::render::RvmLegacyDatabaseTrait for Dx12PcRvmDatabase {
}

impl super::render::RvmDatabaseTrait for Dx12PcRvmDatabase {
}

impl super::core::IResourceObjectTrait for Dx12PcRvmDatabase {
}

pub static DX12PCRVMDATABASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmDatabase",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_dx12::DX12RVMDATABASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12PcRvmDatabase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12PCRVMDATABASE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12PcRvmDatabase {
    fn type_info(&self) -> &'static TypeInfo {
        DX12PCRVMDATABASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DX12PCRVMDATABASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmDatabase-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcRvmDatabase"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Dx12PcRvmBackend {
    pub _glacier_base: super::rvm_common::CommonRvmBackend,
}

pub trait Dx12PcRvmBackendTrait: super::rvm_common::CommonRvmBackendTrait {
}

impl Dx12PcRvmBackendTrait for Dx12PcRvmBackend {
}

impl super::rvm_common::CommonRvmBackendTrait for Dx12PcRvmBackend {
}

impl super::render::RvmBackendTrait for Dx12PcRvmBackend {
}

pub static DX12PCRVMBACKEND_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmBackend",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::COMMONRVMBACKEND_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12PcRvmBackend as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12PCRVMBACKEND_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Dx12PcRvmBackend {
    fn type_info(&self) -> &'static TypeInfo {
        DX12PCRVMBACKEND_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DX12PCRVMBACKEND_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmBackend-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcRvmBackend"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Dx12PcRvmBackendFactory {
    pub _glacier_base: super::render::RvmBackendFactory,
}

pub trait Dx12PcRvmBackendFactoryTrait: super::render::RvmBackendFactoryTrait {
}

impl Dx12PcRvmBackendFactoryTrait for Dx12PcRvmBackendFactory {
}

impl super::render::RvmBackendFactoryTrait for Dx12PcRvmBackendFactory {
}

pub static DX12PCRVMBACKENDFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmBackendFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::render::RVMBACKENDFACTORY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12PcRvmBackendFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12PCRVMBACKENDFACTORY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Dx12PcRvmBackendFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX12PCRVMBACKENDFACTORY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DX12PCRVMBACKENDFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmBackendFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcRvmBackendFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RvmDx12PcSettings {
    pub _glacier_base: super::rvm_common::RvmCommonSettings,
    pub enabled: bool,
    pub enable_bindless: bool,
    pub sampler_descriptor_heap_size: i32,
    pub draw_descriptor_heap_debug: bool,
    pub draw_video_memory_pool_debug: bool,
    pub draw_slab_allocator_stats: bool,
    pub track_slab_allocations: bool,
}

pub trait RvmDx12PcSettingsTrait: super::rvm_common::RvmCommonSettingsTrait {
    fn enabled(&self) -> &bool;
    fn enable_bindless(&self) -> &bool;
    fn sampler_descriptor_heap_size(&self) -> &i32;
    fn draw_descriptor_heap_debug(&self) -> &bool;
    fn draw_video_memory_pool_debug(&self) -> &bool;
    fn draw_slab_allocator_stats(&self) -> &bool;
    fn track_slab_allocations(&self) -> &bool;
}

impl RvmDx12PcSettingsTrait for RvmDx12PcSettings {
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn enable_bindless(&self) -> &bool {
        &self.enable_bindless
    }
    fn sampler_descriptor_heap_size(&self) -> &i32 {
        &self.sampler_descriptor_heap_size
    }
    fn draw_descriptor_heap_debug(&self) -> &bool {
        &self.draw_descriptor_heap_debug
    }
    fn draw_video_memory_pool_debug(&self) -> &bool {
        &self.draw_video_memory_pool_debug
    }
    fn draw_slab_allocator_stats(&self) -> &bool {
        &self.draw_slab_allocator_stats
    }
    fn track_slab_allocations(&self) -> &bool {
        &self.track_slab_allocations
    }
}

impl super::rvm_common::RvmCommonSettingsTrait for RvmDx12PcSettings {
    fn on_demand_building_enable(&self) -> &bool {
        self._glacier_base.on_demand_building_enable()
    }
}

impl super::core::DataContainerTrait for RvmDx12PcSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static RVMDX12PCSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmDx12PcSettings",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMCOMMONSETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmDx12PcSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RvmDx12PcSettings, enabled),
            },
            FieldInfoData {
                name: "EnableBindless",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RvmDx12PcSettings, enable_bindless),
            },
            FieldInfoData {
                name: "SamplerDescriptorHeapSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(RvmDx12PcSettings, sampler_descriptor_heap_size),
            },
            FieldInfoData {
                name: "DrawDescriptorHeapDebug",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RvmDx12PcSettings, draw_descriptor_heap_debug),
            },
            FieldInfoData {
                name: "DrawVideoMemoryPoolDebug",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RvmDx12PcSettings, draw_video_memory_pool_debug),
            },
            FieldInfoData {
                name: "DrawSlabAllocatorStats",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RvmDx12PcSettings, draw_slab_allocator_stats),
            },
            FieldInfoData {
                name: "TrackSlabAllocations",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RvmDx12PcSettings, track_slab_allocations),
            },
        ],
    }),
    array_type: Some(RVMDX12PCSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmDx12PcSettings {
    fn type_info(&self) -> &'static TypeInfo {
        RVMDX12PCSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RVMDX12PCSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmDx12PcSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("RvmDx12PcSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Dx12PcComputePsoInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx12PcComputePsoInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx12PcComputePsoInstructionFactoryTrait for Dx12PcComputePsoInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx12PcComputePsoInstructionFactory {
}

pub static DX12PCCOMPUTEPSOINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcComputePsoInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12PcComputePsoInstructionFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12PCCOMPUTEPSOINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12PcComputePsoInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX12PCCOMPUTEPSOINSTRUCTIONFACTORY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DX12PCCOMPUTEPSOINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcComputePsoInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcComputePsoInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Dx12PcCsSkinningDispatchInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx12PcCsSkinningDispatchInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx12PcCsSkinningDispatchInstructionFactoryTrait for Dx12PcCsSkinningDispatchInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx12PcCsSkinningDispatchInstructionFactory {
}

pub static DX12PCCSSKINNINGDISPATCHINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcCsSkinningDispatchInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12PcCsSkinningDispatchInstructionFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12PCCSSKINNINGDISPATCHINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12PcCsSkinningDispatchInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX12PCCSSKINNINGDISPATCHINSTRUCTIONFACTORY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DX12PCCSSKINNINGDISPATCHINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcCsSkinningDispatchInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcCsSkinningDispatchInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Dx12PcCsSkinningBufferInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx12PcCsSkinningBufferInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx12PcCsSkinningBufferInstructionFactoryTrait for Dx12PcCsSkinningBufferInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx12PcCsSkinningBufferInstructionFactory {
}

pub static DX12PCCSSKINNINGBUFFERINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcCsSkinningBufferInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12PcCsSkinningBufferInstructionFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12PCCSSKINNINGBUFFERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12PcCsSkinningBufferInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX12PCCSSKINNINGBUFFERINSTRUCTIONFACTORY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DX12PCCSSKINNINGBUFFERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcCsSkinningBufferInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcCsSkinningBufferInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Dx12PcCsSkinningParamsInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx12PcCsSkinningParamsInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx12PcCsSkinningParamsInstructionFactoryTrait for Dx12PcCsSkinningParamsInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx12PcCsSkinningParamsInstructionFactory {
}

pub static DX12PCCSSKINNINGPARAMSINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcCsSkinningParamsInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12PcCsSkinningParamsInstructionFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12PCCSSKINNINGPARAMSINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12PcCsSkinningParamsInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX12PCCSSKINNINGPARAMSINSTRUCTIONFACTORY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DX12PCCSSKINNINGPARAMSINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcCsSkinningParamsInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcCsSkinningParamsInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Dx12PcVertexBufferInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx12PcVertexBufferInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx12PcVertexBufferInstructionFactoryTrait for Dx12PcVertexBufferInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx12PcVertexBufferInstructionFactory {
}

pub static DX12PCVERTEXBUFFERINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcVertexBufferInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12PcVertexBufferInstructionFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12PCVERTEXBUFFERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12PcVertexBufferInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX12PCVERTEXBUFFERINSTRUCTIONFACTORY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DX12PCVERTEXBUFFERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcVertexBufferInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcVertexBufferInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Dx12PcVertexBufferViewInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx12PcVertexBufferViewInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx12PcVertexBufferViewInstructionFactoryTrait for Dx12PcVertexBufferViewInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx12PcVertexBufferViewInstructionFactory {
}

pub static DX12PCVERTEXBUFFERVIEWINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcVertexBufferViewInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12PcVertexBufferViewInstructionFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12PCVERTEXBUFFERVIEWINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12PcVertexBufferViewInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX12PCVERTEXBUFFERVIEWINSTRUCTIONFACTORY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DX12PCVERTEXBUFFERVIEWINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcVertexBufferViewInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcVertexBufferViewInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Dx12PcRvmViewStateInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx12PcRvmViewStateInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx12PcRvmViewStateInstructionFactoryTrait for Dx12PcRvmViewStateInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx12PcRvmViewStateInstructionFactory {
}

pub static DX12PCRVMVIEWSTATEINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmViewStateInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12PcRvmViewStateInstructionFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12PCRVMVIEWSTATEINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12PcRvmViewStateInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX12PCRVMVIEWSTATEINSTRUCTIONFACTORY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DX12PCRVMVIEWSTATEINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmViewStateInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcRvmViewStateInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12PcComputePsoInstructionData {
}

pub trait RvmSerializedDb_ns_Dx12PcComputePsoInstructionDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12PcComputePsoInstructionDataTrait for RvmSerializedDb_ns_Dx12PcComputePsoInstructionData {
}

pub static RVMSERIALIZEDDB_NS_DX12PCCOMPUTEPSOINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcComputePsoInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12PcComputePsoInstructionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12PCCOMPUTEPSOINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12PcComputePsoInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12PCCOMPUTEPSOINSTRUCTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12PcCsSkinningDispatchInstructionData {
}

pub trait RvmSerializedDb_ns_Dx12PcCsSkinningDispatchInstructionDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12PcCsSkinningDispatchInstructionDataTrait for RvmSerializedDb_ns_Dx12PcCsSkinningDispatchInstructionData {
}

pub static RVMSERIALIZEDDB_NS_DX12PCCSSKINNINGDISPATCHINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcCsSkinningDispatchInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12PcCsSkinningDispatchInstructionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12PCCSSKINNINGDISPATCHINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12PcCsSkinningDispatchInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12PCCSSKINNINGDISPATCHINSTRUCTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12PcCsSkinningBufferInstructionData {
}

pub trait RvmSerializedDb_ns_Dx12PcCsSkinningBufferInstructionDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12PcCsSkinningBufferInstructionDataTrait for RvmSerializedDb_ns_Dx12PcCsSkinningBufferInstructionData {
}

pub static RVMSERIALIZEDDB_NS_DX12PCCSSKINNINGBUFFERINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcCsSkinningBufferInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12PcCsSkinningBufferInstructionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12PCCSSKINNINGBUFFERINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12PcCsSkinningBufferInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12PCCSSKINNINGBUFFERINSTRUCTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12PcCsSkinningParamsInstructionData {
}

pub trait RvmSerializedDb_ns_Dx12PcCsSkinningParamsInstructionDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12PcCsSkinningParamsInstructionDataTrait for RvmSerializedDb_ns_Dx12PcCsSkinningParamsInstructionData {
}

pub static RVMSERIALIZEDDB_NS_DX12PCCSSKINNINGPARAMSINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcCsSkinningParamsInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12PcCsSkinningParamsInstructionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12PCCSSKINNINGPARAMSINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12PcCsSkinningParamsInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12PCCSSKINNINGPARAMSINSTRUCTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12PcVertexBufferInstructionData {
}

pub trait RvmSerializedDb_ns_Dx12PcVertexBufferInstructionDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12PcVertexBufferInstructionDataTrait for RvmSerializedDb_ns_Dx12PcVertexBufferInstructionData {
}

pub static RVMSERIALIZEDDB_NS_DX12PCVERTEXBUFFERINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcVertexBufferInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12PcVertexBufferInstructionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12PCVERTEXBUFFERINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12PcVertexBufferInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12PCVERTEXBUFFERINSTRUCTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum Dx12PcCommandListFragmentSlot {
    #[default]
    Dx12PcCommandListFragment_Draw = 0,
    Dx12PcCommandListFragmentCount = 1,
}

pub static DX12PCCOMMANDLISTFRAGMENTSLOT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcCommandListFragmentSlot",
    flags: MemberInfoFlags::new(49429),
    module: "RvmDx12Pc",
    data: TypeInfoData::Enum,
    array_type: Some(DX12PCCOMMANDLISTFRAGMENTSLOT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for Dx12PcCommandListFragmentSlot {
    fn type_info(&self) -> &'static TypeInfo {
        DX12PCCOMMANDLISTFRAGMENTSLOT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DX12PCCOMMANDLISTFRAGMENTSLOT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcCommandListFragmentSlot-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcCommandListFragmentSlot"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Dx12PcRvmBackendConfig {
    pub _glacier_base: super::rvm_dx12::Dx12RvmBackendConfig,
}

pub trait Dx12PcRvmBackendConfigTrait: super::rvm_dx12::Dx12RvmBackendConfigTrait {
}

impl Dx12PcRvmBackendConfigTrait for Dx12PcRvmBackendConfig {
}

impl super::rvm_dx12::Dx12RvmBackendConfigTrait for Dx12PcRvmBackendConfig {
}

impl super::rvm_common::RvmBackendConfigTrait for Dx12PcRvmBackendConfig {
}

impl super::core::AssetTrait for Dx12PcRvmBackendConfig {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for Dx12PcRvmBackendConfig {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static DX12PCRVMBACKENDCONFIG_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmBackendConfig",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_dx12::DX12RVMBACKENDCONFIG_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12PcRvmBackendConfig as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12PCRVMBACKENDCONFIG_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12PcRvmBackendConfig {
    fn type_info(&self) -> &'static TypeInfo {
        DX12PCRVMBACKENDCONFIG_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DX12PCRVMBACKENDCONFIG_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmBackendConfig-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcRvmBackendConfig"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12PcSamplerPointer {
}

pub trait RvmSerializedDb_ns_Dx12PcSamplerPointerTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12PcSamplerPointerTrait for RvmSerializedDb_ns_Dx12PcSamplerPointer {
}

pub static RVMSERIALIZEDDB_NS_DX12PCSAMPLERPOINTER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcSamplerPointer",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12PcSamplerPointer as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12PCSAMPLERPOINTER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx12PcSamplerPointer {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12PCSAMPLERPOINTER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12PcSamplerTableWriterInstructionData {
}

pub trait RvmSerializedDb_ns_Dx12PcSamplerTableWriterInstructionDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12PcSamplerTableWriterInstructionDataTrait for RvmSerializedDb_ns_Dx12PcSamplerTableWriterInstructionData {
}

pub static RVMSERIALIZEDDB_NS_DX12PCSAMPLERTABLEWRITERINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcSamplerTableWriterInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12PcSamplerTableWriterInstructionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12PCSAMPLERTABLEWRITERINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx12PcSamplerTableWriterInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12PCSAMPLERTABLEWRITERINSTRUCTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12PcRvmDescriptorTableAssemblyInstructionData {
}

pub trait RvmSerializedDb_ns_Dx12PcRvmDescriptorTableAssemblyInstructionDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12PcRvmDescriptorTableAssemblyInstructionDataTrait for RvmSerializedDb_ns_Dx12PcRvmDescriptorTableAssemblyInstructionData {
}

pub static RVMSERIALIZEDDB_NS_DX12PCRVMDESCRIPTORTABLEASSEMBLYINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcRvmDescriptorTableAssemblyInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12PcRvmDescriptorTableAssemblyInstructionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12PCRVMDESCRIPTORTABLEASSEMBLYINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx12PcRvmDescriptorTableAssemblyInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12PCRVMDESCRIPTORTABLEASSEMBLYINSTRUCTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12PcShaderDispatchDrawInstructionData {
}

pub trait RvmSerializedDb_ns_Dx12PcShaderDispatchDrawInstructionDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12PcShaderDispatchDrawInstructionDataTrait for RvmSerializedDb_ns_Dx12PcShaderDispatchDrawInstructionData {
}

pub static RVMSERIALIZEDDB_NS_DX12PCSHADERDISPATCHDRAWINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcShaderDispatchDrawInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12PcShaderDispatchDrawInstructionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12PCSHADERDISPATCHDRAWINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx12PcShaderDispatchDrawInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12PCSHADERDISPATCHDRAWINSTRUCTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12PcDispatchInstructionData {
}

pub trait RvmSerializedDb_ns_Dx12PcDispatchInstructionDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12PcDispatchInstructionDataTrait for RvmSerializedDb_ns_Dx12PcDispatchInstructionData {
}

pub static RVMSERIALIZEDDB_NS_DX12PCDISPATCHINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcDispatchInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12PcDispatchInstructionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12PCDISPATCHINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx12PcDispatchInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12PCDISPATCHINSTRUCTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct Dx12PcRvmSamplerTableWriterInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx12PcRvmSamplerTableWriterInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx12PcRvmSamplerTableWriterInstructionFactoryTrait for Dx12PcRvmSamplerTableWriterInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx12PcRvmSamplerTableWriterInstructionFactory {
}

pub static DX12PCRVMSAMPLERTABLEWRITERINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmSamplerTableWriterInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12PcRvmSamplerTableWriterInstructionFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12PCRVMSAMPLERTABLEWRITERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12PcRvmSamplerTableWriterInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX12PCRVMSAMPLERTABLEWRITERINSTRUCTIONFACTORY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DX12PCRVMSAMPLERTABLEWRITERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmSamplerTableWriterInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcRvmSamplerTableWriterInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Dx12PcRvmShaderTableRootDescriptorTableAssemblyInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx12PcRvmShaderTableRootDescriptorTableAssemblyInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx12PcRvmShaderTableRootDescriptorTableAssemblyInstructionFactoryTrait for Dx12PcRvmShaderTableRootDescriptorTableAssemblyInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx12PcRvmShaderTableRootDescriptorTableAssemblyInstructionFactory {
}

pub static DX12PCRVMSHADERTABLEROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmShaderTableRootDescriptorTableAssemblyInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12PcRvmShaderTableRootDescriptorTableAssemblyInstructionFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12PCRVMSHADERTABLEROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12PcRvmShaderTableRootDescriptorTableAssemblyInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX12PCRVMSHADERTABLEROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DX12PCRVMSHADERTABLEROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmShaderTableRootDescriptorTableAssemblyInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcRvmShaderTableRootDescriptorTableAssemblyInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Dx12PcRvmComputeRootDescriptorTableAssemblyInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx12PcRvmComputeRootDescriptorTableAssemblyInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx12PcRvmComputeRootDescriptorTableAssemblyInstructionFactoryTrait for Dx12PcRvmComputeRootDescriptorTableAssemblyInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx12PcRvmComputeRootDescriptorTableAssemblyInstructionFactory {
}

pub static DX12PCRVMCOMPUTEROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmComputeRootDescriptorTableAssemblyInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12PcRvmComputeRootDescriptorTableAssemblyInstructionFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12PCRVMCOMPUTEROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12PcRvmComputeRootDescriptorTableAssemblyInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX12PCRVMCOMPUTEROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DX12PCRVMCOMPUTEROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmComputeRootDescriptorTableAssemblyInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcRvmComputeRootDescriptorTableAssemblyInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Dx12PcRvmGraphicsRootDescriptorTableAssemblyInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx12PcRvmGraphicsRootDescriptorTableAssemblyInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx12PcRvmGraphicsRootDescriptorTableAssemblyInstructionFactoryTrait for Dx12PcRvmGraphicsRootDescriptorTableAssemblyInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx12PcRvmGraphicsRootDescriptorTableAssemblyInstructionFactory {
}

pub static DX12PCRVMGRAPHICSROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmGraphicsRootDescriptorTableAssemblyInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12PcRvmGraphicsRootDescriptorTableAssemblyInstructionFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12PCRVMGRAPHICSROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12PcRvmGraphicsRootDescriptorTableAssemblyInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX12PCRVMGRAPHICSROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DX12PCRVMGRAPHICSROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmGraphicsRootDescriptorTableAssemblyInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcRvmGraphicsRootDescriptorTableAssemblyInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Dx12PcRvmInstanceTableAssemblyInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx12PcRvmInstanceTableAssemblyInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx12PcRvmInstanceTableAssemblyInstructionFactoryTrait for Dx12PcRvmInstanceTableAssemblyInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx12PcRvmInstanceTableAssemblyInstructionFactory {
}

pub static DX12PCRVMINSTANCETABLEASSEMBLYINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmInstanceTableAssemblyInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12PcRvmInstanceTableAssemblyInstructionFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12PCRVMINSTANCETABLEASSEMBLYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12PcRvmInstanceTableAssemblyInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX12PCRVMINSTANCETABLEASSEMBLYINSTRUCTIONFACTORY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DX12PCRVMINSTANCETABLEASSEMBLYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmInstanceTableAssemblyInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcRvmInstanceTableAssemblyInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Dx12PcRvmDescriptorTableAssemblyInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx12PcRvmDescriptorTableAssemblyInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx12PcRvmDescriptorTableAssemblyInstructionFactoryTrait for Dx12PcRvmDescriptorTableAssemblyInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx12PcRvmDescriptorTableAssemblyInstructionFactory {
}

pub static DX12PCRVMDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmDescriptorTableAssemblyInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12PcRvmDescriptorTableAssemblyInstructionFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12PCRVMDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12PcRvmDescriptorTableAssemblyInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX12PCRVMDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DX12PCRVMDESCRIPTORTABLEASSEMBLYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmDescriptorTableAssemblyInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcRvmDescriptorTableAssemblyInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Dx12PcRvmTableAssemblyInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx12PcRvmTableAssemblyInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx12PcRvmTableAssemblyInstructionFactoryTrait for Dx12PcRvmTableAssemblyInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx12PcRvmTableAssemblyInstructionFactory {
}

pub static DX12PCRVMTABLEASSEMBLYINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmTableAssemblyInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12PcRvmTableAssemblyInstructionFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12PCRVMTABLEASSEMBLYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12PcRvmTableAssemblyInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX12PCRVMTABLEASSEMBLYINSTRUCTIONFACTORY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DX12PCRVMTABLEASSEMBLYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmTableAssemblyInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcRvmTableAssemblyInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Dx12PcRvmLegacyDrawStateBuilderInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx12PcRvmLegacyDrawStateBuilderInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx12PcRvmLegacyDrawStateBuilderInstructionFactoryTrait for Dx12PcRvmLegacyDrawStateBuilderInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx12PcRvmLegacyDrawStateBuilderInstructionFactory {
}

pub static DX12PCRVMLEGACYDRAWSTATEBUILDERINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmLegacyDrawStateBuilderInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12PcRvmLegacyDrawStateBuilderInstructionFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12PCRVMLEGACYDRAWSTATEBUILDERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12PcRvmLegacyDrawStateBuilderInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX12PCRVMLEGACYDRAWSTATEBUILDERINSTRUCTIONFACTORY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DX12PCRVMLEGACYDRAWSTATEBUILDERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmLegacyDrawStateBuilderInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcRvmLegacyDrawStateBuilderInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Dx12PcRvmShaderDispatchLegacyDrawInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx12PcRvmShaderDispatchLegacyDrawInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx12PcRvmShaderDispatchLegacyDrawInstructionFactoryTrait for Dx12PcRvmShaderDispatchLegacyDrawInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx12PcRvmShaderDispatchLegacyDrawInstructionFactory {
}

pub static DX12PCRVMSHADERDISPATCHLEGACYDRAWINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmShaderDispatchLegacyDrawInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12PcRvmShaderDispatchLegacyDrawInstructionFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12PCRVMSHADERDISPATCHLEGACYDRAWINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12PcRvmShaderDispatchLegacyDrawInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX12PCRVMSHADERDISPATCHLEGACYDRAWINSTRUCTIONFACTORY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DX12PCRVMSHADERDISPATCHLEGACYDRAWINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmShaderDispatchLegacyDrawInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcRvmShaderDispatchLegacyDrawInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Dx12PcRvmDispatchInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx12PcRvmDispatchInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx12PcRvmDispatchInstructionFactoryTrait for Dx12PcRvmDispatchInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx12PcRvmDispatchInstructionFactory {
}

pub static DX12PCRVMDISPATCHINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmDispatchInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx12Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx12PcRvmDispatchInstructionFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX12PCRVMDISPATCHINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12PcRvmDispatchInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX12PCRVMDISPATCHINSTRUCTIONFACTORY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DX12PCRVMDISPATCHINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRvmDispatchInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx12Pc",
    data: TypeInfoData::Array("Dx12PcRvmDispatchInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12PcPSOPreloadOp {
}

pub trait RvmSerializedDb_ns_Dx12PcPSOPreloadOpTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12PcPSOPreloadOpTrait for RvmSerializedDb_ns_Dx12PcPSOPreloadOp {
}

pub static RVMSERIALIZEDDB_NS_DX12PCPSOPRELOADOP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcPSOPreloadOp",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12PcPSOPreloadOp as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12PCPSOPRELOADOP_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx12PcPSOPreloadOp {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12PCPSOPRELOADOP_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12PcSampler {
}

pub trait RvmSerializedDb_ns_Dx12PcSamplerTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12PcSamplerTrait for RvmSerializedDb_ns_Dx12PcSampler {
}

pub static RVMSERIALIZEDDB_NS_DX12PCSAMPLER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcSampler",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12PcSampler as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12PCSAMPLER_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RvmSerializedDb_ns_Dx12PcSampler {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12PCSAMPLER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Dx12PcRootSignature {
}

pub trait RvmSerializedDb_ns_Dx12PcRootSignatureTrait: TypeObject {
}

impl RvmSerializedDb_ns_Dx12PcRootSignatureTrait for RvmSerializedDb_ns_Dx12PcRootSignature {
}

pub static RVMSERIALIZEDDB_NS_DX12PCROOTSIGNATURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcRootSignature",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx12Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Dx12PcRootSignature as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX12PCROOTSIGNATURE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx12PcRootSignature {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX12PCROOTSIGNATURE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

