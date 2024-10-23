use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
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

#[derive(Debug, Default)]
#[repr(C)]
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
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
    fn components(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components_mut()
    }
    fn client_index(&self) -> &u8 {
        self._glacier_base.client_index()
    }
    fn client_index_mut(&mut self) -> &mut u8 {
        self._glacier_base.client_index_mut()
    }
    fn server_index(&self) -> &u8 {
        self._glacier_base.server_index()
    }
    fn server_index_mut(&mut self) -> &mut u8 {
        self._glacier_base.server_index_mut()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
    fn excluded_mut(&mut self) -> &mut bool {
        self._glacier_base.excluded_mut()
    }
}

impl super::entity::GameObjectDataTrait for DebrisCollisionComponentData {
}

impl super::core::DataBusPeerTrait for DebrisCollisionComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for DebrisCollisionComponentData {
}

impl super::core::DataContainerTrait for DebrisCollisionComponentData {
}

pub static DEBRISCOLLISIONCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisCollisionComponentData",
    name_hash: 296877871,
    flags: MemberInfoFlags::new(101),
    module: "Debris",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::COMPONENTDATA_TYPE_INFO),
        super_class_offset: offset_of!(DebrisCollisionComponentData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DebrisCollisionComponentData as Default>::default())),
            create_boxed: || Box::new(<DebrisCollisionComponentData as Default>::default()),
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


pub static DEBRISCOLLISIONCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisCollisionComponentData-Array",
    name_hash: 1593969819,
    flags: MemberInfoFlags::new(145),
    module: "Debris",
    data: TypeInfoData::Array("DebrisCollisionComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn force_mut(&mut self) -> &mut f32;
    fn min_force(&self) -> &f32;
    fn min_force_mut(&mut self) -> &mut f32;
    fn max_force(&self) -> &f32;
    fn max_force_mut(&mut self) -> &mut f32;
    fn layered_impact_delay(&self) -> &f32;
    fn layered_impact_delay_mut(&mut self) -> &mut f32;
    fn layered_collision_delay(&self) -> &f32;
    fn layered_collision_delay_mut(&mut self) -> &mut f32;
}

impl ProceduralDestructionForceDataTrait for ProceduralDestructionForceData {
    fn force(&self) -> &f32 {
        &self.force
    }
    fn force_mut(&mut self) -> &mut f32 {
        &mut self.force
    }
    fn min_force(&self) -> &f32 {
        &self.min_force
    }
    fn min_force_mut(&mut self) -> &mut f32 {
        &mut self.min_force
    }
    fn max_force(&self) -> &f32 {
        &self.max_force
    }
    fn max_force_mut(&mut self) -> &mut f32 {
        &mut self.max_force
    }
    fn layered_impact_delay(&self) -> &f32 {
        &self.layered_impact_delay
    }
    fn layered_impact_delay_mut(&mut self) -> &mut f32 {
        &mut self.layered_impact_delay
    }
    fn layered_collision_delay(&self) -> &f32 {
        &self.layered_collision_delay
    }
    fn layered_collision_delay_mut(&mut self) -> &mut f32 {
        &mut self.layered_collision_delay
    }
}

impl super::entity::PhysicsPropertyRelationPropertyDataTrait for ProceduralDestructionForceData {
}

impl super::entity::MaterialRelationPropertyDataTrait for ProceduralDestructionForceData {
}

impl super::core::DataContainerTrait for ProceduralDestructionForceData {
}

pub static PROCEDURALDESTRUCTIONFORCEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProceduralDestructionForceData",
    name_hash: 2762104371,
    flags: MemberInfoFlags::new(101),
    module: "Debris",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::PHYSICSPROPERTYRELATIONPROPERTYDATA_TYPE_INFO),
        super_class_offset: offset_of!(ProceduralDestructionForceData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ProceduralDestructionForceData as Default>::default())),
            create_boxed: || Box::new(<ProceduralDestructionForceData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Force",
                name_hash: 206910456,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ProceduralDestructionForceData, force),
            },
            FieldInfoData {
                name: "MinForce",
                name_hash: 3352963314,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ProceduralDestructionForceData, min_force),
            },
            FieldInfoData {
                name: "MaxForce",
                name_hash: 427399596,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ProceduralDestructionForceData, max_force),
            },
            FieldInfoData {
                name: "LayeredImpactDelay",
                name_hash: 3016899600,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ProceduralDestructionForceData, layered_impact_delay),
            },
            FieldInfoData {
                name: "LayeredCollisionDelay",
                name_hash: 4035460716,
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


pub static PROCEDURALDESTRUCTIONFORCEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProceduralDestructionForceData-Array",
    name_hash: 2924571015,
    flags: MemberInfoFlags::new(145),
    module: "Debris",
    data: TypeInfoData::Array("ProceduralDestructionForceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DebrisClusterData {
    pub _glacier_base: super::entity::GameComponentEntityData,
    pub max_active_parts_count: u32,
    pub cull_distance_scale: f32,
    pub light_sampling_offset: f32,
    pub height_limit: f32,
    pub runtime_cluster_lifetime: f32,
    pub mesh: Option<LockedTypeObject /* super::render_base::MeshBaseAsset */>,
    pub composite_part_count: u32,
    pub part_hierarchy: Vec<BoxedTypeObject /* DebrisClusterPartInfoData */>,
    pub rigid_bodies: Vec<Option<LockedTypeObject /* super::physics::RigidBodyData */>>,
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
    pub activation_effect: Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>,
    pub is_activation_effect_dynamic: bool,
    pub deactivate_parts_on_sleep: bool,
    pub on_part_collision_enable: bool,
    pub on_part_collision_speed_threshold: f32,
    pub kill_parts_on_collision: bool,
    pub effect: Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>,
    pub explosion: Option<LockedTypeObject /* super::entity::SpatialEntityData */>,
    pub spawn_explosion_on_first_impact_only: bool,
}

pub trait DebrisClusterDataTrait: super::entity::GameComponentEntityDataTrait {
    fn max_active_parts_count(&self) -> &u32;
    fn max_active_parts_count_mut(&mut self) -> &mut u32;
    fn cull_distance_scale(&self) -> &f32;
    fn cull_distance_scale_mut(&mut self) -> &mut f32;
    fn light_sampling_offset(&self) -> &f32;
    fn light_sampling_offset_mut(&mut self) -> &mut f32;
    fn height_limit(&self) -> &f32;
    fn height_limit_mut(&mut self) -> &mut f32;
    fn runtime_cluster_lifetime(&self) -> &f32;
    fn runtime_cluster_lifetime_mut(&mut self) -> &mut f32;
    fn mesh(&self) -> &Option<LockedTypeObject /* super::render_base::MeshBaseAsset */>;
    fn mesh_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::MeshBaseAsset */>;
    fn composite_part_count(&self) -> &u32;
    fn composite_part_count_mut(&mut self) -> &mut u32;
    fn part_hierarchy(&self) -> &Vec<BoxedTypeObject /* DebrisClusterPartInfoData */>;
    fn part_hierarchy_mut(&mut self) -> &mut Vec<BoxedTypeObject /* DebrisClusterPartInfoData */>;
    fn rigid_bodies(&self) -> &Vec<Option<LockedTypeObject /* super::physics::RigidBodyData */>>;
    fn rigid_bodies_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::physics::RigidBodyData */>>;
    fn partial_destruction(&self) -> &bool;
    fn partial_destruction_mut(&mut self) -> &mut bool;
    fn client_side_only(&self) -> &bool;
    fn client_side_only_mut(&mut self) -> &mut bool;
    fn activation_push_force_mul(&self) -> &f32;
    fn activation_push_force_mul_mut(&mut self) -> &mut f32;
    fn push_velocity_mul(&self) -> &super::core::Vec3;
    fn push_velocity_mul_mut(&mut self) -> &mut super::core::Vec3;
    fn push_velocity_rnd_mul(&self) -> &super::core::Vec3;
    fn push_velocity_rnd_mul_mut(&mut self) -> &mut super::core::Vec3;
    fn init_rotation_rnd_mul(&self) -> &super::core::Vec3;
    fn init_rotation_rnd_mul_mut(&mut self) -> &mut super::core::Vec3;
    fn projectile_force_transfer_mul(&self) -> &f32;
    fn projectile_force_transfer_mul_mut(&mut self) -> &mut f32;
    fn activate_on_spawn(&self) -> &bool;
    fn activate_on_spawn_mut(&mut self) -> &mut bool;
    fn in_effect_world_only(&self) -> &bool;
    fn in_effect_world_only_mut(&mut self) -> &mut bool;
    fn no_collision(&self) -> &bool;
    fn no_collision_mut(&mut self) -> &mut bool;
    fn activation_effect(&self) -> &Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>;
    fn activation_effect_mut(&mut self) -> &mut Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>;
    fn is_activation_effect_dynamic(&self) -> &bool;
    fn is_activation_effect_dynamic_mut(&mut self) -> &mut bool;
    fn deactivate_parts_on_sleep(&self) -> &bool;
    fn deactivate_parts_on_sleep_mut(&mut self) -> &mut bool;
    fn on_part_collision_enable(&self) -> &bool;
    fn on_part_collision_enable_mut(&mut self) -> &mut bool;
    fn on_part_collision_speed_threshold(&self) -> &f32;
    fn on_part_collision_speed_threshold_mut(&mut self) -> &mut f32;
    fn kill_parts_on_collision(&self) -> &bool;
    fn kill_parts_on_collision_mut(&mut self) -> &mut bool;
    fn effect(&self) -> &Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>;
    fn effect_mut(&mut self) -> &mut Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>;
    fn explosion(&self) -> &Option<LockedTypeObject /* super::entity::SpatialEntityData */>;
    fn explosion_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::SpatialEntityData */>;
    fn spawn_explosion_on_first_impact_only(&self) -> &bool;
    fn spawn_explosion_on_first_impact_only_mut(&mut self) -> &mut bool;
}

impl DebrisClusterDataTrait for DebrisClusterData {
    fn max_active_parts_count(&self) -> &u32 {
        &self.max_active_parts_count
    }
    fn max_active_parts_count_mut(&mut self) -> &mut u32 {
        &mut self.max_active_parts_count
    }
    fn cull_distance_scale(&self) -> &f32 {
        &self.cull_distance_scale
    }
    fn cull_distance_scale_mut(&mut self) -> &mut f32 {
        &mut self.cull_distance_scale
    }
    fn light_sampling_offset(&self) -> &f32 {
        &self.light_sampling_offset
    }
    fn light_sampling_offset_mut(&mut self) -> &mut f32 {
        &mut self.light_sampling_offset
    }
    fn height_limit(&self) -> &f32 {
        &self.height_limit
    }
    fn height_limit_mut(&mut self) -> &mut f32 {
        &mut self.height_limit
    }
    fn runtime_cluster_lifetime(&self) -> &f32 {
        &self.runtime_cluster_lifetime
    }
    fn runtime_cluster_lifetime_mut(&mut self) -> &mut f32 {
        &mut self.runtime_cluster_lifetime
    }
    fn mesh(&self) -> &Option<LockedTypeObject /* super::render_base::MeshBaseAsset */> {
        &self.mesh
    }
    fn mesh_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::MeshBaseAsset */> {
        &mut self.mesh
    }
    fn composite_part_count(&self) -> &u32 {
        &self.composite_part_count
    }
    fn composite_part_count_mut(&mut self) -> &mut u32 {
        &mut self.composite_part_count
    }
    fn part_hierarchy(&self) -> &Vec<BoxedTypeObject /* DebrisClusterPartInfoData */> {
        &self.part_hierarchy
    }
    fn part_hierarchy_mut(&mut self) -> &mut Vec<BoxedTypeObject /* DebrisClusterPartInfoData */> {
        &mut self.part_hierarchy
    }
    fn rigid_bodies(&self) -> &Vec<Option<LockedTypeObject /* super::physics::RigidBodyData */>> {
        &self.rigid_bodies
    }
    fn rigid_bodies_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::physics::RigidBodyData */>> {
        &mut self.rigid_bodies
    }
    fn partial_destruction(&self) -> &bool {
        &self.partial_destruction
    }
    fn partial_destruction_mut(&mut self) -> &mut bool {
        &mut self.partial_destruction
    }
    fn client_side_only(&self) -> &bool {
        &self.client_side_only
    }
    fn client_side_only_mut(&mut self) -> &mut bool {
        &mut self.client_side_only
    }
    fn activation_push_force_mul(&self) -> &f32 {
        &self.activation_push_force_mul
    }
    fn activation_push_force_mul_mut(&mut self) -> &mut f32 {
        &mut self.activation_push_force_mul
    }
    fn push_velocity_mul(&self) -> &super::core::Vec3 {
        &self.push_velocity_mul
    }
    fn push_velocity_mul_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.push_velocity_mul
    }
    fn push_velocity_rnd_mul(&self) -> &super::core::Vec3 {
        &self.push_velocity_rnd_mul
    }
    fn push_velocity_rnd_mul_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.push_velocity_rnd_mul
    }
    fn init_rotation_rnd_mul(&self) -> &super::core::Vec3 {
        &self.init_rotation_rnd_mul
    }
    fn init_rotation_rnd_mul_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.init_rotation_rnd_mul
    }
    fn projectile_force_transfer_mul(&self) -> &f32 {
        &self.projectile_force_transfer_mul
    }
    fn projectile_force_transfer_mul_mut(&mut self) -> &mut f32 {
        &mut self.projectile_force_transfer_mul
    }
    fn activate_on_spawn(&self) -> &bool {
        &self.activate_on_spawn
    }
    fn activate_on_spawn_mut(&mut self) -> &mut bool {
        &mut self.activate_on_spawn
    }
    fn in_effect_world_only(&self) -> &bool {
        &self.in_effect_world_only
    }
    fn in_effect_world_only_mut(&mut self) -> &mut bool {
        &mut self.in_effect_world_only
    }
    fn no_collision(&self) -> &bool {
        &self.no_collision
    }
    fn no_collision_mut(&mut self) -> &mut bool {
        &mut self.no_collision
    }
    fn activation_effect(&self) -> &Option<LockedTypeObject /* super::effect_base::EffectBlueprint */> {
        &self.activation_effect
    }
    fn activation_effect_mut(&mut self) -> &mut Option<LockedTypeObject /* super::effect_base::EffectBlueprint */> {
        &mut self.activation_effect
    }
    fn is_activation_effect_dynamic(&self) -> &bool {
        &self.is_activation_effect_dynamic
    }
    fn is_activation_effect_dynamic_mut(&mut self) -> &mut bool {
        &mut self.is_activation_effect_dynamic
    }
    fn deactivate_parts_on_sleep(&self) -> &bool {
        &self.deactivate_parts_on_sleep
    }
    fn deactivate_parts_on_sleep_mut(&mut self) -> &mut bool {
        &mut self.deactivate_parts_on_sleep
    }
    fn on_part_collision_enable(&self) -> &bool {
        &self.on_part_collision_enable
    }
    fn on_part_collision_enable_mut(&mut self) -> &mut bool {
        &mut self.on_part_collision_enable
    }
    fn on_part_collision_speed_threshold(&self) -> &f32 {
        &self.on_part_collision_speed_threshold
    }
    fn on_part_collision_speed_threshold_mut(&mut self) -> &mut f32 {
        &mut self.on_part_collision_speed_threshold
    }
    fn kill_parts_on_collision(&self) -> &bool {
        &self.kill_parts_on_collision
    }
    fn kill_parts_on_collision_mut(&mut self) -> &mut bool {
        &mut self.kill_parts_on_collision
    }
    fn effect(&self) -> &Option<LockedTypeObject /* super::effect_base::EffectBlueprint */> {
        &self.effect
    }
    fn effect_mut(&mut self) -> &mut Option<LockedTypeObject /* super::effect_base::EffectBlueprint */> {
        &mut self.effect
    }
    fn explosion(&self) -> &Option<LockedTypeObject /* super::entity::SpatialEntityData */> {
        &self.explosion
    }
    fn explosion_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::SpatialEntityData */> {
        &mut self.explosion
    }
    fn spawn_explosion_on_first_impact_only(&self) -> &bool {
        &self.spawn_explosion_on_first_impact_only
    }
    fn spawn_explosion_on_first_impact_only_mut(&mut self) -> &mut bool {
        &mut self.spawn_explosion_on_first_impact_only
    }
}

impl super::entity::GameComponentEntityDataTrait for DebrisClusterData {
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::entity::ComponentEntityDataTrait for DebrisClusterData {
    fn components(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components_mut()
    }
    fn part_bounding_boxes(&self) -> &Vec<BoxedTypeObject /* super::core::AxisAlignedBox */> {
        self._glacier_base.part_bounding_boxes()
    }
    fn part_bounding_boxes_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::core::AxisAlignedBox */> {
        self._glacier_base.part_bounding_boxes_mut()
    }
    fn client_runtime_component_count(&self) -> &u8 {
        self._glacier_base.client_runtime_component_count()
    }
    fn client_runtime_component_count_mut(&mut self) -> &mut u8 {
        self._glacier_base.client_runtime_component_count_mut()
    }
    fn server_runtime_component_count(&self) -> &u8 {
        self._glacier_base.server_runtime_component_count()
    }
    fn server_runtime_component_count_mut(&mut self) -> &mut u8 {
        self._glacier_base.server_runtime_component_count_mut()
    }
    fn client_runtime_transformation_count(&self) -> &u8 {
        self._glacier_base.client_runtime_transformation_count()
    }
    fn client_runtime_transformation_count_mut(&mut self) -> &mut u8 {
        self._glacier_base.client_runtime_transformation_count_mut()
    }
    fn server_runtime_transformation_count(&self) -> &u8 {
        self._glacier_base.server_runtime_transformation_count()
    }
    fn server_runtime_transformation_count_mut(&mut self) -> &mut u8 {
        self._glacier_base.server_runtime_transformation_count_mut()
    }
}

impl super::entity::SpatialEntityDataTrait for DebrisClusterData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
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
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for DebrisClusterData {
}

impl super::core::DataContainerTrait for DebrisClusterData {
}

pub static DEBRISCLUSTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisClusterData",
    name_hash: 3778917748,
    flags: MemberInfoFlags::new(101),
    module: "Debris",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::GAMECOMPONENTENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(DebrisClusterData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DebrisClusterData as Default>::default())),
            create_boxed: || Box::new(<DebrisClusterData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "MaxActivePartsCount",
                name_hash: 2283606746,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DebrisClusterData, max_active_parts_count),
            },
            FieldInfoData {
                name: "CullDistanceScale",
                name_hash: 3220442920,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebrisClusterData, cull_distance_scale),
            },
            FieldInfoData {
                name: "LightSamplingOffset",
                name_hash: 3078140501,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebrisClusterData, light_sampling_offset),
            },
            FieldInfoData {
                name: "HeightLimit",
                name_hash: 3581073167,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebrisClusterData, height_limit),
            },
            FieldInfoData {
                name: "RuntimeClusterLifetime",
                name_hash: 1329570240,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebrisClusterData, runtime_cluster_lifetime),
            },
            FieldInfoData {
                name: "Mesh",
                name_hash: 2088783990,
                flags: MemberInfoFlags::new(0),
                field_type: "MeshBaseAsset",
                rust_offset: offset_of!(DebrisClusterData, mesh),
            },
            FieldInfoData {
                name: "CompositePartCount",
                name_hash: 3195612804,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DebrisClusterData, composite_part_count),
            },
            FieldInfoData {
                name: "PartHierarchy",
                name_hash: 3544065669,
                flags: MemberInfoFlags::new(144),
                field_type: "DebrisClusterPartInfoData-Array",
                rust_offset: offset_of!(DebrisClusterData, part_hierarchy),
            },
            FieldInfoData {
                name: "RigidBodies",
                name_hash: 3015855522,
                flags: MemberInfoFlags::new(144),
                field_type: "RigidBodyData-Array",
                rust_offset: offset_of!(DebrisClusterData, rigid_bodies),
            },
            FieldInfoData {
                name: "PartialDestruction",
                name_hash: 2157329736,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisClusterData, partial_destruction),
            },
            FieldInfoData {
                name: "ClientSideOnly",
                name_hash: 3628043763,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisClusterData, client_side_only),
            },
            FieldInfoData {
                name: "ActivationPushForceMul",
                name_hash: 4184797926,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebrisClusterData, activation_push_force_mul),
            },
            FieldInfoData {
                name: "PushVelocityMul",
                name_hash: 3314270712,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(DebrisClusterData, push_velocity_mul),
            },
            FieldInfoData {
                name: "PushVelocityRndMul",
                name_hash: 2525151328,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(DebrisClusterData, push_velocity_rnd_mul),
            },
            FieldInfoData {
                name: "InitRotationRndMul",
                name_hash: 2277112647,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(DebrisClusterData, init_rotation_rnd_mul),
            },
            FieldInfoData {
                name: "ProjectileForceTransferMul",
                name_hash: 501994258,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebrisClusterData, projectile_force_transfer_mul),
            },
            FieldInfoData {
                name: "ActivateOnSpawn",
                name_hash: 471103942,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisClusterData, activate_on_spawn),
            },
            FieldInfoData {
                name: "InEffectWorldOnly",
                name_hash: 1269951075,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisClusterData, in_effect_world_only),
            },
            FieldInfoData {
                name: "NoCollision",
                name_hash: 3513186074,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisClusterData, no_collision),
            },
            FieldInfoData {
                name: "ActivationEffect",
                name_hash: 2134350406,
                flags: MemberInfoFlags::new(0),
                field_type: "EffectBlueprint",
                rust_offset: offset_of!(DebrisClusterData, activation_effect),
            },
            FieldInfoData {
                name: "IsActivationEffectDynamic",
                name_hash: 168612777,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisClusterData, is_activation_effect_dynamic),
            },
            FieldInfoData {
                name: "DeactivatePartsOnSleep",
                name_hash: 4098365303,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisClusterData, deactivate_parts_on_sleep),
            },
            FieldInfoData {
                name: "OnPartCollisionEnable",
                name_hash: 1786102988,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisClusterData, on_part_collision_enable),
            },
            FieldInfoData {
                name: "OnPartCollisionSpeedThreshold",
                name_hash: 322306461,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebrisClusterData, on_part_collision_speed_threshold),
            },
            FieldInfoData {
                name: "KillPartsOnCollision",
                name_hash: 1968153084,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisClusterData, kill_parts_on_collision),
            },
            FieldInfoData {
                name: "Effect",
                name_hash: 2332983090,
                flags: MemberInfoFlags::new(0),
                field_type: "EffectBlueprint",
                rust_offset: offset_of!(DebrisClusterData, effect),
            },
            FieldInfoData {
                name: "Explosion",
                name_hash: 2222171184,
                flags: MemberInfoFlags::new(0),
                field_type: "SpatialEntityData",
                rust_offset: offset_of!(DebrisClusterData, explosion),
            },
            FieldInfoData {
                name: "SpawnExplosionOnFirstImpactOnly",
                name_hash: 3769890534,
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


pub static DEBRISCLUSTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisClusterData-Array",
    name_hash: 2781684032,
    flags: MemberInfoFlags::new(145),
    module: "Debris",
    data: TypeInfoData::Array("DebrisClusterData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn part_index_mut(&mut self) -> &mut i32;
    fn number_of_children(&self) -> &i32;
    fn number_of_children_mut(&mut self) -> &mut i32;
    fn in_effect_world_only(&self) -> &bool;
    fn in_effect_world_only_mut(&mut self) -> &mut bool;
    fn affected_by_collision(&self) -> &bool;
    fn affected_by_collision_mut(&mut self) -> &mut bool;
    fn split_speed_threshold(&self) -> &f32;
    fn split_speed_threshold_mut(&mut self) -> &mut f32;
    fn delay(&self) -> &f32;
    fn delay_mut(&mut self) -> &mut f32;
    fn force(&self) -> &f32;
    fn force_mut(&mut self) -> &mut f32;
    fn min_force(&self) -> &f32;
    fn min_force_mut(&mut self) -> &mut f32;
    fn max_force(&self) -> &f32;
    fn max_force_mut(&mut self) -> &mut f32;
    fn layered_impact_delay(&self) -> &f32;
    fn layered_impact_delay_mut(&mut self) -> &mut f32;
    fn layered_collision_delay(&self) -> &f32;
    fn layered_collision_delay_mut(&mut self) -> &mut f32;
    fn linear_velocity(&self) -> &super::core::Vec3;
    fn linear_velocity_mut(&mut self) -> &mut super::core::Vec3;
    fn angular_velocity(&self) -> &super::core::Vec3;
    fn angular_velocity_mut(&mut self) -> &mut super::core::Vec3;
    fn sync_rest_position(&self) -> &bool;
    fn sync_rest_position_mut(&mut self) -> &mut bool;
    fn sync_continous(&self) -> &bool;
    fn sync_continous_mut(&mut self) -> &mut bool;
}

impl DebrisClusterPartInfoDataTrait for DebrisClusterPartInfoData {
    fn part_index(&self) -> &i32 {
        &self.part_index
    }
    fn part_index_mut(&mut self) -> &mut i32 {
        &mut self.part_index
    }
    fn number_of_children(&self) -> &i32 {
        &self.number_of_children
    }
    fn number_of_children_mut(&mut self) -> &mut i32 {
        &mut self.number_of_children
    }
    fn in_effect_world_only(&self) -> &bool {
        &self.in_effect_world_only
    }
    fn in_effect_world_only_mut(&mut self) -> &mut bool {
        &mut self.in_effect_world_only
    }
    fn affected_by_collision(&self) -> &bool {
        &self.affected_by_collision
    }
    fn affected_by_collision_mut(&mut self) -> &mut bool {
        &mut self.affected_by_collision
    }
    fn split_speed_threshold(&self) -> &f32 {
        &self.split_speed_threshold
    }
    fn split_speed_threshold_mut(&mut self) -> &mut f32 {
        &mut self.split_speed_threshold
    }
    fn delay(&self) -> &f32 {
        &self.delay
    }
    fn delay_mut(&mut self) -> &mut f32 {
        &mut self.delay
    }
    fn force(&self) -> &f32 {
        &self.force
    }
    fn force_mut(&mut self) -> &mut f32 {
        &mut self.force
    }
    fn min_force(&self) -> &f32 {
        &self.min_force
    }
    fn min_force_mut(&mut self) -> &mut f32 {
        &mut self.min_force
    }
    fn max_force(&self) -> &f32 {
        &self.max_force
    }
    fn max_force_mut(&mut self) -> &mut f32 {
        &mut self.max_force
    }
    fn layered_impact_delay(&self) -> &f32 {
        &self.layered_impact_delay
    }
    fn layered_impact_delay_mut(&mut self) -> &mut f32 {
        &mut self.layered_impact_delay
    }
    fn layered_collision_delay(&self) -> &f32 {
        &self.layered_collision_delay
    }
    fn layered_collision_delay_mut(&mut self) -> &mut f32 {
        &mut self.layered_collision_delay
    }
    fn linear_velocity(&self) -> &super::core::Vec3 {
        &self.linear_velocity
    }
    fn linear_velocity_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.linear_velocity
    }
    fn angular_velocity(&self) -> &super::core::Vec3 {
        &self.angular_velocity
    }
    fn angular_velocity_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.angular_velocity
    }
    fn sync_rest_position(&self) -> &bool {
        &self.sync_rest_position
    }
    fn sync_rest_position_mut(&mut self) -> &mut bool {
        &mut self.sync_rest_position
    }
    fn sync_continous(&self) -> &bool {
        &self.sync_continous
    }
    fn sync_continous_mut(&mut self) -> &mut bool {
        &mut self.sync_continous
    }
}

pub static DEBRISCLUSTERPARTINFODATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisClusterPartInfoData",
    name_hash: 943470029,
    flags: MemberInfoFlags::new(36937),
    module: "Debris",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DebrisClusterPartInfoData as Default>::default())),
            create_boxed: || Box::new(<DebrisClusterPartInfoData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "PartIndex",
                name_hash: 3213901068,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(DebrisClusterPartInfoData, part_index),
            },
            FieldInfoData {
                name: "NumberOfChildren",
                name_hash: 798522684,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(DebrisClusterPartInfoData, number_of_children),
            },
            FieldInfoData {
                name: "InEffectWorldOnly",
                name_hash: 1269951075,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisClusterPartInfoData, in_effect_world_only),
            },
            FieldInfoData {
                name: "AffectedByCollision",
                name_hash: 93787922,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisClusterPartInfoData, affected_by_collision),
            },
            FieldInfoData {
                name: "SplitSpeedThreshold",
                name_hash: 2280900007,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebrisClusterPartInfoData, split_speed_threshold),
            },
            FieldInfoData {
                name: "Delay",
                name_hash: 208768368,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebrisClusterPartInfoData, delay),
            },
            FieldInfoData {
                name: "Force",
                name_hash: 206910456,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebrisClusterPartInfoData, force),
            },
            FieldInfoData {
                name: "MinForce",
                name_hash: 3352963314,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebrisClusterPartInfoData, min_force),
            },
            FieldInfoData {
                name: "MaxForce",
                name_hash: 427399596,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebrisClusterPartInfoData, max_force),
            },
            FieldInfoData {
                name: "LayeredImpactDelay",
                name_hash: 3016899600,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebrisClusterPartInfoData, layered_impact_delay),
            },
            FieldInfoData {
                name: "LayeredCollisionDelay",
                name_hash: 4035460716,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebrisClusterPartInfoData, layered_collision_delay),
            },
            FieldInfoData {
                name: "LinearVelocity",
                name_hash: 1252550863,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(DebrisClusterPartInfoData, linear_velocity),
            },
            FieldInfoData {
                name: "AngularVelocity",
                name_hash: 1379775472,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(DebrisClusterPartInfoData, angular_velocity),
            },
            FieldInfoData {
                name: "SyncRestPosition",
                name_hash: 535707627,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisClusterPartInfoData, sync_rest_position),
            },
            FieldInfoData {
                name: "SyncContinous",
                name_hash: 618548762,
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


pub static DEBRISCLUSTERPARTINFODATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisClusterPartInfoData-Array",
    name_hash: 2061634041,
    flags: MemberInfoFlags::new(145),
    module: "Debris",
    data: TypeInfoData::Array("DebrisClusterPartInfoData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn enable_mut(&mut self) -> &mut bool;
    fn time_scale(&self) -> &f32;
    fn time_scale_mut(&mut self) -> &mut f32;
    fn enable_jobs(&self) -> &bool;
    fn enable_jobs_mut(&mut self) -> &mut bool;
    fn draw_stats(&self) -> &u32;
    fn draw_stats_mut(&mut self) -> &mut u32;
    fn draw_enable(&self) -> &bool;
    fn draw_enable_mut(&mut self) -> &mut bool;
    fn mesh_havok_rendering_enable(&self) -> &bool;
    fn mesh_havok_rendering_enable_mut(&mut self) -> &mut bool;
    fn mesh_draw_transforms(&self) -> &bool;
    fn mesh_draw_transforms_mut(&mut self) -> &mut bool;
    fn mesh_draw_bounding_boxes(&self) -> &bool;
    fn mesh_draw_bounding_boxes_mut(&mut self) -> &mut bool;
    fn mesh_shadow_enable(&self) -> &bool;
    fn mesh_shadow_enable_mut(&mut self) -> &mut bool;
    fn mesh_view_culling_enable(&self) -> &bool;
    fn mesh_view_culling_enable_mut(&mut self) -> &mut bool;
    fn mesh_culling_distance(&self) -> &f32;
    fn mesh_culling_distance_mut(&mut self) -> &mut f32;
    fn mesh_batch_count_limit(&self) -> &u32;
    fn mesh_batch_count_limit_mut(&mut self) -> &mut u32;
    fn debris_pool_size(&self) -> &u32;
    fn debris_pool_size_mut(&mut self) -> &mut u32;
    fn mesh_draw_count_limit(&self) -> &u32;
    fn mesh_draw_count_limit_mut(&mut self) -> &mut u32;
    fn mesh_streaming_priority_multiplier(&self) -> &f32;
    fn mesh_streaming_priority_multiplier_mut(&mut self) -> &mut f32;
    fn mesh_draw_cull_stats(&self) -> &bool;
    fn mesh_draw_cull_stats_mut(&mut self) -> &mut bool;
    fn client_max_debris_instances(&self) -> &u32;
    fn client_max_debris_instances_mut(&mut self) -> &mut u32;
    fn debris_quality_level(&self) -> &super::core::QualityLevel;
    fn debris_quality_level_mut(&mut self) -> &mut super::core::QualityLevel;
}

impl DebrisSystemSettingsTrait for DebrisSystemSettings {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn enable_mut(&mut self) -> &mut bool {
        &mut self.enable
    }
    fn time_scale(&self) -> &f32 {
        &self.time_scale
    }
    fn time_scale_mut(&mut self) -> &mut f32 {
        &mut self.time_scale
    }
    fn enable_jobs(&self) -> &bool {
        &self.enable_jobs
    }
    fn enable_jobs_mut(&mut self) -> &mut bool {
        &mut self.enable_jobs
    }
    fn draw_stats(&self) -> &u32 {
        &self.draw_stats
    }
    fn draw_stats_mut(&mut self) -> &mut u32 {
        &mut self.draw_stats
    }
    fn draw_enable(&self) -> &bool {
        &self.draw_enable
    }
    fn draw_enable_mut(&mut self) -> &mut bool {
        &mut self.draw_enable
    }
    fn mesh_havok_rendering_enable(&self) -> &bool {
        &self.mesh_havok_rendering_enable
    }
    fn mesh_havok_rendering_enable_mut(&mut self) -> &mut bool {
        &mut self.mesh_havok_rendering_enable
    }
    fn mesh_draw_transforms(&self) -> &bool {
        &self.mesh_draw_transforms
    }
    fn mesh_draw_transforms_mut(&mut self) -> &mut bool {
        &mut self.mesh_draw_transforms
    }
    fn mesh_draw_bounding_boxes(&self) -> &bool {
        &self.mesh_draw_bounding_boxes
    }
    fn mesh_draw_bounding_boxes_mut(&mut self) -> &mut bool {
        &mut self.mesh_draw_bounding_boxes
    }
    fn mesh_shadow_enable(&self) -> &bool {
        &self.mesh_shadow_enable
    }
    fn mesh_shadow_enable_mut(&mut self) -> &mut bool {
        &mut self.mesh_shadow_enable
    }
    fn mesh_view_culling_enable(&self) -> &bool {
        &self.mesh_view_culling_enable
    }
    fn mesh_view_culling_enable_mut(&mut self) -> &mut bool {
        &mut self.mesh_view_culling_enable
    }
    fn mesh_culling_distance(&self) -> &f32 {
        &self.mesh_culling_distance
    }
    fn mesh_culling_distance_mut(&mut self) -> &mut f32 {
        &mut self.mesh_culling_distance
    }
    fn mesh_batch_count_limit(&self) -> &u32 {
        &self.mesh_batch_count_limit
    }
    fn mesh_batch_count_limit_mut(&mut self) -> &mut u32 {
        &mut self.mesh_batch_count_limit
    }
    fn debris_pool_size(&self) -> &u32 {
        &self.debris_pool_size
    }
    fn debris_pool_size_mut(&mut self) -> &mut u32 {
        &mut self.debris_pool_size
    }
    fn mesh_draw_count_limit(&self) -> &u32 {
        &self.mesh_draw_count_limit
    }
    fn mesh_draw_count_limit_mut(&mut self) -> &mut u32 {
        &mut self.mesh_draw_count_limit
    }
    fn mesh_streaming_priority_multiplier(&self) -> &f32 {
        &self.mesh_streaming_priority_multiplier
    }
    fn mesh_streaming_priority_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.mesh_streaming_priority_multiplier
    }
    fn mesh_draw_cull_stats(&self) -> &bool {
        &self.mesh_draw_cull_stats
    }
    fn mesh_draw_cull_stats_mut(&mut self) -> &mut bool {
        &mut self.mesh_draw_cull_stats
    }
    fn client_max_debris_instances(&self) -> &u32 {
        &self.client_max_debris_instances
    }
    fn client_max_debris_instances_mut(&mut self) -> &mut u32 {
        &mut self.client_max_debris_instances
    }
    fn debris_quality_level(&self) -> &super::core::QualityLevel {
        &self.debris_quality_level
    }
    fn debris_quality_level_mut(&mut self) -> &mut super::core::QualityLevel {
        &mut self.debris_quality_level
    }
}

impl super::core::DataContainerTrait for DebrisSystemSettings {
}

pub static DEBRISSYSTEMSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisSystemSettings",
    name_hash: 1652536334,
    flags: MemberInfoFlags::new(101),
    module: "Debris",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(DebrisSystemSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DebrisSystemSettings as Default>::default())),
            create_boxed: || Box::new(<DebrisSystemSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                name_hash: 2342790116,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisSystemSettings, enable),
            },
            FieldInfoData {
                name: "TimeScale",
                name_hash: 169511528,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebrisSystemSettings, time_scale),
            },
            FieldInfoData {
                name: "EnableJobs",
                name_hash: 1190923856,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisSystemSettings, enable_jobs),
            },
            FieldInfoData {
                name: "DrawStats",
                name_hash: 2413142628,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DebrisSystemSettings, draw_stats),
            },
            FieldInfoData {
                name: "DrawEnable",
                name_hash: 1347356004,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisSystemSettings, draw_enable),
            },
            FieldInfoData {
                name: "MeshHavokRenderingEnable",
                name_hash: 1194385862,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisSystemSettings, mesh_havok_rendering_enable),
            },
            FieldInfoData {
                name: "MeshDrawTransforms",
                name_hash: 67393161,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisSystemSettings, mesh_draw_transforms),
            },
            FieldInfoData {
                name: "MeshDrawBoundingBoxes",
                name_hash: 1284065831,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisSystemSettings, mesh_draw_bounding_boxes),
            },
            FieldInfoData {
                name: "MeshShadowEnable",
                name_hash: 418824849,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisSystemSettings, mesh_shadow_enable),
            },
            FieldInfoData {
                name: "MeshViewCullingEnable",
                name_hash: 875555468,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisSystemSettings, mesh_view_culling_enable),
            },
            FieldInfoData {
                name: "MeshCullingDistance",
                name_hash: 2057455619,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebrisSystemSettings, mesh_culling_distance),
            },
            FieldInfoData {
                name: "MeshBatchCountLimit",
                name_hash: 1925711996,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DebrisSystemSettings, mesh_batch_count_limit),
            },
            FieldInfoData {
                name: "DebrisPoolSize",
                name_hash: 1654301847,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DebrisSystemSettings, debris_pool_size),
            },
            FieldInfoData {
                name: "MeshDrawCountLimit",
                name_hash: 3584341216,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DebrisSystemSettings, mesh_draw_count_limit),
            },
            FieldInfoData {
                name: "MeshStreamingPriorityMultiplier",
                name_hash: 1971206387,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebrisSystemSettings, mesh_streaming_priority_multiplier),
            },
            FieldInfoData {
                name: "MeshDrawCullStats",
                name_hash: 2146509665,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebrisSystemSettings, mesh_draw_cull_stats),
            },
            FieldInfoData {
                name: "ClientMaxDebrisInstances",
                name_hash: 2509489817,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DebrisSystemSettings, client_max_debris_instances),
            },
            FieldInfoData {
                name: "DebrisQualityLevel",
                name_hash: 2997009621,
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


pub static DEBRISSYSTEMSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisSystemSettings-Array",
    name_hash: 1518378298,
    flags: MemberInfoFlags::new(145),
    module: "Debris",
    data: TypeInfoData::Array("DebrisSystemSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DebrisSystemAsset {
    pub _glacier_base: super::core::Asset,
    pub havok_meshes: Vec<BoxedTypeObject /* DebrisHavokInfo */>,
    pub havok_mesh_count: i32,
}

pub trait DebrisSystemAssetTrait: super::core::AssetTrait {
    fn havok_meshes(&self) -> &Vec<BoxedTypeObject /* DebrisHavokInfo */>;
    fn havok_meshes_mut(&mut self) -> &mut Vec<BoxedTypeObject /* DebrisHavokInfo */>;
    fn havok_mesh_count(&self) -> &i32;
    fn havok_mesh_count_mut(&mut self) -> &mut i32;
}

impl DebrisSystemAssetTrait for DebrisSystemAsset {
    fn havok_meshes(&self) -> &Vec<BoxedTypeObject /* DebrisHavokInfo */> {
        &self.havok_meshes
    }
    fn havok_meshes_mut(&mut self) -> &mut Vec<BoxedTypeObject /* DebrisHavokInfo */> {
        &mut self.havok_meshes
    }
    fn havok_mesh_count(&self) -> &i32 {
        &self.havok_mesh_count
    }
    fn havok_mesh_count_mut(&mut self) -> &mut i32 {
        &mut self.havok_mesh_count
    }
}

impl super::core::AssetTrait for DebrisSystemAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for DebrisSystemAsset {
}

pub static DEBRISSYSTEMASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisSystemAsset",
    name_hash: 909166395,
    flags: MemberInfoFlags::new(101),
    module: "Debris",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(DebrisSystemAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DebrisSystemAsset as Default>::default())),
            create_boxed: || Box::new(<DebrisSystemAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "HavokMeshes",
                name_hash: 3852503739,
                flags: MemberInfoFlags::new(144),
                field_type: "DebrisHavokInfo-Array",
                rust_offset: offset_of!(DebrisSystemAsset, havok_meshes),
            },
            FieldInfoData {
                name: "HavokMeshCount",
                name_hash: 3497183694,
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


pub static DEBRISSYSTEMASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisSystemAsset-Array",
    name_hash: 2546593423,
    flags: MemberInfoFlags::new(145),
    module: "Debris",
    data: TypeInfoData::Array("DebrisSystemAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DebrisHavokInfo {
    pub havok_asset: Option<LockedTypeObject /* super::physics::HavokAsset */>,
    pub reserve_count: i32,
}

pub trait DebrisHavokInfoTrait: TypeObject {
    fn havok_asset(&self) -> &Option<LockedTypeObject /* super::physics::HavokAsset */>;
    fn havok_asset_mut(&mut self) -> &mut Option<LockedTypeObject /* super::physics::HavokAsset */>;
    fn reserve_count(&self) -> &i32;
    fn reserve_count_mut(&mut self) -> &mut i32;
}

impl DebrisHavokInfoTrait for DebrisHavokInfo {
    fn havok_asset(&self) -> &Option<LockedTypeObject /* super::physics::HavokAsset */> {
        &self.havok_asset
    }
    fn havok_asset_mut(&mut self) -> &mut Option<LockedTypeObject /* super::physics::HavokAsset */> {
        &mut self.havok_asset
    }
    fn reserve_count(&self) -> &i32 {
        &self.reserve_count
    }
    fn reserve_count_mut(&mut self) -> &mut i32 {
        &mut self.reserve_count
    }
}

pub static DEBRISHAVOKINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisHavokInfo",
    name_hash: 1298366875,
    flags: MemberInfoFlags::new(73),
    module: "Debris",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DebrisHavokInfo as Default>::default())),
            create_boxed: || Box::new(<DebrisHavokInfo as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "HavokAsset",
                name_hash: 2342641198,
                flags: MemberInfoFlags::new(0),
                field_type: "HavokAsset",
                rust_offset: offset_of!(DebrisHavokInfo, havok_asset),
            },
            FieldInfoData {
                name: "ReserveCount",
                name_hash: 1845599910,
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


pub static DEBRISHAVOKINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisHavokInfo-Array",
    name_hash: 3994204207,
    flags: MemberInfoFlags::new(145),
    module: "Debris",
    data: TypeInfoData::Array("DebrisHavokInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DebrisSystemMetrics {
    pub havok_particle_count: i32,
    pub havok_particle_part_count: i32,
}

pub trait DebrisSystemMetricsTrait: TypeObject {
    fn havok_particle_count(&self) -> &i32;
    fn havok_particle_count_mut(&mut self) -> &mut i32;
    fn havok_particle_part_count(&self) -> &i32;
    fn havok_particle_part_count_mut(&mut self) -> &mut i32;
}

impl DebrisSystemMetricsTrait for DebrisSystemMetrics {
    fn havok_particle_count(&self) -> &i32 {
        &self.havok_particle_count
    }
    fn havok_particle_count_mut(&mut self) -> &mut i32 {
        &mut self.havok_particle_count
    }
    fn havok_particle_part_count(&self) -> &i32 {
        &self.havok_particle_part_count
    }
    fn havok_particle_part_count_mut(&mut self) -> &mut i32 {
        &mut self.havok_particle_part_count
    }
}

pub static DEBRISSYSTEMMETRICS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisSystemMetrics",
    name_hash: 546430716,
    flags: MemberInfoFlags::new(36937),
    module: "Debris",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DebrisSystemMetrics as Default>::default())),
            create_boxed: || Box::new(<DebrisSystemMetrics as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "HavokParticleCount",
                name_hash: 3946001929,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(DebrisSystemMetrics, havok_particle_count),
            },
            FieldInfoData {
                name: "HavokParticlePartCount",
                name_hash: 580687198,
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


pub static DEBRISSYSTEMMETRICS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisSystemMetrics-Array",
    name_hash: 3883517640,
    flags: MemberInfoFlags::new(145),
    module: "Debris",
    data: TypeInfoData::Array("DebrisSystemMetrics"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 12075201,
    flags: MemberInfoFlags::new(101),
    module: "Debris",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DEBRISCLUSTER_TYPE_INFO),
        super_class_offset: offset_of!(ServerDebrisCluster, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerDebrisCluster as Default>::default())),
            create_boxed: || Box::new(<ServerDebrisCluster as Default>::default()),
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


pub static SERVERDEBRISCLUSTER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDebrisCluster-Array",
    name_hash: 221989621,
    flags: MemberInfoFlags::new(145),
    module: "Debris",
    data: TypeInfoData::Array("ServerDebrisCluster"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1818122265,
    flags: MemberInfoFlags::new(101),
    module: "Debris",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYEVENT_TYPE_INFO),
        super_class_offset: offset_of!(DebrisSpawnEvent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DebrisSpawnEvent as Default>::default())),
            create_boxed: || Box::new(<DebrisSpawnEvent as Default>::default()),
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


pub static DEBRISSPAWNEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisSpawnEvent-Array",
    name_hash: 4064015789,
    flags: MemberInfoFlags::new(145),
    module: "Debris",
    data: TypeInfoData::Array("DebrisSpawnEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2312261956,
    flags: MemberInfoFlags::new(101),
    module: "Debris",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::COMPONENTENTITY_TYPE_INFO),
        super_class_offset: offset_of!(DebrisCluster, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DebrisCluster as Default>::default())),
            create_boxed: || Box::new(<DebrisCluster as Default>::default()),
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


pub static DEBRISCLUSTER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisCluster-Array",
    name_hash: 431789552,
    flags: MemberInfoFlags::new(145),
    module: "Debris",
    data: TypeInfoData::Array("DebrisCluster"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 4016155037,
    flags: MemberInfoFlags::new(101),
    module: "Debris",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DEBRISCLUSTER_TYPE_INFO),
        super_class_offset: offset_of!(ClientDebrisCluster, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientDebrisCluster as Default>::default())),
            create_boxed: || Box::new(<ClientDebrisCluster as Default>::default()),
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


pub static CLIENTDEBRISCLUSTER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDebrisCluster-Array",
    name_hash: 924194089,
    flags: MemberInfoFlags::new(145),
    module: "Debris",
    data: TypeInfoData::Array("ClientDebrisCluster"),
    array_type: None,
    alignment: 8,
};


