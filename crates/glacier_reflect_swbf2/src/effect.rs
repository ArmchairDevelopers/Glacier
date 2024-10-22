use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_effect_types(registry: &mut TypeRegistry) {
    registry.register_type(VISUALENVIRONMENTEFFECTENTITYDATA_TYPE_INFO);
    registry.register_type(VISUALENVIRONMENTEFFECTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SOUNDDYNAMICSTATE_TYPE_INFO);
    registry.register_type(SOUNDDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(SOUNDSTATE_TYPE_INFO);
    registry.register_type(SOUNDSTATE_ARRAY_TYPE_INFO);
    registry.register_type(SOUNDSTATICSTATE_TYPE_INFO);
    registry.register_type(SOUNDSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(LOCATIONEFFECTENTITYDATA_TYPE_INFO);
    registry.register_type(LOCATIONEFFECTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(LOCATIONTYPE_TYPE_INFO);
    registry.register_type(LOCATIONTYPE_ARRAY_TYPE_INFO);
    registry.register_type(LIGHTEFFECTENTITYDATA_TYPE_INFO);
    registry.register_type(LIGHTEFFECTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERSYSTEMCOMPONENT_TYPE_INFO);
    registry.register_type(EMITTERSYSTEMCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(EMITTEREXCLUSIONVOLUMEDATA_TYPE_INFO);
    registry.register_type(EMITTEREXCLUSIONVOLUMEDATA_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERENTITYDATA_TYPE_INFO);
    registry.register_type(EMITTERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERGRAPHENTITYDATA_TYPE_INFO);
    registry.register_type(EMITTERGRAPHENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERCHILDEFFECTENTITYDATA_TYPE_INFO);
    registry.register_type(EMITTERCHILDEFFECTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SPAWNPROBABILITYRANDOMTYPE_TYPE_INFO);
    registry.register_type(SPAWNPROBABILITYRANDOMTYPE_ARRAY_TYPE_INFO);
    registry.register_type(EFFECTSYSTEMSETTINGS_TYPE_INFO);
    registry.register_type(EFFECTSYSTEMSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(EFFECTSYSTEMCOMPONENT_TYPE_INFO);
    registry.register_type(EFFECTSYSTEMCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(BLUEPRINTEFFECTENTITYDATA_TYPE_INFO);
    registry.register_type(BLUEPRINTEFFECTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(EFFECTASSET_TYPE_INFO);
    registry.register_type(EFFECTASSET_ARRAY_TYPE_INFO);
    registry.register_type(EFFECTENTITYDATA_TYPE_INFO);
    registry.register_type(EFFECTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(EFFECTENTITY_TYPE_INFO);
    registry.register_type(EFFECTENTITY_ARRAY_TYPE_INFO);
}

#[derive(Clone, Debug, Default)]
pub struct VisualEnvironmentEffectEntityData {
    pub _glacier_base: super::entity::ChildEffectEntityData,
    pub visual_environment: Option<Arc<Mutex<dyn super::world_sim::VisualEnvironmentBlueprintTrait>>>,
    pub lifetime: f32,
    pub lifetime_curve: super::core::Vec4,
    pub sample_on_start_only: bool,
    pub cull_angle_curve: super::core::Vec4,
    pub cull_distance: super::core::QualityScalableFloat,
    pub cull_distance_curve: super::core::Vec4,
    pub hide_occluded: bool,
}

pub trait VisualEnvironmentEffectEntityDataTrait: super::entity::ChildEffectEntityDataTrait {
    fn visual_environment(&self) -> &Option<Arc<Mutex<dyn super::world_sim::VisualEnvironmentBlueprintTrait>>>;
    fn visual_environment_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::world_sim::VisualEnvironmentBlueprintTrait>>>;
    fn lifetime(&self) -> &f32;
    fn lifetime_mut(&mut self) -> &mut f32;
    fn lifetime_curve(&self) -> &super::core::Vec4;
    fn lifetime_curve_mut(&mut self) -> &mut super::core::Vec4;
    fn sample_on_start_only(&self) -> &bool;
    fn sample_on_start_only_mut(&mut self) -> &mut bool;
    fn cull_angle_curve(&self) -> &super::core::Vec4;
    fn cull_angle_curve_mut(&mut self) -> &mut super::core::Vec4;
    fn cull_distance(&self) -> &super::core::QualityScalableFloat;
    fn cull_distance_mut(&mut self) -> &mut super::core::QualityScalableFloat;
    fn cull_distance_curve(&self) -> &super::core::Vec4;
    fn cull_distance_curve_mut(&mut self) -> &mut super::core::Vec4;
    fn hide_occluded(&self) -> &bool;
    fn hide_occluded_mut(&mut self) -> &mut bool;
}

impl VisualEnvironmentEffectEntityDataTrait for VisualEnvironmentEffectEntityData {
    fn visual_environment(&self) -> &Option<Arc<Mutex<dyn super::world_sim::VisualEnvironmentBlueprintTrait>>> {
        &self.visual_environment
    }
    fn visual_environment_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::world_sim::VisualEnvironmentBlueprintTrait>>> {
        &mut self.visual_environment
    }
    fn lifetime(&self) -> &f32 {
        &self.lifetime
    }
    fn lifetime_mut(&mut self) -> &mut f32 {
        &mut self.lifetime
    }
    fn lifetime_curve(&self) -> &super::core::Vec4 {
        &self.lifetime_curve
    }
    fn lifetime_curve_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.lifetime_curve
    }
    fn sample_on_start_only(&self) -> &bool {
        &self.sample_on_start_only
    }
    fn sample_on_start_only_mut(&mut self) -> &mut bool {
        &mut self.sample_on_start_only
    }
    fn cull_angle_curve(&self) -> &super::core::Vec4 {
        &self.cull_angle_curve
    }
    fn cull_angle_curve_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.cull_angle_curve
    }
    fn cull_distance(&self) -> &super::core::QualityScalableFloat {
        &self.cull_distance
    }
    fn cull_distance_mut(&mut self) -> &mut super::core::QualityScalableFloat {
        &mut self.cull_distance
    }
    fn cull_distance_curve(&self) -> &super::core::Vec4 {
        &self.cull_distance_curve
    }
    fn cull_distance_curve_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.cull_distance_curve
    }
    fn hide_occluded(&self) -> &bool {
        &self.hide_occluded
    }
    fn hide_occluded_mut(&mut self) -> &mut bool {
        &mut self.hide_occluded
    }
}

impl super::entity::ChildEffectEntityDataTrait for VisualEnvironmentEffectEntityData {
    fn components(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components_mut()
    }
    fn start_delay(&self) -> &f32 {
        self._glacier_base.start_delay()
    }
    fn start_delay_mut(&mut self) -> &mut f32 {
        self._glacier_base.start_delay_mut()
    }
    fn attach_to_spawn_surface(&self) -> &bool {
        self._glacier_base.attach_to_spawn_surface()
    }
    fn attach_to_spawn_surface_mut(&mut self) -> &mut bool {
        self._glacier_base.attach_to_spawn_surface_mut()
    }
    fn enable(&self) -> &super::core::QualityScalableBool {
        self._glacier_base.enable()
    }
    fn enable_mut(&mut self) -> &mut super::core::QualityScalableBool {
        self._glacier_base.enable_mut()
    }
    fn override_draw_order(&self) -> &bool {
        self._glacier_base.override_draw_order()
    }
    fn override_draw_order_mut(&mut self) -> &mut bool {
        self._glacier_base.override_draw_order_mut()
    }
    fn keep_alive(&self) -> &bool {
        self._glacier_base.keep_alive()
    }
    fn keep_alive_mut(&mut self) -> &mut bool {
        self._glacier_base.keep_alive_mut()
    }
}

impl super::entity::SpatialEntityDataTrait for VisualEnvironmentEffectEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::entity::EntityDataTrait for VisualEnvironmentEffectEntityData {
}

impl super::entity::GameObjectDataTrait for VisualEnvironmentEffectEntityData {
}

impl super::core::DataBusPeerTrait for VisualEnvironmentEffectEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for VisualEnvironmentEffectEntityData {
}

impl super::core::DataContainerTrait for VisualEnvironmentEffectEntityData {
}

pub static VISUALENVIRONMENTEFFECTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualEnvironmentEffectEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Effect",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::CHILDEFFECTENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VisualEnvironmentEffectEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "VisualEnvironment",
                flags: MemberInfoFlags::new(0),
                field_type: "VisualEnvironmentBlueprint",
                rust_offset: offset_of!(VisualEnvironmentEffectEntityData, visual_environment),
            },
            FieldInfoData {
                name: "Lifetime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualEnvironmentEffectEntityData, lifetime),
            },
            FieldInfoData {
                name: "LifetimeCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(VisualEnvironmentEffectEntityData, lifetime_curve),
            },
            FieldInfoData {
                name: "SampleOnStartOnly",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualEnvironmentEffectEntityData, sample_on_start_only),
            },
            FieldInfoData {
                name: "CullAngleCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(VisualEnvironmentEffectEntityData, cull_angle_curve),
            },
            FieldInfoData {
                name: "CullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(VisualEnvironmentEffectEntityData, cull_distance),
            },
            FieldInfoData {
                name: "CullDistanceCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(VisualEnvironmentEffectEntityData, cull_distance_curve),
            },
            FieldInfoData {
                name: "HideOccluded",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualEnvironmentEffectEntityData, hide_occluded),
            },
        ],
    }),
    array_type: Some(VISUALENVIRONMENTEFFECTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for VisualEnvironmentEffectEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        VISUALENVIRONMENTEFFECTENTITYDATA_TYPE_INFO
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


pub static VISUALENVIRONMENTEFFECTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualEnvironmentEffectEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Effect",
    data: TypeInfoData::Array("VisualEnvironmentEffectEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SoundDynamicState {
    pub transform: super::core::LinearTransform,
    pub state: SoundState,
    pub params: super::effect_base::EffectParams,
    pub field_flag_changed0: u8,
}

pub trait SoundDynamicStateTrait: TypeObject {
    fn transform(&self) -> &super::core::LinearTransform;
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform;
    fn state(&self) -> &SoundState;
    fn state_mut(&mut self) -> &mut SoundState;
    fn params(&self) -> &super::effect_base::EffectParams;
    fn params_mut(&mut self) -> &mut super::effect_base::EffectParams;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl SoundDynamicStateTrait for SoundDynamicState {
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.transform
    }
    fn state(&self) -> &SoundState {
        &self.state
    }
    fn state_mut(&mut self) -> &mut SoundState {
        &mut self.state
    }
    fn params(&self) -> &super::effect_base::EffectParams {
        &self.params
    }
    fn params_mut(&mut self) -> &mut super::effect_base::EffectParams {
        &mut self.params
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static SOUNDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoundDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "Effect",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SoundDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(SoundDynamicState, transform),
            },
            FieldInfoData {
                name: "State",
                flags: MemberInfoFlags::new(0),
                field_type: "SoundState",
                rust_offset: offset_of!(SoundDynamicState, state),
            },
            FieldInfoData {
                name: "Params",
                flags: MemberInfoFlags::new(0),
                field_type: "EffectParams",
                rust_offset: offset_of!(SoundDynamicState, params),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(SoundDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SOUNDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SoundDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        SOUNDDYNAMICSTATE_TYPE_INFO
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


pub static SOUNDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoundDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Effect",
    data: TypeInfoData::Array("SoundDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum SoundState {
    #[default]
    SoundState_Waiting = 0,
    SoundState_Playing = 1,
    SoundState_Stopping = 2,
    SoundState_Dead = 3,
}

pub static SOUNDSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoundState",
    flags: MemberInfoFlags::new(49429),
    module: "Effect",
    data: TypeInfoData::Enum,
    array_type: Some(SOUNDSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SoundState {
    fn type_info(&self) -> &'static TypeInfo {
        SOUNDSTATE_TYPE_INFO
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


pub static SOUNDSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoundState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Effect",
    data: TypeInfoData::Array("SoundState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SoundStaticState {
    pub asset: Option<Arc<Mutex<dyn super::audio::SoundAssetTrait>>>,
    pub is_local_player: bool,
    pub is_first_person: bool,
    pub group_guid: glacier_util::guid::Guid,
    pub max_instance_count_in_group: u32,
    pub kill_on_max_count: bool,
    pub field_flag_changed0: u8,
}

pub trait SoundStaticStateTrait: TypeObject {
    fn asset(&self) -> &Option<Arc<Mutex<dyn super::audio::SoundAssetTrait>>>;
    fn asset_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::audio::SoundAssetTrait>>>;
    fn is_local_player(&self) -> &bool;
    fn is_local_player_mut(&mut self) -> &mut bool;
    fn is_first_person(&self) -> &bool;
    fn is_first_person_mut(&mut self) -> &mut bool;
    fn group_guid(&self) -> &glacier_util::guid::Guid;
    fn group_guid_mut(&mut self) -> &mut glacier_util::guid::Guid;
    fn max_instance_count_in_group(&self) -> &u32;
    fn max_instance_count_in_group_mut(&mut self) -> &mut u32;
    fn kill_on_max_count(&self) -> &bool;
    fn kill_on_max_count_mut(&mut self) -> &mut bool;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl SoundStaticStateTrait for SoundStaticState {
    fn asset(&self) -> &Option<Arc<Mutex<dyn super::audio::SoundAssetTrait>>> {
        &self.asset
    }
    fn asset_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::audio::SoundAssetTrait>>> {
        &mut self.asset
    }
    fn is_local_player(&self) -> &bool {
        &self.is_local_player
    }
    fn is_local_player_mut(&mut self) -> &mut bool {
        &mut self.is_local_player
    }
    fn is_first_person(&self) -> &bool {
        &self.is_first_person
    }
    fn is_first_person_mut(&mut self) -> &mut bool {
        &mut self.is_first_person
    }
    fn group_guid(&self) -> &glacier_util::guid::Guid {
        &self.group_guid
    }
    fn group_guid_mut(&mut self) -> &mut glacier_util::guid::Guid {
        &mut self.group_guid
    }
    fn max_instance_count_in_group(&self) -> &u32 {
        &self.max_instance_count_in_group
    }
    fn max_instance_count_in_group_mut(&mut self) -> &mut u32 {
        &mut self.max_instance_count_in_group
    }
    fn kill_on_max_count(&self) -> &bool {
        &self.kill_on_max_count
    }
    fn kill_on_max_count_mut(&mut self) -> &mut bool {
        &mut self.kill_on_max_count
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static SOUNDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoundStaticState",
    flags: MemberInfoFlags::new(73),
    module: "Effect",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SoundStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Asset",
                flags: MemberInfoFlags::new(0),
                field_type: "SoundAsset",
                rust_offset: offset_of!(SoundStaticState, asset),
            },
            FieldInfoData {
                name: "IsLocalPlayer",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SoundStaticState, is_local_player),
            },
            FieldInfoData {
                name: "IsFirstPerson",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SoundStaticState, is_first_person),
            },
            FieldInfoData {
                name: "GroupGuid",
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(SoundStaticState, group_guid),
            },
            FieldInfoData {
                name: "MaxInstanceCountInGroup",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(SoundStaticState, max_instance_count_in_group),
            },
            FieldInfoData {
                name: "KillOnMaxCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SoundStaticState, kill_on_max_count),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(SoundStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SOUNDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SoundStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        SOUNDSTATICSTATE_TYPE_INFO
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


pub static SOUNDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoundStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Effect",
    data: TypeInfoData::Array("SoundStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LocationEffectEntityData {
    pub _glacier_base: super::entity::ChildEffectEntityData,
    pub location: LocationType,
    pub ctrl_point_index: u32,
}

pub trait LocationEffectEntityDataTrait: super::entity::ChildEffectEntityDataTrait {
    fn location(&self) -> &LocationType;
    fn location_mut(&mut self) -> &mut LocationType;
    fn ctrl_point_index(&self) -> &u32;
    fn ctrl_point_index_mut(&mut self) -> &mut u32;
}

impl LocationEffectEntityDataTrait for LocationEffectEntityData {
    fn location(&self) -> &LocationType {
        &self.location
    }
    fn location_mut(&mut self) -> &mut LocationType {
        &mut self.location
    }
    fn ctrl_point_index(&self) -> &u32 {
        &self.ctrl_point_index
    }
    fn ctrl_point_index_mut(&mut self) -> &mut u32 {
        &mut self.ctrl_point_index
    }
}

impl super::entity::ChildEffectEntityDataTrait for LocationEffectEntityData {
    fn components(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components_mut()
    }
    fn start_delay(&self) -> &f32 {
        self._glacier_base.start_delay()
    }
    fn start_delay_mut(&mut self) -> &mut f32 {
        self._glacier_base.start_delay_mut()
    }
    fn attach_to_spawn_surface(&self) -> &bool {
        self._glacier_base.attach_to_spawn_surface()
    }
    fn attach_to_spawn_surface_mut(&mut self) -> &mut bool {
        self._glacier_base.attach_to_spawn_surface_mut()
    }
    fn enable(&self) -> &super::core::QualityScalableBool {
        self._glacier_base.enable()
    }
    fn enable_mut(&mut self) -> &mut super::core::QualityScalableBool {
        self._glacier_base.enable_mut()
    }
    fn override_draw_order(&self) -> &bool {
        self._glacier_base.override_draw_order()
    }
    fn override_draw_order_mut(&mut self) -> &mut bool {
        self._glacier_base.override_draw_order_mut()
    }
    fn keep_alive(&self) -> &bool {
        self._glacier_base.keep_alive()
    }
    fn keep_alive_mut(&mut self) -> &mut bool {
        self._glacier_base.keep_alive_mut()
    }
}

impl super::entity::SpatialEntityDataTrait for LocationEffectEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::entity::EntityDataTrait for LocationEffectEntityData {
}

impl super::entity::GameObjectDataTrait for LocationEffectEntityData {
}

impl super::core::DataBusPeerTrait for LocationEffectEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for LocationEffectEntityData {
}

impl super::core::DataContainerTrait for LocationEffectEntityData {
}

pub static LOCATIONEFFECTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocationEffectEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Effect",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::CHILDEFFECTENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LocationEffectEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Location",
                flags: MemberInfoFlags::new(0),
                field_type: "LocationType",
                rust_offset: offset_of!(LocationEffectEntityData, location),
            },
            FieldInfoData {
                name: "CtrlPointIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(LocationEffectEntityData, ctrl_point_index),
            },
        ],
    }),
    array_type: Some(LOCATIONEFFECTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LocationEffectEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        LOCATIONEFFECTENTITYDATA_TYPE_INFO
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


pub static LOCATIONEFFECTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocationEffectEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Effect",
    data: TypeInfoData::Array("LocationEffectEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum LocationType {
    #[default]
    LtSource = 0,
    LtTarget = 1,
    LtOther = 2,
    LtCtrlPoint = 3,
}

pub static LOCATIONTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocationType",
    flags: MemberInfoFlags::new(49429),
    module: "Effect",
    data: TypeInfoData::Enum,
    array_type: Some(LOCATIONTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LocationType {
    fn type_info(&self) -> &'static TypeInfo {
        LOCATIONTYPE_TYPE_INFO
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


pub static LOCATIONTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocationType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Effect",
    data: TypeInfoData::Array("LocationType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LightEffectEntityData {
    pub _glacier_base: super::entity::ChildEffectEntityData,
    pub light: Option<Arc<Mutex<dyn super::world_sim::LocalLightEntityDataTrait>>>,
    pub lifetime: f32,
    pub looping: bool,
    pub random_intensity_freq: f32,
    pub spawn_probability: super::core::QualityScalableFloat,
    pub local_player_only: bool,
    pub intensity_multiplier: f32,
    pub tube_width: f32,
    pub random_intensity_min: f32,
    pub random_intensity_max: f32,
    pub intensity_curve: super::core::Vec4,
    pub intensity_min: f32,
    pub intensity_max: f32,
}

pub trait LightEffectEntityDataTrait: super::entity::ChildEffectEntityDataTrait {
    fn light(&self) -> &Option<Arc<Mutex<dyn super::world_sim::LocalLightEntityDataTrait>>>;
    fn light_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::world_sim::LocalLightEntityDataTrait>>>;
    fn lifetime(&self) -> &f32;
    fn lifetime_mut(&mut self) -> &mut f32;
    fn looping(&self) -> &bool;
    fn looping_mut(&mut self) -> &mut bool;
    fn random_intensity_freq(&self) -> &f32;
    fn random_intensity_freq_mut(&mut self) -> &mut f32;
    fn spawn_probability(&self) -> &super::core::QualityScalableFloat;
    fn spawn_probability_mut(&mut self) -> &mut super::core::QualityScalableFloat;
    fn local_player_only(&self) -> &bool;
    fn local_player_only_mut(&mut self) -> &mut bool;
    fn intensity_multiplier(&self) -> &f32;
    fn intensity_multiplier_mut(&mut self) -> &mut f32;
    fn tube_width(&self) -> &f32;
    fn tube_width_mut(&mut self) -> &mut f32;
    fn random_intensity_min(&self) -> &f32;
    fn random_intensity_min_mut(&mut self) -> &mut f32;
    fn random_intensity_max(&self) -> &f32;
    fn random_intensity_max_mut(&mut self) -> &mut f32;
    fn intensity_curve(&self) -> &super::core::Vec4;
    fn intensity_curve_mut(&mut self) -> &mut super::core::Vec4;
    fn intensity_min(&self) -> &f32;
    fn intensity_min_mut(&mut self) -> &mut f32;
    fn intensity_max(&self) -> &f32;
    fn intensity_max_mut(&mut self) -> &mut f32;
}

impl LightEffectEntityDataTrait for LightEffectEntityData {
    fn light(&self) -> &Option<Arc<Mutex<dyn super::world_sim::LocalLightEntityDataTrait>>> {
        &self.light
    }
    fn light_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::world_sim::LocalLightEntityDataTrait>>> {
        &mut self.light
    }
    fn lifetime(&self) -> &f32 {
        &self.lifetime
    }
    fn lifetime_mut(&mut self) -> &mut f32 {
        &mut self.lifetime
    }
    fn looping(&self) -> &bool {
        &self.looping
    }
    fn looping_mut(&mut self) -> &mut bool {
        &mut self.looping
    }
    fn random_intensity_freq(&self) -> &f32 {
        &self.random_intensity_freq
    }
    fn random_intensity_freq_mut(&mut self) -> &mut f32 {
        &mut self.random_intensity_freq
    }
    fn spawn_probability(&self) -> &super::core::QualityScalableFloat {
        &self.spawn_probability
    }
    fn spawn_probability_mut(&mut self) -> &mut super::core::QualityScalableFloat {
        &mut self.spawn_probability
    }
    fn local_player_only(&self) -> &bool {
        &self.local_player_only
    }
    fn local_player_only_mut(&mut self) -> &mut bool {
        &mut self.local_player_only
    }
    fn intensity_multiplier(&self) -> &f32 {
        &self.intensity_multiplier
    }
    fn intensity_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.intensity_multiplier
    }
    fn tube_width(&self) -> &f32 {
        &self.tube_width
    }
    fn tube_width_mut(&mut self) -> &mut f32 {
        &mut self.tube_width
    }
    fn random_intensity_min(&self) -> &f32 {
        &self.random_intensity_min
    }
    fn random_intensity_min_mut(&mut self) -> &mut f32 {
        &mut self.random_intensity_min
    }
    fn random_intensity_max(&self) -> &f32 {
        &self.random_intensity_max
    }
    fn random_intensity_max_mut(&mut self) -> &mut f32 {
        &mut self.random_intensity_max
    }
    fn intensity_curve(&self) -> &super::core::Vec4 {
        &self.intensity_curve
    }
    fn intensity_curve_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.intensity_curve
    }
    fn intensity_min(&self) -> &f32 {
        &self.intensity_min
    }
    fn intensity_min_mut(&mut self) -> &mut f32 {
        &mut self.intensity_min
    }
    fn intensity_max(&self) -> &f32 {
        &self.intensity_max
    }
    fn intensity_max_mut(&mut self) -> &mut f32 {
        &mut self.intensity_max
    }
}

impl super::entity::ChildEffectEntityDataTrait for LightEffectEntityData {
    fn components(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components_mut()
    }
    fn start_delay(&self) -> &f32 {
        self._glacier_base.start_delay()
    }
    fn start_delay_mut(&mut self) -> &mut f32 {
        self._glacier_base.start_delay_mut()
    }
    fn attach_to_spawn_surface(&self) -> &bool {
        self._glacier_base.attach_to_spawn_surface()
    }
    fn attach_to_spawn_surface_mut(&mut self) -> &mut bool {
        self._glacier_base.attach_to_spawn_surface_mut()
    }
    fn enable(&self) -> &super::core::QualityScalableBool {
        self._glacier_base.enable()
    }
    fn enable_mut(&mut self) -> &mut super::core::QualityScalableBool {
        self._glacier_base.enable_mut()
    }
    fn override_draw_order(&self) -> &bool {
        self._glacier_base.override_draw_order()
    }
    fn override_draw_order_mut(&mut self) -> &mut bool {
        self._glacier_base.override_draw_order_mut()
    }
    fn keep_alive(&self) -> &bool {
        self._glacier_base.keep_alive()
    }
    fn keep_alive_mut(&mut self) -> &mut bool {
        self._glacier_base.keep_alive_mut()
    }
}

impl super::entity::SpatialEntityDataTrait for LightEffectEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::entity::EntityDataTrait for LightEffectEntityData {
}

impl super::entity::GameObjectDataTrait for LightEffectEntityData {
}

impl super::core::DataBusPeerTrait for LightEffectEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for LightEffectEntityData {
}

impl super::core::DataContainerTrait for LightEffectEntityData {
}

pub static LIGHTEFFECTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightEffectEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Effect",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::CHILDEFFECTENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LightEffectEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Light",
                flags: MemberInfoFlags::new(0),
                field_type: "LocalLightEntityData",
                rust_offset: offset_of!(LightEffectEntityData, light),
            },
            FieldInfoData {
                name: "Lifetime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LightEffectEntityData, lifetime),
            },
            FieldInfoData {
                name: "Looping",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LightEffectEntityData, looping),
            },
            FieldInfoData {
                name: "RandomIntensityFreq",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LightEffectEntityData, random_intensity_freq),
            },
            FieldInfoData {
                name: "SpawnProbability",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(LightEffectEntityData, spawn_probability),
            },
            FieldInfoData {
                name: "LocalPlayerOnly",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LightEffectEntityData, local_player_only),
            },
            FieldInfoData {
                name: "IntensityMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LightEffectEntityData, intensity_multiplier),
            },
            FieldInfoData {
                name: "TubeWidth",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LightEffectEntityData, tube_width),
            },
            FieldInfoData {
                name: "RandomIntensityMin",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LightEffectEntityData, random_intensity_min),
            },
            FieldInfoData {
                name: "RandomIntensityMax",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LightEffectEntityData, random_intensity_max),
            },
            FieldInfoData {
                name: "IntensityCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(LightEffectEntityData, intensity_curve),
            },
            FieldInfoData {
                name: "IntensityMin",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LightEffectEntityData, intensity_min),
            },
            FieldInfoData {
                name: "IntensityMax",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LightEffectEntityData, intensity_max),
            },
        ],
    }),
    array_type: Some(LIGHTEFFECTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LightEffectEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        LIGHTEFFECTENTITYDATA_TYPE_INFO
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


pub static LIGHTEFFECTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightEffectEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Effect",
    data: TypeInfoData::Array("LightEffectEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EmitterSystemComponent {
    pub _glacier_base: super::entity::SubWorldDataComponent,
    pub exclusion_volumes: Option<Arc<Mutex<dyn super::emitter_base::EmitterExclusionVolumesBaseAssetTrait>>>,
}

pub trait EmitterSystemComponentTrait: super::entity::SubWorldDataComponentTrait {
    fn exclusion_volumes(&self) -> &Option<Arc<Mutex<dyn super::emitter_base::EmitterExclusionVolumesBaseAssetTrait>>>;
    fn exclusion_volumes_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::emitter_base::EmitterExclusionVolumesBaseAssetTrait>>>;
}

impl EmitterSystemComponentTrait for EmitterSystemComponent {
    fn exclusion_volumes(&self) -> &Option<Arc<Mutex<dyn super::emitter_base::EmitterExclusionVolumesBaseAssetTrait>>> {
        &self.exclusion_volumes
    }
    fn exclusion_volumes_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::emitter_base::EmitterExclusionVolumesBaseAssetTrait>>> {
        &mut self.exclusion_volumes
    }
}

impl super::entity::SubWorldDataComponentTrait for EmitterSystemComponent {
}

impl super::core::DataContainerTrait for EmitterSystemComponent {
}

pub static EMITTERSYSTEMCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterSystemComponent",
    flags: MemberInfoFlags::new(101),
    module: "Effect",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SUBWORLDDATACOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EmitterSystemComponent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ExclusionVolumes",
                flags: MemberInfoFlags::new(0),
                field_type: "EmitterExclusionVolumesBaseAsset",
                rust_offset: offset_of!(EmitterSystemComponent, exclusion_volumes),
            },
        ],
    }),
    array_type: Some(EMITTERSYSTEMCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EmitterSystemComponent {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERSYSTEMCOMPONENT_TYPE_INFO
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


pub static EMITTERSYSTEMCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterSystemComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Effect",
    data: TypeInfoData::Array("EmitterSystemComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EmitterExclusionVolumeData {
    pub _glacier_base: super::entity::OBBData,
    pub id: u32,
}

pub trait EmitterExclusionVolumeDataTrait: super::entity::OBBDataTrait {
    fn id(&self) -> &u32;
    fn id_mut(&mut self) -> &mut u32;
}

impl EmitterExclusionVolumeDataTrait for EmitterExclusionVolumeData {
    fn id(&self) -> &u32 {
        &self.id
    }
    fn id_mut(&mut self) -> &mut u32 {
        &mut self.id
    }
}

impl super::entity::OBBDataTrait for EmitterExclusionVolumeData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
    fn half_extents(&self) -> &super::core::Vec3 {
        self._glacier_base.half_extents()
    }
    fn half_extents_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.half_extents_mut()
    }
}

impl super::entity::BaseShapeDataTrait for EmitterExclusionVolumeData {
}

impl super::entity::BaseShapeDataBaseTrait for EmitterExclusionVolumeData {
}

impl super::entity::GameObjectDataTrait for EmitterExclusionVolumeData {
}

impl super::core::DataBusPeerTrait for EmitterExclusionVolumeData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for EmitterExclusionVolumeData {
}

impl super::core::DataContainerTrait for EmitterExclusionVolumeData {
}

pub static EMITTEREXCLUSIONVOLUMEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterExclusionVolumeData",
    flags: MemberInfoFlags::new(101),
    module: "Effect",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::OBBDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EmitterExclusionVolumeData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Id",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EmitterExclusionVolumeData, id),
            },
        ],
    }),
    array_type: Some(EMITTEREXCLUSIONVOLUMEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EmitterExclusionVolumeData {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTEREXCLUSIONVOLUMEDATA_TYPE_INFO
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


pub static EMITTEREXCLUSIONVOLUMEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterExclusionVolumeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Effect",
    data: TypeInfoData::Array("EmitterExclusionVolumeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EmitterEntityData {
    pub _glacier_base: EmitterChildEffectEntityData,
    pub emitter: Option<Arc<Mutex<dyn super::emitter_base::EmitterBaseAssetTrait>>>,
    pub property_id_lookup_table: Vec<super::emitter_base::PropertyIdLookup>,
}

pub trait EmitterEntityDataTrait: EmitterChildEffectEntityDataTrait {
    fn emitter(&self) -> &Option<Arc<Mutex<dyn super::emitter_base::EmitterBaseAssetTrait>>>;
    fn emitter_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::emitter_base::EmitterBaseAssetTrait>>>;
    fn property_id_lookup_table(&self) -> &Vec<super::emitter_base::PropertyIdLookup>;
    fn property_id_lookup_table_mut(&mut self) -> &mut Vec<super::emitter_base::PropertyIdLookup>;
}

impl EmitterEntityDataTrait for EmitterEntityData {
    fn emitter(&self) -> &Option<Arc<Mutex<dyn super::emitter_base::EmitterBaseAssetTrait>>> {
        &self.emitter
    }
    fn emitter_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::emitter_base::EmitterBaseAssetTrait>>> {
        &mut self.emitter
    }
    fn property_id_lookup_table(&self) -> &Vec<super::emitter_base::PropertyIdLookup> {
        &self.property_id_lookup_table
    }
    fn property_id_lookup_table_mut(&mut self) -> &mut Vec<super::emitter_base::PropertyIdLookup> {
        &mut self.property_id_lookup_table
    }
}

impl EmitterChildEffectEntityDataTrait for EmitterEntityData {
    fn local_player_only(&self) -> &bool {
        self._glacier_base.local_player_only()
    }
    fn local_player_only_mut(&mut self) -> &mut bool {
        self._glacier_base.local_player_only_mut()
    }
    fn kill_when_distance_culled(&self) -> &bool {
        self._glacier_base.kill_when_distance_culled()
    }
    fn kill_when_distance_culled_mut(&mut self) -> &mut bool {
        self._glacier_base.kill_when_distance_culled_mut()
    }
    fn spawn_outside_view_radius(&self) -> &f32 {
        self._glacier_base.spawn_outside_view_radius()
    }
    fn spawn_outside_view_radius_mut(&mut self) -> &mut f32 {
        self._glacier_base.spawn_outside_view_radius_mut()
    }
    fn spawn_probability_random_type(&self) -> &SpawnProbabilityRandomType {
        self._glacier_base.spawn_probability_random_type()
    }
    fn spawn_probability_random_type_mut(&mut self) -> &mut SpawnProbabilityRandomType {
        self._glacier_base.spawn_probability_random_type_mut()
    }
    fn spawn_probability_range_min(&self) -> &super::core::QualityScalableFloat {
        self._glacier_base.spawn_probability_range_min()
    }
    fn spawn_probability_range_min_mut(&mut self) -> &mut super::core::QualityScalableFloat {
        self._glacier_base.spawn_probability_range_min_mut()
    }
    fn spawn_probability(&self) -> &super::core::QualityScalableFloat {
        self._glacier_base.spawn_probability()
    }
    fn spawn_probability_mut(&mut self) -> &mut super::core::QualityScalableFloat {
        self._glacier_base.spawn_probability_mut()
    }
    fn draw_order_slot(&self) -> &u8 {
        self._glacier_base.draw_order_slot()
    }
    fn draw_order_slot_mut(&mut self) -> &mut u8 {
        self._glacier_base.draw_order_slot_mut()
    }
    fn max_nearby_instance_count(&self) -> &u32 {
        self._glacier_base.max_nearby_instance_count()
    }
    fn max_nearby_instance_count_mut(&mut self) -> &mut u32 {
        self._glacier_base.max_nearby_instance_count_mut()
    }
    fn nearby_radius(&self) -> &f32 {
        self._glacier_base.nearby_radius()
    }
    fn nearby_radius_mut(&mut self) -> &mut f32 {
        self._glacier_base.nearby_radius_mut()
    }
    fn light_probe_sample_method(&self) -> &super::emitter_base::LightProbeSampleMethod {
        self._glacier_base.light_probe_sample_method()
    }
    fn light_probe_sample_method_mut(&mut self) -> &mut super::emitter_base::LightProbeSampleMethod {
        self._glacier_base.light_probe_sample_method_mut()
    }
    fn light_probe_sample_offset_method(&self) -> &super::emitter_base::LightProbeSampleOffsetMethod {
        self._glacier_base.light_probe_sample_offset_method()
    }
    fn light_probe_sample_offset_method_mut(&mut self) -> &mut super::emitter_base::LightProbeSampleOffsetMethod {
        self._glacier_base.light_probe_sample_offset_method_mut()
    }
    fn light_probe_sample_offset(&self) -> &super::core::Vec3 {
        self._glacier_base.light_probe_sample_offset()
    }
    fn light_probe_sample_offset_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.light_probe_sample_offset_mut()
    }
    fn auto_start(&self) -> &bool {
        self._glacier_base.auto_start()
    }
    fn auto_start_mut(&mut self) -> &mut bool {
        self._glacier_base.auto_start_mut()
    }
}

impl super::entity::ChildEffectEntityDataTrait for EmitterEntityData {
    fn components(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components_mut()
    }
    fn start_delay(&self) -> &f32 {
        self._glacier_base.start_delay()
    }
    fn start_delay_mut(&mut self) -> &mut f32 {
        self._glacier_base.start_delay_mut()
    }
    fn attach_to_spawn_surface(&self) -> &bool {
        self._glacier_base.attach_to_spawn_surface()
    }
    fn attach_to_spawn_surface_mut(&mut self) -> &mut bool {
        self._glacier_base.attach_to_spawn_surface_mut()
    }
    fn enable(&self) -> &super::core::QualityScalableBool {
        self._glacier_base.enable()
    }
    fn enable_mut(&mut self) -> &mut super::core::QualityScalableBool {
        self._glacier_base.enable_mut()
    }
    fn override_draw_order(&self) -> &bool {
        self._glacier_base.override_draw_order()
    }
    fn override_draw_order_mut(&mut self) -> &mut bool {
        self._glacier_base.override_draw_order_mut()
    }
    fn keep_alive(&self) -> &bool {
        self._glacier_base.keep_alive()
    }
    fn keep_alive_mut(&mut self) -> &mut bool {
        self._glacier_base.keep_alive_mut()
    }
}

impl super::entity::SpatialEntityDataTrait for EmitterEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::entity::EntityDataTrait for EmitterEntityData {
}

impl super::entity::GameObjectDataTrait for EmitterEntityData {
}

impl super::core::DataBusPeerTrait for EmitterEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for EmitterEntityData {
}

impl super::core::DataContainerTrait for EmitterEntityData {
}

pub static EMITTERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Effect",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EMITTERCHILDEFFECTENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EmitterEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Emitter",
                flags: MemberInfoFlags::new(0),
                field_type: "EmitterBaseAsset",
                rust_offset: offset_of!(EmitterEntityData, emitter),
            },
            FieldInfoData {
                name: "PropertyIdLookupTable",
                flags: MemberInfoFlags::new(144),
                field_type: "PropertyIdLookup-Array",
                rust_offset: offset_of!(EmitterEntityData, property_id_lookup_table),
            },
        ],
    }),
    array_type: Some(EMITTERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EmitterEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERENTITYDATA_TYPE_INFO
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


pub static EMITTERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Effect",
    data: TypeInfoData::Array("EmitterEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EmitterGraphEntityData {
    pub _glacier_base: EmitterChildEffectEntityData,
    pub emitter_graph: Option<Arc<Mutex<dyn super::emitter_base::EmitterGraphBaseAssetTrait>>>,
    pub emitter_graph_overrides: super::effect_base::EmitterGraphOverrides,
    pub emitter_graph_params: Vec<super::effect_base::EmitterExposedInput>,
    pub emitter_graph_vertex_shader_textures: Vec<super::effect_base::EmitterExposedTextureInput>,
    pub property_id_lookup_table: Vec<super::emitter_base::PropertyIdLookup>,
}

pub trait EmitterGraphEntityDataTrait: EmitterChildEffectEntityDataTrait {
    fn emitter_graph(&self) -> &Option<Arc<Mutex<dyn super::emitter_base::EmitterGraphBaseAssetTrait>>>;
    fn emitter_graph_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::emitter_base::EmitterGraphBaseAssetTrait>>>;
    fn emitter_graph_overrides(&self) -> &super::effect_base::EmitterGraphOverrides;
    fn emitter_graph_overrides_mut(&mut self) -> &mut super::effect_base::EmitterGraphOverrides;
    fn emitter_graph_params(&self) -> &Vec<super::effect_base::EmitterExposedInput>;
    fn emitter_graph_params_mut(&mut self) -> &mut Vec<super::effect_base::EmitterExposedInput>;
    fn emitter_graph_vertex_shader_textures(&self) -> &Vec<super::effect_base::EmitterExposedTextureInput>;
    fn emitter_graph_vertex_shader_textures_mut(&mut self) -> &mut Vec<super::effect_base::EmitterExposedTextureInput>;
    fn property_id_lookup_table(&self) -> &Vec<super::emitter_base::PropertyIdLookup>;
    fn property_id_lookup_table_mut(&mut self) -> &mut Vec<super::emitter_base::PropertyIdLookup>;
}

impl EmitterGraphEntityDataTrait for EmitterGraphEntityData {
    fn emitter_graph(&self) -> &Option<Arc<Mutex<dyn super::emitter_base::EmitterGraphBaseAssetTrait>>> {
        &self.emitter_graph
    }
    fn emitter_graph_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::emitter_base::EmitterGraphBaseAssetTrait>>> {
        &mut self.emitter_graph
    }
    fn emitter_graph_overrides(&self) -> &super::effect_base::EmitterGraphOverrides {
        &self.emitter_graph_overrides
    }
    fn emitter_graph_overrides_mut(&mut self) -> &mut super::effect_base::EmitterGraphOverrides {
        &mut self.emitter_graph_overrides
    }
    fn emitter_graph_params(&self) -> &Vec<super::effect_base::EmitterExposedInput> {
        &self.emitter_graph_params
    }
    fn emitter_graph_params_mut(&mut self) -> &mut Vec<super::effect_base::EmitterExposedInput> {
        &mut self.emitter_graph_params
    }
    fn emitter_graph_vertex_shader_textures(&self) -> &Vec<super::effect_base::EmitterExposedTextureInput> {
        &self.emitter_graph_vertex_shader_textures
    }
    fn emitter_graph_vertex_shader_textures_mut(&mut self) -> &mut Vec<super::effect_base::EmitterExposedTextureInput> {
        &mut self.emitter_graph_vertex_shader_textures
    }
    fn property_id_lookup_table(&self) -> &Vec<super::emitter_base::PropertyIdLookup> {
        &self.property_id_lookup_table
    }
    fn property_id_lookup_table_mut(&mut self) -> &mut Vec<super::emitter_base::PropertyIdLookup> {
        &mut self.property_id_lookup_table
    }
}

impl EmitterChildEffectEntityDataTrait for EmitterGraphEntityData {
    fn local_player_only(&self) -> &bool {
        self._glacier_base.local_player_only()
    }
    fn local_player_only_mut(&mut self) -> &mut bool {
        self._glacier_base.local_player_only_mut()
    }
    fn kill_when_distance_culled(&self) -> &bool {
        self._glacier_base.kill_when_distance_culled()
    }
    fn kill_when_distance_culled_mut(&mut self) -> &mut bool {
        self._glacier_base.kill_when_distance_culled_mut()
    }
    fn spawn_outside_view_radius(&self) -> &f32 {
        self._glacier_base.spawn_outside_view_radius()
    }
    fn spawn_outside_view_radius_mut(&mut self) -> &mut f32 {
        self._glacier_base.spawn_outside_view_radius_mut()
    }
    fn spawn_probability_random_type(&self) -> &SpawnProbabilityRandomType {
        self._glacier_base.spawn_probability_random_type()
    }
    fn spawn_probability_random_type_mut(&mut self) -> &mut SpawnProbabilityRandomType {
        self._glacier_base.spawn_probability_random_type_mut()
    }
    fn spawn_probability_range_min(&self) -> &super::core::QualityScalableFloat {
        self._glacier_base.spawn_probability_range_min()
    }
    fn spawn_probability_range_min_mut(&mut self) -> &mut super::core::QualityScalableFloat {
        self._glacier_base.spawn_probability_range_min_mut()
    }
    fn spawn_probability(&self) -> &super::core::QualityScalableFloat {
        self._glacier_base.spawn_probability()
    }
    fn spawn_probability_mut(&mut self) -> &mut super::core::QualityScalableFloat {
        self._glacier_base.spawn_probability_mut()
    }
    fn draw_order_slot(&self) -> &u8 {
        self._glacier_base.draw_order_slot()
    }
    fn draw_order_slot_mut(&mut self) -> &mut u8 {
        self._glacier_base.draw_order_slot_mut()
    }
    fn max_nearby_instance_count(&self) -> &u32 {
        self._glacier_base.max_nearby_instance_count()
    }
    fn max_nearby_instance_count_mut(&mut self) -> &mut u32 {
        self._glacier_base.max_nearby_instance_count_mut()
    }
    fn nearby_radius(&self) -> &f32 {
        self._glacier_base.nearby_radius()
    }
    fn nearby_radius_mut(&mut self) -> &mut f32 {
        self._glacier_base.nearby_radius_mut()
    }
    fn light_probe_sample_method(&self) -> &super::emitter_base::LightProbeSampleMethod {
        self._glacier_base.light_probe_sample_method()
    }
    fn light_probe_sample_method_mut(&mut self) -> &mut super::emitter_base::LightProbeSampleMethod {
        self._glacier_base.light_probe_sample_method_mut()
    }
    fn light_probe_sample_offset_method(&self) -> &super::emitter_base::LightProbeSampleOffsetMethod {
        self._glacier_base.light_probe_sample_offset_method()
    }
    fn light_probe_sample_offset_method_mut(&mut self) -> &mut super::emitter_base::LightProbeSampleOffsetMethod {
        self._glacier_base.light_probe_sample_offset_method_mut()
    }
    fn light_probe_sample_offset(&self) -> &super::core::Vec3 {
        self._glacier_base.light_probe_sample_offset()
    }
    fn light_probe_sample_offset_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.light_probe_sample_offset_mut()
    }
    fn auto_start(&self) -> &bool {
        self._glacier_base.auto_start()
    }
    fn auto_start_mut(&mut self) -> &mut bool {
        self._glacier_base.auto_start_mut()
    }
}

impl super::entity::ChildEffectEntityDataTrait for EmitterGraphEntityData {
    fn components(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components_mut()
    }
    fn start_delay(&self) -> &f32 {
        self._glacier_base.start_delay()
    }
    fn start_delay_mut(&mut self) -> &mut f32 {
        self._glacier_base.start_delay_mut()
    }
    fn attach_to_spawn_surface(&self) -> &bool {
        self._glacier_base.attach_to_spawn_surface()
    }
    fn attach_to_spawn_surface_mut(&mut self) -> &mut bool {
        self._glacier_base.attach_to_spawn_surface_mut()
    }
    fn enable(&self) -> &super::core::QualityScalableBool {
        self._glacier_base.enable()
    }
    fn enable_mut(&mut self) -> &mut super::core::QualityScalableBool {
        self._glacier_base.enable_mut()
    }
    fn override_draw_order(&self) -> &bool {
        self._glacier_base.override_draw_order()
    }
    fn override_draw_order_mut(&mut self) -> &mut bool {
        self._glacier_base.override_draw_order_mut()
    }
    fn keep_alive(&self) -> &bool {
        self._glacier_base.keep_alive()
    }
    fn keep_alive_mut(&mut self) -> &mut bool {
        self._glacier_base.keep_alive_mut()
    }
}

impl super::entity::SpatialEntityDataTrait for EmitterGraphEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::entity::EntityDataTrait for EmitterGraphEntityData {
}

impl super::entity::GameObjectDataTrait for EmitterGraphEntityData {
}

impl super::core::DataBusPeerTrait for EmitterGraphEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for EmitterGraphEntityData {
}

impl super::core::DataContainerTrait for EmitterGraphEntityData {
}

pub static EMITTERGRAPHENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Effect",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EMITTERCHILDEFFECTENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EmitterGraphEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "EmitterGraph",
                flags: MemberInfoFlags::new(0),
                field_type: "EmitterGraphBaseAsset",
                rust_offset: offset_of!(EmitterGraphEntityData, emitter_graph),
            },
            FieldInfoData {
                name: "EmitterGraphOverrides",
                flags: MemberInfoFlags::new(0),
                field_type: "EmitterGraphOverrides",
                rust_offset: offset_of!(EmitterGraphEntityData, emitter_graph_overrides),
            },
            FieldInfoData {
                name: "EmitterGraphParams",
                flags: MemberInfoFlags::new(144),
                field_type: "EmitterExposedInput-Array",
                rust_offset: offset_of!(EmitterGraphEntityData, emitter_graph_params),
            },
            FieldInfoData {
                name: "EmitterGraphVertexShaderTextures",
                flags: MemberInfoFlags::new(144),
                field_type: "EmitterExposedTextureInput-Array",
                rust_offset: offset_of!(EmitterGraphEntityData, emitter_graph_vertex_shader_textures),
            },
            FieldInfoData {
                name: "PropertyIdLookupTable",
                flags: MemberInfoFlags::new(144),
                field_type: "PropertyIdLookup-Array",
                rust_offset: offset_of!(EmitterGraphEntityData, property_id_lookup_table),
            },
        ],
    }),
    array_type: Some(EMITTERGRAPHENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EmitterGraphEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERGRAPHENTITYDATA_TYPE_INFO
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


pub static EMITTERGRAPHENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Effect",
    data: TypeInfoData::Array("EmitterGraphEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EmitterChildEffectEntityData {
    pub _glacier_base: super::entity::ChildEffectEntityData,
    pub local_player_only: bool,
    pub kill_when_distance_culled: bool,
    pub spawn_outside_view_radius: f32,
    pub spawn_probability_random_type: SpawnProbabilityRandomType,
    pub spawn_probability_range_min: super::core::QualityScalableFloat,
    pub spawn_probability: super::core::QualityScalableFloat,
    pub draw_order_slot: u8,
    pub max_nearby_instance_count: u32,
    pub nearby_radius: f32,
    pub light_probe_sample_method: super::emitter_base::LightProbeSampleMethod,
    pub light_probe_sample_offset_method: super::emitter_base::LightProbeSampleOffsetMethod,
    pub light_probe_sample_offset: super::core::Vec3,
    pub auto_start: bool,
}

pub trait EmitterChildEffectEntityDataTrait: super::entity::ChildEffectEntityDataTrait {
    fn local_player_only(&self) -> &bool;
    fn local_player_only_mut(&mut self) -> &mut bool;
    fn kill_when_distance_culled(&self) -> &bool;
    fn kill_when_distance_culled_mut(&mut self) -> &mut bool;
    fn spawn_outside_view_radius(&self) -> &f32;
    fn spawn_outside_view_radius_mut(&mut self) -> &mut f32;
    fn spawn_probability_random_type(&self) -> &SpawnProbabilityRandomType;
    fn spawn_probability_random_type_mut(&mut self) -> &mut SpawnProbabilityRandomType;
    fn spawn_probability_range_min(&self) -> &super::core::QualityScalableFloat;
    fn spawn_probability_range_min_mut(&mut self) -> &mut super::core::QualityScalableFloat;
    fn spawn_probability(&self) -> &super::core::QualityScalableFloat;
    fn spawn_probability_mut(&mut self) -> &mut super::core::QualityScalableFloat;
    fn draw_order_slot(&self) -> &u8;
    fn draw_order_slot_mut(&mut self) -> &mut u8;
    fn max_nearby_instance_count(&self) -> &u32;
    fn max_nearby_instance_count_mut(&mut self) -> &mut u32;
    fn nearby_radius(&self) -> &f32;
    fn nearby_radius_mut(&mut self) -> &mut f32;
    fn light_probe_sample_method(&self) -> &super::emitter_base::LightProbeSampleMethod;
    fn light_probe_sample_method_mut(&mut self) -> &mut super::emitter_base::LightProbeSampleMethod;
    fn light_probe_sample_offset_method(&self) -> &super::emitter_base::LightProbeSampleOffsetMethod;
    fn light_probe_sample_offset_method_mut(&mut self) -> &mut super::emitter_base::LightProbeSampleOffsetMethod;
    fn light_probe_sample_offset(&self) -> &super::core::Vec3;
    fn light_probe_sample_offset_mut(&mut self) -> &mut super::core::Vec3;
    fn auto_start(&self) -> &bool;
    fn auto_start_mut(&mut self) -> &mut bool;
}

impl EmitterChildEffectEntityDataTrait for EmitterChildEffectEntityData {
    fn local_player_only(&self) -> &bool {
        &self.local_player_only
    }
    fn local_player_only_mut(&mut self) -> &mut bool {
        &mut self.local_player_only
    }
    fn kill_when_distance_culled(&self) -> &bool {
        &self.kill_when_distance_culled
    }
    fn kill_when_distance_culled_mut(&mut self) -> &mut bool {
        &mut self.kill_when_distance_culled
    }
    fn spawn_outside_view_radius(&self) -> &f32 {
        &self.spawn_outside_view_radius
    }
    fn spawn_outside_view_radius_mut(&mut self) -> &mut f32 {
        &mut self.spawn_outside_view_radius
    }
    fn spawn_probability_random_type(&self) -> &SpawnProbabilityRandomType {
        &self.spawn_probability_random_type
    }
    fn spawn_probability_random_type_mut(&mut self) -> &mut SpawnProbabilityRandomType {
        &mut self.spawn_probability_random_type
    }
    fn spawn_probability_range_min(&self) -> &super::core::QualityScalableFloat {
        &self.spawn_probability_range_min
    }
    fn spawn_probability_range_min_mut(&mut self) -> &mut super::core::QualityScalableFloat {
        &mut self.spawn_probability_range_min
    }
    fn spawn_probability(&self) -> &super::core::QualityScalableFloat {
        &self.spawn_probability
    }
    fn spawn_probability_mut(&mut self) -> &mut super::core::QualityScalableFloat {
        &mut self.spawn_probability
    }
    fn draw_order_slot(&self) -> &u8 {
        &self.draw_order_slot
    }
    fn draw_order_slot_mut(&mut self) -> &mut u8 {
        &mut self.draw_order_slot
    }
    fn max_nearby_instance_count(&self) -> &u32 {
        &self.max_nearby_instance_count
    }
    fn max_nearby_instance_count_mut(&mut self) -> &mut u32 {
        &mut self.max_nearby_instance_count
    }
    fn nearby_radius(&self) -> &f32 {
        &self.nearby_radius
    }
    fn nearby_radius_mut(&mut self) -> &mut f32 {
        &mut self.nearby_radius
    }
    fn light_probe_sample_method(&self) -> &super::emitter_base::LightProbeSampleMethod {
        &self.light_probe_sample_method
    }
    fn light_probe_sample_method_mut(&mut self) -> &mut super::emitter_base::LightProbeSampleMethod {
        &mut self.light_probe_sample_method
    }
    fn light_probe_sample_offset_method(&self) -> &super::emitter_base::LightProbeSampleOffsetMethod {
        &self.light_probe_sample_offset_method
    }
    fn light_probe_sample_offset_method_mut(&mut self) -> &mut super::emitter_base::LightProbeSampleOffsetMethod {
        &mut self.light_probe_sample_offset_method
    }
    fn light_probe_sample_offset(&self) -> &super::core::Vec3 {
        &self.light_probe_sample_offset
    }
    fn light_probe_sample_offset_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.light_probe_sample_offset
    }
    fn auto_start(&self) -> &bool {
        &self.auto_start
    }
    fn auto_start_mut(&mut self) -> &mut bool {
        &mut self.auto_start
    }
}

impl super::entity::ChildEffectEntityDataTrait for EmitterChildEffectEntityData {
    fn components(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components_mut()
    }
    fn start_delay(&self) -> &f32 {
        self._glacier_base.start_delay()
    }
    fn start_delay_mut(&mut self) -> &mut f32 {
        self._glacier_base.start_delay_mut()
    }
    fn attach_to_spawn_surface(&self) -> &bool {
        self._glacier_base.attach_to_spawn_surface()
    }
    fn attach_to_spawn_surface_mut(&mut self) -> &mut bool {
        self._glacier_base.attach_to_spawn_surface_mut()
    }
    fn enable(&self) -> &super::core::QualityScalableBool {
        self._glacier_base.enable()
    }
    fn enable_mut(&mut self) -> &mut super::core::QualityScalableBool {
        self._glacier_base.enable_mut()
    }
    fn override_draw_order(&self) -> &bool {
        self._glacier_base.override_draw_order()
    }
    fn override_draw_order_mut(&mut self) -> &mut bool {
        self._glacier_base.override_draw_order_mut()
    }
    fn keep_alive(&self) -> &bool {
        self._glacier_base.keep_alive()
    }
    fn keep_alive_mut(&mut self) -> &mut bool {
        self._glacier_base.keep_alive_mut()
    }
}

impl super::entity::SpatialEntityDataTrait for EmitterChildEffectEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::entity::EntityDataTrait for EmitterChildEffectEntityData {
}

impl super::entity::GameObjectDataTrait for EmitterChildEffectEntityData {
}

impl super::core::DataBusPeerTrait for EmitterChildEffectEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for EmitterChildEffectEntityData {
}

impl super::core::DataContainerTrait for EmitterChildEffectEntityData {
}

pub static EMITTERCHILDEFFECTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterChildEffectEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Effect",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::CHILDEFFECTENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EmitterChildEffectEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "LocalPlayerOnly",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterChildEffectEntityData, local_player_only),
            },
            FieldInfoData {
                name: "KillWhenDistanceCulled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterChildEffectEntityData, kill_when_distance_culled),
            },
            FieldInfoData {
                name: "SpawnOutsideViewRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterChildEffectEntityData, spawn_outside_view_radius),
            },
            FieldInfoData {
                name: "SpawnProbabilityRandomType",
                flags: MemberInfoFlags::new(0),
                field_type: "SpawnProbabilityRandomType",
                rust_offset: offset_of!(EmitterChildEffectEntityData, spawn_probability_random_type),
            },
            FieldInfoData {
                name: "SpawnProbabilityRangeMin",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(EmitterChildEffectEntityData, spawn_probability_range_min),
            },
            FieldInfoData {
                name: "SpawnProbability",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(EmitterChildEffectEntityData, spawn_probability),
            },
            FieldInfoData {
                name: "DrawOrderSlot",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(EmitterChildEffectEntityData, draw_order_slot),
            },
            FieldInfoData {
                name: "MaxNearbyInstanceCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EmitterChildEffectEntityData, max_nearby_instance_count),
            },
            FieldInfoData {
                name: "NearbyRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterChildEffectEntityData, nearby_radius),
            },
            FieldInfoData {
                name: "LightProbeSampleMethod",
                flags: MemberInfoFlags::new(0),
                field_type: "LightProbeSampleMethod",
                rust_offset: offset_of!(EmitterChildEffectEntityData, light_probe_sample_method),
            },
            FieldInfoData {
                name: "LightProbeSampleOffsetMethod",
                flags: MemberInfoFlags::new(0),
                field_type: "LightProbeSampleOffsetMethod",
                rust_offset: offset_of!(EmitterChildEffectEntityData, light_probe_sample_offset_method),
            },
            FieldInfoData {
                name: "LightProbeSampleOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(EmitterChildEffectEntityData, light_probe_sample_offset),
            },
            FieldInfoData {
                name: "AutoStart",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterChildEffectEntityData, auto_start),
            },
        ],
    }),
    array_type: Some(EMITTERCHILDEFFECTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EmitterChildEffectEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERCHILDEFFECTENTITYDATA_TYPE_INFO
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


pub static EMITTERCHILDEFFECTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterChildEffectEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Effect",
    data: TypeInfoData::Array("EmitterChildEffectEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum SpawnProbabilityRandomType {
    #[default]
    SpawnProbabilityRandomType_PerEmitter = 0,
    SpawnProbabilityRandomType_PerEffect = 1,
}

pub static SPAWNPROBABILITYRANDOMTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnProbabilityRandomType",
    flags: MemberInfoFlags::new(49429),
    module: "Effect",
    data: TypeInfoData::Enum,
    array_type: Some(SPAWNPROBABILITYRANDOMTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SpawnProbabilityRandomType {
    fn type_info(&self) -> &'static TypeInfo {
        SPAWNPROBABILITYRANDOMTYPE_TYPE_INFO
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


pub static SPAWNPROBABILITYRANDOMTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnProbabilityRandomType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Effect",
    data: TypeInfoData::Array("SpawnProbabilityRandomType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EffectSystemSettings {
    pub _glacier_base: super::core::DataContainer,
    pub effect_quality_level: super::core::QualityLevel,
    pub enter_level_disable_effect_time: f32,
    pub emitter_root_view_duplication_enable: bool,
}

pub trait EffectSystemSettingsTrait: super::core::DataContainerTrait {
    fn effect_quality_level(&self) -> &super::core::QualityLevel;
    fn effect_quality_level_mut(&mut self) -> &mut super::core::QualityLevel;
    fn enter_level_disable_effect_time(&self) -> &f32;
    fn enter_level_disable_effect_time_mut(&mut self) -> &mut f32;
    fn emitter_root_view_duplication_enable(&self) -> &bool;
    fn emitter_root_view_duplication_enable_mut(&mut self) -> &mut bool;
}

impl EffectSystemSettingsTrait for EffectSystemSettings {
    fn effect_quality_level(&self) -> &super::core::QualityLevel {
        &self.effect_quality_level
    }
    fn effect_quality_level_mut(&mut self) -> &mut super::core::QualityLevel {
        &mut self.effect_quality_level
    }
    fn enter_level_disable_effect_time(&self) -> &f32 {
        &self.enter_level_disable_effect_time
    }
    fn enter_level_disable_effect_time_mut(&mut self) -> &mut f32 {
        &mut self.enter_level_disable_effect_time
    }
    fn emitter_root_view_duplication_enable(&self) -> &bool {
        &self.emitter_root_view_duplication_enable
    }
    fn emitter_root_view_duplication_enable_mut(&mut self) -> &mut bool {
        &mut self.emitter_root_view_duplication_enable
    }
}

impl super::core::DataContainerTrait for EffectSystemSettings {
}

pub static EFFECTSYSTEMSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectSystemSettings",
    flags: MemberInfoFlags::new(101),
    module: "Effect",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EffectSystemSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "EffectQualityLevel",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityLevel",
                rust_offset: offset_of!(EffectSystemSettings, effect_quality_level),
            },
            FieldInfoData {
                name: "EnterLevelDisableEffectTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EffectSystemSettings, enter_level_disable_effect_time),
            },
            FieldInfoData {
                name: "EmitterRootViewDuplicationEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EffectSystemSettings, emitter_root_view_duplication_enable),
            },
        ],
    }),
    array_type: Some(EFFECTSYSTEMSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EffectSystemSettings {
    fn type_info(&self) -> &'static TypeInfo {
        EFFECTSYSTEMSETTINGS_TYPE_INFO
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


pub static EFFECTSYSTEMSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectSystemSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Effect",
    data: TypeInfoData::Array("EffectSystemSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EffectSystemComponent {
    pub _glacier_base: super::entity::SubWorldDataComponent,
    pub effect_parameter_list: Option<Arc<Mutex<dyn super::effect_base::EffectParameterListTrait>>>,
}

pub trait EffectSystemComponentTrait: super::entity::SubWorldDataComponentTrait {
    fn effect_parameter_list(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterListTrait>>>;
    fn effect_parameter_list_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterListTrait>>>;
}

impl EffectSystemComponentTrait for EffectSystemComponent {
    fn effect_parameter_list(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterListTrait>>> {
        &self.effect_parameter_list
    }
    fn effect_parameter_list_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterListTrait>>> {
        &mut self.effect_parameter_list
    }
}

impl super::entity::SubWorldDataComponentTrait for EffectSystemComponent {
}

impl super::core::DataContainerTrait for EffectSystemComponent {
}

pub static EFFECTSYSTEMCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectSystemComponent",
    flags: MemberInfoFlags::new(101),
    module: "Effect",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SUBWORLDDATACOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EffectSystemComponent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "EffectParameterList",
                flags: MemberInfoFlags::new(0),
                field_type: "EffectParameterList",
                rust_offset: offset_of!(EffectSystemComponent, effect_parameter_list),
            },
        ],
    }),
    array_type: Some(EFFECTSYSTEMCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EffectSystemComponent {
    fn type_info(&self) -> &'static TypeInfo {
        EFFECTSYSTEMCOMPONENT_TYPE_INFO
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


pub static EFFECTSYSTEMCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectSystemComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Effect",
    data: TypeInfoData::Array("EffectSystemComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BlueprintEffectEntityData {
    pub _glacier_base: super::entity::ChildEffectEntityData,
    pub blueprint: Option<Arc<Mutex<dyn super::entity::BlueprintTrait>>>,
    pub lifetime: f32,
    pub lifetime_curve: super::core::Vec4,
    pub dimmer: f32,
    pub spawn_probability: super::core::QualityScalableFloat,
    pub local_player_only: bool,
}

pub trait BlueprintEffectEntityDataTrait: super::entity::ChildEffectEntityDataTrait {
    fn blueprint(&self) -> &Option<Arc<Mutex<dyn super::entity::BlueprintTrait>>>;
    fn blueprint_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::entity::BlueprintTrait>>>;
    fn lifetime(&self) -> &f32;
    fn lifetime_mut(&mut self) -> &mut f32;
    fn lifetime_curve(&self) -> &super::core::Vec4;
    fn lifetime_curve_mut(&mut self) -> &mut super::core::Vec4;
    fn dimmer(&self) -> &f32;
    fn dimmer_mut(&mut self) -> &mut f32;
    fn spawn_probability(&self) -> &super::core::QualityScalableFloat;
    fn spawn_probability_mut(&mut self) -> &mut super::core::QualityScalableFloat;
    fn local_player_only(&self) -> &bool;
    fn local_player_only_mut(&mut self) -> &mut bool;
}

impl BlueprintEffectEntityDataTrait for BlueprintEffectEntityData {
    fn blueprint(&self) -> &Option<Arc<Mutex<dyn super::entity::BlueprintTrait>>> {
        &self.blueprint
    }
    fn blueprint_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::entity::BlueprintTrait>>> {
        &mut self.blueprint
    }
    fn lifetime(&self) -> &f32 {
        &self.lifetime
    }
    fn lifetime_mut(&mut self) -> &mut f32 {
        &mut self.lifetime
    }
    fn lifetime_curve(&self) -> &super::core::Vec4 {
        &self.lifetime_curve
    }
    fn lifetime_curve_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.lifetime_curve
    }
    fn dimmer(&self) -> &f32 {
        &self.dimmer
    }
    fn dimmer_mut(&mut self) -> &mut f32 {
        &mut self.dimmer
    }
    fn spawn_probability(&self) -> &super::core::QualityScalableFloat {
        &self.spawn_probability
    }
    fn spawn_probability_mut(&mut self) -> &mut super::core::QualityScalableFloat {
        &mut self.spawn_probability
    }
    fn local_player_only(&self) -> &bool {
        &self.local_player_only
    }
    fn local_player_only_mut(&mut self) -> &mut bool {
        &mut self.local_player_only
    }
}

impl super::entity::ChildEffectEntityDataTrait for BlueprintEffectEntityData {
    fn components(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components_mut()
    }
    fn start_delay(&self) -> &f32 {
        self._glacier_base.start_delay()
    }
    fn start_delay_mut(&mut self) -> &mut f32 {
        self._glacier_base.start_delay_mut()
    }
    fn attach_to_spawn_surface(&self) -> &bool {
        self._glacier_base.attach_to_spawn_surface()
    }
    fn attach_to_spawn_surface_mut(&mut self) -> &mut bool {
        self._glacier_base.attach_to_spawn_surface_mut()
    }
    fn enable(&self) -> &super::core::QualityScalableBool {
        self._glacier_base.enable()
    }
    fn enable_mut(&mut self) -> &mut super::core::QualityScalableBool {
        self._glacier_base.enable_mut()
    }
    fn override_draw_order(&self) -> &bool {
        self._glacier_base.override_draw_order()
    }
    fn override_draw_order_mut(&mut self) -> &mut bool {
        self._glacier_base.override_draw_order_mut()
    }
    fn keep_alive(&self) -> &bool {
        self._glacier_base.keep_alive()
    }
    fn keep_alive_mut(&mut self) -> &mut bool {
        self._glacier_base.keep_alive_mut()
    }
}

impl super::entity::SpatialEntityDataTrait for BlueprintEffectEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::entity::EntityDataTrait for BlueprintEffectEntityData {
}

impl super::entity::GameObjectDataTrait for BlueprintEffectEntityData {
}

impl super::core::DataBusPeerTrait for BlueprintEffectEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for BlueprintEffectEntityData {
}

impl super::core::DataContainerTrait for BlueprintEffectEntityData {
}

pub static BLUEPRINTEFFECTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintEffectEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Effect",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::CHILDEFFECTENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BlueprintEffectEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Blueprint",
                flags: MemberInfoFlags::new(0),
                field_type: "Blueprint",
                rust_offset: offset_of!(BlueprintEffectEntityData, blueprint),
            },
            FieldInfoData {
                name: "Lifetime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BlueprintEffectEntityData, lifetime),
            },
            FieldInfoData {
                name: "LifetimeCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(BlueprintEffectEntityData, lifetime_curve),
            },
            FieldInfoData {
                name: "Dimmer",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BlueprintEffectEntityData, dimmer),
            },
            FieldInfoData {
                name: "SpawnProbability",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(BlueprintEffectEntityData, spawn_probability),
            },
            FieldInfoData {
                name: "LocalPlayerOnly",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(BlueprintEffectEntityData, local_player_only),
            },
        ],
    }),
    array_type: Some(BLUEPRINTEFFECTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for BlueprintEffectEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        BLUEPRINTEFFECTENTITYDATA_TYPE_INFO
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


pub static BLUEPRINTEFFECTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintEffectEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Effect",
    data: TypeInfoData::Array("BlueprintEffectEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EffectAsset {
    pub _glacier_base: super::effect_base::EffectBlueprint,
}

pub trait EffectAssetTrait: super::effect_base::EffectBlueprintTrait {
}

impl EffectAssetTrait for EffectAsset {
}

impl super::effect_base::EffectBlueprintTrait for EffectAsset {
    fn time_delta_type(&self) -> &super::entity::TimeDeltaType {
        self._glacier_base.time_delta_type()
    }
    fn time_delta_type_mut(&mut self) -> &mut super::entity::TimeDeltaType {
        self._glacier_base.time_delta_type_mut()
    }
    fn is_simple(&self) -> &bool {
        self._glacier_base.is_simple()
    }
    fn is_simple_mut(&mut self) -> &mut bool {
        self._glacier_base.is_simple_mut()
    }
}

impl super::entity::ObjectBlueprintTrait for EffectAsset {
    fn object(&self) -> &Option<Arc<Mutex<dyn super::entity::EntityDataTrait>>> {
        self._glacier_base.object()
    }
    fn object_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::entity::EntityDataTrait>>> {
        self._glacier_base.object_mut()
    }
}

impl super::entity::BlueprintTrait for EffectAsset {
    fn objects(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.objects()
    }
    fn objects_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.objects_mut()
    }
    fn schematics(&self) -> &Option<Arc<Mutex<dyn super::schematics::SchematicsBaseAssetTrait>>> {
        self._glacier_base.schematics()
    }
    fn schematics_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::schematics::SchematicsBaseAssetTrait>>> {
        self._glacier_base.schematics_mut()
    }
}

impl super::entity::EntityBusDataTrait for EffectAsset {
    fn event_connections(&self) -> &Vec<super::entity::EventConnection> {
        self._glacier_base.event_connections()
    }
    fn event_connections_mut(&mut self) -> &mut Vec<super::entity::EventConnection> {
        self._glacier_base.event_connections_mut()
    }
}

impl super::core::DataBusDataTrait for EffectAsset {
    fn flags(&self) -> &u16 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.flags_mut()
    }
    fn property_connections(&self) -> &Vec<super::core::PropertyConnection> {
        self._glacier_base.property_connections()
    }
    fn property_connections_mut(&mut self) -> &mut Vec<super::core::PropertyConnection> {
        self._glacier_base.property_connections_mut()
    }
    fn link_connections(&self) -> &Vec<super::core::LinkConnection> {
        self._glacier_base.link_connections()
    }
    fn link_connections_mut(&mut self) -> &mut Vec<super::core::LinkConnection> {
        self._glacier_base.link_connections_mut()
    }
    fn interface(&self) -> &Option<Arc<Mutex<dyn super::core::DynamicDataContainerTrait>>> {
        self._glacier_base.interface()
    }
    fn interface_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::core::DynamicDataContainerTrait>>> {
        self._glacier_base.interface_mut()
    }
}

impl super::core::AssetTrait for EffectAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for EffectAsset {
}

pub static EFFECTASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectAsset",
    flags: MemberInfoFlags::new(101),
    module: "Effect",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::effect_base::EFFECTBLUEPRINT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EffectAsset as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(EFFECTASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EffectAsset {
    fn type_info(&self) -> &'static TypeInfo {
        EFFECTASSET_TYPE_INFO
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


pub static EFFECTASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Effect",
    data: TypeInfoData::Array("EffectAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EffectEntityData {
    pub _glacier_base: super::entity::SpatialEntityData,
    pub components: Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>>,
    pub max_active_instance_count: super::core::QualityScalableInt,
    pub cull_distance: super::core::QualityScalableFloat,
    pub kill_by_water: bool,
    pub start_delay: f32,
    pub kill_on_max_count: bool,
    pub attach_to_spawn_surface: bool,
    pub enable: super::core::QualityScalableBool,
    pub override_draw_order: bool,
    pub keep_alive: bool,
}

pub trait EffectEntityDataTrait: super::entity::SpatialEntityDataTrait {
    fn components(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>>;
    fn components_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>>;
    fn max_active_instance_count(&self) -> &super::core::QualityScalableInt;
    fn max_active_instance_count_mut(&mut self) -> &mut super::core::QualityScalableInt;
    fn cull_distance(&self) -> &super::core::QualityScalableFloat;
    fn cull_distance_mut(&mut self) -> &mut super::core::QualityScalableFloat;
    fn kill_by_water(&self) -> &bool;
    fn kill_by_water_mut(&mut self) -> &mut bool;
    fn start_delay(&self) -> &f32;
    fn start_delay_mut(&mut self) -> &mut f32;
    fn kill_on_max_count(&self) -> &bool;
    fn kill_on_max_count_mut(&mut self) -> &mut bool;
    fn attach_to_spawn_surface(&self) -> &bool;
    fn attach_to_spawn_surface_mut(&mut self) -> &mut bool;
    fn enable(&self) -> &super::core::QualityScalableBool;
    fn enable_mut(&mut self) -> &mut super::core::QualityScalableBool;
    fn override_draw_order(&self) -> &bool;
    fn override_draw_order_mut(&mut self) -> &mut bool;
    fn keep_alive(&self) -> &bool;
    fn keep_alive_mut(&mut self) -> &mut bool;
}

impl EffectEntityDataTrait for EffectEntityData {
    fn components(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        &self.components
    }
    fn components_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        &mut self.components
    }
    fn max_active_instance_count(&self) -> &super::core::QualityScalableInt {
        &self.max_active_instance_count
    }
    fn max_active_instance_count_mut(&mut self) -> &mut super::core::QualityScalableInt {
        &mut self.max_active_instance_count
    }
    fn cull_distance(&self) -> &super::core::QualityScalableFloat {
        &self.cull_distance
    }
    fn cull_distance_mut(&mut self) -> &mut super::core::QualityScalableFloat {
        &mut self.cull_distance
    }
    fn kill_by_water(&self) -> &bool {
        &self.kill_by_water
    }
    fn kill_by_water_mut(&mut self) -> &mut bool {
        &mut self.kill_by_water
    }
    fn start_delay(&self) -> &f32 {
        &self.start_delay
    }
    fn start_delay_mut(&mut self) -> &mut f32 {
        &mut self.start_delay
    }
    fn kill_on_max_count(&self) -> &bool {
        &self.kill_on_max_count
    }
    fn kill_on_max_count_mut(&mut self) -> &mut bool {
        &mut self.kill_on_max_count
    }
    fn attach_to_spawn_surface(&self) -> &bool {
        &self.attach_to_spawn_surface
    }
    fn attach_to_spawn_surface_mut(&mut self) -> &mut bool {
        &mut self.attach_to_spawn_surface
    }
    fn enable(&self) -> &super::core::QualityScalableBool {
        &self.enable
    }
    fn enable_mut(&mut self) -> &mut super::core::QualityScalableBool {
        &mut self.enable
    }
    fn override_draw_order(&self) -> &bool {
        &self.override_draw_order
    }
    fn override_draw_order_mut(&mut self) -> &mut bool {
        &mut self.override_draw_order
    }
    fn keep_alive(&self) -> &bool {
        &self.keep_alive
    }
    fn keep_alive_mut(&mut self) -> &mut bool {
        &mut self.keep_alive
    }
}

impl super::entity::SpatialEntityDataTrait for EffectEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::entity::EntityDataTrait for EffectEntityData {
}

impl super::entity::GameObjectDataTrait for EffectEntityData {
}

impl super::core::DataBusPeerTrait for EffectEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for EffectEntityData {
}

impl super::core::DataContainerTrait for EffectEntityData {
}

pub static EFFECTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Effect",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EffectEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Components",
                flags: MemberInfoFlags::new(144),
                field_type: "GameObjectData-Array",
                rust_offset: offset_of!(EffectEntityData, components),
            },
            FieldInfoData {
                name: "MaxActiveInstanceCount",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableInt",
                rust_offset: offset_of!(EffectEntityData, max_active_instance_count),
            },
            FieldInfoData {
                name: "CullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(EffectEntityData, cull_distance),
            },
            FieldInfoData {
                name: "KillByWater",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EffectEntityData, kill_by_water),
            },
            FieldInfoData {
                name: "StartDelay",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EffectEntityData, start_delay),
            },
            FieldInfoData {
                name: "KillOnMaxCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EffectEntityData, kill_on_max_count),
            },
            FieldInfoData {
                name: "AttachToSpawnSurface",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EffectEntityData, attach_to_spawn_surface),
            },
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableBool",
                rust_offset: offset_of!(EffectEntityData, enable),
            },
            FieldInfoData {
                name: "OverrideDrawOrder",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EffectEntityData, override_draw_order),
            },
            FieldInfoData {
                name: "KeepAlive",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EffectEntityData, keep_alive),
            },
        ],
    }),
    array_type: Some(EFFECTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EffectEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        EFFECTENTITYDATA_TYPE_INFO
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


pub static EFFECTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Effect",
    data: TypeInfoData::Array("EffectEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EffectEntity {
    pub _glacier_base: super::entity::SpatialEntity,
}

pub trait EffectEntityTrait: super::entity::SpatialEntityTrait {
}

impl EffectEntityTrait for EffectEntity {
}

impl super::entity::SpatialEntityTrait for EffectEntity {
}

impl super::entity::EntityTrait for EffectEntity {
}

impl super::entity::EntityBusPeerTrait for EffectEntity {
}

pub static EFFECTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectEntity",
    flags: MemberInfoFlags::new(101),
    module: "Effect",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EffectEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(EFFECTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EffectEntity {
    fn type_info(&self) -> &'static TypeInfo {
        EFFECTENTITY_TYPE_INFO
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


pub static EFFECTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Effect",
    data: TypeInfoData::Array("EffectEntity"),
    array_type: None,
    alignment: 8,
};


