use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
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

pub trait EnvironmentDecalVolumeDynamicStateTrait: TypeObject {
    fn transform(&self) -> &super::core::LinearTransform;
    fn enabled(&self) -> &bool;
    fn override_template_culling_distance(&self) -> &f32;
    fn alpha(&self) -> &f32;
    fn instance_params(&self) -> &super::core::Vec3;
    fn row(&self) -> &u8;
    fn column(&self) -> &u8;
    fn field_flag_changed0(&self) -> &u8;
}

impl EnvironmentDecalVolumeDynamicStateTrait for EnvironmentDecalVolumeDynamicState {
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn override_template_culling_distance(&self) -> &f32 {
        &self.override_template_culling_distance
    }
    fn alpha(&self) -> &f32 {
        &self.alpha
    }
    fn instance_params(&self) -> &super::core::Vec3 {
        &self.instance_params
    }
    fn row(&self) -> &u8 {
        &self.row
    }
    fn column(&self) -> &u8 {
        &self.column
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static ENVIRONMENTDECALVOLUMEDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnvironmentDecalVolumeDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "DecalVolumeBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EnvironmentDecalVolumeDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(EnvironmentDecalVolumeDynamicState, transform),
            },
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnvironmentDecalVolumeDynamicState, enabled),
            },
            FieldInfoData {
                name: "OverrideTemplateCullingDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EnvironmentDecalVolumeDynamicState, override_template_culling_distance),
            },
            FieldInfoData {
                name: "Alpha",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EnvironmentDecalVolumeDynamicState, alpha),
            },
            FieldInfoData {
                name: "InstanceParams",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(EnvironmentDecalVolumeDynamicState, instance_params),
            },
            FieldInfoData {
                name: "Row",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(EnvironmentDecalVolumeDynamicState, row),
            },
            FieldInfoData {
                name: "Column",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(EnvironmentDecalVolumeDynamicState, column),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(EnvironmentDecalVolumeDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(ENVIRONMENTDECALVOLUMEDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EnvironmentDecalVolumeDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        ENVIRONMENTDECALVOLUMEDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ENVIRONMENTDECALVOLUMEDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnvironmentDecalVolumeDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "DecalVolumeBase",
    data: TypeInfoData::Array("EnvironmentDecalVolumeDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EnvironmentDecalVolumeStaticState {
    pub template_data: Option<Arc<Mutex<dyn EnvironmentDecalVolumeTemplateBaseDataTrait>>>,
    pub culling_distance: super::core::QualityScalableFloat,
    pub field_flag_changed0: u8,
}

pub trait EnvironmentDecalVolumeStaticStateTrait: TypeObject {
    fn template_data(&self) -> &Option<Arc<Mutex<dyn EnvironmentDecalVolumeTemplateBaseDataTrait>>>;
    fn culling_distance(&self) -> &super::core::QualityScalableFloat;
    fn field_flag_changed0(&self) -> &u8;
}

impl EnvironmentDecalVolumeStaticStateTrait for EnvironmentDecalVolumeStaticState {
    fn template_data(&self) -> &Option<Arc<Mutex<dyn EnvironmentDecalVolumeTemplateBaseDataTrait>>> {
        &self.template_data
    }
    fn culling_distance(&self) -> &super::core::QualityScalableFloat {
        &self.culling_distance
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static ENVIRONMENTDECALVOLUMESTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnvironmentDecalVolumeStaticState",
    flags: MemberInfoFlags::new(73),
    module: "DecalVolumeBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EnvironmentDecalVolumeStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TemplateData",
                flags: MemberInfoFlags::new(0),
                field_type: "EnvironmentDecalVolumeTemplateBaseData",
                rust_offset: offset_of!(EnvironmentDecalVolumeStaticState, template_data),
            },
            FieldInfoData {
                name: "CullingDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(EnvironmentDecalVolumeStaticState, culling_distance),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(EnvironmentDecalVolumeStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(ENVIRONMENTDECALVOLUMESTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EnvironmentDecalVolumeStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        ENVIRONMENTDECALVOLUMESTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ENVIRONMENTDECALVOLUMESTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnvironmentDecalVolumeStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "DecalVolumeBase",
    data: TypeInfoData::Array("EnvironmentDecalVolumeStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EnvironmentDecalVolumeTemplateBaseData {
    pub _glacier_base: super::core::Asset,
}

pub trait EnvironmentDecalVolumeTemplateBaseDataTrait: super::core::AssetTrait {
}

impl EnvironmentDecalVolumeTemplateBaseDataTrait for EnvironmentDecalVolumeTemplateBaseData {
}

impl super::core::AssetTrait for EnvironmentDecalVolumeTemplateBaseData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for EnvironmentDecalVolumeTemplateBaseData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static ENVIRONMENTDECALVOLUMETEMPLATEBASEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnvironmentDecalVolumeTemplateBaseData",
    flags: MemberInfoFlags::new(101),
    module: "DecalVolumeBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EnvironmentDecalVolumeTemplateBaseData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(ENVIRONMENTDECALVOLUMETEMPLATEBASEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EnvironmentDecalVolumeTemplateBaseData {
    fn type_info(&self) -> &'static TypeInfo {
        ENVIRONMENTDECALVOLUMETEMPLATEBASEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ENVIRONMENTDECALVOLUMETEMPLATEBASEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnvironmentDecalVolumeTemplateBaseData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DecalVolumeBase",
    data: TypeInfoData::Array("EnvironmentDecalVolumeTemplateBaseData"),
    array_type: None,
    alignment: 8,
};


