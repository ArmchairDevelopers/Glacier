use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_pixel_painting_shared_types(registry: &mut TypeRegistry) {
    registry.register_type(PIXELPAINTINGBLUEPRINT_TYPE_INFO);
    registry.register_type(PIXELPAINTINGBLUEPRINT_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PixelPaintingBlueprint {
}

pub const PIXELPAINTINGBLUEPRINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PixelPaintingBlueprint",
    flags: MemberInfoFlags::new(101),
    module: "PixelPaintingShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(OBJECTBLUEPRINT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PIXELPAINTINGBLUEPRINT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PixelPaintingBlueprint {
    fn type_info() -> &'static TypeInfo {
        PIXELPAINTINGBLUEPRINT_TYPE_INFO
    }
}


pub const PIXELPAINTINGBLUEPRINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PixelPaintingBlueprint-Array",
    flags: MemberInfoFlags::new(145),
    module: "PixelPaintingShared",
    data: TypeInfoData::Array("PixelPaintingBlueprint-Array"),
    array_type: None,
    alignment: 8,
};


