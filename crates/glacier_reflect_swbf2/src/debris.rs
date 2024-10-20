use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_debris_types(registry: &mut TypeRegistry) {
    registry.register_type(DEBRISCOLLISIONCOMPONENTDATA_TYPE_INFO);
    registry.register_type(DEBRISCOLLISIONCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(PROCEDURALDESTRUCTIONFORCEDATA_TYPE_INFO);
    registry.register_type(PROCEDURALDESTRUCTIONFORCEDATA_ARRAY_TYPE_INFO);
    registry.register_type(DEBRISCLUSTERDATA_TYPE_INFO);
    registry.register_type(DEBRISCLUSTERDATA_ARRAY_TYPE_INFO);
    registry.register_type(DEBRISCLUSTERPARTINFODATA_TYPE_INFO);
    registry.register_type(DEBRISCLUSTERPARTINFODATA_ARRAY_TYPE_INFO);
    registry.register_type(DEBRISSYSTEMSETTINGS_TYPE_INFO);
    registry.register_type(DEBRISSYSTEMSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(DEBRISSYSTEMASSET_TYPE_INFO);
    registry.register_type(DEBRISSYSTEMASSET_ARRAY_TYPE_INFO);
    registry.register_type(DEBRISHAVOKINFO_TYPE_INFO);
    registry.register_type(DEBRISHAVOKINFO_ARRAY_TYPE_INFO);
    registry.register_type(DEBRISSYSTEMMETRICS_TYPE_INFO);
    registry.register_type(DEBRISSYSTEMMETRICS_ARRAY_TYPE_INFO);
    registry.register_type(SERVERDEBRISCLUSTER_TYPE_INFO);
    registry.register_type(SERVERDEBRISCLUSTER_ARRAY_TYPE_INFO);
    registry.register_type(DEBRISSPAWNEVENT_TYPE_INFO);
    registry.register_type(DEBRISSPAWNEVENT_ARRAY_TYPE_INFO);
    registry.register_type(DEBRISCLUSTER_TYPE_INFO);
    registry.register_type(DEBRISCLUSTER_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTDEBRISCLUSTER_TYPE_INFO);
    registry.register_type(CLIENTDEBRISCLUSTER_ARRAY_TYPE_INFO);
}

#[derive(Clone, Debug, Default)]
pub struct DebrisCollisionComponentData {
    pub _glacier_base: super::entity::ComponentData,
}

pub trait DebrisCollisionComponentDataTrait: super::entity::ComponentDataTrait {
}

impl DebrisCollisionComponentDataTrait for DebrisCollisionComponentData {
}

impl super::entity::ComponentDataTrait for DebrisCollisionComponentData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn components(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components()
    }
    fn client_index(&self) -> &u8 {
        self._glacier_base.client_index()
    }
    fn server_index(&self) -> &u8 {
        self._glacier_base.server_index()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
}

impl super::entity::GameObjectDataTrait for DebrisCollisionComponentData {
}

impl super::core::DataBusPeerTrait for DebrisCollisionComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for DebrisCollisionComponentData {
}

impl super::core::DataContainerTrait for DebrisCollisionComponentData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static DEBRISCOLLISIONCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisCollisionComponentData",
    flags: MemberInfoFlags::new(101),
    module: "Debris",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::COMPONENTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DebrisCollisionComponentData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DEBRISCOLLISIONCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DebrisCollisionComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        DEBRISCOLLISIONCOMPONENTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DEBRISCOLLISIONCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisCollisionComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Debris",
    data: TypeInfoData::Array("DebrisCollisionComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ProceduralDestructionForceData {
    pub _glacier_base: super::entity::PhysicsPropertyRelationPropertyData,
    pub force: f32,
    pub min_force: f32,
    pub max_force: f32,
    pub layered_impact_delay: f32,
    pub layered_collision_delay: f32,
}

pub trait ProceduralDestructionForceDataTrait: super::entity::PhysicsPropertyRelationPropertyDataTrait {
    fn force(&self) -> &f32;
    fn min_force(&self) -> &f32;
    fn max_force(&self) -> &f32;
    fn layered_impact_delay(&self) -> &f32;
    fn layered_collision_delay(&self) -> &f32;
}

impl ProceduralDestructionForceDataTrait for ProceduralDestructionForceData {
    fn force(&self) -> &f32 {
        &self.force
    }
    fn min_force(&self) -> &f32 {
        &self.min_force
    }
    fn max_force(&self) -> &f32 {
        &self.max_force
    }
    fn layered_impact_delay(&self) -> &f32 {
        &self.layered_impact_delay
    }
    fn layered_collision_delay(&self) -> &f32 {
        &self.layered_collision_delay
    }
}

impl super::entity::PhysicsPropertyRelationPropertyDataTrait for ProceduralDestructionForceData {
}

impl super::entity::MaterialRelationPropertyDataTrait for ProceduralDestructionForceData {
}

impl super::core::DataContainerTrait for ProceduralDestructionForceData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PROCEDURALDESTRUCTIONFORCEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProceduralDestructionForceData",
    flags: MemberInfoFlags::new(101),
    module: "Debris",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::PHYSICSPROPERTYRELATIONPROPERTYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ProceduralDestructionForceData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Force",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ProceduralDestructionForceData, force),
            },
            FieldInfoData {
                name: "MinForce",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ProceduralDestructionForceData, min_force),
            },
            FieldInfoData {
                name: "MaxForce",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ProceduralDestructionForceData, max_force),
            },
            FieldInfoData {
                name: "LayeredImpactDelay",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ProceduralDestructionForceData, layered_impact_delay),
            },
            FieldInfoData {
                name: "LayeredCollisionDelay",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ProceduralDestructionForceData, layered_collision_delay),
            },
        ],
    }),
    array_type: Some(PROCEDURALDESTRUCTIONFORCEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProceduralDestructionForceData {
    fn type_info(&self) -> &'static TypeInfo {
        PROCEDURALDESTRUCTIONFORCEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PROCEDURALDESTRUCTIONFORCEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProceduralDestructionForceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Debris",
    data: TypeInfoData::Array("ProceduralDestructionForceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DebrisClusterData {
    pub _glacier_base: super::entity::GameComponentEntityData,
    pub max_active_parts_count: u32,
    pub cull_distance_scale: f32,
    pub light_sampling_offset: f32,
    pub height_limit: f32,
    pub runtime_cluster_lifetime: f32,
    pub mesh: Option<Arc<Mutex<dyn super::render_base::MeshBaseAssetTrait>>>,
    pub composite_part_count: u32,
    pub part_hierarchy: Vec<DebrisClusterPartInfoData>,
    pub rigid_bodies: Vec<Option<Arc<Mutex<dyn super::physics::RigidBodyDataTrait>>>>,
    pub partial_destruction: bool,
    pub client_side_only: bool,
    pub activation_push_force_mul: f32,
    pub push_velocity_mul: super::core::Vec3,
    pub push_velocity_rnd_mul: super::core::Vec3,
    pub init_rotation_rnd_mul: super::core::Vec3,
    pub projectile_force_transfer_mul: f32,
    pub activate_on_spawn: bool,
    pub in_effect_world_only: bool,
    pub no_collision: bool,
    pub activation_effect: Option<Arc<Mutex<dyn super::effect_base::EffectBlueprintTrait>>>,
    pub is_activation_effect_dynamic: bool,
    pub deactivate_parts_on_sleep: bool,
    pub on_part_collision_enable: bool,
    pub on_part_collision_speed_threshold: f32,
    pub kill_parts_on_collision: bool,
    pub effect: Option<Arc<Mutex<dyn super::effect_base::EffectBlueprintTrait>>>,
    pub explosion: Option<Arc<Mutex<dyn super::entity::SpatialEntityDataTrait>>>,
    pub spawn_explosion_on_first_impact_only: bool,
}

pub trait DebrisClusterDataTrait: super::entity::GameComponentEntityDataTrait {
    fn max_active_parts_count(&self) -> &u32;
    fn cull_distance_scale(&self) -> &f32;
    fn light_sampling_offset(&self) -> &f32;
    fn height_limit(&self) -> &f32;
    fn runtime_cluster_lifetime(&self) -> &f32;
    fn mesh(&self) -> &Option<Arc<Mutex<dyn super::render_base::MeshBaseAssetTrait>>>;
    fn composite_part_count(&self) -> &u32;
    fn part_hierarchy(&self) -> &Vec<DebrisClusterPartInfoData>;
    fn rigid_bodies(&self) -> &Vec<Option<Arc<Mutex<dyn super::physics::RigidBodyDataTrait>>>>;
    fn partial_destruction(&self) -> &bool;
    fn client_side_only(&self) -> &bool;
    fn activation_push_force_mul(&self) -> &f32;
    fn push_velocity_mul(&self) -> &super::core::Vec3;
    fn push_velocity_rnd_mul(&self) -> &super::core::Vec3;
    fn init_rotation_rnd_mul(&self) -> &super::core::Vec3;
    fn projectile_force_transfer_mul(&self) -> &f32;
    fn activate_on_spawn(&self) -> &bool;
    fn in_effect_world_only(&self) -> &bool;
    fn no_collision(&self) -> &bool;
    fn activation_effect(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectBlueprintTrait>>>;
    fn is_activation_effect_dynamic(&self) -> &bool;
    fn deactivate_parts_on_sleep(&self) -> &bool;
    fn on_part_collision_enable(&self) -> &bool;
    fn on_part_collision_speed_threshold(&self) -> &f32;
    fn kill_parts_on_collision(&self) -> &bool;
    fn effect(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectBlueprintTrait>>>;
    fn explosion(&self) -> &Option<Arc<Mutex<dyn super::entity::SpatialEntityDataTrait>>>;
    fn spawn_explosion_on_first_impact_only(&self) -> &bool;
}

impl DebrisClusterDataTrait for DebrisClusterData {
    fn max_active_parts_count(&self) -> &u32 {
        &self.max_active_parts_count
    }
    fn cull_distance_scale(&self) -> &f32 {
        &self.cull_distance_scale
    }
    fn light_sampling_offset(&self) -> &f32 {
        &self.light_sampling_offset
    }
    fn height_limit(&self) -> &f32 {
        &self.height_limit
    }
    fn runtime_cluster_lifetime(&self) -> &f32 {
        &self.runtime_cluster_lifetime
    }
    fn mesh(&self) -> &Option<Arc<Mutex<dyn super::render_base::MeshBaseAssetTrait>>> {
        &self.mesh
    }
    fn composite_part_count(&self) -> &u32 {
        &self.composite_part_count
    }
    fn part_hierarchy(&self) -> &Vec<DebrisClusterPartInfoData> {
        &self.part_hierarchy
    }
    fn rigid_bodies(&self) -> &Vec<Option<Arc<Mutex<dyn super::physics::RigidBodyDataTrait>>>> {
        &self.rigid_bodies
    }
    fn partial_destruction(&self) -> &bool {
        &self.partial_destruction
    }
    fn client_side_only(&self) -> &bool {
        &self.client_side_only
    }
    fn activation_push_force_mul(&self) -> &f32 {
        &self.activation_push_force_mul
    }
    fn push_velocity_mul(&self) -> &super::core::Vec3 {
        &self.push_velocity_mul
    }
    fn push_velocity_rnd_mul(&self) -> &super::core::Vec3 {
        &self.push_velocity_rnd_mul
    }
    fn init_rotation_rnd_mul(&self) -> &super::core::Vec3 {
        &self.init_rotation_rnd_mul
    }
    fn projectile_force_transfer_mul(&self) -> &f32 {
        &self.projectile_force_transfer_mul
    }
    fn activate_on_spawn(&self) -> &bool {
        &self.activate_on_spawn
    }
    fn in_effect_world_only(&self) -> &bool {
        &self.in_effect_world_only
    }
    fn no_collision(&self) -> &bool {
        &self.no_collision
    }
    fn activation_effect(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectBlueprintTrait>>> {
        &self.activation_effect
    }
    fn is_activation_effect_dynamic(&self) -> &bool {
        &self.is_activation_effect_dynamic
    }
    fn deactivate_parts_on_sleep(&self) -> &bool {
        &self.deactivate_parts_on_sleep
    }
    fn on_part_collision_enable(&self) -> &bool {
        &self.on_part_collision_enable
    }
    fn on_part_collision_speed_threshold(&self) -> &f32 {
        &self.on_part_collision_speed_threshold
    }
    fn kill_parts_on_collision(&self) -> &bool {
        &self.kill_parts_on_collision
    }
    fn effect(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectBlueprintTrait>>> {
        &self.effect
    }
    fn explosion(&self) -> &Option<Arc<Mutex<dyn super::entity::SpatialEntityDataTrait>>> {
        &self.explosion
    }
    fn spawn_explosion_on_first_impact_only(&self) -> &bool {
        &self.spawn_explosion_on_first_impact_only
    }
}

impl super::entity::GameComponentEntityDataTrait for DebrisClusterData {
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
}

impl super::entity::ComponentEntityDataTrait for DebrisClusterData {
    fn components(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components()
    }
    fn part_bounding_boxes(&self) -> &Vec<super::core::AxisAlignedBox> {
        self._glacier_base.part_bounding_boxes()
    }
    fn client_runtime_component_count(&self) -> &u8 {
        self._glacier_base.client_runtime_component_count()
    }
    fn server_runtime_component_count(&self) -> &u8 {
        self._glacier_base.server_runtime_component_count()
    }
    fn client_runtime_transformation_count(&self) -> &u8 {
        self._glacier_base.client_runtime_transformation_count()
    }
    fn server_runtime_transformation_count(&self) -> &u8 {
        self._glacier_base.server_runtime_transformation_count()
    }
}

impl super::entity::SpatialEntityDataTrait for DebrisClusterData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
}

impl super::entity::EntityDataTrait for DebrisClusterData {
}

impl super::entity::GameObjectDataTrait for DebrisClusterData {
}

impl super::core::DataBusPeerTrait for DebrisClusterData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for DebrisClusterData {
}

impl super::core::DataContainerTrait for DebrisClusterData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static DEBRISCLUSTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisClusterData",
    flags: MemberInfoFlags::new(101),
    module: "Debris",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::GAMECOMPONENTENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DebrisClusterData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MaxActivePartsCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DebrisClusterData, max_active_parts_count),
            },
            FieldInfoData {
                name: "CullDistanceScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebrisClusterData, cull_distance_scale),
            },
            FieldInfoData {
                name: "LightSamplingOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebrisClusterData, light_sampling_offset),
            },
            FieldInfoData {
                name: "HeightLimit",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebrisClusterData, height_limit),
            },
            FieldInfoData {
                name: "RuntimeClusterLifetime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebrisClusterData, runtime_cluster_lifetime),
            },
            FieldInfoData {
                name: "Mesh",
                flags: MemberInfoFlags::new(0),
                field_type: "MeshBaseAsset",
                rust_offset: offset_of!(DebrisClusterData, mesh),
            },
            FieldInfoData {
                name: "CompositePartCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DebrisClusterData, composite_part_count),
            },
            FieldInfoData {
                name: "PartHierarchy",
                flags: MemberInfoFlags::new(144),
                field_type: "DebrisClusterPartInfoData-Array",
                rust_offset: offset_of!(DebrisClusterData, part_hierarchy),
            },
            FieldInfoData {
                name: "RigidBodies",
                flags: MemberInfoFlags::new(144),
                field_type: "RigidBodyData-Array",
                rust_offset: offset_of!(DebrisClusterData, rigid_bodies),
            },
            FieldInfoData {
                name: "PartialDestruction",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisClusterData, partial_destruction),
            },
            FieldInfoData {
                name: "ClientSideOnly",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisClusterData, client_side_only),
            },
            FieldInfoData {
                name: "ActivationPushForceMul",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebrisClusterData, activation_push_force_mul),
            },
            FieldInfoData {
                name: "PushVelocityMul",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(DebrisClusterData, push_velocity_mul),
            },
            FieldInfoData {
                name: "PushVelocityRndMul",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(DebrisClusterData, push_velocity_rnd_mul),
            },
            FieldInfoData {
                name: "InitRotationRndMul",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(DebrisClusterData, init_rotation_rnd_mul),
            },
            FieldInfoData {
                name: "ProjectileForceTransferMul",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebrisClusterData, projectile_force_transfer_mul),
            },
            FieldInfoData {
                name: "ActivateOnSpawn",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisClusterData, activate_on_spawn),
            },
            FieldInfoData {
                name: "InEffectWorldOnly",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisClusterData, in_effect_world_only),
            },
            FieldInfoData {
                name: "NoCollision",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisClusterData, no_collision),
            },
            FieldInfoData {
                name: "ActivationEffect",
                flags: MemberInfoFlags::new(0),
                field_type: "EffectBlueprint",
                rust_offset: offset_of!(DebrisClusterData, activation_effect),
            },
            FieldInfoData {
                name: "IsActivationEffectDynamic",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisClusterData, is_activation_effect_dynamic),
            },
            FieldInfoData {
                name: "DeactivatePartsOnSleep",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisClusterData, deactivate_parts_on_sleep),
            },
            FieldInfoData {
                name: "OnPartCollisionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisClusterData, on_part_collision_enable),
            },
            FieldInfoData {
                name: "OnPartCollisionSpeedThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebrisClusterData, on_part_collision_speed_threshold),
            },
            FieldInfoData {
                name: "KillPartsOnCollision",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisClusterData, kill_parts_on_collision),
            },
            FieldInfoData {
                name: "Effect",
                flags: MemberInfoFlags::new(0),
                field_type: "EffectBlueprint",
                rust_offset: offset_of!(DebrisClusterData, effect),
            },
            FieldInfoData {
                name: "Explosion",
                flags: MemberInfoFlags::new(0),
                field_type: "SpatialEntityData",
                rust_offset: offset_of!(DebrisClusterData, explosion),
            },
            FieldInfoData {
                name: "SpawnExplosionOnFirstImpactOnly",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisClusterData, spawn_explosion_on_first_impact_only),
            },
        ],
    }),
    array_type: Some(DEBRISCLUSTERDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DebrisClusterData {
    fn type_info(&self) -> &'static TypeInfo {
        DEBRISCLUSTERDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DEBRISCLUSTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisClusterData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Debris",
    data: TypeInfoData::Array("DebrisClusterData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DebrisClusterPartInfoData {
    pub part_index: i32,
    pub number_of_children: i32,
    pub in_effect_world_only: bool,
    pub affected_by_collision: bool,
    pub split_speed_threshold: f32,
    pub delay: f32,
    pub force: f32,
    pub min_force: f32,
    pub max_force: f32,
    pub layered_impact_delay: f32,
    pub layered_collision_delay: f32,
    pub linear_velocity: super::core::Vec3,
    pub angular_velocity: super::core::Vec3,
    pub sync_rest_position: bool,
    pub sync_continous: bool,
}

pub trait DebrisClusterPartInfoDataTrait: TypeObject {
    fn part_index(&self) -> &i32;
    fn number_of_children(&self) -> &i32;
    fn in_effect_world_only(&self) -> &bool;
    fn affected_by_collision(&self) -> &bool;
    fn split_speed_threshold(&self) -> &f32;
    fn delay(&self) -> &f32;
    fn force(&self) -> &f32;
    fn min_force(&self) -> &f32;
    fn max_force(&self) -> &f32;
    fn layered_impact_delay(&self) -> &f32;
    fn layered_collision_delay(&self) -> &f32;
    fn linear_velocity(&self) -> &super::core::Vec3;
    fn angular_velocity(&self) -> &super::core::Vec3;
    fn sync_rest_position(&self) -> &bool;
    fn sync_continous(&self) -> &bool;
}

impl DebrisClusterPartInfoDataTrait for DebrisClusterPartInfoData {
    fn part_index(&self) -> &i32 {
        &self.part_index
    }
    fn number_of_children(&self) -> &i32 {
        &self.number_of_children
    }
    fn in_effect_world_only(&self) -> &bool {
        &self.in_effect_world_only
    }
    fn affected_by_collision(&self) -> &bool {
        &self.affected_by_collision
    }
    fn split_speed_threshold(&self) -> &f32 {
        &self.split_speed_threshold
    }
    fn delay(&self) -> &f32 {
        &self.delay
    }
    fn force(&self) -> &f32 {
        &self.force
    }
    fn min_force(&self) -> &f32 {
        &self.min_force
    }
    fn max_force(&self) -> &f32 {
        &self.max_force
    }
    fn layered_impact_delay(&self) -> &f32 {
        &self.layered_impact_delay
    }
    fn layered_collision_delay(&self) -> &f32 {
        &self.layered_collision_delay
    }
    fn linear_velocity(&self) -> &super::core::Vec3 {
        &self.linear_velocity
    }
    fn angular_velocity(&self) -> &super::core::Vec3 {
        &self.angular_velocity
    }
    fn sync_rest_position(&self) -> &bool {
        &self.sync_rest_position
    }
    fn sync_continous(&self) -> &bool {
        &self.sync_continous
    }
}

pub static DEBRISCLUSTERPARTINFODATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisClusterPartInfoData",
    flags: MemberInfoFlags::new(36937),
    module: "Debris",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DebrisClusterPartInfoData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "PartIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(DebrisClusterPartInfoData, part_index),
            },
            FieldInfoData {
                name: "NumberOfChildren",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(DebrisClusterPartInfoData, number_of_children),
            },
            FieldInfoData {
                name: "InEffectWorldOnly",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisClusterPartInfoData, in_effect_world_only),
            },
            FieldInfoData {
                name: "AffectedByCollision",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisClusterPartInfoData, affected_by_collision),
            },
            FieldInfoData {
                name: "SplitSpeedThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebrisClusterPartInfoData, split_speed_threshold),
            },
            FieldInfoData {
                name: "Delay",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebrisClusterPartInfoData, delay),
            },
            FieldInfoData {
                name: "Force",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebrisClusterPartInfoData, force),
            },
            FieldInfoData {
                name: "MinForce",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebrisClusterPartInfoData, min_force),
            },
            FieldInfoData {
                name: "MaxForce",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebrisClusterPartInfoData, max_force),
            },
            FieldInfoData {
                name: "LayeredImpactDelay",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebrisClusterPartInfoData, layered_impact_delay),
            },
            FieldInfoData {
                name: "LayeredCollisionDelay",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebrisClusterPartInfoData, layered_collision_delay),
            },
            FieldInfoData {
                name: "LinearVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(DebrisClusterPartInfoData, linear_velocity),
            },
            FieldInfoData {
                name: "AngularVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(DebrisClusterPartInfoData, angular_velocity),
            },
            FieldInfoData {
                name: "SyncRestPosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisClusterPartInfoData, sync_rest_position),
            },
            FieldInfoData {
                name: "SyncContinous",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisClusterPartInfoData, sync_continous),
            },
        ],
    }),
    array_type: Some(DEBRISCLUSTERPARTINFODATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DebrisClusterPartInfoData {
    fn type_info(&self) -> &'static TypeInfo {
        DEBRISCLUSTERPARTINFODATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DEBRISCLUSTERPARTINFODATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisClusterPartInfoData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Debris",
    data: TypeInfoData::Array("DebrisClusterPartInfoData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DebrisSystemSettings {
    pub _glacier_base: super::core::DataContainer,
    pub enable: bool,
    pub time_scale: f32,
    pub enable_jobs: bool,
    pub draw_stats: u32,
    pub draw_enable: bool,
    pub mesh_havok_rendering_enable: bool,
    pub mesh_draw_transforms: bool,
    pub mesh_draw_bounding_boxes: bool,
    pub mesh_shadow_enable: bool,
    pub mesh_view_culling_enable: bool,
    pub mesh_culling_distance: f32,
    pub mesh_batch_count_limit: u32,
    pub debris_pool_size: u32,
    pub mesh_draw_count_limit: u32,
    pub mesh_streaming_priority_multiplier: f32,
    pub mesh_draw_cull_stats: bool,
    pub client_max_debris_instances: u32,
    pub debris_quality_level: super::core::QualityLevel,
}

pub trait DebrisSystemSettingsTrait: super::core::DataContainerTrait {
    fn enable(&self) -> &bool;
    fn time_scale(&self) -> &f32;
    fn enable_jobs(&self) -> &bool;
    fn draw_stats(&self) -> &u32;
    fn draw_enable(&self) -> &bool;
    fn mesh_havok_rendering_enable(&self) -> &bool;
    fn mesh_draw_transforms(&self) -> &bool;
    fn mesh_draw_bounding_boxes(&self) -> &bool;
    fn mesh_shadow_enable(&self) -> &bool;
    fn mesh_view_culling_enable(&self) -> &bool;
    fn mesh_culling_distance(&self) -> &f32;
    fn mesh_batch_count_limit(&self) -> &u32;
    fn debris_pool_size(&self) -> &u32;
    fn mesh_draw_count_limit(&self) -> &u32;
    fn mesh_streaming_priority_multiplier(&self) -> &f32;
    fn mesh_draw_cull_stats(&self) -> &bool;
    fn client_max_debris_instances(&self) -> &u32;
    fn debris_quality_level(&self) -> &super::core::QualityLevel;
}

impl DebrisSystemSettingsTrait for DebrisSystemSettings {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn time_scale(&self) -> &f32 {
        &self.time_scale
    }
    fn enable_jobs(&self) -> &bool {
        &self.enable_jobs
    }
    fn draw_stats(&self) -> &u32 {
        &self.draw_stats
    }
    fn draw_enable(&self) -> &bool {
        &self.draw_enable
    }
    fn mesh_havok_rendering_enable(&self) -> &bool {
        &self.mesh_havok_rendering_enable
    }
    fn mesh_draw_transforms(&self) -> &bool {
        &self.mesh_draw_transforms
    }
    fn mesh_draw_bounding_boxes(&self) -> &bool {
        &self.mesh_draw_bounding_boxes
    }
    fn mesh_shadow_enable(&self) -> &bool {
        &self.mesh_shadow_enable
    }
    fn mesh_view_culling_enable(&self) -> &bool {
        &self.mesh_view_culling_enable
    }
    fn mesh_culling_distance(&self) -> &f32 {
        &self.mesh_culling_distance
    }
    fn mesh_batch_count_limit(&self) -> &u32 {
        &self.mesh_batch_count_limit
    }
    fn debris_pool_size(&self) -> &u32 {
        &self.debris_pool_size
    }
    fn mesh_draw_count_limit(&self) -> &u32 {
        &self.mesh_draw_count_limit
    }
    fn mesh_streaming_priority_multiplier(&self) -> &f32 {
        &self.mesh_streaming_priority_multiplier
    }
    fn mesh_draw_cull_stats(&self) -> &bool {
        &self.mesh_draw_cull_stats
    }
    fn client_max_debris_instances(&self) -> &u32 {
        &self.client_max_debris_instances
    }
    fn debris_quality_level(&self) -> &super::core::QualityLevel {
        &self.debris_quality_level
    }
}

impl super::core::DataContainerTrait for DebrisSystemSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static DEBRISSYSTEMSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisSystemSettings",
    flags: MemberInfoFlags::new(101),
    module: "Debris",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DebrisSystemSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisSystemSettings, enable),
            },
            FieldInfoData {
                name: "TimeScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebrisSystemSettings, time_scale),
            },
            FieldInfoData {
                name: "EnableJobs",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisSystemSettings, enable_jobs),
            },
            FieldInfoData {
                name: "DrawStats",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DebrisSystemSettings, draw_stats),
            },
            FieldInfoData {
                name: "DrawEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisSystemSettings, draw_enable),
            },
            FieldInfoData {
                name: "MeshHavokRenderingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisSystemSettings, mesh_havok_rendering_enable),
            },
            FieldInfoData {
                name: "MeshDrawTransforms",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisSystemSettings, mesh_draw_transforms),
            },
            FieldInfoData {
                name: "MeshDrawBoundingBoxes",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisSystemSettings, mesh_draw_bounding_boxes),
            },
            FieldInfoData {
                name: "MeshShadowEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisSystemSettings, mesh_shadow_enable),
            },
            FieldInfoData {
                name: "MeshViewCullingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisSystemSettings, mesh_view_culling_enable),
            },
            FieldInfoData {
                name: "MeshCullingDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebrisSystemSettings, mesh_culling_distance),
            },
            FieldInfoData {
                name: "MeshBatchCountLimit",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DebrisSystemSettings, mesh_batch_count_limit),
            },
            FieldInfoData {
                name: "DebrisPoolSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DebrisSystemSettings, debris_pool_size),
            },
            FieldInfoData {
                name: "MeshDrawCountLimit",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DebrisSystemSettings, mesh_draw_count_limit),
            },
            FieldInfoData {
                name: "MeshStreamingPriorityMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebrisSystemSettings, mesh_streaming_priority_multiplier),
            },
            FieldInfoData {
                name: "MeshDrawCullStats",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisSystemSettings, mesh_draw_cull_stats),
            },
            FieldInfoData {
                name: "ClientMaxDebrisInstances",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DebrisSystemSettings, client_max_debris_instances),
            },
            FieldInfoData {
                name: "DebrisQualityLevel",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityLevel",
                rust_offset: offset_of!(DebrisSystemSettings, debris_quality_level),
            },
        ],
    }),
    array_type: Some(DEBRISSYSTEMSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DebrisSystemSettings {
    fn type_info(&self) -> &'static TypeInfo {
        DEBRISSYSTEMSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DEBRISSYSTEMSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisSystemSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Debris",
    data: TypeInfoData::Array("DebrisSystemSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DebrisSystemAsset {
    pub _glacier_base: super::core::Asset,
    pub havok_meshes: Vec<DebrisHavokInfo>,
    pub havok_mesh_count: i32,
}

pub trait DebrisSystemAssetTrait: super::core::AssetTrait {
    fn havok_meshes(&self) -> &Vec<DebrisHavokInfo>;
    fn havok_mesh_count(&self) -> &i32;
}

impl DebrisSystemAssetTrait for DebrisSystemAsset {
    fn havok_meshes(&self) -> &Vec<DebrisHavokInfo> {
        &self.havok_meshes
    }
    fn havok_mesh_count(&self) -> &i32 {
        &self.havok_mesh_count
    }
}

impl super::core::AssetTrait for DebrisSystemAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for DebrisSystemAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static DEBRISSYSTEMASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisSystemAsset",
    flags: MemberInfoFlags::new(101),
    module: "Debris",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DebrisSystemAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "HavokMeshes",
                flags: MemberInfoFlags::new(144),
                field_type: "DebrisHavokInfo-Array",
                rust_offset: offset_of!(DebrisSystemAsset, havok_meshes),
            },
            FieldInfoData {
                name: "HavokMeshCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(DebrisSystemAsset, havok_mesh_count),
            },
        ],
    }),
    array_type: Some(DEBRISSYSTEMASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DebrisSystemAsset {
    fn type_info(&self) -> &'static TypeInfo {
        DEBRISSYSTEMASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DEBRISSYSTEMASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisSystemAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Debris",
    data: TypeInfoData::Array("DebrisSystemAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DebrisHavokInfo {
    pub havok_asset: Option<Arc<Mutex<dyn super::physics::HavokAssetTrait>>>,
    pub reserve_count: i32,
}

pub trait DebrisHavokInfoTrait: TypeObject {
    fn havok_asset(&self) -> &Option<Arc<Mutex<dyn super::physics::HavokAssetTrait>>>;
    fn reserve_count(&self) -> &i32;
}

impl DebrisHavokInfoTrait for DebrisHavokInfo {
    fn havok_asset(&self) -> &Option<Arc<Mutex<dyn super::physics::HavokAssetTrait>>> {
        &self.havok_asset
    }
    fn reserve_count(&self) -> &i32 {
        &self.reserve_count
    }
}

pub static DEBRISHAVOKINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisHavokInfo",
    flags: MemberInfoFlags::new(73),
    module: "Debris",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DebrisHavokInfo as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "HavokAsset",
                flags: MemberInfoFlags::new(0),
                field_type: "HavokAsset",
                rust_offset: offset_of!(DebrisHavokInfo, havok_asset),
            },
            FieldInfoData {
                name: "ReserveCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(DebrisHavokInfo, reserve_count),
            },
        ],
    }),
    array_type: Some(DEBRISHAVOKINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DebrisHavokInfo {
    fn type_info(&self) -> &'static TypeInfo {
        DEBRISHAVOKINFO_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DEBRISHAVOKINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisHavokInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Debris",
    data: TypeInfoData::Array("DebrisHavokInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DebrisSystemMetrics {
    pub havok_particle_count: i32,
    pub havok_particle_part_count: i32,
}

pub trait DebrisSystemMetricsTrait: TypeObject {
    fn havok_particle_count(&self) -> &i32;
    fn havok_particle_part_count(&self) -> &i32;
}

impl DebrisSystemMetricsTrait for DebrisSystemMetrics {
    fn havok_particle_count(&self) -> &i32 {
        &self.havok_particle_count
    }
    fn havok_particle_part_count(&self) -> &i32 {
        &self.havok_particle_part_count
    }
}

pub static DEBRISSYSTEMMETRICS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisSystemMetrics",
    flags: MemberInfoFlags::new(36937),
    module: "Debris",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DebrisSystemMetrics as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "HavokParticleCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(DebrisSystemMetrics, havok_particle_count),
            },
            FieldInfoData {
                name: "HavokParticlePartCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(DebrisSystemMetrics, havok_particle_part_count),
            },
        ],
    }),
    array_type: Some(DEBRISSYSTEMMETRICS_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for DebrisSystemMetrics {
    fn type_info(&self) -> &'static TypeInfo {
        DEBRISSYSTEMMETRICS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DEBRISSYSTEMMETRICS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisSystemMetrics-Array",
    flags: MemberInfoFlags::new(145),
    module: "Debris",
    data: TypeInfoData::Array("DebrisSystemMetrics"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerDebrisCluster {
    pub _glacier_base: DebrisCluster,
}

pub trait ServerDebrisClusterTrait: DebrisClusterTrait {
}

impl ServerDebrisClusterTrait for ServerDebrisCluster {
}

impl DebrisClusterTrait for ServerDebrisCluster {
}

impl super::entity::ComponentEntityTrait for ServerDebrisCluster {
}

impl super::entity::SpatialEntityTrait for ServerDebrisCluster {
}

impl super::entity::EntityTrait for ServerDebrisCluster {
}

impl super::entity::EntityBusPeerTrait for ServerDebrisCluster {
}

pub static SERVERDEBRISCLUSTER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDebrisCluster",
    flags: MemberInfoFlags::new(101),
    module: "Debris",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DEBRISCLUSTER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerDebrisCluster as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERDEBRISCLUSTER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerDebrisCluster {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERDEBRISCLUSTER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERDEBRISCLUSTER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDebrisCluster-Array",
    flags: MemberInfoFlags::new(145),
    module: "Debris",
    data: TypeInfoData::Array("ServerDebrisCluster"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DebrisSpawnEvent {
    pub _glacier_base: super::entity::EntityEvent,
}

pub trait DebrisSpawnEventTrait: super::entity::EntityEventTrait {
}

impl DebrisSpawnEventTrait for DebrisSpawnEvent {
}

impl super::entity::EntityEventTrait for DebrisSpawnEvent {
}

pub static DEBRISSPAWNEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisSpawnEvent",
    flags: MemberInfoFlags::new(101),
    module: "Debris",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DebrisSpawnEvent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DEBRISSPAWNEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DebrisSpawnEvent {
    fn type_info(&self) -> &'static TypeInfo {
        DEBRISSPAWNEVENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DEBRISSPAWNEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisSpawnEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Debris",
    data: TypeInfoData::Array("DebrisSpawnEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DebrisCluster {
    pub _glacier_base: super::entity::ComponentEntity,
}

pub trait DebrisClusterTrait: super::entity::ComponentEntityTrait {
}

impl DebrisClusterTrait for DebrisCluster {
}

impl super::entity::ComponentEntityTrait for DebrisCluster {
}

impl super::entity::SpatialEntityTrait for DebrisCluster {
}

impl super::entity::EntityTrait for DebrisCluster {
}

impl super::entity::EntityBusPeerTrait for DebrisCluster {
}

pub static DEBRISCLUSTER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisCluster",
    flags: MemberInfoFlags::new(101),
    module: "Debris",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::COMPONENTENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DebrisCluster as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DEBRISCLUSTER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DebrisCluster {
    fn type_info(&self) -> &'static TypeInfo {
        DEBRISCLUSTER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DEBRISCLUSTER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisCluster-Array",
    flags: MemberInfoFlags::new(145),
    module: "Debris",
    data: TypeInfoData::Array("DebrisCluster"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientDebrisCluster {
    pub _glacier_base: DebrisCluster,
}

pub trait ClientDebrisClusterTrait: DebrisClusterTrait {
}

impl ClientDebrisClusterTrait for ClientDebrisCluster {
}

impl DebrisClusterTrait for ClientDebrisCluster {
}

impl super::entity::ComponentEntityTrait for ClientDebrisCluster {
}

impl super::entity::SpatialEntityTrait for ClientDebrisCluster {
}

impl super::entity::EntityTrait for ClientDebrisCluster {
}

impl super::entity::EntityBusPeerTrait for ClientDebrisCluster {
}

pub static CLIENTDEBRISCLUSTER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDebrisCluster",
    flags: MemberInfoFlags::new(101),
    module: "Debris",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DEBRISCLUSTER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientDebrisCluster as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTDEBRISCLUSTER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientDebrisCluster {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTDEBRISCLUSTER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTDEBRISCLUSTER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDebrisCluster-Array",
    flags: MemberInfoFlags::new(145),
    module: "Debris",
    data: TypeInfoData::Array("ClientDebrisCluster"),
    array_type: None,
    alignment: 8,
};


