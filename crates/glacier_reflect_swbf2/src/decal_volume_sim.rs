use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_decal_volume_sim_types(registry: &mut TypeRegistry) {
    registry.register_type(ENVIRONMENTDECALVOLUMEDATA_TYPE_INFO);
    registry.register_type(ENVIRONMENTDECALVOLUMEDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENVIRONMENTDECALVOLUMEENTITY_TYPE_INFO);
    registry.register_type(ENVIRONMENTDECALVOLUMEENTITY_ARRAY_TYPE_INFO);
}

#[derive(Clone, Debug, Default)]
pub struct EnvironmentDecalVolumeData {
    pub _glacier_base: super::entity::SpatialEntityData,
    pub is_enabled: bool,
    pub template: Option<Arc<Mutex<dyn super::decal_volume_base::EnvironmentDecalVolumeTemplateBaseDataTrait>>>,
    pub culling_distance: super::core::QualityScalableFloat,
    pub override_template_culling_distance: f32,
    pub alpha: f32,
    pub instance_params: super::core::Vec3,
    pub row: u8,
    pub column: u8,
}

pub trait EnvironmentDecalVolumeDataTrait: super::entity::SpatialEntityDataTrait {
    fn is_enabled(&self) -> &bool;
    fn template(&self) -> &Option<Arc<Mutex<dyn super::decal_volume_base::EnvironmentDecalVolumeTemplateBaseDataTrait>>>;
    fn culling_distance(&self) -> &super::core::QualityScalableFloat;
    fn override_template_culling_distance(&self) -> &f32;
    fn alpha(&self) -> &f32;
    fn instance_params(&self) -> &super::core::Vec3;
    fn row(&self) -> &u8;
    fn column(&self) -> &u8;
}

impl EnvironmentDecalVolumeDataTrait for EnvironmentDecalVolumeData {
    fn is_enabled(&self) -> &bool {
        &self.is_enabled
    }
    fn template(&self) -> &Option<Arc<Mutex<dyn super::decal_volume_base::EnvironmentDecalVolumeTemplateBaseDataTrait>>> {
        &self.template
    }
    fn culling_distance(&self) -> &super::core::QualityScalableFloat {
        &self.culling_distance
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
}

impl super::entity::SpatialEntityDataTrait for EnvironmentDecalVolumeData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
}

impl super::entity::EntityDataTrait for EnvironmentDecalVolumeData {
}

impl super::entity::GameObjectDataTrait for EnvironmentDecalVolumeData {
}

impl super::core::DataBusPeerTrait for EnvironmentDecalVolumeData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for EnvironmentDecalVolumeData {
}

impl super::core::DataContainerTrait for EnvironmentDecalVolumeData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static ENVIRONMENTDECALVOLUMEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnvironmentDecalVolumeData",
    flags: MemberInfoFlags::new(101),
    module: "DecalVolumeSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EnvironmentDecalVolumeData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "IsEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnvironmentDecalVolumeData, is_enabled),
            },
            FieldInfoData {
                name: "Template",
                flags: MemberInfoFlags::new(0),
                field_type: "EnvironmentDecalVolumeTemplateBaseData",
                rust_offset: offset_of!(EnvironmentDecalVolumeData, template),
            },
            FieldInfoData {
                name: "CullingDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(EnvironmentDecalVolumeData, culling_distance),
            },
            FieldInfoData {
                name: "OverrideTemplateCullingDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EnvironmentDecalVolumeData, override_template_culling_distance),
            },
            FieldInfoData {
                name: "Alpha",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EnvironmentDecalVolumeData, alpha),
            },
            FieldInfoData {
                name: "InstanceParams",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(EnvironmentDecalVolumeData, instance_params),
            },
            FieldInfoData {
                name: "Row",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(EnvironmentDecalVolumeData, row),
            },
            FieldInfoData {
                name: "Column",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(EnvironmentDecalVolumeData, column),
            },
        ],
    }),
    array_type: Some(ENVIRONMENTDECALVOLUMEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EnvironmentDecalVolumeData {
    fn type_info(&self) -> &'static TypeInfo {
        ENVIRONMENTDECALVOLUMEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ENVIRONMENTDECALVOLUMEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnvironmentDecalVolumeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DecalVolumeSim",
    data: TypeInfoData::Array("EnvironmentDecalVolumeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EnvironmentDecalVolumeEntity {
    pub _glacier_base: super::entity::SpatialEntity,
}

pub trait EnvironmentDecalVolumeEntityTrait: super::entity::SpatialEntityTrait {
}

impl EnvironmentDecalVolumeEntityTrait for EnvironmentDecalVolumeEntity {
}

impl super::entity::SpatialEntityTrait for EnvironmentDecalVolumeEntity {
}

impl super::entity::EntityTrait for EnvironmentDecalVolumeEntity {
}

impl super::entity::EntityBusPeerTrait for EnvironmentDecalVolumeEntity {
}

pub static ENVIRONMENTDECALVOLUMEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnvironmentDecalVolumeEntity",
    flags: MemberInfoFlags::new(101),
    module: "DecalVolumeSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EnvironmentDecalVolumeEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(ENVIRONMENTDECALVOLUMEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EnvironmentDecalVolumeEntity {
    fn type_info(&self) -> &'static TypeInfo {
        ENVIRONMENTDECALVOLUMEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ENVIRONMENTDECALVOLUMEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnvironmentDecalVolumeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "DecalVolumeSim",
    data: TypeInfoData::Array("EnvironmentDecalVolumeEntity"),
    array_type: None,
    alignment: 8,
};


