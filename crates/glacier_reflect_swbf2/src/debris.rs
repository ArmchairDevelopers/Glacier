use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
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

#[derive(Clone, PartialEq, Debug)]
pub struct DebrisCollisionComponentData {
}

pub const DEBRISCOLLISIONCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisCollisionComponentData",
    flags: MemberInfoFlags::new(101),
    module: "Debris",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENTDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DEBRISCOLLISIONCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DebrisCollisionComponentData {
    fn type_info() -> &'static TypeInfo {
        DEBRISCOLLISIONCOMPONENTDATA_TYPE_INFO
    }
}


pub const DEBRISCOLLISIONCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisCollisionComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Debris",
    data: TypeInfoData::Array("DebrisCollisionComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ProceduralDestructionForceData {
    pub force: f32,
    pub min_force: f32,
    pub max_force: f32,
    pub layered_impact_delay: f32,
    pub layered_collision_delay: f32,
}

pub const PROCEDURALDESTRUCTIONFORCEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProceduralDestructionForceData",
    flags: MemberInfoFlags::new(101),
    module: "Debris",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSPROPERTYRELATIONPROPERTYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Force",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ProceduralDestructionForceData, force),
            },
            FieldInfoData {
                name: "MinForce",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ProceduralDestructionForceData, min_force),
            },
            FieldInfoData {
                name: "MaxForce",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ProceduralDestructionForceData, max_force),
            },
            FieldInfoData {
                name: "LayeredImpactDelay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ProceduralDestructionForceData, layered_impact_delay),
            },
            FieldInfoData {
                name: "LayeredCollisionDelay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ProceduralDestructionForceData, layered_collision_delay),
            },
        ],
    }),
    array_type: Some(PROCEDURALDESTRUCTIONFORCEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProceduralDestructionForceData {
    fn type_info() -> &'static TypeInfo {
        PROCEDURALDESTRUCTIONFORCEDATA_TYPE_INFO
    }
}


pub const PROCEDURALDESTRUCTIONFORCEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProceduralDestructionForceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Debris",
    data: TypeInfoData::Array("ProceduralDestructionForceData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DebrisClusterData {
    pub max_active_parts_count: u32,
    pub cull_distance_scale: f32,
    pub light_sampling_offset: f32,
    pub height_limit: f32,
    pub runtime_cluster_lifetime: f32,
    pub mesh: super::render_base::MeshBaseAsset,
    pub composite_part_count: u32,
    pub part_hierarchy: Vec<DebrisClusterPartInfoData>,
    pub rigid_bodies: Vec<super::physics::RigidBodyData>,
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
    pub activation_effect: super::effect_base::EffectBlueprint,
    pub is_activation_effect_dynamic: bool,
    pub deactivate_parts_on_sleep: bool,
    pub on_part_collision_enable: bool,
    pub on_part_collision_speed_threshold: f32,
    pub kill_parts_on_collision: bool,
    pub effect: super::effect_base::EffectBlueprint,
    pub explosion: super::entity::SpatialEntityData,
    pub spawn_explosion_on_first_impact_only: bool,
}

pub const DEBRISCLUSTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisClusterData",
    flags: MemberInfoFlags::new(101),
    module: "Debris",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMECOMPONENTENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MaxActivePartsCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterData, max_active_parts_count),
            },
            FieldInfoData {
                name: "CullDistanceScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterData, cull_distance_scale),
            },
            FieldInfoData {
                name: "LightSamplingOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterData, light_sampling_offset),
            },
            FieldInfoData {
                name: "HeightLimit",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterData, height_limit),
            },
            FieldInfoData {
                name: "RuntimeClusterLifetime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterData, runtime_cluster_lifetime),
            },
            FieldInfoData {
                name: "Mesh",
                flags: MemberInfoFlags::new(0),
                field_type: MESHBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterData, mesh),
            },
            FieldInfoData {
                name: "CompositePartCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterData, composite_part_count),
            },
            FieldInfoData {
                name: "PartHierarchy",
                flags: MemberInfoFlags::new(144),
                field_type: DEBRISCLUSTERPARTINFODATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterData, part_hierarchy),
            },
            FieldInfoData {
                name: "RigidBodies",
                flags: MemberInfoFlags::new(144),
                field_type: RIGIDBODYDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterData, rigid_bodies),
            },
            FieldInfoData {
                name: "PartialDestruction",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterData, partial_destruction),
            },
            FieldInfoData {
                name: "ClientSideOnly",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterData, client_side_only),
            },
            FieldInfoData {
                name: "ActivationPushForceMul",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterData, activation_push_force_mul),
            },
            FieldInfoData {
                name: "PushVelocityMul",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterData, push_velocity_mul),
            },
            FieldInfoData {
                name: "PushVelocityRndMul",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterData, push_velocity_rnd_mul),
            },
            FieldInfoData {
                name: "InitRotationRndMul",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterData, init_rotation_rnd_mul),
            },
            FieldInfoData {
                name: "ProjectileForceTransferMul",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterData, projectile_force_transfer_mul),
            },
            FieldInfoData {
                name: "ActivateOnSpawn",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterData, activate_on_spawn),
            },
            FieldInfoData {
                name: "InEffectWorldOnly",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterData, in_effect_world_only),
            },
            FieldInfoData {
                name: "NoCollision",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterData, no_collision),
            },
            FieldInfoData {
                name: "ActivationEffect",
                flags: MemberInfoFlags::new(0),
                field_type: EFFECTBLUEPRINT_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterData, activation_effect),
            },
            FieldInfoData {
                name: "IsActivationEffectDynamic",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterData, is_activation_effect_dynamic),
            },
            FieldInfoData {
                name: "DeactivatePartsOnSleep",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterData, deactivate_parts_on_sleep),
            },
            FieldInfoData {
                name: "OnPartCollisionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterData, on_part_collision_enable),
            },
            FieldInfoData {
                name: "OnPartCollisionSpeedThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterData, on_part_collision_speed_threshold),
            },
            FieldInfoData {
                name: "KillPartsOnCollision",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterData, kill_parts_on_collision),
            },
            FieldInfoData {
                name: "Effect",
                flags: MemberInfoFlags::new(0),
                field_type: EFFECTBLUEPRINT_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterData, effect),
            },
            FieldInfoData {
                name: "Explosion",
                flags: MemberInfoFlags::new(0),
                field_type: SPATIALENTITYDATA_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterData, explosion),
            },
            FieldInfoData {
                name: "SpawnExplosionOnFirstImpactOnly",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterData, spawn_explosion_on_first_impact_only),
            },
        ],
    }),
    array_type: Some(DEBRISCLUSTERDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DebrisClusterData {
    fn type_info() -> &'static TypeInfo {
        DEBRISCLUSTERDATA_TYPE_INFO
    }
}


pub const DEBRISCLUSTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisClusterData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Debris",
    data: TypeInfoData::Array("DebrisClusterData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
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

pub const DEBRISCLUSTERPARTINFODATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisClusterPartInfoData",
    flags: MemberInfoFlags::new(36937),
    module: "Debris",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "PartIndex",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterPartInfoData, part_index),
            },
            FieldInfoData {
                name: "NumberOfChildren",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterPartInfoData, number_of_children),
            },
            FieldInfoData {
                name: "InEffectWorldOnly",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterPartInfoData, in_effect_world_only),
            },
            FieldInfoData {
                name: "AffectedByCollision",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterPartInfoData, affected_by_collision),
            },
            FieldInfoData {
                name: "SplitSpeedThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterPartInfoData, split_speed_threshold),
            },
            FieldInfoData {
                name: "Delay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterPartInfoData, delay),
            },
            FieldInfoData {
                name: "Force",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterPartInfoData, force),
            },
            FieldInfoData {
                name: "MinForce",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterPartInfoData, min_force),
            },
            FieldInfoData {
                name: "MaxForce",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterPartInfoData, max_force),
            },
            FieldInfoData {
                name: "LayeredImpactDelay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterPartInfoData, layered_impact_delay),
            },
            FieldInfoData {
                name: "LayeredCollisionDelay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterPartInfoData, layered_collision_delay),
            },
            FieldInfoData {
                name: "LinearVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterPartInfoData, linear_velocity),
            },
            FieldInfoData {
                name: "AngularVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterPartInfoData, angular_velocity),
            },
            FieldInfoData {
                name: "SyncRestPosition",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterPartInfoData, sync_rest_position),
            },
            FieldInfoData {
                name: "SyncContinous",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DebrisClusterPartInfoData, sync_continous),
            },
        ],
    }),
    array_type: Some(DEBRISCLUSTERPARTINFODATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DebrisClusterPartInfoData {
    fn type_info() -> &'static TypeInfo {
        DEBRISCLUSTERPARTINFODATA_TYPE_INFO
    }
}


pub const DEBRISCLUSTERPARTINFODATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisClusterPartInfoData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Debris",
    data: TypeInfoData::Array("DebrisClusterPartInfoData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DebrisSystemSettings {
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

pub const DEBRISSYSTEMSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisSystemSettings",
    flags: MemberInfoFlags::new(101),
    module: "Debris",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DebrisSystemSettings, enable),
            },
            FieldInfoData {
                name: "TimeScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DebrisSystemSettings, time_scale),
            },
            FieldInfoData {
                name: "EnableJobs",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DebrisSystemSettings, enable_jobs),
            },
            FieldInfoData {
                name: "DrawStats",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DebrisSystemSettings, draw_stats),
            },
            FieldInfoData {
                name: "DrawEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DebrisSystemSettings, draw_enable),
            },
            FieldInfoData {
                name: "MeshHavokRenderingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DebrisSystemSettings, mesh_havok_rendering_enable),
            },
            FieldInfoData {
                name: "MeshDrawTransforms",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DebrisSystemSettings, mesh_draw_transforms),
            },
            FieldInfoData {
                name: "MeshDrawBoundingBoxes",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DebrisSystemSettings, mesh_draw_bounding_boxes),
            },
            FieldInfoData {
                name: "MeshShadowEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DebrisSystemSettings, mesh_shadow_enable),
            },
            FieldInfoData {
                name: "MeshViewCullingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DebrisSystemSettings, mesh_view_culling_enable),
            },
            FieldInfoData {
                name: "MeshCullingDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DebrisSystemSettings, mesh_culling_distance),
            },
            FieldInfoData {
                name: "MeshBatchCountLimit",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DebrisSystemSettings, mesh_batch_count_limit),
            },
            FieldInfoData {
                name: "DebrisPoolSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DebrisSystemSettings, debris_pool_size),
            },
            FieldInfoData {
                name: "MeshDrawCountLimit",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DebrisSystemSettings, mesh_draw_count_limit),
            },
            FieldInfoData {
                name: "MeshStreamingPriorityMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DebrisSystemSettings, mesh_streaming_priority_multiplier),
            },
            FieldInfoData {
                name: "MeshDrawCullStats",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DebrisSystemSettings, mesh_draw_cull_stats),
            },
            FieldInfoData {
                name: "ClientMaxDebrisInstances",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DebrisSystemSettings, client_max_debris_instances),
            },
            FieldInfoData {
                name: "DebrisQualityLevel",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYLEVEL_TYPE_INFO,
                rust_offset: offset_of!(DebrisSystemSettings, debris_quality_level),
            },
        ],
    }),
    array_type: Some(DEBRISSYSTEMSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DebrisSystemSettings {
    fn type_info() -> &'static TypeInfo {
        DEBRISSYSTEMSETTINGS_TYPE_INFO
    }
}


pub const DEBRISSYSTEMSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisSystemSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Debris",
    data: TypeInfoData::Array("DebrisSystemSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DebrisSystemAsset {
    pub havok_meshes: Vec<DebrisHavokInfo>,
    pub havok_mesh_count: i32,
}

pub const DEBRISSYSTEMASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisSystemAsset",
    flags: MemberInfoFlags::new(101),
    module: "Debris",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "HavokMeshes",
                flags: MemberInfoFlags::new(144),
                field_type: DEBRISHAVOKINFO_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(DebrisSystemAsset, havok_meshes),
            },
            FieldInfoData {
                name: "HavokMeshCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(DebrisSystemAsset, havok_mesh_count),
            },
        ],
    }),
    array_type: Some(DEBRISSYSTEMASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DebrisSystemAsset {
    fn type_info() -> &'static TypeInfo {
        DEBRISSYSTEMASSET_TYPE_INFO
    }
}


pub const DEBRISSYSTEMASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisSystemAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Debris",
    data: TypeInfoData::Array("DebrisSystemAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DebrisHavokInfo {
    pub havok_asset: super::physics::HavokAsset,
    pub reserve_count: i32,
}

pub const DEBRISHAVOKINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisHavokInfo",
    flags: MemberInfoFlags::new(73),
    module: "Debris",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "HavokAsset",
                flags: MemberInfoFlags::new(0),
                field_type: HAVOKASSET_TYPE_INFO,
                rust_offset: offset_of!(DebrisHavokInfo, havok_asset),
            },
            FieldInfoData {
                name: "ReserveCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(DebrisHavokInfo, reserve_count),
            },
        ],
    }),
    array_type: Some(DEBRISHAVOKINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DebrisHavokInfo {
    fn type_info() -> &'static TypeInfo {
        DEBRISHAVOKINFO_TYPE_INFO
    }
}


pub const DEBRISHAVOKINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisHavokInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Debris",
    data: TypeInfoData::Array("DebrisHavokInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DebrisSystemMetrics {
    pub havok_particle_count: i32,
    pub havok_particle_part_count: i32,
}

pub const DEBRISSYSTEMMETRICS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisSystemMetrics",
    flags: MemberInfoFlags::new(36937),
    module: "Debris",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "HavokParticleCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(DebrisSystemMetrics, havok_particle_count),
            },
            FieldInfoData {
                name: "HavokParticlePartCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(DebrisSystemMetrics, havok_particle_part_count),
            },
        ],
    }),
    array_type: Some(DEBRISSYSTEMMETRICS_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for DebrisSystemMetrics {
    fn type_info() -> &'static TypeInfo {
        DEBRISSYSTEMMETRICS_TYPE_INFO
    }
}


pub const DEBRISSYSTEMMETRICS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisSystemMetrics-Array",
    flags: MemberInfoFlags::new(145),
    module: "Debris",
    data: TypeInfoData::Array("DebrisSystemMetrics-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerDebrisCluster {
}

pub const SERVERDEBRISCLUSTER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDebrisCluster",
    flags: MemberInfoFlags::new(101),
    module: "Debris",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DEBRISCLUSTER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERDEBRISCLUSTER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerDebrisCluster {
    fn type_info() -> &'static TypeInfo {
        SERVERDEBRISCLUSTER_TYPE_INFO
    }
}


pub const SERVERDEBRISCLUSTER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDebrisCluster-Array",
    flags: MemberInfoFlags::new(145),
    module: "Debris",
    data: TypeInfoData::Array("ServerDebrisCluster-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DebrisSpawnEvent {
}

pub const DEBRISSPAWNEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisSpawnEvent",
    flags: MemberInfoFlags::new(101),
    module: "Debris",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYEVENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DEBRISSPAWNEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DebrisSpawnEvent {
    fn type_info() -> &'static TypeInfo {
        DEBRISSPAWNEVENT_TYPE_INFO
    }
}


pub const DEBRISSPAWNEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisSpawnEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Debris",
    data: TypeInfoData::Array("DebrisSpawnEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DebrisCluster {
}

pub const DEBRISCLUSTER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisCluster",
    flags: MemberInfoFlags::new(101),
    module: "Debris",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENTENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DEBRISCLUSTER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DebrisCluster {
    fn type_info() -> &'static TypeInfo {
        DEBRISCLUSTER_TYPE_INFO
    }
}


pub const DEBRISCLUSTER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebrisCluster-Array",
    flags: MemberInfoFlags::new(145),
    module: "Debris",
    data: TypeInfoData::Array("DebrisCluster-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientDebrisCluster {
}

pub const CLIENTDEBRISCLUSTER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDebrisCluster",
    flags: MemberInfoFlags::new(101),
    module: "Debris",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DEBRISCLUSTER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTDEBRISCLUSTER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientDebrisCluster {
    fn type_info() -> &'static TypeInfo {
        CLIENTDEBRISCLUSTER_TYPE_INFO
    }
}


pub const CLIENTDEBRISCLUSTER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDebrisCluster-Array",
    flags: MemberInfoFlags::new(145),
    module: "Debris",
    data: TypeInfoData::Array("ClientDebrisCluster-Array"),
    array_type: None,
    alignment: 8,
};


