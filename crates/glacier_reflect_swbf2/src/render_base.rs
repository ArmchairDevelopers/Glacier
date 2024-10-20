use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_render_base_types(registry: &mut TypeRegistry) {
    registry.register_type(VERTEXSHADERFRAGMENTDYNAMICSTATE_TYPE_INFO);
    registry.register_type(VERTEXSHADERFRAGMENTDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(VERTEXSHADERFRAGMENTSTATICSTATE_TYPE_INFO);
    registry.register_type(VERTEXSHADERFRAGMENTSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(TEXTUREUNLOADONDEMANDDYNAMICSTATE_TYPE_INFO);
    registry.register_type(TEXTUREUNLOADONDEMANDDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(TEXTUREUNLOADONDEMANDSTATICSTATE_TYPE_INFO);
    registry.register_type(TEXTUREUNLOADONDEMANDSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(TEXTUREUNLOADONDEMANDLIFESTATE_TYPE_INFO);
    registry.register_type(TEXTUREUNLOADONDEMANDLIFESTATE_ARRAY_TYPE_INFO);
    registry.register_type(VERTEXSHADERFRAGMENTHANDLE_TYPE_INFO);
    registry.register_type(VERTEXSHADERFRAGMENTHANDLE_ARRAY_TYPE_INFO);
    registry.register_type(RENDERBUFFERHANDLE_TYPE_INFO);
    registry.register_type(RENDERBUFFERHANDLE_ARRAY_TYPE_INFO);
    registry.register_type(TEXTURERESOURCEHANDLE_TYPE_INFO);
    registry.register_type(TEXTURERESOURCEHANDLE_ARRAY_TYPE_INFO);
    registry.register_type(SHADERPARAMETERBLOCKHANDLE_TYPE_INFO);
    registry.register_type(SHADERPARAMETERBLOCKHANDLE_ARRAY_TYPE_INFO);
    registry.register_type(RENDERVIEWHANDLE_TYPE_INFO);
    registry.register_type(RENDERVIEWHANDLE_ARRAY_TYPE_INFO);
    registry.register_type(CULLIDHANDLE_TYPE_INFO);
    registry.register_type(CULLIDHANDLE_ARRAY_TYPE_INFO);
    registry.register_type(SURFACESHADERINSTANCEDATASTRUCT_TYPE_INFO);
    registry.register_type(SURFACESHADERINSTANCEDATASTRUCT_ARRAY_TYPE_INFO);
    registry.register_type(SURFACESHADERBASEASSET_TYPE_INFO);
    registry.register_type(SURFACESHADERBASEASSET_ARRAY_TYPE_INFO);
    registry.register_type(VECTORARRAYSHADERPARAMETER_TYPE_INFO);
    registry.register_type(VECTORARRAYSHADERPARAMETER_ARRAY_TYPE_INFO);
    registry.register_type(TEXTURESHADERPARAMETER_TYPE_INFO);
    registry.register_type(TEXTURESHADERPARAMETER_ARRAY_TYPE_INFO);
    registry.register_type(VECTORSHADERPARAMETER_TYPE_INFO);
    registry.register_type(VECTORSHADERPARAMETER_ARRAY_TYPE_INFO);
    registry.register_type(CONDITIONALSHADERPARAMETER_TYPE_INFO);
    registry.register_type(CONDITIONALSHADERPARAMETER_ARRAY_TYPE_INFO);
    registry.register_type(BOOLSHADERPARAMETER_TYPE_INFO);
    registry.register_type(BOOLSHADERPARAMETER_ARRAY_TYPE_INFO);
    registry.register_type(SHADERPARAMETERDESC_TYPE_INFO);
    registry.register_type(SHADERPARAMETERDESC_ARRAY_TYPE_INFO);
    registry.register_type(SHADERINDIRECTSPECULARPARAMTYPE_TYPE_INFO);
    registry.register_type(SHADERINDIRECTSPECULARPARAMTYPE_ARRAY_TYPE_INFO);
    registry.register_type(SHADERPARAMETERTYPE_TYPE_INFO);
    registry.register_type(SHADERPARAMETERTYPE_ARRAY_TYPE_INFO);
    registry.register_type(EXTERNALVALUECONSTANTTYPE_TYPE_INFO);
    registry.register_type(EXTERNALVALUECONSTANTTYPE_ARRAY_TYPE_INFO);
    registry.register_type(SHADERSHADOWMAPQUALITY_TYPE_INFO);
    registry.register_type(SHADERSHADOWMAPQUALITY_ARRAY_TYPE_INFO);
    registry.register_type(SHADERPARAMETERBLOCKDYNAMICSTATE_TYPE_INFO);
    registry.register_type(SHADERPARAMETERBLOCKDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(SHADERBLOCKRENDERBUFFER_TYPE_INFO);
    registry.register_type(SHADERBLOCKRENDERBUFFER_ARRAY_TYPE_INFO);
    registry.register_type(SHADERBLOCKTEXTURE_TYPE_INFO);
    registry.register_type(SHADERBLOCKTEXTURE_ARRAY_TYPE_INFO);
    registry.register_type(SHADERBLOCKVEC_TYPE_INFO);
    registry.register_type(SHADERBLOCKVEC_ARRAY_TYPE_INFO);
    registry.register_type(SHADERBLOCKCONDITIONAL_TYPE_INFO);
    registry.register_type(SHADERBLOCKCONDITIONAL_ARRAY_TYPE_INFO);
    registry.register_type(SHADERBLOCKBOOL_TYPE_INFO);
    registry.register_type(SHADERBLOCKBOOL_ARRAY_TYPE_INFO);
    registry.register_type(SHADERBLOCKPARAMETER_TYPE_INFO);
    registry.register_type(SHADERBLOCKPARAMETER_ARRAY_TYPE_INFO);
    registry.register_type(SHADERPARAMETERBLOCKSTATICSTATE_TYPE_INFO);
    registry.register_type(SHADERPARAMETERBLOCKSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(SCREENSHOTDYNAMICSTATE_TYPE_INFO);
    registry.register_type(SCREENSHOTDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(SCREENSHOTSTATE_TYPE_INFO);
    registry.register_type(SCREENSHOTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(SCREENSHOTFRAMENAMINGMODE_TYPE_INFO);
    registry.register_type(SCREENSHOTFRAMENAMINGMODE_ARRAY_TYPE_INFO);
    registry.register_type(SCREENSHOTLAYERMODE_TYPE_INFO);
    registry.register_type(SCREENSHOTLAYERMODE_ARRAY_TYPE_INFO);
    registry.register_type(SCREENSHOTFORMAT_TYPE_INFO);
    registry.register_type(SCREENSHOTFORMAT_ARRAY_TYPE_INFO);
    registry.register_type(VIEWDEFINITION_TYPE_INFO);
    registry.register_type(VIEWDEFINITION_ARRAY_TYPE_INFO);
    registry.register_type(VIEWDEFINITIONTYPE_TYPE_INFO);
    registry.register_type(VIEWDEFINITIONTYPE_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICALCAMERADESC_TYPE_INFO);
    registry.register_type(PHYSICALCAMERADESC_ARRAY_TYPE_INFO);
    registry.register_type(AUTOEXPOSUREMETHOD_TYPE_INFO);
    registry.register_type(AUTOEXPOSUREMETHOD_ARRAY_TYPE_INFO);
    registry.register_type(RENDERVIEWDYNAMICSTATE_TYPE_INFO);
    registry.register_type(RENDERVIEWDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(RENDERVIEWSTATICSTATE_TYPE_INFO);
    registry.register_type(RENDERVIEWSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(SCISSORRECT_TYPE_INFO);
    registry.register_type(SCISSORRECT_ARRAY_TYPE_INFO);
    registry.register_type(VIEWPORTRECT_TYPE_INFO);
    registry.register_type(VIEWPORTRECT_ARRAY_TYPE_INFO);
    registry.register_type(MESHCOMPUTEBASENODE_TYPE_INFO);
    registry.register_type(MESHCOMPUTEBASENODE_ARRAY_TYPE_INFO);
    registry.register_type(MESHCOMPUTEBASEASSET_TYPE_INFO);
    registry.register_type(MESHCOMPUTEBASEASSET_ARRAY_TYPE_INFO);
    registry.register_type(STATICENLIGHTENBASEASSET_TYPE_INFO);
    registry.register_type(STATICENLIGHTENBASEASSET_ARRAY_TYPE_INFO);
    registry.register_type(ENLIGHTENRUNTIMECONFIGBASEASSET_TYPE_INFO);
    registry.register_type(ENLIGHTENRUNTIMECONFIGBASEASSET_ARRAY_TYPE_INFO);
    registry.register_type(ENLIGHTENSHADERDATABASEBASEASSET_TYPE_INFO);
    registry.register_type(ENLIGHTENSHADERDATABASEBASEASSET_ARRAY_TYPE_INFO);
    registry.register_type(ENLIGHTENBASEASSET_TYPE_INFO);
    registry.register_type(ENLIGHTENBASEASSET_ARRAY_TYPE_INFO);
    registry.register_type(MESHLODGROUPBASEASSET_TYPE_INFO);
    registry.register_type(MESHLODGROUPBASEASSET_ARRAY_TYPE_INFO);
    registry.register_type(MESHBASEASSET_TYPE_INFO);
    registry.register_type(MESHBASEASSET_ARRAY_TYPE_INFO);
    registry.register_type(RADIOSITYMIXMODE_TYPE_INFO);
    registry.register_type(RADIOSITYMIXMODE_ARRAY_TYPE_INFO);
    registry.register_type(RADIOSITYBACKFACETYPE_TYPE_INFO);
    registry.register_type(RADIOSITYBACKFACETYPE_ARRAY_TYPE_INFO);
    registry.register_type(MESHTYPE_TYPE_INFO);
    registry.register_type(MESHTYPE_ARRAY_TYPE_INFO);
    registry.register_type(CLOUDSHADOWTYPE_TYPE_INFO);
    registry.register_type(CLOUDSHADOWTYPE_ARRAY_TYPE_INFO);
    registry.register_type(TEXTUREADDRESS_TYPE_INFO);
    registry.register_type(TEXTUREADDRESS_ARRAY_TYPE_INFO);
    registry.register_type(RENDERTEXTUREBASEASSET_TYPE_INFO);
    registry.register_type(RENDERTEXTUREBASEASSET_ARRAY_TYPE_INFO);
    registry.register_type(IPGRAPHCROSSREFERENCETEXTUREASSET_TYPE_INFO);
    registry.register_type(IPGRAPHCROSSREFERENCETEXTUREASSET_ARRAY_TYPE_INFO);
    registry.register_type(TEXTUREBASEASSET_TYPE_INFO);
    registry.register_type(TEXTUREBASEASSET_ARRAY_TYPE_INFO);
    registry.register_type(PERFORMANCESUMMARYMESSAGE_TYPE_INFO);
    registry.register_type(PERFORMANCECLIENTMESSAGE_TYPE_INFO);
    registry.register_type(PERFORMANCESHADERMESSAGE_TYPE_INFO);
    registry.register_type(SHADERSTATSMESSAGE_TYPE_INFO);
    registry.register_type(SHADERSTATSMESSAGE_ARRAY_TYPE_INFO);
    registry.register_type(PERFORMANCEMESSAGE_TYPE_INFO);
    registry.register_type(PERFORMANCEMESSAGE_ARRAY_TYPE_INFO);
    registry.register_type(RENDERBUFFERDYNAMICSTATE_TYPE_INFO);
    registry.register_type(RENDERBUFFERDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(RENDERBUFFERSTATICSTATE_TYPE_INFO);
    registry.register_type(RENDERBUFFERSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(LENSREFLECTIONCOMPONENTSTATE_TYPE_INFO);
    registry.register_type(LENSREFLECTIONCOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(LENSREFLECTIONCOMPONENTCOUNT_TYPE_INFO);
    registry.register_type(LENSREFLECTIONCOMPONENTCOUNT_ARRAY_TYPE_INFO);
    registry.register_type(FILMICEFFECTSCOMPONENTSTATE_TYPE_INFO);
    registry.register_type(FILMICEFFECTSCOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(DEBUGCOMPONENTSTATE_TYPE_INFO);
    registry.register_type(DEBUGCOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(LENSSCOPECOMPONENTSTATE_TYPE_INFO);
    registry.register_type(LENSSCOPECOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(FILMGRAINCOMPONENTSTATE_TYPE_INFO);
    registry.register_type(FILMGRAINCOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(VIGNETTECOMPONENTSTATE_TYPE_INFO);
    registry.register_type(VIGNETTECOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(DOFCOMPONENTSTATE_TYPE_INFO);
    registry.register_type(DOFCOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(VIGNETTINGOPERATION_TYPE_INFO);
    registry.register_type(VIGNETTINGOPERATION_ARRAY_TYPE_INFO);
    registry.register_type(DOFSOURCE_TYPE_INFO);
    registry.register_type(DOFSOURCE_ARRAY_TYPE_INFO);
    registry.register_type(COLORCORRECTIONCOMPONENTSTATE_TYPE_INFO);
    registry.register_type(COLORCORRECTIONCOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(COLORGRADING_TYPE_INFO);
    registry.register_type(COLORGRADING_ARRAY_TYPE_INFO);
    registry.register_type(TONEMAPCOMPONENTSTATE_TYPE_INFO);
    registry.register_type(TONEMAPCOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(TONEMAPMETHOD_TYPE_INFO);
    registry.register_type(TONEMAPMETHOD_ARRAY_TYPE_INFO);
    registry.register_type(SUBSURFACEPROFILE_TYPE_INFO);
    registry.register_type(SUBSURFACEPROFILE_ARRAY_TYPE_INFO);
    registry.register_type(SUBSURFACEPROFILEPRESET_TYPE_INFO);
    registry.register_type(SUBSURFACEPROFILEPRESET_ARRAY_TYPE_INFO);
    registry.register_type(COLORGRADINGQUALITYMODE_TYPE_INFO);
    registry.register_type(COLORGRADINGQUALITYMODE_ARRAY_TYPE_INFO);
    registry.register_type(BLOOMDIRECTION_TYPE_INFO);
    registry.register_type(BLOOMDIRECTION_ARRAY_TYPE_INFO);
    registry.register_type(BLOOMMETHOD_TYPE_INFO);
    registry.register_type(BLOOMMETHOD_ARRAY_TYPE_INFO);
    registry.register_type(BLURFILTER_TYPE_INFO);
    registry.register_type(BLURFILTER_ARRAY_TYPE_INFO);
    registry.register_type(LOCALPLAYERVIEWID_TYPE_INFO);
    registry.register_type(LOCALPLAYERVIEWID_ARRAY_TYPE_INFO);
    registry.register_type(DISTANTIBLLOCATIONTYPE_TYPE_INFO);
    registry.register_type(DISTANTIBLLOCATIONTYPE_ARRAY_TYPE_INFO);
    registry.register_type(LIGHTTILEPASSTYPE_TYPE_INFO);
    registry.register_type(LIGHTTILEPASSTYPE_ARRAY_TYPE_INFO);
    registry.register_type(LOCALIBLMODE_TYPE_INFO);
    registry.register_type(LOCALIBLMODE_ARRAY_TYPE_INFO);
    registry.register_type(PUNCTUALLIGHTTYPE_TYPE_INFO);
    registry.register_type(PUNCTUALLIGHTTYPE_ARRAY_TYPE_INFO);
    registry.register_type(RECTANGULARLIGHTSHAPE_TYPE_INFO);
    registry.register_type(RECTANGULARLIGHTSHAPE_ARRAY_TYPE_INFO);
    registry.register_type(LIGHTUNITTYPE_TYPE_INFO);
    registry.register_type(LIGHTUNITTYPE_ARRAY_TYPE_INFO);
    registry.register_type(PIPELINEEXTERNALSHADERCONDITIONAL_TYPE_INFO);
    registry.register_type(PIPELINEEXTERNALSHADERCONDITIONAL_ARRAY_TYPE_INFO);
    registry.register_type(PIPELINEEXTERNALSHADERCONDITIONALBRANCH_TYPE_INFO);
    registry.register_type(PIPELINEEXTERNALSHADERCONDITIONALBRANCH_ARRAY_TYPE_INFO);
    registry.register_type(EXTERNALSHADERCONDITIONALFILTERASSET_TYPE_INFO);
    registry.register_type(EXTERNALSHADERCONDITIONALFILTERASSET_ARRAY_TYPE_INFO);
    registry.register_type(EXTERNALSHADERCONDITIONALSELECTION_TYPE_INFO);
    registry.register_type(EXTERNALSHADERCONDITIONALSELECTION_ARRAY_TYPE_INFO);
    registry.register_type(EXTERNALSHADERBOOLEANDESCRIPTION_TYPE_INFO);
    registry.register_type(EXTERNALSHADERBOOLEANDESCRIPTION_ARRAY_TYPE_INFO);
    registry.register_type(EXTERNALSHADERSWITCHDESCRIPTION_TYPE_INFO);
    registry.register_type(EXTERNALSHADERSWITCHDESCRIPTION_ARRAY_TYPE_INFO);
    registry.register_type(EXTERNALSHADERCONDITIONALASSET_TYPE_INFO);
    registry.register_type(EXTERNALSHADERCONDITIONALASSET_ARRAY_TYPE_INFO);
    registry.register_type(SHADOWCACHEDEPTHBIAS_TYPE_INFO);
    registry.register_type(SHADOWCACHEDEPTHBIAS_ARRAY_TYPE_INFO);
    registry.register_type(SHADOWCACHEMODE_TYPE_INFO);
    registry.register_type(SHADOWCACHEMODE_ARRAY_TYPE_INFO);
    registry.register_type(DECALATLASTILE_TYPE_INFO);
    registry.register_type(DECALATLASTILE_ARRAY_TYPE_INFO);
    registry.register_type(DECALTEMPLATEBASEASSET_TYPE_INFO);
    registry.register_type(DECALTEMPLATEBASEASSET_ARRAY_TYPE_INFO);
    registry.register_type(DECALTYPE_TYPE_INFO);
    registry.register_type(DECALTYPE_ARRAY_TYPE_INFO);
    registry.register_type(CULLIDDYNAMICSTATE_TYPE_INFO);
    registry.register_type(CULLIDDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(CULLIDSTATICSTATE_TYPE_INFO);
    registry.register_type(CULLIDSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(OUTLINECOLORSCOMPONENTDATA_TYPE_INFO);
    registry.register_type(OUTLINECOLORSCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(OCCLUDERREALPLANEENTITYDATA_TYPE_INFO);
    registry.register_type(OCCLUDERREALPLANEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(FOGVOLUMEENTITYDATA_TYPE_INFO);
    registry.register_type(FOGVOLUMEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(LAYERTEXTURECONFIGASSET_TYPE_INFO);
    registry.register_type(LAYERTEXTURECONFIGASSET_ARRAY_TYPE_INFO);
    registry.register_type(LAYERTEXTURECONFIG_TYPE_INFO);
    registry.register_type(LAYERTEXTURECONFIG_ARRAY_TYPE_INFO);
    registry.register_type(FSCRIPT_VERLETCHAINENTITYDATA_TYPE_INFO);
    registry.register_type(FSCRIPT_VERLETCHAINENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(FSCRIPT_PROCEDURALBONEGLOBALSENTITYDATA_TYPE_INFO);
    registry.register_type(FSCRIPT_PROCEDURALBONEGLOBALSENTITYDATA_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VertexShaderFragmentDynamicState {
}

pub const VERTEXSHADERFRAGMENTDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VertexShaderFragmentDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(VERTEXSHADERFRAGMENTDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for VertexShaderFragmentDynamicState {
    fn type_info() -> &'static TypeInfo {
        VERTEXSHADERFRAGMENTDYNAMICSTATE_TYPE_INFO
    }
}


pub const VERTEXSHADERFRAGMENTDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VertexShaderFragmentDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("VertexShaderFragmentDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VertexShaderFragmentStaticState {
    pub fragment_path: String,
    pub field_flag_changed0: u8,
}

pub const VERTEXSHADERFRAGMENTSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VertexShaderFragmentStaticState",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "FragmentPath",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(VertexShaderFragmentStaticState, fragment_path),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(VertexShaderFragmentStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(VERTEXSHADERFRAGMENTSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VertexShaderFragmentStaticState {
    fn type_info() -> &'static TypeInfo {
        VERTEXSHADERFRAGMENTSTATICSTATE_TYPE_INFO
    }
}


pub const VERTEXSHADERFRAGMENTSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VertexShaderFragmentStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("VertexShaderFragmentStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TextureUnloadOnDemandDynamicState {
    pub texture_state: TextureUnloadOnDemandLifeState,
    pub tick_update: u32,
    pub field_flag_changed0: u8,
}

pub const TEXTUREUNLOADONDEMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureUnloadOnDemandDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "TextureState",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREUNLOADONDEMANDLIFESTATE_TYPE_INFO,
                rust_offset: offset_of!(TextureUnloadOnDemandDynamicState, texture_state),
            },
            FieldInfoData {
                name: "TickUpdate",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TextureUnloadOnDemandDynamicState, tick_update),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TextureUnloadOnDemandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TEXTUREUNLOADONDEMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for TextureUnloadOnDemandDynamicState {
    fn type_info() -> &'static TypeInfo {
        TEXTUREUNLOADONDEMANDDYNAMICSTATE_TYPE_INFO
    }
}


pub const TEXTUREUNLOADONDEMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureUnloadOnDemandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("TextureUnloadOnDemandDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TextureUnloadOnDemandStaticState {
    pub identifier: u32,
    pub texture_handle: u16,
    pub field_flag_changed0: u8,
}

pub const TEXTUREUNLOADONDEMANDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureUnloadOnDemandStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Identifier",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TextureUnloadOnDemandStaticState, identifier),
            },
            FieldInfoData {
                name: "TextureHandle",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(TextureUnloadOnDemandStaticState, texture_handle),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TextureUnloadOnDemandStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TEXTUREUNLOADONDEMANDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for TextureUnloadOnDemandStaticState {
    fn type_info() -> &'static TypeInfo {
        TEXTUREUNLOADONDEMANDSTATICSTATE_TYPE_INFO
    }
}


pub const TEXTUREUNLOADONDEMANDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureUnloadOnDemandStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("TextureUnloadOnDemandStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum TextureUnloadOnDemandLifeState {
    #[default]
    TextureUnloadOnDemand_Unloaded = 0,
    TextureUnloadOnDemand_Loaded = 1,
    TextureUnloadOnDemand_Unregistered = 2,
}

pub const TEXTUREUNLOADONDEMANDLIFESTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureUnloadOnDemandLifeState",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(TEXTUREUNLOADONDEMANDLIFESTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TextureUnloadOnDemandLifeState {
    fn type_info() -> &'static TypeInfo {
        TEXTUREUNLOADONDEMANDLIFESTATE_TYPE_INFO
    }
}


pub const TEXTUREUNLOADONDEMANDLIFESTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureUnloadOnDemandLifeState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("TextureUnloadOnDemandLifeState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VertexShaderFragmentHandle {
}

pub const VERTEXSHADERFRAGMENTHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VertexShaderFragmentHandle",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(VERTEXSHADERFRAGMENTHANDLE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for VertexShaderFragmentHandle {
    fn type_info() -> &'static TypeInfo {
        VERTEXSHADERFRAGMENTHANDLE_TYPE_INFO
    }
}


pub const VERTEXSHADERFRAGMENTHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VertexShaderFragmentHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("VertexShaderFragmentHandle-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RenderBufferHandle {
}

pub const RENDERBUFFERHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderBufferHandle",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RENDERBUFFERHANDLE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RenderBufferHandle {
    fn type_info() -> &'static TypeInfo {
        RENDERBUFFERHANDLE_TYPE_INFO
    }
}


pub const RENDERBUFFERHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderBufferHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("RenderBufferHandle-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TextureResourceHandle {
}

pub const TEXTURERESOURCEHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureResourceHandle",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(TEXTURERESOURCEHANDLE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for TextureResourceHandle {
    fn type_info() -> &'static TypeInfo {
        TEXTURERESOURCEHANDLE_TYPE_INFO
    }
}


pub const TEXTURERESOURCEHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureResourceHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("TextureResourceHandle-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShaderParameterBlockHandle {
}

pub const SHADERPARAMETERBLOCKHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterBlockHandle",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(SHADERPARAMETERBLOCKHANDLE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for ShaderParameterBlockHandle {
    fn type_info() -> &'static TypeInfo {
        SHADERPARAMETERBLOCKHANDLE_TYPE_INFO
    }
}


pub const SHADERPARAMETERBLOCKHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterBlockHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ShaderParameterBlockHandle-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RenderViewHandle {
}

pub const RENDERVIEWHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderViewHandle",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RENDERVIEWHANDLE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RenderViewHandle {
    fn type_info() -> &'static TypeInfo {
        RENDERVIEWHANDLE_TYPE_INFO
    }
}


pub const RENDERVIEWHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderViewHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("RenderViewHandle-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CullIdHandle {
}

pub const CULLIDHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CullIdHandle",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(CULLIDHANDLE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for CullIdHandle {
    fn type_info() -> &'static TypeInfo {
        CULLIDHANDLE_TYPE_INFO
    }
}


pub const CULLIDHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CullIdHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("CullIdHandle-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SurfaceShaderInstanceDataStruct {
    pub shader: SurfaceShaderBaseAsset,
    pub surface_shader_name: String,
    pub bool_parameters: Vec<BoolShaderParameter>,
    pub vector_parameters: Vec<VectorShaderParameter>,
    pub vector_array_parameters: Vec<VectorArrayShaderParameter>,
    pub texture_parameters: Vec<TextureShaderParameter>,
    pub conditional_parameters: Vec<ConditionalShaderParameter>,
}

pub const SURFACESHADERINSTANCEDATASTRUCT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SurfaceShaderInstanceDataStruct",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Shader",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SurfaceShaderInstanceDataStruct, shader),
            },
            FieldInfoData {
                name: "SurfaceShaderName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(SurfaceShaderInstanceDataStruct, surface_shader_name),
            },
            FieldInfoData {
                name: "BoolParameters",
                flags: MemberInfoFlags::new(144),
                field_type: BOOLSHADERPARAMETER_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SurfaceShaderInstanceDataStruct, bool_parameters),
            },
            FieldInfoData {
                name: "VectorParameters",
                flags: MemberInfoFlags::new(144),
                field_type: VECTORSHADERPARAMETER_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SurfaceShaderInstanceDataStruct, vector_parameters),
            },
            FieldInfoData {
                name: "VectorArrayParameters",
                flags: MemberInfoFlags::new(144),
                field_type: VECTORARRAYSHADERPARAMETER_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SurfaceShaderInstanceDataStruct, vector_array_parameters),
            },
            FieldInfoData {
                name: "TextureParameters",
                flags: MemberInfoFlags::new(144),
                field_type: TEXTURESHADERPARAMETER_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SurfaceShaderInstanceDataStruct, texture_parameters),
            },
            FieldInfoData {
                name: "ConditionalParameters",
                flags: MemberInfoFlags::new(8336),
                field_type: CONDITIONALSHADERPARAMETER_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SurfaceShaderInstanceDataStruct, conditional_parameters),
            },
        ],
    }),
    array_type: Some(SURFACESHADERINSTANCEDATASTRUCT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SurfaceShaderInstanceDataStruct {
    fn type_info() -> &'static TypeInfo {
        SURFACESHADERINSTANCEDATASTRUCT_TYPE_INFO
    }
}


pub const SURFACESHADERINSTANCEDATASTRUCT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SurfaceShaderInstanceDataStruct-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("SurfaceShaderInstanceDataStruct-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SurfaceShaderBaseAsset {
}

pub const SURFACESHADERBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SurfaceShaderBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SURFACESHADERBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SurfaceShaderBaseAsset {
    fn type_info() -> &'static TypeInfo {
        SURFACESHADERBASEASSET_TYPE_INFO
    }
}


pub const SURFACESHADERBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SurfaceShaderBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("SurfaceShaderBaseAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct VectorArrayShaderParameter {
    pub parameter_name: String,
    pub parameter_type: ShaderParameterType,
    pub values: Vec<super::core::Vec4>,
}

pub const VECTORARRAYSHADERPARAMETER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VectorArrayShaderParameter",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(VectorArrayShaderParameter, parameter_name),
            },
            FieldInfoData {
                name: "ParameterType",
                flags: MemberInfoFlags::new(0),
                field_type: SHADERPARAMETERTYPE_TYPE_INFO,
                rust_offset: offset_of!(VectorArrayShaderParameter, parameter_type),
            },
            FieldInfoData {
                name: "Values",
                flags: MemberInfoFlags::new(144),
                field_type: VEC4_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(VectorArrayShaderParameter, values),
            },
        ],
    }),
    array_type: Some(VECTORARRAYSHADERPARAMETER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VectorArrayShaderParameter {
    fn type_info() -> &'static TypeInfo {
        VECTORARRAYSHADERPARAMETER_TYPE_INFO
    }
}


pub const VECTORARRAYSHADERPARAMETER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VectorArrayShaderParameter-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("VectorArrayShaderParameter-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TextureShaderParameter {
    pub parameter_name: String,
    pub value: TextureBaseAsset,
}

pub const TEXTURESHADERPARAMETER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureShaderParameter",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TextureShaderParameter, parameter_name),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(TextureShaderParameter, value),
            },
        ],
    }),
    array_type: Some(TEXTURESHADERPARAMETER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TextureShaderParameter {
    fn type_info() -> &'static TypeInfo {
        TEXTURESHADERPARAMETER_TYPE_INFO
    }
}


pub const TEXTURESHADERPARAMETER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureShaderParameter-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("TextureShaderParameter-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct VectorShaderParameter {
    pub parameter_name: String,
    pub parameter_type: ShaderParameterType,
    pub value: super::core::Vec4,
}

pub const VECTORSHADERPARAMETER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VectorShaderParameter",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(VectorShaderParameter, parameter_name),
            },
            FieldInfoData {
                name: "ParameterType",
                flags: MemberInfoFlags::new(0),
                field_type: SHADERPARAMETERTYPE_TYPE_INFO,
                rust_offset: offset_of!(VectorShaderParameter, parameter_type),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(VectorShaderParameter, value),
            },
        ],
    }),
    array_type: Some(VECTORSHADERPARAMETER_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for VectorShaderParameter {
    fn type_info() -> &'static TypeInfo {
        VECTORSHADERPARAMETER_TYPE_INFO
    }
}


pub const VECTORSHADERPARAMETER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VectorShaderParameter-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("VectorShaderParameter-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ConditionalShaderParameter {
    pub value: String,
    pub conditional_asset: ExternalShaderConditionalAsset,
}

pub const CONDITIONALSHADERPARAMETER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalShaderParameter",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(8192),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ConditionalShaderParameter, value),
            },
            FieldInfoData {
                name: "ConditionalAsset",
                flags: MemberInfoFlags::new(8192),
                field_type: EXTERNALSHADERCONDITIONALASSET_TYPE_INFO,
                rust_offset: offset_of!(ConditionalShaderParameter, conditional_asset),
            },
        ],
    }),
    array_type: Some(CONDITIONALSHADERPARAMETER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ConditionalShaderParameter {
    fn type_info() -> &'static TypeInfo {
        CONDITIONALSHADERPARAMETER_TYPE_INFO
    }
}


pub const CONDITIONALSHADERPARAMETER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalShaderParameter-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ConditionalShaderParameter-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BoolShaderParameter {
    pub parameter_name: String,
    pub value: bool,
}

pub const BOOLSHADERPARAMETER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoolShaderParameter",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(BoolShaderParameter, parameter_name),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BoolShaderParameter, value),
            },
        ],
    }),
    array_type: Some(BOOLSHADERPARAMETER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BoolShaderParameter {
    fn type_info() -> &'static TypeInfo {
        BOOLSHADERPARAMETER_TYPE_INFO
    }
}


pub const BOOLSHADERPARAMETER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoolShaderParameter-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("BoolShaderParameter-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShaderParameterDesc {
    pub part0: u64,
    pub part1: u64,
}

pub const SHADERPARAMETERDESC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterDesc",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Part0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT64_TYPE_INFO,
                rust_offset: offset_of!(ShaderParameterDesc, part0),
            },
            FieldInfoData {
                name: "Part1",
                flags: MemberInfoFlags::new(0),
                field_type: UINT64_TYPE_INFO,
                rust_offset: offset_of!(ShaderParameterDesc, part1),
            },
        ],
    }),
    array_type: Some(SHADERPARAMETERDESC_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ShaderParameterDesc {
    fn type_info() -> &'static TypeInfo {
        SHADERPARAMETERDESC_TYPE_INFO
    }
}


pub const SHADERPARAMETERDESC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterDesc-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ShaderParameterDesc-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ShaderIndirectSpecularParamType {
    #[default]
    ShaderIndirectSpecularParamType_SpecularIntensity = 0,
    ShaderIndirectSpecularParamType_ReflectanceScale = 1,
    ShaderIndirectSpecularParamType_ProbesSpecularIntensity = 2,
    ShaderIndirectSpecularParamType_ProbesReflectanceScale = 3,
}

pub const SHADERINDIRECTSPECULARPARAMTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderIndirectSpecularParamType",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(SHADERINDIRECTSPECULARPARAMTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShaderIndirectSpecularParamType {
    fn type_info() -> &'static TypeInfo {
        SHADERINDIRECTSPECULARPARAMTYPE_TYPE_INFO
    }
}


pub const SHADERINDIRECTSPECULARPARAMTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderIndirectSpecularParamType-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ShaderIndirectSpecularParamType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ShaderParameterType {
    #[default]
    ShaderParameterType_Bool = 0,
    ShaderParameterType_Int = 1,
    ShaderParameterType_Scalar = 2,
    ShaderParameterType_Vec2 = 3,
    ShaderParameterType_Vec3 = 4,
    ShaderParameterType_Vec4 = 5,
    ShaderParameterType_Color = 6,
    ShaderParameterType_TextureSlice = 7,
    ShaderParameterTypeCount = 8,
}

pub const SHADERPARAMETERTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterType",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(SHADERPARAMETERTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShaderParameterType {
    fn type_info() -> &'static TypeInfo {
        SHADERPARAMETERTYPE_TYPE_INFO
    }
}


pub const SHADERPARAMETERTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterType-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ShaderParameterType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ExternalValueConstantType {
    #[default]
    ExternalValueConstantType_Vec = 0,
    ExternalValueConstantType_Bool = 1,
    ExternalValueConstantType_Texture = 2,
    ExternalValueConstantType_Conditional = 3,
}

pub const EXTERNALVALUECONSTANTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExternalValueConstantType",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(EXTERNALVALUECONSTANTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ExternalValueConstantType {
    fn type_info() -> &'static TypeInfo {
        EXTERNALVALUECONSTANTTYPE_TYPE_INFO
    }
}


pub const EXTERNALVALUECONSTANTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExternalValueConstantType-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ExternalValueConstantType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ShaderShadowmapQuality {
    #[default]
    ShaderShadowmapQuality_Low = 0,
    ShaderShadowmapQuality_High = 1,
    ShaderShadowmapQualityCount = 2,
}

pub const SHADERSHADOWMAPQUALITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderShadowmapQuality",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(SHADERSHADOWMAPQUALITY_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShaderShadowmapQuality {
    fn type_info() -> &'static TypeInfo {
        SHADERSHADOWMAPQUALITY_TYPE_INFO
    }
}


pub const SHADERSHADOWMAPQUALITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderShadowmapQuality-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ShaderShadowmapQuality-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ShaderParameterBlockDynamicState {
    pub vecs: Vec<ShaderBlockVec>,
    pub bools: Vec<ShaderBlockBool>,
    pub conditionals: Vec<ShaderBlockConditional>,
    pub textures: Vec<ShaderBlockTexture>,
    pub render_buffers: Vec<ShaderBlockRenderBuffer>,
    pub field_flag_changed0: u8,
}

pub const SHADERPARAMETERBLOCKDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterBlockDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Vecs",
                flags: MemberInfoFlags::new(144),
                field_type: SHADERBLOCKVEC_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ShaderParameterBlockDynamicState, vecs),
            },
            FieldInfoData {
                name: "Bools",
                flags: MemberInfoFlags::new(144),
                field_type: SHADERBLOCKBOOL_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ShaderParameterBlockDynamicState, bools),
            },
            FieldInfoData {
                name: "Conditionals",
                flags: MemberInfoFlags::new(144),
                field_type: SHADERBLOCKCONDITIONAL_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ShaderParameterBlockDynamicState, conditionals),
            },
            FieldInfoData {
                name: "Textures",
                flags: MemberInfoFlags::new(144),
                field_type: SHADERBLOCKTEXTURE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ShaderParameterBlockDynamicState, textures),
            },
            FieldInfoData {
                name: "RenderBuffers",
                flags: MemberInfoFlags::new(144),
                field_type: SHADERBLOCKRENDERBUFFER_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ShaderParameterBlockDynamicState, render_buffers),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(ShaderParameterBlockDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SHADERPARAMETERBLOCKDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ShaderParameterBlockDynamicState {
    fn type_info() -> &'static TypeInfo {
        SHADERPARAMETERBLOCKDYNAMICSTATE_TYPE_INFO
    }
}


pub const SHADERPARAMETERBLOCKDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterBlockDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ShaderParameterBlockDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShaderBlockRenderBuffer {
    pub name: ShaderBlockParameter,
    pub value: RenderBufferHandle,
}

pub const SHADERBLOCKRENDERBUFFER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderBlockRenderBuffer",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: SHADERBLOCKPARAMETER_TYPE_INFO,
                rust_offset: offset_of!(ShaderBlockRenderBuffer, name),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: RENDERBUFFERHANDLE_TYPE_INFO,
                rust_offset: offset_of!(ShaderBlockRenderBuffer, value),
            },
        ],
    }),
    array_type: Some(SHADERBLOCKRENDERBUFFER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ShaderBlockRenderBuffer {
    fn type_info() -> &'static TypeInfo {
        SHADERBLOCKRENDERBUFFER_TYPE_INFO
    }
}


pub const SHADERBLOCKRENDERBUFFER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderBlockRenderBuffer-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ShaderBlockRenderBuffer-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShaderBlockTexture {
    pub name: ShaderBlockParameter,
    pub value: TextureResourceHandle,
}

pub const SHADERBLOCKTEXTURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderBlockTexture",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: SHADERBLOCKPARAMETER_TYPE_INFO,
                rust_offset: offset_of!(ShaderBlockTexture, name),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTURERESOURCEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(ShaderBlockTexture, value),
            },
        ],
    }),
    array_type: Some(SHADERBLOCKTEXTURE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ShaderBlockTexture {
    fn type_info() -> &'static TypeInfo {
        SHADERBLOCKTEXTURE_TYPE_INFO
    }
}


pub const SHADERBLOCKTEXTURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderBlockTexture-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ShaderBlockTexture-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ShaderBlockVec {
    pub name: ShaderBlockParameter,
    pub values: Vec<super::core::Vec4>,
}

pub const SHADERBLOCKVEC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderBlockVec",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: SHADERBLOCKPARAMETER_TYPE_INFO,
                rust_offset: offset_of!(ShaderBlockVec, name),
            },
            FieldInfoData {
                name: "Values",
                flags: MemberInfoFlags::new(144),
                field_type: VEC4_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ShaderBlockVec, values),
            },
        ],
    }),
    array_type: Some(SHADERBLOCKVEC_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ShaderBlockVec {
    fn type_info() -> &'static TypeInfo {
        SHADERBLOCKVEC_TYPE_INFO
    }
}


pub const SHADERBLOCKVEC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderBlockVec-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ShaderBlockVec-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShaderBlockConditional {
    pub name: ShaderBlockParameter,
    pub value: u8,
}

pub const SHADERBLOCKCONDITIONAL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderBlockConditional",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: SHADERBLOCKPARAMETER_TYPE_INFO,
                rust_offset: offset_of!(ShaderBlockConditional, name),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(ShaderBlockConditional, value),
            },
        ],
    }),
    array_type: Some(SHADERBLOCKCONDITIONAL_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ShaderBlockConditional {
    fn type_info() -> &'static TypeInfo {
        SHADERBLOCKCONDITIONAL_TYPE_INFO
    }
}


pub const SHADERBLOCKCONDITIONAL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderBlockConditional-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ShaderBlockConditional-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShaderBlockBool {
    pub name: ShaderBlockParameter,
    pub value: bool,
}

pub const SHADERBLOCKBOOL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderBlockBool",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: SHADERBLOCKPARAMETER_TYPE_INFO,
                rust_offset: offset_of!(ShaderBlockBool, name),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ShaderBlockBool, value),
            },
        ],
    }),
    array_type: Some(SHADERBLOCKBOOL_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ShaderBlockBool {
    fn type_info() -> &'static TypeInfo {
        SHADERBLOCKBOOL_TYPE_INFO
    }
}


pub const SHADERBLOCKBOOL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderBlockBool-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ShaderBlockBool-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShaderBlockParameter {
    pub part0: u64,
    pub part1: u64,
}

pub const SHADERBLOCKPARAMETER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderBlockParameter",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Part0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT64_TYPE_INFO,
                rust_offset: offset_of!(ShaderBlockParameter, part0),
            },
            FieldInfoData {
                name: "Part1",
                flags: MemberInfoFlags::new(0),
                field_type: UINT64_TYPE_INFO,
                rust_offset: offset_of!(ShaderBlockParameter, part1),
            },
        ],
    }),
    array_type: Some(SHADERBLOCKPARAMETER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ShaderBlockParameter {
    fn type_info() -> &'static TypeInfo {
        SHADERBLOCKPARAMETER_TYPE_INFO
    }
}


pub const SHADERBLOCKPARAMETER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderBlockParameter-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ShaderBlockParameter-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShaderParameterBlockStaticState {
}

pub const SHADERPARAMETERBLOCKSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterBlockStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(SHADERPARAMETERBLOCKSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShaderParameterBlockStaticState {
    fn type_info() -> &'static TypeInfo {
        SHADERPARAMETERBLOCKSTATICSTATE_TYPE_INFO
    }
}


pub const SHADERPARAMETERBLOCKSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterBlockStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ShaderParameterBlockStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ScreenshotDynamicState {
    pub force_auto_render: bool,
    pub trigger_auto_render_delay: i32,
    pub is_frozen: bool,
    pub field_flag_changed0: u8,
}

pub const SCREENSHOTDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenshotDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ForceAutoRender",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotDynamicState, force_auto_render),
            },
            FieldInfoData {
                name: "TriggerAutoRenderDelay",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotDynamicState, trigger_auto_render_delay),
            },
            FieldInfoData {
                name: "IsFrozen",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotDynamicState, is_frozen),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SCREENSHOTDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for ScreenshotDynamicState {
    fn type_info() -> &'static TypeInfo {
        SCREENSHOTDYNAMICSTATE_TYPE_INFO
    }
}


pub const SCREENSHOTDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenshotDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ScreenshotDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ScreenshotState {
    pub explicit_file_name: String,
    pub format: ScreenshotFormat,
    pub layer_mode: ScreenshotLayerMode,
    pub frame_naming_mode: ScreenshotFrameNamingMode,
    pub anti_alias_multiplier: i32,
    pub resolution_multiplier: i32,
    pub png_compression_level: i32,
    pub alpha_enable: bool,
    pub surround_capture: bool,
    pub use_native_file_system: bool,
    pub starting_x_pos: i32,
    pub starting_y_pos: i32,
    pub width: i32,
    pub height: i32,
    pub delay_frames: i32,
    pub upload_to_juice: bool,
    pub overwrite: bool,
    pub auto_render_delay: i32,
    pub frame_padding: u32,
    pub by_frame: u32,
    pub reset_counter: bool,
    pub field_flag_changed0: u32,
}

pub const SCREENSHOTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenshotState",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ExplicitFileName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotState, explicit_file_name),
            },
            FieldInfoData {
                name: "Format",
                flags: MemberInfoFlags::new(0),
                field_type: SCREENSHOTFORMAT_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotState, format),
            },
            FieldInfoData {
                name: "LayerMode",
                flags: MemberInfoFlags::new(0),
                field_type: SCREENSHOTLAYERMODE_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotState, layer_mode),
            },
            FieldInfoData {
                name: "FrameNamingMode",
                flags: MemberInfoFlags::new(0),
                field_type: SCREENSHOTFRAMENAMINGMODE_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotState, frame_naming_mode),
            },
            FieldInfoData {
                name: "AntiAliasMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotState, anti_alias_multiplier),
            },
            FieldInfoData {
                name: "ResolutionMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotState, resolution_multiplier),
            },
            FieldInfoData {
                name: "PngCompressionLevel",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotState, png_compression_level),
            },
            FieldInfoData {
                name: "AlphaEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotState, alpha_enable),
            },
            FieldInfoData {
                name: "SurroundCapture",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotState, surround_capture),
            },
            FieldInfoData {
                name: "UseNativeFileSystem",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotState, use_native_file_system),
            },
            FieldInfoData {
                name: "StartingXPos",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotState, starting_x_pos),
            },
            FieldInfoData {
                name: "StartingYPos",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotState, starting_y_pos),
            },
            FieldInfoData {
                name: "Width",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotState, width),
            },
            FieldInfoData {
                name: "Height",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotState, height),
            },
            FieldInfoData {
                name: "DelayFrames",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotState, delay_frames),
            },
            FieldInfoData {
                name: "UploadToJuice",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotState, upload_to_juice),
            },
            FieldInfoData {
                name: "Overwrite",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotState, overwrite),
            },
            FieldInfoData {
                name: "AutoRenderDelay",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotState, auto_render_delay),
            },
            FieldInfoData {
                name: "FramePadding",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotState, frame_padding),
            },
            FieldInfoData {
                name: "ByFrame",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotState, by_frame),
            },
            FieldInfoData {
                name: "ResetCounter",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotState, reset_counter),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SCREENSHOTSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ScreenshotState {
    fn type_info() -> &'static TypeInfo {
        SCREENSHOTSTATE_TYPE_INFO
    }
}


pub const SCREENSHOTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenshotState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ScreenshotState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ScreenshotFrameNamingMode {
    #[default]
    ScreenshotFrameNamingMode_Sequential = 0,
    ScreenshotFrameNamingMode_Absolute = 1,
}

pub const SCREENSHOTFRAMENAMINGMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenshotFrameNamingMode",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(SCREENSHOTFRAMENAMINGMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ScreenshotFrameNamingMode {
    fn type_info() -> &'static TypeInfo {
        SCREENSHOTFRAMENAMINGMODE_TYPE_INFO
    }
}


pub const SCREENSHOTFRAMENAMINGMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenshotFrameNamingMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ScreenshotFrameNamingMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ScreenshotLayerMode {
    #[default]
    ScreenshotLayerMode_Single = 0,
    ScreenshotLayerMode_Common = 1,
    ScreenshotLayerMode_DLSS = 2,
    ScreenshotLayerMode_All = 3,
}

pub const SCREENSHOTLAYERMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenshotLayerMode",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(SCREENSHOTLAYERMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ScreenshotLayerMode {
    fn type_info() -> &'static TypeInfo {
        SCREENSHOTLAYERMODE_TYPE_INFO
    }
}


pub const SCREENSHOTLAYERMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenshotLayerMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ScreenshotLayerMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ScreenshotFormat {
    #[default]
    ScreenshotFormat_Targa = 0,
    ScreenshotFormat_Png = 1,
    ScreenshotFormat_Png16 = 2,
    ScreenshotFormat_Jpeg = 3,
    ScreenshotFormat_JpegHighCompression = 4,
    ScreenshotFormat_OpenExr = 5,
    ScreenshotFormat_Pfm = 6,
    ScreenshotFormat_RawData = 7,
}

pub const SCREENSHOTFORMAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenshotFormat",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(SCREENSHOTFORMAT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ScreenshotFormat {
    fn type_info() -> &'static TypeInfo {
        SCREENSHOTFORMAT_TYPE_INFO
    }
}


pub const SCREENSHOTFORMAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenshotFormat-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ScreenshotFormat-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ViewDefinition {
    pub view_id: LocalPlayerViewId,
    pub view_type: ViewDefinitionType,
    pub screen_index: u32,
    pub normalized_size: bool,
    pub offset_x: f32,
    pub offset_y: f32,
    pub width: f32,
    pub height: f32,
    pub fov_scale: f32,
}

pub const VIEWDEFINITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ViewDefinition",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ViewId",
                flags: MemberInfoFlags::new(0),
                field_type: LOCALPLAYERVIEWID_TYPE_INFO,
                rust_offset: offset_of!(ViewDefinition, view_id),
            },
            FieldInfoData {
                name: "ViewType",
                flags: MemberInfoFlags::new(0),
                field_type: VIEWDEFINITIONTYPE_TYPE_INFO,
                rust_offset: offset_of!(ViewDefinition, view_type),
            },
            FieldInfoData {
                name: "ScreenIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ViewDefinition, screen_index),
            },
            FieldInfoData {
                name: "NormalizedSize",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ViewDefinition, normalized_size),
            },
            FieldInfoData {
                name: "OffsetX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ViewDefinition, offset_x),
            },
            FieldInfoData {
                name: "OffsetY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ViewDefinition, offset_y),
            },
            FieldInfoData {
                name: "Width",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ViewDefinition, width),
            },
            FieldInfoData {
                name: "Height",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ViewDefinition, height),
            },
            FieldInfoData {
                name: "FovScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ViewDefinition, fov_scale),
            },
        ],
    }),
    array_type: Some(VIEWDEFINITION_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for ViewDefinition {
    fn type_info() -> &'static TypeInfo {
        VIEWDEFINITION_TYPE_INFO
    }
}


pub const VIEWDEFINITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ViewDefinition-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ViewDefinition-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ViewDefinitionType {
    #[default]
    ViewType_FullScreen = 0,
    ViewType_AutoVerticalSplit = 1,
    ViewType_AutoFullHorizontalSplit = 2,
    ViewType_AutoOffsetedHorizontalSplit = 3,
    ViewType_AutoQuadrant = 4,
    ViewType_Custom = 5,
}

pub const VIEWDEFINITIONTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ViewDefinitionType",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(VIEWDEFINITIONTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ViewDefinitionType {
    fn type_info() -> &'static TypeInfo {
        VIEWDEFINITIONTYPE_TYPE_INFO
    }
}


pub const VIEWDEFINITIONTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ViewDefinitionType-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ViewDefinitionType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PhysicalCameraDesc {
    pub exposure_compensation: f32,
    pub e_v100: f32,
    pub shutter_speed: f32,
    pub aperture: f32,
    pub i_s_o: f32,
    pub focal_length: f32,
    pub fov: f32,
    pub focus_distance: f32,
    pub sensor_width: f32,
    pub sensor_height: f32,
    pub spot_meter_scale: f32,
    pub spot_meter_offset: super::core::Vec2,
    pub use_lens_breathing: bool,
    pub use_camera_exposure: bool,
    pub auto_exposure_method: AutoExposureMethod,
}

pub const PHYSICALCAMERADESC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicalCameraDesc",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ExposureCompensation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicalCameraDesc, exposure_compensation),
            },
            FieldInfoData {
                name: "EV100",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicalCameraDesc, e_v100),
            },
            FieldInfoData {
                name: "ShutterSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicalCameraDesc, shutter_speed),
            },
            FieldInfoData {
                name: "Aperture",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicalCameraDesc, aperture),
            },
            FieldInfoData {
                name: "ISO",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicalCameraDesc, i_s_o),
            },
            FieldInfoData {
                name: "FocalLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicalCameraDesc, focal_length),
            },
            FieldInfoData {
                name: "Fov",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicalCameraDesc, fov),
            },
            FieldInfoData {
                name: "FocusDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicalCameraDesc, focus_distance),
            },
            FieldInfoData {
                name: "SensorWidth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicalCameraDesc, sensor_width),
            },
            FieldInfoData {
                name: "SensorHeight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicalCameraDesc, sensor_height),
            },
            FieldInfoData {
                name: "SpotMeterScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PhysicalCameraDesc, spot_meter_scale),
            },
            FieldInfoData {
                name: "SpotMeterOffset",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(PhysicalCameraDesc, spot_meter_offset),
            },
            FieldInfoData {
                name: "UseLensBreathing",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicalCameraDesc, use_lens_breathing),
            },
            FieldInfoData {
                name: "UseCameraExposure",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PhysicalCameraDesc, use_camera_exposure),
            },
            FieldInfoData {
                name: "AutoExposureMethod",
                flags: MemberInfoFlags::new(0),
                field_type: AUTOEXPOSUREMETHOD_TYPE_INFO,
                rust_offset: offset_of!(PhysicalCameraDesc, auto_exposure_method),
            },
        ],
    }),
    array_type: Some(PHYSICALCAMERADESC_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for PhysicalCameraDesc {
    fn type_info() -> &'static TypeInfo {
        PHYSICALCAMERADESC_TYPE_INFO
    }
}


pub const PHYSICALCAMERADESC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicalCameraDesc-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("PhysicalCameraDesc-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum AutoExposureMethod {
    #[default]
    AutoExposureMethod_None = 0,
    AutoExposureMethod_MipAverage = 1,
    AutoExposureMethod_GaussianPyramid = 2,
    AutoExposureMethod_HistogramAverage = 3,
    AutoExposureMethod_UseGlobalSetting = 4,
}

pub const AUTOEXPOSUREMETHOD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoExposureMethod",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(AUTOEXPOSUREMETHOD_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AutoExposureMethod {
    fn type_info() -> &'static TypeInfo {
        AUTOEXPOSUREMETHOD_TYPE_INFO
    }
}


pub const AUTOEXPOSUREMETHOD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoExposureMethod-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("AutoExposureMethod-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RenderViewDynamicState {
    pub transform: super::state_stream::TransformSpaceHandle,
    pub visible: bool,
    pub viewport: ViewportRect,
    pub viewport_offset: super::core::Vec2,
    pub physical_camera_enabled: bool,
    pub physical_camera_desc: PhysicalCameraDesc,
    pub secondary_streaming_view_enabled: bool,
    pub secondary_streaming_view_transform: super::core::LinearTransform,
    pub secondary_streaming_view_fov_rad: f32,
    pub near_plane: f32,
    pub far_plane: f32,
    pub aspect: f32,
    pub fov_rad: f32,
    pub blur_amount: f32,
    pub dof_focus_distance: f32,
    pub shadowmap_min_fov: f32,
    pub shadowmap_distance_scale: f32,
    pub luminance_texture_u_v_scale: f32,
    pub world_fade_amount: f32,
    pub camera_cut_this_frame: bool,
    pub use_physical_camera_changed: bool,
    pub camera_cut_sync_id: u32,
    pub screen_index: u32,
    pub cull_ids: Vec<CullIdHandle>,
    pub render_view_feature_mask: u32,
    pub field_flag_changed0: u32,
}

pub const RENDERVIEWDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderViewDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: TRANSFORMSPACEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(RenderViewDynamicState, transform),
            },
            FieldInfoData {
                name: "Visible",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RenderViewDynamicState, visible),
            },
            FieldInfoData {
                name: "Viewport",
                flags: MemberInfoFlags::new(0),
                field_type: VIEWPORTRECT_TYPE_INFO,
                rust_offset: offset_of!(RenderViewDynamicState, viewport),
            },
            FieldInfoData {
                name: "ViewportOffset",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(RenderViewDynamicState, viewport_offset),
            },
            FieldInfoData {
                name: "PhysicalCameraEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RenderViewDynamicState, physical_camera_enabled),
            },
            FieldInfoData {
                name: "PhysicalCameraDesc",
                flags: MemberInfoFlags::new(0),
                field_type: PHYSICALCAMERADESC_TYPE_INFO,
                rust_offset: offset_of!(RenderViewDynamicState, physical_camera_desc),
            },
            FieldInfoData {
                name: "SecondaryStreamingViewEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RenderViewDynamicState, secondary_streaming_view_enabled),
            },
            FieldInfoData {
                name: "SecondaryStreamingViewTransform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(RenderViewDynamicState, secondary_streaming_view_transform),
            },
            FieldInfoData {
                name: "SecondaryStreamingViewFovRad",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RenderViewDynamicState, secondary_streaming_view_fov_rad),
            },
            FieldInfoData {
                name: "NearPlane",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RenderViewDynamicState, near_plane),
            },
            FieldInfoData {
                name: "FarPlane",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RenderViewDynamicState, far_plane),
            },
            FieldInfoData {
                name: "Aspect",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RenderViewDynamicState, aspect),
            },
            FieldInfoData {
                name: "FovRad",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RenderViewDynamicState, fov_rad),
            },
            FieldInfoData {
                name: "BlurAmount",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RenderViewDynamicState, blur_amount),
            },
            FieldInfoData {
                name: "DofFocusDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RenderViewDynamicState, dof_focus_distance),
            },
            FieldInfoData {
                name: "ShadowmapMinFov",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RenderViewDynamicState, shadowmap_min_fov),
            },
            FieldInfoData {
                name: "ShadowmapDistanceScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RenderViewDynamicState, shadowmap_distance_scale),
            },
            FieldInfoData {
                name: "LuminanceTextureUVScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RenderViewDynamicState, luminance_texture_u_v_scale),
            },
            FieldInfoData {
                name: "WorldFadeAmount",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RenderViewDynamicState, world_fade_amount),
            },
            FieldInfoData {
                name: "CameraCutThisFrame",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RenderViewDynamicState, camera_cut_this_frame),
            },
            FieldInfoData {
                name: "UsePhysicalCameraChanged",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RenderViewDynamicState, use_physical_camera_changed),
            },
            FieldInfoData {
                name: "CameraCutSyncId",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(RenderViewDynamicState, camera_cut_sync_id),
            },
            FieldInfoData {
                name: "ScreenIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(RenderViewDynamicState, screen_index),
            },
            FieldInfoData {
                name: "CullIds",
                flags: MemberInfoFlags::new(144),
                field_type: CULLIDHANDLE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(RenderViewDynamicState, cull_ids),
            },
            FieldInfoData {
                name: "RenderViewFeatureMask",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(RenderViewDynamicState, render_view_feature_mask),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(RenderViewDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(RENDERVIEWDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RenderViewDynamicState {
    fn type_info() -> &'static TypeInfo {
        RENDERVIEWDYNAMICSTATE_TYPE_INFO
    }
}


pub const RENDERVIEWDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderViewDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("RenderViewDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RenderViewStaticState {
    pub priority: u32,
    pub target_texture: RenderTextureBaseAsset,
    pub debug_name: String,
    pub field_flag_changed0: u8,
}

pub const RENDERVIEWSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderViewStaticState",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Priority",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(RenderViewStaticState, priority),
            },
            FieldInfoData {
                name: "TargetTexture",
                flags: MemberInfoFlags::new(0),
                field_type: RENDERTEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(RenderViewStaticState, target_texture),
            },
            FieldInfoData {
                name: "DebugName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(RenderViewStaticState, debug_name),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(RenderViewStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(RENDERVIEWSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RenderViewStaticState {
    fn type_info() -> &'static TypeInfo {
        RENDERVIEWSTATICSTATE_TYPE_INFO
    }
}


pub const RENDERVIEWSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderViewStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("RenderViewStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ScissorRect {
}

pub const SCISSORRECT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScissorRect",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(SCISSORRECT_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for ScissorRect {
    fn type_info() -> &'static TypeInfo {
        SCISSORRECT_TYPE_INFO
    }
}


pub const SCISSORRECT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScissorRect-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ScissorRect-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ViewportRect {
}

pub const VIEWPORTRECT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ViewportRect",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(VIEWPORTRECT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ViewportRect {
    fn type_info() -> &'static TypeInfo {
        VIEWPORTRECT_TYPE_INFO
    }
}


pub const VIEWPORTRECT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ViewportRect-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ViewportRect-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MeshComputeBaseNode {
}

pub const MESHCOMPUTEBASENODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshComputeBaseNode",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(NODE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(MESHCOMPUTEBASENODE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshComputeBaseNode {
    fn type_info() -> &'static TypeInfo {
        MESHCOMPUTEBASENODE_TYPE_INFO
    }
}


pub const MESHCOMPUTEBASENODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshComputeBaseNode-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("MeshComputeBaseNode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MeshComputeBaseAsset {
}

pub const MESHCOMPUTEBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshComputeBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(NODEGRAPH_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(MESHCOMPUTEBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshComputeBaseAsset {
    fn type_info() -> &'static TypeInfo {
        MESHCOMPUTEBASEASSET_TYPE_INFO
    }
}


pub const MESHCOMPUTEBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshComputeBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("MeshComputeBaseAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StaticEnlightenBaseAsset {
}

pub const STATICENLIGHTENBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticEnlightenBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(STATICENLIGHTENBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StaticEnlightenBaseAsset {
    fn type_info() -> &'static TypeInfo {
        STATICENLIGHTENBASEASSET_TYPE_INFO
    }
}


pub const STATICENLIGHTENBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticEnlightenBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("StaticEnlightenBaseAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EnlightenRuntimeConfigBaseAsset {
}

pub const ENLIGHTENRUNTIMECONFIGBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenRuntimeConfigBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ENLIGHTENRUNTIMECONFIGBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EnlightenRuntimeConfigBaseAsset {
    fn type_info() -> &'static TypeInfo {
        ENLIGHTENRUNTIMECONFIGBASEASSET_TYPE_INFO
    }
}


pub const ENLIGHTENRUNTIMECONFIGBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenRuntimeConfigBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("EnlightenRuntimeConfigBaseAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EnlightenShaderDatabaseBaseAsset {
}

pub const ENLIGHTENSHADERDATABASEBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenShaderDatabaseBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ENLIGHTENSHADERDATABASEBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EnlightenShaderDatabaseBaseAsset {
    fn type_info() -> &'static TypeInfo {
        ENLIGHTENSHADERDATABASEBASEASSET_TYPE_INFO
    }
}


pub const ENLIGHTENSHADERDATABASEBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenShaderDatabaseBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("EnlightenShaderDatabaseBaseAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EnlightenBaseAsset {
    pub mix_feature_mode: RadiosityMixMode,
}

pub const ENLIGHTENBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MixFeatureMode",
                flags: MemberInfoFlags::new(0),
                field_type: RADIOSITYMIXMODE_TYPE_INFO,
                rust_offset: offset_of!(EnlightenBaseAsset, mix_feature_mode),
            },
        ],
    }),
    array_type: Some(ENLIGHTENBASEASSET_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EnlightenBaseAsset {
    fn type_info() -> &'static TypeInfo {
        ENLIGHTENBASEASSET_TYPE_INFO
    }
}


pub const ENLIGHTENBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("EnlightenBaseAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MeshLodGroupBaseAsset {
}

pub const MESHLODGROUPBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshLodGroupBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(MESHLODGROUPBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshLodGroupBaseAsset {
    fn type_info() -> &'static TypeInfo {
        MESHLODGROUPBASEASSET_TYPE_INFO
    }
}


pub const MESHLODGROUPBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshLodGroupBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("MeshLodGroupBaseAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MeshBaseAsset {
}

pub const MESHBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(MESHBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshBaseAsset {
    fn type_info() -> &'static TypeInfo {
        MESHBASEASSET_TYPE_INFO
    }
}


pub const MESHBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("MeshBaseAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum RadiosityMixMode {
    #[default]
    RadiosityMixMode_Disabled = 0,
    RadiosityMixMode_EnableWithPermutations = 1,
    RadiosityMixMode_EnableWithoutPermutations = 2,
}

pub const RADIOSITYMIXMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityMixMode",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(RADIOSITYMIXMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RadiosityMixMode {
    fn type_info() -> &'static TypeInfo {
        RADIOSITYMIXMODE_TYPE_INFO
    }
}


pub const RADIOSITYMIXMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityMixMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("RadiosityMixMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum RadiosityBackfaceType {
    #[default]
    RadiosityBackfaceType_Invalid = 0,
    RadiosityBackfaceType_Black = 1,
    RadiosityBackfaceType_Translucent = 2,
    RadiosityBackfaceType_Transparent = 3,
}

pub const RADIOSITYBACKFACETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityBackfaceType",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(RADIOSITYBACKFACETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RadiosityBackfaceType {
    fn type_info() -> &'static TypeInfo {
        RADIOSITYBACKFACETYPE_TYPE_INFO
    }
}


pub const RADIOSITYBACKFACETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityBackfaceType-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("RadiosityBackfaceType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum MeshType {
    #[default]
    MeshType_Rigid = 0,
    MeshType_Skinned = 1,
    MeshType_Composite = 2,
    MeshType_Count = 3,
}

pub const MESHTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshType",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(MESHTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MeshType {
    fn type_info() -> &'static TypeInfo {
        MESHTYPE_TYPE_INFO
    }
}


pub const MESHTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshType-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("MeshType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum CloudShadowType {
    #[default]
    CloudShadowType_Disabled = 0,
    CloudShadowType_Texture = 1,
    CloudShadowType_Volumetric = 2,
    CloudShadowTypeCount = 3,
}

pub const CLOUDSHADOWTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CloudShadowType",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(CLOUDSHADOWTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CloudShadowType {
    fn type_info() -> &'static TypeInfo {
        CLOUDSHADOWTYPE_TYPE_INFO
    }
}


pub const CLOUDSHADOWTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CloudShadowType-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("CloudShadowType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum TextureAddress {
    #[default]
    TaWrap = 0,
    TaMirror = 1,
    TaClamp = 2,
    TaBorder = 4,
    TaMirrorOnce = 5,
}

pub const TEXTUREADDRESS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureAddress",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(TEXTUREADDRESS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TextureAddress {
    fn type_info() -> &'static TypeInfo {
        TEXTUREADDRESS_TYPE_INFO
    }
}


pub const TEXTUREADDRESS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureAddress-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("TextureAddress-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RenderTextureBaseAsset {
}

pub const RENDERTEXTUREBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderTextureBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TEXTUREBASEASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(RENDERTEXTUREBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RenderTextureBaseAsset {
    fn type_info() -> &'static TypeInfo {
        RENDERTEXTUREBASEASSET_TYPE_INFO
    }
}


pub const RENDERTEXTUREBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderTextureBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("RenderTextureBaseAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IPGraphCrossReferenceTextureAsset {
    pub source_overlay_id: u32,
}

pub const IPGRAPHCROSSREFERENCETEXTUREASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IPGraphCrossReferenceTextureAsset",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TEXTUREBASEASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "SourceOverlayId",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(IPGraphCrossReferenceTextureAsset, source_overlay_id),
            },
        ],
    }),
    array_type: Some(IPGRAPHCROSSREFERENCETEXTUREASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IPGraphCrossReferenceTextureAsset {
    fn type_info() -> &'static TypeInfo {
        IPGRAPHCROSSREFERENCETEXTUREASSET_TYPE_INFO
    }
}


pub const IPGRAPHCROSSREFERENCETEXTUREASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IPGraphCrossReferenceTextureAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("IPGraphCrossReferenceTextureAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TextureBaseAsset {
    pub resource: super::core::ResourceRef,
}

pub const TEXTUREBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Resource",
                flags: MemberInfoFlags::new(0),
                field_type: RESOURCEREF_TYPE_INFO,
                rust_offset: offset_of!(TextureBaseAsset, resource),
            },
        ],
    }),
    array_type: Some(TEXTUREBASEASSET_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TextureBaseAsset {
    fn type_info() -> &'static TypeInfo {
        TEXTUREBASEASSET_TYPE_INFO
    }
}


pub const TEXTUREBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("TextureBaseAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PerformanceSummaryMessage {
}

pub const PERFORMANCESUMMARYMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerformanceSummaryMessage",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for PerformanceSummaryMessage {
    fn type_info() -> &'static TypeInfo {
        PERFORMANCESUMMARYMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PerformanceClientMessage {
}

pub const PERFORMANCECLIENTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerformanceClientMessage",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for PerformanceClientMessage {
    fn type_info() -> &'static TypeInfo {
        PERFORMANCECLIENTMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PerformanceShaderMessage {
}

pub const PERFORMANCESHADERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerformanceShaderMessage",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for PerformanceShaderMessage {
    fn type_info() -> &'static TypeInfo {
        PERFORMANCESHADERMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShaderStatsMessage {
}

pub const SHADERSTATSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderStatsMessage",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(SHADERSTATSMESSAGE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for ShaderStatsMessage {
    fn type_info() -> &'static TypeInfo {
        SHADERSTATSMESSAGE_TYPE_INFO
    }
}


pub const SHADERSTATSMESSAGE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderStatsMessage-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ShaderStatsMessage-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PerformanceMessage {
}

pub const PERFORMANCEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerformanceMessage",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(PERFORMANCEMESSAGE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for PerformanceMessage {
    fn type_info() -> &'static TypeInfo {
        PERFORMANCEMESSAGE_TYPE_INFO
    }
}


pub const PERFORMANCEMESSAGE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerformanceMessage-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("PerformanceMessage-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RenderBufferDynamicState {
}

pub const RENDERBUFFERDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderBufferDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RENDERBUFFERDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RenderBufferDynamicState {
    fn type_info() -> &'static TypeInfo {
        RENDERBUFFERDYNAMICSTATE_TYPE_INFO
    }
}


pub const RENDERBUFFERDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderBufferDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("RenderBufferDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RenderBufferStaticState {
    pub element_count: i32,
    pub element_size: i32,
    pub field_flag_changed0: u8,
}

pub const RENDERBUFFERSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderBufferStaticState",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ElementCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(RenderBufferStaticState, element_count),
            },
            FieldInfoData {
                name: "ElementSize",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(RenderBufferStaticState, element_size),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(RenderBufferStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(RENDERBUFFERSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RenderBufferStaticState {
    fn type_info() -> &'static TypeInfo {
        RENDERBUFFERSTATICSTATE_TYPE_INFO
    }
}


pub const RENDERBUFFERSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderBufferStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("RenderBufferStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LensReflectionComponentState {
    pub enable: bool,
    pub inner_color: super::core::Vec3,
    pub outer_color: super::core::Vec3,
    pub mix_start: f32,
    pub mix_stop: f32,
    pub input_exponent: f32,
    pub luminance_threshold: f32,
    pub input_scale: f32,
    pub max_opacity: f32,
    pub transpose_reflection: bool,
    pub scale: f32,
    pub distortion_factor: f32,
    pub vertical_stretch: f32,
    pub field_flag_override0: u16,
    pub field_flag_changed0: u16,
}

pub const LENSREFLECTIONCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensReflectionComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LensReflectionComponentState, enable),
            },
            FieldInfoData {
                name: "InnerColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(LensReflectionComponentState, inner_color),
            },
            FieldInfoData {
                name: "OuterColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(LensReflectionComponentState, outer_color),
            },
            FieldInfoData {
                name: "MixStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LensReflectionComponentState, mix_start),
            },
            FieldInfoData {
                name: "MixStop",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LensReflectionComponentState, mix_stop),
            },
            FieldInfoData {
                name: "InputExponent",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LensReflectionComponentState, input_exponent),
            },
            FieldInfoData {
                name: "LuminanceThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LensReflectionComponentState, luminance_threshold),
            },
            FieldInfoData {
                name: "InputScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LensReflectionComponentState, input_scale),
            },
            FieldInfoData {
                name: "MaxOpacity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LensReflectionComponentState, max_opacity),
            },
            FieldInfoData {
                name: "TransposeReflection",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LensReflectionComponentState, transpose_reflection),
            },
            FieldInfoData {
                name: "Scale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LensReflectionComponentState, scale),
            },
            FieldInfoData {
                name: "DistortionFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LensReflectionComponentState, distortion_factor),
            },
            FieldInfoData {
                name: "VerticalStretch",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LensReflectionComponentState, vertical_stretch),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(LensReflectionComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(LensReflectionComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(LENSREFLECTIONCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LensReflectionComponentState {
    fn type_info() -> &'static TypeInfo {
        LENSREFLECTIONCOMPONENTSTATE_TYPE_INFO
    }
}


pub const LENSREFLECTIONCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensReflectionComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("LensReflectionComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum LensReflectionComponentCount {
    #[default]
    MaxLensReflectionComponentCount = 4,
}

pub const LENSREFLECTIONCOMPONENTCOUNT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensReflectionComponentCount",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(LENSREFLECTIONCOMPONENTCOUNT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LensReflectionComponentCount {
    fn type_info() -> &'static TypeInfo {
        LENSREFLECTIONCOMPONENTCOUNT_TYPE_INFO
    }
}


pub const LENSREFLECTIONCOMPONENTCOUNT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensReflectionComponentCount-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("LensReflectionComponentCount-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FilmicEffectsComponentState {
    pub enable: bool,
    pub enable_chromatic_abberation: bool,
    pub chromatic_abberation_scale: f32,
    pub chromatic_abberation_aspect_ratio: f32,
    pub enable_vignetting: bool,
    pub vignetting_falloff: f32,
    pub vignetting_luminance_percent: f32,
    pub enable_lens_distortion: bool,
    pub lens_distortion_gain: f32,
    pub lens_distortion_cubic_gain: f32,
    pub lens_distortion_stretch: f32,
    pub enable_frame_flash: bool,
    pub frame_flash_gain: f32,
    pub enable_depth_flash: bool,
    pub depth_flash_atmos_color: super::core::Vec3,
    pub depth_flash_half_distance: f32,
    pub enable_distance_blur: bool,
    pub distance_blur_gain: f32,
    pub distance_blur_half_distance: f32,
    pub enable_edge_blur: bool,
    pub edge_blur_gain: f32,
    pub edge_blur_depth_target_scale: f32,
    pub edge_blur_fade_near_depth: f32,
    pub edge_blur_fade_far_depth: f32,
    pub edge_blur_matte_dilate_size: f32,
    pub edge_blur_matte_blur_kernel_size: f32,
    pub enable_heat_ripple: bool,
    pub heat_ripple_gain: f32,
    pub heat_ripple_horizontal_speed: f32,
    pub heat_ripple_vertical_speed: f32,
    pub heat_ripple_noise_scale: f32,
    pub heat_ripple_near_distance: f32,
    pub heat_ripple_far_distance: f32,
    pub heat_ripple_near_gain: f32,
    pub heat_ripple_far_gain: f32,
    pub heat_ripple_texture: TextureBaseAsset,
    pub field_flag_override0: u32,
    pub field_flag_override1: u8,
    pub field_flag_changed0: u32,
    pub field_flag_changed1: u8,
}

pub const FILMICEFFECTSCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FilmicEffectsComponentState",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentState, enable),
            },
            FieldInfoData {
                name: "EnableChromaticAbberation",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentState, enable_chromatic_abberation),
            },
            FieldInfoData {
                name: "ChromaticAbberationScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentState, chromatic_abberation_scale),
            },
            FieldInfoData {
                name: "ChromaticAbberationAspectRatio",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentState, chromatic_abberation_aspect_ratio),
            },
            FieldInfoData {
                name: "EnableVignetting",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentState, enable_vignetting),
            },
            FieldInfoData {
                name: "VignettingFalloff",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentState, vignetting_falloff),
            },
            FieldInfoData {
                name: "VignettingLuminancePercent",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentState, vignetting_luminance_percent),
            },
            FieldInfoData {
                name: "EnableLensDistortion",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentState, enable_lens_distortion),
            },
            FieldInfoData {
                name: "LensDistortionGain",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentState, lens_distortion_gain),
            },
            FieldInfoData {
                name: "LensDistortionCubicGain",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentState, lens_distortion_cubic_gain),
            },
            FieldInfoData {
                name: "LensDistortionStretch",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentState, lens_distortion_stretch),
            },
            FieldInfoData {
                name: "EnableFrameFlash",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentState, enable_frame_flash),
            },
            FieldInfoData {
                name: "FrameFlashGain",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentState, frame_flash_gain),
            },
            FieldInfoData {
                name: "EnableDepthFlash",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentState, enable_depth_flash),
            },
            FieldInfoData {
                name: "DepthFlashAtmosColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentState, depth_flash_atmos_color),
            },
            FieldInfoData {
                name: "DepthFlashHalfDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentState, depth_flash_half_distance),
            },
            FieldInfoData {
                name: "EnableDistanceBlur",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentState, enable_distance_blur),
            },
            FieldInfoData {
                name: "DistanceBlurGain",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentState, distance_blur_gain),
            },
            FieldInfoData {
                name: "DistanceBlurHalfDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentState, distance_blur_half_distance),
            },
            FieldInfoData {
                name: "EnableEdgeBlur",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentState, enable_edge_blur),
            },
            FieldInfoData {
                name: "EdgeBlurGain",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentState, edge_blur_gain),
            },
            FieldInfoData {
                name: "EdgeBlurDepthTargetScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentState, edge_blur_depth_target_scale),
            },
            FieldInfoData {
                name: "EdgeBlurFadeNearDepth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentState, edge_blur_fade_near_depth),
            },
            FieldInfoData {
                name: "EdgeBlurFadeFarDepth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentState, edge_blur_fade_far_depth),
            },
            FieldInfoData {
                name: "EdgeBlurMatteDilateSize",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentState, edge_blur_matte_dilate_size),
            },
            FieldInfoData {
                name: "EdgeBlurMatteBlurKernelSize",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentState, edge_blur_matte_blur_kernel_size),
            },
            FieldInfoData {
                name: "EnableHeatRipple",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentState, enable_heat_ripple),
            },
            FieldInfoData {
                name: "HeatRippleGain",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentState, heat_ripple_gain),
            },
            FieldInfoData {
                name: "HeatRippleHorizontalSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentState, heat_ripple_horizontal_speed),
            },
            FieldInfoData {
                name: "HeatRippleVerticalSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentState, heat_ripple_vertical_speed),
            },
            FieldInfoData {
                name: "HeatRippleNoiseScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentState, heat_ripple_noise_scale),
            },
            FieldInfoData {
                name: "HeatRippleNearDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentState, heat_ripple_near_distance),
            },
            FieldInfoData {
                name: "HeatRippleFarDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentState, heat_ripple_far_distance),
            },
            FieldInfoData {
                name: "HeatRippleNearGain",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentState, heat_ripple_near_gain),
            },
            FieldInfoData {
                name: "HeatRippleFarGain",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentState, heat_ripple_far_gain),
            },
            FieldInfoData {
                name: "HeatRippleTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentState, heat_ripple_texture),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagOverride1",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentState, field_flag_override1),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentState, field_flag_changed0),
            },
            FieldInfoData {
                name: "FieldFlagChanged1",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentState, field_flag_changed1),
            },
        ],
    }),
    array_type: Some(FILMICEFFECTSCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FilmicEffectsComponentState {
    fn type_info() -> &'static TypeInfo {
        FILMICEFFECTSCOMPONENTSTATE_TYPE_INFO
    }
}


pub const FILMICEFFECTSCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FilmicEffectsComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("FilmicEffectsComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DebugComponentState {
    pub enable: bool,
    pub fullscreen: bool,
    pub debug_texture: TextureBaseAsset,
    pub field_flag_override0: u8,
    pub field_flag_changed0: u8,
}

pub const DEBUGCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebugComponentState",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DebugComponentState, enable),
            },
            FieldInfoData {
                name: "Fullscreen",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DebugComponentState, fullscreen),
            },
            FieldInfoData {
                name: "DebugTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(DebugComponentState, debug_texture),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(DebugComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(DebugComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(DEBUGCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DebugComponentState {
    fn type_info() -> &'static TypeInfo {
        DEBUGCOMPONENTSTATE_TYPE_INFO
    }
}


pub const DEBUGCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebugComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("DebugComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LensScopeComponentState {
    pub enable: bool,
    pub blur_scale: f32,
    pub blur_center: super::core::Vec2,
    pub chromatic_aberration_color1: super::core::Vec3,
    pub chromatic_aberration_color2: super::core::Vec3,
    pub chromatic_aberration_strengths: super::core::Vec2,
    pub chromatic_aberration_displacement1: super::core::Vec2,
    pub chromatic_aberration_displacement2: super::core::Vec2,
    pub radial_blend_distance_coefficients: super::core::Vec2,
    pub field_flag_override0: u16,
    pub field_flag_changed0: u16,
}

pub const LENSSCOPECOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensScopeComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LensScopeComponentState, enable),
            },
            FieldInfoData {
                name: "BlurScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LensScopeComponentState, blur_scale),
            },
            FieldInfoData {
                name: "BlurCenter",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(LensScopeComponentState, blur_center),
            },
            FieldInfoData {
                name: "ChromaticAberrationColor1",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(LensScopeComponentState, chromatic_aberration_color1),
            },
            FieldInfoData {
                name: "ChromaticAberrationColor2",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(LensScopeComponentState, chromatic_aberration_color2),
            },
            FieldInfoData {
                name: "ChromaticAberrationStrengths",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(LensScopeComponentState, chromatic_aberration_strengths),
            },
            FieldInfoData {
                name: "ChromaticAberrationDisplacement1",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(LensScopeComponentState, chromatic_aberration_displacement1),
            },
            FieldInfoData {
                name: "ChromaticAberrationDisplacement2",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(LensScopeComponentState, chromatic_aberration_displacement2),
            },
            FieldInfoData {
                name: "RadialBlendDistanceCoefficients",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(LensScopeComponentState, radial_blend_distance_coefficients),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(LensScopeComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(LensScopeComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(LENSSCOPECOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LensScopeComponentState {
    fn type_info() -> &'static TypeInfo {
        LENSSCOPECOMPONENTSTATE_TYPE_INFO
    }
}


pub const LENSSCOPECOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensScopeComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("LensScopeComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FilmGrainComponentState {
    pub enable: bool,
    pub texture_scale: super::core::Vec2,
    pub color_scale: super::core::Vec3,
    pub linear_filtering_enable: bool,
    pub random_enable: bool,
    pub texture: TextureBaseAsset,
    pub grain_grey_fraction: f32,
    pub grain_luminance_control_enable: bool,
    pub grain_shadow_threshold: f32,
    pub grain_highlight_threshold: f32,
    pub grain_shadow_intensity: f32,
    pub grain_highlight_intensity: f32,
    pub field_flag_override0: u16,
    pub field_flag_changed0: u16,
}

pub const FILMGRAINCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FilmGrainComponentState",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FilmGrainComponentState, enable),
            },
            FieldInfoData {
                name: "TextureScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(FilmGrainComponentState, texture_scale),
            },
            FieldInfoData {
                name: "ColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(FilmGrainComponentState, color_scale),
            },
            FieldInfoData {
                name: "LinearFilteringEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FilmGrainComponentState, linear_filtering_enable),
            },
            FieldInfoData {
                name: "RandomEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FilmGrainComponentState, random_enable),
            },
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(FilmGrainComponentState, texture),
            },
            FieldInfoData {
                name: "GrainGreyFraction",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmGrainComponentState, grain_grey_fraction),
            },
            FieldInfoData {
                name: "GrainLuminanceControlEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FilmGrainComponentState, grain_luminance_control_enable),
            },
            FieldInfoData {
                name: "GrainShadowThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmGrainComponentState, grain_shadow_threshold),
            },
            FieldInfoData {
                name: "GrainHighlightThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmGrainComponentState, grain_highlight_threshold),
            },
            FieldInfoData {
                name: "GrainShadowIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmGrainComponentState, grain_shadow_intensity),
            },
            FieldInfoData {
                name: "GrainHighlightIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmGrainComponentState, grain_highlight_intensity),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(FilmGrainComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(FilmGrainComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(FILMGRAINCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FilmGrainComponentState {
    fn type_info() -> &'static TypeInfo {
        FILMGRAINCOMPONENTSTATE_TYPE_INFO
    }
}


pub const FILMGRAINCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FilmGrainComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("FilmGrainComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct VignetteComponentState {
    pub enable: bool,
    pub scale: super::core::Vec2,
    pub exponent: f32,
    pub color: super::core::Vec3,
    pub opacity: f32,
    pub field_flag_override0: u8,
    pub field_flag_changed0: u8,
}

pub const VIGNETTECOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VignetteComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VignetteComponentState, enable),
            },
            FieldInfoData {
                name: "Scale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(VignetteComponentState, scale),
            },
            FieldInfoData {
                name: "Exponent",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VignetteComponentState, exponent),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(VignetteComponentState, color),
            },
            FieldInfoData {
                name: "Opacity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VignetteComponentState, opacity),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(VignetteComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(VignetteComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(VIGNETTECOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for VignetteComponentState {
    fn type_info() -> &'static TypeInfo {
        VIGNETTECOMPONENTSTATE_TYPE_INFO
    }
}


pub const VIGNETTECOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VignetteComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("VignetteComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DofComponentState {
    pub enable: bool,
    pub physical_camera_tweak_enable: bool,
    pub pbc_background_blur_add: f32,
    pub pbc_foreground_blur_add: f32,
    pub pbc_focus_range_add: f32,
    pub dof_source: DofSource,
    pub debug_draw_focus_plane: bool,
    pub focus_dof_max_blur: f32,
    pub blur_factor: f32,
    pub blur_add: f32,
    pub focus_distance: f32,
    pub radial_blur_enable_common_dof: bool,
    pub radial_blur_amount_common_dof: f32,
    pub radial_blur_start_radius_common_dof: f32,
    pub radial_blur_transition_width_common_dof: f32,
    pub radial_blur_tilt_common_dof: f32,
    pub radial_blur_horizontal_scale_common_dof: f32,
    pub radial_blur_aspect_ratio_blend: f32,
    pub radial_blur_position_common_dof: super::core::Vec2,
    pub simple_dof_blur_filter: BlurFilter,
    pub simple_dof_standard_deviation: f32,
    pub sprite_dof_bokeh_texture: TextureBaseAsset,
    pub focus_dof_near_start: f32,
    pub focus_dof_near_end: f32,
    pub focus_dof_far_start: f32,
    pub focus_dof_far_end: f32,
    pub pbr_focus_length_dof: f32,
    pub pbr_film_width_dof: f32,
    pub pbr_f_stop_dof: f32,
    pub optical_vignetting_enable: bool,
    pub optical_vignetting_amount: f32,
    pub optical_vignetting_aspect_ratio: f32,
    pub optical_vignetting_anamorphic_squeeze: f32,
    pub optical_vignetting_size_compensation: f32,
    pub optical_vignetting_operation: VignettingOperation,
    pub r_g_b_bokeh_texture_enable: bool,
    pub bokeh_chromatic_aberration_enable: bool,
    pub bokeh_chromatic_aberration_scale: f32,
    pub bokeh_chromatic_aberration_radius: f32,
    pub bokeh_chromatic_aberration_width: f32,
    pub bokeh_chromatic_aberration_radius_threshold: f32,
    pub bokeh_chromatic_aberration_radius_threshold_width: f32,
    pub bokeh_chromatic_aberration_energy_threshold: f32,
    pub bokeh_chromatic_aberration_fg_color: super::core::Vec3,
    pub bokeh_chromatic_aberration_bg_color: super::core::Vec3,
    pub ironsights_dof_active: bool,
    pub ironsights_dof_extra_blur: bool,
    pub hip_to_ironsights_fade: f32,
    pub ironsights_dof_start_fade: f32,
    pub ironsights_focal_distance: f32,
    pub ironsights_dof_circle_blur: bool,
    pub ironsights_dof_circle_distance: f32,
    pub ironsights_dof_circle_fade_distance: f32,
    pub masked_blur_enabled: bool,
    pub masked_blur_amount: f32,
    pub masked_blur_texture: TextureBaseAsset,
    pub circular_dof_anti_band_artifact: bool,
    pub use_camera_settings: bool,
    pub simple_dof_max_blur: f32,
    pub simple_dof_near_start: f32,
    pub simple_dof_near_end: f32,
    pub simple_dof_far_start: f32,
    pub simple_dof_far_end: f32,
    pub sprite_dof_near_start: f32,
    pub sprite_dof_near_end: f32,
    pub sprite_dof_far_start: f32,
    pub sprite_dof_far_end: f32,
    pub sprite_dof_max_blur: f32,
    pub anisotropy: f32,
    pub full_screen_blur_add_common_dof: f32,
    pub field_flag_override0: u32,
    pub field_flag_override1: u32,
    pub field_flag_override2: u8,
    pub field_flag_changed0: u32,
    pub field_flag_changed1: u32,
    pub field_flag_changed2: u16,
}

pub const DOFCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DofComponentState",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, enable),
            },
            FieldInfoData {
                name: "PhysicalCameraTweakEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, physical_camera_tweak_enable),
            },
            FieldInfoData {
                name: "PbcBackgroundBlurAdd",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, pbc_background_blur_add),
            },
            FieldInfoData {
                name: "PbcForegroundBlurAdd",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, pbc_foreground_blur_add),
            },
            FieldInfoData {
                name: "PbcFocusRangeAdd",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, pbc_focus_range_add),
            },
            FieldInfoData {
                name: "DofSource",
                flags: MemberInfoFlags::new(0),
                field_type: DOFSOURCE_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, dof_source),
            },
            FieldInfoData {
                name: "DebugDrawFocusPlane",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, debug_draw_focus_plane),
            },
            FieldInfoData {
                name: "FocusDofMaxBlur",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, focus_dof_max_blur),
            },
            FieldInfoData {
                name: "BlurFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, blur_factor),
            },
            FieldInfoData {
                name: "BlurAdd",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, blur_add),
            },
            FieldInfoData {
                name: "FocusDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, focus_distance),
            },
            FieldInfoData {
                name: "RadialBlurEnableCommonDof",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, radial_blur_enable_common_dof),
            },
            FieldInfoData {
                name: "RadialBlurAmountCommonDof",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, radial_blur_amount_common_dof),
            },
            FieldInfoData {
                name: "RadialBlurStartRadiusCommonDof",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, radial_blur_start_radius_common_dof),
            },
            FieldInfoData {
                name: "RadialBlurTransitionWidthCommonDof",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, radial_blur_transition_width_common_dof),
            },
            FieldInfoData {
                name: "RadialBlurTiltCommonDof",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, radial_blur_tilt_common_dof),
            },
            FieldInfoData {
                name: "RadialBlurHorizontalScaleCommonDof",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, radial_blur_horizontal_scale_common_dof),
            },
            FieldInfoData {
                name: "RadialBlurAspectRatioBlend",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, radial_blur_aspect_ratio_blend),
            },
            FieldInfoData {
                name: "RadialBlurPositionCommonDof",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, radial_blur_position_common_dof),
            },
            FieldInfoData {
                name: "SimpleDofBlurFilter",
                flags: MemberInfoFlags::new(0),
                field_type: BLURFILTER_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, simple_dof_blur_filter),
            },
            FieldInfoData {
                name: "SimpleDofStandardDeviation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, simple_dof_standard_deviation),
            },
            FieldInfoData {
                name: "SpriteDofBokehTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, sprite_dof_bokeh_texture),
            },
            FieldInfoData {
                name: "FocusDofNearStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, focus_dof_near_start),
            },
            FieldInfoData {
                name: "FocusDofNearEnd",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, focus_dof_near_end),
            },
            FieldInfoData {
                name: "FocusDofFarStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, focus_dof_far_start),
            },
            FieldInfoData {
                name: "FocusDofFarEnd",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, focus_dof_far_end),
            },
            FieldInfoData {
                name: "PbrFocusLengthDof",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, pbr_focus_length_dof),
            },
            FieldInfoData {
                name: "PbrFilmWidthDof",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, pbr_film_width_dof),
            },
            FieldInfoData {
                name: "PbrFStopDof",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, pbr_f_stop_dof),
            },
            FieldInfoData {
                name: "OpticalVignettingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, optical_vignetting_enable),
            },
            FieldInfoData {
                name: "OpticalVignettingAmount",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, optical_vignetting_amount),
            },
            FieldInfoData {
                name: "OpticalVignettingAspectRatio",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, optical_vignetting_aspect_ratio),
            },
            FieldInfoData {
                name: "OpticalVignettingAnamorphicSqueeze",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, optical_vignetting_anamorphic_squeeze),
            },
            FieldInfoData {
                name: "OpticalVignettingSizeCompensation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, optical_vignetting_size_compensation),
            },
            FieldInfoData {
                name: "OpticalVignettingOperation",
                flags: MemberInfoFlags::new(0),
                field_type: VIGNETTINGOPERATION_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, optical_vignetting_operation),
            },
            FieldInfoData {
                name: "RGBBokehTextureEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, r_g_b_bokeh_texture_enable),
            },
            FieldInfoData {
                name: "BokehChromaticAberrationEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, bokeh_chromatic_aberration_enable),
            },
            FieldInfoData {
                name: "BokehChromaticAberrationScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, bokeh_chromatic_aberration_scale),
            },
            FieldInfoData {
                name: "BokehChromaticAberrationRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, bokeh_chromatic_aberration_radius),
            },
            FieldInfoData {
                name: "BokehChromaticAberrationWidth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, bokeh_chromatic_aberration_width),
            },
            FieldInfoData {
                name: "BokehChromaticAberrationRadiusThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, bokeh_chromatic_aberration_radius_threshold),
            },
            FieldInfoData {
                name: "BokehChromaticAberrationRadiusThresholdWidth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, bokeh_chromatic_aberration_radius_threshold_width),
            },
            FieldInfoData {
                name: "BokehChromaticAberrationEnergyThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, bokeh_chromatic_aberration_energy_threshold),
            },
            FieldInfoData {
                name: "BokehChromaticAberrationFgColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, bokeh_chromatic_aberration_fg_color),
            },
            FieldInfoData {
                name: "BokehChromaticAberrationBgColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, bokeh_chromatic_aberration_bg_color),
            },
            FieldInfoData {
                name: "IronsightsDofActive",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, ironsights_dof_active),
            },
            FieldInfoData {
                name: "IronsightsDofExtraBlur",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, ironsights_dof_extra_blur),
            },
            FieldInfoData {
                name: "HipToIronsightsFade",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, hip_to_ironsights_fade),
            },
            FieldInfoData {
                name: "IronsightsDofStartFade",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, ironsights_dof_start_fade),
            },
            FieldInfoData {
                name: "IronsightsFocalDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, ironsights_focal_distance),
            },
            FieldInfoData {
                name: "IronsightsDofCircleBlur",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, ironsights_dof_circle_blur),
            },
            FieldInfoData {
                name: "IronsightsDofCircleDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, ironsights_dof_circle_distance),
            },
            FieldInfoData {
                name: "IronsightsDofCircleFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, ironsights_dof_circle_fade_distance),
            },
            FieldInfoData {
                name: "MaskedBlurEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, masked_blur_enabled),
            },
            FieldInfoData {
                name: "MaskedBlurAmount",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, masked_blur_amount),
            },
            FieldInfoData {
                name: "MaskedBlurTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, masked_blur_texture),
            },
            FieldInfoData {
                name: "CircularDofAntiBandArtifact",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, circular_dof_anti_band_artifact),
            },
            FieldInfoData {
                name: "UseCameraSettings",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, use_camera_settings),
            },
            FieldInfoData {
                name: "SimpleDofMaxBlur",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, simple_dof_max_blur),
            },
            FieldInfoData {
                name: "SimpleDofNearStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, simple_dof_near_start),
            },
            FieldInfoData {
                name: "SimpleDofNearEnd",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, simple_dof_near_end),
            },
            FieldInfoData {
                name: "SimpleDofFarStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, simple_dof_far_start),
            },
            FieldInfoData {
                name: "SimpleDofFarEnd",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, simple_dof_far_end),
            },
            FieldInfoData {
                name: "SpriteDofNearStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, sprite_dof_near_start),
            },
            FieldInfoData {
                name: "SpriteDofNearEnd",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, sprite_dof_near_end),
            },
            FieldInfoData {
                name: "SpriteDofFarStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, sprite_dof_far_start),
            },
            FieldInfoData {
                name: "SpriteDofFarEnd",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, sprite_dof_far_end),
            },
            FieldInfoData {
                name: "SpriteDofMaxBlur",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, sprite_dof_max_blur),
            },
            FieldInfoData {
                name: "Anisotropy",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, anisotropy),
            },
            FieldInfoData {
                name: "FullScreenBlurAddCommonDof",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, full_screen_blur_add_common_dof),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagOverride1",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, field_flag_override1),
            },
            FieldInfoData {
                name: "FieldFlagOverride2",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, field_flag_override2),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, field_flag_changed0),
            },
            FieldInfoData {
                name: "FieldFlagChanged1",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, field_flag_changed1),
            },
            FieldInfoData {
                name: "FieldFlagChanged2",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(DofComponentState, field_flag_changed2),
            },
        ],
    }),
    array_type: Some(DOFCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DofComponentState {
    fn type_info() -> &'static TypeInfo {
        DOFCOMPONENTSTATE_TYPE_INFO
    }
}


pub const DOFCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DofComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("DofComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum VignettingOperation {
    #[default]
    VignettingOperation_Min = 0,
    VignettingOperation_Average = 1,
    VignettingOperation_Max = 2,
}

pub const VIGNETTINGOPERATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VignettingOperation",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(VIGNETTINGOPERATION_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for VignettingOperation {
    fn type_info() -> &'static TypeInfo {
        VIGNETTINGOPERATION_TYPE_INFO
    }
}


pub const VIGNETTINGOPERATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VignettingOperation-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("VignettingOperation-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum DofSource {
    #[default]
    DofSource_Linear = 0,
    DofSource_PhysicallyBased = 1,
    DofSource_Camera = 2,
}

pub const DOFSOURCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DofSource",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(DOFSOURCE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DofSource {
    fn type_info() -> &'static TypeInfo {
        DOFSOURCE_TYPE_INFO
    }
}


pub const DOFSOURCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DofSource-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("DofSource-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ColorCorrectionComponentState {
    pub enable: bool,
    pub brightness: super::core::Vec3,
    pub contrast: super::core::Vec3,
    pub saturation: super::core::Vec3,
    pub hue: f32,
    pub color_grading_enable: bool,
    pub color_grading_texture: TextureBaseAsset,
    pub color_grading_max_hdr_value: f32,
    pub hdr_color_grading_lut: TextureBaseAsset,
    pub color_grading_stack: Vec<ColorGrading>,
    pub field_flag_override0: u16,
    pub field_flag_changed0: u16,
}

pub const COLORCORRECTIONCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ColorCorrectionComponentState",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ColorCorrectionComponentState, enable),
            },
            FieldInfoData {
                name: "Brightness",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ColorCorrectionComponentState, brightness),
            },
            FieldInfoData {
                name: "Contrast",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ColorCorrectionComponentState, contrast),
            },
            FieldInfoData {
                name: "Saturation",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ColorCorrectionComponentState, saturation),
            },
            FieldInfoData {
                name: "Hue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ColorCorrectionComponentState, hue),
            },
            FieldInfoData {
                name: "ColorGradingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ColorCorrectionComponentState, color_grading_enable),
            },
            FieldInfoData {
                name: "ColorGradingTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(ColorCorrectionComponentState, color_grading_texture),
            },
            FieldInfoData {
                name: "ColorGradingMaxHdrValue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ColorCorrectionComponentState, color_grading_max_hdr_value),
            },
            FieldInfoData {
                name: "HdrColorGradingLut",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(ColorCorrectionComponentState, hdr_color_grading_lut),
            },
            FieldInfoData {
                name: "ColorGradingStack",
                flags: MemberInfoFlags::new(144),
                field_type: COLORGRADING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ColorCorrectionComponentState, color_grading_stack),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(ColorCorrectionComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(ColorCorrectionComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(COLORCORRECTIONCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ColorCorrectionComponentState {
    fn type_info() -> &'static TypeInfo {
        COLORCORRECTIONCOMPONENTSTATE_TYPE_INFO
    }
}


pub const COLORCORRECTIONCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ColorCorrectionComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ColorCorrectionComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ColorGrading {
    pub visibility: f32,
    pub texture: TextureBaseAsset,
}

pub const COLORGRADING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ColorGrading",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Visibility",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ColorGrading, visibility),
            },
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(ColorGrading, texture),
            },
        ],
    }),
    array_type: Some(COLORGRADING_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ColorGrading {
    fn type_info() -> &'static TypeInfo {
        COLORGRADING_TYPE_INFO
    }
}


pub const COLORGRADING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ColorGrading-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ColorGrading-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TonemapComponentState {
    pub e_v: f32,
    pub exposure_compensation: f32,
    pub auto_exposure_darkest_exclude: f32,
    pub auto_exposure_brightest_exclude: f32,
    pub dark_adaptation_time: f32,
    pub light_adaptation_time: f32,
    pub automatic_exposure: bool,
    pub auto_exposure_method: AutoExposureMethod,
    pub auto_exposure_higher_threshold: f32,
    pub auto_exposure_lower_threshold: f32,
    pub clamp_e_v: bool,
    pub min_e_v: f32,
    pub max_e_v: f32,
    pub spot_meter_scale: f32,
    pub spot_meter_offset_x: f32,
    pub spot_meter_offset_y: f32,
    pub tonemap_method: TonemapMethod,
    pub bloom_direction: BloomDirection,
    pub directional_bloom_clamp: f32,
    pub bloom_scale: super::core::Vec3,
    pub bloom_soft_clip: f32,
    pub bloom_method: BloomMethod,
    pub gaussian_sharpness: f32,
    pub gaussian1_color: super::core::Vec3,
    pub gaussian1_weight: f32,
    pub gaussian2_color: super::core::Vec3,
    pub gaussian2_weight: f32,
    pub gaussian3_color: super::core::Vec3,
    pub gaussian3_weight: f32,
    pub gaussian4_color: super::core::Vec3,
    pub gaussian4_weight: f32,
    pub gaussian5_color: super::core::Vec3,
    pub gaussian5_weight: f32,
    pub f_f_t_threshold: f32,
    pub f_f_t_cutoff: f32,
    pub f_f_t_kernel_scale: f32,
    pub f_f_t_kernel_rotation: f32,
    pub f_f_t_spike_scale_limit_enable: bool,
    pub f_f_t_spike_scale_limit: f32,
    pub f_f_t_kernel_texture: TextureBaseAsset,
    pub chromostereopsis_enable: bool,
    pub chromostereopsis_scale: f32,
    pub chromostereopsis_offset: f32,
    pub lens_dirt_texture: TextureBaseAsset,
    pub lens_dirt_bias: super::core::Vec3,
    pub lens_dirt_factor: super::core::Vec3,
    pub lens_dirt_exponent: super::core::Vec3,
    pub field_flag_override0: u32,
    pub field_flag_override1: u16,
    pub field_flag_changed0: u32,
    pub field_flag_changed1: u32,
}

pub const TONEMAPCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TonemapComponentState",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "EV",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, e_v),
            },
            FieldInfoData {
                name: "ExposureCompensation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, exposure_compensation),
            },
            FieldInfoData {
                name: "AutoExposureDarkestExclude",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, auto_exposure_darkest_exclude),
            },
            FieldInfoData {
                name: "AutoExposureBrightestExclude",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, auto_exposure_brightest_exclude),
            },
            FieldInfoData {
                name: "DarkAdaptationTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, dark_adaptation_time),
            },
            FieldInfoData {
                name: "LightAdaptationTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, light_adaptation_time),
            },
            FieldInfoData {
                name: "AutomaticExposure",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, automatic_exposure),
            },
            FieldInfoData {
                name: "AutoExposureMethod",
                flags: MemberInfoFlags::new(0),
                field_type: AUTOEXPOSUREMETHOD_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, auto_exposure_method),
            },
            FieldInfoData {
                name: "AutoExposureHigherThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, auto_exposure_higher_threshold),
            },
            FieldInfoData {
                name: "AutoExposureLowerThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, auto_exposure_lower_threshold),
            },
            FieldInfoData {
                name: "ClampEV",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, clamp_e_v),
            },
            FieldInfoData {
                name: "MinEV",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, min_e_v),
            },
            FieldInfoData {
                name: "MaxEV",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, max_e_v),
            },
            FieldInfoData {
                name: "SpotMeterScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, spot_meter_scale),
            },
            FieldInfoData {
                name: "SpotMeterOffsetX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, spot_meter_offset_x),
            },
            FieldInfoData {
                name: "SpotMeterOffsetY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, spot_meter_offset_y),
            },
            FieldInfoData {
                name: "TonemapMethod",
                flags: MemberInfoFlags::new(0),
                field_type: TONEMAPMETHOD_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, tonemap_method),
            },
            FieldInfoData {
                name: "BloomDirection",
                flags: MemberInfoFlags::new(0),
                field_type: BLOOMDIRECTION_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, bloom_direction),
            },
            FieldInfoData {
                name: "DirectionalBloomClamp",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, directional_bloom_clamp),
            },
            FieldInfoData {
                name: "BloomScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, bloom_scale),
            },
            FieldInfoData {
                name: "BloomSoftClip",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, bloom_soft_clip),
            },
            FieldInfoData {
                name: "BloomMethod",
                flags: MemberInfoFlags::new(0),
                field_type: BLOOMMETHOD_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, bloom_method),
            },
            FieldInfoData {
                name: "GaussianSharpness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, gaussian_sharpness),
            },
            FieldInfoData {
                name: "Gaussian1Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, gaussian1_color),
            },
            FieldInfoData {
                name: "Gaussian1Weight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, gaussian1_weight),
            },
            FieldInfoData {
                name: "Gaussian2Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, gaussian2_color),
            },
            FieldInfoData {
                name: "Gaussian2Weight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, gaussian2_weight),
            },
            FieldInfoData {
                name: "Gaussian3Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, gaussian3_color),
            },
            FieldInfoData {
                name: "Gaussian3Weight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, gaussian3_weight),
            },
            FieldInfoData {
                name: "Gaussian4Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, gaussian4_color),
            },
            FieldInfoData {
                name: "Gaussian4Weight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, gaussian4_weight),
            },
            FieldInfoData {
                name: "Gaussian5Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, gaussian5_color),
            },
            FieldInfoData {
                name: "Gaussian5Weight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, gaussian5_weight),
            },
            FieldInfoData {
                name: "FFTThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, f_f_t_threshold),
            },
            FieldInfoData {
                name: "FFTCutoff",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, f_f_t_cutoff),
            },
            FieldInfoData {
                name: "FFTKernelScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, f_f_t_kernel_scale),
            },
            FieldInfoData {
                name: "FFTKernelRotation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, f_f_t_kernel_rotation),
            },
            FieldInfoData {
                name: "FFTSpikeScaleLimitEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, f_f_t_spike_scale_limit_enable),
            },
            FieldInfoData {
                name: "FFTSpikeScaleLimit",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, f_f_t_spike_scale_limit),
            },
            FieldInfoData {
                name: "FFTKernelTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, f_f_t_kernel_texture),
            },
            FieldInfoData {
                name: "ChromostereopsisEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, chromostereopsis_enable),
            },
            FieldInfoData {
                name: "ChromostereopsisScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, chromostereopsis_scale),
            },
            FieldInfoData {
                name: "ChromostereopsisOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, chromostereopsis_offset),
            },
            FieldInfoData {
                name: "LensDirtTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, lens_dirt_texture),
            },
            FieldInfoData {
                name: "LensDirtBias",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, lens_dirt_bias),
            },
            FieldInfoData {
                name: "LensDirtFactor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, lens_dirt_factor),
            },
            FieldInfoData {
                name: "LensDirtExponent",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, lens_dirt_exponent),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagOverride1",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, field_flag_override1),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, field_flag_changed0),
            },
            FieldInfoData {
                name: "FieldFlagChanged1",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentState, field_flag_changed1),
            },
        ],
    }),
    array_type: Some(TONEMAPCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TonemapComponentState {
    fn type_info() -> &'static TypeInfo {
        TONEMAPCOMPONENTSTATE_TYPE_INFO
    }
}


pub const TONEMAPCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TonemapComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("TonemapComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum TonemapMethod {
    #[default]
    TonemapMethod_None = 4,
    TonemapMethod_Linear = 0,
    TonemapMethod_Filmic = 1,
    TonemapMethod_FilmicNeutral = 2,
    TonemapMethod_LinearApproxGamma = 3,
}

pub const TONEMAPMETHOD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TonemapMethod",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(TONEMAPMETHOD_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TonemapMethod {
    fn type_info() -> &'static TypeInfo {
        TONEMAPMETHOD_TYPE_INFO
    }
}


pub const TONEMAPMETHOD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TonemapMethod-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("TonemapMethod-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SubSurfaceProfile {
    pub preset: SubSurfaceProfilePreset,
    pub scattering_enable: bool,
    pub scattering_scale: f32,
    pub scattering_spike_amount: f32,
    pub radius_r: f32,
    pub radius_g: f32,
    pub radius_b: f32,
    pub translucency_enable: bool,
    pub automatic_thickness_enable: bool,
    pub translucency_scale: f32,
    pub translucency_base_color_amount: f32,
    pub translucency_multiplier: f32,
    pub translucency_shadow_bias: f32,
}

pub const SUBSURFACEPROFILE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubSurfaceProfile",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Preset",
                flags: MemberInfoFlags::new(0),
                field_type: SUBSURFACEPROFILEPRESET_TYPE_INFO,
                rust_offset: offset_of!(SubSurfaceProfile, preset),
            },
            FieldInfoData {
                name: "ScatteringEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SubSurfaceProfile, scattering_enable),
            },
            FieldInfoData {
                name: "ScatteringScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SubSurfaceProfile, scattering_scale),
            },
            FieldInfoData {
                name: "ScatteringSpikeAmount",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SubSurfaceProfile, scattering_spike_amount),
            },
            FieldInfoData {
                name: "RadiusR",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SubSurfaceProfile, radius_r),
            },
            FieldInfoData {
                name: "RadiusG",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SubSurfaceProfile, radius_g),
            },
            FieldInfoData {
                name: "RadiusB",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SubSurfaceProfile, radius_b),
            },
            FieldInfoData {
                name: "TranslucencyEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SubSurfaceProfile, translucency_enable),
            },
            FieldInfoData {
                name: "AutomaticThicknessEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SubSurfaceProfile, automatic_thickness_enable),
            },
            FieldInfoData {
                name: "TranslucencyScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SubSurfaceProfile, translucency_scale),
            },
            FieldInfoData {
                name: "TranslucencyBaseColorAmount",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SubSurfaceProfile, translucency_base_color_amount),
            },
            FieldInfoData {
                name: "TranslucencyMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SubSurfaceProfile, translucency_multiplier),
            },
            FieldInfoData {
                name: "TranslucencyShadowBias",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SubSurfaceProfile, translucency_shadow_bias),
            },
        ],
    }),
    array_type: Some(SUBSURFACEPROFILE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for SubSurfaceProfile {
    fn type_info() -> &'static TypeInfo {
        SUBSURFACEPROFILE_TYPE_INFO
    }
}


pub const SUBSURFACEPROFILE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubSurfaceProfile-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("SubSurfaceProfile-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum SubSurfaceProfilePreset {
    #[default]
    SubSurfaceProfilePreset_Custom = 0,
    SubSurfaceProfilePreset_CaucasianSkin = 1,
}

pub const SUBSURFACEPROFILEPRESET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubSurfaceProfilePreset",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(SUBSURFACEPROFILEPRESET_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SubSurfaceProfilePreset {
    fn type_info() -> &'static TypeInfo {
        SUBSURFACEPROFILEPRESET_TYPE_INFO
    }
}


pub const SUBSURFACEPROFILEPRESET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubSurfaceProfilePreset-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("SubSurfaceProfilePreset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ColorGradingQualityMode {
    #[default]
    ColorGradingQualityMode_Low = 0,
    ColorGradingQualityMode_High = 1,
}

pub const COLORGRADINGQUALITYMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ColorGradingQualityMode",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(COLORGRADINGQUALITYMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ColorGradingQualityMode {
    fn type_info() -> &'static TypeInfo {
        COLORGRADINGQUALITYMODE_TYPE_INFO
    }
}


pub const COLORGRADINGQUALITYMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ColorGradingQualityMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ColorGradingQualityMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum BloomDirection {
    #[default]
    BloomDirection_None = 0,
    BloomDirection_Horizontal = 1,
    BloomDirection_Vertical = 2,
}

pub const BLOOMDIRECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BloomDirection",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(BLOOMDIRECTION_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for BloomDirection {
    fn type_info() -> &'static TypeInfo {
        BLOOMDIRECTION_TYPE_INFO
    }
}


pub const BLOOMDIRECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BloomDirection-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("BloomDirection-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum BloomMethod {
    #[default]
    BloomMethod_GaussianSimple = 0,
    BloomMethod_GaussianCustom = 1,
    BloomMethod_FFT = 2,
}

pub const BLOOMMETHOD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BloomMethod",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(BLOOMMETHOD_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for BloomMethod {
    fn type_info() -> &'static TypeInfo {
        BLOOMMETHOD_TYPE_INFO
    }
}


pub const BLOOMMETHOD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BloomMethod-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("BloomMethod-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum BlurFilter {
    #[default]
    BfNone = 0,
    BfGaussian3Pixels = 1,
    BfGaussian5Pixels = 2,
    BfGaussian7Pixels = 3,
    BfGaussian9Pixels = 4,
    BfGaussian15Pixels = 5,
    BfGaussian31Pixels = 6,
}

pub const BLURFILTER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlurFilter",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(BLURFILTER_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for BlurFilter {
    fn type_info() -> &'static TypeInfo {
        BLURFILTER_TYPE_INFO
    }
}


pub const BLURFILTER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlurFilter-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("BlurFilter-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum LocalPlayerViewId {
    #[default]
    LocalPlayerViewId_RootView = 0,
    LocalPlayerViewId_Secondary = 1,
    LocalPlayerViewId_Custom1 = 2,
    LocalPlayerViewId_Custom2 = 3,
    LocalPlayerViewId_Custom3 = 4,
    LocalPlayerViewId_Custom4 = 5,
    LocalPlayerViewId_Count = 6,
}

pub const LOCALPLAYERVIEWID_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalPlayerViewId",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(LOCALPLAYERVIEWID_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LocalPlayerViewId {
    fn type_info() -> &'static TypeInfo {
        LOCALPLAYERVIEWID_TYPE_INFO
    }
}


pub const LOCALPLAYERVIEWID_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalPlayerViewId-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("LocalPlayerViewId-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum DistantIBLLocationType {
    #[default]
    DistantIBLLocationType_Outdoor = 0,
    DistantIBLLocationType_Indoor = 1,
    DistantIBLLocationTypeCount = 2,
}

pub const DISTANTIBLLOCATIONTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistantIBLLocationType",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(DISTANTIBLLOCATIONTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DistantIBLLocationType {
    fn type_info() -> &'static TypeInfo {
        DISTANTIBLLOCATIONTYPE_TYPE_INFO
    }
}


pub const DISTANTIBLLOCATIONTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistantIBLLocationType-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("DistantIBLLocationType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum LightTilePassType {
    #[default]
    LightTilePassPunctual = 0,
    LightTilePassPunctualShadow = 1,
    LightTilePassArea = 2,
    LightTilePassAreaShadow = 3,
    LightTilePassLocalIBL = 4,
    LightTilePassLocalPR = 5,
    LightTilePassCount = 6,
}

pub const LIGHTTILEPASSTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightTilePassType",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(LIGHTTILEPASSTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LightTilePassType {
    fn type_info() -> &'static TypeInfo {
        LIGHTTILEPASSTYPE_TYPE_INFO
    }
}


pub const LIGHTTILEPASSTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightTilePassType-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("LightTilePassType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum LocalIBLMode {
    #[default]
    LocalIBLMode_Static = 0,
    LocalIBLMode_Dynamic = 1,
    LocalIBLMode_Baked = 2,
}

pub const LOCALIBLMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalIBLMode",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(LOCALIBLMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LocalIBLMode {
    fn type_info() -> &'static TypeInfo {
        LOCALIBLMODE_TYPE_INFO
    }
}


pub const LOCALIBLMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalIBLMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("LocalIBLMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum PunctualLightType {
    #[default]
    PunctualLightType_Point = 0,
    PunctualLightType_Line = 1,
    PunctualLightType_Cone = 2,
    PunctualLightType_Spot = 3,
    PunctualLightTypeCount = 4,
}

pub const PUNCTUALLIGHTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PunctualLightType",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(PUNCTUALLIGHTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PunctualLightType {
    fn type_info() -> &'static TypeInfo {
        PUNCTUALLIGHTTYPE_TYPE_INFO
    }
}


pub const PUNCTUALLIGHTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PunctualLightType-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("PunctualLightType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum RectangularLightShape {
    #[default]
    RectangularLightShape_Rectangular = 0,
    RectangularLightShape_Frustum = 1,
    RectangularLightShape_OrthoFrustum = 2,
}

pub const RECTANGULARLIGHTSHAPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RectangularLightShape",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(RECTANGULARLIGHTSHAPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RectangularLightShape {
    fn type_info() -> &'static TypeInfo {
        RECTANGULARLIGHTSHAPE_TYPE_INFO
    }
}


pub const RECTANGULARLIGHTSHAPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RectangularLightShape-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("RectangularLightShape-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum LightUnitType {
    #[default]
    LightUnitType_LuminousPower = 0,
    LightUnitType_Luminance = 1,
}

pub const LIGHTUNITTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightUnitType",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(LIGHTUNITTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LightUnitType {
    fn type_info() -> &'static TypeInfo {
        LIGHTUNITTYPE_TYPE_INFO
    }
}


pub const LIGHTUNITTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightUnitType-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("LightUnitType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PipelineExternalShaderConditional {
    pub condition_name: String,
    pub default_index: i32,
    pub branches: Vec<PipelineExternalShaderConditionalBranch>,
    pub is_global: bool,
}

pub const PIPELINEEXTERNALSHADERCONDITIONAL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PipelineExternalShaderConditional",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ConditionName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(PipelineExternalShaderConditional, condition_name),
            },
            FieldInfoData {
                name: "DefaultIndex",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PipelineExternalShaderConditional, default_index),
            },
            FieldInfoData {
                name: "Branches",
                flags: MemberInfoFlags::new(144),
                field_type: PIPELINEEXTERNALSHADERCONDITIONALBRANCH_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(PipelineExternalShaderConditional, branches),
            },
            FieldInfoData {
                name: "IsGlobal",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PipelineExternalShaderConditional, is_global),
            },
        ],
    }),
    array_type: Some(PIPELINEEXTERNALSHADERCONDITIONAL_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PipelineExternalShaderConditional {
    fn type_info() -> &'static TypeInfo {
        PIPELINEEXTERNALSHADERCONDITIONAL_TYPE_INFO
    }
}


pub const PIPELINEEXTERNALSHADERCONDITIONAL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PipelineExternalShaderConditional-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("PipelineExternalShaderConditional-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PipelineExternalShaderConditionalBranch {
    pub branch_name: String,
    pub condition_name: String,
    pub index: i32,
    pub is_global: bool,
}

pub const PIPELINEEXTERNALSHADERCONDITIONALBRANCH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PipelineExternalShaderConditionalBranch",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "BranchName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(PipelineExternalShaderConditionalBranch, branch_name),
            },
            FieldInfoData {
                name: "ConditionName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(PipelineExternalShaderConditionalBranch, condition_name),
            },
            FieldInfoData {
                name: "Index",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PipelineExternalShaderConditionalBranch, index),
            },
            FieldInfoData {
                name: "IsGlobal",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PipelineExternalShaderConditionalBranch, is_global),
            },
        ],
    }),
    array_type: Some(PIPELINEEXTERNALSHADERCONDITIONALBRANCH_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PipelineExternalShaderConditionalBranch {
    fn type_info() -> &'static TypeInfo {
        PIPELINEEXTERNALSHADERCONDITIONALBRANCH_TYPE_INFO
    }
}


pub const PIPELINEEXTERNALSHADERCONDITIONALBRANCH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PipelineExternalShaderConditionalBranch-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("PipelineExternalShaderConditionalBranch-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ExternalShaderConditionalFilterAsset {
    pub inludees: Vec<ExternalShaderConditionalSelection>,
}

pub const EXTERNALSHADERCONDITIONALFILTERASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExternalShaderConditionalFilterAsset",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Inludees",
                flags: MemberInfoFlags::new(144),
                field_type: EXTERNALSHADERCONDITIONALSELECTION_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ExternalShaderConditionalFilterAsset, inludees),
            },
        ],
    }),
    array_type: Some(EXTERNALSHADERCONDITIONALFILTERASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ExternalShaderConditionalFilterAsset {
    fn type_info() -> &'static TypeInfo {
        EXTERNALSHADERCONDITIONALFILTERASSET_TYPE_INFO
    }
}


pub const EXTERNALSHADERCONDITIONALFILTERASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExternalShaderConditionalFilterAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ExternalShaderConditionalFilterAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ExternalShaderConditionalSelection {
    pub condition_name: String,
    pub branch_name: String,
    pub external_shader_conditional: ExternalShaderConditionalAsset,
}

pub const EXTERNALSHADERCONDITIONALSELECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExternalShaderConditionalSelection",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ConditionName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ExternalShaderConditionalSelection, condition_name),
            },
            FieldInfoData {
                name: "BranchName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ExternalShaderConditionalSelection, branch_name),
            },
            FieldInfoData {
                name: "ExternalShaderConditional",
                flags: MemberInfoFlags::new(0),
                field_type: EXTERNALSHADERCONDITIONALASSET_TYPE_INFO,
                rust_offset: offset_of!(ExternalShaderConditionalSelection, external_shader_conditional),
            },
        ],
    }),
    array_type: Some(EXTERNALSHADERCONDITIONALSELECTION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ExternalShaderConditionalSelection {
    fn type_info() -> &'static TypeInfo {
        EXTERNALSHADERCONDITIONALSELECTION_TYPE_INFO
    }
}


pub const EXTERNALSHADERCONDITIONALSELECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExternalShaderConditionalSelection-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ExternalShaderConditionalSelection-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ExternalShaderBooleanDescription {
}

pub const EXTERNALSHADERBOOLEANDESCRIPTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExternalShaderBooleanDescription",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EXTERNALSHADERCONDITIONALASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(EXTERNALSHADERBOOLEANDESCRIPTION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ExternalShaderBooleanDescription {
    fn type_info() -> &'static TypeInfo {
        EXTERNALSHADERBOOLEANDESCRIPTION_TYPE_INFO
    }
}


pub const EXTERNALSHADERBOOLEANDESCRIPTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExternalShaderBooleanDescription-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ExternalShaderBooleanDescription-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ExternalShaderSwitchDescription {
}

pub const EXTERNALSHADERSWITCHDESCRIPTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExternalShaderSwitchDescription",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EXTERNALSHADERCONDITIONALASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(EXTERNALSHADERSWITCHDESCRIPTION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ExternalShaderSwitchDescription {
    fn type_info() -> &'static TypeInfo {
        EXTERNALSHADERSWITCHDESCRIPTION_TYPE_INFO
    }
}


pub const EXTERNALSHADERSWITCHDESCRIPTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExternalShaderSwitchDescription-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ExternalShaderSwitchDescription-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ExternalShaderConditionalAsset {
    pub condition_name: String,
    pub description: String,
    pub branches: Vec<String>,
    pub default_value: String,
    pub is_global: bool,
}

pub const EXTERNALSHADERCONDITIONALASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExternalShaderConditionalAsset",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ConditionName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ExternalShaderConditionalAsset, condition_name),
            },
            FieldInfoData {
                name: "Description",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ExternalShaderConditionalAsset, description),
            },
            FieldInfoData {
                name: "Branches",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ExternalShaderConditionalAsset, branches),
            },
            FieldInfoData {
                name: "DefaultValue",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ExternalShaderConditionalAsset, default_value),
            },
            FieldInfoData {
                name: "IsGlobal",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ExternalShaderConditionalAsset, is_global),
            },
        ],
    }),
    array_type: Some(EXTERNALSHADERCONDITIONALASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ExternalShaderConditionalAsset {
    fn type_info() -> &'static TypeInfo {
        EXTERNALSHADERCONDITIONALASSET_TYPE_INFO
    }
}


pub const EXTERNALSHADERCONDITIONALASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExternalShaderConditionalAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ExternalShaderConditionalAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ShadowCacheDepthBias {
    #[default]
    ShadowCacheDepthBias_Lowest = 0,
    ShadowCacheDepthBias_Lower = 1,
    ShadowCacheDepthBias_Medium = 2,
    ShadowCacheDepthBias_Higher = 3,
    ShadowCacheDepthBias_Highest = 4,
}

pub const SHADOWCACHEDEPTHBIAS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowCacheDepthBias",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(SHADOWCACHEDEPTHBIAS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShadowCacheDepthBias {
    fn type_info() -> &'static TypeInfo {
        SHADOWCACHEDEPTHBIAS_TYPE_INFO
    }
}


pub const SHADOWCACHEDEPTHBIAS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowCacheDepthBias-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ShadowCacheDepthBias-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ShadowCacheMode {
    #[default]
    ShadowCacheMode_Static = 0,
    ShadowCacheMode_Dynamic = 1,
    ShadowCacheMode_Baked = 2,
    ShadowCacheMode_DynamicProd = 3,
    ShadowCacheMode_BakeEventTriggered = 4,
}

pub const SHADOWCACHEMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowCacheMode",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(SHADOWCACHEMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShadowCacheMode {
    fn type_info() -> &'static TypeInfo {
        SHADOWCACHEMODE_TYPE_INFO
    }
}


pub const SHADOWCACHEMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowCacheMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ShadowCacheMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DecalAtlasTile {
    pub tile_index_x: f32,
    pub tile_index_y: f32,
    pub tile_count_x: f32,
    pub tile_count_y: f32,
    pub flip_x: bool,
    pub flip_y: bool,
}

pub const DECALATLASTILE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalAtlasTile",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "TileIndexX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DecalAtlasTile, tile_index_x),
            },
            FieldInfoData {
                name: "TileIndexY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DecalAtlasTile, tile_index_y),
            },
            FieldInfoData {
                name: "TileCountX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DecalAtlasTile, tile_count_x),
            },
            FieldInfoData {
                name: "TileCountY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DecalAtlasTile, tile_count_y),
            },
            FieldInfoData {
                name: "FlipX",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DecalAtlasTile, flip_x),
            },
            FieldInfoData {
                name: "FlipY",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DecalAtlasTile, flip_y),
            },
        ],
    }),
    array_type: Some(DECALATLASTILE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for DecalAtlasTile {
    fn type_info() -> &'static TypeInfo {
        DECALATLASTILE_TYPE_INFO
    }
}


pub const DECALATLASTILE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalAtlasTile-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("DecalAtlasTile-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DecalTemplateBaseAsset {
}

pub const DECALTEMPLATEBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalTemplateBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DECALTEMPLATEBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DecalTemplateBaseAsset {
    fn type_info() -> &'static TypeInfo {
        DECALTEMPLATEBASEASSET_TYPE_INFO
    }
}


pub const DECALTEMPLATEBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalTemplateBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("DecalTemplateBaseAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum DecalType {
    #[default]
    DecalType_VolumeDecal = 0,
    DecalType_ProjectedMeshDecal = 1,
    DecalType_QuadDecal = 2,
    DecalType_Count = 3,
}

pub const DECALTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalType",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(DECALTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DecalType {
    fn type_info() -> &'static TypeInfo {
        DECALTYPE_TYPE_INFO
    }
}


pub const DECALTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalType-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("DecalType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CullIdDynamicState {
}

pub const CULLIDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CullIdDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(CULLIDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CullIdDynamicState {
    fn type_info() -> &'static TypeInfo {
        CULLIDDYNAMICSTATE_TYPE_INFO
    }
}


pub const CULLIDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CullIdDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("CullIdDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CullIdStaticState {
    pub affect_child_views: bool,
    pub field_flag_changed0: u8,
}

pub const CULLIDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CullIdStaticState",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "AffectChildViews",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CullIdStaticState, affect_child_views),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(CullIdStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(CULLIDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CullIdStaticState {
    fn type_info() -> &'static TypeInfo {
        CULLIDSTATICSTATE_TYPE_INFO
    }
}


pub const CULLIDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CullIdStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("CullIdStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OutlineColorsComponentData {
}

pub const OUTLINECOLORSCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OutlineColorsComponentData",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(OUTLINECOLORSCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OutlineColorsComponentData {
    fn type_info() -> &'static TypeInfo {
        OUTLINECOLORSCOMPONENTDATA_TYPE_INFO
    }
}


pub const OUTLINECOLORSCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OutlineColorsComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("OutlineColorsComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OccluderRealPlaneEntityData {
}

pub const OCCLUDERREALPLANEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderRealPlaneEntityData",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(OCCLUDERREALPLANEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OccluderRealPlaneEntityData {
    fn type_info() -> &'static TypeInfo {
        OCCLUDERREALPLANEENTITYDATA_TYPE_INFO
    }
}


pub const OCCLUDERREALPLANEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderRealPlaneEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("OccluderRealPlaneEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FogVolumeEntityData {
}

pub const FOGVOLUMEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FogVolumeEntityData",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FOGVOLUMEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FogVolumeEntityData {
    fn type_info() -> &'static TypeInfo {
        FOGVOLUMEENTITYDATA_TYPE_INFO
    }
}


pub const FOGVOLUMEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FogVolumeEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("FogVolumeEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LayerTextureConfigAsset {
}

pub const LAYERTEXTURECONFIGASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LayerTextureConfigAsset",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LAYERTEXTURECONFIGASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LayerTextureConfigAsset {
    fn type_info() -> &'static TypeInfo {
        LAYERTEXTURECONFIGASSET_TYPE_INFO
    }
}


pub const LAYERTEXTURECONFIGASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LayerTextureConfigAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("LayerTextureConfigAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LayerTextureConfig {
}

pub const LAYERTEXTURECONFIG_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LayerTextureConfig",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LAYERTEXTURECONFIG_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LayerTextureConfig {
    fn type_info() -> &'static TypeInfo {
        LAYERTEXTURECONFIG_TYPE_INFO
    }
}


pub const LAYERTEXTURECONFIG_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LayerTextureConfig-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("LayerTextureConfig-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FScript_VerletChainEntityData {
}

pub const FSCRIPT_VERLETCHAINENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FScript_VerletChainEntityData",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FSCRIPT_VERLETCHAINENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FScript_VerletChainEntityData {
    fn type_info() -> &'static TypeInfo {
        FSCRIPT_VERLETCHAINENTITYDATA_TYPE_INFO
    }
}


pub const FSCRIPT_VERLETCHAINENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FScript_VerletChainEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("FScript_VerletChainEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FScript_ProceduralBoneGlobalsEntityData {
}

pub const FSCRIPT_PROCEDURALBONEGLOBALSENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FScript_ProceduralBoneGlobalsEntityData",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FSCRIPT_PROCEDURALBONEGLOBALSENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FScript_ProceduralBoneGlobalsEntityData {
    fn type_info() -> &'static TypeInfo {
        FSCRIPT_PROCEDURALBONEGLOBALSENTITYDATA_TYPE_INFO
    }
}


pub const FSCRIPT_PROCEDURALBONEGLOBALSENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FScript_ProceduralBoneGlobalsEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("FScript_ProceduralBoneGlobalsEntityData-Array"),
    array_type: None,
    alignment: 8,
};


