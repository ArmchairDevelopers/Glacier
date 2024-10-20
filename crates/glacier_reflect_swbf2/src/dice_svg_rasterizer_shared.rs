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
    fn height(&self) -> &f32;
    fn resource(&self) -> &glacier_reflect::builtin::ResourceRef;
}

impl SvgImageTrait for SvgImage {
    fn width(&self) -> &f32 {
        &self.width
    }
    fn height(&self) -> &f32 {
        &self.height
    }
    fn resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.resource
    }
}

impl super::core::AssetTrait for SvgImage {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for SvgImage {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
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
}


pub static SVGIMAGE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SvgImage-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceSvgRasterizerShared",
    data: TypeInfoData::Array("SvgImage"),
    array_type: None,
    alignment: 8,
};


