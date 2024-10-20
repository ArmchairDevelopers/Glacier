use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientWeapon {
}

pub const CLIENTWEAPON_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeapon",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPON_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWEAPON_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWeapon {
    fn type_info() -> &'static TypeInfo {
        CLIENTWEAPON_TYPE_INFO
    }
}


pub const CLIENTWEAPON_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeapon-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("ClientWeapon-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientProxyProjectileEntity {
}

pub const CLIENTPROXYPROJECTILEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientProxyProjectileEntity",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTPROJECTILEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPROXYPROJECTILEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientProxyProjectileEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTPROXYPROJECTILEENTITY_TYPE_INFO
    }
}


pub const CLIENTPROXYPROJECTILEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientProxyProjectileEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("ClientProxyProjectileEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientProxyMissileEntity {
}

pub const CLIENTPROXYMISSILEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientProxyMissileEntity",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTPROXYPROJECTILEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPROXYMISSILEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientProxyMissileEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTPROXYMISSILEENTITY_TYPE_INFO
    }
}


pub const CLIENTPROXYMISSILEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientProxyMissileEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("ClientProxyMissileEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientProjectileEntity {
}

pub const CLIENTPROJECTILEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientProjectileEntity",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTPHYSICSENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPROJECTILEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientProjectileEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTPROJECTILEENTITY_TYPE_INFO
    }
}


pub const CLIENTPROJECTILEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientProjectileEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("ClientProjectileEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientMissileEntity {
}

pub const CLIENTMISSILEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMissileEntity",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGHOSTPROJECTILEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTMISSILEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientMissileEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTMISSILEENTITY_TYPE_INFO
    }
}


pub const CLIENTMISSILEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMissileEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("ClientMissileEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientGhostProjectileEntity {
}

pub const CLIENTGHOSTPROJECTILEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGhostProjectileEntity",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTPROJECTILEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTGHOSTPROJECTILEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientGhostProjectileEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTGHOSTPROJECTILEENTITY_TYPE_INFO
    }
}


pub const CLIENTGHOSTPROJECTILEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGhostProjectileEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("ClientGhostProjectileEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientBulletEntity {
}

pub const CLIENTBULLETENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBulletEntity",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTPROJECTILEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTBULLETENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientBulletEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTBULLETENTITY_TYPE_INFO
    }
}


pub const CLIENTBULLETENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBulletEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("ClientBulletEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientWeaponInputRouterComponent {
}

pub const CLIENTWEAPONINPUTROUTERCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponInputRouterComponent",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWEAPONINPUTROUTERCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWeaponInputRouterComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTWEAPONINPUTROUTERCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTWEAPONINPUTROUTERCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponInputRouterComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("ClientWeaponInputRouterComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientWeaponComponent {
}

pub const CLIENTWEAPONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponComponent",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWEAPONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWeaponComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTWEAPONCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTWEAPONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("ClientWeaponComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientMissilePhysicsComponent {
}

pub const CLIENTMISSILEPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMissilePhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PARTPHYSICSCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTMISSILEPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientMissilePhysicsComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTMISSILEPHYSICSCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTMISSILEPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMissilePhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("ClientMissilePhysicsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCharacterSimpleWeaponComponent {
}

pub const CLIENTCHARACTERSIMPLEWEAPONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterSimpleWeaponComponent",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCHARACTERSIMPLEWEAPONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCharacterSimpleWeaponComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTCHARACTERSIMPLEWEAPONCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTCHARACTERSIMPLEWEAPONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCharacterSimpleWeaponComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("ClientCharacterSimpleWeaponComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerBulletEntity {
}

pub const SERVERBULLETENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBulletEntity",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERPROJECTILEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERBULLETENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerBulletEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERBULLETENTITY_TYPE_INFO
    }
}


pub const SERVERBULLETENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBulletEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("ServerBulletEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerWeaponInputRouterComponent {
}

pub const SERVERWEAPONINPUTROUTERCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponInputRouterComponent",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERWEAPONINPUTROUTERCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerWeaponInputRouterComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERWEAPONINPUTROUTERCOMPONENT_TYPE_INFO
    }
}


pub const SERVERWEAPONINPUTROUTERCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponInputRouterComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("ServerWeaponInputRouterComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerWeaponComponent {
}

pub const SERVERWEAPONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponComponent",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERWEAPONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerWeaponComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERWEAPONCOMPONENT_TYPE_INFO
    }
}


pub const SERVERWEAPONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("ServerWeaponComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerMissilePhysicsComponent {
}

pub const SERVERMISSILEPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMissilePhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PARTPHYSICSCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERMISSILEPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerMissilePhysicsComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERMISSILEPHYSICSCOMPONENT_TYPE_INFO
    }
}


pub const SERVERMISSILEPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMissilePhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("ServerMissilePhysicsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCharacterSimpleWeaponComponent {
}

pub const SERVERCHARACTERSIMPLEWEAPONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterSimpleWeaponComponent",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERCHARACTERSIMPLEWEAPONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCharacterSimpleWeaponComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERCHARACTERSIMPLEWEAPONCOMPONENT_TYPE_INFO
    }
}


pub const SERVERCHARACTERSIMPLEWEAPONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterSimpleWeaponComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("ServerCharacterSimpleWeaponComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Weapon {
}

pub const WEAPON_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Weapon",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TOOL_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WEAPON_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Weapon {
    fn type_info() -> &'static TypeInfo {
        WEAPON_TYPE_INFO
    }
}


pub const WEAPON_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Weapon-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("Weapon-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerWeaponOverrideAIDataMessage {
}

pub const SERVERWEAPONOVERRIDEAIDATAMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponOverrideAIDataMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerWeaponOverrideAIDataMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERWEAPONOVERRIDEAIDATAMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerWeaponProjectileTimeoutMessage {
}

pub const SERVERWEAPONPROJECTILETIMEOUTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponProjectileTimeoutMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ServerWeaponProjectileTimeoutMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERWEAPONPROJECTILETIMEOUTMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerWeaponReplacedMessage {
}

pub const SERVERWEAPONREPLACEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponReplacedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerWeaponReplacedMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERWEAPONREPLACEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerWeaponWeaponComponentActivateMessage {
}

pub const SERVERWEAPONWEAPONCOMPONENTACTIVATEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponWeaponComponentActivateMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerWeaponWeaponComponentActivateMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERWEAPONWEAPONCOMPONENTACTIVATEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerWeaponWeaponComponentReloadMessage {
}

pub const SERVERWEAPONWEAPONCOMPONENTRELOADMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponWeaponComponentReloadMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerWeaponWeaponComponentReloadMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERWEAPONWEAPONCOMPONENTRELOADMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerWeaponPlayerResupplyMessage {
}

pub const SERVERWEAPONPLAYERRESUPPLYMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponPlayerResupplyMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerWeaponPlayerResupplyMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERWEAPONPLAYERRESUPPLYMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerWeaponPlayerReloadMessage {
}

pub const SERVERWEAPONPLAYERRELOADMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponPlayerReloadMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ServerWeaponPlayerReloadMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERWEAPONPLAYERRELOADMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerWeaponPlayerWeaponRemovedMessage {
}

pub const SERVERWEAPONPLAYERWEAPONREMOVEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponPlayerWeaponRemovedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerWeaponPlayerWeaponRemovedMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERWEAPONPLAYERWEAPONREMOVEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerWeaponPlayerWeaponReloadEndMessage {
}

pub const SERVERWEAPONPLAYERWEAPONRELOADENDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponPlayerWeaponReloadEndMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerWeaponPlayerWeaponReloadEndMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERWEAPONPLAYERWEAPONRELOADENDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerWeaponPlayerPrimaryFireShotSpawnedMessage {
}

pub const SERVERWEAPONPLAYERPRIMARYFIRESHOTSPAWNEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponPlayerPrimaryFireShotSpawnedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerWeaponPlayerPrimaryFireShotSpawnedMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERWEAPONPLAYERPRIMARYFIRESHOTSPAWNEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerWeaponPlayerPrimaryOutOfAmmoMessage {
}

pub const SERVERWEAPONPLAYERPRIMARYOUTOFAMMOMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponPlayerPrimaryOutOfAmmoMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerWeaponPlayerPrimaryOutOfAmmoMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERWEAPONPLAYERPRIMARYOUTOFAMMOMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerWeaponArtilleryFiredMessage {
}

pub const SERVERWEAPONARTILLERYFIREDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponArtilleryFiredMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ServerWeaponArtilleryFiredMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERWEAPONARTILLERYFIREDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerWeaponLaserDesignatorMessage {
}

pub const SERVERWEAPONLASERDESIGNATORMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponLaserDesignatorMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerWeaponLaserDesignatorMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERWEAPONLASERDESIGNATORMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerWeaponMortarStrikeMessage {
}

pub const SERVERWEAPONMORTARSTRIKEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponMortarStrikeMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ServerWeaponMortarStrikeMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERWEAPONMORTARSTRIKEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientWeaponReplacedMessage {
}

pub const CLIENTWEAPONREPLACEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponReplacedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientWeaponReplacedMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTWEAPONREPLACEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientWeaponWeaponComponentActivateMessage {
}

pub const CLIENTWEAPONWEAPONCOMPONENTACTIVATEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponWeaponComponentActivateMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientWeaponWeaponComponentActivateMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTWEAPONWEAPONCOMPONENTACTIVATEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientWeaponPlayerPrimaryOutOfAmmoMessage {
}

pub const CLIENTWEAPONPLAYERPRIMARYOUTOFAMMOMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponPlayerPrimaryOutOfAmmoMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientWeaponPlayerPrimaryOutOfAmmoMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTWEAPONPLAYERPRIMARYOUTOFAMMOMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientWeaponPlayerWeaponReloadEndMessage {
}

pub const CLIENTWEAPONPLAYERWEAPONRELOADENDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponPlayerWeaponReloadEndMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientWeaponPlayerWeaponReloadEndMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTWEAPONPLAYERWEAPONRELOADENDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientWeaponPlayerWeaponReloadBeginMessage {
}

pub const CLIENTWEAPONPLAYERWEAPONRELOADBEGINMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponPlayerWeaponReloadBeginMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientWeaponPlayerWeaponReloadBeginMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTWEAPONPLAYERWEAPONRELOADBEGINMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientWeaponPlayerPrimaryWeaponFireMessage {
}

pub const CLIENTWEAPONPLAYERPRIMARYWEAPONFIREMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponPlayerPrimaryWeaponFireMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientWeaponPlayerPrimaryWeaponFireMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTWEAPONPLAYERPRIMARYWEAPONFIREMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientWeaponPlayerWeaponChangeCorrectionMessage {
}

pub const CLIENTWEAPONPLAYERWEAPONCHANGECORRECTIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponPlayerWeaponChangeCorrectionMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientWeaponPlayerWeaponChangeCorrectionMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTWEAPONPLAYERWEAPONCHANGECORRECTIONMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientWeaponPlayerWeaponChangeMessage {
}

pub const CLIENTWEAPONPLAYERWEAPONCHANGEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponPlayerWeaponChangeMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Weapon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientWeaponPlayerWeaponChangeMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTWEAPONPLAYERWEAPONCHANGEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerWeapon {
}

pub const SERVERWEAPON_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeapon",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPON_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERWEAPON_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerWeapon {
    fn type_info() -> &'static TypeInfo {
        SERVERWEAPON_TYPE_INFO
    }
}


pub const SERVERWEAPON_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeapon-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("ServerWeapon-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerProjectileEntity {
}

pub const SERVERPROJECTILEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerProjectileEntity",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERPHYSICSENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERPROJECTILEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerProjectileEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERPROJECTILEENTITY_TYPE_INFO
    }
}


pub const SERVERPROJECTILEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerProjectileEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("ServerProjectileEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerMissileEntity {
}

pub const SERVERMISSILEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMissileEntity",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGHOSTPROJECTILEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERMISSILEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerMissileEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERMISSILEENTITY_TYPE_INFO
    }
}


pub const SERVERMISSILEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMissileEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("ServerMissileEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGhostProjectileEntity {
}

pub const SERVERGHOSTPROJECTILEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGhostProjectileEntity",
    flags: MemberInfoFlags::new(101),
    module: "Weapon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERPROJECTILEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERGHOSTPROJECTILEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerGhostProjectileEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERGHOSTPROJECTILEENTITY_TYPE_INFO
    }
}


pub const SERVERGHOSTPROJECTILEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGhostProjectileEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Weapon",
    data: TypeInfoData::Array("ServerGhostProjectileEntity-Array"),
    array_type: None,
    alignment: 8,
};


