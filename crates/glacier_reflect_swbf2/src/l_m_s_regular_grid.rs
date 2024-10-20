use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
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

#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum LMSRegularGridRescaleNodeFilteringMethod {
    #[default]
    LMSRegularGridRescaleNodeFilteringMethod_Bilinear = 0,
    LMSRegularGridRescaleNodeFilteringMethod_Bicubic = 1,
    LMSRegularGridRescaleNodeFilteringMethod_SingleAverageColor = 2,
}

pub const LMSREGULARGRIDRESCALENODEFILTERINGMETHOD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridRescaleNodeFilteringMethod",
    flags: MemberInfoFlags::new(49429),
    module: "LMSRegularGrid",
    data: TypeInfoData::Enum,
    array_type: Some(LMSREGULARGRIDRESCALENODEFILTERINGMETHOD_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LMSRegularGridRescaleNodeFilteringMethod {
    fn type_info() -> &'static TypeInfo {
        LMSREGULARGRIDRESCALENODEFILTERINGMETHOD_TYPE_INFO
    }
}


pub const LMSREGULARGRIDRESCALENODEFILTERINGMETHOD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridRescaleNodeFilteringMethod-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSRegularGrid",
    data: TypeInfoData::Array("LMSRegularGridRescaleNodeFilteringMethod-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
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

pub const LMSREGULARGRIDBINDFLAGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridBindFlags",
    flags: MemberInfoFlags::new(49429),
    module: "LMSRegularGrid",
    data: TypeInfoData::Enum,
    array_type: Some(LMSREGULARGRIDBINDFLAGS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LMSRegularGridBindFlags {
    fn type_info() -> &'static TypeInfo {
        LMSREGULARGRIDBINDFLAGS_TYPE_INFO
    }
}


pub const LMSREGULARGRIDBINDFLAGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridBindFlags-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSRegularGrid",
    data: TypeInfoData::Array("LMSRegularGridBindFlags-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum LMSRegularGridOutputAttributeMode {
    #[default]
    LMSRegularGridOutputAttributeMode_GpuTexture = 0,
    LMSRegularGridOutputAttributeMode_GpuBuffer = 1,
    LMSRegularGridOutputAttributeMode_CpuArray = 2,
    LMSRegularGridOutputAttributeMode_EmitterArray = 3,
}

pub const LMSREGULARGRIDOUTPUTATTRIBUTEMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridOutputAttributeMode",
    flags: MemberInfoFlags::new(49429),
    module: "LMSRegularGrid",
    data: TypeInfoData::Enum,
    array_type: Some(LMSREGULARGRIDOUTPUTATTRIBUTEMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LMSRegularGridOutputAttributeMode {
    fn type_info() -> &'static TypeInfo {
        LMSREGULARGRIDOUTPUTATTRIBUTEMODE_TYPE_INFO
    }
}


pub const LMSREGULARGRIDOUTPUTATTRIBUTEMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridOutputAttributeMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSRegularGrid",
    data: TypeInfoData::Array("LMSRegularGridOutputAttributeMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LMSRegularGridSegmentTargetSurfaceImpCpuArrayBase {
}

pub const LMSREGULARGRIDSEGMENTTARGETSURFACEIMPCPUARRAYBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridSegmentTargetSurfaceImpCpuArrayBase",
    flags: MemberInfoFlags::new(101),
    module: "LMSRegularGrid",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LMSREGULARGRIDSURFACECPUARRAY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LMSREGULARGRIDSEGMENTTARGETSURFACEIMPCPUARRAYBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LMSRegularGridSegmentTargetSurfaceImpCpuArrayBase {
    fn type_info() -> &'static TypeInfo {
        LMSREGULARGRIDSEGMENTTARGETSURFACEIMPCPUARRAYBASE_TYPE_INFO
    }
}


pub const LMSREGULARGRIDSEGMENTTARGETSURFACEIMPCPUARRAYBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridSegmentTargetSurfaceImpCpuArrayBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSRegularGrid",
    data: TypeInfoData::Array("LMSRegularGridSegmentTargetSurfaceImpCpuArrayBase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LMSDynaPackRuntime {
}

pub const LMSDYNAPACKRUNTIME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSDynaPackRuntime",
    flags: MemberInfoFlags::new(101),
    module: "LMSRegularGrid",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LINEARMEDIACHANNELRUNTIME_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LMSDYNAPACKRUNTIME_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LMSDynaPackRuntime {
    fn type_info() -> &'static TypeInfo {
        LMSDYNAPACKRUNTIME_TYPE_INFO
    }
}


pub const LMSDYNAPACKRUNTIME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSDynaPackRuntime-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSRegularGrid",
    data: TypeInfoData::Array("LMSDynaPackRuntime-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LMSRegularGridRuntime {
}

pub const LMSREGULARGRIDRUNTIME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridRuntime",
    flags: MemberInfoFlags::new(101),
    module: "LMSRegularGrid",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LINEARMEDIACHANNELRUNTIME_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LMSREGULARGRIDRUNTIME_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LMSRegularGridRuntime {
    fn type_info() -> &'static TypeInfo {
        LMSREGULARGRIDRUNTIME_TYPE_INFO
    }
}


pub const LMSREGULARGRIDRUNTIME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridRuntime-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSRegularGrid",
    data: TypeInfoData::Array("LMSRegularGridRuntime-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LMSRegularGridCodecRuntime {
}

pub const LMSREGULARGRIDCODECRUNTIME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridCodecRuntime",
    flags: MemberInfoFlags::new(101),
    module: "LMSRegularGrid",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(LMSREGULARGRIDCODECRUNTIME_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LMSRegularGridCodecRuntime {
    fn type_info() -> &'static TypeInfo {
        LMSREGULARGRIDCODECRUNTIME_TYPE_INFO
    }
}


pub const LMSREGULARGRIDCODECRUNTIME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridCodecRuntime-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSRegularGrid",
    data: TypeInfoData::Array("LMSRegularGridCodecRuntime-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LMSRegularGridSurfaceCpuArrayImpDecoderTemp {
}

pub const LMSREGULARGRIDSURFACECPUARRAYIMPDECODERTEMP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridSurfaceCpuArrayImpDecoderTemp",
    flags: MemberInfoFlags::new(101),
    module: "LMSRegularGrid",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LMSREGULARGRIDSURFACECPUARRAY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LMSREGULARGRIDSURFACECPUARRAYIMPDECODERTEMP_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LMSRegularGridSurfaceCpuArrayImpDecoderTemp {
    fn type_info() -> &'static TypeInfo {
        LMSREGULARGRIDSURFACECPUARRAYIMPDECODERTEMP_TYPE_INFO
    }
}


pub const LMSREGULARGRIDSURFACECPUARRAYIMPDECODERTEMP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridSurfaceCpuArrayImpDecoderTemp-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSRegularGrid",
    data: TypeInfoData::Array("LMSRegularGridSurfaceCpuArrayImpDecoderTemp-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LMSRegularGridSurfaceGpuTexture {
}

pub const LMSREGULARGRIDSURFACEGPUTEXTURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridSurfaceGpuTexture",
    flags: MemberInfoFlags::new(101),
    module: "LMSRegularGrid",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LMSREGULARGRIDSURFACE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LMSREGULARGRIDSURFACEGPUTEXTURE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LMSRegularGridSurfaceGpuTexture {
    fn type_info() -> &'static TypeInfo {
        LMSREGULARGRIDSURFACEGPUTEXTURE_TYPE_INFO
    }
}


pub const LMSREGULARGRIDSURFACEGPUTEXTURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridSurfaceGpuTexture-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSRegularGrid",
    data: TypeInfoData::Array("LMSRegularGridSurfaceGpuTexture-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LMSRegularGridSurfaceGpuBuffer {
}

pub const LMSREGULARGRIDSURFACEGPUBUFFER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridSurfaceGpuBuffer",
    flags: MemberInfoFlags::new(101),
    module: "LMSRegularGrid",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LMSREGULARGRIDSURFACE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LMSREGULARGRIDSURFACEGPUBUFFER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LMSRegularGridSurfaceGpuBuffer {
    fn type_info() -> &'static TypeInfo {
        LMSREGULARGRIDSURFACEGPUBUFFER_TYPE_INFO
    }
}


pub const LMSREGULARGRIDSURFACEGPUBUFFER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridSurfaceGpuBuffer-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSRegularGrid",
    data: TypeInfoData::Array("LMSRegularGridSurfaceGpuBuffer-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LMSRegularGridSurfaceCpuArray {
}

pub const LMSREGULARGRIDSURFACECPUARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridSurfaceCpuArray",
    flags: MemberInfoFlags::new(101),
    module: "LMSRegularGrid",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LMSREGULARGRIDSURFACE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LMSREGULARGRIDSURFACECPUARRAY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LMSRegularGridSurfaceCpuArray {
    fn type_info() -> &'static TypeInfo {
        LMSREGULARGRIDSURFACECPUARRAY_TYPE_INFO
    }
}


pub const LMSREGULARGRIDSURFACECPUARRAY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridSurfaceCpuArray-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSRegularGrid",
    data: TypeInfoData::Array("LMSRegularGridSurfaceCpuArray-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LMSRegularGridSurface {
}

pub const LMSREGULARGRIDSURFACE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridSurface",
    flags: MemberInfoFlags::new(101),
    module: "LMSRegularGrid",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(LMSREGULARGRIDSURFACE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LMSRegularGridSurface {
    fn type_info() -> &'static TypeInfo {
        LMSREGULARGRIDSURFACE_TYPE_INFO
    }
}


pub const LMSREGULARGRIDSURFACE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridSurface-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSRegularGrid",
    data: TypeInfoData::Array("LMSRegularGridSurface-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LMSRegularGridCPUArray {
}

pub const LMSREGULARGRIDCPUARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridCPUArray",
    flags: MemberInfoFlags::new(101),
    module: "LMSRegularGrid",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(LMSREGULARGRIDCPUARRAY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LMSRegularGridCPUArray {
    fn type_info() -> &'static TypeInfo {
        LMSREGULARGRIDCPUARRAY_TYPE_INFO
    }
}


pub const LMSREGULARGRIDCPUARRAY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridCPUArray-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSRegularGrid",
    data: TypeInfoData::Array("LMSRegularGridCPUArray-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LMSEffectsDataArray {
}

pub const LMSEFFECTSDATAARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSEffectsDataArray",
    flags: MemberInfoFlags::new(101),
    module: "LMSRegularGrid",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(LMSEFFECTSDATAARRAY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LMSEffectsDataArray {
    fn type_info() -> &'static TypeInfo {
        LMSEFFECTSDATAARRAY_TYPE_INFO
    }
}


pub const LMSEFFECTSDATAARRAY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSEffectsDataArray-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSRegularGrid",
    data: TypeInfoData::Array("LMSEffectsDataArray-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LMSRegularGridDefaultCodecRuntime {
}

pub const LMSREGULARGRIDDEFAULTCODECRUNTIME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridDefaultCodecRuntime",
    flags: MemberInfoFlags::new(101),
    module: "LMSRegularGrid",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LMSREGULARGRIDCODECRUNTIME_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LMSREGULARGRIDDEFAULTCODECRUNTIME_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LMSRegularGridDefaultCodecRuntime {
    fn type_info() -> &'static TypeInfo {
        LMSREGULARGRIDDEFAULTCODECRUNTIME_TYPE_INFO
    }
}


pub const LMSREGULARGRIDDEFAULTCODECRUNTIME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridDefaultCodecRuntime-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSRegularGrid",
    data: TypeInfoData::Array("LMSRegularGridDefaultCodecRuntime-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LMSRegularGridVp6CodecRuntime {
}

pub const LMSREGULARGRIDVP6CODECRUNTIME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridVp6CodecRuntime",
    flags: MemberInfoFlags::new(101),
    module: "LMSRegularGrid",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LMSREGULARGRIDCODECRUNTIME_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LMSREGULARGRIDVP6CODECRUNTIME_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LMSRegularGridVp6CodecRuntime {
    fn type_info() -> &'static TypeInfo {
        LMSREGULARGRIDVP6CODECRUNTIME_TYPE_INFO
    }
}


pub const LMSREGULARGRIDVP6CODECRUNTIME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSRegularGridVp6CodecRuntime-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSRegularGrid",
    data: TypeInfoData::Array("LMSRegularGridVp6CodecRuntime-Array"),
    array_type: None,
    alignment: 8,
};


