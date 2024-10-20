use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_decal_volume_render_types(registry: &mut TypeRegistry) {
    registry.register_type(ENVIRONMENTDECALVOLUMETEMPLATEDATA_TYPE_INFO);
    registry.register_type(ENVIRONMENTDECALVOLUMETEMPLATEDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENVIRONMENTDECALVOLUMEMASKTYPE_TYPE_INFO);
    registry.register_type(ENVIRONMENTDECALVOLUMEMASKTYPE_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Debug)]
pub struct EnvironmentDecalVolumeTemplateData {
    pub shader: super::render_base::SurfaceShaderInstanceDataStruct,
    pub mask_type: EnvironmentDecalVolumeMaskType,
    pub sorting_priority: u8,
    pub culling_distance: super::core::QualityScalableFloat,
}

pub const ENVIRONMENTDECALVOLUMETEMPLATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnvironmentDecalVolumeTemplateData",
    flags: MemberInfoFlags::new(101),
    module: "DecalVolumeRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENVIRONMENTDECALVOLUMETEMPLATEBASEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Shader",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERINSTANCEDATASTRUCT_TYPE_INFO,
                rust_offset: offset_of!(EnvironmentDecalVolumeTemplateData, shader),
            },
            FieldInfoData {
                name: "MaskType",
                flags: MemberInfoFlags::new(0),
                field_type: ENVIRONMENTDECALVOLUMEMASKTYPE_TYPE_INFO,
                rust_offset: offset_of!(EnvironmentDecalVolumeTemplateData, mask_type),
            },
            FieldInfoData {
                name: "SortingPriority",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(EnvironmentDecalVolumeTemplateData, sorting_priority),
            },
            FieldInfoData {
                name: "CullingDistance",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEFLOAT_TYPE_INFO,
                rust_offset: offset_of!(EnvironmentDecalVolumeTemplateData, culling_distance),
            },
        ],
    }),
    array_type: Some(ENVIRONMENTDECALVOLUMETEMPLATEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EnvironmentDecalVolumeTemplateData {
    fn type_info() -> &'static TypeInfo {
        ENVIRONMENTDECALVOLUMETEMPLATEDATA_TYPE_INFO
    }
}


pub const ENVIRONMENTDECALVOLUMETEMPLATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnvironmentDecalVolumeTemplateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DecalVolumeRender",
    data: TypeInfoData::Array("EnvironmentDecalVolumeTemplateData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EnvironmentDecalVolumeMaskType {
    #[default]
    MaskType_NoMask = 0,
    MaskType_StaticOnly = 1,
    MaskType_DynamicOnly = 2,
    MaskType_TerrainOnly = 3,
}

pub const ENVIRONMENTDECALVOLUMEMASKTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnvironmentDecalVolumeMaskType",
    flags: MemberInfoFlags::new(49429),
    module: "DecalVolumeRender",
    data: TypeInfoData::Enum,
    array_type: Some(ENVIRONMENTDECALVOLUMEMASKTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EnvironmentDecalVolumeMaskType {
    fn type_info() -> &'static TypeInfo {
        ENVIRONMENTDECALVOLUMEMASKTYPE_TYPE_INFO
    }
}


pub const ENVIRONMENTDECALVOLUMEMASKTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnvironmentDecalVolumeMaskType-Array",
    flags: MemberInfoFlags::new(145),
    module: "DecalVolumeRender",
    data: TypeInfoData::Array("EnvironmentDecalVolumeMaskType-Array"),
    array_type: None,
    alignment: 8,
};


