use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
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
    fn max_active_distance(&self) -> &f32;
    fn max_effect_distance(&self) -> &f32;
    fn max_active_bones(&self) -> &u32;
    fn max_wiggle_distance(&self) -> &f32;
    fn dissolve_enable(&self) -> &bool;
    fn simulation_ease_out_time(&self) -> &f32;
    fn enable(&self) -> &bool;
    fn time_scale(&self) -> &f32;
    fn enable_jobs(&self) -> &bool;
    fn job_count(&self) -> &u32;
    fn draw_nodes(&self) -> &bool;
    fn draw_node_i_ds(&self) -> &bool;
    fn draw_node_stiffness(&self) -> &bool;
    fn draw_non_simulated_as_rigid(&self) -> &bool;
    fn draw_active_instance_boxes_enable(&self) -> &bool;
    fn draw_effect_debug_info_enable(&self) -> &bool;
    fn draw_damage_debug_info_enable(&self) -> &bool;
    fn draw_stats_enable(&self) -> &bool;
    fn procedural_animation_enable(&self) -> &bool;
}

impl VegetationSystemSettingsTrait for VegetationSystemSettings {
    fn destruction_enable(&self) -> &bool {
        &self.destruction_enable
    }
    fn max_active_distance(&self) -> &f32 {
        &self.max_active_distance
    }
    fn max_effect_distance(&self) -> &f32 {
        &self.max_effect_distance
    }
    fn max_active_bones(&self) -> &u32 {
        &self.max_active_bones
    }
    fn max_wiggle_distance(&self) -> &f32 {
        &self.max_wiggle_distance
    }
    fn dissolve_enable(&self) -> &bool {
        &self.dissolve_enable
    }
    fn simulation_ease_out_time(&self) -> &f32 {
        &self.simulation_ease_out_time
    }
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn time_scale(&self) -> &f32 {
        &self.time_scale
    }
    fn enable_jobs(&self) -> &bool {
        &self.enable_jobs
    }
    fn job_count(&self) -> &u32 {
        &self.job_count
    }
    fn draw_nodes(&self) -> &bool {
        &self.draw_nodes
    }
    fn draw_node_i_ds(&self) -> &bool {
        &self.draw_node_i_ds
    }
    fn draw_node_stiffness(&self) -> &bool {
        &self.draw_node_stiffness
    }
    fn draw_non_simulated_as_rigid(&self) -> &bool {
        &self.draw_non_simulated_as_rigid
    }
    fn draw_active_instance_boxes_enable(&self) -> &bool {
        &self.draw_active_instance_boxes_enable
    }
    fn draw_effect_debug_info_enable(&self) -> &bool {
        &self.draw_effect_debug_info_enable
    }
    fn draw_damage_debug_info_enable(&self) -> &bool {
        &self.draw_damage_debug_info_enable
    }
    fn draw_stats_enable(&self) -> &bool {
        &self.draw_stats_enable
    }
    fn procedural_animation_enable(&self) -> &bool {
        &self.procedural_animation_enable
    }
}

impl super::core::DataContainerTrait for VegetationSystemSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static VEGETATIONSYSTEMSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationSystemSettings",
    flags: MemberInfoFlags::new(101),
    module: "Vegetation",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VegetationSystemSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "DestructionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VegetationSystemSettings, destruction_enable),
            },
            FieldInfoData {
                name: "MaxActiveDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationSystemSettings, max_active_distance),
            },
            FieldInfoData {
                name: "MaxEffectDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationSystemSettings, max_effect_distance),
            },
            FieldInfoData {
                name: "MaxActiveBones",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VegetationSystemSettings, max_active_bones),
            },
            FieldInfoData {
                name: "MaxWiggleDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationSystemSettings, max_wiggle_distance),
            },
            FieldInfoData {
                name: "DissolveEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VegetationSystemSettings, dissolve_enable),
            },
            FieldInfoData {
                name: "SimulationEaseOutTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationSystemSettings, simulation_ease_out_time),
            },
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VegetationSystemSettings, enable),
            },
            FieldInfoData {
                name: "TimeScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationSystemSettings, time_scale),
            },
            FieldInfoData {
                name: "EnableJobs",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VegetationSystemSettings, enable_jobs),
            },
            FieldInfoData {
                name: "JobCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VegetationSystemSettings, job_count),
            },
            FieldInfoData {
                name: "DrawNodes",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VegetationSystemSettings, draw_nodes),
            },
            FieldInfoData {
                name: "DrawNodeIDs",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VegetationSystemSettings, draw_node_i_ds),
            },
            FieldInfoData {
                name: "DrawNodeStiffness",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VegetationSystemSettings, draw_node_stiffness),
            },
            FieldInfoData {
                name: "DrawNonSimulatedAsRigid",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VegetationSystemSettings, draw_non_simulated_as_rigid),
            },
            FieldInfoData {
                name: "DrawActiveInstanceBoxesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VegetationSystemSettings, draw_active_instance_boxes_enable),
            },
            FieldInfoData {
                name: "DrawEffectDebugInfoEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VegetationSystemSettings, draw_effect_debug_info_enable),
            },
            FieldInfoData {
                name: "DrawDamageDebugInfoEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VegetationSystemSettings, draw_damage_debug_info_enable),
            },
            FieldInfoData {
                name: "DrawStatsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VegetationSystemSettings, draw_stats_enable),
            },
            FieldInfoData {
                name: "ProceduralAnimationEnable",
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
}


pub static VEGETATIONSYSTEMSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationSystemSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Vegetation",
    data: TypeInfoData::Array("VegetationSystemSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VegetationTreeBreakNodeState {
    pub break_node_bit: bool,
    pub trans: super::core::Vec3,
    pub rotation: super::core::Quat,
}

pub trait VegetationTreeBreakNodeStateTrait: TypeObject {
    fn break_node_bit(&self) -> &bool;
    fn trans(&self) -> &super::core::Vec3;
    fn rotation(&self) -> &super::core::Quat;
}

impl VegetationTreeBreakNodeStateTrait for VegetationTreeBreakNodeState {
    fn break_node_bit(&self) -> &bool {
        &self.break_node_bit
    }
    fn trans(&self) -> &super::core::Vec3 {
        &self.trans
    }
    fn rotation(&self) -> &super::core::Quat {
        &self.rotation
    }
}

pub static VEGETATIONTREEBREAKNODESTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationTreeBreakNodeState",
    flags: MemberInfoFlags::new(36937),
    module: "Vegetation",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VegetationTreeBreakNodeState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "BreakNodeBit",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VegetationTreeBreakNodeState, break_node_bit),
            },
            FieldInfoData {
                name: "Trans",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(VegetationTreeBreakNodeState, trans),
            },
            FieldInfoData {
                name: "Rotation",
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
}


pub static VEGETATIONTREEBREAKNODESTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationTreeBreakNodeState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Vegetation",
    data: TypeInfoData::Array("VegetationTreeBreakNodeState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VegetationTreeBreakNodeDestruction {
    pub break_node_bit: bool,
    pub height: f32,
    pub part_index: u32,
}

pub trait VegetationTreeBreakNodeDestructionTrait: TypeObject {
    fn break_node_bit(&self) -> &bool;
    fn height(&self) -> &f32;
    fn part_index(&self) -> &u32;
}

impl VegetationTreeBreakNodeDestructionTrait for VegetationTreeBreakNodeDestruction {
    fn break_node_bit(&self) -> &bool {
        &self.break_node_bit
    }
    fn height(&self) -> &f32 {
        &self.height
    }
    fn part_index(&self) -> &u32 {
        &self.part_index
    }
}

pub static VEGETATIONTREEBREAKNODEDESTRUCTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationTreeBreakNodeDestruction",
    flags: MemberInfoFlags::new(36937),
    module: "Vegetation",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VegetationTreeBreakNodeDestruction as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "BreakNodeBit",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VegetationTreeBreakNodeDestruction, break_node_bit),
            },
            FieldInfoData {
                name: "Height",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeBreakNodeDestruction, height),
            },
            FieldInfoData {
                name: "PartIndex",
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
}


pub static VEGETATIONTREEBREAKNODEDESTRUCTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationTreeBreakNodeDestruction-Array",
    flags: MemberInfoFlags::new(145),
    module: "Vegetation",
    data: TypeInfoData::Array("VegetationTreeBreakNodeDestruction"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    pub wind_effect: Option<Arc<Mutex<dyn super::effect_base::EffectBlueprintTrait>>>,
    pub min_respawn_time: f32,
    pub stem_effect_node_threshold: u32,
    pub shadow_l_o_d_offset: u32,
    pub translucency_enabled: bool,
    pub translucency_volume_center: super::core::Vec3,
}

pub trait VegetationTreeEntityDataTrait: VegetationBaseEntityDataTrait {
    fn stiffness(&self) -> &f32;
    fn damping(&self) -> &f32;
    fn stem_mass(&self) -> &f32;
    fn stiffness_spread(&self) -> &f32;
    fn damping_spread(&self) -> &f32;
    fn mass_spread(&self) -> &f32;
    fn stem_locked_up_to(&self) -> &f32;
    fn stem_bone_count(&self) -> &i32;
    fn breakable_joint_threshold(&self) -> &f32;
    fn constant_falloff(&self) -> &bool;
    fn bounding_box_scale_factor(&self) -> &f32;
    fn indestructable(&self) -> &bool;
    fn parts_time_to_live(&self) -> &f32;
    fn linear_velocity_damping(&self) -> &f32;
    fn angular_velocity_damping(&self) -> &f32;
    fn friction(&self) -> &f32;
    fn restitution(&self) -> &f32;
    fn stem_physics_width(&self) -> &f32;
    fn stem_physics_height_scale(&self) -> &f32;
    fn branch_physics_width(&self) -> &f32;
    fn branch_physics_height_scale(&self) -> &f32;
    fn destruction_mass_scale(&self) -> &f32;
    fn inertia_modifier(&self) -> &super::core::Vec3;
    fn center_of_mass_vertical_scale(&self) -> &f32;
    fn stem_break_effect(&self) -> &VegetationEffectSlot;
    fn branch_break_effect(&self) -> &VegetationEffectSlot;
    fn impact_effect(&self) -> &VegetationEffectSlot;
    fn wind_effect(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectBlueprintTrait>>>;
    fn min_respawn_time(&self) -> &f32;
    fn stem_effect_node_threshold(&self) -> &u32;
    fn shadow_l_o_d_offset(&self) -> &u32;
    fn translucency_enabled(&self) -> &bool;
    fn translucency_volume_center(&self) -> &super::core::Vec3;
}

impl VegetationTreeEntityDataTrait for VegetationTreeEntityData {
    fn stiffness(&self) -> &f32 {
        &self.stiffness
    }
    fn damping(&self) -> &f32 {
        &self.damping
    }
    fn stem_mass(&self) -> &f32 {
        &self.stem_mass
    }
    fn stiffness_spread(&self) -> &f32 {
        &self.stiffness_spread
    }
    fn damping_spread(&self) -> &f32 {
        &self.damping_spread
    }
    fn mass_spread(&self) -> &f32 {
        &self.mass_spread
    }
    fn stem_locked_up_to(&self) -> &f32 {
        &self.stem_locked_up_to
    }
    fn stem_bone_count(&self) -> &i32 {
        &self.stem_bone_count
    }
    fn breakable_joint_threshold(&self) -> &f32 {
        &self.breakable_joint_threshold
    }
    fn constant_falloff(&self) -> &bool {
        &self.constant_falloff
    }
    fn bounding_box_scale_factor(&self) -> &f32 {
        &self.bounding_box_scale_factor
    }
    fn indestructable(&self) -> &bool {
        &self.indestructable
    }
    fn parts_time_to_live(&self) -> &f32 {
        &self.parts_time_to_live
    }
    fn linear_velocity_damping(&self) -> &f32 {
        &self.linear_velocity_damping
    }
    fn angular_velocity_damping(&self) -> &f32 {
        &self.angular_velocity_damping
    }
    fn friction(&self) -> &f32 {
        &self.friction
    }
    fn restitution(&self) -> &f32 {
        &self.restitution
    }
    fn stem_physics_width(&self) -> &f32 {
        &self.stem_physics_width
    }
    fn stem_physics_height_scale(&self) -> &f32 {
        &self.stem_physics_height_scale
    }
    fn branch_physics_width(&self) -> &f32 {
        &self.branch_physics_width
    }
    fn branch_physics_height_scale(&self) -> &f32 {
        &self.branch_physics_height_scale
    }
    fn destruction_mass_scale(&self) -> &f32 {
        &self.destruction_mass_scale
    }
    fn inertia_modifier(&self) -> &super::core::Vec3 {
        &self.inertia_modifier
    }
    fn center_of_mass_vertical_scale(&self) -> &f32 {
        &self.center_of_mass_vertical_scale
    }
    fn stem_break_effect(&self) -> &VegetationEffectSlot {
        &self.stem_break_effect
    }
    fn branch_break_effect(&self) -> &VegetationEffectSlot {
        &self.branch_break_effect
    }
    fn impact_effect(&self) -> &VegetationEffectSlot {
        &self.impact_effect
    }
    fn wind_effect(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectBlueprintTrait>>> {
        &self.wind_effect
    }
    fn min_respawn_time(&self) -> &f32 {
        &self.min_respawn_time
    }
    fn stem_effect_node_threshold(&self) -> &u32 {
        &self.stem_effect_node_threshold
    }
    fn shadow_l_o_d_offset(&self) -> &u32 {
        &self.shadow_l_o_d_offset
    }
    fn translucency_enabled(&self) -> &bool {
        &self.translucency_enabled
    }
    fn translucency_volume_center(&self) -> &super::core::Vec3 {
        &self.translucency_volume_center
    }
}

impl VegetationBaseEntityDataTrait for VegetationTreeEntityData {
    fn base_pose_transforms(&self) -> &super::core::SparseTransformArray {
        self._glacier_base.base_pose_transforms()
    }
    fn hierarchy(&self) -> &Vec<i32> {
        self._glacier_base.hierarchy()
    }
    fn part_indirection(&self) -> &Vec<i32> {
        self._glacier_base.part_indirection()
    }
    fn part_hierarchy(&self) -> &Vec<i32> {
        self._glacier_base.part_hierarchy()
    }
    fn part_initial_healths(&self) -> &Vec<f32> {
        self._glacier_base.part_initial_healths()
    }
    fn bone_is_stem(&self) -> &Vec<bool> {
        self._glacier_base.bone_is_stem()
    }
    fn mesh(&self) -> &Option<Arc<Mutex<dyn super::render_base::MeshBaseAssetTrait>>> {
        self._glacier_base.mesh()
    }
    fn rigid_bodies(&self) -> &Vec<Option<Arc<Mutex<dyn super::physics::RigidBodyDataTrait>>>> {
        self._glacier_base.rigid_bodies()
    }
}

impl super::entity::GameComponentEntityDataTrait for VegetationTreeEntityData {
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
}

impl super::entity::ComponentEntityDataTrait for VegetationTreeEntityData {
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

impl super::entity::SpatialEntityDataTrait for VegetationTreeEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
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
}

impl super::core::GameDataContainerTrait for VegetationTreeEntityData {
}

impl super::core::DataContainerTrait for VegetationTreeEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static VEGETATIONTREEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationTreeEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Vegetation",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VEGETATIONBASEENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VegetationTreeEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Stiffness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, stiffness),
            },
            FieldInfoData {
                name: "Damping",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, damping),
            },
            FieldInfoData {
                name: "StemMass",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, stem_mass),
            },
            FieldInfoData {
                name: "StiffnessSpread",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, stiffness_spread),
            },
            FieldInfoData {
                name: "DampingSpread",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, damping_spread),
            },
            FieldInfoData {
                name: "MassSpread",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, mass_spread),
            },
            FieldInfoData {
                name: "StemLockedUpTo",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, stem_locked_up_to),
            },
            FieldInfoData {
                name: "StemBoneCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(VegetationTreeEntityData, stem_bone_count),
            },
            FieldInfoData {
                name: "BreakableJointThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, breakable_joint_threshold),
            },
            FieldInfoData {
                name: "ConstantFalloff",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VegetationTreeEntityData, constant_falloff),
            },
            FieldInfoData {
                name: "BoundingBoxScaleFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, bounding_box_scale_factor),
            },
            FieldInfoData {
                name: "Indestructable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VegetationTreeEntityData, indestructable),
            },
            FieldInfoData {
                name: "PartsTimeToLive",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, parts_time_to_live),
            },
            FieldInfoData {
                name: "LinearVelocityDamping",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, linear_velocity_damping),
            },
            FieldInfoData {
                name: "AngularVelocityDamping",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, angular_velocity_damping),
            },
            FieldInfoData {
                name: "Friction",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, friction),
            },
            FieldInfoData {
                name: "Restitution",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, restitution),
            },
            FieldInfoData {
                name: "StemPhysicsWidth",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, stem_physics_width),
            },
            FieldInfoData {
                name: "StemPhysicsHeightScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, stem_physics_height_scale),
            },
            FieldInfoData {
                name: "BranchPhysicsWidth",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, branch_physics_width),
            },
            FieldInfoData {
                name: "BranchPhysicsHeightScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, branch_physics_height_scale),
            },
            FieldInfoData {
                name: "DestructionMassScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, destruction_mass_scale),
            },
            FieldInfoData {
                name: "InertiaModifier",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(VegetationTreeEntityData, inertia_modifier),
            },
            FieldInfoData {
                name: "CenterOfMassVerticalScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, center_of_mass_vertical_scale),
            },
            FieldInfoData {
                name: "StemBreakEffect",
                flags: MemberInfoFlags::new(0),
                field_type: "VegetationEffectSlot",
                rust_offset: offset_of!(VegetationTreeEntityData, stem_break_effect),
            },
            FieldInfoData {
                name: "BranchBreakEffect",
                flags: MemberInfoFlags::new(0),
                field_type: "VegetationEffectSlot",
                rust_offset: offset_of!(VegetationTreeEntityData, branch_break_effect),
            },
            FieldInfoData {
                name: "ImpactEffect",
                flags: MemberInfoFlags::new(0),
                field_type: "VegetationEffectSlot",
                rust_offset: offset_of!(VegetationTreeEntityData, impact_effect),
            },
            FieldInfoData {
                name: "WindEffect",
                flags: MemberInfoFlags::new(0),
                field_type: "EffectBlueprint",
                rust_offset: offset_of!(VegetationTreeEntityData, wind_effect),
            },
            FieldInfoData {
                name: "MinRespawnTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationTreeEntityData, min_respawn_time),
            },
            FieldInfoData {
                name: "StemEffectNodeThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VegetationTreeEntityData, stem_effect_node_threshold),
            },
            FieldInfoData {
                name: "ShadowLODOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VegetationTreeEntityData, shadow_l_o_d_offset),
            },
            FieldInfoData {
                name: "TranslucencyEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VegetationTreeEntityData, translucency_enabled),
            },
            FieldInfoData {
                name: "TranslucencyVolumeCenter",
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
}


pub static VEGETATIONTREEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationTreeEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Vegetation",
    data: TypeInfoData::Array("VegetationTreeEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VegetationEffectSlot {
    pub effect: Option<Arc<Mutex<dyn super::effect_base::EffectBlueprintTrait>>>,
    pub strength_min: f32,
    pub strength_max: f32,
    pub size_min: f32,
    pub size_max: f32,
}

pub trait VegetationEffectSlotTrait: TypeObject {
    fn effect(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectBlueprintTrait>>>;
    fn strength_min(&self) -> &f32;
    fn strength_max(&self) -> &f32;
    fn size_min(&self) -> &f32;
    fn size_max(&self) -> &f32;
}

impl VegetationEffectSlotTrait for VegetationEffectSlot {
    fn effect(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectBlueprintTrait>>> {
        &self.effect
    }
    fn strength_min(&self) -> &f32 {
        &self.strength_min
    }
    fn strength_max(&self) -> &f32 {
        &self.strength_max
    }
    fn size_min(&self) -> &f32 {
        &self.size_min
    }
    fn size_max(&self) -> &f32 {
        &self.size_max
    }
}

pub static VEGETATIONEFFECTSLOT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationEffectSlot",
    flags: MemberInfoFlags::new(73),
    module: "Vegetation",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VegetationEffectSlot as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Effect",
                flags: MemberInfoFlags::new(0),
                field_type: "EffectBlueprint",
                rust_offset: offset_of!(VegetationEffectSlot, effect),
            },
            FieldInfoData {
                name: "StrengthMin",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationEffectSlot, strength_min),
            },
            FieldInfoData {
                name: "StrengthMax",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationEffectSlot, strength_max),
            },
            FieldInfoData {
                name: "SizeMin",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VegetationEffectSlot, size_min),
            },
            FieldInfoData {
                name: "SizeMax",
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
}


pub static VEGETATIONEFFECTSLOT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationEffectSlot-Array",
    flags: MemberInfoFlags::new(145),
    module: "Vegetation",
    data: TypeInfoData::Array("VegetationEffectSlot"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VegetationBaseEntityData {
    pub _glacier_base: super::entity::GameComponentEntityData,
    pub base_pose_transforms: super::core::SparseTransformArray,
    pub hierarchy: Vec<i32>,
    pub part_indirection: Vec<i32>,
    pub part_hierarchy: Vec<i32>,
    pub part_initial_healths: Vec<f32>,
    pub bone_is_stem: Vec<bool>,
    pub mesh: Option<Arc<Mutex<dyn super::render_base::MeshBaseAssetTrait>>>,
    pub rigid_bodies: Vec<Option<Arc<Mutex<dyn super::physics::RigidBodyDataTrait>>>>,
}

pub trait VegetationBaseEntityDataTrait: super::entity::GameComponentEntityDataTrait {
    fn base_pose_transforms(&self) -> &super::core::SparseTransformArray;
    fn hierarchy(&self) -> &Vec<i32>;
    fn part_indirection(&self) -> &Vec<i32>;
    fn part_hierarchy(&self) -> &Vec<i32>;
    fn part_initial_healths(&self) -> &Vec<f32>;
    fn bone_is_stem(&self) -> &Vec<bool>;
    fn mesh(&self) -> &Option<Arc<Mutex<dyn super::render_base::MeshBaseAssetTrait>>>;
    fn rigid_bodies(&self) -> &Vec<Option<Arc<Mutex<dyn super::physics::RigidBodyDataTrait>>>>;
}

impl VegetationBaseEntityDataTrait for VegetationBaseEntityData {
    fn base_pose_transforms(&self) -> &super::core::SparseTransformArray {
        &self.base_pose_transforms
    }
    fn hierarchy(&self) -> &Vec<i32> {
        &self.hierarchy
    }
    fn part_indirection(&self) -> &Vec<i32> {
        &self.part_indirection
    }
    fn part_hierarchy(&self) -> &Vec<i32> {
        &self.part_hierarchy
    }
    fn part_initial_healths(&self) -> &Vec<f32> {
        &self.part_initial_healths
    }
    fn bone_is_stem(&self) -> &Vec<bool> {
        &self.bone_is_stem
    }
    fn mesh(&self) -> &Option<Arc<Mutex<dyn super::render_base::MeshBaseAssetTrait>>> {
        &self.mesh
    }
    fn rigid_bodies(&self) -> &Vec<Option<Arc<Mutex<dyn super::physics::RigidBodyDataTrait>>>> {
        &self.rigid_bodies
    }
}

impl super::entity::GameComponentEntityDataTrait for VegetationBaseEntityData {
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
}

impl super::entity::ComponentEntityDataTrait for VegetationBaseEntityData {
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

impl super::entity::SpatialEntityDataTrait for VegetationBaseEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
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
}

impl super::core::GameDataContainerTrait for VegetationBaseEntityData {
}

impl super::core::DataContainerTrait for VegetationBaseEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static VEGETATIONBASEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationBaseEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Vegetation",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::GAMECOMPONENTENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VegetationBaseEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "BasePoseTransforms",
                flags: MemberInfoFlags::new(0),
                field_type: "SparseTransformArray",
                rust_offset: offset_of!(VegetationBaseEntityData, base_pose_transforms),
            },
            FieldInfoData {
                name: "Hierarchy",
                flags: MemberInfoFlags::new(144),
                field_type: "Int32-Array",
                rust_offset: offset_of!(VegetationBaseEntityData, hierarchy),
            },
            FieldInfoData {
                name: "PartIndirection",
                flags: MemberInfoFlags::new(144),
                field_type: "Int32-Array",
                rust_offset: offset_of!(VegetationBaseEntityData, part_indirection),
            },
            FieldInfoData {
                name: "PartHierarchy",
                flags: MemberInfoFlags::new(144),
                field_type: "Int32-Array",
                rust_offset: offset_of!(VegetationBaseEntityData, part_hierarchy),
            },
            FieldInfoData {
                name: "PartInitialHealths",
                flags: MemberInfoFlags::new(144),
                field_type: "Float32-Array",
                rust_offset: offset_of!(VegetationBaseEntityData, part_initial_healths),
            },
            FieldInfoData {
                name: "BoneIsStem",
                flags: MemberInfoFlags::new(144),
                field_type: "Boolean-Array",
                rust_offset: offset_of!(VegetationBaseEntityData, bone_is_stem),
            },
            FieldInfoData {
                name: "Mesh",
                flags: MemberInfoFlags::new(0),
                field_type: "MeshBaseAsset",
                rust_offset: offset_of!(VegetationBaseEntityData, mesh),
            },
            FieldInfoData {
                name: "RigidBodies",
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
}


pub static VEGETATIONBASEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationBaseEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Vegetation",
    data: TypeInfoData::Array("VegetationBaseEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "Vegetation",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::COMPONENTENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VegetationTreeEntity as Default>::default())),
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
}


pub static VEGETATIONTREEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VegetationTreeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Vegetation",
    data: TypeInfoData::Array("VegetationTreeEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "Vegetation",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VEGETATIONTREEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerVegetationTreeEntity as Default>::default())),
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
}


pub static SERVERVEGETATIONTREEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVegetationTreeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Vegetation",
    data: TypeInfoData::Array("ServerVegetationTreeEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "Vegetation",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VEGETATIONTREEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientVegetationTreeEntity as Default>::default())),
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
}


pub static CLIENTVEGETATIONTREEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVegetationTreeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Vegetation",
    data: TypeInfoData::Array("ClientVegetationTreeEntity"),
    array_type: None,
    alignment: 8,
};


