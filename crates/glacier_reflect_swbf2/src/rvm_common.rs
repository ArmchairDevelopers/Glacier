use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct RvmBuildSettings {
    pub _glacier_base: super::core::SystemSettings,
    pub built_backends: Vec<Option<Arc<Mutex<dyn RvmBackendConfigTrait>>>>,
}

pub trait RvmBuildSettingsTrait: super::core::SystemSettingsTrait {
    fn built_backends(&self) -> &Vec<Option<Arc<Mutex<dyn RvmBackendConfigTrait>>>>;
}

impl RvmBuildSettingsTrait for RvmBuildSettings {
    fn built_backends(&self) -> &Vec<Option<Arc<Mutex<dyn RvmBackendConfigTrait>>>> {
        &self.built_backends
    }
}

impl super::core::SystemSettingsTrait for RvmBuildSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
}

impl super::core::DataContainerTrait for RvmBuildSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static RVMBUILDSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmBuildSettings",
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmBuildSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "BuiltBackends",
                flags: MemberInfoFlags::new(144),
                field_type: "RvmBackendConfig-Array",
                rust_offset: offset_of!(RvmBuildSettings, built_backends),
            },
        ],
    }),
    array_type: Some(RVMBUILDSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmBuildSettings {
    fn type_info(&self) -> &'static TypeInfo {
        RVMBUILDSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RVMBUILDSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmBuildSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("RvmBuildSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RvmBackendConfig {
    pub _glacier_base: super::core::Asset,
}

pub trait RvmBackendConfigTrait: super::core::AssetTrait {
}

impl RvmBackendConfigTrait for RvmBackendConfig {
}

impl super::core::AssetTrait for RvmBackendConfig {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for RvmBackendConfig {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static RVMBACKENDCONFIG_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmBackendConfig",
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmBackendConfig as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(RVMBACKENDCONFIG_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmBackendConfig {
    fn type_info(&self) -> &'static TypeInfo {
        RVMBACKENDCONFIG_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RVMBACKENDCONFIG_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmBackendConfig-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("RvmBackendConfig"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LodFadeInstructionFactory {
    pub _glacier_base: RvmInstructionFactoryBase,
}

pub trait LodFadeInstructionFactoryTrait: RvmInstructionFactoryBaseTrait {
}

impl LodFadeInstructionFactoryTrait for LodFadeInstructionFactory {
}

impl RvmInstructionFactoryBaseTrait for LodFadeInstructionFactory {
}

pub static LODFADEINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LodFadeInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LodFadeInstructionFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LODFADEINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LodFadeInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        LODFADEINSTRUCTIONFACTORY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LODFADEINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LodFadeInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("LodFadeInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_LodFadeInstructionData {
}

pub trait RvmSerializedDb_ns_LodFadeInstructionDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_LodFadeInstructionDataTrait for RvmSerializedDb_ns_LodFadeInstructionData {
}

pub static RVMSERIALIZEDDB_NS_LODFADEINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_LodFadeInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_LodFadeInstructionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_LODFADEINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_LodFadeInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_LODFADEINSTRUCTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct PackLightMapWeightIntoInstanceInstructionFactory {
    pub _glacier_base: RvmInstructionFactoryBase,
}

pub trait PackLightMapWeightIntoInstanceInstructionFactoryTrait: RvmInstructionFactoryBaseTrait {
}

impl PackLightMapWeightIntoInstanceInstructionFactoryTrait for PackLightMapWeightIntoInstanceInstructionFactory {
}

impl RvmInstructionFactoryBaseTrait for PackLightMapWeightIntoInstanceInstructionFactory {
}

pub static PACKLIGHTMAPWEIGHTINTOINSTANCEINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PackLightMapWeightIntoInstanceInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PackLightMapWeightIntoInstanceInstructionFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PACKLIGHTMAPWEIGHTINTOINSTANCEINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PackLightMapWeightIntoInstanceInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        PACKLIGHTMAPWEIGHTINTOINSTANCEINSTRUCTIONFACTORY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PACKLIGHTMAPWEIGHTINTOINSTANCEINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PackLightMapWeightIntoInstanceInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("PackLightMapWeightIntoInstanceInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SliceCountInstructionFactory {
    pub _glacier_base: RvmInstructionFactoryBase,
}

pub trait SliceCountInstructionFactoryTrait: RvmInstructionFactoryBaseTrait {
}

impl SliceCountInstructionFactoryTrait for SliceCountInstructionFactory {
}

impl RvmInstructionFactoryBaseTrait for SliceCountInstructionFactory {
}

pub static SLICECOUNTINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SliceCountInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SliceCountInstructionFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SLICECOUNTINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SliceCountInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        SLICECOUNTINSTRUCTIONFACTORY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SLICECOUNTINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SliceCountInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("SliceCountInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TessellationParametersInstructionFactory {
    pub _glacier_base: RvmInstructionFactoryBase,
}

pub trait TessellationParametersInstructionFactoryTrait: RvmInstructionFactoryBaseTrait {
}

impl TessellationParametersInstructionFactoryTrait for TessellationParametersInstructionFactory {
}

impl RvmInstructionFactoryBaseTrait for TessellationParametersInstructionFactory {
}

pub static TESSELLATIONPARAMETERSINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TessellationParametersInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TessellationParametersInstructionFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TESSELLATIONPARAMETERSINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TessellationParametersInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        TESSELLATIONPARAMETERSINSTRUCTIONFACTORY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TESSELLATIONPARAMETERSINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TessellationParametersInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("TessellationParametersInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VectorSubtractInstructionFactory {
    pub _glacier_base: RvmInstructionFactoryBase,
}

pub trait VectorSubtractInstructionFactoryTrait: RvmInstructionFactoryBaseTrait {
}

impl VectorSubtractInstructionFactoryTrait for VectorSubtractInstructionFactory {
}

impl RvmInstructionFactoryBaseTrait for VectorSubtractInstructionFactory {
}

pub static VECTORSUBTRACTINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VectorSubtractInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VectorSubtractInstructionFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(VECTORSUBTRACTINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VectorSubtractInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        VECTORSUBTRACTINSTRUCTIONFACTORY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VECTORSUBTRACTINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VectorSubtractInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("VectorSubtractInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct OffsetTranslationInMatrixInstructionFactory {
    pub _glacier_base: RvmInstructionFactoryBase,
}

pub trait OffsetTranslationInMatrixInstructionFactoryTrait: RvmInstructionFactoryBaseTrait {
}

impl OffsetTranslationInMatrixInstructionFactoryTrait for OffsetTranslationInMatrixInstructionFactory {
}

impl RvmInstructionFactoryBaseTrait for OffsetTranslationInMatrixInstructionFactory {
}

pub static OFFSETTRANSLATIONINMATRIXINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OffsetTranslationInMatrixInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OffsetTranslationInMatrixInstructionFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(OFFSETTRANSLATIONINMATRIXINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OffsetTranslationInMatrixInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        OFFSETTRANSLATIONINMATRIXINSTRUCTIONFACTORY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static OFFSETTRANSLATIONINMATRIXINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OffsetTranslationInMatrixInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("OffsetTranslationInMatrixInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CpuToGpuMatrixInstructionFactory {
    pub _glacier_base: RvmInstructionFactoryBase,
}

pub trait CpuToGpuMatrixInstructionFactoryTrait: RvmInstructionFactoryBaseTrait {
}

impl CpuToGpuMatrixInstructionFactoryTrait for CpuToGpuMatrixInstructionFactory {
}

impl RvmInstructionFactoryBaseTrait for CpuToGpuMatrixInstructionFactory {
}

pub static CPUTOGPUMATRIXINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CpuToGpuMatrixInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CpuToGpuMatrixInstructionFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CPUTOGPUMATRIXINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CpuToGpuMatrixInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        CPUTOGPUMATRIXINSTRUCTIONFACTORY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CPUTOGPUMATRIXINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CpuToGpuMatrixInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("CpuToGpuMatrixInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FloatToVecInstructionFactory {
    pub _glacier_base: RvmInstructionFactoryBase,
}

pub trait FloatToVecInstructionFactoryTrait: RvmInstructionFactoryBaseTrait {
}

impl FloatToVecInstructionFactoryTrait for FloatToVecInstructionFactory {
}

impl RvmInstructionFactoryBaseTrait for FloatToVecInstructionFactory {
}

pub static FLOATTOVECINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatToVecInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FloatToVecInstructionFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(FLOATTOVECINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FloatToVecInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        FLOATTOVECINSTRUCTIONFACTORY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FLOATTOVECINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatToVecInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("FloatToVecInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_PackLightMapWeightIntoInstanceInstructionData {
}

pub trait RvmSerializedDb_ns_PackLightMapWeightIntoInstanceInstructionDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_PackLightMapWeightIntoInstanceInstructionDataTrait for RvmSerializedDb_ns_PackLightMapWeightIntoInstanceInstructionData {
}

pub static RVMSERIALIZEDDB_NS_PACKLIGHTMAPWEIGHTINTOINSTANCEINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_PackLightMapWeightIntoInstanceInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_PackLightMapWeightIntoInstanceInstructionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_PACKLIGHTMAPWEIGHTINTOINSTANCEINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_PackLightMapWeightIntoInstanceInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_PACKLIGHTMAPWEIGHTINTOINSTANCEINSTRUCTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_SliceCountInstructionData {
}

pub trait RvmSerializedDb_ns_SliceCountInstructionDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_SliceCountInstructionDataTrait for RvmSerializedDb_ns_SliceCountInstructionData {
}

pub static RVMSERIALIZEDDB_NS_SLICECOUNTINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_SliceCountInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_SliceCountInstructionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_SLICECOUNTINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_SliceCountInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_SLICECOUNTINSTRUCTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_TessellationParametersInstructionData {
}

pub trait RvmSerializedDb_ns_TessellationParametersInstructionDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_TessellationParametersInstructionDataTrait for RvmSerializedDb_ns_TessellationParametersInstructionData {
}

pub static RVMSERIALIZEDDB_NS_TESSELLATIONPARAMETERSINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_TessellationParametersInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_TessellationParametersInstructionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_TESSELLATIONPARAMETERSINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_TessellationParametersInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_TESSELLATIONPARAMETERSINSTRUCTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_VectorSubtractInstructionData {
}

pub trait RvmSerializedDb_ns_VectorSubtractInstructionDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_VectorSubtractInstructionDataTrait for RvmSerializedDb_ns_VectorSubtractInstructionData {
}

pub static RVMSERIALIZEDDB_NS_VECTORSUBTRACTINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_VectorSubtractInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_VectorSubtractInstructionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_VECTORSUBTRACTINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_VectorSubtractInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_VECTORSUBTRACTINSTRUCTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_OffsetTranslationInMatrixInstructionData {
}

pub trait RvmSerializedDb_ns_OffsetTranslationInMatrixInstructionDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_OffsetTranslationInMatrixInstructionDataTrait for RvmSerializedDb_ns_OffsetTranslationInMatrixInstructionData {
}

pub static RVMSERIALIZEDDB_NS_OFFSETTRANSLATIONINMATRIXINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_OffsetTranslationInMatrixInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_OffsetTranslationInMatrixInstructionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_OFFSETTRANSLATIONINMATRIXINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_OffsetTranslationInMatrixInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_OFFSETTRANSLATIONINMATRIXINSTRUCTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_CpuToGpuMatrixInstructionData {
}

pub trait RvmSerializedDb_ns_CpuToGpuMatrixInstructionDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_CpuToGpuMatrixInstructionDataTrait for RvmSerializedDb_ns_CpuToGpuMatrixInstructionData {
}

pub static RVMSERIALIZEDDB_NS_CPUTOGPUMATRIXINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_CpuToGpuMatrixInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_CpuToGpuMatrixInstructionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_CPUTOGPUMATRIXINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_CpuToGpuMatrixInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_CPUTOGPUMATRIXINSTRUCTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_FloatToVecInstructionData {
}

pub trait RvmSerializedDb_ns_FloatToVecInstructionDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_FloatToVecInstructionDataTrait for RvmSerializedDb_ns_FloatToVecInstructionData {
}

pub static RVMSERIALIZEDDB_NS_FLOATTOVECINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_FloatToVecInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_FloatToVecInstructionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_FLOATTOVECINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_FloatToVecInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_FLOATTOVECINSTRUCTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_LegacyVertexBufferConversionInstructionData {
}

pub trait RvmSerializedDb_ns_LegacyVertexBufferConversionInstructionDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_LegacyVertexBufferConversionInstructionDataTrait for RvmSerializedDb_ns_LegacyVertexBufferConversionInstructionData {
}

pub static RVMSERIALIZEDDB_NS_LEGACYVERTEXBUFFERCONVERSIONINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_LegacyVertexBufferConversionInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_LegacyVertexBufferConversionInstructionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_LEGACYVERTEXBUFFERCONVERSIONINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_LegacyVertexBufferConversionInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_LEGACYVERTEXBUFFERCONVERSIONINSTRUCTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_VertexStreamProcessInstructionData {
}

pub trait RvmSerializedDb_ns_VertexStreamProcessInstructionDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_VertexStreamProcessInstructionDataTrait for RvmSerializedDb_ns_VertexStreamProcessInstructionData {
}

pub static RVMSERIALIZEDDB_NS_VERTEXSTREAMPROCESSINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_VertexStreamProcessInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_VertexStreamProcessInstructionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_VERTEXSTREAMPROCESSINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_VertexStreamProcessInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_VERTEXSTREAMPROCESSINSTRUCTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_ViewStateInstructionData {
}

pub trait RvmSerializedDb_ns_ViewStateInstructionDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_ViewStateInstructionDataTrait for RvmSerializedDb_ns_ViewStateInstructionData {
}

pub static RVMSERIALIZEDDB_NS_VIEWSTATEINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ViewStateInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_ViewStateInstructionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_VIEWSTATEINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_ViewStateInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_VIEWSTATEINSTRUCTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmCommonSettings {
    pub _glacier_base: super::core::DataContainer,
    pub on_demand_building_enable: bool,
}

pub trait RvmCommonSettingsTrait: super::core::DataContainerTrait {
    fn on_demand_building_enable(&self) -> &bool;
}

impl RvmCommonSettingsTrait for RvmCommonSettings {
    fn on_demand_building_enable(&self) -> &bool {
        &self.on_demand_building_enable
    }
}

impl super::core::DataContainerTrait for RvmCommonSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static RVMCOMMONSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmCommonSettings",
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmCommonSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "OnDemandBuildingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RvmCommonSettings, on_demand_building_enable),
            },
        ],
    }),
    array_type: Some(RVMCOMMONSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmCommonSettings {
    fn type_info(&self) -> &'static TypeInfo {
        RVMCOMMONSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RVMCOMMONSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmCommonSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("RvmCommonSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RvmPointer {
}

pub trait RvmPointerTrait: TypeObject {
}

impl RvmPointerTrait for RvmPointer {
}

pub static RVMPOINTER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmPointer",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmPointer as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMPOINTER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmPointer {
    fn type_info(&self) -> &'static TypeInfo {
        RVMPOINTER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct GpuMat4x3 {
}

pub trait GpuMat4x3Trait: TypeObject {
}

impl GpuMat4x3Trait for GpuMat4x3 {
}

pub static GPUMAT4X3_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GpuMat4x3",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GpuMat4x3 as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::GPUMAT4X3_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for GpuMat4x3 {
    fn type_info(&self) -> &'static TypeInfo {
        GPUMAT4X3_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct IVec4 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
}

pub trait IVec4Trait: TypeObject {
    fn x(&self) -> &i32;
    fn y(&self) -> &i32;
    fn z(&self) -> &i32;
    fn w(&self) -> &i32;
}

impl IVec4Trait for IVec4 {
    fn x(&self) -> &i32 {
        &self.x
    }
    fn y(&self) -> &i32 {
        &self.y
    }
    fn z(&self) -> &i32 {
        &self.z
    }
    fn w(&self) -> &i32 {
        &self.w
    }
}

pub static IVEC4_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IVec4",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IVec4 as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "x",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(IVec4, x),
            },
            FieldInfoData {
                name: "y",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(IVec4, y),
            },
            FieldInfoData {
                name: "z",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(IVec4, z),
            },
            FieldInfoData {
                name: "w",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(IVec4, w),
            },
        ],
    }),
    array_type: Some(super::core::IVEC4_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for IVec4 {
    fn type_info(&self) -> &'static TypeInfo {
        IVEC4_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct IVec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

pub trait IVec3Trait: TypeObject {
    fn x(&self) -> &i32;
    fn y(&self) -> &i32;
    fn z(&self) -> &i32;
}

impl IVec3Trait for IVec3 {
    fn x(&self) -> &i32 {
        &self.x
    }
    fn y(&self) -> &i32 {
        &self.y
    }
    fn z(&self) -> &i32 {
        &self.z
    }
}

pub static IVEC3_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IVec3",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IVec3 as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "x",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(IVec3, x),
            },
            FieldInfoData {
                name: "y",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(IVec3, y),
            },
            FieldInfoData {
                name: "z",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(IVec3, z),
            },
        ],
    }),
    array_type: Some(super::core::IVEC3_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for IVec3 {
    fn type_info(&self) -> &'static TypeInfo {
        IVEC3_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct IVec2 {
    pub x: i32,
    pub y: i32,
}

pub trait IVec2Trait: TypeObject {
    fn x(&self) -> &i32;
    fn y(&self) -> &i32;
}

impl IVec2Trait for IVec2 {
    fn x(&self) -> &i32 {
        &self.x
    }
    fn y(&self) -> &i32 {
        &self.y
    }
}

pub static IVEC2_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IVec2",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IVec2 as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "x",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(IVec2, x),
            },
            FieldInfoData {
                name: "y",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(IVec2, y),
            },
        ],
    }),
    array_type: Some(super::core::IVEC2_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for IVec2 {
    fn type_info(&self) -> &'static TypeInfo {
        IVEC2_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct Half4 {
    pub x: Half,
    pub y: Half,
    pub z: Half,
    pub w: Half,
}

pub trait Half4Trait: TypeObject {
    fn x(&self) -> &Half;
    fn y(&self) -> &Half;
    fn z(&self) -> &Half;
    fn w(&self) -> &Half;
}

impl Half4Trait for Half4 {
    fn x(&self) -> &Half {
        &self.x
    }
    fn y(&self) -> &Half {
        &self.y
    }
    fn z(&self) -> &Half {
        &self.z
    }
    fn w(&self) -> &Half {
        &self.w
    }
}

pub static HALF4_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Half4",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Half4 as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "x",
                flags: MemberInfoFlags::new(0),
                field_type: "Half",
                rust_offset: offset_of!(Half4, x),
            },
            FieldInfoData {
                name: "y",
                flags: MemberInfoFlags::new(0),
                field_type: "Half",
                rust_offset: offset_of!(Half4, y),
            },
            FieldInfoData {
                name: "z",
                flags: MemberInfoFlags::new(0),
                field_type: "Half",
                rust_offset: offset_of!(Half4, z),
            },
            FieldInfoData {
                name: "w",
                flags: MemberInfoFlags::new(0),
                field_type: "Half",
                rust_offset: offset_of!(Half4, w),
            },
        ],
    }),
    array_type: Some(super::core::HALF4_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for Half4 {
    fn type_info(&self) -> &'static TypeInfo {
        HALF4_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct Half3 {
    pub x: Half,
    pub y: Half,
    pub z: Half,
}

pub trait Half3Trait: TypeObject {
    fn x(&self) -> &Half;
    fn y(&self) -> &Half;
    fn z(&self) -> &Half;
}

impl Half3Trait for Half3 {
    fn x(&self) -> &Half {
        &self.x
    }
    fn y(&self) -> &Half {
        &self.y
    }
    fn z(&self) -> &Half {
        &self.z
    }
}

pub static HALF3_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Half3",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Half3 as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "x",
                flags: MemberInfoFlags::new(0),
                field_type: "Half",
                rust_offset: offset_of!(Half3, x),
            },
            FieldInfoData {
                name: "y",
                flags: MemberInfoFlags::new(0),
                field_type: "Half",
                rust_offset: offset_of!(Half3, y),
            },
            FieldInfoData {
                name: "z",
                flags: MemberInfoFlags::new(0),
                field_type: "Half",
                rust_offset: offset_of!(Half3, z),
            },
        ],
    }),
    array_type: Some(super::core::HALF3_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for Half3 {
    fn type_info(&self) -> &'static TypeInfo {
        HALF3_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct Half2 {
    pub x: Half,
    pub y: Half,
}

pub trait Half2Trait: TypeObject {
    fn x(&self) -> &Half;
    fn y(&self) -> &Half;
}

impl Half2Trait for Half2 {
    fn x(&self) -> &Half {
        &self.x
    }
    fn y(&self) -> &Half {
        &self.y
    }
}

pub static HALF2_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Half2",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Half2 as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "x",
                flags: MemberInfoFlags::new(0),
                field_type: "Half",
                rust_offset: offset_of!(Half2, x),
            },
            FieldInfoData {
                name: "y",
                flags: MemberInfoFlags::new(0),
                field_type: "Half",
                rust_offset: offset_of!(Half2, y),
            },
        ],
    }),
    array_type: Some(super::core::HALF2_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for Half2 {
    fn type_info(&self) -> &'static TypeInfo {
        HALF2_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct Half {
}

pub trait HalfTrait: TypeObject {
}

impl HalfTrait for Half {
}

pub static HALF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Half",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Half as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::HALF_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for Half {
    fn type_info(&self) -> &'static TypeInfo {
        HALF_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmLegacyInstructions_ns_LegacyInstructionData {
}

pub trait RvmLegacyInstructions_ns_LegacyInstructionDataTrait: TypeObject {
}

impl RvmLegacyInstructions_ns_LegacyInstructionDataTrait for RvmLegacyInstructions_ns_LegacyInstructionData {
}

pub static RVMLEGACYINSTRUCTIONS_NS_LEGACYINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmLegacyInstructions_ns_LegacyInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmLegacyInstructions_ns_LegacyInstructionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMLEGACYINSTRUCTIONS_NS_LEGACYINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmLegacyInstructions_ns_LegacyInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMLEGACYINSTRUCTIONS_NS_LEGACYINSTRUCTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmLegacyInstructionFactory {
    pub _glacier_base: RvmInstructionFactoryBase,
}

pub trait RvmLegacyInstructionFactoryTrait: RvmInstructionFactoryBaseTrait {
}

impl RvmLegacyInstructionFactoryTrait for RvmLegacyInstructionFactory {
}

impl RvmInstructionFactoryBaseTrait for RvmLegacyInstructionFactory {
}

pub static RVMLEGACYINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmLegacyInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmLegacyInstructionFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(RVMLEGACYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmLegacyInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        RVMLEGACYINSTRUCTIONFACTORY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RVMLEGACYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmLegacyInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("RvmLegacyInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RvmInstructionFactoryBase {
}

pub trait RvmInstructionFactoryBaseTrait: TypeObject {
}

impl RvmInstructionFactoryBaseTrait for RvmInstructionFactoryBase {
}

pub static RVMINSTRUCTIONFACTORYBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmInstructionFactoryBase",
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmInstructionFactoryBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(RVMINSTRUCTIONFACTORYBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RvmInstructionFactoryBase {
    fn type_info(&self) -> &'static TypeInfo {
        RVMINSTRUCTIONFACTORYBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RVMINSTRUCTIONFACTORYBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmInstructionFactoryBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("RvmInstructionFactoryBase"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_InstanceTableAssemblyInstructionBatchData {
}

pub trait RvmSerializedDb_ns_InstanceTableAssemblyInstructionBatchDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_InstanceTableAssemblyInstructionBatchDataTrait for RvmSerializedDb_ns_InstanceTableAssemblyInstructionBatchData {
}

pub static RVMSERIALIZEDDB_NS_INSTANCETABLEASSEMBLYINSTRUCTIONBATCHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_InstanceTableAssemblyInstructionBatchData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_InstanceTableAssemblyInstructionBatchData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_INSTANCETABLEASSEMBLYINSTRUCTIONBATCHDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_InstanceTableAssemblyInstructionBatchData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_INSTANCETABLEASSEMBLYINSTRUCTIONBATCHDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_InstanceTableAssemblyData {
}

pub trait RvmSerializedDb_ns_InstanceTableAssemblyDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_InstanceTableAssemblyDataTrait for RvmSerializedDb_ns_InstanceTableAssemblyData {
}

pub static RVMSERIALIZEDDB_NS_INSTANCETABLEASSEMBLYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_InstanceTableAssemblyData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_InstanceTableAssemblyData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_INSTANCETABLEASSEMBLYDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RvmSerializedDb_ns_InstanceTableAssemblyData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_INSTANCETABLEASSEMBLYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_DepthBiasGroupData {
}

pub trait RvmSerializedDb_ns_DepthBiasGroupDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_DepthBiasGroupDataTrait for RvmSerializedDb_ns_DepthBiasGroupData {
}

pub static RVMSERIALIZEDDB_NS_DEPTHBIASGROUPDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DepthBiasGroupData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_DepthBiasGroupData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DEPTHBIASGROUPDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RvmSerializedDb_ns_DepthBiasGroupData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DEPTHBIASGROUPDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_TableAssemblyInstructionBatchData {
}

pub trait RvmSerializedDb_ns_TableAssemblyInstructionBatchDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_TableAssemblyInstructionBatchDataTrait for RvmSerializedDb_ns_TableAssemblyInstructionBatchData {
}

pub static RVMSERIALIZEDDB_NS_TABLEASSEMBLYINSTRUCTIONBATCHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_TableAssemblyInstructionBatchData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_TableAssemblyInstructionBatchData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_TABLEASSEMBLYINSTRUCTIONBATCHDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_TableAssemblyInstructionBatchData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_TABLEASSEMBLYINSTRUCTIONBATCHDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_TableAssemblyData {
}

pub trait RvmSerializedDb_ns_TableAssemblyDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_TableAssemblyDataTrait for RvmSerializedDb_ns_TableAssemblyData {
}

pub static RVMSERIALIZEDDB_NS_TABLEASSEMBLYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_TableAssemblyData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_TableAssemblyData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_TABLEASSEMBLYDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RvmSerializedDb_ns_TableAssemblyData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_TABLEASSEMBLYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_WriteOpGroup {
}

pub trait RvmSerializedDb_ns_WriteOpGroupTrait: TypeObject {
}

impl RvmSerializedDb_ns_WriteOpGroupTrait for RvmSerializedDb_ns_WriteOpGroup {
}

pub static RVMSERIALIZEDDB_NS_WRITEOPGROUP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_WriteOpGroup",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_WriteOpGroup as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_WRITEOPGROUP_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RvmSerializedDb_ns_WriteOpGroup {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_WRITEOPGROUP_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_WriteOp {
}

pub trait RvmSerializedDb_ns_WriteOpTrait: TypeObject {
}

impl RvmSerializedDb_ns_WriteOpTrait for RvmSerializedDb_ns_WriteOp {
}

pub static RVMSERIALIZEDDB_NS_WRITEOP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_WriteOp",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_WriteOp as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_WRITEOP_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_WriteOp {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_WRITEOP_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_DrawStateBuilderInstructionData {
}

pub trait RvmSerializedDb_ns_DrawStateBuilderInstructionDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_DrawStateBuilderInstructionDataTrait for RvmSerializedDb_ns_DrawStateBuilderInstructionData {
}

pub static RVMSERIALIZEDDB_NS_DRAWSTATEBUILDERINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DrawStateBuilderInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_DrawStateBuilderInstructionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DRAWSTATEBUILDERINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_DrawStateBuilderInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DRAWSTATEBUILDERINSTRUCTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_DispatchInstructionData {
}

pub trait RvmSerializedDb_ns_DispatchInstructionDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_DispatchInstructionDataTrait for RvmSerializedDb_ns_DispatchInstructionData {
}

pub static RVMSERIALIZEDDB_NS_DISPATCHINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DispatchInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_DispatchInstructionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DISPATCHINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_DispatchInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DISPATCHINSTRUCTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_DirectInputInstructionData {
}

pub trait RvmSerializedDb_ns_DirectInputInstructionDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_DirectInputInstructionDataTrait for RvmSerializedDb_ns_DirectInputInstructionData {
}

pub static RVMSERIALIZEDDB_NS_DIRECTINPUTINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DirectInputInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_DirectInputInstructionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DIRECTINPUTINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_DirectInputInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DIRECTINPUTINSTRUCTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_ConstantValueInstructionData {
}

pub trait RvmSerializedDb_ns_ConstantValueInstructionDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_ConstantValueInstructionDataTrait for RvmSerializedDb_ns_ConstantValueInstructionData {
}

pub static RVMSERIALIZEDDB_NS_CONSTANTVALUEINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ConstantValueInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_ConstantValueInstructionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_CONSTANTVALUEINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_ConstantValueInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_CONSTANTVALUEINSTRUCTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct char {
}

pub trait charTrait: TypeObject {
}

impl charTrait for char {
}

pub static CHAR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "char",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<char as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::CHAR_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for char {
    fn type_info(&self) -> &'static TypeInfo {
        CHAR_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ParamDbHash {
}

pub trait ParamDbHashTrait: TypeObject {
}

impl ParamDbHashTrait for ParamDbHash {
}

pub static PARAMDBHASH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ParamDbHash",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ParamDbHash as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::PARAMDBHASH_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ParamDbHash {
    fn type_info(&self) -> &'static TypeInfo {
        PARAMDBHASH_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmEncodedTableEntry {
}

pub trait RvmEncodedTableEntryTrait: TypeObject {
}

impl RvmEncodedTableEntryTrait for RvmEncodedTableEntry {
}

pub static RVMENCODEDTABLEENTRY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmEncodedTableEntry",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmEncodedTableEntry as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMENCODEDTABLEENTRY_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmEncodedTableEntry {
    fn type_info(&self) -> &'static TypeInfo {
        RVMENCODEDTABLEENTRY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmContextSortKeyInfo {
}

pub trait RvmContextSortKeyInfoTrait: TypeObject {
}

impl RvmContextSortKeyInfoTrait for RvmContextSortKeyInfo {
}

pub static RVMCONTEXTSORTKEYINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmContextSortKeyInfo",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmContextSortKeyInfo as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMCONTEXTSORTKEYINFO_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmContextSortKeyInfo {
    fn type_info(&self) -> &'static TypeInfo {
        RVMCONTEXTSORTKEYINFO_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_PreparedVertexElement {
}

pub trait RvmSerializedDb_ns_PreparedVertexElementTrait: TypeObject {
}

impl RvmSerializedDb_ns_PreparedVertexElementTrait for RvmSerializedDb_ns_PreparedVertexElement {
}

pub static RVMSERIALIZEDDB_NS_PREPAREDVERTEXELEMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_PreparedVertexElement",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_PreparedVertexElement as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_PREPAREDVERTEXELEMENT_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_PreparedVertexElement {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_PREPAREDVERTEXELEMENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_PreparedVertexStream {
}

pub trait RvmSerializedDb_ns_PreparedVertexStreamTrait: TypeObject {
}

impl RvmSerializedDb_ns_PreparedVertexStreamTrait for RvmSerializedDb_ns_PreparedVertexStream {
}

pub static RVMSERIALIZEDDB_NS_PREPAREDVERTEXSTREAM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_PreparedVertexStream",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_PreparedVertexStream as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_PREPAREDVERTEXSTREAM_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_PreparedVertexStream {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_PREPAREDVERTEXSTREAM_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_SerializedParamDbKey {
}

pub trait RvmSerializedDb_ns_SerializedParamDbKeyTrait: TypeObject {
}

impl RvmSerializedDb_ns_SerializedParamDbKeyTrait for RvmSerializedDb_ns_SerializedParamDbKey {
}

pub static RVMSERIALIZEDDB_NS_SERIALIZEDPARAMDBKEY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_SerializedParamDbKey",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_SerializedParamDbKey as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_SERIALIZEDPARAMDBKEY_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RvmSerializedDb_ns_SerializedParamDbKey {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_SERIALIZEDPARAMDBKEY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_BaseShaderState {
}

pub trait RvmSerializedDb_ns_BaseShaderStateTrait: TypeObject {
}

impl RvmSerializedDb_ns_BaseShaderStateTrait for RvmSerializedDb_ns_BaseShaderState {
}

pub static RVMSERIALIZEDDB_NS_BASESHADERSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_BaseShaderState",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_BaseShaderState as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_BASESHADERSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RvmSerializedDb_ns_BaseShaderState {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_BASESHADERSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_RuntimeInstantiatedType {
}

pub trait RvmSerializedDb_ns_RuntimeInstantiatedTypeTrait: TypeObject {
}

impl RvmSerializedDb_ns_RuntimeInstantiatedTypeTrait for RvmSerializedDb_ns_RuntimeInstantiatedType {
}

pub static RVMSERIALIZEDDB_NS_RUNTIMEINSTANTIATEDTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RuntimeInstantiatedType",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_RuntimeInstantiatedType as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_RUNTIMEINSTANTIATEDTYPE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_RuntimeInstantiatedType {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RUNTIMEINSTANTIATEDTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_ParamDbSerializedMultiHashView {
}

pub trait RvmSerializedDb_ns_ParamDbSerializedMultiHashViewTrait: TypeObject {
}

impl RvmSerializedDb_ns_ParamDbSerializedMultiHashViewTrait for RvmSerializedDb_ns_ParamDbSerializedMultiHashView {
}

pub static RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDMULTIHASHVIEW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbSerializedMultiHashView",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_ParamDbSerializedMultiHashView as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDMULTIHASHVIEW_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_ParamDbSerializedMultiHashView {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDMULTIHASHVIEW_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_ParamDbSerializedReadView {
}

pub trait RvmSerializedDb_ns_ParamDbSerializedReadViewTrait: TypeObject {
}

impl RvmSerializedDb_ns_ParamDbSerializedReadViewTrait for RvmSerializedDb_ns_ParamDbSerializedReadView {
}

pub static RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDREADVIEW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbSerializedReadView",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_ParamDbSerializedReadView as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDREADVIEW_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_ParamDbSerializedReadView {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDREADVIEW_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_ParamDbSerializedHashViewRef {
}

pub trait RvmSerializedDb_ns_ParamDbSerializedHashViewRefTrait: TypeObject {
}

impl RvmSerializedDb_ns_ParamDbSerializedHashViewRefTrait for RvmSerializedDb_ns_ParamDbSerializedHashViewRef {
}

pub static RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDHASHVIEWREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbSerializedHashViewRef",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_ParamDbSerializedHashViewRef as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDHASHVIEWREF_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_ParamDbSerializedHashViewRef {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDHASHVIEWREF_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_ParamDbSerializedHashView {
}

pub trait RvmSerializedDb_ns_ParamDbSerializedHashViewTrait: TypeObject {
}

impl RvmSerializedDb_ns_ParamDbSerializedHashViewTrait for RvmSerializedDb_ns_ParamDbSerializedHashView {
}

pub static RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDHASHVIEW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbSerializedHashView",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_ParamDbSerializedHashView as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDHASHVIEW_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_ParamDbSerializedHashView {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDHASHVIEW_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_ParamDbSerializedFilterView {
}

pub trait RvmSerializedDb_ns_ParamDbSerializedFilterViewTrait: TypeObject {
}

impl RvmSerializedDb_ns_ParamDbSerializedFilterViewTrait for RvmSerializedDb_ns_ParamDbSerializedFilterView {
}

pub static RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDFILTERVIEW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbSerializedFilterView",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_ParamDbSerializedFilterView as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDFILTERVIEW_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_ParamDbSerializedFilterView {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDFILTERVIEW_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_CombinedSerializedParameterBlock {
}

pub trait RvmSerializedDb_ns_CombinedSerializedParameterBlockTrait: TypeObject {
}

impl RvmSerializedDb_ns_CombinedSerializedParameterBlockTrait for RvmSerializedDb_ns_CombinedSerializedParameterBlock {
}

pub static RVMSERIALIZEDDB_NS_COMBINEDSERIALIZEDPARAMETERBLOCK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_CombinedSerializedParameterBlock",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_CombinedSerializedParameterBlock as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_COMBINEDSERIALIZEDPARAMETERBLOCK_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_CombinedSerializedParameterBlock {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_COMBINEDSERIALIZEDPARAMETERBLOCK_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_SerializedParameterBlockRef {
}

pub trait RvmSerializedDb_ns_SerializedParameterBlockRefTrait: TypeObject {
}

impl RvmSerializedDb_ns_SerializedParameterBlockRefTrait for RvmSerializedDb_ns_SerializedParameterBlockRef {
}

pub static RVMSERIALIZEDDB_NS_SERIALIZEDPARAMETERBLOCKREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_SerializedParameterBlockRef",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_SerializedParameterBlockRef as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_SERIALIZEDPARAMETERBLOCKREF_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_SerializedParameterBlockRef {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_SERIALIZEDPARAMETERBLOCKREF_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_SerializedParameterBlock {
}

pub trait RvmSerializedDb_ns_SerializedParameterBlockTrait: TypeObject {
}

impl RvmSerializedDb_ns_SerializedParameterBlockTrait for RvmSerializedDb_ns_SerializedParameterBlock {
}

pub static RVMSERIALIZEDDB_NS_SERIALIZEDPARAMETERBLOCK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_SerializedParameterBlock",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_SerializedParameterBlock as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_SERIALIZEDPARAMETERBLOCK_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_SerializedParameterBlock {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_SERIALIZEDPARAMETERBLOCK_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_ShaderStreamableExternalTextureRef {
}

pub trait RvmSerializedDb_ns_ShaderStreamableExternalTextureRefTrait: TypeObject {
}

impl RvmSerializedDb_ns_ShaderStreamableExternalTextureRefTrait for RvmSerializedDb_ns_ShaderStreamableExternalTextureRef {
}

pub static RVMSERIALIZEDDB_NS_SHADERSTREAMABLEEXTERNALTEXTUREREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ShaderStreamableExternalTextureRef",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_ShaderStreamableExternalTextureRef as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_SHADERSTREAMABLEEXTERNALTEXTUREREF_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_ShaderStreamableExternalTextureRef {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_SHADERSTREAMABLEEXTERNALTEXTUREREF_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_ShaderStreamableTextureRef {
}

pub trait RvmSerializedDb_ns_ShaderStreamableTextureRefTrait: TypeObject {
}

impl RvmSerializedDb_ns_ShaderStreamableTextureRefTrait for RvmSerializedDb_ns_ShaderStreamableTextureRef {
}

pub static RVMSERIALIZEDDB_NS_SHADERSTREAMABLETEXTUREREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ShaderStreamableTextureRef",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_ShaderStreamableTextureRef as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_SHADERSTREAMABLETEXTUREREF_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_ShaderStreamableTextureRef {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_SHADERSTREAMABLETEXTUREREF_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_SurfaceShaderDebugInfo {
}

pub trait RvmSerializedDb_ns_SurfaceShaderDebugInfoTrait: TypeObject {
}

impl RvmSerializedDb_ns_SurfaceShaderDebugInfoTrait for RvmSerializedDb_ns_SurfaceShaderDebugInfo {
}

pub static RVMSERIALIZEDDB_NS_SURFACESHADERDEBUGINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_SurfaceShaderDebugInfo",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_SurfaceShaderDebugInfo as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_SURFACESHADERDEBUGINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_SurfaceShaderDebugInfo {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_SURFACESHADERDEBUGINFO_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_SurfaceShader {
}

pub trait RvmSerializedDb_ns_SurfaceShaderTrait: TypeObject {
}

impl RvmSerializedDb_ns_SurfaceShaderTrait for RvmSerializedDb_ns_SurfaceShader {
}

pub static RVMSERIALIZEDDB_NS_SURFACESHADER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_SurfaceShader",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_SurfaceShader as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_SURFACESHADER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_SurfaceShader {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_SURFACESHADER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_ShaderStreamableTexture {
}

pub trait RvmSerializedDb_ns_ShaderStreamableTextureTrait: TypeObject {
}

impl RvmSerializedDb_ns_ShaderStreamableTextureTrait for RvmSerializedDb_ns_ShaderStreamableTexture {
}

pub static RVMSERIALIZEDDB_NS_SHADERSTREAMABLETEXTURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ShaderStreamableTexture",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_ShaderStreamableTexture as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_SHADERSTREAMABLETEXTURE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RvmSerializedDb_ns_ShaderStreamableTexture {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_SHADERSTREAMABLETEXTURE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_ShaderStreamableExternalTexture {
}

pub trait RvmSerializedDb_ns_ShaderStreamableExternalTextureTrait: TypeObject {
}

impl RvmSerializedDb_ns_ShaderStreamableExternalTextureTrait for RvmSerializedDb_ns_ShaderStreamableExternalTexture {
}

pub static RVMSERIALIZEDDB_NS_SHADERSTREAMABLEEXTERNALTEXTURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ShaderStreamableExternalTexture",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_ShaderStreamableExternalTexture as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_SHADERSTREAMABLEEXTERNALTEXTURE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RvmSerializedDb_ns_ShaderStreamableExternalTexture {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_SHADERSTREAMABLEEXTERNALTEXTURE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_RvmFunctionInstanceRef {
}

pub trait RvmSerializedDb_ns_RvmFunctionInstanceRefTrait: TypeObject {
}

impl RvmSerializedDb_ns_RvmFunctionInstanceRefTrait for RvmSerializedDb_ns_RvmFunctionInstanceRef {
}

pub static RVMSERIALIZEDDB_NS_RVMFUNCTIONINSTANCEREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmFunctionInstanceRef",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_RvmFunctionInstanceRef as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_RVMFUNCTIONINSTANCEREF_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_RvmFunctionInstanceRef {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RVMFUNCTIONINSTANCEREF_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_RvmFunctionInstance {
}

pub trait RvmSerializedDb_ns_RvmFunctionInstanceTrait: TypeObject {
}

impl RvmSerializedDb_ns_RvmFunctionInstanceTrait for RvmSerializedDb_ns_RvmFunctionInstance {
}

pub static RVMSERIALIZEDDB_NS_RVMFUNCTIONINSTANCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmFunctionInstance",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_RvmFunctionInstance as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_RVMFUNCTIONINSTANCE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_RvmFunctionInstance {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RVMFUNCTIONINSTANCE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_Settings {
}

pub trait RvmSerializedDb_ns_SettingsTrait: TypeObject {
}

impl RvmSerializedDb_ns_SettingsTrait for RvmSerializedDb_ns_Settings {
}

pub static RVMSERIALIZEDDB_NS_SETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Settings",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_Settings as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_SETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_Settings {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_SETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_RvmPermutationSet {
}

pub trait RvmSerializedDb_ns_RvmPermutationSetTrait: TypeObject {
}

impl RvmSerializedDb_ns_RvmPermutationSetTrait for RvmSerializedDb_ns_RvmPermutationSet {
}

pub static RVMSERIALIZEDDB_NS_RVMPERMUTATIONSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmPermutationSet",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_RvmPermutationSet as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_RVMPERMUTATIONSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_RvmPermutationSet {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RVMPERMUTATIONSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_RvmPermutationLookupTable {
}

pub trait RvmSerializedDb_ns_RvmPermutationLookupTableTrait: TypeObject {
}

impl RvmSerializedDb_ns_RvmPermutationLookupTableTrait for RvmSerializedDb_ns_RvmPermutationLookupTable {
}

pub static RVMSERIALIZEDDB_NS_RVMPERMUTATIONLOOKUPTABLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmPermutationLookupTable",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_RvmPermutationLookupTable as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_RVMPERMUTATIONLOOKUPTABLE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_RvmPermutationLookupTable {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RVMPERMUTATIONLOOKUPTABLE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_RvmPermutationTree {
}

pub trait RvmSerializedDb_ns_RvmPermutationTreeTrait: TypeObject {
}

impl RvmSerializedDb_ns_RvmPermutationTreeTrait for RvmSerializedDb_ns_RvmPermutationTree {
}

pub static RVMSERIALIZEDDB_NS_RVMPERMUTATIONTREE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmPermutationTree",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_RvmPermutationTree as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_RVMPERMUTATIONTREE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_RvmPermutationTree {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RVMPERMUTATIONTREE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_RvmPermutationRef {
}

pub trait RvmSerializedDb_ns_RvmPermutationRefTrait: TypeObject {
}

impl RvmSerializedDb_ns_RvmPermutationRefTrait for RvmSerializedDb_ns_RvmPermutationRef {
}

pub static RVMSERIALIZEDDB_NS_RVMPERMUTATIONREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmPermutationRef",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_RvmPermutationRef as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_RVMPERMUTATIONREF_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_RvmPermutationRef {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RVMPERMUTATIONREF_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_RvmPermutation {
}

pub trait RvmSerializedDb_ns_RvmPermutationTrait: TypeObject {
}

impl RvmSerializedDb_ns_RvmPermutationTrait for RvmSerializedDb_ns_RvmPermutation {
}

pub static RVMSERIALIZEDDB_NS_RVMPERMUTATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmPermutation",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_RvmPermutation as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_RVMPERMUTATION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_RvmPermutation {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RVMPERMUTATION_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_RvmFunctionInputTableIndices {
}

pub trait RvmSerializedDb_ns_RvmFunctionInputTableIndicesTrait: TypeObject {
}

impl RvmSerializedDb_ns_RvmFunctionInputTableIndicesTrait for RvmSerializedDb_ns_RvmFunctionInputTableIndices {
}

pub static RVMSERIALIZEDDB_NS_RVMFUNCTIONINPUTTABLEINDICES_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmFunctionInputTableIndices",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_RvmFunctionInputTableIndices as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_RVMFUNCTIONINPUTTABLEINDICES_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_RvmFunctionInputTableIndices {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RVMFUNCTIONINPUTTABLEINDICES_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_RvmDispatchDebugInfo {
}

pub trait RvmSerializedDb_ns_RvmDispatchDebugInfoTrait: TypeObject {
}

impl RvmSerializedDb_ns_RvmDispatchDebugInfoTrait for RvmSerializedDb_ns_RvmDispatchDebugInfo {
}

pub static RVMSERIALIZEDDB_NS_RVMDISPATCHDEBUGINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmDispatchDebugInfo",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_RvmDispatchDebugInfo as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_RVMDISPATCHDEBUGINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_RvmDispatchDebugInfo {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RVMDISPATCHDEBUGINFO_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_RvmFunctionDebugInfo {
}

pub trait RvmSerializedDb_ns_RvmFunctionDebugInfoTrait: TypeObject {
}

impl RvmSerializedDb_ns_RvmFunctionDebugInfoTrait for RvmSerializedDb_ns_RvmFunctionDebugInfo {
}

pub static RVMSERIALIZEDDB_NS_RVMFUNCTIONDEBUGINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmFunctionDebugInfo",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_RvmFunctionDebugInfo as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_RVMFUNCTIONDEBUGINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_RvmFunctionDebugInfo {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RVMFUNCTIONDEBUGINFO_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_RvmDispatch {
}

pub trait RvmSerializedDb_ns_RvmDispatchTrait: TypeObject {
}

impl RvmSerializedDb_ns_RvmDispatchTrait for RvmSerializedDb_ns_RvmDispatch {
}

pub static RVMSERIALIZEDDB_NS_RVMDISPATCH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmDispatch",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_RvmDispatch as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_RVMDISPATCH_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_RvmDispatch {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RVMDISPATCH_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_RvmFunction {
}

pub trait RvmSerializedDb_ns_RvmFunctionTrait: TypeObject {
}

impl RvmSerializedDb_ns_RvmFunctionTrait for RvmSerializedDb_ns_RvmFunction {
}

pub static RVMSERIALIZEDDB_NS_RVMFUNCTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmFunction",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_RvmFunction as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_RVMFUNCTION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_RvmFunction {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RVMFUNCTION_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_InstructionBatch {
}

pub trait RvmSerializedDb_ns_InstructionBatchTrait: TypeObject {
}

impl RvmSerializedDb_ns_InstructionBatchTrait for RvmSerializedDb_ns_InstructionBatch {
}

pub static RVMSERIALIZEDDB_NS_INSTRUCTIONBATCH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_InstructionBatch",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_InstructionBatch as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_INSTRUCTIONBATCH_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_InstructionBatch {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_INSTRUCTIONBATCH_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_RttiType {
}

pub trait RvmSerializedDb_ns_RttiTypeTrait: TypeObject {
}

impl RvmSerializedDb_ns_RttiTypeTrait for RvmSerializedDb_ns_RttiType {
}

pub static RVMSERIALIZEDDB_NS_RTTITYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RttiType",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_RttiType as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_RTTITYPE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_RttiType {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RTTITYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_DefaultValueRef {
}

pub trait RvmSerializedDb_ns_DefaultValueRefTrait: TypeObject {
}

impl RvmSerializedDb_ns_DefaultValueRefTrait for RvmSerializedDb_ns_DefaultValueRef {
}

pub static RVMSERIALIZEDDB_NS_DEFAULTVALUEREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DefaultValueRef",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_DefaultValueRef as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DEFAULTVALUEREF_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_DefaultValueRef {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DEFAULTVALUEREF_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_ValueRef {
}

pub trait RvmSerializedDb_ns_ValueRefTrait: TypeObject {
}

impl RvmSerializedDb_ns_ValueRefTrait for RvmSerializedDb_ns_ValueRef {
}

pub static RVMSERIALIZEDDB_NS_VALUEREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ValueRef",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_ValueRef as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_VALUEREF_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_ValueRef {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_VALUEREF_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_DefaultValueZeroMem {
}

pub trait RvmSerializedDb_ns_DefaultValueZeroMemTrait: TypeObject {
}

impl RvmSerializedDb_ns_DefaultValueZeroMemTrait for RvmSerializedDb_ns_DefaultValueZeroMem {
}

pub static RVMSERIALIZEDDB_NS_DEFAULTVALUEZEROMEM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DefaultValueZeroMem",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_DefaultValueZeroMem as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DEFAULTVALUEZEROMEM_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_DefaultValueZeroMem {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DEFAULTVALUEZEROMEM_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_DefaultValueStructLegacyData {
}

pub trait RvmSerializedDb_ns_DefaultValueStructLegacyDataTrait: TypeObject {
}

impl RvmSerializedDb_ns_DefaultValueStructLegacyDataTrait for RvmSerializedDb_ns_DefaultValueStructLegacyData {
}

pub static RVMSERIALIZEDDB_NS_DEFAULTVALUESTRUCTLEGACYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DefaultValueStructLegacyData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_DefaultValueStructLegacyData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DEFAULTVALUESTRUCTLEGACYDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RvmSerializedDb_ns_DefaultValueStructLegacyData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DEFAULTVALUESTRUCTLEGACYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_DefaultValueSimpleTexture {
}

pub trait RvmSerializedDb_ns_DefaultValueSimpleTextureTrait: TypeObject {
}

impl RvmSerializedDb_ns_DefaultValueSimpleTextureTrait for RvmSerializedDb_ns_DefaultValueSimpleTexture {
}

pub static RVMSERIALIZEDDB_NS_DEFAULTVALUESIMPLETEXTURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DefaultValueSimpleTexture",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_DefaultValueSimpleTexture as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DEFAULTVALUESIMPLETEXTURE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_DefaultValueSimpleTexture {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DEFAULTVALUESIMPLETEXTURE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_DefaultValueSimpleBuffer {
}

pub trait RvmSerializedDb_ns_DefaultValueSimpleBufferTrait: TypeObject {
}

impl RvmSerializedDb_ns_DefaultValueSimpleBufferTrait for RvmSerializedDb_ns_DefaultValueSimpleBuffer {
}

pub static RVMSERIALIZEDDB_NS_DEFAULTVALUESIMPLEBUFFER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DefaultValueSimpleBuffer",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_DefaultValueSimpleBuffer as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DEFAULTVALUESIMPLEBUFFER_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDb_ns_DefaultValueSimpleBuffer {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DEFAULTVALUESIMPLEBUFFER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_InstructionBatchRef {
}

pub trait RvmSerializedDb_ns_InstructionBatchRefTrait: TypeObject {
}

impl RvmSerializedDb_ns_InstructionBatchRefTrait for RvmSerializedDb_ns_InstructionBatchRef {
}

pub static RVMSERIALIZEDDB_NS_INSTRUCTIONBATCHREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_InstructionBatchRef",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_InstructionBatchRef as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_INSTRUCTIONBATCHREF_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_InstructionBatchRef {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_INSTRUCTIONBATCHREF_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_ParamDbKeyRef {
}

pub trait RvmSerializedDb_ns_ParamDbKeyRefTrait: TypeObject {
}

impl RvmSerializedDb_ns_ParamDbKeyRefTrait for RvmSerializedDb_ns_ParamDbKeyRef {
}

pub static RVMSERIALIZEDDB_NS_PARAMDBKEYREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbKeyRef",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_ParamDbKeyRef as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_PARAMDBKEYREF_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_ParamDbKeyRef {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_PARAMDBKEYREF_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDb_ns_ParamDbKeyDesc {
}

pub trait RvmSerializedDb_ns_ParamDbKeyDescTrait: TypeObject {
}

impl RvmSerializedDb_ns_ParamDbKeyDescTrait for RvmSerializedDb_ns_ParamDbKeyDesc {
}

pub static RVMSERIALIZEDDB_NS_PARAMDBKEYDESC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbKeyDesc",
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDb_ns_ParamDbKeyDesc as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_PARAMDBKEYDESC_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDb_ns_ParamDbKeyDesc {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_PARAMDBKEYDESC_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RvmDirectInputInstructionFactory {
    pub _glacier_base: RvmInstructionFactoryBase,
}

pub trait RvmDirectInputInstructionFactoryTrait: RvmInstructionFactoryBaseTrait {
}

impl RvmDirectInputInstructionFactoryTrait for RvmDirectInputInstructionFactory {
}

impl RvmInstructionFactoryBaseTrait for RvmDirectInputInstructionFactory {
}

pub static RVMDIRECTINPUTINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmDirectInputInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmDirectInputInstructionFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(RVMDIRECTINPUTINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmDirectInputInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        RVMDIRECTINPUTINSTRUCTIONFACTORY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RVMDIRECTINPUTINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmDirectInputInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("RvmDirectInputInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RvmConstantValueInstructionFactory {
    pub _glacier_base: RvmInstructionFactoryBase,
}

pub trait RvmConstantValueInstructionFactoryTrait: RvmInstructionFactoryBaseTrait {
}

impl RvmConstantValueInstructionFactoryTrait for RvmConstantValueInstructionFactory {
}

impl RvmInstructionFactoryBaseTrait for RvmConstantValueInstructionFactory {
}

pub static RVMCONSTANTVALUEINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmConstantValueInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmConstantValueInstructionFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(RVMCONSTANTVALUEINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmConstantValueInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        RVMCONSTANTVALUEINSTRUCTIONFACTORY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RVMCONSTANTVALUEINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmConstantValueInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("RvmConstantValueInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RvmCommonDatabaseLoaderImpl {
    pub _glacier_base: RvmCommonDatabaseLoader,
}

pub trait RvmCommonDatabaseLoaderImplTrait: RvmCommonDatabaseLoaderTrait {
}

impl RvmCommonDatabaseLoaderImplTrait for RvmCommonDatabaseLoaderImpl {
}

impl RvmCommonDatabaseLoaderTrait for RvmCommonDatabaseLoaderImpl {
}

impl super::render::RvmDatabaseLoaderTrait for RvmCommonDatabaseLoaderImpl {
}

pub static RVMCOMMONDATABASELOADERIMPL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmCommonDatabaseLoaderImpl",
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMCOMMONDATABASELOADER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmCommonDatabaseLoaderImpl as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(RVMCOMMONDATABASELOADERIMPL_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RvmCommonDatabaseLoaderImpl {
    fn type_info(&self) -> &'static TypeInfo {
        RVMCOMMONDATABASELOADERIMPL_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RVMCOMMONDATABASELOADERIMPL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmCommonDatabaseLoaderImpl-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("RvmCommonDatabaseLoaderImpl"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RvmCommonDatabaseLoader {
    pub _glacier_base: super::render::RvmDatabaseLoader,
}

pub trait RvmCommonDatabaseLoaderTrait: super::render::RvmDatabaseLoaderTrait {
}

impl RvmCommonDatabaseLoaderTrait for RvmCommonDatabaseLoader {
}

impl super::render::RvmDatabaseLoaderTrait for RvmCommonDatabaseLoader {
}

pub static RVMCOMMONDATABASELOADER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmCommonDatabaseLoader",
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::render::RVMDATABASELOADER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmCommonDatabaseLoader as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(RVMCOMMONDATABASELOADER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RvmCommonDatabaseLoader {
    fn type_info(&self) -> &'static TypeInfo {
        RVMCOMMONDATABASELOADER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RVMCOMMONDATABASELOADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmCommonDatabaseLoader-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("RvmCommonDatabaseLoader"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CommonRvmBackend {
    pub _glacier_base: super::render::RvmBackend,
}

pub trait CommonRvmBackendTrait: super::render::RvmBackendTrait {
}

impl CommonRvmBackendTrait for CommonRvmBackend {
}

impl super::render::RvmBackendTrait for CommonRvmBackend {
}

pub static COMMONRVMBACKEND_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CommonRvmBackend",
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::render::RVMBACKEND_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CommonRvmBackend as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(COMMONRVMBACKEND_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CommonRvmBackend {
    fn type_info(&self) -> &'static TypeInfo {
        COMMONRVMBACKEND_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static COMMONRVMBACKEND_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CommonRvmBackend-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("CommonRvmBackend"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BaseRvmDatabase {
    pub _glacier_base: super::render::RvmLegacyDatabase,
}

pub trait BaseRvmDatabaseTrait: super::render::RvmLegacyDatabaseTrait {
}

impl BaseRvmDatabaseTrait for BaseRvmDatabase {
}

impl super::render::RvmLegacyDatabaseTrait for BaseRvmDatabase {
}

impl super::render::RvmDatabaseTrait for BaseRvmDatabase {
}

impl super::core::IResourceObjectTrait for BaseRvmDatabase {
}

pub static BASERVMDATABASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BaseRvmDatabase",
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::render::RVMLEGACYDATABASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BaseRvmDatabase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(BASERVMDATABASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BaseRvmDatabase {
    fn type_info(&self) -> &'static TypeInfo {
        BASERVMDATABASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static BASERVMDATABASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BaseRvmDatabase-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("BaseRvmDatabase"),
    array_type: None,
    alignment: 8,
};


