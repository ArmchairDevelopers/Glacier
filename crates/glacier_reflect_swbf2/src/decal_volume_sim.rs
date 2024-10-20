use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_decal_volume_sim_types(registry: &mut TypeRegistry) {
    registry.register_type(ENVIRONMENTDECALVOLUMEDATA_TYPE_INFO);
    registry.register_type(ENVIRONMENTDECALVOLUMEDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENVIRONMENTDECALVOLUMEENTITY_TYPE_INFO);
    registry.register_type(ENVIRONMENTDECALVOLUMEENTITY_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Debug)]
pub struct EnvironmentDecalVolumeData {
    pub is_enabled: bool,
    pub template: super::decal_volume_base::EnvironmentDecalVolumeTemplateBaseData,
    pub culling_distance: super::core::QualityScalableFloat,
    pub override_template_culling_distance: f32,
    pub alpha: f32,
    pub instance_params: super::core::Vec3,
    pub row: u8,
    pub column: u8,
}

pub const ENVIRONMENTDECALVOLUMEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnvironmentDecalVolumeData",
    flags: MemberInfoFlags::new(101),
    module: "DecalVolumeSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "IsEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EnvironmentDecalVolumeData, is_enabled),
            },
            FieldInfoData {
                name: "Template",
                flags: MemberInfoFlags::new(0),
                field_type: ENVIRONMENTDECALVOLUMETEMPLATEBASEDATA_TYPE_INFO,
                rust_offset: offset_of!(EnvironmentDecalVolumeData, template),
            },
            FieldInfoData {
                name: "CullingDistance",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEFLOAT_TYPE_INFO,
                rust_offset: offset_of!(EnvironmentDecalVolumeData, culling_distance),
            },
            FieldInfoData {
                name: "OverrideTemplateCullingDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EnvironmentDecalVolumeData, override_template_culling_distance),
            },
            FieldInfoData {
                name: "Alpha",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EnvironmentDecalVolumeData, alpha),
            },
            FieldInfoData {
                name: "InstanceParams",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(EnvironmentDecalVolumeData, instance_params),
            },
            FieldInfoData {
                name: "Row",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(EnvironmentDecalVolumeData, row),
            },
            FieldInfoData {
                name: "Column",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(EnvironmentDecalVolumeData, column),
            },
        ],
    }),
    array_type: Some(ENVIRONMENTDECALVOLUMEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EnvironmentDecalVolumeData {
    fn type_info() -> &'static TypeInfo {
        ENVIRONMENTDECALVOLUMEDATA_TYPE_INFO
    }
}


pub const ENVIRONMENTDECALVOLUMEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnvironmentDecalVolumeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DecalVolumeSim",
    data: TypeInfoData::Array("EnvironmentDecalVolumeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EnvironmentDecalVolumeEntity {
}

pub const ENVIRONMENTDECALVOLUMEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnvironmentDecalVolumeEntity",
    flags: MemberInfoFlags::new(101),
    module: "DecalVolumeSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ENVIRONMENTDECALVOLUMEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EnvironmentDecalVolumeEntity {
    fn type_info() -> &'static TypeInfo {
        ENVIRONMENTDECALVOLUMEENTITY_TYPE_INFO
    }
}


pub const ENVIRONMENTDECALVOLUMEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnvironmentDecalVolumeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DecalVolumeSim",
    data: TypeInfoData::Array("EnvironmentDecalVolumeEntity-Array"),
    array_type: None,
    alignment: 8,
};


