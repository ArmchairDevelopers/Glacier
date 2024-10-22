use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct RvmDx11Settings {
    pub _glacier_base: super::rvm_common::RvmCommonSettings,
    pub enabled: bool,
    pub merged_offsetted_buffers_size: u32,
}

pub trait RvmDx11SettingsTrait: super::rvm_common::RvmCommonSettingsTrait {
    fn enabled(&self) -> &bool;
    fn enabled_mut(&mut self) -> &mut bool;
    fn merged_offsetted_buffers_size(&self) -> &u32;
    fn merged_offsetted_buffers_size_mut(&mut self) -> &mut u32;
}

impl RvmDx11SettingsTrait for RvmDx11Settings {
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
    fn merged_offsetted_buffers_size(&self) -> &u32 {
        &self.merged_offsetted_buffers_size
    }
    fn merged_offsetted_buffers_size_mut(&mut self) -> &mut u32 {
        &mut self.merged_offsetted_buffers_size
    }
}

impl super::rvm_common::RvmCommonSettingsTrait for RvmDx11Settings {
    fn on_demand_building_enable(&self) -> &bool {
        self._glacier_base.on_demand_building_enable()
    }
    fn on_demand_building_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.on_demand_building_enable_mut()
    }
}

impl super::core::DataContainerTrait for RvmDx11Settings {
}

pub static RVMDX11SETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmDx11Settings",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx11Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMCOMMONSETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmDx11Settings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RvmDx11Settings, enabled),
            },
            FieldInfoData {
                name: "MergedOffsettedBuffersSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(RvmDx11Settings, merged_offsetted_buffers_size),
            },
        ],
    }),
    array_type: Some(RVMDX11SETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmDx11Settings {
    fn type_info(&self) -> &'static TypeInfo {
        RVMDX11SETTINGS_TYPE_INFO
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


pub static RVMDX11SETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmDx11Settings-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx11Pc",
    data: TypeInfoData::Array("RvmDx11Settings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Dx11RvmBackendConfig {
    pub _glacier_base: super::rvm_common::RvmBackendConfig,
}

pub trait Dx11RvmBackendConfigTrait: super::rvm_common::RvmBackendConfigTrait {
}

impl Dx11RvmBackendConfigTrait for Dx11RvmBackendConfig {
}

impl super::rvm_common::RvmBackendConfigTrait for Dx11RvmBackendConfig {
}

impl super::core::AssetTrait for Dx11RvmBackendConfig {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for Dx11RvmBackendConfig {
}

pub static DX11RVMBACKENDCONFIG_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmBackendConfig",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx11Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMBACKENDCONFIG_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx11RvmBackendConfig as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX11RVMBACKENDCONFIG_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11RvmBackendConfig {
    fn type_info(&self) -> &'static TypeInfo {
        DX11RVMBACKENDCONFIG_TYPE_INFO
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


pub static DX11RVMBACKENDCONFIG_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmBackendConfig-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx11Pc",
    data: TypeInfoData::Array("Dx11RvmBackendConfig"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Dx11ShaderDispatchDrawInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx11ShaderDispatchDrawInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx11ShaderDispatchDrawInstructionFactoryTrait for Dx11ShaderDispatchDrawInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx11ShaderDispatchDrawInstructionFactory {
}

pub static DX11SHADERDISPATCHDRAWINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11ShaderDispatchDrawInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx11Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx11ShaderDispatchDrawInstructionFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX11SHADERDISPATCHDRAWINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11ShaderDispatchDrawInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX11SHADERDISPATCHDRAWINSTRUCTIONFACTORY_TYPE_INFO
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


pub static DX11SHADERDISPATCHDRAWINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11ShaderDispatchDrawInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx11Pc",
    data: TypeInfoData::Array("Dx11ShaderDispatchDrawInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDbnsDx11ShaderDispatchDrawInstructionData {
}

pub trait RvmSerializedDbnsDx11ShaderDispatchDrawInstructionDataTrait: TypeObject {
}

impl RvmSerializedDbnsDx11ShaderDispatchDrawInstructionDataTrait for RvmSerializedDbnsDx11ShaderDispatchDrawInstructionData {
}

pub static RVMSERIALIZEDDB_NS_DX11SHADERDISPATCHDRAWINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11ShaderDispatchDrawInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx11ShaderDispatchDrawInstructionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX11SHADERDISPATCHDRAWINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDbnsDx11ShaderDispatchDrawInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11SHADERDISPATCHDRAWINSTRUCTIONDATA_TYPE_INFO
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
pub struct Dx11RvmLegacyVertexBufferConversionInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx11RvmLegacyVertexBufferConversionInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx11RvmLegacyVertexBufferConversionInstructionFactoryTrait for Dx11RvmLegacyVertexBufferConversionInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx11RvmLegacyVertexBufferConversionInstructionFactory {
}

pub static DX11RVMLEGACYVERTEXBUFFERCONVERSIONINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmLegacyVertexBufferConversionInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx11Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx11RvmLegacyVertexBufferConversionInstructionFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX11RVMLEGACYVERTEXBUFFERCONVERSIONINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11RvmLegacyVertexBufferConversionInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX11RVMLEGACYVERTEXBUFFERCONVERSIONINSTRUCTIONFACTORY_TYPE_INFO
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


pub static DX11RVMLEGACYVERTEXBUFFERCONVERSIONINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmLegacyVertexBufferConversionInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx11Pc",
    data: TypeInfoData::Array("Dx11RvmLegacyVertexBufferConversionInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDbnsDx11LegacyVertexBufferConversionInstructionData {
}

pub trait RvmSerializedDbnsDx11LegacyVertexBufferConversionInstructionDataTrait: TypeObject {
}

impl RvmSerializedDbnsDx11LegacyVertexBufferConversionInstructionDataTrait for RvmSerializedDbnsDx11LegacyVertexBufferConversionInstructionData {
}

pub static RVMSERIALIZEDDB_NS_DX11LEGACYVERTEXBUFFERCONVERSIONINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11LegacyVertexBufferConversionInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx11LegacyVertexBufferConversionInstructionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX11LEGACYVERTEXBUFFERCONVERSIONINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsDx11LegacyVertexBufferConversionInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11LEGACYVERTEXBUFFERCONVERSIONINSTRUCTIONDATA_TYPE_INFO
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
pub struct Dx11RvmViewStateInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx11RvmViewStateInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx11RvmViewStateInstructionFactoryTrait for Dx11RvmViewStateInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx11RvmViewStateInstructionFactory {
}

pub static DX11RVMVIEWSTATEINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmViewStateInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx11Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx11RvmViewStateInstructionFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX11RVMVIEWSTATEINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11RvmViewStateInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX11RVMVIEWSTATEINSTRUCTIONFACTORY_TYPE_INFO
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


pub static DX11RVMVIEWSTATEINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmViewStateInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx11Pc",
    data: TypeInfoData::Array("Dx11RvmViewStateInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RvmSerializedDbnsDx11ViewStateInstructionData {
}

pub trait RvmSerializedDbnsDx11ViewStateInstructionDataTrait: TypeObject {
}

impl RvmSerializedDbnsDx11ViewStateInstructionDataTrait for RvmSerializedDbnsDx11ViewStateInstructionData {
}

pub static RVMSERIALIZEDDB_NS_DX11VIEWSTATEINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11ViewStateInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx11ViewStateInstructionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX11VIEWSTATEINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDbnsDx11ViewStateInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11VIEWSTATEINSTRUCTIONDATA_TYPE_INFO
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
pub struct D3D11CONSERVATIVERASTERIZATIONMODE {
}

pub trait D3D11CONSERVATIVERASTERIZATIONMODETrait: TypeObject {
}

impl D3D11CONSERVATIVERASTERIZATIONMODETrait for D3D11CONSERVATIVERASTERIZATIONMODE {
}

pub static D3D11_CONSERVATIVE_RASTERIZATION_MODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D11_CONSERVATIVE_RASTERIZATION_MODE",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<D3D11CONSERVATIVERASTERIZATIONMODE as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::D3D11_CONSERVATIVE_RASTERIZATION_MODE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for D3D11CONSERVATIVERASTERIZATIONMODE {
    fn type_info(&self) -> &'static TypeInfo {
        D3D11_CONSERVATIVE_RASTERIZATION_MODE_TYPE_INFO
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
pub struct Dx11RvmViewports {
}

pub trait Dx11RvmViewportsTrait: TypeObject {
}

impl Dx11RvmViewportsTrait for Dx11RvmViewports {
}

pub static DX11RVMVIEWPORTS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmViewports",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx11RvmViewports as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::DX11RVMVIEWPORTS_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for Dx11RvmViewports {
    fn type_info(&self) -> &'static TypeInfo {
        DX11RVMVIEWPORTS_TYPE_INFO
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
pub struct Dx11RvmScissorRects {
}

pub trait Dx11RvmScissorRectsTrait: TypeObject {
}

impl Dx11RvmScissorRectsTrait for Dx11RvmScissorRects {
}

pub static DX11RVMSCISSORRECTS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmScissorRects",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx11RvmScissorRects as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::DX11RVMSCISSORRECTS_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for Dx11RvmScissorRects {
    fn type_info(&self) -> &'static TypeInfo {
        DX11RVMSCISSORRECTS_TYPE_INFO
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
pub struct Dx11RvmDepthStencilState {
}

pub trait Dx11RvmDepthStencilStateTrait: TypeObject {
}

impl Dx11RvmDepthStencilStateTrait for Dx11RvmDepthStencilState {
}

pub static DX11RVMDEPTHSTENCILSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmDepthStencilState",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx11RvmDepthStencilState as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::DX11RVMDEPTHSTENCILSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for Dx11RvmDepthStencilState {
    fn type_info(&self) -> &'static TypeInfo {
        DX11RVMDEPTHSTENCILSTATE_TYPE_INFO
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
pub struct Dx11RvmBlendState {
}

pub trait Dx11RvmBlendStateTrait: TypeObject {
}

impl Dx11RvmBlendStateTrait for Dx11RvmBlendState {
}

pub static DX11RVMBLENDSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmBlendState",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx11RvmBlendState as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::DX11RVMBLENDSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for Dx11RvmBlendState {
    fn type_info(&self) -> &'static TypeInfo {
        DX11RVMBLENDSTATE_TYPE_INFO
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
pub struct D3D11CULLMODE {
}

pub trait D3D11CULLMODETrait: TypeObject {
}

impl D3D11CULLMODETrait for D3D11CULLMODE {
}

pub static D3D11_CULL_MODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D11_CULL_MODE",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<D3D11CULLMODE as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::D3D11_CULL_MODE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for D3D11CULLMODE {
    fn type_info(&self) -> &'static TypeInfo {
        D3D11_CULL_MODE_TYPE_INFO
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
pub struct D3D11FILLMODE {
}

pub trait D3D11FILLMODETrait: TypeObject {
}

impl D3D11FILLMODETrait for D3D11FILLMODE {
}

pub static D3D11_FILL_MODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D11_FILL_MODE",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<D3D11FILLMODE as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::D3D11_FILL_MODE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for D3D11FILLMODE {
    fn type_info(&self) -> &'static TypeInfo {
        D3D11_FILL_MODE_TYPE_INFO
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
pub struct Dx11RvmSampler {
}

pub trait Dx11RvmSamplerTrait: TypeObject {
}

impl Dx11RvmSamplerTrait for Dx11RvmSampler {
}

pub static DX11RVMSAMPLER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmSampler",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx11RvmSampler as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::DX11RVMSAMPLER_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for Dx11RvmSampler {
    fn type_info(&self) -> &'static TypeInfo {
        DX11RVMSAMPLER_TYPE_INFO
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
pub struct Dx11RvmVsShader {
}

pub trait Dx11RvmVsShaderTrait: TypeObject {
}

impl Dx11RvmVsShaderTrait for Dx11RvmVsShader {
}

pub static DX11RVMVSSHADER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmVsShader",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx11RvmVsShader as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::DX11RVMVSSHADER_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for Dx11RvmVsShader {
    fn type_info(&self) -> &'static TypeInfo {
        DX11RVMVSSHADER_TYPE_INFO
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
pub struct Dx11RvmPsShader {
}

pub trait Dx11RvmPsShaderTrait: TypeObject {
}

impl Dx11RvmPsShaderTrait for Dx11RvmPsShader {
}

pub static DX11RVMPSSHADER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmPsShader",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx11RvmPsShader as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::DX11RVMPSSHADER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11RvmPsShader {
    fn type_info(&self) -> &'static TypeInfo {
        DX11RVMPSSHADER_TYPE_INFO
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
pub struct Dx11RvmHsShader {
}

pub trait Dx11RvmHsShaderTrait: TypeObject {
}

impl Dx11RvmHsShaderTrait for Dx11RvmHsShader {
}

pub static DX11RVMHSSHADER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmHsShader",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx11RvmHsShader as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::DX11RVMHSSHADER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11RvmHsShader {
    fn type_info(&self) -> &'static TypeInfo {
        DX11RVMHSSHADER_TYPE_INFO
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
pub struct Dx11RvmDsShader {
}

pub trait Dx11RvmDsShaderTrait: TypeObject {
}

impl Dx11RvmDsShaderTrait for Dx11RvmDsShader {
}

pub static DX11RVMDSSHADER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmDsShader",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx11RvmDsShader as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::DX11RVMDSSHADER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11RvmDsShader {
    fn type_info(&self) -> &'static TypeInfo {
        DX11RVMDSSHADER_TYPE_INFO
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
pub struct RvmSerializedDbnsDx11ByteCodeElement {
}

pub trait RvmSerializedDbnsDx11ByteCodeElementTrait: TypeObject {
}

impl RvmSerializedDbnsDx11ByteCodeElementTrait for RvmSerializedDbnsDx11ByteCodeElement {
}

pub static RVMSERIALIZEDDB_NS_DX11BYTECODEELEMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11ByteCodeElement",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx11ByteCodeElement as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX11BYTECODEELEMENT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RvmSerializedDbnsDx11ByteCodeElement {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11BYTECODEELEMENT_TYPE_INFO
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
pub struct RvmSerializedDbnsDx11DsShader {
}

pub trait RvmSerializedDbnsDx11DsShaderTrait: TypeObject {
}

impl RvmSerializedDbnsDx11DsShaderTrait for RvmSerializedDbnsDx11DsShader {
}

pub static RVMSERIALIZEDDB_NS_DX11DSSHADER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11DsShader",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx11DsShader as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX11DSSHADER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsDx11DsShader {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11DSSHADER_TYPE_INFO
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
pub struct RvmSerializedDbnsDx11HsShader {
}

pub trait RvmSerializedDbnsDx11HsShaderTrait: TypeObject {
}

impl RvmSerializedDbnsDx11HsShaderTrait for RvmSerializedDbnsDx11HsShader {
}

pub static RVMSERIALIZEDDB_NS_DX11HSSHADER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11HsShader",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx11HsShader as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX11HSSHADER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsDx11HsShader {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11HSSHADER_TYPE_INFO
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
pub struct RvmSerializedDbnsDx11PsShader {
}

pub trait RvmSerializedDbnsDx11PsShaderTrait: TypeObject {
}

impl RvmSerializedDbnsDx11PsShaderTrait for RvmSerializedDbnsDx11PsShader {
}

pub static RVMSERIALIZEDDB_NS_DX11PSSHADER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11PsShader",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx11PsShader as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX11PSSHADER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsDx11PsShader {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11PSSHADER_TYPE_INFO
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
pub struct RvmSerializedDbnsDx11VsShader {
}

pub trait RvmSerializedDbnsDx11VsShaderTrait: TypeObject {
}

impl RvmSerializedDbnsDx11VsShaderTrait for RvmSerializedDbnsDx11VsShader {
}

pub static RVMSERIALIZEDDB_NS_DX11VSSHADER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11VsShader",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx11VsShader as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX11VSSHADER_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RvmSerializedDbnsDx11VsShader {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11VSSHADER_TYPE_INFO
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
pub struct RvmSerializedDbnsDx11InputElement {
}

pub trait RvmSerializedDbnsDx11InputElementTrait: TypeObject {
}

impl RvmSerializedDbnsDx11InputElementTrait for RvmSerializedDbnsDx11InputElement {
}

pub static RVMSERIALIZEDDB_NS_DX11INPUTELEMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11InputElement",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx11InputElement as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX11INPUTELEMENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsDx11InputElement {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11INPUTELEMENT_TYPE_INFO
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
pub struct RvmSerializedDbnsDx11Sampler {
}

pub trait RvmSerializedDbnsDx11SamplerTrait: TypeObject {
}

impl RvmSerializedDbnsDx11SamplerTrait for RvmSerializedDbnsDx11Sampler {
}

pub static RVMSERIALIZEDDB_NS_DX11SAMPLER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11Sampler",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx11Sampler as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX11SAMPLER_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RvmSerializedDbnsDx11Sampler {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11SAMPLER_TYPE_INFO
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
pub struct RvmSerializedDbnsDx11BlendStateData {
}

pub trait RvmSerializedDbnsDx11BlendStateDataTrait: TypeObject {
}

impl RvmSerializedDbnsDx11BlendStateDataTrait for RvmSerializedDbnsDx11BlendStateData {
}

pub static RVMSERIALIZEDDB_NS_DX11BLENDSTATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11BlendStateData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx11BlendStateData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX11BLENDSTATEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsDx11BlendStateData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11BLENDSTATEDATA_TYPE_INFO
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
pub struct RvmSerializedDbnsDx11SerializedBlendState {
}

pub trait RvmSerializedDbnsDx11SerializedBlendStateTrait: TypeObject {
}

impl RvmSerializedDbnsDx11SerializedBlendStateTrait for RvmSerializedDbnsDx11SerializedBlendState {
}

pub static RVMSERIALIZEDDB_NS_DX11SERIALIZEDBLENDSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11SerializedBlendState",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx11SerializedBlendState as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX11SERIALIZEDBLENDSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RvmSerializedDbnsDx11SerializedBlendState {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11SERIALIZEDBLENDSTATE_TYPE_INFO
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
pub struct RvmSerializedDbnsDx11TextureConversionInstructionData {
}

pub trait RvmSerializedDbnsDx11TextureConversionInstructionDataTrait: TypeObject {
}

impl RvmSerializedDbnsDx11TextureConversionInstructionDataTrait for RvmSerializedDbnsDx11TextureConversionInstructionData {
}

pub static RVMSERIALIZEDDB_NS_DX11TEXTURECONVERSIONINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11TextureConversionInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx11TextureConversionInstructionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX11TEXTURECONVERSIONINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsDx11TextureConversionInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11TEXTURECONVERSIONINSTRUCTIONDATA_TYPE_INFO
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
pub struct RvmSerializedDbnsDx11BufferConversionInstructionData {
}

pub trait RvmSerializedDbnsDx11BufferConversionInstructionDataTrait: TypeObject {
}

impl RvmSerializedDbnsDx11BufferConversionInstructionDataTrait for RvmSerializedDbnsDx11BufferConversionInstructionData {
}

pub static RVMSERIALIZEDDB_NS_DX11BUFFERCONVERSIONINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11BufferConversionInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx11BufferConversionInstructionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX11BUFFERCONVERSIONINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsDx11BufferConversionInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11BUFFERCONVERSIONINSTRUCTIONDATA_TYPE_INFO
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
pub struct RvmSerializedDbnsDx11LegacyDrawStateBuilderData {
}

pub trait RvmSerializedDbnsDx11LegacyDrawStateBuilderDataTrait: TypeObject {
}

impl RvmSerializedDbnsDx11LegacyDrawStateBuilderDataTrait for RvmSerializedDbnsDx11LegacyDrawStateBuilderData {
}

pub static RVMSERIALIZEDDB_NS_DX11LEGACYDRAWSTATEBUILDERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11LegacyDrawStateBuilderData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx11LegacyDrawStateBuilderData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX11LEGACYDRAWSTATEBUILDERDATA_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSerializedDbnsDx11LegacyDrawStateBuilderData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11LEGACYDRAWSTATEBUILDERDATA_TYPE_INFO
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
pub struct RvmSerializedDbnsDx11ShaderDispatchLegacyDrawInstructionData {
}

pub trait RvmSerializedDbnsDx11ShaderDispatchLegacyDrawInstructionDataTrait: TypeObject {
}

impl RvmSerializedDbnsDx11ShaderDispatchLegacyDrawInstructionDataTrait for RvmSerializedDbnsDx11ShaderDispatchLegacyDrawInstructionData {
}

pub static RVMSERIALIZEDDB_NS_DX11SHADERDISPATCHLEGACYDRAWINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11ShaderDispatchLegacyDrawInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx11ShaderDispatchLegacyDrawInstructionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX11SHADERDISPATCHLEGACYDRAWINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RvmSerializedDbnsDx11ShaderDispatchLegacyDrawInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11SHADERDISPATCHLEGACYDRAWINSTRUCTIONDATA_TYPE_INFO
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
pub struct RvmSerializedDbnsDx11ApplyParametersInstructionData {
}

pub trait RvmSerializedDbnsDx11ApplyParametersInstructionDataTrait: TypeObject {
}

impl RvmSerializedDbnsDx11ApplyParametersInstructionDataTrait for RvmSerializedDbnsDx11ApplyParametersInstructionData {
}

pub static RVMSERIALIZEDDB_NS_DX11APPLYPARAMETERSINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11ApplyParametersInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx11ApplyParametersInstructionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX11APPLYPARAMETERSINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsDx11ApplyParametersInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11APPLYPARAMETERSINSTRUCTIONDATA_TYPE_INFO
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
pub struct RvmSerializedDbnsDx11ApplyParametersBlock {
}

pub trait RvmSerializedDbnsDx11ApplyParametersBlockTrait: TypeObject {
}

impl RvmSerializedDbnsDx11ApplyParametersBlockTrait for RvmSerializedDbnsDx11ApplyParametersBlock {
}

pub static RVMSERIALIZEDDB_NS_DX11APPLYPARAMETERSBLOCK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11ApplyParametersBlock",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx11ApplyParametersBlock as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX11APPLYPARAMETERSBLOCK_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsDx11ApplyParametersBlock {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11APPLYPARAMETERSBLOCK_TYPE_INFO
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
pub struct RvmSerializedDbnsDx11DispatchInstructionData {
}

pub trait RvmSerializedDbnsDx11DispatchInstructionDataTrait: TypeObject {
}

impl RvmSerializedDbnsDx11DispatchInstructionDataTrait for RvmSerializedDbnsDx11DispatchInstructionData {
}

pub static RVMSERIALIZEDDB_NS_DX11DISPATCHINSTRUCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11DispatchInstructionData",
    flags: MemberInfoFlags::new(53321),
    module: "RvmDx11Pc",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RvmSerializedDbnsDx11DispatchInstructionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(super::core::RVMSERIALIZEDDB_NS_DX11DISPATCHINSTRUCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSerializedDbnsDx11DispatchInstructionData {
    fn type_info(&self) -> &'static TypeInfo {
        RVMSERIALIZEDDB_NS_DX11DISPATCHINSTRUCTIONDATA_TYPE_INFO
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
pub struct Dx11RvmLegacyDrawStateBuilderInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx11RvmLegacyDrawStateBuilderInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx11RvmLegacyDrawStateBuilderInstructionFactoryTrait for Dx11RvmLegacyDrawStateBuilderInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx11RvmLegacyDrawStateBuilderInstructionFactory {
}

pub static DX11RVMLEGACYDRAWSTATEBUILDERINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmLegacyDrawStateBuilderInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx11Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx11RvmLegacyDrawStateBuilderInstructionFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX11RVMLEGACYDRAWSTATEBUILDERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11RvmLegacyDrawStateBuilderInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX11RVMLEGACYDRAWSTATEBUILDERINSTRUCTIONFACTORY_TYPE_INFO
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


pub static DX11RVMLEGACYDRAWSTATEBUILDERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmLegacyDrawStateBuilderInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx11Pc",
    data: TypeInfoData::Array("Dx11RvmLegacyDrawStateBuilderInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Dx11RvmTextureConversionInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx11RvmTextureConversionInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx11RvmTextureConversionInstructionFactoryTrait for Dx11RvmTextureConversionInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx11RvmTextureConversionInstructionFactory {
}

pub static DX11RVMTEXTURECONVERSIONINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmTextureConversionInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx11Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx11RvmTextureConversionInstructionFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX11RVMTEXTURECONVERSIONINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11RvmTextureConversionInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX11RVMTEXTURECONVERSIONINSTRUCTIONFACTORY_TYPE_INFO
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


pub static DX11RVMTEXTURECONVERSIONINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmTextureConversionInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx11Pc",
    data: TypeInfoData::Array("Dx11RvmTextureConversionInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Dx11RvmBufferConversionInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx11RvmBufferConversionInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx11RvmBufferConversionInstructionFactoryTrait for Dx11RvmBufferConversionInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx11RvmBufferConversionInstructionFactory {
}

pub static DX11RVMBUFFERCONVERSIONINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmBufferConversionInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx11Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx11RvmBufferConversionInstructionFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX11RVMBUFFERCONVERSIONINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11RvmBufferConversionInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX11RVMBUFFERCONVERSIONINSTRUCTIONFACTORY_TYPE_INFO
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


pub static DX11RVMBUFFERCONVERSIONINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmBufferConversionInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx11Pc",
    data: TypeInfoData::Array("Dx11RvmBufferConversionInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Dx11RvmInstanceBufferWriterInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx11RvmInstanceBufferWriterInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx11RvmInstanceBufferWriterInstructionFactoryTrait for Dx11RvmInstanceBufferWriterInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx11RvmInstanceBufferWriterInstructionFactory {
}

pub static DX11RVMINSTANCEBUFFERWRITERINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmInstanceBufferWriterInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx11Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx11RvmInstanceBufferWriterInstructionFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX11RVMINSTANCEBUFFERWRITERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11RvmInstanceBufferWriterInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX11RVMINSTANCEBUFFERWRITERINSTRUCTIONFACTORY_TYPE_INFO
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


pub static DX11RVMINSTANCEBUFFERWRITERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmInstanceBufferWriterInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx11Pc",
    data: TypeInfoData::Array("Dx11RvmInstanceBufferWriterInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Dx11RvmConstantBufferWriterInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx11RvmConstantBufferWriterInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx11RvmConstantBufferWriterInstructionFactoryTrait for Dx11RvmConstantBufferWriterInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx11RvmConstantBufferWriterInstructionFactory {
}

pub static DX11RVMCONSTANTBUFFERWRITERINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmConstantBufferWriterInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx11Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx11RvmConstantBufferWriterInstructionFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX11RVMCONSTANTBUFFERWRITERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11RvmConstantBufferWriterInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX11RVMCONSTANTBUFFERWRITERINSTRUCTIONFACTORY_TYPE_INFO
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


pub static DX11RVMCONSTANTBUFFERWRITERINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmConstantBufferWriterInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx11Pc",
    data: TypeInfoData::Array("Dx11RvmConstantBufferWriterInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Dx11RvmApplyStateInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx11RvmApplyStateInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx11RvmApplyStateInstructionFactoryTrait for Dx11RvmApplyStateInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx11RvmApplyStateInstructionFactory {
}

pub static DX11RVMAPPLYSTATEINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmApplyStateInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx11Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx11RvmApplyStateInstructionFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX11RVMAPPLYSTATEINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11RvmApplyStateInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX11RVMAPPLYSTATEINSTRUCTIONFACTORY_TYPE_INFO
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


pub static DX11RVMAPPLYSTATEINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmApplyStateInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx11Pc",
    data: TypeInfoData::Array("Dx11RvmApplyStateInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Dx11RvmDispatchInstructionFactory {
    pub _glacier_base: super::rvm_common::RvmInstructionFactoryBase,
}

pub trait Dx11RvmDispatchInstructionFactoryTrait: super::rvm_common::RvmInstructionFactoryBaseTrait {
}

impl Dx11RvmDispatchInstructionFactoryTrait for Dx11RvmDispatchInstructionFactory {
}

impl super::rvm_common::RvmInstructionFactoryBaseTrait for Dx11RvmDispatchInstructionFactory {
}

pub static DX11RVMDISPATCHINSTRUCTIONFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmDispatchInstructionFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx11Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::RVMINSTRUCTIONFACTORYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx11RvmDispatchInstructionFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX11RVMDISPATCHINSTRUCTIONFACTORY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11RvmDispatchInstructionFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX11RVMDISPATCHINSTRUCTIONFACTORY_TYPE_INFO
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


pub static DX11RVMDISPATCHINSTRUCTIONFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmDispatchInstructionFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx11Pc",
    data: TypeInfoData::Array("Dx11RvmDispatchInstructionFactory"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Dx11RvmDatabase {
    pub _glacier_base: super::rvm_common::BaseRvmDatabase,
}

pub trait Dx11RvmDatabaseTrait: super::rvm_common::BaseRvmDatabaseTrait {
}

impl Dx11RvmDatabaseTrait for Dx11RvmDatabase {
}

impl super::rvm_common::BaseRvmDatabaseTrait for Dx11RvmDatabase {
}

impl super::render::RvmLegacyDatabaseTrait for Dx11RvmDatabase {
}

impl super::render::RvmDatabaseTrait for Dx11RvmDatabase {
}

impl super::core::IResourceObjectTrait for Dx11RvmDatabase {
}

pub static DX11RVMDATABASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmDatabase",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx11Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::BASERVMDATABASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx11RvmDatabase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX11RVMDATABASE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11RvmDatabase {
    fn type_info(&self) -> &'static TypeInfo {
        DX11RVMDATABASE_TYPE_INFO
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


pub static DX11RVMDATABASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmDatabase-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx11Pc",
    data: TypeInfoData::Array("Dx11RvmDatabase"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Dx11RvmBackend {
    pub _glacier_base: super::rvm_common::CommonRvmBackend,
}

pub trait Dx11RvmBackendTrait: super::rvm_common::CommonRvmBackendTrait {
}

impl Dx11RvmBackendTrait for Dx11RvmBackend {
}

impl super::rvm_common::CommonRvmBackendTrait for Dx11RvmBackend {
}

impl super::render::RvmBackendTrait for Dx11RvmBackend {
}

pub static DX11RVMBACKEND_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmBackend",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx11Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::rvm_common::COMMONRVMBACKEND_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx11RvmBackend as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX11RVMBACKEND_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Dx11RvmBackend {
    fn type_info(&self) -> &'static TypeInfo {
        DX11RVMBACKEND_TYPE_INFO
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


pub static DX11RVMBACKEND_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmBackend-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx11Pc",
    data: TypeInfoData::Array("Dx11RvmBackend"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Dx11RvmBackendFactory {
    pub _glacier_base: super::render::RvmBackendFactory,
}

pub trait Dx11RvmBackendFactoryTrait: super::render::RvmBackendFactoryTrait {
}

impl Dx11RvmBackendFactoryTrait for Dx11RvmBackendFactory {
}

impl super::render::RvmBackendFactoryTrait for Dx11RvmBackendFactory {
}

pub static DX11RVMBACKENDFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmBackendFactory",
    flags: MemberInfoFlags::new(101),
    module: "RvmDx11Pc",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::render::RVMBACKENDFACTORY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Dx11RvmBackendFactory as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DX11RVMBACKENDFACTORY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Dx11RvmBackendFactory {
    fn type_info(&self) -> &'static TypeInfo {
        DX11RVMBACKENDFACTORY_TYPE_INFO
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


pub static DX11RVMBACKENDFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmBackendFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "RvmDx11Pc",
    data: TypeInfoData::Array("Dx11RvmBackendFactory"),
    array_type: None,
    alignment: 8,
};


