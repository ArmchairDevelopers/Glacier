use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct ClientWeaponStateEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientWeaponStateEntityTrait: super::entity::EntityTrait {
}

impl ClientWeaponStateEntityTrait for ClientWeaponStateEntity {
}

impl super::entity::EntityTrait for ClientWeaponStateEntity {
}

impl super::entity::EntityBusPeerTrait for ClientWeaponStateEntity {
}

pub static CLIENTWEAPONSTATEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponStateEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientWeaponStateEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWEAPONSTATEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWeaponStateEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTWEAPONSTATEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTWEAPONSTATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponStateEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientWeaponStateEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientPlayerLookAtEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientPlayerLookAtEntityTrait: super::entity::EntityTrait {
}

impl ClientPlayerLookAtEntityTrait for ClientPlayerLookAtEntity {
}

impl super::entity::EntityTrait for ClientPlayerLookAtEntity {
}

impl super::entity::EntityBusPeerTrait for ClientPlayerLookAtEntity {
}

pub static CLIENTPLAYERLOOKATENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerLookAtEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPlayerLookAtEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPLAYERLOOKATENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPlayerLookAtEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPLAYERLOOKATENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTPLAYERLOOKATENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerLookAtEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientPlayerLookAtEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientTripwireEntity {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponentEntity,
}

pub trait ClientTripwireEntityTrait: super::gameplay_client_server::ClientGameComponentEntityTrait {
}

impl ClientTripwireEntityTrait for ClientTripwireEntity {
}

impl super::gameplay_client_server::ClientGameComponentEntityTrait for ClientTripwireEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ClientTripwireEntity {
}

impl super::entity::ComponentEntityTrait for ClientTripwireEntity {
}

impl super::entity::SpatialEntityTrait for ClientTripwireEntity {
}

impl super::entity::EntityTrait for ClientTripwireEntity {
}

impl super::entity::EntityBusPeerTrait for ClientTripwireEntity {
}

pub static CLIENTTRIPWIREENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTripwireEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENTENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientTripwireEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTTRIPWIREENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientTripwireEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTTRIPWIREENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTTRIPWIREENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTripwireEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientTripwireEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientTriggerMoveEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientTriggerMoveEntityTrait: super::entity::EntityTrait {
}

impl ClientTriggerMoveEntityTrait for ClientTriggerMoveEntity {
}

impl super::entity::EntityTrait for ClientTriggerMoveEntity {
}

impl super::entity::EntityBusPeerTrait for ClientTriggerMoveEntity {
}

pub static CLIENTTRIGGERMOVEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTriggerMoveEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientTriggerMoveEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTTRIGGERMOVEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientTriggerMoveEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTTRIGGERMOVEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTTRIGGERMOVEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTriggerMoveEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientTriggerMoveEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientSoldierBreathControlComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientSoldierBreathControlComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientSoldierBreathControlComponentTrait for ClientSoldierBreathControlComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientSoldierBreathControlComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientSoldierBreathControlComponent {
}

impl super::entity::ComponentTrait for ClientSoldierBreathControlComponent {
}

impl super::entity::EntityBusPeerTrait for ClientSoldierBreathControlComponent {
}

pub static CLIENTSOLDIERBREATHCONTROLCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierBreathControlComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSoldierBreathControlComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSOLDIERBREATHCONTROLCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSoldierBreathControlComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSOLDIERBREATHCONTROLCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTSOLDIERBREATHCONTROLCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierBreathControlComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierBreathControlComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientSoldierBodyComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientSoldierBodyComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientSoldierBodyComponentTrait for ClientSoldierBodyComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientSoldierBodyComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientSoldierBodyComponent {
}

impl super::entity::ComponentTrait for ClientSoldierBodyComponent {
}

impl super::entity::EntityBusPeerTrait for ClientSoldierBodyComponent {
}

pub static CLIENTSOLDIERBODYCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierBodyComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSoldierBodyComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSOLDIERBODYCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSoldierBodyComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSOLDIERBODYCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTSOLDIERBODYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierBodyComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierBodyComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientPickupPhysicsComponent {
    pub _glacier_base: super::physics::PhysicsComponent,
}

pub trait ClientPickupPhysicsComponentTrait: super::physics::PhysicsComponentTrait {
}

impl ClientPickupPhysicsComponentTrait for ClientPickupPhysicsComponent {
}

impl super::physics::PhysicsComponentTrait for ClientPickupPhysicsComponent {
}

impl super::entity::ComponentTrait for ClientPickupPhysicsComponent {
}

impl super::entity::EntityBusPeerTrait for ClientPickupPhysicsComponent {
}

pub static CLIENTPICKUPPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPickupPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::PHYSICSCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPickupPhysicsComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPICKUPPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPickupPhysicsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPICKUPPHYSICSCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTPICKUPPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPickupPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientPickupPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientPhantomComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientPhantomComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientPhantomComponentTrait for ClientPhantomComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientPhantomComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientPhantomComponent {
}

impl super::entity::ComponentTrait for ClientPhantomComponent {
}

impl super::entity::EntityBusPeerTrait for ClientPhantomComponent {
}

pub static CLIENTPHANTOMCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPhantomComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPhantomComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPHANTOMCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPhantomComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPHANTOMCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTPHANTOMCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPhantomComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientPhantomComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientMovementComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientMovementComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientMovementComponentTrait for ClientMovementComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientMovementComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientMovementComponent {
}

impl super::entity::ComponentTrait for ClientMovementComponent {
}

impl super::entity::EntityBusPeerTrait for ClientMovementComponent {
}

pub static CLIENTMOVEMENTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMovementComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientMovementComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTMOVEMENTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientMovementComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTMOVEMENTCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTMOVEMENTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMovementComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientMovementComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientHitReactionComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientHitReactionComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientHitReactionComponentTrait for ClientHitReactionComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientHitReactionComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientHitReactionComponent {
}

impl super::entity::ComponentTrait for ClientHitReactionComponent {
}

impl super::entity::EntityBusPeerTrait for ClientHitReactionComponent {
}

pub static CLIENTHITREACTIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientHitReactionComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientHitReactionComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTHITREACTIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientHitReactionComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTHITREACTIONCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTHITREACTIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientHitReactionComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientHitReactionComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientFaceposerComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientFaceposerComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientFaceposerComponentTrait for ClientFaceposerComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientFaceposerComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientFaceposerComponent {
}

impl super::entity::ComponentTrait for ClientFaceposerComponent {
}

impl super::entity::EntityBusPeerTrait for ClientFaceposerComponent {
}

pub static CLIENTFACEPOSERCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientFaceposerComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientFaceposerComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTFACEPOSERCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientFaceposerComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTFACEPOSERCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTFACEPOSERCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientFaceposerComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientFaceposerComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientBoneCollisionComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientBoneCollisionComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientBoneCollisionComponentTrait for ClientBoneCollisionComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientBoneCollisionComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientBoneCollisionComponent {
}

impl super::entity::ComponentTrait for ClientBoneCollisionComponent {
}

impl super::entity::EntityBusPeerTrait for ClientBoneCollisionComponent {
}

pub static CLIENTBONECOLLISIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBoneCollisionComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientBoneCollisionComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTBONECOLLISIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientBoneCollisionComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTBONECOLLISIONCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTBONECOLLISIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBoneCollisionComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientBoneCollisionComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientBlockAimAssistComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientBlockAimAssistComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientBlockAimAssistComponentTrait for ClientBlockAimAssistComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientBlockAimAssistComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientBlockAimAssistComponent {
}

impl super::entity::ComponentTrait for ClientBlockAimAssistComponent {
}

impl super::entity::EntityBusPeerTrait for ClientBlockAimAssistComponent {
}

pub static CLIENTBLOCKAIMASSISTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBlockAimAssistComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientBlockAimAssistComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTBLOCKAIMASSISTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientBlockAimAssistComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTBLOCKAIMASSISTCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTBLOCKAIMASSISTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBlockAimAssistComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientBlockAimAssistComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientEntryAimAssistTargetOptionsComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientEntryAimAssistTargetOptionsComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientEntryAimAssistTargetOptionsComponentTrait for ClientEntryAimAssistTargetOptionsComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientEntryAimAssistTargetOptionsComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientEntryAimAssistTargetOptionsComponent {
}

impl super::entity::ComponentTrait for ClientEntryAimAssistTargetOptionsComponent {
}

impl super::entity::EntityBusPeerTrait for ClientEntryAimAssistTargetOptionsComponent {
}

pub static CLIENTENTRYAIMASSISTTARGETOPTIONSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEntryAimAssistTargetOptionsComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientEntryAimAssistTargetOptionsComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTENTRYAIMASSISTTARGETOPTIONSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientEntryAimAssistTargetOptionsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTENTRYAIMASSISTTARGETOPTIONSCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTENTRYAIMASSISTTARGETOPTIONSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEntryAimAssistTargetOptionsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientEntryAimAssistTargetOptionsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientAimAssistNodeSnapPointComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientAimAssistNodeSnapPointComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientAimAssistNodeSnapPointComponentTrait for ClientAimAssistNodeSnapPointComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientAimAssistNodeSnapPointComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientAimAssistNodeSnapPointComponent {
}

impl super::entity::ComponentTrait for ClientAimAssistNodeSnapPointComponent {
}

impl super::entity::EntityBusPeerTrait for ClientAimAssistNodeSnapPointComponent {
}

pub static CLIENTAIMASSISTNODESNAPPOINTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAimAssistNodeSnapPointComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAimAssistNodeSnapPointComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTAIMASSISTNODESNAPPOINTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAimAssistNodeSnapPointComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTAIMASSISTNODESNAPPOINTCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTAIMASSISTNODESNAPPOINTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAimAssistNodeSnapPointComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientAimAssistNodeSnapPointComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientAimAssistNodeComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientAimAssistNodeComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientAimAssistNodeComponentTrait for ClientAimAssistNodeComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientAimAssistNodeComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientAimAssistNodeComponent {
}

impl super::entity::ComponentTrait for ClientAimAssistNodeComponent {
}

impl super::entity::EntityBusPeerTrait for ClientAimAssistNodeComponent {
}

pub static CLIENTAIMASSISTNODECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAimAssistNodeComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAimAssistNodeComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTAIMASSISTNODECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAimAssistNodeComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTAIMASSISTNODECOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTAIMASSISTNODECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAimAssistNodeComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientAimAssistNodeComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SoldierCamera {
    pub _glacier_base: super::gameplay_sim::Camera,
}

pub trait SoldierCameraTrait: super::gameplay_sim::CameraTrait {
}

impl SoldierCameraTrait for SoldierCamera {
}

impl super::gameplay_sim::CameraTrait for SoldierCamera {
}

pub static SOLDIERCAMERA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoldierCamera",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_sim::CAMERA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SoldierCamera as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SOLDIERCAMERA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SoldierCamera {
    fn type_info(&self) -> &'static TypeInfo {
        SOLDIERCAMERA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SOLDIERCAMERA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoldierCamera-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("SoldierCamera"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientWeaponLagEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientWeaponLagEntityTrait: super::entity::EntityTrait {
}

impl ClientWeaponLagEntityTrait for ClientWeaponLagEntity {
}

impl super::entity::EntityTrait for ClientWeaponLagEntity {
}

impl super::entity::EntityBusPeerTrait for ClientWeaponLagEntity {
}

pub static CLIENTWEAPONLAGENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponLagEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientWeaponLagEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWEAPONLAGENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWeaponLagEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTWEAPONLAGENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTWEAPONLAGENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponLagEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientWeaponLagEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientCoverPeekEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientCoverPeekEntityTrait: super::entity::EntityTrait {
}

impl ClientCoverPeekEntityTrait for ClientCoverPeekEntity {
}

impl super::entity::EntityTrait for ClientCoverPeekEntity {
}

impl super::entity::EntityBusPeerTrait for ClientCoverPeekEntity {
}

pub static CLIENTCOVERPEEKENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCoverPeekEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCoverPeekEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCOVERPEEKENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCoverPeekEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCOVERPEEKENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTCOVERPEEKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCoverPeekEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientCoverPeekEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientOcclutionQueryComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientOcclutionQueryComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientOcclutionQueryComponentTrait for ClientOcclutionQueryComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientOcclutionQueryComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientOcclutionQueryComponent {
}

impl super::entity::ComponentTrait for ClientOcclutionQueryComponent {
}

impl super::entity::EntityBusPeerTrait for ClientOcclutionQueryComponent {
}

pub static CLIENTOCCLUTIONQUERYCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientOcclutionQueryComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientOcclutionQueryComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTOCCLUTIONQUERYCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientOcclutionQueryComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTOCCLUTIONQUERYCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTOCCLUTIONQUERYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientOcclutionQueryComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientOcclutionQueryComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientVoiceOverObjectReaderWatcherEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientVoiceOverObjectReaderWatcherEntityTrait: super::entity::EntityTrait {
}

impl ClientVoiceOverObjectReaderWatcherEntityTrait for ClientVoiceOverObjectReaderWatcherEntity {
}

impl super::entity::EntityTrait for ClientVoiceOverObjectReaderWatcherEntity {
}

impl super::entity::EntityBusPeerTrait for ClientVoiceOverObjectReaderWatcherEntity {
}

pub static CLIENTVOICEOVEROBJECTREADERWATCHERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverObjectReaderWatcherEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientVoiceOverObjectReaderWatcherEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVOICEOVEROBJECTREADERWATCHERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientVoiceOverObjectReaderWatcherEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTVOICEOVEROBJECTREADERWATCHERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTVOICEOVEROBJECTREADERWATCHERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverObjectReaderWatcherEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientVoiceOverObjectReaderWatcherEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientVoiceOverObjectReaderEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientVoiceOverObjectReaderEntityTrait: super::entity::EntityTrait {
}

impl ClientVoiceOverObjectReaderEntityTrait for ClientVoiceOverObjectReaderEntity {
}

impl super::entity::EntityTrait for ClientVoiceOverObjectReaderEntity {
}

impl super::entity::EntityBusPeerTrait for ClientVoiceOverObjectReaderEntity {
}

pub static CLIENTVOICEOVEROBJECTREADERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverObjectReaderEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientVoiceOverObjectReaderEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVOICEOVEROBJECTREADERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientVoiceOverObjectReaderEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTVOICEOVEROBJECTREADERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTVOICEOVEROBJECTREADERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverObjectReaderEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientVoiceOverObjectReaderEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerSoldierWeaponsComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerSoldierWeaponsComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerSoldierWeaponsComponentTrait for ServerSoldierWeaponsComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerSoldierWeaponsComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerSoldierWeaponsComponent {
}

impl super::entity::ComponentTrait for ServerSoldierWeaponsComponent {
}

impl super::entity::EntityBusPeerTrait for ServerSoldierWeaponsComponent {
}

pub static SERVERSOLDIERWEAPONSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierWeaponsComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierWeaponsComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSOLDIERWEAPONSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSoldierWeaponsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSOLDIERWEAPONSCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERSOLDIERWEAPONSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierWeaponsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerSoldierWeaponsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerSoldierPhysicsComponent {
    pub _glacier_base: super::game_server::ServerCharacterMasterPhysicsComponent,
}

pub trait ServerSoldierPhysicsComponentTrait: super::game_server::ServerCharacterMasterPhysicsComponentTrait {
}

impl ServerSoldierPhysicsComponentTrait for ServerSoldierPhysicsComponent {
}

impl super::game_server::ServerCharacterMasterPhysicsComponentTrait for ServerSoldierPhysicsComponent {
}

impl super::physics::CharacterPhysicsComponentTrait for ServerSoldierPhysicsComponent {
}

impl super::physics::PhysicsComponentTrait for ServerSoldierPhysicsComponent {
}

impl super::entity::ComponentTrait for ServerSoldierPhysicsComponent {
}

impl super::entity::EntityBusPeerTrait for ServerSoldierPhysicsComponent {
}

pub static SERVERSOLDIERPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_server::SERVERCHARACTERMASTERPHYSICSCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierPhysicsComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSOLDIERPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSoldierPhysicsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSOLDIERPHYSICSCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERSOLDIERPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerSoldierPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerSoldierHealthComponent {
    pub _glacier_base: super::game_server::ServerCharacterHealthComponent,
}

pub trait ServerSoldierHealthComponentTrait: super::game_server::ServerCharacterHealthComponentTrait {
}

impl ServerSoldierHealthComponentTrait for ServerSoldierHealthComponent {
}

impl super::game_server::ServerCharacterHealthComponentTrait for ServerSoldierHealthComponent {
}

impl super::game_server::ServerControllableHealthComponentTrait for ServerSoldierHealthComponent {
}

impl super::game_server::ServerGameHealthComponentTrait for ServerSoldierHealthComponent {
}

impl super::gameplay_sim::HealthComponentTrait for ServerSoldierHealthComponent {
}

impl super::entity::ComponentTrait for ServerSoldierHealthComponent {
}

impl super::entity::EntityBusPeerTrait for ServerSoldierHealthComponent {
}

pub static SERVERSOLDIERHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierHealthComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_server::SERVERCHARACTERHEALTHCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierHealthComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSOLDIERHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSoldierHealthComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSOLDIERHEALTHCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERSOLDIERHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierHealthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerSoldierHealthComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerSoldierCustomizationComponent {
    pub _glacier_base: super::game_server::ServerCharacterCustomizationComponent,
}

pub trait ServerSoldierCustomizationComponentTrait: super::game_server::ServerCharacterCustomizationComponentTrait {
}

impl ServerSoldierCustomizationComponentTrait for ServerSoldierCustomizationComponent {
}

impl super::game_server::ServerCharacterCustomizationComponentTrait for ServerSoldierCustomizationComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerSoldierCustomizationComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerSoldierCustomizationComponent {
}

impl super::entity::ComponentTrait for ServerSoldierCustomizationComponent {
}

impl super::entity::EntityBusPeerTrait for ServerSoldierCustomizationComponent {
}

pub static SERVERSOLDIERCUSTOMIZATIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierCustomizationComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_server::SERVERCHARACTERCUSTOMIZATIONCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierCustomizationComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSOLDIERCUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSoldierCustomizationComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSOLDIERCUSTOMIZATIONCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERSOLDIERCUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierCustomizationComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerSoldierCustomizationComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerSoldierCameraComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerSoldierCameraComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerSoldierCameraComponentTrait for ServerSoldierCameraComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerSoldierCameraComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerSoldierCameraComponent {
}

impl super::entity::ComponentTrait for ServerSoldierCameraComponent {
}

impl super::entity::EntityBusPeerTrait for ServerSoldierCameraComponent {
}

pub static SERVERSOLDIERCAMERACOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierCameraComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierCameraComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSOLDIERCAMERACOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSoldierCameraComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSOLDIERCAMERACOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERSOLDIERCAMERACOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierCameraComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerSoldierCameraComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerSoldierBodyComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerSoldierBodyComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerSoldierBodyComponentTrait for ServerSoldierBodyComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerSoldierBodyComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerSoldierBodyComponent {
}

impl super::entity::ComponentTrait for ServerSoldierBodyComponent {
}

impl super::entity::EntityBusPeerTrait for ServerSoldierBodyComponent {
}

pub static SERVERSOLDIERBODYCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierBodyComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierBodyComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSOLDIERBODYCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSoldierBodyComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSOLDIERBODYCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERSOLDIERBODYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierBodyComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerSoldierBodyComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerPickupPhysicsComponent {
    pub _glacier_base: super::physics::PhysicsComponent,
}

pub trait ServerPickupPhysicsComponentTrait: super::physics::PhysicsComponentTrait {
}

impl ServerPickupPhysicsComponentTrait for ServerPickupPhysicsComponent {
}

impl super::physics::PhysicsComponentTrait for ServerPickupPhysicsComponent {
}

impl super::entity::ComponentTrait for ServerPickupPhysicsComponent {
}

impl super::entity::EntityBusPeerTrait for ServerPickupPhysicsComponent {
}

pub static SERVERPICKUPPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPickupPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::PHYSICSCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPickupPhysicsComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERPICKUPPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPickupPhysicsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPICKUPPHYSICSCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERPICKUPPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPickupPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerPickupPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerMovementComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerMovementComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerMovementComponentTrait for ServerMovementComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerMovementComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerMovementComponent {
}

impl super::entity::ComponentTrait for ServerMovementComponent {
}

impl super::entity::EntityBusPeerTrait for ServerMovementComponent {
}

pub static SERVERMOVEMENTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMovementComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerMovementComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERMOVEMENTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerMovementComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERMOVEMENTCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERMOVEMENTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMovementComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerMovementComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerHitReactionComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerHitReactionComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerHitReactionComponentTrait for ServerHitReactionComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerHitReactionComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerHitReactionComponent {
}

impl super::entity::ComponentTrait for ServerHitReactionComponent {
}

impl super::entity::EntityBusPeerTrait for ServerHitReactionComponent {
}

pub static SERVERHITREACTIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerHitReactionComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerHitReactionComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERHITREACTIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerHitReactionComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERHITREACTIONCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERHITREACTIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerHitReactionComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerHitReactionComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerBoneCollisionComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerBoneCollisionComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerBoneCollisionComponentTrait for ServerBoneCollisionComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerBoneCollisionComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerBoneCollisionComponent {
}

impl super::entity::ComponentTrait for ServerBoneCollisionComponent {
}

impl super::entity::EntityBusPeerTrait for ServerBoneCollisionComponent {
}

pub static SERVERBONECOLLISIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBoneCollisionComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerBoneCollisionComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERBONECOLLISIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerBoneCollisionComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERBONECOLLISIONCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERBONECOLLISIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBoneCollisionComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerBoneCollisionComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientAimingScaleDataProviderEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientAimingScaleDataProviderEntityTrait: super::entity::EntityTrait {
}

impl ClientAimingScaleDataProviderEntityTrait for ClientAimingScaleDataProviderEntity {
}

impl super::entity::EntityTrait for ClientAimingScaleDataProviderEntity {
}

impl super::entity::EntityBusPeerTrait for ClientAimingScaleDataProviderEntity {
}

pub static CLIENTAIMINGSCALEDATAPROVIDERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAimingScaleDataProviderEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAimingScaleDataProviderEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTAIMINGSCALEDATAPROVIDERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAimingScaleDataProviderEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTAIMINGSCALEDATAPROVIDERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTAIMINGSCALEDATAPROVIDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAimingScaleDataProviderEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientAimingScaleDataProviderEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientAimingAngularSpeedConstraintDataProviderEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientAimingAngularSpeedConstraintDataProviderEntityTrait: super::entity::EntityTrait {
}

impl ClientAimingAngularSpeedConstraintDataProviderEntityTrait for ClientAimingAngularSpeedConstraintDataProviderEntity {
}

impl super::entity::EntityTrait for ClientAimingAngularSpeedConstraintDataProviderEntity {
}

impl super::entity::EntityBusPeerTrait for ClientAimingAngularSpeedConstraintDataProviderEntity {
}

pub static CLIENTAIMINGANGULARSPEEDCONSTRAINTDATAPROVIDERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAimingAngularSpeedConstraintDataProviderEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAimingAngularSpeedConstraintDataProviderEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTAIMINGANGULARSPEEDCONSTRAINTDATAPROVIDERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAimingAngularSpeedConstraintDataProviderEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTAIMINGANGULARSPEEDCONSTRAINTDATAPROVIDERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTAIMINGANGULARSPEEDCONSTRAINTDATAPROVIDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAimingAngularSpeedConstraintDataProviderEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientAimingAngularSpeedConstraintDataProviderEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientZeroingWeapon {
    pub _glacier_base: super::weapon::ClientWeapon,
}

pub trait ClientZeroingWeaponTrait: super::weapon::ClientWeaponTrait {
}

impl ClientZeroingWeaponTrait for ClientZeroingWeapon {
}

impl super::weapon::ClientWeaponTrait for ClientZeroingWeapon {
}

impl super::weapon::WeaponTrait for ClientZeroingWeapon {
}

impl super::game_common::ToolTrait for ClientZeroingWeapon {
}

pub static CLIENTZEROINGWEAPON_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientZeroingWeapon",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::weapon::CLIENTWEAPON_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientZeroingWeapon as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTZEROINGWEAPON_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientZeroingWeapon {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTZEROINGWEAPON_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTZEROINGWEAPON_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientZeroingWeapon-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientZeroingWeapon"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientSoldierWeaponSocketEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientSoldierWeaponSocketEntityTrait: super::entity::EntityTrait {
}

impl ClientSoldierWeaponSocketEntityTrait for ClientSoldierWeaponSocketEntity {
}

impl super::entity::EntityTrait for ClientSoldierWeaponSocketEntity {
}

impl super::entity::EntityBusPeerTrait for ClientSoldierWeaponSocketEntity {
}

pub static CLIENTSOLDIERWEAPONSOCKETENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierWeaponSocketEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSoldierWeaponSocketEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSOLDIERWEAPONSOCKETENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSoldierWeaponSocketEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSOLDIERWEAPONSOCKETENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTSOLDIERWEAPONSOCKETENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierWeaponSocketEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierWeaponSocketEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientSoldierWeaponSpawnInfo {
}

pub trait ClientSoldierWeaponSpawnInfoTrait: TypeObject {
}

impl ClientSoldierWeaponSpawnInfoTrait for ClientSoldierWeaponSpawnInfo {
}

pub static CLIENTSOLDIERWEAPONSPAWNINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierWeaponSpawnInfo",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSoldierWeaponSpawnInfo as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSOLDIERWEAPONSPAWNINFO_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSoldierWeaponSpawnInfo {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSOLDIERWEAPONSPAWNINFO_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTSOLDIERWEAPONSPAWNINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierWeaponSpawnInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierWeaponSpawnInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientSoldierWeapon {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponentEntity,
}

pub trait ClientSoldierWeaponTrait: super::gameplay_client_server::ClientGameComponentEntityTrait {
}

impl ClientSoldierWeaponTrait for ClientSoldierWeapon {
}

impl super::gameplay_client_server::ClientGameComponentEntityTrait for ClientSoldierWeapon {
}

impl super::gameplay_sim::GameComponentEntityTrait for ClientSoldierWeapon {
}

impl super::entity::ComponentEntityTrait for ClientSoldierWeapon {
}

impl super::entity::SpatialEntityTrait for ClientSoldierWeapon {
}

impl super::entity::EntityTrait for ClientSoldierWeapon {
}

impl super::entity::EntityBusPeerTrait for ClientSoldierWeapon {
}

pub static CLIENTSOLDIERWEAPON_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierWeapon",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENTENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSoldierWeapon as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSOLDIERWEAPON_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSoldierWeapon {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSOLDIERWEAPON_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTSOLDIERWEAPON_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierWeapon-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierWeapon"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientAnimationTurretRotationComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientAnimationTurretRotationComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientAnimationTurretRotationComponentTrait for ClientAnimationTurretRotationComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientAnimationTurretRotationComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientAnimationTurretRotationComponent {
}

impl super::entity::ComponentTrait for ClientAnimationTurretRotationComponent {
}

impl super::entity::EntityBusPeerTrait for ClientAnimationTurretRotationComponent {
}

pub static CLIENTANIMATIONTURRETROTATIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAnimationTurretRotationComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAnimationTurretRotationComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTANIMATIONTURRETROTATIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAnimationTurretRotationComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTANIMATIONTURRETROTATIONCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTANIMATIONTURRETROTATIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAnimationTurretRotationComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientAnimationTurretRotationComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientProxyGrenadeEntity {
    pub _glacier_base: super::weapon::ClientProxyProjectileEntity,
}

pub trait ClientProxyGrenadeEntityTrait: super::weapon::ClientProxyProjectileEntityTrait {
}

impl ClientProxyGrenadeEntityTrait for ClientProxyGrenadeEntity {
}

impl super::weapon::ClientProxyProjectileEntityTrait for ClientProxyGrenadeEntity {
}

impl super::weapon::ClientProjectileEntityTrait for ClientProxyGrenadeEntity {
}

impl super::gameplay_client_server::ClientPhysicsEntityTrait for ClientProxyGrenadeEntity {
}

impl super::gameplay_client_server::ClientGameComponentEntityTrait for ClientProxyGrenadeEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ClientProxyGrenadeEntity {
}

impl super::entity::ComponentEntityTrait for ClientProxyGrenadeEntity {
}

impl super::entity::SpatialEntityTrait for ClientProxyGrenadeEntity {
}

impl super::entity::EntityTrait for ClientProxyGrenadeEntity {
}

impl super::entity::EntityBusPeerTrait for ClientProxyGrenadeEntity {
}

pub static CLIENTPROXYGRENADEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientProxyGrenadeEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::weapon::CLIENTPROXYPROJECTILEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientProxyGrenadeEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPROXYGRENADEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientProxyGrenadeEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPROXYGRENADEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTPROXYGRENADEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientProxyGrenadeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientProxyGrenadeEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientProxyExplosionPackEntity {
    pub _glacier_base: super::weapon::ClientProxyProjectileEntity,
}

pub trait ClientProxyExplosionPackEntityTrait: super::weapon::ClientProxyProjectileEntityTrait {
}

impl ClientProxyExplosionPackEntityTrait for ClientProxyExplosionPackEntity {
}

impl super::weapon::ClientProxyProjectileEntityTrait for ClientProxyExplosionPackEntity {
}

impl super::weapon::ClientProjectileEntityTrait for ClientProxyExplosionPackEntity {
}

impl super::gameplay_client_server::ClientPhysicsEntityTrait for ClientProxyExplosionPackEntity {
}

impl super::gameplay_client_server::ClientGameComponentEntityTrait for ClientProxyExplosionPackEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ClientProxyExplosionPackEntity {
}

impl super::entity::ComponentEntityTrait for ClientProxyExplosionPackEntity {
}

impl super::entity::SpatialEntityTrait for ClientProxyExplosionPackEntity {
}

impl super::entity::EntityTrait for ClientProxyExplosionPackEntity {
}

impl super::entity::EntityBusPeerTrait for ClientProxyExplosionPackEntity {
}

pub static CLIENTPROXYEXPLOSIONPACKENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientProxyExplosionPackEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::weapon::CLIENTPROXYPROJECTILEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientProxyExplosionPackEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPROXYEXPLOSIONPACKENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientProxyExplosionPackEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPROXYEXPLOSIONPACKENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTPROXYEXPLOSIONPACKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientProxyExplosionPackEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientProxyExplosionPackEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientGrenadeEntity {
    pub _glacier_base: super::weapon::ClientGhostProjectileEntity,
}

pub trait ClientGrenadeEntityTrait: super::weapon::ClientGhostProjectileEntityTrait {
}

impl ClientGrenadeEntityTrait for ClientGrenadeEntity {
}

impl super::weapon::ClientGhostProjectileEntityTrait for ClientGrenadeEntity {
}

impl super::weapon::ClientProjectileEntityTrait for ClientGrenadeEntity {
}

impl super::gameplay_client_server::ClientPhysicsEntityTrait for ClientGrenadeEntity {
}

impl super::gameplay_client_server::ClientGameComponentEntityTrait for ClientGrenadeEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ClientGrenadeEntity {
}

impl super::entity::ComponentEntityTrait for ClientGrenadeEntity {
}

impl super::entity::SpatialEntityTrait for ClientGrenadeEntity {
}

impl super::entity::EntityTrait for ClientGrenadeEntity {
}

impl super::entity::EntityBusPeerTrait for ClientGrenadeEntity {
}

pub static CLIENTGRENADEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGrenadeEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::weapon::CLIENTGHOSTPROJECTILEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientGrenadeEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTGRENADEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientGrenadeEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTGRENADEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTGRENADEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGrenadeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientGrenadeEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientExplosionPackEntity {
    pub _glacier_base: super::weapon::ClientGhostProjectileEntity,
}

pub trait ClientExplosionPackEntityTrait: super::weapon::ClientGhostProjectileEntityTrait {
}

impl ClientExplosionPackEntityTrait for ClientExplosionPackEntity {
}

impl super::weapon::ClientGhostProjectileEntityTrait for ClientExplosionPackEntity {
}

impl super::weapon::ClientProjectileEntityTrait for ClientExplosionPackEntity {
}

impl super::gameplay_client_server::ClientPhysicsEntityTrait for ClientExplosionPackEntity {
}

impl super::gameplay_client_server::ClientGameComponentEntityTrait for ClientExplosionPackEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ClientExplosionPackEntity {
}

impl super::entity::ComponentEntityTrait for ClientExplosionPackEntity {
}

impl super::entity::SpatialEntityTrait for ClientExplosionPackEntity {
}

impl super::entity::EntityTrait for ClientExplosionPackEntity {
}

impl super::entity::EntityBusPeerTrait for ClientExplosionPackEntity {
}

pub static CLIENTEXPLOSIONPACKENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientExplosionPackEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::weapon::CLIENTGHOSTPROJECTILEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientExplosionPackEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTEXPLOSIONPACKENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientExplosionPackEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTEXPLOSIONPACKENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTEXPLOSIONPACKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientExplosionPackEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientExplosionPackEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientExplosionPackPhysicsComponent {
    pub _glacier_base: super::physics::PartPhysicsComponent,
}

pub trait ClientExplosionPackPhysicsComponentTrait: super::physics::PartPhysicsComponentTrait {
}

impl ClientExplosionPackPhysicsComponentTrait for ClientExplosionPackPhysicsComponent {
}

impl super::physics::PartPhysicsComponentTrait for ClientExplosionPackPhysicsComponent {
}

impl super::physics::PhysicsComponentTrait for ClientExplosionPackPhysicsComponent {
}

impl super::entity::ComponentTrait for ClientExplosionPackPhysicsComponent {
}

impl super::entity::EntityBusPeerTrait for ClientExplosionPackPhysicsComponent {
}

pub static CLIENTEXPLOSIONPACKPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientExplosionPackPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::PARTPHYSICSCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientExplosionPackPhysicsComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTEXPLOSIONPACKPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientExplosionPackPhysicsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTEXPLOSIONPACKPHYSICSCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTEXPLOSIONPACKPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientExplosionPackPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientExplosionPackPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DummyExplosionPackPhysicsComponent {
    pub _glacier_base: super::physics::PhysicsComponent,
}

pub trait DummyExplosionPackPhysicsComponentTrait: super::physics::PhysicsComponentTrait {
}

impl DummyExplosionPackPhysicsComponentTrait for DummyExplosionPackPhysicsComponent {
}

impl super::physics::PhysicsComponentTrait for DummyExplosionPackPhysicsComponent {
}

impl super::entity::ComponentTrait for DummyExplosionPackPhysicsComponent {
}

impl super::entity::EntityBusPeerTrait for DummyExplosionPackPhysicsComponent {
}

pub static DUMMYEXPLOSIONPACKPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DummyExplosionPackPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::PHYSICSCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DummyExplosionPackPhysicsComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DUMMYEXPLOSIONPACKPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DummyExplosionPackPhysicsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        DUMMYEXPLOSIONPACKPHYSICSCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DUMMYEXPLOSIONPACKPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DummyExplosionPackPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("DummyExplosionPackPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientSoldierEntryComponent {
    pub _glacier_base: super::game_client::ClientGameEntryComponent,
}

pub trait ClientSoldierEntryComponentTrait: super::game_client::ClientGameEntryComponentTrait {
}

impl ClientSoldierEntryComponentTrait for ClientSoldierEntryComponent {
}

impl super::game_client::ClientGameEntryComponentTrait for ClientSoldierEntryComponent {
}

impl super::gameplay_client_server::ClientEntryComponentTrait for ClientSoldierEntryComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientSoldierEntryComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientSoldierEntryComponent {
}

impl super::entity::ComponentTrait for ClientSoldierEntryComponent {
}

impl super::entity::EntityBusPeerTrait for ClientSoldierEntryComponent {
}

pub static CLIENTSOLDIERENTRYCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierEntryComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_client::CLIENTGAMEENTRYCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSoldierEntryComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSOLDIERENTRYCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSoldierEntryComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSOLDIERENTRYCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTSOLDIERENTRYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierEntryComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierEntryComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientWeaponZeroingComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientWeaponZeroingComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientWeaponZeroingComponentTrait for ClientWeaponZeroingComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientWeaponZeroingComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientWeaponZeroingComponent {
}

impl super::entity::ComponentTrait for ClientWeaponZeroingComponent {
}

impl super::entity::EntityBusPeerTrait for ClientWeaponZeroingComponent {
}

pub static CLIENTWEAPONZEROINGCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponZeroingComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientWeaponZeroingComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWEAPONZEROINGCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWeaponZeroingComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTWEAPONZEROINGCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTWEAPONZEROINGCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponZeroingComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientWeaponZeroingComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientSoldierWeaponsPreviewComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientSoldierWeaponsPreviewComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientSoldierWeaponsPreviewComponentTrait for ClientSoldierWeaponsPreviewComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientSoldierWeaponsPreviewComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientSoldierWeaponsPreviewComponent {
}

impl super::entity::ComponentTrait for ClientSoldierWeaponsPreviewComponent {
}

impl super::entity::EntityBusPeerTrait for ClientSoldierWeaponsPreviewComponent {
}

pub static CLIENTSOLDIERWEAPONSPREVIEWCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierWeaponsPreviewComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSoldierWeaponsPreviewComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSOLDIERWEAPONSPREVIEWCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSoldierWeaponsPreviewComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSOLDIERWEAPONSPREVIEWCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTSOLDIERWEAPONSPREVIEWCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierWeaponsPreviewComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierWeaponsPreviewComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientSoldierWeaponsComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientSoldierWeaponsComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientSoldierWeaponsComponentTrait for ClientSoldierWeaponsComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientSoldierWeaponsComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientSoldierWeaponsComponent {
}

impl super::entity::ComponentTrait for ClientSoldierWeaponsComponent {
}

impl super::entity::EntityBusPeerTrait for ClientSoldierWeaponsComponent {
}

pub static CLIENTSOLDIERWEAPONSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierWeaponsComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSoldierWeaponsComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSOLDIERWEAPONSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSoldierWeaponsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSOLDIERWEAPONSCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTSOLDIERWEAPONSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierWeaponsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierWeaponsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientSoldierPhysicsComponent {
    pub _glacier_base: super::game_client::ClientCharacterMasterPhysicsComponent,
}

pub trait ClientSoldierPhysicsComponentTrait: super::game_client::ClientCharacterMasterPhysicsComponentTrait {
}

impl ClientSoldierPhysicsComponentTrait for ClientSoldierPhysicsComponent {
}

impl super::game_client::ClientCharacterMasterPhysicsComponentTrait for ClientSoldierPhysicsComponent {
}

impl super::physics::CharacterPhysicsComponentTrait for ClientSoldierPhysicsComponent {
}

impl super::physics::PhysicsComponentTrait for ClientSoldierPhysicsComponent {
}

impl super::entity::ComponentTrait for ClientSoldierPhysicsComponent {
}

impl super::entity::EntityBusPeerTrait for ClientSoldierPhysicsComponent {
}

pub static CLIENTSOLDIERPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_client::CLIENTCHARACTERMASTERPHYSICSCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSoldierPhysicsComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSOLDIERPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSoldierPhysicsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSOLDIERPHYSICSCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTSOLDIERPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientSoldierHealthComponent {
    pub _glacier_base: super::game_client::ClientCharacterHealthComponent,
}

pub trait ClientSoldierHealthComponentTrait: super::game_client::ClientCharacterHealthComponentTrait {
}

impl ClientSoldierHealthComponentTrait for ClientSoldierHealthComponent {
}

impl super::game_client::ClientCharacterHealthComponentTrait for ClientSoldierHealthComponent {
}

impl super::gameplay_client_server::ClientControllableHealthComponentTrait for ClientSoldierHealthComponent {
}

impl super::gameplay_client_server::ClientGameHealthComponentTrait for ClientSoldierHealthComponent {
}

impl super::gameplay_sim::HealthComponentTrait for ClientSoldierHealthComponent {
}

impl super::entity::ComponentTrait for ClientSoldierHealthComponent {
}

impl super::entity::EntityBusPeerTrait for ClientSoldierHealthComponent {
}

pub static CLIENTSOLDIERHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierHealthComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_client::CLIENTCHARACTERHEALTHCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSoldierHealthComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSOLDIERHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSoldierHealthComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSOLDIERHEALTHCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTSOLDIERHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierHealthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierHealthComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientSoldierFootplantEffectComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientSoldierFootplantEffectComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientSoldierFootplantEffectComponentTrait for ClientSoldierFootplantEffectComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientSoldierFootplantEffectComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientSoldierFootplantEffectComponent {
}

impl super::entity::ComponentTrait for ClientSoldierFootplantEffectComponent {
}

impl super::entity::EntityBusPeerTrait for ClientSoldierFootplantEffectComponent {
}

pub static CLIENTSOLDIERFOOTPLANTEFFECTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierFootplantEffectComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSoldierFootplantEffectComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSOLDIERFOOTPLANTEFFECTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSoldierFootplantEffectComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSOLDIERFOOTPLANTEFFECTCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTSOLDIERFOOTPLANTEFFECTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierFootplantEffectComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierFootplantEffectComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientSoldierCustomizationComponent {
    pub _glacier_base: super::game_client::ClientCharacterCustomizationComponent,
}

pub trait ClientSoldierCustomizationComponentTrait: super::game_client::ClientCharacterCustomizationComponentTrait {
}

impl ClientSoldierCustomizationComponentTrait for ClientSoldierCustomizationComponent {
}

impl super::game_client::ClientCharacterCustomizationComponentTrait for ClientSoldierCustomizationComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientSoldierCustomizationComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientSoldierCustomizationComponent {
}

impl super::entity::ComponentTrait for ClientSoldierCustomizationComponent {
}

impl super::entity::EntityBusPeerTrait for ClientSoldierCustomizationComponent {
}

pub static CLIENTSOLDIERCUSTOMIZATIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierCustomizationComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_client::CLIENTCHARACTERCUSTOMIZATIONCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSoldierCustomizationComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSOLDIERCUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSoldierCustomizationComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSOLDIERCUSTOMIZATIONCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTSOLDIERCUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierCustomizationComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierCustomizationComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientSoldierCameraComponent {
    pub _glacier_base: super::game_client::ClientCharacterCameraComponent,
}

pub trait ClientSoldierCameraComponentTrait: super::game_client::ClientCharacterCameraComponentTrait {
}

impl ClientSoldierCameraComponentTrait for ClientSoldierCameraComponent {
}

impl super::game_client::ClientCharacterCameraComponentTrait for ClientSoldierCameraComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientSoldierCameraComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientSoldierCameraComponent {
}

impl super::entity::ComponentTrait for ClientSoldierCameraComponent {
}

impl super::entity::EntityBusPeerTrait for ClientSoldierCameraComponent {
}

pub static CLIENTSOLDIERCAMERACOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierCameraComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_client::CLIENTCHARACTERCAMERACOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSoldierCameraComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSOLDIERCAMERACOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSoldierCameraComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSOLDIERCAMERACOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTSOLDIERCAMERACOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierCameraComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierCameraComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientSoldierCameraCallback {
    pub _glacier_base: super::gameplay_sim::TargetCameraCallback,
}

pub trait ClientSoldierCameraCallbackTrait: super::gameplay_sim::TargetCameraCallbackTrait {
}

impl ClientSoldierCameraCallbackTrait for ClientSoldierCameraCallback {
}

impl super::gameplay_sim::TargetCameraCallbackTrait for ClientSoldierCameraCallback {
}

pub static CLIENTSOLDIERCAMERACALLBACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierCameraCallback",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_sim::TARGETCAMERACALLBACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSoldierCameraCallback as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSOLDIERCAMERACALLBACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSoldierCameraCallback {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSOLDIERCAMERACALLBACK_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTSOLDIERCAMERACALLBACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierCameraCallback-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierCameraCallback"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientSoldierEntity {
    pub _glacier_base: super::game_client::ClientCharacterEntity,
}

pub trait ClientSoldierEntityTrait: super::game_client::ClientCharacterEntityTrait {
}

impl ClientSoldierEntityTrait for ClientSoldierEntity {
}

impl super::game_client::ClientCharacterEntityTrait for ClientSoldierEntity {
}

impl super::gameplay_client_server::ClientControllableEntityTrait for ClientSoldierEntity {
}

impl super::gameplay_client_server::ClientPhysicsEntityTrait for ClientSoldierEntity {
}

impl super::gameplay_client_server::ClientGameComponentEntityTrait for ClientSoldierEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ClientSoldierEntity {
}

impl super::entity::ComponentEntityTrait for ClientSoldierEntity {
}

impl super::entity::SpatialEntityTrait for ClientSoldierEntity {
}

impl super::entity::EntityTrait for ClientSoldierEntity {
}

impl super::entity::EntityBusPeerTrait for ClientSoldierEntity {
}

pub static CLIENTSOLDIERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_client::CLIENTCHARACTERENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSoldierEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSOLDIERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSoldierEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSOLDIERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTSOLDIERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientSchematicsAimEntity {
    pub _glacier_base: ClientAimEntityBase,
}

pub trait ClientSchematicsAimEntityTrait: ClientAimEntityBaseTrait {
}

impl ClientSchematicsAimEntityTrait for ClientSchematicsAimEntity {
}

impl ClientAimEntityBaseTrait for ClientSchematicsAimEntity {
}

impl super::entity::EntityTrait for ClientSchematicsAimEntity {
}

impl super::entity::EntityBusPeerTrait for ClientSchematicsAimEntity {
}

pub static CLIENTSCHEMATICSAIMENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSchematicsAimEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTAIMENTITYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSchematicsAimEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSCHEMATICSAIMENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSchematicsAimEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSCHEMATICSAIMENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTSCHEMATICSAIMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSchematicsAimEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSchematicsAimEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientReplicatedAimEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientReplicatedAimEntityTrait: super::entity::EntityTrait {
}

impl ClientReplicatedAimEntityTrait for ClientReplicatedAimEntity {
}

impl super::entity::EntityTrait for ClientReplicatedAimEntity {
}

impl super::entity::EntityBusPeerTrait for ClientReplicatedAimEntity {
}

pub static CLIENTREPLICATEDAIMENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientReplicatedAimEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientReplicatedAimEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTREPLICATEDAIMENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientReplicatedAimEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTREPLICATEDAIMENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTREPLICATEDAIMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientReplicatedAimEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientReplicatedAimEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientPickupEntity {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponentEntity,
}

pub trait ClientPickupEntityTrait: super::gameplay_client_server::ClientGameComponentEntityTrait {
}

impl ClientPickupEntityTrait for ClientPickupEntity {
}

impl super::gameplay_client_server::ClientGameComponentEntityTrait for ClientPickupEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ClientPickupEntity {
}

impl super::entity::ComponentEntityTrait for ClientPickupEntity {
}

impl super::entity::SpatialEntityTrait for ClientPickupEntity {
}

impl super::entity::EntityTrait for ClientPickupEntity {
}

impl super::entity::EntityBusPeerTrait for ClientPickupEntity {
}

pub static CLIENTPICKUPENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPickupEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENTENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPickupEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPICKUPENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPickupEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPICKUPENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTPICKUPENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPickupEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientPickupEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientBlockAimAssistEntity {
    pub _glacier_base: super::entity::SpatialEntity,
}

pub trait ClientBlockAimAssistEntityTrait: super::entity::SpatialEntityTrait {
}

impl ClientBlockAimAssistEntityTrait for ClientBlockAimAssistEntity {
}

impl super::entity::SpatialEntityTrait for ClientBlockAimAssistEntity {
}

impl super::entity::EntityTrait for ClientBlockAimAssistEntity {
}

impl super::entity::EntityBusPeerTrait for ClientBlockAimAssistEntity {
}

pub static CLIENTBLOCKAIMASSISTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBlockAimAssistEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientBlockAimAssistEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTBLOCKAIMASSISTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientBlockAimAssistEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTBLOCKAIMASSISTENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTBLOCKAIMASSISTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBlockAimAssistEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientBlockAimAssistEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientAimingSimulationDataProviderEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientAimingSimulationDataProviderEntityTrait: super::entity::EntityTrait {
}

impl ClientAimingSimulationDataProviderEntityTrait for ClientAimingSimulationDataProviderEntity {
}

impl super::entity::EntityTrait for ClientAimingSimulationDataProviderEntity {
}

impl super::entity::EntityBusPeerTrait for ClientAimingSimulationDataProviderEntity {
}

pub static CLIENTAIMINGSIMULATIONDATAPROVIDERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAimingSimulationDataProviderEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAimingSimulationDataProviderEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTAIMINGSIMULATIONDATAPROVIDERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAimingSimulationDataProviderEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTAIMINGSIMULATIONDATAPROVIDERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTAIMINGSIMULATIONDATAPROVIDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAimingSimulationDataProviderEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientAimingSimulationDataProviderEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientAimEntityBase {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientAimEntityBaseTrait: super::entity::EntityTrait {
}

impl ClientAimEntityBaseTrait for ClientAimEntityBase {
}

impl super::entity::EntityTrait for ClientAimEntityBase {
}

impl super::entity::EntityBusPeerTrait for ClientAimEntityBase {
}

pub static CLIENTAIMENTITYBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAimEntityBase",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAimEntityBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTAIMENTITYBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAimEntityBase {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTAIMENTITYBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTAIMENTITYBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAimEntityBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientAimEntityBase"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientAimEntity {
    pub _glacier_base: ClientAimEntityBase,
}

pub trait ClientAimEntityTrait: ClientAimEntityBaseTrait {
}

impl ClientAimEntityTrait for ClientAimEntity {
}

impl ClientAimEntityBaseTrait for ClientAimEntity {
}

impl super::entity::EntityTrait for ClientAimEntity {
}

impl super::entity::EntityBusPeerTrait for ClientAimEntity {
}

pub static CLIENTAIMENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAimEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTAIMENTITYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAimEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTAIMENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAimEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTAIMENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTAIMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAimEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientAimEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerCustomizeSoldierEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerCustomizeSoldierEntityTrait: super::entity::EntityTrait {
}

impl ServerCustomizeSoldierEntityTrait for ServerCustomizeSoldierEntity {
}

impl super::entity::EntityTrait for ServerCustomizeSoldierEntity {
}

impl super::entity::EntityBusPeerTrait for ServerCustomizeSoldierEntity {
}

pub static SERVERCUSTOMIZESOLDIERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCustomizeSoldierEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCustomizeSoldierEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCUSTOMIZESOLDIERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCustomizeSoldierEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCUSTOMIZESOLDIERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERCUSTOMIZESOLDIERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCustomizeSoldierEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerCustomizeSoldierEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerTripwireEntity {
    pub _glacier_base: super::game_server::ServerGameComponentEntity,
}

pub trait ServerTripwireEntityTrait: super::game_server::ServerGameComponentEntityTrait {
}

impl ServerTripwireEntityTrait for ServerTripwireEntity {
}

impl super::game_server::ServerGameComponentEntityTrait for ServerTripwireEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ServerTripwireEntity {
}

impl super::entity::ComponentEntityTrait for ServerTripwireEntity {
}

impl super::entity::SpatialEntityTrait for ServerTripwireEntity {
}

impl super::entity::EntityTrait for ServerTripwireEntity {
}

impl super::entity::EntityBusPeerTrait for ServerTripwireEntity {
}

pub static SERVERTRIPWIREENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTripwireEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_server::SERVERGAMECOMPONENTENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerTripwireEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERTRIPWIREENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerTripwireEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERTRIPWIREENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERTRIPWIREENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTripwireEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerTripwireEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerSoldierEntity {
    pub _glacier_base: super::game_server::ServerCharacterEntity,
}

pub trait ServerSoldierEntityTrait: super::game_server::ServerCharacterEntityTrait {
}

impl ServerSoldierEntityTrait for ServerSoldierEntity {
}

impl super::game_server::ServerCharacterEntityTrait for ServerSoldierEntity {
}

impl super::game_server::ServerControllableEntityTrait for ServerSoldierEntity {
}

impl super::game_server::ServerPhysicsEntityTrait for ServerSoldierEntity {
}

impl super::game_server::ServerGameComponentEntityTrait for ServerSoldierEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ServerSoldierEntity {
}

impl super::entity::ComponentEntityTrait for ServerSoldierEntity {
}

impl super::entity::SpatialEntityTrait for ServerSoldierEntity {
}

impl super::entity::EntityTrait for ServerSoldierEntity {
}

impl super::entity::EntityBusPeerTrait for ServerSoldierEntity {
}

pub static SERVERSOLDIERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_server::SERVERCHARACTERENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSOLDIERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSoldierEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSOLDIERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERSOLDIERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerSoldierEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerReplicatedAimEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerReplicatedAimEntityTrait: super::entity::EntityTrait {
}

impl ServerReplicatedAimEntityTrait for ServerReplicatedAimEntity {
}

impl super::entity::EntityTrait for ServerReplicatedAimEntity {
}

impl super::entity::EntityBusPeerTrait for ServerReplicatedAimEntity {
}

pub static SERVERREPLICATEDAIMENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerReplicatedAimEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerReplicatedAimEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERREPLICATEDAIMENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerReplicatedAimEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERREPLICATEDAIMENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERREPLICATEDAIMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerReplicatedAimEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerReplicatedAimEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerPickupEntity {
    pub _glacier_base: super::game_server::ServerGameComponentEntity,
}

pub trait ServerPickupEntityTrait: super::game_server::ServerGameComponentEntityTrait {
}

impl ServerPickupEntityTrait for ServerPickupEntity {
}

impl super::game_server::ServerGameComponentEntityTrait for ServerPickupEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ServerPickupEntity {
}

impl super::entity::ComponentEntityTrait for ServerPickupEntity {
}

impl super::entity::SpatialEntityTrait for ServerPickupEntity {
}

impl super::entity::EntityTrait for ServerPickupEntity {
}

impl super::entity::EntityBusPeerTrait for ServerPickupEntity {
}

pub static SERVERPICKUPENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPickupEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_server::SERVERGAMECOMPONENTENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPickupEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERPICKUPENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerPickupEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPICKUPENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERPICKUPENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPickupEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerPickupEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerZeroingWeapon {
    pub _glacier_base: super::weapon::ServerWeapon,
}

pub trait ServerZeroingWeaponTrait: super::weapon::ServerWeaponTrait {
}

impl ServerZeroingWeaponTrait for ServerZeroingWeapon {
}

impl super::weapon::ServerWeaponTrait for ServerZeroingWeapon {
}

impl super::weapon::WeaponTrait for ServerZeroingWeapon {
}

impl super::game_common::ToolTrait for ServerZeroingWeapon {
}

pub static SERVERZEROINGWEAPON_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerZeroingWeapon",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::weapon::SERVERWEAPON_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerZeroingWeapon as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERZEROINGWEAPON_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerZeroingWeapon {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERZEROINGWEAPON_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERZEROINGWEAPON_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerZeroingWeapon-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerZeroingWeapon"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerSoldierWeaponSpawnInfo {
}

pub trait ServerSoldierWeaponSpawnInfoTrait: TypeObject {
}

impl ServerSoldierWeaponSpawnInfoTrait for ServerSoldierWeaponSpawnInfo {
}

pub static SERVERSOLDIERWEAPONSPAWNINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierWeaponSpawnInfo",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierWeaponSpawnInfo as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSOLDIERWEAPONSPAWNINFO_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSoldierWeaponSpawnInfo {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSOLDIERWEAPONSPAWNINFO_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERSOLDIERWEAPONSPAWNINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierWeaponSpawnInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerSoldierWeaponSpawnInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerSoldierWeapon {
    pub _glacier_base: super::game_server::ServerGameComponentEntity,
}

pub trait ServerSoldierWeaponTrait: super::game_server::ServerGameComponentEntityTrait {
}

impl ServerSoldierWeaponTrait for ServerSoldierWeapon {
}

impl super::game_server::ServerGameComponentEntityTrait for ServerSoldierWeapon {
}

impl super::gameplay_sim::GameComponentEntityTrait for ServerSoldierWeapon {
}

impl super::entity::ComponentEntityTrait for ServerSoldierWeapon {
}

impl super::entity::SpatialEntityTrait for ServerSoldierWeapon {
}

impl super::entity::EntityTrait for ServerSoldierWeapon {
}

impl super::entity::EntityBusPeerTrait for ServerSoldierWeapon {
}

pub static SERVERSOLDIERWEAPON_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierWeapon",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_server::SERVERGAMECOMPONENTENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierWeapon as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSOLDIERWEAPON_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSoldierWeapon {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSOLDIERWEAPON_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERSOLDIERWEAPON_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierWeapon-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerSoldierWeapon"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAnimationTurretRotationComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerAnimationTurretRotationComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerAnimationTurretRotationComponentTrait for ServerAnimationTurretRotationComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerAnimationTurretRotationComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerAnimationTurretRotationComponent {
}

impl super::entity::ComponentTrait for ServerAnimationTurretRotationComponent {
}

impl super::entity::EntityBusPeerTrait for ServerAnimationTurretRotationComponent {
}

pub static SERVERANIMATIONTURRETROTATIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAnimationTurretRotationComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAnimationTurretRotationComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERANIMATIONTURRETROTATIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAnimationTurretRotationComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERANIMATIONTURRETROTATIONCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERANIMATIONTURRETROTATIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAnimationTurretRotationComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerAnimationTurretRotationComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerGrenadeEntity {
    pub _glacier_base: super::weapon::ServerGhostProjectileEntity,
}

pub trait ServerGrenadeEntityTrait: super::weapon::ServerGhostProjectileEntityTrait {
}

impl ServerGrenadeEntityTrait for ServerGrenadeEntity {
}

impl super::weapon::ServerGhostProjectileEntityTrait for ServerGrenadeEntity {
}

impl super::weapon::ServerProjectileEntityTrait for ServerGrenadeEntity {
}

impl super::game_server::ServerPhysicsEntityTrait for ServerGrenadeEntity {
}

impl super::game_server::ServerGameComponentEntityTrait for ServerGrenadeEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ServerGrenadeEntity {
}

impl super::entity::ComponentEntityTrait for ServerGrenadeEntity {
}

impl super::entity::SpatialEntityTrait for ServerGrenadeEntity {
}

impl super::entity::EntityTrait for ServerGrenadeEntity {
}

impl super::entity::EntityBusPeerTrait for ServerGrenadeEntity {
}

pub static SERVERGRENADEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGrenadeEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::weapon::SERVERGHOSTPROJECTILEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGrenadeEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERGRENADEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerGrenadeEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGRENADEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERGRENADEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGrenadeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerGrenadeEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerExplosionPackEntity {
    pub _glacier_base: super::weapon::ServerGhostProjectileEntity,
}

pub trait ServerExplosionPackEntityTrait: super::weapon::ServerGhostProjectileEntityTrait {
}

impl ServerExplosionPackEntityTrait for ServerExplosionPackEntity {
}

impl super::weapon::ServerGhostProjectileEntityTrait for ServerExplosionPackEntity {
}

impl super::weapon::ServerProjectileEntityTrait for ServerExplosionPackEntity {
}

impl super::game_server::ServerPhysicsEntityTrait for ServerExplosionPackEntity {
}

impl super::game_server::ServerGameComponentEntityTrait for ServerExplosionPackEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ServerExplosionPackEntity {
}

impl super::entity::ComponentEntityTrait for ServerExplosionPackEntity {
}

impl super::entity::SpatialEntityTrait for ServerExplosionPackEntity {
}

impl super::entity::EntityTrait for ServerExplosionPackEntity {
}

impl super::entity::EntityBusPeerTrait for ServerExplosionPackEntity {
}

pub static SERVEREXPLOSIONPACKENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerExplosionPackEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::weapon::SERVERGHOSTPROJECTILEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerExplosionPackEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVEREXPLOSIONPACKENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerExplosionPackEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVEREXPLOSIONPACKENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVEREXPLOSIONPACKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerExplosionPackEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerExplosionPackEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerMissileHealthComponent {
    pub _glacier_base: super::game_server::ServerGameHealthComponent,
}

pub trait ServerMissileHealthComponentTrait: super::game_server::ServerGameHealthComponentTrait {
}

impl ServerMissileHealthComponentTrait for ServerMissileHealthComponent {
}

impl super::game_server::ServerGameHealthComponentTrait for ServerMissileHealthComponent {
}

impl super::gameplay_sim::HealthComponentTrait for ServerMissileHealthComponent {
}

impl super::entity::ComponentTrait for ServerMissileHealthComponent {
}

impl super::entity::EntityBusPeerTrait for ServerMissileHealthComponent {
}

pub static SERVERMISSILEHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMissileHealthComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_server::SERVERGAMEHEALTHCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerMissileHealthComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERMISSILEHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerMissileHealthComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERMISSILEHEALTHCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERMISSILEHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMissileHealthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerMissileHealthComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerExplosionPackPhysicsComponent {
    pub _glacier_base: super::physics::PartPhysicsComponent,
}

pub trait ServerExplosionPackPhysicsComponentTrait: super::physics::PartPhysicsComponentTrait {
}

impl ServerExplosionPackPhysicsComponentTrait for ServerExplosionPackPhysicsComponent {
}

impl super::physics::PartPhysicsComponentTrait for ServerExplosionPackPhysicsComponent {
}

impl super::physics::PhysicsComponentTrait for ServerExplosionPackPhysicsComponent {
}

impl super::entity::ComponentTrait for ServerExplosionPackPhysicsComponent {
}

impl super::entity::EntityBusPeerTrait for ServerExplosionPackPhysicsComponent {
}

pub static SERVEREXPLOSIONPACKPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerExplosionPackPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::PARTPHYSICSCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerExplosionPackPhysicsComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVEREXPLOSIONPACKPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerExplosionPackPhysicsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVEREXPLOSIONPACKPHYSICSCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVEREXPLOSIONPACKPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerExplosionPackPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerExplosionPackPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerExplosionPackHealthComponent {
    pub _glacier_base: super::game_server::ServerGameHealthComponent,
}

pub trait ServerExplosionPackHealthComponentTrait: super::game_server::ServerGameHealthComponentTrait {
}

impl ServerExplosionPackHealthComponentTrait for ServerExplosionPackHealthComponent {
}

impl super::game_server::ServerGameHealthComponentTrait for ServerExplosionPackHealthComponent {
}

impl super::gameplay_sim::HealthComponentTrait for ServerExplosionPackHealthComponent {
}

impl super::entity::ComponentTrait for ServerExplosionPackHealthComponent {
}

impl super::entity::EntityBusPeerTrait for ServerExplosionPackHealthComponent {
}

pub static SERVEREXPLOSIONPACKHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerExplosionPackHealthComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_server::SERVERGAMEHEALTHCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerExplosionPackHealthComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVEREXPLOSIONPACKHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerExplosionPackHealthComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVEREXPLOSIONPACKHEALTHCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVEREXPLOSIONPACKHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerExplosionPackHealthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerExplosionPackHealthComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerSoldierEntryComponent {
    pub _glacier_base: super::game_server::ServerCharacterEntryComponent,
}

pub trait ServerSoldierEntryComponentTrait: super::game_server::ServerCharacterEntryComponentTrait {
}

impl ServerSoldierEntryComponentTrait for ServerSoldierEntryComponent {
}

impl super::game_server::ServerCharacterEntryComponentTrait for ServerSoldierEntryComponent {
}

impl super::game_server::ServerGameEntryComponentTrait for ServerSoldierEntryComponent {
}

impl super::game_server::ServerEntryComponentTrait for ServerSoldierEntryComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerSoldierEntryComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerSoldierEntryComponent {
}

impl super::entity::ComponentTrait for ServerSoldierEntryComponent {
}

impl super::entity::EntityBusPeerTrait for ServerSoldierEntryComponent {
}

pub static SERVERSOLDIERENTRYCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierEntryComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_server::SERVERCHARACTERENTRYCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierEntryComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSOLDIERENTRYCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSoldierEntryComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSOLDIERENTRYCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERSOLDIERENTRYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierEntryComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerSoldierEntryComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerWeaponZeroingComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerWeaponZeroingComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerWeaponZeroingComponentTrait for ServerWeaponZeroingComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerWeaponZeroingComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerWeaponZeroingComponent {
}

impl super::entity::ComponentTrait for ServerWeaponZeroingComponent {
}

impl super::entity::EntityBusPeerTrait for ServerWeaponZeroingComponent {
}

pub static SERVERWEAPONZEROINGCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponZeroingComponent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerWeaponZeroingComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERWEAPONZEROINGCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerWeaponZeroingComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERWEAPONZEROINGCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERWEAPONZEROINGCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponZeroingComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerWeaponZeroingComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerSoldierInteractionEntityInRangeChangedMessage {
}

pub trait ServerSoldierInteractionEntityInRangeChangedMessageTrait: TypeObject {
}

impl ServerSoldierInteractionEntityInRangeChangedMessageTrait for ServerSoldierInteractionEntityInRangeChangedMessage {
}

pub static SERVERSOLDIERINTERACTIONENTITYINRANGECHANGEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierInteractionEntityInRangeChangedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierInteractionEntityInRangeChangedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerSoldierInteractionEntityInRangeChangedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSOLDIERINTERACTIONENTITYINRANGECHANGEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ServerSoldierSoldierVsSoldierCollisionMessage {
}

pub trait ServerSoldierSoldierVsSoldierCollisionMessageTrait: TypeObject {
}

impl ServerSoldierSoldierVsSoldierCollisionMessageTrait for ServerSoldierSoldierVsSoldierCollisionMessage {
}

pub static SERVERSOLDIERSOLDIERVSSOLDIERCOLLISIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierSoldierVsSoldierCollisionMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierSoldierVsSoldierCollisionMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerSoldierSoldierVsSoldierCollisionMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSOLDIERSOLDIERVSSOLDIERCOLLISIONMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ServerSoldierPickedUpSupplySphereMessage {
}

pub trait ServerSoldierPickedUpSupplySphereMessageTrait: TypeObject {
}

impl ServerSoldierPickedUpSupplySphereMessageTrait for ServerSoldierPickedUpSupplySphereMessage {
}

pub static SERVERSOLDIERPICKEDUPSUPPLYSPHEREMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierPickedUpSupplySphereMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierPickedUpSupplySphereMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerSoldierPickedUpSupplySphereMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSOLDIERPICKEDUPSUPPLYSPHEREMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ServerSoldierOnUnspawnMessage {
}

pub trait ServerSoldierOnUnspawnMessageTrait: TypeObject {
}

impl ServerSoldierOnUnspawnMessageTrait for ServerSoldierOnUnspawnMessage {
}

pub static SERVERSOLDIERONUNSPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierOnUnspawnMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierOnUnspawnMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerSoldierOnUnspawnMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSOLDIERONUNSPAWNMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ServerSoldierThrowDistractionMessage {
}

pub trait ServerSoldierThrowDistractionMessageTrait: TypeObject {
}

impl ServerSoldierThrowDistractionMessageTrait for ServerSoldierThrowDistractionMessage {
}

pub static SERVERSOLDIERTHROWDISTRACTIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierThrowDistractionMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierThrowDistractionMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for ServerSoldierThrowDistractionMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSOLDIERTHROWDISTRACTIONMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ServerSoldierSelfHealMessage {
}

pub trait ServerSoldierSelfHealMessageTrait: TypeObject {
}

impl ServerSoldierSelfHealMessageTrait for ServerSoldierSelfHealMessage {
}

pub static SERVERSOLDIERSELFHEALMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierSelfHealMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierSelfHealMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerSoldierSelfHealMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSOLDIERSELFHEALMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ServerSoldierManDownMessage {
}

pub trait ServerSoldierManDownMessageTrait: TypeObject {
}

impl ServerSoldierManDownMessageTrait for ServerSoldierManDownMessage {
}

pub static SERVERSOLDIERMANDOWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierManDownMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierManDownMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerSoldierManDownMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSOLDIERMANDOWNMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ServerSoldierOnInitMessage {
}

pub trait ServerSoldierOnInitMessageTrait: TypeObject {
}

impl ServerSoldierOnInitMessageTrait for ServerSoldierOnInitMessage {
}

pub static SERVERSOLDIERONINITMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierOnInitMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierOnInitMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerSoldierOnInitMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSOLDIERONINITMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ServerSoldierChangingWeaponMessage {
}

pub trait ServerSoldierChangingWeaponMessageTrait: TypeObject {
}

impl ServerSoldierChangingWeaponMessageTrait for ServerSoldierChangingWeaponMessage {
}

pub static SERVERSOLDIERCHANGINGWEAPONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierChangingWeaponMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierChangingWeaponMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerSoldierChangingWeaponMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSOLDIERCHANGINGWEAPONMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ServerSoldierFiringMessage {
}

pub trait ServerSoldierFiringMessageTrait: TypeObject {
}

impl ServerSoldierFiringMessageTrait for ServerSoldierFiringMessage {
}

pub static SERVERSOLDIERFIRINGMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierFiringMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierFiringMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerSoldierFiringMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSOLDIERFIRINGMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ServerSoldierDamagedMessage {
}

pub trait ServerSoldierDamagedMessageTrait: TypeObject {
}

impl ServerSoldierDamagedMessageTrait for ServerSoldierDamagedMessage {
}

pub static SERVERSOLDIERDAMAGEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierDamagedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierDamagedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerSoldierDamagedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSOLDIERDAMAGEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ServerProjectileMissileDestroyedMessage {
}

pub trait ServerProjectileMissileDestroyedMessageTrait: TypeObject {
}

impl ServerProjectileMissileDestroyedMessageTrait for ServerProjectileMissileDestroyedMessage {
}

pub static SERVERPROJECTILEMISSILEDESTROYEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerProjectileMissileDestroyedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerProjectileMissileDestroyedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerProjectileMissileDestroyedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPROJECTILEMISSILEDESTROYEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ServerProjectileMissileDamagedMessage {
}

pub trait ServerProjectileMissileDamagedMessageTrait: TypeObject {
}

impl ServerProjectileMissileDamagedMessageTrait for ServerProjectileMissileDamagedMessage {
}

pub static SERVERPROJECTILEMISSILEDAMAGEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerProjectileMissileDamagedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerProjectileMissileDamagedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerProjectileMissileDamagedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPROJECTILEMISSILEDAMAGEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientSoldierInteractionEntityInRangeChangedMessage {
}

pub trait ClientSoldierInteractionEntityInRangeChangedMessageTrait: TypeObject {
}

impl ClientSoldierInteractionEntityInRangeChangedMessageTrait for ClientSoldierInteractionEntityInRangeChangedMessage {
}

pub static CLIENTSOLDIERINTERACTIONENTITYINRANGECHANGEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierInteractionEntityInRangeChangedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSoldierInteractionEntityInRangeChangedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientSoldierInteractionEntityInRangeChangedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSOLDIERINTERACTIONENTITYINRANGECHANGEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientSoldierChangeCoverStateMessage {
}

pub trait ClientSoldierChangeCoverStateMessageTrait: TypeObject {
}

impl ClientSoldierChangeCoverStateMessageTrait for ClientSoldierChangeCoverStateMessage {
}

pub static CLIENTSOLDIERCHANGECOVERSTATEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierChangeCoverStateMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSoldierChangeCoverStateMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientSoldierChangeCoverStateMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSOLDIERCHANGECOVERSTATEMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct AimingHandle {
}

pub trait AimingHandleTrait: TypeObject {
}

impl AimingHandleTrait for AimingHandle {
}

pub static AIMINGHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingHandle",
    flags: MemberInfoFlags::new(73),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AimingHandle as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(AIMINGHANDLE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AimingHandle {
    fn type_info(&self) -> &'static TypeInfo {
        AIMINGHANDLE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static AIMINGHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("AimingHandle"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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

pub trait AimingRenderReturnedValuesTrait: TypeObject {
    fn aimer_position(&self) -> &super::core::Vec3;
    fn last_hit_position(&self) -> &super::core::Vec3;
    fn recoil_compensation(&self) -> &super::core::Vec2;
    fn last_apply_angles_from_sim_tick(&self) -> &u32;
    fn yaw(&self) -> &f32;
    fn pitch(&self) -> &f32;
    fn input_magnitude(&self) -> &f32;
    fn snap_zoom_break_away(&self) -> &bool;
    fn use_aim_assist(&self) -> &bool;
    fn allow_blend_out(&self) -> &bool;
}

impl AimingRenderReturnedValuesTrait for AimingRenderReturnedValues {
    fn aimer_position(&self) -> &super::core::Vec3 {
        &self.aimer_position
    }
    fn last_hit_position(&self) -> &super::core::Vec3 {
        &self.last_hit_position
    }
    fn recoil_compensation(&self) -> &super::core::Vec2 {
        &self.recoil_compensation
    }
    fn last_apply_angles_from_sim_tick(&self) -> &u32 {
        &self.last_apply_angles_from_sim_tick
    }
    fn yaw(&self) -> &f32 {
        &self.yaw
    }
    fn pitch(&self) -> &f32 {
        &self.pitch
    }
    fn input_magnitude(&self) -> &f32 {
        &self.input_magnitude
    }
    fn snap_zoom_break_away(&self) -> &bool {
        &self.snap_zoom_break_away
    }
    fn use_aim_assist(&self) -> &bool {
        &self.use_aim_assist
    }
    fn allow_blend_out(&self) -> &bool {
        &self.allow_blend_out
    }
}

pub static AIMINGRENDERRETURNEDVALUES_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingRenderReturnedValues",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AimingRenderReturnedValues as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "AimerPosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AimingRenderReturnedValues, aimer_position),
            },
            FieldInfoData {
                name: "LastHitPosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AimingRenderReturnedValues, last_hit_position),
            },
            FieldInfoData {
                name: "RecoilCompensation",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(AimingRenderReturnedValues, recoil_compensation),
            },
            FieldInfoData {
                name: "LastApplyAnglesFromSimTick",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(AimingRenderReturnedValues, last_apply_angles_from_sim_tick),
            },
            FieldInfoData {
                name: "Yaw",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingRenderReturnedValues, yaw),
            },
            FieldInfoData {
                name: "Pitch",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingRenderReturnedValues, pitch),
            },
            FieldInfoData {
                name: "InputMagnitude",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingRenderReturnedValues, input_magnitude),
            },
            FieldInfoData {
                name: "SnapZoomBreakAway",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingRenderReturnedValues, snap_zoom_break_away),
            },
            FieldInfoData {
                name: "UseAimAssist",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingRenderReturnedValues, use_aim_assist),
            },
            FieldInfoData {
                name: "AllowBlendOut",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingRenderReturnedValues, allow_blend_out),
            },
        ],
    }),
    array_type: Some(AIMINGRENDERRETURNEDVALUES_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AimingRenderReturnedValues {
    fn type_info(&self) -> &'static TypeInfo {
        AIMINGRENDERRETURNEDVALUES_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static AIMINGRENDERRETURNEDVALUES_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingRenderReturnedValues-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("AimingRenderReturnedValues"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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

pub trait AimingRenderUpdateContextTrait: TypeObject {
    fn aimer_position(&self) -> &super::core::Vec3;
    fn aimer_safe_position(&self) -> &super::core::Vec3;
    fn last_aim_direction(&self) -> &super::core::Vec3;
    fn last_hit_position(&self) -> &super::core::Vec3;
    fn aiming_input(&self) -> &super::core::Vec2;
    fn input_direction(&self) -> &super::core::Vec2;
    fn input_acceleration_velocity(&self) -> &super::core::Vec2;
    fn recoil_offset(&self) -> &super::core::Vec2;
    fn last_tick(&self) -> &u32;
    fn last_apply_angles_from_sim_tick(&self) -> &u32;
    fn yaw(&self) -> &f32;
    fn pitch(&self) -> &f32;
    fn input_magnitude(&self) -> &f32;
    fn soft_zone_lambda_yaw_attract(&self) -> &f32;
    fn soft_zone_lambda_pitch_attract(&self) -> &f32;
    fn soft_zone_lambda_slowdown(&self) -> &f32;
    fn target_distance(&self) -> &f32;
    fn yaw_change_attract(&self) -> &f32;
    fn pitch_change_attract(&self) -> &f32;
    fn time_since_yaw_input(&self) -> &f32;
    fn time_since_pitch_input(&self) -> &f32;
    fn acceleration(&self) -> &f32;
    fn acceleration_timer(&self) -> &f32;
    fn aimer_arm_length(&self) -> &f32;
    fn time_to_delay_after_collision(&self) -> &f32;
    fn snap_zoom_break_away(&self) -> &bool;
    fn is_mouse_aiming(&self) -> &bool;
    fn use_aim_assist(&self) -> &bool;
    fn use_input_polynomials(&self) -> &bool;
    fn allow_blend_out(&self) -> &bool;
}

impl AimingRenderUpdateContextTrait for AimingRenderUpdateContext {
    fn aimer_position(&self) -> &super::core::Vec3 {
        &self.aimer_position
    }
    fn aimer_safe_position(&self) -> &super::core::Vec3 {
        &self.aimer_safe_position
    }
    fn last_aim_direction(&self) -> &super::core::Vec3 {
        &self.last_aim_direction
    }
    fn last_hit_position(&self) -> &super::core::Vec3 {
        &self.last_hit_position
    }
    fn aiming_input(&self) -> &super::core::Vec2 {
        &self.aiming_input
    }
    fn input_direction(&self) -> &super::core::Vec2 {
        &self.input_direction
    }
    fn input_acceleration_velocity(&self) -> &super::core::Vec2 {
        &self.input_acceleration_velocity
    }
    fn recoil_offset(&self) -> &super::core::Vec2 {
        &self.recoil_offset
    }
    fn last_tick(&self) -> &u32 {
        &self.last_tick
    }
    fn last_apply_angles_from_sim_tick(&self) -> &u32 {
        &self.last_apply_angles_from_sim_tick
    }
    fn yaw(&self) -> &f32 {
        &self.yaw
    }
    fn pitch(&self) -> &f32 {
        &self.pitch
    }
    fn input_magnitude(&self) -> &f32 {
        &self.input_magnitude
    }
    fn soft_zone_lambda_yaw_attract(&self) -> &f32 {
        &self.soft_zone_lambda_yaw_attract
    }
    fn soft_zone_lambda_pitch_attract(&self) -> &f32 {
        &self.soft_zone_lambda_pitch_attract
    }
    fn soft_zone_lambda_slowdown(&self) -> &f32 {
        &self.soft_zone_lambda_slowdown
    }
    fn target_distance(&self) -> &f32 {
        &self.target_distance
    }
    fn yaw_change_attract(&self) -> &f32 {
        &self.yaw_change_attract
    }
    fn pitch_change_attract(&self) -> &f32 {
        &self.pitch_change_attract
    }
    fn time_since_yaw_input(&self) -> &f32 {
        &self.time_since_yaw_input
    }
    fn time_since_pitch_input(&self) -> &f32 {
        &self.time_since_pitch_input
    }
    fn acceleration(&self) -> &f32 {
        &self.acceleration
    }
    fn acceleration_timer(&self) -> &f32 {
        &self.acceleration_timer
    }
    fn aimer_arm_length(&self) -> &f32 {
        &self.aimer_arm_length
    }
    fn time_to_delay_after_collision(&self) -> &f32 {
        &self.time_to_delay_after_collision
    }
    fn snap_zoom_break_away(&self) -> &bool {
        &self.snap_zoom_break_away
    }
    fn is_mouse_aiming(&self) -> &bool {
        &self.is_mouse_aiming
    }
    fn use_aim_assist(&self) -> &bool {
        &self.use_aim_assist
    }
    fn use_input_polynomials(&self) -> &bool {
        &self.use_input_polynomials
    }
    fn allow_blend_out(&self) -> &bool {
        &self.allow_blend_out
    }
}

pub static AIMINGRENDERUPDATECONTEXT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingRenderUpdateContext",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AimingRenderUpdateContext as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "AimerPosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AimingRenderUpdateContext, aimer_position),
            },
            FieldInfoData {
                name: "AimerSafePosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AimingRenderUpdateContext, aimer_safe_position),
            },
            FieldInfoData {
                name: "LastAimDirection",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AimingRenderUpdateContext, last_aim_direction),
            },
            FieldInfoData {
                name: "LastHitPosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AimingRenderUpdateContext, last_hit_position),
            },
            FieldInfoData {
                name: "AimingInput",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(AimingRenderUpdateContext, aiming_input),
            },
            FieldInfoData {
                name: "InputDirection",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(AimingRenderUpdateContext, input_direction),
            },
            FieldInfoData {
                name: "InputAccelerationVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(AimingRenderUpdateContext, input_acceleration_velocity),
            },
            FieldInfoData {
                name: "RecoilOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(AimingRenderUpdateContext, recoil_offset),
            },
            FieldInfoData {
                name: "LastTick",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(AimingRenderUpdateContext, last_tick),
            },
            FieldInfoData {
                name: "LastApplyAnglesFromSimTick",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(AimingRenderUpdateContext, last_apply_angles_from_sim_tick),
            },
            FieldInfoData {
                name: "Yaw",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingRenderUpdateContext, yaw),
            },
            FieldInfoData {
                name: "Pitch",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingRenderUpdateContext, pitch),
            },
            FieldInfoData {
                name: "InputMagnitude",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingRenderUpdateContext, input_magnitude),
            },
            FieldInfoData {
                name: "SoftZoneLambdaYawAttract",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingRenderUpdateContext, soft_zone_lambda_yaw_attract),
            },
            FieldInfoData {
                name: "SoftZoneLambdaPitchAttract",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingRenderUpdateContext, soft_zone_lambda_pitch_attract),
            },
            FieldInfoData {
                name: "SoftZoneLambdaSlowdown",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingRenderUpdateContext, soft_zone_lambda_slowdown),
            },
            FieldInfoData {
                name: "TargetDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingRenderUpdateContext, target_distance),
            },
            FieldInfoData {
                name: "YawChangeAttract",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingRenderUpdateContext, yaw_change_attract),
            },
            FieldInfoData {
                name: "PitchChangeAttract",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingRenderUpdateContext, pitch_change_attract),
            },
            FieldInfoData {
                name: "TimeSinceYawInput",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingRenderUpdateContext, time_since_yaw_input),
            },
            FieldInfoData {
                name: "TimeSincePitchInput",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingRenderUpdateContext, time_since_pitch_input),
            },
            FieldInfoData {
                name: "Acceleration",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingRenderUpdateContext, acceleration),
            },
            FieldInfoData {
                name: "AccelerationTimer",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingRenderUpdateContext, acceleration_timer),
            },
            FieldInfoData {
                name: "AimerArmLength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingRenderUpdateContext, aimer_arm_length),
            },
            FieldInfoData {
                name: "TimeToDelayAfterCollision",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingRenderUpdateContext, time_to_delay_after_collision),
            },
            FieldInfoData {
                name: "SnapZoomBreakAway",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingRenderUpdateContext, snap_zoom_break_away),
            },
            FieldInfoData {
                name: "IsMouseAiming",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingRenderUpdateContext, is_mouse_aiming),
            },
            FieldInfoData {
                name: "UseAimAssist",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingRenderUpdateContext, use_aim_assist),
            },
            FieldInfoData {
                name: "UseInputPolynomials",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingRenderUpdateContext, use_input_polynomials),
            },
            FieldInfoData {
                name: "AllowBlendOut",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingRenderUpdateContext, allow_blend_out),
            },
        ],
    }),
    array_type: Some(AIMINGRENDERUPDATECONTEXT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AimingRenderUpdateContext {
    fn type_info(&self) -> &'static TypeInfo {
        AIMINGRENDERUPDATECONTEXT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static AIMINGRENDERUPDATECONTEXT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingRenderUpdateContext-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("AimingRenderUpdateContext"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AimingSimUpdateContext {
    pub difficulty_data: Option<Arc<Mutex<dyn super::game_shared::DifficultyDataTrait>>>,
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

pub trait AimingSimUpdateContextTrait: TypeObject {
    fn difficulty_data(&self) -> &Option<Arc<Mutex<dyn super::game_shared::DifficultyDataTrait>>>;
    fn aiming_range(&self) -> &f32;
    fn attract_pitch_strength(&self) -> &f32;
    fn attract_soft_zone(&self) -> &f32;
    fn attract_yaw_strength(&self) -> &f32;
    fn snap_zoom_post_time_no_input(&self) -> &f32;
    fn snap_zoom_post_time(&self) -> &f32;
    fn snap_zoom_since_last_timer(&self) -> &f32;
    fn zoom_transition_timer(&self) -> &f32;
    fn local_player_id(&self) -> &super::core::LocalPlayerId;
    fn attract_zooming_post_player_aiming(&self) -> &bool;
    fn force_aim_snap_deactivate(&self) -> &bool;
    fn force_pick_best_target(&self) -> &bool;
    fn has_been_sprinting(&self) -> &bool;
    fn is_sprinting(&self) -> &bool;
    fn use_aim_helpers_rotation(&self) -> &bool;
    fn use_aim_helpers_slowdown(&self) -> &bool;
    fn snap_zoom_post_time_dynamic_point(&self) -> &bool;
    fn snap_zoom_target_changed(&self) -> &bool;
}

impl AimingSimUpdateContextTrait for AimingSimUpdateContext {
    fn difficulty_data(&self) -> &Option<Arc<Mutex<dyn super::game_shared::DifficultyDataTrait>>> {
        &self.difficulty_data
    }
    fn aiming_range(&self) -> &f32 {
        &self.aiming_range
    }
    fn attract_pitch_strength(&self) -> &f32 {
        &self.attract_pitch_strength
    }
    fn attract_soft_zone(&self) -> &f32 {
        &self.attract_soft_zone
    }
    fn attract_yaw_strength(&self) -> &f32 {
        &self.attract_yaw_strength
    }
    fn snap_zoom_post_time_no_input(&self) -> &f32 {
        &self.snap_zoom_post_time_no_input
    }
    fn snap_zoom_post_time(&self) -> &f32 {
        &self.snap_zoom_post_time
    }
    fn snap_zoom_since_last_timer(&self) -> &f32 {
        &self.snap_zoom_since_last_timer
    }
    fn zoom_transition_timer(&self) -> &f32 {
        &self.zoom_transition_timer
    }
    fn local_player_id(&self) -> &super::core::LocalPlayerId {
        &self.local_player_id
    }
    fn attract_zooming_post_player_aiming(&self) -> &bool {
        &self.attract_zooming_post_player_aiming
    }
    fn force_aim_snap_deactivate(&self) -> &bool {
        &self.force_aim_snap_deactivate
    }
    fn force_pick_best_target(&self) -> &bool {
        &self.force_pick_best_target
    }
    fn has_been_sprinting(&self) -> &bool {
        &self.has_been_sprinting
    }
    fn is_sprinting(&self) -> &bool {
        &self.is_sprinting
    }
    fn use_aim_helpers_rotation(&self) -> &bool {
        &self.use_aim_helpers_rotation
    }
    fn use_aim_helpers_slowdown(&self) -> &bool {
        &self.use_aim_helpers_slowdown
    }
    fn snap_zoom_post_time_dynamic_point(&self) -> &bool {
        &self.snap_zoom_post_time_dynamic_point
    }
    fn snap_zoom_target_changed(&self) -> &bool {
        &self.snap_zoom_target_changed
    }
}

pub static AIMINGSIMUPDATECONTEXT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingSimUpdateContext",
    flags: MemberInfoFlags::new(73),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AimingSimUpdateContext as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "DifficultyData",
                flags: MemberInfoFlags::new(0),
                field_type: "DifficultyData",
                rust_offset: offset_of!(AimingSimUpdateContext, difficulty_data),
            },
            FieldInfoData {
                name: "AimingRange",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimUpdateContext, aiming_range),
            },
            FieldInfoData {
                name: "AttractPitchStrength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimUpdateContext, attract_pitch_strength),
            },
            FieldInfoData {
                name: "AttractSoftZone",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimUpdateContext, attract_soft_zone),
            },
            FieldInfoData {
                name: "AttractYawStrength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimUpdateContext, attract_yaw_strength),
            },
            FieldInfoData {
                name: "SnapZoomPostTimeNoInput",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimUpdateContext, snap_zoom_post_time_no_input),
            },
            FieldInfoData {
                name: "SnapZoomPostTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimUpdateContext, snap_zoom_post_time),
            },
            FieldInfoData {
                name: "SnapZoomSinceLastTimer",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimUpdateContext, snap_zoom_since_last_timer),
            },
            FieldInfoData {
                name: "ZoomTransitionTimer",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimUpdateContext, zoom_transition_timer),
            },
            FieldInfoData {
                name: "LocalPlayerId",
                flags: MemberInfoFlags::new(0),
                field_type: "LocalPlayerId",
                rust_offset: offset_of!(AimingSimUpdateContext, local_player_id),
            },
            FieldInfoData {
                name: "AttractZoomingPostPlayerAiming",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimUpdateContext, attract_zooming_post_player_aiming),
            },
            FieldInfoData {
                name: "ForceAimSnapDeactivate",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimUpdateContext, force_aim_snap_deactivate),
            },
            FieldInfoData {
                name: "ForcePickBestTarget",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimUpdateContext, force_pick_best_target),
            },
            FieldInfoData {
                name: "HasBeenSprinting",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimUpdateContext, has_been_sprinting),
            },
            FieldInfoData {
                name: "IsSprinting",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimUpdateContext, is_sprinting),
            },
            FieldInfoData {
                name: "UseAimHelpersRotation",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimUpdateContext, use_aim_helpers_rotation),
            },
            FieldInfoData {
                name: "UseAimHelpersSlowdown",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimUpdateContext, use_aim_helpers_slowdown),
            },
            FieldInfoData {
                name: "SnapZoomPostTimeDynamicPoint",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimUpdateContext, snap_zoom_post_time_dynamic_point),
            },
            FieldInfoData {
                name: "SnapZoomTargetChanged",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimUpdateContext, snap_zoom_target_changed),
            },
        ],
    }),
    array_type: Some(AIMINGSIMUPDATECONTEXT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AimingSimUpdateContext {
    fn type_info(&self) -> &'static TypeInfo {
        AIMINGSIMUPDATECONTEXT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static AIMINGSIMUPDATECONTEXT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingSimUpdateContext-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("AimingSimUpdateContext"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    pub solider_aiming_simulation_data: Option<Arc<Mutex<dyn super::soldier_shared::SoldierAimingSimulationDataTrait>>>,
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

pub trait AimingSimDynamicStateTrait: TypeObject {
    fn aiming_environment_target(&self) -> &AimingEnvironmentTarget;
    fn aiming_constraints(&self) -> &AimingConstraints;
    fn collision_excluded_bodies(&self) -> &super::physics::PhysicsRenderWorldHandle;
    fn character_entity_space_component_transform(&self) -> &super::core::LinearTransform;
    fn lock_aim_transform(&self) -> &super::core::LinearTransform;
    fn aimer_root_position(&self) -> &super::core::Vec3;
    fn desired_aimer_local_position(&self) -> &super::core::Vec3;
    fn desired_aimer_safe_position(&self) -> &super::core::Vec3;
    fn static_aimer_safe_position(&self) -> &super::core::Vec3;
    fn force_aim_snap_target_position(&self) -> &super::core::Vec3;
    fn reticle_speed(&self) -> &super::core::Vec3;
    fn surface_angular_velocity(&self) -> &super::core::Vec3;
    fn sim_aiming_input(&self) -> &super::core::Vec2;
    fn aim_scale(&self) -> &super::core::Vec2;
    fn attract_distance_fall_off(&self) -> &super::core::Vec2;
    fn max_angular_velocity(&self) -> &super::core::Vec2;
    fn movement_input(&self) -> &super::core::Vec2;
    fn recoil(&self) -> &super::core::Vec2;
    fn aim_sway_offset(&self) -> &super::core::Vec2;
    fn solider_aiming_simulation_data(&self) -> &Option<Arc<Mutex<dyn super::soldier_shared::SoldierAimingSimulationDataTrait>>>;
    fn zoom_level(&self) -> &u32;
    fn tick(&self) -> &u32;
    fn apply_angles_from_sim_tick(&self) -> &u32;
    fn ignore_constraints_tick(&self) -> &u32;
    fn attract_user_input_multiplier(&self) -> &f32;
    fn attract_zooming_post_timer(&self) -> &f32;
    fn attract_zooming_post_time(&self) -> &f32;
    fn last_hit_position_distance(&self) -> &f32;
    fn look_speed_multiplier(&self) -> &f32;
    fn minimum_pitch(&self) -> &f32;
    fn maximum_pitch(&self) -> &f32;
    fn reticle_field_of_view(&self) -> &f32;
    fn sim_pitch_to_apply(&self) -> &f32;
    fn sim_yaw_to_apply(&self) -> &f32;
    fn snap_zoom_break_away_timer(&self) -> &f32;
    fn snap_zoom_timer(&self) -> &f32;
    fn snap_zoom_time(&self) -> &f32;
    fn world_space_lock_efficiency_pitch(&self) -> &f32;
    fn world_space_lock_efficiency_yaw(&self) -> &f32;
    fn aimer_collision_blend_out(&self) -> &f32;
    fn time_to_delay_after_collision(&self) -> &f32;
    fn override_mode(&self) -> &AimOverrideMode;
    fn aim_at_last_hit_position(&self) -> &bool;
    fn force_aim_snap(&self) -> &bool;
    fn has_aiming_constraints(&self) -> &bool;
    fn has_angular_velocity_constraints(&self) -> &bool;
    fn has_character_entity_space_component(&self) -> &bool;
    fn is_alive(&self) -> &bool;
    fn is_dead(&self) -> &bool;
    fn is_fps_aiming_disabled(&self) -> &bool;
    fn is_snap_zoomed(&self) -> &bool;
    fn snap_zoom_allowed(&self) -> &bool;
    fn zoom_has_changed(&self) -> &bool;
    fn zoom_in_aiming_help_active(&self) -> &bool;
    fn aim_assist_option_enabled(&self) -> &bool;
    fn field_flag_changed0(&self) -> &u32;
    fn field_flag_changed1(&self) -> &u32;
}

impl AimingSimDynamicStateTrait for AimingSimDynamicState {
    fn aiming_environment_target(&self) -> &AimingEnvironmentTarget {
        &self.aiming_environment_target
    }
    fn aiming_constraints(&self) -> &AimingConstraints {
        &self.aiming_constraints
    }
    fn collision_excluded_bodies(&self) -> &super::physics::PhysicsRenderWorldHandle {
        &self.collision_excluded_bodies
    }
    fn character_entity_space_component_transform(&self) -> &super::core::LinearTransform {
        &self.character_entity_space_component_transform
    }
    fn lock_aim_transform(&self) -> &super::core::LinearTransform {
        &self.lock_aim_transform
    }
    fn aimer_root_position(&self) -> &super::core::Vec3 {
        &self.aimer_root_position
    }
    fn desired_aimer_local_position(&self) -> &super::core::Vec3 {
        &self.desired_aimer_local_position
    }
    fn desired_aimer_safe_position(&self) -> &super::core::Vec3 {
        &self.desired_aimer_safe_position
    }
    fn static_aimer_safe_position(&self) -> &super::core::Vec3 {
        &self.static_aimer_safe_position
    }
    fn force_aim_snap_target_position(&self) -> &super::core::Vec3 {
        &self.force_aim_snap_target_position
    }
    fn reticle_speed(&self) -> &super::core::Vec3 {
        &self.reticle_speed
    }
    fn surface_angular_velocity(&self) -> &super::core::Vec3 {
        &self.surface_angular_velocity
    }
    fn sim_aiming_input(&self) -> &super::core::Vec2 {
        &self.sim_aiming_input
    }
    fn aim_scale(&self) -> &super::core::Vec2 {
        &self.aim_scale
    }
    fn attract_distance_fall_off(&self) -> &super::core::Vec2 {
        &self.attract_distance_fall_off
    }
    fn max_angular_velocity(&self) -> &super::core::Vec2 {
        &self.max_angular_velocity
    }
    fn movement_input(&self) -> &super::core::Vec2 {
        &self.movement_input
    }
    fn recoil(&self) -> &super::core::Vec2 {
        &self.recoil
    }
    fn aim_sway_offset(&self) -> &super::core::Vec2 {
        &self.aim_sway_offset
    }
    fn solider_aiming_simulation_data(&self) -> &Option<Arc<Mutex<dyn super::soldier_shared::SoldierAimingSimulationDataTrait>>> {
        &self.solider_aiming_simulation_data
    }
    fn zoom_level(&self) -> &u32 {
        &self.zoom_level
    }
    fn tick(&self) -> &u32 {
        &self.tick
    }
    fn apply_angles_from_sim_tick(&self) -> &u32 {
        &self.apply_angles_from_sim_tick
    }
    fn ignore_constraints_tick(&self) -> &u32 {
        &self.ignore_constraints_tick
    }
    fn attract_user_input_multiplier(&self) -> &f32 {
        &self.attract_user_input_multiplier
    }
    fn attract_zooming_post_timer(&self) -> &f32 {
        &self.attract_zooming_post_timer
    }
    fn attract_zooming_post_time(&self) -> &f32 {
        &self.attract_zooming_post_time
    }
    fn last_hit_position_distance(&self) -> &f32 {
        &self.last_hit_position_distance
    }
    fn look_speed_multiplier(&self) -> &f32 {
        &self.look_speed_multiplier
    }
    fn minimum_pitch(&self) -> &f32 {
        &self.minimum_pitch
    }
    fn maximum_pitch(&self) -> &f32 {
        &self.maximum_pitch
    }
    fn reticle_field_of_view(&self) -> &f32 {
        &self.reticle_field_of_view
    }
    fn sim_pitch_to_apply(&self) -> &f32 {
        &self.sim_pitch_to_apply
    }
    fn sim_yaw_to_apply(&self) -> &f32 {
        &self.sim_yaw_to_apply
    }
    fn snap_zoom_break_away_timer(&self) -> &f32 {
        &self.snap_zoom_break_away_timer
    }
    fn snap_zoom_timer(&self) -> &f32 {
        &self.snap_zoom_timer
    }
    fn snap_zoom_time(&self) -> &f32 {
        &self.snap_zoom_time
    }
    fn world_space_lock_efficiency_pitch(&self) -> &f32 {
        &self.world_space_lock_efficiency_pitch
    }
    fn world_space_lock_efficiency_yaw(&self) -> &f32 {
        &self.world_space_lock_efficiency_yaw
    }
    fn aimer_collision_blend_out(&self) -> &f32 {
        &self.aimer_collision_blend_out
    }
    fn time_to_delay_after_collision(&self) -> &f32 {
        &self.time_to_delay_after_collision
    }
    fn override_mode(&self) -> &AimOverrideMode {
        &self.override_mode
    }
    fn aim_at_last_hit_position(&self) -> &bool {
        &self.aim_at_last_hit_position
    }
    fn force_aim_snap(&self) -> &bool {
        &self.force_aim_snap
    }
    fn has_aiming_constraints(&self) -> &bool {
        &self.has_aiming_constraints
    }
    fn has_angular_velocity_constraints(&self) -> &bool {
        &self.has_angular_velocity_constraints
    }
    fn has_character_entity_space_component(&self) -> &bool {
        &self.has_character_entity_space_component
    }
    fn is_alive(&self) -> &bool {
        &self.is_alive
    }
    fn is_dead(&self) -> &bool {
        &self.is_dead
    }
    fn is_fps_aiming_disabled(&self) -> &bool {
        &self.is_fps_aiming_disabled
    }
    fn is_snap_zoomed(&self) -> &bool {
        &self.is_snap_zoomed
    }
    fn snap_zoom_allowed(&self) -> &bool {
        &self.snap_zoom_allowed
    }
    fn zoom_has_changed(&self) -> &bool {
        &self.zoom_has_changed
    }
    fn zoom_in_aiming_help_active(&self) -> &bool {
        &self.zoom_in_aiming_help_active
    }
    fn aim_assist_option_enabled(&self) -> &bool {
        &self.aim_assist_option_enabled
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
    fn field_flag_changed1(&self) -> &u32 {
        &self.field_flag_changed1
    }
}

pub static AIMINGSIMDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingSimDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AimingSimDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "AimingEnvironmentTarget",
                flags: MemberInfoFlags::new(0),
                field_type: "AimingEnvironmentTarget",
                rust_offset: offset_of!(AimingSimDynamicState, aiming_environment_target),
            },
            FieldInfoData {
                name: "AimingConstraints",
                flags: MemberInfoFlags::new(0),
                field_type: "AimingConstraints",
                rust_offset: offset_of!(AimingSimDynamicState, aiming_constraints),
            },
            FieldInfoData {
                name: "CollisionExcludedBodies",
                flags: MemberInfoFlags::new(0),
                field_type: "PhysicsRenderWorldHandle",
                rust_offset: offset_of!(AimingSimDynamicState, collision_excluded_bodies),
            },
            FieldInfoData {
                name: "CharacterEntitySpaceComponentTransform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(AimingSimDynamicState, character_entity_space_component_transform),
            },
            FieldInfoData {
                name: "LockAimTransform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(AimingSimDynamicState, lock_aim_transform),
            },
            FieldInfoData {
                name: "AimerRootPosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AimingSimDynamicState, aimer_root_position),
            },
            FieldInfoData {
                name: "DesiredAimerLocalPosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AimingSimDynamicState, desired_aimer_local_position),
            },
            FieldInfoData {
                name: "DesiredAimerSafePosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AimingSimDynamicState, desired_aimer_safe_position),
            },
            FieldInfoData {
                name: "StaticAimerSafePosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AimingSimDynamicState, static_aimer_safe_position),
            },
            FieldInfoData {
                name: "ForceAimSnapTargetPosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AimingSimDynamicState, force_aim_snap_target_position),
            },
            FieldInfoData {
                name: "ReticleSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AimingSimDynamicState, reticle_speed),
            },
            FieldInfoData {
                name: "SurfaceAngularVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AimingSimDynamicState, surface_angular_velocity),
            },
            FieldInfoData {
                name: "SimAimingInput",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(AimingSimDynamicState, sim_aiming_input),
            },
            FieldInfoData {
                name: "AimScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(AimingSimDynamicState, aim_scale),
            },
            FieldInfoData {
                name: "AttractDistanceFallOff",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(AimingSimDynamicState, attract_distance_fall_off),
            },
            FieldInfoData {
                name: "MaxAngularVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(AimingSimDynamicState, max_angular_velocity),
            },
            FieldInfoData {
                name: "MovementInput",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(AimingSimDynamicState, movement_input),
            },
            FieldInfoData {
                name: "Recoil",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(AimingSimDynamicState, recoil),
            },
            FieldInfoData {
                name: "AimSwayOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(AimingSimDynamicState, aim_sway_offset),
            },
            FieldInfoData {
                name: "SoliderAimingSimulationData",
                flags: MemberInfoFlags::new(0),
                field_type: "SoldierAimingSimulationData",
                rust_offset: offset_of!(AimingSimDynamicState, solider_aiming_simulation_data),
            },
            FieldInfoData {
                name: "ZoomLevel",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(AimingSimDynamicState, zoom_level),
            },
            FieldInfoData {
                name: "Tick",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(AimingSimDynamicState, tick),
            },
            FieldInfoData {
                name: "ApplyAnglesFromSimTick",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(AimingSimDynamicState, apply_angles_from_sim_tick),
            },
            FieldInfoData {
                name: "IgnoreConstraintsTick",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(AimingSimDynamicState, ignore_constraints_tick),
            },
            FieldInfoData {
                name: "AttractUserInputMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimDynamicState, attract_user_input_multiplier),
            },
            FieldInfoData {
                name: "AttractZoomingPostTimer",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimDynamicState, attract_zooming_post_timer),
            },
            FieldInfoData {
                name: "AttractZoomingPostTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimDynamicState, attract_zooming_post_time),
            },
            FieldInfoData {
                name: "LastHitPositionDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimDynamicState, last_hit_position_distance),
            },
            FieldInfoData {
                name: "LookSpeedMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimDynamicState, look_speed_multiplier),
            },
            FieldInfoData {
                name: "MinimumPitch",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimDynamicState, minimum_pitch),
            },
            FieldInfoData {
                name: "MaximumPitch",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimDynamicState, maximum_pitch),
            },
            FieldInfoData {
                name: "ReticleFieldOfView",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimDynamicState, reticle_field_of_view),
            },
            FieldInfoData {
                name: "SimPitchToApply",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimDynamicState, sim_pitch_to_apply),
            },
            FieldInfoData {
                name: "SimYawToApply",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimDynamicState, sim_yaw_to_apply),
            },
            FieldInfoData {
                name: "SnapZoomBreakAwayTimer",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimDynamicState, snap_zoom_break_away_timer),
            },
            FieldInfoData {
                name: "SnapZoomTimer",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimDynamicState, snap_zoom_timer),
            },
            FieldInfoData {
                name: "SnapZoomTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimDynamicState, snap_zoom_time),
            },
            FieldInfoData {
                name: "WorldSpaceLockEfficiencyPitch",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimDynamicState, world_space_lock_efficiency_pitch),
            },
            FieldInfoData {
                name: "WorldSpaceLockEfficiencyYaw",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimDynamicState, world_space_lock_efficiency_yaw),
            },
            FieldInfoData {
                name: "AimerCollisionBlendOut",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimDynamicState, aimer_collision_blend_out),
            },
            FieldInfoData {
                name: "TimeToDelayAfterCollision",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimDynamicState, time_to_delay_after_collision),
            },
            FieldInfoData {
                name: "OverrideMode",
                flags: MemberInfoFlags::new(0),
                field_type: "AimOverrideMode",
                rust_offset: offset_of!(AimingSimDynamicState, override_mode),
            },
            FieldInfoData {
                name: "AimAtLastHitPosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimDynamicState, aim_at_last_hit_position),
            },
            FieldInfoData {
                name: "ForceAimSnap",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimDynamicState, force_aim_snap),
            },
            FieldInfoData {
                name: "HasAimingConstraints",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimDynamicState, has_aiming_constraints),
            },
            FieldInfoData {
                name: "HasAngularVelocityConstraints",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimDynamicState, has_angular_velocity_constraints),
            },
            FieldInfoData {
                name: "HasCharacterEntitySpaceComponent",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimDynamicState, has_character_entity_space_component),
            },
            FieldInfoData {
                name: "IsAlive",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimDynamicState, is_alive),
            },
            FieldInfoData {
                name: "IsDead",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimDynamicState, is_dead),
            },
            FieldInfoData {
                name: "IsFpsAimingDisabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimDynamicState, is_fps_aiming_disabled),
            },
            FieldInfoData {
                name: "IsSnapZoomed",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimDynamicState, is_snap_zoomed),
            },
            FieldInfoData {
                name: "SnapZoomAllowed",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimDynamicState, snap_zoom_allowed),
            },
            FieldInfoData {
                name: "ZoomHasChanged",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimDynamicState, zoom_has_changed),
            },
            FieldInfoData {
                name: "ZoomInAimingHelpActive",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimDynamicState, zoom_in_aiming_help_active),
            },
            FieldInfoData {
                name: "AimAssistOptionEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimDynamicState, aim_assist_option_enabled),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(AimingSimDynamicState, field_flag_changed0),
            },
            FieldInfoData {
                name: "FieldFlagChanged1",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(AimingSimDynamicState, field_flag_changed1),
            },
        ],
    }),
    array_type: Some(AIMINGSIMDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AimingSimDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        AIMINGSIMDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static AIMINGSIMDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingSimDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("AimingSimDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AimingSimStaticState {
    pub local_player_id: super::core::LocalPlayerId,
    pub yaw_input_action: i32,
    pub pitch_input_action: i32,
    pub field_flag_changed0: u8,
}

pub trait AimingSimStaticStateTrait: TypeObject {
    fn local_player_id(&self) -> &super::core::LocalPlayerId;
    fn yaw_input_action(&self) -> &i32;
    fn pitch_input_action(&self) -> &i32;
    fn field_flag_changed0(&self) -> &u8;
}

impl AimingSimStaticStateTrait for AimingSimStaticState {
    fn local_player_id(&self) -> &super::core::LocalPlayerId {
        &self.local_player_id
    }
    fn yaw_input_action(&self) -> &i32 {
        &self.yaw_input_action
    }
    fn pitch_input_action(&self) -> &i32 {
        &self.pitch_input_action
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static AIMINGSIMSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingSimStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AimingSimStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "LocalPlayerId",
                flags: MemberInfoFlags::new(0),
                field_type: "LocalPlayerId",
                rust_offset: offset_of!(AimingSimStaticState, local_player_id),
            },
            FieldInfoData {
                name: "YawInputAction",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AimingSimStaticState, yaw_input_action),
            },
            FieldInfoData {
                name: "PitchInputAction",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AimingSimStaticState, pitch_input_action),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(AimingSimStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(AIMINGSIMSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for AimingSimStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        AIMINGSIMSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static AIMINGSIMSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingSimStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("AimingSimStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AimingConstraints {
    pub min_yaw: f32,
    pub max_yaw: f32,
    pub min_pitch: f32,
    pub max_pitch: f32,
    pub pitch_offset: f32,
    pub yaw_offset: f32,
}

pub trait AimingConstraintsTrait: TypeObject {
    fn min_yaw(&self) -> &f32;
    fn max_yaw(&self) -> &f32;
    fn min_pitch(&self) -> &f32;
    fn max_pitch(&self) -> &f32;
    fn pitch_offset(&self) -> &f32;
    fn yaw_offset(&self) -> &f32;
}

impl AimingConstraintsTrait for AimingConstraints {
    fn min_yaw(&self) -> &f32 {
        &self.min_yaw
    }
    fn max_yaw(&self) -> &f32 {
        &self.max_yaw
    }
    fn min_pitch(&self) -> &f32 {
        &self.min_pitch
    }
    fn max_pitch(&self) -> &f32 {
        &self.max_pitch
    }
    fn pitch_offset(&self) -> &f32 {
        &self.pitch_offset
    }
    fn yaw_offset(&self) -> &f32 {
        &self.yaw_offset
    }
}

pub static AIMINGCONSTRAINTS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingConstraints",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AimingConstraints as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MinYaw",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingConstraints, min_yaw),
            },
            FieldInfoData {
                name: "MaxYaw",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingConstraints, max_yaw),
            },
            FieldInfoData {
                name: "MinPitch",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingConstraints, min_pitch),
            },
            FieldInfoData {
                name: "MaxPitch",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingConstraints, max_pitch),
            },
            FieldInfoData {
                name: "PitchOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingConstraints, pitch_offset),
            },
            FieldInfoData {
                name: "YawOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingConstraints, yaw_offset),
            },
        ],
    }),
    array_type: Some(AIMINGCONSTRAINTS_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for AimingConstraints {
    fn type_info(&self) -> &'static TypeInfo {
        AIMINGCONSTRAINTS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static AIMINGCONSTRAINTS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingConstraints-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("AimingConstraints"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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

pub trait AimingEnvironmentTargetTrait: TypeObject {
    fn position(&self) -> &super::core::Vec3;
    fn snap_position(&self) -> &super::core::Vec3;
    fn velocity(&self) -> &super::core::Vec3;
    fn id(&self) -> &u64;
    fn radius(&self) -> &f32;
    fn snap_radius(&self) -> &f32;
    fn is_sticky(&self) -> &bool;
    fn is_snap(&self) -> &bool;
}

impl AimingEnvironmentTargetTrait for AimingEnvironmentTarget {
    fn position(&self) -> &super::core::Vec3 {
        &self.position
    }
    fn snap_position(&self) -> &super::core::Vec3 {
        &self.snap_position
    }
    fn velocity(&self) -> &super::core::Vec3 {
        &self.velocity
    }
    fn id(&self) -> &u64 {
        &self.id
    }
    fn radius(&self) -> &f32 {
        &self.radius
    }
    fn snap_radius(&self) -> &f32 {
        &self.snap_radius
    }
    fn is_sticky(&self) -> &bool {
        &self.is_sticky
    }
    fn is_snap(&self) -> &bool {
        &self.is_snap
    }
}

pub static AIMINGENVIRONMENTTARGET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingEnvironmentTarget",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AimingEnvironmentTarget as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Position",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AimingEnvironmentTarget, position),
            },
            FieldInfoData {
                name: "SnapPosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AimingEnvironmentTarget, snap_position),
            },
            FieldInfoData {
                name: "Velocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AimingEnvironmentTarget, velocity),
            },
            FieldInfoData {
                name: "Id",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint64",
                rust_offset: offset_of!(AimingEnvironmentTarget, id),
            },
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingEnvironmentTarget, radius),
            },
            FieldInfoData {
                name: "SnapRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingEnvironmentTarget, snap_radius),
            },
            FieldInfoData {
                name: "IsSticky",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingEnvironmentTarget, is_sticky),
            },
            FieldInfoData {
                name: "IsSnap",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingEnvironmentTarget, is_snap),
            },
        ],
    }),
    array_type: Some(AIMINGENVIRONMENTTARGET_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AimingEnvironmentTarget {
    fn type_info(&self) -> &'static TypeInfo {
        AIMINGENVIRONMENTTARGET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static AIMINGENVIRONMENTTARGET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingEnvironmentTarget-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("AimingEnvironmentTarget"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum AimOverrideMode {
    #[default]
    AimOverrideMode_DisableOverride = 0,
    AimOverrideMode_Blend = 1,
    AimOverrideMode_Force = 2,
}

pub static AIMOVERRIDEMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimOverrideMode",
    flags: MemberInfoFlags::new(49429),
    module: "Soldier",
    data: TypeInfoData::Enum,
    array_type: Some(AIMOVERRIDEMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AimOverrideMode {
    fn type_info(&self) -> &'static TypeInfo {
        AIMOVERRIDEMODE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static AIMOVERRIDEMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimOverrideMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("AimOverrideMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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

pub trait SoldierThirdPersonCameraRenderStateTrait: TypeObject {
    fn aimer_position(&self) -> &super::core::Vec3;
    fn hit_position(&self) -> &super::core::Vec3;
    fn yaw(&self) -> &f32;
    fn pitch(&self) -> &f32;
    fn arm_length(&self) -> &f32;
    fn previous_arm_length(&self) -> &f32;
    fn previous_collided_arm_length(&self) -> &f32;
    fn is_colliding(&self) -> &bool;
}

impl SoldierThirdPersonCameraRenderStateTrait for SoldierThirdPersonCameraRenderState {
    fn aimer_position(&self) -> &super::core::Vec3 {
        &self.aimer_position
    }
    fn hit_position(&self) -> &super::core::Vec3 {
        &self.hit_position
    }
    fn yaw(&self) -> &f32 {
        &self.yaw
    }
    fn pitch(&self) -> &f32 {
        &self.pitch
    }
    fn arm_length(&self) -> &f32 {
        &self.arm_length
    }
    fn previous_arm_length(&self) -> &f32 {
        &self.previous_arm_length
    }
    fn previous_collided_arm_length(&self) -> &f32 {
        &self.previous_collided_arm_length
    }
    fn is_colliding(&self) -> &bool {
        &self.is_colliding
    }
}

pub static SOLDIERTHIRDPERSONCAMERARENDERSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoldierThirdPersonCameraRenderState",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SoldierThirdPersonCameraRenderState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "AimerPosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SoldierThirdPersonCameraRenderState, aimer_position),
            },
            FieldInfoData {
                name: "HitPosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SoldierThirdPersonCameraRenderState, hit_position),
            },
            FieldInfoData {
                name: "Yaw",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierThirdPersonCameraRenderState, yaw),
            },
            FieldInfoData {
                name: "Pitch",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierThirdPersonCameraRenderState, pitch),
            },
            FieldInfoData {
                name: "ArmLength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierThirdPersonCameraRenderState, arm_length),
            },
            FieldInfoData {
                name: "PreviousArmLength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierThirdPersonCameraRenderState, previous_arm_length),
            },
            FieldInfoData {
                name: "PreviousCollidedArmLength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierThirdPersonCameraRenderState, previous_collided_arm_length),
            },
            FieldInfoData {
                name: "IsColliding",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SoldierThirdPersonCameraRenderState, is_colliding),
            },
        ],
    }),
    array_type: Some(SOLDIERTHIRDPERSONCAMERARENDERSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SoldierThirdPersonCameraRenderState {
    fn type_info(&self) -> &'static TypeInfo {
        SOLDIERTHIRDPERSONCAMERARENDERSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SOLDIERTHIRDPERSONCAMERARENDERSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoldierThirdPersonCameraRenderState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("SoldierThirdPersonCameraRenderState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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

pub trait SoldierThirdPersonCameraSimStateTrait: TypeObject {
    fn local_transform(&self) -> &super::core::LinearTransform;
    fn free_transform(&self) -> &super::core::LinearTransform;
    fn procedural_transform(&self) -> &super::core::LinearTransform;
    fn shake_transform(&self) -> &super::core::LinearTransform;
    fn roll_transform(&self) -> &super::core::LinearTransform;
    fn sim_aimer_position(&self) -> &super::core::Vec3;
    fn sim_hit_position(&self) -> &super::core::Vec3;
    fn aiming(&self) -> &AimingHandle;
    fn sim_yaw(&self) -> &f32;
    fn sim_pitch(&self) -> &f32;
    fn max_pitch(&self) -> &f32;
    fn arm_length(&self) -> &f32;
    fn min_reduce_pitch(&self) -> &f32;
    fn max_reduce_pitch(&self) -> &f32;
    fn max_reduced_length(&self) -> &f32;
    fn collision_width_padding(&self) -> &f32;
    fn collision_blend_in(&self) -> &f32;
    fn collision_blend_out(&self) -> &f32;
    fn free_transform_blend_value(&self) -> &f32;
    fn near_plane(&self) -> &f32;
    fn far_plane(&self) -> &f32;
    fn fov(&self) -> &f32;
    fn aspect_ratio(&self) -> &f32;
    fn reduce_arm_length_looking_up(&self) -> &bool;
    fn field_flag_changed0(&self) -> &u32;
}

impl SoldierThirdPersonCameraSimStateTrait for SoldierThirdPersonCameraSimState {
    fn local_transform(&self) -> &super::core::LinearTransform {
        &self.local_transform
    }
    fn free_transform(&self) -> &super::core::LinearTransform {
        &self.free_transform
    }
    fn procedural_transform(&self) -> &super::core::LinearTransform {
        &self.procedural_transform
    }
    fn shake_transform(&self) -> &super::core::LinearTransform {
        &self.shake_transform
    }
    fn roll_transform(&self) -> &super::core::LinearTransform {
        &self.roll_transform
    }
    fn sim_aimer_position(&self) -> &super::core::Vec3 {
        &self.sim_aimer_position
    }
    fn sim_hit_position(&self) -> &super::core::Vec3 {
        &self.sim_hit_position
    }
    fn aiming(&self) -> &AimingHandle {
        &self.aiming
    }
    fn sim_yaw(&self) -> &f32 {
        &self.sim_yaw
    }
    fn sim_pitch(&self) -> &f32 {
        &self.sim_pitch
    }
    fn max_pitch(&self) -> &f32 {
        &self.max_pitch
    }
    fn arm_length(&self) -> &f32 {
        &self.arm_length
    }
    fn min_reduce_pitch(&self) -> &f32 {
        &self.min_reduce_pitch
    }
    fn max_reduce_pitch(&self) -> &f32 {
        &self.max_reduce_pitch
    }
    fn max_reduced_length(&self) -> &f32 {
        &self.max_reduced_length
    }
    fn collision_width_padding(&self) -> &f32 {
        &self.collision_width_padding
    }
    fn collision_blend_in(&self) -> &f32 {
        &self.collision_blend_in
    }
    fn collision_blend_out(&self) -> &f32 {
        &self.collision_blend_out
    }
    fn free_transform_blend_value(&self) -> &f32 {
        &self.free_transform_blend_value
    }
    fn near_plane(&self) -> &f32 {
        &self.near_plane
    }
    fn far_plane(&self) -> &f32 {
        &self.far_plane
    }
    fn fov(&self) -> &f32 {
        &self.fov
    }
    fn aspect_ratio(&self) -> &f32 {
        &self.aspect_ratio
    }
    fn reduce_arm_length_looking_up(&self) -> &bool {
        &self.reduce_arm_length_looking_up
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
}

pub static SOLDIERTHIRDPERSONCAMERASIMSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoldierThirdPersonCameraSimState",
    flags: MemberInfoFlags::new(73),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SoldierThirdPersonCameraSimState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "LocalTransform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, local_transform),
            },
            FieldInfoData {
                name: "FreeTransform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, free_transform),
            },
            FieldInfoData {
                name: "ProceduralTransform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, procedural_transform),
            },
            FieldInfoData {
                name: "ShakeTransform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, shake_transform),
            },
            FieldInfoData {
                name: "RollTransform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, roll_transform),
            },
            FieldInfoData {
                name: "SimAimerPosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, sim_aimer_position),
            },
            FieldInfoData {
                name: "SimHitPosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, sim_hit_position),
            },
            FieldInfoData {
                name: "Aiming",
                flags: MemberInfoFlags::new(0),
                field_type: "AimingHandle",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, aiming),
            },
            FieldInfoData {
                name: "SimYaw",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, sim_yaw),
            },
            FieldInfoData {
                name: "SimPitch",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, sim_pitch),
            },
            FieldInfoData {
                name: "MaxPitch",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, max_pitch),
            },
            FieldInfoData {
                name: "ArmLength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, arm_length),
            },
            FieldInfoData {
                name: "MinReducePitch",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, min_reduce_pitch),
            },
            FieldInfoData {
                name: "MaxReducePitch",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, max_reduce_pitch),
            },
            FieldInfoData {
                name: "MaxReducedLength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, max_reduced_length),
            },
            FieldInfoData {
                name: "CollisionWidthPadding",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, collision_width_padding),
            },
            FieldInfoData {
                name: "CollisionBlendIn",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, collision_blend_in),
            },
            FieldInfoData {
                name: "CollisionBlendOut",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, collision_blend_out),
            },
            FieldInfoData {
                name: "FreeTransformBlendValue",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, free_transform_blend_value),
            },
            FieldInfoData {
                name: "NearPlane",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, near_plane),
            },
            FieldInfoData {
                name: "FarPlane",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, far_plane),
            },
            FieldInfoData {
                name: "Fov",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, fov),
            },
            FieldInfoData {
                name: "AspectRatio",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, aspect_ratio),
            },
            FieldInfoData {
                name: "ReduceArmLengthLookingUp",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, reduce_arm_length_looking_up),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SOLDIERTHIRDPERSONCAMERASIMSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SoldierThirdPersonCameraSimState {
    fn type_info(&self) -> &'static TypeInfo {
        SOLDIERTHIRDPERSONCAMERASIMSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SOLDIERTHIRDPERSONCAMERASIMSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoldierThirdPersonCameraSimState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("SoldierThirdPersonCameraSimState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SoldierFirstPersonCameraRenderState {
    pub yaw: f32,
    pub pitch: f32,
}

pub trait SoldierFirstPersonCameraRenderStateTrait: TypeObject {
    fn yaw(&self) -> &f32;
    fn pitch(&self) -> &f32;
}

impl SoldierFirstPersonCameraRenderStateTrait for SoldierFirstPersonCameraRenderState {
    fn yaw(&self) -> &f32 {
        &self.yaw
    }
    fn pitch(&self) -> &f32 {
        &self.pitch
    }
}

pub static SOLDIERFIRSTPERSONCAMERARENDERSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoldierFirstPersonCameraRenderState",
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SoldierFirstPersonCameraRenderState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Yaw",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierFirstPersonCameraRenderState, yaw),
            },
            FieldInfoData {
                name: "Pitch",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierFirstPersonCameraRenderState, pitch),
            },
        ],
    }),
    array_type: Some(SOLDIERFIRSTPERSONCAMERARENDERSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for SoldierFirstPersonCameraRenderState {
    fn type_info(&self) -> &'static TypeInfo {
        SOLDIERFIRSTPERSONCAMERARENDERSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SOLDIERFIRSTPERSONCAMERARENDERSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoldierFirstPersonCameraRenderState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("SoldierFirstPersonCameraRenderState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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

pub trait SoldierFirstPersonCameraSimStateTrait: TypeObject {
    fn camera_bone_local_transform(&self) -> &super::core::LinearTransform;
    fn camera_bone_transform_relative_to_trajectory(&self) -> &super::core::LinearTransform;
    fn roll_transform(&self) -> &super::core::LinearTransform;
    fn entity_space_local_transform(&self) -> &super::core::LinearTransform;
    fn procedural_transform(&self) -> &super::core::LinearTransform;
    fn shake_transform(&self) -> &super::core::LinearTransform;
    fn spine_x_relative_to_camera(&self) -> &super::core::LinearTransform;
    fn weapon_sway_transform(&self) -> &super::core::LinearTransform;
    fn soldier_world_position(&self) -> &super::core::Vec3;
    fn local_eye_position(&self) -> &super::core::Vec3;
    fn aiming(&self) -> &AimingHandle;
    fn soldier_transform_space(&self) -> &super::state_stream::TransformSpaceHandle;
    fn spine_x_bone_index(&self) -> &u32;
    fn trajectory_bone_index(&self) -> &u32;
    fn sim_yaw(&self) -> &f32;
    fn sim_pitch(&self) -> &f32;
    fn spine_x_factor(&self) -> &f32;
    fn animated_camera_factor(&self) -> &f32;
    fn animated_camera_start_pitch(&self) -> &f32;
    fn prevent_ground_clipping_distance(&self) -> &f32;
    fn has_valid_animation_transforms(&self) -> &bool;
    fn use_local_eye_position1p(&self) -> &bool;
    fn is_animated_camera(&self) -> &bool;
    fn has_entity_space(&self) -> &bool;
    fn field_flag_changed0(&self) -> &u32;
}

impl SoldierFirstPersonCameraSimStateTrait for SoldierFirstPersonCameraSimState {
    fn camera_bone_local_transform(&self) -> &super::core::LinearTransform {
        &self.camera_bone_local_transform
    }
    fn camera_bone_transform_relative_to_trajectory(&self) -> &super::core::LinearTransform {
        &self.camera_bone_transform_relative_to_trajectory
    }
    fn roll_transform(&self) -> &super::core::LinearTransform {
        &self.roll_transform
    }
    fn entity_space_local_transform(&self) -> &super::core::LinearTransform {
        &self.entity_space_local_transform
    }
    fn procedural_transform(&self) -> &super::core::LinearTransform {
        &self.procedural_transform
    }
    fn shake_transform(&self) -> &super::core::LinearTransform {
        &self.shake_transform
    }
    fn spine_x_relative_to_camera(&self) -> &super::core::LinearTransform {
        &self.spine_x_relative_to_camera
    }
    fn weapon_sway_transform(&self) -> &super::core::LinearTransform {
        &self.weapon_sway_transform
    }
    fn soldier_world_position(&self) -> &super::core::Vec3 {
        &self.soldier_world_position
    }
    fn local_eye_position(&self) -> &super::core::Vec3 {
        &self.local_eye_position
    }
    fn aiming(&self) -> &AimingHandle {
        &self.aiming
    }
    fn soldier_transform_space(&self) -> &super::state_stream::TransformSpaceHandle {
        &self.soldier_transform_space
    }
    fn spine_x_bone_index(&self) -> &u32 {
        &self.spine_x_bone_index
    }
    fn trajectory_bone_index(&self) -> &u32 {
        &self.trajectory_bone_index
    }
    fn sim_yaw(&self) -> &f32 {
        &self.sim_yaw
    }
    fn sim_pitch(&self) -> &f32 {
        &self.sim_pitch
    }
    fn spine_x_factor(&self) -> &f32 {
        &self.spine_x_factor
    }
    fn animated_camera_factor(&self) -> &f32 {
        &self.animated_camera_factor
    }
    fn animated_camera_start_pitch(&self) -> &f32 {
        &self.animated_camera_start_pitch
    }
    fn prevent_ground_clipping_distance(&self) -> &f32 {
        &self.prevent_ground_clipping_distance
    }
    fn has_valid_animation_transforms(&self) -> &bool {
        &self.has_valid_animation_transforms
    }
    fn use_local_eye_position1p(&self) -> &bool {
        &self.use_local_eye_position1p
    }
    fn is_animated_camera(&self) -> &bool {
        &self.is_animated_camera
    }
    fn has_entity_space(&self) -> &bool {
        &self.has_entity_space
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
}

pub static SOLDIERFIRSTPERSONCAMERASIMSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoldierFirstPersonCameraSimState",
    flags: MemberInfoFlags::new(73),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SoldierFirstPersonCameraSimState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "CameraBoneLocalTransform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, camera_bone_local_transform),
            },
            FieldInfoData {
                name: "CameraBoneTransformRelativeToTrajectory",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, camera_bone_transform_relative_to_trajectory),
            },
            FieldInfoData {
                name: "RollTransform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, roll_transform),
            },
            FieldInfoData {
                name: "EntitySpaceLocalTransform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, entity_space_local_transform),
            },
            FieldInfoData {
                name: "ProceduralTransform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, procedural_transform),
            },
            FieldInfoData {
                name: "ShakeTransform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, shake_transform),
            },
            FieldInfoData {
                name: "SpineXRelativeToCamera",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, spine_x_relative_to_camera),
            },
            FieldInfoData {
                name: "WeaponSwayTransform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, weapon_sway_transform),
            },
            FieldInfoData {
                name: "SoldierWorldPosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, soldier_world_position),
            },
            FieldInfoData {
                name: "LocalEyePosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, local_eye_position),
            },
            FieldInfoData {
                name: "Aiming",
                flags: MemberInfoFlags::new(0),
                field_type: "AimingHandle",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, aiming),
            },
            FieldInfoData {
                name: "SoldierTransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: "TransformSpaceHandle",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, soldier_transform_space),
            },
            FieldInfoData {
                name: "SpineXBoneIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, spine_x_bone_index),
            },
            FieldInfoData {
                name: "TrajectoryBoneIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, trajectory_bone_index),
            },
            FieldInfoData {
                name: "SimYaw",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, sim_yaw),
            },
            FieldInfoData {
                name: "SimPitch",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, sim_pitch),
            },
            FieldInfoData {
                name: "SpineXFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, spine_x_factor),
            },
            FieldInfoData {
                name: "AnimatedCameraFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, animated_camera_factor),
            },
            FieldInfoData {
                name: "AnimatedCameraStartPitch",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, animated_camera_start_pitch),
            },
            FieldInfoData {
                name: "PreventGroundClippingDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, prevent_ground_clipping_distance),
            },
            FieldInfoData {
                name: "HasValidAnimationTransforms",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, has_valid_animation_transforms),
            },
            FieldInfoData {
                name: "UseLocalEyePosition1p",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, use_local_eye_position1p),
            },
            FieldInfoData {
                name: "IsAnimatedCamera",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, is_animated_camera),
            },
            FieldInfoData {
                name: "HasEntitySpace",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, has_entity_space),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SOLDIERFIRSTPERSONCAMERASIMSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SoldierFirstPersonCameraSimState {
    fn type_info(&self) -> &'static TypeInfo {
        SOLDIERFIRSTPERSONCAMERASIMSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SOLDIERFIRSTPERSONCAMERASIMSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoldierFirstPersonCameraSimState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("SoldierFirstPersonCameraSimState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SoldierServerPlayerExtent {
    pub _glacier_base: super::game_server::ServerGamePlayerInternalExtent,
}

pub trait SoldierServerPlayerExtentTrait: super::game_server::ServerGamePlayerInternalExtentTrait {
}

impl SoldierServerPlayerExtentTrait for SoldierServerPlayerExtent {
}

impl super::game_server::ServerGamePlayerInternalExtentTrait for SoldierServerPlayerExtent {
}

impl super::game_server::ServerPlayerExtentTrait for SoldierServerPlayerExtent {
}

pub static SOLDIERSERVERPLAYEREXTENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoldierServerPlayerExtent",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_server::SERVERGAMEPLAYERINTERNALEXTENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SoldierServerPlayerExtent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SOLDIERSERVERPLAYEREXTENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SoldierServerPlayerExtent {
    fn type_info(&self) -> &'static TypeInfo {
        SOLDIERSERVERPLAYEREXTENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SOLDIERSERVERPLAYEREXTENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoldierServerPlayerExtent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("SoldierServerPlayerExtent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerLookAtTriggerEntity {
    pub _glacier_base: super::game_server::ServerCharacterLookAtTriggerEntity,
}

pub trait ServerLookAtTriggerEntityTrait: super::game_server::ServerCharacterLookAtTriggerEntityTrait {
}

impl ServerLookAtTriggerEntityTrait for ServerLookAtTriggerEntity {
}

impl super::game_server::ServerCharacterLookAtTriggerEntityTrait for ServerLookAtTriggerEntity {
}

impl super::game_server::ServerTriggerEntityTrait for ServerLookAtTriggerEntity {
}

impl super::entity::EntityTrait for ServerLookAtTriggerEntity {
}

impl super::entity::EntityBusPeerTrait for ServerLookAtTriggerEntity {
}

pub static SERVERLOOKATTRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLookAtTriggerEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_server::SERVERCHARACTERLOOKATTRIGGERENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerLookAtTriggerEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERLOOKATTRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerLookAtTriggerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERLOOKATTRIGGERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERLOOKATTRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLookAtTriggerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerLookAtTriggerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerCharacterStateTriggerEntity {
    pub _glacier_base: super::game_server::ServerTriggerEntity,
}

pub trait ServerCharacterStateTriggerEntityTrait: super::game_server::ServerTriggerEntityTrait {
}

impl ServerCharacterStateTriggerEntityTrait for ServerCharacterStateTriggerEntity {
}

impl super::game_server::ServerTriggerEntityTrait for ServerCharacterStateTriggerEntity {
}

impl super::entity::EntityTrait for ServerCharacterStateTriggerEntity {
}

impl super::entity::EntityBusPeerTrait for ServerCharacterStateTriggerEntity {
}

pub static SERVERCHARACTERSTATETRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterStateTriggerEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_server::SERVERTRIGGERENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCharacterStateTriggerEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCHARACTERSTATETRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCharacterStateTriggerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCHARACTERSTATETRIGGERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERCHARACTERSTATETRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterStateTriggerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerCharacterStateTriggerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerWeaponStateEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerWeaponStateEntityTrait: super::entity::EntityTrait {
}

impl ServerWeaponStateEntityTrait for ServerWeaponStateEntity {
}

impl super::entity::EntityTrait for ServerWeaponStateEntity {
}

impl super::entity::EntityBusPeerTrait for ServerWeaponStateEntity {
}

pub static SERVERWEAPONSTATEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponStateEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerWeaponStateEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERWEAPONSTATEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerWeaponStateEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERWEAPONSTATEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERWEAPONSTATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponStateEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerWeaponStateEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerStateEventGateEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerStateEventGateEntityTrait: super::entity::EntityTrait {
}

impl ServerStateEventGateEntityTrait for ServerStateEventGateEntity {
}

impl super::entity::EntityTrait for ServerStateEventGateEntity {
}

impl super::entity::EntityBusPeerTrait for ServerStateEventGateEntity {
}

pub static SERVERSTATEEVENTGATEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStateEventGateEntity",
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerStateEventGateEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSTATEEVENTGATEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerStateEventGateEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSTATEEVENTGATEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERSTATEEVENTGATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStateEventGateEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerStateEventGateEntity"),
    array_type: None,
    alignment: 8,
};


