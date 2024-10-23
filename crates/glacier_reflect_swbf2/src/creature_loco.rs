use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_creature_loco_types(registry: &mut TypeRegistry) {
    registry.register_type(SERVERCREATUREFOLLOWWAYPOINTSENTITY_TYPE_INFO);
    registry.register_type(SERVERCREATUREFOLLOWWAYPOINTSENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCREATUREFOLLOWWAYPOINTSEGMENTENTITY_TYPE_INFO);
    registry.register_type(SERVERCREATUREFOLLOWWAYPOINTSEGMENTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCREATURECOLLISIONGROUPENTITY_TYPE_INFO);
    registry.register_type(SERVERCREATURECOLLISIONGROUPENTITY_ARRAY_TYPE_INFO);
    registry.register_type(IMOVEMENTPROVIDER_TYPE_INFO);
    registry.register_type(IMOVEMENTPROVIDER_ARRAY_TYPE_INFO);
    registry.register_type(ISTEERINGPROVIDER_TYPE_INFO);
    registry.register_type(ISTEERINGPROVIDER_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREWAYPOINT_PLAYANIMATION_TYPE_INFO);
    registry.register_type(CREATUREWAYPOINT_PLAYANIMATION_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREWAYPOINT_PAUSE_TYPE_INFO);
    registry.register_type(CREATUREWAYPOINT_PAUSE_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREWAYPOINT_TYPE_INFO);
    registry.register_type(CREATUREWAYPOINT_ARRAY_TYPE_INFO);
    registry.register_type(CL_PROCEDURALMOTION_TYPE_INFO);
    registry.register_type(CL_PROCEDURALMOTION_ARRAY_TYPE_INFO);
    registry.register_type(CL_CURVESTEERING_TYPE_INFO);
    registry.register_type(CL_CURVESTEERING_ARRAY_TYPE_INFO);
    registry.register_type(CLCOLAVOIDINGSTEERING_TYPE_INFO);
    registry.register_type(CLCOLAVOIDINGSTEERING_ARRAY_TYPE_INFO);
    registry.register_type(IASSESSOR_TYPE_INFO);
    registry.register_type(IASSESSOR_ARRAY_TYPE_INFO);
    registry.register_type(CLCLIENTSTATE_TYPE_INFO);
    registry.register_type(CLCLIENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(CLSTATE_TYPE_INFO);
    registry.register_type(CLSTATE_ARRAY_TYPE_INFO);
    registry.register_type(CLCONDUITSTATE_TYPE_INFO);
    registry.register_type(CLCONDUITSTATE_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCREATURESPAWNENTITY_TYPE_INFO);
    registry.register_type(CLIENTCREATURESPAWNENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCREATURESPAWNENTITY_TYPE_INFO);
    registry.register_type(SERVERCREATURESPAWNENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CREATURELOCOENTITY_TYPE_INFO);
    registry.register_type(CREATURELOCOENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTUNSPAWNENTITY_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTUNSPAWNENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWBASEENTITY_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWBASEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCREATUREFOLLOWWAYPOINTPROVIDERENTITY_TYPE_INFO);
    registry.register_type(SERVERCREATUREFOLLOWWAYPOINTPROVIDERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCREATUREFOLLOWWAYPOINTPROVIDERENTITY_TYPE_INFO);
    registry.register_type(CLIENTCREATUREFOLLOWWAYPOINTPROVIDERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTCLOSESTCHOOSERENTITY_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTCLOSESTCHOOSERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTOCCUPANCYCHOOSERENTITY_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTOCCUPANCYCHOOSERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTBOOLCHOOSERENTITY_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTBOOLCHOOSERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CREATURECONFIGURATIONPROVIDERENTITY_TYPE_INFO);
    registry.register_type(CREATURECONFIGURATIONPROVIDERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREBASEWAYPOINTPROVIDERENTITY_TYPE_INFO);
    registry.register_type(CREATUREBASEWAYPOINTPROVIDERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLINFLUENCEFILTERENTITY_TYPE_INFO);
    registry.register_type(CLINFLUENCEFILTERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLINFLUENCECOMPAREENTITY_TYPE_INFO);
    registry.register_type(CLINFLUENCECOMPAREENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLAPPLYINFLUENCEENTITY_TYPE_INFO);
    registry.register_type(CLAPPLYINFLUENCEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCREATURELOCOMOTORENTITY_TYPE_INFO);
    registry.register_type(CLIENTCREATURELOCOMOTORENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCREATUREFOLLOWWAYPOINTSENTITY_TYPE_INFO);
    registry.register_type(CLIENTCREATUREFOLLOWWAYPOINTSENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCREATUREFOLLOWWAYPOINTSEGMENTENTITY_TYPE_INFO);
    registry.register_type(CLIENTCREATUREFOLLOWWAYPOINTSEGMENTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCREATURECOLLISIONGROUPENTITY_TYPE_INFO);
    registry.register_type(CLIENTCREATURECOLLISIONGROUPENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CREATURESPAWNENTITYDATA_TYPE_INFO);
    registry.register_type(CREATURESPAWNENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCREATURELOCOMOTORENTITY_TYPE_INFO);
    registry.register_type(SERVERCREATURELOCOMOTORENTITY_ARRAY_TYPE_INFO);
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerCreatureFollowWaypointsEntity {
    pub _glacier_base: ServerCreatureFollowWaypointSegmentEntity,
}

pub trait ServerCreatureFollowWaypointsEntityTrait: ServerCreatureFollowWaypointSegmentEntityTrait {
}

impl ServerCreatureFollowWaypointsEntityTrait for ServerCreatureFollowWaypointsEntity {
}

impl ServerCreatureFollowWaypointSegmentEntityTrait for ServerCreatureFollowWaypointsEntity {
}

impl CreatureFollowBaseEntityTrait for ServerCreatureFollowWaypointsEntity {
}

impl super::entity::EntityTrait for ServerCreatureFollowWaypointsEntity {
}

impl super::entity::EntityBusPeerTrait for ServerCreatureFollowWaypointsEntity {
}

pub static SERVERCREATUREFOLLOWWAYPOINTSENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreatureFollowWaypointsEntity",
    name_hash: 688844889,
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERCREATUREFOLLOWWAYPOINTSEGMENTENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerCreatureFollowWaypointsEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCreatureFollowWaypointsEntity as Default>::default())),
            create_boxed: || Box::new(<ServerCreatureFollowWaypointsEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCREATUREFOLLOWWAYPOINTSENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCreatureFollowWaypointsEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCREATUREFOLLOWWAYPOINTSENTITY_TYPE_INFO
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


pub static SERVERCREATUREFOLLOWWAYPOINTSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreatureFollowWaypointsEntity-Array",
    name_hash: 1019854445,
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("ServerCreatureFollowWaypointsEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerCreatureFollowWaypointSegmentEntity {
    pub _glacier_base: CreatureFollowBaseEntity,
}

pub trait ServerCreatureFollowWaypointSegmentEntityTrait: CreatureFollowBaseEntityTrait {
}

impl ServerCreatureFollowWaypointSegmentEntityTrait for ServerCreatureFollowWaypointSegmentEntity {
}

impl CreatureFollowBaseEntityTrait for ServerCreatureFollowWaypointSegmentEntity {
}

impl super::entity::EntityTrait for ServerCreatureFollowWaypointSegmentEntity {
}

impl super::entity::EntityBusPeerTrait for ServerCreatureFollowWaypointSegmentEntity {
}

pub static SERVERCREATUREFOLLOWWAYPOINTSEGMENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreatureFollowWaypointSegmentEntity",
    name_hash: 4019297577,
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREFOLLOWBASEENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerCreatureFollowWaypointSegmentEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCreatureFollowWaypointSegmentEntity as Default>::default())),
            create_boxed: || Box::new(<ServerCreatureFollowWaypointSegmentEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCREATUREFOLLOWWAYPOINTSEGMENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCreatureFollowWaypointSegmentEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCREATUREFOLLOWWAYPOINTSEGMENTENTITY_TYPE_INFO
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


pub static SERVERCREATUREFOLLOWWAYPOINTSEGMENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreatureFollowWaypointSegmentEntity-Array",
    name_hash: 2653422493,
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("ServerCreatureFollowWaypointSegmentEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerCreatureCollisionGroupEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerCreatureCollisionGroupEntityTrait: super::entity::EntityTrait {
}

impl ServerCreatureCollisionGroupEntityTrait for ServerCreatureCollisionGroupEntity {
}

impl super::entity::EntityTrait for ServerCreatureCollisionGroupEntity {
}

impl super::entity::EntityBusPeerTrait for ServerCreatureCollisionGroupEntity {
}

pub static SERVERCREATURECOLLISIONGROUPENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreatureCollisionGroupEntity",
    name_hash: 717101145,
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerCreatureCollisionGroupEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCreatureCollisionGroupEntity as Default>::default())),
            create_boxed: || Box::new(<ServerCreatureCollisionGroupEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCREATURECOLLISIONGROUPENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCreatureCollisionGroupEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCREATURECOLLISIONGROUPENTITY_TYPE_INFO
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


pub static SERVERCREATURECOLLISIONGROUPENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreatureCollisionGroupEntity-Array",
    name_hash: 2671175277,
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("ServerCreatureCollisionGroupEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct IMovementProvider {
}

pub trait IMovementProviderTrait: TypeObject {
}

impl IMovementProviderTrait for IMovementProvider {
}

pub static IMOVEMENTPROVIDER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IMovementProvider",
    name_hash: 1963913358,
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IMovementProvider as Default>::default())),
            create_boxed: || Box::new(<IMovementProvider as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(IMOVEMENTPROVIDER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IMovementProvider {
    fn type_info(&self) -> &'static TypeInfo {
        IMOVEMENTPROVIDER_TYPE_INFO
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


pub static IMOVEMENTPROVIDER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IMovementProvider-Array",
    name_hash: 4289411514,
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("IMovementProvider"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ISteeringProvider {
}

pub trait ISteeringProviderTrait: TypeObject {
}

impl ISteeringProviderTrait for ISteeringProvider {
}

pub static ISTEERINGPROVIDER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ISteeringProvider",
    name_hash: 1728881624,
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ISteeringProvider as Default>::default())),
            create_boxed: || Box::new(<ISteeringProvider as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(ISTEERINGPROVIDER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ISteeringProvider {
    fn type_info(&self) -> &'static TypeInfo {
        ISTEERINGPROVIDER_TYPE_INFO
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


pub static ISTEERINGPROVIDER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ISteeringProvider-Array",
    name_hash: 266157676,
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("ISteeringProvider"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CreatureWaypointPlayAnimation {
    pub _glacier_base: CreatureWaypoint,
}

pub trait CreatureWaypointPlayAnimationTrait: CreatureWaypointTrait {
}

impl CreatureWaypointPlayAnimationTrait for CreatureWaypointPlayAnimation {
}

impl CreatureWaypointTrait for CreatureWaypointPlayAnimation {
}

pub static CREATUREWAYPOINT_PLAYANIMATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureWaypoint_PlayAnimation",
    name_hash: 3537698760,
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREWAYPOINT_TYPE_INFO),
        super_class_offset: offset_of!(CreatureWaypointPlayAnimation, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureWaypointPlayAnimation as Default>::default())),
            create_boxed: || Box::new(<CreatureWaypointPlayAnimation as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CREATUREWAYPOINT_PLAYANIMATION_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CreatureWaypointPlayAnimation {
    fn type_info(&self) -> &'static TypeInfo {
        CREATUREWAYPOINT_PLAYANIMATION_TYPE_INFO
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


pub static CREATUREWAYPOINT_PLAYANIMATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureWaypoint_PlayAnimation-Array",
    name_hash: 904909436,
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CreatureWaypoint_PlayAnimation"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CreatureWaypointPause {
    pub _glacier_base: CreatureWaypoint,
}

pub trait CreatureWaypointPauseTrait: CreatureWaypointTrait {
}

impl CreatureWaypointPauseTrait for CreatureWaypointPause {
}

impl CreatureWaypointTrait for CreatureWaypointPause {
}

pub static CREATUREWAYPOINT_PAUSE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureWaypoint_Pause",
    name_hash: 3214906184,
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREWAYPOINT_TYPE_INFO),
        super_class_offset: offset_of!(CreatureWaypointPause, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureWaypointPause as Default>::default())),
            create_boxed: || Box::new(<CreatureWaypointPause as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CREATUREWAYPOINT_PAUSE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CreatureWaypointPause {
    fn type_info(&self) -> &'static TypeInfo {
        CREATUREWAYPOINT_PAUSE_TYPE_INFO
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


pub static CREATUREWAYPOINT_PAUSE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureWaypoint_Pause-Array",
    name_hash: 1513293308,
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CreatureWaypoint_Pause"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CreatureWaypoint {
}

pub trait CreatureWaypointTrait: TypeObject {
}

impl CreatureWaypointTrait for CreatureWaypoint {
}

pub static CREATUREWAYPOINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureWaypoint",
    name_hash: 4085496549,
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureWaypoint as Default>::default())),
            create_boxed: || Box::new(<CreatureWaypoint as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CREATUREWAYPOINT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CreatureWaypoint {
    fn type_info(&self) -> &'static TypeInfo {
        CREATUREWAYPOINT_TYPE_INFO
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


pub static CREATUREWAYPOINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureWaypoint-Array",
    name_hash: 1278870481,
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CreatureWaypoint"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CLProceduralMotion {
    pub _glacier_base: IMovementProvider,
}

pub trait CLProceduralMotionTrait: IMovementProviderTrait {
}

impl CLProceduralMotionTrait for CLProceduralMotion {
}

impl IMovementProviderTrait for CLProceduralMotion {
}

pub static CL_PROCEDURALMOTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CL_ProceduralMotion",
    name_hash: 949260910,
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IMOVEMENTPROVIDER_TYPE_INFO),
        super_class_offset: offset_of!(CLProceduralMotion, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CLProceduralMotion as Default>::default())),
            create_boxed: || Box::new(<CLProceduralMotion as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CL_PROCEDURALMOTION_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CLProceduralMotion {
    fn type_info(&self) -> &'static TypeInfo {
        CL_PROCEDURALMOTION_TYPE_INFO
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


pub static CL_PROCEDURALMOTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CL_ProceduralMotion-Array",
    name_hash: 3098446682,
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CL_ProceduralMotion"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CLCurveSteering {
    pub _glacier_base: ISteeringProvider,
}

pub trait CLCurveSteeringTrait: ISteeringProviderTrait {
}

impl CLCurveSteeringTrait for CLCurveSteering {
}

impl ISteeringProviderTrait for CLCurveSteering {
}

pub static CL_CURVESTEERING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CL_CurveSteering",
    name_hash: 3368270615,
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ISTEERINGPROVIDER_TYPE_INFO),
        super_class_offset: offset_of!(CLCurveSteering, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CLCurveSteering as Default>::default())),
            create_boxed: || Box::new(<CLCurveSteering as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CL_CURVESTEERING_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CLCurveSteering {
    fn type_info(&self) -> &'static TypeInfo {
        CL_CURVESTEERING_TYPE_INFO
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


pub static CL_CURVESTEERING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CL_CurveSteering-Array",
    name_hash: 2228773795,
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CL_CurveSteering"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CLColAvoidingSteering {
    pub _glacier_base: CLCurveSteering,
}

pub trait CLColAvoidingSteeringTrait: CLCurveSteeringTrait {
}

impl CLColAvoidingSteeringTrait for CLColAvoidingSteering {
}

impl CLCurveSteeringTrait for CLColAvoidingSteering {
}

impl ISteeringProviderTrait for CLColAvoidingSteering {
}

pub static CLCOLAVOIDINGSTEERING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLColAvoidingSteering",
    name_hash: 601321930,
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CL_CURVESTEERING_TYPE_INFO),
        super_class_offset: offset_of!(CLColAvoidingSteering, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CLColAvoidingSteering as Default>::default())),
            create_boxed: || Box::new(<CLColAvoidingSteering as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLCOLAVOIDINGSTEERING_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CLColAvoidingSteering {
    fn type_info(&self) -> &'static TypeInfo {
        CLCOLAVOIDINGSTEERING_TYPE_INFO
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


pub static CLCOLAVOIDINGSTEERING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLColAvoidingSteering-Array",
    name_hash: 3819978110,
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CLColAvoidingSteering"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct IAssessor {
}

pub trait IAssessorTrait: TypeObject {
}

impl IAssessorTrait for IAssessor {
}

pub static IASSESSOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IAssessor",
    name_hash: 2483990421,
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IAssessor as Default>::default())),
            create_boxed: || Box::new(<IAssessor as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(IASSESSOR_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IAssessor {
    fn type_info(&self) -> &'static TypeInfo {
        IASSESSOR_TYPE_INFO
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


pub static IASSESSOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IAssessor-Array",
    name_hash: 2221948705,
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("IAssessor"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CLClientState {
    pub _glacier_base: CLState,
}

pub trait CLClientStateTrait: CLStateTrait {
}

impl CLClientStateTrait for CLClientState {
}

impl CLStateTrait for CLClientState {
}

impl IAssessorTrait for CLClientState {
}

pub static CLCLIENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLClientState",
    name_hash: 3388397284,
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLSTATE_TYPE_INFO),
        super_class_offset: offset_of!(CLClientState, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CLClientState as Default>::default())),
            create_boxed: || Box::new(<CLClientState as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLCLIENTSTATE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CLClientState {
    fn type_info(&self) -> &'static TypeInfo {
        CLCLIENTSTATE_TYPE_INFO
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


pub static CLCLIENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLClientState-Array",
    name_hash: 1688640208,
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CLClientState"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CLState {
    pub _glacier_base: IAssessor,
}

pub trait CLStateTrait: IAssessorTrait {
}

impl CLStateTrait for CLState {
}

impl IAssessorTrait for CLState {
}

pub static CLSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLState",
    name_hash: 866975357,
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IASSESSOR_TYPE_INFO),
        super_class_offset: offset_of!(CLState, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CLState as Default>::default())),
            create_boxed: || Box::new(<CLState as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLSTATE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CLState {
    fn type_info(&self) -> &'static TypeInfo {
        CLSTATE_TYPE_INFO
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


pub static CLSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLState-Array",
    name_hash: 1413090121,
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CLState"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CLConduitState {
    pub _glacier_base: CLState,
}

pub trait CLConduitStateTrait: CLStateTrait {
}

impl CLConduitStateTrait for CLConduitState {
}

impl CLStateTrait for CLConduitState {
}

impl IAssessorTrait for CLConduitState {
}

pub static CLCONDUITSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLConduitState",
    name_hash: 757824691,
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLSTATE_TYPE_INFO),
        super_class_offset: offset_of!(CLConduitState, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CLConduitState as Default>::default())),
            create_boxed: || Box::new(<CLConduitState as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLCONDUITSTATE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CLConduitState {
    fn type_info(&self) -> &'static TypeInfo {
        CLCONDUITSTATE_TYPE_INFO
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


pub static CLCONDUITSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLConduitState-Array",
    name_hash: 2312595463,
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CLConduitState"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientCreatureSpawnEntity {
    pub _glacier_base: super::dice_commons::ClientBlueprintSpawnEntity,
}

pub trait ClientCreatureSpawnEntityTrait: super::dice_commons::ClientBlueprintSpawnEntityTrait {
}

impl ClientCreatureSpawnEntityTrait for ClientCreatureSpawnEntity {
}

impl super::dice_commons::ClientBlueprintSpawnEntityTrait for ClientCreatureSpawnEntity {
}

impl super::dice_commons::BlueprintSpawnEntityTrait for ClientCreatureSpawnEntity {
}

impl super::entity::SpatialEntityTrait for ClientCreatureSpawnEntity {
}

impl super::entity::EntityTrait for ClientCreatureSpawnEntity {
}

impl super::entity::EntityBusPeerTrait for ClientCreatureSpawnEntity {
}

pub static CLIENTCREATURESPAWNENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCreatureSpawnEntity",
    name_hash: 419339743,
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::dice_commons::CLIENTBLUEPRINTSPAWNENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientCreatureSpawnEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCreatureSpawnEntity as Default>::default())),
            create_boxed: || Box::new(<ClientCreatureSpawnEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCREATURESPAWNENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCreatureSpawnEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCREATURESPAWNENTITY_TYPE_INFO
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


pub static CLIENTCREATURESPAWNENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCreatureSpawnEntity-Array",
    name_hash: 1997632235,
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("ClientCreatureSpawnEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerCreatureSpawnEntity {
    pub _glacier_base: super::dice_commons::ServerBlueprintSpawnEntity,
}

pub trait ServerCreatureSpawnEntityTrait: super::dice_commons::ServerBlueprintSpawnEntityTrait {
}

impl ServerCreatureSpawnEntityTrait for ServerCreatureSpawnEntity {
}

impl super::dice_commons::ServerBlueprintSpawnEntityTrait for ServerCreatureSpawnEntity {
}

impl super::dice_commons::BlueprintSpawnEntityTrait for ServerCreatureSpawnEntity {
}

impl super::entity::SpatialEntityTrait for ServerCreatureSpawnEntity {
}

impl super::entity::EntityTrait for ServerCreatureSpawnEntity {
}

impl super::entity::EntityBusPeerTrait for ServerCreatureSpawnEntity {
}

pub static SERVERCREATURESPAWNENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreatureSpawnEntity",
    name_hash: 2055447555,
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::dice_commons::SERVERBLUEPRINTSPAWNENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerCreatureSpawnEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCreatureSpawnEntity as Default>::default())),
            create_boxed: || Box::new(<ServerCreatureSpawnEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCREATURESPAWNENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCreatureSpawnEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCREATURESPAWNENTITY_TYPE_INFO
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


pub static SERVERCREATURESPAWNENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreatureSpawnEntity-Array",
    name_hash: 441489079,
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("ServerCreatureSpawnEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CreatureLocoEntity {
    pub _glacier_base: super::a_i_tools::LocoEntity,
}

pub trait CreatureLocoEntityTrait: super::a_i_tools::LocoEntityTrait {
}

impl CreatureLocoEntityTrait for CreatureLocoEntity {
}

impl super::a_i_tools::LocoEntityTrait for CreatureLocoEntity {
}

impl super::entity::EntityTrait for CreatureLocoEntity {
}

impl super::entity::EntityBusPeerTrait for CreatureLocoEntity {
}

pub static CREATURELOCOENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureLocoEntity",
    name_hash: 1897448306,
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::a_i_tools::LOCOENTITY_TYPE_INFO),
        super_class_offset: offset_of!(CreatureLocoEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureLocoEntity as Default>::default())),
            create_boxed: || Box::new(<CreatureLocoEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CREATURELOCOENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CreatureLocoEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CREATURELOCOENTITY_TYPE_INFO
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


pub static CREATURELOCOENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureLocoEntity-Array",
    name_hash: 2510764614,
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CreatureLocoEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CreatureFollowWaypointUnspawnEntity {
    pub _glacier_base: CreatureFollowBaseEntity,
}

pub trait CreatureFollowWaypointUnspawnEntityTrait: CreatureFollowBaseEntityTrait {
}

impl CreatureFollowWaypointUnspawnEntityTrait for CreatureFollowWaypointUnspawnEntity {
}

impl CreatureFollowBaseEntityTrait for CreatureFollowWaypointUnspawnEntity {
}

impl super::entity::EntityTrait for CreatureFollowWaypointUnspawnEntity {
}

impl super::entity::EntityBusPeerTrait for CreatureFollowWaypointUnspawnEntity {
}

pub static CREATUREFOLLOWWAYPOINTUNSPAWNENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointUnspawnEntity",
    name_hash: 260601551,
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREFOLLOWBASEENTITY_TYPE_INFO),
        super_class_offset: offset_of!(CreatureFollowWaypointUnspawnEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureFollowWaypointUnspawnEntity as Default>::default())),
            create_boxed: || Box::new(<CreatureFollowWaypointUnspawnEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CREATUREFOLLOWWAYPOINTUNSPAWNENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CreatureFollowWaypointUnspawnEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CREATUREFOLLOWWAYPOINTUNSPAWNENTITY_TYPE_INFO
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


pub static CREATUREFOLLOWWAYPOINTUNSPAWNENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointUnspawnEntity-Array",
    name_hash: 3192341499,
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CreatureFollowWaypointUnspawnEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CreatureFollowBaseEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait CreatureFollowBaseEntityTrait: super::entity::EntityTrait {
}

impl CreatureFollowBaseEntityTrait for CreatureFollowBaseEntity {
}

impl super::entity::EntityTrait for CreatureFollowBaseEntity {
}

impl super::entity::EntityBusPeerTrait for CreatureFollowBaseEntity {
}

pub static CREATUREFOLLOWBASEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowBaseEntity",
    name_hash: 3965562297,
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(CreatureFollowBaseEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureFollowBaseEntity as Default>::default())),
            create_boxed: || Box::new(<CreatureFollowBaseEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CREATUREFOLLOWBASEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CreatureFollowBaseEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CREATUREFOLLOWBASEENTITY_TYPE_INFO
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


pub static CREATUREFOLLOWBASEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowBaseEntity-Array",
    name_hash: 4101061645,
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CreatureFollowBaseEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerCreatureFollowWaypointProviderEntity {
    pub _glacier_base: CreatureBaseWaypointProviderEntity,
}

pub trait ServerCreatureFollowWaypointProviderEntityTrait: CreatureBaseWaypointProviderEntityTrait {
}

impl ServerCreatureFollowWaypointProviderEntityTrait for ServerCreatureFollowWaypointProviderEntity {
}

impl CreatureBaseWaypointProviderEntityTrait for ServerCreatureFollowWaypointProviderEntity {
}

impl super::entity::EntityTrait for ServerCreatureFollowWaypointProviderEntity {
}

impl super::entity::EntityBusPeerTrait for ServerCreatureFollowWaypointProviderEntity {
}

pub static SERVERCREATUREFOLLOWWAYPOINTPROVIDERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreatureFollowWaypointProviderEntity",
    name_hash: 1239115595,
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREBASEWAYPOINTPROVIDERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerCreatureFollowWaypointProviderEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCreatureFollowWaypointProviderEntity as Default>::default())),
            create_boxed: || Box::new(<ServerCreatureFollowWaypointProviderEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCREATUREFOLLOWWAYPOINTPROVIDERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCreatureFollowWaypointProviderEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCREATUREFOLLOWWAYPOINTPROVIDERENTITY_TYPE_INFO
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


pub static SERVERCREATUREFOLLOWWAYPOINTPROVIDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreatureFollowWaypointProviderEntity-Array",
    name_hash: 3807176319,
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("ServerCreatureFollowWaypointProviderEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientCreatureFollowWaypointProviderEntity {
    pub _glacier_base: CreatureBaseWaypointProviderEntity,
}

pub trait ClientCreatureFollowWaypointProviderEntityTrait: CreatureBaseWaypointProviderEntityTrait {
}

impl ClientCreatureFollowWaypointProviderEntityTrait for ClientCreatureFollowWaypointProviderEntity {
}

impl CreatureBaseWaypointProviderEntityTrait for ClientCreatureFollowWaypointProviderEntity {
}

impl super::entity::EntityTrait for ClientCreatureFollowWaypointProviderEntity {
}

impl super::entity::EntityBusPeerTrait for ClientCreatureFollowWaypointProviderEntity {
}

pub static CLIENTCREATUREFOLLOWWAYPOINTPROVIDERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCreatureFollowWaypointProviderEntity",
    name_hash: 3743477271,
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREBASEWAYPOINTPROVIDERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientCreatureFollowWaypointProviderEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCreatureFollowWaypointProviderEntity as Default>::default())),
            create_boxed: || Box::new(<ClientCreatureFollowWaypointProviderEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCREATUREFOLLOWWAYPOINTPROVIDERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCreatureFollowWaypointProviderEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCREATUREFOLLOWWAYPOINTPROVIDERENTITY_TYPE_INFO
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


pub static CLIENTCREATUREFOLLOWWAYPOINTPROVIDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCreatureFollowWaypointProviderEntity-Array",
    name_hash: 118478499,
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("ClientCreatureFollowWaypointProviderEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CreatureFollowWaypointClosestChooserEntity {
    pub _glacier_base: CreatureFollowBaseEntity,
}

pub trait CreatureFollowWaypointClosestChooserEntityTrait: CreatureFollowBaseEntityTrait {
}

impl CreatureFollowWaypointClosestChooserEntityTrait for CreatureFollowWaypointClosestChooserEntity {
}

impl CreatureFollowBaseEntityTrait for CreatureFollowWaypointClosestChooserEntity {
}

impl super::entity::EntityTrait for CreatureFollowWaypointClosestChooserEntity {
}

impl super::entity::EntityBusPeerTrait for CreatureFollowWaypointClosestChooserEntity {
}

pub static CREATUREFOLLOWWAYPOINTCLOSESTCHOOSERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointClosestChooserEntity",
    name_hash: 3784309841,
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREFOLLOWBASEENTITY_TYPE_INFO),
        super_class_offset: offset_of!(CreatureFollowWaypointClosestChooserEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureFollowWaypointClosestChooserEntity as Default>::default())),
            create_boxed: || Box::new(<CreatureFollowWaypointClosestChooserEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CREATUREFOLLOWWAYPOINTCLOSESTCHOOSERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CreatureFollowWaypointClosestChooserEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CREATUREFOLLOWWAYPOINTCLOSESTCHOOSERENTITY_TYPE_INFO
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


pub static CREATUREFOLLOWWAYPOINTCLOSESTCHOOSERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointClosestChooserEntity-Array",
    name_hash: 174354021,
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CreatureFollowWaypointClosestChooserEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CreatureFollowWaypointOccupancyChooserEntity {
    pub _glacier_base: CreatureFollowBaseEntity,
}

pub trait CreatureFollowWaypointOccupancyChooserEntityTrait: CreatureFollowBaseEntityTrait {
}

impl CreatureFollowWaypointOccupancyChooserEntityTrait for CreatureFollowWaypointOccupancyChooserEntity {
}

impl CreatureFollowBaseEntityTrait for CreatureFollowWaypointOccupancyChooserEntity {
}

impl super::entity::EntityTrait for CreatureFollowWaypointOccupancyChooserEntity {
}

impl super::entity::EntityBusPeerTrait for CreatureFollowWaypointOccupancyChooserEntity {
}

pub static CREATUREFOLLOWWAYPOINTOCCUPANCYCHOOSERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointOccupancyChooserEntity",
    name_hash: 245394335,
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREFOLLOWBASEENTITY_TYPE_INFO),
        super_class_offset: offset_of!(CreatureFollowWaypointOccupancyChooserEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureFollowWaypointOccupancyChooserEntity as Default>::default())),
            create_boxed: || Box::new(<CreatureFollowWaypointOccupancyChooserEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CREATUREFOLLOWWAYPOINTOCCUPANCYCHOOSERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CreatureFollowWaypointOccupancyChooserEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CREATUREFOLLOWWAYPOINTOCCUPANCYCHOOSERENTITY_TYPE_INFO
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


pub static CREATUREFOLLOWWAYPOINTOCCUPANCYCHOOSERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointOccupancyChooserEntity-Array",
    name_hash: 2299973675,
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CreatureFollowWaypointOccupancyChooserEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CreatureFollowWaypointBoolChooserEntity {
    pub _glacier_base: CreatureFollowBaseEntity,
}

pub trait CreatureFollowWaypointBoolChooserEntityTrait: CreatureFollowBaseEntityTrait {
}

impl CreatureFollowWaypointBoolChooserEntityTrait for CreatureFollowWaypointBoolChooserEntity {
}

impl CreatureFollowBaseEntityTrait for CreatureFollowWaypointBoolChooserEntity {
}

impl super::entity::EntityTrait for CreatureFollowWaypointBoolChooserEntity {
}

impl super::entity::EntityBusPeerTrait for CreatureFollowWaypointBoolChooserEntity {
}

pub static CREATUREFOLLOWWAYPOINTBOOLCHOOSERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointBoolChooserEntity",
    name_hash: 3105311406,
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREFOLLOWBASEENTITY_TYPE_INFO),
        super_class_offset: offset_of!(CreatureFollowWaypointBoolChooserEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureFollowWaypointBoolChooserEntity as Default>::default())),
            create_boxed: || Box::new(<CreatureFollowWaypointBoolChooserEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CREATUREFOLLOWWAYPOINTBOOLCHOOSERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CreatureFollowWaypointBoolChooserEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CREATUREFOLLOWWAYPOINTBOOLCHOOSERENTITY_TYPE_INFO
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


pub static CREATUREFOLLOWWAYPOINTBOOLCHOOSERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointBoolChooserEntity-Array",
    name_hash: 909816602,
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CreatureFollowWaypointBoolChooserEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CreatureConfigurationProviderEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait CreatureConfigurationProviderEntityTrait: super::entity::EntityTrait {
}

impl CreatureConfigurationProviderEntityTrait for CreatureConfigurationProviderEntity {
}

impl super::entity::EntityTrait for CreatureConfigurationProviderEntity {
}

impl super::entity::EntityBusPeerTrait for CreatureConfigurationProviderEntity {
}

pub static CREATURECONFIGURATIONPROVIDERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureConfigurationProviderEntity",
    name_hash: 1848491596,
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(CreatureConfigurationProviderEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureConfigurationProviderEntity as Default>::default())),
            create_boxed: || Box::new(<CreatureConfigurationProviderEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CREATURECONFIGURATIONPROVIDERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CreatureConfigurationProviderEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CREATURECONFIGURATIONPROVIDERENTITY_TYPE_INFO
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


pub static CREATURECONFIGURATIONPROVIDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureConfigurationProviderEntity-Array",
    name_hash: 1689591544,
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CreatureConfigurationProviderEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CreatureBaseWaypointProviderEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait CreatureBaseWaypointProviderEntityTrait: super::entity::EntityTrait {
}

impl CreatureBaseWaypointProviderEntityTrait for CreatureBaseWaypointProviderEntity {
}

impl super::entity::EntityTrait for CreatureBaseWaypointProviderEntity {
}

impl super::entity::EntityBusPeerTrait for CreatureBaseWaypointProviderEntity {
}

pub static CREATUREBASEWAYPOINTPROVIDERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureBaseWaypointProviderEntity",
    name_hash: 1830909994,
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(CreatureBaseWaypointProviderEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureBaseWaypointProviderEntity as Default>::default())),
            create_boxed: || Box::new(<CreatureBaseWaypointProviderEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CREATUREBASEWAYPOINTPROVIDERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CreatureBaseWaypointProviderEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CREATUREBASEWAYPOINTPROVIDERENTITY_TYPE_INFO
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


pub static CREATUREBASEWAYPOINTPROVIDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureBaseWaypointProviderEntity-Array",
    name_hash: 2723238558,
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CreatureBaseWaypointProviderEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CLInfluenceFilterEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait CLInfluenceFilterEntityTrait: super::entity::EntityTrait {
}

impl CLInfluenceFilterEntityTrait for CLInfluenceFilterEntity {
}

impl super::entity::EntityTrait for CLInfluenceFilterEntity {
}

impl super::entity::EntityBusPeerTrait for CLInfluenceFilterEntity {
}

pub static CLINFLUENCEFILTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLInfluenceFilterEntity",
    name_hash: 3654966884,
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(CLInfluenceFilterEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CLInfluenceFilterEntity as Default>::default())),
            create_boxed: || Box::new(<CLInfluenceFilterEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLINFLUENCEFILTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CLInfluenceFilterEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLINFLUENCEFILTERENTITY_TYPE_INFO
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


pub static CLINFLUENCEFILTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLInfluenceFilterEntity-Array",
    name_hash: 99730000,
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CLInfluenceFilterEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CLInfluenceCompareEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait CLInfluenceCompareEntityTrait: super::entity::EntityTrait {
}

impl CLInfluenceCompareEntityTrait for CLInfluenceCompareEntity {
}

impl super::entity::EntityTrait for CLInfluenceCompareEntity {
}

impl super::entity::EntityBusPeerTrait for CLInfluenceCompareEntity {
}

pub static CLINFLUENCECOMPAREENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLInfluenceCompareEntity",
    name_hash: 2893352739,
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(CLInfluenceCompareEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CLInfluenceCompareEntity as Default>::default())),
            create_boxed: || Box::new(<CLInfluenceCompareEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLINFLUENCECOMPAREENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CLInfluenceCompareEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLINFLUENCECOMPAREENTITY_TYPE_INFO
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


pub static CLINFLUENCECOMPAREENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLInfluenceCompareEntity-Array",
    name_hash: 1184407,
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CLInfluenceCompareEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CLApplyInfluenceEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait CLApplyInfluenceEntityTrait: super::entity::EntityTrait {
}

impl CLApplyInfluenceEntityTrait for CLApplyInfluenceEntity {
}

impl super::entity::EntityTrait for CLApplyInfluenceEntity {
}

impl super::entity::EntityBusPeerTrait for CLApplyInfluenceEntity {
}

pub static CLAPPLYINFLUENCEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLApplyInfluenceEntity",
    name_hash: 1239760432,
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(CLApplyInfluenceEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CLApplyInfluenceEntity as Default>::default())),
            create_boxed: || Box::new(<CLApplyInfluenceEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLAPPLYINFLUENCEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CLApplyInfluenceEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLAPPLYINFLUENCEENTITY_TYPE_INFO
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


pub static CLAPPLYINFLUENCEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLApplyInfluenceEntity-Array",
    name_hash: 1126992772,
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CLApplyInfluenceEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientCreatureLocoMotorEntity {
    pub _glacier_base: CreatureLocoEntity,
}

pub trait ClientCreatureLocoMotorEntityTrait: CreatureLocoEntityTrait {
}

impl ClientCreatureLocoMotorEntityTrait for ClientCreatureLocoMotorEntity {
}

impl CreatureLocoEntityTrait for ClientCreatureLocoMotorEntity {
}

impl super::a_i_tools::LocoEntityTrait for ClientCreatureLocoMotorEntity {
}

impl super::entity::EntityTrait for ClientCreatureLocoMotorEntity {
}

impl super::entity::EntityBusPeerTrait for ClientCreatureLocoMotorEntity {
}

pub static CLIENTCREATURELOCOMOTORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCreatureLocoMotorEntity",
    name_hash: 223047424,
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATURELOCOENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientCreatureLocoMotorEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCreatureLocoMotorEntity as Default>::default())),
            create_boxed: || Box::new(<ClientCreatureLocoMotorEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCREATURELOCOMOTORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCreatureLocoMotorEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCREATURELOCOMOTORENTITY_TYPE_INFO
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


pub static CLIENTCREATURELOCOMOTORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCreatureLocoMotorEntity-Array",
    name_hash: 2122405172,
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("ClientCreatureLocoMotorEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientCreatureFollowWaypointsEntity {
    pub _glacier_base: ClientCreatureFollowWaypointSegmentEntity,
}

pub trait ClientCreatureFollowWaypointsEntityTrait: ClientCreatureFollowWaypointSegmentEntityTrait {
}

impl ClientCreatureFollowWaypointsEntityTrait for ClientCreatureFollowWaypointsEntity {
}

impl ClientCreatureFollowWaypointSegmentEntityTrait for ClientCreatureFollowWaypointsEntity {
}

impl CreatureFollowBaseEntityTrait for ClientCreatureFollowWaypointsEntity {
}

impl super::entity::EntityTrait for ClientCreatureFollowWaypointsEntity {
}

impl super::entity::EntityBusPeerTrait for ClientCreatureFollowWaypointsEntity {
}

pub static CLIENTCREATUREFOLLOWWAYPOINTSENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCreatureFollowWaypointsEntity",
    name_hash: 3090853637,
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTCREATUREFOLLOWWAYPOINTSEGMENTENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientCreatureFollowWaypointsEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCreatureFollowWaypointsEntity as Default>::default())),
            create_boxed: || Box::new(<ClientCreatureFollowWaypointsEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCREATUREFOLLOWWAYPOINTSENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCreatureFollowWaypointsEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCREATUREFOLLOWWAYPOINTSENTITY_TYPE_INFO
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


pub static CLIENTCREATUREFOLLOWWAYPOINTSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCreatureFollowWaypointsEntity-Array",
    name_hash: 3437958833,
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("ClientCreatureFollowWaypointsEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientCreatureFollowWaypointSegmentEntity {
    pub _glacier_base: CreatureFollowBaseEntity,
}

pub trait ClientCreatureFollowWaypointSegmentEntityTrait: CreatureFollowBaseEntityTrait {
}

impl ClientCreatureFollowWaypointSegmentEntityTrait for ClientCreatureFollowWaypointSegmentEntity {
}

impl CreatureFollowBaseEntityTrait for ClientCreatureFollowWaypointSegmentEntity {
}

impl super::entity::EntityTrait for ClientCreatureFollowWaypointSegmentEntity {
}

impl super::entity::EntityBusPeerTrait for ClientCreatureFollowWaypointSegmentEntity {
}

pub static CLIENTCREATUREFOLLOWWAYPOINTSEGMENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCreatureFollowWaypointSegmentEntity",
    name_hash: 204493557,
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREFOLLOWBASEENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientCreatureFollowWaypointSegmentEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCreatureFollowWaypointSegmentEntity as Default>::default())),
            create_boxed: || Box::new(<ClientCreatureFollowWaypointSegmentEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCREATUREFOLLOWWAYPOINTSEGMENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCreatureFollowWaypointSegmentEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCREATUREFOLLOWWAYPOINTSEGMENTENTITY_TYPE_INFO
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


pub static CLIENTCREATUREFOLLOWWAYPOINTSEGMENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCreatureFollowWaypointSegmentEntity-Array",
    name_hash: 2576049089,
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("ClientCreatureFollowWaypointSegmentEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientCreatureCollisionGroupEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientCreatureCollisionGroupEntityTrait: super::entity::EntityTrait {
}

impl ClientCreatureCollisionGroupEntityTrait for ClientCreatureCollisionGroupEntity {
}

impl super::entity::EntityTrait for ClientCreatureCollisionGroupEntity {
}

impl super::entity::EntityBusPeerTrait for ClientCreatureCollisionGroupEntity {
}

pub static CLIENTCREATURECOLLISIONGROUPENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCreatureCollisionGroupEntity",
    name_hash: 3908193285,
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientCreatureCollisionGroupEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCreatureCollisionGroupEntity as Default>::default())),
            create_boxed: || Box::new(<ClientCreatureCollisionGroupEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCREATURECOLLISIONGROUPENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCreatureCollisionGroupEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCREATURECOLLISIONGROUPENTITY_TYPE_INFO
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


pub static CLIENTCREATURECOLLISIONGROUPENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCreatureCollisionGroupEntity-Array",
    name_hash: 3794039729,
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("ClientCreatureCollisionGroupEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CreatureSpawnEntityData {
    pub _glacier_base: super::dice_commons_shared::BlueprintSpawnReferenceObjectData,
}

pub trait CreatureSpawnEntityDataTrait: super::dice_commons_shared::BlueprintSpawnReferenceObjectDataTrait {
}

impl CreatureSpawnEntityDataTrait for CreatureSpawnEntityData {
}

impl super::dice_commons_shared::BlueprintSpawnReferenceObjectDataTrait for CreatureSpawnEntityData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        self._glacier_base.realm_mut()
    }
    fn initial_auto_spawn(&self) -> &bool {
        self._glacier_base.initial_auto_spawn()
    }
    fn initial_auto_spawn_mut(&mut self) -> &mut bool {
        self._glacier_base.initial_auto_spawn_mut()
    }
    fn auto_spawn(&self) -> &bool {
        self._glacier_base.auto_spawn()
    }
    fn auto_spawn_mut(&mut self) -> &mut bool {
        self._glacier_base.auto_spawn_mut()
    }
    fn queue_spawn_event(&self) -> &bool {
        self._glacier_base.queue_spawn_event()
    }
    fn queue_spawn_event_mut(&mut self) -> &mut bool {
        self._glacier_base.queue_spawn_event_mut()
    }
    fn use_as_spawn_point(&self) -> &bool {
        self._glacier_base.use_as_spawn_point()
    }
    fn use_as_spawn_point_mut(&mut self) -> &mut bool {
        self._glacier_base.use_as_spawn_point_mut()
    }
    fn spawns_occupy_locations(&self) -> &bool {
        self._glacier_base.spawns_occupy_locations()
    }
    fn spawns_occupy_locations_mut(&mut self) -> &mut bool {
        self._glacier_base.spawns_occupy_locations_mut()
    }
    fn initial_spawn_delay(&self) -> &f32 {
        self._glacier_base.initial_spawn_delay()
    }
    fn initial_spawn_delay_mut(&mut self) -> &mut f32 {
        self._glacier_base.initial_spawn_delay_mut()
    }
    fn spawn_delay(&self) -> &f32 {
        self._glacier_base.spawn_delay()
    }
    fn spawn_delay_mut(&mut self) -> &mut f32 {
        self._glacier_base.spawn_delay_mut()
    }
    fn max_count(&self) -> &i32 {
        self._glacier_base.max_count()
    }
    fn max_count_mut(&mut self) -> &mut i32 {
        self._glacier_base.max_count_mut()
    }
    fn max_count_simultaneously(&self) -> &i32 {
        self._glacier_base.max_count_simultaneously()
    }
    fn max_count_simultaneously_mut(&mut self) -> &mut i32 {
        self._glacier_base.max_count_simultaneously_mut()
    }
}

impl super::entity::ReferenceObjectDataTrait for CreatureSpawnEntityData {
    fn blueprint_transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.blueprint_transform()
    }
    fn blueprint_transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.blueprint_transform_mut()
    }
    fn blueprint(&self) -> &Option<LockedTypeObject /* super::entity::Blueprint */> {
        self._glacier_base.blueprint()
    }
    fn blueprint_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::Blueprint */> {
        self._glacier_base.blueprint_mut()
    }
    fn object_variation(&self) -> &Option<LockedTypeObject /* super::entity::ObjectVariation */> {
        self._glacier_base.object_variation()
    }
    fn object_variation_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::ObjectVariation */> {
        self._glacier_base.object_variation_mut()
    }
    fn stream_realm(&self) -> &super::entity::StreamRealm {
        self._glacier_base.stream_realm()
    }
    fn stream_realm_mut(&mut self) -> &mut super::entity::StreamRealm {
        self._glacier_base.stream_realm_mut()
    }
    fn radiosity_type_override(&self) -> &super::core::RadiosityTypeOverride {
        self._glacier_base.radiosity_type_override()
    }
    fn radiosity_type_override_mut(&mut self) -> &mut super::core::RadiosityTypeOverride {
        self._glacier_base.radiosity_type_override_mut()
    }
    fn lightmap_resolution_scale(&self) -> &u32 {
        self._glacier_base.lightmap_resolution_scale()
    }
    fn lightmap_resolution_scale_mut(&mut self) -> &mut u32 {
        self._glacier_base.lightmap_resolution_scale_mut()
    }
    fn lightmap_scale_with_size(&self) -> &bool {
        self._glacier_base.lightmap_scale_with_size()
    }
    fn lightmap_scale_with_size_mut(&mut self) -> &mut bool {
        self._glacier_base.lightmap_scale_with_size_mut()
    }
    fn rendering_overrides(&self) -> &super::core::RenderingOverrides {
        self._glacier_base.rendering_overrides()
    }
    fn rendering_overrides_mut(&mut self) -> &mut super::core::RenderingOverrides {
        self._glacier_base.rendering_overrides_mut()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
    fn excluded_mut(&mut self) -> &mut bool {
        self._glacier_base.excluded_mut()
    }
    fn create_indestructible_entity(&self) -> &bool {
        self._glacier_base.create_indestructible_entity()
    }
    fn create_indestructible_entity_mut(&mut self) -> &mut bool {
        self._glacier_base.create_indestructible_entity_mut()
    }
}

impl super::entity::GameObjectDataTrait for CreatureSpawnEntityData {
}

impl super::core::DataBusPeerTrait for CreatureSpawnEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for CreatureSpawnEntityData {
}

impl super::core::DataContainerTrait for CreatureSpawnEntityData {
}

pub static CREATURESPAWNENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureSpawnEntityData",
    name_hash: 1048880438,
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::dice_commons_shared::BLUEPRINTSPAWNREFERENCEOBJECTDATA_TYPE_INFO),
        super_class_offset: offset_of!(CreatureSpawnEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureSpawnEntityData as Default>::default())),
            create_boxed: || Box::new(<CreatureSpawnEntityData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CREATURESPAWNENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CreatureSpawnEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CREATURESPAWNENTITYDATA_TYPE_INFO
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


pub static CREATURESPAWNENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureSpawnEntityData-Array",
    name_hash: 1559583106,
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CreatureSpawnEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerCreatureLocoMotorEntity {
    pub _glacier_base: CreatureLocoEntity,
}

pub trait ServerCreatureLocoMotorEntityTrait: CreatureLocoEntityTrait {
}

impl ServerCreatureLocoMotorEntityTrait for ServerCreatureLocoMotorEntity {
}

impl CreatureLocoEntityTrait for ServerCreatureLocoMotorEntity {
}

impl super::a_i_tools::LocoEntityTrait for ServerCreatureLocoMotorEntity {
}

impl super::entity::EntityTrait for ServerCreatureLocoMotorEntity {
}

impl super::entity::EntityBusPeerTrait for ServerCreatureLocoMotorEntity {
}

pub static SERVERCREATURELOCOMOTORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreatureLocoMotorEntity",
    name_hash: 4168452444,
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATURELOCOENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerCreatureLocoMotorEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCreatureLocoMotorEntity as Default>::default())),
            create_boxed: || Box::new(<ServerCreatureLocoMotorEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCREATURELOCOMOTORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCreatureLocoMotorEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCREATURELOCOMOTORENTITY_TYPE_INFO
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


pub static SERVERCREATURELOCOMOTORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreatureLocoMotorEntity-Array",
    name_hash: 2247881192,
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("ServerCreatureLocoMotorEntity"),
    array_type: None,
    alignment: 8,
};


