use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_weapon_types(registry: &mut TypeRegistry) {
    registry.register_type(CLIENTWEAPON_TYPE_INFO);
    registry.register_type(CLIENTWEAPON_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPROXYPROJECTILEENTITY_TYPE_INFO);
    registry.register_type(CLIENTPROXYPROJECTILEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPROXYMISSILEENTITY_TYPE_INFO);
    registry.register_type(CLIENTPROXYMISSILEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPROJECTILEENTITY_TYPE_INFO);
    registry.register_type(CLIENTPROJECTILEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTMISSILEENTITY_TYPE_INFO);
    registry.register_type(CLIENTMISSILEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTGHOSTPROJECTILEENTITY_TYPE_INFO);
    registry.register_type(CLIENTGHOSTPROJECTILEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTBULLETENTITY_TYPE_INFO);
    registry.register_type(CLIENTBULLETENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTWEAPONINPUTROUTERCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTWEAPONINPUTROUTERCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTWEAPONCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTWEAPONCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTMISSILEPHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTMISSILEPHYSICSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCHARACTERSIMPLEWEAPONCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTCHARACTERSIMPLEWEAPONCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERBULLETENTITY_TYPE_INFO);
    registry.register_type(SERVERBULLETENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERWEAPONINPUTROUTERCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERWEAPONINPUTROUTERCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERWEAPONCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERWEAPONCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERMISSILEPHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERMISSILEPHYSICSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCHARACTERSIMPLEWEAPONCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERCHARACTERSIMPLEWEAPONCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(WEAPON_TYPE_INFO);
    registry.register_type(WEAPON_ARRAY_TYPE_INFO);
    registry.register_type(SERVERWEAPONOVERRIDEAIDATAMESSAGE_TYPE_INFO);
    registry.register_type(SERVERWEAPONPROJECTILETIMEOUTMESSAGE_TYPE_INFO);
    registry.register_type(SERVERWEAPONREPLACEDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERWEAPONWEAPONCOMPONENTACTIVATEMESSAGE_TYPE_INFO);
    registry.register_type(SERVERWEAPONWEAPONCOMPONENTRELOADMESSAGE_TYPE_INFO);
    registry.register_type(SERVERWEAPONPLAYERRESUPPLYMESSAGE_TYPE_INFO);
    registry.register_type(SERVERWEAPONPLAYERRELOADMESSAGE_TYPE_INFO);
    registry.register_type(SERVERWEAPONPLAYERWEAPONREMOVEDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERWEAPONPLAYERWEAPONRELOADENDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERWEAPONPLAYERPRIMARYFIRESHOTSPAWNEDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERWEAPONPLAYERPRIMARYOUTOFAMMOMESSAGE_TYPE_INFO);
    registry.register_type(SERVERWEAPONARTILLERYFIREDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERWEAPONLASERDESIGNATORMESSAGE_TYPE_INFO);
    registry.register_type(SERVERWEAPONMORTARSTRIKEMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTWEAPONREPLACEDMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTWEAPONWEAPONCOMPONENTACTIVATEMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTWEAPONPLAYERPRIMARYOUTOFAMMOMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTWEAPONPLAYERWEAPONRELOADENDMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTWEAPONPLAYERWEAPONRELOADBEGINMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTWEAPONPLAYERPRIMARYWEAPONFIREMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTWEAPONPLAYERWEAPONCHANGECORRECTIONMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTWEAPONPLAYERWEAPONCHANGEMESSAGE_TYPE_INFO);
    registry.register_type(SERVERWEAPON_TYPE_INFO);
    registry.register_type(SERVERWEAPON_ARRAY_TYPE_INFO);
    registry.register_type(SERVERPROJECTILEENTITY_TYPE_INFO);
    registry.register_type(SERVERPROJECTILEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERMISSILEENTITY_TYPE_INFO);
    registry.register_type(SERVERMISSILEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERGHOSTPROJECTILEENTITY_TYPE_INFO);
    registry.register_type(SERVERGHOSTPROJECTILEENTITY_ARRAY_TYPE_INFO);
}

#[derive(Clone, Debug, Default)]
pub struct ClientWeapon {
    pub _glacier_base: Weapon,
}

pub trait ClientWeaponTrait: WeaponTrait {
}

impl ClientWeaponTrait for ClientWeapon {
}

impl WeaponTrait for ClientWeapon {
}

impl super::game_common::ToolTrait for ClientWeapon {
}

pub static CLIENTWEAPON_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeapon",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPON_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientWeapon as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWEAPON_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWeapon {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTWEAPON_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTWEAPON_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeapon-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("ClientWeapon"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientProxyProjectileEntity {
    pub _glacier_base: ClientProjectileEntity,
}

pub trait ClientProxyProjectileEntityTrait: ClientProjectileEntityTrait {
}

impl ClientProxyProjectileEntityTrait for ClientProxyProjectileEntity {
}

impl ClientProjectileEntityTrait for ClientProxyProjectileEntity {
}

impl super::gameplay_client_server::ClientPhysicsEntityTrait for ClientProxyProjectileEntity {
}

impl super::gameplay_client_server::ClientGameComponentEntityTrait for ClientProxyProjectileEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ClientProxyProjectileEntity {
}

impl super::entity::ComponentEntityTrait for ClientProxyProjectileEntity {
}

impl super::entity::SpatialEntityTrait for ClientProxyProjectileEntity {
}

impl super::entity::EntityTrait for ClientProxyProjectileEntity {
}

impl super::entity::EntityBusPeerTrait for ClientProxyProjectileEntity {
}

pub static CLIENTPROXYPROJECTILEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientProxyProjectileEntity",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTPROJECTILEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientProxyProjectileEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPROXYPROJECTILEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientProxyProjectileEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPROXYPROJECTILEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTPROXYPROJECTILEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientProxyProjectileEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("ClientProxyProjectileEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientProxyMissileEntity {
    pub _glacier_base: ClientProxyProjectileEntity,
}

pub trait ClientProxyMissileEntityTrait: ClientProxyProjectileEntityTrait {
}

impl ClientProxyMissileEntityTrait for ClientProxyMissileEntity {
}

impl ClientProxyProjectileEntityTrait for ClientProxyMissileEntity {
}

impl ClientProjectileEntityTrait for ClientProxyMissileEntity {
}

impl super::gameplay_client_server::ClientPhysicsEntityTrait for ClientProxyMissileEntity {
}

impl super::gameplay_client_server::ClientGameComponentEntityTrait for ClientProxyMissileEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ClientProxyMissileEntity {
}

impl super::entity::ComponentEntityTrait for ClientProxyMissileEntity {
}

impl super::entity::SpatialEntityTrait for ClientProxyMissileEntity {
}

impl super::entity::EntityTrait for ClientProxyMissileEntity {
}

impl super::entity::EntityBusPeerTrait for ClientProxyMissileEntity {
}

pub static CLIENTPROXYMISSILEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientProxyMissileEntity",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTPROXYPROJECTILEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientProxyMissileEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPROXYMISSILEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientProxyMissileEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPROXYMISSILEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTPROXYMISSILEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientProxyMissileEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("ClientProxyMissileEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientProjectileEntity {
    pub _glacier_base: super::gameplay_client_server::ClientPhysicsEntity,
}

pub trait ClientProjectileEntityTrait: super::gameplay_client_server::ClientPhysicsEntityTrait {
}

impl ClientProjectileEntityTrait for ClientProjectileEntity {
}

impl super::gameplay_client_server::ClientPhysicsEntityTrait for ClientProjectileEntity {
}

impl super::gameplay_client_server::ClientGameComponentEntityTrait for ClientProjectileEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ClientProjectileEntity {
}

impl super::entity::ComponentEntityTrait for ClientProjectileEntity {
}

impl super::entity::SpatialEntityTrait for ClientProjectileEntity {
}

impl super::entity::EntityTrait for ClientProjectileEntity {
}

impl super::entity::EntityBusPeerTrait for ClientProjectileEntity {
}

pub static CLIENTPROJECTILEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientProjectileEntity",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTPHYSICSENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientProjectileEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPROJECTILEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientProjectileEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPROJECTILEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTPROJECTILEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientProjectileEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("ClientProjectileEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientMissileEntity {
    pub _glacier_base: ClientGhostProjectileEntity,
}

pub trait ClientMissileEntityTrait: ClientGhostProjectileEntityTrait {
}

impl ClientMissileEntityTrait for ClientMissileEntity {
}

impl ClientGhostProjectileEntityTrait for ClientMissileEntity {
}

impl ClientProjectileEntityTrait for ClientMissileEntity {
}

impl super::gameplay_client_server::ClientPhysicsEntityTrait for ClientMissileEntity {
}

impl super::gameplay_client_server::ClientGameComponentEntityTrait for ClientMissileEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ClientMissileEntity {
}

impl super::entity::ComponentEntityTrait for ClientMissileEntity {
}

impl super::entity::SpatialEntityTrait for ClientMissileEntity {
}

impl super::entity::EntityTrait for ClientMissileEntity {
}

impl super::entity::EntityBusPeerTrait for ClientMissileEntity {
}

pub static CLIENTMISSILEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMissileEntity",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGHOSTPROJECTILEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientMissileEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTMISSILEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientMissileEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTMISSILEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTMISSILEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMissileEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("ClientMissileEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientGhostProjectileEntity {
    pub _glacier_base: ClientProjectileEntity,
}

pub trait ClientGhostProjectileEntityTrait: ClientProjectileEntityTrait {
}

impl ClientGhostProjectileEntityTrait for ClientGhostProjectileEntity {
}

impl ClientProjectileEntityTrait for ClientGhostProjectileEntity {
}

impl super::gameplay_client_server::ClientPhysicsEntityTrait for ClientGhostProjectileEntity {
}

impl super::gameplay_client_server::ClientGameComponentEntityTrait for ClientGhostProjectileEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ClientGhostProjectileEntity {
}

impl super::entity::ComponentEntityTrait for ClientGhostProjectileEntity {
}

impl super::entity::SpatialEntityTrait for ClientGhostProjectileEntity {
}

impl super::entity::EntityTrait for ClientGhostProjectileEntity {
}

impl super::entity::EntityBusPeerTrait for ClientGhostProjectileEntity {
}

pub static CLIENTGHOSTPROJECTILEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGhostProjectileEntity",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTPROJECTILEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientGhostProjectileEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTGHOSTPROJECTILEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientGhostProjectileEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTGHOSTPROJECTILEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTGHOSTPROJECTILEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGhostProjectileEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("ClientGhostProjectileEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientBulletEntity {
    pub _glacier_base: ClientProjectileEntity,
}

pub trait ClientBulletEntityTrait: ClientProjectileEntityTrait {
}

impl ClientBulletEntityTrait for ClientBulletEntity {
}

impl ClientProjectileEntityTrait for ClientBulletEntity {
}

impl super::gameplay_client_server::ClientPhysicsEntityTrait for ClientBulletEntity {
}

impl super::gameplay_client_server::ClientGameComponentEntityTrait for ClientBulletEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ClientBulletEntity {
}

impl super::entity::ComponentEntityTrait for ClientBulletEntity {
}

impl super::entity::SpatialEntityTrait for ClientBulletEntity {
}

impl super::entity::EntityTrait for ClientBulletEntity {
}

impl super::entity::EntityBusPeerTrait for ClientBulletEntity {
}

pub static CLIENTBULLETENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBulletEntity",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTPROJECTILEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientBulletEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTBULLETENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientBulletEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTBULLETENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTBULLETENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBulletEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("ClientBulletEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientWeaponInputRouterComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientWeaponInputRouterComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientWeaponInputRouterComponentTrait for ClientWeaponInputRouterComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientWeaponInputRouterComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientWeaponInputRouterComponent {
}

impl super::entity::ComponentTrait for ClientWeaponInputRouterComponent {
}

impl super::entity::EntityBusPeerTrait for ClientWeaponInputRouterComponent {
}

pub static CLIENTWEAPONINPUTROUTERCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponInputRouterComponent",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientWeaponInputRouterComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWEAPONINPUTROUTERCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWeaponInputRouterComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTWEAPONINPUTROUTERCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTWEAPONINPUTROUTERCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponInputRouterComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("ClientWeaponInputRouterComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientWeaponComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientWeaponComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientWeaponComponentTrait for ClientWeaponComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientWeaponComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientWeaponComponent {
}

impl super::entity::ComponentTrait for ClientWeaponComponent {
}

impl super::entity::EntityBusPeerTrait for ClientWeaponComponent {
}

pub static CLIENTWEAPONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponComponent",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientWeaponComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWEAPONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWeaponComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTWEAPONCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTWEAPONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("ClientWeaponComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientMissilePhysicsComponent {
    pub _glacier_base: super::physics::PartPhysicsComponent,
}

pub trait ClientMissilePhysicsComponentTrait: super::physics::PartPhysicsComponentTrait {
}

impl ClientMissilePhysicsComponentTrait for ClientMissilePhysicsComponent {
}

impl super::physics::PartPhysicsComponentTrait for ClientMissilePhysicsComponent {
}

impl super::physics::PhysicsComponentTrait for ClientMissilePhysicsComponent {
}

impl super::entity::ComponentTrait for ClientMissilePhysicsComponent {
}

impl super::entity::EntityBusPeerTrait for ClientMissilePhysicsComponent {
}

pub static CLIENTMISSILEPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMissilePhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::PARTPHYSICSCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientMissilePhysicsComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTMISSILEPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientMissilePhysicsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTMISSILEPHYSICSCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTMISSILEPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMissilePhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("ClientMissilePhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientCharacterSimpleWeaponComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientCharacterSimpleWeaponComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientCharacterSimpleWeaponComponentTrait for ClientCharacterSimpleWeaponComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientCharacterSimpleWeaponComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientCharacterSimpleWeaponComponent {
}

impl super::entity::ComponentTrait for ClientCharacterSimpleWeaponComponent {
}

impl super::entity::EntityBusPeerTrait for ClientCharacterSimpleWeaponComponent {
}

pub static CLIENTCHARACTERSIMPLEWEAPONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterSimpleWeaponComponent",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCharacterSimpleWeaponComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCHARACTERSIMPLEWEAPONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCharacterSimpleWeaponComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCHARACTERSIMPLEWEAPONCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTCHARACTERSIMPLEWEAPONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterSimpleWeaponComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("ClientCharacterSimpleWeaponComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerBulletEntity {
    pub _glacier_base: ServerProjectileEntity,
}

pub trait ServerBulletEntityTrait: ServerProjectileEntityTrait {
}

impl ServerBulletEntityTrait for ServerBulletEntity {
}

impl ServerProjectileEntityTrait for ServerBulletEntity {
}

impl super::game_server::ServerPhysicsEntityTrait for ServerBulletEntity {
}

impl super::game_server::ServerGameComponentEntityTrait for ServerBulletEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ServerBulletEntity {
}

impl super::entity::ComponentEntityTrait for ServerBulletEntity {
}

impl super::entity::SpatialEntityTrait for ServerBulletEntity {
}

impl super::entity::EntityTrait for ServerBulletEntity {
}

impl super::entity::EntityBusPeerTrait for ServerBulletEntity {
}

pub static SERVERBULLETENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBulletEntity",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERPROJECTILEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerBulletEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERBULLETENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerBulletEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERBULLETENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERBULLETENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBulletEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("ServerBulletEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerWeaponInputRouterComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerWeaponInputRouterComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerWeaponInputRouterComponentTrait for ServerWeaponInputRouterComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerWeaponInputRouterComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerWeaponInputRouterComponent {
}

impl super::entity::ComponentTrait for ServerWeaponInputRouterComponent {
}

impl super::entity::EntityBusPeerTrait for ServerWeaponInputRouterComponent {
}

pub static SERVERWEAPONINPUTROUTERCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponInputRouterComponent",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerWeaponInputRouterComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERWEAPONINPUTROUTERCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerWeaponInputRouterComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERWEAPONINPUTROUTERCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERWEAPONINPUTROUTERCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponInputRouterComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("ServerWeaponInputRouterComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerWeaponComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerWeaponComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerWeaponComponentTrait for ServerWeaponComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerWeaponComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerWeaponComponent {
}

impl super::entity::ComponentTrait for ServerWeaponComponent {
}

impl super::entity::EntityBusPeerTrait for ServerWeaponComponent {
}

pub static SERVERWEAPONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponComponent",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerWeaponComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERWEAPONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerWeaponComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERWEAPONCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERWEAPONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("ServerWeaponComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerMissilePhysicsComponent {
    pub _glacier_base: super::physics::PartPhysicsComponent,
}

pub trait ServerMissilePhysicsComponentTrait: super::physics::PartPhysicsComponentTrait {
}

impl ServerMissilePhysicsComponentTrait for ServerMissilePhysicsComponent {
}

impl super::physics::PartPhysicsComponentTrait for ServerMissilePhysicsComponent {
}

impl super::physics::PhysicsComponentTrait for ServerMissilePhysicsComponent {
}

impl super::entity::ComponentTrait for ServerMissilePhysicsComponent {
}

impl super::entity::EntityBusPeerTrait for ServerMissilePhysicsComponent {
}

pub static SERVERMISSILEPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMissilePhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::PARTPHYSICSCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerMissilePhysicsComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERMISSILEPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerMissilePhysicsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERMISSILEPHYSICSCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERMISSILEPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMissilePhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("ServerMissilePhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerCharacterSimpleWeaponComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerCharacterSimpleWeaponComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerCharacterSimpleWeaponComponentTrait for ServerCharacterSimpleWeaponComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerCharacterSimpleWeaponComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerCharacterSimpleWeaponComponent {
}

impl super::entity::ComponentTrait for ServerCharacterSimpleWeaponComponent {
}

impl super::entity::EntityBusPeerTrait for ServerCharacterSimpleWeaponComponent {
}

pub static SERVERCHARACTERSIMPLEWEAPONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterSimpleWeaponComponent",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCharacterSimpleWeaponComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCHARACTERSIMPLEWEAPONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCharacterSimpleWeaponComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCHARACTERSIMPLEWEAPONCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERCHARACTERSIMPLEWEAPONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterSimpleWeaponComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("ServerCharacterSimpleWeaponComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Weapon {
    pub _glacier_base: super::game_common::Tool,
}

pub trait WeaponTrait: super::game_common::ToolTrait {
}

impl WeaponTrait for Weapon {
}

impl super::game_common::ToolTrait for Weapon {
}

pub static WEAPON_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Weapon",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_common::TOOL_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Weapon as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(WEAPON_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Weapon {
    fn type_info(&self) -> &'static TypeInfo {
        WEAPON_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WEAPON_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Weapon-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("Weapon"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerWeaponOverrideAIDataMessage {
}

pub trait ServerWeaponOverrideAIDataMessageTrait: TypeObject {
}

impl ServerWeaponOverrideAIDataMessageTrait for ServerWeaponOverrideAIDataMessage {
}

pub static SERVERWEAPONOVERRIDEAIDATAMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponOverrideAIDataMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerWeaponOverrideAIDataMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerWeaponOverrideAIDataMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERWEAPONOVERRIDEAIDATAMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ServerWeaponProjectileTimeoutMessage {
}

pub trait ServerWeaponProjectileTimeoutMessageTrait: TypeObject {
}

impl ServerWeaponProjectileTimeoutMessageTrait for ServerWeaponProjectileTimeoutMessage {
}

pub static SERVERWEAPONPROJECTILETIMEOUTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponProjectileTimeoutMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerWeaponProjectileTimeoutMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ServerWeaponProjectileTimeoutMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERWEAPONPROJECTILETIMEOUTMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ServerWeaponReplacedMessage {
}

pub trait ServerWeaponReplacedMessageTrait: TypeObject {
}

impl ServerWeaponReplacedMessageTrait for ServerWeaponReplacedMessage {
}

pub static SERVERWEAPONREPLACEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponReplacedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerWeaponReplacedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerWeaponReplacedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERWEAPONREPLACEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ServerWeaponWeaponComponentActivateMessage {
}

pub trait ServerWeaponWeaponComponentActivateMessageTrait: TypeObject {
}

impl ServerWeaponWeaponComponentActivateMessageTrait for ServerWeaponWeaponComponentActivateMessage {
}

pub static SERVERWEAPONWEAPONCOMPONENTACTIVATEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponWeaponComponentActivateMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerWeaponWeaponComponentActivateMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerWeaponWeaponComponentActivateMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERWEAPONWEAPONCOMPONENTACTIVATEMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ServerWeaponWeaponComponentReloadMessage {
}

pub trait ServerWeaponWeaponComponentReloadMessageTrait: TypeObject {
}

impl ServerWeaponWeaponComponentReloadMessageTrait for ServerWeaponWeaponComponentReloadMessage {
}

pub static SERVERWEAPONWEAPONCOMPONENTRELOADMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponWeaponComponentReloadMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerWeaponWeaponComponentReloadMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerWeaponWeaponComponentReloadMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERWEAPONWEAPONCOMPONENTRELOADMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ServerWeaponPlayerResupplyMessage {
}

pub trait ServerWeaponPlayerResupplyMessageTrait: TypeObject {
}

impl ServerWeaponPlayerResupplyMessageTrait for ServerWeaponPlayerResupplyMessage {
}

pub static SERVERWEAPONPLAYERRESUPPLYMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponPlayerResupplyMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerWeaponPlayerResupplyMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerWeaponPlayerResupplyMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERWEAPONPLAYERRESUPPLYMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ServerWeaponPlayerReloadMessage {
}

pub trait ServerWeaponPlayerReloadMessageTrait: TypeObject {
}

impl ServerWeaponPlayerReloadMessageTrait for ServerWeaponPlayerReloadMessage {
}

pub static SERVERWEAPONPLAYERRELOADMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponPlayerReloadMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerWeaponPlayerReloadMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ServerWeaponPlayerReloadMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERWEAPONPLAYERRELOADMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ServerWeaponPlayerWeaponRemovedMessage {
}

pub trait ServerWeaponPlayerWeaponRemovedMessageTrait: TypeObject {
}

impl ServerWeaponPlayerWeaponRemovedMessageTrait for ServerWeaponPlayerWeaponRemovedMessage {
}

pub static SERVERWEAPONPLAYERWEAPONREMOVEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponPlayerWeaponRemovedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerWeaponPlayerWeaponRemovedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerWeaponPlayerWeaponRemovedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERWEAPONPLAYERWEAPONREMOVEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ServerWeaponPlayerWeaponReloadEndMessage {
}

pub trait ServerWeaponPlayerWeaponReloadEndMessageTrait: TypeObject {
}

impl ServerWeaponPlayerWeaponReloadEndMessageTrait for ServerWeaponPlayerWeaponReloadEndMessage {
}

pub static SERVERWEAPONPLAYERWEAPONRELOADENDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponPlayerWeaponReloadEndMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerWeaponPlayerWeaponReloadEndMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerWeaponPlayerWeaponReloadEndMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERWEAPONPLAYERWEAPONRELOADENDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ServerWeaponPlayerPrimaryFireShotSpawnedMessage {
}

pub trait ServerWeaponPlayerPrimaryFireShotSpawnedMessageTrait: TypeObject {
}

impl ServerWeaponPlayerPrimaryFireShotSpawnedMessageTrait for ServerWeaponPlayerPrimaryFireShotSpawnedMessage {
}

pub static SERVERWEAPONPLAYERPRIMARYFIRESHOTSPAWNEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponPlayerPrimaryFireShotSpawnedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerWeaponPlayerPrimaryFireShotSpawnedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerWeaponPlayerPrimaryFireShotSpawnedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERWEAPONPLAYERPRIMARYFIRESHOTSPAWNEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ServerWeaponPlayerPrimaryOutOfAmmoMessage {
}

pub trait ServerWeaponPlayerPrimaryOutOfAmmoMessageTrait: TypeObject {
}

impl ServerWeaponPlayerPrimaryOutOfAmmoMessageTrait for ServerWeaponPlayerPrimaryOutOfAmmoMessage {
}

pub static SERVERWEAPONPLAYERPRIMARYOUTOFAMMOMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponPlayerPrimaryOutOfAmmoMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerWeaponPlayerPrimaryOutOfAmmoMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerWeaponPlayerPrimaryOutOfAmmoMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERWEAPONPLAYERPRIMARYOUTOFAMMOMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ServerWeaponArtilleryFiredMessage {
}

pub trait ServerWeaponArtilleryFiredMessageTrait: TypeObject {
}

impl ServerWeaponArtilleryFiredMessageTrait for ServerWeaponArtilleryFiredMessage {
}

pub static SERVERWEAPONARTILLERYFIREDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponArtilleryFiredMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerWeaponArtilleryFiredMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ServerWeaponArtilleryFiredMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERWEAPONARTILLERYFIREDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ServerWeaponLaserDesignatorMessage {
}

pub trait ServerWeaponLaserDesignatorMessageTrait: TypeObject {
}

impl ServerWeaponLaserDesignatorMessageTrait for ServerWeaponLaserDesignatorMessage {
}

pub static SERVERWEAPONLASERDESIGNATORMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponLaserDesignatorMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerWeaponLaserDesignatorMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerWeaponLaserDesignatorMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERWEAPONLASERDESIGNATORMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ServerWeaponMortarStrikeMessage {
}

pub trait ServerWeaponMortarStrikeMessageTrait: TypeObject {
}

impl ServerWeaponMortarStrikeMessageTrait for ServerWeaponMortarStrikeMessage {
}

pub static SERVERWEAPONMORTARSTRIKEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponMortarStrikeMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerWeaponMortarStrikeMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ServerWeaponMortarStrikeMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERWEAPONMORTARSTRIKEMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientWeaponReplacedMessage {
}

pub trait ClientWeaponReplacedMessageTrait: TypeObject {
}

impl ClientWeaponReplacedMessageTrait for ClientWeaponReplacedMessage {
}

pub static CLIENTWEAPONREPLACEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponReplacedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientWeaponReplacedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientWeaponReplacedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTWEAPONREPLACEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientWeaponWeaponComponentActivateMessage {
}

pub trait ClientWeaponWeaponComponentActivateMessageTrait: TypeObject {
}

impl ClientWeaponWeaponComponentActivateMessageTrait for ClientWeaponWeaponComponentActivateMessage {
}

pub static CLIENTWEAPONWEAPONCOMPONENTACTIVATEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponWeaponComponentActivateMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientWeaponWeaponComponentActivateMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientWeaponWeaponComponentActivateMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTWEAPONWEAPONCOMPONENTACTIVATEMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientWeaponPlayerPrimaryOutOfAmmoMessage {
}

pub trait ClientWeaponPlayerPrimaryOutOfAmmoMessageTrait: TypeObject {
}

impl ClientWeaponPlayerPrimaryOutOfAmmoMessageTrait for ClientWeaponPlayerPrimaryOutOfAmmoMessage {
}

pub static CLIENTWEAPONPLAYERPRIMARYOUTOFAMMOMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponPlayerPrimaryOutOfAmmoMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientWeaponPlayerPrimaryOutOfAmmoMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientWeaponPlayerPrimaryOutOfAmmoMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTWEAPONPLAYERPRIMARYOUTOFAMMOMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientWeaponPlayerWeaponReloadEndMessage {
}

pub trait ClientWeaponPlayerWeaponReloadEndMessageTrait: TypeObject {
}

impl ClientWeaponPlayerWeaponReloadEndMessageTrait for ClientWeaponPlayerWeaponReloadEndMessage {
}

pub static CLIENTWEAPONPLAYERWEAPONRELOADENDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponPlayerWeaponReloadEndMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientWeaponPlayerWeaponReloadEndMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientWeaponPlayerWeaponReloadEndMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTWEAPONPLAYERWEAPONRELOADENDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientWeaponPlayerWeaponReloadBeginMessage {
}

pub trait ClientWeaponPlayerWeaponReloadBeginMessageTrait: TypeObject {
}

impl ClientWeaponPlayerWeaponReloadBeginMessageTrait for ClientWeaponPlayerWeaponReloadBeginMessage {
}

pub static CLIENTWEAPONPLAYERWEAPONRELOADBEGINMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponPlayerWeaponReloadBeginMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientWeaponPlayerWeaponReloadBeginMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientWeaponPlayerWeaponReloadBeginMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTWEAPONPLAYERWEAPONRELOADBEGINMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientWeaponPlayerPrimaryWeaponFireMessage {
}

pub trait ClientWeaponPlayerPrimaryWeaponFireMessageTrait: TypeObject {
}

impl ClientWeaponPlayerPrimaryWeaponFireMessageTrait for ClientWeaponPlayerPrimaryWeaponFireMessage {
}

pub static CLIENTWEAPONPLAYERPRIMARYWEAPONFIREMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponPlayerPrimaryWeaponFireMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientWeaponPlayerPrimaryWeaponFireMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientWeaponPlayerPrimaryWeaponFireMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTWEAPONPLAYERPRIMARYWEAPONFIREMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientWeaponPlayerWeaponChangeCorrectionMessage {
}

pub trait ClientWeaponPlayerWeaponChangeCorrectionMessageTrait: TypeObject {
}

impl ClientWeaponPlayerWeaponChangeCorrectionMessageTrait for ClientWeaponPlayerWeaponChangeCorrectionMessage {
}

pub static CLIENTWEAPONPLAYERWEAPONCHANGECORRECTIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponPlayerWeaponChangeCorrectionMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientWeaponPlayerWeaponChangeCorrectionMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientWeaponPlayerWeaponChangeCorrectionMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTWEAPONPLAYERWEAPONCHANGECORRECTIONMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientWeaponPlayerWeaponChangeMessage {
}

pub trait ClientWeaponPlayerWeaponChangeMessageTrait: TypeObject {
}

impl ClientWeaponPlayerWeaponChangeMessageTrait for ClientWeaponPlayerWeaponChangeMessage {
}

pub static CLIENTWEAPONPLAYERWEAPONCHANGEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponPlayerWeaponChangeMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientWeaponPlayerWeaponChangeMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientWeaponPlayerWeaponChangeMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTWEAPONPLAYERWEAPONCHANGEMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ServerWeapon {
    pub _glacier_base: Weapon,
}

pub trait ServerWeaponTrait: WeaponTrait {
}

impl ServerWeaponTrait for ServerWeapon {
}

impl WeaponTrait for ServerWeapon {
}

impl super::game_common::ToolTrait for ServerWeapon {
}

pub static SERVERWEAPON_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeapon",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPON_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerWeapon as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERWEAPON_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerWeapon {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERWEAPON_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERWEAPON_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeapon-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("ServerWeapon"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerProjectileEntity {
    pub _glacier_base: super::game_server::ServerPhysicsEntity,
}

pub trait ServerProjectileEntityTrait: super::game_server::ServerPhysicsEntityTrait {
}

impl ServerProjectileEntityTrait for ServerProjectileEntity {
}

impl super::game_server::ServerPhysicsEntityTrait for ServerProjectileEntity {
}

impl super::game_server::ServerGameComponentEntityTrait for ServerProjectileEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ServerProjectileEntity {
}

impl super::entity::ComponentEntityTrait for ServerProjectileEntity {
}

impl super::entity::SpatialEntityTrait for ServerProjectileEntity {
}

impl super::entity::EntityTrait for ServerProjectileEntity {
}

impl super::entity::EntityBusPeerTrait for ServerProjectileEntity {
}

pub static SERVERPROJECTILEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerProjectileEntity",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_server::SERVERPHYSICSENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerProjectileEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERPROJECTILEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerProjectileEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPROJECTILEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERPROJECTILEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerProjectileEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("ServerProjectileEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerMissileEntity {
    pub _glacier_base: ServerGhostProjectileEntity,
}

pub trait ServerMissileEntityTrait: ServerGhostProjectileEntityTrait {
}

impl ServerMissileEntityTrait for ServerMissileEntity {
}

impl ServerGhostProjectileEntityTrait for ServerMissileEntity {
}

impl ServerProjectileEntityTrait for ServerMissileEntity {
}

impl super::game_server::ServerPhysicsEntityTrait for ServerMissileEntity {
}

impl super::game_server::ServerGameComponentEntityTrait for ServerMissileEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ServerMissileEntity {
}

impl super::entity::ComponentEntityTrait for ServerMissileEntity {
}

impl super::entity::SpatialEntityTrait for ServerMissileEntity {
}

impl super::entity::EntityTrait for ServerMissileEntity {
}

impl super::entity::EntityBusPeerTrait for ServerMissileEntity {
}

pub static SERVERMISSILEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMissileEntity",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGHOSTPROJECTILEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerMissileEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERMISSILEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerMissileEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERMISSILEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERMISSILEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMissileEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("ServerMissileEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerGhostProjectileEntity {
    pub _glacier_base: ServerProjectileEntity,
}

pub trait ServerGhostProjectileEntityTrait: ServerProjectileEntityTrait {
}

impl ServerGhostProjectileEntityTrait for ServerGhostProjectileEntity {
}

impl ServerProjectileEntityTrait for ServerGhostProjectileEntity {
}

impl super::game_server::ServerPhysicsEntityTrait for ServerGhostProjectileEntity {
}

impl super::game_server::ServerGameComponentEntityTrait for ServerGhostProjectileEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ServerGhostProjectileEntity {
}

impl super::entity::ComponentEntityTrait for ServerGhostProjectileEntity {
}

impl super::entity::SpatialEntityTrait for ServerGhostProjectileEntity {
}

impl super::entity::EntityTrait for ServerGhostProjectileEntity {
}

impl super::entity::EntityBusPeerTrait for ServerGhostProjectileEntity {
}

pub static SERVERGHOSTPROJECTILEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGhostProjectileEntity",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERPROJECTILEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGhostProjectileEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERGHOSTPROJECTILEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerGhostProjectileEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGHOSTPROJECTILEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERGHOSTPROJECTILEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGhostProjectileEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("ServerGhostProjectileEntity"),
    array_type: None,
    alignment: 8,
};


