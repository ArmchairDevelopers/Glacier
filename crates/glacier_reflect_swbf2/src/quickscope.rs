use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_quickscope_types(registry: &mut TypeRegistry) {
    registry.register_type(QUICKSCOPECONTROLENTITY_TYPE_INFO);
    registry.register_type(QUICKSCOPECONTROLENTITY_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct QuickscopeControlEntity {
}

pub const QUICKSCOPECONTROLENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeControlEntity",
    flags: MemberInfoFlags::new(101),
    module: "Quickscope",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(QUICKSCOPECONTROLENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for QuickscopeControlEntity {
    fn type_info() -> &'static TypeInfo {
        QUICKSCOPECONTROLENTITY_TYPE_INFO
    }
}


pub const QUICKSCOPECONTROLENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeControlEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Quickscope",
    data: TypeInfoData::Array("QuickscopeControlEntity-Array"),
    array_type: None,
    alignment: 8,
};


