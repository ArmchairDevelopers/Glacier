use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_dice_svg_rasterizer_shared_types(registry: &mut TypeRegistry) {
    registry.register_type(SVGIMAGE_TYPE_INFO);
    registry.register_type(SVGIMAGE_ARRAY_TYPE_INFO);
}

#[derive(Clone, Debug, Default)]
pub struct SvgImage {
    pub _glacier_base: super::core::Asset,
    pub width: f32,
    pub height: f32,
    pub resource: glacier_reflect::builtin::ResourceRef,
}

pub trait SvgImageTrait: super::core::AssetTrait {
    fn width(&self) -> &f32;
    fn width_mut(&mut self) -> &mut f32;
    fn height(&self) -> &f32;
    fn height_mut(&mut self) -> &mut f32;
    fn resource(&self) -> &glacier_reflect::builtin::ResourceRef;
    fn resource_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef;
}

impl SvgImageTrait for SvgImage {
    fn width(&self) -> &f32 {
        &self.width
    }
    fn width_mut(&mut self) -> &mut f32 {
        &mut self.width
    }
    fn height(&self) -> &f32 {
        &self.height
    }
    fn height_mut(&mut self) -> &mut f32 {
        &mut self.height
    }
    fn resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.resource
    }
    fn resource_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef {
        &mut self.resource
    }
}

impl super::core::AssetTrait for SvgImage {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for SvgImage {
}

pub static SVGIMAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SvgImage",
    flags: MemberInfoFlags::new(101),
    module: "DiceSvgRasterizerShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SvgImage as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Width",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SvgImage, width),
            },
            FieldInfoData {
                name: "Height",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SvgImage, height),
            },
            FieldInfoData {
                name: "Resource",
                flags: MemberInfoFlags::new(0),
                field_type: "ResourceRef",
                rust_offset: offset_of!(SvgImage, resource),
            },
        ],
    }),
    array_type: Some(SVGIMAGE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SvgImage {
    fn type_info(&self) -> &'static TypeInfo {
        SVGIMAGE_TYPE_INFO
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


pub static SVGIMAGE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SvgImage-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceSvgRasterizerShared",
    data: TypeInfoData::Array("SvgImage"),
    array_type: None,
    alignment: 8,
};


