use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_rvm_common_types(registry: &mut TypeRegistry) {
    registry.register_type(RVMBUILDSETTINGS_TYPE_INFO);
    registry.register_type(RVMBUILDSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(RVMBACKENDCONFIG_TYPE_INFO);
    registry.register_type(RVMBACKENDCONFIG_ARRAY_TYPE_INFO);
    registry.register_type(LODFADEINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(LODFADEINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_LODFADEINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(PACKLIGHTMAPWEIGHTINTOINSTANCEINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(PACKLIGHTMAPWEIGHTINTOINSTANCEINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(SLICECOUNTINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(SLICECOUNTINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(TESSELLATIONPARAMETERSINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(TESSELLATIONPARAMETERSINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(VECTORSUBTRACTINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(VECTORSUBTRACTINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(OFFSETTRANSLATIONINMATRIXINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(OFFSETTRANSLATIONINMATRIXINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(CPUTOGPUMATRIXINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(CPUTOGPUMATRIXINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(FLOATTOVECINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(FLOATTOVECINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_PACKLIGHTMAPWEIGHTINTOINSTANCEINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_SLICECOUNTINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_TESSELLATIONPARAMETERSINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_VECTORSUBTRACTINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_OFFSETTRANSLATIONINMATRIXINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_CPUTOGPUMATRIXINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_FLOATTOVECINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_LEGACYVERTEXBUFFERCONVERSIONINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_VERTEXSTREAMPROCESSINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_VIEWSTATEINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(RVMCOMMONSETTINGS_TYPE_INFO);
    registry.register_type(RVMCOMMONSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(RVMPOINTER_TYPE_INFO);
    registry.register_type(GPUMAT4X3_TYPE_INFO);
    registry.register_type(IVEC4_TYPE_INFO);
    registry.register_type(IVEC3_TYPE_INFO);
    registry.register_type(IVEC2_TYPE_INFO);
    registry.register_type(HALF4_TYPE_INFO);
    registry.register_type(HALF3_TYPE_INFO);
    registry.register_type(HALF2_TYPE_INFO);
    registry.register_type(HALF_TYPE_INFO);
    registry.register_type(RVMLEGACYINSTRUCTIONS_NS_LEGACYINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(RVMLEGACYINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(RVMLEGACYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO);
    registry.register_type(RVMINSTRUCTIONFACTORYBASE_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_INSTANCETABLEASSEMBLYINSTRUCTIONBATCHDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_INSTANCETABLEASSEMBLYDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DEPTHBIASGROUPDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_TABLEASSEMBLYINSTRUCTIONBATCHDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_TABLEASSEMBLYDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_WRITEOPGROUP_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_WRITEOP_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DRAWSTATEBUILDERINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DISPATCHINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DIRECTINPUTINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_CONSTANTVALUEINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(CHAR_TYPE_INFO);
    registry.register_type(PARAMDBHASH_TYPE_INFO);
    registry.register_type(RVMENCODEDTABLEENTRY_TYPE_INFO);
    registry.register_type(RVMCONTEXTSORTKEYINFO_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_PREPAREDVERTEXELEMENT_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_PREPAREDVERTEXSTREAM_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_SERIALIZEDPARAMDBKEY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_BASESHADERSTATE_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_RUNTIMEINSTANTIATEDTYPE_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDMULTIHASHVIEW_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDREADVIEW_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDHASHVIEWREF_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDHASHVIEW_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDFILTERVIEW_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_COMBINEDSERIALIZEDPARAMETERBLOCK_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_SERIALIZEDPARAMETERBLOCKREF_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_SERIALIZEDPARAMETERBLOCK_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_SHADERSTREAMABLEEXTERNALTEXTUREREF_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_SHADERSTREAMABLETEXTUREREF_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_SURFACESHADERDEBUGINFO_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_SURFACESHADER_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_SHADERSTREAMABLETEXTURE_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_SHADERSTREAMABLEEXTERNALTEXTURE_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_RVMFUNCTIONINSTANCEREF_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_RVMFUNCTIONINSTANCE_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_SETTINGS_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_RVMPERMUTATIONSET_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_RVMPERMUTATIONLOOKUPTABLE_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_RVMPERMUTATIONTREE_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_RVMPERMUTATIONREF_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_RVMPERMUTATION_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_RVMFUNCTIONINPUTTABLEINDICES_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_RVMDISPATCHDEBUGINFO_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_RVMFUNCTIONDEBUGINFO_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_RVMDISPATCH_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_RVMFUNCTION_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_INSTRUCTIONBATCH_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_RTTITYPE_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DEFAULTVALUEREF_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_VALUEREF_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DEFAULTVALUEZEROMEM_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DEFAULTVALUESTRUCTLEGACYDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DEFAULTVALUESIMPLETEXTURE_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DEFAULTVALUESIMPLEBUFFER_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_INSTRUCTIONBATCHREF_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_PARAMDBKEYREF_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_PARAMDBKEYDESC_TYPE_INFO);
    registry.register_type(RVMDIRECTINPUTINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(RVMDIRECTINPUTINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(RVMCONSTANTVALUEINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(RVMCONSTANTVALUEINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(RVMCOMMONDATABASELOADERIMPL_TYPE_INFO);
    registry.register_type(RVMCOMMONDATABASELOADERIMPL_ARRAY_TYPE_INFO);
    registry.register_type(RVMCOMMONDATABASELOADER_TYPE_INFO);
    registry.register_type(RVMCOMMONDATABASELOADER_ARRAY_TYPE_INFO);
    registry.register_type(COMMONRVMBACKEND_TYPE_INFO);
    registry.register_type(COMMONRVMBACKEND_ARRAY_TYPE_INFO);
    registry.register_type(BASERVMDATABASE_TYPE_INFO);
    registry.register_type(BASERVMDATABASE_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmBuildSettings {
    pub built_backends: Vec<RvmBackendConfig>,
}

pub const RVMBUILDSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmBuildSettings",
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "BuiltBackends",
                flags: MemberInfoFlags::new(144),
                field_type: RVMBACKENDCONFIG_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(RvmBuildSettings, built_backends),
            },
        ],
    }),
    array_type: Some(RVMBUILDSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmBuildSettings {
    fn type_info() -> &'static TypeInfo {
        RVMBUILDSETTINGS_TYPE_INFO
    }
}


pub const RVMBUILDSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmBuildSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("RvmBuildSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmBackendConfig {
}

pub const RVMBACKENDCONFIG_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmBackendConfig",
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(RVMBACKENDCONFIG_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmBackendConfig {
    fn type_info() -> &'static TypeInfo {
        RVMBACKENDCONFIG_TYPE_INFO
    }
}


pub const RVMBACKENDCONFIG_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmBackendConfig-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("RvmBackendConfig-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LodFadeInstructionFactory {
}

pub const LODFADEINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LodFadeInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LODFADEINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LodFadeInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        LODFADEINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const LODFADEINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LodFadeInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("LodFadeInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_LodFadeInstructionData {
}

pub const RVMSERIALIZEDDB_NS_LODFADEINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_LodFadeInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_LODFADEINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_LodFadeInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_LODFADEINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PackLightMapWeightIntoInstanceInstructionFactory {
}

pub const PACKLIGHTMAPWEIGHTINTOINSTANCEINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PackLightMapWeightIntoInstanceInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PACKLIGHTMAPWEIGHTINTOINSTANCEINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PackLightMapWeightIntoInstanceInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        PACKLIGHTMAPWEIGHTINTOINSTANCEINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const PACKLIGHTMAPWEIGHTINTOINSTANCEINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PackLightMapWeightIntoInstanceInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("PackLightMapWeightIntoInstanceInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SliceCountInstructionFactory {
}

pub const SLICECOUNTINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SliceCountInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SLICECOUNTINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SliceCountInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        SLICECOUNTINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const SLICECOUNTINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SliceCountInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("SliceCountInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TessellationParametersInstructionFactory {
}

pub const TESSELLATIONPARAMETERSINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TessellationParametersInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TESSELLATIONPARAMETERSINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TessellationParametersInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        TESSELLATIONPARAMETERSINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const TESSELLATIONPARAMETERSINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TessellationParametersInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("TessellationParametersInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VectorSubtractInstructionFactory {
}

pub const VECTORSUBTRACTINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VectorSubtractInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VECTORSUBTRACTINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VectorSubtractInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        VECTORSUBTRACTINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const VECTORSUBTRACTINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VectorSubtractInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("VectorSubtractInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OffsetTranslationInMatrixInstructionFactory {
}

pub const OFFSETTRANSLATIONINMATRIXINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OffsetTranslationInMatrixInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(OFFSETTRANSLATIONINMATRIXINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OffsetTranslationInMatrixInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        OFFSETTRANSLATIONINMATRIXINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const OFFSETTRANSLATIONINMATRIXINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OffsetTranslationInMatrixInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("OffsetTranslationInMatrixInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CpuToGpuMatrixInstructionFactory {
}

pub const CPUTOGPUMATRIXINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CpuToGpuMatrixInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CPUTOGPUMATRIXINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CpuToGpuMatrixInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        CPUTOGPUMATRIXINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const CPUTOGPUMATRIXINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CpuToGpuMatrixInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("CpuToGpuMatrixInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FloatToVecInstructionFactory {
}

pub const FLOATTOVECINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatToVecInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FLOATTOVECINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FloatToVecInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        FLOATTOVECINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const FLOATTOVECINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatToVecInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("FloatToVecInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_PackLightMapWeightIntoInstanceInstructionData {
}

pub const RVMSERIALIZEDDB_NS_PACKLIGHTMAPWEIGHTINTOINSTANCEINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_PackLightMapWeightIntoInstanceInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_PACKLIGHTMAPWEIGHTINTOINSTANCEINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_PackLightMapWeightIntoInstanceInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_PACKLIGHTMAPWEIGHTINTOINSTANCEINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_SliceCountInstructionData {
}

pub const RVMSERIALIZEDDB_NS_SLICECOUNTINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_SliceCountInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_SLICECOUNTINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_SliceCountInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_SLICECOUNTINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_TessellationParametersInstructionData {
}

pub const RVMSERIALIZEDDB_NS_TESSELLATIONPARAMETERSINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_TessellationParametersInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_TESSELLATIONPARAMETERSINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_TessellationParametersInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_TESSELLATIONPARAMETERSINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_VectorSubtractInstructionData {
}

pub const RVMSERIALIZEDDB_NS_VECTORSUBTRACTINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_VectorSubtractInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_VECTORSUBTRACTINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_VectorSubtractInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_VECTORSUBTRACTINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_OffsetTranslationInMatrixInstructionData {
}

pub const RVMSERIALIZEDDB_NS_OFFSETTRANSLATIONINMATRIXINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_OffsetTranslationInMatrixInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_OFFSETTRANSLATIONINMATRIXINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_OffsetTranslationInMatrixInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_OFFSETTRANSLATIONINMATRIXINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_CpuToGpuMatrixInstructionData {
}

pub const RVMSERIALIZEDDB_NS_CPUTOGPUMATRIXINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_CpuToGpuMatrixInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_CPUTOGPUMATRIXINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_CpuToGpuMatrixInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_CPUTOGPUMATRIXINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_FloatToVecInstructionData {
}

pub const RVMSERIALIZEDDB_NS_FLOATTOVECINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_FloatToVecInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_FLOATTOVECINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_FloatToVecInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_FLOATTOVECINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_LegacyVertexBufferConversionInstructionData {
}

pub const RVMSERIALIZEDDB_NS_LEGACYVERTEXBUFFERCONVERSIONINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_LegacyVertexBufferConversionInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_LEGACYVERTEXBUFFERCONVERSIONINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_LegacyVertexBufferConversionInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_LEGACYVERTEXBUFFERCONVERSIONINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_VertexStreamProcessInstructionData {
}

pub const RVMSERIALIZEDDB_NS_VERTEXSTREAMPROCESSINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_VertexStreamProcessInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_VERTEXSTREAMPROCESSINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_VertexStreamProcessInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_VERTEXSTREAMPROCESSINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_ViewStateInstructionData {
}

pub const RVMSERIALIZEDDB_NS_VIEWSTATEINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ViewStateInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_VIEWSTATEINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_ViewStateInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_VIEWSTATEINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmCommonSettings {
    pub on_demand_building_enable: bool,
}

pub const RVMCOMMONSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmCommonSettings",
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "OnDemandBuildingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RvmCommonSettings, on_demand_building_enable),
            },
        ],
    }),
    array_type: Some(RVMCOMMONSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmCommonSettings {
    fn type_info() -> &'static TypeInfo {
        RVMCOMMONSETTINGS_TYPE_INFO
    }
}


pub const RVMCOMMONSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmCommonSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("RvmCommonSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmPointer {
}

pub const RVMPOINTER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmPointer",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMPOINTER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmPointer {
    fn type_info() -> &'static TypeInfo {
        RVMPOINTER_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GpuMat4x3 {
}

pub const GPUMAT4X3_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GpuMat4x3",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(GPUMAT4X3_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for GpuMat4x3 {
    fn type_info() -> &'static TypeInfo {
        GPUMAT4X3_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IVec4 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
}

pub const IVEC4_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IVec4",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "x",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(IVec4, x),
            },
            FieldInfoData {
                name: "y",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(IVec4, y),
            },
            FieldInfoData {
                name: "z",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(IVec4, z),
            },
            FieldInfoData {
                name: "w",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(IVec4, w),
            },
        ],
    }),
    array_type: Some(IVEC4_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for IVec4 {
    fn type_info() -> &'static TypeInfo {
        IVEC4_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IVec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

pub const IVEC3_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IVec3",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "x",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(IVec3, x),
            },
            FieldInfoData {
                name: "y",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(IVec3, y),
            },
            FieldInfoData {
                name: "z",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(IVec3, z),
            },
        ],
    }),
    array_type: Some(IVEC3_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for IVec3 {
    fn type_info() -> &'static TypeInfo {
        IVEC3_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IVec2 {
    pub x: i32,
    pub y: i32,
}

pub const IVEC2_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IVec2",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "x",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(IVec2, x),
            },
            FieldInfoData {
                name: "y",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(IVec2, y),
            },
        ],
    }),
    array_type: Some(IVEC2_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for IVec2 {
    fn type_info() -> &'static TypeInfo {
        IVEC2_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Half4 {
    pub x: Half,
    pub y: Half,
    pub z: Half,
    pub w: Half,
}

pub const HALF4_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Half4",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "x",
                flags: MemberInfoFlags::new(0),
                field_type: HALF_TYPE_INFO,
                rust_offset: offset_of!(Half4, x),
            },
            FieldInfoData {
                name: "y",
                flags: MemberInfoFlags::new(0),
                field_type: HALF_TYPE_INFO,
                rust_offset: offset_of!(Half4, y),
            },
            FieldInfoData {
                name: "z",
                flags: MemberInfoFlags::new(0),
                field_type: HALF_TYPE_INFO,
                rust_offset: offset_of!(Half4, z),
            },
            FieldInfoData {
                name: "w",
                flags: MemberInfoFlags::new(0),
                field_type: HALF_TYPE_INFO,
                rust_offset: offset_of!(Half4, w),
            },
        ],
    }),
    array_type: Some(HALF4_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for Half4 {
    fn type_info() -> &'static TypeInfo {
        HALF4_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Half3 {
    pub x: Half,
    pub y: Half,
    pub z: Half,
}

pub const HALF3_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Half3",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "x",
                flags: MemberInfoFlags::new(0),
                field_type: HALF_TYPE_INFO,
                rust_offset: offset_of!(Half3, x),
            },
            FieldInfoData {
                name: "y",
                flags: MemberInfoFlags::new(0),
                field_type: HALF_TYPE_INFO,
                rust_offset: offset_of!(Half3, y),
            },
            FieldInfoData {
                name: "z",
                flags: MemberInfoFlags::new(0),
                field_type: HALF_TYPE_INFO,
                rust_offset: offset_of!(Half3, z),
            },
        ],
    }),
    array_type: Some(HALF3_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for Half3 {
    fn type_info() -> &'static TypeInfo {
        HALF3_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Half2 {
    pub x: Half,
    pub y: Half,
}

pub const HALF2_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Half2",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "x",
                flags: MemberInfoFlags::new(0),
                field_type: HALF_TYPE_INFO,
                rust_offset: offset_of!(Half2, x),
            },
            FieldInfoData {
                name: "y",
                flags: MemberInfoFlags::new(0),
                field_type: HALF_TYPE_INFO,
                rust_offset: offset_of!(Half2, y),
            },
        ],
    }),
    array_type: Some(HALF2_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for Half2 {
    fn type_info() -> &'static TypeInfo {
        HALF2_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Half {
}

pub const HALF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Half",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(HALF_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for Half {
    fn type_info() -> &'static TypeInfo {
        HALF_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmLegacyInstructions_ns_LegacyInstructionData {
}

pub const RVMLEGACYINSTRUCTIONS_NS_LEGACYINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmLegacyInstructions_ns_LegacyInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMLEGACYINSTRUCTIONS_NS_LEGACYINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmLegacyInstructions_ns_LegacyInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMLEGACYINSTRUCTIONS_NS_LEGACYINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmLegacyInstructionFactory {
}

pub const RVMLEGACYINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmLegacyInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(RVMLEGACYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmLegacyInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        RVMLEGACYINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const RVMLEGACYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmLegacyInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("RvmLegacyInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmInstructionFactoryBase {
}

pub const RVMINSTRUCTIONFACTORYBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmInstructionFactoryBase",
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(RVMINSTRUCTIONFACTORYBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RvmInstructionFactoryBase {
    fn type_info() -> &'static TypeInfo {
        RVMINSTRUCTIONFACTORYBASE_TYPE_INFO
    }
}


pub const RVMINSTRUCTIONFACTORYBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmInstructionFactoryBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("RvmInstructionFactoryBase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_InstanceTableAssemblyInstructionBatchData {
}

pub const RVMSERIALIZEDDB_NS_INSTANCETABLEASSEMBLYINSTRUCTIONBATCHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_InstanceTableAssemblyInstructionBatchData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_INSTANCETABLEASSEMBLYINSTRUCTIONBATCHDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_InstanceTableAssemblyInstructionBatchData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_INSTANCETABLEASSEMBLYINSTRUCTIONBATCHDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_InstanceTableAssemblyData {
}

pub const RVMSERIALIZEDDB_NS_INSTANCETABLEASSEMBLYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_InstanceTableAssemblyData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_INSTANCETABLEASSEMBLYDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RvmSerializedDb_ns_InstanceTableAssemblyData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_INSTANCETABLEASSEMBLYDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_DepthBiasGroupData {
}

pub const RVMSERIALIZEDDB_NS_DEPTHBIASGROUPDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DepthBiasGroupData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DEPTHBIASGROUPDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RvmSerializedDb_ns_DepthBiasGroupData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DEPTHBIASGROUPDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_TableAssemblyInstructionBatchData {
}

pub const RVMSERIALIZEDDB_NS_TABLEASSEMBLYINSTRUCTIONBATCHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_TableAssemblyInstructionBatchData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_TABLEASSEMBLYINSTRUCTIONBATCHDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_TableAssemblyInstructionBatchData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_TABLEASSEMBLYINSTRUCTIONBATCHDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_TableAssemblyData {
}

pub const RVMSERIALIZEDDB_NS_TABLEASSEMBLYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_TableAssemblyData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_TABLEASSEMBLYDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RvmSerializedDb_ns_TableAssemblyData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_TABLEASSEMBLYDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_WriteOpGroup {
}

pub const RVMSERIALIZEDDB_NS_WRITEOPGROUP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_WriteOpGroup",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_WRITEOPGROUP_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RvmSerializedDb_ns_WriteOpGroup {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_WRITEOPGROUP_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_WriteOp {
}

pub const RVMSERIALIZEDDB_NS_WRITEOP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_WriteOp",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_WRITEOP_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_WriteOp {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_WRITEOP_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_DrawStateBuilderInstructionData {
}

pub const RVMSERIALIZEDDB_NS_DRAWSTATEBUILDERINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DrawStateBuilderInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DRAWSTATEBUILDERINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_DrawStateBuilderInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DRAWSTATEBUILDERINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_DispatchInstructionData {
}

pub const RVMSERIALIZEDDB_NS_DISPATCHINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DispatchInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DISPATCHINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_DispatchInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DISPATCHINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_DirectInputInstructionData {
}

pub const RVMSERIALIZEDDB_NS_DIRECTINPUTINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DirectInputInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DIRECTINPUTINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_DirectInputInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DIRECTINPUTINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_ConstantValueInstructionData {
}

pub const RVMSERIALIZEDDB_NS_CONSTANTVALUEINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ConstantValueInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_CONSTANTVALUEINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_ConstantValueInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_CONSTANTVALUEINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct char {
}

pub const CHAR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "char",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(CHAR_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for char {
    fn type_info() -> &'static TypeInfo {
        CHAR_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ParamDbHash {
}

pub const PARAMDBHASH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ParamDbHash",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(PARAMDBHASH_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ParamDbHash {
    fn type_info() -> &'static TypeInfo {
        PARAMDBHASH_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmEncodedTableEntry {
}

pub const RVMENCODEDTABLEENTRY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmEncodedTableEntry",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMENCODEDTABLEENTRY_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmEncodedTableEntry {
    fn type_info() -> &'static TypeInfo {
        RVMENCODEDTABLEENTRY_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmContextSortKeyInfo {
}

pub const RVMCONTEXTSORTKEYINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmContextSortKeyInfo",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMCONTEXTSORTKEYINFO_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmContextSortKeyInfo {
    fn type_info() -> &'static TypeInfo {
        RVMCONTEXTSORTKEYINFO_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_PreparedVertexElement {
}

pub const RVMSERIALIZEDDB_NS_PREPAREDVERTEXELEMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_PreparedVertexElement",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_PREPAREDVERTEXELEMENT_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_PreparedVertexElement {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_PREPAREDVERTEXELEMENT_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_PreparedVertexStream {
}

pub const RVMSERIALIZEDDB_NS_PREPAREDVERTEXSTREAM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_PreparedVertexStream",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_PREPAREDVERTEXSTREAM_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_PreparedVertexStream {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_PREPAREDVERTEXSTREAM_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_SerializedParamDbKey {
}

pub const RVMSERIALIZEDDB_NS_SERIALIZEDPARAMDBKEY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_SerializedParamDbKey",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_SERIALIZEDPARAMDBKEY_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RvmSerializedDb_ns_SerializedParamDbKey {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_SERIALIZEDPARAMDBKEY_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_BaseShaderState {
}

pub const RVMSERIALIZEDDB_NS_BASESHADERSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_BaseShaderState",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_BASESHADERSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RvmSerializedDb_ns_BaseShaderState {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_BASESHADERSTATE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_RuntimeInstantiatedType {
}

pub const RVMSERIALIZEDDB_NS_RUNTIMEINSTANTIATEDTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RuntimeInstantiatedType",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_RUNTIMEINSTANTIATEDTYPE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_RuntimeInstantiatedType {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RUNTIMEINSTANTIATEDTYPE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_ParamDbSerializedMultiHashView {
}

pub const RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDMULTIHASHVIEW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbSerializedMultiHashView",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDMULTIHASHVIEW_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_ParamDbSerializedMultiHashView {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDMULTIHASHVIEW_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_ParamDbSerializedReadView {
}

pub const RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDREADVIEW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbSerializedReadView",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDREADVIEW_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_ParamDbSerializedReadView {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDREADVIEW_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_ParamDbSerializedHashViewRef {
}

pub const RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDHASHVIEWREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbSerializedHashViewRef",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDHASHVIEWREF_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_ParamDbSerializedHashViewRef {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDHASHVIEWREF_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_ParamDbSerializedHashView {
}

pub const RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDHASHVIEW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbSerializedHashView",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDHASHVIEW_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_ParamDbSerializedHashView {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDHASHVIEW_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_ParamDbSerializedFilterView {
}

pub const RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDFILTERVIEW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbSerializedFilterView",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDFILTERVIEW_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_ParamDbSerializedFilterView {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDFILTERVIEW_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_CombinedSerializedParameterBlock {
}

pub const RVMSERIALIZEDDB_NS_COMBINEDSERIALIZEDPARAMETERBLOCK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_CombinedSerializedParameterBlock",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_COMBINEDSERIALIZEDPARAMETERBLOCK_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_CombinedSerializedParameterBlock {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_COMBINEDSERIALIZEDPARAMETERBLOCK_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_SerializedParameterBlockRef {
}

pub const RVMSERIALIZEDDB_NS_SERIALIZEDPARAMETERBLOCKREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_SerializedParameterBlockRef",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_SERIALIZEDPARAMETERBLOCKREF_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_SerializedParameterBlockRef {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_SERIALIZEDPARAMETERBLOCKREF_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_SerializedParameterBlock {
}

pub const RVMSERIALIZEDDB_NS_SERIALIZEDPARAMETERBLOCK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_SerializedParameterBlock",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_SERIALIZEDPARAMETERBLOCK_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_SerializedParameterBlock {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_SERIALIZEDPARAMETERBLOCK_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_ShaderStreamableExternalTextureRef {
}

pub const RVMSERIALIZEDDB_NS_SHADERSTREAMABLEEXTERNALTEXTUREREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ShaderStreamableExternalTextureRef",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_SHADERSTREAMABLEEXTERNALTEXTUREREF_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_ShaderStreamableExternalTextureRef {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_SHADERSTREAMABLEEXTERNALTEXTUREREF_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_ShaderStreamableTextureRef {
}

pub const RVMSERIALIZEDDB_NS_SHADERSTREAMABLETEXTUREREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ShaderStreamableTextureRef",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_SHADERSTREAMABLETEXTUREREF_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_ShaderStreamableTextureRef {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_SHADERSTREAMABLETEXTUREREF_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_SurfaceShaderDebugInfo {
}

pub const RVMSERIALIZEDDB_NS_SURFACESHADERDEBUGINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_SurfaceShaderDebugInfo",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_SURFACESHADERDEBUGINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_SurfaceShaderDebugInfo {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_SURFACESHADERDEBUGINFO_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_SurfaceShader {
}

pub const RVMSERIALIZEDDB_NS_SURFACESHADER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_SurfaceShader",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_SURFACESHADER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_SurfaceShader {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_SURFACESHADER_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_ShaderStreamableTexture {
}

pub const RVMSERIALIZEDDB_NS_SHADERSTREAMABLETEXTURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ShaderStreamableTexture",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_SHADERSTREAMABLETEXTURE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RvmSerializedDb_ns_ShaderStreamableTexture {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_SHADERSTREAMABLETEXTURE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_ShaderStreamableExternalTexture {
}

pub const RVMSERIALIZEDDB_NS_SHADERSTREAMABLEEXTERNALTEXTURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ShaderStreamableExternalTexture",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_SHADERSTREAMABLEEXTERNALTEXTURE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RvmSerializedDb_ns_ShaderStreamableExternalTexture {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_SHADERSTREAMABLEEXTERNALTEXTURE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_RvmFunctionInstanceRef {
}

pub const RVMSERIALIZEDDB_NS_RVMFUNCTIONINSTANCEREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmFunctionInstanceRef",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_RVMFUNCTIONINSTANCEREF_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_RvmFunctionInstanceRef {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RVMFUNCTIONINSTANCEREF_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_RvmFunctionInstance {
}

pub const RVMSERIALIZEDDB_NS_RVMFUNCTIONINSTANCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmFunctionInstance",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_RVMFUNCTIONINSTANCE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_RvmFunctionInstance {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RVMFUNCTIONINSTANCE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Settings {
}

pub const RVMSERIALIZEDDB_NS_SETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Settings",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_SETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Settings {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_SETTINGS_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_RvmPermutationSet {
}

pub const RVMSERIALIZEDDB_NS_RVMPERMUTATIONSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmPermutationSet",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_RVMPERMUTATIONSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_RvmPermutationSet {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RVMPERMUTATIONSET_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_RvmPermutationLookupTable {
}

pub const RVMSERIALIZEDDB_NS_RVMPERMUTATIONLOOKUPTABLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmPermutationLookupTable",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_RVMPERMUTATIONLOOKUPTABLE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_RvmPermutationLookupTable {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RVMPERMUTATIONLOOKUPTABLE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_RvmPermutationTree {
}

pub const RVMSERIALIZEDDB_NS_RVMPERMUTATIONTREE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmPermutationTree",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_RVMPERMUTATIONTREE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_RvmPermutationTree {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RVMPERMUTATIONTREE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_RvmPermutationRef {
}

pub const RVMSERIALIZEDDB_NS_RVMPERMUTATIONREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmPermutationRef",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_RVMPERMUTATIONREF_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_RvmPermutationRef {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RVMPERMUTATIONREF_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_RvmPermutation {
}

pub const RVMSERIALIZEDDB_NS_RVMPERMUTATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmPermutation",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_RVMPERMUTATION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_RvmPermutation {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RVMPERMUTATION_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_RvmFunctionInputTableIndices {
}

pub const RVMSERIALIZEDDB_NS_RVMFUNCTIONINPUTTABLEINDICES_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmFunctionInputTableIndices",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_RVMFUNCTIONINPUTTABLEINDICES_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_RvmFunctionInputTableIndices {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RVMFUNCTIONINPUTTABLEINDICES_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_RvmDispatchDebugInfo {
}

pub const RVMSERIALIZEDDB_NS_RVMDISPATCHDEBUGINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmDispatchDebugInfo",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_RVMDISPATCHDEBUGINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_RvmDispatchDebugInfo {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RVMDISPATCHDEBUGINFO_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_RvmFunctionDebugInfo {
}

pub const RVMSERIALIZEDDB_NS_RVMFUNCTIONDEBUGINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmFunctionDebugInfo",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_RVMFUNCTIONDEBUGINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_RvmFunctionDebugInfo {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RVMFUNCTIONDEBUGINFO_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_RvmDispatch {
}

pub const RVMSERIALIZEDDB_NS_RVMDISPATCH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmDispatch",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_RVMDISPATCH_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_RvmDispatch {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RVMDISPATCH_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_RvmFunction {
}

pub const RVMSERIALIZEDDB_NS_RVMFUNCTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmFunction",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_RVMFUNCTION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_RvmFunction {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RVMFUNCTION_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_InstructionBatch {
}

pub const RVMSERIALIZEDDB_NS_INSTRUCTIONBATCH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_InstructionBatch",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_INSTRUCTIONBATCH_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_InstructionBatch {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_INSTRUCTIONBATCH_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_RttiType {
}

pub const RVMSERIALIZEDDB_NS_RTTITYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RttiType",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_RTTITYPE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_RttiType {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RTTITYPE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_DefaultValueRef {
}

pub const RVMSERIALIZEDDB_NS_DEFAULTVALUEREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DefaultValueRef",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DEFAULTVALUEREF_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_DefaultValueRef {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DEFAULTVALUEREF_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_ValueRef {
}

pub const RVMSERIALIZEDDB_NS_VALUEREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ValueRef",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_VALUEREF_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_ValueRef {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_VALUEREF_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_DefaultValueZeroMem {
}

pub const RVMSERIALIZEDDB_NS_DEFAULTVALUEZEROMEM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DefaultValueZeroMem",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DEFAULTVALUEZEROMEM_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_DefaultValueZeroMem {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DEFAULTVALUEZEROMEM_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_DefaultValueStructLegacyData {
}

pub const RVMSERIALIZEDDB_NS_DEFAULTVALUESTRUCTLEGACYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DefaultValueStructLegacyData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DEFAULTVALUESTRUCTLEGACYDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RvmSerializedDb_ns_DefaultValueStructLegacyData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DEFAULTVALUESTRUCTLEGACYDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_DefaultValueSimpleTexture {
}

pub const RVMSERIALIZEDDB_NS_DEFAULTVALUESIMPLETEXTURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DefaultValueSimpleTexture",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DEFAULTVALUESIMPLETEXTURE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_DefaultValueSimpleTexture {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DEFAULTVALUESIMPLETEXTURE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_DefaultValueSimpleBuffer {
}

pub const RVMSERIALIZEDDB_NS_DEFAULTVALUESIMPLEBUFFER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DefaultValueSimpleBuffer",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DEFAULTVALUESIMPLEBUFFER_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_DefaultValueSimpleBuffer {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DEFAULTVALUESIMPLEBUFFER_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_InstructionBatchRef {
}

pub const RVMSERIALIZEDDB_NS_INSTRUCTIONBATCHREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_InstructionBatchRef",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_INSTRUCTIONBATCHREF_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_InstructionBatchRef {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_INSTRUCTIONBATCHREF_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_ParamDbKeyRef {
}

pub const RVMSERIALIZEDDB_NS_PARAMDBKEYREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbKeyRef",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_PARAMDBKEYREF_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_ParamDbKeyRef {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_PARAMDBKEYREF_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_ParamDbKeyDesc {
}

pub const RVMSERIALIZEDDB_NS_PARAMDBKEYDESC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbKeyDesc",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_PARAMDBKEYDESC_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_ParamDbKeyDesc {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_PARAMDBKEYDESC_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmDirectInputInstructionFactory {
}

pub const RVMDIRECTINPUTINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmDirectInputInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(RVMDIRECTINPUTINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmDirectInputInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        RVMDIRECTINPUTINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const RVMDIRECTINPUTINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmDirectInputInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("RvmDirectInputInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmConstantValueInstructionFactory {
}

pub const RVMCONSTANTVALUEINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmConstantValueInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(RVMCONSTANTVALUEINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmConstantValueInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        RVMCONSTANTVALUEINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const RVMCONSTANTVALUEINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmConstantValueInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("RvmConstantValueInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmCommonDatabaseLoaderImpl {
}

pub const RVMCOMMONDATABASELOADERIMPL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmCommonDatabaseLoaderImpl",
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMCOMMONDATABASELOADER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(RVMCOMMONDATABASELOADERIMPL_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RvmCommonDatabaseLoaderImpl {
    fn type_info() -> &'static TypeInfo {
        RVMCOMMONDATABASELOADERIMPL_TYPE_INFO
    }
}


pub const RVMCOMMONDATABASELOADERIMPL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmCommonDatabaseLoaderImpl-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("RvmCommonDatabaseLoaderImpl-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmCommonDatabaseLoader {
}

pub const RVMCOMMONDATABASELOADER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmCommonDatabaseLoader",
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMDATABASELOADER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(RVMCOMMONDATABASELOADER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RvmCommonDatabaseLoader {
    fn type_info() -> &'static TypeInfo {
        RVMCOMMONDATABASELOADER_TYPE_INFO
    }
}


pub const RVMCOMMONDATABASELOADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmCommonDatabaseLoader-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("RvmCommonDatabaseLoader-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CommonRvmBackend {
}

pub const COMMONRVMBACKEND_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CommonRvmBackend",
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMBACKEND_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(COMMONRVMBACKEND_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CommonRvmBackend {
    fn type_info() -> &'static TypeInfo {
        COMMONRVMBACKEND_TYPE_INFO
    }
}


pub const COMMONRVMBACKEND_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CommonRvmBackend-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("CommonRvmBackend-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BaseRvmDatabase {
}

pub const BASERVMDATABASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BaseRvmDatabase",
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMLEGACYDATABASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(BASERVMDATABASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BaseRvmDatabase {
    fn type_info() -> &'static TypeInfo {
        BASERVMDATABASE_TYPE_INFO
    }
}


pub const BASERVMDATABASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BaseRvmDatabase-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("BaseRvmDatabase-Array"),
    array_type: None,
    alignment: 8,
};


