use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_dice_svg_rasterizer_shared_types(registry: &mut TypeRegistry) {
    registry.register_type(SVGIMAGE_TYPE_INFO);
    registry.register_type(SVGIMAGE_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Debug)]
pub struct SvgImage {
    pub width: f32,
    pub height: f32,
    pub resource: super::core::ResourceRef,
}

pub const SVGIMAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SvgImage",
    flags: MemberInfoFlags::new(101),
    module: "DiceSvgRasterizerShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Width",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SvgImage, width),
            },
            FieldInfoData {
                name: "Height",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SvgImage, height),
            },
            FieldInfoData {
                name: "Resource",
                flags: MemberInfoFlags::new(0),
                field_type: RESOURCEREF_TYPE_INFO,
                rust_offset: offset_of!(SvgImage, resource),
            },
        ],
    }),
    array_type: Some(SVGIMAGE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SvgImage {
    fn type_info() -> &'static TypeInfo {
        SVGIMAGE_TYPE_INFO
    }
}


pub const SVGIMAGE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SvgImage-Array",
    flags: MemberInfoFlags::new(145),
    module: "DiceSvgRasterizerShared",
    data: TypeInfoData::Array("SvgImage-Array"),
    array_type: None,
    alignment: 8,
};


