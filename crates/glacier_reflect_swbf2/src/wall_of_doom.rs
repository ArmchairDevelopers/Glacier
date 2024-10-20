use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_wall_of_doom_types(registry: &mut TypeRegistry) {
    registry.register_type(CLIENTWALLOFDOOMMESHENTITY_TYPE_INFO);
    registry.register_type(CLIENTWALLOFDOOMMESHENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTWALLOFDOOMENTITY_TYPE_INFO);
    registry.register_type(CLIENTWALLOFDOOMENTITY_ARRAY_TYPE_INFO);
}

#[derive(Clone, Debug, Default)]
pub struct ClientWallOfDoomMeshEntity {
    pub _glacier_base: super::game_client::ClientStaticModelEntity,
}

pub trait ClientWallOfDoomMeshEntityTrait: super::game_client::ClientStaticModelEntityTrait {
}

impl ClientWallOfDoomMeshEntityTrait for ClientWallOfDoomMeshEntity {
}

impl super::game_client::ClientStaticModelEntityTrait for ClientWallOfDoomMeshEntity {
}

impl super::gameplay_client_server::ClientPhysicsEntityTrait for ClientWallOfDoomMeshEntity {
}

impl super::gameplay_client_server::ClientGameComponentEntityTrait for ClientWallOfDoomMeshEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ClientWallOfDoomMeshEntity {
}

impl super::entity::ComponentEntityTrait for ClientWallOfDoomMeshEntity {
}

impl super::entity::SpatialEntityTrait for ClientWallOfDoomMeshEntity {
}

impl super::entity::EntityTrait for ClientWallOfDoomMeshEntity {
}

impl super::entity::EntityBusPeerTrait for ClientWallOfDoomMeshEntity {
}

pub static CLIENTWALLOFDOOMMESHENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWallOfDoomMeshEntity",
    flags: MemberInfoFlags::new(101),
    module: "WallOfDoom",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_client::CLIENTSTATICMODELENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientWallOfDoomMeshEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWALLOFDOOMMESHENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWallOfDoomMeshEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTWALLOFDOOMMESHENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTWALLOFDOOMMESHENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWallOfDoomMeshEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WallOfDoom",
    data: TypeInfoData::Array("ClientWallOfDoomMeshEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientWallOfDoomEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientWallOfDoomEntityTrait: super::entity::EntityTrait {
}

impl ClientWallOfDoomEntityTrait for ClientWallOfDoomEntity {
}

impl super::entity::EntityTrait for ClientWallOfDoomEntity {
}

impl super::entity::EntityBusPeerTrait for ClientWallOfDoomEntity {
}

pub static CLIENTWALLOFDOOMENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWallOfDoomEntity",
    flags: MemberInfoFlags::new(101),
    module: "WallOfDoom",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientWallOfDoomEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWALLOFDOOMENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWallOfDoomEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTWALLOFDOOMENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTWALLOFDOOMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWallOfDoomEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WallOfDoom",
    data: TypeInfoData::Array("ClientWallOfDoomEntity"),
    array_type: None,
    alignment: 8,
};


