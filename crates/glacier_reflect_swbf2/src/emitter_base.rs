use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_emitter_base_types(registry: &mut TypeRegistry) {
    registry.register_type(EMITTERHANDLE_TYPE_INFO);
    registry.register_type(EMITTERHANDLE_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERCREATESTATE_TYPE_INFO);
    registry.register_type(EMITTERCREATESTATE_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERDYNAMICSTATE_TYPE_INFO);
    registry.register_type(EMITTERDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERCONTROLPOINT_TYPE_INFO);
    registry.register_type(EMITTERCONTROLPOINT_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERSTATE_TYPE_INFO);
    registry.register_type(EMITTERSTATE_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERSTATICSTATE_TYPE_INFO);
    registry.register_type(EMITTERSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(EMITTEREXCLUSIONVOLUMESBASEASSET_TYPE_INFO);
    registry.register_type(EMITTEREXCLUSIONVOLUMESBASEASSET_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERTAG_TYPE_INFO);
    registry.register_type(EMITTERTAG_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERGRAPHBASEASSET_TYPE_INFO);
    registry.register_type(EMITTERGRAPHBASEASSET_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERBASEASSET_TYPE_INFO);
    registry.register_type(EMITTERBASEASSET_ARRAY_TYPE_INFO);
    registry.register_type(PROPERTYIDLOOKUP_TYPE_INFO);
    registry.register_type(PROPERTYIDLOOKUP_ARRAY_TYPE_INFO);
    registry.register_type(EMITTEREXPOSABLETYPE_TYPE_INFO);
    registry.register_type(EMITTEREXPOSABLETYPE_ARRAY_TYPE_INFO);
    registry.register_type(EMITTEREXCLUSIONVOLUMERESULTDATA_TYPE_INFO);
    registry.register_type(EMITTEREXCLUSIONVOLUMERESULTDATA_ARRAY_TYPE_INFO);
    registry.register_type(LIGHTPROBESAMPLEOFFSETMETHOD_TYPE_INFO);
    registry.register_type(LIGHTPROBESAMPLEOFFSETMETHOD_ARRAY_TYPE_INFO);
    registry.register_type(LIGHTPROBESAMPLEMETHOD_TYPE_INFO);
    registry.register_type(LIGHTPROBESAMPLEMETHOD_ARRAY_TYPE_INFO);
}

#[derive(Clone, Debug, Default)]
pub struct EmitterHandle {
}

pub trait EmitterHandleTrait: TypeObject {
}

impl EmitterHandleTrait for EmitterHandle {
}

pub static EMITTERHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterHandle",
    flags: MemberInfoFlags::new(73),
    module: "EmitterBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EmitterHandle as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(EMITTERHANDLE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for EmitterHandle {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERHANDLE_TYPE_INFO
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


pub static EMITTERHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "EmitterBase",
    data: TypeInfoData::Array("EmitterHandle"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EmitterCreateState {
    pub transform: super::core::LinearTransform,
}

pub trait EmitterCreateStateTrait: TypeObject {
    fn transform(&self) -> &super::core::LinearTransform;
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform;
}

impl EmitterCreateStateTrait for EmitterCreateState {
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.transform
    }
}

pub static EMITTERCREATESTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterCreateState",
    flags: MemberInfoFlags::new(36937),
    module: "EmitterBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EmitterCreateState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(EmitterCreateState, transform),
            },
        ],
    }),
    array_type: Some(EMITTERCREATESTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EmitterCreateState {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERCREATESTATE_TYPE_INFO
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


pub static EMITTERCREATESTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterCreateState-Array",
    flags: MemberInfoFlags::new(145),
    module: "EmitterBase",
    data: TypeInfoData::Array("EmitterCreateState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EmitterDynamicState {
    pub params: super::effect_base::EffectParams,
    pub effect_transform_space_params: Vec<super::effect_base::EffectTransformSpaceParam>,
    pub state: EmitterState,
    pub source_pt: super::core::Vec3,
    pub target_pt: super::core::Vec3,
    pub other_pt: super::core::Vec3,
    pub effect_position: super::core::Vec3,
    pub light_probe_sample_offset: super::core::Vec3,
    pub ctrl_points: Vec<EmitterControlPoint>,
    pub active_ctrl_point_count: u32,
    pub emitter_exposed_inputs: Vec<super::effect_base::EmitterExposedInput>,
    pub emitter_graph_overrides: super::effect_base::EmitterGraphOverrides,
    pub included_cull_id: super::render_base::CullIdHandle,
    pub excluded_cull_id: super::render_base::CullIdHandle,
    pub field_flag_changed0: u16,
}

pub trait EmitterDynamicStateTrait: TypeObject {
    fn params(&self) -> &super::effect_base::EffectParams;
    fn params_mut(&mut self) -> &mut super::effect_base::EffectParams;
    fn effect_transform_space_params(&self) -> &Vec<super::effect_base::EffectTransformSpaceParam>;
    fn effect_transform_space_params_mut(&mut self) -> &mut Vec<super::effect_base::EffectTransformSpaceParam>;
    fn state(&self) -> &EmitterState;
    fn state_mut(&mut self) -> &mut EmitterState;
    fn source_pt(&self) -> &super::core::Vec3;
    fn source_pt_mut(&mut self) -> &mut super::core::Vec3;
    fn target_pt(&self) -> &super::core::Vec3;
    fn target_pt_mut(&mut self) -> &mut super::core::Vec3;
    fn other_pt(&self) -> &super::core::Vec3;
    fn other_pt_mut(&mut self) -> &mut super::core::Vec3;
    fn effect_position(&self) -> &super::core::Vec3;
    fn effect_position_mut(&mut self) -> &mut super::core::Vec3;
    fn light_probe_sample_offset(&self) -> &super::core::Vec3;
    fn light_probe_sample_offset_mut(&mut self) -> &mut super::core::Vec3;
    fn ctrl_points(&self) -> &Vec<EmitterControlPoint>;
    fn ctrl_points_mut(&mut self) -> &mut Vec<EmitterControlPoint>;
    fn active_ctrl_point_count(&self) -> &u32;
    fn active_ctrl_point_count_mut(&mut self) -> &mut u32;
    fn emitter_exposed_inputs(&self) -> &Vec<super::effect_base::EmitterExposedInput>;
    fn emitter_exposed_inputs_mut(&mut self) -> &mut Vec<super::effect_base::EmitterExposedInput>;
    fn emitter_graph_overrides(&self) -> &super::effect_base::EmitterGraphOverrides;
    fn emitter_graph_overrides_mut(&mut self) -> &mut super::effect_base::EmitterGraphOverrides;
    fn included_cull_id(&self) -> &super::render_base::CullIdHandle;
    fn included_cull_id_mut(&mut self) -> &mut super::render_base::CullIdHandle;
    fn excluded_cull_id(&self) -> &super::render_base::CullIdHandle;
    fn excluded_cull_id_mut(&mut self) -> &mut super::render_base::CullIdHandle;
    fn field_flag_changed0(&self) -> &u16;
    fn field_flag_changed0_mut(&mut self) -> &mut u16;
}

impl EmitterDynamicStateTrait for EmitterDynamicState {
    fn params(&self) -> &super::effect_base::EffectParams {
        &self.params
    }
    fn params_mut(&mut self) -> &mut super::effect_base::EffectParams {
        &mut self.params
    }
    fn effect_transform_space_params(&self) -> &Vec<super::effect_base::EffectTransformSpaceParam> {
        &self.effect_transform_space_params
    }
    fn effect_transform_space_params_mut(&mut self) -> &mut Vec<super::effect_base::EffectTransformSpaceParam> {
        &mut self.effect_transform_space_params
    }
    fn state(&self) -> &EmitterState {
        &self.state
    }
    fn state_mut(&mut self) -> &mut EmitterState {
        &mut self.state
    }
    fn source_pt(&self) -> &super::core::Vec3 {
        &self.source_pt
    }
    fn source_pt_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.source_pt
    }
    fn target_pt(&self) -> &super::core::Vec3 {
        &self.target_pt
    }
    fn target_pt_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.target_pt
    }
    fn other_pt(&self) -> &super::core::Vec3 {
        &self.other_pt
    }
    fn other_pt_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.other_pt
    }
    fn effect_position(&self) -> &super::core::Vec3 {
        &self.effect_position
    }
    fn effect_position_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.effect_position
    }
    fn light_probe_sample_offset(&self) -> &super::core::Vec3 {
        &self.light_probe_sample_offset
    }
    fn light_probe_sample_offset_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.light_probe_sample_offset
    }
    fn ctrl_points(&self) -> &Vec<EmitterControlPoint> {
        &self.ctrl_points
    }
    fn ctrl_points_mut(&mut self) -> &mut Vec<EmitterControlPoint> {
        &mut self.ctrl_points
    }
    fn active_ctrl_point_count(&self) -> &u32 {
        &self.active_ctrl_point_count
    }
    fn active_ctrl_point_count_mut(&mut self) -> &mut u32 {
        &mut self.active_ctrl_point_count
    }
    fn emitter_exposed_inputs(&self) -> &Vec<super::effect_base::EmitterExposedInput> {
        &self.emitter_exposed_inputs
    }
    fn emitter_exposed_inputs_mut(&mut self) -> &mut Vec<super::effect_base::EmitterExposedInput> {
        &mut self.emitter_exposed_inputs
    }
    fn emitter_graph_overrides(&self) -> &super::effect_base::EmitterGraphOverrides {
        &self.emitter_graph_overrides
    }
    fn emitter_graph_overrides_mut(&mut self) -> &mut super::effect_base::EmitterGraphOverrides {
        &mut self.emitter_graph_overrides
    }
    fn included_cull_id(&self) -> &super::render_base::CullIdHandle {
        &self.included_cull_id
    }
    fn included_cull_id_mut(&mut self) -> &mut super::render_base::CullIdHandle {
        &mut self.included_cull_id
    }
    fn excluded_cull_id(&self) -> &super::render_base::CullIdHandle {
        &self.excluded_cull_id
    }
    fn excluded_cull_id_mut(&mut self) -> &mut super::render_base::CullIdHandle {
        &mut self.excluded_cull_id
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_changed0
    }
}

pub static EMITTERDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "EmitterBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EmitterDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Params",
                flags: MemberInfoFlags::new(0),
                field_type: "EffectParams",
                rust_offset: offset_of!(EmitterDynamicState, params),
            },
            FieldInfoData {
                name: "EffectTransformSpaceParams",
                flags: MemberInfoFlags::new(144),
                field_type: "EffectTransformSpaceParam-Array",
                rust_offset: offset_of!(EmitterDynamicState, effect_transform_space_params),
            },
            FieldInfoData {
                name: "State",
                flags: MemberInfoFlags::new(0),
                field_type: "EmitterState",
                rust_offset: offset_of!(EmitterDynamicState, state),
            },
            FieldInfoData {
                name: "SourcePt",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(EmitterDynamicState, source_pt),
            },
            FieldInfoData {
                name: "TargetPt",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(EmitterDynamicState, target_pt),
            },
            FieldInfoData {
                name: "OtherPt",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(EmitterDynamicState, other_pt),
            },
            FieldInfoData {
                name: "EffectPosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(EmitterDynamicState, effect_position),
            },
            FieldInfoData {
                name: "LightProbeSampleOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(EmitterDynamicState, light_probe_sample_offset),
            },
            FieldInfoData {
                name: "CtrlPoints",
                flags: MemberInfoFlags::new(144),
                field_type: "EmitterControlPoint-Array",
                rust_offset: offset_of!(EmitterDynamicState, ctrl_points),
            },
            FieldInfoData {
                name: "ActiveCtrlPointCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EmitterDynamicState, active_ctrl_point_count),
            },
            FieldInfoData {
                name: "EmitterExposedInputs",
                flags: MemberInfoFlags::new(144),
                field_type: "EmitterExposedInput-Array",
                rust_offset: offset_of!(EmitterDynamicState, emitter_exposed_inputs),
            },
            FieldInfoData {
                name: "EmitterGraphOverrides",
                flags: MemberInfoFlags::new(0),
                field_type: "EmitterGraphOverrides",
                rust_offset: offset_of!(EmitterDynamicState, emitter_graph_overrides),
            },
            FieldInfoData {
                name: "IncludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: "CullIdHandle",
                rust_offset: offset_of!(EmitterDynamicState, included_cull_id),
            },
            FieldInfoData {
                name: "ExcludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: "CullIdHandle",
                rust_offset: offset_of!(EmitterDynamicState, excluded_cull_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(EmitterDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(EMITTERDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EmitterDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERDYNAMICSTATE_TYPE_INFO
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


pub static EMITTERDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "EmitterBase",
    data: TypeInfoData::Array("EmitterDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EmitterControlPoint {
    pub position: super::core::Vec3,
    pub index: u32,
}

pub trait EmitterControlPointTrait: TypeObject {
    fn position(&self) -> &super::core::Vec3;
    fn position_mut(&mut self) -> &mut super::core::Vec3;
    fn index(&self) -> &u32;
    fn index_mut(&mut self) -> &mut u32;
}

impl EmitterControlPointTrait for EmitterControlPoint {
    fn position(&self) -> &super::core::Vec3 {
        &self.position
    }
    fn position_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.position
    }
    fn index(&self) -> &u32 {
        &self.index
    }
    fn index_mut(&mut self) -> &mut u32 {
        &mut self.index
    }
}

pub static EMITTERCONTROLPOINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterControlPoint",
    flags: MemberInfoFlags::new(36937),
    module: "EmitterBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EmitterControlPoint as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Position",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(EmitterControlPoint, position),
            },
            FieldInfoData {
                name: "Index",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EmitterControlPoint, index),
            },
        ],
    }),
    array_type: Some(EMITTERCONTROLPOINT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EmitterControlPoint {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERCONTROLPOINT_TYPE_INFO
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


pub static EMITTERCONTROLPOINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterControlPoint-Array",
    flags: MemberInfoFlags::new(145),
    module: "EmitterBase",
    data: TypeInfoData::Array("EmitterControlPoint"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum EmitterState {
    #[default]
    EmitterState_Waiting = 0,
    EmitterState_Playing = 1,
    EmitterState_Stopping = 2,
    EmitterState_Dead = 3,
}

pub static EMITTERSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterState",
    flags: MemberInfoFlags::new(49429),
    module: "EmitterBase",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterState {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERSTATE_TYPE_INFO
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


pub static EMITTERSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterState-Array",
    flags: MemberInfoFlags::new(145),
    module: "EmitterBase",
    data: TypeInfoData::Array("EmitterState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EmitterStaticState {
    pub transform_space: super::state_stream::TransformSpaceHandle,
    pub emitter_base_asset: Option<Arc<Mutex<dyn EmitterBaseAssetTrait>>>,
    pub is_emitter_graph: bool,
    pub emitter_graph_base_asset: Option<Arc<Mutex<dyn super::core::AssetTrait>>>,
    pub spawn_probability: super::core::QualityScalableFloat,
    pub simulation_cull_distance: super::core::QualityScalableFloat,
    pub kill_when_distance_culled: bool,
    pub kill_by_water: bool,
    pub spawn_outside_view_radius: f32,
    pub nearby_radius: f32,
    pub max_nearby_instance_count: u32,
    pub override_draw_order: bool,
    pub draw_order_slot: u8,
    pub is_first_person: bool,
    pub cast_shadows: bool,
    pub cast_reflection: bool,
    pub use_lightprobe_visibility: bool,
    pub inherited_velocity_and_start_delta_time: super::core::FbVec,
    pub inherited_velocity_enabled: bool,
    pub light_probe_sample_method: LightProbeSampleMethod,
    pub light_probe_sample_offset_method: LightProbeSampleOffsetMethod,
    pub group_guid: glacier_util::guid::Guid,
    pub max_instance_count_in_group: u32,
    pub kill_on_max_count: bool,
    pub property_id_lookup_table: Vec<PropertyIdLookup>,
    pub emitter_exposed_texture_inputs: Vec<super::effect_base::EmitterExposedTextureInput>,
    pub effect_time_delta_type: u32,
    pub internal_duplication_render_view_id: u16,
    pub field_flag_changed0: u32,
}

pub trait EmitterStaticStateTrait: TypeObject {
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle;
    fn transform_space_mut(&mut self) -> &mut super::state_stream::TransformSpaceHandle;
    fn emitter_base_asset(&self) -> &Option<Arc<Mutex<dyn EmitterBaseAssetTrait>>>;
    fn emitter_base_asset_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EmitterBaseAssetTrait>>>;
    fn is_emitter_graph(&self) -> &bool;
    fn is_emitter_graph_mut(&mut self) -> &mut bool;
    fn emitter_graph_base_asset(&self) -> &Option<Arc<Mutex<dyn super::core::AssetTrait>>>;
    fn emitter_graph_base_asset_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::core::AssetTrait>>>;
    fn spawn_probability(&self) -> &super::core::QualityScalableFloat;
    fn spawn_probability_mut(&mut self) -> &mut super::core::QualityScalableFloat;
    fn simulation_cull_distance(&self) -> &super::core::QualityScalableFloat;
    fn simulation_cull_distance_mut(&mut self) -> &mut super::core::QualityScalableFloat;
    fn kill_when_distance_culled(&self) -> &bool;
    fn kill_when_distance_culled_mut(&mut self) -> &mut bool;
    fn kill_by_water(&self) -> &bool;
    fn kill_by_water_mut(&mut self) -> &mut bool;
    fn spawn_outside_view_radius(&self) -> &f32;
    fn spawn_outside_view_radius_mut(&mut self) -> &mut f32;
    fn nearby_radius(&self) -> &f32;
    fn nearby_radius_mut(&mut self) -> &mut f32;
    fn max_nearby_instance_count(&self) -> &u32;
    fn max_nearby_instance_count_mut(&mut self) -> &mut u32;
    fn override_draw_order(&self) -> &bool;
    fn override_draw_order_mut(&mut self) -> &mut bool;
    fn draw_order_slot(&self) -> &u8;
    fn draw_order_slot_mut(&mut self) -> &mut u8;
    fn is_first_person(&self) -> &bool;
    fn is_first_person_mut(&mut self) -> &mut bool;
    fn cast_shadows(&self) -> &bool;
    fn cast_shadows_mut(&mut self) -> &mut bool;
    fn cast_reflection(&self) -> &bool;
    fn cast_reflection_mut(&mut self) -> &mut bool;
    fn use_lightprobe_visibility(&self) -> &bool;
    fn use_lightprobe_visibility_mut(&mut self) -> &mut bool;
    fn inherited_velocity_and_start_delta_time(&self) -> &super::core::FbVec;
    fn inherited_velocity_and_start_delta_time_mut(&mut self) -> &mut super::core::FbVec;
    fn inherited_velocity_enabled(&self) -> &bool;
    fn inherited_velocity_enabled_mut(&mut self) -> &mut bool;
    fn light_probe_sample_method(&self) -> &LightProbeSampleMethod;
    fn light_probe_sample_method_mut(&mut self) -> &mut LightProbeSampleMethod;
    fn light_probe_sample_offset_method(&self) -> &LightProbeSampleOffsetMethod;
    fn light_probe_sample_offset_method_mut(&mut self) -> &mut LightProbeSampleOffsetMethod;
    fn group_guid(&self) -> &glacier_util::guid::Guid;
    fn group_guid_mut(&mut self) -> &mut glacier_util::guid::Guid;
    fn max_instance_count_in_group(&self) -> &u32;
    fn max_instance_count_in_group_mut(&mut self) -> &mut u32;
    fn kill_on_max_count(&self) -> &bool;
    fn kill_on_max_count_mut(&mut self) -> &mut bool;
    fn property_id_lookup_table(&self) -> &Vec<PropertyIdLookup>;
    fn property_id_lookup_table_mut(&mut self) -> &mut Vec<PropertyIdLookup>;
    fn emitter_exposed_texture_inputs(&self) -> &Vec<super::effect_base::EmitterExposedTextureInput>;
    fn emitter_exposed_texture_inputs_mut(&mut self) -> &mut Vec<super::effect_base::EmitterExposedTextureInput>;
    fn effect_time_delta_type(&self) -> &u32;
    fn effect_time_delta_type_mut(&mut self) -> &mut u32;
    fn internal_duplication_render_view_id(&self) -> &u16;
    fn internal_duplication_render_view_id_mut(&mut self) -> &mut u16;
    fn field_flag_changed0(&self) -> &u32;
    fn field_flag_changed0_mut(&mut self) -> &mut u32;
}

impl EmitterStaticStateTrait for EmitterStaticState {
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle {
        &self.transform_space
    }
    fn transform_space_mut(&mut self) -> &mut super::state_stream::TransformSpaceHandle {
        &mut self.transform_space
    }
    fn emitter_base_asset(&self) -> &Option<Arc<Mutex<dyn EmitterBaseAssetTrait>>> {
        &self.emitter_base_asset
    }
    fn emitter_base_asset_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EmitterBaseAssetTrait>>> {
        &mut self.emitter_base_asset
    }
    fn is_emitter_graph(&self) -> &bool {
        &self.is_emitter_graph
    }
    fn is_emitter_graph_mut(&mut self) -> &mut bool {
        &mut self.is_emitter_graph
    }
    fn emitter_graph_base_asset(&self) -> &Option<Arc<Mutex<dyn super::core::AssetTrait>>> {
        &self.emitter_graph_base_asset
    }
    fn emitter_graph_base_asset_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::core::AssetTrait>>> {
        &mut self.emitter_graph_base_asset
    }
    fn spawn_probability(&self) -> &super::core::QualityScalableFloat {
        &self.spawn_probability
    }
    fn spawn_probability_mut(&mut self) -> &mut super::core::QualityScalableFloat {
        &mut self.spawn_probability
    }
    fn simulation_cull_distance(&self) -> &super::core::QualityScalableFloat {
        &self.simulation_cull_distance
    }
    fn simulation_cull_distance_mut(&mut self) -> &mut super::core::QualityScalableFloat {
        &mut self.simulation_cull_distance
    }
    fn kill_when_distance_culled(&self) -> &bool {
        &self.kill_when_distance_culled
    }
    fn kill_when_distance_culled_mut(&mut self) -> &mut bool {
        &mut self.kill_when_distance_culled
    }
    fn kill_by_water(&self) -> &bool {
        &self.kill_by_water
    }
    fn kill_by_water_mut(&mut self) -> &mut bool {
        &mut self.kill_by_water
    }
    fn spawn_outside_view_radius(&self) -> &f32 {
        &self.spawn_outside_view_radius
    }
    fn spawn_outside_view_radius_mut(&mut self) -> &mut f32 {
        &mut self.spawn_outside_view_radius
    }
    fn nearby_radius(&self) -> &f32 {
        &self.nearby_radius
    }
    fn nearby_radius_mut(&mut self) -> &mut f32 {
        &mut self.nearby_radius
    }
    fn max_nearby_instance_count(&self) -> &u32 {
        &self.max_nearby_instance_count
    }
    fn max_nearby_instance_count_mut(&mut self) -> &mut u32 {
        &mut self.max_nearby_instance_count
    }
    fn override_draw_order(&self) -> &bool {
        &self.override_draw_order
    }
    fn override_draw_order_mut(&mut self) -> &mut bool {
        &mut self.override_draw_order
    }
    fn draw_order_slot(&self) -> &u8 {
        &self.draw_order_slot
    }
    fn draw_order_slot_mut(&mut self) -> &mut u8 {
        &mut self.draw_order_slot
    }
    fn is_first_person(&self) -> &bool {
        &self.is_first_person
    }
    fn is_first_person_mut(&mut self) -> &mut bool {
        &mut self.is_first_person
    }
    fn cast_shadows(&self) -> &bool {
        &self.cast_shadows
    }
    fn cast_shadows_mut(&mut self) -> &mut bool {
        &mut self.cast_shadows
    }
    fn cast_reflection(&self) -> &bool {
        &self.cast_reflection
    }
    fn cast_reflection_mut(&mut self) -> &mut bool {
        &mut self.cast_reflection
    }
    fn use_lightprobe_visibility(&self) -> &bool {
        &self.use_lightprobe_visibility
    }
    fn use_lightprobe_visibility_mut(&mut self) -> &mut bool {
        &mut self.use_lightprobe_visibility
    }
    fn inherited_velocity_and_start_delta_time(&self) -> &super::core::FbVec {
        &self.inherited_velocity_and_start_delta_time
    }
    fn inherited_velocity_and_start_delta_time_mut(&mut self) -> &mut super::core::FbVec {
        &mut self.inherited_velocity_and_start_delta_time
    }
    fn inherited_velocity_enabled(&self) -> &bool {
        &self.inherited_velocity_enabled
    }
    fn inherited_velocity_enabled_mut(&mut self) -> &mut bool {
        &mut self.inherited_velocity_enabled
    }
    fn light_probe_sample_method(&self) -> &LightProbeSampleMethod {
        &self.light_probe_sample_method
    }
    fn light_probe_sample_method_mut(&mut self) -> &mut LightProbeSampleMethod {
        &mut self.light_probe_sample_method
    }
    fn light_probe_sample_offset_method(&self) -> &LightProbeSampleOffsetMethod {
        &self.light_probe_sample_offset_method
    }
    fn light_probe_sample_offset_method_mut(&mut self) -> &mut LightProbeSampleOffsetMethod {
        &mut self.light_probe_sample_offset_method
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
    fn property_id_lookup_table(&self) -> &Vec<PropertyIdLookup> {
        &self.property_id_lookup_table
    }
    fn property_id_lookup_table_mut(&mut self) -> &mut Vec<PropertyIdLookup> {
        &mut self.property_id_lookup_table
    }
    fn emitter_exposed_texture_inputs(&self) -> &Vec<super::effect_base::EmitterExposedTextureInput> {
        &self.emitter_exposed_texture_inputs
    }
    fn emitter_exposed_texture_inputs_mut(&mut self) -> &mut Vec<super::effect_base::EmitterExposedTextureInput> {
        &mut self.emitter_exposed_texture_inputs
    }
    fn effect_time_delta_type(&self) -> &u32 {
        &self.effect_time_delta_type
    }
    fn effect_time_delta_type_mut(&mut self) -> &mut u32 {
        &mut self.effect_time_delta_type
    }
    fn internal_duplication_render_view_id(&self) -> &u16 {
        &self.internal_duplication_render_view_id
    }
    fn internal_duplication_render_view_id_mut(&mut self) -> &mut u16 {
        &mut self.internal_duplication_render_view_id
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u32 {
        &mut self.field_flag_changed0
    }
}

pub static EMITTERSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterStaticState",
    flags: MemberInfoFlags::new(73),
    module: "EmitterBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EmitterStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: "TransformSpaceHandle",
                rust_offset: offset_of!(EmitterStaticState, transform_space),
            },
            FieldInfoData {
                name: "EmitterBaseAsset",
                flags: MemberInfoFlags::new(0),
                field_type: "EmitterBaseAsset",
                rust_offset: offset_of!(EmitterStaticState, emitter_base_asset),
            },
            FieldInfoData {
                name: "IsEmitterGraph",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterStaticState, is_emitter_graph),
            },
            FieldInfoData {
                name: "EmitterGraphBaseAsset",
                flags: MemberInfoFlags::new(0),
                field_type: "Asset",
                rust_offset: offset_of!(EmitterStaticState, emitter_graph_base_asset),
            },
            FieldInfoData {
                name: "SpawnProbability",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(EmitterStaticState, spawn_probability),
            },
            FieldInfoData {
                name: "SimulationCullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(EmitterStaticState, simulation_cull_distance),
            },
            FieldInfoData {
                name: "KillWhenDistanceCulled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterStaticState, kill_when_distance_culled),
            },
            FieldInfoData {
                name: "KillByWater",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterStaticState, kill_by_water),
            },
            FieldInfoData {
                name: "SpawnOutsideViewRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterStaticState, spawn_outside_view_radius),
            },
            FieldInfoData {
                name: "NearbyRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterStaticState, nearby_radius),
            },
            FieldInfoData {
                name: "MaxNearbyInstanceCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EmitterStaticState, max_nearby_instance_count),
            },
            FieldInfoData {
                name: "OverrideDrawOrder",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterStaticState, override_draw_order),
            },
            FieldInfoData {
                name: "DrawOrderSlot",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(EmitterStaticState, draw_order_slot),
            },
            FieldInfoData {
                name: "IsFirstPerson",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterStaticState, is_first_person),
            },
            FieldInfoData {
                name: "CastShadows",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterStaticState, cast_shadows),
            },
            FieldInfoData {
                name: "CastReflection",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterStaticState, cast_reflection),
            },
            FieldInfoData {
                name: "UseLightprobeVisibility",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterStaticState, use_lightprobe_visibility),
            },
            FieldInfoData {
                name: "InheritedVelocityAndStartDeltaTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec",
                rust_offset: offset_of!(EmitterStaticState, inherited_velocity_and_start_delta_time),
            },
            FieldInfoData {
                name: "InheritedVelocityEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterStaticState, inherited_velocity_enabled),
            },
            FieldInfoData {
                name: "LightProbeSampleMethod",
                flags: MemberInfoFlags::new(0),
                field_type: "LightProbeSampleMethod",
                rust_offset: offset_of!(EmitterStaticState, light_probe_sample_method),
            },
            FieldInfoData {
                name: "LightProbeSampleOffsetMethod",
                flags: MemberInfoFlags::new(0),
                field_type: "LightProbeSampleOffsetMethod",
                rust_offset: offset_of!(EmitterStaticState, light_probe_sample_offset_method),
            },
            FieldInfoData {
                name: "GroupGuid",
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(EmitterStaticState, group_guid),
            },
            FieldInfoData {
                name: "MaxInstanceCountInGroup",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EmitterStaticState, max_instance_count_in_group),
            },
            FieldInfoData {
                name: "KillOnMaxCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterStaticState, kill_on_max_count),
            },
            FieldInfoData {
                name: "PropertyIdLookupTable",
                flags: MemberInfoFlags::new(144),
                field_type: "PropertyIdLookup-Array",
                rust_offset: offset_of!(EmitterStaticState, property_id_lookup_table),
            },
            FieldInfoData {
                name: "EmitterExposedTextureInputs",
                flags: MemberInfoFlags::new(144),
                field_type: "EmitterExposedTextureInput-Array",
                rust_offset: offset_of!(EmitterStaticState, emitter_exposed_texture_inputs),
            },
            FieldInfoData {
                name: "EffectTimeDeltaType",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EmitterStaticState, effect_time_delta_type),
            },
            FieldInfoData {
                name: "InternalDuplicationRenderViewId",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(EmitterStaticState, internal_duplication_render_view_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EmitterStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(EMITTERSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EmitterStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERSTATICSTATE_TYPE_INFO
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


pub static EMITTERSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "EmitterBase",
    data: TypeInfoData::Array("EmitterStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EmitterExclusionVolumesBaseAsset {
    pub _glacier_base: super::core::Asset,
}

pub trait EmitterExclusionVolumesBaseAssetTrait: super::core::AssetTrait {
}

impl EmitterExclusionVolumesBaseAssetTrait for EmitterExclusionVolumesBaseAsset {
}

impl super::core::AssetTrait for EmitterExclusionVolumesBaseAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for EmitterExclusionVolumesBaseAsset {
}

pub static EMITTEREXCLUSIONVOLUMESBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterExclusionVolumesBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "EmitterBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EmitterExclusionVolumesBaseAsset as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(EMITTEREXCLUSIONVOLUMESBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EmitterExclusionVolumesBaseAsset {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTEREXCLUSIONVOLUMESBASEASSET_TYPE_INFO
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


pub static EMITTEREXCLUSIONVOLUMESBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterExclusionVolumesBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "EmitterBase",
    data: TypeInfoData::Array("EmitterExclusionVolumesBaseAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EmitterTag {
    pub _glacier_base: super::core::DataContainer,
    pub name: String,
}

pub trait EmitterTagTrait: super::core::DataContainerTrait {
    fn name(&self) -> &String;
    fn name_mut(&mut self) -> &mut String;
}

impl EmitterTagTrait for EmitterTag {
    fn name(&self) -> &String {
        &self.name
    }
    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
}

impl super::core::DataContainerTrait for EmitterTag {
}

pub static EMITTERTAG_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterTag",
    flags: MemberInfoFlags::new(101),
    module: "EmitterBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EmitterTag as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(EmitterTag, name),
            },
        ],
    }),
    array_type: Some(EMITTERTAG_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EmitterTag {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERTAG_TYPE_INFO
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


pub static EMITTERTAG_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterTag-Array",
    flags: MemberInfoFlags::new(145),
    module: "EmitterBase",
    data: TypeInfoData::Array("EmitterTag"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EmitterGraphBaseAsset {
    pub _glacier_base: super::core::Asset,
}

pub trait EmitterGraphBaseAssetTrait: super::core::AssetTrait {
}

impl EmitterGraphBaseAssetTrait for EmitterGraphBaseAsset {
}

impl super::core::AssetTrait for EmitterGraphBaseAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for EmitterGraphBaseAsset {
}

pub static EMITTERGRAPHBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "EmitterBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EmitterGraphBaseAsset as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(EMITTERGRAPHBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EmitterGraphBaseAsset {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERGRAPHBASEASSET_TYPE_INFO
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


pub static EMITTERGRAPHBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "EmitterBase",
    data: TypeInfoData::Array("EmitterGraphBaseAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EmitterBaseAsset {
    pub _glacier_base: super::core::Asset,
}

pub trait EmitterBaseAssetTrait: super::core::AssetTrait {
}

impl EmitterBaseAssetTrait for EmitterBaseAsset {
}

impl super::core::AssetTrait for EmitterBaseAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for EmitterBaseAsset {
}

pub static EMITTERBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "EmitterBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EmitterBaseAsset as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(EMITTERBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EmitterBaseAsset {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERBASEASSET_TYPE_INFO
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


pub static EMITTERBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "EmitterBase",
    data: TypeInfoData::Array("EmitterBaseAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PropertyIdLookup {
    pub field_property_id: i32,
    pub field_property_hash: i32,
    pub emitter_exposable_type: EmitterExposableType,
}

pub trait PropertyIdLookupTrait: TypeObject {
    fn field_property_id(&self) -> &i32;
    fn field_property_id_mut(&mut self) -> &mut i32;
    fn field_property_hash(&self) -> &i32;
    fn field_property_hash_mut(&mut self) -> &mut i32;
    fn emitter_exposable_type(&self) -> &EmitterExposableType;
    fn emitter_exposable_type_mut(&mut self) -> &mut EmitterExposableType;
}

impl PropertyIdLookupTrait for PropertyIdLookup {
    fn field_property_id(&self) -> &i32 {
        &self.field_property_id
    }
    fn field_property_id_mut(&mut self) -> &mut i32 {
        &mut self.field_property_id
    }
    fn field_property_hash(&self) -> &i32 {
        &self.field_property_hash
    }
    fn field_property_hash_mut(&mut self) -> &mut i32 {
        &mut self.field_property_hash
    }
    fn emitter_exposable_type(&self) -> &EmitterExposableType {
        &self.emitter_exposable_type
    }
    fn emitter_exposable_type_mut(&mut self) -> &mut EmitterExposableType {
        &mut self.emitter_exposable_type
    }
}

pub static PROPERTYIDLOOKUP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyIdLookup",
    flags: MemberInfoFlags::new(36937),
    module: "EmitterBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PropertyIdLookup as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "FieldPropertyId",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PropertyIdLookup, field_property_id),
            },
            FieldInfoData {
                name: "FieldPropertyHash",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PropertyIdLookup, field_property_hash),
            },
            FieldInfoData {
                name: "EmitterExposableType",
                flags: MemberInfoFlags::new(0),
                field_type: "EmitterExposableType",
                rust_offset: offset_of!(PropertyIdLookup, emitter_exposable_type),
            },
        ],
    }),
    array_type: Some(PROPERTYIDLOOKUP_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for PropertyIdLookup {
    fn type_info(&self) -> &'static TypeInfo {
        PROPERTYIDLOOKUP_TYPE_INFO
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


pub static PROPERTYIDLOOKUP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyIdLookup-Array",
    flags: MemberInfoFlags::new(145),
    module: "EmitterBase",
    data: TypeInfoData::Array("PropertyIdLookup"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum EmitterExposableType {
    #[default]
    EmitterExposableType_Float = 0,
    EmitterExposableType_Vec2 = 1,
    EmitterExposableType_Vec3 = 2,
    EmitterExposableType_Vec4 = 3,
}

pub static EMITTEREXPOSABLETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterExposableType",
    flags: MemberInfoFlags::new(49429),
    module: "EmitterBase",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTEREXPOSABLETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterExposableType {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTEREXPOSABLETYPE_TYPE_INFO
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


pub static EMITTEREXPOSABLETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterExposableType-Array",
    flags: MemberInfoFlags::new(145),
    module: "EmitterBase",
    data: TypeInfoData::Array("EmitterExposableType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EmitterExclusionVolumeResultData {
    pub transform: super::core::LinearTransform,
    pub half_extent: super::core::Vec3,
    pub id: u32,
}

pub trait EmitterExclusionVolumeResultDataTrait: TypeObject {
    fn transform(&self) -> &super::core::LinearTransform;
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform;
    fn half_extent(&self) -> &super::core::Vec3;
    fn half_extent_mut(&mut self) -> &mut super::core::Vec3;
    fn id(&self) -> &u32;
    fn id_mut(&mut self) -> &mut u32;
}

impl EmitterExclusionVolumeResultDataTrait for EmitterExclusionVolumeResultData {
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.transform
    }
    fn half_extent(&self) -> &super::core::Vec3 {
        &self.half_extent
    }
    fn half_extent_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.half_extent
    }
    fn id(&self) -> &u32 {
        &self.id
    }
    fn id_mut(&mut self) -> &mut u32 {
        &mut self.id
    }
}

pub static EMITTEREXCLUSIONVOLUMERESULTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterExclusionVolumeResultData",
    flags: MemberInfoFlags::new(36937),
    module: "EmitterBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EmitterExclusionVolumeResultData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(EmitterExclusionVolumeResultData, transform),
            },
            FieldInfoData {
                name: "HalfExtent",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(EmitterExclusionVolumeResultData, half_extent),
            },
            FieldInfoData {
                name: "Id",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EmitterExclusionVolumeResultData, id),
            },
        ],
    }),
    array_type: Some(EMITTEREXCLUSIONVOLUMERESULTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EmitterExclusionVolumeResultData {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTEREXCLUSIONVOLUMERESULTDATA_TYPE_INFO
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


pub static EMITTEREXCLUSIONVOLUMERESULTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterExclusionVolumeResultData-Array",
    flags: MemberInfoFlags::new(145),
    module: "EmitterBase",
    data: TypeInfoData::Array("EmitterExclusionVolumeResultData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum LightProbeSampleOffsetMethod {
    #[default]
    LightProbeSampleOffsetMethod_World = 0,
    LightProbeSampleOffsetMethod_Local = 1,
}

pub static LIGHTPROBESAMPLEOFFSETMETHOD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightProbeSampleOffsetMethod",
    flags: MemberInfoFlags::new(49429),
    module: "EmitterBase",
    data: TypeInfoData::Enum,
    array_type: Some(LIGHTPROBESAMPLEOFFSETMETHOD_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LightProbeSampleOffsetMethod {
    fn type_info(&self) -> &'static TypeInfo {
        LIGHTPROBESAMPLEOFFSETMETHOD_TYPE_INFO
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


pub static LIGHTPROBESAMPLEOFFSETMETHOD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightProbeSampleOffsetMethod-Array",
    flags: MemberInfoFlags::new(145),
    module: "EmitterBase",
    data: TypeInfoData::Array("LightProbeSampleOffsetMethod"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum LightProbeSampleMethod {
    #[default]
    LightProbeSampleMethod_BoundingBoxCenter = 0,
    LightProbeSampleMethod_EmitterTransform = 1,
    LightProbeSampleMethod_DefaultLightProbe = 2,
    LightProbeSampleMethod_WorldAbsolute = 3,
}

pub static LIGHTPROBESAMPLEMETHOD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightProbeSampleMethod",
    flags: MemberInfoFlags::new(49429),
    module: "EmitterBase",
    data: TypeInfoData::Enum,
    array_type: Some(LIGHTPROBESAMPLEMETHOD_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LightProbeSampleMethod {
    fn type_info(&self) -> &'static TypeInfo {
        LIGHTPROBESAMPLEMETHOD_TYPE_INFO
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


pub static LIGHTPROBESAMPLEMETHOD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightProbeSampleMethod-Array",
    flags: MemberInfoFlags::new(145),
    module: "EmitterBase",
    data: TypeInfoData::Array("LightProbeSampleMethod"),
    array_type: None,
    alignment: 8,
};


