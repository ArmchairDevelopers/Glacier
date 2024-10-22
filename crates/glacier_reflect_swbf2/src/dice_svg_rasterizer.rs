use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_dice_svg_rasterizer_types(registry: &mut TypeRegistry) {
    registry.register_type(SVGIMAGEDATA_TYPE_INFO);
    registry.register_type(SVGIMAGEDATA_ARRAY_TYPE_INFO);
}

#[derive(Clone, Debug, Default)]
pub struct SvgImageData {
}

pub trait SvgImageDataTrait: TypeObject {
}

impl SvgImageDataTrait for SvgImageData {
}

pub static SVGIMAGEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SvgImageData",
    flags: MemberInfoFlags::new(101),
    module: "DiceSvgRasterizer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SvgImageData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SVGIMAGEDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SvgImageData {
    fn type_info(&self) -> &'static TypeInfo {
        SVGIMAGEDATA_TYPE_INFO
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


pub static SVGIMAGEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SvgImageData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceSvgRasterizer",
    data: TypeInfoData::Array("SvgImageData"),
    array_type: None,
    alignment: 8,
};


