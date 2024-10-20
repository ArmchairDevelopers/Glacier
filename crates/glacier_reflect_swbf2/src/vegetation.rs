use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_vegetation_types(registry: &mut TypeRegistry) {
    registry.register_type(VEGETATIONSYSTEMSETTINGS_TYPE_INFO);
    registry.register_type(VEGETATIONSYSTEMSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(VEGETATIONTREEBREAKNODESTATE_TYPE_INFO);
    registry.register_type(VEGETATIONTREEBREAKNODESTATE_ARRAY_TYPE_INFO);
    registry.register_type(VEGETATIONTREEBREAKNODEDESTRUCTION_TYPE_INFO);
    registry.register_type(VEGETATIONTREEBREAKNODEDESTRUCTION_ARRAY_TYPE_INFO);
    registry.register_type(VEGETATIONTREEENTITYDATA_TYPE_INFO);
    registry.register_type(VEGETATIONTREEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(VEGETATIONEFFECTSLOT_TYPE_INFO);
    registry.register_type(VEGETATIONEFFECTSLOT_ARRAY_TYPE_INFO);
    registry.register_type(VEGETATIONBASEENTITYDATA_TYPE_INFO);
    registry.register_type(VEGETATIONBASEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(VEGETATIONTREEENTITY_TYPE_INFO);
    registry.register_type(VEGETATIONTREEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERVEGETATIONTREEENTITY_TYPE_INFO);
    registry.register_type(SERVERVEGETATIONTREEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTVEGETATIONTREEENTITY_TYPE_INFO);
    registry.register_type(CLIENTVEGETATIONTREEENTITY_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Debug)]
pub struct VegetationSystemSettings {
    pub destruction_enable: bool,
    pub max_active_distance: f32,
    pub max_effect_distance: f32,
    pub max_active_bones: u32,
    pub max_wiggle_distance: f32,
    pub dissolve_enable: bool,
    pub simulation_ease_out_time: f32,
    pub enable: bool,
    pub time_scale: f32,
    pub enable_jobs: bool,
    pub job_count: u32,
    pub draw_nodes: bool,
    pub draw_node_i_ds: bool,
    pub draw_node_stiffness: bool,
    pub draw_non_simulated_as_rigid: bool,
    pub draw_active_instance_boxes_enable: bool,
    pub draw_effect_debug_info_enable: bool,
    pub draw_damage_debug_info_enable: bool,
    pub draw_stats_enable: bool,
    pub procedural_animation_enable: bool,
}

pub const VEGETATIONSYSTEMSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationSystemSettings",
    flags: MemberInfoFlags::new(101),
    module: "Vegetation",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DestructionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VegetationSystemSettings, destruction_enable),
            },
            FieldInfoData {
                name: "MaxActiveDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VegetationSystemSettings, max_active_distance),
            },
            FieldInfoData {
                name: "MaxEffectDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VegetationSystemSettings, max_effect_distance),
            },
            FieldInfoData {
                name: "MaxActiveBones",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VegetationSystemSettings, max_active_bones),
            },
            FieldInfoData {
                name: "MaxWiggleDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VegetationSystemSettings, max_wiggle_distance),
            },
            FieldInfoData {
                name: "DissolveEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VegetationSystemSettings, dissolve_enable),
            },
            FieldInfoData {
                name: "SimulationEaseOutTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VegetationSystemSettings, simulation_ease_out_time),
            },
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VegetationSystemSettings, enable),
            },
            FieldInfoData {
                name: "TimeScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VegetationSystemSettings, time_scale),
            },
            FieldInfoData {
                name: "EnableJobs",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VegetationSystemSettings, enable_jobs),
            },
            FieldInfoData {
                name: "JobCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VegetationSystemSettings, job_count),
            },
            FieldInfoData {
                name: "DrawNodes",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VegetationSystemSettings, draw_nodes),
            },
            FieldInfoData {
                name: "DrawNodeIDs",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VegetationSystemSettings, draw_node_i_ds),
            },
            FieldInfoData {
                name: "DrawNodeStiffness",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VegetationSystemSettings, draw_node_stiffness),
            },
            FieldInfoData {
                name: "DrawNonSimulatedAsRigid",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VegetationSystemSettings, draw_non_simulated_as_rigid),
            },
            FieldInfoData {
                name: "DrawActiveInstanceBoxesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VegetationSystemSettings, draw_active_instance_boxes_enable),
            },
            FieldInfoData {
                name: "DrawEffectDebugInfoEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VegetationSystemSettings, draw_effect_debug_info_enable),
            },
            FieldInfoData {
                name: "DrawDamageDebugInfoEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VegetationSystemSettings, draw_damage_debug_info_enable),
            },
            FieldInfoData {
                name: "DrawStatsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VegetationSystemSettings, draw_stats_enable),
            },
            FieldInfoData {
                name: "ProceduralAnimationEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VegetationSystemSettings, procedural_animation_enable),
            },
        ],
    }),
    array_type: Some(VEGETATIONSYSTEMSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VegetationSystemSettings {
    fn type_info() -> &'static TypeInfo {
        VEGETATIONSYSTEMSETTINGS_TYPE_INFO
    }
}


pub const VEGETATIONSYSTEMSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationSystemSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Vegetation",
    data: TypeInfoData::Array("VegetationSystemSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct VegetationTreeBreakNodeState {
    pub break_node_bit: bool,
    pub trans: super::core::Vec3,
    pub rotation: super::core::Quat,
}

pub const VEGETATIONTREEBREAKNODESTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationTreeBreakNodeState",
    flags: MemberInfoFlags::new(36937),
    module: "Vegetation",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "BreakNodeBit",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VegetationTreeBreakNodeState, break_node_bit),
            },
            FieldInfoData {
                name: "Trans",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(VegetationTreeBreakNodeState, trans),
            },
            FieldInfoData {
                name: "Rotation",
                flags: MemberInfoFlags::new(0),
                field_type: QUAT_TYPE_INFO,
                rust_offset: offset_of!(VegetationTreeBreakNodeState, rotation),
            },
        ],
    }),
    array_type: Some(VEGETATIONTREEBREAKNODESTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for VegetationTreeBreakNodeState {
    fn type_info() -> &'static TypeInfo {
        VEGETATIONTREEBREAKNODESTATE_TYPE_INFO
    }
}


pub const VEGETATIONTREEBREAKNODESTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationTreeBreakNodeState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Vegetation",
    data: TypeInfoData::Array("VegetationTreeBreakNodeState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct VegetationTreeBreakNodeDestruction {
    pub break_node_bit: bool,
    pub height: f32,
    pub part_index: u32,
}

pub const VEGETATIONTREEBREAKNODEDESTRUCTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationTreeBreakNodeDestruction",
    flags: MemberInfoFlags::new(36937),
    module: "Vegetation",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "BreakNodeBit",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VegetationTreeBreakNodeDestruction, break_node_bit),
            },
            FieldInfoData {
                name: "Height",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VegetationTreeBreakNodeDestruction, height),
            },
            FieldInfoData {
                name: "PartIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VegetationTreeBreakNodeDestruction, part_index),
            },
        ],
    }),
    array_type: Some(VEGETATIONTREEBREAKNODEDESTRUCTION_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for VegetationTreeBreakNodeDestruction {
    fn type_info() -> &'static TypeInfo {
        VEGETATIONTREEBREAKNODEDESTRUCTION_TYPE_INFO
    }
}


pub const VEGETATIONTREEBREAKNODEDESTRUCTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationTreeBreakNodeDestruction-Array",
    flags: MemberInfoFlags::new(145),
    module: "Vegetation",
    data: TypeInfoData::Array("VegetationTreeBreakNodeDestruction-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct VegetationTreeEntityData {
    pub stiffness: f32,
    pub damping: f32,
    pub stem_mass: f32,
    pub stiffness_spread: f32,
    pub damping_spread: f32,
    pub mass_spread: f32,
    pub stem_locked_up_to: f32,
    pub stem_bone_count: i32,
    pub breakable_joint_threshold: f32,
    pub constant_falloff: bool,
    pub bounding_box_scale_factor: f32,
    pub indestructable: bool,
    pub parts_time_to_live: f32,
    pub linear_velocity_damping: f32,
    pub angular_velocity_damping: f32,
    pub friction: f32,
    pub restitution: f32,
    pub stem_physics_width: f32,
    pub stem_physics_height_scale: f32,
    pub branch_physics_width: f32,
    pub branch_physics_height_scale: f32,
    pub destruction_mass_scale: f32,
    pub inertia_modifier: super::core::Vec3,
    pub center_of_mass_vertical_scale: f32,
    pub stem_break_effect: VegetationEffectSlot,
    pub branch_break_effect: VegetationEffectSlot,
    pub impact_effect: VegetationEffectSlot,
    pub wind_effect: super::effect_base::EffectBlueprint,
    pub min_respawn_time: f32,
    pub stem_effect_node_threshold: u32,
    pub shadow_l_o_d_offset: u32,
    pub translucency_enabled: bool,
    pub translucency_volume_center: super::core::Vec3,
}

pub const VEGETATIONTREEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationTreeEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Vegetation",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VEGETATIONBASEENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Stiffness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VegetationTreeEntityData, stiffness),
            },
            FieldInfoData {
                name: "Damping",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VegetationTreeEntityData, damping),
            },
            FieldInfoData {
                name: "StemMass",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VegetationTreeEntityData, stem_mass),
            },
            FieldInfoData {
                name: "StiffnessSpread",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VegetationTreeEntityData, stiffness_spread),
            },
            FieldInfoData {
                name: "DampingSpread",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VegetationTreeEntityData, damping_spread),
            },
            FieldInfoData {
                name: "MassSpread",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VegetationTreeEntityData, mass_spread),
            },
            FieldInfoData {
                name: "StemLockedUpTo",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VegetationTreeEntityData, stem_locked_up_to),
            },
            FieldInfoData {
                name: "StemBoneCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(VegetationTreeEntityData, stem_bone_count),
            },
            FieldInfoData {
                name: "BreakableJointThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VegetationTreeEntityData, breakable_joint_threshold),
            },
            FieldInfoData {
                name: "ConstantFalloff",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VegetationTreeEntityData, constant_falloff),
            },
            FieldInfoData {
                name: "BoundingBoxScaleFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VegetationTreeEntityData, bounding_box_scale_factor),
            },
            FieldInfoData {
                name: "Indestructable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VegetationTreeEntityData, indestructable),
            },
            FieldInfoData {
                name: "PartsTimeToLive",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VegetationTreeEntityData, parts_time_to_live),
            },
            FieldInfoData {
                name: "LinearVelocityDamping",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VegetationTreeEntityData, linear_velocity_damping),
            },
            FieldInfoData {
                name: "AngularVelocityDamping",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VegetationTreeEntityData, angular_velocity_damping),
            },
            FieldInfoData {
                name: "Friction",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VegetationTreeEntityData, friction),
            },
            FieldInfoData {
                name: "Restitution",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VegetationTreeEntityData, restitution),
            },
            FieldInfoData {
                name: "StemPhysicsWidth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VegetationTreeEntityData, stem_physics_width),
            },
            FieldInfoData {
                name: "StemPhysicsHeightScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VegetationTreeEntityData, stem_physics_height_scale),
            },
            FieldInfoData {
                name: "BranchPhysicsWidth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VegetationTreeEntityData, branch_physics_width),
            },
            FieldInfoData {
                name: "BranchPhysicsHeightScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VegetationTreeEntityData, branch_physics_height_scale),
            },
            FieldInfoData {
                name: "DestructionMassScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VegetationTreeEntityData, destruction_mass_scale),
            },
            FieldInfoData {
                name: "InertiaModifier",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(VegetationTreeEntityData, inertia_modifier),
            },
            FieldInfoData {
                name: "CenterOfMassVerticalScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VegetationTreeEntityData, center_of_mass_vertical_scale),
            },
            FieldInfoData {
                name: "StemBreakEffect",
                flags: MemberInfoFlags::new(0),
                field_type: VEGETATIONEFFECTSLOT_TYPE_INFO,
                rust_offset: offset_of!(VegetationTreeEntityData, stem_break_effect),
            },
            FieldInfoData {
                name: "BranchBreakEffect",
                flags: MemberInfoFlags::new(0),
                field_type: VEGETATIONEFFECTSLOT_TYPE_INFO,
                rust_offset: offset_of!(VegetationTreeEntityData, branch_break_effect),
            },
            FieldInfoData {
                name: "ImpactEffect",
                flags: MemberInfoFlags::new(0),
                field_type: VEGETATIONEFFECTSLOT_TYPE_INFO,
                rust_offset: offset_of!(VegetationTreeEntityData, impact_effect),
            },
            FieldInfoData {
                name: "WindEffect",
                flags: MemberInfoFlags::new(0),
                field_type: EFFECTBLUEPRINT_TYPE_INFO,
                rust_offset: offset_of!(VegetationTreeEntityData, wind_effect),
            },
            FieldInfoData {
                name: "MinRespawnTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VegetationTreeEntityData, min_respawn_time),
            },
            FieldInfoData {
                name: "StemEffectNodeThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VegetationTreeEntityData, stem_effect_node_threshold),
            },
            FieldInfoData {
                name: "ShadowLODOffset",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VegetationTreeEntityData, shadow_l_o_d_offset),
            },
            FieldInfoData {
                name: "TranslucencyEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VegetationTreeEntityData, translucency_enabled),
            },
            FieldInfoData {
                name: "TranslucencyVolumeCenter",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(VegetationTreeEntityData, translucency_volume_center),
            },
        ],
    }),
    array_type: Some(VEGETATIONTREEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for VegetationTreeEntityData {
    fn type_info() -> &'static TypeInfo {
        VEGETATIONTREEENTITYDATA_TYPE_INFO
    }
}


pub const VEGETATIONTREEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationTreeEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Vegetation",
    data: TypeInfoData::Array("VegetationTreeEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct VegetationEffectSlot {
    pub effect: super::effect_base::EffectBlueprint,
    pub strength_min: f32,
    pub strength_max: f32,
    pub size_min: f32,
    pub size_max: f32,
}

pub const VEGETATIONEFFECTSLOT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationEffectSlot",
    flags: MemberInfoFlags::new(73),
    module: "Vegetation",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Effect",
                flags: MemberInfoFlags::new(0),
                field_type: EFFECTBLUEPRINT_TYPE_INFO,
                rust_offset: offset_of!(VegetationEffectSlot, effect),
            },
            FieldInfoData {
                name: "StrengthMin",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VegetationEffectSlot, strength_min),
            },
            FieldInfoData {
                name: "StrengthMax",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VegetationEffectSlot, strength_max),
            },
            FieldInfoData {
                name: "SizeMin",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VegetationEffectSlot, size_min),
            },
            FieldInfoData {
                name: "SizeMax",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VegetationEffectSlot, size_max),
            },
        ],
    }),
    array_type: Some(VEGETATIONEFFECTSLOT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VegetationEffectSlot {
    fn type_info() -> &'static TypeInfo {
        VEGETATIONEFFECTSLOT_TYPE_INFO
    }
}


pub const VEGETATIONEFFECTSLOT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationEffectSlot-Array",
    flags: MemberInfoFlags::new(145),
    module: "Vegetation",
    data: TypeInfoData::Array("VegetationEffectSlot-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct VegetationBaseEntityData {
    pub base_pose_transforms: super::core::SparseTransformArray,
    pub hierarchy: Vec<i32>,
    pub part_indirection: Vec<i32>,
    pub part_hierarchy: Vec<i32>,
    pub part_initial_healths: Vec<f32>,
    pub bone_is_stem: Vec<bool>,
    pub mesh: super::render_base::MeshBaseAsset,
    pub rigid_bodies: Vec<super::physics::RigidBodyData>,
}

pub const VEGETATIONBASEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationBaseEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Vegetation",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMECOMPONENTENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "BasePoseTransforms",
                flags: MemberInfoFlags::new(0),
                field_type: SPARSETRANSFORMARRAY_TYPE_INFO,
                rust_offset: offset_of!(VegetationBaseEntityData, base_pose_transforms),
            },
            FieldInfoData {
                name: "Hierarchy",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(VegetationBaseEntityData, hierarchy),
            },
            FieldInfoData {
                name: "PartIndirection",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(VegetationBaseEntityData, part_indirection),
            },
            FieldInfoData {
                name: "PartHierarchy",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(VegetationBaseEntityData, part_hierarchy),
            },
            FieldInfoData {
                name: "PartInitialHealths",
                flags: MemberInfoFlags::new(144),
                field_type: FLOAT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(VegetationBaseEntityData, part_initial_healths),
            },
            FieldInfoData {
                name: "BoneIsStem",
                flags: MemberInfoFlags::new(144),
                field_type: BOOLEAN_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(VegetationBaseEntityData, bone_is_stem),
            },
            FieldInfoData {
                name: "Mesh",
                flags: MemberInfoFlags::new(0),
                field_type: MESHBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(VegetationBaseEntityData, mesh),
            },
            FieldInfoData {
                name: "RigidBodies",
                flags: MemberInfoFlags::new(144),
                field_type: RIGIDBODYDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(VegetationBaseEntityData, rigid_bodies),
            },
        ],
    }),
    array_type: Some(VEGETATIONBASEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for VegetationBaseEntityData {
    fn type_info() -> &'static TypeInfo {
        VEGETATIONBASEENTITYDATA_TYPE_INFO
    }
}


pub const VEGETATIONBASEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationBaseEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Vegetation",
    data: TypeInfoData::Array("VegetationBaseEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VegetationTreeEntity {
}

pub const VEGETATIONTREEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationTreeEntity",
    flags: MemberInfoFlags::new(101),
    module: "Vegetation",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENTENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VEGETATIONTREEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for VegetationTreeEntity {
    fn type_info() -> &'static TypeInfo {
        VEGETATIONTREEENTITY_TYPE_INFO
    }
}


pub const VEGETATIONTREEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationTreeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Vegetation",
    data: TypeInfoData::Array("VegetationTreeEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerVegetationTreeEntity {
}

pub const SERVERVEGETATIONTREEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVegetationTreeEntity",
    flags: MemberInfoFlags::new(101),
    module: "Vegetation",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VEGETATIONTREEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERVEGETATIONTREEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerVegetationTreeEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERVEGETATIONTREEENTITY_TYPE_INFO
    }
}


pub const SERVERVEGETATIONTREEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVegetationTreeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Vegetation",
    data: TypeInfoData::Array("ServerVegetationTreeEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientVegetationTreeEntity {
}

pub const CLIENTVEGETATIONTREEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVegetationTreeEntity",
    flags: MemberInfoFlags::new(101),
    module: "Vegetation",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VEGETATIONTREEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVEGETATIONTREEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientVegetationTreeEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTVEGETATIONTREEENTITY_TYPE_INFO
    }
}


pub const CLIENTVEGETATIONTREEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVegetationTreeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Vegetation",
    data: TypeInfoData::Array("ClientVegetationTreeEntity-Array"),
    array_type: None,
    alignment: 8,
};


