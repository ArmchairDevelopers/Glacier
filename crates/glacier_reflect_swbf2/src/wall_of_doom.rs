use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_wall_of_doom_types(registry: &mut TypeRegistry) {
    registry.register_type(CLIENTWALLOFDOOMMESHENTITY_TYPE_INFO);
    registry.register_type(CLIENTWALLOFDOOMMESHENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTWALLOFDOOMENTITY_TYPE_INFO);
    registry.register_type(CLIENTWALLOFDOOMENTITY_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientWallOfDoomMeshEntity {
}

pub const CLIENTWALLOFDOOMMESHENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWallOfDoomMeshEntity",
    flags: MemberInfoFlags::new(101),
    module: "WallOfDoom",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTSTATICMODELENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWALLOFDOOMMESHENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWallOfDoomMeshEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTWALLOFDOOMMESHENTITY_TYPE_INFO
    }
}


pub const CLIENTWALLOFDOOMMESHENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWallOfDoomMeshEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WallOfDoom",
    data: TypeInfoData::Array("ClientWallOfDoomMeshEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientWallOfDoomEntity {
}

pub const CLIENTWALLOFDOOMENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWallOfDoomEntity",
    flags: MemberInfoFlags::new(101),
    module: "WallOfDoom",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWALLOFDOOMENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWallOfDoomEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTWALLOFDOOMENTITY_TYPE_INFO
    }
}


pub const CLIENTWALLOFDOOMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWallOfDoomEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WallOfDoom",
    data: TypeInfoData::Array("ClientWallOfDoomEntity-Array"),
    array_type: None,
    alignment: 8,
};


