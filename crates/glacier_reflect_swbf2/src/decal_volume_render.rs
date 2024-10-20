use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_decal_volume_render_types(registry: &mut TypeRegistry) {
    registry.register_type(ENVIRONMENTDECALVOLUMETEMPLATEDATA_TYPE_INFO);
    registry.register_type(ENVIRONMENTDECALVOLUMETEMPLATEDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENVIRONMENTDECALVOLUMEMASKTYPE_TYPE_INFO);
    registry.register_type(ENVIRONMENTDECALVOLUMEMASKTYPE_ARRAY_TYPE_INFO);
}

#[derive(Clone, Debug, Default)]
pub struct EnvironmentDecalVolumeTemplateData {
    pub _glacier_base: super::decal_volume_base::EnvironmentDecalVolumeTemplateBaseData,
    pub shader: super::render_base::SurfaceShaderInstanceDataStruct,
    pub mask_type: EnvironmentDecalVolumeMaskType,
    pub sorting_priority: u8,
    pub culling_distance: super::core::QualityScalableFloat,
}

pub trait EnvironmentDecalVolumeTemplateDataTrait: super::decal_volume_base::EnvironmentDecalVolumeTemplateBaseDataTrait {
    fn shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct;
    fn mask_type(&self) -> &EnvironmentDecalVolumeMaskType;
    fn sorting_priority(&self) -> &u8;
    fn culling_distance(&self) -> &super::core::QualityScalableFloat;
}

impl EnvironmentDecalVolumeTemplateDataTrait for EnvironmentDecalVolumeTemplateData {
    fn shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct {
        &self.shader
    }
    fn mask_type(&self) -> &EnvironmentDecalVolumeMaskType {
        &self.mask_type
    }
    fn sorting_priority(&self) -> &u8 {
        &self.sorting_priority
    }
    fn culling_distance(&self) -> &super::core::QualityScalableFloat {
        &self.culling_distance
    }
}

impl super::decal_volume_base::EnvironmentDecalVolumeTemplateBaseDataTrait for EnvironmentDecalVolumeTemplateData {
}

impl super::core::AssetTrait for EnvironmentDecalVolumeTemplateData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for EnvironmentDecalVolumeTemplateData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static ENVIRONMENTDECALVOLUMETEMPLATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnvironmentDecalVolumeTemplateData",
    flags: MemberInfoFlags::new(101),
    module: "DecalVolumeRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::decal_volume_base::ENVIRONMENTDECALVOLUMETEMPLATEBASEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EnvironmentDecalVolumeTemplateData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Shader",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderInstanceDataStruct",
                rust_offset: offset_of!(EnvironmentDecalVolumeTemplateData, shader),
            },
            FieldInfoData {
                name: "MaskType",
                flags: MemberInfoFlags::new(0),
                field_type: "EnvironmentDecalVolumeMaskType",
                rust_offset: offset_of!(EnvironmentDecalVolumeTemplateData, mask_type),
            },
            FieldInfoData {
                name: "SortingPriority",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(EnvironmentDecalVolumeTemplateData, sorting_priority),
            },
            FieldInfoData {
                name: "CullingDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(EnvironmentDecalVolumeTemplateData, culling_distance),
            },
        ],
    }),
    array_type: Some(ENVIRONMENTDECALVOLUMETEMPLATEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EnvironmentDecalVolumeTemplateData {
    fn type_info(&self) -> &'static TypeInfo {
        ENVIRONMENTDECALVOLUMETEMPLATEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ENVIRONMENTDECALVOLUMETEMPLATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnvironmentDecalVolumeTemplateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DecalVolumeRender",
    data: TypeInfoData::Array("EnvironmentDecalVolumeTemplateData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum EnvironmentDecalVolumeMaskType {
    #[default]
    MaskType_NoMask = 0,
    MaskType_StaticOnly = 1,
    MaskType_DynamicOnly = 2,
    MaskType_TerrainOnly = 3,
}

pub static ENVIRONMENTDECALVOLUMEMASKTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnvironmentDecalVolumeMaskType",
    flags: MemberInfoFlags::new(49429),
    module: "DecalVolumeRender",
    data: TypeInfoData::Enum,
    array_type: Some(ENVIRONMENTDECALVOLUMEMASKTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EnvironmentDecalVolumeMaskType {
    fn type_info(&self) -> &'static TypeInfo {
        ENVIRONMENTDECALVOLUMEMASKTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ENVIRONMENTDECALVOLUMEMASKTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnvironmentDecalVolumeMaskType-Array",
    flags: MemberInfoFlags::new(145),
    module: "DecalVolumeRender",
    data: TypeInfoData::Array("EnvironmentDecalVolumeMaskType"),
    array_type: None,
    alignment: 8,
};


