use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_e_a_character_physics_shared_types(registry: &mut TypeRegistry) {
    registry.register_type(EACHARACTERPHYSICSSTATETESTENTITYDATA_TYPE_INFO);
    registry.register_type(EACHARACTERPHYSICSSTATETESTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(EACHARACTERPHYSICSCOMPONENTPOSITIONS_TYPE_INFO);
    registry.register_type(EACHARACTERPHYSICSCOMPONENTPOSITIONS_ARRAY_TYPE_INFO);
    registry.register_type(EACHARACTERPHYSICSCOMPONENTPOSITION_TYPE_INFO);
    registry.register_type(EACHARACTERPHYSICSCOMPONENTPOSITION_ARRAY_TYPE_INFO);
    registry.register_type(EACHARACTERPHYSICSCOMPONENTDATA_TYPE_INFO);
    registry.register_type(EACHARACTERPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EACharacterPhysicsStateTestEntityData {
    pub realm: super::core::Realm,
}

pub const EACHARACTERPHYSICSSTATETESTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EACharacterPhysicsStateTestEntityData",
    flags: MemberInfoFlags::new(101),
    module: "EACharacterPhysicsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(EACharacterPhysicsStateTestEntityData, realm),
            },
        ],
    }),
    array_type: Some(EACHARACTERPHYSICSSTATETESTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EACharacterPhysicsStateTestEntityData {
    fn type_info() -> &'static TypeInfo {
        EACHARACTERPHYSICSSTATETESTENTITYDATA_TYPE_INFO
    }
}


pub const EACHARACTERPHYSICSSTATETESTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EACharacterPhysicsStateTestEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "EACharacterPhysicsShared",
    data: TypeInfoData::Array("EACharacterPhysicsStateTestEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EACharacterPhysicsComponentPositions {
}

pub const EACHARACTERPHYSICSCOMPONENTPOSITIONS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EACharacterPhysicsComponentPositions",
    flags: MemberInfoFlags::new(73),
    module: "EACharacterPhysicsShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(EACHARACTERPHYSICSCOMPONENTPOSITIONS_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EACharacterPhysicsComponentPositions {
    fn type_info() -> &'static TypeInfo {
        EACHARACTERPHYSICSCOMPONENTPOSITIONS_TYPE_INFO
    }
}


pub const EACHARACTERPHYSICSCOMPONENTPOSITIONS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EACharacterPhysicsComponentPositions-Array",
    flags: MemberInfoFlags::new(145),
    module: "EACharacterPhysicsShared",
    data: TypeInfoData::Array("EACharacterPhysicsComponentPositions-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EACharacterPhysicsComponentPosition {
    pub orientation: super::core::Quat,
    pub translation: super::core::Vec3,
}

pub const EACHARACTERPHYSICSCOMPONENTPOSITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EACharacterPhysicsComponentPosition",
    flags: MemberInfoFlags::new(36937),
    module: "EACharacterPhysicsShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Orientation",
                flags: MemberInfoFlags::new(0),
                field_type: QUAT_TYPE_INFO,
                rust_offset: offset_of!(EACharacterPhysicsComponentPosition, orientation),
            },
            FieldInfoData {
                name: "Translation",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(EACharacterPhysicsComponentPosition, translation),
            },
        ],
    }),
    array_type: Some(EACHARACTERPHYSICSCOMPONENTPOSITION_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EACharacterPhysicsComponentPosition {
    fn type_info() -> &'static TypeInfo {
        EACHARACTERPHYSICSCOMPONENTPOSITION_TYPE_INFO
    }
}


pub const EACHARACTERPHYSICSCOMPONENTPOSITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EACharacterPhysicsComponentPosition-Array",
    flags: MemberInfoFlags::new(145),
    module: "EACharacterPhysicsShared",
    data: TypeInfoData::Array("EACharacterPhysicsComponentPosition-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EACharacterPhysicsComponentData {
    pub realm: super::core::Realm,
    pub client_authoritative: bool,
    pub material_pair: super::entity::MaterialDecl,
    pub physics_blueprint: super::entity::ObjectBlueprint,
    pub bone_materials: Vec<super::entity::MaterialDecl>,
}

pub const EACHARACTERPHYSICSCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EACharacterPhysicsComponentData",
    flags: MemberInfoFlags::new(101),
    module: "EACharacterPhysicsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMECOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(EACharacterPhysicsComponentData, realm),
            },
            FieldInfoData {
                name: "ClientAuthoritative",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EACharacterPhysicsComponentData, client_authoritative),
            },
            FieldInfoData {
                name: "MaterialPair",
                flags: MemberInfoFlags::new(0),
                field_type: MATERIALDECL_TYPE_INFO,
                rust_offset: offset_of!(EACharacterPhysicsComponentData, material_pair),
            },
            FieldInfoData {
                name: "PhysicsBlueprint",
                flags: MemberInfoFlags::new(0),
                field_type: OBJECTBLUEPRINT_TYPE_INFO,
                rust_offset: offset_of!(EACharacterPhysicsComponentData, physics_blueprint),
            },
            FieldInfoData {
                name: "BoneMaterials",
                flags: MemberInfoFlags::new(144),
                field_type: MATERIALDECL_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EACharacterPhysicsComponentData, bone_materials),
            },
        ],
    }),
    array_type: Some(EACHARACTERPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EACharacterPhysicsComponentData {
    fn type_info() -> &'static TypeInfo {
        EACHARACTERPHYSICSCOMPONENTDATA_TYPE_INFO
    }
}


pub const EACHARACTERPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EACharacterPhysicsComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "EACharacterPhysicsShared",
    data: TypeInfoData::Array("EACharacterPhysicsComponentData-Array"),
    array_type: None,
    alignment: 8,
};


