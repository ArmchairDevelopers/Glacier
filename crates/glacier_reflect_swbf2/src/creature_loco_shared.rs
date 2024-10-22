use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_creature_loco_shared_types(registry: &mut TypeRegistry) {
    registry.register_type(CREATUREWAYPOINTNETSTATE_TYPE_INFO);
    registry.register_type(CREATUREWAYPOINTNETSTATE_ARRAY_TYPE_INFO);
    registry.register_type(AIWAYPOINTEXTRAALIGNINFO_TYPE_INFO);
    registry.register_type(AIWAYPOINTEXTRAALIGNINFO_ARRAY_TYPE_INFO);
    registry.register_type(AIWAYPOINTEXTRACREATURELOCO_TYPE_INFO);
    registry.register_type(AIWAYPOINTEXTRACREATURELOCO_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREPLAYANIMATIONWAYPOINTDATA_TYPE_INFO);
    registry.register_type(CREATUREPLAYANIMATIONWAYPOINTDATA_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREPAUSEWAYPOINTDATA_TYPE_INFO);
    registry.register_type(CREATUREPAUSEWAYPOINTDATA_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREPAUSEDATA_TYPE_INFO);
    registry.register_type(CREATUREPAUSEDATA_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREMOVEWAYPOINTDATA_TYPE_INFO);
    registry.register_type(CREATUREMOVEWAYPOINTDATA_ARRAY_TYPE_INFO);
    registry.register_type(CREATURESPEEDLEVEL_TYPE_INFO);
    registry.register_type(CREATURESPEEDLEVEL_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREWAYPOINTSSHAPEDATA_TYPE_INFO);
    registry.register_type(CREATUREWAYPOINTSSHAPEDATA_ARRAY_TYPE_INFO);
    registry.register_type(CREATURELOCOSETTINGS_TYPE_INFO);
    registry.register_type(CREATURELOCOSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(PUSHSETTINGS_TYPE_INFO);
    registry.register_type(PUSHSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(PUSHSETTINGSFILTER_TYPE_INFO);
    registry.register_type(PUSHSETTINGSFILTER_ARRAY_TYPE_INFO);
    registry.register_type(SIZESETTINGS_TYPE_INFO);
    registry.register_type(SIZESETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(WORLDEVENTACTIONSSETTINGS_TYPE_INFO);
    registry.register_type(WORLDEVENTACTIONSSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(WORLDEVENTACTIONVENTPARAMETERS_TYPE_INFO);
    registry.register_type(WORLDEVENTACTIONVENTPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(CREATURELOCOEXTERNALINFLUENCEREACTIONALIGNMENT_TYPE_INFO);
    registry.register_type(CREATURELOCOEXTERNALINFLUENCEREACTIONALIGNMENT_ARRAY_TYPE_INFO);
    registry.register_type(CREATURELOCOEXTERNALINFLUENCEREACTIONTYPE_TYPE_INFO);
    registry.register_type(CREATURELOCOEXTERNALINFLUENCEREACTIONTYPE_ARRAY_TYPE_INFO);
    registry.register_type(CREATURELOCOEXTERNALINFLUENCETYPE_TYPE_INFO);
    registry.register_type(CREATURELOCOEXTERNALINFLUENCETYPE_ARRAY_TYPE_INFO);
    registry.register_type(PROCEDURALMOVEMENTSTATESETTINGS_TYPE_INFO);
    registry.register_type(PROCEDURALMOVEMENTSTATESETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(LOCOSTATESPEEDRANGE_TYPE_INFO);
    registry.register_type(LOCOSTATESPEEDRANGE_ARRAY_TYPE_INFO);
    registry.register_type(AVOIDANCESTEERINGSETTINGS_TYPE_INFO);
    registry.register_type(AVOIDANCESTEERINGSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(AVOIDANCEDATA_TYPE_INFO);
    registry.register_type(AVOIDANCEDATA_ARRAY_TYPE_INFO);
    registry.register_type(AIAVOIDANCEPLAYERBEHAVIOUR_TYPE_INFO);
    registry.register_type(AIAVOIDANCEPLAYERBEHAVIOUR_ARRAY_TYPE_INFO);
    registry.register_type(CURVESTEERINGSETTINGS_TYPE_INFO);
    registry.register_type(CURVESTEERINGSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(CURVESTEERINGSETTINGSDATA_TYPE_INFO);
    registry.register_type(CURVESTEERINGSETTINGSDATA_ARRAY_TYPE_INFO);
    registry.register_type(BASICSTEERINGSETTINGSDATA_TYPE_INFO);
    registry.register_type(BASICSTEERINGSETTINGSDATA_ARRAY_TYPE_INFO);
    registry.register_type(COMMONCLIENTSETTINGS_TYPE_INFO);
    registry.register_type(COMMONCLIENTSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(PLAYANIMSETTINGS_TYPE_INFO);
    registry.register_type(PLAYANIMSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(TURNSETTINGS_TYPE_INFO);
    registry.register_type(TURNSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(STOPSETTINGS_TYPE_INFO);
    registry.register_type(STOPSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(STARTSETTINGS_TYPE_INFO);
    registry.register_type(STARTSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(MOVECYCLESETTINGS_TYPE_INFO);
    registry.register_type(MOVECYCLESETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(IDLESETTINGS_TYPE_INFO);
    registry.register_type(IDLESETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(LOCOMOTIONSTATESETTINGS_TYPE_INFO);
    registry.register_type(LOCOMOTIONSTATESETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(CREATURELOCOSERVERAUTHENTITYDATA_TYPE_INFO);
    registry.register_type(CREATURELOCOSERVERAUTHENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CREATURELOCOENTITYDATA_TYPE_INFO);
    registry.register_type(CREATURELOCOENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREBASEWAYPOINTPROVIDERENTITYDATA_TYPE_INFO);
    registry.register_type(CREATUREBASEWAYPOINTPROVIDERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CREATURECONFIGURATIONPROVIDERENTITYDATA_TYPE_INFO);
    registry.register_type(CREATURECONFIGURATIONPROVIDERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CREATURELOCOBINDINGS_TYPE_INFO);
    registry.register_type(CREATURELOCOBINDINGS_ARRAY_TYPE_INFO);
    registry.register_type(EVENTREACTIONPARAMDATA_TYPE_INFO);
    registry.register_type(EVENTREACTIONPARAMDATA_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREREACTIONBINDING_TYPE_INFO);
    registry.register_type(CREATUREREACTIONBINDING_ARRAY_TYPE_INFO);
    registry.register_type(PROCEDURALMOTIONPARAMDATA_TYPE_INFO);
    registry.register_type(PROCEDURALMOTIONPARAMDATA_ARRAY_TYPE_INFO);
    registry.register_type(COMMONCLIENTBINDINGSPARAMDATA_TYPE_INFO);
    registry.register_type(COMMONCLIENTBINDINGSPARAMDATA_ARRAY_TYPE_INFO);
    registry.register_type(PLAYANIMPARAMDATA_TYPE_INFO);
    registry.register_type(PLAYANIMPARAMDATA_ARRAY_TYPE_INFO);
    registry.register_type(TURNPARAMDATA_TYPE_INFO);
    registry.register_type(TURNPARAMDATA_ARRAY_TYPE_INFO);
    registry.register_type(STOPPARAMDATA_TYPE_INFO);
    registry.register_type(STOPPARAMDATA_ARRAY_TYPE_INFO);
    registry.register_type(STARTPARAMDATA_TYPE_INFO);
    registry.register_type(STARTPARAMDATA_ARRAY_TYPE_INFO);
    registry.register_type(MOVECYCLEPARAMDATA_TYPE_INFO);
    registry.register_type(MOVECYCLEPARAMDATA_ARRAY_TYPE_INFO);
    registry.register_type(IDLEPARAMDATA_TYPE_INFO);
    registry.register_type(IDLEPARAMDATA_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREPLAYANIMATIONBINDING_TYPE_INFO);
    registry.register_type(CREATUREPLAYANIMATIONBINDING_ARRAY_TYPE_INFO);
    registry.register_type(CREATURETURNBINDING_TYPE_INFO);
    registry.register_type(CREATURETURNBINDING_ARRAY_TYPE_INFO);
    registry.register_type(CREATURESTOPBINDING_TYPE_INFO);
    registry.register_type(CREATURESTOPBINDING_ARRAY_TYPE_INFO);
    registry.register_type(CREATURESTARTBINDING_TYPE_INFO);
    registry.register_type(CREATURESTARTBINDING_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREMISCBINDING_TYPE_INFO);
    registry.register_type(CREATUREMISCBINDING_ARRAY_TYPE_INFO);
    registry.register_type(CREATURELOCOBINDING_TYPE_INFO);
    registry.register_type(CREATURELOCOBINDING_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREIDLEBINDING_TYPE_INFO);
    registry.register_type(CREATUREIDLEBINDING_ARRAY_TYPE_INFO);
    registry.register_type(CREATURECOMMONBINDING_TYPE_INFO);
    registry.register_type(CREATURECOMMONBINDING_ARRAY_TYPE_INFO);
    registry.register_type(EARLYOUTTYPE_TYPE_INFO);
    registry.register_type(EARLYOUTTYPE_ARRAY_TYPE_INFO);
    registry.register_type(LOCOMOTIONPARAMBLOCK_TYPE_INFO);
    registry.register_type(LOCOMOTIONPARAMBLOCK_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTUNSPAWNENTITYDATA_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTUNSPAWNENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTCLOSESTCHOOSERENTITYDATA_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTCLOSESTCHOOSERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CL_WAYPOINTLISTCHOOSER_CLOSESTTOTYPE_TYPE_INFO);
    registry.register_type(CL_WAYPOINTLISTCHOOSER_CLOSESTTOTYPE_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTOCCUPANCYCHOOSERENTITYDATA_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTOCCUPANCYCHOOSERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTBOOLCHOOSERENTITYDATA_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTBOOLCHOOSERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTSENTITYDATA_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTSENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTSEGMENTENTITYDATA_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTSEGMENTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWBASEDATA_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWBASEDATA_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTPROVIDERENTITYDATA_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTPROVIDERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CL_WAYPOINTLIST_STARTTYPE_TYPE_INFO);
    registry.register_type(CL_WAYPOINTLIST_STARTTYPE_ARRAY_TYPE_INFO);
    registry.register_type(CREATURECOLLISIONGROUPDATA_TYPE_INFO);
    registry.register_type(CREATURECOLLISIONGROUPDATA_ARRAY_TYPE_INFO);
    registry.register_type(CLINFLUENCEFILTERENTITYDATA_TYPE_INFO);
    registry.register_type(CLINFLUENCEFILTERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CLINFLUENCECOMPAREENTITYDATA_TYPE_INFO);
    registry.register_type(CLINFLUENCECOMPAREENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CLAPPLYINFLUENCEENTITYDATA_TYPE_INFO);
    registry.register_type(CLAPPLYINFLUENCEENTITYDATA_ARRAY_TYPE_INFO);
}

#[derive(Clone, Debug, Default)]
pub struct CreatureWaypointNetState {
    pub g_u_i_d: super::a_i_tools::AIWaypointGUID,
}

pub trait CreatureWaypointNetStateTrait: TypeObject {
    fn g_u_i_d(&self) -> &super::a_i_tools::AIWaypointGUID;
    fn g_u_i_d_mut(&mut self) -> &mut super::a_i_tools::AIWaypointGUID;
}

impl CreatureWaypointNetStateTrait for CreatureWaypointNetState {
    fn g_u_i_d(&self) -> &super::a_i_tools::AIWaypointGUID {
        &self.g_u_i_d
    }
    fn g_u_i_d_mut(&mut self) -> &mut super::a_i_tools::AIWaypointGUID {
        &mut self.g_u_i_d
    }
}

pub static CREATUREWAYPOINTNETSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureWaypointNetState",
    flags: MemberInfoFlags::new(73),
    module: "CreatureLocoShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureWaypointNetState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "GUID",
                flags: MemberInfoFlags::new(0),
                field_type: "AIWaypointGUID",
                rust_offset: offset_of!(CreatureWaypointNetState, g_u_i_d),
            },
        ],
    }),
    array_type: Some(CREATUREWAYPOINTNETSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CreatureWaypointNetState {
    fn type_info(&self) -> &'static TypeInfo {
        CREATUREWAYPOINTNETSTATE_TYPE_INFO
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


pub static CREATUREWAYPOINTNETSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureWaypointNetState-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureWaypointNetState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AIWaypointExtraAlignInfo {
    pub align_pos: super::core::Vec3,
    pub align_facing: f32,
}

pub trait AIWaypointExtraAlignInfoTrait: TypeObject {
    fn align_pos(&self) -> &super::core::Vec3;
    fn align_pos_mut(&mut self) -> &mut super::core::Vec3;
    fn align_facing(&self) -> &f32;
    fn align_facing_mut(&mut self) -> &mut f32;
}

impl AIWaypointExtraAlignInfoTrait for AIWaypointExtraAlignInfo {
    fn align_pos(&self) -> &super::core::Vec3 {
        &self.align_pos
    }
    fn align_pos_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.align_pos
    }
    fn align_facing(&self) -> &f32 {
        &self.align_facing
    }
    fn align_facing_mut(&mut self) -> &mut f32 {
        &mut self.align_facing
    }
}

pub static AIWAYPOINTEXTRAALIGNINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIWaypointExtraAlignInfo",
    flags: MemberInfoFlags::new(36937),
    module: "CreatureLocoShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AIWaypointExtraAlignInfo as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "AlignPos",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AIWaypointExtraAlignInfo, align_pos),
            },
            FieldInfoData {
                name: "AlignFacing",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AIWaypointExtraAlignInfo, align_facing),
            },
        ],
    }),
    array_type: Some(AIWAYPOINTEXTRAALIGNINFO_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AIWaypointExtraAlignInfo {
    fn type_info(&self) -> &'static TypeInfo {
        AIWAYPOINTEXTRAALIGNINFO_TYPE_INFO
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


pub static AIWAYPOINTEXTRAALIGNINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIWaypointExtraAlignInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("AIWaypointExtraAlignInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AIWaypointExtraCreatureLoco {
    pub desired_facing: f32,
    pub speed_level: i32,
    pub stop: bool,
    pub backwards: bool,
    pub force_height: bool,
}

pub trait AIWaypointExtraCreatureLocoTrait: TypeObject {
    fn desired_facing(&self) -> &f32;
    fn desired_facing_mut(&mut self) -> &mut f32;
    fn speed_level(&self) -> &i32;
    fn speed_level_mut(&mut self) -> &mut i32;
    fn stop(&self) -> &bool;
    fn stop_mut(&mut self) -> &mut bool;
    fn backwards(&self) -> &bool;
    fn backwards_mut(&mut self) -> &mut bool;
    fn force_height(&self) -> &bool;
    fn force_height_mut(&mut self) -> &mut bool;
}

impl AIWaypointExtraCreatureLocoTrait for AIWaypointExtraCreatureLoco {
    fn desired_facing(&self) -> &f32 {
        &self.desired_facing
    }
    fn desired_facing_mut(&mut self) -> &mut f32 {
        &mut self.desired_facing
    }
    fn speed_level(&self) -> &i32 {
        &self.speed_level
    }
    fn speed_level_mut(&mut self) -> &mut i32 {
        &mut self.speed_level
    }
    fn stop(&self) -> &bool {
        &self.stop
    }
    fn stop_mut(&mut self) -> &mut bool {
        &mut self.stop
    }
    fn backwards(&self) -> &bool {
        &self.backwards
    }
    fn backwards_mut(&mut self) -> &mut bool {
        &mut self.backwards
    }
    fn force_height(&self) -> &bool {
        &self.force_height
    }
    fn force_height_mut(&mut self) -> &mut bool {
        &mut self.force_height
    }
}

pub static AIWAYPOINTEXTRACREATURELOCO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIWaypointExtraCreatureLoco",
    flags: MemberInfoFlags::new(36937),
    module: "CreatureLocoShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AIWaypointExtraCreatureLoco as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "DesiredFacing",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AIWaypointExtraCreatureLoco, desired_facing),
            },
            FieldInfoData {
                name: "SpeedLevel",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AIWaypointExtraCreatureLoco, speed_level),
            },
            FieldInfoData {
                name: "Stop",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AIWaypointExtraCreatureLoco, stop),
            },
            FieldInfoData {
                name: "Backwards",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AIWaypointExtraCreatureLoco, backwards),
            },
            FieldInfoData {
                name: "ForceHeight",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AIWaypointExtraCreatureLoco, force_height),
            },
        ],
    }),
    array_type: Some(AIWAYPOINTEXTRACREATURELOCO_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for AIWaypointExtraCreatureLoco {
    fn type_info(&self) -> &'static TypeInfo {
        AIWAYPOINTEXTRACREATURELOCO_TYPE_INFO
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


pub static AIWAYPOINTEXTRACREATURELOCO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIWaypointExtraCreatureLoco-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("AIWaypointExtraCreatureLoco"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreaturePlayAnimationWaypointData {
    pub _glacier_base: CreatureMoveWaypointData,
    pub play_animation: Option<Arc<Mutex<dyn super::game_shared::PlayAnimationDataTrait>>>,
    pub stop_at_waypoint: bool,
    pub align_joint: String,
    pub align_transform: super::core::LinearTransform,
    pub enter_position: super::core::Vec3,
    pub exit_position: super::core::Vec3,
}

pub trait CreaturePlayAnimationWaypointDataTrait: CreatureMoveWaypointDataTrait {
    fn play_animation(&self) -> &Option<Arc<Mutex<dyn super::game_shared::PlayAnimationDataTrait>>>;
    fn play_animation_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::game_shared::PlayAnimationDataTrait>>>;
    fn stop_at_waypoint(&self) -> &bool;
    fn stop_at_waypoint_mut(&mut self) -> &mut bool;
    fn align_joint(&self) -> &String;
    fn align_joint_mut(&mut self) -> &mut String;
    fn align_transform(&self) -> &super::core::LinearTransform;
    fn align_transform_mut(&mut self) -> &mut super::core::LinearTransform;
    fn enter_position(&self) -> &super::core::Vec3;
    fn enter_position_mut(&mut self) -> &mut super::core::Vec3;
    fn exit_position(&self) -> &super::core::Vec3;
    fn exit_position_mut(&mut self) -> &mut super::core::Vec3;
}

impl CreaturePlayAnimationWaypointDataTrait for CreaturePlayAnimationWaypointData {
    fn play_animation(&self) -> &Option<Arc<Mutex<dyn super::game_shared::PlayAnimationDataTrait>>> {
        &self.play_animation
    }
    fn play_animation_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::game_shared::PlayAnimationDataTrait>>> {
        &mut self.play_animation
    }
    fn stop_at_waypoint(&self) -> &bool {
        &self.stop_at_waypoint
    }
    fn stop_at_waypoint_mut(&mut self) -> &mut bool {
        &mut self.stop_at_waypoint
    }
    fn align_joint(&self) -> &String {
        &self.align_joint
    }
    fn align_joint_mut(&mut self) -> &mut String {
        &mut self.align_joint
    }
    fn align_transform(&self) -> &super::core::LinearTransform {
        &self.align_transform
    }
    fn align_transform_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.align_transform
    }
    fn enter_position(&self) -> &super::core::Vec3 {
        &self.enter_position
    }
    fn enter_position_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.enter_position
    }
    fn exit_position(&self) -> &super::core::Vec3 {
        &self.exit_position
    }
    fn exit_position_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.exit_position
    }
}

impl CreatureMoveWaypointDataTrait for CreaturePlayAnimationWaypointData {
    fn override_creature_angle(&self) -> &bool {
        self._glacier_base.override_creature_angle()
    }
    fn override_creature_angle_mut(&mut self) -> &mut bool {
        self._glacier_base.override_creature_angle_mut()
    }
    fn world_angle(&self) -> &f32 {
        self._glacier_base.world_angle()
    }
    fn world_angle_mut(&mut self) -> &mut f32 {
        self._glacier_base.world_angle_mut()
    }
    fn radius(&self) -> &f32 {
        self._glacier_base.radius()
    }
    fn radius_mut(&mut self) -> &mut f32 {
        self._glacier_base.radius_mut()
    }
    fn speed_level(&self) -> &CreatureSpeedLevel {
        self._glacier_base.speed_level()
    }
    fn speed_level_mut(&mut self) -> &mut CreatureSpeedLevel {
        self._glacier_base.speed_level_mut()
    }
    fn move_backward(&self) -> &bool {
        self._glacier_base.move_backward()
    }
    fn move_backward_mut(&mut self) -> &mut bool {
        self._glacier_base.move_backward_mut()
    }
    fn explicit_height(&self) -> &bool {
        self._glacier_base.explicit_height()
    }
    fn explicit_height_mut(&mut self) -> &mut bool {
        self._glacier_base.explicit_height_mut()
    }
}

impl super::pathfinding_shared::WaypointDataTrait for CreaturePlayAnimationWaypointData {
    fn use_clients_position(&self) -> &bool {
        self._glacier_base.use_clients_position()
    }
    fn use_clients_position_mut(&mut self) -> &mut bool {
        self._glacier_base.use_clients_position_mut()
    }
    fn schematics_name_hash(&self) -> &i32 {
        self._glacier_base.schematics_name_hash()
    }
    fn schematics_name_hash_mut(&mut self) -> &mut i32 {
        self._glacier_base.schematics_name_hash_mut()
    }
    fn waypoint_id(&self) -> &u32 {
        self._glacier_base.waypoint_id()
    }
    fn waypoint_id_mut(&mut self) -> &mut u32 {
        self._glacier_base.waypoint_id_mut()
    }
}

impl super::core::DataContainerTrait for CreaturePlayAnimationWaypointData {
}

pub static CREATUREPLAYANIMATIONWAYPOINTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreaturePlayAnimationWaypointData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREMOVEWAYPOINTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreaturePlayAnimationWaypointData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "PlayAnimation",
                flags: MemberInfoFlags::new(0),
                field_type: "PlayAnimationData",
                rust_offset: offset_of!(CreaturePlayAnimationWaypointData, play_animation),
            },
            FieldInfoData {
                name: "StopAtWaypoint",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CreaturePlayAnimationWaypointData, stop_at_waypoint),
            },
            FieldInfoData {
                name: "AlignJoint",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(CreaturePlayAnimationWaypointData, align_joint),
            },
            FieldInfoData {
                name: "AlignTransform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(CreaturePlayAnimationWaypointData, align_transform),
            },
            FieldInfoData {
                name: "EnterPosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(CreaturePlayAnimationWaypointData, enter_position),
            },
            FieldInfoData {
                name: "ExitPosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(CreaturePlayAnimationWaypointData, exit_position),
            },
        ],
    }),
    array_type: Some(CREATUREPLAYANIMATIONWAYPOINTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CreaturePlayAnimationWaypointData {
    fn type_info(&self) -> &'static TypeInfo {
        CREATUREPLAYANIMATIONWAYPOINTDATA_TYPE_INFO
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


pub static CREATUREPLAYANIMATIONWAYPOINTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreaturePlayAnimationWaypointData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreaturePlayAnimationWaypointData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreaturePauseWaypointData {
    pub _glacier_base: CreatureMoveWaypointData,
    pub pause_settings_for_slow_speed: CreaturePauseData,
    pub pause_settings_for_fast_speed: CreaturePauseData,
}

pub trait CreaturePauseWaypointDataTrait: CreatureMoveWaypointDataTrait {
    fn pause_settings_for_slow_speed(&self) -> &CreaturePauseData;
    fn pause_settings_for_slow_speed_mut(&mut self) -> &mut CreaturePauseData;
    fn pause_settings_for_fast_speed(&self) -> &CreaturePauseData;
    fn pause_settings_for_fast_speed_mut(&mut self) -> &mut CreaturePauseData;
}

impl CreaturePauseWaypointDataTrait for CreaturePauseWaypointData {
    fn pause_settings_for_slow_speed(&self) -> &CreaturePauseData {
        &self.pause_settings_for_slow_speed
    }
    fn pause_settings_for_slow_speed_mut(&mut self) -> &mut CreaturePauseData {
        &mut self.pause_settings_for_slow_speed
    }
    fn pause_settings_for_fast_speed(&self) -> &CreaturePauseData {
        &self.pause_settings_for_fast_speed
    }
    fn pause_settings_for_fast_speed_mut(&mut self) -> &mut CreaturePauseData {
        &mut self.pause_settings_for_fast_speed
    }
}

impl CreatureMoveWaypointDataTrait for CreaturePauseWaypointData {
    fn override_creature_angle(&self) -> &bool {
        self._glacier_base.override_creature_angle()
    }
    fn override_creature_angle_mut(&mut self) -> &mut bool {
        self._glacier_base.override_creature_angle_mut()
    }
    fn world_angle(&self) -> &f32 {
        self._glacier_base.world_angle()
    }
    fn world_angle_mut(&mut self) -> &mut f32 {
        self._glacier_base.world_angle_mut()
    }
    fn radius(&self) -> &f32 {
        self._glacier_base.radius()
    }
    fn radius_mut(&mut self) -> &mut f32 {
        self._glacier_base.radius_mut()
    }
    fn speed_level(&self) -> &CreatureSpeedLevel {
        self._glacier_base.speed_level()
    }
    fn speed_level_mut(&mut self) -> &mut CreatureSpeedLevel {
        self._glacier_base.speed_level_mut()
    }
    fn move_backward(&self) -> &bool {
        self._glacier_base.move_backward()
    }
    fn move_backward_mut(&mut self) -> &mut bool {
        self._glacier_base.move_backward_mut()
    }
    fn explicit_height(&self) -> &bool {
        self._glacier_base.explicit_height()
    }
    fn explicit_height_mut(&mut self) -> &mut bool {
        self._glacier_base.explicit_height_mut()
    }
}

impl super::pathfinding_shared::WaypointDataTrait for CreaturePauseWaypointData {
    fn use_clients_position(&self) -> &bool {
        self._glacier_base.use_clients_position()
    }
    fn use_clients_position_mut(&mut self) -> &mut bool {
        self._glacier_base.use_clients_position_mut()
    }
    fn schematics_name_hash(&self) -> &i32 {
        self._glacier_base.schematics_name_hash()
    }
    fn schematics_name_hash_mut(&mut self) -> &mut i32 {
        self._glacier_base.schematics_name_hash_mut()
    }
    fn waypoint_id(&self) -> &u32 {
        self._glacier_base.waypoint_id()
    }
    fn waypoint_id_mut(&mut self) -> &mut u32 {
        self._glacier_base.waypoint_id_mut()
    }
}

impl super::core::DataContainerTrait for CreaturePauseWaypointData {
}

pub static CREATUREPAUSEWAYPOINTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreaturePauseWaypointData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREMOVEWAYPOINTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreaturePauseWaypointData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "PauseSettingsForSlowSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: "CreaturePauseData",
                rust_offset: offset_of!(CreaturePauseWaypointData, pause_settings_for_slow_speed),
            },
            FieldInfoData {
                name: "PauseSettingsForFastSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: "CreaturePauseData",
                rust_offset: offset_of!(CreaturePauseWaypointData, pause_settings_for_fast_speed),
            },
        ],
    }),
    array_type: Some(CREATUREPAUSEWAYPOINTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CreaturePauseWaypointData {
    fn type_info(&self) -> &'static TypeInfo {
        CREATUREPAUSEWAYPOINTDATA_TYPE_INFO
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


pub static CREATUREPAUSEWAYPOINTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreaturePauseWaypointData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreaturePauseWaypointData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreaturePauseData {
    pub probability: f32,
    pub minimum_duration: f32,
    pub maximum_duration: f32,
}

pub trait CreaturePauseDataTrait: TypeObject {
    fn probability(&self) -> &f32;
    fn probability_mut(&mut self) -> &mut f32;
    fn minimum_duration(&self) -> &f32;
    fn minimum_duration_mut(&mut self) -> &mut f32;
    fn maximum_duration(&self) -> &f32;
    fn maximum_duration_mut(&mut self) -> &mut f32;
}

impl CreaturePauseDataTrait for CreaturePauseData {
    fn probability(&self) -> &f32 {
        &self.probability
    }
    fn probability_mut(&mut self) -> &mut f32 {
        &mut self.probability
    }
    fn minimum_duration(&self) -> &f32 {
        &self.minimum_duration
    }
    fn minimum_duration_mut(&mut self) -> &mut f32 {
        &mut self.minimum_duration
    }
    fn maximum_duration(&self) -> &f32 {
        &self.maximum_duration
    }
    fn maximum_duration_mut(&mut self) -> &mut f32 {
        &mut self.maximum_duration
    }
}

pub static CREATUREPAUSEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreaturePauseData",
    flags: MemberInfoFlags::new(36937),
    module: "CreatureLocoShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreaturePauseData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Probability",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CreaturePauseData, probability),
            },
            FieldInfoData {
                name: "MinimumDuration",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CreaturePauseData, minimum_duration),
            },
            FieldInfoData {
                name: "MaximumDuration",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CreaturePauseData, maximum_duration),
            },
        ],
    }),
    array_type: Some(CREATUREPAUSEDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for CreaturePauseData {
    fn type_info(&self) -> &'static TypeInfo {
        CREATUREPAUSEDATA_TYPE_INFO
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


pub static CREATUREPAUSEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreaturePauseData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreaturePauseData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreatureMoveWaypointData {
    pub _glacier_base: super::pathfinding_shared::WaypointData,
    pub override_creature_angle: bool,
    pub world_angle: f32,
    pub radius: f32,
    pub speed_level: CreatureSpeedLevel,
    pub move_backward: bool,
    pub explicit_height: bool,
}

pub trait CreatureMoveWaypointDataTrait: super::pathfinding_shared::WaypointDataTrait {
    fn override_creature_angle(&self) -> &bool;
    fn override_creature_angle_mut(&mut self) -> &mut bool;
    fn world_angle(&self) -> &f32;
    fn world_angle_mut(&mut self) -> &mut f32;
    fn radius(&self) -> &f32;
    fn radius_mut(&mut self) -> &mut f32;
    fn speed_level(&self) -> &CreatureSpeedLevel;
    fn speed_level_mut(&mut self) -> &mut CreatureSpeedLevel;
    fn move_backward(&self) -> &bool;
    fn move_backward_mut(&mut self) -> &mut bool;
    fn explicit_height(&self) -> &bool;
    fn explicit_height_mut(&mut self) -> &mut bool;
}

impl CreatureMoveWaypointDataTrait for CreatureMoveWaypointData {
    fn override_creature_angle(&self) -> &bool {
        &self.override_creature_angle
    }
    fn override_creature_angle_mut(&mut self) -> &mut bool {
        &mut self.override_creature_angle
    }
    fn world_angle(&self) -> &f32 {
        &self.world_angle
    }
    fn world_angle_mut(&mut self) -> &mut f32 {
        &mut self.world_angle
    }
    fn radius(&self) -> &f32 {
        &self.radius
    }
    fn radius_mut(&mut self) -> &mut f32 {
        &mut self.radius
    }
    fn speed_level(&self) -> &CreatureSpeedLevel {
        &self.speed_level
    }
    fn speed_level_mut(&mut self) -> &mut CreatureSpeedLevel {
        &mut self.speed_level
    }
    fn move_backward(&self) -> &bool {
        &self.move_backward
    }
    fn move_backward_mut(&mut self) -> &mut bool {
        &mut self.move_backward
    }
    fn explicit_height(&self) -> &bool {
        &self.explicit_height
    }
    fn explicit_height_mut(&mut self) -> &mut bool {
        &mut self.explicit_height
    }
}

impl super::pathfinding_shared::WaypointDataTrait for CreatureMoveWaypointData {
    fn use_clients_position(&self) -> &bool {
        self._glacier_base.use_clients_position()
    }
    fn use_clients_position_mut(&mut self) -> &mut bool {
        self._glacier_base.use_clients_position_mut()
    }
    fn schematics_name_hash(&self) -> &i32 {
        self._glacier_base.schematics_name_hash()
    }
    fn schematics_name_hash_mut(&mut self) -> &mut i32 {
        self._glacier_base.schematics_name_hash_mut()
    }
    fn waypoint_id(&self) -> &u32 {
        self._glacier_base.waypoint_id()
    }
    fn waypoint_id_mut(&mut self) -> &mut u32 {
        self._glacier_base.waypoint_id_mut()
    }
}

impl super::core::DataContainerTrait for CreatureMoveWaypointData {
}

pub static CREATUREMOVEWAYPOINTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureMoveWaypointData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::pathfinding_shared::WAYPOINTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureMoveWaypointData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "OverrideCreatureAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CreatureMoveWaypointData, override_creature_angle),
            },
            FieldInfoData {
                name: "WorldAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CreatureMoveWaypointData, world_angle),
            },
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CreatureMoveWaypointData, radius),
            },
            FieldInfoData {
                name: "SpeedLevel",
                flags: MemberInfoFlags::new(0),
                field_type: "CreatureSpeedLevel",
                rust_offset: offset_of!(CreatureMoveWaypointData, speed_level),
            },
            FieldInfoData {
                name: "MoveBackward",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CreatureMoveWaypointData, move_backward),
            },
            FieldInfoData {
                name: "Explicit_Height",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CreatureMoveWaypointData, explicit_height),
            },
        ],
    }),
    array_type: Some(CREATUREMOVEWAYPOINTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CreatureMoveWaypointData {
    fn type_info(&self) -> &'static TypeInfo {
        CREATUREMOVEWAYPOINTDATA_TYPE_INFO
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


pub static CREATUREMOVEWAYPOINTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureMoveWaypointData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureMoveWaypointData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum CreatureSpeedLevel {
    #[default]
    CreatureSpeedLevel_Slow = 0,
    CreatureSpeedLevel_Fast = 2,
    CreatureSpeedLevel_NoChange = 3,
}

pub static CREATURESPEEDLEVEL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureSpeedLevel",
    flags: MemberInfoFlags::new(49429),
    module: "CreatureLocoShared",
    data: TypeInfoData::Enum,
    array_type: Some(CREATURESPEEDLEVEL_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CreatureSpeedLevel {
    fn type_info(&self) -> &'static TypeInfo {
        CREATURESPEEDLEVEL_TYPE_INFO
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


pub static CREATURESPEEDLEVEL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureSpeedLevel-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureSpeedLevel"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreatureWaypointsShapeData {
    pub _glacier_base: super::pathfinding_shared::WaypointsShapeData,
}

pub trait CreatureWaypointsShapeDataTrait: super::pathfinding_shared::WaypointsShapeDataTrait {
}

impl CreatureWaypointsShapeDataTrait for CreatureWaypointsShapeData {
}

impl super::pathfinding_shared::WaypointsShapeDataTrait for CreatureWaypointsShapeData {
    fn waypoints(&self) -> &Vec<Option<Arc<Mutex<dyn super::pathfinding_shared::WaypointDataTrait>>>> {
        self._glacier_base.waypoints()
    }
    fn waypoints_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::pathfinding_shared::WaypointDataTrait>>>> {
        self._glacier_base.waypoints_mut()
    }
}

impl super::entity::VectorShapeDataTrait for CreatureWaypointsShapeData {
    fn points(&self) -> &Vec<super::core::Vec3> {
        self._glacier_base.points()
    }
    fn points_mut(&mut self) -> &mut Vec<super::core::Vec3> {
        self._glacier_base.points_mut()
    }
    fn tension(&self) -> &f32 {
        self._glacier_base.tension()
    }
    fn tension_mut(&mut self) -> &mut f32 {
        self._glacier_base.tension_mut()
    }
    fn is_closed(&self) -> &bool {
        self._glacier_base.is_closed()
    }
    fn is_closed_mut(&mut self) -> &mut bool {
        self._glacier_base.is_closed_mut()
    }
    fn allow_roll(&self) -> &bool {
        self._glacier_base.allow_roll()
    }
    fn allow_roll_mut(&mut self) -> &mut bool {
        self._glacier_base.allow_roll_mut()
    }
    fn allow_yaw_pitch(&self) -> &bool {
        self._glacier_base.allow_yaw_pitch()
    }
    fn allow_yaw_pitch_mut(&mut self) -> &mut bool {
        self._glacier_base.allow_yaw_pitch_mut()
    }
}

impl super::entity::BaseShapeDataTrait for CreatureWaypointsShapeData {
}

impl super::entity::BaseShapeDataBaseTrait for CreatureWaypointsShapeData {
}

impl super::entity::GameObjectDataTrait for CreatureWaypointsShapeData {
}

impl super::core::DataBusPeerTrait for CreatureWaypointsShapeData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for CreatureWaypointsShapeData {
}

impl super::core::DataContainerTrait for CreatureWaypointsShapeData {
}

pub static CREATUREWAYPOINTSSHAPEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureWaypointsShapeData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::pathfinding_shared::WAYPOINTSSHAPEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureWaypointsShapeData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CREATUREWAYPOINTSSHAPEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CreatureWaypointsShapeData {
    fn type_info(&self) -> &'static TypeInfo {
        CREATUREWAYPOINTSSHAPEDATA_TYPE_INFO
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


pub static CREATUREWAYPOINTSSHAPEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureWaypointsShapeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureWaypointsShapeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreatureLocoSettings {
    pub _glacier_base: super::core::DataContainerPolicyAsset,
    pub state_settingss: Vec<Option<Arc<Mutex<dyn LocomotionStateSettingsTrait>>>>,
}

pub trait CreatureLocoSettingsTrait: super::core::DataContainerPolicyAssetTrait {
    fn state_settingss(&self) -> &Vec<Option<Arc<Mutex<dyn LocomotionStateSettingsTrait>>>>;
    fn state_settingss_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn LocomotionStateSettingsTrait>>>>;
}

impl CreatureLocoSettingsTrait for CreatureLocoSettings {
    fn state_settingss(&self) -> &Vec<Option<Arc<Mutex<dyn LocomotionStateSettingsTrait>>>> {
        &self.state_settingss
    }
    fn state_settingss_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn LocomotionStateSettingsTrait>>>> {
        &mut self.state_settingss
    }
}

impl super::core::DataContainerPolicyAssetTrait for CreatureLocoSettings {
}

impl super::core::AssetTrait for CreatureLocoSettings {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for CreatureLocoSettings {
}

pub static CREATURELOCOSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureLocoSettings",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINERPOLICYASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureLocoSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "StateSettingss",
                flags: MemberInfoFlags::new(144),
                field_type: "LocomotionStateSettings-Array",
                rust_offset: offset_of!(CreatureLocoSettings, state_settingss),
            },
        ],
    }),
    array_type: Some(CREATURELOCOSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CreatureLocoSettings {
    fn type_info(&self) -> &'static TypeInfo {
        CREATURELOCOSETTINGS_TYPE_INFO
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


pub static CREATURELOCOSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureLocoSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureLocoSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PushSettings {
    pub _glacier_base: LocomotionStateSettings,
    pub can_be_pushed_by: PushSettingsFilter,
    pub is_pushed_by_player: bool,
}

pub trait PushSettingsTrait: LocomotionStateSettingsTrait {
    fn can_be_pushed_by(&self) -> &PushSettingsFilter;
    fn can_be_pushed_by_mut(&mut self) -> &mut PushSettingsFilter;
    fn is_pushed_by_player(&self) -> &bool;
    fn is_pushed_by_player_mut(&mut self) -> &mut bool;
}

impl PushSettingsTrait for PushSettings {
    fn can_be_pushed_by(&self) -> &PushSettingsFilter {
        &self.can_be_pushed_by
    }
    fn can_be_pushed_by_mut(&mut self) -> &mut PushSettingsFilter {
        &mut self.can_be_pushed_by
    }
    fn is_pushed_by_player(&self) -> &bool {
        &self.is_pushed_by_player
    }
    fn is_pushed_by_player_mut(&mut self) -> &mut bool {
        &mut self.is_pushed_by_player
    }
}

impl LocomotionStateSettingsTrait for PushSettings {
}

impl super::core::DataContainerTrait for PushSettings {
}

pub static PUSHSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PushSettings",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONSTATESETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PushSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "CanBePushedBy",
                flags: MemberInfoFlags::new(0),
                field_type: "PushSettingsFilter",
                rust_offset: offset_of!(PushSettings, can_be_pushed_by),
            },
            FieldInfoData {
                name: "IsPushedByPlayer",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PushSettings, is_pushed_by_player),
            },
        ],
    }),
    array_type: Some(PUSHSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PushSettings {
    fn type_info(&self) -> &'static TypeInfo {
        PUSHSETTINGS_TYPE_INFO
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


pub static PUSHSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PushSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("PushSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum PushSettingsFilter {
    #[default]
    PushFilter_Everyone = 0,
    PushFilter_LargerThanMe = 1,
    PushFilter_EqualToOrLargerThanMe = 2,
    PushFilter_NoOne = 3,
}

pub static PUSHSETTINGSFILTER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PushSettingsFilter",
    flags: MemberInfoFlags::new(49429),
    module: "CreatureLocoShared",
    data: TypeInfoData::Enum,
    array_type: Some(PUSHSETTINGSFILTER_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PushSettingsFilter {
    fn type_info(&self) -> &'static TypeInfo {
        PUSHSETTINGSFILTER_TYPE_INFO
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


pub static PUSHSETTINGSFILTER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PushSettingsFilter-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("PushSettingsFilter"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SizeSettings {
    pub _glacier_base: LocomotionStateSettings,
    pub radius: f32,
    pub length: f32,
}

pub trait SizeSettingsTrait: LocomotionStateSettingsTrait {
    fn radius(&self) -> &f32;
    fn radius_mut(&mut self) -> &mut f32;
    fn length(&self) -> &f32;
    fn length_mut(&mut self) -> &mut f32;
}

impl SizeSettingsTrait for SizeSettings {
    fn radius(&self) -> &f32 {
        &self.radius
    }
    fn radius_mut(&mut self) -> &mut f32 {
        &mut self.radius
    }
    fn length(&self) -> &f32 {
        &self.length
    }
    fn length_mut(&mut self) -> &mut f32 {
        &mut self.length
    }
}

impl LocomotionStateSettingsTrait for SizeSettings {
}

impl super::core::DataContainerTrait for SizeSettings {
}

pub static SIZESETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SizeSettings",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONSTATESETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SizeSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SizeSettings, radius),
            },
            FieldInfoData {
                name: "Length",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SizeSettings, length),
            },
        ],
    }),
    array_type: Some(SIZESETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SizeSettings {
    fn type_info(&self) -> &'static TypeInfo {
        SIZESETTINGS_TYPE_INFO
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


pub static SIZESETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SizeSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("SizeSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WorldEventActionsSettings {
    pub _glacier_base: LocomotionStateSettings,
    pub event_actions: Vec<Option<Arc<Mutex<dyn WorldEventActionventParametersTrait>>>>,
}

pub trait WorldEventActionsSettingsTrait: LocomotionStateSettingsTrait {
    fn event_actions(&self) -> &Vec<Option<Arc<Mutex<dyn WorldEventActionventParametersTrait>>>>;
    fn event_actions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn WorldEventActionventParametersTrait>>>>;
}

impl WorldEventActionsSettingsTrait for WorldEventActionsSettings {
    fn event_actions(&self) -> &Vec<Option<Arc<Mutex<dyn WorldEventActionventParametersTrait>>>> {
        &self.event_actions
    }
    fn event_actions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn WorldEventActionventParametersTrait>>>> {
        &mut self.event_actions
    }
}

impl LocomotionStateSettingsTrait for WorldEventActionsSettings {
}

impl super::core::DataContainerTrait for WorldEventActionsSettings {
}

pub static WORLDEVENTACTIONSSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldEventActionsSettings",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONSTATESETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WorldEventActionsSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "EventActions",
                flags: MemberInfoFlags::new(144),
                field_type: "WorldEventActionventParameters-Array",
                rust_offset: offset_of!(WorldEventActionsSettings, event_actions),
            },
        ],
    }),
    array_type: Some(WORLDEVENTACTIONSSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WorldEventActionsSettings {
    fn type_info(&self) -> &'static TypeInfo {
        WORLDEVENTACTIONSSETTINGS_TYPE_INFO
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


pub static WORLDEVENTACTIONSSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldEventActionsSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("WorldEventActionsSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WorldEventActionventParameters {
    pub _glacier_base: super::core::DataContainer,
    pub event_type: CreatureLocoExternalInfluenceType,
    pub consideration_range: f32,
    pub minimum_number: i32,
    pub time_window: f32,
    pub probability_of_action: f32,
    pub alignment_rate: f32,
    pub minimum_impact_radius: f32,
    pub fake_physiscs: bool,
    pub fake_mass: f32,
    pub stop_delay: f32,
    pub cooldown_time: f32,
    pub action_type: CreatureLocoExternalInfluenceReactionType,
    pub action_alignment: CreatureLocoExternalInfluenceReactionAlignment,
}

pub trait WorldEventActionventParametersTrait: super::core::DataContainerTrait {
    fn event_type(&self) -> &CreatureLocoExternalInfluenceType;
    fn event_type_mut(&mut self) -> &mut CreatureLocoExternalInfluenceType;
    fn consideration_range(&self) -> &f32;
    fn consideration_range_mut(&mut self) -> &mut f32;
    fn minimum_number(&self) -> &i32;
    fn minimum_number_mut(&mut self) -> &mut i32;
    fn time_window(&self) -> &f32;
    fn time_window_mut(&mut self) -> &mut f32;
    fn probability_of_action(&self) -> &f32;
    fn probability_of_action_mut(&mut self) -> &mut f32;
    fn alignment_rate(&self) -> &f32;
    fn alignment_rate_mut(&mut self) -> &mut f32;
    fn minimum_impact_radius(&self) -> &f32;
    fn minimum_impact_radius_mut(&mut self) -> &mut f32;
    fn fake_physiscs(&self) -> &bool;
    fn fake_physiscs_mut(&mut self) -> &mut bool;
    fn fake_mass(&self) -> &f32;
    fn fake_mass_mut(&mut self) -> &mut f32;
    fn stop_delay(&self) -> &f32;
    fn stop_delay_mut(&mut self) -> &mut f32;
    fn cooldown_time(&self) -> &f32;
    fn cooldown_time_mut(&mut self) -> &mut f32;
    fn action_type(&self) -> &CreatureLocoExternalInfluenceReactionType;
    fn action_type_mut(&mut self) -> &mut CreatureLocoExternalInfluenceReactionType;
    fn action_alignment(&self) -> &CreatureLocoExternalInfluenceReactionAlignment;
    fn action_alignment_mut(&mut self) -> &mut CreatureLocoExternalInfluenceReactionAlignment;
}

impl WorldEventActionventParametersTrait for WorldEventActionventParameters {
    fn event_type(&self) -> &CreatureLocoExternalInfluenceType {
        &self.event_type
    }
    fn event_type_mut(&mut self) -> &mut CreatureLocoExternalInfluenceType {
        &mut self.event_type
    }
    fn consideration_range(&self) -> &f32 {
        &self.consideration_range
    }
    fn consideration_range_mut(&mut self) -> &mut f32 {
        &mut self.consideration_range
    }
    fn minimum_number(&self) -> &i32 {
        &self.minimum_number
    }
    fn minimum_number_mut(&mut self) -> &mut i32 {
        &mut self.minimum_number
    }
    fn time_window(&self) -> &f32 {
        &self.time_window
    }
    fn time_window_mut(&mut self) -> &mut f32 {
        &mut self.time_window
    }
    fn probability_of_action(&self) -> &f32 {
        &self.probability_of_action
    }
    fn probability_of_action_mut(&mut self) -> &mut f32 {
        &mut self.probability_of_action
    }
    fn alignment_rate(&self) -> &f32 {
        &self.alignment_rate
    }
    fn alignment_rate_mut(&mut self) -> &mut f32 {
        &mut self.alignment_rate
    }
    fn minimum_impact_radius(&self) -> &f32 {
        &self.minimum_impact_radius
    }
    fn minimum_impact_radius_mut(&mut self) -> &mut f32 {
        &mut self.minimum_impact_radius
    }
    fn fake_physiscs(&self) -> &bool {
        &self.fake_physiscs
    }
    fn fake_physiscs_mut(&mut self) -> &mut bool {
        &mut self.fake_physiscs
    }
    fn fake_mass(&self) -> &f32 {
        &self.fake_mass
    }
    fn fake_mass_mut(&mut self) -> &mut f32 {
        &mut self.fake_mass
    }
    fn stop_delay(&self) -> &f32 {
        &self.stop_delay
    }
    fn stop_delay_mut(&mut self) -> &mut f32 {
        &mut self.stop_delay
    }
    fn cooldown_time(&self) -> &f32 {
        &self.cooldown_time
    }
    fn cooldown_time_mut(&mut self) -> &mut f32 {
        &mut self.cooldown_time
    }
    fn action_type(&self) -> &CreatureLocoExternalInfluenceReactionType {
        &self.action_type
    }
    fn action_type_mut(&mut self) -> &mut CreatureLocoExternalInfluenceReactionType {
        &mut self.action_type
    }
    fn action_alignment(&self) -> &CreatureLocoExternalInfluenceReactionAlignment {
        &self.action_alignment
    }
    fn action_alignment_mut(&mut self) -> &mut CreatureLocoExternalInfluenceReactionAlignment {
        &mut self.action_alignment
    }
}

impl super::core::DataContainerTrait for WorldEventActionventParameters {
}

pub static WORLDEVENTACTIONVENTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldEventActionventParameters",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WorldEventActionventParameters as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "EventType",
                flags: MemberInfoFlags::new(0),
                field_type: "CreatureLocoExternalInfluenceType",
                rust_offset: offset_of!(WorldEventActionventParameters, event_type),
            },
            FieldInfoData {
                name: "ConsiderationRange",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldEventActionventParameters, consideration_range),
            },
            FieldInfoData {
                name: "MinimumNumber",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(WorldEventActionventParameters, minimum_number),
            },
            FieldInfoData {
                name: "TimeWindow",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldEventActionventParameters, time_window),
            },
            FieldInfoData {
                name: "ProbabilityOfAction",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldEventActionventParameters, probability_of_action),
            },
            FieldInfoData {
                name: "AlignmentRate",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldEventActionventParameters, alignment_rate),
            },
            FieldInfoData {
                name: "MinimumImpactRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldEventActionventParameters, minimum_impact_radius),
            },
            FieldInfoData {
                name: "fakePhysiscs",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldEventActionventParameters, fake_physiscs),
            },
            FieldInfoData {
                name: "FakeMass",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldEventActionventParameters, fake_mass),
            },
            FieldInfoData {
                name: "StopDelay",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldEventActionventParameters, stop_delay),
            },
            FieldInfoData {
                name: "CooldownTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldEventActionventParameters, cooldown_time),
            },
            FieldInfoData {
                name: "ActionType",
                flags: MemberInfoFlags::new(0),
                field_type: "CreatureLocoExternalInfluenceReactionType",
                rust_offset: offset_of!(WorldEventActionventParameters, action_type),
            },
            FieldInfoData {
                name: "ActionAlignment",
                flags: MemberInfoFlags::new(0),
                field_type: "CreatureLocoExternalInfluenceReactionAlignment",
                rust_offset: offset_of!(WorldEventActionventParameters, action_alignment),
            },
        ],
    }),
    array_type: Some(WORLDEVENTACTIONVENTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WorldEventActionventParameters {
    fn type_info(&self) -> &'static TypeInfo {
        WORLDEVENTACTIONVENTPARAMETERS_TYPE_INFO
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


pub static WORLDEVENTACTIONVENTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldEventActionventParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("WorldEventActionventParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum CreatureLocoExternalInfluenceReactionAlignment {
    #[default]
    ExternalInfluence_AlignNone = 0,
    ExternalInfluence_AlignTowards = 1,
    ExternalInfluence_AlignAway = 2,
}

pub static CREATURELOCOEXTERNALINFLUENCEREACTIONALIGNMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureLocoExternalInfluenceReactionAlignment",
    flags: MemberInfoFlags::new(49429),
    module: "CreatureLocoShared",
    data: TypeInfoData::Enum,
    array_type: Some(CREATURELOCOEXTERNALINFLUENCEREACTIONALIGNMENT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CreatureLocoExternalInfluenceReactionAlignment {
    fn type_info(&self) -> &'static TypeInfo {
        CREATURELOCOEXTERNALINFLUENCEREACTIONALIGNMENT_TYPE_INFO
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


pub static CREATURELOCOEXTERNALINFLUENCEREACTIONALIGNMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureLocoExternalInfluenceReactionAlignment-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureLocoExternalInfluenceReactionAlignment"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum CreatureLocoExternalInfluenceReactionType {
    #[default]
    ExternalInfluence_Act = 0,
    ExternalInfluence_StopAndAct = 1,
}

pub static CREATURELOCOEXTERNALINFLUENCEREACTIONTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureLocoExternalInfluenceReactionType",
    flags: MemberInfoFlags::new(49429),
    module: "CreatureLocoShared",
    data: TypeInfoData::Enum,
    array_type: Some(CREATURELOCOEXTERNALINFLUENCEREACTIONTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CreatureLocoExternalInfluenceReactionType {
    fn type_info(&self) -> &'static TypeInfo {
        CREATURELOCOEXTERNALINFLUENCEREACTIONTYPE_TYPE_INFO
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


pub static CREATURELOCOEXTERNALINFLUENCEREACTIONTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureLocoExternalInfluenceReactionType-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureLocoExternalInfluenceReactionType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum CreatureLocoExternalInfluenceType {
    #[default]
    ExternalInfluence_ShotHit = 0,
    ExternalInfluence_ExplosionHit = 1,
    ExternalInfluence_HumanPlayer = 2,
    ExternalInfluence_PushAway = 3,
    ExternalInfluence_PullIn = 4,
    ExternalInfluence_Freeze = 5,
    ExternalInfluence_UnknownEvent = 6,
    ExternalInfluence_Stun = 7,
    ExternalInfluence_StunMachines = 8,
    ExternalInfluence_NonHumanNear = 9,
    ExternalInfluence_Shoved = 10,
    ExternalInfluence_Count = 11,
}

pub static CREATURELOCOEXTERNALINFLUENCETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureLocoExternalInfluenceType",
    flags: MemberInfoFlags::new(49429),
    module: "CreatureLocoShared",
    data: TypeInfoData::Enum,
    array_type: Some(CREATURELOCOEXTERNALINFLUENCETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CreatureLocoExternalInfluenceType {
    fn type_info(&self) -> &'static TypeInfo {
        CREATURELOCOEXTERNALINFLUENCETYPE_TYPE_INFO
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


pub static CREATURELOCOEXTERNALINFLUENCETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureLocoExternalInfluenceType-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureLocoExternalInfluenceType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ProceduralMovementStateSettings {
    pub _glacier_base: LocomotionStateSettings,
    pub slow_speed: LocoStateSpeedRange,
    pub medium_speed: LocoStateSpeedRange,
    pub fast_speed: LocoStateSpeedRange,
    pub acceleration: f32,
    pub deceleration: f32,
    pub forward_speed_rate: f32,
    pub lateral_speed_rate: f32,
    pub reverse_speed_rate: f32,
    pub height_change_on_centerpoint: bool,
}

pub trait ProceduralMovementStateSettingsTrait: LocomotionStateSettingsTrait {
    fn slow_speed(&self) -> &LocoStateSpeedRange;
    fn slow_speed_mut(&mut self) -> &mut LocoStateSpeedRange;
    fn medium_speed(&self) -> &LocoStateSpeedRange;
    fn medium_speed_mut(&mut self) -> &mut LocoStateSpeedRange;
    fn fast_speed(&self) -> &LocoStateSpeedRange;
    fn fast_speed_mut(&mut self) -> &mut LocoStateSpeedRange;
    fn acceleration(&self) -> &f32;
    fn acceleration_mut(&mut self) -> &mut f32;
    fn deceleration(&self) -> &f32;
    fn deceleration_mut(&mut self) -> &mut f32;
    fn forward_speed_rate(&self) -> &f32;
    fn forward_speed_rate_mut(&mut self) -> &mut f32;
    fn lateral_speed_rate(&self) -> &f32;
    fn lateral_speed_rate_mut(&mut self) -> &mut f32;
    fn reverse_speed_rate(&self) -> &f32;
    fn reverse_speed_rate_mut(&mut self) -> &mut f32;
    fn height_change_on_centerpoint(&self) -> &bool;
    fn height_change_on_centerpoint_mut(&mut self) -> &mut bool;
}

impl ProceduralMovementStateSettingsTrait for ProceduralMovementStateSettings {
    fn slow_speed(&self) -> &LocoStateSpeedRange {
        &self.slow_speed
    }
    fn slow_speed_mut(&mut self) -> &mut LocoStateSpeedRange {
        &mut self.slow_speed
    }
    fn medium_speed(&self) -> &LocoStateSpeedRange {
        &self.medium_speed
    }
    fn medium_speed_mut(&mut self) -> &mut LocoStateSpeedRange {
        &mut self.medium_speed
    }
    fn fast_speed(&self) -> &LocoStateSpeedRange {
        &self.fast_speed
    }
    fn fast_speed_mut(&mut self) -> &mut LocoStateSpeedRange {
        &mut self.fast_speed
    }
    fn acceleration(&self) -> &f32 {
        &self.acceleration
    }
    fn acceleration_mut(&mut self) -> &mut f32 {
        &mut self.acceleration
    }
    fn deceleration(&self) -> &f32 {
        &self.deceleration
    }
    fn deceleration_mut(&mut self) -> &mut f32 {
        &mut self.deceleration
    }
    fn forward_speed_rate(&self) -> &f32 {
        &self.forward_speed_rate
    }
    fn forward_speed_rate_mut(&mut self) -> &mut f32 {
        &mut self.forward_speed_rate
    }
    fn lateral_speed_rate(&self) -> &f32 {
        &self.lateral_speed_rate
    }
    fn lateral_speed_rate_mut(&mut self) -> &mut f32 {
        &mut self.lateral_speed_rate
    }
    fn reverse_speed_rate(&self) -> &f32 {
        &self.reverse_speed_rate
    }
    fn reverse_speed_rate_mut(&mut self) -> &mut f32 {
        &mut self.reverse_speed_rate
    }
    fn height_change_on_centerpoint(&self) -> &bool {
        &self.height_change_on_centerpoint
    }
    fn height_change_on_centerpoint_mut(&mut self) -> &mut bool {
        &mut self.height_change_on_centerpoint
    }
}

impl LocomotionStateSettingsTrait for ProceduralMovementStateSettings {
}

impl super::core::DataContainerTrait for ProceduralMovementStateSettings {
}

pub static PROCEDURALMOVEMENTSTATESETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProceduralMovementStateSettings",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONSTATESETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ProceduralMovementStateSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "SlowSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: "LocoStateSpeedRange",
                rust_offset: offset_of!(ProceduralMovementStateSettings, slow_speed),
            },
            FieldInfoData {
                name: "MediumSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: "LocoStateSpeedRange",
                rust_offset: offset_of!(ProceduralMovementStateSettings, medium_speed),
            },
            FieldInfoData {
                name: "FastSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: "LocoStateSpeedRange",
                rust_offset: offset_of!(ProceduralMovementStateSettings, fast_speed),
            },
            FieldInfoData {
                name: "Acceleration",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ProceduralMovementStateSettings, acceleration),
            },
            FieldInfoData {
                name: "Deceleration",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ProceduralMovementStateSettings, deceleration),
            },
            FieldInfoData {
                name: "ForwardSpeedRate",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ProceduralMovementStateSettings, forward_speed_rate),
            },
            FieldInfoData {
                name: "LateralSpeedRate",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ProceduralMovementStateSettings, lateral_speed_rate),
            },
            FieldInfoData {
                name: "ReverseSpeedRate",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ProceduralMovementStateSettings, reverse_speed_rate),
            },
            FieldInfoData {
                name: "HeightChangeOnCenterpoint",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ProceduralMovementStateSettings, height_change_on_centerpoint),
            },
        ],
    }),
    array_type: Some(PROCEDURALMOVEMENTSTATESETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProceduralMovementStateSettings {
    fn type_info(&self) -> &'static TypeInfo {
        PROCEDURALMOVEMENTSTATESETTINGS_TYPE_INFO
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


pub static PROCEDURALMOVEMENTSTATESETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProceduralMovementStateSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("ProceduralMovementStateSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LocoStateSpeedRange {
    pub max_speed: f32,
    pub min_speed: f32,
}

pub trait LocoStateSpeedRangeTrait: TypeObject {
    fn max_speed(&self) -> &f32;
    fn max_speed_mut(&mut self) -> &mut f32;
    fn min_speed(&self) -> &f32;
    fn min_speed_mut(&mut self) -> &mut f32;
}

impl LocoStateSpeedRangeTrait for LocoStateSpeedRange {
    fn max_speed(&self) -> &f32 {
        &self.max_speed
    }
    fn max_speed_mut(&mut self) -> &mut f32 {
        &mut self.max_speed
    }
    fn min_speed(&self) -> &f32 {
        &self.min_speed
    }
    fn min_speed_mut(&mut self) -> &mut f32 {
        &mut self.min_speed
    }
}

pub static LOCOSTATESPEEDRANGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocoStateSpeedRange",
    flags: MemberInfoFlags::new(36937),
    module: "CreatureLocoShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LocoStateSpeedRange as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MaxSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LocoStateSpeedRange, max_speed),
            },
            FieldInfoData {
                name: "MinSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LocoStateSpeedRange, min_speed),
            },
        ],
    }),
    array_type: Some(LOCOSTATESPEEDRANGE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for LocoStateSpeedRange {
    fn type_info(&self) -> &'static TypeInfo {
        LOCOSTATESPEEDRANGE_TYPE_INFO
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


pub static LOCOSTATESPEEDRANGE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocoStateSpeedRange-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("LocoStateSpeedRange"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AvoidanceSteeringSettings {
    pub _glacier_base: LocomotionStateSettings,
    pub settings_data: AvoidanceData,
}

pub trait AvoidanceSteeringSettingsTrait: LocomotionStateSettingsTrait {
    fn settings_data(&self) -> &AvoidanceData;
    fn settings_data_mut(&mut self) -> &mut AvoidanceData;
}

impl AvoidanceSteeringSettingsTrait for AvoidanceSteeringSettings {
    fn settings_data(&self) -> &AvoidanceData {
        &self.settings_data
    }
    fn settings_data_mut(&mut self) -> &mut AvoidanceData {
        &mut self.settings_data
    }
}

impl LocomotionStateSettingsTrait for AvoidanceSteeringSettings {
}

impl super::core::DataContainerTrait for AvoidanceSteeringSettings {
}

pub static AVOIDANCESTEERINGSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AvoidanceSteeringSettings",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONSTATESETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AvoidanceSteeringSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "SettingsData",
                flags: MemberInfoFlags::new(0),
                field_type: "AvoidanceData",
                rust_offset: offset_of!(AvoidanceSteeringSettings, settings_data),
            },
        ],
    }),
    array_type: Some(AVOIDANCESTEERINGSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AvoidanceSteeringSettings {
    fn type_info(&self) -> &'static TypeInfo {
        AVOIDANCESTEERINGSETTINGS_TYPE_INFO
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


pub static AVOIDANCESTEERINGSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AvoidanceSteeringSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("AvoidanceSteeringSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AvoidanceData {
    pub time_to_consider: f32,
    pub obstical_width: f32,
    pub avoids_weapons_fire: bool,
    pub behaviour_towards_players: AIAvoidancePlayerBehaviour,
    pub avoidance_cone_width: f32,
    pub max_repulsion_weight: f32,
    pub repulsion_gain_rate: f32,
    pub repulsion_decay_rate: f32,
}

pub trait AvoidanceDataTrait: TypeObject {
    fn time_to_consider(&self) -> &f32;
    fn time_to_consider_mut(&mut self) -> &mut f32;
    fn obstical_width(&self) -> &f32;
    fn obstical_width_mut(&mut self) -> &mut f32;
    fn avoids_weapons_fire(&self) -> &bool;
    fn avoids_weapons_fire_mut(&mut self) -> &mut bool;
    fn behaviour_towards_players(&self) -> &AIAvoidancePlayerBehaviour;
    fn behaviour_towards_players_mut(&mut self) -> &mut AIAvoidancePlayerBehaviour;
    fn avoidance_cone_width(&self) -> &f32;
    fn avoidance_cone_width_mut(&mut self) -> &mut f32;
    fn max_repulsion_weight(&self) -> &f32;
    fn max_repulsion_weight_mut(&mut self) -> &mut f32;
    fn repulsion_gain_rate(&self) -> &f32;
    fn repulsion_gain_rate_mut(&mut self) -> &mut f32;
    fn repulsion_decay_rate(&self) -> &f32;
    fn repulsion_decay_rate_mut(&mut self) -> &mut f32;
}

impl AvoidanceDataTrait for AvoidanceData {
    fn time_to_consider(&self) -> &f32 {
        &self.time_to_consider
    }
    fn time_to_consider_mut(&mut self) -> &mut f32 {
        &mut self.time_to_consider
    }
    fn obstical_width(&self) -> &f32 {
        &self.obstical_width
    }
    fn obstical_width_mut(&mut self) -> &mut f32 {
        &mut self.obstical_width
    }
    fn avoids_weapons_fire(&self) -> &bool {
        &self.avoids_weapons_fire
    }
    fn avoids_weapons_fire_mut(&mut self) -> &mut bool {
        &mut self.avoids_weapons_fire
    }
    fn behaviour_towards_players(&self) -> &AIAvoidancePlayerBehaviour {
        &self.behaviour_towards_players
    }
    fn behaviour_towards_players_mut(&mut self) -> &mut AIAvoidancePlayerBehaviour {
        &mut self.behaviour_towards_players
    }
    fn avoidance_cone_width(&self) -> &f32 {
        &self.avoidance_cone_width
    }
    fn avoidance_cone_width_mut(&mut self) -> &mut f32 {
        &mut self.avoidance_cone_width
    }
    fn max_repulsion_weight(&self) -> &f32 {
        &self.max_repulsion_weight
    }
    fn max_repulsion_weight_mut(&mut self) -> &mut f32 {
        &mut self.max_repulsion_weight
    }
    fn repulsion_gain_rate(&self) -> &f32 {
        &self.repulsion_gain_rate
    }
    fn repulsion_gain_rate_mut(&mut self) -> &mut f32 {
        &mut self.repulsion_gain_rate
    }
    fn repulsion_decay_rate(&self) -> &f32 {
        &self.repulsion_decay_rate
    }
    fn repulsion_decay_rate_mut(&mut self) -> &mut f32 {
        &mut self.repulsion_decay_rate
    }
}

pub static AVOIDANCEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AvoidanceData",
    flags: MemberInfoFlags::new(36937),
    module: "CreatureLocoShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AvoidanceData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TimeToConsider",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AvoidanceData, time_to_consider),
            },
            FieldInfoData {
                name: "ObsticalWidth",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AvoidanceData, obstical_width),
            },
            FieldInfoData {
                name: "AvoidsWeaponsFire",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AvoidanceData, avoids_weapons_fire),
            },
            FieldInfoData {
                name: "BehaviourTowardsPlayers",
                flags: MemberInfoFlags::new(0),
                field_type: "AIAvoidancePlayerBehaviour",
                rust_offset: offset_of!(AvoidanceData, behaviour_towards_players),
            },
            FieldInfoData {
                name: "AvoidanceConeWidth",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AvoidanceData, avoidance_cone_width),
            },
            FieldInfoData {
                name: "MaxRepulsionWeight",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AvoidanceData, max_repulsion_weight),
            },
            FieldInfoData {
                name: "RepulsionGainRate",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AvoidanceData, repulsion_gain_rate),
            },
            FieldInfoData {
                name: "RepulsionDecayRate",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AvoidanceData, repulsion_decay_rate),
            },
        ],
    }),
    array_type: Some(AVOIDANCEDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for AvoidanceData {
    fn type_info(&self) -> &'static TypeInfo {
        AVOIDANCEDATA_TYPE_INFO
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


pub static AVOIDANCEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AvoidanceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("AvoidanceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum AIAvoidancePlayerBehaviour {
    #[default]
    IGNORES = 0,
    AVOIDS = 1,
    APPROACHES = 2,
}

pub static AIAVOIDANCEPLAYERBEHAVIOUR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIAvoidancePlayerBehaviour",
    flags: MemberInfoFlags::new(49429),
    module: "CreatureLocoShared",
    data: TypeInfoData::Enum,
    array_type: Some(AIAVOIDANCEPLAYERBEHAVIOUR_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AIAvoidancePlayerBehaviour {
    fn type_info(&self) -> &'static TypeInfo {
        AIAVOIDANCEPLAYERBEHAVIOUR_TYPE_INFO
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


pub static AIAVOIDANCEPLAYERBEHAVIOUR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIAvoidancePlayerBehaviour-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("AIAvoidancePlayerBehaviour"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CurveSteeringSettings {
    pub _glacier_base: LocomotionStateSettings,
    pub base_settings_data: BasicSteeringSettingsData,
    pub settings_data: CurveSteeringSettingsData,
}

pub trait CurveSteeringSettingsTrait: LocomotionStateSettingsTrait {
    fn base_settings_data(&self) -> &BasicSteeringSettingsData;
    fn base_settings_data_mut(&mut self) -> &mut BasicSteeringSettingsData;
    fn settings_data(&self) -> &CurveSteeringSettingsData;
    fn settings_data_mut(&mut self) -> &mut CurveSteeringSettingsData;
}

impl CurveSteeringSettingsTrait for CurveSteeringSettings {
    fn base_settings_data(&self) -> &BasicSteeringSettingsData {
        &self.base_settings_data
    }
    fn base_settings_data_mut(&mut self) -> &mut BasicSteeringSettingsData {
        &mut self.base_settings_data
    }
    fn settings_data(&self) -> &CurveSteeringSettingsData {
        &self.settings_data
    }
    fn settings_data_mut(&mut self) -> &mut CurveSteeringSettingsData {
        &mut self.settings_data
    }
}

impl LocomotionStateSettingsTrait for CurveSteeringSettings {
}

impl super::core::DataContainerTrait for CurveSteeringSettings {
}

pub static CURVESTEERINGSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CurveSteeringSettings",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONSTATESETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CurveSteeringSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "BaseSettingsData",
                flags: MemberInfoFlags::new(0),
                field_type: "BasicSteeringSettingsData",
                rust_offset: offset_of!(CurveSteeringSettings, base_settings_data),
            },
            FieldInfoData {
                name: "SettingsData",
                flags: MemberInfoFlags::new(0),
                field_type: "CurveSteeringSettingsData",
                rust_offset: offset_of!(CurveSteeringSettings, settings_data),
            },
        ],
    }),
    array_type: Some(CURVESTEERINGSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CurveSteeringSettings {
    fn type_info(&self) -> &'static TypeInfo {
        CURVESTEERINGSETTINGS_TYPE_INFO
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


pub static CURVESTEERINGSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CurveSteeringSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CurveSteeringSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CurveSteeringSettingsData {
    pub foo: bool,
}

pub trait CurveSteeringSettingsDataTrait: TypeObject {
    fn foo(&self) -> &bool;
    fn foo_mut(&mut self) -> &mut bool;
}

impl CurveSteeringSettingsDataTrait for CurveSteeringSettingsData {
    fn foo(&self) -> &bool {
        &self.foo
    }
    fn foo_mut(&mut self) -> &mut bool {
        &mut self.foo
    }
}

pub static CURVESTEERINGSETTINGSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CurveSteeringSettingsData",
    flags: MemberInfoFlags::new(36937),
    module: "CreatureLocoShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CurveSteeringSettingsData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "foo",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CurveSteeringSettingsData, foo),
            },
        ],
    }),
    array_type: Some(CURVESTEERINGSETTINGSDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CurveSteeringSettingsData {
    fn type_info(&self) -> &'static TypeInfo {
        CURVESTEERINGSETTINGSDATA_TYPE_INFO
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


pub static CURVESTEERINGSETTINGSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CurveSteeringSettingsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CurveSteeringSettingsData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BasicSteeringSettingsData {
    pub response_time: f32,
    pub maximum_angle_deflection: f32,
    pub error_distance: f32,
}

pub trait BasicSteeringSettingsDataTrait: TypeObject {
    fn response_time(&self) -> &f32;
    fn response_time_mut(&mut self) -> &mut f32;
    fn maximum_angle_deflection(&self) -> &f32;
    fn maximum_angle_deflection_mut(&mut self) -> &mut f32;
    fn error_distance(&self) -> &f32;
    fn error_distance_mut(&mut self) -> &mut f32;
}

impl BasicSteeringSettingsDataTrait for BasicSteeringSettingsData {
    fn response_time(&self) -> &f32 {
        &self.response_time
    }
    fn response_time_mut(&mut self) -> &mut f32 {
        &mut self.response_time
    }
    fn maximum_angle_deflection(&self) -> &f32 {
        &self.maximum_angle_deflection
    }
    fn maximum_angle_deflection_mut(&mut self) -> &mut f32 {
        &mut self.maximum_angle_deflection
    }
    fn error_distance(&self) -> &f32 {
        &self.error_distance
    }
    fn error_distance_mut(&mut self) -> &mut f32 {
        &mut self.error_distance
    }
}

pub static BASICSTEERINGSETTINGSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BasicSteeringSettingsData",
    flags: MemberInfoFlags::new(36937),
    module: "CreatureLocoShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BasicSteeringSettingsData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ResponseTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BasicSteeringSettingsData, response_time),
            },
            FieldInfoData {
                name: "MaximumAngleDeflection",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BasicSteeringSettingsData, maximum_angle_deflection),
            },
            FieldInfoData {
                name: "ErrorDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BasicSteeringSettingsData, error_distance),
            },
        ],
    }),
    array_type: Some(BASICSTEERINGSETTINGSDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for BasicSteeringSettingsData {
    fn type_info(&self) -> &'static TypeInfo {
        BASICSTEERINGSETTINGSDATA_TYPE_INFO
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


pub static BASICSTEERINGSETTINGSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BasicSteeringSettingsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("BasicSteeringSettingsData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CommonClientSettings {
    pub _glacier_base: LocomotionStateSettings,
}

pub trait CommonClientSettingsTrait: LocomotionStateSettingsTrait {
}

impl CommonClientSettingsTrait for CommonClientSettings {
}

impl LocomotionStateSettingsTrait for CommonClientSettings {
}

impl super::core::DataContainerTrait for CommonClientSettings {
}

pub static COMMONCLIENTSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CommonClientSettings",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONSTATESETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CommonClientSettings as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(COMMONCLIENTSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CommonClientSettings {
    fn type_info(&self) -> &'static TypeInfo {
        COMMONCLIENTSETTINGS_TYPE_INFO
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


pub static COMMONCLIENTSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CommonClientSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CommonClientSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PlayAnimSettings {
    pub _glacier_base: LocomotionStateSettings,
}

pub trait PlayAnimSettingsTrait: LocomotionStateSettingsTrait {
}

impl PlayAnimSettingsTrait for PlayAnimSettings {
}

impl LocomotionStateSettingsTrait for PlayAnimSettings {
}

impl super::core::DataContainerTrait for PlayAnimSettings {
}

pub static PLAYANIMSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayAnimSettings",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONSTATESETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PlayAnimSettings as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PLAYANIMSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PlayAnimSettings {
    fn type_info(&self) -> &'static TypeInfo {
        PLAYANIMSETTINGS_TYPE_INFO
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


pub static PLAYANIMSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayAnimSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("PlayAnimSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TurnSettings {
    pub _glacier_base: LocomotionStateSettings,
}

pub trait TurnSettingsTrait: LocomotionStateSettingsTrait {
}

impl TurnSettingsTrait for TurnSettings {
}

impl LocomotionStateSettingsTrait for TurnSettings {
}

impl super::core::DataContainerTrait for TurnSettings {
}

pub static TURNSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TurnSettings",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONSTATESETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TurnSettings as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TURNSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TurnSettings {
    fn type_info(&self) -> &'static TypeInfo {
        TURNSETTINGS_TYPE_INFO
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


pub static TURNSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TurnSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("TurnSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct StopSettings {
    pub _glacier_base: LocomotionStateSettings,
    pub base_deceleration_rate: f32,
}

pub trait StopSettingsTrait: LocomotionStateSettingsTrait {
    fn base_deceleration_rate(&self) -> &f32;
    fn base_deceleration_rate_mut(&mut self) -> &mut f32;
}

impl StopSettingsTrait for StopSettings {
    fn base_deceleration_rate(&self) -> &f32 {
        &self.base_deceleration_rate
    }
    fn base_deceleration_rate_mut(&mut self) -> &mut f32 {
        &mut self.base_deceleration_rate
    }
}

impl LocomotionStateSettingsTrait for StopSettings {
}

impl super::core::DataContainerTrait for StopSettings {
}

pub static STOPSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StopSettings",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONSTATESETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StopSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "BaseDecelerationRate",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(StopSettings, base_deceleration_rate),
            },
        ],
    }),
    array_type: Some(STOPSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StopSettings {
    fn type_info(&self) -> &'static TypeInfo {
        STOPSETTINGS_TYPE_INFO
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


pub static STOPSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StopSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("StopSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct StartSettings {
    pub _glacier_base: LocomotionStateSettings,
    pub base_acceleration_rate: f32,
}

pub trait StartSettingsTrait: LocomotionStateSettingsTrait {
    fn base_acceleration_rate(&self) -> &f32;
    fn base_acceleration_rate_mut(&mut self) -> &mut f32;
}

impl StartSettingsTrait for StartSettings {
    fn base_acceleration_rate(&self) -> &f32 {
        &self.base_acceleration_rate
    }
    fn base_acceleration_rate_mut(&mut self) -> &mut f32 {
        &mut self.base_acceleration_rate
    }
}

impl LocomotionStateSettingsTrait for StartSettings {
}

impl super::core::DataContainerTrait for StartSettings {
}

pub static STARTSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StartSettings",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONSTATESETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StartSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "BaseAccelerationRate",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(StartSettings, base_acceleration_rate),
            },
        ],
    }),
    array_type: Some(STARTSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StartSettings {
    fn type_info(&self) -> &'static TypeInfo {
        STARTSETTINGS_TYPE_INFO
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


pub static STARTSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StartSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("StartSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MoveCycleSettings {
    pub _glacier_base: LocomotionStateSettings,
}

pub trait MoveCycleSettingsTrait: LocomotionStateSettingsTrait {
}

impl MoveCycleSettingsTrait for MoveCycleSettings {
}

impl LocomotionStateSettingsTrait for MoveCycleSettings {
}

impl super::core::DataContainerTrait for MoveCycleSettings {
}

pub static MOVECYCLESETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MoveCycleSettings",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONSTATESETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MoveCycleSettings as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(MOVECYCLESETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MoveCycleSettings {
    fn type_info(&self) -> &'static TypeInfo {
        MOVECYCLESETTINGS_TYPE_INFO
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


pub static MOVECYCLESETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MoveCycleSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("MoveCycleSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct IdleSettings {
    pub _glacier_base: LocomotionStateSettings,
}

pub trait IdleSettingsTrait: LocomotionStateSettingsTrait {
}

impl IdleSettingsTrait for IdleSettings {
}

impl LocomotionStateSettingsTrait for IdleSettings {
}

impl super::core::DataContainerTrait for IdleSettings {
}

pub static IDLESETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IdleSettings",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONSTATESETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IdleSettings as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(IDLESETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IdleSettings {
    fn type_info(&self) -> &'static TypeInfo {
        IDLESETTINGS_TYPE_INFO
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


pub static IDLESETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IdleSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("IdleSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LocomotionStateSettings {
    pub _glacier_base: super::core::DataContainer,
}

pub trait LocomotionStateSettingsTrait: super::core::DataContainerTrait {
}

impl LocomotionStateSettingsTrait for LocomotionStateSettings {
}

impl super::core::DataContainerTrait for LocomotionStateSettings {
}

pub static LOCOMOTIONSTATESETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocomotionStateSettings",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LocomotionStateSettings as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LOCOMOTIONSTATESETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LocomotionStateSettings {
    fn type_info(&self) -> &'static TypeInfo {
        LOCOMOTIONSTATESETTINGS_TYPE_INFO
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


pub static LOCOMOTIONSTATESETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocomotionStateSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("LocomotionStateSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreatureLocoServerAuthEntityData {
    pub _glacier_base: CreatureLocoEntityData,
    pub realm: super::core::Realm,
}

pub trait CreatureLocoServerAuthEntityDataTrait: CreatureLocoEntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
}

impl CreatureLocoServerAuthEntityDataTrait for CreatureLocoServerAuthEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
}

impl CreatureLocoEntityDataTrait for CreatureLocoServerAuthEntityData {
    fn pin_to_ground(&self) -> &bool {
        self._glacier_base.pin_to_ground()
    }
    fn pin_to_ground_mut(&mut self) -> &mut bool {
        self._glacier_base.pin_to_ground_mut()
    }
    fn enable_updates(&self) -> &bool {
        self._glacier_base.enable_updates()
    }
    fn enable_updates_mut(&mut self) -> &mut bool {
        self._glacier_base.enable_updates_mut()
    }
}

impl super::a_i_tools::LocoEntityDataTrait for CreatureLocoServerAuthEntityData {
}

impl super::entity::EntityDataTrait for CreatureLocoServerAuthEntityData {
}

impl super::entity::GameObjectDataTrait for CreatureLocoServerAuthEntityData {
}

impl super::core::DataBusPeerTrait for CreatureLocoServerAuthEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for CreatureLocoServerAuthEntityData {
}

impl super::core::DataContainerTrait for CreatureLocoServerAuthEntityData {
}

pub static CREATURELOCOSERVERAUTHENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureLocoServerAuthEntityData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATURELOCOENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureLocoServerAuthEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(CreatureLocoServerAuthEntityData, realm),
            },
        ],
    }),
    array_type: Some(CREATURELOCOSERVERAUTHENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CreatureLocoServerAuthEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CREATURELOCOSERVERAUTHENTITYDATA_TYPE_INFO
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


pub static CREATURELOCOSERVERAUTHENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureLocoServerAuthEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureLocoServerAuthEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreatureLocoEntityData {
    pub _glacier_base: super::a_i_tools::LocoEntityData,
    pub pin_to_ground: bool,
    pub enable_updates: bool,
}

pub trait CreatureLocoEntityDataTrait: super::a_i_tools::LocoEntityDataTrait {
    fn pin_to_ground(&self) -> &bool;
    fn pin_to_ground_mut(&mut self) -> &mut bool;
    fn enable_updates(&self) -> &bool;
    fn enable_updates_mut(&mut self) -> &mut bool;
}

impl CreatureLocoEntityDataTrait for CreatureLocoEntityData {
    fn pin_to_ground(&self) -> &bool {
        &self.pin_to_ground
    }
    fn pin_to_ground_mut(&mut self) -> &mut bool {
        &mut self.pin_to_ground
    }
    fn enable_updates(&self) -> &bool {
        &self.enable_updates
    }
    fn enable_updates_mut(&mut self) -> &mut bool {
        &mut self.enable_updates
    }
}

impl super::a_i_tools::LocoEntityDataTrait for CreatureLocoEntityData {
}

impl super::entity::EntityDataTrait for CreatureLocoEntityData {
}

impl super::entity::GameObjectDataTrait for CreatureLocoEntityData {
}

impl super::core::DataBusPeerTrait for CreatureLocoEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for CreatureLocoEntityData {
}

impl super::core::DataContainerTrait for CreatureLocoEntityData {
}

pub static CREATURELOCOENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureLocoEntityData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::a_i_tools::LOCOENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureLocoEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "PinToGround",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CreatureLocoEntityData, pin_to_ground),
            },
            FieldInfoData {
                name: "EnableUpdates",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CreatureLocoEntityData, enable_updates),
            },
        ],
    }),
    array_type: Some(CREATURELOCOENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CreatureLocoEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CREATURELOCOENTITYDATA_TYPE_INFO
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


pub static CREATURELOCOENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureLocoEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureLocoEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreatureBaseWaypointProviderEntityData {
    pub _glacier_base: super::entity::EntityData,
}

pub trait CreatureBaseWaypointProviderEntityDataTrait: super::entity::EntityDataTrait {
}

impl CreatureBaseWaypointProviderEntityDataTrait for CreatureBaseWaypointProviderEntityData {
}

impl super::entity::EntityDataTrait for CreatureBaseWaypointProviderEntityData {
}

impl super::entity::GameObjectDataTrait for CreatureBaseWaypointProviderEntityData {
}

impl super::core::DataBusPeerTrait for CreatureBaseWaypointProviderEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for CreatureBaseWaypointProviderEntityData {
}

impl super::core::DataContainerTrait for CreatureBaseWaypointProviderEntityData {
}

pub static CREATUREBASEWAYPOINTPROVIDERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureBaseWaypointProviderEntityData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureBaseWaypointProviderEntityData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CREATUREBASEWAYPOINTPROVIDERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CreatureBaseWaypointProviderEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CREATUREBASEWAYPOINTPROVIDERENTITYDATA_TYPE_INFO
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


pub static CREATUREBASEWAYPOINTPROVIDERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureBaseWaypointProviderEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureBaseWaypointProviderEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreatureConfigurationProviderEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub default_settings: Option<Arc<Mutex<dyn CreatureLocoSettingsTrait>>>,
    pub ant_bindings: Option<Arc<Mutex<dyn CreatureLocoBindingsTrait>>>,
}

pub trait CreatureConfigurationProviderEntityDataTrait: super::entity::EntityDataTrait {
    fn default_settings(&self) -> &Option<Arc<Mutex<dyn CreatureLocoSettingsTrait>>>;
    fn default_settings_mut(&mut self) -> &mut Option<Arc<Mutex<dyn CreatureLocoSettingsTrait>>>;
    fn ant_bindings(&self) -> &Option<Arc<Mutex<dyn CreatureLocoBindingsTrait>>>;
    fn ant_bindings_mut(&mut self) -> &mut Option<Arc<Mutex<dyn CreatureLocoBindingsTrait>>>;
}

impl CreatureConfigurationProviderEntityDataTrait for CreatureConfigurationProviderEntityData {
    fn default_settings(&self) -> &Option<Arc<Mutex<dyn CreatureLocoSettingsTrait>>> {
        &self.default_settings
    }
    fn default_settings_mut(&mut self) -> &mut Option<Arc<Mutex<dyn CreatureLocoSettingsTrait>>> {
        &mut self.default_settings
    }
    fn ant_bindings(&self) -> &Option<Arc<Mutex<dyn CreatureLocoBindingsTrait>>> {
        &self.ant_bindings
    }
    fn ant_bindings_mut(&mut self) -> &mut Option<Arc<Mutex<dyn CreatureLocoBindingsTrait>>> {
        &mut self.ant_bindings
    }
}

impl super::entity::EntityDataTrait for CreatureConfigurationProviderEntityData {
}

impl super::entity::GameObjectDataTrait for CreatureConfigurationProviderEntityData {
}

impl super::core::DataBusPeerTrait for CreatureConfigurationProviderEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for CreatureConfigurationProviderEntityData {
}

impl super::core::DataContainerTrait for CreatureConfigurationProviderEntityData {
}

pub static CREATURECONFIGURATIONPROVIDERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureConfigurationProviderEntityData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureConfigurationProviderEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "DefaultSettings",
                flags: MemberInfoFlags::new(0),
                field_type: "CreatureLocoSettings",
                rust_offset: offset_of!(CreatureConfigurationProviderEntityData, default_settings),
            },
            FieldInfoData {
                name: "AntBindings",
                flags: MemberInfoFlags::new(0),
                field_type: "CreatureLocoBindings",
                rust_offset: offset_of!(CreatureConfigurationProviderEntityData, ant_bindings),
            },
        ],
    }),
    array_type: Some(CREATURECONFIGURATIONPROVIDERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CreatureConfigurationProviderEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CREATURECONFIGURATIONPROVIDERENTITYDATA_TYPE_INFO
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


pub static CREATURECONFIGURATIONPROVIDERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureConfigurationProviderEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureConfigurationProviderEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreatureLocoBindings {
    pub _glacier_base: super::core::DataContainerPolicyAsset,
    pub state_parameters: Vec<Option<Arc<Mutex<dyn LocomotionParamBlockTrait>>>>,
}

pub trait CreatureLocoBindingsTrait: super::core::DataContainerPolicyAssetTrait {
    fn state_parameters(&self) -> &Vec<Option<Arc<Mutex<dyn LocomotionParamBlockTrait>>>>;
    fn state_parameters_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn LocomotionParamBlockTrait>>>>;
}

impl CreatureLocoBindingsTrait for CreatureLocoBindings {
    fn state_parameters(&self) -> &Vec<Option<Arc<Mutex<dyn LocomotionParamBlockTrait>>>> {
        &self.state_parameters
    }
    fn state_parameters_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn LocomotionParamBlockTrait>>>> {
        &mut self.state_parameters
    }
}

impl super::core::DataContainerPolicyAssetTrait for CreatureLocoBindings {
}

impl super::core::AssetTrait for CreatureLocoBindings {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for CreatureLocoBindings {
}

pub static CREATURELOCOBINDINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureLocoBindings",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINERPOLICYASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureLocoBindings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "StateParameters",
                flags: MemberInfoFlags::new(144),
                field_type: "LocomotionParamBlock-Array",
                rust_offset: offset_of!(CreatureLocoBindings, state_parameters),
            },
        ],
    }),
    array_type: Some(CREATURELOCOBINDINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CreatureLocoBindings {
    fn type_info(&self) -> &'static TypeInfo {
        CREATURELOCOBINDINGS_TYPE_INFO
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


pub static CREATURELOCOBINDINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureLocoBindings-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureLocoBindings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EventReactionParamData {
    pub _glacier_base: LocomotionParamBlock,
    pub reaction_binding: CreatureReactionBinding,
}

pub trait EventReactionParamDataTrait: LocomotionParamBlockTrait {
    fn reaction_binding(&self) -> &CreatureReactionBinding;
    fn reaction_binding_mut(&mut self) -> &mut CreatureReactionBinding;
}

impl EventReactionParamDataTrait for EventReactionParamData {
    fn reaction_binding(&self) -> &CreatureReactionBinding {
        &self.reaction_binding
    }
    fn reaction_binding_mut(&mut self) -> &mut CreatureReactionBinding {
        &mut self.reaction_binding
    }
}

impl LocomotionParamBlockTrait for EventReactionParamData {
}

impl super::core::DataContainerTrait for EventReactionParamData {
}

pub static EVENTREACTIONPARAMDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventReactionParamData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONPARAMBLOCK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EventReactionParamData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ReactionBinding",
                flags: MemberInfoFlags::new(0),
                field_type: "CreatureReactionBinding",
                rust_offset: offset_of!(EventReactionParamData, reaction_binding),
            },
        ],
    }),
    array_type: Some(EVENTREACTIONPARAMDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EventReactionParamData {
    fn type_info(&self) -> &'static TypeInfo {
        EVENTREACTIONPARAMDATA_TYPE_INFO
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


pub static EVENTREACTIONPARAMDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventReactionParamData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("EventReactionParamData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreatureReactionBinding {
    pub reaction_trigger: super::ant::AntRef,
    pub reaction_from_stop: super::ant::AntRef,
    pub reaction_event_type: super::ant::AntRef,
}

pub trait CreatureReactionBindingTrait: TypeObject {
    fn reaction_trigger(&self) -> &super::ant::AntRef;
    fn reaction_trigger_mut(&mut self) -> &mut super::ant::AntRef;
    fn reaction_from_stop(&self) -> &super::ant::AntRef;
    fn reaction_from_stop_mut(&mut self) -> &mut super::ant::AntRef;
    fn reaction_event_type(&self) -> &super::ant::AntRef;
    fn reaction_event_type_mut(&mut self) -> &mut super::ant::AntRef;
}

impl CreatureReactionBindingTrait for CreatureReactionBinding {
    fn reaction_trigger(&self) -> &super::ant::AntRef {
        &self.reaction_trigger
    }
    fn reaction_trigger_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.reaction_trigger
    }
    fn reaction_from_stop(&self) -> &super::ant::AntRef {
        &self.reaction_from_stop
    }
    fn reaction_from_stop_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.reaction_from_stop
    }
    fn reaction_event_type(&self) -> &super::ant::AntRef {
        &self.reaction_event_type
    }
    fn reaction_event_type_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.reaction_event_type
    }
}

pub static CREATUREREACTIONBINDING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureReactionBinding",
    flags: MemberInfoFlags::new(32841),
    module: "CreatureLocoShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureReactionBinding as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ReactionTrigger",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(CreatureReactionBinding, reaction_trigger),
            },
            FieldInfoData {
                name: "ReactionFromStop",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(CreatureReactionBinding, reaction_from_stop),
            },
            FieldInfoData {
                name: "ReactionEventType",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(CreatureReactionBinding, reaction_event_type),
            },
        ],
    }),
    array_type: Some(CREATUREREACTIONBINDING_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for CreatureReactionBinding {
    fn type_info(&self) -> &'static TypeInfo {
        CREATUREREACTIONBINDING_TYPE_INFO
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


pub static CREATUREREACTIONBINDING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureReactionBinding-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureReactionBinding"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ProceduralMotionParamData {
    pub _glacier_base: LocomotionParamBlock,
}

pub trait ProceduralMotionParamDataTrait: LocomotionParamBlockTrait {
}

impl ProceduralMotionParamDataTrait for ProceduralMotionParamData {
}

impl LocomotionParamBlockTrait for ProceduralMotionParamData {
}

impl super::core::DataContainerTrait for ProceduralMotionParamData {
}

pub static PROCEDURALMOTIONPARAMDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProceduralMotionParamData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONPARAMBLOCK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ProceduralMotionParamData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PROCEDURALMOTIONPARAMDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProceduralMotionParamData {
    fn type_info(&self) -> &'static TypeInfo {
        PROCEDURALMOTIONPARAMDATA_TYPE_INFO
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


pub static PROCEDURALMOTIONPARAMDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProceduralMotionParamData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("ProceduralMotionParamData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CommonClientBindingsParamData {
    pub _glacier_base: LocomotionParamBlock,
    pub common_binding: CreatureCommonBinding,
    pub misc_binding: CreatureMiscBinding,
}

pub trait CommonClientBindingsParamDataTrait: LocomotionParamBlockTrait {
    fn common_binding(&self) -> &CreatureCommonBinding;
    fn common_binding_mut(&mut self) -> &mut CreatureCommonBinding;
    fn misc_binding(&self) -> &CreatureMiscBinding;
    fn misc_binding_mut(&mut self) -> &mut CreatureMiscBinding;
}

impl CommonClientBindingsParamDataTrait for CommonClientBindingsParamData {
    fn common_binding(&self) -> &CreatureCommonBinding {
        &self.common_binding
    }
    fn common_binding_mut(&mut self) -> &mut CreatureCommonBinding {
        &mut self.common_binding
    }
    fn misc_binding(&self) -> &CreatureMiscBinding {
        &self.misc_binding
    }
    fn misc_binding_mut(&mut self) -> &mut CreatureMiscBinding {
        &mut self.misc_binding
    }
}

impl LocomotionParamBlockTrait for CommonClientBindingsParamData {
}

impl super::core::DataContainerTrait for CommonClientBindingsParamData {
}

pub static COMMONCLIENTBINDINGSPARAMDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CommonClientBindingsParamData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONPARAMBLOCK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CommonClientBindingsParamData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "CommonBinding",
                flags: MemberInfoFlags::new(0),
                field_type: "CreatureCommonBinding",
                rust_offset: offset_of!(CommonClientBindingsParamData, common_binding),
            },
            FieldInfoData {
                name: "MiscBinding",
                flags: MemberInfoFlags::new(0),
                field_type: "CreatureMiscBinding",
                rust_offset: offset_of!(CommonClientBindingsParamData, misc_binding),
            },
        ],
    }),
    array_type: Some(COMMONCLIENTBINDINGSPARAMDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CommonClientBindingsParamData {
    fn type_info(&self) -> &'static TypeInfo {
        COMMONCLIENTBINDINGSPARAMDATA_TYPE_INFO
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


pub static COMMONCLIENTBINDINGSPARAMDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CommonClientBindingsParamData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CommonClientBindingsParamData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PlayAnimParamData {
    pub _glacier_base: LocomotionParamBlock,
    pub play_animation_binding: CreaturePlayAnimationBinding,
}

pub trait PlayAnimParamDataTrait: LocomotionParamBlockTrait {
    fn play_animation_binding(&self) -> &CreaturePlayAnimationBinding;
    fn play_animation_binding_mut(&mut self) -> &mut CreaturePlayAnimationBinding;
}

impl PlayAnimParamDataTrait for PlayAnimParamData {
    fn play_animation_binding(&self) -> &CreaturePlayAnimationBinding {
        &self.play_animation_binding
    }
    fn play_animation_binding_mut(&mut self) -> &mut CreaturePlayAnimationBinding {
        &mut self.play_animation_binding
    }
}

impl LocomotionParamBlockTrait for PlayAnimParamData {
}

impl super::core::DataContainerTrait for PlayAnimParamData {
}

pub static PLAYANIMPARAMDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayAnimParamData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONPARAMBLOCK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PlayAnimParamData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "PlayAnimationBinding",
                flags: MemberInfoFlags::new(0),
                field_type: "CreaturePlayAnimationBinding",
                rust_offset: offset_of!(PlayAnimParamData, play_animation_binding),
            },
        ],
    }),
    array_type: Some(PLAYANIMPARAMDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PlayAnimParamData {
    fn type_info(&self) -> &'static TypeInfo {
        PLAYANIMPARAMDATA_TYPE_INFO
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


pub static PLAYANIMPARAMDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayAnimParamData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("PlayAnimParamData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TurnParamData {
    pub _glacier_base: LocomotionParamBlock,
    pub turn_binding: CreatureTurnBinding,
    pub turn_context_database: super::ant::AntRef,
}

pub trait TurnParamDataTrait: LocomotionParamBlockTrait {
    fn turn_binding(&self) -> &CreatureTurnBinding;
    fn turn_binding_mut(&mut self) -> &mut CreatureTurnBinding;
    fn turn_context_database(&self) -> &super::ant::AntRef;
    fn turn_context_database_mut(&mut self) -> &mut super::ant::AntRef;
}

impl TurnParamDataTrait for TurnParamData {
    fn turn_binding(&self) -> &CreatureTurnBinding {
        &self.turn_binding
    }
    fn turn_binding_mut(&mut self) -> &mut CreatureTurnBinding {
        &mut self.turn_binding
    }
    fn turn_context_database(&self) -> &super::ant::AntRef {
        &self.turn_context_database
    }
    fn turn_context_database_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.turn_context_database
    }
}

impl LocomotionParamBlockTrait for TurnParamData {
}

impl super::core::DataContainerTrait for TurnParamData {
}

pub static TURNPARAMDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TurnParamData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONPARAMBLOCK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TurnParamData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TurnBinding",
                flags: MemberInfoFlags::new(0),
                field_type: "CreatureTurnBinding",
                rust_offset: offset_of!(TurnParamData, turn_binding),
            },
            FieldInfoData {
                name: "TurnContextDatabase",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(TurnParamData, turn_context_database),
            },
        ],
    }),
    array_type: Some(TURNPARAMDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TurnParamData {
    fn type_info(&self) -> &'static TypeInfo {
        TURNPARAMDATA_TYPE_INFO
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


pub static TURNPARAMDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TurnParamData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("TurnParamData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct StopParamData {
    pub _glacier_base: LocomotionParamBlock,
    pub stop_binding: CreatureStopBinding,
    pub stop_context_database: super::ant::AntRef,
}

pub trait StopParamDataTrait: LocomotionParamBlockTrait {
    fn stop_binding(&self) -> &CreatureStopBinding;
    fn stop_binding_mut(&mut self) -> &mut CreatureStopBinding;
    fn stop_context_database(&self) -> &super::ant::AntRef;
    fn stop_context_database_mut(&mut self) -> &mut super::ant::AntRef;
}

impl StopParamDataTrait for StopParamData {
    fn stop_binding(&self) -> &CreatureStopBinding {
        &self.stop_binding
    }
    fn stop_binding_mut(&mut self) -> &mut CreatureStopBinding {
        &mut self.stop_binding
    }
    fn stop_context_database(&self) -> &super::ant::AntRef {
        &self.stop_context_database
    }
    fn stop_context_database_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.stop_context_database
    }
}

impl LocomotionParamBlockTrait for StopParamData {
}

impl super::core::DataContainerTrait for StopParamData {
}

pub static STOPPARAMDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StopParamData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONPARAMBLOCK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StopParamData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "StopBinding",
                flags: MemberInfoFlags::new(0),
                field_type: "CreatureStopBinding",
                rust_offset: offset_of!(StopParamData, stop_binding),
            },
            FieldInfoData {
                name: "StopContextDatabase",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(StopParamData, stop_context_database),
            },
        ],
    }),
    array_type: Some(STOPPARAMDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StopParamData {
    fn type_info(&self) -> &'static TypeInfo {
        STOPPARAMDATA_TYPE_INFO
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


pub static STOPPARAMDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StopParamData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("StopParamData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct StartParamData {
    pub _glacier_base: LocomotionParamBlock,
    pub start_binding: CreatureStartBinding,
    pub start_context_database: super::ant::AntRef,
}

pub trait StartParamDataTrait: LocomotionParamBlockTrait {
    fn start_binding(&self) -> &CreatureStartBinding;
    fn start_binding_mut(&mut self) -> &mut CreatureStartBinding;
    fn start_context_database(&self) -> &super::ant::AntRef;
    fn start_context_database_mut(&mut self) -> &mut super::ant::AntRef;
}

impl StartParamDataTrait for StartParamData {
    fn start_binding(&self) -> &CreatureStartBinding {
        &self.start_binding
    }
    fn start_binding_mut(&mut self) -> &mut CreatureStartBinding {
        &mut self.start_binding
    }
    fn start_context_database(&self) -> &super::ant::AntRef {
        &self.start_context_database
    }
    fn start_context_database_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.start_context_database
    }
}

impl LocomotionParamBlockTrait for StartParamData {
}

impl super::core::DataContainerTrait for StartParamData {
}

pub static STARTPARAMDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StartParamData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONPARAMBLOCK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StartParamData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "StartBinding",
                flags: MemberInfoFlags::new(0),
                field_type: "CreatureStartBinding",
                rust_offset: offset_of!(StartParamData, start_binding),
            },
            FieldInfoData {
                name: "StartContextDatabase",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(StartParamData, start_context_database),
            },
        ],
    }),
    array_type: Some(STARTPARAMDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StartParamData {
    fn type_info(&self) -> &'static TypeInfo {
        STARTPARAMDATA_TYPE_INFO
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


pub static STARTPARAMDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StartParamData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("StartParamData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MoveCycleParamData {
    pub _glacier_base: LocomotionParamBlock,
    pub loco_binding: CreatureLocoBinding,
    pub loco_context_database: super::ant::AntRef,
    pub accel_context_database: super::ant::AntRef,
}

pub trait MoveCycleParamDataTrait: LocomotionParamBlockTrait {
    fn loco_binding(&self) -> &CreatureLocoBinding;
    fn loco_binding_mut(&mut self) -> &mut CreatureLocoBinding;
    fn loco_context_database(&self) -> &super::ant::AntRef;
    fn loco_context_database_mut(&mut self) -> &mut super::ant::AntRef;
    fn accel_context_database(&self) -> &super::ant::AntRef;
    fn accel_context_database_mut(&mut self) -> &mut super::ant::AntRef;
}

impl MoveCycleParamDataTrait for MoveCycleParamData {
    fn loco_binding(&self) -> &CreatureLocoBinding {
        &self.loco_binding
    }
    fn loco_binding_mut(&mut self) -> &mut CreatureLocoBinding {
        &mut self.loco_binding
    }
    fn loco_context_database(&self) -> &super::ant::AntRef {
        &self.loco_context_database
    }
    fn loco_context_database_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.loco_context_database
    }
    fn accel_context_database(&self) -> &super::ant::AntRef {
        &self.accel_context_database
    }
    fn accel_context_database_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.accel_context_database
    }
}

impl LocomotionParamBlockTrait for MoveCycleParamData {
}

impl super::core::DataContainerTrait for MoveCycleParamData {
}

pub static MOVECYCLEPARAMDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MoveCycleParamData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONPARAMBLOCK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MoveCycleParamData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "LocoBinding",
                flags: MemberInfoFlags::new(0),
                field_type: "CreatureLocoBinding",
                rust_offset: offset_of!(MoveCycleParamData, loco_binding),
            },
            FieldInfoData {
                name: "LocoContextDatabase",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(MoveCycleParamData, loco_context_database),
            },
            FieldInfoData {
                name: "AccelContextDatabase",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(MoveCycleParamData, accel_context_database),
            },
        ],
    }),
    array_type: Some(MOVECYCLEPARAMDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MoveCycleParamData {
    fn type_info(&self) -> &'static TypeInfo {
        MOVECYCLEPARAMDATA_TYPE_INFO
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


pub static MOVECYCLEPARAMDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MoveCycleParamData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("MoveCycleParamData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct IdleParamData {
    pub _glacier_base: LocomotionParamBlock,
    pub idle_binding: CreatureIdleBinding,
}

pub trait IdleParamDataTrait: LocomotionParamBlockTrait {
    fn idle_binding(&self) -> &CreatureIdleBinding;
    fn idle_binding_mut(&mut self) -> &mut CreatureIdleBinding;
}

impl IdleParamDataTrait for IdleParamData {
    fn idle_binding(&self) -> &CreatureIdleBinding {
        &self.idle_binding
    }
    fn idle_binding_mut(&mut self) -> &mut CreatureIdleBinding {
        &mut self.idle_binding
    }
}

impl LocomotionParamBlockTrait for IdleParamData {
}

impl super::core::DataContainerTrait for IdleParamData {
}

pub static IDLEPARAMDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IdleParamData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONPARAMBLOCK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IdleParamData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "IdleBinding",
                flags: MemberInfoFlags::new(0),
                field_type: "CreatureIdleBinding",
                rust_offset: offset_of!(IdleParamData, idle_binding),
            },
        ],
    }),
    array_type: Some(IDLEPARAMDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IdleParamData {
    fn type_info(&self) -> &'static TypeInfo {
        IDLEPARAMDATA_TYPE_INFO
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


pub static IDLEPARAMDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IdleParamData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("IdleParamData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreaturePlayAnimationBinding {
    pub branch_in_phase: super::ant::AntRef,
    pub align_translation: super::ant::AntRef,
    pub align_facing_rotation: super::ant::AntRef,
    pub ant_in_play_animation: super::ant::AntRef,
}

pub trait CreaturePlayAnimationBindingTrait: TypeObject {
    fn branch_in_phase(&self) -> &super::ant::AntRef;
    fn branch_in_phase_mut(&mut self) -> &mut super::ant::AntRef;
    fn align_translation(&self) -> &super::ant::AntRef;
    fn align_translation_mut(&mut self) -> &mut super::ant::AntRef;
    fn align_facing_rotation(&self) -> &super::ant::AntRef;
    fn align_facing_rotation_mut(&mut self) -> &mut super::ant::AntRef;
    fn ant_in_play_animation(&self) -> &super::ant::AntRef;
    fn ant_in_play_animation_mut(&mut self) -> &mut super::ant::AntRef;
}

impl CreaturePlayAnimationBindingTrait for CreaturePlayAnimationBinding {
    fn branch_in_phase(&self) -> &super::ant::AntRef {
        &self.branch_in_phase
    }
    fn branch_in_phase_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.branch_in_phase
    }
    fn align_translation(&self) -> &super::ant::AntRef {
        &self.align_translation
    }
    fn align_translation_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.align_translation
    }
    fn align_facing_rotation(&self) -> &super::ant::AntRef {
        &self.align_facing_rotation
    }
    fn align_facing_rotation_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.align_facing_rotation
    }
    fn ant_in_play_animation(&self) -> &super::ant::AntRef {
        &self.ant_in_play_animation
    }
    fn ant_in_play_animation_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.ant_in_play_animation
    }
}

pub static CREATUREPLAYANIMATIONBINDING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreaturePlayAnimationBinding",
    flags: MemberInfoFlags::new(32841),
    module: "CreatureLocoShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreaturePlayAnimationBinding as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "BranchInPhase",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(CreaturePlayAnimationBinding, branch_in_phase),
            },
            FieldInfoData {
                name: "AlignTranslation",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(CreaturePlayAnimationBinding, align_translation),
            },
            FieldInfoData {
                name: "AlignFacingRotation",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(CreaturePlayAnimationBinding, align_facing_rotation),
            },
            FieldInfoData {
                name: "AntInPlayAnimation",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(CreaturePlayAnimationBinding, ant_in_play_animation),
            },
        ],
    }),
    array_type: Some(CREATUREPLAYANIMATIONBINDING_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for CreaturePlayAnimationBinding {
    fn type_info(&self) -> &'static TypeInfo {
        CREATUREPLAYANIMATIONBINDING_TYPE_INFO
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


pub static CREATUREPLAYANIMATIONBINDING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreaturePlayAnimationBinding-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreaturePlayAnimationBinding"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreatureTurnBinding {
    pub turn_angle: super::ant::AntRef,
    pub turn_phase: super::ant::AntRef,
    pub turn_trigger: super::ant::AntRef,
}

pub trait CreatureTurnBindingTrait: TypeObject {
    fn turn_angle(&self) -> &super::ant::AntRef;
    fn turn_angle_mut(&mut self) -> &mut super::ant::AntRef;
    fn turn_phase(&self) -> &super::ant::AntRef;
    fn turn_phase_mut(&mut self) -> &mut super::ant::AntRef;
    fn turn_trigger(&self) -> &super::ant::AntRef;
    fn turn_trigger_mut(&mut self) -> &mut super::ant::AntRef;
}

impl CreatureTurnBindingTrait for CreatureTurnBinding {
    fn turn_angle(&self) -> &super::ant::AntRef {
        &self.turn_angle
    }
    fn turn_angle_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.turn_angle
    }
    fn turn_phase(&self) -> &super::ant::AntRef {
        &self.turn_phase
    }
    fn turn_phase_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.turn_phase
    }
    fn turn_trigger(&self) -> &super::ant::AntRef {
        &self.turn_trigger
    }
    fn turn_trigger_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.turn_trigger
    }
}

pub static CREATURETURNBINDING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureTurnBinding",
    flags: MemberInfoFlags::new(32841),
    module: "CreatureLocoShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureTurnBinding as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TurnAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(CreatureTurnBinding, turn_angle),
            },
            FieldInfoData {
                name: "TurnPhase",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(CreatureTurnBinding, turn_phase),
            },
            FieldInfoData {
                name: "TurnTrigger",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(CreatureTurnBinding, turn_trigger),
            },
        ],
    }),
    array_type: Some(CREATURETURNBINDING_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for CreatureTurnBinding {
    fn type_info(&self) -> &'static TypeInfo {
        CREATURETURNBINDING_TYPE_INFO
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


pub static CREATURETURNBINDING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureTurnBinding-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureTurnBinding"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreatureStopBinding {
    pub stop_angle: super::ant::AntRef,
    pub stop_relative_facing_angle: super::ant::AntRef,
    pub stop_phase: super::ant::AntRef,
    pub stop_trigger: super::ant::AntRef,
}

pub trait CreatureStopBindingTrait: TypeObject {
    fn stop_angle(&self) -> &super::ant::AntRef;
    fn stop_angle_mut(&mut self) -> &mut super::ant::AntRef;
    fn stop_relative_facing_angle(&self) -> &super::ant::AntRef;
    fn stop_relative_facing_angle_mut(&mut self) -> &mut super::ant::AntRef;
    fn stop_phase(&self) -> &super::ant::AntRef;
    fn stop_phase_mut(&mut self) -> &mut super::ant::AntRef;
    fn stop_trigger(&self) -> &super::ant::AntRef;
    fn stop_trigger_mut(&mut self) -> &mut super::ant::AntRef;
}

impl CreatureStopBindingTrait for CreatureStopBinding {
    fn stop_angle(&self) -> &super::ant::AntRef {
        &self.stop_angle
    }
    fn stop_angle_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.stop_angle
    }
    fn stop_relative_facing_angle(&self) -> &super::ant::AntRef {
        &self.stop_relative_facing_angle
    }
    fn stop_relative_facing_angle_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.stop_relative_facing_angle
    }
    fn stop_phase(&self) -> &super::ant::AntRef {
        &self.stop_phase
    }
    fn stop_phase_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.stop_phase
    }
    fn stop_trigger(&self) -> &super::ant::AntRef {
        &self.stop_trigger
    }
    fn stop_trigger_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.stop_trigger
    }
}

pub static CREATURESTOPBINDING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureStopBinding",
    flags: MemberInfoFlags::new(32841),
    module: "CreatureLocoShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureStopBinding as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "StopAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(CreatureStopBinding, stop_angle),
            },
            FieldInfoData {
                name: "StopRelativeFacingAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(CreatureStopBinding, stop_relative_facing_angle),
            },
            FieldInfoData {
                name: "StopPhase",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(CreatureStopBinding, stop_phase),
            },
            FieldInfoData {
                name: "StopTrigger",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(CreatureStopBinding, stop_trigger),
            },
        ],
    }),
    array_type: Some(CREATURESTOPBINDING_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for CreatureStopBinding {
    fn type_info(&self) -> &'static TypeInfo {
        CREATURESTOPBINDING_TYPE_INFO
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


pub static CREATURESTOPBINDING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureStopBinding-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureStopBinding"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreatureStartBinding {
    pub start_facing_offset: super::ant::AntRef,
    pub start_motion_delta: super::ant::AntRef,
    pub start_trigger: super::ant::AntRef,
}

pub trait CreatureStartBindingTrait: TypeObject {
    fn start_facing_offset(&self) -> &super::ant::AntRef;
    fn start_facing_offset_mut(&mut self) -> &mut super::ant::AntRef;
    fn start_motion_delta(&self) -> &super::ant::AntRef;
    fn start_motion_delta_mut(&mut self) -> &mut super::ant::AntRef;
    fn start_trigger(&self) -> &super::ant::AntRef;
    fn start_trigger_mut(&mut self) -> &mut super::ant::AntRef;
}

impl CreatureStartBindingTrait for CreatureStartBinding {
    fn start_facing_offset(&self) -> &super::ant::AntRef {
        &self.start_facing_offset
    }
    fn start_facing_offset_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.start_facing_offset
    }
    fn start_motion_delta(&self) -> &super::ant::AntRef {
        &self.start_motion_delta
    }
    fn start_motion_delta_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.start_motion_delta
    }
    fn start_trigger(&self) -> &super::ant::AntRef {
        &self.start_trigger
    }
    fn start_trigger_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.start_trigger
    }
}

pub static CREATURESTARTBINDING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureStartBinding",
    flags: MemberInfoFlags::new(32841),
    module: "CreatureLocoShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureStartBinding as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "StartFacingOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(CreatureStartBinding, start_facing_offset),
            },
            FieldInfoData {
                name: "StartMotionDelta",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(CreatureStartBinding, start_motion_delta),
            },
            FieldInfoData {
                name: "StartTrigger",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(CreatureStartBinding, start_trigger),
            },
        ],
    }),
    array_type: Some(CREATURESTARTBINDING_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for CreatureStartBinding {
    fn type_info(&self) -> &'static TypeInfo {
        CREATURESTARTBINDING_TYPE_INFO
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


pub static CREATURESTARTBINDING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureStartBinding-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureStartBinding"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreatureMiscBinding {
    pub current_speed: super::ant::AntRef,
    pub loco_speed_multiplier: super::ant::AntRef,
    pub g_force: super::ant::AntRef,
    pub relative_pitch: super::ant::AntRef,
    pub path_steering: super::ant::AntRef,
    pub relative_steering: super::ant::AntRef,
    pub raw_delat_traj: super::ant::AntRef,
    pub speed_mode: super::ant::AntRef,
    pub awareness_x_target: super::ant::AntRef,
    pub awareness_y_target: super::ant::AntRef,
}

pub trait CreatureMiscBindingTrait: TypeObject {
    fn current_speed(&self) -> &super::ant::AntRef;
    fn current_speed_mut(&mut self) -> &mut super::ant::AntRef;
    fn loco_speed_multiplier(&self) -> &super::ant::AntRef;
    fn loco_speed_multiplier_mut(&mut self) -> &mut super::ant::AntRef;
    fn g_force(&self) -> &super::ant::AntRef;
    fn g_force_mut(&mut self) -> &mut super::ant::AntRef;
    fn relative_pitch(&self) -> &super::ant::AntRef;
    fn relative_pitch_mut(&mut self) -> &mut super::ant::AntRef;
    fn path_steering(&self) -> &super::ant::AntRef;
    fn path_steering_mut(&mut self) -> &mut super::ant::AntRef;
    fn relative_steering(&self) -> &super::ant::AntRef;
    fn relative_steering_mut(&mut self) -> &mut super::ant::AntRef;
    fn raw_delat_traj(&self) -> &super::ant::AntRef;
    fn raw_delat_traj_mut(&mut self) -> &mut super::ant::AntRef;
    fn speed_mode(&self) -> &super::ant::AntRef;
    fn speed_mode_mut(&mut self) -> &mut super::ant::AntRef;
    fn awareness_x_target(&self) -> &super::ant::AntRef;
    fn awareness_x_target_mut(&mut self) -> &mut super::ant::AntRef;
    fn awareness_y_target(&self) -> &super::ant::AntRef;
    fn awareness_y_target_mut(&mut self) -> &mut super::ant::AntRef;
}

impl CreatureMiscBindingTrait for CreatureMiscBinding {
    fn current_speed(&self) -> &super::ant::AntRef {
        &self.current_speed
    }
    fn current_speed_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.current_speed
    }
    fn loco_speed_multiplier(&self) -> &super::ant::AntRef {
        &self.loco_speed_multiplier
    }
    fn loco_speed_multiplier_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.loco_speed_multiplier
    }
    fn g_force(&self) -> &super::ant::AntRef {
        &self.g_force
    }
    fn g_force_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.g_force
    }
    fn relative_pitch(&self) -> &super::ant::AntRef {
        &self.relative_pitch
    }
    fn relative_pitch_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.relative_pitch
    }
    fn path_steering(&self) -> &super::ant::AntRef {
        &self.path_steering
    }
    fn path_steering_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.path_steering
    }
    fn relative_steering(&self) -> &super::ant::AntRef {
        &self.relative_steering
    }
    fn relative_steering_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.relative_steering
    }
    fn raw_delat_traj(&self) -> &super::ant::AntRef {
        &self.raw_delat_traj
    }
    fn raw_delat_traj_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.raw_delat_traj
    }
    fn speed_mode(&self) -> &super::ant::AntRef {
        &self.speed_mode
    }
    fn speed_mode_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.speed_mode
    }
    fn awareness_x_target(&self) -> &super::ant::AntRef {
        &self.awareness_x_target
    }
    fn awareness_x_target_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.awareness_x_target
    }
    fn awareness_y_target(&self) -> &super::ant::AntRef {
        &self.awareness_y_target
    }
    fn awareness_y_target_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.awareness_y_target
    }
}

pub static CREATUREMISCBINDING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureMiscBinding",
    flags: MemberInfoFlags::new(32841),
    module: "CreatureLocoShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureMiscBinding as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "CurrentSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(CreatureMiscBinding, current_speed),
            },
            FieldInfoData {
                name: "LocoSpeedMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(CreatureMiscBinding, loco_speed_multiplier),
            },
            FieldInfoData {
                name: "G_Force",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(CreatureMiscBinding, g_force),
            },
            FieldInfoData {
                name: "RelativePitch",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(CreatureMiscBinding, relative_pitch),
            },
            FieldInfoData {
                name: "PathSteering",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(CreatureMiscBinding, path_steering),
            },
            FieldInfoData {
                name: "RelativeSteering",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(CreatureMiscBinding, relative_steering),
            },
            FieldInfoData {
                name: "RawDelatTraj",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(CreatureMiscBinding, raw_delat_traj),
            },
            FieldInfoData {
                name: "SpeedMode",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(CreatureMiscBinding, speed_mode),
            },
            FieldInfoData {
                name: "AwarenessXTarget",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(CreatureMiscBinding, awareness_x_target),
            },
            FieldInfoData {
                name: "AwarenessYTarget",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(CreatureMiscBinding, awareness_y_target),
            },
        ],
    }),
    array_type: Some(CREATUREMISCBINDING_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for CreatureMiscBinding {
    fn type_info(&self) -> &'static TypeInfo {
        CREATUREMISCBINDING_TYPE_INFO
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


pub static CREATUREMISCBINDING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureMiscBinding-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureMiscBinding"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreatureLocoBinding {
    pub transition_speed_mode: super::ant::AntRef,
    pub trigger_speed_transition: super::ant::AntRef,
    pub accel_decel_phase: super::ant::AntRef,
    pub loco_turn_phase: super::ant::AntRef,
    pub loco_end_phase: super::ant::AntRef,
    pub loco_trigger: super::ant::AntRef,
}

pub trait CreatureLocoBindingTrait: TypeObject {
    fn transition_speed_mode(&self) -> &super::ant::AntRef;
    fn transition_speed_mode_mut(&mut self) -> &mut super::ant::AntRef;
    fn trigger_speed_transition(&self) -> &super::ant::AntRef;
    fn trigger_speed_transition_mut(&mut self) -> &mut super::ant::AntRef;
    fn accel_decel_phase(&self) -> &super::ant::AntRef;
    fn accel_decel_phase_mut(&mut self) -> &mut super::ant::AntRef;
    fn loco_turn_phase(&self) -> &super::ant::AntRef;
    fn loco_turn_phase_mut(&mut self) -> &mut super::ant::AntRef;
    fn loco_end_phase(&self) -> &super::ant::AntRef;
    fn loco_end_phase_mut(&mut self) -> &mut super::ant::AntRef;
    fn loco_trigger(&self) -> &super::ant::AntRef;
    fn loco_trigger_mut(&mut self) -> &mut super::ant::AntRef;
}

impl CreatureLocoBindingTrait for CreatureLocoBinding {
    fn transition_speed_mode(&self) -> &super::ant::AntRef {
        &self.transition_speed_mode
    }
    fn transition_speed_mode_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.transition_speed_mode
    }
    fn trigger_speed_transition(&self) -> &super::ant::AntRef {
        &self.trigger_speed_transition
    }
    fn trigger_speed_transition_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.trigger_speed_transition
    }
    fn accel_decel_phase(&self) -> &super::ant::AntRef {
        &self.accel_decel_phase
    }
    fn accel_decel_phase_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.accel_decel_phase
    }
    fn loco_turn_phase(&self) -> &super::ant::AntRef {
        &self.loco_turn_phase
    }
    fn loco_turn_phase_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.loco_turn_phase
    }
    fn loco_end_phase(&self) -> &super::ant::AntRef {
        &self.loco_end_phase
    }
    fn loco_end_phase_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.loco_end_phase
    }
    fn loco_trigger(&self) -> &super::ant::AntRef {
        &self.loco_trigger
    }
    fn loco_trigger_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.loco_trigger
    }
}

pub static CREATURELOCOBINDING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureLocoBinding",
    flags: MemberInfoFlags::new(32841),
    module: "CreatureLocoShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureLocoBinding as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TransitionSpeedMode",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(CreatureLocoBinding, transition_speed_mode),
            },
            FieldInfoData {
                name: "TriggerSpeedTransition",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(CreatureLocoBinding, trigger_speed_transition),
            },
            FieldInfoData {
                name: "AccelDecelPhase",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(CreatureLocoBinding, accel_decel_phase),
            },
            FieldInfoData {
                name: "LocoTurnPhase",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(CreatureLocoBinding, loco_turn_phase),
            },
            FieldInfoData {
                name: "LocoEndPhase",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(CreatureLocoBinding, loco_end_phase),
            },
            FieldInfoData {
                name: "LocoTrigger",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(CreatureLocoBinding, loco_trigger),
            },
        ],
    }),
    array_type: Some(CREATURELOCOBINDING_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for CreatureLocoBinding {
    fn type_info(&self) -> &'static TypeInfo {
        CREATURELOCOBINDING_TYPE_INFO
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


pub static CREATURELOCOBINDING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureLocoBinding-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureLocoBinding"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreatureIdleBinding {
    pub idle_turn_angle: super::ant::AntRef,
    pub idle_turn_trigger: super::ant::AntRef,
}

pub trait CreatureIdleBindingTrait: TypeObject {
    fn idle_turn_angle(&self) -> &super::ant::AntRef;
    fn idle_turn_angle_mut(&mut self) -> &mut super::ant::AntRef;
    fn idle_turn_trigger(&self) -> &super::ant::AntRef;
    fn idle_turn_trigger_mut(&mut self) -> &mut super::ant::AntRef;
}

impl CreatureIdleBindingTrait for CreatureIdleBinding {
    fn idle_turn_angle(&self) -> &super::ant::AntRef {
        &self.idle_turn_angle
    }
    fn idle_turn_angle_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.idle_turn_angle
    }
    fn idle_turn_trigger(&self) -> &super::ant::AntRef {
        &self.idle_turn_trigger
    }
    fn idle_turn_trigger_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.idle_turn_trigger
    }
}

pub static CREATUREIDLEBINDING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureIdleBinding",
    flags: MemberInfoFlags::new(32841),
    module: "CreatureLocoShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureIdleBinding as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "IdleTurnAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(CreatureIdleBinding, idle_turn_angle),
            },
            FieldInfoData {
                name: "IdleTurnTrigger",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(CreatureIdleBinding, idle_turn_trigger),
            },
        ],
    }),
    array_type: Some(CREATUREIDLEBINDING_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for CreatureIdleBinding {
    fn type_info(&self) -> &'static TypeInfo {
        CREATUREIDLEBINDING_TYPE_INFO
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


pub static CREATUREIDLEBINDING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureIdleBinding-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureIdleBinding"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreatureCommonBinding {
    pub warp_angle: super::ant::AntRef,
    pub breakout_early: super::ant::AntRef,
    pub early_out_branch_types: Vec<EarlyOutType>,
}

pub trait CreatureCommonBindingTrait: TypeObject {
    fn warp_angle(&self) -> &super::ant::AntRef;
    fn warp_angle_mut(&mut self) -> &mut super::ant::AntRef;
    fn breakout_early(&self) -> &super::ant::AntRef;
    fn breakout_early_mut(&mut self) -> &mut super::ant::AntRef;
    fn early_out_branch_types(&self) -> &Vec<EarlyOutType>;
    fn early_out_branch_types_mut(&mut self) -> &mut Vec<EarlyOutType>;
}

impl CreatureCommonBindingTrait for CreatureCommonBinding {
    fn warp_angle(&self) -> &super::ant::AntRef {
        &self.warp_angle
    }
    fn warp_angle_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.warp_angle
    }
    fn breakout_early(&self) -> &super::ant::AntRef {
        &self.breakout_early
    }
    fn breakout_early_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.breakout_early
    }
    fn early_out_branch_types(&self) -> &Vec<EarlyOutType> {
        &self.early_out_branch_types
    }
    fn early_out_branch_types_mut(&mut self) -> &mut Vec<EarlyOutType> {
        &mut self.early_out_branch_types
    }
}

pub static CREATURECOMMONBINDING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureCommonBinding",
    flags: MemberInfoFlags::new(73),
    module: "CreatureLocoShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureCommonBinding as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "WarpAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(CreatureCommonBinding, warp_angle),
            },
            FieldInfoData {
                name: "BreakoutEarly",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(CreatureCommonBinding, breakout_early),
            },
            FieldInfoData {
                name: "EarlyOutBranchTypes",
                flags: MemberInfoFlags::new(144),
                field_type: "EarlyOutType-Array",
                rust_offset: offset_of!(CreatureCommonBinding, early_out_branch_types),
            },
        ],
    }),
    array_type: Some(CREATURECOMMONBINDING_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CreatureCommonBinding {
    fn type_info(&self) -> &'static TypeInfo {
        CREATURECOMMONBINDING_TYPE_INFO
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


pub static CREATURECOMMONBINDING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureCommonBinding-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureCommonBinding"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EarlyOutType {
    pub early_out_branch_type: super::ant::AntRef,
}

pub trait EarlyOutTypeTrait: TypeObject {
    fn early_out_branch_type(&self) -> &super::ant::AntRef;
    fn early_out_branch_type_mut(&mut self) -> &mut super::ant::AntRef;
}

impl EarlyOutTypeTrait for EarlyOutType {
    fn early_out_branch_type(&self) -> &super::ant::AntRef {
        &self.early_out_branch_type
    }
    fn early_out_branch_type_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.early_out_branch_type
    }
}

pub static EARLYOUTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EarlyOutType",
    flags: MemberInfoFlags::new(32841),
    module: "CreatureLocoShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EarlyOutType as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "EarlyOutBranchType",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(EarlyOutType, early_out_branch_type),
            },
        ],
    }),
    array_type: Some(EARLYOUTTYPE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for EarlyOutType {
    fn type_info(&self) -> &'static TypeInfo {
        EARLYOUTTYPE_TYPE_INFO
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


pub static EARLYOUTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EarlyOutType-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("EarlyOutType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LocomotionParamBlock {
    pub _glacier_base: super::core::DataContainer,
}

pub trait LocomotionParamBlockTrait: super::core::DataContainerTrait {
}

impl LocomotionParamBlockTrait for LocomotionParamBlock {
}

impl super::core::DataContainerTrait for LocomotionParamBlock {
}

pub static LOCOMOTIONPARAMBLOCK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocomotionParamBlock",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LocomotionParamBlock as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LOCOMOTIONPARAMBLOCK_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LocomotionParamBlock {
    fn type_info(&self) -> &'static TypeInfo {
        LOCOMOTIONPARAMBLOCK_TYPE_INFO
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


pub static LOCOMOTIONPARAMBLOCK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocomotionParamBlock-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("LocomotionParamBlock"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreatureFollowWaypointUnspawnEntityData {
    pub _glacier_base: CreatureFollowBaseData,
    pub realm: super::core::Realm,
}

pub trait CreatureFollowWaypointUnspawnEntityDataTrait: CreatureFollowBaseDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
}

impl CreatureFollowWaypointUnspawnEntityDataTrait for CreatureFollowWaypointUnspawnEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
}

impl CreatureFollowBaseDataTrait for CreatureFollowWaypointUnspawnEntityData {
}

impl super::entity::EntityDataTrait for CreatureFollowWaypointUnspawnEntityData {
}

impl super::entity::GameObjectDataTrait for CreatureFollowWaypointUnspawnEntityData {
}

impl super::core::DataBusPeerTrait for CreatureFollowWaypointUnspawnEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for CreatureFollowWaypointUnspawnEntityData {
}

impl super::core::DataContainerTrait for CreatureFollowWaypointUnspawnEntityData {
}

pub static CREATUREFOLLOWWAYPOINTUNSPAWNENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointUnspawnEntityData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREFOLLOWBASEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureFollowWaypointUnspawnEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(CreatureFollowWaypointUnspawnEntityData, realm),
            },
        ],
    }),
    array_type: Some(CREATUREFOLLOWWAYPOINTUNSPAWNENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CreatureFollowWaypointUnspawnEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CREATUREFOLLOWWAYPOINTUNSPAWNENTITYDATA_TYPE_INFO
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


pub static CREATUREFOLLOWWAYPOINTUNSPAWNENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointUnspawnEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureFollowWaypointUnspawnEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreatureFollowWaypointClosestChooserEntityData {
    pub _glacier_base: CreatureFollowBaseData,
    pub realm: super::core::Realm,
    pub behaviour_type: CLWaypointListChooserClosestToType,
}

pub trait CreatureFollowWaypointClosestChooserEntityDataTrait: CreatureFollowBaseDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn behaviour_type(&self) -> &CLWaypointListChooserClosestToType;
    fn behaviour_type_mut(&mut self) -> &mut CLWaypointListChooserClosestToType;
}

impl CreatureFollowWaypointClosestChooserEntityDataTrait for CreatureFollowWaypointClosestChooserEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn behaviour_type(&self) -> &CLWaypointListChooserClosestToType {
        &self.behaviour_type
    }
    fn behaviour_type_mut(&mut self) -> &mut CLWaypointListChooserClosestToType {
        &mut self.behaviour_type
    }
}

impl CreatureFollowBaseDataTrait for CreatureFollowWaypointClosestChooserEntityData {
}

impl super::entity::EntityDataTrait for CreatureFollowWaypointClosestChooserEntityData {
}

impl super::entity::GameObjectDataTrait for CreatureFollowWaypointClosestChooserEntityData {
}

impl super::core::DataBusPeerTrait for CreatureFollowWaypointClosestChooserEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for CreatureFollowWaypointClosestChooserEntityData {
}

impl super::core::DataContainerTrait for CreatureFollowWaypointClosestChooserEntityData {
}

pub static CREATUREFOLLOWWAYPOINTCLOSESTCHOOSERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointClosestChooserEntityData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREFOLLOWBASEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureFollowWaypointClosestChooserEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(CreatureFollowWaypointClosestChooserEntityData, realm),
            },
            FieldInfoData {
                name: "BehaviourType",
                flags: MemberInfoFlags::new(0),
                field_type: "CL_WaypointListChooser_ClosestToType",
                rust_offset: offset_of!(CreatureFollowWaypointClosestChooserEntityData, behaviour_type),
            },
        ],
    }),
    array_type: Some(CREATUREFOLLOWWAYPOINTCLOSESTCHOOSERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CreatureFollowWaypointClosestChooserEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CREATUREFOLLOWWAYPOINTCLOSESTCHOOSERENTITYDATA_TYPE_INFO
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


pub static CREATUREFOLLOWWAYPOINTCLOSESTCHOOSERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointClosestChooserEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureFollowWaypointClosestChooserEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum CLWaypointListChooserClosestToType {
    #[default]
    Closest_To_Current_Character = 0,
    Closest_To_Nearest_PLayer = 1,
    Farthest_From_Nearest_Player = 2,
}

pub static CL_WAYPOINTLISTCHOOSER_CLOSESTTOTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CL_WaypointListChooser_ClosestToType",
    flags: MemberInfoFlags::new(49429),
    module: "CreatureLocoShared",
    data: TypeInfoData::Enum,
    array_type: Some(CL_WAYPOINTLISTCHOOSER_CLOSESTTOTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CLWaypointListChooserClosestToType {
    fn type_info(&self) -> &'static TypeInfo {
        CL_WAYPOINTLISTCHOOSER_CLOSESTTOTYPE_TYPE_INFO
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


pub static CL_WAYPOINTLISTCHOOSER_CLOSESTTOTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CL_WaypointListChooser_ClosestToType-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CL_WaypointListChooser_ClosestToType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreatureFollowWaypointOccupancyChooserEntityData {
    pub _glacier_base: CreatureFollowBaseData,
    pub realm: super::core::Realm,
    pub enable_available: bool,
    pub occupancy_limit: i32,
}

pub trait CreatureFollowWaypointOccupancyChooserEntityDataTrait: CreatureFollowBaseDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn enable_available(&self) -> &bool;
    fn enable_available_mut(&mut self) -> &mut bool;
    fn occupancy_limit(&self) -> &i32;
    fn occupancy_limit_mut(&mut self) -> &mut i32;
}

impl CreatureFollowWaypointOccupancyChooserEntityDataTrait for CreatureFollowWaypointOccupancyChooserEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn enable_available(&self) -> &bool {
        &self.enable_available
    }
    fn enable_available_mut(&mut self) -> &mut bool {
        &mut self.enable_available
    }
    fn occupancy_limit(&self) -> &i32 {
        &self.occupancy_limit
    }
    fn occupancy_limit_mut(&mut self) -> &mut i32 {
        &mut self.occupancy_limit
    }
}

impl CreatureFollowBaseDataTrait for CreatureFollowWaypointOccupancyChooserEntityData {
}

impl super::entity::EntityDataTrait for CreatureFollowWaypointOccupancyChooserEntityData {
}

impl super::entity::GameObjectDataTrait for CreatureFollowWaypointOccupancyChooserEntityData {
}

impl super::core::DataBusPeerTrait for CreatureFollowWaypointOccupancyChooserEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for CreatureFollowWaypointOccupancyChooserEntityData {
}

impl super::core::DataContainerTrait for CreatureFollowWaypointOccupancyChooserEntityData {
}

pub static CREATUREFOLLOWWAYPOINTOCCUPANCYCHOOSERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointOccupancyChooserEntityData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREFOLLOWBASEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureFollowWaypointOccupancyChooserEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(CreatureFollowWaypointOccupancyChooserEntityData, realm),
            },
            FieldInfoData {
                name: "EnableAvailable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CreatureFollowWaypointOccupancyChooserEntityData, enable_available),
            },
            FieldInfoData {
                name: "OccupancyLimit",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(CreatureFollowWaypointOccupancyChooserEntityData, occupancy_limit),
            },
        ],
    }),
    array_type: Some(CREATUREFOLLOWWAYPOINTOCCUPANCYCHOOSERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CreatureFollowWaypointOccupancyChooserEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CREATUREFOLLOWWAYPOINTOCCUPANCYCHOOSERENTITYDATA_TYPE_INFO
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


pub static CREATUREFOLLOWWAYPOINTOCCUPANCYCHOOSERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointOccupancyChooserEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureFollowWaypointOccupancyChooserEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreatureFollowWaypointBoolChooserEntityData {
    pub _glacier_base: CreatureFollowBaseData,
    pub realm: super::core::Realm,
    pub chance_of_true: f32,
    pub enable_random_choice: bool,
    pub selection_condition: bool,
}

pub trait CreatureFollowWaypointBoolChooserEntityDataTrait: CreatureFollowBaseDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn chance_of_true(&self) -> &f32;
    fn chance_of_true_mut(&mut self) -> &mut f32;
    fn enable_random_choice(&self) -> &bool;
    fn enable_random_choice_mut(&mut self) -> &mut bool;
    fn selection_condition(&self) -> &bool;
    fn selection_condition_mut(&mut self) -> &mut bool;
}

impl CreatureFollowWaypointBoolChooserEntityDataTrait for CreatureFollowWaypointBoolChooserEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn chance_of_true(&self) -> &f32 {
        &self.chance_of_true
    }
    fn chance_of_true_mut(&mut self) -> &mut f32 {
        &mut self.chance_of_true
    }
    fn enable_random_choice(&self) -> &bool {
        &self.enable_random_choice
    }
    fn enable_random_choice_mut(&mut self) -> &mut bool {
        &mut self.enable_random_choice
    }
    fn selection_condition(&self) -> &bool {
        &self.selection_condition
    }
    fn selection_condition_mut(&mut self) -> &mut bool {
        &mut self.selection_condition
    }
}

impl CreatureFollowBaseDataTrait for CreatureFollowWaypointBoolChooserEntityData {
}

impl super::entity::EntityDataTrait for CreatureFollowWaypointBoolChooserEntityData {
}

impl super::entity::GameObjectDataTrait for CreatureFollowWaypointBoolChooserEntityData {
}

impl super::core::DataBusPeerTrait for CreatureFollowWaypointBoolChooserEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for CreatureFollowWaypointBoolChooserEntityData {
}

impl super::core::DataContainerTrait for CreatureFollowWaypointBoolChooserEntityData {
}

pub static CREATUREFOLLOWWAYPOINTBOOLCHOOSERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointBoolChooserEntityData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREFOLLOWBASEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureFollowWaypointBoolChooserEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(CreatureFollowWaypointBoolChooserEntityData, realm),
            },
            FieldInfoData {
                name: "ChanceOfTrue",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CreatureFollowWaypointBoolChooserEntityData, chance_of_true),
            },
            FieldInfoData {
                name: "EnableRandomChoice",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CreatureFollowWaypointBoolChooserEntityData, enable_random_choice),
            },
            FieldInfoData {
                name: "SelectionCondition",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CreatureFollowWaypointBoolChooserEntityData, selection_condition),
            },
        ],
    }),
    array_type: Some(CREATUREFOLLOWWAYPOINTBOOLCHOOSERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CreatureFollowWaypointBoolChooserEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CREATUREFOLLOWWAYPOINTBOOLCHOOSERENTITYDATA_TYPE_INFO
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


pub static CREATUREFOLLOWWAYPOINTBOOLCHOOSERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointBoolChooserEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureFollowWaypointBoolChooserEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreatureFollowWaypointsEntityData {
    pub _glacier_base: CreatureFollowWaypointSegmentEntityData,
}

pub trait CreatureFollowWaypointsEntityDataTrait: CreatureFollowWaypointSegmentEntityDataTrait {
}

impl CreatureFollowWaypointsEntityDataTrait for CreatureFollowWaypointsEntityData {
}

impl CreatureFollowWaypointSegmentEntityDataTrait for CreatureFollowWaypointsEntityData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        self._glacier_base.realm_mut()
    }
    fn type_of_route(&self) -> &super::pathfinding_shared::RouteType {
        self._glacier_base.type_of_route()
    }
    fn type_of_route_mut(&mut self) -> &mut super::pathfinding_shared::RouteType {
        self._glacier_base.type_of_route_mut()
    }
    fn start_point(&self) -> &CLWaypointListStartType {
        self._glacier_base.start_point()
    }
    fn start_point_mut(&mut self) -> &mut CLWaypointListStartType {
        self._glacier_base.start_point_mut()
    }
    fn is_reversable(&self) -> &bool {
        self._glacier_base.is_reversable()
    }
    fn is_reversable_mut(&mut self) -> &mut bool {
        self._glacier_base.is_reversable_mut()
    }
    fn max_repititions(&self) -> &u32 {
        self._glacier_base.max_repititions()
    }
    fn max_repititions_mut(&mut self) -> &mut u32 {
        self._glacier_base.max_repititions_mut()
    }
    fn speed_override(&self) -> &CreatureSpeedLevel {
        self._glacier_base.speed_override()
    }
    fn speed_override_mut(&mut self) -> &mut CreatureSpeedLevel {
        self._glacier_base.speed_override_mut()
    }
    fn force_explicit_height(&self) -> &bool {
        self._glacier_base.force_explicit_height()
    }
    fn force_explicit_height_mut(&mut self) -> &mut bool {
        self._glacier_base.force_explicit_height_mut()
    }
}

impl CreatureFollowBaseDataTrait for CreatureFollowWaypointsEntityData {
}

impl super::entity::EntityDataTrait for CreatureFollowWaypointsEntityData {
}

impl super::entity::GameObjectDataTrait for CreatureFollowWaypointsEntityData {
}

impl super::core::DataBusPeerTrait for CreatureFollowWaypointsEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for CreatureFollowWaypointsEntityData {
}

impl super::core::DataContainerTrait for CreatureFollowWaypointsEntityData {
}

pub static CREATUREFOLLOWWAYPOINTSENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointsEntityData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREFOLLOWWAYPOINTSEGMENTENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureFollowWaypointsEntityData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CREATUREFOLLOWWAYPOINTSENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CreatureFollowWaypointsEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CREATUREFOLLOWWAYPOINTSENTITYDATA_TYPE_INFO
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


pub static CREATUREFOLLOWWAYPOINTSENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointsEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureFollowWaypointsEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreatureFollowWaypointSegmentEntityData {
    pub _glacier_base: CreatureFollowBaseData,
    pub realm: super::core::Realm,
    pub type_of_route: super::pathfinding_shared::RouteType,
    pub start_point: CLWaypointListStartType,
    pub is_reversable: bool,
    pub max_repititions: u32,
    pub speed_override: CreatureSpeedLevel,
    pub force_explicit_height: bool,
}

pub trait CreatureFollowWaypointSegmentEntityDataTrait: CreatureFollowBaseDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn type_of_route(&self) -> &super::pathfinding_shared::RouteType;
    fn type_of_route_mut(&mut self) -> &mut super::pathfinding_shared::RouteType;
    fn start_point(&self) -> &CLWaypointListStartType;
    fn start_point_mut(&mut self) -> &mut CLWaypointListStartType;
    fn is_reversable(&self) -> &bool;
    fn is_reversable_mut(&mut self) -> &mut bool;
    fn max_repititions(&self) -> &u32;
    fn max_repititions_mut(&mut self) -> &mut u32;
    fn speed_override(&self) -> &CreatureSpeedLevel;
    fn speed_override_mut(&mut self) -> &mut CreatureSpeedLevel;
    fn force_explicit_height(&self) -> &bool;
    fn force_explicit_height_mut(&mut self) -> &mut bool;
}

impl CreatureFollowWaypointSegmentEntityDataTrait for CreatureFollowWaypointSegmentEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn type_of_route(&self) -> &super::pathfinding_shared::RouteType {
        &self.type_of_route
    }
    fn type_of_route_mut(&mut self) -> &mut super::pathfinding_shared::RouteType {
        &mut self.type_of_route
    }
    fn start_point(&self) -> &CLWaypointListStartType {
        &self.start_point
    }
    fn start_point_mut(&mut self) -> &mut CLWaypointListStartType {
        &mut self.start_point
    }
    fn is_reversable(&self) -> &bool {
        &self.is_reversable
    }
    fn is_reversable_mut(&mut self) -> &mut bool {
        &mut self.is_reversable
    }
    fn max_repititions(&self) -> &u32 {
        &self.max_repititions
    }
    fn max_repititions_mut(&mut self) -> &mut u32 {
        &mut self.max_repititions
    }
    fn speed_override(&self) -> &CreatureSpeedLevel {
        &self.speed_override
    }
    fn speed_override_mut(&mut self) -> &mut CreatureSpeedLevel {
        &mut self.speed_override
    }
    fn force_explicit_height(&self) -> &bool {
        &self.force_explicit_height
    }
    fn force_explicit_height_mut(&mut self) -> &mut bool {
        &mut self.force_explicit_height
    }
}

impl CreatureFollowBaseDataTrait for CreatureFollowWaypointSegmentEntityData {
}

impl super::entity::EntityDataTrait for CreatureFollowWaypointSegmentEntityData {
}

impl super::entity::GameObjectDataTrait for CreatureFollowWaypointSegmentEntityData {
}

impl super::core::DataBusPeerTrait for CreatureFollowWaypointSegmentEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for CreatureFollowWaypointSegmentEntityData {
}

impl super::core::DataContainerTrait for CreatureFollowWaypointSegmentEntityData {
}

pub static CREATUREFOLLOWWAYPOINTSEGMENTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointSegmentEntityData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREFOLLOWBASEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureFollowWaypointSegmentEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(CreatureFollowWaypointSegmentEntityData, realm),
            },
            FieldInfoData {
                name: "TypeOfRoute",
                flags: MemberInfoFlags::new(0),
                field_type: "RouteType",
                rust_offset: offset_of!(CreatureFollowWaypointSegmentEntityData, type_of_route),
            },
            FieldInfoData {
                name: "Start_Point",
                flags: MemberInfoFlags::new(0),
                field_type: "CL_WaypointList_StartType",
                rust_offset: offset_of!(CreatureFollowWaypointSegmentEntityData, start_point),
            },
            FieldInfoData {
                name: "IsReversable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CreatureFollowWaypointSegmentEntityData, is_reversable),
            },
            FieldInfoData {
                name: "MaxRepititions",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(CreatureFollowWaypointSegmentEntityData, max_repititions),
            },
            FieldInfoData {
                name: "SpeedOverride",
                flags: MemberInfoFlags::new(0),
                field_type: "CreatureSpeedLevel",
                rust_offset: offset_of!(CreatureFollowWaypointSegmentEntityData, speed_override),
            },
            FieldInfoData {
                name: "ForceExplicitHeight",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CreatureFollowWaypointSegmentEntityData, force_explicit_height),
            },
        ],
    }),
    array_type: Some(CREATUREFOLLOWWAYPOINTSEGMENTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CreatureFollowWaypointSegmentEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CREATUREFOLLOWWAYPOINTSEGMENTENTITYDATA_TYPE_INFO
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


pub static CREATUREFOLLOWWAYPOINTSEGMENTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointSegmentEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureFollowWaypointSegmentEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreatureFollowBaseData {
    pub _glacier_base: super::entity::EntityData,
}

pub trait CreatureFollowBaseDataTrait: super::entity::EntityDataTrait {
}

impl CreatureFollowBaseDataTrait for CreatureFollowBaseData {
}

impl super::entity::EntityDataTrait for CreatureFollowBaseData {
}

impl super::entity::GameObjectDataTrait for CreatureFollowBaseData {
}

impl super::core::DataBusPeerTrait for CreatureFollowBaseData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for CreatureFollowBaseData {
}

impl super::core::DataContainerTrait for CreatureFollowBaseData {
}

pub static CREATUREFOLLOWBASEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowBaseData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureFollowBaseData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CREATUREFOLLOWBASEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CreatureFollowBaseData {
    fn type_info(&self) -> &'static TypeInfo {
        CREATUREFOLLOWBASEDATA_TYPE_INFO
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


pub static CREATUREFOLLOWBASEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowBaseData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureFollowBaseData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreatureFollowWaypointProviderEntityData {
    pub _glacier_base: CreatureBaseWaypointProviderEntityData,
}

pub trait CreatureFollowWaypointProviderEntityDataTrait: CreatureBaseWaypointProviderEntityDataTrait {
}

impl CreatureFollowWaypointProviderEntityDataTrait for CreatureFollowWaypointProviderEntityData {
}

impl CreatureBaseWaypointProviderEntityDataTrait for CreatureFollowWaypointProviderEntityData {
}

impl super::entity::EntityDataTrait for CreatureFollowWaypointProviderEntityData {
}

impl super::entity::GameObjectDataTrait for CreatureFollowWaypointProviderEntityData {
}

impl super::core::DataBusPeerTrait for CreatureFollowWaypointProviderEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for CreatureFollowWaypointProviderEntityData {
}

impl super::core::DataContainerTrait for CreatureFollowWaypointProviderEntityData {
}

pub static CREATUREFOLLOWWAYPOINTPROVIDERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointProviderEntityData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREBASEWAYPOINTPROVIDERENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureFollowWaypointProviderEntityData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CREATUREFOLLOWWAYPOINTPROVIDERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CreatureFollowWaypointProviderEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CREATUREFOLLOWWAYPOINTPROVIDERENTITYDATA_TYPE_INFO
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


pub static CREATUREFOLLOWWAYPOINTPROVIDERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointProviderEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureFollowWaypointProviderEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum CLWaypointListStartType {
    #[default]
    First_Point = 0,
    Nearest_Point = 1,
    Nearest_Point_Ahead = 2,
}

pub static CL_WAYPOINTLIST_STARTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CL_WaypointList_StartType",
    flags: MemberInfoFlags::new(49429),
    module: "CreatureLocoShared",
    data: TypeInfoData::Enum,
    array_type: Some(CL_WAYPOINTLIST_STARTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CLWaypointListStartType {
    fn type_info(&self) -> &'static TypeInfo {
        CL_WAYPOINTLIST_STARTTYPE_TYPE_INFO
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


pub static CL_WAYPOINTLIST_STARTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CL_WaypointList_StartType-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CL_WaypointList_StartType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreatureCollisionGroupData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub avoidance_threshold: f32,
    pub average_group_size: i32,
}

pub trait CreatureCollisionGroupDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn avoidance_threshold(&self) -> &f32;
    fn avoidance_threshold_mut(&mut self) -> &mut f32;
    fn average_group_size(&self) -> &i32;
    fn average_group_size_mut(&mut self) -> &mut i32;
}

impl CreatureCollisionGroupDataTrait for CreatureCollisionGroupData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn avoidance_threshold(&self) -> &f32 {
        &self.avoidance_threshold
    }
    fn avoidance_threshold_mut(&mut self) -> &mut f32 {
        &mut self.avoidance_threshold
    }
    fn average_group_size(&self) -> &i32 {
        &self.average_group_size
    }
    fn average_group_size_mut(&mut self) -> &mut i32 {
        &mut self.average_group_size
    }
}

impl super::entity::EntityDataTrait for CreatureCollisionGroupData {
}

impl super::entity::GameObjectDataTrait for CreatureCollisionGroupData {
}

impl super::core::DataBusPeerTrait for CreatureCollisionGroupData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for CreatureCollisionGroupData {
}

impl super::core::DataContainerTrait for CreatureCollisionGroupData {
}

pub static CREATURECOLLISIONGROUPDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureCollisionGroupData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureCollisionGroupData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(CreatureCollisionGroupData, realm),
            },
            FieldInfoData {
                name: "AvoidanceThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CreatureCollisionGroupData, avoidance_threshold),
            },
            FieldInfoData {
                name: "AverageGroupSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(CreatureCollisionGroupData, average_group_size),
            },
        ],
    }),
    array_type: Some(CREATURECOLLISIONGROUPDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CreatureCollisionGroupData {
    fn type_info(&self) -> &'static TypeInfo {
        CREATURECOLLISIONGROUPDATA_TYPE_INFO
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


pub static CREATURECOLLISIONGROUPDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureCollisionGroupData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureCollisionGroupData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CLInfluenceFilterEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub influence_type: CreatureLocoExternalInfluenceType,
}

pub trait CLInfluenceFilterEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn influence_type(&self) -> &CreatureLocoExternalInfluenceType;
    fn influence_type_mut(&mut self) -> &mut CreatureLocoExternalInfluenceType;
}

impl CLInfluenceFilterEntityDataTrait for CLInfluenceFilterEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn influence_type(&self) -> &CreatureLocoExternalInfluenceType {
        &self.influence_type
    }
    fn influence_type_mut(&mut self) -> &mut CreatureLocoExternalInfluenceType {
        &mut self.influence_type
    }
}

impl super::entity::EntityDataTrait for CLInfluenceFilterEntityData {
}

impl super::entity::GameObjectDataTrait for CLInfluenceFilterEntityData {
}

impl super::core::DataBusPeerTrait for CLInfluenceFilterEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for CLInfluenceFilterEntityData {
}

impl super::core::DataContainerTrait for CLInfluenceFilterEntityData {
}

pub static CLINFLUENCEFILTERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLInfluenceFilterEntityData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CLInfluenceFilterEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(CLInfluenceFilterEntityData, realm),
            },
            FieldInfoData {
                name: "InfluenceType",
                flags: MemberInfoFlags::new(0),
                field_type: "CreatureLocoExternalInfluenceType",
                rust_offset: offset_of!(CLInfluenceFilterEntityData, influence_type),
            },
        ],
    }),
    array_type: Some(CLINFLUENCEFILTERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CLInfluenceFilterEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CLINFLUENCEFILTERENTITYDATA_TYPE_INFO
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


pub static CLINFLUENCEFILTERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLInfluenceFilterEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CLInfluenceFilterEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CLInfluenceCompareEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub influence_a: CreatureLocoExternalInfluenceType,
    pub influence_b: CreatureLocoExternalInfluenceType,
}

pub trait CLInfluenceCompareEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn influence_a(&self) -> &CreatureLocoExternalInfluenceType;
    fn influence_a_mut(&mut self) -> &mut CreatureLocoExternalInfluenceType;
    fn influence_b(&self) -> &CreatureLocoExternalInfluenceType;
    fn influence_b_mut(&mut self) -> &mut CreatureLocoExternalInfluenceType;
}

impl CLInfluenceCompareEntityDataTrait for CLInfluenceCompareEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn influence_a(&self) -> &CreatureLocoExternalInfluenceType {
        &self.influence_a
    }
    fn influence_a_mut(&mut self) -> &mut CreatureLocoExternalInfluenceType {
        &mut self.influence_a
    }
    fn influence_b(&self) -> &CreatureLocoExternalInfluenceType {
        &self.influence_b
    }
    fn influence_b_mut(&mut self) -> &mut CreatureLocoExternalInfluenceType {
        &mut self.influence_b
    }
}

impl super::entity::EntityDataTrait for CLInfluenceCompareEntityData {
}

impl super::entity::GameObjectDataTrait for CLInfluenceCompareEntityData {
}

impl super::core::DataBusPeerTrait for CLInfluenceCompareEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for CLInfluenceCompareEntityData {
}

impl super::core::DataContainerTrait for CLInfluenceCompareEntityData {
}

pub static CLINFLUENCECOMPAREENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLInfluenceCompareEntityData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CLInfluenceCompareEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(CLInfluenceCompareEntityData, realm),
            },
            FieldInfoData {
                name: "Influence_A",
                flags: MemberInfoFlags::new(0),
                field_type: "CreatureLocoExternalInfluenceType",
                rust_offset: offset_of!(CLInfluenceCompareEntityData, influence_a),
            },
            FieldInfoData {
                name: "Influence_B",
                flags: MemberInfoFlags::new(0),
                field_type: "CreatureLocoExternalInfluenceType",
                rust_offset: offset_of!(CLInfluenceCompareEntityData, influence_b),
            },
        ],
    }),
    array_type: Some(CLINFLUENCECOMPAREENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CLInfluenceCompareEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CLINFLUENCECOMPAREENTITYDATA_TYPE_INFO
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


pub static CLINFLUENCECOMPAREENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLInfluenceCompareEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CLInfluenceCompareEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CLApplyInfluenceEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub influence_type: CreatureLocoExternalInfluenceType,
    pub location: super::core::Vec3,
    pub radius: f32,
    pub direction: super::core::Vec3,
    pub cone_angle: f32,
    pub is_omnidirectional: bool,
    pub realm: super::core::Realm,
}

pub trait CLApplyInfluenceEntityDataTrait: super::entity::EntityDataTrait {
    fn influence_type(&self) -> &CreatureLocoExternalInfluenceType;
    fn influence_type_mut(&mut self) -> &mut CreatureLocoExternalInfluenceType;
    fn location(&self) -> &super::core::Vec3;
    fn location_mut(&mut self) -> &mut super::core::Vec3;
    fn radius(&self) -> &f32;
    fn radius_mut(&mut self) -> &mut f32;
    fn direction(&self) -> &super::core::Vec3;
    fn direction_mut(&mut self) -> &mut super::core::Vec3;
    fn cone_angle(&self) -> &f32;
    fn cone_angle_mut(&mut self) -> &mut f32;
    fn is_omnidirectional(&self) -> &bool;
    fn is_omnidirectional_mut(&mut self) -> &mut bool;
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
}

impl CLApplyInfluenceEntityDataTrait for CLApplyInfluenceEntityData {
    fn influence_type(&self) -> &CreatureLocoExternalInfluenceType {
        &self.influence_type
    }
    fn influence_type_mut(&mut self) -> &mut CreatureLocoExternalInfluenceType {
        &mut self.influence_type
    }
    fn location(&self) -> &super::core::Vec3 {
        &self.location
    }
    fn location_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.location
    }
    fn radius(&self) -> &f32 {
        &self.radius
    }
    fn radius_mut(&mut self) -> &mut f32 {
        &mut self.radius
    }
    fn direction(&self) -> &super::core::Vec3 {
        &self.direction
    }
    fn direction_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.direction
    }
    fn cone_angle(&self) -> &f32 {
        &self.cone_angle
    }
    fn cone_angle_mut(&mut self) -> &mut f32 {
        &mut self.cone_angle
    }
    fn is_omnidirectional(&self) -> &bool {
        &self.is_omnidirectional
    }
    fn is_omnidirectional_mut(&mut self) -> &mut bool {
        &mut self.is_omnidirectional
    }
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
}

impl super::entity::EntityDataTrait for CLApplyInfluenceEntityData {
}

impl super::entity::GameObjectDataTrait for CLApplyInfluenceEntityData {
}

impl super::core::DataBusPeerTrait for CLApplyInfluenceEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for CLApplyInfluenceEntityData {
}

impl super::core::DataContainerTrait for CLApplyInfluenceEntityData {
}

pub static CLAPPLYINFLUENCEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLApplyInfluenceEntityData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CLApplyInfluenceEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "InfluenceType",
                flags: MemberInfoFlags::new(0),
                field_type: "CreatureLocoExternalInfluenceType",
                rust_offset: offset_of!(CLApplyInfluenceEntityData, influence_type),
            },
            FieldInfoData {
                name: "Location",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(CLApplyInfluenceEntityData, location),
            },
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CLApplyInfluenceEntityData, radius),
            },
            FieldInfoData {
                name: "Direction",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(CLApplyInfluenceEntityData, direction),
            },
            FieldInfoData {
                name: "ConeAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CLApplyInfluenceEntityData, cone_angle),
            },
            FieldInfoData {
                name: "IsOmnidirectional",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CLApplyInfluenceEntityData, is_omnidirectional),
            },
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(CLApplyInfluenceEntityData, realm),
            },
        ],
    }),
    array_type: Some(CLAPPLYINFLUENCEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CLApplyInfluenceEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CLAPPLYINFLUENCEENTITYDATA_TYPE_INFO
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


pub static CLAPPLYINFLUENCEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLApplyInfluenceEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CLApplyInfluenceEntityData"),
    array_type: None,
    alignment: 8,
};


