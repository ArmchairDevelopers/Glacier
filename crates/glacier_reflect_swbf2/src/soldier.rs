use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
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

#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1136875890,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientWeaponStateEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientWeaponStateEntity as Default>::default())),
            create_boxed: || Box::new(<ClientWeaponStateEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTWEAPONSTATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponStateEntity-Array",
    name_hash: 3748286534,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientWeaponStateEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 4142713990,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientPlayerLookAtEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPlayerLookAtEntity as Default>::default())),
            create_boxed: || Box::new(<ClientPlayerLookAtEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTPLAYERLOOKATENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerLookAtEntity-Array",
    name_hash: 2097068466,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientPlayerLookAtEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2692729809,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENTENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientTripwireEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientTripwireEntity as Default>::default())),
            create_boxed: || Box::new(<ClientTripwireEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTTRIPWIREENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTripwireEntity-Array",
    name_hash: 3570523621,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientTripwireEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2370626990,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientTriggerMoveEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientTriggerMoveEntity as Default>::default())),
            create_boxed: || Box::new(<ClientTriggerMoveEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTTRIGGERMOVEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientTriggerMoveEntity-Array",
    name_hash: 553160218,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientTriggerMoveEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 694677302,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientSoldierBreathControlComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSoldierBreathControlComponent as Default>::default())),
            create_boxed: || Box::new(<ClientSoldierBreathControlComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTSOLDIERBREATHCONTROLCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierBreathControlComponent-Array",
    name_hash: 4222100354,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierBreathControlComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3279702441,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientSoldierBodyComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSoldierBodyComponent as Default>::default())),
            create_boxed: || Box::new(<ClientSoldierBodyComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTSOLDIERBODYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierBodyComponent-Array",
    name_hash: 3615625245,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierBodyComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 174335884,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::PHYSICSCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientPickupPhysicsComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPickupPhysicsComponent as Default>::default())),
            create_boxed: || Box::new(<ClientPickupPhysicsComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTPICKUPPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPickupPhysicsComponent-Array",
    name_hash: 2854320056,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientPickupPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3372437970,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientPhantomComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPhantomComponent as Default>::default())),
            create_boxed: || Box::new(<ClientPhantomComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTPHANTOMCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPhantomComponent-Array",
    name_hash: 4283926886,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientPhantomComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 484955792,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientMovementComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientMovementComponent as Default>::default())),
            create_boxed: || Box::new(<ClientMovementComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTMOVEMENTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMovementComponent-Array",
    name_hash: 3460310180,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientMovementComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2236544143,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientHitReactionComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientHitReactionComponent as Default>::default())),
            create_boxed: || Box::new(<ClientHitReactionComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTHITREACTIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientHitReactionComponent-Array",
    name_hash: 3845476667,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientHitReactionComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2035885129,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientFaceposerComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientFaceposerComponent as Default>::default())),
            create_boxed: || Box::new(<ClientFaceposerComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTFACEPOSERCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientFaceposerComponent-Array",
    name_hash: 559426173,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientFaceposerComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3031033227,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientBoneCollisionComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientBoneCollisionComponent as Default>::default())),
            create_boxed: || Box::new(<ClientBoneCollisionComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTBONECOLLISIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBoneCollisionComponent-Array",
    name_hash: 570512447,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientBoneCollisionComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1748180048,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientBlockAimAssistComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientBlockAimAssistComponent as Default>::default())),
            create_boxed: || Box::new(<ClientBlockAimAssistComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTBLOCKAIMASSISTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBlockAimAssistComponent-Array",
    name_hash: 1553775332,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientBlockAimAssistComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 4191795596,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientEntryAimAssistTargetOptionsComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientEntryAimAssistTargetOptionsComponent as Default>::default())),
            create_boxed: || Box::new(<ClientEntryAimAssistTargetOptionsComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTENTRYAIMASSISTTARGETOPTIONSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEntryAimAssistTargetOptionsComponent-Array",
    name_hash: 1409059256,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientEntryAimAssistTargetOptionsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3091926233,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientAimAssistNodeSnapPointComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAimAssistNodeSnapPointComponent as Default>::default())),
            create_boxed: || Box::new(<ClientAimAssistNodeSnapPointComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTAIMASSISTNODESNAPPOINTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAimAssistNodeSnapPointComponent-Array",
    name_hash: 617167597,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientAimAssistNodeSnapPointComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 4171495385,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientAimAssistNodeComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAimAssistNodeComponent as Default>::default())),
            create_boxed: || Box::new(<ClientAimAssistNodeComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTAIMASSISTNODECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAimAssistNodeComponent-Array",
    name_hash: 4242957805,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientAimAssistNodeComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1574896726,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_sim::CAMERA_TYPE_INFO),
        super_class_offset: offset_of!(SoldierCamera, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SoldierCamera as Default>::default())),
            create_boxed: || Box::new(<SoldierCamera as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SOLDIERCAMERA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoldierCamera-Array",
    name_hash: 688835042,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("SoldierCamera"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3126077455,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientWeaponLagEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientWeaponLagEntity as Default>::default())),
            create_boxed: || Box::new(<ClientWeaponLagEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTWEAPONLAGENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponLagEntity-Array",
    name_hash: 1630741691,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientWeaponLagEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3561874097,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientCoverPeekEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCoverPeekEntity as Default>::default())),
            create_boxed: || Box::new(<ClientCoverPeekEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTCOVERPEEKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCoverPeekEntity-Array",
    name_hash: 2624967941,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientCoverPeekEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 156232179,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientOcclutionQueryComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientOcclutionQueryComponent as Default>::default())),
            create_boxed: || Box::new(<ClientOcclutionQueryComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTOCCLUTIONQUERYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientOcclutionQueryComponent-Array",
    name_hash: 3291268551,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientOcclutionQueryComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3965987185,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientVoiceOverObjectReaderWatcherEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientVoiceOverObjectReaderWatcherEntity as Default>::default())),
            create_boxed: || Box::new(<ClientVoiceOverObjectReaderWatcherEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTVOICEOVEROBJECTREADERWATCHERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverObjectReaderWatcherEntity-Array",
    name_hash: 514223173,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientVoiceOverObjectReaderWatcherEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3724869135,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientVoiceOverObjectReaderEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientVoiceOverObjectReaderEntity as Default>::default())),
            create_boxed: || Box::new(<ClientVoiceOverObjectReaderEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTVOICEOVEROBJECTREADERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVoiceOverObjectReaderEntity-Array",
    name_hash: 3081534139,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientVoiceOverObjectReaderEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1371010964,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerSoldierWeaponsComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierWeaponsComponent as Default>::default())),
            create_boxed: || Box::new(<ServerSoldierWeaponsComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERSOLDIERWEAPONSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierWeaponsComponent-Array",
    name_hash: 3257185184,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerSoldierWeaponsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2129701934,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_server::SERVERCHARACTERMASTERPHYSICSCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerSoldierPhysicsComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierPhysicsComponent as Default>::default())),
            create_boxed: || Box::new(<ServerSoldierPhysicsComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERSOLDIERPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierPhysicsComponent-Array",
    name_hash: 2297902234,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerSoldierPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 4079602393,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_server::SERVERCHARACTERHEALTHCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerSoldierHealthComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierHealthComponent as Default>::default())),
            create_boxed: || Box::new(<ServerSoldierHealthComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERSOLDIERHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierHealthComponent-Array",
    name_hash: 1195276525,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerSoldierHealthComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2161190680,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_server::SERVERCHARACTERCUSTOMIZATIONCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerSoldierCustomizationComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierCustomizationComponent as Default>::default())),
            create_boxed: || Box::new(<ServerSoldierCustomizationComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERSOLDIERCUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierCustomizationComponent-Array",
    name_hash: 2944610604,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerSoldierCustomizationComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3834316124,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerSoldierCameraComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierCameraComponent as Default>::default())),
            create_boxed: || Box::new(<ServerSoldierCameraComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERSOLDIERCAMERACOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierCameraComponent-Array",
    name_hash: 2738777576,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerSoldierCameraComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 82929141,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerSoldierBodyComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierBodyComponent as Default>::default())),
            create_boxed: || Box::new(<ServerSoldierBodyComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERSOLDIERBODYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierBodyComponent-Array",
    name_hash: 4052661953,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerSoldierBodyComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1730557136,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::PHYSICSCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerPickupPhysicsComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPickupPhysicsComponent as Default>::default())),
            create_boxed: || Box::new(<ServerPickupPhysicsComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERPICKUPPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPickupPhysicsComponent-Array",
    name_hash: 2023764836,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerPickupPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2024848332,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerMovementComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerMovementComponent as Default>::default())),
            create_boxed: || Box::new(<ServerMovementComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERMOVEMENTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMovementComponent-Array",
    name_hash: 2480946296,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerMovementComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3140380627,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerHitReactionComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerHitReactionComponent as Default>::default())),
            create_boxed: || Box::new(<ServerHitReactionComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERHITREACTIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerHitReactionComponent-Array",
    name_hash: 3501306599,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerHitReactionComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3122913239,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerBoneCollisionComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerBoneCollisionComponent as Default>::default())),
            create_boxed: || Box::new(<ServerBoneCollisionComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERBONECOLLISIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBoneCollisionComponent-Array",
    name_hash: 3777026275,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerBoneCollisionComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3847673323,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientAimingScaleDataProviderEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAimingScaleDataProviderEntity as Default>::default())),
            create_boxed: || Box::new(<ClientAimingScaleDataProviderEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTAIMINGSCALEDATAPROVIDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAimingScaleDataProviderEntity-Array",
    name_hash: 799098335,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientAimingScaleDataProviderEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3592869107,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientAimingAngularSpeedConstraintDataProviderEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAimingAngularSpeedConstraintDataProviderEntity as Default>::default())),
            create_boxed: || Box::new(<ClientAimingAngularSpeedConstraintDataProviderEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTAIMINGANGULARSPEEDCONSTRAINTDATAPROVIDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAimingAngularSpeedConstraintDataProviderEntity-Array",
    name_hash: 256372935,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientAimingAngularSpeedConstraintDataProviderEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1975603196,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::weapon::CLIENTWEAPON_TYPE_INFO),
        super_class_offset: offset_of!(ClientZeroingWeapon, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientZeroingWeapon as Default>::default())),
            create_boxed: || Box::new(<ClientZeroingWeapon as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTZEROINGWEAPON_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientZeroingWeapon-Array",
    name_hash: 2562324424,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientZeroingWeapon"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1361891242,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientSoldierWeaponSocketEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSoldierWeaponSocketEntity as Default>::default())),
            create_boxed: || Box::new(<ClientSoldierWeaponSocketEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTSOLDIERWEAPONSOCKETENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierWeaponSocketEntity-Array",
    name_hash: 2858437150,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierWeaponSocketEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientSoldierWeaponSpawnInfo {
}

pub trait ClientSoldierWeaponSpawnInfoTrait: TypeObject {
}

impl ClientSoldierWeaponSpawnInfoTrait for ClientSoldierWeaponSpawnInfo {
}

pub static CLIENTSOLDIERWEAPONSPAWNINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierWeaponSpawnInfo",
    name_hash: 1730742913,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSoldierWeaponSpawnInfo as Default>::default())),
            create_boxed: || Box::new(<ClientSoldierWeaponSpawnInfo as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static CLIENTSOLDIERWEAPONSPAWNINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierWeaponSpawnInfo-Array",
    name_hash: 3318768693,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierWeaponSpawnInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3867470964,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENTENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientSoldierWeapon, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSoldierWeapon as Default>::default())),
            create_boxed: || Box::new(<ClientSoldierWeapon as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTSOLDIERWEAPON_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierWeapon-Array",
    name_hash: 123836480,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierWeapon"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1821773889,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientAnimationTurretRotationComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAnimationTurretRotationComponent as Default>::default())),
            create_boxed: || Box::new(<ClientAnimationTurretRotationComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTANIMATIONTURRETROTATIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAnimationTurretRotationComponent-Array",
    name_hash: 4279679605,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientAnimationTurretRotationComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 4118533493,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::weapon::CLIENTPROXYPROJECTILEENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientProxyGrenadeEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientProxyGrenadeEntity as Default>::default())),
            create_boxed: || Box::new(<ClientProxyGrenadeEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTPROXYGRENADEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientProxyGrenadeEntity-Array",
    name_hash: 2668545601,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientProxyGrenadeEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1851639527,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::weapon::CLIENTPROXYPROJECTILEENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientProxyExplosionPackEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientProxyExplosionPackEntity as Default>::default())),
            create_boxed: || Box::new(<ClientProxyExplosionPackEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTPROXYEXPLOSIONPACKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientProxyExplosionPackEntity-Array",
    name_hash: 1461274835,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientProxyExplosionPackEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1547990777,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::weapon::CLIENTGHOSTPROJECTILEENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientGrenadeEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientGrenadeEntity as Default>::default())),
            create_boxed: || Box::new(<ClientGrenadeEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTGRENADEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGrenadeEntity-Array",
    name_hash: 4201264589,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientGrenadeEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3835631467,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::weapon::CLIENTGHOSTPROJECTILEENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientExplosionPackEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientExplosionPackEntity as Default>::default())),
            create_boxed: || Box::new(<ClientExplosionPackEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTEXPLOSIONPACKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientExplosionPackEntity-Array",
    name_hash: 2339678047,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientExplosionPackEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 788138100,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::PARTPHYSICSCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientExplosionPackPhysicsComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientExplosionPackPhysicsComponent as Default>::default())),
            create_boxed: || Box::new(<ClientExplosionPackPhysicsComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTEXPLOSIONPACKPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientExplosionPackPhysicsComponent-Array",
    name_hash: 1916515392,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientExplosionPackPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1969515077,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::PHYSICSCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(DummyExplosionPackPhysicsComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DummyExplosionPackPhysicsComponent as Default>::default())),
            create_boxed: || Box::new(<DummyExplosionPackPhysicsComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DUMMYEXPLOSIONPACKPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DummyExplosionPackPhysicsComponent-Array",
    name_hash: 83903089,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("DummyExplosionPackPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2156992525,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_client::CLIENTGAMEENTRYCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientSoldierEntryComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSoldierEntryComponent as Default>::default())),
            create_boxed: || Box::new(<ClientSoldierEntryComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTSOLDIERENTRYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierEntryComponent-Array",
    name_hash: 3811285945,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierEntryComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2160235475,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientWeaponZeroingComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientWeaponZeroingComponent as Default>::default())),
            create_boxed: || Box::new(<ClientWeaponZeroingComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTWEAPONZEROINGCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWeaponZeroingComponent-Array",
    name_hash: 2008473319,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientWeaponZeroingComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1820876098,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientSoldierWeaponsPreviewComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSoldierWeaponsPreviewComponent as Default>::default())),
            create_boxed: || Box::new(<ClientSoldierWeaponsPreviewComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTSOLDIERWEAPONSPREVIEWCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierWeaponsPreviewComponent-Array",
    name_hash: 3770969334,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierWeaponsPreviewComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2944153672,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientSoldierWeaponsComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSoldierWeaponsComponent as Default>::default())),
            create_boxed: || Box::new(<ClientSoldierWeaponsComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTSOLDIERWEAPONSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierWeaponsComponent-Array",
    name_hash: 137849596,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierWeaponsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3336375794,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_client::CLIENTCHARACTERMASTERPHYSICSCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientSoldierPhysicsComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSoldierPhysicsComponent as Default>::default())),
            create_boxed: || Box::new(<ClientSoldierPhysicsComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTSOLDIERPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierPhysicsComponent-Array",
    name_hash: 18441414,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 4099652485,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_client::CLIENTCHARACTERHEALTHCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientSoldierHealthComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSoldierHealthComponent as Default>::default())),
            create_boxed: || Box::new(<ClientSoldierHealthComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTSOLDIERHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierHealthComponent-Array",
    name_hash: 365134641,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierHealthComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3206063707,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientSoldierFootplantEffectComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSoldierFootplantEffectComponent as Default>::default())),
            create_boxed: || Box::new(<ClientSoldierFootplantEffectComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTSOLDIERFOOTPLANTEFFECTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierFootplantEffectComponent-Array",
    name_hash: 4134546799,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierFootplantEffectComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 625594820,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_client::CLIENTCHARACTERCUSTOMIZATIONCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientSoldierCustomizationComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSoldierCustomizationComponent as Default>::default())),
            create_boxed: || Box::new(<ClientSoldierCustomizationComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTSOLDIERCUSTOMIZATIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierCustomizationComponent-Array",
    name_hash: 3455219312,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierCustomizationComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1289813760,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_client::CLIENTCHARACTERCAMERACOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientSoldierCameraComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSoldierCameraComponent as Default>::default())),
            create_boxed: || Box::new(<ClientSoldierCameraComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTSOLDIERCAMERACOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierCameraComponent-Array",
    name_hash: 469951796,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierCameraComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3061357414,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_sim::TARGETCAMERACALLBACK_TYPE_INFO),
        super_class_offset: offset_of!(ClientSoldierCameraCallback, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSoldierCameraCallback as Default>::default())),
            create_boxed: || Box::new(<ClientSoldierCameraCallback as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTSOLDIERCAMERACALLBACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierCameraCallback-Array",
    name_hash: 2604005458,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierCameraCallback"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 438763501,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_client::CLIENTCHARACTERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientSoldierEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSoldierEntity as Default>::default())),
            create_boxed: || Box::new(<ClientSoldierEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTSOLDIERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierEntity-Array",
    name_hash: 1442604249,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSoldierEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 4144168350,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTAIMENTITYBASE_TYPE_INFO),
        super_class_offset: offset_of!(ClientSchematicsAimEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSchematicsAimEntity as Default>::default())),
            create_boxed: || Box::new(<ClientSchematicsAimEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTSCHEMATICSAIMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSchematicsAimEntity-Array",
    name_hash: 2907164330,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientSchematicsAimEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1908251575,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientReplicatedAimEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientReplicatedAimEntity as Default>::default())),
            create_boxed: || Box::new(<ClientReplicatedAimEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTREPLICATEDAIMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientReplicatedAimEntity-Array",
    name_hash: 3800331523,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientReplicatedAimEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 771592275,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENTENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientPickupEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPickupEntity as Default>::default())),
            create_boxed: || Box::new(<ClientPickupEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTPICKUPENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPickupEntity-Array",
    name_hash: 2036153191,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientPickupEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1056686852,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientBlockAimAssistEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientBlockAimAssistEntity as Default>::default())),
            create_boxed: || Box::new(<ClientBlockAimAssistEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTBLOCKAIMASSISTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBlockAimAssistEntity-Array",
    name_hash: 1902139184,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientBlockAimAssistEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2887763552,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientAimingSimulationDataProviderEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAimingSimulationDataProviderEntity as Default>::default())),
            create_boxed: || Box::new(<ClientAimingSimulationDataProviderEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTAIMINGSIMULATIONDATAPROVIDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAimingSimulationDataProviderEntity-Array",
    name_hash: 1063615572,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientAimingSimulationDataProviderEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1524856215,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientAimEntityBase, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAimEntityBase as Default>::default())),
            create_boxed: || Box::new(<ClientAimEntityBase as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTAIMENTITYBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAimEntityBase-Array",
    name_hash: 1064959523,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientAimEntityBase"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1593453378,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTAIMENTITYBASE_TYPE_INFO),
        super_class_offset: offset_of!(ClientAimEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAimEntity as Default>::default())),
            create_boxed: || Box::new(<ClientAimEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTAIMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAimEntity-Array",
    name_hash: 2161687286,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ClientAimEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2177211668,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerCustomizeSoldierEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCustomizeSoldierEntity as Default>::default())),
            create_boxed: || Box::new(<ServerCustomizeSoldierEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERCUSTOMIZESOLDIERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCustomizeSoldierEntity-Array",
    name_hash: 3459807520,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerCustomizeSoldierEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1505792781,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_server::SERVERGAMECOMPONENTENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerTripwireEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerTripwireEntity as Default>::default())),
            create_boxed: || Box::new(<ServerTripwireEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERTRIPWIREENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerTripwireEntity-Array",
    name_hash: 3737867961,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerTripwireEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2882800433,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_server::SERVERCHARACTERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerSoldierEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierEntity as Default>::default())),
            create_boxed: || Box::new(<ServerSoldierEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERSOLDIERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierEntity-Array",
    name_hash: 387472773,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerSoldierEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3153981675,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerReplicatedAimEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerReplicatedAimEntity as Default>::default())),
            create_boxed: || Box::new(<ServerReplicatedAimEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERREPLICATEDAIMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerReplicatedAimEntity-Array",
    name_hash: 2347802847,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerReplicatedAimEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 373796495,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_server::SERVERGAMECOMPONENTENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerPickupEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPickupEntity as Default>::default())),
            create_boxed: || Box::new(<ServerPickupEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERPICKUPENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPickupEntity-Array",
    name_hash: 542214971,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerPickupEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2415184928,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::weapon::SERVERWEAPON_TYPE_INFO),
        super_class_offset: offset_of!(ServerZeroingWeapon, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerZeroingWeapon as Default>::default())),
            create_boxed: || Box::new(<ServerZeroingWeapon as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERZEROINGWEAPON_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerZeroingWeapon-Array",
    name_hash: 2526794132,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerZeroingWeapon"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerSoldierWeaponSpawnInfo {
}

pub trait ServerSoldierWeaponSpawnInfoTrait: TypeObject {
}

impl ServerSoldierWeaponSpawnInfoTrait for ServerSoldierWeaponSpawnInfo {
}

pub static SERVERSOLDIERWEAPONSPAWNINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierWeaponSpawnInfo",
    name_hash: 3149900509,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierWeaponSpawnInfo as Default>::default())),
            create_boxed: || Box::new(<ServerSoldierWeaponSpawnInfo as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static SERVERSOLDIERWEAPONSPAWNINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierWeaponSpawnInfo-Array",
    name_hash: 2180777705,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerSoldierWeaponSpawnInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3590225320,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_server::SERVERGAMECOMPONENTENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerSoldierWeapon, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierWeapon as Default>::default())),
            create_boxed: || Box::new(<ServerSoldierWeapon as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERSOLDIERWEAPON_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierWeapon-Array",
    name_hash: 1124335388,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerSoldierWeapon"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2944960157,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerAnimationTurretRotationComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAnimationTurretRotationComponent as Default>::default())),
            create_boxed: || Box::new(<ServerAnimationTurretRotationComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERANIMATIONTURRETROTATIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAnimationTurretRotationComponent-Array",
    name_hash: 36614697,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerAnimationTurretRotationComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3823724325,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::weapon::SERVERGHOSTPROJECTILEENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerGrenadeEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGrenadeEntity as Default>::default())),
            create_boxed: || Box::new(<ServerGrenadeEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERGRENADEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGrenadeEntity-Array",
    name_hash: 2322571153,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerGrenadeEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1503126967,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::weapon::SERVERGHOSTPROJECTILEENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerExplosionPackEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerExplosionPackEntity as Default>::default())),
            create_boxed: || Box::new(<ServerExplosionPackEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVEREXPLOSIONPACKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerExplosionPackEntity-Array",
    name_hash: 460209923,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerExplosionPackEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1597284887,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_server::SERVERGAMEHEALTHCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerMissileHealthComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerMissileHealthComponent as Default>::default())),
            create_boxed: || Box::new(<ServerMissileHealthComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERMISSILEHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerMissileHealthComponent-Array",
    name_hash: 4229597859,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerMissileHealthComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 560165544,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::PARTPHYSICSCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerExplosionPackPhysicsComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerExplosionPackPhysicsComponent as Default>::default())),
            create_boxed: || Box::new(<ServerExplosionPackPhysicsComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVEREXPLOSIONPACKPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerExplosionPackPhysicsComponent-Array",
    name_hash: 93107228,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerExplosionPackPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 133762719,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_server::SERVERGAMEHEALTHCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerExplosionPackHealthComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerExplosionPackHealthComponent as Default>::default())),
            create_boxed: || Box::new(<ServerExplosionPackHealthComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVEREXPLOSIONPACKHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerExplosionPackHealthComponent-Array",
    name_hash: 2031989547,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerExplosionPackHealthComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3866190673,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_server::SERVERCHARACTERENTRYCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerSoldierEntryComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierEntryComponent as Default>::default())),
            create_boxed: || Box::new(<ServerSoldierEntryComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERSOLDIERENTRYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierEntryComponent-Array",
    name_hash: 578229605,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerSoldierEntryComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3523768847,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerWeaponZeroingComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerWeaponZeroingComponent as Default>::default())),
            create_boxed: || Box::new(<ServerWeaponZeroingComponent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERWEAPONZEROINGCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponZeroingComponent-Array",
    name_hash: 3312905915,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerWeaponZeroingComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerSoldierInteractionEntityInRangeChangedMessage {
}

pub trait ServerSoldierInteractionEntityInRangeChangedMessageTrait: TypeObject {
}

impl ServerSoldierInteractionEntityInRangeChangedMessageTrait for ServerSoldierInteractionEntityInRangeChangedMessage {
}

pub static SERVERSOLDIERINTERACTIONENTITYINRANGECHANGEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierInteractionEntityInRangeChangedMessage",
    name_hash: 3972532314,
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierInteractionEntityInRangeChangedMessage as Default>::default())),
            create_boxed: || Box::new(<ServerSoldierInteractionEntityInRangeChangedMessage as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerSoldierSoldierVsSoldierCollisionMessage {
}

pub trait ServerSoldierSoldierVsSoldierCollisionMessageTrait: TypeObject {
}

impl ServerSoldierSoldierVsSoldierCollisionMessageTrait for ServerSoldierSoldierVsSoldierCollisionMessage {
}

pub static SERVERSOLDIERSOLDIERVSSOLDIERCOLLISIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierSoldierVsSoldierCollisionMessage",
    name_hash: 3267695354,
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierSoldierVsSoldierCollisionMessage as Default>::default())),
            create_boxed: || Box::new(<ServerSoldierSoldierVsSoldierCollisionMessage as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerSoldierPickedUpSupplySphereMessage {
}

pub trait ServerSoldierPickedUpSupplySphereMessageTrait: TypeObject {
}

impl ServerSoldierPickedUpSupplySphereMessageTrait for ServerSoldierPickedUpSupplySphereMessage {
}

pub static SERVERSOLDIERPICKEDUPSUPPLYSPHEREMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierPickedUpSupplySphereMessage",
    name_hash: 1186500254,
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierPickedUpSupplySphereMessage as Default>::default())),
            create_boxed: || Box::new(<ServerSoldierPickedUpSupplySphereMessage as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerSoldierOnUnspawnMessage {
}

pub trait ServerSoldierOnUnspawnMessageTrait: TypeObject {
}

impl ServerSoldierOnUnspawnMessageTrait for ServerSoldierOnUnspawnMessage {
}

pub static SERVERSOLDIERONUNSPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierOnUnspawnMessage",
    name_hash: 2048538016,
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierOnUnspawnMessage as Default>::default())),
            create_boxed: || Box::new(<ServerSoldierOnUnspawnMessage as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerSoldierThrowDistractionMessage {
}

pub trait ServerSoldierThrowDistractionMessageTrait: TypeObject {
}

impl ServerSoldierThrowDistractionMessageTrait for ServerSoldierThrowDistractionMessage {
}

pub static SERVERSOLDIERTHROWDISTRACTIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierThrowDistractionMessage",
    name_hash: 2375701201,
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierThrowDistractionMessage as Default>::default())),
            create_boxed: || Box::new(<ServerSoldierThrowDistractionMessage as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerSoldierSelfHealMessage {
}

pub trait ServerSoldierSelfHealMessageTrait: TypeObject {
}

impl ServerSoldierSelfHealMessageTrait for ServerSoldierSelfHealMessage {
}

pub static SERVERSOLDIERSELFHEALMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierSelfHealMessage",
    name_hash: 1952412637,
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierSelfHealMessage as Default>::default())),
            create_boxed: || Box::new(<ServerSoldierSelfHealMessage as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerSoldierManDownMessage {
}

pub trait ServerSoldierManDownMessageTrait: TypeObject {
}

impl ServerSoldierManDownMessageTrait for ServerSoldierManDownMessage {
}

pub static SERVERSOLDIERMANDOWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierManDownMessage",
    name_hash: 3208378225,
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierManDownMessage as Default>::default())),
            create_boxed: || Box::new(<ServerSoldierManDownMessage as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerSoldierOnInitMessage {
}

pub trait ServerSoldierOnInitMessageTrait: TypeObject {
}

impl ServerSoldierOnInitMessageTrait for ServerSoldierOnInitMessage {
}

pub static SERVERSOLDIERONINITMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierOnInitMessage",
    name_hash: 1619831450,
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierOnInitMessage as Default>::default())),
            create_boxed: || Box::new(<ServerSoldierOnInitMessage as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerSoldierChangingWeaponMessage {
}

pub trait ServerSoldierChangingWeaponMessageTrait: TypeObject {
}

impl ServerSoldierChangingWeaponMessageTrait for ServerSoldierChangingWeaponMessage {
}

pub static SERVERSOLDIERCHANGINGWEAPONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierChangingWeaponMessage",
    name_hash: 2213605728,
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierChangingWeaponMessage as Default>::default())),
            create_boxed: || Box::new(<ServerSoldierChangingWeaponMessage as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerSoldierFiringMessage {
}

pub trait ServerSoldierFiringMessageTrait: TypeObject {
}

impl ServerSoldierFiringMessageTrait for ServerSoldierFiringMessage {
}

pub static SERVERSOLDIERFIRINGMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierFiringMessage",
    name_hash: 3659808348,
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierFiringMessage as Default>::default())),
            create_boxed: || Box::new(<ServerSoldierFiringMessage as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerSoldierDamagedMessage {
}

pub trait ServerSoldierDamagedMessageTrait: TypeObject {
}

impl ServerSoldierDamagedMessageTrait for ServerSoldierDamagedMessage {
}

pub static SERVERSOLDIERDAMAGEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoldierDamagedMessage",
    name_hash: 200221454,
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoldierDamagedMessage as Default>::default())),
            create_boxed: || Box::new(<ServerSoldierDamagedMessage as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerProjectileMissileDestroyedMessage {
}

pub trait ServerProjectileMissileDestroyedMessageTrait: TypeObject {
}

impl ServerProjectileMissileDestroyedMessageTrait for ServerProjectileMissileDestroyedMessage {
}

pub static SERVERPROJECTILEMISSILEDESTROYEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerProjectileMissileDestroyedMessage",
    name_hash: 2666035225,
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerProjectileMissileDestroyedMessage as Default>::default())),
            create_boxed: || Box::new(<ServerProjectileMissileDestroyedMessage as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerProjectileMissileDamagedMessage {
}

pub trait ServerProjectileMissileDamagedMessageTrait: TypeObject {
}

impl ServerProjectileMissileDamagedMessageTrait for ServerProjectileMissileDamagedMessage {
}

pub static SERVERPROJECTILEMISSILEDAMAGEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerProjectileMissileDamagedMessage",
    name_hash: 3163578837,
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerProjectileMissileDamagedMessage as Default>::default())),
            create_boxed: || Box::new(<ServerProjectileMissileDamagedMessage as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientSoldierInteractionEntityInRangeChangedMessage {
}

pub trait ClientSoldierInteractionEntityInRangeChangedMessageTrait: TypeObject {
}

impl ClientSoldierInteractionEntityInRangeChangedMessageTrait for ClientSoldierInteractionEntityInRangeChangedMessage {
}

pub static CLIENTSOLDIERINTERACTIONENTITYINRANGECHANGEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierInteractionEntityInRangeChangedMessage",
    name_hash: 3369803270,
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSoldierInteractionEntityInRangeChangedMessage as Default>::default())),
            create_boxed: || Box::new(<ClientSoldierInteractionEntityInRangeChangedMessage as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientSoldierChangeCoverStateMessage {
}

pub trait ClientSoldierChangeCoverStateMessageTrait: TypeObject {
}

impl ClientSoldierChangeCoverStateMessageTrait for ClientSoldierChangeCoverStateMessage {
}

pub static CLIENTSOLDIERCHANGECOVERSTATEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSoldierChangeCoverStateMessage",
    name_hash: 2462780705,
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSoldierChangeCoverStateMessage as Default>::default())),
            create_boxed: || Box::new(<ClientSoldierChangeCoverStateMessage as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct AimingHandle {
}

pub trait AimingHandleTrait: TypeObject {
}

impl AimingHandleTrait for AimingHandle {
}

pub static AIMINGHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingHandle",
    name_hash: 3378344330,
    flags: MemberInfoFlags::new(73),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AimingHandle as Default>::default())),
            create_boxed: || Box::new(<AimingHandle as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static AIMINGHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingHandle-Array",
    name_hash: 3453406398,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("AimingHandle"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn aimer_position_mut(&mut self) -> &mut super::core::Vec3;
    fn last_hit_position(&self) -> &super::core::Vec3;
    fn last_hit_position_mut(&mut self) -> &mut super::core::Vec3;
    fn recoil_compensation(&self) -> &super::core::Vec2;
    fn recoil_compensation_mut(&mut self) -> &mut super::core::Vec2;
    fn last_apply_angles_from_sim_tick(&self) -> &u32;
    fn last_apply_angles_from_sim_tick_mut(&mut self) -> &mut u32;
    fn yaw(&self) -> &f32;
    fn yaw_mut(&mut self) -> &mut f32;
    fn pitch(&self) -> &f32;
    fn pitch_mut(&mut self) -> &mut f32;
    fn input_magnitude(&self) -> &f32;
    fn input_magnitude_mut(&mut self) -> &mut f32;
    fn snap_zoom_break_away(&self) -> &bool;
    fn snap_zoom_break_away_mut(&mut self) -> &mut bool;
    fn use_aim_assist(&self) -> &bool;
    fn use_aim_assist_mut(&mut self) -> &mut bool;
    fn allow_blend_out(&self) -> &bool;
    fn allow_blend_out_mut(&mut self) -> &mut bool;
}

impl AimingRenderReturnedValuesTrait for AimingRenderReturnedValues {
    fn aimer_position(&self) -> &super::core::Vec3 {
        &self.aimer_position
    }
    fn aimer_position_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.aimer_position
    }
    fn last_hit_position(&self) -> &super::core::Vec3 {
        &self.last_hit_position
    }
    fn last_hit_position_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.last_hit_position
    }
    fn recoil_compensation(&self) -> &super::core::Vec2 {
        &self.recoil_compensation
    }
    fn recoil_compensation_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.recoil_compensation
    }
    fn last_apply_angles_from_sim_tick(&self) -> &u32 {
        &self.last_apply_angles_from_sim_tick
    }
    fn last_apply_angles_from_sim_tick_mut(&mut self) -> &mut u32 {
        &mut self.last_apply_angles_from_sim_tick
    }
    fn yaw(&self) -> &f32 {
        &self.yaw
    }
    fn yaw_mut(&mut self) -> &mut f32 {
        &mut self.yaw
    }
    fn pitch(&self) -> &f32 {
        &self.pitch
    }
    fn pitch_mut(&mut self) -> &mut f32 {
        &mut self.pitch
    }
    fn input_magnitude(&self) -> &f32 {
        &self.input_magnitude
    }
    fn input_magnitude_mut(&mut self) -> &mut f32 {
        &mut self.input_magnitude
    }
    fn snap_zoom_break_away(&self) -> &bool {
        &self.snap_zoom_break_away
    }
    fn snap_zoom_break_away_mut(&mut self) -> &mut bool {
        &mut self.snap_zoom_break_away
    }
    fn use_aim_assist(&self) -> &bool {
        &self.use_aim_assist
    }
    fn use_aim_assist_mut(&mut self) -> &mut bool {
        &mut self.use_aim_assist
    }
    fn allow_blend_out(&self) -> &bool {
        &self.allow_blend_out
    }
    fn allow_blend_out_mut(&mut self) -> &mut bool {
        &mut self.allow_blend_out
    }
}

pub static AIMINGRENDERRETURNEDVALUES_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingRenderReturnedValues",
    name_hash: 1121551673,
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AimingRenderReturnedValues as Default>::default())),
            create_boxed: || Box::new(<AimingRenderReturnedValues as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "AimerPosition",
                name_hash: 2367265358,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AimingRenderReturnedValues, aimer_position),
            },
            FieldInfoData {
                name: "LastHitPosition",
                name_hash: 1175631075,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AimingRenderReturnedValues, last_hit_position),
            },
            FieldInfoData {
                name: "RecoilCompensation",
                name_hash: 4002576943,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(AimingRenderReturnedValues, recoil_compensation),
            },
            FieldInfoData {
                name: "LastApplyAnglesFromSimTick",
                name_hash: 488227261,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(AimingRenderReturnedValues, last_apply_angles_from_sim_tick),
            },
            FieldInfoData {
                name: "Yaw",
                name_hash: 193468618,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingRenderReturnedValues, yaw),
            },
            FieldInfoData {
                name: "Pitch",
                name_hash: 232604323,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingRenderReturnedValues, pitch),
            },
            FieldInfoData {
                name: "InputMagnitude",
                name_hash: 2562804191,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingRenderReturnedValues, input_magnitude),
            },
            FieldInfoData {
                name: "SnapZoomBreakAway",
                name_hash: 2140908591,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingRenderReturnedValues, snap_zoom_break_away),
            },
            FieldInfoData {
                name: "UseAimAssist",
                name_hash: 1326379692,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingRenderReturnedValues, use_aim_assist),
            },
            FieldInfoData {
                name: "AllowBlendOut",
                name_hash: 1958760403,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static AIMINGRENDERRETURNEDVALUES_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingRenderReturnedValues-Array",
    name_hash: 2706420621,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("AimingRenderReturnedValues"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn aimer_position_mut(&mut self) -> &mut super::core::Vec3;
    fn aimer_safe_position(&self) -> &super::core::Vec3;
    fn aimer_safe_position_mut(&mut self) -> &mut super::core::Vec3;
    fn last_aim_direction(&self) -> &super::core::Vec3;
    fn last_aim_direction_mut(&mut self) -> &mut super::core::Vec3;
    fn last_hit_position(&self) -> &super::core::Vec3;
    fn last_hit_position_mut(&mut self) -> &mut super::core::Vec3;
    fn aiming_input(&self) -> &super::core::Vec2;
    fn aiming_input_mut(&mut self) -> &mut super::core::Vec2;
    fn input_direction(&self) -> &super::core::Vec2;
    fn input_direction_mut(&mut self) -> &mut super::core::Vec2;
    fn input_acceleration_velocity(&self) -> &super::core::Vec2;
    fn input_acceleration_velocity_mut(&mut self) -> &mut super::core::Vec2;
    fn recoil_offset(&self) -> &super::core::Vec2;
    fn recoil_offset_mut(&mut self) -> &mut super::core::Vec2;
    fn last_tick(&self) -> &u32;
    fn last_tick_mut(&mut self) -> &mut u32;
    fn last_apply_angles_from_sim_tick(&self) -> &u32;
    fn last_apply_angles_from_sim_tick_mut(&mut self) -> &mut u32;
    fn yaw(&self) -> &f32;
    fn yaw_mut(&mut self) -> &mut f32;
    fn pitch(&self) -> &f32;
    fn pitch_mut(&mut self) -> &mut f32;
    fn input_magnitude(&self) -> &f32;
    fn input_magnitude_mut(&mut self) -> &mut f32;
    fn soft_zone_lambda_yaw_attract(&self) -> &f32;
    fn soft_zone_lambda_yaw_attract_mut(&mut self) -> &mut f32;
    fn soft_zone_lambda_pitch_attract(&self) -> &f32;
    fn soft_zone_lambda_pitch_attract_mut(&mut self) -> &mut f32;
    fn soft_zone_lambda_slowdown(&self) -> &f32;
    fn soft_zone_lambda_slowdown_mut(&mut self) -> &mut f32;
    fn target_distance(&self) -> &f32;
    fn target_distance_mut(&mut self) -> &mut f32;
    fn yaw_change_attract(&self) -> &f32;
    fn yaw_change_attract_mut(&mut self) -> &mut f32;
    fn pitch_change_attract(&self) -> &f32;
    fn pitch_change_attract_mut(&mut self) -> &mut f32;
    fn time_since_yaw_input(&self) -> &f32;
    fn time_since_yaw_input_mut(&mut self) -> &mut f32;
    fn time_since_pitch_input(&self) -> &f32;
    fn time_since_pitch_input_mut(&mut self) -> &mut f32;
    fn acceleration(&self) -> &f32;
    fn acceleration_mut(&mut self) -> &mut f32;
    fn acceleration_timer(&self) -> &f32;
    fn acceleration_timer_mut(&mut self) -> &mut f32;
    fn aimer_arm_length(&self) -> &f32;
    fn aimer_arm_length_mut(&mut self) -> &mut f32;
    fn time_to_delay_after_collision(&self) -> &f32;
    fn time_to_delay_after_collision_mut(&mut self) -> &mut f32;
    fn snap_zoom_break_away(&self) -> &bool;
    fn snap_zoom_break_away_mut(&mut self) -> &mut bool;
    fn is_mouse_aiming(&self) -> &bool;
    fn is_mouse_aiming_mut(&mut self) -> &mut bool;
    fn use_aim_assist(&self) -> &bool;
    fn use_aim_assist_mut(&mut self) -> &mut bool;
    fn use_input_polynomials(&self) -> &bool;
    fn use_input_polynomials_mut(&mut self) -> &mut bool;
    fn allow_blend_out(&self) -> &bool;
    fn allow_blend_out_mut(&mut self) -> &mut bool;
}

impl AimingRenderUpdateContextTrait for AimingRenderUpdateContext {
    fn aimer_position(&self) -> &super::core::Vec3 {
        &self.aimer_position
    }
    fn aimer_position_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.aimer_position
    }
    fn aimer_safe_position(&self) -> &super::core::Vec3 {
        &self.aimer_safe_position
    }
    fn aimer_safe_position_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.aimer_safe_position
    }
    fn last_aim_direction(&self) -> &super::core::Vec3 {
        &self.last_aim_direction
    }
    fn last_aim_direction_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.last_aim_direction
    }
    fn last_hit_position(&self) -> &super::core::Vec3 {
        &self.last_hit_position
    }
    fn last_hit_position_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.last_hit_position
    }
    fn aiming_input(&self) -> &super::core::Vec2 {
        &self.aiming_input
    }
    fn aiming_input_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.aiming_input
    }
    fn input_direction(&self) -> &super::core::Vec2 {
        &self.input_direction
    }
    fn input_direction_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.input_direction
    }
    fn input_acceleration_velocity(&self) -> &super::core::Vec2 {
        &self.input_acceleration_velocity
    }
    fn input_acceleration_velocity_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.input_acceleration_velocity
    }
    fn recoil_offset(&self) -> &super::core::Vec2 {
        &self.recoil_offset
    }
    fn recoil_offset_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.recoil_offset
    }
    fn last_tick(&self) -> &u32 {
        &self.last_tick
    }
    fn last_tick_mut(&mut self) -> &mut u32 {
        &mut self.last_tick
    }
    fn last_apply_angles_from_sim_tick(&self) -> &u32 {
        &self.last_apply_angles_from_sim_tick
    }
    fn last_apply_angles_from_sim_tick_mut(&mut self) -> &mut u32 {
        &mut self.last_apply_angles_from_sim_tick
    }
    fn yaw(&self) -> &f32 {
        &self.yaw
    }
    fn yaw_mut(&mut self) -> &mut f32 {
        &mut self.yaw
    }
    fn pitch(&self) -> &f32 {
        &self.pitch
    }
    fn pitch_mut(&mut self) -> &mut f32 {
        &mut self.pitch
    }
    fn input_magnitude(&self) -> &f32 {
        &self.input_magnitude
    }
    fn input_magnitude_mut(&mut self) -> &mut f32 {
        &mut self.input_magnitude
    }
    fn soft_zone_lambda_yaw_attract(&self) -> &f32 {
        &self.soft_zone_lambda_yaw_attract
    }
    fn soft_zone_lambda_yaw_attract_mut(&mut self) -> &mut f32 {
        &mut self.soft_zone_lambda_yaw_attract
    }
    fn soft_zone_lambda_pitch_attract(&self) -> &f32 {
        &self.soft_zone_lambda_pitch_attract
    }
    fn soft_zone_lambda_pitch_attract_mut(&mut self) -> &mut f32 {
        &mut self.soft_zone_lambda_pitch_attract
    }
    fn soft_zone_lambda_slowdown(&self) -> &f32 {
        &self.soft_zone_lambda_slowdown
    }
    fn soft_zone_lambda_slowdown_mut(&mut self) -> &mut f32 {
        &mut self.soft_zone_lambda_slowdown
    }
    fn target_distance(&self) -> &f32 {
        &self.target_distance
    }
    fn target_distance_mut(&mut self) -> &mut f32 {
        &mut self.target_distance
    }
    fn yaw_change_attract(&self) -> &f32 {
        &self.yaw_change_attract
    }
    fn yaw_change_attract_mut(&mut self) -> &mut f32 {
        &mut self.yaw_change_attract
    }
    fn pitch_change_attract(&self) -> &f32 {
        &self.pitch_change_attract
    }
    fn pitch_change_attract_mut(&mut self) -> &mut f32 {
        &mut self.pitch_change_attract
    }
    fn time_since_yaw_input(&self) -> &f32 {
        &self.time_since_yaw_input
    }
    fn time_since_yaw_input_mut(&mut self) -> &mut f32 {
        &mut self.time_since_yaw_input
    }
    fn time_since_pitch_input(&self) -> &f32 {
        &self.time_since_pitch_input
    }
    fn time_since_pitch_input_mut(&mut self) -> &mut f32 {
        &mut self.time_since_pitch_input
    }
    fn acceleration(&self) -> &f32 {
        &self.acceleration
    }
    fn acceleration_mut(&mut self) -> &mut f32 {
        &mut self.acceleration
    }
    fn acceleration_timer(&self) -> &f32 {
        &self.acceleration_timer
    }
    fn acceleration_timer_mut(&mut self) -> &mut f32 {
        &mut self.acceleration_timer
    }
    fn aimer_arm_length(&self) -> &f32 {
        &self.aimer_arm_length
    }
    fn aimer_arm_length_mut(&mut self) -> &mut f32 {
        &mut self.aimer_arm_length
    }
    fn time_to_delay_after_collision(&self) -> &f32 {
        &self.time_to_delay_after_collision
    }
    fn time_to_delay_after_collision_mut(&mut self) -> &mut f32 {
        &mut self.time_to_delay_after_collision
    }
    fn snap_zoom_break_away(&self) -> &bool {
        &self.snap_zoom_break_away
    }
    fn snap_zoom_break_away_mut(&mut self) -> &mut bool {
        &mut self.snap_zoom_break_away
    }
    fn is_mouse_aiming(&self) -> &bool {
        &self.is_mouse_aiming
    }
    fn is_mouse_aiming_mut(&mut self) -> &mut bool {
        &mut self.is_mouse_aiming
    }
    fn use_aim_assist(&self) -> &bool {
        &self.use_aim_assist
    }
    fn use_aim_assist_mut(&mut self) -> &mut bool {
        &mut self.use_aim_assist
    }
    fn use_input_polynomials(&self) -> &bool {
        &self.use_input_polynomials
    }
    fn use_input_polynomials_mut(&mut self) -> &mut bool {
        &mut self.use_input_polynomials
    }
    fn allow_blend_out(&self) -> &bool {
        &self.allow_blend_out
    }
    fn allow_blend_out_mut(&mut self) -> &mut bool {
        &mut self.allow_blend_out
    }
}

pub static AIMINGRENDERUPDATECONTEXT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingRenderUpdateContext",
    name_hash: 1205648228,
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AimingRenderUpdateContext as Default>::default())),
            create_boxed: || Box::new(<AimingRenderUpdateContext as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "AimerPosition",
                name_hash: 2367265358,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AimingRenderUpdateContext, aimer_position),
            },
            FieldInfoData {
                name: "AimerSafePosition",
                name_hash: 1220423135,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AimingRenderUpdateContext, aimer_safe_position),
            },
            FieldInfoData {
                name: "LastAimDirection",
                name_hash: 1289590575,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AimingRenderUpdateContext, last_aim_direction),
            },
            FieldInfoData {
                name: "LastHitPosition",
                name_hash: 1175631075,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AimingRenderUpdateContext, last_hit_position),
            },
            FieldInfoData {
                name: "AimingInput",
                name_hash: 1533038230,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(AimingRenderUpdateContext, aiming_input),
            },
            FieldInfoData {
                name: "InputDirection",
                name_hash: 544243606,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(AimingRenderUpdateContext, input_direction),
            },
            FieldInfoData {
                name: "InputAccelerationVelocity",
                name_hash: 1060647110,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(AimingRenderUpdateContext, input_acceleration_velocity),
            },
            FieldInfoData {
                name: "RecoilOffset",
                name_hash: 3958395670,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(AimingRenderUpdateContext, recoil_offset),
            },
            FieldInfoData {
                name: "LastTick",
                name_hash: 313640090,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(AimingRenderUpdateContext, last_tick),
            },
            FieldInfoData {
                name: "LastApplyAnglesFromSimTick",
                name_hash: 488227261,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(AimingRenderUpdateContext, last_apply_angles_from_sim_tick),
            },
            FieldInfoData {
                name: "Yaw",
                name_hash: 193468618,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingRenderUpdateContext, yaw),
            },
            FieldInfoData {
                name: "Pitch",
                name_hash: 232604323,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingRenderUpdateContext, pitch),
            },
            FieldInfoData {
                name: "InputMagnitude",
                name_hash: 2562804191,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingRenderUpdateContext, input_magnitude),
            },
            FieldInfoData {
                name: "SoftZoneLambdaYawAttract",
                name_hash: 181366424,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingRenderUpdateContext, soft_zone_lambda_yaw_attract),
            },
            FieldInfoData {
                name: "SoftZoneLambdaPitchAttract",
                name_hash: 3948880977,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingRenderUpdateContext, soft_zone_lambda_pitch_attract),
            },
            FieldInfoData {
                name: "SoftZoneLambdaSlowdown",
                name_hash: 3837739239,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingRenderUpdateContext, soft_zone_lambda_slowdown),
            },
            FieldInfoData {
                name: "TargetDistance",
                name_hash: 2573372439,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingRenderUpdateContext, target_distance),
            },
            FieldInfoData {
                name: "YawChangeAttract",
                name_hash: 2534205801,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingRenderUpdateContext, yaw_change_attract),
            },
            FieldInfoData {
                name: "PitchChangeAttract",
                name_hash: 1236233568,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingRenderUpdateContext, pitch_change_attract),
            },
            FieldInfoData {
                name: "TimeSinceYawInput",
                name_hash: 1974538875,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingRenderUpdateContext, time_since_yaw_input),
            },
            FieldInfoData {
                name: "TimeSincePitchInput",
                name_hash: 3138265266,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingRenderUpdateContext, time_since_pitch_input),
            },
            FieldInfoData {
                name: "Acceleration",
                name_hash: 62601415,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingRenderUpdateContext, acceleration),
            },
            FieldInfoData {
                name: "AccelerationTimer",
                name_hash: 117484448,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingRenderUpdateContext, acceleration_timer),
            },
            FieldInfoData {
                name: "AimerArmLength",
                name_hash: 3700328117,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingRenderUpdateContext, aimer_arm_length),
            },
            FieldInfoData {
                name: "TimeToDelayAfterCollision",
                name_hash: 3019047172,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingRenderUpdateContext, time_to_delay_after_collision),
            },
            FieldInfoData {
                name: "SnapZoomBreakAway",
                name_hash: 2140908591,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingRenderUpdateContext, snap_zoom_break_away),
            },
            FieldInfoData {
                name: "IsMouseAiming",
                name_hash: 2651550683,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingRenderUpdateContext, is_mouse_aiming),
            },
            FieldInfoData {
                name: "UseAimAssist",
                name_hash: 1326379692,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingRenderUpdateContext, use_aim_assist),
            },
            FieldInfoData {
                name: "UseInputPolynomials",
                name_hash: 2686592737,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingRenderUpdateContext, use_input_polynomials),
            },
            FieldInfoData {
                name: "AllowBlendOut",
                name_hash: 1958760403,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static AIMINGRENDERUPDATECONTEXT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingRenderUpdateContext-Array",
    name_hash: 1434836816,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("AimingRenderUpdateContext"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AimingSimUpdateContext {
    pub difficulty_data: Option<LockedTypeObject /* super::game_shared::DifficultyData */>,
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
    fn difficulty_data(&self) -> &Option<LockedTypeObject /* super::game_shared::DifficultyData */>;
    fn difficulty_data_mut(&mut self) -> &mut Option<LockedTypeObject /* super::game_shared::DifficultyData */>;
    fn aiming_range(&self) -> &f32;
    fn aiming_range_mut(&mut self) -> &mut f32;
    fn attract_pitch_strength(&self) -> &f32;
    fn attract_pitch_strength_mut(&mut self) -> &mut f32;
    fn attract_soft_zone(&self) -> &f32;
    fn attract_soft_zone_mut(&mut self) -> &mut f32;
    fn attract_yaw_strength(&self) -> &f32;
    fn attract_yaw_strength_mut(&mut self) -> &mut f32;
    fn snap_zoom_post_time_no_input(&self) -> &f32;
    fn snap_zoom_post_time_no_input_mut(&mut self) -> &mut f32;
    fn snap_zoom_post_time(&self) -> &f32;
    fn snap_zoom_post_time_mut(&mut self) -> &mut f32;
    fn snap_zoom_since_last_timer(&self) -> &f32;
    fn snap_zoom_since_last_timer_mut(&mut self) -> &mut f32;
    fn zoom_transition_timer(&self) -> &f32;
    fn zoom_transition_timer_mut(&mut self) -> &mut f32;
    fn local_player_id(&self) -> &super::core::LocalPlayerId;
    fn local_player_id_mut(&mut self) -> &mut super::core::LocalPlayerId;
    fn attract_zooming_post_player_aiming(&self) -> &bool;
    fn attract_zooming_post_player_aiming_mut(&mut self) -> &mut bool;
    fn force_aim_snap_deactivate(&self) -> &bool;
    fn force_aim_snap_deactivate_mut(&mut self) -> &mut bool;
    fn force_pick_best_target(&self) -> &bool;
    fn force_pick_best_target_mut(&mut self) -> &mut bool;
    fn has_been_sprinting(&self) -> &bool;
    fn has_been_sprinting_mut(&mut self) -> &mut bool;
    fn is_sprinting(&self) -> &bool;
    fn is_sprinting_mut(&mut self) -> &mut bool;
    fn use_aim_helpers_rotation(&self) -> &bool;
    fn use_aim_helpers_rotation_mut(&mut self) -> &mut bool;
    fn use_aim_helpers_slowdown(&self) -> &bool;
    fn use_aim_helpers_slowdown_mut(&mut self) -> &mut bool;
    fn snap_zoom_post_time_dynamic_point(&self) -> &bool;
    fn snap_zoom_post_time_dynamic_point_mut(&mut self) -> &mut bool;
    fn snap_zoom_target_changed(&self) -> &bool;
    fn snap_zoom_target_changed_mut(&mut self) -> &mut bool;
}

impl AimingSimUpdateContextTrait for AimingSimUpdateContext {
    fn difficulty_data(&self) -> &Option<LockedTypeObject /* super::game_shared::DifficultyData */> {
        &self.difficulty_data
    }
    fn difficulty_data_mut(&mut self) -> &mut Option<LockedTypeObject /* super::game_shared::DifficultyData */> {
        &mut self.difficulty_data
    }
    fn aiming_range(&self) -> &f32 {
        &self.aiming_range
    }
    fn aiming_range_mut(&mut self) -> &mut f32 {
        &mut self.aiming_range
    }
    fn attract_pitch_strength(&self) -> &f32 {
        &self.attract_pitch_strength
    }
    fn attract_pitch_strength_mut(&mut self) -> &mut f32 {
        &mut self.attract_pitch_strength
    }
    fn attract_soft_zone(&self) -> &f32 {
        &self.attract_soft_zone
    }
    fn attract_soft_zone_mut(&mut self) -> &mut f32 {
        &mut self.attract_soft_zone
    }
    fn attract_yaw_strength(&self) -> &f32 {
        &self.attract_yaw_strength
    }
    fn attract_yaw_strength_mut(&mut self) -> &mut f32 {
        &mut self.attract_yaw_strength
    }
    fn snap_zoom_post_time_no_input(&self) -> &f32 {
        &self.snap_zoom_post_time_no_input
    }
    fn snap_zoom_post_time_no_input_mut(&mut self) -> &mut f32 {
        &mut self.snap_zoom_post_time_no_input
    }
    fn snap_zoom_post_time(&self) -> &f32 {
        &self.snap_zoom_post_time
    }
    fn snap_zoom_post_time_mut(&mut self) -> &mut f32 {
        &mut self.snap_zoom_post_time
    }
    fn snap_zoom_since_last_timer(&self) -> &f32 {
        &self.snap_zoom_since_last_timer
    }
    fn snap_zoom_since_last_timer_mut(&mut self) -> &mut f32 {
        &mut self.snap_zoom_since_last_timer
    }
    fn zoom_transition_timer(&self) -> &f32 {
        &self.zoom_transition_timer
    }
    fn zoom_transition_timer_mut(&mut self) -> &mut f32 {
        &mut self.zoom_transition_timer
    }
    fn local_player_id(&self) -> &super::core::LocalPlayerId {
        &self.local_player_id
    }
    fn local_player_id_mut(&mut self) -> &mut super::core::LocalPlayerId {
        &mut self.local_player_id
    }
    fn attract_zooming_post_player_aiming(&self) -> &bool {
        &self.attract_zooming_post_player_aiming
    }
    fn attract_zooming_post_player_aiming_mut(&mut self) -> &mut bool {
        &mut self.attract_zooming_post_player_aiming
    }
    fn force_aim_snap_deactivate(&self) -> &bool {
        &self.force_aim_snap_deactivate
    }
    fn force_aim_snap_deactivate_mut(&mut self) -> &mut bool {
        &mut self.force_aim_snap_deactivate
    }
    fn force_pick_best_target(&self) -> &bool {
        &self.force_pick_best_target
    }
    fn force_pick_best_target_mut(&mut self) -> &mut bool {
        &mut self.force_pick_best_target
    }
    fn has_been_sprinting(&self) -> &bool {
        &self.has_been_sprinting
    }
    fn has_been_sprinting_mut(&mut self) -> &mut bool {
        &mut self.has_been_sprinting
    }
    fn is_sprinting(&self) -> &bool {
        &self.is_sprinting
    }
    fn is_sprinting_mut(&mut self) -> &mut bool {
        &mut self.is_sprinting
    }
    fn use_aim_helpers_rotation(&self) -> &bool {
        &self.use_aim_helpers_rotation
    }
    fn use_aim_helpers_rotation_mut(&mut self) -> &mut bool {
        &mut self.use_aim_helpers_rotation
    }
    fn use_aim_helpers_slowdown(&self) -> &bool {
        &self.use_aim_helpers_slowdown
    }
    fn use_aim_helpers_slowdown_mut(&mut self) -> &mut bool {
        &mut self.use_aim_helpers_slowdown
    }
    fn snap_zoom_post_time_dynamic_point(&self) -> &bool {
        &self.snap_zoom_post_time_dynamic_point
    }
    fn snap_zoom_post_time_dynamic_point_mut(&mut self) -> &mut bool {
        &mut self.snap_zoom_post_time_dynamic_point
    }
    fn snap_zoom_target_changed(&self) -> &bool {
        &self.snap_zoom_target_changed
    }
    fn snap_zoom_target_changed_mut(&mut self) -> &mut bool {
        &mut self.snap_zoom_target_changed
    }
}

pub static AIMINGSIMUPDATECONTEXT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingSimUpdateContext",
    name_hash: 4002154809,
    flags: MemberInfoFlags::new(73),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AimingSimUpdateContext as Default>::default())),
            create_boxed: || Box::new(<AimingSimUpdateContext as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "DifficultyData",
                name_hash: 2481634118,
                flags: MemberInfoFlags::new(0),
                field_type: "DifficultyData",
                rust_offset: offset_of!(AimingSimUpdateContext, difficulty_data),
            },
            FieldInfoData {
                name: "AimingRange",
                name_hash: 1541306623,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimUpdateContext, aiming_range),
            },
            FieldInfoData {
                name: "AttractPitchStrength",
                name_hash: 2360266947,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimUpdateContext, attract_pitch_strength),
            },
            FieldInfoData {
                name: "AttractSoftZone",
                name_hash: 2537844656,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimUpdateContext, attract_soft_zone),
            },
            FieldInfoData {
                name: "AttractYawStrength",
                name_hash: 4291196522,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimUpdateContext, attract_yaw_strength),
            },
            FieldInfoData {
                name: "SnapZoomPostTimeNoInput",
                name_hash: 2975942180,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimUpdateContext, snap_zoom_post_time_no_input),
            },
            FieldInfoData {
                name: "SnapZoomPostTime",
                name_hash: 1146345843,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimUpdateContext, snap_zoom_post_time),
            },
            FieldInfoData {
                name: "SnapZoomSinceLastTimer",
                name_hash: 2851940961,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimUpdateContext, snap_zoom_since_last_timer),
            },
            FieldInfoData {
                name: "ZoomTransitionTimer",
                name_hash: 3732311162,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimUpdateContext, zoom_transition_timer),
            },
            FieldInfoData {
                name: "LocalPlayerId",
                name_hash: 1029133718,
                flags: MemberInfoFlags::new(0),
                field_type: "LocalPlayerId",
                rust_offset: offset_of!(AimingSimUpdateContext, local_player_id),
            },
            FieldInfoData {
                name: "AttractZoomingPostPlayerAiming",
                name_hash: 1649904153,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimUpdateContext, attract_zooming_post_player_aiming),
            },
            FieldInfoData {
                name: "ForceAimSnapDeactivate",
                name_hash: 255611817,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimUpdateContext, force_aim_snap_deactivate),
            },
            FieldInfoData {
                name: "ForcePickBestTarget",
                name_hash: 3984114360,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimUpdateContext, force_pick_best_target),
            },
            FieldInfoData {
                name: "HasBeenSprinting",
                name_hash: 3812634705,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimUpdateContext, has_been_sprinting),
            },
            FieldInfoData {
                name: "IsSprinting",
                name_hash: 1831681117,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimUpdateContext, is_sprinting),
            },
            FieldInfoData {
                name: "UseAimHelpersRotation",
                name_hash: 129836194,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimUpdateContext, use_aim_helpers_rotation),
            },
            FieldInfoData {
                name: "UseAimHelpersSlowdown",
                name_hash: 4215620131,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimUpdateContext, use_aim_helpers_slowdown),
            },
            FieldInfoData {
                name: "SnapZoomPostTimeDynamicPoint",
                name_hash: 3871858506,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimUpdateContext, snap_zoom_post_time_dynamic_point),
            },
            FieldInfoData {
                name: "SnapZoomTargetChanged",
                name_hash: 4007549037,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static AIMINGSIMUPDATECONTEXT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingSimUpdateContext-Array",
    name_hash: 3520477069,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("AimingSimUpdateContext"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    pub solider_aiming_simulation_data: Option<LockedTypeObject /* super::soldier_shared::SoldierAimingSimulationData */>,
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
    fn aiming_environment_target_mut(&mut self) -> &mut AimingEnvironmentTarget;
    fn aiming_constraints(&self) -> &AimingConstraints;
    fn aiming_constraints_mut(&mut self) -> &mut AimingConstraints;
    fn collision_excluded_bodies(&self) -> &super::physics::PhysicsRenderWorldHandle;
    fn collision_excluded_bodies_mut(&mut self) -> &mut super::physics::PhysicsRenderWorldHandle;
    fn character_entity_space_component_transform(&self) -> &super::core::LinearTransform;
    fn character_entity_space_component_transform_mut(&mut self) -> &mut super::core::LinearTransform;
    fn lock_aim_transform(&self) -> &super::core::LinearTransform;
    fn lock_aim_transform_mut(&mut self) -> &mut super::core::LinearTransform;
    fn aimer_root_position(&self) -> &super::core::Vec3;
    fn aimer_root_position_mut(&mut self) -> &mut super::core::Vec3;
    fn desired_aimer_local_position(&self) -> &super::core::Vec3;
    fn desired_aimer_local_position_mut(&mut self) -> &mut super::core::Vec3;
    fn desired_aimer_safe_position(&self) -> &super::core::Vec3;
    fn desired_aimer_safe_position_mut(&mut self) -> &mut super::core::Vec3;
    fn static_aimer_safe_position(&self) -> &super::core::Vec3;
    fn static_aimer_safe_position_mut(&mut self) -> &mut super::core::Vec3;
    fn force_aim_snap_target_position(&self) -> &super::core::Vec3;
    fn force_aim_snap_target_position_mut(&mut self) -> &mut super::core::Vec3;
    fn reticle_speed(&self) -> &super::core::Vec3;
    fn reticle_speed_mut(&mut self) -> &mut super::core::Vec3;
    fn surface_angular_velocity(&self) -> &super::core::Vec3;
    fn surface_angular_velocity_mut(&mut self) -> &mut super::core::Vec3;
    fn sim_aiming_input(&self) -> &super::core::Vec2;
    fn sim_aiming_input_mut(&mut self) -> &mut super::core::Vec2;
    fn aim_scale(&self) -> &super::core::Vec2;
    fn aim_scale_mut(&mut self) -> &mut super::core::Vec2;
    fn attract_distance_fall_off(&self) -> &super::core::Vec2;
    fn attract_distance_fall_off_mut(&mut self) -> &mut super::core::Vec2;
    fn max_angular_velocity(&self) -> &super::core::Vec2;
    fn max_angular_velocity_mut(&mut self) -> &mut super::core::Vec2;
    fn movement_input(&self) -> &super::core::Vec2;
    fn movement_input_mut(&mut self) -> &mut super::core::Vec2;
    fn recoil(&self) -> &super::core::Vec2;
    fn recoil_mut(&mut self) -> &mut super::core::Vec2;
    fn aim_sway_offset(&self) -> &super::core::Vec2;
    fn aim_sway_offset_mut(&mut self) -> &mut super::core::Vec2;
    fn solider_aiming_simulation_data(&self) -> &Option<LockedTypeObject /* super::soldier_shared::SoldierAimingSimulationData */>;
    fn solider_aiming_simulation_data_mut(&mut self) -> &mut Option<LockedTypeObject /* super::soldier_shared::SoldierAimingSimulationData */>;
    fn zoom_level(&self) -> &u32;
    fn zoom_level_mut(&mut self) -> &mut u32;
    fn tick(&self) -> &u32;
    fn tick_mut(&mut self) -> &mut u32;
    fn apply_angles_from_sim_tick(&self) -> &u32;
    fn apply_angles_from_sim_tick_mut(&mut self) -> &mut u32;
    fn ignore_constraints_tick(&self) -> &u32;
    fn ignore_constraints_tick_mut(&mut self) -> &mut u32;
    fn attract_user_input_multiplier(&self) -> &f32;
    fn attract_user_input_multiplier_mut(&mut self) -> &mut f32;
    fn attract_zooming_post_timer(&self) -> &f32;
    fn attract_zooming_post_timer_mut(&mut self) -> &mut f32;
    fn attract_zooming_post_time(&self) -> &f32;
    fn attract_zooming_post_time_mut(&mut self) -> &mut f32;
    fn last_hit_position_distance(&self) -> &f32;
    fn last_hit_position_distance_mut(&mut self) -> &mut f32;
    fn look_speed_multiplier(&self) -> &f32;
    fn look_speed_multiplier_mut(&mut self) -> &mut f32;
    fn minimum_pitch(&self) -> &f32;
    fn minimum_pitch_mut(&mut self) -> &mut f32;
    fn maximum_pitch(&self) -> &f32;
    fn maximum_pitch_mut(&mut self) -> &mut f32;
    fn reticle_field_of_view(&self) -> &f32;
    fn reticle_field_of_view_mut(&mut self) -> &mut f32;
    fn sim_pitch_to_apply(&self) -> &f32;
    fn sim_pitch_to_apply_mut(&mut self) -> &mut f32;
    fn sim_yaw_to_apply(&self) -> &f32;
    fn sim_yaw_to_apply_mut(&mut self) -> &mut f32;
    fn snap_zoom_break_away_timer(&self) -> &f32;
    fn snap_zoom_break_away_timer_mut(&mut self) -> &mut f32;
    fn snap_zoom_timer(&self) -> &f32;
    fn snap_zoom_timer_mut(&mut self) -> &mut f32;
    fn snap_zoom_time(&self) -> &f32;
    fn snap_zoom_time_mut(&mut self) -> &mut f32;
    fn world_space_lock_efficiency_pitch(&self) -> &f32;
    fn world_space_lock_efficiency_pitch_mut(&mut self) -> &mut f32;
    fn world_space_lock_efficiency_yaw(&self) -> &f32;
    fn world_space_lock_efficiency_yaw_mut(&mut self) -> &mut f32;
    fn aimer_collision_blend_out(&self) -> &f32;
    fn aimer_collision_blend_out_mut(&mut self) -> &mut f32;
    fn time_to_delay_after_collision(&self) -> &f32;
    fn time_to_delay_after_collision_mut(&mut self) -> &mut f32;
    fn override_mode(&self) -> &AimOverrideMode;
    fn override_mode_mut(&mut self) -> &mut AimOverrideMode;
    fn aim_at_last_hit_position(&self) -> &bool;
    fn aim_at_last_hit_position_mut(&mut self) -> &mut bool;
    fn force_aim_snap(&self) -> &bool;
    fn force_aim_snap_mut(&mut self) -> &mut bool;
    fn has_aiming_constraints(&self) -> &bool;
    fn has_aiming_constraints_mut(&mut self) -> &mut bool;
    fn has_angular_velocity_constraints(&self) -> &bool;
    fn has_angular_velocity_constraints_mut(&mut self) -> &mut bool;
    fn has_character_entity_space_component(&self) -> &bool;
    fn has_character_entity_space_component_mut(&mut self) -> &mut bool;
    fn is_alive(&self) -> &bool;
    fn is_alive_mut(&mut self) -> &mut bool;
    fn is_dead(&self) -> &bool;
    fn is_dead_mut(&mut self) -> &mut bool;
    fn is_fps_aiming_disabled(&self) -> &bool;
    fn is_fps_aiming_disabled_mut(&mut self) -> &mut bool;
    fn is_snap_zoomed(&self) -> &bool;
    fn is_snap_zoomed_mut(&mut self) -> &mut bool;
    fn snap_zoom_allowed(&self) -> &bool;
    fn snap_zoom_allowed_mut(&mut self) -> &mut bool;
    fn zoom_has_changed(&self) -> &bool;
    fn zoom_has_changed_mut(&mut self) -> &mut bool;
    fn zoom_in_aiming_help_active(&self) -> &bool;
    fn zoom_in_aiming_help_active_mut(&mut self) -> &mut bool;
    fn aim_assist_option_enabled(&self) -> &bool;
    fn aim_assist_option_enabled_mut(&mut self) -> &mut bool;
    fn field_flag_changed0(&self) -> &u32;
    fn field_flag_changed0_mut(&mut self) -> &mut u32;
    fn field_flag_changed1(&self) -> &u32;
    fn field_flag_changed1_mut(&mut self) -> &mut u32;
}

impl AimingSimDynamicStateTrait for AimingSimDynamicState {
    fn aiming_environment_target(&self) -> &AimingEnvironmentTarget {
        &self.aiming_environment_target
    }
    fn aiming_environment_target_mut(&mut self) -> &mut AimingEnvironmentTarget {
        &mut self.aiming_environment_target
    }
    fn aiming_constraints(&self) -> &AimingConstraints {
        &self.aiming_constraints
    }
    fn aiming_constraints_mut(&mut self) -> &mut AimingConstraints {
        &mut self.aiming_constraints
    }
    fn collision_excluded_bodies(&self) -> &super::physics::PhysicsRenderWorldHandle {
        &self.collision_excluded_bodies
    }
    fn collision_excluded_bodies_mut(&mut self) -> &mut super::physics::PhysicsRenderWorldHandle {
        &mut self.collision_excluded_bodies
    }
    fn character_entity_space_component_transform(&self) -> &super::core::LinearTransform {
        &self.character_entity_space_component_transform
    }
    fn character_entity_space_component_transform_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.character_entity_space_component_transform
    }
    fn lock_aim_transform(&self) -> &super::core::LinearTransform {
        &self.lock_aim_transform
    }
    fn lock_aim_transform_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.lock_aim_transform
    }
    fn aimer_root_position(&self) -> &super::core::Vec3 {
        &self.aimer_root_position
    }
    fn aimer_root_position_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.aimer_root_position
    }
    fn desired_aimer_local_position(&self) -> &super::core::Vec3 {
        &self.desired_aimer_local_position
    }
    fn desired_aimer_local_position_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.desired_aimer_local_position
    }
    fn desired_aimer_safe_position(&self) -> &super::core::Vec3 {
        &self.desired_aimer_safe_position
    }
    fn desired_aimer_safe_position_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.desired_aimer_safe_position
    }
    fn static_aimer_safe_position(&self) -> &super::core::Vec3 {
        &self.static_aimer_safe_position
    }
    fn static_aimer_safe_position_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.static_aimer_safe_position
    }
    fn force_aim_snap_target_position(&self) -> &super::core::Vec3 {
        &self.force_aim_snap_target_position
    }
    fn force_aim_snap_target_position_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.force_aim_snap_target_position
    }
    fn reticle_speed(&self) -> &super::core::Vec3 {
        &self.reticle_speed
    }
    fn reticle_speed_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.reticle_speed
    }
    fn surface_angular_velocity(&self) -> &super::core::Vec3 {
        &self.surface_angular_velocity
    }
    fn surface_angular_velocity_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.surface_angular_velocity
    }
    fn sim_aiming_input(&self) -> &super::core::Vec2 {
        &self.sim_aiming_input
    }
    fn sim_aiming_input_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.sim_aiming_input
    }
    fn aim_scale(&self) -> &super::core::Vec2 {
        &self.aim_scale
    }
    fn aim_scale_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.aim_scale
    }
    fn attract_distance_fall_off(&self) -> &super::core::Vec2 {
        &self.attract_distance_fall_off
    }
    fn attract_distance_fall_off_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.attract_distance_fall_off
    }
    fn max_angular_velocity(&self) -> &super::core::Vec2 {
        &self.max_angular_velocity
    }
    fn max_angular_velocity_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.max_angular_velocity
    }
    fn movement_input(&self) -> &super::core::Vec2 {
        &self.movement_input
    }
    fn movement_input_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.movement_input
    }
    fn recoil(&self) -> &super::core::Vec2 {
        &self.recoil
    }
    fn recoil_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.recoil
    }
    fn aim_sway_offset(&self) -> &super::core::Vec2 {
        &self.aim_sway_offset
    }
    fn aim_sway_offset_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.aim_sway_offset
    }
    fn solider_aiming_simulation_data(&self) -> &Option<LockedTypeObject /* super::soldier_shared::SoldierAimingSimulationData */> {
        &self.solider_aiming_simulation_data
    }
    fn solider_aiming_simulation_data_mut(&mut self) -> &mut Option<LockedTypeObject /* super::soldier_shared::SoldierAimingSimulationData */> {
        &mut self.solider_aiming_simulation_data
    }
    fn zoom_level(&self) -> &u32 {
        &self.zoom_level
    }
    fn zoom_level_mut(&mut self) -> &mut u32 {
        &mut self.zoom_level
    }
    fn tick(&self) -> &u32 {
        &self.tick
    }
    fn tick_mut(&mut self) -> &mut u32 {
        &mut self.tick
    }
    fn apply_angles_from_sim_tick(&self) -> &u32 {
        &self.apply_angles_from_sim_tick
    }
    fn apply_angles_from_sim_tick_mut(&mut self) -> &mut u32 {
        &mut self.apply_angles_from_sim_tick
    }
    fn ignore_constraints_tick(&self) -> &u32 {
        &self.ignore_constraints_tick
    }
    fn ignore_constraints_tick_mut(&mut self) -> &mut u32 {
        &mut self.ignore_constraints_tick
    }
    fn attract_user_input_multiplier(&self) -> &f32 {
        &self.attract_user_input_multiplier
    }
    fn attract_user_input_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.attract_user_input_multiplier
    }
    fn attract_zooming_post_timer(&self) -> &f32 {
        &self.attract_zooming_post_timer
    }
    fn attract_zooming_post_timer_mut(&mut self) -> &mut f32 {
        &mut self.attract_zooming_post_timer
    }
    fn attract_zooming_post_time(&self) -> &f32 {
        &self.attract_zooming_post_time
    }
    fn attract_zooming_post_time_mut(&mut self) -> &mut f32 {
        &mut self.attract_zooming_post_time
    }
    fn last_hit_position_distance(&self) -> &f32 {
        &self.last_hit_position_distance
    }
    fn last_hit_position_distance_mut(&mut self) -> &mut f32 {
        &mut self.last_hit_position_distance
    }
    fn look_speed_multiplier(&self) -> &f32 {
        &self.look_speed_multiplier
    }
    fn look_speed_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.look_speed_multiplier
    }
    fn minimum_pitch(&self) -> &f32 {
        &self.minimum_pitch
    }
    fn minimum_pitch_mut(&mut self) -> &mut f32 {
        &mut self.minimum_pitch
    }
    fn maximum_pitch(&self) -> &f32 {
        &self.maximum_pitch
    }
    fn maximum_pitch_mut(&mut self) -> &mut f32 {
        &mut self.maximum_pitch
    }
    fn reticle_field_of_view(&self) -> &f32 {
        &self.reticle_field_of_view
    }
    fn reticle_field_of_view_mut(&mut self) -> &mut f32 {
        &mut self.reticle_field_of_view
    }
    fn sim_pitch_to_apply(&self) -> &f32 {
        &self.sim_pitch_to_apply
    }
    fn sim_pitch_to_apply_mut(&mut self) -> &mut f32 {
        &mut self.sim_pitch_to_apply
    }
    fn sim_yaw_to_apply(&self) -> &f32 {
        &self.sim_yaw_to_apply
    }
    fn sim_yaw_to_apply_mut(&mut self) -> &mut f32 {
        &mut self.sim_yaw_to_apply
    }
    fn snap_zoom_break_away_timer(&self) -> &f32 {
        &self.snap_zoom_break_away_timer
    }
    fn snap_zoom_break_away_timer_mut(&mut self) -> &mut f32 {
        &mut self.snap_zoom_break_away_timer
    }
    fn snap_zoom_timer(&self) -> &f32 {
        &self.snap_zoom_timer
    }
    fn snap_zoom_timer_mut(&mut self) -> &mut f32 {
        &mut self.snap_zoom_timer
    }
    fn snap_zoom_time(&self) -> &f32 {
        &self.snap_zoom_time
    }
    fn snap_zoom_time_mut(&mut self) -> &mut f32 {
        &mut self.snap_zoom_time
    }
    fn world_space_lock_efficiency_pitch(&self) -> &f32 {
        &self.world_space_lock_efficiency_pitch
    }
    fn world_space_lock_efficiency_pitch_mut(&mut self) -> &mut f32 {
        &mut self.world_space_lock_efficiency_pitch
    }
    fn world_space_lock_efficiency_yaw(&self) -> &f32 {
        &self.world_space_lock_efficiency_yaw
    }
    fn world_space_lock_efficiency_yaw_mut(&mut self) -> &mut f32 {
        &mut self.world_space_lock_efficiency_yaw
    }
    fn aimer_collision_blend_out(&self) -> &f32 {
        &self.aimer_collision_blend_out
    }
    fn aimer_collision_blend_out_mut(&mut self) -> &mut f32 {
        &mut self.aimer_collision_blend_out
    }
    fn time_to_delay_after_collision(&self) -> &f32 {
        &self.time_to_delay_after_collision
    }
    fn time_to_delay_after_collision_mut(&mut self) -> &mut f32 {
        &mut self.time_to_delay_after_collision
    }
    fn override_mode(&self) -> &AimOverrideMode {
        &self.override_mode
    }
    fn override_mode_mut(&mut self) -> &mut AimOverrideMode {
        &mut self.override_mode
    }
    fn aim_at_last_hit_position(&self) -> &bool {
        &self.aim_at_last_hit_position
    }
    fn aim_at_last_hit_position_mut(&mut self) -> &mut bool {
        &mut self.aim_at_last_hit_position
    }
    fn force_aim_snap(&self) -> &bool {
        &self.force_aim_snap
    }
    fn force_aim_snap_mut(&mut self) -> &mut bool {
        &mut self.force_aim_snap
    }
    fn has_aiming_constraints(&self) -> &bool {
        &self.has_aiming_constraints
    }
    fn has_aiming_constraints_mut(&mut self) -> &mut bool {
        &mut self.has_aiming_constraints
    }
    fn has_angular_velocity_constraints(&self) -> &bool {
        &self.has_angular_velocity_constraints
    }
    fn has_angular_velocity_constraints_mut(&mut self) -> &mut bool {
        &mut self.has_angular_velocity_constraints
    }
    fn has_character_entity_space_component(&self) -> &bool {
        &self.has_character_entity_space_component
    }
    fn has_character_entity_space_component_mut(&mut self) -> &mut bool {
        &mut self.has_character_entity_space_component
    }
    fn is_alive(&self) -> &bool {
        &self.is_alive
    }
    fn is_alive_mut(&mut self) -> &mut bool {
        &mut self.is_alive
    }
    fn is_dead(&self) -> &bool {
        &self.is_dead
    }
    fn is_dead_mut(&mut self) -> &mut bool {
        &mut self.is_dead
    }
    fn is_fps_aiming_disabled(&self) -> &bool {
        &self.is_fps_aiming_disabled
    }
    fn is_fps_aiming_disabled_mut(&mut self) -> &mut bool {
        &mut self.is_fps_aiming_disabled
    }
    fn is_snap_zoomed(&self) -> &bool {
        &self.is_snap_zoomed
    }
    fn is_snap_zoomed_mut(&mut self) -> &mut bool {
        &mut self.is_snap_zoomed
    }
    fn snap_zoom_allowed(&self) -> &bool {
        &self.snap_zoom_allowed
    }
    fn snap_zoom_allowed_mut(&mut self) -> &mut bool {
        &mut self.snap_zoom_allowed
    }
    fn zoom_has_changed(&self) -> &bool {
        &self.zoom_has_changed
    }
    fn zoom_has_changed_mut(&mut self) -> &mut bool {
        &mut self.zoom_has_changed
    }
    fn zoom_in_aiming_help_active(&self) -> &bool {
        &self.zoom_in_aiming_help_active
    }
    fn zoom_in_aiming_help_active_mut(&mut self) -> &mut bool {
        &mut self.zoom_in_aiming_help_active
    }
    fn aim_assist_option_enabled(&self) -> &bool {
        &self.aim_assist_option_enabled
    }
    fn aim_assist_option_enabled_mut(&mut self) -> &mut bool {
        &mut self.aim_assist_option_enabled
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u32 {
        &mut self.field_flag_changed0
    }
    fn field_flag_changed1(&self) -> &u32 {
        &self.field_flag_changed1
    }
    fn field_flag_changed1_mut(&mut self) -> &mut u32 {
        &mut self.field_flag_changed1
    }
}

pub static AIMINGSIMDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingSimDynamicState",
    name_hash: 3865772629,
    flags: MemberInfoFlags::new(73),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AimingSimDynamicState as Default>::default())),
            create_boxed: || Box::new(<AimingSimDynamicState as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "AimingEnvironmentTarget",
                name_hash: 1862020548,
                flags: MemberInfoFlags::new(0),
                field_type: "AimingEnvironmentTarget",
                rust_offset: offset_of!(AimingSimDynamicState, aiming_environment_target),
            },
            FieldInfoData {
                name: "AimingConstraints",
                name_hash: 1523178198,
                flags: MemberInfoFlags::new(0),
                field_type: "AimingConstraints",
                rust_offset: offset_of!(AimingSimDynamicState, aiming_constraints),
            },
            FieldInfoData {
                name: "CollisionExcludedBodies",
                name_hash: 1252137199,
                flags: MemberInfoFlags::new(0),
                field_type: "PhysicsRenderWorldHandle",
                rust_offset: offset_of!(AimingSimDynamicState, collision_excluded_bodies),
            },
            FieldInfoData {
                name: "CharacterEntitySpaceComponentTransform",
                name_hash: 3167660064,
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(AimingSimDynamicState, character_entity_space_component_transform),
            },
            FieldInfoData {
                name: "LockAimTransform",
                name_hash: 430473799,
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(AimingSimDynamicState, lock_aim_transform),
            },
            FieldInfoData {
                name: "AimerRootPosition",
                name_hash: 1826913864,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AimingSimDynamicState, aimer_root_position),
            },
            FieldInfoData {
                name: "DesiredAimerLocalPosition",
                name_hash: 1937662571,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AimingSimDynamicState, desired_aimer_local_position),
            },
            FieldInfoData {
                name: "DesiredAimerSafePosition",
                name_hash: 2830005463,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AimingSimDynamicState, desired_aimer_safe_position),
            },
            FieldInfoData {
                name: "StaticAimerSafePosition",
                name_hash: 1587614983,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AimingSimDynamicState, static_aimer_safe_position),
            },
            FieldInfoData {
                name: "ForceAimSnapTargetPosition",
                name_hash: 2374030425,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AimingSimDynamicState, force_aim_snap_target_position),
            },
            FieldInfoData {
                name: "ReticleSpeed",
                name_hash: 3764712802,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AimingSimDynamicState, reticle_speed),
            },
            FieldInfoData {
                name: "SurfaceAngularVelocity",
                name_hash: 4227357029,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AimingSimDynamicState, surface_angular_velocity),
            },
            FieldInfoData {
                name: "SimAimingInput",
                name_hash: 3045777569,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(AimingSimDynamicState, sim_aiming_input),
            },
            FieldInfoData {
                name: "AimScale",
                name_hash: 2680888952,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(AimingSimDynamicState, aim_scale),
            },
            FieldInfoData {
                name: "AttractDistanceFallOff",
                name_hash: 422416875,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(AimingSimDynamicState, attract_distance_fall_off),
            },
            FieldInfoData {
                name: "MaxAngularVelocity",
                name_hash: 2007589540,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(AimingSimDynamicState, max_angular_velocity),
            },
            FieldInfoData {
                name: "MovementInput",
                name_hash: 3235799632,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(AimingSimDynamicState, movement_input),
            },
            FieldInfoData {
                name: "Recoil",
                name_hash: 3293845435,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(AimingSimDynamicState, recoil),
            },
            FieldInfoData {
                name: "AimSwayOffset",
                name_hash: 2866170641,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(AimingSimDynamicState, aim_sway_offset),
            },
            FieldInfoData {
                name: "SoliderAimingSimulationData",
                name_hash: 3677032201,
                flags: MemberInfoFlags::new(0),
                field_type: "SoldierAimingSimulationData",
                rust_offset: offset_of!(AimingSimDynamicState, solider_aiming_simulation_data),
            },
            FieldInfoData {
                name: "ZoomLevel",
                name_hash: 3650803780,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(AimingSimDynamicState, zoom_level),
            },
            FieldInfoData {
                name: "Tick",
                name_hash: 2089313808,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(AimingSimDynamicState, tick),
            },
            FieldInfoData {
                name: "ApplyAnglesFromSimTick",
                name_hash: 1110865079,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(AimingSimDynamicState, apply_angles_from_sim_tick),
            },
            FieldInfoData {
                name: "IgnoreConstraintsTick",
                name_hash: 3474277598,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(AimingSimDynamicState, ignore_constraints_tick),
            },
            FieldInfoData {
                name: "AttractUserInputMultiplier",
                name_hash: 583637228,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimDynamicState, attract_user_input_multiplier),
            },
            FieldInfoData {
                name: "AttractZoomingPostTimer",
                name_hash: 852520552,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimDynamicState, attract_zooming_post_timer),
            },
            FieldInfoData {
                name: "AttractZoomingPostTime",
                name_hash: 286135002,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimDynamicState, attract_zooming_post_time),
            },
            FieldInfoData {
                name: "LastHitPositionDistance",
                name_hash: 3020663520,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimDynamicState, last_hit_position_distance),
            },
            FieldInfoData {
                name: "LookSpeedMultiplier",
                name_hash: 1418472942,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimDynamicState, look_speed_multiplier),
            },
            FieldInfoData {
                name: "MinimumPitch",
                name_hash: 4284033781,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimDynamicState, minimum_pitch),
            },
            FieldInfoData {
                name: "MaximumPitch",
                name_hash: 1925433387,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimDynamicState, maximum_pitch),
            },
            FieldInfoData {
                name: "ReticleFieldOfView",
                name_hash: 3131697059,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimDynamicState, reticle_field_of_view),
            },
            FieldInfoData {
                name: "SimPitchToApply",
                name_hash: 3267492315,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimDynamicState, sim_pitch_to_apply),
            },
            FieldInfoData {
                name: "SimYawToApply",
                name_hash: 3109637138,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimDynamicState, sim_yaw_to_apply),
            },
            FieldInfoData {
                name: "SnapZoomBreakAwayTimer",
                name_hash: 584535816,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimDynamicState, snap_zoom_break_away_timer),
            },
            FieldInfoData {
                name: "SnapZoomTimer",
                name_hash: 930071417,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimDynamicState, snap_zoom_timer),
            },
            FieldInfoData {
                name: "SnapZoomTime",
                name_hash: 28183979,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimDynamicState, snap_zoom_time),
            },
            FieldInfoData {
                name: "WorldSpaceLockEfficiencyPitch",
                name_hash: 3673803289,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimDynamicState, world_space_lock_efficiency_pitch),
            },
            FieldInfoData {
                name: "WorldSpaceLockEfficiencyYaw",
                name_hash: 1888576688,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimDynamicState, world_space_lock_efficiency_yaw),
            },
            FieldInfoData {
                name: "AimerCollisionBlendOut",
                name_hash: 2465749926,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimDynamicState, aimer_collision_blend_out),
            },
            FieldInfoData {
                name: "TimeToDelayAfterCollision",
                name_hash: 3019047172,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingSimDynamicState, time_to_delay_after_collision),
            },
            FieldInfoData {
                name: "OverrideMode",
                name_hash: 4160559794,
                flags: MemberInfoFlags::new(0),
                field_type: "AimOverrideMode",
                rust_offset: offset_of!(AimingSimDynamicState, override_mode),
            },
            FieldInfoData {
                name: "AimAtLastHitPosition",
                name_hash: 3621993779,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimDynamicState, aim_at_last_hit_position),
            },
            FieldInfoData {
                name: "ForceAimSnap",
                name_hash: 3920416113,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimDynamicState, force_aim_snap),
            },
            FieldInfoData {
                name: "HasAimingConstraints",
                name_hash: 3583856268,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimDynamicState, has_aiming_constraints),
            },
            FieldInfoData {
                name: "HasAngularVelocityConstraints",
                name_hash: 3902772188,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimDynamicState, has_angular_velocity_constraints),
            },
            FieldInfoData {
                name: "HasCharacterEntitySpaceComponent",
                name_hash: 2734668598,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimDynamicState, has_character_entity_space_component),
            },
            FieldInfoData {
                name: "IsAlive",
                name_hash: 2763355624,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimDynamicState, is_alive),
            },
            FieldInfoData {
                name: "IsDead",
                name_hash: 2817018459,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimDynamicState, is_dead),
            },
            FieldInfoData {
                name: "IsFpsAimingDisabled",
                name_hash: 738876207,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimDynamicState, is_fps_aiming_disabled),
            },
            FieldInfoData {
                name: "IsSnapZoomed",
                name_hash: 2931311237,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimDynamicState, is_snap_zoomed),
            },
            FieldInfoData {
                name: "SnapZoomAllowed",
                name_hash: 3690882086,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimDynamicState, snap_zoom_allowed),
            },
            FieldInfoData {
                name: "ZoomHasChanged",
                name_hash: 890666442,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimDynamicState, zoom_has_changed),
            },
            FieldInfoData {
                name: "ZoomInAimingHelpActive",
                name_hash: 1065915469,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimDynamicState, zoom_in_aiming_help_active),
            },
            FieldInfoData {
                name: "AimAssistOptionEnabled",
                name_hash: 3930055625,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingSimDynamicState, aim_assist_option_enabled),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                name_hash: 4279507097,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(AimingSimDynamicState, field_flag_changed0),
            },
            FieldInfoData {
                name: "FieldFlagChanged1",
                name_hash: 4279507096,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static AIMINGSIMDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingSimDynamicState-Array",
    name_hash: 103522913,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("AimingSimDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AimingSimStaticState {
    pub local_player_id: super::core::LocalPlayerId,
    pub yaw_input_action: i32,
    pub pitch_input_action: i32,
    pub field_flag_changed0: u8,
}

pub trait AimingSimStaticStateTrait: TypeObject {
    fn local_player_id(&self) -> &super::core::LocalPlayerId;
    fn local_player_id_mut(&mut self) -> &mut super::core::LocalPlayerId;
    fn yaw_input_action(&self) -> &i32;
    fn yaw_input_action_mut(&mut self) -> &mut i32;
    fn pitch_input_action(&self) -> &i32;
    fn pitch_input_action_mut(&mut self) -> &mut i32;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl AimingSimStaticStateTrait for AimingSimStaticState {
    fn local_player_id(&self) -> &super::core::LocalPlayerId {
        &self.local_player_id
    }
    fn local_player_id_mut(&mut self) -> &mut super::core::LocalPlayerId {
        &mut self.local_player_id
    }
    fn yaw_input_action(&self) -> &i32 {
        &self.yaw_input_action
    }
    fn yaw_input_action_mut(&mut self) -> &mut i32 {
        &mut self.yaw_input_action
    }
    fn pitch_input_action(&self) -> &i32 {
        &self.pitch_input_action
    }
    fn pitch_input_action_mut(&mut self) -> &mut i32 {
        &mut self.pitch_input_action
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static AIMINGSIMSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingSimStaticState",
    name_hash: 1532201144,
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AimingSimStaticState as Default>::default())),
            create_boxed: || Box::new(<AimingSimStaticState as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "LocalPlayerId",
                name_hash: 1029133718,
                flags: MemberInfoFlags::new(0),
                field_type: "LocalPlayerId",
                rust_offset: offset_of!(AimingSimStaticState, local_player_id),
            },
            FieldInfoData {
                name: "YawInputAction",
                name_hash: 1432982466,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AimingSimStaticState, yaw_input_action),
            },
            FieldInfoData {
                name: "PitchInputAction",
                name_hash: 4136660683,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AimingSimStaticState, pitch_input_action),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                name_hash: 4279507097,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static AIMINGSIMSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingSimStaticState-Array",
    name_hash: 1145573900,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("AimingSimStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn min_yaw_mut(&mut self) -> &mut f32;
    fn max_yaw(&self) -> &f32;
    fn max_yaw_mut(&mut self) -> &mut f32;
    fn min_pitch(&self) -> &f32;
    fn min_pitch_mut(&mut self) -> &mut f32;
    fn max_pitch(&self) -> &f32;
    fn max_pitch_mut(&mut self) -> &mut f32;
    fn pitch_offset(&self) -> &f32;
    fn pitch_offset_mut(&mut self) -> &mut f32;
    fn yaw_offset(&self) -> &f32;
    fn yaw_offset_mut(&mut self) -> &mut f32;
}

impl AimingConstraintsTrait for AimingConstraints {
    fn min_yaw(&self) -> &f32 {
        &self.min_yaw
    }
    fn min_yaw_mut(&mut self) -> &mut f32 {
        &mut self.min_yaw
    }
    fn max_yaw(&self) -> &f32 {
        &self.max_yaw
    }
    fn max_yaw_mut(&mut self) -> &mut f32 {
        &mut self.max_yaw
    }
    fn min_pitch(&self) -> &f32 {
        &self.min_pitch
    }
    fn min_pitch_mut(&mut self) -> &mut f32 {
        &mut self.min_pitch
    }
    fn max_pitch(&self) -> &f32 {
        &self.max_pitch
    }
    fn max_pitch_mut(&mut self) -> &mut f32 {
        &mut self.max_pitch
    }
    fn pitch_offset(&self) -> &f32 {
        &self.pitch_offset
    }
    fn pitch_offset_mut(&mut self) -> &mut f32 {
        &mut self.pitch_offset
    }
    fn yaw_offset(&self) -> &f32 {
        &self.yaw_offset
    }
    fn yaw_offset_mut(&mut self) -> &mut f32 {
        &mut self.yaw_offset
    }
}

pub static AIMINGCONSTRAINTS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingConstraints",
    name_hash: 1523178198,
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AimingConstraints as Default>::default())),
            create_boxed: || Box::new(<AimingConstraints as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "MinYaw",
                name_hash: 2633709248,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingConstraints, min_yaw),
            },
            FieldInfoData {
                name: "MaxYaw",
                name_hash: 2642824094,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingConstraints, max_yaw),
            },
            FieldInfoData {
                name: "MinPitch",
                name_hash: 3374061353,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingConstraints, min_pitch),
            },
            FieldInfoData {
                name: "MaxPitch",
                name_hash: 397101687,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingConstraints, max_pitch),
            },
            FieldInfoData {
                name: "PitchOffset",
                name_hash: 1850416654,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingConstraints, pitch_offset),
            },
            FieldInfoData {
                name: "YawOffset",
                name_hash: 1194218663,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static AIMINGCONSTRAINTS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingConstraints-Array",
    name_hash: 1499496546,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("AimingConstraints"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn position_mut(&mut self) -> &mut super::core::Vec3;
    fn snap_position(&self) -> &super::core::Vec3;
    fn snap_position_mut(&mut self) -> &mut super::core::Vec3;
    fn velocity(&self) -> &super::core::Vec3;
    fn velocity_mut(&mut self) -> &mut super::core::Vec3;
    fn id(&self) -> &u64;
    fn id_mut(&mut self) -> &mut u64;
    fn radius(&self) -> &f32;
    fn radius_mut(&mut self) -> &mut f32;
    fn snap_radius(&self) -> &f32;
    fn snap_radius_mut(&mut self) -> &mut f32;
    fn is_sticky(&self) -> &bool;
    fn is_sticky_mut(&mut self) -> &mut bool;
    fn is_snap(&self) -> &bool;
    fn is_snap_mut(&mut self) -> &mut bool;
}

impl AimingEnvironmentTargetTrait for AimingEnvironmentTarget {
    fn position(&self) -> &super::core::Vec3 {
        &self.position
    }
    fn position_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.position
    }
    fn snap_position(&self) -> &super::core::Vec3 {
        &self.snap_position
    }
    fn snap_position_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.snap_position
    }
    fn velocity(&self) -> &super::core::Vec3 {
        &self.velocity
    }
    fn velocity_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.velocity
    }
    fn id(&self) -> &u64 {
        &self.id
    }
    fn id_mut(&mut self) -> &mut u64 {
        &mut self.id
    }
    fn radius(&self) -> &f32 {
        &self.radius
    }
    fn radius_mut(&mut self) -> &mut f32 {
        &mut self.radius
    }
    fn snap_radius(&self) -> &f32 {
        &self.snap_radius
    }
    fn snap_radius_mut(&mut self) -> &mut f32 {
        &mut self.snap_radius
    }
    fn is_sticky(&self) -> &bool {
        &self.is_sticky
    }
    fn is_sticky_mut(&mut self) -> &mut bool {
        &mut self.is_sticky
    }
    fn is_snap(&self) -> &bool {
        &self.is_snap
    }
    fn is_snap_mut(&mut self) -> &mut bool {
        &mut self.is_snap
    }
}

pub static AIMINGENVIRONMENTTARGET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingEnvironmentTarget",
    name_hash: 1862020548,
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AimingEnvironmentTarget as Default>::default())),
            create_boxed: || Box::new(<AimingEnvironmentTarget as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Position",
                name_hash: 3402582524,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AimingEnvironmentTarget, position),
            },
            FieldInfoData {
                name: "SnapPosition",
                name_hash: 1656387024,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AimingEnvironmentTarget, snap_position),
            },
            FieldInfoData {
                name: "Velocity",
                name_hash: 3860766482,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AimingEnvironmentTarget, velocity),
            },
            FieldInfoData {
                name: "Id",
                name_hash: 5862152,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint64",
                rust_offset: offset_of!(AimingEnvironmentTarget, id),
            },
            FieldInfoData {
                name: "Radius",
                name_hash: 3298407133,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingEnvironmentTarget, radius),
            },
            FieldInfoData {
                name: "SnapRadius",
                name_hash: 2020535217,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AimingEnvironmentTarget, snap_radius),
            },
            FieldInfoData {
                name: "IsSticky",
                name_hash: 471541312,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AimingEnvironmentTarget, is_sticky),
            },
            FieldInfoData {
                name: "IsSnap",
                name_hash: 2816397619,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static AIMINGENVIRONMENTTARGET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimingEnvironmentTarget-Array",
    name_hash: 49926768,
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
    name_hash: 4021362743,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static AIMOVERRIDEMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimOverrideMode-Array",
    name_hash: 1328109955,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("AimOverrideMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn aimer_position_mut(&mut self) -> &mut super::core::Vec3;
    fn hit_position(&self) -> &super::core::Vec3;
    fn hit_position_mut(&mut self) -> &mut super::core::Vec3;
    fn yaw(&self) -> &f32;
    fn yaw_mut(&mut self) -> &mut f32;
    fn pitch(&self) -> &f32;
    fn pitch_mut(&mut self) -> &mut f32;
    fn arm_length(&self) -> &f32;
    fn arm_length_mut(&mut self) -> &mut f32;
    fn previous_arm_length(&self) -> &f32;
    fn previous_arm_length_mut(&mut self) -> &mut f32;
    fn previous_collided_arm_length(&self) -> &f32;
    fn previous_collided_arm_length_mut(&mut self) -> &mut f32;
    fn is_colliding(&self) -> &bool;
    fn is_colliding_mut(&mut self) -> &mut bool;
}

impl SoldierThirdPersonCameraRenderStateTrait for SoldierThirdPersonCameraRenderState {
    fn aimer_position(&self) -> &super::core::Vec3 {
        &self.aimer_position
    }
    fn aimer_position_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.aimer_position
    }
    fn hit_position(&self) -> &super::core::Vec3 {
        &self.hit_position
    }
    fn hit_position_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.hit_position
    }
    fn yaw(&self) -> &f32 {
        &self.yaw
    }
    fn yaw_mut(&mut self) -> &mut f32 {
        &mut self.yaw
    }
    fn pitch(&self) -> &f32 {
        &self.pitch
    }
    fn pitch_mut(&mut self) -> &mut f32 {
        &mut self.pitch
    }
    fn arm_length(&self) -> &f32 {
        &self.arm_length
    }
    fn arm_length_mut(&mut self) -> &mut f32 {
        &mut self.arm_length
    }
    fn previous_arm_length(&self) -> &f32 {
        &self.previous_arm_length
    }
    fn previous_arm_length_mut(&mut self) -> &mut f32 {
        &mut self.previous_arm_length
    }
    fn previous_collided_arm_length(&self) -> &f32 {
        &self.previous_collided_arm_length
    }
    fn previous_collided_arm_length_mut(&mut self) -> &mut f32 {
        &mut self.previous_collided_arm_length
    }
    fn is_colliding(&self) -> &bool {
        &self.is_colliding
    }
    fn is_colliding_mut(&mut self) -> &mut bool {
        &mut self.is_colliding
    }
}

pub static SOLDIERTHIRDPERSONCAMERARENDERSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoldierThirdPersonCameraRenderState",
    name_hash: 3315303037,
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SoldierThirdPersonCameraRenderState as Default>::default())),
            create_boxed: || Box::new(<SoldierThirdPersonCameraRenderState as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "AimerPosition",
                name_hash: 2367265358,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SoldierThirdPersonCameraRenderState, aimer_position),
            },
            FieldInfoData {
                name: "HitPosition",
                name_hash: 3136845865,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SoldierThirdPersonCameraRenderState, hit_position),
            },
            FieldInfoData {
                name: "Yaw",
                name_hash: 193468618,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierThirdPersonCameraRenderState, yaw),
            },
            FieldInfoData {
                name: "Pitch",
                name_hash: 232604323,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierThirdPersonCameraRenderState, pitch),
            },
            FieldInfoData {
                name: "ArmLength",
                name_hash: 274703751,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierThirdPersonCameraRenderState, arm_length),
            },
            FieldInfoData {
                name: "PreviousArmLength",
                name_hash: 493236918,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierThirdPersonCameraRenderState, previous_arm_length),
            },
            FieldInfoData {
                name: "PreviousCollidedArmLength",
                name_hash: 1432791030,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierThirdPersonCameraRenderState, previous_collided_arm_length),
            },
            FieldInfoData {
                name: "IsColliding",
                name_hash: 3673002142,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static SOLDIERTHIRDPERSONCAMERARENDERSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoldierThirdPersonCameraRenderState-Array",
    name_hash: 670797641,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("SoldierThirdPersonCameraRenderState"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn local_transform_mut(&mut self) -> &mut super::core::LinearTransform;
    fn free_transform(&self) -> &super::core::LinearTransform;
    fn free_transform_mut(&mut self) -> &mut super::core::LinearTransform;
    fn procedural_transform(&self) -> &super::core::LinearTransform;
    fn procedural_transform_mut(&mut self) -> &mut super::core::LinearTransform;
    fn shake_transform(&self) -> &super::core::LinearTransform;
    fn shake_transform_mut(&mut self) -> &mut super::core::LinearTransform;
    fn roll_transform(&self) -> &super::core::LinearTransform;
    fn roll_transform_mut(&mut self) -> &mut super::core::LinearTransform;
    fn sim_aimer_position(&self) -> &super::core::Vec3;
    fn sim_aimer_position_mut(&mut self) -> &mut super::core::Vec3;
    fn sim_hit_position(&self) -> &super::core::Vec3;
    fn sim_hit_position_mut(&mut self) -> &mut super::core::Vec3;
    fn aiming(&self) -> &AimingHandle;
    fn aiming_mut(&mut self) -> &mut AimingHandle;
    fn sim_yaw(&self) -> &f32;
    fn sim_yaw_mut(&mut self) -> &mut f32;
    fn sim_pitch(&self) -> &f32;
    fn sim_pitch_mut(&mut self) -> &mut f32;
    fn max_pitch(&self) -> &f32;
    fn max_pitch_mut(&mut self) -> &mut f32;
    fn arm_length(&self) -> &f32;
    fn arm_length_mut(&mut self) -> &mut f32;
    fn min_reduce_pitch(&self) -> &f32;
    fn min_reduce_pitch_mut(&mut self) -> &mut f32;
    fn max_reduce_pitch(&self) -> &f32;
    fn max_reduce_pitch_mut(&mut self) -> &mut f32;
    fn max_reduced_length(&self) -> &f32;
    fn max_reduced_length_mut(&mut self) -> &mut f32;
    fn collision_width_padding(&self) -> &f32;
    fn collision_width_padding_mut(&mut self) -> &mut f32;
    fn collision_blend_in(&self) -> &f32;
    fn collision_blend_in_mut(&mut self) -> &mut f32;
    fn collision_blend_out(&self) -> &f32;
    fn collision_blend_out_mut(&mut self) -> &mut f32;
    fn free_transform_blend_value(&self) -> &f32;
    fn free_transform_blend_value_mut(&mut self) -> &mut f32;
    fn near_plane(&self) -> &f32;
    fn near_plane_mut(&mut self) -> &mut f32;
    fn far_plane(&self) -> &f32;
    fn far_plane_mut(&mut self) -> &mut f32;
    fn fov(&self) -> &f32;
    fn fov_mut(&mut self) -> &mut f32;
    fn aspect_ratio(&self) -> &f32;
    fn aspect_ratio_mut(&mut self) -> &mut f32;
    fn reduce_arm_length_looking_up(&self) -> &bool;
    fn reduce_arm_length_looking_up_mut(&mut self) -> &mut bool;
    fn field_flag_changed0(&self) -> &u32;
    fn field_flag_changed0_mut(&mut self) -> &mut u32;
}

impl SoldierThirdPersonCameraSimStateTrait for SoldierThirdPersonCameraSimState {
    fn local_transform(&self) -> &super::core::LinearTransform {
        &self.local_transform
    }
    fn local_transform_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.local_transform
    }
    fn free_transform(&self) -> &super::core::LinearTransform {
        &self.free_transform
    }
    fn free_transform_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.free_transform
    }
    fn procedural_transform(&self) -> &super::core::LinearTransform {
        &self.procedural_transform
    }
    fn procedural_transform_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.procedural_transform
    }
    fn shake_transform(&self) -> &super::core::LinearTransform {
        &self.shake_transform
    }
    fn shake_transform_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.shake_transform
    }
    fn roll_transform(&self) -> &super::core::LinearTransform {
        &self.roll_transform
    }
    fn roll_transform_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.roll_transform
    }
    fn sim_aimer_position(&self) -> &super::core::Vec3 {
        &self.sim_aimer_position
    }
    fn sim_aimer_position_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.sim_aimer_position
    }
    fn sim_hit_position(&self) -> &super::core::Vec3 {
        &self.sim_hit_position
    }
    fn sim_hit_position_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.sim_hit_position
    }
    fn aiming(&self) -> &AimingHandle {
        &self.aiming
    }
    fn aiming_mut(&mut self) -> &mut AimingHandle {
        &mut self.aiming
    }
    fn sim_yaw(&self) -> &f32 {
        &self.sim_yaw
    }
    fn sim_yaw_mut(&mut self) -> &mut f32 {
        &mut self.sim_yaw
    }
    fn sim_pitch(&self) -> &f32 {
        &self.sim_pitch
    }
    fn sim_pitch_mut(&mut self) -> &mut f32 {
        &mut self.sim_pitch
    }
    fn max_pitch(&self) -> &f32 {
        &self.max_pitch
    }
    fn max_pitch_mut(&mut self) -> &mut f32 {
        &mut self.max_pitch
    }
    fn arm_length(&self) -> &f32 {
        &self.arm_length
    }
    fn arm_length_mut(&mut self) -> &mut f32 {
        &mut self.arm_length
    }
    fn min_reduce_pitch(&self) -> &f32 {
        &self.min_reduce_pitch
    }
    fn min_reduce_pitch_mut(&mut self) -> &mut f32 {
        &mut self.min_reduce_pitch
    }
    fn max_reduce_pitch(&self) -> &f32 {
        &self.max_reduce_pitch
    }
    fn max_reduce_pitch_mut(&mut self) -> &mut f32 {
        &mut self.max_reduce_pitch
    }
    fn max_reduced_length(&self) -> &f32 {
        &self.max_reduced_length
    }
    fn max_reduced_length_mut(&mut self) -> &mut f32 {
        &mut self.max_reduced_length
    }
    fn collision_width_padding(&self) -> &f32 {
        &self.collision_width_padding
    }
    fn collision_width_padding_mut(&mut self) -> &mut f32 {
        &mut self.collision_width_padding
    }
    fn collision_blend_in(&self) -> &f32 {
        &self.collision_blend_in
    }
    fn collision_blend_in_mut(&mut self) -> &mut f32 {
        &mut self.collision_blend_in
    }
    fn collision_blend_out(&self) -> &f32 {
        &self.collision_blend_out
    }
    fn collision_blend_out_mut(&mut self) -> &mut f32 {
        &mut self.collision_blend_out
    }
    fn free_transform_blend_value(&self) -> &f32 {
        &self.free_transform_blend_value
    }
    fn free_transform_blend_value_mut(&mut self) -> &mut f32 {
        &mut self.free_transform_blend_value
    }
    fn near_plane(&self) -> &f32 {
        &self.near_plane
    }
    fn near_plane_mut(&mut self) -> &mut f32 {
        &mut self.near_plane
    }
    fn far_plane(&self) -> &f32 {
        &self.far_plane
    }
    fn far_plane_mut(&mut self) -> &mut f32 {
        &mut self.far_plane
    }
    fn fov(&self) -> &f32 {
        &self.fov
    }
    fn fov_mut(&mut self) -> &mut f32 {
        &mut self.fov
    }
    fn aspect_ratio(&self) -> &f32 {
        &self.aspect_ratio
    }
    fn aspect_ratio_mut(&mut self) -> &mut f32 {
        &mut self.aspect_ratio
    }
    fn reduce_arm_length_looking_up(&self) -> &bool {
        &self.reduce_arm_length_looking_up
    }
    fn reduce_arm_length_looking_up_mut(&mut self) -> &mut bool {
        &mut self.reduce_arm_length_looking_up
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u32 {
        &mut self.field_flag_changed0
    }
}

pub static SOLDIERTHIRDPERSONCAMERASIMSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoldierThirdPersonCameraSimState",
    name_hash: 1054455776,
    flags: MemberInfoFlags::new(73),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SoldierThirdPersonCameraSimState as Default>::default())),
            create_boxed: || Box::new(<SoldierThirdPersonCameraSimState as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "LocalTransform",
                name_hash: 3992192676,
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, local_transform),
            },
            FieldInfoData {
                name: "FreeTransform",
                name_hash: 3946747965,
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, free_transform),
            },
            FieldInfoData {
                name: "ProceduralTransform",
                name_hash: 2647165228,
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, procedural_transform),
            },
            FieldInfoData {
                name: "ShakeTransform",
                name_hash: 705802429,
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, shake_transform),
            },
            FieldInfoData {
                name: "RollTransform",
                name_hash: 820891380,
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, roll_transform),
            },
            FieldInfoData {
                name: "SimAimerPosition",
                name_hash: 623055417,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, sim_aimer_position),
            },
            FieldInfoData {
                name: "SimHitPosition",
                name_hash: 1719497566,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, sim_hit_position),
            },
            FieldInfoData {
                name: "Aiming",
                name_hash: 2495067616,
                flags: MemberInfoFlags::new(0),
                field_type: "AimingHandle",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, aiming),
            },
            FieldInfoData {
                name: "SimYaw",
                name_hash: 3351391869,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, sim_yaw),
            },
            FieldInfoData {
                name: "SimPitch",
                name_hash: 3230361748,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, sim_pitch),
            },
            FieldInfoData {
                name: "MaxPitch",
                name_hash: 397101687,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, max_pitch),
            },
            FieldInfoData {
                name: "ArmLength",
                name_hash: 274703751,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, arm_length),
            },
            FieldInfoData {
                name: "MinReducePitch",
                name_hash: 913792681,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, min_reduce_pitch),
            },
            FieldInfoData {
                name: "MaxReducePitch",
                name_hash: 490435575,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, max_reduce_pitch),
            },
            FieldInfoData {
                name: "MaxReducedLength",
                name_hash: 2588116265,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, max_reduced_length),
            },
            FieldInfoData {
                name: "CollisionWidthPadding",
                name_hash: 482590604,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, collision_width_padding),
            },
            FieldInfoData {
                name: "CollisionBlendIn",
                name_hash: 1011111549,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, collision_blend_in),
            },
            FieldInfoData {
                name: "CollisionBlendOut",
                name_hash: 3301907028,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, collision_blend_out),
            },
            FieldInfoData {
                name: "FreeTransformBlendValue",
                name_hash: 3108365207,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, free_transform_blend_value),
            },
            FieldInfoData {
                name: "NearPlane",
                name_hash: 3156145579,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, near_plane),
            },
            FieldInfoData {
                name: "FarPlane",
                name_hash: 192290566,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, far_plane),
            },
            FieldInfoData {
                name: "Fov",
                name_hash: 193443802,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, fov),
            },
            FieldInfoData {
                name: "AspectRatio",
                name_hash: 1269402004,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, aspect_ratio),
            },
            FieldInfoData {
                name: "ReduceArmLengthLookingUp",
                name_hash: 1105100645,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SoldierThirdPersonCameraSimState, reduce_arm_length_looking_up),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                name_hash: 4279507097,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static SOLDIERTHIRDPERSONCAMERASIMSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoldierThirdPersonCameraSimState-Array",
    name_hash: 1227787732,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("SoldierThirdPersonCameraSimState"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SoldierFirstPersonCameraRenderState {
    pub yaw: f32,
    pub pitch: f32,
}

pub trait SoldierFirstPersonCameraRenderStateTrait: TypeObject {
    fn yaw(&self) -> &f32;
    fn yaw_mut(&mut self) -> &mut f32;
    fn pitch(&self) -> &f32;
    fn pitch_mut(&mut self) -> &mut f32;
}

impl SoldierFirstPersonCameraRenderStateTrait for SoldierFirstPersonCameraRenderState {
    fn yaw(&self) -> &f32 {
        &self.yaw
    }
    fn yaw_mut(&mut self) -> &mut f32 {
        &mut self.yaw
    }
    fn pitch(&self) -> &f32 {
        &self.pitch
    }
    fn pitch_mut(&mut self) -> &mut f32 {
        &mut self.pitch
    }
}

pub static SOLDIERFIRSTPERSONCAMERARENDERSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoldierFirstPersonCameraRenderState",
    name_hash: 26346564,
    flags: MemberInfoFlags::new(36937),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SoldierFirstPersonCameraRenderState as Default>::default())),
            create_boxed: || Box::new(<SoldierFirstPersonCameraRenderState as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Yaw",
                name_hash: 193468618,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierFirstPersonCameraRenderState, yaw),
            },
            FieldInfoData {
                name: "Pitch",
                name_hash: 232604323,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static SOLDIERFIRSTPERSONCAMERARENDERSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoldierFirstPersonCameraRenderState-Array",
    name_hash: 490113264,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("SoldierFirstPersonCameraRenderState"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn camera_bone_local_transform_mut(&mut self) -> &mut super::core::LinearTransform;
    fn camera_bone_transform_relative_to_trajectory(&self) -> &super::core::LinearTransform;
    fn camera_bone_transform_relative_to_trajectory_mut(&mut self) -> &mut super::core::LinearTransform;
    fn roll_transform(&self) -> &super::core::LinearTransform;
    fn roll_transform_mut(&mut self) -> &mut super::core::LinearTransform;
    fn entity_space_local_transform(&self) -> &super::core::LinearTransform;
    fn entity_space_local_transform_mut(&mut self) -> &mut super::core::LinearTransform;
    fn procedural_transform(&self) -> &super::core::LinearTransform;
    fn procedural_transform_mut(&mut self) -> &mut super::core::LinearTransform;
    fn shake_transform(&self) -> &super::core::LinearTransform;
    fn shake_transform_mut(&mut self) -> &mut super::core::LinearTransform;
    fn spine_x_relative_to_camera(&self) -> &super::core::LinearTransform;
    fn spine_x_relative_to_camera_mut(&mut self) -> &mut super::core::LinearTransform;
    fn weapon_sway_transform(&self) -> &super::core::LinearTransform;
    fn weapon_sway_transform_mut(&mut self) -> &mut super::core::LinearTransform;
    fn soldier_world_position(&self) -> &super::core::Vec3;
    fn soldier_world_position_mut(&mut self) -> &mut super::core::Vec3;
    fn local_eye_position(&self) -> &super::core::Vec3;
    fn local_eye_position_mut(&mut self) -> &mut super::core::Vec3;
    fn aiming(&self) -> &AimingHandle;
    fn aiming_mut(&mut self) -> &mut AimingHandle;
    fn soldier_transform_space(&self) -> &super::state_stream::TransformSpaceHandle;
    fn soldier_transform_space_mut(&mut self) -> &mut super::state_stream::TransformSpaceHandle;
    fn spine_x_bone_index(&self) -> &u32;
    fn spine_x_bone_index_mut(&mut self) -> &mut u32;
    fn trajectory_bone_index(&self) -> &u32;
    fn trajectory_bone_index_mut(&mut self) -> &mut u32;
    fn sim_yaw(&self) -> &f32;
    fn sim_yaw_mut(&mut self) -> &mut f32;
    fn sim_pitch(&self) -> &f32;
    fn sim_pitch_mut(&mut self) -> &mut f32;
    fn spine_x_factor(&self) -> &f32;
    fn spine_x_factor_mut(&mut self) -> &mut f32;
    fn animated_camera_factor(&self) -> &f32;
    fn animated_camera_factor_mut(&mut self) -> &mut f32;
    fn animated_camera_start_pitch(&self) -> &f32;
    fn animated_camera_start_pitch_mut(&mut self) -> &mut f32;
    fn prevent_ground_clipping_distance(&self) -> &f32;
    fn prevent_ground_clipping_distance_mut(&mut self) -> &mut f32;
    fn has_valid_animation_transforms(&self) -> &bool;
    fn has_valid_animation_transforms_mut(&mut self) -> &mut bool;
    fn use_local_eye_position1p(&self) -> &bool;
    fn use_local_eye_position1p_mut(&mut self) -> &mut bool;
    fn is_animated_camera(&self) -> &bool;
    fn is_animated_camera_mut(&mut self) -> &mut bool;
    fn has_entity_space(&self) -> &bool;
    fn has_entity_space_mut(&mut self) -> &mut bool;
    fn field_flag_changed0(&self) -> &u32;
    fn field_flag_changed0_mut(&mut self) -> &mut u32;
}

impl SoldierFirstPersonCameraSimStateTrait for SoldierFirstPersonCameraSimState {
    fn camera_bone_local_transform(&self) -> &super::core::LinearTransform {
        &self.camera_bone_local_transform
    }
    fn camera_bone_local_transform_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.camera_bone_local_transform
    }
    fn camera_bone_transform_relative_to_trajectory(&self) -> &super::core::LinearTransform {
        &self.camera_bone_transform_relative_to_trajectory
    }
    fn camera_bone_transform_relative_to_trajectory_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.camera_bone_transform_relative_to_trajectory
    }
    fn roll_transform(&self) -> &super::core::LinearTransform {
        &self.roll_transform
    }
    fn roll_transform_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.roll_transform
    }
    fn entity_space_local_transform(&self) -> &super::core::LinearTransform {
        &self.entity_space_local_transform
    }
    fn entity_space_local_transform_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.entity_space_local_transform
    }
    fn procedural_transform(&self) -> &super::core::LinearTransform {
        &self.procedural_transform
    }
    fn procedural_transform_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.procedural_transform
    }
    fn shake_transform(&self) -> &super::core::LinearTransform {
        &self.shake_transform
    }
    fn shake_transform_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.shake_transform
    }
    fn spine_x_relative_to_camera(&self) -> &super::core::LinearTransform {
        &self.spine_x_relative_to_camera
    }
    fn spine_x_relative_to_camera_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.spine_x_relative_to_camera
    }
    fn weapon_sway_transform(&self) -> &super::core::LinearTransform {
        &self.weapon_sway_transform
    }
    fn weapon_sway_transform_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.weapon_sway_transform
    }
    fn soldier_world_position(&self) -> &super::core::Vec3 {
        &self.soldier_world_position
    }
    fn soldier_world_position_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.soldier_world_position
    }
    fn local_eye_position(&self) -> &super::core::Vec3 {
        &self.local_eye_position
    }
    fn local_eye_position_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.local_eye_position
    }
    fn aiming(&self) -> &AimingHandle {
        &self.aiming
    }
    fn aiming_mut(&mut self) -> &mut AimingHandle {
        &mut self.aiming
    }
    fn soldier_transform_space(&self) -> &super::state_stream::TransformSpaceHandle {
        &self.soldier_transform_space
    }
    fn soldier_transform_space_mut(&mut self) -> &mut super::state_stream::TransformSpaceHandle {
        &mut self.soldier_transform_space
    }
    fn spine_x_bone_index(&self) -> &u32 {
        &self.spine_x_bone_index
    }
    fn spine_x_bone_index_mut(&mut self) -> &mut u32 {
        &mut self.spine_x_bone_index
    }
    fn trajectory_bone_index(&self) -> &u32 {
        &self.trajectory_bone_index
    }
    fn trajectory_bone_index_mut(&mut self) -> &mut u32 {
        &mut self.trajectory_bone_index
    }
    fn sim_yaw(&self) -> &f32 {
        &self.sim_yaw
    }
    fn sim_yaw_mut(&mut self) -> &mut f32 {
        &mut self.sim_yaw
    }
    fn sim_pitch(&self) -> &f32 {
        &self.sim_pitch
    }
    fn sim_pitch_mut(&mut self) -> &mut f32 {
        &mut self.sim_pitch
    }
    fn spine_x_factor(&self) -> &f32 {
        &self.spine_x_factor
    }
    fn spine_x_factor_mut(&mut self) -> &mut f32 {
        &mut self.spine_x_factor
    }
    fn animated_camera_factor(&self) -> &f32 {
        &self.animated_camera_factor
    }
    fn animated_camera_factor_mut(&mut self) -> &mut f32 {
        &mut self.animated_camera_factor
    }
    fn animated_camera_start_pitch(&self) -> &f32 {
        &self.animated_camera_start_pitch
    }
    fn animated_camera_start_pitch_mut(&mut self) -> &mut f32 {
        &mut self.animated_camera_start_pitch
    }
    fn prevent_ground_clipping_distance(&self) -> &f32 {
        &self.prevent_ground_clipping_distance
    }
    fn prevent_ground_clipping_distance_mut(&mut self) -> &mut f32 {
        &mut self.prevent_ground_clipping_distance
    }
    fn has_valid_animation_transforms(&self) -> &bool {
        &self.has_valid_animation_transforms
    }
    fn has_valid_animation_transforms_mut(&mut self) -> &mut bool {
        &mut self.has_valid_animation_transforms
    }
    fn use_local_eye_position1p(&self) -> &bool {
        &self.use_local_eye_position1p
    }
    fn use_local_eye_position1p_mut(&mut self) -> &mut bool {
        &mut self.use_local_eye_position1p
    }
    fn is_animated_camera(&self) -> &bool {
        &self.is_animated_camera
    }
    fn is_animated_camera_mut(&mut self) -> &mut bool {
        &mut self.is_animated_camera
    }
    fn has_entity_space(&self) -> &bool {
        &self.has_entity_space
    }
    fn has_entity_space_mut(&mut self) -> &mut bool {
        &mut self.has_entity_space
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u32 {
        &mut self.field_flag_changed0
    }
}

pub static SOLDIERFIRSTPERSONCAMERASIMSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoldierFirstPersonCameraSimState",
    name_hash: 3885032313,
    flags: MemberInfoFlags::new(73),
    module: "Soldier",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SoldierFirstPersonCameraSimState as Default>::default())),
            create_boxed: || Box::new(<SoldierFirstPersonCameraSimState as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "CameraBoneLocalTransform",
                name_hash: 2274916155,
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, camera_bone_local_transform),
            },
            FieldInfoData {
                name: "CameraBoneTransformRelativeToTrajectory",
                name_hash: 3002713122,
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, camera_bone_transform_relative_to_trajectory),
            },
            FieldInfoData {
                name: "RollTransform",
                name_hash: 820891380,
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, roll_transform),
            },
            FieldInfoData {
                name: "EntitySpaceLocalTransform",
                name_hash: 604361179,
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, entity_space_local_transform),
            },
            FieldInfoData {
                name: "ProceduralTransform",
                name_hash: 2647165228,
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, procedural_transform),
            },
            FieldInfoData {
                name: "ShakeTransform",
                name_hash: 705802429,
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, shake_transform),
            },
            FieldInfoData {
                name: "SpineXRelativeToCamera",
                name_hash: 1783102794,
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, spine_x_relative_to_camera),
            },
            FieldInfoData {
                name: "WeaponSwayTransform",
                name_hash: 3482539799,
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, weapon_sway_transform),
            },
            FieldInfoData {
                name: "SoldierWorldPosition",
                name_hash: 1210083284,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, soldier_world_position),
            },
            FieldInfoData {
                name: "LocalEyePosition",
                name_hash: 1339134248,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, local_eye_position),
            },
            FieldInfoData {
                name: "Aiming",
                name_hash: 2495067616,
                flags: MemberInfoFlags::new(0),
                field_type: "AimingHandle",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, aiming),
            },
            FieldInfoData {
                name: "SoldierTransformSpace",
                name_hash: 748274311,
                flags: MemberInfoFlags::new(0),
                field_type: "TransformSpaceHandle",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, soldier_transform_space),
            },
            FieldInfoData {
                name: "SpineXBoneIndex",
                name_hash: 1258817828,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, spine_x_bone_index),
            },
            FieldInfoData {
                name: "TrajectoryBoneIndex",
                name_hash: 646665510,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, trajectory_bone_index),
            },
            FieldInfoData {
                name: "SimYaw",
                name_hash: 3351391869,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, sim_yaw),
            },
            FieldInfoData {
                name: "SimPitch",
                name_hash: 3230361748,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, sim_pitch),
            },
            FieldInfoData {
                name: "SpineXFactor",
                name_hash: 4145487377,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, spine_x_factor),
            },
            FieldInfoData {
                name: "AnimatedCameraFactor",
                name_hash: 3224333742,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, animated_camera_factor),
            },
            FieldInfoData {
                name: "AnimatedCameraStartPitch",
                name_hash: 1111906373,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, animated_camera_start_pitch),
            },
            FieldInfoData {
                name: "PreventGroundClippingDistance",
                name_hash: 3483509035,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, prevent_ground_clipping_distance),
            },
            FieldInfoData {
                name: "HasValidAnimationTransforms",
                name_hash: 633653856,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, has_valid_animation_transforms),
            },
            FieldInfoData {
                name: "UseLocalEyePosition1p",
                name_hash: 2912996106,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, use_local_eye_position1p),
            },
            FieldInfoData {
                name: "IsAnimatedCamera",
                name_hash: 2652932249,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, is_animated_camera),
            },
            FieldInfoData {
                name: "HasEntitySpace",
                name_hash: 3160576352,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SoldierFirstPersonCameraSimState, has_entity_space),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                name_hash: 4279507097,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static SOLDIERFIRSTPERSONCAMERASIMSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoldierFirstPersonCameraSimState-Array",
    name_hash: 3715498573,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("SoldierFirstPersonCameraSimState"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3706173999,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_server::SERVERGAMEPLAYERINTERNALEXTENT_TYPE_INFO),
        super_class_offset: offset_of!(SoldierServerPlayerExtent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SoldierServerPlayerExtent as Default>::default())),
            create_boxed: || Box::new(<SoldierServerPlayerExtent as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SOLDIERSERVERPLAYEREXTENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoldierServerPlayerExtent-Array",
    name_hash: 3361212315,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("SoldierServerPlayerExtent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3085147569,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_server::SERVERCHARACTERLOOKATTRIGGERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerLookAtTriggerEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerLookAtTriggerEntity as Default>::default())),
            create_boxed: || Box::new(<ServerLookAtTriggerEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERLOOKATTRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLookAtTriggerEntity-Array",
    name_hash: 3861630981,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerLookAtTriggerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3482456461,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_server::SERVERTRIGGERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerCharacterStateTriggerEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCharacterStateTriggerEntity as Default>::default())),
            create_boxed: || Box::new(<ServerCharacterStateTriggerEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERCHARACTERSTATETRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCharacterStateTriggerEntity-Array",
    name_hash: 1415689529,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerCharacterStateTriggerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3697562542,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerWeaponStateEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerWeaponStateEntity as Default>::default())),
            create_boxed: || Box::new(<ServerWeaponStateEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERWEAPONSTATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWeaponStateEntity-Array",
    name_hash: 2875882522,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerWeaponStateEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 350914647,
    flags: MemberInfoFlags::new(101),
    module: "Soldier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerStateEventGateEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerStateEventGateEntity as Default>::default())),
            create_boxed: || Box::new(<ServerStateEventGateEntity as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SERVERSTATEEVENTGATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStateEventGateEntity-Array",
    name_hash: 1835262307,
    flags: MemberInfoFlags::new(145),
    module: "Soldier",
    data: TypeInfoData::Array("ServerStateEventGateEntity"),
    array_type: None,
    alignment: 8,
};


