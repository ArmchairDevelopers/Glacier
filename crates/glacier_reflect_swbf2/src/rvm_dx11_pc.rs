use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_rvm_dx11_pc_types(registry: &mut TypeRegistry) {
    registry.register_type(RVMDX11SETTINGS_TYPE_INFO);
    registry.register_type(RVMDX11SETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(DX11RVMBACKENDCONFIG_TYPE_INFO);
    registry.register_type(DX11RVMBACKENDCONFIG_ARRAY_TYPE_INFO);
    registry.register_type(DX11SHADERDISPATCHDRAWINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX11SHADERDISPATCHDRAWINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11SHADERDISPATCHDRAWINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(DX11RVMLEGACYVERTEXBUFFERCONVERSIONINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX11RVMLEGACYVERTEXBUFFERCONVERSIONINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11LEGACYVERTEXBUFFERCONVERSIONINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(DX11RVMVIEWSTATEINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX11RVMVIEWSTATEINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11VIEWSTATEINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(D3D11_CONSERVATIVE_RASTERIZATION_MODE_TYPE_INFO);
    registry.register_type(DX11RVMVIEWPORTS_TYPE_INFO);
    registry.register_type(DX11RVMSCISSORRECTS_TYPE_INFO);
    registry.register_type(DX11RVMDEPTHSTENCILSTATE_TYPE_INFO);
    registry.register_type(DX11RVMBLENDSTATE_TYPE_INFO);
    registry.register_type(D3D11_CULL_MODE_TYPE_INFO);
    registry.register_type(D3D11_FILL_MODE_TYPE_INFO);
    registry.register_type(DX11RVMSAMPLER_TYPE_INFO);
    registry.register_type(DX11RVMVSSHADER_TYPE_INFO);
    registry.register_type(DX11RVMPSSHADER_TYPE_INFO);
    registry.register_type(DX11RVMHSSHADER_TYPE_INFO);
    registry.register_type(DX11RVMDSSHADER_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11BYTECODEELEMENT_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11DSSHADER_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11HSSHADER_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11PSSHADER_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11VSSHADER_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11INPUTELEMENT_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11SAMPLER_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11BLENDSTATEDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11SERIALIZEDBLENDSTATE_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11TEXTURECONVERSIONINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11BUFFERCONVERSIONINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11LEGACYDRAWSTATEBUILDERDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11SHADERDISPATCHLEGACYDRAWINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11APPLYPARAMETERSINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11APPLYPARAMETERSBLOCK_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11DISPATCHINSTRUCTIONDATA_TYPE_INFO);
    registry.register_type(DX11RVMLEGACYDRAWSTATEBUILDERINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX11RVMLEGACYDRAWSTATEBUILDERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DX11RVMTEXTURECONVERSIONINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX11RVMTEXTURECONVERSIONINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DX11RVMBUFFERCONVERSIONINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX11RVMBUFFERCONVERSIONINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DX11RVMINSTANCEBUFFERWRITERINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX11RVMINSTANCEBUFFERWRITERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DX11RVMCONSTANTBUFFERWRITERINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX11RVMCONSTANTBUFFERWRITERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DX11RVMAPPLYSTATEINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX11RVMAPPLYSTATEINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DX11RVMDISPATCHINSTRUCTIONFACTORY_TYPE_INFO);
    registry.register_type(DX11RVMDISPATCHINSTRUCTIONFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(DX11RVMDATABASE_TYPE_INFO);
    registry.register_type(DX11RVMDATABASE_ARRAY_TYPE_INFO);
    registry.register_type(DX11RVMBACKEND_TYPE_INFO);
    registry.register_type(DX11RVMBACKEND_ARRAY_TYPE_INFO);
    registry.register_type(DX11RVMBACKENDFACTORY_TYPE_INFO);
    registry.register_type(DX11RVMBACKENDFACTORY_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmDx11Settings {
    pub enabled: bool,
    pub merged_offsetted_buffers_size: u32,
}

pub const RVMDX11SETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmDx11Settings",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx11Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMCOMMONSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RvmDx11Settings, enabled),
            },
            FieldInfoData {
                name: "MergedOffsettedBuffersSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(RvmDx11Settings, merged_offsetted_buffers_size),
            },
        ],
    }),
    array_type: Some(RVMDX11SETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmDx11Settings {
    fn type_info() -> &'static TypeInfo {
        RVMDX11SETTINGS_TYPE_INFO
    }
}


pub const RVMDX11SETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmDx11Settings-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx11Pc",
    data: TypeInfoData::Array("RvmDx11Settings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx11RvmBackendConfig {
}

pub const DX11RVMBACKENDCONFIG_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmBackendConfig",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx11Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMBACKENDCONFIG_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX11RVMBACKENDCONFIG_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11RvmBackendConfig {
    fn type_info() -> &'static TypeInfo {
        DX11RVMBACKENDCONFIG_TYPE_INFO
    }
}


pub const DX11RVMBACKENDCONFIG_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmBackendConfig-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx11Pc",
    data: TypeInfoData::Array("Dx11RvmBackendConfig-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx11ShaderDispatchDrawInstructionFactory {
}

pub const DX11SHADERDISPATCHDRAWINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11ShaderDispatchDrawInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx11Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX11SHADERDISPATCHDRAWINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11ShaderDispatchDrawInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX11SHADERDISPATCHDRAWINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX11SHADERDISPATCHDRAWINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11ShaderDispatchDrawInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx11Pc",
    data: TypeInfoData::Array("Dx11ShaderDispatchDrawInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx11ShaderDispatchDrawInstructionData {
}

pub const RVMSERIALIZEDDB_NS_DX11SHADERDISPATCHDRAWINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11ShaderDispatchDrawInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX11SHADERDISPATCHDRAWINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx11ShaderDispatchDrawInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11SHADERDISPATCHDRAWINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx11RvmLegacyVertexBufferConversionInstructionFactory {
}

pub const DX11RVMLEGACYVERTEXBUFFERCONVERSIONINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmLegacyVertexBufferConversionInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx11Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX11RVMLEGACYVERTEXBUFFERCONVERSIONINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11RvmLegacyVertexBufferConversionInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX11RVMLEGACYVERTEXBUFFERCONVERSIONINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX11RVMLEGACYVERTEXBUFFERCONVERSIONINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmLegacyVertexBufferConversionInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx11Pc",
    data: TypeInfoData::Array("Dx11RvmLegacyVertexBufferConversionInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx11LegacyVertexBufferConversionInstructionData {
}

pub const RVMSERIALIZEDDB_NS_DX11LEGACYVERTEXBUFFERCONVERSIONINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11LegacyVertexBufferConversionInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX11LEGACYVERTEXBUFFERCONVERSIONINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx11LegacyVertexBufferConversionInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11LEGACYVERTEXBUFFERCONVERSIONINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx11RvmViewStateInstructionFactory {
}

pub const DX11RVMVIEWSTATEINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmViewStateInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx11Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX11RVMVIEWSTATEINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11RvmViewStateInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX11RVMVIEWSTATEINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX11RVMVIEWSTATEINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmViewStateInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx11Pc",
    data: TypeInfoData::Array("Dx11RvmViewStateInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx11ViewStateInstructionData {
}

pub const RVMSERIALIZEDDB_NS_DX11VIEWSTATEINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11ViewStateInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX11VIEWSTATEINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx11ViewStateInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11VIEWSTATEINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct D3D11_CONSERVATIVE_RASTERIZATION_MODE {
}

pub const D3D11_CONSERVATIVE_RASTERIZATION_MODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D11_CONSERVATIVE_RASTERIZATION_MODE",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(D3D11_CONSERVATIVE_RASTERIZATION_MODE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for D3D11_CONSERVATIVE_RASTERIZATION_MODE {
    fn type_info() -> &'static TypeInfo {
        D3D11_CONSERVATIVE_RASTERIZATION_MODE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx11RvmViewports {
}

pub const DX11RVMVIEWPORTS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmViewports",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(DX11RVMVIEWPORTS_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for Dx11RvmViewports {
    fn type_info() -> &'static TypeInfo {
        DX11RVMVIEWPORTS_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx11RvmScissorRects {
}

pub const DX11RVMSCISSORRECTS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmScissorRects",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(DX11RVMSCISSORRECTS_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for Dx11RvmScissorRects {
    fn type_info() -> &'static TypeInfo {
        DX11RVMSCISSORRECTS_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx11RvmDepthStencilState {
}

pub const DX11RVMDEPTHSTENCILSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmDepthStencilState",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(DX11RVMDEPTHSTENCILSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for Dx11RvmDepthStencilState {
    fn type_info() -> &'static TypeInfo {
        DX11RVMDEPTHSTENCILSTATE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx11RvmBlendState {
}

pub const DX11RVMBLENDSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmBlendState",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(DX11RVMBLENDSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for Dx11RvmBlendState {
    fn type_info() -> &'static TypeInfo {
        DX11RVMBLENDSTATE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct D3D11_CULL_MODE {
}

pub const D3D11_CULL_MODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D11_CULL_MODE",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(D3D11_CULL_MODE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for D3D11_CULL_MODE {
    fn type_info() -> &'static TypeInfo {
        D3D11_CULL_MODE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct D3D11_FILL_MODE {
}

pub const D3D11_FILL_MODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D11_FILL_MODE",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(D3D11_FILL_MODE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for D3D11_FILL_MODE {
    fn type_info() -> &'static TypeInfo {
        D3D11_FILL_MODE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx11RvmSampler {
}

pub const DX11RVMSAMPLER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmSampler",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(DX11RVMSAMPLER_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for Dx11RvmSampler {
    fn type_info() -> &'static TypeInfo {
        DX11RVMSAMPLER_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx11RvmVsShader {
}

pub const DX11RVMVSSHADER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmVsShader",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(DX11RVMVSSHADER_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for Dx11RvmVsShader {
    fn type_info() -> &'static TypeInfo {
        DX11RVMVSSHADER_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx11RvmPsShader {
}

pub const DX11RVMPSSHADER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmPsShader",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(DX11RVMPSSHADER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11RvmPsShader {
    fn type_info() -> &'static TypeInfo {
        DX11RVMPSSHADER_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx11RvmHsShader {
}

pub const DX11RVMHSSHADER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmHsShader",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(DX11RVMHSSHADER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11RvmHsShader {
    fn type_info() -> &'static TypeInfo {
        DX11RVMHSSHADER_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx11RvmDsShader {
}

pub const DX11RVMDSSHADER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmDsShader",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(DX11RVMDSSHADER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11RvmDsShader {
    fn type_info() -> &'static TypeInfo {
        DX11RVMDSSHADER_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx11ByteCodeElement {
}

pub const RVMSERIALIZEDDB_NS_DX11BYTECODEELEMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11ByteCodeElement",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX11BYTECODEELEMENT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RvmSerializedDb_ns_Dx11ByteCodeElement {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11BYTECODEELEMENT_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx11DsShader {
}

pub const RVMSERIALIZEDDB_NS_DX11DSSHADER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11DsShader",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX11DSSHADER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx11DsShader {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11DSSHADER_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx11HsShader {
}

pub const RVMSERIALIZEDDB_NS_DX11HSSHADER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11HsShader",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX11HSSHADER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx11HsShader {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11HSSHADER_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx11PsShader {
}

pub const RVMSERIALIZEDDB_NS_DX11PSSHADER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11PsShader",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX11PSSHADER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx11PsShader {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11PSSHADER_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx11VsShader {
}

pub const RVMSERIALIZEDDB_NS_DX11VSSHADER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11VsShader",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX11VSSHADER_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RvmSerializedDb_ns_Dx11VsShader {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11VSSHADER_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx11InputElement {
}

pub const RVMSERIALIZEDDB_NS_DX11INPUTELEMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11InputElement",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX11INPUTELEMENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx11InputElement {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11INPUTELEMENT_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx11Sampler {
}

pub const RVMSERIALIZEDDB_NS_DX11SAMPLER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11Sampler",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX11SAMPLER_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RvmSerializedDb_ns_Dx11Sampler {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11SAMPLER_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx11BlendStateData {
}

pub const RVMSERIALIZEDDB_NS_DX11BLENDSTATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11BlendStateData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX11BLENDSTATEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx11BlendStateData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11BLENDSTATEDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx11SerializedBlendState {
}

pub const RVMSERIALIZEDDB_NS_DX11SERIALIZEDBLENDSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11SerializedBlendState",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX11SERIALIZEDBLENDSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RvmSerializedDb_ns_Dx11SerializedBlendState {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11SERIALIZEDBLENDSTATE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx11TextureConversionInstructionData {
}

pub const RVMSERIALIZEDDB_NS_DX11TEXTURECONVERSIONINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11TextureConversionInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX11TEXTURECONVERSIONINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx11TextureConversionInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11TEXTURECONVERSIONINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx11BufferConversionInstructionData {
}

pub const RVMSERIALIZEDDB_NS_DX11BUFFERCONVERSIONINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11BufferConversionInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX11BUFFERCONVERSIONINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx11BufferConversionInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11BUFFERCONVERSIONINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx11LegacyDrawStateBuilderData {
}

pub const RVMSERIALIZEDDB_NS_DX11LEGACYDRAWSTATEBUILDERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11LegacyDrawStateBuilderData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX11LEGACYDRAWSTATEBUILDERDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_Dx11LegacyDrawStateBuilderData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11LEGACYDRAWSTATEBUILDERDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx11ShaderDispatchLegacyDrawInstructionData {
}

pub const RVMSERIALIZEDDB_NS_DX11SHADERDISPATCHLEGACYDRAWINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11ShaderDispatchLegacyDrawInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX11SHADERDISPATCHLEGACYDRAWINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RvmSerializedDb_ns_Dx11ShaderDispatchLegacyDrawInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11SHADERDISPATCHLEGACYDRAWINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx11ApplyParametersInstructionData {
}

pub const RVMSERIALIZEDDB_NS_DX11APPLYPARAMETERSINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11ApplyParametersInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX11APPLYPARAMETERSINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx11ApplyParametersInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11APPLYPARAMETERSINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx11ApplyParametersBlock {
}

pub const RVMSERIALIZEDDB_NS_DX11APPLYPARAMETERSBLOCK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11ApplyParametersBlock",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX11APPLYPARAMETERSBLOCK_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx11ApplyParametersBlock {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11APPLYPARAMETERSBLOCK_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSerializedDb_ns_Dx11DispatchInstructionData {
}

pub const RVMSERIALIZEDDB_NS_DX11DISPATCHINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11DispatchInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSERIALIZEDDB_NS_DX11DISPATCHINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Dx11DispatchInstructionData {
    fn type_info() -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11DISPATCHINSTRUCTIONDATA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx11RvmLegacyDrawStateBuilderInstructionFactory {
}

pub const DX11RVMLEGACYDRAWSTATEBUILDERINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmLegacyDrawStateBuilderInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx11Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX11RVMLEGACYDRAWSTATEBUILDERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11RvmLegacyDrawStateBuilderInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX11RVMLEGACYDRAWSTATEBUILDERINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX11RVMLEGACYDRAWSTATEBUILDERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmLegacyDrawStateBuilderInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx11Pc",
    data: TypeInfoData::Array("Dx11RvmLegacyDrawStateBuilderInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx11RvmTextureConversionInstructionFactory {
}

pub const DX11RVMTEXTURECONVERSIONINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmTextureConversionInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx11Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX11RVMTEXTURECONVERSIONINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11RvmTextureConversionInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX11RVMTEXTURECONVERSIONINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX11RVMTEXTURECONVERSIONINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmTextureConversionInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx11Pc",
    data: TypeInfoData::Array("Dx11RvmTextureConversionInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx11RvmBufferConversionInstructionFactory {
}

pub const DX11RVMBUFFERCONVERSIONINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmBufferConversionInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx11Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX11RVMBUFFERCONVERSIONINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11RvmBufferConversionInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX11RVMBUFFERCONVERSIONINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX11RVMBUFFERCONVERSIONINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmBufferConversionInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx11Pc",
    data: TypeInfoData::Array("Dx11RvmBufferConversionInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx11RvmInstanceBufferWriterInstructionFactory {
}

pub const DX11RVMINSTANCEBUFFERWRITERINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmInstanceBufferWriterInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx11Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX11RVMINSTANCEBUFFERWRITERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11RvmInstanceBufferWriterInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX11RVMINSTANCEBUFFERWRITERINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX11RVMINSTANCEBUFFERWRITERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmInstanceBufferWriterInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx11Pc",
    data: TypeInfoData::Array("Dx11RvmInstanceBufferWriterInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx11RvmConstantBufferWriterInstructionFactory {
}

pub const DX11RVMCONSTANTBUFFERWRITERINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmConstantBufferWriterInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx11Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX11RVMCONSTANTBUFFERWRITERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11RvmConstantBufferWriterInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX11RVMCONSTANTBUFFERWRITERINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX11RVMCONSTANTBUFFERWRITERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmConstantBufferWriterInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx11Pc",
    data: TypeInfoData::Array("Dx11RvmConstantBufferWriterInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx11RvmApplyStateInstructionFactory {
}

pub const DX11RVMAPPLYSTATEINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmApplyStateInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx11Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX11RVMAPPLYSTATEINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11RvmApplyStateInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX11RVMAPPLYSTATEINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX11RVMAPPLYSTATEINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmApplyStateInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx11Pc",
    data: TypeInfoData::Array("Dx11RvmApplyStateInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx11RvmDispatchInstructionFactory {
}

pub const DX11RVMDISPATCHINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmDispatchInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx11Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX11RVMDISPATCHINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11RvmDispatchInstructionFactory {
    fn type_info() -> &'static TypeInfo {
        DX11RVMDISPATCHINSTRUCTIONFACTORY_TYPE_INFO
    }
}


pub const DX11RVMDISPATCHINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmDispatchInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx11Pc",
    data: TypeInfoData::Array("Dx11RvmDispatchInstructionFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx11RvmDatabase {
}

pub const DX11RVMDATABASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmDatabase",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx11Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BASERVMDATABASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX11RVMDATABASE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11RvmDatabase {
    fn type_info() -> &'static TypeInfo {
        DX11RVMDATABASE_TYPE_INFO
    }
}


pub const DX11RVMDATABASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmDatabase-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx11Pc",
    data: TypeInfoData::Array("Dx11RvmDatabase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx11RvmBackend {
}

pub const DX11RVMBACKEND_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmBackend",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx11Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMMONRVMBACKEND_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX11RVMBACKEND_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Dx11RvmBackend {
    fn type_info() -> &'static TypeInfo {
        DX11RVMBACKEND_TYPE_INFO
    }
}


pub const DX11RVMBACKEND_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmBackend-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx11Pc",
    data: TypeInfoData::Array("Dx11RvmBackend-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx11RvmBackendFactory {
}

pub const DX11RVMBACKENDFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmBackendFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx11Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMBACKENDFACTORY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX11RVMBACKENDFACTORY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Dx11RvmBackendFactory {
    fn type_info() -> &'static TypeInfo {
        DX11RVMBACKENDFACTORY_TYPE_INFO
    }
}


pub const DX11RVMBACKENDFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmBackendFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx11Pc",
    data: TypeInfoData::Array("Dx11RvmBackendFactory-Array"),
    array_type: None,
    alignment: 8,
};


