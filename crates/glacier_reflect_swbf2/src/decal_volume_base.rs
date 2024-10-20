use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_decal_volume_base_types(registry: &mut TypeRegistry) {
    registry.register_type(ENVIRONMENTDECALVOLUMEDYNAMICSTATE_TYPE_INFO);
    registry.register_type(ENVIRONMENTDECALVOLUMEDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(ENVIRONMENTDECALVOLUMESTATICSTATE_TYPE_INFO);
    registry.register_type(ENVIRONMENTDECALVOLUMESTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(ENVIRONMENTDECALVOLUMETEMPLATEBASEDATA_TYPE_INFO);
    registry.register_type(ENVIRONMENTDECALVOLUMETEMPLATEBASEDATA_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Debug)]
pub struct EnvironmentDecalVolumeDynamicState {
    pub transform: super::core::LinearTransform,
    pub enabled: bool,
    pub override_template_culling_distance: f32,
    pub alpha: f32,
    pub instance_params: super::core::Vec3,
    pub row: u8,
    pub column: u8,
    pub field_flag_changed0: u8,
}

pub const ENVIRONMENTDECALVOLUMEDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnvironmentDecalVolumeDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "DecalVolumeBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(EnvironmentDecalVolumeDynamicState, transform),
            },
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EnvironmentDecalVolumeDynamicState, enabled),
            },
            FieldInfoData {
                name: "OverrideTemplateCullingDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EnvironmentDecalVolumeDynamicState, override_template_culling_distance),
            },
            FieldInfoData {
                name: "Alpha",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EnvironmentDecalVolumeDynamicState, alpha),
            },
            FieldInfoData {
                name: "InstanceParams",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(EnvironmentDecalVolumeDynamicState, instance_params),
            },
            FieldInfoData {
                name: "Row",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(EnvironmentDecalVolumeDynamicState, row),
            },
            FieldInfoData {
                name: "Column",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(EnvironmentDecalVolumeDynamicState, column),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(EnvironmentDecalVolumeDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(ENVIRONMENTDECALVOLUMEDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EnvironmentDecalVolumeDynamicState {
    fn type_info() -> &'static TypeInfo {
        ENVIRONMENTDECALVOLUMEDYNAMICSTATE_TYPE_INFO
    }
}


pub const ENVIRONMENTDECALVOLUMEDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnvironmentDecalVolumeDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "DecalVolumeBase",
    data: TypeInfoData::Array("EnvironmentDecalVolumeDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EnvironmentDecalVolumeStaticState {
    pub template_data: EnvironmentDecalVolumeTemplateBaseData,
    pub culling_distance: super::core::QualityScalableFloat,
    pub field_flag_changed0: u8,
}

pub const ENVIRONMENTDECALVOLUMESTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnvironmentDecalVolumeStaticState",
    flags: MemberInfoFlags::new(73),
    module: "DecalVolumeBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "TemplateData",
                flags: MemberInfoFlags::new(0),
                field_type: ENVIRONMENTDECALVOLUMETEMPLATEBASEDATA_TYPE_INFO,
                rust_offset: offset_of!(EnvironmentDecalVolumeStaticState, template_data),
            },
            FieldInfoData {
                name: "CullingDistance",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEFLOAT_TYPE_INFO,
                rust_offset: offset_of!(EnvironmentDecalVolumeStaticState, culling_distance),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(EnvironmentDecalVolumeStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(ENVIRONMENTDECALVOLUMESTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EnvironmentDecalVolumeStaticState {
    fn type_info() -> &'static TypeInfo {
        ENVIRONMENTDECALVOLUMESTATICSTATE_TYPE_INFO
    }
}


pub const ENVIRONMENTDECALVOLUMESTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnvironmentDecalVolumeStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "DecalVolumeBase",
    data: TypeInfoData::Array("EnvironmentDecalVolumeStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EnvironmentDecalVolumeTemplateBaseData {
}

pub const ENVIRONMENTDECALVOLUMETEMPLATEBASEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnvironmentDecalVolumeTemplateBaseData",
    flags: MemberInfoFlags::new(101),
    module: "DecalVolumeBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ENVIRONMENTDECALVOLUMETEMPLATEBASEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EnvironmentDecalVolumeTemplateBaseData {
    fn type_info() -> &'static TypeInfo {
        ENVIRONMENTDECALVOLUMETEMPLATEBASEDATA_TYPE_INFO
    }
}


pub const ENVIRONMENTDECALVOLUMETEMPLATEBASEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnvironmentDecalVolumeTemplateBaseData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DecalVolumeBase",
    data: TypeInfoData::Array("EnvironmentDecalVolumeTemplateBaseData-Array"),
    array_type: None,
    alignment: 8,
};


