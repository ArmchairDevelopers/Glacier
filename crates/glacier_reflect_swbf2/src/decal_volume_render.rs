use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_decal_volume_render_types(registry: &mut TypeRegistry) {
    registry.register_type(ENVIRONMENTDECALVOLUMETEMPLATEDATA_TYPE_INFO);
    registry.register_type(ENVIRONMENTDECALVOLUMETEMPLATEDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENVIRONMENTDECALVOLUMEMASKTYPE_TYPE_INFO);
    registry.register_type(ENVIRONMENTDECALVOLUMEMASKTYPE_ARRAY_TYPE_INFO);
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct EnvironmentDecalVolumeTemplateData {
    pub _glacier_base: super::decal_volume_base::EnvironmentDecalVolumeTemplateBaseData,
    pub shader: super::render_base::SurfaceShaderInstanceDataStruct,
    pub mask_type: EnvironmentDecalVolumeMaskType,
    pub sorting_priority: u8,
    pub culling_distance: super::core::QualityScalableFloat,
}

pub trait EnvironmentDecalVolumeTemplateDataTrait: super::decal_volume_base::EnvironmentDecalVolumeTemplateBaseDataTrait {
    fn shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct;
    fn shader_mut(&mut self) -> &mut super::render_base::SurfaceShaderInstanceDataStruct;
    fn mask_type(&self) -> &EnvironmentDecalVolumeMaskType;
    fn mask_type_mut(&mut self) -> &mut EnvironmentDecalVolumeMaskType;
    fn sorting_priority(&self) -> &u8;
    fn sorting_priority_mut(&mut self) -> &mut u8;
    fn culling_distance(&self) -> &super::core::QualityScalableFloat;
    fn culling_distance_mut(&mut self) -> &mut super::core::QualityScalableFloat;
}

impl EnvironmentDecalVolumeTemplateDataTrait for EnvironmentDecalVolumeTemplateData {
    fn shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct {
        &self.shader
    }
    fn shader_mut(&mut self) -> &mut super::render_base::SurfaceShaderInstanceDataStruct {
        &mut self.shader
    }
    fn mask_type(&self) -> &EnvironmentDecalVolumeMaskType {
        &self.mask_type
    }
    fn mask_type_mut(&mut self) -> &mut EnvironmentDecalVolumeMaskType {
        &mut self.mask_type
    }
    fn sorting_priority(&self) -> &u8 {
        &self.sorting_priority
    }
    fn sorting_priority_mut(&mut self) -> &mut u8 {
        &mut self.sorting_priority
    }
    fn culling_distance(&self) -> &super::core::QualityScalableFloat {
        &self.culling_distance
    }
    fn culling_distance_mut(&mut self) -> &mut super::core::QualityScalableFloat {
        &mut self.culling_distance
    }
}

impl super::decal_volume_base::EnvironmentDecalVolumeTemplateBaseDataTrait for EnvironmentDecalVolumeTemplateData {
}

impl super::core::AssetTrait for EnvironmentDecalVolumeTemplateData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for EnvironmentDecalVolumeTemplateData {
}

pub static ENVIRONMENTDECALVOLUMETEMPLATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnvironmentDecalVolumeTemplateData",
    name_hash: 1462032119,
    flags: MemberInfoFlags::new(101),
    module: "DecalVolumeRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::decal_volume_base::ENVIRONMENTDECALVOLUMETEMPLATEBASEDATA_TYPE_INFO),
        super_class_offset: offset_of!(EnvironmentDecalVolumeTemplateData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EnvironmentDecalVolumeTemplateData as Default>::default())),
            create_boxed: || Box::new(<EnvironmentDecalVolumeTemplateData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Shader",
                name_hash: 3352909900,
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderInstanceDataStruct",
                rust_offset: offset_of!(EnvironmentDecalVolumeTemplateData, shader),
            },
            FieldInfoData {
                name: "MaskType",
                name_hash: 950003465,
                flags: MemberInfoFlags::new(0),
                field_type: "EnvironmentDecalVolumeMaskType",
                rust_offset: offset_of!(EnvironmentDecalVolumeTemplateData, mask_type),
            },
            FieldInfoData {
                name: "SortingPriority",
                name_hash: 3523655821,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(EnvironmentDecalVolumeTemplateData, sorting_priority),
            },
            FieldInfoData {
                name: "CullingDistance",
                name_hash: 1073297232,
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


pub static ENVIRONMENTDECALVOLUMETEMPLATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnvironmentDecalVolumeTemplateData-Array",
    name_hash: 3967754435,
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
    name_hash: 2893760987,
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


pub static ENVIRONMENTDECALVOLUMEMASKTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnvironmentDecalVolumeMaskType-Array",
    name_hash: 3072291055,
    flags: MemberInfoFlags::new(145),
    module: "DecalVolumeRender",
    data: TypeInfoData::Array("EnvironmentDecalVolumeMaskType"),
    array_type: None,
    alignment: 8,
};


