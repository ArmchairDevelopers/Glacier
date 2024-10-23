use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct VegetationSystemSettings {
    pub _glacier_base: super::core::DataContainer,
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

pub trait VegetationSystemSettingsTrait: super::core::DataContainerTrait {
    fn destruction_enable(&self) -> &bool;
    fn destruction_enable_mut(&mut self) -> &mut bool;
    fn max_active_distance(&self) -> &f32;
    fn max_active_distance_mut(&mut self) -> &mut f32;
    fn max_effect_distance(&self) -> &f32;
    fn max_effect_distance_mut(&mut self) -> &mut f32;
    fn max_active_bones(&self) -> &u32;
    fn max_active_bones_mut(&mut self) -> &mut u32;
    fn max_wiggle_distance(&self) -> &f32;
    fn max_wiggle_distance_mut(&mut self) -> &mut f32;
    fn dissolve_enable(&self) -> &bool;
    fn dissolve_enable_mut(&mut self) -> &mut bool;
    fn simulation_ease_out_time(&self) -> &f32;
    fn simulation_ease_out_time_mut(&mut self) -> &mut f32;
    fn enable(&self) -> &bool;
    fn enable_mut(&mut self) -> &mut bool;
    fn time_scale(&self) -> &f32;
    fn time_scale_mut(&mut self) -> &mut f32;
    fn enable_jobs(&self) -> &bool;
    fn enable_jobs_mut(&mut self) -> &mut bool;
    fn job_count(&self) -> &u32;
    fn job_count_mut(&mut self) -> &mut u32;
    fn draw_nodes(&self) -> &bool;
    fn draw_nodes_mut(&mut self) -> &mut bool;
    fn draw_node_i_ds(&self) -> &bool;
    fn draw_node_i_ds_mut(&mut self) -> &mut bool;
    fn draw_node_stiffness(&self) -> &bool;
    fn draw_node_stiffness_mut(&mut self) -> &mut bool;
    fn draw_non_simulated_as_rigid(&self) -> &bool;
    fn draw_non_simulated_as_rigid_mut(&mut self) -> &mut bool;
    fn draw_active_instance_boxes_enable(&self) -> &bool;
    fn draw_active_instance_boxes_enable_mut(&mut self) -> &mut bool;
    fn draw_effect_debug_info_enable(&self) -> &bool;
    fn draw_effect_debug_info_enable_mut(&mut self) -> &mut bool;
    fn draw_damage_debug_info_enable(&self) -> &bool;
    fn draw_damage_debug_info_enable_mut(&mut self) -> &mut bool;
    fn draw_stats_enable(&self) -> &bool;
    fn draw_stats_enable_mut(&mut self) -> &mut bool;
    fn procedural_animation_enable(&self) -> &bool;
    fn procedural_animation_enable_mut(&mut self) -> &mut bool;
}

impl VegetationSystemSettingsTrait for VegetationSystemSettings {
    fn destruction_enable(&self) -> &bool {
        &self.destruction_enable
    }
    fn destruction_enable_mut(&mut self) -> &mut bool {
        &mut self.destruction_enable
    }
    fn max_active_distance(&self) -> &f32 {
        &self.max_active_distance
    }
    fn max_active_distance_mut(&mut self) -> &mut f32 {
        &mut self.max_active_distance
    }
    fn max_effect_distance(&self) -> &f32 {
        &self.max_effect_distance
    }
    fn max_effect_distance_mut(&mut self) -> &mut f32 {
        &mut self.max_effect_distance
    }
    fn max_active_bones(&self) -> &u32 {
        &self.max_active_bones
    }
    fn max_active_bones_mut(&mut self) -> &mut u32 {
        &mut self.max_active_bones
    }
    fn max_wiggle_distance(&self) -> &f32 {
        &self.max_wiggle_distance
    }
    fn max_wiggle_distance_mut(&mut self) -> &mut f32 {
        &mut self.max_wiggle_distance
    }
    fn dissolve_enable(&self) -> &bool {
        &self.dissolve_enable
    }
    fn dissolve_enable_mut(&mut self) -> &mut bool {
        &mut self.dissolve_enable
    }
    fn simulation_ease_out_time(&self) -> &f32 {
        &self.simulation_ease_out_time
    }
    fn simulation_ease_out_time_mut(&mut self) -> &mut f32 {
        &mut self.simulation_ease_out_time
    }
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
    fn job_count(&self) -> &u32 {
        &self.job_count
    }
    fn job_count_mut(&mut self) -> &mut u32 {
        &mut self.job_count
    }
    fn draw_nodes(&self) -> &bool {
        &self.draw_nodes
    }
    fn draw_nodes_mut(&mut self) -> &mut bool {
        &mut self.draw_nodes
    }
    fn draw_node_i_ds(&self) -> &bool {
        &self.draw_node_i_ds
    }
    fn draw_node_i_ds_mut(&mut self) -> &mut bool {
        &mut self.draw_node_i_ds
    }
    fn draw_node_stiffness(&self) -> &bool {
        &self.draw_node_stiffness
    }
    fn draw_node_stiffness_mut(&mut self) -> &mut bool {
        &mut self.draw_node_stiffness
    }
    fn draw_non_simulated_as_rigid(&self) -> &bool {
        &self.draw_non_simulated_as_rigid
    }
    fn draw_non_simulated_as_rigid_mut(&mut self) -> &mut bool {
        &mut self.draw_non_simulated_as_rigid
    }
    fn draw_active_instance_boxes_enable(&self) -> &bool {
        &self.draw_active_instance_boxes_enable
    }
    fn draw_active_instance_boxes_enable_mut(&mut self) -> &mut bool {
        &mut self.draw_active_instance_boxes_enable
    }
    fn draw_effect_debug_info_enable(&self) -> &bool {
        &self.draw_effect_debug_info_enable
    }
    fn draw_effect_debug_info_enable_mut(&mut self) -> &mut bool {
        &mut self.draw_effect_debug_info_enable
    }
    fn draw_damage_debug_info_enable(&self) -> &bool {
        &self.draw_damage_debug_info_enable
    }
    fn draw_damage_debug_info_enable_mut(&mut self) -> &mut bool {
        &mut self.draw_damage_debug_info_enable
    }
    fn draw_stats_enable(&self) -> &bool {
        &self.draw_stats_enable
    }
    fn draw_stats_enable_mut(&mut self) -> &mut bool {
        &mut self.draw_stats_enable
    }
    fn procedural_animation_enable(&self) -> &bool {
        &self.procedural_animation_enable
    }
    fn procedural_animation_enable_mut(&mut self) -> &mut bool {
        &mut self.procedural_animation_enable
    }
}

impl super::core::DataContainerTrait for VegetationSystemSettings {
}

pub static VEGETATIONSYSTEMSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationSystemSettings",
    name_hash: 1040614397,
    flags: MemberInfoFlags::new(101),
    module: "Vegetation",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(VegetationSystemSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VegetationSystemSettings as Default>::default())),
            create_boxed: || Box::new(<VegetationSystemSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "DestructionEnable",
                name_hash: 3898235578,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VegetationSystemSettings, destruction_enable),
            },
            FieldInfoData {
                name: "MaxActiveDistance",
                name_hash: 2638833886,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationSystemSettings, max_active_distance),
            },
            FieldInfoData {
                name: "MaxEffectDistance",
                name_hash: 309741285,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationSystemSettings, max_effect_distance),
            },
            FieldInfoData {
                name: "MaxActiveBones",
                name_hash: 2559067432,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VegetationSystemSettings, max_active_bones),
            },
            FieldInfoData {
                name: "MaxWiggleDistance",
                name_hash: 4138955877,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationSystemSettings, max_wiggle_distance),
            },
            FieldInfoData {
                name: "DissolveEnable",
                name_hash: 84402905,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VegetationSystemSettings, dissolve_enable),
            },
            FieldInfoData {
                name: "SimulationEaseOutTime",
                name_hash: 3187152671,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationSystemSettings, simulation_ease_out_time),
            },
            FieldInfoData {
                name: "Enable",
                name_hash: 2342790116,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VegetationSystemSettings, enable),
            },
            FieldInfoData {
                name: "TimeScale",
                name_hash: 169511528,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationSystemSettings, time_scale),
            },
            FieldInfoData {
                name: "EnableJobs",
                name_hash: 1190923856,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VegetationSystemSettings, enable_jobs),
            },
            FieldInfoData {
                name: "JobCount",
                name_hash: 4166996065,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VegetationSystemSettings, job_count),
            },
            FieldInfoData {
                name: "DrawNodes",
                name_hash: 2398188982,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VegetationSystemSettings, draw_nodes),
            },
            FieldInfoData {
                name: "DrawNodeIDs",
                name_hash: 287641659,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VegetationSystemSettings, draw_node_i_ds),
            },
            FieldInfoData {
                name: "DrawNodeStiffness",
                name_hash: 285040416,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VegetationSystemSettings, draw_node_stiffness),
            },
            FieldInfoData {
                name: "DrawNonSimulatedAsRigid",
                name_hash: 1584906195,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VegetationSystemSettings, draw_non_simulated_as_rigid),
            },
            FieldInfoData {
                name: "DrawActiveInstanceBoxesEnable",
                name_hash: 3544041922,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VegetationSystemSettings, draw_active_instance_boxes_enable),
            },
            FieldInfoData {
                name: "DrawEffectDebugInfoEnable",
                name_hash: 3500128844,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VegetationSystemSettings, draw_effect_debug_info_enable),
            },
            FieldInfoData {
                name: "DrawDamageDebugInfoEnable",
                name_hash: 48494608,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VegetationSystemSettings, draw_damage_debug_info_enable),
            },
            FieldInfoData {
                name: "DrawStatsEnable",
                name_hash: 711726149,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VegetationSystemSettings, draw_stats_enable),
            },
            FieldInfoData {
                name: "ProceduralAnimationEnable",
                name_hash: 1764606103,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VegetationSystemSettings, procedural_animation_enable),
            },
        ],
    }),
    array_type: Some(VEGETATIONSYSTEMSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VegetationSystemSettings {
    fn type_info(&self) -> &'static TypeInfo {
        VEGETATIONSYSTEMSETTINGS_TYPE_INFO
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


pub static VEGETATIONSYSTEMSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationSystemSettings-Array",
    name_hash: 2080485577,
    flags: MemberInfoFlags::new(145),
    module: "Vegetation",
    data: TypeInfoData::Array("VegetationSystemSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct VegetationTreeBreakNodeState {
    pub break_node_bit: bool,
    pub trans: super::core::Vec3,
    pub rotation: super::core::Quat,
}

pub trait VegetationTreeBreakNodeStateTrait: TypeObject {
    fn break_node_bit(&self) -> &bool;
    fn break_node_bit_mut(&mut self) -> &mut bool;
    fn trans(&self) -> &super::core::Vec3;
    fn trans_mut(&mut self) -> &mut super::core::Vec3;
    fn rotation(&self) -> &super::core::Quat;
    fn rotation_mut(&mut self) -> &mut super::core::Quat;
}

impl VegetationTreeBreakNodeStateTrait for VegetationTreeBreakNodeState {
    fn break_node_bit(&self) -> &bool {
        &self.break_node_bit
    }
    fn break_node_bit_mut(&mut self) -> &mut bool {
        &mut self.break_node_bit
    }
    fn trans(&self) -> &super::core::Vec3 {
        &self.trans
    }
    fn trans_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.trans
    }
    fn rotation(&self) -> &super::core::Quat {
        &self.rotation
    }
    fn rotation_mut(&mut self) -> &mut super::core::Quat {
        &mut self.rotation
    }
}

pub static VEGETATIONTREEBREAKNODESTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationTreeBreakNodeState",
    name_hash: 2900498387,
    flags: MemberInfoFlags::new(36937),
    module: "Vegetation",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VegetationTreeBreakNodeState as Default>::default())),
            create_boxed: || Box::new(<VegetationTreeBreakNodeState as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "BreakNodeBit",
                name_hash: 2635024069,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VegetationTreeBreakNodeState, break_node_bit),
            },
            FieldInfoData {
                name: "Trans",
                name_hash: 227190399,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(VegetationTreeBreakNodeState, trans),
            },
            FieldInfoData {
                name: "Rotation",
                name_hash: 48673745,
                flags: MemberInfoFlags::new(0),
                field_type: "Quat",
                rust_offset: offset_of!(VegetationTreeBreakNodeState, rotation),
            },
        ],
    }),
    array_type: Some(VEGETATIONTREEBREAKNODESTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for VegetationTreeBreakNodeState {
    fn type_info(&self) -> &'static TypeInfo {
        VEGETATIONTREEBREAKNODESTATE_TYPE_INFO
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


pub static VEGETATIONTREEBREAKNODESTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationTreeBreakNodeState-Array",
    name_hash: 107569895,
    flags: MemberInfoFlags::new(145),
    module: "Vegetation",
    data: TypeInfoData::Array("VegetationTreeBreakNodeState"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct VegetationTreeBreakNodeDestruction {
    pub break_node_bit: bool,
    pub height: f32,
    pub part_index: u32,
}

pub trait VegetationTreeBreakNodeDestructionTrait: TypeObject {
    fn break_node_bit(&self) -> &bool;
    fn break_node_bit_mut(&mut self) -> &mut bool;
    fn height(&self) -> &f32;
    fn height_mut(&mut self) -> &mut f32;
    fn part_index(&self) -> &u32;
    fn part_index_mut(&mut self) -> &mut u32;
}

impl VegetationTreeBreakNodeDestructionTrait for VegetationTreeBreakNodeDestruction {
    fn break_node_bit(&self) -> &bool {
        &self.break_node_bit
    }
    fn break_node_bit_mut(&mut self) -> &mut bool {
        &mut self.break_node_bit
    }
    fn height(&self) -> &f32 {
        &self.height
    }
    fn height_mut(&mut self) -> &mut f32 {
        &mut self.height
    }
    fn part_index(&self) -> &u32 {
        &self.part_index
    }
    fn part_index_mut(&mut self) -> &mut u32 {
        &mut self.part_index
    }
}

pub static VEGETATIONTREEBREAKNODEDESTRUCTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationTreeBreakNodeDestruction",
    name_hash: 2708040314,
    flags: MemberInfoFlags::new(36937),
    module: "Vegetation",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VegetationTreeBreakNodeDestruction as Default>::default())),
            create_boxed: || Box::new(<VegetationTreeBreakNodeDestruction as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "BreakNodeBit",
                name_hash: 2635024069,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VegetationTreeBreakNodeDestruction, break_node_bit),
            },
            FieldInfoData {
                name: "Height",
                name_hash: 3054065626,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeBreakNodeDestruction, height),
            },
            FieldInfoData {
                name: "PartIndex",
                name_hash: 3213901068,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VegetationTreeBreakNodeDestruction, part_index),
            },
        ],
    }),
    array_type: Some(VEGETATIONTREEBREAKNODEDESTRUCTION_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for VegetationTreeBreakNodeDestruction {
    fn type_info(&self) -> &'static TypeInfo {
        VEGETATIONTREEBREAKNODEDESTRUCTION_TYPE_INFO
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


pub static VEGETATIONTREEBREAKNODEDESTRUCTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationTreeBreakNodeDestruction-Array",
    name_hash: 1108901198,
    flags: MemberInfoFlags::new(145),
    module: "Vegetation",
    data: TypeInfoData::Array("VegetationTreeBreakNodeDestruction"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct VegetationTreeEntityData {
    pub _glacier_base: VegetationBaseEntityData,
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
    pub wind_effect: Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>,
    pub min_respawn_time: f32,
    pub stem_effect_node_threshold: u32,
    pub shadow_l_o_d_offset: u32,
    pub translucency_enabled: bool,
    pub translucency_volume_center: super::core::Vec3,
}

pub trait VegetationTreeEntityDataTrait: VegetationBaseEntityDataTrait {
    fn stiffness(&self) -> &f32;
    fn stiffness_mut(&mut self) -> &mut f32;
    fn damping(&self) -> &f32;
    fn damping_mut(&mut self) -> &mut f32;
    fn stem_mass(&self) -> &f32;
    fn stem_mass_mut(&mut self) -> &mut f32;
    fn stiffness_spread(&self) -> &f32;
    fn stiffness_spread_mut(&mut self) -> &mut f32;
    fn damping_spread(&self) -> &f32;
    fn damping_spread_mut(&mut self) -> &mut f32;
    fn mass_spread(&self) -> &f32;
    fn mass_spread_mut(&mut self) -> &mut f32;
    fn stem_locked_up_to(&self) -> &f32;
    fn stem_locked_up_to_mut(&mut self) -> &mut f32;
    fn stem_bone_count(&self) -> &i32;
    fn stem_bone_count_mut(&mut self) -> &mut i32;
    fn breakable_joint_threshold(&self) -> &f32;
    fn breakable_joint_threshold_mut(&mut self) -> &mut f32;
    fn constant_falloff(&self) -> &bool;
    fn constant_falloff_mut(&mut self) -> &mut bool;
    fn bounding_box_scale_factor(&self) -> &f32;
    fn bounding_box_scale_factor_mut(&mut self) -> &mut f32;
    fn indestructable(&self) -> &bool;
    fn indestructable_mut(&mut self) -> &mut bool;
    fn parts_time_to_live(&self) -> &f32;
    fn parts_time_to_live_mut(&mut self) -> &mut f32;
    fn linear_velocity_damping(&self) -> &f32;
    fn linear_velocity_damping_mut(&mut self) -> &mut f32;
    fn angular_velocity_damping(&self) -> &f32;
    fn angular_velocity_damping_mut(&mut self) -> &mut f32;
    fn friction(&self) -> &f32;
    fn friction_mut(&mut self) -> &mut f32;
    fn restitution(&self) -> &f32;
    fn restitution_mut(&mut self) -> &mut f32;
    fn stem_physics_width(&self) -> &f32;
    fn stem_physics_width_mut(&mut self) -> &mut f32;
    fn stem_physics_height_scale(&self) -> &f32;
    fn stem_physics_height_scale_mut(&mut self) -> &mut f32;
    fn branch_physics_width(&self) -> &f32;
    fn branch_physics_width_mut(&mut self) -> &mut f32;
    fn branch_physics_height_scale(&self) -> &f32;
    fn branch_physics_height_scale_mut(&mut self) -> &mut f32;
    fn destruction_mass_scale(&self) -> &f32;
    fn destruction_mass_scale_mut(&mut self) -> &mut f32;
    fn inertia_modifier(&self) -> &super::core::Vec3;
    fn inertia_modifier_mut(&mut self) -> &mut super::core::Vec3;
    fn center_of_mass_vertical_scale(&self) -> &f32;
    fn center_of_mass_vertical_scale_mut(&mut self) -> &mut f32;
    fn stem_break_effect(&self) -> &VegetationEffectSlot;
    fn stem_break_effect_mut(&mut self) -> &mut VegetationEffectSlot;
    fn branch_break_effect(&self) -> &VegetationEffectSlot;
    fn branch_break_effect_mut(&mut self) -> &mut VegetationEffectSlot;
    fn impact_effect(&self) -> &VegetationEffectSlot;
    fn impact_effect_mut(&mut self) -> &mut VegetationEffectSlot;
    fn wind_effect(&self) -> &Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>;
    fn wind_effect_mut(&mut self) -> &mut Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>;
    fn min_respawn_time(&self) -> &f32;
    fn min_respawn_time_mut(&mut self) -> &mut f32;
    fn stem_effect_node_threshold(&self) -> &u32;
    fn stem_effect_node_threshold_mut(&mut self) -> &mut u32;
    fn shadow_l_o_d_offset(&self) -> &u32;
    fn shadow_l_o_d_offset_mut(&mut self) -> &mut u32;
    fn translucency_enabled(&self) -> &bool;
    fn translucency_enabled_mut(&mut self) -> &mut bool;
    fn translucency_volume_center(&self) -> &super::core::Vec3;
    fn translucency_volume_center_mut(&mut self) -> &mut super::core::Vec3;
}

impl VegetationTreeEntityDataTrait for VegetationTreeEntityData {
    fn stiffness(&self) -> &f32 {
        &self.stiffness
    }
    fn stiffness_mut(&mut self) -> &mut f32 {
        &mut self.stiffness
    }
    fn damping(&self) -> &f32 {
        &self.damping
    }
    fn damping_mut(&mut self) -> &mut f32 {
        &mut self.damping
    }
    fn stem_mass(&self) -> &f32 {
        &self.stem_mass
    }
    fn stem_mass_mut(&mut self) -> &mut f32 {
        &mut self.stem_mass
    }
    fn stiffness_spread(&self) -> &f32 {
        &self.stiffness_spread
    }
    fn stiffness_spread_mut(&mut self) -> &mut f32 {
        &mut self.stiffness_spread
    }
    fn damping_spread(&self) -> &f32 {
        &self.damping_spread
    }
    fn damping_spread_mut(&mut self) -> &mut f32 {
        &mut self.damping_spread
    }
    fn mass_spread(&self) -> &f32 {
        &self.mass_spread
    }
    fn mass_spread_mut(&mut self) -> &mut f32 {
        &mut self.mass_spread
    }
    fn stem_locked_up_to(&self) -> &f32 {
        &self.stem_locked_up_to
    }
    fn stem_locked_up_to_mut(&mut self) -> &mut f32 {
        &mut self.stem_locked_up_to
    }
    fn stem_bone_count(&self) -> &i32 {
        &self.stem_bone_count
    }
    fn stem_bone_count_mut(&mut self) -> &mut i32 {
        &mut self.stem_bone_count
    }
    fn breakable_joint_threshold(&self) -> &f32 {
        &self.breakable_joint_threshold
    }
    fn breakable_joint_threshold_mut(&mut self) -> &mut f32 {
        &mut self.breakable_joint_threshold
    }
    fn constant_falloff(&self) -> &bool {
        &self.constant_falloff
    }
    fn constant_falloff_mut(&mut self) -> &mut bool {
        &mut self.constant_falloff
    }
    fn bounding_box_scale_factor(&self) -> &f32 {
        &self.bounding_box_scale_factor
    }
    fn bounding_box_scale_factor_mut(&mut self) -> &mut f32 {
        &mut self.bounding_box_scale_factor
    }
    fn indestructable(&self) -> &bool {
        &self.indestructable
    }
    fn indestructable_mut(&mut self) -> &mut bool {
        &mut self.indestructable
    }
    fn parts_time_to_live(&self) -> &f32 {
        &self.parts_time_to_live
    }
    fn parts_time_to_live_mut(&mut self) -> &mut f32 {
        &mut self.parts_time_to_live
    }
    fn linear_velocity_damping(&self) -> &f32 {
        &self.linear_velocity_damping
    }
    fn linear_velocity_damping_mut(&mut self) -> &mut f32 {
        &mut self.linear_velocity_damping
    }
    fn angular_velocity_damping(&self) -> &f32 {
        &self.angular_velocity_damping
    }
    fn angular_velocity_damping_mut(&mut self) -> &mut f32 {
        &mut self.angular_velocity_damping
    }
    fn friction(&self) -> &f32 {
        &self.friction
    }
    fn friction_mut(&mut self) -> &mut f32 {
        &mut self.friction
    }
    fn restitution(&self) -> &f32 {
        &self.restitution
    }
    fn restitution_mut(&mut self) -> &mut f32 {
        &mut self.restitution
    }
    fn stem_physics_width(&self) -> &f32 {
        &self.stem_physics_width
    }
    fn stem_physics_width_mut(&mut self) -> &mut f32 {
        &mut self.stem_physics_width
    }
    fn stem_physics_height_scale(&self) -> &f32 {
        &self.stem_physics_height_scale
    }
    fn stem_physics_height_scale_mut(&mut self) -> &mut f32 {
        &mut self.stem_physics_height_scale
    }
    fn branch_physics_width(&self) -> &f32 {
        &self.branch_physics_width
    }
    fn branch_physics_width_mut(&mut self) -> &mut f32 {
        &mut self.branch_physics_width
    }
    fn branch_physics_height_scale(&self) -> &f32 {
        &self.branch_physics_height_scale
    }
    fn branch_physics_height_scale_mut(&mut self) -> &mut f32 {
        &mut self.branch_physics_height_scale
    }
    fn destruction_mass_scale(&self) -> &f32 {
        &self.destruction_mass_scale
    }
    fn destruction_mass_scale_mut(&mut self) -> &mut f32 {
        &mut self.destruction_mass_scale
    }
    fn inertia_modifier(&self) -> &super::core::Vec3 {
        &self.inertia_modifier
    }
    fn inertia_modifier_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.inertia_modifier
    }
    fn center_of_mass_vertical_scale(&self) -> &f32 {
        &self.center_of_mass_vertical_scale
    }
    fn center_of_mass_vertical_scale_mut(&mut self) -> &mut f32 {
        &mut self.center_of_mass_vertical_scale
    }
    fn stem_break_effect(&self) -> &VegetationEffectSlot {
        &self.stem_break_effect
    }
    fn stem_break_effect_mut(&mut self) -> &mut VegetationEffectSlot {
        &mut self.stem_break_effect
    }
    fn branch_break_effect(&self) -> &VegetationEffectSlot {
        &self.branch_break_effect
    }
    fn branch_break_effect_mut(&mut self) -> &mut VegetationEffectSlot {
        &mut self.branch_break_effect
    }
    fn impact_effect(&self) -> &VegetationEffectSlot {
        &self.impact_effect
    }
    fn impact_effect_mut(&mut self) -> &mut VegetationEffectSlot {
        &mut self.impact_effect
    }
    fn wind_effect(&self) -> &Option<LockedTypeObject /* super::effect_base::EffectBlueprint */> {
        &self.wind_effect
    }
    fn wind_effect_mut(&mut self) -> &mut Option<LockedTypeObject /* super::effect_base::EffectBlueprint */> {
        &mut self.wind_effect
    }
    fn min_respawn_time(&self) -> &f32 {
        &self.min_respawn_time
    }
    fn min_respawn_time_mut(&mut self) -> &mut f32 {
        &mut self.min_respawn_time
    }
    fn stem_effect_node_threshold(&self) -> &u32 {
        &self.stem_effect_node_threshold
    }
    fn stem_effect_node_threshold_mut(&mut self) -> &mut u32 {
        &mut self.stem_effect_node_threshold
    }
    fn shadow_l_o_d_offset(&self) -> &u32 {
        &self.shadow_l_o_d_offset
    }
    fn shadow_l_o_d_offset_mut(&mut self) -> &mut u32 {
        &mut self.shadow_l_o_d_offset
    }
    fn translucency_enabled(&self) -> &bool {
        &self.translucency_enabled
    }
    fn translucency_enabled_mut(&mut self) -> &mut bool {
        &mut self.translucency_enabled
    }
    fn translucency_volume_center(&self) -> &super::core::Vec3 {
        &self.translucency_volume_center
    }
    fn translucency_volume_center_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.translucency_volume_center
    }
}

impl VegetationBaseEntityDataTrait for VegetationTreeEntityData {
    fn base_pose_transforms(&self) -> &super::core::SparseTransformArray {
        self._glacier_base.base_pose_transforms()
    }
    fn base_pose_transforms_mut(&mut self) -> &mut super::core::SparseTransformArray {
        self._glacier_base.base_pose_transforms_mut()
    }
    fn hierarchy(&self) -> &Vec<i32> {
        self._glacier_base.hierarchy()
    }
    fn hierarchy_mut(&mut self) -> &mut Vec<i32> {
        self._glacier_base.hierarchy_mut()
    }
    fn part_indirection(&self) -> &Vec<i32> {
        self._glacier_base.part_indirection()
    }
    fn part_indirection_mut(&mut self) -> &mut Vec<i32> {
        self._glacier_base.part_indirection_mut()
    }
    fn part_hierarchy(&self) -> &Vec<i32> {
        self._glacier_base.part_hierarchy()
    }
    fn part_hierarchy_mut(&mut self) -> &mut Vec<i32> {
        self._glacier_base.part_hierarchy_mut()
    }
    fn part_initial_healths(&self) -> &Vec<f32> {
        self._glacier_base.part_initial_healths()
    }
    fn part_initial_healths_mut(&mut self) -> &mut Vec<f32> {
        self._glacier_base.part_initial_healths_mut()
    }
    fn bone_is_stem(&self) -> &Vec<bool> {
        self._glacier_base.bone_is_stem()
    }
    fn bone_is_stem_mut(&mut self) -> &mut Vec<bool> {
        self._glacier_base.bone_is_stem_mut()
    }
    fn mesh(&self) -> &Option<LockedTypeObject /* super::render_base::MeshBaseAsset */> {
        self._glacier_base.mesh()
    }
    fn mesh_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::MeshBaseAsset */> {
        self._glacier_base.mesh_mut()
    }
    fn rigid_bodies(&self) -> &Vec<Option<LockedTypeObject /* super::physics::RigidBodyData */>> {
        self._glacier_base.rigid_bodies()
    }
    fn rigid_bodies_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::physics::RigidBodyData */>> {
        self._glacier_base.rigid_bodies_mut()
    }
}

impl super::entity::GameComponentEntityDataTrait for VegetationTreeEntityData {
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::entity::ComponentEntityDataTrait for VegetationTreeEntityData {
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

impl super::entity::SpatialEntityDataTrait for VegetationTreeEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::entity::EntityDataTrait for VegetationTreeEntityData {
}

impl super::entity::GameObjectDataTrait for VegetationTreeEntityData {
}

impl super::core::DataBusPeerTrait for VegetationTreeEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for VegetationTreeEntityData {
}

impl super::core::DataContainerTrait for VegetationTreeEntityData {
}

pub static VEGETATIONTREEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationTreeEntityData",
    name_hash: 2102677968,
    flags: MemberInfoFlags::new(101),
    module: "Vegetation",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VEGETATIONBASEENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(VegetationTreeEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VegetationTreeEntityData as Default>::default())),
            create_boxed: || Box::new(<VegetationTreeEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Stiffness",
                name_hash: 721813632,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, stiffness),
            },
            FieldInfoData {
                name: "Damping",
                name_hash: 3862601053,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, damping),
            },
            FieldInfoData {
                name: "StemMass",
                name_hash: 3249249158,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, stem_mass),
            },
            FieldInfoData {
                name: "StiffnessSpread",
                name_hash: 393858929,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, stiffness_spread),
            },
            FieldInfoData {
                name: "DampingSpread",
                name_hash: 3654959468,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, damping_spread),
            },
            FieldInfoData {
                name: "MassSpread",
                name_hash: 3335040984,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, mass_spread),
            },
            FieldInfoData {
                name: "StemLockedUpTo",
                name_hash: 3527237886,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, stem_locked_up_to),
            },
            FieldInfoData {
                name: "StemBoneCount",
                name_hash: 4140582895,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(VegetationTreeEntityData, stem_bone_count),
            },
            FieldInfoData {
                name: "BreakableJointThreshold",
                name_hash: 2951351665,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, breakable_joint_threshold),
            },
            FieldInfoData {
                name: "ConstantFalloff",
                name_hash: 918730195,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VegetationTreeEntityData, constant_falloff),
            },
            FieldInfoData {
                name: "BoundingBoxScaleFactor",
                name_hash: 3835398359,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, bounding_box_scale_factor),
            },
            FieldInfoData {
                name: "Indestructable",
                name_hash: 464130430,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VegetationTreeEntityData, indestructable),
            },
            FieldInfoData {
                name: "PartsTimeToLive",
                name_hash: 209652569,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, parts_time_to_live),
            },
            FieldInfoData {
                name: "LinearVelocityDamping",
                name_hash: 1004384727,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, linear_velocity_damping),
            },
            FieldInfoData {
                name: "AngularVelocityDamping",
                name_hash: 2367237320,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, angular_velocity_damping),
            },
            FieldInfoData {
                name: "Friction",
                name_hash: 306207591,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, friction),
            },
            FieldInfoData {
                name: "Restitution",
                name_hash: 2298929185,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, restitution),
            },
            FieldInfoData {
                name: "StemPhysicsWidth",
                name_hash: 3773105703,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, stem_physics_width),
            },
            FieldInfoData {
                name: "StemPhysicsHeightScale",
                name_hash: 968139206,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, stem_physics_height_scale),
            },
            FieldInfoData {
                name: "BranchPhysicsWidth",
                name_hash: 2028462780,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, branch_physics_width),
            },
            FieldInfoData {
                name: "BranchPhysicsHeightScale",
                name_hash: 3596469085,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, branch_physics_height_scale),
            },
            FieldInfoData {
                name: "DestructionMassScale",
                name_hash: 101589487,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, destruction_mass_scale),
            },
            FieldInfoData {
                name: "InertiaModifier",
                name_hash: 3532865534,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(VegetationTreeEntityData, inertia_modifier),
            },
            FieldInfoData {
                name: "CenterOfMassVerticalScale",
                name_hash: 2894567649,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, center_of_mass_vertical_scale),
            },
            FieldInfoData {
                name: "StemBreakEffect",
                name_hash: 2818645282,
                flags: MemberInfoFlags::new(0),
                field_type: "VegetationEffectSlot",
                rust_offset: offset_of!(VegetationTreeEntityData, stem_break_effect),
            },
            FieldInfoData {
                name: "BranchBreakEffect",
                name_hash: 1809843993,
                flags: MemberInfoFlags::new(0),
                field_type: "VegetationEffectSlot",
                rust_offset: offset_of!(VegetationTreeEntityData, branch_break_effect),
            },
            FieldInfoData {
                name: "ImpactEffect",
                name_hash: 2147535120,
                flags: MemberInfoFlags::new(0),
                field_type: "VegetationEffectSlot",
                rust_offset: offset_of!(VegetationTreeEntityData, impact_effect),
            },
            FieldInfoData {
                name: "WindEffect",
                name_hash: 2533736230,
                flags: MemberInfoFlags::new(0),
                field_type: "EffectBlueprint",
                rust_offset: offset_of!(VegetationTreeEntityData, wind_effect),
            },
            FieldInfoData {
                name: "MinRespawnTime",
                name_hash: 2022883894,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, min_respawn_time),
            },
            FieldInfoData {
                name: "StemEffectNodeThreshold",
                name_hash: 2900269930,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VegetationTreeEntityData, stem_effect_node_threshold),
            },
            FieldInfoData {
                name: "ShadowLODOffset",
                name_hash: 1990024297,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VegetationTreeEntityData, shadow_l_o_d_offset),
            },
            FieldInfoData {
                name: "TranslucencyEnabled",
                name_hash: 1802794289,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VegetationTreeEntityData, translucency_enabled),
            },
            FieldInfoData {
                name: "TranslucencyVolumeCenter",
                name_hash: 1131296183,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(VegetationTreeEntityData, translucency_volume_center),
            },
        ],
    }),
    array_type: Some(VEGETATIONTREEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for VegetationTreeEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        VEGETATIONTREEENTITYDATA_TYPE_INFO
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


pub static VEGETATIONTREEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationTreeEntityData-Array",
    name_hash: 1378881636,
    flags: MemberInfoFlags::new(145),
    module: "Vegetation",
    data: TypeInfoData::Array("VegetationTreeEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct VegetationEffectSlot {
    pub effect: Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>,
    pub strength_min: f32,
    pub strength_max: f32,
    pub size_min: f32,
    pub size_max: f32,
}

pub trait VegetationEffectSlotTrait: TypeObject {
    fn effect(&self) -> &Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>;
    fn effect_mut(&mut self) -> &mut Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>;
    fn strength_min(&self) -> &f32;
    fn strength_min_mut(&mut self) -> &mut f32;
    fn strength_max(&self) -> &f32;
    fn strength_max_mut(&mut self) -> &mut f32;
    fn size_min(&self) -> &f32;
    fn size_min_mut(&mut self) -> &mut f32;
    fn size_max(&self) -> &f32;
    fn size_max_mut(&mut self) -> &mut f32;
}

impl VegetationEffectSlotTrait for VegetationEffectSlot {
    fn effect(&self) -> &Option<LockedTypeObject /* super::effect_base::EffectBlueprint */> {
        &self.effect
    }
    fn effect_mut(&mut self) -> &mut Option<LockedTypeObject /* super::effect_base::EffectBlueprint */> {
        &mut self.effect
    }
    fn strength_min(&self) -> &f32 {
        &self.strength_min
    }
    fn strength_min_mut(&mut self) -> &mut f32 {
        &mut self.strength_min
    }
    fn strength_max(&self) -> &f32 {
        &self.strength_max
    }
    fn strength_max_mut(&mut self) -> &mut f32 {
        &mut self.strength_max
    }
    fn size_min(&self) -> &f32 {
        &self.size_min
    }
    fn size_min_mut(&mut self) -> &mut f32 {
        &mut self.size_min
    }
    fn size_max(&self) -> &f32 {
        &self.size_max
    }
    fn size_max_mut(&mut self) -> &mut f32 {
        &mut self.size_max
    }
}

pub static VEGETATIONEFFECTSLOT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationEffectSlot",
    name_hash: 2732371950,
    flags: MemberInfoFlags::new(73),
    module: "Vegetation",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VegetationEffectSlot as Default>::default())),
            create_boxed: || Box::new(<VegetationEffectSlot as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Effect",
                name_hash: 2332983090,
                flags: MemberInfoFlags::new(0),
                field_type: "EffectBlueprint",
                rust_offset: offset_of!(VegetationEffectSlot, effect),
            },
            FieldInfoData {
                name: "StrengthMin",
                name_hash: 382626730,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationEffectSlot, strength_min),
            },
            FieldInfoData {
                name: "StrengthMax",
                name_hash: 382626996,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationEffectSlot, strength_max),
            },
            FieldInfoData {
                name: "SizeMin",
                name_hash: 3200697194,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationEffectSlot, size_min),
            },
            FieldInfoData {
                name: "SizeMax",
                name_hash: 3200697460,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationEffectSlot, size_max),
            },
        ],
    }),
    array_type: Some(VEGETATIONEFFECTSLOT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VegetationEffectSlot {
    fn type_info(&self) -> &'static TypeInfo {
        VEGETATIONEFFECTSLOT_TYPE_INFO
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


pub static VEGETATIONEFFECTSLOT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationEffectSlot-Array",
    name_hash: 2398537946,
    flags: MemberInfoFlags::new(145),
    module: "Vegetation",
    data: TypeInfoData::Array("VegetationEffectSlot"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct VegetationBaseEntityData {
    pub _glacier_base: super::entity::GameComponentEntityData,
    pub base_pose_transforms: super::core::SparseTransformArray,
    pub hierarchy: Vec<i32>,
    pub part_indirection: Vec<i32>,
    pub part_hierarchy: Vec<i32>,
    pub part_initial_healths: Vec<f32>,
    pub bone_is_stem: Vec<bool>,
    pub mesh: Option<LockedTypeObject /* super::render_base::MeshBaseAsset */>,
    pub rigid_bodies: Vec<Option<LockedTypeObject /* super::physics::RigidBodyData */>>,
}

pub trait VegetationBaseEntityDataTrait: super::entity::GameComponentEntityDataTrait {
    fn base_pose_transforms(&self) -> &super::core::SparseTransformArray;
    fn base_pose_transforms_mut(&mut self) -> &mut super::core::SparseTransformArray;
    fn hierarchy(&self) -> &Vec<i32>;
    fn hierarchy_mut(&mut self) -> &mut Vec<i32>;
    fn part_indirection(&self) -> &Vec<i32>;
    fn part_indirection_mut(&mut self) -> &mut Vec<i32>;
    fn part_hierarchy(&self) -> &Vec<i32>;
    fn part_hierarchy_mut(&mut self) -> &mut Vec<i32>;
    fn part_initial_healths(&self) -> &Vec<f32>;
    fn part_initial_healths_mut(&mut self) -> &mut Vec<f32>;
    fn bone_is_stem(&self) -> &Vec<bool>;
    fn bone_is_stem_mut(&mut self) -> &mut Vec<bool>;
    fn mesh(&self) -> &Option<LockedTypeObject /* super::render_base::MeshBaseAsset */>;
    fn mesh_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::MeshBaseAsset */>;
    fn rigid_bodies(&self) -> &Vec<Option<LockedTypeObject /* super::physics::RigidBodyData */>>;
    fn rigid_bodies_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::physics::RigidBodyData */>>;
}

impl VegetationBaseEntityDataTrait for VegetationBaseEntityData {
    fn base_pose_transforms(&self) -> &super::core::SparseTransformArray {
        &self.base_pose_transforms
    }
    fn base_pose_transforms_mut(&mut self) -> &mut super::core::SparseTransformArray {
        &mut self.base_pose_transforms
    }
    fn hierarchy(&self) -> &Vec<i32> {
        &self.hierarchy
    }
    fn hierarchy_mut(&mut self) -> &mut Vec<i32> {
        &mut self.hierarchy
    }
    fn part_indirection(&self) -> &Vec<i32> {
        &self.part_indirection
    }
    fn part_indirection_mut(&mut self) -> &mut Vec<i32> {
        &mut self.part_indirection
    }
    fn part_hierarchy(&self) -> &Vec<i32> {
        &self.part_hierarchy
    }
    fn part_hierarchy_mut(&mut self) -> &mut Vec<i32> {
        &mut self.part_hierarchy
    }
    fn part_initial_healths(&self) -> &Vec<f32> {
        &self.part_initial_healths
    }
    fn part_initial_healths_mut(&mut self) -> &mut Vec<f32> {
        &mut self.part_initial_healths
    }
    fn bone_is_stem(&self) -> &Vec<bool> {
        &self.bone_is_stem
    }
    fn bone_is_stem_mut(&mut self) -> &mut Vec<bool> {
        &mut self.bone_is_stem
    }
    fn mesh(&self) -> &Option<LockedTypeObject /* super::render_base::MeshBaseAsset */> {
        &self.mesh
    }
    fn mesh_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::MeshBaseAsset */> {
        &mut self.mesh
    }
    fn rigid_bodies(&self) -> &Vec<Option<LockedTypeObject /* super::physics::RigidBodyData */>> {
        &self.rigid_bodies
    }
    fn rigid_bodies_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::physics::RigidBodyData */>> {
        &mut self.rigid_bodies
    }
}

impl super::entity::GameComponentEntityDataTrait for VegetationBaseEntityData {
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::entity::ComponentEntityDataTrait for VegetationBaseEntityData {
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

impl super::entity::SpatialEntityDataTrait for VegetationBaseEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::entity::EntityDataTrait for VegetationBaseEntityData {
}

impl super::entity::GameObjectDataTrait for VegetationBaseEntityData {
}

impl super::core::DataBusPeerTrait for VegetationBaseEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for VegetationBaseEntityData {
}

impl super::core::DataContainerTrait for VegetationBaseEntityData {
}

pub static VEGETATIONBASEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationBaseEntityData",
    name_hash: 262092099,
    flags: MemberInfoFlags::new(101),
    module: "Vegetation",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::GAMECOMPONENTENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(VegetationBaseEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VegetationBaseEntityData as Default>::default())),
            create_boxed: || Box::new(<VegetationBaseEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "BasePoseTransforms",
                name_hash: 2949884966,
                flags: MemberInfoFlags::new(0),
                field_type: "SparseTransformArray",
                rust_offset: offset_of!(VegetationBaseEntityData, base_pose_transforms),
            },
            FieldInfoData {
                name: "Hierarchy",
                name_hash: 2981249554,
                flags: MemberInfoFlags::new(144),
                field_type: "Int32-Array",
                rust_offset: offset_of!(VegetationBaseEntityData, hierarchy),
            },
            FieldInfoData {
                name: "PartIndirection",
                name_hash: 307876816,
                flags: MemberInfoFlags::new(144),
                field_type: "Int32-Array",
                rust_offset: offset_of!(VegetationBaseEntityData, part_indirection),
            },
            FieldInfoData {
                name: "PartHierarchy",
                name_hash: 3544065669,
                flags: MemberInfoFlags::new(144),
                field_type: "Int32-Array",
                rust_offset: offset_of!(VegetationBaseEntityData, part_hierarchy),
            },
            FieldInfoData {
                name: "PartInitialHealths",
                name_hash: 1403213219,
                flags: MemberInfoFlags::new(144),
                field_type: "Float32-Array",
                rust_offset: offset_of!(VegetationBaseEntityData, part_initial_healths),
            },
            FieldInfoData {
                name: "BoneIsStem",
                name_hash: 1168072854,
                flags: MemberInfoFlags::new(144),
                field_type: "Boolean-Array",
                rust_offset: offset_of!(VegetationBaseEntityData, bone_is_stem),
            },
            FieldInfoData {
                name: "Mesh",
                name_hash: 2088783990,
                flags: MemberInfoFlags::new(0),
                field_type: "MeshBaseAsset",
                rust_offset: offset_of!(VegetationBaseEntityData, mesh),
            },
            FieldInfoData {
                name: "RigidBodies",
                name_hash: 3015855522,
                flags: MemberInfoFlags::new(144),
                field_type: "RigidBodyData-Array",
                rust_offset: offset_of!(VegetationBaseEntityData, rigid_bodies),
            },
        ],
    }),
    array_type: Some(VEGETATIONBASEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for VegetationBaseEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        VEGETATIONBASEENTITYDATA_TYPE_INFO
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


pub static VEGETATIONBASEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationBaseEntityData-Array",
    name_hash: 3377715319,
    flags: MemberInfoFlags::new(145),
    module: "Vegetation",
    data: TypeInfoData::Array("VegetationBaseEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct VegetationTreeEntity {
    pub _glacier_base: super::entity::ComponentEntity,
}

pub trait VegetationTreeEntityTrait: super::entity::ComponentEntityTrait {
}

impl VegetationTreeEntityTrait for VegetationTreeEntity {
}

impl super::entity::ComponentEntityTrait for VegetationTreeEntity {
}

impl super::entity::SpatialEntityTrait for VegetationTreeEntity {
}

impl super::entity::EntityTrait for VegetationTreeEntity {
}

impl super::entity::EntityBusPeerTrait for VegetationTreeEntity {
}

pub static VEGETATIONTREEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationTreeEntity",
    name_hash: 3840689888,
    flags: MemberInfoFlags::new(101),
    module: "Vegetation",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::COMPONENTENTITY_TYPE_INFO),
        super_class_offset: offset_of!(VegetationTreeEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VegetationTreeEntity as Default>::default())),
            create_boxed: || Box::new(<VegetationTreeEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(VEGETATIONTREEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for VegetationTreeEntity {
    fn type_info(&self) -> &'static TypeInfo {
        VEGETATIONTREEENTITY_TYPE_INFO
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


pub static VEGETATIONTREEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationTreeEntity-Array",
    name_hash: 2091932884,
    flags: MemberInfoFlags::new(145),
    module: "Vegetation",
    data: TypeInfoData::Array("VegetationTreeEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerVegetationTreeEntity {
    pub _glacier_base: VegetationTreeEntity,
}

pub trait ServerVegetationTreeEntityTrait: VegetationTreeEntityTrait {
}

impl ServerVegetationTreeEntityTrait for ServerVegetationTreeEntity {
}

impl VegetationTreeEntityTrait for ServerVegetationTreeEntity {
}

impl super::entity::ComponentEntityTrait for ServerVegetationTreeEntity {
}

impl super::entity::SpatialEntityTrait for ServerVegetationTreeEntity {
}

impl super::entity::EntityTrait for ServerVegetationTreeEntity {
}

impl super::entity::EntityBusPeerTrait for ServerVegetationTreeEntity {
}

pub static SERVERVEGETATIONTREEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVegetationTreeEntity",
    name_hash: 219382405,
    flags: MemberInfoFlags::new(101),
    module: "Vegetation",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VEGETATIONTREEENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerVegetationTreeEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerVegetationTreeEntity as Default>::default())),
            create_boxed: || Box::new(<ServerVegetationTreeEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERVEGETATIONTREEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerVegetationTreeEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERVEGETATIONTREEENTITY_TYPE_INFO
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


pub static SERVERVEGETATIONTREEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVegetationTreeEntity-Array",
    name_hash: 552897073,
    flags: MemberInfoFlags::new(145),
    module: "Vegetation",
    data: TypeInfoData::Array("ServerVegetationTreeEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientVegetationTreeEntity {
    pub _glacier_base: VegetationTreeEntity,
}

pub trait ClientVegetationTreeEntityTrait: VegetationTreeEntityTrait {
}

impl ClientVegetationTreeEntityTrait for ClientVegetationTreeEntity {
}

impl VegetationTreeEntityTrait for ClientVegetationTreeEntity {
}

impl super::entity::ComponentEntityTrait for ClientVegetationTreeEntity {
}

impl super::entity::SpatialEntityTrait for ClientVegetationTreeEntity {
}

impl super::entity::EntityTrait for ClientVegetationTreeEntity {
}

impl super::entity::EntityBusPeerTrait for ClientVegetationTreeEntity {
}

pub static CLIENTVEGETATIONTREEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVegetationTreeEntity",
    name_hash: 3576978009,
    flags: MemberInfoFlags::new(101),
    module: "Vegetation",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VEGETATIONTREEENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientVegetationTreeEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientVegetationTreeEntity as Default>::default())),
            create_boxed: || Box::new(<ClientVegetationTreeEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVEGETATIONTREEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientVegetationTreeEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTVEGETATIONTREEENTITY_TYPE_INFO
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


pub static CLIENTVEGETATIONTREEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVegetationTreeEntity-Array",
    name_hash: 1305913453,
    flags: MemberInfoFlags::new(145),
    module: "Vegetation",
    data: TypeInfoData::Array("ClientVegetationTreeEntity"),
    array_type: None,
    alignment: 8,
};


