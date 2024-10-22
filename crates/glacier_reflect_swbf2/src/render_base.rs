use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct VertexShaderFragmentDynamicState {
}

pub trait VertexShaderFragmentDynamicStateTrait: TypeObject {
}

impl VertexShaderFragmentDynamicStateTrait for VertexShaderFragmentDynamicState {
}

pub static VERTEXSHADERFRAGMENTDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VertexShaderFragmentDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VertexShaderFragmentDynamicState as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(VERTEXSHADERFRAGMENTDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for VertexShaderFragmentDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        VERTEXSHADERFRAGMENTDYNAMICSTATE_TYPE_INFO
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


pub static VERTEXSHADERFRAGMENTDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VertexShaderFragmentDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("VertexShaderFragmentDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VertexShaderFragmentStaticState {
    pub fragment_path: String,
    pub field_flag_changed0: u8,
}

pub trait VertexShaderFragmentStaticStateTrait: TypeObject {
    fn fragment_path(&self) -> &String;
    fn fragment_path_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl VertexShaderFragmentStaticStateTrait for VertexShaderFragmentStaticState {
    fn fragment_path(&self) -> &String {
        &self.fragment_path
    }
    fn fragment_path_mut(&mut self) -> &mut String {
        &mut self.fragment_path
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static VERTEXSHADERFRAGMENTSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VertexShaderFragmentStaticState",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VertexShaderFragmentStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "FragmentPath",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(VertexShaderFragmentStaticState, fragment_path),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(VertexShaderFragmentStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(VERTEXSHADERFRAGMENTSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VertexShaderFragmentStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        VERTEXSHADERFRAGMENTSTATICSTATE_TYPE_INFO
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


pub static VERTEXSHADERFRAGMENTSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VertexShaderFragmentStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("VertexShaderFragmentStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TextureUnloadOnDemandDynamicState {
    pub texture_state: TextureUnloadOnDemandLifeState,
    pub tick_update: u32,
    pub field_flag_changed0: u8,
}

pub trait TextureUnloadOnDemandDynamicStateTrait: TypeObject {
    fn texture_state(&self) -> &TextureUnloadOnDemandLifeState;
    fn texture_state_mut(&mut self) -> &mut TextureUnloadOnDemandLifeState;
    fn tick_update(&self) -> &u32;
    fn tick_update_mut(&mut self) -> &mut u32;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TextureUnloadOnDemandDynamicStateTrait for TextureUnloadOnDemandDynamicState {
    fn texture_state(&self) -> &TextureUnloadOnDemandLifeState {
        &self.texture_state
    }
    fn texture_state_mut(&mut self) -> &mut TextureUnloadOnDemandLifeState {
        &mut self.texture_state
    }
    fn tick_update(&self) -> &u32 {
        &self.tick_update
    }
    fn tick_update_mut(&mut self) -> &mut u32 {
        &mut self.tick_update
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static TEXTUREUNLOADONDEMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureUnloadOnDemandDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TextureUnloadOnDemandDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TextureState",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureUnloadOnDemandLifeState",
                rust_offset: offset_of!(TextureUnloadOnDemandDynamicState, texture_state),
            },
            FieldInfoData {
                name: "TickUpdate",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TextureUnloadOnDemandDynamicState, tick_update),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TextureUnloadOnDemandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TEXTUREUNLOADONDEMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for TextureUnloadOnDemandDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        TEXTUREUNLOADONDEMANDDYNAMICSTATE_TYPE_INFO
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


pub static TEXTUREUNLOADONDEMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureUnloadOnDemandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("TextureUnloadOnDemandDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TextureUnloadOnDemandStaticState {
    pub identifier: u32,
    pub texture_handle: u16,
    pub field_flag_changed0: u8,
}

pub trait TextureUnloadOnDemandStaticStateTrait: TypeObject {
    fn identifier(&self) -> &u32;
    fn identifier_mut(&mut self) -> &mut u32;
    fn texture_handle(&self) -> &u16;
    fn texture_handle_mut(&mut self) -> &mut u16;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TextureUnloadOnDemandStaticStateTrait for TextureUnloadOnDemandStaticState {
    fn identifier(&self) -> &u32 {
        &self.identifier
    }
    fn identifier_mut(&mut self) -> &mut u32 {
        &mut self.identifier
    }
    fn texture_handle(&self) -> &u16 {
        &self.texture_handle
    }
    fn texture_handle_mut(&mut self) -> &mut u16 {
        &mut self.texture_handle
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static TEXTUREUNLOADONDEMANDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureUnloadOnDemandStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TextureUnloadOnDemandStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Identifier",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TextureUnloadOnDemandStaticState, identifier),
            },
            FieldInfoData {
                name: "TextureHandle",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(TextureUnloadOnDemandStaticState, texture_handle),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TextureUnloadOnDemandStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TEXTUREUNLOADONDEMANDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for TextureUnloadOnDemandStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        TEXTUREUNLOADONDEMANDSTATICSTATE_TYPE_INFO
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


pub static TEXTUREUNLOADONDEMANDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureUnloadOnDemandStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("TextureUnloadOnDemandStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum TextureUnloadOnDemandLifeState {
    #[default]
    TextureUnloadOnDemand_Unloaded = 0,
    TextureUnloadOnDemand_Loaded = 1,
    TextureUnloadOnDemand_Unregistered = 2,
}

pub static TEXTUREUNLOADONDEMANDLIFESTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureUnloadOnDemandLifeState",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(TEXTUREUNLOADONDEMANDLIFESTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TextureUnloadOnDemandLifeState {
    fn type_info(&self) -> &'static TypeInfo {
        TEXTUREUNLOADONDEMANDLIFESTATE_TYPE_INFO
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


pub static TEXTUREUNLOADONDEMANDLIFESTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureUnloadOnDemandLifeState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("TextureUnloadOnDemandLifeState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VertexShaderFragmentHandle {
}

pub trait VertexShaderFragmentHandleTrait: TypeObject {
}

impl VertexShaderFragmentHandleTrait for VertexShaderFragmentHandle {
}

pub static VERTEXSHADERFRAGMENTHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VertexShaderFragmentHandle",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VertexShaderFragmentHandle as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(VERTEXSHADERFRAGMENTHANDLE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for VertexShaderFragmentHandle {
    fn type_info(&self) -> &'static TypeInfo {
        VERTEXSHADERFRAGMENTHANDLE_TYPE_INFO
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


pub static VERTEXSHADERFRAGMENTHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VertexShaderFragmentHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("VertexShaderFragmentHandle"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RenderBufferHandle {
}

pub trait RenderBufferHandleTrait: TypeObject {
}

impl RenderBufferHandleTrait for RenderBufferHandle {
}

pub static RENDERBUFFERHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderBufferHandle",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RenderBufferHandle as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(RENDERBUFFERHANDLE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RenderBufferHandle {
    fn type_info(&self) -> &'static TypeInfo {
        RENDERBUFFERHANDLE_TYPE_INFO
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


pub static RENDERBUFFERHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderBufferHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("RenderBufferHandle"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TextureResourceHandle {
}

pub trait TextureResourceHandleTrait: TypeObject {
}

impl TextureResourceHandleTrait for TextureResourceHandle {
}

pub static TEXTURERESOURCEHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureResourceHandle",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TextureResourceHandle as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TEXTURERESOURCEHANDLE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for TextureResourceHandle {
    fn type_info(&self) -> &'static TypeInfo {
        TEXTURERESOURCEHANDLE_TYPE_INFO
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


pub static TEXTURERESOURCEHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureResourceHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("TextureResourceHandle"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ShaderParameterBlockHandle {
}

pub trait ShaderParameterBlockHandleTrait: TypeObject {
}

impl ShaderParameterBlockHandleTrait for ShaderParameterBlockHandle {
}

pub static SHADERPARAMETERBLOCKHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterBlockHandle",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ShaderParameterBlockHandle as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SHADERPARAMETERBLOCKHANDLE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for ShaderParameterBlockHandle {
    fn type_info(&self) -> &'static TypeInfo {
        SHADERPARAMETERBLOCKHANDLE_TYPE_INFO
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


pub static SHADERPARAMETERBLOCKHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterBlockHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ShaderParameterBlockHandle"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RenderViewHandle {
}

pub trait RenderViewHandleTrait: TypeObject {
}

impl RenderViewHandleTrait for RenderViewHandle {
}

pub static RENDERVIEWHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderViewHandle",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RenderViewHandle as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(RENDERVIEWHANDLE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RenderViewHandle {
    fn type_info(&self) -> &'static TypeInfo {
        RENDERVIEWHANDLE_TYPE_INFO
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


pub static RENDERVIEWHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderViewHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("RenderViewHandle"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CullIdHandle {
}

pub trait CullIdHandleTrait: TypeObject {
}

impl CullIdHandleTrait for CullIdHandle {
}

pub static CULLIDHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CullIdHandle",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CullIdHandle as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CULLIDHANDLE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for CullIdHandle {
    fn type_info(&self) -> &'static TypeInfo {
        CULLIDHANDLE_TYPE_INFO
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


pub static CULLIDHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CullIdHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("CullIdHandle"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SurfaceShaderInstanceDataStruct {
    pub shader: Option<Arc<Mutex<dyn SurfaceShaderBaseAssetTrait>>>,
    pub surface_shader_name: String,
    pub bool_parameters: Vec<BoolShaderParameter>,
    pub vector_parameters: Vec<VectorShaderParameter>,
    pub vector_array_parameters: Vec<VectorArrayShaderParameter>,
    pub texture_parameters: Vec<TextureShaderParameter>,
    pub conditional_parameters: Vec<ConditionalShaderParameter>,
}

pub trait SurfaceShaderInstanceDataStructTrait: TypeObject {
    fn shader(&self) -> &Option<Arc<Mutex<dyn SurfaceShaderBaseAssetTrait>>>;
    fn shader_mut(&mut self) -> &mut Option<Arc<Mutex<dyn SurfaceShaderBaseAssetTrait>>>;
    fn surface_shader_name(&self) -> &String;
    fn surface_shader_name_mut(&mut self) -> &mut String;
    fn bool_parameters(&self) -> &Vec<BoolShaderParameter>;
    fn bool_parameters_mut(&mut self) -> &mut Vec<BoolShaderParameter>;
    fn vector_parameters(&self) -> &Vec<VectorShaderParameter>;
    fn vector_parameters_mut(&mut self) -> &mut Vec<VectorShaderParameter>;
    fn vector_array_parameters(&self) -> &Vec<VectorArrayShaderParameter>;
    fn vector_array_parameters_mut(&mut self) -> &mut Vec<VectorArrayShaderParameter>;
    fn texture_parameters(&self) -> &Vec<TextureShaderParameter>;
    fn texture_parameters_mut(&mut self) -> &mut Vec<TextureShaderParameter>;
    fn conditional_parameters(&self) -> &Vec<ConditionalShaderParameter>;
    fn conditional_parameters_mut(&mut self) -> &mut Vec<ConditionalShaderParameter>;
}

impl SurfaceShaderInstanceDataStructTrait for SurfaceShaderInstanceDataStruct {
    fn shader(&self) -> &Option<Arc<Mutex<dyn SurfaceShaderBaseAssetTrait>>> {
        &self.shader
    }
    fn shader_mut(&mut self) -> &mut Option<Arc<Mutex<dyn SurfaceShaderBaseAssetTrait>>> {
        &mut self.shader
    }
    fn surface_shader_name(&self) -> &String {
        &self.surface_shader_name
    }
    fn surface_shader_name_mut(&mut self) -> &mut String {
        &mut self.surface_shader_name
    }
    fn bool_parameters(&self) -> &Vec<BoolShaderParameter> {
        &self.bool_parameters
    }
    fn bool_parameters_mut(&mut self) -> &mut Vec<BoolShaderParameter> {
        &mut self.bool_parameters
    }
    fn vector_parameters(&self) -> &Vec<VectorShaderParameter> {
        &self.vector_parameters
    }
    fn vector_parameters_mut(&mut self) -> &mut Vec<VectorShaderParameter> {
        &mut self.vector_parameters
    }
    fn vector_array_parameters(&self) -> &Vec<VectorArrayShaderParameter> {
        &self.vector_array_parameters
    }
    fn vector_array_parameters_mut(&mut self) -> &mut Vec<VectorArrayShaderParameter> {
        &mut self.vector_array_parameters
    }
    fn texture_parameters(&self) -> &Vec<TextureShaderParameter> {
        &self.texture_parameters
    }
    fn texture_parameters_mut(&mut self) -> &mut Vec<TextureShaderParameter> {
        &mut self.texture_parameters
    }
    fn conditional_parameters(&self) -> &Vec<ConditionalShaderParameter> {
        &self.conditional_parameters
    }
    fn conditional_parameters_mut(&mut self) -> &mut Vec<ConditionalShaderParameter> {
        &mut self.conditional_parameters
    }
}

pub static SURFACESHADERINSTANCEDATASTRUCT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SurfaceShaderInstanceDataStruct",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SurfaceShaderInstanceDataStruct as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Shader",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderBaseAsset",
                rust_offset: offset_of!(SurfaceShaderInstanceDataStruct, shader),
            },
            FieldInfoData {
                name: "SurfaceShaderName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(SurfaceShaderInstanceDataStruct, surface_shader_name),
            },
            FieldInfoData {
                name: "BoolParameters",
                flags: MemberInfoFlags::new(144),
                field_type: "BoolShaderParameter-Array",
                rust_offset: offset_of!(SurfaceShaderInstanceDataStruct, bool_parameters),
            },
            FieldInfoData {
                name: "VectorParameters",
                flags: MemberInfoFlags::new(144),
                field_type: "VectorShaderParameter-Array",
                rust_offset: offset_of!(SurfaceShaderInstanceDataStruct, vector_parameters),
            },
            FieldInfoData {
                name: "VectorArrayParameters",
                flags: MemberInfoFlags::new(144),
                field_type: "VectorArrayShaderParameter-Array",
                rust_offset: offset_of!(SurfaceShaderInstanceDataStruct, vector_array_parameters),
            },
            FieldInfoData {
                name: "TextureParameters",
                flags: MemberInfoFlags::new(144),
                field_type: "TextureShaderParameter-Array",
                rust_offset: offset_of!(SurfaceShaderInstanceDataStruct, texture_parameters),
            },
            FieldInfoData {
                name: "ConditionalParameters",
                flags: MemberInfoFlags::new(8336),
                field_type: "ConditionalShaderParameter-Array",
                rust_offset: offset_of!(SurfaceShaderInstanceDataStruct, conditional_parameters),
            },
        ],
    }),
    array_type: Some(SURFACESHADERINSTANCEDATASTRUCT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SurfaceShaderInstanceDataStruct {
    fn type_info(&self) -> &'static TypeInfo {
        SURFACESHADERINSTANCEDATASTRUCT_TYPE_INFO
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


pub static SURFACESHADERINSTANCEDATASTRUCT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SurfaceShaderInstanceDataStruct-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("SurfaceShaderInstanceDataStruct"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SurfaceShaderBaseAsset {
    pub _glacier_base: super::core::Asset,
}

pub trait SurfaceShaderBaseAssetTrait: super::core::AssetTrait {
}

impl SurfaceShaderBaseAssetTrait for SurfaceShaderBaseAsset {
}

impl super::core::AssetTrait for SurfaceShaderBaseAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for SurfaceShaderBaseAsset {
}

pub static SURFACESHADERBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SurfaceShaderBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SurfaceShaderBaseAsset as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SURFACESHADERBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SurfaceShaderBaseAsset {
    fn type_info(&self) -> &'static TypeInfo {
        SURFACESHADERBASEASSET_TYPE_INFO
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


pub static SURFACESHADERBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SurfaceShaderBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("SurfaceShaderBaseAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VectorArrayShaderParameter {
    pub parameter_name: String,
    pub parameter_type: ShaderParameterType,
    pub values: Vec<super::core::Vec4>,
}

pub trait VectorArrayShaderParameterTrait: TypeObject {
    fn parameter_name(&self) -> &String;
    fn parameter_name_mut(&mut self) -> &mut String;
    fn parameter_type(&self) -> &ShaderParameterType;
    fn parameter_type_mut(&mut self) -> &mut ShaderParameterType;
    fn values(&self) -> &Vec<super::core::Vec4>;
    fn values_mut(&mut self) -> &mut Vec<super::core::Vec4>;
}

impl VectorArrayShaderParameterTrait for VectorArrayShaderParameter {
    fn parameter_name(&self) -> &String {
        &self.parameter_name
    }
    fn parameter_name_mut(&mut self) -> &mut String {
        &mut self.parameter_name
    }
    fn parameter_type(&self) -> &ShaderParameterType {
        &self.parameter_type
    }
    fn parameter_type_mut(&mut self) -> &mut ShaderParameterType {
        &mut self.parameter_type
    }
    fn values(&self) -> &Vec<super::core::Vec4> {
        &self.values
    }
    fn values_mut(&mut self) -> &mut Vec<super::core::Vec4> {
        &mut self.values
    }
}

pub static VECTORARRAYSHADERPARAMETER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VectorArrayShaderParameter",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VectorArrayShaderParameter as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(VectorArrayShaderParameter, parameter_name),
            },
            FieldInfoData {
                name: "ParameterType",
                flags: MemberInfoFlags::new(0),
                field_type: "ShaderParameterType",
                rust_offset: offset_of!(VectorArrayShaderParameter, parameter_type),
            },
            FieldInfoData {
                name: "Values",
                flags: MemberInfoFlags::new(144),
                field_type: "Vec4-Array",
                rust_offset: offset_of!(VectorArrayShaderParameter, values),
            },
        ],
    }),
    array_type: Some(VECTORARRAYSHADERPARAMETER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VectorArrayShaderParameter {
    fn type_info(&self) -> &'static TypeInfo {
        VECTORARRAYSHADERPARAMETER_TYPE_INFO
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


pub static VECTORARRAYSHADERPARAMETER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VectorArrayShaderParameter-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("VectorArrayShaderParameter"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TextureShaderParameter {
    pub parameter_name: String,
    pub value: Option<Arc<Mutex<dyn TextureBaseAssetTrait>>>,
}

pub trait TextureShaderParameterTrait: TypeObject {
    fn parameter_name(&self) -> &String;
    fn parameter_name_mut(&mut self) -> &mut String;
    fn value(&self) -> &Option<Arc<Mutex<dyn TextureBaseAssetTrait>>>;
    fn value_mut(&mut self) -> &mut Option<Arc<Mutex<dyn TextureBaseAssetTrait>>>;
}

impl TextureShaderParameterTrait for TextureShaderParameter {
    fn parameter_name(&self) -> &String {
        &self.parameter_name
    }
    fn parameter_name_mut(&mut self) -> &mut String {
        &mut self.parameter_name
    }
    fn value(&self) -> &Option<Arc<Mutex<dyn TextureBaseAssetTrait>>> {
        &self.value
    }
    fn value_mut(&mut self) -> &mut Option<Arc<Mutex<dyn TextureBaseAssetTrait>>> {
        &mut self.value
    }
}

pub static TEXTURESHADERPARAMETER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureShaderParameter",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TextureShaderParameter as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TextureShaderParameter, parameter_name),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(TextureShaderParameter, value),
            },
        ],
    }),
    array_type: Some(TEXTURESHADERPARAMETER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TextureShaderParameter {
    fn type_info(&self) -> &'static TypeInfo {
        TEXTURESHADERPARAMETER_TYPE_INFO
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


pub static TEXTURESHADERPARAMETER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureShaderParameter-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("TextureShaderParameter"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VectorShaderParameter {
    pub parameter_name: String,
    pub parameter_type: ShaderParameterType,
    pub value: super::core::Vec4,
}

pub trait VectorShaderParameterTrait: TypeObject {
    fn parameter_name(&self) -> &String;
    fn parameter_name_mut(&mut self) -> &mut String;
    fn parameter_type(&self) -> &ShaderParameterType;
    fn parameter_type_mut(&mut self) -> &mut ShaderParameterType;
    fn value(&self) -> &super::core::Vec4;
    fn value_mut(&mut self) -> &mut super::core::Vec4;
}

impl VectorShaderParameterTrait for VectorShaderParameter {
    fn parameter_name(&self) -> &String {
        &self.parameter_name
    }
    fn parameter_name_mut(&mut self) -> &mut String {
        &mut self.parameter_name
    }
    fn parameter_type(&self) -> &ShaderParameterType {
        &self.parameter_type
    }
    fn parameter_type_mut(&mut self) -> &mut ShaderParameterType {
        &mut self.parameter_type
    }
    fn value(&self) -> &super::core::Vec4 {
        &self.value
    }
    fn value_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.value
    }
}

pub static VECTORSHADERPARAMETER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VectorShaderParameter",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VectorShaderParameter as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(VectorShaderParameter, parameter_name),
            },
            FieldInfoData {
                name: "ParameterType",
                flags: MemberInfoFlags::new(0),
                field_type: "ShaderParameterType",
                rust_offset: offset_of!(VectorShaderParameter, parameter_type),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(VectorShaderParameter, value),
            },
        ],
    }),
    array_type: Some(VECTORSHADERPARAMETER_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for VectorShaderParameter {
    fn type_info(&self) -> &'static TypeInfo {
        VECTORSHADERPARAMETER_TYPE_INFO
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


pub static VECTORSHADERPARAMETER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VectorShaderParameter-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("VectorShaderParameter"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ConditionalShaderParameter {
    pub value: String,
    pub conditional_asset: Option<Arc<Mutex<dyn ExternalShaderConditionalAssetTrait>>>,
}

pub trait ConditionalShaderParameterTrait: TypeObject {
    fn value(&self) -> &String;
    fn value_mut(&mut self) -> &mut String;
    fn conditional_asset(&self) -> &Option<Arc<Mutex<dyn ExternalShaderConditionalAssetTrait>>>;
    fn conditional_asset_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ExternalShaderConditionalAssetTrait>>>;
}

impl ConditionalShaderParameterTrait for ConditionalShaderParameter {
    fn value(&self) -> &String {
        &self.value
    }
    fn value_mut(&mut self) -> &mut String {
        &mut self.value
    }
    fn conditional_asset(&self) -> &Option<Arc<Mutex<dyn ExternalShaderConditionalAssetTrait>>> {
        &self.conditional_asset
    }
    fn conditional_asset_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ExternalShaderConditionalAssetTrait>>> {
        &mut self.conditional_asset
    }
}

pub static CONDITIONALSHADERPARAMETER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalShaderParameter",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ConditionalShaderParameter as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(8192),
                field_type: "CString",
                rust_offset: offset_of!(ConditionalShaderParameter, value),
            },
            FieldInfoData {
                name: "ConditionalAsset",
                flags: MemberInfoFlags::new(8192),
                field_type: "ExternalShaderConditionalAsset",
                rust_offset: offset_of!(ConditionalShaderParameter, conditional_asset),
            },
        ],
    }),
    array_type: Some(CONDITIONALSHADERPARAMETER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ConditionalShaderParameter {
    fn type_info(&self) -> &'static TypeInfo {
        CONDITIONALSHADERPARAMETER_TYPE_INFO
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


pub static CONDITIONALSHADERPARAMETER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalShaderParameter-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ConditionalShaderParameter"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BoolShaderParameter {
    pub parameter_name: String,
    pub value: bool,
}

pub trait BoolShaderParameterTrait: TypeObject {
    fn parameter_name(&self) -> &String;
    fn parameter_name_mut(&mut self) -> &mut String;
    fn value(&self) -> &bool;
    fn value_mut(&mut self) -> &mut bool;
}

impl BoolShaderParameterTrait for BoolShaderParameter {
    fn parameter_name(&self) -> &String {
        &self.parameter_name
    }
    fn parameter_name_mut(&mut self) -> &mut String {
        &mut self.parameter_name
    }
    fn value(&self) -> &bool {
        &self.value
    }
    fn value_mut(&mut self) -> &mut bool {
        &mut self.value
    }
}

pub static BOOLSHADERPARAMETER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoolShaderParameter",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BoolShaderParameter as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(BoolShaderParameter, parameter_name),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(BoolShaderParameter, value),
            },
        ],
    }),
    array_type: Some(BOOLSHADERPARAMETER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BoolShaderParameter {
    fn type_info(&self) -> &'static TypeInfo {
        BOOLSHADERPARAMETER_TYPE_INFO
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


pub static BOOLSHADERPARAMETER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoolShaderParameter-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("BoolShaderParameter"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ShaderParameterDesc {
    pub part0: u64,
    pub part1: u64,
}

pub trait ShaderParameterDescTrait: TypeObject {
    fn part0(&self) -> &u64;
    fn part0_mut(&mut self) -> &mut u64;
    fn part1(&self) -> &u64;
    fn part1_mut(&mut self) -> &mut u64;
}

impl ShaderParameterDescTrait for ShaderParameterDesc {
    fn part0(&self) -> &u64 {
        &self.part0
    }
    fn part0_mut(&mut self) -> &mut u64 {
        &mut self.part0
    }
    fn part1(&self) -> &u64 {
        &self.part1
    }
    fn part1_mut(&mut self) -> &mut u64 {
        &mut self.part1
    }
}

pub static SHADERPARAMETERDESC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterDesc",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ShaderParameterDesc as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Part0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint64",
                rust_offset: offset_of!(ShaderParameterDesc, part0),
            },
            FieldInfoData {
                name: "Part1",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint64",
                rust_offset: offset_of!(ShaderParameterDesc, part1),
            },
        ],
    }),
    array_type: Some(SHADERPARAMETERDESC_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ShaderParameterDesc {
    fn type_info(&self) -> &'static TypeInfo {
        SHADERPARAMETERDESC_TYPE_INFO
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


pub static SHADERPARAMETERDESC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterDesc-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ShaderParameterDesc"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ShaderIndirectSpecularParamType {
    #[default]
    ShaderIndirectSpecularParamType_SpecularIntensity = 0,
    ShaderIndirectSpecularParamType_ReflectanceScale = 1,
    ShaderIndirectSpecularParamType_ProbesSpecularIntensity = 2,
    ShaderIndirectSpecularParamType_ProbesReflectanceScale = 3,
}

pub static SHADERINDIRECTSPECULARPARAMTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderIndirectSpecularParamType",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(SHADERINDIRECTSPECULARPARAMTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShaderIndirectSpecularParamType {
    fn type_info(&self) -> &'static TypeInfo {
        SHADERINDIRECTSPECULARPARAMTYPE_TYPE_INFO
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


pub static SHADERINDIRECTSPECULARPARAMTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderIndirectSpecularParamType-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ShaderIndirectSpecularParamType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static SHADERPARAMETERTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterType",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(SHADERPARAMETERTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShaderParameterType {
    fn type_info(&self) -> &'static TypeInfo {
        SHADERPARAMETERTYPE_TYPE_INFO
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


pub static SHADERPARAMETERTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterType-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ShaderParameterType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ExternalValueConstantType {
    #[default]
    ExternalValueConstantType_Vec = 0,
    ExternalValueConstantType_Bool = 1,
    ExternalValueConstantType_Texture = 2,
    ExternalValueConstantType_Conditional = 3,
}

pub static EXTERNALVALUECONSTANTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExternalValueConstantType",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(EXTERNALVALUECONSTANTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ExternalValueConstantType {
    fn type_info(&self) -> &'static TypeInfo {
        EXTERNALVALUECONSTANTTYPE_TYPE_INFO
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


pub static EXTERNALVALUECONSTANTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExternalValueConstantType-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ExternalValueConstantType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ShaderShadowmapQuality {
    #[default]
    ShaderShadowmapQuality_Low = 0,
    ShaderShadowmapQuality_High = 1,
    ShaderShadowmapQualityCount = 2,
}

pub static SHADERSHADOWMAPQUALITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderShadowmapQuality",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(SHADERSHADOWMAPQUALITY_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShaderShadowmapQuality {
    fn type_info(&self) -> &'static TypeInfo {
        SHADERSHADOWMAPQUALITY_TYPE_INFO
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


pub static SHADERSHADOWMAPQUALITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderShadowmapQuality-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ShaderShadowmapQuality"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ShaderParameterBlockDynamicState {
    pub vecs: Vec<ShaderBlockVec>,
    pub bools: Vec<ShaderBlockBool>,
    pub conditionals: Vec<ShaderBlockConditional>,
    pub textures: Vec<ShaderBlockTexture>,
    pub render_buffers: Vec<ShaderBlockRenderBuffer>,
    pub field_flag_changed0: u8,
}

pub trait ShaderParameterBlockDynamicStateTrait: TypeObject {
    fn vecs(&self) -> &Vec<ShaderBlockVec>;
    fn vecs_mut(&mut self) -> &mut Vec<ShaderBlockVec>;
    fn bools(&self) -> &Vec<ShaderBlockBool>;
    fn bools_mut(&mut self) -> &mut Vec<ShaderBlockBool>;
    fn conditionals(&self) -> &Vec<ShaderBlockConditional>;
    fn conditionals_mut(&mut self) -> &mut Vec<ShaderBlockConditional>;
    fn textures(&self) -> &Vec<ShaderBlockTexture>;
    fn textures_mut(&mut self) -> &mut Vec<ShaderBlockTexture>;
    fn render_buffers(&self) -> &Vec<ShaderBlockRenderBuffer>;
    fn render_buffers_mut(&mut self) -> &mut Vec<ShaderBlockRenderBuffer>;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl ShaderParameterBlockDynamicStateTrait for ShaderParameterBlockDynamicState {
    fn vecs(&self) -> &Vec<ShaderBlockVec> {
        &self.vecs
    }
    fn vecs_mut(&mut self) -> &mut Vec<ShaderBlockVec> {
        &mut self.vecs
    }
    fn bools(&self) -> &Vec<ShaderBlockBool> {
        &self.bools
    }
    fn bools_mut(&mut self) -> &mut Vec<ShaderBlockBool> {
        &mut self.bools
    }
    fn conditionals(&self) -> &Vec<ShaderBlockConditional> {
        &self.conditionals
    }
    fn conditionals_mut(&mut self) -> &mut Vec<ShaderBlockConditional> {
        &mut self.conditionals
    }
    fn textures(&self) -> &Vec<ShaderBlockTexture> {
        &self.textures
    }
    fn textures_mut(&mut self) -> &mut Vec<ShaderBlockTexture> {
        &mut self.textures
    }
    fn render_buffers(&self) -> &Vec<ShaderBlockRenderBuffer> {
        &self.render_buffers
    }
    fn render_buffers_mut(&mut self) -> &mut Vec<ShaderBlockRenderBuffer> {
        &mut self.render_buffers
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static SHADERPARAMETERBLOCKDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterBlockDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ShaderParameterBlockDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Vecs",
                flags: MemberInfoFlags::new(144),
                field_type: "ShaderBlockVec-Array",
                rust_offset: offset_of!(ShaderParameterBlockDynamicState, vecs),
            },
            FieldInfoData {
                name: "Bools",
                flags: MemberInfoFlags::new(144),
                field_type: "ShaderBlockBool-Array",
                rust_offset: offset_of!(ShaderParameterBlockDynamicState, bools),
            },
            FieldInfoData {
                name: "Conditionals",
                flags: MemberInfoFlags::new(144),
                field_type: "ShaderBlockConditional-Array",
                rust_offset: offset_of!(ShaderParameterBlockDynamicState, conditionals),
            },
            FieldInfoData {
                name: "Textures",
                flags: MemberInfoFlags::new(144),
                field_type: "ShaderBlockTexture-Array",
                rust_offset: offset_of!(ShaderParameterBlockDynamicState, textures),
            },
            FieldInfoData {
                name: "RenderBuffers",
                flags: MemberInfoFlags::new(144),
                field_type: "ShaderBlockRenderBuffer-Array",
                rust_offset: offset_of!(ShaderParameterBlockDynamicState, render_buffers),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(ShaderParameterBlockDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SHADERPARAMETERBLOCKDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ShaderParameterBlockDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        SHADERPARAMETERBLOCKDYNAMICSTATE_TYPE_INFO
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


pub static SHADERPARAMETERBLOCKDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterBlockDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ShaderParameterBlockDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ShaderBlockRenderBuffer {
    pub name: ShaderBlockParameter,
    pub value: RenderBufferHandle,
}

pub trait ShaderBlockRenderBufferTrait: TypeObject {
    fn name(&self) -> &ShaderBlockParameter;
    fn name_mut(&mut self) -> &mut ShaderBlockParameter;
    fn value(&self) -> &RenderBufferHandle;
    fn value_mut(&mut self) -> &mut RenderBufferHandle;
}

impl ShaderBlockRenderBufferTrait for ShaderBlockRenderBuffer {
    fn name(&self) -> &ShaderBlockParameter {
        &self.name
    }
    fn name_mut(&mut self) -> &mut ShaderBlockParameter {
        &mut self.name
    }
    fn value(&self) -> &RenderBufferHandle {
        &self.value
    }
    fn value_mut(&mut self) -> &mut RenderBufferHandle {
        &mut self.value
    }
}

pub static SHADERBLOCKRENDERBUFFER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderBlockRenderBuffer",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ShaderBlockRenderBuffer as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: "ShaderBlockParameter",
                rust_offset: offset_of!(ShaderBlockRenderBuffer, name),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "RenderBufferHandle",
                rust_offset: offset_of!(ShaderBlockRenderBuffer, value),
            },
        ],
    }),
    array_type: Some(SHADERBLOCKRENDERBUFFER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ShaderBlockRenderBuffer {
    fn type_info(&self) -> &'static TypeInfo {
        SHADERBLOCKRENDERBUFFER_TYPE_INFO
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


pub static SHADERBLOCKRENDERBUFFER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderBlockRenderBuffer-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ShaderBlockRenderBuffer"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ShaderBlockTexture {
    pub name: ShaderBlockParameter,
    pub value: TextureResourceHandle,
}

pub trait ShaderBlockTextureTrait: TypeObject {
    fn name(&self) -> &ShaderBlockParameter;
    fn name_mut(&mut self) -> &mut ShaderBlockParameter;
    fn value(&self) -> &TextureResourceHandle;
    fn value_mut(&mut self) -> &mut TextureResourceHandle;
}

impl ShaderBlockTextureTrait for ShaderBlockTexture {
    fn name(&self) -> &ShaderBlockParameter {
        &self.name
    }
    fn name_mut(&mut self) -> &mut ShaderBlockParameter {
        &mut self.name
    }
    fn value(&self) -> &TextureResourceHandle {
        &self.value
    }
    fn value_mut(&mut self) -> &mut TextureResourceHandle {
        &mut self.value
    }
}

pub static SHADERBLOCKTEXTURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderBlockTexture",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ShaderBlockTexture as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: "ShaderBlockParameter",
                rust_offset: offset_of!(ShaderBlockTexture, name),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureResourceHandle",
                rust_offset: offset_of!(ShaderBlockTexture, value),
            },
        ],
    }),
    array_type: Some(SHADERBLOCKTEXTURE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ShaderBlockTexture {
    fn type_info(&self) -> &'static TypeInfo {
        SHADERBLOCKTEXTURE_TYPE_INFO
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


pub static SHADERBLOCKTEXTURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderBlockTexture-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ShaderBlockTexture"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ShaderBlockVec {
    pub name: ShaderBlockParameter,
    pub values: Vec<super::core::Vec4>,
}

pub trait ShaderBlockVecTrait: TypeObject {
    fn name(&self) -> &ShaderBlockParameter;
    fn name_mut(&mut self) -> &mut ShaderBlockParameter;
    fn values(&self) -> &Vec<super::core::Vec4>;
    fn values_mut(&mut self) -> &mut Vec<super::core::Vec4>;
}

impl ShaderBlockVecTrait for ShaderBlockVec {
    fn name(&self) -> &ShaderBlockParameter {
        &self.name
    }
    fn name_mut(&mut self) -> &mut ShaderBlockParameter {
        &mut self.name
    }
    fn values(&self) -> &Vec<super::core::Vec4> {
        &self.values
    }
    fn values_mut(&mut self) -> &mut Vec<super::core::Vec4> {
        &mut self.values
    }
}

pub static SHADERBLOCKVEC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderBlockVec",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ShaderBlockVec as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: "ShaderBlockParameter",
                rust_offset: offset_of!(ShaderBlockVec, name),
            },
            FieldInfoData {
                name: "Values",
                flags: MemberInfoFlags::new(144),
                field_type: "Vec4-Array",
                rust_offset: offset_of!(ShaderBlockVec, values),
            },
        ],
    }),
    array_type: Some(SHADERBLOCKVEC_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ShaderBlockVec {
    fn type_info(&self) -> &'static TypeInfo {
        SHADERBLOCKVEC_TYPE_INFO
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


pub static SHADERBLOCKVEC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderBlockVec-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ShaderBlockVec"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ShaderBlockConditional {
    pub name: ShaderBlockParameter,
    pub value: u8,
}

pub trait ShaderBlockConditionalTrait: TypeObject {
    fn name(&self) -> &ShaderBlockParameter;
    fn name_mut(&mut self) -> &mut ShaderBlockParameter;
    fn value(&self) -> &u8;
    fn value_mut(&mut self) -> &mut u8;
}

impl ShaderBlockConditionalTrait for ShaderBlockConditional {
    fn name(&self) -> &ShaderBlockParameter {
        &self.name
    }
    fn name_mut(&mut self) -> &mut ShaderBlockParameter {
        &mut self.name
    }
    fn value(&self) -> &u8 {
        &self.value
    }
    fn value_mut(&mut self) -> &mut u8 {
        &mut self.value
    }
}

pub static SHADERBLOCKCONDITIONAL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderBlockConditional",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ShaderBlockConditional as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: "ShaderBlockParameter",
                rust_offset: offset_of!(ShaderBlockConditional, name),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(ShaderBlockConditional, value),
            },
        ],
    }),
    array_type: Some(SHADERBLOCKCONDITIONAL_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ShaderBlockConditional {
    fn type_info(&self) -> &'static TypeInfo {
        SHADERBLOCKCONDITIONAL_TYPE_INFO
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


pub static SHADERBLOCKCONDITIONAL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderBlockConditional-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ShaderBlockConditional"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ShaderBlockBool {
    pub name: ShaderBlockParameter,
    pub value: bool,
}

pub trait ShaderBlockBoolTrait: TypeObject {
    fn name(&self) -> &ShaderBlockParameter;
    fn name_mut(&mut self) -> &mut ShaderBlockParameter;
    fn value(&self) -> &bool;
    fn value_mut(&mut self) -> &mut bool;
}

impl ShaderBlockBoolTrait for ShaderBlockBool {
    fn name(&self) -> &ShaderBlockParameter {
        &self.name
    }
    fn name_mut(&mut self) -> &mut ShaderBlockParameter {
        &mut self.name
    }
    fn value(&self) -> &bool {
        &self.value
    }
    fn value_mut(&mut self) -> &mut bool {
        &mut self.value
    }
}

pub static SHADERBLOCKBOOL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderBlockBool",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ShaderBlockBool as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: "ShaderBlockParameter",
                rust_offset: offset_of!(ShaderBlockBool, name),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ShaderBlockBool, value),
            },
        ],
    }),
    array_type: Some(SHADERBLOCKBOOL_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ShaderBlockBool {
    fn type_info(&self) -> &'static TypeInfo {
        SHADERBLOCKBOOL_TYPE_INFO
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


pub static SHADERBLOCKBOOL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderBlockBool-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ShaderBlockBool"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ShaderBlockParameter {
    pub part0: u64,
    pub part1: u64,
}

pub trait ShaderBlockParameterTrait: TypeObject {
    fn part0(&self) -> &u64;
    fn part0_mut(&mut self) -> &mut u64;
    fn part1(&self) -> &u64;
    fn part1_mut(&mut self) -> &mut u64;
}

impl ShaderBlockParameterTrait for ShaderBlockParameter {
    fn part0(&self) -> &u64 {
        &self.part0
    }
    fn part0_mut(&mut self) -> &mut u64 {
        &mut self.part0
    }
    fn part1(&self) -> &u64 {
        &self.part1
    }
    fn part1_mut(&mut self) -> &mut u64 {
        &mut self.part1
    }
}

pub static SHADERBLOCKPARAMETER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderBlockParameter",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ShaderBlockParameter as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Part0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint64",
                rust_offset: offset_of!(ShaderBlockParameter, part0),
            },
            FieldInfoData {
                name: "Part1",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint64",
                rust_offset: offset_of!(ShaderBlockParameter, part1),
            },
        ],
    }),
    array_type: Some(SHADERBLOCKPARAMETER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ShaderBlockParameter {
    fn type_info(&self) -> &'static TypeInfo {
        SHADERBLOCKPARAMETER_TYPE_INFO
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


pub static SHADERBLOCKPARAMETER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderBlockParameter-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ShaderBlockParameter"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ShaderParameterBlockStaticState {
}

pub trait ShaderParameterBlockStaticStateTrait: TypeObject {
}

impl ShaderParameterBlockStaticStateTrait for ShaderParameterBlockStaticState {
}

pub static SHADERPARAMETERBLOCKSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterBlockStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ShaderParameterBlockStaticState as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SHADERPARAMETERBLOCKSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShaderParameterBlockStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        SHADERPARAMETERBLOCKSTATICSTATE_TYPE_INFO
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


pub static SHADERPARAMETERBLOCKSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterBlockStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ShaderParameterBlockStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ScreenshotDynamicState {
    pub force_auto_render: bool,
    pub trigger_auto_render_delay: i32,
    pub is_frozen: bool,
    pub field_flag_changed0: u8,
}

pub trait ScreenshotDynamicStateTrait: TypeObject {
    fn force_auto_render(&self) -> &bool;
    fn force_auto_render_mut(&mut self) -> &mut bool;
    fn trigger_auto_render_delay(&self) -> &i32;
    fn trigger_auto_render_delay_mut(&mut self) -> &mut i32;
    fn is_frozen(&self) -> &bool;
    fn is_frozen_mut(&mut self) -> &mut bool;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl ScreenshotDynamicStateTrait for ScreenshotDynamicState {
    fn force_auto_render(&self) -> &bool {
        &self.force_auto_render
    }
    fn force_auto_render_mut(&mut self) -> &mut bool {
        &mut self.force_auto_render
    }
    fn trigger_auto_render_delay(&self) -> &i32 {
        &self.trigger_auto_render_delay
    }
    fn trigger_auto_render_delay_mut(&mut self) -> &mut i32 {
        &mut self.trigger_auto_render_delay
    }
    fn is_frozen(&self) -> &bool {
        &self.is_frozen
    }
    fn is_frozen_mut(&mut self) -> &mut bool {
        &mut self.is_frozen
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static SCREENSHOTDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenshotDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ScreenshotDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ForceAutoRender",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ScreenshotDynamicState, force_auto_render),
            },
            FieldInfoData {
                name: "TriggerAutoRenderDelay",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ScreenshotDynamicState, trigger_auto_render_delay),
            },
            FieldInfoData {
                name: "IsFrozen",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ScreenshotDynamicState, is_frozen),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(ScreenshotDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SCREENSHOTDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for ScreenshotDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        SCREENSHOTDYNAMICSTATE_TYPE_INFO
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


pub static SCREENSHOTDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenshotDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ScreenshotDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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

pub trait ScreenshotStateTrait: TypeObject {
    fn explicit_file_name(&self) -> &String;
    fn explicit_file_name_mut(&mut self) -> &mut String;
    fn format(&self) -> &ScreenshotFormat;
    fn format_mut(&mut self) -> &mut ScreenshotFormat;
    fn layer_mode(&self) -> &ScreenshotLayerMode;
    fn layer_mode_mut(&mut self) -> &mut ScreenshotLayerMode;
    fn frame_naming_mode(&self) -> &ScreenshotFrameNamingMode;
    fn frame_naming_mode_mut(&mut self) -> &mut ScreenshotFrameNamingMode;
    fn anti_alias_multiplier(&self) -> &i32;
    fn anti_alias_multiplier_mut(&mut self) -> &mut i32;
    fn resolution_multiplier(&self) -> &i32;
    fn resolution_multiplier_mut(&mut self) -> &mut i32;
    fn png_compression_level(&self) -> &i32;
    fn png_compression_level_mut(&mut self) -> &mut i32;
    fn alpha_enable(&self) -> &bool;
    fn alpha_enable_mut(&mut self) -> &mut bool;
    fn surround_capture(&self) -> &bool;
    fn surround_capture_mut(&mut self) -> &mut bool;
    fn use_native_file_system(&self) -> &bool;
    fn use_native_file_system_mut(&mut self) -> &mut bool;
    fn starting_x_pos(&self) -> &i32;
    fn starting_x_pos_mut(&mut self) -> &mut i32;
    fn starting_y_pos(&self) -> &i32;
    fn starting_y_pos_mut(&mut self) -> &mut i32;
    fn width(&self) -> &i32;
    fn width_mut(&mut self) -> &mut i32;
    fn height(&self) -> &i32;
    fn height_mut(&mut self) -> &mut i32;
    fn delay_frames(&self) -> &i32;
    fn delay_frames_mut(&mut self) -> &mut i32;
    fn upload_to_juice(&self) -> &bool;
    fn upload_to_juice_mut(&mut self) -> &mut bool;
    fn overwrite(&self) -> &bool;
    fn overwrite_mut(&mut self) -> &mut bool;
    fn auto_render_delay(&self) -> &i32;
    fn auto_render_delay_mut(&mut self) -> &mut i32;
    fn frame_padding(&self) -> &u32;
    fn frame_padding_mut(&mut self) -> &mut u32;
    fn by_frame(&self) -> &u32;
    fn by_frame_mut(&mut self) -> &mut u32;
    fn reset_counter(&self) -> &bool;
    fn reset_counter_mut(&mut self) -> &mut bool;
    fn field_flag_changed0(&self) -> &u32;
    fn field_flag_changed0_mut(&mut self) -> &mut u32;
}

impl ScreenshotStateTrait for ScreenshotState {
    fn explicit_file_name(&self) -> &String {
        &self.explicit_file_name
    }
    fn explicit_file_name_mut(&mut self) -> &mut String {
        &mut self.explicit_file_name
    }
    fn format(&self) -> &ScreenshotFormat {
        &self.format
    }
    fn format_mut(&mut self) -> &mut ScreenshotFormat {
        &mut self.format
    }
    fn layer_mode(&self) -> &ScreenshotLayerMode {
        &self.layer_mode
    }
    fn layer_mode_mut(&mut self) -> &mut ScreenshotLayerMode {
        &mut self.layer_mode
    }
    fn frame_naming_mode(&self) -> &ScreenshotFrameNamingMode {
        &self.frame_naming_mode
    }
    fn frame_naming_mode_mut(&mut self) -> &mut ScreenshotFrameNamingMode {
        &mut self.frame_naming_mode
    }
    fn anti_alias_multiplier(&self) -> &i32 {
        &self.anti_alias_multiplier
    }
    fn anti_alias_multiplier_mut(&mut self) -> &mut i32 {
        &mut self.anti_alias_multiplier
    }
    fn resolution_multiplier(&self) -> &i32 {
        &self.resolution_multiplier
    }
    fn resolution_multiplier_mut(&mut self) -> &mut i32 {
        &mut self.resolution_multiplier
    }
    fn png_compression_level(&self) -> &i32 {
        &self.png_compression_level
    }
    fn png_compression_level_mut(&mut self) -> &mut i32 {
        &mut self.png_compression_level
    }
    fn alpha_enable(&self) -> &bool {
        &self.alpha_enable
    }
    fn alpha_enable_mut(&mut self) -> &mut bool {
        &mut self.alpha_enable
    }
    fn surround_capture(&self) -> &bool {
        &self.surround_capture
    }
    fn surround_capture_mut(&mut self) -> &mut bool {
        &mut self.surround_capture
    }
    fn use_native_file_system(&self) -> &bool {
        &self.use_native_file_system
    }
    fn use_native_file_system_mut(&mut self) -> &mut bool {
        &mut self.use_native_file_system
    }
    fn starting_x_pos(&self) -> &i32 {
        &self.starting_x_pos
    }
    fn starting_x_pos_mut(&mut self) -> &mut i32 {
        &mut self.starting_x_pos
    }
    fn starting_y_pos(&self) -> &i32 {
        &self.starting_y_pos
    }
    fn starting_y_pos_mut(&mut self) -> &mut i32 {
        &mut self.starting_y_pos
    }
    fn width(&self) -> &i32 {
        &self.width
    }
    fn width_mut(&mut self) -> &mut i32 {
        &mut self.width
    }
    fn height(&self) -> &i32 {
        &self.height
    }
    fn height_mut(&mut self) -> &mut i32 {
        &mut self.height
    }
    fn delay_frames(&self) -> &i32 {
        &self.delay_frames
    }
    fn delay_frames_mut(&mut self) -> &mut i32 {
        &mut self.delay_frames
    }
    fn upload_to_juice(&self) -> &bool {
        &self.upload_to_juice
    }
    fn upload_to_juice_mut(&mut self) -> &mut bool {
        &mut self.upload_to_juice
    }
    fn overwrite(&self) -> &bool {
        &self.overwrite
    }
    fn overwrite_mut(&mut self) -> &mut bool {
        &mut self.overwrite
    }
    fn auto_render_delay(&self) -> &i32 {
        &self.auto_render_delay
    }
    fn auto_render_delay_mut(&mut self) -> &mut i32 {
        &mut self.auto_render_delay
    }
    fn frame_padding(&self) -> &u32 {
        &self.frame_padding
    }
    fn frame_padding_mut(&mut self) -> &mut u32 {
        &mut self.frame_padding
    }
    fn by_frame(&self) -> &u32 {
        &self.by_frame
    }
    fn by_frame_mut(&mut self) -> &mut u32 {
        &mut self.by_frame
    }
    fn reset_counter(&self) -> &bool {
        &self.reset_counter
    }
    fn reset_counter_mut(&mut self) -> &mut bool {
        &mut self.reset_counter
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u32 {
        &mut self.field_flag_changed0
    }
}

pub static SCREENSHOTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenshotState",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ScreenshotState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ExplicitFileName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ScreenshotState, explicit_file_name),
            },
            FieldInfoData {
                name: "Format",
                flags: MemberInfoFlags::new(0),
                field_type: "ScreenshotFormat",
                rust_offset: offset_of!(ScreenshotState, format),
            },
            FieldInfoData {
                name: "LayerMode",
                flags: MemberInfoFlags::new(0),
                field_type: "ScreenshotLayerMode",
                rust_offset: offset_of!(ScreenshotState, layer_mode),
            },
            FieldInfoData {
                name: "FrameNamingMode",
                flags: MemberInfoFlags::new(0),
                field_type: "ScreenshotFrameNamingMode",
                rust_offset: offset_of!(ScreenshotState, frame_naming_mode),
            },
            FieldInfoData {
                name: "AntiAliasMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ScreenshotState, anti_alias_multiplier),
            },
            FieldInfoData {
                name: "ResolutionMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ScreenshotState, resolution_multiplier),
            },
            FieldInfoData {
                name: "PngCompressionLevel",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ScreenshotState, png_compression_level),
            },
            FieldInfoData {
                name: "AlphaEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ScreenshotState, alpha_enable),
            },
            FieldInfoData {
                name: "SurroundCapture",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ScreenshotState, surround_capture),
            },
            FieldInfoData {
                name: "UseNativeFileSystem",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ScreenshotState, use_native_file_system),
            },
            FieldInfoData {
                name: "StartingXPos",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ScreenshotState, starting_x_pos),
            },
            FieldInfoData {
                name: "StartingYPos",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ScreenshotState, starting_y_pos),
            },
            FieldInfoData {
                name: "Width",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ScreenshotState, width),
            },
            FieldInfoData {
                name: "Height",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ScreenshotState, height),
            },
            FieldInfoData {
                name: "DelayFrames",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ScreenshotState, delay_frames),
            },
            FieldInfoData {
                name: "UploadToJuice",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ScreenshotState, upload_to_juice),
            },
            FieldInfoData {
                name: "Overwrite",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ScreenshotState, overwrite),
            },
            FieldInfoData {
                name: "AutoRenderDelay",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ScreenshotState, auto_render_delay),
            },
            FieldInfoData {
                name: "FramePadding",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ScreenshotState, frame_padding),
            },
            FieldInfoData {
                name: "ByFrame",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ScreenshotState, by_frame),
            },
            FieldInfoData {
                name: "ResetCounter",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ScreenshotState, reset_counter),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ScreenshotState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SCREENSHOTSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ScreenshotState {
    fn type_info(&self) -> &'static TypeInfo {
        SCREENSHOTSTATE_TYPE_INFO
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


pub static SCREENSHOTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenshotState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ScreenshotState"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ScreenshotFrameNamingMode {
    #[default]
    ScreenshotFrameNamingMode_Sequential = 0,
    ScreenshotFrameNamingMode_Absolute = 1,
}

pub static SCREENSHOTFRAMENAMINGMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenshotFrameNamingMode",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(SCREENSHOTFRAMENAMINGMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ScreenshotFrameNamingMode {
    fn type_info(&self) -> &'static TypeInfo {
        SCREENSHOTFRAMENAMINGMODE_TYPE_INFO
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


pub static SCREENSHOTFRAMENAMINGMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenshotFrameNamingMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ScreenshotFrameNamingMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ScreenshotLayerMode {
    #[default]
    ScreenshotLayerMode_Single = 0,
    ScreenshotLayerMode_Common = 1,
    ScreenshotLayerMode_DLSS = 2,
    ScreenshotLayerMode_All = 3,
}

pub static SCREENSHOTLAYERMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenshotLayerMode",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(SCREENSHOTLAYERMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ScreenshotLayerMode {
    fn type_info(&self) -> &'static TypeInfo {
        SCREENSHOTLAYERMODE_TYPE_INFO
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


pub static SCREENSHOTLAYERMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenshotLayerMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ScreenshotLayerMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static SCREENSHOTFORMAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenshotFormat",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(SCREENSHOTFORMAT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ScreenshotFormat {
    fn type_info(&self) -> &'static TypeInfo {
        SCREENSHOTFORMAT_TYPE_INFO
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


pub static SCREENSHOTFORMAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenshotFormat-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ScreenshotFormat"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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

pub trait ViewDefinitionTrait: TypeObject {
    fn view_id(&self) -> &LocalPlayerViewId;
    fn view_id_mut(&mut self) -> &mut LocalPlayerViewId;
    fn view_type(&self) -> &ViewDefinitionType;
    fn view_type_mut(&mut self) -> &mut ViewDefinitionType;
    fn screen_index(&self) -> &u32;
    fn screen_index_mut(&mut self) -> &mut u32;
    fn normalized_size(&self) -> &bool;
    fn normalized_size_mut(&mut self) -> &mut bool;
    fn offset_x(&self) -> &f32;
    fn offset_x_mut(&mut self) -> &mut f32;
    fn offset_y(&self) -> &f32;
    fn offset_y_mut(&mut self) -> &mut f32;
    fn width(&self) -> &f32;
    fn width_mut(&mut self) -> &mut f32;
    fn height(&self) -> &f32;
    fn height_mut(&mut self) -> &mut f32;
    fn fov_scale(&self) -> &f32;
    fn fov_scale_mut(&mut self) -> &mut f32;
}

impl ViewDefinitionTrait for ViewDefinition {
    fn view_id(&self) -> &LocalPlayerViewId {
        &self.view_id
    }
    fn view_id_mut(&mut self) -> &mut LocalPlayerViewId {
        &mut self.view_id
    }
    fn view_type(&self) -> &ViewDefinitionType {
        &self.view_type
    }
    fn view_type_mut(&mut self) -> &mut ViewDefinitionType {
        &mut self.view_type
    }
    fn screen_index(&self) -> &u32 {
        &self.screen_index
    }
    fn screen_index_mut(&mut self) -> &mut u32 {
        &mut self.screen_index
    }
    fn normalized_size(&self) -> &bool {
        &self.normalized_size
    }
    fn normalized_size_mut(&mut self) -> &mut bool {
        &mut self.normalized_size
    }
    fn offset_x(&self) -> &f32 {
        &self.offset_x
    }
    fn offset_x_mut(&mut self) -> &mut f32 {
        &mut self.offset_x
    }
    fn offset_y(&self) -> &f32 {
        &self.offset_y
    }
    fn offset_y_mut(&mut self) -> &mut f32 {
        &mut self.offset_y
    }
    fn width(&self) -> &f32 {
        &self.width
    }
    fn width_mut(&mut self) -> &mut f32 {
        &mut self.width
    }
    fn height(&self) -> &f32 {
        &self.height
    }
    fn height_mut(&mut self) -> &mut f32 {
        &mut self.height
    }
    fn fov_scale(&self) -> &f32 {
        &self.fov_scale
    }
    fn fov_scale_mut(&mut self) -> &mut f32 {
        &mut self.fov_scale
    }
}

pub static VIEWDEFINITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ViewDefinition",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ViewDefinition as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ViewId",
                flags: MemberInfoFlags::new(0),
                field_type: "LocalPlayerViewId",
                rust_offset: offset_of!(ViewDefinition, view_id),
            },
            FieldInfoData {
                name: "ViewType",
                flags: MemberInfoFlags::new(0),
                field_type: "ViewDefinitionType",
                rust_offset: offset_of!(ViewDefinition, view_type),
            },
            FieldInfoData {
                name: "ScreenIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ViewDefinition, screen_index),
            },
            FieldInfoData {
                name: "NormalizedSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ViewDefinition, normalized_size),
            },
            FieldInfoData {
                name: "OffsetX",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ViewDefinition, offset_x),
            },
            FieldInfoData {
                name: "OffsetY",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ViewDefinition, offset_y),
            },
            FieldInfoData {
                name: "Width",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ViewDefinition, width),
            },
            FieldInfoData {
                name: "Height",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ViewDefinition, height),
            },
            FieldInfoData {
                name: "FovScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ViewDefinition, fov_scale),
            },
        ],
    }),
    array_type: Some(VIEWDEFINITION_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for ViewDefinition {
    fn type_info(&self) -> &'static TypeInfo {
        VIEWDEFINITION_TYPE_INFO
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


pub static VIEWDEFINITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ViewDefinition-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ViewDefinition"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ViewDefinitionType {
    #[default]
    ViewType_FullScreen = 0,
    ViewType_AutoVerticalSplit = 1,
    ViewType_AutoFullHorizontalSplit = 2,
    ViewType_AutoOffsetedHorizontalSplit = 3,
    ViewType_AutoQuadrant = 4,
    ViewType_Custom = 5,
}

pub static VIEWDEFINITIONTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ViewDefinitionType",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(VIEWDEFINITIONTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ViewDefinitionType {
    fn type_info(&self) -> &'static TypeInfo {
        VIEWDEFINITIONTYPE_TYPE_INFO
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


pub static VIEWDEFINITIONTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ViewDefinitionType-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ViewDefinitionType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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

pub trait PhysicalCameraDescTrait: TypeObject {
    fn exposure_compensation(&self) -> &f32;
    fn exposure_compensation_mut(&mut self) -> &mut f32;
    fn e_v100(&self) -> &f32;
    fn e_v100_mut(&mut self) -> &mut f32;
    fn shutter_speed(&self) -> &f32;
    fn shutter_speed_mut(&mut self) -> &mut f32;
    fn aperture(&self) -> &f32;
    fn aperture_mut(&mut self) -> &mut f32;
    fn i_s_o(&self) -> &f32;
    fn i_s_o_mut(&mut self) -> &mut f32;
    fn focal_length(&self) -> &f32;
    fn focal_length_mut(&mut self) -> &mut f32;
    fn fov(&self) -> &f32;
    fn fov_mut(&mut self) -> &mut f32;
    fn focus_distance(&self) -> &f32;
    fn focus_distance_mut(&mut self) -> &mut f32;
    fn sensor_width(&self) -> &f32;
    fn sensor_width_mut(&mut self) -> &mut f32;
    fn sensor_height(&self) -> &f32;
    fn sensor_height_mut(&mut self) -> &mut f32;
    fn spot_meter_scale(&self) -> &f32;
    fn spot_meter_scale_mut(&mut self) -> &mut f32;
    fn spot_meter_offset(&self) -> &super::core::Vec2;
    fn spot_meter_offset_mut(&mut self) -> &mut super::core::Vec2;
    fn use_lens_breathing(&self) -> &bool;
    fn use_lens_breathing_mut(&mut self) -> &mut bool;
    fn use_camera_exposure(&self) -> &bool;
    fn use_camera_exposure_mut(&mut self) -> &mut bool;
    fn auto_exposure_method(&self) -> &AutoExposureMethod;
    fn auto_exposure_method_mut(&mut self) -> &mut AutoExposureMethod;
}

impl PhysicalCameraDescTrait for PhysicalCameraDesc {
    fn exposure_compensation(&self) -> &f32 {
        &self.exposure_compensation
    }
    fn exposure_compensation_mut(&mut self) -> &mut f32 {
        &mut self.exposure_compensation
    }
    fn e_v100(&self) -> &f32 {
        &self.e_v100
    }
    fn e_v100_mut(&mut self) -> &mut f32 {
        &mut self.e_v100
    }
    fn shutter_speed(&self) -> &f32 {
        &self.shutter_speed
    }
    fn shutter_speed_mut(&mut self) -> &mut f32 {
        &mut self.shutter_speed
    }
    fn aperture(&self) -> &f32 {
        &self.aperture
    }
    fn aperture_mut(&mut self) -> &mut f32 {
        &mut self.aperture
    }
    fn i_s_o(&self) -> &f32 {
        &self.i_s_o
    }
    fn i_s_o_mut(&mut self) -> &mut f32 {
        &mut self.i_s_o
    }
    fn focal_length(&self) -> &f32 {
        &self.focal_length
    }
    fn focal_length_mut(&mut self) -> &mut f32 {
        &mut self.focal_length
    }
    fn fov(&self) -> &f32 {
        &self.fov
    }
    fn fov_mut(&mut self) -> &mut f32 {
        &mut self.fov
    }
    fn focus_distance(&self) -> &f32 {
        &self.focus_distance
    }
    fn focus_distance_mut(&mut self) -> &mut f32 {
        &mut self.focus_distance
    }
    fn sensor_width(&self) -> &f32 {
        &self.sensor_width
    }
    fn sensor_width_mut(&mut self) -> &mut f32 {
        &mut self.sensor_width
    }
    fn sensor_height(&self) -> &f32 {
        &self.sensor_height
    }
    fn sensor_height_mut(&mut self) -> &mut f32 {
        &mut self.sensor_height
    }
    fn spot_meter_scale(&self) -> &f32 {
        &self.spot_meter_scale
    }
    fn spot_meter_scale_mut(&mut self) -> &mut f32 {
        &mut self.spot_meter_scale
    }
    fn spot_meter_offset(&self) -> &super::core::Vec2 {
        &self.spot_meter_offset
    }
    fn spot_meter_offset_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.spot_meter_offset
    }
    fn use_lens_breathing(&self) -> &bool {
        &self.use_lens_breathing
    }
    fn use_lens_breathing_mut(&mut self) -> &mut bool {
        &mut self.use_lens_breathing
    }
    fn use_camera_exposure(&self) -> &bool {
        &self.use_camera_exposure
    }
    fn use_camera_exposure_mut(&mut self) -> &mut bool {
        &mut self.use_camera_exposure
    }
    fn auto_exposure_method(&self) -> &AutoExposureMethod {
        &self.auto_exposure_method
    }
    fn auto_exposure_method_mut(&mut self) -> &mut AutoExposureMethod {
        &mut self.auto_exposure_method
    }
}

pub static PHYSICALCAMERADESC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicalCameraDesc",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicalCameraDesc as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ExposureCompensation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PhysicalCameraDesc, exposure_compensation),
            },
            FieldInfoData {
                name: "EV100",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PhysicalCameraDesc, e_v100),
            },
            FieldInfoData {
                name: "ShutterSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PhysicalCameraDesc, shutter_speed),
            },
            FieldInfoData {
                name: "Aperture",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PhysicalCameraDesc, aperture),
            },
            FieldInfoData {
                name: "ISO",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PhysicalCameraDesc, i_s_o),
            },
            FieldInfoData {
                name: "FocalLength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PhysicalCameraDesc, focal_length),
            },
            FieldInfoData {
                name: "Fov",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PhysicalCameraDesc, fov),
            },
            FieldInfoData {
                name: "FocusDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PhysicalCameraDesc, focus_distance),
            },
            FieldInfoData {
                name: "SensorWidth",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PhysicalCameraDesc, sensor_width),
            },
            FieldInfoData {
                name: "SensorHeight",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PhysicalCameraDesc, sensor_height),
            },
            FieldInfoData {
                name: "SpotMeterScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PhysicalCameraDesc, spot_meter_scale),
            },
            FieldInfoData {
                name: "SpotMeterOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(PhysicalCameraDesc, spot_meter_offset),
            },
            FieldInfoData {
                name: "UseLensBreathing",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicalCameraDesc, use_lens_breathing),
            },
            FieldInfoData {
                name: "UseCameraExposure",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PhysicalCameraDesc, use_camera_exposure),
            },
            FieldInfoData {
                name: "AutoExposureMethod",
                flags: MemberInfoFlags::new(0),
                field_type: "AutoExposureMethod",
                rust_offset: offset_of!(PhysicalCameraDesc, auto_exposure_method),
            },
        ],
    }),
    array_type: Some(PHYSICALCAMERADESC_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for PhysicalCameraDesc {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICALCAMERADESC_TYPE_INFO
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


pub static PHYSICALCAMERADESC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicalCameraDesc-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("PhysicalCameraDesc"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum AutoExposureMethod {
    #[default]
    AutoExposureMethod_None = 0,
    AutoExposureMethod_MipAverage = 1,
    AutoExposureMethod_GaussianPyramid = 2,
    AutoExposureMethod_HistogramAverage = 3,
    AutoExposureMethod_UseGlobalSetting = 4,
}

pub static AUTOEXPOSUREMETHOD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoExposureMethod",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(AUTOEXPOSUREMETHOD_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AutoExposureMethod {
    fn type_info(&self) -> &'static TypeInfo {
        AUTOEXPOSUREMETHOD_TYPE_INFO
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


pub static AUTOEXPOSUREMETHOD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoExposureMethod-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("AutoExposureMethod"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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

pub trait RenderViewDynamicStateTrait: TypeObject {
    fn transform(&self) -> &super::state_stream::TransformSpaceHandle;
    fn transform_mut(&mut self) -> &mut super::state_stream::TransformSpaceHandle;
    fn visible(&self) -> &bool;
    fn visible_mut(&mut self) -> &mut bool;
    fn viewport(&self) -> &ViewportRect;
    fn viewport_mut(&mut self) -> &mut ViewportRect;
    fn viewport_offset(&self) -> &super::core::Vec2;
    fn viewport_offset_mut(&mut self) -> &mut super::core::Vec2;
    fn physical_camera_enabled(&self) -> &bool;
    fn physical_camera_enabled_mut(&mut self) -> &mut bool;
    fn physical_camera_desc(&self) -> &PhysicalCameraDesc;
    fn physical_camera_desc_mut(&mut self) -> &mut PhysicalCameraDesc;
    fn secondary_streaming_view_enabled(&self) -> &bool;
    fn secondary_streaming_view_enabled_mut(&mut self) -> &mut bool;
    fn secondary_streaming_view_transform(&self) -> &super::core::LinearTransform;
    fn secondary_streaming_view_transform_mut(&mut self) -> &mut super::core::LinearTransform;
    fn secondary_streaming_view_fov_rad(&self) -> &f32;
    fn secondary_streaming_view_fov_rad_mut(&mut self) -> &mut f32;
    fn near_plane(&self) -> &f32;
    fn near_plane_mut(&mut self) -> &mut f32;
    fn far_plane(&self) -> &f32;
    fn far_plane_mut(&mut self) -> &mut f32;
    fn aspect(&self) -> &f32;
    fn aspect_mut(&mut self) -> &mut f32;
    fn fov_rad(&self) -> &f32;
    fn fov_rad_mut(&mut self) -> &mut f32;
    fn blur_amount(&self) -> &f32;
    fn blur_amount_mut(&mut self) -> &mut f32;
    fn dof_focus_distance(&self) -> &f32;
    fn dof_focus_distance_mut(&mut self) -> &mut f32;
    fn shadowmap_min_fov(&self) -> &f32;
    fn shadowmap_min_fov_mut(&mut self) -> &mut f32;
    fn shadowmap_distance_scale(&self) -> &f32;
    fn shadowmap_distance_scale_mut(&mut self) -> &mut f32;
    fn luminance_texture_u_v_scale(&self) -> &f32;
    fn luminance_texture_u_v_scale_mut(&mut self) -> &mut f32;
    fn world_fade_amount(&self) -> &f32;
    fn world_fade_amount_mut(&mut self) -> &mut f32;
    fn camera_cut_this_frame(&self) -> &bool;
    fn camera_cut_this_frame_mut(&mut self) -> &mut bool;
    fn use_physical_camera_changed(&self) -> &bool;
    fn use_physical_camera_changed_mut(&mut self) -> &mut bool;
    fn camera_cut_sync_id(&self) -> &u32;
    fn camera_cut_sync_id_mut(&mut self) -> &mut u32;
    fn screen_index(&self) -> &u32;
    fn screen_index_mut(&mut self) -> &mut u32;
    fn cull_ids(&self) -> &Vec<CullIdHandle>;
    fn cull_ids_mut(&mut self) -> &mut Vec<CullIdHandle>;
    fn render_view_feature_mask(&self) -> &u32;
    fn render_view_feature_mask_mut(&mut self) -> &mut u32;
    fn field_flag_changed0(&self) -> &u32;
    fn field_flag_changed0_mut(&mut self) -> &mut u32;
}

impl RenderViewDynamicStateTrait for RenderViewDynamicState {
    fn transform(&self) -> &super::state_stream::TransformSpaceHandle {
        &self.transform
    }
    fn transform_mut(&mut self) -> &mut super::state_stream::TransformSpaceHandle {
        &mut self.transform
    }
    fn visible(&self) -> &bool {
        &self.visible
    }
    fn visible_mut(&mut self) -> &mut bool {
        &mut self.visible
    }
    fn viewport(&self) -> &ViewportRect {
        &self.viewport
    }
    fn viewport_mut(&mut self) -> &mut ViewportRect {
        &mut self.viewport
    }
    fn viewport_offset(&self) -> &super::core::Vec2 {
        &self.viewport_offset
    }
    fn viewport_offset_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.viewport_offset
    }
    fn physical_camera_enabled(&self) -> &bool {
        &self.physical_camera_enabled
    }
    fn physical_camera_enabled_mut(&mut self) -> &mut bool {
        &mut self.physical_camera_enabled
    }
    fn physical_camera_desc(&self) -> &PhysicalCameraDesc {
        &self.physical_camera_desc
    }
    fn physical_camera_desc_mut(&mut self) -> &mut PhysicalCameraDesc {
        &mut self.physical_camera_desc
    }
    fn secondary_streaming_view_enabled(&self) -> &bool {
        &self.secondary_streaming_view_enabled
    }
    fn secondary_streaming_view_enabled_mut(&mut self) -> &mut bool {
        &mut self.secondary_streaming_view_enabled
    }
    fn secondary_streaming_view_transform(&self) -> &super::core::LinearTransform {
        &self.secondary_streaming_view_transform
    }
    fn secondary_streaming_view_transform_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.secondary_streaming_view_transform
    }
    fn secondary_streaming_view_fov_rad(&self) -> &f32 {
        &self.secondary_streaming_view_fov_rad
    }
    fn secondary_streaming_view_fov_rad_mut(&mut self) -> &mut f32 {
        &mut self.secondary_streaming_view_fov_rad
    }
    fn near_plane(&self) -> &f32 {
        &self.near_plane
    }
    fn near_plane_mut(&mut self) -> &mut f32 {
        &mut self.near_plane
    }
    fn far_plane(&self) -> &f32 {
        &self.far_plane
    }
    fn far_plane_mut(&mut self) -> &mut f32 {
        &mut self.far_plane
    }
    fn aspect(&self) -> &f32 {
        &self.aspect
    }
    fn aspect_mut(&mut self) -> &mut f32 {
        &mut self.aspect
    }
    fn fov_rad(&self) -> &f32 {
        &self.fov_rad
    }
    fn fov_rad_mut(&mut self) -> &mut f32 {
        &mut self.fov_rad
    }
    fn blur_amount(&self) -> &f32 {
        &self.blur_amount
    }
    fn blur_amount_mut(&mut self) -> &mut f32 {
        &mut self.blur_amount
    }
    fn dof_focus_distance(&self) -> &f32 {
        &self.dof_focus_distance
    }
    fn dof_focus_distance_mut(&mut self) -> &mut f32 {
        &mut self.dof_focus_distance
    }
    fn shadowmap_min_fov(&self) -> &f32 {
        &self.shadowmap_min_fov
    }
    fn shadowmap_min_fov_mut(&mut self) -> &mut f32 {
        &mut self.shadowmap_min_fov
    }
    fn shadowmap_distance_scale(&self) -> &f32 {
        &self.shadowmap_distance_scale
    }
    fn shadowmap_distance_scale_mut(&mut self) -> &mut f32 {
        &mut self.shadowmap_distance_scale
    }
    fn luminance_texture_u_v_scale(&self) -> &f32 {
        &self.luminance_texture_u_v_scale
    }
    fn luminance_texture_u_v_scale_mut(&mut self) -> &mut f32 {
        &mut self.luminance_texture_u_v_scale
    }
    fn world_fade_amount(&self) -> &f32 {
        &self.world_fade_amount
    }
    fn world_fade_amount_mut(&mut self) -> &mut f32 {
        &mut self.world_fade_amount
    }
    fn camera_cut_this_frame(&self) -> &bool {
        &self.camera_cut_this_frame
    }
    fn camera_cut_this_frame_mut(&mut self) -> &mut bool {
        &mut self.camera_cut_this_frame
    }
    fn use_physical_camera_changed(&self) -> &bool {
        &self.use_physical_camera_changed
    }
    fn use_physical_camera_changed_mut(&mut self) -> &mut bool {
        &mut self.use_physical_camera_changed
    }
    fn camera_cut_sync_id(&self) -> &u32 {
        &self.camera_cut_sync_id
    }
    fn camera_cut_sync_id_mut(&mut self) -> &mut u32 {
        &mut self.camera_cut_sync_id
    }
    fn screen_index(&self) -> &u32 {
        &self.screen_index
    }
    fn screen_index_mut(&mut self) -> &mut u32 {
        &mut self.screen_index
    }
    fn cull_ids(&self) -> &Vec<CullIdHandle> {
        &self.cull_ids
    }
    fn cull_ids_mut(&mut self) -> &mut Vec<CullIdHandle> {
        &mut self.cull_ids
    }
    fn render_view_feature_mask(&self) -> &u32 {
        &self.render_view_feature_mask
    }
    fn render_view_feature_mask_mut(&mut self) -> &mut u32 {
        &mut self.render_view_feature_mask
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u32 {
        &mut self.field_flag_changed0
    }
}

pub static RENDERVIEWDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderViewDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RenderViewDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: "TransformSpaceHandle",
                rust_offset: offset_of!(RenderViewDynamicState, transform),
            },
            FieldInfoData {
                name: "Visible",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RenderViewDynamicState, visible),
            },
            FieldInfoData {
                name: "Viewport",
                flags: MemberInfoFlags::new(0),
                field_type: "ViewportRect",
                rust_offset: offset_of!(RenderViewDynamicState, viewport),
            },
            FieldInfoData {
                name: "ViewportOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(RenderViewDynamicState, viewport_offset),
            },
            FieldInfoData {
                name: "PhysicalCameraEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RenderViewDynamicState, physical_camera_enabled),
            },
            FieldInfoData {
                name: "PhysicalCameraDesc",
                flags: MemberInfoFlags::new(0),
                field_type: "PhysicalCameraDesc",
                rust_offset: offset_of!(RenderViewDynamicState, physical_camera_desc),
            },
            FieldInfoData {
                name: "SecondaryStreamingViewEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RenderViewDynamicState, secondary_streaming_view_enabled),
            },
            FieldInfoData {
                name: "SecondaryStreamingViewTransform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(RenderViewDynamicState, secondary_streaming_view_transform),
            },
            FieldInfoData {
                name: "SecondaryStreamingViewFovRad",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RenderViewDynamicState, secondary_streaming_view_fov_rad),
            },
            FieldInfoData {
                name: "NearPlane",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RenderViewDynamicState, near_plane),
            },
            FieldInfoData {
                name: "FarPlane",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RenderViewDynamicState, far_plane),
            },
            FieldInfoData {
                name: "Aspect",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RenderViewDynamicState, aspect),
            },
            FieldInfoData {
                name: "FovRad",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RenderViewDynamicState, fov_rad),
            },
            FieldInfoData {
                name: "BlurAmount",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RenderViewDynamicState, blur_amount),
            },
            FieldInfoData {
                name: "DofFocusDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RenderViewDynamicState, dof_focus_distance),
            },
            FieldInfoData {
                name: "ShadowmapMinFov",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RenderViewDynamicState, shadowmap_min_fov),
            },
            FieldInfoData {
                name: "ShadowmapDistanceScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RenderViewDynamicState, shadowmap_distance_scale),
            },
            FieldInfoData {
                name: "LuminanceTextureUVScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RenderViewDynamicState, luminance_texture_u_v_scale),
            },
            FieldInfoData {
                name: "WorldFadeAmount",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RenderViewDynamicState, world_fade_amount),
            },
            FieldInfoData {
                name: "CameraCutThisFrame",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RenderViewDynamicState, camera_cut_this_frame),
            },
            FieldInfoData {
                name: "UsePhysicalCameraChanged",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RenderViewDynamicState, use_physical_camera_changed),
            },
            FieldInfoData {
                name: "CameraCutSyncId",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(RenderViewDynamicState, camera_cut_sync_id),
            },
            FieldInfoData {
                name: "ScreenIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(RenderViewDynamicState, screen_index),
            },
            FieldInfoData {
                name: "CullIds",
                flags: MemberInfoFlags::new(144),
                field_type: "CullIdHandle-Array",
                rust_offset: offset_of!(RenderViewDynamicState, cull_ids),
            },
            FieldInfoData {
                name: "RenderViewFeatureMask",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(RenderViewDynamicState, render_view_feature_mask),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(RenderViewDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(RENDERVIEWDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RenderViewDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        RENDERVIEWDYNAMICSTATE_TYPE_INFO
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


pub static RENDERVIEWDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderViewDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("RenderViewDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RenderViewStaticState {
    pub priority: u32,
    pub target_texture: Option<Arc<Mutex<dyn RenderTextureBaseAssetTrait>>>,
    pub debug_name: String,
    pub field_flag_changed0: u8,
}

pub trait RenderViewStaticStateTrait: TypeObject {
    fn priority(&self) -> &u32;
    fn priority_mut(&mut self) -> &mut u32;
    fn target_texture(&self) -> &Option<Arc<Mutex<dyn RenderTextureBaseAssetTrait>>>;
    fn target_texture_mut(&mut self) -> &mut Option<Arc<Mutex<dyn RenderTextureBaseAssetTrait>>>;
    fn debug_name(&self) -> &String;
    fn debug_name_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl RenderViewStaticStateTrait for RenderViewStaticState {
    fn priority(&self) -> &u32 {
        &self.priority
    }
    fn priority_mut(&mut self) -> &mut u32 {
        &mut self.priority
    }
    fn target_texture(&self) -> &Option<Arc<Mutex<dyn RenderTextureBaseAssetTrait>>> {
        &self.target_texture
    }
    fn target_texture_mut(&mut self) -> &mut Option<Arc<Mutex<dyn RenderTextureBaseAssetTrait>>> {
        &mut self.target_texture
    }
    fn debug_name(&self) -> &String {
        &self.debug_name
    }
    fn debug_name_mut(&mut self) -> &mut String {
        &mut self.debug_name
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static RENDERVIEWSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderViewStaticState",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RenderViewStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Priority",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(RenderViewStaticState, priority),
            },
            FieldInfoData {
                name: "TargetTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "RenderTextureBaseAsset",
                rust_offset: offset_of!(RenderViewStaticState, target_texture),
            },
            FieldInfoData {
                name: "DebugName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(RenderViewStaticState, debug_name),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(RenderViewStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(RENDERVIEWSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RenderViewStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        RENDERVIEWSTATICSTATE_TYPE_INFO
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


pub static RENDERVIEWSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderViewStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("RenderViewStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ScissorRect {
}

pub trait ScissorRectTrait: TypeObject {
}

impl ScissorRectTrait for ScissorRect {
}

pub static SCISSORRECT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScissorRect",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ScissorRect as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SCISSORRECT_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for ScissorRect {
    fn type_info(&self) -> &'static TypeInfo {
        SCISSORRECT_TYPE_INFO
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


pub static SCISSORRECT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScissorRect-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ScissorRect"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ViewportRect {
}

pub trait ViewportRectTrait: TypeObject {
}

impl ViewportRectTrait for ViewportRect {
}

pub static VIEWPORTRECT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ViewportRect",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ViewportRect as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(VIEWPORTRECT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ViewportRect {
    fn type_info(&self) -> &'static TypeInfo {
        VIEWPORTRECT_TYPE_INFO
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


pub static VIEWPORTRECT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ViewportRect-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ViewportRect"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MeshComputeBaseNode {
    pub _glacier_base: super::core::Node,
}

pub trait MeshComputeBaseNodeTrait: super::core::NodeTrait {
}

impl MeshComputeBaseNodeTrait for MeshComputeBaseNode {
}

impl super::core::NodeTrait for MeshComputeBaseNode {
}

impl super::core::DataContainerTrait for MeshComputeBaseNode {
}

pub static MESHCOMPUTEBASENODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshComputeBaseNode",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::NODE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MeshComputeBaseNode as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(MESHCOMPUTEBASENODE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshComputeBaseNode {
    fn type_info(&self) -> &'static TypeInfo {
        MESHCOMPUTEBASENODE_TYPE_INFO
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


pub static MESHCOMPUTEBASENODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshComputeBaseNode-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("MeshComputeBaseNode"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MeshComputeBaseAsset {
    pub _glacier_base: super::core::NodeGraph,
}

pub trait MeshComputeBaseAssetTrait: super::core::NodeGraphTrait {
}

impl MeshComputeBaseAssetTrait for MeshComputeBaseAsset {
}

impl super::core::NodeGraphTrait for MeshComputeBaseAsset {
}

impl super::core::AssetTrait for MeshComputeBaseAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for MeshComputeBaseAsset {
}

pub static MESHCOMPUTEBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshComputeBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::NODEGRAPH_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MeshComputeBaseAsset as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(MESHCOMPUTEBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshComputeBaseAsset {
    fn type_info(&self) -> &'static TypeInfo {
        MESHCOMPUTEBASEASSET_TYPE_INFO
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


pub static MESHCOMPUTEBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshComputeBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("MeshComputeBaseAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct StaticEnlightenBaseAsset {
    pub _glacier_base: super::core::Asset,
}

pub trait StaticEnlightenBaseAssetTrait: super::core::AssetTrait {
}

impl StaticEnlightenBaseAssetTrait for StaticEnlightenBaseAsset {
}

impl super::core::AssetTrait for StaticEnlightenBaseAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for StaticEnlightenBaseAsset {
}

pub static STATICENLIGHTENBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticEnlightenBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StaticEnlightenBaseAsset as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(STATICENLIGHTENBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StaticEnlightenBaseAsset {
    fn type_info(&self) -> &'static TypeInfo {
        STATICENLIGHTENBASEASSET_TYPE_INFO
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


pub static STATICENLIGHTENBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticEnlightenBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("StaticEnlightenBaseAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EnlightenRuntimeConfigBaseAsset {
    pub _glacier_base: super::core::Asset,
}

pub trait EnlightenRuntimeConfigBaseAssetTrait: super::core::AssetTrait {
}

impl EnlightenRuntimeConfigBaseAssetTrait for EnlightenRuntimeConfigBaseAsset {
}

impl super::core::AssetTrait for EnlightenRuntimeConfigBaseAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for EnlightenRuntimeConfigBaseAsset {
}

pub static ENLIGHTENRUNTIMECONFIGBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenRuntimeConfigBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EnlightenRuntimeConfigBaseAsset as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(ENLIGHTENRUNTIMECONFIGBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EnlightenRuntimeConfigBaseAsset {
    fn type_info(&self) -> &'static TypeInfo {
        ENLIGHTENRUNTIMECONFIGBASEASSET_TYPE_INFO
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


pub static ENLIGHTENRUNTIMECONFIGBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenRuntimeConfigBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("EnlightenRuntimeConfigBaseAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EnlightenShaderDatabaseBaseAsset {
    pub _glacier_base: super::core::Asset,
}

pub trait EnlightenShaderDatabaseBaseAssetTrait: super::core::AssetTrait {
}

impl EnlightenShaderDatabaseBaseAssetTrait for EnlightenShaderDatabaseBaseAsset {
}

impl super::core::AssetTrait for EnlightenShaderDatabaseBaseAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for EnlightenShaderDatabaseBaseAsset {
}

pub static ENLIGHTENSHADERDATABASEBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenShaderDatabaseBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EnlightenShaderDatabaseBaseAsset as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(ENLIGHTENSHADERDATABASEBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EnlightenShaderDatabaseBaseAsset {
    fn type_info(&self) -> &'static TypeInfo {
        ENLIGHTENSHADERDATABASEBASEASSET_TYPE_INFO
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


pub static ENLIGHTENSHADERDATABASEBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenShaderDatabaseBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("EnlightenShaderDatabaseBaseAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EnlightenBaseAsset {
    pub _glacier_base: super::core::Asset,
    pub mix_feature_mode: RadiosityMixMode,
}

pub trait EnlightenBaseAssetTrait: super::core::AssetTrait {
    fn mix_feature_mode(&self) -> &RadiosityMixMode;
    fn mix_feature_mode_mut(&mut self) -> &mut RadiosityMixMode;
}

impl EnlightenBaseAssetTrait for EnlightenBaseAsset {
    fn mix_feature_mode(&self) -> &RadiosityMixMode {
        &self.mix_feature_mode
    }
    fn mix_feature_mode_mut(&mut self) -> &mut RadiosityMixMode {
        &mut self.mix_feature_mode
    }
}

impl super::core::AssetTrait for EnlightenBaseAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for EnlightenBaseAsset {
}

pub static ENLIGHTENBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EnlightenBaseAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MixFeatureMode",
                flags: MemberInfoFlags::new(0),
                field_type: "RadiosityMixMode",
                rust_offset: offset_of!(EnlightenBaseAsset, mix_feature_mode),
            },
        ],
    }),
    array_type: Some(ENLIGHTENBASEASSET_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EnlightenBaseAsset {
    fn type_info(&self) -> &'static TypeInfo {
        ENLIGHTENBASEASSET_TYPE_INFO
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


pub static ENLIGHTENBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("EnlightenBaseAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MeshLodGroupBaseAsset {
    pub _glacier_base: super::core::Asset,
}

pub trait MeshLodGroupBaseAssetTrait: super::core::AssetTrait {
}

impl MeshLodGroupBaseAssetTrait for MeshLodGroupBaseAsset {
}

impl super::core::AssetTrait for MeshLodGroupBaseAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for MeshLodGroupBaseAsset {
}

pub static MESHLODGROUPBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshLodGroupBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MeshLodGroupBaseAsset as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(MESHLODGROUPBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshLodGroupBaseAsset {
    fn type_info(&self) -> &'static TypeInfo {
        MESHLODGROUPBASEASSET_TYPE_INFO
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


pub static MESHLODGROUPBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshLodGroupBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("MeshLodGroupBaseAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MeshBaseAsset {
    pub _glacier_base: super::core::Asset,
}

pub trait MeshBaseAssetTrait: super::core::AssetTrait {
}

impl MeshBaseAssetTrait for MeshBaseAsset {
}

impl super::core::AssetTrait for MeshBaseAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for MeshBaseAsset {
}

pub static MESHBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MeshBaseAsset as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(MESHBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshBaseAsset {
    fn type_info(&self) -> &'static TypeInfo {
        MESHBASEASSET_TYPE_INFO
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


pub static MESHBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("MeshBaseAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum RadiosityMixMode {
    #[default]
    RadiosityMixMode_Disabled = 0,
    RadiosityMixMode_EnableWithPermutations = 1,
    RadiosityMixMode_EnableWithoutPermutations = 2,
}

pub static RADIOSITYMIXMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityMixMode",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(RADIOSITYMIXMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RadiosityMixMode {
    fn type_info(&self) -> &'static TypeInfo {
        RADIOSITYMIXMODE_TYPE_INFO
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


pub static RADIOSITYMIXMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityMixMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("RadiosityMixMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum RadiosityBackfaceType {
    #[default]
    RadiosityBackfaceType_Invalid = 0,
    RadiosityBackfaceType_Black = 1,
    RadiosityBackfaceType_Translucent = 2,
    RadiosityBackfaceType_Transparent = 3,
}

pub static RADIOSITYBACKFACETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityBackfaceType",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(RADIOSITYBACKFACETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RadiosityBackfaceType {
    fn type_info(&self) -> &'static TypeInfo {
        RADIOSITYBACKFACETYPE_TYPE_INFO
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


pub static RADIOSITYBACKFACETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityBackfaceType-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("RadiosityBackfaceType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum MeshType {
    #[default]
    MeshType_Rigid = 0,
    MeshType_Skinned = 1,
    MeshType_Composite = 2,
    MeshType_Count = 3,
}

pub static MESHTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshType",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(MESHTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MeshType {
    fn type_info(&self) -> &'static TypeInfo {
        MESHTYPE_TYPE_INFO
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


pub static MESHTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshType-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("MeshType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum CloudShadowType {
    #[default]
    CloudShadowType_Disabled = 0,
    CloudShadowType_Texture = 1,
    CloudShadowType_Volumetric = 2,
    CloudShadowTypeCount = 3,
}

pub static CLOUDSHADOWTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CloudShadowType",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(CLOUDSHADOWTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CloudShadowType {
    fn type_info(&self) -> &'static TypeInfo {
        CLOUDSHADOWTYPE_TYPE_INFO
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


pub static CLOUDSHADOWTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CloudShadowType-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("CloudShadowType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum TextureAddress {
    #[default]
    TaWrap = 0,
    TaMirror = 1,
    TaClamp = 2,
    TaBorder = 4,
    TaMirrorOnce = 5,
}

pub static TEXTUREADDRESS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureAddress",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(TEXTUREADDRESS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TextureAddress {
    fn type_info(&self) -> &'static TypeInfo {
        TEXTUREADDRESS_TYPE_INFO
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


pub static TEXTUREADDRESS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureAddress-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("TextureAddress"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RenderTextureBaseAsset {
    pub _glacier_base: TextureBaseAsset,
}

pub trait RenderTextureBaseAssetTrait: TextureBaseAssetTrait {
}

impl RenderTextureBaseAssetTrait for RenderTextureBaseAsset {
}

impl TextureBaseAssetTrait for RenderTextureBaseAsset {
    fn resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        self._glacier_base.resource()
    }
    fn resource_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef {
        self._glacier_base.resource_mut()
    }
}

impl super::core::AssetTrait for RenderTextureBaseAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for RenderTextureBaseAsset {
}

pub static RENDERTEXTUREBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderTextureBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TEXTUREBASEASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RenderTextureBaseAsset as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(RENDERTEXTUREBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RenderTextureBaseAsset {
    fn type_info(&self) -> &'static TypeInfo {
        RENDERTEXTUREBASEASSET_TYPE_INFO
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


pub static RENDERTEXTUREBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderTextureBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("RenderTextureBaseAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct IPGraphCrossReferenceTextureAsset {
    pub _glacier_base: TextureBaseAsset,
    pub source_overlay_id: u32,
}

pub trait IPGraphCrossReferenceTextureAssetTrait: TextureBaseAssetTrait {
    fn source_overlay_id(&self) -> &u32;
    fn source_overlay_id_mut(&mut self) -> &mut u32;
}

impl IPGraphCrossReferenceTextureAssetTrait for IPGraphCrossReferenceTextureAsset {
    fn source_overlay_id(&self) -> &u32 {
        &self.source_overlay_id
    }
    fn source_overlay_id_mut(&mut self) -> &mut u32 {
        &mut self.source_overlay_id
    }
}

impl TextureBaseAssetTrait for IPGraphCrossReferenceTextureAsset {
    fn resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        self._glacier_base.resource()
    }
    fn resource_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef {
        self._glacier_base.resource_mut()
    }
}

impl super::core::AssetTrait for IPGraphCrossReferenceTextureAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for IPGraphCrossReferenceTextureAsset {
}

pub static IPGRAPHCROSSREFERENCETEXTUREASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IPGraphCrossReferenceTextureAsset",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TEXTUREBASEASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IPGraphCrossReferenceTextureAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "SourceOverlayId",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(IPGraphCrossReferenceTextureAsset, source_overlay_id),
            },
        ],
    }),
    array_type: Some(IPGRAPHCROSSREFERENCETEXTUREASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IPGraphCrossReferenceTextureAsset {
    fn type_info(&self) -> &'static TypeInfo {
        IPGRAPHCROSSREFERENCETEXTUREASSET_TYPE_INFO
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


pub static IPGRAPHCROSSREFERENCETEXTUREASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IPGraphCrossReferenceTextureAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("IPGraphCrossReferenceTextureAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TextureBaseAsset {
    pub _glacier_base: super::core::Asset,
    pub resource: glacier_reflect::builtin::ResourceRef,
}

pub trait TextureBaseAssetTrait: super::core::AssetTrait {
    fn resource(&self) -> &glacier_reflect::builtin::ResourceRef;
    fn resource_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef;
}

impl TextureBaseAssetTrait for TextureBaseAsset {
    fn resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.resource
    }
    fn resource_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef {
        &mut self.resource
    }
}

impl super::core::AssetTrait for TextureBaseAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for TextureBaseAsset {
}

pub static TEXTUREBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TextureBaseAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Resource",
                flags: MemberInfoFlags::new(0),
                field_type: "ResourceRef",
                rust_offset: offset_of!(TextureBaseAsset, resource),
            },
        ],
    }),
    array_type: Some(TEXTUREBASEASSET_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TextureBaseAsset {
    fn type_info(&self) -> &'static TypeInfo {
        TEXTUREBASEASSET_TYPE_INFO
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


pub static TEXTUREBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("TextureBaseAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PerformanceSummaryMessage {
}

pub trait PerformanceSummaryMessageTrait: TypeObject {
}

impl PerformanceSummaryMessageTrait for PerformanceSummaryMessage {
}

pub static PERFORMANCESUMMARYMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerformanceSummaryMessage",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PerformanceSummaryMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for PerformanceSummaryMessage {
    fn type_info(&self) -> &'static TypeInfo {
        PERFORMANCESUMMARYMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct PerformanceClientMessage {
}

pub trait PerformanceClientMessageTrait: TypeObject {
}

impl PerformanceClientMessageTrait for PerformanceClientMessage {
}

pub static PERFORMANCECLIENTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerformanceClientMessage",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PerformanceClientMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for PerformanceClientMessage {
    fn type_info(&self) -> &'static TypeInfo {
        PERFORMANCECLIENTMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct PerformanceShaderMessage {
}

pub trait PerformanceShaderMessageTrait: TypeObject {
}

impl PerformanceShaderMessageTrait for PerformanceShaderMessage {
}

pub static PERFORMANCESHADERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerformanceShaderMessage",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PerformanceShaderMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for PerformanceShaderMessage {
    fn type_info(&self) -> &'static TypeInfo {
        PERFORMANCESHADERMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ShaderStatsMessage {
}

pub trait ShaderStatsMessageTrait: TypeObject {
}

impl ShaderStatsMessageTrait for ShaderStatsMessage {
}

pub static SHADERSTATSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderStatsMessage",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ShaderStatsMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SHADERSTATSMESSAGE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for ShaderStatsMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SHADERSTATSMESSAGE_TYPE_INFO
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


pub static SHADERSTATSMESSAGE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderStatsMessage-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ShaderStatsMessage"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PerformanceMessage {
}

pub trait PerformanceMessageTrait: TypeObject {
}

impl PerformanceMessageTrait for PerformanceMessage {
}

pub static PERFORMANCEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerformanceMessage",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PerformanceMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PERFORMANCEMESSAGE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for PerformanceMessage {
    fn type_info(&self) -> &'static TypeInfo {
        PERFORMANCEMESSAGE_TYPE_INFO
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


pub static PERFORMANCEMESSAGE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerformanceMessage-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("PerformanceMessage"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RenderBufferDynamicState {
}

pub trait RenderBufferDynamicStateTrait: TypeObject {
}

impl RenderBufferDynamicStateTrait for RenderBufferDynamicState {
}

pub static RENDERBUFFERDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderBufferDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RenderBufferDynamicState as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(RENDERBUFFERDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RenderBufferDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        RENDERBUFFERDYNAMICSTATE_TYPE_INFO
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


pub static RENDERBUFFERDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderBufferDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("RenderBufferDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RenderBufferStaticState {
    pub element_count: i32,
    pub element_size: i32,
    pub field_flag_changed0: u8,
}

pub trait RenderBufferStaticStateTrait: TypeObject {
    fn element_count(&self) -> &i32;
    fn element_count_mut(&mut self) -> &mut i32;
    fn element_size(&self) -> &i32;
    fn element_size_mut(&mut self) -> &mut i32;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl RenderBufferStaticStateTrait for RenderBufferStaticState {
    fn element_count(&self) -> &i32 {
        &self.element_count
    }
    fn element_count_mut(&mut self) -> &mut i32 {
        &mut self.element_count
    }
    fn element_size(&self) -> &i32 {
        &self.element_size
    }
    fn element_size_mut(&mut self) -> &mut i32 {
        &mut self.element_size
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static RENDERBUFFERSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderBufferStaticState",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RenderBufferStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ElementCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(RenderBufferStaticState, element_count),
            },
            FieldInfoData {
                name: "ElementSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(RenderBufferStaticState, element_size),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(RenderBufferStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(RENDERBUFFERSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RenderBufferStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        RENDERBUFFERSTATICSTATE_TYPE_INFO
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


pub static RENDERBUFFERSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderBufferStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("RenderBufferStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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

pub trait LensReflectionComponentStateTrait: TypeObject {
    fn enable(&self) -> &bool;
    fn enable_mut(&mut self) -> &mut bool;
    fn inner_color(&self) -> &super::core::Vec3;
    fn inner_color_mut(&mut self) -> &mut super::core::Vec3;
    fn outer_color(&self) -> &super::core::Vec3;
    fn outer_color_mut(&mut self) -> &mut super::core::Vec3;
    fn mix_start(&self) -> &f32;
    fn mix_start_mut(&mut self) -> &mut f32;
    fn mix_stop(&self) -> &f32;
    fn mix_stop_mut(&mut self) -> &mut f32;
    fn input_exponent(&self) -> &f32;
    fn input_exponent_mut(&mut self) -> &mut f32;
    fn luminance_threshold(&self) -> &f32;
    fn luminance_threshold_mut(&mut self) -> &mut f32;
    fn input_scale(&self) -> &f32;
    fn input_scale_mut(&mut self) -> &mut f32;
    fn max_opacity(&self) -> &f32;
    fn max_opacity_mut(&mut self) -> &mut f32;
    fn transpose_reflection(&self) -> &bool;
    fn transpose_reflection_mut(&mut self) -> &mut bool;
    fn scale(&self) -> &f32;
    fn scale_mut(&mut self) -> &mut f32;
    fn distortion_factor(&self) -> &f32;
    fn distortion_factor_mut(&mut self) -> &mut f32;
    fn vertical_stretch(&self) -> &f32;
    fn vertical_stretch_mut(&mut self) -> &mut f32;
    fn field_flag_override0(&self) -> &u16;
    fn field_flag_override0_mut(&mut self) -> &mut u16;
    fn field_flag_changed0(&self) -> &u16;
    fn field_flag_changed0_mut(&mut self) -> &mut u16;
}

impl LensReflectionComponentStateTrait for LensReflectionComponentState {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn enable_mut(&mut self) -> &mut bool {
        &mut self.enable
    }
    fn inner_color(&self) -> &super::core::Vec3 {
        &self.inner_color
    }
    fn inner_color_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.inner_color
    }
    fn outer_color(&self) -> &super::core::Vec3 {
        &self.outer_color
    }
    fn outer_color_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.outer_color
    }
    fn mix_start(&self) -> &f32 {
        &self.mix_start
    }
    fn mix_start_mut(&mut self) -> &mut f32 {
        &mut self.mix_start
    }
    fn mix_stop(&self) -> &f32 {
        &self.mix_stop
    }
    fn mix_stop_mut(&mut self) -> &mut f32 {
        &mut self.mix_stop
    }
    fn input_exponent(&self) -> &f32 {
        &self.input_exponent
    }
    fn input_exponent_mut(&mut self) -> &mut f32 {
        &mut self.input_exponent
    }
    fn luminance_threshold(&self) -> &f32 {
        &self.luminance_threshold
    }
    fn luminance_threshold_mut(&mut self) -> &mut f32 {
        &mut self.luminance_threshold
    }
    fn input_scale(&self) -> &f32 {
        &self.input_scale
    }
    fn input_scale_mut(&mut self) -> &mut f32 {
        &mut self.input_scale
    }
    fn max_opacity(&self) -> &f32 {
        &self.max_opacity
    }
    fn max_opacity_mut(&mut self) -> &mut f32 {
        &mut self.max_opacity
    }
    fn transpose_reflection(&self) -> &bool {
        &self.transpose_reflection
    }
    fn transpose_reflection_mut(&mut self) -> &mut bool {
        &mut self.transpose_reflection
    }
    fn scale(&self) -> &f32 {
        &self.scale
    }
    fn scale_mut(&mut self) -> &mut f32 {
        &mut self.scale
    }
    fn distortion_factor(&self) -> &f32 {
        &self.distortion_factor
    }
    fn distortion_factor_mut(&mut self) -> &mut f32 {
        &mut self.distortion_factor
    }
    fn vertical_stretch(&self) -> &f32 {
        &self.vertical_stretch
    }
    fn vertical_stretch_mut(&mut self) -> &mut f32 {
        &mut self.vertical_stretch
    }
    fn field_flag_override0(&self) -> &u16 {
        &self.field_flag_override0
    }
    fn field_flag_override0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_override0
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_changed0
    }
}

pub static LENSREFLECTIONCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensReflectionComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LensReflectionComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LensReflectionComponentState, enable),
            },
            FieldInfoData {
                name: "InnerColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(LensReflectionComponentState, inner_color),
            },
            FieldInfoData {
                name: "OuterColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(LensReflectionComponentState, outer_color),
            },
            FieldInfoData {
                name: "MixStart",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LensReflectionComponentState, mix_start),
            },
            FieldInfoData {
                name: "MixStop",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LensReflectionComponentState, mix_stop),
            },
            FieldInfoData {
                name: "InputExponent",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LensReflectionComponentState, input_exponent),
            },
            FieldInfoData {
                name: "LuminanceThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LensReflectionComponentState, luminance_threshold),
            },
            FieldInfoData {
                name: "InputScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LensReflectionComponentState, input_scale),
            },
            FieldInfoData {
                name: "MaxOpacity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LensReflectionComponentState, max_opacity),
            },
            FieldInfoData {
                name: "TransposeReflection",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LensReflectionComponentState, transpose_reflection),
            },
            FieldInfoData {
                name: "Scale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LensReflectionComponentState, scale),
            },
            FieldInfoData {
                name: "DistortionFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LensReflectionComponentState, distortion_factor),
            },
            FieldInfoData {
                name: "VerticalStretch",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LensReflectionComponentState, vertical_stretch),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(LensReflectionComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(LensReflectionComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(LENSREFLECTIONCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LensReflectionComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        LENSREFLECTIONCOMPONENTSTATE_TYPE_INFO
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


pub static LENSREFLECTIONCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensReflectionComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("LensReflectionComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum LensReflectionComponentCount {
    #[default]
    MaxLensReflectionComponentCount = 4,
}

pub static LENSREFLECTIONCOMPONENTCOUNT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensReflectionComponentCount",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(LENSREFLECTIONCOMPONENTCOUNT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LensReflectionComponentCount {
    fn type_info(&self) -> &'static TypeInfo {
        LENSREFLECTIONCOMPONENTCOUNT_TYPE_INFO
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


pub static LENSREFLECTIONCOMPONENTCOUNT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensReflectionComponentCount-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("LensReflectionComponentCount"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    pub heat_ripple_texture: Option<Arc<Mutex<dyn TextureBaseAssetTrait>>>,
    pub field_flag_override0: u32,
    pub field_flag_override1: u8,
    pub field_flag_changed0: u32,
    pub field_flag_changed1: u8,
}

pub trait FilmicEffectsComponentStateTrait: TypeObject {
    fn enable(&self) -> &bool;
    fn enable_mut(&mut self) -> &mut bool;
    fn enable_chromatic_abberation(&self) -> &bool;
    fn enable_chromatic_abberation_mut(&mut self) -> &mut bool;
    fn chromatic_abberation_scale(&self) -> &f32;
    fn chromatic_abberation_scale_mut(&mut self) -> &mut f32;
    fn chromatic_abberation_aspect_ratio(&self) -> &f32;
    fn chromatic_abberation_aspect_ratio_mut(&mut self) -> &mut f32;
    fn enable_vignetting(&self) -> &bool;
    fn enable_vignetting_mut(&mut self) -> &mut bool;
    fn vignetting_falloff(&self) -> &f32;
    fn vignetting_falloff_mut(&mut self) -> &mut f32;
    fn vignetting_luminance_percent(&self) -> &f32;
    fn vignetting_luminance_percent_mut(&mut self) -> &mut f32;
    fn enable_lens_distortion(&self) -> &bool;
    fn enable_lens_distortion_mut(&mut self) -> &mut bool;
    fn lens_distortion_gain(&self) -> &f32;
    fn lens_distortion_gain_mut(&mut self) -> &mut f32;
    fn lens_distortion_cubic_gain(&self) -> &f32;
    fn lens_distortion_cubic_gain_mut(&mut self) -> &mut f32;
    fn lens_distortion_stretch(&self) -> &f32;
    fn lens_distortion_stretch_mut(&mut self) -> &mut f32;
    fn enable_frame_flash(&self) -> &bool;
    fn enable_frame_flash_mut(&mut self) -> &mut bool;
    fn frame_flash_gain(&self) -> &f32;
    fn frame_flash_gain_mut(&mut self) -> &mut f32;
    fn enable_depth_flash(&self) -> &bool;
    fn enable_depth_flash_mut(&mut self) -> &mut bool;
    fn depth_flash_atmos_color(&self) -> &super::core::Vec3;
    fn depth_flash_atmos_color_mut(&mut self) -> &mut super::core::Vec3;
    fn depth_flash_half_distance(&self) -> &f32;
    fn depth_flash_half_distance_mut(&mut self) -> &mut f32;
    fn enable_distance_blur(&self) -> &bool;
    fn enable_distance_blur_mut(&mut self) -> &mut bool;
    fn distance_blur_gain(&self) -> &f32;
    fn distance_blur_gain_mut(&mut self) -> &mut f32;
    fn distance_blur_half_distance(&self) -> &f32;
    fn distance_blur_half_distance_mut(&mut self) -> &mut f32;
    fn enable_edge_blur(&self) -> &bool;
    fn enable_edge_blur_mut(&mut self) -> &mut bool;
    fn edge_blur_gain(&self) -> &f32;
    fn edge_blur_gain_mut(&mut self) -> &mut f32;
    fn edge_blur_depth_target_scale(&self) -> &f32;
    fn edge_blur_depth_target_scale_mut(&mut self) -> &mut f32;
    fn edge_blur_fade_near_depth(&self) -> &f32;
    fn edge_blur_fade_near_depth_mut(&mut self) -> &mut f32;
    fn edge_blur_fade_far_depth(&self) -> &f32;
    fn edge_blur_fade_far_depth_mut(&mut self) -> &mut f32;
    fn edge_blur_matte_dilate_size(&self) -> &f32;
    fn edge_blur_matte_dilate_size_mut(&mut self) -> &mut f32;
    fn edge_blur_matte_blur_kernel_size(&self) -> &f32;
    fn edge_blur_matte_blur_kernel_size_mut(&mut self) -> &mut f32;
    fn enable_heat_ripple(&self) -> &bool;
    fn enable_heat_ripple_mut(&mut self) -> &mut bool;
    fn heat_ripple_gain(&self) -> &f32;
    fn heat_ripple_gain_mut(&mut self) -> &mut f32;
    fn heat_ripple_horizontal_speed(&self) -> &f32;
    fn heat_ripple_horizontal_speed_mut(&mut self) -> &mut f32;
    fn heat_ripple_vertical_speed(&self) -> &f32;
    fn heat_ripple_vertical_speed_mut(&mut self) -> &mut f32;
    fn heat_ripple_noise_scale(&self) -> &f32;
    fn heat_ripple_noise_scale_mut(&mut self) -> &mut f32;
    fn heat_ripple_near_distance(&self) -> &f32;
    fn heat_ripple_near_distance_mut(&mut self) -> &mut f32;
    fn heat_ripple_far_distance(&self) -> &f32;
    fn heat_ripple_far_distance_mut(&mut self) -> &mut f32;
    fn heat_ripple_near_gain(&self) -> &f32;
    fn heat_ripple_near_gain_mut(&mut self) -> &mut f32;
    fn heat_ripple_far_gain(&self) -> &f32;
    fn heat_ripple_far_gain_mut(&mut self) -> &mut f32;
    fn heat_ripple_texture(&self) -> &Option<Arc<Mutex<dyn TextureBaseAssetTrait>>>;
    fn heat_ripple_texture_mut(&mut self) -> &mut Option<Arc<Mutex<dyn TextureBaseAssetTrait>>>;
    fn field_flag_override0(&self) -> &u32;
    fn field_flag_override0_mut(&mut self) -> &mut u32;
    fn field_flag_override1(&self) -> &u8;
    fn field_flag_override1_mut(&mut self) -> &mut u8;
    fn field_flag_changed0(&self) -> &u32;
    fn field_flag_changed0_mut(&mut self) -> &mut u32;
    fn field_flag_changed1(&self) -> &u8;
    fn field_flag_changed1_mut(&mut self) -> &mut u8;
}

impl FilmicEffectsComponentStateTrait for FilmicEffectsComponentState {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn enable_mut(&mut self) -> &mut bool {
        &mut self.enable
    }
    fn enable_chromatic_abberation(&self) -> &bool {
        &self.enable_chromatic_abberation
    }
    fn enable_chromatic_abberation_mut(&mut self) -> &mut bool {
        &mut self.enable_chromatic_abberation
    }
    fn chromatic_abberation_scale(&self) -> &f32 {
        &self.chromatic_abberation_scale
    }
    fn chromatic_abberation_scale_mut(&mut self) -> &mut f32 {
        &mut self.chromatic_abberation_scale
    }
    fn chromatic_abberation_aspect_ratio(&self) -> &f32 {
        &self.chromatic_abberation_aspect_ratio
    }
    fn chromatic_abberation_aspect_ratio_mut(&mut self) -> &mut f32 {
        &mut self.chromatic_abberation_aspect_ratio
    }
    fn enable_vignetting(&self) -> &bool {
        &self.enable_vignetting
    }
    fn enable_vignetting_mut(&mut self) -> &mut bool {
        &mut self.enable_vignetting
    }
    fn vignetting_falloff(&self) -> &f32 {
        &self.vignetting_falloff
    }
    fn vignetting_falloff_mut(&mut self) -> &mut f32 {
        &mut self.vignetting_falloff
    }
    fn vignetting_luminance_percent(&self) -> &f32 {
        &self.vignetting_luminance_percent
    }
    fn vignetting_luminance_percent_mut(&mut self) -> &mut f32 {
        &mut self.vignetting_luminance_percent
    }
    fn enable_lens_distortion(&self) -> &bool {
        &self.enable_lens_distortion
    }
    fn enable_lens_distortion_mut(&mut self) -> &mut bool {
        &mut self.enable_lens_distortion
    }
    fn lens_distortion_gain(&self) -> &f32 {
        &self.lens_distortion_gain
    }
    fn lens_distortion_gain_mut(&mut self) -> &mut f32 {
        &mut self.lens_distortion_gain
    }
    fn lens_distortion_cubic_gain(&self) -> &f32 {
        &self.lens_distortion_cubic_gain
    }
    fn lens_distortion_cubic_gain_mut(&mut self) -> &mut f32 {
        &mut self.lens_distortion_cubic_gain
    }
    fn lens_distortion_stretch(&self) -> &f32 {
        &self.lens_distortion_stretch
    }
    fn lens_distortion_stretch_mut(&mut self) -> &mut f32 {
        &mut self.lens_distortion_stretch
    }
    fn enable_frame_flash(&self) -> &bool {
        &self.enable_frame_flash
    }
    fn enable_frame_flash_mut(&mut self) -> &mut bool {
        &mut self.enable_frame_flash
    }
    fn frame_flash_gain(&self) -> &f32 {
        &self.frame_flash_gain
    }
    fn frame_flash_gain_mut(&mut self) -> &mut f32 {
        &mut self.frame_flash_gain
    }
    fn enable_depth_flash(&self) -> &bool {
        &self.enable_depth_flash
    }
    fn enable_depth_flash_mut(&mut self) -> &mut bool {
        &mut self.enable_depth_flash
    }
    fn depth_flash_atmos_color(&self) -> &super::core::Vec3 {
        &self.depth_flash_atmos_color
    }
    fn depth_flash_atmos_color_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.depth_flash_atmos_color
    }
    fn depth_flash_half_distance(&self) -> &f32 {
        &self.depth_flash_half_distance
    }
    fn depth_flash_half_distance_mut(&mut self) -> &mut f32 {
        &mut self.depth_flash_half_distance
    }
    fn enable_distance_blur(&self) -> &bool {
        &self.enable_distance_blur
    }
    fn enable_distance_blur_mut(&mut self) -> &mut bool {
        &mut self.enable_distance_blur
    }
    fn distance_blur_gain(&self) -> &f32 {
        &self.distance_blur_gain
    }
    fn distance_blur_gain_mut(&mut self) -> &mut f32 {
        &mut self.distance_blur_gain
    }
    fn distance_blur_half_distance(&self) -> &f32 {
        &self.distance_blur_half_distance
    }
    fn distance_blur_half_distance_mut(&mut self) -> &mut f32 {
        &mut self.distance_blur_half_distance
    }
    fn enable_edge_blur(&self) -> &bool {
        &self.enable_edge_blur
    }
    fn enable_edge_blur_mut(&mut self) -> &mut bool {
        &mut self.enable_edge_blur
    }
    fn edge_blur_gain(&self) -> &f32 {
        &self.edge_blur_gain
    }
    fn edge_blur_gain_mut(&mut self) -> &mut f32 {
        &mut self.edge_blur_gain
    }
    fn edge_blur_depth_target_scale(&self) -> &f32 {
        &self.edge_blur_depth_target_scale
    }
    fn edge_blur_depth_target_scale_mut(&mut self) -> &mut f32 {
        &mut self.edge_blur_depth_target_scale
    }
    fn edge_blur_fade_near_depth(&self) -> &f32 {
        &self.edge_blur_fade_near_depth
    }
    fn edge_blur_fade_near_depth_mut(&mut self) -> &mut f32 {
        &mut self.edge_blur_fade_near_depth
    }
    fn edge_blur_fade_far_depth(&self) -> &f32 {
        &self.edge_blur_fade_far_depth
    }
    fn edge_blur_fade_far_depth_mut(&mut self) -> &mut f32 {
        &mut self.edge_blur_fade_far_depth
    }
    fn edge_blur_matte_dilate_size(&self) -> &f32 {
        &self.edge_blur_matte_dilate_size
    }
    fn edge_blur_matte_dilate_size_mut(&mut self) -> &mut f32 {
        &mut self.edge_blur_matte_dilate_size
    }
    fn edge_blur_matte_blur_kernel_size(&self) -> &f32 {
        &self.edge_blur_matte_blur_kernel_size
    }
    fn edge_blur_matte_blur_kernel_size_mut(&mut self) -> &mut f32 {
        &mut self.edge_blur_matte_blur_kernel_size
    }
    fn enable_heat_ripple(&self) -> &bool {
        &self.enable_heat_ripple
    }
    fn enable_heat_ripple_mut(&mut self) -> &mut bool {
        &mut self.enable_heat_ripple
    }
    fn heat_ripple_gain(&self) -> &f32 {
        &self.heat_ripple_gain
    }
    fn heat_ripple_gain_mut(&mut self) -> &mut f32 {
        &mut self.heat_ripple_gain
    }
    fn heat_ripple_horizontal_speed(&self) -> &f32 {
        &self.heat_ripple_horizontal_speed
    }
    fn heat_ripple_horizontal_speed_mut(&mut self) -> &mut f32 {
        &mut self.heat_ripple_horizontal_speed
    }
    fn heat_ripple_vertical_speed(&self) -> &f32 {
        &self.heat_ripple_vertical_speed
    }
    fn heat_ripple_vertical_speed_mut(&mut self) -> &mut f32 {
        &mut self.heat_ripple_vertical_speed
    }
    fn heat_ripple_noise_scale(&self) -> &f32 {
        &self.heat_ripple_noise_scale
    }
    fn heat_ripple_noise_scale_mut(&mut self) -> &mut f32 {
        &mut self.heat_ripple_noise_scale
    }
    fn heat_ripple_near_distance(&self) -> &f32 {
        &self.heat_ripple_near_distance
    }
    fn heat_ripple_near_distance_mut(&mut self) -> &mut f32 {
        &mut self.heat_ripple_near_distance
    }
    fn heat_ripple_far_distance(&self) -> &f32 {
        &self.heat_ripple_far_distance
    }
    fn heat_ripple_far_distance_mut(&mut self) -> &mut f32 {
        &mut self.heat_ripple_far_distance
    }
    fn heat_ripple_near_gain(&self) -> &f32 {
        &self.heat_ripple_near_gain
    }
    fn heat_ripple_near_gain_mut(&mut self) -> &mut f32 {
        &mut self.heat_ripple_near_gain
    }
    fn heat_ripple_far_gain(&self) -> &f32 {
        &self.heat_ripple_far_gain
    }
    fn heat_ripple_far_gain_mut(&mut self) -> &mut f32 {
        &mut self.heat_ripple_far_gain
    }
    fn heat_ripple_texture(&self) -> &Option<Arc<Mutex<dyn TextureBaseAssetTrait>>> {
        &self.heat_ripple_texture
    }
    fn heat_ripple_texture_mut(&mut self) -> &mut Option<Arc<Mutex<dyn TextureBaseAssetTrait>>> {
        &mut self.heat_ripple_texture
    }
    fn field_flag_override0(&self) -> &u32 {
        &self.field_flag_override0
    }
    fn field_flag_override0_mut(&mut self) -> &mut u32 {
        &mut self.field_flag_override0
    }
    fn field_flag_override1(&self) -> &u8 {
        &self.field_flag_override1
    }
    fn field_flag_override1_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_override1
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u32 {
        &mut self.field_flag_changed0
    }
    fn field_flag_changed1(&self) -> &u8 {
        &self.field_flag_changed1
    }
    fn field_flag_changed1_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed1
    }
}

pub static FILMICEFFECTSCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FilmicEffectsComponentState",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FilmicEffectsComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FilmicEffectsComponentState, enable),
            },
            FieldInfoData {
                name: "EnableChromaticAbberation",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FilmicEffectsComponentState, enable_chromatic_abberation),
            },
            FieldInfoData {
                name: "ChromaticAbberationScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FilmicEffectsComponentState, chromatic_abberation_scale),
            },
            FieldInfoData {
                name: "ChromaticAbberationAspectRatio",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FilmicEffectsComponentState, chromatic_abberation_aspect_ratio),
            },
            FieldInfoData {
                name: "EnableVignetting",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FilmicEffectsComponentState, enable_vignetting),
            },
            FieldInfoData {
                name: "VignettingFalloff",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FilmicEffectsComponentState, vignetting_falloff),
            },
            FieldInfoData {
                name: "VignettingLuminancePercent",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FilmicEffectsComponentState, vignetting_luminance_percent),
            },
            FieldInfoData {
                name: "EnableLensDistortion",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FilmicEffectsComponentState, enable_lens_distortion),
            },
            FieldInfoData {
                name: "LensDistortionGain",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FilmicEffectsComponentState, lens_distortion_gain),
            },
            FieldInfoData {
                name: "LensDistortionCubicGain",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FilmicEffectsComponentState, lens_distortion_cubic_gain),
            },
            FieldInfoData {
                name: "LensDistortionStretch",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FilmicEffectsComponentState, lens_distortion_stretch),
            },
            FieldInfoData {
                name: "EnableFrameFlash",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FilmicEffectsComponentState, enable_frame_flash),
            },
            FieldInfoData {
                name: "FrameFlashGain",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FilmicEffectsComponentState, frame_flash_gain),
            },
            FieldInfoData {
                name: "EnableDepthFlash",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FilmicEffectsComponentState, enable_depth_flash),
            },
            FieldInfoData {
                name: "DepthFlashAtmosColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(FilmicEffectsComponentState, depth_flash_atmos_color),
            },
            FieldInfoData {
                name: "DepthFlashHalfDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FilmicEffectsComponentState, depth_flash_half_distance),
            },
            FieldInfoData {
                name: "EnableDistanceBlur",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FilmicEffectsComponentState, enable_distance_blur),
            },
            FieldInfoData {
                name: "DistanceBlurGain",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FilmicEffectsComponentState, distance_blur_gain),
            },
            FieldInfoData {
                name: "DistanceBlurHalfDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FilmicEffectsComponentState, distance_blur_half_distance),
            },
            FieldInfoData {
                name: "EnableEdgeBlur",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FilmicEffectsComponentState, enable_edge_blur),
            },
            FieldInfoData {
                name: "EdgeBlurGain",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FilmicEffectsComponentState, edge_blur_gain),
            },
            FieldInfoData {
                name: "EdgeBlurDepthTargetScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FilmicEffectsComponentState, edge_blur_depth_target_scale),
            },
            FieldInfoData {
                name: "EdgeBlurFadeNearDepth",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FilmicEffectsComponentState, edge_blur_fade_near_depth),
            },
            FieldInfoData {
                name: "EdgeBlurFadeFarDepth",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FilmicEffectsComponentState, edge_blur_fade_far_depth),
            },
            FieldInfoData {
                name: "EdgeBlurMatteDilateSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FilmicEffectsComponentState, edge_blur_matte_dilate_size),
            },
            FieldInfoData {
                name: "EdgeBlurMatteBlurKernelSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FilmicEffectsComponentState, edge_blur_matte_blur_kernel_size),
            },
            FieldInfoData {
                name: "EnableHeatRipple",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FilmicEffectsComponentState, enable_heat_ripple),
            },
            FieldInfoData {
                name: "HeatRippleGain",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FilmicEffectsComponentState, heat_ripple_gain),
            },
            FieldInfoData {
                name: "HeatRippleHorizontalSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FilmicEffectsComponentState, heat_ripple_horizontal_speed),
            },
            FieldInfoData {
                name: "HeatRippleVerticalSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FilmicEffectsComponentState, heat_ripple_vertical_speed),
            },
            FieldInfoData {
                name: "HeatRippleNoiseScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FilmicEffectsComponentState, heat_ripple_noise_scale),
            },
            FieldInfoData {
                name: "HeatRippleNearDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FilmicEffectsComponentState, heat_ripple_near_distance),
            },
            FieldInfoData {
                name: "HeatRippleFarDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FilmicEffectsComponentState, heat_ripple_far_distance),
            },
            FieldInfoData {
                name: "HeatRippleNearGain",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FilmicEffectsComponentState, heat_ripple_near_gain),
            },
            FieldInfoData {
                name: "HeatRippleFarGain",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FilmicEffectsComponentState, heat_ripple_far_gain),
            },
            FieldInfoData {
                name: "HeatRippleTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(FilmicEffectsComponentState, heat_ripple_texture),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FilmicEffectsComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagOverride1",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(FilmicEffectsComponentState, field_flag_override1),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FilmicEffectsComponentState, field_flag_changed0),
            },
            FieldInfoData {
                name: "FieldFlagChanged1",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(FilmicEffectsComponentState, field_flag_changed1),
            },
        ],
    }),
    array_type: Some(FILMICEFFECTSCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FilmicEffectsComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        FILMICEFFECTSCOMPONENTSTATE_TYPE_INFO
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


pub static FILMICEFFECTSCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FilmicEffectsComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("FilmicEffectsComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DebugComponentState {
    pub enable: bool,
    pub fullscreen: bool,
    pub debug_texture: Option<Arc<Mutex<dyn TextureBaseAssetTrait>>>,
    pub field_flag_override0: u8,
    pub field_flag_changed0: u8,
}

pub trait DebugComponentStateTrait: TypeObject {
    fn enable(&self) -> &bool;
    fn enable_mut(&mut self) -> &mut bool;
    fn fullscreen(&self) -> &bool;
    fn fullscreen_mut(&mut self) -> &mut bool;
    fn debug_texture(&self) -> &Option<Arc<Mutex<dyn TextureBaseAssetTrait>>>;
    fn debug_texture_mut(&mut self) -> &mut Option<Arc<Mutex<dyn TextureBaseAssetTrait>>>;
    fn field_flag_override0(&self) -> &u8;
    fn field_flag_override0_mut(&mut self) -> &mut u8;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl DebugComponentStateTrait for DebugComponentState {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn enable_mut(&mut self) -> &mut bool {
        &mut self.enable
    }
    fn fullscreen(&self) -> &bool {
        &self.fullscreen
    }
    fn fullscreen_mut(&mut self) -> &mut bool {
        &mut self.fullscreen
    }
    fn debug_texture(&self) -> &Option<Arc<Mutex<dyn TextureBaseAssetTrait>>> {
        &self.debug_texture
    }
    fn debug_texture_mut(&mut self) -> &mut Option<Arc<Mutex<dyn TextureBaseAssetTrait>>> {
        &mut self.debug_texture
    }
    fn field_flag_override0(&self) -> &u8 {
        &self.field_flag_override0
    }
    fn field_flag_override0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_override0
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static DEBUGCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebugComponentState",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DebugComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebugComponentState, enable),
            },
            FieldInfoData {
                name: "Fullscreen",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebugComponentState, fullscreen),
            },
            FieldInfoData {
                name: "DebugTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(DebugComponentState, debug_texture),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(DebugComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(DebugComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(DEBUGCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DebugComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        DEBUGCOMPONENTSTATE_TYPE_INFO
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


pub static DEBUGCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebugComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("DebugComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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

pub trait LensScopeComponentStateTrait: TypeObject {
    fn enable(&self) -> &bool;
    fn enable_mut(&mut self) -> &mut bool;
    fn blur_scale(&self) -> &f32;
    fn blur_scale_mut(&mut self) -> &mut f32;
    fn blur_center(&self) -> &super::core::Vec2;
    fn blur_center_mut(&mut self) -> &mut super::core::Vec2;
    fn chromatic_aberration_color1(&self) -> &super::core::Vec3;
    fn chromatic_aberration_color1_mut(&mut self) -> &mut super::core::Vec3;
    fn chromatic_aberration_color2(&self) -> &super::core::Vec3;
    fn chromatic_aberration_color2_mut(&mut self) -> &mut super::core::Vec3;
    fn chromatic_aberration_strengths(&self) -> &super::core::Vec2;
    fn chromatic_aberration_strengths_mut(&mut self) -> &mut super::core::Vec2;
    fn chromatic_aberration_displacement1(&self) -> &super::core::Vec2;
    fn chromatic_aberration_displacement1_mut(&mut self) -> &mut super::core::Vec2;
    fn chromatic_aberration_displacement2(&self) -> &super::core::Vec2;
    fn chromatic_aberration_displacement2_mut(&mut self) -> &mut super::core::Vec2;
    fn radial_blend_distance_coefficients(&self) -> &super::core::Vec2;
    fn radial_blend_distance_coefficients_mut(&mut self) -> &mut super::core::Vec2;
    fn field_flag_override0(&self) -> &u16;
    fn field_flag_override0_mut(&mut self) -> &mut u16;
    fn field_flag_changed0(&self) -> &u16;
    fn field_flag_changed0_mut(&mut self) -> &mut u16;
}

impl LensScopeComponentStateTrait for LensScopeComponentState {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn enable_mut(&mut self) -> &mut bool {
        &mut self.enable
    }
    fn blur_scale(&self) -> &f32 {
        &self.blur_scale
    }
    fn blur_scale_mut(&mut self) -> &mut f32 {
        &mut self.blur_scale
    }
    fn blur_center(&self) -> &super::core::Vec2 {
        &self.blur_center
    }
    fn blur_center_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.blur_center
    }
    fn chromatic_aberration_color1(&self) -> &super::core::Vec3 {
        &self.chromatic_aberration_color1
    }
    fn chromatic_aberration_color1_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.chromatic_aberration_color1
    }
    fn chromatic_aberration_color2(&self) -> &super::core::Vec3 {
        &self.chromatic_aberration_color2
    }
    fn chromatic_aberration_color2_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.chromatic_aberration_color2
    }
    fn chromatic_aberration_strengths(&self) -> &super::core::Vec2 {
        &self.chromatic_aberration_strengths
    }
    fn chromatic_aberration_strengths_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.chromatic_aberration_strengths
    }
    fn chromatic_aberration_displacement1(&self) -> &super::core::Vec2 {
        &self.chromatic_aberration_displacement1
    }
    fn chromatic_aberration_displacement1_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.chromatic_aberration_displacement1
    }
    fn chromatic_aberration_displacement2(&self) -> &super::core::Vec2 {
        &self.chromatic_aberration_displacement2
    }
    fn chromatic_aberration_displacement2_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.chromatic_aberration_displacement2
    }
    fn radial_blend_distance_coefficients(&self) -> &super::core::Vec2 {
        &self.radial_blend_distance_coefficients
    }
    fn radial_blend_distance_coefficients_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.radial_blend_distance_coefficients
    }
    fn field_flag_override0(&self) -> &u16 {
        &self.field_flag_override0
    }
    fn field_flag_override0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_override0
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_changed0
    }
}

pub static LENSSCOPECOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensScopeComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LensScopeComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LensScopeComponentState, enable),
            },
            FieldInfoData {
                name: "BlurScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LensScopeComponentState, blur_scale),
            },
            FieldInfoData {
                name: "BlurCenter",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(LensScopeComponentState, blur_center),
            },
            FieldInfoData {
                name: "ChromaticAberrationColor1",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(LensScopeComponentState, chromatic_aberration_color1),
            },
            FieldInfoData {
                name: "ChromaticAberrationColor2",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(LensScopeComponentState, chromatic_aberration_color2),
            },
            FieldInfoData {
                name: "ChromaticAberrationStrengths",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(LensScopeComponentState, chromatic_aberration_strengths),
            },
            FieldInfoData {
                name: "ChromaticAberrationDisplacement1",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(LensScopeComponentState, chromatic_aberration_displacement1),
            },
            FieldInfoData {
                name: "ChromaticAberrationDisplacement2",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(LensScopeComponentState, chromatic_aberration_displacement2),
            },
            FieldInfoData {
                name: "RadialBlendDistanceCoefficients",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(LensScopeComponentState, radial_blend_distance_coefficients),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(LensScopeComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(LensScopeComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(LENSSCOPECOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LensScopeComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        LENSSCOPECOMPONENTSTATE_TYPE_INFO
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


pub static LENSSCOPECOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensScopeComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("LensScopeComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FilmGrainComponentState {
    pub enable: bool,
    pub texture_scale: super::core::Vec2,
    pub color_scale: super::core::Vec3,
    pub linear_filtering_enable: bool,
    pub random_enable: bool,
    pub texture: Option<Arc<Mutex<dyn TextureBaseAssetTrait>>>,
    pub grain_grey_fraction: f32,
    pub grain_luminance_control_enable: bool,
    pub grain_shadow_threshold: f32,
    pub grain_highlight_threshold: f32,
    pub grain_shadow_intensity: f32,
    pub grain_highlight_intensity: f32,
    pub field_flag_override0: u16,
    pub field_flag_changed0: u16,
}

pub trait FilmGrainComponentStateTrait: TypeObject {
    fn enable(&self) -> &bool;
    fn enable_mut(&mut self) -> &mut bool;
    fn texture_scale(&self) -> &super::core::Vec2;
    fn texture_scale_mut(&mut self) -> &mut super::core::Vec2;
    fn color_scale(&self) -> &super::core::Vec3;
    fn color_scale_mut(&mut self) -> &mut super::core::Vec3;
    fn linear_filtering_enable(&self) -> &bool;
    fn linear_filtering_enable_mut(&mut self) -> &mut bool;
    fn random_enable(&self) -> &bool;
    fn random_enable_mut(&mut self) -> &mut bool;
    fn texture(&self) -> &Option<Arc<Mutex<dyn TextureBaseAssetTrait>>>;
    fn texture_mut(&mut self) -> &mut Option<Arc<Mutex<dyn TextureBaseAssetTrait>>>;
    fn grain_grey_fraction(&self) -> &f32;
    fn grain_grey_fraction_mut(&mut self) -> &mut f32;
    fn grain_luminance_control_enable(&self) -> &bool;
    fn grain_luminance_control_enable_mut(&mut self) -> &mut bool;
    fn grain_shadow_threshold(&self) -> &f32;
    fn grain_shadow_threshold_mut(&mut self) -> &mut f32;
    fn grain_highlight_threshold(&self) -> &f32;
    fn grain_highlight_threshold_mut(&mut self) -> &mut f32;
    fn grain_shadow_intensity(&self) -> &f32;
    fn grain_shadow_intensity_mut(&mut self) -> &mut f32;
    fn grain_highlight_intensity(&self) -> &f32;
    fn grain_highlight_intensity_mut(&mut self) -> &mut f32;
    fn field_flag_override0(&self) -> &u16;
    fn field_flag_override0_mut(&mut self) -> &mut u16;
    fn field_flag_changed0(&self) -> &u16;
    fn field_flag_changed0_mut(&mut self) -> &mut u16;
}

impl FilmGrainComponentStateTrait for FilmGrainComponentState {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn enable_mut(&mut self) -> &mut bool {
        &mut self.enable
    }
    fn texture_scale(&self) -> &super::core::Vec2 {
        &self.texture_scale
    }
    fn texture_scale_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.texture_scale
    }
    fn color_scale(&self) -> &super::core::Vec3 {
        &self.color_scale
    }
    fn color_scale_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.color_scale
    }
    fn linear_filtering_enable(&self) -> &bool {
        &self.linear_filtering_enable
    }
    fn linear_filtering_enable_mut(&mut self) -> &mut bool {
        &mut self.linear_filtering_enable
    }
    fn random_enable(&self) -> &bool {
        &self.random_enable
    }
    fn random_enable_mut(&mut self) -> &mut bool {
        &mut self.random_enable
    }
    fn texture(&self) -> &Option<Arc<Mutex<dyn TextureBaseAssetTrait>>> {
        &self.texture
    }
    fn texture_mut(&mut self) -> &mut Option<Arc<Mutex<dyn TextureBaseAssetTrait>>> {
        &mut self.texture
    }
    fn grain_grey_fraction(&self) -> &f32 {
        &self.grain_grey_fraction
    }
    fn grain_grey_fraction_mut(&mut self) -> &mut f32 {
        &mut self.grain_grey_fraction
    }
    fn grain_luminance_control_enable(&self) -> &bool {
        &self.grain_luminance_control_enable
    }
    fn grain_luminance_control_enable_mut(&mut self) -> &mut bool {
        &mut self.grain_luminance_control_enable
    }
    fn grain_shadow_threshold(&self) -> &f32 {
        &self.grain_shadow_threshold
    }
    fn grain_shadow_threshold_mut(&mut self) -> &mut f32 {
        &mut self.grain_shadow_threshold
    }
    fn grain_highlight_threshold(&self) -> &f32 {
        &self.grain_highlight_threshold
    }
    fn grain_highlight_threshold_mut(&mut self) -> &mut f32 {
        &mut self.grain_highlight_threshold
    }
    fn grain_shadow_intensity(&self) -> &f32 {
        &self.grain_shadow_intensity
    }
    fn grain_shadow_intensity_mut(&mut self) -> &mut f32 {
        &mut self.grain_shadow_intensity
    }
    fn grain_highlight_intensity(&self) -> &f32 {
        &self.grain_highlight_intensity
    }
    fn grain_highlight_intensity_mut(&mut self) -> &mut f32 {
        &mut self.grain_highlight_intensity
    }
    fn field_flag_override0(&self) -> &u16 {
        &self.field_flag_override0
    }
    fn field_flag_override0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_override0
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_changed0
    }
}

pub static FILMGRAINCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FilmGrainComponentState",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FilmGrainComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FilmGrainComponentState, enable),
            },
            FieldInfoData {
                name: "TextureScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(FilmGrainComponentState, texture_scale),
            },
            FieldInfoData {
                name: "ColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(FilmGrainComponentState, color_scale),
            },
            FieldInfoData {
                name: "LinearFilteringEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FilmGrainComponentState, linear_filtering_enable),
            },
            FieldInfoData {
                name: "RandomEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FilmGrainComponentState, random_enable),
            },
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(FilmGrainComponentState, texture),
            },
            FieldInfoData {
                name: "GrainGreyFraction",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FilmGrainComponentState, grain_grey_fraction),
            },
            FieldInfoData {
                name: "GrainLuminanceControlEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FilmGrainComponentState, grain_luminance_control_enable),
            },
            FieldInfoData {
                name: "GrainShadowThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FilmGrainComponentState, grain_shadow_threshold),
            },
            FieldInfoData {
                name: "GrainHighlightThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FilmGrainComponentState, grain_highlight_threshold),
            },
            FieldInfoData {
                name: "GrainShadowIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FilmGrainComponentState, grain_shadow_intensity),
            },
            FieldInfoData {
                name: "GrainHighlightIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FilmGrainComponentState, grain_highlight_intensity),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(FilmGrainComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(FilmGrainComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(FILMGRAINCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FilmGrainComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        FILMGRAINCOMPONENTSTATE_TYPE_INFO
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


pub static FILMGRAINCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FilmGrainComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("FilmGrainComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VignetteComponentState {
    pub enable: bool,
    pub scale: super::core::Vec2,
    pub exponent: f32,
    pub color: super::core::Vec3,
    pub opacity: f32,
    pub field_flag_override0: u8,
    pub field_flag_changed0: u8,
}

pub trait VignetteComponentStateTrait: TypeObject {
    fn enable(&self) -> &bool;
    fn enable_mut(&mut self) -> &mut bool;
    fn scale(&self) -> &super::core::Vec2;
    fn scale_mut(&mut self) -> &mut super::core::Vec2;
    fn exponent(&self) -> &f32;
    fn exponent_mut(&mut self) -> &mut f32;
    fn color(&self) -> &super::core::Vec3;
    fn color_mut(&mut self) -> &mut super::core::Vec3;
    fn opacity(&self) -> &f32;
    fn opacity_mut(&mut self) -> &mut f32;
    fn field_flag_override0(&self) -> &u8;
    fn field_flag_override0_mut(&mut self) -> &mut u8;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl VignetteComponentStateTrait for VignetteComponentState {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn enable_mut(&mut self) -> &mut bool {
        &mut self.enable
    }
    fn scale(&self) -> &super::core::Vec2 {
        &self.scale
    }
    fn scale_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.scale
    }
    fn exponent(&self) -> &f32 {
        &self.exponent
    }
    fn exponent_mut(&mut self) -> &mut f32 {
        &mut self.exponent
    }
    fn color(&self) -> &super::core::Vec3 {
        &self.color
    }
    fn color_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.color
    }
    fn opacity(&self) -> &f32 {
        &self.opacity
    }
    fn opacity_mut(&mut self) -> &mut f32 {
        &mut self.opacity
    }
    fn field_flag_override0(&self) -> &u8 {
        &self.field_flag_override0
    }
    fn field_flag_override0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_override0
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static VIGNETTECOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VignetteComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VignetteComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VignetteComponentState, enable),
            },
            FieldInfoData {
                name: "Scale",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(VignetteComponentState, scale),
            },
            FieldInfoData {
                name: "Exponent",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VignetteComponentState, exponent),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(VignetteComponentState, color),
            },
            FieldInfoData {
                name: "Opacity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VignetteComponentState, opacity),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(VignetteComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(VignetteComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(VIGNETTECOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for VignetteComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        VIGNETTECOMPONENTSTATE_TYPE_INFO
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


pub static VIGNETTECOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VignetteComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("VignetteComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    pub sprite_dof_bokeh_texture: Option<Arc<Mutex<dyn TextureBaseAssetTrait>>>,
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
    pub masked_blur_texture: Option<Arc<Mutex<dyn TextureBaseAssetTrait>>>,
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

pub trait DofComponentStateTrait: TypeObject {
    fn enable(&self) -> &bool;
    fn enable_mut(&mut self) -> &mut bool;
    fn physical_camera_tweak_enable(&self) -> &bool;
    fn physical_camera_tweak_enable_mut(&mut self) -> &mut bool;
    fn pbc_background_blur_add(&self) -> &f32;
    fn pbc_background_blur_add_mut(&mut self) -> &mut f32;
    fn pbc_foreground_blur_add(&self) -> &f32;
    fn pbc_foreground_blur_add_mut(&mut self) -> &mut f32;
    fn pbc_focus_range_add(&self) -> &f32;
    fn pbc_focus_range_add_mut(&mut self) -> &mut f32;
    fn dof_source(&self) -> &DofSource;
    fn dof_source_mut(&mut self) -> &mut DofSource;
    fn debug_draw_focus_plane(&self) -> &bool;
    fn debug_draw_focus_plane_mut(&mut self) -> &mut bool;
    fn focus_dof_max_blur(&self) -> &f32;
    fn focus_dof_max_blur_mut(&mut self) -> &mut f32;
    fn blur_factor(&self) -> &f32;
    fn blur_factor_mut(&mut self) -> &mut f32;
    fn blur_add(&self) -> &f32;
    fn blur_add_mut(&mut self) -> &mut f32;
    fn focus_distance(&self) -> &f32;
    fn focus_distance_mut(&mut self) -> &mut f32;
    fn radial_blur_enable_common_dof(&self) -> &bool;
    fn radial_blur_enable_common_dof_mut(&mut self) -> &mut bool;
    fn radial_blur_amount_common_dof(&self) -> &f32;
    fn radial_blur_amount_common_dof_mut(&mut self) -> &mut f32;
    fn radial_blur_start_radius_common_dof(&self) -> &f32;
    fn radial_blur_start_radius_common_dof_mut(&mut self) -> &mut f32;
    fn radial_blur_transition_width_common_dof(&self) -> &f32;
    fn radial_blur_transition_width_common_dof_mut(&mut self) -> &mut f32;
    fn radial_blur_tilt_common_dof(&self) -> &f32;
    fn radial_blur_tilt_common_dof_mut(&mut self) -> &mut f32;
    fn radial_blur_horizontal_scale_common_dof(&self) -> &f32;
    fn radial_blur_horizontal_scale_common_dof_mut(&mut self) -> &mut f32;
    fn radial_blur_aspect_ratio_blend(&self) -> &f32;
    fn radial_blur_aspect_ratio_blend_mut(&mut self) -> &mut f32;
    fn radial_blur_position_common_dof(&self) -> &super::core::Vec2;
    fn radial_blur_position_common_dof_mut(&mut self) -> &mut super::core::Vec2;
    fn simple_dof_blur_filter(&self) -> &BlurFilter;
    fn simple_dof_blur_filter_mut(&mut self) -> &mut BlurFilter;
    fn simple_dof_standard_deviation(&self) -> &f32;
    fn simple_dof_standard_deviation_mut(&mut self) -> &mut f32;
    fn sprite_dof_bokeh_texture(&self) -> &Option<Arc<Mutex<dyn TextureBaseAssetTrait>>>;
    fn sprite_dof_bokeh_texture_mut(&mut self) -> &mut Option<Arc<Mutex<dyn TextureBaseAssetTrait>>>;
    fn focus_dof_near_start(&self) -> &f32;
    fn focus_dof_near_start_mut(&mut self) -> &mut f32;
    fn focus_dof_near_end(&self) -> &f32;
    fn focus_dof_near_end_mut(&mut self) -> &mut f32;
    fn focus_dof_far_start(&self) -> &f32;
    fn focus_dof_far_start_mut(&mut self) -> &mut f32;
    fn focus_dof_far_end(&self) -> &f32;
    fn focus_dof_far_end_mut(&mut self) -> &mut f32;
    fn pbr_focus_length_dof(&self) -> &f32;
    fn pbr_focus_length_dof_mut(&mut self) -> &mut f32;
    fn pbr_film_width_dof(&self) -> &f32;
    fn pbr_film_width_dof_mut(&mut self) -> &mut f32;
    fn pbr_f_stop_dof(&self) -> &f32;
    fn pbr_f_stop_dof_mut(&mut self) -> &mut f32;
    fn optical_vignetting_enable(&self) -> &bool;
    fn optical_vignetting_enable_mut(&mut self) -> &mut bool;
    fn optical_vignetting_amount(&self) -> &f32;
    fn optical_vignetting_amount_mut(&mut self) -> &mut f32;
    fn optical_vignetting_aspect_ratio(&self) -> &f32;
    fn optical_vignetting_aspect_ratio_mut(&mut self) -> &mut f32;
    fn optical_vignetting_anamorphic_squeeze(&self) -> &f32;
    fn optical_vignetting_anamorphic_squeeze_mut(&mut self) -> &mut f32;
    fn optical_vignetting_size_compensation(&self) -> &f32;
    fn optical_vignetting_size_compensation_mut(&mut self) -> &mut f32;
    fn optical_vignetting_operation(&self) -> &VignettingOperation;
    fn optical_vignetting_operation_mut(&mut self) -> &mut VignettingOperation;
    fn r_g_b_bokeh_texture_enable(&self) -> &bool;
    fn r_g_b_bokeh_texture_enable_mut(&mut self) -> &mut bool;
    fn bokeh_chromatic_aberration_enable(&self) -> &bool;
    fn bokeh_chromatic_aberration_enable_mut(&mut self) -> &mut bool;
    fn bokeh_chromatic_aberration_scale(&self) -> &f32;
    fn bokeh_chromatic_aberration_scale_mut(&mut self) -> &mut f32;
    fn bokeh_chromatic_aberration_radius(&self) -> &f32;
    fn bokeh_chromatic_aberration_radius_mut(&mut self) -> &mut f32;
    fn bokeh_chromatic_aberration_width(&self) -> &f32;
    fn bokeh_chromatic_aberration_width_mut(&mut self) -> &mut f32;
    fn bokeh_chromatic_aberration_radius_threshold(&self) -> &f32;
    fn bokeh_chromatic_aberration_radius_threshold_mut(&mut self) -> &mut f32;
    fn bokeh_chromatic_aberration_radius_threshold_width(&self) -> &f32;
    fn bokeh_chromatic_aberration_radius_threshold_width_mut(&mut self) -> &mut f32;
    fn bokeh_chromatic_aberration_energy_threshold(&self) -> &f32;
    fn bokeh_chromatic_aberration_energy_threshold_mut(&mut self) -> &mut f32;
    fn bokeh_chromatic_aberration_fg_color(&self) -> &super::core::Vec3;
    fn bokeh_chromatic_aberration_fg_color_mut(&mut self) -> &mut super::core::Vec3;
    fn bokeh_chromatic_aberration_bg_color(&self) -> &super::core::Vec3;
    fn bokeh_chromatic_aberration_bg_color_mut(&mut self) -> &mut super::core::Vec3;
    fn ironsights_dof_active(&self) -> &bool;
    fn ironsights_dof_active_mut(&mut self) -> &mut bool;
    fn ironsights_dof_extra_blur(&self) -> &bool;
    fn ironsights_dof_extra_blur_mut(&mut self) -> &mut bool;
    fn hip_to_ironsights_fade(&self) -> &f32;
    fn hip_to_ironsights_fade_mut(&mut self) -> &mut f32;
    fn ironsights_dof_start_fade(&self) -> &f32;
    fn ironsights_dof_start_fade_mut(&mut self) -> &mut f32;
    fn ironsights_focal_distance(&self) -> &f32;
    fn ironsights_focal_distance_mut(&mut self) -> &mut f32;
    fn ironsights_dof_circle_blur(&self) -> &bool;
    fn ironsights_dof_circle_blur_mut(&mut self) -> &mut bool;
    fn ironsights_dof_circle_distance(&self) -> &f32;
    fn ironsights_dof_circle_distance_mut(&mut self) -> &mut f32;
    fn ironsights_dof_circle_fade_distance(&self) -> &f32;
    fn ironsights_dof_circle_fade_distance_mut(&mut self) -> &mut f32;
    fn masked_blur_enabled(&self) -> &bool;
    fn masked_blur_enabled_mut(&mut self) -> &mut bool;
    fn masked_blur_amount(&self) -> &f32;
    fn masked_blur_amount_mut(&mut self) -> &mut f32;
    fn masked_blur_texture(&self) -> &Option<Arc<Mutex<dyn TextureBaseAssetTrait>>>;
    fn masked_blur_texture_mut(&mut self) -> &mut Option<Arc<Mutex<dyn TextureBaseAssetTrait>>>;
    fn circular_dof_anti_band_artifact(&self) -> &bool;
    fn circular_dof_anti_band_artifact_mut(&mut self) -> &mut bool;
    fn use_camera_settings(&self) -> &bool;
    fn use_camera_settings_mut(&mut self) -> &mut bool;
    fn simple_dof_max_blur(&self) -> &f32;
    fn simple_dof_max_blur_mut(&mut self) -> &mut f32;
    fn simple_dof_near_start(&self) -> &f32;
    fn simple_dof_near_start_mut(&mut self) -> &mut f32;
    fn simple_dof_near_end(&self) -> &f32;
    fn simple_dof_near_end_mut(&mut self) -> &mut f32;
    fn simple_dof_far_start(&self) -> &f32;
    fn simple_dof_far_start_mut(&mut self) -> &mut f32;
    fn simple_dof_far_end(&self) -> &f32;
    fn simple_dof_far_end_mut(&mut self) -> &mut f32;
    fn sprite_dof_near_start(&self) -> &f32;
    fn sprite_dof_near_start_mut(&mut self) -> &mut f32;
    fn sprite_dof_near_end(&self) -> &f32;
    fn sprite_dof_near_end_mut(&mut self) -> &mut f32;
    fn sprite_dof_far_start(&self) -> &f32;
    fn sprite_dof_far_start_mut(&mut self) -> &mut f32;
    fn sprite_dof_far_end(&self) -> &f32;
    fn sprite_dof_far_end_mut(&mut self) -> &mut f32;
    fn sprite_dof_max_blur(&self) -> &f32;
    fn sprite_dof_max_blur_mut(&mut self) -> &mut f32;
    fn anisotropy(&self) -> &f32;
    fn anisotropy_mut(&mut self) -> &mut f32;
    fn full_screen_blur_add_common_dof(&self) -> &f32;
    fn full_screen_blur_add_common_dof_mut(&mut self) -> &mut f32;
    fn field_flag_override0(&self) -> &u32;
    fn field_flag_override0_mut(&mut self) -> &mut u32;
    fn field_flag_override1(&self) -> &u32;
    fn field_flag_override1_mut(&mut self) -> &mut u32;
    fn field_flag_override2(&self) -> &u8;
    fn field_flag_override2_mut(&mut self) -> &mut u8;
    fn field_flag_changed0(&self) -> &u32;
    fn field_flag_changed0_mut(&mut self) -> &mut u32;
    fn field_flag_changed1(&self) -> &u32;
    fn field_flag_changed1_mut(&mut self) -> &mut u32;
    fn field_flag_changed2(&self) -> &u16;
    fn field_flag_changed2_mut(&mut self) -> &mut u16;
}

impl DofComponentStateTrait for DofComponentState {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn enable_mut(&mut self) -> &mut bool {
        &mut self.enable
    }
    fn physical_camera_tweak_enable(&self) -> &bool {
        &self.physical_camera_tweak_enable
    }
    fn physical_camera_tweak_enable_mut(&mut self) -> &mut bool {
        &mut self.physical_camera_tweak_enable
    }
    fn pbc_background_blur_add(&self) -> &f32 {
        &self.pbc_background_blur_add
    }
    fn pbc_background_blur_add_mut(&mut self) -> &mut f32 {
        &mut self.pbc_background_blur_add
    }
    fn pbc_foreground_blur_add(&self) -> &f32 {
        &self.pbc_foreground_blur_add
    }
    fn pbc_foreground_blur_add_mut(&mut self) -> &mut f32 {
        &mut self.pbc_foreground_blur_add
    }
    fn pbc_focus_range_add(&self) -> &f32 {
        &self.pbc_focus_range_add
    }
    fn pbc_focus_range_add_mut(&mut self) -> &mut f32 {
        &mut self.pbc_focus_range_add
    }
    fn dof_source(&self) -> &DofSource {
        &self.dof_source
    }
    fn dof_source_mut(&mut self) -> &mut DofSource {
        &mut self.dof_source
    }
    fn debug_draw_focus_plane(&self) -> &bool {
        &self.debug_draw_focus_plane
    }
    fn debug_draw_focus_plane_mut(&mut self) -> &mut bool {
        &mut self.debug_draw_focus_plane
    }
    fn focus_dof_max_blur(&self) -> &f32 {
        &self.focus_dof_max_blur
    }
    fn focus_dof_max_blur_mut(&mut self) -> &mut f32 {
        &mut self.focus_dof_max_blur
    }
    fn blur_factor(&self) -> &f32 {
        &self.blur_factor
    }
    fn blur_factor_mut(&mut self) -> &mut f32 {
        &mut self.blur_factor
    }
    fn blur_add(&self) -> &f32 {
        &self.blur_add
    }
    fn blur_add_mut(&mut self) -> &mut f32 {
        &mut self.blur_add
    }
    fn focus_distance(&self) -> &f32 {
        &self.focus_distance
    }
    fn focus_distance_mut(&mut self) -> &mut f32 {
        &mut self.focus_distance
    }
    fn radial_blur_enable_common_dof(&self) -> &bool {
        &self.radial_blur_enable_common_dof
    }
    fn radial_blur_enable_common_dof_mut(&mut self) -> &mut bool {
        &mut self.radial_blur_enable_common_dof
    }
    fn radial_blur_amount_common_dof(&self) -> &f32 {
        &self.radial_blur_amount_common_dof
    }
    fn radial_blur_amount_common_dof_mut(&mut self) -> &mut f32 {
        &mut self.radial_blur_amount_common_dof
    }
    fn radial_blur_start_radius_common_dof(&self) -> &f32 {
        &self.radial_blur_start_radius_common_dof
    }
    fn radial_blur_start_radius_common_dof_mut(&mut self) -> &mut f32 {
        &mut self.radial_blur_start_radius_common_dof
    }
    fn radial_blur_transition_width_common_dof(&self) -> &f32 {
        &self.radial_blur_transition_width_common_dof
    }
    fn radial_blur_transition_width_common_dof_mut(&mut self) -> &mut f32 {
        &mut self.radial_blur_transition_width_common_dof
    }
    fn radial_blur_tilt_common_dof(&self) -> &f32 {
        &self.radial_blur_tilt_common_dof
    }
    fn radial_blur_tilt_common_dof_mut(&mut self) -> &mut f32 {
        &mut self.radial_blur_tilt_common_dof
    }
    fn radial_blur_horizontal_scale_common_dof(&self) -> &f32 {
        &self.radial_blur_horizontal_scale_common_dof
    }
    fn radial_blur_horizontal_scale_common_dof_mut(&mut self) -> &mut f32 {
        &mut self.radial_blur_horizontal_scale_common_dof
    }
    fn radial_blur_aspect_ratio_blend(&self) -> &f32 {
        &self.radial_blur_aspect_ratio_blend
    }
    fn radial_blur_aspect_ratio_blend_mut(&mut self) -> &mut f32 {
        &mut self.radial_blur_aspect_ratio_blend
    }
    fn radial_blur_position_common_dof(&self) -> &super::core::Vec2 {
        &self.radial_blur_position_common_dof
    }
    fn radial_blur_position_common_dof_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.radial_blur_position_common_dof
    }
    fn simple_dof_blur_filter(&self) -> &BlurFilter {
        &self.simple_dof_blur_filter
    }
    fn simple_dof_blur_filter_mut(&mut self) -> &mut BlurFilter {
        &mut self.simple_dof_blur_filter
    }
    fn simple_dof_standard_deviation(&self) -> &f32 {
        &self.simple_dof_standard_deviation
    }
    fn simple_dof_standard_deviation_mut(&mut self) -> &mut f32 {
        &mut self.simple_dof_standard_deviation
    }
    fn sprite_dof_bokeh_texture(&self) -> &Option<Arc<Mutex<dyn TextureBaseAssetTrait>>> {
        &self.sprite_dof_bokeh_texture
    }
    fn sprite_dof_bokeh_texture_mut(&mut self) -> &mut Option<Arc<Mutex<dyn TextureBaseAssetTrait>>> {
        &mut self.sprite_dof_bokeh_texture
    }
    fn focus_dof_near_start(&self) -> &f32 {
        &self.focus_dof_near_start
    }
    fn focus_dof_near_start_mut(&mut self) -> &mut f32 {
        &mut self.focus_dof_near_start
    }
    fn focus_dof_near_end(&self) -> &f32 {
        &self.focus_dof_near_end
    }
    fn focus_dof_near_end_mut(&mut self) -> &mut f32 {
        &mut self.focus_dof_near_end
    }
    fn focus_dof_far_start(&self) -> &f32 {
        &self.focus_dof_far_start
    }
    fn focus_dof_far_start_mut(&mut self) -> &mut f32 {
        &mut self.focus_dof_far_start
    }
    fn focus_dof_far_end(&self) -> &f32 {
        &self.focus_dof_far_end
    }
    fn focus_dof_far_end_mut(&mut self) -> &mut f32 {
        &mut self.focus_dof_far_end
    }
    fn pbr_focus_length_dof(&self) -> &f32 {
        &self.pbr_focus_length_dof
    }
    fn pbr_focus_length_dof_mut(&mut self) -> &mut f32 {
        &mut self.pbr_focus_length_dof
    }
    fn pbr_film_width_dof(&self) -> &f32 {
        &self.pbr_film_width_dof
    }
    fn pbr_film_width_dof_mut(&mut self) -> &mut f32 {
        &mut self.pbr_film_width_dof
    }
    fn pbr_f_stop_dof(&self) -> &f32 {
        &self.pbr_f_stop_dof
    }
    fn pbr_f_stop_dof_mut(&mut self) -> &mut f32 {
        &mut self.pbr_f_stop_dof
    }
    fn optical_vignetting_enable(&self) -> &bool {
        &self.optical_vignetting_enable
    }
    fn optical_vignetting_enable_mut(&mut self) -> &mut bool {
        &mut self.optical_vignetting_enable
    }
    fn optical_vignetting_amount(&self) -> &f32 {
        &self.optical_vignetting_amount
    }
    fn optical_vignetting_amount_mut(&mut self) -> &mut f32 {
        &mut self.optical_vignetting_amount
    }
    fn optical_vignetting_aspect_ratio(&self) -> &f32 {
        &self.optical_vignetting_aspect_ratio
    }
    fn optical_vignetting_aspect_ratio_mut(&mut self) -> &mut f32 {
        &mut self.optical_vignetting_aspect_ratio
    }
    fn optical_vignetting_anamorphic_squeeze(&self) -> &f32 {
        &self.optical_vignetting_anamorphic_squeeze
    }
    fn optical_vignetting_anamorphic_squeeze_mut(&mut self) -> &mut f32 {
        &mut self.optical_vignetting_anamorphic_squeeze
    }
    fn optical_vignetting_size_compensation(&self) -> &f32 {
        &self.optical_vignetting_size_compensation
    }
    fn optical_vignetting_size_compensation_mut(&mut self) -> &mut f32 {
        &mut self.optical_vignetting_size_compensation
    }
    fn optical_vignetting_operation(&self) -> &VignettingOperation {
        &self.optical_vignetting_operation
    }
    fn optical_vignetting_operation_mut(&mut self) -> &mut VignettingOperation {
        &mut self.optical_vignetting_operation
    }
    fn r_g_b_bokeh_texture_enable(&self) -> &bool {
        &self.r_g_b_bokeh_texture_enable
    }
    fn r_g_b_bokeh_texture_enable_mut(&mut self) -> &mut bool {
        &mut self.r_g_b_bokeh_texture_enable
    }
    fn bokeh_chromatic_aberration_enable(&self) -> &bool {
        &self.bokeh_chromatic_aberration_enable
    }
    fn bokeh_chromatic_aberration_enable_mut(&mut self) -> &mut bool {
        &mut self.bokeh_chromatic_aberration_enable
    }
    fn bokeh_chromatic_aberration_scale(&self) -> &f32 {
        &self.bokeh_chromatic_aberration_scale
    }
    fn bokeh_chromatic_aberration_scale_mut(&mut self) -> &mut f32 {
        &mut self.bokeh_chromatic_aberration_scale
    }
    fn bokeh_chromatic_aberration_radius(&self) -> &f32 {
        &self.bokeh_chromatic_aberration_radius
    }
    fn bokeh_chromatic_aberration_radius_mut(&mut self) -> &mut f32 {
        &mut self.bokeh_chromatic_aberration_radius
    }
    fn bokeh_chromatic_aberration_width(&self) -> &f32 {
        &self.bokeh_chromatic_aberration_width
    }
    fn bokeh_chromatic_aberration_width_mut(&mut self) -> &mut f32 {
        &mut self.bokeh_chromatic_aberration_width
    }
    fn bokeh_chromatic_aberration_radius_threshold(&self) -> &f32 {
        &self.bokeh_chromatic_aberration_radius_threshold
    }
    fn bokeh_chromatic_aberration_radius_threshold_mut(&mut self) -> &mut f32 {
        &mut self.bokeh_chromatic_aberration_radius_threshold
    }
    fn bokeh_chromatic_aberration_radius_threshold_width(&self) -> &f32 {
        &self.bokeh_chromatic_aberration_radius_threshold_width
    }
    fn bokeh_chromatic_aberration_radius_threshold_width_mut(&mut self) -> &mut f32 {
        &mut self.bokeh_chromatic_aberration_radius_threshold_width
    }
    fn bokeh_chromatic_aberration_energy_threshold(&self) -> &f32 {
        &self.bokeh_chromatic_aberration_energy_threshold
    }
    fn bokeh_chromatic_aberration_energy_threshold_mut(&mut self) -> &mut f32 {
        &mut self.bokeh_chromatic_aberration_energy_threshold
    }
    fn bokeh_chromatic_aberration_fg_color(&self) -> &super::core::Vec3 {
        &self.bokeh_chromatic_aberration_fg_color
    }
    fn bokeh_chromatic_aberration_fg_color_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.bokeh_chromatic_aberration_fg_color
    }
    fn bokeh_chromatic_aberration_bg_color(&self) -> &super::core::Vec3 {
        &self.bokeh_chromatic_aberration_bg_color
    }
    fn bokeh_chromatic_aberration_bg_color_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.bokeh_chromatic_aberration_bg_color
    }
    fn ironsights_dof_active(&self) -> &bool {
        &self.ironsights_dof_active
    }
    fn ironsights_dof_active_mut(&mut self) -> &mut bool {
        &mut self.ironsights_dof_active
    }
    fn ironsights_dof_extra_blur(&self) -> &bool {
        &self.ironsights_dof_extra_blur
    }
    fn ironsights_dof_extra_blur_mut(&mut self) -> &mut bool {
        &mut self.ironsights_dof_extra_blur
    }
    fn hip_to_ironsights_fade(&self) -> &f32 {
        &self.hip_to_ironsights_fade
    }
    fn hip_to_ironsights_fade_mut(&mut self) -> &mut f32 {
        &mut self.hip_to_ironsights_fade
    }
    fn ironsights_dof_start_fade(&self) -> &f32 {
        &self.ironsights_dof_start_fade
    }
    fn ironsights_dof_start_fade_mut(&mut self) -> &mut f32 {
        &mut self.ironsights_dof_start_fade
    }
    fn ironsights_focal_distance(&self) -> &f32 {
        &self.ironsights_focal_distance
    }
    fn ironsights_focal_distance_mut(&mut self) -> &mut f32 {
        &mut self.ironsights_focal_distance
    }
    fn ironsights_dof_circle_blur(&self) -> &bool {
        &self.ironsights_dof_circle_blur
    }
    fn ironsights_dof_circle_blur_mut(&mut self) -> &mut bool {
        &mut self.ironsights_dof_circle_blur
    }
    fn ironsights_dof_circle_distance(&self) -> &f32 {
        &self.ironsights_dof_circle_distance
    }
    fn ironsights_dof_circle_distance_mut(&mut self) -> &mut f32 {
        &mut self.ironsights_dof_circle_distance
    }
    fn ironsights_dof_circle_fade_distance(&self) -> &f32 {
        &self.ironsights_dof_circle_fade_distance
    }
    fn ironsights_dof_circle_fade_distance_mut(&mut self) -> &mut f32 {
        &mut self.ironsights_dof_circle_fade_distance
    }
    fn masked_blur_enabled(&self) -> &bool {
        &self.masked_blur_enabled
    }
    fn masked_blur_enabled_mut(&mut self) -> &mut bool {
        &mut self.masked_blur_enabled
    }
    fn masked_blur_amount(&self) -> &f32 {
        &self.masked_blur_amount
    }
    fn masked_blur_amount_mut(&mut self) -> &mut f32 {
        &mut self.masked_blur_amount
    }
    fn masked_blur_texture(&self) -> &Option<Arc<Mutex<dyn TextureBaseAssetTrait>>> {
        &self.masked_blur_texture
    }
    fn masked_blur_texture_mut(&mut self) -> &mut Option<Arc<Mutex<dyn TextureBaseAssetTrait>>> {
        &mut self.masked_blur_texture
    }
    fn circular_dof_anti_band_artifact(&self) -> &bool {
        &self.circular_dof_anti_band_artifact
    }
    fn circular_dof_anti_band_artifact_mut(&mut self) -> &mut bool {
        &mut self.circular_dof_anti_band_artifact
    }
    fn use_camera_settings(&self) -> &bool {
        &self.use_camera_settings
    }
    fn use_camera_settings_mut(&mut self) -> &mut bool {
        &mut self.use_camera_settings
    }
    fn simple_dof_max_blur(&self) -> &f32 {
        &self.simple_dof_max_blur
    }
    fn simple_dof_max_blur_mut(&mut self) -> &mut f32 {
        &mut self.simple_dof_max_blur
    }
    fn simple_dof_near_start(&self) -> &f32 {
        &self.simple_dof_near_start
    }
    fn simple_dof_near_start_mut(&mut self) -> &mut f32 {
        &mut self.simple_dof_near_start
    }
    fn simple_dof_near_end(&self) -> &f32 {
        &self.simple_dof_near_end
    }
    fn simple_dof_near_end_mut(&mut self) -> &mut f32 {
        &mut self.simple_dof_near_end
    }
    fn simple_dof_far_start(&self) -> &f32 {
        &self.simple_dof_far_start
    }
    fn simple_dof_far_start_mut(&mut self) -> &mut f32 {
        &mut self.simple_dof_far_start
    }
    fn simple_dof_far_end(&self) -> &f32 {
        &self.simple_dof_far_end
    }
    fn simple_dof_far_end_mut(&mut self) -> &mut f32 {
        &mut self.simple_dof_far_end
    }
    fn sprite_dof_near_start(&self) -> &f32 {
        &self.sprite_dof_near_start
    }
    fn sprite_dof_near_start_mut(&mut self) -> &mut f32 {
        &mut self.sprite_dof_near_start
    }
    fn sprite_dof_near_end(&self) -> &f32 {
        &self.sprite_dof_near_end
    }
    fn sprite_dof_near_end_mut(&mut self) -> &mut f32 {
        &mut self.sprite_dof_near_end
    }
    fn sprite_dof_far_start(&self) -> &f32 {
        &self.sprite_dof_far_start
    }
    fn sprite_dof_far_start_mut(&mut self) -> &mut f32 {
        &mut self.sprite_dof_far_start
    }
    fn sprite_dof_far_end(&self) -> &f32 {
        &self.sprite_dof_far_end
    }
    fn sprite_dof_far_end_mut(&mut self) -> &mut f32 {
        &mut self.sprite_dof_far_end
    }
    fn sprite_dof_max_blur(&self) -> &f32 {
        &self.sprite_dof_max_blur
    }
    fn sprite_dof_max_blur_mut(&mut self) -> &mut f32 {
        &mut self.sprite_dof_max_blur
    }
    fn anisotropy(&self) -> &f32 {
        &self.anisotropy
    }
    fn anisotropy_mut(&mut self) -> &mut f32 {
        &mut self.anisotropy
    }
    fn full_screen_blur_add_common_dof(&self) -> &f32 {
        &self.full_screen_blur_add_common_dof
    }
    fn full_screen_blur_add_common_dof_mut(&mut self) -> &mut f32 {
        &mut self.full_screen_blur_add_common_dof
    }
    fn field_flag_override0(&self) -> &u32 {
        &self.field_flag_override0
    }
    fn field_flag_override0_mut(&mut self) -> &mut u32 {
        &mut self.field_flag_override0
    }
    fn field_flag_override1(&self) -> &u32 {
        &self.field_flag_override1
    }
    fn field_flag_override1_mut(&mut self) -> &mut u32 {
        &mut self.field_flag_override1
    }
    fn field_flag_override2(&self) -> &u8 {
        &self.field_flag_override2
    }
    fn field_flag_override2_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_override2
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u32 {
        &mut self.field_flag_changed0
    }
    fn field_flag_changed1(&self) -> &u32 {
        &self.field_flag_changed1
    }
    fn field_flag_changed1_mut(&mut self) -> &mut u32 {
        &mut self.field_flag_changed1
    }
    fn field_flag_changed2(&self) -> &u16 {
        &self.field_flag_changed2
    }
    fn field_flag_changed2_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_changed2
    }
}

pub static DOFCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DofComponentState",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DofComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DofComponentState, enable),
            },
            FieldInfoData {
                name: "PhysicalCameraTweakEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DofComponentState, physical_camera_tweak_enable),
            },
            FieldInfoData {
                name: "PbcBackgroundBlurAdd",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, pbc_background_blur_add),
            },
            FieldInfoData {
                name: "PbcForegroundBlurAdd",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, pbc_foreground_blur_add),
            },
            FieldInfoData {
                name: "PbcFocusRangeAdd",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, pbc_focus_range_add),
            },
            FieldInfoData {
                name: "DofSource",
                flags: MemberInfoFlags::new(0),
                field_type: "DofSource",
                rust_offset: offset_of!(DofComponentState, dof_source),
            },
            FieldInfoData {
                name: "DebugDrawFocusPlane",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DofComponentState, debug_draw_focus_plane),
            },
            FieldInfoData {
                name: "FocusDofMaxBlur",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, focus_dof_max_blur),
            },
            FieldInfoData {
                name: "BlurFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, blur_factor),
            },
            FieldInfoData {
                name: "BlurAdd",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, blur_add),
            },
            FieldInfoData {
                name: "FocusDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, focus_distance),
            },
            FieldInfoData {
                name: "RadialBlurEnableCommonDof",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DofComponentState, radial_blur_enable_common_dof),
            },
            FieldInfoData {
                name: "RadialBlurAmountCommonDof",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, radial_blur_amount_common_dof),
            },
            FieldInfoData {
                name: "RadialBlurStartRadiusCommonDof",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, radial_blur_start_radius_common_dof),
            },
            FieldInfoData {
                name: "RadialBlurTransitionWidthCommonDof",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, radial_blur_transition_width_common_dof),
            },
            FieldInfoData {
                name: "RadialBlurTiltCommonDof",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, radial_blur_tilt_common_dof),
            },
            FieldInfoData {
                name: "RadialBlurHorizontalScaleCommonDof",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, radial_blur_horizontal_scale_common_dof),
            },
            FieldInfoData {
                name: "RadialBlurAspectRatioBlend",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, radial_blur_aspect_ratio_blend),
            },
            FieldInfoData {
                name: "RadialBlurPositionCommonDof",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(DofComponentState, radial_blur_position_common_dof),
            },
            FieldInfoData {
                name: "SimpleDofBlurFilter",
                flags: MemberInfoFlags::new(0),
                field_type: "BlurFilter",
                rust_offset: offset_of!(DofComponentState, simple_dof_blur_filter),
            },
            FieldInfoData {
                name: "SimpleDofStandardDeviation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, simple_dof_standard_deviation),
            },
            FieldInfoData {
                name: "SpriteDofBokehTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(DofComponentState, sprite_dof_bokeh_texture),
            },
            FieldInfoData {
                name: "FocusDofNearStart",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, focus_dof_near_start),
            },
            FieldInfoData {
                name: "FocusDofNearEnd",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, focus_dof_near_end),
            },
            FieldInfoData {
                name: "FocusDofFarStart",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, focus_dof_far_start),
            },
            FieldInfoData {
                name: "FocusDofFarEnd",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, focus_dof_far_end),
            },
            FieldInfoData {
                name: "PbrFocusLengthDof",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, pbr_focus_length_dof),
            },
            FieldInfoData {
                name: "PbrFilmWidthDof",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, pbr_film_width_dof),
            },
            FieldInfoData {
                name: "PbrFStopDof",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, pbr_f_stop_dof),
            },
            FieldInfoData {
                name: "OpticalVignettingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DofComponentState, optical_vignetting_enable),
            },
            FieldInfoData {
                name: "OpticalVignettingAmount",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, optical_vignetting_amount),
            },
            FieldInfoData {
                name: "OpticalVignettingAspectRatio",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, optical_vignetting_aspect_ratio),
            },
            FieldInfoData {
                name: "OpticalVignettingAnamorphicSqueeze",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, optical_vignetting_anamorphic_squeeze),
            },
            FieldInfoData {
                name: "OpticalVignettingSizeCompensation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, optical_vignetting_size_compensation),
            },
            FieldInfoData {
                name: "OpticalVignettingOperation",
                flags: MemberInfoFlags::new(0),
                field_type: "VignettingOperation",
                rust_offset: offset_of!(DofComponentState, optical_vignetting_operation),
            },
            FieldInfoData {
                name: "RGBBokehTextureEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DofComponentState, r_g_b_bokeh_texture_enable),
            },
            FieldInfoData {
                name: "BokehChromaticAberrationEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DofComponentState, bokeh_chromatic_aberration_enable),
            },
            FieldInfoData {
                name: "BokehChromaticAberrationScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, bokeh_chromatic_aberration_scale),
            },
            FieldInfoData {
                name: "BokehChromaticAberrationRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, bokeh_chromatic_aberration_radius),
            },
            FieldInfoData {
                name: "BokehChromaticAberrationWidth",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, bokeh_chromatic_aberration_width),
            },
            FieldInfoData {
                name: "BokehChromaticAberrationRadiusThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, bokeh_chromatic_aberration_radius_threshold),
            },
            FieldInfoData {
                name: "BokehChromaticAberrationRadiusThresholdWidth",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, bokeh_chromatic_aberration_radius_threshold_width),
            },
            FieldInfoData {
                name: "BokehChromaticAberrationEnergyThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, bokeh_chromatic_aberration_energy_threshold),
            },
            FieldInfoData {
                name: "BokehChromaticAberrationFgColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(DofComponentState, bokeh_chromatic_aberration_fg_color),
            },
            FieldInfoData {
                name: "BokehChromaticAberrationBgColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(DofComponentState, bokeh_chromatic_aberration_bg_color),
            },
            FieldInfoData {
                name: "IronsightsDofActive",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DofComponentState, ironsights_dof_active),
            },
            FieldInfoData {
                name: "IronsightsDofExtraBlur",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DofComponentState, ironsights_dof_extra_blur),
            },
            FieldInfoData {
                name: "HipToIronsightsFade",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, hip_to_ironsights_fade),
            },
            FieldInfoData {
                name: "IronsightsDofStartFade",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, ironsights_dof_start_fade),
            },
            FieldInfoData {
                name: "IronsightsFocalDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, ironsights_focal_distance),
            },
            FieldInfoData {
                name: "IronsightsDofCircleBlur",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DofComponentState, ironsights_dof_circle_blur),
            },
            FieldInfoData {
                name: "IronsightsDofCircleDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, ironsights_dof_circle_distance),
            },
            FieldInfoData {
                name: "IronsightsDofCircleFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, ironsights_dof_circle_fade_distance),
            },
            FieldInfoData {
                name: "MaskedBlurEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DofComponentState, masked_blur_enabled),
            },
            FieldInfoData {
                name: "MaskedBlurAmount",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, masked_blur_amount),
            },
            FieldInfoData {
                name: "MaskedBlurTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(DofComponentState, masked_blur_texture),
            },
            FieldInfoData {
                name: "CircularDofAntiBandArtifact",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DofComponentState, circular_dof_anti_band_artifact),
            },
            FieldInfoData {
                name: "UseCameraSettings",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DofComponentState, use_camera_settings),
            },
            FieldInfoData {
                name: "SimpleDofMaxBlur",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, simple_dof_max_blur),
            },
            FieldInfoData {
                name: "SimpleDofNearStart",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, simple_dof_near_start),
            },
            FieldInfoData {
                name: "SimpleDofNearEnd",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, simple_dof_near_end),
            },
            FieldInfoData {
                name: "SimpleDofFarStart",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, simple_dof_far_start),
            },
            FieldInfoData {
                name: "SimpleDofFarEnd",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, simple_dof_far_end),
            },
            FieldInfoData {
                name: "SpriteDofNearStart",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, sprite_dof_near_start),
            },
            FieldInfoData {
                name: "SpriteDofNearEnd",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, sprite_dof_near_end),
            },
            FieldInfoData {
                name: "SpriteDofFarStart",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, sprite_dof_far_start),
            },
            FieldInfoData {
                name: "SpriteDofFarEnd",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, sprite_dof_far_end),
            },
            FieldInfoData {
                name: "SpriteDofMaxBlur",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, sprite_dof_max_blur),
            },
            FieldInfoData {
                name: "Anisotropy",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, anisotropy),
            },
            FieldInfoData {
                name: "FullScreenBlurAddCommonDof",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DofComponentState, full_screen_blur_add_common_dof),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DofComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagOverride1",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DofComponentState, field_flag_override1),
            },
            FieldInfoData {
                name: "FieldFlagOverride2",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(DofComponentState, field_flag_override2),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DofComponentState, field_flag_changed0),
            },
            FieldInfoData {
                name: "FieldFlagChanged1",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DofComponentState, field_flag_changed1),
            },
            FieldInfoData {
                name: "FieldFlagChanged2",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(DofComponentState, field_flag_changed2),
            },
        ],
    }),
    array_type: Some(DOFCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DofComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        DOFCOMPONENTSTATE_TYPE_INFO
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


pub static DOFCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DofComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("DofComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum VignettingOperation {
    #[default]
    VignettingOperation_Min = 0,
    VignettingOperation_Average = 1,
    VignettingOperation_Max = 2,
}

pub static VIGNETTINGOPERATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VignettingOperation",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(VIGNETTINGOPERATION_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for VignettingOperation {
    fn type_info(&self) -> &'static TypeInfo {
        VIGNETTINGOPERATION_TYPE_INFO
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


pub static VIGNETTINGOPERATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VignettingOperation-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("VignettingOperation"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum DofSource {
    #[default]
    DofSource_Linear = 0,
    DofSource_PhysicallyBased = 1,
    DofSource_Camera = 2,
}

pub static DOFSOURCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DofSource",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(DOFSOURCE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DofSource {
    fn type_info(&self) -> &'static TypeInfo {
        DOFSOURCE_TYPE_INFO
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


pub static DOFSOURCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DofSource-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("DofSource"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ColorCorrectionComponentState {
    pub enable: bool,
    pub brightness: super::core::Vec3,
    pub contrast: super::core::Vec3,
    pub saturation: super::core::Vec3,
    pub hue: f32,
    pub color_grading_enable: bool,
    pub color_grading_texture: Option<Arc<Mutex<dyn TextureBaseAssetTrait>>>,
    pub color_grading_max_hdr_value: f32,
    pub hdr_color_grading_lut: Option<Arc<Mutex<dyn TextureBaseAssetTrait>>>,
    pub color_grading_stack: Vec<ColorGrading>,
    pub field_flag_override0: u16,
    pub field_flag_changed0: u16,
}

pub trait ColorCorrectionComponentStateTrait: TypeObject {
    fn enable(&self) -> &bool;
    fn enable_mut(&mut self) -> &mut bool;
    fn brightness(&self) -> &super::core::Vec3;
    fn brightness_mut(&mut self) -> &mut super::core::Vec3;
    fn contrast(&self) -> &super::core::Vec3;
    fn contrast_mut(&mut self) -> &mut super::core::Vec3;
    fn saturation(&self) -> &super::core::Vec3;
    fn saturation_mut(&mut self) -> &mut super::core::Vec3;
    fn hue(&self) -> &f32;
    fn hue_mut(&mut self) -> &mut f32;
    fn color_grading_enable(&self) -> &bool;
    fn color_grading_enable_mut(&mut self) -> &mut bool;
    fn color_grading_texture(&self) -> &Option<Arc<Mutex<dyn TextureBaseAssetTrait>>>;
    fn color_grading_texture_mut(&mut self) -> &mut Option<Arc<Mutex<dyn TextureBaseAssetTrait>>>;
    fn color_grading_max_hdr_value(&self) -> &f32;
    fn color_grading_max_hdr_value_mut(&mut self) -> &mut f32;
    fn hdr_color_grading_lut(&self) -> &Option<Arc<Mutex<dyn TextureBaseAssetTrait>>>;
    fn hdr_color_grading_lut_mut(&mut self) -> &mut Option<Arc<Mutex<dyn TextureBaseAssetTrait>>>;
    fn color_grading_stack(&self) -> &Vec<ColorGrading>;
    fn color_grading_stack_mut(&mut self) -> &mut Vec<ColorGrading>;
    fn field_flag_override0(&self) -> &u16;
    fn field_flag_override0_mut(&mut self) -> &mut u16;
    fn field_flag_changed0(&self) -> &u16;
    fn field_flag_changed0_mut(&mut self) -> &mut u16;
}

impl ColorCorrectionComponentStateTrait for ColorCorrectionComponentState {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn enable_mut(&mut self) -> &mut bool {
        &mut self.enable
    }
    fn brightness(&self) -> &super::core::Vec3 {
        &self.brightness
    }
    fn brightness_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.brightness
    }
    fn contrast(&self) -> &super::core::Vec3 {
        &self.contrast
    }
    fn contrast_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.contrast
    }
    fn saturation(&self) -> &super::core::Vec3 {
        &self.saturation
    }
    fn saturation_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.saturation
    }
    fn hue(&self) -> &f32 {
        &self.hue
    }
    fn hue_mut(&mut self) -> &mut f32 {
        &mut self.hue
    }
    fn color_grading_enable(&self) -> &bool {
        &self.color_grading_enable
    }
    fn color_grading_enable_mut(&mut self) -> &mut bool {
        &mut self.color_grading_enable
    }
    fn color_grading_texture(&self) -> &Option<Arc<Mutex<dyn TextureBaseAssetTrait>>> {
        &self.color_grading_texture
    }
    fn color_grading_texture_mut(&mut self) -> &mut Option<Arc<Mutex<dyn TextureBaseAssetTrait>>> {
        &mut self.color_grading_texture
    }
    fn color_grading_max_hdr_value(&self) -> &f32 {
        &self.color_grading_max_hdr_value
    }
    fn color_grading_max_hdr_value_mut(&mut self) -> &mut f32 {
        &mut self.color_grading_max_hdr_value
    }
    fn hdr_color_grading_lut(&self) -> &Option<Arc<Mutex<dyn TextureBaseAssetTrait>>> {
        &self.hdr_color_grading_lut
    }
    fn hdr_color_grading_lut_mut(&mut self) -> &mut Option<Arc<Mutex<dyn TextureBaseAssetTrait>>> {
        &mut self.hdr_color_grading_lut
    }
    fn color_grading_stack(&self) -> &Vec<ColorGrading> {
        &self.color_grading_stack
    }
    fn color_grading_stack_mut(&mut self) -> &mut Vec<ColorGrading> {
        &mut self.color_grading_stack
    }
    fn field_flag_override0(&self) -> &u16 {
        &self.field_flag_override0
    }
    fn field_flag_override0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_override0
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_changed0
    }
}

pub static COLORCORRECTIONCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ColorCorrectionComponentState",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ColorCorrectionComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ColorCorrectionComponentState, enable),
            },
            FieldInfoData {
                name: "Brightness",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(ColorCorrectionComponentState, brightness),
            },
            FieldInfoData {
                name: "Contrast",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(ColorCorrectionComponentState, contrast),
            },
            FieldInfoData {
                name: "Saturation",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(ColorCorrectionComponentState, saturation),
            },
            FieldInfoData {
                name: "Hue",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ColorCorrectionComponentState, hue),
            },
            FieldInfoData {
                name: "ColorGradingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ColorCorrectionComponentState, color_grading_enable),
            },
            FieldInfoData {
                name: "ColorGradingTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(ColorCorrectionComponentState, color_grading_texture),
            },
            FieldInfoData {
                name: "ColorGradingMaxHdrValue",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ColorCorrectionComponentState, color_grading_max_hdr_value),
            },
            FieldInfoData {
                name: "HdrColorGradingLut",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(ColorCorrectionComponentState, hdr_color_grading_lut),
            },
            FieldInfoData {
                name: "ColorGradingStack",
                flags: MemberInfoFlags::new(144),
                field_type: "ColorGrading-Array",
                rust_offset: offset_of!(ColorCorrectionComponentState, color_grading_stack),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(ColorCorrectionComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(ColorCorrectionComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(COLORCORRECTIONCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ColorCorrectionComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        COLORCORRECTIONCOMPONENTSTATE_TYPE_INFO
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


pub static COLORCORRECTIONCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ColorCorrectionComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ColorCorrectionComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ColorGrading {
    pub visibility: f32,
    pub texture: Option<Arc<Mutex<dyn TextureBaseAssetTrait>>>,
}

pub trait ColorGradingTrait: TypeObject {
    fn visibility(&self) -> &f32;
    fn visibility_mut(&mut self) -> &mut f32;
    fn texture(&self) -> &Option<Arc<Mutex<dyn TextureBaseAssetTrait>>>;
    fn texture_mut(&mut self) -> &mut Option<Arc<Mutex<dyn TextureBaseAssetTrait>>>;
}

impl ColorGradingTrait for ColorGrading {
    fn visibility(&self) -> &f32 {
        &self.visibility
    }
    fn visibility_mut(&mut self) -> &mut f32 {
        &mut self.visibility
    }
    fn texture(&self) -> &Option<Arc<Mutex<dyn TextureBaseAssetTrait>>> {
        &self.texture
    }
    fn texture_mut(&mut self) -> &mut Option<Arc<Mutex<dyn TextureBaseAssetTrait>>> {
        &mut self.texture
    }
}

pub static COLORGRADING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ColorGrading",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ColorGrading as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Visibility",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ColorGrading, visibility),
            },
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(ColorGrading, texture),
            },
        ],
    }),
    array_type: Some(COLORGRADING_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ColorGrading {
    fn type_info(&self) -> &'static TypeInfo {
        COLORGRADING_TYPE_INFO
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


pub static COLORGRADING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ColorGrading-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ColorGrading"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    pub f_f_t_kernel_texture: Option<Arc<Mutex<dyn TextureBaseAssetTrait>>>,
    pub chromostereopsis_enable: bool,
    pub chromostereopsis_scale: f32,
    pub chromostereopsis_offset: f32,
    pub lens_dirt_texture: Option<Arc<Mutex<dyn TextureBaseAssetTrait>>>,
    pub lens_dirt_bias: super::core::Vec3,
    pub lens_dirt_factor: super::core::Vec3,
    pub lens_dirt_exponent: super::core::Vec3,
    pub field_flag_override0: u32,
    pub field_flag_override1: u16,
    pub field_flag_changed0: u32,
    pub field_flag_changed1: u32,
}

pub trait TonemapComponentStateTrait: TypeObject {
    fn e_v(&self) -> &f32;
    fn e_v_mut(&mut self) -> &mut f32;
    fn exposure_compensation(&self) -> &f32;
    fn exposure_compensation_mut(&mut self) -> &mut f32;
    fn auto_exposure_darkest_exclude(&self) -> &f32;
    fn auto_exposure_darkest_exclude_mut(&mut self) -> &mut f32;
    fn auto_exposure_brightest_exclude(&self) -> &f32;
    fn auto_exposure_brightest_exclude_mut(&mut self) -> &mut f32;
    fn dark_adaptation_time(&self) -> &f32;
    fn dark_adaptation_time_mut(&mut self) -> &mut f32;
    fn light_adaptation_time(&self) -> &f32;
    fn light_adaptation_time_mut(&mut self) -> &mut f32;
    fn automatic_exposure(&self) -> &bool;
    fn automatic_exposure_mut(&mut self) -> &mut bool;
    fn auto_exposure_method(&self) -> &AutoExposureMethod;
    fn auto_exposure_method_mut(&mut self) -> &mut AutoExposureMethod;
    fn auto_exposure_higher_threshold(&self) -> &f32;
    fn auto_exposure_higher_threshold_mut(&mut self) -> &mut f32;
    fn auto_exposure_lower_threshold(&self) -> &f32;
    fn auto_exposure_lower_threshold_mut(&mut self) -> &mut f32;
    fn clamp_e_v(&self) -> &bool;
    fn clamp_e_v_mut(&mut self) -> &mut bool;
    fn min_e_v(&self) -> &f32;
    fn min_e_v_mut(&mut self) -> &mut f32;
    fn max_e_v(&self) -> &f32;
    fn max_e_v_mut(&mut self) -> &mut f32;
    fn spot_meter_scale(&self) -> &f32;
    fn spot_meter_scale_mut(&mut self) -> &mut f32;
    fn spot_meter_offset_x(&self) -> &f32;
    fn spot_meter_offset_x_mut(&mut self) -> &mut f32;
    fn spot_meter_offset_y(&self) -> &f32;
    fn spot_meter_offset_y_mut(&mut self) -> &mut f32;
    fn tonemap_method(&self) -> &TonemapMethod;
    fn tonemap_method_mut(&mut self) -> &mut TonemapMethod;
    fn bloom_direction(&self) -> &BloomDirection;
    fn bloom_direction_mut(&mut self) -> &mut BloomDirection;
    fn directional_bloom_clamp(&self) -> &f32;
    fn directional_bloom_clamp_mut(&mut self) -> &mut f32;
    fn bloom_scale(&self) -> &super::core::Vec3;
    fn bloom_scale_mut(&mut self) -> &mut super::core::Vec3;
    fn bloom_soft_clip(&self) -> &f32;
    fn bloom_soft_clip_mut(&mut self) -> &mut f32;
    fn bloom_method(&self) -> &BloomMethod;
    fn bloom_method_mut(&mut self) -> &mut BloomMethod;
    fn gaussian_sharpness(&self) -> &f32;
    fn gaussian_sharpness_mut(&mut self) -> &mut f32;
    fn gaussian1_color(&self) -> &super::core::Vec3;
    fn gaussian1_color_mut(&mut self) -> &mut super::core::Vec3;
    fn gaussian1_weight(&self) -> &f32;
    fn gaussian1_weight_mut(&mut self) -> &mut f32;
    fn gaussian2_color(&self) -> &super::core::Vec3;
    fn gaussian2_color_mut(&mut self) -> &mut super::core::Vec3;
    fn gaussian2_weight(&self) -> &f32;
    fn gaussian2_weight_mut(&mut self) -> &mut f32;
    fn gaussian3_color(&self) -> &super::core::Vec3;
    fn gaussian3_color_mut(&mut self) -> &mut super::core::Vec3;
    fn gaussian3_weight(&self) -> &f32;
    fn gaussian3_weight_mut(&mut self) -> &mut f32;
    fn gaussian4_color(&self) -> &super::core::Vec3;
    fn gaussian4_color_mut(&mut self) -> &mut super::core::Vec3;
    fn gaussian4_weight(&self) -> &f32;
    fn gaussian4_weight_mut(&mut self) -> &mut f32;
    fn gaussian5_color(&self) -> &super::core::Vec3;
    fn gaussian5_color_mut(&mut self) -> &mut super::core::Vec3;
    fn gaussian5_weight(&self) -> &f32;
    fn gaussian5_weight_mut(&mut self) -> &mut f32;
    fn f_f_t_threshold(&self) -> &f32;
    fn f_f_t_threshold_mut(&mut self) -> &mut f32;
    fn f_f_t_cutoff(&self) -> &f32;
    fn f_f_t_cutoff_mut(&mut self) -> &mut f32;
    fn f_f_t_kernel_scale(&self) -> &f32;
    fn f_f_t_kernel_scale_mut(&mut self) -> &mut f32;
    fn f_f_t_kernel_rotation(&self) -> &f32;
    fn f_f_t_kernel_rotation_mut(&mut self) -> &mut f32;
    fn f_f_t_spike_scale_limit_enable(&self) -> &bool;
    fn f_f_t_spike_scale_limit_enable_mut(&mut self) -> &mut bool;
    fn f_f_t_spike_scale_limit(&self) -> &f32;
    fn f_f_t_spike_scale_limit_mut(&mut self) -> &mut f32;
    fn f_f_t_kernel_texture(&self) -> &Option<Arc<Mutex<dyn TextureBaseAssetTrait>>>;
    fn f_f_t_kernel_texture_mut(&mut self) -> &mut Option<Arc<Mutex<dyn TextureBaseAssetTrait>>>;
    fn chromostereopsis_enable(&self) -> &bool;
    fn chromostereopsis_enable_mut(&mut self) -> &mut bool;
    fn chromostereopsis_scale(&self) -> &f32;
    fn chromostereopsis_scale_mut(&mut self) -> &mut f32;
    fn chromostereopsis_offset(&self) -> &f32;
    fn chromostereopsis_offset_mut(&mut self) -> &mut f32;
    fn lens_dirt_texture(&self) -> &Option<Arc<Mutex<dyn TextureBaseAssetTrait>>>;
    fn lens_dirt_texture_mut(&mut self) -> &mut Option<Arc<Mutex<dyn TextureBaseAssetTrait>>>;
    fn lens_dirt_bias(&self) -> &super::core::Vec3;
    fn lens_dirt_bias_mut(&mut self) -> &mut super::core::Vec3;
    fn lens_dirt_factor(&self) -> &super::core::Vec3;
    fn lens_dirt_factor_mut(&mut self) -> &mut super::core::Vec3;
    fn lens_dirt_exponent(&self) -> &super::core::Vec3;
    fn lens_dirt_exponent_mut(&mut self) -> &mut super::core::Vec3;
    fn field_flag_override0(&self) -> &u32;
    fn field_flag_override0_mut(&mut self) -> &mut u32;
    fn field_flag_override1(&self) -> &u16;
    fn field_flag_override1_mut(&mut self) -> &mut u16;
    fn field_flag_changed0(&self) -> &u32;
    fn field_flag_changed0_mut(&mut self) -> &mut u32;
    fn field_flag_changed1(&self) -> &u32;
    fn field_flag_changed1_mut(&mut self) -> &mut u32;
}

impl TonemapComponentStateTrait for TonemapComponentState {
    fn e_v(&self) -> &f32 {
        &self.e_v
    }
    fn e_v_mut(&mut self) -> &mut f32 {
        &mut self.e_v
    }
    fn exposure_compensation(&self) -> &f32 {
        &self.exposure_compensation
    }
    fn exposure_compensation_mut(&mut self) -> &mut f32 {
        &mut self.exposure_compensation
    }
    fn auto_exposure_darkest_exclude(&self) -> &f32 {
        &self.auto_exposure_darkest_exclude
    }
    fn auto_exposure_darkest_exclude_mut(&mut self) -> &mut f32 {
        &mut self.auto_exposure_darkest_exclude
    }
    fn auto_exposure_brightest_exclude(&self) -> &f32 {
        &self.auto_exposure_brightest_exclude
    }
    fn auto_exposure_brightest_exclude_mut(&mut self) -> &mut f32 {
        &mut self.auto_exposure_brightest_exclude
    }
    fn dark_adaptation_time(&self) -> &f32 {
        &self.dark_adaptation_time
    }
    fn dark_adaptation_time_mut(&mut self) -> &mut f32 {
        &mut self.dark_adaptation_time
    }
    fn light_adaptation_time(&self) -> &f32 {
        &self.light_adaptation_time
    }
    fn light_adaptation_time_mut(&mut self) -> &mut f32 {
        &mut self.light_adaptation_time
    }
    fn automatic_exposure(&self) -> &bool {
        &self.automatic_exposure
    }
    fn automatic_exposure_mut(&mut self) -> &mut bool {
        &mut self.automatic_exposure
    }
    fn auto_exposure_method(&self) -> &AutoExposureMethod {
        &self.auto_exposure_method
    }
    fn auto_exposure_method_mut(&mut self) -> &mut AutoExposureMethod {
        &mut self.auto_exposure_method
    }
    fn auto_exposure_higher_threshold(&self) -> &f32 {
        &self.auto_exposure_higher_threshold
    }
    fn auto_exposure_higher_threshold_mut(&mut self) -> &mut f32 {
        &mut self.auto_exposure_higher_threshold
    }
    fn auto_exposure_lower_threshold(&self) -> &f32 {
        &self.auto_exposure_lower_threshold
    }
    fn auto_exposure_lower_threshold_mut(&mut self) -> &mut f32 {
        &mut self.auto_exposure_lower_threshold
    }
    fn clamp_e_v(&self) -> &bool {
        &self.clamp_e_v
    }
    fn clamp_e_v_mut(&mut self) -> &mut bool {
        &mut self.clamp_e_v
    }
    fn min_e_v(&self) -> &f32 {
        &self.min_e_v
    }
    fn min_e_v_mut(&mut self) -> &mut f32 {
        &mut self.min_e_v
    }
    fn max_e_v(&self) -> &f32 {
        &self.max_e_v
    }
    fn max_e_v_mut(&mut self) -> &mut f32 {
        &mut self.max_e_v
    }
    fn spot_meter_scale(&self) -> &f32 {
        &self.spot_meter_scale
    }
    fn spot_meter_scale_mut(&mut self) -> &mut f32 {
        &mut self.spot_meter_scale
    }
    fn spot_meter_offset_x(&self) -> &f32 {
        &self.spot_meter_offset_x
    }
    fn spot_meter_offset_x_mut(&mut self) -> &mut f32 {
        &mut self.spot_meter_offset_x
    }
    fn spot_meter_offset_y(&self) -> &f32 {
        &self.spot_meter_offset_y
    }
    fn spot_meter_offset_y_mut(&mut self) -> &mut f32 {
        &mut self.spot_meter_offset_y
    }
    fn tonemap_method(&self) -> &TonemapMethod {
        &self.tonemap_method
    }
    fn tonemap_method_mut(&mut self) -> &mut TonemapMethod {
        &mut self.tonemap_method
    }
    fn bloom_direction(&self) -> &BloomDirection {
        &self.bloom_direction
    }
    fn bloom_direction_mut(&mut self) -> &mut BloomDirection {
        &mut self.bloom_direction
    }
    fn directional_bloom_clamp(&self) -> &f32 {
        &self.directional_bloom_clamp
    }
    fn directional_bloom_clamp_mut(&mut self) -> &mut f32 {
        &mut self.directional_bloom_clamp
    }
    fn bloom_scale(&self) -> &super::core::Vec3 {
        &self.bloom_scale
    }
    fn bloom_scale_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.bloom_scale
    }
    fn bloom_soft_clip(&self) -> &f32 {
        &self.bloom_soft_clip
    }
    fn bloom_soft_clip_mut(&mut self) -> &mut f32 {
        &mut self.bloom_soft_clip
    }
    fn bloom_method(&self) -> &BloomMethod {
        &self.bloom_method
    }
    fn bloom_method_mut(&mut self) -> &mut BloomMethod {
        &mut self.bloom_method
    }
    fn gaussian_sharpness(&self) -> &f32 {
        &self.gaussian_sharpness
    }
    fn gaussian_sharpness_mut(&mut self) -> &mut f32 {
        &mut self.gaussian_sharpness
    }
    fn gaussian1_color(&self) -> &super::core::Vec3 {
        &self.gaussian1_color
    }
    fn gaussian1_color_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.gaussian1_color
    }
    fn gaussian1_weight(&self) -> &f32 {
        &self.gaussian1_weight
    }
    fn gaussian1_weight_mut(&mut self) -> &mut f32 {
        &mut self.gaussian1_weight
    }
    fn gaussian2_color(&self) -> &super::core::Vec3 {
        &self.gaussian2_color
    }
    fn gaussian2_color_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.gaussian2_color
    }
    fn gaussian2_weight(&self) -> &f32 {
        &self.gaussian2_weight
    }
    fn gaussian2_weight_mut(&mut self) -> &mut f32 {
        &mut self.gaussian2_weight
    }
    fn gaussian3_color(&self) -> &super::core::Vec3 {
        &self.gaussian3_color
    }
    fn gaussian3_color_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.gaussian3_color
    }
    fn gaussian3_weight(&self) -> &f32 {
        &self.gaussian3_weight
    }
    fn gaussian3_weight_mut(&mut self) -> &mut f32 {
        &mut self.gaussian3_weight
    }
    fn gaussian4_color(&self) -> &super::core::Vec3 {
        &self.gaussian4_color
    }
    fn gaussian4_color_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.gaussian4_color
    }
    fn gaussian4_weight(&self) -> &f32 {
        &self.gaussian4_weight
    }
    fn gaussian4_weight_mut(&mut self) -> &mut f32 {
        &mut self.gaussian4_weight
    }
    fn gaussian5_color(&self) -> &super::core::Vec3 {
        &self.gaussian5_color
    }
    fn gaussian5_color_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.gaussian5_color
    }
    fn gaussian5_weight(&self) -> &f32 {
        &self.gaussian5_weight
    }
    fn gaussian5_weight_mut(&mut self) -> &mut f32 {
        &mut self.gaussian5_weight
    }
    fn f_f_t_threshold(&self) -> &f32 {
        &self.f_f_t_threshold
    }
    fn f_f_t_threshold_mut(&mut self) -> &mut f32 {
        &mut self.f_f_t_threshold
    }
    fn f_f_t_cutoff(&self) -> &f32 {
        &self.f_f_t_cutoff
    }
    fn f_f_t_cutoff_mut(&mut self) -> &mut f32 {
        &mut self.f_f_t_cutoff
    }
    fn f_f_t_kernel_scale(&self) -> &f32 {
        &self.f_f_t_kernel_scale
    }
    fn f_f_t_kernel_scale_mut(&mut self) -> &mut f32 {
        &mut self.f_f_t_kernel_scale
    }
    fn f_f_t_kernel_rotation(&self) -> &f32 {
        &self.f_f_t_kernel_rotation
    }
    fn f_f_t_kernel_rotation_mut(&mut self) -> &mut f32 {
        &mut self.f_f_t_kernel_rotation
    }
    fn f_f_t_spike_scale_limit_enable(&self) -> &bool {
        &self.f_f_t_spike_scale_limit_enable
    }
    fn f_f_t_spike_scale_limit_enable_mut(&mut self) -> &mut bool {
        &mut self.f_f_t_spike_scale_limit_enable
    }
    fn f_f_t_spike_scale_limit(&self) -> &f32 {
        &self.f_f_t_spike_scale_limit
    }
    fn f_f_t_spike_scale_limit_mut(&mut self) -> &mut f32 {
        &mut self.f_f_t_spike_scale_limit
    }
    fn f_f_t_kernel_texture(&self) -> &Option<Arc<Mutex<dyn TextureBaseAssetTrait>>> {
        &self.f_f_t_kernel_texture
    }
    fn f_f_t_kernel_texture_mut(&mut self) -> &mut Option<Arc<Mutex<dyn TextureBaseAssetTrait>>> {
        &mut self.f_f_t_kernel_texture
    }
    fn chromostereopsis_enable(&self) -> &bool {
        &self.chromostereopsis_enable
    }
    fn chromostereopsis_enable_mut(&mut self) -> &mut bool {
        &mut self.chromostereopsis_enable
    }
    fn chromostereopsis_scale(&self) -> &f32 {
        &self.chromostereopsis_scale
    }
    fn chromostereopsis_scale_mut(&mut self) -> &mut f32 {
        &mut self.chromostereopsis_scale
    }
    fn chromostereopsis_offset(&self) -> &f32 {
        &self.chromostereopsis_offset
    }
    fn chromostereopsis_offset_mut(&mut self) -> &mut f32 {
        &mut self.chromostereopsis_offset
    }
    fn lens_dirt_texture(&self) -> &Option<Arc<Mutex<dyn TextureBaseAssetTrait>>> {
        &self.lens_dirt_texture
    }
    fn lens_dirt_texture_mut(&mut self) -> &mut Option<Arc<Mutex<dyn TextureBaseAssetTrait>>> {
        &mut self.lens_dirt_texture
    }
    fn lens_dirt_bias(&self) -> &super::core::Vec3 {
        &self.lens_dirt_bias
    }
    fn lens_dirt_bias_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.lens_dirt_bias
    }
    fn lens_dirt_factor(&self) -> &super::core::Vec3 {
        &self.lens_dirt_factor
    }
    fn lens_dirt_factor_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.lens_dirt_factor
    }
    fn lens_dirt_exponent(&self) -> &super::core::Vec3 {
        &self.lens_dirt_exponent
    }
    fn lens_dirt_exponent_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.lens_dirt_exponent
    }
    fn field_flag_override0(&self) -> &u32 {
        &self.field_flag_override0
    }
    fn field_flag_override0_mut(&mut self) -> &mut u32 {
        &mut self.field_flag_override0
    }
    fn field_flag_override1(&self) -> &u16 {
        &self.field_flag_override1
    }
    fn field_flag_override1_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_override1
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u32 {
        &mut self.field_flag_changed0
    }
    fn field_flag_changed1(&self) -> &u32 {
        &self.field_flag_changed1
    }
    fn field_flag_changed1_mut(&mut self) -> &mut u32 {
        &mut self.field_flag_changed1
    }
}

pub static TONEMAPCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TonemapComponentState",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TonemapComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "EV",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TonemapComponentState, e_v),
            },
            FieldInfoData {
                name: "ExposureCompensation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TonemapComponentState, exposure_compensation),
            },
            FieldInfoData {
                name: "AutoExposureDarkestExclude",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TonemapComponentState, auto_exposure_darkest_exclude),
            },
            FieldInfoData {
                name: "AutoExposureBrightestExclude",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TonemapComponentState, auto_exposure_brightest_exclude),
            },
            FieldInfoData {
                name: "DarkAdaptationTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TonemapComponentState, dark_adaptation_time),
            },
            FieldInfoData {
                name: "LightAdaptationTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TonemapComponentState, light_adaptation_time),
            },
            FieldInfoData {
                name: "AutomaticExposure",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TonemapComponentState, automatic_exposure),
            },
            FieldInfoData {
                name: "AutoExposureMethod",
                flags: MemberInfoFlags::new(0),
                field_type: "AutoExposureMethod",
                rust_offset: offset_of!(TonemapComponentState, auto_exposure_method),
            },
            FieldInfoData {
                name: "AutoExposureHigherThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TonemapComponentState, auto_exposure_higher_threshold),
            },
            FieldInfoData {
                name: "AutoExposureLowerThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TonemapComponentState, auto_exposure_lower_threshold),
            },
            FieldInfoData {
                name: "ClampEV",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TonemapComponentState, clamp_e_v),
            },
            FieldInfoData {
                name: "MinEV",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TonemapComponentState, min_e_v),
            },
            FieldInfoData {
                name: "MaxEV",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TonemapComponentState, max_e_v),
            },
            FieldInfoData {
                name: "SpotMeterScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TonemapComponentState, spot_meter_scale),
            },
            FieldInfoData {
                name: "SpotMeterOffsetX",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TonemapComponentState, spot_meter_offset_x),
            },
            FieldInfoData {
                name: "SpotMeterOffsetY",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TonemapComponentState, spot_meter_offset_y),
            },
            FieldInfoData {
                name: "TonemapMethod",
                flags: MemberInfoFlags::new(0),
                field_type: "TonemapMethod",
                rust_offset: offset_of!(TonemapComponentState, tonemap_method),
            },
            FieldInfoData {
                name: "BloomDirection",
                flags: MemberInfoFlags::new(0),
                field_type: "BloomDirection",
                rust_offset: offset_of!(TonemapComponentState, bloom_direction),
            },
            FieldInfoData {
                name: "DirectionalBloomClamp",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TonemapComponentState, directional_bloom_clamp),
            },
            FieldInfoData {
                name: "BloomScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TonemapComponentState, bloom_scale),
            },
            FieldInfoData {
                name: "BloomSoftClip",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TonemapComponentState, bloom_soft_clip),
            },
            FieldInfoData {
                name: "BloomMethod",
                flags: MemberInfoFlags::new(0),
                field_type: "BloomMethod",
                rust_offset: offset_of!(TonemapComponentState, bloom_method),
            },
            FieldInfoData {
                name: "GaussianSharpness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TonemapComponentState, gaussian_sharpness),
            },
            FieldInfoData {
                name: "Gaussian1Color",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TonemapComponentState, gaussian1_color),
            },
            FieldInfoData {
                name: "Gaussian1Weight",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TonemapComponentState, gaussian1_weight),
            },
            FieldInfoData {
                name: "Gaussian2Color",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TonemapComponentState, gaussian2_color),
            },
            FieldInfoData {
                name: "Gaussian2Weight",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TonemapComponentState, gaussian2_weight),
            },
            FieldInfoData {
                name: "Gaussian3Color",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TonemapComponentState, gaussian3_color),
            },
            FieldInfoData {
                name: "Gaussian3Weight",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TonemapComponentState, gaussian3_weight),
            },
            FieldInfoData {
                name: "Gaussian4Color",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TonemapComponentState, gaussian4_color),
            },
            FieldInfoData {
                name: "Gaussian4Weight",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TonemapComponentState, gaussian4_weight),
            },
            FieldInfoData {
                name: "Gaussian5Color",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TonemapComponentState, gaussian5_color),
            },
            FieldInfoData {
                name: "Gaussian5Weight",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TonemapComponentState, gaussian5_weight),
            },
            FieldInfoData {
                name: "FFTThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TonemapComponentState, f_f_t_threshold),
            },
            FieldInfoData {
                name: "FFTCutoff",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TonemapComponentState, f_f_t_cutoff),
            },
            FieldInfoData {
                name: "FFTKernelScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TonemapComponentState, f_f_t_kernel_scale),
            },
            FieldInfoData {
                name: "FFTKernelRotation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TonemapComponentState, f_f_t_kernel_rotation),
            },
            FieldInfoData {
                name: "FFTSpikeScaleLimitEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TonemapComponentState, f_f_t_spike_scale_limit_enable),
            },
            FieldInfoData {
                name: "FFTSpikeScaleLimit",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TonemapComponentState, f_f_t_spike_scale_limit),
            },
            FieldInfoData {
                name: "FFTKernelTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(TonemapComponentState, f_f_t_kernel_texture),
            },
            FieldInfoData {
                name: "ChromostereopsisEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TonemapComponentState, chromostereopsis_enable),
            },
            FieldInfoData {
                name: "ChromostereopsisScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TonemapComponentState, chromostereopsis_scale),
            },
            FieldInfoData {
                name: "ChromostereopsisOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TonemapComponentState, chromostereopsis_offset),
            },
            FieldInfoData {
                name: "LensDirtTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(TonemapComponentState, lens_dirt_texture),
            },
            FieldInfoData {
                name: "LensDirtBias",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TonemapComponentState, lens_dirt_bias),
            },
            FieldInfoData {
                name: "LensDirtFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TonemapComponentState, lens_dirt_factor),
            },
            FieldInfoData {
                name: "LensDirtExponent",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TonemapComponentState, lens_dirt_exponent),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TonemapComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagOverride1",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(TonemapComponentState, field_flag_override1),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TonemapComponentState, field_flag_changed0),
            },
            FieldInfoData {
                name: "FieldFlagChanged1",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TonemapComponentState, field_flag_changed1),
            },
        ],
    }),
    array_type: Some(TONEMAPCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TonemapComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        TONEMAPCOMPONENTSTATE_TYPE_INFO
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


pub static TONEMAPCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TonemapComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("TonemapComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum TonemapMethod {
    #[default]
    TonemapMethod_None = 4,
    TonemapMethod_Linear = 0,
    TonemapMethod_Filmic = 1,
    TonemapMethod_FilmicNeutral = 2,
    TonemapMethod_LinearApproxGamma = 3,
}

pub static TONEMAPMETHOD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TonemapMethod",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(TONEMAPMETHOD_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TonemapMethod {
    fn type_info(&self) -> &'static TypeInfo {
        TONEMAPMETHOD_TYPE_INFO
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


pub static TONEMAPMETHOD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TonemapMethod-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("TonemapMethod"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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

pub trait SubSurfaceProfileTrait: TypeObject {
    fn preset(&self) -> &SubSurfaceProfilePreset;
    fn preset_mut(&mut self) -> &mut SubSurfaceProfilePreset;
    fn scattering_enable(&self) -> &bool;
    fn scattering_enable_mut(&mut self) -> &mut bool;
    fn scattering_scale(&self) -> &f32;
    fn scattering_scale_mut(&mut self) -> &mut f32;
    fn scattering_spike_amount(&self) -> &f32;
    fn scattering_spike_amount_mut(&mut self) -> &mut f32;
    fn radius_r(&self) -> &f32;
    fn radius_r_mut(&mut self) -> &mut f32;
    fn radius_g(&self) -> &f32;
    fn radius_g_mut(&mut self) -> &mut f32;
    fn radius_b(&self) -> &f32;
    fn radius_b_mut(&mut self) -> &mut f32;
    fn translucency_enable(&self) -> &bool;
    fn translucency_enable_mut(&mut self) -> &mut bool;
    fn automatic_thickness_enable(&self) -> &bool;
    fn automatic_thickness_enable_mut(&mut self) -> &mut bool;
    fn translucency_scale(&self) -> &f32;
    fn translucency_scale_mut(&mut self) -> &mut f32;
    fn translucency_base_color_amount(&self) -> &f32;
    fn translucency_base_color_amount_mut(&mut self) -> &mut f32;
    fn translucency_multiplier(&self) -> &f32;
    fn translucency_multiplier_mut(&mut self) -> &mut f32;
    fn translucency_shadow_bias(&self) -> &f32;
    fn translucency_shadow_bias_mut(&mut self) -> &mut f32;
}

impl SubSurfaceProfileTrait for SubSurfaceProfile {
    fn preset(&self) -> &SubSurfaceProfilePreset {
        &self.preset
    }
    fn preset_mut(&mut self) -> &mut SubSurfaceProfilePreset {
        &mut self.preset
    }
    fn scattering_enable(&self) -> &bool {
        &self.scattering_enable
    }
    fn scattering_enable_mut(&mut self) -> &mut bool {
        &mut self.scattering_enable
    }
    fn scattering_scale(&self) -> &f32 {
        &self.scattering_scale
    }
    fn scattering_scale_mut(&mut self) -> &mut f32 {
        &mut self.scattering_scale
    }
    fn scattering_spike_amount(&self) -> &f32 {
        &self.scattering_spike_amount
    }
    fn scattering_spike_amount_mut(&mut self) -> &mut f32 {
        &mut self.scattering_spike_amount
    }
    fn radius_r(&self) -> &f32 {
        &self.radius_r
    }
    fn radius_r_mut(&mut self) -> &mut f32 {
        &mut self.radius_r
    }
    fn radius_g(&self) -> &f32 {
        &self.radius_g
    }
    fn radius_g_mut(&mut self) -> &mut f32 {
        &mut self.radius_g
    }
    fn radius_b(&self) -> &f32 {
        &self.radius_b
    }
    fn radius_b_mut(&mut self) -> &mut f32 {
        &mut self.radius_b
    }
    fn translucency_enable(&self) -> &bool {
        &self.translucency_enable
    }
    fn translucency_enable_mut(&mut self) -> &mut bool {
        &mut self.translucency_enable
    }
    fn automatic_thickness_enable(&self) -> &bool {
        &self.automatic_thickness_enable
    }
    fn automatic_thickness_enable_mut(&mut self) -> &mut bool {
        &mut self.automatic_thickness_enable
    }
    fn translucency_scale(&self) -> &f32 {
        &self.translucency_scale
    }
    fn translucency_scale_mut(&mut self) -> &mut f32 {
        &mut self.translucency_scale
    }
    fn translucency_base_color_amount(&self) -> &f32 {
        &self.translucency_base_color_amount
    }
    fn translucency_base_color_amount_mut(&mut self) -> &mut f32 {
        &mut self.translucency_base_color_amount
    }
    fn translucency_multiplier(&self) -> &f32 {
        &self.translucency_multiplier
    }
    fn translucency_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.translucency_multiplier
    }
    fn translucency_shadow_bias(&self) -> &f32 {
        &self.translucency_shadow_bias
    }
    fn translucency_shadow_bias_mut(&mut self) -> &mut f32 {
        &mut self.translucency_shadow_bias
    }
}

pub static SUBSURFACEPROFILE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubSurfaceProfile",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SubSurfaceProfile as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Preset",
                flags: MemberInfoFlags::new(0),
                field_type: "SubSurfaceProfilePreset",
                rust_offset: offset_of!(SubSurfaceProfile, preset),
            },
            FieldInfoData {
                name: "ScatteringEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SubSurfaceProfile, scattering_enable),
            },
            FieldInfoData {
                name: "ScatteringScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SubSurfaceProfile, scattering_scale),
            },
            FieldInfoData {
                name: "ScatteringSpikeAmount",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SubSurfaceProfile, scattering_spike_amount),
            },
            FieldInfoData {
                name: "RadiusR",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SubSurfaceProfile, radius_r),
            },
            FieldInfoData {
                name: "RadiusG",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SubSurfaceProfile, radius_g),
            },
            FieldInfoData {
                name: "RadiusB",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SubSurfaceProfile, radius_b),
            },
            FieldInfoData {
                name: "TranslucencyEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SubSurfaceProfile, translucency_enable),
            },
            FieldInfoData {
                name: "AutomaticThicknessEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SubSurfaceProfile, automatic_thickness_enable),
            },
            FieldInfoData {
                name: "TranslucencyScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SubSurfaceProfile, translucency_scale),
            },
            FieldInfoData {
                name: "TranslucencyBaseColorAmount",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SubSurfaceProfile, translucency_base_color_amount),
            },
            FieldInfoData {
                name: "TranslucencyMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SubSurfaceProfile, translucency_multiplier),
            },
            FieldInfoData {
                name: "TranslucencyShadowBias",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SubSurfaceProfile, translucency_shadow_bias),
            },
        ],
    }),
    array_type: Some(SUBSURFACEPROFILE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for SubSurfaceProfile {
    fn type_info(&self) -> &'static TypeInfo {
        SUBSURFACEPROFILE_TYPE_INFO
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


pub static SUBSURFACEPROFILE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubSurfaceProfile-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("SubSurfaceProfile"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum SubSurfaceProfilePreset {
    #[default]
    SubSurfaceProfilePreset_Custom = 0,
    SubSurfaceProfilePreset_CaucasianSkin = 1,
}

pub static SUBSURFACEPROFILEPRESET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubSurfaceProfilePreset",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(SUBSURFACEPROFILEPRESET_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SubSurfaceProfilePreset {
    fn type_info(&self) -> &'static TypeInfo {
        SUBSURFACEPROFILEPRESET_TYPE_INFO
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


pub static SUBSURFACEPROFILEPRESET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubSurfaceProfilePreset-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("SubSurfaceProfilePreset"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ColorGradingQualityMode {
    #[default]
    ColorGradingQualityMode_Low = 0,
    ColorGradingQualityMode_High = 1,
}

pub static COLORGRADINGQUALITYMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ColorGradingQualityMode",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(COLORGRADINGQUALITYMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ColorGradingQualityMode {
    fn type_info(&self) -> &'static TypeInfo {
        COLORGRADINGQUALITYMODE_TYPE_INFO
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


pub static COLORGRADINGQUALITYMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ColorGradingQualityMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ColorGradingQualityMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum BloomDirection {
    #[default]
    BloomDirection_None = 0,
    BloomDirection_Horizontal = 1,
    BloomDirection_Vertical = 2,
}

pub static BLOOMDIRECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BloomDirection",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(BLOOMDIRECTION_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for BloomDirection {
    fn type_info(&self) -> &'static TypeInfo {
        BLOOMDIRECTION_TYPE_INFO
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


pub static BLOOMDIRECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BloomDirection-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("BloomDirection"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum BloomMethod {
    #[default]
    BloomMethod_GaussianSimple = 0,
    BloomMethod_GaussianCustom = 1,
    BloomMethod_FFT = 2,
}

pub static BLOOMMETHOD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BloomMethod",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(BLOOMMETHOD_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for BloomMethod {
    fn type_info(&self) -> &'static TypeInfo {
        BLOOMMETHOD_TYPE_INFO
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


pub static BLOOMMETHOD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BloomMethod-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("BloomMethod"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static BLURFILTER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlurFilter",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(BLURFILTER_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for BlurFilter {
    fn type_info(&self) -> &'static TypeInfo {
        BLURFILTER_TYPE_INFO
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


pub static BLURFILTER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlurFilter-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("BlurFilter"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static LOCALPLAYERVIEWID_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalPlayerViewId",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(LOCALPLAYERVIEWID_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LocalPlayerViewId {
    fn type_info(&self) -> &'static TypeInfo {
        LOCALPLAYERVIEWID_TYPE_INFO
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


pub static LOCALPLAYERVIEWID_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalPlayerViewId-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("LocalPlayerViewId"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum DistantIBLLocationType {
    #[default]
    DistantIBLLocationType_Outdoor = 0,
    DistantIBLLocationType_Indoor = 1,
    DistantIBLLocationTypeCount = 2,
}

pub static DISTANTIBLLOCATIONTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistantIBLLocationType",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(DISTANTIBLLOCATIONTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DistantIBLLocationType {
    fn type_info(&self) -> &'static TypeInfo {
        DISTANTIBLLOCATIONTYPE_TYPE_INFO
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


pub static DISTANTIBLLOCATIONTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistantIBLLocationType-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("DistantIBLLocationType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static LIGHTTILEPASSTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightTilePassType",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(LIGHTTILEPASSTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LightTilePassType {
    fn type_info(&self) -> &'static TypeInfo {
        LIGHTTILEPASSTYPE_TYPE_INFO
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


pub static LIGHTTILEPASSTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightTilePassType-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("LightTilePassType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum LocalIBLMode {
    #[default]
    LocalIBLMode_Static = 0,
    LocalIBLMode_Dynamic = 1,
    LocalIBLMode_Baked = 2,
}

pub static LOCALIBLMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalIBLMode",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(LOCALIBLMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LocalIBLMode {
    fn type_info(&self) -> &'static TypeInfo {
        LOCALIBLMODE_TYPE_INFO
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


pub static LOCALIBLMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalIBLMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("LocalIBLMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum PunctualLightType {
    #[default]
    PunctualLightType_Point = 0,
    PunctualLightType_Line = 1,
    PunctualLightType_Cone = 2,
    PunctualLightType_Spot = 3,
    PunctualLightTypeCount = 4,
}

pub static PUNCTUALLIGHTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PunctualLightType",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(PUNCTUALLIGHTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PunctualLightType {
    fn type_info(&self) -> &'static TypeInfo {
        PUNCTUALLIGHTTYPE_TYPE_INFO
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


pub static PUNCTUALLIGHTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PunctualLightType-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("PunctualLightType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum RectangularLightShape {
    #[default]
    RectangularLightShape_Rectangular = 0,
    RectangularLightShape_Frustum = 1,
    RectangularLightShape_OrthoFrustum = 2,
}

pub static RECTANGULARLIGHTSHAPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RectangularLightShape",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(RECTANGULARLIGHTSHAPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RectangularLightShape {
    fn type_info(&self) -> &'static TypeInfo {
        RECTANGULARLIGHTSHAPE_TYPE_INFO
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


pub static RECTANGULARLIGHTSHAPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RectangularLightShape-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("RectangularLightShape"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum LightUnitType {
    #[default]
    LightUnitType_LuminousPower = 0,
    LightUnitType_Luminance = 1,
}

pub static LIGHTUNITTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightUnitType",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(LIGHTUNITTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LightUnitType {
    fn type_info(&self) -> &'static TypeInfo {
        LIGHTUNITTYPE_TYPE_INFO
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


pub static LIGHTUNITTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightUnitType-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("LightUnitType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PipelineExternalShaderConditional {
    pub condition_name: String,
    pub default_index: i32,
    pub branches: Vec<PipelineExternalShaderConditionalBranch>,
    pub is_global: bool,
}

pub trait PipelineExternalShaderConditionalTrait: TypeObject {
    fn condition_name(&self) -> &String;
    fn condition_name_mut(&mut self) -> &mut String;
    fn default_index(&self) -> &i32;
    fn default_index_mut(&mut self) -> &mut i32;
    fn branches(&self) -> &Vec<PipelineExternalShaderConditionalBranch>;
    fn branches_mut(&mut self) -> &mut Vec<PipelineExternalShaderConditionalBranch>;
    fn is_global(&self) -> &bool;
    fn is_global_mut(&mut self) -> &mut bool;
}

impl PipelineExternalShaderConditionalTrait for PipelineExternalShaderConditional {
    fn condition_name(&self) -> &String {
        &self.condition_name
    }
    fn condition_name_mut(&mut self) -> &mut String {
        &mut self.condition_name
    }
    fn default_index(&self) -> &i32 {
        &self.default_index
    }
    fn default_index_mut(&mut self) -> &mut i32 {
        &mut self.default_index
    }
    fn branches(&self) -> &Vec<PipelineExternalShaderConditionalBranch> {
        &self.branches
    }
    fn branches_mut(&mut self) -> &mut Vec<PipelineExternalShaderConditionalBranch> {
        &mut self.branches
    }
    fn is_global(&self) -> &bool {
        &self.is_global
    }
    fn is_global_mut(&mut self) -> &mut bool {
        &mut self.is_global
    }
}

pub static PIPELINEEXTERNALSHADERCONDITIONAL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PipelineExternalShaderConditional",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PipelineExternalShaderConditional as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ConditionName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(PipelineExternalShaderConditional, condition_name),
            },
            FieldInfoData {
                name: "DefaultIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PipelineExternalShaderConditional, default_index),
            },
            FieldInfoData {
                name: "Branches",
                flags: MemberInfoFlags::new(144),
                field_type: "PipelineExternalShaderConditionalBranch-Array",
                rust_offset: offset_of!(PipelineExternalShaderConditional, branches),
            },
            FieldInfoData {
                name: "IsGlobal",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PipelineExternalShaderConditional, is_global),
            },
        ],
    }),
    array_type: Some(PIPELINEEXTERNALSHADERCONDITIONAL_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PipelineExternalShaderConditional {
    fn type_info(&self) -> &'static TypeInfo {
        PIPELINEEXTERNALSHADERCONDITIONAL_TYPE_INFO
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


pub static PIPELINEEXTERNALSHADERCONDITIONAL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PipelineExternalShaderConditional-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("PipelineExternalShaderConditional"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PipelineExternalShaderConditionalBranch {
    pub branch_name: String,
    pub condition_name: String,
    pub index: i32,
    pub is_global: bool,
}

pub trait PipelineExternalShaderConditionalBranchTrait: TypeObject {
    fn branch_name(&self) -> &String;
    fn branch_name_mut(&mut self) -> &mut String;
    fn condition_name(&self) -> &String;
    fn condition_name_mut(&mut self) -> &mut String;
    fn index(&self) -> &i32;
    fn index_mut(&mut self) -> &mut i32;
    fn is_global(&self) -> &bool;
    fn is_global_mut(&mut self) -> &mut bool;
}

impl PipelineExternalShaderConditionalBranchTrait for PipelineExternalShaderConditionalBranch {
    fn branch_name(&self) -> &String {
        &self.branch_name
    }
    fn branch_name_mut(&mut self) -> &mut String {
        &mut self.branch_name
    }
    fn condition_name(&self) -> &String {
        &self.condition_name
    }
    fn condition_name_mut(&mut self) -> &mut String {
        &mut self.condition_name
    }
    fn index(&self) -> &i32 {
        &self.index
    }
    fn index_mut(&mut self) -> &mut i32 {
        &mut self.index
    }
    fn is_global(&self) -> &bool {
        &self.is_global
    }
    fn is_global_mut(&mut self) -> &mut bool {
        &mut self.is_global
    }
}

pub static PIPELINEEXTERNALSHADERCONDITIONALBRANCH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PipelineExternalShaderConditionalBranch",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PipelineExternalShaderConditionalBranch as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "BranchName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(PipelineExternalShaderConditionalBranch, branch_name),
            },
            FieldInfoData {
                name: "ConditionName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(PipelineExternalShaderConditionalBranch, condition_name),
            },
            FieldInfoData {
                name: "Index",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PipelineExternalShaderConditionalBranch, index),
            },
            FieldInfoData {
                name: "IsGlobal",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PipelineExternalShaderConditionalBranch, is_global),
            },
        ],
    }),
    array_type: Some(PIPELINEEXTERNALSHADERCONDITIONALBRANCH_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PipelineExternalShaderConditionalBranch {
    fn type_info(&self) -> &'static TypeInfo {
        PIPELINEEXTERNALSHADERCONDITIONALBRANCH_TYPE_INFO
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


pub static PIPELINEEXTERNALSHADERCONDITIONALBRANCH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PipelineExternalShaderConditionalBranch-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("PipelineExternalShaderConditionalBranch"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ExternalShaderConditionalFilterAsset {
    pub _glacier_base: super::core::Asset,
    pub inludees: Vec<ExternalShaderConditionalSelection>,
}

pub trait ExternalShaderConditionalFilterAssetTrait: super::core::AssetTrait {
    fn inludees(&self) -> &Vec<ExternalShaderConditionalSelection>;
    fn inludees_mut(&mut self) -> &mut Vec<ExternalShaderConditionalSelection>;
}

impl ExternalShaderConditionalFilterAssetTrait for ExternalShaderConditionalFilterAsset {
    fn inludees(&self) -> &Vec<ExternalShaderConditionalSelection> {
        &self.inludees
    }
    fn inludees_mut(&mut self) -> &mut Vec<ExternalShaderConditionalSelection> {
        &mut self.inludees
    }
}

impl super::core::AssetTrait for ExternalShaderConditionalFilterAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for ExternalShaderConditionalFilterAsset {
}

pub static EXTERNALSHADERCONDITIONALFILTERASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExternalShaderConditionalFilterAsset",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ExternalShaderConditionalFilterAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Inludees",
                flags: MemberInfoFlags::new(144),
                field_type: "ExternalShaderConditionalSelection-Array",
                rust_offset: offset_of!(ExternalShaderConditionalFilterAsset, inludees),
            },
        ],
    }),
    array_type: Some(EXTERNALSHADERCONDITIONALFILTERASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ExternalShaderConditionalFilterAsset {
    fn type_info(&self) -> &'static TypeInfo {
        EXTERNALSHADERCONDITIONALFILTERASSET_TYPE_INFO
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


pub static EXTERNALSHADERCONDITIONALFILTERASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExternalShaderConditionalFilterAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ExternalShaderConditionalFilterAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ExternalShaderConditionalSelection {
    pub condition_name: String,
    pub branch_name: String,
    pub external_shader_conditional: Option<Arc<Mutex<dyn ExternalShaderConditionalAssetTrait>>>,
}

pub trait ExternalShaderConditionalSelectionTrait: TypeObject {
    fn condition_name(&self) -> &String;
    fn condition_name_mut(&mut self) -> &mut String;
    fn branch_name(&self) -> &String;
    fn branch_name_mut(&mut self) -> &mut String;
    fn external_shader_conditional(&self) -> &Option<Arc<Mutex<dyn ExternalShaderConditionalAssetTrait>>>;
    fn external_shader_conditional_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ExternalShaderConditionalAssetTrait>>>;
}

impl ExternalShaderConditionalSelectionTrait for ExternalShaderConditionalSelection {
    fn condition_name(&self) -> &String {
        &self.condition_name
    }
    fn condition_name_mut(&mut self) -> &mut String {
        &mut self.condition_name
    }
    fn branch_name(&self) -> &String {
        &self.branch_name
    }
    fn branch_name_mut(&mut self) -> &mut String {
        &mut self.branch_name
    }
    fn external_shader_conditional(&self) -> &Option<Arc<Mutex<dyn ExternalShaderConditionalAssetTrait>>> {
        &self.external_shader_conditional
    }
    fn external_shader_conditional_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ExternalShaderConditionalAssetTrait>>> {
        &mut self.external_shader_conditional
    }
}

pub static EXTERNALSHADERCONDITIONALSELECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExternalShaderConditionalSelection",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ExternalShaderConditionalSelection as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ConditionName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ExternalShaderConditionalSelection, condition_name),
            },
            FieldInfoData {
                name: "BranchName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ExternalShaderConditionalSelection, branch_name),
            },
            FieldInfoData {
                name: "ExternalShaderConditional",
                flags: MemberInfoFlags::new(0),
                field_type: "ExternalShaderConditionalAsset",
                rust_offset: offset_of!(ExternalShaderConditionalSelection, external_shader_conditional),
            },
        ],
    }),
    array_type: Some(EXTERNALSHADERCONDITIONALSELECTION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ExternalShaderConditionalSelection {
    fn type_info(&self) -> &'static TypeInfo {
        EXTERNALSHADERCONDITIONALSELECTION_TYPE_INFO
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


pub static EXTERNALSHADERCONDITIONALSELECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExternalShaderConditionalSelection-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ExternalShaderConditionalSelection"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ExternalShaderBooleanDescription {
    pub _glacier_base: ExternalShaderConditionalAsset,
}

pub trait ExternalShaderBooleanDescriptionTrait: ExternalShaderConditionalAssetTrait {
}

impl ExternalShaderBooleanDescriptionTrait for ExternalShaderBooleanDescription {
}

impl ExternalShaderConditionalAssetTrait for ExternalShaderBooleanDescription {
    fn condition_name(&self) -> &String {
        self._glacier_base.condition_name()
    }
    fn condition_name_mut(&mut self) -> &mut String {
        self._glacier_base.condition_name_mut()
    }
    fn description(&self) -> &String {
        self._glacier_base.description()
    }
    fn description_mut(&mut self) -> &mut String {
        self._glacier_base.description_mut()
    }
    fn branches(&self) -> &Vec<String> {
        self._glacier_base.branches()
    }
    fn branches_mut(&mut self) -> &mut Vec<String> {
        self._glacier_base.branches_mut()
    }
    fn default_value(&self) -> &String {
        self._glacier_base.default_value()
    }
    fn default_value_mut(&mut self) -> &mut String {
        self._glacier_base.default_value_mut()
    }
    fn is_global(&self) -> &bool {
        self._glacier_base.is_global()
    }
    fn is_global_mut(&mut self) -> &mut bool {
        self._glacier_base.is_global_mut()
    }
}

impl super::core::AssetTrait for ExternalShaderBooleanDescription {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for ExternalShaderBooleanDescription {
}

pub static EXTERNALSHADERBOOLEANDESCRIPTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExternalShaderBooleanDescription",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EXTERNALSHADERCONDITIONALASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ExternalShaderBooleanDescription as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(EXTERNALSHADERBOOLEANDESCRIPTION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ExternalShaderBooleanDescription {
    fn type_info(&self) -> &'static TypeInfo {
        EXTERNALSHADERBOOLEANDESCRIPTION_TYPE_INFO
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


pub static EXTERNALSHADERBOOLEANDESCRIPTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExternalShaderBooleanDescription-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ExternalShaderBooleanDescription"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ExternalShaderSwitchDescription {
    pub _glacier_base: ExternalShaderConditionalAsset,
}

pub trait ExternalShaderSwitchDescriptionTrait: ExternalShaderConditionalAssetTrait {
}

impl ExternalShaderSwitchDescriptionTrait for ExternalShaderSwitchDescription {
}

impl ExternalShaderConditionalAssetTrait for ExternalShaderSwitchDescription {
    fn condition_name(&self) -> &String {
        self._glacier_base.condition_name()
    }
    fn condition_name_mut(&mut self) -> &mut String {
        self._glacier_base.condition_name_mut()
    }
    fn description(&self) -> &String {
        self._glacier_base.description()
    }
    fn description_mut(&mut self) -> &mut String {
        self._glacier_base.description_mut()
    }
    fn branches(&self) -> &Vec<String> {
        self._glacier_base.branches()
    }
    fn branches_mut(&mut self) -> &mut Vec<String> {
        self._glacier_base.branches_mut()
    }
    fn default_value(&self) -> &String {
        self._glacier_base.default_value()
    }
    fn default_value_mut(&mut self) -> &mut String {
        self._glacier_base.default_value_mut()
    }
    fn is_global(&self) -> &bool {
        self._glacier_base.is_global()
    }
    fn is_global_mut(&mut self) -> &mut bool {
        self._glacier_base.is_global_mut()
    }
}

impl super::core::AssetTrait for ExternalShaderSwitchDescription {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for ExternalShaderSwitchDescription {
}

pub static EXTERNALSHADERSWITCHDESCRIPTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExternalShaderSwitchDescription",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EXTERNALSHADERCONDITIONALASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ExternalShaderSwitchDescription as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(EXTERNALSHADERSWITCHDESCRIPTION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ExternalShaderSwitchDescription {
    fn type_info(&self) -> &'static TypeInfo {
        EXTERNALSHADERSWITCHDESCRIPTION_TYPE_INFO
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


pub static EXTERNALSHADERSWITCHDESCRIPTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExternalShaderSwitchDescription-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ExternalShaderSwitchDescription"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ExternalShaderConditionalAsset {
    pub _glacier_base: super::core::Asset,
    pub condition_name: String,
    pub description: String,
    pub branches: Vec<String>,
    pub default_value: String,
    pub is_global: bool,
}

pub trait ExternalShaderConditionalAssetTrait: super::core::AssetTrait {
    fn condition_name(&self) -> &String;
    fn condition_name_mut(&mut self) -> &mut String;
    fn description(&self) -> &String;
    fn description_mut(&mut self) -> &mut String;
    fn branches(&self) -> &Vec<String>;
    fn branches_mut(&mut self) -> &mut Vec<String>;
    fn default_value(&self) -> &String;
    fn default_value_mut(&mut self) -> &mut String;
    fn is_global(&self) -> &bool;
    fn is_global_mut(&mut self) -> &mut bool;
}

impl ExternalShaderConditionalAssetTrait for ExternalShaderConditionalAsset {
    fn condition_name(&self) -> &String {
        &self.condition_name
    }
    fn condition_name_mut(&mut self) -> &mut String {
        &mut self.condition_name
    }
    fn description(&self) -> &String {
        &self.description
    }
    fn description_mut(&mut self) -> &mut String {
        &mut self.description
    }
    fn branches(&self) -> &Vec<String> {
        &self.branches
    }
    fn branches_mut(&mut self) -> &mut Vec<String> {
        &mut self.branches
    }
    fn default_value(&self) -> &String {
        &self.default_value
    }
    fn default_value_mut(&mut self) -> &mut String {
        &mut self.default_value
    }
    fn is_global(&self) -> &bool {
        &self.is_global
    }
    fn is_global_mut(&mut self) -> &mut bool {
        &mut self.is_global
    }
}

impl super::core::AssetTrait for ExternalShaderConditionalAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for ExternalShaderConditionalAsset {
}

pub static EXTERNALSHADERCONDITIONALASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExternalShaderConditionalAsset",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ExternalShaderConditionalAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ConditionName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ExternalShaderConditionalAsset, condition_name),
            },
            FieldInfoData {
                name: "Description",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ExternalShaderConditionalAsset, description),
            },
            FieldInfoData {
                name: "Branches",
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(ExternalShaderConditionalAsset, branches),
            },
            FieldInfoData {
                name: "DefaultValue",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ExternalShaderConditionalAsset, default_value),
            },
            FieldInfoData {
                name: "IsGlobal",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ExternalShaderConditionalAsset, is_global),
            },
        ],
    }),
    array_type: Some(EXTERNALSHADERCONDITIONALASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ExternalShaderConditionalAsset {
    fn type_info(&self) -> &'static TypeInfo {
        EXTERNALSHADERCONDITIONALASSET_TYPE_INFO
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


pub static EXTERNALSHADERCONDITIONALASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExternalShaderConditionalAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ExternalShaderConditionalAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ShadowCacheDepthBias {
    #[default]
    ShadowCacheDepthBias_Lowest = 0,
    ShadowCacheDepthBias_Lower = 1,
    ShadowCacheDepthBias_Medium = 2,
    ShadowCacheDepthBias_Higher = 3,
    ShadowCacheDepthBias_Highest = 4,
}

pub static SHADOWCACHEDEPTHBIAS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowCacheDepthBias",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(SHADOWCACHEDEPTHBIAS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShadowCacheDepthBias {
    fn type_info(&self) -> &'static TypeInfo {
        SHADOWCACHEDEPTHBIAS_TYPE_INFO
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


pub static SHADOWCACHEDEPTHBIAS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowCacheDepthBias-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ShadowCacheDepthBias"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ShadowCacheMode {
    #[default]
    ShadowCacheMode_Static = 0,
    ShadowCacheMode_Dynamic = 1,
    ShadowCacheMode_Baked = 2,
    ShadowCacheMode_DynamicProd = 3,
    ShadowCacheMode_BakeEventTriggered = 4,
}

pub static SHADOWCACHEMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowCacheMode",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(SHADOWCACHEMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShadowCacheMode {
    fn type_info(&self) -> &'static TypeInfo {
        SHADOWCACHEMODE_TYPE_INFO
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


pub static SHADOWCACHEMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowCacheMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("ShadowCacheMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DecalAtlasTile {
    pub tile_index_x: f32,
    pub tile_index_y: f32,
    pub tile_count_x: f32,
    pub tile_count_y: f32,
    pub flip_x: bool,
    pub flip_y: bool,
}

pub trait DecalAtlasTileTrait: TypeObject {
    fn tile_index_x(&self) -> &f32;
    fn tile_index_x_mut(&mut self) -> &mut f32;
    fn tile_index_y(&self) -> &f32;
    fn tile_index_y_mut(&mut self) -> &mut f32;
    fn tile_count_x(&self) -> &f32;
    fn tile_count_x_mut(&mut self) -> &mut f32;
    fn tile_count_y(&self) -> &f32;
    fn tile_count_y_mut(&mut self) -> &mut f32;
    fn flip_x(&self) -> &bool;
    fn flip_x_mut(&mut self) -> &mut bool;
    fn flip_y(&self) -> &bool;
    fn flip_y_mut(&mut self) -> &mut bool;
}

impl DecalAtlasTileTrait for DecalAtlasTile {
    fn tile_index_x(&self) -> &f32 {
        &self.tile_index_x
    }
    fn tile_index_x_mut(&mut self) -> &mut f32 {
        &mut self.tile_index_x
    }
    fn tile_index_y(&self) -> &f32 {
        &self.tile_index_y
    }
    fn tile_index_y_mut(&mut self) -> &mut f32 {
        &mut self.tile_index_y
    }
    fn tile_count_x(&self) -> &f32 {
        &self.tile_count_x
    }
    fn tile_count_x_mut(&mut self) -> &mut f32 {
        &mut self.tile_count_x
    }
    fn tile_count_y(&self) -> &f32 {
        &self.tile_count_y
    }
    fn tile_count_y_mut(&mut self) -> &mut f32 {
        &mut self.tile_count_y
    }
    fn flip_x(&self) -> &bool {
        &self.flip_x
    }
    fn flip_x_mut(&mut self) -> &mut bool {
        &mut self.flip_x
    }
    fn flip_y(&self) -> &bool {
        &self.flip_y
    }
    fn flip_y_mut(&mut self) -> &mut bool {
        &mut self.flip_y
    }
}

pub static DECALATLASTILE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalAtlasTile",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DecalAtlasTile as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TileIndexX",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DecalAtlasTile, tile_index_x),
            },
            FieldInfoData {
                name: "TileIndexY",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DecalAtlasTile, tile_index_y),
            },
            FieldInfoData {
                name: "TileCountX",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DecalAtlasTile, tile_count_x),
            },
            FieldInfoData {
                name: "TileCountY",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DecalAtlasTile, tile_count_y),
            },
            FieldInfoData {
                name: "FlipX",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DecalAtlasTile, flip_x),
            },
            FieldInfoData {
                name: "FlipY",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DecalAtlasTile, flip_y),
            },
        ],
    }),
    array_type: Some(DECALATLASTILE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for DecalAtlasTile {
    fn type_info(&self) -> &'static TypeInfo {
        DECALATLASTILE_TYPE_INFO
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


pub static DECALATLASTILE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalAtlasTile-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("DecalAtlasTile"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DecalTemplateBaseAsset {
    pub _glacier_base: super::core::Asset,
}

pub trait DecalTemplateBaseAssetTrait: super::core::AssetTrait {
}

impl DecalTemplateBaseAssetTrait for DecalTemplateBaseAsset {
}

impl super::core::AssetTrait for DecalTemplateBaseAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for DecalTemplateBaseAsset {
}

pub static DECALTEMPLATEBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalTemplateBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DecalTemplateBaseAsset as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DECALTEMPLATEBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DecalTemplateBaseAsset {
    fn type_info(&self) -> &'static TypeInfo {
        DECALTEMPLATEBASEASSET_TYPE_INFO
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


pub static DECALTEMPLATEBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalTemplateBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("DecalTemplateBaseAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum DecalType {
    #[default]
    DecalType_VolumeDecal = 0,
    DecalType_ProjectedMeshDecal = 1,
    DecalType_QuadDecal = 2,
    DecalType_Count = 3,
}

pub static DECALTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalType",
    flags: MemberInfoFlags::new(49429),
    module: "RenderBase",
    data: TypeInfoData::Enum,
    array_type: Some(DECALTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DecalType {
    fn type_info(&self) -> &'static TypeInfo {
        DECALTYPE_TYPE_INFO
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


pub static DECALTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalType-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("DecalType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CullIdDynamicState {
}

pub trait CullIdDynamicStateTrait: TypeObject {
}

impl CullIdDynamicStateTrait for CullIdDynamicState {
}

pub static CULLIDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CullIdDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CullIdDynamicState as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CULLIDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CullIdDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        CULLIDDYNAMICSTATE_TYPE_INFO
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


pub static CULLIDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CullIdDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("CullIdDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CullIdStaticState {
    pub affect_child_views: bool,
    pub field_flag_changed0: u8,
}

pub trait CullIdStaticStateTrait: TypeObject {
    fn affect_child_views(&self) -> &bool;
    fn affect_child_views_mut(&mut self) -> &mut bool;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl CullIdStaticStateTrait for CullIdStaticState {
    fn affect_child_views(&self) -> &bool {
        &self.affect_child_views
    }
    fn affect_child_views_mut(&mut self) -> &mut bool {
        &mut self.affect_child_views
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static CULLIDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CullIdStaticState",
    flags: MemberInfoFlags::new(73),
    module: "RenderBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CullIdStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "AffectChildViews",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CullIdStaticState, affect_child_views),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(CullIdStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(CULLIDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CullIdStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        CULLIDSTATICSTATE_TYPE_INFO
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


pub static CULLIDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CullIdStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("CullIdStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct OutlineColorsComponentData {
    pub _glacier_base: super::core::DataContainer,
}

pub trait OutlineColorsComponentDataTrait: super::core::DataContainerTrait {
}

impl OutlineColorsComponentDataTrait for OutlineColorsComponentData {
}

impl super::core::DataContainerTrait for OutlineColorsComponentData {
}

pub static OUTLINECOLORSCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OutlineColorsComponentData",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OutlineColorsComponentData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(OUTLINECOLORSCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OutlineColorsComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        OUTLINECOLORSCOMPONENTDATA_TYPE_INFO
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


pub static OUTLINECOLORSCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OutlineColorsComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("OutlineColorsComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct OccluderRealPlaneEntityData {
    pub _glacier_base: super::core::DataContainer,
}

pub trait OccluderRealPlaneEntityDataTrait: super::core::DataContainerTrait {
}

impl OccluderRealPlaneEntityDataTrait for OccluderRealPlaneEntityData {
}

impl super::core::DataContainerTrait for OccluderRealPlaneEntityData {
}

pub static OCCLUDERREALPLANEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderRealPlaneEntityData",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OccluderRealPlaneEntityData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(OCCLUDERREALPLANEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OccluderRealPlaneEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        OCCLUDERREALPLANEENTITYDATA_TYPE_INFO
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


pub static OCCLUDERREALPLANEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderRealPlaneEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("OccluderRealPlaneEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FogVolumeEntityData {
    pub _glacier_base: super::core::DataContainer,
}

pub trait FogVolumeEntityDataTrait: super::core::DataContainerTrait {
}

impl FogVolumeEntityDataTrait for FogVolumeEntityData {
}

impl super::core::DataContainerTrait for FogVolumeEntityData {
}

pub static FOGVOLUMEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FogVolumeEntityData",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FogVolumeEntityData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(FOGVOLUMEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FogVolumeEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        FOGVOLUMEENTITYDATA_TYPE_INFO
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


pub static FOGVOLUMEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FogVolumeEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("FogVolumeEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LayerTextureConfigAsset {
    pub _glacier_base: super::core::DataContainer,
}

pub trait LayerTextureConfigAssetTrait: super::core::DataContainerTrait {
}

impl LayerTextureConfigAssetTrait for LayerTextureConfigAsset {
}

impl super::core::DataContainerTrait for LayerTextureConfigAsset {
}

pub static LAYERTEXTURECONFIGASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LayerTextureConfigAsset",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LayerTextureConfigAsset as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LAYERTEXTURECONFIGASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LayerTextureConfigAsset {
    fn type_info(&self) -> &'static TypeInfo {
        LAYERTEXTURECONFIGASSET_TYPE_INFO
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


pub static LAYERTEXTURECONFIGASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LayerTextureConfigAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("LayerTextureConfigAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LayerTextureConfig {
    pub _glacier_base: super::core::DataContainer,
}

pub trait LayerTextureConfigTrait: super::core::DataContainerTrait {
}

impl LayerTextureConfigTrait for LayerTextureConfig {
}

impl super::core::DataContainerTrait for LayerTextureConfig {
}

pub static LAYERTEXTURECONFIG_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LayerTextureConfig",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LayerTextureConfig as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LAYERTEXTURECONFIG_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LayerTextureConfig {
    fn type_info(&self) -> &'static TypeInfo {
        LAYERTEXTURECONFIG_TYPE_INFO
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


pub static LAYERTEXTURECONFIG_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LayerTextureConfig-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("LayerTextureConfig"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FScriptVerletChainEntityData {
    pub _glacier_base: super::core::DataContainer,
}

pub trait FScriptVerletChainEntityDataTrait: super::core::DataContainerTrait {
}

impl FScriptVerletChainEntityDataTrait for FScriptVerletChainEntityData {
}

impl super::core::DataContainerTrait for FScriptVerletChainEntityData {
}

pub static FSCRIPT_VERLETCHAINENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FScript_VerletChainEntityData",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FScriptVerletChainEntityData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(FSCRIPT_VERLETCHAINENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FScriptVerletChainEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        FSCRIPT_VERLETCHAINENTITYDATA_TYPE_INFO
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


pub static FSCRIPT_VERLETCHAINENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FScript_VerletChainEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("FScript_VerletChainEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FScriptProceduralBoneGlobalsEntityData {
    pub _glacier_base: super::core::DataContainer,
}

pub trait FScriptProceduralBoneGlobalsEntityDataTrait: super::core::DataContainerTrait {
}

impl FScriptProceduralBoneGlobalsEntityDataTrait for FScriptProceduralBoneGlobalsEntityData {
}

impl super::core::DataContainerTrait for FScriptProceduralBoneGlobalsEntityData {
}

pub static FSCRIPT_PROCEDURALBONEGLOBALSENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FScript_ProceduralBoneGlobalsEntityData",
    flags: MemberInfoFlags::new(101),
    module: "RenderBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FScriptProceduralBoneGlobalsEntityData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(FSCRIPT_PROCEDURALBONEGLOBALSENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FScriptProceduralBoneGlobalsEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        FSCRIPT_PROCEDURALBONEGLOBALSENTITYDATA_TYPE_INFO
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


pub static FSCRIPT_PROCEDURALBONEGLOBALSENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FScript_ProceduralBoneGlobalsEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "RenderBase",
    data: TypeInfoData::Array("FScript_ProceduralBoneGlobalsEntityData"),
    array_type: None,
    alignment: 8,
};


