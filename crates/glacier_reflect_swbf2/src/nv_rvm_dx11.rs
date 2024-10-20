use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx11NvDrawStateFTInstructionFactory {
}

pub const DX11NVDRAWSTATEFTINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11NvDrawStateFTInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "NvRvmDx11",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX11NVDRAWSTATEFTINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11NvDrawStateFTInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX11NVDRAWSTATEFTINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX11NVDRAWSTATEFTINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11NvDrawStateFTInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "NvRvmDx11",
    data: TypeInfoData::Array("Dx11NvDrawStateFTInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx11NvViewStateFTInstructionFactory {
}

pub const DX11NVVIEWSTATEFTINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11NvViewStateFTInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "NvRvmDx11",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX11NVVIEWSTATEFTINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11NvViewStateFTInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX11NVVIEWSTATEFTINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX11NVVIEWSTATEFTINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11NvViewStateFTInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "NvRvmDx11",
    data: TypeInfoData::Array("Dx11NvViewStateFTInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx11NvDrawStateDepthInstructionFactory {
}

pub const DX11NVDRAWSTATEDEPTHINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11NvDrawStateDepthInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "NvRvmDx11",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX11NVDRAWSTATEDEPTHINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11NvDrawStateDepthInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX11NVDRAWSTATEDEPTHINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX11NVDRAWSTATEDEPTHINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11NvDrawStateDepthInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "NvRvmDx11",
    data: TypeInfoData::Array("Dx11NvDrawStateDepthInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx11NvViewStateDepthInstructionFactory {
}

pub const DX11NVVIEWSTATEDEPTHINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11NvViewStateDepthInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "NvRvmDx11",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX11NVVIEWSTATEDEPTHINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11NvViewStateDepthInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX11NVVIEWSTATEDEPTHINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX11NVVIEWSTATEDEPTHINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11NvViewStateDepthInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "NvRvmDx11",
    data: TypeInfoData::Array("Dx11NvViewStateDepthInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx11NvViewStateDepthInstructionData {
}

pub const RVMSERIALIZEDDB_NS_DX11NVVIEWSTATEDEPTHINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11NvViewStateDepthInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "NvRvmDx11",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX11NVVIEWSTATEDEPTHINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx11NvViewStateDepthInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11NVVIEWSTATEDEPTHINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx11NvDrawStateInstructionData {
}

pub const RVMSERIALIZEDDB_NS_DX11NVDRAWSTATEINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11NvDrawStateInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "NvRvmDx11",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX11NVDRAWSTATEINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx11NvDrawStateInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11NVDRAWSTATEINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx11NvViewStateInstructionData {
}

pub const RVMSERIALIZEDDB_NS_DX11NVVIEWSTATEINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11NvViewStateInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "NvRvmDx11",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX11NVVIEWSTATEINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx11NvViewStateInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11NVVIEWSTATEINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx11NvRvmBackendConfig {
}

pub const DX11NVRVMBACKENDCONFIG_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11NvRvmBackendConfig",
    flags: MemberInfoFlags::new(101),
    module: "NvRvmDx11",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DX11RVMBACKENDCONFIG_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX11NVRVMBACKENDCONFIG_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11NvRvmBackendConfig {
    fn type_info() -> &'static TypeInfo {
        DX11NVRVMBACKENDCONFIG_TYPE_INFO
    }
}


pub const DX11NVRVMBACKENDCONFIG_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11NvRvmBackendConfig-Array",
    flags: MemberInfoFlags::new(145),
    module: "NvRvmDx11",
    data: TypeInfoData::Array("Dx11NvRvmBackendConfig-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx11NvRvmDatabaseLoader {
}

pub const DX11NVRVMDATABASELOADER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11NvRvmDatabaseLoader",
    flags: MemberInfoFlags::new(101),
    module: "NvRvmDx11",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMCOMMONDATABASELOADER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX11NVRVMDATABASELOADER_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for Dx11NvRvmDatabaseLoader {
    fn type_info() -> &'static TypeInfo {
        DX11NVRVMDATABASELOADER_TYPE_INFO
    }
}


pub const DX11NVRVMDATABASELOADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11NvRvmDatabaseLoader-Array",
    flags: MemberInfoFlags::new(145),
    module: "NvRvmDx11",
    data: TypeInfoData::Array("Dx11NvRvmDatabaseLoader-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx11NvRvmDatabase {
}

pub const DX11NVRVMDATABASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11NvRvmDatabase",
    flags: MemberInfoFlags::new(101),
    module: "NvRvmDx11",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DX11RVMDATABASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX11NVRVMDATABASE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11NvRvmDatabase {
    fn type_info() -> &'static TypeInfo {
        DX11NVRVMDATABASE_TYPE_INFO
    }
}


pub const DX11NVRVMDATABASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11NvRvmDatabase-Array",
    flags: MemberInfoFlags::new(145),
    module: "NvRvmDx11",
    data: TypeInfoData::Array("Dx11NvRvmDatabase-Array"),
    array_type: None,
    alignment: 8,
};


