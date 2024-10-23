use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_decal_volume_sim_types(registry: &mut TypeRegistry) {
    registry.register_type(ENVIRONMENTDECALVOLUMEDATA_TYPE_INFO);
    registry.register_type(ENVIRONMENTDECALVOLUMEDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENVIRONMENTDECALVOLUMEENTITY_TYPE_INFO);
    registry.register_type(ENVIRONMENTDECALVOLUMEENTITY_ARRAY_TYPE_INFO);
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct EnvironmentDecalVolumeData {
    pub _glacier_base: super::entity::SpatialEntityData,
    pub is_enabled: bool,
    pub template: Option<LockedTypeObject /* super::decal_volume_base::EnvironmentDecalVolumeTemplateBaseData */>,
    pub culling_distance: super::core::QualityScalableFloat,
    pub override_template_culling_distance: f32,
    pub alpha: f32,
    pub instance_params: super::core::Vec3,
    pub row: u8,
    pub column: u8,
}

pub trait EnvironmentDecalVolumeDataTrait: super::entity::SpatialEntityDataTrait {
    fn is_enabled(&self) -> &bool;
    fn is_enabled_mut(&mut self) -> &mut bool;
    fn template(&self) -> &Option<LockedTypeObject /* super::decal_volume_base::EnvironmentDecalVolumeTemplateBaseData */>;
    fn template_mut(&mut self) -> &mut Option<LockedTypeObject /* super::decal_volume_base::EnvironmentDecalVolumeTemplateBaseData */>;
    fn culling_distance(&self) -> &super::core::QualityScalableFloat;
    fn culling_distance_mut(&mut self) -> &mut super::core::QualityScalableFloat;
    fn override_template_culling_distance(&self) -> &f32;
    fn override_template_culling_distance_mut(&mut self) -> &mut f32;
    fn alpha(&self) -> &f32;
    fn alpha_mut(&mut self) -> &mut f32;
    fn instance_params(&self) -> &super::core::Vec3;
    fn instance_params_mut(&mut self) -> &mut super::core::Vec3;
    fn row(&self) -> &u8;
    fn row_mut(&mut self) -> &mut u8;
    fn column(&self) -> &u8;
    fn column_mut(&mut self) -> &mut u8;
}

impl EnvironmentDecalVolumeDataTrait for EnvironmentDecalVolumeData {
    fn is_enabled(&self) -> &bool {
        &self.is_enabled
    }
    fn is_enabled_mut(&mut self) -> &mut bool {
        &mut self.is_enabled
    }
    fn template(&self) -> &Option<LockedTypeObject /* super::decal_volume_base::EnvironmentDecalVolumeTemplateBaseData */> {
        &self.template
    }
    fn template_mut(&mut self) -> &mut Option<LockedTypeObject /* super::decal_volume_base::EnvironmentDecalVolumeTemplateBaseData */> {
        &mut self.template
    }
    fn culling_distance(&self) -> &super::core::QualityScalableFloat {
        &self.culling_distance
    }
    fn culling_distance_mut(&mut self) -> &mut super::core::QualityScalableFloat {
        &mut self.culling_distance
    }
    fn override_template_culling_distance(&self) -> &f32 {
        &self.override_template_culling_distance
    }
    fn override_template_culling_distance_mut(&mut self) -> &mut f32 {
        &mut self.override_template_culling_distance
    }
    fn alpha(&self) -> &f32 {
        &self.alpha
    }
    fn alpha_mut(&mut self) -> &mut f32 {
        &mut self.alpha
    }
    fn instance_params(&self) -> &super::core::Vec3 {
        &self.instance_params
    }
    fn instance_params_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.instance_params
    }
    fn row(&self) -> &u8 {
        &self.row
    }
    fn row_mut(&mut self) -> &mut u8 {
        &mut self.row
    }
    fn column(&self) -> &u8 {
        &self.column
    }
    fn column_mut(&mut self) -> &mut u8 {
        &mut self.column
    }
}

impl super::entity::SpatialEntityDataTrait for EnvironmentDecalVolumeData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
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
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for EnvironmentDecalVolumeData {
}

impl super::core::DataContainerTrait for EnvironmentDecalVolumeData {
}

pub static ENVIRONMENTDECALVOLUMEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnvironmentDecalVolumeData",
    name_hash: 1771467047,
    flags: MemberInfoFlags::new(101),
    module: "DecalVolumeSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(EnvironmentDecalVolumeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EnvironmentDecalVolumeData as Default>::default())),
            create_boxed: || Box::new(<EnvironmentDecalVolumeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "IsEnabled",
                name_hash: 2323834330,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnvironmentDecalVolumeData, is_enabled),
            },
            FieldInfoData {
                name: "Template",
                name_hash: 2427043285,
                flags: MemberInfoFlags::new(0),
                field_type: "EnvironmentDecalVolumeTemplateBaseData",
                rust_offset: offset_of!(EnvironmentDecalVolumeData, template),
            },
            FieldInfoData {
                name: "CullingDistance",
                name_hash: 1073297232,
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(EnvironmentDecalVolumeData, culling_distance),
            },
            FieldInfoData {
                name: "OverrideTemplateCullingDistance",
                name_hash: 2641928820,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EnvironmentDecalVolumeData, override_template_culling_distance),
            },
            FieldInfoData {
                name: "Alpha",
                name_hash: 205677681,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EnvironmentDecalVolumeData, alpha),
            },
            FieldInfoData {
                name: "InstanceParams",
                name_hash: 976408944,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(EnvironmentDecalVolumeData, instance_params),
            },
            FieldInfoData {
                name: "Row",
                name_hash: 193465295,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(EnvironmentDecalVolumeData, row),
            },
            FieldInfoData {
                name: "Column",
                name_hash: 2713816499,
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


pub static ENVIRONMENTDECALVOLUMEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnvironmentDecalVolumeData-Array",
    name_hash: 1047831187,
    flags: MemberInfoFlags::new(145),
    module: "DecalVolumeSim",
    data: TypeInfoData::Array("EnvironmentDecalVolumeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 739177932,
    flags: MemberInfoFlags::new(101),
    module: "DecalVolumeSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITY_TYPE_INFO),
        super_class_offset: offset_of!(EnvironmentDecalVolumeEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EnvironmentDecalVolumeEntity as Default>::default())),
            create_boxed: || Box::new(<EnvironmentDecalVolumeEntity as Default>::default()),
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


pub static ENVIRONMENTDECALVOLUMEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnvironmentDecalVolumeEntity-Array",
    name_hash: 2273441400,
    flags: MemberInfoFlags::new(145),
    module: "DecalVolumeSim",
    data: TypeInfoData::Array("EnvironmentDecalVolumeEntity"),
    array_type: None,
    alignment: 8,
};


