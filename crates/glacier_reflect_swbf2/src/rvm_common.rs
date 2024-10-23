use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmBuildSettings {
    pub _glacier_base: super::core::SystemSettings,
    pub built_backends: Vec<Option<LockedTypeObject /* RvmBackendConfig */>>,
}

pub trait RvmBuildSettingsTrait: super::core::SystemSettingsTrait {
    fn built_backends(&self) -> &Vec<Option<LockedTypeObject /* RvmBackendConfig */>>;
    fn built_backends_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* RvmBackendConfig */>>;
}

impl RvmBuildSettingsTrait for RvmBuildSettings {
    fn built_backends(&self) -> &Vec<Option<LockedTypeObject /* RvmBackendConfig */>> {
        &self.built_backends
    }
    fn built_backends_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* RvmBackendConfig */>> {
        &mut self.built_backends
    }
}

impl super::core::SystemSettingsTrait for RvmBuildSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl super::core::DataContainerTrait for RvmBuildSettings {
}

pub static RVMBUILDSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmBuildSettings",
    name_hash: 903137503,
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        super_class_offset: offset_of!(RvmBuildSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmBuildSettings as Default>::default())),
            create_boxed: || Box::new(<RvmBuildSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "BuiltBackends",
                name_hash: 2943848116,
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


pub static RVMBUILDSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmBuildSettings-Array",
    name_hash: 1687936491,
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("RvmBuildSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for RvmBackendConfig {
}

pub static RVMBACKENDCONFIG_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmBackendConfig",
    name_hash: 2529929538,
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(RvmBackendConfig, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmBackendConfig as Default>::default())),
            create_boxed: || Box::new(<RvmBackendConfig as Default>::default()),
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


pub static RVMBACKENDCONFIG_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmBackendConfig-Array",
    name_hash: 3142629622,
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("RvmBackendConfig"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2145371720,
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        super_class_offset: offset_of!(LodFadeInstructionFactory, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LodFadeInstructionFactory as Default>::default())),
            create_boxed: || Box::new(<LodFadeInstructionFactory as Default>::default()),
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


pub static LODFADEINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LodFadeInstructionFactory-Array",
    name_hash: 613251324,
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("LodFadeInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsLodFadeInstructionData {
}

pub trait RvmSerializedDbnsLodFadeInstructionDataTrait: TypeObject {
}

impl RvmSerializedDbnsLodFadeInstructionDataTrait for RvmSerializedDbnsLodFadeInstructionData {
}

pub static RVMSERIALIZEDDB_NS_LODFADEINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_LodFadeInstructionData",
    name_hash: 1073714028,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsLodFadeInstructionData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsLodFadeInstructionData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_LODFADEINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDbnsLodFadeInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_LODFADEINSTRUCTIONDATA_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 396424615,
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        super_class_offset: offset_of!(PackLightMapWeightIntoInstanceInstructionFactory, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PackLightMapWeightIntoInstanceInstructionFactory as Default>::default())),
            create_boxed: || Box::new(<PackLightMapWeightIntoInstanceInstructionFactory as Default>::default()),
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


pub static PACKLIGHTMAPWEIGHTINTOINSTANCEINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PackLightMapWeightIntoInstanceInstructionFactory-Array",
    name_hash: 1349551891,
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("PackLightMapWeightIntoInstanceInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3325153594,
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        super_class_offset: offset_of!(SliceCountInstructionFactory, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SliceCountInstructionFactory as Default>::default())),
            create_boxed: || Box::new(<SliceCountInstructionFactory as Default>::default()),
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


pub static SLICECOUNTINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SliceCountInstructionFactory-Array",
    name_hash: 1444098958,
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("SliceCountInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1377894650,
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        super_class_offset: offset_of!(TessellationParametersInstructionFactory, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TessellationParametersInstructionFactory as Default>::default())),
            create_boxed: || Box::new(<TessellationParametersInstructionFactory as Default>::default()),
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


pub static TESSELLATIONPARAMETERSINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TessellationParametersInstructionFactory-Array",
    name_hash: 2093413326,
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("TessellationParametersInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3429030180,
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        super_class_offset: offset_of!(VectorSubtractInstructionFactory, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VectorSubtractInstructionFactory as Default>::default())),
            create_boxed: || Box::new(<VectorSubtractInstructionFactory as Default>::default()),
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


pub static VECTORSUBTRACTINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VectorSubtractInstructionFactory-Array",
    name_hash: 3286191248,
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("VectorSubtractInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2899805971,
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        super_class_offset: offset_of!(OffsetTranslationInMatrixInstructionFactory, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OffsetTranslationInMatrixInstructionFactory as Default>::default())),
            create_boxed: || Box::new(<OffsetTranslationInMatrixInstructionFactory as Default>::default()),
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


pub static OFFSETTRANSLATIONINMATRIXINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OffsetTranslationInMatrixInstructionFactory-Array",
    name_hash: 844464551,
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("OffsetTranslationInMatrixInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3737046989,
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        super_class_offset: offset_of!(CpuToGpuMatrixInstructionFactory, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CpuToGpuMatrixInstructionFactory as Default>::default())),
            create_boxed: || Box::new(<CpuToGpuMatrixInstructionFactory as Default>::default()),
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


pub static CPUTOGPUMATRIXINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CpuToGpuMatrixInstructionFactory-Array",
    name_hash: 57680889,
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("CpuToGpuMatrixInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 785938866,
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        super_class_offset: offset_of!(FloatToVecInstructionFactory, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FloatToVecInstructionFactory as Default>::default())),
            create_boxed: || Box::new(<FloatToVecInstructionFactory as Default>::default()),
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


pub static FLOATTOVECINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatToVecInstructionFactory-Array",
    name_hash: 2511790086,
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("FloatToVecInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsPackLightMapWeightIntoInstanceInstructionData {
}

pub trait RvmSerializedDbnsPackLightMapWeightIntoInstanceInstructionDataTrait: TypeObject {
}

impl RvmSerializedDbnsPackLightMapWeightIntoInstanceInstructionDataTrait for RvmSerializedDbnsPackLightMapWeightIntoInstanceInstructionData {
}

pub static RVMSERIALIZEDDB_NS_PACKLIGHTMAPWEIGHTINTOINSTANCEINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_PackLightMapWeightIntoInstanceInstructionData",
    name_hash: 38537891,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsPackLightMapWeightIntoInstanceInstructionData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsPackLightMapWeightIntoInstanceInstructionData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_PACKLIGHTMAPWEIGHTINTOINSTANCEINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDbnsPackLightMapWeightIntoInstanceInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_PACKLIGHTMAPWEIGHTINTOINSTANCEINSTRUCTIONDATA_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsSliceCountInstructionData {
}

pub trait RvmSerializedDbnsSliceCountInstructionDataTrait: TypeObject {
}

impl RvmSerializedDbnsSliceCountInstructionDataTrait for RvmSerializedDbnsSliceCountInstructionData {
}

pub static RVMSERIALIZEDDB_NS_SLICECOUNTINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_SliceCountInstructionData",
    name_hash: 4280082206,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsSliceCountInstructionData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsSliceCountInstructionData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_SLICECOUNTINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDbnsSliceCountInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_SLICECOUNTINSTRUCTIONDATA_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsTessellationParametersInstructionData {
}

pub trait RvmSerializedDbnsTessellationParametersInstructionDataTrait: TypeObject {
}

impl RvmSerializedDbnsTessellationParametersInstructionDataTrait for RvmSerializedDbnsTessellationParametersInstructionData {
}

pub static RVMSERIALIZEDDB_NS_TESSELLATIONPARAMETERSINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_TessellationParametersInstructionData",
    name_hash: 2949991774,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsTessellationParametersInstructionData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsTessellationParametersInstructionData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_TESSELLATIONPARAMETERSINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDbnsTessellationParametersInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_TESSELLATIONPARAMETERSINSTRUCTIONDATA_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsVectorSubtractInstructionData {
}

pub trait RvmSerializedDbnsVectorSubtractInstructionDataTrait: TypeObject {
}

impl RvmSerializedDbnsVectorSubtractInstructionDataTrait for RvmSerializedDbnsVectorSubtractInstructionData {
}

pub static RVMSERIALIZEDDB_NS_VECTORSUBTRACTINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_VectorSubtractInstructionData",
    name_hash: 2413226688,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsVectorSubtractInstructionData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsVectorSubtractInstructionData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_VECTORSUBTRACTINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDbnsVectorSubtractInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_VECTORSUBTRACTINSTRUCTIONDATA_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsOffsetTranslationInMatrixInstructionData {
}

pub trait RvmSerializedDbnsOffsetTranslationInMatrixInstructionDataTrait: TypeObject {
}

impl RvmSerializedDbnsOffsetTranslationInMatrixInstructionDataTrait for RvmSerializedDbnsOffsetTranslationInMatrixInstructionData {
}

pub static RVMSERIALIZEDDB_NS_OFFSETTRANSLATIONINMATRIXINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_OffsetTranslationInMatrixInstructionData",
    name_hash: 1684473495,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsOffsetTranslationInMatrixInstructionData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsOffsetTranslationInMatrixInstructionData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_OFFSETTRANSLATIONINMATRIXINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDbnsOffsetTranslationInMatrixInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_OFFSETTRANSLATIONINMATRIXINSTRUCTIONDATA_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsCpuToGpuMatrixInstructionData {
}

pub trait RvmSerializedDbnsCpuToGpuMatrixInstructionDataTrait: TypeObject {
}

impl RvmSerializedDbnsCpuToGpuMatrixInstructionDataTrait for RvmSerializedDbnsCpuToGpuMatrixInstructionData {
}

pub static RVMSERIALIZEDDB_NS_CPUTOGPUMATRIXINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_CpuToGpuMatrixInstructionData",
    name_hash: 1053422793,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsCpuToGpuMatrixInstructionData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsCpuToGpuMatrixInstructionData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_CPUTOGPUMATRIXINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDbnsCpuToGpuMatrixInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_CPUTOGPUMATRIXINSTRUCTIONDATA_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsFloatToVecInstructionData {
}

pub trait RvmSerializedDbnsFloatToVecInstructionDataTrait: TypeObject {
}

impl RvmSerializedDbnsFloatToVecInstructionDataTrait for RvmSerializedDbnsFloatToVecInstructionData {
}

pub static RVMSERIALIZEDDB_NS_FLOATTOVECINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_FloatToVecInstructionData",
    name_hash: 1420046038,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsFloatToVecInstructionData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsFloatToVecInstructionData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_FLOATTOVECINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDbnsFloatToVecInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_FLOATTOVECINSTRUCTIONDATA_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsLegacyVertexBufferConversionInstructionData {
}

pub trait RvmSerializedDbnsLegacyVertexBufferConversionInstructionDataTrait: TypeObject {
}

impl RvmSerializedDbnsLegacyVertexBufferConversionInstructionDataTrait for RvmSerializedDbnsLegacyVertexBufferConversionInstructionData {
}

pub static RVMSERIALIZEDDB_NS_LEGACYVERTEXBUFFERCONVERSIONINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_LegacyVertexBufferConversionInstructionData",
    name_hash: 1431728200,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsLegacyVertexBufferConversionInstructionData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsLegacyVertexBufferConversionInstructionData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_LEGACYVERTEXBUFFERCONVERSIONINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsLegacyVertexBufferConversionInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_LEGACYVERTEXBUFFERCONVERSIONINSTRUCTIONDATA_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsVertexStreamProcessInstructionData {
}

pub trait RvmSerializedDbnsVertexStreamProcessInstructionDataTrait: TypeObject {
}

impl RvmSerializedDbnsVertexStreamProcessInstructionDataTrait for RvmSerializedDbnsVertexStreamProcessInstructionData {
}

pub static RVMSERIALIZEDDB_NS_VERTEXSTREAMPROCESSINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_VertexStreamProcessInstructionData",
    name_hash: 683452498,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsVertexStreamProcessInstructionData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsVertexStreamProcessInstructionData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_VERTEXSTREAMPROCESSINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsVertexStreamProcessInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_VERTEXSTREAMPROCESSINSTRUCTIONDATA_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsViewStateInstructionData {
}

pub trait RvmSerializedDbnsViewStateInstructionDataTrait: TypeObject {
}

impl RvmSerializedDbnsViewStateInstructionDataTrait for RvmSerializedDbnsViewStateInstructionData {
}

pub static RVMSERIALIZEDDB_NS_VIEWSTATEINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ViewStateInstructionData",
    name_hash: 3562000695,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsViewStateInstructionData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsViewStateInstructionData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_VIEWSTATEINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDbnsViewStateInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_VIEWSTATEINSTRUCTIONDATA_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmCommonSettings {
    pub _glacier_base: super::core::DataContainer,
    pub on_demand_building_enable: bool,
}

pub trait RvmCommonSettingsTrait: super::core::DataContainerTrait {
    fn on_demand_building_enable(&self) -> &bool;
    fn on_demand_building_enable_mut(&mut self) -> &mut bool;
}

impl RvmCommonSettingsTrait for RvmCommonSettings {
    fn on_demand_building_enable(&self) -> &bool {
        &self.on_demand_building_enable
    }
    fn on_demand_building_enable_mut(&mut self) -> &mut bool {
        &mut self.on_demand_building_enable
    }
}

impl super::core::DataContainerTrait for RvmCommonSettings {
}

pub static RVMCOMMONSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmCommonSettings",
    name_hash: 1325672804,
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(RvmCommonSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmCommonSettings as Default>::default())),
            create_boxed: || Box::new(<RvmCommonSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "OnDemandBuildingEnable",
                name_hash: 3142983540,
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


pub static RVMCOMMONSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmCommonSettings-Array",
    name_hash: 1678232912,
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("RvmCommonSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmPointer {
}

pub trait RvmPointerTrait: TypeObject {
}

impl RvmPointerTrait for RvmPointer {
}

pub static RVMPOINTER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmPointer",
    name_hash: 2689611191,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmPointer as Default>::default())),
            create_boxed: || Box::new(<RvmPointer as Default>::default()),
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct GpuMat4x3 {
}

pub trait GpuMat4x3Trait: TypeObject {
}

impl GpuMat4x3Trait for GpuMat4x3 {
}

pub static GPUMAT4X3_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GpuMat4x3",
    name_hash: 4277515296,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GpuMat4x3 as Default>::default())),
            create_boxed: || Box::new(<GpuMat4x3 as Default>::default()),
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct IVec4 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
}

pub trait IVec4Trait: TypeObject {
    fn x(&self) -> &i32;
    fn x_mut(&mut self) -> &mut i32;
    fn y(&self) -> &i32;
    fn y_mut(&mut self) -> &mut i32;
    fn z(&self) -> &i32;
    fn z_mut(&mut self) -> &mut i32;
    fn w(&self) -> &i32;
    fn w_mut(&mut self) -> &mut i32;
}

impl IVec4Trait for IVec4 {
    fn x(&self) -> &i32 {
        &self.x
    }
    fn x_mut(&mut self) -> &mut i32 {
        &mut self.x
    }
    fn y(&self) -> &i32 {
        &self.y
    }
    fn y_mut(&mut self) -> &mut i32 {
        &mut self.y
    }
    fn z(&self) -> &i32 {
        &self.z
    }
    fn z_mut(&mut self) -> &mut i32 {
        &mut self.z
    }
    fn w(&self) -> &i32 {
        &self.w
    }
    fn w_mut(&mut self) -> &mut i32 {
        &mut self.w
    }
}

pub static IVEC4_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IVec4",
    name_hash: 216453416,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IVec4 as Default>::default())),
            create_boxed: || Box::new(<IVec4 as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "x",
                name_hash: 177629,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(IVec4, x),
            },
            FieldInfoData {
                name: "y",
                name_hash: 177628,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(IVec4, y),
            },
            FieldInfoData {
                name: "z",
                name_hash: 177631,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(IVec4, z),
            },
            FieldInfoData {
                name: "w",
                name_hash: 177618,
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct IVec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

pub trait IVec3Trait: TypeObject {
    fn x(&self) -> &i32;
    fn x_mut(&mut self) -> &mut i32;
    fn y(&self) -> &i32;
    fn y_mut(&mut self) -> &mut i32;
    fn z(&self) -> &i32;
    fn z_mut(&mut self) -> &mut i32;
}

impl IVec3Trait for IVec3 {
    fn x(&self) -> &i32 {
        &self.x
    }
    fn x_mut(&mut self) -> &mut i32 {
        &mut self.x
    }
    fn y(&self) -> &i32 {
        &self.y
    }
    fn y_mut(&mut self) -> &mut i32 {
        &mut self.y
    }
    fn z(&self) -> &i32 {
        &self.z
    }
    fn z_mut(&mut self) -> &mut i32 {
        &mut self.z
    }
}

pub static IVEC3_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IVec3",
    name_hash: 216453423,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IVec3 as Default>::default())),
            create_boxed: || Box::new(<IVec3 as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "x",
                name_hash: 177629,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(IVec3, x),
            },
            FieldInfoData {
                name: "y",
                name_hash: 177628,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(IVec3, y),
            },
            FieldInfoData {
                name: "z",
                name_hash: 177631,
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct IVec2 {
    pub x: i32,
    pub y: i32,
}

pub trait IVec2Trait: TypeObject {
    fn x(&self) -> &i32;
    fn x_mut(&mut self) -> &mut i32;
    fn y(&self) -> &i32;
    fn y_mut(&mut self) -> &mut i32;
}

impl IVec2Trait for IVec2 {
    fn x(&self) -> &i32 {
        &self.x
    }
    fn x_mut(&mut self) -> &mut i32 {
        &mut self.x
    }
    fn y(&self) -> &i32 {
        &self.y
    }
    fn y_mut(&mut self) -> &mut i32 {
        &mut self.y
    }
}

pub static IVEC2_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IVec2",
    name_hash: 216453422,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IVec2 as Default>::default())),
            create_boxed: || Box::new(<IVec2 as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "x",
                name_hash: 177629,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(IVec2, x),
            },
            FieldInfoData {
                name: "y",
                name_hash: 177628,
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct Half4 {
    pub x: Half,
    pub y: Half,
    pub z: Half,
    pub w: Half,
}

pub trait Half4Trait: TypeObject {
    fn x(&self) -> &Half;
    fn x_mut(&mut self) -> &mut Half;
    fn y(&self) -> &Half;
    fn y_mut(&mut self) -> &mut Half;
    fn z(&self) -> &Half;
    fn z_mut(&mut self) -> &mut Half;
    fn w(&self) -> &Half;
    fn w_mut(&mut self) -> &mut Half;
}

impl Half4Trait for Half4 {
    fn x(&self) -> &Half {
        &self.x
    }
    fn x_mut(&mut self) -> &mut Half {
        &mut self.x
    }
    fn y(&self) -> &Half {
        &self.y
    }
    fn y_mut(&mut self) -> &mut Half {
        &mut self.y
    }
    fn z(&self) -> &Half {
        &self.z
    }
    fn z_mut(&mut self) -> &mut Half {
        &mut self.z
    }
    fn w(&self) -> &Half {
        &self.w
    }
    fn w_mut(&mut self) -> &mut Half {
        &mut self.w
    }
}

pub static HALF4_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Half4",
    name_hash: 222838290,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Half4 as Default>::default())),
            create_boxed: || Box::new(<Half4 as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "x",
                name_hash: 177629,
                flags: MemberInfoFlags::new(0),
                field_type: "Half",
                rust_offset: offset_of!(Half4, x),
            },
            FieldInfoData {
                name: "y",
                name_hash: 177628,
                flags: MemberInfoFlags::new(0),
                field_type: "Half",
                rust_offset: offset_of!(Half4, y),
            },
            FieldInfoData {
                name: "z",
                name_hash: 177631,
                flags: MemberInfoFlags::new(0),
                field_type: "Half",
                rust_offset: offset_of!(Half4, z),
            },
            FieldInfoData {
                name: "w",
                name_hash: 177618,
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct Half3 {
    pub x: Half,
    pub y: Half,
    pub z: Half,
}

pub trait Half3Trait: TypeObject {
    fn x(&self) -> &Half;
    fn x_mut(&mut self) -> &mut Half;
    fn y(&self) -> &Half;
    fn y_mut(&mut self) -> &mut Half;
    fn z(&self) -> &Half;
    fn z_mut(&mut self) -> &mut Half;
}

impl Half3Trait for Half3 {
    fn x(&self) -> &Half {
        &self.x
    }
    fn x_mut(&mut self) -> &mut Half {
        &mut self.x
    }
    fn y(&self) -> &Half {
        &self.y
    }
    fn y_mut(&mut self) -> &mut Half {
        &mut self.y
    }
    fn z(&self) -> &Half {
        &self.z
    }
    fn z_mut(&mut self) -> &mut Half {
        &mut self.z
    }
}

pub static HALF3_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Half3",
    name_hash: 222838293,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Half3 as Default>::default())),
            create_boxed: || Box::new(<Half3 as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "x",
                name_hash: 177629,
                flags: MemberInfoFlags::new(0),
                field_type: "Half",
                rust_offset: offset_of!(Half3, x),
            },
            FieldInfoData {
                name: "y",
                name_hash: 177628,
                flags: MemberInfoFlags::new(0),
                field_type: "Half",
                rust_offset: offset_of!(Half3, y),
            },
            FieldInfoData {
                name: "z",
                name_hash: 177631,
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct Half2 {
    pub x: Half,
    pub y: Half,
}

pub trait Half2Trait: TypeObject {
    fn x(&self) -> &Half;
    fn x_mut(&mut self) -> &mut Half;
    fn y(&self) -> &Half;
    fn y_mut(&mut self) -> &mut Half;
}

impl Half2Trait for Half2 {
    fn x(&self) -> &Half {
        &self.x
    }
    fn x_mut(&mut self) -> &mut Half {
        &mut self.x
    }
    fn y(&self) -> &Half {
        &self.y
    }
    fn y_mut(&mut self) -> &mut Half {
        &mut self.y
    }
}

pub static HALF2_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Half2",
    name_hash: 222838292,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Half2 as Default>::default())),
            create_boxed: || Box::new(<Half2 as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "x",
                name_hash: 177629,
                flags: MemberInfoFlags::new(0),
                field_type: "Half",
                rust_offset: offset_of!(Half2, x),
            },
            FieldInfoData {
                name: "y",
                name_hash: 177628,
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct Half {
}

pub trait HalfTrait: TypeObject {
}

impl HalfTrait for Half {
}

pub static HALF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Half",
    name_hash: 2089161062,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Half as Default>::default())),
            create_boxed: || Box::new(<Half as Default>::default()),
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmLegacyInstructionsnsLegacyInstructionData {
}

pub trait RvmLegacyInstructionsnsLegacyInstructionDataTrait: TypeObject {
}

impl RvmLegacyInstructionsnsLegacyInstructionDataTrait for RvmLegacyInstructionsnsLegacyInstructionData {
}

pub static RVMLEGACYINSTRUCTIONS_NS_LEGACYINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmLegacyInstructions_ns_LegacyInstructionData",
    name_hash: 926900402,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmLegacyInstructionsnsLegacyInstructionData as Default>::default())),
            create_boxed: || Box::new(<RvmLegacyInstructionsnsLegacyInstructionData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMLEGACYINSTRUCTIONS_NS_LEGACYINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmLegacyInstructionsnsLegacyInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMLEGACYINSTRUCTIONS_NS_LEGACYINSTRUCTIONDATA_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1782503029,
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        super_class_offset: offset_of!(RvmLegacyInstructionFactory, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmLegacyInstructionFactory as Default>::default())),
            create_boxed: || Box::new(<RvmLegacyInstructionFactory as Default>::default()),
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


pub static RVMLEGACYINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmLegacyInstructionFactory-Array",
    name_hash: 1414432577,
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("RvmLegacyInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmInstructionFactoryBase {
}

pub trait RvmInstructionFactoryBaseTrait: TypeObject {
}

impl RvmInstructionFactoryBaseTrait for RvmInstructionFactoryBase {
}

pub static RVMINSTRUCTIONFACTORYBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmInstructionFactoryBase",
    name_hash: 2216690581,
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmInstructionFactoryBase as Default>::default())),
            create_boxed: || Box::new(<RvmInstructionFactoryBase as Default>::default()),
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


pub static RVMINSTRUCTIONFACTORYBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmInstructionFactoryBase-Array",
    name_hash: 2570097441,
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("RvmInstructionFactoryBase"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsInstanceTableAssemblyInstructionBatchData {
}

pub trait RvmSerializedDbnsInstanceTableAssemblyInstructionBatchDataTrait: TypeObject {
}

impl RvmSerializedDbnsInstanceTableAssemblyInstructionBatchDataTrait for RvmSerializedDbnsInstanceTableAssemblyInstructionBatchData {
}

pub static RVMSERIALIZEDDB_NS_INSTANCETABLEASSEMBLYINSTRUCTIONBATCHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_InstanceTableAssemblyInstructionBatchData",
    name_hash: 385722968,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsInstanceTableAssemblyInstructionBatchData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsInstanceTableAssemblyInstructionBatchData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_INSTANCETABLEASSEMBLYINSTRUCTIONBATCHDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsInstanceTableAssemblyInstructionBatchData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_INSTANCETABLEASSEMBLYINSTRUCTIONBATCHDATA_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsInstanceTableAssemblyData {
}

pub trait RvmSerializedDbnsInstanceTableAssemblyDataTrait: TypeObject {
}

impl RvmSerializedDbnsInstanceTableAssemblyDataTrait for RvmSerializedDbnsInstanceTableAssemblyData {
}

pub static RVMSERIALIZEDDB_NS_INSTANCETABLEASSEMBLYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_InstanceTableAssemblyData",
    name_hash: 1816312604,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsInstanceTableAssemblyData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsInstanceTableAssemblyData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_INSTANCETABLEASSEMBLYDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RvmSerializedDbnsInstanceTableAssemblyData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_INSTANCETABLEASSEMBLYDATA_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsDepthBiasGroupData {
}

pub trait RvmSerializedDbnsDepthBiasGroupDataTrait: TypeObject {
}

impl RvmSerializedDbnsDepthBiasGroupDataTrait for RvmSerializedDbnsDepthBiasGroupData {
}

pub static RVMSERIALIZEDDB_NS_DEPTHBIASGROUPDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DepthBiasGroupData",
    name_hash: 3645281054,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDepthBiasGroupData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDepthBiasGroupData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DEPTHBIASGROUPDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RvmSerializedDbnsDepthBiasGroupData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DEPTHBIASGROUPDATA_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsTableAssemblyInstructionBatchData {
}

pub trait RvmSerializedDbnsTableAssemblyInstructionBatchDataTrait: TypeObject {
}

impl RvmSerializedDbnsTableAssemblyInstructionBatchDataTrait for RvmSerializedDbnsTableAssemblyInstructionBatchData {
}

pub static RVMSERIALIZEDDB_NS_TABLEASSEMBLYINSTRUCTIONBATCHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_TableAssemblyInstructionBatchData",
    name_hash: 4223020689,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsTableAssemblyInstructionBatchData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsTableAssemblyInstructionBatchData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_TABLEASSEMBLYINSTRUCTIONBATCHDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsTableAssemblyInstructionBatchData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_TABLEASSEMBLYINSTRUCTIONBATCHDATA_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsTableAssemblyData {
}

pub trait RvmSerializedDbnsTableAssemblyDataTrait: TypeObject {
}

impl RvmSerializedDbnsTableAssemblyDataTrait for RvmSerializedDbnsTableAssemblyData {
}

pub static RVMSERIALIZEDDB_NS_TABLEASSEMBLYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_TableAssemblyData",
    name_hash: 3624816661,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsTableAssemblyData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsTableAssemblyData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_TABLEASSEMBLYDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RvmSerializedDbnsTableAssemblyData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_TABLEASSEMBLYDATA_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsWriteOpGroup {
}

pub trait RvmSerializedDbnsWriteOpGroupTrait: TypeObject {
}

impl RvmSerializedDbnsWriteOpGroupTrait for RvmSerializedDbnsWriteOpGroup {
}

pub static RVMSERIALIZEDDB_NS_WRITEOPGROUP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_WriteOpGroup",
    name_hash: 1750895384,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsWriteOpGroup as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsWriteOpGroup as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_WRITEOPGROUP_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RvmSerializedDbnsWriteOpGroup {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_WRITEOPGROUP_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsWriteOp {
}

pub trait RvmSerializedDbnsWriteOpTrait: TypeObject {
}

impl RvmSerializedDbnsWriteOpTrait for RvmSerializedDbnsWriteOp {
}

pub static RVMSERIALIZEDDB_NS_WRITEOP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_WriteOp",
    name_hash: 942972679,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsWriteOp as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsWriteOp as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_WRITEOP_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDbnsWriteOp {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_WRITEOP_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsDrawStateBuilderInstructionData {
}

pub trait RvmSerializedDbnsDrawStateBuilderInstructionDataTrait: TypeObject {
}

impl RvmSerializedDbnsDrawStateBuilderInstructionDataTrait for RvmSerializedDbnsDrawStateBuilderInstructionData {
}

pub static RVMSERIALIZEDDB_NS_DRAWSTATEBUILDERINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DrawStateBuilderInstructionData",
    name_hash: 2460957275,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDrawStateBuilderInstructionData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDrawStateBuilderInstructionData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DRAWSTATEBUILDERINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDbnsDrawStateBuilderInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DRAWSTATEBUILDERINSTRUCTIONDATA_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsDispatchInstructionData {
}

pub trait RvmSerializedDbnsDispatchInstructionDataTrait: TypeObject {
}

impl RvmSerializedDbnsDispatchInstructionDataTrait for RvmSerializedDbnsDispatchInstructionData {
}

pub static RVMSERIALIZEDDB_NS_DISPATCHINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DispatchInstructionData",
    name_hash: 2326572157,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDispatchInstructionData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDispatchInstructionData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DISPATCHINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDbnsDispatchInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DISPATCHINSTRUCTIONDATA_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsDirectInputInstructionData {
}

pub trait RvmSerializedDbnsDirectInputInstructionDataTrait: TypeObject {
}

impl RvmSerializedDbnsDirectInputInstructionDataTrait for RvmSerializedDbnsDirectInputInstructionData {
}

pub static RVMSERIALIZEDDB_NS_DIRECTINPUTINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DirectInputInstructionData",
    name_hash: 1832125366,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDirectInputInstructionData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDirectInputInstructionData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DIRECTINPUTINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsDirectInputInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DIRECTINPUTINSTRUCTIONDATA_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsConstantValueInstructionData {
}

pub trait RvmSerializedDbnsConstantValueInstructionDataTrait: TypeObject {
}

impl RvmSerializedDbnsConstantValueInstructionDataTrait for RvmSerializedDbnsConstantValueInstructionData {
}

pub static RVMSERIALIZEDDB_NS_CONSTANTVALUEINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ConstantValueInstructionData",
    name_hash: 913289400,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsConstantValueInstructionData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsConstantValueInstructionData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_CONSTANTVALUEINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsConstantValueInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_CONSTANTVALUEINSTRUCTIONDATA_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct char {
}

pub trait charTrait: TypeObject {
}

impl charTrait for char {
}

pub static CHAR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "char",
    name_hash: 2087773917,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<char as Default>::default())),
            create_boxed: || Box::new(<char as Default>::default()),
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct ParamDbHash {
}

pub trait ParamDbHashTrait: TypeObject {
}

impl ParamDbHashTrait for ParamDbHash {
}

pub static PARAMDBHASH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ParamDbHash",
    name_hash: 3560589918,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ParamDbHash as Default>::default())),
            create_boxed: || Box::new(<ParamDbHash as Default>::default()),
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmEncodedTableEntry {
}

pub trait RvmEncodedTableEntryTrait: TypeObject {
}

impl RvmEncodedTableEntryTrait for RvmEncodedTableEntry {
}

pub static RVMENCODEDTABLEENTRY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmEncodedTableEntry",
    name_hash: 3711184388,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmEncodedTableEntry as Default>::default())),
            create_boxed: || Box::new(<RvmEncodedTableEntry as Default>::default()),
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmContextSortKeyInfo {
}

pub trait RvmContextSortKeyInfoTrait: TypeObject {
}

impl RvmContextSortKeyInfoTrait for RvmContextSortKeyInfo {
}

pub static RVMCONTEXTSORTKEYINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmContextSortKeyInfo",
    name_hash: 1614081232,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmContextSortKeyInfo as Default>::default())),
            create_boxed: || Box::new(<RvmContextSortKeyInfo as Default>::default()),
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsPreparedVertexElement {
}

pub trait RvmSerializedDbnsPreparedVertexElementTrait: TypeObject {
}

impl RvmSerializedDbnsPreparedVertexElementTrait for RvmSerializedDbnsPreparedVertexElement {
}

pub static RVMSERIALIZEDDB_NS_PREPAREDVERTEXELEMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_PreparedVertexElement",
    name_hash: 3866961878,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsPreparedVertexElement as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsPreparedVertexElement as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_PREPAREDVERTEXELEMENT_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDbnsPreparedVertexElement {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_PREPAREDVERTEXELEMENT_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsPreparedVertexStream {
}

pub trait RvmSerializedDbnsPreparedVertexStreamTrait: TypeObject {
}

impl RvmSerializedDbnsPreparedVertexStreamTrait for RvmSerializedDbnsPreparedVertexStream {
}

pub static RVMSERIALIZEDDB_NS_PREPAREDVERTEXSTREAM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_PreparedVertexStream",
    name_hash: 1467945364,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsPreparedVertexStream as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsPreparedVertexStream as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_PREPAREDVERTEXSTREAM_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDbnsPreparedVertexStream {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_PREPAREDVERTEXSTREAM_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsSerializedParamDbKey {
}

pub trait RvmSerializedDbnsSerializedParamDbKeyTrait: TypeObject {
}

impl RvmSerializedDbnsSerializedParamDbKeyTrait for RvmSerializedDbnsSerializedParamDbKey {
}

pub static RVMSERIALIZEDDB_NS_SERIALIZEDPARAMDBKEY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_SerializedParamDbKey",
    name_hash: 788411241,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsSerializedParamDbKey as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsSerializedParamDbKey as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_SERIALIZEDPARAMDBKEY_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RvmSerializedDbnsSerializedParamDbKey {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_SERIALIZEDPARAMDBKEY_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsBaseShaderState {
}

pub trait RvmSerializedDbnsBaseShaderStateTrait: TypeObject {
}

impl RvmSerializedDbnsBaseShaderStateTrait for RvmSerializedDbnsBaseShaderState {
}

pub static RVMSERIALIZEDDB_NS_BASESHADERSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_BaseShaderState",
    name_hash: 2450159502,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsBaseShaderState as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsBaseShaderState as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_BASESHADERSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RvmSerializedDbnsBaseShaderState {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_BASESHADERSTATE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsRuntimeInstantiatedType {
}

pub trait RvmSerializedDbnsRuntimeInstantiatedTypeTrait: TypeObject {
}

impl RvmSerializedDbnsRuntimeInstantiatedTypeTrait for RvmSerializedDbnsRuntimeInstantiatedType {
}

pub static RVMSERIALIZEDDB_NS_RUNTIMEINSTANTIATEDTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RuntimeInstantiatedType",
    name_hash: 2581277799,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsRuntimeInstantiatedType as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsRuntimeInstantiatedType as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_RUNTIMEINSTANTIATEDTYPE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsRuntimeInstantiatedType {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RUNTIMEINSTANTIATEDTYPE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsParamDbSerializedMultiHashView {
}

pub trait RvmSerializedDbnsParamDbSerializedMultiHashViewTrait: TypeObject {
}

impl RvmSerializedDbnsParamDbSerializedMultiHashViewTrait for RvmSerializedDbnsParamDbSerializedMultiHashView {
}

pub static RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDMULTIHASHVIEW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbSerializedMultiHashView",
    name_hash: 2759261608,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsParamDbSerializedMultiHashView as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsParamDbSerializedMultiHashView as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDMULTIHASHVIEW_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsParamDbSerializedMultiHashView {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDMULTIHASHVIEW_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsParamDbSerializedReadView {
}

pub trait RvmSerializedDbnsParamDbSerializedReadViewTrait: TypeObject {
}

impl RvmSerializedDbnsParamDbSerializedReadViewTrait for RvmSerializedDbnsParamDbSerializedReadView {
}

pub static RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDREADVIEW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbSerializedReadView",
    name_hash: 3358556289,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsParamDbSerializedReadView as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsParamDbSerializedReadView as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDREADVIEW_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsParamDbSerializedReadView {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDREADVIEW_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsParamDbSerializedHashViewRef {
}

pub trait RvmSerializedDbnsParamDbSerializedHashViewRefTrait: TypeObject {
}

impl RvmSerializedDbnsParamDbSerializedHashViewRefTrait for RvmSerializedDbnsParamDbSerializedHashViewRef {
}

pub static RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDHASHVIEWREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbSerializedHashViewRef",
    name_hash: 2500837264,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsParamDbSerializedHashViewRef as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsParamDbSerializedHashViewRef as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDHASHVIEWREF_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsParamDbSerializedHashViewRef {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDHASHVIEWREF_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsParamDbSerializedHashView {
}

pub trait RvmSerializedDbnsParamDbSerializedHashViewTrait: TypeObject {
}

impl RvmSerializedDbnsParamDbSerializedHashViewTrait for RvmSerializedDbnsParamDbSerializedHashView {
}

pub static RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDHASHVIEW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbSerializedHashView",
    name_hash: 2413292161,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsParamDbSerializedHashView as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsParamDbSerializedHashView as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDHASHVIEW_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsParamDbSerializedHashView {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDHASHVIEW_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsParamDbSerializedFilterView {
}

pub trait RvmSerializedDbnsParamDbSerializedFilterViewTrait: TypeObject {
}

impl RvmSerializedDbnsParamDbSerializedFilterViewTrait for RvmSerializedDbnsParamDbSerializedFilterView {
}

pub static RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDFILTERVIEW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbSerializedFilterView",
    name_hash: 2571168915,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsParamDbSerializedFilterView as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsParamDbSerializedFilterView as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDFILTERVIEW_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsParamDbSerializedFilterView {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDFILTERVIEW_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsCombinedSerializedParameterBlock {
}

pub trait RvmSerializedDbnsCombinedSerializedParameterBlockTrait: TypeObject {
}

impl RvmSerializedDbnsCombinedSerializedParameterBlockTrait for RvmSerializedDbnsCombinedSerializedParameterBlock {
}

pub static RVMSERIALIZEDDB_NS_COMBINEDSERIALIZEDPARAMETERBLOCK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_CombinedSerializedParameterBlock",
    name_hash: 3292859314,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsCombinedSerializedParameterBlock as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsCombinedSerializedParameterBlock as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_COMBINEDSERIALIZEDPARAMETERBLOCK_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsCombinedSerializedParameterBlock {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_COMBINEDSERIALIZEDPARAMETERBLOCK_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsSerializedParameterBlockRef {
}

pub trait RvmSerializedDbnsSerializedParameterBlockRefTrait: TypeObject {
}

impl RvmSerializedDbnsSerializedParameterBlockRefTrait for RvmSerializedDbnsSerializedParameterBlockRef {
}

pub static RVMSERIALIZEDDB_NS_SERIALIZEDPARAMETERBLOCKREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_SerializedParameterBlockRef",
    name_hash: 1620971014,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsSerializedParameterBlockRef as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsSerializedParameterBlockRef as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_SERIALIZEDPARAMETERBLOCKREF_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsSerializedParameterBlockRef {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_SERIALIZEDPARAMETERBLOCKREF_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsSerializedParameterBlock {
}

pub trait RvmSerializedDbnsSerializedParameterBlockTrait: TypeObject {
}

impl RvmSerializedDbnsSerializedParameterBlockTrait for RvmSerializedDbnsSerializedParameterBlock {
}

pub static RVMSERIALIZEDDB_NS_SERIALIZEDPARAMETERBLOCK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_SerializedParameterBlock",
    name_hash: 709478999,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsSerializedParameterBlock as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsSerializedParameterBlock as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_SERIALIZEDPARAMETERBLOCK_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsSerializedParameterBlock {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_SERIALIZEDPARAMETERBLOCK_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsShaderStreamableExternalTextureRef {
}

pub trait RvmSerializedDbnsShaderStreamableExternalTextureRefTrait: TypeObject {
}

impl RvmSerializedDbnsShaderStreamableExternalTextureRefTrait for RvmSerializedDbnsShaderStreamableExternalTextureRef {
}

pub static RVMSERIALIZEDDB_NS_SHADERSTREAMABLEEXTERNALTEXTUREREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ShaderStreamableExternalTextureRef",
    name_hash: 3710142121,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsShaderStreamableExternalTextureRef as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsShaderStreamableExternalTextureRef as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_SHADERSTREAMABLEEXTERNALTEXTUREREF_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsShaderStreamableExternalTextureRef {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_SHADERSTREAMABLEEXTERNALTEXTUREREF_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsShaderStreamableTextureRef {
}

pub trait RvmSerializedDbnsShaderStreamableTextureRefTrait: TypeObject {
}

impl RvmSerializedDbnsShaderStreamableTextureRefTrait for RvmSerializedDbnsShaderStreamableTextureRef {
}

pub static RVMSERIALIZEDDB_NS_SHADERSTREAMABLETEXTUREREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ShaderStreamableTextureRef",
    name_hash: 2223500116,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsShaderStreamableTextureRef as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsShaderStreamableTextureRef as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_SHADERSTREAMABLETEXTUREREF_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsShaderStreamableTextureRef {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_SHADERSTREAMABLETEXTUREREF_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsSurfaceShaderDebugInfo {
}

pub trait RvmSerializedDbnsSurfaceShaderDebugInfoTrait: TypeObject {
}

impl RvmSerializedDbnsSurfaceShaderDebugInfoTrait for RvmSerializedDbnsSurfaceShaderDebugInfo {
}

pub static RVMSERIALIZEDDB_NS_SURFACESHADERDEBUGINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_SurfaceShaderDebugInfo",
    name_hash: 3056045798,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsSurfaceShaderDebugInfo as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsSurfaceShaderDebugInfo as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_SURFACESHADERDEBUGINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsSurfaceShaderDebugInfo {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_SURFACESHADERDEBUGINFO_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsSurfaceShader {
}

pub trait RvmSerializedDbnsSurfaceShaderTrait: TypeObject {
}

impl RvmSerializedDbnsSurfaceShaderTrait for RvmSerializedDbnsSurfaceShader {
}

pub static RVMSERIALIZEDDB_NS_SURFACESHADER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_SurfaceShader",
    name_hash: 538222361,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsSurfaceShader as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsSurfaceShader as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_SURFACESHADER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsSurfaceShader {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_SURFACESHADER_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsShaderStreamableTexture {
}

pub trait RvmSerializedDbnsShaderStreamableTextureTrait: TypeObject {
}

impl RvmSerializedDbnsShaderStreamableTextureTrait for RvmSerializedDbnsShaderStreamableTexture {
}

pub static RVMSERIALIZEDDB_NS_SHADERSTREAMABLETEXTURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ShaderStreamableTexture",
    name_hash: 1368255813,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsShaderStreamableTexture as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsShaderStreamableTexture as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_SHADERSTREAMABLETEXTURE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RvmSerializedDbnsShaderStreamableTexture {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_SHADERSTREAMABLETEXTURE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsShaderStreamableExternalTexture {
}

pub trait RvmSerializedDbnsShaderStreamableExternalTextureTrait: TypeObject {
}

impl RvmSerializedDbnsShaderStreamableExternalTextureTrait for RvmSerializedDbnsShaderStreamableExternalTexture {
}

pub static RVMSERIALIZEDDB_NS_SHADERSTREAMABLEEXTERNALTEXTURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ShaderStreamableExternalTexture",
    name_hash: 509590552,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsShaderStreamableExternalTexture as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsShaderStreamableExternalTexture as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_SHADERSTREAMABLEEXTERNALTEXTURE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RvmSerializedDbnsShaderStreamableExternalTexture {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_SHADERSTREAMABLEEXTERNALTEXTURE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsRvmFunctionInstanceRef {
}

pub trait RvmSerializedDbnsRvmFunctionInstanceRefTrait: TypeObject {
}

impl RvmSerializedDbnsRvmFunctionInstanceRefTrait for RvmSerializedDbnsRvmFunctionInstanceRef {
}

pub static RVMSERIALIZEDDB_NS_RVMFUNCTIONINSTANCEREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmFunctionInstanceRef",
    name_hash: 3527922742,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsRvmFunctionInstanceRef as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsRvmFunctionInstanceRef as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_RVMFUNCTIONINSTANCEREF_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsRvmFunctionInstanceRef {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RVMFUNCTIONINSTANCEREF_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsRvmFunctionInstance {
}

pub trait RvmSerializedDbnsRvmFunctionInstanceTrait: TypeObject {
}

impl RvmSerializedDbnsRvmFunctionInstanceTrait for RvmSerializedDbnsRvmFunctionInstance {
}

pub static RVMSERIALIZEDDB_NS_RVMFUNCTIONINSTANCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmFunctionInstance",
    name_hash: 2474392295,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsRvmFunctionInstance as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsRvmFunctionInstance as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_RVMFUNCTIONINSTANCE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsRvmFunctionInstance {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RVMFUNCTIONINSTANCE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsSettings {
}

pub trait RvmSerializedDbnsSettingsTrait: TypeObject {
}

impl RvmSerializedDbnsSettingsTrait for RvmSerializedDbnsSettings {
}

pub static RVMSERIALIZEDDB_NS_SETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Settings",
    name_hash: 3672624224,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsSettings as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsSettings as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_SETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsSettings {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_SETTINGS_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsRvmPermutationSet {
}

pub trait RvmSerializedDbnsRvmPermutationSetTrait: TypeObject {
}

impl RvmSerializedDbnsRvmPermutationSetTrait for RvmSerializedDbnsRvmPermutationSet {
}

pub static RVMSERIALIZEDDB_NS_RVMPERMUTATIONSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmPermutationSet",
    name_hash: 2063989016,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsRvmPermutationSet as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsRvmPermutationSet as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_RVMPERMUTATIONSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsRvmPermutationSet {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RVMPERMUTATIONSET_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsRvmPermutationLookupTable {
}

pub trait RvmSerializedDbnsRvmPermutationLookupTableTrait: TypeObject {
}

impl RvmSerializedDbnsRvmPermutationLookupTableTrait for RvmSerializedDbnsRvmPermutationLookupTable {
}

pub static RVMSERIALIZEDDB_NS_RVMPERMUTATIONLOOKUPTABLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmPermutationLookupTable",
    name_hash: 4260537446,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsRvmPermutationLookupTable as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsRvmPermutationLookupTable as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_RVMPERMUTATIONLOOKUPTABLE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsRvmPermutationLookupTable {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RVMPERMUTATIONLOOKUPTABLE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsRvmPermutationTree {
}

pub trait RvmSerializedDbnsRvmPermutationTreeTrait: TypeObject {
}

impl RvmSerializedDbnsRvmPermutationTreeTrait for RvmSerializedDbnsRvmPermutationTree {
}

pub static RVMSERIALIZEDDB_NS_RVMPERMUTATIONTREE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmPermutationTree",
    name_hash: 3687251644,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsRvmPermutationTree as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsRvmPermutationTree as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_RVMPERMUTATIONTREE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsRvmPermutationTree {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RVMPERMUTATIONTREE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsRvmPermutationRef {
}

pub trait RvmSerializedDbnsRvmPermutationRefTrait: TypeObject {
}

impl RvmSerializedDbnsRvmPermutationRefTrait for RvmSerializedDbnsRvmPermutationRef {
}

pub static RVMSERIALIZEDDB_NS_RVMPERMUTATIONREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmPermutationRef",
    name_hash: 2063981771,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsRvmPermutationRef as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsRvmPermutationRef as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_RVMPERMUTATIONREF_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsRvmPermutationRef {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RVMPERMUTATIONREF_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsRvmPermutation {
}

pub trait RvmSerializedDbnsRvmPermutationTrait: TypeObject {
}

impl RvmSerializedDbnsRvmPermutationTrait for RvmSerializedDbnsRvmPermutation {
}

pub static RVMSERIALIZEDDB_NS_RVMPERMUTATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmPermutation",
    name_hash: 122081018,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsRvmPermutation as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsRvmPermutation as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_RVMPERMUTATION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsRvmPermutation {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RVMPERMUTATION_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsRvmFunctionInputTableIndices {
}

pub trait RvmSerializedDbnsRvmFunctionInputTableIndicesTrait: TypeObject {
}

impl RvmSerializedDbnsRvmFunctionInputTableIndicesTrait for RvmSerializedDbnsRvmFunctionInputTableIndices {
}

pub static RVMSERIALIZEDDB_NS_RVMFUNCTIONINPUTTABLEINDICES_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmFunctionInputTableIndices",
    name_hash: 910996953,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsRvmFunctionInputTableIndices as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsRvmFunctionInputTableIndices as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_RVMFUNCTIONINPUTTABLEINDICES_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsRvmFunctionInputTableIndices {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RVMFUNCTIONINPUTTABLEINDICES_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsRvmDispatchDebugInfo {
}

pub trait RvmSerializedDbnsRvmDispatchDebugInfoTrait: TypeObject {
}

impl RvmSerializedDbnsRvmDispatchDebugInfoTrait for RvmSerializedDbnsRvmDispatchDebugInfo {
}

pub static RVMSERIALIZEDDB_NS_RVMDISPATCHDEBUGINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmDispatchDebugInfo",
    name_hash: 3866358275,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsRvmDispatchDebugInfo as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsRvmDispatchDebugInfo as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_RVMDISPATCHDEBUGINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsRvmDispatchDebugInfo {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RVMDISPATCHDEBUGINFO_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsRvmFunctionDebugInfo {
}

pub trait RvmSerializedDbnsRvmFunctionDebugInfoTrait: TypeObject {
}

impl RvmSerializedDbnsRvmFunctionDebugInfoTrait for RvmSerializedDbnsRvmFunctionDebugInfo {
}

pub static RVMSERIALIZEDDB_NS_RVMFUNCTIONDEBUGINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmFunctionDebugInfo",
    name_hash: 4247402385,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsRvmFunctionDebugInfo as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsRvmFunctionDebugInfo as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_RVMFUNCTIONDEBUGINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsRvmFunctionDebugInfo {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RVMFUNCTIONDEBUGINFO_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsRvmDispatch {
}

pub trait RvmSerializedDbnsRvmDispatchTrait: TypeObject {
}

impl RvmSerializedDbnsRvmDispatchTrait for RvmSerializedDbnsRvmDispatch {
}

pub static RVMSERIALIZEDDB_NS_RVMDISPATCH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmDispatch",
    name_hash: 2416284060,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsRvmDispatch as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsRvmDispatch as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_RVMDISPATCH_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsRvmDispatch {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RVMDISPATCH_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsRvmFunction {
}

pub trait RvmSerializedDbnsRvmFunctionTrait: TypeObject {
}

impl RvmSerializedDbnsRvmFunctionTrait for RvmSerializedDbnsRvmFunction {
}

pub static RVMSERIALIZEDDB_NS_RVMFUNCTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmFunction",
    name_hash: 336264270,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsRvmFunction as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsRvmFunction as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_RVMFUNCTION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsRvmFunction {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RVMFUNCTION_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsInstructionBatch {
}

pub trait RvmSerializedDbnsInstructionBatchTrait: TypeObject {
}

impl RvmSerializedDbnsInstructionBatchTrait for RvmSerializedDbnsInstructionBatch {
}

pub static RVMSERIALIZEDDB_NS_INSTRUCTIONBATCH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_InstructionBatch",
    name_hash: 228272417,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsInstructionBatch as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsInstructionBatch as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_INSTRUCTIONBATCH_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsInstructionBatch {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_INSTRUCTIONBATCH_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsRttiType {
}

pub trait RvmSerializedDbnsRttiTypeTrait: TypeObject {
}

impl RvmSerializedDbnsRttiTypeTrait for RvmSerializedDbnsRttiType {
}

pub static RVMSERIALIZEDDB_NS_RTTITYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RttiType",
    name_hash: 4051377062,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsRttiType as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsRttiType as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_RTTITYPE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsRttiType {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_RTTITYPE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsDefaultValueRef {
}

pub trait RvmSerializedDbnsDefaultValueRefTrait: TypeObject {
}

impl RvmSerializedDbnsDefaultValueRefTrait for RvmSerializedDbnsDefaultValueRef {
}

pub static RVMSERIALIZEDDB_NS_DEFAULTVALUEREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DefaultValueRef",
    name_hash: 32262548,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDefaultValueRef as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDefaultValueRef as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DEFAULTVALUEREF_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsDefaultValueRef {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DEFAULTVALUEREF_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsValueRef {
}

pub trait RvmSerializedDbnsValueRefTrait: TypeObject {
}

impl RvmSerializedDbnsValueRefTrait for RvmSerializedDbnsValueRef {
}

pub static RVMSERIALIZEDDB_NS_VALUEREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ValueRef",
    name_hash: 2009813119,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsValueRef as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsValueRef as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_VALUEREF_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsValueRef {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_VALUEREF_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsDefaultValueZeroMem {
}

pub trait RvmSerializedDbnsDefaultValueZeroMemTrait: TypeObject {
}

impl RvmSerializedDbnsDefaultValueZeroMemTrait for RvmSerializedDbnsDefaultValueZeroMem {
}

pub static RVMSERIALIZEDDB_NS_DEFAULTVALUEZEROMEM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DefaultValueZeroMem",
    name_hash: 2985951458,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDefaultValueZeroMem as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDefaultValueZeroMem as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DEFAULTVALUEZEROMEM_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDbnsDefaultValueZeroMem {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DEFAULTVALUEZEROMEM_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsDefaultValueStructLegacyData {
}

pub trait RvmSerializedDbnsDefaultValueStructLegacyDataTrait: TypeObject {
}

impl RvmSerializedDbnsDefaultValueStructLegacyDataTrait for RvmSerializedDbnsDefaultValueStructLegacyData {
}

pub static RVMSERIALIZEDDB_NS_DEFAULTVALUESTRUCTLEGACYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DefaultValueStructLegacyData",
    name_hash: 1525777655,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDefaultValueStructLegacyData as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDefaultValueStructLegacyData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DEFAULTVALUESTRUCTLEGACYDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RvmSerializedDbnsDefaultValueStructLegacyData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DEFAULTVALUESTRUCTLEGACYDATA_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsDefaultValueSimpleTexture {
}

pub trait RvmSerializedDbnsDefaultValueSimpleTextureTrait: TypeObject {
}

impl RvmSerializedDbnsDefaultValueSimpleTextureTrait for RvmSerializedDbnsDefaultValueSimpleTexture {
}

pub static RVMSERIALIZEDDB_NS_DEFAULTVALUESIMPLETEXTURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DefaultValueSimpleTexture",
    name_hash: 1288146548,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDefaultValueSimpleTexture as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDefaultValueSimpleTexture as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DEFAULTVALUESIMPLETEXTURE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDbnsDefaultValueSimpleTexture {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DEFAULTVALUESIMPLETEXTURE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsDefaultValueSimpleBuffer {
}

pub trait RvmSerializedDbnsDefaultValueSimpleBufferTrait: TypeObject {
}

impl RvmSerializedDbnsDefaultValueSimpleBufferTrait for RvmSerializedDbnsDefaultValueSimpleBuffer {
}

pub static RVMSERIALIZEDDB_NS_DEFAULTVALUESIMPLEBUFFER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DefaultValueSimpleBuffer",
    name_hash: 3784209643,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDefaultValueSimpleBuffer as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsDefaultValueSimpleBuffer as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DEFAULTVALUESIMPLEBUFFER_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDbnsDefaultValueSimpleBuffer {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DEFAULTVALUESIMPLEBUFFER_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsInstructionBatchRef {
}

pub trait RvmSerializedDbnsInstructionBatchRefTrait: TypeObject {
}

impl RvmSerializedDbnsInstructionBatchRefTrait for RvmSerializedDbnsInstructionBatchRef {
}

pub static RVMSERIALIZEDDB_NS_INSTRUCTIONBATCHREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_InstructionBatchRef",
    name_hash: 38261168,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsInstructionBatchRef as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsInstructionBatchRef as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_INSTRUCTIONBATCHREF_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsInstructionBatchRef {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_INSTRUCTIONBATCHREF_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsParamDbKeyRef {
}

pub trait RvmSerializedDbnsParamDbKeyRefTrait: TypeObject {
}

impl RvmSerializedDbnsParamDbKeyRefTrait for RvmSerializedDbnsParamDbKeyRef {
}

pub static RVMSERIALIZEDDB_NS_PARAMDBKEYREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbKeyRef",
    name_hash: 3752331210,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsParamDbKeyRef as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsParamDbKeyRef as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_PARAMDBKEYREF_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsParamDbKeyRef {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_PARAMDBKEYREF_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RvmSerializedDbnsParamDbKeyDesc {
}

pub trait RvmSerializedDbnsParamDbKeyDescTrait: TypeObject {
}

impl RvmSerializedDbnsParamDbKeyDescTrait for RvmSerializedDbnsParamDbKeyDesc {
}

pub static RVMSERIALIZEDDB_NS_PARAMDBKEYDESC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbKeyDesc",
    name_hash: 3568766346,
    flags: MemberInfoFlags::new(53321),
    module: "RvmCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsParamDbKeyDesc as Default>::default())),
            create_boxed: || Box::new(<RvmSerializedDbnsParamDbKeyDesc as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_PARAMDBKEYDESC_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsParamDbKeyDesc {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_PARAMDBKEYDESC_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 69259163,
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        super_class_offset: offset_of!(RvmDirectInputInstructionFactory, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmDirectInputInstructionFactory as Default>::default())),
            create_boxed: || Box::new(<RvmDirectInputInstructionFactory as Default>::default()),
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


pub static RVMDIRECTINPUTINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmDirectInputInstructionFactory-Array",
    name_hash: 1024624175,
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("RvmDirectInputInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2563153877,
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        super_class_offset: offset_of!(RvmConstantValueInstructionFactory, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmConstantValueInstructionFactory as Default>::default())),
            create_boxed: || Box::new(<RvmConstantValueInstructionFactory as Default>::default()),
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


pub static RVMCONSTANTVALUEINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmConstantValueInstructionFactory-Array",
    name_hash: 3307692001,
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("RvmConstantValueInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3513237389,
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMCOMMONDATABASELOADER_TYPE_INFO),
        super_class_offset: offset_of!(RvmCommonDatabaseLoaderImpl, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmCommonDatabaseLoaderImpl as Default>::default())),
            create_boxed: || Box::new(<RvmCommonDatabaseLoaderImpl as Default>::default()),
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


pub static RVMCOMMONDATABASELOADERIMPL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmCommonDatabaseLoaderImpl-Array",
    name_hash: 3074548537,
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("RvmCommonDatabaseLoaderImpl"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3632845717,
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::render::RVMDATABASELOADER_TYPE_INFO),
        super_class_offset: offset_of!(RvmCommonDatabaseLoader, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmCommonDatabaseLoader as Default>::default())),
            create_boxed: || Box::new(<RvmCommonDatabaseLoader as Default>::default()),
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


pub static RVMCOMMONDATABASELOADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmCommonDatabaseLoader-Array",
    name_hash: 3975504673,
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("RvmCommonDatabaseLoader"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2366860517,
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::render::RVMBACKEND_TYPE_INFO),
        super_class_offset: offset_of!(CommonRvmBackend, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CommonRvmBackend as Default>::default())),
            create_boxed: || Box::new(<CommonRvmBackend as Default>::default()),
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


pub static COMMONRVMBACKEND_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CommonRvmBackend-Array",
    name_hash: 2470917585,
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("CommonRvmBackend"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3826806812,
    flags: MemberInfoFlags::new(101),
    module: "RvmCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::render::RVMLEGACYDATABASE_TYPE_INFO),
        super_class_offset: offset_of!(BaseRvmDatabase, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BaseRvmDatabase as Default>::default())),
            create_boxed: || Box::new(<BaseRvmDatabase as Default>::default()),
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


pub static BASERVMDATABASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BaseRvmDatabase-Array",
    name_hash: 2828855336,
    flags: MemberInfoFlags::new(145),
    module: "RvmCommon",
    data: TypeInfoData::Array("BaseRvmDatabase"),
    array_type: None,
    alignment: 8,
};


