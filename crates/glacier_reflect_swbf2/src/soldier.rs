use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_soldier_types(registry: &mut TypeRegistry) {
    registry.register_type(CLIENTWEAPONSTATEENTITY_TYPE_INFO);
    registry.register_type(CLIENTWEAPONSTATEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPLAYERLOOKATENTITY_TYPE_INFO);
    registry.register_type(CLIENTPLAYERLOOKATENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTTRIPWIREENTITY_TYPE_INFO);
    registry.register_type(CLIENTTRIPWIREENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTTRIGGERMOVEENTITY_TYPE_INFO);
    registry.register_type(CLIENTTRIGGERMOVEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSOLDIERBREATHCONTROLCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTSOLDIERBREATHCONTROLCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSOLDIERBODYCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTSOLDIERBODYCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPICKUPPHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTPICKUPPHYSICSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPHANTOMCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTPHANTOMCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTMOVEMENTCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTMOVEMENTCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTHITREACTIONCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTHITREACTIONCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTFACEPOSERCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTFACEPOSERCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTBONECOLLISIONCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTBONECOLLISIONCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTBLOCKAIMASSISTCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTBLOCKAIMASSISTCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTENTRYAIMASSISTTARGETOPTIONSCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTENTRYAIMASSISTTARGETOPTIONSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTAIMASSISTNODESNAPPOINTCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTAIMASSISTNODESNAPPOINTCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTAIMASSISTNODECOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTAIMASSISTNODECOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SOLDIERCAMERA_TYPE_INFO);
    registry.register_type(SOLDIERCAMERA_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTWEAPONLAGENTITY_TYPE_INFO);
    registry.register_type(CLIENTWEAPONLAGENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCOVERPEEKENTITY_TYPE_INFO);
    registry.register_type(CLIENTCOVERPEEKENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTOCCLUTIONQUERYCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTOCCLUTIONQUERYCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTVOICEOVEROBJECTREADERWATCHERENTITY_TYPE_INFO);
    registry.register_type(CLIENTVOICEOVEROBJECTREADERWATCHERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTVOICEOVEROBJECTREADERENTITY_TYPE_INFO);
    registry.register_type(CLIENTVOICEOVEROBJECTREADERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSOLDIERWEAPONSCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERSOLDIERWEAPONSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSOLDIERPHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERSOLDIERPHYSICSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSOLDIERHEALTHCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERSOLDIERHEALTHCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSOLDIERCUSTOMIZATIONCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERSOLDIERCUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSOLDIERCAMERACOMPONENT_TYPE_INFO);
    registry.register_type(SERVERSOLDIERCAMERACOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSOLDIERBODYCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERSOLDIERBODYCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERPICKUPPHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERPICKUPPHYSICSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERMOVEMENTCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERMOVEMENTCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERHITREACTIONCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERHITREACTIONCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERBONECOLLISIONCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERBONECOLLISIONCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTAIMINGSCALEDATAPROVIDERENTITY_TYPE_INFO);
    registry.register_type(CLIENTAIMINGSCALEDATAPROVIDERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTAIMINGANGULARSPEEDCONSTRAINTDATAPROVIDERENTITY_TYPE_INFO);
    registry.register_type(CLIENTAIMINGANGULARSPEEDCONSTRAINTDATAPROVIDERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTZEROINGWEAPON_TYPE_INFO);
    registry.register_type(CLIENTZEROINGWEAPON_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSOLDIERWEAPONSOCKETENTITY_TYPE_INFO);
    registry.register_type(CLIENTSOLDIERWEAPONSOCKETENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSOLDIERWEAPONSPAWNINFO_TYPE_INFO);
    registry.register_type(CLIENTSOLDIERWEAPONSPAWNINFO_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSOLDIERWEAPON_TYPE_INFO);
    registry.register_type(CLIENTSOLDIERWEAPON_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTANIMATIONTURRETROTATIONCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTANIMATIONTURRETROTATIONCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPROXYGRENADEENTITY_TYPE_INFO);
    registry.register_type(CLIENTPROXYGRENADEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPROXYEXPLOSIONPACKENTITY_TYPE_INFO);
    registry.register_type(CLIENTPROXYEXPLOSIONPACKENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTGRENADEENTITY_TYPE_INFO);
    registry.register_type(CLIENTGRENADEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTEXPLOSIONPACKENTITY_TYPE_INFO);
    registry.register_type(CLIENTEXPLOSIONPACKENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTEXPLOSIONPACKPHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTEXPLOSIONPACKPHYSICSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(DUMMYEXPLOSIONPACKPHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(DUMMYEXPLOSIONPACKPHYSICSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSOLDIERENTRYCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTSOLDIERENTRYCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTWEAPONZEROINGCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTWEAPONZEROINGCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSOLDIERWEAPONSPREVIEWCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTSOLDIERWEAPONSPREVIEWCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSOLDIERWEAPONSCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTSOLDIERWEAPONSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSOLDIERPHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTSOLDIERPHYSICSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSOLDIERHEALTHCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTSOLDIERHEALTHCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSOLDIERFOOTPLANTEFFECTCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTSOLDIERFOOTPLANTEFFECTCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSOLDIERCUSTOMIZATIONCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTSOLDIERCUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSOLDIERCAMERACOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTSOLDIERCAMERACOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSOLDIERCAMERACALLBACK_TYPE_INFO);
    registry.register_type(CLIENTSOLDIERCAMERACALLBACK_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSOLDIERENTITY_TYPE_INFO);
    registry.register_type(CLIENTSOLDIERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSCHEMATICSAIMENTITY_TYPE_INFO);
    registry.register_type(CLIENTSCHEMATICSAIMENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTREPLICATEDAIMENTITY_TYPE_INFO);
    registry.register_type(CLIENTREPLICATEDAIMENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPICKUPENTITY_TYPE_INFO);
    registry.register_type(CLIENTPICKUPENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTBLOCKAIMASSISTENTITY_TYPE_INFO);
    registry.register_type(CLIENTBLOCKAIMASSISTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTAIMINGSIMULATIONDATAPROVIDERENTITY_TYPE_INFO);
    registry.register_type(CLIENTAIMINGSIMULATIONDATAPROVIDERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTAIMENTITYBASE_TYPE_INFO);
    registry.register_type(CLIENTAIMENTITYBASE_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTAIMENTITY_TYPE_INFO);
    registry.register_type(CLIENTAIMENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCUSTOMIZESOLDIERENTITY_TYPE_INFO);
    registry.register_type(SERVERCUSTOMIZESOLDIERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERTRIPWIREENTITY_TYPE_INFO);
    registry.register_type(SERVERTRIPWIREENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSOLDIERENTITY_TYPE_INFO);
    registry.register_type(SERVERSOLDIERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERREPLICATEDAIMENTITY_TYPE_INFO);
    registry.register_type(SERVERREPLICATEDAIMENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERPICKUPENTITY_TYPE_INFO);
    registry.register_type(SERVERPICKUPENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERZEROINGWEAPON_TYPE_INFO);
    registry.register_type(SERVERZEROINGWEAPON_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSOLDIERWEAPONSPAWNINFO_TYPE_INFO);
    registry.register_type(SERVERSOLDIERWEAPONSPAWNINFO_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSOLDIERWEAPON_TYPE_INFO);
    registry.register_type(SERVERSOLDIERWEAPON_ARRAY_TYPE_INFO);
    registry.register_type(SERVERANIMATIONTURRETROTATIONCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERANIMATIONTURRETROTATIONCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERGRENADEENTITY_TYPE_INFO);
    registry.register_type(SERVERGRENADEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVEREXPLOSIONPACKENTITY_TYPE_INFO);
    registry.register_type(SERVEREXPLOSIONPACKENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERMISSILEHEALTHCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERMISSILEHEALTHCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVEREXPLOSIONPACKPHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(SERVEREXPLOSIONPACKPHYSICSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVEREXPLOSIONPACKHEALTHCOMPONENT_TYPE_INFO);
    registry.register_type(SERVEREXPLOSIONPACKHEALTHCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSOLDIERENTRYCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERSOLDIERENTRYCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERWEAPONZEROINGCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERWEAPONZEROINGCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSOLDIERINTERACTIONENTITYINRANGECHANGEDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERSOLDIERSOLDIERVSSOLDIERCOLLISIONMESSAGE_TYPE_INFO);
    registry.register_type(SERVERSOLDIERPICKEDUPSUPPLYSPHEREMESSAGE_TYPE_INFO);
    registry.register_type(SERVERSOLDIERONUNSPAWNMESSAGE_TYPE_INFO);
    registry.register_type(SERVERSOLDIERTHROWDISTRACTIONMESSAGE_TYPE_INFO);
    registry.register_type(SERVERSOLDIERSELFHEALMESSAGE_TYPE_INFO);
    registry.register_type(SERVERSOLDIERMANDOWNMESSAGE_TYPE_INFO);
    registry.register_type(SERVERSOLDIERONINITMESSAGE_TYPE_INFO);
    registry.register_type(SERVERSOLDIERCHANGINGWEAPONMESSAGE_TYPE_INFO);
    registry.register_type(SERVERSOLDIERFIRINGMESSAGE_TYPE_INFO);
    registry.register_type(SERVERSOLDIERDAMAGEDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERPROJECTILEMISSILEDESTROYEDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERPROJECTILEMISSILEDAMAGEDMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTSOLDIERINTERACTIONENTITYINRANGECHANGEDMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTSOLDIERCHANGECOVERSTATEMESSAGE_TYPE_INFO);
    registry.register_type(AIMINGHANDLE_TYPE_INFO);
    registry.register_type(AIMINGHANDLE_ARRAY_TYPE_INFO);
    registry.register_type(AIMINGRENDERRETURNEDVALUES_TYPE_INFO);
    registry.register_type(AIMINGRENDERRETURNEDVALUES_ARRAY_TYPE_INFO);
    registry.register_type(AIMINGRENDERUPDATECONTEXT_TYPE_INFO);
    registry.register_type(AIMINGRENDERUPDATECONTEXT_ARRAY_TYPE_INFO);
    registry.register_type(AIMINGSIMUPDATECONTEXT_TYPE_INFO);
    registry.register_type(AIMINGSIMUPDATECONTEXT_ARRAY_TYPE_INFO);
    registry.register_type(AIMINGSIMDYNAMICSTATE_TYPE_INFO);
    registry.register_type(AIMINGSIMDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(AIMINGSIMSTATICSTATE_TYPE_INFO);
    registry.register_type(AIMINGSIMSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(AIMINGCONSTRAINTS_TYPE_INFO);
    registry.register_type(AIMINGCONSTRAINTS_ARRAY_TYPE_INFO);
    registry.register_type(AIMINGENVIRONMENTTARGET_TYPE_INFO);
    registry.register_type(AIMINGENVIRONMENTTARGET_ARRAY_TYPE_INFO);
    registry.register_type(AIMOVERRIDEMODE_TYPE_INFO);
    registry.register_type(AIMOVERRIDEMODE_ARRAY_TYPE_INFO);
    registry.register_type(SOLDIERTHIRDPERSONCAMERARENDERSTATE_TYPE_INFO);
    registry.register_type(SOLDIERTHIRDPERSONCAMERARENDERSTATE_ARRAY_TYPE_INFO);
    registry.register_type(SOLDIERTHIRDPERSONCAMERASIMSTATE_TYPE_INFO);
    registry.register_type(SOLDIERTHIRDPERSONCAMERASIMSTATE_ARRAY_TYPE_INFO);
    registry.register_type(SOLDIERFIRSTPERSONCAMERARENDERSTATE_TYPE_INFO);
    registry.register_type(SOLDIERFIRSTPERSONCAMERARENDERSTATE_ARRAY_TYPE_INFO);
    registry.register_type(SOLDIERFIRSTPERSONCAMERASIMSTATE_TYPE_INFO);
    registry.register_type(SOLDIERFIRSTPERSONCAMERASIMSTATE_ARRAY_TYPE_INFO);
    registry.register_type(SOLDIERSERVERPLAYEREXTENT_TYPE_INFO);
    registry.register_type(SOLDIERSERVERPLAYEREXTENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERLOOKATTRIGGERENTITY_TYPE_INFO);
    registry.register_type(SERVERLOOKATTRIGGERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCHARACTERSTATETRIGGERENTITY_TYPE_INFO);
    registry.register_type(SERVERCHARACTERSTATETRIGGERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERWEAPONSTATEENTITY_TYPE_INFO);
    registry.register_type(SERVERWEAPONSTATEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSTATEEVENTGATEENTITY_TYPE_INFO);
    registry.register_type(SERVERSTATEEVENTGATEENTITY_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientWeaponStateEntity {
}

pub const CLIENTWEAPONSTATEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponStateEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWEAPONSTATEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWeaponStateEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTWEAPONSTATEENTITY_TYPE_INFO
    }
}


pub const CLIENTWEAPONSTATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponStateEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientWeaponStateEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPlayerLookAtEntity {
}

pub const CLIENTPLAYERLOOKATENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerLookAtEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPLAYERLOOKATENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPlayerLookAtEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTPLAYERLOOKATENTITY_TYPE_INFO
    }
}


pub const CLIENTPLAYERLOOKATENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerLookAtEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientPlayerLookAtEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientTripwireEntity {
}

pub const CLIENTTRIPWIREENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTripwireEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENTENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTTRIPWIREENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientTripwireEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTTRIPWIREENTITY_TYPE_INFO
    }
}


pub const CLIENTTRIPWIREENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTripwireEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientTripwireEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientTriggerMoveEntity {
}

pub const CLIENTTRIGGERMOVEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTriggerMoveEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTTRIGGERMOVEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientTriggerMoveEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTTRIGGERMOVEENTITY_TYPE_INFO
    }
}


pub const CLIENTTRIGGERMOVEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTriggerMoveEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientTriggerMoveEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientSoldierBreathControlComponent {
}

pub const CLIENTSOLDIERBREATHCONTROLCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierBreathControlComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSOLDIERBREATHCONTROLCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSoldierBreathControlComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTSOLDIERBREATHCONTROLCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTSOLDIERBREATHCONTROLCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierBreathControlComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierBreathControlComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientSoldierBodyComponent {
}

pub const CLIENTSOLDIERBODYCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierBodyComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSOLDIERBODYCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSoldierBodyComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTSOLDIERBODYCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTSOLDIERBODYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierBodyComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierBodyComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPickupPhysicsComponent {
}

pub const CLIENTPICKUPPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPickupPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPICKUPPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPickupPhysicsComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTPICKUPPHYSICSCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTPICKUPPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPickupPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientPickupPhysicsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPhantomComponent {
}

pub const CLIENTPHANTOMCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPhantomComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPHANTOMCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPhantomComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTPHANTOMCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTPHANTOMCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPhantomComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientPhantomComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientMovementComponent {
}

pub const CLIENTMOVEMENTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMovementComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTMOVEMENTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientMovementComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTMOVEMENTCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTMOVEMENTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMovementComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientMovementComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientHitReactionComponent {
}

pub const CLIENTHITREACTIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientHitReactionComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTHITREACTIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientHitReactionComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTHITREACTIONCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTHITREACTIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientHitReactionComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientHitReactionComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientFaceposerComponent {
}

pub const CLIENTFACEPOSERCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientFaceposerComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTFACEPOSERCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientFaceposerComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTFACEPOSERCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTFACEPOSERCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientFaceposerComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientFaceposerComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientBoneCollisionComponent {
}

pub const CLIENTBONECOLLISIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBoneCollisionComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTBONECOLLISIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientBoneCollisionComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTBONECOLLISIONCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTBONECOLLISIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBoneCollisionComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientBoneCollisionComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientBlockAimAssistComponent {
}

pub const CLIENTBLOCKAIMASSISTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBlockAimAssistComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTBLOCKAIMASSISTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientBlockAimAssistComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTBLOCKAIMASSISTCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTBLOCKAIMASSISTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBlockAimAssistComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientBlockAimAssistComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientEntryAimAssistTargetOptionsComponent {
}

pub const CLIENTENTRYAIMASSISTTARGETOPTIONSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEntryAimAssistTargetOptionsComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTENTRYAIMASSISTTARGETOPTIONSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientEntryAimAssistTargetOptionsComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTENTRYAIMASSISTTARGETOPTIONSCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTENTRYAIMASSISTTARGETOPTIONSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEntryAimAssistTargetOptionsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientEntryAimAssistTargetOptionsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientAimAssistNodeSnapPointComponent {
}

pub const CLIENTAIMASSISTNODESNAPPOINTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAimAssistNodeSnapPointComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTAIMASSISTNODESNAPPOINTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAimAssistNodeSnapPointComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTAIMASSISTNODESNAPPOINTCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTAIMASSISTNODESNAPPOINTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAimAssistNodeSnapPointComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientAimAssistNodeSnapPointComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientAimAssistNodeComponent {
}

pub const CLIENTAIMASSISTNODECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAimAssistNodeComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTAIMASSISTNODECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAimAssistNodeComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTAIMASSISTNODECOMPONENT_TYPE_INFO
    }
}


pub const CLIENTAIMASSISTNODECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAimAssistNodeComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientAimAssistNodeComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SoldierCamera {
}

pub const SOLDIERCAMERA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoldierCamera",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CAMERA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SOLDIERCAMERA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SoldierCamera {
    fn type_info() -> &'static TypeInfo {
        SOLDIERCAMERA_TYPE_INFO
    }
}


pub const SOLDIERCAMERA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoldierCamera-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("SoldierCamera-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientWeaponLagEntity {
}

pub const CLIENTWEAPONLAGENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponLagEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWEAPONLAGENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWeaponLagEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTWEAPONLAGENTITY_TYPE_INFO
    }
}


pub const CLIENTWEAPONLAGENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponLagEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientWeaponLagEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCoverPeekEntity {
}

pub const CLIENTCOVERPEEKENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCoverPeekEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCOVERPEEKENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCoverPeekEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTCOVERPEEKENTITY_TYPE_INFO
    }
}


pub const CLIENTCOVERPEEKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCoverPeekEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientCoverPeekEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientOcclutionQueryComponent {
}

pub const CLIENTOCCLUTIONQUERYCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientOcclutionQueryComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTOCCLUTIONQUERYCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientOcclutionQueryComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTOCCLUTIONQUERYCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTOCCLUTIONQUERYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientOcclutionQueryComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientOcclutionQueryComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientVoiceOverObjectReaderWatcherEntity {
}

pub const CLIENTVOICEOVEROBJECTREADERWATCHERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverObjectReaderWatcherEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVOICEOVEROBJECTREADERWATCHERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientVoiceOverObjectReaderWatcherEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTVOICEOVEROBJECTREADERWATCHERENTITY_TYPE_INFO
    }
}


pub const CLIENTVOICEOVEROBJECTREADERWATCHERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverObjectReaderWatcherEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientVoiceOverObjectReaderWatcherEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientVoiceOverObjectReaderEntity {
}

pub const CLIENTVOICEOVEROBJECTREADERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverObjectReaderEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVOICEOVEROBJECTREADERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientVoiceOverObjectReaderEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTVOICEOVEROBJECTREADERENTITY_TYPE_INFO
    }
}


pub const CLIENTVOICEOVEROBJECTREADERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverObjectReaderEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientVoiceOverObjectReaderEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSoldierWeaponsComponent {
}

pub const SERVERSOLDIERWEAPONSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierWeaponsComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSOLDIERWEAPONSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSoldierWeaponsComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERSOLDIERWEAPONSCOMPONENT_TYPE_INFO
    }
}


pub const SERVERSOLDIERWEAPONSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierWeaponsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerSoldierWeaponsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSoldierPhysicsComponent {
}

pub const SERVERSOLDIERPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERCHARACTERMASTERPHYSICSCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSOLDIERPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSoldierPhysicsComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERSOLDIERPHYSICSCOMPONENT_TYPE_INFO
    }
}


pub const SERVERSOLDIERPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerSoldierPhysicsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSoldierHealthComponent {
}

pub const SERVERSOLDIERHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierHealthComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERCHARACTERHEALTHCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSOLDIERHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSoldierHealthComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERSOLDIERHEALTHCOMPONENT_TYPE_INFO
    }
}


pub const SERVERSOLDIERHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierHealthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerSoldierHealthComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSoldierCustomizationComponent {
}

pub const SERVERSOLDIERCUSTOMIZATIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierCustomizationComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERCHARACTERCUSTOMIZATIONCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSOLDIERCUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSoldierCustomizationComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERSOLDIERCUSTOMIZATIONCOMPONENT_TYPE_INFO
    }
}


pub const SERVERSOLDIERCUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierCustomizationComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerSoldierCustomizationComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSoldierCameraComponent {
}

pub const SERVERSOLDIERCAMERACOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierCameraComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSOLDIERCAMERACOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSoldierCameraComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERSOLDIERCAMERACOMPONENT_TYPE_INFO
    }
}


pub const SERVERSOLDIERCAMERACOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierCameraComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerSoldierCameraComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSoldierBodyComponent {
}

pub const SERVERSOLDIERBODYCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierBodyComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSOLDIERBODYCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSoldierBodyComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERSOLDIERBODYCOMPONENT_TYPE_INFO
    }
}


pub const SERVERSOLDIERBODYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierBodyComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerSoldierBodyComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPickupPhysicsComponent {
}

pub const SERVERPICKUPPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPickupPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERPICKUPPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPickupPhysicsComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERPICKUPPHYSICSCOMPONENT_TYPE_INFO
    }
}


pub const SERVERPICKUPPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPickupPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerPickupPhysicsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerMovementComponent {
}

pub const SERVERMOVEMENTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMovementComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERMOVEMENTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerMovementComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERMOVEMENTCOMPONENT_TYPE_INFO
    }
}


pub const SERVERMOVEMENTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMovementComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerMovementComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerHitReactionComponent {
}

pub const SERVERHITREACTIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerHitReactionComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERHITREACTIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerHitReactionComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERHITREACTIONCOMPONENT_TYPE_INFO
    }
}


pub const SERVERHITREACTIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerHitReactionComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerHitReactionComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerBoneCollisionComponent {
}

pub const SERVERBONECOLLISIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBoneCollisionComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERBONECOLLISIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerBoneCollisionComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERBONECOLLISIONCOMPONENT_TYPE_INFO
    }
}


pub const SERVERBONECOLLISIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBoneCollisionComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerBoneCollisionComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientAimingScaleDataProviderEntity {
}

pub const CLIENTAIMINGSCALEDATAPROVIDERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAimingScaleDataProviderEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTAIMINGSCALEDATAPROVIDERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAimingScaleDataProviderEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTAIMINGSCALEDATAPROVIDERENTITY_TYPE_INFO
    }
}


pub const CLIENTAIMINGSCALEDATAPROVIDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAimingScaleDataProviderEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientAimingScaleDataProviderEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientAimingAngularSpeedConstraintDataProviderEntity {
}

pub const CLIENTAIMINGANGULARSPEEDCONSTRAINTDATAPROVIDERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAimingAngularSpeedConstraintDataProviderEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTAIMINGANGULARSPEEDCONSTRAINTDATAPROVIDERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAimingAngularSpeedConstraintDataProviderEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTAIMINGANGULARSPEEDCONSTRAINTDATAPROVIDERENTITY_TYPE_INFO
    }
}


pub const CLIENTAIMINGANGULARSPEEDCONSTRAINTDATAPROVIDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAimingAngularSpeedConstraintDataProviderEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientAimingAngularSpeedConstraintDataProviderEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientZeroingWeapon {
}

pub const CLIENTZEROINGWEAPON_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientZeroingWeapon",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTWEAPON_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTZEROINGWEAPON_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientZeroingWeapon {
    fn type_info() -> &'static TypeInfo {
        CLIENTZEROINGWEAPON_TYPE_INFO
    }
}


pub const CLIENTZEROINGWEAPON_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientZeroingWeapon-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientZeroingWeapon-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientSoldierWeaponSocketEntity {
}

pub const CLIENTSOLDIERWEAPONSOCKETENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierWeaponSocketEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSOLDIERWEAPONSOCKETENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSoldierWeaponSocketEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTSOLDIERWEAPONSOCKETENTITY_TYPE_INFO
    }
}


pub const CLIENTSOLDIERWEAPONSOCKETENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierWeaponSocketEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierWeaponSocketEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientSoldierWeaponSpawnInfo {
}

pub const CLIENTSOLDIERWEAPONSPAWNINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierWeaponSpawnInfo",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSOLDIERWEAPONSPAWNINFO_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSoldierWeaponSpawnInfo {
    fn type_info() -> &'static TypeInfo {
        CLIENTSOLDIERWEAPONSPAWNINFO_TYPE_INFO
    }
}


pub const CLIENTSOLDIERWEAPONSPAWNINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierWeaponSpawnInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierWeaponSpawnInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientSoldierWeapon {
}

pub const CLIENTSOLDIERWEAPON_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierWeapon",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENTENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSOLDIERWEAPON_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSoldierWeapon {
    fn type_info() -> &'static TypeInfo {
        CLIENTSOLDIERWEAPON_TYPE_INFO
    }
}


pub const CLIENTSOLDIERWEAPON_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierWeapon-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierWeapon-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientAnimationTurretRotationComponent {
}

pub const CLIENTANIMATIONTURRETROTATIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAnimationTurretRotationComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTANIMATIONTURRETROTATIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAnimationTurretRotationComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTANIMATIONTURRETROTATIONCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTANIMATIONTURRETROTATIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAnimationTurretRotationComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientAnimationTurretRotationComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientProxyGrenadeEntity {
}

pub const CLIENTPROXYGRENADEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientProxyGrenadeEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTPROXYPROJECTILEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPROXYGRENADEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientProxyGrenadeEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTPROXYGRENADEENTITY_TYPE_INFO
    }
}


pub const CLIENTPROXYGRENADEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientProxyGrenadeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientProxyGrenadeEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientProxyExplosionPackEntity {
}

pub const CLIENTPROXYEXPLOSIONPACKENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientProxyExplosionPackEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTPROXYPROJECTILEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPROXYEXPLOSIONPACKENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientProxyExplosionPackEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTPROXYEXPLOSIONPACKENTITY_TYPE_INFO
    }
}


pub const CLIENTPROXYEXPLOSIONPACKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientProxyExplosionPackEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientProxyExplosionPackEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientGrenadeEntity {
}

pub const CLIENTGRENADEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGrenadeEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGHOSTPROJECTILEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTGRENADEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientGrenadeEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTGRENADEENTITY_TYPE_INFO
    }
}


pub const CLIENTGRENADEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGrenadeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientGrenadeEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientExplosionPackEntity {
}

pub const CLIENTEXPLOSIONPACKENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientExplosionPackEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGHOSTPROJECTILEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTEXPLOSIONPACKENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientExplosionPackEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTEXPLOSIONPACKENTITY_TYPE_INFO
    }
}


pub const CLIENTEXPLOSIONPACKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientExplosionPackEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientExplosionPackEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientExplosionPackPhysicsComponent {
}

pub const CLIENTEXPLOSIONPACKPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientExplosionPackPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PARTPHYSICSCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTEXPLOSIONPACKPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientExplosionPackPhysicsComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTEXPLOSIONPACKPHYSICSCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTEXPLOSIONPACKPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientExplosionPackPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientExplosionPackPhysicsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DummyExplosionPackPhysicsComponent {
}

pub const DUMMYEXPLOSIONPACKPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DummyExplosionPackPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DUMMYEXPLOSIONPACKPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DummyExplosionPackPhysicsComponent {
    fn type_info() -> &'static TypeInfo {
        DUMMYEXPLOSIONPACKPHYSICSCOMPONENT_TYPE_INFO
    }
}


pub const DUMMYEXPLOSIONPACKPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DummyExplosionPackPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("DummyExplosionPackPhysicsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientSoldierEntryComponent {
}

pub const CLIENTSOLDIERENTRYCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierEntryComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMEENTRYCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSOLDIERENTRYCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSoldierEntryComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTSOLDIERENTRYCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTSOLDIERENTRYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierEntryComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierEntryComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientWeaponZeroingComponent {
}

pub const CLIENTWEAPONZEROINGCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponZeroingComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWEAPONZEROINGCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWeaponZeroingComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTWEAPONZEROINGCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTWEAPONZEROINGCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponZeroingComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientWeaponZeroingComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientSoldierWeaponsPreviewComponent {
}

pub const CLIENTSOLDIERWEAPONSPREVIEWCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierWeaponsPreviewComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSOLDIERWEAPONSPREVIEWCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSoldierWeaponsPreviewComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTSOLDIERWEAPONSPREVIEWCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTSOLDIERWEAPONSPREVIEWCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierWeaponsPreviewComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierWeaponsPreviewComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientSoldierWeaponsComponent {
}

pub const CLIENTSOLDIERWEAPONSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierWeaponsComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSOLDIERWEAPONSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSoldierWeaponsComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTSOLDIERWEAPONSCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTSOLDIERWEAPONSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierWeaponsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierWeaponsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientSoldierPhysicsComponent {
}

pub const CLIENTSOLDIERPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTCHARACTERMASTERPHYSICSCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSOLDIERPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSoldierPhysicsComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTSOLDIERPHYSICSCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTSOLDIERPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierPhysicsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientSoldierHealthComponent {
}

pub const CLIENTSOLDIERHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierHealthComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTCHARACTERHEALTHCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSOLDIERHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSoldierHealthComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTSOLDIERHEALTHCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTSOLDIERHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierHealthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierHealthComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientSoldierFootplantEffectComponent {
}

pub const CLIENTSOLDIERFOOTPLANTEFFECTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierFootplantEffectComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSOLDIERFOOTPLANTEFFECTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSoldierFootplantEffectComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTSOLDIERFOOTPLANTEFFECTCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTSOLDIERFOOTPLANTEFFECTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierFootplantEffectComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierFootplantEffectComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientSoldierCustomizationComponent {
}

pub const CLIENTSOLDIERCUSTOMIZATIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierCustomizationComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTCHARACTERCUSTOMIZATIONCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSOLDIERCUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSoldierCustomizationComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTSOLDIERCUSTOMIZATIONCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTSOLDIERCUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierCustomizationComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierCustomizationComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientSoldierCameraComponent {
}

pub const CLIENTSOLDIERCAMERACOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierCameraComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTCHARACTERCAMERACOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSOLDIERCAMERACOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSoldierCameraComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTSOLDIERCAMERACOMPONENT_TYPE_INFO
    }
}


pub const CLIENTSOLDIERCAMERACOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierCameraComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierCameraComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientSoldierCameraCallback {
}

pub const CLIENTSOLDIERCAMERACALLBACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierCameraCallback",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TARGETCAMERACALLBACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSOLDIERCAMERACALLBACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSoldierCameraCallback {
    fn type_info() -> &'static TypeInfo {
        CLIENTSOLDIERCAMERACALLBACK_TYPE_INFO
    }
}


pub const CLIENTSOLDIERCAMERACALLBACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierCameraCallback-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierCameraCallback-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientSoldierEntity {
}

pub const CLIENTSOLDIERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTCHARACTERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSOLDIERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSoldierEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTSOLDIERENTITY_TYPE_INFO
    }
}


pub const CLIENTSOLDIERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientSchematicsAimEntity {
}

pub const CLIENTSCHEMATICSAIMENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSchematicsAimEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTAIMENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSCHEMATICSAIMENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSchematicsAimEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTSCHEMATICSAIMENTITY_TYPE_INFO
    }
}


pub const CLIENTSCHEMATICSAIMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSchematicsAimEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSchematicsAimEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientReplicatedAimEntity {
}

pub const CLIENTREPLICATEDAIMENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientReplicatedAimEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTREPLICATEDAIMENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientReplicatedAimEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTREPLICATEDAIMENTITY_TYPE_INFO
    }
}


pub const CLIENTREPLICATEDAIMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientReplicatedAimEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientReplicatedAimEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPickupEntity {
}

pub const CLIENTPICKUPENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPickupEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENTENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPICKUPENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPickupEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTPICKUPENTITY_TYPE_INFO
    }
}


pub const CLIENTPICKUPENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPickupEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientPickupEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientBlockAimAssistEntity {
}

pub const CLIENTBLOCKAIMASSISTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBlockAimAssistEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTBLOCKAIMASSISTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientBlockAimAssistEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTBLOCKAIMASSISTENTITY_TYPE_INFO
    }
}


pub const CLIENTBLOCKAIMASSISTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBlockAimAssistEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientBlockAimAssistEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientAimingSimulationDataProviderEntity {
}

pub const CLIENTAIMINGSIMULATIONDATAPROVIDERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAimingSimulationDataProviderEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTAIMINGSIMULATIONDATAPROVIDERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAimingSimulationDataProviderEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTAIMINGSIMULATIONDATAPROVIDERENTITY_TYPE_INFO
    }
}


pub const CLIENTAIMINGSIMULATIONDATAPROVIDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAimingSimulationDataProviderEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientAimingSimulationDataProviderEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientAimEntityBase {
}

pub const CLIENTAIMENTITYBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAimEntityBase",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTAIMENTITYBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAimEntityBase {
    fn type_info() -> &'static TypeInfo {
        CLIENTAIMENTITYBASE_TYPE_INFO
    }
}


pub const CLIENTAIMENTITYBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAimEntityBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientAimEntityBase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientAimEntity {
}

pub const CLIENTAIMENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAimEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTAIMENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTAIMENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAimEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTAIMENTITY_TYPE_INFO
    }
}


pub const CLIENTAIMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAimEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientAimEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCustomizeSoldierEntity {
}

pub const SERVERCUSTOMIZESOLDIERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCustomizeSoldierEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERCUSTOMIZESOLDIERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCustomizeSoldierEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERCUSTOMIZESOLDIERENTITY_TYPE_INFO
    }
}


pub const SERVERCUSTOMIZESOLDIERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCustomizeSoldierEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerCustomizeSoldierEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerTripwireEntity {
}

pub const SERVERTRIPWIREENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTripwireEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENTENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERTRIPWIREENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerTripwireEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERTRIPWIREENTITY_TYPE_INFO
    }
}


pub const SERVERTRIPWIREENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTripwireEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerTripwireEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSoldierEntity {
}

pub const SERVERSOLDIERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERCHARACTERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSOLDIERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSoldierEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERSOLDIERENTITY_TYPE_INFO
    }
}


pub const SERVERSOLDIERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerSoldierEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerReplicatedAimEntity {
}

pub const SERVERREPLICATEDAIMENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerReplicatedAimEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERREPLICATEDAIMENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerReplicatedAimEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERREPLICATEDAIMENTITY_TYPE_INFO
    }
}


pub const SERVERREPLICATEDAIMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerReplicatedAimEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerReplicatedAimEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPickupEntity {
}

pub const SERVERPICKUPENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPickupEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENTENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERPICKUPENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPickupEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERPICKUPENTITY_TYPE_INFO
    }
}


pub const SERVERPICKUPENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPickupEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerPickupEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerZeroingWeapon {
}

pub const SERVERZEROINGWEAPON_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerZeroingWeapon",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERWEAPON_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERZEROINGWEAPON_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerZeroingWeapon {
    fn type_info() -> &'static TypeInfo {
        SERVERZEROINGWEAPON_TYPE_INFO
    }
}


pub const SERVERZEROINGWEAPON_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerZeroingWeapon-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerZeroingWeapon-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSoldierWeaponSpawnInfo {
}

pub const SERVERSOLDIERWEAPONSPAWNINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierWeaponSpawnInfo",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(SERVERSOLDIERWEAPONSPAWNINFO_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSoldierWeaponSpawnInfo {
    fn type_info() -> &'static TypeInfo {
        SERVERSOLDIERWEAPONSPAWNINFO_TYPE_INFO
    }
}


pub const SERVERSOLDIERWEAPONSPAWNINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierWeaponSpawnInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerSoldierWeaponSpawnInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSoldierWeapon {
}

pub const SERVERSOLDIERWEAPON_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierWeapon",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENTENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSOLDIERWEAPON_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSoldierWeapon {
    fn type_info() -> &'static TypeInfo {
        SERVERSOLDIERWEAPON_TYPE_INFO
    }
}


pub const SERVERSOLDIERWEAPON_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierWeapon-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerSoldierWeapon-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAnimationTurretRotationComponent {
}

pub const SERVERANIMATIONTURRETROTATIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAnimationTurretRotationComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERANIMATIONTURRETROTATIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAnimationTurretRotationComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERANIMATIONTURRETROTATIONCOMPONENT_TYPE_INFO
    }
}


pub const SERVERANIMATIONTURRETROTATIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAnimationTurretRotationComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerAnimationTurretRotationComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGrenadeEntity {
}

pub const SERVERGRENADEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGrenadeEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGHOSTPROJECTILEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERGRENADEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerGrenadeEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERGRENADEENTITY_TYPE_INFO
    }
}


pub const SERVERGRENADEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGrenadeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerGrenadeEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerExplosionPackEntity {
}

pub const SERVEREXPLOSIONPACKENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerExplosionPackEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGHOSTPROJECTILEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVEREXPLOSIONPACKENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerExplosionPackEntity {
    fn type_info() -> &'static TypeInfo {
        SERVEREXPLOSIONPACKENTITY_TYPE_INFO
    }
}


pub const SERVEREXPLOSIONPACKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerExplosionPackEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerExplosionPackEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerMissileHealthComponent {
}

pub const SERVERMISSILEHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMissileHealthComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMEHEALTHCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERMISSILEHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerMissileHealthComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERMISSILEHEALTHCOMPONENT_TYPE_INFO
    }
}


pub const SERVERMISSILEHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMissileHealthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerMissileHealthComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerExplosionPackPhysicsComponent {
}

pub const SERVEREXPLOSIONPACKPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerExplosionPackPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PARTPHYSICSCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVEREXPLOSIONPACKPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerExplosionPackPhysicsComponent {
    fn type_info() -> &'static TypeInfo {
        SERVEREXPLOSIONPACKPHYSICSCOMPONENT_TYPE_INFO
    }
}


pub const SERVEREXPLOSIONPACKPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerExplosionPackPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerExplosionPackPhysicsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerExplosionPackHealthComponent {
}

pub const SERVEREXPLOSIONPACKHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerExplosionPackHealthComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMEHEALTHCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVEREXPLOSIONPACKHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerExplosionPackHealthComponent {
    fn type_info() -> &'static TypeInfo {
        SERVEREXPLOSIONPACKHEALTHCOMPONENT_TYPE_INFO
    }
}


pub const SERVEREXPLOSIONPACKHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerExplosionPackHealthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerExplosionPackHealthComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSoldierEntryComponent {
}

pub const SERVERSOLDIERENTRYCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierEntryComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERCHARACTERENTRYCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSOLDIERENTRYCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSoldierEntryComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERSOLDIERENTRYCOMPONENT_TYPE_INFO
    }
}


pub const SERVERSOLDIERENTRYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierEntryComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerSoldierEntryComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerWeaponZeroingComponent {
}

pub const SERVERWEAPONZEROINGCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponZeroingComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERWEAPONZEROINGCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerWeaponZeroingComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERWEAPONZEROINGCOMPONENT_TYPE_INFO
    }
}


pub const SERVERWEAPONZEROINGCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponZeroingComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerWeaponZeroingComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSoldierInteractionEntityInRangeChangedMessage {
}

pub const SERVERSOLDIERINTERACTIONENTITYINRANGECHANGEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierInteractionEntityInRangeChangedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerSoldierInteractionEntityInRangeChangedMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERSOLDIERINTERACTIONENTITYINRANGECHANGEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSoldierSoldierVsSoldierCollisionMessage {
}

pub const SERVERSOLDIERSOLDIERVSSOLDIERCOLLISIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierSoldierVsSoldierCollisionMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerSoldierSoldierVsSoldierCollisionMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERSOLDIERSOLDIERVSSOLDIERCOLLISIONMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSoldierPickedUpSupplySphereMessage {
}

pub const SERVERSOLDIERPICKEDUPSUPPLYSPHEREMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierPickedUpSupplySphereMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerSoldierPickedUpSupplySphereMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERSOLDIERPICKEDUPSUPPLYSPHEREMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSoldierOnUnspawnMessage {
}

pub const SERVERSOLDIERONUNSPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierOnUnspawnMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerSoldierOnUnspawnMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERSOLDIERONUNSPAWNMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSoldierThrowDistractionMessage {
}

pub const SERVERSOLDIERTHROWDISTRACTIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierThrowDistractionMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ServerSoldierThrowDistractionMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERSOLDIERTHROWDISTRACTIONMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSoldierSelfHealMessage {
}

pub const SERVERSOLDIERSELFHEALMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierSelfHealMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerSoldierSelfHealMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERSOLDIERSELFHEALMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSoldierManDownMessage {
}

pub const SERVERSOLDIERMANDOWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierManDownMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerSoldierManDownMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERSOLDIERMANDOWNMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSoldierOnInitMessage {
}

pub const SERVERSOLDIERONINITMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierOnInitMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerSoldierOnInitMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERSOLDIERONINITMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSoldierChangingWeaponMessage {
}

pub const SERVERSOLDIERCHANGINGWEAPONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierChangingWeaponMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerSoldierChangingWeaponMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERSOLDIERCHANGINGWEAPONMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSoldierFiringMessage {
}

pub const SERVERSOLDIERFIRINGMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierFiringMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerSoldierFiringMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERSOLDIERFIRINGMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSoldierDamagedMessage {
}

pub const SERVERSOLDIERDAMAGEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierDamagedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerSoldierDamagedMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERSOLDIERDAMAGEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerProjectileMissileDestroyedMessage {
}

pub const SERVERPROJECTILEMISSILEDESTROYEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerProjectileMissileDestroyedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerProjectileMissileDestroyedMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERPROJECTILEMISSILEDESTROYEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerProjectileMissileDamagedMessage {
}

pub const SERVERPROJECTILEMISSILEDAMAGEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerProjectileMissileDamagedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerProjectileMissileDamagedMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERPROJECTILEMISSILEDAMAGEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientSoldierInteractionEntityInRangeChangedMessage {
}

pub const CLIENTSOLDIERINTERACTIONENTITYINRANGECHANGEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierInteractionEntityInRangeChangedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientSoldierInteractionEntityInRangeChangedMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTSOLDIERINTERACTIONENTITYINRANGECHANGEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientSoldierChangeCoverStateMessage {
}

pub const CLIENTSOLDIERCHANGECOVERSTATEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierChangeCoverStateMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientSoldierChangeCoverStateMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTSOLDIERCHANGECOVERSTATEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AimingHandle {
}

pub const AIMINGHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingHandle",
    flags: MemberInfoFlags::new(73),
    module: "Soldier",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(AIMINGHANDLE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AimingHandle {
    fn type_info() -> &'static TypeInfo {
        AIMINGHANDLE_TYPE_INFO
    }
}


pub const AIMINGHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("AimingHandle-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AimingRenderReturnedValues {
    pub aimer_position: super::core::Vec3,
    pub last_hit_position: super::core::Vec3,
    pub recoil_compensation: super::core::Vec2,
    pub last_apply_angles_from_sim_tick: u32,
    pub yaw: f32,
    pub pitch: f32,
    pub input_magnitude: f32,
    pub snap_zoom_break_away: bool,
    pub use_aim_assist: bool,
    pub allow_blend_out: bool,
}

pub const AIMINGRENDERRETURNEDVALUES_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingRenderReturnedValues",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "AimerPosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AimingRenderReturnedValues, aimer_position),
            },
            FieldInfoData {
                name: "LastHitPosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AimingRenderReturnedValues, last_hit_position),
            },
            FieldInfoData {
                name: "RecoilCompensation",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(AimingRenderReturnedValues, recoil_compensation),
            },
            FieldInfoData {
                name: "LastApplyAnglesFromSimTick",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(AimingRenderReturnedValues, last_apply_angles_from_sim_tick),
            },
            FieldInfoData {
                name: "Yaw",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingRenderReturnedValues, yaw),
            },
            FieldInfoData {
                name: "Pitch",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingRenderReturnedValues, pitch),
            },
            FieldInfoData {
                name: "InputMagnitude",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingRenderReturnedValues, input_magnitude),
            },
            FieldInfoData {
                name: "SnapZoomBreakAway",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AimingRenderReturnedValues, snap_zoom_break_away),
            },
            FieldInfoData {
                name: "UseAimAssist",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AimingRenderReturnedValues, use_aim_assist),
            },
            FieldInfoData {
                name: "AllowBlendOut",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AimingRenderReturnedValues, allow_blend_out),
            },
        ],
    }),
    array_type: Some(AIMINGRENDERRETURNEDVALUES_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AimingRenderReturnedValues {
    fn type_info() -> &'static TypeInfo {
        AIMINGRENDERRETURNEDVALUES_TYPE_INFO
    }
}


pub const AIMINGRENDERRETURNEDVALUES_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingRenderReturnedValues-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("AimingRenderReturnedValues-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AimingRenderUpdateContext {
    pub aimer_position: super::core::Vec3,
    pub aimer_safe_position: super::core::Vec3,
    pub last_aim_direction: super::core::Vec3,
    pub last_hit_position: super::core::Vec3,
    pub aiming_input: super::core::Vec2,
    pub input_direction: super::core::Vec2,
    pub input_acceleration_velocity: super::core::Vec2,
    pub recoil_offset: super::core::Vec2,
    pub last_tick: u32,
    pub last_apply_angles_from_sim_tick: u32,
    pub yaw: f32,
    pub pitch: f32,
    pub input_magnitude: f32,
    pub soft_zone_lambda_yaw_attract: f32,
    pub soft_zone_lambda_pitch_attract: f32,
    pub soft_zone_lambda_slowdown: f32,
    pub target_distance: f32,
    pub yaw_change_attract: f32,
    pub pitch_change_attract: f32,
    pub time_since_yaw_input: f32,
    pub time_since_pitch_input: f32,
    pub acceleration: f32,
    pub acceleration_timer: f32,
    pub aimer_arm_length: f32,
    pub time_to_delay_after_collision: f32,
    pub snap_zoom_break_away: bool,
    pub is_mouse_aiming: bool,
    pub use_aim_assist: bool,
    pub use_input_polynomials: bool,
    pub allow_blend_out: bool,
}

pub const AIMINGRENDERUPDATECONTEXT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingRenderUpdateContext",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "AimerPosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AimingRenderUpdateContext, aimer_position),
            },
            FieldInfoData {
                name: "AimerSafePosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AimingRenderUpdateContext, aimer_safe_position),
            },
            FieldInfoData {
                name: "LastAimDirection",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AimingRenderUpdateContext, last_aim_direction),
            },
            FieldInfoData {
                name: "LastHitPosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AimingRenderUpdateContext, last_hit_position),
            },
            FieldInfoData {
                name: "AimingInput",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(AimingRenderUpdateContext, aiming_input),
            },
            FieldInfoData {
                name: "InputDirection",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(AimingRenderUpdateContext, input_direction),
            },
            FieldInfoData {
                name: "InputAccelerationVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(AimingRenderUpdateContext, input_acceleration_velocity),
            },
            FieldInfoData {
                name: "RecoilOffset",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(AimingRenderUpdateContext, recoil_offset),
            },
            FieldInfoData {
                name: "LastTick",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(AimingRenderUpdateContext, last_tick),
            },
            FieldInfoData {
                name: "LastApplyAnglesFromSimTick",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(AimingRenderUpdateContext, last_apply_angles_from_sim_tick),
            },
            FieldInfoData {
                name: "Yaw",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingRenderUpdateContext, yaw),
            },
            FieldInfoData {
                name: "Pitch",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingRenderUpdateContext, pitch),
            },
            FieldInfoData {
                name: "InputMagnitude",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingRenderUpdateContext, input_magnitude),
            },
            FieldInfoData {
                name: "SoftZoneLambdaYawAttract",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingRenderUpdateContext, soft_zone_lambda_yaw_attract),
            },
            FieldInfoData {
                name: "SoftZoneLambdaPitchAttract",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingRenderUpdateContext, soft_zone_lambda_pitch_attract),
            },
            FieldInfoData {
                name: "SoftZoneLambdaSlowdown",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingRenderUpdateContext, soft_zone_lambda_slowdown),
            },
            FieldInfoData {
                name: "TargetDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingRenderUpdateContext, target_distance),
            },
            FieldInfoData {
                name: "YawChangeAttract",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingRenderUpdateContext, yaw_change_attract),
            },
            FieldInfoData {
                name: "PitchChangeAttract",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingRenderUpdateContext, pitch_change_attract),
            },
            FieldInfoData {
                name: "TimeSinceYawInput",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingRenderUpdateContext, time_since_yaw_input),
            },
            FieldInfoData {
                name: "TimeSincePitchInput",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingRenderUpdateContext, time_since_pitch_input),
            },
            FieldInfoData {
                name: "Acceleration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingRenderUpdateContext, acceleration),
            },
            FieldInfoData {
                name: "AccelerationTimer",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingRenderUpdateContext, acceleration_timer),
            },
            FieldInfoData {
                name: "AimerArmLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingRenderUpdateContext, aimer_arm_length),
            },
            FieldInfoData {
                name: "TimeToDelayAfterCollision",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingRenderUpdateContext, time_to_delay_after_collision),
            },
            FieldInfoData {
                name: "SnapZoomBreakAway",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AimingRenderUpdateContext, snap_zoom_break_away),
            },
            FieldInfoData {
                name: "IsMouseAiming",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AimingRenderUpdateContext, is_mouse_aiming),
            },
            FieldInfoData {
                name: "UseAimAssist",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AimingRenderUpdateContext, use_aim_assist),
            },
            FieldInfoData {
                name: "UseInputPolynomials",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AimingRenderUpdateContext, use_input_polynomials),
            },
            FieldInfoData {
                name: "AllowBlendOut",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AimingRenderUpdateContext, allow_blend_out),
            },
        ],
    }),
    array_type: Some(AIMINGRENDERUPDATECONTEXT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AimingRenderUpdateContext {
    fn type_info() -> &'static TypeInfo {
        AIMINGRENDERUPDATECONTEXT_TYPE_INFO
    }
}


pub const AIMINGRENDERUPDATECONTEXT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingRenderUpdateContext-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("AimingRenderUpdateContext-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AimingSimUpdateContext {
    pub difficulty_data: super::game_shared::DifficultyData,
    pub aiming_range: f32,
    pub attract_pitch_strength: f32,
    pub attract_soft_zone: f32,
    pub attract_yaw_strength: f32,
    pub snap_zoom_post_time_no_input: f32,
    pub snap_zoom_post_time: f32,
    pub snap_zoom_since_last_timer: f32,
    pub zoom_transition_timer: f32,
    pub local_player_id: super::core::LocalPlayerId,
    pub attract_zooming_post_player_aiming: bool,
    pub force_aim_snap_deactivate: bool,
    pub force_pick_best_target: bool,
    pub has_been_sprinting: bool,
    pub is_sprinting: bool,
    pub use_aim_helpers_rotation: bool,
    pub use_aim_helpers_slowdown: bool,
    pub snap_zoom_post_time_dynamic_point: bool,
    pub snap_zoom_target_changed: bool,
}

pub const AIMINGSIMUPDATECONTEXT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingSimUpdateContext",
    flags: MemberInfoFlags::new(73),
    module: "Soldier",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "DifficultyData",
                flags: MemberInfoFlags::new(0),
                field_type: DIFFICULTYDATA_TYPE_INFO,
                rust_offset: offset_of!(AimingSimUpdateContext, difficulty_data),
            },
            FieldInfoData {
                name: "AimingRange",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingSimUpdateContext, aiming_range),
            },
            FieldInfoData {
                name: "AttractPitchStrength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingSimUpdateContext, attract_pitch_strength),
            },
            FieldInfoData {
                name: "AttractSoftZone",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingSimUpdateContext, attract_soft_zone),
            },
            FieldInfoData {
                name: "AttractYawStrength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingSimUpdateContext, attract_yaw_strength),
            },
            FieldInfoData {
                name: "SnapZoomPostTimeNoInput",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingSimUpdateContext, snap_zoom_post_time_no_input),
            },
            FieldInfoData {
                name: "SnapZoomPostTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingSimUpdateContext, snap_zoom_post_time),
            },
            FieldInfoData {
                name: "SnapZoomSinceLastTimer",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingSimUpdateContext, snap_zoom_since_last_timer),
            },
            FieldInfoData {
                name: "ZoomTransitionTimer",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingSimUpdateContext, zoom_transition_timer),
            },
            FieldInfoData {
                name: "LocalPlayerId",
                flags: MemberInfoFlags::new(0),
                field_type: LOCALPLAYERID_TYPE_INFO,
                rust_offset: offset_of!(AimingSimUpdateContext, local_player_id),
            },
            FieldInfoData {
                name: "AttractZoomingPostPlayerAiming",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AimingSimUpdateContext, attract_zooming_post_player_aiming),
            },
            FieldInfoData {
                name: "ForceAimSnapDeactivate",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AimingSimUpdateContext, force_aim_snap_deactivate),
            },
            FieldInfoData {
                name: "ForcePickBestTarget",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AimingSimUpdateContext, force_pick_best_target),
            },
            FieldInfoData {
                name: "HasBeenSprinting",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AimingSimUpdateContext, has_been_sprinting),
            },
            FieldInfoData {
                name: "IsSprinting",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AimingSimUpdateContext, is_sprinting),
            },
            FieldInfoData {
                name: "UseAimHelpersRotation",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AimingSimUpdateContext, use_aim_helpers_rotation),
            },
            FieldInfoData {
                name: "UseAimHelpersSlowdown",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AimingSimUpdateContext, use_aim_helpers_slowdown),
            },
            FieldInfoData {
                name: "SnapZoomPostTimeDynamicPoint",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AimingSimUpdateContext, snap_zoom_post_time_dynamic_point),
            },
            FieldInfoData {
                name: "SnapZoomTargetChanged",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AimingSimUpdateContext, snap_zoom_target_changed),
            },
        ],
    }),
    array_type: Some(AIMINGSIMUPDATECONTEXT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AimingSimUpdateContext {
    fn type_info() -> &'static TypeInfo {
        AIMINGSIMUPDATECONTEXT_TYPE_INFO
    }
}


pub const AIMINGSIMUPDATECONTEXT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingSimUpdateContext-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("AimingSimUpdateContext-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AimingSimDynamicState {
    pub aiming_environment_target: AimingEnvironmentTarget,
    pub aiming_constraints: AimingConstraints,
    pub collision_excluded_bodies: super::physics::PhysicsRenderWorldHandle,
    pub character_entity_space_component_transform: super::core::LinearTransform,
    pub lock_aim_transform: super::core::LinearTransform,
    pub aimer_root_position: super::core::Vec3,
    pub desired_aimer_local_position: super::core::Vec3,
    pub desired_aimer_safe_position: super::core::Vec3,
    pub static_aimer_safe_position: super::core::Vec3,
    pub force_aim_snap_target_position: super::core::Vec3,
    pub reticle_speed: super::core::Vec3,
    pub surface_angular_velocity: super::core::Vec3,
    pub sim_aiming_input: super::core::Vec2,
    pub aim_scale: super::core::Vec2,
    pub attract_distance_fall_off: super::core::Vec2,
    pub max_angular_velocity: super::core::Vec2,
    pub movement_input: super::core::Vec2,
    pub recoil: super::core::Vec2,
    pub aim_sway_offset: super::core::Vec2,
    pub solider_aiming_simulation_data: super::soldier_shared::SoldierAimingSimulationData,
    pub zoom_level: u32,
    pub tick: u32,
    pub apply_angles_from_sim_tick: u32,
    pub ignore_constraints_tick: u32,
    pub attract_user_input_multiplier: f32,
    pub attract_zooming_post_timer: f32,
    pub attract_zooming_post_time: f32,
    pub last_hit_position_distance: f32,
    pub look_speed_multiplier: f32,
    pub minimum_pitch: f32,
    pub maximum_pitch: f32,
    pub reticle_field_of_view: f32,
    pub sim_pitch_to_apply: f32,
    pub sim_yaw_to_apply: f32,
    pub snap_zoom_break_away_timer: f32,
    pub snap_zoom_timer: f32,
    pub snap_zoom_time: f32,
    pub world_space_lock_efficiency_pitch: f32,
    pub world_space_lock_efficiency_yaw: f32,
    pub aimer_collision_blend_out: f32,
    pub time_to_delay_after_collision: f32,
    pub override_mode: AimOverrideMode,
    pub aim_at_last_hit_position: bool,
    pub force_aim_snap: bool,
    pub has_aiming_constraints: bool,
    pub has_angular_velocity_constraints: bool,
    pub has_character_entity_space_component: bool,
    pub is_alive: bool,
    pub is_dead: bool,
    pub is_fps_aiming_disabled: bool,
    pub is_snap_zoomed: bool,
    pub snap_zoom_allowed: bool,
    pub zoom_has_changed: bool,
    pub zoom_in_aiming_help_active: bool,
    pub aim_assist_option_enabled: bool,
    pub field_flag_changed0: u32,
    pub field_flag_changed1: u32,
}

pub const AIMINGSIMDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingSimDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "Soldier",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "AimingEnvironmentTarget",
                flags: MemberInfoFlags::new(0),
                field_type: AIMINGENVIRONMENTTARGET_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, aiming_environment_target),
            },
            FieldInfoData {
                name: "AimingConstraints",
                flags: MemberInfoFlags::new(0),
                field_type: AIMINGCONSTRAINTS_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, aiming_constraints),
            },
            FieldInfoData {
                name: "CollisionExcludedBodies",
                flags: MemberInfoFlags::new(0),
                field_type: PHYSICSRENDERWORLDHANDLE_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, collision_excluded_bodies),
            },
            FieldInfoData {
                name: "CharacterEntitySpaceComponentTransform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, character_entity_space_component_transform),
            },
            FieldInfoData {
                name: "LockAimTransform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, lock_aim_transform),
            },
            FieldInfoData {
                name: "AimerRootPosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, aimer_root_position),
            },
            FieldInfoData {
                name: "DesiredAimerLocalPosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, desired_aimer_local_position),
            },
            FieldInfoData {
                name: "DesiredAimerSafePosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, desired_aimer_safe_position),
            },
            FieldInfoData {
                name: "StaticAimerSafePosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, static_aimer_safe_position),
            },
            FieldInfoData {
                name: "ForceAimSnapTargetPosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, force_aim_snap_target_position),
            },
            FieldInfoData {
                name: "ReticleSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, reticle_speed),
            },
            FieldInfoData {
                name: "SurfaceAngularVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, surface_angular_velocity),
            },
            FieldInfoData {
                name: "SimAimingInput",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, sim_aiming_input),
            },
            FieldInfoData {
                name: "AimScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, aim_scale),
            },
            FieldInfoData {
                name: "AttractDistanceFallOff",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, attract_distance_fall_off),
            },
            FieldInfoData {
                name: "MaxAngularVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, max_angular_velocity),
            },
            FieldInfoData {
                name: "MovementInput",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, movement_input),
            },
            FieldInfoData {
                name: "Recoil",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, recoil),
            },
            FieldInfoData {
                name: "AimSwayOffset",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, aim_sway_offset),
            },
            FieldInfoData {
                name: "SoliderAimingSimulationData",
                flags: MemberInfoFlags::new(0),
                field_type: SOLDIERAIMINGSIMULATIONDATA_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, solider_aiming_simulation_data),
            },
            FieldInfoData {
                name: "ZoomLevel",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, zoom_level),
            },
            FieldInfoData {
                name: "Tick",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, tick),
            },
            FieldInfoData {
                name: "ApplyAnglesFromSimTick",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, apply_angles_from_sim_tick),
            },
            FieldInfoData {
                name: "IgnoreConstraintsTick",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, ignore_constraints_tick),
            },
            FieldInfoData {
                name: "AttractUserInputMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, attract_user_input_multiplier),
            },
            FieldInfoData {
                name: "AttractZoomingPostTimer",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, attract_zooming_post_timer),
            },
            FieldInfoData {
                name: "AttractZoomingPostTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, attract_zooming_post_time),
            },
            FieldInfoData {
                name: "LastHitPositionDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, last_hit_position_distance),
            },
            FieldInfoData {
                name: "LookSpeedMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, look_speed_multiplier),
            },
            FieldInfoData {
                name: "MinimumPitch",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, minimum_pitch),
            },
            FieldInfoData {
                name: "MaximumPitch",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, maximum_pitch),
            },
            FieldInfoData {
                name: "ReticleFieldOfView",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, reticle_field_of_view),
            },
            FieldInfoData {
                name: "SimPitchToApply",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, sim_pitch_to_apply),
            },
            FieldInfoData {
                name: "SimYawToApply",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, sim_yaw_to_apply),
            },
            FieldInfoData {
                name: "SnapZoomBreakAwayTimer",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, snap_zoom_break_away_timer),
            },
            FieldInfoData {
                name: "SnapZoomTimer",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, snap_zoom_timer),
            },
            FieldInfoData {
                name: "SnapZoomTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, snap_zoom_time),
            },
            FieldInfoData {
                name: "WorldSpaceLockEfficiencyPitch",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, world_space_lock_efficiency_pitch),
            },
            FieldInfoData {
                name: "WorldSpaceLockEfficiencyYaw",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, world_space_lock_efficiency_yaw),
            },
            FieldInfoData {
                name: "AimerCollisionBlendOut",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, aimer_collision_blend_out),
            },
            FieldInfoData {
                name: "TimeToDelayAfterCollision",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, time_to_delay_after_collision),
            },
            FieldInfoData {
                name: "OverrideMode",
                flags: MemberInfoFlags::new(0),
                field_type: AIMOVERRIDEMODE_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, override_mode),
            },
            FieldInfoData {
                name: "AimAtLastHitPosition",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, aim_at_last_hit_position),
            },
            FieldInfoData {
                name: "ForceAimSnap",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, force_aim_snap),
            },
            FieldInfoData {
                name: "HasAimingConstraints",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, has_aiming_constraints),
            },
            FieldInfoData {
                name: "HasAngularVelocityConstraints",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, has_angular_velocity_constraints),
            },
            FieldInfoData {
                name: "HasCharacterEntitySpaceComponent",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, has_character_entity_space_component),
            },
            FieldInfoData {
                name: "IsAlive",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, is_alive),
            },
            FieldInfoData {
                name: "IsDead",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, is_dead),
            },
            FieldInfoData {
                name: "IsFpsAimingDisabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, is_fps_aiming_disabled),
            },
            FieldInfoData {
                name: "IsSnapZoomed",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, is_snap_zoomed),
            },
            FieldInfoData {
                name: "SnapZoomAllowed",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, snap_zoom_allowed),
            },
            FieldInfoData {
                name: "ZoomHasChanged",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, zoom_has_changed),
            },
            FieldInfoData {
                name: "ZoomInAimingHelpActive",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, zoom_in_aiming_help_active),
            },
            FieldInfoData {
                name: "AimAssistOptionEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, aim_assist_option_enabled),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, field_flag_changed0),
            },
            FieldInfoData {
                name: "FieldFlagChanged1",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(AimingSimDynamicState, field_flag_changed1),
            },
        ],
    }),
    array_type: Some(AIMINGSIMDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AimingSimDynamicState {
    fn type_info() -> &'static TypeInfo {
        AIMINGSIMDYNAMICSTATE_TYPE_INFO
    }
}


pub const AIMINGSIMDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingSimDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("AimingSimDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AimingSimStaticState {
    pub local_player_id: super::core::LocalPlayerId,
    pub yaw_input_action: i32,
    pub pitch_input_action: i32,
    pub field_flag_changed0: u8,
}

pub const AIMINGSIMSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingSimStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "LocalPlayerId",
                flags: MemberInfoFlags::new(0),
                field_type: LOCALPLAYERID_TYPE_INFO,
                rust_offset: offset_of!(AimingSimStaticState, local_player_id),
            },
            FieldInfoData {
                name: "YawInputAction",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AimingSimStaticState, yaw_input_action),
            },
            FieldInfoData {
                name: "PitchInputAction",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AimingSimStaticState, pitch_input_action),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(AimingSimStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(AIMINGSIMSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for AimingSimStaticState {
    fn type_info() -> &'static TypeInfo {
        AIMINGSIMSTATICSTATE_TYPE_INFO
    }
}


pub const AIMINGSIMSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingSimStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("AimingSimStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AimingConstraints {
    pub min_yaw: f32,
    pub max_yaw: f32,
    pub min_pitch: f32,
    pub max_pitch: f32,
    pub pitch_offset: f32,
    pub yaw_offset: f32,
}

pub const AIMINGCONSTRAINTS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingConstraints",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "MinYaw",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingConstraints, min_yaw),
            },
            FieldInfoData {
                name: "MaxYaw",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingConstraints, max_yaw),
            },
            FieldInfoData {
                name: "MinPitch",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingConstraints, min_pitch),
            },
            FieldInfoData {
                name: "MaxPitch",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingConstraints, max_pitch),
            },
            FieldInfoData {
                name: "PitchOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingConstraints, pitch_offset),
            },
            FieldInfoData {
                name: "YawOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingConstraints, yaw_offset),
            },
        ],
    }),
    array_type: Some(AIMINGCONSTRAINTS_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for AimingConstraints {
    fn type_info() -> &'static TypeInfo {
        AIMINGCONSTRAINTS_TYPE_INFO
    }
}


pub const AIMINGCONSTRAINTS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingConstraints-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("AimingConstraints-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AimingEnvironmentTarget {
    pub position: super::core::Vec3,
    pub snap_position: super::core::Vec3,
    pub velocity: super::core::Vec3,
    pub id: u64,
    pub radius: f32,
    pub snap_radius: f32,
    pub is_sticky: bool,
    pub is_snap: bool,
}

pub const AIMINGENVIRONMENTTARGET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingEnvironmentTarget",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Position",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AimingEnvironmentTarget, position),
            },
            FieldInfoData {
                name: "SnapPosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AimingEnvironmentTarget, snap_position),
            },
            FieldInfoData {
                name: "Velocity",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AimingEnvironmentTarget, velocity),
            },
            FieldInfoData {
                name: "Id",
                flags: MemberInfoFlags::new(0),
                field_type: UINT64_TYPE_INFO,
                rust_offset: offset_of!(AimingEnvironmentTarget, id),
            },
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingEnvironmentTarget, radius),
            },
            FieldInfoData {
                name: "SnapRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AimingEnvironmentTarget, snap_radius),
            },
            FieldInfoData {
                name: "IsSticky",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AimingEnvironmentTarget, is_sticky),
            },
            FieldInfoData {
                name: "IsSnap",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AimingEnvironmentTarget, is_snap),
            },
        ],
    }),
    array_type: Some(AIMINGENVIRONMENTTARGET_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AimingEnvironmentTarget {
    fn type_info() -> &'static TypeInfo {
        AIMINGENVIRONMENTTARGET_TYPE_INFO
    }
}


pub const AIMINGENVIRONMENTTARGET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingEnvironmentTarget-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("AimingEnvironmentTarget-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum AimOverrideMode {
    #[default]
    AimOverrideMode_DisableOverride = 0,
    AimOverrideMode_Blend = 1,
    AimOverrideMode_Force = 2,
}

pub const AIMOVERRIDEMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimOverrideMode",
    flags: MemberInfoFlags::new(49429),
    module: "Soldier",
    data: TypeInfoData::Enum,
    array_type: Some(AIMOVERRIDEMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AimOverrideMode {
    fn type_info() -> &'static TypeInfo {
        AIMOVERRIDEMODE_TYPE_INFO
    }
}


pub const AIMOVERRIDEMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimOverrideMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("AimOverrideMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SoldierThirdPersonCameraRenderState {
    pub aimer_position: super::core::Vec3,
    pub hit_position: super::core::Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub arm_length: f32,
    pub previous_arm_length: f32,
    pub previous_collided_arm_length: f32,
    pub is_colliding: bool,
}

pub const SOLDIERTHIRDPERSONCAMERARENDERSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoldierThirdPersonCameraRenderState",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "AimerPosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SoldierThirdPersonCameraRenderState, aimer_position),
            },
            FieldInfoData {
                name: "HitPosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SoldierThirdPersonCameraRenderState, hit_position),
            },
            FieldInfoData {
                name: "Yaw",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SoldierThirdPersonCameraRenderState, yaw),
            },
            FieldInfoData {
                name: "Pitch",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SoldierThirdPersonCameraRenderState, pitch),
            },
            FieldInfoData {
                name: "ArmLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SoldierThirdPersonCameraRenderState, arm_length),
            },
            FieldInfoData {
                name: "PreviousArmLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SoldierThirdPersonCameraRenderState, previous_arm_length),
            },
            FieldInfoData {
                name: "PreviousCollidedArmLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SoldierThirdPersonCameraRenderState, previous_collided_arm_length),
            },
            FieldInfoData {
                name: "IsColliding",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SoldierThirdPersonCameraRenderState, is_colliding),
            },
        ],
    }),
    array_type: Some(SOLDIERTHIRDPERSONCAMERARENDERSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SoldierThirdPersonCameraRenderState {
    fn type_info() -> &'static TypeInfo {
        SOLDIERTHIRDPERSONCAMERARENDERSTATE_TYPE_INFO
    }
}


pub const SOLDIERTHIRDPERSONCAMERARENDERSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoldierThirdPersonCameraRenderState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("SoldierThirdPersonCameraRenderState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SoldierThirdPersonCameraSimState {
    pub local_transform: super::core::LinearTransform,
    pub free_transform: super::core::LinearTransform,
    pub procedural_transform: super::core::LinearTransform,
    pub shake_transform: super::core::LinearTransform,
    pub roll_transform: super::core::LinearTransform,
    pub sim_aimer_position: super::core::Vec3,
    pub sim_hit_position: super::core::Vec3,
    pub aiming: AimingHandle,
    pub sim_yaw: f32,
    pub sim_pitch: f32,
    pub max_pitch: f32,
    pub arm_length: f32,
    pub min_reduce_pitch: f32,
    pub max_reduce_pitch: f32,
    pub max_reduced_length: f32,
    pub collision_width_padding: f32,
    pub collision_blend_in: f32,
    pub collision_blend_out: f32,
    pub free_transform_blend_value: f32,
    pub near_plane: f32,
    pub far_plane: f32,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub reduce_arm_length_looking_up: bool,
    pub field_flag_changed0: u32,
}

pub const SOLDIERTHIRDPERSONCAMERASIMSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoldierThirdPersonCameraSimState",
    flags: MemberInfoFlags::new(73),
    module: "Soldier",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "LocalTransform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, local_transform),
            },
            FieldInfoData {
                name: "FreeTransform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, free_transform),
            },
            FieldInfoData {
                name: "ProceduralTransform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, procedural_transform),
            },
            FieldInfoData {
                name: "ShakeTransform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, shake_transform),
            },
            FieldInfoData {
                name: "RollTransform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, roll_transform),
            },
            FieldInfoData {
                name: "SimAimerPosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, sim_aimer_position),
            },
            FieldInfoData {
                name: "SimHitPosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, sim_hit_position),
            },
            FieldInfoData {
                name: "Aiming",
                flags: MemberInfoFlags::new(0),
                field_type: AIMINGHANDLE_TYPE_INFO,
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, aiming),
            },
            FieldInfoData {
                name: "SimYaw",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, sim_yaw),
            },
            FieldInfoData {
                name: "SimPitch",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, sim_pitch),
            },
            FieldInfoData {
                name: "MaxPitch",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, max_pitch),
            },
            FieldInfoData {
                name: "ArmLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, arm_length),
            },
            FieldInfoData {
                name: "MinReducePitch",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, min_reduce_pitch),
            },
            FieldInfoData {
                name: "MaxReducePitch",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, max_reduce_pitch),
            },
            FieldInfoData {
                name: "MaxReducedLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, max_reduced_length),
            },
            FieldInfoData {
                name: "CollisionWidthPadding",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, collision_width_padding),
            },
            FieldInfoData {
                name: "CollisionBlendIn",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, collision_blend_in),
            },
            FieldInfoData {
                name: "CollisionBlendOut",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, collision_blend_out),
            },
            FieldInfoData {
                name: "FreeTransformBlendValue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, free_transform_blend_value),
            },
            FieldInfoData {
                name: "NearPlane",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, near_plane),
            },
            FieldInfoData {
                name: "FarPlane",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, far_plane),
            },
            FieldInfoData {
                name: "Fov",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, fov),
            },
            FieldInfoData {
                name: "AspectRatio",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, aspect_ratio),
            },
            FieldInfoData {
                name: "ReduceArmLengthLookingUp",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, reduce_arm_length_looking_up),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SOLDIERTHIRDPERSONCAMERASIMSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SoldierThirdPersonCameraSimState {
    fn type_info() -> &'static TypeInfo {
        SOLDIERTHIRDPERSONCAMERASIMSTATE_TYPE_INFO
    }
}


pub const SOLDIERTHIRDPERSONCAMERASIMSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoldierThirdPersonCameraSimState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("SoldierThirdPersonCameraSimState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SoldierFirstPersonCameraRenderState {
    pub yaw: f32,
    pub pitch: f32,
}

pub const SOLDIERFIRSTPERSONCAMERARENDERSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoldierFirstPersonCameraRenderState",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Yaw",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SoldierFirstPersonCameraRenderState, yaw),
            },
            FieldInfoData {
                name: "Pitch",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SoldierFirstPersonCameraRenderState, pitch),
            },
        ],
    }),
    array_type: Some(SOLDIERFIRSTPERSONCAMERARENDERSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for SoldierFirstPersonCameraRenderState {
    fn type_info() -> &'static TypeInfo {
        SOLDIERFIRSTPERSONCAMERARENDERSTATE_TYPE_INFO
    }
}


pub const SOLDIERFIRSTPERSONCAMERARENDERSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoldierFirstPersonCameraRenderState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("SoldierFirstPersonCameraRenderState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SoldierFirstPersonCameraSimState {
    pub camera_bone_local_transform: super::core::LinearTransform,
    pub camera_bone_transform_relative_to_trajectory: super::core::LinearTransform,
    pub roll_transform: super::core::LinearTransform,
    pub entity_space_local_transform: super::core::LinearTransform,
    pub procedural_transform: super::core::LinearTransform,
    pub shake_transform: super::core::LinearTransform,
    pub spine_x_relative_to_camera: super::core::LinearTransform,
    pub weapon_sway_transform: super::core::LinearTransform,
    pub soldier_world_position: super::core::Vec3,
    pub local_eye_position: super::core::Vec3,
    pub aiming: AimingHandle,
    pub soldier_transform_space: super::state_stream::TransformSpaceHandle,
    pub spine_x_bone_index: u32,
    pub trajectory_bone_index: u32,
    pub sim_yaw: f32,
    pub sim_pitch: f32,
    pub spine_x_factor: f32,
    pub animated_camera_factor: f32,
    pub animated_camera_start_pitch: f32,
    pub prevent_ground_clipping_distance: f32,
    pub has_valid_animation_transforms: bool,
    pub use_local_eye_position1p: bool,
    pub is_animated_camera: bool,
    pub has_entity_space: bool,
    pub field_flag_changed0: u32,
}

pub const SOLDIERFIRSTPERSONCAMERASIMSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoldierFirstPersonCameraSimState",
    flags: MemberInfoFlags::new(73),
    module: "Soldier",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "CameraBoneLocalTransform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, camera_bone_local_transform),
            },
            FieldInfoData {
                name: "CameraBoneTransformRelativeToTrajectory",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, camera_bone_transform_relative_to_trajectory),
            },
            FieldInfoData {
                name: "RollTransform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, roll_transform),
            },
            FieldInfoData {
                name: "EntitySpaceLocalTransform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, entity_space_local_transform),
            },
            FieldInfoData {
                name: "ProceduralTransform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, procedural_transform),
            },
            FieldInfoData {
                name: "ShakeTransform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, shake_transform),
            },
            FieldInfoData {
                name: "SpineXRelativeToCamera",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, spine_x_relative_to_camera),
            },
            FieldInfoData {
                name: "WeaponSwayTransform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, weapon_sway_transform),
            },
            FieldInfoData {
                name: "SoldierWorldPosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, soldier_world_position),
            },
            FieldInfoData {
                name: "LocalEyePosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, local_eye_position),
            },
            FieldInfoData {
                name: "Aiming",
                flags: MemberInfoFlags::new(0),
                field_type: AIMINGHANDLE_TYPE_INFO,
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, aiming),
            },
            FieldInfoData {
                name: "SoldierTransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: TRANSFORMSPACEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, soldier_transform_space),
            },
            FieldInfoData {
                name: "SpineXBoneIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, spine_x_bone_index),
            },
            FieldInfoData {
                name: "TrajectoryBoneIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, trajectory_bone_index),
            },
            FieldInfoData {
                name: "SimYaw",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, sim_yaw),
            },
            FieldInfoData {
                name: "SimPitch",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, sim_pitch),
            },
            FieldInfoData {
                name: "SpineXFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, spine_x_factor),
            },
            FieldInfoData {
                name: "AnimatedCameraFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, animated_camera_factor),
            },
            FieldInfoData {
                name: "AnimatedCameraStartPitch",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, animated_camera_start_pitch),
            },
            FieldInfoData {
                name: "PreventGroundClippingDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, prevent_ground_clipping_distance),
            },
            FieldInfoData {
                name: "HasValidAnimationTransforms",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, has_valid_animation_transforms),
            },
            FieldInfoData {
                name: "UseLocalEyePosition1p",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, use_local_eye_position1p),
            },
            FieldInfoData {
                name: "IsAnimatedCamera",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, is_animated_camera),
            },
            FieldInfoData {
                name: "HasEntitySpace",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, has_entity_space),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SOLDIERFIRSTPERSONCAMERASIMSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SoldierFirstPersonCameraSimState {
    fn type_info() -> &'static TypeInfo {
        SOLDIERFIRSTPERSONCAMERASIMSTATE_TYPE_INFO
    }
}


pub const SOLDIERFIRSTPERSONCAMERASIMSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoldierFirstPersonCameraSimState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("SoldierFirstPersonCameraSimState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SoldierServerPlayerExtent {
}

pub const SOLDIERSERVERPLAYEREXTENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoldierServerPlayerExtent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMEPLAYERINTERNALEXTENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SOLDIERSERVERPLAYEREXTENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SoldierServerPlayerExtent {
    fn type_info() -> &'static TypeInfo {
        SOLDIERSERVERPLAYEREXTENT_TYPE_INFO
    }
}


pub const SOLDIERSERVERPLAYEREXTENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoldierServerPlayerExtent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("SoldierServerPlayerExtent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerLookAtTriggerEntity {
}

pub const SERVERLOOKATTRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLookAtTriggerEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERCHARACTERLOOKATTRIGGERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERLOOKATTRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerLookAtTriggerEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERLOOKATTRIGGERENTITY_TYPE_INFO
    }
}


pub const SERVERLOOKATTRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLookAtTriggerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerLookAtTriggerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCharacterStateTriggerEntity {
}

pub const SERVERCHARACTERSTATETRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterStateTriggerEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERTRIGGERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERCHARACTERSTATETRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCharacterStateTriggerEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERCHARACTERSTATETRIGGERENTITY_TYPE_INFO
    }
}


pub const SERVERCHARACTERSTATETRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterStateTriggerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerCharacterStateTriggerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerWeaponStateEntity {
}

pub const SERVERWEAPONSTATEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponStateEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERWEAPONSTATEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerWeaponStateEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERWEAPONSTATEENTITY_TYPE_INFO
    }
}


pub const SERVERWEAPONSTATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponStateEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerWeaponStateEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerStateEventGateEntity {
}

pub const SERVERSTATEEVENTGATEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStateEventGateEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSTATEEVENTGATEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerStateEventGateEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERSTATEEVENTGATEENTITY_TYPE_INFO
    }
}


pub const SERVERSTATEEVENTGATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStateEventGateEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerStateEventGateEntity-Array"),
    array_type: None,
    alignment: 8,
};


