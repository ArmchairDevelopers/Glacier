use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_l_m_s_regular_grid_types(registry: &mut TypeRegistry) {
    registry.register_type(LMSREGULARGRIDRESCALENODEFILTERINGMETHOD_TYPE_INFO);
    registry.register_type(LMSREGULARGRIDRESCALENODEFILTERINGMETHOD_ARRAY_TYPE_INFO);
    registry.register_type(LMSREGULARGRIDBINDFLAGS_TYPE_INFO);
    registry.register_type(LMSREGULARGRIDBINDFLAGS_ARRAY_TYPE_INFO);
    registry.register_type(LMSREGULARGRIDOUTPUTATTRIBUTEMODE_TYPE_INFO);
    registry.register_type(LMSREGULARGRIDOUTPUTATTRIBUTEMODE_ARRAY_TYPE_INFO);
    registry.register_type(LMSREGULARGRIDSEGMENTTARGETSURFACEIMPCPUARRAYBASE_TYPE_INFO);
    registry.register_type(LMSREGULARGRIDSEGMENTTARGETSURFACEIMPCPUARRAYBASE_ARRAY_TYPE_INFO);
    registry.register_type(LMSDYNAPACKRUNTIME_TYPE_INFO);
    registry.register_type(LMSDYNAPACKRUNTIME_ARRAY_TYPE_INFO);
    registry.register_type(LMSREGULARGRIDRUNTIME_TYPE_INFO);
    registry.register_type(LMSREGULARGRIDRUNTIME_ARRAY_TYPE_INFO);
    registry.register_type(LMSREGULARGRIDCODECRUNTIME_TYPE_INFO);
    registry.register_type(LMSREGULARGRIDCODECRUNTIME_ARRAY_TYPE_INFO);
    registry.register_type(LMSREGULARGRIDSURFACECPUARRAYIMPDECODERTEMP_TYPE_INFO);
    registry.register_type(LMSREGULARGRIDSURFACECPUARRAYIMPDECODERTEMP_ARRAY_TYPE_INFO);
    registry.register_type(LMSREGULARGRIDSURFACEGPUTEXTURE_TYPE_INFO);
    registry.register_type(LMSREGULARGRIDSURFACEGPUTEXTURE_ARRAY_TYPE_INFO);
    registry.register_type(LMSREGULARGRIDSURFACEGPUBUFFER_TYPE_INFO);
    registry.register_type(LMSREGULARGRIDSURFACEGPUBUFFER_ARRAY_TYPE_INFO);
    registry.register_type(LMSREGULARGRIDSURFACECPUARRAY_TYPE_INFO);
    registry.register_type(LMSREGULARGRIDSURFACECPUARRAY_ARRAY_TYPE_INFO);
    registry.register_type(LMSREGULARGRIDSURFACE_TYPE_INFO);
    registry.register_type(LMSREGULARGRIDSURFACE_ARRAY_TYPE_INFO);
    registry.register_type(LMSREGULARGRIDCPUARRAY_TYPE_INFO);
    registry.register_type(LMSREGULARGRIDCPUARRAY_ARRAY_TYPE_INFO);
    registry.register_type(LMSEFFECTSDATAARRAY_TYPE_INFO);
    registry.register_type(LMSEFFECTSDATAARRAY_ARRAY_TYPE_INFO);
    registry.register_type(LMSREGULARGRIDDEFAULTCODECRUNTIME_TYPE_INFO);
    registry.register_type(LMSREGULARGRIDDEFAULTCODECRUNTIME_ARRAY_TYPE_INFO);
    registry.register_type(LMSREGULARGRIDVP6CODECRUNTIME_TYPE_INFO);
    registry.register_type(LMSREGULARGRIDVP6CODECRUNTIME_ARRAY_TYPE_INFO);
}

#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum LMSRegularGridRescaleNodeFilteringMethod {
    #[default]
    LMSRegularGridRescaleNodeFilteringMethod_Bilinear = 0,
    LMSRegularGridRescaleNodeFilteringMethod_Bicubic = 1,
    LMSRegularGridRescaleNodeFilteringMethod_SingleAverageColor = 2,
}

pub static LMSREGULARGRIDRESCALENODEFILTERINGMETHOD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridRescaleNodeFilteringMethod",
    flags: MemberInfoFlags::new(49429),
    module: "LMSRegularGrid",
    data: TypeInfoData::Enum,
    array_type: Some(LMSREGULARGRIDRESCALENODEFILTERINGMETHOD_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LMSRegularGridRescaleNodeFilteringMethod {
    fn type_info(&self) -> &'static TypeInfo {
        LMSREGULARGRIDRESCALENODEFILTERINGMETHOD_TYPE_INFO
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


pub static LMSREGULARGRIDRESCALENODEFILTERINGMETHOD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridRescaleNodeFilteringMethod-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSRegularGrid",
    data: TypeInfoData::Array("LMSRegularGridRescaleNodeFilteringMethod"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum LMSRegularGridBindFlags {
    #[default]
    LMSRegularGridBindFlags_None = 0,
    LMSRegularGridBindFlags_VertexBuffer = 1,
    LMSRegularGridBindFlags_IndexBuffer = 2,
    LMSRegularGridBindFlags_ShaderResource = 4,
    LMSRegularGridBindFlags_ConstantBuffer = 32,
    LMSRegularGridBindFlags_UnorderedAccess = 64,
    LMSRegularGridBindFlags_MaterialRange = 128,
}

pub static LMSREGULARGRIDBINDFLAGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridBindFlags",
    flags: MemberInfoFlags::new(49429),
    module: "LMSRegularGrid",
    data: TypeInfoData::Enum,
    array_type: Some(LMSREGULARGRIDBINDFLAGS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LMSRegularGridBindFlags {
    fn type_info(&self) -> &'static TypeInfo {
        LMSREGULARGRIDBINDFLAGS_TYPE_INFO
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


pub static LMSREGULARGRIDBINDFLAGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridBindFlags-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSRegularGrid",
    data: TypeInfoData::Array("LMSRegularGridBindFlags"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum LMSRegularGridOutputAttributeMode {
    #[default]
    LMSRegularGridOutputAttributeMode_GpuTexture = 0,
    LMSRegularGridOutputAttributeMode_GpuBuffer = 1,
    LMSRegularGridOutputAttributeMode_CpuArray = 2,
    LMSRegularGridOutputAttributeMode_EmitterArray = 3,
}

pub static LMSREGULARGRIDOUTPUTATTRIBUTEMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridOutputAttributeMode",
    flags: MemberInfoFlags::new(49429),
    module: "LMSRegularGrid",
    data: TypeInfoData::Enum,
    array_type: Some(LMSREGULARGRIDOUTPUTATTRIBUTEMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LMSRegularGridOutputAttributeMode {
    fn type_info(&self) -> &'static TypeInfo {
        LMSREGULARGRIDOUTPUTATTRIBUTEMODE_TYPE_INFO
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


pub static LMSREGULARGRIDOUTPUTATTRIBUTEMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridOutputAttributeMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSRegularGrid",
    data: TypeInfoData::Array("LMSRegularGridOutputAttributeMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LMSRegularGridSegmentTargetSurfaceImpCpuArrayBase {
    pub _glacier_base: LMSRegularGridSurfaceCpuArray,
}

pub trait LMSRegularGridSegmentTargetSurfaceImpCpuArrayBaseTrait: LMSRegularGridSurfaceCpuArrayTrait {
}

impl LMSRegularGridSegmentTargetSurfaceImpCpuArrayBaseTrait for LMSRegularGridSegmentTargetSurfaceImpCpuArrayBase {
}

impl LMSRegularGridSurfaceCpuArrayTrait for LMSRegularGridSegmentTargetSurfaceImpCpuArrayBase {
}

impl LMSRegularGridSurfaceTrait for LMSRegularGridSegmentTargetSurfaceImpCpuArrayBase {
}

pub static LMSREGULARGRIDSEGMENTTARGETSURFACEIMPCPUARRAYBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridSegmentTargetSurfaceImpCpuArrayBase",
    flags: MemberInfoFlags::new(101),
    module: "LMSRegularGrid",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LMSREGULARGRIDSURFACECPUARRAY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LMSRegularGridSegmentTargetSurfaceImpCpuArrayBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LMSREGULARGRIDSEGMENTTARGETSURFACEIMPCPUARRAYBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LMSRegularGridSegmentTargetSurfaceImpCpuArrayBase {
    fn type_info(&self) -> &'static TypeInfo {
        LMSREGULARGRIDSEGMENTTARGETSURFACEIMPCPUARRAYBASE_TYPE_INFO
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


pub static LMSREGULARGRIDSEGMENTTARGETSURFACEIMPCPUARRAYBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridSegmentTargetSurfaceImpCpuArrayBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSRegularGrid",
    data: TypeInfoData::Array("LMSRegularGridSegmentTargetSurfaceImpCpuArrayBase"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LMSDynaPackRuntime {
    pub _glacier_base: super::linear_media::LinearMediaChannelRuntime,
}

pub trait LMSDynaPackRuntimeTrait: super::linear_media::LinearMediaChannelRuntimeTrait {
}

impl LMSDynaPackRuntimeTrait for LMSDynaPackRuntime {
}

impl super::linear_media::LinearMediaChannelRuntimeTrait for LMSDynaPackRuntime {
}

pub static LMSDYNAPACKRUNTIME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSDynaPackRuntime",
    flags: MemberInfoFlags::new(101),
    module: "LMSRegularGrid",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::linear_media::LINEARMEDIACHANNELRUNTIME_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LMSDynaPackRuntime as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LMSDYNAPACKRUNTIME_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LMSDynaPackRuntime {
    fn type_info(&self) -> &'static TypeInfo {
        LMSDYNAPACKRUNTIME_TYPE_INFO
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


pub static LMSDYNAPACKRUNTIME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSDynaPackRuntime-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSRegularGrid",
    data: TypeInfoData::Array("LMSDynaPackRuntime"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LMSRegularGridRuntime {
    pub _glacier_base: super::linear_media::LinearMediaChannelRuntime,
}

pub trait LMSRegularGridRuntimeTrait: super::linear_media::LinearMediaChannelRuntimeTrait {
}

impl LMSRegularGridRuntimeTrait for LMSRegularGridRuntime {
}

impl super::linear_media::LinearMediaChannelRuntimeTrait for LMSRegularGridRuntime {
}

pub static LMSREGULARGRIDRUNTIME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridRuntime",
    flags: MemberInfoFlags::new(101),
    module: "LMSRegularGrid",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::linear_media::LINEARMEDIACHANNELRUNTIME_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LMSRegularGridRuntime as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LMSREGULARGRIDRUNTIME_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LMSRegularGridRuntime {
    fn type_info(&self) -> &'static TypeInfo {
        LMSREGULARGRIDRUNTIME_TYPE_INFO
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


pub static LMSREGULARGRIDRUNTIME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridRuntime-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSRegularGrid",
    data: TypeInfoData::Array("LMSRegularGridRuntime"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LMSRegularGridCodecRuntime {
}

pub trait LMSRegularGridCodecRuntimeTrait: TypeObject {
}

impl LMSRegularGridCodecRuntimeTrait for LMSRegularGridCodecRuntime {
}

pub static LMSREGULARGRIDCODECRUNTIME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridCodecRuntime",
    flags: MemberInfoFlags::new(101),
    module: "LMSRegularGrid",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LMSRegularGridCodecRuntime as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LMSREGULARGRIDCODECRUNTIME_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LMSRegularGridCodecRuntime {
    fn type_info(&self) -> &'static TypeInfo {
        LMSREGULARGRIDCODECRUNTIME_TYPE_INFO
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


pub static LMSREGULARGRIDCODECRUNTIME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridCodecRuntime-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSRegularGrid",
    data: TypeInfoData::Array("LMSRegularGridCodecRuntime"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LMSRegularGridSurfaceCpuArrayImpDecoderTemp {
    pub _glacier_base: LMSRegularGridSurfaceCpuArray,
}

pub trait LMSRegularGridSurfaceCpuArrayImpDecoderTempTrait: LMSRegularGridSurfaceCpuArrayTrait {
}

impl LMSRegularGridSurfaceCpuArrayImpDecoderTempTrait for LMSRegularGridSurfaceCpuArrayImpDecoderTemp {
}

impl LMSRegularGridSurfaceCpuArrayTrait for LMSRegularGridSurfaceCpuArrayImpDecoderTemp {
}

impl LMSRegularGridSurfaceTrait for LMSRegularGridSurfaceCpuArrayImpDecoderTemp {
}

pub static LMSREGULARGRIDSURFACECPUARRAYIMPDECODERTEMP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridSurfaceCpuArrayImpDecoderTemp",
    flags: MemberInfoFlags::new(101),
    module: "LMSRegularGrid",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LMSREGULARGRIDSURFACECPUARRAY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LMSRegularGridSurfaceCpuArrayImpDecoderTemp as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LMSREGULARGRIDSURFACECPUARRAYIMPDECODERTEMP_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LMSRegularGridSurfaceCpuArrayImpDecoderTemp {
    fn type_info(&self) -> &'static TypeInfo {
        LMSREGULARGRIDSURFACECPUARRAYIMPDECODERTEMP_TYPE_INFO
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


pub static LMSREGULARGRIDSURFACECPUARRAYIMPDECODERTEMP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridSurfaceCpuArrayImpDecoderTemp-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSRegularGrid",
    data: TypeInfoData::Array("LMSRegularGridSurfaceCpuArrayImpDecoderTemp"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LMSRegularGridSurfaceGpuTexture {
    pub _glacier_base: LMSRegularGridSurface,
}

pub trait LMSRegularGridSurfaceGpuTextureTrait: LMSRegularGridSurfaceTrait {
}

impl LMSRegularGridSurfaceGpuTextureTrait for LMSRegularGridSurfaceGpuTexture {
}

impl LMSRegularGridSurfaceTrait for LMSRegularGridSurfaceGpuTexture {
}

pub static LMSREGULARGRIDSURFACEGPUTEXTURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridSurfaceGpuTexture",
    flags: MemberInfoFlags::new(101),
    module: "LMSRegularGrid",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LMSREGULARGRIDSURFACE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LMSRegularGridSurfaceGpuTexture as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LMSREGULARGRIDSURFACEGPUTEXTURE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LMSRegularGridSurfaceGpuTexture {
    fn type_info(&self) -> &'static TypeInfo {
        LMSREGULARGRIDSURFACEGPUTEXTURE_TYPE_INFO
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


pub static LMSREGULARGRIDSURFACEGPUTEXTURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridSurfaceGpuTexture-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSRegularGrid",
    data: TypeInfoData::Array("LMSRegularGridSurfaceGpuTexture"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LMSRegularGridSurfaceGpuBuffer {
    pub _glacier_base: LMSRegularGridSurface,
}

pub trait LMSRegularGridSurfaceGpuBufferTrait: LMSRegularGridSurfaceTrait {
}

impl LMSRegularGridSurfaceGpuBufferTrait for LMSRegularGridSurfaceGpuBuffer {
}

impl LMSRegularGridSurfaceTrait for LMSRegularGridSurfaceGpuBuffer {
}

pub static LMSREGULARGRIDSURFACEGPUBUFFER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridSurfaceGpuBuffer",
    flags: MemberInfoFlags::new(101),
    module: "LMSRegularGrid",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LMSREGULARGRIDSURFACE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LMSRegularGridSurfaceGpuBuffer as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LMSREGULARGRIDSURFACEGPUBUFFER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LMSRegularGridSurfaceGpuBuffer {
    fn type_info(&self) -> &'static TypeInfo {
        LMSREGULARGRIDSURFACEGPUBUFFER_TYPE_INFO
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


pub static LMSREGULARGRIDSURFACEGPUBUFFER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridSurfaceGpuBuffer-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSRegularGrid",
    data: TypeInfoData::Array("LMSRegularGridSurfaceGpuBuffer"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LMSRegularGridSurfaceCpuArray {
    pub _glacier_base: LMSRegularGridSurface,
}

pub trait LMSRegularGridSurfaceCpuArrayTrait: LMSRegularGridSurfaceTrait {
}

impl LMSRegularGridSurfaceCpuArrayTrait for LMSRegularGridSurfaceCpuArray {
}

impl LMSRegularGridSurfaceTrait for LMSRegularGridSurfaceCpuArray {
}

pub static LMSREGULARGRIDSURFACECPUARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridSurfaceCpuArray",
    flags: MemberInfoFlags::new(101),
    module: "LMSRegularGrid",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LMSREGULARGRIDSURFACE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LMSRegularGridSurfaceCpuArray as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LMSREGULARGRIDSURFACECPUARRAY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LMSRegularGridSurfaceCpuArray {
    fn type_info(&self) -> &'static TypeInfo {
        LMSREGULARGRIDSURFACECPUARRAY_TYPE_INFO
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


pub static LMSREGULARGRIDSURFACECPUARRAY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridSurfaceCpuArray-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSRegularGrid",
    data: TypeInfoData::Array("LMSRegularGridSurfaceCpuArray"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LMSRegularGridSurface {
}

pub trait LMSRegularGridSurfaceTrait: TypeObject {
}

impl LMSRegularGridSurfaceTrait for LMSRegularGridSurface {
}

pub static LMSREGULARGRIDSURFACE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridSurface",
    flags: MemberInfoFlags::new(101),
    module: "LMSRegularGrid",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LMSRegularGridSurface as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LMSREGULARGRIDSURFACE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LMSRegularGridSurface {
    fn type_info(&self) -> &'static TypeInfo {
        LMSREGULARGRIDSURFACE_TYPE_INFO
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


pub static LMSREGULARGRIDSURFACE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridSurface-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSRegularGrid",
    data: TypeInfoData::Array("LMSRegularGridSurface"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LMSRegularGridCPUArray {
}

pub trait LMSRegularGridCPUArrayTrait: TypeObject {
}

impl LMSRegularGridCPUArrayTrait for LMSRegularGridCPUArray {
}

pub static LMSREGULARGRIDCPUARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridCPUArray",
    flags: MemberInfoFlags::new(101),
    module: "LMSRegularGrid",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LMSRegularGridCPUArray as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LMSREGULARGRIDCPUARRAY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LMSRegularGridCPUArray {
    fn type_info(&self) -> &'static TypeInfo {
        LMSREGULARGRIDCPUARRAY_TYPE_INFO
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


pub static LMSREGULARGRIDCPUARRAY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridCPUArray-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSRegularGrid",
    data: TypeInfoData::Array("LMSRegularGridCPUArray"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LMSEffectsDataArray {
}

pub trait LMSEffectsDataArrayTrait: TypeObject {
}

impl LMSEffectsDataArrayTrait for LMSEffectsDataArray {
}

pub static LMSEFFECTSDATAARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSEffectsDataArray",
    flags: MemberInfoFlags::new(101),
    module: "LMSRegularGrid",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LMSEffectsDataArray as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LMSEFFECTSDATAARRAY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LMSEffectsDataArray {
    fn type_info(&self) -> &'static TypeInfo {
        LMSEFFECTSDATAARRAY_TYPE_INFO
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


pub static LMSEFFECTSDATAARRAY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSEffectsDataArray-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSRegularGrid",
    data: TypeInfoData::Array("LMSEffectsDataArray"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LMSRegularGridDefaultCodecRuntime {
    pub _glacier_base: LMSRegularGridCodecRuntime,
}

pub trait LMSRegularGridDefaultCodecRuntimeTrait: LMSRegularGridCodecRuntimeTrait {
}

impl LMSRegularGridDefaultCodecRuntimeTrait for LMSRegularGridDefaultCodecRuntime {
}

impl LMSRegularGridCodecRuntimeTrait for LMSRegularGridDefaultCodecRuntime {
}

pub static LMSREGULARGRIDDEFAULTCODECRUNTIME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridDefaultCodecRuntime",
    flags: MemberInfoFlags::new(101),
    module: "LMSRegularGrid",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LMSREGULARGRIDCODECRUNTIME_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LMSRegularGridDefaultCodecRuntime as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LMSREGULARGRIDDEFAULTCODECRUNTIME_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LMSRegularGridDefaultCodecRuntime {
    fn type_info(&self) -> &'static TypeInfo {
        LMSREGULARGRIDDEFAULTCODECRUNTIME_TYPE_INFO
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


pub static LMSREGULARGRIDDEFAULTCODECRUNTIME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridDefaultCodecRuntime-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSRegularGrid",
    data: TypeInfoData::Array("LMSRegularGridDefaultCodecRuntime"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LMSRegularGridVp6CodecRuntime {
    pub _glacier_base: LMSRegularGridCodecRuntime,
}

pub trait LMSRegularGridVp6CodecRuntimeTrait: LMSRegularGridCodecRuntimeTrait {
}

impl LMSRegularGridVp6CodecRuntimeTrait for LMSRegularGridVp6CodecRuntime {
}

impl LMSRegularGridCodecRuntimeTrait for LMSRegularGridVp6CodecRuntime {
}

pub static LMSREGULARGRIDVP6CODECRUNTIME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridVp6CodecRuntime",
    flags: MemberInfoFlags::new(101),
    module: "LMSRegularGrid",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LMSREGULARGRIDCODECRUNTIME_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LMSRegularGridVp6CodecRuntime as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LMSREGULARGRIDVP6CODECRUNTIME_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LMSRegularGridVp6CodecRuntime {
    fn type_info(&self) -> &'static TypeInfo {
        LMSREGULARGRIDVP6CODECRUNTIME_TYPE_INFO
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


pub static LMSREGULARGRIDVP6CODECRUNTIME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridVp6CodecRuntime-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSRegularGrid",
    data: TypeInfoData::Array("LMSRegularGridVp6CodecRuntime"),
    array_type: None,
    alignment: 8,
};


