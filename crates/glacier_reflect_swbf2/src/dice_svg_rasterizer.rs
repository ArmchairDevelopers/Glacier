use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_dice_svg_rasterizer_types(registry: &mut TypeRegistry) {
    registry.register_type(SVGIMAGEDATA_TYPE_INFO);
    registry.register_type(SVGIMAGEDATA_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SvgImageData {
}

pub const SVGIMAGEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SvgImageData",
    flags: MemberInfoFlags::new(101),
    module: "DiceSvgRasterizer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(SVGIMAGEDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SvgImageData {
    fn type_info() -> &'static TypeInfo {
        SVGIMAGEDATA_TYPE_INFO
    }
}


pub const SVGIMAGEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SvgImageData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceSvgRasterizer",
    data: TypeInfoData::Array("SvgImageData-Array"),
    array_type: None,
    alignment: 8,
};


