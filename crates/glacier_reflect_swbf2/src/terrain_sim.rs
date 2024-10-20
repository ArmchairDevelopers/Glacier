use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_terrain_sim_types(registry: &mut TypeRegistry) {
    registry.register_type(WATERASSET_TYPE_INFO);
    registry.register_type(WATERASSET_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSTERRAINUPDATERCOMPONENTDATA_TYPE_INFO);
    registry.register_type(PHYSICSTERRAINUPDATERCOMPONENTDATA_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WaterAsset {
}

pub const WATERASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterAsset",
    flags: MemberInfoFlags::new(101),
    module: "TerrainSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSRESOURCECONTAINERASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WATERASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WaterAsset {
    fn type_info() -> &'static TypeInfo {
        WATERASSET_TYPE_INFO
    }
}


pub const WATERASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainSim",
    data: TypeInfoData::Array("WaterAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PhysicsTerrainUpdaterComponentData {
    pub in_categories: super::physics::PhysicsCategorySet,
    pub collides_with_categories: super::physics::PhysicsCategorySet,
}

pub const PHYSICSTERRAINUPDATERCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsTerrainUpdaterComponentData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "InCategories",
                flags: MemberInfoFlags::new(0),
                field_type: PHYSICSCATEGORYSET_TYPE_INFO,
                rust_offset: offset_of!(PhysicsTerrainUpdaterComponentData, in_categories),
            },
            FieldInfoData {
                name: "CollidesWithCategories",
                flags: MemberInfoFlags::new(0),
                field_type: PHYSICSCATEGORYSET_TYPE_INFO,
                rust_offset: offset_of!(PhysicsTerrainUpdaterComponentData, collides_with_categories),
            },
        ],
    }),
    array_type: Some(PHYSICSTERRAINUPDATERCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsTerrainUpdaterComponentData {
    fn type_info() -> &'static TypeInfo {
        PHYSICSTERRAINUPDATERCOMPONENTDATA_TYPE_INFO
    }
}


pub const PHYSICSTERRAINUPDATERCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsTerrainUpdaterComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainSim",
    data: TypeInfoData::Array("PhysicsTerrainUpdaterComponentData-Array"),
    array_type: None,
    alignment: 8,
};


